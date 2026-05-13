use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::state::{CloudHsmV2Error, CloudHsmV2State};
use crate::views::CloudHsmV2StateView;
use crate::{types, wire};

pub struct CloudHsmV2Service {
    pub(crate) state: Arc<BackendState<CloudHsmV2State>>,
    pub(crate) notifier: StateChangeNotifier<CloudHsmV2StateView>,
}

impl CloudHsmV2Service {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CloudHsmV2Service {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CloudHsmV2Service {
    fn service_name(&self) -> &str {
        "cloudhsm"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://cloudhsmv2\.(.+)\.amazonaws\.com",
            r"https?://cloudhsm\.(.+)\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl CloudHsmV2Service {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        // Extract action from X-Amz-Target header
        // Format: "BaldrApiService.CreateCluster"
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

        match action.as_str() {
            "CreateCluster" => {
                self.handle_create_cluster(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeClusters" => self.handle_describe_clusters(&state, body_bytes).await,
            "DeleteCluster" => self.handle_delete_cluster(&state, body_bytes).await,
            "InitializeCluster" => self.handle_initialize_cluster(&state, body_bytes).await,
            "CreateHsm" => self.handle_create_hsm(&state, body_bytes).await,
            "DeleteHsm" => self.handle_delete_hsm(&state, body_bytes).await,
            "DescribeBackups" => self.handle_describe_backups(&state, body_bytes).await,
            "DeleteBackup" => self.handle_delete_backup(&state, body_bytes).await,
            "RestoreBackup" => self.handle_restore_backup(&state, body_bytes).await,
            "CopyBackupToRegion" => {
                self.handle_copy_backup_to_region(&state, body_bytes, account_id)
                    .await
            }
            "ModifyBackupAttributes" => {
                self.handle_modify_backup_attributes(&state, body_bytes)
                    .await
            }
            "ModifyCluster" => self.handle_modify_cluster(&state, body_bytes).await,
            "ListTags" => self.handle_list_tags(&state, body_bytes).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "GetResourcePolicy" => self.handle_get_resource_policy(&state, body_bytes).await,
            "PutResourcePolicy" => self.handle_put_resource_policy(&state, body_bytes).await,
            "DeleteResourcePolicy" => self.handle_delete_resource_policy(&state, body_bytes).await,
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for CloudHSM V2"),
            ),
        }
    }

    async fn handle_create_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudHsmV2State>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_cluster_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.hsm_type.is_empty() {
            return json_error_response(
                400,
                "CloudHsmInvalidRequestException",
                "Missing 'HsmType'",
            );
        }
        if input.subnet_ids.is_empty() {
            return json_error_response(
                400,
                "CloudHsmInvalidRequestException",
                "Missing 'SubnetIds'",
            );
        }
        let backup_retention_policy =
            input
                .backup_retention_policy
                .map(|p| types::BackupRetentionPolicy {
                    r#type: p.r#type.unwrap_or_else(|| "DAYS".to_string()),
                    value: p.value,
                });
        let tag_list = wire_tags_to_internal(input.tag_list.unwrap_or_default());

        let mut state = state.write().await;
        match state.create_cluster(
            &input.hsm_type,
            input.subnet_ids,
            input.source_backup_id,
            backup_retention_policy,
            tag_list,
            account_id,
            region,
        ) {
            Ok(cluster) => {
                let resp = wire::CreateClusterResponse {
                    cluster: Some(cluster_to_wire(cluster)),
                };
                wire::serialize_create_cluster_response(&resp)
            }
            Err(e) => cloudhsmv2_error_response(&e),
        }
    }

    async fn handle_describe_clusters(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudHsmV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_clusters_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let filters = input.filters.unwrap_or_default();

        let state = state.read().await;
        let clusters = state.describe_clusters(&filters);

        let resp = wire::DescribeClustersResponse {
            clusters: Some(clusters.iter().map(|c| cluster_to_wire(c)).collect()),
            next_token: None,
        };
        wire::serialize_describe_clusters_response(&resp)
    }

