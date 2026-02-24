use aws_sdk_fsx::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_fsx::FsxService;

const FAKE_SUBNET_ID: &str = "subnet-012345678";
const FAKE_SECURITY_GROUP_IDS: [&str; 2] = ["sg-0123456789abcdef0", "sg-0123456789abcdef1"];

async fn make_fsx_client() -> aws_sdk_fsx::Client {
    let mock = MockAws::builder().with_service(FsxService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_fsx::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_fsx::Client::new(&config)
}

fn make_tag(key: &str, value: &str) -> aws_sdk_fsx::types::Tag {
    aws_sdk_fsx::types::Tag::builder()
        .key(key)
        .value(value)
        .build()
}

#[tokio::test]
async fn test_create_filesystem() {
    let client = make_fsx_client().await;

    let resp = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Lustre)
        .storage_capacity(1200)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .set_security_group_ids(Some(
            FAKE_SECURITY_GROUP_IDS
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ))
        .send()
        .await
        .expect("create_file_system should succeed");

    let file_system = resp.file_system().expect("should have file_system");
    let fs_id = file_system.file_system_id().unwrap();
    assert!(fs_id.starts_with("fs-"));
    assert!(fs_id.len() >= 10);
    assert_eq!(
        file_system.file_system_type(),
        Some(&aws_sdk_fsx::types::FileSystemType::Lustre)
    );
}

#[tokio::test]
async fn test_describe_filesystems() {
    let client = make_fsx_client().await;

    // Create LUSTRE
    client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Lustre)
        .storage_capacity(1200)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .set_security_group_ids(Some(
            FAKE_SECURITY_GROUP_IDS
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ))
        .send()
        .await
        .unwrap();

    // Create WINDOWS
    client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Windows)
        .storage_capacity(1200)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .set_security_group_ids(Some(
            FAKE_SECURITY_GROUP_IDS
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ))
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_file_systems()
        .send()
        .await
        .expect("describe_file_systems should succeed");

    let file_systems = resp.file_systems();
    assert_eq!(file_systems.len(), 2);
}

#[tokio::test]
async fn test_describe_filesystems_does_not_exist() {
    let client = make_fsx_client().await;

    let resp = client
        .describe_file_systems()
        .file_system_ids("fs-1234567890")
        .send()
        .await
        .expect("describe_file_systems should succeed");

    assert_eq!(resp.file_systems().len(), 0);
}

#[tokio::test]
async fn test_delete_file_system() {
    let client = make_fsx_client().await;

    let fs = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Lustre)
        .storage_capacity(1200)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .set_security_group_ids(Some(
            FAKE_SECURITY_GROUP_IDS
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ))
        .send()
        .await
        .unwrap();

    let fs_id = fs
        .file_system()
        .unwrap()
        .file_system_id()
        .unwrap()
        .to_string();

    let resp = client
        .delete_file_system()
        .file_system_id(&fs_id)
        .send()
        .await
        .expect("delete_file_system should succeed");

    assert_eq!(resp.file_system_id(), Some(fs_id.as_str()));
    assert_eq!(
        resp.lifecycle(),
        Some(&aws_sdk_fsx::types::FileSystemLifecycle::Deleting)
    );
    assert!(resp.lustre_response().is_some());
}

#[tokio::test]
async fn test_create_backup() {
    let client = make_fsx_client().await;

    let fs = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Lustre)
        .storage_capacity(1200)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .set_security_group_ids(Some(
            FAKE_SECURITY_GROUP_IDS
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ))
        .send()
        .await
        .unwrap();
    let fs_id = fs
        .file_system()
        .unwrap()
        .file_system_id()
        .unwrap()
        .to_string();

    let tags = vec![make_tag("Moto", "Hello"), make_tag("Environment", "Dev")];

    let resp = client
        .create_backup()
        .file_system_id(&fs_id)
        .set_tags(Some(tags))
        .send()
        .await
        .expect("create_backup should succeed");

    let backup = resp.backup().expect("should have backup");
    assert!(backup.backup_id().unwrap().starts_with("backup-"));
    assert!(backup.resource_arn().unwrap().starts_with("arn:aws:fsx:"));
    let backup_tags = backup.tags();
    assert_eq!(backup_tags.len(), 2);
    assert_eq!(backup_tags[0].key(), Some("Moto"));
    assert_eq!(backup_tags[0].value(), Some("Hello"));
    assert_eq!(backup_tags[1].key(), Some("Environment"));
    assert_eq!(backup_tags[1].value(), Some("Dev"));
}

