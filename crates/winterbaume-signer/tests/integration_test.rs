use aws_sdk_signer::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_signer::SignerService;

async fn make_client() -> aws_sdk_signer::Client {
    let mock = MockAws::builder()
        .with_service(SignerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_signer::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_signer::Client::new(&config)
}

#[tokio::test]
async fn test_put_and_get_signing_profile() {
    let client = make_client().await;

    let put_resp = client
        .put_signing_profile()
        .profile_name("my-profile")
        .platform_id("AWSLambda-SHA384-ECDSA")
        .send()
        .await
        .expect("put_signing_profile should succeed");

    let arn = put_resp.arn().expect("should have arn");
    assert!(!arn.is_empty());
    assert!(put_resp.profile_version().is_some());

    let get_resp = client
        .get_signing_profile()
        .profile_name("my-profile")
        .send()
        .await
        .expect("get_signing_profile should succeed");

    assert_eq!(get_resp.profile_name(), Some("my-profile"));
    assert_eq!(get_resp.platform_id(), Some("AWSLambda-SHA384-ECDSA"));
    assert_eq!(
        get_resp.status(),
        Some(&aws_sdk_signer::types::SigningProfileStatus::Active)
    );
}

#[tokio::test]
async fn test_list_signing_profiles() {
    let client = make_client().await;

    for name in ["profile-a", "profile-b"] {
        client
            .put_signing_profile()
            .profile_name(name)
            .platform_id("AWSLambda-SHA384-ECDSA")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_signing_profiles()
        .send()
        .await
        .expect("list_signing_profiles should succeed");

    assert_eq!(resp.profiles().len(), 2);
}

#[tokio::test]
async fn test_cancel_signing_profile() {
    // Port of moto test_cancel_signing_profile:
    // After cancellation, get_signing_profile returns with "Canceled" status
    let client = make_client().await;

    client
        .put_signing_profile()
        .profile_name("delete-me")
        .platform_id("AWSLambda-SHA384-ECDSA")
        .send()
        .await
        .unwrap();

    client
        .cancel_signing_profile()
        .profile_name("delete-me")
        .send()
        .await
        .expect("cancel_signing_profile should succeed");

    let resp = client
        .get_signing_profile()
        .profile_name("delete-me")
        .send()
        .await
        .expect("get_signing_profile should still succeed after cancel");
    assert_eq!(
        resp.status(),
        Some(&aws_sdk_signer::types::SigningProfileStatus::Canceled)
    );
}

#[tokio::test]
async fn test_create_cancel_then_list_empty() {
    // After cancellation, the profile is excluded from list_signing_profiles
    let client = make_client().await;

    client
        .put_signing_profile()
        .profile_name("ephemeral-profile")
        .platform_id("AWSLambda-SHA384-ECDSA")
        .send()
        .await
        .unwrap();

    let list = client.list_signing_profiles().send().await.unwrap();
    assert_eq!(list.profiles().len(), 1);

    client
        .cancel_signing_profile()
        .profile_name("ephemeral-profile")
        .send()
        .await
        .unwrap();

    let list = client.list_signing_profiles().send().await.unwrap();
    assert_eq!(list.profiles().len(), 0);
}

#[tokio::test]
async fn test_get_nonexistent_profile() {
    let client = make_client().await;

    let result = client
        .get_signing_profile()
        .profile_name("no-such-profile")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_put_signing_profile_response_fields() {
    // Port of moto test_put_signing_profile - verify arn, profileVersion, profileVersionArn
    let client = make_client().await;

    let resp = client
        .put_signing_profile()
        .profile_name("prof1")
        .platform_id("AWSLambda-SHA384-ECDSA")
        .send()
        .await
        .expect("put_signing_profile should succeed");

    assert!(resp.arn().is_some());
    assert!(!resp.arn().unwrap().is_empty());
    assert!(resp.profile_version().is_some());
    assert!(!resp.profile_version().unwrap().is_empty());
    assert!(resp.profile_version_arn().is_some());
    assert!(!resp.profile_version_arn().unwrap().is_empty());
}

#[tokio::test]
async fn test_get_signing_profile_fields() {
    // Port of moto test_get_signing_profile - verify all fields present
    let client = make_client().await;

    client
        .put_signing_profile()
        .profile_name("prof2")
        .platform_id("AWSLambda-SHA384-ECDSA")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_signing_profile()
        .profile_name("prof2")
        .send()
        .await
        .expect("get_signing_profile should succeed");

    assert!(resp.arn().is_some());
    assert!(resp.profile_version().is_some());
    assert!(resp.profile_version_arn().is_some());
    assert_eq!(
        resp.status(),
        Some(&aws_sdk_signer::types::SigningProfileStatus::Active)
    );
    assert_eq!(resp.profile_name(), Some("prof2"));
    assert_eq!(resp.platform_id(), Some("AWSLambda-SHA384-ECDSA"));
}

#[tokio::test]
async fn test_tag_resource_and_list_tags() {
    // Port of moto test_get_signing_profile__with_args - test tag operations
    let client = make_client().await;

    let put_resp = client
        .put_signing_profile()
        .profile_name("tagged-profile")
        .platform_id("AWSLambda-SHA384-ECDSA")
        .send()
        .await
        .unwrap();

    let profile_arn = put_resp.arn().unwrap().to_string();

    // Tag the resource
    client
        .tag_resource()
        .resource_arn(&profile_arn)
        .tags("k1", "v1")
        .tags("k2", "v2")
        .send()
        .await
        .expect("tag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&profile_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tags_resp.tags().expect("should have tags");
    assert_eq!(tags.get("k1").map(|s| s.as_str()), Some("v1"));
    assert_eq!(tags.get("k2").map(|s| s.as_str()), Some("v2"));

    // Untag one key
    client
        .untag_resource()
        .resource_arn(&profile_arn)
        .tag_keys("k2")
        .send()
        .await
        .expect("untag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&profile_arn)
        .send()
        .await
        .unwrap();

    let tags = tags_resp.tags().expect("should have tags");
    assert_eq!(tags.get("k1").map(|s| s.as_str()), Some("v1"));
    assert!(tags.get("k2").is_none());
}

#[tokio::test]
async fn test_list_signing_platforms() {
    // Port of moto test_list_signing_platforms - verify platforms are returned
    let client = make_client().await;

    let resp = client
        .list_signing_platforms()
        .send()
        .await
        .expect("list_signing_platforms should succeed");

    let platforms = resp.platforms();
    assert!(!platforms.is_empty());
    // Should include AWSLambda-SHA384-ECDSA
    let platform_ids: Vec<&str> = platforms
        .iter()
        .map(|p| p.platform_id().unwrap_or(""))
        .collect();
    assert!(platform_ids.contains(&"AWSLambda-SHA384-ECDSA"));
}

// ============================================================================
// Tests derived from AWS documentation: AWS Signer
// ============================================================================

#[tokio::test]
async fn test_list_signing_profiles_include_canceled() {
    // ListSigningProfiles with includeCanceled=true must return canceled profiles.
    // AWS docs: "Returns only profiles with an ACTIVE status unless the
    // includeCanceled request field is set to true."
    let client = make_client().await;

    client
        .put_signing_profile()
        .profile_name("active-profile")
        .platform_id("AWSLambda-SHA384-ECDSA")
        .send()
        .await
        .unwrap();

    client
        .put_signing_profile()
        .profile_name("canceled-profile")
        .platform_id("AWSLambda-SHA384-ECDSA")
        .send()
        .await
        .unwrap();

    client
        .cancel_signing_profile()
        .profile_name("canceled-profile")
        .send()
        .await
        .unwrap();

    // Without includeCanceled: only active
    let list = client
        .list_signing_profiles()
        .send()
        .await
        .expect("list_signing_profiles should succeed");
    assert_eq!(
        list.profiles().len(),
        1,
        "Default list should only return active profiles"
    );

    // With includeCanceled=true: both
    let list_with_canceled = client
        .list_signing_profiles()
        .include_canceled(true)
        .send()
        .await
        .expect("list_signing_profiles with includeCanceled=true should succeed");
    assert_eq!(
        list_with_canceled.profiles().len(),
        2,
        "includeCanceled=true should return both active and canceled profiles"
    );

    let statuses: Vec<&str> = list_with_canceled
        .profiles()
        .iter()
        .map(|p| p.status().map(|s| s.as_str()).unwrap_or(""))
        .collect();
    assert!(
        statuses.contains(&"Active"),
        "Active profile should be in list"
    );
    assert!(
        statuses.contains(&"Canceled"),
        "Canceled profile should be in list when includeCanceled=true"
    );
}

#[tokio::test]
async fn test_list_signing_profiles_exclude_canceled_by_default() {
    // Without includeCanceled, canceled profiles must be absent from the list.
    let client = make_client().await;

    client
        .put_signing_profile()
        .profile_name("to-cancel")
        .platform_id("AWSLambda-SHA384-ECDSA")
        .send()
        .await
        .unwrap();

    client
        .cancel_signing_profile()
        .profile_name("to-cancel")
        .send()
        .await
        .unwrap();

    let list = client
        .list_signing_profiles()
        .send()
        .await
        .expect("list_signing_profiles should succeed");
    assert_eq!(
        list.profiles().len(),
        0,
        "Canceled profiles must not appear in default list"
    );
}

#[tokio::test]
async fn test_cancel_nonexistent_profile() {
    // CancelSigningProfile on a nonexistent profile must return ResourceNotFoundException.
    let client = make_client().await;

    let result = client
        .cancel_signing_profile()
        .profile_name("no-such-profile")
        .send()
        .await;
    assert!(result.is_err(), "Expected error for nonexistent profile");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_tags_nonexistent_resource() {
    // ListTagsForResource on a nonexistent ARN must return ResourceNotFoundException.
    let client = make_client().await;

    let result = client
        .list_tags_for_resource()
        .resource_arn("arn:aws:signer:us-east-1:123456789012:/signing-profiles/no-such-profile")
        .send()
        .await;
    assert!(result.is_err(), "Expected error for nonexistent resource");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_signing_platforms_has_fields() {
    // ListSigningPlatforms: each platform should have key fields populated.
    let client = make_client().await;

    let resp = client
        .list_signing_platforms()
        .send()
        .await
        .expect("list_signing_platforms should succeed");

    let platforms = resp.platforms();
    assert!(!platforms.is_empty(), "Should have at least one platform");

    for platform in platforms {
        assert!(
            platform.platform_id().is_some() && !platform.platform_id().unwrap().is_empty(),
            "platformId must be present"
        );
        assert!(
            platform.display_name().is_some(),
            "displayName should be present"
        );
        assert!(platform.partner().is_some(), "partner should be present");
        assert!(platform.target().is_some(), "target should be present");
        assert!(platform.category().is_some(), "category should be present");
    }
}

// ============================================================================
// Tests for 11 newly implemented operations
// ============================================================================

#[tokio::test]
async fn test_get_signing_platform() {
    let client = make_client().await;

    let resp = client
        .get_signing_platform()
        .platform_id("AWSLambda-SHA384-ECDSA")
        .send()
        .await
        .expect("get_signing_platform should succeed");

    assert_eq!(resp.platform_id(), Some("AWSLambda-SHA384-ECDSA"));
    assert!(
        resp.display_name().is_some(),
        "displayName should be present"
    );
    assert!(resp.partner().is_some(), "partner should be present");
}

#[tokio::test]
async fn test_get_signing_platform_nonexistent() {
    let client = make_client().await;

    let result = client
        .get_signing_platform()
        .platform_id("no-such-platform")
        .send()
        .await;
    assert!(result.is_err(), "Expected error for unknown platform");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_start_signing_job_and_describe() {
    let client = make_client().await;

    client
        .put_signing_profile()
        .profile_name("job-profile")
        .platform_id("AWSLambda-SHA384-ECDSA")
        .send()
        .await
        .unwrap();

    let start_resp = client
        .start_signing_job()
        .source(
            aws_sdk_signer::types::Source::builder()
                .s3(aws_sdk_signer::types::S3Source::builder()
                    .bucket_name("my-bucket")
                    .key("my-key")
                    .version("v1")
                    .build()
                    .unwrap())
                .build(),
        )
        .destination(
            aws_sdk_signer::types::Destination::builder()
                .s3(aws_sdk_signer::types::S3Destination::builder()
                    .bucket_name("dest-bucket")
                    .build())
                .build(),
        )
        .profile_name("job-profile")
        .client_request_token("token-1")
        .send()
        .await
        .expect("start_signing_job should succeed");

    let job_id = start_resp.job_id().expect("should have jobId");
    assert!(!job_id.is_empty());

    let desc_resp = client
        .describe_signing_job()
        .job_id(job_id)
        .send()
        .await
        .expect("describe_signing_job should succeed");

    assert_eq!(desc_resp.job_id(), Some(job_id));
    assert_eq!(desc_resp.profile_name(), Some("job-profile"));
    assert_eq!(
        desc_resp.status(),
        Some(&aws_sdk_signer::types::SigningStatus::Succeeded)
    );
}

#[tokio::test]
async fn test_describe_signing_job_nonexistent() {
    let client = make_client().await;

    let result = client
        .describe_signing_job()
        .job_id("no-such-job-id")
        .send()
        .await;
    assert!(result.is_err(), "Expected error for nonexistent job");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_signing_jobs() {
    let client = make_client().await;

    client
        .put_signing_profile()
        .profile_name("list-jobs-profile")
        .platform_id("AWSLambda-SHA384-ECDSA")
        .send()
        .await
        .unwrap();

    // Start two jobs
    for token in ["tok-a", "tok-b"] {
        client
            .start_signing_job()
            .source(
                aws_sdk_signer::types::Source::builder()
                    .s3(aws_sdk_signer::types::S3Source::builder()
                        .bucket_name("b")
                        .key("k")
                        .version("v")
                        .build()
                        .unwrap())
                    .build(),
            )
            .destination(
                aws_sdk_signer::types::Destination::builder()
                    .s3(aws_sdk_signer::types::S3Destination::builder()
                        .bucket_name("d")
                        .build())
                    .build(),
            )
            .profile_name("list-jobs-profile")
            .client_request_token(token)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_signing_jobs()
        .send()
        .await
        .expect("list_signing_jobs should succeed");

    assert_eq!(resp.jobs().len(), 2, "Should have 2 jobs");
}

#[tokio::test]
async fn test_revoke_signature() {
    let client = make_client().await;

    client
        .put_signing_profile()
        .profile_name("revoke-sig-profile")
        .platform_id("AWSLambda-SHA384-ECDSA")
        .send()
        .await
        .unwrap();

    let start_resp = client
        .start_signing_job()
        .source(
            aws_sdk_signer::types::Source::builder()
                .s3(aws_sdk_signer::types::S3Source::builder()
                    .bucket_name("b")
                    .key("k")
                    .version("v")
                    .build()
                    .unwrap())
                .build(),
        )
        .destination(
            aws_sdk_signer::types::Destination::builder()
                .s3(aws_sdk_signer::types::S3Destination::builder()
                    .bucket_name("d")
                    .build())
                .build(),
        )
        .profile_name("revoke-sig-profile")
        .client_request_token("tok-rev")
        .send()
        .await
        .unwrap();

    let job_id = start_resp.job_id().unwrap().to_string();

    client
        .revoke_signature()
        .job_id(&job_id)
        .reason("Test revocation")
        .send()
        .await
        .expect("revoke_signature should succeed");

    let desc = client
        .describe_signing_job()
        .job_id(&job_id)
        .send()
        .await
        .unwrap();
    let status_str = desc.status().map(|s| s.as_str()).unwrap_or("");
    assert_eq!(
        status_str, "Revoked",
        "Job should be Revoked after revoke_signature, got: {status_str}"
    );
}

#[tokio::test]
async fn test_add_and_list_profile_permissions() {
    let client = make_client().await;

    client
        .put_signing_profile()
        .profile_name("perm-profile")
        .platform_id("AWSLambda-SHA384-ECDSA")
        .send()
        .await
        .unwrap();

    let add_resp = client
        .add_profile_permission()
        .profile_name("perm-profile")
        .statement_id("stmt-1")
        .action("signer:StartSigningJob")
        .principal("123456789012")
        .send()
        .await
        .expect("add_profile_permission should succeed");

    assert!(add_resp.revision_id().is_some(), "Should return revisionId");

    let list_resp = client
        .list_profile_permissions()
        .profile_name("perm-profile")
        .send()
        .await
        .expect("list_profile_permissions should succeed");

    assert_eq!(list_resp.permissions().len(), 1, "Should have 1 permission");
    assert_eq!(list_resp.permissions()[0].statement_id(), Some("stmt-1"));
    assert_eq!(
        list_resp.permissions()[0].action(),
        Some("signer:StartSigningJob")
    );
}

#[tokio::test]
async fn test_remove_profile_permission() {
    let client = make_client().await;

    client
        .put_signing_profile()
        .profile_name("remove-perm-profile")
        .platform_id("AWSLambda-SHA384-ECDSA")
        .send()
        .await
        .unwrap();

    // Add two permissions
    for stmt in ["stmt-a", "stmt-b"] {
        client
            .add_profile_permission()
            .profile_name("remove-perm-profile")
            .statement_id(stmt)
            .action("signer:StartSigningJob")
            .principal("111111111111")
            .send()
            .await
            .unwrap();
    }

    let list_before = client
        .list_profile_permissions()
        .profile_name("remove-perm-profile")
        .send()
        .await
        .unwrap();
    assert_eq!(list_before.permissions().len(), 2);
    let rev_id = list_before.revision_id().unwrap_or("").to_string();

    // Remove one
    let remove_resp = client
        .remove_profile_permission()
        .profile_name("remove-perm-profile")
        .statement_id("stmt-a")
        .revision_id(&rev_id)
        .send()
        .await
        .expect("remove_profile_permission should succeed");

    assert!(remove_resp.revision_id().is_some());

    let list_after = client
        .list_profile_permissions()
        .profile_name("remove-perm-profile")
        .send()
        .await
        .unwrap();
    assert_eq!(list_after.permissions().len(), 1);
    assert_eq!(list_after.permissions()[0].statement_id(), Some("stmt-b"));
}

#[tokio::test]
async fn test_revoke_signing_profile() {
    let client = make_client().await;

    let put_resp = client
        .put_signing_profile()
        .profile_name("revoke-profile")
        .platform_id("AWSLambda-SHA384-ECDSA")
        .send()
        .await
        .unwrap();
    let profile_version = put_resp.profile_version().unwrap().to_string();

    client
        .revoke_signing_profile()
        .profile_name("revoke-profile")
        .profile_version(&profile_version)
        .reason("Test revocation")
        .effective_time(aws_smithy_types::DateTime::from_secs(9999999999))
        .send()
        .await
        .expect("revoke_signing_profile should succeed");

    let get_resp = client
        .get_signing_profile()
        .profile_name("revoke-profile")
        .send()
        .await
        .unwrap();
    assert_eq!(
        get_resp.status(),
        Some(&aws_sdk_signer::types::SigningProfileStatus::Revoked),
        "Profile should be Revoked after revoke_signing_profile"
    );
}

#[tokio::test]
async fn test_sign_payload() {
    let client = make_client().await;

    client
        .put_signing_profile()
        .profile_name("payload-profile")
        .platform_id("AWSLambda-SHA384-ECDSA")
        .send()
        .await
        .unwrap();

    let resp = client
        .sign_payload()
        .profile_name("payload-profile")
        .payload(aws_smithy_types::Blob::new(b"hello".as_ref()))
        .payload_format("application/vnd.cncf.notary.payload.v1+json")
        .send()
        .await
        .expect("sign_payload should succeed");

    assert!(resp.job_id().is_some(), "Should return jobId");
    assert!(resp.signature().is_some(), "Should return signature");
}

#[tokio::test]
async fn test_get_revocation_status() {
    let client = make_client().await;

    // GetRevocationStatus should succeed (returns empty revoked entities list)
    let resp = client
        .get_revocation_status()
        .signature_timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .platform_id("AWSLambda-SHA384-ECDSA")
        .profile_version_arn("arn:aws:signer:us-east-1:123456789012:/signing-profiles/test/abc123")
        .job_arn("arn:aws:signer:us-east-1:123456789012:/signing-jobs/job-123")
        .certificate_hashes("aabbcc")
        .send()
        .await
        .expect("get_revocation_status should succeed");

    // Returns a (possibly empty) list of revoked entities
    let _ = resp.revoked_entities();
}

#[tokio::test]
async fn test_start_signing_job_nonexistent_profile() {
    let client = make_client().await;

    let result = client
        .start_signing_job()
        .source(
            aws_sdk_signer::types::Source::builder()
                .s3(aws_sdk_signer::types::S3Source::builder()
                    .bucket_name("b")
                    .key("k")
                    .version("v")
                    .build()
                    .unwrap())
                .build(),
        )
        .destination(
            aws_sdk_signer::types::Destination::builder()
                .s3(aws_sdk_signer::types::S3Destination::builder()
                    .bucket_name("d")
                    .build())
                .build(),
        )
        .profile_name("no-such-profile")
        .client_request_token("tok-x")
        .send()
        .await;
    assert!(result.is_err(), "Expected error for nonexistent profile");
}

#[tokio::test]
async fn test_list_profile_permissions_nonexistent_profile() {
    let client = make_client().await;

    let result = client
        .list_profile_permissions()
        .profile_name("no-such-profile")
        .send()
        .await;
    assert!(result.is_err(), "Expected error for nonexistent profile");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}
