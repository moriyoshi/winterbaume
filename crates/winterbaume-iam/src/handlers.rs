use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::state::{IamError, IamState};
use crate::types::Tag;
use crate::views::IamStateView;
use crate::wire;

/// IAM service handler that processes awsQuery protocol requests.
pub struct IamService {
    pub(crate) state: Arc<BackendState<IamState>>,
    pub(crate) notifier: StateChangeNotifier<IamStateView>,
}

impl IamService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for IamService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for IamService {
    fn service_name(&self) -> &str {
        "iam"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://iam\..*\.amazonaws\.com",
            r"https?://iam\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl IamService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        let body_str = std::str::from_utf8(&request.body).unwrap_or("");
        let params = parse_query_string(body_str);

        let action = match params.get("Action") {
            Some(a) => a.as_str(),
            None => {
                return MockResponse::error(400, "MissingAction", "Missing 'Action' parameter");
            }
        };

        let state = self.state.get(account_id, &region);

        match action {
            // User operations
            "CreateUser" => self.handle_create_user(&state, &params, account_id).await,
            "GetUser" => self.handle_get_user(&state, &params).await,
            "DeleteUser" => self.handle_delete_user(&state, &params).await,
            "ListUsers" => self.handle_list_users(&state, &params).await,
            "UpdateUser" => self.handle_update_user(&state, &params).await,
            "TagUser" => self.handle_tag_user(&state, &params).await,
            "UntagUser" => self.handle_untag_user(&state, &params).await,
            "ListUserTags" => self.handle_list_user_tags(&state, &params).await,
            "AttachUserPolicy" => self.handle_attach_user_policy(&state, &params).await,
            "DetachUserPolicy" => self.handle_detach_user_policy(&state, &params).await,
            "ListAttachedUserPolicies" => {
                self.handle_list_attached_user_policies(&state, &params)
                    .await
            }
            "PutUserPolicy" => self.handle_put_user_policy(&state, &params).await,
            "GetUserPolicy" => self.handle_get_user_policy(&state, &params).await,
            "DeleteUserPolicy" => self.handle_delete_user_policy(&state, &params).await,
            "ListUserPolicies" => self.handle_list_user_policies(&state, &params).await,

            // Role operations
            "CreateRole" => self.handle_create_role(&state, &params, account_id).await,
            "GetRole" => self.handle_get_role(&state, &params).await,
            "DeleteRole" => self.handle_delete_role(&state, &params).await,
            "ListRoles" => self.handle_list_roles(&state, &params).await,
            "UpdateRole" => self.handle_update_role(&state, &params).await,
            "UpdateRoleDescription" => self.handle_update_role_description(&state, &params).await,
            "UpdateAssumeRolePolicy" => {
                self.handle_update_assume_role_policy(&state, &params).await
            }
            "TagRole" => self.handle_tag_role(&state, &params).await,
            "UntagRole" => self.handle_untag_role(&state, &params).await,
            "ListRoleTags" => self.handle_list_role_tags(&state, &params).await,
            "AttachRolePolicy" => self.handle_attach_role_policy(&state, &params).await,
            "DetachRolePolicy" => self.handle_detach_role_policy(&state, &params).await,
            "ListAttachedRolePolicies" => {
                self.handle_list_attached_role_policies(&state, &params)
                    .await
            }
            "PutRolePolicy" => self.handle_put_role_policy(&state, &params).await,
            "GetRolePolicy" => self.handle_get_role_policy(&state, &params).await,
            "DeleteRolePolicy" => self.handle_delete_role_policy(&state, &params).await,
            "ListRolePolicies" => self.handle_list_role_policies(&state, &params).await,
            "PutRolePermissionsBoundary" => {
                self.handle_put_role_permissions_boundary(&state, &params)
                    .await
            }
            "DeleteRolePermissionsBoundary" => {
                self.handle_delete_role_permissions_boundary(&state, &params)
                    .await
            }
            "CreateServiceLinkedRole" => {
                self.handle_create_service_linked_role(&state, &params, account_id)
                    .await
            }
            "DeleteServiceLinkedRole" => {
                self.handle_delete_service_linked_role(&state, &params)
                    .await
            }
            "GetServiceLinkedRoleDeletionStatus" => {
                self.handle_get_service_linked_role_deletion_status(&state, &params)
                    .await
            }

            // Group operations
            "CreateGroup" => self.handle_create_group(&state, &params, account_id).await,
            "GetGroup" => self.handle_get_group(&state, &params).await,
            "DeleteGroup" => self.handle_delete_group(&state, &params).await,
            "ListGroups" => self.handle_list_groups(&state, &params).await,
            "UpdateGroup" => self.handle_update_group(&state, &params).await,
            "AddUserToGroup" => self.handle_add_user_to_group(&state, &params).await,
            "RemoveUserFromGroup" => self.handle_remove_user_from_group(&state, &params).await,
            "AttachGroupPolicy" => self.handle_attach_group_policy(&state, &params).await,
            "DetachGroupPolicy" => self.handle_detach_group_policy(&state, &params).await,
            "ListAttachedGroupPolicies" => {
                self.handle_list_attached_group_policies(&state, &params)
                    .await
            }
            "PutGroupPolicy" => self.handle_put_group_policy(&state, &params).await,
            "GetGroupPolicy" => self.handle_get_group_policy(&state, &params).await,
            "DeleteGroupPolicy" => self.handle_delete_group_policy(&state, &params).await,
            "ListGroupPolicies" => self.handle_list_group_policies(&state, &params).await,

            // Access key operations
            "CreateAccessKey" => self.handle_create_access_key(&state, &params).await,
            "ListAccessKeys" => self.handle_list_access_keys(&state, &params).await,
            "DeleteAccessKey" => self.handle_delete_access_key(&state, &params).await,
            "UpdateAccessKey" => self.handle_update_access_key(&state, &params).await,
            "GetAccessKeyLastUsed" => self.handle_get_access_key_last_used(&state, &params).await,

            // Policy operations
            "CreatePolicy" => self.handle_create_policy(&state, &params, account_id).await,
            "GetPolicy" => self.handle_get_policy(&state, &params).await,
            "DeletePolicy" => self.handle_delete_policy(&state, &params).await,
            "ListPolicies" => self.handle_list_policies(&state, &params).await,
            "CreatePolicyVersion" => self.handle_create_policy_version(&state, &params).await,
            "GetPolicyVersion" => self.handle_get_policy_version(&state, &params).await,
            "DeletePolicyVersion" => self.handle_delete_policy_version(&state, &params).await,
            "ListPolicyVersions" => self.handle_list_policy_versions(&state, &params).await,
            "SetDefaultPolicyVersion" => {
                self.handle_set_default_policy_version(&state, &params)
                    .await
            }
            "TagPolicy" => self.handle_tag_policy(&state, &params).await,
            "UntagPolicy" => self.handle_untag_policy(&state, &params).await,
            "ListPolicyTags" => self.handle_list_policy_tags(&state, &params).await,

            // Instance profile operations
            "CreateInstanceProfile" => {
                self.handle_create_instance_profile(&state, &params, account_id)
                    .await
            }
            "GetInstanceProfile" => self.handle_get_instance_profile(&state, &params).await,
            "DeleteInstanceProfile" => self.handle_delete_instance_profile(&state, &params).await,
            "AddRoleToInstanceProfile" => {
                self.handle_add_role_to_instance_profile(&state, &params)
                    .await
            }
            "RemoveRoleFromInstanceProfile" => {
                self.handle_remove_role_from_instance_profile(&state, &params)
                    .await
            }
            "TagInstanceProfile" => self.handle_tag_instance_profile(&state, &params).await,
            "UntagInstanceProfile" => self.handle_untag_instance_profile(&state, &params).await,
            "ListInstanceProfileTags" => {
                self.handle_list_instance_profile_tags(&state, &params)
                    .await
            }

            // Login profile operations
            "CreateLoginProfile" => self.handle_create_login_profile(&state, &params).await,
            "GetLoginProfile" => self.handle_get_login_profile(&state, &params).await,
            "UpdateLoginProfile" => self.handle_update_login_profile(&state, &params).await,
            "DeleteLoginProfile" => self.handle_delete_login_profile(&state, &params).await,

            // OpenID Connect provider operations
            "CreateOpenIDConnectProvider" => {
                self.handle_create_oidc_provider(&state, &params, account_id)
                    .await
            }
            "GetOpenIDConnectProvider" => self.handle_get_oidc_provider(&state, &params).await,
            "DeleteOpenIDConnectProvider" => {
                self.handle_delete_oidc_provider(&state, &params).await
            }
            "ListOpenIDConnectProviders" => self.handle_list_oidc_providers(&state).await,
            "UpdateOpenIDConnectProviderThumbprint" => {
                self.handle_update_oidc_provider_thumbprint(&state, &params)
                    .await
            }
            "TagOpenIDConnectProvider" => self.handle_tag_oidc_provider(&state, &params).await,
            "UntagOpenIDConnectProvider" => self.handle_untag_oidc_provider(&state, &params).await,
            "ListOpenIDConnectProviderTags" => {
                self.handle_list_oidc_provider_tags(&state, &params).await
            }

            // SAML provider operations
            "CreateSAMLProvider" => {
                self.handle_create_saml_provider(&state, &params, account_id)
                    .await
            }
            "GetSAMLProvider" => self.handle_get_saml_provider(&state, &params).await,
            "DeleteSAMLProvider" => self.handle_delete_saml_provider(&state, &params).await,
            "UpdateSAMLProvider" => self.handle_update_saml_provider(&state, &params).await,
            "ListSAMLProviders" => self.handle_list_saml_providers(&state).await,

            // MFA operations
            "CreateVirtualMFADevice" => {
                self.handle_create_virtual_mfa_device(&state, &params, account_id)
                    .await
            }
            "DeleteVirtualMFADevice" => {
                self.handle_delete_virtual_mfa_device(&state, &params).await
            }
            "ListVirtualMFADevices" => self.handle_list_virtual_mfa_devices(&state).await,
            "EnableMFADevice" => self.handle_enable_mfa_device(&state, &params).await,
            "DeactivateMFADevice" => self.handle_deactivate_mfa_device(&state, &params).await,
            "ListMFADevices" => self.handle_list_mfa_devices(&state, &params).await,

            // Password policy operations
            "UpdateAccountPasswordPolicy" => {
                self.handle_update_account_password_policy(&state, &params)
                    .await
            }
            "GetAccountPasswordPolicy" => self.handle_get_account_password_policy(&state).await,
            "DeleteAccountPasswordPolicy" => {
                self.handle_delete_account_password_policy(&state).await
            }

            // Account alias operations
            "CreateAccountAlias" => self.handle_create_account_alias(&state, &params).await,
            "DeleteAccountAlias" => self.handle_delete_account_alias(&state, &params).await,
            "ListAccountAliases" => self.handle_list_account_aliases(&state).await,

            // Server certificate operations
            "UploadServerCertificate" => {
                self.handle_upload_server_certificate(&state, &params, account_id)
                    .await
            }
            "GetServerCertificate" => self.handle_get_server_certificate(&state, &params).await,
            "DeleteServerCertificate" => {
                self.handle_delete_server_certificate(&state, &params).await
            }
            "ListServerCertificates" => self.handle_list_server_certificates(&state).await,

            // SSH public key operations
            "UploadSSHPublicKey" => self.handle_upload_ssh_public_key(&state, &params).await,
            "GetSSHPublicKey" => self.handle_get_ssh_public_key(&state, &params).await,
            "UpdateSSHPublicKey" => self.handle_update_ssh_public_key(&state, &params).await,
            "DeleteSSHPublicKey" => self.handle_delete_ssh_public_key(&state, &params).await,

            // Signing certificate operations
            "UploadSigningCertificate" => {
                self.handle_upload_signing_certificate(&state, &params)
                    .await
            }
            "ListSigningCertificates" => {
                self.handle_list_signing_certificates(&state, &params).await
            }
            "UpdateSigningCertificate" => {
                self.handle_update_signing_certificate(&state, &params)
                    .await
            }
            "DeleteSigningCertificate" => {
                self.handle_delete_signing_certificate(&state, &params)
                    .await
            }

            // Account summary / reports
            "GetAccountSummary" => self.handle_get_account_summary(&state).await,
            "GetCredentialReport" => self.handle_get_credential_report(&state).await,
            "GetAccountAuthorizationDetails" => {
                self.handle_get_account_authorization_details(&state).await
            }

            // Delegation operations (stub)
            "AcceptDelegationRequest" => self.handle_accept_delegation_request().await,
            "AssociateDelegationRequest" => self.handle_associate_delegation_request().await,
            "CreateDelegationRequest" => self.handle_create_delegation_request().await,
            "GetDelegationRequest" => self.handle_get_delegation_request().await,
            "ListDelegationRequests" => self.handle_list_delegation_requests().await,
            "RejectDelegationRequest" => self.handle_reject_delegation_request().await,
            "SendDelegationToken" => self.handle_send_delegation_token().await,
            "UpdateDelegationRequest" => self.handle_update_delegation_request().await,

            // Password / credential operations (stub)
            "ChangePassword" => self.handle_change_password().await,
            "GenerateCredentialReport" => self.handle_generate_credential_report(&state).await,
            "GenerateOrganizationsAccessReport" => {
                self.handle_generate_organizations_access_report().await
            }
            "GetOrganizationsAccessReport" => self.handle_get_organizations_access_report().await,

            // Service-last-accessed details (stub)
            "GenerateServiceLastAccessedDetails" => {
                self.handle_generate_service_last_accessed_details(&state, &params)
                    .await
            }
            "GetServiceLastAccessedDetails" => {
                self.handle_get_service_last_accessed_details(&state, &params)
                    .await
            }
            "GetServiceLastAccessedDetailsWithEntities" => {
                self.handle_get_service_last_accessed_details_with_entities(&state, &params)
                    .await
            }

            // Service-specific credentials
            "CreateServiceSpecificCredential" => {
                self.handle_create_service_specific_credential(&state, &params)
                    .await
            }
            "DeleteServiceSpecificCredential" => {
                self.handle_delete_service_specific_credential(&state, &params)
                    .await
            }
            "ListServiceSpecificCredentials" => {
                self.handle_list_service_specific_credentials(&state, &params)
                    .await
            }
            "ResetServiceSpecificCredential" => {
                self.handle_reset_service_specific_credential(&state, &params)
                    .await
            }
            "UpdateServiceSpecificCredential" => {
                self.handle_update_service_specific_credential(&state, &params)
                    .await
            }

            // Context key operations (stub)
            "GetContextKeysForCustomPolicy" => {
                self.handle_get_context_keys_for_custom_policy(&params)
                    .await
            }
            "GetContextKeysForPrincipalPolicy" => {
                self.handle_get_context_keys_for_principal_policy(&params)
                    .await
            }

            // Simulation operations
            "SimulateCustomPolicy" => self.handle_simulate_custom_policy(&params),
            "SimulatePrincipalPolicy" => {
                self.handle_simulate_principal_policy(&state, &params).await
            }

            // MFA device operations
            "GetMFADevice" => self.handle_get_m_f_a_device(&state, &params).await,
            "TagMFADevice" => self.handle_tag_m_f_a_device(&state, &params).await,
            "UntagMFADevice" => self.handle_untag_m_f_a_device(&state, &params).await,
            "ListMFADeviceTags" => self.handle_list_m_f_a_device_tags(&state, &params).await,
            "ResyncMFADevice" => self.handle_resync_m_f_a_device(&state, &params).await,

            // SAML provider tagging
            "TagSAMLProvider" => self.handle_tag_s_a_m_l_provider(&state, &params).await,
            "UntagSAMLProvider" => self.handle_untag_s_a_m_l_provider(&state, &params).await,
            "ListSAMLProviderTags" => {
                self.handle_list_s_a_m_l_provider_tags(&state, &params)
                    .await
            }

            // Server certificate tagging
            "TagServerCertificate" => self.handle_tag_server_certificate(&state, &params).await,
            "UntagServerCertificate" => self.handle_untag_server_certificate(&state, &params).await,
            "ListServerCertificateTags" => {
                self.handle_list_server_certificate_tags(&state, &params)
                    .await
            }
            "UpdateServerCertificate" => {
                self.handle_update_server_certificate(&state, &params).await
            }

            // SSH public keys listing
            "ListSSHPublicKeys" => self.handle_list_s_s_h_public_keys(&state, &params).await,

            // Instance profiles listing
            "ListInstanceProfiles" => self.handle_list_instance_profiles(&state, &params).await,
            "ListInstanceProfilesForRole" => {
                self.handle_list_instance_profiles_for_role(&state, &params)
                    .await
            }

            // Groups for user
            "ListGroupsForUser" => self.handle_list_groups_for_user(&state, &params).await,

            // Entities for policy
            "ListEntitiesForPolicy" => self.handle_list_entities_for_policy(&state, &params).await,

            // Policies granting service access (stub)
            "ListPoliciesGrantingServiceAccess" => {
                self.handle_list_policies_granting_service_access(&params)
                    .await
            }

            // Organizations features (stub)
            "ListOrganizationsFeatures" => self.handle_list_organizations_features().await,

            // Outbound web identity federation (stub)
            "DisableOutboundWebIdentityFederation" => {
                self.handle_disable_outbound_web_identity_federation().await
            }
            "EnableOutboundWebIdentityFederation" => {
                self.handle_enable_outbound_web_identity_federation().await
            }
            "GetOutboundWebIdentityFederationInfo" => {
                self.handle_get_outbound_web_identity_federation_info()
                    .await
            }

            // Organizations root operations (stub)
            "DisableOrganizationsRootCredentialsManagement" => {
                self.handle_disable_organizations_root_credentials_management(&state)
                    .await
            }
            "DisableOrganizationsRootSessions" => {
                self.handle_disable_organizations_root_sessions(&state)
                    .await
            }
            "EnableOrganizationsRootCredentialsManagement" => {
                self.handle_enable_organizations_root_credentials_management(&state)
                    .await
            }
            "EnableOrganizationsRootSessions" => {
                self.handle_enable_organizations_root_sessions(&state).await
            }

            // Human readable summary (stub)
            "GetHumanReadableSummary" => self.handle_get_human_readable_summary().await,

            // User permissions boundary
            "PutUserPermissionsBoundary" => {
                self.handle_put_user_permissions_boundary(&state, &params)
                    .await
            }
            "DeleteUserPermissionsBoundary" => {
                self.handle_delete_user_permissions_boundary(&state, &params)
                    .await
            }

            // OIDC provider: add/remove client ID
            "AddClientIDToOpenIDConnectProvider" => {
                self.handle_add_client_i_d_to_open_i_d_connect_provider(&state, &params)
                    .await
            }
            "RemoveClientIDFromOpenIDConnectProvider" => {
                self.handle_remove_client_i_d_from_open_i_d_connect_provider(&state, &params)
                    .await
            }

            // STS preferences (stub)
            "SetSecurityTokenServicePreferences" => {
                self.handle_set_security_token_service_preferences().await
            }

            _ => MockResponse::error(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for IAM"),
            ),
        }
    }

