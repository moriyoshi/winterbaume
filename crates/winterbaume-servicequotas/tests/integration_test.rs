use aws_sdk_servicequotas::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_servicequotas::ServiceQuotasService;

async fn make_client() -> aws_sdk_servicequotas::Client {
    let mock = MockAws::builder()
        .with_service(ServiceQuotasService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_servicequotas::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_servicequotas::Client::new(&config)
}

#[tokio::test]
async fn test_get_service_quota() {
    let client = make_client().await;

    let resp = client
        .get_service_quota()
        .service_code("ec2")
        .quota_code("L-1216C47A")
        .send()
        .await
        .expect("get_service_quota should succeed");

    let quota = resp.quota().expect("should have quota");
    assert_eq!(quota.service_code().unwrap(), "ec2");
    assert_eq!(quota.quota_code().unwrap(), "L-1216C47A");
    assert_eq!(
        quota.quota_name().unwrap(),
        "Running On-Demand Standard (A, C, D, H, I, M, R, T, Z) instances"
    );
    assert_eq!(quota.value().unwrap(), 1152.0);
}

#[tokio::test]
async fn test_get_service_quota_not_found() {
    let client = make_client().await;

    let result = client
        .get_service_quota()
        .service_code("ec2")
        .quota_code("L-NONEXISTENT")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_service_quotas() {
    let client = make_client().await;

    let resp = client
        .list_service_quotas()
        .service_code("ec2")
        .send()
        .await
        .expect("list_service_quotas should succeed");

    let quotas = resp.quotas();
    assert!(!quotas.is_empty());
    assert!(quotas.iter().all(|q| q.service_code().unwrap() == "ec2"));
}

#[tokio::test]
async fn test_get_aws_default_service_quota() {
    let client = make_client().await;

    let resp = client
        .get_aws_default_service_quota()
        .service_code("s3")
        .quota_code("L-DC2B2D3D")
        .send()
        .await
        .expect("get_aws_default_service_quota should succeed");

    let quota = resp.quota().expect("should have quota");
    assert_eq!(quota.service_code().unwrap(), "s3");
    assert_eq!(quota.quota_code().unwrap(), "L-DC2B2D3D");
    assert_eq!(quota.quota_name().unwrap(), "Buckets");
    assert_eq!(quota.value().unwrap(), 100.0);
}

#[tokio::test]
async fn test_list_services() {
    let client = make_client().await;

    let resp = client
        .list_services()
        .send()
        .await
        .expect("list_services should succeed");

    let services = resp.services();
    assert!(!services.is_empty());

    // Check that at least some well-known services are present
    let codes: Vec<&str> = services.iter().filter_map(|s| s.service_code()).collect();
    assert!(codes.contains(&"ec2"));
    assert!(codes.contains(&"s3"));
    assert!(codes.contains(&"lambda"));
}

#[tokio::test]
async fn test_list_service_quotas_for_unknown_service() {
    let client = make_client().await;

    // Should still succeed, returning a default quota
    let resp = client
        .list_service_quotas()
        .service_code("unknown-service")
        .send()
        .await
        .expect("list_service_quotas for unknown service should succeed");

    let quotas = resp.quotas();
    assert!(!quotas.is_empty());
}

// ============================================================================
// Tests derived from AWS documentation: Service Quotas
// ============================================================================

#[tokio::test]
async fn test_get_service_quota_missing_quota_code() {
    // Sending an empty quota_code should return an error
    let client = make_client().await;

    let result = client
        .get_service_quota()
        .service_code("ec2")
        .quota_code("")
        .send()
        .await;

    // Empty quota_code is not in the quota store — expect not-found or illegal-argument
    assert!(result.is_err(), "empty quota_code should return an error");
}

#[tokio::test]
async fn test_get_service_quota_quota_arn_format() {
    let client = make_client().await;

    let resp = client
        .get_service_quota()
        .service_code("ec2")
        .quota_code("L-1216C47A")
        .send()
        .await
        .expect("get_service_quota should succeed");

    let quota = resp.quota().expect("should have quota");
    let arn = quota.quota_arn().unwrap_or_default();
    assert!(!arn.is_empty(), "quota_arn should be non-empty");
    assert!(
        arn.contains("ec2"),
        "quota_arn should contain service code: {arn}"
    );
    assert!(
        arn.contains("L-1216C47A"),
        "quota_arn should contain quota code: {arn}"
    );
    assert!(
        arn.starts_with("arn:aws:servicequotas:"),
        "quota_arn should start with arn:aws:servicequotas: — got: {arn}"
    );
}

#[tokio::test]
async fn test_get_service_quota_fields() {
    let client = make_client().await;

    let resp = client
        .get_service_quota()
        .service_code("ec2")
        .quota_code("L-1216C47A")
        .send()
        .await
        .expect("get_service_quota should succeed");

    let quota = resp.quota().expect("should have quota");
    // service_name is present
    assert!(
        quota.service_name().is_some(),
        "service_name should be present"
    );
    // unit is present
    assert!(quota.unit().is_some(), "unit should be present");
    // adjustable is a bool (always present)
    let _ = quota.adjustable();
    // global_quota is a bool (always present)
    let _ = quota.global_quota();
    // description is present
    assert!(
        quota.description().is_some(),
        "description should be present"
    );
    // value is present and positive
    let value = quota.value().unwrap_or(0.0);
    assert!(value > 0.0, "quota value should be positive");
}

#[tokio::test]
async fn test_list_service_quotas_s3() {
    let client = make_client().await;

    let resp = client
        .list_service_quotas()
        .service_code("s3")
        .send()
        .await
        .expect("list_service_quotas for s3 should succeed");

    let quotas = resp.quotas();
    assert!(!quotas.is_empty(), "s3 quotas should be non-empty");
    assert!(
        quotas.iter().all(|q| q.service_code() == Some("s3")),
        "all quotas should have service_code=s3"
    );
}

#[tokio::test]
async fn test_list_service_quotas_lambda() {
    let client = make_client().await;

    let resp = client
        .list_service_quotas()
        .service_code("lambda")
        .send()
        .await
        .expect("list_service_quotas for lambda should succeed");

    let quotas = resp.quotas();
    assert!(!quotas.is_empty(), "lambda quotas should be non-empty");
    assert!(
        quotas.iter().all(|q| q.service_code() == Some("lambda")),
        "all quotas should have service_code=lambda"
    );
}

#[tokio::test]
async fn test_list_service_quotas_quota_fields() {
    let client = make_client().await;

    let resp = client
        .list_service_quotas()
        .service_code("ec2")
        .send()
        .await
        .expect("list_service_quotas should succeed");

    let quotas = resp.quotas();
    assert!(!quotas.is_empty());

    for q in quotas {
        assert!(q.quota_code().is_some(), "quota_code should be present");
        assert!(q.quota_name().is_some(), "quota_name should be present");
        assert!(q.value().is_some(), "value should be present");
        assert!(q.unit().is_some(), "unit should be present");
        let _ = q.adjustable(); // bool, always present
    }
}

#[tokio::test]
async fn test_get_aws_default_service_quota_not_found() {
    let client = make_client().await;

    let result = client
        .get_aws_default_service_quota()
        .service_code("s3")
        .quota_code("L-NONEXISTENT")
        .send()
        .await;

    assert!(result.is_err(), "nonexistent quota should return an error");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("NoSuchResourceException") || err_str.contains("NoSuchResource"),
        "expected NoSuchResourceException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_aws_default_service_quota_lambda() {
    let client = make_client().await;

    let resp = client
        .get_aws_default_service_quota()
        .service_code("lambda")
        .quota_code("L-B99A9384")
        .send()
        .await
        .expect("get_aws_default_service_quota for lambda should succeed");

    let quota = resp.quota().expect("should have quota");
    assert_eq!(quota.service_code(), Some("lambda"));
    assert_eq!(quota.quota_code(), Some("L-B99A9384"));
    assert_eq!(quota.quota_name(), Some("Concurrent executions"));
    assert_eq!(quota.value(), Some(1000.0));
}

#[tokio::test]
async fn test_list_services_service_fields() {
    let client = make_client().await;

    let resp = client
        .list_services()
        .send()
        .await
        .expect("list_services should succeed");

    let services = resp.services();
    assert!(!services.is_empty());

    for s in services {
        assert!(s.service_code().is_some(), "service_code should be present");
        assert!(s.service_name().is_some(), "service_name should be present");
        let code = s.service_code().unwrap_or_default();
        assert!(!code.is_empty(), "service_code should not be empty");
    }
}

#[tokio::test]
async fn test_list_services_contains_dynamodb() {
    let client = make_client().await;

    let resp = client
        .list_services()
        .send()
        .await
        .expect("list_services should succeed");

    let codes: Vec<&str> = resp
        .services()
        .iter()
        .filter_map(|s| s.service_code())
        .collect();

    assert!(
        codes.contains(&"dynamodb"),
        "dynamodb should be in services list"
    );
    assert!(codes.contains(&"iam"), "iam should be in services list");
    assert!(codes.contains(&"sqs"), "sqs should be in services list");
}

#[tokio::test]
async fn test_list_aws_default_service_quotas() {
    let client = make_client().await;

    let resp = client
        .list_aws_default_service_quotas()
        .service_code("ec2")
        .send()
        .await
        .expect("list_aws_default_service_quotas should succeed");

    let quotas = resp.quotas();
    assert!(!quotas.is_empty(), "ec2 default quotas should be non-empty");
    assert!(
        quotas.iter().all(|q| q.service_code() == Some("ec2")),
        "all default quotas should have service_code=ec2"
    );
}

#[tokio::test]
async fn test_list_aws_default_service_quotas_s3() {
    let client = make_client().await;

    let resp = client
        .list_aws_default_service_quotas()
        .service_code("s3")
        .send()
        .await
        .expect("list_aws_default_service_quotas for s3 should succeed");

    let quotas = resp.quotas();
    assert!(!quotas.is_empty(), "s3 default quotas should be non-empty");
    assert!(
        quotas.iter().all(|q| q.service_code() == Some("s3")),
        "all default quotas should have service_code=s3"
    );
}

#[tokio::test]
async fn test_list_aws_default_service_quotas_quota_fields() {
    let client = make_client().await;

    let resp = client
        .list_aws_default_service_quotas()
        .service_code("lambda")
        .send()
        .await
        .expect("list_aws_default_service_quotas should succeed");

    let quotas = resp.quotas();
    assert!(!quotas.is_empty());

    for q in quotas {
        assert!(q.quota_code().is_some(), "quota_code should be present");
        assert!(q.quota_name().is_some(), "quota_name should be present");
        assert!(q.value().is_some(), "value should be present");
    }
}

#[tokio::test]
async fn test_get_service_quota_and_default_agree() {
    let client = make_client().await;

    let applied = client
        .get_service_quota()
        .service_code("s3")
        .quota_code("L-DC2B2D3D")
        .send()
        .await
        .expect("get_service_quota should succeed");

    let default = client
        .get_aws_default_service_quota()
        .service_code("s3")
        .quota_code("L-DC2B2D3D")
        .send()
        .await
        .expect("get_aws_default_service_quota should succeed");

    let applied_quota = applied.quota().expect("applied should have quota");
    let default_quota = default.quota().expect("default should have quota");

    assert_eq!(
        applied_quota.quota_code(),
        default_quota.quota_code(),
        "quota_code should match"
    );
    assert_eq!(
        applied_quota.value(),
        default_quota.value(),
        "value should match between applied and default"
    );
}

#[tokio::test]
async fn test_list_and_get_quota_consistency() {
    let client = make_client().await;

    // Get the list of ec2 quotas and look up the first one individually
    let list_resp = client
        .list_service_quotas()
        .service_code("ec2")
        .send()
        .await
        .expect("list_service_quotas should succeed");

    let quotas = list_resp.quotas();
    assert!(!quotas.is_empty());

    let first = &quotas[0];
    let quota_code = first.quota_code().expect("quota_code should be present");
    let list_value = first.value().expect("value should be present");

    // Now fetch the same quota individually
    let get_resp = client
        .get_service_quota()
        .service_code("ec2")
        .quota_code(quota_code)
        .send()
        .await
        .expect("get_service_quota should succeed");

    let get_quota = get_resp.quota().expect("should have quota");
    assert_eq!(
        get_quota.quota_code(),
        Some(quota_code),
        "quota_code should match"
    );
    assert_eq!(
        get_quota.value(),
        Some(list_value),
        "value should be consistent between list and get"
    );
}
