use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::Value;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    json_error_response,
};

use crate::state::{ComprehendError, ComprehendState};
use crate::types::JobType;
use crate::views::ComprehendStateView;
use crate::wire;

pub struct ComprehendService {
    pub(crate) state: Arc<BackendState<ComprehendState>>,
    pub(crate) notifier: StateChangeNotifier<ComprehendStateView>,
}

impl ComprehendService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ComprehendService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ComprehendService {
    fn service_name(&self) -> &str {
        "comprehend"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://comprehend\..*\.amazonaws\.com",
            r"https?://comprehend\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

/// Common job-start payload, populated from each `Start*JobRequest` typed deserialiser.
struct StartJobInput {
    data_access_role_arn: String,
    input_s3_uri: String,
    output_s3_uri: String,
    job_name: Option<String>,
    language_code: Option<String>,
}

impl ComprehendService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').next_back())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => return comprehend_error(400, "MissingAction", "Missing X-Amz-Target header"),
        };

        // Validate the body is well-formed JSON up-front; the typed deserialisers in
        // `wire` re-parse the bytes per operation.
        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return comprehend_error(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        match action.as_str() {
            // ---- Detect operations (stateless) ----
            "DetectSentiment" => self.handle_detect_sentiment(body_bytes).await,
            "DetectEntities" => self.handle_detect_entities(body_bytes).await,
            "DetectDominantLanguage" => self.handle_detect_dominant_language(body_bytes).await,
            "DetectKeyPhrases" => self.handle_detect_key_phrases(body_bytes).await,
            "DetectPiiEntities" => self.handle_detect_pii_entities(body_bytes).await,

            // ---- Document Classifier ----
            "CreateDocumentClassifier" => {
                self.handle_create_document_classifier(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeDocumentClassifier" => {
                self.handle_describe_document_classifier(&state, body_bytes)
                    .await
            }
            "DeleteDocumentClassifier" => {
                self.handle_delete_document_classifier(&state, body_bytes)
                    .await
            }
            "ListDocumentClassifiers" => {
                self.handle_list_document_classifiers(&state, body_bytes)
                    .await
            }
            "StopTrainingDocumentClassifier" => {
                self.handle_stop_training_document_classifier(&state, body_bytes)
                    .await
            }

            // ---- Entity Recognizer ----
            "CreateEntityRecognizer" => {
                self.handle_create_entity_recognizer(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeEntityRecognizer" => {
                self.handle_describe_entity_recognizer(&state, body_bytes)
                    .await
            }
            "DeleteEntityRecognizer" => {
                self.handle_delete_entity_recognizer(&state, body_bytes)
                    .await
            }
            "ListEntityRecognizers" => {
                self.handle_list_entity_recognizers(&state, body_bytes)
                    .await
            }
            "StopTrainingEntityRecognizer" => {
                self.handle_stop_training_entity_recognizer(&state, body_bytes)
                    .await
            }

            // ---- Endpoint ----
            "CreateEndpoint" => {
                self.handle_create_endpoint(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeEndpoint" => self.handle_describe_endpoint(&state, body_bytes).await,
            "DeleteEndpoint" => self.handle_delete_endpoint(&state, body_bytes).await,
            "UpdateEndpoint" => self.handle_update_endpoint(&state, body_bytes).await,
            "ListEndpoints" => self.handle_list_endpoints(&state, body_bytes).await,

            // ---- Flywheel ----
            "CreateFlywheel" => {
                self.handle_create_flywheel(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeFlywheel" => self.handle_describe_flywheel(&state, body_bytes).await,
            "DeleteFlywheel" => self.handle_delete_flywheel(&state, body_bytes).await,
            "ListFlywheels" => self.handle_list_flywheels(&state, body_bytes).await,
            "StartFlywheelIteration" => {
                self.handle_start_flywheel_iteration(&state, body_bytes)
                    .await
            }

            // ---- Jobs: Start ----
            "StartDocumentClassificationJob" => {
                let req =
                    match wire::deserialize_start_document_classification_job_request(body_bytes) {
                        Ok(v) => v,
                        Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                    };
                let input = StartJobInput {
                    data_access_role_arn: req.data_access_role_arn,
                    input_s3_uri: if req.input_data_config.s3_uri.is_empty() {
                        "s3://default-bucket/input".to_string()
                    } else {
                        req.input_data_config.s3_uri
                    },
                    output_s3_uri: if req.output_data_config.s3_uri.is_empty() {
                        "s3://default-bucket/output".to_string()
                    } else {
                        req.output_data_config.s3_uri
                    },
                    job_name: req.job_name,
                    language_code: None,
                };
                self.start_job_with_input(
                    &state,
                    input,
                    JobType::DocumentClassification,
                    account_id,
                    &region,
                )
                .await
            }
            "StartDominantLanguageDetectionJob" => {
                let req = match wire::deserialize_start_dominant_language_detection_job_request(
                    body_bytes,
                ) {
                    Ok(v) => v,
                    Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                };
                let input = StartJobInput {
                    data_access_role_arn: req.data_access_role_arn,
                    input_s3_uri: if req.input_data_config.s3_uri.is_empty() {
                        "s3://default-bucket/input".to_string()
                    } else {
                        req.input_data_config.s3_uri
                    },
                    output_s3_uri: if req.output_data_config.s3_uri.is_empty() {
                        "s3://default-bucket/output".to_string()
                    } else {
                        req.output_data_config.s3_uri
                    },
                    job_name: req.job_name,
                    language_code: None,
                };
                self.start_job_with_input(
                    &state,
                    input,
                    JobType::DominantLanguageDetection,
                    account_id,
                    &region,
                )
                .await
            }
            "StartEntitiesDetectionJob" => {
                let req = match wire::deserialize_start_entities_detection_job_request(body_bytes) {
                    Ok(v) => v,
                    Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                };
                let input = StartJobInput {
                    data_access_role_arn: req.data_access_role_arn,
                    input_s3_uri: if req.input_data_config.s3_uri.is_empty() {
                        "s3://default-bucket/input".to_string()
                    } else {
                        req.input_data_config.s3_uri
                    },
                    output_s3_uri: if req.output_data_config.s3_uri.is_empty() {
                        "s3://default-bucket/output".to_string()
                    } else {
                        req.output_data_config.s3_uri
                    },
                    job_name: req.job_name,
                    language_code: if req.language_code.is_empty() {
                        None
                    } else {
                        Some(req.language_code)
                    },
                };
                self.start_job_with_input(
                    &state,
                    input,
                    JobType::EntitiesDetection,
                    account_id,
                    &region,
                )
                .await
            }
            "StartEventsDetectionJob" => {
                let req = match wire::deserialize_start_events_detection_job_request(body_bytes) {
                    Ok(v) => v,
                    Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                };
                let input = StartJobInput {
                    data_access_role_arn: req.data_access_role_arn,
                    input_s3_uri: if req.input_data_config.s3_uri.is_empty() {
                        "s3://default-bucket/input".to_string()
                    } else {
                        req.input_data_config.s3_uri
                    },
                    output_s3_uri: if req.output_data_config.s3_uri.is_empty() {
                        "s3://default-bucket/output".to_string()
                    } else {
                        req.output_data_config.s3_uri
                    },
                    job_name: req.job_name,
                    language_code: if req.language_code.is_empty() {
                        None
                    } else {
                        Some(req.language_code)
                    },
                };
                self.start_job_with_input(
                    &state,
                    input,
                    JobType::EventsDetection,
                    account_id,
                    &region,
                )
                .await
            }
            "StartKeyPhrasesDetectionJob" => {
                let req =
                    match wire::deserialize_start_key_phrases_detection_job_request(body_bytes) {
                        Ok(v) => v,
                        Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                    };
                let input = StartJobInput {
                    data_access_role_arn: req.data_access_role_arn,
                    input_s3_uri: if req.input_data_config.s3_uri.is_empty() {
                        "s3://default-bucket/input".to_string()
                    } else {
                        req.input_data_config.s3_uri
                    },
                    output_s3_uri: if req.output_data_config.s3_uri.is_empty() {
                        "s3://default-bucket/output".to_string()
                    } else {
                        req.output_data_config.s3_uri
                    },
                    job_name: req.job_name,
                    language_code: if req.language_code.is_empty() {
                        None
                    } else {
                        Some(req.language_code)
                    },
                };
                self.start_job_with_input(
                    &state,
                    input,
                    JobType::KeyPhrasesDetection,
                    account_id,
                    &region,
                )
                .await
            }
            "StartPiiEntitiesDetectionJob" => {
                let req =
                    match wire::deserialize_start_pii_entities_detection_job_request(body_bytes) {
                        Ok(v) => v,
                        Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                    };
                let input = StartJobInput {
                    data_access_role_arn: req.data_access_role_arn,
                    input_s3_uri: if req.input_data_config.s3_uri.is_empty() {
                        "s3://default-bucket/input".to_string()
                    } else {
                        req.input_data_config.s3_uri
                    },
                    output_s3_uri: if req.output_data_config.s3_uri.is_empty() {
                        "s3://default-bucket/output".to_string()
                    } else {
                        req.output_data_config.s3_uri
                    },
                    job_name: req.job_name,
                    language_code: if req.language_code.is_empty() {
                        None
                    } else {
                        Some(req.language_code)
                    },
                };
                self.start_job_with_input(
                    &state,
                    input,
                    JobType::PiiEntitiesDetection,
                    account_id,
                    &region,
                )
                .await
            }
            "StartSentimentDetectionJob" => {
                let req = match wire::deserialize_start_sentiment_detection_job_request(body_bytes)
                {
                    Ok(v) => v,
                    Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                };
                let input = StartJobInput {
                    data_access_role_arn: req.data_access_role_arn,
                    input_s3_uri: if req.input_data_config.s3_uri.is_empty() {
                        "s3://default-bucket/input".to_string()
                    } else {
                        req.input_data_config.s3_uri
                    },
                    output_s3_uri: if req.output_data_config.s3_uri.is_empty() {
                        "s3://default-bucket/output".to_string()
                    } else {
                        req.output_data_config.s3_uri
                    },
                    job_name: req.job_name,
                    language_code: if req.language_code.is_empty() {
                        None
                    } else {
                        Some(req.language_code)
                    },
                };
                self.start_job_with_input(
                    &state,
                    input,
                    JobType::SentimentDetection,
                    account_id,
                    &region,
                )
                .await
            }
            "StartTargetedSentimentDetectionJob" => {
                let req = match wire::deserialize_start_targeted_sentiment_detection_job_request(
                    body_bytes,
                ) {
                    Ok(v) => v,
                    Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                };
                let input = StartJobInput {
                    data_access_role_arn: req.data_access_role_arn,
                    input_s3_uri: if req.input_data_config.s3_uri.is_empty() {
                        "s3://default-bucket/input".to_string()
                    } else {
                        req.input_data_config.s3_uri
                    },
                    output_s3_uri: if req.output_data_config.s3_uri.is_empty() {
                        "s3://default-bucket/output".to_string()
                    } else {
                        req.output_data_config.s3_uri
                    },
                    job_name: req.job_name,
                    language_code: if req.language_code.is_empty() {
                        None
                    } else {
                        Some(req.language_code)
                    },
                };
                self.start_job_with_input(
                    &state,
                    input,
                    JobType::TargetedSentimentDetection,
                    account_id,
                    &region,
                )
                .await
            }
            "StartTopicsDetectionJob" => {
                let req = match wire::deserialize_start_topics_detection_job_request(body_bytes) {
                    Ok(v) => v,
                    Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                };
                let input = StartJobInput {
                    data_access_role_arn: req.data_access_role_arn,
                    input_s3_uri: if req.input_data_config.s3_uri.is_empty() {
                        "s3://default-bucket/input".to_string()
                    } else {
                        req.input_data_config.s3_uri
                    },
                    output_s3_uri: if req.output_data_config.s3_uri.is_empty() {
                        "s3://default-bucket/output".to_string()
                    } else {
                        req.output_data_config.s3_uri
                    },
                    job_name: req.job_name,
                    language_code: None,
                };
                self.start_job_with_input(
                    &state,
                    input,
                    JobType::TopicsDetection,
                    account_id,
                    &region,
                )
                .await
            }

            // ---- Jobs: Describe ----
            "DescribeDocumentClassificationJob" => {
                let job_id = match wire::deserialize_describe_document_classification_job_request(
                    body_bytes,
                ) {
                    Ok(v) => v.job_id,
                    Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                };
                self.handle_describe_job(&state, &job_id, &JobType::DocumentClassification)
                    .await
            }
            "DescribeDominantLanguageDetectionJob" => {
                let job_id =
                    match wire::deserialize_describe_dominant_language_detection_job_request(
                        body_bytes,
                    ) {
                        Ok(v) => v.job_id,
                        Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                    };
                self.handle_describe_job(&state, &job_id, &JobType::DominantLanguageDetection)
                    .await
            }
            "DescribeEntitiesDetectionJob" => {
                let job_id =
                    match wire::deserialize_describe_entities_detection_job_request(body_bytes) {
                        Ok(v) => v.job_id,
                        Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                    };
                self.handle_describe_job(&state, &job_id, &JobType::EntitiesDetection)
                    .await
            }
            "DescribeEventsDetectionJob" => {
                let job_id =
                    match wire::deserialize_describe_events_detection_job_request(body_bytes) {
                        Ok(v) => v.job_id,
                        Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                    };
                self.handle_describe_job(&state, &job_id, &JobType::EventsDetection)
                    .await
            }
            "DescribeKeyPhrasesDetectionJob" => {
                let job_id = match wire::deserialize_describe_key_phrases_detection_job_request(
                    body_bytes,
                ) {
                    Ok(v) => v.job_id,
                    Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                };
                self.handle_describe_job(&state, &job_id, &JobType::KeyPhrasesDetection)
                    .await
            }
            "DescribePiiEntitiesDetectionJob" => {
                let job_id =
                    match wire::deserialize_describe_pii_entities_detection_job_request(body_bytes)
                    {
                        Ok(v) => v.job_id,
                        Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                    };
                self.handle_describe_job(&state, &job_id, &JobType::PiiEntitiesDetection)
                    .await
            }
            "DescribeSentimentDetectionJob" => {
                let job_id =
                    match wire::deserialize_describe_sentiment_detection_job_request(body_bytes) {
                        Ok(v) => v.job_id,
                        Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                    };
                self.handle_describe_job(&state, &job_id, &JobType::SentimentDetection)
                    .await
            }
            "DescribeTargetedSentimentDetectionJob" => {
                let job_id =
                    match wire::deserialize_describe_targeted_sentiment_detection_job_request(
                        body_bytes,
                    ) {
                        Ok(v) => v.job_id,
                        Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                    };
                self.handle_describe_job(&state, &job_id, &JobType::TargetedSentimentDetection)
                    .await
            }
            "DescribeTopicsDetectionJob" => {
                let job_id =
                    match wire::deserialize_describe_topics_detection_job_request(body_bytes) {
                        Ok(v) => v.job_id,
                        Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                    };
                self.handle_describe_job(&state, &job_id, &JobType::TopicsDetection)
                    .await
            }

            // ---- Jobs: Stop ----
            "StopDominantLanguageDetectionJob" => {
                let job_id = match wire::deserialize_stop_dominant_language_detection_job_request(
                    body_bytes,
                ) {
                    Ok(v) => v.job_id,
                    Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                };
                self.handle_stop_job(&state, &job_id, &JobType::DominantLanguageDetection)
                    .await
            }
            "StopEntitiesDetectionJob" => {
                let job_id = match wire::deserialize_stop_entities_detection_job_request(body_bytes)
                {
                    Ok(v) => v.job_id,
                    Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                };
                self.handle_stop_job(&state, &job_id, &JobType::EntitiesDetection)
                    .await
            }
            "StopEventsDetectionJob" => {
                let job_id = match wire::deserialize_stop_events_detection_job_request(body_bytes) {
                    Ok(v) => v.job_id,
                    Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                };
                self.handle_stop_job(&state, &job_id, &JobType::EventsDetection)
                    .await
            }
            "StopKeyPhrasesDetectionJob" => {
                let job_id =
                    match wire::deserialize_stop_key_phrases_detection_job_request(body_bytes) {
                        Ok(v) => v.job_id,
                        Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                    };
                self.handle_stop_job(&state, &job_id, &JobType::KeyPhrasesDetection)
                    .await
            }
            "StopPiiEntitiesDetectionJob" => {
                let job_id =
                    match wire::deserialize_stop_pii_entities_detection_job_request(body_bytes) {
                        Ok(v) => v.job_id,
                        Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                    };
                self.handle_stop_job(&state, &job_id, &JobType::PiiEntitiesDetection)
                    .await
            }
            "StopSentimentDetectionJob" => {
                let job_id =
                    match wire::deserialize_stop_sentiment_detection_job_request(body_bytes) {
                        Ok(v) => v.job_id,
                        Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                    };
                self.handle_stop_job(&state, &job_id, &JobType::SentimentDetection)
                    .await
            }
            "StopTargetedSentimentDetectionJob" => {
                let job_id = match wire::deserialize_stop_targeted_sentiment_detection_job_request(
                    body_bytes,
                ) {
                    Ok(v) => v.job_id,
                    Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
                };
                self.handle_stop_job(&state, &job_id, &JobType::TargetedSentimentDetection)
                    .await
            }

            // ---- Jobs: List ----
            "ListDocumentClassificationJobs" => {
                self.handle_list_jobs(&state, &JobType::DocumentClassification)
                    .await
            }
            "ListDominantLanguageDetectionJobs" => {
                self.handle_list_jobs(&state, &JobType::DominantLanguageDetection)
                    .await
            }
            "ListEntitiesDetectionJobs" => {
                self.handle_list_jobs(&state, &JobType::EntitiesDetection)
                    .await
            }
            "ListEventsDetectionJobs" => {
                self.handle_list_jobs(&state, &JobType::EventsDetection)
                    .await
            }
            "ListKeyPhrasesDetectionJobs" => {
                self.handle_list_jobs(&state, &JobType::KeyPhrasesDetection)
                    .await
            }
            "ListPiiEntitiesDetectionJobs" => {
                self.handle_list_jobs(&state, &JobType::PiiEntitiesDetection)
                    .await
            }
            "ListSentimentDetectionJobs" => {
                self.handle_list_jobs(&state, &JobType::SentimentDetection)
                    .await
            }
            "ListTargetedSentimentDetectionJobs" => {
                self.handle_list_jobs(&state, &JobType::TargetedSentimentDetection)
                    .await
            }
            "ListTopicsDetectionJobs" => {
                self.handle_list_jobs(&state, &JobType::TopicsDetection)
                    .await
            }

            // ---- Resource Policy ----
            "PutResourcePolicy" => self.handle_put_resource_policy(&state, body_bytes).await,
            "DescribeResourcePolicy" => {
                self.handle_describe_resource_policy(&state, body_bytes)
                    .await
            }
            "DeleteResourcePolicy" => self.handle_delete_resource_policy(&state, body_bytes).await,

            // ---- Tags ----
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,

            // --- Unimplemented operations (auto-generated stubs) ---
            "BatchDetectDominantLanguage" => comprehend_error(
                501,
                "NotImplementedError",
                "BatchDetectDominantLanguage is not yet implemented in winterbaume-comprehend",
            ),
            "BatchDetectEntities" => comprehend_error(
                501,
                "NotImplementedError",
                "BatchDetectEntities is not yet implemented in winterbaume-comprehend",
            ),
            "BatchDetectKeyPhrases" => comprehend_error(
                501,
                "NotImplementedError",
                "BatchDetectKeyPhrases is not yet implemented in winterbaume-comprehend",
            ),
            "BatchDetectSentiment" => comprehend_error(
                501,
                "NotImplementedError",
                "BatchDetectSentiment is not yet implemented in winterbaume-comprehend",
            ),
            "BatchDetectSyntax" => comprehend_error(
                501,
                "NotImplementedError",
                "BatchDetectSyntax is not yet implemented in winterbaume-comprehend",
            ),
            "BatchDetectTargetedSentiment" => comprehend_error(
                501,
                "NotImplementedError",
                "BatchDetectTargetedSentiment is not yet implemented in winterbaume-comprehend",
            ),
            "ClassifyDocument" => comprehend_error(
                501,
                "NotImplementedError",
                "ClassifyDocument is not yet implemented in winterbaume-comprehend",
            ),
            "ContainsPiiEntities" => comprehend_error(
                501,
                "NotImplementedError",
                "ContainsPiiEntities is not yet implemented in winterbaume-comprehend",
            ),
            "CreateDataset" => comprehend_error(
                501,
                "NotImplementedError",
                "CreateDataset is not yet implemented in winterbaume-comprehend",
            ),
            "DescribeDataset" => comprehend_error(
                501,
                "NotImplementedError",
                "DescribeDataset is not yet implemented in winterbaume-comprehend",
            ),
            "DescribeFlywheelIteration" => comprehend_error(
                501,
                "NotImplementedError",
                "DescribeFlywheelIteration is not yet implemented in winterbaume-comprehend",
            ),
            "DetectSyntax" => comprehend_error(
                501,
                "NotImplementedError",
                "DetectSyntax is not yet implemented in winterbaume-comprehend",
            ),
            "DetectTargetedSentiment" => comprehend_error(
                501,
                "NotImplementedError",
                "DetectTargetedSentiment is not yet implemented in winterbaume-comprehend",
            ),
            "DetectToxicContent" => comprehend_error(
                501,
                "NotImplementedError",
                "DetectToxicContent is not yet implemented in winterbaume-comprehend",
            ),
            "ImportModel" => comprehend_error(
                501,
                "NotImplementedError",
                "ImportModel is not yet implemented in winterbaume-comprehend",
            ),
            "ListDatasets" => comprehend_error(
                501,
                "NotImplementedError",
                "ListDatasets is not yet implemented in winterbaume-comprehend",
            ),
            "ListDocumentClassifierSummaries" => comprehend_error(
                501,
                "NotImplementedError",
                "ListDocumentClassifierSummaries is not yet implemented in winterbaume-comprehend",
            ),
            "ListEntityRecognizerSummaries" => comprehend_error(
                501,
                "NotImplementedError",
                "ListEntityRecognizerSummaries is not yet implemented in winterbaume-comprehend",
            ),
            "ListFlywheelIterationHistory" => comprehend_error(
                501,
                "NotImplementedError",
                "ListFlywheelIterationHistory is not yet implemented in winterbaume-comprehend",
            ),
            "UpdateFlywheel" => comprehend_error(
                501,
                "NotImplementedError",
                "UpdateFlywheel is not yet implemented in winterbaume-comprehend",
            ),
            _ => comprehend_error(400, "InvalidAction", &format!("Unknown operation {action}")),
        }
    }

    // ---- Detect operations (stateless) ----

    // STUB[no-engine]: Sentiment detection requires a real NLP evaluation engine;
    //   returns a fixed neutral score.
    async fn handle_detect_sentiment(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_detect_sentiment_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.text.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "Text is required");
        }
        if input.language_code.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "LanguageCode is required");
        }

        wire::serialize_detect_sentiment_response(&wire::DetectSentimentResponse {
            sentiment: Some("NEUTRAL".to_string()),
            sentiment_score: Some(wire::SentimentScore {
                positive: Some(0.1),
                negative: Some(0.1),
                neutral: Some(0.7),
                mixed: Some(0.1),
            }),
        })
    }

    // STUB[no-engine]: Entity detection requires a real NLP evaluation engine;
    //   always returns an empty entity list.
    async fn handle_detect_entities(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_detect_entities_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.text.as_deref().unwrap_or("").is_empty() {
            return comprehend_error(400, "InvalidRequestException", "Text is required");
        }
        if input.language_code.as_deref().unwrap_or("").is_empty() {
            return comprehend_error(400, "InvalidRequestException", "LanguageCode is required");
        }

        wire::serialize_detect_entities_response(&wire::DetectEntitiesResponse {
            entities: Some(vec![]),
            ..Default::default()
        })
    }

    // STUB[no-engine]: Language detection requires a real NLP evaluation engine;
    //   always returns English with high confidence.
    async fn handle_detect_dominant_language(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_detect_dominant_language_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.text.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "Text is required");
        }

        wire::serialize_detect_dominant_language_response(&wire::DetectDominantLanguageResponse {
            languages: Some(vec![wire::DominantLanguage {
                language_code: Some("en".to_string()),
                score: Some(0.99),
            }]),
        })
    }

