//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-signer

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
pub fn serialize_add_profile_permission_response(
    result: &AddProfilePermissionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_cancel_signing_profile_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_signing_job_response(
    result: &DescribeSigningJobResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_revocation_status_response(
    result: &GetRevocationStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_signing_platform_response(
    result: &GetSigningPlatformResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_signing_profile_response(result: &GetSigningProfileResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_profile_permissions_response(
    result: &ListProfilePermissionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_signing_jobs_response(result: &ListSigningJobsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_signing_platforms_response(
    result: &ListSigningPlatformsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_signing_profiles_response(
    result: &ListSigningProfilesResponse,
) -> MockResponse {
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
pub fn serialize_put_signing_profile_response(result: &PutSigningProfileResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_remove_profile_permission_response(
    result: &RemoveProfilePermissionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_revoke_signature_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_revoke_signing_profile_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_sign_payload_response(result: &SignPayloadResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_signing_job_response(result: &StartSigningJobResponse) -> MockResponse {
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

/// Deserialize request for restJson protocol.
pub fn deserialize_add_profile_permission_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AddProfilePermissionRequest, String> {
    let mut input = AddProfilePermissionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AddProfilePermissionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AddProfilePermission request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "profileName" => {
                input.profile_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_signing_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelSigningProfileRequest, String> {
    let mut input = CancelSigningProfileRequest::default();
    for (name, value) in labels {
        match *name {
            "profileName" => {
                input.profile_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_signing_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeSigningJobRequest, String> {
    let mut input = DescribeSigningJobRequest::default();
    for (name, value) in labels {
        match *name {
            "jobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_revocation_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetRevocationStatusRequest, String> {
    let mut input = GetRevocationStatusRequest::default();
    if let Some(value) = query.get("certificateHashes") {
        input.certificate_hashes = value
            .split(',')
            .filter(|item| !item.trim().is_empty())
            .map(|item| Ok(item.trim().to_string()))
            .collect::<Result<Vec<_>, String>>()?;
    }
    if let Some(value) = query.get("jobArn") {
        input.job_arn = value.to_string();
    }
    if let Some(value) = query.get("platformId") {
        input.platform_id = value.to_string();
    }
    if let Some(value) = query.get("profileVersionArn") {
        input.profile_version_arn = value.to_string();
    }
    if let Some(value) = query.get("signatureTimestamp") {
        input.signature_timestamp = value
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
pub fn deserialize_get_signing_platform_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSigningPlatformRequest, String> {
    let mut input = GetSigningPlatformRequest::default();
    for (name, value) in labels {
        match *name {
            "platformId" => {
                input.platform_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_signing_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSigningProfileRequest, String> {
    let mut input = GetSigningProfileRequest::default();
    for (name, value) in labels {
        match *name {
            "profileName" => {
                input.profile_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("profileOwner") {
        input.profile_owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_profile_permissions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListProfilePermissionsRequest, String> {
    let mut input = ListProfilePermissionsRequest::default();
    for (name, value) in labels {
        match *name {
            "profileName" => {
                input.profile_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_signing_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSigningJobsRequest, String> {
    let mut input = ListSigningJobsRequest::default();
    if let Some(value) = query.get("isRevoked") {
        input.is_revoked = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("jobInvoker") {
        input.job_invoker = Some(value.to_string());
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
    if let Some(value) = query.get("platformId") {
        input.platform_id = Some(value.to_string());
    }
    if let Some(value) = query.get("requestedBy") {
        input.requested_by = Some(value.to_string());
    }
    if let Some(value) = query.get("signatureExpiresAfter") {
        input.signature_expires_after = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("signatureExpiresBefore") {
        input.signature_expires_before = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("status") {
        input.status = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_signing_platforms_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSigningPlatformsRequest, String> {
    let mut input = ListSigningPlatformsRequest::default();
    if let Some(value) = query.get("category") {
        input.category = Some(value.to_string());
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
    if let Some(value) = query.get("partner") {
        input.partner = Some(value.to_string());
    }
    if let Some(value) = query.get("target") {
        input.target = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_signing_profiles_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSigningProfilesRequest, String> {
    let mut input = ListSigningProfilesRequest::default();
    if let Some(value) = query.get("includeCanceled") {
        input.include_canceled = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
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
    if let Some(value) = query.get("platformId") {
        input.platform_id = Some(value.to_string());
    }
    if let Some(value) = query.get("statuses") {
        input.statuses = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
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
pub fn deserialize_put_signing_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutSigningProfileRequest, String> {
    let mut input = PutSigningProfileRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutSigningProfileRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutSigningProfile request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "profileName" => {
                input.profile_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_remove_profile_permission_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RemoveProfilePermissionRequest, String> {
    let mut input = RemoveProfilePermissionRequest::default();
    for (name, value) in labels {
        match *name {
            "profileName" => {
                input.profile_name = value.to_string();
            }
            "statementId" => {
                input.statement_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("revisionId") {
        input.revision_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_revoke_signature_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RevokeSignatureRequest, String> {
    let mut input = RevokeSignatureRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RevokeSignatureRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize RevokeSignature request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "jobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_revoke_signing_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RevokeSigningProfileRequest, String> {
    let mut input = RevokeSigningProfileRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RevokeSigningProfileRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize RevokeSigningProfile request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "profileName" => {
                input.profile_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_sign_payload_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SignPayloadRequest, String> {
    let mut input = SignPayloadRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SignPayloadRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SignPayload request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_signing_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartSigningJobRequest, String> {
    let mut input = StartSigningJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartSigningJobRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartSigningJob request: {err}"))?;
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
