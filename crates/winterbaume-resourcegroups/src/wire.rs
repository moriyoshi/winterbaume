//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-resourcegroups

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

/// Serialize void response for restJson protocol.
pub fn serialize_cancel_tag_sync_task_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_create_group_response(result: &CreateGroupOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_group_response(result: &DeleteGroupOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_account_settings_response(result: &GetAccountSettingsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_group_response(result: &GetGroupOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_group_configuration_response(
    result: &GetGroupConfigurationOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_group_query_response(result: &GetGroupQueryOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_tag_sync_task_response(result: &GetTagSyncTaskOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_tags_response(result: &GetTagsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_group_resources_response(result: &GroupResourcesOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_group_resources_response(result: &ListGroupResourcesOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_grouping_statuses_response(
    result: &ListGroupingStatusesOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_groups_response(result: &ListGroupsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tag_sync_tasks_response(result: &ListTagSyncTasksOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_group_configuration_response(
    result: &PutGroupConfigurationOutput,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_search_resources_response(result: &SearchResourcesOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_tag_sync_task_response(result: &StartTagSyncTaskOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_response(result: &TagOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_ungroup_resources_response(result: &UngroupResourcesOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_untag_response(result: &UntagOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_account_settings_response(
    result: &UpdateAccountSettingsOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_group_response(result: &UpdateGroupOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_group_query_response(result: &UpdateGroupQueryOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_tag_sync_task_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelTagSyncTaskInput, String> {
    let mut input = CancelTagSyncTaskInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CancelTagSyncTaskInput>(&request.body)
            .map_err(|err| format!("failed to deserialize CancelTagSyncTask request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateGroupInput, String> {
    let mut input = CreateGroupInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateGroupInput>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateGroup request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteGroupInput, String> {
    let mut input = DeleteGroupInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteGroupInput>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteGroup request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetGroupInput, String> {
    let mut input = GetGroupInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetGroupInput>(&request.body)
            .map_err(|err| format!("failed to deserialize GetGroup request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_group_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetGroupConfigurationInput, String> {
    let mut input = GetGroupConfigurationInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetGroupConfigurationInput>(&request.body)
            .map_err(|err| format!("failed to deserialize GetGroupConfiguration request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_group_query_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetGroupQueryInput, String> {
    let mut input = GetGroupQueryInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetGroupQueryInput>(&request.body)
            .map_err(|err| format!("failed to deserialize GetGroupQuery request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_tag_sync_task_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTagSyncTaskInput, String> {
    let mut input = GetTagSyncTaskInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetTagSyncTaskInput>(&request.body)
            .map_err(|err| format!("failed to deserialize GetTagSyncTask request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_tags_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTagsInput, String> {
    let mut input = GetTagsInput::default();
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_group_resources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GroupResourcesInput, String> {
    let mut input = GroupResourcesInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GroupResourcesInput>(&request.body)
            .map_err(|err| format!("failed to deserialize GroupResources request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_group_resources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListGroupResourcesInput, String> {
    let mut input = ListGroupResourcesInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListGroupResourcesInput>(&request.body)
            .map_err(|err| format!("failed to deserialize ListGroupResources request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_grouping_statuses_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListGroupingStatusesInput, String> {
    let mut input = ListGroupingStatusesInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListGroupingStatusesInput>(&request.body)
            .map_err(|err| format!("failed to deserialize ListGroupingStatuses request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListGroupsInput, String> {
    let mut input = ListGroupsInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListGroupsInput>(&request.body)
            .map_err(|err| format!("failed to deserialize ListGroups request: {err}"))?;
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
pub fn deserialize_list_tag_sync_tasks_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagSyncTasksInput, String> {
    let mut input = ListTagSyncTasksInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListTagSyncTasksInput>(&request.body)
            .map_err(|err| format!("failed to deserialize ListTagSyncTasks request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_group_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutGroupConfigurationInput, String> {
    let mut input = PutGroupConfigurationInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutGroupConfigurationInput>(&request.body)
            .map_err(|err| format!("failed to deserialize PutGroupConfiguration request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_search_resources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SearchResourcesInput, String> {
    let mut input = SearchResourcesInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SearchResourcesInput>(&request.body)
            .map_err(|err| format!("failed to deserialize SearchResources request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_tag_sync_task_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartTagSyncTaskInput, String> {
    let mut input = StartTagSyncTaskInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartTagSyncTaskInput>(&request.body)
            .map_err(|err| format!("failed to deserialize StartTagSyncTask request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_tag_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TagInput, String> {
    let mut input = TagInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TagInput>(&request.body)
            .map_err(|err| format!("failed to deserialize Tag request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_ungroup_resources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UngroupResourcesInput, String> {
    let mut input = UngroupResourcesInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UngroupResourcesInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UngroupResources request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_untag_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UntagInput, String> {
    let mut input = UntagInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UntagInput>(&request.body)
            .map_err(|err| format!("failed to deserialize Untag request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_account_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAccountSettingsInput, String> {
    let mut input = UpdateAccountSettingsInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAccountSettingsInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateAccountSettings request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateGroupInput, String> {
    let mut input = UpdateGroupInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateGroupInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateGroup request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_group_query_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateGroupQueryInput, String> {
    let mut input = UpdateGroupQueryInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateGroupQueryInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateGroupQuery request: {err}"))?;
    }
    Ok(input)
}
