use aws_sdk_efs::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_efs::EfsService;

async fn make_efs_client() -> aws_sdk_efs::Client {
    let mock = MockAws::builder().with_service(EfsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_efs::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_efs::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_describe_file_system() {
    let client = make_efs_client().await;

    let create_resp = client
        .create_file_system()
        .creation_token("test-token-1")
        .send()
        .await
        .expect("create_file_system should succeed");

    let fs_id = create_resp.file_system_id();
    assert!(
        fs_id.starts_with("fs-"),
        "file system id should start with fs-"
    );

    let describe_resp = client
        .describe_file_systems()
        .file_system_id(fs_id)
        .send()
        .await
        .expect("describe_file_systems should succeed");

    let file_systems = describe_resp.file_systems();
    assert_eq!(file_systems.len(), 1);
    assert_eq!(file_systems[0].file_system_id(), fs_id);
}

#[tokio::test]
async fn test_create_file_system_with_options() {
    let client = make_efs_client().await;

    let tag = aws_sdk_efs::types::Tag::builder()
        .key("Name")
        .value("my-test-fs")
        .build()
        .unwrap();

    let create_resp = client
        .create_file_system()
        .creation_token("test-token-opts")
        .performance_mode(aws_sdk_efs::types::PerformanceMode::GeneralPurpose)
        .encrypted(true)
        .tags(tag)
        .send()
        .await
        .expect("create_file_system with options should succeed");

    assert_eq!(create_resp.encrypted(), Some(true));
    assert_eq!(
        *create_resp.performance_mode(),
        aws_sdk_efs::types::PerformanceMode::GeneralPurpose
    );
}

#[tokio::test]
async fn test_delete_file_system() {
    let client = make_efs_client().await;

    let create_resp = client
        .create_file_system()
        .creation_token("test-token-del")
        .send()
        .await
        .unwrap();

    let fs_id = create_resp.file_system_id().to_string();

    client
        .delete_file_system()
        .file_system_id(&fs_id)
        .send()
        .await
        .expect("delete_file_system should succeed");

    // Verify it's gone
    let describe_resp = client
        .describe_file_systems()
        .file_system_id(&fs_id)
        .send()
        .await;
    assert!(describe_resp.is_err(), "describe after delete should fail");
}

#[tokio::test]
async fn test_describe_all_file_systems() {
    let client = make_efs_client().await;

    client
        .create_file_system()
        .creation_token("token-a")
        .send()
        .await
        .unwrap();

    client
        .create_file_system()
        .creation_token("token-b")
        .send()
        .await
        .unwrap();

    let describe_resp = client
        .describe_file_systems()
        .send()
        .await
        .expect("describe all file systems should succeed");

    assert_eq!(describe_resp.file_systems().len(), 2);
}

// Ported from moto: test_file_system.py::test_create_file_system_file_system_already_exists
#[tokio::test]
async fn test_create_file_system_already_exists() {
    let client = make_efs_client().await;

    client
        .create_file_system()
        .creation_token("idempotent-token")
        .send()
        .await
        .unwrap();

    let err = client
        .create_file_system()
        .creation_token("idempotent-token")
        .send()
        .await
        .unwrap_err();

    let service_err = err.into_service_error();
    assert!(service_err.is_file_system_already_exists());
}

#[tokio::test]
async fn test_delete_nonexistent_file_system() {
    let client = make_efs_client().await;

    let result = client
        .delete_file_system()
        .file_system_id("fs-nonexistent")
        .send()
        .await;
    assert!(result.is_err(), "delete nonexistent should fail");
}

#[tokio::test]
async fn test_create_mount_target() {
    let client = make_efs_client().await;

    let create_resp = client
        .create_file_system()
        .creation_token("mt-test-token")
        .send()
        .await
        .unwrap();

    let fs_id = create_resp.file_system_id().to_string();

    let mt_resp = client
        .create_mount_target()
        .file_system_id(&fs_id)
        .subnet_id("subnet-12345678")
        .send()
        .await
        .expect("create_mount_target should succeed");

    assert!(mt_resp.mount_target_id().starts_with("fsmt-"));
    assert_eq!(mt_resp.file_system_id(), fs_id);
}

#[tokio::test]
async fn test_describe_mount_targets() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("desc-mt-token")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    client
        .create_mount_target()
        .file_system_id(&fs_id)
        .subnet_id("subnet-aaa")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_mount_targets()
        .file_system_id(&fs_id)
        .send()
        .await
        .expect("describe_mount_targets should succeed");

    assert_eq!(resp.mount_targets().len(), 1);
    assert_eq!(resp.mount_targets()[0].file_system_id(), fs_id);
}

#[tokio::test]
async fn test_delete_mount_target() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("del-mt-token")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let mt = client
        .create_mount_target()
        .file_system_id(&fs_id)
        .subnet_id("subnet-bbb")
        .send()
        .await
        .unwrap();
    let mt_id = mt.mount_target_id().to_string();

    client
        .delete_mount_target()
        .mount_target_id(&mt_id)
        .send()
        .await
        .expect("delete_mount_target should succeed");

    // Verify it's gone
    let resp = client
        .describe_mount_targets()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.mount_targets().len(), 0);
}

#[tokio::test]
async fn test_delete_nonexistent_mount_target() {
    let client = make_efs_client().await;

    let result = client
        .delete_mount_target()
        .mount_target_id("fsmt-nonexistent")
        .send()
        .await;
    assert!(
        result.is_err(),
        "delete nonexistent mount target should fail"
    );
}

#[tokio::test]
async fn test_create_and_describe_access_point() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("ap-test-token")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let ap_resp = client
        .create_access_point()
        .client_token("ap-client-token-1")
        .file_system_id(&fs_id)
        .send()
        .await
        .expect("create_access_point should succeed");

    let ap_id = ap_resp.access_point_id().unwrap();
    assert!(ap_id.starts_with("fsap-"));
    assert_eq!(ap_resp.file_system_id().unwrap(), fs_id);
    assert_eq!(ap_resp.life_cycle_state().unwrap().as_str(), "available");

    // Describe by access point ID
    let desc_resp = client
        .describe_access_points()
        .access_point_id(ap_id)
        .send()
        .await
        .expect("describe_access_points should succeed");
    assert_eq!(desc_resp.access_points().len(), 1);
    assert_eq!(
        desc_resp.access_points()[0].access_point_id().unwrap(),
        ap_id
    );
}

#[tokio::test]
async fn test_create_access_point_with_posix_user() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("ap-posix-token")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let posix_user = aws_sdk_efs::types::PosixUser::builder()
        .uid(1000)
        .gid(1000)
        .build()
        .unwrap();

    let root_dir = aws_sdk_efs::types::RootDirectory::builder()
        .path("/data")
        .creation_info(
            aws_sdk_efs::types::CreationInfo::builder()
                .owner_uid(1000)
                .owner_gid(1000)
                .permissions("755")
                .build()
                .unwrap(),
        )
        .build();

    let ap_resp = client
        .create_access_point()
        .client_token("ap-posix-client")
        .file_system_id(&fs_id)
        .posix_user(posix_user)
        .root_directory(root_dir)
        .send()
        .await
        .expect("create_access_point with posix user should succeed");

    assert!(ap_resp.posix_user().is_some());
    assert_eq!(ap_resp.posix_user().unwrap().uid(), 1000);
    assert_eq!(ap_resp.posix_user().unwrap().gid(), 1000);
    assert!(ap_resp.root_directory().is_some());
    assert_eq!(ap_resp.root_directory().unwrap().path().unwrap(), "/data");
}