#[tokio::test]
async fn test_nonexistent_file_system() {
    let client = make_fsx_client().await;

    let result = client
        .create_backup()
        .file_system_id("NONEXISTENTFILESYSTEMID")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_backup() {
    let client = make_fsx_client().await;

    let fs = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Lustre)
        .storage_capacity(1200)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .set_security_group_ids(Some(
            FAKE_SECURITY_GROUP_IDS
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ))
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system().unwrap().file_system_id().unwrap();

    let backup_resp = client
        .create_backup()
        .file_system_id(fs_id)
        .send()
        .await
        .unwrap();

    let backup = backup_resp.backup().unwrap();
    let backup_id = backup.backup_id().unwrap().to_string();
    assert!(backup_id.starts_with("backup-"));
    assert!(backup.resource_arn().unwrap().starts_with("arn:aws:fsx:"));

    let resp = client
        .delete_backup()
        .backup_id(&backup_id)
        .send()
        .await
        .expect("delete_backup should succeed");

    assert_eq!(resp.backup_id(), Some(backup_id.as_str()));
    assert_eq!(
        resp.lifecycle(),
        Some(&aws_sdk_fsx::types::BackupLifecycle::Deleted)
    );
}

#[tokio::test]
async fn test_backup_not_found() {
    let client = make_fsx_client().await;

    let result = client
        .delete_backup()
        .backup_id("NONEXISTENTBACKUPID")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_backups() {
    let client = make_fsx_client().await;

    let fs = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Lustre)
        .storage_capacity(1200)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .set_security_group_ids(Some(
            FAKE_SECURITY_GROUP_IDS
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ))
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system().unwrap().file_system_id().unwrap();

    client
        .create_backup()
        .file_system_id(fs_id)
        .send()
        .await
        .unwrap();
    client
        .create_backup()
        .file_system_id(fs_id)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_backups()
        .send()
        .await
        .expect("describe_backups should succeed");

    let backups = resp.backups();
    assert_eq!(backups.len(), 2);
    assert!(!backups[0].backup_id().unwrap().is_empty());
    assert!(backups[0].resource_arn().unwrap().contains(":backup/"));
}

#[tokio::test]
async fn test_describe_backups_with_id() {
    let client = make_fsx_client().await;

    let fs = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Lustre)
        .storage_capacity(1200)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .set_security_group_ids(Some(
            FAKE_SECURITY_GROUP_IDS
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ))
        .send()
        .await
        .unwrap();
    let fs_id = fs.file_system().unwrap().file_system_id().unwrap();

    let backup_resp = client
        .create_backup()
        .file_system_id(fs_id)
        .send()
        .await
        .unwrap();
    let backup_id = backup_resp
        .backup()
        .unwrap()
        .backup_id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_backups()
        .backup_ids(&backup_id)
        .send()
        .await
        .expect("describe_backups with id should succeed");

    let backups = resp.backups();
    assert_eq!(backups.len(), 1);
    assert!(!backups[0].backup_id().unwrap().is_empty());
    assert!(backups[0].resource_arn().unwrap().contains(":backup/"));
}

#[tokio::test]
async fn test_describe_backups_does_not_exist() {
    let client = make_fsx_client().await;

    let resp = client
        .describe_backups()
        .backup_ids("NONEXISTENTBACKUPID")
        .send()
        .await
        .expect("describe_backups should succeed");

    assert_eq!(resp.backups().len(), 0);
}

