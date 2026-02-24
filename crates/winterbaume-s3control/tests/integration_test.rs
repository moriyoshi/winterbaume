use aws_sdk_s3control::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_s3control::S3ControlService;

async fn make_client() -> (aws_sdk_s3control::Client, MockAws) {
    let svc = S3ControlService::new();
    let mock = MockAws::builder().with_service(svc).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3control::config::Region::new("us-east-1"))
        .load()
        .await;
    let client = aws_sdk_s3control::Client::new(&config);
    (client, mock)
}

// ============================================================================
// CreateAccessPoint / GetAccessPoint / DeleteAccessPoint / ListAccessPoints
// ============================================================================

#[tokio::test]
async fn test_create_and_get_access_point() {
    let (client, _mock) = make_client().await;

    let resp = client
        .create_access_point()
        .account_id("123456789012")
        .name("my-ap")
        .bucket("my-bucket")
        .send()
        .await
        .expect("create_access_point should succeed");

    assert!(resp.access_point_arn().is_some());
    assert!(resp.alias().is_some());

    let get_resp = client
        .get_access_point()
        .account_id("123456789012")
        .name("my-ap")
        .send()
        .await
        .expect("get_access_point should succeed");

    assert_eq!(get_resp.name(), Some("my-ap"));
    assert_eq!(get_resp.bucket(), Some("my-bucket"));
    assert_eq!(
        get_resp.network_origin().map(|n| n.as_str()),
        Some("Internet")
    );
    assert!(get_resp.public_access_block_configuration().is_some());
}

#[tokio::test]
async fn test_create_access_point_vpc() {
    let (client, _mock) = make_client().await;

    let resp = client
        .create_access_point()
        .account_id("123456789012")
        .name("vpc-ap")
        .bucket("my-bucket")
        .vpc_configuration(
            aws_sdk_s3control::types::VpcConfiguration::builder()
                .vpc_id("vpc-12345678")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_access_point with VPC should succeed");

    assert!(resp.access_point_arn().is_some());

    let get_resp = client
        .get_access_point()
        .account_id("123456789012")
        .name("vpc-ap")
        .send()
        .await
        .unwrap();

    assert_eq!(get_resp.network_origin().map(|n| n.as_str()), Some("VPC"));
    assert!(get_resp.vpc_configuration().is_some());
    assert_eq!(
        get_resp.vpc_configuration().unwrap().vpc_id(),
        "vpc-12345678"
    );
}

#[tokio::test]
async fn test_get_access_point_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .get_access_point()
        .account_id("123456789012")
        .name("nonexistent-ap")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_access_point() {
    let (client, _mock) = make_client().await;

    client
        .create_access_point()
        .account_id("123456789012")
        .name("to-delete")
        .bucket("my-bucket")
        .send()
        .await
        .unwrap();

    client
        .delete_access_point()
        .account_id("123456789012")
        .name("to-delete")
        .send()
        .await
        .expect("delete_access_point should succeed");

    // Verify it's gone
    let result = client
        .get_access_point()
        .account_id("123456789012")
        .name("to-delete")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_access_point_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .delete_access_point()
        .account_id("123456789012")
        .name("nonexistent")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_access_points_empty() {
    let (client, _mock) = make_client().await;

    let resp = client
        .list_access_points()
        .account_id("123456789012")
        .send()
        .await
        .expect("list_access_points should succeed");

    assert!(resp.access_point_list().is_empty());
}

#[tokio::test]
async fn test_list_access_points() {
    let (client, _mock) = make_client().await;

    client
        .create_access_point()
        .account_id("123456789012")
        .name("ap-1")
        .bucket("bucket-a")
        .send()
        .await
        .unwrap();

    client
        .create_access_point()
        .account_id("123456789012")
        .name("ap-2")
        .bucket("bucket-b")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_access_points()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.access_point_list().len(), 2);
}

#[tokio::test]
async fn test_list_access_points_filter_by_bucket() {
    let (client, _mock) = make_client().await;

    client
        .create_access_point()
        .account_id("123456789012")
        .name("ap-a")
        .bucket("bucket-a")
        .send()
        .await
        .unwrap();

    client
        .create_access_point()
        .account_id("123456789012")
        .name("ap-b")
        .bucket("bucket-b")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_access_points()
        .account_id("123456789012")
        .bucket("bucket-a")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.access_point_list().len(), 1);
    assert_eq!(resp.access_point_list()[0].name(), "ap-a");
}

// ============================================================================
// GetPublicAccessBlock / PutPublicAccessBlock / DeletePublicAccessBlock
// ============================================================================

#[tokio::test]
async fn test_get_public_access_block_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .get_public_access_block()
        .account_id("123456789012")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_put_and_get_public_access_block() {
    let (client, _mock) = make_client().await;

    client
        .put_public_access_block()
        .account_id("123456789012")
        .public_access_block_configuration(
            aws_sdk_s3control::types::PublicAccessBlockConfiguration::builder()
                .block_public_acls(true)
                .ignore_public_acls(false)
                .block_public_policy(true)
                .restrict_public_buckets(false)
                .build(),
        )
        .send()
        .await
        .expect("put_public_access_block should succeed");

    let resp = client
        .get_public_access_block()
        .account_id("123456789012")
        .send()
        .await
        .expect("get_public_access_block should succeed");

    let config = resp.public_access_block_configuration().unwrap();
    assert_eq!(config.block_public_acls(), Some(true));
    assert_eq!(config.ignore_public_acls(), Some(false));
    assert_eq!(config.block_public_policy(), Some(true));
    assert_eq!(config.restrict_public_buckets(), Some(false));
}

#[tokio::test]
async fn test_delete_public_access_block() {
    let (client, _mock) = make_client().await;

    client
        .put_public_access_block()
        .account_id("123456789012")
        .public_access_block_configuration(
            aws_sdk_s3control::types::PublicAccessBlockConfiguration::builder()
                .block_public_acls(true)
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .delete_public_access_block()
        .account_id("123456789012")
        .send()
        .await
        .expect("delete_public_access_block should succeed");

    // Verify it's gone
    let result = client
        .get_public_access_block()
        .account_id("123456789012")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_public_access_block_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .delete_public_access_block()
        .account_id("123456789012")
        .send()
        .await;

    assert!(result.is_err());
}

// ============================================================================
// Full lifecycle
// ============================================================================

#[tokio::test]
async fn test_full_access_point_lifecycle() {
    let (client, _mock) = make_client().await;

    // No access points initially
    let list = client
        .list_access_points()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert!(list.access_point_list().is_empty());

    // Create
    let create = client
        .create_access_point()
        .account_id("123456789012")
        .name("lifecycle-ap")
        .bucket("lifecycle-bucket")
        .send()
        .await
        .unwrap();
    let arn = create.access_point_arn().unwrap().to_string();
    assert!(arn.contains("lifecycle-ap"));

    // List shows it
    let list = client
        .list_access_points()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert_eq!(list.access_point_list().len(), 1);

    // Get
    let get = client
        .get_access_point()
        .account_id("123456789012")
        .name("lifecycle-ap")
        .send()
        .await
        .unwrap();
    assert_eq!(get.bucket(), Some("lifecycle-bucket"));

    // Delete
    client
        .delete_access_point()
        .account_id("123456789012")
        .name("lifecycle-ap")
        .send()
        .await
        .unwrap();

    // Gone
    let list = client
        .list_access_points()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert!(list.access_point_list().is_empty());
}

// ============================================================================
// State views
// ============================================================================

#[tokio::test]
async fn test_state_snapshot_restore() {
    use winterbaume_core::StatefulService;

    let svc = S3ControlService::new();
    let mock = MockAws::builder()
        .with_service(winterbaume_s3control::S3ControlService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3control::config::Region::new("us-east-1"))
        .load()
        .await;
    let client = aws_sdk_s3control::Client::new(&config);

    // Create an access point
    client
        .create_access_point()
        .account_id("123456789012")
        .name("snap-ap")
        .bucket("snap-bucket")
        .send()
        .await
        .unwrap();

    // Take a snapshot from the service directly
    let snap = svc.snapshot("123456789012", "us-east-1").await;
    // Snapshot should be empty since it's a separate service instance
    assert!(snap.access_points.is_empty());

    // Restore into svc
    let view = winterbaume_s3control::S3ControlStateView {
        access_points: {
            let mut m = std::collections::HashMap::new();
            m.insert(
                "restored-ap".to_string(),
                winterbaume_s3control::views::AccessPointView {
                    name: "restored-ap".to_string(),
                    bucket: "restored-bucket".to_string(),
                    account_id: "123456789012".to_string(),
                    region: "us-east-1".to_string(),
                    alias: "restored-alias".to_string(),
                    arn: "arn:aws:s3:us-east-1:123456789012:accesspoint/restored-ap".to_string(),
                    network_origin: "Internet".to_string(),
                    vpc_id: None,
                    block_public_acls: true,
                    ignore_public_acls: true,
                    block_public_policy: true,
                    restrict_public_buckets: true,
                    creation_date: "2024-01-01T00:00:00Z".to_string(),
                    policy: None,
                },
            );
            m
        },
        public_access_block: None,
        ..Default::default()
    };
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    let snap2 = svc.snapshot("123456789012", "us-east-1").await;
    assert!(snap2.access_points.contains_key("restored-ap"));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;

    let svc = S3ControlService::new();
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
    assert_eq!(got[0].0, "123456789012");
    assert_eq!(got[0].1, "us-east-1");
}

// ============================================================================
// Tests derived from AWS documentation: S3 Control
// ============================================================================

// ============================================================================
// Access Point - duplicate creation
// ============================================================================

#[tokio::test]
async fn test_create_access_point_duplicate() {
    let (client, _mock) = make_client().await;

    client
        .create_access_point()
        .account_id("123456789012")
        .name("dup-ap")
        .bucket("my-bucket")
        .send()
        .await
        .unwrap();

    let err = client
        .create_access_point()
        .account_id("123456789012")
        .name("dup-ap")
        .bucket("my-bucket")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("AccessPointAlreadyOwnedByYou"),
        "Expected duplicate error, got: {err_str}"
    );
}