#[tokio::test]
async fn test_delete_access_point() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("ap-del-token")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let ap = client
        .create_access_point()
        .client_token("ap-del-client")
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    let ap_id = ap.access_point_id().unwrap().to_string();

    client
        .delete_access_point()
        .access_point_id(&ap_id)
        .send()
        .await
        .expect("delete_access_point should succeed");

    // Verify it's gone
    let desc = client
        .describe_access_points()
        .access_point_id(&ap_id)
        .send()
        .await;
    assert!(desc.is_err(), "describe deleted access point should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_access_point() {
    let client = make_efs_client().await;

    let result = client
        .delete_access_point()
        .access_point_id("fsap-nonexistent")
        .send()
        .await;
    assert!(
        result.is_err(),
        "delete nonexistent access point should fail"
    );
}

#[tokio::test]
async fn test_describe_access_points_by_file_system() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("ap-fs-filter")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    client
        .create_access_point()
        .client_token("ap-a")
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();

    client
        .create_access_point()
        .client_token("ap-b")
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_access_points()
        .file_system_id(&fs_id)
        .send()
        .await
        .expect("describe_access_points by fs should succeed");

    assert_eq!(resp.access_points().len(), 2);
}

#[tokio::test]
async fn test_describe_mount_target_security_groups() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("sg-test-token")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let mt = client
        .create_mount_target()
        .file_system_id(&fs_id)
        .subnet_id("subnet-sg-test")
        .security_groups("sg-12345678")
        .send()
        .await
        .unwrap();
    let mt_id = mt.mount_target_id().to_string();

    let resp = client
        .describe_mount_target_security_groups()
        .mount_target_id(&mt_id)
        .send()
        .await
        .expect("describe_mount_target_security_groups should succeed");

    assert_eq!(resp.security_groups().len(), 1);
    assert_eq!(resp.security_groups()[0], "sg-12345678");
}

#[tokio::test]
async fn test_put_and_describe_file_system_policy() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("policy-test-token")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let policy = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Action":"*","Principal":{"AWS":"*"}}]}"#;

    let put_resp = client
        .put_file_system_policy()
        .file_system_id(&fs_id)
        .policy(policy)
        .send()
        .await
        .expect("put_file_system_policy should succeed");

    assert_eq!(put_resp.file_system_id().unwrap(), fs_id);
    assert_eq!(put_resp.policy().unwrap(), policy);

    // Describe the policy
    let desc_resp = client
        .describe_file_system_policy()
        .file_system_id(&fs_id)
        .send()
        .await
        .expect("describe_file_system_policy should succeed");

    assert_eq!(desc_resp.file_system_id().unwrap(), fs_id);
    assert_eq!(desc_resp.policy().unwrap(), policy);
}

#[tokio::test]
async fn test_describe_file_system_policy_not_found() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("no-policy-token")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let result = client
        .describe_file_system_policy()
        .file_system_id(&fs_id)
        .send()
        .await;
    assert!(
        result.is_err(),
        "describe policy on fs without policy should fail"
    );
}

#[tokio::test]
async fn test_delete_file_system_policy() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("del-policy-token")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let policy = r#"{"Version":"2012-10-17","Statement":[]}"#;
    client
        .put_file_system_policy()
        .file_system_id(&fs_id)
        .policy(policy)
        .send()
        .await
        .unwrap();

    client
        .delete_file_system_policy()
        .file_system_id(&fs_id)
        .send()
        .await
        .expect("delete_file_system_policy should succeed");

    // Verify the policy is gone
    let result = client
        .describe_file_system_policy()
        .file_system_id(&fs_id)
        .send()
        .await;
    assert!(result.is_err(), "policy should be deleted");
}

#[tokio::test]
async fn test_list_tags_for_resource() {
    let client = make_efs_client().await;

    let tag = aws_sdk_efs::types::Tag::builder()
        .key("Environment")
        .value("test")
        .build()
        .unwrap();

    let fs = client
        .create_file_system()
        .creation_token("tags-list-token")
        .tags(tag)
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let resp = client
        .list_tags_for_resource()
        .resource_id(&fs_id)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), "Environment");
    assert_eq!(resp.tags()[0].value(), "test");
}

#[tokio::test]
async fn test_tag_and_untag_resource() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("tag-untag-token")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    // Tag the resource
    let tag1 = aws_sdk_efs::types::Tag::builder()
        .key("Env")
        .value("prod")
        .build()
        .unwrap();
    let tag2 = aws_sdk_efs::types::Tag::builder()
        .key("Team")
        .value("platform")
        .build()
        .unwrap();

    client
        .tag_resource()
        .resource_id(&fs_id)
        .tags(tag1)
        .tags(tag2)
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags
    let resp = client
        .list_tags_for_resource()
        .resource_id(&fs_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 2);

    // Untag the resource
    client
        .untag_resource()
        .resource_id(&fs_id)
        .tag_keys("Env")
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify tag removed
    let resp = client
        .list_tags_for_resource()
        .resource_id(&fs_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), "Team");
}

#[tokio::test]
async fn test_access_point_lifecycle() {
    let client = make_efs_client().await;

    // Create file system
    let fs = client
        .create_file_system()
        .creation_token("ap-lifecycle-token")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    // Create access point
    let ap = client
        .create_access_point()
        .client_token("lifecycle-ap-client")
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    let ap_id = ap.access_point_id().unwrap().to_string();

    // Describe it
    let desc = client
        .describe_access_points()
        .access_point_id(&ap_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.access_points().len(), 1);

    // Delete it
    client
        .delete_access_point()
        .access_point_id(&ap_id)
        .send()
        .await
        .unwrap();

    // Verify gone
    let desc = client
        .describe_access_points()
        .access_point_id(&ap_id)
        .send()
        .await;
    assert!(desc.is_err());
}

// ============================================================================
// Ported from moto: test_file_system.py
// ============================================================================

// Ported from moto: test_file_system.py::test_create_file_system_correct_use
#[tokio::test]
async fn test_moto_create_file_system_correct_use() {
    let client = make_efs_client().await;

    let resp = client
        .create_file_system()
        .creation_token("test_efs_create")
        .tags(
            aws_sdk_efs::types::Tag::builder()
                .key("Name")
                .value("Test EFS Container")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    assert_eq!(resp.creation_token(), "test_efs_create");
    assert!(resp.file_system_id().starts_with("fs-"));
    // creation_time is a required field (DateTime), just verify it's non-zero
    let _ = resp.creation_time();
    assert_eq!(resp.life_cycle_state().as_str(), "available");
    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), "Name");
    assert_eq!(resp.tags()[0].value(), "Test EFS Container");
    assert_eq!(resp.throughput_mode().unwrap().as_str(), "bursting");
    assert_eq!(resp.performance_mode().as_str(), "generalPurpose");
    assert_eq!(resp.encrypted(), Some(false));
    assert_eq!(resp.number_of_mount_targets(), 0);
    assert_eq!(resp.name(), Some("Test EFS Container"));

    // Check SizeInBytes
    let size = resp.size_in_bytes().unwrap();
    assert_eq!(size.value(), 0);
    assert_eq!(size.value_in_ia(), Some(0));
    assert_eq!(size.value_in_standard(), Some(0));

    // Check ARN
    let arn = resp.file_system_arn().unwrap();
    assert!(arn.starts_with("arn:aws:elasticfilesystem:"));
    assert!(arn.contains(":file-system/"));
    assert!(arn.contains(resp.file_system_id()));
}

// Ported from moto: test_file_system.py::test_create_file_system_aws_sample_1
#[tokio::test]
async fn test_moto_create_file_system_sample_1() {
    let client = make_efs_client().await;

    let resp = client
        .create_file_system()
        .creation_token("myFileSystem1")
        .performance_mode(aws_sdk_efs::types::PerformanceMode::GeneralPurpose)
        .encrypted(true)
        .tags(
            aws_sdk_efs::types::Tag::builder()
                .key("Name")
                .value("Test Group1")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), "Name");
    assert_eq!(resp.tags()[0].value(), "Test Group1");
    assert_eq!(resp.performance_mode().as_str(), "generalPurpose");
    assert_eq!(resp.encrypted(), Some(true));
    assert_eq!(resp.name(), Some("Test Group1"));
}

