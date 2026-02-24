use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{MemoryDbError, MemoryDbState};
use crate::views::MemoryDbStateView;
use crate::wire;

/// MemoryDB service handler that processes awsJson1.1 protocol requests.
pub struct MemoryDbService {
    pub(crate) state: Arc<BackendState<MemoryDbState>>,
    pub(crate) notifier: StateChangeNotifier<MemoryDbStateView>,
}

impl MemoryDbService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for MemoryDbService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for MemoryDbService {
    fn service_name(&self) -> &str {
        "memory-db"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://memory-db\..*\.amazonaws\.com",
            r"https?://memory-db\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl MemoryDbService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        // Extract action from X-Amz-Target header
        // Format: "AmazonMemoryDB.CreateCluster"
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
        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreateCluster" => {
                self.handle_create_cluster(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeClusters" => self.handle_describe_clusters(&state, body_bytes).await,
            "DeleteCluster" => {
                self.handle_delete_cluster(&state, body_bytes, account_id, &region)
                    .await
            }
            "UpdateCluster" => self.handle_update_cluster(&state, body_bytes).await,
            "CreateSnapshot" => {
                self.handle_create_snapshot(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteSnapshot" => self.handle_delete_snapshot(&state, body_bytes).await,
            "DescribeSnapshots" => self.handle_describe_snapshots(&state, body_bytes).await,
            "CreateSubnetGroup" => {
                self.handle_create_subnet_group(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteSubnetGroup" => self.handle_delete_subnet_group(&state, body_bytes).await,
            "DescribeSubnetGroups" => self.handle_describe_subnet_groups(&state, body_bytes).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ListTags" => self.handle_list_tags(&state, body_bytes).await,
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for MemoryDB"),
            ),
        };

        if matches!(
            action.as_str(),
            "CreateCluster"
                | "DeleteCluster"
                | "UpdateCluster"
                | "CreateSnapshot"
                | "DeleteSnapshot"
                | "CreateSubnetGroup"
                | "DeleteSubnetGroup"
                | "TagResource"
                | "UntagResource"
        ) && response.status / 100 == 2
        {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<MemoryDbState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_cluster_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterValueException", &e),
        };
        if input.cluster_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterValueException",
                "ClusterName is required",
            );
        }
        if input.node_type.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterValueException",
                "NodeType is required",
            );
        }
        if input.a_c_l_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterValueException",
                "ACLName is required",
            );
        }

        let description = input.description.as_deref();
        let num_shards = input.num_shards;
        let num_replicas_per_shard = input.num_replicas_per_shard;
        let subnet_group_name = input.subnet_group_name.as_deref();
        let security_group_ids = input.security_group_ids.unwrap_or_default();
        let engine_version = input.engine_version.as_deref();
        let maintenance_window = input.maintenance_window.as_deref();
        let snapshot_retention_limit = input.snapshot_retention_limit;
        let snapshot_window = input.snapshot_window.as_deref();
        let parameter_group_name = input.parameter_group_name.as_deref();
        let tls_enabled = input.t_l_s_enabled;
        let auto_minor_version_upgrade = input.auto_minor_version_upgrade;
        let tags = wire_tags_to_data(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_cluster(
            account_id,
            region,
            &input.cluster_name,
            &input.node_type,
            &input.a_c_l_name,
            description,
            num_shards,
            num_replicas_per_shard,
            subnet_group_name,
            security_group_ids,
            engine_version,
            maintenance_window,
            snapshot_retention_limit,
            snapshot_window,
            parameter_group_name,
            tls_enabled,
            auto_minor_version_upgrade,
            tags,
        ) {
            Ok(cluster) => wire::serialize_create_cluster_response(&wire::CreateClusterResponse {
                cluster: Some(cluster_to_wire(cluster)),
            }),
            Err(e) => memorydb_error_response(&e),
        }
    }

    async fn handle_describe_clusters(
        &self,
        state: &Arc<tokio::sync::RwLock<MemoryDbState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_clusters_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterValueException", &e),
        };
        let cluster_name = input.cluster_name.as_deref();
        let show_shard_details = input.show_shard_details.unwrap_or(false);

        let state = state.read().await;
        match state.describe_clusters(cluster_name) {
            Ok(clusters) => {
                wire::serialize_describe_clusters_response(&wire::DescribeClustersResponse {
                    clusters: Some(
                        clusters
                            .iter()
                            .map(|c| cluster_to_wire_with_shards(c, show_shard_details))
                            .collect(),
                    ),
                    next_token: None,
                })
            }
            Err(e) => memorydb_error_response(&e),
        }
    }

    async fn handle_delete_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<MemoryDbState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_cluster_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterValueException", &e),
        };
        if input.cluster_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterValueException",
                "ClusterName is required",
            );
        }
        let final_snapshot_name = input.final_snapshot_name.as_deref();

        let mut state = state.write().await;
        match state.delete_cluster(&input.cluster_name, final_snapshot_name, account_id, region) {
            Ok(cluster) => wire::serialize_delete_cluster_response(&wire::DeleteClusterResponse {
                cluster: Some(cluster_to_wire(&cluster)),
            }),
            Err(e) => memorydb_error_response(&e),
        }
    }

    async fn handle_update_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<MemoryDbState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_cluster_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterValueException", &e),
        };
        if input.cluster_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterValueException",
                "ClusterName is required",
            );
        }

        let description = input.description.as_deref();
        let security_group_ids = input.security_group_ids;
        let maintenance_window = input.maintenance_window.as_deref();
        let snapshot_retention_limit = input.snapshot_retention_limit;
        let snapshot_window = input.snapshot_window.as_deref();
        let parameter_group_name = input.parameter_group_name.as_deref();
        let engine_version = input.engine_version.as_deref();
        let num_shards = input
            .shard_configuration
            .as_ref()
            .and_then(|s| s.shard_count);
        let num_replicas_per_shard = input
            .replica_configuration
            .as_ref()
            .and_then(|r| r.replica_count);
        let node_type = input.node_type.as_deref();
        let acl_name = input.a_c_l_name.as_deref();

        let mut state = state.write().await;
        match state.update_cluster(
            &input.cluster_name,
            description,
            security_group_ids,
            maintenance_window,
            snapshot_retention_limit,
            snapshot_window,
            parameter_group_name,
            engine_version,
            num_shards,
            num_replicas_per_shard,
            node_type,
            acl_name,
        ) {
            Ok(cluster) => wire::serialize_update_cluster_response(&wire::UpdateClusterResponse {
                cluster: Some(cluster_to_wire(cluster)),
            }),
            Err(e) => memorydb_error_response(&e),
        }
    }

    // --- Snapshot operations ---

    async fn handle_create_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<MemoryDbState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_snapshot_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterValueException", &e),
        };
        if input.snapshot_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterValueException",
                "SnapshotName is required",
            );
        }
        if input.cluster_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterValueException",
                "ClusterName is required",
            );
        }

        let kms_key_id = input.kms_key_id.as_deref();
        let tags = wire_tags_to_data(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_snapshot(
            account_id,
            region,
            &input.snapshot_name,
            &input.cluster_name,
            kms_key_id,
            tags,
        ) {
            Ok(snapshot) => {
                wire::serialize_create_snapshot_response(&wire::CreateSnapshotResponse {
                    snapshot: Some(snapshot_to_wire(snapshot)),
                })
            }
            Err(e) => memorydb_error_response(&e),
        }
    }

    async fn handle_delete_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<MemoryDbState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_snapshot_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterValueException", &e),
        };
        if input.snapshot_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterValueException",
                "SnapshotName is required",
            );
        }

        let mut state = state.write().await;
        match state.delete_snapshot(&input.snapshot_name) {
            Ok(snapshot) => {
                wire::serialize_delete_snapshot_response(&wire::DeleteSnapshotResponse {
                    snapshot: Some(snapshot_to_wire(&snapshot)),
                })
            }
            Err(e) => memorydb_error_response(&e),
        }
    }

    async fn handle_describe_snapshots(
        &self,
        state: &Arc<tokio::sync::RwLock<MemoryDbState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_snapshots_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterValueException", &e),
        };
        let cluster_name = input.cluster_name.as_deref();
        let snapshot_name = input.snapshot_name.as_deref();
        let source = input.source.as_deref();
        let show_detail = input.show_detail.unwrap_or(false);

        let state = state.read().await;
        match state.describe_snapshots(cluster_name, snapshot_name, source) {
            Ok(snapshots) => {
                wire::serialize_describe_snapshots_response(&wire::DescribeSnapshotsResponse {
                    snapshots: Some(
                        snapshots
                            .iter()
                            .map(|s| snapshot_to_wire_with_details(s, show_detail))
                            .collect(),
                    ),
                    next_token: None,
                })
            }
            Err(e) => memorydb_error_response(&e),
        }
    }

    // --- Subnet group operations ---

    async fn handle_create_subnet_group(
        &self,
        state: &Arc<tokio::sync::RwLock<MemoryDbState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_subnet_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterValueException", &e),
        };
        if input.subnet_group_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterValueException",
                "SubnetGroupName is required",
            );
        }

        let subnet_ids = input.subnet_ids;
        let description = input.description.as_deref();
        let tags = wire_tags_to_data(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_subnet_group(
            account_id,
            region,
            &input.subnet_group_name,
            description,
            subnet_ids,
            tags,
        ) {
            Ok(sg) => {
                wire::serialize_create_subnet_group_response(&wire::CreateSubnetGroupResponse {
                    subnet_group: Some(subnet_group_to_wire(sg)),
                })
            }
            Err(e) => memorydb_error_response(&e),
        }
    }

    async fn handle_delete_subnet_group(
        &self,
        state: &Arc<tokio::sync::RwLock<MemoryDbState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_subnet_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterValueException", &e),
        };
        if input.subnet_group_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterValueException",
                "SubnetGroupName is required",
            );
        }

        let mut state = state.write().await;
        match state.delete_subnet_group(&input.subnet_group_name) {
            Ok(sg) => {
                wire::serialize_delete_subnet_group_response(&wire::DeleteSubnetGroupResponse {
                    subnet_group: Some(subnet_group_to_wire(&sg)),
                })
            }
            Err(e) => memorydb_error_response(&e),
        }
    }

    async fn handle_describe_subnet_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<MemoryDbState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_subnet_groups_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterValueException", &e),
        };
        let name = input.subnet_group_name.as_deref();

        let state = state.read().await;
        match state.describe_subnet_groups(name) {
            Ok(groups) => wire::serialize_describe_subnet_groups_response(
                &wire::DescribeSubnetGroupsResponse {
                    subnet_groups: Some(groups.iter().map(|sg| subnet_group_to_wire(sg)).collect()),
                    next_token: None,
                },
            ),
            Err(e) => memorydb_error_response(&e),
        }
    }

    // --- Tag operations ---

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<MemoryDbState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterValueException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterValueException",
                "ResourceArn is required",
            );
        }

        let tags = wire_tags_to_data(Some(input.tags.as_slice()));
        if tags.is_empty() {
            return json_error_response(400, "InvalidParameterValueException", "Tags is required");
        }

        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, tags) {
            Ok(tag_list) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {
                tag_list: Some(tags_to_wire(&tag_list)),
            }),
            Err(e) => memorydb_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<MemoryDbState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterValueException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterValueException",
                "ResourceArn is required",
            );
        }

        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, &input.tag_keys) {
            Ok(tag_list) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {
                tag_list: Some(tags_to_wire(&tag_list)),
            }),
            Err(e) => memorydb_error_response(&e),
        }
    }

    async fn handle_list_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<MemoryDbState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterValueException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterValueException",
                "ResourceArn is required",
            );
        }

        let state = state.read().await;
        match state.list_tags(&input.resource_arn) {
            Ok(tag_list) => wire::serialize_list_tags_response(&wire::ListTagsResponse {
                tag_list: Some(tags_to_wire(&tag_list)),
            }),
            Err(e) => memorydb_error_response(&e),
        }
    }
}