    // STUB[no-engine]: Key phrase detection requires a real NLP evaluation engine;
    //   always returns an empty phrase list.
    async fn handle_detect_key_phrases(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_detect_key_phrases_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.text.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "Text is required");
        }
        if input.language_code.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "LanguageCode is required");
        }

        wire::serialize_detect_key_phrases_response(&wire::DetectKeyPhrasesResponse {
            key_phrases: Some(vec![]),
        })
    }

    // STUB[no-engine]: PII entity detection requires a real NLP evaluation engine;
    //   always returns an empty entity list.
    async fn handle_detect_pii_entities(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_detect_pii_entities_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.text.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "Text is required");
        }
        if input.language_code.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "LanguageCode is required");
        }

        wire::serialize_detect_pii_entities_response(&wire::DetectPiiEntitiesResponse {
            entities: Some(vec![]),
        })
    }

    // ---- Document Classifier ----

    async fn handle_create_document_classifier(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_document_classifier_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.document_classifier_name.is_empty() {
            return comprehend_error(
                400,
                "InvalidRequestException",
                "DocumentClassifierName is required",
            );
        }
        if input.language_code.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "LanguageCode is required");
        }
        if input.data_access_role_arn.is_empty() {
            return comprehend_error(
                400,
                "InvalidRequestException",
                "DataAccessRoleArn is required",
            );
        }
        let input_s3_uri = input
            .input_data_config
            .s3_uri
            .as_deref()
            .filter(|s| !s.is_empty())
            .unwrap_or("s3://default-bucket/input")
            .to_string();
        let version_name = input.version_name;
        let tags = wire_tags_to_pairs(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_document_classifier(
            &input.document_classifier_name,
            &input.language_code,
            &input.data_access_role_arn,
            &input_s3_uri,
            version_name.as_deref(),
            account_id,
            region,
            &tags,
        ) {
            Ok(arn) => wire::serialize_create_document_classifier_response(
                &wire::CreateDocumentClassifierResponse {
                    document_classifier_arn: Some(arn),
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_describe_document_classifier(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_document_classifier_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.document_classifier_arn.is_empty() {
            return comprehend_error(
                400,
                "InvalidRequestException",
                "DocumentClassifierArn is required",
            );
        }
        let state = state.read().await;
        match state.describe_document_classifier(&input.document_classifier_arn) {
            Ok(dc) => wire::serialize_describe_document_classifier_response(
                &wire::DescribeDocumentClassifierResponse {
                    document_classifier_properties: Some(wire::DocumentClassifierProperties {
                        document_classifier_arn: Some(dc.arn.clone()),
                        language_code: Some(dc.language_code.clone()),
                        data_access_role_arn: Some(dc.data_access_role_arn.clone()),
                        status: Some(dc.status.clone()),
                        submit_time: Some(dc.submit_time),
                        ..Default::default()
                    }),
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_delete_document_classifier(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_document_classifier_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.document_classifier_arn.is_empty() {
            return comprehend_error(
                400,
                "InvalidRequestException",
                "DocumentClassifierArn is required",
            );
        }
        let mut state = state.write().await;
        match state.delete_document_classifier(&input.document_classifier_arn) {
            Ok(()) => wire::serialize_delete_document_classifier_response(
                &wire::DeleteDocumentClassifierResponse {},
            ),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_document_classifiers(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_document_classifiers_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        let name_filter = input
            .filter
            .as_ref()
            .and_then(|f| f.document_classifier_name.as_deref());
        let state = state.read().await;
        let classifiers = state.list_document_classifiers();
        let props: Vec<wire::DocumentClassifierProperties> = classifiers
            .iter()
            .filter(|dc| {
                if let Some(name) = name_filter {
                    dc.name == name
                } else {
                    true
                }
            })
            .map(|dc| wire::DocumentClassifierProperties {
                document_classifier_arn: Some(dc.arn.clone()),
                language_code: Some(dc.language_code.clone()),
                data_access_role_arn: Some(dc.data_access_role_arn.clone()),
                status: Some(dc.status.clone()),
                submit_time: Some(dc.submit_time),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_document_classifiers_response(&wire::ListDocumentClassifiersResponse {
            document_classifier_properties_list: Some(props),
            next_token: None,
        })
    }

    async fn handle_stop_training_document_classifier(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_training_document_classifier_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.document_classifier_arn.is_empty() {
            return comprehend_error(
                400,
                "InvalidRequestException",
                "DocumentClassifierArn is required",
            );
        }
        let mut state = state.write().await;
        match state.stop_training_document_classifier(&input.document_classifier_arn) {
            Ok(()) => wire::serialize_stop_training_document_classifier_response(
                &wire::StopTrainingDocumentClassifierResponse {},
            ),
            Err(e) => service_error_response(&e),
        }
    }

    // ---- Entity Recognizer ----

    async fn handle_create_entity_recognizer(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_entity_recognizer_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.recognizer_name.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "RecognizerName is required");
        }
        if input.language_code.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "LanguageCode is required");
        }
        if input.data_access_role_arn.is_empty() {
            return comprehend_error(
                400,
                "InvalidRequestException",
                "DataAccessRoleArn is required",
            );
        }
        let input_s3_uri = input
            .input_data_config
            .documents
            .as_ref()
            .map(|d| d.s3_uri.clone())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "s3://default-bucket/input".to_string());
        let entity_types: Vec<String> = input
            .input_data_config
            .entity_types
            .iter()
            .map(|et| et.r#type.clone())
            .collect();
        let version_name = input.version_name;
        let tags = wire_tags_to_pairs(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_entity_recognizer(
            &input.recognizer_name,
            &input.language_code,
            &input.data_access_role_arn,
            &input_s3_uri,
            entity_types,
            version_name.as_deref(),
            account_id,
            region,
            &tags,
        ) {
            Ok(arn) => wire::serialize_create_entity_recognizer_response(
                &wire::CreateEntityRecognizerResponse {
                    entity_recognizer_arn: Some(arn),
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_describe_entity_recognizer(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_entity_recognizer_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.entity_recognizer_arn.is_empty() {
            return comprehend_error(
                400,
                "InvalidRequestException",
                "EntityRecognizerArn is required",
            );
        }
        let state = state.read().await;
        match state.describe_entity_recognizer(&input.entity_recognizer_arn) {
            Ok(er) => wire::serialize_describe_entity_recognizer_response(
                &wire::DescribeEntityRecognizerResponse {
                    entity_recognizer_properties: Some(wire::EntityRecognizerProperties {
                        entity_recognizer_arn: Some(er.arn.clone()),
                        language_code: Some(er.language_code.clone()),
                        data_access_role_arn: Some(er.data_access_role_arn.clone()),
                        status: Some(er.status.clone()),
                        submit_time: Some(er.submit_time),
                        ..Default::default()
                    }),
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_delete_entity_recognizer(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_entity_recognizer_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.entity_recognizer_arn.is_empty() {
            return comprehend_error(
                400,
                "InvalidRequestException",
                "EntityRecognizerArn is required",
            );
        }
        let mut state = state.write().await;
        match state.delete_entity_recognizer(&input.entity_recognizer_arn) {
            Ok(()) => wire::serialize_delete_entity_recognizer_response(
                &wire::DeleteEntityRecognizerResponse {},
            ),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_entity_recognizers(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_entity_recognizers_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        let name_filter = input
            .filter
            .as_ref()
            .and_then(|f| f.recognizer_name.as_deref());
        let state = state.read().await;
        let recognizers = state.list_entity_recognizers();
        let props: Vec<wire::EntityRecognizerProperties> = recognizers
            .iter()
            .filter(|er| {
                if let Some(name) = name_filter {
                    er.name == name
                } else {
                    true
                }
            })
            .map(|er| wire::EntityRecognizerProperties {
                entity_recognizer_arn: Some(er.arn.clone()),
                language_code: Some(er.language_code.clone()),
                data_access_role_arn: Some(er.data_access_role_arn.clone()),
                status: Some(er.status.clone()),
                submit_time: Some(er.submit_time),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_entity_recognizers_response(&wire::ListEntityRecognizersResponse {
            entity_recognizer_properties_list: Some(props),
            next_token: None,
        })
    }

    async fn handle_stop_training_entity_recognizer(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_training_entity_recognizer_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.entity_recognizer_arn.is_empty() {
            return comprehend_error(
                400,
                "InvalidRequestException",
                "EntityRecognizerArn is required",
            );
        }
        let mut state = state.write().await;
        match state.stop_training_entity_recognizer(&input.entity_recognizer_arn) {
            Ok(()) => wire::serialize_stop_training_entity_recognizer_response(
                &wire::StopTrainingEntityRecognizerResponse {},
            ),
            Err(e) => service_error_response(&e),
        }
    }

    // ---- Endpoint ----

    async fn handle_create_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_endpoint_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.endpoint_name.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "EndpointName is required");
        }
        let model_arn = match input.model_arn.as_deref() {
            Some(m) if !m.is_empty() => m.to_string(),
            _ => {
                return comprehend_error(400, "InvalidRequestException", "ModelArn is required");
            }
        };
        let desired_inference_units = if input.desired_inference_units == 0 {
            1
        } else {
            input.desired_inference_units
        };

        let tags = wire_tags_to_pairs(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_endpoint(
            &input.endpoint_name,
            &model_arn,
            desired_inference_units,
            account_id,
            region,
            &tags,
        ) {
            Ok((arn, m_arn)) => {
                wire::serialize_create_endpoint_response(&wire::CreateEndpointResponse {
                    endpoint_arn: Some(arn),
                    model_arn: Some(m_arn),
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_describe_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_endpoint_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.endpoint_arn.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "EndpointArn is required");
        }
        let state = state.read().await;
        match state.describe_endpoint(&input.endpoint_arn) {
            Ok(ep) => wire::serialize_describe_endpoint_response(&wire::DescribeEndpointResponse {
                endpoint_properties: Some(wire::EndpointProperties {
                    endpoint_arn: Some(ep.arn.clone()),
                    model_arn: Some(ep.model_arn.clone()),
                    desired_model_arn: Some(ep.desired_model_arn.clone()),
                    desired_inference_units: Some(ep.desired_inference_units),
                    current_inference_units: Some(ep.current_inference_units),
                    status: Some(ep.status.clone()),
                    creation_time: Some(ep.creation_time),
                    last_modified_time: Some(ep.last_modified_time),
                    ..Default::default()
                }),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_delete_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_endpoint_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.endpoint_arn.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "EndpointArn is required");
        }
        let mut state = state.write().await;
        match state.delete_endpoint(&input.endpoint_arn) {
            Ok(()) => wire::serialize_delete_endpoint_response(&wire::DeleteEndpointResponse {}),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_update_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_endpoint_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.endpoint_arn.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "EndpointArn is required");
        }
        let desired_model_arn = input.desired_model_arn.as_deref();
        let desired_inference_units = input.desired_inference_units;

        let mut state = state.write().await;
        match state.update_endpoint(
            &input.endpoint_arn,
            desired_model_arn,
            desired_inference_units,
        ) {
            Ok(model_arn) => {
                wire::serialize_update_endpoint_response(&wire::UpdateEndpointResponse {
                    desired_model_arn: model_arn,
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_endpoints(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_endpoints_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        let status_filter = input.filter.as_ref().and_then(|f| f.status.as_deref());
        let state = state.read().await;
        let endpoints = state.list_endpoints();
        let props: Vec<wire::EndpointProperties> = endpoints
            .iter()
            .filter(|ep| {
                if let Some(status) = status_filter {
                    ep.status == status
                } else {
                    true
                }
            })
            .map(|ep| wire::EndpointProperties {
                endpoint_arn: Some(ep.arn.clone()),
                model_arn: Some(ep.model_arn.clone()),
                desired_model_arn: Some(ep.desired_model_arn.clone()),
                desired_inference_units: Some(ep.desired_inference_units),
                current_inference_units: Some(ep.current_inference_units),
                status: Some(ep.status.clone()),
                creation_time: Some(ep.creation_time),
                last_modified_time: Some(ep.last_modified_time),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_endpoints_response(&wire::ListEndpointsResponse {
            endpoint_properties_list: Some(props),
            next_token: None,
        })
    }

    // ---- Flywheel ----

    async fn handle_create_flywheel(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_flywheel_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.flywheel_name.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "FlywheelName is required");
        }
        let active_model_arn = match input.active_model_arn.as_deref() {
            Some(m) if !m.is_empty() => m.to_string(),
            _ => {
                return comprehend_error(
                    400,
                    "InvalidRequestException",
                    "ActiveModelArn is required",
                );
            }
        };
        if input.data_access_role_arn.is_empty() {
            return comprehend_error(
                400,
                "InvalidRequestException",
                "DataAccessRoleArn is required",
            );
        }
        if input.data_lake_s3_uri.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "DataLakeS3Uri is required");
        }
        let model_type = input
            .model_type
            .as_deref()
            .filter(|s| !s.is_empty())
            .unwrap_or("DOCUMENT_CLASSIFIER");

        let tags = wire_tags_to_pairs(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_flywheel(
            &input.flywheel_name,
            &active_model_arn,
            &input.data_access_role_arn,
            &input.data_lake_s3_uri,
            model_type,
            account_id,
            region,
            &tags,
        ) {
            Ok((arn, model_arn)) => {
                wire::serialize_create_flywheel_response(&wire::CreateFlywheelResponse {
                    flywheel_arn: Some(arn),
                    active_model_arn: Some(model_arn),
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_describe_flywheel(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_flywheel_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.flywheel_arn.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "FlywheelArn is required");
        }
        let state = state.read().await;
        match state.describe_flywheel(&input.flywheel_arn) {
            Ok(fw) => wire::serialize_describe_flywheel_response(&wire::DescribeFlywheelResponse {
                flywheel_properties: Some(wire::FlywheelProperties {
                    flywheel_arn: Some(fw.arn.clone()),
                    active_model_arn: Some(fw.active_model_arn.clone()),
                    data_access_role_arn: Some(fw.data_access_role_arn.clone()),
                    data_lake_s3_uri: Some(fw.data_lake_s3_uri.clone()),
                    model_type: Some(fw.model_type.clone()),
                    status: Some(fw.status.clone()),
                    creation_time: Some(fw.creation_time),
                    last_modified_time: Some(fw.last_modified_time),
                    ..Default::default()
                }),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_delete_flywheel(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_flywheel_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.flywheel_arn.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "FlywheelArn is required");
        }
        let mut state = state.write().await;
        match state.delete_flywheel(&input.flywheel_arn) {
            Ok(()) => wire::serialize_delete_flywheel_response(&wire::DeleteFlywheelResponse {}),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_flywheels(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_flywheels_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        let status_filter = input.filter.as_ref().and_then(|f| f.status.as_deref());
        let state = state.read().await;
        let flywheels = state.list_flywheels();
        let summaries: Vec<wire::FlywheelSummary> = flywheels
            .iter()
            .filter(|fw| {
                if let Some(status) = status_filter {
                    fw.status == status
                } else {
                    true
                }
            })
            .map(|fw| wire::FlywheelSummary {
                flywheel_arn: Some(fw.arn.clone()),
                active_model_arn: Some(fw.active_model_arn.clone()),
                data_lake_s3_uri: Some(fw.data_lake_s3_uri.clone()),
                model_type: Some(fw.model_type.clone()),
                status: Some(fw.status.clone()),
                creation_time: Some(fw.creation_time),
                last_modified_time: Some(fw.last_modified_time),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_flywheels_response(&wire::ListFlywheelsResponse {
            flywheel_summary_list: Some(summaries),
            next_token: None,
        })
    }

    async fn handle_start_flywheel_iteration(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_flywheel_iteration_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.flywheel_arn.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "FlywheelArn is required");
        }
        let state = state.read().await;
        match state.start_flywheel_iteration(&input.flywheel_arn) {
            Ok(iteration_id) => wire::serialize_start_flywheel_iteration_response(
                &wire::StartFlywheelIterationResponse {
                    flywheel_arn: Some(input.flywheel_arn.clone()),
                    flywheel_iteration_id: Some(iteration_id),
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    // ---- Generic Job handlers ----

    async fn start_job_with_input(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        input: StartJobInput,
        job_type: JobType,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        if input.data_access_role_arn.is_empty() {
            return comprehend_error(
                400,
                "InvalidRequestException",
                "DataAccessRoleArn is required",
            );
        }

        let mut state = state.write().await;
        let (job_id, job_arn) = state.start_job(
            input.job_name.as_deref(),
            &input.data_access_role_arn,
            &input.input_s3_uri,
            &input.output_s3_uri,
            input.language_code.as_deref(),
            job_type.clone(),
            account_id,
            region,
        );

        let status = "SUBMITTED".to_string();
        match job_type {
            JobType::DocumentClassification => {
                wire::serialize_start_document_classification_job_response(
                    &wire::StartDocumentClassificationJobResponse {
                        job_id: Some(job_id),
                        job_arn: Some(job_arn),
                        job_status: Some(status),
                        ..Default::default()
                    },
                )
            }
            JobType::DominantLanguageDetection => {
                wire::serialize_start_dominant_language_detection_job_response(
                    &wire::StartDominantLanguageDetectionJobResponse {
                        job_id: Some(job_id),
                        job_arn: Some(job_arn),
                        job_status: Some(status),
                    },
                )
            }
            JobType::EntitiesDetection => wire::serialize_start_entities_detection_job_response(
                &wire::StartEntitiesDetectionJobResponse {
                    job_id: Some(job_id),
                    job_arn: Some(job_arn),
                    job_status: Some(status),
                    ..Default::default()
                },
            ),
            JobType::EventsDetection => wire::serialize_start_events_detection_job_response(
                &wire::StartEventsDetectionJobResponse {
                    job_id: Some(job_id),
                    job_arn: Some(job_arn),
                    job_status: Some(status),
                },
            ),
            JobType::KeyPhrasesDetection => {
                wire::serialize_start_key_phrases_detection_job_response(
                    &wire::StartKeyPhrasesDetectionJobResponse {
                        job_id: Some(job_id),
                        job_arn: Some(job_arn),
                        job_status: Some(status),
                    },
                )
            }
            JobType::PiiEntitiesDetection => {
                wire::serialize_start_pii_entities_detection_job_response(
                    &wire::StartPiiEntitiesDetectionJobResponse {
                        job_id: Some(job_id),
                        job_arn: Some(job_arn),
                        job_status: Some(status),
                    },
                )
            }
            JobType::SentimentDetection => wire::serialize_start_sentiment_detection_job_response(
                &wire::StartSentimentDetectionJobResponse {
                    job_id: Some(job_id),
                    job_arn: Some(job_arn),
                    job_status: Some(status),
                },
            ),
            JobType::TargetedSentimentDetection => {
                wire::serialize_start_targeted_sentiment_detection_job_response(
                    &wire::StartTargetedSentimentDetectionJobResponse {
                        job_id: Some(job_id),
                        job_arn: Some(job_arn),
                        job_status: Some(status),
                    },
                )
            }
            JobType::TopicsDetection => wire::serialize_start_topics_detection_job_response(
                &wire::StartTopicsDetectionJobResponse {
                    job_id: Some(job_id),
                    job_arn: Some(job_arn),
                    job_status: Some(status),
                },
            ),
        }
    }

    async fn handle_describe_job(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        job_id: &str,
        job_type: &JobType,
    ) -> MockResponse {
        if job_id.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "JobId is required");
        }
        let state = state.read().await;
        match state.describe_job(job_id, job_type) {
            Ok(job) => build_describe_job_response(job, job_type),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_stop_job(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        job_id: &str,
        job_type: &JobType,
    ) -> MockResponse {
        if job_id.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "JobId is required");
        }
        let mut state = state.write().await;
        match state.stop_job(job_id, job_type) {
            Ok(job) => build_stop_job_response(job, job_type),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        job_type: &JobType,
    ) -> MockResponse {
        let state = state.read().await;
        let jobs = state.list_jobs(job_type);
        build_list_jobs_response(&jobs, job_type)
    }

    // ---- Resource Policy ----

    async fn handle_put_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.resource_arn.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "ResourceArn is required");
        }
        if input.resource_policy.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "ResourcePolicy is required");
        }
        let mut state = state.write().await;
        match state.put_resource_policy(&input.resource_arn, &input.resource_policy) {
            Ok(revision_id) => {
                wire::serialize_put_resource_policy_response(&wire::PutResourcePolicyResponse {
                    policy_revision_id: Some(revision_id),
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_describe_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.resource_arn.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "ResourceArn is required");
        }
        let state = state.read().await;
        match state.describe_resource_policy(&input.resource_arn) {
            Ok(rp) => wire::serialize_describe_resource_policy_response(
                &wire::DescribeResourcePolicyResponse {
                    resource_policy: Some(rp.policy.clone()),
                    policy_revision_id: Some(rp.revision_id.clone()),
                    creation_time: Some(rp.creation_time),
                    last_modified_time: Some(rp.last_modified_time),
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_delete_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.resource_arn.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "ResourceArn is required");
        }
        let mut state = state.write().await;
        match state.delete_resource_policy(&input.resource_arn) {
            Ok(()) => wire::serialize_delete_resource_policy_response(
                &wire::DeleteResourcePolicyResponse {},
            ),
            Err(e) => service_error_response(&e),
        }
    }

    // ---- Tags ----

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.resource_arn.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "ResourceArn is required");
        }
        let tags = wire_tags_to_pairs(Some(input.tags.as_slice()));
        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, &tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.resource_arn.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "ResourceArn is required");
        }
        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, &input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ComprehendState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return comprehend_error(400, "InvalidRequestException", &e),
        };
        if input.resource_arn.is_empty() {
            return comprehend_error(400, "InvalidRequestException", "ResourceArn is required");
        }
        let state = state.read().await;
        let tags = state.list_tags_for_resource(&input.resource_arn);
        let wire_tags: Vec<wire::Tag> = tags
            .iter()
            .map(|(k, v)| wire::Tag {
                key: k.clone(),
                value: Some(v.clone()),
            })
            .collect();
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            resource_arn: Some(input.resource_arn.clone()),
            tags: Some(wire_tags),
        })
    }
}

// ---- Helper functions ----

fn comprehend_error(status: u16, error_type: &str, message: &str) -> MockResponse {
    json_error_response(status, error_type, message)
}

fn service_error_response(e: &ComprehendError) -> MockResponse {
    let (status, error_type) = match e {
        ComprehendError::ResourceInUse { .. } => (400, "ResourceInUseException"),
        ComprehendError::ResourceNotFound { .. } => (400, "ResourceNotFoundException"),
        ComprehendError::JobNotFound { .. } => (400, "JobNotFoundException"),
    };
    json_error_response(status, error_type, &e.to_string())
}

fn wire_tags_to_pairs(tags: Option<&[wire::Tag]>) -> Vec<(String, String)> {
    tags.map(|arr| {
        arr.iter()
            .map(|t| (t.key.clone(), t.value.clone().unwrap_or_default()))
            .collect()
    })
    .unwrap_or_default()
}

fn build_common_job_props(
    job: &crate::types::ComprehendJob,
) -> (
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<f64>,
    Option<wire::InputDataConfig>,
    Option<wire::OutputDataConfig>,
    Option<String>,
) {
    (
        Some(job.job_id.clone()),
        Some(job.job_arn.clone()),
        job.job_name.clone(),
        Some(job.job_status.clone()),
        Some(job.data_access_role_arn.clone()),
        Some(job.submit_time),
        Some(wire::InputDataConfig {
            s3_uri: job.input_s3_uri.clone(),
            ..Default::default()
        }),
        Some(wire::OutputDataConfig {
            s3_uri: job.output_s3_uri.clone(),
            ..Default::default()
        }),
        job.language_code.clone(),
    )
}

fn build_describe_job_response(
    job: &crate::types::ComprehendJob,
    job_type: &JobType,
) -> MockResponse {
    let (
        job_id,
        job_arn,
        job_name,
        job_status,
        data_access_role_arn,
        submit_time,
        input_data_config,
        output_data_config,
        language_code,
    ) = build_common_job_props(job);

    match job_type {
        JobType::DocumentClassification => {
            wire::serialize_describe_document_classification_job_response(
                &wire::DescribeDocumentClassificationJobResponse {
                    document_classification_job_properties: Some(
                        wire::DocumentClassificationJobProperties {
                            job_id,
                            job_arn,
                            job_name,
                            job_status,
                            data_access_role_arn,
                            submit_time,
                            input_data_config,
                            output_data_config,
                            ..Default::default()
                        },
                    ),
                },
            )
        }
        JobType::DominantLanguageDetection => {
            wire::serialize_describe_dominant_language_detection_job_response(
                &wire::DescribeDominantLanguageDetectionJobResponse {
                    dominant_language_detection_job_properties: Some(
                        wire::DominantLanguageDetectionJobProperties {
                            job_id,
                            job_arn,
                            job_name,
                            job_status,
                            data_access_role_arn,
                            submit_time,
                            input_data_config,
                            output_data_config,
                            ..Default::default()
                        },
                    ),
                },
            )
        }
        JobType::EntitiesDetection => wire::serialize_describe_entities_detection_job_response(
            &wire::DescribeEntitiesDetectionJobResponse {
                entities_detection_job_properties: Some(wire::EntitiesDetectionJobProperties {
                    job_id,
                    job_arn,
                    job_name,
                    job_status,
                    data_access_role_arn,
                    submit_time,
                    input_data_config,
                    output_data_config,
                    language_code,
                    ..Default::default()
                }),
            },
        ),
        JobType::EventsDetection => wire::serialize_describe_events_detection_job_response(
            &wire::DescribeEventsDetectionJobResponse {
                events_detection_job_properties: Some(wire::EventsDetectionJobProperties {
                    job_id,
                    job_arn,
                    job_name,
                    job_status,
                    data_access_role_arn,
                    submit_time,
                    input_data_config,
                    output_data_config,
                    language_code,
                    ..Default::default()
                }),
            },
        ),
        JobType::KeyPhrasesDetection => {
            wire::serialize_describe_key_phrases_detection_job_response(
                &wire::DescribeKeyPhrasesDetectionJobResponse {
                    key_phrases_detection_job_properties: Some(
                        wire::KeyPhrasesDetectionJobProperties {
                            job_id,
                            job_arn,
                            job_name,
                            job_status,
                            data_access_role_arn,
                            submit_time,
                            input_data_config,
                            output_data_config,
                            language_code,
                            ..Default::default()
                        },
                    ),
                },
            )
        }
        JobType::PiiEntitiesDetection => {
            wire::serialize_describe_pii_entities_detection_job_response(
                &wire::DescribePiiEntitiesDetectionJobResponse {
                    pii_entities_detection_job_properties: Some(
                        wire::PiiEntitiesDetectionJobProperties {
                            job_id,
                            job_arn,
                            job_name,
                            job_status,
                            data_access_role_arn,
                            submit_time,
                            input_data_config,
                            language_code,
                            ..Default::default()
                        },
                    ),
                },
            )
        }
        JobType::SentimentDetection => wire::serialize_describe_sentiment_detection_job_response(
            &wire::DescribeSentimentDetectionJobResponse {
                sentiment_detection_job_properties: Some(wire::SentimentDetectionJobProperties {
                    job_id,
                    job_arn,
                    job_name,
                    job_status,
                    data_access_role_arn,
                    submit_time,
                    input_data_config,
                    output_data_config,
                    language_code,
                    ..Default::default()
                }),
            },
        ),
        JobType::TargetedSentimentDetection => {
            wire::serialize_describe_targeted_sentiment_detection_job_response(
                &wire::DescribeTargetedSentimentDetectionJobResponse {
                    targeted_sentiment_detection_job_properties: Some(
                        wire::TargetedSentimentDetectionJobProperties {
                            job_id,
                            job_arn,
                            job_name,
                            job_status,
                            data_access_role_arn,
                            submit_time,
                            input_data_config,
                            output_data_config,
                            language_code,
                            ..Default::default()
                        },
                    ),
                },
            )
        }
        JobType::TopicsDetection => wire::serialize_describe_topics_detection_job_response(
            &wire::DescribeTopicsDetectionJobResponse {
                topics_detection_job_properties: Some(wire::TopicsDetectionJobProperties {
                    job_id,
                    job_arn,
                    job_name,
                    job_status,
                    data_access_role_arn,
                    submit_time,
                    input_data_config,
                    output_data_config,
                    ..Default::default()
                }),
            },
        ),
    }
}

fn build_stop_job_response(job: &crate::types::ComprehendJob, job_type: &JobType) -> MockResponse {
    let job_id = Some(job.job_id.clone());
    let job_status = Some(job.job_status.clone());

    match job_type {
        JobType::DominantLanguageDetection => {
            wire::serialize_stop_dominant_language_detection_job_response(
                &wire::StopDominantLanguageDetectionJobResponse { job_id, job_status },
            )
        }
        JobType::EntitiesDetection => wire::serialize_stop_entities_detection_job_response(
            &wire::StopEntitiesDetectionJobResponse { job_id, job_status },
        ),
        JobType::EventsDetection => wire::serialize_stop_events_detection_job_response(
            &wire::StopEventsDetectionJobResponse { job_id, job_status },
        ),
        JobType::KeyPhrasesDetection => wire::serialize_stop_key_phrases_detection_job_response(
            &wire::StopKeyPhrasesDetectionJobResponse { job_id, job_status },
        ),
        JobType::PiiEntitiesDetection => wire::serialize_stop_pii_entities_detection_job_response(
            &wire::StopPiiEntitiesDetectionJobResponse { job_id, job_status },
        ),
        JobType::SentimentDetection => wire::serialize_stop_sentiment_detection_job_response(
            &wire::StopSentimentDetectionJobResponse { job_id, job_status },
        ),
        JobType::TargetedSentimentDetection => {
            wire::serialize_stop_targeted_sentiment_detection_job_response(
                &wire::StopTargetedSentimentDetectionJobResponse { job_id, job_status },
            )
        }
        _ => comprehend_error(400, "InvalidRequestException", "Cannot stop this job type"),
    }
}

fn build_list_jobs_response(
    jobs: &[&crate::types::ComprehendJob],
    job_type: &JobType,
) -> MockResponse {
    match job_type {
        JobType::DocumentClassification => {
            let props: Vec<wire::DocumentClassificationJobProperties> = jobs
                .iter()
                .map(|j| {
                    let (
                        job_id,
                        job_arn,
                        job_name,
                        job_status,
                        data_access_role_arn,
                        submit_time,
                        input_data_config,
                        output_data_config,
                        _,
                    ) = build_common_job_props(j);
                    wire::DocumentClassificationJobProperties {
                        job_id,
                        job_arn,
                        job_name,
                        job_status,
                        data_access_role_arn,
                        submit_time,
                        input_data_config,
                        output_data_config,
                        ..Default::default()
                    }
                })
                .collect();
            wire::serialize_list_document_classification_jobs_response(
                &wire::ListDocumentClassificationJobsResponse {
                    document_classification_job_properties_list: Some(props),
                    next_token: None,
                },
            )
        }
        JobType::DominantLanguageDetection => {
            let props: Vec<wire::DominantLanguageDetectionJobProperties> = jobs
                .iter()
                .map(|j| {
                    let (
                        job_id,
                        job_arn,
                        job_name,
                        job_status,
                        data_access_role_arn,
                        submit_time,
                        input_data_config,
                        output_data_config,
                        _,
                    ) = build_common_job_props(j);
                    wire::DominantLanguageDetectionJobProperties {
                        job_id,
                        job_arn,
                        job_name,
                        job_status,
                        data_access_role_arn,
                        submit_time,
                        input_data_config,
                        output_data_config,
                        ..Default::default()
                    }
                })
                .collect();
            wire::serialize_list_dominant_language_detection_jobs_response(
                &wire::ListDominantLanguageDetectionJobsResponse {
                    dominant_language_detection_job_properties_list: Some(props),
                    next_token: None,
                },
            )
        }
        JobType::EntitiesDetection => {
            let props: Vec<wire::EntitiesDetectionJobProperties> = jobs
                .iter()
                .map(|j| {
                    let (
                        job_id,
                        job_arn,
                        job_name,
                        job_status,
                        data_access_role_arn,
                        submit_time,
                        input_data_config,
                        output_data_config,
                        language_code,
                    ) = build_common_job_props(j);
                    wire::EntitiesDetectionJobProperties {
                        job_id,
                        job_arn,
                        job_name,
                        job_status,
                        data_access_role_arn,
                        submit_time,
                        input_data_config,
                        output_data_config,
                        language_code,
                        ..Default::default()
                    }
                })
                .collect();
            wire::serialize_list_entities_detection_jobs_response(
                &wire::ListEntitiesDetectionJobsResponse {
                    entities_detection_job_properties_list: Some(props),
                    next_token: None,
                },
            )
        }
        JobType::EventsDetection => {
            let props: Vec<wire::EventsDetectionJobProperties> = jobs
                .iter()
                .map(|j| {
                    let (
                        job_id,
                        job_arn,
                        job_name,
                        job_status,
                        data_access_role_arn,
                        submit_time,
                        input_data_config,
                        output_data_config,
                        language_code,
                    ) = build_common_job_props(j);
                    wire::EventsDetectionJobProperties {
                        job_id,
                        job_arn,
                        job_name,
                        job_status,
                        data_access_role_arn,
                        submit_time,
                        input_data_config,
                        output_data_config,
                        language_code,
                        ..Default::default()
                    }
                })
                .collect();
            wire::serialize_list_events_detection_jobs_response(
                &wire::ListEventsDetectionJobsResponse {
                    events_detection_job_properties_list: Some(props),
                    next_token: None,
                },
            )
        }
        JobType::KeyPhrasesDetection => {
            let props: Vec<wire::KeyPhrasesDetectionJobProperties> = jobs
                .iter()
                .map(|j| {
                    let (
                        job_id,
                        job_arn,
                        job_name,
                        job_status,
                        data_access_role_arn,
                        submit_time,
                        input_data_config,
                        output_data_config,
                        language_code,
                    ) = build_common_job_props(j);
                    wire::KeyPhrasesDetectionJobProperties {
                        job_id,
                        job_arn,
                        job_name,
                        job_status,
                        data_access_role_arn,
                        submit_time,
                        input_data_config,
                        output_data_config,
                        language_code,
                        ..Default::default()
                    }
                })
                .collect();
            wire::serialize_list_key_phrases_detection_jobs_response(
                &wire::ListKeyPhrasesDetectionJobsResponse {
                    key_phrases_detection_job_properties_list: Some(props),
                    next_token: None,
                },
            )
        }
        JobType::PiiEntitiesDetection => {
            let props: Vec<wire::PiiEntitiesDetectionJobProperties> = jobs
                .iter()
                .map(|j| {
                    let (
                        job_id,
                        job_arn,
                        job_name,
                        job_status,
                        data_access_role_arn,
                        submit_time,
                        input_data_config,
                        _output_data_config,
                        language_code,
                    ) = build_common_job_props(j);
                    wire::PiiEntitiesDetectionJobProperties {
                        job_id,
                        job_arn,
                        job_name,
                        job_status,
                        data_access_role_arn,
                        submit_time,
                        input_data_config,
                        language_code,
                        ..Default::default()
                    }
                })
                .collect();
            wire::serialize_list_pii_entities_detection_jobs_response(
                &wire::ListPiiEntitiesDetectionJobsResponse {
                    pii_entities_detection_job_properties_list: Some(props),
                    next_token: None,
                },
            )
        }
        JobType::SentimentDetection => {
            let props: Vec<wire::SentimentDetectionJobProperties> = jobs
                .iter()
                .map(|j| {
                    let (
                        job_id,
                        job_arn,
                        job_name,
                        job_status,
                        data_access_role_arn,
                        submit_time,
                        input_data_config,
                        output_data_config,
                        language_code,
                    ) = build_common_job_props(j);
                    wire::SentimentDetectionJobProperties {
                        job_id,
                        job_arn,
                        job_name,
                        job_status,
                        data_access_role_arn,
                        submit_time,
                        input_data_config,
                        output_data_config,
                        language_code,
                        ..Default::default()
                    }
                })
                .collect();
            wire::serialize_list_sentiment_detection_jobs_response(
                &wire::ListSentimentDetectionJobsResponse {
                    sentiment_detection_job_properties_list: Some(props),
                    next_token: None,
                },
            )
        }
        JobType::TargetedSentimentDetection => {
            let props: Vec<wire::TargetedSentimentDetectionJobProperties> = jobs
                .iter()
                .map(|j| {
                    let (
                        job_id,
                        job_arn,
                        job_name,
                        job_status,
                        data_access_role_arn,
                        submit_time,
                        input_data_config,
                        output_data_config,
                        language_code,
                    ) = build_common_job_props(j);
                    wire::TargetedSentimentDetectionJobProperties {
                        job_id,
                        job_arn,
                        job_name,
                        job_status,
                        data_access_role_arn,
                        submit_time,
                        input_data_config,
                        output_data_config,
                        language_code,
                        ..Default::default()
                    }
                })
                .collect();
            wire::serialize_list_targeted_sentiment_detection_jobs_response(
                &wire::ListTargetedSentimentDetectionJobsResponse {
                    targeted_sentiment_detection_job_properties_list: Some(props),
                    next_token: None,
                },
            )
        }
        JobType::TopicsDetection => {
            let props: Vec<wire::TopicsDetectionJobProperties> = jobs
                .iter()
                .map(|j| {
                    let (
                        job_id,
                        job_arn,
                        job_name,
                        job_status,
                        data_access_role_arn,
                        submit_time,
                        input_data_config,
                        output_data_config,
                        _,
                    ) = build_common_job_props(j);
                    wire::TopicsDetectionJobProperties {
                        job_id,
                        job_arn,
                        job_name,
                        job_status,
                        data_access_role_arn,
                        submit_time,
                        input_data_config,
                        output_data_config,
                        ..Default::default()
                    }
                })
                .collect();
            wire::serialize_list_topics_detection_jobs_response(
                &wire::ListTopicsDetectionJobsResponse {
                    topics_detection_job_properties_list: Some(props),
                    next_token: None,
                },
            )
        }
    }
}