// Ported from moto: test_file_system.py::test_describe_file_systems_using_identifier
#[tokio::test]
async fn test_moto_describe_file_systems_using_identifier() {
    let client = make_efs_client().await;

    let create_resp = client
        .create_file_system()
        .creation_token("foobar")
        .send()
        .await
        .unwrap();
    let fs_id = create_resp.file_system_id().to_string();

    let desc_resp = client
        .describe_file_systems()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc_resp.file_systems().len(), 1);
    assert_eq!(desc_resp.file_systems()[0].file_system_id(), fs_id);
    // When no Name tag, name should be empty string
    assert_eq!(desc_resp.file_systems()[0].name(), Some(""));
}

// Ported from moto: test_file_system.py::test_describe_file_systems_using_unknown_identifier
#[tokio::test]
async fn test_moto_describe_file_systems_unknown_id() {
    let client = make_efs_client().await;

    let err = client
        .describe_file_systems()
        .file_system_id("unknown")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("FileSystemNotFound"),
        "Expected FileSystemNotFound error, got: {err_str}"
    );
}

// Ported from moto: test_file_system.py::test_describe_file_systems_minimal_case
#[tokio::test]
async fn test_moto_describe_file_systems_minimal() {
    let client = make_efs_client().await;

    let create_resp = client
        .create_file_system()
        .creation_token("foobar")
        .send()
        .await
        .unwrap();

    let desc_resp = client.describe_file_systems().send().await.unwrap();

    let fs_list = desc_resp.file_systems();
    assert_eq!(fs_list.len(), 1);
    let fs = &fs_list[0];
    assert_eq!(fs.file_system_id(), create_resp.file_system_id());
    assert_eq!(fs.name(), Some(""));
    assert_eq!(fs.throughput_mode().unwrap().as_str(), "bursting");
    assert!(fs.file_system_arn().is_some());
    assert!(!fs.owner_id().is_empty());
}

// Ported from moto: test_file_system.py::test_describe_file_systems_invalid_creation_token
#[tokio::test]
async fn test_moto_describe_file_systems_invalid_creation_token() {
    let client = make_efs_client().await;

    let resp = client
        .describe_file_systems()
        .creation_token("fizzle")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.file_systems().len(), 0);
}

// Ported from moto: test_file_system.py::test_describe_file_systems_invalid_file_system_id
#[tokio::test]
async fn test_moto_describe_file_systems_invalid_fs_id() {
    let client = make_efs_client().await;

    let err = client
        .describe_file_systems()
        .file_system_id("fs-29879313")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("FileSystemNotFound"));
}

// Ported from moto: test_file_system.py::test_delete_file_system_minimal_case
#[tokio::test]
async fn test_moto_delete_file_system_minimal() {
    let client = make_efs_client().await;

    let resp = client
        .create_file_system()
        .creation_token("del-test")
        .send()
        .await
        .unwrap();

    let desc1 = client.describe_file_systems().send().await.unwrap();
    assert_eq!(desc1.file_systems().len(), 1);

    client
        .delete_file_system()
        .file_system_id(resp.file_system_id())
        .send()
        .await
        .unwrap();

    let desc2 = client.describe_file_systems().send().await.unwrap();
    assert_eq!(desc2.file_systems().len(), 0);
}

// Ported from moto: test_file_system.py::test_delete_file_system_invalid_file_system_id
#[tokio::test]
async fn test_moto_delete_file_system_invalid_id() {
    let client = make_efs_client().await;

    let err = client
        .delete_file_system()
        .file_system_id("fs-2394287")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("FileSystemNotFound"));
}

// ============================================================================
// Ported from moto: test_access_points.py
// ============================================================================

// Ported from moto: test_access_points.py::test_describe_access_points__initial
#[tokio::test]
async fn test_moto_describe_access_points_initial() {
    let client = make_efs_client().await;

    let resp = client.describe_access_points().send().await.unwrap();
    assert_eq!(resp.access_points().len(), 0);
}

// Ported from moto: test_access_points.py::test_create_access_point__simple
#[tokio::test]
async fn test_moto_create_access_point_simple() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("foobarbaz")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let resp = client
        .create_access_point()
        .client_token("ct")
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.client_token().unwrap(), "ct");
    assert!(resp.access_point_id().is_some());
    assert!(resp.access_point_arn().is_some());
    assert_eq!(resp.file_system_id().unwrap(), fs_id);
    assert_eq!(resp.life_cycle_state().unwrap().as_str(), "available");

    // Default root directory should be {"Path": "/"}
    let rd = resp.root_directory().unwrap();
    assert_eq!(rd.path().unwrap(), "/");
    assert!(rd.creation_info().is_none());
}

// Ported from moto: test_access_points.py::test_create_access_point__full
#[tokio::test]
async fn test_moto_create_access_point_full() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("foobarbaz")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let posix_user = aws_sdk_efs::types::PosixUser::builder()
        .uid(123)
        .gid(123)
        .secondary_gids(124)
        .secondary_gids(125)
        .build()
        .unwrap();

    let root_dir = aws_sdk_efs::types::RootDirectory::builder()
        .path("/root/path")
        .creation_info(
            aws_sdk_efs::types::CreationInfo::builder()
                .owner_uid(987)
                .owner_gid(986)
                .permissions("root_permissions")
                .build()
                .unwrap(),
        )
        .build();

    let resp = client
        .create_access_point()
        .client_token("ct")
        .tags(
            aws_sdk_efs::types::Tag::builder()
                .key("key")
                .value("value")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_efs::types::Tag::builder()
                .key("Name")
                .value("myname")
                .build()
                .unwrap(),
        )
        .file_system_id(&fs_id)
        .posix_user(posix_user)
        .root_directory(root_dir)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.client_token().unwrap(), "ct");
    assert_eq!(resp.name().unwrap(), "myname");
    assert_eq!(resp.tags().len(), 2);
    assert!(resp.access_point_id().is_some());
    assert!(resp.access_point_arn().is_some());
    assert_eq!(resp.file_system_id().unwrap(), fs_id);

    let pu = resp.posix_user().unwrap();
    assert_eq!(pu.uid(), 123);
    assert_eq!(pu.gid(), 123);
    assert_eq!(pu.secondary_gids(), &[124, 125]);

    let rd = resp.root_directory().unwrap();
    assert_eq!(rd.path().unwrap(), "/root/path");
    let ci = rd.creation_info().unwrap();
    assert_eq!(ci.owner_uid(), 987);
    assert_eq!(ci.owner_gid(), 986);
    assert_eq!(ci.permissions(), "root_permissions");

    assert_eq!(resp.life_cycle_state().unwrap().as_str(), "available");
}

// Ported from moto: test_access_points.py::test_describe_access_point
#[tokio::test]
async fn test_moto_describe_access_point() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("foobarbaz")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let ap = client
        .create_access_point()
        .client_token("ct")
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    let ap_id = ap.access_point_id().unwrap().to_string();

    let resp = client
        .describe_access_points()
        .access_point_id(&ap_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.access_points().len(), 1);
    let access_point = &resp.access_points()[0];
    assert_eq!(access_point.client_token().unwrap(), "ct");
    assert!(access_point.access_point_id().is_some());
    assert!(access_point.access_point_arn().is_some());
    assert_eq!(access_point.file_system_id().unwrap(), fs_id);
    assert_eq!(
        access_point.life_cycle_state().unwrap().as_str(),
        "available"
    );
}