    async fn handle_delete_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudHsmV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_cluster_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.cluster_id.is_empty() {
            return json_error_response(
                400,
                "CloudHsmInvalidRequestException",
                "Missing 'ClusterId'",
            );
        }

        let mut state = state.write().await;
        match state.delete_cluster(&input.cluster_id) {
            Ok(cluster) => {
                let resp = wire::DeleteClusterResponse {
                    cluster: Some(cluster_to_wire(cluster)),
                };
                wire::serialize_delete_cluster_response(&resp)
            }
            Err(e) => cloudhsmv2_error_response(&e),
        }
    }

    async fn handle_initialize_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudHsmV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_initialize_cluster_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.cluster_id.is_empty() {
            return json_error_response(
                400,
                "CloudHsmInvalidRequestException",
                "Missing 'ClusterId'",
            );
        }
        if input.signed_cert.is_empty() {
            return json_error_response(
                400,
                "CloudHsmInvalidRequestException",
                "Missing 'SignedCert'",
            );
        }
        if input.trust_anchor.is_empty() {
            return json_error_response(
                400,
                "CloudHsmInvalidRequestException",
                "Missing 'TrustAnchor'",
            );
        }

        let mut state = state.write().await;
        match state.initialize_cluster(&input.cluster_id, &input.signed_cert, &input.trust_anchor) {
            Ok(cluster) => {
                let resp = wire::InitializeClusterResponse {
                    state: Some(cluster.state.as_str().to_string()),
                    state_message: Some("Cluster is initialized".to_string()),
                };
                wire::serialize_initialize_cluster_response(&resp)
            }
            Err(e) => cloudhsmv2_error_response(&e),
        }
    }

    async fn handle_create_hsm(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudHsmV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_hsm_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.cluster_id.is_empty() {
            return json_error_response(
                400,
                "CloudHsmInvalidRequestException",
                "Missing 'ClusterId'",
            );
        }
        if input.availability_zone.is_empty() {
            return json_error_response(
                400,
                "CloudHsmInvalidRequestException",
                "Missing 'AvailabilityZone'",
            );
        }

        let mut state = state.write().await;
        match state.create_hsm(
            &input.cluster_id,
            &input.availability_zone,
            input.ip_address,
        ) {
            Ok(hsm) => {
                let resp = wire::CreateHsmResponse {
                    hsm: Some(hsm_to_wire(&hsm)),
                };
                wire::serialize_create_hsm_response(&resp)
            }
            Err(e) => cloudhsmv2_error_response(&e),
        }
    }

    async fn handle_delete_hsm(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudHsmV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_hsm_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.cluster_id.is_empty() {
            return json_error_response(
                400,
                "CloudHsmInvalidRequestException",
                "Missing 'ClusterId'",
            );
        }

        let hsm_id = input.hsm_id.as_deref();
        let eni_id = input.eni_id.as_deref();
        let eni_ip = input.eni_ip.as_deref();

        if hsm_id.is_none() && eni_id.is_none() && eni_ip.is_none() {
            return json_error_response(
                400,
                "CloudHsmInvalidRequestException",
                "Must specify one of: HsmId, EniId, or EniIp",
            );
        }

        let mut state = state.write().await;
        match state.delete_hsm(&input.cluster_id, hsm_id, eni_id, eni_ip) {
            Ok(deleted_hsm_id) => {
                let resp = wire::DeleteHsmResponse {
                    hsm_id: Some(deleted_hsm_id),
                };
                wire::serialize_delete_hsm_response(&resp)
            }
            Err(e) => cloudhsmv2_error_response(&e),
        }
    }

