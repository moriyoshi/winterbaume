use aws_sdk_comprehend::config::BehaviorVersion;
use aws_sdk_comprehend::types::{
    EntityRecognizerInputDataConfig, EntityTypesListItem, InputDataConfig, LanguageCode,
    OutputDataConfig, Tag,
};
use winterbaume_comprehend::ComprehendService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_comprehend::Client {
    let mock = MockAws::builder()
        .with_service(ComprehendService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_comprehend::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_comprehend::Client::new(&config)
}

// ---- Detect operations ----

#[tokio::test]
async fn test_detect_sentiment() {
    let client = make_client().await;
    let resp = client
        .detect_sentiment()
        .text("I love this product!")
        .language_code(LanguageCode::En)
        .send()
        .await
        .expect("detect_sentiment should succeed");

    let sentiment = resp.sentiment().expect("should have sentiment");
    assert_eq!(sentiment.as_str(), "NEUTRAL");
    let score = resp.sentiment_score().expect("should have sentiment score");
    assert!(score.positive().is_some());
}

#[tokio::test]
async fn test_detect_entities() {
    let client = make_client().await;
    let resp = client
        .detect_entities()
        .text("Amazon Web Services is based in Seattle.")
        .language_code(LanguageCode::En)
        .send()
        .await
        .expect("detect_entities should succeed");
    let _entities = resp.entities();
}

#[tokio::test]
async fn test_detect_dominant_language() {
    let client = make_client().await;
    let resp = client
        .detect_dominant_language()
        .text("This is a test in English.")
        .send()
        .await
        .expect("detect_dominant_language should succeed");

    let languages = resp.languages();
    assert!(!languages.is_empty());
    assert_eq!(languages[0].language_code().unwrap(), "en");
}

#[tokio::test]
async fn test_detect_key_phrases() {
    let client = make_client().await;
    let resp = client
        .detect_key_phrases()
        .text("The quick brown fox jumps over the lazy dog.")
        .language_code(LanguageCode::En)
        .send()
        .await
        .expect("detect_key_phrases should succeed");
    let _phrases = resp.key_phrases();
}

#[tokio::test]
async fn test_detect_pii_entities() {
    let client = make_client().await;
    let resp = client
        .detect_pii_entities()
        .text("My SSN is 123-45-6789")
        .language_code(LanguageCode::En)
        .send()
        .await
        .expect("detect_pii_entities should succeed");
    let _entities = resp.entities();
}

#[tokio::test]
async fn test_detect_sentiment_missing_text() {
    let client = make_client().await;
    let result = client
        .detect_sentiment()
        .language_code(LanguageCode::En)
        .send()
        .await;
    assert!(result.is_err(), "should fail without text");
}

// ---- Document Classifier lifecycle ----

#[tokio::test]
async fn test_document_classifier_lifecycle() {
    let client = make_client().await;

    // Create
    let resp = client
        .create_document_classifier()
        .document_classifier_name("test-classifier")
        .language_code(LanguageCode::En)
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .input_data_config(
            aws_sdk_comprehend::types::DocumentClassifierInputDataConfig::builder()
                .s3_uri("s3://my-bucket/input")
                .build(),
        )
        .send()
        .await
        .expect("create_document_classifier should succeed");

    let arn = resp.document_classifier_arn().expect("should have ARN");
    assert!(arn.contains("test-classifier"));

    // Describe
    let resp = client
        .describe_document_classifier()
        .document_classifier_arn(arn)
        .send()
        .await
        .expect("describe should succeed");
    let props = resp.document_classifier_properties().unwrap();
    assert_eq!(props.document_classifier_arn().unwrap(), arn);

    // List
    let resp = client
        .list_document_classifiers()
        .send()
        .await
        .expect("list should succeed");
    assert!(!resp.document_classifier_properties_list().is_empty());

    // StopTraining
    let _resp = client
        .stop_training_document_classifier()
        .document_classifier_arn(arn)
        .send()
        .await
        .expect("stop_training should succeed");

    // Delete
    client
        .delete_document_classifier()
        .document_classifier_arn(arn)
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let result = client
        .describe_document_classifier()
        .document_classifier_arn(arn)
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Entity Recognizer lifecycle ----

#[tokio::test]
async fn test_entity_recognizer_lifecycle() {
    let client = make_client().await;

    // Create
    let resp = client
        .create_entity_recognizer()
        .recognizer_name("test-recognizer")
        .language_code(LanguageCode::En)
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .input_data_config(
            EntityRecognizerInputDataConfig::builder()
                .entity_types(
                    EntityTypesListItem::builder()
                        .r#type("PERSON")
                        .build()
                        .unwrap(),
                )
                .documents(
                    aws_sdk_comprehend::types::EntityRecognizerDocuments::builder()
                        .s3_uri("s3://my-bucket/docs")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_entity_recognizer should succeed");

    let arn = resp.entity_recognizer_arn().expect("should have ARN");
    assert!(arn.contains("test-recognizer"));

    // Describe
    let resp = client
        .describe_entity_recognizer()
        .entity_recognizer_arn(arn)
        .send()
        .await
        .expect("describe should succeed");
    assert!(resp.entity_recognizer_properties().is_some());

    // List
    let resp = client
        .list_entity_recognizers()
        .send()
        .await
        .expect("list should succeed");
    assert!(!resp.entity_recognizer_properties_list().is_empty());

    // StopTraining
    client
        .stop_training_entity_recognizer()
        .entity_recognizer_arn(arn)
        .send()
        .await
        .expect("stop_training should succeed");

    // Delete
    client
        .delete_entity_recognizer()
        .entity_recognizer_arn(arn)
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let result = client
        .describe_entity_recognizer()
        .entity_recognizer_arn(arn)
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Endpoint lifecycle ----

#[tokio::test]
async fn test_endpoint_lifecycle() {
    let client = make_client().await;

    // Create
    let resp = client
        .create_endpoint()
        .endpoint_name("test-endpoint")
        .model_arn("arn:aws:comprehend:us-east-1:123456789012:document-classifier/model")
        .desired_inference_units(1)
        .send()
        .await
        .expect("create_endpoint should succeed");

    let arn = resp.endpoint_arn().expect("should have ARN");
    assert!(arn.contains("test-endpoint"));

    // Describe
    let resp = client
        .describe_endpoint()
        .endpoint_arn(arn)
        .send()
        .await
        .expect("describe should succeed");
    let props = resp.endpoint_properties().unwrap();
    assert_eq!(props.endpoint_arn().unwrap(), arn);
    assert_eq!(props.current_inference_units().unwrap(), 1);

    // Update
    let _resp = client
        .update_endpoint()
        .endpoint_arn(arn)
        .desired_inference_units(2)
        .send()
        .await
        .expect("update should succeed");

    // List
    let resp = client
        .list_endpoints()
        .send()
        .await
        .expect("list should succeed");
    assert!(!resp.endpoint_properties_list().is_empty());

    // Delete
    client
        .delete_endpoint()
        .endpoint_arn(arn)
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let result = client.describe_endpoint().endpoint_arn(arn).send().await;
    assert!(result.is_err());
}

// ---- Flywheel lifecycle ----

#[tokio::test]
async fn test_flywheel_lifecycle() {
    let client = make_client().await;

    // Create
    let resp = client
        .create_flywheel()
        .flywheel_name("test-flywheel")
        .active_model_arn("arn:aws:comprehend:us-east-1:123456789012:document-classifier/model")
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .data_lake_s3_uri("s3://my-bucket/data-lake")
        .send()
        .await
        .expect("create_flywheel should succeed");

    let arn = resp.flywheel_arn().expect("should have ARN");
    assert!(arn.contains("test-flywheel"));

    // Describe
    let resp = client
        .describe_flywheel()
        .flywheel_arn(arn)
        .send()
        .await
        .expect("describe should succeed");
    assert!(resp.flywheel_properties().is_some());

    // StartFlywheelIteration
    let resp = client
        .start_flywheel_iteration()
        .flywheel_arn(arn)
        .send()
        .await
        .expect("start_flywheel_iteration should succeed");
    assert!(resp.flywheel_iteration_id().is_some());

    // List
    let resp = client
        .list_flywheels()
        .send()
        .await
        .expect("list should succeed");
    assert!(!resp.flywheel_summary_list().is_empty());

    // Delete
    client
        .delete_flywheel()
        .flywheel_arn(arn)
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let result = client.describe_flywheel().flywheel_arn(arn).send().await;
    assert!(result.is_err());
}

// ---- Sentiment Detection Job lifecycle ----

#[tokio::test]
async fn test_sentiment_detection_job_lifecycle() {
    let client = make_client().await;

    // Start
    let resp = client
        .start_sentiment_detection_job()
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .input_data_config(
            InputDataConfig::builder()
                .s3_uri("s3://my-bucket/input")
                .build()
                .unwrap(),
        )
        .output_data_config(
            OutputDataConfig::builder()
                .s3_uri("s3://my-bucket/output")
                .build()
                .unwrap(),
        )
        .language_code(LanguageCode::En)
        .send()
        .await
        .expect("start should succeed");

    let job_id = resp.job_id().expect("should have job ID");
    assert_eq!(resp.job_status().unwrap().as_str(), "SUBMITTED");

    // Describe
    let resp = client
        .describe_sentiment_detection_job()
        .job_id(job_id)
        .send()
        .await
        .expect("describe should succeed");
    let props = resp.sentiment_detection_job_properties().unwrap();
    assert_eq!(props.job_id().unwrap(), job_id);

    // List
    let resp = client
        .list_sentiment_detection_jobs()
        .send()
        .await
        .expect("list should succeed");
    assert!(!resp.sentiment_detection_job_properties_list().is_empty());

    // Stop
    let resp = client
        .stop_sentiment_detection_job()
        .job_id(job_id)
        .send()
        .await
        .expect("stop should succeed");
    assert_eq!(resp.job_status().unwrap().as_str(), "STOP_REQUESTED");
}

// ---- Document Classification Job ----

#[tokio::test]
async fn test_document_classification_job() {
    let client = make_client().await;

    let resp = client
        .start_document_classification_job()
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .input_data_config(
            InputDataConfig::builder()
                .s3_uri("s3://my-bucket/input")
                .build()
                .unwrap(),
        )
        .output_data_config(
            OutputDataConfig::builder()
                .s3_uri("s3://my-bucket/output")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("start should succeed");

    let job_id = resp.job_id().unwrap();

    let resp = client
        .describe_document_classification_job()
        .job_id(job_id)
        .send()
        .await
        .expect("describe should succeed");
    assert!(resp.document_classification_job_properties().is_some());

    let resp = client
        .list_document_classification_jobs()
        .send()
        .await
        .expect("list should succeed");
    assert!(
        !resp
            .document_classification_job_properties_list()
            .is_empty()
    );
}

// ---- Dominant Language Detection Job ----

#[tokio::test]
async fn test_dominant_language_detection_job() {
    let client = make_client().await;

    let resp = client
        .start_dominant_language_detection_job()
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .input_data_config(
            InputDataConfig::builder()
                .s3_uri("s3://my-bucket/input")
                .build()
                .unwrap(),
        )
        .output_data_config(
            OutputDataConfig::builder()
                .s3_uri("s3://my-bucket/output")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("start should succeed");

    let job_id = resp.job_id().unwrap();

    let resp = client
        .describe_dominant_language_detection_job()
        .job_id(job_id)
        .send()
        .await
        .expect("describe should succeed");
    assert!(resp.dominant_language_detection_job_properties().is_some());

    let resp = client
        .list_dominant_language_detection_jobs()
        .send()
        .await
        .expect("list should succeed");
    assert!(
        !resp
            .dominant_language_detection_job_properties_list()
            .is_empty()
    );

    let resp = client
        .stop_dominant_language_detection_job()
        .job_id(job_id)
        .send()
        .await
        .expect("stop should succeed");
    assert_eq!(resp.job_status().unwrap().as_str(), "STOP_REQUESTED");
}

// ---- Entities Detection Job ----

#[tokio::test]
async fn test_entities_detection_job() {
    let client = make_client().await;

    let resp = client
        .start_entities_detection_job()
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .input_data_config(
            InputDataConfig::builder()
                .s3_uri("s3://my-bucket/input")
                .build()
                .unwrap(),
        )
        .output_data_config(
            OutputDataConfig::builder()
                .s3_uri("s3://my-bucket/output")
                .build()
                .unwrap(),
        )
        .language_code(LanguageCode::En)
        .send()
        .await
        .expect("start should succeed");

    let job_id = resp.job_id().unwrap();

    client
        .describe_entities_detection_job()
        .job_id(job_id)
        .send()
        .await
        .expect("describe should succeed");

    let resp = client
        .list_entities_detection_jobs()
        .send()
        .await
        .expect("list should succeed");
    assert!(!resp.entities_detection_job_properties_list().is_empty());

    let resp = client
        .stop_entities_detection_job()
        .job_id(job_id)
        .send()
        .await
        .expect("stop should succeed");
    assert_eq!(resp.job_status().unwrap().as_str(), "STOP_REQUESTED");
}

// ---- Events Detection Job ----

#[tokio::test]
async fn test_events_detection_job() {
    let client = make_client().await;

    let resp = client
        .start_events_detection_job()
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .input_data_config(
            InputDataConfig::builder()
                .s3_uri("s3://my-bucket/input")
                .build()
                .unwrap(),
        )
        .output_data_config(
            OutputDataConfig::builder()
                .s3_uri("s3://my-bucket/output")
                .build()
                .unwrap(),
        )
        .language_code(LanguageCode::En)
        .target_event_types("BANKRUPTCY")
        .send()
        .await
        .expect("start should succeed");

    let job_id = resp.job_id().unwrap();

    client
        .describe_events_detection_job()
        .job_id(job_id)
        .send()
        .await
        .expect("describe should succeed");

    let resp = client
        .list_events_detection_jobs()
        .send()
        .await
        .expect("list should succeed");
    assert!(!resp.events_detection_job_properties_list().is_empty());

    let resp = client
        .stop_events_detection_job()
        .job_id(job_id)
        .send()
        .await
        .expect("stop should succeed");
    assert_eq!(resp.job_status().unwrap().as_str(), "STOP_REQUESTED");
}

// ---- Key Phrases Detection Job ----

#[tokio::test]
async fn test_key_phrases_detection_job() {
    let client = make_client().await;

    let resp = client
        .start_key_phrases_detection_job()
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .input_data_config(
            InputDataConfig::builder()
                .s3_uri("s3://my-bucket/input")
                .build()
                .unwrap(),
        )
        .output_data_config(
            OutputDataConfig::builder()
                .s3_uri("s3://my-bucket/output")
                .build()
                .unwrap(),
        )
        .language_code(LanguageCode::En)
        .send()
        .await
        .expect("start should succeed");

    let job_id = resp.job_id().unwrap();

    client
        .describe_key_phrases_detection_job()
        .job_id(job_id)
        .send()
        .await
        .expect("describe should succeed");

    let resp = client
        .list_key_phrases_detection_jobs()
        .send()
        .await
        .expect("list should succeed");
    assert!(!resp.key_phrases_detection_job_properties_list().is_empty());

    let resp = client
        .stop_key_phrases_detection_job()
        .job_id(job_id)
        .send()
        .await
        .expect("stop should succeed");
    assert_eq!(resp.job_status().unwrap().as_str(), "STOP_REQUESTED");
}

// ---- PII Entities Detection Job ----

#[tokio::test]
async fn test_pii_entities_detection_job() {
    let client = make_client().await;

    let resp = client
        .start_pii_entities_detection_job()
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .input_data_config(
            InputDataConfig::builder()
                .s3_uri("s3://my-bucket/input")
                .build()
                .unwrap(),
        )
        .output_data_config(
            aws_sdk_comprehend::types::OutputDataConfig::builder()
                .s3_uri("s3://my-bucket/output")
                .build()
                .unwrap(),
        )
        .language_code(LanguageCode::En)
        .mode(aws_sdk_comprehend::types::PiiEntitiesDetectionMode::OnlyRedaction)
        .send()
        .await
        .expect("start should succeed");

    let job_id = resp.job_id().unwrap();

    client
        .describe_pii_entities_detection_job()
        .job_id(job_id)
        .send()
        .await
        .expect("describe should succeed");

    let resp = client
        .list_pii_entities_detection_jobs()
        .send()
        .await
        .expect("list should succeed");
    assert!(!resp.pii_entities_detection_job_properties_list().is_empty());

    let resp = client
        .stop_pii_entities_detection_job()
        .job_id(job_id)
        .send()
        .await
        .expect("stop should succeed");
    assert_eq!(resp.job_status().unwrap().as_str(), "STOP_REQUESTED");
}

// ---- Targeted Sentiment Detection Job ----

#[tokio::test]
async fn test_targeted_sentiment_detection_job() {
    let client = make_client().await;

    let resp = client
        .start_targeted_sentiment_detection_job()
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .input_data_config(
            InputDataConfig::builder()
                .s3_uri("s3://my-bucket/input")
                .build()
                .unwrap(),
        )
        .output_data_config(
            OutputDataConfig::builder()
                .s3_uri("s3://my-bucket/output")
                .build()
                .unwrap(),
        )
        .language_code(LanguageCode::En)
        .send()
        .await
        .expect("start should succeed");

    let job_id = resp.job_id().unwrap();

    client
        .describe_targeted_sentiment_detection_job()
        .job_id(job_id)
        .send()
        .await
        .expect("describe should succeed");

    let resp = client
        .list_targeted_sentiment_detection_jobs()
        .send()
        .await
        .expect("list should succeed");
    assert!(
        !resp
            .targeted_sentiment_detection_job_properties_list()
            .is_empty()
    );

    let resp = client
        .stop_targeted_sentiment_detection_job()
        .job_id(job_id)
        .send()
        .await
        .expect("stop should succeed");
    assert_eq!(resp.job_status().unwrap().as_str(), "STOP_REQUESTED");
}

// ---- Topics Detection Job ----

#[tokio::test]
async fn test_topics_detection_job() {
    let client = make_client().await;

    let resp = client
        .start_topics_detection_job()
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .input_data_config(
            InputDataConfig::builder()
                .s3_uri("s3://my-bucket/input")
                .build()
                .unwrap(),
        )
        .output_data_config(
            OutputDataConfig::builder()
                .s3_uri("s3://my-bucket/output")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("start should succeed");

    let job_id = resp.job_id().unwrap();

    client
        .describe_topics_detection_job()
        .job_id(job_id)
        .send()
        .await
        .expect("describe should succeed");

    let resp = client
        .list_topics_detection_jobs()
        .send()
        .await
        .expect("list should succeed");
    assert!(!resp.topics_detection_job_properties_list().is_empty());
}

// ---- Resource Policy ----

#[tokio::test]
async fn test_resource_policy_lifecycle() {
    let client = make_client().await;

    let resource_arn = "arn:aws:comprehend:us-east-1:123456789012:document-classifier/my-model";

    // Put
    let resp = client
        .put_resource_policy()
        .resource_arn(resource_arn)
        .resource_policy(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .expect("put_resource_policy should succeed");
    assert!(resp.policy_revision_id().is_some());

    // Describe
    let resp = client
        .describe_resource_policy()
        .resource_arn(resource_arn)
        .send()
        .await
        .expect("describe should succeed");
    assert!(resp.resource_policy().is_some());
    assert!(resp.policy_revision_id().is_some());

    // Delete
    client
        .delete_resource_policy()
        .resource_arn(resource_arn)
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let result = client
        .describe_resource_policy()
        .resource_arn(resource_arn)
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Tags ----

#[tokio::test]
async fn test_tags_lifecycle() {
    let client = make_client().await;

    let resource_arn = "arn:aws:comprehend:us-east-1:123456789012:document-classifier/tagged-model";

    // Tag
    client
        .tag_resource()
        .resource_arn(resource_arn)
        .tags(Tag::builder().key("env").value("test").build().unwrap())
        .tags(Tag::builder().key("team").value("ml").build().unwrap())
        .send()
        .await
        .expect("tag_resource should succeed");

    // ListTags
    let resp = client
        .list_tags_for_resource()
        .resource_arn(resource_arn)
        .send()
        .await
        .expect("list_tags should succeed");
    let tags = resp.tags();
    assert_eq!(tags.len(), 2);

    // Untag
    client
        .untag_resource()
        .resource_arn(resource_arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag should succeed");

    // Verify
    let resp = client
        .list_tags_for_resource()
        .resource_arn(resource_arn)
        .send()
        .await
        .expect("list_tags should succeed");
    let tags = resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "team");
}

// ---- Error paths ----

#[tokio::test]
async fn test_delete_nonexistent_classifier() {
    let client = make_client().await;
    let result = client
        .delete_document_classifier()
        .document_classifier_arn(
            "arn:aws:comprehend:us-east-1:123456789012:document-classifier/nonexistent",
        )
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_endpoint() {
    let client = make_client().await;
    let result = client
        .delete_endpoint()
        .endpoint_arn(
            "arn:aws:comprehend:us-east-1:123456789012:document-classifier-endpoint/nonexistent",
        )
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_nonexistent_job() {
    let client = make_client().await;
    let result = client
        .describe_sentiment_detection_job()
        .job_id("nonexistent-job-id")
        .send()
        .await;
    assert!(result.is_err());
}

// ---- Version name support ----

/// Translated from moto test_create_entity_recognizer_without_version and
/// test_create_entity_recognizer (with version)
#[tokio::test]
async fn test_entity_recognizer_version_in_arn() {
    let client = make_client().await;

    // Without version: ARN has no version suffix
    let resp = client
        .create_entity_recognizer()
        .recognizer_name("rec-no-version")
        .language_code(LanguageCode::En)
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .input_data_config(
            EntityRecognizerInputDataConfig::builder()
                .entity_types(
                    EntityTypesListItem::builder()
                        .r#type("PERSON")
                        .build()
                        .unwrap(),
                )
                .documents(
                    aws_sdk_comprehend::types::EntityRecognizerDocuments::builder()
                        .s3_uri("s3://my-bucket/docs")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create without version should succeed");
    let arn = resp.entity_recognizer_arn().unwrap();
    assert!(arn.ends_with(":entity-recognizer/rec-no-version"));

    // With version: ARN includes version segment
    let resp2 = client
        .create_entity_recognizer()
        .recognizer_name("rec-with-version")
        .version_name("v1")
        .language_code(LanguageCode::En)
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .input_data_config(
            EntityRecognizerInputDataConfig::builder()
                .entity_types(
                    EntityTypesListItem::builder()
                        .r#type("PERSON")
                        .build()
                        .unwrap(),
                )
                .documents(
                    aws_sdk_comprehend::types::EntityRecognizerDocuments::builder()
                        .s3_uri("s3://my-bucket/docs")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create with version should succeed");
    let arn2 = resp2.entity_recognizer_arn().unwrap();
    assert!(
        arn2.contains("/version/v1"),
        "ARN should contain /version/v1, got {arn2}"
    );
}

/// Translated from moto test_list_entity_recognizers (with Filter.RecognizerName)
#[tokio::test]
async fn test_list_entity_recognizers_filter_by_name() {
    let client = make_client().await;

    // Create two recognizers with version names so ARNs are distinct
    client
        .create_entity_recognizer()
        .recognizer_name("target-recognizer")
        .version_name("v1")
        .language_code(LanguageCode::En)
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .input_data_config(
            EntityRecognizerInputDataConfig::builder()
                .entity_types(
                    EntityTypesListItem::builder()
                        .r#type("PERSON")
                        .build()
                        .unwrap(),
                )
                .documents(
                    aws_sdk_comprehend::types::EntityRecognizerDocuments::builder()
                        .s3_uri("s3://my-bucket/docs")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .create_entity_recognizer()
        .recognizer_name("target-recognizer")
        .version_name("v2")
        .language_code(LanguageCode::En)
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .input_data_config(
            EntityRecognizerInputDataConfig::builder()
                .entity_types(
                    EntityTypesListItem::builder()
                        .r#type("PERSON")
                        .build()
                        .unwrap(),
                )
                .documents(
                    aws_sdk_comprehend::types::EntityRecognizerDocuments::builder()
                        .s3_uri("s3://my-bucket/docs")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .create_entity_recognizer()
        .recognizer_name("other-recognizer")
        .version_name("v1")
        .language_code(LanguageCode::En)
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .input_data_config(
            EntityRecognizerInputDataConfig::builder()
                .entity_types(
                    EntityTypesListItem::builder()
                        .r#type("PERSON")
                        .build()
                        .unwrap(),
                )
                .documents(
                    aws_sdk_comprehend::types::EntityRecognizerDocuments::builder()
                        .s3_uri("s3://my-bucket/docs")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    // Filter by name: should only return the 2 "target-recognizer" entries
    let resp = client
        .list_entity_recognizers()
        .filter(
            aws_sdk_comprehend::types::EntityRecognizerFilter::builder()
                .recognizer_name("target-recognizer")
                .build(),
        )
        .send()
        .await
        .expect("list with filter should succeed");
    assert_eq!(
        resp.entity_recognizer_properties_list().len(),
        2,
        "should return both versions of target-recognizer"
    );

    // Filter by unknown name: should return empty list
    let resp2 = client
        .list_entity_recognizers()
        .filter(
            aws_sdk_comprehend::types::EntityRecognizerFilter::builder()
                .recognizer_name("nonexistent")
                .build(),
        )
        .send()
        .await
        .expect("list with unknown filter should succeed");
    assert!(resp2.entity_recognizer_properties_list().is_empty());
}

/// Translated from moto test_list_document_classifiers (with Filter.DocumentClassifierName)
#[tokio::test]
async fn test_list_document_classifiers_filter_by_name() {
    let client = make_client().await;

    // Create two versions of the same classifier name
    client
        .create_document_classifier()
        .document_classifier_name("my-classifier")
        .version_name("v1")
        .language_code(LanguageCode::En)
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .input_data_config(
            aws_sdk_comprehend::types::DocumentClassifierInputDataConfig::builder()
                .s3_uri("s3://my-bucket/input")
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .create_document_classifier()
        .document_classifier_name("my-classifier")
        .version_name("v2")
        .language_code(LanguageCode::En)
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .input_data_config(
            aws_sdk_comprehend::types::DocumentClassifierInputDataConfig::builder()
                .s3_uri("s3://my-bucket/input")
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .create_document_classifier()
        .document_classifier_name("other-classifier")
        .version_name("v1")
        .language_code(LanguageCode::En)
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .input_data_config(
            aws_sdk_comprehend::types::DocumentClassifierInputDataConfig::builder()
                .s3_uri("s3://my-bucket/input")
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Filter by name should return only my-classifier entries (2)
    let resp = client
        .list_document_classifiers()
        .filter(
            aws_sdk_comprehend::types::DocumentClassifierFilter::builder()
                .document_classifier_name("my-classifier")
                .build(),
        )
        .send()
        .await
        .expect("list with filter should succeed");
    assert_eq!(resp.document_classifier_properties_list().len(), 2);

    // Filter by unknown name should be empty
    let resp2 = client
        .list_document_classifiers()
        .filter(
            aws_sdk_comprehend::types::DocumentClassifierFilter::builder()
                .document_classifier_name("unknown")
                .build(),
        )
        .send()
        .await
        .expect("list with unknown filter should succeed");
    assert!(resp2.document_classifier_properties_list().is_empty());
}

/// Translated from moto test_list_endpoints (with Filter.Status)
#[tokio::test]
async fn test_list_endpoints_filter_by_status() {
    let client = make_client().await;

    // All created endpoints start as IN_SERVICE
    client
        .create_endpoint()
        .endpoint_name("endpoint-filter-test-1")
        .model_arn("arn:aws:comprehend:us-east-1:123456789012:document-classifier/model")
        .desired_inference_units(1)
        .send()
        .await
        .unwrap();

    client
        .create_endpoint()
        .endpoint_name("endpoint-filter-test-2")
        .model_arn("arn:aws:comprehend:us-east-1:123456789012:document-classifier/model")
        .desired_inference_units(1)
        .send()
        .await
        .unwrap();

    // Filter by IN_SERVICE: both returned
    let resp = client
        .list_endpoints()
        .filter(
            aws_sdk_comprehend::types::EndpointFilter::builder()
                .status(aws_sdk_comprehend::types::EndpointStatus::InService)
                .build(),
        )
        .send()
        .await
        .expect("list with status filter should succeed");
    assert_eq!(resp.endpoint_properties_list().len(), 2);

    // Filter by FAILED: none returned
    let resp2 = client
        .list_endpoints()
        .filter(
            aws_sdk_comprehend::types::EndpointFilter::builder()
                .status(aws_sdk_comprehend::types::EndpointStatus::Failed)
                .build(),
        )
        .send()
        .await
        .expect("list with failed filter should succeed");
    assert!(resp2.endpoint_properties_list().is_empty());
}

/// Translated from moto test_list_flywheels (with Filter.Status)
#[tokio::test]
async fn test_list_flywheels_filter_by_status() {
    let client = make_client().await;

    // All created flywheels start as ACTIVE
    client
        .create_flywheel()
        .flywheel_name("fw-filter-test")
        .active_model_arn("arn:aws:comprehend:us-east-1:123456789012:document-classifier/model")
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .data_lake_s3_uri("s3://my-bucket/data-lake")
        .send()
        .await
        .unwrap();

    // Filter by ACTIVE: should return the flywheel
    let resp = client
        .list_flywheels()
        .filter(
            aws_sdk_comprehend::types::FlywheelFilter::builder()
                .status(aws_sdk_comprehend::types::FlywheelStatus::Active)
                .build(),
        )
        .send()
        .await
        .expect("list with ACTIVE filter should succeed");
    assert!(!resp.flywheel_summary_list().is_empty());

    // Filter by CREATING: should be empty
    let resp2 = client
        .list_flywheels()
        .filter(
            aws_sdk_comprehend::types::FlywheelFilter::builder()
                .status(aws_sdk_comprehend::types::FlywheelStatus::Creating)
                .build(),
        )
        .send()
        .await
        .expect("list with CREATING filter should succeed");
    assert!(resp2.flywheel_summary_list().is_empty());
}

/// Translated from moto test_stop_training_document_classifier:
/// document classifier starts as TRAINING and stop_training sets it to STOP_REQUESTED.
#[tokio::test]
async fn test_document_classifier_status_lifecycle() {
    let client = make_client().await;

    let resp = client
        .create_document_classifier()
        .document_classifier_name("status-test-classifier")
        .language_code(LanguageCode::En)
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .input_data_config(
            aws_sdk_comprehend::types::DocumentClassifierInputDataConfig::builder()
                .s3_uri("s3://my-bucket/input")
                .build(),
        )
        .send()
        .await
        .unwrap();
    let arn = resp.document_classifier_arn().unwrap();

    // Status after creation should be TRAINING
    let desc = client
        .describe_document_classifier()
        .document_classifier_arn(arn)
        .send()
        .await
        .unwrap();
    let props = desc.document_classifier_properties().unwrap();
    assert_eq!(
        props.status().unwrap().as_str(),
        "TRAINING",
        "newly created classifier should be TRAINING"
    );

    // stop_training changes status to STOP_REQUESTED
    client
        .stop_training_document_classifier()
        .document_classifier_arn(arn)
        .send()
        .await
        .unwrap();

    let desc2 = client
        .describe_document_classifier()
        .document_classifier_arn(arn)
        .send()
        .await
        .unwrap();
    let props2 = desc2.document_classifier_properties().unwrap();
    assert_eq!(
        props2.status().unwrap().as_str(),
        "STOP_REQUESTED",
        "after stop_training, status should be STOP_REQUESTED"
    );
}

/// Translated from moto test_stop_training_entity_recognizer:
/// entity recognizer starts as TRAINED and stop_training does NOT change its status.
#[tokio::test]
async fn test_entity_recognizer_stop_training_preserves_status() {
    let client = make_client().await;

    let resp = client
        .create_entity_recognizer()
        .recognizer_name("status-test-recognizer")
        .language_code(LanguageCode::En)
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .input_data_config(
            EntityRecognizerInputDataConfig::builder()
                .entity_types(
                    EntityTypesListItem::builder()
                        .r#type("PERSON")
                        .build()
                        .unwrap(),
                )
                .documents(
                    aws_sdk_comprehend::types::EntityRecognizerDocuments::builder()
                        .s3_uri("s3://my-bucket/docs")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let arn = resp.entity_recognizer_arn().unwrap();

    // Status after creation should be TRAINED
    let desc = client
        .describe_entity_recognizer()
        .entity_recognizer_arn(arn)
        .send()
        .await
        .unwrap();
    let props = desc.entity_recognizer_properties().unwrap();
    assert_eq!(
        props.status().unwrap().as_str(),
        "TRAINED",
        "newly created entity recognizer should be TRAINED"
    );

    // stop_training should NOT change status
    client
        .stop_training_entity_recognizer()
        .entity_recognizer_arn(arn)
        .send()
        .await
        .unwrap();

    let desc2 = client
        .describe_entity_recognizer()
        .entity_recognizer_arn(arn)
        .send()
        .await
        .unwrap();
    let props2 = desc2.entity_recognizer_properties().unwrap();
    assert_eq!(
        props2.status().unwrap().as_str(),
        "TRAINED",
        "after stop_training, entity recognizer status should remain TRAINED"
    );
}

/// Translated from moto test_create_entity_recognizer_with_tags:
/// tags passed at creation time are retrievable via list_tags_for_resource.
#[tokio::test]
async fn test_entity_recognizer_create_with_tags() {
    let client = make_client().await;

    let resp = client
        .create_entity_recognizer()
        .recognizer_name("tagged-recognizer")
        .language_code(LanguageCode::En)
        .data_access_role_arn("arn:aws:iam::123456789012:role/test-role")
        .tags(Tag::builder().key("k1").value("v1").build().unwrap())
        .input_data_config(
            EntityRecognizerInputDataConfig::builder()
                .entity_types(
                    EntityTypesListItem::builder()
                        .r#type("PERSON")
                        .build()
                        .unwrap(),
                )
                .documents(
                    aws_sdk_comprehend::types::EntityRecognizerDocuments::builder()
                        .s3_uri("s3://my-bucket/docs")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let arn = resp.entity_recognizer_arn().unwrap();

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .expect("list_tags should succeed");
    assert_eq!(tags_resp.resource_arn(), Some(arn));
    assert_eq!(tags_resp.tags().len(), 1);
    assert_eq!(tags_resp.tags()[0].key(), "k1");
    assert_eq!(tags_resp.tags()[0].value(), Some("v1"));
}