// Ported from moto: test_access_points.py::test_describe_access_points__multiple
#[tokio::test]
async fn test_moto_describe_access_points_multiple() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("foobarbaz")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    client
        .create_access_point()
        .client_token("ct1")
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();

    client
        .create_access_point()
        .client_token("ct2")
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();

    let resp = client.describe_access_points().send().await.unwrap();
    assert_eq!(resp.access_points().len(), 2);
}

// Ported from moto: test_access_points.py::test_describe_access_points__filters
#[tokio::test]
async fn test_moto_describe_access_points_filters() {
    let client = make_efs_client().await;

    let fs1 = client
        .create_file_system()
        .creation_token("foobarbaz")
        .send()
        .await
        .unwrap();
    let fs_id1 = fs1.file_system_id().to_string();

    let fs2 = client
        .create_file_system()
        .creation_token("bazbarfoo")
        .send()
        .await
        .unwrap();
    let fs_id2 = fs2.file_system_id().to_string();

    let ap1 = client
        .create_access_point()
        .client_token("ct")
        .file_system_id(&fs_id1)
        .send()
        .await
        .unwrap();
    let ap_id1 = ap1.access_point_id().unwrap().to_string();

    let ap2 = client
        .create_access_point()
        .client_token("ct")
        .file_system_id(&fs_id2)
        .send()
        .await
        .unwrap();
    let ap_id2 = ap2.access_point_id().unwrap().to_string();

    // Filter by file system ID
    let resp = client
        .describe_access_points()
        .file_system_id(&fs_id1)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.access_points().len(), 1);
    assert_eq!(resp.access_points()[0].file_system_id().unwrap(), fs_id1);
    assert_eq!(resp.access_points()[0].access_point_id().unwrap(), ap_id1);

    // Filter by access point ID
    let resp = client
        .describe_access_points()
        .access_point_id(&ap_id2)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.access_points().len(), 1);
    assert_eq!(resp.access_points()[0].file_system_id().unwrap(), fs_id2);
    assert_eq!(resp.access_points()[0].access_point_id().unwrap(), ap_id2);
}

// Ported from moto: test_access_points.py::test_delete_access_points
#[tokio::test]
async fn test_moto_delete_access_points() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("foobarbaz")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let ap1 = client
        .create_access_point()
        .client_token("ct1")
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    let ap_id1 = ap1.access_point_id().unwrap().to_string();

    let ap2 = client
        .create_access_point()
        .client_token("ct2")
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    let ap_id2 = ap2.access_point_id().unwrap().to_string();

    // Delete one
    client
        .delete_access_point()
        .access_point_id(&ap_id2)
        .send()
        .await
        .unwrap();

    // Only one remains
    let resp = client.describe_access_points().send().await.unwrap();
    assert_eq!(resp.access_points().len(), 1);

    // First still exists
    client
        .describe_access_points()
        .access_point_id(&ap_id1)
        .send()
        .await
        .unwrap();

    // Second is gone
    let err = client
        .describe_access_points()
        .access_point_id(&ap_id2)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("AccessPointNotFound"));
}

// ============================================================================
// Ported from moto: test_filesystem_policy.py
// ============================================================================

// Ported from moto: test_filesystem_policy.py::test_describe_file_system_policy__initial
#[tokio::test]
async fn test_moto_describe_file_system_policy_initial() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("foobar")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let err = client
        .describe_file_system_policy()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("PolicyNotFound"));
}

// Ported from moto: test_filesystem_policy.py::test_put_file_system_policy
#[tokio::test]
async fn test_moto_put_file_system_policy() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("foobar")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let policy = r#"{
  "Version" : "2012-10-17",
  "Id" : "efs-policy-1234",
  "Statement" : [ {
    "Sid" : "efs-statement-1234",
    "Effect" : "Allow",
    "Principal" : {
      "AWS" : "*"
    },
    "Action" : [ "elasticfilesystem:ClientRootAccess", "elasticfilesystem:ClientWrite" ],
    "Resource" : "arn:aws:elasticfilesystem:us-east-1:1234:file-system/fs-1234"
  } ]
}"#;
    let resp = client
        .put_file_system_policy()
        .file_system_id(&fs_id)
        .policy(policy)
        .send()
        .await
        .unwrap();
    assert!(resp.policy().unwrap().len() > 1);
    assert!(resp.policy().unwrap().contains("ClientRootAccess"));

    // Describe the policy
    let resp = client
        .describe_file_system_policy()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    assert!(resp.policy().unwrap().len() > 1);
    assert!(resp.policy().unwrap().contains("ClientRootAccess"));
}

// ============================================================================
// Ported from moto: test_filesystem_tagging.py
// ============================================================================

// Ported from moto: test_filesystem_tagging.py::test_list_tags_for_resource__without_tags
#[tokio::test]
async fn test_moto_list_tags_for_resource_without_tags() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("foobarbaz")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let resp = client
        .list_tags_for_resource()
        .resource_id(&fs_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 0);
}

// Ported from moto: test_filesystem_tagging.py::test_list_tags_for_resource__with_tags
#[tokio::test]
async fn test_moto_list_tags_for_resource_with_tags() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("foobarbaz")
        .tags(
            aws_sdk_efs::types::Tag::builder()
                .key("key")
                .value("value")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_efs::types::Tag::builder()
                .key("Name")
                .value("myname")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let resp = client
        .list_tags_for_resource()
        .resource_id(&fs_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 2);
    assert_eq!(resp.tags()[0].key(), "key");
    assert_eq!(resp.tags()[0].value(), "value");
    assert_eq!(resp.tags()[1].key(), "Name");
    assert_eq!(resp.tags()[1].value(), "myname");
}

// Ported from moto: test_filesystem_tagging.py::test_tag_resource
#[tokio::test]
async fn test_moto_tag_resource_fs() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("foobarbaz")
        .tags(
            aws_sdk_efs::types::Tag::builder()
                .key("key")
                .value("value")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_efs::types::Tag::builder()
                .key("Name")
                .value("myname")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    // Tag again with same tags (should be idempotent)
    client
        .tag_resource()
        .resource_id(&fs_id)
        .tags(
            aws_sdk_efs::types::Tag::builder()
                .key("key")
                .value("value")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_efs::types::Tag::builder()
                .key("Name")
                .value("myname")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_id(&fs_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 2);
    assert_eq!(resp.tags()[0].key(), "key");
    assert_eq!(resp.tags()[0].value(), "value");
    assert_eq!(resp.tags()[1].key(), "Name");
    assert_eq!(resp.tags()[1].value(), "myname");
}

// Ported from moto: test_filesystem_tagging.py::test_untag_resource
#[tokio::test]
async fn test_moto_untag_resource_fs() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("foobarbaz")
        .tags(
            aws_sdk_efs::types::Tag::builder()
                .key("key1")
                .value("val1")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    client
        .tag_resource()
        .resource_id(&fs_id)
        .tags(
            aws_sdk_efs::types::Tag::builder()
                .key("key2")
                .value("val2")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_efs::types::Tag::builder()
                .key("key3")
                .value("val3")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_id(&fs_id)
        .tag_keys("key2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_id(&fs_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 2);
    assert_eq!(resp.tags()[0].key(), "key1");
    assert_eq!(resp.tags()[0].value(), "val1");
    assert_eq!(resp.tags()[1].key(), "key3");
    assert_eq!(resp.tags()[1].value(), "val3");
}

// ============================================================================
// Ported from moto: test_access_point_tagging.py
// ============================================================================

// Ported from moto: test_access_point_tagging.py::test_list_tags_for_resource__without_tags
#[tokio::test]
async fn test_moto_ap_list_tags_without_tags() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("foobarbaz")
        .send()
        .await
        .unwrap();

    let ap = client
        .create_access_point()
        .client_token("ct")
        .file_system_id(fs.file_system_id())
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_id(ap.access_point_id().unwrap())
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 0);
}

// Ported from moto: test_access_point_tagging.py::test_list_tags_for_resource__with_tags
#[tokio::test]
async fn test_moto_ap_list_tags_with_tags() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("foobarbaz")
        .send()
        .await
        .unwrap();

    let ap = client
        .create_access_point()
        .client_token("ct")
        .tags(
            aws_sdk_efs::types::Tag::builder()
                .key("key")
                .value("value")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_efs::types::Tag::builder()
                .key("Name")
                .value("myname")
                .build()
                .unwrap(),
        )
        .file_system_id(fs.file_system_id())
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_id(ap.access_point_id().unwrap())
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 2);
    assert_eq!(resp.tags()[0].key(), "key");
    assert_eq!(resp.tags()[0].value(), "value");
    assert_eq!(resp.tags()[1].key(), "Name");
    assert_eq!(resp.tags()[1].value(), "myname");
}

