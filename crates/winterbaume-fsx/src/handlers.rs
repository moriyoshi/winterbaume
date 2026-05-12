use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{Filter, FsxError, FsxState};
use crate::types::Tag;
use crate::views::FsxStateView;
use crate::wire;

pub struct FsxService {
    pub(crate) state: Arc<BackendState<FsxState>>,
    pub(crate) notifier: StateChangeNotifier<FsxStateView>,
}

impl FsxService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for FsxService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for FsxService {
    fn service_name(&self) -> &str {
        "fsx"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://fsx\..*\.amazonaws\.com",
            r"https?://fsx\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl FsxService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        // FSx uses awsJson1.1 protocol with X-Amz-Target header
        let target = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let operation = target.split('.').next_back().unwrap_or("");

        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error(400, "SerializationException", "Invalid JSON");
        }
        let body_bytes: &[u8] = &request.body;

        let response = match operation {
            "CreateFileSystem" => {
                let input = match wire::deserialize_create_file_system_request(body_bytes) {
                    Ok(v) => v,
                    Err(e) => return json_error(400, "ValidationException", &e),
                };
                let file_system_type = input.file_system_type.as_str();
                let storage_capacity = input.storage_capacity.unwrap_or(0) as i64;
                let storage_type = input.storage_type.as_deref().unwrap_or("");
                let subnet_ids = input.subnet_ids;
                let security_group_ids = input.security_group_ids.unwrap_or_default();
                let tags = input.tags.map(|ts| {
                    ts.into_iter()
                        .map(|t| Tag {
                            key: t.key,
                            value: t.value,
                        })
                        .collect()
                });
                let kms_key_id = input.kms_key_id;
                let windows_configuration = input
                    .windows_configuration
                    .as_ref()
                    .and_then(|c| serde_json::to_value(c).ok());
                let lustre_configuration = input
                    .lustre_configuration
                    .as_ref()
                    .and_then(|c| serde_json::to_value(c).ok());
                let ontap_configuration = input
                    .ontap_configuration
                    .as_ref()
                    .and_then(|c| serde_json::to_value(c).ok());
                let open_zfs_configuration = input
                    .open_z_f_s_configuration
                    .as_ref()
                    .and_then(|c| serde_json::to_value(c).ok());

                let mut state = state.write().await;
                let fs = state.create_file_system(
                    file_system_type,
                    storage_capacity,
                    storage_type,
                    subnet_ids,
                    security_group_ids,
                    tags,
                    kms_key_id,
                    windows_configuration,
                    lustre_configuration,
                    ontap_configuration,
                    open_zfs_configuration,
                    account_id,
                    &region,
                );
                wire::serialize_create_file_system_response(
                    &crate::model::CreateFileSystemResponse {
                        file_system: Some(file_system_to_wire(fs)),
                    },
                )
            }
            "DescribeFileSystems" => {
                let input = match wire::deserialize_describe_file_systems_request(body_bytes) {
                    Ok(v) => v,
                    Err(e) => return json_error(400, "ValidationException", &e),
                };
                let file_system_ids = input.file_system_ids;

                let state = state.read().await;
                let file_systems = state.describe_file_systems(file_system_ids.as_deref());
                let fs_list: Vec<crate::model::FileSystem> = file_systems
                    .iter()
                    .map(|fs| file_system_to_wire(fs))
                    .collect();
                wire::serialize_describe_file_systems_response(
                    &crate::model::DescribeFileSystemsResponse {
                        file_systems: Some(fs_list),
                        next_token: None,
                    },
                )
            }
            "DeleteFileSystem" => {
                let input = match wire::deserialize_delete_file_system_request(body_bytes) {
                    Ok(v) => v,
                    Err(e) => return json_error(400, "ValidationException", &e),
                };
                let file_system_id = input.file_system_id.as_str();
                let mut state = state.write().await;
                match state.delete_file_system(file_system_id) {
                    Ok((id, lifecycle, windows_resp, lustre_resp, open_zfs_resp)) => {
                        wire::serialize_delete_file_system_response(
                            &crate::model::DeleteFileSystemResponse {
                                file_system_id: Some(id),
                                lifecycle: Some(lifecycle),
                                windows_response: windows_resp.map(|_| {
                                    crate::model::DeleteFileSystemWindowsResponse {
                                        final_backup_id: Some(String::new()),
                                        final_backup_tags: Some(vec![]),
                                    }
                                }),
                                lustre_response: lustre_resp.map(|_| {
                                    crate::model::DeleteFileSystemLustreResponse {
                                        final_backup_id: Some(String::new()),
                                        final_backup_tags: Some(vec![]),
                                    }
                                }),
                                open_z_f_s_response: open_zfs_resp.map(|_| {
                                    crate::model::DeleteFileSystemOpenZFSResponse {
                                        final_backup_id: Some(String::new()),
                                        final_backup_tags: Some(vec![]),
                                    }
                                }),
                            },
                        )
                    }
                    Err(e) => fsx_error_response(&e),
                }
            }
            "CreateBackup" => {
                let input = match wire::deserialize_create_backup_request(body_bytes) {
                    Ok(v) => v,
                    Err(e) => return json_error(400, "ValidationException", &e),
                };
                let file_system_id = input.file_system_id.as_deref().unwrap_or("");
                let tags = input.tags.map(|ts| {
                    ts.into_iter()
                        .map(|t| Tag {
                            key: t.key,
                            value: t.value,
                        })
                        .collect()
                });

                let mut state = state.write().await;
                match state.create_backup(file_system_id, tags, account_id, &region) {
                    Ok(backup) => {
                        let backup = backup.clone();
                        let fs = state.file_systems.get(&backup.file_system_id);
                        wire::serialize_create_backup_response(
                            &crate::model::CreateBackupResponse {
                                backup: Some(backup_to_wire(&backup, fs)),
                            },
                        )
                    }
                    Err(e) => fsx_error_response(&e),
                }
            }
            "DeleteBackup" => {
                let input = match wire::deserialize_delete_backup_request(body_bytes) {
                    Ok(v) => v,
                    Err(e) => return json_error(400, "ValidationException", &e),
                };
                let backup_id = input.backup_id.as_str();
                let mut state = state.write().await;
                match state.delete_backup(backup_id) {
                    Ok((id, lifecycle)) => wire::serialize_delete_backup_response(
                        &crate::model::DeleteBackupResponse {
                            backup_id: Some(id),
                            lifecycle: Some(lifecycle),
                        },
                    ),
                    Err(e) => fsx_error_response(&e),
                }
            }
            "DescribeBackups" => {
                let input = match wire::deserialize_describe_backups_request(body_bytes) {
                    Ok(v) => v,
                    Err(e) => return json_error(400, "ValidationException", &e),
                };
                let backup_ids = input.backup_ids;
                let filters = input.filters.map(|fs| {
                    fs.into_iter()
                        .map(|f| Filter {
                            name: f.name.unwrap_or_default(),
                            values: f.values.unwrap_or_default(),
                        })
                        .collect::<Vec<_>>()
                });

                let state = state.read().await;
                let backups = state.describe_backups(backup_ids.as_deref(), filters.as_deref());
                let backup_list: Vec<crate::model::Backup> = backups
                    .iter()
                    .map(|b| {
                        let fs = state.file_systems.get(&b.file_system_id);
                        backup_to_wire(b, fs)
                    })
                    .collect();
                wire::serialize_describe_backups_response(&crate::model::DescribeBackupsResponse {
                    backups: Some(backup_list),
                    next_token: None,
                })
            }
            "TagResource" => {
                let input = match wire::deserialize_tag_resource_request(body_bytes) {
                    Ok(v) => v,
                    Err(e) => return json_error(400, "ValidationException", &e),
                };
                let resource_arn = input.resource_a_r_n.as_str();
                let tags: Vec<Tag> = input
                    .tags
                    .into_iter()
                    .map(|t| Tag {
                        key: t.key,
                        value: t.value,
                    })
                    .collect();
                let mut state = state.write().await;
                state.tag_resource(resource_arn, tags);
                wire::serialize_tag_resource_response(&crate::model::TagResourceResponse {})
            }
            "UntagResource" => {
                let input = match wire::deserialize_untag_resource_request(body_bytes) {
                    Ok(v) => v,
                    Err(e) => return json_error(400, "ValidationException", &e),
                };
                let resource_arn = input.resource_a_r_n.as_str();
                let tag_keys = input.tag_keys;
                let mut state = state.write().await;
                state.untag_resource(resource_arn, &tag_keys);
                wire::serialize_untag_resource_response(&crate::model::UntagResourceResponse {})
            }
            "ListTagsForResource" => {
                let input = match wire::deserialize_list_tags_for_resource_request(body_bytes) {
                    Ok(v) => v,
                    Err(e) => return json_error(400, "ValidationException", &e),
                };
                let resource_arn = input.resource_a_r_n.as_str();
                let state = state.read().await;
                let tags = state.list_tags_for_resource(resource_arn);
                let wire_tags: Vec<crate::model::Tag> =
                    tags.iter().map(|t| tag_to_wire(t)).collect();
                wire::serialize_list_tags_for_resource_response(
                    &crate::model::ListTagsForResourceResponse {
                        tags: Some(wire_tags),
                        next_token: None,
                    },
                )
            }
            _ => json_error(
                400,
                "UnknownOperationException",
                &format!("Unknown operation: {operation}"),
            ),
        };

        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }
}

