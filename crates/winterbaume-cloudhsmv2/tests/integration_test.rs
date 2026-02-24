use aws_sdk_cloudhsmv2::config::BehaviorVersion;
use winterbaume_cloudhsmv2::CloudHsmV2Service;
use winterbaume_core::MockAws;

async fn make_cloudhsmv2_client() -> aws_sdk_cloudhsmv2::Client {
    let mock = MockAws::builder()
        .with_service(CloudHsmV2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudhsmv2::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_cloudhsmv2::Client::new(&config)
}

#[tokio::test]
async fn test_create_cluster() {
    let client = make_cloudhsmv2_client().await;

    let resp = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-12345678")
        .send()
        .await
        .expect("create_cluster should succeed");

    let cluster = resp.cluster().expect("should have cluster");
    assert!(cluster.cluster_id().is_some());
    assert_eq!(cluster.hsm_type(), Some("hsm1.medium"));
}

#[tokio::test]
async fn test_describe_clusters() {
    let client = make_cloudhsmv2_client().await;

    // Create two clusters
    client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-11111111")
        .send()
        .await
        .unwrap();

    client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-22222222")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_clusters()
        .send()
        .await
        .expect("describe_clusters should succeed");

    assert_eq!(resp.clusters().len(), 2);
}

#[tokio::test]
async fn test_delete_cluster() {
    let client = make_cloudhsmv2_client().await;

    let resp = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-12345678")
        .send()
        .await
        .unwrap();

    let cluster_id = resp.cluster().unwrap().cluster_id().unwrap().to_string();

    let del_resp = client
        .delete_cluster()
        .cluster_id(&cluster_id)
        .send()
        .await
        .expect("delete_cluster should succeed");

    let deleted_cluster = del_resp.cluster().expect("should have cluster");
    assert_eq!(
        deleted_cluster.state(),
        Some(&aws_sdk_cloudhsmv2::types::ClusterState::Deleted)
    );
}

#[tokio::test]
async fn test_initialize_cluster() {
    let client = make_cloudhsmv2_client().await;

    let resp = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-12345678")
        .send()
        .await
        .unwrap();

    let cluster_id = resp.cluster().unwrap().cluster_id().unwrap().to_string();

    let init_resp = client
        .initialize_cluster()
        .cluster_id(&cluster_id)
        .signed_cert("-----BEGIN CERTIFICATE-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8A\n-----END CERTIFICATE-----")
        .trust_anchor("-----BEGIN CERTIFICATE-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8A\n-----END CERTIFICATE-----")
        .send()
        .await
        .expect("initialize_cluster should succeed");

    assert!(init_resp.state().is_some());
}

#[tokio::test]
async fn test_delete_nonexistent_cluster() {
    let client = make_cloudhsmv2_client().await;

    let result = client
        .delete_cluster()
        .cluster_id("cluster-nonexistent")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_initialize_already_initialized_cluster() {
    let client = make_cloudhsmv2_client().await;

    let resp = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-12345678")
        .send()
        .await
        .unwrap();

    let cluster_id = resp.cluster().unwrap().cluster_id().unwrap().to_string();

    // First initialization should succeed
    client
        .initialize_cluster()
        .cluster_id(&cluster_id)
        .signed_cert("cert")
        .trust_anchor("anchor")
        .send()
        .await
        .unwrap();

    // Second initialization should fail
    let result = client
        .initialize_cluster()
        .cluster_id(&cluster_id)
        .signed_cert("cert")
        .trust_anchor("anchor")
        .send()
        .await;

    assert!(result.is_err());
}

// ============================================================================
// Tests derived from AWS documentation: AWS CloudHSM V2
// ============================================================================

#[tokio::test]
async fn test_create_cluster_with_tags() {
    let client = make_cloudhsmv2_client().await;

    let resp = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-12345678")
        .tag_list(
            aws_sdk_cloudhsmv2::types::Tag::builder()
                .key("Environment")
                .value("Test")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_cluster with tags should succeed");

    let cluster = resp.cluster().expect("should have cluster");
    let tags = cluster.tag_list();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "Environment");
    assert_eq!(tags[0].value(), "Test");
}

#[tokio::test]
async fn test_create_cluster_with_backup_retention_policy() {
    let client = make_cloudhsmv2_client().await;

    let policy = aws_sdk_cloudhsmv2::types::BackupRetentionPolicy::builder()
        .r#type(aws_sdk_cloudhsmv2::types::BackupRetentionType::Days)
        .value("7")
        .build();

    let resp = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-12345678")
        .backup_retention_policy(policy)
        .send()
        .await
        .expect("create_cluster with backup retention policy should succeed");

    let cluster = resp.cluster().expect("should have cluster");
    let brp = cluster
        .backup_retention_policy()
        .expect("should have backup_retention_policy");
    assert_eq!(
        brp.r#type(),
        Some(&aws_sdk_cloudhsmv2::types::BackupRetentionType::Days)
    );
    assert_eq!(brp.value(), Some("7"));
}

#[tokio::test]
async fn test_create_cluster_initial_state_is_uninitialized() {
    let client = make_cloudhsmv2_client().await;

    let resp = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-12345678")
        .send()
        .await
        .expect("create_cluster should succeed");

    let cluster = resp.cluster().expect("should have cluster");
    assert_eq!(
        cluster.state().map(|s| s.as_str()),
        Some("UNINITIALIZED"),
        "freshly created cluster should be UNINITIALIZED"
    );
}

#[tokio::test]
async fn test_describe_clusters_empty() {
    let client = make_cloudhsmv2_client().await;

    let resp = client
        .describe_clusters()
        .send()
        .await
        .expect("describe_clusters on empty store should succeed");

    assert_eq!(resp.clusters().len(), 0);
}

#[tokio::test]
async fn test_describe_clusters_filter_by_cluster_id() {
    let client = make_cloudhsmv2_client().await;

    let c1 = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-11111111")
        .send()
        .await
        .unwrap();
    let id1 = c1.cluster().unwrap().cluster_id().unwrap().to_string();

    client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-22222222")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_clusters()
        .filters("clusterIds", vec![id1.clone()])
        .send()
        .await
        .expect("describe_clusters with clusterIds filter should succeed");

    assert_eq!(resp.clusters().len(), 1);
    assert_eq!(resp.clusters()[0].cluster_id().unwrap(), id1.as_str());
}

#[tokio::test]
async fn test_describe_clusters_filter_by_state() {
    let client = make_cloudhsmv2_client().await;

    let c1 = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-11111111")
        .send()
        .await
        .unwrap();
    let id1 = c1.cluster().unwrap().cluster_id().unwrap().to_string();

    client
        .initialize_cluster()
        .cluster_id(&id1)
        .signed_cert("cert")
        .trust_anchor("anchor")
        .send()
        .await
        .unwrap();

    client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-22222222")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_clusters()
        .filters("states", vec!["INITIALIZED".to_string()])
        .send()
        .await
        .expect("describe_clusters with states filter should succeed");

    assert_eq!(resp.clusters().len(), 1);
    assert_eq!(resp.clusters()[0].cluster_id().unwrap(), id1.as_str());
}

#[tokio::test]
async fn test_delete_already_deleted_cluster() {
    let client = make_cloudhsmv2_client().await;

    let resp = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-12345678")
        .send()
        .await
        .unwrap();
    let cluster_id = resp.cluster().unwrap().cluster_id().unwrap().to_string();

    client
        .delete_cluster()
        .cluster_id(&cluster_id)
        .send()
        .await
        .expect("first delete_cluster should succeed");

    let result = client.delete_cluster().cluster_id(&cluster_id).send().await;

    assert!(
        result.is_err(),
        "deleting an already-deleted cluster should return an error"
    );
}

#[tokio::test]
async fn test_initialize_nonexistent_cluster() {
    let client = make_cloudhsmv2_client().await;

    let result = client
        .initialize_cluster()
        .cluster_id("cluster-doesnotexist")
        .signed_cert("cert")
        .trust_anchor("anchor")
        .send()
        .await;

    assert!(
        result.is_err(),
        "initialize_cluster on a nonexistent cluster should fail"
    );
}

#[tokio::test]
async fn test_initialize_cluster_response_state() {
    let client = make_cloudhsmv2_client().await;

    let resp = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-12345678")
        .send()
        .await
        .unwrap();
    let cluster_id = resp.cluster().unwrap().cluster_id().unwrap().to_string();

    let init_resp = client
        .initialize_cluster()
        .cluster_id(&cluster_id)
        .signed_cert("cert")
        .trust_anchor("anchor")
        .send()
        .await
        .expect("initialize_cluster should succeed");

    assert_eq!(
        init_resp.state().map(|s| s.as_str()),
        Some("INITIALIZED"),
        "initialized cluster should report INITIALIZED state"
    );
}

// ============================================================================
// Additional tests derived from AWS documentation: AWS CloudHSM V2
// ============================================================================

#[tokio::test]
async fn test_create_cluster_response_fields() {
    let client = make_cloudhsmv2_client().await;

    let resp = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-aabbccdd")
        .send()
        .await
        .expect("create_cluster should succeed");

    let cluster = resp.cluster().expect("should have cluster");

    // cluster_id must be present and non-empty
    assert!(
        !cluster.cluster_id().unwrap_or_default().is_empty(),
        "cluster_id should be present and non-empty"
    );
    // hsm_type must match input
    assert_eq!(cluster.hsm_type(), Some("hsm1.medium"));
    // vpc_id must be present
    assert!(
        !cluster.vpc_id().unwrap_or_default().is_empty(),
        "vpc_id should be present"
    );
    // security_group must be present
    assert!(
        !cluster.security_group().unwrap_or_default().is_empty(),
        "security_group should be present"
    );
    // create_timestamp must be present
    assert!(
        cluster.create_timestamp().is_some(),
        "create_timestamp should be present"
    );
    // backup_policy must be present
    assert!(
        cluster.backup_policy().is_some(),
        "backup_policy should be present"
    );
    // state must be UNINITIALIZED
    assert_eq!(cluster.state().map(|s| s.as_str()), Some("UNINITIALIZED"));
}

#[tokio::test]
async fn test_create_cluster_subnet_mapping() {
    let client = make_cloudhsmv2_client().await;

    let resp = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-11111111")
        .subnet_ids("subnet-22222222")
        .send()
        .await
        .expect("create_cluster should succeed");

    let cluster = resp.cluster().expect("should have cluster");
    let mapping = cluster
        .subnet_mapping()
        .expect("should have subnet_mapping");
    // Two subnets => two AZ mappings
    assert_eq!(
        mapping.len(),
        2,
        "subnet_mapping should contain one entry per subnet"
    );
    // Both subnets must appear as values in the mapping
    let values: Vec<&str> = mapping.values().map(|s: &String| s.as_str()).collect();
    assert!(values.contains(&"subnet-11111111"));
    assert!(values.contains(&"subnet-22222222"));
}

#[tokio::test]
async fn test_create_cluster_with_source_backup_id() {
    let client = make_cloudhsmv2_client().await;

    let resp = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-12345678")
        .source_backup_id("backup-abcdefghijk")
        .send()
        .await
        .expect("create_cluster with source_backup_id should succeed");

    let cluster = resp.cluster().expect("should have cluster");
    assert_eq!(
        cluster.source_backup_id(),
        Some("backup-abcdefghijk"),
        "source_backup_id should be reflected in the response"
    );
}

#[tokio::test]
async fn test_describe_clusters_filter_by_vpc_id() {
    let client = make_cloudhsmv2_client().await;

    // Create a cluster and capture its VPC ID from the response
    let c1 = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-11111111")
        .send()
        .await
        .unwrap();
    let vpc_id = c1.cluster().unwrap().vpc_id().unwrap().to_string();

    // Create a second cluster — it gets its own VPC ID
    client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-22222222")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_clusters()
        .filters("vpcIds", vec![vpc_id.clone()])
        .send()
        .await
        .expect("describe_clusters with vpcIds filter should succeed");

    assert_eq!(
        resp.clusters().len(),
        1,
        "only one cluster should match the vpcIds filter"
    );
    assert_eq!(resp.clusters()[0].vpc_id().unwrap(), vpc_id.as_str());
}

#[tokio::test]
async fn test_delete_cluster_response_fields() {
    let client = make_cloudhsmv2_client().await;

    let create_resp = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-12345678")
        .send()
        .await
        .unwrap();
    let cluster_id = create_resp
        .cluster()
        .unwrap()
        .cluster_id()
        .unwrap()
        .to_string();

    let del_resp = client
        .delete_cluster()
        .cluster_id(&cluster_id)
        .send()
        .await
        .expect("delete_cluster should succeed");

    let deleted = del_resp.cluster().expect("response should contain cluster");
    assert_eq!(
        deleted.cluster_id(),
        Some(cluster_id.as_str()),
        "deleted cluster should carry back its cluster_id"
    );
    assert_eq!(
        deleted.state(),
        Some(&aws_sdk_cloudhsmv2::types::ClusterState::Deleted),
        "deleted cluster state should be DELETED"
    );
}

#[tokio::test]
async fn test_delete_cluster_still_in_describe() {
    // After deletion the cluster is still returned by DescribeClusters with state DELETED.
    let client = make_cloudhsmv2_client().await;

    let create_resp = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-12345678")
        .send()
        .await
        .unwrap();
    let cluster_id = create_resp
        .cluster()
        .unwrap()
        .cluster_id()
        .unwrap()
        .to_string();

    client
        .delete_cluster()
        .cluster_id(&cluster_id)
        .send()
        .await
        .unwrap();

    let desc_resp = client
        .describe_clusters()
        .filters("clusterIds", vec![cluster_id.clone()])
        .send()
        .await
        .expect("describe_clusters after delete should succeed");

    assert_eq!(
        desc_resp.clusters().len(),
        1,
        "deleted cluster should still appear in describe_clusters"
    );
    assert_eq!(
        desc_resp.clusters()[0].state(),
        Some(&aws_sdk_cloudhsmv2::types::ClusterState::Deleted)
    );
}

#[tokio::test]
async fn test_initialize_cluster_state_message() {
    let client = make_cloudhsmv2_client().await;

    let create_resp = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-12345678")
        .send()
        .await
        .unwrap();
    let cluster_id = create_resp
        .cluster()
        .unwrap()
        .cluster_id()
        .unwrap()
        .to_string();

    let init_resp = client
        .initialize_cluster()
        .cluster_id(&cluster_id)
        .signed_cert("cert")
        .trust_anchor("anchor")
        .send()
        .await
        .expect("initialize_cluster should succeed");

    assert!(
        init_resp.state_message().is_some(),
        "initialize_cluster response should include a state_message"
    );
    assert!(
        !init_resp.state_message().unwrap_or_default().is_empty(),
        "state_message should not be empty"
    );
}

#[tokio::test]
async fn test_cluster_full_lifecycle() {
    let client = make_cloudhsmv2_client().await;

    // 1. Create
    let create_resp = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-12345678")
        .send()
        .await
        .expect("create_cluster should succeed");
    let cluster_id = create_resp
        .cluster()
        .unwrap()
        .cluster_id()
        .unwrap()
        .to_string();

    // 2. Describe — cluster is UNINITIALIZED
    let desc_resp = client
        .describe_clusters()
        .filters("clusterIds", vec![cluster_id.clone()])
        .send()
        .await
        .unwrap();
    assert_eq!(desc_resp.clusters().len(), 1);
    assert_eq!(
        desc_resp.clusters()[0].state().map(|s| s.as_str()),
        Some("UNINITIALIZED")
    );

    // 3. Initialize — state becomes INITIALIZED
    let init_resp = client
        .initialize_cluster()
        .cluster_id(&cluster_id)
        .signed_cert("cert")
        .trust_anchor("anchor")
        .send()
        .await
        .expect("initialize_cluster should succeed");
    assert_eq!(init_resp.state().map(|s| s.as_str()), Some("INITIALIZED"));

    // Verify via describe
    let desc2 = client
        .describe_clusters()
        .filters("clusterIds", vec![cluster_id.clone()])
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc2.clusters()[0].state().map(|s| s.as_str()),
        Some("INITIALIZED")
    );

    // 4. Delete — state becomes DELETED
    let del_resp = client
        .delete_cluster()
        .cluster_id(&cluster_id)
        .send()
        .await
        .expect("delete_cluster should succeed");
    assert_eq!(
        del_resp.cluster().unwrap().state(),
        Some(&aws_sdk_cloudhsmv2::types::ClusterState::Deleted)
    );

    // Verify via describe
    let desc3 = client
        .describe_clusters()
        .filters("clusterIds", vec![cluster_id.clone()])
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc3.clusters()[0].state().map(|s| s.as_str()),
        Some("DELETED")
    );
}

