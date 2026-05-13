use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::model;
use crate::state::WorkSpacesState;
use crate::types::{
    ClientProperties, IpRule, RebootRequest, RebuildRequest, SelfservicePermissions, StartRequest,
    StopRequest, Tag, TerminateRequest, Workspace, WorkspaceCreationProperties, WorkspaceDirectory,
    WorkspaceProperties, WorkspaceRequest, WorkspacesPool,
};
use crate::views::WorkSpacesStateView;
use crate::wire;

/// Convert an RFC 3339 timestamp string into the wire's `f64` epoch seconds.
fn rfc3339_to_epoch(s: &str) -> Option<f64> {
    chrono::DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|dt| dt.timestamp() as f64)
}

/// WorkSpaces service handler that processes awsJson1.1 protocol requests.
pub struct WorkSpacesService {
    pub(crate) state: Arc<BackendState<WorkSpacesState>>,
    pub(crate) notifier: StateChangeNotifier<WorkSpacesStateView>,
}

impl WorkSpacesService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for WorkSpacesService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for WorkSpacesService {
    fn service_name(&self) -> &str {
        "workspaces"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://workspaces\..*\.amazonaws\.com",
            r"https?://workspaces\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl WorkSpacesService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        // Extract action from X-Amz-Target header
        // Format: "WorkspacesService.CreateWorkspaces"
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
        // `wire` re-parse the bytes per operation. An empty body is treated as `{}`.
        let body_bytes: &[u8] = if request.body.is_empty() {
            b"{}"
        } else {
            if serde_json::from_slice::<Value>(&request.body).is_err() {
                return json_error_response(400, "SerializationException", "Invalid JSON body");
            }
            &request.body
        };

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreateWorkspaces" => {
                self.handle_create_workspaces(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeWorkspaces" => self.handle_describe_workspaces(&state, body_bytes).await,
            "TerminateWorkspaces" => self.handle_terminate_workspaces(&state, body_bytes).await,
            "DescribeWorkspaceDirectories" => {
                self.handle_describe_workspace_directories(&state, body_bytes)
                    .await
            }
            "CreateTags" => self.handle_create_tags(&state, body_bytes).await,
            "DescribeTags" => self.handle_describe_tags(&state, body_bytes).await,
            "CreateWorkspaceImage" => {
                self.handle_create_workspace_image(&state, body_bytes, account_id)
                    .await
            }
            "DescribeWorkspaceImages" => {
                self.handle_describe_workspace_images(&state, body_bytes)
                    .await
            }
            "DescribeWorkspaceImagePermissions" => {
                self.handle_describe_workspace_image_permissions(&state, body_bytes)
                    .await
            }
            "UpdateWorkspaceImagePermission" => {
                self.handle_update_workspace_image_permission(&state, body_bytes)
                    .await
            }
            "DescribeClientProperties" => {
                self.handle_describe_client_properties(&state, body_bytes)
                    .await
            }
            "ModifyClientProperties" => {
                self.handle_modify_client_properties(&state, body_bytes)
                    .await
            }
            "ModifySelfservicePermissions" => {
                self.handle_modify_selfservice_permissions(&state, body_bytes)
                    .await
            }
            "ModifyWorkspaceCreationProperties" => {
                self.handle_modify_workspace_creation_properties(&state, body_bytes)
                    .await
            }
            "RegisterWorkspaceDirectory" => {
                self.handle_register_workspace_directory(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeregisterWorkspaceDirectory" => {
                self.handle_deregister_workspace_directory(&state, body_bytes)
                    .await
            }
            "DeleteTags" => self.handle_delete_tags(&state, body_bytes).await,
            "StartWorkspaces" => self.handle_start_workspaces(&state, body_bytes).await,
            "StopWorkspaces" => self.handle_stop_workspaces(&state, body_bytes).await,
            "RebootWorkspaces" => self.handle_reboot_workspaces(&state, body_bytes).await,
            "RebuildWorkspaces" => self.handle_rebuild_workspaces(&state, body_bytes).await,
            "RestoreWorkspace" => self.handle_restore_workspace(&state, body_bytes).await,
            "ModifyWorkspaceProperties" => {
                self.handle_modify_workspace_properties(&state, body_bytes)
                    .await
            }
            "ModifyWorkspaceState" => self.handle_modify_workspace_state(&state, body_bytes).await,
            "DescribeWorkspacesConnectionStatus" => {
                self.handle_describe_workspaces_connection_status(&state, body_bytes)
                    .await
            }
            "CreateIpGroup" => {
                self.handle_create_ip_group(&state, body_bytes, account_id)
                    .await
            }
            "DescribeIpGroups" => self.handle_describe_ip_groups(&state, body_bytes).await,
            "DeleteIpGroup" => self.handle_delete_ip_group(&state, body_bytes).await,
            "AuthorizeIpRules" => self.handle_authorize_ip_rules(&state, body_bytes).await,
            "RevokeIpRules" => self.handle_revoke_ip_rules(&state, body_bytes).await,
            "UpdateRulesOfIpGroup" => {
                self.handle_update_rules_of_ip_group(&state, body_bytes)
                    .await
            }
            "AssociateIpGroups" => self.handle_associate_ip_groups(&state, body_bytes).await,
            "DisassociateIpGroups" => self.handle_disassociate_ip_groups(&state, body_bytes).await,
            "CreateConnectionAlias" => {
                self.handle_create_connection_alias(&state, body_bytes, account_id)
                    .await
            }
            "DescribeConnectionAliases" => {
                self.handle_describe_connection_aliases(&state, body_bytes)
                    .await
            }
            "DeleteConnectionAlias" => {
                self.handle_delete_connection_alias(&state, body_bytes)
                    .await
            }
            "AssociateConnectionAlias" => {
                self.handle_associate_connection_alias(&state, body_bytes)
                    .await
            }
            "DisassociateConnectionAlias" => {
                self.handle_disassociate_connection_alias(&state, body_bytes)
                    .await
            }
            "DescribeConnectionAliasPermissions" => {
                self.handle_describe_connection_alias_permissions(&state, body_bytes)
                    .await
            }
            "UpdateConnectionAliasPermission" => {
                self.handle_update_connection_alias_permission(&state, body_bytes)
                    .await
            }
            "DescribeWorkspaceBundles" => {
                self.handle_describe_workspace_bundles(&state, body_bytes)
                    .await
            }
            "CreateWorkspaceBundle" => {
                self.handle_create_workspace_bundle(&state, body_bytes, account_id)
                    .await
            }
            "DeleteWorkspaceBundle" => {
                self.handle_delete_workspace_bundle(&state, body_bytes)
                    .await
            }
            "DeleteWorkspaceImage" => self.handle_delete_workspace_image(&state, body_bytes).await,
            "CreateWorkspacesPool" => {
                self.handle_create_workspaces_pool(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeWorkspacesPools" => {
                self.handle_describe_workspaces_pools(&state, body_bytes)
                    .await
            }
            "TerminateWorkspacesPool" => {
                self.handle_terminate_workspaces_pool(&state, body_bytes)
                    .await
            }
            "UpdateWorkspacesPool" => self.handle_update_workspaces_pool(&state, body_bytes).await,
            "StartWorkspacesPool" => self.handle_start_workspaces_pool(&state, body_bytes).await,
            "StopWorkspacesPool" => self.handle_stop_workspaces_pool(&state, body_bytes).await,
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for WorkSpaces"),
            ),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_workspaces(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_workspaces_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let requests: Vec<WorkspaceRequest> = input
            .workspaces
            .into_iter()
            .filter_map(|item| {
                if item.directory_id.is_empty()
                    || item.user_name.is_empty()
                    || item.bundle_id.is_empty()
                {
                    return None;
                }
                let workspace_properties =
                    item.workspace_properties.map(|props| WorkspaceProperties {
                        running_mode: props.running_mode,
                        running_mode_auto_stop_timeout_in_minutes: props
                            .running_mode_auto_stop_timeout_in_minutes,
                        root_volume_size_gib: props.root_volume_size_gib,
                        user_volume_size_gib: props.user_volume_size_gib,
                        compute_type_name: props.compute_type_name,
                    });
                let tags: Option<Vec<Tag>> = item.tags.map(|arr| {
                    arr.into_iter()
                        .map(|t| Tag {
                            key: t.key,
                            value: t.value.unwrap_or_default(),
                        })
                        .collect()
                });
                Some(WorkspaceRequest {
                    directory_id: item.directory_id,
                    user_name: item.user_name,
                    bundle_id: item.bundle_id,
                    volume_encryption_key: item.volume_encryption_key,
                    user_volume_encryption_enabled: item.user_volume_encryption_enabled,
                    root_volume_encryption_enabled: item.root_volume_encryption_enabled,
                    workspace_properties,
                    tags,
                })
            })
            .collect();

        let mut state = state.write().await;
        let (pending, failed) = state.create_workspaces(&requests, account_id, region);

        let pending_wire: Vec<model::Workspace> = pending.iter().map(workspace_to_model).collect();
        let failed_wire: Vec<model::FailedCreateWorkspaceRequest> = failed
            .iter()
            .map(|f| model::FailedCreateWorkspaceRequest {
                workspace_request: f
                    .workspace_request
                    .as_ref()
                    .map(|r| model::WorkspaceRequest {
                        directory_id: r.directory_id.clone(),
                        user_name: r.user_name.clone(),
                        bundle_id: r.bundle_id.clone(),
                        ..Default::default()
                    }),
                error_code: Some(f.error_code.clone()),
                error_message: Some(f.error_message.clone()),
            })
            .collect();

        let result = model::CreateWorkspacesResult {
            pending_requests: Some(pending_wire),
            failed_requests: Some(failed_wire),
        };
        wire::serialize_create_workspaces_response(&result)
    }

    async fn handle_describe_workspaces(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_workspaces_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let workspace_ids = input.workspace_ids;
        let directory_id = input.directory_id.as_deref();
        let user_name = input.user_name.as_deref();

        let state = state.read().await;
        let workspaces =
            state.describe_workspaces(workspace_ids.as_deref(), directory_id, user_name);

        let entries: Vec<model::Workspace> =
            workspaces.iter().map(|ws| workspace_to_model(ws)).collect();

        let result = model::DescribeWorkspacesResult {
            workspaces: Some(entries),
            next_token: None,
        };
        wire::serialize_describe_workspaces_response(&result)
    }

    async fn handle_terminate_workspaces(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_terminate_workspaces_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let requests: Vec<TerminateRequest> = input
            .terminate_workspace_requests
            .into_iter()
            .map(|item| TerminateRequest {
                workspace_id: item.workspace_id,
            })
            .collect();

        let mut state = state.write().await;
        let failed = state.terminate_workspaces(&requests);

        let failed_wire: Vec<model::FailedWorkspaceChangeRequest> = failed
            .iter()
            .map(|f| model::FailedWorkspaceChangeRequest {
                workspace_id: Some(f.workspace_id.clone()),
                error_code: Some(f.error_code.clone()),
                error_message: Some(f.error_message.clone()),
            })
            .collect();

        let result = model::TerminateWorkspacesResult {
            failed_requests: Some(failed_wire),
        };
        wire::serialize_terminate_workspaces_response(&result)
    }

    async fn handle_describe_workspace_directories(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_workspace_directories_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let directory_ids = input.directory_ids;

        let state = state.read().await;
        let directories = state.describe_workspace_directories(directory_ids.as_deref());

        let entries: Vec<model::WorkspaceDirectory> = directories
            .iter()
            .map(|dir| directory_to_model(dir))
            .collect();

        let result = model::DescribeWorkspaceDirectoriesResult {
            directories: Some(entries),
            next_token: None,
        };
        wire::serialize_describe_workspace_directories_response(&result)
    }

    async fn handle_create_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_tags_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceId' field");
        }
        let resource_id = input.resource_id;
        let tags: Vec<Tag> = input
            .tags
            .into_iter()
            .map(|t| Tag {
                key: t.key,
                value: t.value.unwrap_or_default(),
            })
            .collect();

        let mut state = state.write().await;
        state.create_tags(&resource_id, &tags);

        let result = model::CreateTagsResult {};
        wire::serialize_create_tags_response(&result)
    }

    async fn handle_describe_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_tags_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceId' field");
        }
        let resource_id = input.resource_id;

