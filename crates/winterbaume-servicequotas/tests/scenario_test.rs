/// Scenario tests for winterbaume-servicequotas.
///
/// These tests chain 3+ operations to verify end-to-end use-case correctness
/// rather than per-API shape fidelity (which is covered in integration_test.rs).
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

/// Scenario: Quota discovery workflow
///
/// A user discovers which services are available, browses the quotas for a
/// service, then retrieves detailed information about a specific quota.
/// Operations: ListServices → ListServiceQuotas → GetServiceQuota
#[tokio::test]
async fn test_quota_discovery_workflow() {
    let client = make_client().await;

    // Step 1: List available services
    let services_resp = client
        .list_services()
        .send()
        .await
        .expect("list_services should succeed");

    let services = services_resp.services();
    assert!(!services.is_empty(), "should have at least one service");

    // Find ec2 in the list
    let ec2_service = services
        .iter()
        .find(|s| s.service_code() == Some("ec2"))
        .expect("ec2 should be in the service list");
    let service_code = ec2_service.service_code().unwrap();

    // Step 2: List quotas for that service
    let quotas_resp = client
        .list_service_quotas()
        .service_code(service_code)
        .send()
        .await
        .expect("list_service_quotas should succeed");

    let quotas = quotas_resp.quotas();
    assert!(!quotas.is_empty(), "ec2 should have at least one quota");

    // Step 3: Get details for the first quota found
    let first_quota = &quotas[0];
    let quota_code = first_quota
        .quota_code()
        .expect("quota_code must be present");

    let detail_resp = client
        .get_service_quota()
        .service_code(service_code)
        .quota_code(quota_code)
        .send()
        .await
        .expect("get_service_quota should succeed");

    let detail = detail_resp.quota().expect("quota detail must be present");
    assert_eq!(
        detail.service_code(),
        Some(service_code),
        "service_code should be consistent"
    );
    assert_eq!(
        detail.quota_code(),
        Some(quota_code),
        "quota_code should be consistent"
    );
    assert_eq!(
        detail.value(),
        first_quota.value(),
        "quota value should be consistent between list and get"
    );
}

/// Scenario: Default quota comparison workflow
///
/// A user compares the default AWS quota against the applied quota for a
/// service, verifying that both return the same information when no custom
/// value has been requested.
/// Operations: GetAWSDefaultServiceQuota → GetServiceQuota → comparison
#[tokio::test]
async fn test_default_vs_applied_quota_comparison() {
    let client = make_client().await;

    // Step 1: Get default quota for s3 buckets
    let default_resp = client
        .get_aws_default_service_quota()
        .service_code("s3")
        .quota_code("L-DC2B2D3D")
        .send()
        .await
        .expect("get_aws_default_service_quota should succeed");

    let default_quota = default_resp.quota().expect("default quota must be present");
    let default_value = default_quota
        .value()
        .expect("default value must be present");
    let default_name = default_quota
        .quota_name()
        .expect("quota_name must be present");

    // Step 2: Get applied quota for s3 buckets
    let applied_resp = client
        .get_service_quota()
        .service_code("s3")
        .quota_code("L-DC2B2D3D")
        .send()
        .await
        .expect("get_service_quota should succeed");

    let applied_quota = applied_resp.quota().expect("applied quota must be present");

    // Step 3: Assert they agree on all key fields
    assert_eq!(
        applied_quota.value(),
        Some(default_value),
        "applied and default quota values should agree"
    );
    assert_eq!(
        applied_quota.quota_name(),
        Some(default_name),
        "applied and default quota names should agree"
    );
    assert_eq!(
        applied_quota.service_code(),
        Some("s3"),
        "service_code should be s3"
    );
    assert!(
        applied_quota
            .quota_arn()
            .unwrap_or_default()
            .starts_with("arn:aws:servicequotas:"),
        "quota_arn should be a valid ARN"
    );
}

/// Scenario: Cross-service quota survey
///
/// A user checks the same category of resource limits across multiple services,
/// collecting the quota values to build a capacity summary.
/// Operations: ListAWSDefaultServiceQuotas(ec2) → ListAWSDefaultServiceQuotas(s3)
///             → ListAWSDefaultServiceQuotas(lambda) → comparison
#[tokio::test]
async fn test_cross_service_quota_survey() {
    let client = make_client().await;

    let services = ["ec2", "s3", "lambda"];
    let mut quota_counts = Vec::new();

    for service_code in &services {
        // Step N: List default quotas for each service
        let resp = client
            .list_aws_default_service_quotas()
            .service_code(*service_code)
            .send()
            .await
            .unwrap_or_else(|_| {
                panic!("list_aws_default_service_quotas for {service_code} should succeed")
            });

        let quotas = resp.quotas();
        assert!(
            !quotas.is_empty(),
            "{service_code} should have at least one default quota"
        );
        quota_counts.push(quotas.len());

        // Verify each quota is well-formed
        for q in quotas {
            assert_eq!(
                q.service_code(),
                Some(*service_code),
                "all quotas should belong to {service_code}"
            );
            assert!(
                q.quota_code().is_some(),
                "quota_code must be present for {service_code}"
            );
            assert!(
                q.value().unwrap_or(0.0) > 0.0,
                "quota value should be positive for {service_code}"
            );
        }
    }

    // All three services returned quotas
    assert_eq!(
        quota_counts.len(),
        3,
        "should have surveyed all three services"
    );
    assert!(
        quota_counts.iter().all(|&c| c > 0),
        "every service should have at least one quota"
    );
}
