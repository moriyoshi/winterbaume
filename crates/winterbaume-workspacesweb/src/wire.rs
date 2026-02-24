//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-workspacesweb

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
pub fn serialize_associate_browser_settings_response(
    result: &AssociateBrowserSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_data_protection_settings_response(
    result: &AssociateDataProtectionSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_ip_access_settings_response(
    result: &AssociateIpAccessSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_network_settings_response(
    result: &AssociateNetworkSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_session_logger_response(
    result: &AssociateSessionLoggerResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_trust_store_response(
    result: &AssociateTrustStoreResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_user_access_logging_settings_response(
    result: &AssociateUserAccessLoggingSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_user_settings_response(
    result: &AssociateUserSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_browser_settings_response(
    result: &CreateBrowserSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_data_protection_settings_response(
    result: &CreateDataProtectionSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_identity_provider_response(
    result: &CreateIdentityProviderResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_ip_access_settings_response(
    result: &CreateIpAccessSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_network_settings_response(
    result: &CreateNetworkSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_portal_response(result: &CreatePortalResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_session_logger_response(
    result: &CreateSessionLoggerResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_trust_store_response(result: &CreateTrustStoreResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_user_access_logging_settings_response(
    result: &CreateUserAccessLoggingSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_user_settings_response(
    result: &CreateUserSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_browser_settings_response(
    result: &DeleteBrowserSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_data_protection_settings_response(
    result: &DeleteDataProtectionSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_identity_provider_response(
    result: &DeleteIdentityProviderResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_ip_access_settings_response(
    result: &DeleteIpAccessSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_network_settings_response(
    result: &DeleteNetworkSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_portal_response(result: &DeletePortalResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_session_logger_response(
    result: &DeleteSessionLoggerResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_trust_store_response(result: &DeleteTrustStoreResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_user_access_logging_settings_response(
    result: &DeleteUserAccessLoggingSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_user_settings_response(
    result: &DeleteUserSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_browser_settings_response(
    result: &DisassociateBrowserSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_data_protection_settings_response(
    result: &DisassociateDataProtectionSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_ip_access_settings_response(
    result: &DisassociateIpAccessSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_network_settings_response(
    result: &DisassociateNetworkSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_session_logger_response(
    result: &DisassociateSessionLoggerResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_trust_store_response(
    result: &DisassociateTrustStoreResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_user_access_logging_settings_response(
    result: &DisassociateUserAccessLoggingSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_user_settings_response(
    result: &DisassociateUserSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_expire_session_response(result: &ExpireSessionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_browser_settings_response(
    result: &GetBrowserSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_data_protection_settings_response(
    result: &GetDataProtectionSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_identity_provider_response(
    result: &GetIdentityProviderResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_ip_access_settings_response(
    result: &GetIpAccessSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_network_settings_response(
    result: &GetNetworkSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_portal_response(result: &GetPortalResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_portal_service_provider_metadata_response(
    result: &GetPortalServiceProviderMetadataResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_session_response(result: &GetSessionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_session_logger_response(result: &GetSessionLoggerResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_trust_store_response(result: &GetTrustStoreResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_trust_store_certificate_response(
    result: &GetTrustStoreCertificateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_user_access_logging_settings_response(
    result: &GetUserAccessLoggingSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_user_settings_response(result: &GetUserSettingsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_browser_settings_response(
    result: &ListBrowserSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_data_protection_settings_response(
    result: &ListDataProtectionSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_identity_providers_response(
    result: &ListIdentityProvidersResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_ip_access_settings_response(
    result: &ListIpAccessSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_network_settings_response(
    result: &ListNetworkSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_portals_response(result: &ListPortalsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_session_loggers_response(
    result: &ListSessionLoggersResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_sessions_response(result: &ListSessionsResponse) -> MockResponse {
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
pub fn serialize_list_trust_store_certificates_response(
    result: &ListTrustStoreCertificatesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_trust_stores_response(result: &ListTrustStoresResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_user_access_logging_settings_response(
    result: &ListUserAccessLoggingSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_user_settings_response(result: &ListUserSettingsResponse) -> MockResponse {
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

/// Serialize response for restJson protocol.
pub fn serialize_update_browser_settings_response(
    result: &UpdateBrowserSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_data_protection_settings_response(
    result: &UpdateDataProtectionSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_identity_provider_response(
    result: &UpdateIdentityProviderResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_ip_access_settings_response(
    result: &UpdateIpAccessSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_network_settings_response(
    result: &UpdateNetworkSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_portal_response(result: &UpdatePortalResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_session_logger_response(
    result: &UpdateSessionLoggerResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_trust_store_response(result: &UpdateTrustStoreResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_user_access_logging_settings_response(
    result: &UpdateUserAccessLoggingSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_user_settings_response(
    result: &UpdateUserSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_browser_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateBrowserSettingsRequest, String> {
    let mut input = AssociateBrowserSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "portalArn" => {
                input.portal_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("browserSettingsArn") {
        input.browser_settings_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_data_protection_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateDataProtectionSettingsRequest, String> {
    let mut input = AssociateDataProtectionSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "portalArn" => {
                input.portal_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("dataProtectionSettingsArn") {
        input.data_protection_settings_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_ip_access_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateIpAccessSettingsRequest, String> {
    let mut input = AssociateIpAccessSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "portalArn" => {
                input.portal_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("ipAccessSettingsArn") {
        input.ip_access_settings_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_network_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateNetworkSettingsRequest, String> {
    let mut input = AssociateNetworkSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "portalArn" => {
                input.portal_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("networkSettingsArn") {
        input.network_settings_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_session_logger_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateSessionLoggerRequest, String> {
    let mut input = AssociateSessionLoggerRequest::default();
    for (name, value) in labels {
        match *name {
            "portalArn" => {
                input.portal_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("sessionLoggerArn") {
        input.session_logger_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_trust_store_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateTrustStoreRequest, String> {
    let mut input = AssociateTrustStoreRequest::default();
    for (name, value) in labels {
        match *name {
            "portalArn" => {
                input.portal_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("trustStoreArn") {
        input.trust_store_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_user_access_logging_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateUserAccessLoggingSettingsRequest, String> {
    let mut input = AssociateUserAccessLoggingSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "portalArn" => {
                input.portal_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("userAccessLoggingSettingsArn") {
        input.user_access_logging_settings_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_user_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateUserSettingsRequest, String> {
    let mut input = AssociateUserSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "portalArn" => {
                input.portal_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("userSettingsArn") {
        input.user_settings_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_browser_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateBrowserSettingsRequest, String> {
    let mut input = CreateBrowserSettingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateBrowserSettingsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateBrowserSettings request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_data_protection_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDataProtectionSettingsRequest, String> {
    let mut input = CreateDataProtectionSettingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDataProtectionSettingsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateDataProtectionSettings request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_identity_provider_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateIdentityProviderRequest, String> {
    let mut input = CreateIdentityProviderRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateIdentityProviderRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateIdentityProvider request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_ip_access_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateIpAccessSettingsRequest, String> {
    let mut input = CreateIpAccessSettingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateIpAccessSettingsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateIpAccessSettings request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_network_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateNetworkSettingsRequest, String> {
    let mut input = CreateNetworkSettingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateNetworkSettingsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateNetworkSettings request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_portal_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreatePortalRequest, String> {
    let mut input = CreatePortalRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreatePortalRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreatePortal request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_session_logger_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateSessionLoggerRequest, String> {
    let mut input = CreateSessionLoggerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateSessionLoggerRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateSessionLogger request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_trust_store_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTrustStoreRequest, String> {
    let mut input = CreateTrustStoreRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTrustStoreRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateTrustStore request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_user_access_logging_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateUserAccessLoggingSettingsRequest, String> {
    let mut input = CreateUserAccessLoggingSettingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateUserAccessLoggingSettingsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateUserAccessLoggingSettings request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_user_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateUserSettingsRequest, String> {
    let mut input = CreateUserSettingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateUserSettingsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateUserSettings request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_browser_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteBrowserSettingsRequest, String> {
    let mut input = DeleteBrowserSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "browserSettingsArn" => {
                input.browser_settings_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_data_protection_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDataProtectionSettingsRequest, String> {
    let mut input = DeleteDataProtectionSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "dataProtectionSettingsArn" => {
                input.data_protection_settings_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_identity_provider_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteIdentityProviderRequest, String> {
    let mut input = DeleteIdentityProviderRequest::default();
    for (name, value) in labels {
        match *name {
            "identityProviderArn" => {
                input.identity_provider_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_ip_access_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteIpAccessSettingsRequest, String> {
    let mut input = DeleteIpAccessSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "ipAccessSettingsArn" => {
                input.ip_access_settings_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_network_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteNetworkSettingsRequest, String> {
    let mut input = DeleteNetworkSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "networkSettingsArn" => {
                input.network_settings_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_portal_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePortalRequest, String> {
    let mut input = DeletePortalRequest::default();
    for (name, value) in labels {
        match *name {
            "portalArn" => {
                input.portal_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_session_logger_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteSessionLoggerRequest, String> {
    let mut input = DeleteSessionLoggerRequest::default();
    for (name, value) in labels {
        match *name {
            "sessionLoggerArn" => {
                input.session_logger_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_trust_store_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTrustStoreRequest, String> {
    let mut input = DeleteTrustStoreRequest::default();
    for (name, value) in labels {
        match *name {
            "trustStoreArn" => {
                input.trust_store_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_user_access_logging_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteUserAccessLoggingSettingsRequest, String> {
    let mut input = DeleteUserAccessLoggingSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "userAccessLoggingSettingsArn" => {
                input.user_access_logging_settings_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_user_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteUserSettingsRequest, String> {
    let mut input = DeleteUserSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "userSettingsArn" => {
                input.user_settings_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_browser_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateBrowserSettingsRequest, String> {
    let mut input = DisassociateBrowserSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "portalArn" => {
                input.portal_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_data_protection_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateDataProtectionSettingsRequest, String> {
    let mut input = DisassociateDataProtectionSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "portalArn" => {
                input.portal_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_ip_access_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateIpAccessSettingsRequest, String> {
    let mut input = DisassociateIpAccessSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "portalArn" => {
                input.portal_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_network_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateNetworkSettingsRequest, String> {
    let mut input = DisassociateNetworkSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "portalArn" => {
                input.portal_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_session_logger_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateSessionLoggerRequest, String> {
    let mut input = DisassociateSessionLoggerRequest::default();
    for (name, value) in labels {
        match *name {
            "portalArn" => {
                input.portal_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_trust_store_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateTrustStoreRequest, String> {
    let mut input = DisassociateTrustStoreRequest::default();
    for (name, value) in labels {
        match *name {
            "portalArn" => {
                input.portal_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_user_access_logging_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateUserAccessLoggingSettingsRequest, String> {
    let mut input = DisassociateUserAccessLoggingSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "portalArn" => {
                input.portal_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_user_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateUserSettingsRequest, String> {
    let mut input = DisassociateUserSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "portalArn" => {
                input.portal_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_expire_session_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ExpireSessionRequest, String> {
    let mut input = ExpireSessionRequest::default();
    for (name, value) in labels {
        match *name {
            "portalId" => {
                input.portal_id = value.to_string();
            }
            "sessionId" => {
                input.session_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_browser_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBrowserSettingsRequest, String> {
    let mut input = GetBrowserSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "browserSettingsArn" => {
                input.browser_settings_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_data_protection_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDataProtectionSettingsRequest, String> {
    let mut input = GetDataProtectionSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "dataProtectionSettingsArn" => {
                input.data_protection_settings_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_identity_provider_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetIdentityProviderRequest, String> {
    let mut input = GetIdentityProviderRequest::default();
    for (name, value) in labels {
        match *name {
            "identityProviderArn" => {
                input.identity_provider_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_ip_access_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetIpAccessSettingsRequest, String> {
    let mut input = GetIpAccessSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "ipAccessSettingsArn" => {
                input.ip_access_settings_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_network_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetNetworkSettingsRequest, String> {
    let mut input = GetNetworkSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "networkSettingsArn" => {
                input.network_settings_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_portal_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPortalRequest, String> {
    let mut input = GetPortalRequest::default();
    for (name, value) in labels {
        match *name {
            "portalArn" => {
                input.portal_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_portal_service_provider_metadata_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPortalServiceProviderMetadataRequest, String> {
    let mut input = GetPortalServiceProviderMetadataRequest::default();
    for (name, value) in labels {
        match *name {
            "portalArn" => {
                input.portal_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_session_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSessionRequest, String> {
    let mut input = GetSessionRequest::default();
    for (name, value) in labels {
        match *name {
            "portalId" => {
                input.portal_id = value.to_string();
            }
            "sessionId" => {
                input.session_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_session_logger_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSessionLoggerRequest, String> {
    let mut input = GetSessionLoggerRequest::default();
    for (name, value) in labels {
        match *name {
            "sessionLoggerArn" => {
                input.session_logger_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_trust_store_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTrustStoreRequest, String> {
    let mut input = GetTrustStoreRequest::default();
    for (name, value) in labels {
        match *name {
            "trustStoreArn" => {
                input.trust_store_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_trust_store_certificate_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTrustStoreCertificateRequest, String> {
    let mut input = GetTrustStoreCertificateRequest::default();
    for (name, value) in labels {
        match *name {
            "trustStoreArn" => {
                input.trust_store_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("thumbprint") {
        input.thumbprint = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_user_access_logging_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetUserAccessLoggingSettingsRequest, String> {
    let mut input = GetUserAccessLoggingSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "userAccessLoggingSettingsArn" => {
                input.user_access_logging_settings_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_user_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetUserSettingsRequest, String> {
    let mut input = GetUserSettingsRequest::default();
    for (name, value) in labels {
        match *name {
            "userSettingsArn" => {
                input.user_settings_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_browser_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListBrowserSettingsRequest, String> {
    let mut input = ListBrowserSettingsRequest::default();
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
pub fn deserialize_list_data_protection_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDataProtectionSettingsRequest, String> {
    let mut input = ListDataProtectionSettingsRequest::default();
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
pub fn deserialize_list_identity_providers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListIdentityProvidersRequest, String> {
    let mut input = ListIdentityProvidersRequest::default();
    for (name, value) in labels {
        match *name {
            "portalArn" => {
                input.portal_arn = value.to_string();
            }
            _ => {}
        }
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
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_ip_access_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListIpAccessSettingsRequest, String> {
    let mut input = ListIpAccessSettingsRequest::default();
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
pub fn deserialize_list_network_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListNetworkSettingsRequest, String> {
    let mut input = ListNetworkSettingsRequest::default();
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
pub fn deserialize_list_portals_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPortalsRequest, String> {
    let mut input = ListPortalsRequest::default();
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
pub fn deserialize_list_session_loggers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSessionLoggersRequest, String> {
    let mut input = ListSessionLoggersRequest::default();
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
pub fn deserialize_list_sessions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSessionsRequest, String> {
    let mut input = ListSessionsRequest::default();
    for (name, value) in labels {
        match *name {
            "portalId" => {
                input.portal_id = value.to_string();
            }
            _ => {}
        }
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
    if let Some(value) = query.get("sessionId") {
        input.session_id = Some(value.to_string());
    }
    if let Some(value) = query.get("sortBy") {
        input.sort_by = Some(value.to_string());
    }
    if let Some(value) = query.get("status") {
        input.status = Some(value.to_string());
    }
    if let Some(value) = query.get("username") {
        input.username = Some(value.to_string());
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
pub fn deserialize_list_trust_store_certificates_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTrustStoreCertificatesRequest, String> {
    let mut input = ListTrustStoreCertificatesRequest::default();
    for (name, value) in labels {
        match *name {
            "trustStoreArn" => {
                input.trust_store_arn = value.to_string();
            }
            _ => {}
        }
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
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_trust_stores_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTrustStoresRequest, String> {
    let mut input = ListTrustStoresRequest::default();
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
pub fn deserialize_list_user_access_logging_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListUserAccessLoggingSettingsRequest, String> {
    let mut input = ListUserAccessLoggingSettingsRequest::default();
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
pub fn deserialize_list_user_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListUserSettingsRequest, String> {
    let mut input = ListUserSettingsRequest::default();
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

/// Deserialize request for restJson protocol.
pub fn deserialize_update_browser_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateBrowserSettingsRequest, String> {
    let mut input = UpdateBrowserSettingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateBrowserSettingsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateBrowserSettings request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "browserSettingsArn" => {
                input.browser_settings_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_data_protection_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDataProtectionSettingsRequest, String> {
    let mut input = UpdateDataProtectionSettingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDataProtectionSettingsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateDataProtectionSettings request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "dataProtectionSettingsArn" => {
                input.data_protection_settings_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_identity_provider_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateIdentityProviderRequest, String> {
    let mut input = UpdateIdentityProviderRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateIdentityProviderRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateIdentityProvider request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "identityProviderArn" => {
                input.identity_provider_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_ip_access_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateIpAccessSettingsRequest, String> {
    let mut input = UpdateIpAccessSettingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateIpAccessSettingsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateIpAccessSettings request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "ipAccessSettingsArn" => {
                input.ip_access_settings_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_network_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateNetworkSettingsRequest, String> {
    let mut input = UpdateNetworkSettingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateNetworkSettingsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateNetworkSettings request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "networkSettingsArn" => {
                input.network_settings_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_portal_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdatePortalRequest, String> {
    let mut input = UpdatePortalRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdatePortalRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdatePortal request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "portalArn" => {
                input.portal_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_session_logger_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateSessionLoggerRequest, String> {
    let mut input = UpdateSessionLoggerRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateSessionLoggerRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateSessionLogger request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "sessionLoggerArn" => {
                input.session_logger_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_trust_store_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTrustStoreRequest, String> {
    let mut input = UpdateTrustStoreRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateTrustStoreRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateTrustStore request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "trustStoreArn" => {
                input.trust_store_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_user_access_logging_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateUserAccessLoggingSettingsRequest, String> {
    let mut input = UpdateUserAccessLoggingSettingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateUserAccessLoggingSettingsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateUserAccessLoggingSettings request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "userAccessLoggingSettingsArn" => {
                input.user_access_logging_settings_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_user_settings_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateUserSettingsRequest, String> {
    let mut input = UpdateUserSettingsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateUserSettingsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateUserSettings request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "userSettingsArn" => {
                input.user_settings_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
