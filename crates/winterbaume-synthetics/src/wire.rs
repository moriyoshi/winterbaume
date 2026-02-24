//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-synthetics

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
pub fn serialize_associate_resource_response(result: &AssociateResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_canary_response(result: &CreateCanaryResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_group_response(result: &CreateGroupResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_canary_response(result: &DeleteCanaryResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_group_response(result: &DeleteGroupResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_canaries_response(result: &DescribeCanariesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_canaries_last_run_response(
    result: &DescribeCanariesLastRunResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_runtime_versions_response(
    result: &DescribeRuntimeVersionsResponse,
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
pub fn serialize_get_canary_response(result: &GetCanaryResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_canary_runs_response(result: &GetCanaryRunsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_group_response(result: &GetGroupResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_associated_groups_response(
    result: &ListAssociatedGroupsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_group_resources_response(
    result: &ListGroupResourcesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_groups_response(result: &ListGroupsResponse) -> MockResponse {
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
pub fn serialize_start_canary_response(result: &StartCanaryResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_canary_dry_run_response(result: &StartCanaryDryRunResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_canary_response(result: &StopCanaryResponse) -> MockResponse {
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
pub fn serialize_update_canary_response(result: &UpdateCanaryResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
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
            "GroupIdentifier" => {
                input.group_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_canary_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCanaryRequest, String> {
    let mut input = CreateCanaryRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateCanaryRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateCanary request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateGroupRequest, String> {
    let mut input = CreateGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateGroupRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateGroup request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_canary_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCanaryRequest, String> {
    let mut input = DeleteCanaryRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("deleteLambda") {
        input.delete_lambda = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteGroupRequest, String> {
    let mut input = DeleteGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "GroupIdentifier" => {
                input.group_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_canaries_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeCanariesRequest, String> {
    let mut input = DescribeCanariesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeCanariesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DescribeCanaries request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_canaries_last_run_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeCanariesLastRunRequest, String> {
    let mut input = DescribeCanariesLastRunRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeCanariesLastRunRequest>(&request.body).map_err(
            |err| format!("failed to deserialize DescribeCanariesLastRun request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_runtime_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeRuntimeVersionsRequest, String> {
    let mut input = DescribeRuntimeVersionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeRuntimeVersionsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize DescribeRuntimeVersions request: {err}"),
        )?;
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
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DisassociateResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DisassociateResource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "GroupIdentifier" => {
                input.group_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_canary_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCanaryRequest, String> {
    let mut input = GetCanaryRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("dryRunId") {
        input.dry_run_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_canary_runs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCanaryRunsRequest, String> {
    let mut input = GetCanaryRunsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetCanaryRunsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetCanaryRuns request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetGroupRequest, String> {
    let mut input = GetGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "GroupIdentifier" => {
                input.group_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_associated_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAssociatedGroupsRequest, String> {
    let mut input = ListAssociatedGroupsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAssociatedGroupsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListAssociatedGroups request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_group_resources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListGroupResourcesRequest, String> {
    let mut input = ListGroupResourcesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListGroupResourcesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListGroupResources request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "GroupIdentifier" => {
                input.group_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListGroupsRequest, String> {
    let mut input = ListGroupsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListGroupsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListGroups request: {err}"))?;
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
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_canary_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartCanaryRequest, String> {
    let mut input = StartCanaryRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_canary_dry_run_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartCanaryDryRunRequest, String> {
    let mut input = StartCanaryDryRunRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartCanaryDryRunRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartCanaryDryRun request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_canary_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopCanaryRequest, String> {
    let mut input = StopCanaryRequest::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
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
            "ResourceArn" => {
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
            "ResourceArn" => {
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
pub fn deserialize_update_canary_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCanaryRequest, String> {
    let mut input = UpdateCanaryRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateCanaryRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateCanary request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
