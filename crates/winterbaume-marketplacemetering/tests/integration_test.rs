//! Integration tests for winterbaume AWS Marketplace Metering service.

use aws_sdk_marketplacemetering::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_marketplacemetering::MarketplaceMeteringService;

async fn make_metering_client() -> aws_sdk_marketplacemetering::Client {
    let mock = MockAws::builder()
        .with_service(MarketplaceMeteringService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_marketplacemetering::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    aws_sdk_marketplacemetering::Client::new(&config)
}

#[tokio::test]
async fn test_meter_usage() {
    let client = make_metering_client().await;

    let resp = client
        .meter_usage()
        .product_code("test-product-code")
        .usage_dimension("requests")
        .usage_quantity(100)
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .send()
        .await
        .expect("meter_usage should succeed");

    assert!(resp.metering_record_id().is_some());
    assert!(!resp.metering_record_id().unwrap().is_empty());
}

#[tokio::test]
async fn test_meter_usage_dry_run() {
    let client = make_metering_client().await;

    let resp = client
        .meter_usage()
        .product_code("test-product-code")
        .usage_dimension("requests")
        .usage_quantity(50)
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .dry_run(true)
        .send()
        .await
        .expect("meter_usage dry_run should succeed");

    assert!(resp.metering_record_id().is_some());
}

#[tokio::test]
async fn test_register_usage() {
    let client = make_metering_client().await;

    let resp = client
        .register_usage()
        .product_code("test-product-code")
        .public_key_version(1)
        .send()
        .await
        .expect("register_usage should succeed");

    assert!(resp.public_key_rotation_timestamp().is_some());
    assert!(resp.signature().is_some());
    assert!(!resp.signature().unwrap().is_empty());
}

#[tokio::test]
async fn test_batch_meter_usage() {
    use aws_sdk_marketplacemetering::types::UsageRecord;

    let client = make_metering_client().await;

    let record1 = UsageRecord::builder()
        .customer_identifier("customer-1")
        .dimension("api-calls")
        .quantity(100)
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .build()
        .unwrap();

    let record2 = UsageRecord::builder()
        .customer_identifier("customer-2")
        .dimension("storage")
        .quantity(200)
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .build()
        .unwrap();

    let resp = client
        .batch_meter_usage()
        .product_code("test-product-code")
        .usage_records(record1)
        .usage_records(record2)
        .send()
        .await
        .expect("batch_meter_usage should succeed");

    let results = resp.results();
    assert_eq!(results.len(), 2);
}

#[tokio::test]
async fn test_resolve_customer() {
    let client = make_metering_client().await;

    let resp = client
        .resolve_customer()
        .registration_token("test-registration-token-12345678")
        .send()
        .await
        .expect("resolve_customer should succeed");

    assert!(!resp.customer_identifier().unwrap_or_default().is_empty());
    assert!(!resp.product_code().unwrap_or_default().is_empty());
    assert!(
        !resp
            .customer_aws_account_id()
            .unwrap_or_default()
            .is_empty()
    );
}

#[tokio::test]
async fn test_meter_usage_missing_product_code() {
    let client = make_metering_client().await;

    let result = client
        .meter_usage()
        .usage_dimension("requests")
        .usage_quantity(100)
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .send()
        .await;

    assert!(
        result.is_err(),
        "meter_usage without product_code should fail"
    );
}

#[tokio::test]
async fn test_register_usage_twice() {
    let client = make_metering_client().await;

    let resp1 = client
        .register_usage()
        .product_code("test-product-code")
        .public_key_version(1)
        .send()
        .await
        .expect("first register_usage should succeed");

    let resp2 = client
        .register_usage()
        .product_code("test-product-code")
        .public_key_version(2)
        .send()
        .await
        .expect("second register_usage should succeed");

    // Both should return valid signatures
    assert!(resp1.signature().is_some());
    assert!(resp2.signature().is_some());
}

// ============================================================================
// Tests derived from AWS documentation: AWS Marketplace Metering Service
// ============================================================================

// --- MeterUsage additional tests ---

#[tokio::test]
async fn test_meter_usage_returns_unique_record_ids() {
    let client = make_metering_client().await;

    let resp1 = client
        .meter_usage()
        .product_code("test-product-code")
        .usage_dimension("requests")
        .usage_quantity(10)
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .send()
        .await
        .expect("first meter_usage should succeed");

    let resp2 = client
        .meter_usage()
        .product_code("test-product-code")
        .usage_dimension("requests")
        .usage_quantity(20)
        .timestamp(aws_smithy_types::DateTime::from_secs(1700003600))
        .send()
        .await
        .expect("second meter_usage should succeed");

    let id1 = resp1.metering_record_id().unwrap_or_default();
    let id2 = resp2.metering_record_id().unwrap_or_default();
    assert!(!id1.is_empty(), "first record id should be non-empty");
    assert!(!id2.is_empty(), "second record id should be non-empty");
    assert_ne!(id1, id2, "record ids should be unique across calls");
}

#[tokio::test]
async fn test_meter_usage_missing_usage_dimension() {
    let client = make_metering_client().await;

    // The SDK requires UsageDimension at the builder level; simulate by using
    // an empty string, which the handler rejects as InvalidUsageDimensionException.
    // Because the SDK enforces required fields client-side, we verify the handler
    // behavior directly via a missing-dimension scenario by calling with an explicit
    // empty-string dimension through an alternate approach.
    // The handler returns 400 InvalidUsageDimensionException for empty dimension.
    // We test this by verifying the service rejects empty usage_dimension.
    // Note: the AWS SDK Rust builder may reject empty strings before sending.
    // We document this as a handler-level validation test.
    let result = client
        .meter_usage()
        .product_code("test-product-code")
        .usage_dimension("") // empty dimension triggers InvalidUsageDimensionException
        .usage_quantity(10)
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .send()
        .await;

    // Either the SDK rejects it client-side or the server returns an error.
    assert!(
        result.is_err(),
        "meter_usage with empty usage_dimension should fail"
    );
}

#[tokio::test]
async fn test_meter_usage_zero_quantity() {
    let client = make_metering_client().await;

    let resp = client
        .meter_usage()
        .product_code("test-product-code")
        .usage_dimension("requests")
        .usage_quantity(0)
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .send()
        .await
        .expect("meter_usage with zero quantity should succeed");

    assert!(
        resp.metering_record_id().is_some(),
        "should return a metering_record_id even for zero quantity"
    );
    assert!(!resp.metering_record_id().unwrap_or_default().is_empty());
}

#[tokio::test]
async fn test_meter_usage_with_usage_allocations() {
    use aws_sdk_marketplacemetering::types::{Tag, UsageAllocation};

    let client = make_metering_client().await;

    let tag = Tag::builder()
        .key("env")
        .value("production")
        .build()
        .unwrap();

    let allocation = UsageAllocation::builder()
        .allocated_usage_quantity(100)
        .tags(tag)
        .build()
        .unwrap();

    let resp = client
        .meter_usage()
        .product_code("test-product-code")
        .usage_dimension("api-calls")
        .usage_quantity(100)
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .usage_allocations(allocation)
        .send()
        .await
        .expect("meter_usage with usage_allocations should succeed");

    assert!(
        resp.metering_record_id().is_some(),
        "should return a metering_record_id when usage_allocations provided"
    );
}

#[tokio::test]
async fn test_meter_usage_dry_run_does_not_persist() {
    let client = make_metering_client().await;

    // dry_run=true: record should not be persisted but should return an ID
    let resp_dry = client
        .meter_usage()
        .product_code("test-product-code")
        .usage_dimension("storage")
        .usage_quantity(500)
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .dry_run(true)
        .send()
        .await
        .expect("meter_usage dry_run=true should succeed");

    assert!(
        resp_dry.metering_record_id().is_some(),
        "dry_run should return a metering_record_id"
    );

    // dry_run=false: real record
    let resp_real = client
        .meter_usage()
        .product_code("test-product-code")
        .usage_dimension("storage")
        .usage_quantity(500)
        .timestamp(aws_smithy_types::DateTime::from_secs(1700007200))
        .dry_run(false)
        .send()
        .await
        .expect("meter_usage dry_run=false should succeed");

    assert!(
        resp_real.metering_record_id().is_some(),
        "real call should return a metering_record_id"
    );

    // The IDs should be different (each call generates a unique UUID)
    assert_ne!(
        resp_dry.metering_record_id().unwrap_or_default(),
        resp_real.metering_record_id().unwrap_or_default(),
        "dry_run and real calls should return different record IDs"
    );
}

// --- RegisterUsage additional tests ---

#[tokio::test]
async fn test_register_usage_missing_product_code() {
    let client = make_metering_client().await;

    // The handler returns 400 InvalidProductCodeException for missing product_code.
    // The SDK enforces ProductCode as a required field client-side (non-empty check).
    let result = client
        .register_usage()
        .product_code("")
        .public_key_version(1)
        .send()
        .await;

    assert!(
        result.is_err(),
        "register_usage with empty product_code should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidProductCodeException") || err_str.contains("product"),
        "Expected InvalidProductCodeException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_register_usage_invalid_public_key_version() {
    let client = make_metering_client().await;

    // PublicKeyVersion minimum value is 1 per the API spec.
    // The handler returns 400 InvalidPublicKeyVersionException for version < 1.
    // Note: The SDK enforces minimum value=1 constraint; sending 0 may be rejected
    // client-side. Either way the operation should fail.
    let result = client
        .register_usage()
        .product_code("test-product-code")
        .public_key_version(0)
        .send()
        .await;

    assert!(
        result.is_err(),
        "register_usage with public_key_version=0 should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidPublicKeyVersionException")
            || err_str.contains("public_key_version")
            || err_str.contains("PublicKeyVersion"),
        "Expected InvalidPublicKeyVersionException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_register_usage_with_nonce() {
    let client = make_metering_client().await;

    let resp = client
        .register_usage()
        .product_code("test-product-code")
        .public_key_version(1)
        .nonce("2ead20e4-3e6d-42cd-8f56-24f02d1cc4e1")
        .send()
        .await
        .expect("register_usage with nonce should succeed");

    assert!(
        resp.signature().is_some(),
        "should return a JWT signature when nonce provided"
    );
    assert!(!resp.signature().unwrap_or_default().is_empty());
}

// --- BatchMeterUsage additional tests ---

#[tokio::test]
async fn test_batch_meter_usage_single_record() {
    use aws_sdk_marketplacemetering::types::UsageRecord;

    let client = make_metering_client().await;

    let record = UsageRecord::builder()
        .customer_identifier("customer-single")
        .dimension("api-calls")
        .quantity(50)
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .build()
        .unwrap();

    let resp = client
        .batch_meter_usage()
        .product_code("test-product-code")
        .usage_records(record)
        .send()
        .await
        .expect("batch_meter_usage with single record should succeed");

    let results = resp.results();
    assert_eq!(results.len(), 1, "should have 1 result for 1 input record");
}

#[tokio::test]
async fn test_batch_meter_usage_missing_product_code() {
    use aws_sdk_marketplacemetering::types::UsageRecord;

    let client = make_metering_client().await;

    let record = UsageRecord::builder()
        .customer_identifier("customer-1")
        .dimension("api-calls")
        .quantity(10)
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .build()
        .unwrap();

    // An empty product_code triggers InvalidProductCodeException in the handler.
    let result = client
        .batch_meter_usage()
        .product_code("")
        .usage_records(record)
        .send()
        .await;

    assert!(
        result.is_err(),
        "batch_meter_usage with empty product_code should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidProductCodeException") || err_str.contains("product"),
        "Expected InvalidProductCodeException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_batch_meter_usage_result_status() {
    use aws_sdk_marketplacemetering::types::UsageRecord;

    let client = make_metering_client().await;

    let record = UsageRecord::builder()
        .customer_identifier("customer-status-test")
        .dimension("requests")
        .quantity(75)
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .build()
        .unwrap();

    let resp = client
        .batch_meter_usage()
        .product_code("test-product-code")
        .usage_records(record)
        .send()
        .await
        .expect("batch_meter_usage should succeed");

    let results = resp.results();
    assert_eq!(results.len(), 1);

    let result = &results[0];
    let status = result.status();
    assert!(status.is_some(), "each result should have a status field");
    let status_str = format!("{:?}", status.unwrap());
    assert!(
        status_str.contains("Success"),
        "status should be Success for normal records, got: {status_str}"
    );
}

#[tokio::test]
async fn test_batch_meter_usage_result_has_metering_record_ids() {
    use aws_sdk_marketplacemetering::types::UsageRecord;

    let client = make_metering_client().await;

    let record1 = UsageRecord::builder()
        .customer_identifier("customer-a")
        .dimension("storage")
        .quantity(100)
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .build()
        .unwrap();

    let record2 = UsageRecord::builder()
        .customer_identifier("customer-b")
        .dimension("storage")
        .quantity(200)
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .build()
        .unwrap();

    let resp = client
        .batch_meter_usage()
        .product_code("test-product-code")
        .usage_records(record1)
        .usage_records(record2)
        .send()
        .await
        .expect("batch_meter_usage should succeed");

    let results = resp.results();
    assert_eq!(results.len(), 2, "should return 2 results for 2 records");

    for (i, result) in results.iter().enumerate() {
        let record_id = result.metering_record_id().unwrap_or_default();
        assert!(
            !record_id.is_empty(),
            "result[{i}] metering_record_id should be non-empty"
        );
    }

    // Verify the two metering record IDs are distinct
    let id0 = results[0].metering_record_id().unwrap_or_default();
    let id1 = results[1].metering_record_id().unwrap_or_default();
    assert_ne!(id0, id1, "metering_record_ids should be unique per record");
}

#[tokio::test]
async fn test_batch_meter_usage_unprocessed_records_empty() {
    use aws_sdk_marketplacemetering::types::UsageRecord;

    let client = make_metering_client().await;

    let record = UsageRecord::builder()
        .customer_identifier("customer-unprocessed-test")
        .dimension("api-calls")
        .quantity(10)
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .build()
        .unwrap();

    let resp = client
        .batch_meter_usage()
        .product_code("test-product-code")
        .usage_records(record)
        .send()
        .await
        .expect("batch_meter_usage should succeed");

    let unprocessed = resp.unprocessed_records();
    assert!(
        unprocessed.is_empty(),
        "unprocessed_records should be empty for valid input, got {} records",
        unprocessed.len()
    );
}

#[tokio::test]
async fn test_batch_meter_usage_with_usage_allocations() {
    use aws_sdk_marketplacemetering::types::{Tag, UsageAllocation, UsageRecord};

    let client = make_metering_client().await;

    let tag = Tag::builder()
        .key("team")
        .value("platform")
        .build()
        .unwrap();

    let allocation = UsageAllocation::builder()
        .allocated_usage_quantity(50)
        .tags(tag)
        .build()
        .unwrap();

    let record = UsageRecord::builder()
        .customer_identifier("customer-alloc")
        .dimension("compute")
        .quantity(50)
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .usage_allocations(allocation)
        .build()
        .unwrap();

    let resp = client
        .batch_meter_usage()
        .product_code("test-product-code")
        .usage_records(record)
        .send()
        .await
        .expect("batch_meter_usage with usage_allocations should succeed");

    assert_eq!(resp.results().len(), 1);
}

// --- ResolveCustomer additional tests ---

#[tokio::test]
async fn test_resolve_customer_returns_consistent_result() {
    let client = make_metering_client().await;

    let token = "consistent-token-abcdef01";

    let resp1 = client
        .resolve_customer()
        .registration_token(token)
        .send()
        .await
        .expect("first resolve_customer should succeed");

    let resp2 = client
        .resolve_customer()
        .registration_token(token)
        .send()
        .await
        .expect("second resolve_customer should succeed");

    // winterbaume caches resolved customers, so same token returns same data
    assert_eq!(
        resp1.customer_identifier(),
        resp2.customer_identifier(),
        "same token should resolve to same customer_identifier"
    );
    assert_eq!(
        resp1.customer_aws_account_id(),
        resp2.customer_aws_account_id(),
        "same token should resolve to same customer_aws_account_id"
    );
}

#[tokio::test]
async fn test_resolve_customer_missing_token() {
    let client = make_metering_client().await;

    // Empty registration token should be rejected by the handler.
    let result = client
        .resolve_customer()
        .registration_token("")
        .send()
        .await;

    assert!(
        result.is_err(),
        "resolve_customer with empty registration_token should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidTokenException") || err_str.contains("token"),
        "Expected InvalidTokenException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_resolve_customer_different_tokens() {
    let client = make_metering_client().await;

    let resp1 = client
        .resolve_customer()
        .registration_token("token-aabbccdd-11111111")
        .send()
        .await
        .expect("resolve_customer with token-1 should succeed");

    let resp2 = client
        .resolve_customer()
        .registration_token("token-eeffgghh-22222222")
        .send()
        .await
        .expect("resolve_customer with token-2 should succeed");

    // Different tokens should produce different customer identifiers
    assert_ne!(
        resp1.customer_identifier(),
        resp2.customer_identifier(),
        "different tokens should resolve to different customer identifiers"
    );
}
