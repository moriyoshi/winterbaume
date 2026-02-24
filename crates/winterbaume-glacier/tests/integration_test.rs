use aws_sdk_glacier::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_glacier::{GlacierService, GlacierStateView};

async fn make_glacier_client() -> aws_sdk_glacier::Client {
    let mock = MockAws::builder()
        .with_service(GlacierService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_glacier::config::Region::new("us-west-2"))
        .load()
        .await;

    aws_sdk_glacier::Client::new(&config)
}

#[tokio::test]
async fn test_describe_vault() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("myvault")
        .account_id("-")
        .send()
        .await
        .expect("create_vault should succeed");

    let describe = client
        .describe_vault()
        .vault_name("myvault")
        .account_id("-")
        .send()
        .await
        .expect("describe_vault should succeed");

    assert_eq!(describe.number_of_archives(), 0);
    assert_eq!(describe.size_in_bytes(), 0);
    assert!(describe.last_inventory_date().is_some());
    assert!(describe.creation_date().is_some());
    assert_eq!(describe.vault_name(), Some("myvault"));
    assert_eq!(
        describe.vault_arn(),
        Some("arn:aws:glacier:us-west-2:123456789012:vaults/myvault")
    );
}

#[tokio::test]
async fn test_delete_vault() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("myvault")
        .account_id("-")
        .send()
        .await
        .unwrap();

    client
        .delete_vault()
        .vault_name("myvault")
        .account_id("-")
        .send()
        .await
        .expect("delete_vault should succeed");

    let result = client
        .describe_vault()
        .vault_name("myvault")
        .account_id("-")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_vaults() {
    let client = make_glacier_client().await;

    // Initially empty
    let resp = client
        .list_vaults()
        .account_id("-")
        .send()
        .await
        .expect("list_vaults should succeed");
    assert!(resp.vault_list().is_empty());

    client
        .create_vault()
        .vault_name("vault1")
        .account_id("-")
        .send()
        .await
        .unwrap();
    client
        .create_vault()
        .vault_name("vault2")
        .account_id("-")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_vaults()
        .account_id("-")
        .send()
        .await
        .expect("list_vaults should succeed");
    let vault_names: Vec<&str> = resp
        .vault_list()
        .iter()
        .filter_map(|v| v.vault_name())
        .collect();
    assert!(vault_names.contains(&"vault1"));
    assert!(vault_names.contains(&"vault2"));

    // Verify vault format
    for vault in resp.vault_list() {
        assert_eq!(vault.number_of_archives(), 0);
        assert_eq!(vault.size_in_bytes(), 0);
        assert!(vault.last_inventory_date().is_some());
        assert!(vault.creation_date().is_some());
        assert!(vault.vault_name().is_some());
        let vault_name = vault.vault_name().unwrap();
        assert_eq!(
            vault.vault_arn(),
            Some(format!("arn:aws:glacier:us-west-2:123456789012:vaults/{vault_name}").as_str())
        );
    }

    // Delete one vault
    client
        .delete_vault()
        .vault_name("vault1")
        .account_id("-")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_vaults()
        .account_id("-")
        .send()
        .await
        .expect("list_vaults should succeed");
    let vault_names: Vec<&str> = resp
        .vault_list()
        .iter()
        .filter_map(|v| v.vault_name())
        .collect();
    assert!(!vault_names.contains(&"vault1"));
    assert!(vault_names.contains(&"vault2"));
}

#[tokio::test]
async fn test_upload_archive() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("asdf")
        .account_id("-")
        .send()
        .await
        .unwrap();

    let resp = client
        .upload_archive()
        .vault_name("asdf")
        .account_id("-")
        .archive_description("my archive")
        .body(aws_sdk_glacier::primitives::ByteStream::from_static(
            b"body of archive",
        ))
        .send()
        .await
        .expect("upload_archive should succeed");

    assert!(resp.checksum().is_some());
    assert!(resp.archive_id().is_some());
}

#[tokio::test]
async fn test_delete_archive() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("asdf")
        .account_id("-")
        .send()
        .await
        .unwrap();

    let archive = client
        .upload_archive()
        .vault_name("asdf")
        .account_id("-")
        .body(aws_sdk_glacier::primitives::ByteStream::from_static(
            b"body of archive",
        ))
        .send()
        .await
        .unwrap();

    let result = client
        .delete_archive()
        .vault_name("asdf")
        .account_id("-")
        .archive_id(archive.archive_id().unwrap())
        .send()
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_initiate_job() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("myname")
        .account_id("-")
        .send()
        .await
        .unwrap();

    let archive = client
        .upload_archive()
        .vault_name("myname")
        .account_id("-")
        .body(aws_sdk_glacier::primitives::ByteStream::from_static(
            b"body of archive",
        ))
        .send()
        .await
        .unwrap();

    let job = client
        .initiate_job()
        .vault_name("myname")
        .account_id("-")
        .job_parameters(
            aws_sdk_glacier::types::JobParameters::builder()
                .archive_id(archive.archive_id().unwrap())
                .r#type("archive-retrieval")
                .build(),
        )
        .send()
        .await
        .expect("initiate_job should succeed");

    assert!(job.job_id().is_some());
    assert!(job.location().is_some());
}

