//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-taxsettings

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
pub fn serialize_batch_delete_tax_registration_response(
    result: &BatchDeleteTaxRegistrationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_get_tax_exemptions_response(
    result: &BatchGetTaxExemptionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_batch_put_tax_registration_response(
    result: &BatchPutTaxRegistrationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_supplemental_tax_registration_response(
    result: &DeleteSupplementalTaxRegistrationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_tax_registration_response(
    result: &DeleteTaxRegistrationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_tax_exemption_types_response(
    result: &GetTaxExemptionTypesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_tax_inheritance_response(result: &GetTaxInheritanceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_tax_registration_response(
    result: &GetTaxRegistrationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_tax_registration_document_response(
    result: &GetTaxRegistrationDocumentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_supplemental_tax_registrations_response(
    result: &ListSupplementalTaxRegistrationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tax_exemptions_response(result: &ListTaxExemptionsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tax_registrations_response(
    result: &ListTaxRegistrationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_supplemental_tax_registration_response(
    result: &PutSupplementalTaxRegistrationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_tax_exemption_response(result: &PutTaxExemptionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_tax_inheritance_response(result: &PutTaxInheritanceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_tax_registration_response(
    result: &PutTaxRegistrationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_delete_tax_registration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchDeleteTaxRegistrationRequest, String> {
    let mut input = BatchDeleteTaxRegistrationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchDeleteTaxRegistrationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize BatchDeleteTaxRegistration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_get_tax_exemptions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchGetTaxExemptionsRequest, String> {
    let mut input = BatchGetTaxExemptionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchGetTaxExemptionsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchGetTaxExemptions request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_put_tax_registration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchPutTaxRegistrationRequest, String> {
    let mut input = BatchPutTaxRegistrationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchPutTaxRegistrationRequest>(&request.body).map_err(
            |err| format!("failed to deserialize BatchPutTaxRegistration request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_supplemental_tax_registration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteSupplementalTaxRegistrationRequest, String> {
    let mut input = DeleteSupplementalTaxRegistrationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteSupplementalTaxRegistrationRequest>(&request.body)
            .map_err(|err| {
            format!("failed to deserialize DeleteSupplementalTaxRegistration request: {err}")
        })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_tax_registration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTaxRegistrationRequest, String> {
    let mut input = DeleteTaxRegistrationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteTaxRegistrationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteTaxRegistration request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_tax_exemption_types_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTaxExemptionTypesRequest, String> {
    let input = GetTaxExemptionTypesRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_tax_inheritance_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTaxInheritanceRequest, String> {
    let input = GetTaxInheritanceRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_tax_registration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTaxRegistrationRequest, String> {
    let mut input = GetTaxRegistrationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetTaxRegistrationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetTaxRegistration request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_tax_registration_document_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTaxRegistrationDocumentRequest, String> {
    let mut input = GetTaxRegistrationDocumentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetTaxRegistrationDocumentRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize GetTaxRegistrationDocument request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_supplemental_tax_registrations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSupplementalTaxRegistrationsRequest, String> {
    let mut input = ListSupplementalTaxRegistrationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListSupplementalTaxRegistrationsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListSupplementalTaxRegistrations request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_tax_exemptions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTaxExemptionsRequest, String> {
    let mut input = ListTaxExemptionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListTaxExemptionsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListTaxExemptions request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_tax_registrations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTaxRegistrationsRequest, String> {
    let mut input = ListTaxRegistrationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListTaxRegistrationsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListTaxRegistrations request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_supplemental_tax_registration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutSupplementalTaxRegistrationRequest, String> {
    let mut input = PutSupplementalTaxRegistrationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutSupplementalTaxRegistrationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutSupplementalTaxRegistration request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_tax_exemption_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutTaxExemptionRequest, String> {
    let mut input = PutTaxExemptionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutTaxExemptionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutTaxExemption request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_tax_inheritance_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutTaxInheritanceRequest, String> {
    let mut input = PutTaxInheritanceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutTaxInheritanceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutTaxInheritance request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_tax_registration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutTaxRegistrationRequest, String> {
    let mut input = PutTaxRegistrationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutTaxRegistrationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutTaxRegistration request: {err}"))?;
    }
    Ok(input)
}
