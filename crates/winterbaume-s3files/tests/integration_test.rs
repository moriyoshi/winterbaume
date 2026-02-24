use std::collections::HashMap;

use aws_sdk_s3files::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_s3files::S3FilesService;
use winterbaume_s3files::views::FileSystemView;

async fn make_client() -> aws_sdk_s3files::Client {
    let mock = MockAws::builder()
        .with_service(S3FilesService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3files::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_s3files::Client::new(&config)
}

fn fs_view(id: &str, bucket: &str) -> FileSystemView {
    FileSystemView {
        file_system_id: id.to_string(),
        file_system_arn: format!("arn:aws:s3files:us-east-1:123456789012:file-system/{id}"),
        bucket: bucket.to_string(),
        prefix: None,
        kms_key_id: None,
        role_arn: "arn:aws:iam::123456789012:role/Seed".to_string(),
        client_token: None,
        name: None,
        status: "available".to_string(),
        status_message: None,
        creation_time: chrono::Utc::now(),
        owner_id: "123456789012".to_string(),
        tags: HashMap::new(),
        policy: None,
        synchronization_configuration: Default::default(),
    }
}

// ── File system lifecycle ─────────────────────────────────────────

#[tokio::test]
async fn test_create_and_get_file_system() {
    let client = make_client().await;

    let resp = client
        .create_file_system()
        .bucket("my-bucket")
        .role_arn("arn:aws:iam::123456789012:role/S3FilesRole")
        .send()
        .await
        .expect("create_file_system should succeed");

    let fs_id = resp.file_system_id().expect("file_system_id");
    assert!(fs_id.starts_with("fs-"));
    assert_eq!(resp.bucket(), Some("my-bucket"));
    assert_eq!(
        resp.status(),
        Some(&aws_sdk_s3files::types::LifeCycleState::Available)
    );

    let got = client
        .get_file_system()
        .file_system_id(fs_id)
        .send()
        .await
        .expect("get_file_system should succeed");

    assert_eq!(got.file_system_id(), Some(fs_id));
    assert_eq!(got.bucket(), Some("my-bucket"));
    assert_eq!(
        got.role_arn(),
        Some("arn:aws:iam::123456789012:role/S3FilesRole")
    );
}

#[tokio::test]
async fn test_create_file_system_with_optional_fields() {
    let client = make_client().await;

    let resp = client
        .create_file_system()
        .bucket("my-bucket")
        .prefix("data/")
        .role_arn("arn:aws:iam::123456789012:role/Role")
        .kms_key_id("alias/aws/s3")
        .client_token("idem-1")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.prefix(), "data/");
    assert_eq!(resp.kms_key_id(), Some("alias/aws/s3"));
    assert_eq!(resp.client_token(), Some("idem-1"));
}

#[tokio::test]
async fn test_create_file_system_idempotency_token() {
    let client = make_client().await;

    let first = client
        .create_file_system()
        .bucket("b")
        .role_arn("arn:aws:iam::123456789012:role/Role")
        .client_token("repeat")
        .send()
        .await
        .unwrap();

    let second = client
        .create_file_system()
        .bucket("b")
        .role_arn("arn:aws:iam::123456789012:role/Role")
        .client_token("repeat")
        .send()
        .await
        .unwrap();

    assert_eq!(first.file_system_id(), second.file_system_id());
}

#[tokio::test]
async fn test_create_file_system_missing_bucket() {
    let client = make_client().await;

    let result = client
        .create_file_system()
        .bucket("")
        .role_arn("arn:aws:iam::123456789012:role/Role")
        .send()
        .await;

    assert!(result.is_err(), "empty bucket should fail validation");
}

