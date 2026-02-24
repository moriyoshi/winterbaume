//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-appconfig

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
pub fn serialize_create_application_response(result: &Application) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_configuration_profile_response(
    result: &ConfigurationProfile,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_deployment_strategy_response(result: &DeploymentStrategy) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_environment_response(result: &Environment) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_extension_response(result: &Extension) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_extension_association_response(
    result: &ExtensionAssociation,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_hosted_configuration_version_response(
    result: &HostedConfigurationVersion,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "application-id": set by caller from application_id field
    // Header "configuration-profile-id": set by caller from configuration_profile_id field
    // Header "content-type": set by caller from content_type field
    // Header "description": set by caller from description field
    // Header "kmskeyarn": set by caller from kms_key_arn field
    // Header "versionlabel": set by caller from version_label field
    // Header "version-number": set by caller from version_number field
    resp
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_application_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_configuration_profile_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_deployment_strategy_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_environment_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_extension_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_extension_association_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_hosted_configuration_version_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_get_account_settings_response(result: &AccountSettings) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_application_response(result: &Application) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_configuration_response(result: &Configuration) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "configuration-version": set by caller from configuration_version field
    // Header "content-type": set by caller from content_type field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_get_configuration_profile_response(result: &ConfigurationProfile) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_deployment_response(result: &Deployment) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_deployment_strategy_response(result: &DeploymentStrategy) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_environment_response(result: &Environment) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_extension_response(result: &Extension) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_extension_association_response(result: &ExtensionAssociation) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_hosted_configuration_version_response(
    result: &HostedConfigurationVersion,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "application-id": set by caller from application_id field
    // Header "configuration-profile-id": set by caller from configuration_profile_id field
    // Header "content-type": set by caller from content_type field
    // Header "description": set by caller from description field
    // Header "kmskeyarn": set by caller from kms_key_arn field
    // Header "versionlabel": set by caller from version_label field
    // Header "version-number": set by caller from version_number field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_list_applications_response(result: &Applications) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_configuration_profiles_response(
    result: &ConfigurationProfiles,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_deployment_strategies_response(
    result: &DeploymentStrategies,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_deployments_response(result: &Deployments) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_environments_response(result: &Environments) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_extension_associations_response(
    result: &ExtensionAssociations,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_extensions_response(result: &Extensions) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_hosted_configuration_versions_response(
    result: &HostedConfigurationVersions,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_for_resource_response(result: &ResourceTags) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_deployment_response(result: &Deployment) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_deployment_response(result: &Deployment) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_tag_resource_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_account_settings_response(result: &AccountSettings) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_application_response(result: &Application) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_configuration_profile_response(
    result: &ConfigurationProfile,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_deployment_strategy_response(result: &DeploymentStrategy) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_environment_response(result: &Environment) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_extension_response(result: &Extension) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_extension_association_response(
    result: &ExtensionAssociation,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_validate_configuration_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_application_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateApplicationRequest, String> {
    let mut input = CreateApplicationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateApplicationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateApplication request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_configuration_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateConfigurationProfileRequest, String> {
    let mut input = CreateConfigurationProfileRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateConfigurationProfileRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateConfigurationProfile request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ApplicationId" => {
                input.application_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_deployment_strategy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDeploymentStrategyRequest, String> {
    let mut input = CreateDeploymentStrategyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDeploymentStrategyRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateDeploymentStrategy request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_environment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateEnvironmentRequest, String> {
    let mut input = CreateEnvironmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateEnvironmentRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateEnvironment request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ApplicationId" => {
                input.application_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_extension_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateExtensionRequest, String> {
    let mut input = CreateExtensionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateExtensionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateExtension request: {err}"))?;
    }
    if let Some(value) = request
        .headers
        .get("Latest-Version-Number")
        .and_then(|value| value.to_str().ok())
    {
        input.latest_version_number = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_extension_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateExtensionAssociationRequest, String> {
    let mut input = CreateExtensionAssociationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateExtensionAssociationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateExtensionAssociation request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_hosted_configuration_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateHostedConfigurationVersionRequest, String> {
    let mut input = CreateHostedConfigurationVersionRequest::default();
    if !request.body.is_empty() {
        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;
        input.content = body.to_string();
    }
    for (name, value) in labels {
        match *name {
            "ApplicationId" => {
                input.application_id = value.to_string();
            }
            "ConfigurationProfileId" => {
                input.configuration_profile_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("Content-Type")
        .and_then(|value| value.to_str().ok())
    {
        input.content_type = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("Description")
        .and_then(|value| value.to_str().ok())
    {
        input.description = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("Latest-Version-Number")
        .and_then(|value| value.to_str().ok())
    {
        input.latest_version_number = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = request
        .headers
        .get("VersionLabel")
        .and_then(|value| value.to_str().ok())
    {
        input.version_label = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_application_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteApplicationRequest, String> {
    let mut input = DeleteApplicationRequest::default();
    for (name, value) in labels {
        match *name {
            "ApplicationId" => {
                input.application_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_configuration_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteConfigurationProfileRequest, String> {
    let mut input = DeleteConfigurationProfileRequest::default();
    for (name, value) in labels {
        match *name {
            "ApplicationId" => {
                input.application_id = value.to_string();
            }
            "ConfigurationProfileId" => {
                input.configuration_profile_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amzn-deletion-protection-check")
        .and_then(|value| value.to_str().ok())
    {
        input.deletion_protection_check = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_deployment_strategy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDeploymentStrategyRequest, String> {
    let mut input = DeleteDeploymentStrategyRequest::default();
    for (name, value) in labels {
        match *name {
            "DeploymentStrategyId" => {
                input.deployment_strategy_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_environment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteEnvironmentRequest, String> {
    let mut input = DeleteEnvironmentRequest::default();
    for (name, value) in labels {
        match *name {
            "ApplicationId" => {
                input.application_id = value.to_string();
            }
            "EnvironmentId" => {
                input.environment_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amzn-deletion-protection-check")
        .and_then(|value| value.to_str().ok())
    {
        input.deletion_protection_check = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_extension_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteExtensionRequest, String> {
    let mut input = DeleteExtensionRequest::default();
    for (name, value) in labels {
        match *name {
            "ExtensionIdentifier" => {
                input.extension_identifier = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("version") {
        input.version_number = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_extension_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteExtensionAssociationRequest, String> {
    let mut input = DeleteExtensionAssociationRequest::default();
    for (name, value) in labels {
        match *name {
            "ExtensionAssociationId" => {
                input.extension_association_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_hosted_configuration_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteHostedConfigurationVersionRequest, String> {
    let mut input = DeleteHostedConfigurationVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "ApplicationId" => {
                input.application_id = value.to_string();
            }
            "ConfigurationProfileId" => {
                input.configuration_profile_id = value.to_string();
            }
            "VersionNumber" => {
                input.version_number = value
                    .parse::<i32>()
                    .map_err(|err| format!("failed to parse integer: {err}"))?;
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_application_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetApplicationRequest, String> {
    let mut input = GetApplicationRequest::default();
    for (name, value) in labels {
        match *name {
            "ApplicationId" => {
                input.application_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetConfigurationRequest, String> {
    let mut input = GetConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "Application" => {
                input.application = value.to_string();
            }
            "Configuration" => {
                input.configuration = value.to_string();
            }
            "Environment" => {
                input.environment = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("client_configuration_version") {
        input.client_configuration_version = Some(value.to_string());
    }
    if let Some(value) = query.get("client_id") {
        input.client_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_configuration_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetConfigurationProfileRequest, String> {
    let mut input = GetConfigurationProfileRequest::default();
    for (name, value) in labels {
        match *name {
            "ApplicationId" => {
                input.application_id = value.to_string();
            }
            "ConfigurationProfileId" => {
                input.configuration_profile_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_deployment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDeploymentRequest, String> {
    let mut input = GetDeploymentRequest::default();
    for (name, value) in labels {
        match *name {
            "ApplicationId" => {
                input.application_id = value.to_string();
            }
            "DeploymentNumber" => {
                input.deployment_number = value
                    .parse::<i32>()
                    .map_err(|err| format!("failed to parse integer: {err}"))?;
            }
            "EnvironmentId" => {
                input.environment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_deployment_strategy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDeploymentStrategyRequest, String> {
    let mut input = GetDeploymentStrategyRequest::default();
    for (name, value) in labels {
        match *name {
            "DeploymentStrategyId" => {
                input.deployment_strategy_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_environment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetEnvironmentRequest, String> {
    let mut input = GetEnvironmentRequest::default();
    for (name, value) in labels {
        match *name {
            "ApplicationId" => {
                input.application_id = value.to_string();
            }
            "EnvironmentId" => {
                input.environment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_extension_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetExtensionRequest, String> {
    let mut input = GetExtensionRequest::default();
    for (name, value) in labels {
        match *name {
            "ExtensionIdentifier" => {
                input.extension_identifier = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("version_number") {
        input.version_number = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_extension_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetExtensionAssociationRequest, String> {
    let mut input = GetExtensionAssociationRequest::default();
    for (name, value) in labels {
        match *name {
            "ExtensionAssociationId" => {
                input.extension_association_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_hosted_configuration_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetHostedConfigurationVersionRequest, String> {
    let mut input = GetHostedConfigurationVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "ApplicationId" => {
                input.application_id = value.to_string();
            }
            "ConfigurationProfileId" => {
                input.configuration_profile_id = value.to_string();
            }
            "VersionNumber" => {
                input.version_number = value
                    .parse::<i32>()
                    .map_err(|err| format!("failed to parse integer: {err}"))?;
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_applications_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListApplicationsRequest, String> {
    let mut input = ListApplicationsRequest::default();
    if let Some(value) = query.get("max_results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next_token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_configuration_profiles_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListConfigurationProfilesRequest, String> {
    let mut input = ListConfigurationProfilesRequest::default();
    for (name, value) in labels {
        match *name {
            "ApplicationId" => {
                input.application_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max_results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next_token") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("type") {
        input.r#type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_deployment_strategies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDeploymentStrategiesRequest, String> {
    let mut input = ListDeploymentStrategiesRequest::default();
    if let Some(value) = query.get("max_results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next_token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_deployments_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDeploymentsRequest, String> {
    let mut input = ListDeploymentsRequest::default();
    for (name, value) in labels {
        match *name {
            "ApplicationId" => {
                input.application_id = value.to_string();
            }
            "EnvironmentId" => {
                input.environment_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max_results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next_token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_environments_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListEnvironmentsRequest, String> {
    let mut input = ListEnvironmentsRequest::default();
    for (name, value) in labels {
        match *name {
            "ApplicationId" => {
                input.application_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max_results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next_token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_extension_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListExtensionAssociationsRequest, String> {
    let mut input = ListExtensionAssociationsRequest::default();
    if let Some(value) = query.get("extension_identifier") {
        input.extension_identifier = Some(value.to_string());
    }
    if let Some(value) = query.get("extension_version_number") {
        input.extension_version_number = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("max_results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next_token") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("resource_identifier") {
        input.resource_identifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_extensions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListExtensionsRequest, String> {
    let mut input = ListExtensionsRequest::default();
    if let Some(value) = query.get("max_results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("name") {
        input.name = Some(value.to_string());
    }
    if let Some(value) = query.get("next_token") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_hosted_configuration_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListHostedConfigurationVersionsRequest, String> {
    let mut input = ListHostedConfigurationVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "ApplicationId" => {
                input.application_id = value.to_string();
            }
            "ConfigurationProfileId" => {
                input.configuration_profile_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("max_results") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("next_token") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("version_label") {
        input.version_label = Some(value.to_string());
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
            "ResourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_deployment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartDeploymentRequest, String> {
    let mut input = StartDeploymentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartDeploymentRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartDeployment request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ApplicationId" => {
                input.application_id = value.to_string();
            }
            "EnvironmentId" => {
                input.environment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_deployment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopDeploymentRequest, String> {
    let mut input = StopDeploymentRequest::default();
    for (name, value) in labels {
        match *name {
            "ApplicationId" => {
                input.application_id = value.to_string();
            }
            "DeploymentNumber" => {
                input.deployment_number = value
                    .parse::<i32>()
                    .map_err(|err| format!("failed to parse integer: {err}"))?;
            }
            "EnvironmentId" => {
                input.environment_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("Allow-Revert")
        .and_then(|value| value.to_str().ok())
    {
        input.allow_revert = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
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
) -> Result<UntagResourceRequest, String> {
    let mut input = UntagResourceRequest::default();
    for (name, value) in labels {
        match *name {
            "ResourceArn" => {
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

/// Deserialize request for restJson protocol.
pub fn deserialize_update_account_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAccountSettingsRequest, String> {
    let mut input = UpdateAccountSettingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAccountSettingsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateAccountSettings request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_application_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateApplicationRequest, String> {
    let mut input = UpdateApplicationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateApplicationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateApplication request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ApplicationId" => {
                input.application_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_configuration_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateConfigurationProfileRequest, String> {
    let mut input = UpdateConfigurationProfileRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateConfigurationProfileRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateConfigurationProfile request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ApplicationId" => {
                input.application_id = value.to_string();
            }
            "ConfigurationProfileId" => {
                input.configuration_profile_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_deployment_strategy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDeploymentStrategyRequest, String> {
    let mut input = UpdateDeploymentStrategyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDeploymentStrategyRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateDeploymentStrategy request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "DeploymentStrategyId" => {
                input.deployment_strategy_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_environment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateEnvironmentRequest, String> {
    let mut input = UpdateEnvironmentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateEnvironmentRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateEnvironment request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ApplicationId" => {
                input.application_id = value.to_string();
            }
            "EnvironmentId" => {
                input.environment_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_extension_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateExtensionRequest, String> {
    let mut input = UpdateExtensionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateExtensionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateExtension request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ExtensionIdentifier" => {
                input.extension_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_extension_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateExtensionAssociationRequest, String> {
    let mut input = UpdateExtensionAssociationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateExtensionAssociationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateExtensionAssociation request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ExtensionAssociationId" => {
                input.extension_association_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_validate_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ValidateConfigurationRequest, String> {
    let mut input = ValidateConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "ApplicationId" => {
                input.application_id = value.to_string();
            }
            "ConfigurationProfileId" => {
                input.configuration_profile_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("configuration_version") {
        input.configuration_version = value.to_string();
    }
    Ok(input)
}