// Ported from moto: test_access_point_tagging.py::test_tag_resource
#[tokio::test]
async fn test_moto_ap_tag_resource() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("foobarbaz")
        .send()
        .await
        .unwrap();

    let ap = client
        .create_access_point()
        .client_token("ct")
        .file_system_id(fs.file_system_id())
        .send()
        .await
        .unwrap();
    let ap_id = ap.access_point_id().unwrap().to_string();

    client
        .tag_resource()
        .resource_id(&ap_id)
        .tags(
            aws_sdk_efs::types::Tag::builder()
                .key("key")
                .value("value")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_efs::types::Tag::builder()
                .key("Name")
                .value("myname")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_id(&ap_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 2);
    assert_eq!(resp.tags()[0].key(), "key");
    assert_eq!(resp.tags()[0].value(), "value");
    assert_eq!(resp.tags()[1].key(), "Name");
    assert_eq!(resp.tags()[1].value(), "myname");
}

// Ported from moto: test_access_point_tagging.py::test_untag_resource
#[tokio::test]
async fn test_moto_ap_untag_resource() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("foobarbaz")
        .send()
        .await
        .unwrap();

    let ap = client
        .create_access_point()
        .client_token("ct")
        .tags(
            aws_sdk_efs::types::Tag::builder()
                .key("key1")
                .value("val1")
                .build()
                .unwrap(),
        )
        .file_system_id(fs.file_system_id())
        .send()
        .await
        .unwrap();
    let ap_id = ap.access_point_id().unwrap().to_string();

    client
        .tag_resource()
        .resource_id(&ap_id)
        .tags(
            aws_sdk_efs::types::Tag::builder()
                .key("key2")
                .value("val2")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_efs::types::Tag::builder()
                .key("key3")
                .value("val3")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_id(&ap_id)
        .tag_keys("key2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_id(&ap_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 2);
    assert_eq!(resp.tags()[0].key(), "key1");
    assert_eq!(resp.tags()[0].value(), "val1");
    assert_eq!(resp.tags()[1].key(), "key3");
    assert_eq!(resp.tags()[1].value(), "val3");
}

// ============================================================================
// Ported from moto: test_mount_target.py
// ============================================================================

// Ported from moto: test_mount_target.py::test_create_mount_target_minimal_correct_use
#[tokio::test]
async fn test_moto_create_mount_target_minimal() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("foobarbaz")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let mt = client
        .create_mount_target()
        .file_system_id(&fs_id)
        .subnet_id("subnet-12345678")
        .send()
        .await
        .unwrap();

    assert!(mt.mount_target_id().starts_with("fsmt-"));
    assert!(mt.network_interface_id().unwrap().starts_with("eni-"));
    assert_eq!(mt.file_system_id(), fs_id);
    assert_eq!(mt.life_cycle_state().as_str(), "available");
    assert!(mt.ip_address().is_some());

    // Check mount target count incremented
    let desc = client.describe_file_systems().send().await.unwrap();
    assert_eq!(desc.file_systems()[0].number_of_mount_targets(), 1);
}

// Ported from moto: test_mount_target.py::test_create_mount_target_invalid_file_system_id
#[tokio::test]
async fn test_moto_create_mount_target_invalid_fs() {
    let client = make_efs_client().await;

    let err = client
        .create_mount_target()
        .file_system_id("fs-12343289")
        .subnet_id("subnet-12345678")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("FileSystemNotFound"));
}

// Ported from moto: test_mount_target.py::test_describe_mount_targets_minimal_case
#[tokio::test]
async fn test_moto_describe_mount_targets_minimal() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("foobarbaz")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let create_resp = client
        .create_mount_target()
        .file_system_id(&fs_id)
        .subnet_id("subnet-aaa")
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_mount_targets()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.mount_targets().len(), 1);
    assert_eq!(
        desc.mount_targets()[0].mount_target_id(),
        create_resp.mount_target_id()
    );
}

// Ported from moto: test_mount_target.py::test_describe_mount_targets_invalid_file_system_id
#[tokio::test]
async fn test_moto_describe_mount_targets_invalid_fs() {
    let client = make_efs_client().await;

    let err = client
        .describe_mount_targets()
        .file_system_id("fs-12343289")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("FileSystemNotFound"));
}

// Ported from moto: test_mount_target.py::test_describe_mount_targets_invalid_mount_target_id
#[tokio::test]
async fn test_moto_describe_mount_targets_invalid_mt() {
    let client = make_efs_client().await;

    let err = client
        .describe_mount_targets()
        .mount_target_id("fsmt-ad9f8987")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("MountTargetNotFound"));
}

// Ported from moto: test_mount_target.py::test_delete_mount_target_minimal_case
#[tokio::test]
async fn test_moto_delete_mount_target_minimal() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("foobarbaz")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let mt = client
        .create_mount_target()
        .file_system_id(&fs_id)
        .subnet_id("subnet-aaa")
        .send()
        .await
        .unwrap();

    client
        .delete_mount_target()
        .mount_target_id(mt.mount_target_id())
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_mount_targets()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.mount_targets().len(), 0);
}

// Ported from moto: test_mount_target.py::test_delete_mount_target_invalid_mount_target_id
#[tokio::test]
async fn test_moto_delete_mount_target_invalid() {
    let client = make_efs_client().await;

    let err = client
        .delete_mount_target()
        .mount_target_id("fsmt-98487aef0a7")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("MountTargetNotFound"));
}

// Ported from moto: test_mount_target.py::test_delete_file_system_mount_targets_attached
#[tokio::test]
async fn test_moto_delete_file_system_with_mount_targets() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("foobarbaz")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    client
        .create_mount_target()
        .file_system_id(&fs_id)
        .subnet_id("subnet-aaa")
        .send()
        .await
        .unwrap();

    let err = client
        .delete_file_system()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("FileSystemInUse"));
}

// ============================================================================
// Ported from moto: test_mount_target_security_groups.py
// ============================================================================

// Ported from moto: test_mount_target_security_groups.py::test_describe_mount_target_security_groups__unknown
#[tokio::test]
async fn test_moto_describe_mt_sg_unknown() {
    let client = make_efs_client().await;

    let err = client
        .describe_mount_target_security_groups()
        .mount_target_id("mt-asdf1234asdf")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("MountTargetNotFound"));
}

// Ported from moto: test_mount_target_security_groups.py::test_describe_mount_target_security_groups
#[tokio::test]
async fn test_moto_describe_mt_security_groups() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("foobarbaz")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let mt = client
        .create_mount_target()
        .file_system_id(&fs_id)
        .subnet_id("subnet-aaa")
        .security_groups("sg-12345678")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_mount_target_security_groups()
        .mount_target_id(mt.mount_target_id())
        .send()
        .await
        .unwrap();
    assert_eq!(resp.security_groups(), &["sg-12345678"]);
}

