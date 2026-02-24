//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-comprehend

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_batch_detect_dominant_language_response(
    result: &BatchDetectDominantLanguageResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_detect_entities_response(
    result: &BatchDetectEntitiesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_detect_key_phrases_response(
    result: &BatchDetectKeyPhrasesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_detect_sentiment_response(
    result: &BatchDetectSentimentResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_detect_syntax_response(result: &BatchDetectSyntaxResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_detect_targeted_sentiment_response(
    result: &BatchDetectTargetedSentimentResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_classify_document_response(result: &ClassifyDocumentResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_contains_pii_entities_response(
    result: &ContainsPiiEntitiesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_dataset_response(result: &CreateDatasetResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_document_classifier_response(
    result: &CreateDocumentClassifierResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_endpoint_response(result: &CreateEndpointResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_entity_recognizer_response(
    result: &CreateEntityRecognizerResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_flywheel_response(result: &CreateFlywheelResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_document_classifier_response(
    result: &DeleteDocumentClassifierResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_endpoint_response(result: &DeleteEndpointResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_entity_recognizer_response(
    result: &DeleteEntityRecognizerResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_flywheel_response(result: &DeleteFlywheelResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_resource_policy_response(
    result: &DeleteResourcePolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_dataset_response(result: &DescribeDatasetResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_document_classification_job_response(
    result: &DescribeDocumentClassificationJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_document_classifier_response(
    result: &DescribeDocumentClassifierResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_dominant_language_detection_job_response(
    result: &DescribeDominantLanguageDetectionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_endpoint_response(result: &DescribeEndpointResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_entities_detection_job_response(
    result: &DescribeEntitiesDetectionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_entity_recognizer_response(
    result: &DescribeEntityRecognizerResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_events_detection_job_response(
    result: &DescribeEventsDetectionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_flywheel_response(result: &DescribeFlywheelResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_flywheel_iteration_response(
    result: &DescribeFlywheelIterationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_key_phrases_detection_job_response(
    result: &DescribeKeyPhrasesDetectionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_pii_entities_detection_job_response(
    result: &DescribePiiEntitiesDetectionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_resource_policy_response(
    result: &DescribeResourcePolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_sentiment_detection_job_response(
    result: &DescribeSentimentDetectionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_targeted_sentiment_detection_job_response(
    result: &DescribeTargetedSentimentDetectionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_topics_detection_job_response(
    result: &DescribeTopicsDetectionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_detect_dominant_language_response(
    result: &DetectDominantLanguageResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_detect_entities_response(result: &DetectEntitiesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_detect_key_phrases_response(result: &DetectKeyPhrasesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_detect_pii_entities_response(result: &DetectPiiEntitiesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_detect_sentiment_response(result: &DetectSentimentResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_detect_syntax_response(result: &DetectSyntaxResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_detect_targeted_sentiment_response(
    result: &DetectTargetedSentimentResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_detect_toxic_content_response(
    result: &DetectToxicContentResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_import_model_response(result: &ImportModelResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_datasets_response(result: &ListDatasetsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_document_classification_jobs_response(
    result: &ListDocumentClassificationJobsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_document_classifier_summaries_response(
    result: &ListDocumentClassifierSummariesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_document_classifiers_response(
    result: &ListDocumentClassifiersResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_dominant_language_detection_jobs_response(
    result: &ListDominantLanguageDetectionJobsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_endpoints_response(result: &ListEndpointsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_entities_detection_jobs_response(
    result: &ListEntitiesDetectionJobsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_entity_recognizer_summaries_response(
    result: &ListEntityRecognizerSummariesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_entity_recognizers_response(
    result: &ListEntityRecognizersResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_events_detection_jobs_response(
    result: &ListEventsDetectionJobsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_flywheel_iteration_history_response(
    result: &ListFlywheelIterationHistoryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_flywheels_response(result: &ListFlywheelsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_key_phrases_detection_jobs_response(
    result: &ListKeyPhrasesDetectionJobsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_pii_entities_detection_jobs_response(
    result: &ListPiiEntitiesDetectionJobsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_sentiment_detection_jobs_response(
    result: &ListSentimentDetectionJobsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_targeted_sentiment_detection_jobs_response(
    result: &ListTargetedSentimentDetectionJobsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_topics_detection_jobs_response(
    result: &ListTopicsDetectionJobsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_resource_policy_response(result: &PutResourcePolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_document_classification_job_response(
    result: &StartDocumentClassificationJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_dominant_language_detection_job_response(
    result: &StartDominantLanguageDetectionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_entities_detection_job_response(
    result: &StartEntitiesDetectionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_events_detection_job_response(
    result: &StartEventsDetectionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_flywheel_iteration_response(
    result: &StartFlywheelIterationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_key_phrases_detection_job_response(
    result: &StartKeyPhrasesDetectionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_pii_entities_detection_job_response(
    result: &StartPiiEntitiesDetectionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_sentiment_detection_job_response(
    result: &StartSentimentDetectionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_targeted_sentiment_detection_job_response(
    result: &StartTargetedSentimentDetectionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_topics_detection_job_response(
    result: &StartTopicsDetectionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_dominant_language_detection_job_response(
    result: &StopDominantLanguageDetectionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_entities_detection_job_response(
    result: &StopEntitiesDetectionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_events_detection_job_response(
    result: &StopEventsDetectionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_key_phrases_detection_job_response(
    result: &StopKeyPhrasesDetectionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_pii_entities_detection_job_response(
    result: &StopPiiEntitiesDetectionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_sentiment_detection_job_response(
    result: &StopSentimentDetectionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_targeted_sentiment_detection_job_response(
    result: &StopTargetedSentimentDetectionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_training_document_classifier_response(
    result: &StopTrainingDocumentClassifierResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_training_entity_recognizer_response(
    result: &StopTrainingEntityRecognizerResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_endpoint_response(result: &UpdateEndpointResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_flywheel_response(result: &UpdateFlywheelResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_detect_dominant_language_request(
    body: &[u8],
) -> Result<BatchDetectDominantLanguageRequest, String> {
    if body.is_empty() {
        return Ok(BatchDetectDominantLanguageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchDetectDominantLanguage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_detect_entities_request(
    body: &[u8],
) -> Result<BatchDetectEntitiesRequest, String> {
    if body.is_empty() {
        return Ok(BatchDetectEntitiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchDetectEntities request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_detect_key_phrases_request(
    body: &[u8],
) -> Result<BatchDetectKeyPhrasesRequest, String> {
    if body.is_empty() {
        return Ok(BatchDetectKeyPhrasesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchDetectKeyPhrases request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_detect_sentiment_request(
    body: &[u8],
) -> Result<BatchDetectSentimentRequest, String> {
    if body.is_empty() {
        return Ok(BatchDetectSentimentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchDetectSentiment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_detect_syntax_request(
    body: &[u8],
) -> Result<BatchDetectSyntaxRequest, String> {
    if body.is_empty() {
        return Ok(BatchDetectSyntaxRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchDetectSyntax request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_detect_targeted_sentiment_request(
    body: &[u8],
) -> Result<BatchDetectTargetedSentimentRequest, String> {
    if body.is_empty() {
        return Ok(BatchDetectTargetedSentimentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchDetectTargetedSentiment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_classify_document_request(
    body: &[u8],
) -> Result<ClassifyDocumentRequest, String> {
    if body.is_empty() {
        return Ok(ClassifyDocumentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ClassifyDocument request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_contains_pii_entities_request(
    body: &[u8],
) -> Result<ContainsPiiEntitiesRequest, String> {
    if body.is_empty() {
        return Ok(ContainsPiiEntitiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ContainsPiiEntities request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_dataset_request(body: &[u8]) -> Result<CreateDatasetRequest, String> {
    if body.is_empty() {
        return Ok(CreateDatasetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDataset request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_document_classifier_request(
    body: &[u8],
) -> Result<CreateDocumentClassifierRequest, String> {
    if body.is_empty() {
        return Ok(CreateDocumentClassifierRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateDocumentClassifier request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_endpoint_request(body: &[u8]) -> Result<CreateEndpointRequest, String> {
    if body.is_empty() {
        return Ok(CreateEndpointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_entity_recognizer_request(
    body: &[u8],
) -> Result<CreateEntityRecognizerRequest, String> {
    if body.is_empty() {
        return Ok(CreateEntityRecognizerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateEntityRecognizer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_flywheel_request(body: &[u8]) -> Result<CreateFlywheelRequest, String> {
    if body.is_empty() {
        return Ok(CreateFlywheelRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateFlywheel request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_document_classifier_request(
    body: &[u8],
) -> Result<DeleteDocumentClassifierRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDocumentClassifierRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDocumentClassifier request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_endpoint_request(body: &[u8]) -> Result<DeleteEndpointRequest, String> {
    if body.is_empty() {
        return Ok(DeleteEndpointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_entity_recognizer_request(
    body: &[u8],
) -> Result<DeleteEntityRecognizerRequest, String> {
    if body.is_empty() {
        return Ok(DeleteEntityRecognizerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteEntityRecognizer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_flywheel_request(body: &[u8]) -> Result<DeleteFlywheelRequest, String> {
    if body.is_empty() {
        return Ok(DeleteFlywheelRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteFlywheel request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_resource_policy_request(
    body: &[u8],
) -> Result<DeleteResourcePolicyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteResourcePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_dataset_request(body: &[u8]) -> Result<DescribeDatasetRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDatasetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDataset request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_document_classification_job_request(
    body: &[u8],
) -> Result<DescribeDocumentClassificationJobRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDocumentClassificationJobRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeDocumentClassificationJob request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_document_classifier_request(
    body: &[u8],
) -> Result<DescribeDocumentClassifierRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDocumentClassifierRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeDocumentClassifier request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_dominant_language_detection_job_request(
    body: &[u8],
) -> Result<DescribeDominantLanguageDetectionJobRequest, String> {
    if body.is_empty() {
        return Ok(DescribeDominantLanguageDetectionJobRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeDominantLanguageDetectionJob request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_endpoint_request(
    body: &[u8],
) -> Result<DescribeEndpointRequest, String> {
    if body.is_empty() {
        return Ok(DescribeEndpointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_entities_detection_job_request(
    body: &[u8],
) -> Result<DescribeEntitiesDetectionJobRequest, String> {
    if body.is_empty() {
        return Ok(DescribeEntitiesDetectionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEntitiesDetectionJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_entity_recognizer_request(
    body: &[u8],
) -> Result<DescribeEntityRecognizerRequest, String> {
    if body.is_empty() {
        return Ok(DescribeEntityRecognizerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEntityRecognizer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_events_detection_job_request(
    body: &[u8],
) -> Result<DescribeEventsDetectionJobRequest, String> {
    if body.is_empty() {
        return Ok(DescribeEventsDetectionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeEventsDetectionJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_flywheel_request(
    body: &[u8],
) -> Result<DescribeFlywheelRequest, String> {
    if body.is_empty() {
        return Ok(DescribeFlywheelRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeFlywheel request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_flywheel_iteration_request(
    body: &[u8],
) -> Result<DescribeFlywheelIterationRequest, String> {
    if body.is_empty() {
        return Ok(DescribeFlywheelIterationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeFlywheelIteration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_key_phrases_detection_job_request(
    body: &[u8],
) -> Result<DescribeKeyPhrasesDetectionJobRequest, String> {
    if body.is_empty() {
        return Ok(DescribeKeyPhrasesDetectionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeKeyPhrasesDetectionJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_pii_entities_detection_job_request(
    body: &[u8],
) -> Result<DescribePiiEntitiesDetectionJobRequest, String> {
    if body.is_empty() {
        return Ok(DescribePiiEntitiesDetectionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribePiiEntitiesDetectionJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_resource_policy_request(
    body: &[u8],
) -> Result<DescribeResourcePolicyRequest, String> {
    if body.is_empty() {
        return Ok(DescribeResourcePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_sentiment_detection_job_request(
    body: &[u8],
) -> Result<DescribeSentimentDetectionJobRequest, String> {
    if body.is_empty() {
        return Ok(DescribeSentimentDetectionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeSentimentDetectionJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_targeted_sentiment_detection_job_request(
    body: &[u8],
) -> Result<DescribeTargetedSentimentDetectionJobRequest, String> {
    if body.is_empty() {
        return Ok(DescribeTargetedSentimentDetectionJobRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeTargetedSentimentDetectionJob request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_topics_detection_job_request(
    body: &[u8],
) -> Result<DescribeTopicsDetectionJobRequest, String> {
    if body.is_empty() {
        return Ok(DescribeTopicsDetectionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeTopicsDetectionJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_detect_dominant_language_request(
    body: &[u8],
) -> Result<DetectDominantLanguageRequest, String> {
    if body.is_empty() {
        return Ok(DetectDominantLanguageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DetectDominantLanguage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_detect_entities_request(body: &[u8]) -> Result<DetectEntitiesRequest, String> {
    if body.is_empty() {
        return Ok(DetectEntitiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DetectEntities request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_detect_key_phrases_request(
    body: &[u8],
) -> Result<DetectKeyPhrasesRequest, String> {
    if body.is_empty() {
        return Ok(DetectKeyPhrasesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DetectKeyPhrases request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_detect_pii_entities_request(
    body: &[u8],
) -> Result<DetectPiiEntitiesRequest, String> {
    if body.is_empty() {
        return Ok(DetectPiiEntitiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DetectPiiEntities request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_detect_sentiment_request(body: &[u8]) -> Result<DetectSentimentRequest, String> {
    if body.is_empty() {
        return Ok(DetectSentimentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DetectSentiment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_detect_syntax_request(body: &[u8]) -> Result<DetectSyntaxRequest, String> {
    if body.is_empty() {
        return Ok(DetectSyntaxRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DetectSyntax request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_detect_targeted_sentiment_request(
    body: &[u8],
) -> Result<DetectTargetedSentimentRequest, String> {
    if body.is_empty() {
        return Ok(DetectTargetedSentimentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DetectTargetedSentiment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_detect_toxic_content_request(
    body: &[u8],
) -> Result<DetectToxicContentRequest, String> {
    if body.is_empty() {
        return Ok(DetectToxicContentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DetectToxicContent request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_import_model_request(body: &[u8]) -> Result<ImportModelRequest, String> {
    if body.is_empty() {
        return Ok(ImportModelRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ImportModel request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_datasets_request(body: &[u8]) -> Result<ListDatasetsRequest, String> {
    if body.is_empty() {
        return Ok(ListDatasetsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDatasets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_document_classification_jobs_request(
    body: &[u8],
) -> Result<ListDocumentClassificationJobsRequest, String> {
    if body.is_empty() {
        return Ok(ListDocumentClassificationJobsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDocumentClassificationJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_document_classifier_summaries_request(
    body: &[u8],
) -> Result<ListDocumentClassifierSummariesRequest, String> {
    if body.is_empty() {
        return Ok(ListDocumentClassifierSummariesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDocumentClassifierSummaries request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_document_classifiers_request(
    body: &[u8],
) -> Result<ListDocumentClassifiersRequest, String> {
    if body.is_empty() {
        return Ok(ListDocumentClassifiersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDocumentClassifiers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_dominant_language_detection_jobs_request(
    body: &[u8],
) -> Result<ListDominantLanguageDetectionJobsRequest, String> {
    if body.is_empty() {
        return Ok(ListDominantLanguageDetectionJobsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListDominantLanguageDetectionJobs request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_endpoints_request(body: &[u8]) -> Result<ListEndpointsRequest, String> {
    if body.is_empty() {
        return Ok(ListEndpointsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListEndpoints request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_entities_detection_jobs_request(
    body: &[u8],
) -> Result<ListEntitiesDetectionJobsRequest, String> {
    if body.is_empty() {
        return Ok(ListEntitiesDetectionJobsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListEntitiesDetectionJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_entity_recognizer_summaries_request(
    body: &[u8],
) -> Result<ListEntityRecognizerSummariesRequest, String> {
    if body.is_empty() {
        return Ok(ListEntityRecognizerSummariesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListEntityRecognizerSummaries request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_entity_recognizers_request(
    body: &[u8],
) -> Result<ListEntityRecognizersRequest, String> {
    if body.is_empty() {
        return Ok(ListEntityRecognizersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListEntityRecognizers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_events_detection_jobs_request(
    body: &[u8],
) -> Result<ListEventsDetectionJobsRequest, String> {
    if body.is_empty() {
        return Ok(ListEventsDetectionJobsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListEventsDetectionJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_flywheel_iteration_history_request(
    body: &[u8],
) -> Result<ListFlywheelIterationHistoryRequest, String> {
    if body.is_empty() {
        return Ok(ListFlywheelIterationHistoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListFlywheelIterationHistory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_flywheels_request(body: &[u8]) -> Result<ListFlywheelsRequest, String> {
    if body.is_empty() {
        return Ok(ListFlywheelsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListFlywheels request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_key_phrases_detection_jobs_request(
    body: &[u8],
) -> Result<ListKeyPhrasesDetectionJobsRequest, String> {
    if body.is_empty() {
        return Ok(ListKeyPhrasesDetectionJobsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListKeyPhrasesDetectionJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_pii_entities_detection_jobs_request(
    body: &[u8],
) -> Result<ListPiiEntitiesDetectionJobsRequest, String> {
    if body.is_empty() {
        return Ok(ListPiiEntitiesDetectionJobsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListPiiEntitiesDetectionJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_sentiment_detection_jobs_request(
    body: &[u8],
) -> Result<ListSentimentDetectionJobsRequest, String> {
    if body.is_empty() {
        return Ok(ListSentimentDetectionJobsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListSentimentDetectionJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    body: &[u8],
) -> Result<ListTagsForResourceRequest, String> {
    if body.is_empty() {
        return Ok(ListTagsForResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTagsForResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_targeted_sentiment_detection_jobs_request(
    body: &[u8],
) -> Result<ListTargetedSentimentDetectionJobsRequest, String> {
    if body.is_empty() {
        return Ok(ListTargetedSentimentDetectionJobsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListTargetedSentimentDetectionJobs request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_topics_detection_jobs_request(
    body: &[u8],
) -> Result<ListTopicsDetectionJobsRequest, String> {
    if body.is_empty() {
        return Ok(ListTopicsDetectionJobsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTopicsDetectionJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_resource_policy_request(
    body: &[u8],
) -> Result<PutResourcePolicyRequest, String> {
    if body.is_empty() {
        return Ok(PutResourcePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutResourcePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_document_classification_job_request(
    body: &[u8],
) -> Result<StartDocumentClassificationJobRequest, String> {
    if body.is_empty() {
        return Ok(StartDocumentClassificationJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartDocumentClassificationJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_dominant_language_detection_job_request(
    body: &[u8],
) -> Result<StartDominantLanguageDetectionJobRequest, String> {
    if body.is_empty() {
        return Ok(StartDominantLanguageDetectionJobRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize StartDominantLanguageDetectionJob request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_entities_detection_job_request(
    body: &[u8],
) -> Result<StartEntitiesDetectionJobRequest, String> {
    if body.is_empty() {
        return Ok(StartEntitiesDetectionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartEntitiesDetectionJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_events_detection_job_request(
    body: &[u8],
) -> Result<StartEventsDetectionJobRequest, String> {
    if body.is_empty() {
        return Ok(StartEventsDetectionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartEventsDetectionJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_flywheel_iteration_request(
    body: &[u8],
) -> Result<StartFlywheelIterationRequest, String> {
    if body.is_empty() {
        return Ok(StartFlywheelIterationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartFlywheelIteration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_key_phrases_detection_job_request(
    body: &[u8],
) -> Result<StartKeyPhrasesDetectionJobRequest, String> {
    if body.is_empty() {
        return Ok(StartKeyPhrasesDetectionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartKeyPhrasesDetectionJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_pii_entities_detection_job_request(
    body: &[u8],
) -> Result<StartPiiEntitiesDetectionJobRequest, String> {
    if body.is_empty() {
        return Ok(StartPiiEntitiesDetectionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartPiiEntitiesDetectionJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_sentiment_detection_job_request(
    body: &[u8],
) -> Result<StartSentimentDetectionJobRequest, String> {
    if body.is_empty() {
        return Ok(StartSentimentDetectionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartSentimentDetectionJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_targeted_sentiment_detection_job_request(
    body: &[u8],
) -> Result<StartTargetedSentimentDetectionJobRequest, String> {
    if body.is_empty() {
        return Ok(StartTargetedSentimentDetectionJobRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize StartTargetedSentimentDetectionJob request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_topics_detection_job_request(
    body: &[u8],
) -> Result<StartTopicsDetectionJobRequest, String> {
    if body.is_empty() {
        return Ok(StartTopicsDetectionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartTopicsDetectionJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_dominant_language_detection_job_request(
    body: &[u8],
) -> Result<StopDominantLanguageDetectionJobRequest, String> {
    if body.is_empty() {
        return Ok(StopDominantLanguageDetectionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopDominantLanguageDetectionJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_entities_detection_job_request(
    body: &[u8],
) -> Result<StopEntitiesDetectionJobRequest, String> {
    if body.is_empty() {
        return Ok(StopEntitiesDetectionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopEntitiesDetectionJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_events_detection_job_request(
    body: &[u8],
) -> Result<StopEventsDetectionJobRequest, String> {
    if body.is_empty() {
        return Ok(StopEventsDetectionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopEventsDetectionJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_key_phrases_detection_job_request(
    body: &[u8],
) -> Result<StopKeyPhrasesDetectionJobRequest, String> {
    if body.is_empty() {
        return Ok(StopKeyPhrasesDetectionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopKeyPhrasesDetectionJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_pii_entities_detection_job_request(
    body: &[u8],
) -> Result<StopPiiEntitiesDetectionJobRequest, String> {
    if body.is_empty() {
        return Ok(StopPiiEntitiesDetectionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopPiiEntitiesDetectionJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_sentiment_detection_job_request(
    body: &[u8],
) -> Result<StopSentimentDetectionJobRequest, String> {
    if body.is_empty() {
        return Ok(StopSentimentDetectionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopSentimentDetectionJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_targeted_sentiment_detection_job_request(
    body: &[u8],
) -> Result<StopTargetedSentimentDetectionJobRequest, String> {
    if body.is_empty() {
        return Ok(StopTargetedSentimentDetectionJobRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize StopTargetedSentimentDetectionJob request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_training_document_classifier_request(
    body: &[u8],
) -> Result<StopTrainingDocumentClassifierRequest, String> {
    if body.is_empty() {
        return Ok(StopTrainingDocumentClassifierRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopTrainingDocumentClassifier request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_training_entity_recognizer_request(
    body: &[u8],
) -> Result<StopTrainingEntityRecognizerRequest, String> {
    if body.is_empty() {
        return Ok(StopTrainingEntityRecognizerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopTrainingEntityRecognizer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_tag_resource_request(body: &[u8]) -> Result<TagResourceRequest, String> {
    if body.is_empty() {
        return Ok(TagResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_untag_resource_request(body: &[u8]) -> Result<UntagResourceRequest, String> {
    if body.is_empty() {
        return Ok(UntagResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UntagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_endpoint_request(body: &[u8]) -> Result<UpdateEndpointRequest, String> {
    if body.is_empty() {
        return Ok(UpdateEndpointRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateEndpoint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_flywheel_request(body: &[u8]) -> Result<UpdateFlywheelRequest, String> {
    if body.is_empty() {
        return Ok(UpdateFlywheelRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateFlywheel request: {e}"))
}