// ============================================================================
// Access Point Policy lifecycle
// ============================================================================

#[tokio::test]
async fn test_access_point_policy_lifecycle() {
    let (client, _mock) = make_client().await;

    // Create an access point first
    client
        .create_access_point()
        .account_id("123456789012")
        .name("policy-ap")
        .bucket("policy-bucket")
        .send()
        .await
        .unwrap();

    // Get policy - should be empty initially
    let get_resp = client
        .get_access_point_policy()
        .account_id("123456789012")
        .name("policy-ap")
        .send()
        .await
        .unwrap();
    assert!(get_resp.policy().is_none() || get_resp.policy() == Some(""));

    // Put a policy
    let policy_doc = r#"{"Version":"2012-10-17","Statement":[]}"#;
    client
        .put_access_point_policy()
        .account_id("123456789012")
        .name("policy-ap")
        .policy(policy_doc)
        .send()
        .await
        .expect("put_access_point_policy should succeed");

    // Get policy - should return the policy
    let get_resp = client
        .get_access_point_policy()
        .account_id("123456789012")
        .name("policy-ap")
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp.policy(), Some(policy_doc));

    // Delete policy
    client
        .delete_access_point_policy()
        .account_id("123456789012")
        .name("policy-ap")
        .send()
        .await
        .expect("delete_access_point_policy should succeed");

    // Get policy - should be empty again
    let get_resp = client
        .get_access_point_policy()
        .account_id("123456789012")
        .name("policy-ap")
        .send()
        .await
        .unwrap();
    assert!(get_resp.policy().is_none() || get_resp.policy() == Some(""));
}

// ============================================================================
// Access Point Policy Status
// ============================================================================