/// Convert a state `FileSystem` to the wire `model::FileSystem` for serialization.
fn file_system_to_wire(fs: &crate::types::FileSystem) -> crate::model::FileSystem {
    crate::model::FileSystem {
        file_system_id: Some(fs.file_system_id.clone()),
        file_system_type: Some(fs.file_system_type.clone()),
        storage_capacity: Some(fs.storage_capacity as i32),
        storage_type: Some(fs.storage_type.clone()),
        subnet_ids: Some(fs.subnet_ids.clone()),
        d_n_s_name: Some(fs.dns_name.clone()),
        resource_a_r_n: Some(fs.resource_arn.clone()),
        kms_key_id: fs.kms_key_id.clone(),
        tags: if fs.tags.is_empty() {
            None
        } else {
            Some(fs.tags.iter().map(tag_to_wire).collect())
        },
        ..Default::default()
    }
}

/// Convert a state `Backup` to the wire `model::Backup` for serialization.
fn backup_to_wire(
    backup: &crate::types::Backup,
    fs: Option<&crate::types::FileSystem>,
) -> crate::model::Backup {
    crate::model::Backup {
        backup_id: Some(backup.backup_id.clone()),
        file_system: Some(fs.map(file_system_to_wire).unwrap_or_default()),
        lifecycle: Some(backup.lifecycle.clone()),
        creation_time: chrono::DateTime::parse_from_rfc3339(&backup.creation_time)
            .ok()
            .map(|dt| dt.timestamp() as f64),
        resource_a_r_n: Some(backup.resource_arn.clone()),
        r#type: Some("USER_INITIATED".to_string()),
        tags: if backup.tags.is_empty() {
            None
        } else {
            Some(backup.tags.iter().map(tag_to_wire).collect())
        },
        ..Default::default()
    }
}

/// Convert a state `Tag` to the wire `model::Tag`.
fn tag_to_wire(tag: &Tag) -> crate::model::Tag {
    crate::model::Tag {
        key: tag.key.clone(),
        value: tag.value.clone(),
    }
}

fn fsx_error_response(err: &FsxError) -> MockResponse {
    match err {
        FsxError::FileSystemNotFound { .. } => {
            json_error(404, "FileSystemNotFound", &err.to_string())
        }
        FsxError::ResourceNotFound { .. } => {
            json_error(400, "ResourceNotFoundException", &err.to_string())
        }
    }
}

fn json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "Message": message,
    });
    MockResponse::json(status, body.to_string())
}
