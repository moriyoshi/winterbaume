//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-amp

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
pub fn serialize_create_alert_manager_definition_response(
    result: &CreateAlertManagerDefinitionResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_anomaly_detector_response(
    result: &CreateAnomalyDetectorResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_logging_configuration_response(
    result: &CreateLoggingConfigurationResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_query_logging_configuration_response(
    result: &CreateQueryLoggingConfigurationResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_rule_groups_namespace_response(
    result: &CreateRuleGroupsNamespaceResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_scraper_response(result: &CreateScraperResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_workspace_response(result: &CreateWorkspaceResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_alert_manager_definition_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_anomaly_detector_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_logging_configuration_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_query_logging_configuration_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_resource_policy_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_rule_groups_namespace_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_scraper_response(result: &DeleteScraperResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_scraper_logging_configuration_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_workspace_response() -> MockResponse {
    MockResponse::rest_json(202, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_alert_manager_definition_response(
    result: &DescribeAlertManagerDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_anomaly_detector_response(
    result: &DescribeAnomalyDetectorResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_logging_configuration_response(
    result: &DescribeLoggingConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_query_logging_configuration_response(
    result: &DescribeQueryLoggingConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_resource_policy_response(
    result: &DescribeResourcePolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_rule_groups_namespace_response(
    result: &DescribeRuleGroupsNamespaceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_scraper_response(result: &DescribeScraperResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_scraper_logging_configuration_response(
    result: &DescribeScraperLoggingConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_workspace_response(result: &DescribeWorkspaceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_workspace_configuration_response(
    result: &DescribeWorkspaceConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_default_scraper_configuration_response(
    result: &GetDefaultScraperConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_anomaly_detectors_response(
    result: &ListAnomalyDetectorsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_rule_groups_namespaces_response(
    result: &ListRuleGroupsNamespacesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_scrapers_response(result: &ListScrapersResponse) -> MockResponse {
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
pub fn serialize_list_workspaces_response(result: &ListWorkspacesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_alert_manager_definition_response(
    result: &PutAlertManagerDefinitionResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_anomaly_detector_response(
    result: &PutAnomalyDetectorResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_resource_policy_response(result: &PutResourcePolicyResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_rule_groups_namespace_response(
    result: &PutRuleGroupsNamespaceResponse,
) -> MockResponse {
    let status = 202_u16;
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
pub fn serialize_update_logging_configuration_response(
    result: &UpdateLoggingConfigurationResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_query_logging_configuration_response(
    result: &UpdateQueryLoggingConfigurationResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_scraper_response(result: &UpdateScraperResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_scraper_logging_configuration_response(
    result: &UpdateScraperLoggingConfigurationResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_workspace_alias_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_workspace_configuration_response(
    result: &UpdateWorkspaceConfigurationResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_alert_manager_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAlertManagerDefinitionRequest, String> {
    let mut input = CreateAlertManagerDefinitionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAlertManagerDefinitionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateAlertManagerDefinition request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_anomaly_detector_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAnomalyDetectorRequest, String> {
    let mut input = CreateAnomalyDetectorRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAnomalyDetectorRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateAnomalyDetector request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_logging_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateLoggingConfigurationRequest, String> {
    let mut input = CreateLoggingConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateLoggingConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateLoggingConfiguration request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_query_logging_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateQueryLoggingConfigurationRequest, String> {
    let mut input = CreateQueryLoggingConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateQueryLoggingConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateQueryLoggingConfiguration request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_rule_groups_namespace_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateRuleGroupsNamespaceRequest, String> {
    let mut input = CreateRuleGroupsNamespaceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateRuleGroupsNamespaceRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateRuleGroupsNamespace request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_scraper_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateScraperRequest, String> {
    let mut input = CreateScraperRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateScraperRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateScraper request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_workspace_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateWorkspaceRequest, String> {
    let mut input = CreateWorkspaceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateWorkspaceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateWorkspace request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_alert_manager_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAlertManagerDefinitionRequest, String> {
    let mut input = DeleteAlertManagerDefinitionRequest::default();
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("clientToken") {
        input.client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_anomaly_detector_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAnomalyDetectorRequest, String> {
    let mut input = DeleteAnomalyDetectorRequest::default();
    for (name, value) in labels {
        match *name {
            "anomalyDetectorId" => {
                input.anomaly_detector_id = value.to_string();
            }
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("clientToken") {
        input.client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_logging_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteLoggingConfigurationRequest, String> {
    let mut input = DeleteLoggingConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("clientToken") {
        input.client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_query_logging_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteQueryLoggingConfigurationRequest, String> {
    let mut input = DeleteQueryLoggingConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("clientToken") {
        input.client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteResourcePolicyRequest, String> {
    let mut input = DeleteResourcePolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("clientToken") {
        input.client_token = Some(value.to_string());
    }
    if let Some(value) = query.get("revisionId") {
        input.revision_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_rule_groups_namespace_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteRuleGroupsNamespaceRequest, String> {
    let mut input = DeleteRuleGroupsNamespaceRequest::default();
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("clientToken") {
        input.client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_scraper_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteScraperRequest, String> {
    let mut input = DeleteScraperRequest::default();
    for (name, value) in labels {
        match *name {
            "scraperId" => {
                input.scraper_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("clientToken") {
        input.client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_scraper_logging_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteScraperLoggingConfigurationRequest, String> {
    let mut input = DeleteScraperLoggingConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "scraperId" => {
                input.scraper_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("clientToken") {
        input.client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_workspace_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteWorkspaceRequest, String> {
    let mut input = DeleteWorkspaceRequest::default();
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("clientToken") {
        input.client_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_alert_manager_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAlertManagerDefinitionRequest, String> {
    let mut input = DescribeAlertManagerDefinitionRequest::default();
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_anomaly_detector_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeAnomalyDetectorRequest, String> {
    let mut input = DescribeAnomalyDetectorRequest::default();
    for (name, value) in labels {
        match *name {
            "anomalyDetectorId" => {
                input.anomaly_detector_id = value.to_string();
            }
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_logging_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeLoggingConfigurationRequest, String> {
    let mut input = DescribeLoggingConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_query_logging_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeQueryLoggingConfigurationRequest, String> {
    let mut input = DescribeQueryLoggingConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeResourcePolicyRequest, String> {
    let mut input = DescribeResourcePolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_rule_groups_namespace_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeRuleGroupsNamespaceRequest, String> {
    let mut input = DescribeRuleGroupsNamespaceRequest::default();
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_scraper_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeScraperRequest, String> {
    let mut input = DescribeScraperRequest::default();
    for (name, value) in labels {
        match *name {
            "scraperId" => {
                input.scraper_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_scraper_logging_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeScraperLoggingConfigurationRequest, String> {
    let mut input = DescribeScraperLoggingConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "scraperId" => {
                input.scraper_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_workspace_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeWorkspaceRequest, String> {
    let mut input = DescribeWorkspaceRequest::default();
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_workspace_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeWorkspaceConfigurationRequest, String> {
    let mut input = DescribeWorkspaceConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_default_scraper_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDefaultScraperConfigurationRequest, String> {
    let input = GetDefaultScraperConfigurationRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_anomaly_detectors_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAnomalyDetectorsRequest, String> {
    let mut input = ListAnomalyDetectorsRequest::default();
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("alias") {
        input.alias = Some(value.to_string());
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
pub fn deserialize_list_rule_groups_namespaces_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRuleGroupsNamespacesRequest, String> {
    let mut input = ListRuleGroupsNamespacesRequest::default();
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
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
    if let Some(value) = query.get("name") {
        input.name = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_scrapers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListScrapersRequest, String> {
    let mut input = ListScrapersRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListScrapersRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListScrapers request: {err}"))?;
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
pub fn deserialize_list_workspaces_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListWorkspacesRequest, String> {
    let mut input = ListWorkspacesRequest::default();
    if let Some(value) = query.get("alias") {
        input.alias = Some(value.to_string());
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
pub fn deserialize_put_alert_manager_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutAlertManagerDefinitionRequest, String> {
    let mut input = PutAlertManagerDefinitionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutAlertManagerDefinitionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize PutAlertManagerDefinition request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_anomaly_detector_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutAnomalyDetectorRequest, String> {
    let mut input = PutAnomalyDetectorRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutAnomalyDetectorRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutAnomalyDetector request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "anomalyDetectorId" => {
                input.anomaly_detector_id = value.to_string();
            }
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutResourcePolicyRequest, String> {
    let mut input = PutResourcePolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutResourcePolicyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutResourcePolicy request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_rule_groups_namespace_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutRuleGroupsNamespaceRequest, String> {
    let mut input = PutRuleGroupsNamespaceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutRuleGroupsNamespaceRequest>(&request.body).map_err(
            |err| format!("failed to deserialize PutRuleGroupsNamespace request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "name" => {
                input.name = value.to_string();
            }
            "workspaceId" => {
                input.workspace_id = value.to_string();
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
pub fn deserialize_update_logging_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateLoggingConfigurationRequest, String> {
    let mut input = UpdateLoggingConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateLoggingConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateLoggingConfiguration request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_query_logging_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateQueryLoggingConfigurationRequest, String> {
    let mut input = UpdateQueryLoggingConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateQueryLoggingConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateQueryLoggingConfiguration request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_scraper_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateScraperRequest, String> {
    let mut input = UpdateScraperRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateScraperRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateScraper request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "scraperId" => {
                input.scraper_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_scraper_logging_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateScraperLoggingConfigurationRequest, String> {
    let mut input = UpdateScraperLoggingConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateScraperLoggingConfigurationRequest>(&request.body)
            .map_err(|err| {
            format!("failed to deserialize UpdateScraperLoggingConfiguration request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "scraperId" => {
                input.scraper_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_workspace_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateWorkspaceAliasRequest, String> {
    let mut input = UpdateWorkspaceAliasRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateWorkspaceAliasRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateWorkspaceAlias request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_workspace_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateWorkspaceConfigurationRequest, String> {
    let mut input = UpdateWorkspaceConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateWorkspaceConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateWorkspaceConfiguration request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "workspaceId" => {
                input.workspace_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