#[tokio::test]
async fn test_describe_job() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("myname")
        .account_id("-")
        .send()
        .await
        .unwrap();

    let archive = client
        .upload_archive()
        .vault_name("myname")
        .account_id("-")
        .body(aws_sdk_glacier::primitives::ByteStream::from_static(
            b"body of archive",
        ))
        .send()
        .await
        .unwrap();

    let job = client
        .initiate_job()
        .vault_name("myname")
        .account_id("-")
        .job_parameters(
            aws_sdk_glacier::types::JobParameters::builder()
                .archive_id(archive.archive_id().unwrap())
                .r#type("archive-retrieval")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let describe = client
        .describe_job()
        .vault_name("myname")
        .account_id("-")
        .job_id(job.job_id().unwrap())
        .send()
        .await
        .expect("describe_job should succeed");

    assert_eq!(describe.job_id(), Some(job.job_id().unwrap()));
    assert_eq!(
        describe.action(),
        Some(&aws_sdk_glacier::types::ActionCode::ArchiveRetrieval)
    );
    assert_eq!(describe.archive_id(), Some(archive.archive_id().unwrap()));
    assert_eq!(
        describe.vault_arn(),
        Some("arn:aws:glacier:us-west-2:123456789012:vaults/myname")
    );
    assert!(describe.creation_date().is_some());
    assert_eq!(
        describe.status_code(),
        Some(&aws_sdk_glacier::types::StatusCode::InProgress)
    );
    assert_eq!(describe.archive_size_in_bytes(), Some(0));
    assert_eq!(describe.inventory_size_in_bytes(), Some(0));
    assert_eq!(describe.tier(), Some("Standard"));
}

#[tokio::test]
async fn test_list_jobs() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("myname")
        .account_id("-")
        .send()
        .await
        .unwrap();

    let archive1 = client
        .upload_archive()
        .vault_name("myname")
        .account_id("-")
        .body(aws_sdk_glacier::primitives::ByteStream::from_static(
            b"first archive",
        ))
        .send()
        .await
        .unwrap();

    let archive2 = client
        .upload_archive()
        .vault_name("myname")
        .account_id("-")
        .body(aws_sdk_glacier::primitives::ByteStream::from_static(
            b"second archive",
        ))
        .send()
        .await
        .unwrap();

    let job1 = client
        .initiate_job()
        .vault_name("myname")
        .account_id("-")
        .job_parameters(
            aws_sdk_glacier::types::JobParameters::builder()
                .archive_id(archive1.archive_id().unwrap())
                .r#type("archive-retrieval")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let job2 = client
        .initiate_job()
        .vault_name("myname")
        .account_id("-")
        .job_parameters(
            aws_sdk_glacier::types::JobParameters::builder()
                .archive_id(archive2.archive_id().unwrap())
                .r#type("archive-retrieval")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let list = client
        .list_jobs()
        .vault_name("myname")
        .account_id("-")
        .send()
        .await
        .expect("list_jobs should succeed");

    let jobs = list.job_list();
    let found_job_ids: Vec<Option<&str>> = jobs.iter().map(|j| j.job_id()).collect();
    assert!(found_job_ids.contains(&job1.job_id()));
    assert!(found_job_ids.contains(&job2.job_id()));

    // Verify correct archive associations
    let found_job1 = jobs.iter().find(|j| j.job_id() == job1.job_id()).unwrap();
    assert_eq!(found_job1.archive_id(), archive1.archive_id());
    let found_job2 = jobs.iter().find(|j| j.job_id() == job2.job_id()).unwrap();
    assert_eq!(found_job2.archive_id(), archive2.archive_id());

    // Verify all jobs have correct format
    for job in jobs {
        assert!(job.job_id().is_some());
        assert!(job.action().is_some());
        assert!(job.vault_arn().is_some());
        assert!(job.creation_date().is_some());
        assert!(job.status_code().is_some());
        assert!(job.tier().is_some());
    }
}

#[tokio::test]
async fn test_vault_name_with_special_characters() {
    let client = make_glacier_client().await;

    let vault_name = "Vault.name-with_Special.characters";

    client
        .create_vault()
        .vault_name(vault_name)
        .account_id("-")
        .send()
        .await
        .expect("create_vault with special chars should succeed");

    let describe = client
        .describe_vault()
        .vault_name(vault_name)
        .account_id("-")
        .send()
        .await
        .expect("describe_vault should succeed");

    assert_eq!(describe.vault_name(), Some(vault_name));
}

#[tokio::test]
async fn test_upload_zip_archive() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("asdf")
        .account_id("-")
        .send()
        .await
        .unwrap();

    // Upload a binary archive (simulating a zip/gz file)
    let binary_content: Vec<u8> = (0u8..=255u8).cycle().take(512).collect();

    let resp = client
        .upload_archive()
        .vault_name("asdf")
        .account_id("-")
        .body(aws_sdk_glacier::primitives::ByteStream::from(
            binary_content,
        ))
        .send()
        .await
        .expect("upload_archive with binary content should succeed");

    assert!(resp.checksum().is_some());
    assert!(resp.archive_id().is_some());
}

