//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-amplifybackend

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
pub fn serialize_clone_backend_response(result: &CloneBackendResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_backend_response(result: &CreateBackendResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_backend_a_p_i_response(result: &CreateBackendAPIResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_backend_auth_response(result: &CreateBackendAuthResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_backend_config_response(
    result: &CreateBackendConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_backend_storage_response(
    result: &CreateBackendStorageResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_token_response(result: &CreateTokenResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_backend_response(result: &DeleteBackendResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_backend_a_p_i_response(result: &DeleteBackendAPIResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_backend_auth_response(result: &DeleteBackendAuthResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_backend_storage_response(
    result: &DeleteBackendStorageResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_token_response(result: &DeleteTokenResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_generate_backend_a_p_i_models_response(
    result: &GenerateBackendAPIModelsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_backend_response(result: &GetBackendResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_backend_a_p_i_response(result: &GetBackendAPIResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_backend_a_p_i_models_response(
    result: &GetBackendAPIModelsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_backend_auth_response(result: &GetBackendAuthResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_backend_job_response(result: &GetBackendJobResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_backend_storage_response(result: &GetBackendStorageResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_token_response(result: &GetTokenResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_import_backend_auth_response(result: &ImportBackendAuthResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_import_backend_storage_response(
    result: &ImportBackendStorageResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_backend_jobs_response(result: &ListBackendJobsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_s3_buckets_response(result: &ListS3BucketsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_remove_all_backends_response(result: &RemoveAllBackendsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_remove_backend_config_response(
    result: &RemoveBackendConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_backend_a_p_i_response(result: &UpdateBackendAPIResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_backend_auth_response(result: &UpdateBackendAuthResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_backend_config_response(
    result: &UpdateBackendConfigResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_backend_job_response(result: &UpdateBackendJobResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_backend_storage_response(
    result: &UpdateBackendStorageResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_clone_backend_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CloneBackendRequest, String> {
    let mut input = CloneBackendRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CloneBackendRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CloneBackend request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            "BackendEnvironmentName" => {
                input.backend_environment_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_backend_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBackendRequest, String> {
    let mut input = CreateBackendRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateBackendRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateBackend request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_backend_a_p_i_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBackendAPIRequest, String> {
    let mut input = CreateBackendAPIRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateBackendAPIRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateBackendAPI request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_backend_auth_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBackendAuthRequest, String> {
    let mut input = CreateBackendAuthRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateBackendAuthRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateBackendAuth request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_backend_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBackendConfigRequest, String> {
    let mut input = CreateBackendConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateBackendConfigRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateBackendConfig request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_backend_storage_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBackendStorageRequest, String> {
    let mut input = CreateBackendStorageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateBackendStorageRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateBackendStorage request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_token_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTokenRequest, String> {
    let mut input = CreateTokenRequest::default();
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_backend_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBackendRequest, String> {
    let mut input = DeleteBackendRequest::default();
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            "BackendEnvironmentName" => {
                input.backend_environment_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_backend_a_p_i_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBackendAPIRequest, String> {
    let mut input = DeleteBackendAPIRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteBackendAPIRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteBackendAPI request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            "BackendEnvironmentName" => {
                input.backend_environment_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_backend_auth_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBackendAuthRequest, String> {
    let mut input = DeleteBackendAuthRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteBackendAuthRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteBackendAuth request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            "BackendEnvironmentName" => {
                input.backend_environment_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_backend_storage_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBackendStorageRequest, String> {
    let mut input = DeleteBackendStorageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteBackendStorageRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteBackendStorage request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            "BackendEnvironmentName" => {
                input.backend_environment_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_token_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTokenRequest, String> {
    let mut input = DeleteTokenRequest::default();
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            "SessionId" => {
                input.session_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_generate_backend_a_p_i_models_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GenerateBackendAPIModelsRequest, String> {
    let mut input = GenerateBackendAPIModelsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GenerateBackendAPIModelsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize GenerateBackendAPIModels request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            "BackendEnvironmentName" => {
                input.backend_environment_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_backend_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBackendRequest, String> {
    let mut input = GetBackendRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetBackendRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetBackend request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_backend_a_p_i_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBackendAPIRequest, String> {
    let mut input = GetBackendAPIRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetBackendAPIRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetBackendAPI request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            "BackendEnvironmentName" => {
                input.backend_environment_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_backend_a_p_i_models_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBackendAPIModelsRequest, String> {
    let mut input = GetBackendAPIModelsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetBackendAPIModelsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetBackendAPIModels request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            "BackendEnvironmentName" => {
                input.backend_environment_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_backend_auth_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBackendAuthRequest, String> {
    let mut input = GetBackendAuthRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetBackendAuthRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetBackendAuth request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            "BackendEnvironmentName" => {
                input.backend_environment_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_backend_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBackendJobRequest, String> {
    let mut input = GetBackendJobRequest::default();
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            "BackendEnvironmentName" => {
                input.backend_environment_name = value.to_string();
            }
            "JobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_backend_storage_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBackendStorageRequest, String> {
    let mut input = GetBackendStorageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetBackendStorageRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetBackendStorage request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            "BackendEnvironmentName" => {
                input.backend_environment_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_token_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTokenRequest, String> {
    let mut input = GetTokenRequest::default();
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            "SessionId" => {
                input.session_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_import_backend_auth_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ImportBackendAuthRequest, String> {
    let mut input = ImportBackendAuthRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ImportBackendAuthRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ImportBackendAuth request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            "BackendEnvironmentName" => {
                input.backend_environment_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_import_backend_storage_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ImportBackendStorageRequest, String> {
    let mut input = ImportBackendStorageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ImportBackendStorageRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ImportBackendStorage request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            "BackendEnvironmentName" => {
                input.backend_environment_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_backend_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBackendJobsRequest, String> {
    let mut input = ListBackendJobsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListBackendJobsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListBackendJobs request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            "BackendEnvironmentName" => {
                input.backend_environment_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_s3_buckets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListS3BucketsRequest, String> {
    let mut input = ListS3BucketsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListS3BucketsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListS3Buckets request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_remove_all_backends_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RemoveAllBackendsRequest, String> {
    let mut input = RemoveAllBackendsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RemoveAllBackendsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize RemoveAllBackends request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_remove_backend_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RemoveBackendConfigRequest, String> {
    let mut input = RemoveBackendConfigRequest::default();
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_backend_a_p_i_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBackendAPIRequest, String> {
    let mut input = UpdateBackendAPIRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBackendAPIRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateBackendAPI request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            "BackendEnvironmentName" => {
                input.backend_environment_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_backend_auth_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBackendAuthRequest, String> {
    let mut input = UpdateBackendAuthRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBackendAuthRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateBackendAuth request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            "BackendEnvironmentName" => {
                input.backend_environment_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_backend_config_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBackendConfigRequest, String> {
    let mut input = UpdateBackendConfigRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBackendConfigRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateBackendConfig request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_backend_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBackendJobRequest, String> {
    let mut input = UpdateBackendJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBackendJobRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateBackendJob request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            "BackendEnvironmentName" => {
                input.backend_environment_name = value.to_string();
            }
            "JobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_backend_storage_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBackendStorageRequest, String> {
    let mut input = UpdateBackendStorageRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBackendStorageRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateBackendStorage request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "AppId" => {
                input.app_id = value.to_string();
            }
            "BackendEnvironmentName" => {
                input.backend_environment_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
