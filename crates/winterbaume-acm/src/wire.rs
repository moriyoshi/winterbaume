//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-acm

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize void response for awsJson protocol.
pub fn serialize_add_tags_to_certificate_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_certificate_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_certificate_response(
    result: &DescribeCertificateResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_export_certificate_response(result: &ExportCertificateResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_account_configuration_response(
    result: &GetAccountConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_certificate_response(result: &GetCertificateResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_import_certificate_response(result: &ImportCertificateResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_certificates_response(result: &ListCertificatesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_for_certificate_response(
    result: &ListTagsForCertificateResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_account_configuration_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_remove_tags_from_certificate_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_renew_certificate_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_request_certificate_response(result: &RequestCertificateResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_resend_validation_email_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_revoke_certificate_response(result: &RevokeCertificateResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_search_certificates_response(result: &SearchCertificatesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_update_certificate_options_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_tags_to_certificate_request(
    body: &[u8],
) -> Result<AddTagsToCertificateRequest, String> {
    if body.is_empty() {
        return Ok(AddTagsToCertificateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AddTagsToCertificate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_certificate_request(
    body: &[u8],
) -> Result<DeleteCertificateRequest, String> {
    if body.is_empty() {
        return Ok(DeleteCertificateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteCertificate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_certificate_request(
    body: &[u8],
) -> Result<DescribeCertificateRequest, String> {
    if body.is_empty() {
        return Ok(DescribeCertificateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeCertificate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_export_certificate_request(
    body: &[u8],
) -> Result<ExportCertificateRequest, String> {
    if body.is_empty() {
        return Ok(ExportCertificateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ExportCertificate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_certificate_request(body: &[u8]) -> Result<GetCertificateRequest, String> {
    if body.is_empty() {
        return Ok(GetCertificateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCertificate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_import_certificate_request(
    body: &[u8],
) -> Result<ImportCertificateRequest, String> {
    if body.is_empty() {
        return Ok(ImportCertificateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ImportCertificate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_certificates_request(
    body: &[u8],
) -> Result<ListCertificatesRequest, String> {
    if body.is_empty() {
        return Ok(ListCertificatesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListCertificates request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_for_certificate_request(
    body: &[u8],
) -> Result<ListTagsForCertificateRequest, String> {
    if body.is_empty() {
        return Ok(ListTagsForCertificateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTagsForCertificate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_account_configuration_request(
    body: &[u8],
) -> Result<PutAccountConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(PutAccountConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutAccountConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_remove_tags_from_certificate_request(
    body: &[u8],
) -> Result<RemoveTagsFromCertificateRequest, String> {
    if body.is_empty() {
        return Ok(RemoveTagsFromCertificateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RemoveTagsFromCertificate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_renew_certificate_request(
    body: &[u8],
) -> Result<RenewCertificateRequest, String> {
    if body.is_empty() {
        return Ok(RenewCertificateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RenewCertificate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_request_certificate_request(
    body: &[u8],
) -> Result<RequestCertificateRequest, String> {
    if body.is_empty() {
        return Ok(RequestCertificateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RequestCertificate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_resend_validation_email_request(
    body: &[u8],
) -> Result<ResendValidationEmailRequest, String> {
    if body.is_empty() {
        return Ok(ResendValidationEmailRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ResendValidationEmail request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_revoke_certificate_request(
    body: &[u8],
) -> Result<RevokeCertificateRequest, String> {
    if body.is_empty() {
        return Ok(RevokeCertificateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RevokeCertificate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_search_certificates_request(
    body: &[u8],
) -> Result<SearchCertificatesRequest, String> {
    if body.is_empty() {
        return Ok(SearchCertificatesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SearchCertificates request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_certificate_options_request(
    body: &[u8],
) -> Result<UpdateCertificateOptionsRequest, String> {
    if body.is_empty() {
        return Ok(UpdateCertificateOptionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateCertificateOptions request: {e}"))
}
