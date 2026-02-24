use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{OrganizationsError, OrganizationsState};
use crate::types::OrgTag;
use crate::views::OrganizationsStateView;
use crate::wire;

pub struct OrganizationsService {
    pub(crate) state: Arc<BackendState<OrganizationsState>>,
    pub(crate) notifier: StateChangeNotifier<OrganizationsStateView>,
}

impl OrganizationsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for OrganizationsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for OrganizationsService {
    fn service_name(&self) -> &str {
        "organizations"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://organizations\..*\.amazonaws\.com",
            r"https?://organizations\.us-east-1\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl OrganizationsService {
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

        let body: Value = match serde_json::from_slice(&request.body) {
            Ok(v) => v,
            Err(_) => {
                return json_error_response(400, "SerializationException", "Invalid JSON body");
            }
        };

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreateOrganization" => self.handle_create_organization(&state, account_id).await,
            "DeleteOrganization" => self.handle_delete_organization(&state).await,
            "DescribeOrganization" => self.handle_describe_organization(&state).await,
            "ListAccounts" => self.handle_list_accounts(&state).await,
            "CreateAccount" => self.handle_create_account(&state, &body, account_id).await,
            "DescribeAccount" => self.handle_describe_account(&state, &body).await,
            "CloseAccount" => self.handle_close_account(&state, &body).await,
            "RemoveAccountFromOrganization" => {
                self.handle_remove_account_from_organization(&state, &body)
                    .await
            }
            "DescribeCreateAccountStatus" => {
                self.handle_describe_create_account_status(&state, &body)
                    .await
            }
            "ListCreateAccountStatus" => {
                self.handle_list_create_account_status(&state, &body).await
            }
            "RegisterDelegatedAdministrator" => {
                self.handle_register_delegated_administrator(&state, &body, account_id)
                    .await
            }
            "ListDelegatedAdministrators" => {
                self.handle_list_delegated_administrators(&state, &body)
                    .await
            }
            "ListDelegatedServicesForAccount" => {
                self.handle_list_delegated_services_for_account(&state, &body)
                    .await
            }
            "DeregisterDelegatedAdministrator" => {
                self.handle_deregister_delegated_administrator(&state, &body, account_id)
                    .await
            }
            "CreateOrganizationalUnit" => {
                self.handle_create_organizational_unit(&state, &body, account_id)
                    .await
            }
            "DescribeOrganizationalUnit" => {
                self.handle_describe_organizational_unit(&state, &body)
                    .await
            }
            "UpdateOrganizationalUnit" => {
                self.handle_update_organizational_unit(&state, &body).await
            }
            "DeleteOrganizationalUnit" => {
                self.handle_delete_organizational_unit(&state, &body).await
            }
            "ListOrganizationalUnitsForParent" => {
                self.handle_list_organizational_units_for_parent(&state, &body)
                    .await
            }
            "ListRoots" => self.handle_list_roots(&state).await,
            "CreatePolicy" => self.handle_create_policy(&state, &body, account_id).await,
            "DescribePolicy" => self.handle_describe_policy(&state, &body).await,
            "UpdatePolicy" => self.handle_update_policy(&state, &body).await,
            "DeletePolicy" => self.handle_delete_policy(&state, &body).await,
            "ListPolicies" => self.handle_list_policies(&state, &body).await,
            "AttachPolicy" => self.handle_attach_policy(&state, &body).await,
            "DetachPolicy" => self.handle_detach_policy(&state, &body).await,
            "ListPoliciesForTarget" => self.handle_list_policies_for_target(&state, &body).await,
            "ListTargetsForPolicy" => self.handle_list_targets_for_policy(&state, &body).await,
            "EnablePolicyType" => self.handle_enable_policy_type(&state, &body).await,
            "DisablePolicyType" => self.handle_disable_policy_type(&state, &body).await,
            "EnableAWSServiceAccess" => self.handle_enable_aws_service_access(&state, &body).await,
            "DisableAWSServiceAccess" => {
                self.handle_disable_aws_service_access(&state, &body).await
            }
            "ListAWSServiceAccessForOrganization" => {
                self.handle_list_aws_service_access_for_organization(&state)
                    .await
            }
            "ListAccountsForParent" => self.handle_list_accounts_for_parent(&state, &body).await,
            "ListChildren" => self.handle_list_children(&state, &body).await,
            "ListParents" => self.handle_list_parents(&state, &body).await,
            "MoveAccount" => self.handle_move_account(&state, &body).await,
            "TagResource" => self.handle_tag_resource(&state, &body).await,
            "UntagResource" => self.handle_untag_resource(&state, &body).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, &body).await,
            // Handshake operations
            "AcceptHandshake" => self.handle_accept_handshake(&state, &body).await,
            "CancelHandshake" => self.handle_cancel_handshake(&state, &body).await,
            "DeclineHandshake" => self.handle_decline_handshake(&state, &body).await,
            "DescribeHandshake" => self.handle_describe_handshake(&state, &body).await,
            "InviteAccountToOrganization" => {
                self.handle_invite_account_to_organization(&state, &body, account_id)
                    .await
            }
            "ListHandshakesForAccount" => self.handle_list_handshakes_for_account(&state).await,
            "ListHandshakesForOrganization" => {
                self.handle_list_handshakes_for_organization(&state).await
            }
            "EnableAllFeatures" => self.handle_enable_all_features(&state, account_id).await,
            // GovCloud
            "CreateGovCloudAccount" => {
                self.handle_create_gov_cloud_account(&state, &body, account_id)
                    .await
            }
            // Resource policy
            "PutResourcePolicy" => {
                self.handle_put_resource_policy(&state, &body, account_id)
                    .await
            }
            "DescribeResourcePolicy" => {
                self.handle_describe_resource_policy(&state, account_id)
                    .await
            }
            "DeleteResourcePolicy" => self.handle_delete_resource_policy(&state).await,
            // Effective policy
            "DescribeEffectivePolicy" => {
                self.handle_describe_effective_policy(&state, &body, account_id)
                    .await
            }
            // Leave org
            "LeaveOrganization" => self.handle_leave_organization(&state).await,
            // List with empty results
            "ListAccountsWithInvalidEffectivePolicy" => {
                self.handle_list_accounts_with_invalid_effective_policy(&body)
                    .await
            }
            "ListEffectivePolicyValidationErrors" => {
                self.handle_list_effective_policy_validation_errors(&body)
                    .await
            }
            // Responsibility transfer operations
            "InviteOrganizationToTransferResponsibility" => {
                self.handle_invite_organization_to_transfer_responsibility(
                    &state, &body, account_id,
                )
                .await
            }
            "DescribeResponsibilityTransfer" => {
                self.handle_describe_responsibility_transfer(&state, &body)
                    .await
            }
            "ListInboundResponsibilityTransfers" => {
                self.handle_list_inbound_responsibility_transfers(&state, account_id)
                    .await
            }
            "ListOutboundResponsibilityTransfers" => {
                self.handle_list_outbound_responsibility_transfers(&state, account_id)
                    .await
            }
            "TerminateResponsibilityTransfer" => {
                self.handle_terminate_responsibility_transfer(&state, &body)
                    .await
            }
            "UpdateResponsibilityTransfer" => {
                self.handle_update_responsibility_transfer(&state, &body)
                    .await
            }
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for Organizations"),
            ),
        };

        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_organization(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        account_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.create_organization(account_id) {
            Ok(org) => {
                wire::serialize_create_organization_response(&wire::CreateOrganizationResponse {
                    organization: Some(organization_wire(org)),
                })
            }
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_delete_organization(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_organization() {
            Ok(()) => wire::serialize_delete_organization_response(),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_describe_organization(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_organization() {
            Ok(org) => wire::serialize_describe_organization_response(
                &wire::DescribeOrganizationResponse {
                    organization: Some(organization_wire(org)),
                },
            ),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_list_accounts(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let accounts = state.list_accounts();
        let entries: Vec<wire::Account> = accounts.iter().map(|a| account_wire(a)).collect();
        wire::serialize_list_accounts_response(&wire::ListAccountsResponse {
            accounts: Some(entries),
            next_token: None,
        })
    }

    async fn handle_create_account(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
        account_id: &str,
    ) -> MockResponse {
        let name = match body.get("AccountName").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return json_error_response(400, "ValidationException", "Missing 'AccountName'");
            }
        };
        let email = match body.get("Email").and_then(|v| v.as_str()) {
            Some(e) => e,
            None => return json_error_response(400, "ValidationException", "Missing 'Email'"),
        };

        let mut state = state.write().await;
        match state.create_account(name, email, account_id) {
            Ok(account) => wire::serialize_create_account_response(&wire::CreateAccountResponse {
                create_account_status: Some(account_to_create_status_wire(account)),
            }),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_describe_account(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let account_id = match body.get("AccountId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'AccountId'"),
        };

        let state = state.read().await;
        match state.describe_account(account_id) {
            Ok(account) => {
                wire::serialize_describe_account_response(&wire::DescribeAccountResponse {
                    account: Some(account_wire(account)),
                })
            }
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_close_account(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let account_id = match body.get("AccountId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'AccountId'"),
        };

        let mut state = state.write().await;
        match state.close_account(account_id) {
            Ok(()) => wire::serialize_close_account_response(),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_remove_account_from_organization(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let account_id = match body.get("AccountId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'AccountId'"),
        };

        let mut state = state.write().await;
        match state.remove_account_from_organization(account_id) {
            Ok(()) => wire::serialize_remove_account_from_organization_response(),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_describe_create_account_status(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let request_id = match body.get("CreateAccountRequestId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing 'CreateAccountRequestId'",
                );
            }
        };

        let state = state.read().await;
        match state.describe_create_account_status(request_id) {
            Ok(account) => wire::serialize_describe_create_account_status_response(
                &wire::DescribeCreateAccountStatusResponse {
                    create_account_status: Some(account_to_create_status_wire(account)),
                },
            ),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_list_create_account_status(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let states_filter: Option<Vec<String>> =
            body.get("States").and_then(|v| v.as_array()).map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            });

        let state = state.read().await;
        let accounts = state.list_create_account_status(states_filter.as_deref());
        let entries: Vec<wire::CreateAccountStatus> = accounts
            .iter()
            .map(|a| account_to_create_status_wire(a))
            .collect();
        wire::serialize_list_create_account_status_response(
            &wire::ListCreateAccountStatusResponse {
                create_account_statuses: Some(entries),
                next_token: None,
            },
        )
    }

    async fn handle_register_delegated_administrator(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
        master_account_id: &str,
    ) -> MockResponse {
        let account_id = match body.get("AccountId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'AccountId'"),
        };
        let service_principal = match body.get("ServicePrincipal").and_then(|v| v.as_str()) {
            Some(sp) => sp,
            None => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing 'ServicePrincipal'",
                );
            }
        };

        let mut state = state.write().await;
        match state.register_delegated_administrator(
            account_id,
            service_principal,
            master_account_id,
        ) {
            Ok(()) => wire::serialize_register_delegated_administrator_response(),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_list_delegated_administrators(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let service_principal = body.get("ServicePrincipal").and_then(|v| v.as_str());

        let state = state.read().await;
        match state.list_delegated_administrators(service_principal) {
            Ok(admins) => {
                let entries: Vec<wire::DelegatedAdministrator> =
                    admins.iter().map(|a| delegated_admin_wire(a)).collect();
                wire::serialize_list_delegated_administrators_response(
                    &wire::ListDelegatedAdministratorsResponse {
                        delegated_administrators: Some(entries),
                        next_token: None,
                    },
                )
            }
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_list_delegated_services_for_account(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let account_id = match body.get("AccountId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'AccountId'"),
        };

        let state = state.read().await;
        match state.list_delegated_services_for_account(account_id) {
            Ok(services) => {
                let entries: Vec<wire::DelegatedService> = services
                    .iter()
                    .map(|s| wire::DelegatedService {
                        service_principal: Some(s.service_principal.clone()),
                        delegation_enabled_date: Some(s.delegation_enabled_date.timestamp() as f64),
                    })
                    .collect();
                wire::serialize_list_delegated_services_for_account_response(
                    &wire::ListDelegatedServicesForAccountResponse {
                        delegated_services: Some(entries),
                        next_token: None,
                    },
                )
            }
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_deregister_delegated_administrator(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
        master_account_id: &str,
    ) -> MockResponse {
        let account_id = match body.get("AccountId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'AccountId'"),
        };
        let service_principal = match body.get("ServicePrincipal").and_then(|v| v.as_str()) {
            Some(sp) => sp,
            None => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing 'ServicePrincipal'",
                );
            }
        };

        let mut state = state.write().await;
        match state.deregister_delegated_administrator(
            account_id,
            service_principal,
            master_account_id,
        ) {
            Ok(()) => wire::serialize_deregister_delegated_administrator_response(),
            Err(e) => organizations_error_response(&e),
        }
    }

    // ==================== Organizational Unit handlers ====================

    async fn handle_create_organizational_unit(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
        account_id: &str,
    ) -> MockResponse {
        let parent_id = match body.get("ParentId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'ParentId'"),
        };
        let name = match body.get("Name").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => return json_error_response(400, "ValidationException", "Missing 'Name'"),
        };

        let mut state = state.write().await;
        match state.create_organizational_unit(parent_id, name, account_id) {
            Ok(ou) => wire::serialize_create_organizational_unit_response(
                &wire::CreateOrganizationalUnitResponse {
                    organizational_unit: Some(ou_wire(ou)),
                },
            ),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_describe_organizational_unit(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let ou_id = match body.get("OrganizationalUnitId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing 'OrganizationalUnitId'",
                );
            }
        };

        let state = state.read().await;
        match state.describe_organizational_unit(ou_id) {
            Ok(ou) => wire::serialize_describe_organizational_unit_response(
                &wire::DescribeOrganizationalUnitResponse {
                    organizational_unit: Some(ou_wire(ou)),
                },
            ),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_update_organizational_unit(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let ou_id = match body.get("OrganizationalUnitId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing 'OrganizationalUnitId'",
                );
            }
        };
        let name = body.get("Name").and_then(|v| v.as_str());

        let mut state = state.write().await;
        match state.update_organizational_unit(ou_id, name) {
            Ok(ou) => wire::serialize_update_organizational_unit_response(
                &wire::UpdateOrganizationalUnitResponse {
                    organizational_unit: Some(ou_wire(ou)),
                },
            ),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_delete_organizational_unit(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let ou_id = match body.get("OrganizationalUnitId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing 'OrganizationalUnitId'",
                );
            }
        };

        let mut state = state.write().await;
        match state.delete_organizational_unit(ou_id) {
            Ok(()) => wire::serialize_delete_organizational_unit_response(),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_list_organizational_units_for_parent(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let parent_id = match body.get("ParentId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'ParentId'"),
        };

        let state = state.read().await;
        match state.list_organizational_units_for_parent(parent_id) {
            Ok(ous) => {
                let entries: Vec<wire::OrganizationalUnit> =
                    ous.iter().map(|ou| ou_wire(ou)).collect();
                wire::serialize_list_organizational_units_for_parent_response(
                    &wire::ListOrganizationalUnitsForParentResponse {
                        organizational_units: Some(entries),
                        next_token: None,
                    },
                )
            }
            Err(e) => organizations_error_response(&e),
        }
    }

    // ==================== Root handlers ====================

    async fn handle_list_roots(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_roots() {
            Ok(roots) => {
                let entries: Vec<wire::Root> = roots.iter().map(|r| root_wire(r)).collect();
                wire::serialize_list_roots_response(&wire::ListRootsResponse {
                    roots: Some(entries),
                    next_token: None,
                })
            }
            Err(e) => organizations_error_response(&e),
        }
    }

    // ==================== Policy handlers ====================

    async fn handle_create_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
        account_id: &str,
    ) -> MockResponse {
        let name = match body.get("Name").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => return json_error_response(400, "ValidationException", "Missing 'Name'"),
        };
        let description = body
            .get("Description")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let content = match body.get("Content").and_then(|v| v.as_str()) {
            Some(c) => c,
            None => return json_error_response(400, "ValidationException", "Missing 'Content'"),
        };
        let policy_type = match body.get("Type").and_then(|v| v.as_str()) {
            Some(t) => t,
            None => return json_error_response(400, "ValidationException", "Missing 'Type'"),
        };

        let mut state = state.write().await;
        match state.create_policy(name, description, content, policy_type, account_id) {
            Ok(policy) => wire::serialize_create_policy_response(&wire::CreatePolicyResponse {
                policy: Some(policy_wire(policy)),
            }),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_describe_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let policy_id = match body.get("PolicyId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'PolicyId'"),
        };

        let state = state.read().await;
        match state.describe_policy(policy_id) {
            Ok(policy) => wire::serialize_describe_policy_response(&wire::DescribePolicyResponse {
                policy: Some(policy_wire(policy)),
            }),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_update_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let policy_id = match body.get("PolicyId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'PolicyId'"),
        };
        let name = body.get("Name").and_then(|v| v.as_str());
        let description = body.get("Description").and_then(|v| v.as_str());
        let content = body.get("Content").and_then(|v| v.as_str());

        let mut state = state.write().await;
        match state.update_policy(policy_id, name, description, content) {
            Ok(policy) => wire::serialize_update_policy_response(&wire::UpdatePolicyResponse {
                policy: Some(policy_wire(policy)),
            }),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_delete_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let policy_id = match body.get("PolicyId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'PolicyId'"),
        };

        let mut state = state.write().await;
        match state.delete_policy(policy_id) {
            Ok(()) => wire::serialize_delete_policy_response(),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_list_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let filter = match body.get("Filter").and_then(|v| v.as_str()) {
            Some(f) => f,
            None => return json_error_response(400, "ValidationException", "Missing 'Filter'"),
        };

        let state = state.read().await;
        match state.list_policies(filter) {
            Ok(policies) => {
                let entries: Vec<wire::PolicySummary> =
                    policies.iter().map(|p| policy_summary_wire(p)).collect();
                wire::serialize_list_policies_response(&wire::ListPoliciesResponse {
                    policies: Some(entries),
                    next_token: None,
                })
            }
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_attach_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let policy_id = match body.get("PolicyId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'PolicyId'"),
        };
        let target_id = match body.get("TargetId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'TargetId'"),
        };

        let mut state = state.write().await;
        match state.attach_policy(policy_id, target_id) {
            Ok(()) => wire::serialize_attach_policy_response(),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_detach_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let policy_id = match body.get("PolicyId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'PolicyId'"),
        };
        let target_id = match body.get("TargetId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'TargetId'"),
        };

        let mut state = state.write().await;
        match state.detach_policy(policy_id, target_id) {
            Ok(()) => wire::serialize_detach_policy_response(),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_list_policies_for_target(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let target_id = match body.get("TargetId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'TargetId'"),
        };
        let filter = match body.get("Filter").and_then(|v| v.as_str()) {
            Some(f) => f,
            None => return json_error_response(400, "ValidationException", "Missing 'Filter'"),
        };

        let state = state.read().await;
        match state.list_policies_for_target(target_id, filter) {
            Ok(policies) => {
                let entries: Vec<wire::PolicySummary> =
                    policies.iter().map(|p| policy_summary_wire(p)).collect();
                wire::serialize_list_policies_for_target_response(
                    &wire::ListPoliciesForTargetResponse {
                        policies: Some(entries),
                        next_token: None,
                    },
                )
            }
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_list_targets_for_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let policy_id = match body.get("PolicyId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'PolicyId'"),
        };

        let state = state.read().await;
        match state.list_targets_for_policy(policy_id) {
            Ok(targets) => {
                let entries: Vec<wire::PolicyTargetSummary> = targets
                    .iter()
                    .map(|t| wire::PolicyTargetSummary {
                        target_id: Some(t.target_id.clone()),
                        arn: Some(t.arn.clone()),
                        name: Some(t.name.clone()),
                        r#type: Some(t.target_type.clone()),
                    })
                    .collect();
                wire::serialize_list_targets_for_policy_response(
                    &wire::ListTargetsForPolicyResponse {
                        targets: Some(entries),
                        next_token: None,
                    },
                )
            }
            Err(e) => organizations_error_response(&e),
        }
    }

    // ==================== Policy type handlers ====================

    async fn handle_enable_policy_type(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let root_id = match body.get("RootId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'RootId'"),
        };
        let policy_type = match body.get("PolicyType").and_then(|v| v.as_str()) {
            Some(pt) => pt,
            None => return json_error_response(400, "ValidationException", "Missing 'PolicyType'"),
        };

        let mut state = state.write().await;
        match state.enable_policy_type(root_id, policy_type) {
            Ok(root) => {
                wire::serialize_enable_policy_type_response(&wire::EnablePolicyTypeResponse {
                    root: Some(root_wire(root)),
                })
            }
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_disable_policy_type(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let root_id = match body.get("RootId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'RootId'"),
        };
        let policy_type = match body.get("PolicyType").and_then(|v| v.as_str()) {
            Some(pt) => pt,
            None => return json_error_response(400, "ValidationException", "Missing 'PolicyType'"),
        };

        let mut state = state.write().await;
        match state.disable_policy_type(root_id, policy_type) {
            Ok(root) => {
                wire::serialize_disable_policy_type_response(&wire::DisablePolicyTypeResponse {
                    root: Some(root_wire(root)),
                })
            }
            Err(e) => organizations_error_response(&e),
        }
    }

    // ==================== Service access handlers ====================

    async fn handle_enable_aws_service_access(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let service_principal = match body.get("ServicePrincipal").and_then(|v| v.as_str()) {
            Some(sp) => sp,
            None => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing 'ServicePrincipal'",
                );
            }
        };

        let mut state = state.write().await;
        match state.enable_aws_service_access(service_principal) {
            Ok(()) => wire::serialize_enable_a_w_s_service_access_response(),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_disable_aws_service_access(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let service_principal = match body.get("ServicePrincipal").and_then(|v| v.as_str()) {
            Some(sp) => sp,
            None => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing 'ServicePrincipal'",
                );
            }
        };

        let mut state = state.write().await;
        match state.disable_aws_service_access(service_principal) {
            Ok(()) => wire::serialize_disable_a_w_s_service_access_response(),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_list_aws_service_access_for_organization(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_aws_service_access_for_organization() {
            Ok(services) => {
                let entries: Vec<wire::EnabledServicePrincipal> = services
                    .iter()
                    .map(|s| wire::EnabledServicePrincipal {
                        service_principal: Some(s.service_principal.clone()),
                        date_enabled: Some(s.date_enabled.timestamp() as f64),
                    })
                    .collect();
                wire::serialize_list_a_w_s_service_access_for_organization_response(
                    &wire::ListAWSServiceAccessForOrganizationResponse {
                        enabled_service_principals: Some(entries),
                        next_token: None,
                    },
                )
            }
            Err(e) => organizations_error_response(&e),
        }
    }

    // ==================== Account parent/child handlers ====================

    async fn handle_list_accounts_for_parent(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let parent_id = match body.get("ParentId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'ParentId'"),
        };

        let state = state.read().await;
        match state.list_accounts_for_parent(parent_id) {
            Ok(accounts) => {
                let entries: Vec<wire::Account> =
                    accounts.iter().map(|a| account_wire(a)).collect();
                wire::serialize_list_accounts_for_parent_response(
                    &wire::ListAccountsForParentResponse {
                        accounts: Some(entries),
                        next_token: None,
                    },
                )
            }
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_list_children(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let parent_id = match body.get("ParentId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'ParentId'"),
        };
        let child_type = match body.get("ChildType").and_then(|v| v.as_str()) {
            Some(ct) => ct,
            None => return json_error_response(400, "ValidationException", "Missing 'ChildType'"),
        };

        let state = state.read().await;
        match state.list_children(parent_id, child_type) {
            Ok(children) => {
                let entries: Vec<wire::Child> = children
                    .iter()
                    .map(|c| wire::Child {
                        id: Some(c.id.clone()),
                        r#type: Some(c.child_type.clone()),
                    })
                    .collect();
                wire::serialize_list_children_response(&wire::ListChildrenResponse {
                    children: Some(entries),
                    next_token: None,
                })
            }
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_list_parents(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let child_id = match body.get("ChildId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'ChildId'"),
        };

        let state = state.read().await;
        match state.list_parents(child_id) {
            Ok(parents) => {
                let entries: Vec<wire::Parent> = parents
                    .iter()
                    .map(|p| wire::Parent {
                        id: Some(p.id.clone()),
                        r#type: Some(p.parent_type.clone()),
                    })
                    .collect();
                wire::serialize_list_parents_response(&wire::ListParentsResponse {
                    parents: Some(entries),
                    next_token: None,
                })
            }
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_move_account(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let account_id = match body.get("AccountId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'AccountId'"),
        };
        let source_parent_id = match body.get("SourceParentId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => {
                return json_error_response(400, "ValidationException", "Missing 'SourceParentId'");
            }
        };
        let destination_parent_id = match body.get("DestinationParentId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing 'DestinationParentId'",
                );
            }
        };

        let mut state = state.write().await;
        match state.move_account(account_id, source_parent_id, destination_parent_id) {
            Ok(()) => wire::serialize_move_account_response(),
            Err(e) => organizations_error_response(&e),
        }
    }

    // ==================== Tag handlers ====================

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let resource_id = match body.get("ResourceId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'ResourceId'"),
        };
        let tags: Vec<OrgTag> = match body.get("Tags").and_then(|v| v.as_array()) {
            Some(arr) => arr
                .iter()
                .filter_map(|t| {
                    let key = t.get("Key")?.as_str()?.to_string();
                    let value = t.get("Value")?.as_str()?.to_string();
                    Some(OrgTag { key, value })
                })
                .collect(),
            None => return json_error_response(400, "ValidationException", "Missing 'Tags'"),
        };

        let mut state = state.write().await;
        match state.tag_resource(resource_id, tags) {
            Ok(()) => wire::serialize_tag_resource_response(),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let resource_id = match body.get("ResourceId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'ResourceId'"),
        };
        let tag_keys: Vec<String> = match body.get("TagKeys").and_then(|v| v.as_array()) {
            Some(arr) => arr
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect(),
            None => return json_error_response(400, "ValidationException", "Missing 'TagKeys'"),
        };

        let mut state = state.write().await;
        match state.untag_resource(resource_id, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let resource_id = match body.get("ResourceId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return json_error_response(400, "ValidationException", "Missing 'ResourceId'"),
        };

        let state = state.read().await;
        match state.list_tags_for_resource(resource_id) {
            Ok(tags) => {
                let entries: Vec<wire::Tag> = tags
                    .iter()
                    .map(|t| wire::Tag {
                        key: t.key.clone(),
                        value: t.value.clone(),
                    })
                    .collect();
                wire::serialize_list_tags_for_resource_response(
                    &wire::ListTagsForResourceResponse {
                        tags: Some(entries),
                        next_token: None,
                    },
                )
            }
            Err(e) => organizations_error_response(&e),
        }
    }
    // ==================== Handshake handlers ====================

    async fn handle_invite_account_to_organization(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
        account_id: &str,
    ) -> MockResponse {
        let target_id = match body
            .get("Target")
            .and_then(|t| t.get("Id"))
            .and_then(|v| v.as_str())
        {
            Some(id) => id.to_string(),
            None => {
                return json_error_response(400, "ValidationException", "Missing 'Target.Id'");
            }
        };

        let mut state = state.write().await;
        let org_id = match &state.organization {
            Some(o) => o.id.clone(),
            None => {
                return json_error_response(
                    400,
                    "AWSOrganizationsNotInUseException",
                    "Your account is not a member of an organization.",
                );
            }
        };

        match state.invite_account_to_organization(&target_id, account_id, &org_id) {
            Ok(hs) => wire::serialize_invite_account_to_organization_response(
                &wire::InviteAccountToOrganizationResponse {
                    handshake: Some(handshake_wire(hs)),
                },
            ),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_accept_handshake(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let handshake_id = match body.get("HandshakeId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => {
                return json_error_response(400, "ValidationException", "Missing 'HandshakeId'");
            }
        };

        let mut state = state.write().await;
        match state.accept_handshake(handshake_id) {
            Ok(hs) => wire::serialize_accept_handshake_response(&wire::AcceptHandshakeResponse {
                handshake: Some(handshake_wire(hs)),
            }),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_cancel_handshake(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let handshake_id = match body.get("HandshakeId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => {
                return json_error_response(400, "ValidationException", "Missing 'HandshakeId'");
            }
        };

        let mut state = state.write().await;
        match state.cancel_handshake(handshake_id) {
            Ok(hs) => wire::serialize_cancel_handshake_response(&wire::CancelHandshakeResponse {
                handshake: Some(handshake_wire(hs)),
            }),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_decline_handshake(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let handshake_id = match body.get("HandshakeId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => {
                return json_error_response(400, "ValidationException", "Missing 'HandshakeId'");
            }
        };

        let mut state = state.write().await;
        match state.decline_handshake(handshake_id) {
            Ok(hs) => wire::serialize_decline_handshake_response(&wire::DeclineHandshakeResponse {
                handshake: Some(handshake_wire(hs)),
            }),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_describe_handshake(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let handshake_id = match body.get("HandshakeId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => {
                return json_error_response(400, "ValidationException", "Missing 'HandshakeId'");
            }
        };

        let state = state.read().await;
        match state.describe_handshake(handshake_id) {
            Ok(hs) => {
                wire::serialize_describe_handshake_response(&wire::DescribeHandshakeResponse {
                    handshake: Some(handshake_wire(hs)),
                })
            }
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_list_handshakes_for_account(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let handshakes: Vec<wire::Handshake> = state
            .list_handshakes_for_account()
            .iter()
            .map(|hs| handshake_wire(hs))
            .collect();
        wire::serialize_list_handshakes_for_account_response(
            &wire::ListHandshakesForAccountResponse {
                handshakes: Some(handshakes),
                next_token: None,
            },
        )
    }

    async fn handle_list_handshakes_for_organization(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let handshakes: Vec<wire::Handshake> = state
            .list_handshakes_for_organization()
            .iter()
            .map(|hs| handshake_wire(hs))
            .collect();
        wire::serialize_list_handshakes_for_organization_response(
            &wire::ListHandshakesForOrganizationResponse {
                handshakes: Some(handshakes),
                next_token: None,
            },
        )
    }

    async fn handle_enable_all_features(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        account_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.enable_all_features(account_id) {
            Ok(hs) => {
                wire::serialize_enable_all_features_response(&wire::EnableAllFeaturesResponse {
                    handshake: Some(handshake_wire(hs)),
                })
            }
            Err(e) => organizations_error_response(&e),
        }
    }

    // ==================== GovCloud handler ====================

    async fn handle_create_gov_cloud_account(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
        account_id: &str,
    ) -> MockResponse {
        let name = match body.get("AccountName").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return json_error_response(400, "ValidationException", "Missing 'AccountName'");
            }
        };
        let email = match body.get("Email").and_then(|v| v.as_str()) {
            Some(e) => e,
            None => return json_error_response(400, "ValidationException", "Missing 'Email'"),
        };

        let mut state = state.write().await;
        match state.create_gov_cloud_account(name, email, account_id) {
            Ok(account) => {
                let mut status = account_to_create_status_wire(account);
                // Set GovCloudAccountId to a distinct value
                status.gov_cloud_account_id = Some(format!("gov-{}", account.id));
                wire::serialize_create_gov_cloud_account_response(
                    &wire::CreateGovCloudAccountResponse {
                        create_account_status: Some(status),
                    },
                )
            }
            Err(e) => organizations_error_response(&e),
        }
    }

    // ==================== Resource policy handlers ====================

    async fn handle_put_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
        account_id: &str,
    ) -> MockResponse {
        let content = match body.get("Content").and_then(|v| v.as_str()) {
            Some(c) => c,
            None => return json_error_response(400, "ValidationException", "Missing 'Content'"),
        };

        let mut state = state.write().await;
        match state.put_resource_policy(content) {
            Ok(()) => {
                let org_id = state
                    .organization
                    .as_ref()
                    .map(|o| o.id.clone())
                    .unwrap_or_default();
                let rp_id = "rp-00000001".to_string();
                let rp_arn =
                    format!("arn:aws:organizations::{account_id}:resourcepolicy/{org_id}/{rp_id}");
                wire::serialize_put_resource_policy_response(&wire::PutResourcePolicyResponse {
                    resource_policy: Some(wire::ResourcePolicy {
                        content: Some(content.to_string()),
                        resource_policy_summary: Some(wire::ResourcePolicySummary {
                            id: Some(rp_id),
                            arn: Some(rp_arn),
                        }),
                    }),
                })
            }
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_describe_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        account_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_resource_policy() {
            Ok(content) => {
                let org_id = state
                    .organization
                    .as_ref()
                    .map(|o| o.id.clone())
                    .unwrap_or_default();
                let rp_id = "rp-00000001".to_string();
                let rp_arn =
                    format!("arn:aws:organizations::{account_id}:resourcepolicy/{org_id}/{rp_id}");
                wire::serialize_describe_resource_policy_response(
                    &wire::DescribeResourcePolicyResponse {
                        resource_policy: Some(wire::ResourcePolicy {
                            content: Some(content.clone()),
                            resource_policy_summary: Some(wire::ResourcePolicySummary {
                                id: Some(rp_id),
                                arn: Some(rp_arn),
                            }),
                        }),
                    },
                )
            }
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_delete_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_resource_policy() {
            Ok(()) => wire::serialize_delete_resource_policy_response(),
            Err(e) => organizations_error_response(&e),
        }
    }

    // ==================== Effective policy handler ====================

    // STUB[no-engine]: Effective policy evaluation requires aggregating SCPs across the entire
    //   org hierarchy; always returns an empty policy document.
    async fn handle_describe_effective_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
        account_id: &str,
    ) -> MockResponse {
        let policy_type = match body.get("PolicyType").and_then(|v| v.as_str()) {
            Some(pt) => pt,
            None => return json_error_response(400, "ValidationException", "Missing 'PolicyType'"),
        };
        let target_id = body
            .get("TargetId")
            .and_then(|v| v.as_str())
            .unwrap_or(account_id);

        let state = state.read().await;
        if state.organization.is_none() {
            return json_error_response(
                400,
                "AWSOrganizationsNotInUseException",
                "Your account is not a member of an organization.",
            );
        }

        let now = chrono::Utc::now().timestamp() as f64;
        wire::serialize_describe_effective_policy_response(&wire::DescribeEffectivePolicyResponse {
            effective_policy: Some(wire::EffectivePolicy {
                policy_type: Some(policy_type.to_string()),
                target_id: Some(target_id.to_string()),
                policy_content: Some("{}".to_string()),
                last_updated_timestamp: Some(now),
            }),
        })
    }

    // ==================== LeaveOrganization handler ====================

    async fn handle_leave_organization(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        if state.organization.is_none() {
            return json_error_response(
                400,
                "AWSOrganizationsNotInUseException",
                "Your account is not a member of an organization.",
            );
        }
        wire::serialize_leave_organization_response()
    }

    // ==================== Empty-list handlers ====================

    // STUB[no-engine]: Determining which accounts have invalid effective policies requires
    //   full SCP evaluation across the org hierarchy; always returns empty list.
    async fn handle_list_accounts_with_invalid_effective_policy(
        &self,
        body: &Value,
    ) -> MockResponse {
        let policy_type = body
            .get("PolicyType")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        wire::serialize_list_accounts_with_invalid_effective_policy_response(
            &wire::ListAccountsWithInvalidEffectivePolicyResponse {
                accounts: Some(vec![]),
                next_token: None,
                policy_type,
            },
        )
    }

    // STUB[no-engine]: Policy validation error detection requires full SCP evaluation;
    //   always returns empty list.
    async fn handle_list_effective_policy_validation_errors(&self, body: &Value) -> MockResponse {
        let account_id = body
            .get("AccountId")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let policy_type = body
            .get("PolicyType")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        wire::serialize_list_effective_policy_validation_errors_response(
            &wire::ListEffectivePolicyValidationErrorsResponse {
                account_id,
                policy_type,
                effective_policy_validation_errors: Some(vec![]),
                evaluation_timestamp: Some(chrono::Utc::now().timestamp() as f64),
                next_token: None,
                path: None,
            },
        )
    }

    // ==================== Responsibility transfer handlers ====================

    async fn handle_invite_organization_to_transfer_responsibility(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
        account_id: &str,
    ) -> MockResponse {
        let target_id = match body
            .get("Target")
            .and_then(|t| t.get("Id"))
            .and_then(|v| v.as_str())
        {
            Some(id) => id.to_string(),
            None => {
                return json_error_response(400, "ValidationException", "Missing 'Target.Id'");
            }
        };
        let transfer_type = body
            .get("Type")
            .and_then(|v| v.as_str())
            .unwrap_or("BILLING")
            .to_string();
        let name = body
            .get("Name")
            .and_then(|v| v.as_str())
            .unwrap_or("transfer")
            .to_string();

        let mut state = state.write().await;
        let org_id = match &state.organization {
            Some(o) => o.id.clone(),
            None => {
                return json_error_response(
                    400,
                    "AWSOrganizationsNotInUseException",
                    "Your account is not a member of an organization.",
                );
            }
        };

        match state.invite_organization_to_transfer_responsibility(
            &target_id,
            &transfer_type,
            &name,
            account_id,
            &org_id,
        ) {
            Ok(hs) => wire::serialize_invite_organization_to_transfer_responsibility_response(
                &wire::InviteOrganizationToTransferResponsibilityResponse {
                    handshake: Some(handshake_wire(hs)),
                },
            ),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_describe_responsibility_transfer(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let transfer_id = match body
            .get("ResponsibilityTransferId")
            .and_then(|v| v.as_str())
        {
            Some(id) => id,
            None => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing 'ResponsibilityTransferId'",
                );
            }
        };

        let state = state.read().await;
        match state.describe_responsibility_transfer(transfer_id) {
            Ok(t) => wire::serialize_describe_responsibility_transfer_response(
                &wire::DescribeResponsibilityTransferResponse {
                    responsibility_transfer: Some(responsibility_transfer_wire(t)),
                },
            ),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_list_inbound_responsibility_transfers(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        account_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let transfers: Vec<wire::ResponsibilityTransfer> = state
            .list_inbound_responsibility_transfers(account_id)
            .iter()
            .map(|t| responsibility_transfer_wire(t))
            .collect();
        wire::serialize_list_inbound_responsibility_transfers_response(
            &wire::ListInboundResponsibilityTransfersResponse {
                responsibility_transfers: Some(transfers),
                next_token: None,
            },
        )
    }

    async fn handle_list_outbound_responsibility_transfers(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        account_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let transfers: Vec<wire::ResponsibilityTransfer> = state
            .list_outbound_responsibility_transfers(account_id)
            .iter()
            .map(|t| responsibility_transfer_wire(t))
            .collect();
        wire::serialize_list_outbound_responsibility_transfers_response(
            &wire::ListOutboundResponsibilityTransfersResponse {
                responsibility_transfers: Some(transfers),
                next_token: None,
            },
        )
    }

    async fn handle_terminate_responsibility_transfer(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let transfer_id = match body
            .get("ResponsibilityTransferId")
            .and_then(|v| v.as_str())
        {
            Some(id) => id,
            None => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing 'ResponsibilityTransferId'",
                );
            }
        };

        let mut state = state.write().await;
        match state.terminate_responsibility_transfer(transfer_id) {
            Ok(t) => wire::serialize_terminate_responsibility_transfer_response(
                &wire::TerminateResponsibilityTransferResponse {
                    responsibility_transfer: Some(responsibility_transfer_wire(t)),
                },
            ),
            Err(e) => organizations_error_response(&e),
        }
    }

    async fn handle_update_responsibility_transfer(
        &self,
        state: &Arc<tokio::sync::RwLock<OrganizationsState>>,
        body: &Value,
    ) -> MockResponse {
        let transfer_id = match body
            .get("ResponsibilityTransferId")
            .and_then(|v| v.as_str())
        {
            Some(id) => id,
            None => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing 'ResponsibilityTransferId'",
                );
            }
        };
        let name = body.get("Name").and_then(|v| v.as_str());

        let mut state = state.write().await;
        match state.update_responsibility_transfer(transfer_id, name) {
            Ok(t) => wire::serialize_update_responsibility_transfer_response(
                &wire::UpdateResponsibilityTransferResponse {
                    responsibility_transfer: Some(responsibility_transfer_wire(t)),
                },
            ),
            Err(e) => organizations_error_response(&e),
        }
    }
}

fn organization_wire(org: &crate::types::Organization) -> wire::Organization {
    wire::Organization {
        id: Some(org.id.clone()),
        arn: Some(org.arn.clone()),
        master_account_id: Some(org.master_account_id.clone()),
        master_account_email: Some(org.master_account_email.clone()),
        feature_set: Some("ALL".to_string()),
        ..Default::default()
    }
}

fn account_wire(account: &crate::types::Account) -> wire::Account {
    wire::Account {
        id: Some(account.id.clone()),
        arn: Some(account.arn.clone()),
        name: Some(account.name.clone()),
        email: Some(account.email.clone()),
        status: Some(account.status.clone()),
        joined_method: Some(account.joined_method.clone()),
        joined_timestamp: Some(account.joined_timestamp.timestamp() as f64),
        ..Default::default()
    }
}

fn account_to_create_status_wire(account: &crate::types::Account) -> wire::CreateAccountStatus {
    wire::CreateAccountStatus {
        id: Some(account.create_account_status_id.clone()),
        account_name: Some(account.name.clone()),
        state: Some("SUCCEEDED".to_string()),
        account_id: Some(account.id.clone()),
        requested_timestamp: Some(account.joined_timestamp.timestamp() as f64),
        completed_timestamp: Some(account.joined_timestamp.timestamp() as f64),
        ..Default::default()
    }
}

fn delegated_admin_wire(
    admin: &crate::types::DelegatedAdministrator,
) -> wire::DelegatedAdministrator {
    wire::DelegatedAdministrator {
        id: Some(admin.account.id.clone()),
        arn: Some(admin.account.arn.clone()),
        name: Some(admin.account.name.clone()),
        email: Some(admin.account.email.clone()),
        status: Some(admin.account.status.clone()),
        joined_method: Some(admin.account.joined_method.clone()),
        joined_timestamp: Some(admin.account.joined_timestamp.timestamp() as f64),
        delegation_enabled_date: Some(admin.delegation_enabled_date.timestamp() as f64),
        state: None,
    }
}

fn ou_wire(ou: &crate::types::OrganizationalUnit) -> wire::OrganizationalUnit {
    wire::OrganizationalUnit {
        id: Some(ou.id.clone()),
        arn: Some(ou.arn.clone()),
        name: Some(ou.name.clone()),
    }
}

fn root_wire(root: &crate::types::OrgRoot) -> wire::Root {
    wire::Root {
        id: Some(root.id.clone()),
        arn: Some(root.arn.clone()),
        name: Some(root.name.clone()),
        policy_types: Some(
            root.policy_types
                .iter()
                .map(|pt| wire::PolicyTypeSummary {
                    r#type: Some(pt.policy_type.clone()),
                    status: Some(pt.status.clone()),
                })
                .collect(),
        ),
    }
}

fn policy_wire(policy: &crate::types::OrgPolicy) -> wire::Policy {
    wire::Policy {
        content: Some(policy.content.clone()),
        policy_summary: Some(policy_summary_wire(policy)),
    }
}

fn policy_summary_wire(policy: &crate::types::OrgPolicy) -> wire::PolicySummary {
    wire::PolicySummary {
        id: Some(policy.id.clone()),
        arn: Some(policy.arn.clone()),
        name: Some(policy.name.clone()),
        description: Some(policy.description.clone()),
        r#type: Some(policy.policy_type.clone()),
        aws_managed: Some(policy.aws_managed),
    }
}

fn handshake_wire(hs: &crate::types::Handshake) -> wire::Handshake {
    wire::Handshake {
        id: Some(hs.id.clone()),
        arn: Some(hs.arn.clone()),
        state: Some(hs.state.clone()),
        action: Some(hs.action.clone()),
        parties: Some(
            hs.parties
                .iter()
                .map(|p| wire::HandshakeParty {
                    id: p.id.clone(),
                    r#type: p.party_type.clone(),
                })
                .collect(),
        ),
        expiration_timestamp: Some(hs.expiration_timestamp.timestamp() as f64),
        requested_timestamp: Some(hs.requested_timestamp.timestamp() as f64),
        resources: Some(
            hs.resources
                .iter()
                .map(|r| wire::HandshakeResource {
                    r#type: Some(r.resource_type.clone()),
                    value: Some(r.value.clone()),
                    resources: None,
                })
                .collect(),
        ),
    }
}

fn responsibility_transfer_wire(
    t: &crate::types::ResponsibilityTransfer,
) -> wire::ResponsibilityTransfer {
    wire::ResponsibilityTransfer {
        id: Some(t.id.clone()),
        arn: Some(t.arn.clone()),
        status: Some(t.status.clone()),
        name: Some(t.name.clone()),
        r#type: Some(t.transfer_type.clone()),
        source: Some(wire::TransferParticipant {
            management_account_id: Some(t.source_account_id.clone()),
            management_account_email: None,
        }),
        target: Some(wire::TransferParticipant {
            management_account_id: Some(t.target_account_id.clone()),
            management_account_email: None,
        }),
        start_timestamp: Some(t.start_timestamp.timestamp() as f64),
        end_timestamp: None,
        active_handshake_id: t.active_handshake_id.clone(),
    }
}

fn organizations_error_response(err: &OrganizationsError) -> MockResponse {
    let (status, error_type) = match err {
        OrganizationsError::AwsOrganizationsNotInUse => (400, "AWSOrganizationsNotInUseException"),
        OrganizationsError::AlreadyInOrganization => (400, "AlreadyInOrganizationException"),
        OrganizationsError::OrganizationNotEmpty => (400, "OrganizationNotEmptyException"),
        OrganizationsError::AccountNotFound => (400, "AccountNotFoundException"),
        OrganizationsError::AccountAlreadyClosed => (400, "AccountAlreadyClosedException"),
        OrganizationsError::ConstraintViolationRemoveMaster => {
            (400, "ConstraintViolationException")
        }
        OrganizationsError::ConstraintViolationRegisterMaster => {
            (400, "ConstraintViolationException")
        }
        OrganizationsError::CreateAccountStatusNotFound(_) => {
            (400, "CreateAccountStatusNotFoundException")
        }
        OrganizationsError::InvalidInputUnrecognizedServicePrincipal => {
            (400, "InvalidInputException")
        }
        OrganizationsError::InvalidInputInvalidPolicyType => (400, "InvalidInputException"),
        OrganizationsError::InvalidInputInvalidChildType => (400, "InvalidInputException"),
        OrganizationsError::AccountAlreadyRegistered => (400, "AccountAlreadyRegisteredException"),
        OrganizationsError::AccountNotRegistered => (400, "AccountNotRegisteredException"),
        OrganizationsError::OrganizationalUnitNotFound => {
            (400, "OrganizationalUnitNotFoundException")
        }
        OrganizationsError::OrganizationalUnitNotEmpty => {
            (400, "OrganizationalUnitNotEmptyException")
        }
        OrganizationsError::PolicyNotFound => (400, "PolicyNotFoundException"),
        OrganizationsError::PolicyInUse => (400, "PolicyInUseException"),
        OrganizationsError::DuplicatePolicyAttachment => {
            (400, "DuplicatePolicyAttachmentException")
        }
        OrganizationsError::PolicyNotAttached => (400, "PolicyNotAttachedException"),
        OrganizationsError::PolicyTypeNotEnabledForOperation => {
            (400, "PolicyTypeNotEnabledException")
        }
        OrganizationsError::PolicyTypeNotEnabled => (400, "PolicyTypeNotEnabledException"),
        OrganizationsError::RootNotFound => (400, "RootNotFoundException"),
        OrganizationsError::PolicyTypeAlreadyEnabled => (400, "PolicyTypeAlreadyEnabledException"),
        OrganizationsError::ParentNotFound => (400, "ParentNotFoundException"),
        OrganizationsError::TargetNotFound => (400, "TargetNotFoundException"),
        OrganizationsError::ChildNotFound => (400, "ChildNotFoundException"),
        OrganizationsError::SourceParentNotFound => (400, "SourceParentNotFoundException"),
        OrganizationsError::ResourcePolicyNotFound => (400, "ResourcePolicyNotFoundException"),
        OrganizationsError::HandshakeNotFound => (400, "HandshakeNotFoundException"),
        OrganizationsError::InvalidHandshakeTransitionAccept => {
            (400, "InvalidHandshakeTransitionException")
        }
        OrganizationsError::InvalidHandshakeTransitionCancel => {
            (400, "InvalidHandshakeTransitionException")
        }
        OrganizationsError::InvalidHandshakeTransitionDecline => {
            (400, "InvalidHandshakeTransitionException")
        }
        OrganizationsError::ResponsibilityTransferNotFound => {
            (400, "ResponsibilityTransferNotFoundException")
        }
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