#[tokio::test]
async fn test_get_job_output() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("myname")
        .account_id("-")
        .send()
        .await
        .unwrap();

    let archive = client
        .upload_archive()
        .vault_name("myname")
        .account_id("-")
        .body(aws_sdk_glacier::primitives::ByteStream::from_static(
            b"contents of archive",
        ))
        .send()
        .await
        .unwrap();

    let job = client
        .initiate_job()
        .vault_name("myname")
        .account_id("-")
        .job_parameters(
            aws_sdk_glacier::types::JobParameters::builder()
                .archive_id(archive.archive_id().unwrap())
                .r#type("archive-retrieval")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let job_id = job.job_id().unwrap().to_string();

    // The job completes after a few seconds (Standard tier = 5s in mock)
    // Poll until complete, up to 10 seconds
    let mut output = None;
    for _ in 0..20 {
        match client
            .get_job_output()
            .vault_name("myname")
            .account_id("-")
            .job_id(&job_id)
            .send()
            .await
        {
            Ok(resp) => {
                output = Some(resp);
                break;
            }
            Err(_) => {
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
        }
    }

    let output = output.expect("get_job_output should succeed within 10 seconds");
    assert_eq!(output.status(), 200);
    assert_eq!(output.content_type(), Some("application/octet-stream"));
    let body_bytes = output
        .body
        .collect()
        .await
        .expect("should collect body")
        .into_bytes();
    assert_eq!(&body_bytes[..], b"contents of archive");
}

#[tokio::test]
async fn test_initiate_job_vault_not_found() {
    let client = make_glacier_client().await;

    let result = client
        .initiate_job()
        .vault_name("nonexistent")
        .account_id("-")
        .job_parameters(
            aws_sdk_glacier::types::JobParameters::builder()
                .archive_id("fake-archive")
                .r#type("archive-retrieval")
                .build(),
        )
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_vault_not_found() {
    let client = make_glacier_client().await;

    let result = client
        .describe_vault()
        .vault_name("nonexistent-vault")
        .account_id("-")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_upload_multiple_archives_to_same_vault() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("multivault")
        .account_id("-")
        .send()
        .await
        .unwrap();

    let resp1 = client
        .upload_archive()
        .vault_name("multivault")
        .account_id("-")
        .archive_description("first")
        .body(aws_sdk_glacier::primitives::ByteStream::from_static(
            b"first archive data",
        ))
        .send()
        .await
        .expect("first upload should succeed");

    let resp2 = client
        .upload_archive()
        .vault_name("multivault")
        .account_id("-")
        .archive_description("second")
        .body(aws_sdk_glacier::primitives::ByteStream::from_static(
            b"second archive data",
        ))
        .send()
        .await
        .expect("second upload should succeed");

    let resp3 = client
        .upload_archive()
        .vault_name("multivault")
        .account_id("-")
        .archive_description("third")
        .body(aws_sdk_glacier::primitives::ByteStream::from_static(
            b"third archive data",
        ))
        .send()
        .await
        .expect("third upload should succeed");

    // All archive IDs must be present and non-empty
    assert!(resp1.archive_id().is_some());
    assert!(resp2.archive_id().is_some());
    assert!(resp3.archive_id().is_some());
    // All checksums must be present
    assert!(resp1.checksum().is_some());
    assert!(resp2.checksum().is_some());
    assert!(resp3.checksum().is_some());
}

#[tokio::test]
async fn test_describe_vault_reflects_archive_stats() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("statvault")
        .account_id("-")
        .send()
        .await
        .unwrap();

    // Initially no archives
    let desc = client
        .describe_vault()
        .vault_name("statvault")
        .account_id("-")
        .send()
        .await
        .expect("describe_vault should succeed");
    assert_eq!(desc.number_of_archives(), 0);
    assert_eq!(desc.size_in_bytes(), 0);

    let body1 = b"hello world";
    let body2 = b"second file content here";

    client
        .upload_archive()
        .vault_name("statvault")
        .account_id("-")
        .body(aws_sdk_glacier::primitives::ByteStream::from_static(body1))
        .send()
        .await
        .unwrap();

    client
        .upload_archive()
        .vault_name("statvault")
        .account_id("-")
        .body(aws_sdk_glacier::primitives::ByteStream::from_static(body2))
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_vault()
        .vault_name("statvault")
        .account_id("-")
        .send()
        .await
        .expect("describe_vault should succeed after uploads");

    assert_eq!(desc.number_of_archives(), 2);
    assert_eq!(desc.size_in_bytes(), (body1.len() + body2.len()) as i64);
}

#[tokio::test]
async fn test_initiate_inventory_retrieval_job() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("invvault")
        .account_id("-")
        .send()
        .await
        .unwrap();

    let job = client
        .initiate_job()
        .vault_name("invvault")
        .account_id("-")
        .job_parameters(
            aws_sdk_glacier::types::JobParameters::builder()
                .r#type("inventory-retrieval")
                .build(),
        )
        .send()
        .await
        .expect("initiate inventory-retrieval job should succeed");

    assert!(job.job_id().is_some());
    assert!(job.location().is_some());

    let describe = client
        .describe_job()
        .vault_name("invvault")
        .account_id("-")
        .job_id(job.job_id().unwrap())
        .send()
        .await
        .expect("describe_job should succeed");

    assert_eq!(
        describe.action(),
        Some(&aws_sdk_glacier::types::ActionCode::InventoryRetrieval)
    );
    assert_eq!(describe.job_id(), Some(job.job_id().unwrap()));
    assert!(describe.archive_id().is_none());
}

