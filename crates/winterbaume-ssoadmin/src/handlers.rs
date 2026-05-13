//! HTTP dispatch handler for SSO Admin (awsJson1_1 protocol).

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::Value;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id, json_error_response,
};

use crate::state::{AssignmentStatus, SsoAdminError, SsoAdminState};
use crate::types::{IDENTITY_STORE_ID, INSTANCE_ARN};
use crate::views::SsoAdminStateView;
use crate::wire;
use crate::wire::{
    AccountAssignment, AccountAssignmentForPrincipal, AccountAssignmentOperationStatus,
    AttachCustomerManagedPolicyReferenceToPermissionSetResponse,
    AttachManagedPolicyToPermissionSetResponse, AttachedManagedPolicy,
    CreateAccountAssignmentResponse, CreatePermissionSetResponse, CustomerManagedPolicyReference,
    DeleteAccountAssignmentResponse, DeleteInlinePolicyFromPermissionSetResponse,
    DeletePermissionSetResponse, DescribeAccountAssignmentCreationStatusResponse,
    DescribeAccountAssignmentDeletionStatusResponse, DescribePermissionSetResponse,
    DetachCustomerManagedPolicyReferenceFromPermissionSetResponse,
    DetachManagedPolicyFromPermissionSetResponse, GetInlinePolicyForPermissionSetResponse,
    InstanceMetadata, ListAccountAssignmentsForPrincipalResponse, ListAccountAssignmentsResponse,
    ListAccountsForProvisionedPermissionSetResponse,
    ListCustomerManagedPolicyReferencesInPermissionSetResponse, ListInstancesResponse,
    ListManagedPoliciesInPermissionSetResponse, ListPermissionSetsProvisionedToAccountResponse,
    ListPermissionSetsResponse, ListTagsForResourceResponse, PermissionSet,
    PermissionSetProvisioningStatus, ProvisionPermissionSetResponse,
    PutInlinePolicyToPermissionSetResponse, Tag, TagResourceResponse, UntagResourceResponse,
    UpdateInstanceResponse, UpdatePermissionSetResponse,
};

pub struct SsoAdminService {
    pub(crate) state: Arc<BackendState<SsoAdminState>>,
    pub(crate) notifier: StateChangeNotifier<SsoAdminStateView>,
}