// ============================================================================
// Helper: create a backup directly via state view for test purposes
// ============================================================================

async fn make_cloudhsmv2_client_with_backup() -> (aws_sdk_cloudhsmv2::Client, String) {
    use winterbaume_cloudhsmv2::views::{BackupView, CloudHsmV2StateView};
    use winterbaume_core::StatefulService;

    let svc = CloudHsmV2Service::new();

    let backup_id = "backup-testbackup0001234".to_string();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64();

    let view = CloudHsmV2StateView {
        backups: std::collections::HashMap::from([(
            backup_id.clone(),
            BackupView {
                backup_id: backup_id.clone(),
                backup_arn: format!("arn:aws:cloudhsm:us-east-1:123456789012:backup/{backup_id}"),
                backup_state: "READY".to_string(),
                cluster_id: Some("cluster-test12345678901".to_string()),
                create_timestamp: now,
                hsm_type: Some("hsm1.medium".to_string()),
                ..Default::default()
            },
        )]),
        ..Default::default()
    };

    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    let mock = winterbaume_core::MockAws::builder()
        .with_service(svc)
        .build();

    let config = aws_config::defaults(aws_sdk_cloudhsmv2::config::BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudhsmv2::config::Region::new("us-east-1"))
        .load()
        .await;

    (aws_sdk_cloudhsmv2::Client::new(&config), backup_id)
}

