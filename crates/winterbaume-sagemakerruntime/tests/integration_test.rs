use aws_sdk_sagemakerruntime::config::BehaviorVersion;
use aws_sdk_sagemakerruntime::primitives::Blob;
use winterbaume_core::MockAws;
use winterbaume_sagemakerruntime::SageMakerRuntimeService;

async fn make_client() -> aws_sdk_sagemakerruntime::Client {
    let mock = MockAws::builder()
        .with_service(SageMakerRuntimeService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sagemakerruntime::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_sagemakerruntime::Client::new(&config)
}

#[tokio::test]
async fn test_invoke_endpoint_basic() {
    let client = make_client().await;

    let resp = client
        .invoke_endpoint()
        .endpoint_name("my-endpoint")
        .content_type("application/json")
        .body(Blob::new(b"{\"input\": \"hello\"}"))
        .send()
        .await
        .expect("invoke_endpoint should succeed");

    // Body is no longer stored or echoed by the mock
    let body_bytes = resp.body().map(|b| b.as_ref().to_vec()).unwrap_or_default();
    assert_eq!(body_bytes, b"");
}

#[tokio::test]
async fn test_invoke_endpoint_with_custom_attributes() {
    let client = make_client().await;

    let resp = client
        .invoke_endpoint()
        .endpoint_name("my-endpoint")
        .content_type("application/json")
        .custom_attributes("custom-attr-value")
        .body(Blob::new(b"test-body"))
        .send()
        .await
        .expect("invoke_endpoint with custom attributes should succeed");

    // Body is no longer stored or echoed by the mock
    assert_eq!(resp.custom_attributes(), Some("custom-attr-value"));
}

#[tokio::test]
async fn test_invoke_endpoint_with_target_model() {
    let client = make_client().await;

    let resp = client
        .invoke_endpoint()
        .endpoint_name("multi-model-endpoint")
        .content_type("application/json")
        .target_model("model-a.tar.gz")
        .body(Blob::new(b"{\"data\": 42}"))
        .send()
        .await
        .expect("invoke_endpoint with target model should succeed");

    // Body is no longer stored or echoed by the mock
    let _ = resp;
}

#[tokio::test]
async fn test_invoke_endpoint_returns_production_variant() {
    let client = make_client().await;

    let resp = client
        .invoke_endpoint()
        .endpoint_name("variant-endpoint")
        .content_type("text/plain")
        .body(Blob::new(b"payload"))
        .send()
        .await
        .expect("invoke_endpoint should succeed");

    assert_eq!(resp.invoked_production_variant(), Some("AllTraffic"));
}

#[tokio::test]
async fn test_invoke_endpoint_async_basic() {
    let client = make_client().await;

    let resp = client
        .invoke_endpoint_async()
        .endpoint_name("async-endpoint")
        .content_type("application/json")
        .input_location("s3://my-bucket/input/data.json")
        .send()
        .await
        .expect("invoke_endpoint_async should succeed");

    let output_location = resp.output_location();
    assert!(
        output_location.is_some(),
        "output_location should be present"
    );
    assert!(
        output_location.unwrap().contains("async-endpoint"),
        "output_location should contain the endpoint name"
    );
}

#[tokio::test]
async fn test_invoke_endpoint_async_with_inference_id() {
    let client = make_client().await;

    let resp = client
        .invoke_endpoint_async()
        .endpoint_name("async-endpoint-2")
        .content_type("application/json")
        .inference_id("my-inference-123")
        .input_location("s3://bucket/input.json")
        .send()
        .await
        .expect("invoke_endpoint_async with inference_id should succeed");

    assert!(resp.output_location().is_some());
    assert_eq!(resp.inference_id(), Some("my-inference-123"));
}

#[tokio::test]
async fn test_invoke_endpoint_different_content_types() {
    let client = make_client().await;

    // Test with CSV content type
    let resp = client
        .invoke_endpoint()
        .endpoint_name("csv-endpoint")
        .content_type("text/csv")
        .body(Blob::new(b"1,2,3\n4,5,6"))
        .send()
        .await
        .expect("invoke_endpoint with CSV should succeed");

    // Body is no longer stored or echoed by the mock
    assert_eq!(resp.content_type(), Some("text/csv"));
}

#[tokio::test]
async fn test_invoke_endpoint_empty_body() {
    let client = make_client().await;

    let resp = client
        .invoke_endpoint()
        .endpoint_name("empty-body-endpoint")
        .content_type("application/json")
        .body(Blob::new(b""))
        .send()
        .await
        .expect("invoke_endpoint with empty body should succeed");

    let body = resp.body();
    // Empty body may be None or Some with empty bytes
    if let Some(b) = body {
        assert_eq!(b.as_ref(), b"");
    }
}

// ============================================================================
// Tests derived from AWS documentation: Amazon SageMaker Runtime
// ============================================================================

#[tokio::test]
async fn test_invoke_endpoint_with_target_variant() {
    let client = make_client().await;

    let resp = client
        .invoke_endpoint()
        .endpoint_name("variant-endpoint")
        .content_type("application/json")
        .target_variant("MyVariant")
        .body(Blob::new(b"{\"x\": 1}"))
        .send()
        .await
        .expect("invoke_endpoint with target_variant should succeed");

    // Body is no longer stored or echoed by the mock
    assert_eq!(resp.invoked_production_variant(), Some("AllTraffic"));
}

#[tokio::test]
async fn test_invoke_endpoint_with_inference_id() {
    let client = make_client().await;

    let resp = client
        .invoke_endpoint()
        .endpoint_name("my-endpoint")
        .content_type("application/json")
        .inference_id("infer-abc-123")
        .body(Blob::new(b"{\"val\": 42}"))
        .send()
        .await
        .expect("invoke_endpoint with inference_id should succeed");

    // Body is no longer stored or echoed by the mock
    let _ = resp;
}

#[tokio::test]
async fn test_invoke_endpoint_with_inference_component() {
    let client = make_client().await;

    let resp = client
        .invoke_endpoint()
        .endpoint_name("multi-component-endpoint")
        .content_type("application/json")
        .inference_component_name("component-a")
        .body(Blob::new(b"{\"data\": \"hello\"}"))
        .send()
        .await
        .expect("invoke_endpoint with inference_component_name should succeed");

    // Body is no longer stored or echoed by the mock
    let _ = resp;
}

#[tokio::test]
async fn test_invoke_endpoint_binary_body() {
    let client = make_client().await;

    // Binary payload with non-UTF8 bytes
    let binary_payload: Vec<u8> = (0u8..=255u8).collect();
    let resp = client
        .invoke_endpoint()
        .endpoint_name("binary-endpoint")
        .content_type("application/octet-stream")
        .body(Blob::new(binary_payload.clone()))
        .send()
        .await
        .expect("invoke_endpoint with binary body should succeed");

    // Body is no longer stored or echoed by the mock
    let body_bytes = resp.body().map(|b| b.as_ref().to_vec()).unwrap_or_default();
    assert_eq!(body_bytes, b"");
}

#[tokio::test]
async fn test_invoke_endpoint_large_body() {
    let client = make_client().await;

    // 10 KB payload
    let large_payload: Vec<u8> = (0u8..255u8).cycle().take(10 * 1024).collect();
    let resp = client
        .invoke_endpoint()
        .endpoint_name("large-body-endpoint")
        .content_type("application/octet-stream")
        .body(Blob::new(large_payload.clone()))
        .send()
        .await
        .expect("invoke_endpoint with large body should succeed");

    // Body is no longer stored or echoed by the mock
    let body_bytes = resp.body().map(|b| b.as_ref().to_vec()).unwrap_or_default();
    assert_eq!(body_bytes, b"");
}

#[tokio::test]
async fn test_invoke_endpoint_multiple_calls_same_endpoint() {
    let client = make_client().await;

    let endpoint = "shared-endpoint";
    for i in 0u32..3 {
        let payload = format!("{{\"call\": {}}}", i);
        let resp = client
            .invoke_endpoint()
            .endpoint_name(endpoint)
            .content_type("application/json")
            .body(Blob::new(payload.as_bytes().to_vec()))
            .send()
            .await
            .unwrap_or_else(|e| panic!("call {} should succeed: {e:?}", i));

        // Body is no longer stored or echoed by the mock
        let _ = resp;
    }
}

#[tokio::test]
async fn test_invoke_endpoint_async_output_location_format() {
    let client = make_client().await;

    let resp = client
        .invoke_endpoint_async()
        .endpoint_name("format-check-endpoint")
        .content_type("application/json")
        .input_location("s3://my-bucket/input.json")
        .send()
        .await
        .expect("invoke_endpoint_async should succeed");

    let output_location = resp
        .output_location()
        .expect("output_location must be present");
    assert!(
        output_location.starts_with("s3://"),
        "output_location should be a valid S3 URI, got: {output_location}"
    );
}

#[tokio::test]
async fn test_invoke_endpoint_async_sequential_unique_output_locations() {
    let client = make_client().await;

    let resp1 = client
        .invoke_endpoint_async()
        .endpoint_name("seq-endpoint")
        .input_location("s3://bucket/input1.json")
        .send()
        .await
        .expect("first invoke_endpoint_async should succeed");

    let resp2 = client
        .invoke_endpoint_async()
        .endpoint_name("seq-endpoint")
        .input_location("s3://bucket/input2.json")
        .send()
        .await
        .expect("second invoke_endpoint_async should succeed");

    let loc1 = resp1
        .output_location()
        .expect("output_location 1 must be present");
    let loc2 = resp2
        .output_location()
        .expect("output_location 2 must be present");
    assert_ne!(
        loc1, loc2,
        "sequential async invocations should produce distinct output locations"
    );
}

#[tokio::test]
async fn test_invoke_endpoint_async_auto_generated_inference_id() {
    let client = make_client().await;

    // No inference_id supplied — mock should auto-generate (or return empty string, but must not fail)
    let resp = client
        .invoke_endpoint_async()
        .endpoint_name("autoid-endpoint")
        .input_location("s3://bucket/input.json")
        .send()
        .await
        .expect("invoke_endpoint_async without inference_id should succeed");

    // output_location must always be present
    assert!(
        resp.output_location().is_some(),
        "output_location should be present even without inference_id"
    );
}

#[tokio::test]
async fn test_invoke_endpoint_with_response_stream_basic() {
    let client = make_client().await;

    let resp = client
        .invoke_endpoint_with_response_stream()
        .endpoint_name("stream-endpoint")
        .content_type("application/json")
        .body(Blob::new(b"{\"stream\": true}"))
        .send()
        .await
        .expect("invoke_endpoint_with_response_stream should succeed");

    assert_eq!(
        resp.invoked_production_variant(),
        Some("AllTraffic"),
        "invoked_production_variant should be AllTraffic"
    );
}

#[tokio::test]
async fn test_invoke_endpoint_with_response_stream_custom_attributes() {
    let client = make_client().await;

    let resp = client
        .invoke_endpoint_with_response_stream()
        .endpoint_name("stream-endpoint")
        .content_type("application/json")
        .custom_attributes("trace-id-xyz")
        .body(Blob::new(b"{\"x\": 1}"))
        .send()
        .await
        .expect("invoke_endpoint_with_response_stream with custom_attributes should succeed");

    assert_eq!(
        resp.custom_attributes(),
        Some("trace-id-xyz"),
        "custom_attributes should be echoed back"
    );
}

#[tokio::test]
async fn test_invoke_endpoint_with_response_stream_content_type() {
    let client = make_client().await;

    let resp = client
        .invoke_endpoint_with_response_stream()
        .endpoint_name("stream-endpoint")
        .content_type("text/plain")
        .body(Blob::new(b"hello stream"))
        .send()
        .await
        .expect("invoke_endpoint_with_response_stream should succeed");

    assert_eq!(
        resp.content_type(),
        Some("text/plain"),
        "content_type should be echoed back"
    );
}
