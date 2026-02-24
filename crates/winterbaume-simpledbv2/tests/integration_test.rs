use aws_sdk_simpledbv2::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_simpledbv2::SimpleDbV2Service;

async fn make_sdb_client() -> aws_sdk_simpledbv2::Client {
    let svc = SimpleDbV2Service::new()
        .with_domain("us-east-1", "test-domain")
        .await;
    let mock = MockAws::builder().with_service(svc).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_simpledbv2::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_simpledbv2::Client::new(&config)
}

#[tokio::test]
async fn test_start_domain_export_and_get_export() {
    let client = make_sdb_client().await;

    let start_resp = client
        .start_domain_export()
        .domain_name("test-domain")
        .s3_bucket("my-export-bucket")
        .send()
        .await
        .expect("start_domain_export should succeed");

    let export_arn = start_resp.export_arn();
    assert!(!export_arn.is_empty());
    assert!(export_arn.contains("test-domain"));

    let get_resp = client
        .get_export()
        .export_arn(export_arn)
        .send()
        .await
        .expect("get_export should succeed");

    assert_eq!(get_resp.export_arn(), export_arn);
    assert_eq!(get_resp.domain_name(), "test-domain");
    assert_eq!(get_resp.s3_bucket(), "my-export-bucket");
    assert_eq!(get_resp.export_status().as_str(), "SUCCEEDED");
}

#[tokio::test]
async fn test_list_exports() {
    let client = make_sdb_client().await;

    // List should be empty initially
    let list_resp = client
        .list_exports()
        .send()
        .await
        .expect("list_exports should succeed");

    assert!(list_resp.export_summaries().is_empty());

    // Start an export
    client
        .start_domain_export()
        .domain_name("test-domain")
        .s3_bucket("bucket-1")
        .send()
        .await
        .expect("start_domain_export should succeed");

    // List again
    let list_resp = client
        .list_exports()
        .send()
        .await
        .expect("list_exports should succeed");

    assert_eq!(list_resp.export_summaries().len(), 1);
    let summary = &list_resp.export_summaries()[0];
    assert_eq!(summary.domain_name(), "test-domain");
    assert_eq!(summary.export_status().as_str(), "SUCCEEDED");
}

#[tokio::test]
async fn test_list_exports_with_domain_filter() {
    let svc = SimpleDbV2Service::new()
        .with_domain("us-east-1", "domain-a")
        .await
        .with_domain("us-east-1", "domain-b")
        .await;
    let mock = MockAws::builder().with_service(svc).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_simpledbv2::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_simpledbv2::Client::new(&config);

    // Create exports for two domains
    client
        .start_domain_export()
        .domain_name("domain-a")
        .s3_bucket("bucket")
        .send()
        .await
        .unwrap();

    client
        .start_domain_export()
        .domain_name("domain-b")
        .s3_bucket("bucket")
        .send()
        .await
        .unwrap();

    // List all
    let all = client.list_exports().send().await.unwrap();
    assert_eq!(all.export_summaries().len(), 2);

    // Filter by domain-a
    let filtered = client
        .list_exports()
        .domain_name("domain-a")
        .send()
        .await
        .unwrap();
    assert_eq!(filtered.export_summaries().len(), 1);
    assert_eq!(filtered.export_summaries()[0].domain_name(), "domain-a");
}

