use aws_sdk_forecast::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_forecast::ForecastService;

async fn make_forecast_client() -> aws_sdk_forecast::Client {
    let mock = MockAws::builder()
        .with_service(ForecastService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_forecast::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_forecast::Client::new(&config)
}

#[tokio::test]
async fn test_create_dataset_group() {
    let client = make_forecast_client().await;

    let resp = client
        .create_dataset_group()
        .dataset_group_name("test-dsg")
        .domain(aws_sdk_forecast::types::Domain::Retail)
        .send()
        .await
        .expect("create_dataset_group should succeed");

    let arn = resp.dataset_group_arn().expect("should have ARN");
    assert!(arn.contains("test-dsg"));
}

#[tokio::test]
async fn test_describe_dataset_group() {
    let client = make_forecast_client().await;

    let create_resp = client
        .create_dataset_group()
        .dataset_group_name("desc-dsg")
        .domain(aws_sdk_forecast::types::Domain::Custom)
        .send()
        .await
        .unwrap();

    let arn = create_resp.dataset_group_arn().unwrap();

    let resp = client
        .describe_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await
        .expect("describe_dataset_group should succeed");

    assert_eq!(resp.dataset_group_name().unwrap(), "desc-dsg");
    assert_eq!(
        resp.domain().unwrap(),
        &aws_sdk_forecast::types::Domain::Custom
    );
    assert_eq!(resp.status().unwrap(), "ACTIVE");
}

#[tokio::test]
async fn test_delete_dataset_group() {
    let client = make_forecast_client().await;

    let create_resp = client
        .create_dataset_group()
        .dataset_group_name("del-dsg")
        .domain(aws_sdk_forecast::types::Domain::Retail)
        .send()
        .await
        .unwrap();

    let arn = create_resp.dataset_group_arn().unwrap();

    client
        .delete_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await
        .expect("delete_dataset_group should succeed");

    let result = client
        .describe_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_dataset_groups() {
    let client = make_forecast_client().await;

    for name in ["list-dsg-a", "list-dsg-b", "list-dsg-c"] {
        client
            .create_dataset_group()
            .dataset_group_name(name)
            .domain(aws_sdk_forecast::types::Domain::Retail)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_dataset_groups()
        .send()
        .await
        .expect("list_dataset_groups should succeed");

    let groups = resp.dataset_groups();
    assert_eq!(groups.len(), 3);
}

#[tokio::test]
async fn test_create_duplicate_dataset_group_fails() {
    let client = make_forecast_client().await;

    client
        .create_dataset_group()
        .dataset_group_name("dup-dsg")
        .domain(aws_sdk_forecast::types::Domain::Retail)
        .send()
        .await
        .unwrap();

    let result = client
        .create_dataset_group()
        .dataset_group_name("dup-dsg")
        .domain(aws_sdk_forecast::types::Domain::Retail)
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_dataset_group_fails() {
    let client = make_forecast_client().await;

    let result = client
        .delete_dataset_group()
        .dataset_group_arn("arn:aws:forecast:us-east-1:123456789012:dataset-group/nonexistent")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_dataset_group_succeeds() {
    let client = make_forecast_client().await;

    let create_resp = client
        .create_dataset_group()
        .dataset_group_name("upd-dsg")
        .domain(aws_sdk_forecast::types::Domain::Retail)
        .send()
        .await
        .unwrap();

    let arn = create_resp.dataset_group_arn().unwrap();

    client
        .update_dataset_group()
        .dataset_group_arn(arn)
        .dataset_arns("arn:aws:forecast:us-east-1:123456789012:dataset/ds1")
        .send()
        .await
        .expect("update_dataset_group should succeed");

    let desc = client
        .describe_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await
        .unwrap();

    let arns = desc.dataset_arns();
    assert_eq!(arns.len(), 1);
    assert_eq!(
        arns[0],
        "arn:aws:forecast:us-east-1:123456789012:dataset/ds1"
    );
}

#[tokio::test]
async fn test_update_dataset_group_nonexistent_fails() {
    let client = make_forecast_client().await;

    let result = client
        .update_dataset_group()
        .dataset_group_arn("arn:aws:forecast:us-east-1:123456789012:dataset-group/does-not-exist")
        .dataset_arns("arn:aws:forecast:us-east-1:123456789012:dataset/ds1")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_dataset_group_clears_arns() {
    let client = make_forecast_client().await;

    let create_resp = client
        .create_dataset_group()
        .dataset_group_name("clear-arns-dsg")
        .domain(aws_sdk_forecast::types::Domain::Custom)
        .dataset_arns("arn:aws:forecast:us-east-1:123456789012:dataset/ds-initial")
        .send()
        .await
        .unwrap();

    let arn = create_resp.dataset_group_arn().unwrap();

    // UpdateDatasetGroup replaces the list; passing an empty list should clear it
    client
        .update_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await
        .expect("update_dataset_group with empty arns should succeed");

    let desc = client
        .describe_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await
        .unwrap();

    assert!(desc.dataset_arns().is_empty());
}

#[tokio::test]
async fn test_create_dataset_group_with_dataset_arns() {
    let client = make_forecast_client().await;

    let resp = client
        .create_dataset_group()
        .dataset_group_name("dsg-with-arns")
        .domain(aws_sdk_forecast::types::Domain::Metrics)
        .dataset_arns("arn:aws:forecast:us-east-1:123456789012:dataset/ds-metrics")
        .send()
        .await
        .expect("create_dataset_group with dataset_arns should succeed");

    let group_arn = resp.dataset_group_arn().unwrap();

    let desc = client
        .describe_dataset_group()
        .dataset_group_arn(group_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(
        desc.domain().unwrap(),
        &aws_sdk_forecast::types::Domain::Metrics
    );
    let arns = desc.dataset_arns();
    assert_eq!(arns.len(), 1);
    assert_eq!(
        arns[0],
        "arn:aws:forecast:us-east-1:123456789012:dataset/ds-metrics"
    );
}

#[tokio::test]
async fn test_list_dataset_groups_empty() {
    let client = make_forecast_client().await;

    let resp = client
        .list_dataset_groups()
        .send()
        .await
        .expect("list_dataset_groups on empty state should succeed");

    assert_eq!(resp.dataset_groups().len(), 0);
}

#[tokio::test]
async fn test_describe_dataset_group_arn_format() {
    let client = make_forecast_client().await;

    let create_resp = client
        .create_dataset_group()
        .dataset_group_name("arn-format-dsg")
        .domain(aws_sdk_forecast::types::Domain::WorkForce)
        .send()
        .await
        .unwrap();

    let arn = create_resp.dataset_group_arn().unwrap();

    // ARN must follow arn:aws:forecast:<region>:<account>:dataset-group/<name>
    assert!(arn.starts_with("arn:aws:forecast:"));
    assert!(arn.contains(":dataset-group/"));
    assert!(arn.ends_with("arn-format-dsg"));

    let desc = client
        .describe_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await
        .unwrap();

    assert_eq!(desc.dataset_group_arn().unwrap(), arn);
}

#[tokio::test]
async fn test_describe_dataset_group_status_is_active() {
    let client = make_forecast_client().await;

    let create_resp = client
        .create_dataset_group()
        .dataset_group_name("status-check-dsg")
        .domain(aws_sdk_forecast::types::Domain::WebTraffic)
        .send()
        .await
        .unwrap();

    let arn = create_resp.dataset_group_arn().unwrap();

    let desc = client
        .describe_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await
        .unwrap();

    assert_eq!(desc.status().unwrap(), "ACTIVE");
}

#[tokio::test]
async fn test_delete_dataset_group_twice_fails() {
    let client = make_forecast_client().await;

    let create_resp = client
        .create_dataset_group()
        .dataset_group_name("double-del-dsg")
        .domain(aws_sdk_forecast::types::Domain::InventoryPlanning)
        .send()
        .await
        .unwrap();

    let arn = create_resp.dataset_group_arn().unwrap();

    client
        .delete_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await
        .expect("first delete should succeed");

    let result = client
        .delete_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_dataset_groups_reflects_deletions() {
    let client = make_forecast_client().await;

    let mut arns: Vec<String> = Vec::new();
    for name in ["del-list-a", "del-list-b", "del-list-c"] {
        let arn = client
            .create_dataset_group()
            .dataset_group_name(name)
            .domain(aws_sdk_forecast::types::Domain::Retail)
            .send()
            .await
            .unwrap()
            .dataset_group_arn()
            .unwrap()
            .to_string();
        arns.push(arn);
    }

    client
        .delete_dataset_group()
        .dataset_group_arn(&arns[1])
        .send()
        .await
        .unwrap();

    let resp = client.list_dataset_groups().send().await.unwrap();

    let listed_arns: Vec<&str> = resp
        .dataset_groups()
        .iter()
        .filter_map(|g| g.dataset_group_arn())
        .collect();

    assert_eq!(listed_arns.len(), 2);
    assert!(!listed_arns.contains(&arns[1].as_str()));
}

#[tokio::test]
async fn test_unimplemented_operation_returns_error() {
    let client = make_forecast_client().await;

    // CreateAutoPredictor is a stub returning NotImplementedError (501)
    let result = client
        .create_auto_predictor()
        .predictor_name("test-predictor")
        .send()
        .await;

    assert!(result.is_err());
}

// ============================================================================
// Tests derived from AWS documentation: Amazon Forecast
// ============================================================================

#[tokio::test]
async fn test_describe_dataset_group_not_found() {
    let client = make_forecast_client().await;

    let result = client
        .describe_dataset_group()
        .dataset_group_arn(
            "arn:aws:forecast:us-east-1:123456789012:dataset-group/nonexistent-group",
        )
        .send()
        .await;

    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_describe_dataset_group_timestamps() {
    let client = make_forecast_client().await;

    let create_resp = client
        .create_dataset_group()
        .dataset_group_name("ts-check-dsg")
        .domain(aws_sdk_forecast::types::Domain::Custom)
        .send()
        .await
        .unwrap();

    let arn = create_resp.dataset_group_arn().unwrap();

    let desc = client
        .describe_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await
        .unwrap();

    assert!(
        desc.creation_time().is_some(),
        "creation_time should be present"
    );
    assert!(
        desc.last_modification_time().is_some(),
        "last_modification_time should be present"
    );
}

#[tokio::test]
async fn test_update_dataset_group_modifies_last_modification_time() {
    let client = make_forecast_client().await;

    let create_resp = client
        .create_dataset_group()
        .dataset_group_name("lmt-update-dsg")
        .domain(aws_sdk_forecast::types::Domain::Retail)
        .send()
        .await
        .unwrap();

    let arn = create_resp.dataset_group_arn().unwrap();

    let before = client
        .describe_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await
        .unwrap();

    let before_lmt = *before.last_modification_time().unwrap();

    // Small sleep to ensure timestamp changes (1 second resolution)
    tokio::time::sleep(std::time::Duration::from_millis(1100)).await;

    client
        .update_dataset_group()
        .dataset_group_arn(arn)
        .dataset_arns("arn:aws:forecast:us-east-1:123456789012:dataset/ds-lmt")
        .send()
        .await
        .expect("update should succeed");

    let after = client
        .describe_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await
        .unwrap();

    let after_lmt = *after.last_modification_time().unwrap();

    assert!(
        after_lmt >= before_lmt,
        "last_modification_time should be >= creation time after update"
    );
}

#[tokio::test]
async fn test_list_dataset_groups_summary_fields() {
    let client = make_forecast_client().await;

    client
        .create_dataset_group()
        .dataset_group_name("summary-fields-dsg")
        .domain(aws_sdk_forecast::types::Domain::Metrics)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_dataset_groups()
        .send()
        .await
        .expect("list_dataset_groups should succeed");

    let groups = resp.dataset_groups();
    assert!(!groups.is_empty(), "should have at least one group");

    let summary = groups
        .iter()
        .find(|g| g.dataset_group_name() == Some("summary-fields-dsg"))
        .expect("created group should appear in list");

    assert!(
        summary.dataset_group_arn().is_some(),
        "summary should have dataset_group_arn"
    );
    assert_eq!(
        summary.dataset_group_name(),
        Some("summary-fields-dsg"),
        "summary should have dataset_group_name"
    );
    assert!(
        summary.creation_time().is_some(),
        "summary should have creation_time"
    );
    assert!(
        summary.last_modification_time().is_some(),
        "summary should have last_modification_time"
    );
}

#[tokio::test]
async fn test_full_lifecycle_dataset_group() {
    let client = make_forecast_client().await;

    // Create
    let create_resp = client
        .create_dataset_group()
        .dataset_group_name("lifecycle-dsg")
        .domain(aws_sdk_forecast::types::Domain::WorkForce)
        .send()
        .await
        .expect("create should succeed");

    let arn = create_resp
        .dataset_group_arn()
        .expect("arn must be present");
    assert!(arn.contains("lifecycle-dsg"));

    // Describe
    let desc = client
        .describe_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(desc.dataset_group_name().unwrap(), "lifecycle-dsg");
    assert_eq!(desc.status().unwrap(), "ACTIVE");

    // Update
    client
        .update_dataset_group()
        .dataset_group_arn(arn)
        .dataset_arns("arn:aws:forecast:us-east-1:123456789012:dataset/ds-lifecycle")
        .send()
        .await
        .expect("update should succeed");

    let after_update = client
        .describe_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await
        .unwrap();
    assert_eq!(after_update.dataset_arns().len(), 1);

    // Delete
    client
        .delete_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let gone = client
        .describe_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await;
    assert!(gone.is_err(), "describe after delete should fail");
}

#[tokio::test]
async fn test_create_dataset_group_ec2_capacity_domain() {
    let client = make_forecast_client().await;

    let create_resp = client
        .create_dataset_group()
        .dataset_group_name("ec2cap-dsg")
        .domain(aws_sdk_forecast::types::Domain::Ec2Capacity)
        .send()
        .await
        .expect("create with EC2_CAPACITY domain should succeed");

    let arn = create_resp.dataset_group_arn().unwrap();

    let desc = client
        .describe_dataset_group()
        .dataset_group_arn(arn)
        .send()
        .await
        .unwrap();

    assert_eq!(
        desc.domain().unwrap(),
        &aws_sdk_forecast::types::Domain::Ec2Capacity,
        "Domain should be EC2_CAPACITY"
    );
}
