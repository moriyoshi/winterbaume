//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-account

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
pub fn serialize_accept_primary_email_update_response(
    result: &AcceptPrimaryEmailUpdateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_alternate_contact_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_disable_region_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_enable_region_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_get_account_information_response(
    result: &GetAccountInformationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_alternate_contact_response(
    result: &GetAlternateContactResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_contact_information_response(
    result: &GetContactInformationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_gov_cloud_account_information_response(
    result: &GetGovCloudAccountInformationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_primary_email_response(result: &GetPrimaryEmailResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_region_opt_status_response(
    result: &GetRegionOptStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_regions_response(result: &ListRegionsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_put_account_name_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_put_alternate_contact_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_put_contact_information_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_start_primary_email_update_response(
    result: &StartPrimaryEmailUpdateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_accept_primary_email_update_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AcceptPrimaryEmailUpdateRequest, String> {
    let mut input = AcceptPrimaryEmailUpdateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AcceptPrimaryEmailUpdateRequest>(&request.body).map_err(
            |err| format!("failed to deserialize AcceptPrimaryEmailUpdate request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_alternate_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAlternateContactRequest, String> {
    let mut input = DeleteAlternateContactRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteAlternateContactRequest>(&request.body).map_err(
            |err| format!("failed to deserialize DeleteAlternateContact request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disable_region_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisableRegionRequest, String> {
    let mut input = DisableRegionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DisableRegionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DisableRegion request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_enable_region_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<EnableRegionRequest, String> {
    let mut input = EnableRegionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<EnableRegionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize EnableRegion request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_account_information_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAccountInformationRequest, String> {
    let mut input = GetAccountInformationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetAccountInformationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetAccountInformation request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_alternate_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAlternateContactRequest, String> {
    let mut input = GetAlternateContactRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetAlternateContactRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetAlternateContact request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_contact_information_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetContactInformationRequest, String> {
    let mut input = GetContactInformationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetContactInformationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetContactInformation request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_gov_cloud_account_information_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetGovCloudAccountInformationRequest, String> {
    let mut input = GetGovCloudAccountInformationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetGovCloudAccountInformationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize GetGovCloudAccountInformation request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_primary_email_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPrimaryEmailRequest, String> {
    let mut input = GetPrimaryEmailRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetPrimaryEmailRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetPrimaryEmail request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_region_opt_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRegionOptStatusRequest, String> {
    let mut input = GetRegionOptStatusRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetRegionOptStatusRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetRegionOptStatus request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_regions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRegionsRequest, String> {
    let mut input = ListRegionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListRegionsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListRegions request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_account_name_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutAccountNameRequest, String> {
    let mut input = PutAccountNameRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutAccountNameRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutAccountName request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_alternate_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutAlternateContactRequest, String> {
    let mut input = PutAlternateContactRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutAlternateContactRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutAlternateContact request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_contact_information_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutContactInformationRequest, String> {
    let mut input = PutContactInformationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutContactInformationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutContactInformation request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_primary_email_update_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartPrimaryEmailUpdateRequest, String> {
    let mut input = StartPrimaryEmailUpdateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartPrimaryEmailUpdateRequest>(&request.body).map_err(
            |err| format!("failed to deserialize StartPrimaryEmailUpdate request: {err}"),
        )?;
    }
    Ok(input)
}
