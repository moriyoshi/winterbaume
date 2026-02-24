//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-transcribe

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_create_call_analytics_category_response(
    result: &CreateCallAnalyticsCategoryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_language_model_response(
    result: &CreateLanguageModelResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_medical_vocabulary_response(
    result: &CreateMedicalVocabularyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_vocabulary_response(result: &CreateVocabularyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_vocabulary_filter_response(
    result: &CreateVocabularyFilterResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(201, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_call_analytics_category_response(
    result: &DeleteCallAnalyticsCategoryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(204, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_call_analytics_job_response(
    result: &DeleteCallAnalyticsJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(204, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_language_model_response() -> MockResponse {
    MockResponse::json(204, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_medical_scribe_job_response() -> MockResponse {
    MockResponse::json(204, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_medical_transcription_job_response() -> MockResponse {
    MockResponse::json(204, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_medical_vocabulary_response() -> MockResponse {
    MockResponse::json(204, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_transcription_job_response() -> MockResponse {
    MockResponse::json(204, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_vocabulary_response() -> MockResponse {
    MockResponse::json(204, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_vocabulary_filter_response() -> MockResponse {
    MockResponse::json(204, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_language_model_response(
    result: &DescribeLanguageModelResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_call_analytics_category_response(
    result: &GetCallAnalyticsCategoryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_call_analytics_job_response(
    result: &GetCallAnalyticsJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_medical_scribe_job_response(
    result: &GetMedicalScribeJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_medical_transcription_job_response(
    result: &GetMedicalTranscriptionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_medical_vocabulary_response(
    result: &GetMedicalVocabularyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_transcription_job_response(
    result: &GetTranscriptionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_vocabulary_response(result: &GetVocabularyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_vocabulary_filter_response(
    result: &GetVocabularyFilterResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_call_analytics_categories_response(
    result: &ListCallAnalyticsCategoriesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_call_analytics_jobs_response(
    result: &ListCallAnalyticsJobsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_language_models_response(
    result: &ListLanguageModelsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_medical_scribe_jobs_response(
    result: &ListMedicalScribeJobsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_medical_transcription_jobs_response(
    result: &ListMedicalTranscriptionJobsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_medical_vocabularies_response(
    result: &ListMedicalVocabulariesResponse,
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
pub fn serialize_list_transcription_jobs_response(
    result: &ListTranscriptionJobsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_vocabularies_response(result: &ListVocabulariesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_vocabulary_filters_response(
    result: &ListVocabularyFiltersResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_call_analytics_job_response(
    result: &StartCallAnalyticsJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_medical_scribe_job_response(
    result: &StartMedicalScribeJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_medical_transcription_job_response(
    result: &StartMedicalTranscriptionJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_transcription_job_response(
    result: &StartTranscriptionJobResponse,
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
    MockResponse::json(204, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_call_analytics_category_response(
    result: &UpdateCallAnalyticsCategoryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_medical_vocabulary_response(
    result: &UpdateMedicalVocabularyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_vocabulary_response(result: &UpdateVocabularyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_vocabulary_filter_response(
    result: &UpdateVocabularyFilterResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_call_analytics_category_request(
    body: &[u8],
) -> Result<CreateCallAnalyticsCategoryRequest, String> {
    if body.is_empty() {
        return Ok(CreateCallAnalyticsCategoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateCallAnalyticsCategory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_language_model_request(
    body: &[u8],
) -> Result<CreateLanguageModelRequest, String> {
    if body.is_empty() {
        return Ok(CreateLanguageModelRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateLanguageModel request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_medical_vocabulary_request(
    body: &[u8],
) -> Result<CreateMedicalVocabularyRequest, String> {
    if body.is_empty() {
        return Ok(CreateMedicalVocabularyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateMedicalVocabulary request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_vocabulary_request(
    body: &[u8],
) -> Result<CreateVocabularyRequest, String> {
    if body.is_empty() {
        return Ok(CreateVocabularyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateVocabulary request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_vocabulary_filter_request(
    body: &[u8],
) -> Result<CreateVocabularyFilterRequest, String> {
    if body.is_empty() {
        return Ok(CreateVocabularyFilterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateVocabularyFilter request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_call_analytics_category_request(
    body: &[u8],
) -> Result<DeleteCallAnalyticsCategoryRequest, String> {
    if body.is_empty() {
        return Ok(DeleteCallAnalyticsCategoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteCallAnalyticsCategory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_call_analytics_job_request(
    body: &[u8],
) -> Result<DeleteCallAnalyticsJobRequest, String> {
    if body.is_empty() {
        return Ok(DeleteCallAnalyticsJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteCallAnalyticsJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_language_model_request(
    body: &[u8],
) -> Result<DeleteLanguageModelRequest, String> {
    if body.is_empty() {
        return Ok(DeleteLanguageModelRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteLanguageModel request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_medical_scribe_job_request(
    body: &[u8],
) -> Result<DeleteMedicalScribeJobRequest, String> {
    if body.is_empty() {
        return Ok(DeleteMedicalScribeJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteMedicalScribeJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_medical_transcription_job_request(
    body: &[u8],
) -> Result<DeleteMedicalTranscriptionJobRequest, String> {
    if body.is_empty() {
        return Ok(DeleteMedicalTranscriptionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteMedicalTranscriptionJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_medical_vocabulary_request(
    body: &[u8],
) -> Result<DeleteMedicalVocabularyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteMedicalVocabularyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteMedicalVocabulary request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_transcription_job_request(
    body: &[u8],
) -> Result<DeleteTranscriptionJobRequest, String> {
    if body.is_empty() {
        return Ok(DeleteTranscriptionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteTranscriptionJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_vocabulary_request(
    body: &[u8],
) -> Result<DeleteVocabularyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteVocabularyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteVocabulary request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_vocabulary_filter_request(
    body: &[u8],
) -> Result<DeleteVocabularyFilterRequest, String> {
    if body.is_empty() {
        return Ok(DeleteVocabularyFilterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteVocabularyFilter request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_language_model_request(
    body: &[u8],
) -> Result<DescribeLanguageModelRequest, String> {
    if body.is_empty() {
        return Ok(DescribeLanguageModelRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeLanguageModel request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_call_analytics_category_request(
    body: &[u8],
) -> Result<GetCallAnalyticsCategoryRequest, String> {
    if body.is_empty() {
        return Ok(GetCallAnalyticsCategoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCallAnalyticsCategory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_call_analytics_job_request(
    body: &[u8],
) -> Result<GetCallAnalyticsJobRequest, String> {
    if body.is_empty() {
        return Ok(GetCallAnalyticsJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCallAnalyticsJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_medical_scribe_job_request(
    body: &[u8],
) -> Result<GetMedicalScribeJobRequest, String> {
    if body.is_empty() {
        return Ok(GetMedicalScribeJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetMedicalScribeJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_medical_transcription_job_request(
    body: &[u8],
) -> Result<GetMedicalTranscriptionJobRequest, String> {
    if body.is_empty() {
        return Ok(GetMedicalTranscriptionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetMedicalTranscriptionJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_medical_vocabulary_request(
    body: &[u8],
) -> Result<GetMedicalVocabularyRequest, String> {
    if body.is_empty() {
        return Ok(GetMedicalVocabularyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetMedicalVocabulary request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_transcription_job_request(
    body: &[u8],
) -> Result<GetTranscriptionJobRequest, String> {
    if body.is_empty() {
        return Ok(GetTranscriptionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetTranscriptionJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_vocabulary_request(body: &[u8]) -> Result<GetVocabularyRequest, String> {
    if body.is_empty() {
        return Ok(GetVocabularyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetVocabulary request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_vocabulary_filter_request(
    body: &[u8],
) -> Result<GetVocabularyFilterRequest, String> {
    if body.is_empty() {
        return Ok(GetVocabularyFilterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetVocabularyFilter request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_call_analytics_categories_request(
    body: &[u8],
) -> Result<ListCallAnalyticsCategoriesRequest, String> {
    if body.is_empty() {
        return Ok(ListCallAnalyticsCategoriesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListCallAnalyticsCategories request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_call_analytics_jobs_request(
    body: &[u8],
) -> Result<ListCallAnalyticsJobsRequest, String> {
    if body.is_empty() {
        return Ok(ListCallAnalyticsJobsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListCallAnalyticsJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_language_models_request(
    body: &[u8],
) -> Result<ListLanguageModelsRequest, String> {
    if body.is_empty() {
        return Ok(ListLanguageModelsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListLanguageModels request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_medical_scribe_jobs_request(
    body: &[u8],
) -> Result<ListMedicalScribeJobsRequest, String> {
    if body.is_empty() {
        return Ok(ListMedicalScribeJobsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListMedicalScribeJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_medical_transcription_jobs_request(
    body: &[u8],
) -> Result<ListMedicalTranscriptionJobsRequest, String> {
    if body.is_empty() {
        return Ok(ListMedicalTranscriptionJobsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListMedicalTranscriptionJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_medical_vocabularies_request(
    body: &[u8],
) -> Result<ListMedicalVocabulariesRequest, String> {
    if body.is_empty() {
        return Ok(ListMedicalVocabulariesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListMedicalVocabularies request: {e}"))
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
pub fn deserialize_list_transcription_jobs_request(
    body: &[u8],
) -> Result<ListTranscriptionJobsRequest, String> {
    if body.is_empty() {
        return Ok(ListTranscriptionJobsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTranscriptionJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_vocabularies_request(
    body: &[u8],
) -> Result<ListVocabulariesRequest, String> {
    if body.is_empty() {
        return Ok(ListVocabulariesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListVocabularies request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_vocabulary_filters_request(
    body: &[u8],
) -> Result<ListVocabularyFiltersRequest, String> {
    if body.is_empty() {
        return Ok(ListVocabularyFiltersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListVocabularyFilters request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_call_analytics_job_request(
    body: &[u8],
) -> Result<StartCallAnalyticsJobRequest, String> {
    if body.is_empty() {
        return Ok(StartCallAnalyticsJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartCallAnalyticsJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_medical_scribe_job_request(
    body: &[u8],
) -> Result<StartMedicalScribeJobRequest, String> {
    if body.is_empty() {
        return Ok(StartMedicalScribeJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartMedicalScribeJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_medical_transcription_job_request(
    body: &[u8],
) -> Result<StartMedicalTranscriptionJobRequest, String> {
    if body.is_empty() {
        return Ok(StartMedicalTranscriptionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartMedicalTranscriptionJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_transcription_job_request(
    body: &[u8],
) -> Result<StartTranscriptionJobRequest, String> {
    if body.is_empty() {
        return Ok(StartTranscriptionJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartTranscriptionJob request: {e}"))
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
pub fn deserialize_update_call_analytics_category_request(
    body: &[u8],
) -> Result<UpdateCallAnalyticsCategoryRequest, String> {
    if body.is_empty() {
        return Ok(UpdateCallAnalyticsCategoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateCallAnalyticsCategory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_medical_vocabulary_request(
    body: &[u8],
) -> Result<UpdateMedicalVocabularyRequest, String> {
    if body.is_empty() {
        return Ok(UpdateMedicalVocabularyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateMedicalVocabulary request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_vocabulary_request(
    body: &[u8],
) -> Result<UpdateVocabularyRequest, String> {
    if body.is_empty() {
        return Ok(UpdateVocabularyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateVocabulary request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_vocabulary_filter_request(
    body: &[u8],
) -> Result<UpdateVocabularyFilterRequest, String> {
    if body.is_empty() {
        return Ok(UpdateVocabularyFilterRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateVocabularyFilter request: {e}"))
}