#[tokio::test]
async fn test_get_inventory_job_output() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("invoutvault")
        .account_id("-")
        .send()
        .await
        .unwrap();

    // Upload a couple of archives so the inventory is non-empty
    client
        .upload_archive()
        .vault_name("invoutvault")
        .account_id("-")
        .archive_description("doc1")
        .body(aws_sdk_glacier::primitives::ByteStream::from_static(
            b"document one",
        ))
        .send()
        .await
        .unwrap();

    client
        .upload_archive()
        .vault_name("invoutvault")
        .account_id("-")
        .archive_description("doc2")
        .body(aws_sdk_glacier::primitives::ByteStream::from_static(
            b"document two",
        ))
        .send()
        .await
        .unwrap();

    let job = client
        .initiate_job()
        .vault_name("invoutvault")
        .account_id("-")
        .job_parameters(
            aws_sdk_glacier::types::JobParameters::builder()
                .r#type("inventory-retrieval")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let job_id = job.job_id().unwrap().to_string();

    // Poll until the job output is ready (Standard tier = 5s in mock)
    let mut output = None;
    for _ in 0..20 {
        match client
            .get_job_output()
            .vault_name("invoutvault")
            .account_id("-")
            .job_id(&job_id)
            .send()
            .await
        {
            Ok(resp) => {
                output = Some(resp);
                break;
            }
            Err(_) => {
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
        }
    }

    let output = output.expect("get_job_output for inventory should succeed within 10 seconds");
    assert_eq!(output.status(), 200);

    let body_bytes = output
        .body
        .collect()
        .await
        .expect("should collect inventory body")
        .into_bytes();

    let inventory: serde_json::Value =
        serde_json::from_slice(&body_bytes).expect("inventory output should be valid JSON");

    assert!(inventory.get("VaultARN").is_some());
    let archive_list = inventory
        .get("ArchiveList")
        .and_then(|v| v.as_array())
        .expect("ArchiveList should be an array");
    assert_eq!(archive_list.len(), 2);

    for entry in archive_list {
        assert!(entry.get("ArchiveId").is_some());
        assert!(entry.get("ArchiveDescription").is_some());
        assert!(entry.get("CreationDate").is_some());
        assert!(entry.get("Size").is_some());
        assert!(entry.get("SHA256TreeHash").is_some());
    }
}

#[tokio::test]
async fn test_list_jobs_empty_vault() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("emptyjobvault")
        .account_id("-")
        .send()
        .await
        .unwrap();

    let list = client
        .list_jobs()
        .vault_name("emptyjobvault")
        .account_id("-")
        .send()
        .await
        .expect("list_jobs on vault with no jobs should succeed");

    assert!(list.job_list().is_empty());
}

#[tokio::test]
async fn test_describe_job_not_found() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("jobvault")
        .account_id("-")
        .send()
        .await
        .unwrap();

    let result = client
        .describe_job()
        .vault_name("jobvault")
        .account_id("-")
        .job_id("nonexistent-job-id")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_job_tier_expedited() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("tiervault")
        .account_id("-")
        .send()
        .await
        .unwrap();

    let archive = client
        .upload_archive()
        .vault_name("tiervault")
        .account_id("-")
        .body(aws_sdk_glacier::primitives::ByteStream::from_static(
            b"expedited data",
        ))
        .send()
        .await
        .unwrap();

    let job = client
        .initiate_job()
        .vault_name("tiervault")
        .account_id("-")
        .job_parameters(
            aws_sdk_glacier::types::JobParameters::builder()
                .archive_id(archive.archive_id().unwrap())
                .r#type("archive-retrieval")
                .tier("Expedited")
                .build(),
        )
        .send()
        .await
        .expect("initiate_job with Expedited tier should succeed");

    let job_id = job.job_id().unwrap().to_string();

    let describe = client
        .describe_job()
        .vault_name("tiervault")
        .account_id("-")
        .job_id(&job_id)
        .send()
        .await
        .expect("describe_job should succeed");

    assert_eq!(describe.tier(), Some("Expedited"));

    // Expedited tier completes in ~2s; poll for output
    let mut output = None;
    for _ in 0..12 {
        match client
            .get_job_output()
            .vault_name("tiervault")
            .account_id("-")
            .job_id(&job_id)
            .send()
            .await
        {
            Ok(resp) => {
                output = Some(resp);
                break;
            }
            Err(_) => {
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
        }
    }

    let output = output.expect("Expedited job output should be ready within 6 seconds");
    let body_bytes = output
        .body
        .collect()
        .await
        .expect("should collect body")
        .into_bytes();
    assert_eq!(&body_bytes[..], b"expedited data");
}

#[tokio::test]
async fn test_delete_archive_then_inventory_reflects_removal() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("delvault")
        .account_id("-")
        .send()
        .await
        .unwrap();

    let archive = client
        .upload_archive()
        .vault_name("delvault")
        .account_id("-")
        .body(aws_sdk_glacier::primitives::ByteStream::from_static(
            b"to be deleted",
        ))
        .send()
        .await
        .unwrap();

    let archive_id = archive.archive_id().unwrap().to_string();

    // Delete the archive
    client
        .delete_archive()
        .vault_name("delvault")
        .account_id("-")
        .archive_id(&archive_id)
        .send()
        .await
        .expect("delete_archive should succeed");

    // describe_vault should reflect 0 archives
    let desc = client
        .describe_vault()
        .vault_name("delvault")
        .account_id("-")
        .send()
        .await
        .expect("describe_vault should succeed");
    assert_eq!(desc.number_of_archives(), 0);
    assert_eq!(desc.size_in_bytes(), 0);
}

