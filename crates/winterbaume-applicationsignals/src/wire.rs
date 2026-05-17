//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-application-signals

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
pub fn serialize_batch_get_service_level_objective_budget_report_response(
    result: &BatchGetServiceLevelObjectiveBudgetReportOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_update_exclusion_windows_response(
    result: &BatchUpdateExclusionWindowsOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_service_level_objective_response(
    result: &CreateServiceLevelObjectiveOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_grouping_configuration_response(
    result: &DeleteGroupingConfigurationOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_service_level_objective_response(
    result: &DeleteServiceLevelObjectiveOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_service_response(result: &GetServiceOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_service_level_objective_response(
    result: &GetServiceLevelObjectiveOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_audit_findings_response(result: &ListAuditFindingsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_entity_events_response(result: &ListEntityEventsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_grouping_attribute_definitions_response(
    result: &ListGroupingAttributeDefinitionsOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_service_dependencies_response(
    result: &ListServiceDependenciesOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_service_dependents_response(
    result: &ListServiceDependentsOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_service_level_objective_exclusion_windows_response(
    result: &ListServiceLevelObjectiveExclusionWindowsOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_service_level_objectives_response(
    result: &ListServiceLevelObjectivesOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_service_operations_response(
    result: &ListServiceOperationsOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_service_states_response(result: &ListServiceStatesOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_services_response(result: &ListServicesOutput) -> MockResponse {
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
pub fn serialize_put_grouping_configuration_response(
    result: &PutGroupingConfigurationOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_discovery_response(result: &StartDiscoveryOutput) -> MockResponse {
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
pub fn serialize_update_service_level_objective_response(
    result: &UpdateServiceLevelObjectiveOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_get_service_level_objective_budget_report_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchGetServiceLevelObjectiveBudgetReportInput, String> {
    let mut input = BatchGetServiceLevelObjectiveBudgetReportInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchGetServiceLevelObjectiveBudgetReportInput>(
            &request.body,
        )
        .map_err(|err| {
            format!(
                "failed to deserialize BatchGetServiceLevelObjectiveBudgetReport request: {err}"
            )
        })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_update_exclusion_windows_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchUpdateExclusionWindowsInput, String> {
    let mut input = BatchUpdateExclusionWindowsInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchUpdateExclusionWindowsInput>(&request.body).map_err(
            |err| format!("failed to deserialize BatchUpdateExclusionWindows request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_service_level_objective_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateServiceLevelObjectiveInput, String> {
    let mut input = CreateServiceLevelObjectiveInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateServiceLevelObjectiveInput>(&request.body).map_err(
            |err| format!("failed to deserialize CreateServiceLevelObjective request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_service_level_objective_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteServiceLevelObjectiveInput, String> {
    let mut input = DeleteServiceLevelObjectiveInput::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_service_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetServiceInput, String> {
    let mut input = GetServiceInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetServiceInput>(&request.body)
            .map_err(|err| format!("failed to deserialize GetService request: {err}"))?;
    }
    if let Some(value) = query.get("EndTime") {
        input.end_time = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
    }
    if let Some(value) = query.get("StartTime") {
        input.start_time = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_service_level_objective_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetServiceLevelObjectiveInput, String> {
    let mut input = GetServiceLevelObjectiveInput::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_audit_findings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAuditFindingsInput, String> {
    let mut input = ListAuditFindingsInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAuditFindingsInput>(&request.body)
            .map_err(|err| format!("failed to deserialize ListAuditFindings request: {err}"))?;
    }
    if let Some(value) = query.get("EndTime") {
        input.end_time = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
    }
    if let Some(value) = query.get("StartTime") {
        input.start_time = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_entity_events_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListEntityEventsInput, String> {
    let mut input = ListEntityEventsInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListEntityEventsInput>(&request.body)
            .map_err(|err| format!("failed to deserialize ListEntityEvents request: {err}"))?;
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_grouping_attribute_definitions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListGroupingAttributeDefinitionsInput, String> {
    let mut input = ListGroupingAttributeDefinitionsInput::default();
    if let Some(value) = query.get("AwsAccountId") {
        input.aws_account_id = Some(value.to_string());
    }
    if let Some(value) = query.get("IncludeLinkedAccounts") {
        input.include_linked_accounts = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_service_dependencies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListServiceDependenciesInput, String> {
    let mut input = ListServiceDependenciesInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListServiceDependenciesInput>(&request.body).map_err(
            |err| format!("failed to deserialize ListServiceDependencies request: {err}"),
        )?;
    }
    if let Some(value) = query.get("EndTime") {
        input.end_time = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("StartTime") {
        input.start_time = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_service_dependents_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListServiceDependentsInput, String> {
    let mut input = ListServiceDependentsInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListServiceDependentsInput>(&request.body)
            .map_err(|err| format!("failed to deserialize ListServiceDependents request: {err}"))?;
    }
    if let Some(value) = query.get("EndTime") {
        input.end_time = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("StartTime") {
        input.start_time = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_service_level_objective_exclusion_windows_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListServiceLevelObjectiveExclusionWindowsInput, String> {
    let mut input = ListServiceLevelObjectiveExclusionWindowsInput::default();
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_service_level_objectives_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListServiceLevelObjectivesInput, String> {
    let mut input = ListServiceLevelObjectivesInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListServiceLevelObjectivesInput>(&request.body).map_err(
            |err| format!("failed to deserialize ListServiceLevelObjectives request: {err}"),
        )?;
    }
    if let Some(value) = query.get("IncludeLinkedAccounts") {
        input.include_linked_accounts = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("OperationName") {
        input.operation_name = Some(value.to_string());
    }
    if let Some(value) = query.get("SloOwnerAwsAccountId") {
        input.slo_owner_aws_account_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_service_operations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListServiceOperationsInput, String> {
    let mut input = ListServiceOperationsInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListServiceOperationsInput>(&request.body)
            .map_err(|err| format!("failed to deserialize ListServiceOperations request: {err}"))?;
    }
    if let Some(value) = query.get("EndTime") {
        input.end_time = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("StartTime") {
        input.start_time = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_service_states_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListServiceStatesInput, String> {
    let mut input = ListServiceStatesInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListServiceStatesInput>(&request.body)
            .map_err(|err| format!("failed to deserialize ListServiceStates request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_services_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListServicesInput, String> {
    let mut input = ListServicesInput::default();
    if let Some(value) = query.get("AwsAccountId") {
        input.aws_account_id = Some(value.to_string());
    }
    if let Some(value) = query.get("EndTime") {
        input.end_time = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
    }
    if let Some(value) = query.get("IncludeLinkedAccounts") {
        input.include_linked_accounts = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("StartTime") {
        input.start_time = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
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
    if let Some(value) = query.get("ResourceArn") {
        input.resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_grouping_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutGroupingConfigurationInput, String> {
    let mut input = PutGroupingConfigurationInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutGroupingConfigurationInput>(&request.body).map_err(
            |err| format!("failed to deserialize PutGroupingConfiguration request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_discovery_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartDiscoveryInput, String> {
    let input = StartDiscoveryInput {};
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
pub fn deserialize_update_service_level_objective_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateServiceLevelObjectiveInput, String> {
    let mut input = UpdateServiceLevelObjectiveInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateServiceLevelObjectiveInput>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateServiceLevelObjective request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "Id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
