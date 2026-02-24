//! Integration tests for winterbaume Textract service.
//!
//! These tests verify that aws-sdk-textract operations work end-to-end
//! through the winterbaume mock infrastructure.

use aws_sdk_textract::config::BehaviorVersion;
use aws_sdk_textract::primitives::Blob;
use winterbaume_core::MockAws;
use winterbaume_textract::TextractService;

/// Helper to create a configured Textract client backed by winterbaume.
async fn make_textract_client() -> aws_sdk_textract::Client {
    let mock = MockAws::builder()
        .with_service(TextractService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_textract::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_textract::Client::new(&config)
}

#[tokio::test]
async fn test_detect_document_text() {
    let client = make_textract_client().await;

    // Minimal PNG-like bytes (just enough to pass SDK validation)
    let document_bytes = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

    let resp = client
        .detect_document_text()
        .document(
            aws_sdk_textract::types::Document::builder()
                .bytes(Blob::new(document_bytes))
                .build(),
        )
        .send()
        .await
        .expect("detect_document_text should succeed");

    // Should return empty blocks list
    let blocks = resp.blocks();
    assert!(blocks.is_empty(), "mock should return empty blocks list");

    // Should have document metadata
    let metadata = resp.document_metadata();
    assert!(metadata.is_some(), "should have document metadata");
}

#[tokio::test]
async fn test_analyze_document() {
    let client = make_textract_client().await;

    let document_bytes = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

    let resp = client
        .analyze_document()
        .document(
            aws_sdk_textract::types::Document::builder()
                .bytes(Blob::new(document_bytes))
                .build(),
        )
        .feature_types(aws_sdk_textract::types::FeatureType::Tables)
        .send()
        .await
        .expect("analyze_document should succeed");

    // Should return empty blocks list
    let blocks = resp.blocks();
    assert!(blocks.is_empty(), "mock should return empty blocks list");

    // Should have document metadata
    let metadata = resp.document_metadata();
    assert!(metadata.is_some(), "should have document metadata");
}

#[tokio::test]
async fn test_get_document_analysis() {
    let client = make_textract_client().await;

    let resp = client
        .get_document_analysis()
        .job_id("mock-job-id-12345")
        .send()
        .await
        .expect("get_document_analysis should succeed");

    // Should return SUCCEEDED status
    assert_eq!(
        resp.job_status(),
        Some(&aws_sdk_textract::types::JobStatus::Succeeded),
        "mock should return SUCCEEDED job status"
    );

    // Should return empty blocks list
    let blocks = resp.blocks();
    assert!(blocks.is_empty(), "mock should return empty blocks list");
}

#[tokio::test]
async fn test_analyze_document_with_multiple_feature_types() {
    let client = make_textract_client().await;

    let document_bytes = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

    let resp = client
        .analyze_document()
        .document(
            aws_sdk_textract::types::Document::builder()
                .bytes(Blob::new(document_bytes))
                .build(),
        )
        .feature_types(aws_sdk_textract::types::FeatureType::Tables)
        .feature_types(aws_sdk_textract::types::FeatureType::Forms)
        .send()
        .await
        .expect("analyze_document with multiple feature types should succeed");

    let blocks = resp.blocks();
    assert!(blocks.is_empty(), "mock should return empty blocks list");
}

#[tokio::test]
async fn test_detect_document_text_with_s3_object() {
    let client = make_textract_client().await;

    // Test using S3Object reference instead of bytes
    let resp = client
        .detect_document_text()
        .document(
            aws_sdk_textract::types::Document::builder()
                .s3_object(
                    aws_sdk_textract::types::S3Object::builder()
                        .bucket("my-bucket")
                        .name("my-document.jpg")
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("detect_document_text with S3 object should succeed");

    let blocks = resp.blocks();
    assert!(blocks.is_empty());

    let metadata = resp.document_metadata();
    assert!(metadata.is_some());
    assert_eq!(resp.detect_document_text_model_version(), Some("1.0"));
}

#[tokio::test]
async fn test_start_document_text_detection() {
    let client = make_textract_client().await;

    let resp = client
        .start_document_text_detection()
        .document_location(
            aws_sdk_textract::types::DocumentLocation::builder()
                .s3_object(
                    aws_sdk_textract::types::S3Object::builder()
                        .bucket("bucket")
                        .name("name")
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("start_document_text_detection should succeed");

    assert!(resp.job_id().is_some(), "should have a job ID");
    assert!(!resp.job_id().unwrap().is_empty());
}

#[tokio::test]
async fn test_get_document_text_detection() {
    let client = make_textract_client().await;

    // Start a job first
    let start_resp = client
        .start_document_text_detection()
        .document_location(
            aws_sdk_textract::types::DocumentLocation::builder()
                .s3_object(
                    aws_sdk_textract::types::S3Object::builder()
                        .bucket("bucket")
                        .name("name")
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let job_id = start_resp.job_id().unwrap().to_string();

    let resp = client
        .get_document_text_detection()
        .job_id(&job_id)
        .send()
        .await
        .expect("get_document_text_detection should succeed");

    assert_eq!(
        resp.job_status(),
        Some(&aws_sdk_textract::types::JobStatus::Succeeded)
    );
    // A job started via StartDocumentTextDetection always gets at least one mock block.
    assert!(
        !resp.blocks().is_empty(),
        "GetDocumentTextDetection should return blocks for a known job"
    );
}

#[tokio::test]
async fn test_start_document_analysis() {
    let client = make_textract_client().await;

    let resp = client
        .start_document_analysis()
        .document_location(
            aws_sdk_textract::types::DocumentLocation::builder()
                .s3_object(
                    aws_sdk_textract::types::S3Object::builder()
                        .bucket("bucket")
                        .name("name")
                        .build(),
                )
                .build(),
        )
        .feature_types(aws_sdk_textract::types::FeatureType::Tables)
        .feature_types(aws_sdk_textract::types::FeatureType::Forms)
        .send()
        .await
        .expect("start_document_analysis should succeed");

    assert!(resp.job_id().is_some(), "should have a job ID");
    assert!(
        !resp.job_id().unwrap().is_empty(),
        "job ID must be non-empty"
    );
}

// ============================================================================
// Tests derived from AWS documentation: Amazon Textract
// ============================================================================

#[tokio::test]
async fn test_detect_document_text_model_version() {
    let client = make_textract_client().await;

    let document_bytes = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

    let resp = client
        .detect_document_text()
        .document(
            aws_sdk_textract::types::Document::builder()
                .bytes(Blob::new(document_bytes))
                .build(),
        )
        .send()
        .await
        .expect("detect_document_text should succeed");

    assert_eq!(
        resp.detect_document_text_model_version(),
        Some("1.0"),
        "should return model version 1.0"
    );
}

#[tokio::test]
async fn test_analyze_document_model_version() {
    let client = make_textract_client().await;

    let document_bytes = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

    let resp = client
        .analyze_document()
        .document(
            aws_sdk_textract::types::Document::builder()
                .bytes(Blob::new(document_bytes))
                .build(),
        )
        .feature_types(aws_sdk_textract::types::FeatureType::Tables)
        .send()
        .await
        .expect("analyze_document should succeed");

    assert_eq!(
        resp.analyze_document_model_version(),
        Some("1.0"),
        "should return model version 1.0"
    );
}

#[tokio::test]
async fn test_analyze_document_forms_feature_type() {
    let client = make_textract_client().await;

    let document_bytes = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

    let resp = client
        .analyze_document()
        .document(
            aws_sdk_textract::types::Document::builder()
                .bytes(Blob::new(document_bytes))
                .build(),
        )
        .feature_types(aws_sdk_textract::types::FeatureType::Forms)
        .send()
        .await
        .expect("analyze_document with Forms feature type should succeed");

    assert!(
        resp.document_metadata().is_some(),
        "should have document metadata"
    );
    assert!(resp.blocks().is_empty(), "mock should return empty blocks");
}

#[tokio::test]
async fn test_analyze_document_with_queries_feature_type() {
    let client = make_textract_client().await;

    let document_bytes = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

    let resp = client
        .analyze_document()
        .document(
            aws_sdk_textract::types::Document::builder()
                .bytes(Blob::new(document_bytes))
                .build(),
        )
        .feature_types(aws_sdk_textract::types::FeatureType::Queries)
        .send()
        .await
        .expect("analyze_document with Queries feature type should succeed");

    assert!(resp.blocks().is_empty());
    assert!(resp.document_metadata().is_some());
}

#[tokio::test]
async fn test_get_document_analysis_model_version() {
    let client = make_textract_client().await;

    let resp = client
        .get_document_analysis()
        .job_id("any-job-id-for-version-check")
        .send()
        .await
        .expect("get_document_analysis should succeed");

    assert_eq!(
        resp.analyze_document_model_version(),
        Some("1.0"),
        "should return model version 1.0"
    );
}

#[tokio::test]
async fn test_get_document_text_detection_model_version() {
    let client = make_textract_client().await;

    let resp = client
        .get_document_text_detection()
        .job_id("any-job-id-for-version-check")
        .send()
        .await
        .expect("get_document_text_detection should succeed");

    assert_eq!(
        resp.detect_document_text_model_version(),
        Some("1.0"),
        "should return model version 1.0"
    );
}

#[tokio::test]
async fn test_get_document_analysis_document_metadata() {
    let client = make_textract_client().await;

    let resp = client
        .get_document_analysis()
        .job_id("metadata-check-job-id")
        .send()
        .await
        .expect("get_document_analysis should succeed");

    let metadata = resp.document_metadata();
    assert!(metadata.is_some(), "should have document metadata");
    assert_eq!(
        metadata.unwrap().pages(),
        Some(1),
        "mock should report 1 page"
    );
}

#[tokio::test]
async fn test_get_document_text_detection_document_metadata() {
    let client = make_textract_client().await;

    let resp = client
        .get_document_text_detection()
        .job_id("metadata-check-job-id")
        .send()
        .await
        .expect("get_document_text_detection should succeed");

    let metadata = resp.document_metadata();
    assert!(metadata.is_some(), "should have document metadata");
    assert_eq!(
        metadata.unwrap().pages(),
        Some(1),
        "mock should report 1 page"
    );
}

#[tokio::test]
async fn test_start_document_analysis_lifecycle() {
    let client = make_textract_client().await;

    // Start an async document analysis job
    let start_resp = client
        .start_document_analysis()
        .document_location(
            aws_sdk_textract::types::DocumentLocation::builder()
                .s3_object(
                    aws_sdk_textract::types::S3Object::builder()
                        .bucket("my-docs-bucket")
                        .name("contracts/contract-2026.pdf")
                        .build(),
                )
                .build(),
        )
        .feature_types(aws_sdk_textract::types::FeatureType::Tables)
        .feature_types(aws_sdk_textract::types::FeatureType::Forms)
        .send()
        .await
        .expect("start_document_analysis should succeed");

    let job_id = start_resp.job_id().expect("must have job ID").to_string();
    assert!(!job_id.is_empty(), "job ID must not be empty");

    // Poll with the returned job ID
    let get_resp = client
        .get_document_analysis()
        .job_id(&job_id)
        .send()
        .await
        .expect("get_document_analysis should succeed");

    assert_eq!(
        get_resp.job_status(),
        Some(&aws_sdk_textract::types::JobStatus::Succeeded),
        "async job should report SUCCEEDED"
    );
    assert!(
        !get_resp.blocks().is_empty(),
        "GetDocumentAnalysis should return blocks for a job started via StartDocumentAnalysis"
    );
    assert!(
        get_resp.document_metadata().is_some(),
        "should have document metadata"
    );
}

#[tokio::test]
async fn test_start_text_detection_lifecycle() {
    let client = make_textract_client().await;

    // Start an async text detection job
    let start_resp = client
        .start_document_text_detection()
        .document_location(
            aws_sdk_textract::types::DocumentLocation::builder()
                .s3_object(
                    aws_sdk_textract::types::S3Object::builder()
                        .bucket("my-docs-bucket")
                        .name("invoices/invoice-001.jpg")
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("start_document_text_detection should succeed");

    let job_id = start_resp.job_id().expect("must have job ID").to_string();
    assert!(!job_id.is_empty(), "job ID must not be empty");

    // Poll with the returned job ID
    let get_resp = client
        .get_document_text_detection()
        .job_id(&job_id)
        .send()
        .await
        .expect("get_document_text_detection should succeed");

    assert_eq!(
        get_resp.job_status(),
        Some(&aws_sdk_textract::types::JobStatus::Succeeded),
        "async job should report SUCCEEDED"
    );
    assert!(
        !get_resp.blocks().is_empty(),
        "GetDocumentTextDetection should return blocks for a job started via StartDocumentTextDetection"
    );
}

#[tokio::test]
async fn test_start_document_text_detection_unique_job_ids() {
    let client = make_textract_client().await;

    let location = || {
        aws_sdk_textract::types::DocumentLocation::builder()
            .s3_object(
                aws_sdk_textract::types::S3Object::builder()
                    .bucket("bucket")
                    .name("doc.pdf")
                    .build(),
            )
            .build()
    };

    let resp1 = client
        .start_document_text_detection()
        .document_location(location())
        .send()
        .await
        .expect("first start should succeed");

    let resp2 = client
        .start_document_text_detection()
        .document_location(location())
        .send()
        .await
        .expect("second start should succeed");

    let job_id1 = resp1.job_id().unwrap();
    let job_id2 = resp2.job_id().unwrap();

    assert_ne!(
        job_id1, job_id2,
        "each StartDocumentTextDetection call should produce a unique job ID"
    );
}

#[tokio::test]
async fn test_start_document_analysis_unique_job_ids() {
    let client = make_textract_client().await;

    let location = || {
        aws_sdk_textract::types::DocumentLocation::builder()
            .s3_object(
                aws_sdk_textract::types::S3Object::builder()
                    .bucket("bucket")
                    .name("doc.pdf")
                    .build(),
            )
            .build()
    };

    let resp1 = client
        .start_document_analysis()
        .document_location(location())
        .feature_types(aws_sdk_textract::types::FeatureType::Tables)
        .send()
        .await
        .expect("first start should succeed");

    let resp2 = client
        .start_document_analysis()
        .document_location(location())
        .feature_types(aws_sdk_textract::types::FeatureType::Tables)
        .send()
        .await
        .expect("second start should succeed");

    let job_id1 = resp1.job_id().unwrap();
    let job_id2 = resp2.job_id().unwrap();

    assert_ne!(
        job_id1, job_id2,
        "each StartDocumentAnalysis call should produce a unique job ID"
    );
}

#[tokio::test]
async fn test_detect_document_text_page_count() {
    let client = make_textract_client().await;

    let document_bytes = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];

    let resp = client
        .detect_document_text()
        .document(
            aws_sdk_textract::types::Document::builder()
                .bytes(Blob::new(document_bytes))
                .build(),
        )
        .send()
        .await
        .expect("detect_document_text should succeed");

    let metadata = resp.document_metadata().expect("must have metadata");
    assert_eq!(metadata.pages(), Some(1), "mock should report 1 page");
}

#[tokio::test]
async fn test_start_document_analysis_with_client_request_token() {
    let client = make_textract_client().await;

    let resp = client
        .start_document_analysis()
        .document_location(
            aws_sdk_textract::types::DocumentLocation::builder()
                .s3_object(
                    aws_sdk_textract::types::S3Object::builder()
                        .bucket("bucket")
                        .name("document.pdf")
                        .build(),
                )
                .build(),
        )
        .feature_types(aws_sdk_textract::types::FeatureType::Tables)
        .client_request_token("idempotency-token-abc123")
        .send()
        .await
        .expect("start_document_analysis with client token should succeed");

    assert!(resp.job_id().is_some(), "should have a job ID");
    assert!(
        !resp.job_id().unwrap().is_empty(),
        "job ID must be non-empty"
    );
}

#[tokio::test]
async fn test_get_document_analysis_no_next_token() {
    let client = make_textract_client().await;

    let resp = client
        .get_document_analysis()
        .job_id("complete-job-no-pagination")
        .send()
        .await
        .expect("get_document_analysis should succeed");

    // Mock returns all results in one page — no pagination token
    assert!(
        resp.next_token().is_none(),
        "mock should not return a next_token when all results fit in one page"
    );
}

#[tokio::test]
async fn test_get_document_text_detection_no_next_token() {
    let client = make_textract_client().await;

    let resp = client
        .get_document_text_detection()
        .job_id("complete-job-no-pagination")
        .send()
        .await
        .expect("get_document_text_detection should succeed");

    // Mock returns all results in one page — no pagination token
    assert!(
        resp.next_token().is_none(),
        "mock should not return a next_token when all results fit in one page"
    );
}

// ============================================================================
// M7/M8: GetDocumentAnalysis / GetDocumentTextDetection return stored blocks
// ============================================================================

#[tokio::test]
async fn test_get_document_analysis_returns_blocks_after_start() {
    let client = make_textract_client().await;

    // Start an analysis job so the mock stores a PAGE block.
    let start_resp = client
        .start_document_analysis()
        .document_location(
            aws_sdk_textract::types::DocumentLocation::builder()
                .s3_object(
                    aws_sdk_textract::types::S3Object::builder()
                        .bucket("test-bucket")
                        .name("test-doc.pdf")
                        .build(),
                )
                .build(),
        )
        .feature_types(aws_sdk_textract::types::FeatureType::Tables)
        .send()
        .await
        .expect("start_document_analysis should succeed");

    let job_id = start_resp.job_id().expect("must have job ID").to_string();

    // Fetch the results — should have at least one block.
    let get_resp = client
        .get_document_analysis()
        .job_id(&job_id)
        .send()
        .await
        .expect("get_document_analysis should succeed");

    assert_eq!(
        get_resp.job_status(),
        Some(&aws_sdk_textract::types::JobStatus::Succeeded),
    );

    let blocks = get_resp.blocks();
    assert!(
        !blocks.is_empty(),
        "GetDocumentAnalysis must return at least one block for a started job"
    );

    // The mock PAGE block should be present.
    let page_block = blocks
        .iter()
        .find(|b| b.block_type() == Some(&aws_sdk_textract::types::BlockType::Page));
    assert!(page_block.is_some(), "response should contain a PAGE block");
}

#[tokio::test]
async fn test_get_document_text_detection_returns_blocks_after_start() {
    let client = make_textract_client().await;

    // Start a text detection job so the mock stores a PAGE block.
    let start_resp = client
        .start_document_text_detection()
        .document_location(
            aws_sdk_textract::types::DocumentLocation::builder()
                .s3_object(
                    aws_sdk_textract::types::S3Object::builder()
                        .bucket("test-bucket")
                        .name("test-doc.pdf")
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("start_document_text_detection should succeed");

    let job_id = start_resp.job_id().expect("must have job ID").to_string();

    // Fetch the results — should have at least one block.
    let get_resp = client
        .get_document_text_detection()
        .job_id(&job_id)
        .send()
        .await
        .expect("get_document_text_detection should succeed");

    assert_eq!(
        get_resp.job_status(),
        Some(&aws_sdk_textract::types::JobStatus::Succeeded),
    );

    let blocks = get_resp.blocks();
    assert!(
        !blocks.is_empty(),
        "GetDocumentTextDetection must return at least one block for a started job"
    );

    // The mock PAGE block should be present.
    let page_block = blocks
        .iter()
        .find(|b| b.block_type() == Some(&aws_sdk_textract::types::BlockType::Page));
    assert!(page_block.is_some(), "response should contain a PAGE block");
}

#[tokio::test]
async fn test_get_document_analysis_unknown_job_returns_empty_blocks() {
    let client = make_textract_client().await;

    // A job ID that was never started — should return empty blocks (not an error).
    let resp = client
        .get_document_analysis()
        .job_id("unknown-job-id-does-not-exist")
        .send()
        .await
        .expect("get_document_analysis should succeed even for unknown job IDs");

    assert_eq!(
        resp.job_status(),
        Some(&aws_sdk_textract::types::JobStatus::Succeeded),
    );

    assert!(
        resp.blocks().is_empty(),
        "unknown job ID should yield empty blocks"
    );
}

#[tokio::test]
async fn test_get_document_text_detection_unknown_job_returns_empty_blocks() {
    let client = make_textract_client().await;

    // A job ID that was never started — should return empty blocks (not an error).
    let resp = client
        .get_document_text_detection()
        .job_id("unknown-job-id-does-not-exist")
        .send()
        .await
        .expect("get_document_text_detection should succeed even for unknown job IDs");

    assert_eq!(
        resp.job_status(),
        Some(&aws_sdk_textract::types::JobStatus::Succeeded),
    );

    assert!(
        resp.blocks().is_empty(),
        "unknown job ID should yield empty blocks"
    );
}
