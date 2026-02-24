//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cognitoidentityprovider

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_add_custom_attributes_response(
    result: &AddCustomAttributesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_add_user_pool_client_secret_response(
    result: &AddUserPoolClientSecretResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_admin_add_user_to_group_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_admin_confirm_sign_up_response(
    result: &AdminConfirmSignUpResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_admin_create_user_response(result: &AdminCreateUserResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_admin_delete_user_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_admin_delete_user_attributes_response(
    result: &AdminDeleteUserAttributesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_admin_disable_provider_for_user_response(
    result: &AdminDisableProviderForUserResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_admin_disable_user_response(result: &AdminDisableUserResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_admin_enable_user_response(result: &AdminEnableUserResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_admin_forget_device_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_admin_get_device_response(result: &AdminGetDeviceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_admin_get_user_response(result: &AdminGetUserResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_admin_initiate_auth_response(result: &AdminInitiateAuthResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_admin_link_provider_for_user_response(
    result: &AdminLinkProviderForUserResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_admin_list_devices_response(result: &AdminListDevicesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_admin_list_groups_for_user_response(
    result: &AdminListGroupsForUserResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_admin_list_user_auth_events_response(
    result: &AdminListUserAuthEventsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_admin_remove_user_from_group_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_admin_reset_user_password_response(
    result: &AdminResetUserPasswordResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_admin_respond_to_auth_challenge_response(
    result: &AdminRespondToAuthChallengeResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_admin_set_user_m_f_a_preference_response(
    result: &AdminSetUserMFAPreferenceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_admin_set_user_password_response(
    result: &AdminSetUserPasswordResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_admin_set_user_settings_response(
    result: &AdminSetUserSettingsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_admin_update_auth_event_feedback_response(
    result: &AdminUpdateAuthEventFeedbackResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_admin_update_device_status_response(
    result: &AdminUpdateDeviceStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_admin_update_user_attributes_response(
    result: &AdminUpdateUserAttributesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_admin_user_global_sign_out_response(
    result: &AdminUserGlobalSignOutResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_software_token_response(
    result: &AssociateSoftwareTokenResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_change_password_response(result: &ChangePasswordResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_complete_web_authn_registration_response(
    result: &CompleteWebAuthnRegistrationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_confirm_device_response(result: &ConfirmDeviceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_confirm_forgot_password_response(
    result: &ConfirmForgotPasswordResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_confirm_sign_up_response(result: &ConfirmSignUpResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_group_response(result: &CreateGroupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_identity_provider_response(
    result: &CreateIdentityProviderResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_managed_login_branding_response(
    result: &CreateManagedLoginBrandingResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_resource_server_response(
    result: &CreateResourceServerResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_terms_response(result: &CreateTermsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_user_import_job_response(
    result: &CreateUserImportJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_user_pool_response(result: &CreateUserPoolResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_user_pool_client_response(
    result: &CreateUserPoolClientResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_user_pool_domain_response(
    result: &CreateUserPoolDomainResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_group_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_identity_provider_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_managed_login_branding_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_resource_server_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_terms_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_user_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_user_attributes_response(
    result: &DeleteUserAttributesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_user_pool_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_user_pool_client_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_user_pool_client_secret_response(
    result: &DeleteUserPoolClientSecretResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_user_pool_domain_response(
    result: &DeleteUserPoolDomainResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_web_authn_credential_response(
    result: &DeleteWebAuthnCredentialResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_identity_provider_response(
    result: &DescribeIdentityProviderResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_managed_login_branding_response(
    result: &DescribeManagedLoginBrandingResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_managed_login_branding_by_client_response(
    result: &DescribeManagedLoginBrandingByClientResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_resource_server_response(
    result: &DescribeResourceServerResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_risk_configuration_response(
    result: &DescribeRiskConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_terms_response(result: &DescribeTermsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_user_import_job_response(
    result: &DescribeUserImportJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_user_pool_response(result: &DescribeUserPoolResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_user_pool_client_response(
    result: &DescribeUserPoolClientResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_user_pool_domain_response(
    result: &DescribeUserPoolDomainResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_forget_device_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_forgot_password_response(result: &ForgotPasswordResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_c_s_v_header_response(result: &GetCSVHeaderResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_device_response(result: &GetDeviceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_group_response(result: &GetGroupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_identity_provider_by_identifier_response(
    result: &GetIdentityProviderByIdentifierResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_log_delivery_configuration_response(
    result: &GetLogDeliveryConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_signing_certificate_response(
    result: &GetSigningCertificateResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_tokens_from_refresh_token_response(
    result: &GetTokensFromRefreshTokenResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_u_i_customization_response(
    result: &GetUICustomizationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_user_response(result: &GetUserResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_user_attribute_verification_code_response(
    result: &GetUserAttributeVerificationCodeResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_user_auth_factors_response(
    result: &GetUserAuthFactorsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_user_pool_mfa_config_response(
    result: &GetUserPoolMfaConfigResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_global_sign_out_response(result: &GlobalSignOutResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_initiate_auth_response(result: &InitiateAuthResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_devices_response(result: &ListDevicesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_groups_response(result: &ListGroupsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_identity_providers_response(
    result: &ListIdentityProvidersResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_resource_servers_response(
    result: &ListResourceServersResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_terms_response(result: &ListTermsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_user_import_jobs_response(
    result: &ListUserImportJobsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_user_pool_client_secrets_response(
    result: &ListUserPoolClientSecretsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_user_pool_clients_response(
    result: &ListUserPoolClientsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_user_pools_response(result: &ListUserPoolsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_users_response(result: &ListUsersResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_users_in_group_response(result: &ListUsersInGroupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_web_authn_credentials_response(
    result: &ListWebAuthnCredentialsResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_resend_confirmation_code_response(
    result: &ResendConfirmationCodeResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_respond_to_auth_challenge_response(
    result: &RespondToAuthChallengeResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_revoke_token_response(result: &RevokeTokenResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_set_log_delivery_configuration_response(
    result: &SetLogDeliveryConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_set_risk_configuration_response(
    result: &SetRiskConfigurationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_set_u_i_customization_response(
    result: &SetUICustomizationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_set_user_m_f_a_preference_response(
    result: &SetUserMFAPreferenceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_set_user_pool_mfa_config_response(
    result: &SetUserPoolMfaConfigResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_set_user_settings_response(result: &SetUserSettingsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_sign_up_response(result: &SignUpResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_user_import_job_response(
    result: &StartUserImportJobResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_web_authn_registration_response(
    result: &StartWebAuthnRegistrationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_user_import_job_response(result: &StopUserImportJobResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_auth_event_feedback_response(
    result: &UpdateAuthEventFeedbackResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_device_status_response(
    result: &UpdateDeviceStatusResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_group_response(result: &UpdateGroupResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_identity_provider_response(
    result: &UpdateIdentityProviderResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_managed_login_branding_response(
    result: &UpdateManagedLoginBrandingResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_resource_server_response(
    result: &UpdateResourceServerResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_terms_response(result: &UpdateTermsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_user_attributes_response(
    result: &UpdateUserAttributesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_user_pool_response(result: &UpdateUserPoolResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_user_pool_client_response(
    result: &UpdateUserPoolClientResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_user_pool_domain_response(
    result: &UpdateUserPoolDomainResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_verify_software_token_response(
    result: &VerifySoftwareTokenResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_verify_user_attribute_response(
    result: &VerifyUserAttributeResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_custom_attributes_request(
    body: &[u8],
) -> Result<AddCustomAttributesRequest, String> {
    if body.is_empty() {
        return Ok(AddCustomAttributesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AddCustomAttributes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_add_user_pool_client_secret_request(
    body: &[u8],
) -> Result<AddUserPoolClientSecretRequest, String> {
    if body.is_empty() {
        return Ok(AddUserPoolClientSecretRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AddUserPoolClientSecret request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_add_user_to_group_request(
    body: &[u8],
) -> Result<AdminAddUserToGroupRequest, String> {
    if body.is_empty() {
        return Ok(AdminAddUserToGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminAddUserToGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_confirm_sign_up_request(
    body: &[u8],
) -> Result<AdminConfirmSignUpRequest, String> {
    if body.is_empty() {
        return Ok(AdminConfirmSignUpRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminConfirmSignUp request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_create_user_request(
    body: &[u8],
) -> Result<AdminCreateUserRequest, String> {
    if body.is_empty() {
        return Ok(AdminCreateUserRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminCreateUser request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_delete_user_request(
    body: &[u8],
) -> Result<AdminDeleteUserRequest, String> {
    if body.is_empty() {
        return Ok(AdminDeleteUserRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminDeleteUser request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_delete_user_attributes_request(
    body: &[u8],
) -> Result<AdminDeleteUserAttributesRequest, String> {
    if body.is_empty() {
        return Ok(AdminDeleteUserAttributesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminDeleteUserAttributes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_disable_provider_for_user_request(
    body: &[u8],
) -> Result<AdminDisableProviderForUserRequest, String> {
    if body.is_empty() {
        return Ok(AdminDisableProviderForUserRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminDisableProviderForUser request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_disable_user_request(
    body: &[u8],
) -> Result<AdminDisableUserRequest, String> {
    if body.is_empty() {
        return Ok(AdminDisableUserRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminDisableUser request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_enable_user_request(
    body: &[u8],
) -> Result<AdminEnableUserRequest, String> {
    if body.is_empty() {
        return Ok(AdminEnableUserRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminEnableUser request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_forget_device_request(
    body: &[u8],
) -> Result<AdminForgetDeviceRequest, String> {
    if body.is_empty() {
        return Ok(AdminForgetDeviceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminForgetDevice request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_get_device_request(body: &[u8]) -> Result<AdminGetDeviceRequest, String> {
    if body.is_empty() {
        return Ok(AdminGetDeviceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminGetDevice request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_get_user_request(body: &[u8]) -> Result<AdminGetUserRequest, String> {
    if body.is_empty() {
        return Ok(AdminGetUserRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminGetUser request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_initiate_auth_request(
    body: &[u8],
) -> Result<AdminInitiateAuthRequest, String> {
    if body.is_empty() {
        return Ok(AdminInitiateAuthRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminInitiateAuth request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_link_provider_for_user_request(
    body: &[u8],
) -> Result<AdminLinkProviderForUserRequest, String> {
    if body.is_empty() {
        return Ok(AdminLinkProviderForUserRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminLinkProviderForUser request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_list_devices_request(
    body: &[u8],
) -> Result<AdminListDevicesRequest, String> {
    if body.is_empty() {
        return Ok(AdminListDevicesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminListDevices request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_list_groups_for_user_request(
    body: &[u8],
) -> Result<AdminListGroupsForUserRequest, String> {
    if body.is_empty() {
        return Ok(AdminListGroupsForUserRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminListGroupsForUser request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_list_user_auth_events_request(
    body: &[u8],
) -> Result<AdminListUserAuthEventsRequest, String> {
    if body.is_empty() {
        return Ok(AdminListUserAuthEventsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminListUserAuthEvents request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_remove_user_from_group_request(
    body: &[u8],
) -> Result<AdminRemoveUserFromGroupRequest, String> {
    if body.is_empty() {
        return Ok(AdminRemoveUserFromGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminRemoveUserFromGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_reset_user_password_request(
    body: &[u8],
) -> Result<AdminResetUserPasswordRequest, String> {
    if body.is_empty() {
        return Ok(AdminResetUserPasswordRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminResetUserPassword request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_respond_to_auth_challenge_request(
    body: &[u8],
) -> Result<AdminRespondToAuthChallengeRequest, String> {
    if body.is_empty() {
        return Ok(AdminRespondToAuthChallengeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminRespondToAuthChallenge request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_set_user_m_f_a_preference_request(
    body: &[u8],
) -> Result<AdminSetUserMFAPreferenceRequest, String> {
    if body.is_empty() {
        return Ok(AdminSetUserMFAPreferenceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminSetUserMFAPreference request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_set_user_password_request(
    body: &[u8],
) -> Result<AdminSetUserPasswordRequest, String> {
    if body.is_empty() {
        return Ok(AdminSetUserPasswordRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminSetUserPassword request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_set_user_settings_request(
    body: &[u8],
) -> Result<AdminSetUserSettingsRequest, String> {
    if body.is_empty() {
        return Ok(AdminSetUserSettingsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminSetUserSettings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_update_auth_event_feedback_request(
    body: &[u8],
) -> Result<AdminUpdateAuthEventFeedbackRequest, String> {
    if body.is_empty() {
        return Ok(AdminUpdateAuthEventFeedbackRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminUpdateAuthEventFeedback request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_update_device_status_request(
    body: &[u8],
) -> Result<AdminUpdateDeviceStatusRequest, String> {
    if body.is_empty() {
        return Ok(AdminUpdateDeviceStatusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminUpdateDeviceStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_update_user_attributes_request(
    body: &[u8],
) -> Result<AdminUpdateUserAttributesRequest, String> {
    if body.is_empty() {
        return Ok(AdminUpdateUserAttributesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminUpdateUserAttributes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_admin_user_global_sign_out_request(
    body: &[u8],
) -> Result<AdminUserGlobalSignOutRequest, String> {
    if body.is_empty() {
        return Ok(AdminUserGlobalSignOutRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AdminUserGlobalSignOut request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_software_token_request(
    body: &[u8],
) -> Result<AssociateSoftwareTokenRequest, String> {
    if body.is_empty() {
        return Ok(AssociateSoftwareTokenRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateSoftwareToken request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_change_password_request(body: &[u8]) -> Result<ChangePasswordRequest, String> {
    if body.is_empty() {
        return Ok(ChangePasswordRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ChangePassword request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_complete_web_authn_registration_request(
    body: &[u8],
) -> Result<CompleteWebAuthnRegistrationRequest, String> {
    if body.is_empty() {
        return Ok(CompleteWebAuthnRegistrationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CompleteWebAuthnRegistration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_confirm_device_request(body: &[u8]) -> Result<ConfirmDeviceRequest, String> {
    if body.is_empty() {
        return Ok(ConfirmDeviceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ConfirmDevice request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_confirm_forgot_password_request(
    body: &[u8],
) -> Result<ConfirmForgotPasswordRequest, String> {
    if body.is_empty() {
        return Ok(ConfirmForgotPasswordRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ConfirmForgotPassword request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_confirm_sign_up_request(body: &[u8]) -> Result<ConfirmSignUpRequest, String> {
    if body.is_empty() {
        return Ok(ConfirmSignUpRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ConfirmSignUp request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_group_request(body: &[u8]) -> Result<CreateGroupRequest, String> {
    if body.is_empty() {
        return Ok(CreateGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_identity_provider_request(
    body: &[u8],
) -> Result<CreateIdentityProviderRequest, String> {
    if body.is_empty() {
        return Ok(CreateIdentityProviderRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateIdentityProvider request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_managed_login_branding_request(
    body: &[u8],
) -> Result<CreateManagedLoginBrandingRequest, String> {
    if body.is_empty() {
        return Ok(CreateManagedLoginBrandingRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateManagedLoginBranding request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_resource_server_request(
    body: &[u8],
) -> Result<CreateResourceServerRequest, String> {
    if body.is_empty() {
        return Ok(CreateResourceServerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateResourceServer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_terms_request(body: &[u8]) -> Result<CreateTermsRequest, String> {
    if body.is_empty() {
        return Ok(CreateTermsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateTerms request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_user_import_job_request(
    body: &[u8],
) -> Result<CreateUserImportJobRequest, String> {
    if body.is_empty() {
        return Ok(CreateUserImportJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateUserImportJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_user_pool_request(body: &[u8]) -> Result<CreateUserPoolRequest, String> {
    if body.is_empty() {
        return Ok(CreateUserPoolRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateUserPool request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_user_pool_client_request(
    body: &[u8],
) -> Result<CreateUserPoolClientRequest, String> {
    if body.is_empty() {
        return Ok(CreateUserPoolClientRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateUserPoolClient request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_user_pool_domain_request(
    body: &[u8],
) -> Result<CreateUserPoolDomainRequest, String> {
    if body.is_empty() {
        return Ok(CreateUserPoolDomainRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateUserPoolDomain request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_group_request(body: &[u8]) -> Result<DeleteGroupRequest, String> {
    if body.is_empty() {
        return Ok(DeleteGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_identity_provider_request(
    body: &[u8],
) -> Result<DeleteIdentityProviderRequest, String> {
    if body.is_empty() {
        return Ok(DeleteIdentityProviderRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteIdentityProvider request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_managed_login_branding_request(
    body: &[u8],
) -> Result<DeleteManagedLoginBrandingRequest, String> {
    if body.is_empty() {
        return Ok(DeleteManagedLoginBrandingRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteManagedLoginBranding request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_resource_server_request(
    body: &[u8],
) -> Result<DeleteResourceServerRequest, String> {
    if body.is_empty() {
        return Ok(DeleteResourceServerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteResourceServer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_terms_request(body: &[u8]) -> Result<DeleteTermsRequest, String> {
    if body.is_empty() {
        return Ok(DeleteTermsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteTerms request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_user_request(body: &[u8]) -> Result<DeleteUserRequest, String> {
    if body.is_empty() {
        return Ok(DeleteUserRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteUser request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_user_attributes_request(
    body: &[u8],
) -> Result<DeleteUserAttributesRequest, String> {
    if body.is_empty() {
        return Ok(DeleteUserAttributesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteUserAttributes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_user_pool_request(body: &[u8]) -> Result<DeleteUserPoolRequest, String> {
    if body.is_empty() {
        return Ok(DeleteUserPoolRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteUserPool request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_user_pool_client_request(
    body: &[u8],
) -> Result<DeleteUserPoolClientRequest, String> {
    if body.is_empty() {
        return Ok(DeleteUserPoolClientRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteUserPoolClient request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_user_pool_client_secret_request(
    body: &[u8],
) -> Result<DeleteUserPoolClientSecretRequest, String> {
    if body.is_empty() {
        return Ok(DeleteUserPoolClientSecretRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteUserPoolClientSecret request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_user_pool_domain_request(
    body: &[u8],
) -> Result<DeleteUserPoolDomainRequest, String> {
    if body.is_empty() {
        return Ok(DeleteUserPoolDomainRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteUserPoolDomain request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_web_authn_credential_request(
    body: &[u8],
) -> Result<DeleteWebAuthnCredentialRequest, String> {
    if body.is_empty() {
        return Ok(DeleteWebAuthnCredentialRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteWebAuthnCredential request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_identity_provider_request(
    body: &[u8],
) -> Result<DescribeIdentityProviderRequest, String> {
    if body.is_empty() {
        return Ok(DescribeIdentityProviderRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeIdentityProvider request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_managed_login_branding_request(
    body: &[u8],
) -> Result<DescribeManagedLoginBrandingRequest, String> {
    if body.is_empty() {
        return Ok(DescribeManagedLoginBrandingRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeManagedLoginBranding request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_managed_login_branding_by_client_request(
    body: &[u8],
) -> Result<DescribeManagedLoginBrandingByClientRequest, String> {
    if body.is_empty() {
        return Ok(DescribeManagedLoginBrandingByClientRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeManagedLoginBrandingByClient request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_resource_server_request(
    body: &[u8],
) -> Result<DescribeResourceServerRequest, String> {
    if body.is_empty() {
        return Ok(DescribeResourceServerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeResourceServer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_risk_configuration_request(
    body: &[u8],
) -> Result<DescribeRiskConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(DescribeRiskConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeRiskConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_terms_request(body: &[u8]) -> Result<DescribeTermsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeTermsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeTerms request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_user_import_job_request(
    body: &[u8],
) -> Result<DescribeUserImportJobRequest, String> {
    if body.is_empty() {
        return Ok(DescribeUserImportJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeUserImportJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_user_pool_request(
    body: &[u8],
) -> Result<DescribeUserPoolRequest, String> {
    if body.is_empty() {
        return Ok(DescribeUserPoolRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeUserPool request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_user_pool_client_request(
    body: &[u8],
) -> Result<DescribeUserPoolClientRequest, String> {
    if body.is_empty() {
        return Ok(DescribeUserPoolClientRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeUserPoolClient request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_user_pool_domain_request(
    body: &[u8],
) -> Result<DescribeUserPoolDomainRequest, String> {
    if body.is_empty() {
        return Ok(DescribeUserPoolDomainRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeUserPoolDomain request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_forget_device_request(body: &[u8]) -> Result<ForgetDeviceRequest, String> {
    if body.is_empty() {
        return Ok(ForgetDeviceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ForgetDevice request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_forgot_password_request(body: &[u8]) -> Result<ForgotPasswordRequest, String> {
    if body.is_empty() {
        return Ok(ForgotPasswordRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ForgotPassword request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_c_s_v_header_request(body: &[u8]) -> Result<GetCSVHeaderRequest, String> {
    if body.is_empty() {
        return Ok(GetCSVHeaderRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCSVHeader request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_device_request(body: &[u8]) -> Result<GetDeviceRequest, String> {
    if body.is_empty() {
        return Ok(GetDeviceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetDevice request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_group_request(body: &[u8]) -> Result<GetGroupRequest, String> {
    if body.is_empty() {
        return Ok(GetGroupRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize GetGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_identity_provider_by_identifier_request(
    body: &[u8],
) -> Result<GetIdentityProviderByIdentifierRequest, String> {
    if body.is_empty() {
        return Ok(GetIdentityProviderByIdentifierRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetIdentityProviderByIdentifier request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_log_delivery_configuration_request(
    body: &[u8],
) -> Result<GetLogDeliveryConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(GetLogDeliveryConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetLogDeliveryConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_signing_certificate_request(
    body: &[u8],
) -> Result<GetSigningCertificateRequest, String> {
    if body.is_empty() {
        return Ok(GetSigningCertificateRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetSigningCertificate request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_tokens_from_refresh_token_request(
    body: &[u8],
) -> Result<GetTokensFromRefreshTokenRequest, String> {
    if body.is_empty() {
        return Ok(GetTokensFromRefreshTokenRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetTokensFromRefreshToken request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_u_i_customization_request(
    body: &[u8],
) -> Result<GetUICustomizationRequest, String> {
    if body.is_empty() {
        return Ok(GetUICustomizationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetUICustomization request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_user_request(body: &[u8]) -> Result<GetUserRequest, String> {
    if body.is_empty() {
        return Ok(GetUserRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize GetUser request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_user_attribute_verification_code_request(
    body: &[u8],
) -> Result<GetUserAttributeVerificationCodeRequest, String> {
    if body.is_empty() {
        return Ok(GetUserAttributeVerificationCodeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetUserAttributeVerificationCode request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_user_auth_factors_request(
    body: &[u8],
) -> Result<GetUserAuthFactorsRequest, String> {
    if body.is_empty() {
        return Ok(GetUserAuthFactorsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetUserAuthFactors request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_user_pool_mfa_config_request(
    body: &[u8],
) -> Result<GetUserPoolMfaConfigRequest, String> {
    if body.is_empty() {
        return Ok(GetUserPoolMfaConfigRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetUserPoolMfaConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_global_sign_out_request(body: &[u8]) -> Result<GlobalSignOutRequest, String> {
    if body.is_empty() {
        return Ok(GlobalSignOutRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GlobalSignOut request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_initiate_auth_request(body: &[u8]) -> Result<InitiateAuthRequest, String> {
    if body.is_empty() {
        return Ok(InitiateAuthRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize InitiateAuth request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_devices_request(body: &[u8]) -> Result<ListDevicesRequest, String> {
    if body.is_empty() {
        return Ok(ListDevicesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListDevices request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_groups_request(body: &[u8]) -> Result<ListGroupsRequest, String> {
    if body.is_empty() {
        return Ok(ListGroupsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListGroups request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_identity_providers_request(
    body: &[u8],
) -> Result<ListIdentityProvidersRequest, String> {
    if body.is_empty() {
        return Ok(ListIdentityProvidersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListIdentityProviders request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_resource_servers_request(
    body: &[u8],
) -> Result<ListResourceServersRequest, String> {
    if body.is_empty() {
        return Ok(ListResourceServersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListResourceServers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    body: &[u8],
) -> Result<ListTagsForResourceRequest, String> {
    if body.is_empty() {
        return Ok(ListTagsForResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTagsForResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_terms_request(body: &[u8]) -> Result<ListTermsRequest, String> {
    if body.is_empty() {
        return Ok(ListTermsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTerms request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_user_import_jobs_request(
    body: &[u8],
) -> Result<ListUserImportJobsRequest, String> {
    if body.is_empty() {
        return Ok(ListUserImportJobsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListUserImportJobs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_user_pool_client_secrets_request(
    body: &[u8],
) -> Result<ListUserPoolClientSecretsRequest, String> {
    if body.is_empty() {
        return Ok(ListUserPoolClientSecretsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListUserPoolClientSecrets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_user_pool_clients_request(
    body: &[u8],
) -> Result<ListUserPoolClientsRequest, String> {
    if body.is_empty() {
        return Ok(ListUserPoolClientsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListUserPoolClients request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_user_pools_request(body: &[u8]) -> Result<ListUserPoolsRequest, String> {
    if body.is_empty() {
        return Ok(ListUserPoolsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListUserPools request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_users_request(body: &[u8]) -> Result<ListUsersRequest, String> {
    if body.is_empty() {
        return Ok(ListUsersRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListUsers request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_users_in_group_request(
    body: &[u8],
) -> Result<ListUsersInGroupRequest, String> {
    if body.is_empty() {
        return Ok(ListUsersInGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListUsersInGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_web_authn_credentials_request(
    body: &[u8],
) -> Result<ListWebAuthnCredentialsRequest, String> {
    if body.is_empty() {
        return Ok(ListWebAuthnCredentialsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListWebAuthnCredentials request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_resend_confirmation_code_request(
    body: &[u8],
) -> Result<ResendConfirmationCodeRequest, String> {
    if body.is_empty() {
        return Ok(ResendConfirmationCodeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ResendConfirmationCode request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_respond_to_auth_challenge_request(
    body: &[u8],
) -> Result<RespondToAuthChallengeRequest, String> {
    if body.is_empty() {
        return Ok(RespondToAuthChallengeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RespondToAuthChallenge request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_revoke_token_request(body: &[u8]) -> Result<RevokeTokenRequest, String> {
    if body.is_empty() {
        return Ok(RevokeTokenRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RevokeToken request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_set_log_delivery_configuration_request(
    body: &[u8],
) -> Result<SetLogDeliveryConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(SetLogDeliveryConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SetLogDeliveryConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_set_risk_configuration_request(
    body: &[u8],
) -> Result<SetRiskConfigurationRequest, String> {
    if body.is_empty() {
        return Ok(SetRiskConfigurationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SetRiskConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_set_u_i_customization_request(
    body: &[u8],
) -> Result<SetUICustomizationRequest, String> {
    if body.is_empty() {
        return Ok(SetUICustomizationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SetUICustomization request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_set_user_m_f_a_preference_request(
    body: &[u8],
) -> Result<SetUserMFAPreferenceRequest, String> {
    if body.is_empty() {
        return Ok(SetUserMFAPreferenceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SetUserMFAPreference request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_set_user_pool_mfa_config_request(
    body: &[u8],
) -> Result<SetUserPoolMfaConfigRequest, String> {
    if body.is_empty() {
        return Ok(SetUserPoolMfaConfigRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SetUserPoolMfaConfig request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_set_user_settings_request(
    body: &[u8],
) -> Result<SetUserSettingsRequest, String> {
    if body.is_empty() {
        return Ok(SetUserSettingsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SetUserSettings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_sign_up_request(body: &[u8]) -> Result<SignUpRequest, String> {
    if body.is_empty() {
        return Ok(SignUpRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize SignUp request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_user_import_job_request(
    body: &[u8],
) -> Result<StartUserImportJobRequest, String> {
    if body.is_empty() {
        return Ok(StartUserImportJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartUserImportJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_web_authn_registration_request(
    body: &[u8],
) -> Result<StartWebAuthnRegistrationRequest, String> {
    if body.is_empty() {
        return Ok(StartWebAuthnRegistrationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartWebAuthnRegistration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_stop_user_import_job_request(
    body: &[u8],
) -> Result<StopUserImportJobRequest, String> {
    if body.is_empty() {
        return Ok(StopUserImportJobRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StopUserImportJob request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_tag_resource_request(body: &[u8]) -> Result<TagResourceRequest, String> {
    if body.is_empty() {
        return Ok(TagResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_untag_resource_request(body: &[u8]) -> Result<UntagResourceRequest, String> {
    if body.is_empty() {
        return Ok(UntagResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UntagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_auth_event_feedback_request(
    body: &[u8],
) -> Result<UpdateAuthEventFeedbackRequest, String> {
    if body.is_empty() {
        return Ok(UpdateAuthEventFeedbackRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateAuthEventFeedback request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_device_status_request(
    body: &[u8],
) -> Result<UpdateDeviceStatusRequest, String> {
    if body.is_empty() {
        return Ok(UpdateDeviceStatusRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateDeviceStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_group_request(body: &[u8]) -> Result<UpdateGroupRequest, String> {
    if body.is_empty() {
        return Ok(UpdateGroupRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateGroup request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_identity_provider_request(
    body: &[u8],
) -> Result<UpdateIdentityProviderRequest, String> {
    if body.is_empty() {
        return Ok(UpdateIdentityProviderRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateIdentityProvider request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_managed_login_branding_request(
    body: &[u8],
) -> Result<UpdateManagedLoginBrandingRequest, String> {
    if body.is_empty() {
        return Ok(UpdateManagedLoginBrandingRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateManagedLoginBranding request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_resource_server_request(
    body: &[u8],
) -> Result<UpdateResourceServerRequest, String> {
    if body.is_empty() {
        return Ok(UpdateResourceServerRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateResourceServer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_terms_request(body: &[u8]) -> Result<UpdateTermsRequest, String> {
    if body.is_empty() {
        return Ok(UpdateTermsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateTerms request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_user_attributes_request(
    body: &[u8],
) -> Result<UpdateUserAttributesRequest, String> {
    if body.is_empty() {
        return Ok(UpdateUserAttributesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateUserAttributes request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_user_pool_request(body: &[u8]) -> Result<UpdateUserPoolRequest, String> {
    if body.is_empty() {
        return Ok(UpdateUserPoolRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateUserPool request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_user_pool_client_request(
    body: &[u8],
) -> Result<UpdateUserPoolClientRequest, String> {
    if body.is_empty() {
        return Ok(UpdateUserPoolClientRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateUserPoolClient request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_user_pool_domain_request(
    body: &[u8],
) -> Result<UpdateUserPoolDomainRequest, String> {
    if body.is_empty() {
        return Ok(UpdateUserPoolDomainRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateUserPoolDomain request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_verify_software_token_request(
    body: &[u8],
) -> Result<VerifySoftwareTokenRequest, String> {
    if body.is_empty() {
        return Ok(VerifySoftwareTokenRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize VerifySoftwareToken request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_verify_user_attribute_request(
    body: &[u8],
) -> Result<VerifyUserAttributeRequest, String> {
    if body.is_empty() {
        return Ok(VerifyUserAttributeRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize VerifyUserAttribute request: {e}"))
}
