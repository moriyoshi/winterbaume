use aws_sdk_cloudcontrol::config::BehaviorVersion;
use winterbaume_cloudcontrol::CloudControlService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_cloudcontrol::Client {
    let mock = MockAws::builder()
        .with_service(CloudControlService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudcontrol::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_cloudcontrol::Client::new(&config)
}

fn make_service() -> CloudControlService {
    CloudControlService::new()
}

// ---------------------------------------------------------------------------
// CreateResource
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_resource() {
    let client = make_client().await;
    let resp = client
        .create_resource()
        .type_name("AWS::S3::Bucket")
        .desired_state(r#"{"BucketName":"my-bucket"}"#)
        .send()
        .await
        .expect("create_resource should succeed");

    let event = resp.progress_event().expect("should have progress event");
    assert_eq!(
        event.operation_status().map(|s| s.as_str()),
        Some("SUCCESS")
    );
    assert_eq!(event.operation().map(|s| s.as_str()), Some("CREATE"));
    assert_eq!(event.type_name(), Some("AWS::S3::Bucket"));
    assert_eq!(event.identifier(), Some("my-bucket"));
    assert!(event.request_token().is_some());
}

// ---------------------------------------------------------------------------
// GetResource
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_get_resource_after_create() {
    let client = make_client().await;
    client
        .create_resource()
        .type_name("AWS::S3::Bucket")
        .desired_state(r#"{"BucketName":"test-bucket"}"#)
        .send()
        .await
        .expect("create should succeed");

    let resp = client
        .get_resource()
        .type_name("AWS::S3::Bucket")
        .identifier("test-bucket")
        .send()
        .await
        .expect("get_resource should succeed");

    assert_eq!(resp.type_name(), Some("AWS::S3::Bucket"));
    let desc = resp
        .resource_description()
        .expect("should have description");
    assert_eq!(desc.identifier(), Some("test-bucket"));
    assert!(desc.properties().is_some());
}

#[tokio::test]
async fn test_get_resource_not_found() {
    let client = make_client().await;
    let err = client
        .get_resource()
        .type_name("AWS::S3::Bucket")
        .identifier("nonexistent")
        .send()
        .await
        .expect_err("should fail for nonexistent resource");

    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// ---------------------------------------------------------------------------
// DeleteResource
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_delete_resource() {
    let client = make_client().await;
    client
        .create_resource()
        .type_name("AWS::S3::Bucket")
        .desired_state(r#"{"BucketName":"del-bucket"}"#)
        .send()
        .await
        .expect("create should succeed");

    let resp = client
        .delete_resource()
        .type_name("AWS::S3::Bucket")
        .identifier("del-bucket")
        .send()
        .await
        .expect("delete_resource should succeed");

    let event = resp.progress_event().expect("should have progress event");
    assert_eq!(
        event.operation_status().map(|s| s.as_str()),
        Some("SUCCESS")
    );
    assert_eq!(event.operation().map(|s| s.as_str()), Some("DELETE"));

    // Verify it's gone
    let err = client
        .get_resource()
        .type_name("AWS::S3::Bucket")
        .identifier("del-bucket")
        .send()
        .await
        .expect_err("should fail after deletion");

    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

#[tokio::test]
async fn test_delete_resource_not_found() {
    let client = make_client().await;
    let err = client
        .delete_resource()
        .type_name("AWS::S3::Bucket")
        .identifier("nonexistent")
        .send()
        .await
        .expect_err("should fail for nonexistent resource");

    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// ---------------------------------------------------------------------------
// UpdateResource
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_update_resource() {
    let client = make_client().await;
    client
        .create_resource()
        .type_name("AWS::S3::Bucket")
        .desired_state(r#"{"BucketName":"upd-bucket","Tags":[]}"#)
        .send()
        .await
        .expect("create should succeed");

    let resp = client
        .update_resource()
        .type_name("AWS::S3::Bucket")
        .identifier("upd-bucket")
        .patch_document(r#"[{"op":"add","path":"/Tags","value":[{"Key":"env","Value":"test"}]}]"#)
        .send()
        .await
        .expect("update_resource should succeed");

    let event = resp.progress_event().expect("should have progress event");
    assert_eq!(
        event.operation_status().map(|s| s.as_str()),
        Some("SUCCESS")
    );
    assert_eq!(event.operation().map(|s| s.as_str()), Some("UPDATE"));

    // Verify the update took effect
    let get_resp = client
        .get_resource()
        .type_name("AWS::S3::Bucket")
        .identifier("upd-bucket")
        .send()
        .await
        .expect("get should succeed after update");

    let desc = get_resp.resource_description().unwrap();
    let props = desc.properties().unwrap();
    assert!(props.contains("env"));
}

#[tokio::test]
async fn test_update_resource_not_found() {
    let client = make_client().await;
    let err = client
        .update_resource()
        .type_name("AWS::S3::Bucket")
        .identifier("nonexistent")
        .patch_document(r#"[{"op":"add","path":"/Tag","value":"x"}]"#)
        .send()
        .await
        .expect_err("should fail for nonexistent resource");

    let service_err = err.into_service_error();
    assert!(service_err.is_resource_not_found_exception());
}

// ---------------------------------------------------------------------------
// ListResources
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_list_resources() {
    let client = make_client().await;
    client
        .create_resource()
        .type_name("AWS::Lambda::Function")
        .desired_state(r#"{"FunctionName":"fn-a"}"#)
        .send()
        .await
        .expect("create a should succeed");

    client
        .create_resource()
        .type_name("AWS::Lambda::Function")
        .desired_state(r#"{"FunctionName":"fn-b"}"#)
        .send()
        .await
        .expect("create b should succeed");

    // Create a resource of a different type
    client
        .create_resource()
        .type_name("AWS::S3::Bucket")
        .desired_state(r#"{"BucketName":"bkt"}"#)
        .send()
        .await
        .expect("create bucket should succeed");

    let resp = client
        .list_resources()
        .type_name("AWS::Lambda::Function")
        .send()
        .await
        .expect("list_resources should succeed");

    assert_eq!(resp.type_name(), Some("AWS::Lambda::Function"));
    let descriptions = resp.resource_descriptions();
    assert_eq!(descriptions.len(), 2);
}

// ---------------------------------------------------------------------------
// GetResourceRequestStatus
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_get_resource_request_status() {
    let client = make_client().await;
    let create_resp = client
        .create_resource()
        .type_name("AWS::S3::Bucket")
        .desired_state(r#"{"BucketName":"status-test"}"#)
        .send()
        .await
        .expect("create should succeed");

    let token = create_resp
        .progress_event()
        .unwrap()
        .request_token()
        .unwrap()
        .to_string();

    let resp = client
        .get_resource_request_status()
        .request_token(&token)
        .send()
        .await
        .expect("get_resource_request_status should succeed");

    let event = resp.progress_event().expect("should have progress event");
    assert_eq!(
        event.operation_status().map(|s| s.as_str()),
        Some("SUCCESS")
    );
    assert_eq!(event.operation().map(|s| s.as_str()), Some("CREATE"));
    assert_eq!(event.request_token(), Some(token.as_str()));
}

#[tokio::test]
async fn test_get_resource_request_status_not_found() {
    let client = make_client().await;
    let err = client
        .get_resource_request_status()
        .request_token("nonexistent-token")
        .send()
        .await
        .expect_err("should fail for nonexistent token");

    let service_err = err.into_service_error();
    assert!(service_err.is_request_token_not_found_exception());
}

// ---------------------------------------------------------------------------
// ListResourceRequests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_list_resource_requests() {
    let client = make_client().await;

    client
        .create_resource()
        .type_name("AWS::S3::Bucket")
        .desired_state(r#"{"BucketName":"req-list-1"}"#)
        .send()
        .await
        .expect("create should succeed");

    client
        .create_resource()
        .type_name("AWS::S3::Bucket")
        .desired_state(r#"{"BucketName":"req-list-2"}"#)
        .send()
        .await
        .expect("create should succeed");

    let resp = client
        .list_resource_requests()
        .send()
        .await
        .expect("list_resource_requests should succeed");

    let summaries = resp.resource_request_status_summaries();
    assert!(summaries.len() >= 2);
}

// ---------------------------------------------------------------------------
// CancelResourceRequest (error path - already completed)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_cancel_resource_request_already_complete() {
    let client = make_client().await;
    let create_resp = client
        .create_resource()
        .type_name("AWS::S3::Bucket")
        .desired_state(r#"{"BucketName":"cancel-test"}"#)
        .send()
        .await
        .expect("create should succeed");

    let token = create_resp
        .progress_event()
        .unwrap()
        .request_token()
        .unwrap()
        .to_string();

    // The operation completed synchronously (SUCCESS), so cancellation should fail.
    let err = client
        .cancel_resource_request()
        .request_token(&token)
        .send()
        .await
        .expect_err("should fail because op is already complete");

    // The SDK maps the error type; just check it's an error.
    assert!(err.into_service_error().meta().message().is_some());
}

// ---------------------------------------------------------------------------
// CreateResource - AlreadyExists
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_resource_already_exists() {
    let client = make_client().await;
    client
        .create_resource()
        .type_name("AWS::S3::Bucket")
        .desired_state(r#"{"BucketName":"dup-bucket"}"#)
        .send()
        .await
        .expect("first create should succeed");

    let err = client
        .create_resource()
        .type_name("AWS::S3::Bucket")
        .desired_state(r#"{"BucketName":"dup-bucket"}"#)
        .send()
        .await
        .expect_err("second create should fail");

    let service_err = err.into_service_error();
    assert!(service_err.is_already_exists_exception());
}

// ---------------------------------------------------------------------------
// Full lifecycle: create -> get -> update -> get -> delete -> verify gone
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_full_lifecycle() {
    let client = make_client().await;

    // Create
    let create_resp = client
        .create_resource()
        .type_name("AWS::DynamoDB::Table")
        .desired_state(
            r#"{"TableName":"my-table","KeySchema":[{"AttributeName":"pk","KeyType":"HASH"}]}"#,
        )
        .send()
        .await
        .expect("create should succeed");
    let event = create_resp.progress_event().unwrap();
    assert_eq!(
        event.operation_status().map(|s| s.as_str()),
        Some("SUCCESS")
    );

    // Get
    let get_resp = client
        .get_resource()
        .type_name("AWS::DynamoDB::Table")
        .identifier("my-table")
        .send()
        .await
        .expect("get should succeed");
    let desc = get_resp.resource_description().unwrap();
    assert_eq!(desc.identifier(), Some("my-table"));

    // Update
    client
        .update_resource()
        .type_name("AWS::DynamoDB::Table")
        .identifier("my-table")
        .patch_document(r#"[{"op":"add","path":"/BillingMode","value":"PAY_PER_REQUEST"}]"#)
        .send()
        .await
        .expect("update should succeed");

    // Get after update
    let get_resp2 = client
        .get_resource()
        .type_name("AWS::DynamoDB::Table")
        .identifier("my-table")
        .send()
        .await
        .expect("get after update should succeed");
    let props = get_resp2
        .resource_description()
        .unwrap()
        .properties()
        .unwrap();
    assert!(props.contains("PAY_PER_REQUEST"));

    // Delete
    let del_resp = client
        .delete_resource()
        .type_name("AWS::DynamoDB::Table")
        .identifier("my-table")
        .send()
        .await
        .expect("delete should succeed");
    let del_event = del_resp.progress_event().unwrap();
    assert_eq!(
        del_event.operation_status().map(|s| s.as_str()),
        Some("SUCCESS")
    );

    // Verify gone
    let err = client
        .get_resource()
        .type_name("AWS::DynamoDB::Table")
        .identifier("my-table")
        .send()
        .await
        .expect_err("should fail after deletion");
    assert!(err.into_service_error().is_resource_not_found_exception());
}

// ---------------------------------------------------------------------------
// CFN resource-type shaping: AWS::KMS::Key (regression for issue #6)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_get_resource_kms_key_applies_cfn_schema() {
    // Real Cloud Control shapes the GetResource read model from the
    // AWS::KMS::Key CFN schema: writeOnlyProperties are stripped,
    // readOnlyProperties are generated, and schema defaults are filled in.
    // Regression for https://github.com/moriyoshi/winterbaume/issues/6.
    let client = make_client().await;

    let desired_state = r#"{
        "Description": "probe",
        "KeyPolicy": {
            "Version": "2012-10-17",
            "Statement": [{
                "Effect": "Allow",
                "Principal": {"AWS": "arn:aws:iam::111122223333:root"},
                "Action": "kms:*",
                "Resource": "*"
            }]
        },
        "PendingWindowInDays": 7
    }"#;

    let create_resp = client
        .create_resource()
        .type_name("AWS::KMS::Key")
        .desired_state(desired_state)
        .send()
        .await
        .expect("create_resource should succeed");

    let key_id = create_resp
        .progress_event()
        .and_then(|e| e.identifier())
        .expect("create should return primary identifier (KeyId)")
        .to_string();

    let get_resp = client
        .get_resource()
        .type_name("AWS::KMS::Key")
        .identifier(&key_id)
        .send()
        .await
        .expect("get_resource should succeed");

    let props_json = get_resp
        .resource_description()
        .and_then(|d| d.properties())
        .expect("should have properties")
        .to_string();
    let props: serde_json::Value =
        serde_json::from_str(&props_json).expect("properties should be valid JSON");

    // writeOnly: PendingWindowInDays must be stripped
    assert!(
        props.get("PendingWindowInDays").is_none(),
        "PendingWindowInDays is writeOnly and must not appear in GetResource output"
    );

    // readOnly: KeyId + Arn generated by the service
    assert_eq!(
        props.get("KeyId").and_then(|v| v.as_str()),
        Some(key_id.as_str())
    );
    let arn = props
        .get("Arn")
        .and_then(|v| v.as_str())
        .expect("Arn must be present");
    assert!(arn.starts_with("arn:aws:kms:"), "got Arn={arn}");
    assert!(arn.ends_with(&format!(":key/{key_id}")), "got Arn={arn}");

    // Schema defaults filled in
    assert_eq!(
        props.get("KeySpec").and_then(|v| v.as_str()),
        Some("SYMMETRIC_DEFAULT")
    );
    assert_eq!(
        props.get("KeyUsage").and_then(|v| v.as_str()),
        Some("ENCRYPT_DECRYPT")
    );
    assert_eq!(
        props.get("Origin").and_then(|v| v.as_str()),
        Some("AWS_KMS")
    );
    assert_eq!(
        props.get("MultiRegion").and_then(|v| v.as_bool()),
        Some(false)
    );
    assert_eq!(props.get("Enabled").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(
        props.get("EnableKeyRotation").and_then(|v| v.as_bool()),
        Some(false)
    );
    assert_eq!(
        props
            .get("Tags")
            .and_then(|v| v.as_array())
            .map(|a| a.len()),
        Some(0)
    );

    // Pass-through properties survive unchanged
    assert_eq!(
        props.get("Description").and_then(|v| v.as_str()),
        Some("probe")
    );
    assert!(props.get("KeyPolicy").is_some());
}

#[tokio::test]
async fn test_update_resource_kms_key_strips_write_only() {
    let client = make_client().await;

    client
        .create_resource()
        .type_name("AWS::KMS::Key")
        .desired_state(r#"{"Description":"orig"}"#)
        .send()
        .await
        .expect("create should succeed");

    // Resolve KeyId via the response from a fresh create.
    let create_resp = client
        .create_resource()
        .type_name("AWS::KMS::Key")
        .desired_state(r#"{"Description":"two"}"#)
        .send()
        .await
        .expect("create should succeed");
    let key_id = create_resp
        .progress_event()
        .and_then(|e| e.identifier())
        .expect("KeyId")
        .to_string();

    // An update that re-introduces a writeOnly property must not leak it
    // into the stored model.
    client
        .update_resource()
        .type_name("AWS::KMS::Key")
        .identifier(&key_id)
        .patch_document(r#"[{"op":"add","path":"/PendingWindowInDays","value":30}]"#)
        .send()
        .await
        .expect("update should succeed");

    let get_resp = client
        .get_resource()
        .type_name("AWS::KMS::Key")
        .identifier(&key_id)
        .send()
        .await
        .expect("get should succeed");
    let props: serde_json::Value = serde_json::from_str(
        get_resp
            .resource_description()
            .unwrap()
            .properties()
            .unwrap(),
    )
    .unwrap();
    assert!(
        props.get("PendingWindowInDays").is_none(),
        "writeOnly property must be stripped after update"
    );
}

// ---------------------------------------------------------------------------
// CFN resource-type shaping: AWS::DynamoDB::Table (regression for issue #7)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_get_resource_dynamodb_table_applies_cfn_schema() {
    // Real Cloud Control shapes the GetResource read model from the
    // AWS::DynamoDB::Table CFN schema and read handler: writeOnlyProperties
    // are stripped, readOnlyProperties (Arn) are generated, and the read
    // handler reports schema-default specification blocks.
    // Regression for https://github.com/moriyoshi/winterbaume/issues/7.
    let client = make_client().await;

    let desired_state = r#"{
        "TableName": "jti-store",
        "AttributeDefinitions": [{"AttributeName": "jti", "AttributeType": "S"}],
        "KeySchema": [{"AttributeName": "jti", "KeyType": "HASH"}],
        "BillingMode": "PAY_PER_REQUEST",
        "ImportSourceSpecification": {"S3BucketSource": {"S3Bucket": "ignored"}}
    }"#;

    let create_resp = client
        .create_resource()
        .type_name("AWS::DynamoDB::Table")
        .desired_state(desired_state)
        .send()
        .await
        .expect("create_resource should succeed");

    let table_name = create_resp
        .progress_event()
        .and_then(|e| e.identifier())
        .expect("create should return primary identifier (TableName)")
        .to_string();
    assert_eq!(table_name, "jti-store");

    let get_resp = client
        .get_resource()
        .type_name("AWS::DynamoDB::Table")
        .identifier(&table_name)
        .send()
        .await
        .expect("get_resource should succeed");

    let props: serde_json::Value = serde_json::from_str(
        get_resp
            .resource_description()
            .and_then(|d| d.properties())
            .expect("should have properties"),
    )
    .expect("properties should be valid JSON");

    // writeOnly: ImportSourceSpecification must be stripped.
    assert!(
        props.get("ImportSourceSpecification").is_none(),
        "ImportSourceSpecification is writeOnly and must not appear in GetResource output"
    );

    // readOnly: Arn generated by the service.
    let arn = props
        .get("Arn")
        .and_then(|v| v.as_str())
        .expect("Arn must be present");
    assert!(arn.starts_with("arn:aws:dynamodb:"), "got Arn={arn}");
    assert!(arn.ends_with(":table/jti-store"), "got Arn={arn}");

    // Pass-through properties survive unchanged.
    assert_eq!(props["TableName"], "jti-store");
    assert_eq!(props["BillingMode"], "PAY_PER_REQUEST");

    // Schema defaults reported by the read handler.
    assert_eq!(props["ContributorInsightsSpecification"]["Enabled"], false);
    assert_eq!(props["DeletionProtectionEnabled"], false);
    assert_eq!(
        props["PointInTimeRecoverySpecification"]["PointInTimeRecoveryEnabled"],
        false
    );
    assert_eq!(props["SSESpecification"]["SSEEnabled"], false);
    assert_eq!(props["TimeToLiveSpecification"]["Enabled"], false);
    assert!(props["Tags"].as_array().unwrap().is_empty());
    assert_eq!(props["WarmThroughput"]["ReadUnitsPerSecond"], 12000);
    assert_eq!(props["WarmThroughput"]["WriteUnitsPerSecond"], 4000);
}

// ---------------------------------------------------------------------------
// CFN resource-type shaping: AWS::ElasticLoadBalancingV2::TargetGroup (#9)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_get_resource_elbv2_target_group_applies_cfn_schema() {
    // Real Cloud Control returns TargetGroupArn (the primaryIdentifier) as the
    // create identifier, synthesises TargetGroupArn / TargetGroupName /
    // TargetGroupFullName / LoadBalancerArns, and fills every health-check
    // default plus the 14-entry TargetGroupAttributes block.
    // Regression for https://github.com/moriyoshi/winterbaume/issues/9.
    let client = make_client().await;

    let desired_state = r#"{
        "Name": "wb-probe-tg",
        "Protocol": "HTTP",
        "Port": 8080,
        "VpcId": "vpc-085f4b7505f74650d",
        "TargetType": "ip",
        "HealthCheckPath": "/health",
        "Tags": [{"Key": "Environment", "Value": "probe"}]
    }"#;

    let create_resp = client
        .create_resource()
        .type_name("AWS::ElasticLoadBalancingV2::TargetGroup")
        .desired_state(desired_state)
        .send()
        .await
        .expect("create_resource should succeed");

    // primaryIdentifier must be TargetGroupArn, not Name.
    let identifier = create_resp
        .progress_event()
        .and_then(|e| e.identifier())
        .expect("create should return primary identifier")
        .to_string();
    assert!(
        identifier.starts_with("arn:aws:elasticloadbalancing:")
            && identifier.contains(":targetgroup/wb-probe-tg/"),
        "identifier must be the TargetGroupArn (primaryIdentifier), not Name; got {identifier}"
    );

    let get_resp = client
        .get_resource()
        .type_name("AWS::ElasticLoadBalancingV2::TargetGroup")
        .identifier(&identifier)
        .send()
        .await
        .expect("get_resource by TargetGroupArn should succeed");

    let props: serde_json::Value = serde_json::from_str(
        get_resp
            .resource_description()
            .and_then(|d| d.properties())
            .expect("should have properties"),
    )
    .expect("properties should be valid JSON");

    // readOnly properties synthesised.
    let arn = props["TargetGroupArn"]
        .as_str()
        .expect("TargetGroupArn must be present");
    assert!(arn.starts_with("arn:aws:elasticloadbalancing:"), "{arn}");
    assert!(arn.contains(":targetgroup/wb-probe-tg/"), "{arn}");
    assert_eq!(props["TargetGroupName"], "wb-probe-tg");
    let full_name = props["TargetGroupFullName"]
        .as_str()
        .expect("TargetGroupFullName must be present");
    assert!(
        full_name.starts_with("targetgroup/wb-probe-tg/"),
        "{full_name}"
    );
    assert!(props["LoadBalancerArns"].as_array().unwrap().is_empty());

    // Pass-through.
    assert_eq!(props["Name"], "wb-probe-tg");
    assert_eq!(props["Protocol"], "HTTP");
    assert_eq!(props["Port"], 8080);
    assert_eq!(props["VpcId"], "vpc-085f4b7505f74650d");
    assert_eq!(props["TargetType"], "ip");
    assert_eq!(props["HealthCheckPath"], "/health");
    assert_eq!(props["Tags"][0]["Key"], "Environment");

    // Read-handler defaults.
    assert_eq!(props["IpAddressType"], "ipv4");
    assert_eq!(props["HealthCheckEnabled"], true);
    assert_eq!(props["HealthCheckIntervalSeconds"], 30);
    assert_eq!(props["HealthCheckProtocol"], "HTTP");
    assert_eq!(props["HealthCheckPort"], "traffic-port");
    assert_eq!(props["HealthCheckTimeoutSeconds"], 5);
    assert_eq!(props["HealthyThresholdCount"], 5);
    assert_eq!(props["UnhealthyThresholdCount"], 2);
    assert_eq!(props["ProtocolVersion"], "HTTP1");
    assert_eq!(props["Matcher"]["HttpCode"], "200");
    assert!(props["Targets"].as_array().unwrap().is_empty());
    let attrs = props["TargetGroupAttributes"]
        .as_array()
        .expect("TargetGroupAttributes must be an array");
    assert_eq!(
        attrs.len(),
        14,
        "expected 14 attribute defaults; got {}",
        attrs.len()
    );
}

// ---------------------------------------------------------------------------
// CFN resource-type shaping: AWS::ElasticLoadBalancingV2::LoadBalancer (#10)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_get_resource_elbv2_load_balancer_applies_cfn_schema() {
    // primaryIdentifier is LoadBalancerArn; readOnly properties (Arn, DNSName,
    // CanonicalHostedZoneID, LoadBalancerName, LoadBalancerFullName) are
    // synthesised; SubnetMappings is derived from Subnets; the 22-entry
    // LoadBalancerAttributes block is filled.
    // Regression for https://github.com/moriyoshi/winterbaume/issues/10.
    let client = make_client().await;

    let desired_state = r#"{
        "Name": "wb-probe-alb",
        "Type": "application",
        "Scheme": "internet-facing",
        "Subnets": ["subnet-aaa", "subnet-bbb"],
        "SecurityGroups": ["sg-ccc"],
        "Tags": [{"Key": "Environment", "Value": "probe"}]
    }"#;

    let create_resp = client
        .create_resource()
        .type_name("AWS::ElasticLoadBalancingV2::LoadBalancer")
        .desired_state(desired_state)
        .send()
        .await
        .expect("create_resource should succeed");

    let identifier = create_resp
        .progress_event()
        .and_then(|e| e.identifier())
        .expect("create should return primary identifier")
        .to_string();
    assert!(
        identifier.starts_with("arn:aws:elasticloadbalancing:")
            && identifier.contains(":loadbalancer/app/wb-probe-alb/"),
        "identifier must be the LoadBalancerArn, not Name; got {identifier}"
    );

    let get_resp = client
        .get_resource()
        .type_name("AWS::ElasticLoadBalancingV2::LoadBalancer")
        .identifier(&identifier)
        .send()
        .await
        .expect("get_resource by LoadBalancerArn should succeed");

    let props: serde_json::Value = serde_json::from_str(
        get_resp
            .resource_description()
            .and_then(|d| d.properties())
            .expect("should have properties"),
    )
    .expect("properties should be valid JSON");

    // readOnly properties.
    let arn = props["LoadBalancerArn"].as_str().expect("LoadBalancerArn");
    assert!(arn.starts_with("arn:aws:elasticloadbalancing:"), "{arn}");
    assert!(arn.contains(":loadbalancer/app/wb-probe-alb/"), "{arn}");
    assert_eq!(props["LoadBalancerName"], "wb-probe-alb");
    let full_name = props["LoadBalancerFullName"]
        .as_str()
        .expect("LoadBalancerFullName");
    assert!(full_name.starts_with("app/wb-probe-alb/"), "{full_name}");
    let dns = props["DNSName"].as_str().expect("DNSName");
    assert!(dns.starts_with("wb-probe-alb-"), "{dns}");
    assert!(dns.ends_with(".us-east-1.elb.amazonaws.com"), "{dns}");
    assert!(
        props["CanonicalHostedZoneID"]
            .as_str()
            .unwrap_or("")
            .starts_with('Z')
    );

    // Pass-through.
    assert_eq!(props["Type"], "application");
    assert_eq!(props["Scheme"], "internet-facing");
    assert_eq!(props["Subnets"][0], "subnet-aaa");
    assert_eq!(props["SecurityGroups"][0], "sg-ccc");
    assert_eq!(props["Tags"][0]["Value"], "probe");

    // Read-handler defaults.
    assert_eq!(props["IpAddressType"], "ipv4");
    assert_eq!(props["EnablePrefixForIpv6SourceNat"], "off");
    let attrs = props["LoadBalancerAttributes"]
        .as_array()
        .expect("LoadBalancerAttributes must be an array");
    assert_eq!(
        attrs.len(),
        22,
        "expected 22 attribute defaults; got {}",
        attrs.len()
    );

    // SubnetMappings derived from Subnets.
    let mappings = props["SubnetMappings"]
        .as_array()
        .expect("SubnetMappings must be an array");
    assert_eq!(mappings.len(), 2);
    assert_eq!(mappings[0]["SubnetId"], "subnet-aaa");
    assert_eq!(mappings[1]["SubnetId"], "subnet-bbb");
}

// ---------------------------------------------------------------------------
// CFN resource-type shaping: AWS::ECS::Cluster (regression for issue #8)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_get_resource_ecs_cluster_applies_cfn_schema() {
    // Real Cloud Control uses ClusterName as the primaryIdentifier, generates
    // Arn, and the read handler fills in DefaultCapacityProviderStrategy: [].
    // Regression for https://github.com/moriyoshi/winterbaume/issues/8.
    let client = make_client().await;

    let desired_state = r#"{
        "ClusterName": "wb-probe-ecs",
        "CapacityProviders": ["FARGATE", "FARGATE_SPOT"],
        "ClusterSettings": [{"Name": "containerInsights", "Value": "enabled"}],
        "Tags": [{"Key": "Environment", "Value": "probe"}],
        "ServiceConnectDefaults": {"Namespace": "ignored"}
    }"#;

    let create_resp = client
        .create_resource()
        .type_name("AWS::ECS::Cluster")
        .desired_state(desired_state)
        .send()
        .await
        .expect("create_resource should succeed");

    // primaryIdentifier: ClusterName, NOT a UUID.
    let cluster_name = create_resp
        .progress_event()
        .and_then(|e| e.identifier())
        .expect("create should return primary identifier")
        .to_string();
    assert_eq!(
        cluster_name, "wb-probe-ecs",
        "identifier must be ClusterName (the schema's primaryIdentifier), not a random UUID"
    );

    let get_resp = client
        .get_resource()
        .type_name("AWS::ECS::Cluster")
        .identifier(&cluster_name)
        .send()
        .await
        .expect("get_resource by ClusterName should succeed");

    let props: serde_json::Value = serde_json::from_str(
        get_resp
            .resource_description()
            .and_then(|d| d.properties())
            .expect("should have properties"),
    )
    .expect("properties should be valid JSON");

    // writeOnly: ServiceConnectDefaults must be stripped.
    assert!(
        props.get("ServiceConnectDefaults").is_none(),
        "ServiceConnectDefaults is writeOnly and must not appear in GetResource output"
    );

    // readOnly: Arn generated by the service.
    let arn = props
        .get("Arn")
        .and_then(|v| v.as_str())
        .expect("Arn must be present");
    assert!(arn.starts_with("arn:aws:ecs:"), "got Arn={arn}");
    assert!(arn.ends_with(":cluster/wb-probe-ecs"), "got Arn={arn}");

    // Pass-through properties survive unchanged.
    assert_eq!(props["ClusterName"], "wb-probe-ecs");
    assert_eq!(props["CapacityProviders"][0], "FARGATE");
    assert_eq!(props["CapacityProviders"][1], "FARGATE_SPOT");
    assert_eq!(props["ClusterSettings"][0]["Name"], "containerInsights");
    assert_eq!(props["ClusterSettings"][0]["Value"], "enabled");
    assert_eq!(props["Tags"][0]["Key"], "Environment");
    assert_eq!(props["Tags"][0]["Value"], "probe");

    // Schema default reported by the read handler.
    assert!(
        props["DefaultCapacityProviderStrategy"]
            .as_array()
            .unwrap()
            .is_empty()
    );
}

// ---------------------------------------------------------------------------
// State views: snapshot, restore, merge
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_state_snapshot_restore() {
    use winterbaume_cloudcontrol::views::{CloudControlStateView, ResourceView};
    use winterbaume_core::StatefulService;

    let svc = make_service();

    // Seed state via restore
    let mut view = CloudControlStateView::default();
    view.resources.insert(
        "AWS::S3::Bucket|snap-bucket".to_string(),
        ResourceView {
            type_name: "AWS::S3::Bucket".to_string(),
            identifier: "snap-bucket".to_string(),
            resource_model: r#"{"BucketName":"snap-bucket"}"#.to_string(),
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    // Snapshot
    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert!(snap.resources.contains_key("AWS::S3::Bucket|snap-bucket"));

    // Restore to empty
    svc.restore("123456789012", "us-east-1", Default::default())
        .await
        .unwrap();
    let empty = svc.snapshot("123456789012", "us-east-1").await;
    assert!(empty.resources.is_empty());

    // Restore from snapshot
    svc.restore("123456789012", "us-east-1", snap)
        .await
        .unwrap();
    let restored = svc.snapshot("123456789012", "us-east-1").await;
    assert!(
        restored
            .resources
            .contains_key("AWS::S3::Bucket|snap-bucket")
    );
}

#[tokio::test]
async fn test_state_merge_additive() {
    use winterbaume_cloudcontrol::views::{CloudControlStateView, ResourceView};
    use winterbaume_core::StatefulService;

    let svc = make_service();

    // Seed existing resource
    let mut initial = CloudControlStateView::default();
    initial.resources.insert(
        "AWS::S3::Bucket|existing-bucket".to_string(),
        ResourceView {
            type_name: "AWS::S3::Bucket".to_string(),
            identifier: "existing-bucket".to_string(),
            resource_model: r#"{"BucketName":"existing-bucket"}"#.to_string(),
        },
    );
    svc.restore("123456789012", "us-east-1", initial)
        .await
        .unwrap();

    // Merge a new resource
    let mut merge_view = CloudControlStateView::default();
    merge_view.resources.insert(
        "AWS::Lambda::Function|merged-fn".to_string(),
        ResourceView {
            type_name: "AWS::Lambda::Function".to_string(),
            identifier: "merged-fn".to_string(),
            resource_model: r#"{"FunctionName":"merged-fn"}"#.to_string(),
        },
    );
    svc.merge("123456789012", "us-east-1", merge_view)
        .await
        .unwrap();

    // Both should exist
    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert!(
        snap.resources
            .contains_key("AWS::S3::Bucket|existing-bucket")
    );
    assert!(
        snap.resources
            .contains_key("AWS::Lambda::Function|merged-fn")
    );
}

// ---------------------------------------------------------------------------
// State change notifications
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;

    let svc = CloudControlService::new();
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

    use winterbaume_cloudcontrol::views::{CloudControlStateView, ResourceView};
    use winterbaume_core::StatefulService;

    let svc = CloudControlService::new();

    // Pre-seed with a resource
    let mut view = CloudControlStateView::default();
    view.resources.insert(
        "AWS::S3::Bucket|notify-bucket".to_string(),
        ResourceView {
            type_name: "AWS::S3::Bucket".to_string(),
            identifier: "notify-bucket".to_string(),
            resource_model: r#"{"BucketName":"notify-bucket"}"#.to_string(),
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    // Subscribe and capture snapshot
    let snapshots: Arc<Mutex<Vec<CloudControlStateView>>> = Arc::new(Mutex::new(vec![]));
    let snapshots2 = Arc::clone(&snapshots);
    svc.notifier().subscribe(move |_account_id, _region, view| {
        snapshots2.lock().unwrap().push(view.clone());
    });

    // Merge another resource
    let mut view2 = CloudControlStateView::default();
    view2.resources.insert(
        "AWS::Lambda::Function|notify-fn".to_string(),
        ResourceView {
            type_name: "AWS::Lambda::Function".to_string(),
            identifier: "notify-fn".to_string(),
            resource_model: r#"{"FunctionName":"notify-fn"}"#.to_string(),
        },
    );
    svc.merge("123456789012", "us-east-1", view2).await.unwrap();

    let got = snapshots.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert!(
        got[0]
            .resources
            .contains_key("AWS::Lambda::Function|notify-fn")
    );
    assert!(
        got[0]
            .resources
            .contains_key("AWS::S3::Bucket|notify-bucket")
    );
}
