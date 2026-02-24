//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-clouddirectory

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

#[allow(unused_imports)]
use http::header::HeaderName;
use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for restJson protocol.
pub fn serialize_add_facet_to_object_response(result: &AddFacetToObjectResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_apply_schema_response(result: &ApplySchemaResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_attach_object_response(result: &AttachObjectResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_attach_policy_response(result: &AttachPolicyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_attach_to_index_response(result: &AttachToIndexResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_attach_typed_link_response(result: &AttachTypedLinkResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_read_response(result: &BatchReadResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_write_response(result: &BatchWriteResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_directory_response(result: &CreateDirectoryResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_facet_response(result: &CreateFacetResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_index_response(result: &CreateIndexResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_object_response(result: &CreateObjectResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_schema_response(result: &CreateSchemaResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_typed_link_facet_response(
    result: &CreateTypedLinkFacetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_directory_response(result: &DeleteDirectoryResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_facet_response(result: &DeleteFacetResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_object_response(result: &DeleteObjectResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_schema_response(result: &DeleteSchemaResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_typed_link_facet_response(
    result: &DeleteTypedLinkFacetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_detach_from_index_response(result: &DetachFromIndexResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_detach_object_response(result: &DetachObjectResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_detach_policy_response(result: &DetachPolicyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_detach_typed_link_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_disable_directory_response(result: &DisableDirectoryResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_enable_directory_response(result: &EnableDirectoryResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_applied_schema_version_response(
    result: &GetAppliedSchemaVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_directory_response(result: &GetDirectoryResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_facet_response(result: &GetFacetResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_link_attributes_response(result: &GetLinkAttributesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_object_attributes_response(
    result: &GetObjectAttributesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_object_information_response(
    result: &GetObjectInformationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_schema_as_json_response(result: &GetSchemaAsJsonResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_typed_link_facet_information_response(
    result: &GetTypedLinkFacetInformationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_applied_schema_arns_response(
    result: &ListAppliedSchemaArnsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_attached_indices_response(
    result: &ListAttachedIndicesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_development_schema_arns_response(
    result: &ListDevelopmentSchemaArnsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_directories_response(result: &ListDirectoriesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_facet_attributes_response(
    result: &ListFacetAttributesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_facet_names_response(result: &ListFacetNamesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_incoming_typed_links_response(
    result: &ListIncomingTypedLinksResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_index_response(result: &ListIndexResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_managed_schema_arns_response(
    result: &ListManagedSchemaArnsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_object_attributes_response(
    result: &ListObjectAttributesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_object_children_response(
    result: &ListObjectChildrenResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_object_parent_paths_response(
    result: &ListObjectParentPathsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_object_parents_response(result: &ListObjectParentsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_object_policies_response(
    result: &ListObjectPoliciesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_outgoing_typed_links_response(
    result: &ListOutgoingTypedLinksResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_policy_attachments_response(
    result: &ListPolicyAttachmentsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_published_schema_arns_response(
    result: &ListPublishedSchemaArnsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_typed_link_facet_attributes_response(
    result: &ListTypedLinkFacetAttributesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_typed_link_facet_names_response(
    result: &ListTypedLinkFacetNamesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_lookup_policy_response(result: &LookupPolicyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_publish_schema_response(result: &PublishSchemaResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_schema_from_json_response(result: &PutSchemaFromJsonResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_remove_facet_from_object_response(
    result: &RemoveFacetFromObjectResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_facet_response(result: &UpdateFacetResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_link_attributes_response(
    result: &UpdateLinkAttributesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_object_attributes_response(
    result: &UpdateObjectAttributesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_schema_response(result: &UpdateSchemaResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_typed_link_facet_response(
    result: &UpdateTypedLinkFacetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_upgrade_applied_schema_response(
    result: &UpgradeAppliedSchemaResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_upgrade_published_schema_response(
    result: &UpgradePublishedSchemaResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_add_facet_to_object_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AddFacetToObjectRequest, String> {
    let mut input = AddFacetToObjectRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AddFacetToObjectRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AddFacetToObject request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_apply_schema_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ApplySchemaRequest, String> {
    let mut input = ApplySchemaRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ApplySchemaRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ApplySchema request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_attach_object_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AttachObjectRequest, String> {
    let mut input = AttachObjectRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AttachObjectRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AttachObject request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_attach_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AttachPolicyRequest, String> {
    let mut input = AttachPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AttachPolicyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AttachPolicy request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_attach_to_index_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AttachToIndexRequest, String> {
    let mut input = AttachToIndexRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AttachToIndexRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AttachToIndex request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_attach_typed_link_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AttachTypedLinkRequest, String> {
    let mut input = AttachTypedLinkRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AttachTypedLinkRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AttachTypedLink request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_read_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchReadRequest, String> {
    let mut input = BatchReadRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchReadRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchRead request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-consistency-level")
        .and_then(|value| value.to_str().ok())
    {
        input.consistency_level = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_write_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchWriteRequest, String> {
    let mut input = BatchWriteRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchWriteRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchWrite request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_directory_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDirectoryRequest, String> {
    let mut input = CreateDirectoryRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDirectoryRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateDirectory request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.schema_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_facet_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFacetRequest, String> {
    let mut input = CreateFacetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateFacetRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateFacet request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.schema_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_index_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateIndexRequest, String> {
    let mut input = CreateIndexRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateIndexRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateIndex request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_object_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateObjectRequest, String> {
    let mut input = CreateObjectRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateObjectRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateObject request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_schema_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateSchemaRequest, String> {
    let mut input = CreateSchemaRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateSchemaRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateSchema request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_typed_link_facet_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTypedLinkFacetRequest, String> {
    let mut input = CreateTypedLinkFacetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTypedLinkFacetRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateTypedLinkFacet request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.schema_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_directory_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDirectoryRequest, String> {
    let mut input = DeleteDirectoryRequest::default();
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_facet_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFacetRequest, String> {
    let mut input = DeleteFacetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteFacetRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteFacet request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.schema_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_object_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteObjectRequest, String> {
    let mut input = DeleteObjectRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteObjectRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteObject request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_schema_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteSchemaRequest, String> {
    let mut input = DeleteSchemaRequest::default();
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.schema_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_typed_link_facet_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTypedLinkFacetRequest, String> {
    let mut input = DeleteTypedLinkFacetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteTypedLinkFacetRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteTypedLinkFacet request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.schema_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_detach_from_index_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DetachFromIndexRequest, String> {
    let mut input = DetachFromIndexRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DetachFromIndexRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DetachFromIndex request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_detach_object_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DetachObjectRequest, String> {
    let mut input = DetachObjectRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DetachObjectRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DetachObject request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_detach_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DetachPolicyRequest, String> {
    let mut input = DetachPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DetachPolicyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DetachPolicy request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_detach_typed_link_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DetachTypedLinkRequest, String> {
    let mut input = DetachTypedLinkRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DetachTypedLinkRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DetachTypedLink request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disable_directory_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisableDirectoryRequest, String> {
    let mut input = DisableDirectoryRequest::default();
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_enable_directory_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<EnableDirectoryRequest, String> {
    let mut input = EnableDirectoryRequest::default();
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_applied_schema_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAppliedSchemaVersionRequest, String> {
    let mut input = GetAppliedSchemaVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetAppliedSchemaVersionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize GetAppliedSchemaVersion request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_directory_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDirectoryRequest, String> {
    let mut input = GetDirectoryRequest::default();
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_facet_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFacetRequest, String> {
    let mut input = GetFacetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetFacetRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetFacet request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.schema_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_link_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetLinkAttributesRequest, String> {
    let mut input = GetLinkAttributesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetLinkAttributesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetLinkAttributes request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_object_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetObjectAttributesRequest, String> {
    let mut input = GetObjectAttributesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetObjectAttributesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetObjectAttributes request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-consistency-level")
        .and_then(|value| value.to_str().ok())
    {
        input.consistency_level = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_object_information_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetObjectInformationRequest, String> {
    let mut input = GetObjectInformationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetObjectInformationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetObjectInformation request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-consistency-level")
        .and_then(|value| value.to_str().ok())
    {
        input.consistency_level = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_schema_as_json_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSchemaAsJsonRequest, String> {
    let mut input = GetSchemaAsJsonRequest::default();
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.schema_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_typed_link_facet_information_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTypedLinkFacetInformationRequest, String> {
    let mut input = GetTypedLinkFacetInformationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetTypedLinkFacetInformationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize GetTypedLinkFacetInformation request: {err}")
            })?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.schema_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_applied_schema_arns_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAppliedSchemaArnsRequest, String> {
    let mut input = ListAppliedSchemaArnsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAppliedSchemaArnsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListAppliedSchemaArns request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_attached_indices_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAttachedIndicesRequest, String> {
    let mut input = ListAttachedIndicesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAttachedIndicesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListAttachedIndices request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-consistency-level")
        .and_then(|value| value.to_str().ok())
    {
        input.consistency_level = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_development_schema_arns_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDevelopmentSchemaArnsRequest, String> {
    let mut input = ListDevelopmentSchemaArnsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListDevelopmentSchemaArnsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListDevelopmentSchemaArns request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_directories_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDirectoriesRequest, String> {
    let mut input = ListDirectoriesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListDirectoriesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListDirectories request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_facet_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFacetAttributesRequest, String> {
    let mut input = ListFacetAttributesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListFacetAttributesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListFacetAttributes request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.schema_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_facet_names_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFacetNamesRequest, String> {
    let mut input = ListFacetNamesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListFacetNamesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListFacetNames request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.schema_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_incoming_typed_links_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListIncomingTypedLinksRequest, String> {
    let mut input = ListIncomingTypedLinksRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListIncomingTypedLinksRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListIncomingTypedLinks request: {err}"),
        )?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_index_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListIndexRequest, String> {
    let mut input = ListIndexRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListIndexRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListIndex request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-consistency-level")
        .and_then(|value| value.to_str().ok())
    {
        input.consistency_level = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_managed_schema_arns_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListManagedSchemaArnsRequest, String> {
    let mut input = ListManagedSchemaArnsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListManagedSchemaArnsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListManagedSchemaArns request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_object_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListObjectAttributesRequest, String> {
    let mut input = ListObjectAttributesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListObjectAttributesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListObjectAttributes request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-consistency-level")
        .and_then(|value| value.to_str().ok())
    {
        input.consistency_level = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_object_children_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListObjectChildrenRequest, String> {
    let mut input = ListObjectChildrenRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListObjectChildrenRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListObjectChildren request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-consistency-level")
        .and_then(|value| value.to_str().ok())
    {
        input.consistency_level = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_object_parent_paths_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListObjectParentPathsRequest, String> {
    let mut input = ListObjectParentPathsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListObjectParentPathsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListObjectParentPaths request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_object_parents_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListObjectParentsRequest, String> {
    let mut input = ListObjectParentsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListObjectParentsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListObjectParents request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-consistency-level")
        .and_then(|value| value.to_str().ok())
    {
        input.consistency_level = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_object_policies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListObjectPoliciesRequest, String> {
    let mut input = ListObjectPoliciesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListObjectPoliciesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListObjectPolicies request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-consistency-level")
        .and_then(|value| value.to_str().ok())
    {
        input.consistency_level = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_outgoing_typed_links_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListOutgoingTypedLinksRequest, String> {
    let mut input = ListOutgoingTypedLinksRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListOutgoingTypedLinksRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListOutgoingTypedLinks request: {err}"),
        )?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_policy_attachments_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPolicyAttachmentsRequest, String> {
    let mut input = ListPolicyAttachmentsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListPolicyAttachmentsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListPolicyAttachments request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-consistency-level")
        .and_then(|value| value.to_str().ok())
    {
        input.consistency_level = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_published_schema_arns_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPublishedSchemaArnsRequest, String> {
    let mut input = ListPublishedSchemaArnsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListPublishedSchemaArnsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListPublishedSchemaArns request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForResourceRequest, String> {
    let mut input = ListTagsForResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListTagsForResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListTagsForResource request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_typed_link_facet_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTypedLinkFacetAttributesRequest, String> {
    let mut input = ListTypedLinkFacetAttributesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListTypedLinkFacetAttributesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListTypedLinkFacetAttributes request: {err}")
            })?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.schema_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_typed_link_facet_names_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTypedLinkFacetNamesRequest, String> {
    let mut input = ListTypedLinkFacetNamesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListTypedLinkFacetNamesRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListTypedLinkFacetNames request: {err}"),
        )?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.schema_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_lookup_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<LookupPolicyRequest, String> {
    let mut input = LookupPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<LookupPolicyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize LookupPolicy request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_publish_schema_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PublishSchemaRequest, String> {
    let mut input = PublishSchemaRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PublishSchemaRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PublishSchema request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.development_schema_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_schema_from_json_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutSchemaFromJsonRequest, String> {
    let mut input = PutSchemaFromJsonRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutSchemaFromJsonRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutSchemaFromJson request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.schema_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_remove_facet_from_object_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RemoveFacetFromObjectRequest, String> {
    let mut input = RemoveFacetFromObjectRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RemoveFacetFromObjectRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize RemoveFacetFromObject request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_tag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TagResourceRequest, String> {
    let mut input = TagResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TagResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize TagResource request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_untag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UntagResourceRequest, String> {
    let mut input = UntagResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UntagResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UntagResource request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_facet_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFacetRequest, String> {
    let mut input = UpdateFacetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFacetRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateFacet request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.schema_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_link_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateLinkAttributesRequest, String> {
    let mut input = UpdateLinkAttributesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateLinkAttributesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateLinkAttributes request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_object_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateObjectAttributesRequest, String> {
    let mut input = UpdateObjectAttributesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateObjectAttributesRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateObjectAttributes request: {err}"),
        )?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.directory_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_schema_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateSchemaRequest, String> {
    let mut input = UpdateSchemaRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateSchemaRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateSchema request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.schema_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_typed_link_facet_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTypedLinkFacetRequest, String> {
    let mut input = UpdateTypedLinkFacetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateTypedLinkFacetRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateTypedLinkFacet request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("x-amz-data-partition")
        .and_then(|value| value.to_str().ok())
    {
        input.schema_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_upgrade_applied_schema_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpgradeAppliedSchemaRequest, String> {
    let mut input = UpgradeAppliedSchemaRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpgradeAppliedSchemaRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpgradeAppliedSchema request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_upgrade_published_schema_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpgradePublishedSchemaRequest, String> {
    let mut input = UpgradePublishedSchemaRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpgradePublishedSchemaRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpgradePublishedSchema request: {err}"),
        )?;
    }
    Ok(input)
}