// Ported from moto: test_mount_target_security_groups.py::test_modify_mount_target_security_groups__unknown
#[tokio::test]
async fn test_moto_modify_mt_sg_unknown() {
    let client = make_efs_client().await;

    let err = client
        .modify_mount_target_security_groups()
        .mount_target_id("mt-asdf1234asdf")
        .security_groups("sg-aaa")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("MountTargetNotFound"));
}

// Ported from moto: test_mount_target_security_groups.py::test_modify_mount_target_security_groups
#[tokio::test]
async fn test_moto_modify_mt_security_groups() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("foobarbaz")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let mt = client
        .create_mount_target()
        .file_system_id(&fs_id)
        .subnet_id("subnet-aaa")
        .security_groups("sg-original")
        .send()
        .await
        .unwrap();
    let mt_id = mt.mount_target_id().to_string();

    // Modify security groups
    client
        .modify_mount_target_security_groups()
        .mount_target_id(&mt_id)
        .security_groups("sg-new1")
        .security_groups("sg-new2")
        .send()
        .await
        .unwrap();

    // Verify the change
    let resp = client
        .describe_mount_target_security_groups()
        .mount_target_id(&mt_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.security_groups().len(), 2);
    assert!(resp.security_groups().contains(&"sg-new1".to_string()));
    assert!(resp.security_groups().contains(&"sg-new2".to_string()));
}

// ============================================================================
// Ported from moto: test_lifecycle_config.py
// ============================================================================

// Ported from moto: test_lifecycle_config.py::test_describe_filesystem_config__unknown
#[tokio::test]
async fn test_moto_describe_lifecycle_config_unknown() {
    let client = make_efs_client().await;

    let err = client
        .describe_lifecycle_configuration()
        .file_system_id("unknown")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("FileSystemNotFound"));
}

// Ported from moto: test_lifecycle_config.py::test_describe_filesystem_config__initial
#[tokio::test]
async fn test_moto_describe_lifecycle_config_initial() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("foobar")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let resp = client
        .describe_lifecycle_configuration()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.lifecycle_policies().len(), 0);
}

// Ported from moto: test_lifecycle_config.py::test_put_lifecycle_configuration
#[tokio::test]
async fn test_moto_put_lifecycle_configuration() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("foobar")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let policy = aws_sdk_efs::types::LifecyclePolicy::builder()
        .transition_to_ia(aws_sdk_efs::types::TransitionToIaRules::After30Days)
        .build();

    let resp = client
        .put_lifecycle_configuration()
        .file_system_id(&fs_id)
        .lifecycle_policies(policy)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.lifecycle_policies().len(), 1);
    assert_eq!(
        resp.lifecycle_policies()[0]
            .transition_to_ia()
            .unwrap()
            .as_str(),
        "AFTER_30_DAYS"
    );

    // Describe the lifecycle configuration
    let resp = client
        .describe_lifecycle_configuration()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.lifecycle_policies().len(), 1);
    assert_eq!(
        resp.lifecycle_policies()[0]
            .transition_to_ia()
            .unwrap()
            .as_str(),
        "AFTER_30_DAYS"
    );
}

// ============================================================================
// Existing tests (kept for backward compatibility)
// ============================================================================

#[tokio::test]
async fn test_file_system_policy_lifecycle() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("policy-lifecycle-token")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    // Initially no policy
    let result = client
        .describe_file_system_policy()
        .file_system_id(&fs_id)
        .send()
        .await;
    assert!(result.is_err());

    // Put policy
    let policy = r#"{"Version":"2012-10-17","Statement":[]}"#;
    client
        .put_file_system_policy()
        .file_system_id(&fs_id)
        .policy(policy)
        .send()
        .await
        .unwrap();

    // Describe policy
    let desc = client
        .describe_file_system_policy()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.policy().unwrap(), policy);

    // Delete policy
    client
        .delete_file_system_policy()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();

    // Verify gone
    let result = client
        .describe_file_system_policy()
        .file_system_id(&fs_id)
        .send()
        .await;
    assert!(result.is_err());
}

// --- DescribeBackupPolicy / PutBackupPolicy tests ---

#[tokio::test]
async fn test_put_and_describe_backup_policy() {
    let client = make_efs_client().await;

    let create_resp = client
        .create_file_system()
        .creation_token("backup-policy-test")
        .send()
        .await
        .unwrap();
    let fs_id = create_resp.file_system_id();

    let policy = aws_sdk_efs::types::BackupPolicy::builder()
        .status(aws_sdk_efs::types::Status::Enabled)
        .build()
        .unwrap();

    client
        .put_backup_policy()
        .file_system_id(fs_id)
        .backup_policy(policy)
        .send()
        .await
        .expect("put_backup_policy should succeed");

    let describe_resp = client
        .describe_backup_policy()
        .file_system_id(fs_id)
        .send()
        .await
        .expect("describe_backup_policy should succeed");

    let bp = describe_resp
        .backup_policy()
        .expect("should have backup_policy");
    assert_eq!(bp.status(), &aws_sdk_efs::types::Status::Enabled);
}

#[tokio::test]
async fn test_describe_backup_policy_default_none() {
    let client = make_efs_client().await;

    let create_resp = client
        .create_file_system()
        .creation_token("bp-default-test")
        .send()
        .await
        .unwrap();
    let fs_id = create_resp.file_system_id();

    let describe_resp = client
        .describe_backup_policy()
        .file_system_id(fs_id)
        .send()
        .await
        .expect("describe_backup_policy should succeed for new fs");

    // No policy set yet - backup_policy should be None
    assert!(describe_resp.backup_policy().is_none());
}

#[tokio::test]
async fn test_describe_backup_policy_not_found() {
    let client = make_efs_client().await;

    let result = client
        .describe_backup_policy()
        .file_system_id("fs-nonexistent")
        .send()
        .await;
    assert!(result.is_err(), "describe on nonexistent fs should fail");
}

#[tokio::test]
async fn test_describe_mount_targets_no_filter_returns_error() {
    let client = make_efs_client().await;

    let result = client.describe_mount_targets().send().await;
    assert!(
        result.is_err(),
        "describe_mount_targets with no filter should fail"
    );
}

#[tokio::test]
async fn test_create_access_point_nonexistent_file_system() {
    let client = make_efs_client().await;

    let err = client
        .create_access_point()
        .client_token("ap-no-fs-token")
        .file_system_id("fs-nonexistent999")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("FileSystemNotFound"),
        "Expected FileSystemNotFound, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_access_point_idempotency_same_token_and_fs() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("ap-idem-fs-token")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let ap1 = client
        .create_access_point()
        .client_token("idem-client-token")
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();

    let ap2 = client
        .create_access_point()
        .client_token("idem-client-token")
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();

    assert_eq!(
        ap1.access_point_id().unwrap(),
        ap2.access_point_id().unwrap(),
        "idempotent create_access_point should return the same access point ID"
    );

    // Only one access point should exist
    let desc = client
        .describe_access_points()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.access_points().len(), 1);
}

#[tokio::test]
async fn test_tag_resource_updates_existing_tag_value() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("tag-update-token")
        .tags(
            aws_sdk_efs::types::Tag::builder()
                .key("Env")
                .value("staging")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    // Update the existing "Env" tag with a new value
    client
        .tag_resource()
        .resource_id(&fs_id)
        .tags(
            aws_sdk_efs::types::Tag::builder()
                .key("Env")
                .value("production")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource update should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_id(&fs_id)
        .send()
        .await
        .unwrap();

    // Still one tag, with the updated value
    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), "Env");
    assert_eq!(resp.tags()[0].value(), "production");
}

#[tokio::test]
async fn test_tag_name_updates_file_system_name() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("name-tag-update-token")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    // Initially no Name tag, name should be empty
    let desc = client
        .describe_file_systems()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.file_systems()[0].name(), Some(""));

    // Tag with Name
    client
        .tag_resource()
        .resource_id(&fs_id)
        .tags(
            aws_sdk_efs::types::Tag::builder()
                .key("Name")
                .value("MyFileSystem")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    // Name should now reflect the tag value
    let desc2 = client
        .describe_file_systems()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc2.file_systems()[0].name(), Some("MyFileSystem"));
}

