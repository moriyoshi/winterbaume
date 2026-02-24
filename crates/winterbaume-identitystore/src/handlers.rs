use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{IdentityStoreError, IdentityStoreState};
use crate::types;
use crate::views::IdentityStoreStateView;
use crate::wire;

pub struct IdentityStoreService {
    pub(crate) state: Arc<BackendState<IdentityStoreState>>,
    pub(crate) notifier: StateChangeNotifier<IdentityStoreStateView>,
}

impl IdentityStoreService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for IdentityStoreService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for IdentityStoreService {
    fn service_name(&self) -> &str {
        "identitystore"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://identitystore\..*\.amazonaws\.com",
            r"https?://identitystore\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl IdentityStoreService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        // Extract action from X-Amz-Target header.
        // Format: "AWSIdentityStore.CreateUser"
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
            "CreateUser" => self.handle_create_user(&state, body_bytes).await,
            "DescribeUser" => self.handle_describe_user(&state, body_bytes).await,
            "DeleteUser" => self.handle_delete_user(&state, body_bytes).await,
            "ListUsers" => self.handle_list_users(&state, body_bytes).await,
            "CreateGroup" => self.handle_create_group(&state, body_bytes).await,
            "DescribeGroup" => self.handle_describe_group(&state, body_bytes).await,
            "DeleteGroup" => self.handle_delete_group(&state, body_bytes).await,
            "ListGroups" => self.handle_list_groups(&state, body_bytes).await,
            "GetGroupId" => self.handle_get_group_id(&state, body_bytes).await,
            "GetUserId" => self.handle_get_user_id(&state, body_bytes).await,
            "CreateGroupMembership" => {
                self.handle_create_group_membership(&state, body_bytes)
                    .await
            }
            "DeleteGroupMembership" => {
                self.handle_delete_group_membership(&state, body_bytes)
                    .await
            }
            "ListGroupMemberships" => self.handle_list_group_memberships(&state, body_bytes).await,
            "ListGroupMembershipsForMember" => {
                self.handle_list_group_memberships_for_member(&state, body_bytes)
                    .await
            }
            "DescribeGroupMembership" => {
                self.handle_describe_group_membership(&state, body_bytes)
                    .await
            }
            "UpdateGroup" => self.handle_update_group(&state, body_bytes).await,
            "UpdateUser" => self.handle_update_user(&state, body_bytes).await,
            // --- Unimplemented operations (auto-generated stubs) ---
            "GetGroupMembershipId" => json_error_response(
                501,
                "NotImplementedError",
                "GetGroupMembershipId is not yet implemented in winterbaume-identitystore",
            ),
            "IsMemberInGroups" => json_error_response(
                501,
                "NotImplementedError",
                "IsMemberInGroups is not yet implemented in winterbaume-identitystore",
            ),
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for IdentityStore"),
            ),
        };

        let is_mutating = !matches!(
            action.as_str(),
            "DescribeUser"
                | "ListUsers"
                | "DescribeGroup"
                | "ListGroups"
                | "GetGroupId"
                | "GetUserId"
                | "ListGroupMemberships"
                | "ListGroupMembershipsForMember"
                | "DescribeGroupMembership"
                | "GetGroupMembershipId"
                | "IsMemberInGroups"
        );
        if is_mutating && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_user(
        &self,
        state: &Arc<tokio::sync::RwLock<IdentityStoreState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_user_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_store_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'IdentityStoreId'",
            );
        }
        let body_value = serde_json::to_value(&input).unwrap_or(Value::Null);

        let mut state = state.write().await;
        match state.create_user(&input.identity_store_id, &body_value) {
            Ok(user) => wire::serialize_create_user_response(&wire::CreateUserResponse {
                identity_store_id: Some(user.identity_store_id.clone()),
                user_id: Some(user.user_id.clone()),
            }),
            Err(e) => identitystore_error_response(&e),
        }
    }

    async fn handle_describe_user(
        &self,
        state: &Arc<tokio::sync::RwLock<IdentityStoreState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_user_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_store_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'IdentityStoreId'",
            );
        }
        if input.user_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'UserId'",
            );
        }

        let state = state.read().await;
        match state.describe_user(&input.identity_store_id, &input.user_id) {
            Ok(user) => {
                let resp = user_to_describe_user_response(user);
                wire::serialize_describe_user_response(&resp)
            }
            Err(e) => identitystore_error_response(&e),
        }
    }

    async fn handle_delete_user(
        &self,
        state: &Arc<tokio::sync::RwLock<IdentityStoreState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_user_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_store_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'IdentityStoreId'",
            );
        }
        if input.user_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'UserId'",
            );
        }

        let mut state = state.write().await;
        match state.delete_user(&input.identity_store_id, &input.user_id) {
            Ok(()) => wire::serialize_delete_user_response(&wire::DeleteUserResponse {}),
            Err(e) => identitystore_error_response(&e),
        }
    }

    async fn handle_list_users(
        &self,
        state: &Arc<tokio::sync::RwLock<IdentityStoreState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_users_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_store_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'IdentityStoreId'",
            );
        }

        let max_results = input.max_results.map(|v| v as usize);
        let next_token = input.next_token.as_deref();

        let state = state.read().await;
        let (users, new_next_token) =
            state.list_users(&input.identity_store_id, max_results, next_token);

        let user_entries: Vec<wire::User> = users.iter().map(|u| user_to_wire_user(u)).collect();

        wire::serialize_list_users_response(&wire::ListUsersResponse {
            users: Some(user_entries),
            next_token: new_next_token,
        })
    }

    // --- Group handlers ---

    async fn handle_create_group(
        &self,
        state: &Arc<tokio::sync::RwLock<IdentityStoreState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_store_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'IdentityStoreId'",
            );
        }
        let body_value = serde_json::to_value(&input).unwrap_or(Value::Null);

        let mut state = state.write().await;
        match state.create_group(&input.identity_store_id, &body_value) {
            Ok(group) => wire::serialize_create_group_response(&wire::CreateGroupResponse {
                identity_store_id: Some(group.identity_store_id.clone()),
                group_id: Some(group.group_id.clone()),
            }),
            Err(e) => identitystore_error_response(&e),
        }
    }

    async fn handle_describe_group(
        &self,
        state: &Arc<tokio::sync::RwLock<IdentityStoreState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_store_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'IdentityStoreId'",
            );
        }
        if input.group_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'GroupId'",
            );
        }

        let state = state.read().await;
        match state.describe_group(&input.identity_store_id, &input.group_id) {
            Ok(group) => {
                let resp = group_to_describe_group_response(group);
                wire::serialize_describe_group_response(&resp)
            }
            Err(e) => identitystore_error_response(&e),
        }
    }

    async fn handle_delete_group(
        &self,
        state: &Arc<tokio::sync::RwLock<IdentityStoreState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_store_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'IdentityStoreId'",
            );
        }
        if input.group_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'GroupId'",
            );
        }

        let mut state = state.write().await;
        match state.delete_group(&input.identity_store_id, &input.group_id) {
            Ok(()) => wire::serialize_delete_group_response(&wire::DeleteGroupResponse {}),
            Err(e) => identitystore_error_response(&e),
        }
    }

    async fn handle_list_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<IdentityStoreState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_groups_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_store_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'IdentityStoreId'",
            );
        }

        let max_results = input.max_results.map(|v| v as usize);
        let next_token = input.next_token.as_deref();

        let state = state.read().await;
        let (groups, new_next_token) =
            state.list_groups(&input.identity_store_id, max_results, next_token);

        let group_entries: Vec<wire::Group> =
            groups.iter().map(|g| group_to_wire_group(g)).collect();

        wire::serialize_list_groups_response(&wire::ListGroupsResponse {
            groups: Some(group_entries),
            next_token: new_next_token,
        })
    }

    async fn handle_get_group_id(
        &self,
        state: &Arc<tokio::sync::RwLock<IdentityStoreState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_group_id_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_store_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'IdentityStoreId'",
            );
        }

        // AlternateIdentifier.UniqueAttribute.AttributeValue
        let display_name = input
            .alternate_identifier
            .unique_attribute
            .as_ref()
            .and_then(|ua| ua.attribute_value.as_str());

        let display_name = match display_name {
            Some(dn) if !dn.is_empty() => dn,
            _ => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing required AlternateIdentifier.UniqueAttribute.AttributeValue",
                );
            }
        };

        let state = state.read().await;
        match state.get_group_id(&input.identity_store_id, display_name) {
            Ok(group) => wire::serialize_get_group_id_response(&wire::GetGroupIdResponse {
                identity_store_id: Some(group.identity_store_id.clone()),
                group_id: Some(group.group_id.clone()),
            }),
            Err(e) => identitystore_error_response(&e),
        }
    }

    async fn handle_get_user_id(
        &self,
        state: &Arc<tokio::sync::RwLock<IdentityStoreState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_user_id_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_store_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'IdentityStoreId'",
            );
        }

        let user_name = input
            .alternate_identifier
            .unique_attribute
            .as_ref()
            .and_then(|ua| ua.attribute_value.as_str());

        let user_name = match user_name {
            Some(un) if !un.is_empty() => un,
            _ => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing required AlternateIdentifier.UniqueAttribute.AttributeValue",
                );
            }
        };

        let state = state.read().await;
        match state.get_user_id(&input.identity_store_id, user_name) {
            Ok(user) => wire::serialize_get_user_id_response(&wire::GetUserIdResponse {
                identity_store_id: Some(user.identity_store_id.clone()),
                user_id: Some(user.user_id.clone()),
            }),
            Err(e) => identitystore_error_response(&e),
        }
    }

    // --- Group membership handlers ---

    async fn handle_create_group_membership(
        &self,
        state: &Arc<tokio::sync::RwLock<IdentityStoreState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_group_membership_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_store_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'IdentityStoreId'",
            );
        }
        if input.group_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'GroupId'",
            );
        }

        let member_user_id = input.member_id.user_id.as_deref().filter(|s| !s.is_empty());

        let member_user_id = match member_user_id {
            Some(id) => id,
            None => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing required field 'MemberId.UserId'",
                );
            }
        };

        let mut state = state.write().await;
        match state.create_group_membership(
            &input.identity_store_id,
            &input.group_id,
            member_user_id,
        ) {
            Ok(membership) => wire::serialize_create_group_membership_response(
                &wire::CreateGroupMembershipResponse {
                    identity_store_id: Some(membership.identity_store_id.clone()),
                    membership_id: Some(membership.membership_id.clone()),
                },
            ),
            Err(e) => identitystore_error_response(&e),
        }
    }

    async fn handle_delete_group_membership(
        &self,
        state: &Arc<tokio::sync::RwLock<IdentityStoreState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_group_membership_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_store_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'IdentityStoreId'",
            );
        }
        if input.membership_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'MembershipId'",
            );
        }

        let mut state = state.write().await;
        match state.delete_group_membership(&input.identity_store_id, &input.membership_id) {
            Ok(()) => wire::serialize_delete_group_membership_response(
                &wire::DeleteGroupMembershipResponse {},
            ),
            Err(e) => identitystore_error_response(&e),
        }
    }

    async fn handle_list_group_memberships(
        &self,
        state: &Arc<tokio::sync::RwLock<IdentityStoreState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_group_memberships_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_store_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'IdentityStoreId'",
            );
        }
        if input.group_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'GroupId'",
            );
        }

        let max_results = input.max_results.map(|v| v as usize);
        let next_token = input.next_token.as_deref();

        let state = state.read().await;
        let (memberships, new_next_token) = state.list_group_memberships(
            &input.identity_store_id,
            &input.group_id,
            max_results,
            next_token,
        );

        let membership_entries: Vec<wire::GroupMembership> = memberships
            .iter()
            .map(|m| membership_to_wire_membership(m))
            .collect();

        wire::serialize_list_group_memberships_response(&wire::ListGroupMembershipsResponse {
            group_memberships: Some(membership_entries),
            next_token: new_next_token,
        })
    }

    async fn handle_list_group_memberships_for_member(
        &self,
        state: &Arc<tokio::sync::RwLock<IdentityStoreState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_group_memberships_for_member_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_store_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'IdentityStoreId'",
            );
        }

        let member_user_id = input.member_id.user_id.as_deref().filter(|s| !s.is_empty());

        let member_user_id = match member_user_id {
            Some(id) => id,
            None => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing required field 'MemberId.UserId'",
                );
            }
        };

        let max_results = input.max_results.map(|v| v as usize);
        let next_token = input.next_token.as_deref();

        let state = state.read().await;
        let (memberships, new_next_token) = state.list_group_memberships_for_member(
            &input.identity_store_id,
            member_user_id,
            max_results,
            next_token,
        );

        let membership_entries: Vec<wire::GroupMembership> = memberships
            .iter()
            .map(|m| membership_to_wire_membership(m))
            .collect();

        wire::serialize_list_group_memberships_for_member_response(
            &wire::ListGroupMembershipsForMemberResponse {
                group_memberships: Some(membership_entries),
                next_token: new_next_token,
            },
        )
    }

    async fn handle_describe_group_membership(
        &self,
        state: &Arc<tokio::sync::RwLock<IdentityStoreState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_group_membership_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_store_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'IdentityStoreId'",
            );
        }
        if input.membership_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'MembershipId'",
            );
        }

        let state = state.read().await;
        match state.describe_group_membership(&input.identity_store_id, &input.membership_id) {
            Ok(m) => wire::serialize_describe_group_membership_response(
                &wire::DescribeGroupMembershipResponse {
                    identity_store_id: Some(m.identity_store_id.clone()),
                    membership_id: Some(m.membership_id.clone()),
                    group_id: Some(m.group_id.clone()),
                    member_id: Some(wire::MemberId {
                        user_id: Some(m.member_user_id.clone()),
                    }),
                    ..Default::default()
                },
            ),
            Err(e) => identitystore_error_response(&e),
        }
    }

    async fn handle_update_group(
        &self,
        state: &Arc<tokio::sync::RwLock<IdentityStoreState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_store_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'IdentityStoreId'",
            );
        }
        if input.group_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'GroupId'",
            );
        }

        let operations = serde_json::to_value(&input.operations).unwrap_or(Value::Array(vec![]));

        let mut state = state.write().await;
        match state.update_group(&input.identity_store_id, &input.group_id, &operations) {
            Ok(()) => wire::serialize_update_group_response(&wire::UpdateGroupResponse {}),
            Err(e) => identitystore_error_response(&e),
        }
    }

    async fn handle_update_user(
        &self,
        state: &Arc<tokio::sync::RwLock<IdentityStoreState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_user_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_store_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'IdentityStoreId'",
            );
        }
        if input.user_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'UserId'",
            );
        }

        let operations = serde_json::to_value(&input.operations).unwrap_or(Value::Array(vec![]));

        let mut state = state.write().await;
        match state.update_user(&input.identity_store_id, &input.user_id, &operations) {
            Ok(()) => wire::serialize_update_user_response(&wire::UpdateUserResponse {}),
            Err(e) => identitystore_error_response(&e),
        }
    }
}

