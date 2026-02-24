//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-arczonalshift

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
pub fn serialize_cancel_practice_run_response(result: &CancelPracticeRunResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_cancel_zonal_shift_response(result: &ZonalShift) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_practice_run_configuration_response(
    result: &CreatePracticeRunConfigurationResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_practice_run_configuration_response(
    result: &DeletePracticeRunConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_autoshift_observer_notification_status_response(
    result: &GetAutoshiftObserverNotificationStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_managed_resource_response(
    result: &GetManagedResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_autoshifts_response(result: &ListAutoshiftsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_managed_resources_response(
    result: &ListManagedResourcesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_zonal_shifts_response(result: &ListZonalShiftsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_practice_run_response(result: &StartPracticeRunResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_zonal_shift_response(result: &ZonalShift) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_autoshift_observer_notification_status_response(
    result: &UpdateAutoshiftObserverNotificationStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_practice_run_configuration_response(
    result: &UpdatePracticeRunConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_zonal_autoshift_configuration_response(
    result: &UpdateZonalAutoshiftConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_zonal_shift_response(result: &ZonalShift) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_practice_run_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelPracticeRunRequest, String> {
    let mut input = CancelPracticeRunRequest::default();
    for (name, value) in labels {
        match *name {
            "zonalShiftId" => {
                input.zonal_shift_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_zonal_shift_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelZonalShiftRequest, String> {
    let mut input = CancelZonalShiftRequest::default();
    for (name, value) in labels {
        match *name {
            "zonalShiftId" => {
                input.zonal_shift_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_practice_run_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreatePracticeRunConfigurationRequest, String> {
    let mut input = CreatePracticeRunConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreatePracticeRunConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreatePracticeRunConfiguration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_practice_run_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePracticeRunConfigurationRequest, String> {
    let mut input = DeletePracticeRunConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceIdentifier" => {
                input.resource_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_autoshift_observer_notification_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAutoshiftObserverNotificationStatusRequest, String> {
    let input = GetAutoshiftObserverNotificationStatusRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_managed_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetManagedResourceRequest, String> {
    let mut input = GetManagedResourceRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceIdentifier" => {
                input.resource_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_autoshifts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAutoshiftsRequest, String> {
    let mut input = ListAutoshiftsRequest::default();
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
    if let Some(value) = query.get("status") {
        input.status = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_managed_resources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListManagedResourcesRequest, String> {
    let mut input = ListManagedResourcesRequest::default();
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
pub fn deserialize_list_zonal_shifts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListZonalShiftsRequest, String> {
    let mut input = ListZonalShiftsRequest::default();
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
    if let Some(value) = query.get("resourceIdentifier") {
        input.resource_identifier = Some(value.to_string());
    }
    if let Some(value) = query.get("status") {
        input.status = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_practice_run_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartPracticeRunRequest, String> {
    let mut input = StartPracticeRunRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartPracticeRunRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartPracticeRun request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_zonal_shift_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartZonalShiftRequest, String> {
    let mut input = StartZonalShiftRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartZonalShiftRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartZonalShift request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_autoshift_observer_notification_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAutoshiftObserverNotificationStatusRequest, String> {
    let mut input = UpdateAutoshiftObserverNotificationStatusRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAutoshiftObserverNotificationStatusRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!(
                "failed to deserialize UpdateAutoshiftObserverNotificationStatus request: {err}"
            )
        })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_practice_run_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdatePracticeRunConfigurationRequest, String> {
    let mut input = UpdatePracticeRunConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdatePracticeRunConfigurationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdatePracticeRunConfiguration request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "resourceIdentifier" => {
                input.resource_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_zonal_autoshift_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateZonalAutoshiftConfigurationRequest, String> {
    let mut input = UpdateZonalAutoshiftConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateZonalAutoshiftConfigurationRequest>(&request.body)
            .map_err(|err| {
            format!("failed to deserialize UpdateZonalAutoshiftConfiguration request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "resourceIdentifier" => {
                input.resource_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_zonal_shift_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateZonalShiftRequest, String> {
    let mut input = UpdateZonalShiftRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateZonalShiftRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateZonalShift request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "zonalShiftId" => {
                input.zonal_shift_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