#[tokio::test]
async fn test_describe_backups_filters() {
    let client = make_fsx_client().await;

    let fs = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Lustre)
        .storage_capacity(1200)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .set_security_group_ids(Some(
            FAKE_SECURITY_GROUP_IDS
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ))
        .send()
        .await
        .unwrap();
    let fs_id = fs
        .file_system()
        .unwrap()
        .file_system_id()
        .unwrap()
        .to_string();

    let backup_resp = client
        .create_backup()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    let backup_id = backup_resp
        .backup()
        .unwrap()
        .backup_id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_backups()
        .filters(
            aws_sdk_fsx::types::Filter::builder()
                .name(aws_sdk_fsx::types::FilterName::FileSystemId)
                .values(&fs_id)
                .build(),
        )
        .send()
        .await
        .expect("describe_backups with filter should succeed");

    let backups = resp.backups();
    assert_eq!(backups.len(), 1);
    assert_eq!(backups[0].backup_id(), Some(backup_id.as_str()));
}

#[tokio::test]
async fn test_tag_resource() {
    let client = make_fsx_client().await;

    let fs = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Lustre)
        .storage_capacity(1200)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .set_security_group_ids(Some(
            FAKE_SECURITY_GROUP_IDS
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ))
        .send()
        .await
        .unwrap();
    let fs_id = fs
        .file_system()
        .unwrap()
        .file_system_id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_file_systems()
        .file_system_ids(&fs_id)
        .send()
        .await
        .unwrap();
    let resource_arn = resp.file_systems()[0].resource_arn().unwrap().to_string();

    let tags = vec![make_tag("Moto", "Hello"), make_tag("Environment", "Dev")];

    client
        .tag_resource()
        .resource_arn(&resource_arn)
        .set_tags(Some(tags))
        .send()
        .await
        .expect("tag_resource should succeed");

    let resp = client
        .describe_file_systems()
        .file_system_ids(&fs_id)
        .send()
        .await
        .unwrap();
    let fs_tags = resp.file_systems()[0].tags();
    assert_eq!(fs_tags.len(), 2);
    assert_eq!(fs_tags[0].key(), Some("Moto"));
    assert_eq!(fs_tags[0].value(), Some("Hello"));
    assert_eq!(fs_tags[1].key(), Some("Environment"));
    assert_eq!(fs_tags[1].value(), Some("Dev"));
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_fsx_client().await;

    let initial_tags = vec![make_tag("Moto", "Hello"), make_tag("Environment", "Dev")];

    let fs = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Lustre)
        .storage_capacity(1200)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .set_security_group_ids(Some(
            FAKE_SECURITY_GROUP_IDS
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ))
        .set_tags(Some(initial_tags))
        .send()
        .await
        .unwrap();
    let fs_id = fs
        .file_system()
        .unwrap()
        .file_system_id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_file_systems()
        .file_system_ids(&fs_id)
        .send()
        .await
        .unwrap();
    let resource_arn = resp.file_systems()[0].resource_arn().unwrap().to_string();

    client
        .untag_resource()
        .resource_arn(&resource_arn)
        .tag_keys("Moto")
        .send()
        .await
        .expect("untag_resource should succeed");

    let resp = client
        .describe_file_systems()
        .file_system_ids(&fs_id)
        .send()
        .await
        .unwrap();
    let fs_tags = resp.file_systems()[0].tags();
    assert_eq!(fs_tags.len(), 1);
    assert_eq!(fs_tags[0].key(), Some("Environment"));
    assert_eq!(fs_tags[0].value(), Some("Dev"));
}

#[tokio::test]
async fn test_list_tags_for_resource() {
    let client = make_fsx_client().await;

    let fs = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Lustre)
        .storage_capacity(1200)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .set_security_group_ids(Some(
            FAKE_SECURITY_GROUP_IDS
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ))
        .send()
        .await
        .unwrap();

    let resource_arn = fs
        .file_system()
        .unwrap()
        .resource_arn()
        .unwrap()
        .to_string();

    let tags = vec![make_tag("Moto", "Hello"), make_tag("Environment", "Dev")];

    client
        .tag_resource()
        .resource_arn(&resource_arn)
        .set_tags(Some(tags))
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&resource_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = resp.tags();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0].key(), Some("Moto"));
    assert_eq!(tags[0].value(), Some("Hello"));
    assert_eq!(tags[1].key(), Some("Environment"));
    assert_eq!(tags[1].value(), Some("Dev"));
}

