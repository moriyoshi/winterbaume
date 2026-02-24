//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-rekognition

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_associate_faces_response(result: &AssociateFacesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_compare_faces_response(result: &CompareFacesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_copy_project_version_response(
    result: &CopyProjectVersionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_collection_response(result: &CreateCollectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_dataset_response(result: &CreateDatasetResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_face_liveness_session_response(
    result: &CreateFaceLivenessSessionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_project_response(result: &CreateProjectResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_project_version_response(
    result: &CreateProjectVersionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_stream_processor_response(
    result: &CreateStreamProcessorResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_user_response(result: &CreateUserResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_collection_response(result: &DeleteCollectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_dataset_response(result: &DeleteDatasetResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_faces_response(result: &DeleteFacesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_project_response(result: &DeleteProjectResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_project_policy_response(
    result: &DeleteProjectPolicyResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_project_version_response(
    result: &DeleteProjectVersionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_stream_processor_response(
    result: &DeleteStreamProcessorResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_user_response(result: &DeleteUserResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_collection_response(result: &DescribeCollectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_dataset_response(result: &DescribeDatasetResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_project_versions_response(
    result: &DescribeProjectVersionsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_projects_response(result: &DescribeProjectsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_stream_processor_response(
    result: &DescribeStreamProcessorResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_detect_custom_labels_response(
    result: &DetectCustomLabelsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_detect_faces_response(result: &DetectFacesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_detect_labels_response(result: &DetectLabelsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_detect_moderation_labels_response(
    result: &DetectModerationLabelsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_detect_protective_equipment_response(
    result: &DetectProtectiveEquipmentResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_detect_text_response(result: &DetectTextResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_faces_response(result: &DisassociateFacesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_distribute_dataset_entries_response(
    result: &DistributeDatasetEntriesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_celebrity_info_response(result: &GetCelebrityInfoResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_celebrity_recognition_response(
    result: &GetCelebrityRecognitionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_content_moderation_response(
    result: &GetContentModerationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_face_detection_response(result: &GetFaceDetectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_face_liveness_session_results_response(
    result: &GetFaceLivenessSessionResultsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_face_search_response(result: &GetFaceSearchResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_label_detection_response(result: &GetLabelDetectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_media_analysis_job_response(
    result: &GetMediaAnalysisJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_person_tracking_response(result: &GetPersonTrackingResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_segment_detection_response(
    result: &GetSegmentDetectionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_text_detection_response(result: &GetTextDetectionResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_index_faces_response(result: &IndexFacesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_collections_response(result: &ListCollectionsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_dataset_entries_response(
    result: &ListDatasetEntriesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_dataset_labels_response(result: &ListDatasetLabelsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_faces_response(result: &ListFacesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_media_analysis_jobs_response(
    result: &ListMediaAnalysisJobsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_project_policies_response(
    result: &ListProjectPoliciesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_stream_processors_response(
    result: &ListStreamProcessorsResponse,
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
pub fn serialize_list_users_response(result: &ListUsersResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_project_policy_response(result: &PutProjectPolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_recognize_celebrities_response(
    result: &RecognizeCelebritiesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_search_faces_response(result: &SearchFacesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_search_faces_by_image_response(
    result: &SearchFacesByImageResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_search_users_response(result: &SearchUsersResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_search_users_by_image_response(
    result: &SearchUsersByImageResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_celebrity_recognition_response(
    result: &StartCelebrityRecognitionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_content_moderation_response(
    result: &StartContentModerationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_face_detection_response(
    result: &StartFaceDetectionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_face_search_response(result: &StartFaceSearchResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_label_detection_response(
    result: &StartLabelDetectionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_media_analysis_job_response(
    result: &StartMediaAnalysisJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_person_tracking_response(
    result: &StartPersonTrackingResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_project_version_response(
    result: &StartProjectVersionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_segment_detection_response(
    result: &StartSegmentDetectionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_stream_processor_response(
    result: &StartStreamProcessorResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_text_detection_response(
    result: &StartTextDetectionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_project_version_response(
    result: &StopProjectVersionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_stream_processor_response(
    result: &StopStreamProcessorResponse,
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
pub fn serialize_update_dataset_entries_response(
    result: &UpdateDatasetEntriesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_stream_processor_response(
    result: &UpdateStreamProcessorResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_faces_request(body: &[u8]) -> Result<AssociateFacesRequest, String> {
    if body.is_empty() {
        return Ok(AssociateFacesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateFaces request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_compare_faces_request(body: &[u8]) -> Result<CompareFacesRequest, String> {
    if body.is_empty() {
        return Ok(CompareFacesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CompareFaces request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_copy_project_version_request(
    body: &[u8],
) -> Result<CopyProjectVersionRequest, String> {
    if body.is_empty() {
        return Ok(CopyProjectVersionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CopyProjectVersion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_collection_request(
    body: &[u8],
) -> Result<CreateCollectionRequest, String> {
    if body.is_empty() {
        return Ok(CreateCollectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateCollection request: {e}"))
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
pub fn deserialize_create_face_liveness_session_request(
    body: &[u8],
) -> Result<CreateFaceLivenessSessionRequest, String> {
    if body.is_empty() {
        return Ok(CreateFaceLivenessSessionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateFaceLivenessSession request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_project_request(body: &[u8]) -> Result<CreateProjectRequest, String> {
    if body.is_empty() {
        return Ok(CreateProjectRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateProject request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_project_version_request(
    body: &[u8],
) -> Result<CreateProjectVersionRequest, String> {
    if body.is_empty() {
        return Ok(CreateProjectVersionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateProjectVersion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_stream_processor_request(
    body: &[u8],
) -> Result<CreateStreamProcessorRequest, String> {
    if body.is_empty() {
        return Ok(CreateStreamProcessorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateStreamProcessor request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_user_request(body: &[u8]) -> Result<CreateUserRequest, String> {
    if body.is_empty() {
        return Ok(CreateUserRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateUser request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_collection_request(
    body: &[u8],
) -> Result<DeleteCollectionRequest, String> {
    if body.is_empty() {
        return Ok(DeleteCollectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteCollection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_dataset_request(body: &[u8]) -> Result<DeleteDatasetRequest, String> {
    if body.is_empty() {
        return Ok(DeleteDatasetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteDataset request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_faces_request(body: &[u8]) -> Result<DeleteFacesRequest, String> {
    if body.is_empty() {
        return Ok(DeleteFacesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteFaces request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_project_request(body: &[u8]) -> Result<DeleteProjectRequest, String> {
    if body.is_empty() {
        return Ok(DeleteProjectRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteProject request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_project_policy_request(
    body: &[u8],
) -> Result<DeleteProjectPolicyRequest, String> {
    if body.is_empty() {
        return Ok(DeleteProjectPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteProjectPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_project_version_request(
    body: &[u8],
) -> Result<DeleteProjectVersionRequest, String> {
    if body.is_empty() {
        return Ok(DeleteProjectVersionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteProjectVersion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_stream_processor_request(
    body: &[u8],
) -> Result<DeleteStreamProcessorRequest, String> {
    if body.is_empty() {
        return Ok(DeleteStreamProcessorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteStreamProcessor request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_user_request(body: &[u8]) -> Result<DeleteUserRequest, String> {
    if body.is_empty() {
        return Ok(DeleteUserRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteUser request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_collection_request(
    body: &[u8],
) -> Result<DescribeCollectionRequest, String> {
    if body.is_empty() {
        return Ok(DescribeCollectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeCollection request: {e}"))
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
pub fn deserialize_describe_project_versions_request(
    body: &[u8],
) -> Result<DescribeProjectVersionsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeProjectVersionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeProjectVersions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_projects_request(
    body: &[u8],
) -> Result<DescribeProjectsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeProjectsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeProjects request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_stream_processor_request(
    body: &[u8],
) -> Result<DescribeStreamProcessorRequest, String> {
    if body.is_empty() {
        return Ok(DescribeStreamProcessorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeStreamProcessor request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_detect_custom_labels_request(
    body: &[u8],
) -> Result<DetectCustomLabelsRequest, String> {
    if body.is_empty() {
        return Ok(DetectCustomLabelsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DetectCustomLabels request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_detect_faces_request(body: &[u8]) -> Result<DetectFacesRequest, String> {
    if body.is_empty() {
        return Ok(DetectFacesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DetectFaces request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_detect_labels_request(body: &[u8]) -> Result<DetectLabelsRequest, String> {
    if body.is_empty() {
        return Ok(DetectLabelsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DetectLabels request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_detect_moderation_labels_request(
    body: &[u8],
) -> Result<DetectModerationLabelsRequest, String> {
    if body.is_empty() {
        return Ok(DetectModerationLabelsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DetectModerationLabels request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_detect_protective_equipment_request(
    body: &[u8],
) -> Result<DetectProtectiveEquipmentRequest, String> {
    if body.is_empty() {
        return Ok(DetectProtectiveEquipmentRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DetectProtectiveEquipment request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_detect_text_request(body: &[u8]) -> Result<DetectTextRequest, String> {
    if body.is_empty() {
        return Ok(DetectTextRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DetectText request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_faces_request(
    body: &[u8],
) -> Result<DisassociateFacesRequest, String> {
    if body.is_empty() {
        return Ok(DisassociateFacesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisassociateFaces request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_distribute_dataset_entries_request(
    body: &[u8],
) -> Result<DistributeDatasetEntriesRequest, String> {
    if body.is_empty() {
        return Ok(DistributeDatasetEntriesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DistributeDatasetEntries request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_celebrity_info_request(
    body: &[u8],
) -> Result<GetCelebrityInfoRequest, String> {
    if body.is_empty() {
        return Ok(GetCelebrityInfoRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCelebrityInfo request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_celebrity_recognition_request(
    body: &[u8],
) -> Result<GetCelebrityRecognitionRequest, String> {
    if body.is_empty() {
        return Ok(GetCelebrityRecognitionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCelebrityRecognition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_content_moderation_request(
    body: &[u8],
) -> Result<GetContentModerationRequest, String> {
    if body.is_empty() {
        return Ok(GetContentModerationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetContentModeration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_face_detection_request(
    body: &[u8],
) -> Result<GetFaceDetectionRequest, String> {
    if body.is_empty() {
        return Ok(GetFaceDetectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetFaceDetection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_face_liveness_session_results_request(
    body: &[u8],
) -> Result<GetFaceLivenessSessionResultsRequest, String> {
    if body.is_empty() {
        return Ok(GetFaceLivenessSessionResultsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetFaceLivenessSessionResults request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_face_search_request(body: &[u8]) -> Result<GetFaceSearchRequest, String> {
    if body.is_empty() {
        return Ok(GetFaceSearchRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetFaceSearch request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_label_detection_request(
    body: &[u8],
) -> Result<GetLabelDetectionRequest, String> {
    if body.is_empty() {
        return Ok(GetLabelDetectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetLabelDetection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_media_analysis_job_request(
    body: &[u8],
) -> Result<GetMediaAnalysisJobRequest, String> {
    if body.is_empty() {
        return Ok(GetMediaAnalysisJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetMediaAnalysisJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_person_tracking_request(
    body: &[u8],
) -> Result<GetPersonTrackingRequest, String> {
    if body.is_empty() {
        return Ok(GetPersonTrackingRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetPersonTracking request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_segment_detection_request(
    body: &[u8],
) -> Result<GetSegmentDetectionRequest, String> {
    if body.is_empty() {
        return Ok(GetSegmentDetectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetSegmentDetection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_text_detection_request(
    body: &[u8],
) -> Result<GetTextDetectionRequest, String> {
    if body.is_empty() {
        return Ok(GetTextDetectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetTextDetection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_index_faces_request(body: &[u8]) -> Result<IndexFacesRequest, String> {
    if body.is_empty() {
        return Ok(IndexFacesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize IndexFaces request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_collections_request(body: &[u8]) -> Result<ListCollectionsRequest, String> {
    if body.is_empty() {
        return Ok(ListCollectionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListCollections request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_dataset_entries_request(
    body: &[u8],
) -> Result<ListDatasetEntriesRequest, String> {
    if body.is_empty() {
        return Ok(ListDatasetEntriesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDatasetEntries request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_dataset_labels_request(
    body: &[u8],
) -> Result<ListDatasetLabelsRequest, String> {
    if body.is_empty() {
        return Ok(ListDatasetLabelsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDatasetLabels request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_faces_request(body: &[u8]) -> Result<ListFacesRequest, String> {
    if body.is_empty() {
        return Ok(ListFacesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListFaces request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_media_analysis_jobs_request(
    body: &[u8],
) -> Result<ListMediaAnalysisJobsRequest, String> {
    if body.is_empty() {
        return Ok(ListMediaAnalysisJobsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListMediaAnalysisJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_project_policies_request(
    body: &[u8],
) -> Result<ListProjectPoliciesRequest, String> {
    if body.is_empty() {
        return Ok(ListProjectPoliciesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListProjectPolicies request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_stream_processors_request(
    body: &[u8],
) -> Result<ListStreamProcessorsRequest, String> {
    if body.is_empty() {
        return Ok(ListStreamProcessorsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListStreamProcessors request: {e}"))
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
pub fn deserialize_list_users_request(body: &[u8]) -> Result<ListUsersRequest, String> {
    if body.is_empty() {
        return Ok(ListUsersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListUsers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_project_policy_request(
    body: &[u8],
) -> Result<PutProjectPolicyRequest, String> {
    if body.is_empty() {
        return Ok(PutProjectPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutProjectPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_recognize_celebrities_request(
    body: &[u8],
) -> Result<RecognizeCelebritiesRequest, String> {
    if body.is_empty() {
        return Ok(RecognizeCelebritiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RecognizeCelebrities request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_search_faces_request(body: &[u8]) -> Result<SearchFacesRequest, String> {
    if body.is_empty() {
        return Ok(SearchFacesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SearchFaces request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_search_faces_by_image_request(
    body: &[u8],
) -> Result<SearchFacesByImageRequest, String> {
    if body.is_empty() {
        return Ok(SearchFacesByImageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SearchFacesByImage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_search_users_request(body: &[u8]) -> Result<SearchUsersRequest, String> {
    if body.is_empty() {
        return Ok(SearchUsersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SearchUsers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_search_users_by_image_request(
    body: &[u8],
) -> Result<SearchUsersByImageRequest, String> {
    if body.is_empty() {
        return Ok(SearchUsersByImageRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SearchUsersByImage request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_celebrity_recognition_request(
    body: &[u8],
) -> Result<StartCelebrityRecognitionRequest, String> {
    if body.is_empty() {
        return Ok(StartCelebrityRecognitionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartCelebrityRecognition request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_content_moderation_request(
    body: &[u8],
) -> Result<StartContentModerationRequest, String> {
    if body.is_empty() {
        return Ok(StartContentModerationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartContentModeration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_face_detection_request(
    body: &[u8],
) -> Result<StartFaceDetectionRequest, String> {
    if body.is_empty() {
        return Ok(StartFaceDetectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartFaceDetection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_face_search_request(
    body: &[u8],
) -> Result<StartFaceSearchRequest, String> {
    if body.is_empty() {
        return Ok(StartFaceSearchRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartFaceSearch request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_label_detection_request(
    body: &[u8],
) -> Result<StartLabelDetectionRequest, String> {
    if body.is_empty() {
        return Ok(StartLabelDetectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartLabelDetection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_media_analysis_job_request(
    body: &[u8],
) -> Result<StartMediaAnalysisJobRequest, String> {
    if body.is_empty() {
        return Ok(StartMediaAnalysisJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartMediaAnalysisJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_person_tracking_request(
    body: &[u8],
) -> Result<StartPersonTrackingRequest, String> {
    if body.is_empty() {
        return Ok(StartPersonTrackingRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartPersonTracking request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_project_version_request(
    body: &[u8],
) -> Result<StartProjectVersionRequest, String> {
    if body.is_empty() {
        return Ok(StartProjectVersionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartProjectVersion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_segment_detection_request(
    body: &[u8],
) -> Result<StartSegmentDetectionRequest, String> {
    if body.is_empty() {
        return Ok(StartSegmentDetectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartSegmentDetection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_stream_processor_request(
    body: &[u8],
) -> Result<StartStreamProcessorRequest, String> {
    if body.is_empty() {
        return Ok(StartStreamProcessorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartStreamProcessor request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_text_detection_request(
    body: &[u8],
) -> Result<StartTextDetectionRequest, String> {
    if body.is_empty() {
        return Ok(StartTextDetectionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartTextDetection request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_project_version_request(
    body: &[u8],
) -> Result<StopProjectVersionRequest, String> {
    if body.is_empty() {
        return Ok(StopProjectVersionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopProjectVersion request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_stream_processor_request(
    body: &[u8],
) -> Result<StopStreamProcessorRequest, String> {
    if body.is_empty() {
        return Ok(StopStreamProcessorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopStreamProcessor request: {e}"))
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
pub fn deserialize_update_dataset_entries_request(
    body: &[u8],
) -> Result<UpdateDatasetEntriesRequest, String> {
    if body.is_empty() {
        return Ok(UpdateDatasetEntriesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDatasetEntries request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_stream_processor_request(
    body: &[u8],
) -> Result<UpdateStreamProcessorRequest, String> {
    if body.is_empty() {
        return Ok(UpdateStreamProcessorRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateStreamProcessor request: {e}"))
}