#[tokio::test]
async fn test_put_lifecycle_configuration_nonexistent_fs() {
    let client = make_efs_client().await;

    let policy = aws_sdk_efs::types::LifecyclePolicy::builder()
        .transition_to_ia(aws_sdk_efs::types::TransitionToIaRules::After7Days)
        .build();

    let result = client
        .put_lifecycle_configuration()
        .file_system_id("fs-nonexistent")
        .lifecycle_policies(policy)
        .send()
        .await;

    assert!(
        result.is_err(),
        "put_lifecycle_configuration on nonexistent fs should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("FileSystemNotFound"),
        "Expected FileSystemNotFound, got: {err_str}"
    );
}

#[tokio::test]
async fn test_put_file_system_policy_nonexistent_fs() {
    let client = make_efs_client().await;

    let policy = r#"{"Version":"2012-10-17","Statement":[]}"#;
    let result = client
        .put_file_system_policy()
        .file_system_id("fs-doesnotexist")
        .policy(policy)
        .send()
        .await;

    assert!(
        result.is_err(),
        "put_file_system_policy on nonexistent fs should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("FileSystemNotFound"),
        "Expected FileSystemNotFound, got: {err_str}"
    );
}

#[tokio::test]
async fn test_delete_file_system_policy_nonexistent_fs() {
    let client = make_efs_client().await;

    let result = client
        .delete_file_system_policy()
        .file_system_id("fs-doesnotexist")
        .send()
        .await;

    assert!(
        result.is_err(),
        "delete_file_system_policy on nonexistent fs should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("FileSystemNotFound"),
        "Expected FileSystemNotFound, got: {err_str}"
    );
}

#[tokio::test]
async fn test_put_backup_policy_disabled() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("bp-disabled-token")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    // First enable
    let enabled_policy = aws_sdk_efs::types::BackupPolicy::builder()
        .status(aws_sdk_efs::types::Status::Enabled)
        .build()
        .unwrap();
    client
        .put_backup_policy()
        .file_system_id(&fs_id)
        .backup_policy(enabled_policy)
        .send()
        .await
        .unwrap();

    // Now disable
    let disabled_policy = aws_sdk_efs::types::BackupPolicy::builder()
        .status(aws_sdk_efs::types::Status::Disabled)
        .build()
        .unwrap();
    let resp = client
        .put_backup_policy()
        .file_system_id(&fs_id)
        .backup_policy(disabled_policy)
        .send()
        .await
        .expect("put_backup_policy DISABLED should succeed");

    let bp = resp.backup_policy().expect("should have backup policy");
    assert_eq!(bp.status(), &aws_sdk_efs::types::Status::Disabled);

    // Describe should reflect the DISABLED state
    let desc = client
        .describe_backup_policy()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    let bp = desc.backup_policy().expect("should have backup policy");
    assert_eq!(bp.status(), &aws_sdk_efs::types::Status::Disabled);
}

#[tokio::test]
async fn test_create_mount_target_with_ip_address() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("mt-ip-token")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let mt = client
        .create_mount_target()
        .file_system_id(&fs_id)
        .subnet_id("subnet-ip-test")
        .ip_address("10.1.2.3")
        .send()
        .await
        .expect("create_mount_target with explicit IP should succeed");

    assert_eq!(mt.ip_address(), Some("10.1.2.3"));
    assert_eq!(mt.file_system_id(), fs_id);
}

#[tokio::test]
async fn test_delete_file_system_allows_creation_token_reuse() {
    let client = make_efs_client().await;

    let fs1 = client
        .create_file_system()
        .creation_token("reusable-token")
        .send()
        .await
        .unwrap();
    let fs_id1 = fs1.file_system_id().to_string();

    // Delete the file system
    client
        .delete_file_system()
        .file_system_id(&fs_id1)
        .send()
        .await
        .unwrap();

    // Creating with the same token should succeed and produce a new FS ID
    let fs2 = client
        .create_file_system()
        .creation_token("reusable-token")
        .send()
        .await
        .expect("creation token should be reusable after deletion");

    assert_ne!(
        fs2.file_system_id(),
        fs_id1,
        "new file system should have a different ID"
    );
    assert!(fs2.file_system_id().starts_with("fs-"));
}

#[tokio::test]
async fn test_describe_access_points_by_nonexistent_file_system() {
    let client = make_efs_client().await;

    let result = client
        .describe_access_points()
        .file_system_id("fs-nonexistent")
        .send()
        .await;

    assert!(
        result.is_err(),
        "describe_access_points with nonexistent fs_id should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("FileSystemNotFound"),
        "Expected FileSystemNotFound, got: {err_str}"
    );
}

#[tokio::test]
async fn test_put_lifecycle_configuration_replaces_existing() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("lc-replace-token")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    // Put an initial lifecycle policy
    let policy1 = aws_sdk_efs::types::LifecyclePolicy::builder()
        .transition_to_ia(aws_sdk_efs::types::TransitionToIaRules::After30Days)
        .build();
    client
        .put_lifecycle_configuration()
        .file_system_id(&fs_id)
        .lifecycle_policies(policy1)
        .send()
        .await
        .unwrap();

    // Replace with a different policy
    let policy2 = aws_sdk_efs::types::LifecyclePolicy::builder()
        .transition_to_ia(aws_sdk_efs::types::TransitionToIaRules::After7Days)
        .build();
    let resp = client
        .put_lifecycle_configuration()
        .file_system_id(&fs_id)
        .lifecycle_policies(policy2)
        .send()
        .await
        .expect("replacing lifecycle configuration should succeed");

    assert_eq!(resp.lifecycle_policies().len(), 1);
    assert_eq!(
        resp.lifecycle_policies()[0]
            .transition_to_ia()
            .unwrap()
            .as_str(),
        "AFTER_7_DAYS"
    );

    // Describe should also show the new policy
    let desc = client
        .describe_lifecycle_configuration()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.lifecycle_policies().len(), 1);
    assert_eq!(
        desc.lifecycle_policies()[0]
            .transition_to_ia()
            .unwrap()
            .as_str(),
        "AFTER_7_DAYS"
    );
}

#[tokio::test]
async fn test_delete_mount_target_decrements_mount_target_count() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("mt-count-token")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let mt1 = client
        .create_mount_target()
        .file_system_id(&fs_id)
        .subnet_id("subnet-cnt-1")
        .send()
        .await
        .unwrap();

    let mt2 = client
        .create_mount_target()
        .file_system_id(&fs_id)
        .subnet_id("subnet-cnt-2")
        .send()
        .await
        .unwrap();

    // Should have 2 mount targets
    let desc1 = client
        .describe_file_systems()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc1.file_systems()[0].number_of_mount_targets(), 2);

    // Delete one
    client
        .delete_mount_target()
        .mount_target_id(mt1.mount_target_id())
        .send()
        .await
        .unwrap();

    let desc2 = client
        .describe_file_systems()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc2.file_systems()[0].number_of_mount_targets(), 1);

    // Delete second
    client
        .delete_mount_target()
        .mount_target_id(mt2.mount_target_id())
        .send()
        .await
        .unwrap();

    let desc3 = client
        .describe_file_systems()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc3.file_systems()[0].number_of_mount_targets(), 0);

    // Now the FS can be deleted
    client
        .delete_file_system()
        .file_system_id(&fs_id)
        .send()
        .await
        .expect("delete_file_system after removing all mount targets should succeed");
}