#[tokio::test]
async fn test_access_point_policy_status() {
    let (client, _mock) = make_client().await;

    client
        .create_access_point()
        .account_id("123456789012")
        .name("status-ap")
        .bucket("status-bucket")
        .send()
        .await
        .unwrap();

    // No policy yet -> not public
    let resp = client
        .get_access_point_policy_status()
        .account_id("123456789012")
        .name("status-ap")
        .send()
        .await
        .unwrap();
    let status = resp.policy_status().unwrap();
    assert!(!status.is_public());

    // Put a policy
    client
        .put_access_point_policy()
        .account_id("123456789012")
        .name("status-ap")
        .policy(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .unwrap();

    // Now has policy -> is_public = true (mock implementation)
    let resp = client
        .get_access_point_policy_status()
        .account_id("123456789012")
        .name("status-ap")
        .send()
        .await
        .unwrap();
    let status = resp.policy_status().unwrap();
    assert!(status.is_public());
}

// ============================================================================
// Object Lambda Access Points
// ============================================================================

#[tokio::test]
async fn test_object_lambda_access_point_lifecycle() {
    let (client, _mock) = make_client().await;

    // List - empty
    let list = client
        .list_access_points_for_object_lambda()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert!(list.object_lambda_access_point_list().is_empty());

    // Create
    let config = aws_sdk_s3control::types::ObjectLambdaConfiguration::builder()
        .supporting_access_point("arn:aws:s3:us-east-1:123456789012:accesspoint/supporting-ap")
        .transformation_configurations(
            aws_sdk_s3control::types::ObjectLambdaTransformationConfiguration::builder()
                .actions(aws_sdk_s3control::types::ObjectLambdaTransformationConfigurationAction::GetObject)
                .content_transformation(
                    aws_sdk_s3control::types::ObjectLambdaContentTransformation::AwsLambda(
                        aws_sdk_s3control::types::AwsLambdaTransformation::builder()
                            .function_arn("arn:aws:lambda:us-east-1:123456789012:function:my-func")
                            .build()
                            .unwrap(),
                    ),
                )
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let create_resp = client
        .create_access_point_for_object_lambda()
        .account_id("123456789012")
        .name("ol-ap")
        .configuration(config)
        .send()
        .await
        .expect("create OLAP should succeed");
    assert!(create_resp.object_lambda_access_point_arn().is_some());

    // Get
    let get_resp = client
        .get_access_point_for_object_lambda()
        .account_id("123456789012")
        .name("ol-ap")
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp.name(), Some("ol-ap"));
    assert!(get_resp.creation_date().is_some());

    // List - one item
    let list = client
        .list_access_points_for_object_lambda()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert_eq!(list.object_lambda_access_point_list().len(), 1);

    // Delete
    client
        .delete_access_point_for_object_lambda()
        .account_id("123456789012")
        .name("ol-ap")
        .send()
        .await
        .unwrap();

    // Verify gone
    let result = client
        .get_access_point_for_object_lambda()
        .account_id("123456789012")
        .name("ol-ap")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_object_lambda_access_point_duplicate() {
    let (client, _mock) = make_client().await;

    let config = aws_sdk_s3control::types::ObjectLambdaConfiguration::builder()
        .supporting_access_point("arn:aws:s3:us-east-1:123456789012:accesspoint/supporting-ap")
        .transformation_configurations(
            aws_sdk_s3control::types::ObjectLambdaTransformationConfiguration::builder()
                .actions(aws_sdk_s3control::types::ObjectLambdaTransformationConfigurationAction::GetObject)
                .content_transformation(
                    aws_sdk_s3control::types::ObjectLambdaContentTransformation::AwsLambda(
                        aws_sdk_s3control::types::AwsLambdaTransformation::builder()
                            .function_arn("arn:aws:lambda:us-east-1:123456789012:function:my-func")
                            .build()
                            .unwrap(),
                    ),
                )
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    client
        .create_access_point_for_object_lambda()
        .account_id("123456789012")
        .name("dup-ol-ap")
        .configuration(config.clone())
        .send()
        .await
        .unwrap();

    let err = client
        .create_access_point_for_object_lambda()
        .account_id("123456789012")
        .name("dup-ol-ap")
        .configuration(config)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ObjectLambdaAccessPointAlreadyExists")
            || err_str.contains("already exists"),
        "Expected duplicate error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_object_lambda_access_point_policy_lifecycle() {
    let (client, _mock) = make_client().await;

    let config = aws_sdk_s3control::types::ObjectLambdaConfiguration::builder()
        .supporting_access_point("arn:aws:s3:us-east-1:123456789012:accesspoint/supporting-ap")
        .transformation_configurations(
            aws_sdk_s3control::types::ObjectLambdaTransformationConfiguration::builder()
                .actions(aws_sdk_s3control::types::ObjectLambdaTransformationConfigurationAction::GetObject)
                .content_transformation(
                    aws_sdk_s3control::types::ObjectLambdaContentTransformation::AwsLambda(
                        aws_sdk_s3control::types::AwsLambdaTransformation::builder()
                            .function_arn("arn:aws:lambda:us-east-1:123456789012:function:my-func")
                            .build()
                            .unwrap(),
                    ),
                )
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    client
        .create_access_point_for_object_lambda()
        .account_id("123456789012")
        .name("ol-policy-ap")
        .configuration(config)
        .send()
        .await
        .unwrap();

    // Get policy - initially empty
    let get_resp = client
        .get_access_point_policy_for_object_lambda()
        .account_id("123456789012")
        .name("ol-policy-ap")
        .send()
        .await
        .unwrap();
    assert!(get_resp.policy().is_none() || get_resp.policy() == Some(""));

    // Put policy
    let policy = r#"{"Version":"2012-10-17","Statement":[]}"#;
    client
        .put_access_point_policy_for_object_lambda()
        .account_id("123456789012")
        .name("ol-policy-ap")
        .policy(policy)
        .send()
        .await
        .unwrap();

    // Get policy
    let get_resp = client
        .get_access_point_policy_for_object_lambda()
        .account_id("123456789012")
        .name("ol-policy-ap")
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp.policy(), Some(policy));

    // Delete policy
    client
        .delete_access_point_policy_for_object_lambda()
        .account_id("123456789012")
        .name("ol-policy-ap")
        .send()
        .await
        .unwrap();

    // Verify policy removed
    let get_resp = client
        .get_access_point_policy_for_object_lambda()
        .account_id("123456789012")
        .name("ol-policy-ap")
        .send()
        .await
        .unwrap();
    assert!(get_resp.policy().is_none() || get_resp.policy() == Some(""));
}

#[tokio::test]
async fn test_object_lambda_access_point_policy_status() {
    let (client, _mock) = make_client().await;

    let config = aws_sdk_s3control::types::ObjectLambdaConfiguration::builder()
        .supporting_access_point("arn:aws:s3:us-east-1:123456789012:accesspoint/supporting-ap")
        .transformation_configurations(
            aws_sdk_s3control::types::ObjectLambdaTransformationConfiguration::builder()
                .actions(aws_sdk_s3control::types::ObjectLambdaTransformationConfigurationAction::GetObject)
                .content_transformation(
                    aws_sdk_s3control::types::ObjectLambdaContentTransformation::AwsLambda(
                        aws_sdk_s3control::types::AwsLambdaTransformation::builder()
                            .function_arn("arn:aws:lambda:us-east-1:123456789012:function:my-func")
                            .build()
                            .unwrap(),
                    ),
                )
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    client
        .create_access_point_for_object_lambda()
        .account_id("123456789012")
        .name("ol-status-ap")
        .configuration(config)
        .send()
        .await
        .unwrap();

    // No policy -> not public
    let resp = client
        .get_access_point_policy_status_for_object_lambda()
        .account_id("123456789012")
        .name("ol-status-ap")
        .send()
        .await
        .unwrap();
    assert!(!resp.policy_status().unwrap().is_public());

    // Add a policy
    client
        .put_access_point_policy_for_object_lambda()
        .account_id("123456789012")
        .name("ol-status-ap")
        .policy(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .unwrap();

    // Has policy -> is_public = true
    let resp = client
        .get_access_point_policy_status_for_object_lambda()
        .account_id("123456789012")
        .name("ol-status-ap")
        .send()
        .await
        .unwrap();
    assert!(resp.policy_status().unwrap().is_public());
}

// ============================================================================
// Access Grants Instance
// ============================================================================

#[tokio::test]
async fn test_access_grants_instance_lifecycle() {
    let (client, _mock) = make_client().await;

    // List - initially empty
    let list = client
        .list_access_grants_instances()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert!(list.access_grants_instances_list().is_empty());

    // Create
    let create = client
        .create_access_grants_instance()
        .account_id("123456789012")
        .send()
        .await
        .expect("create AGI should succeed");
    assert!(create.access_grants_instance_id().is_some());
    assert!(create.access_grants_instance_arn().is_some());
    assert!(create.created_at().is_some());

    // Get
    let get = client
        .get_access_grants_instance()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert!(get.access_grants_instance_id().is_some());
    assert!(get.access_grants_instance_arn().is_some());

    // List - one item
    let list = client
        .list_access_grants_instances()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert_eq!(list.access_grants_instances_list().len(), 1);

    // Delete
    client
        .delete_access_grants_instance()
        .account_id("123456789012")
        .send()
        .await
        .expect("delete AGI should succeed");

    // Verify gone
    let result = client
        .get_access_grants_instance()
        .account_id("123456789012")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_access_grants_instance_duplicate() {
    let (client, _mock) = make_client().await;

    client
        .create_access_grants_instance()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();

    let err = client
        .create_access_grants_instance()
        .account_id("123456789012")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("AccessGrantsInstanceAlreadyExists") || err_str.contains("already exists"),
        "Expected duplicate error, got: {err_str}"
    );
}

// ============================================================================
// Access Grants Instance Resource Policy
// ============================================================================

#[tokio::test]
async fn test_access_grants_resource_policy_lifecycle() {
    let (client, _mock) = make_client().await;

    // Create instance first
    client
        .create_access_grants_instance()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();

    // Put resource policy
    let policy = r#"{"Version":"2012-10-17","Statement":[]}"#;
    let put_resp = client
        .put_access_grants_instance_resource_policy()
        .account_id("123456789012")
        .policy(policy)
        .send()
        .await
        .expect("put resource policy should succeed");
    assert_eq!(put_resp.policy(), Some(policy));
    assert!(put_resp.created_at().is_some());

    // Get resource policy
    let get_resp = client
        .get_access_grants_instance_resource_policy()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp.policy(), Some(policy));

    // Delete resource policy
    client
        .delete_access_grants_instance_resource_policy()
        .account_id("123456789012")
        .send()
        .await
        .expect("delete resource policy should succeed");

    // Get again - policy should be gone
    let get_resp = client
        .get_access_grants_instance_resource_policy()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert!(get_resp.policy().is_none() || get_resp.policy() == Some(""));
}

// ============================================================================
// Access Grants Locations
// ============================================================================

#[tokio::test]
async fn test_access_grants_location_lifecycle() {
    let (client, _mock) = make_client().await;

    // List - empty
    let list = client
        .list_access_grants_locations()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert!(list.access_grants_locations_list().is_empty());

    // Create
    let create = client
        .create_access_grants_location()
        .account_id("123456789012")
        .location_scope("s3://my-bucket/*")
        .iam_role_arn("arn:aws:iam::123456789012:role/my-role")
        .send()
        .await
        .expect("create location should succeed");
    let loc_id = create.access_grants_location_id().unwrap().to_string();
    assert!(!loc_id.is_empty());
    assert!(create.access_grants_location_arn().is_some());
    assert_eq!(create.location_scope(), Some("s3://my-bucket/*"));

    // Get
    let get = client
        .get_access_grants_location()
        .account_id("123456789012")
        .access_grants_location_id(&loc_id)
        .send()
        .await
        .unwrap();
    assert_eq!(get.access_grants_location_id(), Some(loc_id.as_str()));
    assert_eq!(get.location_scope(), Some("s3://my-bucket/*"));

    // Update
    let update = client
        .update_access_grants_location()
        .account_id("123456789012")
        .access_grants_location_id(&loc_id)
        .iam_role_arn("arn:aws:iam::123456789012:role/updated-role")
        .send()
        .await
        .unwrap();
    assert_eq!(
        update.iam_role_arn(),
        Some("arn:aws:iam::123456789012:role/updated-role")
    );

    // List - one item
    let list = client
        .list_access_grants_locations()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert_eq!(list.access_grants_locations_list().len(), 1);

    // Delete
    client
        .delete_access_grants_location()
        .account_id("123456789012")
        .access_grants_location_id(&loc_id)
        .send()
        .await
        .unwrap();

    // Verify gone
    let result = client
        .get_access_grants_location()
        .account_id("123456789012")
        .access_grants_location_id(&loc_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_access_grants_location_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .get_access_grants_location()
        .account_id("123456789012")
        .access_grants_location_id("nonexistent-loc-id")
        .send()
        .await;
    assert!(result.is_err());

    let result = client
        .delete_access_grants_location()
        .account_id("123456789012")
        .access_grants_location_id("nonexistent-loc-id")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Access Grants
// ============================================================================

#[tokio::test]
async fn test_access_grant_lifecycle() {
    let (client, _mock) = make_client().await;

    // List - empty
    let list = client
        .list_access_grants()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert!(list.access_grants_list().is_empty());

    // Create a location first (required for grants)
    let loc = client
        .create_access_grants_location()
        .account_id("123456789012")
        .location_scope("s3://grant-bucket/*")
        .iam_role_arn("arn:aws:iam::123456789012:role/grant-role")
        .send()
        .await
        .unwrap();
    let loc_id = loc.access_grants_location_id().unwrap().to_string();

    // Create grant
    let grantee = aws_sdk_s3control::types::Grantee::builder()
        .grantee_type(aws_sdk_s3control::types::GranteeType::DirectoryUser)
        .grantee_identifier("user@example.com")
        .build();

    let create = client
        .create_access_grant()
        .account_id("123456789012")
        .access_grants_location_id(&loc_id)
        .grantee(grantee)
        .permission(aws_sdk_s3control::types::Permission::Read)
        .send()
        .await
        .expect("create grant should succeed");

    let grant_id = create.access_grant_id().unwrap().to_string();
    assert!(!grant_id.is_empty());
    assert!(create.access_grant_arn().is_some());
    assert!(create.created_at().is_some());

    // Get grant
    let get = client
        .get_access_grant()
        .account_id("123456789012")
        .access_grant_id(&grant_id)
        .send()
        .await
        .unwrap();
    assert_eq!(get.access_grant_id(), Some(grant_id.as_str()));
    assert!(get.grantee().is_some());

    // List - one item
    let list = client
        .list_access_grants()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert_eq!(list.access_grants_list().len(), 1);

    // Delete grant
    client
        .delete_access_grant()
        .account_id("123456789012")
        .access_grant_id(&grant_id)
        .send()
        .await
        .unwrap();

    // Verify gone
    let result = client
        .get_access_grant()
        .account_id("123456789012")
        .access_grant_id(&grant_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_access_grant_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .get_access_grant()
        .account_id("123456789012")
        .access_grant_id("nonexistent-grant-id")
        .send()
        .await;
    assert!(result.is_err());

    let result = client
        .delete_access_grant()
        .account_id("123456789012")
        .access_grant_id("nonexistent-grant-id")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Multi-Region Access Points
// ============================================================================

#[tokio::test]
async fn test_multi_region_access_point_lifecycle() {
    let (client, _mock) = make_client().await;

    // List - empty
    let list = client
        .list_multi_region_access_points()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert!(list.access_points().is_empty());

    // Create
    let region1 = aws_sdk_s3control::types::Region::builder()
        .bucket("bucket-us-east-1")
        .build()
        .unwrap();

    let details = aws_sdk_s3control::types::CreateMultiRegionAccessPointInput::builder()
        .name("test-mrap")
        .regions(region1)
        .build()
        .unwrap();

    let create = client
        .create_multi_region_access_point()
        .account_id("123456789012")
        .details(details)
        .send()
        .await
        .expect("create MRAP should succeed");
    assert!(create.request_token_arn().is_some());

    // Get
    let get = client
        .get_multi_region_access_point()
        .account_id("123456789012")
        .name("test-mrap")
        .send()
        .await
        .unwrap();
    let ap = get.access_point().unwrap();
    assert_eq!(ap.name(), Some("test-mrap"));
    assert_eq!(ap.status().map(|s| s.as_str()), Some("READY"));
    assert!(ap.alias().is_some());
    assert!(!ap.regions().is_empty());

    // List - one item
    let list = client
        .list_multi_region_access_points()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert_eq!(list.access_points().len(), 1);

    // Delete
    let delete_details = aws_sdk_s3control::types::DeleteMultiRegionAccessPointInput::builder()
        .name("test-mrap")
        .build()
        .unwrap();

    let delete = client
        .delete_multi_region_access_point()
        .account_id("123456789012")
        .details(delete_details)
        .send()
        .await
        .unwrap();
    assert!(delete.request_token_arn().is_some());

    // Verify gone
    let result = client
        .get_multi_region_access_point()
        .account_id("123456789012")
        .name("test-mrap")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_multi_region_access_point_duplicate() {
    let (client, _mock) = make_client().await;

    let region1 = aws_sdk_s3control::types::Region::builder()
        .bucket("bucket-1")
        .build()
        .unwrap();

    let details = aws_sdk_s3control::types::CreateMultiRegionAccessPointInput::builder()
        .name("dup-mrap")
        .regions(region1)
        .build()
        .unwrap();

    client
        .create_multi_region_access_point()
        .account_id("123456789012")
        .details(details.clone())
        .send()
        .await
        .unwrap();

    let err = client
        .create_multi_region_access_point()
        .account_id("123456789012")
        .details(details)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("MultiRegionAccessPointAlreadyExists")
            || err_str.contains("already exists"),
        "Expected duplicate error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_multi_region_access_point_policy() {
    let (client, _mock) = make_client().await;

    let region1 = aws_sdk_s3control::types::Region::builder()
        .bucket("bucket-policy")
        .build()
        .unwrap();

    let details = aws_sdk_s3control::types::CreateMultiRegionAccessPointInput::builder()
        .name("policy-mrap")
        .regions(region1)
        .build()
        .unwrap();

    client
        .create_multi_region_access_point()
        .account_id("123456789012")
        .details(details)
        .send()
        .await
        .unwrap();

    // Put policy
    let policy_doc = r#"{"Version":"2012-10-17","Statement":[]}"#;
    let put_details = aws_sdk_s3control::types::PutMultiRegionAccessPointPolicyInput::builder()
        .name("policy-mrap")
        .policy(policy_doc)
        .build()
        .unwrap();

    let put = client
        .put_multi_region_access_point_policy()
        .account_id("123456789012")
        .details(put_details)
        .send()
        .await
        .unwrap();
    assert!(put.request_token_arn().is_some());

    // Get policy
    let get = client
        .get_multi_region_access_point_policy()
        .account_id("123456789012")
        .name("policy-mrap")
        .send()
        .await
        .unwrap();
    let policy = get.policy().unwrap();
    let established = policy.established().unwrap();
    assert_eq!(established.policy(), Some(policy_doc));
}

#[tokio::test]
async fn test_multi_region_access_point_policy_status() {
    let (client, _mock) = make_client().await;

    let region1 = aws_sdk_s3control::types::Region::builder()
        .bucket("bucket-ps")
        .build()
        .unwrap();

    let details = aws_sdk_s3control::types::CreateMultiRegionAccessPointInput::builder()
        .name("ps-mrap")
        .regions(region1)
        .build()
        .unwrap();

    client
        .create_multi_region_access_point()
        .account_id("123456789012")
        .details(details)
        .send()
        .await
        .unwrap();

    // No policy -> not public
    let resp = client
        .get_multi_region_access_point_policy_status()
        .account_id("123456789012")
        .name("ps-mrap")
        .send()
        .await
        .unwrap();
    assert!(!resp.established().unwrap().is_public());

    // Put a policy
    let put_details = aws_sdk_s3control::types::PutMultiRegionAccessPointPolicyInput::builder()
        .name("ps-mrap")
        .policy(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .build()
        .unwrap();

    client
        .put_multi_region_access_point_policy()
        .account_id("123456789012")
        .details(put_details)
        .send()
        .await
        .unwrap();

    // Has policy -> is_public = true
    let resp = client
        .get_multi_region_access_point_policy_status()
        .account_id("123456789012")
        .name("ps-mrap")
        .send()
        .await
        .unwrap();
    assert!(resp.established().unwrap().is_public());
}

// ============================================================================
// Jobs (S3 Batch Operations)
// ============================================================================

#[tokio::test]
async fn test_job_lifecycle() {
    let (client, _mock) = make_client().await;

    // List - empty
    let list = client
        .list_jobs()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert!(list.jobs().is_empty());

    // Create a job
    let operation = aws_sdk_s3control::types::JobOperation::builder()
        .s3_put_object_tagging(
            aws_sdk_s3control::types::S3SetObjectTaggingOperation::builder().build(),
        )
        .build();

    let manifest = aws_sdk_s3control::types::JobManifest::builder()
        .spec(
            aws_sdk_s3control::types::JobManifestSpec::builder()
                .format(aws_sdk_s3control::types::JobManifestFormat::S3BatchOperationsCsv20180820)
                .build()
                .unwrap(),
        )
        .location(
            aws_sdk_s3control::types::JobManifestLocation::builder()
                .object_arn("arn:aws:s3:::my-bucket/manifest.csv")
                .e_tag("abc123")
                .build()
                .unwrap(),
        )
        .build();

    let report = aws_sdk_s3control::types::JobReport::builder()
        .enabled(false)
        .build();

    let create = client
        .create_job()
        .account_id("123456789012")
        .operation(operation)
        .manifest(manifest)
        .report(report)
        .priority(10)
        .role_arn("arn:aws:iam::123456789012:role/batch-role")
        .description("Test batch job")
        .send()
        .await
        .expect("create job should succeed");

    let job_id = create.job_id().unwrap().to_string();
    assert!(!job_id.is_empty());

    // Describe
    let desc = client
        .describe_job()
        .account_id("123456789012")
        .job_id(&job_id)
        .send()
        .await
        .unwrap();

    let job = desc.job().unwrap();
    assert_eq!(job.job_id(), Some(job_id.as_str()));
    assert_eq!(job.priority(), 10);
    assert_eq!(job.description(), Some("Test batch job"));
    assert_eq!(job.status().map(|s| s.as_str()), Some("New"));

    // List - one job
    let list = client
        .list_jobs()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert_eq!(list.jobs().len(), 1);
}

#[tokio::test]
async fn test_job_tagging_lifecycle() {
    let (client, _mock) = make_client().await;

    // Create a job
    let operation = aws_sdk_s3control::types::JobOperation::builder()
        .s3_put_object_tagging(
            aws_sdk_s3control::types::S3SetObjectTaggingOperation::builder().build(),
        )
        .build();

    let manifest = aws_sdk_s3control::types::JobManifest::builder()
        .spec(
            aws_sdk_s3control::types::JobManifestSpec::builder()
                .format(aws_sdk_s3control::types::JobManifestFormat::S3BatchOperationsCsv20180820)
                .build()
                .unwrap(),
        )
        .location(
            aws_sdk_s3control::types::JobManifestLocation::builder()
                .object_arn("arn:aws:s3:::bucket/manifest.csv")
                .e_tag("abc")
                .build()
                .unwrap(),
        )
        .build();

    let report = aws_sdk_s3control::types::JobReport::builder()
        .enabled(false)
        .build();

    let create = client
        .create_job()
        .account_id("123456789012")
        .operation(operation)
        .manifest(manifest)
        .report(report)
        .priority(5)
        .role_arn("arn:aws:iam::123456789012:role/batch-role")
        .send()
        .await
        .unwrap();

    let job_id = create.job_id().unwrap().to_string();

    // Get tags - initially empty
    let tags = client
        .get_job_tagging()
        .account_id("123456789012")
        .job_id(&job_id)
        .send()
        .await
        .unwrap();
    assert!(tags.tags().is_empty());

    // Put tags
    let tag1 = aws_sdk_s3control::types::S3Tag::builder()
        .key("env")
        .value("test")
        .build()
        .unwrap();
    let tag2 = aws_sdk_s3control::types::S3Tag::builder()
        .key("team")
        .value("platform")
        .build()
        .unwrap();

    client
        .put_job_tagging()
        .account_id("123456789012")
        .job_id(&job_id)
        .tags(tag1)
        .tags(tag2)
        .send()
        .await
        .expect("put_job_tagging should succeed");

    // Get tags - should have 2
    let tags = client
        .get_job_tagging()
        .account_id("123456789012")
        .job_id(&job_id)
        .send()
        .await
        .unwrap();
    assert_eq!(tags.tags().len(), 2);

    // Delete tags
    client
        .delete_job_tagging()
        .account_id("123456789012")
        .job_id(&job_id)
        .send()
        .await
        .expect("delete_job_tagging should succeed");

    // Get tags - should be empty
    let tags = client
        .get_job_tagging()
        .account_id("123456789012")
        .job_id(&job_id)
        .send()
        .await
        .unwrap();
    assert!(tags.tags().is_empty());
}

#[tokio::test]
async fn test_job_update_priority() {
    let (client, _mock) = make_client().await;

    let operation = aws_sdk_s3control::types::JobOperation::builder()
        .s3_put_object_tagging(
            aws_sdk_s3control::types::S3SetObjectTaggingOperation::builder().build(),
        )
        .build();

    let manifest = aws_sdk_s3control::types::JobManifest::builder()
        .spec(
            aws_sdk_s3control::types::JobManifestSpec::builder()
                .format(aws_sdk_s3control::types::JobManifestFormat::S3BatchOperationsCsv20180820)
                .build()
                .unwrap(),
        )
        .location(
            aws_sdk_s3control::types::JobManifestLocation::builder()
                .object_arn("arn:aws:s3:::bucket/manifest.csv")
                .e_tag("abc")
                .build()
                .unwrap(),
        )
        .build();

    let report = aws_sdk_s3control::types::JobReport::builder()
        .enabled(false)
        .build();

    let create = client
        .create_job()
        .account_id("123456789012")
        .operation(operation)
        .manifest(manifest)
        .report(report)
        .priority(10)
        .role_arn("arn:aws:iam::123456789012:role/batch-role")
        .send()
        .await
        .unwrap();

    let job_id = create.job_id().unwrap().to_string();

    // Update priority
    let update = client
        .update_job_priority()
        .account_id("123456789012")
        .job_id(&job_id)
        .priority(50)
        .send()
        .await
        .expect("update_job_priority should succeed");

    assert_eq!(update.priority(), 50);

    // Verify via describe
    let desc = client
        .describe_job()
        .account_id("123456789012")
        .job_id(&job_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.job().unwrap().priority(), 50);
}

#[tokio::test]
async fn test_job_update_status() {
    let (client, _mock) = make_client().await;

    let operation = aws_sdk_s3control::types::JobOperation::builder()
        .s3_put_object_tagging(
            aws_sdk_s3control::types::S3SetObjectTaggingOperation::builder().build(),
        )
        .build();

    let manifest = aws_sdk_s3control::types::JobManifest::builder()
        .spec(
            aws_sdk_s3control::types::JobManifestSpec::builder()
                .format(aws_sdk_s3control::types::JobManifestFormat::S3BatchOperationsCsv20180820)
                .build()
                .unwrap(),
        )
        .location(
            aws_sdk_s3control::types::JobManifestLocation::builder()
                .object_arn("arn:aws:s3:::bucket/manifest.csv")
                .e_tag("abc")
                .build()
                .unwrap(),
        )
        .build();

    let report = aws_sdk_s3control::types::JobReport::builder()
        .enabled(false)
        .build();

    let create = client
        .create_job()
        .account_id("123456789012")
        .operation(operation)
        .manifest(manifest)
        .report(report)
        .priority(10)
        .role_arn("arn:aws:iam::123456789012:role/batch-role")
        .send()
        .await
        .unwrap();

    let job_id = create.job_id().unwrap().to_string();

    // Update status to Ready
    let update = client
        .update_job_status()
        .account_id("123456789012")
        .job_id(&job_id)
        .requested_job_status(aws_sdk_s3control::types::RequestedJobStatus::Ready)
        .send()
        .await
        .expect("update_job_status should succeed");

    assert_eq!(update.status().map(|s| s.as_str()), Some("Ready"));

    // Verify
    let desc = client
        .describe_job()
        .account_id("123456789012")
        .job_id(&job_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.job().unwrap().status().map(|s| s.as_str()),
        Some("Ready")
    );
}

#[tokio::test]
async fn test_job_confirmation_required() {
    let (client, _mock) = make_client().await;

    let operation = aws_sdk_s3control::types::JobOperation::builder()
        .s3_put_object_tagging(
            aws_sdk_s3control::types::S3SetObjectTaggingOperation::builder().build(),
        )
        .build();

    let manifest = aws_sdk_s3control::types::JobManifest::builder()
        .spec(
            aws_sdk_s3control::types::JobManifestSpec::builder()
                .format(aws_sdk_s3control::types::JobManifestFormat::S3BatchOperationsCsv20180820)
                .build()
                .unwrap(),
        )
        .location(
            aws_sdk_s3control::types::JobManifestLocation::builder()
                .object_arn("arn:aws:s3:::bucket/manifest.csv")
                .e_tag("abc")
                .build()
                .unwrap(),
        )
        .build();

    let report = aws_sdk_s3control::types::JobReport::builder()
        .enabled(false)
        .build();

    let create = client
        .create_job()
        .account_id("123456789012")
        .operation(operation)
        .manifest(manifest)
        .report(report)
        .priority(10)
        .role_arn("arn:aws:iam::123456789012:role/batch-role")
        .confirmation_required(true)
        .send()
        .await
        .unwrap();

    let job_id = create.job_id().unwrap().to_string();

    // Job should be in Suspended status
    let desc = client
        .describe_job()
        .account_id("123456789012")
        .job_id(&job_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.job().unwrap().status().map(|s| s.as_str()),
        Some("Suspended")
    );
    assert_eq!(desc.job().unwrap().confirmation_required(), Some(true));
}

// ============================================================================
// Storage Lens Groups
// ============================================================================

#[tokio::test]
async fn test_storage_lens_group_lifecycle() {
    let (client, _mock) = make_client().await;

    // List - empty
    let list = client
        .list_storage_lens_groups()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert!(list.storage_lens_group_list().is_empty());

    // Create
    let filter = aws_sdk_s3control::types::StorageLensGroupFilter::builder().build();
    let group = aws_sdk_s3control::types::StorageLensGroup::builder()
        .name("test-slg")
        .filter(filter)
        .build()
        .unwrap();

    client
        .create_storage_lens_group()
        .account_id("123456789012")
        .storage_lens_group(group)
        .send()
        .await
        .expect("create SLG should succeed");

    // Get
    let get = client
        .get_storage_lens_group()
        .account_id("123456789012")
        .name("test-slg")
        .send()
        .await
        .unwrap();
    assert_eq!(get.storage_lens_group().unwrap().name(), "test-slg");
    assert!(
        get.storage_lens_group()
            .unwrap()
            .storage_lens_group_arn()
            .is_some()
    );

    // List - one item
    let list = client
        .list_storage_lens_groups()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert_eq!(list.storage_lens_group_list().len(), 1);

    // Update
    let updated_filter = aws_sdk_s3control::types::StorageLensGroupFilter::builder().build();
    let updated_group = aws_sdk_s3control::types::StorageLensGroup::builder()
        .name("test-slg")
        .filter(updated_filter)
        .build()
        .unwrap();

    client
        .update_storage_lens_group()
        .account_id("123456789012")
        .name("test-slg")
        .storage_lens_group(updated_group)
        .send()
        .await
        .expect("update SLG should succeed");

    // Delete
    client
        .delete_storage_lens_group()
        .account_id("123456789012")
        .name("test-slg")
        .send()
        .await
        .unwrap();

    // Verify gone
    let result = client
        .get_storage_lens_group()
        .account_id("123456789012")
        .name("test-slg")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_storage_lens_group_duplicate() {
    let (client, _mock) = make_client().await;

    let filter = aws_sdk_s3control::types::StorageLensGroupFilter::builder().build();
    let group = aws_sdk_s3control::types::StorageLensGroup::builder()
        .name("dup-slg")
        .filter(filter)
        .build()
        .unwrap();

    client
        .create_storage_lens_group()
        .account_id("123456789012")
        .storage_lens_group(group.clone())
        .send()
        .await
        .unwrap();

    let err = client
        .create_storage_lens_group()
        .account_id("123456789012")
        .storage_lens_group(group)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("StorageLensGroupAlreadyExists") || err_str.contains("already exists"),
        "Expected duplicate error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_storage_lens_group_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .get_storage_lens_group()
        .account_id("123456789012")
        .name("nonexistent-slg")
        .send()
        .await;
    assert!(result.is_err());

    let result = client
        .delete_storage_lens_group()
        .account_id("123456789012")
        .name("nonexistent-slg")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Outposts Buckets
// ============================================================================

#[tokio::test]
async fn test_outposts_bucket_lifecycle() {
    let (client, _mock) = make_client().await;

    // List - empty
    let list = client
        .list_regional_buckets()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert!(list.regional_bucket_list().is_empty());

    // Create
    let create = client
        .create_bucket()
        .bucket("outpost-bucket")
        .outpost_id("op-12345678")
        .send()
        .await
        .expect("create outpost bucket should succeed");
    assert!(create.bucket_arn().is_some());

    // Get
    let get = client
        .get_bucket()
        .account_id("123456789012")
        .bucket("outpost-bucket")
        .send()
        .await
        .unwrap();
    assert!(get.bucket().is_some());
    assert!(get.creation_date().is_some());

    // List - one item
    let list = client
        .list_regional_buckets()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert_eq!(list.regional_bucket_list().len(), 1);

    // Delete
    client
        .delete_bucket()
        .account_id("123456789012")
        .bucket("outpost-bucket")
        .send()
        .await
        .unwrap();

    // Verify gone
    let result = client
        .get_bucket()
        .account_id("123456789012")
        .bucket("outpost-bucket")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_outposts_bucket_policy_lifecycle() {
    let (client, _mock) = make_client().await;

    // Create bucket
    client
        .create_bucket()
        .bucket("policy-bucket")
        .outpost_id("op-12345678")
        .send()
        .await
        .unwrap();

    // Get policy - initially empty
    let get = client
        .get_bucket_policy()
        .account_id("123456789012")
        .bucket("policy-bucket")
        .send()
        .await
        .unwrap();
    assert!(get.policy().is_none() || get.policy() == Some(""));

    // Put policy
    let policy = r#"{"Version":"2012-10-17","Statement":[]}"#;
    client
        .put_bucket_policy()
        .account_id("123456789012")
        .bucket("policy-bucket")
        .policy(policy)
        .send()
        .await
        .expect("put bucket policy should succeed");

    // Get policy
    let get = client
        .get_bucket_policy()
        .account_id("123456789012")
        .bucket("policy-bucket")
        .send()
        .await
        .unwrap();
    assert_eq!(get.policy(), Some(policy));

    // Delete policy
    client
        .delete_bucket_policy()
        .account_id("123456789012")
        .bucket("policy-bucket")
        .send()
        .await
        .unwrap();

    // Get policy - should be empty
    let get = client
        .get_bucket_policy()
        .account_id("123456789012")
        .bucket("policy-bucket")
        .send()
        .await
        .unwrap();
    assert!(get.policy().is_none() || get.policy() == Some(""));
}

#[tokio::test]
async fn test_outposts_bucket_tagging_lifecycle() {
    let (client, _mock) = make_client().await;

    // Create bucket
    client
        .create_bucket()
        .bucket("tag-bucket")
        .outpost_id("op-12345678")
        .send()
        .await
        .unwrap();

    // Get tags - initially empty
    let tags = client
        .get_bucket_tagging()
        .account_id("123456789012")
        .bucket("tag-bucket")
        .send()
        .await
        .unwrap();
    assert!(tags.tag_set().is_empty());

    // Put tags
    let tag1 = aws_sdk_s3control::types::S3Tag::builder()
        .key("env")
        .value("prod")
        .build()
        .unwrap();

    let tagging = aws_sdk_s3control::types::Tagging::builder()
        .tag_set(tag1)
        .build()
        .unwrap();

    client
        .put_bucket_tagging()
        .account_id("123456789012")
        .bucket("tag-bucket")
        .tagging(tagging)
        .send()
        .await
        .expect("put bucket tagging should succeed");

    // Get tags - one tag
    let tags = client
        .get_bucket_tagging()
        .account_id("123456789012")
        .bucket("tag-bucket")
        .send()
        .await
        .unwrap();
    assert_eq!(tags.tag_set().len(), 1);

    // Delete tags
    client
        .delete_bucket_tagging()
        .account_id("123456789012")
        .bucket("tag-bucket")
        .send()
        .await
        .unwrap();

    // Get tags - empty
    let tags = client
        .get_bucket_tagging()
        .account_id("123456789012")
        .bucket("tag-bucket")
        .send()
        .await
        .unwrap();
    assert!(tags.tag_set().is_empty());
}

// ============================================================================
// Resource Tags (TagResource / UntagResource / ListTagsForResource)
// ============================================================================

#[tokio::test]
async fn test_resource_tags_lifecycle() {
    let (client, _mock) = make_client().await;

    // Create an access point to get an ARN to tag
    let create = client
        .create_access_point()
        .account_id("123456789012")
        .name("tag-test-ap")
        .bucket("tag-test-bucket")
        .send()
        .await
        .unwrap();
    let arn = create.access_point_arn().unwrap().to_string();

    // List tags - initially empty
    let tags = client
        .list_tags_for_resource()
        .account_id("123456789012")
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert!(tags.tags().is_empty());

    // Tag the resource
    let tag1 = aws_sdk_s3control::types::Tag::builder()
        .key("env")
        .value("staging")
        .build()
        .unwrap();
    let tag2 = aws_sdk_s3control::types::Tag::builder()
        .key("project")
        .value("winterbaume")
        .build()
        .unwrap();

    client
        .tag_resource()
        .account_id("123456789012")
        .resource_arn(&arn)
        .tags(tag1)
        .tags(tag2)
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags - two tags
    let tags = client
        .list_tags_for_resource()
        .account_id("123456789012")
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(tags.tags().len(), 2);

    // Untag one key
    client
        .untag_resource()
        .account_id("123456789012")
        .resource_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    // List tags - one tag remaining
    let tags = client
        .list_tags_for_resource()
        .account_id("123456789012")
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(tags.tags().len(), 1);
    assert_eq!(tags.tags()[0].key(), "project");
    assert_eq!(tags.tags()[0].value(), "winterbaume");
}

// ============================================================================
// Access Point Scope / Directory Buckets
// ============================================================================
// NOTE: Access Point Scope and Directory Buckets tests are skipped because
// the SDK routes these operations to s3express-control endpoints which the
// mock does not currently intercept.

// ============================================================================
// Object Lambda AP Configuration
// ============================================================================

#[tokio::test]
async fn test_object_lambda_access_point_configuration() {
    let (client, _mock) = make_client().await;

    let config = aws_sdk_s3control::types::ObjectLambdaConfiguration::builder()
        .supporting_access_point("arn:aws:s3:us-east-1:123456789012:accesspoint/supporting-ap")
        .transformation_configurations(
            aws_sdk_s3control::types::ObjectLambdaTransformationConfiguration::builder()
                .actions(aws_sdk_s3control::types::ObjectLambdaTransformationConfigurationAction::GetObject)
                .content_transformation(
                    aws_sdk_s3control::types::ObjectLambdaContentTransformation::AwsLambda(
                        aws_sdk_s3control::types::AwsLambdaTransformation::builder()
                            .function_arn("arn:aws:lambda:us-east-1:123456789012:function:my-func")
                            .build()
                            .unwrap(),
                    ),
                )
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    client
        .create_access_point_for_object_lambda()
        .account_id("123456789012")
        .name("config-ol-ap")
        .configuration(config)
        .send()
        .await
        .unwrap();

    // Get configuration
    let get = client
        .get_access_point_configuration_for_object_lambda()
        .account_id("123456789012")
        .name("config-ol-ap")
        .send()
        .await
        .expect("get OLAP configuration should succeed");
    let _ = get;
}

#[tokio::test]
async fn test_object_lambda_access_point_configuration_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .get_access_point_configuration_for_object_lambda()
        .account_id("123456789012")
        .name("nonexistent-ol-ap")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Object Lambda AP - not found errors
// ============================================================================

#[tokio::test]
async fn test_object_lambda_access_point_get_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .get_access_point_for_object_lambda()
        .account_id("123456789012")
        .name("nonexistent-ol-ap")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_object_lambda_access_point_delete_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .delete_access_point_for_object_lambda()
        .account_id("123456789012")
        .name("nonexistent-ol-ap")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Object Lambda AP Policy - not found errors
// ============================================================================

#[tokio::test]
async fn test_object_lambda_access_point_policy_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .get_access_point_policy_for_object_lambda()
        .account_id("123456789012")
        .name("nonexistent-ol-ap")
        .send()
        .await;
    assert!(result.is_err());

    let result = client
        .put_access_point_policy_for_object_lambda()
        .account_id("123456789012")
        .name("nonexistent-ol-ap")
        .policy(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await;
    assert!(result.is_err());

    let result = client
        .delete_access_point_policy_for_object_lambda()
        .account_id("123456789012")
        .name("nonexistent-ol-ap")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_object_lambda_access_point_policy_status_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .get_access_point_policy_status_for_object_lambda()
        .account_id("123456789012")
        .name("nonexistent-ol-ap")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Access Point Policy - not found errors
// ============================================================================

#[tokio::test]
async fn test_access_point_policy_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .get_access_point_policy()
        .account_id("123456789012")
        .name("nonexistent-ap")
        .send()
        .await;
    assert!(result.is_err());

    let result = client
        .put_access_point_policy()
        .account_id("123456789012")
        .name("nonexistent-ap")
        .policy(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await;
    assert!(result.is_err());

    let result = client
        .delete_access_point_policy()
        .account_id("123456789012")
        .name("nonexistent-ap")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_access_point_policy_status_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .get_access_point_policy_status()
        .account_id("123456789012")
        .name("nonexistent-ap")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Describe Multi-Region Access Point Operation
// ============================================================================

#[tokio::test]
async fn test_describe_multi_region_access_point_operation() {
    let (client, _mock) = make_client().await;

    // Create a MRAP to get a request token ARN
    let region1 = aws_sdk_s3control::types::Region::builder()
        .bucket("bucket-op")
        .build()
        .unwrap();

    let details = aws_sdk_s3control::types::CreateMultiRegionAccessPointInput::builder()
        .name("op-mrap")
        .regions(region1)
        .build()
        .unwrap();

    let create = client
        .create_multi_region_access_point()
        .account_id("123456789012")
        .details(details)
        .send()
        .await
        .unwrap();

    let token_arn = create.request_token_arn().unwrap().to_string();

    // Describe operation
    let resp = client
        .describe_multi_region_access_point_operation()
        .account_id("123456789012")
        .request_token_arn(&token_arn)
        .send()
        .await
        .expect("describe MRAP operation should succeed");

    let op = resp.async_operation().unwrap();
    assert!(op.request_token_arn().is_some());
    assert_eq!(op.request_status(), Some("SUCCEEDED"));
}

// ============================================================================
// Multi-Region Access Point - not found errors
// ============================================================================

#[tokio::test]
async fn test_multi_region_access_point_get_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .get_multi_region_access_point()
        .account_id("123456789012")
        .name("nonexistent-mrap")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_multi_region_access_point_policy_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .get_multi_region_access_point_policy()
        .account_id("123456789012")
        .name("nonexistent-mrap")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_multi_region_access_point_policy_status_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .get_multi_region_access_point_policy_status()
        .account_id("123456789012")
        .name("nonexistent-mrap")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Job - not found errors
// ============================================================================

#[tokio::test]
async fn test_describe_job_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .describe_job()
        .account_id("123456789012")
        .job_id("nonexistent-job-id")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_job_tagging_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .get_job_tagging()
        .account_id("123456789012")
        .job_id("nonexistent-job-id")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_job_priority_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .update_job_priority()
        .account_id("123456789012")
        .job_id("nonexistent-job-id")
        .priority(50)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_job_status_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .update_job_status()
        .account_id("123456789012")
        .job_id("nonexistent-job-id")
        .requested_job_status(aws_sdk_s3control::types::RequestedJobStatus::Ready)
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Job - list with status filter
// ============================================================================

#[tokio::test]
async fn test_list_jobs_filter_by_status() {
    let (client, _mock) = make_client().await;

    let operation = aws_sdk_s3control::types::JobOperation::builder()
        .s3_put_object_tagging(
            aws_sdk_s3control::types::S3SetObjectTaggingOperation::builder().build(),
        )
        .build();

    let manifest = aws_sdk_s3control::types::JobManifest::builder()
        .spec(
            aws_sdk_s3control::types::JobManifestSpec::builder()
                .format(aws_sdk_s3control::types::JobManifestFormat::S3BatchOperationsCsv20180820)
                .build()
                .unwrap(),
        )
        .location(
            aws_sdk_s3control::types::JobManifestLocation::builder()
                .object_arn("arn:aws:s3:::bucket/manifest.csv")
                .e_tag("abc")
                .build()
                .unwrap(),
        )
        .build();

    let report = aws_sdk_s3control::types::JobReport::builder()
        .enabled(false)
        .build();

    // Create a normal job (status=New)
    let create = client
        .create_job()
        .account_id("123456789012")
        .operation(operation.clone())
        .manifest(manifest.clone())
        .report(report.clone())
        .priority(10)
        .role_arn("arn:aws:iam::123456789012:role/batch-role")
        .send()
        .await
        .unwrap();

    let job_id = create.job_id().unwrap().to_string();

    // Update first job to Ready
    client
        .update_job_status()
        .account_id("123456789012")
        .job_id(&job_id)
        .requested_job_status(aws_sdk_s3control::types::RequestedJobStatus::Ready)
        .send()
        .await
        .unwrap();

    // Create a second job (stays as New)
    client
        .create_job()
        .account_id("123456789012")
        .operation(operation)
        .manifest(manifest)
        .report(report)
        .priority(5)
        .role_arn("arn:aws:iam::123456789012:role/batch-role")
        .send()
        .await
        .unwrap();

    // List all jobs - should have 2
    let all = client
        .list_jobs()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert_eq!(all.jobs().len(), 2);

    // List with status filter "Ready" - should have 1
    let ready = client
        .list_jobs()
        .account_id("123456789012")
        .job_statuses(aws_sdk_s3control::types::JobStatus::Ready)
        .send()
        .await
        .unwrap();
    assert_eq!(ready.jobs().len(), 1);
    assert_eq!(ready.jobs()[0].status().map(|s| s.as_str()), Some("Ready"));
}

// ============================================================================
// Outposts Bucket - not found errors
// ============================================================================

// ============================================================================
// Outposts Bucket error tests
// ============================================================================
// NOTE: Outposts Bucket tests (get/delete/duplicate not found, policy/tagging
// not found) are skipped because the SDK routes these operations to
// s3-outposts endpoints which the mock does not currently intercept.

// ============================================================================
// Access Grants Instance - not found errors
// ============================================================================

#[tokio::test]
async fn test_access_grants_instance_get_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .get_access_grants_instance()
        .account_id("123456789012")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_access_grants_instance_delete_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .delete_access_grants_instance()
        .account_id("123456789012")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Access Grants Instance Resource Policy - not found errors
// ============================================================================

#[tokio::test]
async fn test_access_grants_resource_policy_without_instance() {
    let (client, _mock) = make_client().await;

    // No instance created - resource policy ops should fail
    let result = client
        .get_access_grants_instance_resource_policy()
        .account_id("123456789012")
        .send()
        .await;
    assert!(result.is_err());

    let result = client
        .put_access_grants_instance_resource_policy()
        .account_id("123456789012")
        .policy(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Resource Tags - non-existent resource
// ============================================================================

#[tokio::test]
async fn test_resource_tags_list_nonexistent_resource() {
    let (client, _mock) = make_client().await;

    // Listing tags on a resource that doesn't exist in the tag map
    // should return empty (not error) or error depending on implementation
    let result = client
        .list_tags_for_resource()
        .account_id("123456789012")
        .resource_arn("arn:aws:s3:us-east-1:123456789012:accesspoint/nonexistent")
        .send()
        .await;

    // Either succeeds with empty tags or fails - both are acceptable
    if let Ok(resp) = result {
        assert!(resp.tags().is_empty());
    }
}

#[tokio::test]
async fn test_resource_tags_add_multiple_then_untag_multiple() {
    let (client, _mock) = make_client().await;

    // Create an access point
    let create = client
        .create_access_point()
        .account_id("123456789012")
        .name("multi-tag-ap")
        .bucket("multi-tag-bucket")
        .send()
        .await
        .unwrap();
    let arn = create.access_point_arn().unwrap().to_string();

    // Add 3 tags
    let tag1 = aws_sdk_s3control::types::Tag::builder()
        .key("env")
        .value("prod")
        .build()
        .unwrap();
    let tag2 = aws_sdk_s3control::types::Tag::builder()
        .key("team")
        .value("platform")
        .build()
        .unwrap();
    let tag3 = aws_sdk_s3control::types::Tag::builder()
        .key("cost-center")
        .value("12345")
        .build()
        .unwrap();

    client
        .tag_resource()
        .account_id("123456789012")
        .resource_arn(&arn)
        .tags(tag1)
        .tags(tag2)
        .tags(tag3)
        .send()
        .await
        .unwrap();

    let tags = client
        .list_tags_for_resource()
        .account_id("123456789012")
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(tags.tags().len(), 3);

    // Untag one key
    client
        .untag_resource()
        .account_id("123456789012")
        .resource_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .unwrap();

    let tags = client
        .list_tags_for_resource()
        .account_id("123456789012")
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(tags.tags().len(), 2);

    // Untag another key
    client
        .untag_resource()
        .account_id("123456789012")
        .resource_arn(&arn)
        .tag_keys("cost-center")
        .send()
        .await
        .unwrap();

    let tags = client
        .list_tags_for_resource()
        .account_id("123456789012")
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(tags.tags().len(), 1);
    assert_eq!(tags.tags()[0].key(), "team");
}

// ============================================================================
// Public Access Block - overwrite existing
// ============================================================================

#[tokio::test]
async fn test_public_access_block_overwrite() {
    let (client, _mock) = make_client().await;

    // Put initial config
    client
        .put_public_access_block()
        .account_id("123456789012")
        .public_access_block_configuration(
            aws_sdk_s3control::types::PublicAccessBlockConfiguration::builder()
                .block_public_acls(true)
                .ignore_public_acls(true)
                .block_public_policy(false)
                .restrict_public_buckets(false)
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Overwrite with different values
    client
        .put_public_access_block()
        .account_id("123456789012")
        .public_access_block_configuration(
            aws_sdk_s3control::types::PublicAccessBlockConfiguration::builder()
                .block_public_acls(false)
                .ignore_public_acls(false)
                .block_public_policy(true)
                .restrict_public_buckets(true)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .get_public_access_block()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();

    let config = resp.public_access_block_configuration().unwrap();
    assert_eq!(config.block_public_acls(), Some(false));
    assert_eq!(config.ignore_public_acls(), Some(false));
    assert_eq!(config.block_public_policy(), Some(true));
    assert_eq!(config.restrict_public_buckets(), Some(true));
}

// ============================================================================
// Access Point - create with public access block configuration
// ============================================================================

#[tokio::test]
async fn test_create_access_point_with_public_access_block() {
    let (client, _mock) = make_client().await;

    let pab = aws_sdk_s3control::types::PublicAccessBlockConfiguration::builder()
        .block_public_acls(false)
        .ignore_public_acls(false)
        .block_public_policy(false)
        .restrict_public_buckets(false)
        .build();

    client
        .create_access_point()
        .account_id("123456789012")
        .name("pab-ap")
        .bucket("pab-bucket")
        .public_access_block_configuration(pab)
        .send()
        .await
        .expect("create AP with custom PAB should succeed");

    let get = client
        .get_access_point()
        .account_id("123456789012")
        .name("pab-ap")
        .send()
        .await
        .unwrap();

    let pab_config = get.public_access_block_configuration().unwrap();
    assert_eq!(pab_config.block_public_acls(), Some(false));
    assert_eq!(pab_config.ignore_public_acls(), Some(false));
    assert_eq!(pab_config.block_public_policy(), Some(false));
    assert_eq!(pab_config.restrict_public_buckets(), Some(false));
}

// ============================================================================
// Access Grant - create with application ARN
// ============================================================================

#[tokio::test]
async fn test_access_grant_with_application_arn() {
    let (client, _mock) = make_client().await;

    // Create location
    let loc = client
        .create_access_grants_location()
        .account_id("123456789012")
        .location_scope("s3://app-bucket/*")
        .iam_role_arn("arn:aws:iam::123456789012:role/app-role")
        .send()
        .await
        .unwrap();
    let loc_id = loc.access_grants_location_id().unwrap().to_string();

    // Create grant with application ARN
    let grantee = aws_sdk_s3control::types::Grantee::builder()
        .grantee_type(aws_sdk_s3control::types::GranteeType::DirectoryUser)
        .grantee_identifier("appuser@example.com")
        .build();

    let create = client
        .create_access_grant()
        .account_id("123456789012")
        .access_grants_location_id(&loc_id)
        .grantee(grantee)
        .permission(aws_sdk_s3control::types::Permission::Readwrite)
        .application_arn("arn:aws:sso::123456789012:application/app-id")
        .send()
        .await
        .expect("create grant with app ARN should succeed");

    let grant_id = create.access_grant_id().unwrap().to_string();
    assert!(create.access_grant_arn().is_some());

    // Get and verify
    let get = client
        .get_access_grant()
        .account_id("123456789012")
        .access_grant_id(&grant_id)
        .send()
        .await
        .unwrap();
    assert_eq!(get.access_grant_id(), Some(grant_id.as_str()));
    assert!(get.grantee().is_some());
}

// ============================================================================
// Multiple Access Grants Locations
// ============================================================================

#[tokio::test]
async fn test_multiple_access_grants_locations() {
    let (client, _mock) = make_client().await;

    // Create two locations
    let loc1 = client
        .create_access_grants_location()
        .account_id("123456789012")
        .location_scope("s3://bucket-one/*")
        .iam_role_arn("arn:aws:iam::123456789012:role/role-one")
        .send()
        .await
        .unwrap();
    assert!(loc1.access_grants_location_id().is_some());

    let loc2 = client
        .create_access_grants_location()
        .account_id("123456789012")
        .location_scope("s3://bucket-two/*")
        .iam_role_arn("arn:aws:iam::123456789012:role/role-two")
        .send()
        .await
        .unwrap();
    assert!(loc2.access_grants_location_id().is_some());

    // List should have 2
    let list = client
        .list_access_grants_locations()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert_eq!(list.access_grants_locations_list().len(), 2);

    // Delete one
    client
        .delete_access_grants_location()
        .account_id("123456789012")
        .access_grants_location_id(loc1.access_grants_location_id().unwrap())
        .send()
        .await
        .unwrap();

    // List should have 1
    let list = client
        .list_access_grants_locations()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert_eq!(list.access_grants_locations_list().len(), 1);
}

// ============================================================================
// Storage Lens Group - update not found
// ============================================================================

#[tokio::test]
async fn test_storage_lens_group_update_not_found() {
    let (client, _mock) = make_client().await;

    let filter = aws_sdk_s3control::types::StorageLensGroupFilter::builder().build();
    let group = aws_sdk_s3control::types::StorageLensGroup::builder()
        .name("nonexistent-slg")
        .filter(filter)
        .build()
        .unwrap();

    let result = client
        .update_storage_lens_group()
        .account_id("123456789012")
        .name("nonexistent-slg")
        .storage_lens_group(group)
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Account isolation - different accounts don't see each other's resources
// ============================================================================

#[tokio::test]
async fn test_account_isolation_access_points() {
    let (client, _mock) = make_client().await;

    // Create AP in account A
    client
        .create_access_point()
        .account_id("111111111111")
        .name("iso-ap")
        .bucket("iso-bucket")
        .send()
        .await
        .unwrap();

    // Account B should not see it
    let list = client
        .list_access_points()
        .account_id("222222222222")
        .send()
        .await
        .unwrap();
    assert!(list.access_point_list().is_empty());

    // Account A should see it
    let list = client
        .list_access_points()
        .account_id("111111111111")
        .send()
        .await
        .unwrap();
    assert_eq!(list.access_point_list().len(), 1);
}

// ============================================================================
// Multi-Region AP - multiple regions
// ============================================================================

#[tokio::test]
async fn test_multi_region_access_point_multiple_regions() {
    let (client, _mock) = make_client().await;

    let region1 = aws_sdk_s3control::types::Region::builder()
        .bucket("bucket-us-east-1")
        .build()
        .unwrap();
    let region2 = aws_sdk_s3control::types::Region::builder()
        .bucket("bucket-eu-west-1")
        .build()
        .unwrap();

    let details = aws_sdk_s3control::types::CreateMultiRegionAccessPointInput::builder()
        .name("multi-region-mrap")
        .regions(region1)
        .regions(region2)
        .build()
        .unwrap();

    client
        .create_multi_region_access_point()
        .account_id("123456789012")
        .details(details)
        .send()
        .await
        .unwrap();

    let get = client
        .get_multi_region_access_point()
        .account_id("123456789012")
        .name("multi-region-mrap")
        .send()
        .await
        .unwrap();

    let ap = get.access_point().unwrap();
    assert_eq!(ap.regions().len(), 2);
}

// ============================================================================
// Job - create multiple and list
// ============================================================================

#[tokio::test]
async fn test_create_multiple_jobs_and_list() {
    let (client, _mock) = make_client().await;

    let operation = aws_sdk_s3control::types::JobOperation::builder()
        .s3_put_object_tagging(
            aws_sdk_s3control::types::S3SetObjectTaggingOperation::builder().build(),
        )
        .build();

    let manifest = aws_sdk_s3control::types::JobManifest::builder()
        .spec(
            aws_sdk_s3control::types::JobManifestSpec::builder()
                .format(aws_sdk_s3control::types::JobManifestFormat::S3BatchOperationsCsv20180820)
                .build()
                .unwrap(),
        )
        .location(
            aws_sdk_s3control::types::JobManifestLocation::builder()
                .object_arn("arn:aws:s3:::bucket/manifest.csv")
                .e_tag("abc")
                .build()
                .unwrap(),
        )
        .build();

    let report = aws_sdk_s3control::types::JobReport::builder()
        .enabled(false)
        .build();

    // Create three jobs
    for i in 0..3 {
        client
            .create_job()
            .account_id("123456789012")
            .operation(operation.clone())
            .manifest(manifest.clone())
            .report(report.clone())
            .priority(i * 10 + 1)
            .role_arn("arn:aws:iam::123456789012:role/batch-role")
            .description(format!("Job {i}"))
            .send()
            .await
            .unwrap();
    }

    let list = client
        .list_jobs()
        .account_id("123456789012")
        .send()
        .await
        .unwrap();
    assert_eq!(list.jobs().len(), 3);
}

// ============================================================================
// Outposts Bucket - list / tagging overwrite tests
// ============================================================================
// NOTE: Outposts Bucket list and tagging overwrite tests are skipped because
// the SDK routes these operations to s3-outposts endpoints which the mock
// does not currently intercept.