// --- Utility functions ---

fn cluster_to_wire(cluster: &crate::types::Cluster) -> wire::Cluster {
    cluster_to_wire_with_shards(cluster, false)
}

fn cluster_to_wire_with_shards(
    cluster: &crate::types::Cluster,
    show_shard_details: bool,
) -> wire::Cluster {
    let shards = if show_shard_details {
        Some(generate_shards(
            cluster.num_shards,
            cluster.num_replicas_per_shard,
        ))
    } else {
        None
    };

    wire::Cluster {
        name: Some(cluster.name.clone()),
        a_r_n: Some(cluster.arn.clone()),
        status: Some(cluster.status.clone()),
        node_type: Some(cluster.node_type.clone()),
        number_of_shards: Some(cluster.num_shards),
        description: Some(cluster.description.clone()),
        engine: Some(cluster.engine.clone()),
        engine_version: Some(cluster.engine_version.clone()),
        subnet_group_name: Some(cluster.subnet_group_name.clone()),
        security_groups: Some(
            cluster
                .security_group_ids
                .iter()
                .map(|sg| wire::SecurityGroupMembership {
                    security_group_id: Some(sg.clone()),
                    status: Some("active".to_string()),
                })
                .collect(),
        ),
        maintenance_window: Some(cluster.maintenance_window.clone()),
        snapshot_retention_limit: Some(cluster.snapshot_retention_limit),
        snapshot_window: Some(cluster.snapshot_window.clone()),
        a_c_l_name: Some(cluster.acl_name.clone()),
        parameter_group_name: Some(cluster.parameter_group_name.clone()),
        t_l_s_enabled: Some(cluster.tls_enabled),
        auto_minor_version_upgrade: Some(cluster.auto_minor_version_upgrade),
        shards,
        ..Default::default()
    }
}