// --- Tags ---

#[tokio::test]
async fn test_add_tags_to_vault() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("tagvault")
        .account_id("-")
        .send()
        .await
        .unwrap();

    client
        .add_tags_to_vault()
        .vault_name("tagvault")
        .account_id("-")
        .tags("env", "test")
        .tags("team", "platform")
        .send()
        .await
        .expect("add_tags_to_vault should succeed");

    let resp = client
        .list_tags_for_vault()
        .vault_name("tagvault")
        .account_id("-")
        .send()
        .await
        .expect("list_tags_for_vault should succeed");

    let tags = resp.tags().expect("should have tags");
    assert_eq!(tags.get("env"), Some(&"test".to_string()));
    assert_eq!(tags.get("team"), Some(&"platform".to_string()));
}

#[tokio::test]
async fn test_remove_tags_from_vault() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("tagvault2")
        .account_id("-")
        .send()
        .await
        .unwrap();

    client
        .add_tags_to_vault()
        .vault_name("tagvault2")
        .account_id("-")
        .tags("k1", "v1")
        .tags("k2", "v2")
        .tags("k3", "v3")
        .send()
        .await
        .unwrap();

    client
        .remove_tags_from_vault()
        .vault_name("tagvault2")
        .account_id("-")
        .tag_keys("k1")
        .tag_keys("k3")
        .send()
        .await
        .expect("remove_tags_from_vault should succeed");

    let resp = client
        .list_tags_for_vault()
        .vault_name("tagvault2")
        .account_id("-")
        .send()
        .await
        .unwrap();

    let tags = resp.tags().expect("should have tags map");
    assert!(!tags.contains_key("k1"));
    assert!(tags.contains_key("k2"));
    assert!(!tags.contains_key("k3"));
}

#[tokio::test]
async fn test_list_tags_empty() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("notags")
        .account_id("-")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_vault()
        .vault_name("notags")
        .account_id("-")
        .send()
        .await
        .expect("list_tags_for_vault on vault with no tags should succeed");

    assert!(resp.tags().map(|t| t.is_empty()).unwrap_or(true));
}

// --- Vault Access Policy ---

#[tokio::test]
async fn test_vault_access_policy_lifecycle() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("policyvault")
        .account_id("-")
        .send()
        .await
        .unwrap();

    let policy_str = r#"{"Version":"2012-10-17","Statement":[]}"#;
    client
        .set_vault_access_policy()
        .vault_name("policyvault")
        .account_id("-")
        .policy(
            aws_sdk_glacier::types::VaultAccessPolicy::builder()
                .policy(policy_str)
                .build(),
        )
        .send()
        .await
        .expect("set_vault_access_policy should succeed");

    let resp = client
        .get_vault_access_policy()
        .vault_name("policyvault")
        .account_id("-")
        .send()
        .await
        .expect("get_vault_access_policy should succeed");

    assert!(resp.policy().is_some());
    assert_eq!(resp.policy().and_then(|p| p.policy()), Some(policy_str));

    client
        .delete_vault_access_policy()
        .vault_name("policyvault")
        .account_id("-")
        .send()
        .await
        .expect("delete_vault_access_policy should succeed");

    let resp2 = client
        .get_vault_access_policy()
        .vault_name("policyvault")
        .account_id("-")
        .send()
        .await
        .expect("get_vault_access_policy after delete should succeed");

    // Policy should be gone after delete
    assert!(resp2.policy().is_none() || resp2.policy().and_then(|p| p.policy()).is_none());
}

// --- Vault Notifications ---

#[tokio::test]
async fn test_vault_notifications_lifecycle() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("notisvault")
        .account_id("-")
        .send()
        .await
        .unwrap();

    client
        .set_vault_notifications()
        .vault_name("notisvault")
        .account_id("-")
        .vault_notification_config(
            aws_sdk_glacier::types::VaultNotificationConfig::builder()
                .sns_topic("arn:aws:sns:us-west-2:123456789012:mytopic")
                .events("ArchiveRetrievalCompleted")
                .events("InventoryRetrievalCompleted")
                .build(),
        )
        .send()
        .await
        .expect("set_vault_notifications should succeed");

    let resp = client
        .get_vault_notifications()
        .vault_name("notisvault")
        .account_id("-")
        .send()
        .await
        .expect("get_vault_notifications should succeed");

    let config = resp
        .vault_notification_config()
        .expect("should have config");
    assert_eq!(
        config.sns_topic(),
        Some("arn:aws:sns:us-west-2:123456789012:mytopic")
    );
    assert!(
        config
            .events()
            .contains(&"ArchiveRetrievalCompleted".to_string())
    );
    assert!(
        config
            .events()
            .contains(&"InventoryRetrievalCompleted".to_string())
    );

    client
        .delete_vault_notifications()
        .vault_name("notisvault")
        .account_id("-")
        .send()
        .await
        .expect("delete_vault_notifications should succeed");

    // Getting notifications after delete - config should be absent or empty
    let resp2 = client
        .get_vault_notifications()
        .vault_name("notisvault")
        .account_id("-")
        .send()
        .await
        .expect("get_vault_notifications after delete should succeed");
    assert!(resp2.vault_notification_config().is_none());
}