#[tokio::test]
async fn test_delete_nonexistent_file_system() {
    let client = make_fsx_client().await;

    let result = client
        .delete_file_system()
        .file_system_id("fs-0000000000")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_filesystem_ontap() {
    let client = make_fsx_client().await;

    let resp = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Ontap)
        .storage_capacity(1024)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .set_security_group_ids(Some(
            FAKE_SECURITY_GROUP_IDS
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ))
        .send()
        .await
        .expect("create_file_system ONTAP should succeed");

    let file_system = resp.file_system().expect("should have file_system");
    assert_eq!(
        file_system.file_system_type(),
        Some(&aws_sdk_fsx::types::FileSystemType::Ontap)
    );
    let fs_id = file_system.file_system_id().unwrap();
    assert!(fs_id.starts_with("fs-"));
}

#[tokio::test]
async fn test_create_filesystem_open_zfs() {
    let client = make_fsx_client().await;

    let resp = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Openzfs)
        .storage_capacity(64)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .send()
        .await
        .expect("create_file_system OpenZFS should succeed");

    let file_system = resp.file_system().expect("should have file_system");
    assert_eq!(
        file_system.file_system_type(),
        Some(&aws_sdk_fsx::types::FileSystemType::Openzfs)
    );
}

#[tokio::test]
async fn test_create_filesystem_storage_capacity_preserved() {
    let client = make_fsx_client().await;

    let resp = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Lustre)
        .storage_capacity(2400)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .send()
        .await
        .expect("create_file_system should succeed");

    let file_system = resp.file_system().expect("should have file_system");
    assert_eq!(file_system.storage_capacity(), Some(2400));
    assert_eq!(
        file_system.storage_type(),
        Some(&aws_sdk_fsx::types::StorageType::Ssd)
    );
}

#[tokio::test]
async fn test_create_filesystem_with_kms_key_id() {
    let client = make_fsx_client().await;

    let kms_key_id = "arn:aws:kms:us-east-1:123456789012:key/mrk-1234abcd12ab34cd56ef1234567890ab";

    let resp = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Lustre)
        .storage_capacity(1200)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .kms_key_id(kms_key_id)
        .send()
        .await
        .expect("create_file_system with KMS key should succeed");

    let file_system = resp.file_system().expect("should have file_system");
    assert_eq!(file_system.kms_key_id(), Some(kms_key_id));
}

#[tokio::test]
async fn test_create_filesystem_with_tags() {
    let client = make_fsx_client().await;

    let tags = vec![make_tag("Project", "MyProject"), make_tag("Owner", "TeamA")];

    let resp = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Windows)
        .storage_capacity(300)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .set_tags(Some(tags))
        .send()
        .await
        .expect("create_file_system with tags should succeed");

    let file_system = resp.file_system().expect("should have file_system");
    let fs_tags = file_system.tags();
    assert_eq!(fs_tags.len(), 2);
    assert_eq!(fs_tags[0].key(), Some("Project"));
    assert_eq!(fs_tags[0].value(), Some("MyProject"));
    assert_eq!(fs_tags[1].key(), Some("Owner"));
    assert_eq!(fs_tags[1].value(), Some("TeamA"));
}

#[tokio::test]
async fn test_describe_filesystems_by_multiple_ids() {
    let client = make_fsx_client().await;

    let fs1 = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Lustre)
        .storage_capacity(1200)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .send()
        .await
        .unwrap();
    let fs1_id = fs1
        .file_system()
        .unwrap()
        .file_system_id()
        .unwrap()
        .to_string();

    let fs2 = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Windows)
        .storage_capacity(300)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .send()
        .await
        .unwrap();
    let fs2_id = fs2
        .file_system()
        .unwrap()
        .file_system_id()
        .unwrap()
        .to_string();

    // Create a third FS that we won't query
    client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Ontap)
        .storage_capacity(1024)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_file_systems()
        .file_system_ids(&fs1_id)
        .file_system_ids(&fs2_id)
        .send()
        .await
        .expect("describe_file_systems with multiple IDs should succeed");

    let file_systems = resp.file_systems();
    assert_eq!(file_systems.len(), 2);
    let ids: Vec<&str> = file_systems
        .iter()
        .map(|fs| fs.file_system_id().unwrap())
        .collect();
    assert!(ids.contains(&fs1_id.as_str()));
    assert!(ids.contains(&fs2_id.as_str()));
}

