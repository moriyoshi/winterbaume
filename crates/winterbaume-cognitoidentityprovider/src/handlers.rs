use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{CognitoIdpError, CognitoIdpState};
use crate::views::CognitoidpStateView;
use crate::wire;

pub struct CognitoIdentityProviderService {
    pub(crate) state: Arc<BackendState<CognitoIdpState>>,
    pub(crate) notifier: StateChangeNotifier<CognitoidpStateView>,
}

impl CognitoIdentityProviderService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CognitoIdentityProviderService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CognitoIdentityProviderService {
    fn service_name(&self) -> &str {
        "cognito-idp"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://cognito-idp\..*\.amazonaws\.com",
            r"https?://cognito-idp\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl CognitoIdentityProviderService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').next_back())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            // User Pool operations
            "CreateUserPool" => {
                self.handle_create_user_pool(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeUserPool" => self.handle_describe_user_pool(&state, body_bytes).await,
            "UpdateUserPool" => self.handle_update_user_pool(&state, body_bytes).await,
            "DeleteUserPool" => self.handle_delete_user_pool(&state, body_bytes).await,
            "ListUserPools" => self.handle_list_user_pools(&state).await,

            // User Pool Client operations
            "CreateUserPoolClient" => {
                self.handle_create_user_pool_client(&state, body_bytes)
                    .await
            }
            "DescribeUserPoolClient" => {
                self.handle_describe_user_pool_client(&state, body_bytes)
                    .await
            }
            "UpdateUserPoolClient" => {
                self.handle_update_user_pool_client(&state, body_bytes)
                    .await
            }
            "DeleteUserPoolClient" => {
                self.handle_delete_user_pool_client(&state, body_bytes)
                    .await
            }
            "ListUserPoolClients" => self.handle_list_user_pool_clients(&state, body_bytes).await,

            // User Pool Domain operations
            "CreateUserPoolDomain" => {
                self.handle_create_user_pool_domain(&state, body_bytes)
                    .await
            }
            "DescribeUserPoolDomain" => {
                self.handle_describe_user_pool_domain(&state, body_bytes)
                    .await
            }
            "UpdateUserPoolDomain" => {
                self.handle_update_user_pool_domain(&state, body_bytes)
                    .await
            }
            "DeleteUserPoolDomain" => {
                self.handle_delete_user_pool_domain(&state, body_bytes)
                    .await
            }

            // Admin user operations
            "AdminCreateUser" => self.handle_admin_create_user(&state, body_bytes).await,
            "AdminGetUser" => self.handle_admin_get_user(&state, body_bytes).await,
            "AdminDeleteUser" => self.handle_admin_delete_user(&state, body_bytes).await,
            "AdminDeleteUserAttributes" => {
                self.handle_admin_delete_user_attributes(&state, body_bytes)
                    .await
            }
            "AdminUpdateUserAttributes" => {
                self.handle_admin_update_user_attributes(&state, body_bytes)
                    .await
            }
            "AdminEnableUser" => self.handle_admin_enable_user(&state, body_bytes).await,
            "AdminDisableUser" => self.handle_admin_disable_user(&state, body_bytes).await,
            "AdminSetUserPassword" => {
                self.handle_admin_set_user_password(&state, body_bytes)
                    .await
            }
            "AdminConfirmSignUp" => self.handle_admin_confirm_sign_up(&state, body_bytes).await,
            "AdminResetUserPassword" => {
                self.handle_admin_reset_user_password(&state, body_bytes)
                    .await
            }
            "AdminSetUserMFAPreference" => {
                self.handle_admin_set_user_mfa_preference(&state, body_bytes)
                    .await
            }

            // Admin group operations
            "AdminAddUserToGroup" => {
                self.handle_admin_add_user_to_group(&state, body_bytes)
                    .await
            }
            "AdminRemoveUserFromGroup" => {
                self.handle_admin_remove_user_from_group(&state, body_bytes)
                    .await
            }
            "AdminListGroupsForUser" => {
                self.handle_admin_list_groups_for_user(&state, body_bytes)
                    .await
            }

            // Admin auth operations
            "AdminInitiateAuth" => self.handle_admin_initiate_auth(&state, body_bytes).await,
            "AdminRespondToAuthChallenge" => {
                self.handle_admin_respond_to_auth_challenge(&state, body_bytes)
                    .await
            }
            "AdminUserGlobalSignOut" => {
                self.handle_admin_user_global_sign_out(&state, body_bytes)
                    .await
            }

            // Sign up / confirm operations
            "SignUp" => self.handle_sign_up(&state, body_bytes).await,
            "ConfirmSignUp" => self.handle_confirm_sign_up(&state, body_bytes).await,
            "ForgotPassword" => self.handle_forgot_password(&state, body_bytes).await,
            "ConfirmForgotPassword" => {
                self.handle_confirm_forgot_password(&state, body_bytes)
                    .await
            }

            // Auth operations
            "InitiateAuth" => self.handle_initiate_auth(&state, body_bytes).await,
            "RespondToAuthChallenge" => {
                self.handle_respond_to_auth_challenge(&state, body_bytes)
                    .await
            }
            "GlobalSignOut" => self.handle_global_sign_out(&state).await,
            "ChangePassword" => self.handle_change_password(&state, body_bytes).await,

            // User self-service operations
            "GetUser" => self.handle_get_user(&state, body_bytes).await,
            "UpdateUserAttributes" => self.handle_update_user_attributes(&state, body_bytes).await,
            "SetUserMFAPreference" => self.handle_set_user_mfa_preference(&state).await,
            "ListUsers" => self.handle_list_users(&state, body_bytes).await,
            "ListUsersInGroup" => self.handle_list_users_in_group(&state, body_bytes).await,

            // Group operations
            "CreateGroup" => self.handle_create_group(&state, body_bytes).await,
            "GetGroup" => self.handle_get_group(&state, body_bytes).await,
            "UpdateGroup" => self.handle_update_group(&state, body_bytes).await,
            "DeleteGroup" => self.handle_delete_group(&state, body_bytes).await,
            "ListGroups" => self.handle_list_groups(&state, body_bytes).await,

            // Identity Provider operations
            "CreateIdentityProvider" => {
                self.handle_create_identity_provider(&state, body_bytes)
                    .await
            }
            "DescribeIdentityProvider" => {
                self.handle_describe_identity_provider(&state, body_bytes)
                    .await
            }
            "UpdateIdentityProvider" => {
                self.handle_update_identity_provider(&state, body_bytes)
                    .await
            }
            "DeleteIdentityProvider" => {
                self.handle_delete_identity_provider(&state, body_bytes)
                    .await
            }
            "ListIdentityProviders" => {
                self.handle_list_identity_providers(&state, body_bytes)
                    .await
            }

            // Resource Server operations
            "CreateResourceServer" => self.handle_create_resource_server(&state, body_bytes).await,
            "DescribeResourceServer" => {
                self.handle_describe_resource_server(&state, body_bytes)
                    .await
            }
            "ListResourceServers" => self.handle_list_resource_servers(&state, body_bytes).await,

            // Custom Attributes
            "AddCustomAttributes" => self.handle_add_custom_attributes(&state, body_bytes).await,

            // MFA Config
            "GetUserPoolMfaConfig" => {
                self.handle_get_user_pool_mfa_config(&state, body_bytes)
                    .await
            }
            "SetUserPoolMfaConfig" => {
                self.handle_set_user_pool_mfa_config(&state, body_bytes)
                    .await
            }

            // Token operations
            "AssociateSoftwareToken" => self.handle_associate_software_token(&state).await,
            "VerifySoftwareToken" => self.handle_verify_software_token(&state).await,

            // Tag operations
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,

            // Device tracking (admin)
            "AdminForgetDevice" => self.handle_admin_forget_device(&state, body_bytes).await,
            "AdminGetDevice" => self.handle_admin_get_device(&state, body_bytes).await,
            "AdminListDevices" => self.handle_admin_list_devices(&state, body_bytes).await,
            "AdminUpdateDeviceStatus" => {
                self.handle_admin_update_device_status(&state, body_bytes)
                    .await
            }

            // Device tracking (user)
            "ConfirmDevice" => self.handle_confirm_device(&state, body_bytes).await,
            "ForgetDevice" => self.handle_forget_device(&state, body_bytes).await,
            "GetDevice" => self.handle_get_device(&state, body_bytes).await,
            "ListDevices" => self.handle_list_devices(&state, body_bytes).await,
            "UpdateDeviceStatus" => self.handle_update_device_status(&state, body_bytes).await,

            // User import jobs
            "CreateUserImportJob" => {
                self.handle_create_user_import_job(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeUserImportJob" => {
                self.handle_describe_user_import_job(&state, body_bytes)
                    .await
            }
            "GetCSVHeader" => self.handle_get_csv_header(&state, body_bytes).await,
            "ListUserImportJobs" => self.handle_list_user_import_jobs(&state, body_bytes).await,
            "StartUserImportJob" => self.handle_start_user_import_job(&state, body_bytes).await,
            "StopUserImportJob" => self.handle_stop_user_import_job(&state, body_bytes).await,

            // Auth events
            "AdminListUserAuthEvents" => {
                self.handle_admin_list_user_auth_events(&state, body_bytes)
                    .await
            }
            "AdminUpdateAuthEventFeedback" => {
                self.handle_admin_update_auth_event_feedback(&state, body_bytes)
                    .await
            }
            "UpdateAuthEventFeedback" => {
                self.handle_update_auth_event_feedback(&state, body_bytes)
                    .await
            }

            // User self-service (token-based)
            "DeleteUser" => self.handle_delete_user(&state, body_bytes).await,
            "GetUserAuthFactors" => self.handle_get_user_auth_factors(&state, body_bytes).await,

            // User attributes (token-based)
            "DeleteUserAttributes" => self.handle_delete_user_attributes(&state, body_bytes).await,
            "GetUserAttributeVerificationCode" => {
                self.handle_get_user_attribute_verification_code(&state, body_bytes)
                    .await
            }
            "VerifyUserAttribute" => self.handle_verify_user_attribute(&state, body_bytes).await,

            // Token operations
            "GetTokensFromRefreshToken" => {
                self.handle_get_tokens_from_refresh_token(&state, body_bytes)
                    .await
            }
            "RevokeToken" => self.handle_revoke_token(&state, body_bytes).await,

            // UI Customization
            "GetUICustomization" => self.handle_get_ui_customization(&state, body_bytes).await,
            "SetUICustomization" => self.handle_set_ui_customization(&state, body_bytes).await,

            // Managed Login Branding
            "CreateManagedLoginBranding" => {
                self.handle_create_managed_login_branding(&state, body_bytes)
                    .await
            }
            "DeleteManagedLoginBranding" => {
                self.handle_delete_managed_login_branding(&state, body_bytes)
                    .await
            }
            "DescribeManagedLoginBranding" => {
                self.handle_describe_managed_login_branding(&state, body_bytes)
                    .await
            }
            "DescribeManagedLoginBrandingByClient" => {
                self.handle_describe_managed_login_branding_by_client(&state, body_bytes)
                    .await
            }
            "UpdateManagedLoginBranding" => {
                self.handle_update_managed_login_branding(&state, body_bytes)
                    .await
            }

            // Risk configuration
            "DescribeRiskConfiguration" => {
                self.handle_describe_risk_configuration(&state, body_bytes)
                    .await
            }
            "SetRiskConfiguration" => self.handle_set_risk_configuration(&state, body_bytes).await,

            // Log delivery
            "GetLogDeliveryConfiguration" => {
                self.handle_get_log_delivery_configuration(&state, body_bytes)
                    .await
            }
            "SetLogDeliveryConfiguration" => {
                self.handle_set_log_delivery_configuration(&state, body_bytes)
                    .await
            }

            // Resource Servers (missing ops)
            "DeleteResourceServer" => self.handle_delete_resource_server(&state, body_bytes).await,
            "UpdateResourceServer" => self.handle_update_resource_server(&state, body_bytes).await,

            // Identity providers (missing ops)
            "AdminDisableProviderForUser" => {
                self.handle_admin_disable_provider_for_user(&state, body_bytes)
                    .await
            }
            "AdminLinkProviderForUser" => {
                self.handle_admin_link_provider_for_user(&state, body_bytes)
                    .await
            }
            "GetIdentityProviderByIdentifier" => {
                self.handle_get_identity_provider_by_identifier(&state, body_bytes)
                    .await
            }

            // WebAuthn (stubs)
            "CompleteWebAuthnRegistration" => {
                self.handle_complete_web_authn_registration(&state).await
            }
            "DeleteWebAuthnCredential" => self.handle_delete_web_authn_credential(&state).await,
            "ListWebAuthnCredentials" => self.handle_list_web_authn_credentials(&state).await,
            "StartWebAuthnRegistration" => self.handle_start_web_authn_registration(&state).await,

            // Terms
            "CreateTerms" => self.handle_create_terms(&state, body_bytes).await,
            "DeleteTerms" => self.handle_delete_terms(&state, body_bytes).await,
            "DescribeTerms" => self.handle_describe_terms(&state, body_bytes).await,
            "ListTerms" => self.handle_list_terms(&state, body_bytes).await,
            "UpdateTerms" => self.handle_update_terms(&state, body_bytes).await,

            // Settings
            "AdminSetUserSettings" => {
                self.handle_admin_set_user_settings(&state, body_bytes)
                    .await
            }
            "GetSigningCertificate" => {
                self.handle_get_signing_certificate(&state, body_bytes)
                    .await
            }
            "SetUserSettings" => self.handle_set_user_settings(&state).await,

            // User Pool Client Secrets
            "AddUserPoolClientSecret" => {
                self.handle_add_user_pool_client_secret(&state, body_bytes)
                    .await
            }
            "DeleteUserPoolClientSecret" => {
                self.handle_delete_user_pool_client_secret(&state, body_bytes)
                    .await
            }
            "ListUserPoolClientSecrets" => {
                self.handle_list_user_pool_client_secrets(&state, body_bytes)
                    .await
            }

            // Resend Confirmation
            "ResendConfirmationCode" => {
                self.handle_resend_confirmation_code(&state, body_bytes)
                    .await
            }

            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for Cognito Identity Provider"),
            ),
        };

        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // --- User Pool ---

    async fn handle_create_user_pool(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_user_pool_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.pool_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PoolName'");
        }
        let name = &input.pool_name;

        let tags = input.user_pool_tags.clone().unwrap_or_default();
        let mut state = state.write().await;
        match state.create_user_pool(name, account_id, region, tags) {
            Ok(pool) => wire::serialize_create_user_pool_response(&wire::CreateUserPoolResponse {
                user_pool: Some(user_pool_wire(pool)),
            }),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_describe_user_pool(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_user_pool_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;

        let state = state.read().await;
        match state.describe_user_pool(user_pool_id) {
            Ok(pool) => {
                wire::serialize_describe_user_pool_response(&wire::DescribeUserPoolResponse {
                    user_pool: Some(user_pool_wire(pool)),
                })
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_update_user_pool(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_user_pool_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;

        let mut state = state.write().await;
        match state.update_user_pool(user_pool_id) {
            Ok(_) => wire::serialize_update_user_pool_response(&wire::UpdateUserPoolResponse {}),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_delete_user_pool(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_user_pool_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;

        let mut state = state.write().await;
        match state.delete_user_pool(user_pool_id) {
            Ok(()) => wire::serialize_delete_user_pool_response(),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_list_user_pools(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let pools = state.list_user_pools();
        let entries: Vec<wire::UserPoolDescriptionType> = pools
            .iter()
            .map(|p| wire::UserPoolDescriptionType {
                id: Some(p.id.clone()),
                name: Some(p.name.clone()),
                status: Some(p.status.clone()),
                creation_date: Some(p.created_date.timestamp() as f64),
                last_modified_date: Some(p.created_date.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_user_pools_response(&wire::ListUserPoolsResponse {
            user_pools: Some(entries),
            next_token: None,
        })
    }

    // --- User Pool Client ---

    async fn handle_create_user_pool_client(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_user_pool_client_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.client_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClientName'");
        }
        let client_name = &input.client_name;

        let generate_secret = input.generate_secret.unwrap_or(false);
        let explicit_auth_flows = input.explicit_auth_flows.unwrap_or_default();
        let allowed_oauth_flows = input.allowed_o_auth_flows.unwrap_or_default();
        let allowed_oauth_scopes = input.allowed_o_auth_scopes.unwrap_or_default();
        let callback_urls = input.callback_u_r_ls.unwrap_or_default();
        let logout_urls = input.logout_u_r_ls.unwrap_or_default();
        let allowed_oauth_flows_user_pool_client =
            input.allowed_o_auth_flows_user_pool_client.unwrap_or(false);
        let refresh_token_validity = input.refresh_token_validity;
        let supported_identity_providers = input.supported_identity_providers.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_user_pool_client(
            user_pool_id,
            client_name,
            generate_secret,
            explicit_auth_flows,
            allowed_oauth_flows,
            allowed_oauth_scopes,
            callback_urls,
            logout_urls,
            allowed_oauth_flows_user_pool_client,
            refresh_token_validity,
            supported_identity_providers,
        ) {
            Ok(client) => wire::serialize_create_user_pool_client_response(
                &wire::CreateUserPoolClientResponse {
                    user_pool_client: Some(user_pool_client_wire(client)),
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_describe_user_pool_client(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_user_pool_client_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.client_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClientId'");
        }
        let client_id = &input.client_id;

        let state = state.read().await;
        match state.describe_user_pool_client(user_pool_id, client_id) {
            Ok(client) => wire::serialize_describe_user_pool_client_response(
                &wire::DescribeUserPoolClientResponse {
                    user_pool_client: Some(user_pool_client_wire(client)),
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_update_user_pool_client(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_user_pool_client_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.client_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClientId'");
        }
        let client_id = &input.client_id;
        let client_name = input.client_name.as_deref();

        let mut state = state.write().await;
        match state.update_user_pool_client(user_pool_id, client_id, client_name) {
            Ok(client) => wire::serialize_update_user_pool_client_response(
                &wire::UpdateUserPoolClientResponse {
                    user_pool_client: Some(user_pool_client_wire(client)),
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_delete_user_pool_client(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_user_pool_client_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.client_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClientId'");
        }
        let client_id = &input.client_id;

        let mut state = state.write().await;
        match state.delete_user_pool_client(user_pool_id, client_id) {
            Ok(()) => wire::serialize_delete_user_pool_client_response(),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_list_user_pool_clients(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_user_pool_clients_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;

        let state = state.read().await;
        match state.list_user_pool_clients(user_pool_id) {
            Ok(clients) => {
                let entries: Vec<wire::UserPoolClientDescription> = clients
                    .iter()
                    .map(|c| wire::UserPoolClientDescription {
                        client_id: Some(c.id.clone()),
                        client_name: Some(c.name.clone()),
                        user_pool_id: Some(c.user_pool_id.clone()),
                    })
                    .collect();
                wire::serialize_list_user_pool_clients_response(
                    &wire::ListUserPoolClientsResponse {
                        user_pool_clients: Some(entries),
                        next_token: None,
                    },
                )
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- User Pool Domain ---

    async fn handle_create_user_pool_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_user_pool_domain_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.domain.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Domain'");
        }
        let domain = &input.domain;

        let mut state = state.write().await;
        match state.create_user_pool_domain(user_pool_id, domain) {
            Ok(()) => wire::serialize_create_user_pool_domain_response(
                &wire::CreateUserPoolDomainResponse {
                    ..Default::default()
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_describe_user_pool_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_user_pool_domain_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.domain.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Domain'");
        }
        let domain = &input.domain;

        let state = state.read().await;
        let domain_info = state.describe_user_pool_domain(domain);
        let description = domain_info.map(|d| wire::DomainDescriptionType {
            domain: Some(d.domain.clone()),
            user_pool_id: Some(d.user_pool_id.clone()),
            status: Some(d.status.clone()),
            cloud_front_distribution: d.cloud_front_distribution.clone(),
            ..Default::default()
        });
        wire::serialize_describe_user_pool_domain_response(&wire::DescribeUserPoolDomainResponse {
            domain_description: description,
        })
    }

    async fn handle_update_user_pool_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_user_pool_domain_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.domain.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Domain'");
        }
        let domain = &input.domain;
        let custom_domain_config =
            input
                .custom_domain_config
                .as_ref()
                .map(|cfg| crate::types::CustomDomainConfig {
                    certificate_arn: cfg.certificate_arn.clone(),
                });

        let mut state = state.write().await;
        match state.update_user_pool_domain(user_pool_id, domain, custom_domain_config) {
            Ok(()) => wire::serialize_update_user_pool_domain_response(
                &wire::UpdateUserPoolDomainResponse {
                    ..Default::default()
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_delete_user_pool_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_user_pool_domain_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.domain.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Domain'");
        }
        let domain = &input.domain;

        let mut state = state.write().await;
        match state.delete_user_pool_domain(user_pool_id, domain) {
            Ok(()) => wire::serialize_delete_user_pool_domain_response(
                &wire::DeleteUserPoolDomainResponse {},
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- Admin User operations ---

    async fn handle_admin_create_user(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_create_user_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;
        let attributes = input
            .user_attributes
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter_map(|a| Some((a.name, a.value?)))
            .collect::<HashMap<_, _>>();

        let mut state = state.write().await;
        match state.admin_create_user(user_pool_id, username, attributes) {
            Ok(user) => {
                wire::serialize_admin_create_user_response(&wire::AdminCreateUserResponse {
                    user: Some(cognito_user_wire(user)),
                })
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_admin_get_user(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_get_user_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;

        let state = state.read().await;
        match state.admin_get_user(user_pool_id, username) {
            Ok(user) => {
                let attrs = user_attributes_wire(&user.attributes);
                wire::serialize_admin_get_user_response(&wire::AdminGetUserResponse {
                    username: Some(user.username.clone()),
                    user_status: Some(user.status.clone()),
                    user_create_date: Some(user.created_date.timestamp() as f64),
                    user_last_modified_date: Some(user.created_date.timestamp() as f64),
                    user_attributes: Some(attrs),
                    enabled: Some(user.enabled),
                    ..Default::default()
                })
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_admin_delete_user(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_delete_user_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;

        let mut state = state.write().await;
        match state.admin_delete_user(user_pool_id, username) {
            Ok(()) => wire::serialize_admin_delete_user_response(),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_admin_delete_user_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_delete_user_attributes_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;
        let attribute_names: Vec<&str> = input
            .user_attribute_names
            .iter()
            .map(String::as_str)
            .collect();

        let mut state = state.write().await;
        match state.admin_delete_user_attributes(user_pool_id, username, &attribute_names) {
            Ok(()) => wire::serialize_admin_delete_user_attributes_response(
                &wire::AdminDeleteUserAttributesResponse {},
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_admin_update_user_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_update_user_attributes_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;
        let attributes = input
            .user_attributes
            .clone()
            .into_iter()
            .filter_map(|a| Some((a.name, a.value?)))
            .collect::<HashMap<_, _>>();

        let mut state = state.write().await;
        match state.admin_update_user_attributes(user_pool_id, username, attributes) {
            Ok(()) => wire::serialize_admin_update_user_attributes_response(
                &wire::AdminUpdateUserAttributesResponse {},
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_admin_enable_user(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_enable_user_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;

        let mut state = state.write().await;
        match state.admin_enable_user(user_pool_id, username) {
            Ok(()) => wire::serialize_admin_enable_user_response(&wire::AdminEnableUserResponse {}),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_admin_disable_user(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_disable_user_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;

        let mut state = state.write().await;
        match state.admin_disable_user(user_pool_id, username) {
            Ok(()) => {
                wire::serialize_admin_disable_user_response(&wire::AdminDisableUserResponse {})
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_admin_set_user_password(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_set_user_password_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;
        if input.password.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Password'");
        }
        let password = &input.password;
        let permanent = input.permanent.unwrap_or(false);

        let mut state = state.write().await;
        match state.admin_set_user_password(user_pool_id, username, password, permanent) {
            Ok(()) => wire::serialize_admin_set_user_password_response(
                &wire::AdminSetUserPasswordResponse {},
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_admin_confirm_sign_up(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_confirm_sign_up_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;

        let mut state = state.write().await;
        match state.admin_confirm_sign_up(user_pool_id, username) {
            Ok(()) => {
                wire::serialize_admin_confirm_sign_up_response(&wire::AdminConfirmSignUpResponse {})
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_admin_reset_user_password(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_reset_user_password_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;

        let mut state = state.write().await;
        match state.admin_reset_user_password(user_pool_id, username) {
            Ok(()) => wire::serialize_admin_reset_user_password_response(
                &wire::AdminResetUserPasswordResponse {},
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_admin_set_user_mfa_preference(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_set_user_m_f_a_preference_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;

        // Verify user exists
        let state = state.read().await;
        match state.admin_get_user(user_pool_id, username) {
            Ok(_) => wire::serialize_admin_set_user_m_f_a_preference_response(
                &wire::AdminSetUserMFAPreferenceResponse {},
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- Admin Group operations ---

    async fn handle_admin_add_user_to_group(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_add_user_to_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;
        if input.group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'GroupName'");
        }
        let group_name = &input.group_name;

        let mut state = state.write().await;
        match state.admin_add_user_to_group(user_pool_id, username, group_name) {
            Ok(()) => wire::serialize_admin_add_user_to_group_response(),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_admin_remove_user_from_group(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_remove_user_from_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;
        if input.group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'GroupName'");
        }
        let group_name = &input.group_name;

        let mut state = state.write().await;
        match state.admin_remove_user_from_group(user_pool_id, username, group_name) {
            Ok(()) => wire::serialize_admin_remove_user_from_group_response(),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_admin_list_groups_for_user(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_list_groups_for_user_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;

        let state = state.read().await;
        match state.admin_list_groups_for_user(user_pool_id, username) {
            Ok(groups) => {
                let entries: Vec<wire::GroupType> = groups.iter().map(|g| group_wire(g)).collect();
                wire::serialize_admin_list_groups_for_user_response(
                    &wire::AdminListGroupsForUserResponse {
                        groups: Some(entries),
                        next_token: None,
                    },
                )
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- Admin Auth operations ---

    async fn handle_admin_initiate_auth(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_initiate_auth_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.client_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClientId'");
        }
        let client_id = &input.client_id;
        if input.auth_flow.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AuthFlow'");
        }
        let auth_flow = &input.auth_flow;
        let auth_parameters = input.auth_parameters.clone().unwrap_or_default();

        let state = state.read().await;
        match state.admin_initiate_auth(user_pool_id, client_id, auth_flow, &auth_parameters) {
            Ok(result) => {
                let auth_result =
                    result
                        .get("AuthenticationResult")
                        .map(|ar| wire::AuthenticationResultType {
                            access_token: ar
                                .get("AccessToken")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            id_token: ar
                                .get("IdToken")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            refresh_token: ar
                                .get("RefreshToken")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            expires_in: ar
                                .get("ExpiresIn")
                                .and_then(|v| v.as_i64())
                                .map(|v| v as i32),
                            token_type: ar
                                .get("TokenType")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            ..Default::default()
                        });
                wire::serialize_admin_initiate_auth_response(&wire::AdminInitiateAuthResponse {
                    authentication_result: auth_result,
                    ..Default::default()
                })
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_admin_respond_to_auth_challenge(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_respond_to_auth_challenge_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.client_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClientId'");
        }
        let client_id = &input.client_id;
        if input.challenge_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ChallengeName'");
        }
        let challenge_name = &input.challenge_name;
        let challenge_responses = input.challenge_responses.clone().unwrap_or_default();

        let state = state.read().await;
        match state.admin_respond_to_auth_challenge(
            user_pool_id,
            client_id,
            challenge_name,
            &challenge_responses,
        ) {
            Ok(result) => {
                let auth_result =
                    result
                        .get("AuthenticationResult")
                        .map(|ar| wire::AuthenticationResultType {
                            access_token: ar
                                .get("AccessToken")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            id_token: ar
                                .get("IdToken")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            refresh_token: ar
                                .get("RefreshToken")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            expires_in: ar
                                .get("ExpiresIn")
                                .and_then(|v| v.as_i64())
                                .map(|v| v as i32),
                            token_type: ar
                                .get("TokenType")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            ..Default::default()
                        });
                wire::serialize_admin_respond_to_auth_challenge_response(
                    &wire::AdminRespondToAuthChallengeResponse {
                        authentication_result: auth_result,
                        ..Default::default()
                    },
                )
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_admin_user_global_sign_out(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_user_global_sign_out_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;

        let state = state.read().await;
        match state.admin_user_global_sign_out(user_pool_id, username) {
            Ok(()) => wire::serialize_admin_user_global_sign_out_response(
                &wire::AdminUserGlobalSignOutResponse {},
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- Sign Up / Confirm ---

    async fn handle_sign_up(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_sign_up_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.client_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClientId'");
        }
        let client_id = &input.client_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;
        let password = match input.password.as_deref() {
            Some(s) if !s.is_empty() => s,
            _ => return json_error_response(400, "ValidationException", "Missing 'Password'"),
        };
        let attributes = input
            .user_attributes
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter_map(|a| Some((a.name, a.value?)))
            .collect::<HashMap<_, _>>();

        let mut state = state.write().await;
        // Find pool by client_id
        let pool_id = match state.find_pool_id_by_client_id(client_id) {
            Some(id) => id,
            None => {
                return json_error_response(
                    400,
                    "ResourceNotFoundException",
                    &format!("Client {client_id} does not exist."),
                );
            }
        };
        match state.sign_up(&pool_id, username, password, attributes) {
            Ok(user) => {
                let sub = user
                    .attributes
                    .get("sub")
                    .cloned()
                    .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
                wire::serialize_sign_up_response(&wire::SignUpResponse {
                    user_confirmed: Some(user.confirmed),
                    user_sub: Some(sub),
                    ..Default::default()
                })
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_confirm_sign_up(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_confirm_sign_up_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.client_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClientId'");
        }
        let client_id = &input.client_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;
        // ConfirmationCode is required by the API but we just accept anything
        let _confirmation_code = if input.confirmation_code.is_empty() {
            None
        } else {
            Some(input.confirmation_code.as_str())
        };

        let mut state = state.write().await;
        let pool_id = match state.find_pool_id_by_client_id(client_id) {
            Some(id) => id,
            None => {
                return json_error_response(
                    400,
                    "ResourceNotFoundException",
                    &format!("Client {client_id} does not exist."),
                );
            }
        };
        match state.confirm_sign_up(&pool_id, username) {
            Ok(()) => wire::serialize_confirm_sign_up_response(&wire::ConfirmSignUpResponse {
                ..Default::default()
            }),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_forgot_password(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_forgot_password_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.client_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClientId'");
        }
        let client_id = &input.client_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;

        let state = state.read().await;
        let pool_id = match state.find_pool_id_by_client_id(client_id) {
            Some(id) => id,
            None => {
                return json_error_response(
                    400,
                    "ResourceNotFoundException",
                    &format!("Client {client_id} does not exist."),
                );
            }
        };
        // Verify user exists
        match state.admin_get_user(&pool_id, username) {
            Ok(_) => wire::serialize_forgot_password_response(&wire::ForgotPasswordResponse {
                code_delivery_details: Some(wire::CodeDeliveryDetailsType {
                    delivery_medium: Some("EMAIL".to_string()),
                    destination: Some("t***@e***.com".to_string()),
                    attribute_name: Some("email".to_string()),
                }),
            }),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_confirm_forgot_password(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_confirm_forgot_password_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.client_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClientId'");
        }
        let client_id = &input.client_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;
        if input.password.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Password'");
        }
        let password = &input.password;
        let _confirmation_code = if input.confirmation_code.is_empty() {
            None
        } else {
            Some(input.confirmation_code.as_str())
        };

        let mut state = state.write().await;
        let pool_id = match state.find_pool_id_by_client_id(client_id) {
            Some(id) => id,
            None => {
                return json_error_response(
                    400,
                    "ResourceNotFoundException",
                    &format!("Client {client_id} does not exist."),
                );
            }
        };
        match state.admin_set_user_password(&pool_id, username, password, true) {
            Ok(()) => wire::serialize_confirm_forgot_password_response(
                &wire::ConfirmForgotPasswordResponse {},
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- Auth operations ---

    async fn handle_initiate_auth(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_initiate_auth_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.client_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClientId'");
        }
        let client_id = &input.client_id;
        if input.auth_flow.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AuthFlow'");
        }
        let auth_flow = &input.auth_flow;
        let auth_parameters = input.auth_parameters.clone().unwrap_or_default();

        let state = state.read().await;
        let pool_id = match state.find_pool_id_by_client_id(client_id) {
            Some(id) => id,
            None => {
                return json_error_response(
                    400,
                    "ResourceNotFoundException",
                    &format!("Client {client_id} does not exist."),
                );
            }
        };
        match state.initiate_auth(&pool_id, client_id, auth_flow, &auth_parameters) {
            Ok(result) => {
                let auth_result =
                    result
                        .get("AuthenticationResult")
                        .map(|ar| wire::AuthenticationResultType {
                            access_token: ar
                                .get("AccessToken")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            id_token: ar
                                .get("IdToken")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            refresh_token: ar
                                .get("RefreshToken")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            expires_in: ar
                                .get("ExpiresIn")
                                .and_then(|v| v.as_i64())
                                .map(|v| v as i32),
                            token_type: ar
                                .get("TokenType")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            ..Default::default()
                        });
                wire::serialize_initiate_auth_response(&wire::InitiateAuthResponse {
                    authentication_result: auth_result,
                    ..Default::default()
                })
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_respond_to_auth_challenge(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_respond_to_auth_challenge_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.client_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClientId'");
        }
        let client_id = &input.client_id;
        if input.challenge_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ChallengeName'");
        }
        let challenge_name = &input.challenge_name;
        let challenge_responses = input.challenge_responses.clone().unwrap_or_default();

        let state = state.read().await;
        let pool_id = match state.find_pool_id_by_client_id(client_id) {
            Some(id) => id,
            None => {
                return json_error_response(
                    400,
                    "ResourceNotFoundException",
                    &format!("Client {client_id} does not exist."),
                );
            }
        };
        match state.respond_to_auth_challenge(
            &pool_id,
            client_id,
            challenge_name,
            &challenge_responses,
        ) {
            Ok(result) => {
                let auth_result =
                    result
                        .get("AuthenticationResult")
                        .map(|ar| wire::AuthenticationResultType {
                            access_token: ar
                                .get("AccessToken")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            id_token: ar
                                .get("IdToken")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            refresh_token: ar
                                .get("RefreshToken")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            expires_in: ar
                                .get("ExpiresIn")
                                .and_then(|v| v.as_i64())
                                .map(|v| v as i32),
                            token_type: ar
                                .get("TokenType")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                            ..Default::default()
                        });
                wire::serialize_respond_to_auth_challenge_response(
                    &wire::RespondToAuthChallengeResponse {
                        authentication_result: auth_result,
                        ..Default::default()
                    },
                )
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_global_sign_out(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.global_sign_out() {
            Ok(()) => wire::serialize_global_sign_out_response(&wire::GlobalSignOutResponse {}),
            Err(e) => cognito_error_response(&e),
        }
    }

    // STUB[no-auth]: The mock has no token validation layer; ChangePassword accepts
    //   any AccessToken and returns success without verifying credentials or state.
    async fn handle_change_password(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_change_password_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        // AccessToken-based operation; we just accept it
        let _access_token = if input.access_token.is_empty() {
            None
        } else {
            Some(input.access_token.as_str())
        };
        let _previous_password = input.previous_password.as_deref();
        let _proposed_password = if input.proposed_password.is_empty() {
            None
        } else {
            Some(input.proposed_password.as_str())
        };

        let _state = state.read().await;
        wire::serialize_change_password_response(&wire::ChangePasswordResponse {})
    }

    // --- User self-service operations ---

    // STUB[no-auth]: GetUser requires decoding the AccessToken to identify the caller;
    //   the mock has no token layer so it returns the first user found in any pool.
    async fn handle_get_user(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_user_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        // GetUser is token-based; we just need AccessToken
        let _access_token = if input.access_token.is_empty() {
            None
        } else {
            Some(input.access_token.as_str())
        };

        // Since this is token-based in a mock, return a minimal response
        // In real impl you'd decode the token. We'll find the first user in any pool.
        let state = state.read().await;
        for pool in state.user_pools.values() {
            if let Some(user) = pool.users.values().next() {
                let attrs = user_attributes_wire(&user.attributes);
                return wire::serialize_get_user_response(&wire::GetUserResponse {
                    username: Some(user.username.clone()),
                    user_attributes: Some(attrs),
                    ..Default::default()
                });
            }
        }
        json_error_response(400, "NotAuthorizedException", "Invalid access token.")
    }

    // STUB[no-auth]: UpdateUserAttributes is token-based; the mock accepts any
    //   AccessToken and returns success without actually applying attribute changes.
    async fn handle_update_user_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_user_attributes_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        // Token-based; in mock we accept it
        let _access_token = if input.access_token.is_empty() {
            None
        } else {
            Some(input.access_token.as_str())
        };

        let _state = state.read().await;
        wire::serialize_update_user_attributes_response(&wire::UpdateUserAttributesResponse {
            ..Default::default()
        })
    }

    // STUB[no-auth]: SetUserMFAPreference is token-based; the mock has no token
    //   layer and always returns success without persisting any preference change.
    async fn handle_set_user_mfa_preference(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
    ) -> MockResponse {
        let _state = state.read().await;
        wire::serialize_set_user_m_f_a_preference_response(&wire::SetUserMFAPreferenceResponse {})
    }

    async fn handle_list_users(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_users_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;

        let state = state.read().await;
        match state.list_users(user_pool_id) {
            Ok(users) => {
                let entries: Vec<wire::UserType> =
                    users.iter().map(|u| cognito_user_wire(u)).collect();
                wire::serialize_list_users_response(&wire::ListUsersResponse {
                    users: Some(entries),
                    pagination_token: None,
                })
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_list_users_in_group(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_users_in_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'GroupName'");
        }
        let group_name = &input.group_name;

        let state = state.read().await;
        match state.list_users_in_group(user_pool_id, group_name) {
            Ok(users) => {
                let entries: Vec<wire::UserType> =
                    users.iter().map(|u| cognito_user_wire(u)).collect();
                wire::serialize_list_users_in_group_response(&wire::ListUsersInGroupResponse {
                    users: Some(entries),
                    next_token: None,
                })
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- Group operations ---

    async fn handle_create_group(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'GroupName'");
        }
        let group_name = &input.group_name;
        let description = input.description.as_deref();
        let role_arn = input.role_arn.as_deref();
        let precedence = input.precedence;

        let mut state = state.write().await;
        match state.create_group(user_pool_id, group_name, description, role_arn, precedence) {
            Ok(group) => wire::serialize_create_group_response(&wire::CreateGroupResponse {
                group: Some(group_wire(group)),
            }),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_get_group(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'GroupName'");
        }
        let group_name = &input.group_name;

        let state = state.read().await;
        match state.get_group(user_pool_id, group_name) {
            Ok(group) => wire::serialize_get_group_response(&wire::GetGroupResponse {
                group: Some(group_wire(group)),
            }),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_update_group(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'GroupName'");
        }
        let group_name = &input.group_name;
        let description = input.description.as_deref();
        let role_arn = input.role_arn.as_deref();
        let precedence = input.precedence;

        let mut state = state.write().await;
        match state.update_group(user_pool_id, group_name, description, role_arn, precedence) {
            Ok(group) => wire::serialize_update_group_response(&wire::UpdateGroupResponse {
                group: Some(group_wire(group)),
            }),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_delete_group(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'GroupName'");
        }
        let group_name = &input.group_name;

        let mut state = state.write().await;
        match state.delete_group(user_pool_id, group_name) {
            Ok(()) => wire::serialize_delete_group_response(),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_list_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_groups_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;

        let state = state.read().await;
        match state.list_groups(user_pool_id) {
            Ok(groups) => {
                let entries: Vec<wire::GroupType> = groups.iter().map(|g| group_wire(g)).collect();
                wire::serialize_list_groups_response(&wire::ListGroupsResponse {
                    groups: Some(entries),
                    next_token: None,
                })
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- Identity Provider operations ---

    async fn handle_create_identity_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_identity_provider_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.provider_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ProviderName'");
        }
        let provider_name = &input.provider_name;
        if input.provider_type.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ProviderType'");
        }
        let provider_type = &input.provider_type;
        let provider_details = input.provider_details.clone();
        let attribute_mapping = input.attribute_mapping.clone().unwrap_or_default();
        let idp_identifiers = input.idp_identifiers.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_identity_provider(
            user_pool_id,
            provider_name,
            provider_type,
            provider_details,
            attribute_mapping,
            idp_identifiers,
        ) {
            Ok(idp) => wire::serialize_create_identity_provider_response(
                &wire::CreateIdentityProviderResponse {
                    identity_provider: Some(idp_wire(idp)),
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_describe_identity_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_identity_provider_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.provider_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ProviderName'");
        }
        let provider_name = &input.provider_name;

        let state = state.read().await;
        match state.describe_identity_provider(user_pool_id, provider_name) {
            Ok(idp) => wire::serialize_describe_identity_provider_response(
                &wire::DescribeIdentityProviderResponse {
                    identity_provider: Some(idp_wire(idp)),
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_update_identity_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_identity_provider_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.provider_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ProviderName'");
        }
        let provider_name = &input.provider_name;
        let provider_details = input.provider_details.clone();
        let attribute_mapping = input.attribute_mapping.clone();
        let idp_identifiers = input.idp_identifiers.clone();

        let mut state = state.write().await;
        match state.update_identity_provider(
            user_pool_id,
            provider_name,
            provider_details,
            attribute_mapping,
            idp_identifiers,
        ) {
            Ok(idp) => wire::serialize_update_identity_provider_response(
                &wire::UpdateIdentityProviderResponse {
                    identity_provider: Some(idp_wire(idp)),
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_delete_identity_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_identity_provider_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.provider_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ProviderName'");
        }
        let provider_name = &input.provider_name;

        let mut state = state.write().await;
        match state.delete_identity_provider(user_pool_id, provider_name) {
            Ok(()) => wire::serialize_delete_identity_provider_response(),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_list_identity_providers(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_identity_providers_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;

        let state = state.read().await;
        match state.list_identity_providers(user_pool_id) {
            Ok(idps) => {
                let entries: Vec<wire::ProviderDescription> = idps
                    .iter()
                    .map(|idp| wire::ProviderDescription {
                        provider_name: Some(idp.provider_name.clone()),
                        provider_type: Some(idp.provider_type.clone()),
                        creation_date: Some(idp.created_date.timestamp() as f64),
                        last_modified_date: Some(idp.last_modified_date.timestamp() as f64),
                    })
                    .collect();
                wire::serialize_list_identity_providers_response(
                    &wire::ListIdentityProvidersResponse {
                        providers: Some(entries),
                        next_token: None,
                    },
                )
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- Resource Server operations ---

    async fn handle_create_resource_server(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_resource_server_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.identifier.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Identifier'");
        }
        let identifier = &input.identifier;
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        let name = &input.name;
        let scopes = input
            .scopes
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(|s| crate::types::ResourceServerScope {
                scope_name: s.scope_name,
                scope_description: s.scope_description,
            })
            .collect::<Vec<_>>();

        let mut state = state.write().await;
        match state.create_resource_server(user_pool_id, identifier, name, scopes) {
            Ok(rs) => wire::serialize_create_resource_server_response(
                &wire::CreateResourceServerResponse {
                    resource_server: Some(resource_server_wire(rs)),
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_describe_resource_server(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_resource_server_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.identifier.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Identifier'");
        }
        let identifier = &input.identifier;

        let state = state.read().await;
        match state.describe_resource_server(user_pool_id, identifier) {
            Ok(rs) => wire::serialize_describe_resource_server_response(
                &wire::DescribeResourceServerResponse {
                    resource_server: Some(resource_server_wire(rs)),
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_list_resource_servers(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_resource_servers_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;

        let state = state.read().await;
        match state.list_resource_servers(user_pool_id) {
            Ok(servers) => {
                let entries: Vec<wire::ResourceServerType> =
                    servers.iter().map(|rs| resource_server_wire(rs)).collect();
                wire::serialize_list_resource_servers_response(&wire::ListResourceServersResponse {
                    resource_servers: Some(entries),
                    next_token: None,
                })
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- Custom Attributes ---

    async fn handle_add_custom_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_add_custom_attributes_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        let attributes: Vec<String> = input
            .custom_attributes
            .iter()
            .filter_map(|a| a.name.clone())
            .collect();

        let mut state = state.write().await;
        match state.add_custom_attributes(user_pool_id, attributes) {
            Ok(()) => wire::serialize_add_custom_attributes_response(
                &wire::AddCustomAttributesResponse {},
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- MFA Config ---

    async fn handle_get_user_pool_mfa_config(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_user_pool_mfa_config_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;

        let state = state.read().await;
        match state.get_user_pool_mfa_config(user_pool_id) {
            Ok(mfa_config) => {
                let mut resp = wire::GetUserPoolMfaConfigResponse {
                    ..Default::default()
                };
                if let Some(config) = mfa_config {
                    resp.mfa_configuration = Some(config.mfa_configuration.clone());
                    if let Some(ref sms) = config.sms_mfa_configuration {
                        resp.sms_mfa_configuration = Some(wire::SmsMfaConfigType {
                            sms_authentication_message: sms.sms_authentication_message.clone(),
                            ..Default::default()
                        });
                    }
                    if let Some(ref sw) = config.software_token_mfa_configuration {
                        resp.software_token_mfa_configuration =
                            Some(wire::SoftwareTokenMfaConfigType {
                                enabled: Some(sw.enabled),
                            });
                    }
                }
                wire::serialize_get_user_pool_mfa_config_response(&resp)
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_set_user_pool_mfa_config(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_set_user_pool_mfa_config_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        let mfa_configuration = match input.mfa_configuration.as_deref() {
            Some(s) if !s.is_empty() => s,
            _ => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing 'MfaConfiguration'",
                );
            }
        };

        let sms_config = input
            .sms_mfa_configuration
            .as_ref()
            .map(|v| crate::types::SmsMfaConfig {
                sms_authentication_message: v.sms_authentication_message.clone(),
            });

        let sw_config = input.software_token_mfa_configuration.as_ref().map(|v| {
            crate::types::SoftwareTokenMfaConfig {
                enabled: v.enabled.unwrap_or(false),
            }
        });

        let mut state = state.write().await;
        match state.set_user_pool_mfa_config(user_pool_id, mfa_configuration, sms_config, sw_config)
        {
            Ok(config) => {
                let mut resp = wire::SetUserPoolMfaConfigResponse {
                    mfa_configuration: Some(config.mfa_configuration.clone()),
                    ..Default::default()
                };
                if let Some(ref sms) = config.sms_mfa_configuration {
                    resp.sms_mfa_configuration = Some(wire::SmsMfaConfigType {
                        sms_authentication_message: sms.sms_authentication_message.clone(),
                        ..Default::default()
                    });
                }
                if let Some(ref sw) = config.software_token_mfa_configuration {
                    resp.software_token_mfa_configuration =
                        Some(wire::SoftwareTokenMfaConfigType {
                            enabled: Some(sw.enabled),
                        });
                }
                wire::serialize_set_user_pool_mfa_config_response(&resp)
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- Token operations ---

    // STUB[no-auth]: AssociateSoftwareToken is token-based; the mock generates a
    //   random mock TOTP secret code without verifying the caller's identity.
    async fn handle_associate_software_token(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
    ) -> MockResponse {
        let _state = state.read().await;
        wire::serialize_associate_software_token_response(&wire::AssociateSoftwareTokenResponse {
            secret_code: Some(format!(
                "MOCK{}",
                uuid::Uuid::new_v4()
                    .to_string()
                    .replace('-', "")
                    .chars()
                    .take(20)
                    .collect::<String>()
            )),
            ..Default::default()
        })
    }

    // STUB[no-engine]: VerifySoftwareToken requires a real TOTP validation engine;
    //   the mock always returns SUCCESS without verifying the user-supplied code.
    async fn handle_verify_software_token(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
    ) -> MockResponse {
        let _state = state.read().await;
        wire::serialize_verify_software_token_response(&wire::VerifySoftwareTokenResponse {
            status: Some("SUCCESS".to_string()),
            ..Default::default()
        })
    }

    // --- Tags ---

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceArn'");
        }
        let resource_arn = &input.resource_arn;
        let tags = input.tags.clone();

        let mut state = state.write().await;
        match state.tag_resource(resource_arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceArn'");
        }
        let resource_arn = &input.resource_arn;
        let tag_keys = input.tag_keys.clone();

        let mut state = state.write().await;
        match state.untag_resource(resource_arn, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceArn'");
        }
        let resource_arn = &input.resource_arn;

        let state = state.read().await;
        match state.list_tags_for_resource(resource_arn) {
            Ok(tags) => wire::serialize_list_tags_for_resource_response(
                &wire::ListTagsForResourceResponse {
                    tags: Some(tags.clone()),
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- Device tracking (admin) ---

    async fn handle_admin_forget_device(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_forget_device_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;
        if input.device_key.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DeviceKey'");
        }
        let device_key = &input.device_key;

        let mut state = state.write().await;
        match state.admin_forget_device(user_pool_id, username, device_key) {
            Ok(()) => wire::serialize_admin_forget_device_response(),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_admin_get_device(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_get_device_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;
        if input.device_key.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DeviceKey'");
        }
        let device_key = &input.device_key;

        let state = state.read().await;
        match state.admin_get_device(user_pool_id, username, device_key) {
            Ok(device_opt) => {
                let device_wire = device_opt.map(|d| wire::DeviceType {
                    device_key: Some(d.device_key.clone()),
                    device_create_date: Some(d.device_created_date.timestamp() as f64),
                    device_last_modified_date: Some(d.device_last_modified_date.timestamp() as f64),
                    device_attributes: if d.device_attributes.is_empty() {
                        None
                    } else {
                        Some(
                            d.device_attributes
                                .iter()
                                .map(|(k, v)| wire::AttributeType {
                                    name: k.clone(),
                                    value: Some(v.clone()),
                                })
                                .collect(),
                        )
                    },
                    ..Default::default()
                });
                wire::serialize_admin_get_device_response(&wire::AdminGetDeviceResponse {
                    device: device_wire,
                })
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_admin_list_devices(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_list_devices_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;

        let state = state.read().await;
        match state.admin_list_devices(user_pool_id, username) {
            Ok(devices) => {
                let entries: Vec<wire::DeviceType> = devices
                    .iter()
                    .map(|d| wire::DeviceType {
                        device_key: Some(d.device_key.clone()),
                        device_create_date: Some(d.device_created_date.timestamp() as f64),
                        device_last_modified_date: Some(
                            d.device_last_modified_date.timestamp() as f64
                        ),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_admin_list_devices_response(&wire::AdminListDevicesResponse {
                    devices: Some(entries),
                    pagination_token: None,
                })
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_admin_update_device_status(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_update_device_status_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;
        if input.device_key.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DeviceKey'");
        }
        let device_key = &input.device_key;

        let mut state = state.write().await;
        match state.admin_update_device_status(user_pool_id, username, device_key) {
            Ok(()) => wire::serialize_admin_update_device_status_response(
                &wire::AdminUpdateDeviceStatusResponse {},
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- Device tracking (user, token-based) ---

    async fn handle_confirm_device(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_confirm_device_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.device_key.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DeviceKey'");
        }
        let device_key = &input.device_key;
        let device_name = input.device_name.as_deref();

        let mut state = state.write().await;
        match state.confirm_device(device_key, device_name) {
            Ok(()) => wire::serialize_confirm_device_response(&wire::ConfirmDeviceResponse {
                user_confirmation_necessary: Some(false),
            }),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_forget_device(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_forget_device_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.device_key.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DeviceKey'");
        }
        let device_key = &input.device_key;
        let mut state = state.write().await;
        match state.forget_device(device_key) {
            Ok(()) => wire::serialize_forget_device_response(),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_get_device(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_device_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.device_key.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DeviceKey'");
        }
        let device_key = &input.device_key;
        let state = state.read().await;
        match state.get_device(device_key) {
            Ok(_) => wire::serialize_get_device_response(&wire::GetDeviceResponse {
                device: Some(wire::DeviceType {
                    device_key: Some(device_key.to_string()),
                    ..Default::default()
                }),
            }),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_list_devices(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_devices_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_devices() {
            Ok(_) => wire::serialize_list_devices_response(&wire::ListDevicesResponse {
                devices: Some(vec![]),
                pagination_token: None,
            }),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_update_device_status(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_device_status_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.device_key.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DeviceKey'");
        }
        let device_key = &input.device_key;
        let mut state = state.write().await;
        match state.update_device_status(device_key) {
            Ok(()) => {
                wire::serialize_update_device_status_response(&wire::UpdateDeviceStatusResponse {})
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- User Import Jobs ---

    async fn handle_create_user_import_job(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_user_import_job_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.job_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'JobName'");
        }
        let job_name = &input.job_name;

        let mut state = state.write().await;
        match state.create_user_import_job(user_pool_id, job_name, account_id, region) {
            Ok(job) => wire::serialize_create_user_import_job_response(
                &wire::CreateUserImportJobResponse {
                    user_import_job: Some(user_import_job_wire(job)),
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_describe_user_import_job(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_user_import_job_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.job_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'JobId'");
        }
        let job_id = &input.job_id;

        let state = state.read().await;
        match state.describe_user_import_job(user_pool_id, job_id) {
            Ok(job) => wire::serialize_describe_user_import_job_response(
                &wire::DescribeUserImportJobResponse {
                    user_import_job: Some(user_import_job_wire(job)),
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_get_csv_header(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_c_s_v_header_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        let state = state.read().await;
        match state.describe_user_pool(user_pool_id) {
            Ok(_) => wire::serialize_get_c_s_v_header_response(&wire::GetCSVHeaderResponse {
                user_pool_id: Some(user_pool_id.to_string()),
                c_s_v_header: Some(vec![
                    "cognito:username".to_string(),
                    "name".to_string(),
                    "given_name".to_string(),
                    "family_name".to_string(),
                    "email".to_string(),
                    "email_verified".to_string(),
                    "phone_number".to_string(),
                    "phone_number_verified".to_string(),
                ]),
            }),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_list_user_import_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_user_import_jobs_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;

        let state = state.read().await;
        match state.list_user_import_jobs(user_pool_id) {
            Ok(jobs) => {
                let entries: Vec<wire::UserImportJobType> =
                    jobs.iter().map(|j| user_import_job_wire(j)).collect();
                wire::serialize_list_user_import_jobs_response(&wire::ListUserImportJobsResponse {
                    user_import_jobs: Some(entries),
                    pagination_token: None,
                })
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_start_user_import_job(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_user_import_job_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.job_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'JobId'");
        }
        let job_id = &input.job_id;

        let mut state = state.write().await;
        match state.start_user_import_job(user_pool_id, job_id) {
            Ok(job) => {
                wire::serialize_start_user_import_job_response(&wire::StartUserImportJobResponse {
                    user_import_job: Some(user_import_job_wire(job)),
                })
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_stop_user_import_job(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_user_import_job_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.job_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'JobId'");
        }
        let job_id = &input.job_id;

        let mut state = state.write().await;
        match state.stop_user_import_job(user_pool_id, job_id) {
            Ok(job) => {
                wire::serialize_stop_user_import_job_response(&wire::StopUserImportJobResponse {
                    user_import_job: Some(user_import_job_wire(job)),
                })
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- Auth Events ---

    // STUB[no-telemetry]: AdminListUserAuthEvents returns real authentication event
    //   history driven by infrastructure telemetry; the mock always returns empty.
    async fn handle_admin_list_user_auth_events(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_list_user_auth_events_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;

        let state = state.read().await;
        match state.admin_get_user(user_pool_id, username) {
            Ok(_) => wire::serialize_admin_list_user_auth_events_response(
                &wire::AdminListUserAuthEventsResponse {
                    auth_events: Some(vec![]),
                    next_token: None,
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    // STUB[no-telemetry]: AdminUpdateAuthEventFeedback feeds back into real adaptive
    //   auth risk scoring; the mock accepts the call but performs no state change.
    async fn handle_admin_update_auth_event_feedback(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_update_auth_event_feedback_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;

        let state = state.read().await;
        match state.admin_get_user(user_pool_id, username) {
            Ok(_) => wire::serialize_admin_update_auth_event_feedback_response(
                &wire::AdminUpdateAuthEventFeedbackResponse {},
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    // STUB[no-telemetry]: UpdateAuthEventFeedback (user-facing) feeds adaptive auth
    //   risk scoring; the mock accepts the call but performs no state change.
    async fn handle_update_auth_event_feedback(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        _body: &[u8],
    ) -> MockResponse {
        let _state = state.read().await;
        wire::serialize_update_auth_event_feedback_response(
            &wire::UpdateAuthEventFeedbackResponse {},
        )
    }

    // --- User self-service (token-based) ---

    // STUB[no-auth]: DeleteUser is token-based; the mock has no token layer and
    //   accepts any AccessToken without actually deleting the identified user.
    async fn handle_delete_user(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_user_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        // DeleteUser deletes the user identified by AccessToken.
        // For the mock, we accept the call and return success.
        if input.access_token.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AccessToken'");
        }

        let _ = state;
        wire::serialize_delete_user_response()
    }

    // STUB[no-auth]: GetUserAuthFactors is token-based; the mock returns an empty
    //   configured factors list without resolving the caller from the AccessToken.
    async fn handle_get_user_auth_factors(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_user_auth_factors_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.access_token.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AccessToken'");
        }

        let _ = state;
        wire::serialize_get_user_auth_factors_response(&wire::GetUserAuthFactorsResponse {
            configured_user_auth_factors: Some(vec![]),
            preferred_mfa_setting: None,
            user_m_f_a_setting_list: None,
            username: None,
        })
    }

    // --- User attributes (token-based) ---

    async fn handle_delete_user_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_user_attributes_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.access_token.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AccessToken'");
        }
        let access_token = &input.access_token;
        let attribute_names = input.user_attribute_names.clone();

        let mut state = state.write().await;
        match state.delete_user_attributes(access_token, &attribute_names) {
            Ok(()) => wire::serialize_delete_user_attributes_response(
                &wire::DeleteUserAttributesResponse {},
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    // STUB[no-auth]: GetUserAttributeVerificationCode sends a real verification
    //   code via email/SMS; the mock returns a fake delivery destination without
    //   actually sending anything or resolving the caller from the AccessToken.
    async fn handle_get_user_attribute_verification_code(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_user_attribute_verification_code_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let _access_token = if input.access_token.is_empty() {
            None
        } else {
            Some(input.access_token.as_str())
        };
        if input.attribute_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AttributeName'");
        }
        let attribute_name = &input.attribute_name;

        let _state = state.read().await;
        wire::serialize_get_user_attribute_verification_code_response(
            &wire::GetUserAttributeVerificationCodeResponse {
                code_delivery_details: Some(wire::CodeDeliveryDetailsType {
                    delivery_medium: Some("EMAIL".to_string()),
                    destination: Some("t***@e***.com".to_string()),
                    attribute_name: Some(attribute_name.to_string()),
                }),
            },
        )
    }

    // STUB[no-auth]: VerifyUserAttribute verifies a code sent to the caller's
    //   email or phone; the mock accepts any call and returns success without
    //   validation or resolving the caller from the AccessToken.
    async fn handle_verify_user_attribute(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_verify_user_attribute_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let _state = state.read().await;
        wire::serialize_verify_user_attribute_response(&wire::VerifyUserAttributeResponse {})
    }

    // --- Token operations ---

    // STUB[no-auth]: GetTokensFromRefreshToken requires validating a real refresh
    //   token; the mock issues new random mock tokens without validation.
    async fn handle_get_tokens_from_refresh_token(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        _body: &[u8],
    ) -> MockResponse {
        let _state = state.read().await;
        wire::serialize_get_tokens_from_refresh_token_response(
            &wire::GetTokensFromRefreshTokenResponse {
                authentication_result: Some(wire::AuthenticationResultType {
                    access_token: Some(format!("access-token-{}", uuid::Uuid::new_v4())),
                    id_token: Some(format!("id-token-{}", uuid::Uuid::new_v4())),
                    refresh_token: Some(format!("refresh-token-{}", uuid::Uuid::new_v4())),
                    expires_in: Some(3600),
                    token_type: Some("Bearer".to_string()),
                    ..Default::default()
                }),
            },
        )
    }

    // STUB[no-auth]: RevokeToken invalidates a refresh token; the mock has no
    //   token store so it accepts the call and returns success without any action.
    async fn handle_revoke_token(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_revoke_token_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let _state = state.read().await;
        wire::serialize_revoke_token_response(&wire::RevokeTokenResponse {})
    }

    // --- UI Customization ---

    async fn handle_get_ui_customization(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_u_i_customization_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;

        let state = state.read().await;
        match state.get_ui_customization(user_pool_id) {
            Ok(ui_opt) => {
                let ui_wire = ui_opt.map(|u| wire::UICustomizationType {
                    user_pool_id: Some(u.user_pool_id.clone()),
                    client_id: u.client_id.clone(),
                    c_s_s: u.css.clone(),
                    c_s_s_version: u.css_version.clone(),
                    image_url: u.image_url.clone(),
                    creation_date: Some(u.creation_date.timestamp() as f64),
                    last_modified_date: Some(u.last_modified_date.timestamp() as f64),
                });
                wire::serialize_get_u_i_customization_response(&wire::GetUICustomizationResponse {
                    u_i_customization: ui_wire,
                })
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_set_ui_customization(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_set_u_i_customization_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        let client_id = input.client_id.as_deref();
        let css = input.c_s_s.as_deref();
        let image_url = input.image_file.as_deref();

        let mut state = state.write().await;
        match state.set_ui_customization(user_pool_id, client_id, css, image_url) {
            Ok(u) => {
                let ui_wire = wire::UICustomizationType {
                    user_pool_id: Some(u.user_pool_id.clone()),
                    client_id: u.client_id.clone(),
                    c_s_s: u.css.clone(),
                    c_s_s_version: u.css_version.clone(),
                    image_url: u.image_url.clone(),
                    creation_date: Some(u.creation_date.timestamp() as f64),
                    last_modified_date: Some(u.last_modified_date.timestamp() as f64),
                };
                wire::serialize_set_u_i_customization_response(&wire::SetUICustomizationResponse {
                    u_i_customization: Some(ui_wire),
                })
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- Managed Login Branding ---

    async fn handle_create_managed_login_branding(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_managed_login_branding_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        let settings = input.settings.clone();

        let mut state = state.write().await;
        match state.create_managed_login_branding(user_pool_id, settings) {
            Ok(b) => wire::serialize_create_managed_login_branding_response(
                &wire::CreateManagedLoginBrandingResponse {
                    managed_login_branding: Some(managed_login_branding_wire(b)),
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_delete_managed_login_branding(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_managed_login_branding_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.managed_login_branding_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'ManagedLoginBrandingId'",
            );
        }
        let branding_id = &input.managed_login_branding_id;

        let mut state = state.write().await;
        match state.delete_managed_login_branding(user_pool_id, branding_id) {
            Ok(()) => wire::serialize_delete_managed_login_branding_response(),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_describe_managed_login_branding(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_managed_login_branding_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.managed_login_branding_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'ManagedLoginBrandingId'",
            );
        }
        let branding_id = &input.managed_login_branding_id;

        let state = state.read().await;
        match state.describe_managed_login_branding(user_pool_id, branding_id) {
            Ok(b_opt) => wire::serialize_describe_managed_login_branding_response(
                &wire::DescribeManagedLoginBrandingResponse {
                    managed_login_branding: b_opt.map(managed_login_branding_wire),
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_describe_managed_login_branding_by_client(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_managed_login_branding_by_client_request(body)
        {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.client_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClientId'");
        }
        let client_id = &input.client_id;

        let state = state.read().await;
        match state.describe_managed_login_branding_by_client(user_pool_id, client_id) {
            Ok(b_opt) => wire::serialize_describe_managed_login_branding_by_client_response(
                &wire::DescribeManagedLoginBrandingByClientResponse {
                    managed_login_branding: b_opt.map(managed_login_branding_wire),
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_update_managed_login_branding(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_managed_login_branding_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let user_pool_id = match input.user_pool_id.as_deref() {
            Some(s) if !s.is_empty() => s,
            _ => return json_error_response(400, "ValidationException", "Missing 'UserPoolId'"),
        };
        let branding_id = match input.managed_login_branding_id.as_deref() {
            Some(s) if !s.is_empty() => s,
            _ => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing 'ManagedLoginBrandingId'",
                );
            }
        };
        let settings = input.settings.clone();

        let mut state = state.write().await;
        match state.update_managed_login_branding(user_pool_id, branding_id, settings) {
            Ok(b) => wire::serialize_update_managed_login_branding_response(
                &wire::UpdateManagedLoginBrandingResponse {
                    managed_login_branding: Some(managed_login_branding_wire(b)),
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- Risk Configuration ---

    async fn handle_describe_risk_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_risk_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;

        let state = state.read().await;
        match state.get_risk_configuration(user_pool_id) {
            Ok(_) => wire::serialize_describe_risk_configuration_response(
                &wire::DescribeRiskConfigurationResponse {
                    risk_configuration: Some(wire::RiskConfigurationType {
                        user_pool_id: Some(user_pool_id.to_string()),
                        ..Default::default()
                    }),
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_set_risk_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_set_risk_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        let config = serde_json::to_value(&input).unwrap_or(Value::Null);

        let mut state = state.write().await;
        match state.set_risk_configuration(user_pool_id, config) {
            Ok(_) => wire::serialize_set_risk_configuration_response(
                &wire::SetRiskConfigurationResponse {
                    risk_configuration: Some(wire::RiskConfigurationType {
                        user_pool_id: Some(user_pool_id.to_string()),
                        ..Default::default()
                    }),
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- Log Delivery ---

    async fn handle_get_log_delivery_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_log_delivery_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;

        let state = state.read().await;
        match state.get_log_delivery_configuration(user_pool_id) {
            Ok(configs_opt) => {
                let log_configs: Vec<wire::LogConfigurationType> = configs_opt
                    .map(|configs| {
                        configs
                            .iter()
                            .filter_map(|c| {
                                serde_json::from_value::<wire::LogConfigurationType>(c.clone()).ok()
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                wire::serialize_get_log_delivery_configuration_response(
                    &wire::GetLogDeliveryConfigurationResponse {
                        log_delivery_configuration: Some(wire::LogDeliveryConfigurationType {
                            log_configurations: Some(log_configs),
                            user_pool_id: Some(user_pool_id.to_string()),
                        }),
                    },
                )
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_set_log_delivery_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_set_log_delivery_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        let configs: Vec<serde_json::Value> = input
            .log_configurations
            .iter()
            .filter_map(|c| serde_json::to_value(c).ok())
            .collect();

        let mut state = state.write().await;
        match state.set_log_delivery_configuration(user_pool_id, configs.clone()) {
            Ok(()) => {
                let log_configs: Vec<wire::LogConfigurationType> = configs
                    .iter()
                    .filter_map(|c| {
                        serde_json::from_value::<wire::LogConfigurationType>(c.clone()).ok()
                    })
                    .collect();
                wire::serialize_set_log_delivery_configuration_response(
                    &wire::SetLogDeliveryConfigurationResponse {
                        log_delivery_configuration: Some(wire::LogDeliveryConfigurationType {
                            log_configurations: Some(log_configs),
                            user_pool_id: Some(user_pool_id.to_string()),
                        }),
                    },
                )
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- Resource Servers (missing ops) ---

    async fn handle_delete_resource_server(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_resource_server_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.identifier.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Identifier'");
        }
        let identifier = &input.identifier;

        let mut state = state.write().await;
        match state.delete_resource_server(user_pool_id, identifier) {
            Ok(()) => wire::serialize_delete_resource_server_response(),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_update_resource_server(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_resource_server_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.identifier.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Identifier'");
        }
        let identifier = &input.identifier;
        let name = if input.name.is_empty() {
            None
        } else {
            Some(input.name.as_str())
        };
        let scopes = input
            .scopes
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(|s| crate::types::ResourceServerScope {
                scope_name: s.scope_name,
                scope_description: s.scope_description,
            })
            .collect::<Vec<_>>();
        let scopes_opt = if scopes.is_empty() {
            None
        } else {
            Some(scopes)
        };

        let mut state = state.write().await;
        match state.update_resource_server(user_pool_id, identifier, name, scopes_opt) {
            Ok(rs) => wire::serialize_update_resource_server_response(
                &wire::UpdateResourceServerResponse {
                    resource_server: Some(resource_server_wire(rs)),
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- Identity Providers (missing ops) ---

    async fn handle_admin_disable_provider_for_user(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_disable_provider_for_user_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        let provider_name = input.user.provider_name.as_deref();
        let provider_attribute_value = input.user.provider_attribute_value.as_deref();

        let mut state = state.write().await;
        match state.admin_disable_provider_for_user(
            user_pool_id,
            provider_name,
            provider_attribute_value,
        ) {
            Ok(()) => wire::serialize_admin_disable_provider_for_user_response(
                &wire::AdminDisableProviderForUserResponse {},
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_admin_link_provider_for_user(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_link_provider_for_user_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        let destination_username = input
            .destination_user
            .provider_attribute_value
            .as_deref()
            .unwrap_or("");
        let source_provider_name = input.source_user.provider_name.clone();
        let source_attr_name = input.source_user.provider_attribute_name.clone();
        let source_attr_value = input.source_user.provider_attribute_value.clone();

        let mut state = state.write().await;
        match state.admin_link_provider_for_user(
            user_pool_id,
            destination_username,
            source_provider_name,
            source_attr_name,
            source_attr_value,
        ) {
            Ok(()) => wire::serialize_admin_link_provider_for_user_response(
                &wire::AdminLinkProviderForUserResponse {},
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_get_identity_provider_by_identifier(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_identity_provider_by_identifier_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.idp_identifier.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'IdpIdentifier'");
        }
        let idp_identifier = &input.idp_identifier;

        let state = state.read().await;
        match state.get_identity_provider_by_identifier(user_pool_id, idp_identifier) {
            Ok(idp) => wire::serialize_get_identity_provider_by_identifier_response(
                &wire::GetIdentityProviderByIdentifierResponse {
                    identity_provider: Some(idp_wire(idp)),
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- WebAuthn (stubs) ---

    // STUB[no-engine]: CompleteWebAuthnRegistration requires a real WebAuthn
    //   ceremony engine to validate the client attestation; always returns success.
    async fn handle_complete_web_authn_registration(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
    ) -> MockResponse {
        let _state = state.read().await;
        wire::serialize_complete_web_authn_registration_response(
            &wire::CompleteWebAuthnRegistrationResponse {},
        )
    }

    // WebAuthn credentials are not fully modelled but the handlers acknowledge requests.
    async fn handle_delete_web_authn_credential(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
    ) -> MockResponse {
        let _state = state.read().await;
        wire::serialize_delete_web_authn_credential_response(
            &wire::DeleteWebAuthnCredentialResponse {},
        )
    }

    async fn handle_list_web_authn_credentials(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
    ) -> MockResponse {
        let _state = state.read().await;
        wire::serialize_list_web_authn_credentials_response(
            &wire::ListWebAuthnCredentialsResponse {
                credentials: Some(vec![]),
                next_token: None,
            },
        )
    }

    // STUB[no-engine]: StartWebAuthnRegistration initiates a WebAuthn ceremony
    //   requiring real RP server logic; returns a minimal empty options object.
    async fn handle_start_web_authn_registration(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
    ) -> MockResponse {
        let _state = state.read().await;
        wire::serialize_start_web_authn_registration_response(
            &wire::StartWebAuthnRegistrationResponse {
                credential_creation_options: Some(serde_json::json!({})),
            },
        )
    }

    // --- Terms ---

    async fn handle_create_terms(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_terms_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = input.user_pool_id.clone();
        let terms_id = if input.terms_name.is_empty() {
            "default".to_string()
        } else {
            input.terms_name.clone()
        };
        let data = serde_json::to_value(&input).unwrap_or(Value::Null);

        let mut state = state.write().await;
        match state.create_terms(&user_pool_id, &terms_id, data) {
            Ok(_) => wire::serialize_create_terms_response(&wire::CreateTermsResponse {
                ..Default::default()
            }),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_delete_terms(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_terms_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.terms_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'TermsId'");
        }
        let terms_id = &input.terms_id;

        let mut state = state.write().await;
        match state.delete_terms(user_pool_id, terms_id) {
            Ok(()) => wire::serialize_delete_terms_response(),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_describe_terms(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_terms_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.terms_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'TermsId'");
        }
        let terms_id = &input.terms_id;

        let state = state.read().await;
        match state.describe_terms(user_pool_id, terms_id) {
            Ok(_) => wire::serialize_describe_terms_response(&wire::DescribeTermsResponse {
                ..Default::default()
            }),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_list_terms(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_terms_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;

        let state = state.read().await;
        match state.list_terms(user_pool_id) {
            Ok(_) => wire::serialize_list_terms_response(&wire::ListTermsResponse {
                ..Default::default()
            }),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_update_terms(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_terms_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = input.user_pool_id.clone();
        if input.terms_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'TermsId'");
        }
        let terms_id = input.terms_id.clone();
        let data = serde_json::to_value(&input).unwrap_or(Value::Null);

        let mut state = state.write().await;
        match state.update_terms(&user_pool_id, &terms_id, data) {
            Ok(_) => wire::serialize_update_terms_response(&wire::UpdateTermsResponse {
                ..Default::default()
            }),
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- Settings ---

    async fn handle_admin_set_user_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_admin_set_user_settings_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;

        let state = state.read().await;
        match state.admin_get_user(user_pool_id, username) {
            Ok(_) => wire::serialize_admin_set_user_settings_response(
                &wire::AdminSetUserSettingsResponse {},
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_get_signing_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_signing_certificate_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;

        let state = state.read().await;
        match state.describe_user_pool(user_pool_id) {
            Ok(_) => wire::serialize_get_signing_certificate_response(
                &wire::GetSigningCertificateResponse {
                    certificate: Some("-----BEGIN CERTIFICATE-----\nMIICpDCCAYwCCQDm5s8H1HXPMTANB\n-----END CERTIFICATE-----".to_string()),
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    // STUB[no-auth]: SetUserSettings (legacy) is token-based; the mock accepts
    //   any call and returns success without persisting MFA delivery preferences.
    async fn handle_set_user_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
    ) -> MockResponse {
        let _state = state.read().await;
        wire::serialize_set_user_settings_response(&wire::SetUserSettingsResponse {})
    }

    // --- User Pool Client Secrets ---

    async fn handle_add_user_pool_client_secret(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_add_user_pool_client_secret_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.client_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClientId'");
        }
        let client_id = &input.client_id;

        let state = state.read().await;
        match state.describe_user_pool_client(user_pool_id, client_id) {
            Ok(_) => {
                let now = chrono::Utc::now();
                let secret_id = uuid::Uuid::new_v4().to_string().replace('-', "");
                wire::serialize_add_user_pool_client_secret_response(
                    &wire::AddUserPoolClientSecretResponse {
                        client_secret_descriptor: Some(wire::ClientSecretDescriptorType {
                            client_secret_id: Some(secret_id.clone()),
                            client_secret_value: Some(secret_id),
                            client_secret_create_date: Some(now.timestamp() as f64),
                        }),
                    },
                )
            }
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_delete_user_pool_client_secret(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_user_pool_client_secret_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.client_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClientId'");
        }
        let client_id = &input.client_id;

        let state = state.read().await;
        match state.describe_user_pool_client(user_pool_id, client_id) {
            Ok(_) => wire::serialize_delete_user_pool_client_secret_response(
                &wire::DeleteUserPoolClientSecretResponse {},
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    async fn handle_list_user_pool_client_secrets(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_user_pool_client_secrets_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.user_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserPoolId'");
        }
        let user_pool_id = &input.user_pool_id;
        if input.client_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClientId'");
        }
        let client_id = &input.client_id;

        let state = state.read().await;
        match state.describe_user_pool_client(user_pool_id, client_id) {
            Ok(_) => wire::serialize_list_user_pool_client_secrets_response(
                &wire::ListUserPoolClientSecretsResponse {
                    client_secrets: Some(vec![]),
                    next_token: None,
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }

    // --- Resend Confirmation Code ---

    async fn handle_resend_confirmation_code(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdpState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_resend_confirmation_code_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.client_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ClientId'");
        }
        let client_id = &input.client_id;
        if input.username.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Username'");
        }
        let username = &input.username;

        let state = state.read().await;
        match state.resend_confirmation_code(client_id, username) {
            Ok(()) => wire::serialize_resend_confirmation_code_response(
                &wire::ResendConfirmationCodeResponse {
                    code_delivery_details: Some(wire::CodeDeliveryDetailsType {
                        delivery_medium: Some("EMAIL".to_string()),
                        destination: Some("t***@e***.com".to_string()),
                        attribute_name: Some("email".to_string()),
                    }),
                },
            ),
            Err(e) => cognito_error_response(&e),
        }
    }
}

// --- Helper functions ---

fn user_pool_wire(pool: &crate::types::UserPool) -> wire::UserPoolType {
    wire::UserPoolType {
        id: Some(pool.id.clone()),
        name: Some(pool.name.clone()),
        arn: Some(pool.arn.clone()),
        status: Some(pool.status.clone()),
        creation_date: Some(pool.created_date.timestamp() as f64),
        last_modified_date: Some(pool.created_date.timestamp() as f64),
        estimated_number_of_users: Some(0),
        // FIX(terraform-e2e): include default nested objects to prevent nil pointer
        // dereference in the terraform provider when it accesses these fields
        // after CreateUserPool / DescribeUserPool.
        policies: Some(wire::UserPoolPolicyType {
            password_policy: Some(wire::PasswordPolicyType {
                minimum_length: Some(8),
                require_lowercase: Some(false),
                require_numbers: Some(false),
                require_symbols: Some(false),
                require_uppercase: Some(false),
                temporary_password_validity_days: Some(7),
                ..Default::default()
            }),
            ..Default::default()
        }),
        admin_create_user_config: Some(wire::AdminCreateUserConfigType {
            allow_admin_create_user_only: Some(false),
            ..Default::default()
        }),
        account_recovery_setting: Some(wire::AccountRecoverySettingType {
            recovery_mechanisms: Some(vec![wire::RecoveryOptionType {
                name: "verified_email".to_string(),
                priority: 1,
            }]),
        }),
        ..Default::default()
    }
}

fn user_pool_client_wire(client: &crate::types::UserPoolClient) -> wire::UserPoolClientType {
    wire::UserPoolClientType {
        client_id: Some(client.id.clone()),
        client_name: Some(client.name.clone()),
        user_pool_id: Some(client.user_pool_id.clone()),
        client_secret: client.client_secret.clone(),
        explicit_auth_flows: if client.explicit_auth_flows.is_empty() {
            None
        } else {
            Some(client.explicit_auth_flows.clone())
        },
        allowed_o_auth_flows: if client.allowed_oauth_flows.is_empty() {
            None
        } else {
            Some(client.allowed_oauth_flows.clone())
        },
        allowed_o_auth_scopes: if client.allowed_oauth_scopes.is_empty() {
            None
        } else {
            Some(client.allowed_oauth_scopes.clone())
        },
        callback_u_r_ls: if client.callback_urls.is_empty() {
            None
        } else {
            Some(client.callback_urls.clone())
        },
        logout_u_r_ls: if client.logout_urls.is_empty() {
            None
        } else {
            Some(client.logout_urls.clone())
        },
        allowed_o_auth_flows_user_pool_client: if client.allowed_oauth_flows_user_pool_client {
            Some(true)
        } else {
            None
        },
        refresh_token_validity: client.refresh_token_validity,
        supported_identity_providers: if client.supported_identity_providers.is_empty() {
            None
        } else {
            Some(client.supported_identity_providers.clone())
        },
        ..Default::default()
    }
}

fn cognito_user_wire(user: &crate::types::CognitoUser) -> wire::UserType {
    let attrs = user_attributes_wire(&user.attributes);
    wire::UserType {
        username: Some(user.username.clone()),
        user_status: Some(user.status.clone()),
        user_create_date: Some(user.created_date.timestamp() as f64),
        user_last_modified_date: Some(user.created_date.timestamp() as f64),
        attributes: Some(attrs),
        enabled: Some(user.enabled),
        ..Default::default()
    }
}

fn user_attributes_wire(attributes: &HashMap<String, String>) -> Vec<wire::AttributeType> {
    attributes
        .iter()
        .map(|(k, v)| wire::AttributeType {
            name: k.clone(),
            value: Some(v.clone()),
        })
        .collect()
}

fn group_wire(group: &crate::types::Group) -> wire::GroupType {
    wire::GroupType {
        group_name: Some(group.group_name.clone()),
        user_pool_id: Some(group.user_pool_id.clone()),
        description: group.description.clone(),
        role_arn: group.role_arn.clone(),
        precedence: group.precedence,
        creation_date: Some(group.created_date.timestamp() as f64),
        last_modified_date: Some(group.last_modified_date.timestamp() as f64),
    }
}

fn idp_wire(idp: &crate::types::IdentityProvider) -> wire::IdentityProviderType {
    wire::IdentityProviderType {
        provider_name: Some(idp.provider_name.clone()),
        provider_type: Some(idp.provider_type.clone()),
        user_pool_id: Some(idp.user_pool_id.clone()),
        provider_details: Some(idp.provider_details.clone()),
        attribute_mapping: Some(idp.attribute_mapping.clone()),
        idp_identifiers: Some(idp.idp_identifiers.clone()),
        creation_date: Some(idp.created_date.timestamp() as f64),
        last_modified_date: Some(idp.last_modified_date.timestamp() as f64),
    }
}

fn resource_server_wire(rs: &crate::types::ResourceServer) -> wire::ResourceServerType {
    wire::ResourceServerType {
        identifier: Some(rs.identifier.clone()),
        name: Some(rs.name.clone()),
        user_pool_id: Some(rs.user_pool_id.clone()),
        scopes: Some(
            rs.scopes
                .iter()
                .map(|s| wire::ResourceServerScopeType {
                    scope_name: s.scope_name.clone(),
                    scope_description: s.scope_description.clone(),
                })
                .collect(),
        ),
    }
}

fn user_import_job_wire(job: &crate::types::UserImportJob) -> wire::UserImportJobType {
    wire::UserImportJobType {
        job_id: Some(job.job_id.clone()),
        job_name: Some(job.job_name.clone()),
        user_pool_id: Some(job.user_pool_id.clone()),
        pre_signed_url: job.pre_signed_url.clone(),
        creation_date: Some(job.created_date.timestamp() as f64),
        status: Some(job.status.clone()),
        ..Default::default()
    }
}

fn managed_login_branding_wire(
    b: &crate::types::ManagedLoginBranding,
) -> wire::ManagedLoginBrandingType {
    wire::ManagedLoginBrandingType {
        managed_login_branding_id: Some(b.branding_id.clone()),
        user_pool_id: Some(b.user_pool_id.clone()),
        settings: b.settings.clone(),
        creation_date: Some(b.creation_date.timestamp() as f64),
        last_modified_date: Some(b.last_modified_date.timestamp() as f64),
        ..Default::default()
    }
}

fn cognito_error_response(err: &CognitoIdpError) -> MockResponse {
    use CognitoIdpError::*;
    let (status, error_type) = match err {
        PoolNotFound(_) => (400u16, "ResourceNotFoundException"),
        UserNotFound => (400, "UserNotFoundException"),
        ClientNotFound(_) => (400, "ResourceNotFoundException"),
        UsernameExists => (400, "UsernameExistsException"),
        MissingUsername => (400, "InvalidParameterException"),
        MissingPassword => (400, "InvalidParameterException"),
        UserDisabled => (400, "NotAuthorizedException"),
        IncorrectPassword => (400, "NotAuthorizedException"),
        UnsupportedAuthFlow(_) => (400, "InvalidParameterException"),
        UserAlreadyExists => (400, "UsernameExistsException"),
        GroupExists(_) => (400, "GroupExistsException"),
        GroupNotFound(_) => (400, "ResourceNotFoundException"),
        DuplicateProvider(_) => (400, "DuplicateProviderException"),
        IdentityProviderNotFound(_) => (400, "ResourceNotFoundException"),
        ResourceServerAlreadyExists(_, _) => (400, "InvalidParameterException"),
        ResourceServerNotFound(_) => (400, "ResourceNotFoundException"),
        ResourceServerNotFoundAlt(_) => (400, "ResourceNotFoundException"),
        DomainAlreadyExists => (400, "InvalidParameterException"),
        DomainMismatch(_) => (400, "InvalidParameterException"),
        NoDomainExists => (400, "InvalidParameterException"),
        ResourceNotFound(_) => (400, "ResourceNotFoundException"),
        ImportJobNotFound(_) => (400, "ResourceNotFoundException"),
        IdentityProviderByIdentifierNotFound(_) => (400, "ResourceNotFoundException"),
    };
    let body = json!({
        "__type": error_type,
        "message": err.to_string(),
    });
    MockResponse::json(status, body.to_string())
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}