#[tokio::test]
async fn test_get_file_system_not_found() {
    let client = make_client().await;

    let result = client
        .get_file_system()
        .file_system_id("fs-doesnotexist")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_file_systems() {
    let client = make_client().await;

    for i in 0..3 {
        client
            .create_file_system()
            .bucket(format!("bucket-{i}"))
            .role_arn("arn:aws:iam::123456789012:role/Role")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_file_systems()
        .send()
        .await
        .expect("list_file_systems should succeed");

    assert_eq!(resp.file_systems().len(), 3);
}

#[tokio::test]
async fn test_list_file_systems_filter_by_bucket() {
    let client = make_client().await;

    client
        .create_file_system()
        .bucket("alpha")
        .role_arn("arn:aws:iam::123456789012:role/Role")
        .send()
        .await
        .unwrap();
    client
        .create_file_system()
        .bucket("beta")
        .role_arn("arn:aws:iam::123456789012:role/Role")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_file_systems()
        .bucket("alpha")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.file_systems().len(), 1);
    assert_eq!(resp.file_systems()[0].bucket(), "alpha");
}

#[tokio::test]
async fn test_delete_file_system() {
    let client = make_client().await;

    let created = client
        .create_file_system()
        .bucket("doomed")
        .role_arn("arn:aws:iam::123456789012:role/Role")
        .send()
        .await
        .unwrap();

    let fs_id = created.file_system_id().unwrap();

    client
        .delete_file_system()
        .file_system_id(fs_id)
        .send()
        .await
        .expect("delete_file_system should succeed");

    let result = client.get_file_system().file_system_id(fs_id).send().await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_delete_file_system_not_found() {
    let client = make_client().await;

    let result = client
        .delete_file_system()
        .file_system_id("fs-doesnotexist")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_file_system_in_use_returns_conflict() {
    // Documented constraint: associated mount targets must be deleted first.
    let client = make_client().await;

    let fs = client
        .create_file_system()
        .bucket("inuse")
        .role_arn("arn:aws:iam::123456789012:role/Role")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().unwrap();

    client
        .create_mount_target()
        .file_system_id(fs_id)
        .subnet_id("subnet-vpcA-az1")
        .send()
        .await
        .unwrap();

    let result = client
        .delete_file_system()
        .file_system_id(fs_id)
        .send()
        .await;

    assert!(
        result.is_err(),
        "delete with attached mount target should fail"
    );
}

// ── Tag operations ────────────────────────────────────────────────

#[tokio::test]
async fn test_tag_and_list_tags_for_resource() {
    let client = make_client().await;

    let fs = client
        .create_file_system()
        .bucket("tagged")
        .role_arn("arn:aws:iam::123456789012:role/Role")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().unwrap();

    let tag1 = aws_sdk_s3files::types::Tag::builder()
        .key("env")
        .value("prod")
        .build()
        .unwrap();
    let tag2 = aws_sdk_s3files::types::Tag::builder()
        .key("team")
        .value("storage")
        .build()
        .unwrap();

    client
        .tag_resource()
        .resource_id(fs_id)
        .tags(tag1)
        .tags(tag2)
        .send()
        .await
        .expect("tag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_id(fs_id)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tags().len(), 2);
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_client().await;

    let fs = client
        .create_file_system()
        .bucket("untagged")
        .role_arn("arn:aws:iam::123456789012:role/Role")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().unwrap();

    let tag = aws_sdk_s3files::types::Tag::builder()
        .key("env")
        .value("prod")
        .build()
        .unwrap();

    client
        .tag_resource()
        .resource_id(fs_id)
        .tags(tag)
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_id(fs_id)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_id(fs_id)
        .send()
        .await
        .unwrap();

    assert!(resp.tags().is_empty());
}

#[tokio::test]
async fn test_tag_resource_not_found() {
    let client = make_client().await;

    let tag = aws_sdk_s3files::types::Tag::builder()
        .key("env")
        .value("prod")
        .build()
        .unwrap();

    let result = client
        .tag_resource()
        .resource_id("fs-missing")
        .tags(tag)
        .send()
        .await;

    assert!(result.is_err());
}

// ── Mount target operations ───────────────────────────────────────

async fn create_test_fs(client: &aws_sdk_s3files::Client, bucket: &str) -> String {
    client
        .create_file_system()
        .bucket(bucket)
        .role_arn("arn:aws:iam::123456789012:role/Role")
        .send()
        .await
        .unwrap()
        .file_system_id()
        .unwrap()
        .to_string()
}

#[tokio::test]
async fn test_create_and_get_mount_target() {
    let client = make_client().await;
    let fs_id = create_test_fs(&client, "mt-bucket").await;

    let mt = client
        .create_mount_target()
        .file_system_id(&fs_id)
        .subnet_id("subnet-vpcA-az1")
        .send()
        .await
        .expect("create_mount_target should succeed");

    let mt_id = mt.mount_target_id();
    assert!(mt_id.starts_with("mt-"));
    assert!(
        mt.network_interface_id()
            .unwrap_or_default()
            .starts_with("eni-")
    );
    assert_eq!(mt.vpc_id(), Some("vpc-vpcA"));

    let got = client
        .get_mount_target()
        .mount_target_id(mt_id)
        .send()
        .await
        .unwrap();

    assert_eq!(got.mount_target_id(), mt_id);
    assert_eq!(got.subnet_id(), "subnet-vpcA-az1");
}

#[tokio::test]
async fn test_create_mount_target_az_conflict() {
    let client = make_client().await;
    let fs_id = create_test_fs(&client, "az-conflict").await;

    client
        .create_mount_target()
        .file_system_id(&fs_id)
        .subnet_id("subnet-vpcA-az1")
        .send()
        .await
        .unwrap();

    // Same AZ ( derived from the subnet id ) — should fail.
    let result = client
        .create_mount_target()
        .file_system_id(&fs_id)
        .subnet_id("subnet-vpcA-az1")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_mount_target_vpc_conflict() {
    let client = make_client().await;
    let fs_id = create_test_fs(&client, "vpc-conflict").await;

    client
        .create_mount_target()
        .file_system_id(&fs_id)
        .subnet_id("subnet-vpcA-az1")
        .send()
        .await
        .unwrap();

    // Different VPC ( derived from a different vpc tag in the subnet id ).
    let result = client
        .create_mount_target()
        .file_system_id(&fs_id)
        .subnet_id("subnet-vpcB-az2")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_mount_targets_by_file_system() {
    let client = make_client().await;
    let fs_id = create_test_fs(&client, "list-mt").await;

    for az in &["az1", "az2", "az3"] {
        client
            .create_mount_target()
            .file_system_id(&fs_id)
            .subnet_id(format!("subnet-vpcL-{az}"))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_mount_targets()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.mount_targets().len(), 3);
}

#[tokio::test]
async fn test_update_mount_target_security_groups() {
    let client = make_client().await;
    let fs_id = create_test_fs(&client, "upd-mt").await;

    let mt = client
        .create_mount_target()
        .file_system_id(&fs_id)
        .subnet_id("subnet-vpcU-az1")
        .send()
        .await
        .unwrap();
    let mt_id = mt.mount_target_id();

    let updated = client
        .update_mount_target()
        .mount_target_id(mt_id)
        .security_groups("sg-1")
        .security_groups("sg-2")
        .send()
        .await
        .expect("update_mount_target should succeed");

    assert_eq!(updated.security_groups().len(), 2);
}

#[tokio::test]
async fn test_delete_mount_target_lifecycle() {
    let client = make_client().await;
    let fs_id = create_test_fs(&client, "del-mt").await;

    let mt = client
        .create_mount_target()
        .file_system_id(&fs_id)
        .subnet_id("subnet-vpcD-az1")
        .send()
        .await
        .unwrap();
    let mt_id = mt.mount_target_id();

    client
        .delete_mount_target()
        .mount_target_id(mt_id)
        .send()
        .await
        .expect("delete_mount_target should succeed");

    let result = client
        .get_mount_target()
        .mount_target_id(mt_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_mount_target_not_found() {
    let client = make_client().await;

    let result = client
        .get_mount_target()
        .mount_target_id("mt-missing")
        .send()
        .await;
    assert!(result.is_err());
}

// ── Access point operations ───────────────────────────────────────

#[tokio::test]
async fn test_create_and_get_access_point() {
    let client = make_client().await;
    let fs_id = create_test_fs(&client, "ap-bucket").await;

    let posix = aws_sdk_s3files::types::PosixUser::builder()
        .uid(1000)
        .gid(1000)
        .build()
        .unwrap();

    let root = aws_sdk_s3files::types::RootDirectory::builder()
        .path("/data")
        .build();

    let ap = client
        .create_access_point()
        .file_system_id(&fs_id)
        .posix_user(posix)
        .root_directory(root)
        .send()
        .await
        .expect("create_access_point should succeed");

    let ap_id = ap.access_point_id();
    assert!(ap_id.starts_with("ap-"));
    assert!(ap.access_point_arn().contains("access-point/"));

    let got = client
        .get_access_point()
        .access_point_id(ap_id)
        .send()
        .await
        .unwrap();

    assert_eq!(got.access_point_id(), ap_id);
    assert_eq!(got.file_system_id(), fs_id.as_str());
    assert_eq!(got.posix_user().unwrap().uid(), 1000);
    assert_eq!(got.root_directory().unwrap().path(), Some("/data"));
}

#[tokio::test]
async fn test_create_access_point_idempotency_token() {
    let client = make_client().await;
    let fs_id = create_test_fs(&client, "ap-idem").await;

    let first = client
        .create_access_point()
        .file_system_id(&fs_id)
        .client_token("ap-token")
        .send()
        .await
        .unwrap();
    let second = client
        .create_access_point()
        .file_system_id(&fs_id)
        .client_token("ap-token")
        .send()
        .await
        .unwrap();

    assert_eq!(first.access_point_id(), second.access_point_id());
}

#[tokio::test]
async fn test_create_access_point_unknown_file_system() {
    let client = make_client().await;

    let result = client
        .create_access_point()
        .file_system_id("fs-missing")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_access_points() {
    let client = make_client().await;
    let fs_id = create_test_fs(&client, "list-ap").await;

    for _ in 0..3 {
        client
            .create_access_point()
            .file_system_id(&fs_id)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_access_points()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.access_points().len(), 3);
}

#[tokio::test]
async fn test_delete_access_point_lifecycle() {
    let client = make_client().await;
    let fs_id = create_test_fs(&client, "del-ap").await;

    let ap = client
        .create_access_point()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    let ap_id = ap.access_point_id();

    client
        .delete_access_point()
        .access_point_id(ap_id)
        .send()
        .await
        .expect("delete_access_point should succeed");

    let result = client
        .get_access_point()
        .access_point_id(ap_id)
        .send()
        .await;
    assert!(result.is_err());
}

// ── File system policy operations ─────────────────────────────────

#[tokio::test]
async fn test_put_get_delete_file_system_policy() {
    let client = make_client().await;
    let fs_id = create_test_fs(&client, "policy-fs").await;

    let policy_doc = r#"{"Version":"2012-10-17","Statement":[]}"#;

    client
        .put_file_system_policy()
        .file_system_id(&fs_id)
        .policy(policy_doc)
        .send()
        .await
        .expect("put_file_system_policy should succeed");

    let got = client
        .get_file_system_policy()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    assert_eq!(got.policy(), policy_doc);

    client
        .delete_file_system_policy()
        .file_system_id(&fs_id)
        .send()
        .await
        .expect("delete_file_system_policy should succeed");

    let after = client
        .get_file_system_policy()
        .file_system_id(&fs_id)
        .send()
        .await;
    assert!(after.is_err(), "policy should be gone after delete");
}

#[tokio::test]
async fn test_put_file_system_policy_too_large() {
    let client = make_client().await;
    let fs_id = create_test_fs(&client, "policy-large").await;

    let huge_policy = "x".repeat(20_001);
    let result = client
        .put_file_system_policy()
        .file_system_id(&fs_id)
        .policy(huge_policy)
        .send()
        .await;

    assert!(result.is_err(), "policy >20K chars should be rejected");
}

// ── Synchronization configuration operations ──────────────────────

#[tokio::test]
async fn test_put_get_synchronization_configuration() {
    let client = make_client().await;
    let fs_id = create_test_fs(&client, "sync-fs").await;

    let import_rule = aws_sdk_s3files::types::ImportDataRule::builder()
        .prefix("data/")
        .trigger(aws_sdk_s3files::types::ImportTrigger::OnFileAccess)
        .size_less_than(1024)
        .build()
        .unwrap();

    let expiration_rule = aws_sdk_s3files::types::ExpirationDataRule::builder()
        .days_after_last_access(30)
        .build()
        .unwrap();

    client
        .put_synchronization_configuration()
        .file_system_id(&fs_id)
        .import_data_rules(import_rule)
        .expiration_data_rules(expiration_rule)
        .send()
        .await
        .expect("put_synchronization_configuration should succeed");

    let got = client
        .get_synchronization_configuration()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();

    assert_eq!(got.latest_version_number(), Some(1));
    assert_eq!(got.import_data_rules().len(), 1);
    assert_eq!(got.expiration_data_rules().len(), 1);
}

#[tokio::test]
async fn test_put_synchronization_configuration_version_conflict() {
    let client = make_client().await;
    let fs_id = create_test_fs(&client, "sync-conflict").await;

    // First write at version 0 ( fresh file system ).
    client
        .put_synchronization_configuration()
        .file_system_id(&fs_id)
        .latest_version_number(0)
        .send()
        .await
        .unwrap();

    // Second write with stale version 0 ( current is now 1 ) — should conflict.
    let result = client
        .put_synchronization_configuration()
        .file_system_id(&fs_id)
        .latest_version_number(0)
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_put_synchronization_configuration_too_many_rules() {
    let client = make_client().await;
    let fs_id = create_test_fs(&client, "sync-too-many").await;

    let mut req = client
        .put_synchronization_configuration()
        .file_system_id(&fs_id);
    for i in 0..11 {
        let rule = aws_sdk_s3files::types::ImportDataRule::builder()
            .prefix(format!("p{i}/"))
            .trigger(aws_sdk_s3files::types::ImportTrigger::OnFileAccess)
            .size_less_than(0)
            .build()
            .unwrap();
        req = req.import_data_rules(rule);
    }
    let result = req.send().await;
    assert!(result.is_err(), "more than 10 import rules should fail");
}

// ── State view round-trip ─────────────────────────────────────────

#[tokio::test]
async fn test_state_view_snapshot_restore() {
    use winterbaume_core::StatefulService;

    let svc = S3FilesService::new();

    let mut file_systems = HashMap::new();
    file_systems.insert("fs-seed".to_string(), fs_view("fs-seed", "seed-bucket"));
    let view = winterbaume_s3files::S3FilesStateView {
        file_systems,
        ..Default::default()
    };

    svc.restore("123456789012", "us-east-1", view.clone())
        .await
        .unwrap();

    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert!(snap.file_systems.contains_key("fs-seed"));
    assert_eq!(snap.file_systems["fs-seed"].bucket, "seed-bucket");
}

#[tokio::test]
async fn test_state_view_merge_is_additive() {
    use winterbaume_core::StatefulService;

    let svc = S3FilesService::new();

    let mut first = HashMap::new();
    first.insert("fs-a".to_string(), fs_view("fs-a", "a"));
    svc.restore(
        "123456789012",
        "us-east-1",
        winterbaume_s3files::S3FilesStateView {
            file_systems: first,
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let mut second = HashMap::new();
    second.insert("fs-b".to_string(), fs_view("fs-b", "b"));
    svc.merge(
        "123456789012",
        "us-east-1",
        winterbaume_s3files::S3FilesStateView {
            file_systems: second,
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert!(snap.file_systems.contains_key("fs-a"));
    assert!(snap.file_systems.contains_key("fs-b"));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;

    let svc = S3FilesService::new();
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

#[tokio::test]
async fn test_state_change_listener_snapshot_reflects_mutation() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;

    let svc = S3FilesService::new();

    let snapshots: Arc<Mutex<Vec<winterbaume_s3files::S3FilesStateView>>> =
        Arc::new(Mutex::new(vec![]));
    let snapshots2 = Arc::clone(&snapshots);
    svc.notifier().subscribe(move |_account_id, _region, view| {
        snapshots2.lock().unwrap().push(view.clone());
    });

    let mut file_systems = HashMap::new();
    file_systems.insert("fs-listen".to_string(), fs_view("fs-listen", "x"));
    svc.restore(
        "123456789012",
        "us-east-1",
        winterbaume_s3files::S3FilesStateView {
            file_systems,
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let got = snapshots.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert!(got[0].file_systems.contains_key("fs-listen"));
}