/// Convert a state `types::Group` to a wire `Group` (for list results).
fn group_to_wire_group(group: &types::Group) -> wire::Group {
    let resp = group_to_describe_group_response(group);
    wire::Group {
        identity_store_id: resp.identity_store_id,
        group_id: resp.group_id,
        display_name: resp.display_name,
        description: resp.description,
        external_ids: resp.external_ids,
        ..Default::default()
    }
}

/// Convert a state `types::Group` to a wire `DescribeGroupResponse`.
fn group_to_describe_group_response(group: &types::Group) -> wire::DescribeGroupResponse {
    wire::DescribeGroupResponse {
        identity_store_id: Some(group.identity_store_id.clone()),
        group_id: Some(group.group_id.clone()),
        display_name: group.display_name.clone(),
        description: group.description.clone(),
        external_ids: Some(
            group
                .external_ids
                .iter()
                .map(|e| wire::ExternalId {
                    issuer: e.issuer.clone(),
                    id: e.id.clone(),
                })
                .collect(),
        ),
        ..Default::default()
    }
}

/// Convert a state `types::GroupMembership` to a wire `GroupMembership`.
fn membership_to_wire_membership(m: &types::GroupMembership) -> wire::GroupMembership {
    wire::GroupMembership {
        identity_store_id: Some(m.identity_store_id.clone()),
        membership_id: Some(m.membership_id.clone()),
        group_id: Some(m.group_id.clone()),
        member_id: Some(wire::MemberId {
            user_id: Some(m.member_user_id.clone()),
        }),
        ..Default::default()
    }
}