    // ==================== User handlers ====================

    async fn handle_create_user(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_user_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        let path = input.path.as_deref().unwrap_or("/");
        let tags: Vec<Tag> = input
            .tags
            .map(|tl| {
                tl.items
                    .into_iter()
                    .map(|t| Tag {
                        key: t.key,
                        value: t.value,
                    })
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.create_user(account_id, &input.user_name, path, tags) {
            Ok(user) => {
                let xml = format!(
                    r#"<CreateUserResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <CreateUserResult>
    <User>
      <Path>{path}</Path>
      <UserName>{user_name}</UserName>
      <UserId>{user_id}</UserId>
      <Arn>{arn}</Arn>
      <CreateDate>{create_date}</CreateDate>
    </User>
  </CreateUserResult>
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateUserResponse>"#,
                    path = xml_escape(&user.path),
                    user_name = xml_escape(&user.name),
                    user_id = xml_escape(&user.user_id),
                    arn = xml_escape(&user.arn),
                    create_date = user.create_date.format("%Y-%m-%dT%H:%M:%SZ"),
                    request_id = uuid::Uuid::new_v4(),
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_get_user(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_user_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let user_name = match input.user_name.as_deref() {
            Some(v) if !v.is_empty() => v,
            _ => return MockResponse::error(400, "MissingParameter", "Missing 'UserName'"),
        };

        let state = state.read().await;
        match state.get_user(user_name) {
            Ok(user) => {
                let xml = format!(
                    r#"<GetUserResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <GetUserResult>
    <User>
      <Path>{path}</Path>
      <UserName>{user_name}</UserName>
      <UserId>{user_id}</UserId>
      <Arn>{arn}</Arn>
      <CreateDate>{create_date}</CreateDate>
    </User>
  </GetUserResult>
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetUserResponse>"#,
                    path = xml_escape(&user.path),
                    user_name = xml_escape(&user.name),
                    user_id = xml_escape(&user.user_id),
                    arn = xml_escape(&user.arn),
                    create_date = user.create_date.format("%Y-%m-%dT%H:%M:%SZ"),
                    request_id = uuid::Uuid::new_v4(),
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_delete_user(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_user_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }

        let mut state = state.write().await;
        match state.delete_user(&input.user_name) {
            Ok(()) => {
                let xml = format!(
                    r#"<DeleteUserResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteUserResponse>"#,
                    request_id = uuid::Uuid::new_v4(),
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_users(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_users_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let path_prefix = input.path_prefix.as_deref();

        let state = state.read().await;
        let users = state.list_users(path_prefix);

        let mut members_xml = String::new();
        for user in &users {
            members_xml.push_str(&format!(
                r#"      <member>
        <Path>{path}</Path>
        <UserName>{user_name}</UserName>
        <UserId>{user_id}</UserId>
        <Arn>{arn}</Arn>
        <CreateDate>{create_date}</CreateDate>
      </member>
"#,
                path = xml_escape(&user.path),
                user_name = xml_escape(&user.name),
                user_id = xml_escape(&user.user_id),
                arn = xml_escape(&user.arn),
                create_date = user.create_date.format("%Y-%m-%dT%H:%M:%SZ"),
            ));
        }

        let xml = format!(
            r#"<ListUsersResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ListUsersResult>
    <IsTruncated>false</IsTruncated>
    <Users>
{members_xml}    </Users>
  </ListUsersResult>
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListUsersResponse>"#,
            members_xml = members_xml,
            request_id = uuid::Uuid::new_v4(),
        );
        MockResponse::xml(200, xml)
    }

    async fn handle_update_user(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_user_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        let new_user_name = input.new_user_name.as_deref();
        let new_path = input.new_path.as_deref();

        let mut state = state.write().await;
        match state.update_user(&input.user_name, new_user_name, new_path) {
            Ok(()) => wire::serialize_update_user_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_tag_user(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_user_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        let tags: Vec<Tag> = input
            .tags
            .items
            .into_iter()
            .map(|t| Tag {
                key: t.key,
                value: t.value,
            })
            .collect();

        let mut state = state.write().await;
        match state.tag_user(&input.user_name, tags) {
            Ok(()) => {
                let xml = format!(
                    r#"<TagUserResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</TagUserResponse>"#,
                    request_id = uuid::Uuid::new_v4(),
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_untag_user(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_user_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        let tag_keys = input.tag_keys.items;

        let mut state = state.write().await;
        match state.untag_user(&input.user_name, &tag_keys) {
            Ok(()) => {
                let xml = format!(
                    r#"<UntagUserResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UntagUserResponse>"#,
                    request_id = uuid::Uuid::new_v4(),
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_user_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_user_tags_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }

        let state = state.read().await;
        match state.list_user_tags(&input.user_name) {
            Ok(tags) => {
                let mut members_xml = String::new();
                for tag in tags {
                    members_xml.push_str(&format!(
                        r#"      <member>
        <Key>{key}</Key>
        <Value>{value}</Value>
      </member>
"#,
                        key = xml_escape(&tag.key),
                        value = xml_escape(&tag.value),
                    ));
                }

                let xml = format!(
                    r#"<ListUserTagsResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ListUserTagsResult>
    <IsTruncated>false</IsTruncated>
    <Tags>
{members_xml}    </Tags>
  </ListUserTagsResult>
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListUserTagsResponse>"#,
                    members_xml = members_xml,
                    request_id = uuid::Uuid::new_v4(),
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_attach_user_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_attach_user_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        if input.policy_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyArn'");
        }

        let mut state = state.write().await;
        match state.attach_user_policy(&input.user_name, &input.policy_arn) {
            Ok(()) => wire::serialize_attach_user_policy_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_detach_user_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_detach_user_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        if input.policy_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyArn'");
        }

        let mut state = state.write().await;
        match state.detach_user_policy(&input.user_name, &input.policy_arn) {
            Ok(()) => wire::serialize_detach_user_policy_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_attached_user_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_attached_user_policies_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }

        let state = state.read().await;
        match state.list_attached_user_policies(&input.user_name) {
            Ok(policies) => {
                let members: Vec<wire::AttachedPolicy> = policies
                    .iter()
                    .map(|p| wire::AttachedPolicy {
                        policy_arn: Some(p.policy_arn.clone()),
                        policy_name: Some(p.policy_name.clone()),
                    })
                    .collect();
                wire::serialize_list_attached_user_policies_response(
                    &wire::ListAttachedUserPoliciesResponse {
                        attached_policies: Some(members.into()),
                        ..Default::default()
                    },
                )
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_put_user_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_user_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        if input.policy_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyName'");
        }
        if input.policy_document.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyDocument'");
        }

        let mut state = state.write().await;
        match state.put_user_policy(&input.user_name, &input.policy_name, &input.policy_document) {
            Ok(()) => wire::serialize_put_user_policy_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_get_user_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_user_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        if input.policy_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyName'");
        }

        let state = state.read().await;
        match state.get_user_policy(&input.user_name, &input.policy_name) {
            Ok(policy) => wire::serialize_get_user_policy_response(&wire::GetUserPolicyResponse {
                user_name: Some(input.user_name.clone()),
                policy_name: Some(policy.policy_name.clone()),
                policy_document: Some(policy.policy_document.clone()),
            }),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_delete_user_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_user_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        if input.policy_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyName'");
        }

        let mut state = state.write().await;
        match state.delete_user_policy(&input.user_name, &input.policy_name) {
            Ok(()) => wire::serialize_delete_user_policy_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_user_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_user_policies_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }

        let state = state.read().await;
        match state.list_user_policies(&input.user_name) {
            Ok(names) => {
                let mut members_xml = String::new();
                for name in &names {
                    members_xml
                        .push_str(&format!("      <member>{}</member>\n", xml_escape(name),));
                }
                let xml = format!(
                    r#"<ListUserPoliciesResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ListUserPoliciesResult>
    <IsTruncated>false</IsTruncated>
    <PolicyNames>
{members_xml}    </PolicyNames>
  </ListUserPoliciesResult>
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListUserPoliciesResponse>"#,
                    members_xml = members_xml,
                    request_id = uuid::Uuid::new_v4(),
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => iam_error_response(&e),
        }
    }

    // ==================== Role handlers ====================

    async fn handle_create_role(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_role_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.role_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RoleName'");
        }
        if input.assume_role_policy_document.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'AssumeRolePolicyDocument'",
            );
        }
        let path = input.path.as_deref().unwrap_or("/");
        let description = input.description.as_deref().unwrap_or("");
        let max_session_duration: i32 = input.max_session_duration.unwrap_or(3600);
        let tags: Vec<Tag> = input
            .tags
            .map(|tl| {
                tl.items
                    .into_iter()
                    .map(|t| Tag {
                        key: t.key,
                        value: t.value,
                    })
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.create_role(
            account_id,
            &input.role_name,
            path,
            &input.assume_role_policy_document,
            description,
            max_session_duration,
            tags,
        ) {
            Ok(role) => {
                let xml = format!(
                    r#"<CreateRoleResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <CreateRoleResult>
    <Role>
      <Path>{path}</Path>
      <RoleName>{role_name}</RoleName>
      <RoleId>{role_id}</RoleId>
      <Arn>{arn}</Arn>
      <CreateDate>{create_date}</CreateDate>
      <AssumeRolePolicyDocument>{policy_doc}</AssumeRolePolicyDocument>
      <MaxSessionDuration>{max_session_duration}</MaxSessionDuration>
    </Role>
  </CreateRoleResult>
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateRoleResponse>"#,
                    path = xml_escape(&role.path),
                    role_name = xml_escape(&role.name),
                    role_id = xml_escape(&role.role_id),
                    arn = xml_escape(&role.arn),
                    create_date = role.create_date.format("%Y-%m-%dT%H:%M:%SZ"),
                    policy_doc = xml_escape(&role.assume_role_policy_document),
                    max_session_duration = role.max_session_duration,
                    request_id = uuid::Uuid::new_v4(),
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_get_role(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_role_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.role_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RoleName'");
        }

        let state = state.read().await;
        match state.get_role(&input.role_name) {
            Ok(role) => {
                let xml = format!(
                    r#"<GetRoleResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <GetRoleResult>
    <Role>
      <Path>{path}</Path>
      <RoleName>{role_name}</RoleName>
      <RoleId>{role_id}</RoleId>
      <Arn>{arn}</Arn>
      <CreateDate>{create_date}</CreateDate>
      <AssumeRolePolicyDocument>{policy_doc}</AssumeRolePolicyDocument>
      <MaxSessionDuration>{max_session_duration}</MaxSessionDuration>
    </Role>
  </GetRoleResult>
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetRoleResponse>"#,
                    path = xml_escape(&role.path),
                    role_name = xml_escape(&role.name),
                    role_id = xml_escape(&role.role_id),
                    arn = xml_escape(&role.arn),
                    create_date = role.create_date.format("%Y-%m-%dT%H:%M:%SZ"),
                    policy_doc = xml_escape(&role.assume_role_policy_document),
                    max_session_duration = role.max_session_duration,
                    request_id = uuid::Uuid::new_v4(),
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_delete_role(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_role_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.role_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RoleName'");
        }

        let mut state = state.write().await;
        match state.delete_role(&input.role_name) {
            Ok(()) => {
                let xml = format!(
                    r#"<DeleteRoleResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteRoleResponse>"#,
                    request_id = uuid::Uuid::new_v4(),
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_roles(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_roles_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let path_prefix = input.path_prefix.as_deref();

        let state = state.read().await;
        let roles = state.list_roles(path_prefix);

        let mut members_xml = String::new();
        for role in &roles {
            members_xml.push_str(&format!(
                r#"      <member>
        <Path>{path}</Path>
        <RoleName>{role_name}</RoleName>
        <RoleId>{role_id}</RoleId>
        <Arn>{arn}</Arn>
        <CreateDate>{create_date}</CreateDate>
        <AssumeRolePolicyDocument>{policy_doc}</AssumeRolePolicyDocument>
        <MaxSessionDuration>{max_session_duration}</MaxSessionDuration>
      </member>
"#,
                path = xml_escape(&role.path),
                role_name = xml_escape(&role.name),
                role_id = xml_escape(&role.role_id),
                arn = xml_escape(&role.arn),
                create_date = role.create_date.format("%Y-%m-%dT%H:%M:%SZ"),
                policy_doc = xml_escape(&role.assume_role_policy_document),
                max_session_duration = role.max_session_duration,
            ));
        }

        let xml = format!(
            r#"<ListRolesResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ListRolesResult>
    <IsTruncated>false</IsTruncated>
    <Roles>
{members_xml}    </Roles>
  </ListRolesResult>
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListRolesResponse>"#,
            members_xml = members_xml,
            request_id = uuid::Uuid::new_v4(),
        );
        MockResponse::xml(200, xml)
    }

    async fn handle_update_role(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_role_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.role_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RoleName'");
        }
        let description = input.description.as_deref();
        let max_session_duration = input.max_session_duration;

        let mut state = state.write().await;
        match state.update_role(&input.role_name, description, max_session_duration) {
            Ok(()) => wire::serialize_update_role_response(&wire::UpdateRoleResponse {}),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_update_role_description(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_role_description_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.role_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RoleName'");
        }
        if input.description.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'Description'");
        }

        let mut state = state.write().await;
        match state.update_role_description(&input.role_name, &input.description) {
            Ok(role) => wire::serialize_update_role_description_response(
                &wire::UpdateRoleDescriptionResponse {
                    role: Some(make_wire_role(role)),
                },
            ),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_update_assume_role_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_assume_role_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.role_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RoleName'");
        }
        if input.policy_document.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyDocument'");
        }

        let mut state = state.write().await;
        match state.update_assume_role_policy(&input.role_name, &input.policy_document) {
            Ok(()) => wire::serialize_update_assume_role_policy_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_tag_role(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_role_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.role_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RoleName'");
        }
        let tags: Vec<Tag> = input
            .tags
            .items
            .into_iter()
            .map(|t| Tag {
                key: t.key,
                value: t.value,
            })
            .collect();

        let mut state = state.write().await;
        match state.tag_role(&input.role_name, tags) {
            Ok(()) => wire::serialize_tag_role_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_untag_role(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_role_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.role_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RoleName'");
        }
        let tag_keys = input.tag_keys.items;

        let mut state = state.write().await;
        match state.untag_role(&input.role_name, &tag_keys) {
            Ok(()) => wire::serialize_untag_role_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_role_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_role_tags_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.role_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RoleName'");
        }

        let state = state.read().await;
        match state.list_role_tags(&input.role_name) {
            Ok(tags) => {
                let wire_tags = tags_to_wire(tags);
                wire::serialize_list_role_tags_response(&wire::ListRoleTagsResponse {
                    tags: Some(wire_tags),
                    ..Default::default()
                })
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_attach_role_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_attach_role_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.role_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RoleName'");
        }
        if input.policy_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyArn'");
        }

        let mut state = state.write().await;
        match state.attach_role_policy(&input.role_name, &input.policy_arn) {
            Ok(()) => wire::serialize_attach_role_policy_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_detach_role_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_detach_role_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.role_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RoleName'");
        }
        if input.policy_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyArn'");
        }

        let mut state = state.write().await;
        match state.detach_role_policy(&input.role_name, &input.policy_arn) {
            Ok(()) => wire::serialize_detach_role_policy_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_attached_role_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_attached_role_policies_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.role_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RoleName'");
        }

        let state = state.read().await;
        match state.list_attached_role_policies(&input.role_name) {
            Ok(policies) => {
                let members: Vec<wire::AttachedPolicy> = policies
                    .iter()
                    .map(|p| wire::AttachedPolicy {
                        policy_arn: Some(p.policy_arn.clone()),
                        policy_name: Some(p.policy_name.clone()),
                    })
                    .collect();
                wire::serialize_list_attached_role_policies_response(
                    &wire::ListAttachedRolePoliciesResponse {
                        attached_policies: Some(members.into()),
                        ..Default::default()
                    },
                )
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_put_role_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_role_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.role_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RoleName'");
        }
        if input.policy_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyName'");
        }
        if input.policy_document.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyDocument'");
        }

        let mut state = state.write().await;
        match state.put_role_policy(&input.role_name, &input.policy_name, &input.policy_document) {
            Ok(()) => wire::serialize_put_role_policy_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_get_role_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_role_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.role_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RoleName'");
        }
        if input.policy_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyName'");
        }

        let state = state.read().await;
        match state.get_role_policy(&input.role_name, &input.policy_name) {
            Ok(policy) => wire::serialize_get_role_policy_response(&wire::GetRolePolicyResponse {
                role_name: Some(input.role_name.clone()),
                policy_name: Some(policy.policy_name.clone()),
                policy_document: Some(policy.policy_document.clone()),
            }),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_delete_role_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_role_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.role_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RoleName'");
        }
        if input.policy_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyName'");
        }

        let mut state = state.write().await;
        match state.delete_role_policy(&input.role_name, &input.policy_name) {
            Ok(()) => wire::serialize_delete_role_policy_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_role_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_role_policies_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.role_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RoleName'");
        }

        let state = state.read().await;
        match state.list_role_policies(&input.role_name) {
            Ok(names) => {
                let mut members_xml = String::new();
                for name in &names {
                    members_xml
                        .push_str(&format!("      <member>{}</member>\n", xml_escape(name),));
                }
                let xml = format!(
                    r#"<ListRolePoliciesResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ListRolePoliciesResult>
    <IsTruncated>false</IsTruncated>
    <PolicyNames>
{members_xml}    </PolicyNames>
  </ListRolePoliciesResult>
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListRolePoliciesResponse>"#,
                    members_xml = members_xml,
                    request_id = uuid::Uuid::new_v4(),
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_put_role_permissions_boundary(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_role_permissions_boundary_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.role_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RoleName'");
        }
        if input.permissions_boundary.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PermissionsBoundary'");
        }

        let mut state = state.write().await;
        match state.put_role_permissions_boundary(&input.role_name, &input.permissions_boundary) {
            Ok(()) => wire::serialize_put_role_permissions_boundary_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_delete_role_permissions_boundary(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_role_permissions_boundary_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.role_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RoleName'");
        }

        let mut state = state.write().await;
        match state.delete_role_permissions_boundary(&input.role_name) {
            Ok(()) => wire::serialize_delete_role_permissions_boundary_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_create_service_linked_role(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_service_linked_role_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.a_w_s_service_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'AWSServiceName'");
        }
        let description = input.description.as_deref().unwrap_or("");
        let custom_suffix = input.custom_suffix.as_deref();

        let mut state = state.write().await;
        match state.create_service_linked_role(
            account_id,
            &input.a_w_s_service_name,
            description,
            custom_suffix,
        ) {
            Ok(role) => wire::serialize_create_service_linked_role_response(
                &wire::CreateServiceLinkedRoleResponse {
                    role: Some(make_wire_role(role)),
                },
            ),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_delete_service_linked_role(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_service_linked_role_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.role_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RoleName'");
        }

        let mut state = state.write().await;
        match state.delete_service_linked_role(&input.role_name) {
            Ok(task_id) => wire::serialize_delete_service_linked_role_response(
                &wire::DeleteServiceLinkedRoleResponse {
                    deletion_task_id: Some(task_id),
                },
            ),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_get_service_linked_role_deletion_status(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_service_linked_role_deletion_status_request(params)
        {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.deletion_task_id.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'DeletionTaskId'");
        }

        let state = state.read().await;
        match state.get_service_linked_role_deletion_status(&input.deletion_task_id) {
            Ok(status) => wire::serialize_get_service_linked_role_deletion_status_response(
                &wire::GetServiceLinkedRoleDeletionStatusResponse {
                    status: Some(status.to_string()),
                    ..Default::default()
                },
            ),
            Err(e) => iam_error_response(&e),
        }
    }

    // ==================== Group handlers ====================

    async fn handle_create_group(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.group_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'GroupName'");
        }
        let path = input.path.as_deref().unwrap_or("/");

        let mut state = state.write().await;
        match state.create_group(account_id, &input.group_name, path) {
            Ok(group) => wire::serialize_create_group_response(&wire::CreateGroupResponse {
                group: Some(wire::Group {
                    arn: Some(group.arn.clone()),
                    create_date: Some(group.create_date.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                    group_id: Some(group.group_id.clone()),
                    group_name: Some(group.name.clone()),
                    path: Some(group.path.clone()),
                }),
            }),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_get_group(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.group_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'GroupName'");
        }

        let state = state.read().await;
        match state.get_group(&input.group_name) {
            Ok((group, users)) => {
                let mut users_xml = String::new();
                for u in &users {
                    users_xml.push_str(&format!(
                        r#"      <member>
        <Path>{path}</Path>
        <UserName>{user_name}</UserName>
        <UserId>{user_id}</UserId>
        <Arn>{arn}</Arn>
        <CreateDate>{create_date}</CreateDate>
      </member>
"#,
                        path = xml_escape(&u.path),
                        user_name = xml_escape(&u.name),
                        user_id = xml_escape(&u.user_id),
                        arn = xml_escape(&u.arn),
                        create_date = u.create_date.format("%Y-%m-%dT%H:%M:%SZ"),
                    ));
                }
                let xml = format!(
                    r#"<GetGroupResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <GetGroupResult>
    <Group>
      <GroupName>{group_name}</GroupName>
      <GroupId>{group_id}</GroupId>
      <Arn>{arn}</Arn>
      <Path>{path}</Path>
      <CreateDate>{create_date}</CreateDate>
    </Group>
    <IsTruncated>false</IsTruncated>
    <Users>
{users_xml}    </Users>
  </GetGroupResult>
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetGroupResponse>"#,
                    group_name = xml_escape(&group.name),
                    group_id = xml_escape(&group.group_id),
                    arn = xml_escape(&group.arn),
                    path = xml_escape(&group.path),
                    create_date = group.create_date.format("%Y-%m-%dT%H:%M:%SZ"),
                    users_xml = users_xml,
                    request_id = uuid::Uuid::new_v4(),
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_delete_group(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.group_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'GroupName'");
        }

        let mut state = state.write().await;
        match state.delete_group(&input.group_name) {
            Ok(()) => wire::serialize_delete_group_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_groups_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let path_prefix = input.path_prefix.as_deref();

        let state = state.read().await;
        let groups = state.list_groups(path_prefix);

        let mut members_xml = String::new();
        for g in &groups {
            members_xml.push_str(&format!(
                r#"      <member>
        <GroupName>{group_name}</GroupName>
        <GroupId>{group_id}</GroupId>
        <Arn>{arn}</Arn>
        <Path>{path}</Path>
        <CreateDate>{create_date}</CreateDate>
      </member>
"#,
                group_name = xml_escape(&g.name),
                group_id = xml_escape(&g.group_id),
                arn = xml_escape(&g.arn),
                path = xml_escape(&g.path),
                create_date = g.create_date.format("%Y-%m-%dT%H:%M:%SZ"),
            ));
        }

        let xml = format!(
            r#"<ListGroupsResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ListGroupsResult>
    <IsTruncated>false</IsTruncated>
    <Groups>
{members_xml}    </Groups>
  </ListGroupsResult>
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListGroupsResponse>"#,
            members_xml = members_xml,
            request_id = uuid::Uuid::new_v4(),
        );
        MockResponse::xml(200, xml)
    }

    async fn handle_update_group(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.group_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'GroupName'");
        }
        let new_group_name = input.new_group_name.as_deref();
        let new_path = input.new_path.as_deref();

        let mut state = state.write().await;
        match state.update_group(&input.group_name, new_group_name, new_path) {
            Ok(()) => wire::serialize_update_group_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_add_user_to_group(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_add_user_to_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.group_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'GroupName'");
        }
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }

        let mut state = state.write().await;
        match state.add_user_to_group(&input.group_name, &input.user_name) {
            Ok(()) => wire::serialize_add_user_to_group_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_remove_user_from_group(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_remove_user_from_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.group_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'GroupName'");
        }
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }

        let mut state = state.write().await;
        match state.remove_user_from_group(&input.group_name, &input.user_name) {
            Ok(()) => wire::serialize_remove_user_from_group_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_attach_group_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_attach_group_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.group_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'GroupName'");
        }
        if input.policy_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyArn'");
        }

        let mut state = state.write().await;
        match state.attach_group_policy(&input.group_name, &input.policy_arn) {
            Ok(()) => wire::serialize_attach_group_policy_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_detach_group_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_detach_group_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.group_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'GroupName'");
        }
        if input.policy_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyArn'");
        }

        let mut state = state.write().await;
        match state.detach_group_policy(&input.group_name, &input.policy_arn) {
            Ok(()) => wire::serialize_detach_group_policy_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_attached_group_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_attached_group_policies_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.group_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'GroupName'");
        }

        let state = state.read().await;
        match state.list_attached_group_policies(&input.group_name) {
            Ok(policies) => {
                let members: Vec<wire::AttachedPolicy> = policies
                    .iter()
                    .map(|p| wire::AttachedPolicy {
                        policy_arn: Some(p.policy_arn.clone()),
                        policy_name: Some(p.policy_name.clone()),
                    })
                    .collect();
                wire::serialize_list_attached_group_policies_response(
                    &wire::ListAttachedGroupPoliciesResponse {
                        attached_policies: Some(members.into()),
                        ..Default::default()
                    },
                )
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_put_group_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_group_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.group_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'GroupName'");
        }
        if input.policy_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyName'");
        }
        if input.policy_document.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyDocument'");
        }

        let mut state = state.write().await;
        match state.put_group_policy(
            &input.group_name,
            &input.policy_name,
            &input.policy_document,
        ) {
            Ok(()) => wire::serialize_put_group_policy_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_get_group_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_group_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.group_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'GroupName'");
        }
        if input.policy_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyName'");
        }

        let state = state.read().await;
        match state.get_group_policy(&input.group_name, &input.policy_name) {
            Ok(policy) => {
                wire::serialize_get_group_policy_response(&wire::GetGroupPolicyResponse {
                    group_name: Some(input.group_name.clone()),
                    policy_name: Some(policy.policy_name.clone()),
                    policy_document: Some(policy.policy_document.clone()),
                })
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_delete_group_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_group_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.group_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'GroupName'");
        }
        if input.policy_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyName'");
        }

        let mut state = state.write().await;
        match state.delete_group_policy(&input.group_name, &input.policy_name) {
            Ok(()) => wire::serialize_delete_group_policy_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_group_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_group_policies_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.group_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'GroupName'");
        }

        let state = state.read().await;
        match state.list_group_policies(&input.group_name) {
            Ok(names) => {
                let mut members_xml = String::new();
                for name in &names {
                    members_xml
                        .push_str(&format!("      <member>{}</member>\n", xml_escape(name),));
                }
                let xml = format!(
                    r#"<ListGroupPoliciesResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ListGroupPoliciesResult>
    <IsTruncated>false</IsTruncated>
    <PolicyNames>
{members_xml}    </PolicyNames>
  </ListGroupPoliciesResult>
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListGroupPoliciesResponse>"#,
                    members_xml = members_xml,
                    request_id = uuid::Uuid::new_v4(),
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => iam_error_response(&e),
        }
    }

    // ==================== Access key handlers ====================

    async fn handle_create_access_key(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_access_key_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.as_ref().is_none_or(|s| s.is_empty()) {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }

        let mut state = state.write().await;
        match state.create_access_key(input.user_name.as_deref().unwrap()) {
            Ok(key) => {
                let xml = format!(
                    r#"<CreateAccessKeyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <CreateAccessKeyResult>
    <AccessKey>
      <UserName>{user_name}</UserName>
      <AccessKeyId>{access_key_id}</AccessKeyId>
      <Status>{status}</Status>
      <SecretAccessKey>{secret_access_key}</SecretAccessKey>
      <CreateDate>{create_date}</CreateDate>
    </AccessKey>
  </CreateAccessKeyResult>
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateAccessKeyResponse>"#,
                    user_name = xml_escape(&key.user_name),
                    access_key_id = xml_escape(&key.access_key_id),
                    status = xml_escape(&key.status),
                    secret_access_key = xml_escape(&key.secret_access_key),
                    create_date = key.create_date.format("%Y-%m-%dT%H:%M:%SZ"),
                    request_id = uuid::Uuid::new_v4(),
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_access_keys(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_access_keys_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.as_ref().is_none_or(|s| s.is_empty()) {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }

        let state = state.read().await;
        match state.list_access_keys(input.user_name.as_deref().unwrap()) {
            Ok(keys) => {
                let mut members_xml = String::new();
                for key in &keys {
                    members_xml.push_str(&format!(
                        r#"      <member>
        <UserName>{user_name}</UserName>
        <AccessKeyId>{access_key_id}</AccessKeyId>
        <Status>{status}</Status>
        <CreateDate>{create_date}</CreateDate>
      </member>
"#,
                        user_name = xml_escape(&key.user_name),
                        access_key_id = xml_escape(&key.access_key_id),
                        status = xml_escape(&key.status),
                        create_date = key.create_date.format("%Y-%m-%dT%H:%M:%SZ"),
                    ));
                }

                let xml = format!(
                    r#"<ListAccessKeysResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ListAccessKeysResult>
    <IsTruncated>false</IsTruncated>
    <AccessKeyMetadata>
{members_xml}    </AccessKeyMetadata>
  </ListAccessKeysResult>
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListAccessKeysResponse>"#,
                    members_xml = members_xml,
                    request_id = uuid::Uuid::new_v4(),
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_delete_access_key(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_access_key_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.as_ref().is_none_or(|s| s.is_empty()) {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        if input.access_key_id.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'AccessKeyId'");
        }

        let mut state = state.write().await;
        match state.delete_access_key(input.user_name.as_deref().unwrap(), &input.access_key_id) {
            Ok(()) => {
                let xml = format!(
                    r#"<DeleteAccessKeyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteAccessKeyResponse>"#,
                    request_id = uuid::Uuid::new_v4(),
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_update_access_key(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_access_key_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.as_ref().is_none_or(|s| s.is_empty()) {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        if input.access_key_id.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'AccessKeyId'");
        }
        if input.status.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'Status'");
        }

        let mut state = state.write().await;
        match state.update_access_key(
            input.user_name.as_deref().unwrap(),
            &input.access_key_id,
            &input.status,
        ) {
            Ok(()) => wire::serialize_update_access_key_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_get_access_key_last_used(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_access_key_last_used_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.access_key_id.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'AccessKeyId'");
        }

        let state = state.read().await;
        match state.get_access_key_last_used(&input.access_key_id) {
            Ok((key, info)) => {
                let last_used = info.map(|i| wire::AccessKeyLastUsed {
                    last_used_date: i
                        .last_used_date
                        .map(|d| d.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                    service_name: Some(i.service_name.clone().unwrap_or_else(|| "N/A".to_string())),
                    region: Some(i.region.clone().unwrap_or_else(|| "N/A".to_string())),
                });
                wire::serialize_get_access_key_last_used_response(
                    &wire::GetAccessKeyLastUsedResponse {
                        user_name: Some(key.user_name.clone()),
                        access_key_last_used: last_used,
                    },
                )
            }
            Err(e) => iam_error_response(&e),
        }
    }

    // ==================== Policy handlers ====================

    async fn handle_create_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.policy_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyName'");
        }
        if input.policy_document.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyDocument'");
        }
        let path = input.path.as_deref().unwrap_or("/");
        let description = input.description.as_deref().unwrap_or("");
        let tags: Vec<Tag> = input
            .tags
            .map(|tl| {
                tl.items
                    .into_iter()
                    .map(|t| Tag {
                        key: t.key,
                        value: t.value,
                    })
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.create_policy(
            account_id,
            &input.policy_name,
            path,
            &input.policy_document,
            description,
            tags,
        ) {
            Ok(policy) => {
                let tags_xml = if policy.tags.is_empty() {
                    String::new()
                } else {
                    let members: Vec<String> = policy
                        .tags
                        .iter()
                        .map(|t| {
                            format!(
                                "<member><Key>{}</Key><Value>{}</Value></member>",
                                xml_escape(&t.key),
                                xml_escape(&t.value)
                            )
                        })
                        .collect();
                    format!("<Tags>{}</Tags>", members.join(""))
                };
                let desc_xml = if policy.description.is_empty() {
                    String::new()
                } else {
                    format!(
                        "<Description>{}</Description>",
                        xml_escape(&policy.description)
                    )
                };
                let xml = format!(
                    r#"<CreatePolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <CreatePolicyResult>
    <Policy>
      <PolicyName>{policy_name}</PolicyName>
      <PolicyId>{policy_id}</PolicyId>
      <Arn>{arn}</Arn>
      <Path>{path}</Path>
      <DefaultVersionId>{default_version_id}</DefaultVersionId>
      <AttachmentCount>{attachment_count}</AttachmentCount>
      <IsAttachable>{is_attachable}</IsAttachable>
      <CreateDate>{create_date}</CreateDate>
      <UpdateDate>{update_date}</UpdateDate>
      {desc_xml}
      {tags_xml}
    </Policy>
  </CreatePolicyResult>
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreatePolicyResponse>"#,
                    policy_name = xml_escape(&policy.policy_name),
                    policy_id = xml_escape(&policy.policy_id),
                    arn = xml_escape(&policy.arn),
                    path = xml_escape(&policy.path),
                    default_version_id = xml_escape(&policy.default_version_id),
                    attachment_count = policy.attachment_count,
                    is_attachable = policy.is_attachable,
                    create_date = policy.create_date.format("%Y-%m-%dT%H:%M:%SZ"),
                    update_date = policy.update_date.format("%Y-%m-%dT%H:%M:%SZ"),
                    request_id = uuid::Uuid::new_v4(),
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_get_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.policy_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyArn'");
        }

        let state = state.read().await;
        match state.get_policy(&input.policy_arn) {
            Ok(policy) => {
                let tags_xml = if policy.tags.is_empty() {
                    String::new()
                } else {
                    let members: Vec<String> = policy
                        .tags
                        .iter()
                        .map(|t| {
                            format!(
                                "<member><Key>{}</Key><Value>{}</Value></member>",
                                xml_escape(&t.key),
                                xml_escape(&t.value)
                            )
                        })
                        .collect();
                    format!("<Tags>{}</Tags>", members.join(""))
                };
                let desc_xml = if policy.description.is_empty() {
                    String::new()
                } else {
                    format!(
                        "<Description>{}</Description>",
                        xml_escape(&policy.description)
                    )
                };
                let xml = format!(
                    r#"<GetPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <GetPolicyResult>
    <Policy>
      <PolicyName>{policy_name}</PolicyName>
      <PolicyId>{policy_id}</PolicyId>
      <Arn>{arn}</Arn>
      <Path>{path}</Path>
      <DefaultVersionId>{default_version_id}</DefaultVersionId>
      <AttachmentCount>{attachment_count}</AttachmentCount>
      <IsAttachable>{is_attachable}</IsAttachable>
      <CreateDate>{create_date}</CreateDate>
      <UpdateDate>{update_date}</UpdateDate>
      {desc_xml}
      {tags_xml}
    </Policy>
  </GetPolicyResult>
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetPolicyResponse>"#,
                    policy_name = xml_escape(&policy.policy_name),
                    policy_id = xml_escape(&policy.policy_id),
                    arn = xml_escape(&policy.arn),
                    path = xml_escape(&policy.path),
                    default_version_id = xml_escape(&policy.default_version_id),
                    attachment_count = policy.attachment_count,
                    is_attachable = policy.is_attachable,
                    create_date = policy.create_date.format("%Y-%m-%dT%H:%M:%SZ"),
                    update_date = policy.update_date.format("%Y-%m-%dT%H:%M:%SZ"),
                    request_id = uuid::Uuid::new_v4(),
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_delete_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.policy_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyArn'");
        }

        let mut state = state.write().await;
        match state.delete_policy(&input.policy_arn) {
            Ok(()) => wire::serialize_delete_policy_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_policies_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let scope = input.scope.as_deref();
        let path_prefix = input.path_prefix.as_deref();

        let state = state.read().await;
        let policies = state.list_policies(scope, path_prefix);

        let mut members_xml = String::new();
        for p in &policies {
            members_xml.push_str(&format!(
                r#"      <member>
        <PolicyName>{policy_name}</PolicyName>
        <PolicyId>{policy_id}</PolicyId>
        <Arn>{arn}</Arn>
        <Path>{path}</Path>
        <DefaultVersionId>{default_version_id}</DefaultVersionId>
        <AttachmentCount>{attachment_count}</AttachmentCount>
        <IsAttachable>{is_attachable}</IsAttachable>
        <CreateDate>{create_date}</CreateDate>
        <UpdateDate>{update_date}</UpdateDate>
      </member>
"#,
                policy_name = xml_escape(&p.policy_name),
                policy_id = xml_escape(&p.policy_id),
                arn = xml_escape(&p.arn),
                path = xml_escape(&p.path),
                default_version_id = xml_escape(&p.default_version_id),
                attachment_count = p.attachment_count,
                is_attachable = p.is_attachable,
                create_date = p.create_date.format("%Y-%m-%dT%H:%M:%SZ"),
                update_date = p.update_date.format("%Y-%m-%dT%H:%M:%SZ"),
            ));
        }

        let xml = format!(
            r#"<ListPoliciesResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ListPoliciesResult>
    <IsTruncated>false</IsTruncated>
    <Policies>
{members_xml}    </Policies>
  </ListPoliciesResult>
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListPoliciesResponse>"#,
            members_xml = members_xml,
            request_id = uuid::Uuid::new_v4(),
        );
        MockResponse::xml(200, xml)
    }

    async fn handle_create_policy_version(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_policy_version_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.policy_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyArn'");
        }
        if input.policy_document.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyDocument'");
        }
        let set_as_default = params
            .get("SetAsDefault")
            .map(|s| s == "true")
            .unwrap_or(false);

        let mut state = state.write().await;
        match state.create_policy_version(&input.policy_arn, &input.policy_document, set_as_default)
        {
            Ok(version) => {
                wire::serialize_create_policy_version_response(&wire::CreatePolicyVersionResponse {
                    policy_version: Some(wire::PolicyVersion {
                        version_id: Some(version.version_id.clone()),
                        document: Some(version.document.clone()),
                        is_default_version: Some(version.is_default_version),
                        create_date: Some(
                            version.create_date.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                        ),
                    }),
                })
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_get_policy_version(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_policy_version_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.policy_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyArn'");
        }
        if input.version_id.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'VersionId'");
        }

        let state = state.read().await;
        match state.get_policy_version(&input.policy_arn, &input.version_id) {
            Ok(version) => {
                wire::serialize_get_policy_version_response(&wire::GetPolicyVersionResponse {
                    policy_version: Some(wire::PolicyVersion {
                        version_id: Some(version.version_id.clone()),
                        document: Some(version.document.clone()),
                        is_default_version: Some(version.is_default_version),
                        create_date: Some(
                            version.create_date.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                        ),
                    }),
                })
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_delete_policy_version(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_policy_version_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.policy_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyArn'");
        }
        if input.version_id.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'VersionId'");
        }

        let mut state = state.write().await;
        match state.delete_policy_version(&input.policy_arn, &input.version_id) {
            Ok(()) => wire::serialize_delete_policy_version_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_policy_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_policy_versions_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.policy_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyArn'");
        }

        let state = state.read().await;
        match state.list_policy_versions(&input.policy_arn) {
            Ok(versions) => {
                let wire_versions: Vec<wire::PolicyVersion> = versions
                    .iter()
                    .map(|v| wire::PolicyVersion {
                        version_id: Some(v.version_id.clone()),
                        document: Some(v.document.clone()),
                        is_default_version: Some(v.is_default_version),
                        create_date: Some(v.create_date.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                    })
                    .collect();
                wire::serialize_list_policy_versions_response(&wire::ListPolicyVersionsResponse {
                    versions: Some(wire_versions.into()),
                    ..Default::default()
                })
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_set_default_policy_version(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_set_default_policy_version_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.policy_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyArn'");
        }
        if input.version_id.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'VersionId'");
        }

        let mut state = state.write().await;
        match state.set_default_policy_version(&input.policy_arn, &input.version_id) {
            Ok(()) => wire::serialize_set_default_policy_version_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_tag_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.policy_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyArn'");
        }
        let tags: Vec<Tag> = input
            .tags
            .items
            .into_iter()
            .map(|t| Tag {
                key: t.key,
                value: t.value,
            })
            .collect();

        let mut state = state.write().await;
        match state.tag_policy(&input.policy_arn, tags) {
            Ok(()) => wire::serialize_tag_policy_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_untag_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.policy_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyArn'");
        }
        let tag_keys = input.tag_keys.items;

        let mut state = state.write().await;
        match state.untag_policy(&input.policy_arn, &tag_keys) {
            Ok(()) => wire::serialize_untag_policy_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_policy_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_policy_tags_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.policy_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyArn'");
        }

        let state = state.read().await;
        match state.list_policy_tags(&input.policy_arn) {
            Ok(tags) => {
                let wire_tags = tags_to_wire(tags);
                wire::serialize_list_policy_tags_response(&wire::ListPolicyTagsResponse {
                    tags: Some(wire_tags),
                    ..Default::default()
                })
            }
            Err(e) => iam_error_response(&e),
        }
    }

    // ==================== Instance profile handlers ====================

    async fn handle_create_instance_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_instance_profile_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.instance_profile_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'InstanceProfileName'");
        }
        let path = input.path.as_deref().unwrap_or("/");
        let tags: Vec<Tag> = input
            .tags
            .map(|tl| {
                tl.items
                    .into_iter()
                    .map(|t| Tag {
                        key: t.key,
                        value: t.value,
                    })
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.create_instance_profile(account_id, &input.instance_profile_name, path, tags) {
            Ok(ip) => {
                let ip_clone = ip.clone();
                wire::serialize_create_instance_profile_response(
                    &wire::CreateInstanceProfileResponse {
                        instance_profile: Some(make_wire_instance_profile(&ip_clone, &state)),
                    },
                )
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_get_instance_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_instance_profile_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.instance_profile_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'InstanceProfileName'");
        }

        let state = state.read().await;
        match state.get_instance_profile(&input.instance_profile_name) {
            Ok(ip) => {
                wire::serialize_get_instance_profile_response(&wire::GetInstanceProfileResponse {
                    instance_profile: Some(make_wire_instance_profile(ip, &state)),
                })
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_delete_instance_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_instance_profile_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.instance_profile_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'InstanceProfileName'");
        }

        let mut state = state.write().await;
        match state.delete_instance_profile(&input.instance_profile_name) {
            Ok(()) => wire::serialize_delete_instance_profile_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_add_role_to_instance_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_add_role_to_instance_profile_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.instance_profile_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'InstanceProfileName'");
        }
        if input.role_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RoleName'");
        }

        let mut state = state.write().await;
        match state.add_role_to_instance_profile(&input.instance_profile_name, &input.role_name) {
            Ok(()) => wire::serialize_add_role_to_instance_profile_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_remove_role_from_instance_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_remove_role_from_instance_profile_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.instance_profile_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'InstanceProfileName'");
        }
        if input.role_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RoleName'");
        }

        let mut state = state.write().await;
        match state
            .remove_role_from_instance_profile(&input.instance_profile_name, &input.role_name)
        {
            Ok(()) => wire::serialize_remove_role_from_instance_profile_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_tag_instance_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_instance_profile_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.instance_profile_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'InstanceProfileName'");
        }
        let tags: Vec<Tag> = input
            .tags
            .items
            .into_iter()
            .map(|t| Tag {
                key: t.key,
                value: t.value,
            })
            .collect();

        let mut state = state.write().await;
        match state.tag_instance_profile(&input.instance_profile_name, tags) {
            Ok(()) => wire::serialize_tag_instance_profile_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_untag_instance_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_instance_profile_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.instance_profile_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'InstanceProfileName'");
        }
        let tag_keys = input.tag_keys.items;

        let mut state = state.write().await;
        match state.untag_instance_profile(&input.instance_profile_name, &tag_keys) {
            Ok(()) => wire::serialize_untag_instance_profile_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_instance_profile_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_instance_profile_tags_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.instance_profile_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'InstanceProfileName'");
        }
        let state = state.read().await;
        match state.list_instance_profile_tags(&input.instance_profile_name) {
            Ok(tags) => wire::serialize_list_instance_profile_tags_response(
                &wire::ListInstanceProfileTagsResponse {
                    tags: Some(tags_to_wire(tags)),
                    is_truncated: Some(false),
                    ..Default::default()
                },
            ),
            Err(e) => iam_error_response(&e),
        }
    }

    // ==================== Login profile handlers ====================

    async fn handle_create_login_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_login_profile_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.as_ref().is_none_or(|s| s.is_empty()) {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        let password = input.password.as_deref().unwrap_or("");
        let password_reset_required = params
            .get("PasswordResetRequired")
            .map(|s| s == "true")
            .unwrap_or(false);

        let mut state = state.write().await;
        match state.create_login_profile(
            input.user_name.as_deref().unwrap(),
            password,
            password_reset_required,
        ) {
            Ok(lp) => {
                wire::serialize_create_login_profile_response(&wire::CreateLoginProfileResponse {
                    login_profile: Some(wire::LoginProfile {
                        user_name: Some(lp.user_name.clone()),
                        create_date: Some(lp.create_date.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                        password_reset_required: Some(lp.password_reset_required),
                    }),
                })
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_get_login_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_login_profile_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.as_ref().is_none_or(|s| s.is_empty()) {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }

        let state = state.read().await;
        match state.get_login_profile(input.user_name.as_deref().unwrap()) {
            Ok(lp) => wire::serialize_get_login_profile_response(&wire::GetLoginProfileResponse {
                login_profile: Some(wire::LoginProfile {
                    user_name: Some(lp.user_name.clone()),
                    create_date: Some(lp.create_date.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                    password_reset_required: Some(lp.password_reset_required),
                }),
            }),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_update_login_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_login_profile_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        let password = input.password.as_deref();
        let password_reset_required = input.password_reset_required;

        let mut state = state.write().await;
        match state.update_login_profile(&input.user_name, password, password_reset_required) {
            Ok(()) => wire::serialize_update_login_profile_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_delete_login_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_login_profile_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.as_ref().is_none_or(|s| s.is_empty()) {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }

        let mut state = state.write().await;
        match state.delete_login_profile(input.user_name.as_deref().unwrap()) {
            Ok(()) => wire::serialize_delete_login_profile_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    // ==================== OpenID Connect provider handlers ====================

    async fn handle_create_oidc_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_open_i_d_connect_provider_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.url.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'Url'");
        }
        let client_id_list = input.client_i_d_list.map(|l| l.items).unwrap_or_default();
        let thumbprint_list = input.thumbprint_list.map(|l| l.items).unwrap_or_default();
        let tags: Vec<Tag> = input
            .tags
            .map(|tl| {
                tl.items
                    .into_iter()
                    .map(|t| Tag {
                        key: t.key,
                        value: t.value,
                    })
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.create_oidc_provider(
            account_id,
            &input.url,
            client_id_list,
            thumbprint_list,
            tags,
        ) {
            Ok(provider) => wire::serialize_create_open_i_d_connect_provider_response(
                &wire::CreateOpenIDConnectProviderResponse {
                    open_i_d_connect_provider_arn: Some(provider.arn.clone()),
                    tags: if provider.tags.is_empty() {
                        None
                    } else {
                        Some(tags_to_wire(&provider.tags))
                    },
                },
            ),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_get_oidc_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_open_i_d_connect_provider_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.open_i_d_connect_provider_arn.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'OpenIDConnectProviderArn'",
            );
        }

        let state = state.read().await;
        match state.get_oidc_provider(&input.open_i_d_connect_provider_arn) {
            Ok(provider) => wire::serialize_get_open_i_d_connect_provider_response(
                &wire::GetOpenIDConnectProviderResponse {
                    url: Some(provider.url.clone()),
                    client_i_d_list: Some(provider.client_id_list.clone().into()),
                    thumbprint_list: Some(provider.thumbprint_list.clone().into()),
                    create_date: Some(
                        provider
                            .create_date
                            .format("%Y-%m-%dT%H:%M:%SZ")
                            .to_string(),
                    ),
                    tags: if provider.tags.is_empty() {
                        None
                    } else {
                        Some(tags_to_wire(&provider.tags))
                    },
                },
            ),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_delete_oidc_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_open_i_d_connect_provider_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.open_i_d_connect_provider_arn.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'OpenIDConnectProviderArn'",
            );
        }

        let mut state = state.write().await;
        match state.delete_oidc_provider(&input.open_i_d_connect_provider_arn) {
            Ok(()) => wire::serialize_delete_open_i_d_connect_provider_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_oidc_providers(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let providers = state.list_oidc_providers();

        let entries: Vec<wire::OpenIDConnectProviderListEntry> = providers
            .iter()
            .map(|p| wire::OpenIDConnectProviderListEntry {
                arn: Some(p.arn.clone()),
            })
            .collect();

        wire::serialize_list_open_i_d_connect_providers_response(
            &wire::ListOpenIDConnectProvidersResponse {
                open_i_d_connect_provider_list: Some(entries.into()),
            },
        )
    }

    async fn handle_update_oidc_provider_thumbprint(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_open_i_d_connect_provider_thumbprint_request(params) {
                Ok(v) => v,
                Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
            };
        if input.open_i_d_connect_provider_arn.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'OpenIDConnectProviderArn'",
            );
        }
        let thumbprint_list = input.thumbprint_list.items;

        let mut state = state.write().await;
        match state
            .update_oidc_provider_thumbprint(&input.open_i_d_connect_provider_arn, thumbprint_list)
        {
            Ok(()) => wire::serialize_update_open_i_d_connect_provider_thumbprint_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_tag_oidc_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_open_i_d_connect_provider_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.open_i_d_connect_provider_arn.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'OpenIDConnectProviderArn'",
            );
        }
        let tags: Vec<Tag> = input
            .tags
            .items
            .into_iter()
            .map(|t| Tag {
                key: t.key,
                value: t.value,
            })
            .collect();

        let mut state = state.write().await;
        match state.tag_oidc_provider(&input.open_i_d_connect_provider_arn, tags) {
            Ok(()) => wire::serialize_tag_open_i_d_connect_provider_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_untag_oidc_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_open_i_d_connect_provider_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.open_i_d_connect_provider_arn.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'OpenIDConnectProviderArn'",
            );
        }
        let tag_keys = input.tag_keys.items;

        let mut state = state.write().await;
        match state.untag_oidc_provider(&input.open_i_d_connect_provider_arn, &tag_keys) {
            Ok(()) => wire::serialize_untag_open_i_d_connect_provider_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_oidc_provider_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_open_i_d_connect_provider_tags_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.open_i_d_connect_provider_arn.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'OpenIDConnectProviderArn'",
            );
        }

        let state = state.read().await;
        match state.list_oidc_provider_tags(&input.open_i_d_connect_provider_arn) {
            Ok(tags) => {
                let wire_tags = tags_to_wire(tags);
                wire::serialize_list_open_i_d_connect_provider_tags_response(
                    &wire::ListOpenIDConnectProviderTagsResponse {
                        tags: Some(wire_tags),
                        ..Default::default()
                    },
                )
            }
            Err(e) => iam_error_response(&e),
        }
    }

    // ==================== SAML provider handlers ====================

    async fn handle_create_saml_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_s_a_m_l_provider_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'Name'");
        }
        if input.s_a_m_l_metadata_document.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SAMLMetadataDocument'");
        }
        let tags: Vec<Tag> = input
            .tags
            .map(|tl| {
                tl.items
                    .into_iter()
                    .map(|t| Tag {
                        key: t.key,
                        value: t.value,
                    })
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.create_saml_provider(
            account_id,
            &input.name,
            &input.s_a_m_l_metadata_document,
            tags,
        ) {
            Ok(provider) => wire::serialize_create_s_a_m_l_provider_response(
                &wire::CreateSAMLProviderResponse {
                    s_a_m_l_provider_arn: Some(provider.arn.clone()),
                    tags: if provider.tags.is_empty() {
                        None
                    } else {
                        Some(tags_to_wire(&provider.tags))
                    },
                },
            ),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_get_saml_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_s_a_m_l_provider_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.s_a_m_l_provider_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SAMLProviderArn'");
        }

        let state = state.read().await;
        match state.get_saml_provider(&input.s_a_m_l_provider_arn) {
            Ok(provider) => {
                wire::serialize_get_s_a_m_l_provider_response(&wire::GetSAMLProviderResponse {
                    s_a_m_l_metadata_document: Some(provider.saml_metadata_document.clone()),
                    create_date: Some(
                        provider
                            .create_date
                            .format("%Y-%m-%dT%H:%M:%SZ")
                            .to_string(),
                    ),
                    valid_until: provider
                        .valid_until
                        .map(|d| d.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                    tags: if provider.tags.is_empty() {
                        None
                    } else {
                        Some(tags_to_wire(&provider.tags))
                    },
                    ..Default::default()
                })
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_delete_saml_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_s_a_m_l_provider_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.s_a_m_l_provider_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SAMLProviderArn'");
        }

        let mut state = state.write().await;
        match state.delete_saml_provider(&input.s_a_m_l_provider_arn) {
            Ok(()) => wire::serialize_delete_s_a_m_l_provider_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_update_saml_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_s_a_m_l_provider_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.s_a_m_l_provider_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SAMLProviderArn'");
        }
        if input
            .s_a_m_l_metadata_document
            .as_ref()
            .is_none_or(|s| s.is_empty())
        {
            return MockResponse::error(400, "MissingParameter", "Missing 'SAMLMetadataDocument'");
        }

        let mut state = state.write().await;
        match state.update_saml_provider(
            &input.s_a_m_l_provider_arn,
            input.s_a_m_l_metadata_document.as_deref().unwrap(),
        ) {
            Ok(provider) => wire::serialize_update_s_a_m_l_provider_response(
                &wire::UpdateSAMLProviderResponse {
                    s_a_m_l_provider_arn: Some(provider.arn.clone()),
                },
            ),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_saml_providers(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let providers = state.list_saml_providers();

        let entries: Vec<wire::SAMLProviderListEntry> = providers
            .iter()
            .map(|p| wire::SAMLProviderListEntry {
                arn: Some(p.arn.clone()),
                create_date: Some(p.create_date.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                valid_until: p
                    .valid_until
                    .map(|d| d.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
            })
            .collect();

        wire::serialize_list_s_a_m_l_providers_response(&wire::ListSAMLProvidersResponse {
            s_a_m_l_provider_list: Some(entries.into()),
        })
    }

    // ==================== MFA handlers ====================

    async fn handle_create_virtual_mfa_device(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_virtual_m_f_a_device_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.virtual_m_f_a_device_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'VirtualMFADeviceName'");
        }
        let path = input.path.as_deref().unwrap_or("/");
        let tags: Vec<Tag> = input
            .tags
            .map(|tl| {
                tl.items
                    .into_iter()
                    .map(|t| Tag {
                        key: t.key,
                        value: t.value,
                    })
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.create_virtual_mfa_device(
            account_id,
            &input.virtual_m_f_a_device_name,
            path,
            tags,
        ) {
            Ok(device) => wire::serialize_create_virtual_m_f_a_device_response(
                &wire::CreateVirtualMFADeviceResponse {
                    virtual_m_f_a_device: Some(wire::VirtualMFADevice {
                        serial_number: Some(device.serial_number.clone()),
                        base32_string_seed: Some(device.base32_string_seed.clone()),
                        q_r_code_p_n_g: Some(device.qr_code_png.clone()),
                        ..Default::default()
                    }),
                },
            ),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_delete_virtual_mfa_device(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_virtual_m_f_a_device_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.serial_number.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SerialNumber'");
        }

        let mut state = state.write().await;
        match state.delete_virtual_mfa_device(&input.serial_number) {
            Ok(()) => wire::serialize_delete_virtual_m_f_a_device_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_virtual_mfa_devices(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let devices = state.list_virtual_mfa_devices();

        let wire_devices: Vec<wire::VirtualMFADevice> = devices
            .iter()
            .map(|d| wire::VirtualMFADevice {
                serial_number: Some(d.serial_number.clone()),
                enable_date: d
                    .enable_date
                    .map(|dt| dt.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_virtual_m_f_a_devices_response(&wire::ListVirtualMFADevicesResponse {
            virtual_m_f_a_devices: Some(wire_devices.into()),
            ..Default::default()
        })
    }

    async fn handle_enable_mfa_device(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_enable_m_f_a_device_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        if input.serial_number.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SerialNumber'");
        }
        let auth_code1 = params
            .get("AuthenticationCode1")
            .map(|s| s.as_str())
            .unwrap_or("000000");
        let auth_code2 = params
            .get("AuthenticationCode2")
            .map(|s| s.as_str())
            .unwrap_or("000000");

        let mut state = state.write().await;
        match state.enable_mfa_device(
            &input.user_name,
            &input.serial_number,
            auth_code1,
            auth_code2,
        ) {
            Ok(()) => wire::serialize_enable_m_f_a_device_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_deactivate_mfa_device(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_deactivate_m_f_a_device_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.as_ref().is_none_or(|s| s.is_empty()) {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        if input.serial_number.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SerialNumber'");
        }

        let mut state = state.write().await;
        match state.deactivate_mfa_device(input.user_name.as_deref().unwrap(), &input.serial_number)
        {
            Ok(()) => wire::serialize_deactivate_m_f_a_device_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_mfa_devices(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_m_f_a_devices_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.as_ref().is_none_or(|s| s.is_empty()) {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }

        let state = state.read().await;
        match state.list_mfa_devices(input.user_name.as_deref().unwrap()) {
            Ok(devices) => {
                let wire_devices: Vec<wire::MFADevice> = devices
                    .iter()
                    .map(|d| wire::MFADevice {
                        user_name: Some(d.user_name.clone()),
                        serial_number: Some(d.serial_number.clone()),
                        enable_date: Some(d.enable_date.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                    })
                    .collect();
                wire::serialize_list_m_f_a_devices_response(&wire::ListMFADevicesResponse {
                    m_f_a_devices: Some(wire_devices.into()),
                    ..Default::default()
                })
            }
            Err(e) => iam_error_response(&e),
        }
    }

    // ==================== Password policy handlers ====================

    async fn handle_update_account_password_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_account_password_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let policy = crate::types::AccountPasswordPolicy {
            minimum_password_length: input.minimum_password_length.unwrap_or(6),
            require_symbols: input.require_symbols.unwrap_or(false),
            require_numbers: input.require_numbers.unwrap_or(false),
            require_uppercase_characters: input.require_uppercase_characters.unwrap_or(false),
            require_lowercase_characters: input.require_lowercase_characters.unwrap_or(false),
            allow_users_to_change_password: input.allow_users_to_change_password.unwrap_or(false),
            max_password_age: input.max_password_age,
            password_reuse_prevention: input.password_reuse_prevention,
            hard_expiry: input.hard_expiry,
            expire_passwords: input.max_password_age.map(|_| true).unwrap_or(false),
        };

        let mut state = state.write().await;
        state.update_account_password_policy(policy);
        wire::serialize_update_account_password_policy_response()
    }

    async fn handle_get_account_password_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_account_password_policy() {
            Ok(policy) => wire::serialize_get_account_password_policy_response(
                &wire::GetAccountPasswordPolicyResponse {
                    password_policy: Some(wire::PasswordPolicy {
                        minimum_password_length: Some(policy.minimum_password_length),
                        require_symbols: Some(policy.require_symbols),
                        require_numbers: Some(policy.require_numbers),
                        require_uppercase_characters: Some(policy.require_uppercase_characters),
                        require_lowercase_characters: Some(policy.require_lowercase_characters),
                        allow_users_to_change_password: Some(policy.allow_users_to_change_password),
                        max_password_age: policy.max_password_age,
                        password_reuse_prevention: policy.password_reuse_prevention,
                        hard_expiry: policy.hard_expiry,
                        expire_passwords: Some(policy.expire_passwords),
                    }),
                },
            ),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_delete_account_password_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_account_password_policy() {
            Ok(()) => wire::serialize_delete_account_password_policy_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    // ==================== Account alias handlers ====================

    async fn handle_create_account_alias(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_account_alias_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.account_alias.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'AccountAlias'");
        }

        let mut state = state.write().await;
        match state.create_account_alias(&input.account_alias) {
            Ok(()) => wire::serialize_create_account_alias_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_delete_account_alias(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_account_alias_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.account_alias.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'AccountAlias'");
        }

        let mut state = state.write().await;
        match state.delete_account_alias(&input.account_alias) {
            Ok(()) => wire::serialize_delete_account_alias_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_account_aliases(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let aliases = state.list_account_aliases();

        wire::serialize_list_account_aliases_response(&wire::ListAccountAliasesResponse {
            account_aliases: Some(aliases.to_vec().into()),
            ..Default::default()
        })
    }

    // ==================== Server certificate handlers ====================

    async fn handle_upload_server_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_upload_server_certificate_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.server_certificate_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ServerCertificateName'");
        }
        if input.certificate_body.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'CertificateBody'");
        }
        if input.private_key.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PrivateKey'");
        }
        let certificate_chain = input.certificate_chain.as_deref();
        let path = input.path.as_deref().unwrap_or("/");
        let tags: Vec<Tag> = input
            .tags
            .map(|tl| {
                tl.items
                    .into_iter()
                    .map(|t| Tag {
                        key: t.key,
                        value: t.value,
                    })
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.upload_server_certificate(
            account_id,
            &input.server_certificate_name,
            path,
            &input.certificate_body,
            &input.private_key,
            certificate_chain,
            tags,
        ) {
            Ok(cert) => wire::serialize_upload_server_certificate_response(
                &wire::UploadServerCertificateResponse {
                    server_certificate_metadata: Some(wire::ServerCertificateMetadata {
                        server_certificate_name: Some(cert.server_certificate_name.clone()),
                        server_certificate_id: Some(cert.server_certificate_id.clone()),
                        arn: Some(cert.arn.clone()),
                        path: Some(cert.path.clone()),
                        upload_date: Some(
                            cert.upload_date.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                        ),
                        expiration: cert
                            .expiration
                            .map(|d| d.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                    }),
                    tags: if cert.tags.is_empty() {
                        None
                    } else {
                        Some(tags_to_wire(&cert.tags))
                    },
                },
            ),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_get_server_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_server_certificate_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.server_certificate_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ServerCertificateName'");
        }

        let state = state.read().await;
        match state.get_server_certificate(&input.server_certificate_name) {
            Ok(cert) => wire::serialize_get_server_certificate_response(
                &wire::GetServerCertificateResponse {
                    server_certificate: Some(wire::ServerCertificate {
                        certificate_body: Some(cert.certificate_body.clone()),
                        certificate_chain: cert.certificate_chain.clone(),
                        server_certificate_metadata: Some(wire::ServerCertificateMetadata {
                            server_certificate_name: Some(cert.server_certificate_name.clone()),
                            server_certificate_id: Some(cert.server_certificate_id.clone()),
                            arn: Some(cert.arn.clone()),
                            path: Some(cert.path.clone()),
                            upload_date: Some(
                                cert.upload_date.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                            ),
                            expiration: cert
                                .expiration
                                .map(|d| d.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                        }),
                        tags: if cert.tags.is_empty() {
                            None
                        } else {
                            Some(tags_to_wire(&cert.tags))
                        },
                    }),
                },
            ),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_delete_server_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_server_certificate_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.server_certificate_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ServerCertificateName'");
        }

        let mut state = state.write().await;
        match state.delete_server_certificate(&input.server_certificate_name) {
            Ok(()) => wire::serialize_delete_server_certificate_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_server_certificates(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let certs = state.list_server_certificates();

        let wire_certs: Vec<wire::ServerCertificateMetadata> = certs
            .iter()
            .map(|c| wire::ServerCertificateMetadata {
                server_certificate_name: Some(c.server_certificate_name.clone()),
                server_certificate_id: Some(c.server_certificate_id.clone()),
                arn: Some(c.arn.clone()),
                path: Some(c.path.clone()),
                upload_date: Some(c.upload_date.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                expiration: c
                    .expiration
                    .map(|d| d.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
            })
            .collect();

        wire::serialize_list_server_certificates_response(&wire::ListServerCertificatesResponse {
            server_certificate_metadata_list: Some(wire_certs.into()),
            ..Default::default()
        })
    }

    // ==================== SSH public key handlers ====================

    async fn handle_upload_ssh_public_key(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_upload_s_s_h_public_key_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        if input.s_s_h_public_key_body.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SSHPublicKeyBody'");
        }

        let mut state = state.write().await;
        match state.upload_ssh_public_key(&input.user_name, &input.s_s_h_public_key_body) {
            Ok(key) => wire::serialize_upload_s_s_h_public_key_response(
                &wire::UploadSSHPublicKeyResponse {
                    s_s_h_public_key: Some(wire::SSHPublicKey {
                        user_name: Some(key.user_name.clone()),
                        s_s_h_public_key_id: Some(key.ssh_public_key_id.clone()),
                        fingerprint: Some(key.fingerprint.clone()),
                        s_s_h_public_key_body: Some(key.ssh_public_key_body.clone()),
                        status: Some(key.status.clone()),
                        upload_date: Some(key.upload_date.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                    }),
                },
            ),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_get_ssh_public_key(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_s_s_h_public_key_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        if input.s_s_h_public_key_id.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SSHPublicKeyId'");
        }
        let _encoding = if input.encoding.is_empty() {
            "SSH"
        } else {
            &input.encoding
        };

        let state = state.read().await;
        match state.get_ssh_public_key(&input.user_name, &input.s_s_h_public_key_id) {
            Ok(key) => {
                wire::serialize_get_s_s_h_public_key_response(&wire::GetSSHPublicKeyResponse {
                    s_s_h_public_key: Some(wire::SSHPublicKey {
                        user_name: Some(key.user_name.clone()),
                        s_s_h_public_key_id: Some(key.ssh_public_key_id.clone()),
                        fingerprint: Some(key.fingerprint.clone()),
                        s_s_h_public_key_body: Some(key.ssh_public_key_body.clone()),
                        status: Some(key.status.clone()),
                        upload_date: Some(key.upload_date.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                    }),
                })
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_update_ssh_public_key(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_s_s_h_public_key_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        if input.s_s_h_public_key_id.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SSHPublicKeyId'");
        }
        if input.status.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'Status'");
        }

        let mut state = state.write().await;
        match state.update_ssh_public_key(
            &input.user_name,
            &input.s_s_h_public_key_id,
            &input.status,
        ) {
            Ok(()) => wire::serialize_update_s_s_h_public_key_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_delete_ssh_public_key(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_s_s_h_public_key_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        if input.s_s_h_public_key_id.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SSHPublicKeyId'");
        }

        let mut state = state.write().await;
        match state.delete_ssh_public_key(&input.user_name, &input.s_s_h_public_key_id) {
            Ok(()) => wire::serialize_delete_s_s_h_public_key_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    // ==================== Signing certificate handlers ====================

    async fn handle_upload_signing_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_upload_signing_certificate_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.as_ref().is_none_or(|s| s.is_empty()) {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        if input.certificate_body.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'CertificateBody'");
        }

        let mut state = state.write().await;
        match state.upload_signing_certificate(
            input.user_name.as_deref().unwrap(),
            &input.certificate_body,
        ) {
            Ok(cert) => wire::serialize_upload_signing_certificate_response(
                &wire::UploadSigningCertificateResponse {
                    certificate: Some(wire::SigningCertificate {
                        user_name: Some(cert.user_name.clone()),
                        certificate_id: Some(cert.certificate_id.clone()),
                        certificate_body: Some(cert.certificate_body.clone()),
                        status: Some(cert.status.clone()),
                        upload_date: Some(
                            cert.upload_date.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                        ),
                    }),
                },
            ),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_signing_certificates(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_signing_certificates_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.as_ref().is_none_or(|s| s.is_empty()) {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }

        let state = state.read().await;
        match state.list_signing_certificates(input.user_name.as_deref().unwrap()) {
            Ok(certs) => {
                let wire_certs: Vec<wire::SigningCertificate> = certs
                    .iter()
                    .map(|c| wire::SigningCertificate {
                        user_name: Some(c.user_name.clone()),
                        certificate_id: Some(c.certificate_id.clone()),
                        certificate_body: Some(c.certificate_body.clone()),
                        status: Some(c.status.clone()),
                        upload_date: Some(c.upload_date.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                    })
                    .collect();
                wire::serialize_list_signing_certificates_response(
                    &wire::ListSigningCertificatesResponse {
                        certificates: Some(wire_certs.into()),
                        ..Default::default()
                    },
                )
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_update_signing_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_signing_certificate_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.as_ref().is_none_or(|s| s.is_empty()) {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        if input.certificate_id.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'CertificateId'");
        }
        if input.status.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'Status'");
        }

        let mut state = state.write().await;
        match state.update_signing_certificate(
            input.user_name.as_deref().unwrap(),
            &input.certificate_id,
            &input.status,
        ) {
            Ok(()) => wire::serialize_update_signing_certificate_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_delete_signing_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_signing_certificate_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.as_ref().is_none_or(|s| s.is_empty()) {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        if input.certificate_id.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'CertificateId'");
        }

        let mut state = state.write().await;
        match state
            .delete_signing_certificate(input.user_name.as_deref().unwrap(), &input.certificate_id)
        {
            Ok(()) => wire::serialize_delete_signing_certificate_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    // ==================== Account summary / reports ====================

    async fn handle_get_account_summary(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let summary = state.get_account_summary();

        // Build XML manually since the summary map has a special format
        let mut entries_xml = String::new();
        for (key, value) in &summary {
            entries_xml.push_str(&format!(
                r#"      <entry>
        <key>{key}</key>
        <value>{value}</value>
      </entry>
"#,
                key = xml_escape(key),
                value = value,
            ));
        }

        let xml = format!(
            r#"<GetAccountSummaryResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <GetAccountSummaryResult>
    <SummaryMap>
{entries_xml}    </SummaryMap>
  </GetAccountSummaryResult>
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetAccountSummaryResponse>"#,
            entries_xml = entries_xml,
            request_id = uuid::Uuid::new_v4(),
        );
        MockResponse::xml(200, xml)
    }

    async fn handle_get_credential_report(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_credential_report() {
            Ok(content) => {
                use std::io::Write;
                let encoded = {
                    let mut buf = Vec::new();
                    write!(&mut buf, "{}", base64_encode(&content)).unwrap();
                    String::from_utf8(buf).unwrap()
                };
                wire::serialize_get_credential_report_response(&wire::GetCredentialReportResponse {
                    content: Some(encoded),
                    generated_time: Some(
                        chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                    ),
                    report_format: Some("text/csv".to_string()),
                })
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_get_account_authorization_details(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let (users, roles, groups, policies) = state.get_account_authorization_details();

        let user_details: Vec<wire::UserDetail> = users
            .values()
            .map(|u| wire::UserDetail {
                user_name: Some(u.name.clone()),
                user_id: Some(u.user_id.clone()),
                arn: Some(u.arn.clone()),
                path: Some(u.path.clone()),
                create_date: Some(u.create_date.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                attached_managed_policies: Some(
                    u.attached_policies
                        .iter()
                        .map(|p| wire::AttachedPolicy {
                            policy_arn: Some(p.policy_arn.clone()),
                            policy_name: Some(p.policy_name.clone()),
                        })
                        .collect(),
                ),
                user_policy_list: Some(
                    u.inline_policies
                        .iter()
                        .map(|p| wire::PolicyDetail {
                            policy_name: Some(p.policy_name.clone()),
                            policy_document: Some(p.policy_document.clone()),
                        })
                        .collect(),
                ),
                ..Default::default()
            })
            .collect();

        let role_details: Vec<wire::RoleDetail> = roles
            .values()
            .map(|r| wire::RoleDetail {
                role_name: Some(r.name.clone()),
                role_id: Some(r.role_id.clone()),
                arn: Some(r.arn.clone()),
                path: Some(r.path.clone()),
                create_date: Some(r.create_date.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                assume_role_policy_document: Some(r.assume_role_policy_document.clone()),
                attached_managed_policies: Some(
                    r.attached_policies
                        .iter()
                        .map(|p| wire::AttachedPolicy {
                            policy_arn: Some(p.policy_arn.clone()),
                            policy_name: Some(p.policy_name.clone()),
                        })
                        .collect(),
                ),
                role_policy_list: Some(
                    r.inline_policies
                        .iter()
                        .map(|p| wire::PolicyDetail {
                            policy_name: Some(p.policy_name.clone()),
                            policy_document: Some(p.policy_document.clone()),
                        })
                        .collect(),
                ),
                ..Default::default()
            })
            .collect();

        let group_details: Vec<wire::GroupDetail> = groups
            .values()
            .map(|g| wire::GroupDetail {
                group_name: Some(g.name.clone()),
                group_id: Some(g.group_id.clone()),
                arn: Some(g.arn.clone()),
                path: Some(g.path.clone()),
                create_date: Some(g.create_date.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                attached_managed_policies: Some(
                    g.attached_policies
                        .iter()
                        .map(|p| wire::AttachedPolicy {
                            policy_arn: Some(p.policy_arn.clone()),
                            policy_name: Some(p.policy_name.clone()),
                        })
                        .collect(),
                ),
                group_policy_list: Some(
                    g.inline_policies
                        .iter()
                        .map(|p| wire::PolicyDetail {
                            policy_name: Some(p.policy_name.clone()),
                            policy_document: Some(p.policy_document.clone()),
                        })
                        .collect(),
                ),
            })
            .collect();

        let policy_details: Vec<wire::ManagedPolicyDetail> = policies
            .values()
            .map(|p| wire::ManagedPolicyDetail {
                policy_name: Some(p.policy_name.clone()),
                policy_id: Some(p.policy_id.clone()),
                arn: Some(p.arn.clone()),
                path: Some(p.path.clone()),
                default_version_id: Some(p.default_version_id.clone()),
                attachment_count: Some(p.attachment_count as i32),
                is_attachable: Some(p.is_attachable),
                create_date: Some(p.create_date.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                update_date: Some(p.update_date.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                policy_version_list: Some(
                    p.versions
                        .iter()
                        .map(|v| wire::PolicyVersion {
                            version_id: Some(v.version_id.clone()),
                            document: Some(v.document.clone()),
                            is_default_version: Some(v.is_default_version),
                            create_date: Some(
                                v.create_date.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                            ),
                        })
                        .collect(),
                ),
                ..Default::default()
            })
            .collect();

        wire::serialize_get_account_authorization_details_response(
            &wire::GetAccountAuthorizationDetailsResponse {
                user_detail_list: Some(user_details.into()),
                role_detail_list: Some(role_details.into()),
                group_detail_list: Some(group_details.into()),
                policies: Some(policy_details.into()),
                is_truncated: Some(false),
                ..Default::default()
            },
        )
    }

    // ==================== Stub / delegation handlers ====================

    // STUB[delegation-api]: IAM Delegated Admin flows require real AWS Organizations
    //   cross-account trust relationships; no equivalent mock state exists.
    async fn handle_accept_delegation_request(&self) -> MockResponse {
        wire::serialize_accept_delegation_request_response()
    }

    // STUB[delegation-api]: see handle_accept_delegation_request.
    async fn handle_associate_delegation_request(&self) -> MockResponse {
        wire::serialize_associate_delegation_request_response()
    }

    // STUB[delegation-api]: see handle_accept_delegation_request.
    async fn handle_create_delegation_request(&self) -> MockResponse {
        wire::serialize_create_delegation_request_response(&wire::CreateDelegationRequestResponse {
            ..Default::default()
        })
    }

    // STUB[delegation-api]: see handle_accept_delegation_request.
    async fn handle_get_delegation_request(&self) -> MockResponse {
        wire::serialize_get_delegation_request_response(&wire::GetDelegationRequestResponse {
            ..Default::default()
        })
    }

    // STUB[delegation-api]: see handle_accept_delegation_request.
    async fn handle_list_delegation_requests(&self) -> MockResponse {
        wire::serialize_list_delegation_requests_response(&wire::ListDelegationRequestsResponse {
            ..Default::default()
        })
    }

    // STUB[delegation-api]: see handle_accept_delegation_request.
    async fn handle_reject_delegation_request(&self) -> MockResponse {
        wire::serialize_reject_delegation_request_response()
    }

    // STUB[delegation-api]: see handle_accept_delegation_request.
    async fn handle_send_delegation_token(&self) -> MockResponse {
        wire::serialize_send_delegation_token_response()
    }

    // STUB[delegation-api]: see handle_accept_delegation_request.
    async fn handle_update_delegation_request(&self) -> MockResponse {
        wire::serialize_update_delegation_request_response()
    }

    // STUB[no-auth]: Mock has no real authentication layer; password change is a no-op.
    async fn handle_change_password(&self) -> MockResponse {
        wire::serialize_change_password_response()
    }

    async fn handle_generate_credential_report(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
    ) -> MockResponse {
        let mut state = state.write().await;
        let newly_generated = state.generate_credential_report();
        let (description, report_state) = if newly_generated {
            (
                "No report exists. Starting a new report generation task".to_string(),
                "STARTED".to_string(),
            )
        } else {
            (
                "A credential report has been generated".to_string(),
                "COMPLETE".to_string(),
            )
        };
        wire::serialize_generate_credential_report_response(
            &wire::GenerateCredentialReportResponse {
                description: Some(description),
                state: Some(report_state),
            },
        )
    }

    // STUB[org-integration]: Organizations access reports require real Organizations
    //   service integration; returns a synthetic job ID so callers can poll.
    async fn handle_generate_organizations_access_report(&self) -> MockResponse {
        let job_id = uuid::Uuid::new_v4().to_string();
        wire::serialize_generate_organizations_access_report_response(
            &wire::GenerateOrganizationsAccessReportResponse {
                job_id: Some(job_id),
            },
        )
    }

    // STUB[org-integration]: see handle_generate_organizations_access_report.
    //   Always reports COMPLETED with empty results.
    async fn handle_get_organizations_access_report(&self) -> MockResponse {
        wire::serialize_get_organizations_access_report_response(
            &wire::GetOrganizationsAccessReportResponse {
                job_status: Some("COMPLETED".to_string()),
                job_creation_date: Some(
                    chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                ),
                ..Default::default()
            },
        )
    }

    // STUB[no-telemetry]: Service last-accessed details are derived from CloudTrail
    //   access logs; mock has no access history to report. Stores the job in state so
    //   GetServiceLastAccessedDetails can validate the job ID.
    async fn handle_generate_service_last_accessed_details(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_generate_service_last_accessed_details_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let mut state = state.write().await;
        let job_id = state.create_service_last_accessed_job(&input.arn);
        wire::serialize_generate_service_last_accessed_details_response(
            &wire::GenerateServiceLastAccessedDetailsResponse {
                job_id: Some(job_id),
            },
        )
    }

    // STUB[no-telemetry]: see handle_generate_service_last_accessed_details.
    //   Always reports COMPLETED with empty results. Validates that the job ID was created.
    async fn handle_get_service_last_accessed_details(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_service_last_accessed_details_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let state = state.read().await;
        if state.get_service_last_accessed_job(&input.job_id).is_none() {
            return MockResponse::error(
                400,
                "InvalidInput",
                &format!("Job with id {} does not exist.", input.job_id),
            );
        }
        let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        wire::serialize_get_service_last_accessed_details_response(
            &wire::GetServiceLastAccessedDetailsResponse {
                job_status: Some("COMPLETED".to_string()),
                job_creation_date: Some(now.clone()),
                job_completion_date: Some(now),
                ..Default::default()
            },
        )
    }

    // STUB[no-telemetry]: see handle_generate_service_last_accessed_details.
    //   Always reports COMPLETED with empty results. Validates that the job ID was created.
    async fn handle_get_service_last_accessed_details_with_entities(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_service_last_accessed_details_with_entities_request(params)
            {
                Ok(v) => v,
                Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
            };
        let state = state.read().await;
        if state.get_service_last_accessed_job(&input.job_id).is_none() {
            return MockResponse::error(
                400,
                "InvalidInput",
                &format!("Job with id {} does not exist.", input.job_id),
            );
        }
        let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        wire::serialize_get_service_last_accessed_details_with_entities_response(
            &wire::GetServiceLastAccessedDetailsWithEntitiesResponse {
                job_status: Some("COMPLETED".to_string()),
                job_creation_date: Some(now.clone()),
                job_completion_date: Some(now),
                ..Default::default()
            },
        )
    }

    // ==================== Service-specific credential handlers ====================

    async fn handle_create_service_specific_credential(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_service_specific_credential_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        if input.service_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ServiceName'");
        }

        let mut state = state.write().await;
        match state.create_service_specific_credential(&input.user_name, &input.service_name) {
            Ok(cred) => wire::serialize_create_service_specific_credential_response(
                &wire::CreateServiceSpecificCredentialResponse {
                    service_specific_credential: Some(wire::ServiceSpecificCredential {
                        service_specific_credential_id: Some(
                            cred.service_specific_credential_id.clone(),
                        ),
                        user_name: Some(cred.user_name.clone()),
                        service_name: Some(cred.service_name.clone()),
                        service_user_name: Some(cred.service_user_name.clone()),
                        service_password: Some(cred.service_password.clone()),
                        status: Some(cred.status.clone()),
                        create_date: Some(
                            cred.create_date.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                        ),
                        ..Default::default()
                    }),
                },
            ),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_delete_service_specific_credential(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_service_specific_credential_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.service_specific_credential_id.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'ServiceSpecificCredentialId'",
            );
        }
        let user_name = input.user_name.as_deref();

        let mut state = state.write().await;
        match state
            .delete_service_specific_credential(&input.service_specific_credential_id, user_name)
        {
            Ok(()) => wire::serialize_delete_service_specific_credential_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_service_specific_credentials(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_service_specific_credentials_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let user_name = input.user_name.as_deref();
        let service_name = input.service_name.as_deref();

        let state = state.read().await;
        let creds = state.list_service_specific_credentials(user_name, service_name);

        let items: Vec<wire::ServiceSpecificCredentialMetadata> = creds
            .iter()
            .map(|c| wire::ServiceSpecificCredentialMetadata {
                service_specific_credential_id: Some(c.service_specific_credential_id.clone()),
                user_name: Some(c.user_name.clone()),
                service_name: Some(c.service_name.clone()),
                service_user_name: Some(c.service_user_name.clone()),
                status: Some(c.status.clone()),
                create_date: Some(c.create_date.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_service_specific_credentials_response(
            &wire::ListServiceSpecificCredentialsResponse {
                service_specific_credentials: if items.is_empty() {
                    None
                } else {
                    Some(items.into())
                },
                ..Default::default()
            },
        )
    }

    async fn handle_reset_service_specific_credential(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_reset_service_specific_credential_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.service_specific_credential_id.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'ServiceSpecificCredentialId'",
            );
        }
        let user_name = input.user_name.as_deref();

        let mut state = state.write().await;
        match state
            .reset_service_specific_credential(&input.service_specific_credential_id, user_name)
        {
            Ok(cred) => wire::serialize_reset_service_specific_credential_response(
                &wire::ResetServiceSpecificCredentialResponse {
                    service_specific_credential: Some(wire::ServiceSpecificCredential {
                        service_specific_credential_id: Some(
                            cred.service_specific_credential_id.clone(),
                        ),
                        user_name: Some(cred.user_name.clone()),
                        service_name: Some(cred.service_name.clone()),
                        service_user_name: Some(cred.service_user_name.clone()),
                        service_password: Some(cred.service_password.clone()),
                        status: Some(cred.status.clone()),
                        create_date: Some(
                            cred.create_date.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                        ),
                        ..Default::default()
                    }),
                },
            ),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_update_service_specific_credential(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_service_specific_credential_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.service_specific_credential_id.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'ServiceSpecificCredentialId'",
            );
        }
        if input.status.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'Status'");
        }
        let user_name = input.user_name.as_deref();

        let mut state = state.write().await;
        match state.update_service_specific_credential(
            &input.service_specific_credential_id,
            user_name,
            &input.status,
        ) {
            Ok(()) => wire::serialize_update_service_specific_credential_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    // STUB[no-engine]: Context key extraction requires parsing and evaluating IAM
    //   policy documents; returns an empty key list.
    async fn handle_get_context_keys_for_custom_policy(
        &self,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_context_keys_for_custom_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let keys = extract_context_keys_from_policies(&input.policy_input_list.items);
        let context_key_names = if keys.is_empty() {
            None
        } else {
            Some(wire::ContextKeyNamesResultListType { items: keys })
        };
        wire::serialize_get_context_keys_for_custom_policy_response(
            &wire::GetContextKeysForPolicyResponse { context_key_names },
        )
    }

    // STUB[no-engine]: see handle_get_context_keys_for_custom_policy.
    async fn handle_get_context_keys_for_principal_policy(
        &self,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_context_keys_for_principal_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let policy_docs = input.policy_input_list.map(|l| l.items).unwrap_or_default();
        let keys = extract_context_keys_from_policies(&policy_docs);
        let context_key_names = if keys.is_empty() {
            None
        } else {
            Some(wire::ContextKeyNamesResultListType { items: keys })
        };
        wire::serialize_get_context_keys_for_principal_policy_response(
            &wire::GetContextKeysForPolicyResponse { context_key_names },
        )
    }

    fn handle_simulate_custom_policy(&self, params: &HashMap<String, String>) -> MockResponse {
        let input = match wire::deserialize_simulate_custom_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let policy_input_list = input.policy_input_list.items;
        let action_names = input.action_names.items;
        let resource_arns = input.resource_arns.map(|l| l.items).unwrap_or_default();

        if action_names.is_empty() {
            return MockResponse::error(400, "InvalidInput", "ActionNames is required");
        }

        let policies = match parse_policy_input_list(&policy_input_list, "UserPolicy") {
            Ok(p) => p,
            Err(resp) => return resp,
        };

        let resource_list = if resource_arns.is_empty() {
            vec!["*".to_string()]
        } else {
            resource_arns
        };

        let results = run_simulation(&policies, &action_names, &resource_list);

        let mut resp =
            wire::serialize_simulate_custom_policy_response(&wire::SimulatePolicyResponse {
                evaluation_results: Some(results.into()),
                is_truncated: Some(false),
                ..Default::default()
            });
        // The auto-generated model uses serde(rename = "SimulatePrincipalPolicyResult")
        // which is wrong for the custom policy variant. Fix the XML tag name.
        let body_str = std::str::from_utf8(&resp.body).unwrap_or("").replace(
            "SimulatePrincipalPolicyResult",
            "SimulateCustomPolicyResult",
        );
        resp.body = bytes::Bytes::from(body_str);
        resp
    }

    async fn handle_simulate_principal_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_simulate_principal_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.policy_source_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicySourceArn'");
        }

        let action_names = input.action_names.items;
        if action_names.is_empty() {
            return MockResponse::error(400, "InvalidInput", "ActionNames is required");
        }

        let resource_arns = input.resource_arns.map(|l| l.items).unwrap_or_default();
        let policy_input_list = input.policy_input_list.map(|l| l.items).unwrap_or_default();

        let state = state.read().await;

        // Collect policies from the principal identified by PolicySourceArn.
        let mut policies: Vec<(
            winterbaume_iam_rule_eval::PolicyDocument,
            winterbaume_iam_rule_eval::PolicySource,
        )> = Vec::new();

        let found = collect_principal_policies(&state, &input.policy_source_arn, &mut policies);

        if !found {
            return MockResponse::error(
                404,
                "NoSuchEntity",
                &format!(
                    "Policy source ARN does not exist: {}",
                    input.policy_source_arn
                ),
            );
        }

        // Add optional PolicyInputList overrides.
        if !policy_input_list.is_empty() {
            match parse_policy_input_list(&policy_input_list, "UserPolicy") {
                Ok(extra) => policies.extend(extra),
                Err(resp) => return resp,
            }
        }

        let resource_list = if resource_arns.is_empty() {
            vec!["*".to_string()]
        } else {
            resource_arns
        };

        let results = run_simulation(&policies, &action_names, &resource_list);

        wire::serialize_simulate_principal_policy_response(&wire::SimulatePolicyResponse {
            evaluation_results: Some(results.into()),
            is_truncated: Some(false),
            ..Default::default()
        })
    }

    // ==================== MFA device handlers ====================

    async fn handle_get_m_f_a_device(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_m_f_a_device_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.serial_number.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SerialNumber'");
        }

        let state = state.read().await;
        match state.get_mfa_device(&input.serial_number) {
            Ok(device) => {
                // Build XML manually to avoid quick_xml HashMap serialization issues
                // (GetMFADeviceResponse has an Option<HashMap> certifications field)
                let serial_xml = xml_escape(&device.serial_number);
                let user_name_xml = device
                    .user_name
                    .as_deref()
                    .map(|u| format!("      <UserName>{}</UserName>\n", xml_escape(u)))
                    .unwrap_or_default();
                let enable_date_xml = device
                    .enable_date
                    .map(|d| {
                        format!(
                            "      <EnableDate>{}</EnableDate>\n",
                            d.format("%Y-%m-%dT%H:%M:%SZ")
                        )
                    })
                    .unwrap_or_default();
                let xml = format!(
                    r#"<GetMFADeviceResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <GetMFADeviceResult>
{user_name_xml}      <SerialNumber>{serial_xml}</SerialNumber>
{enable_date_xml}  </GetMFADeviceResult>
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetMFADeviceResponse>"#,
                    request_id = uuid::Uuid::new_v4(),
                );
                MockResponse::xml(200, xml)
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_tag_m_f_a_device(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_m_f_a_device_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.serial_number.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SerialNumber'");
        }
        let tags: Vec<Tag> = input
            .tags
            .items
            .into_iter()
            .map(|t| Tag {
                key: t.key,
                value: t.value,
            })
            .collect();

        let mut state = state.write().await;
        match state.tag_mfa_device(&input.serial_number, tags) {
            Ok(()) => wire::serialize_tag_m_f_a_device_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_untag_m_f_a_device(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_m_f_a_device_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.serial_number.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SerialNumber'");
        }
        let tag_keys = input.tag_keys.items;

        let mut state = state.write().await;
        match state.untag_mfa_device(&input.serial_number, &tag_keys) {
            Ok(()) => wire::serialize_untag_m_f_a_device_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_m_f_a_device_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_m_f_a_device_tags_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.serial_number.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SerialNumber'");
        }

        let state = state.read().await;
        match state.list_mfa_device_tags(&input.serial_number) {
            Ok(tags) => {
                wire::serialize_list_m_f_a_device_tags_response(&wire::ListMFADeviceTagsResponse {
                    tags: if tags.is_empty() {
                        None
                    } else {
                        Some(tags_to_wire(tags))
                    },
                    is_truncated: Some(false),
                    ..Default::default()
                })
            }
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_resync_m_f_a_device(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_resync_m_f_a_device_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.serial_number.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SerialNumber'");
        }

        // Verify the device exists; resync is a no-op in the mock (no TOTP validation).
        let state = state.read().await;
        match state.get_mfa_device(&input.serial_number) {
            Ok(_) => wire::serialize_resync_m_f_a_device_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    // ==================== SAML provider tag handlers ====================

    async fn handle_tag_s_a_m_l_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_s_a_m_l_provider_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.s_a_m_l_provider_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SAMLProviderArn'");
        }
        let tags: Vec<Tag> = input
            .tags
            .items
            .into_iter()
            .map(|t| Tag {
                key: t.key,
                value: t.value,
            })
            .collect();
        let mut state = state.write().await;
        match state.tag_saml_provider(&input.s_a_m_l_provider_arn, tags) {
            Ok(()) => wire::serialize_tag_s_a_m_l_provider_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_untag_s_a_m_l_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_s_a_m_l_provider_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.s_a_m_l_provider_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SAMLProviderArn'");
        }
        let tag_keys = input.tag_keys.items;
        let mut state = state.write().await;
        match state.untag_saml_provider(&input.s_a_m_l_provider_arn, &tag_keys) {
            Ok(()) => wire::serialize_untag_s_a_m_l_provider_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_s_a_m_l_provider_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_s_a_m_l_provider_tags_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.s_a_m_l_provider_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SAMLProviderArn'");
        }
        let state = state.read().await;
        match state.list_saml_provider_tags(&input.s_a_m_l_provider_arn) {
            Ok(tags) => wire::serialize_list_s_a_m_l_provider_tags_response(
                &wire::ListSAMLProviderTagsResponse {
                    tags: Some(tags_to_wire(tags)),
                    is_truncated: Some(false),
                    ..Default::default()
                },
            ),
            Err(e) => iam_error_response(&e),
        }
    }

    // ==================== Server certificate tag handlers ====================

    async fn handle_tag_server_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_server_certificate_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.server_certificate_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ServerCertificateName'");
        }
        let tags: Vec<Tag> = input
            .tags
            .items
            .into_iter()
            .map(|t| Tag {
                key: t.key,
                value: t.value,
            })
            .collect();
        let mut state = state.write().await;
        match state.tag_server_certificate(&input.server_certificate_name, tags) {
            Ok(()) => wire::serialize_tag_server_certificate_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_untag_server_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_server_certificate_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.server_certificate_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ServerCertificateName'");
        }
        let tag_keys = input.tag_keys.items;
        let mut state = state.write().await;
        match state.untag_server_certificate(&input.server_certificate_name, &tag_keys) {
            Ok(()) => wire::serialize_untag_server_certificate_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_list_server_certificate_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_server_certificate_tags_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.server_certificate_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ServerCertificateName'");
        }
        let state = state.read().await;
        match state.list_server_certificate_tags(&input.server_certificate_name) {
            Ok(tags) => wire::serialize_list_server_certificate_tags_response(
                &wire::ListServerCertificateTagsResponse {
                    tags: Some(tags_to_wire(tags)),
                    is_truncated: Some(false),
                    ..Default::default()
                },
            ),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_update_server_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_server_certificate_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.server_certificate_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ServerCertificateName'");
        }
        let new_path = input.new_path.as_deref();
        let new_name = input.new_server_certificate_name.as_deref();
        let mut state = state.write().await;
        match state.update_server_certificate(&input.server_certificate_name, new_path, new_name) {
            Ok(()) => wire::serialize_update_server_certificate_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    // ==================== SSH public key listing handler ====================

    async fn handle_list_s_s_h_public_keys(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_s_s_h_public_keys_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let user_name = input.user_name.as_deref();
        let state = state.read().await;
        let keys = state.list_ssh_public_keys(user_name);

        wire::serialize_list_s_s_h_public_keys_response(&wire::ListSSHPublicKeysResponse {
            s_s_h_public_keys: Some(
                keys.iter()
                    .map(|k| wire::SSHPublicKeyMetadata {
                        user_name: Some(k.user_name.clone()),
                        s_s_h_public_key_id: Some(k.ssh_public_key_id.clone()),
                        status: Some(k.status.clone()),
                        upload_date: Some(k.upload_date.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                    })
                    .collect(),
            ),
            is_truncated: Some(false),
            ..Default::default()
        })
    }

    // ==================== Instance profiles listing handlers ====================

    async fn handle_list_instance_profiles(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_instance_profiles_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let path_prefix = input.path_prefix.as_deref();
        let state = state.read().await;
        let ips = state.list_instance_profiles(path_prefix);

        wire::serialize_list_instance_profiles_response(&wire::ListInstanceProfilesResponse {
            instance_profiles: Some(
                ips.iter()
                    .map(|ip| make_wire_instance_profile(ip, &state))
                    .collect(),
            ),
            is_truncated: Some(false),
            ..Default::default()
        })
    }

    async fn handle_list_instance_profiles_for_role(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_instance_profiles_for_role_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.role_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RoleName'");
        }
        let state = state.read().await;
        let ips = state.list_instance_profiles_for_role(&input.role_name);

        wire::serialize_list_instance_profiles_for_role_response(
            &wire::ListInstanceProfilesForRoleResponse {
                instance_profiles: Some(
                    ips.iter()
                        .map(|ip| make_wire_instance_profile(ip, &state))
                        .collect(),
                ),
                is_truncated: Some(false),
                ..Default::default()
            },
        )
    }

    // ==================== List groups for user handler ====================

    async fn handle_list_groups_for_user(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_groups_for_user_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        let state = state.read().await;
        match state.list_groups_for_user(&input.user_name) {
            Ok(groups) => {
                wire::serialize_list_groups_for_user_response(&wire::ListGroupsForUserResponse {
                    groups: Some(
                        groups
                            .iter()
                            .map(|g| wire::Group {
                                group_name: Some(g.name.clone()),
                                group_id: Some(g.group_id.clone()),
                                arn: Some(g.arn.clone()),
                                path: Some(g.path.clone()),
                                create_date: Some(
                                    g.create_date.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                                ),
                            })
                            .collect(),
                    ),
                    is_truncated: Some(false),
                    ..Default::default()
                })
            }
            Err(e) => iam_error_response(&e),
        }
    }

    // ==================== List entities for policy handler ====================

    async fn handle_list_entities_for_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_entities_for_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.policy_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PolicyArn'");
        }
        let state = state.read().await;
        match state.list_entities_for_policy(&input.policy_arn) {
            Ok((groups, roles, users)) => wire::serialize_list_entities_for_policy_response(
                &wire::ListEntitiesForPolicyResponse {
                    policy_groups: Some(
                        groups
                            .iter()
                            .map(|g| wire::PolicyGroup {
                                group_id: Some(g.group_id.clone()),
                                group_name: Some(g.name.clone()),
                            })
                            .collect(),
                    ),
                    policy_roles: Some(
                        roles
                            .iter()
                            .map(|r| wire::PolicyRole {
                                role_id: Some(r.role_id.clone()),
                                role_name: Some(r.name.clone()),
                            })
                            .collect(),
                    ),
                    policy_users: Some(
                        users
                            .iter()
                            .map(|u| wire::PolicyUser {
                                user_id: Some(u.user_id.clone()),
                                user_name: Some(u.name.clone()),
                            })
                            .collect(),
                    ),
                    is_truncated: Some(false),
                    ..Default::default()
                },
            ),
            Err(e) => iam_error_response(&e),
        }
    }

    // STUB[no-engine]: Determining which policies grant access to a given service requires
    //   full policy evaluation across all attached policies; returns empty results.
    async fn handle_list_policies_granting_service_access(
        &self,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_list_policies_granting_service_access_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        wire::serialize_list_policies_granting_service_access_response(
            &wire::ListPoliciesGrantingServiceAccessResponse {
                is_truncated: Some(false),
                ..Default::default()
            },
        )
    }

    // STUB[org-integration]: Organizations feature flags require real Organizations
    //   service integration; returns empty feature list.
    async fn handle_list_organizations_features(&self) -> MockResponse {
        wire::serialize_list_organizations_features_response(
            &wire::ListOrganizationsFeaturesResponse {
                ..Default::default()
            },
        )
    }

    // STUB[org-integration]: Outbound web identity federation is an Organizations-level
    //   feature; mock has no federation state to manage.
    async fn handle_disable_outbound_web_identity_federation(&self) -> MockResponse {
        wire::serialize_disable_outbound_web_identity_federation_response()
    }

    // STUB[org-integration]: see handle_disable_outbound_web_identity_federation.
    async fn handle_enable_outbound_web_identity_federation(&self) -> MockResponse {
        wire::serialize_enable_outbound_web_identity_federation_response(
            &wire::EnableOutboundWebIdentityFederationResponse {
                ..Default::default()
            },
        )
    }

    // STUB[org-integration]: see handle_disable_outbound_web_identity_federation.
    async fn handle_get_outbound_web_identity_federation_info(&self) -> MockResponse {
        wire::serialize_get_outbound_web_identity_federation_info_response(
            &wire::GetOutboundWebIdentityFederationInfoResponse {
                ..Default::default()
            },
        )
    }

    async fn handle_disable_organizations_root_credentials_management(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
    ) -> MockResponse {
        let mut state = state.write().await;
        let remaining = state.disable_organizations_root_credentials_management();
        let enabled_features = if remaining.is_empty() {
            None
        } else {
            Some(wire::FeaturesListType { items: remaining })
        };
        wire::serialize_disable_organizations_root_credentials_management_response(
            &wire::DisableOrganizationsRootCredentialsManagementResponse {
                enabled_features,
                organization_id: Some("o-mock000000".to_string()),
            },
        )
    }

    async fn handle_disable_organizations_root_sessions(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
    ) -> MockResponse {
        let mut state = state.write().await;
        let remaining = state.disable_organizations_root_sessions();
        let enabled_features = if remaining.is_empty() {
            None
        } else {
            Some(wire::FeaturesListType { items: remaining })
        };
        wire::serialize_disable_organizations_root_sessions_response(
            &wire::DisableOrganizationsRootSessionsResponse {
                enabled_features,
                organization_id: Some("o-mock000000".to_string()),
            },
        )
    }

    async fn handle_enable_organizations_root_credentials_management(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
    ) -> MockResponse {
        let mut state = state.write().await;
        let features = state.enable_organizations_root_credentials_management("o-mock000000");
        wire::serialize_enable_organizations_root_credentials_management_response(
            &wire::EnableOrganizationsRootCredentialsManagementResponse {
                enabled_features: Some(wire::FeaturesListType { items: features }),
                organization_id: Some("o-mock000000".to_string()),
            },
        )
    }

    async fn handle_enable_organizations_root_sessions(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
    ) -> MockResponse {
        let mut state = state.write().await;
        let features = state.enable_organizations_root_sessions("o-mock000000");
        wire::serialize_enable_organizations_root_sessions_response(
            &wire::EnableOrganizationsRootSessionsResponse {
                enabled_features: Some(wire::FeaturesListType { items: features }),
                organization_id: Some("o-mock000000".to_string()),
            },
        )
    }

    // STUB[no-engine]: Human readable summary aggregates policy and resource counts
    //   across all IAM entities; not worth implementing without a full policy index.
    async fn handle_get_human_readable_summary(&self) -> MockResponse {
        wire::serialize_get_human_readable_summary_response(
            &wire::GetHumanReadableSummaryResponse {
                ..Default::default()
            },
        )
    }

    // ==================== User permissions boundary handlers ====================

    async fn handle_put_user_permissions_boundary(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_user_permissions_boundary_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        if input.permissions_boundary.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'PermissionsBoundary'");
        }
        let mut state = state.write().await;
        match state.put_user_permissions_boundary(&input.user_name, &input.permissions_boundary) {
            Ok(()) => wire::serialize_put_user_permissions_boundary_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_delete_user_permissions_boundary(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_user_permissions_boundary_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UserName'");
        }
        let mut state = state.write().await;
        match state.delete_user_permissions_boundary(&input.user_name) {
            Ok(()) => wire::serialize_delete_user_permissions_boundary_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    // ==================== OIDC provider client ID handlers ====================

    async fn handle_add_client_i_d_to_open_i_d_connect_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_add_client_i_d_to_open_i_d_connect_provider_request(params) {
                Ok(v) => v,
                Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
            };
        if input.open_i_d_connect_provider_arn.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'OpenIDConnectProviderArn'",
            );
        }
        if input.client_i_d.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClientID'");
        }
        let mut state = state.write().await;
        match state
            .add_client_id_to_oidc_provider(&input.open_i_d_connect_provider_arn, &input.client_i_d)
        {
            Ok(()) => wire::serialize_add_client_i_d_to_open_i_d_connect_provider_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    async fn handle_remove_client_i_d_from_open_i_d_connect_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<IamState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_remove_client_i_d_from_open_i_d_connect_provider_request(
            params,
        ) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.open_i_d_connect_provider_arn.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'OpenIDConnectProviderArn'",
            );
        }
        if input.client_i_d.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClientID'");
        }
        let mut state = state.write().await;
        match state.remove_client_id_from_oidc_provider(
            &input.open_i_d_connect_provider_arn,
            &input.client_i_d,
        ) {
            Ok(()) => wire::serialize_remove_client_i_d_from_open_i_d_connect_provider_response(),
            Err(e) => iam_error_response(&e),
        }
    }

    // ==================== STS preferences (stub) ====================

    async fn handle_set_security_token_service_preferences(&self) -> MockResponse {
        wire::serialize_set_security_token_service_preferences_response()
    }
}

// --- Simulation helpers ---

/// Parse a list of JSON policy document strings into evaluation-engine types.
fn parse_policy_input_list(
    policy_jsons: &[String],
    policy_type: &str,
) -> Result<
    Vec<(
        winterbaume_iam_rule_eval::PolicyDocument,
        winterbaume_iam_rule_eval::PolicySource,
    )>,
    MockResponse,
> {
    let mut out = Vec::with_capacity(policy_jsons.len());
    for (i, json) in policy_jsons.iter().enumerate() {
        let doc: winterbaume_iam_rule_eval::PolicyDocument =
            serde_json::from_str(json).map_err(|e| {
                MockResponse::error(
                    400,
                    "InvalidInput",
                    &format!("Invalid policy document at index {i}: {e}"),
                )
            })?;
        let source = winterbaume_iam_rule_eval::PolicySource {
            id: format!("PolicyInputList.{}", i + 1),
            policy_type: policy_type.to_string(),
        };
        out.push((doc, source));
    }
    Ok(out)
}

/// Run the IAM evaluation engine for every action x resource combination and
/// build the wire-format `EvaluationResult` list.
fn run_simulation(
    policies: &[(
        winterbaume_iam_rule_eval::PolicyDocument,
        winterbaume_iam_rule_eval::PolicySource,
    )],
    action_names: &[String],
    resource_list: &[String],
) -> Vec<wire::EvaluationResult> {
    let ctx = winterbaume_iam_rule_eval::SimulationContext::default();
    let mut results = Vec::new();

    for action in action_names {
        for resource in resource_list {
            let outcome = winterbaume_iam_rule_eval::evaluate(policies, action, resource, &ctx);

            let matched_stmts: Vec<wire::Statement> = outcome
                .matched_statements
                .iter()
                .map(|ms| wire::Statement {
                    source_policy_id: Some(ms.source_policy_id.clone()),
                    source_policy_type: Some(ms.source_policy_type.clone()),
                    start_position: ms.start_position.map(|(line, col)| wire::Position {
                        line: Some(line as i32),
                        column: Some(col as i32),
                    }),
                    end_position: ms.end_position.map(|(line, col)| wire::Position {
                        line: Some(line as i32),
                        column: Some(col as i32),
                    }),
                })
                .collect();

            let missing_ctx: Vec<String> = outcome.missing_context_keys;

            results.push(wire::EvaluationResult {
                eval_action_name: Some(action.clone()),
                eval_resource_name: Some(resource.clone()),
                eval_decision: Some(outcome.decision.as_aws_str().to_string()),
                matched_statements: if matched_stmts.is_empty() {
                    None
                } else {
                    Some(matched_stmts.into())
                },
                missing_context_values: if missing_ctx.is_empty() {
                    None
                } else {
                    Some(missing_ctx.into())
                },
                ..Default::default()
            });
        }
    }
    results
}

/// Collect all policies (inline + attached managed) for a principal identified
/// by ARN, including group-inherited policies for users.
/// Returns `true` if the principal was found.
fn collect_principal_policies(
    state: &IamState,
    arn: &str,
    out: &mut Vec<(
        winterbaume_iam_rule_eval::PolicyDocument,
        winterbaume_iam_rule_eval::PolicySource,
    )>,
) -> bool {
    // Try user
    if let Some(user) = state.users.values().find(|u| u.arn == arn) {
        collect_inline_policies(&user.inline_policies, &user.name, "IAMUserPolicy", out);
        collect_attached_policies(&user.attached_policies, state, "IAMUserPolicy", out);
        // Group-inherited policies
        for group in state.groups.values() {
            if group.members.contains(&user.name) {
                collect_inline_policies(&group.inline_policies, &group.name, "IAMGroupPolicy", out);
                collect_attached_policies(&group.attached_policies, state, "IAMGroupPolicy", out);
            }
        }
        return true;
    }

    // Try role
    if let Some(role) = state.roles.values().find(|r| r.arn == arn) {
        collect_inline_policies(&role.inline_policies, &role.name, "IAMRolePolicy", out);
        collect_attached_policies(&role.attached_policies, state, "IAMRolePolicy", out);
        return true;
    }

    // Try group
    if let Some(group) = state.groups.values().find(|g| g.arn == arn) {
        collect_inline_policies(&group.inline_policies, &group.name, "IAMGroupPolicy", out);
        collect_attached_policies(&group.attached_policies, state, "IAMGroupPolicy", out);
        return true;
    }

    false
}

/// Parse inline policy documents and add them to the output list.
fn collect_inline_policies(
    inline_policies: &[crate::types::InlinePolicy],
    principal_name: &str,
    policy_type: &str,
    out: &mut Vec<(
        winterbaume_iam_rule_eval::PolicyDocument,
        winterbaume_iam_rule_eval::PolicySource,
    )>,
) {
    for ip in inline_policies {
        if let Ok(doc) =
            serde_json::from_str::<winterbaume_iam_rule_eval::PolicyDocument>(&ip.policy_document)
        {
            out.push((
                doc,
                winterbaume_iam_rule_eval::PolicySource {
                    id: format!("{principal_name}:{}", ip.policy_name),
                    policy_type: policy_type.to_string(),
                },
            ));
        }
    }
}

/// Look up attached managed policies and add them to the output list.
fn collect_attached_policies(
    attached: &[crate::types::AttachedPolicy],
    state: &IamState,
    policy_type: &str,
    out: &mut Vec<(
        winterbaume_iam_rule_eval::PolicyDocument,
        winterbaume_iam_rule_eval::PolicySource,
    )>,
) {
    for ap in attached {
        if let Some(managed) = state.policies.get(&ap.policy_arn) {
            if let Ok(doc) =
                serde_json::from_str::<winterbaume_iam_rule_eval::PolicyDocument>(&managed.document)
            {
                out.push((
                    doc,
                    winterbaume_iam_rule_eval::PolicySource {
                        id: ap.policy_arn.clone(),
                        policy_type: policy_type.to_string(),
                    },
                ));
            }
        }
    }
}

// --- Utility functions ---

/// Extract condition key names from a list of IAM policy JSON documents.
///
/// Walks each policy's `Statement[].Condition` object and collects the operator keys
/// (e.g. `aws:RequestedRegion`, `s3:prefix`). Duplicate keys are deduplicated and
/// the result is sorted for deterministic output.
fn extract_context_keys_from_policies(policy_docs: &[String]) -> Vec<String> {
    use std::collections::BTreeSet;
    let mut keys: BTreeSet<String> = BTreeSet::new();

    for doc in policy_docs {
        let Ok(value) = serde_json::from_str::<serde_json::Value>(doc) else {
            continue;
        };
        let Some(statements) = value.get("Statement").and_then(|s| s.as_array()) else {
            continue;
        };
        for stmt in statements {
            let Some(condition) = stmt.get("Condition").and_then(|c| c.as_object()) else {
                continue;
            };
            // condition = { "<operator>": { "<key>": <value>, ... }, ... }
            for (_operator, key_map) in condition {
                let Some(key_obj) = key_map.as_object() else {
                    continue;
                };
                for key in key_obj.keys() {
                    keys.insert(key.to_lowercase());
                }
            }
        }
    }

    keys.into_iter().collect()
}

fn iam_error_response(err: &IamError) -> MockResponse {
    let (status, code) = match err {
        // EntityAlreadyExists (409)
        IamError::UserAlreadyExists(_) => (409, "EntityAlreadyExists"),
        IamError::RoleAlreadyExists(_) => (409, "EntityAlreadyExists"),
        IamError::GroupAlreadyExists(_) => (409, "EntityAlreadyExists"),
        IamError::GroupAlreadyExistsRename(_) => (409, "EntityAlreadyExists"),
        IamError::PolicyAlreadyExists(_) => (409, "EntityAlreadyExists"),
        IamError::InstanceProfileAlreadyExists(_) => (409, "EntityAlreadyExists"),
        IamError::LoginProfileAlreadyExists(_) => (409, "EntityAlreadyExists"),
        IamError::OidcProviderAlreadyExists => (409, "EntityAlreadyExists"),
        IamError::SamlProviderAlreadyExists(_) => (409, "EntityAlreadyExists"),
        IamError::MfaDeviceAlreadyExists(_) => (409, "EntityAlreadyExists"),
        IamError::AccountAliasAlreadyExists(_) => (409, "EntityAlreadyExists"),
        IamError::ServerCertificateAlreadyExists(_) => (409, "EntityAlreadyExists"),
        // NoSuchEntity (404)
        IamError::UserNotFound(_) => (404, "NoSuchEntity"),
        IamError::UserNotFoundAlt(_) => (404, "NoSuchEntity"),
        IamError::UserNotFoundSsc(_) => (404, "NoSuchEntity"),
        IamError::RoleNotFound(_) => (404, "NoSuchEntity"),
        IamError::GroupNotFound(_) => (404, "NoSuchEntity"),
        IamError::UserNotInGroup(_, _) => (404, "NoSuchEntity"),
        IamError::AccessKeyNotFound(_) => (404, "NoSuchEntity"),
        IamError::PolicyNotFound(_) => (404, "NoSuchEntity"),
        IamError::PolicyNotFoundDelete(_) => (404, "NoSuchEntity"),
        IamError::PolicyNotAttachable(_) => (404, "NoSuchEntity"),
        IamError::PolicyNotFoundOrNotAttachable(_) => (404, "NoSuchEntity"),
        IamError::PolicyVersionNotFound(_, _) => (404, "NoSuchEntity"),
        IamError::PolicyVersionNotFoundSimple(_) => (404, "NoSuchEntity"),
        IamError::PolicyNotOnRole(_, _) => (404, "NoSuchEntity"),
        IamError::PolicyNotOnUser(_, _) => (404, "NoSuchEntity"),
        IamError::PolicyNotOnGroup(_, _) => (404, "NoSuchEntity"),
        IamError::UserInlinePolicyNotFound(_) => (404, "NoSuchEntity"),
        IamError::RoleInlinePolicyNotFound(_) => (404, "NoSuchEntity"),
        IamError::GroupInlinePolicyNotFound(_) => (404, "NoSuchEntity"),
        IamError::InstanceProfileNotFound(_) => (404, "NoSuchEntity"),
        IamError::RoleNotInInstanceProfile(_, _) => (404, "NoSuchEntity"),
        IamError::LoginProfileNotFound(_) => (404, "NoSuchEntity"),
        IamError::OidcProviderNotFound(_) => (404, "NoSuchEntity"),
        IamError::SamlProviderNotFound(_) => (404, "NoSuchEntity"),
        IamError::MfaDeviceNotFound(_) => (404, "NoSuchEntity"),
        IamError::MfaDeviceNotAssociated(_, _) => (404, "NoSuchEntity"),
        IamError::PasswordPolicyNotFound => (404, "NoSuchEntity"),
        IamError::AccountAliasNotFound(_) => (404, "NoSuchEntity"),
        IamError::ServerCertificateNotFound(_) => (404, "NoSuchEntity"),
        IamError::SshPublicKeyNotFound(_) => (404, "NoSuchEntity"),
        IamError::SigningCertificateNotFound(_) => (404, "NoSuchEntity"),
        IamError::DeletionTaskNotFound(_) => (404, "NoSuchEntity"),
        IamError::ServiceSpecificCredentialNotFound(_) => (404, "NoSuchEntity"),
        IamError::ServiceSpecificCredentialWrongUser(_, _) => (404, "NoSuchEntity"),
        // DeleteConflict (409)
        IamError::DeleteConflictUserHasAccessKeys(_) => (409, "DeleteConflict"),
        IamError::DeleteConflictUserHasAttachedPolicies(_) => (409, "DeleteConflict"),
        IamError::DeleteConflictUserHasInlinePolicies(_) => (409, "DeleteConflict"),
        IamError::DeleteConflictRoleHasAttachedPolicies(_) => (409, "DeleteConflict"),
        IamError::DeleteConflictRoleHasInlinePolicies(_) => (409, "DeleteConflict"),
        IamError::DeleteConflictRoleInInstanceProfile => (409, "DeleteConflict"),
        IamError::DeleteConflictGroupHasMembers => (409, "DeleteConflict"),
        IamError::DeleteConflictPolicyAttached => (409, "DeleteConflict"),
        IamError::DeleteConflictInstanceProfileHasRoles => (409, "DeleteConflict"),
        IamError::DeleteConflictPolicyDefaultVersion => (409, "DeleteConflict"),
        // LimitExceeded (409)
        IamError::LimitExceededAccessKeys(_) => (409, "LimitExceeded"),
        IamError::LimitExceededPolicyVersions(_) => (409, "LimitExceeded"),
        IamError::LimitExceededRoleAlreadyInInstanceProfile(_, _) => (409, "LimitExceeded"),
        // InvalidInput (400)
        IamError::ServiceRoleNameTaken(_) => (400, "InvalidInput"),
    };
    let body = format!(
        r#"<ErrorResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <Error>
    <Type>Sender</Type>
    <Code>{code}</Code>
    <Message>{message}</Message>
  </Error>
  <RequestId>{request_id}</RequestId>
</ErrorResponse>"#,
        message = xml_escape(&err.to_string()),
        request_id = uuid::Uuid::new_v4(),
    );
    MockResponse::xml(status, body)
}

fn parse_query_string(s: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for pair in s.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            let key = urldecode(key);
            let value = urldecode(value);
            map.insert(key, value);
        }
    }
    map
}

fn urldecode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.bytes();
    while let Some(b) = chars.next() {
        match b {
            b'+' => result.push(' '),
            b'%' => {
                let hi = chars.next().and_then(hex_val);
                let lo = chars.next().and_then(hex_val);
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    result.push((hi << 4 | lo) as char);
                }
            }
            _ => result.push(b as char),
        }
    }
    result
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Convert domain tags to wire tags.
fn tags_to_wire(tags: &[Tag]) -> wire::tagListType {
    tags.iter()
        .map(|t| wire::Tag {
            key: t.key.clone(),
            value: t.value.clone(),
        })
        .collect()
}

/// Convert a domain Role to a wire Role.
fn make_wire_role(role: &crate::types::Role) -> wire::Role {
    wire::Role {
        arn: Some(role.arn.clone()),
        create_date: Some(role.create_date.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
        path: Some(role.path.clone()),
        role_id: Some(role.role_id.clone()),
        role_name: Some(role.name.clone()),
        assume_role_policy_document: Some(role.assume_role_policy_document.clone()),
        description: if role.description.is_empty() {
            None
        } else {
            Some(role.description.clone())
        },
        max_session_duration: Some(role.max_session_duration),
        permissions_boundary: role.permissions_boundary.as_ref().map(|pb| {
            wire::AttachedPermissionsBoundary {
                permissions_boundary_arn: Some(pb.clone()),
                permissions_boundary_type: Some("Policy".to_string()),
            }
        }),
        tags: if role.tags.is_empty() {
            None
        } else {
            Some(tags_to_wire(&role.tags))
        },
        ..Default::default()
    }
}

/// Convert a domain InstanceProfile to a wire InstanceProfile.
fn make_wire_instance_profile(
    ip: &crate::types::InstanceProfile,
    state: &IamState,
) -> wire::InstanceProfile {
    let wire_roles: Vec<wire::Role> = ip
        .roles
        .iter()
        .filter_map(|role_name| state.roles.get(role_name))
        .map(make_wire_role)
        .collect();

    wire::InstanceProfile {
        arn: Some(ip.arn.clone()),
        create_date: Some(ip.create_date.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
        instance_profile_id: Some(ip.instance_profile_id.clone()),
        instance_profile_name: Some(ip.name.clone()),
        path: Some(ip.path.clone()),
        roles: Some(wire_roles.into()),
        tags: if ip.tags.is_empty() {
            None
        } else {
            Some(tags_to_wire(&ip.tags))
        },
    }
}

/// Simple base64 encoder (no padding).
fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let n = (b0 << 16) | (b1 << 8) | b2;
        result.push(CHARS[((n >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((n >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARS[((n >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARS[(n & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_query_string() {
        let qs = "Action=GetCallerIdentity&Version=2011-06-15";
        let params = parse_query_string(qs);
        assert_eq!(params.get("Action").unwrap(), "GetCallerIdentity");
        assert_eq!(params.get("Version").unwrap(), "2011-06-15");
    }

    #[test]
    fn test_urldecode() {
        assert_eq!(urldecode("hello+world"), "hello world");
        assert_eq!(urldecode("foo%20bar"), "foo bar");
        assert_eq!(urldecode("a%3Db"), "a=b");
    }

    #[test]
    fn test_xml_escape() {
        assert_eq!(xml_escape("a&b<c>d"), "a&amp;b&lt;c&gt;d");
    }
}
