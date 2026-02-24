//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-appflow

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
pub fn serialize_cancel_flow_executions_response(
    result: &CancelFlowExecutionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_connector_profile_response(
    result: &CreateConnectorProfileResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_flow_response(result: &CreateFlowResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_connector_profile_response(
    result: &DeleteConnectorProfileResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_flow_response(result: &DeleteFlowResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_connector_response(result: &DescribeConnectorResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_connector_entity_response(
    result: &DescribeConnectorEntityResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_connector_profiles_response(
    result: &DescribeConnectorProfilesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_connectors_response(result: &DescribeConnectorsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_flow_response(result: &DescribeFlowResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_flow_execution_records_response(
    result: &DescribeFlowExecutionRecordsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_connector_entities_response(
    result: &ListConnectorEntitiesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_connectors_response(result: &ListConnectorsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_flows_response(result: &ListFlowsResponse) -> MockResponse {
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
pub fn serialize_register_connector_response(result: &RegisterConnectorResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_reset_connector_metadata_cache_response(
    result: &ResetConnectorMetadataCacheResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_flow_response(result: &StartFlowResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_flow_response(result: &StopFlowResponse) -> MockResponse {
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
pub fn serialize_unregister_connector_response(
    result: &UnregisterConnectorResponse,
) -> MockResponse {
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
pub fn serialize_update_connector_profile_response(
    result: &UpdateConnectorProfileResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_connector_registration_response(
    result: &UpdateConnectorRegistrationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_flow_response(result: &UpdateFlowResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_flow_executions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelFlowExecutionsRequest, String> {
    let mut input = CancelFlowExecutionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CancelFlowExecutionsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CancelFlowExecutions request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_connector_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateConnectorProfileRequest, String> {
    let mut input = CreateConnectorProfileRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateConnectorProfileRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateConnectorProfile request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFlowRequest, String> {
    let mut input = CreateFlowRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateFlowRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateFlow request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_connector_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteConnectorProfileRequest, String> {
    let mut input = DeleteConnectorProfileRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteConnectorProfileRequest>(&request.body).map_err(
            |err| format!("failed to deserialize DeleteConnectorProfile request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFlowRequest, String> {
    let mut input = DeleteFlowRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteFlowRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteFlow request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_connector_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeConnectorRequest, String> {
    let mut input = DescribeConnectorRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeConnectorRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DescribeConnector request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_connector_entity_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeConnectorEntityRequest, String> {
    let mut input = DescribeConnectorEntityRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeConnectorEntityRequest>(&request.body).map_err(
            |err| format!("failed to deserialize DescribeConnectorEntity request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_connector_profiles_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeConnectorProfilesRequest, String> {
    let mut input = DescribeConnectorProfilesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeConnectorProfilesRequest>(&request.body).map_err(
            |err| format!("failed to deserialize DescribeConnectorProfiles request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_connectors_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeConnectorsRequest, String> {
    let mut input = DescribeConnectorsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeConnectorsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DescribeConnectors request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeFlowRequest, String> {
    let mut input = DescribeFlowRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeFlowRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DescribeFlow request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_flow_execution_records_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeFlowExecutionRecordsRequest, String> {
    let mut input = DescribeFlowExecutionRecordsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DescribeFlowExecutionRecordsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DescribeFlowExecutionRecords request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_connector_entities_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListConnectorEntitiesRequest, String> {
    let mut input = ListConnectorEntitiesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListConnectorEntitiesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListConnectorEntities request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_connectors_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListConnectorsRequest, String> {
    let mut input = ListConnectorsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListConnectorsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListConnectors request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_flows_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFlowsRequest, String> {
    let mut input = ListFlowsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListFlowsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListFlows request: {err}"))?;
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
pub fn deserialize_register_connector_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RegisterConnectorRequest, String> {
    let mut input = RegisterConnectorRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RegisterConnectorRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize RegisterConnector request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_reset_connector_metadata_cache_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ResetConnectorMetadataCacheRequest, String> {
    let mut input = ResetConnectorMetadataCacheRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ResetConnectorMetadataCacheRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ResetConnectorMetadataCache request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartFlowRequest, String> {
    let mut input = StartFlowRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartFlowRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartFlow request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopFlowRequest, String> {
    let mut input = StopFlowRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StopFlowRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StopFlow request: {err}"))?;
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
pub fn deserialize_unregister_connector_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UnregisterConnectorRequest, String> {
    let mut input = UnregisterConnectorRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UnregisterConnectorRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UnregisterConnector request: {err}"))?;
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
pub fn deserialize_update_connector_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateConnectorProfileRequest, String> {
    let mut input = UpdateConnectorProfileRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateConnectorProfileRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateConnectorProfile request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_connector_registration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateConnectorRegistrationRequest, String> {
    let mut input = UpdateConnectorRegistrationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateConnectorRegistrationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateConnectorRegistration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFlowRequest, String> {
    let mut input = UpdateFlowRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFlowRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateFlow request: {err}"))?;
    }
    Ok(input)
}