/// Generate mock shard data matching moto behavior.
/// Each shard has num_replicas + 1 nodes (1 primary + N replicas).
fn generate_shards(num_shards: i32, num_replicas: i32) -> Vec<wire::Shard> {
    let total_slots = 16384;
    let slots_per_shard = total_slots / num_shards;
    (0..num_shards)
        .map(|i| {
            let start = i * slots_per_shard;
            let end = if i == num_shards - 1 {
                total_slots - 1
            } else {
                (i + 1) * slots_per_shard - 1
            };
            wire::Shard {
                name: Some(format!("{:04}", i + 1)),
                number_of_nodes: Some(num_replicas + 1),
                slots: Some(format!("{start}-{end}")),
                status: Some("available".to_string()),
                ..Default::default()
            }
        })
        .collect()
}

fn memorydb_error_response(err: &MemoryDbError) -> MockResponse {
    let (error_type, status) = match err {
        MemoryDbError::ClusterAlreadyExists(_) => ("ClusterAlreadyExistsFault", 400),
        MemoryDbError::ClusterNotFound(_) => ("ClusterNotFoundFault", 400),
        MemoryDbError::SnapshotAlreadyExists(_) => ("SnapshotAlreadyExistsFault", 400),
        MemoryDbError::SnapshotNotFound(_) => ("SnapshotNotFoundFault", 400),
        MemoryDbError::SubnetGroupAlreadyExists(_) => ("SubnetGroupAlreadyExistsFault", 400),
        MemoryDbError::SubnetGroupNotFound(_) => ("SubnetGroupNotFoundFault", 400),
        MemoryDbError::SubnetGroupInUse(_) => ("SubnetGroupInUseFault", 400),
        MemoryDbError::DefaultSubnetGroupCannotBeDeleted => ("InvalidParameterValueException", 400),
        MemoryDbError::TagNotFound(_) => ("TagNotFoundFault", 400),
        MemoryDbError::InvalidArn(_) => ("InvalidARNFault", 400),
    };
    let body = json!({
        "__type": error_type,
        "message": err.to_string(),
    });
    MockResponse::json(status, body.to_string())
}