// --- Vault Lock ---

#[tokio::test]
async fn test_vault_lock_lifecycle() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("lockvault")
        .account_id("-")
        .send()
        .await
        .unwrap();

    let init_resp = client
        .initiate_vault_lock()
        .vault_name("lockvault")
        .account_id("-")
        .policy(
            aws_sdk_glacier::types::VaultLockPolicy::builder()
                .policy(r#"{"Version":"2012-10-17","Statement":[]}"#)
                .build(),
        )
        .send()
        .await
        .expect("initiate_vault_lock should succeed");

    let lock_id = init_resp.lock_id().expect("should have lock_id");

    let get_resp = client
        .get_vault_lock()
        .vault_name("lockvault")
        .account_id("-")
        .send()
        .await
        .expect("get_vault_lock should succeed");

    assert_eq!(get_resp.state(), Some("InProgress"));
    assert!(get_resp.creation_date().is_some());
    assert!(get_resp.expiration_date().is_some());

    client
        .complete_vault_lock()
        .vault_name("lockvault")
        .account_id("-")
        .lock_id(lock_id)
        .send()
        .await
        .expect("complete_vault_lock should succeed");

    let get_resp2 = client
        .get_vault_lock()
        .vault_name("lockvault")
        .account_id("-")
        .send()
        .await
        .expect("get_vault_lock after complete should succeed");

    assert_eq!(get_resp2.state(), Some("Locked"));
}

#[tokio::test]
async fn test_abort_vault_lock() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("abortlockvault")
        .account_id("-")
        .send()
        .await
        .unwrap();

    client
        .initiate_vault_lock()
        .vault_name("abortlockvault")
        .account_id("-")
        .send()
        .await
        .expect("initiate_vault_lock should succeed");

    client
        .abort_vault_lock()
        .vault_name("abortlockvault")
        .account_id("-")
        .send()
        .await
        .expect("abort_vault_lock should succeed");

    // After abort, lock state should be absent
    let resp = client
        .get_vault_lock()
        .vault_name("abortlockvault")
        .account_id("-")
        .send()
        .await
        .expect("get_vault_lock after abort should succeed");

    assert!(resp.state().is_none() || resp.state() == Some(""));
}

// --- Multipart Uploads ---

#[tokio::test]
async fn test_multipart_upload_lifecycle() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("mpvault")
        .account_id("-")
        .send()
        .await
        .unwrap();

    let init_resp = client
        .initiate_multipart_upload()
        .vault_name("mpvault")
        .account_id("-")
        .archive_description("my multipart archive")
        .part_size("4194304")
        .send()
        .await
        .expect("initiate_multipart_upload should succeed");

    let upload_id = init_resp.upload_id().expect("should have upload_id");
    assert!(init_resp.location().is_some());

    // List multipart uploads - should show our in-progress upload
    let list_resp = client
        .list_multipart_uploads()
        .vault_name("mpvault")
        .account_id("-")
        .send()
        .await
        .expect("list_multipart_uploads should succeed");

    let uploads = list_resp.uploads_list();
    assert_eq!(uploads.len(), 1);
    assert_eq!(uploads[0].multipart_upload_id(), Some(upload_id));
    assert_eq!(
        uploads[0].archive_description(),
        Some("my multipart archive")
    );

    // Upload a part
    client
        .upload_multipart_part()
        .vault_name("mpvault")
        .account_id("-")
        .upload_id(upload_id)
        .range("bytes 0-4194303/*")
        .body(aws_sdk_glacier::primitives::ByteStream::from_static(
            &[0u8; 64],
        ))
        .send()
        .await
        .expect("upload_multipart_part should succeed");

    // List parts
    let parts_resp = client
        .list_parts()
        .vault_name("mpvault")
        .account_id("-")
        .upload_id(upload_id)
        .send()
        .await
        .expect("list_parts should succeed");

    assert_eq!(parts_resp.multipart_upload_id(), Some(upload_id));
    assert_eq!(
        parts_resp.archive_description(),
        Some("my multipart archive")
    );
    assert!(!parts_resp.parts().is_empty());

    // Complete the multipart upload
    let complete_resp = client
        .complete_multipart_upload()
        .vault_name("mpvault")
        .account_id("-")
        .upload_id(upload_id)
        .archive_size("64")
        .send()
        .await
        .expect("complete_multipart_upload should succeed");

    assert!(complete_resp.archive_id().is_some());
    assert!(complete_resp.location().is_some());

    // After complete, upload should be gone from list
    let list_resp2 = client
        .list_multipart_uploads()
        .vault_name("mpvault")
        .account_id("-")
        .send()
        .await
        .unwrap();
    assert!(list_resp2.uploads_list().is_empty());
}

#[tokio::test]
async fn test_abort_multipart_upload() {
    let client = make_glacier_client().await;

    client
        .create_vault()
        .vault_name("abortmpvault")
        .account_id("-")
        .send()
        .await
        .unwrap();

    let init_resp = client
        .initiate_multipart_upload()
        .vault_name("abortmpvault")
        .account_id("-")
        .send()
        .await
        .unwrap();

    let upload_id = init_resp.upload_id().expect("should have upload_id");

    client
        .abort_multipart_upload()
        .vault_name("abortmpvault")
        .account_id("-")
        .upload_id(upload_id)
        .send()
        .await
        .expect("abort_multipart_upload should succeed");

    let list_resp = client
        .list_multipart_uploads()
        .vault_name("abortmpvault")
        .account_id("-")
        .send()
        .await
        .unwrap();
    assert!(list_resp.uploads_list().is_empty());
}