// ============================================================================
// Tests for CreateHsm / DeleteHsm
// ============================================================================

#[tokio::test]
async fn test_create_hsm() {
    let client = make_cloudhsmv2_client().await;

    let cluster_resp = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-12345678")
        .send()
        .await
        .unwrap();
    let cluster_id = cluster_resp
        .cluster()
        .unwrap()
        .cluster_id()
        .unwrap()
        .to_string();

    let resp = client
        .create_hsm()
        .cluster_id(&cluster_id)
        .availability_zone("us-east-1a")
        .send()
        .await
        .expect("create_hsm should succeed");

    let hsm = resp.hsm().expect("should have hsm");
    assert!(!hsm.hsm_id().is_empty(), "hsm_id should be non-empty");
    assert_eq!(hsm.cluster_id(), Some(cluster_id.as_str()));
    assert_eq!(hsm.availability_zone(), Some("us-east-1a"));
}

#[tokio::test]
async fn test_create_hsm_nonexistent_cluster() {
    let client = make_cloudhsmv2_client().await;

    let result = client
        .create_hsm()
        .cluster_id("cluster-doesnotexist")
        .availability_zone("us-east-1a")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_hsm() {
    let client = make_cloudhsmv2_client().await;

    let cluster_resp = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-12345678")
        .send()
        .await
        .unwrap();
    let cluster_id = cluster_resp
        .cluster()
        .unwrap()
        .cluster_id()
        .unwrap()
        .to_string();

    let hsm_resp = client
        .create_hsm()
        .cluster_id(&cluster_id)
        .availability_zone("us-east-1a")
        .send()
        .await
        .unwrap();
    let hsm_id = hsm_resp.hsm().unwrap().hsm_id().to_string();

    let del_resp = client
        .delete_hsm()
        .cluster_id(&cluster_id)
        .hsm_id(&hsm_id)
        .send()
        .await
        .expect("delete_hsm should succeed");

    assert_eq!(del_resp.hsm_id(), Some(hsm_id.as_str()));
}

#[tokio::test]
async fn test_delete_hsm_nonexistent() {
    let client = make_cloudhsmv2_client().await;

    let cluster_resp = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-12345678")
        .send()
        .await
        .unwrap();
    let cluster_id = cluster_resp
        .cluster()
        .unwrap()
        .cluster_id()
        .unwrap()
        .to_string();

    let result = client
        .delete_hsm()
        .cluster_id(&cluster_id)
        .hsm_id("hsm-doesnotexist")
        .send()
        .await;

    assert!(result.is_err());
}

// ============================================================================
// Tests for DescribeBackups / DeleteBackup / RestoreBackup
// ============================================================================

#[tokio::test]
async fn test_describe_backups_empty() {
    let client = make_cloudhsmv2_client().await;

    let resp = client
        .describe_backups()
        .send()
        .await
        .expect("describe_backups on empty store should succeed");

    assert_eq!(resp.backups().len(), 0);
}

#[tokio::test]
async fn test_describe_backups_with_data() {
    let (client, backup_id) = make_cloudhsmv2_client_with_backup().await;

    let resp = client
        .describe_backups()
        .send()
        .await
        .expect("describe_backups should succeed");

    assert_eq!(resp.backups().len(), 1);
    assert_eq!(resp.backups()[0].backup_id(), backup_id.as_str());
}

#[tokio::test]
async fn test_delete_backup() {
    let (client, backup_id) = make_cloudhsmv2_client_with_backup().await;

    let resp = client
        .delete_backup()
        .backup_id(&backup_id)
        .send()
        .await
        .expect("delete_backup should succeed");

    let backup = resp.backup().expect("should have backup");
    assert_eq!(backup.backup_id(), backup_id.as_str());
    assert_eq!(backup.backup_state().map(|s| s.as_str()), Some("DELETED"));
}

#[tokio::test]
async fn test_delete_backup_nonexistent() {
    let client = make_cloudhsmv2_client().await;

    let result = client
        .delete_backup()
        .backup_id("backup-doesnotexist")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_restore_backup() {
    let (client, backup_id) = make_cloudhsmv2_client_with_backup().await;

    // Delete the backup first
    client
        .delete_backup()
        .backup_id(&backup_id)
        .send()
        .await
        .unwrap();

    // Now restore it
    let resp = client
        .restore_backup()
        .backup_id(&backup_id)
        .send()
        .await
        .expect("restore_backup should succeed");

    let backup = resp.backup().expect("should have backup");
    assert_eq!(backup.backup_id(), backup_id.as_str());
    assert_eq!(backup.backup_state().map(|s| s.as_str()), Some("READY"));
}

#[tokio::test]
async fn test_restore_backup_not_deleted() {
    let (client, backup_id) = make_cloudhsmv2_client_with_backup().await;

    // Restore a READY backup (not deleted) should fail
    let result = client.restore_backup().backup_id(&backup_id).send().await;

    assert!(result.is_err());
}

// ============================================================================
// Tests for ModifyBackupAttributes
// ============================================================================

#[tokio::test]
async fn test_modify_backup_attributes() {
    let (client, backup_id) = make_cloudhsmv2_client_with_backup().await;

    let resp = client
        .modify_backup_attributes()
        .backup_id(&backup_id)
        .never_expires(true)
        .send()
        .await
        .expect("modify_backup_attributes should succeed");

    let backup = resp.backup().expect("should have backup");
    assert_eq!(backup.never_expires(), Some(true));
}

#[tokio::test]
async fn test_modify_backup_attributes_nonexistent() {
    let client = make_cloudhsmv2_client().await;

    let result = client
        .modify_backup_attributes()
        .backup_id("backup-doesnotexist")
        .never_expires(true)
        .send()
        .await;

    assert!(result.is_err());
}

// ============================================================================
// Tests for ModifyCluster
// ============================================================================

#[tokio::test]
async fn test_modify_cluster() {
    let client = make_cloudhsmv2_client().await;

    let cluster_resp = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-12345678")
        .send()
        .await
        .unwrap();
    let cluster_id = cluster_resp
        .cluster()
        .unwrap()
        .cluster_id()
        .unwrap()
        .to_string();

    let new_policy = aws_sdk_cloudhsmv2::types::BackupRetentionPolicy::builder()
        .r#type(aws_sdk_cloudhsmv2::types::BackupRetentionType::Days)
        .value("14")
        .build();

    let resp = client
        .modify_cluster()
        .cluster_id(&cluster_id)
        .backup_retention_policy(new_policy)
        .send()
        .await
        .expect("modify_cluster should succeed");

    let cluster = resp.cluster().expect("should have cluster");
    let brp = cluster
        .backup_retention_policy()
        .expect("should have backup_retention_policy");
    assert_eq!(brp.value(), Some("14"));
}

#[tokio::test]
async fn test_modify_cluster_nonexistent() {
    let client = make_cloudhsmv2_client().await;

    let new_policy = aws_sdk_cloudhsmv2::types::BackupRetentionPolicy::builder()
        .r#type(aws_sdk_cloudhsmv2::types::BackupRetentionType::Days)
        .value("7")
        .build();

    let result = client
        .modify_cluster()
        .cluster_id("cluster-doesnotexist")
        .backup_retention_policy(new_policy)
        .send()
        .await;

    assert!(result.is_err());
}

// ============================================================================
// Tests for ListTags / TagResource / UntagResource
// ============================================================================

#[tokio::test]
async fn test_tag_and_list_tags_cluster() {
    let client = make_cloudhsmv2_client().await;

    let cluster_resp = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-12345678")
        .send()
        .await
        .unwrap();
    let cluster_id = cluster_resp
        .cluster()
        .unwrap()
        .cluster_id()
        .unwrap()
        .to_string();

    client
        .tag_resource()
        .resource_id(&cluster_id)
        .tag_list(
            aws_sdk_cloudhsmv2::types::Tag::builder()
                .key("Env")
                .value("Prod")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    let list_resp = client
        .list_tags()
        .resource_id(&cluster_id)
        .send()
        .await
        .expect("list_tags should succeed");

    let tags = list_resp.tag_list();
    assert!(
        tags.iter().any(|t| t.key() == "Env" && t.value() == "Prod"),
        "tag should be present"
    );
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_cloudhsmv2_client().await;

    let cluster_resp = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-12345678")
        .tag_list(
            aws_sdk_cloudhsmv2::types::Tag::builder()
                .key("ToRemove")
                .value("Yes")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let cluster_id = cluster_resp
        .cluster()
        .unwrap()
        .cluster_id()
        .unwrap()
        .to_string();

    client
        .untag_resource()
        .resource_id(&cluster_id)
        .tag_key_list("ToRemove")
        .send()
        .await
        .expect("untag_resource should succeed");

    let list_resp = client
        .list_tags()
        .resource_id(&cluster_id)
        .send()
        .await
        .unwrap();

    assert!(
        !list_resp.tag_list().iter().any(|t| t.key() == "ToRemove"),
        "tag should be removed"
    );
}

#[tokio::test]
async fn test_list_tags_nonexistent_resource() {
    let client = make_cloudhsmv2_client().await;

    let result = client
        .list_tags()
        .resource_id("cluster-doesnotexist")
        .send()
        .await;

    assert!(result.is_err());
}

// ============================================================================
// Tests for GetResourcePolicy / PutResourcePolicy / DeleteResourcePolicy
// ============================================================================

#[tokio::test]
async fn test_put_and_get_resource_policy() {
    let client = make_cloudhsmv2_client().await;
    let arn = "arn:aws:cloudhsm:us-east-1:123456789012:backup/backup-test0001";
    let policy = r#"{"Version":"2012-10-17","Statement":[]}"#;

    client
        .put_resource_policy()
        .resource_arn(arn)
        .policy(policy)
        .send()
        .await
        .expect("put_resource_policy should succeed");

    let resp = client
        .get_resource_policy()
        .resource_arn(arn)
        .send()
        .await
        .expect("get_resource_policy should succeed");

    assert_eq!(resp.policy(), Some(policy));
}

#[tokio::test]
async fn test_delete_resource_policy() {
    let client = make_cloudhsmv2_client().await;
    let arn = "arn:aws:cloudhsm:us-east-1:123456789012:backup/backup-test0002";
    let policy = r#"{"Version":"2012-10-17","Statement":[]}"#;

    client
        .put_resource_policy()
        .resource_arn(arn)
        .policy(policy)
        .send()
        .await
        .unwrap();

    let del_resp = client
        .delete_resource_policy()
        .resource_arn(arn)
        .send()
        .await
        .expect("delete_resource_policy should succeed");

    assert_eq!(del_resp.resource_arn(), Some(arn));

    // After deletion, get should return None policy
    let get_resp = client
        .get_resource_policy()
        .resource_arn(arn)
        .send()
        .await
        .expect("get_resource_policy after delete should succeed");

    assert!(get_resp.policy().is_none());
}

// ============================================================================
// Tests for CopyBackupToRegion
// ============================================================================

#[tokio::test]
async fn test_copy_backup_to_region() {
    let (client, backup_id) = make_cloudhsmv2_client_with_backup().await;

    let resp = client
        .copy_backup_to_region()
        .backup_id(&backup_id)
        .destination_region("eu-west-1")
        .send()
        .await
        .expect("copy_backup_to_region should succeed");

    let dest = resp
        .destination_backup()
        .expect("should have destination_backup");
    assert!(dest.source_backup().is_some());
    assert_eq!(dest.source_backup(), Some(backup_id.as_str()));
}

#[tokio::test]
async fn test_copy_backup_to_region_nonexistent() {
    let client = make_cloudhsmv2_client().await;

    let result = client
        .copy_backup_to_region()
        .backup_id("backup-doesnotexist")
        .destination_region("eu-west-1")
        .send()
        .await;

    assert!(result.is_err());
}

// ============================================================================
// Tests for HSM lifecycle in cluster
// ============================================================================

#[tokio::test]
async fn test_cluster_hsm_appears_in_describe() {
    let client = make_cloudhsmv2_client().await;

    let cluster_resp = client
        .create_cluster()
        .hsm_type("hsm1.medium")
        .subnet_ids("subnet-12345678")
        .send()
        .await
        .unwrap();
    let cluster_id = cluster_resp
        .cluster()
        .unwrap()
        .cluster_id()
        .unwrap()
        .to_string();

    client
        .create_hsm()
        .cluster_id(&cluster_id)
        .availability_zone("us-east-1a")
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_clusters()
        .filters("clusterIds", vec![cluster_id.clone()])
        .send()
        .await
        .unwrap();

    let cluster = &desc.clusters()[0];
    assert_eq!(
        cluster.hsms().len(),
        1,
        "HSM should appear in describe_clusters"
    );
}

#[tokio::test]
async fn test_describe_backups_filter_by_state() {
    let (client, backup_id) = make_cloudhsmv2_client_with_backup().await;

    // Delete it so state is DELETED
    client
        .delete_backup()
        .backup_id(&backup_id)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_backups()
        .filters("states", vec!["DELETED".to_string()])
        .send()
        .await
        .expect("describe_backups with states filter should succeed");

    assert_eq!(resp.backups().len(), 1);
    assert_eq!(
        resp.backups()[0].backup_state().map(|s| s.as_str()),
        Some("DELETED")
    );
}
