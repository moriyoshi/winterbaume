//! End-to-end scenario tests for winterbaume-forecast.
//!
//! Scenario: Dataset group management workflow
//! A user creates a dataset group for retail demand forecasting, attaches
//! datasets, updates the group membership, and finally deletes the group once
//! the forecasting project is complete.

use aws_sdk_forecast::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_forecast::ForecastService;

async fn make_client() -> aws_sdk_forecast::Client {
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

/// Scenario: Retail demand-forecasting project lifecycle.
///
/// 1. Create a dataset group for the retail domain.
/// 2. Describe the group and verify its initial state.
/// 3. Update the group to attach two dataset ARNs.
/// 4. Describe again and confirm the datasets are recorded.
/// 5. Update the group to replace the dataset list with a single entry.
/// 6. Delete the group.
/// 7. Confirm the group is no longer visible in the list.
#[tokio::test]
async fn test_retail_forecasting_project_lifecycle() {
    let client = make_client().await;

    // Step 1: Create dataset group
    let create_resp = client
        .create_dataset_group()
        .dataset_group_name("retail-forecast-project")
        .domain(aws_sdk_forecast::types::Domain::Retail)
        .send()
        .await
        .expect("create dataset group should succeed");

    let arn = create_resp
        .dataset_group_arn()
        .expect("ARN must be returned")
        .to_string();

    assert!(
        arn.contains("retail-forecast-project"),
        "ARN should contain the group name"
    );

    // Step 2: Describe and verify initial state
    let desc = client
        .describe_dataset_group()
        .dataset_group_arn(&arn)
        .send()
        .await
        .expect("initial describe should succeed");

    assert_eq!(
        desc.dataset_group_name().unwrap(),
        "retail-forecast-project"
    );
    assert_eq!(
        desc.domain().unwrap(),
        &aws_sdk_forecast::types::Domain::Retail
    );
    assert_eq!(desc.status().unwrap(), "ACTIVE");
    assert!(
        desc.dataset_arns().is_empty(),
        "no datasets attached initially"
    );

    // Step 3: Attach two datasets
    client
        .update_dataset_group()
        .dataset_group_arn(&arn)
        .dataset_arns("arn:aws:forecast:us-east-1:123456789012:dataset/sales-history")
        .dataset_arns("arn:aws:forecast:us-east-1:123456789012:dataset/weather-data")
        .send()
        .await
        .expect("update with two datasets should succeed");

    // Step 4: Confirm two datasets are recorded
    let desc2 = client
        .describe_dataset_group()
        .dataset_group_arn(&arn)
        .send()
        .await
        .expect("describe after first update should succeed");

    assert_eq!(
        desc2.dataset_arns().len(),
        2,
        "should have two datasets after first update"
    );

    // Step 5: Replace dataset list with one entry
    client
        .update_dataset_group()
        .dataset_group_arn(&arn)
        .dataset_arns("arn:aws:forecast:us-east-1:123456789012:dataset/sales-history")
        .send()
        .await
        .expect("update to single dataset should succeed");

    let desc3 = client
        .describe_dataset_group()
        .dataset_group_arn(&arn)
        .send()
        .await
        .expect("describe after second update should succeed");

    assert_eq!(
        desc3.dataset_arns().len(),
        1,
        "should have one dataset after replacement"
    );

    // Step 6: Delete the group
    client
        .delete_dataset_group()
        .dataset_group_arn(&arn)
        .send()
        .await
        .expect("delete should succeed");

    // Step 7: Confirm it is gone from the list
    let list_resp = client
        .list_dataset_groups()
        .send()
        .await
        .expect("list should succeed after deletion");

    let still_present = list_resp
        .dataset_groups()
        .iter()
        .any(|g| g.dataset_group_arn() == Some(arn.as_str()));

    assert!(
        !still_present,
        "deleted dataset group must not appear in list"
    );
}

/// Scenario: Multi-project isolation.
///
/// Creates two independent dataset groups (different domains), verifies they
/// coexist in the list, then deletes one and confirms the other remains intact.
#[tokio::test]
async fn test_multi_project_isolation() {
    let client = make_client().await;

    // Create two groups with different domains
    let retail_arn = client
        .create_dataset_group()
        .dataset_group_name("multi-retail")
        .domain(aws_sdk_forecast::types::Domain::Retail)
        .send()
        .await
        .unwrap()
        .dataset_group_arn()
        .unwrap()
        .to_string();

    let metrics_arn = client
        .create_dataset_group()
        .dataset_group_name("multi-metrics")
        .domain(aws_sdk_forecast::types::Domain::Metrics)
        .send()
        .await
        .unwrap()
        .dataset_group_arn()
        .unwrap()
        .to_string();

    // Both must appear in the list
    let list = client.list_dataset_groups().send().await.unwrap();
    let arns: Vec<&str> = list
        .dataset_groups()
        .iter()
        .filter_map(|g| g.dataset_group_arn())
        .collect();

    assert!(arns.contains(&retail_arn.as_str()), "retail group in list");
    assert!(
        arns.contains(&metrics_arn.as_str()),
        "metrics group in list"
    );

    // Delete the retail group
    client
        .delete_dataset_group()
        .dataset_group_arn(&retail_arn)
        .send()
        .await
        .expect("delete retail group should succeed");

    // Metrics group must still be fully accessible
    let desc = client
        .describe_dataset_group()
        .dataset_group_arn(&metrics_arn)
        .send()
        .await
        .expect("metrics group should still be describable after retail group deletion");

    assert_eq!(desc.dataset_group_name().unwrap(), "multi-metrics");
    assert_eq!(
        desc.domain().unwrap(),
        &aws_sdk_forecast::types::Domain::Metrics
    );
}