fn snapshot_to_wire(snapshot: &crate::types::SnapshotData) -> wire::Snapshot {
    snapshot_to_wire_with_details(snapshot, false)
}

fn snapshot_to_wire_with_details(
    snapshot: &crate::types::SnapshotData,
    show_detail: bool,
) -> wire::Snapshot {
    let shards = if show_detail {
        Some(vec![wire::ShardDetail {
            name: Some("0001".to_string()),
            configuration: Some(wire::ShardConfiguration {
                slots: Some("0-16383".to_string()),
                replica_count: Some(1),
            }),
            size: Some("10".to_string()),
            ..Default::default()
        }])
    } else {
        None
    };

    wire::Snapshot {
        name: Some(snapshot.name.clone()),
        a_r_n: Some(snapshot.arn.clone()),
        status: Some(snapshot.status.clone()),
        source: Some(snapshot.source.clone()),
        cluster_configuration: Some(wire::ClusterConfiguration {
            name: Some(snapshot.cluster_name.clone()),
            description: Some(snapshot.cluster_description.clone()),
            engine: Some(snapshot.cluster_engine.clone()),
            engine_version: Some(snapshot.cluster_engine_version.clone()),
            node_type: Some(snapshot.cluster_node_type.clone()),
            num_shards: Some(snapshot.cluster_num_shards),
            subnet_group_name: Some(snapshot.cluster_subnet_group_name.clone()),
            snapshot_retention_limit: Some(snapshot.cluster_snapshot_retention_limit),
            snapshot_window: Some(snapshot.cluster_snapshot_window.clone()),
            maintenance_window: Some(snapshot.cluster_maintenance_window.clone()),
            parameter_group_name: Some(snapshot.cluster_parameter_group_name.clone()),
            shards,
            ..Default::default()
        }),
        kms_key_id: snapshot.kms_key_id.clone(),
        data_tiering: Some("false".to_string()),
        ..Default::default()
    }
}

