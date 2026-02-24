//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-scheduler

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
pub fn serialize_create_schedule_response(result: &CreateScheduleOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_schedule_group_response(
    result: &CreateScheduleGroupOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_schedule_response(result: &DeleteScheduleOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_schedule_group_response(
    result: &DeleteScheduleGroupOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_schedule_response(result: &GetScheduleOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_schedule_group_response(result: &GetScheduleGroupOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_schedule_groups_response(result: &ListScheduleGroupsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_schedules_response(result: &ListSchedulesOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_schedule_response(result: &UpdateScheduleOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_schedule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateScheduleInput, String> {
    let mut input = CreateScheduleInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateScheduleInput>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateSchedule request: {err}"))?;
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
pub fn deserialize_create_schedule_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateScheduleGroupInput, String> {
    let mut input = CreateScheduleGroupInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateScheduleGroupInput>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateScheduleGroup request: {err}"))?;
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
pub fn deserialize_delete_schedule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteScheduleInput, String> {
    let mut input = DeleteScheduleInput::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("clientToken") {
        input.client_token = Some(value.to_string());
    }
    if let Some(value) = query.get("groupName") {
        input.group_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_schedule_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteScheduleGroupInput, String> {
    let mut input = DeleteScheduleGroupInput::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
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
pub fn deserialize_get_schedule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetScheduleInput, String> {
    let mut input = GetScheduleInput::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("groupName") {
        input.group_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_schedule_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetScheduleGroupInput, String> {
    let mut input = GetScheduleGroupInput::default();
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
pub fn deserialize_list_schedule_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListScheduleGroupsInput, String> {
    let mut input = ListScheduleGroupsInput::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NamePrefix") {
        input.name_prefix = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_schedules_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSchedulesInput, String> {
    let mut input = ListSchedulesInput::default();
    if let Some(value) = query.get("ScheduleGroup") {
        input.group_name = Some(value.to_string());
    }
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NamePrefix") {
        input.name_prefix = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("State") {
        input.state = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForResourceInput, String> {
    let mut input = ListTagsForResourceInput::default();
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
pub fn deserialize_tag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TagResourceInput, String> {
    let mut input = TagResourceInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TagResourceInput>(&request.body)
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
) -> Result<UntagResourceInput, String> {
    let mut input = UntagResourceInput::default();
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("TagKeys") {
        input.tag_keys = value
            .split(',')
            .filter(|item| !item.trim().is_empty())
            .map(|item| Ok(item.trim().to_string()))
            .collect::<Result<Vec<_>, String>>()?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_schedule_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateScheduleInput, String> {
    let mut input = UpdateScheduleInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateScheduleInput>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateSchedule request: {err}"))?;
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