#[tokio::test]
async fn test_start_export_nonexistent_domain() {
    let mock = MockAws::builder()
        .with_service(SimpleDbV2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_simpledbv2::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_simpledbv2::Client::new(&config);

    let result = client
        .start_domain_export()
        .domain_name("nonexistent-domain")
        .s3_bucket("bucket")
        .send()
        .await;

    assert!(result.is_err(), "should fail for nonexistent domain");
}

#[tokio::test]
async fn test_get_export_nonexistent() {
    let client = make_sdb_client().await;

    let result = client
        .get_export()
        .export_arn("arn:aws:sdb:us-east-1:123456789012:export/nonexistent")
        .send()
        .await;

    assert!(result.is_err(), "should fail for nonexistent export");
}

#[tokio::test]
async fn test_start_export_with_s3_options() {
    let client = make_sdb_client().await;

    let resp = client
        .start_domain_export()
        .domain_name("test-domain")
        .s3_bucket("my-bucket")
        .s3_key_prefix("exports/")
        .s3_sse_algorithm(aws_sdk_simpledbv2::types::S3SseAlgorithm::Aes256)
        .send()
        .await
        .expect("start_domain_export with options should succeed");

    let export_arn = resp.export_arn();

    let get_resp = client
        .get_export()
        .export_arn(export_arn)
        .send()
        .await
        .expect("get_export should succeed");

    assert_eq!(get_resp.s3_key_prefix(), Some("exports/"));
    assert_eq!(
        get_resp.s3_sse_algorithm(),
        Some(&aws_sdk_simpledbv2::types::S3SseAlgorithm::Aes256)
    );
}

// ============================================================================
// Tests derived from AWS documentation: Amazon SimpleDB v2 Export API
// Source: https://docs.aws.amazon.com/AmazonSimpleDB/latest/DeveloperGuide/SDB_API_StartDomainExport.html
// ============================================================================

#[tokio::test]
async fn test_start_export_response_fields() {
    let client = make_sdb_client().await;

    let resp = client
        .start_domain_export()
        .domain_name("test-domain")
        .s3_bucket("response-fields-bucket")
        .send()
        .await
        .expect("start_domain_export should succeed");

    assert!(!resp.export_arn().is_empty(), "exportArn must be non-empty");
    assert!(
        !resp.client_token().is_empty(),
        "clientToken must be non-empty"
    );
    // requestedAt is a required timestamp field — ensure it deserialised without panic
    let _requested_at = resp.requested_at();
}

#[tokio::test]
async fn test_get_export_all_fields() {
    let client = make_sdb_client().await;

    let start_resp = client
        .start_domain_export()
        .domain_name("test-domain")
        .s3_bucket("all-fields-bucket")
        .send()
        .await
        .expect("start_domain_export should succeed");

    let export_arn = start_resp.export_arn();

    let get_resp = client
        .get_export()
        .export_arn(export_arn)
        .send()
        .await
        .expect("get_export should succeed");

    assert_eq!(get_resp.export_arn(), export_arn);
    assert!(
        !get_resp.client_token().is_empty(),
        "clientToken must be non-empty"
    );
    assert_eq!(get_resp.export_status().as_str(), "SUCCEEDED");
    assert_eq!(get_resp.domain_name(), "test-domain");
    assert_eq!(get_resp.s3_bucket(), "all-fields-bucket");
    let _requested_at = get_resp.requested_at();
}

#[tokio::test]
async fn test_get_export_optional_fields_present() {
    // exportDataCutoffTime and exportManifest should be present in the mock
    let client = make_sdb_client().await;

    let start_resp = client
        .start_domain_export()
        .domain_name("test-domain")
        .s3_bucket("optional-fields-bucket")
        .send()
        .await
        .expect("start_domain_export should succeed");

    let get_resp = client
        .get_export()
        .export_arn(start_resp.export_arn())
        .send()
        .await
        .expect("get_export should succeed");

    assert!(
        get_resp.export_data_cutoff_time().is_some(),
        "exportDataCutoffTime should be present"
    );
    assert!(
        get_resp.export_manifest().is_some(),
        "exportManifest should be present"
    );
    assert!(
        get_resp.items_count().is_some(),
        "itemsCount should be present"
    );
}

#[tokio::test]
async fn test_start_export_with_s3_bucket_owner() {
    let client = make_sdb_client().await;

    let resp = client
        .start_domain_export()
        .domain_name("test-domain")
        .s3_bucket("cross-account-bucket")
        .s3_bucket_owner("111122223333")
        .send()
        .await
        .expect("start_domain_export with s3BucketOwner should succeed");

    let get_resp = client
        .get_export()
        .export_arn(resp.export_arn())
        .send()
        .await
        .expect("get_export should succeed");

    assert_eq!(
        get_resp.s3_bucket_owner(),
        Some("111122223333"),
        "s3BucketOwner should be reflected in GetExport response"
    );
}

#[tokio::test]
async fn test_start_export_with_kms_encryption() {
    let client = make_sdb_client().await;

    let kms_key_id = "arn:aws:kms:us-east-1:123456789012:key/my-kms-key";

    let resp = client
        .start_domain_export()
        .domain_name("test-domain")
        .s3_bucket("kms-encrypted-bucket")
        .s3_sse_algorithm(aws_sdk_simpledbv2::types::S3SseAlgorithm::Kms)
        .s3_sse_kms_key_id(kms_key_id)
        .send()
        .await
        .expect("start_domain_export with KMS encryption should succeed");

    let get_resp = client
        .get_export()
        .export_arn(resp.export_arn())
        .send()
        .await
        .expect("get_export should succeed");

    assert_eq!(
        get_resp.s3_sse_algorithm(),
        Some(&aws_sdk_simpledbv2::types::S3SseAlgorithm::Kms),
        "s3SseAlgorithm should be KMS"
    );
    assert_eq!(
        get_resp.s3_sse_kms_key_id(),
        Some(kms_key_id),
        "s3SseKmsKeyId should match the provided key ID"
    );
}

#[tokio::test]
async fn test_client_token_idempotency() {
    let client = make_sdb_client().await;

    let token = "idempotency-token-abc123";

    let first = client
        .start_domain_export()
        .domain_name("test-domain")
        .s3_bucket("idempotency-bucket")
        .client_token(token)
        .send()
        .await
        .expect("first start_domain_export should succeed");

    let second = client
        .start_domain_export()
        .domain_name("test-domain")
        .s3_bucket("idempotency-bucket")
        .client_token(token)
        .send()
        .await
        .expect("second start_domain_export with same token should succeed (idempotent)");

    assert_eq!(
        first.export_arn(),
        second.export_arn(),
        "same clientToken must return the same exportArn"
    );
    assert_eq!(
        first.client_token(),
        second.client_token(),
        "clientToken must be stable across idempotent calls"
    );
}

#[tokio::test]
async fn test_client_token_conflict() {
    // Same token but different domainName must return ConflictException
    let svc = SimpleDbV2Service::new()
        .with_domain("us-east-1", "domain-one")
        .await
        .with_domain("us-east-1", "domain-two")
        .await;
    let mock = MockAws::builder().with_service(svc).build();

    let config = aws_config::defaults(aws_sdk_simpledbv2::config::BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_simpledbv2::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_simpledbv2::Client::new(&config);

    let token = "conflict-token-xyz";

    client
        .start_domain_export()
        .domain_name("domain-one")
        .s3_bucket("bucket")
        .client_token(token)
        .send()
        .await
        .expect("first export should succeed");

    let err = client
        .start_domain_export()
        .domain_name("domain-two")
        .s3_bucket("bucket")
        .client_token(token)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ConflictException"),
        "Expected ConflictException for same token with different params, got: {err_str}"
    );
}

#[tokio::test]
async fn test_export_arn_format() {
    let client = make_sdb_client().await;

    let resp = client
        .start_domain_export()
        .domain_name("test-domain")
        .s3_bucket("arn-format-bucket")
        .send()
        .await
        .expect("start_domain_export should succeed");

    let arn = resp.export_arn();
    // Expected format: arn:aws:sdb:{region}:{account}:domain/{domainName}/export/{exportId}
    assert!(
        arn.starts_with("arn:aws:sdb:"),
        "ARN must start with arn:aws:sdb:"
    );
    assert!(
        arn.contains("test-domain"),
        "ARN must contain the domain name"
    );
    assert!(arn.contains("/export/"), "ARN must contain /export/");
}

#[tokio::test]
async fn test_export_status_is_succeeded() {
    let client = make_sdb_client().await;

    let start_resp = client
        .start_domain_export()
        .domain_name("test-domain")
        .s3_bucket("status-bucket")
        .send()
        .await
        .expect("start_domain_export should succeed");

    let get_resp = client
        .get_export()
        .export_arn(start_resp.export_arn())
        .send()
        .await
        .expect("get_export should succeed");

    assert_eq!(
        get_resp.export_status().as_str(),
        "SUCCEEDED",
        "Mock export status should be SUCCEEDED"
    );
}

#[tokio::test]
async fn test_list_exports_max_results() {
    let client = make_sdb_client().await;

    // Start 3 exports
    for i in 0..3 {
        client
            .start_domain_export()
            .domain_name("test-domain")
            .s3_bucket(format!("bucket-{i}"))
            .send()
            .await
            .expect("start_domain_export should succeed");
    }

    let list_resp = client
        .list_exports()
        .max_results(2)
        .send()
        .await
        .expect("list_exports with maxResults should succeed");

    assert!(
        list_resp.export_summaries().len() <= 2,
        "maxResults=2 should return at most 2 exports, got {}",
        list_resp.export_summaries().len()
    );
}

#[tokio::test]
async fn test_list_exports_nonexistent_domain_filter() {
    let client = make_sdb_client().await;

    let err = client
        .list_exports()
        .domain_name("does-not-exist")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchDomain") || err_str.contains("NoSuchDomainException"),
        "Expected NoSuchDomain error for nonexistent domain filter, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_exports_summary_fields() {
    let client = make_sdb_client().await;

    client
        .start_domain_export()
        .domain_name("test-domain")
        .s3_bucket("summary-fields-bucket")
        .send()
        .await
        .expect("start_domain_export should succeed");

    let list_resp = client
        .list_exports()
        .send()
        .await
        .expect("list_exports should succeed");

    assert!(!list_resp.export_summaries().is_empty());
    let summary = &list_resp.export_summaries()[0];
    assert!(
        !summary.export_arn().is_empty(),
        "exportArn must be non-empty in summary"
    );
    assert_eq!(
        summary.domain_name(),
        "test-domain",
        "domainName must match in summary"
    );
    assert!(
        !summary.export_status().as_str().is_empty(),
        "exportStatus must be non-empty in summary"
    );
}

#[tokio::test]
async fn test_full_lifecycle() {
    let client = make_sdb_client().await;

    // 1. Start export
    let start_resp = client
        .start_domain_export()
        .domain_name("test-domain")
        .s3_bucket("lifecycle-bucket")
        .s3_key_prefix("exports/2026/")
        .send()
        .await
        .expect("start_domain_export should succeed");

    let export_arn = start_resp.export_arn().to_string();
    assert!(!export_arn.is_empty());

    // 2. Get export and verify consistency
    let get_resp = client
        .get_export()
        .export_arn(&export_arn)
        .send()
        .await
        .expect("get_export should succeed");

    assert_eq!(get_resp.export_arn(), export_arn);
    assert_eq!(get_resp.domain_name(), "test-domain");
    assert_eq!(get_resp.s3_bucket(), "lifecycle-bucket");
    assert_eq!(get_resp.s3_key_prefix(), Some("exports/2026/"));
    assert_eq!(get_resp.export_status().as_str(), "SUCCEEDED");

    // 3. List exports and verify the export appears
    let list_resp = client
        .list_exports()
        .send()
        .await
        .expect("list_exports should succeed");

    let found = list_resp
        .export_summaries()
        .iter()
        .any(|s| s.export_arn() == export_arn);
    assert!(found, "started export should appear in list_exports");

    // 4. List filtered by domain — export must still appear
    let filtered = client
        .list_exports()
        .domain_name("test-domain")
        .send()
        .await
        .expect("list_exports with domain filter should succeed");

    let found_filtered = filtered
        .export_summaries()
        .iter()
        .any(|s| s.export_arn() == export_arn);
    assert!(
        found_filtered,
        "export should appear when filtered by its domain name"
    );
}

// ============================================================================
// Additional tests derived from AWS documentation gaps (2026-03-28)
// ============================================================================

#[tokio::test]
async fn test_start_export_missing_domain_name() {
    // AWS docs: domainName is a required parameter for StartDomainExport.
    // The SDK enforces this at the builder level with a runtime validation error.
    let client = make_sdb_client().await;

    // Build the request without domain_name — the SDK returns a BuildError before
    // sending, which surfaces as an SdkError::Construction variant.
    let result = client
        .start_domain_export()
        .s3_bucket("some-bucket")
        .send()
        .await;

    assert!(
        result.is_err(),
        "omitting domainName should return an error"
    );
}

#[tokio::test]
async fn test_start_export_missing_s3_bucket() {
    // AWS docs: s3Bucket is a required parameter for StartDomainExport.
    let client = make_sdb_client().await;

    let result = client
        .start_domain_export()
        .domain_name("test-domain")
        .send()
        .await;

    assert!(result.is_err(), "omitting s3Bucket should return an error");
}

#[tokio::test]
async fn test_get_export_missing_export_arn() {
    // AWS docs: exportArn is a required parameter for GetExport.
    let client = make_sdb_client().await;

    let result = client.get_export().send().await;

    assert!(result.is_err(), "omitting exportArn should return an error");
}

#[tokio::test]
async fn test_idempotency_no_duplicate_in_list() {
    // Idempotent StartDomainExport calls with the same clientToken must not create
    // a second export entry — ListExports should still return exactly 1 item.
    let client = make_sdb_client().await;

    let token = "dedup-token-xyz";

    client
        .start_domain_export()
        .domain_name("test-domain")
        .s3_bucket("dedup-bucket")
        .client_token(token)
        .send()
        .await
        .expect("first call should succeed");

    client
        .start_domain_export()
        .domain_name("test-domain")
        .s3_bucket("dedup-bucket")
        .client_token(token)
        .send()
        .await
        .expect("second (idempotent) call should succeed");

    let list = client
        .list_exports()
        .send()
        .await
        .expect("list_exports should succeed");

    assert_eq!(
        list.export_summaries().len(),
        1,
        "idempotent calls must not create duplicate entries; expected 1, got {}",
        list.export_summaries().len()
    );
}

#[tokio::test]
async fn test_list_exports_summary_has_requested_at() {
    // winterbaume includes requestedAt in ListExports summaries; verify it is present.
    let client = make_sdb_client().await;

    client
        .start_domain_export()
        .domain_name("test-domain")
        .s3_bucket("ts-bucket")
        .send()
        .await
        .expect("start_domain_export should succeed");

    let list = client
        .list_exports()
        .send()
        .await
        .expect("list_exports should succeed");

    assert!(!list.export_summaries().is_empty());
    for summary in list.export_summaries() {
        // requested_at() returns a non-optional DateTime on the summary type;
        // just ensure calling it doesn't panic.
        let _ts = summary.requested_at();
    }
}

#[tokio::test]
async fn test_list_exports_next_token_absent_when_no_more() {
    // When all results fit within the page, nextToken should be absent.
    let client = make_sdb_client().await;

    client
        .start_domain_export()
        .domain_name("test-domain")
        .s3_bucket("no-more-bucket")
        .send()
        .await
        .expect("start_domain_export should succeed");

    let list = client
        .list_exports()
        .max_results(100)
        .send()
        .await
        .expect("list_exports should succeed");

    assert!(
        list.next_token().is_none(),
        "nextToken should be absent when all results fit within maxResults"
    );
}

#[tokio::test]
async fn test_start_export_without_client_token() {
    // AWS docs list clientToken as required, but winterbaume auto-generates one when
    // it is omitted (accepted divergence for mock convenience). Verify the response
    // still includes a non-empty clientToken.
    let client = make_sdb_client().await;

    let resp = client
        .start_domain_export()
        .domain_name("test-domain")
        .s3_bucket("no-token-bucket")
        // intentionally no .client_token(...)
        .send()
        .await
        .expect("start_domain_export without clientToken should succeed in the mock");

    assert!(
        !resp.client_token().is_empty(),
        "clientToken should be auto-generated and non-empty when omitted"
    );
    assert!(
        !resp.export_arn().is_empty(),
        "exportArn should be non-empty"
    );
}

#[tokio::test]
async fn test_get_export_returns_consistent_arn() {
    // GetExport called twice for the same export should return the identical exportArn.
    let client = make_sdb_client().await;

    let start_resp = client
        .start_domain_export()
        .domain_name("test-domain")
        .s3_bucket("consistent-arn-bucket")
        .send()
        .await
        .expect("start_domain_export should succeed");

    let arn = start_resp.export_arn().to_string();

    let first = client
        .get_export()
        .export_arn(&arn)
        .send()
        .await
        .expect("first get_export should succeed");

    let second = client
        .get_export()
        .export_arn(&arn)
        .send()
        .await
        .expect("second get_export should succeed");

    assert_eq!(
        first.export_arn(),
        second.export_arn(),
        "exportArn must be identical across multiple GetExport calls for the same export"
    );
    assert_eq!(
        first.export_status(),
        second.export_status(),
        "exportStatus must be identical across consecutive GetExport calls"
    );
}
