use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{EfsError, EfsState};
use crate::types::Tag;
use crate::views::EfsStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct EfsService {
    pub(crate) state: Arc<BackendState<EfsState>>,
    pub(crate) notifier: StateChangeNotifier<EfsStateView>,
}

impl EfsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for EfsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for EfsService {
    fn service_name(&self) -> &str {
        "elasticfilesystem"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://elasticfilesystem\..*\.amazonaws\.com",
            r"https?://elasticfilesystem\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl EfsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();

        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        // Query parameters
        let query = extract_query(&request.uri);

        // Expected routes (EFS API v1):
        // POST   /2015-02-01/file-systems                       - CreateFileSystem
        // GET    /2015-02-01/file-systems                       - DescribeFileSystems
        // DELETE /2015-02-01/file-systems/{FileSystemId}         - DeleteFileSystem
        // POST   /2015-02-01/mount-targets                      - CreateMountTarget
        // GET    /2015-02-01/mount-targets                      - DescribeMountTargets
        // DELETE /2015-02-01/mount-targets/{MountTargetId}       - DeleteMountTarget

        if segments.is_empty() || segments[0] != "2015-02-01" {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        if segments.len() < 2 {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        // Validate body up-front; the typed deserialisers re-parse per operation.
        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return rest_json_error(400, "BadRequest", "Invalid JSON body");
        }

        // Build a HashMap view of the query for wire deserialisers; the existing
        // tuple-based `query` is retained for handlers that consume multi-value
        // params (e.g., UntagResource's repeated `tagKeys`).
        let mut query_map: HashMap<String, String> = HashMap::new();
        for (k, v) in &query {
            query_map.insert(k.clone(), v.clone());
        }

        let response = match (method, segments[1], segments.len()) {
            // POST /2015-02-01/file-systems - CreateFileSystem
            ("POST", "file-systems", 2) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_create_file_system(
                    &state, &request, labels, &query_map, account_id, &region,
                )
                .await
            }
            // GET /2015-02-01/file-systems - DescribeFileSystems
            ("GET", "file-systems", 2) => self.handle_describe_file_systems(&state, &query).await,
            // DELETE /2015-02-01/file-systems/{FileSystemId} - DeleteFileSystem
            ("DELETE", "file-systems", 3) if segments[2] != "replication-configurations" => {
                let fs_id = percent_decode(segments[2]);
                self.handle_delete_file_system(&state, &fs_id).await
            }
            // POST /2015-02-01/access-points - CreateAccessPoint
            ("POST", "access-points", 2) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_create_access_point(
                    &state, &request, labels, &query_map, account_id, &region,
                )
                .await
            }
            // GET /2015-02-01/access-points - DescribeAccessPoints
            ("GET", "access-points", 2) => self.handle_describe_access_points(&state, &query).await,
            // DELETE /2015-02-01/access-points/{AccessPointId} - DeleteAccessPoint
            ("DELETE", "access-points", 3) => {
                let ap_id = percent_decode(segments[2]);
                self.handle_delete_access_point(&state, &ap_id).await
            }
            // POST /2015-02-01/mount-targets - CreateMountTarget
            ("POST", "mount-targets", 2) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_create_mount_target(&state, &request, labels, &query_map, account_id)
                    .await
            }
            // GET /2015-02-01/mount-targets - DescribeMountTargets
            ("GET", "mount-targets", 2) => self.handle_describe_mount_targets(&state, &query).await,
            // DELETE /2015-02-01/mount-targets/{MountTargetId} - DeleteMountTarget
            ("DELETE", "mount-targets", 3) => {
                let mt_id = percent_decode(segments[2]);
                self.handle_delete_mount_target(&state, &mt_id).await
            }
            // PUT /2015-02-01/mount-targets/{MountTargetId}/security-groups - ModifyMountTargetSecurityGroups
            ("PUT", "mount-targets", 4) if segments[3] == "security-groups" => {
                let mt_id = percent_decode(segments[2]);
                let labels: &[(&str, &str)] = &[("MountTargetId", mt_id.as_str())];
                self.handle_modify_mount_target_security_groups(
                    &state, &request, labels, &query_map,
                )
                .await
            }
            // GET /2015-02-01/mount-targets/{MountTargetId}/security-groups - DescribeMountTargetSecurityGroups
            ("GET", "mount-targets", 4) if segments[3] == "security-groups" => {
                let mt_id = percent_decode(segments[2]);
                self.handle_describe_mount_target_security_groups(&state, &mt_id)
                    .await
            }
            // GET /2015-02-01/file-systems/{FileSystemId}/lifecycle-configuration - DescribeLifecycleConfiguration
            ("GET", "file-systems", 4) if segments[3] == "lifecycle-configuration" => {
                let fs_id = percent_decode(segments[2]);
                self.handle_describe_lifecycle_configuration(&state, &fs_id)
                    .await
            }
            // PUT /2015-02-01/file-systems/{FileSystemId}/lifecycle-configuration - PutLifecycleConfiguration
            ("PUT", "file-systems", 4) if segments[3] == "lifecycle-configuration" => {
                let fs_id = percent_decode(segments[2]);
                let labels: &[(&str, &str)] = &[("FileSystemId", fs_id.as_str())];
                self.handle_put_lifecycle_configuration(&state, &request, labels, &query_map)
                    .await
            }
            // GET /2015-02-01/file-systems/{FileSystemId}/policy - DescribeFileSystemPolicy
            ("GET", "file-systems", 4) if segments[3] == "policy" => {
                let fs_id = percent_decode(segments[2]);
                self.handle_describe_file_system_policy(&state, &fs_id)
                    .await
            }
            // PUT /2015-02-01/file-systems/{FileSystemId}/policy - PutFileSystemPolicy
            ("PUT", "file-systems", 4) if segments[3] == "policy" => {
                let fs_id = percent_decode(segments[2]);
                let labels: &[(&str, &str)] = &[("FileSystemId", fs_id.as_str())];
                self.handle_put_file_system_policy(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /2015-02-01/file-systems/{FileSystemId}/policy - DeleteFileSystemPolicy
            ("DELETE", "file-systems", 4) if segments[3] == "policy" => {
                let fs_id = percent_decode(segments[2]);
                self.handle_delete_file_system_policy(&state, &fs_id).await
            }
            // GET /2015-02-01/resource-tags/{ResourceId} - ListTagsForResource
            ("GET", "resource-tags", 3) => {
                let resource_id = percent_decode(segments[2]);
                self.handle_list_tags_for_resource(&state, &resource_id)
                    .await
            }
            // POST /2015-02-01/resource-tags/{ResourceId} - TagResource
            ("POST", "resource-tags", 3) => {
                let resource_id = percent_decode(segments[2]);
                let labels: &[(&str, &str)] = &[("ResourceId", resource_id.as_str())];
                self.handle_tag_resource(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /2015-02-01/resource-tags/{ResourceId} - UntagResource
            ("DELETE", "resource-tags", 3) => {
                let resource_id = percent_decode(segments[2]);
                self.handle_untag_resource(&state, &resource_id, &query)
                    .await
            }
            // GET /2015-02-01/file-systems/{FileSystemId}/backup-policy => DescribeBackupPolicy
            ("GET", "file-systems", 4) if segments[3] == "backup-policy" => {
                let fs_id = percent_decode(segments[2]);
                self.handle_describe_backup_policy(&state, &fs_id).await
            }
            // PUT /2015-02-01/file-systems/{FileSystemId}/backup-policy => PutBackupPolicy
            ("PUT", "file-systems", 4) if segments[3] == "backup-policy" => {
                let fs_id = percent_decode(segments[2]);
                let labels: &[(&str, &str)] = &[("FileSystemId", fs_id.as_str())];
                self.handle_put_backup_policy(&state, &request, labels, &query_map)
                    .await
            }
            // POST /2015-02-01/file-systems/{SourceFileSystemId}/replication-configuration => CreateReplicationConfiguration
            ("POST", "file-systems", 4) if segments[3] == "replication-configuration" => {
                let fs_id = percent_decode(segments[2]);
                let labels: &[(&str, &str)] = &[("SourceFileSystemId", fs_id.as_str())];
                self.handle_create_replication_configuration(
                    &state, &request, labels, &query_map, account_id, &region,
                )
                .await
            }
            // DELETE /2015-02-01/file-systems/{SourceFileSystemId}/replication-configuration => DeleteReplicationConfiguration
            ("DELETE", "file-systems", 4) if segments[3] == "replication-configuration" => {
                let fs_id = percent_decode(segments[2]);
                self.handle_delete_replication_configuration(&state, &fs_id)
                    .await
            }
            // GET /2015-02-01/file-systems/replication-configurations => DescribeReplicationConfigurations
            ("GET", "file-systems", 3) if segments[2] == "replication-configurations" => {
                self.handle_describe_replication_configurations(&state, &query)
                    .await
            }
            // POST /2015-02-01/create-tags/{FileSystemId} => CreateTags
            ("POST", "create-tags", 3) => {
                let fs_id = percent_decode(segments[2]);
                let labels: &[(&str, &str)] = &[("FileSystemId", fs_id.as_str())];
                self.handle_create_tags(&state, &request, labels, &query_map)
                    .await
            }
            // POST /2015-02-01/delete-tags/{FileSystemId} => DeleteTags
            ("POST", "delete-tags", 3) => {
                let fs_id = percent_decode(segments[2]);
                let labels: &[(&str, &str)] = &[("FileSystemId", fs_id.as_str())];
                self.handle_delete_tags(&state, &request, labels, &query_map)
                    .await
            }
            // GET /2015-02-01/tags/{FileSystemId} => DescribeTags
            ("GET", "tags", 3) => {
                let fs_id = percent_decode(segments[2]);
                self.handle_describe_tags(&state, &fs_id).await
            }
            // GET /2015-02-01/account-preferences => DescribeAccountPreferences
            ("GET", "account-preferences", 2) => {
                self.handle_describe_account_preferences(&state).await
            }
            // PUT /2015-02-01/account-preferences => PutAccountPreferences
            ("PUT", "account-preferences", 2) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_put_account_preferences(&state, &request, labels, &query_map)
                    .await
            }
            // PUT /2015-02-01/file-systems/{FileSystemId} => UpdateFileSystem
            ("PUT", "file-systems", 3) => {
                let fs_id = percent_decode(segments[2]);
                let labels: &[(&str, &str)] = &[("FileSystemId", fs_id.as_str())];
                self.handle_update_file_system(&state, &request, labels, &query_map)
                    .await
            }
            // PUT /2015-02-01/file-systems/{FileSystemId}/protection => UpdateFileSystemProtection
            ("PUT", "file-systems", 4) if segments[3] == "protection" => {
                let fs_id = percent_decode(segments[2]);
                let labels: &[(&str, &str)] = &[("FileSystemId", fs_id.as_str())];
                self.handle_update_file_system_protection(&state, &request, labels, &query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        if matches!(method, "PUT" | "POST" | "DELETE") && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_file_system(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_file_system_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequest", &e),
        };
        if input.creation_token.is_empty() {
            return rest_json_error(400, "BadRequest", "Missing required field 'CreationToken'");
        }
        let creation_token = input.creation_token.as_str();
        let performance_mode = input.performance_mode.as_deref();
        let throughput_mode = input.throughput_mode.as_deref();
        let encrypted = input.encrypted;

        let tags: Vec<Tag> = input
            .tags
            .unwrap_or_default()
            .into_iter()
            .map(|t| Tag {
                key: t.key,
                value: t.value,
            })
            .collect();

        let mut state = state.write().await;
        match state.create_file_system(
            creation_token,
            account_id,
            region,
            performance_mode,
            throughput_mode,
            encrypted,
            tags,
        ) {
            Ok(fs) => wire::serialize_create_file_system_response(&file_system_to_wire(&fs)),
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_describe_file_systems(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        query: &[(String, String)],
    ) -> MockResponse {
        let file_system_id = query
            .iter()
            .find(|(k, _)| k == "FileSystemId")
            .map(|(_, v)| v.as_str());
        let creation_token = query
            .iter()
            .find(|(k, _)| k == "CreationToken")
            .map(|(_, v)| v.as_str());

        let state = state.read().await;
        match state.describe_file_systems(file_system_id, creation_token) {
            Ok(file_systems) => {
                let entries: Vec<wire::FileSystemDescription> = file_systems
                    .iter()
                    .map(|fs| file_system_to_wire(fs))
                    .collect();
                wire::serialize_describe_file_systems_response(&wire::DescribeFileSystemsResponse {
                    file_systems: Some(entries),
                    ..Default::default()
                })
            }
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_delete_file_system(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        file_system_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_file_system(file_system_id) {
            Ok(()) => wire::serialize_delete_file_system_response(),
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_create_mount_target(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_mount_target_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequest", &e),
        };
        if input.file_system_id.is_empty() {
            return rest_json_error(400, "BadRequest", "Missing required field 'FileSystemId'");
        }
        if input.subnet_id.is_empty() {
            return rest_json_error(400, "BadRequest", "Missing required field 'SubnetId'");
        }
        let file_system_id = input.file_system_id.clone();
        let subnet_id = input.subnet_id.clone();
        let ip_address = input.ip_address;
        let security_groups: Option<Vec<String>> = input.security_groups;

        let mut state = state.write().await;
        match state.create_mount_target(
            &file_system_id,
            &subnet_id,
            account_id,
            ip_address.as_deref(),
            security_groups.as_deref(),
        ) {
            Ok(mt) => wire::serialize_create_mount_target_response(&mount_target_to_wire(&mt)),
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_describe_mount_targets(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        query: &[(String, String)],
    ) -> MockResponse {
        let file_system_id = query
            .iter()
            .find(|(k, _)| k == "FileSystemId")
            .map(|(_, v)| v.as_str());
        let mount_target_id = query
            .iter()
            .find(|(k, _)| k == "MountTargetId")
            .map(|(_, v)| v.as_str());

        let state = state.read().await;
        match state.describe_mount_targets(file_system_id, mount_target_id) {
            Ok(mount_targets) => {
                let entries: Vec<wire::MountTargetDescription> = mount_targets
                    .iter()
                    .map(|mt| mount_target_to_wire(mt))
                    .collect();
                wire::serialize_describe_mount_targets_response(
                    &wire::DescribeMountTargetsResponse {
                        mount_targets: Some(entries),
                        ..Default::default()
                    },
                )
            }
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_delete_mount_target(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        mount_target_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_mount_target(mount_target_id) {
            Ok(()) => wire::serialize_delete_mount_target_response(),
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_create_access_point(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        // RootDirectory / PosixUser presence is detected directly from the raw
        // body so that an explicit `{}` and an absent value can be distinguished.
        // The wire model has these as default-equipped non-Option structs, so the
        // typed input collapses the two states.
        let root_directory_present =
            winterbaume_core::body_has_top_level_field(&request.body, "RootDirectory");
        let posix_user_present =
            winterbaume_core::body_has_top_level_field(&request.body, "PosixUser");

        let input = match wire::deserialize_create_access_point_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequest", &e),
        };
        if input.client_token.is_empty() {
            return rest_json_error(400, "BadRequest", "Missing required field 'ClientToken'");
        }
        if input.file_system_id.is_empty() {
            return rest_json_error(400, "BadRequest", "Missing required field 'FileSystemId'");
        }
        let client_token = input.client_token.clone();
        let file_system_id = input.file_system_id.clone();

        let posix_user = if posix_user_present {
            input.posix_user.map(|p| crate::types::PosixUser {
                uid: p.uid,
                gid: p.gid,
                secondary_gids: p.secondary_gids,
            })
        } else {
            None
        };

        let root_directory = if root_directory_present {
            Some(input.root_directory.map(|r| crate::types::RootDirectory {
                path: r.path,
                creation_info: r.creation_info.map(|ci| crate::types::CreationInfo {
                    owner_uid: ci.owner_uid,
                    owner_gid: ci.owner_gid,
                    permissions: ci.permissions,
                }),
            }))
            .flatten()
            .or(Some(crate::types::RootDirectory {
                path: None,
                creation_info: None,
            }))
        } else {
            None
        };

        let tags: Vec<Tag> = input
            .tags
            .unwrap_or_default()
            .into_iter()
            .map(|t| Tag {
                key: t.key,
                value: t.value,
            })
            .collect();

        let mut state = state.write().await;
        match state.create_access_point(
            &client_token,
            &file_system_id,
            account_id,
            region,
            posix_user,
            root_directory,
            tags,
        ) {
            Ok(ap) => wire::serialize_create_access_point_response(&access_point_to_wire(&ap)),
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_delete_access_point(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        access_point_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_access_point(access_point_id) {
            Ok(()) => wire::serialize_delete_access_point_response(),
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_describe_access_points(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        query: &[(String, String)],
    ) -> MockResponse {
        let access_point_id = query
            .iter()
            .find(|(k, _)| k == "AccessPointId")
            .map(|(_, v)| v.as_str());
        let file_system_id = query
            .iter()
            .find(|(k, _)| k == "FileSystemId")
            .map(|(_, v)| v.as_str());

        let state = state.read().await;
        match state.describe_access_points(access_point_id, file_system_id) {
            Ok(access_points) => {
                let entries: Vec<wire::AccessPointDescription> = access_points
                    .iter()
                    .map(|ap| access_point_to_wire(ap))
                    .collect();
                wire::serialize_describe_access_points_response(
                    &wire::DescribeAccessPointsResponse {
                        access_points: Some(entries),
                        ..Default::default()
                    },
                )
            }
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_describe_mount_target_security_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        mount_target_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_mount_target_security_groups(mount_target_id) {
            Ok(security_groups) => wire::serialize_describe_mount_target_security_groups_response(
                &wire::DescribeMountTargetSecurityGroupsResponse {
                    security_groups: Some(security_groups),
                },
            ),
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_describe_file_system_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        file_system_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_file_system_policy(file_system_id) {
            Ok((fs_id, policy)) => wire::serialize_describe_file_system_policy_response(
                &wire::FileSystemPolicyDescription {
                    file_system_id: Some(fs_id),
                    policy: Some(policy),
                },
            ),
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_put_file_system_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_file_system_policy_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequest", &e),
        };
        if input.policy.is_empty() {
            return rest_json_error(400, "BadRequest", "Missing required field 'Policy'");
        }
        let file_system_id = input.file_system_id.clone();
        let policy = input.policy;

        let mut state = state.write().await;
        match state.put_file_system_policy(&file_system_id, &policy) {
            Ok((fs_id, policy)) => wire::serialize_put_file_system_policy_response(
                &wire::FileSystemPolicyDescription {
                    file_system_id: Some(fs_id),
                    policy: Some(policy),
                },
            ),
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_delete_file_system_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        file_system_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_file_system_policy(file_system_id) {
            Ok(()) => wire::serialize_delete_file_system_policy_response(),
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        resource_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_tags_for_resource(resource_id) {
            Ok(tags) => {
                let wire_tags: Vec<wire::Tag> = tags
                    .iter()
                    .map(|t| wire::Tag {
                        key: t.key.clone(),
                        value: t.value.clone(),
                    })
                    .collect();
                wire::serialize_list_tags_for_resource_response(
                    &wire::ListTagsForResourceResponse {
                        tags: Some(wire_tags),
                        ..Default::default()
                    },
                )
            }
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequest", &e),
        };
        let resource_id = input.resource_id.clone();
        let tags: Vec<Tag> = input
            .tags
            .into_iter()
            .map(|t| Tag {
                key: t.key,
                value: t.value,
            })
            .collect();

        let mut state = state.write().await;
        match state.tag_resource(&resource_id, tags) {
            Ok(()) => wire::serialize_tag_resource_response(),
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_describe_lifecycle_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        file_system_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_lifecycle_configuration(file_system_id) {
            Ok(policies) => {
                let wire_policies: Vec<wire::LifecyclePolicy> = policies
                    .iter()
                    .map(|p| wire::LifecyclePolicy {
                        transition_to_i_a: p.transition_to_ia.clone(),
                        transition_to_primary_storage_class: p
                            .transition_to_primary_storage_class
                            .clone(),
                        transition_to_archive: p.transition_to_archive.clone(),
                    })
                    .collect();
                wire::serialize_describe_lifecycle_configuration_response(
                    &wire::LifecycleConfigurationDescription {
                        lifecycle_policies: Some(wire_policies),
                    },
                )
            }
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_put_lifecycle_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_put_lifecycle_configuration_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "BadRequest", &e),
            };
        let file_system_id = input.file_system_id.clone();
        let lifecycle_policies: Vec<crate::types::LifecyclePolicy> = input
            .lifecycle_policies
            .into_iter()
            .map(|p| crate::types::LifecyclePolicy {
                transition_to_ia: p.transition_to_i_a,
                transition_to_primary_storage_class: p.transition_to_primary_storage_class,
                transition_to_archive: p.transition_to_archive,
            })
            .collect();

        let mut state = state.write().await;
        match state.put_lifecycle_configuration(&file_system_id, lifecycle_policies) {
            Ok(policies) => {
                let wire_policies: Vec<wire::LifecyclePolicy> = policies
                    .iter()
                    .map(|p| wire::LifecyclePolicy {
                        transition_to_i_a: p.transition_to_ia.clone(),
                        transition_to_primary_storage_class: p
                            .transition_to_primary_storage_class
                            .clone(),
                        transition_to_archive: p.transition_to_archive.clone(),
                    })
                    .collect();
                wire::serialize_put_lifecycle_configuration_response(
                    &wire::LifecycleConfigurationDescription {
                        lifecycle_policies: Some(wire_policies),
                    },
                )
            }
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_modify_mount_target_security_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_mount_target_security_groups_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequest", &e),
        };
        let mount_target_id = input.mount_target_id.clone();
        let security_groups: Vec<String> = input.security_groups.unwrap_or_default();

        let mut state = state.write().await;
        match state.modify_mount_target_security_groups(&mount_target_id, security_groups) {
            Ok(()) => wire::serialize_modify_mount_target_security_groups_response(),
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        resource_id: &str,
        query: &[(String, String)],
    ) -> MockResponse {
        let tag_keys: Vec<String> = query
            .iter()
            .filter(|(k, _)| k == "tagKeys")
            .map(|(_, v)| v.clone())
            .collect();

        let mut state = state.write().await;
        match state.untag_resource(resource_id, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(),
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_describe_backup_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        fs_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_backup_policy(fs_id) {
            Ok(status_opt) => {
                let backup_policy = status_opt.map(|s| wire::BackupPolicy { status: s });
                wire::serialize_describe_backup_policy_response(&wire::BackupPolicyDescription {
                    backup_policy,
                })
            }
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_put_backup_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_backup_policy_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequest", &e),
        };
        let fs_id = input.file_system_id.clone();
        let status = if input.backup_policy.status.is_empty() {
            "ENABLED".to_string()
        } else {
            input.backup_policy.status
        };

        let mut state = state.write().await;
        match state.put_backup_policy(&fs_id, &status) {
            Ok(s) => wire::serialize_put_backup_policy_response(&wire::BackupPolicyDescription {
                backup_policy: Some(wire::BackupPolicy { status: s }),
            }),
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_create_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_tags_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequest", &e),
        };
        let file_system_id = input.file_system_id.clone();
        let tags: Vec<Tag> = input
            .tags
            .into_iter()
            .map(|t| Tag {
                key: t.key,
                value: t.value,
            })
            .collect();

        let mut state = state.write().await;
        match state.create_tags(&file_system_id, tags) {
            Ok(()) => wire::serialize_create_tags_response(),
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_delete_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_tags_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequest", &e),
        };
        let file_system_id = input.file_system_id.clone();
        let tag_keys: Vec<String> = input.tag_keys;

        let mut state = state.write().await;
        match state.delete_tags(&file_system_id, &tag_keys) {
            Ok(()) => wire::serialize_delete_tags_response(),
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_describe_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        file_system_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_tags(file_system_id) {
            Ok(tags) => {
                let wire_tags: Vec<wire::Tag> = tags
                    .iter()
                    .map(|t| wire::Tag {
                        key: t.key.clone(),
                        value: t.value.clone(),
                    })
                    .collect();
                wire::serialize_describe_tags_response(&wire::DescribeTagsResponse {
                    tags: Some(wire_tags),
                    ..Default::default()
                })
            }
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_create_replication_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_replication_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequest", &e),
        };
        let source_file_system_id = input.source_file_system_id.clone();
        let destinations: Vec<crate::types::ReplicationDestination> = input
            .destinations
            .into_iter()
            .map(|d| crate::types::ReplicationDestination {
                file_system_id: d.file_system_id.unwrap_or_default(),
                region: d.region.unwrap_or_else(|| region.to_string()),
                status: "ENABLED".to_string(),
                role_arn: d.role_arn,
            })
            .collect();

        let mut state = state.write().await;
        match state.create_replication_configuration(
            &source_file_system_id,
            account_id,
            region,
            destinations,
        ) {
            Ok(rc) => wire::serialize_create_replication_configuration_response(
                &replication_config_to_wire(&rc),
            ),
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_delete_replication_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        source_file_system_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_replication_configuration(source_file_system_id) {
            Ok(()) => wire::serialize_delete_replication_configuration_response(),
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_describe_replication_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        query: &[(String, String)],
    ) -> MockResponse {
        let file_system_id = query
            .iter()
            .find(|(k, _)| k == "FileSystemId")
            .map(|(_, v)| v.as_str());

        let state = state.read().await;
        match state.describe_replication_configurations(file_system_id) {
            Ok(configs) => {
                let entries: Vec<wire::ReplicationConfigurationDescription> = configs
                    .iter()
                    .map(|rc| replication_config_to_wire(rc))
                    .collect();
                wire::serialize_describe_replication_configurations_response(
                    &wire::DescribeReplicationConfigurationsResponse {
                        replications: Some(entries),
                        ..Default::default()
                    },
                )
            }
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_describe_account_preferences(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let prefs = state.describe_account_preferences();
        wire::serialize_describe_account_preferences_response(
            &wire::DescribeAccountPreferencesResponse {
                resource_id_preference: Some(wire::ResourceIdPreference {
                    resource_id_type: Some(prefs.resource_id_type),
                    resources: Some(prefs.resources),
                }),
                ..Default::default()
            },
        )
    }

    async fn handle_put_account_preferences(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_account_preferences_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequest", &e),
        };
        let resource_id_type = if input.resource_id_type.is_empty() {
            "LONG_ID".to_string()
        } else {
            input.resource_id_type
        };
        // The Smithy model only declares ResourceIdType; preserve the historical
        // default Resources list for state-layer compatibility.
        let resources: Vec<String> = vec!["FILE_SYSTEM".to_string(), "MOUNT_TARGET".to_string()];

        let mut state = state.write().await;
        let prefs = state.put_account_preferences(&resource_id_type, resources);
        wire::serialize_put_account_preferences_response(&wire::PutAccountPreferencesResponse {
            resource_id_preference: Some(wire::ResourceIdPreference {
                resource_id_type: Some(prefs.resource_id_type),
                resources: Some(prefs.resources),
            }),
        })
    }

    async fn handle_update_file_system(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_file_system_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequest", &e),
        };
        let file_system_id = input.file_system_id.clone();
        let throughput_mode = input.throughput_mode;
        let provisioned_throughput_in_mibps = input.provisioned_throughput_in_mibps;

        let mut state = state.write().await;
        match state.update_file_system(
            &file_system_id,
            throughput_mode.as_deref(),
            provisioned_throughput_in_mibps,
        ) {
            Ok(fs) => {
                let mut wire_fs = file_system_to_wire(&fs);
                if let Some(mibps) = provisioned_throughput_in_mibps {
                    wire_fs.provisioned_throughput_in_mibps = Some(mibps);
                }
                wire::serialize_update_file_system_response(&wire_fs)
            }
            Err(e) => efs_error_response(&e),
        }
    }

    async fn handle_update_file_system_protection(
        &self,
        state: &Arc<tokio::sync::RwLock<EfsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_file_system_protection_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "BadRequest", &e),
            };
        let file_system_id = input.file_system_id.clone();
        let replication_overwrite_protection = input.replication_overwrite_protection;

        let mut state = state.write().await;
        match state.update_file_system_protection(
            &file_system_id,
            replication_overwrite_protection.as_deref(),
        ) {
            Ok(protection) => wire::serialize_update_file_system_protection_response(
                &wire::FileSystemProtectionDescription {
                    replication_overwrite_protection: protection.replication_overwrite_protection,
                },
            ),
            Err(e) => efs_error_response(&e),
        }
    }
}

fn file_system_to_wire(fs: &crate::types::FileSystem) -> wire::FileSystemDescription {
    wire::FileSystemDescription {
        file_system_id: Some(fs.file_system_id.clone()),
        file_system_arn: Some(fs.file_system_arn.clone()),
        creation_time: Some(fs.creation_time.timestamp() as f64),
        owner_id: Some(fs.owner_id.clone()),
        creation_token: Some(fs.creation_token.clone()),
        performance_mode: Some(fs.performance_mode.clone()),
        throughput_mode: Some(fs.throughput_mode.clone()),
        life_cycle_state: Some(fs.life_cycle_state.clone()),
        number_of_mount_targets: Some(fs.number_of_mount_targets),
        size_in_bytes: Some(wire::FileSystemSize {
            value: Some(fs.size_in_bytes.value),
            timestamp: fs.size_in_bytes.timestamp.map(|ts| ts.timestamp() as f64),
            value_in_i_a: Some(fs.size_in_bytes.value_in_ia),
            value_in_standard: Some(fs.size_in_bytes.value_in_standard),
            ..Default::default()
        }),
        encrypted: Some(fs.encrypted),
        tags: Some(
            fs.tags
                .iter()
                .map(|t| wire::Tag {
                    key: t.key.clone(),
                    value: t.value.clone(),
                })
                .collect(),
        ),
        name: Some(fs.name.clone().unwrap_or_default()),
        ..Default::default()
    }
}

fn access_point_to_wire(ap: &crate::types::AccessPoint) -> wire::AccessPointDescription {
    wire::AccessPointDescription {
        access_point_id: Some(ap.access_point_id.clone()),
        access_point_arn: Some(ap.access_point_arn.clone()),
        client_token: Some(ap.client_token.clone()),
        file_system_id: Some(ap.file_system_id.clone()),
        life_cycle_state: Some(ap.life_cycle_state.clone()),
        name: ap.name.clone(),
        owner_id: Some(ap.owner_id.clone()),
        posix_user: ap.posix_user.as_ref().map(|pu| wire::PosixUser {
            uid: pu.uid,
            gid: pu.gid,
            secondary_gids: pu.secondary_gids.clone(),
        }),
        root_directory: ap.root_directory.as_ref().map(|rd| wire::RootDirectory {
            path: rd.path.clone(),
            creation_info: rd.creation_info.as_ref().map(|ci| wire::CreationInfo {
                owner_uid: ci.owner_uid,
                owner_gid: ci.owner_gid,
                permissions: ci.permissions.clone(),
            }),
        }),
        tags: Some(
            ap.tags
                .iter()
                .map(|t| wire::Tag {
                    key: t.key.clone(),
                    value: t.value.clone(),
                })
                .collect(),
        ),
    }
}

fn mount_target_to_wire(mt: &crate::types::MountTarget) -> wire::MountTargetDescription {
    wire::MountTargetDescription {
        mount_target_id: Some(mt.mount_target_id.clone()),
        file_system_id: Some(mt.file_system_id.clone()),
        subnet_id: Some(mt.subnet_id.clone()),
        life_cycle_state: Some(mt.life_cycle_state.clone()),
        ip_address: Some(mt.ip_address.clone()),
        network_interface_id: Some(mt.network_interface_id.clone()),
        availability_zone_id: Some(mt.availability_zone_id.clone()),
        availability_zone_name: Some(mt.availability_zone_name.clone()),
        owner_id: Some(mt.owner_id.clone()),
        ..Default::default()
    }
}

fn replication_config_to_wire(
    rc: &crate::types::ReplicationConfiguration,
) -> wire::ReplicationConfigurationDescription {
    wire::ReplicationConfigurationDescription {
        source_file_system_id: Some(rc.source_file_system_id.clone()),
        source_file_system_arn: Some(rc.source_file_system_arn.clone()),
        original_source_file_system_arn: Some(rc.original_source_file_system_arn.clone()),
        source_file_system_region: Some(rc.source_file_system_region.clone()),
        source_file_system_owner_id: Some(rc.source_file_system_owner_id.clone()),
        creation_time: Some(rc.creation_time.timestamp() as f64),
        destinations: Some(
            rc.destinations
                .iter()
                .map(|d| wire::Destination {
                    file_system_id: Some(d.file_system_id.clone()),
                    region: Some(d.region.clone()),
                    status: Some(d.status.clone()),
                    role_arn: d.role_arn.clone(),
                    ..Default::default()
                })
                .collect(),
        ),
    }
}

fn extract_path(uri: &str) -> String {
    // Delegate to the shared core helper, which correctly strips the scheme
    // and host (including custom-endpoint hostnames like `127.0.0.1:PORT`)
    // before returning the path. The previous implementation only matched on
    // `amazonaws.com` and returned the entire URI for non-AWS endpoints,
    // causing dispatch to fail with 404 against the in-process mock server.
    winterbaume_core::extract_path(uri)
}

fn extract_query(uri: &str) -> Vec<(String, String)> {
    if let Some(q) = uri.find('?') {
        uri[q + 1..]
            .split('&')
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                let key = parts.next()?;
                let value = parts.next().unwrap_or("");
                Some((percent_decode(key), percent_decode(value)))
            })
            .collect()
    } else {
        Vec::new()
    }
}

fn percent_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut bytes = s.bytes();
    while let Some(b) = bytes.next() {
        match b {
            b'%' => {
                let hi = bytes.next().and_then(hex_val);
                let lo = bytes.next().and_then(hex_val);
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    result.push((hi << 4 | lo) as char);
                }
            }
            b'+' => result.push(' '),
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

fn efs_error_response(err: &EfsError) -> MockResponse {
    let (status, error_type) = match err {
        EfsError::FileSystemAlreadyExists(_) => (409, "FileSystemAlreadyExists"),
        EfsError::FileSystemNotFound(_) => (404, "FileSystemNotFound"),
        EfsError::FileSystemInUse(_) => (409, "FileSystemInUse"),
        EfsError::MountTargetNotFound(_) => (404, "MountTargetNotFound"),
        EfsError::MissingMountTargetFilter => (400, "BadRequest"),
        EfsError::AccessPointNotFound(_) => (404, "AccessPointNotFound"),
        EfsError::PolicyNotFound(_) => (404, "PolicyNotFound"),
        EfsError::ReplicationAlreadyExists(_) => (409, "ReplicationAlreadyExists"),
        EfsError::ReplicationNotFound(_) => (404, "ReplicationNotFound"),
        EfsError::ResourceNotFound(_) => (404, "FileSystemNotFound"),
    };
    let body = json!({
        "ErrorCode": error_type,
        "Message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "ErrorCode": code,
        "Message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
