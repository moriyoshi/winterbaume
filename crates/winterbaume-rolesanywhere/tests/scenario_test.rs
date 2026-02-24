/// End-to-end scenario tests for IAM Roles Anywhere.
///
/// These tests chain multiple operations to verify realistic use-case workflows
/// rather than testing individual API shapes.
use aws_sdk_rolesanywhere::config::BehaviorVersion;
use aws_sdk_rolesanywhere::types::{Source, SourceData, Tag, TrustAnchorType};
use aws_smithy_types::Blob;
use winterbaume_core::MockAws;
use winterbaume_rolesanywhere::RolesAnywhereService;

async fn make_client() -> aws_sdk_rolesanywhere::Client {
    let mock = MockAws::builder()
        .with_service(RolesAnywhereService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_rolesanywhere::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_rolesanywhere::Client::new(&config)
}

/// Scenario: Full trust anchor + profile lifecycle with tags.
///
/// 1. Create a trust anchor with a certificate bundle.
/// 2. Create a profile scoped to a role ARN.
/// 3. Tag both resources.
/// 4. List tags to verify they were applied.
/// 5. Disable and re-enable the trust anchor and profile.
/// 6. Delete profile, then delete trust anchor.
/// 7. Verify both are gone.
#[tokio::test]
async fn test_trust_anchor_and_profile_lifecycle() {
    let client = make_client().await;

    // Step 1: create trust anchor
    let ta_resp = client
        .create_trust_anchor()
        .name("scenario-anchor")
        .source(
            Source::builder()
                .source_type(TrustAnchorType::CertificateBundle)
                .source_data(SourceData::X509CertificateData("PEM-CERT".to_string()))
                .build(),
        )
        .send()
        .await
        .expect("create_trust_anchor should succeed");

    let ta = ta_resp.trust_anchor().unwrap();
    let ta_id = ta.trust_anchor_id().unwrap().to_string();
    let ta_arn = ta.trust_anchor_arn().unwrap().to_string();
    assert_eq!(ta.name(), Some("scenario-anchor"));
    assert_eq!(ta.enabled(), Some(true));

    // Step 2: create profile
    let profile_resp = client
        .create_profile()
        .name("scenario-profile")
        .role_arns("arn:aws:iam::123456789012:role/ScenarioRole")
        .duration_seconds(3600)
        .send()
        .await
        .expect("create_profile should succeed");

    let profile = profile_resp.profile().unwrap();
    let profile_id = profile.profile_id().unwrap().to_string();
    let profile_arn = profile.profile_arn().unwrap().to_string();

    // Step 3: tag both resources
    client
        .tag_resource()
        .resource_arn(&ta_arn)
        .tags(Tag::builder().key("env").value("test").build().unwrap())
        .send()
        .await
        .expect("tag trust anchor should succeed");

    client
        .tag_resource()
        .resource_arn(&profile_arn)
        .tags(Tag::builder().key("env").value("test").build().unwrap())
        .send()
        .await
        .expect("tag profile should succeed");

    // Step 4: verify tags
    let ta_tags = client
        .list_tags_for_resource()
        .resource_arn(&ta_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");
    assert_eq!(ta_tags.tags().len(), 1);
    assert_eq!(ta_tags.tags()[0].key(), "env");

    // Step 5: disable/enable trust anchor and profile
    let disabled_ta = client
        .disable_trust_anchor()
        .trust_anchor_id(&ta_id)
        .send()
        .await
        .unwrap();
    assert_eq!(disabled_ta.trust_anchor().unwrap().enabled(), Some(false));

    let disabled_profile = client
        .disable_profile()
        .profile_id(&profile_id)
        .send()
        .await
        .unwrap();
    assert_eq!(disabled_profile.profile().unwrap().enabled(), Some(false));

    client
        .enable_trust_anchor()
        .trust_anchor_id(&ta_id)
        .send()
        .await
        .unwrap();
    client
        .enable_profile()
        .profile_id(&profile_id)
        .send()
        .await
        .unwrap();

    // Step 6: delete profile, then trust anchor
    client
        .delete_profile()
        .profile_id(&profile_id)
        .send()
        .await
        .expect("delete_profile should succeed");

    client
        .delete_trust_anchor()
        .trust_anchor_id(&ta_id)
        .send()
        .await
        .expect("delete_trust_anchor should succeed");

    // Step 7: verify both are gone
    let profile_err = client.get_profile().profile_id(&profile_id).send().await;
    assert!(
        profile_err.is_err(),
        "get_profile after delete should return error"
    );

    let ta_err = client
        .get_trust_anchor()
        .trust_anchor_id(&ta_id)
        .send()
        .await;
    assert!(
        ta_err.is_err(),
        "get_trust_anchor after delete should return error"
    );
}

/// Scenario: CRL lifecycle tied to a trust anchor ARN.
///
/// 1. Import a CRL associated with a trust anchor ARN.
/// 2. Update the CRL name.
/// 3. Enable and disable the CRL.
/// 4. Delete the CRL and verify it is gone.
#[tokio::test]
async fn test_crl_lifecycle() {
    let client = make_client().await;

    let ta_arn = "arn:aws:rolesanywhere:us-east-1:123456789012:trust-anchor/test-crl-anchor";

    // Step 1: import CRL
    let import_resp = client
        .import_crl()
        .name("scenario-crl")
        .crl_data(Blob::new(b"FAKE-CRL-DER-BYTES".to_vec()))
        .trust_anchor_arn(ta_arn)
        .send()
        .await
        .expect("import_crl should succeed");

    let crl = import_resp.crl().unwrap();
    let crl_id = crl.crl_id().unwrap().to_string();
    assert_eq!(crl.name(), Some("scenario-crl"));
    assert_eq!(crl.enabled(), Some(true));

    // Step 2: update name
    let updated = client
        .update_crl()
        .crl_id(&crl_id)
        .name("scenario-crl-renamed")
        .send()
        .await
        .expect("update_crl should succeed");
    assert_eq!(updated.crl().unwrap().name(), Some("scenario-crl-renamed"));

    // Step 3: disable then enable
    let disabled = client.disable_crl().crl_id(&crl_id).send().await.unwrap();
    assert_eq!(disabled.crl().unwrap().enabled(), Some(false));

    let enabled = client.enable_crl().crl_id(&crl_id).send().await.unwrap();
    assert_eq!(enabled.crl().unwrap().enabled(), Some(true));

    // Step 4: delete and verify
    client
        .delete_crl()
        .crl_id(&crl_id)
        .send()
        .await
        .expect("delete_crl should succeed");

    let err = client.get_crl().crl_id(&crl_id).send().await;
    assert!(err.is_err(), "get_crl after delete should return error");
}