#[tokio::test]
async fn test_create_and_describe_replication_configuration() {
    let client = make_efs_client().await;

    // Create source file system
    let fs = client
        .create_file_system()
        .creation_token("repl-src-token")
        .send()
        .await
        .unwrap();
    let src_fs_id = fs.file_system_id().to_string();

    // Create a destination file system
    let dest_fs = client
        .create_file_system()
        .creation_token("repl-dest-token")
        .send()
        .await
        .unwrap();
    let dest_fs_id = dest_fs.file_system_id().to_string();

    // Create replication configuration
    let repl_resp = client
        .create_replication_configuration()
        .source_file_system_id(&src_fs_id)
        .destinations(
            aws_sdk_efs::types::DestinationToCreate::builder()
                .file_system_id(&dest_fs_id)
                .region("us-east-1")
                .build(),
        )
        .send()
        .await
        .expect("create_replication_configuration should succeed");

    assert_eq!(repl_resp.source_file_system_id(), src_fs_id);
    assert!(!repl_resp.destinations().is_empty());
    assert_eq!(
        repl_resp.destinations()[0].file_system_id(),
        dest_fs_id.as_str()
    );

    // Describe replication configurations
    let desc_resp = client
        .describe_replication_configurations()
        .file_system_id(&src_fs_id)
        .send()
        .await
        .expect("describe_replication_configurations should succeed");

    let replications = desc_resp.replications();
    assert_eq!(replications.len(), 1);
    assert_eq!(replications[0].source_file_system_id(), src_fs_id);
}

#[tokio::test]
async fn test_delete_replication_configuration() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("repl-del-src")
        .send()
        .await
        .unwrap();
    let src_fs_id = fs.file_system_id().to_string();

    let dest_fs = client
        .create_file_system()
        .creation_token("repl-del-dest")
        .send()
        .await
        .unwrap();
    let dest_fs_id = dest_fs.file_system_id().to_string();

    client
        .create_replication_configuration()
        .source_file_system_id(&src_fs_id)
        .destinations(
            aws_sdk_efs::types::DestinationToCreate::builder()
                .file_system_id(&dest_fs_id)
                .region("us-east-1")
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Delete the replication configuration
    client
        .delete_replication_configuration()
        .source_file_system_id(&src_fs_id)
        .send()
        .await
        .expect("delete_replication_configuration should succeed");

    // Verify it's gone
    let desc_resp = client
        .describe_replication_configurations()
        .file_system_id(&src_fs_id)
        .send()
        .await
        .expect("describe_replication_configurations should succeed");

    assert!(
        desc_resp.replications().is_empty(),
        "replication config should be deleted"
    );
}

#[tokio::test]
async fn test_legacy_create_and_describe_tags() {
    use aws_sdk_efs::types::Tag;

    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("legacy-tags-token")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    // Create tags using legacy API
    #[allow(deprecated)]
    client
        .create_tags()
        .file_system_id(&fs_id)
        .tags(
            Tag::builder()
                .key("Environment")
                .value("Production")
                .build()
                .unwrap(),
        )
        .tags(Tag::builder().key("Team").value("Backend").build().unwrap())
        .send()
        .await
        .expect("create_tags should succeed");

    // Describe tags using legacy API
    #[allow(deprecated)]
    let desc_resp = client
        .describe_tags()
        .file_system_id(&fs_id)
        .send()
        .await
        .expect("describe_tags should succeed");

    let tags = desc_resp.tags();
    // Should have the created tags plus any default tags
    let env_tag = tags.iter().find(|t| t.key() == "Environment");
    assert!(env_tag.is_some(), "Environment tag should exist");
    assert_eq!(env_tag.unwrap().value(), "Production");

    let team_tag = tags.iter().find(|t| t.key() == "Team");
    assert!(team_tag.is_some(), "Team tag should exist");
    assert_eq!(team_tag.unwrap().value(), "Backend");
}

#[tokio::test]
async fn test_legacy_delete_tags() {
    use aws_sdk_efs::types::Tag;

    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("legacy-del-tags-token")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    #[allow(deprecated)]
    client
        .create_tags()
        .file_system_id(&fs_id)
        .tags(Tag::builder().key("ToDelete").value("yes").build().unwrap())
        .tags(Tag::builder().key("ToKeep").value("yes").build().unwrap())
        .send()
        .await
        .unwrap();

    // Delete one tag
    #[allow(deprecated)]
    client
        .delete_tags()
        .file_system_id(&fs_id)
        .tag_keys("ToDelete")
        .send()
        .await
        .expect("delete_tags should succeed");

    // Verify the tag is deleted
    #[allow(deprecated)]
    let desc_resp = client
        .describe_tags()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();

    let tags = desc_resp.tags();
    assert!(
        !tags.iter().any(|t| t.key() == "ToDelete"),
        "ToDelete tag should be gone"
    );
    assert!(
        tags.iter().any(|t| t.key() == "ToKeep"),
        "ToKeep tag should remain"
    );
}

#[tokio::test]
async fn test_describe_account_preferences() {
    let client = make_efs_client().await;

    let resp = client
        .describe_account_preferences()
        .send()
        .await
        .expect("describe_account_preferences should succeed");

    let pref = resp
        .resource_id_preference()
        .expect("should have preference");
    // Default should be LONG_ID
    assert!(
        pref.resource_id_type().is_some(),
        "resource_id_type should be present"
    );
}

#[tokio::test]
async fn test_put_account_preferences() {
    let client = make_efs_client().await;

    let resp = client
        .put_account_preferences()
        .resource_id_type(aws_sdk_efs::types::ResourceIdType::LongId)
        .send()
        .await
        .expect("put_account_preferences should succeed");

    let pref = resp
        .resource_id_preference()
        .expect("should have preference");
    assert_eq!(
        pref.resource_id_type(),
        Some(&aws_sdk_efs::types::ResourceIdType::LongId)
    );
}

#[tokio::test]
async fn test_update_file_system() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("update-fs-token")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    // Update throughput mode to provisioned
    let update_resp = client
        .update_file_system()
        .file_system_id(&fs_id)
        .throughput_mode(aws_sdk_efs::types::ThroughputMode::Provisioned)
        .provisioned_throughput_in_mibps(128.0)
        .send()
        .await
        .expect("update_file_system should succeed");

    assert_eq!(
        update_resp.throughput_mode(),
        Some(&aws_sdk_efs::types::ThroughputMode::Provisioned)
    );
    assert_eq!(update_resp.provisioned_throughput_in_mibps(), Some(128.0));
}

#[tokio::test]
async fn test_update_file_system_nonexistent() {
    let client = make_efs_client().await;

    let result = client
        .update_file_system()
        .file_system_id("fs-nonexistent")
        .throughput_mode(aws_sdk_efs::types::ThroughputMode::Bursting)
        .send()
        .await;

    assert!(result.is_err(), "should fail for nonexistent file system");
}

#[tokio::test]
async fn test_update_file_system_protection() {
    let client = make_efs_client().await;

    let fs = client
        .create_file_system()
        .creation_token("update-protection-token")
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system_id().to_string();

    let resp = client
        .update_file_system_protection()
        .file_system_id(&fs_id)
        .replication_overwrite_protection(
            aws_sdk_efs::types::ReplicationOverwriteProtection::Disabled,
        )
        .send()
        .await
        .expect("update_file_system_protection should succeed");

    assert_eq!(
        resp.replication_overwrite_protection(),
        Some(&aws_sdk_efs::types::ReplicationOverwriteProtection::Disabled)
    );
}

#[tokio::test]
async fn test_update_file_system_protection_nonexistent() {
    let client = make_efs_client().await;

    let result = client
        .update_file_system_protection()
        .file_system_id("fs-nonexistent")
        .replication_overwrite_protection(
            aws_sdk_efs::types::ReplicationOverwriteProtection::Enabled,
        )
        .send()
        .await;

    assert!(result.is_err(), "should fail for nonexistent file system");
}
