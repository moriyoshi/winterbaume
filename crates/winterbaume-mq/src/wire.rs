//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-mq

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
pub fn serialize_create_broker_response(result: &CreateBrokerResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_configuration_response(
    result: &CreateConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_create_tags_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_create_user_response(result: &CreateUserResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_broker_response(result: &DeleteBrokerResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_configuration_response(
    result: &DeleteConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_tags_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_user_response(result: &DeleteUserResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_broker_response(result: &DescribeBrokerResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_broker_engine_types_response(
    result: &DescribeBrokerEngineTypesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_broker_instance_options_response(
    result: &DescribeBrokerInstanceOptionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_configuration_response(
    result: &DescribeConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_configuration_revision_response(
    result: &DescribeConfigurationRevisionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_user_response(result: &DescribeUserResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_brokers_response(result: &ListBrokersResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_configuration_revisions_response(
    result: &ListConfigurationRevisionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_configurations_response(result: &ListConfigurationsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_response(result: &ListTagsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_users_response(result: &ListUsersResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_promote_response(result: &PromoteResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_reboot_broker_response(result: &RebootBrokerResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_broker_response(result: &UpdateBrokerResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_configuration_response(
    result: &UpdateConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_user_response(result: &UpdateUserResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_broker_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBrokerRequest, String> {
    let mut input = CreateBrokerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateBrokerRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateBroker request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateConfigurationRequest, String> {
    let mut input = CreateConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateConfigurationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateConfiguration request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_tags_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTagsRequest, String> {
    let mut input = CreateTagsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTagsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateTags request: {err}"))?;
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
pub fn deserialize_create_user_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateUserRequest, String> {
    let mut input = CreateUserRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateUserRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateUser request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "BrokerId" => {
                input.broker_id = value.to_string();
            }
            "Username" => {
                input.username = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_broker_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBrokerRequest, String> {
    let mut input = DeleteBrokerRequest::default();
    for (name, value) in labels {
        match *name {
            "BrokerId" => {
                input.broker_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteConfigurationRequest, String> {
    let mut input = DeleteConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "ConfigurationId" => {
                input.configuration_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_tags_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTagsRequest, String> {
    let mut input = DeleteTagsRequest::default();
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
pub fn deserialize_delete_user_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteUserRequest, String> {
    let mut input = DeleteUserRequest::default();
    for (name, value) in labels {
        match *name {
            "BrokerId" => {
                input.broker_id = value.to_string();
            }
            "Username" => {
                input.username = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_broker_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeBrokerRequest, String> {
    let mut input = DescribeBrokerRequest::default();
    for (name, value) in labels {
        match *name {
            "BrokerId" => {
                input.broker_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_broker_engine_types_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeBrokerEngineTypesRequest, String> {
    let mut input = DescribeBrokerEngineTypesRequest::default();
    if let Some(value) = query.get("engineType") {
        input.engine_type = Some(value.to_string());
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
pub fn deserialize_describe_broker_instance_options_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeBrokerInstanceOptionsRequest, String> {
    let mut input = DescribeBrokerInstanceOptionsRequest::default();
    if let Some(value) = query.get("engineType") {
        input.engine_type = Some(value.to_string());
    }
    if let Some(value) = query.get("hostInstanceType") {
        input.host_instance_type = Some(value.to_string());
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
    if let Some(value) = query.get("storageType") {
        input.storage_type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeConfigurationRequest, String> {
    let mut input = DescribeConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "ConfigurationId" => {
                input.configuration_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_configuration_revision_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeConfigurationRevisionRequest, String> {
    let mut input = DescribeConfigurationRevisionRequest::default();
    for (name, value) in labels {
        match *name {
            "ConfigurationId" => {
                input.configuration_id = value.to_string();
            }
            "ConfigurationRevision" => {
                input.configuration_revision = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_user_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeUserRequest, String> {
    let mut input = DescribeUserRequest::default();
    for (name, value) in labels {
        match *name {
            "BrokerId" => {
                input.broker_id = value.to_string();
            }
            "Username" => {
                input.username = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_brokers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBrokersRequest, String> {
    let mut input = ListBrokersRequest::default();
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
pub fn deserialize_list_configuration_revisions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListConfigurationRevisionsRequest, String> {
    let mut input = ListConfigurationRevisionsRequest::default();
    for (name, value) in labels {
        match *name {
            "ConfigurationId" => {
                input.configuration_id = value.to_string();
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
pub fn deserialize_list_configurations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListConfigurationsRequest, String> {
    let mut input = ListConfigurationsRequest::default();
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
pub fn deserialize_list_tags_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagsRequest, String> {
    let mut input = ListTagsRequest::default();
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
pub fn deserialize_list_users_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListUsersRequest, String> {
    let mut input = ListUsersRequest::default();
    for (name, value) in labels {
        match *name {
            "BrokerId" => {
                input.broker_id = value.to_string();
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
pub fn deserialize_promote_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PromoteRequest, String> {
    let mut input = PromoteRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PromoteRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize Promote request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "BrokerId" => {
                input.broker_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_reboot_broker_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RebootBrokerRequest, String> {
    let mut input = RebootBrokerRequest::default();
    for (name, value) in labels {
        match *name {
            "BrokerId" => {
                input.broker_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_broker_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBrokerRequest, String> {
    let mut input = UpdateBrokerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBrokerRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateBroker request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "BrokerId" => {
                input.broker_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateConfigurationRequest, String> {
    let mut input = UpdateConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateConfigurationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateConfiguration request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ConfigurationId" => {
                input.configuration_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_user_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateUserRequest, String> {
    let mut input = UpdateUserRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateUserRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateUser request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "BrokerId" => {
                input.broker_id = value.to_string();
            }
            "Username" => {
                input.username = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
