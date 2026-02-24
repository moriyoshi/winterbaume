//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-acmpca

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_create_certificate_authority_response(
    result: &CreateCertificateAuthorityResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_certificate_authority_audit_report_response(
    result: &CreateCertificateAuthorityAuditReportResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_create_permission_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_certificate_authority_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_permission_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_policy_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_certificate_authority_response(
    result: &DescribeCertificateAuthorityResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_certificate_authority_audit_report_response(
    result: &DescribeCertificateAuthorityAuditReportResponse,
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
pub fn serialize_get_certificate_authority_certificate_response(
    result: &GetCertificateAuthorityCertificateResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_certificate_authority_csr_response(
    result: &GetCertificateAuthorityCsrResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_policy_response(result: &GetPolicyResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_import_certificate_authority_certificate_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_issue_certificate_response(result: &IssueCertificateResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_certificate_authorities_response(
    result: &ListCertificateAuthoritiesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_permissions_response(result: &ListPermissionsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_response(result: &ListTagsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_policy_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_restore_certificate_authority_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_revoke_certificate_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_tag_certificate_authority_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_untag_certificate_authority_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_update_certificate_authority_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_certificate_authority_request(
    body: &[u8],
) -> Result<CreateCertificateAuthorityRequest, String> {
    if body.is_empty() {
        return Ok(CreateCertificateAuthorityRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateCertificateAuthority request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_certificate_authority_audit_report_request(
    body: &[u8],
) -> Result<CreateCertificateAuthorityAuditReportRequest, String> {
    if body.is_empty() {
        return Ok(CreateCertificateAuthorityAuditReportRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize CreateCertificateAuthorityAuditReport request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_permission_request(
    body: &[u8],
) -> Result<CreatePermissionRequest, String> {
    if body.is_empty() {
        return Ok(CreatePermissionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreatePermission request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_certificate_authority_request(
    body: &[u8],
) -> Result<DeleteCertificateAuthorityRequest, String> {
    if body.is_empty() {
        return Ok(DeleteCertificateAuthorityRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteCertificateAuthority request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_permission_request(
    body: &[u8],
) -> Result<DeletePermissionRequest, String> {
    if body.is_empty() {
        return Ok(DeletePermissionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeletePermission request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_policy_request(body: &[u8]) -> Result<DeletePolicyRequest, String> {
    if body.is_empty() {
        return Ok(DeletePolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeletePolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_certificate_authority_request(
    body: &[u8],
) -> Result<DescribeCertificateAuthorityRequest, String> {
    if body.is_empty() {
        return Ok(DescribeCertificateAuthorityRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeCertificateAuthority request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_certificate_authority_audit_report_request(
    body: &[u8],
) -> Result<DescribeCertificateAuthorityAuditReportRequest, String> {
    if body.is_empty() {
        return Ok(DescribeCertificateAuthorityAuditReportRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeCertificateAuthorityAuditReport request: {e}")
    })
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
pub fn deserialize_get_certificate_authority_certificate_request(
    body: &[u8],
) -> Result<GetCertificateAuthorityCertificateRequest, String> {
    if body.is_empty() {
        return Ok(GetCertificateAuthorityCertificateRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetCertificateAuthorityCertificate request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_certificate_authority_csr_request(
    body: &[u8],
) -> Result<GetCertificateAuthorityCsrRequest, String> {
    if body.is_empty() {
        return Ok(GetCertificateAuthorityCsrRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCertificateAuthorityCsr request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_policy_request(body: &[u8]) -> Result<GetPolicyRequest, String> {
    if body.is_empty() {
        return Ok(GetPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_import_certificate_authority_certificate_request(
    body: &[u8],
) -> Result<ImportCertificateAuthorityCertificateRequest, String> {
    if body.is_empty() {
        return Ok(ImportCertificateAuthorityCertificateRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ImportCertificateAuthorityCertificate request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_issue_certificate_request(
    body: &[u8],
) -> Result<IssueCertificateRequest, String> {
    if body.is_empty() {
        return Ok(IssueCertificateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize IssueCertificate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_certificate_authorities_request(
    body: &[u8],
) -> Result<ListCertificateAuthoritiesRequest, String> {
    if body.is_empty() {
        return Ok(ListCertificateAuthoritiesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListCertificateAuthorities request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_permissions_request(body: &[u8]) -> Result<ListPermissionsRequest, String> {
    if body.is_empty() {
        return Ok(ListPermissionsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListPermissions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_request(body: &[u8]) -> Result<ListTagsRequest, String> {
    if body.is_empty() {
        return Ok(ListTagsRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize ListTags request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_policy_request(body: &[u8]) -> Result<PutPolicyRequest, String> {
    if body.is_empty() {
        return Ok(PutPolicyRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutPolicy request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_restore_certificate_authority_request(
    body: &[u8],
) -> Result<RestoreCertificateAuthorityRequest, String> {
    if body.is_empty() {
        return Ok(RestoreCertificateAuthorityRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RestoreCertificateAuthority request: {e}"))
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
pub fn deserialize_tag_certificate_authority_request(
    body: &[u8],
) -> Result<TagCertificateAuthorityRequest, String> {
    if body.is_empty() {
        return Ok(TagCertificateAuthorityRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TagCertificateAuthority request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_untag_certificate_authority_request(
    body: &[u8],
) -> Result<UntagCertificateAuthorityRequest, String> {
    if body.is_empty() {
        return Ok(UntagCertificateAuthorityRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UntagCertificateAuthority request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_certificate_authority_request(
    body: &[u8],
) -> Result<UpdateCertificateAuthorityRequest, String> {
    if body.is_empty() {
        return Ok(UpdateCertificateAuthorityRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateCertificateAuthority request: {e}"))
}
