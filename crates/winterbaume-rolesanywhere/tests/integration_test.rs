use aws_sdk_rolesanywhere::config::BehaviorVersion;
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

// ── Profile CRUD lifecycle ──────────────────────────────

#[tokio::test]
async fn test_create_and_get_profile() {
    let client = make_client().await;

    let resp = client
        .create_profile()
        .name("test-profile")
        .role_arns("arn:aws:iam::123456789012:role/TestRole")
        .send()
        .await
        .expect("create_profile should succeed");

    let profile = resp.profile().expect("profile should be present");
    assert_eq!(profile.name(), Some("test-profile"));
    assert!(profile.enabled() == Some(true));
    let profile_id = profile.profile_id().expect("profile_id should be present");

    let get_resp = client
        .get_profile()
        .profile_id(profile_id)
        .send()
        .await
        .expect("get_profile should succeed");

    let got = get_resp.profile().expect("profile should be present");
    assert_eq!(got.name(), Some("test-profile"));
    assert_eq!(got.profile_id(), Some(profile_id));
}

#[tokio::test]
async fn test_list_profiles() {
    let client = make_client().await;

    client
        .create_profile()
        .name("profile-1")
        .role_arns("arn:aws:iam::123456789012:role/R1")
        .send()
        .await
        .expect("create should succeed");

    client
        .create_profile()
        .name("profile-2")
        .role_arns("arn:aws:iam::123456789012:role/R2")
        .send()
        .await
        .expect("create should succeed");

    let resp = client
        .list_profiles()
        .send()
        .await
        .expect("list_profiles should succeed");

    let profiles = resp.profiles();
    assert_eq!(profiles.len(), 2);
}

#[tokio::test]
async fn test_delete_profile() {
    let client = make_client().await;

    let resp = client
        .create_profile()
        .name("to-delete")
        .role_arns("arn:aws:iam::123456789012:role/R1")
        .send()
        .await
        .expect("create should succeed");

    let profile_id = resp.profile().unwrap().profile_id().unwrap().to_string();

    client
        .delete_profile()
        .profile_id(&profile_id)
        .send()
        .await
        .expect("delete should succeed");

    let err = client.get_profile().profile_id(&profile_id).send().await;
    assert!(err.is_err(), "get_profile after delete should fail");
}

#[tokio::test]
async fn test_update_profile() {
    let client = make_client().await;

    let resp = client
        .create_profile()
        .name("original")
        .role_arns("arn:aws:iam::123456789012:role/R1")
        .send()
        .await
        .expect("create should succeed");

    let profile_id = resp.profile().unwrap().profile_id().unwrap().to_string();

    let updated = client
        .update_profile()
        .profile_id(&profile_id)
        .name("updated-name")
        .duration_seconds(7200)
        .send()
        .await
        .expect("update should succeed");

    let profile = updated.profile().unwrap();
    assert_eq!(profile.name(), Some("updated-name"));
    assert_eq!(profile.duration_seconds(), Some(7200));
}

#[tokio::test]
async fn test_enable_disable_profile() {
    let client = make_client().await;

    let resp = client
        .create_profile()
        .name("toggleable")
        .role_arns("arn:aws:iam::123456789012:role/R1")
        .send()
        .await
        .unwrap();

    let profile_id = resp.profile().unwrap().profile_id().unwrap().to_string();

    let disabled = client
        .disable_profile()
        .profile_id(&profile_id)
        .send()
        .await
        .unwrap();
    assert_eq!(disabled.profile().unwrap().enabled(), Some(false));

    let enabled = client
        .enable_profile()
        .profile_id(&profile_id)
        .send()
        .await
        .unwrap();
    assert_eq!(enabled.profile().unwrap().enabled(), Some(true));
}

// ── Trust Anchor CRUD lifecycle ──────────────────────────

