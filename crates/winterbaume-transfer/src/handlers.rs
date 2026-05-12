use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{TransferError, TransferState};
use crate::types::Tag;
use crate::views::TransferStateView;
use crate::wire;

fn convert_tags(tags: Option<Vec<wire::Tag>>) -> Vec<Tag> {
    tags.unwrap_or_default()
        .into_iter()
        .map(|t| Tag {
            key: t.key,
            value: t.value,
        })
        .collect()
}

pub struct TransferService {
    pub(crate) state: Arc<BackendState<TransferState>>,
    pub(crate) notifier: StateChangeNotifier<TransferStateView>,
}

impl TransferService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for TransferService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for TransferService {
    fn service_name(&self) -> &str {
        "transfer"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://transfer\..*\.amazonaws\.com",
            r"https?://transfer\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl TransferService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        // Extract action from X-Amz-Target header
        // Format: "TransferService.CreateServer"
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
            "CreateServer" => {
                self.handle_create_server(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeServer" => self.handle_describe_server(&state, body_bytes).await,
            "DeleteServer" => self.handle_delete_server(&state, body_bytes).await,
            "ListServers" => self.handle_list_servers(&state).await,
            "UpdateServer" => self.handle_update_server(&state, body_bytes).await,
            "CreateUser" => {
                self.handle_create_user(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeUser" => self.handle_describe_user(&state, body_bytes).await,
            "DeleteUser" => self.handle_delete_user(&state, body_bytes).await,
            "ListUsers" => self.handle_list_users(&state, body_bytes).await,
            "UpdateUser" => self.handle_update_user(&state, body_bytes).await,
            "ImportSshPublicKey" => self.handle_import_ssh_public_key(&state, body_bytes).await,
            "DeleteSshPublicKey" => self.handle_delete_ssh_public_key(&state, body_bytes).await,
            // Agreements
            "CreateAgreement" => {
                self.handle_create_agreement(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeAgreement" => self.handle_describe_agreement(&state, body_bytes).await,
            "ListAgreements" => self.handle_list_agreements(&state, body_bytes).await,
            "UpdateAgreement" => self.handle_update_agreement(&state, body_bytes).await,
            "DeleteAgreement" => self.handle_delete_agreement(&state, body_bytes).await,
            // Certificates
            "ImportCertificate" => {
                self.handle_import_certificate(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeCertificate" => self.handle_describe_certificate(&state, body_bytes).await,
            "ListCertificates" => self.handle_list_certificates(&state).await,
            "UpdateCertificate" => self.handle_update_certificate(&state, body_bytes).await,
            "DeleteCertificate" => self.handle_delete_certificate(&state, body_bytes).await,
            // Connectors
            "CreateConnector" => {
                self.handle_create_connector(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeConnector" => self.handle_describe_connector(&state, body_bytes).await,
            "ListConnectors" => self.handle_list_connectors(&state).await,
            "UpdateConnector" => self.handle_update_connector(&state, body_bytes).await,
            "DeleteConnector" => self.handle_delete_connector(&state, body_bytes).await,
            // Profiles
            "CreateProfile" => {
                self.handle_create_profile(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeProfile" => self.handle_describe_profile(&state, body_bytes).await,
            "ListProfiles" => self.handle_list_profiles(&state, body_bytes).await,
            "UpdateProfile" => self.handle_update_profile(&state, body_bytes).await,
            "DeleteProfile" => self.handle_delete_profile(&state, body_bytes).await,
            // WebApps
            "CreateWebApp" => {
                self.handle_create_web_app(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeWebApp" => self.handle_describe_web_app(&state, body_bytes).await,
            "ListWebApps" => self.handle_list_web_apps(&state).await,
            "UpdateWebApp" => self.handle_update_web_app(&state, body_bytes).await,
            "DeleteWebApp" => self.handle_delete_web_app(&state, body_bytes).await,
            "DescribeWebAppCustomization" => {
                self.handle_describe_web_app_customization(&state, body_bytes)
                    .await
            }
            "UpdateWebAppCustomization" => {
                self.handle_update_web_app_customization(&state, body_bytes)
                    .await
            }
            "DeleteWebAppCustomization" => {
                self.handle_delete_web_app_customization(&state, body_bytes)
                    .await
            }
            // Workflows
            "CreateWorkflow" => {
                self.handle_create_workflow(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeWorkflow" => self.handle_describe_workflow(&state, body_bytes).await,
            "ListWorkflows" => self.handle_list_workflows(&state).await,
            "DeleteWorkflow" => self.handle_delete_workflow(&state, body_bytes).await,
            // --- Unimplemented operations (auto-generated stubs) ---
            "CreateAccess" => json_error_response(
                501,
                "NotImplementedError",
                "CreateAccess is not yet implemented in winterbaume-transfer",
            ),
            "DeleteAccess" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteAccess is not yet implemented in winterbaume-transfer",
            ),
            "DeleteHostKey" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteHostKey is not yet implemented in winterbaume-transfer",
            ),
            "DescribeAccess" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeAccess is not yet implemented in winterbaume-transfer",
            ),
            "DescribeExecution" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeExecution is not yet implemented in winterbaume-transfer",
            ),
            "DescribeHostKey" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeHostKey is not yet implemented in winterbaume-transfer",
            ),
            "DescribeSecurityPolicy" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeSecurityPolicy is not yet implemented in winterbaume-transfer",
            ),
            "ImportHostKey" => json_error_response(
                501,
                "NotImplementedError",
                "ImportHostKey is not yet implemented in winterbaume-transfer",
            ),
            "ListAccesses" => json_error_response(
                501,
                "NotImplementedError",
                "ListAccesses is not yet implemented in winterbaume-transfer",
            ),
            "ListExecutions" => json_error_response(
                501,
                "NotImplementedError",
                "ListExecutions is not yet implemented in winterbaume-transfer",
            ),
            "ListFileTransferResults" => json_error_response(
                501,
                "NotImplementedError",
                "ListFileTransferResults is not yet implemented in winterbaume-transfer",
            ),
            "ListHostKeys" => json_error_response(
                501,
                "NotImplementedError",
                "ListHostKeys is not yet implemented in winterbaume-transfer",
            ),
            "ListSecurityPolicies" => json_error_response(
                501,
                "NotImplementedError",
                "ListSecurityPolicies is not yet implemented in winterbaume-transfer",
            ),
            "ListTagsForResource" => json_error_response(
                501,
                "NotImplementedError",
                "ListTagsForResource is not yet implemented in winterbaume-transfer",
            ),
            "SendWorkflowStepState" => json_error_response(
                501,
                "NotImplementedError",
                "SendWorkflowStepState is not yet implemented in winterbaume-transfer",
            ),
            "StartDirectoryListing" => json_error_response(
                501,
                "NotImplementedError",
                "StartDirectoryListing is not yet implemented in winterbaume-transfer",
            ),
            "StartFileTransfer" => json_error_response(
                501,
                "NotImplementedError",
                "StartFileTransfer is not yet implemented in winterbaume-transfer",
            ),
            "StartRemoteDelete" => json_error_response(
                501,
                "NotImplementedError",
                "StartRemoteDelete is not yet implemented in winterbaume-transfer",
            ),
            "StartRemoteMove" => json_error_response(
                501,
                "NotImplementedError",
                "StartRemoteMove is not yet implemented in winterbaume-transfer",
            ),
            "StartServer" => json_error_response(
                501,
                "NotImplementedError",
                "StartServer is not yet implemented in winterbaume-transfer",
            ),
            "StopServer" => json_error_response(
                501,
                "NotImplementedError",
                "StopServer is not yet implemented in winterbaume-transfer",
            ),
            "TagResource" => json_error_response(
                501,
                "NotImplementedError",
                "TagResource is not yet implemented in winterbaume-transfer",
            ),
            "TestConnection" => json_error_response(
                501,
                "NotImplementedError",
                "TestConnection is not yet implemented in winterbaume-transfer",
            ),
            "TestIdentityProvider" => json_error_response(
                501,
                "NotImplementedError",
                "TestIdentityProvider is not yet implemented in winterbaume-transfer",
            ),
            "UntagResource" => json_error_response(
                501,
                "NotImplementedError",
                "UntagResource is not yet implemented in winterbaume-transfer",
            ),
            "UpdateAccess" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateAccess is not yet implemented in winterbaume-transfer",
            ),
            "UpdateHostKey" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateHostKey is not yet implemented in winterbaume-transfer",
            ),
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for Transfer"),
            ),
        };

        if matches!(
            action.as_str(),
            "CreateServer"
                | "DeleteServer"
                | "UpdateServer"
                | "CreateUser"
                | "DeleteUser"
                | "UpdateUser"
                | "ImportSshPublicKey"
                | "DeleteSshPublicKey"
                | "CreateAgreement"
                | "UpdateAgreement"
                | "DeleteAgreement"
                | "ImportCertificate"
                | "UpdateCertificate"
                | "DeleteCertificate"
                | "CreateConnector"
                | "UpdateConnector"
                | "DeleteConnector"
                | "CreateProfile"
                | "UpdateProfile"
                | "DeleteProfile"
                | "CreateWebApp"
                | "UpdateWebApp"
                | "DeleteWebApp"
                | "UpdateWebAppCustomization"
                | "DeleteWebAppCustomization"
                | "CreateWorkflow"
                | "DeleteWorkflow"
        ) && response.status / 100 == 2
        {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_server(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_server_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let endpoint_type = input.endpoint_type.as_deref().unwrap_or("PUBLIC");
        let identity_provider_type = input
            .identity_provider_type
            .as_deref()
            .unwrap_or("SERVICE_MANAGED");
        let protocols = input.protocols.unwrap_or_else(|| vec!["SFTP".to_string()]);
        let domain = input.domain.as_deref().unwrap_or("S3");
        let tags = convert_tags(input.tags);

        let mut state = state.write().await;
        match state.create_server(
            account_id,
            region,
            endpoint_type,
            identity_provider_type,
            protocols,
            domain,
            tags,
        ) {
            Ok(server) => wire::serialize_create_server_response(&wire::CreateServerResponse {
                server_id: Some(server.server_id.clone()),
            }),
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_describe_server(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_server_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.server_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ServerId'");
        }

        let state = state.read().await;
        match state.describe_server(&input.server_id) {
            Ok(server) => wire::serialize_describe_server_response(&wire::DescribeServerResponse {
                server: Some(build_wire_described_server(server)),
            }),
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_delete_server(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_server_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.server_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ServerId'");
        }

        let mut state = state.write().await;
        match state.delete_server(&input.server_id) {
            Ok(()) => wire::serialize_delete_server_response(),
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_list_servers(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let servers = state.list_servers();
        let entries: Vec<wire::ListedServer> = servers
            .iter()
            .map(|s| wire::ListedServer {
                arn: Some(s.arn.clone()),
                server_id: Some(s.server_id.clone()),
                state: Some(s.state.clone()),
                endpoint_type: Some(s.endpoint_type.clone()),
                identity_provider_type: Some(s.identity_provider_type.clone()),
                domain: Some(s.domain.clone()),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_servers_response(&wire::ListServersResponse {
            servers: if entries.is_empty() {
                None
            } else {
                Some(entries)
            },
            next_token: None,
        })
    }

    async fn handle_update_server(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_server_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.server_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ServerId'");
        }

        let mut state = state.write().await;
        match state.update_server(
            &input.server_id,
            input.protocols,
            input.endpoint_type.as_deref(),
            input.logging_role.as_deref(),
            input.certificate.as_deref(),
            input.security_policy_name.as_deref(),
        ) {
            Ok(id) => wire::serialize_update_server_response(&wire::UpdateServerResponse {
                server_id: Some(id),
            }),
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_create_user(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_user_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.server_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ServerId'");
        }
        if input.user_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserName'");
        }
        let tags = convert_tags(input.tags);

        let mut state = state.write().await;
        match state.create_user(
            &input.server_id,
            &input.user_name,
            &input.role,
            input.home_directory.as_deref(),
            tags,
            account_id,
            region,
        ) {
            Ok(user) => wire::serialize_create_user_response(&wire::CreateUserResponse {
                server_id: Some(user.server_id.clone()),
                user_name: Some(user.user_name.clone()),
            }),
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_describe_user(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_user_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.server_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ServerId'");
        }
        if input.user_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserName'");
        }
        let server_id = input.server_id.as_str();
        let user_name = input.user_name.as_str();

        let state = state.read().await;
        match state.describe_user(server_id, user_name) {
            Ok(user) => {
                let ssh_keys: Vec<wire::SshPublicKey> = user
                    .ssh_public_keys
                    .iter()
                    .map(|k| wire::SshPublicKey {
                        ssh_public_key_id: Some(k.ssh_public_key_id.clone()),
                        ssh_public_key_body: Some(k.ssh_public_key_body.clone()),
                        date_imported: Some(k.date_imported.timestamp() as f64),
                    })
                    .collect();
                let tags: Vec<wire::Tag> = user
                    .tags
                    .iter()
                    .map(|t| wire::Tag {
                        key: t.key.clone(),
                        value: t.value.clone(),
                    })
                    .collect();
                wire::serialize_describe_user_response(&wire::DescribeUserResponse {
                    server_id: Some(server_id.to_string()),
                    user: Some(wire::DescribedUser {
                        arn: Some(user.arn.clone()),
                        home_directory: user.home_directory.clone(),
                        home_directory_type: Some(user.home_directory_type.clone()),
                        role: Some(user.role.clone()),
                        ssh_public_keys: if ssh_keys.is_empty() {
                            None
                        } else {
                            Some(ssh_keys)
                        },
                        tags: if tags.is_empty() { None } else { Some(tags) },
                        user_name: Some(user.user_name.clone()),
                        ..Default::default()
                    }),
                })
            }
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_list_users(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_users_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.server_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ServerId'");
        }
        let server_id = input.server_id.as_str();

        let state = state.read().await;
        match state.list_users(server_id) {
            Ok(users) => {
                let entries: Vec<wire::ListedUser> = users
                    .iter()
                    .map(|u| wire::ListedUser {
                        arn: Some(u.arn.clone()),
                        home_directory: u.home_directory.clone(),
                        home_directory_type: Some(u.home_directory_type.clone()),
                        role: Some(u.role.clone()),
                        ssh_public_key_count: Some(u.ssh_public_keys.len() as i32),
                        user_name: Some(u.user_name.clone()),
                    })
                    .collect();
                wire::serialize_list_users_response(&wire::ListUsersResponse {
                    server_id: Some(server_id.to_string()),
                    users: if entries.is_empty() {
                        None
                    } else {
                        Some(entries)
                    },
                    next_token: None,
                })
            }
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_update_user(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_user_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.server_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ServerId'");
        }
        if input.user_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserName'");
        }

        let mut state = state.write().await;
        match state.update_user(
            &input.server_id,
            &input.user_name,
            input.role.as_deref(),
            input.home_directory.as_deref(),
            input.home_directory_type.as_deref(),
        ) {
            Ok((sid, uname)) => wire::serialize_update_user_response(&wire::UpdateUserResponse {
                server_id: Some(sid),
                user_name: Some(uname),
            }),
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_delete_user(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_user_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.server_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ServerId'");
        }
        if input.user_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserName'");
        }

        let mut state = state.write().await;
        match state.delete_user(&input.server_id, &input.user_name) {
            Ok(()) => wire::serialize_delete_user_response(),
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_import_ssh_public_key(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_import_ssh_public_key_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.server_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ServerId'");
        }
        if input.user_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserName'");
        }
        if input.ssh_public_key_body.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SshPublicKeyBody'");
        }

        let mut state = state.write().await;
        match state.import_ssh_public_key(
            &input.server_id,
            &input.user_name,
            &input.ssh_public_key_body,
        ) {
            Ok(key_id) => {
                wire::serialize_import_ssh_public_key_response(&wire::ImportSshPublicKeyResponse {
                    server_id: Some(input.server_id.clone()),
                    ssh_public_key_id: Some(key_id),
                    user_name: Some(input.user_name.clone()),
                })
            }
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_delete_ssh_public_key(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_ssh_public_key_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.server_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ServerId'");
        }
        if input.user_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'UserName'");
        }
        if input.ssh_public_key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SshPublicKeyId'");
        }

        let mut state = state.write().await;
        match state.delete_ssh_public_key(
            &input.server_id,
            &input.user_name,
            &input.ssh_public_key_id,
        ) {
            Ok(()) => wire::serialize_delete_ssh_public_key_response(),
            Err(e) => transfer_error_response(&e),
        }
    }

    // --- Agreements ---

    async fn handle_create_agreement(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_agreement_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.server_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ServerId'");
        }
        if input.local_profile_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'LocalProfileId'");
        }
        if input.partner_profile_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PartnerProfileId'");
        }
        let base_directory = match input.base_directory.as_deref() {
            Some(d) if !d.is_empty() => d,
            _ => {
                return json_error_response(400, "ValidationException", "Missing 'BaseDirectory'");
            }
        };
        if input.access_role.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AccessRole'");
        }
        let tags = convert_tags(input.tags);

        let mut state = state.write().await;
        match state.create_agreement(
            account_id,
            region,
            &input.server_id,
            &input.local_profile_id,
            &input.partner_profile_id,
            base_directory,
            &input.access_role,
            input.description.as_deref(),
            tags,
        ) {
            Ok(agreement_id) => {
                wire::serialize_create_agreement_response(&wire::CreateAgreementResponse {
                    agreement_id: Some(agreement_id),
                })
            }
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_describe_agreement(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_agreement_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.agreement_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AgreementId'");
        }
        let agreement_id = input.agreement_id.as_str();

        let state = state.read().await;
        match state.describe_agreement(agreement_id) {
            Ok(a) => {
                let tags: Vec<wire::Tag> = a
                    .tags
                    .iter()
                    .map(|t| wire::Tag {
                        key: t.key.clone(),
                        value: t.value.clone(),
                    })
                    .collect();
                wire::serialize_describe_agreement_response(&wire::DescribeAgreementResponse {
                    agreement: Some(wire::DescribedAgreement {
                        agreement_id: Some(a.agreement_id.clone()),
                        arn: Some(a.arn.clone()),
                        server_id: Some(a.server_id.clone()),
                        local_profile_id: Some(a.local_profile_id.clone()),
                        partner_profile_id: Some(a.partner_profile_id.clone()),
                        base_directory: Some(a.base_directory.clone()),
                        access_role: Some(a.access_role.clone()),
                        status: Some(a.status.clone()),
                        description: a.description.clone(),
                        tags: if tags.is_empty() { None } else { Some(tags) },
                        ..Default::default()
                    }),
                })
            }
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_list_agreements(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_agreements_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.server_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ServerId'");
        }
        let server_id = input.server_id.as_str();

        let state = state.read().await;
        match state.list_agreements(server_id) {
            Ok(agreements) => {
                let entries: Vec<wire::ListedAgreement> = agreements
                    .iter()
                    .map(|a| wire::ListedAgreement {
                        agreement_id: Some(a.agreement_id.clone()),
                        arn: Some(a.arn.clone()),
                        server_id: Some(a.server_id.clone()),
                        local_profile_id: Some(a.local_profile_id.clone()),
                        partner_profile_id: Some(a.partner_profile_id.clone()),
                        status: Some(a.status.clone()),
                        description: a.description.clone(),
                    })
                    .collect();
                wire::serialize_list_agreements_response(&wire::ListAgreementsResponse {
                    agreements: if entries.is_empty() {
                        None
                    } else {
                        Some(entries)
                    },
                    next_token: None,
                })
            }
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_update_agreement(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_agreement_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.agreement_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AgreementId'");
        }

        let mut state = state.write().await;
        match state.update_agreement(
            &input.agreement_id,
            input.base_directory.as_deref(),
            input.access_role.as_deref(),
            input.status.as_deref(),
            input.local_profile_id.as_deref(),
            input.partner_profile_id.as_deref(),
            input.description.as_deref(),
        ) {
            Ok(id) => wire::serialize_update_agreement_response(&wire::UpdateAgreementResponse {
                agreement_id: Some(id),
            }),
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_delete_agreement(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_agreement_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.agreement_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AgreementId'");
        }

        let mut state = state.write().await;
        match state.delete_agreement(&input.agreement_id) {
            Ok(()) => wire::serialize_delete_agreement_response(),
            Err(e) => transfer_error_response(&e),
        }
    }

    // --- Certificates ---

    async fn handle_import_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_import_certificate_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.usage.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Usage'");
        }
        if input.certificate.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Certificate'");
        }
        let tags = convert_tags(input.tags);

        let mut state = state.write().await;
        match state.import_certificate(
            account_id,
            region,
            &input.usage,
            &input.certificate,
            input.certificate_chain.as_deref(),
            input.private_key.as_deref(),
            input.active_date,
            input.inactive_date,
            input.description.as_deref(),
            tags,
        ) {
            Ok(cert_id) => {
                wire::serialize_import_certificate_response(&wire::ImportCertificateResponse {
                    certificate_id: Some(cert_id),
                })
            }
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_describe_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_certificate_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'CertificateId'");
        }
        let certificate_id = input.certificate_id.as_str();

        let state = state.read().await;
        match state.describe_certificate(certificate_id) {
            Ok(c) => {
                let tags: Vec<wire::Tag> = c
                    .tags
                    .iter()
                    .map(|t| wire::Tag {
                        key: t.key.clone(),
                        value: t.value.clone(),
                    })
                    .collect();
                wire::serialize_describe_certificate_response(&wire::DescribeCertificateResponse {
                    certificate: Some(wire::DescribedCertificate {
                        certificate_id: Some(c.certificate_id.clone()),
                        arn: Some(c.arn.clone()),
                        usage: Some(c.usage.clone()),
                        status: Some(c.status.clone()),
                        certificate: Some(c.certificate.clone()),
                        certificate_chain: c.certificate_chain.clone(),
                        active_date: c.active_date,
                        inactive_date: c.inactive_date,
                        description: c.description.clone(),
                        r#type: Some(c.certificate_type.clone()),
                        tags: if tags.is_empty() { None } else { Some(tags) },
                        ..Default::default()
                    }),
                })
            }
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_list_certificates(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let certificates = state.list_certificates();
        let entries: Vec<wire::ListedCertificate> = certificates
            .iter()
            .map(|c| wire::ListedCertificate {
                certificate_id: Some(c.certificate_id.clone()),
                arn: Some(c.arn.clone()),
                usage: Some(c.usage.clone()),
                status: Some(c.status.clone()),
                active_date: c.active_date,
                inactive_date: c.inactive_date,
                description: c.description.clone(),
                r#type: Some(c.certificate_type.clone()),
            })
            .collect();
        wire::serialize_list_certificates_response(&wire::ListCertificatesResponse {
            certificates: if entries.is_empty() {
                None
            } else {
                Some(entries)
            },
            next_token: None,
        })
    }

    async fn handle_update_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_certificate_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'CertificateId'");
        }

        let mut state = state.write().await;
        match state.update_certificate(
            &input.certificate_id,
            input.active_date,
            input.inactive_date,
            input.description.as_deref(),
        ) {
            Ok(id) => {
                wire::serialize_update_certificate_response(&wire::UpdateCertificateResponse {
                    certificate_id: Some(id),
                })
            }
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_delete_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_certificate_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'CertificateId'");
        }

        let mut state = state.write().await;
        match state.delete_certificate(&input.certificate_id) {
            Ok(()) => wire::serialize_delete_certificate_response(),
            Err(e) => transfer_error_response(&e),
        }
    }

    // --- Connectors ---

    async fn handle_create_connector(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_connector_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.access_role.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AccessRole'");
        }
        let as2_config = input.as2_config.and_then(|c| serde_json::to_value(c).ok());
        let sftp_config = input.sftp_config.and_then(|c| serde_json::to_value(c).ok());
        let tags = convert_tags(input.tags);

        let mut state = state.write().await;
        match state.create_connector(
            account_id,
            region,
            input.url.as_deref(),
            as2_config,
            sftp_config,
            &input.access_role,
            input.logging_role.as_deref(),
            tags,
        ) {
            Ok(connector_id) => {
                wire::serialize_create_connector_response(&wire::CreateConnectorResponse {
                    connector_id: Some(connector_id),
                })
            }
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_describe_connector(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_connector_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.connector_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ConnectorId'");
        }
        let connector_id = input.connector_id.as_str();

        let state = state.read().await;
        match state.describe_connector(connector_id) {
            Ok(c) => {
                let tags: Vec<wire::Tag> = c
                    .tags
                    .iter()
                    .map(|t| wire::Tag {
                        key: t.key.clone(),
                        value: t.value.clone(),
                    })
                    .collect();
                // Build as2_config and sftp_config from stored JSON
                let as2_config: Option<wire::As2ConnectorConfig> = c
                    .as2_config
                    .as_ref()
                    .and_then(|v| serde_json::from_value(v.clone()).ok());
                let sftp_config: Option<wire::SftpConnectorConfig> = c
                    .sftp_config
                    .as_ref()
                    .and_then(|v| serde_json::from_value(v.clone()).ok());
                wire::serialize_describe_connector_response(&wire::DescribeConnectorResponse {
                    connector: Some(wire::DescribedConnector {
                        connector_id: Some(c.connector_id.clone()),
                        arn: Some(c.arn.clone()),
                        url: c.url.clone(),
                        access_role: Some(c.access_role.clone()),
                        logging_role: c.logging_role.clone(),
                        as2_config,
                        sftp_config,
                        status: Some(c.status.clone()),
                        tags: if tags.is_empty() { None } else { Some(tags) },
                        ..Default::default()
                    }),
                })
            }
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_list_connectors(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let connectors = state.list_connectors();
        let entries: Vec<wire::ListedConnector> = connectors
            .iter()
            .map(|c| wire::ListedConnector {
                connector_id: Some(c.connector_id.clone()),
                arn: Some(c.arn.clone()),
                url: c.url.clone(),
            })
            .collect();
        wire::serialize_list_connectors_response(&wire::ListConnectorsResponse {
            connectors: if entries.is_empty() {
                None
            } else {
                Some(entries)
            },
            next_token: None,
        })
    }

    async fn handle_update_connector(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_connector_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.connector_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ConnectorId'");
        }
        let as2_config = input.as2_config.and_then(|c| serde_json::to_value(c).ok());
        let sftp_config = input.sftp_config.and_then(|c| serde_json::to_value(c).ok());

        let mut state = state.write().await;
        match state.update_connector(
            &input.connector_id,
            input.url.as_deref(),
            as2_config,
            sftp_config,
            input.access_role.as_deref(),
            input.logging_role.as_deref(),
        ) {
            Ok(id) => wire::serialize_update_connector_response(&wire::UpdateConnectorResponse {
                connector_id: Some(id),
            }),
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_delete_connector(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_connector_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.connector_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ConnectorId'");
        }

        let mut state = state.write().await;
        match state.delete_connector(&input.connector_id) {
            Ok(()) => wire::serialize_delete_connector_response(),
            Err(e) => transfer_error_response(&e),
        }
    }

    // --- Profiles ---

    async fn handle_create_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_profile_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.profile_type.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ProfileType'");
        }
        if input.as2_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'As2Id'");
        }
        let certificate_ids = input.certificate_ids.unwrap_or_default();
        let tags = convert_tags(input.tags);

        let mut state = state.write().await;
        match state.create_profile(
            account_id,
            region,
            &input.profile_type,
            &input.as2_id,
            certificate_ids,
            tags,
        ) {
            Ok(profile_id) => {
                wire::serialize_create_profile_response(&wire::CreateProfileResponse {
                    profile_id: Some(profile_id),
                })
            }
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_describe_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_profile_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.profile_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ProfileId'");
        }
        let profile_id = input.profile_id.as_str();

        let state = state.read().await;
        match state.describe_profile(profile_id) {
            Ok(p) => {
                let tags: Vec<wire::Tag> = p
                    .tags
                    .iter()
                    .map(|t| wire::Tag {
                        key: t.key.clone(),
                        value: t.value.clone(),
                    })
                    .collect();
                wire::serialize_describe_profile_response(&wire::DescribeProfileResponse {
                    profile: Some(wire::DescribedProfile {
                        profile_id: Some(p.profile_id.clone()),
                        arn: Some(p.arn.clone()),
                        profile_type: Some(p.profile_type.clone()),
                        as2_id: Some(p.as2_id.clone()),
                        certificate_ids: if p.certificate_ids.is_empty() {
                            None
                        } else {
                            Some(p.certificate_ids.clone())
                        },
                        tags: if tags.is_empty() { None } else { Some(tags) },
                    }),
                })
            }
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_list_profiles(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_profiles_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };

        let state = state.read().await;
        let profiles = state.list_profiles(input.profile_type.as_deref());
        let entries: Vec<wire::ListedProfile> = profiles
            .iter()
            .map(|p| wire::ListedProfile {
                profile_id: Some(p.profile_id.clone()),
                arn: Some(p.arn.clone()),
                profile_type: Some(p.profile_type.clone()),
                as2_id: Some(p.as2_id.clone()),
            })
            .collect();
        wire::serialize_list_profiles_response(&wire::ListProfilesResponse {
            profiles: if entries.is_empty() {
                None
            } else {
                Some(entries)
            },
            next_token: None,
        })
    }

    async fn handle_update_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_profile_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.profile_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ProfileId'");
        }

        let mut state = state.write().await;
        match state.update_profile(&input.profile_id, input.certificate_ids) {
            Ok(id) => wire::serialize_update_profile_response(&wire::UpdateProfileResponse {
                profile_id: Some(id),
            }),
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_delete_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_profile_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.profile_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ProfileId'");
        }

        let mut state = state.write().await;
        match state.delete_profile(&input.profile_id) {
            Ok(()) => wire::serialize_delete_profile_response(),
            Err(e) => transfer_error_response(&e),
        }
    }

    // --- WebApps ---

    async fn handle_create_web_app(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_web_app_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let tags = convert_tags(input.tags);

        let mut state = state.write().await;
        match state.create_web_app(account_id, region, tags) {
            Ok(web_app_id) => {
                wire::serialize_create_web_app_response(&wire::CreateWebAppResponse {
                    web_app_id: Some(web_app_id),
                })
            }
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_describe_web_app(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_web_app_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.web_app_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WebAppId'");
        }
        let web_app_id = input.web_app_id.as_str();

        let state = state.read().await;
        match state.describe_web_app(web_app_id) {
            Ok(w) => {
                let tags: Vec<wire::Tag> = w
                    .tags
                    .iter()
                    .map(|t| wire::Tag {
                        key: t.key.clone(),
                        value: t.value.clone(),
                    })
                    .collect();
                wire::serialize_describe_web_app_response(&wire::DescribeWebAppResponse {
                    web_app: Some(wire::DescribedWebApp {
                        web_app_id: Some(w.web_app_id.clone()),
                        arn: Some(w.arn.clone()),
                        web_app_endpoint: w.web_app_endpoint.clone(),
                        tags: if tags.is_empty() { None } else { Some(tags) },
                        ..Default::default()
                    }),
                })
            }
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_list_web_apps(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let web_apps = state.list_web_apps();
        let entries: Vec<wire::ListedWebApp> = web_apps
            .iter()
            .map(|w| wire::ListedWebApp {
                web_app_id: Some(w.web_app_id.clone()),
                arn: Some(w.arn.clone()),
                web_app_endpoint: w.web_app_endpoint.clone(),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_web_apps_response(&wire::ListWebAppsResponse {
            web_apps: if entries.is_empty() {
                None
            } else {
                Some(entries)
            },
            next_token: None,
        })
    }

    async fn handle_update_web_app(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_web_app_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.web_app_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WebAppId'");
        }

        let mut state = state.write().await;
        match state.update_web_app(&input.web_app_id) {
            Ok(id) => wire::serialize_update_web_app_response(&wire::UpdateWebAppResponse {
                web_app_id: Some(id),
            }),
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_delete_web_app(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_web_app_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.web_app_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WebAppId'");
        }

        let mut state = state.write().await;
        match state.delete_web_app(&input.web_app_id) {
            Ok(()) => wire::serialize_delete_web_app_response(),
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_describe_web_app_customization(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_web_app_customization_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.web_app_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WebAppId'");
        }
        let web_app_id = input.web_app_id.as_str();

        let state = state.read().await;
        match state.describe_web_app_customization(web_app_id) {
            Ok((w, cust)) => {
                let cust_wire = cust.as_ref().map(|c| wire::DescribedWebAppCustomization {
                    web_app_id: Some(w.web_app_id.clone()),
                    arn: Some(w.arn.clone()),
                    title: c.title.clone(),
                    logo_file: c.logo_file.clone(),
                    favicon_file: c.favicon_file.clone(),
                });
                wire::serialize_describe_web_app_customization_response(
                    &wire::DescribeWebAppCustomizationResponse {
                        web_app_customization: cust_wire,
                    },
                )
            }
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_update_web_app_customization(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_web_app_customization_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.web_app_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WebAppId'");
        }

        let mut state = state.write().await;
        match state.update_web_app_customization(
            &input.web_app_id,
            input.title.as_deref(),
            input.logo_file.as_deref(),
            input.favicon_file.as_deref(),
        ) {
            Ok(id) => wire::serialize_update_web_app_customization_response(
                &wire::UpdateWebAppCustomizationResponse {
                    web_app_id: Some(id),
                },
            ),
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_delete_web_app_customization(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_web_app_customization_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.web_app_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WebAppId'");
        }

        let mut state = state.write().await;
        match state.delete_web_app_customization(&input.web_app_id) {
            Ok(()) => wire::serialize_delete_web_app_customization_response(),
            Err(e) => transfer_error_response(&e),
        }
    }

    // --- Workflows ---

    async fn handle_create_workflow(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_workflow_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let steps: Vec<serde_json::Value> = input
            .steps
            .into_iter()
            .filter_map(|s| serde_json::to_value(s).ok())
            .collect();
        let on_exception_steps: Vec<serde_json::Value> = input
            .on_exception_steps
            .unwrap_or_default()
            .into_iter()
            .filter_map(|s| serde_json::to_value(s).ok())
            .collect();
        let tags = convert_tags(input.tags);

        let mut state = state.write().await;
        match state.create_workflow(
            account_id,
            region,
            steps,
            on_exception_steps,
            input.description.as_deref(),
            tags,
        ) {
            Ok(workflow_id) => {
                wire::serialize_create_workflow_response(&wire::CreateWorkflowResponse {
                    workflow_id: Some(workflow_id),
                })
            }
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_describe_workflow(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_workflow_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.workflow_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WorkflowId'");
        }
        let workflow_id = input.workflow_id.as_str();

        let state = state.read().await;
        match state.describe_workflow(workflow_id) {
            Ok(wf) => {
                let tags: Vec<wire::Tag> = wf
                    .tags
                    .iter()
                    .map(|t| wire::Tag {
                        key: t.key.clone(),
                        value: t.value.clone(),
                    })
                    .collect();
                // Convert steps from raw JSON values to WorkflowStep wire types
                let steps: Vec<wire::WorkflowStep> = wf
                    .steps
                    .iter()
                    .filter_map(|v| serde_json::from_value(v.clone()).ok())
                    .collect();
                let on_exception_steps: Vec<wire::WorkflowStep> = wf
                    .on_exception_steps
                    .iter()
                    .filter_map(|v| serde_json::from_value(v.clone()).ok())
                    .collect();
                wire::serialize_describe_workflow_response(&wire::DescribeWorkflowResponse {
                    workflow: Some(wire::DescribedWorkflow {
                        workflow_id: Some(wf.workflow_id.clone()),
                        arn: Some(wf.arn.clone()),
                        description: wf.description.clone(),
                        steps: if steps.is_empty() { None } else { Some(steps) },
                        on_exception_steps: if on_exception_steps.is_empty() {
                            None
                        } else {
                            Some(on_exception_steps)
                        },
                        tags: if tags.is_empty() { None } else { Some(tags) },
                    }),
                })
            }
            Err(e) => transfer_error_response(&e),
        }
    }

    async fn handle_list_workflows(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let workflows = state.list_workflows();
        let entries: Vec<wire::ListedWorkflow> = workflows
            .iter()
            .map(|wf| wire::ListedWorkflow {
                workflow_id: Some(wf.workflow_id.clone()),
                arn: Some(wf.arn.clone()),
                description: wf.description.clone(),
            })
            .collect();
        wire::serialize_list_workflows_response(&wire::ListWorkflowsResponse {
            workflows: if entries.is_empty() {
                None
            } else {
                Some(entries)
            },
            next_token: None,
        })
    }

    async fn handle_delete_workflow(
        &self,
        state: &Arc<tokio::sync::RwLock<TransferState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_workflow_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.workflow_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WorkflowId'");
        }

        let mut state = state.write().await;
        match state.delete_workflow(&input.workflow_id) {
            Ok(()) => wire::serialize_delete_workflow_response(),
            Err(e) => transfer_error_response(&e),
        }
    }
}

fn build_wire_described_server(server: &crate::types::Server) -> wire::DescribedServer {
    let tags: Vec<wire::Tag> = server
        .tags
        .iter()
        .map(|t| wire::Tag {
            key: t.key.clone(),
            value: t.value.clone(),
        })
        .collect();

    wire::DescribedServer {
        arn: Some(server.arn.clone()),
        server_id: Some(server.server_id.clone()),
        state: Some(server.state.clone()),
        endpoint_type: Some(server.endpoint_type.clone()),
        identity_provider_type: Some(server.identity_provider_type.clone()),
        protocols: Some(server.protocols.clone()),
        domain: Some(server.domain.clone()),
        tags: if tags.is_empty() { None } else { Some(tags) },
        ..Default::default()
    }
}

fn transfer_error_response(err: &TransferError) -> MockResponse {
    let (status, error_type) = match err {
        TransferError::ServerNotFound(_) => (400, "ResourceNotFoundException"),
        TransferError::UserNotFound(_) => (400, "ResourceNotFoundException"),
        TransferError::UserAlreadyExists(_) => (409, "ResourceExistsException"),
        TransferError::SshPublicKeyNotFound(_) => (400, "ResourceNotFoundException"),
        TransferError::AgreementNotFound(_) => (400, "ResourceNotFoundException"),
        TransferError::CertificateNotFound(_) => (400, "ResourceNotFoundException"),
        TransferError::ConnectorNotFound(_) => (400, "ResourceNotFoundException"),
        TransferError::ProfileNotFound(_) => (400, "ResourceNotFoundException"),
        TransferError::WebAppNotFound(_) => (400, "ResourceNotFoundException"),
        TransferError::WorkflowNotFound(_) => (400, "ResourceNotFoundException"),
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
