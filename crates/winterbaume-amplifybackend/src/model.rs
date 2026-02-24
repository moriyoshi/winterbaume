//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-amplifybackend

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloneBackendRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "BackendEnvironmentName")]
    #[serde(default)]
    pub backend_environment_name: String,
    #[serde(rename = "targetEnvironmentName")]
    #[serde(default)]
    pub target_environment_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloneBackendResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackendAPIRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    pub backend_environment_name: String,
    #[serde(rename = "resourceConfig")]
    #[serde(default)]
    pub resource_config: BackendAPIResourceConfig,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    pub resource_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackendAPIResourceConfig {
    #[serde(rename = "additionalAuthTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_auth_types: Option<Vec<BackendAPIAuthType>>,
    #[serde(rename = "apiName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
    #[serde(rename = "conflictResolution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolution: Option<BackendAPIConflictResolution>,
    #[serde(rename = "defaultAuthType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_auth_type: Option<BackendAPIAuthType>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(rename = "transformSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_schema: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackendAPIAuthType {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<BackendAPIAppSyncAuthSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackendAPIAppSyncAuthSettings {
    #[serde(rename = "cognitoUserPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_user_pool_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "expirationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<f64>,
    #[serde(rename = "openIDAuthTTL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_i_d_auth_t_t_l: Option<String>,
    #[serde(rename = "openIDClientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_i_d_client_id: Option<String>,
    #[serde(rename = "openIDIatTTL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_i_d_iat_t_t_l: Option<String>,
    #[serde(rename = "openIDIssueURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_i_d_issue_u_r_l: Option<String>,
    #[serde(rename = "openIDProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_i_d_provider_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackendAPIConflictResolution {
    #[serde(rename = "resolutionStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_strategy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackendAPIResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackendAuthRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    pub backend_environment_name: String,
    #[serde(rename = "resourceConfig")]
    #[serde(default)]
    pub resource_config: CreateBackendAuthResourceConfig,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    pub resource_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackendAuthResourceConfig {
    #[serde(rename = "authResources")]
    #[serde(default)]
    pub auth_resources: String,
    #[serde(rename = "identityPoolConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_configs: Option<CreateBackendAuthIdentityPoolConfig>,
    #[serde(default)]
    pub service: String,
    #[serde(rename = "userPoolConfigs")]
    #[serde(default)]
    pub user_pool_configs: CreateBackendAuthUserPoolConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackendAuthIdentityPoolConfig {
    #[serde(rename = "identityPoolName")]
    #[serde(default)]
    pub identity_pool_name: String,
    #[serde(rename = "unauthenticatedLogin")]
    #[serde(default)]
    pub unauthenticated_login: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackendAuthUserPoolConfig {
    #[serde(rename = "forgotPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forgot_password: Option<CreateBackendAuthForgotPasswordConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa: Option<CreateBackendAuthMFAConfig>,
    #[serde(rename = "oAuth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth: Option<CreateBackendAuthOAuthConfig>,
    #[serde(rename = "passwordPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_policy: Option<CreateBackendAuthPasswordPolicyConfig>,
    #[serde(rename = "requiredSignUpAttributes")]
    #[serde(default)]
    pub required_sign_up_attributes: Vec<String>,
    #[serde(rename = "signInMethod")]
    #[serde(default)]
    pub sign_in_method: String,
    #[serde(rename = "userPoolName")]
    #[serde(default)]
    pub user_pool_name: String,
    #[serde(rename = "verificationMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_message: Option<CreateBackendAuthVerificationMessageConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackendAuthForgotPasswordConfig {
    #[serde(rename = "deliveryMethod")]
    #[serde(default)]
    pub delivery_method: String,
    #[serde(rename = "emailSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_settings: Option<EmailSettings>,
    #[serde(rename = "smsSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_settings: Option<SmsSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailSettings {
    #[serde(rename = "emailMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_message: Option<String>,
    #[serde(rename = "emailSubject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_subject: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SmsSettings {
    #[serde(rename = "smsMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackendAuthMFAConfig {
    #[serde(rename = "MFAMode")]
    #[serde(default)]
    pub m_f_a_mode: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Settings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Settings {
    #[serde(rename = "mfaTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_types: Option<Vec<String>>,
    #[serde(rename = "smsMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackendAuthOAuthConfig {
    #[serde(rename = "domainPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_prefix: Option<String>,
    #[serde(rename = "oAuthGrantType")]
    #[serde(default)]
    pub o_auth_grant_type: String,
    #[serde(rename = "oAuthScopes")]
    #[serde(default)]
    pub o_auth_scopes: Vec<String>,
    #[serde(rename = "redirectSignInURIs")]
    #[serde(default)]
    pub redirect_sign_in_u_r_is: Vec<String>,
    #[serde(rename = "redirectSignOutURIs")]
    #[serde(default)]
    pub redirect_sign_out_u_r_is: Vec<String>,
    #[serde(rename = "socialProviderSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_provider_settings: Option<SocialProviderSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SocialProviderSettings {
    #[serde(rename = "Facebook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook: Option<BackendAuthSocialProviderConfig>,
    #[serde(rename = "Google")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google: Option<BackendAuthSocialProviderConfig>,
    #[serde(rename = "LoginWithAmazon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_with_amazon: Option<BackendAuthSocialProviderConfig>,
    #[serde(rename = "SignInWithApple")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_in_with_apple: Option<BackendAuthAppleProviderConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackendAuthSocialProviderConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackendAuthAppleProviderConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackendAuthPasswordPolicyConfig {
    #[serde(rename = "additionalConstraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_constraints: Option<Vec<String>>,
    #[serde(rename = "minimumLength")]
    #[serde(default)]
    pub minimum_length: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackendAuthVerificationMessageConfig {
    #[serde(rename = "deliveryMethod")]
    #[serde(default)]
    pub delivery_method: String,
    #[serde(rename = "emailSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_settings: Option<EmailSettings>,
    #[serde(rename = "smsSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_settings: Option<SmsSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackendAuthResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackendConfigRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "backendManagerAppId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_manager_app_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackendConfigResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackendRequest {
    #[serde(rename = "appId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "appName")]
    #[serde(default)]
    pub app_name: String,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    pub backend_environment_name: String,
    #[serde(rename = "resourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_config: Option<ResourceConfig>,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceConfig {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackendResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackendStorageRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    pub backend_environment_name: String,
    #[serde(rename = "resourceConfig")]
    #[serde(default)]
    pub resource_config: CreateBackendStorageResourceConfig,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    pub resource_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackendStorageResourceConfig {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(default)]
    pub permissions: BackendStoragePermissions,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    pub service_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackendStoragePermissions {
    #[serde(default)]
    pub authenticated: Vec<String>,
    #[serde(rename = "unAuthenticated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub un_authenticated: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackendStorageResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTokenRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTokenResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "challengeCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_code: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBackendAPIRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "BackendEnvironmentName")]
    #[serde(default)]
    pub backend_environment_name: String,
    #[serde(rename = "resourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_config: Option<BackendAPIResourceConfig>,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    pub resource_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBackendAPIResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBackendAuthRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "BackendEnvironmentName")]
    #[serde(default)]
    pub backend_environment_name: String,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    pub resource_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBackendAuthResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBackendRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "BackendEnvironmentName")]
    #[serde(default)]
    pub backend_environment_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBackendResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBackendStorageRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "BackendEnvironmentName")]
    #[serde(default)]
    pub backend_environment_name: String,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    pub resource_name: String,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    pub service_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBackendStorageResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTokenRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTokenResponse {
    #[serde(rename = "isSuccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateBackendAPIModelsRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "BackendEnvironmentName")]
    #[serde(default)]
    pub backend_environment_name: String,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    pub resource_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateBackendAPIModelsResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackendAPIModelsRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "BackendEnvironmentName")]
    #[serde(default)]
    pub backend_environment_name: String,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    pub resource_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackendAPIModelsResponse {
    #[serde(rename = "modelIntrospectionSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_introspection_schema: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackendAPIRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "BackendEnvironmentName")]
    #[serde(default)]
    pub backend_environment_name: String,
    #[serde(rename = "resourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_config: Option<BackendAPIResourceConfig>,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    pub resource_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackendAPIResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "resourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_config: Option<BackendAPIResourceConfig>,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackendAuthRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "BackendEnvironmentName")]
    #[serde(default)]
    pub backend_environment_name: String,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    pub resource_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackendAuthResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "resourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_config: Option<CreateBackendAuthResourceConfig>,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackendJobRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "BackendEnvironmentName")]
    #[serde(default)]
    pub backend_environment_name: String,
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackendJobResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
    #[serde(rename = "createTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackendRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackendResponse {
    #[serde(rename = "amplifyFeatureFlags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amplify_feature_flags: Option<String>,
    #[serde(rename = "amplifyMetaConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amplify_meta_config: Option<String>,
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "appName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(rename = "backendEnvironmentList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_list: Option<Vec<String>>,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackendStorageRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "BackendEnvironmentName")]
    #[serde(default)]
    pub backend_environment_name: String,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    pub resource_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackendStorageResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
    #[serde(rename = "resourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_config: Option<GetBackendStorageResourceConfig>,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBackendStorageResourceConfig {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<BackendStoragePermissions>,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTokenRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTokenResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "challengeCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_code: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportBackendAuthRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "BackendEnvironmentName")]
    #[serde(default)]
    pub backend_environment_name: String,
    #[serde(rename = "identityPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<String>,
    #[serde(rename = "nativeClientId")]
    #[serde(default)]
    pub native_client_id: String,
    #[serde(rename = "userPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
    #[serde(rename = "webClientId")]
    #[serde(default)]
    pub web_client_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportBackendAuthResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportBackendStorageRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "BackendEnvironmentName")]
    #[serde(default)]
    pub backend_environment_name: String,
    #[serde(rename = "bucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    pub service_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportBackendStorageResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBackendJobsRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "BackendEnvironmentName")]
    #[serde(default)]
    pub backend_environment_name: String,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBackendJobsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<BackendJobRespObj>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackendJobRespObj {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
    #[serde(rename = "createTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListS3BucketsRequest {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListS3BucketsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Vec<S3BucketInfo>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3BucketInfo {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveAllBackendsRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "cleanAmplifyApp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clean_amplify_app: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveAllBackendsResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveBackendConfigRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveBackendConfigResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBackendAPIRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "BackendEnvironmentName")]
    #[serde(default)]
    pub backend_environment_name: String,
    #[serde(rename = "resourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_config: Option<BackendAPIResourceConfig>,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    pub resource_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBackendAPIResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBackendAuthRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "BackendEnvironmentName")]
    #[serde(default)]
    pub backend_environment_name: String,
    #[serde(rename = "resourceConfig")]
    #[serde(default)]
    pub resource_config: UpdateBackendAuthResourceConfig,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    pub resource_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBackendAuthResourceConfig {
    #[serde(rename = "authResources")]
    #[serde(default)]
    pub auth_resources: String,
    #[serde(rename = "identityPoolConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_configs: Option<UpdateBackendAuthIdentityPoolConfig>,
    #[serde(default)]
    pub service: String,
    #[serde(rename = "userPoolConfigs")]
    #[serde(default)]
    pub user_pool_configs: UpdateBackendAuthUserPoolConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBackendAuthIdentityPoolConfig {
    #[serde(rename = "unauthenticatedLogin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unauthenticated_login: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBackendAuthUserPoolConfig {
    #[serde(rename = "forgotPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forgot_password: Option<UpdateBackendAuthForgotPasswordConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa: Option<UpdateBackendAuthMFAConfig>,
    #[serde(rename = "oAuth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth: Option<UpdateBackendAuthOAuthConfig>,
    #[serde(rename = "passwordPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_policy: Option<UpdateBackendAuthPasswordPolicyConfig>,
    #[serde(rename = "verificationMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_message: Option<UpdateBackendAuthVerificationMessageConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBackendAuthForgotPasswordConfig {
    #[serde(rename = "deliveryMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_method: Option<String>,
    #[serde(rename = "emailSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_settings: Option<EmailSettings>,
    #[serde(rename = "smsSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_settings: Option<SmsSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBackendAuthMFAConfig {
    #[serde(rename = "MFAMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_f_a_mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Settings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBackendAuthOAuthConfig {
    #[serde(rename = "domainPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_prefix: Option<String>,
    #[serde(rename = "oAuthGrantType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_grant_type: Option<String>,
    #[serde(rename = "oAuthScopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_scopes: Option<Vec<String>>,
    #[serde(rename = "redirectSignInURIs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_sign_in_u_r_is: Option<Vec<String>>,
    #[serde(rename = "redirectSignOutURIs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_sign_out_u_r_is: Option<Vec<String>>,
    #[serde(rename = "socialProviderSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_provider_settings: Option<SocialProviderSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBackendAuthPasswordPolicyConfig {
    #[serde(rename = "additionalConstraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_constraints: Option<Vec<String>>,
    #[serde(rename = "minimumLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBackendAuthVerificationMessageConfig {
    #[serde(rename = "deliveryMethod")]
    #[serde(default)]
    pub delivery_method: String,
    #[serde(rename = "emailSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_settings: Option<EmailSettings>,
    #[serde(rename = "smsSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_settings: Option<SmsSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBackendAuthResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBackendConfigRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "loginAuthConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_auth_config: Option<LoginAuthConfigReqObj>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoginAuthConfigReqObj {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_cognito_identity_pool_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_cognito_region: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_user_pools_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_user_pools_web_client_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBackendConfigResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "backendManagerAppId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_manager_app_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "loginAuthConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_auth_config: Option<LoginAuthConfigReqObj>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBackendJobRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "BackendEnvironmentName")]
    #[serde(default)]
    pub backend_environment_name: String,
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBackendJobResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
    #[serde(rename = "createTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBackendStorageRequest {
    #[serde(rename = "AppId")]
    #[serde(default)]
    pub app_id: String,
    #[serde(rename = "BackendEnvironmentName")]
    #[serde(default)]
    pub backend_environment_name: String,
    #[serde(rename = "resourceConfig")]
    #[serde(default)]
    pub resource_config: UpdateBackendStorageResourceConfig,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    pub resource_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBackendStorageResourceConfig {
    #[serde(default)]
    pub permissions: BackendStoragePermissions,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    pub service_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBackendStorageResponse {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "backendEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_environment_name: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
