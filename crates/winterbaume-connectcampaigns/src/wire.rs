//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-connectcampaigns

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
pub fn serialize_create_campaign_response(result: &CreateCampaignResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_campaign_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_connect_instance_config_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_instance_onboarding_job_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_campaign_response(result: &DescribeCampaignResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_campaign_state_response(result: &GetCampaignStateResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_campaign_state_batch_response(
    result: &GetCampaignStateBatchResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_connect_instance_config_response(
    result: &GetConnectInstanceConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_instance_onboarding_job_status_response(
    result: &GetInstanceOnboardingJobStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_campaigns_response(result: &ListCampaignsResponse) -> MockResponse {
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
pub fn serialize_pause_campaign_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_put_dial_request_batch_response(
    result: &PutDialRequestBatchResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_resume_campaign_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_start_campaign_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_start_instance_onboarding_job_response(
    result: &StartInstanceOnboardingJobResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_stop_campaign_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_tag_resource_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_campaign_dialer_config_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_campaign_name_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_update_campaign_outbound_call_config_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_campaign_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCampaignRequest, String> {
    let mut input = CreateCampaignRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateCampaignRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateCampaign request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_campaign_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCampaignRequest, String> {
    let mut input = DeleteCampaignRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_connect_instance_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteConnectInstanceConfigRequest, String> {
    let mut input = DeleteConnectInstanceConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "connectInstanceId" => {
                input.connect_instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_instance_onboarding_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteInstanceOnboardingJobRequest, String> {
    let mut input = DeleteInstanceOnboardingJobRequest::default();
    for (name, value) in labels {
        match *name {
            "connectInstanceId" => {
                input.connect_instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_campaign_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeCampaignRequest, String> {
    let mut input = DescribeCampaignRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_campaign_state_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCampaignStateRequest, String> {
    let mut input = GetCampaignStateRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_campaign_state_batch_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCampaignStateBatchRequest, String> {
    let mut input = GetCampaignStateBatchRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetCampaignStateBatchRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetCampaignStateBatch request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_connect_instance_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetConnectInstanceConfigRequest, String> {
    let mut input = GetConnectInstanceConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "connectInstanceId" => {
                input.connect_instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_instance_onboarding_job_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetInstanceOnboardingJobStatusRequest, String> {
    let mut input = GetInstanceOnboardingJobStatusRequest::default();
    for (name, value) in labels {
        match *name {
            "connectInstanceId" => {
                input.connect_instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_campaigns_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCampaignsRequest, String> {
    let mut input = ListCampaignsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListCampaignsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListCampaigns request: {err}"))?;
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
            "arn" => {
                input.arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_pause_campaign_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PauseCampaignRequest, String> {
    let mut input = PauseCampaignRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_dial_request_batch_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutDialRequestBatchRequest, String> {
    let mut input = PutDialRequestBatchRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutDialRequestBatchRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutDialRequestBatch request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_resume_campaign_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ResumeCampaignRequest, String> {
    let mut input = ResumeCampaignRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_campaign_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartCampaignRequest, String> {
    let mut input = StartCampaignRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_instance_onboarding_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartInstanceOnboardingJobRequest, String> {
    let mut input = StartInstanceOnboardingJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartInstanceOnboardingJobRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize StartInstanceOnboardingJob request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "connectInstanceId" => {
                input.connect_instance_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_campaign_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopCampaignRequest, String> {
    let mut input = StopCampaignRequest::default();
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
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
            "arn" => {
                input.arn = value.to_string();
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
            "arn" => {
                input.arn = value.to_string();
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
pub fn deserialize_update_campaign_dialer_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCampaignDialerConfigRequest, String> {
    let mut input = UpdateCampaignDialerConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateCampaignDialerConfigRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateCampaignDialerConfig request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_campaign_name_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCampaignNameRequest, String> {
    let mut input = UpdateCampaignNameRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateCampaignNameRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateCampaignName request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_campaign_outbound_call_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCampaignOutboundCallConfigRequest, String> {
    let mut input = UpdateCampaignOutboundCallConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateCampaignOutboundCallConfigRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateCampaignOutboundCallConfig request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