    async fn handle_describe_backups(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudHsmV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_backups_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let filters = input.filters.unwrap_or_default();
        let state = state.read().await;
        let backups = state.describe_backups(&filters);

        let resp = wire::DescribeBackupsResponse {
            backups: Some(backups.iter().map(|b| backup_to_wire(b)).collect()),
            next_token: None,
        };
        wire::serialize_describe_backups_response(&resp)
    }

    async fn handle_delete_backup(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudHsmV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_backup_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.backup_id.is_empty() {
            return json_error_response(
                400,
                "CloudHsmInvalidRequestException",
                "Missing 'BackupId'",
            );
        }

        let mut state = state.write().await;
        match state.delete_backup(&input.backup_id) {
            Ok(backup) => {
                let resp = wire::DeleteBackupResponse {
                    backup: Some(backup_to_wire(&backup)),
                };
                wire::serialize_delete_backup_response(&resp)
            }
            Err(e) => cloudhsmv2_error_response(&e),
        }
    }

    async fn handle_restore_backup(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudHsmV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_restore_backup_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.backup_id.is_empty() {
            return json_error_response(
                400,
                "CloudHsmInvalidRequestException",
                "Missing 'BackupId'",
            );
        }

        let mut state = state.write().await;
        match state.restore_backup(&input.backup_id) {
            Ok(backup) => {
                let resp = wire::RestoreBackupResponse {
                    backup: Some(backup_to_wire(&backup)),
                };
                wire::serialize_restore_backup_response(&resp)
            }
            Err(e) => cloudhsmv2_error_response(&e),
        }
    }

    async fn handle_copy_backup_to_region(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudHsmV2State>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_copy_backup_to_region_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.backup_id.is_empty() {
            return json_error_response(
                400,
                "CloudHsmInvalidRequestException",
                "Missing 'BackupId'",
            );
        }
        if input.destination_region.is_empty() {
            return json_error_response(
                400,
                "CloudHsmInvalidRequestException",
                "Missing 'DestinationRegion'",
            );
        }

        let tag_list = wire_tags_to_internal(input.tag_list.unwrap_or_default());

        let mut state = state.write().await;
        match state.copy_backup_to_region(
            &input.backup_id,
            &input.destination_region,
            tag_list,
            account_id,
        ) {
            Ok(backup) => {
                let resp = wire::CopyBackupToRegionResponse {
                    destination_backup: Some(wire::DestinationBackup {
                        create_timestamp: Some(backup.create_timestamp),
                        source_backup: backup.source_backup.clone(),
                        source_cluster: backup.source_cluster.clone(),
                        source_region: backup.source_region.clone(),
                    }),
                };
                wire::serialize_copy_backup_to_region_response(&resp)
            }
            Err(e) => cloudhsmv2_error_response(&e),
        }
    }

    async fn handle_modify_backup_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudHsmV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_modify_backup_attributes_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.backup_id.is_empty() {
            return json_error_response(
                400,
                "CloudHsmInvalidRequestException",
                "Missing 'BackupId'",
            );
        }

