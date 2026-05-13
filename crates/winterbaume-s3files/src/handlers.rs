use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id, extract_path, percent_decode, rest_json_error,
};

use crate::state::{S3FilesError, S3FilesState};
use crate::types::{
    CreationPermissions, ExpirationDataRule, ImportDataRule, PosixUser, RootDirectory,
};
use crate::views::S3FilesStateView;
use crate::wire;

pub struct S3FilesService {
    pub(crate) state: Arc<BackendState<S3FilesState>>,
    pub(crate) notifier: StateChangeNotifier<S3FilesStateView>,
}

impl S3FilesService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for S3FilesService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for S3FilesService {
    fn service_name(&self) -> &str {
        "s3files"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://s3files\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl S3FilesService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();
        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();
        let query = winterbaume_core::extract_query_string(&request.uri);
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(query);

        let response = match (method, segments.as_slice()) {
            ("PUT", ["file-systems"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_create_file_system(&state, &request, labels, &query_map, &region)
                    .await
            }
            ("GET", ["file-systems"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_list_file_systems(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["file-systems", id]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("fileSystemId", id_decoded.as_str())];
                self.handle_get_file_system(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["file-systems", id]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("fileSystemId", id_decoded.as_str())];
                self.handle_delete_file_system(&state, &request, labels, &query_map)
                    .await
            }
            // --- Mount targets ---
            ("PUT", ["mount-targets"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_create_mount_target(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["mount-targets"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_list_mount_targets(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["mount-targets", id]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("mountTargetId", id_decoded.as_str())];
                self.handle_get_mount_target(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["mount-targets", id]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("mountTargetId", id_decoded.as_str())];
                self.handle_delete_mount_target(&state, &request, labels, &query_map)
                    .await
            }
            ("PUT", ["mount-targets", id]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("mountTargetId", id_decoded.as_str())];
                self.handle_update_mount_target(&state, &request, labels, &query_map)
                    .await
            }
            // --- Access points ---
            ("PUT", ["access-points"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_create_access_point(&state, &request, labels, &query_map, &region)
                    .await
            }
            ("GET", ["access-points"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_list_access_points(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["access-points", id]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("accessPointId", id_decoded.as_str())];
                self.handle_get_access_point(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["access-points", id]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("accessPointId", id_decoded.as_str())];
                self.handle_delete_access_point(&state, &request, labels, &query_map)
                    .await
            }
            // --- File system policy ---
            ("PUT", ["file-systems", id, "policy"]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("fileSystemId", id_decoded.as_str())];
                self.handle_put_file_system_policy(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["file-systems", id, "policy"]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("fileSystemId", id_decoded.as_str())];
                self.handle_get_file_system_policy(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["file-systems", id, "policy"]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("fileSystemId", id_decoded.as_str())];
                self.handle_delete_file_system_policy(&state, &request, labels, &query_map)
                    .await
            }
            // --- Synchronization configuration ---
            ("PUT", ["file-systems", id, "synchronization-configuration"]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("fileSystemId", id_decoded.as_str())];
                self.handle_put_synchronization_configuration(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["file-systems", id, "synchronization-configuration"]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("fileSystemId", id_decoded.as_str())];
                self.handle_get_synchronization_configuration(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", ["resource-tags", id]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("resourceId", id_decoded.as_str())];
                self.handle_tag_resource(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["resource-tags", id]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("resourceId", id_decoded.as_str())];
                self.handle_untag_resource(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["resource-tags", id]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("resourceId", id_decoded.as_str())];
                self.handle_list_tags_for_resource(&state, &request, labels, &query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        let mutating = matches!(method, "PUT" | "POST" | "DELETE");
        if mutating && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_file_system(
        &self,
        state: &Arc<tokio::sync::RwLock<S3FilesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_file_system_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let tags = tags_from_input(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_file_system(
            input.bucket.clone(),
            input.prefix.clone(),
            input.role_arn.clone(),
            input.kms_key_id.clone(),
            input.client_token.clone(),
            tags,
            default_account_id(),
            region,
        ) {
            Ok(fs) => {
                wire::serialize_create_file_system_response(&wire::CreateFileSystemResponse {
                    bucket: Some(fs.bucket.clone()),
                    client_token: fs.client_token.clone(),
                    creation_time: Some(fs.creation_time.timestamp() as f64),
                    file_system_arn: Some(fs.file_system_arn.clone()),
                    file_system_id: Some(fs.file_system_id.clone()),
                    kms_key_id: fs.kms_key_id.clone(),
                    name: fs.name.clone(),
                    owner_id: Some(fs.owner_id.clone()),
                    prefix: fs.prefix.clone(),
                    role_arn: Some(fs.role_arn.clone()),
                    status: Some(fs.status.clone()),
                    status_message: fs.status_message.clone(),
                    tags: Some(tags_to_wire(&fs.tags)),
                })
            }
            Err(e) => s3files_error_response(&e),
        }
    }

    async fn handle_get_file_system(
        &self,
        state: &Arc<tokio::sync::RwLock<S3FilesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_file_system_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_file_system(&input.file_system_id) {
            Ok(fs) => wire::serialize_get_file_system_response(&wire::GetFileSystemResponse {
                bucket: Some(fs.bucket.clone()),
                client_token: fs.client_token.clone(),
                creation_time: Some(fs.creation_time.timestamp() as f64),
                file_system_arn: Some(fs.file_system_arn.clone()),
                file_system_id: Some(fs.file_system_id.clone()),
                kms_key_id: fs.kms_key_id.clone(),
                name: fs.name.clone(),
                owner_id: Some(fs.owner_id.clone()),
                prefix: fs.prefix.clone(),
                role_arn: Some(fs.role_arn.clone()),
                status: Some(fs.status.clone()),
                status_message: fs.status_message.clone(),
                tags: Some(tags_to_wire(&fs.tags)),
            }),
            Err(e) => s3files_error_response(&e),
        }
    }

    async fn handle_list_file_systems(
        &self,
        state: &Arc<tokio::sync::RwLock<S3FilesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_file_systems_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let entries: Vec<wire::ListFileSystemsDescription> = state
            .list_file_systems(input.bucket.as_deref())
            .into_iter()
            .map(|fs| wire::ListFileSystemsDescription {
                bucket: Some(fs.bucket.clone()),
                creation_time: Some(fs.creation_time.timestamp() as f64),
                file_system_arn: Some(fs.file_system_arn.clone()),
                file_system_id: Some(fs.file_system_id.clone()),
                name: fs.name.clone(),
                owner_id: Some(fs.owner_id.clone()),
                role_arn: Some(fs.role_arn.clone()),
                status: Some(fs.status.clone()),
                status_message: fs.status_message.clone(),
            })
            .collect();
        wire::serialize_list_file_systems_response(&wire::ListFileSystemsResponse {
            file_systems: Some(entries),
            next_token: None,
        })
    }

    async fn handle_delete_file_system(
        &self,
        state: &Arc<tokio::sync::RwLock<S3FilesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_file_system_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let force_delete = input.force_delete.unwrap_or(false);
        let mut state = state.write().await;
        match state.delete_file_system(&input.file_system_id, force_delete) {
            Ok(()) => wire::serialize_delete_file_system_response(),
            Err(e) => s3files_error_response(&e),
        }
    }

    // --- Mount target handlers ----------------------------------------

    async fn handle_create_mount_target(
        &self,
        state: &Arc<tokio::sync::RwLock<S3FilesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_mount_target_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.file_system_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'fileSystemId'");
        }
        if input.subnet_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'subnetId'");
        }
        let security_groups = input.security_groups.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_mount_target(
            &input.file_system_id,
            input.subnet_id.clone(),
            input.ipv4_address.clone(),
            input.ipv6_address.clone(),
            input.ip_address_type.clone(),
            security_groups,
            default_account_id(),
        ) {
            Ok(mt) => {
                wire::serialize_create_mount_target_response(&wire::CreateMountTargetResponse {
                    availability_zone_id: Some(mt.availability_zone_id.clone()),
                    file_system_id: Some(mt.file_system_id.clone()),
                    ipv4_address: mt.ipv4_address.clone(),
                    ipv6_address: mt.ipv6_address.clone(),
                    mount_target_id: Some(mt.mount_target_id.clone()),
                    network_interface_id: Some(mt.network_interface_id.clone()),
                    owner_id: Some(mt.owner_id.clone()),
                    security_groups: Some(mt.security_groups.clone()),
                    status: Some(mt.status.clone()),
                    status_message: mt.status_message.clone(),
                    subnet_id: Some(mt.subnet_id.clone()),
                    vpc_id: Some(mt.vpc_id.clone()),
                })
            }
            Err(e) => s3files_error_response(&e),
        }
    }

    async fn handle_get_mount_target(
        &self,
        state: &Arc<tokio::sync::RwLock<S3FilesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_mount_target_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_mount_target(&input.mount_target_id) {
            Ok(mt) => wire::serialize_get_mount_target_response(&wire::GetMountTargetResponse {
                availability_zone_id: Some(mt.availability_zone_id.clone()),
                file_system_id: Some(mt.file_system_id.clone()),
                ipv4_address: mt.ipv4_address.clone(),
                ipv6_address: mt.ipv6_address.clone(),
                mount_target_id: Some(mt.mount_target_id.clone()),
                network_interface_id: Some(mt.network_interface_id.clone()),
                owner_id: Some(mt.owner_id.clone()),
                security_groups: Some(mt.security_groups.clone()),
                status: Some(mt.status.clone()),
                status_message: mt.status_message.clone(),
                subnet_id: Some(mt.subnet_id.clone()),
                vpc_id: Some(mt.vpc_id.clone()),
            }),
            Err(e) => s3files_error_response(&e),
        }
    }

    async fn handle_list_mount_targets(
        &self,
        state: &Arc<tokio::sync::RwLock<S3FilesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_mount_targets_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // Note: the original handler also accepted 'vpcId' as a query parameter,
        // but it isn't part of the model schema. Keep backward compat by reading raw.
        let vpc_id = query.get("vpcId").cloned();
        let state = state.read().await;
        let mts: Vec<wire::ListMountTargetsDescription> = state
            .list_mount_targets(
                input.file_system_id.as_deref(),
                input.access_point_id.as_deref(),
                vpc_id.as_deref(),
            )
            .into_iter()
            .map(|mt| wire::ListMountTargetsDescription {
                availability_zone_id: Some(mt.availability_zone_id.clone()),
                file_system_id: Some(mt.file_system_id.clone()),
                ipv4_address: mt.ipv4_address.clone(),
                ipv6_address: mt.ipv6_address.clone(),
                mount_target_id: Some(mt.mount_target_id.clone()),
                network_interface_id: Some(mt.network_interface_id.clone()),
                owner_id: Some(mt.owner_id.clone()),
                status: Some(mt.status.clone()),
                status_message: mt.status_message.clone(),
                subnet_id: Some(mt.subnet_id.clone()),
                vpc_id: Some(mt.vpc_id.clone()),
            })
            .collect();
        wire::serialize_list_mount_targets_response(&wire::ListMountTargetsResponse {
            mount_targets: Some(mts),
            next_token: None,
        })
    }

    async fn handle_delete_mount_target(
        &self,
        state: &Arc<tokio::sync::RwLock<S3FilesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_mount_target_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_mount_target(&input.mount_target_id) {
            Ok(()) => wire::serialize_delete_mount_target_response(),
            Err(e) => s3files_error_response(&e),
        }
    }

    async fn handle_update_mount_target(
        &self,
        state: &Arc<tokio::sync::RwLock<S3FilesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_mount_target_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.update_mount_target(&input.mount_target_id, input.security_groups) {
            Ok(mt) => {
                wire::serialize_update_mount_target_response(&wire::UpdateMountTargetResponse {
                    availability_zone_id: Some(mt.availability_zone_id.clone()),
                    file_system_id: Some(mt.file_system_id.clone()),
                    ipv4_address: mt.ipv4_address.clone(),
                    ipv6_address: mt.ipv6_address.clone(),
                    mount_target_id: Some(mt.mount_target_id.clone()),
                    network_interface_id: Some(mt.network_interface_id.clone()),
                    owner_id: Some(mt.owner_id.clone()),
                    security_groups: Some(mt.security_groups.clone()),
                    status: Some(mt.status.clone()),
                    status_message: mt.status_message.clone(),
                    subnet_id: Some(mt.subnet_id.clone()),
                    vpc_id: Some(mt.vpc_id.clone()),
                })
            }
            Err(e) => s3files_error_response(&e),
        }
    }

    // --- Access point handlers ---------------------------------------

    async fn handle_create_access_point(
        &self,
        state: &Arc<tokio::sync::RwLock<S3FilesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_access_point_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.file_system_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'fileSystemId'");
        }
        let posix_user = input.posix_user.as_ref().map(posix_user_from_wire);
        let root_directory = input.root_directory.as_ref().map(root_directory_from_wire);
        let tags = tags_from_input(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_access_point(
            &input.file_system_id,
            posix_user,
            root_directory,
            input.client_token.clone(),
            tags,
            default_account_id(),
            region,
        ) {
            Ok(ap) => {
                wire::serialize_create_access_point_response(&wire::CreateAccessPointResponse {
                    access_point_arn: Some(ap.access_point_arn.clone()),
                    access_point_id: Some(ap.access_point_id.clone()),
                    client_token: ap.client_token.clone(),
                    file_system_id: Some(ap.file_system_id.clone()),
                    name: ap.name.clone(),
                    owner_id: Some(ap.owner_id.clone()),
                    posix_user: ap.posix_user.as_ref().map(posix_user_to_wire),
                    root_directory: ap.root_directory.as_ref().map(root_directory_to_wire),
                    status: Some(ap.status.clone()),
                    tags: Some(tags_to_wire(&ap.tags)),
                })
            }
            Err(e) => s3files_error_response(&e),
        }
    }

    async fn handle_get_access_point(
        &self,
        state: &Arc<tokio::sync::RwLock<S3FilesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_access_point_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_access_point(&input.access_point_id) {
            Ok(ap) => wire::serialize_get_access_point_response(&wire::GetAccessPointResponse {
                access_point_arn: Some(ap.access_point_arn.clone()),
                access_point_id: Some(ap.access_point_id.clone()),
                client_token: ap.client_token.clone(),
                file_system_id: Some(ap.file_system_id.clone()),
                name: ap.name.clone(),
                owner_id: Some(ap.owner_id.clone()),
                posix_user: ap.posix_user.as_ref().map(posix_user_to_wire),
                root_directory: ap.root_directory.as_ref().map(root_directory_to_wire),
                status: Some(ap.status.clone()),
                tags: Some(tags_to_wire(&ap.tags)),
            }),
            Err(e) => s3files_error_response(&e),
        }
    }

    async fn handle_list_access_points(
        &self,
        state: &Arc<tokio::sync::RwLock<S3FilesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_access_points_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.file_system_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'fileSystemId'");
        }
        let state = state.read().await;
        match state.list_access_points(&input.file_system_id) {
            Ok(aps) => {
                let entries: Vec<wire::ListAccessPointsDescription> = aps
                    .into_iter()
                    .map(|ap| wire::ListAccessPointsDescription {
                        access_point_arn: Some(ap.access_point_arn.clone()),
                        access_point_id: Some(ap.access_point_id.clone()),
                        file_system_id: Some(ap.file_system_id.clone()),
                        name: ap.name.clone(),
                        owner_id: Some(ap.owner_id.clone()),
                        posix_user: ap.posix_user.as_ref().map(posix_user_to_wire),
                        root_directory: ap.root_directory.as_ref().map(root_directory_to_wire),
                        status: Some(ap.status.clone()),
                    })
                    .collect();
                wire::serialize_list_access_points_response(&wire::ListAccessPointsResponse {
                    access_points: Some(entries),
                    next_token: None,
                })
            }
            Err(e) => s3files_error_response(&e),
        }
    }

    async fn handle_delete_access_point(
        &self,
        state: &Arc<tokio::sync::RwLock<S3FilesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_access_point_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_access_point(&input.access_point_id) {
            Ok(()) => wire::serialize_delete_access_point_response(),
            Err(e) => s3files_error_response(&e),
        }
    }

    // --- File system policy handlers ---------------------------------

    async fn handle_put_file_system_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3FilesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_file_system_policy_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.policy.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'policy'");
        }
        let mut state = state.write().await;
        match state.put_file_system_policy(&input.file_system_id, input.policy.clone()) {
            Ok(()) => wire::serialize_put_file_system_policy_response(
                &wire::PutFileSystemPolicyResponse {},
            ),
            Err(e) => s3files_error_response(&e),
        }
    }

    async fn handle_get_file_system_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3FilesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_file_system_policy_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_file_system_policy(&input.file_system_id) {
            Ok(policy) => wire::serialize_get_file_system_policy_response(
                &wire::GetFileSystemPolicyResponse {
                    file_system_id: Some(input.file_system_id.clone()),
                    policy: Some(policy.to_string()),
                },
            ),
            Err(e) => s3files_error_response(&e),
        }
    }

    async fn handle_delete_file_system_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3FilesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_file_system_policy_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.delete_file_system_policy(&input.file_system_id) {
            Ok(()) => wire::serialize_delete_file_system_policy_response(),
            Err(e) => s3files_error_response(&e),
        }
    }

    // --- Synchronization configuration handlers ----------------------

    async fn handle_put_synchronization_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3FilesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_synchronization_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let import_data_rules: Vec<ImportDataRule> = input
            .import_data_rules
            .iter()
            .map(import_rule_from_wire)
            .collect();
        let expiration_data_rules: Vec<ExpirationDataRule> = input
            .expiration_data_rules
            .iter()
            .map(expiration_rule_from_wire)
            .collect();

        let mut state = state.write().await;
        match state.put_synchronization_configuration(
            &input.file_system_id,
            input.latest_version_number,
            import_data_rules,
            expiration_data_rules,
        ) {
            Ok(_) => wire::serialize_put_synchronization_configuration_response(
                &wire::PutSynchronizationConfigurationResponse {},
            ),
            Err(e) => s3files_error_response(&e),
        }
    }

    async fn handle_get_synchronization_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3FilesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_synchronization_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_synchronization_configuration(&input.file_system_id) {
            Ok(sync) => wire::serialize_get_synchronization_configuration_response(
                &wire::GetSynchronizationConfigurationResponse {
                    expiration_data_rules: Some(
                        sync.expiration_data_rules
                            .iter()
                            .map(expiration_rule_to_wire)
                            .collect(),
                    ),
                    import_data_rules: Some(
                        sync.import_data_rules
                            .iter()
                            .map(import_rule_to_wire)
                            .collect(),
                    ),
                    latest_version_number: Some(sync.latest_version_number),
                },
            ),
            Err(e) => s3files_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<S3FilesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let tags: HashMap<String, String> = input
            .tags
            .iter()
            .map(|t| (t.key.clone(), t.value.clone()))
            .collect();
        let mut state = state.write().await;
        match state.tag_resource(&input.resource_id, tags) {
            Ok(()) => wire::serialize_tag_resource_response(),
            Err(e) => s3files_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<S3FilesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.untag_resource(&input.resource_id, &input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(),
            Err(e) => s3files_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<S3FilesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_tags_for_resource(&input.resource_id) {
            Ok(tags) => wire::serialize_list_tags_for_resource_response(
                &wire::ListTagsForResourceResponse {
                    next_token: None,
                    tags: Some(tags_to_wire(tags)),
                },
            ),
            Err(e) => s3files_error_response(&e),
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn tags_from_input(tags: Option<&[wire::Tag]>) -> HashMap<String, String> {
    let mut out = HashMap::new();
    if let Some(t) = tags {
        for tag in t {
            out.insert(tag.key.clone(), tag.value.clone());
        }
    }
    out
}

fn tags_to_wire(tags: &HashMap<String, String>) -> Vec<wire::Tag> {
    let mut out: Vec<wire::Tag> = tags
        .iter()
        .map(|(k, v)| wire::Tag {
            key: k.clone(),
            value: v.clone(),
        })
        .collect();
    out.sort_by(|a, b| a.key.cmp(&b.key));
    out
}

fn s3files_error_response(err: &S3FilesError) -> MockResponse {
    let (status, error_type) = match err {
        S3FilesError::FileSystemNotFound(_) => (404u16, "ResourceNotFoundException"),
        S3FilesError::MountTargetNotFound(_) => (404, "ResourceNotFoundException"),
        S3FilesError::AccessPointNotFound(_) => (404, "ResourceNotFoundException"),
        S3FilesError::ResourceNotFound(_) => (404, "ResourceNotFoundException"),
        S3FilesError::PolicyNotFound(_) => (404, "ResourceNotFoundException"),
        S3FilesError::MountTargetVpcConflict { .. } => (409, "ConflictException"),
        S3FilesError::MountTargetAzConflict { .. } => (409, "ConflictException"),
        S3FilesError::SynchronizationVersionConflict { .. } => (409, "ConflictException"),
        S3FilesError::FileSystemInUse(_, _) => (409, "ConflictException"),
        S3FilesError::Validation { .. } => (400, "ValidationException"),
    };
    rest_json_error(status, error_type, &err.to_string())
}

// ── Conversions between wire model and crate::types ─────────────────────

fn posix_user_from_wire(p: &wire::PosixUser) -> PosixUser {
    PosixUser {
        uid: p.uid,
        gid: p.gid,
        secondary_gids: p.secondary_gids.clone().unwrap_or_default(),
    }
}

fn root_directory_from_wire(r: &wire::RootDirectory) -> RootDirectory {
    RootDirectory {
        path: r.path.clone(),
        creation_permissions: r
            .creation_permissions
            .as_ref()
            .map(creation_permissions_from_wire),
    }
}

fn creation_permissions_from_wire(c: &wire::CreationPermissions) -> CreationPermissions {
    CreationPermissions {
        owner_uid: c.owner_uid,
        owner_gid: c.owner_gid,
        permissions: c.permissions.clone(),
    }
}

fn import_rule_from_wire(r: &wire::ImportDataRule) -> ImportDataRule {
    ImportDataRule {
        prefix: r.prefix.clone(),
        size_less_than: r.size_less_than,
        trigger: r.trigger.clone(),
    }
}

fn expiration_rule_from_wire(r: &wire::ExpirationDataRule) -> ExpirationDataRule {
    ExpirationDataRule {
        days_after_last_access: r.days_after_last_access,
    }
}

fn posix_user_to_wire(p: &PosixUser) -> wire::PosixUser {
    wire::PosixUser {
        uid: p.uid,
        gid: p.gid,
        secondary_gids: if p.secondary_gids.is_empty() {
            None
        } else {
            Some(p.secondary_gids.clone())
        },
    }
}

fn root_directory_to_wire(r: &RootDirectory) -> wire::RootDirectory {
    wire::RootDirectory {
        path: r.path.clone(),
        creation_permissions: r
            .creation_permissions
            .as_ref()
            .map(creation_permissions_to_wire),
    }
}

fn creation_permissions_to_wire(c: &CreationPermissions) -> wire::CreationPermissions {
    wire::CreationPermissions {
        owner_uid: c.owner_uid,
        owner_gid: c.owner_gid,
        permissions: c.permissions.clone(),
    }
}

fn import_rule_to_wire(r: &ImportDataRule) -> wire::ImportDataRule {
    wire::ImportDataRule {
        prefix: r.prefix.clone(),
        size_less_than: r.size_less_than,
        trigger: r.trigger.clone(),
    }
}

fn expiration_rule_to_wire(r: &ExpirationDataRule) -> wire::ExpirationDataRule {
    wire::ExpirationDataRule {
        days_after_last_access: r.days_after_last_access,
    }
}