impl SsoAdminService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for SsoAdminService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SsoAdminService {
    fn service_name(&self) -> &str {
        "sso"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://sso\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

async fn ssoadmin_error_response(e: &SsoAdminError) -> MockResponse {
    let (status, error_type) = match e {
        SsoAdminError::PermissionSetAlreadyExists(_) => (409, "ConflictException"),
        SsoAdminError::PermissionSetNotFound(_) => (404, "ResourceNotFoundException"),
        SsoAdminError::RequestNotFound(_) => (404, "ResourceNotFoundException"),
        SsoAdminError::ResourceNotFound(_) => (404, "ResourceNotFoundException"),
    };
    json_error_response(status, error_type, &e.to_string())
}

impl SsoAdminService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

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

        // Validate the body is well-formed JSON up-front; the typed deserialisers in
        // `wire` re-parse the bytes per operation.
        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let mutating = matches!(
            action.as_str(),
            "CreatePermissionSet"
                | "DeletePermissionSet"
                | "UpdatePermissionSet"
                | "CreateAccountAssignment"
                | "DeleteAccountAssignment"
                | "AttachManagedPolicyToPermissionSet"
                | "DetachManagedPolicyFromPermissionSet"
                | "AttachCustomerManagedPolicyReferenceToPermissionSet"
                | "DetachCustomerManagedPolicyReferenceFromPermissionSet"
                | "PutInlinePolicyToPermissionSet"
                | "DeleteInlinePolicyFromPermissionSet"
                | "TagResource"
                | "UntagResource"
                | "UpdateInstance"
        );

        let response = match action.as_str() {
            "ListInstances" => self.handle_list_instances().await,
            "CreatePermissionSet" => {
                self.handle_create_permission_set(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribePermissionSet" => {
                self.handle_describe_permission_set(&state, body_bytes)
                    .await
            }
            "ListPermissionSets" => self.handle_list_permission_sets(&state).await,
            "DeletePermissionSet" => self.handle_delete_permission_set(&state, body_bytes).await,
            "UpdatePermissionSet" => self.handle_update_permission_set(&state, body_bytes).await,
            "CreateAccountAssignment" => {
                self.handle_create_account_assignment(&state, body_bytes)
                    .await
            }
            "DescribeAccountAssignmentCreationStatus" => {
                self.handle_describe_account_assignment_creation_status(&state, body_bytes)
                    .await
            }
            "DeleteAccountAssignment" => {
                self.handle_delete_account_assignment(&state, body_bytes)
                    .await
            }
            "DescribeAccountAssignmentDeletionStatus" => {
                self.handle_describe_account_assignment_deletion_status(&state, body_bytes)
                    .await
            }
            "ListAccountAssignments" => {
                self.handle_list_account_assignments(&state, body_bytes)
                    .await
            }
            "ListAccountsForProvisionedPermissionSet" => {
                self.handle_list_accounts_for_provisioned_permission_set(&state, body_bytes)
                    .await
            }
            "ListPermissionSetsProvisionedToAccount" => {
                self.handle_list_permission_sets_provisioned_to_account(&state, body_bytes)
                    .await
            }
            "AttachManagedPolicyToPermissionSet" => {
                self.handle_attach_managed_policy(&state, body_bytes).await
            }
            "DetachManagedPolicyFromPermissionSet" => {
                self.handle_detach_managed_policy(&state, body_bytes).await
            }
            "ListManagedPoliciesInPermissionSet" => {
                self.handle_list_managed_policies(&state, body_bytes).await
            }
            "AttachCustomerManagedPolicyReferenceToPermissionSet" => {
                self.handle_attach_customer_managed_policy(&state, body_bytes)
                    .await
            }
            "DetachCustomerManagedPolicyReferenceFromPermissionSet" => {
                self.handle_detach_customer_managed_policy(&state, body_bytes)
                    .await
            }
            "ListCustomerManagedPolicyReferencesInPermissionSet" => {
                self.handle_list_customer_managed_policies(&state, body_bytes)
                    .await
            }
            "PutInlinePolicyToPermissionSet" => {
                self.handle_put_inline_policy(&state, body_bytes).await
            }
            "GetInlinePolicyForPermissionSet" => {
                self.handle_get_inline_policy(&state, body_bytes).await
            }
            "DeleteInlinePolicyFromPermissionSet" => {
                self.handle_delete_inline_policy(&state, body_bytes).await
            }
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ProvisionPermissionSet" => {
                self.handle_provision_permission_set(&state, body_bytes)
                    .await
            }
            "ListAccountAssignmentsForPrincipal" => {
                self.handle_list_account_assignments_for_principal(&state, body_bytes)
                    .await
            }
            "UpdateInstance" => self.handle_update_instance(body_bytes).await,
            // --- Unimplemented operations (auto-generated stubs) ---
            "AddRegion" => json_error_response(
                501,
                "NotImplementedError",
                "AddRegion is not yet implemented in winterbaume-ssoadmin",
            ),
            "CreateApplication" => json_error_response(
                501,
                "NotImplementedError",
                "CreateApplication is not yet implemented in winterbaume-ssoadmin",
            ),
            "CreateApplicationAssignment" => json_error_response(
                501,
                "NotImplementedError",
                "CreateApplicationAssignment is not yet implemented in winterbaume-ssoadmin",
            ),
            "CreateInstance" => json_error_response(
                501,
                "NotImplementedError",
                "CreateInstance is not yet implemented in winterbaume-ssoadmin",
            ),
            "CreateInstanceAccessControlAttributeConfiguration" => json_error_response(
                501,
                "NotImplementedError",
                "CreateInstanceAccessControlAttributeConfiguration is not yet implemented in winterbaume-ssoadmin",
            ),
            "CreateTrustedTokenIssuer" => json_error_response(
                501,
                "NotImplementedError",
                "CreateTrustedTokenIssuer is not yet implemented in winterbaume-ssoadmin",
            ),
            "DeleteApplication" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteApplication is not yet implemented in winterbaume-ssoadmin",
            ),
            "DeleteApplicationAccessScope" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteApplicationAccessScope is not yet implemented in winterbaume-ssoadmin",
            ),
            "DeleteApplicationAssignment" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteApplicationAssignment is not yet implemented in winterbaume-ssoadmin",
            ),
            "DeleteApplicationAuthenticationMethod" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteApplicationAuthenticationMethod is not yet implemented in winterbaume-ssoadmin",
            ),
            "DeleteApplicationGrant" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteApplicationGrant is not yet implemented in winterbaume-ssoadmin",
            ),
            "DeleteInstance" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteInstance is not yet implemented in winterbaume-ssoadmin",
            ),
            "DeleteInstanceAccessControlAttributeConfiguration" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteInstanceAccessControlAttributeConfiguration is not yet implemented in winterbaume-ssoadmin",
            ),
            "DeletePermissionsBoundaryFromPermissionSet" => json_error_response(
                501,
                "NotImplementedError",
                "DeletePermissionsBoundaryFromPermissionSet is not yet implemented in winterbaume-ssoadmin",
            ),
            "DeleteTrustedTokenIssuer" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteTrustedTokenIssuer is not yet implemented in winterbaume-ssoadmin",
            ),
            "DescribeApplication" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeApplication is not yet implemented in winterbaume-ssoadmin",
            ),
            "DescribeApplicationAssignment" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeApplicationAssignment is not yet implemented in winterbaume-ssoadmin",
            ),
            "DescribeApplicationProvider" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeApplicationProvider is not yet implemented in winterbaume-ssoadmin",
            ),
            "DescribeInstance" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeInstance is not yet implemented in winterbaume-ssoadmin",
            ),
            "DescribeInstanceAccessControlAttributeConfiguration" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeInstanceAccessControlAttributeConfiguration is not yet implemented in winterbaume-ssoadmin",
            ),
            "DescribePermissionSetProvisioningStatus" => json_error_response(
                501,
                "NotImplementedError",
                "DescribePermissionSetProvisioningStatus is not yet implemented in winterbaume-ssoadmin",
            ),
            "DescribeRegion" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeRegion is not yet implemented in winterbaume-ssoadmin",
            ),
            "DescribeTrustedTokenIssuer" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeTrustedTokenIssuer is not yet implemented in winterbaume-ssoadmin",
            ),
            "GetApplicationAccessScope" => json_error_response(
                501,
                "NotImplementedError",
                "GetApplicationAccessScope is not yet implemented in winterbaume-ssoadmin",
            ),
            "GetApplicationAssignmentConfiguration" => json_error_response(
                501,
                "NotImplementedError",
                "GetApplicationAssignmentConfiguration is not yet implemented in winterbaume-ssoadmin",
            ),
            "GetApplicationAuthenticationMethod" => json_error_response(
                501,
                "NotImplementedError",
                "GetApplicationAuthenticationMethod is not yet implemented in winterbaume-ssoadmin",
            ),
            "GetApplicationGrant" => json_error_response(
                501,
                "NotImplementedError",
                "GetApplicationGrant is not yet implemented in winterbaume-ssoadmin",
            ),
            "GetApplicationSessionConfiguration" => json_error_response(
                501,
                "NotImplementedError",
                "GetApplicationSessionConfiguration is not yet implemented in winterbaume-ssoadmin",
            ),
            "GetPermissionsBoundaryForPermissionSet" => json_error_response(
                501,
                "NotImplementedError",
                "GetPermissionsBoundaryForPermissionSet is not yet implemented in winterbaume-ssoadmin",
            ),
            "ListAccountAssignmentCreationStatus" => json_error_response(
                501,
                "NotImplementedError",
                "ListAccountAssignmentCreationStatus is not yet implemented in winterbaume-ssoadmin",
            ),
            "ListAccountAssignmentDeletionStatus" => json_error_response(
                501,
                "NotImplementedError",
                "ListAccountAssignmentDeletionStatus is not yet implemented in winterbaume-ssoadmin",
            ),
            "ListApplicationAccessScopes" => json_error_response(
                501,
                "NotImplementedError",
                "ListApplicationAccessScopes is not yet implemented in winterbaume-ssoadmin",
            ),
            "ListApplicationAssignments" => json_error_response(
                501,
                "NotImplementedError",
                "ListApplicationAssignments is not yet implemented in winterbaume-ssoadmin",
            ),
            "ListApplicationAssignmentsForPrincipal" => json_error_response(
                501,
                "NotImplementedError",
                "ListApplicationAssignmentsForPrincipal is not yet implemented in winterbaume-ssoadmin",
            ),
            "ListApplicationAuthenticationMethods" => json_error_response(
                501,
                "NotImplementedError",
                "ListApplicationAuthenticationMethods is not yet implemented in winterbaume-ssoadmin",
            ),
            "ListApplicationGrants" => json_error_response(
                501,
                "NotImplementedError",
                "ListApplicationGrants is not yet implemented in winterbaume-ssoadmin",
            ),
            "ListApplicationProviders" => json_error_response(
                501,
                "NotImplementedError",
                "ListApplicationProviders is not yet implemented in winterbaume-ssoadmin",
            ),
            "ListApplications" => json_error_response(
                501,
                "NotImplementedError",
                "ListApplications is not yet implemented in winterbaume-ssoadmin",
            ),
            "ListPermissionSetProvisioningStatus" => json_error_response(
                501,
                "NotImplementedError",
                "ListPermissionSetProvisioningStatus is not yet implemented in winterbaume-ssoadmin",
            ),
            "ListRegions" => json_error_response(
                501,
                "NotImplementedError",
                "ListRegions is not yet implemented in winterbaume-ssoadmin",
            ),
            "ListTrustedTokenIssuers" => json_error_response(
                501,
                "NotImplementedError",
                "ListTrustedTokenIssuers is not yet implemented in winterbaume-ssoadmin",
            ),
            "PutApplicationAccessScope" => json_error_response(
                501,
                "NotImplementedError",
                "PutApplicationAccessScope is not yet implemented in winterbaume-ssoadmin",
            ),
            "PutApplicationAssignmentConfiguration" => json_error_response(
                501,
                "NotImplementedError",
                "PutApplicationAssignmentConfiguration is not yet implemented in winterbaume-ssoadmin",
            ),
            "PutApplicationAuthenticationMethod" => json_error_response(
                501,
                "NotImplementedError",
                "PutApplicationAuthenticationMethod is not yet implemented in winterbaume-ssoadmin",
            ),
            "PutApplicationGrant" => json_error_response(
                501,
                "NotImplementedError",
                "PutApplicationGrant is not yet implemented in winterbaume-ssoadmin",
            ),
            "PutApplicationSessionConfiguration" => json_error_response(
                501,
                "NotImplementedError",
                "PutApplicationSessionConfiguration is not yet implemented in winterbaume-ssoadmin",
            ),
            "PutPermissionsBoundaryToPermissionSet" => json_error_response(
                501,
                "NotImplementedError",
                "PutPermissionsBoundaryToPermissionSet is not yet implemented in winterbaume-ssoadmin",
            ),
            "RemoveRegion" => json_error_response(
                501,
                "NotImplementedError",
                "RemoveRegion is not yet implemented in winterbaume-ssoadmin",
            ),
            "UpdateApplication" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateApplication is not yet implemented in winterbaume-ssoadmin",
            ),
            "UpdateInstanceAccessControlAttributeConfiguration" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateInstanceAccessControlAttributeConfiguration is not yet implemented in winterbaume-ssoadmin",
            ),
            "UpdateTrustedTokenIssuer" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateTrustedTokenIssuer is not yet implemented in winterbaume-ssoadmin",
            ),
            _ => json_error_response(
                501,
                "NotImplementedError",
                &format!("{action} is not yet implemented in winterbaume-ssoadmin"),
            ),
        };

        if mutating && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }

        response
    }

    async fn handle_list_instances(&self) -> MockResponse {
        wire::serialize_list_instances_response(&ListInstancesResponse {
            instances: Some(vec![InstanceMetadata {
                instance_arn: Some(INSTANCE_ARN.to_string()),
                identity_store_id: Some(IDENTITY_STORE_ID.to_string()),
                ..Default::default()
            }]),
            ..Default::default()
        })
    }

    async fn handle_create_permission_set(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_permission_set_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        let description = input.description;
        let session_duration = input.session_duration;
        let relay_state = input.relay_state;
        let tags = wire_tags_to_map(input.tags.as_deref().unwrap_or(&[]));

        let mut st = state.write().await;
        match st.create_permission_set(
            &input.name,
            description,
            session_duration,
            relay_state,
            tags,
            account_id,
            region,
        ) {
            Ok(ps) => {
                wire::serialize_create_permission_set_response(&CreatePermissionSetResponse {
                    permission_set: Some(permission_set_to_wire(ps)),
                    ..Default::default()
                })
            }
            Err(e) => ssoadmin_error_response(&e).await,
        }
    }

    async fn handle_describe_permission_set(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_permission_set_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.permission_set_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PermissionSetArn'");
        }
        let st = state.read().await;
        match st.describe_permission_set(&input.permission_set_arn) {
            Ok(ps) => {
                wire::serialize_describe_permission_set_response(&DescribePermissionSetResponse {
                    permission_set: Some(permission_set_to_wire(ps)),
                    ..Default::default()
                })
            }
            Err(e) => ssoadmin_error_response(&e).await,
        }
    }

    async fn handle_list_permission_sets(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
    ) -> MockResponse {
        let st = state.read().await;
        let arns: Vec<String> = st
            .list_permission_sets()
            .iter()
            .map(|ps| ps.permission_set_arn.clone())
            .collect();
        wire::serialize_list_permission_sets_response(&ListPermissionSetsResponse {
            permission_sets: Some(arns),
            ..Default::default()
        })
    }

    async fn handle_delete_permission_set(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_permission_set_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.permission_set_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PermissionSetArn'");
        }
        let mut st = state.write().await;
        match st.delete_permission_set(&input.permission_set_arn) {
            Ok(()) => {
                wire::serialize_delete_permission_set_response(&DeletePermissionSetResponse {})
            }
            Err(e) => ssoadmin_error_response(&e).await,
        }
    }

    async fn handle_update_permission_set(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_permission_set_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.permission_set_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PermissionSetArn'");
        }

        let mut st = state.write().await;
        match st.update_permission_set(
            &input.permission_set_arn,
            input.description,
            input.session_duration,
            input.relay_state,
        ) {
            Ok(()) => {
                wire::serialize_update_permission_set_response(&UpdatePermissionSetResponse {})
            }
            Err(e) => ssoadmin_error_response(&e).await,
        }
    }

    async fn handle_create_account_assignment(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_account_assignment_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.permission_set_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PermissionSetArn'");
        }
        if input.principal_type.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PrincipalType'");
        }
        if input.principal_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PrincipalId'");
        }
        if input.target_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'TargetId'");
        }
        let target_type = if input.target_type.is_empty() {
            "AWS_ACCOUNT"
        } else {
            input.target_type.as_str()
        };

        let mut st = state.write().await;
        match st.create_account_assignment(
            &input.permission_set_arn,
            &input.principal_type,
            &input.principal_id,
            &input.target_id,
            target_type,
        ) {
            Ok(status) => wire::serialize_create_account_assignment_response(
                &CreateAccountAssignmentResponse {
                    account_assignment_creation_status: Some(assignment_status_to_wire(&status)),
                    ..Default::default()
                },
            ),
            Err(e) => ssoadmin_error_response(&e).await,
        }
    }

    async fn handle_describe_account_assignment_creation_status(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input =
            match wire::deserialize_describe_account_assignment_creation_status_request(body) {
                Ok(v) => v,
                Err(e) => return json_error_response(400, "ValidationException", &e),
            };
        if input.account_assignment_creation_request_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'AccountAssignmentCreationRequestId'",
            );
        }
        let st = state.read().await;
        match st.describe_account_assignment_creation_status(
            &input.account_assignment_creation_request_id,
        ) {
            Ok(status) => wire::serialize_describe_account_assignment_creation_status_response(
                &DescribeAccountAssignmentCreationStatusResponse {
                    account_assignment_creation_status: Some(assignment_status_to_wire(status)),
                    ..Default::default()
                },
            ),
            Err(e) => ssoadmin_error_response(&e).await,
        }
    }

    async fn handle_delete_account_assignment(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_account_assignment_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.permission_set_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PermissionSetArn'");
        }
        if input.principal_type.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PrincipalType'");
        }
        if input.principal_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PrincipalId'");
        }
        if input.target_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'TargetId'");
        }
        let target_type = if input.target_type.is_empty() {
            "AWS_ACCOUNT"
        } else {
            input.target_type.as_str()
        };

        let mut st = state.write().await;
        match st.delete_account_assignment(
            &input.permission_set_arn,
            &input.principal_type,
            &input.principal_id,
            &input.target_id,
            target_type,
        ) {
            Ok(status) => wire::serialize_delete_account_assignment_response(
                &DeleteAccountAssignmentResponse {
                    account_assignment_deletion_status: Some(assignment_status_to_wire(&status)),
                    ..Default::default()
                },
            ),
            Err(e) => ssoadmin_error_response(&e).await,
        }
    }

    async fn handle_describe_account_assignment_deletion_status(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input =
            match wire::deserialize_describe_account_assignment_deletion_status_request(body) {
                Ok(v) => v,
                Err(e) => return json_error_response(400, "ValidationException", &e),
            };
        if input.account_assignment_deletion_request_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'AccountAssignmentDeletionRequestId'",
            );
        }
        let st = state.read().await;
        match st.describe_account_assignment_deletion_status(
            &input.account_assignment_deletion_request_id,
        ) {
            Ok(status) => wire::serialize_describe_account_assignment_deletion_status_response(
                &DescribeAccountAssignmentDeletionStatusResponse {
                    account_assignment_deletion_status: Some(assignment_status_to_wire(status)),
                    ..Default::default()
                },
            ),
            Err(e) => ssoadmin_error_response(&e).await,
        }
    }

    async fn handle_list_account_assignments(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_account_assignments_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.account_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AccountId'");
        }
        if input.permission_set_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PermissionSetArn'");
        }
        let st = state.read().await;
        let assignments: Vec<AccountAssignment> = st
            .list_account_assignments(&input.account_id, &input.permission_set_arn)
            .iter()
            .map(|k| AccountAssignment {
                account_id: Some(k.account_id.clone()),
                permission_set_arn: Some(k.permission_set_arn.clone()),
                principal_type: Some(k.principal_type.clone()),
                principal_id: Some(k.principal_id.clone()),
            })
            .collect();
        wire::serialize_list_account_assignments_response(&ListAccountAssignmentsResponse {
            account_assignments: Some(assignments),
            ..Default::default()
        })
    }

    async fn handle_list_accounts_for_provisioned_permission_set(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input =
            match wire::deserialize_list_accounts_for_provisioned_permission_set_request(body) {
                Ok(v) => v,
                Err(e) => return json_error_response(400, "ValidationException", &e),
            };
        if input.permission_set_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PermissionSetArn'");
        }
        let st = state.read().await;
        let account_ids =
            st.list_accounts_for_provisioned_permission_set(&input.permission_set_arn);
        wire::serialize_list_accounts_for_provisioned_permission_set_response(
            &ListAccountsForProvisionedPermissionSetResponse {
                account_ids: Some(account_ids),
                ..Default::default()
            },
        )
    }

    async fn handle_list_permission_sets_provisioned_to_account(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input =
            match wire::deserialize_list_permission_sets_provisioned_to_account_request(body) {
                Ok(v) => v,
                Err(e) => return json_error_response(400, "ValidationException", &e),
            };
        if input.account_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AccountId'");
        }
        let st = state.read().await;
        let arns = st.list_permission_sets_provisioned_to_account(&input.account_id);
        wire::serialize_list_permission_sets_provisioned_to_account_response(
            &ListPermissionSetsProvisionedToAccountResponse {
                permission_sets: Some(arns),
                ..Default::default()
            },
        )
    }

    async fn handle_attach_managed_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_attach_managed_policy_to_permission_set_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.permission_set_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PermissionSetArn'");
        }
        if input.managed_policy_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ManagedPolicyArn'");
        }
        let mut st = state.write().await;
        match st.attach_managed_policy(&input.permission_set_arn, &input.managed_policy_arn) {
            Ok(()) => wire::serialize_attach_managed_policy_to_permission_set_response(
                &AttachManagedPolicyToPermissionSetResponse {},
            ),
            Err(e) => ssoadmin_error_response(&e).await,
        }
    }

    async fn handle_detach_managed_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_detach_managed_policy_from_permission_set_request(body)
        {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.permission_set_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PermissionSetArn'");
        }
        if input.managed_policy_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ManagedPolicyArn'");
        }
        let mut st = state.write().await;
        match st.detach_managed_policy(&input.permission_set_arn, &input.managed_policy_arn) {
            Ok(()) => wire::serialize_detach_managed_policy_from_permission_set_response(
                &DetachManagedPolicyFromPermissionSetResponse {},
            ),
            Err(e) => ssoadmin_error_response(&e).await,
        }
    }

    async fn handle_list_managed_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_managed_policies_in_permission_set_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.permission_set_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PermissionSetArn'");
        }
        let st = state.read().await;
        match st.list_managed_policies(&input.permission_set_arn) {
            Ok(policies) => {
                let wire_policies: Vec<AttachedManagedPolicy> = policies
                    .iter()
                    .map(|(arn, name)| AttachedManagedPolicy {
                        arn: Some(arn.clone()),
                        name: Some(name.clone()),
                    })
                    .collect();
                wire::serialize_list_managed_policies_in_permission_set_response(
                    &ListManagedPoliciesInPermissionSetResponse {
                        attached_managed_policies: Some(wire_policies),
                        ..Default::default()
                    },
                )
            }
            Err(e) => ssoadmin_error_response(&e).await,
        }
    }

    async fn handle_attach_customer_managed_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_attach_customer_managed_policy_reference_to_permission_set_request(
            body,
        ) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.permission_set_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PermissionSetArn'");
        }
        let cmp_ref = &input.customer_managed_policy_reference;
        if cmp_ref.name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'CustomerManagedPolicyReference.Name'",
            );
        }
        let policy_path = cmp_ref.path.clone();

        let mut st = state.write().await;
        match st.attach_customer_managed_policy(
            &input.permission_set_arn,
            &cmp_ref.name,
            policy_path,
        ) {
            Ok(()) => {
                wire::serialize_attach_customer_managed_policy_reference_to_permission_set_response(
                    &AttachCustomerManagedPolicyReferenceToPermissionSetResponse {},
                )
            }
            Err(e) => ssoadmin_error_response(&e).await,
        }
    }

    async fn handle_detach_customer_managed_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_detach_customer_managed_policy_reference_from_permission_set_request(
            body,
        ) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.permission_set_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PermissionSetArn'");
        }
        let cmp_ref = &input.customer_managed_policy_reference;
        if cmp_ref.name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'CustomerManagedPolicyReference.Name'",
            );
        }
        let policy_path = cmp_ref.path.as_deref();

        let mut st = state.write().await;
        match st.detach_customer_managed_policy(&input.permission_set_arn, &cmp_ref.name, policy_path) {
            Ok(()) => {
                wire::serialize_detach_customer_managed_policy_reference_from_permission_set_response(
                    &DetachCustomerManagedPolicyReferenceFromPermissionSetResponse {},
                )
            }
            Err(e) => ssoadmin_error_response(&e).await,
        }
    }

    async fn handle_list_customer_managed_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_customer_managed_policy_references_in_permission_set_request(
            body,
        ) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.permission_set_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PermissionSetArn'");
        }
        let st = state.read().await;
        match st.list_customer_managed_policies(&input.permission_set_arn) {
            Ok(policies) => {
                let wire_policies: Vec<CustomerManagedPolicyReference> = policies
                    .iter()
                    .map(|(name, path)| CustomerManagedPolicyReference {
                        name: name.clone(),
                        path: path.clone(),
                    })
                    .collect();
                wire::serialize_list_customer_managed_policy_references_in_permission_set_response(
                    &ListCustomerManagedPolicyReferencesInPermissionSetResponse {
                        customer_managed_policy_references: Some(wire_policies),
                        ..Default::default()
                    },
                )
            }
            Err(e) => ssoadmin_error_response(&e).await,
        }
    }

    async fn handle_put_inline_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_inline_policy_to_permission_set_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.permission_set_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PermissionSetArn'");
        }
        if input.inline_policy.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'InlinePolicy'");
        }
        let mut st = state.write().await;
        match st.put_inline_policy(&input.permission_set_arn, &input.inline_policy) {
            Ok(()) => wire::serialize_put_inline_policy_to_permission_set_response(
                &PutInlinePolicyToPermissionSetResponse {},
            ),
            Err(e) => ssoadmin_error_response(&e).await,
        }
    }

    async fn handle_get_inline_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_inline_policy_for_permission_set_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.permission_set_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PermissionSetArn'");
        }
        let st = state.read().await;
        match st.get_inline_policy(&input.permission_set_arn) {
            Ok(policy) => wire::serialize_get_inline_policy_for_permission_set_response(
                &GetInlinePolicyForPermissionSetResponse {
                    inline_policy: policy,
                    ..Default::default()
                },
            ),
            Err(e) => ssoadmin_error_response(&e).await,
        }
    }

    async fn handle_delete_inline_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_inline_policy_from_permission_set_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.permission_set_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PermissionSetArn'");
        }
        let mut st = state.write().await;
        match st.delete_inline_policy(&input.permission_set_arn) {
            Ok(()) => wire::serialize_delete_inline_policy_from_permission_set_response(
                &DeleteInlinePolicyFromPermissionSetResponse {},
            ),
            Err(e) => ssoadmin_error_response(&e).await,
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceArn'");
        }
        let st = state.read().await;
        match st.list_tags_for_resource(&input.resource_arn) {
            Ok(tags) => {
                let wire_tags: Vec<Tag> = tags
                    .iter()
                    .map(|(k, v)| Tag {
                        key: k.clone(),
                        value: v.clone(),
                    })
                    .collect();
                wire::serialize_list_tags_for_resource_response(&ListTagsForResourceResponse {
                    tags: Some(wire_tags),
                    ..Default::default()
                })
            }
            Err(e) => ssoadmin_error_response(&e).await,
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceArn'");
        }
        let tags_vec: Vec<(String, String)> =
            input.tags.into_iter().map(|t| (t.key, t.value)).collect();
        let mut st = state.write().await;
        match st.tag_resource(&input.resource_arn, tags_vec) {
            Ok(()) => wire::serialize_tag_resource_response(&TagResourceResponse {}),
            Err(e) => ssoadmin_error_response(&e).await,
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceArn'");
        }
        let tag_keys = input.tag_keys;
        let mut st = state.write().await;
        match st.untag_resource(&input.resource_arn, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&UntagResourceResponse {}),
            Err(e) => ssoadmin_error_response(&e).await,
        }
    }

    async fn handle_provision_permission_set(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_provision_permission_set_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.permission_set_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PermissionSetArn'");
        }
        let permission_set_arn = input.permission_set_arn;
        let st = state.read().await;
        match st.describe_permission_set(&permission_set_arn) {
            Ok(_) => {
                use chrono::Utc;
                use uuid::Uuid;
                let request_id = Uuid::new_v4().to_string();
                wire::serialize_provision_permission_set_response(&ProvisionPermissionSetResponse {
                    permission_set_provisioning_status: Some(PermissionSetProvisioningStatus {
                        status: Some("SUCCEEDED".to_string()),
                        request_id: Some(request_id),
                        permission_set_arn: Some(permission_set_arn),
                        created_date: Some(Utc::now().timestamp() as f64),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
            }
            Err(e) => ssoadmin_error_response(&e).await,
        }
    }

    async fn handle_list_account_assignments_for_principal(
        &self,
        state: &Arc<tokio::sync::RwLock<SsoAdminState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_account_assignments_for_principal_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.principal_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PrincipalId'");
        }
        if input.principal_type.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PrincipalType'");
        }
        let filter_account_id = input.filter.as_ref().and_then(|f| f.account_id.clone());
        let principal_id = input.principal_id;
        let principal_type = input.principal_type;
        let st = state.read().await;
        let assignments: Vec<AccountAssignmentForPrincipal> = st
            .account_assignments
            .iter()
            .filter(|k| {
                k.principal_id == principal_id
                    && k.principal_type == principal_type
                    && filter_account_id
                        .as_deref()
                        .is_none_or(|id| k.account_id == id)
            })
            .map(|k| AccountAssignmentForPrincipal {
                account_id: Some(k.account_id.clone()),
                permission_set_arn: Some(k.permission_set_arn.clone()),
                principal_id: Some(k.principal_id.clone()),
                principal_type: Some(k.principal_type.clone()),
            })
            .collect();
        wire::serialize_list_account_assignments_for_principal_response(
            &ListAccountAssignmentsForPrincipalResponse {
                account_assignments: Some(assignments),
                ..Default::default()
            },
        )
    }

    async fn handle_update_instance(&self, body: &[u8]) -> MockResponse {
        // Validate input shape; ignore the payload otherwise (no state mutation).
        if !body.is_empty() && wire::deserialize_update_instance_request(body).is_err() {
            return json_error_response(400, "ValidationException", "Invalid UpdateInstance input");
        }
        wire::serialize_update_instance_response(&UpdateInstanceResponse {})
    }
}

// ---- Helpers ----

fn permission_set_to_wire(ps: &crate::types::PermissionSetData) -> PermissionSet {
    PermissionSet {
        permission_set_arn: Some(ps.permission_set_arn.clone()),
        name: Some(ps.name.clone()),
        description: ps.description.clone(),
        session_duration: ps.session_duration.clone(),
        relay_state: ps.relay_state.clone(),
        created_date: Some(ps.created_date),
    }
}

fn assignment_status_to_wire(s: &AssignmentStatus) -> AccountAssignmentOperationStatus {
    AccountAssignmentOperationStatus {
        request_id: Some(s.request_id.clone()),
        status: Some(s.status.clone()),
        permission_set_arn: Some(s.permission_set_arn.clone()),
        principal_type: Some(s.principal_type.clone()),
        principal_id: Some(s.principal_id.clone()),
        target_id: Some(s.target_id.clone()),
        target_type: Some(s.target_type.clone()),
        created_date: Some(s.created_date),
        failure_reason: None,
    }
}

fn wire_tags_to_map(tags: &[Tag]) -> HashMap<String, String> {
    tags.iter()
        .map(|t| (t.key.clone(), t.value.clone()))
        .collect()
}