        let mut state = state.write().await;
        match state.modify_backup_attributes(&input.backup_id, input.never_expires) {
            Ok(backup) => {
                let resp = wire::ModifyBackupAttributesResponse {
                    backup: Some(backup_to_wire(&backup)),
                };
                wire::serialize_modify_backup_attributes_response(&resp)
            }
            Err(e) => cloudhsmv2_error_response(&e),
        }
    }

    async fn handle_modify_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudHsmV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_modify_cluster_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.cluster_id.is_empty() {
            return json_error_response(
                400,
                "CloudHsmInvalidRequestException",
                "Missing 'ClusterId'",
            );
        }

        let backup_retention_policy =
            input
                .backup_retention_policy
                .map(|p| types::BackupRetentionPolicy {
                    r#type: p.r#type.unwrap_or_else(|| "DAYS".to_string()),
                    value: p.value,
                });

        let mut state = state.write().await;
        match state.modify_cluster(&input.cluster_id, backup_retention_policy, input.hsm_type) {
            Ok(cluster) => {
                let resp = wire::ModifyClusterResponse {
                    cluster: Some(cluster_to_wire(cluster)),
                };
                wire::serialize_modify_cluster_response(&resp)
            }
            Err(e) => cloudhsmv2_error_response(&e),
        }
    }

    async fn handle_list_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudHsmV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_id.is_empty() {
            return json_error_response(
                400,
                "CloudHsmInvalidRequestException",
                "Missing 'ResourceId'",
            );
        }

        let state = state.read().await;
        match state.list_tags(&input.resource_id) {
            Ok(tags) => {
                let resp = wire::ListTagsResponse {
                    tag_list: Some(
                        tags.iter()
                            .map(|t| wire::Tag {
                                key: t.key.clone(),
                                value: t.value.clone(),
                            })
                            .collect(),
                    ),
                    next_token: None,
                };
                wire::serialize_list_tags_response(&resp)
            }
            Err(e) => cloudhsmv2_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudHsmV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_id.is_empty() {
            return json_error_response(
                400,
                "CloudHsmInvalidRequestException",
                "Missing 'ResourceId'",
            );
        }

        let tags = wire_tags_to_internal(input.tag_list);

        let mut state = state.write().await;
        match state.tag_resource(&input.resource_id, tags) {
            Ok(()) => {
                let resp = wire::TagResourceResponse {};
                wire::serialize_tag_resource_response(&resp)
            }
            Err(e) => cloudhsmv2_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudHsmV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_id.is_empty() {
            return json_error_response(
                400,
                "CloudHsmInvalidRequestException",
                "Missing 'ResourceId'",
            );
        }

        let mut state = state.write().await;
        match state.untag_resource(&input.resource_id, &input.tag_key_list) {
            Ok(()) => {
                let resp = wire::UntagResourceResponse {};
                wire::serialize_untag_resource_response(&resp)
            }
            Err(e) => cloudhsmv2_error_response(&e),
        }
    }

    async fn handle_get_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudHsmV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let resource_arn = input.resource_arn.as_deref().unwrap_or_default();

        let state = state.read().await;
        let policy = state.get_resource_policy(resource_arn);
        let resp = wire::GetResourcePolicyResponse { policy };
        wire::serialize_get_resource_policy_response(&resp)
    }

    async fn handle_put_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudHsmV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let resource_arn = match input.resource_arn.as_deref() {
            Some(arn) if !arn.is_empty() => arn,
            _ => {
                return json_error_response(
                    400,
                    "CloudHsmInvalidRequestException",
                    "Missing 'ResourceArn'",
                );
            }
        };

        let mut state = state.write().await;
        let stored_policy = state.put_resource_policy(resource_arn, input.policy);
        let resp = wire::PutResourcePolicyResponse {
            resource_arn: Some(resource_arn.to_string()),
            policy: Some(stored_policy),
        };
        wire::serialize_put_resource_policy_response(&resp)
    }

    async fn handle_delete_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudHsmV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let resource_arn = input.resource_arn.as_deref().unwrap_or_default();

        let mut state = state.write().await;
        let policy = state.delete_resource_policy(resource_arn);
        let resp = wire::DeleteResourcePolicyResponse {
            resource_arn: if resource_arn.is_empty() {
                None
            } else {
                Some(resource_arn.to_string())
            },
            policy,
        };
        wire::serialize_delete_resource_policy_response(&resp)
    }
}