#[tokio::test]
async fn test_delete_filesystem_then_describe_returns_empty() {
    let client = make_fsx_client().await;

    let fs = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Lustre)
        .storage_capacity(1200)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .send()
        .await
        .unwrap();
    let fs_id = fs
        .file_system()
        .unwrap()
        .file_system_id()
        .unwrap()
        .to_string();

    client
        .delete_file_system()
        .file_system_id(&fs_id)
        .send()
        .await
        .expect("delete_file_system should succeed");

    let resp = client
        .describe_file_systems()
        .file_system_ids(&fs_id)
        .send()
        .await
        .expect("describe after delete should succeed");

    assert_eq!(resp.file_systems().len(), 0);
}

#[tokio::test]
async fn test_delete_file_system_windows_has_windows_response() {
    let client = make_fsx_client().await;

    let fs = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Windows)
        .storage_capacity(300)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .send()
        .await
        .unwrap();
    let fs_id = fs
        .file_system()
        .unwrap()
        .file_system_id()
        .unwrap()
        .to_string();

    let resp = client
        .delete_file_system()
        .file_system_id(&fs_id)
        .send()
        .await
        .expect("delete_file_system (Windows) should succeed");

    assert_eq!(
        resp.lifecycle(),
        Some(&aws_sdk_fsx::types::FileSystemLifecycle::Deleting)
    );
    assert!(resp.windows_response().is_some());
    assert!(resp.lustre_response().is_none());
}

#[tokio::test]
async fn test_tag_resource_overwrites_existing_tag() {
    let client = make_fsx_client().await;

    let fs = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Lustre)
        .storage_capacity(1200)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .set_tags(Some(vec![make_tag("Env", "staging")]))
        .send()
        .await
        .unwrap();
    let resource_arn = fs
        .file_system()
        .unwrap()
        .resource_arn()
        .unwrap()
        .to_string();
    let fs_id = fs
        .file_system()
        .unwrap()
        .file_system_id()
        .unwrap()
        .to_string();

    // Overwrite the "Env" tag with a new value
    client
        .tag_resource()
        .resource_arn(&resource_arn)
        .tags(make_tag("Env", "production"))
        .send()
        .await
        .expect("tag_resource overwrite should succeed");

    let resp = client
        .describe_file_systems()
        .file_system_ids(&fs_id)
        .send()
        .await
        .unwrap();
    let fs_tags = resp.file_systems()[0].tags();
    // Should still have exactly one "Env" tag with updated value
    assert_eq!(fs_tags.len(), 1);
    assert_eq!(fs_tags[0].key(), Some("Env"));
    assert_eq!(fs_tags[0].value(), Some("production"));
}

#[tokio::test]
async fn test_untag_nonexistent_key_is_noop() {
    let client = make_fsx_client().await;

    let fs = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Lustre)
        .storage_capacity(1200)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .set_tags(Some(vec![make_tag("Keep", "me")]))
        .send()
        .await
        .unwrap();
    let resource_arn = fs
        .file_system()
        .unwrap()
        .resource_arn()
        .unwrap()
        .to_string();
    let fs_id = fs
        .file_system()
        .unwrap()
        .file_system_id()
        .unwrap()
        .to_string();

    // Attempt to remove a tag that does not exist
    client
        .untag_resource()
        .resource_arn(&resource_arn)
        .tag_keys("DoesNotExist")
        .send()
        .await
        .expect("untag_resource with non-existent key should be a no-op");

    let resp = client
        .describe_file_systems()
        .file_system_ids(&fs_id)
        .send()
        .await
        .unwrap();
    let fs_tags = resp.file_systems()[0].tags();
    assert_eq!(fs_tags.len(), 1);
    assert_eq!(fs_tags[0].key(), Some("Keep"));
    assert_eq!(fs_tags[0].value(), Some("me"));
}

