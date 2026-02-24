//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-servicecatalogappregistry

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
pub fn serialize_associate_attribute_group_response(
    result: &AssociateAttributeGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_resource_response(result: &AssociateResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_application_response(result: &CreateApplicationResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_attribute_group_response(
    result: &CreateAttributeGroupResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_application_response(result: &DeleteApplicationResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_attribute_group_response(
    result: &DeleteAttributeGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_attribute_group_response(
    result: &DisassociateAttributeGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_resource_response(
    result: &DisassociateResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_application_response(result: &GetApplicationResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_associated_resource_response(
    result: &GetAssociatedResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_attribute_group_response(result: &GetAttributeGroupResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_configuration_response(result: &GetConfigurationResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_applications_response(result: &ListApplicationsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_associated_attribute_groups_response(
    result: &ListAssociatedAttributeGroupsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_associated_resources_response(
    result: &ListAssociatedResourcesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_attribute_groups_response(
    result: &ListAttributeGroupsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_attribute_groups_for_application_response(
    result: &ListAttributeGroupsForApplicationResponse,
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

/// Serialize void response for restJson protocol.
pub fn serialize_put_configuration_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_sync_resource_response(result: &SyncResourceResponse) -> MockResponse {
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
pub fn serialize_update_application_response(result: &UpdateApplicationResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_attribute_group_response(
    result: &UpdateAttributeGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_attribute_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateAttributeGroupRequest, String> {
    let mut input = AssociateAttributeGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "application" => {
                input.application = value.to_string();
            }
            "attributeGroup" => {
                input.attribute_group = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateResourceRequest, String> {
    let mut input = AssociateResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AssociateResource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "application" => {
                input.application = value.to_string();
            }
            "resource" => {
                input.resource = value.to_string();
            }
            "resourceType" => {
                input.resource_type = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_application_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateApplicationRequest, String> {
    let mut input = CreateApplicationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateApplicationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateApplication request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_attribute_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAttributeGroupRequest, String> {
    let mut input = CreateAttributeGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAttributeGroupRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateAttributeGroup request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_application_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteApplicationRequest, String> {
    let mut input = DeleteApplicationRequest::default();
    for (name, value) in labels {
        match *name {
            "application" => {
                input.application = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_attribute_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAttributeGroupRequest, String> {
    let mut input = DeleteAttributeGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "attributeGroup" => {
                input.attribute_group = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_attribute_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateAttributeGroupRequest, String> {
    let mut input = DisassociateAttributeGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "application" => {
                input.application = value.to_string();
            }
            "attributeGroup" => {
                input.attribute_group = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateResourceRequest, String> {
    let mut input = DisassociateResourceRequest::default();
    for (name, value) in labels {
        match *name {
            "application" => {
                input.application = value.to_string();
            }
            "resource" => {
                input.resource = value.to_string();
            }
            "resourceType" => {
                input.resource_type = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_application_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetApplicationRequest, String> {
    let mut input = GetApplicationRequest::default();
    for (name, value) in labels {
        match *name {
            "application" => {
                input.application = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_associated_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAssociatedResourceRequest, String> {
    let mut input = GetAssociatedResourceRequest::default();
    for (name, value) in labels {
        match *name {
            "application" => {
                input.application = value.to_string();
            }
            "resource" => {
                input.resource = value.to_string();
            }
            "resourceType" => {
                input.resource_type = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("resourceTagStatus") {
        input.resource_tag_status = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_attribute_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAttributeGroupRequest, String> {
    let mut input = GetAttributeGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "attributeGroup" => {
                input.attribute_group = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_applications_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListApplicationsRequest, String> {
    let mut input = ListApplicationsRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_associated_attribute_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAssociatedAttributeGroupsRequest, String> {
    let mut input = ListAssociatedAttributeGroupsRequest::default();
    for (name, value) in labels {
        match *name {
            "application" => {
                input.application = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_associated_resources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAssociatedResourcesRequest, String> {
    let mut input = ListAssociatedResourcesRequest::default();
    for (name, value) in labels {
        match *name {
            "application" => {
                input.application = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_attribute_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAttributeGroupsRequest, String> {
    let mut input = ListAttributeGroupsRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_attribute_groups_for_application_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAttributeGroupsForApplicationRequest, String> {
    let mut input = ListAttributeGroupsForApplicationRequest::default();
    for (name, value) in labels {
        match *name {
            "application" => {
                input.application = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
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
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutConfigurationRequest, String> {
    let mut input = PutConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutConfigurationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutConfiguration request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_sync_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SyncResourceRequest, String> {
    let mut input = SyncResourceRequest::default();
    for (name, value) in labels {
        match *name {
            "resource" => {
                input.resource = value.to_string();
            }
            "resourceType" => {
                input.resource_type = value.to_string();
            }
            _ => {}
        }
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
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
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
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("tagKeys") {
        input.tag_keys = value
            .split(',')
            .filter(|item| !item.trim().is_empty())
            .map(|item| Ok(item.trim().to_string()))
            .collect::<Result<Vec<_>, String>>()?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_application_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateApplicationRequest, String> {
    let mut input = UpdateApplicationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateApplicationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateApplication request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "application" => {
                input.application = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_attribute_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAttributeGroupRequest, String> {
    let mut input = UpdateAttributeGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAttributeGroupRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateAttributeGroup request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "attributeGroup" => {
                input.attribute_group = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