fn cluster_to_wire(cluster: &types::Cluster) -> wire::Cluster {
    wire::Cluster {
        cluster_id: Some(cluster.cluster_id.clone()),
        hsm_type: Some(cluster.hsm_type.clone()),
        subnet_mapping: Some(cluster.subnet_mapping.clone()),
        vpc_id: Some(cluster.vpc_id.clone()),
        state: Some(cluster.state.as_str().to_string()),
        security_group: Some(cluster.security_group.clone()),
        backup_policy: Some(cluster.backup_policy.clone()),
        create_timestamp: Some(cluster.create_timestamp),
        hsms: Some(cluster.hsms.iter().map(hsm_to_wire).collect()),
        tag_list: Some(
            cluster
                .tag_list
                .iter()
                .map(|t| wire::Tag {
                    key: t.key.clone(),
                    value: t.value.clone(),
                })
                .collect(),
        ),
        source_backup_id: cluster.source_backup_id.clone(),
        backup_retention_policy: cluster.backup_retention_policy.as_ref().map(|p| {
            wire::BackupRetentionPolicy {
                r#type: Some(p.r#type.clone()),
                value: p.value.clone(),
            }
        }),
        ..Default::default()
    }
}

fn hsm_to_wire(hsm: &types::Hsm) -> wire::Hsm {
    wire::Hsm {
        hsm_id: Some(hsm.hsm_id.clone()),
        cluster_id: Some(hsm.cluster_id.clone()),
        availability_zone: Some(hsm.availability_zone.clone()),
        subnet_id: hsm.subnet_id.clone(),
        eni_id: hsm.eni_id.clone(),
        eni_ip: hsm.eni_ip.clone(),
        state: Some(hsm.state.clone()),
        ..Default::default()
    }
}

fn backup_to_wire(backup: &types::Backup) -> wire::Backup {
    wire::Backup {
        backup_id: Some(backup.backup_id.clone()),
        backup_arn: Some(backup.backup_arn.clone()),
        backup_state: Some(backup.backup_state.clone()),
        cluster_id: backup.cluster_id.clone(),
        create_timestamp: Some(backup.create_timestamp),
        copy_timestamp: backup.copy_timestamp,
        delete_timestamp: backup.delete_timestamp,
        hsm_type: backup.hsm_type.clone(),
        never_expires: Some(backup.never_expires),
        source_backup: backup.source_backup.clone(),
        source_cluster: backup.source_cluster.clone(),
        source_region: backup.source_region.clone(),
        tag_list: Some(
            backup
                .tag_list
                .iter()
                .map(|t| wire::Tag {
                    key: t.key.clone(),
                    value: t.value.clone(),
                })
                .collect(),
        ),
        ..Default::default()
    }
}

fn wire_tags_to_internal(tags: Vec<wire::Tag>) -> Vec<types::Tag> {
    tags.into_iter()
        .map(|t| types::Tag {
            key: t.key,
            value: t.value,
        })
        .collect()
}

fn cloudhsmv2_error_response(err: &CloudHsmV2Error) -> MockResponse {
    let (error_type, status) = match err {
        CloudHsmV2Error::ClusterNotFound { .. } => ("CloudHsmResourceNotFoundException", 400),
        CloudHsmV2Error::ClusterAlreadyDeleted { .. } => ("CloudHsmInvalidRequestException", 400),
        CloudHsmV2Error::ClusterNotInitializable { .. } => ("CloudHsmInvalidRequestException", 400),
        CloudHsmV2Error::ClusterDeletedCannotAddHsm { .. } => {
            ("CloudHsmInvalidRequestException", 400)
        }
        CloudHsmV2Error::ClusterDeletedCannotModify { .. } => {
            ("CloudHsmInvalidRequestException", 400)
        }
        CloudHsmV2Error::HsmNotFound { .. } => ("CloudHsmResourceNotFoundException", 400),
        CloudHsmV2Error::BackupNotFound { .. } => ("CloudHsmResourceNotFoundException", 400),
        CloudHsmV2Error::BackupAlreadyDeleted { .. } => ("CloudHsmInvalidRequestException", 400),
        CloudHsmV2Error::BackupNotRestorable { .. } => ("CloudHsmInvalidRequestException", 400),
        CloudHsmV2Error::ResourceNotFound { .. } => ("CloudHsmResourceNotFoundException", 400),
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