// --- Data Retrieval Policy ---

#[tokio::test]
async fn test_data_retrieval_policy_lifecycle() {
    let client = make_glacier_client().await;

    // Initially should return empty/default policy
    let resp = client
        .get_data_retrieval_policy()
        .account_id("-")
        .send()
        .await
        .expect("get_data_retrieval_policy should succeed");

    // Set a policy
    client
        .set_data_retrieval_policy()
        .account_id("-")
        .policy(
            aws_sdk_glacier::types::DataRetrievalPolicy::builder()
                .rules(
                    aws_sdk_glacier::types::DataRetrievalRule::builder()
                        .strategy("BytesPerHour")
                        .bytes_per_hour(10737418240)
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("set_data_retrieval_policy should succeed");

    let resp2 = client
        .get_data_retrieval_policy()
        .account_id("-")
        .send()
        .await
        .expect("get_data_retrieval_policy after set should succeed");

    let policy = resp2.policy().expect("should have policy");
    let rules = policy.rules();
    assert!(!rules.is_empty());
    assert_eq!(rules[0].strategy(), Some("BytesPerHour"));
    assert_eq!(rules[0].bytes_per_hour(), Some(10737418240));
    let _ = resp; // suppress unused warning
}

// --- Provisioned Capacity ---

#[tokio::test]
async fn test_provisioned_capacity_lifecycle() {
    let client = make_glacier_client().await;

    // Initially empty
    let list_resp = client
        .list_provisioned_capacity()
        .account_id("-")
        .send()
        .await
        .expect("list_provisioned_capacity should succeed");
    assert!(list_resp.provisioned_capacity_list().is_empty());

    // Purchase capacity
    let purchase_resp = client
        .purchase_provisioned_capacity()
        .account_id("-")
        .send()
        .await
        .expect("purchase_provisioned_capacity should succeed");

    let capacity_id = purchase_resp
        .capacity_id()
        .expect("should have capacity_id");

    let list_resp2 = client
        .list_provisioned_capacity()
        .account_id("-")
        .send()
        .await
        .expect("list_provisioned_capacity after purchase should succeed");

    let caps = list_resp2.provisioned_capacity_list();
    assert_eq!(caps.len(), 1);
    assert_eq!(caps[0].capacity_id(), Some(capacity_id));
    assert!(caps[0].start_date().is_some());
    assert!(caps[0].expiration_date().is_some());
}

// --- State Views ---

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;

    let svc = GlacierService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore("123456789012", "us-east-1", GlacierStateView::default())
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
async fn test_state_view_snapshot_restore_merge() {
    use std::collections::HashMap;

    use winterbaume_core::StatefulService;
    use winterbaume_glacier::views::{GlacierStateView, VaultView};

    let svc = GlacierService::new();

    // Create a view with one vault
    let mut vaults = HashMap::new();
    vaults.insert(
        "restore-vault".to_string(),
        VaultView {
            vault_name: "restore-vault".to_string(),
            arn: "arn:aws:glacier:us-east-1:123456789012:vaults/restore-vault".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            archives: HashMap::new(),
            jobs: HashMap::new(),
            tags: HashMap::new(),
            access_policy: None,
            notification_config: None,
            vault_lock: None,
            multipart_uploads: HashMap::new(),
        },
    );
    let view = GlacierStateView {
        vaults: vaults.clone(),
        data_retrieval_policy: None,
        provisioned_capacity: vec![],
    };

    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    let snapshot = svc.snapshot("123456789012", "us-east-1").await;
    assert!(snapshot.vaults.contains_key("restore-vault"));

    // Merge a second vault
    let mut vaults2 = HashMap::new();
    vaults2.insert(
        "merge-vault".to_string(),
        VaultView {
            vault_name: "merge-vault".to_string(),
            arn: "arn:aws:glacier:us-east-1:123456789012:vaults/merge-vault".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            archives: HashMap::new(),
            jobs: HashMap::new(),
            tags: HashMap::new(),
            access_policy: None,
            notification_config: None,
            vault_lock: None,
            multipart_uploads: HashMap::new(),
        },
    );
    let view2 = GlacierStateView {
        vaults: vaults2,
        data_retrieval_policy: None,
        provisioned_capacity: vec![],
    };
    svc.merge("123456789012", "us-east-1", view2).await.unwrap();

    let snapshot2 = svc.snapshot("123456789012", "us-east-1").await;
    // Both vaults should be present after merge
    assert!(snapshot2.vaults.contains_key("restore-vault"));
    assert!(snapshot2.vaults.contains_key("merge-vault"));
}

// ---------------------------------------------------------------------------
// BlobBackedService smoke tests
// ---------------------------------------------------------------------------

mod blob_backed {
    use std::collections::HashMap;
    use std::pin::Pin;
    use std::sync::Arc;

    use bytes::Bytes;
    use tokio::io::AsyncReadExt;
    use winterbaume_core::{
        BlobBackedService, BlobExportEntry, BlobSource, BlobStore, BlobVisitor, MemVfs,
        StatefulService, VfsError,
    };
    use winterbaume_glacier::GlacierService;
    use winterbaume_glacier::views::{ArchiveView, GlacierStateView, VaultView};

    struct BlobCollector {
        blobs: HashMap<String, Bytes>,
    }

    impl BlobCollector {
        fn new() -> Self {
            Self {
                blobs: HashMap::new(),
            }
        }
    }

    impl BlobVisitor for BlobCollector {
        fn visit(
            &mut self,
            batch: Vec<BlobExportEntry>,
        ) -> Pin<Box<dyn std::future::Future<Output = Result<(), VfsError>> + Send + '_>> {
            Box::pin(async move {
                for mut entry in batch {
                    let mut buf = Vec::new();
                    entry
                        .reader
                        .read_to_end(&mut buf)
                        .await
                        .map_err(VfsError::Io)?;
                    self.blobs.insert(entry.key.clone(), Bytes::from(buf));
                }
                Ok(())
            })
        }
    }

    struct MapBlobSource {
        data: HashMap<String, Bytes>,
    }

    impl BlobSource for MapBlobSource {
        fn fetch(
            &mut self,
            key: String,
        ) -> Pin<
            Box<
                dyn std::future::Future<
                        Output = Result<Box<dyn tokio::io::AsyncRead + Send + Unpin>, VfsError>,
                    > + Send
                    + '_,
            >,
        > {
            Box::pin(async move {
                let bytes = self
                    .data
                    .get(&key)
                    .cloned()
                    .ok_or(VfsError::NotFound(key))?;
                Ok(Box::new(std::io::Cursor::new(bytes))
                    as Box<dyn tokio::io::AsyncRead + Send + Unpin>)
            })
        }
    }

    fn make_service_with_shared_vfs() -> (GlacierService, BlobStore) {
        let vfs = Arc::new(MemVfs::new());
        let svc = GlacierService::with_vfs(vfs.clone());
        // The service scopes blobs to glacier/{account_id}/{region}; mirror that here.
        let blobs = BlobStore::new(vfs, "glacier").child("111111111111/us-east-1");
        (svc, blobs)
    }

    fn seed_view(blob_key: &str) -> GlacierStateView {
        let mut archives = HashMap::new();
        archives.insert(
            "archive-001".to_string(),
            ArchiveView {
                archive_id: "archive-001".to_string(),
                blob_key: blob_key.to_string(),
                size: 7,
                sha256: "deadbeef".to_string(),
                description: "test archive".to_string(),
                creation_date: "2026-01-01T00:00:00Z".to_string(),
            },
        );
        let mut vaults = HashMap::new();
        vaults.insert(
            "test-vault".to_string(),
            VaultView {
                vault_name: "test-vault".to_string(),
                arn: "arn:aws:glacier:us-east-1:111111111111:vaults/test-vault".to_string(),
                created_at: "2026-01-01T00:00:00Z".to_string(),
                archives,
                jobs: HashMap::new(),
                tags: HashMap::new(),
                access_policy: None,
                notification_config: None,
                vault_lock: None,
                multipart_uploads: HashMap::new(),
            },
        );
        GlacierStateView {
            vaults,
            data_retrieval_policy: None,
            provisioned_capacity: vec![],
        }
    }

    #[tokio::test]
    async fn roundtrip_snapshot_restore_with_blobs() {
        let (src_svc, src_blobs) = make_service_with_shared_vfs();
        let blob_key = "vault/archive-001";

        // Seed.
        src_blobs
            .put(blob_key, Bytes::from_static(b"glacial"))
            .await
            .unwrap();
        src_svc
            .restore("111111111111", "us-east-1", seed_view(blob_key))
            .await
            .unwrap();

        // Export.
        let mut collector = BlobCollector::new();
        let view = src_svc
            .snapshot_with_blobs("111111111111", "us-east-1", &mut collector)
            .await
            .unwrap();

        assert_eq!(view.vaults.len(), 1);
        assert!(collector.blobs.contains_key(blob_key));
        assert_eq!(collector.blobs[blob_key], Bytes::from_static(b"glacial"));

        // Import into fresh service.
        let (dst_svc, dst_blobs) = make_service_with_shared_vfs();
        let mut source = MapBlobSource {
            data: collector.blobs,
        };
        dst_svc
            .restore_with_blobs("111111111111", "us-east-1", view, &mut source)
            .await
            .unwrap();

        let dst_view = dst_svc.snapshot("111111111111", "us-east-1").await;
        assert_eq!(dst_view.vaults.len(), 1);
        assert_eq!(
            dst_view.vaults["test-vault"].archives["archive-001"].blob_key,
            blob_key
        );
        let blob = dst_blobs.get(blob_key).await.unwrap();
        assert_eq!(blob, Bytes::from_static(b"glacial"));
    }

    #[tokio::test]
    async fn snapshot_with_blobs_empty_state_exports_nothing() {
        let (svc, _) = make_service_with_shared_vfs();
        let mut collector = BlobCollector::new();
        let view = svc
            .snapshot_with_blobs("111111111111", "us-east-1", &mut collector)
            .await
            .unwrap();
        assert!(view.vaults.is_empty());
        assert!(collector.blobs.is_empty());
    }
}
