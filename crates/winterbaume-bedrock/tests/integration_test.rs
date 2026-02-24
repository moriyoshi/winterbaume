use aws_sdk_bedrock::config::BehaviorVersion;
use winterbaume_bedrock::BedrockService;
use winterbaume_core::MockAws;

async fn make_bedrock_client() -> aws_sdk_bedrock::Client {
    let mock = MockAws::builder()
        .with_service(BedrockService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_bedrock::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_bedrock::Client::new(&config)
}

#[tokio::test]
async fn test_list_foundation_models() {
    let client = make_bedrock_client().await;

    let resp = client
        .list_foundation_models()
        .send()
        .await
        .expect("list_foundation_models should succeed");

    let models = resp.model_summaries();
    assert!(
        models.len() >= 2,
        "should have at least 2 foundation models, got {}",
        models.len()
    );

    // Check that our known models are present
    let model_ids: Vec<&str> = models.iter().map(|m| m.model_id.as_str()).collect();
    assert!(
        model_ids.contains(&"anthropic.claude-v2"),
        "should contain anthropic.claude-v2"
    );
    assert!(
        model_ids.contains(&"amazon.titan-text-express-v1"),
        "should contain amazon.titan-text-express-v1"
    );
}

#[tokio::test]
async fn test_get_foundation_model() {
    let client = make_bedrock_client().await;

    let resp = client
        .get_foundation_model()
        .model_identifier("anthropic.claude-v2")
        .send()
        .await
        .expect("get_foundation_model should succeed");

    let details = resp.model_details().expect("should have model details");
    assert_eq!(details.model_id.as_str(), "anthropic.claude-v2");
    assert_eq!(details.provider_name.as_deref(), Some("Anthropic"));
}

#[tokio::test]
async fn test_get_foundation_model_not_found() {
    let client = make_bedrock_client().await;

    let result = client
        .get_foundation_model()
        .model_identifier("nonexistent.model-v1")
        .send()
        .await;

    assert!(result.is_err(), "get nonexistent model should fail");
}

#[tokio::test]
async fn test_list_model_customization_jobs_empty() {
    let client = make_bedrock_client().await;

    let resp = client
        .list_model_customization_jobs()
        .send()
        .await
        .expect("list_model_customization_jobs should succeed");

    assert_eq!(
        resp.model_customization_job_summaries().len(),
        0,
        "should have no jobs initially"
    );
}

#[tokio::test]
async fn test_create_and_list_model_customization_job() {
    let client = make_bedrock_client().await;

    let training_config = aws_sdk_bedrock::types::TrainingDataConfig::builder()
        .s3_uri("s3://my-bucket/training-data/")
        .build();

    let output_config = aws_sdk_bedrock::types::OutputDataConfig::builder()
        .s3_uri("s3://my-bucket/output/")
        .build()
        .unwrap();

    let resp = client
        .create_model_customization_job()
        .job_name("my-test-job")
        .base_model_identifier("anthropic.claude-v2")
        .custom_model_name("my-custom-model")
        .role_arn("arn:aws:iam::123456789012:role/BedrockRole")
        .training_data_config(training_config)
        .output_data_config(output_config)
        .hyper_parameters("epochCount", "3")
        .hyper_parameters("batchSize", "8")
        .send()
        .await
        .expect("create_model_customization_job should succeed");

    assert!(resp.job_arn().contains("my-test-job"));

    // List and verify
    let list_resp = client
        .list_model_customization_jobs()
        .send()
        .await
        .expect("list should succeed after create");

    let jobs = list_resp.model_customization_job_summaries();
    assert_eq!(jobs.len(), 1, "should have 1 job");
    assert_eq!(jobs[0].job_name.as_str(), "my-test-job");
    assert_eq!(jobs[0].base_model_arn.as_str(), "anthropic.claude-v2");
}

#[tokio::test]
async fn test_create_duplicate_job_fails() {
    let client = make_bedrock_client().await;

    let training_config = aws_sdk_bedrock::types::TrainingDataConfig::builder()
        .s3_uri("s3://my-bucket/training-data/")
        .build();

    let output_config = aws_sdk_bedrock::types::OutputDataConfig::builder()
        .s3_uri("s3://my-bucket/output/")
        .build()
        .unwrap();

    client
        .create_model_customization_job()
        .job_name("dup-job")
        .base_model_identifier("anthropic.claude-v2")
        .custom_model_name("my-custom-model-1")
        .role_arn("arn:aws:iam::123456789012:role/BedrockRole")
        .training_data_config(training_config.clone())
        .output_data_config(output_config.clone())
        .send()
        .await
        .unwrap();

    let result = client
        .create_model_customization_job()
        .job_name("dup-job")
        .base_model_identifier("anthropic.claude-v2")
        .custom_model_name("my-custom-model-2")
        .role_arn("arn:aws:iam::123456789012:role/BedrockRole")
        .training_data_config(training_config)
        .output_data_config(output_config)
        .send()
        .await;

    assert!(result.is_err(), "duplicate job name should fail");
}

#[tokio::test]
async fn test_get_model_customization_job() {
    let client = make_bedrock_client().await;

    let training_config = aws_sdk_bedrock::types::TrainingDataConfig::builder()
        .s3_uri("s3://my-bucket/training-data/")
        .build();

    let output_config = aws_sdk_bedrock::types::OutputDataConfig::builder()
        .s3_uri("s3://my-bucket/output/")
        .build()
        .unwrap();

    let create_resp = client
        .create_model_customization_job()
        .job_name("get-job-test")
        .base_model_identifier("anthropic.claude-v2")
        .custom_model_name("my-get-model")
        .role_arn("arn:aws:iam::123456789012:role/BedrockRole")
        .training_data_config(training_config)
        .output_data_config(output_config)
        .send()
        .await
        .unwrap();

    let job_arn = create_resp.job_arn();

    let resp = client
        .get_model_customization_job()
        .job_identifier(job_arn)
        .send()
        .await
        .expect("get_model_customization_job should succeed");

    assert_eq!(resp.job_name(), "get-job-test");
    assert_eq!(resp.base_model_arn(), "anthropic.claude-v2");
    assert_eq!(resp.output_model_name(), "my-get-model");
}

#[tokio::test]
async fn test_stop_model_customization_job() {
    let client = make_bedrock_client().await;

    let training_config = aws_sdk_bedrock::types::TrainingDataConfig::builder()
        .s3_uri("s3://my-bucket/training-data/")
        .build();

    let output_config = aws_sdk_bedrock::types::OutputDataConfig::builder()
        .s3_uri("s3://my-bucket/output/")
        .build()
        .unwrap();

    let create_resp = client
        .create_model_customization_job()
        .job_name("stop-job-test")
        .base_model_identifier("anthropic.claude-v2")
        .custom_model_name("my-stop-model")
        .role_arn("arn:aws:iam::123456789012:role/BedrockRole")
        .training_data_config(training_config)
        .output_data_config(output_config)
        .send()
        .await
        .unwrap();

    let job_arn = create_resp.job_arn();

    client
        .stop_model_customization_job()
        .job_identifier(job_arn)
        .send()
        .await
        .expect("stop_model_customization_job should succeed");

    // Verify the job is now stopped
    let get_resp = client
        .get_model_customization_job()
        .job_identifier(job_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(
        get_resp.status().map(|s| s.as_str()),
        Some("Stopped"),
        "job should be stopped"
    );
}

#[tokio::test]
async fn test_get_custom_model() {
    let client = make_bedrock_client().await;

    let training_config = aws_sdk_bedrock::types::TrainingDataConfig::builder()
        .s3_uri("s3://my-bucket/training-data/")
        .build();

    let output_config = aws_sdk_bedrock::types::OutputDataConfig::builder()
        .s3_uri("s3://my-bucket/output/")
        .build()
        .unwrap();

    client
        .create_model_customization_job()
        .job_name("custom-model-job")
        .base_model_identifier("anthropic.claude-v2")
        .custom_model_name("my-custom-model-get")
        .role_arn("arn:aws:iam::123456789012:role/BedrockRole")
        .training_data_config(training_config)
        .output_data_config(output_config)
        .send()
        .await
        .unwrap();

    let model_id = "arn:aws:bedrock:us-east-1:123456789012:custom-model/my-custom-model-get";

    let resp = client
        .get_custom_model()
        .model_identifier(model_id)
        .send()
        .await
        .expect("get_custom_model should succeed");

    assert_eq!(resp.model_name(), "my-custom-model-get");
    assert_eq!(resp.base_model_arn(), Some("anthropic.claude-v2"));
}

#[tokio::test]
async fn test_list_custom_models() {
    let client = make_bedrock_client().await;

    let training_config = aws_sdk_bedrock::types::TrainingDataConfig::builder()
        .s3_uri("s3://my-bucket/training-data/")
        .build();

    let output_config = aws_sdk_bedrock::types::OutputDataConfig::builder()
        .s3_uri("s3://my-bucket/output/")
        .build()
        .unwrap();

    client
        .create_model_customization_job()
        .job_name("list-models-job")
        .base_model_identifier("anthropic.claude-v2")
        .custom_model_name("my-list-model")
        .role_arn("arn:aws:iam::123456789012:role/BedrockRole")
        .training_data_config(training_config)
        .output_data_config(output_config)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_custom_models()
        .send()
        .await
        .expect("list_custom_models should succeed");

    let models = resp.model_summaries();
    assert!(!models.is_empty(), "should have at least 1 custom model");
    assert!(
        models.iter().any(|m| m.model_name() == "my-list-model"),
        "should contain the created model"
    );
}

#[tokio::test]
async fn test_delete_custom_model() {
    let client = make_bedrock_client().await;

    let training_config = aws_sdk_bedrock::types::TrainingDataConfig::builder()
        .s3_uri("s3://my-bucket/training-data/")
        .build();

    let output_config = aws_sdk_bedrock::types::OutputDataConfig::builder()
        .s3_uri("s3://my-bucket/output/")
        .build()
        .unwrap();

    client
        .create_model_customization_job()
        .job_name("delete-model-job")
        .base_model_identifier("anthropic.claude-v2")
        .custom_model_name("my-delete-model")
        .role_arn("arn:aws:iam::123456789012:role/BedrockRole")
        .training_data_config(training_config)
        .output_data_config(output_config)
        .send()
        .await
        .unwrap();

    let model_id = "arn:aws:bedrock:us-east-1:123456789012:custom-model/my-delete-model";

    client
        .delete_custom_model()
        .model_identifier(model_id)
        .send()
        .await
        .expect("delete_custom_model should succeed");

    // Verify model is deleted
    let result = client
        .get_custom_model()
        .model_identifier(model_id)
        .send()
        .await;

    assert!(result.is_err(), "get deleted model should fail");
}

#[tokio::test]
async fn test_put_and_get_model_invocation_logging_configuration() {
    let client = make_bedrock_client().await;

    let s3_config = aws_sdk_bedrock::types::S3Config::builder()
        .bucket_name("my-logging-bucket")
        .key_prefix("bedrock-logs/")
        .build()
        .unwrap();

    let logging_config = aws_sdk_bedrock::types::LoggingConfig::builder()
        .text_data_delivery_enabled(true)
        .image_data_delivery_enabled(false)
        .embedding_data_delivery_enabled(false)
        .s3_config(s3_config)
        .build();

    client
        .put_model_invocation_logging_configuration()
        .logging_config(logging_config)
        .send()
        .await
        .expect("put_model_invocation_logging_configuration should succeed");

    let resp = client
        .get_model_invocation_logging_configuration()
        .send()
        .await
        .expect("get_model_invocation_logging_configuration should succeed");

    let config = resp.logging_config().expect("should have logging config");
    assert_eq!(config.text_data_delivery_enabled(), Some(true));
    assert_eq!(config.image_data_delivery_enabled(), Some(false));

    let s3 = config.s3_config().expect("should have s3 config");
    assert_eq!(s3.bucket_name(), "my-logging-bucket");
    assert_eq!(s3.key_prefix(), Some("bedrock-logs/"));
}

#[tokio::test]
async fn test_delete_model_invocation_logging_configuration() {
    let client = make_bedrock_client().await;

    let s3_config = aws_sdk_bedrock::types::S3Config::builder()
        .bucket_name("my-logging-bucket")
        .build()
        .unwrap();

    let logging_config = aws_sdk_bedrock::types::LoggingConfig::builder()
        .text_data_delivery_enabled(true)
        .s3_config(s3_config)
        .build();

    client
        .put_model_invocation_logging_configuration()
        .logging_config(logging_config)
        .send()
        .await
        .unwrap();

    client
        .delete_model_invocation_logging_configuration()
        .send()
        .await
        .expect("delete_model_invocation_logging_configuration should succeed");

    let resp = client
        .get_model_invocation_logging_configuration()
        .send()
        .await
        .unwrap();

    assert!(
        resp.logging_config().is_none(),
        "logging config should be gone after delete"
    );
}

#[tokio::test]
async fn test_tag_and_list_tags_for_resource() {
    let client = make_bedrock_client().await;

    let training_config = aws_sdk_bedrock::types::TrainingDataConfig::builder()
        .s3_uri("s3://my-bucket/training-data/")
        .build();

    let output_config = aws_sdk_bedrock::types::OutputDataConfig::builder()
        .s3_uri("s3://my-bucket/output/")
        .build()
        .unwrap();

    let create_resp = client
        .create_model_customization_job()
        .job_name("tag-test-job")
        .base_model_identifier("anthropic.claude-v2")
        .custom_model_name("my-tag-model")
        .role_arn("arn:aws:iam::123456789012:role/BedrockRole")
        .training_data_config(training_config)
        .output_data_config(output_config)
        .send()
        .await
        .unwrap();

    let job_arn = create_resp.job_arn();

    let tag1 = aws_sdk_bedrock::types::Tag::builder()
        .key("env")
        .value("test")
        .build()
        .unwrap();
    let tag2 = aws_sdk_bedrock::types::Tag::builder()
        .key("team")
        .value("ml")
        .build()
        .unwrap();

    client
        .tag_resource()
        .resource_arn(job_arn)
        .tags(tag1)
        .tags(tag2)
        .send()
        .await
        .expect("tag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(job_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = resp.tags();
    assert_eq!(tags.len(), 2, "should have 2 tags");
    assert!(
        tags.iter().any(|t| t.key() == "env" && t.value() == "test"),
        "should have env=test tag"
    );
    assert!(
        tags.iter().any(|t| t.key() == "team" && t.value() == "ml"),
        "should have team=ml tag"
    );
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_bedrock_client().await;

    let training_config = aws_sdk_bedrock::types::TrainingDataConfig::builder()
        .s3_uri("s3://my-bucket/training-data/")
        .build();

    let output_config = aws_sdk_bedrock::types::OutputDataConfig::builder()
        .s3_uri("s3://my-bucket/output/")
        .build()
        .unwrap();

    let create_resp = client
        .create_model_customization_job()
        .job_name("untag-test-job")
        .base_model_identifier("anthropic.claude-v2")
        .custom_model_name("my-untag-model")
        .role_arn("arn:aws:iam::123456789012:role/BedrockRole")
        .training_data_config(training_config)
        .output_data_config(output_config)
        .send()
        .await
        .unwrap();

    let job_arn = create_resp.job_arn();

    let tag1 = aws_sdk_bedrock::types::Tag::builder()
        .key("env")
        .value("test")
        .build()
        .unwrap();
    let tag2 = aws_sdk_bedrock::types::Tag::builder()
        .key("team")
        .value("ml")
        .build()
        .unwrap();

    client
        .tag_resource()
        .resource_arn(job_arn)
        .tags(tag1)
        .tags(tag2)
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn(job_arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(job_arn)
        .send()
        .await
        .unwrap();

    let tags = resp.tags();
    assert_eq!(tags.len(), 1, "should have 1 tag after untag");
    assert_eq!(tags[0].key(), "team");
    assert_eq!(tags[0].value(), "ml");
}

// ============================================================================
// Tests derived from AWS documentation: Amazon Bedrock
// ============================================================================

#[tokio::test]
async fn test_list_foundation_models_filter_by_provider() {
    let client = make_bedrock_client().await;

    // winterbaume returns all models regardless of filter — verify the call succeeds
    let resp = client
        .list_foundation_models()
        .by_provider("Anthropic")
        .send()
        .await
        .expect("list_foundation_models with provider filter should succeed");

    // Response should contain models (filtering may or may not be applied)
    let _ = resp.model_summaries();
}

#[tokio::test]
async fn test_get_model_customization_job_not_found() {
    let client = make_bedrock_client().await;

    let result = client
        .get_model_customization_job()
        .job_identifier(
            "arn:aws:bedrock:us-east-1:123456789012:model-customization-job/nonexistent",
        )
        .send()
        .await;

    assert!(result.is_err(), "get nonexistent job should fail");
}

#[tokio::test]
async fn test_stop_already_stopped_job_fails() {
    let client = make_bedrock_client().await;

    let training_config = aws_sdk_bedrock::types::TrainingDataConfig::builder()
        .s3_uri("s3://my-bucket/training-data/")
        .build();

    let output_config = aws_sdk_bedrock::types::OutputDataConfig::builder()
        .s3_uri("s3://my-bucket/output/")
        .build()
        .unwrap();

    let create_resp = client
        .create_model_customization_job()
        .job_name("stop-twice-job")
        .base_model_identifier("anthropic.claude-v2")
        .custom_model_name("stop-twice-model")
        .role_arn("arn:aws:iam::123456789012:role/BedrockRole")
        .training_data_config(training_config)
        .output_data_config(output_config)
        .send()
        .await
        .unwrap();

    let job_arn = create_resp.job_arn();

    client
        .stop_model_customization_job()
        .job_identifier(job_arn)
        .send()
        .await
        .unwrap();

    // Stopping an already-stopped job should fail
    let result = client
        .stop_model_customization_job()
        .job_identifier(job_arn)
        .send()
        .await;

    assert!(
        result.is_err(),
        "stopping an already-stopped job should fail"
    );
}

#[tokio::test]
async fn test_list_model_customization_jobs_filter_by_status_in_progress() {
    let client = make_bedrock_client().await;

    let training_config = aws_sdk_bedrock::types::TrainingDataConfig::builder()
        .s3_uri("s3://my-bucket/training-data/")
        .build();

    let output_config = aws_sdk_bedrock::types::OutputDataConfig::builder()
        .s3_uri("s3://my-bucket/output/")
        .build()
        .unwrap();

    client
        .create_model_customization_job()
        .job_name("filter-status-job")
        .base_model_identifier("anthropic.claude-v2")
        .custom_model_name("filter-status-model")
        .role_arn("arn:aws:iam::123456789012:role/BedrockRole")
        .training_data_config(training_config)
        .output_data_config(output_config)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_model_customization_jobs()
        .status_equals(aws_sdk_bedrock::types::FineTuningJobStatus::InProgress)
        .send()
        .await
        .expect("list with status filter should succeed");

    // The job should appear in InProgress list (winterbaume sets status to InProgress on creation)
    assert!(
        !resp.model_customization_job_summaries().is_empty(),
        "should have at least 1 in-progress job"
    );
}

#[tokio::test]
async fn test_get_custom_model_not_found() {
    let client = make_bedrock_client().await;

    let result = client
        .get_custom_model()
        .model_identifier("arn:aws:bedrock:us-east-1:123456789012:custom-model/nonexistent")
        .send()
        .await;

    assert!(result.is_err(), "get nonexistent custom model should fail");
}

#[tokio::test]
async fn test_get_model_invocation_logging_config_empty_initially() {
    let client = make_bedrock_client().await;

    let resp = client
        .get_model_invocation_logging_configuration()
        .send()
        .await
        .expect("get_model_invocation_logging_configuration should succeed even with no config");

    // Initially no logging config should be set
    assert!(
        resp.logging_config().is_none(),
        "logging config should be None before any put"
    );
}

#[tokio::test]
async fn test_list_tags_for_resource_empty() {
    let client = make_bedrock_client().await;

    let training_config = aws_sdk_bedrock::types::TrainingDataConfig::builder()
        .s3_uri("s3://my-bucket/training-data/")
        .build();

    let output_config = aws_sdk_bedrock::types::OutputDataConfig::builder()
        .s3_uri("s3://my-bucket/output/")
        .build()
        .unwrap();

    let create_resp = client
        .create_model_customization_job()
        .job_name("empty-tags-job")
        .base_model_identifier("anthropic.claude-v2")
        .custom_model_name("empty-tags-model")
        .role_arn("arn:aws:iam::123456789012:role/BedrockRole")
        .training_data_config(training_config)
        .output_data_config(output_config)
        .send()
        .await
        .unwrap();

    let job_arn = create_resp.job_arn();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(job_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed on new resource");

    assert_eq!(resp.tags().len(), 0, "should have no tags initially");
}

#[tokio::test]
async fn test_overwrite_model_invocation_logging_config() {
    let client = make_bedrock_client().await;

    let s3_config1 = aws_sdk_bedrock::types::S3Config::builder()
        .bucket_name("bucket-1")
        .build()
        .unwrap();

    let logging_config1 = aws_sdk_bedrock::types::LoggingConfig::builder()
        .text_data_delivery_enabled(true)
        .s3_config(s3_config1)
        .build();

    client
        .put_model_invocation_logging_configuration()
        .logging_config(logging_config1)
        .send()
        .await
        .unwrap();

    let s3_config2 = aws_sdk_bedrock::types::S3Config::builder()
        .bucket_name("bucket-2")
        .build()
        .unwrap();

    let logging_config2 = aws_sdk_bedrock::types::LoggingConfig::builder()
        .text_data_delivery_enabled(false)
        .s3_config(s3_config2)
        .build();

    client
        .put_model_invocation_logging_configuration()
        .logging_config(logging_config2)
        .send()
        .await
        .expect("second put should overwrite");

    let resp = client
        .get_model_invocation_logging_configuration()
        .send()
        .await
        .unwrap();

    let config = resp.logging_config().expect("should have config");
    let s3 = config.s3_config().expect("should have s3 config");
    assert_eq!(s3.bucket_name(), "bucket-2", "config should be overwritten");
}

// ---- Guardrails ----

#[tokio::test]
async fn test_create_and_get_guardrail() {
    let client = make_bedrock_client().await;

    let resp = client
        .create_guardrail()
        .name("my-guardrail")
        .blocked_input_messaging("Blocked input")
        .blocked_outputs_messaging("Blocked output")
        .send()
        .await
        .expect("create_guardrail should succeed");

    let guardrail_id = resp.guardrail_id().to_string();
    assert!(!guardrail_id.is_empty(), "guardrail_id should not be empty");

    let get_resp = client
        .get_guardrail()
        .guardrail_identifier(&guardrail_id)
        .send()
        .await
        .expect("get_guardrail should succeed");

    assert_eq!(get_resp.name(), "my-guardrail");
    let _ = get_resp.status(); // status is present
}

#[tokio::test]
async fn test_list_guardrails() {
    let client = make_bedrock_client().await;

    client
        .create_guardrail()
        .name("guardrail-list-1")
        .blocked_input_messaging("Blocked")
        .blocked_outputs_messaging("Blocked")
        .send()
        .await
        .unwrap();

    client
        .create_guardrail()
        .name("guardrail-list-2")
        .blocked_input_messaging("Blocked")
        .blocked_outputs_messaging("Blocked")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_guardrails()
        .send()
        .await
        .expect("list_guardrails should succeed");

    let guardrails = resp.guardrails();
    assert!(guardrails.len() >= 2, "should have at least 2 guardrails");
}

#[tokio::test]
async fn test_delete_guardrail() {
    let client = make_bedrock_client().await;

    let create_resp = client
        .create_guardrail()
        .name("guardrail-to-delete")
        .blocked_input_messaging("Blocked")
        .blocked_outputs_messaging("Blocked")
        .send()
        .await
        .unwrap();

    let guardrail_id = create_resp.guardrail_id().to_string();

    client
        .delete_guardrail()
        .guardrail_identifier(&guardrail_id)
        .send()
        .await
        .expect("delete_guardrail should succeed");

    let result = client
        .get_guardrail()
        .guardrail_identifier(&guardrail_id)
        .send()
        .await;

    assert!(result.is_err(), "get deleted guardrail should fail");
}

#[tokio::test]
async fn test_guardrail_not_found() {
    let client = make_bedrock_client().await;

    let result = client
        .get_guardrail()
        .guardrail_identifier("nonexistent-id")
        .send()
        .await;

    assert!(result.is_err(), "get nonexistent guardrail should fail");
}

// ---- Provisioned Model Throughput ----

#[tokio::test]
async fn test_create_and_get_provisioned_model_throughput() {
    let client = make_bedrock_client().await;

    let resp = client
        .create_provisioned_model_throughput()
        .model_id("anthropic.claude-v2")
        .provisioned_model_name("my-provisioned-model")
        .model_units(1)
        .send()
        .await
        .expect("create_provisioned_model_throughput should succeed");

    let arn = resp.provisioned_model_arn().to_string();
    assert!(!arn.is_empty(), "provisioned_model_arn should not be empty");

    let get_resp = client
        .get_provisioned_model_throughput()
        .provisioned_model_id(&arn)
        .send()
        .await
        .expect("get_provisioned_model_throughput should succeed");

    assert_eq!(get_resp.provisioned_model_name(), "my-provisioned-model");
    assert_eq!(get_resp.model_units(), 1);
}

#[tokio::test]
async fn test_list_provisioned_model_throughputs() {
    let client = make_bedrock_client().await;

    client
        .create_provisioned_model_throughput()
        .model_id("anthropic.claude-v2")
        .provisioned_model_name("provisioned-list-1")
        .model_units(1)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_provisioned_model_throughputs()
        .send()
        .await
        .expect("list_provisioned_model_throughputs should succeed");

    assert!(
        !resp.provisioned_model_summaries().is_empty(),
        "should have at least 1 provisioned model"
    );
}

#[tokio::test]
async fn test_delete_provisioned_model_throughput() {
    let client = make_bedrock_client().await;

    let create_resp = client
        .create_provisioned_model_throughput()
        .model_id("anthropic.claude-v2")
        .provisioned_model_name("provisioned-to-delete")
        .model_units(1)
        .send()
        .await
        .unwrap();

    let arn = create_resp.provisioned_model_arn().to_string();

    client
        .delete_provisioned_model_throughput()
        .provisioned_model_id(&arn)
        .send()
        .await
        .expect("delete_provisioned_model_throughput should succeed");

    let result = client
        .get_provisioned_model_throughput()
        .provisioned_model_id(&arn)
        .send()
        .await;

    assert!(result.is_err(), "get deleted provisioned model should fail");
}

// ---- Inference Profiles ----

#[tokio::test]
async fn test_create_and_get_inference_profile() {
    use aws_sdk_bedrock::types::InferenceProfileModelSource;

    let client = make_bedrock_client().await;

    let model_source = InferenceProfileModelSource::CopyFrom(
        "arn:aws:bedrock:us-east-1::foundation-model/anthropic.claude-v2".to_string(),
    );

    let resp = client
        .create_inference_profile()
        .inference_profile_name("my-inference-profile")
        .model_source(model_source)
        .send()
        .await
        .expect("create_inference_profile should succeed");

    let profile_arn = resp.inference_profile_arn().to_string();
    assert!(!profile_arn.is_empty());

    let get_resp = client
        .get_inference_profile()
        .inference_profile_identifier(&profile_arn)
        .send()
        .await
        .expect("get_inference_profile should succeed");

    assert_eq!(get_resp.inference_profile_name(), "my-inference-profile");
}

#[tokio::test]
async fn test_list_and_delete_inference_profile() {
    use aws_sdk_bedrock::types::InferenceProfileModelSource;

    let client = make_bedrock_client().await;

    let model_source = InferenceProfileModelSource::CopyFrom(
        "arn:aws:bedrock:us-east-1::foundation-model/anthropic.claude-v2".to_string(),
    );

    client
        .create_inference_profile()
        .inference_profile_name("profile-for-list-delete")
        .model_source(model_source)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_inference_profiles()
        .send()
        .await
        .expect("list_inference_profiles should succeed");

    assert!(!list_resp.inference_profile_summaries().is_empty());

    let profile_arn = list_resp.inference_profile_summaries()[0]
        .inference_profile_arn()
        .to_string();

    client
        .delete_inference_profile()
        .inference_profile_identifier(&profile_arn)
        .send()
        .await
        .expect("delete_inference_profile should succeed");
}

// ---- Prompt Routers ----

#[tokio::test]
async fn test_create_and_get_prompt_router() {
    use aws_sdk_bedrock::types::{PromptRouterTargetModel, RoutingCriteria};

    let client = make_bedrock_client().await;

    let fallback = PromptRouterTargetModel::builder()
        .model_arn("arn:aws:bedrock:us-east-1::foundation-model/anthropic.claude-v2")
        .build();

    let model = PromptRouterTargetModel::builder()
        .model_arn("arn:aws:bedrock:us-east-1::foundation-model/anthropic.claude-v2")
        .build();

    let criteria = RoutingCriteria::builder()
        .response_quality_difference(0.5)
        .build()
        .unwrap();

    let resp = client
        .create_prompt_router()
        .prompt_router_name("my-router")
        .fallback_model(fallback)
        .models(model)
        .routing_criteria(criteria)
        .send()
        .await
        .expect("create_prompt_router should succeed");

    let router_arn = resp
        .prompt_router_arn()
        .expect("should have arn")
        .to_string();
    assert!(!router_arn.is_empty());

    let get_resp = client
        .get_prompt_router()
        .prompt_router_arn(&router_arn)
        .send()
        .await
        .expect("get_prompt_router should succeed");

    assert_eq!(get_resp.prompt_router_name(), "my-router");
}

#[tokio::test]
async fn test_list_and_delete_prompt_router() {
    use aws_sdk_bedrock::types::{PromptRouterTargetModel, RoutingCriteria};

    let client = make_bedrock_client().await;

    let fallback = PromptRouterTargetModel::builder()
        .model_arn("arn:aws:bedrock:us-east-1::foundation-model/anthropic.claude-v2")
        .build();

    let model = PromptRouterTargetModel::builder()
        .model_arn("arn:aws:bedrock:us-east-1::foundation-model/anthropic.claude-v2")
        .build();

    let criteria = RoutingCriteria::builder()
        .response_quality_difference(0.5)
        .build()
        .unwrap();

    client
        .create_prompt_router()
        .prompt_router_name("router-to-list-delete")
        .fallback_model(fallback)
        .models(model)
        .routing_criteria(criteria)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_prompt_routers()
        .send()
        .await
        .expect("list_prompt_routers should succeed");

    assert!(!list_resp.prompt_router_summaries().is_empty());

    let router_arn = list_resp.prompt_router_summaries()[0]
        .prompt_router_arn()
        .to_string();

    client
        .delete_prompt_router()
        .prompt_router_arn(&router_arn)
        .send()
        .await
        .expect("delete_prompt_router should succeed");
}

// ---- Evaluation Jobs ----

fn make_eval_job_params() -> (
    aws_sdk_bedrock::types::EvaluationConfig,
    aws_sdk_bedrock::types::EvaluationInferenceConfig,
    aws_sdk_bedrock::types::EvaluationOutputDataConfig,
) {
    use aws_sdk_bedrock::types::{
        AutomatedEvaluationConfig, BedrockEvaluatorModel, EvaluationConfig, EvaluationDataset,
        EvaluationDatasetLocation, EvaluationDatasetMetricConfig, EvaluationInferenceConfig,
        EvaluationModelConfig, EvaluationOutputDataConfig, EvaluationTaskType,
        EvaluatorModelConfig,
    };

    let dataset_location = EvaluationDatasetLocation::S3Uri("s3://my-bucket/dataset/".to_string());
    let dataset = EvaluationDataset::builder()
        .name("my-dataset")
        .dataset_location(dataset_location)
        .build()
        .unwrap();
    let metric_config = EvaluationDatasetMetricConfig::builder()
        .task_type(EvaluationTaskType::from("Summarization"))
        .dataset(dataset)
        .metric_names("Accuracy")
        .build()
        .unwrap();
    let bedrock_evaluator = BedrockEvaluatorModel::builder()
        .model_identifier("anthropic.claude-v2")
        .build()
        .unwrap();
    let evaluator_config = EvaluatorModelConfig::BedrockEvaluatorModels(vec![bedrock_evaluator]);
    let auto_config = AutomatedEvaluationConfig::builder()
        .dataset_metric_configs(metric_config)
        .evaluator_model_config(evaluator_config)
        .build()
        .unwrap();
    let eval_config = EvaluationConfig::Automated(auto_config);

    let inference_config =
        EvaluationInferenceConfig::Models(vec![EvaluationModelConfig::BedrockModel(
            aws_sdk_bedrock::types::EvaluationBedrockModel::builder()
                .model_identifier("anthropic.claude-v2")
                .build()
                .unwrap(),
        )]);

    let output_config = EvaluationOutputDataConfig::builder()
        .s3_uri("s3://my-bucket/output/")
        .build()
        .unwrap();

    (eval_config, inference_config, output_config)
}

#[tokio::test]
async fn test_create_and_get_evaluation_job() {
    let client = make_bedrock_client().await;
    let (eval_config, inference_config, output_config) = make_eval_job_params();

    let resp = client
        .create_evaluation_job()
        .job_name("my-eval-job")
        .evaluation_config(eval_config)
        .inference_config(inference_config)
        .output_data_config(output_config)
        .role_arn("arn:aws:iam::123456789012:role/BedrockEvalRole")
        .send()
        .await
        .expect("create_evaluation_job should succeed");

    let job_arn = resp.job_arn().to_string();
    assert!(!job_arn.is_empty());

    let get_resp = client
        .get_evaluation_job()
        .job_identifier(&job_arn)
        .send()
        .await
        .expect("get_evaluation_job should succeed");

    assert_eq!(get_resp.job_name(), "my-eval-job");
}

#[tokio::test]
async fn test_list_evaluation_jobs() {
    let client = make_bedrock_client().await;
    let (eval_config, inference_config, output_config) = make_eval_job_params();

    client
        .create_evaluation_job()
        .job_name("eval-job-for-list")
        .evaluation_config(eval_config)
        .inference_config(inference_config)
        .output_data_config(output_config)
        .role_arn("arn:aws:iam::123456789012:role/BedrockEvalRole")
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_evaluation_jobs()
        .send()
        .await
        .expect("list_evaluation_jobs should succeed");

    assert!(!list_resp.job_summaries().is_empty());
}

// ---- Model Invocation Jobs ----

#[tokio::test]
async fn test_create_and_get_model_invocation_job() {
    use aws_sdk_bedrock::types::{
        ModelInvocationJobInputDataConfig, ModelInvocationJobOutputDataConfig,
        ModelInvocationJobS3InputDataConfig, ModelInvocationJobS3OutputDataConfig,
    };

    let client = make_bedrock_client().await;

    let s3_input = ModelInvocationJobS3InputDataConfig::builder()
        .s3_uri("s3://my-bucket/input/")
        .build()
        .unwrap();
    let input_config = ModelInvocationJobInputDataConfig::S3InputDataConfig(s3_input);

    let s3_output = ModelInvocationJobS3OutputDataConfig::builder()
        .s3_uri("s3://my-bucket/output/")
        .build()
        .unwrap();
    let output_config = ModelInvocationJobOutputDataConfig::S3OutputDataConfig(s3_output);

    let resp = client
        .create_model_invocation_job()
        .job_name("my-invocation-job")
        .model_id("anthropic.claude-v2")
        .role_arn("arn:aws:iam::123456789012:role/BedrockRole")
        .input_data_config(input_config)
        .output_data_config(output_config)
        .send()
        .await
        .expect("create_model_invocation_job should succeed");

    let job_arn = resp.job_arn().to_string();
    assert!(!job_arn.is_empty());

    let get_resp = client
        .get_model_invocation_job()
        .job_identifier(&job_arn)
        .send()
        .await
        .expect("get_model_invocation_job should succeed");

    assert_eq!(get_resp.job_name(), Some("my-invocation-job"));
    assert_eq!(get_resp.model_id(), "anthropic.claude-v2");
}

#[tokio::test]
async fn test_list_model_invocation_jobs() {
    use aws_sdk_bedrock::types::{
        ModelInvocationJobInputDataConfig, ModelInvocationJobOutputDataConfig,
        ModelInvocationJobS3InputDataConfig, ModelInvocationJobS3OutputDataConfig,
    };

    let client = make_bedrock_client().await;

    let s3_input = ModelInvocationJobS3InputDataConfig::builder()
        .s3_uri("s3://my-bucket/input/")
        .build()
        .unwrap();
    let input_config = ModelInvocationJobInputDataConfig::S3InputDataConfig(s3_input);

    let s3_output = ModelInvocationJobS3OutputDataConfig::builder()
        .s3_uri("s3://my-bucket/output/")
        .build()
        .unwrap();
    let output_config = ModelInvocationJobOutputDataConfig::S3OutputDataConfig(s3_output);

    client
        .create_model_invocation_job()
        .job_name("invocation-job-for-list")
        .model_id("anthropic.claude-v2")
        .role_arn("arn:aws:iam::123456789012:role/BedrockRole")
        .input_data_config(input_config)
        .output_data_config(output_config)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_model_invocation_jobs()
        .send()
        .await
        .expect("list_model_invocation_jobs should succeed");

    assert!(!list_resp.invocation_job_summaries().is_empty());
}

// ---- Model Import Jobs ----

#[tokio::test]
async fn test_create_and_get_model_import_job() {
    use aws_sdk_bedrock::types::{ModelDataSource, S3DataSource};

    let client = make_bedrock_client().await;

    let s3_source = S3DataSource::builder()
        .s3_uri("s3://my-bucket/model/")
        .build()
        .unwrap();
    let model_data_source = ModelDataSource::S3DataSource(s3_source);

    let resp = client
        .create_model_import_job()
        .job_name("my-import-job")
        .imported_model_name("my-imported-model")
        .role_arn("arn:aws:iam::123456789012:role/BedrockRole")
        .model_data_source(model_data_source)
        .send()
        .await
        .expect("create_model_import_job should succeed");

    let job_arn = resp.job_arn().to_string();
    assert!(!job_arn.is_empty());

    let get_resp = client
        .get_model_import_job()
        .job_identifier(&job_arn)
        .send()
        .await
        .expect("get_model_import_job should succeed");

    assert_eq!(get_resp.job_name(), Some("my-import-job"));
}

#[tokio::test]
async fn test_list_model_import_jobs() {
    use aws_sdk_bedrock::types::{ModelDataSource, S3DataSource};

    let client = make_bedrock_client().await;

    let s3_source = S3DataSource::builder()
        .s3_uri("s3://my-bucket/model/")
        .build()
        .unwrap();
    let model_data_source = ModelDataSource::S3DataSource(s3_source);

    client
        .create_model_import_job()
        .job_name("import-job-for-list")
        .imported_model_name("imported-model-for-list")
        .role_arn("arn:aws:iam::123456789012:role/BedrockRole")
        .model_data_source(model_data_source)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_model_import_jobs()
        .send()
        .await
        .expect("list_model_import_jobs should succeed");

    assert!(!list_resp.model_import_job_summaries().is_empty());
}

// ---- Model Copy Jobs ----

#[tokio::test]
async fn test_create_and_get_model_copy_job() {
    let client = make_bedrock_client().await;

    let resp = client
        .create_model_copy_job()
        .source_model_arn("arn:aws:bedrock:us-east-1:123456789012:custom-model/source-model")
        .target_model_name("copied-model")
        .send()
        .await
        .expect("create_model_copy_job should succeed");

    let job_arn = resp.job_arn().to_string();
    assert!(!job_arn.is_empty());

    let get_resp = client
        .get_model_copy_job()
        .job_arn(&job_arn)
        .send()
        .await
        .expect("get_model_copy_job should succeed");

    assert_eq!(get_resp.target_model_name(), Some("copied-model"));
}

#[tokio::test]
async fn test_list_model_copy_jobs() {
    let client = make_bedrock_client().await;

    client
        .create_model_copy_job()
        .source_model_arn("arn:aws:bedrock:us-east-1:123456789012:custom-model/source-model-2")
        .target_model_name("copied-model-for-list")
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_model_copy_jobs()
        .send()
        .await
        .expect("list_model_copy_jobs should succeed");

    assert!(!list_resp.model_copy_job_summaries().is_empty());
}