#[tokio::test]
async fn test_list_tags_for_resource_no_tags() {
    let client = make_fsx_client().await;

    let fs = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Lustre)
        .storage_capacity(1200)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .send()
        .await
        .unwrap();
    let resource_arn = fs
        .file_system()
        .unwrap()
        .resource_arn()
        .unwrap()
        .to_string();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&resource_arn)
        .send()
        .await
        .expect("list_tags_for_resource with no tags should succeed");

    assert_eq!(resp.tags().len(), 0);
}

#[tokio::test]
async fn test_backup_lifecycle_is_creating() {
    let client = make_fsx_client().await;

    let fs = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Lustre)
        .storage_capacity(1200)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .send()
        .await
        .unwrap();
    let fs_id = fs
        .file_system()
        .unwrap()
        .file_system_id()
        .unwrap()
        .to_string();

    let resp = client
        .create_backup()
        .file_system_id(&fs_id)
        .send()
        .await
        .expect("create_backup should succeed");

    let backup = resp.backup().expect("should have backup");
    assert_eq!(
        backup.lifecycle(),
        Some(&aws_sdk_fsx::types::BackupLifecycle::Creating)
    );
}

#[tokio::test]
async fn test_delete_backup_then_describe_returns_empty() {
    let client = make_fsx_client().await;

    let fs = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Lustre)
        .storage_capacity(1200)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .send()
        .await
        .unwrap();
    let fs_id = fs
        .file_system()
        .unwrap()
        .file_system_id()
        .unwrap()
        .to_string();

    let backup_resp = client
        .create_backup()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    let backup_id = backup_resp
        .backup()
        .unwrap()
        .backup_id()
        .unwrap()
        .to_string();

    client
        .delete_backup()
        .backup_id(&backup_id)
        .send()
        .await
        .expect("delete_backup should succeed");

    let resp = client
        .describe_backups()
        .backup_ids(&backup_id)
        .send()
        .await
        .expect("describe_backups after delete should succeed");

    assert_eq!(resp.backups().len(), 0);
}

#[tokio::test]
async fn test_delete_one_of_multiple_backups() {
    let client = make_fsx_client().await;

    let fs = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Lustre)
        .storage_capacity(1200)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .send()
        .await
        .unwrap();
    let fs_id = fs
        .file_system()
        .unwrap()
        .file_system_id()
        .unwrap()
        .to_string();

    let b1 = client
        .create_backup()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    let b1_id = b1.backup().unwrap().backup_id().unwrap().to_string();

    let b2 = client
        .create_backup()
        .file_system_id(&fs_id)
        .send()
        .await
        .unwrap();
    let b2_id = b2.backup().unwrap().backup_id().unwrap().to_string();

    client
        .delete_backup()
        .backup_id(&b1_id)
        .send()
        .await
        .expect("delete first backup should succeed");

    let resp = client
        .describe_backups()
        .send()
        .await
        .expect("describe_backups should succeed");

    let backups = resp.backups();
    assert_eq!(backups.len(), 1);
    assert_eq!(backups[0].backup_id(), Some(b2_id.as_str()));
}

#[tokio::test]
async fn test_filesystem_dns_name_format() {
    let client = make_fsx_client().await;

    let resp = client
        .create_file_system()
        .file_system_type(aws_sdk_fsx::types::FileSystemType::Lustre)
        .storage_capacity(1200)
        .storage_type(aws_sdk_fsx::types::StorageType::Ssd)
        .subnet_ids(FAKE_SUBNET_ID)
        .send()
        .await
        .expect("create_file_system should succeed");

    let file_system = resp.file_system().expect("should have file_system");
    let fs_id = file_system.file_system_id().unwrap();
    let dns_name = file_system.dns_name().unwrap_or("");
    assert!(
        dns_name.starts_with(fs_id),
        "DNS name '{dns_name}' should start with fs_id '{fs_id}'"
    );
    assert!(
        dns_name.contains("amazonaws.com"),
        "DNS name '{dns_name}' should contain 'amazonaws.com'"
    );
}