fn subnet_group_to_wire(sg: &crate::types::SubnetGroupData) -> wire::SubnetGroup {
    wire::SubnetGroup {
        name: Some(sg.name.clone()),
        a_r_n: Some(sg.arn.clone()),
        description: Some(sg.description.clone()),
        vpc_id: Some(sg.vpc_id.clone()),
        subnets: Some(
            sg.subnet_ids
                .iter()
                .map(|sid| wire::Subnet {
                    identifier: Some(sid.clone()),
                    availability_zone: Some(wire::AvailabilityZone {
                        name: Some(az_for_subnet(sid)),
                    }),
                    ..Default::default()
                })
                .collect(),
        ),
        ..Default::default()
    }
}

/// Derive a stable, plausible AZ name from a subnet id by mapping the trailing
/// character of the id to one of three AZs. Distinct subnets in the same group
/// will therefore typically advertise distinct AZs in describe responses.
fn az_for_subnet(subnet_id: &str) -> String {
    const ZONES: [&str; 3] = ["us-east-1a", "us-east-1b", "us-east-1c"];
    let idx = subnet_id
        .chars()
        .last()
        .map(|c| (c as usize) % ZONES.len())
        .unwrap_or(0);
    ZONES[idx].to_string()
}

fn tags_to_wire(tags: &[crate::types::TagData]) -> Vec<wire::Tag> {
    tags.iter()
        .map(|t| wire::Tag {
            key: Some(t.key.clone()),
            value: Some(t.value.clone()),
        })
        .collect()
}

fn wire_tags_to_data(tags: Option<&[wire::Tag]>) -> Vec<crate::types::TagData> {
    tags.map(|arr| {
        arr.iter()
            .filter_map(|t| {
                let key = t.key.as_deref()?.to_string();
                let value = t.value.as_deref().unwrap_or("").to_string();
                Some(crate::types::TagData { key, value })
            })
            .collect()
    })
    .unwrap_or_default()
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}