        let state = state.read().await;
        let tags = state.describe_tags(&resource_id);

        let tag_list: Vec<model::Tag> = tags
            .iter()
            .map(|t| model::Tag {
                key: t.key.clone(),
                value: Some(t.value.clone()),
            })
            .collect();

        let result = model::DescribeTagsResult {
            tag_list: Some(tag_list),
        };
        wire::serialize_describe_tags_response(&result)
    }

    async fn handle_create_workspace_image(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_workspace_image_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name' field");
        }
        let name = input.name;
        if input.description.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Description' field");
        }
        let description = input.description;
        if input.workspace_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WorkspaceId' field");
        }
        let workspace_id = input.workspace_id;

        let tags: Option<Vec<Tag>> = input.tags.map(|arr| {
            arr.into_iter()
                .map(|t| Tag {
                    key: t.key,
                    value: t.value.unwrap_or_default(),
                })
                .collect()
        });

        let mut state = state.write().await;
        match state.create_workspace_image(
            &name,
            &description,
            &workspace_id,
            account_id,
            tags.as_deref(),
        ) {
            Ok(image) => {
                let result = model::CreateWorkspaceImageResult {
                    image_id: Some(image.image_id),
                    name: Some(image.name),
                    description: Some(image.description),
                    state: Some(image.state),
                    operating_system: Some(model::OperatingSystem {
                        r#type: image.operating_system_type,
                    }),
                    owner_account_id: Some(image.owner_account_id),
                    required_tenancy: Some(image.required_tenancy),
                    created: rfc3339_to_epoch(&image.created),
                };
                wire::serialize_create_workspace_image_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_describe_workspace_images(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_workspace_images_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let image_ids = input.image_ids;

        let state = state.read().await;
        let images = state.describe_workspace_images(image_ids.as_deref());

        let entries: Vec<model::WorkspaceImage> = images
            .iter()
            .map(|img| model::WorkspaceImage {
                image_id: Some(img.image_id.clone()),
                name: Some(img.name.clone()),
                description: Some(img.description.clone()),
                state: Some(img.state.clone()),
                operating_system: Some(model::OperatingSystem {
                    r#type: img.operating_system_type.clone(),
                }),
                owner_account_id: Some(img.owner_account_id.clone()),
                required_tenancy: Some(img.required_tenancy.clone()),
                created: rfc3339_to_epoch(&img.created),
                ..Default::default()
            })
            .collect();

        let result = model::DescribeWorkspaceImagesResult {
            images: Some(entries),
            next_token: None,
        };
        wire::serialize_describe_workspace_images_response(&result)
    }

    async fn handle_describe_workspace_image_permissions(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_workspace_image_permissions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.image_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ImageId' field");
        }
        let image_id = input.image_id;

        let state = state.read().await;
        match state.describe_workspace_image_permissions(&image_id) {
            Ok(shared_ids) => {
                let perms: Vec<model::ImagePermission> = shared_ids
                    .iter()
                    .map(|id| model::ImagePermission {
                        shared_account_id: Some(id.clone()),
                    })
                    .collect();

                let result = model::DescribeWorkspaceImagePermissionsResult {
                    image_id: Some(image_id),
                    image_permissions: Some(perms),
                    next_token: None,
                };
                wire::serialize_describe_workspace_image_permissions_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_update_workspace_image_permission(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_workspace_image_permission_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.image_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ImageId' field");
        }
        let image_id = input.image_id;
        if input.shared_account_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'SharedAccountId' field",
            );
        }
        let shared_account_id = input.shared_account_id;
        let allow_copy_image = input.allow_copy_image;

        let mut state = state.write().await;
        match state.update_workspace_image_permission(
            &image_id,
            &shared_account_id,
            allow_copy_image,
        ) {
            Ok(()) => {
                let result = model::UpdateWorkspaceImagePermissionResult {};
                wire::serialize_update_workspace_image_permission_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_describe_client_properties(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_client_properties_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let resource_ids = input.resource_ids;

        let state = state.read().await;
        let props = state.describe_client_properties(&resource_ids);

        let entries: Vec<model::ClientPropertiesResult> = props
            .iter()
            .map(|(id, cp)| model::ClientPropertiesResult {
                resource_id: Some(id.clone()),
                client_properties: Some(model::ClientProperties {
                    reconnect_enabled: cp.reconnect_enabled.clone(),
                    log_upload_enabled: cp.log_upload_enabled.clone(),
                }),
            })
            .collect();

        let result = model::DescribeClientPropertiesResult {
            client_properties_list: Some(entries),
        };
        wire::serialize_describe_client_properties_response(&result)
    }

    async fn handle_modify_client_properties(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_modify_client_properties_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceId' field");
        }
        let resource_id = input.resource_id;

        let client_props = ClientProperties {
            reconnect_enabled: input.client_properties.reconnect_enabled,
            log_upload_enabled: input.client_properties.log_upload_enabled,
        };

        let mut state = state.write().await;
        state.modify_client_properties(&resource_id, &client_props);

        let result = model::ModifyClientPropertiesResult {};
        wire::serialize_modify_client_properties_response(&result)
    }

    async fn handle_modify_selfservice_permissions(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_modify_selfservice_permissions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceId' field");
        }
        let resource_id = input.resource_id;

        let sp = input.selfservice_permissions;
        let perms = SelfservicePermissions {
            restart_workspace: sp.restart_workspace,
            increase_volume_size: sp.increase_volume_size,
            change_compute_type: sp.change_compute_type,
            switch_running_mode: sp.switch_running_mode,
            rebuild_workspace: sp.rebuild_workspace,
        };

        let mut state = state.write().await;
        match state.modify_selfservice_permissions(&resource_id, &perms) {
            Ok(()) => {
                let result = model::ModifySelfservicePermissionsResult {};
                wire::serialize_modify_selfservice_permissions_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_modify_workspace_creation_properties(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_modify_workspace_creation_properties_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceId' field");
        }
        let resource_id = input.resource_id;

        let wcp = input.workspace_creation_properties;
        let props = WorkspaceCreationProperties {
            custom_security_group_id: wcp.custom_security_group_id,
            default_ou: wcp.default_ou,
            enable_internet_access: wcp.enable_internet_access,
            enable_maintenance_mode: wcp.enable_maintenance_mode,
            user_enabled_as_local_administrator: wcp.user_enabled_as_local_administrator,
            instance_iam_role_arn: wcp.instance_iam_role_arn,
        };

        let mut state = state.write().await;
        match state.modify_workspace_creation_properties(&resource_id, &props) {
            Ok(()) => {
                let result = model::ModifyWorkspaceCreationPropertiesResult {};
                wire::serialize_modify_workspace_creation_properties_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_register_workspace_directory(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_register_workspace_directory_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let directory_id = match input.directory_id {
            Some(id) if !id.is_empty() => id,
            _ => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing 'DirectoryId' field",
                );
            }
        };

        let mut state = state.write().await;
        match state.register_workspace_directory(&directory_id, account_id, region) {
            Ok(dir) => {
                let result = model::RegisterWorkspaceDirectoryResult {
                    directory_id: Some(dir.directory_id),
                    state: Some(dir.state),
                };
                wire::serialize_register_workspace_directory_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_deregister_workspace_directory(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_deregister_workspace_directory_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.directory_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DirectoryId' field");
        }
        let directory_id = input.directory_id;

        let mut state = state.write().await;
        match state.deregister_workspace_directory(&directory_id) {
            Ok(()) => {
                let result = model::DeregisterWorkspaceDirectoryResult {};
                wire::serialize_deregister_workspace_directory_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_delete_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_tags_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceId' field");
        }
        let resource_id = input.resource_id;
        let tag_keys = input.tag_keys;

        let mut state = state.write().await;
        state.delete_tags(&resource_id, &tag_keys);

        let result = model::DeleteTagsResult {};
        wire::serialize_delete_tags_response(&result)
    }

    async fn handle_start_workspaces(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_workspaces_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let requests: Vec<StartRequest> = input
            .start_workspace_requests
            .into_iter()
            .filter_map(|item| {
                item.workspace_id
                    .map(|workspace_id| StartRequest { workspace_id })
            })
            .collect();

        let mut state = state.write().await;
        let failed = state.start_workspaces(&requests);

        let failed_wire: Vec<model::FailedWorkspaceChangeRequest> = failed
            .iter()
            .map(|f| model::FailedWorkspaceChangeRequest {
                workspace_id: Some(f.workspace_id.clone()),
                error_code: Some(f.error_code.clone()),
                error_message: Some(f.error_message.clone()),
            })
            .collect();
        let result = model::StartWorkspacesResult {
            failed_requests: Some(failed_wire),
        };
        wire::serialize_start_workspaces_response(&result)
    }

    async fn handle_stop_workspaces(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_workspaces_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let requests: Vec<StopRequest> = input
            .stop_workspace_requests
            .into_iter()
            .filter_map(|item| {
                item.workspace_id
                    .map(|workspace_id| StopRequest { workspace_id })
            })
            .collect();

        let mut state = state.write().await;
        let failed = state.stop_workspaces(&requests);

        let failed_wire: Vec<model::FailedWorkspaceChangeRequest> = failed
            .iter()
            .map(|f| model::FailedWorkspaceChangeRequest {
                workspace_id: Some(f.workspace_id.clone()),
                error_code: Some(f.error_code.clone()),
                error_message: Some(f.error_message.clone()),
            })
            .collect();
        let result = model::StopWorkspacesResult {
            failed_requests: Some(failed_wire),
        };
        wire::serialize_stop_workspaces_response(&result)
    }

    async fn handle_reboot_workspaces(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_reboot_workspaces_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let requests: Vec<RebootRequest> = input
            .reboot_workspace_requests
            .into_iter()
            .map(|item| RebootRequest {
                workspace_id: item.workspace_id,
            })
            .collect();

        let mut state = state.write().await;
        let failed = state.reboot_workspaces(&requests);

        let failed_wire: Vec<model::FailedWorkspaceChangeRequest> = failed
            .iter()
            .map(|f| model::FailedWorkspaceChangeRequest {
                workspace_id: Some(f.workspace_id.clone()),
                error_code: Some(f.error_code.clone()),
                error_message: Some(f.error_message.clone()),
            })
            .collect();
        let result = model::RebootWorkspacesResult {
            failed_requests: Some(failed_wire),
        };
        wire::serialize_reboot_workspaces_response(&result)
    }

    async fn handle_rebuild_workspaces(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_rebuild_workspaces_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let requests: Vec<RebuildRequest> = input
            .rebuild_workspace_requests
            .into_iter()
            .map(|item| RebuildRequest {
                workspace_id: item.workspace_id,
            })
            .collect();

        let mut state = state.write().await;
        let failed = state.rebuild_workspaces(&requests);

        let failed_wire: Vec<model::FailedWorkspaceChangeRequest> = failed
            .iter()
            .map(|f| model::FailedWorkspaceChangeRequest {
                workspace_id: Some(f.workspace_id.clone()),
                error_code: Some(f.error_code.clone()),
                error_message: Some(f.error_message.clone()),
            })
            .collect();
        let result = model::RebuildWorkspacesResult {
            failed_requests: Some(failed_wire),
        };
        wire::serialize_rebuild_workspaces_response(&result)
    }

    async fn handle_restore_workspace(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_restore_workspace_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.workspace_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WorkspaceId' field");
        }
        let workspace_id = input.workspace_id;

        let mut state = state.write().await;
        match state.restore_workspace(&workspace_id) {
            Ok(()) => {
                let result = model::RestoreWorkspaceResult {};
                wire::serialize_restore_workspace_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_modify_workspace_properties(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_modify_workspace_properties_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.workspace_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WorkspaceId' field");
        }
        let workspace_id = input.workspace_id;

        let (
            running_mode,
            root_volume_size_gib,
            user_volume_size_gib,
            compute_type_name,
            running_mode_auto_stop,
        ) = match input.workspace_properties {
            Some(p) => (
                p.running_mode,
                p.root_volume_size_gib,
                p.user_volume_size_gib,
                p.compute_type_name,
                p.running_mode_auto_stop_timeout_in_minutes,
            ),
            None => (None, None, None, None, None),
        };

        let mut state = state.write().await;
        match state.modify_workspace_properties(
            &workspace_id,
            running_mode.as_deref(),
            root_volume_size_gib,
            user_volume_size_gib,
            compute_type_name.as_deref(),
            running_mode_auto_stop,
        ) {
            Ok(()) => {
                let result = model::ModifyWorkspacePropertiesResult {};
                wire::serialize_modify_workspace_properties_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_modify_workspace_state(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_modify_workspace_state_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.workspace_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'WorkspaceId' field");
        }
        let workspace_id = input.workspace_id;
        if input.workspace_state.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'WorkspaceState' field",
            );
        }
        let workspace_state = input.workspace_state;

        let mut state = state.write().await;
        match state.modify_workspace_state(&workspace_id, &workspace_state) {
            Ok(()) => {
                let result = model::ModifyWorkspaceStateResult {};
                wire::serialize_modify_workspace_state_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_describe_workspaces_connection_status(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_workspaces_connection_status_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let workspace_ids = input.workspace_ids;

        let state = state.read().await;
        let statuses = state.describe_workspaces_connection_status(workspace_ids.as_deref());

        let entries: Vec<model::WorkspaceConnectionStatus> = statuses
            .iter()
            .map(|(id, st)| model::WorkspaceConnectionStatus {
                workspace_id: Some(id.to_string()),
                connection_state: Some(if *st == "AVAILABLE" {
                    "CONNECTED".to_string()
                } else {
                    "DISCONNECTED".to_string()
                }),
                ..Default::default()
            })
            .collect();

        let result = model::DescribeWorkspacesConnectionStatusResult {
            workspaces_connection_status: Some(entries),
            next_token: None,
        };
        wire::serialize_describe_workspaces_connection_status_response(&result)
    }

    // --- IP Group handlers ---

    async fn handle_create_ip_group(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_ip_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'GroupName' field");
        }
        let group_name = input.group_name;
        let group_desc = input.group_desc;
        let user_rules: Vec<IpRule> = input
            .user_rules
            .unwrap_or_default()
            .into_iter()
            .filter_map(|item| {
                item.ip_rule.map(|ip_rule| IpRule {
                    ip_rule,
                    rule_desc: item.rule_desc,
                })
            })
            .collect();

        let mut state = state.write().await;
        let group =
            state.create_ip_group(&group_name, group_desc.as_deref(), user_rules, account_id);

        let result = model::CreateIpGroupResult {
            group_id: Some(group.group_id),
        };
        wire::serialize_create_ip_group_response(&result)
    }

    async fn handle_describe_ip_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_ip_groups_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let group_ids = input.group_ids;

        let state = state.read().await;
        let groups = state.describe_ip_groups(group_ids.as_deref());

        let entries: Vec<model::WorkspacesIpGroup> = groups
            .iter()
            .map(|g| model::WorkspacesIpGroup {
                group_id: Some(g.group_id.clone()),
                group_name: Some(g.group_name.clone()),
                group_desc: g.group_desc.clone(),
                user_rules: Some(
                    g.user_rules
                        .iter()
                        .map(|r| model::IpRuleItem {
                            ip_rule: Some(r.ip_rule.clone()),
                            rule_desc: r.rule_desc.clone(),
                        })
                        .collect(),
                ),
            })
            .collect();

        let result = model::DescribeIpGroupsResult {
            result: Some(entries),
            next_token: None,
        };
        wire::serialize_describe_ip_groups_response(&result)
    }

    async fn handle_delete_ip_group(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_ip_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.group_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'GroupId' field");
        }
        let group_id = input.group_id;

        let mut state = state.write().await;
        match state.delete_ip_group(&group_id) {
            Ok(()) => {
                let result = model::DeleteIpGroupResult {};
                wire::serialize_delete_ip_group_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_authorize_ip_rules(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_authorize_ip_rules_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.group_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'GroupId' field");
        }
        let group_id = input.group_id;
        let user_rules: Vec<IpRule> = input
            .user_rules
            .into_iter()
            .filter_map(|item| {
                item.ip_rule.map(|ip_rule| IpRule {
                    ip_rule,
                    rule_desc: item.rule_desc,
                })
            })
            .collect();

        let mut state = state.write().await;
        match state.authorize_ip_rules(&group_id, user_rules) {
            Ok(()) => {
                let result = model::AuthorizeIpRulesResult {};
                wire::serialize_authorize_ip_rules_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_revoke_ip_rules(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_revoke_ip_rules_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.group_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'GroupId' field");
        }
        let group_id = input.group_id;
        let user_rules = input.user_rules;

        let mut state = state.write().await;
        match state.revoke_ip_rules(&group_id, &user_rules) {
            Ok(()) => {
                let result = model::RevokeIpRulesResult {};
                wire::serialize_revoke_ip_rules_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_update_rules_of_ip_group(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_rules_of_ip_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.group_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'GroupId' field");
        }
        let group_id = input.group_id;
        let user_rules: Vec<IpRule> = input
            .user_rules
            .into_iter()
            .filter_map(|item| {
                item.ip_rule.map(|ip_rule| IpRule {
                    ip_rule,
                    rule_desc: item.rule_desc,
                })
            })
            .collect();

        let mut state = state.write().await;
        match state.update_rules_of_ip_group(&group_id, user_rules) {
            Ok(()) => {
                let result = model::UpdateRulesOfIpGroupResult {};
                wire::serialize_update_rules_of_ip_group_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_associate_ip_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_associate_ip_groups_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.directory_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DirectoryId' field");
        }
        let directory_id = input.directory_id;
        let group_ids = input.group_ids;

        let mut state = state.write().await;
        match state.associate_ip_groups(&directory_id, &group_ids) {
            Ok(()) => {
                let result = model::AssociateIpGroupsResult {};
                wire::serialize_associate_ip_groups_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_disassociate_ip_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_disassociate_ip_groups_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.directory_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DirectoryId' field");
        }
        let directory_id = input.directory_id;
        let group_ids = input.group_ids;

        let mut state = state.write().await;
        match state.disassociate_ip_groups(&directory_id, &group_ids) {
            Ok(()) => {
                let result = model::DisassociateIpGroupsResult {};
                wire::serialize_disassociate_ip_groups_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    // --- Connection Alias handlers ---

    async fn handle_create_connection_alias(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_connection_alias_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.connection_string.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'ConnectionString' field",
            );
        }
        let connection_string = input.connection_string;

        let mut state = state.write().await;
        let alias = state.create_connection_alias(&connection_string, account_id);

        let result = model::CreateConnectionAliasResult {
            alias_id: Some(alias.alias_id),
        };
        wire::serialize_create_connection_alias_response(&result)
    }

    async fn handle_describe_connection_aliases(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_connection_aliases_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let alias_ids = input.alias_ids;
        let resource_id = input.resource_id.as_deref();

        let state = state.read().await;
        let aliases = state.describe_connection_aliases(alias_ids.as_deref(), resource_id);

        let entries: Vec<model::ConnectionAlias> = aliases
            .iter()
            .map(|a| model::ConnectionAlias {
                alias_id: Some(a.alias_id.clone()),
                connection_string: Some(a.connection_string.clone()),
                owner_account_id: Some(a.owner_account_id.clone()),
                state: Some(a.state.clone()),
                associations: if a.associations.is_empty() {
                    None
                } else {
                    Some(
                        a.associations
                            .iter()
                            .map(|assoc| model::ConnectionAliasAssociation {
                                connection_identifier: Some(assoc.connection_identifier.clone()),
                                resource_id: assoc.resource_id.clone(),
                                associated_account_id: assoc.associated_account_id.clone(),
                                ..Default::default()
                            })
                            .collect(),
                    )
                },
            })
            .collect();

        let result = model::DescribeConnectionAliasesResult {
            connection_aliases: Some(entries),
            next_token: None,
        };
        wire::serialize_describe_connection_aliases_response(&result)
    }

    async fn handle_delete_connection_alias(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_connection_alias_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.alias_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AliasId' field");
        }
        let alias_id = input.alias_id;

        let mut state = state.write().await;
        match state.delete_connection_alias(&alias_id) {
            Ok(()) => {
                let result = model::DeleteConnectionAliasResult {};
                wire::serialize_delete_connection_alias_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_associate_connection_alias(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_associate_connection_alias_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.alias_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AliasId' field");
        }
        let alias_id = input.alias_id;
        if input.resource_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceId' field");
        }
        let resource_id = input.resource_id;

        let mut state = state.write().await;
        match state.associate_connection_alias(&alias_id, &resource_id) {
            Ok(connection_identifier) => {
                let result = model::AssociateConnectionAliasResult {
                    connection_identifier: Some(connection_identifier),
                };
                wire::serialize_associate_connection_alias_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_disassociate_connection_alias(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_disassociate_connection_alias_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.alias_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AliasId' field");
        }
        let alias_id = input.alias_id;

        let mut state = state.write().await;
        match state.disassociate_connection_alias(&alias_id) {
            Ok(()) => {
                let result = model::DisassociateConnectionAliasResult {};
                wire::serialize_disassociate_connection_alias_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_describe_connection_alias_permissions(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_connection_alias_permissions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.alias_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AliasId' field");
        }
        let alias_id = input.alias_id;

        let state = state.read().await;
        match state.describe_connection_alias_permissions(&alias_id) {
            Ok(perms) => {
                let entries: Vec<model::ConnectionAliasPermission> = perms
                    .iter()
                    .map(|p| model::ConnectionAliasPermission {
                        shared_account_id: p.shared_account_id.clone(),
                        allow_association: p.allow_association,
                    })
                    .collect();

                let result = model::DescribeConnectionAliasPermissionsResult {
                    alias_id: Some(alias_id),
                    connection_alias_permissions: Some(entries),
                    next_token: None,
                };
                wire::serialize_describe_connection_alias_permissions_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_update_connection_alias_permission(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_connection_alias_permission_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.alias_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AliasId' field");
        }
        let alias_id = input.alias_id;
        let perm = input.connection_alias_permission;
        if perm.shared_account_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'SharedAccountId' in ConnectionAliasPermission",
            );
        }
        let shared_account_id = perm.shared_account_id;
        let allow_association = perm.allow_association;

        let mut state = state.write().await;
        match state.update_connection_alias_permission(
            &alias_id,
            &shared_account_id,
            allow_association,
        ) {
            Ok(()) => {
                let result = model::UpdateConnectionAliasPermissionResult {};
                wire::serialize_update_connection_alias_permission_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    // --- Bundle handlers ---

    async fn handle_describe_workspace_bundles(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_workspace_bundles_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let bundle_ids = input.bundle_ids;
        let owner = input.owner.as_deref();

        let state = state.read().await;
        let bundles = state.describe_workspace_bundles(bundle_ids.as_deref(), owner);

        let entries: Vec<model::WorkspaceBundle> = bundles
            .iter()
            .map(|b| model::WorkspaceBundle {
                bundle_id: Some(b.bundle_id.clone()),
                name: Some(b.name.clone()),
                owner: b.owner.clone(),
                description: b.description.clone(),
                bundle_type: b.bundle_type.clone(),
                compute_type: b.compute_type_name.as_ref().map(|n| model::ComputeType {
                    name: Some(n.clone()),
                }),
                creation_time: rfc3339_to_epoch(&b.creation_time),
                root_storage: b.root_storage_capacity.map(|c| model::RootStorage {
                    capacity: c.to_string(),
                }),
                user_storage: b.user_storage_capacity.map(|c| model::UserStorage {
                    capacity: c.to_string(),
                }),
                ..Default::default()
            })
            .collect();

        let result = model::DescribeWorkspaceBundlesResult {
            bundles: Some(entries),
            next_token: None,
        };
        wire::serialize_describe_workspace_bundles_response(&result)
    }

    async fn handle_create_workspace_bundle(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_workspace_bundle_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.bundle_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'BundleName' field");
        }
        let bundle_name = input.bundle_name;
        let bundle_description = if input.bundle_description.is_empty() {
            None
        } else {
            Some(input.bundle_description)
        };
        if input.image_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ImageId' field");
        }
        let image_id = input.image_id;
        let compute_type = input
            .compute_type
            .name
            .unwrap_or_else(|| "VALUE".to_string());
        let user_storage = input.user_storage.capacity.parse::<i32>().unwrap_or(50);
        let root_storage = input
            .root_storage
            .and_then(|rs| rs.capacity.parse::<i32>().ok());

        let mut state = state.write().await;
        match state.create_workspace_bundle(
            &bundle_name,
            bundle_description.as_deref(),
            &image_id,
            &compute_type,
            user_storage,
            root_storage,
            account_id,
        ) {
            Ok(bundle) => {
                let wb = model::WorkspaceBundle {
                    bundle_id: Some(bundle.bundle_id.clone()),
                    name: Some(bundle.name.clone()),
                    owner: bundle.owner.clone(),
                    description: bundle.description.clone(),
                    compute_type: bundle
                        .compute_type_name
                        .as_ref()
                        .map(|n| model::ComputeType {
                            name: Some(n.clone()),
                        }),
                    creation_time: rfc3339_to_epoch(&bundle.creation_time),
                    root_storage: bundle.root_storage_capacity.map(|c| model::RootStorage {
                        capacity: c.to_string(),
                    }),
                    user_storage: bundle.user_storage_capacity.map(|c| model::UserStorage {
                        capacity: c.to_string(),
                    }),
                    ..Default::default()
                };
                let result = model::CreateWorkspaceBundleResult {
                    workspace_bundle: Some(wb),
                };
                wire::serialize_create_workspace_bundle_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_delete_workspace_bundle(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_workspace_bundle_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let bundle_id = match input.bundle_id {
            Some(id) if !id.is_empty() => id,
            _ => {
                return json_error_response(400, "ValidationException", "Missing 'BundleId' field");
            }
        };

        let mut state = state.write().await;
        match state.delete_workspace_bundle(&bundle_id) {
            Ok(()) => {
                let result = model::DeleteWorkspaceBundleResult {};
                wire::serialize_delete_workspace_bundle_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_delete_workspace_image(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_workspace_image_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.image_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ImageId' field");
        }
        let image_id = input.image_id;

        let mut state = state.write().await;
        match state.delete_workspace_image(&image_id) {
            Ok(()) => {
                let result = model::DeleteWorkspaceImageResult {};
                wire::serialize_delete_workspace_image_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    // --- Pool handlers ---

    async fn handle_create_workspaces_pool(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_workspaces_pool_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.pool_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PoolName' field");
        }
        let pool_name = input.pool_name;
        if input.bundle_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'BundleId' field");
        }
        let bundle_id = input.bundle_id;
        if input.directory_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DirectoryId' field");
        }
        let directory_id = input.directory_id;
        let description = if input.description.is_empty() {
            None
        } else {
            Some(input.description)
        };

        let mut state = state.write().await;
        let pool = state.create_workspaces_pool(
            &pool_name,
            description.as_deref(),
            &bundle_id,
            &directory_id,
            account_id,
            region,
        );

        let pool_model = pool_to_model(&pool);
        let result = model::CreateWorkspacesPoolResult {
            workspaces_pool: Some(pool_model),
        };
        wire::serialize_create_workspaces_pool_response(&result)
    }

    async fn handle_describe_workspaces_pools(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_workspaces_pools_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let pool_ids = input.pool_ids;

        let state = state.read().await;
        let pools = state.describe_workspaces_pools(pool_ids.as_deref());

        let entries: Vec<model::WorkspacesPool> = pools.iter().map(|p| pool_to_model(p)).collect();

        let result = model::DescribeWorkspacesPoolsResult {
            workspaces_pools: Some(entries),
            next_token: None,
        };
        wire::serialize_describe_workspaces_pools_response(&result)
    }

    async fn handle_terminate_workspaces_pool(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_terminate_workspaces_pool_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PoolId' field");
        }
        let pool_id = input.pool_id;

        let mut state = state.write().await;
        match state.terminate_workspaces_pool(&pool_id) {
            Ok(pool) => {
                let pool_model = pool_to_model(&pool);
                let result = model::TerminateWorkspacesPoolResult {};
                let _ = pool_model;
                wire::serialize_terminate_workspaces_pool_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_update_workspaces_pool(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_workspaces_pool_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PoolId' field");
        }
        let pool_id = input.pool_id;
        let description = input.description.as_deref();
        let bundle_id = input.bundle_id.as_deref();
        let directory_id = input.directory_id.as_deref();

        let mut state = state.write().await;
        match state.update_workspaces_pool(&pool_id, description, bundle_id, directory_id) {
            Ok(pool) => {
                let pool_model = pool_to_model(&pool);
                let result = model::UpdateWorkspacesPoolResult {
                    workspaces_pool: Some(pool_model),
                };
                wire::serialize_update_workspaces_pool_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_start_workspaces_pool(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_workspaces_pool_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PoolId' field");
        }
        let pool_id = input.pool_id;

        let mut state = state.write().await;
        match state.start_workspaces_pool(&pool_id) {
            Ok(()) => {
                let result = model::StartWorkspacesPoolResult {};
                wire::serialize_start_workspaces_pool_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }

    async fn handle_stop_workspaces_pool(
        &self,
        state: &Arc<tokio::sync::RwLock<WorkSpacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_workspaces_pool_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PoolId' field");
        }
        let pool_id = input.pool_id;

        let mut state = state.write().await;
        match state.stop_workspaces_pool(&pool_id) {
            Ok(()) => {
                let result = model::StopWorkspacesPoolResult {};
                wire::serialize_stop_workspaces_pool_response(&result)
            }
            Err(e) => workspaces_error_response(&e),
        }
    }
}

// --- Utility functions ---

/// Convert a state Workspace to the wire model Workspace.
fn workspace_to_model(ws: &Workspace) -> model::Workspace {
    model::Workspace {
        workspace_id: Some(ws.workspace_id.clone()),
        directory_id: Some(ws.directory_id.clone()),
        user_name: Some(ws.user_name.clone()),
        bundle_id: Some(ws.bundle_id.clone()),
        state: Some(ws.state.clone()),
        ip_address: Some(ws.ip_address.clone()),
        computer_name: Some(ws.computer_name.clone()),
        subnet_id: Some(ws.subnet_id.clone()),
        root_volume_encryption_enabled: Some(ws.root_volume_encryption_enabled),
        user_volume_encryption_enabled: Some(ws.user_volume_encryption_enabled),
        volume_encryption_key: ws.volume_encryption_key.clone(),
        workspace_properties: Some(model::WorkspaceProperties {
            running_mode: Some(ws.running_mode.clone()),
            running_mode_auto_stop_timeout_in_minutes: Some(
                ws.running_mode_auto_stop_timeout_in_minutes,
            ),
            root_volume_size_gib: Some(ws.root_volume_size_gib),
            user_volume_size_gib: Some(ws.user_volume_size_gib),
            ..Default::default()
        }),
        ..Default::default()
    }
}

/// Convert a state WorkspaceDirectory to the wire model WorkspaceDirectory.
fn directory_to_model(dir: &WorkspaceDirectory) -> model::WorkspaceDirectory {
    model::WorkspaceDirectory {
        directory_id: Some(dir.directory_id.clone()),
        directory_name: Some(dir.directory_name.clone()),
        directory_type: Some(dir.directory_type.clone()),
        alias: Some(dir.alias.clone()),
        state: Some(dir.state.clone()),
        registration_code: Some(dir.registration_code.clone()),
        workspace_security_group_id: Some(dir.workspace_security_group_id.clone()),
        iam_role_id: Some(dir.iam_role_id.clone()),
        ..Default::default()
    }
}

/// Convert a state WorkspacesPool to the wire model WorkspacesPool.
fn pool_to_model(pool: &WorkspacesPool) -> model::WorkspacesPool {
    model::WorkspacesPool {
        pool_id: Some(pool.pool_id.clone()),
        pool_arn: Some(pool.pool_arn.clone()),
        pool_name: Some(pool.pool_name.clone()),
        description: pool.description.clone(),
        state: Some(pool.state.clone()),
        bundle_id: Some(pool.bundle_id.clone()),
        directory_id: Some(pool.directory_id.clone()),
        created_at: rfc3339_to_epoch(&pool.created_at),
        ..Default::default()
    }
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}

fn workspaces_error_response(e: &crate::state::WorkSpacesError) -> MockResponse {
    use crate::state::WorkSpacesError;
    let (status, error_type) = match e {
        WorkSpacesError::WorkspaceNotFound(_) => (400u16, "ResourceNotFoundException"),
        WorkSpacesError::ImageNotFound(_) => (400u16, "ResourceNotFoundException"),
        WorkSpacesError::DirectoryNotFound(_) => (400u16, "ResourceNotFoundException"),
        WorkSpacesError::DirectoryAlreadyRegistered(_) => {
            (400u16, "ResourceAlreadyExistsException")
        }
        WorkSpacesError::IpGroupNotFound(_) => (400u16, "ResourceNotFoundException"),
        WorkSpacesError::ConnectionAliasNotFound(_) => (400u16, "ResourceNotFoundException"),
        WorkSpacesError::BundleNotFound(_) => (400u16, "ResourceNotFoundException"),
        WorkSpacesError::PoolNotFound(_) => (400u16, "ResourceNotFoundException"),
    };
    json_error_response(status, error_type, &e.to_string())
}