/// Convert a state `types::User` to a wire `User` (for list results).
fn user_to_wire_user(user: &types::User) -> wire::User {
    let resp = user_to_describe_user_response(user);
    wire::User {
        identity_store_id: resp.identity_store_id,
        user_id: resp.user_id,
        user_name: resp.user_name,
        name: resp.name,
        display_name: resp.display_name,
        nick_name: resp.nick_name,
        profile_url: resp.profile_url,
        emails: resp.emails,
        addresses: resp.addresses,
        phone_numbers: resp.phone_numbers,
        user_type: resp.user_type,
        title: resp.title,
        preferred_language: resp.preferred_language,
        locale: resp.locale,
        timezone: resp.timezone,
        external_ids: resp.external_ids,
        photos: resp.photos,
        birthdate: resp.birthdate,
        roles: resp.roles,
        ..Default::default()
    }
}

/// Convert a state `types::User` to a wire `DescribeUserResponse`.
fn user_to_describe_user_response(user: &types::User) -> wire::DescribeUserResponse {
    wire::DescribeUserResponse {
        identity_store_id: Some(user.identity_store_id.clone()),
        user_id: Some(user.user_id.clone()),
        user_name: user.user_name.clone(),
        name: user.name.as_ref().map(|n| wire::Name {
            formatted: n.formatted.clone(),
            family_name: n.family_name.clone(),
            given_name: n.given_name.clone(),
            middle_name: n.middle_name.clone(),
            honorific_prefix: n.honorific_prefix.clone(),
            honorific_suffix: n.honorific_suffix.clone(),
        }),
        display_name: user.display_name.clone(),
        nick_name: user.nick_name.clone(),
        profile_url: user.profile_url.clone(),
        emails: user.emails.as_ref().map(|es| {
            es.iter()
                .map(|e| wire::Email {
                    value: e.value.clone(),
                    r#type: e.email_type.clone(),
                    primary: e.primary,
                })
                .collect()
        }),
        addresses: user.addresses.as_ref().map(|addrs| {
            addrs
                .iter()
                .map(|a| wire::Address {
                    street_address: a.street_address.clone(),
                    locality: a.locality.clone(),
                    region: a.region.clone(),
                    postal_code: a.postal_code.clone(),
                    country: a.country.clone(),
                    formatted: a.formatted.clone(),
                    r#type: a.address_type.clone(),
                    primary: a.primary,
                })
                .collect()
        }),
        phone_numbers: user.phone_numbers.as_ref().map(|pns| {
            pns.iter()
                .map(|p| wire::PhoneNumber {
                    value: p.value.clone(),
                    r#type: p.phone_type.clone(),
                    primary: p.primary,
                })
                .collect()
        }),
        user_type: user.user_type.clone(),
        title: user.title.clone(),
        preferred_language: user.preferred_language.clone(),
        locale: user.locale.clone(),
        timezone: user.timezone.clone(),
        external_ids: Some(
            user.external_ids
                .iter()
                .map(|e| wire::ExternalId {
                    issuer: e.issuer.clone(),
                    id: e.id.clone(),
                })
                .collect(),
        ),
        photos: user.photos.as_ref().map(|ps| {
            ps.iter()
                .map(|p| wire::Photo {
                    value: p.value.clone(),
                    r#type: p.photo_type.clone(),
                    display: p.display.clone(),
                    primary: p.primary,
                })
                .collect()
        }),
        birthdate: user.birthdate.clone(),
        roles: user.roles.as_ref().map(|rs| {
            rs.iter()
                .map(|r| wire::Role {
                    value: r.value.clone(),
                    r#type: r.role_type.clone(),
                    primary: r.primary,
                })
                .collect()
        }),
        ..Default::default()
    }
}

fn identitystore_error_response(err: &IdentityStoreError) -> MockResponse {
    let (status, error_type) = match err {
        IdentityStoreError::UserNotFound { .. } => (400u16, "ResourceNotFoundException"),
        IdentityStoreError::GroupNotFound { .. } => (400u16, "ResourceNotFoundException"),
        IdentityStoreError::MembershipNotFound { .. } => (400u16, "ResourceNotFoundException"),
        IdentityStoreError::GroupNotFoundByDisplayName { .. } => {
            (400u16, "ResourceNotFoundException")
        }
        IdentityStoreError::UserNotFoundByUserName { .. } => (400u16, "ResourceNotFoundException"),
        IdentityStoreError::MembershipAlreadyExists { .. } => (409u16, "ConflictException"),
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