#[tokio::test]
async fn test_create_and_get_trust_anchor() {
    use aws_sdk_rolesanywhere::types::{Source, SourceData, TrustAnchorType};

    let client = make_client().await;

    let resp = client
        .create_trust_anchor()
        .name("test-anchor")
        .source(
            Source::builder()
                .source_type(TrustAnchorType::CertificateBundle)
                .source_data(SourceData::X509CertificateData("CERT-DATA".to_string()))
                .build(),
        )
        .send()
        .await
        .expect("create should succeed");

    let ta = resp.trust_anchor().expect("trust_anchor should be present");
    assert_eq!(ta.name(), Some("test-anchor"));
    assert!(ta.enabled() == Some(true));

    let ta_id = ta.trust_anchor_id().unwrap().to_string();

    let get_resp = client
        .get_trust_anchor()
        .trust_anchor_id(&ta_id)
        .send()
        .await
        .expect("get should succeed");

    assert_eq!(get_resp.trust_anchor().unwrap().name(), Some("test-anchor"));
}

#[tokio::test]
async fn test_list_trust_anchors() {
    use aws_sdk_rolesanywhere::types::{Source, SourceData, TrustAnchorType};

    let client = make_client().await;

    client
        .create_trust_anchor()
        .name("anchor-1")
        .source(
            Source::builder()
                .source_type(TrustAnchorType::CertificateBundle)
                .source_data(SourceData::X509CertificateData("CERT".to_string()))
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_trust_anchors()
        .send()
        .await
        .expect("list should succeed");
    assert!(!resp.trust_anchors().is_empty());
}

#[tokio::test]
async fn test_delete_trust_anchor() {
    use aws_sdk_rolesanywhere::types::{Source, SourceData, TrustAnchorType};

    let client = make_client().await;

    let resp = client
        .create_trust_anchor()
        .name("to-delete")
        .source(
            Source::builder()
                .source_type(TrustAnchorType::CertificateBundle)
                .source_data(SourceData::X509CertificateData("CERT".to_string()))
                .build(),
        )
        .send()
        .await
        .unwrap();

    let ta_id = resp
        .trust_anchor()
        .unwrap()
        .trust_anchor_id()
        .unwrap()
        .to_string();

    client
        .delete_trust_anchor()
        .trust_anchor_id(&ta_id)
        .send()
        .await
        .unwrap();

    let err = client
        .get_trust_anchor()
        .trust_anchor_id(&ta_id)
        .send()
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_enable_disable_trust_anchor() {
    use aws_sdk_rolesanywhere::types::{Source, SourceData, TrustAnchorType};

    let client = make_client().await;

    let resp = client
        .create_trust_anchor()
        .name("toggleable")
        .source(
            Source::builder()
                .source_type(TrustAnchorType::CertificateBundle)
                .source_data(SourceData::X509CertificateData("CERT".to_string()))
                .build(),
        )
        .send()
        .await
        .unwrap();

    let ta_id = resp
        .trust_anchor()
        .unwrap()
        .trust_anchor_id()
        .unwrap()
        .to_string();

    let disabled = client
        .disable_trust_anchor()
        .trust_anchor_id(&ta_id)
        .send()
        .await
        .unwrap();
    assert_eq!(disabled.trust_anchor().unwrap().enabled(), Some(false));

    let enabled = client
        .enable_trust_anchor()
        .trust_anchor_id(&ta_id)
        .send()
        .await
        .unwrap();
    assert_eq!(enabled.trust_anchor().unwrap().enabled(), Some(true));
}

// ── CRL lifecycle ──────────────────────────────────

#[tokio::test]
async fn test_import_and_get_crl() {
    use aws_smithy_types::Blob;

    let client = make_client().await;

    let resp = client
        .import_crl()
        .name("test-crl")
        .crl_data(Blob::new(b"CRL-BINARY-DATA".to_vec()))
        .trust_anchor_arn("arn:aws:rolesanywhere:us-east-1:123456789012:trust-anchor/abc123")
        .send()
        .await
        .expect("import_crl should succeed");

    let crl = resp.crl().expect("crl should be present");
    assert_eq!(crl.name(), Some("test-crl"));

    let crl_id = crl.crl_id().unwrap().to_string();

    let get_resp = client
        .get_crl()
        .crl_id(&crl_id)
        .send()
        .await
        .expect("get_crl should succeed");

    assert_eq!(get_resp.crl().unwrap().name(), Some("test-crl"));
}

#[tokio::test]
async fn test_list_crls() {
    use aws_smithy_types::Blob;

    let client = make_client().await;

    client
        .import_crl()
        .name("crl-1")
        .crl_data(Blob::new(b"DATA".to_vec()))
        .trust_anchor_arn("arn:aws:rolesanywhere:us-east-1:123456789012:trust-anchor/abc123")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_crls()
        .send()
        .await
        .expect("list should succeed");
    assert!(!resp.crls().is_empty());
}

#[tokio::test]
async fn test_delete_crl() {
    use aws_smithy_types::Blob;

    let client = make_client().await;

    let resp = client
        .import_crl()
        .name("to-delete")
        .crl_data(Blob::new(b"DATA".to_vec()))
        .trust_anchor_arn("arn:aws:rolesanywhere:us-east-1:123456789012:trust-anchor/abc123")
        .send()
        .await
        .unwrap();

    let crl_id = resp.crl().unwrap().crl_id().unwrap().to_string();

    client.delete_crl().crl_id(&crl_id).send().await.unwrap();

    let err = client.get_crl().crl_id(&crl_id).send().await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_enable_disable_crl() {
    use aws_smithy_types::Blob;

    let client = make_client().await;

    let resp = client
        .import_crl()
        .name("toggleable")
        .crl_data(Blob::new(b"DATA".to_vec()))
        .trust_anchor_arn("arn:aws:rolesanywhere:us-east-1:123456789012:trust-anchor/abc123")
        .send()
        .await
        .unwrap();

    let crl_id = resp.crl().unwrap().crl_id().unwrap().to_string();

    let disabled = client.disable_crl().crl_id(&crl_id).send().await.unwrap();
    assert_eq!(disabled.crl().unwrap().enabled(), Some(false));

    let enabled = client.enable_crl().crl_id(&crl_id).send().await.unwrap();
    assert_eq!(enabled.crl().unwrap().enabled(), Some(true));
}

// ── Tag operations ──────────────────────────────────

#[tokio::test]
async fn test_tag_and_list_tags() {
    use aws_sdk_rolesanywhere::types::Tag;

    let client = make_client().await;

    let resp = client
        .create_profile()
        .name("tagged-profile")
        .role_arns("arn:aws:iam::123456789012:role/R1")
        .send()
        .await
        .unwrap();

    let profile_arn = resp.profile().unwrap().profile_arn().unwrap().to_string();

    client
        .tag_resource()
        .resource_arn(&profile_arn)
        .tags(Tag::builder().key("env").value("test").build().unwrap())
        .send()
        .await
        .expect("tag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&profile_arn)
        .send()
        .await
        .expect("list_tags should succeed");

    let tags = tags_resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "env");
    assert_eq!(tags[0].value(), "test");
}

#[tokio::test]
async fn test_untag_resource() {
    use aws_sdk_rolesanywhere::types::Tag;

    let client = make_client().await;

    let resp = client
        .create_profile()
        .name("to-untag")
        .role_arns("arn:aws:iam::123456789012:role/R1")
        .tags(Tag::builder().key("env").value("test").build().unwrap())
        .send()
        .await
        .unwrap();

    let profile_arn = resp.profile().unwrap().profile_arn().unwrap().to_string();

    client
        .untag_resource()
        .resource_arn(&profile_arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&profile_arn)
        .send()
        .await
        .unwrap();

    assert!(tags_resp.tags().is_empty());
}

// ── Subject operations ──────────────────────────────────

#[tokio::test]
async fn test_list_subjects_returns_empty() {
    let client = make_client().await;

    let resp = client
        .list_subjects()
        .send()
        .await
        .expect("list_subjects should succeed");
    // No subjects in mock
    assert!(resp.subjects().is_empty());
}

// ── Error paths ──────────────────────────────────

#[tokio::test]
async fn test_get_nonexistent_profile_returns_error() {
    let client = make_client().await;

    let err = client
        .get_profile()
        .profile_id("00000000-0000-0000-0000-000000000000")
        .send()
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_trust_anchor_returns_error() {
    let client = make_client().await;

    let err = client
        .get_trust_anchor()
        .trust_anchor_id("00000000-0000-0000-0000-000000000000")
        .send()
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_crl_returns_error() {
    let client = make_client().await;

    let err = client
        .get_crl()
        .crl_id("00000000-0000-0000-0000-000000000000")
        .send()
        .await;
    assert!(err.is_err());
}

// ── State views ──────────────────────────────────

#[tokio::test]
async fn test_snapshot_restore_roundtrip() {
    use std::collections::HashMap;

    use winterbaume_core::StatefulService;
    use winterbaume_rolesanywhere::RolesAnywhereStateView;
    use winterbaume_rolesanywhere::views::ProfileView;

    let svc = RolesAnywhereService::new();

    // Build a view with one profile
    let mut profiles = HashMap::new();
    profiles.insert(
        "snap-id".to_string(),
        ProfileView {
            profile_id: "snap-id".to_string(),
            profile_arn: "arn:aws:rolesanywhere:us-east-1:123456789012:profile/snap-id".to_string(),
            name: "snap-test".to_string(),
            enabled: true,
            role_arns: vec!["arn:aws:iam::123456789012:role/R1".to_string()],
            managed_policy_arns: vec![],
            session_policy: None,
            duration_seconds: None,
            require_instance_properties: None,
            accept_role_session_name: None,
            attribute_mappings: vec![],
            created_by: None,
            created_at: "2025-01-01T00:00:00+00:00".to_string(),
            updated_at: "2025-01-01T00:00:00+00:00".to_string(),
            tags: HashMap::new(),
        },
    );

    let view = RolesAnywhereStateView {
        profiles,
        ..Default::default()
    };

    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    // Snapshot and verify
    let snapped = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snapped.profiles.len(), 1);
    assert!(snapped.profiles.contains_key("snap-id"));

    // Restore into a fresh service
    let svc2 = RolesAnywhereService::new();
    svc2.restore("123456789012", "us-east-1", snapped.clone())
        .await
        .unwrap();

    let restored_view = svc2.snapshot("123456789012", "us-east-1").await;
    assert_eq!(restored_view.profiles.len(), 1);
}

#[tokio::test]
async fn test_merge_is_additive() {
    use std::collections::HashMap;

    use winterbaume_core::StatefulService;
    use winterbaume_rolesanywhere::RolesAnywhereStateView;
    use winterbaume_rolesanywhere::views::ProfileView;

    let svc = RolesAnywhereService::new();

    // Merge profile A
    let mut profiles_a = HashMap::new();
    profiles_a.insert(
        "id-a".to_string(),
        ProfileView {
            profile_id: "id-a".to_string(),
            profile_arn: "arn:aws:rolesanywhere:us-east-1:123456789012:profile/id-a".to_string(),
            name: "profile-a".to_string(),
            enabled: true,
            role_arns: vec![],
            managed_policy_arns: vec![],
            session_policy: None,
            duration_seconds: None,
            require_instance_properties: None,
            accept_role_session_name: None,
            attribute_mappings: vec![],
            created_by: None,
            created_at: "2025-01-01T00:00:00+00:00".to_string(),
            updated_at: "2025-01-01T00:00:00+00:00".to_string(),
            tags: HashMap::new(),
        },
    );

    svc.merge(
        "123456789012",
        "us-east-1",
        RolesAnywhereStateView {
            profiles: profiles_a,
            ..Default::default()
        },
    )
    .await
    .unwrap();

    // Merge profile B
    let mut profiles_b = HashMap::new();
    profiles_b.insert(
        "id-b".to_string(),
        ProfileView {
            profile_id: "id-b".to_string(),
            profile_arn: "arn:aws:rolesanywhere:us-east-1:123456789012:profile/id-b".to_string(),
            name: "profile-b".to_string(),
            enabled: true,
            role_arns: vec![],
            managed_policy_arns: vec![],
            session_policy: None,
            duration_seconds: None,
            require_instance_properties: None,
            accept_role_session_name: None,
            attribute_mappings: vec![],
            created_by: None,
            created_at: "2025-01-01T00:00:00+00:00".to_string(),
            updated_at: "2025-01-01T00:00:00+00:00".to_string(),
            tags: HashMap::new(),
        },
    );

    svc.merge(
        "123456789012",
        "us-east-1",
        RolesAnywhereStateView {
            profiles: profiles_b,
            ..Default::default()
        },
    )
    .await
    .unwrap();

    // Both should exist
    let view = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(view.profiles.len(), 2);
    assert!(view.profiles.contains_key("id-a"));
    assert!(view.profiles.contains_key("id-b"));
}

// ── State change notification ──────────────────────────

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;

    let svc = RolesAnywhereService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore("123456789012", "us-east-1", Default::default())
        .await
        .unwrap();

    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(
        got[0],
        ("123456789012".to_string(), "us-east-1".to_string())
    );
}
