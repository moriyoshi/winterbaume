use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{NeptuneError, NeptuneState};
use crate::types::{Parameter, ServerlessV2ScalingConfiguration, Tag};
use crate::views::NeptuneStateView;
use crate::wire;

/// Neptune service handler.
pub struct NeptuneService {
    pub(crate) state: Arc<BackendState<NeptuneState>>,
    pub(crate) notifier: StateChangeNotifier<NeptuneStateView>,
}

impl NeptuneService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for NeptuneService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for NeptuneService {
    fn service_name(&self) -> &str {
        "neptune"
    }

    fn url_patterns(&self) -> Vec<&str> {
        // aws-sdk-neptune sends requests to rds.*.amazonaws.com (same endpoint prefix as RDS).
        // We register with more-specific patterns than winterbaume-rds so the neptune
        // service catches requests when it is registered.  In practice the test builds
        // a MockAws with only NeptuneService, so there is no conflict with RDS.
        vec![
            r"https?://rds\..*\.amazonaws\.com",
            r"https?://rds\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

const MUTATING_ACTIONS: &[&str] = &[
    "CreateDBCluster",
    "DeleteDBCluster",
    "ModifyDBCluster",
    "StartDBCluster",
    "StopDBCluster",
    "FailoverDBCluster",
    "RestoreDBClusterFromSnapshot",
    "RestoreDBClusterToPointInTime",
    "CreateDBInstance",
    "DeleteDBInstance",
    "ModifyDBInstance",
    "RebootDBInstance",
    "CreateDBSubnetGroup",
    "DeleteDBSubnetGroup",
    "ModifyDBSubnetGroup",
    "CreateDBParameterGroup",
    "DeleteDBParameterGroup",
    "ModifyDBParameterGroup",
    "ResetDBParameterGroup",
    "CreateDBClusterParameterGroup",
    "DeleteDBClusterParameterGroup",
    "ModifyDBClusterParameterGroup",
    "ResetDBClusterParameterGroup",
    "CreateDBClusterSnapshot",
    "DeleteDBClusterSnapshot",
    "CopyDBClusterSnapshot",
    "AddTagsToResource",
    "RemoveTagsFromResource",
    "AddRoleToDBCluster",
    "RemoveRoleFromDBCluster",
    "CreateEventSubscription",
    "DeleteEventSubscription",
    "ModifyEventSubscription",
    "CreateGlobalCluster",
    "DeleteGlobalCluster",
    "ModifyGlobalCluster",
    "CreateDBClusterEndpoint",
    "DeleteDBClusterEndpoint",
    "ModifyDBClusterEndpoint",
    "AddSourceIdentifierToSubscription",
    "RemoveSourceIdentifierFromSubscription",
    "ModifyDBClusterSnapshotAttribute",
    "FailoverGlobalCluster",
    "SwitchoverGlobalCluster",
    "RemoveFromGlobalCluster",
    "CopyDBClusterParameterGroup",
    "CopyDBParameterGroup",
];

impl NeptuneService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        let body_str = std::str::from_utf8(&request.body).unwrap_or("");
        let params = parse_query_string(body_str);

        let action = match params.get("Action") {
            Some(a) => a.clone(),
            None => {
                return MockResponse::error(400, "MissingAction", "Missing 'Action' parameter");
            }
        };

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            // DB Clusters
            "CreateDBCluster" => {
                self.handle_create_db_cluster(&state, &params, account_id, &region)
                    .await
            }
            "DescribeDBClusters" => self.handle_describe_db_clusters(&state, &params).await,
            "DeleteDBCluster" => self.handle_delete_db_cluster(&state, &params).await,
            "ModifyDBCluster" => self.handle_modify_db_cluster(&state, &params).await,
            "StartDBCluster" => self.handle_start_db_cluster(&state, &params).await,
            "StopDBCluster" => self.handle_stop_db_cluster(&state, &params).await,
            "FailoverDBCluster" => self.handle_failover_db_cluster(&state, &params).await,
            "PromoteReadReplicaDBCluster" => {
                self.handle_promote_read_replica_db_cluster(&state, &params)
                    .await
            }
            "RestoreDBClusterFromSnapshot" => {
                self.handle_restore_db_cluster_from_snapshot(&state, &params, account_id, &region)
                    .await
            }
            "RestoreDBClusterToPointInTime" => {
                self.handle_restore_db_cluster_to_point_in_time(
                    &state, &params, account_id, &region,
                )
                .await
            }
            // DB Instances
            "CreateDBInstance" => {
                self.handle_create_db_instance(&state, &params, account_id, &region)
                    .await
            }
            "DescribeDBInstances" => self.handle_describe_db_instances(&state, &params).await,
            "DeleteDBInstance" => self.handle_delete_db_instance(&state, &params).await,
            "ModifyDBInstance" => self.handle_modify_db_instance(&state, &params).await,
            "RebootDBInstance" => self.handle_reboot_db_instance(&state, &params).await,
            // DB Subnet Groups
            "CreateDBSubnetGroup" => {
                self.handle_create_db_subnet_group(&state, &params, account_id, &region)
                    .await
            }
            "DescribeDBSubnetGroups" => {
                self.handle_describe_db_subnet_groups(&state, &params).await
            }
            "DeleteDBSubnetGroup" => self.handle_delete_db_subnet_group(&state, &params).await,
            "ModifyDBSubnetGroup" => self.handle_modify_db_subnet_group(&state, &params).await,
            // DB Parameter Groups
            "CreateDBParameterGroup" => {
                self.handle_create_db_parameter_group(&state, &params, account_id, &region)
                    .await
            }
            "DescribeDBParameterGroups" => {
                self.handle_describe_db_parameter_groups(&state, &params)
                    .await
            }
            "DeleteDBParameterGroup" => {
                self.handle_delete_db_parameter_group(&state, &params).await
            }
            "ModifyDBParameterGroup" => {
                self.handle_modify_db_parameter_group(&state, &params).await
            }
            "ResetDBParameterGroup" => self.handle_reset_db_parameter_group(&state, &params).await,
            "DescribeDBParameters" => self.handle_describe_db_parameters(&state, &params).await,
            // DB Cluster Parameter Groups
            "CreateDBClusterParameterGroup" => {
                self.handle_create_db_cluster_parameter_group(&state, &params, account_id, &region)
                    .await
            }
            "DescribeDBClusterParameterGroups" => {
                self.handle_describe_db_cluster_parameter_groups(&state, &params)
                    .await
            }
            "DeleteDBClusterParameterGroup" => {
                self.handle_delete_db_cluster_parameter_group(&state, &params)
                    .await
            }
            "ModifyDBClusterParameterGroup" => {
                self.handle_modify_db_cluster_parameter_group(&state, &params)
                    .await
            }
            "ResetDBClusterParameterGroup" => {
                self.handle_reset_db_cluster_parameter_group(&state, &params)
                    .await
            }
            "DescribeDBClusterParameters" => {
                self.handle_describe_db_cluster_parameters(&state, &params)
                    .await
            }
            // DB Cluster Snapshots
            "CreateDBClusterSnapshot" => {
                self.handle_create_db_cluster_snapshot(&state, &params, account_id, &region)
                    .await
            }
            "DescribeDBClusterSnapshots" => {
                self.handle_describe_db_cluster_snapshots(&state, &params)
                    .await
            }
            "DeleteDBClusterSnapshot" => {
                self.handle_delete_db_cluster_snapshot(&state, &params)
                    .await
            }
            // Tags
            "AddTagsToResource" => self.handle_add_tags_to_resource(&state, &params).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, &params).await,
            "RemoveTagsFromResource" => {
                self.handle_remove_tags_from_resource(&state, &params).await
            }
            // Engine Versions
            "DescribeDBEngineVersions" => self.handle_describe_db_engine_versions(&params).await,
            "DescribeOrderableDBInstanceOptions" => {
                self.handle_describe_orderable_db_instance_options(&params)
                    .await
            }
            // Misc read-only
            "DescribeDBClusterSnapshotAttributes" => {
                self.handle_describe_db_cluster_snapshot_attributes(&state, &params)
                    .await
            }
            "ModifyDBClusterSnapshotAttribute" => {
                self.handle_modify_db_cluster_snapshot_attribute(&state, &params)
                    .await
            }
            "DescribeEngineDefaultClusterParameters" => {
                self.handle_describe_engine_default_cluster_parameters(&params)
                    .await
            }
            "DescribeEngineDefaultParameters" => {
                self.handle_describe_engine_default_parameters(&params)
                    .await
            }
            "DescribeEventCategories" => self.handle_describe_event_categories().await,
            "DescribeEvents" => self.handle_describe_events().await,
            "DescribePendingMaintenanceActions" => {
                self.handle_describe_pending_maintenance_actions().await
            }
            "DescribeValidDBInstanceModifications" => {
                self.handle_describe_valid_db_instance_modifications(&state, &params)
                    .await
            }
            // Global clusters
            "CreateGlobalCluster" => {
                self.handle_create_global_cluster(&state, &params, account_id, &region)
                    .await
            }
            "DescribeGlobalClusters" => self.handle_describe_global_clusters(&state, &params).await,
            "DeleteGlobalCluster" => self.handle_delete_global_cluster(&state, &params).await,
            "ModifyGlobalCluster" => self.handle_modify_global_cluster(&state, &params).await,
            "FailoverGlobalCluster" => self.handle_failover_global_cluster(&state, &params).await,
            "SwitchoverGlobalCluster" => {
                self.handle_switchover_global_cluster(&state, &params).await
            }
            "RemoveFromGlobalCluster" => {
                self.handle_remove_from_global_cluster(&state, &params)
                    .await
            }
            // Roles
            "AddRoleToDBCluster" => self.handle_add_role_to_db_cluster(&state, &params).await,
            "RemoveRoleFromDBCluster" => {
                self.handle_remove_role_from_db_cluster(&state, &params)
                    .await
            }
            // Endpoint (cluster endpoints)
            "CreateDBClusterEndpoint" => {
                self.handle_create_db_cluster_endpoint(&state, &params, account_id, &region)
                    .await
            }
            "DeleteDBClusterEndpoint" => {
                self.handle_delete_db_cluster_endpoint(&state, &params)
                    .await
            }
            "DescribeDBClusterEndpoints" => {
                self.handle_describe_db_cluster_endpoints(&state, &params)
                    .await
            }
            "ModifyDBClusterEndpoint" => {
                self.handle_modify_db_cluster_endpoint(&state, &params)
                    .await
            }
            // Copy
            "CopyDBClusterParameterGroup" => {
                self.handle_copy_db_cluster_parameter_group(&state, &params, account_id, &region)
                    .await
            }
            "CopyDBClusterSnapshot" => {
                self.handle_copy_db_cluster_snapshot(&state, &params, account_id, &region)
                    .await
            }
            "CopyDBParameterGroup" => {
                self.handle_copy_db_parameter_group(&state, &params, account_id, &region)
                    .await
            }
            // Event subscriptions
            "CreateEventSubscription" => {
                self.handle_create_event_subscription(&state, &params, account_id, &region)
                    .await
            }
            "DeleteEventSubscription" => {
                self.handle_delete_event_subscription(&state, &params).await
            }
            "DescribeEventSubscriptions" => {
                self.handle_describe_event_subscriptions(&state, &params)
                    .await
            }
            "ModifyEventSubscription" => {
                self.handle_modify_event_subscription(&state, &params).await
            }
            "AddSourceIdentifierToSubscription" => {
                self.handle_add_source_identifier_to_subscription(&state, &params)
                    .await
            }
            "RemoveSourceIdentifierFromSubscription" => {
                self.handle_remove_source_identifier_from_subscription(&state, &params)
                    .await
            }
            // Apply pending maintenance (stub)
            "ApplyPendingMaintenanceAction" => self.handle_apply_pending_maintenance_action().await,
            _ => MockResponse::error(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for Neptune"),
            ),
        };

        if MUTATING_ACTIONS.contains(&action.as_str()) && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // -------------------------------------------------------------------------
    // DB Clusters
    // -------------------------------------------------------------------------

    async fn handle_create_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_d_b_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_cluster_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBClusterIdentifier",
            );
        }
        let engine = if input.engine.is_empty() {
            "neptune".to_string()
        } else {
            input.engine
        };
        let vpc_security_group_ids = input
            .vpc_security_group_ids
            .map(|l| l.items)
            .unwrap_or_default();
        let availability_zones = input
            .availability_zones
            .map(|l| l.items)
            .unwrap_or_default();
        let tags = wire_tags_to_domain(input.tags);

        let serverless_v2_scaling_configuration = parse_serverless_v2_scaling_config(params);

        let mut st = state.write().await;
        match st.create_db_cluster(
            input.d_b_cluster_identifier,
            engine,
            input.engine_version,
            input.master_username,
            input.database_name,
            input.d_b_subnet_group_name,
            vpc_security_group_ids,
            availability_zones,
            input.storage_encrypted.unwrap_or(false),
            input.kms_key_id,
            input.d_b_cluster_parameter_group_name,
            params.get("EngineMode").cloned(),
            input.backup_retention_period.unwrap_or(1),
            input.deletion_protection.unwrap_or(false),
            serverless_v2_scaling_configuration,
            tags,
            account_id,
            region,
        ) {
            Ok(cluster) => {
                wire::serialize_create_d_b_cluster_response(&wire::CreateDBClusterResult {
                    d_b_cluster: Some(domain_cluster_to_wire(&cluster)),
                })
            }
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_describe_db_clusters(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_d_b_clusters_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        match st.describe_db_clusters(input.d_b_cluster_identifier.as_deref()) {
            Ok(clusters) => {
                wire::serialize_describe_d_b_clusters_response(&wire::DBClusterMessage {
                    d_b_clusters: Some(wire::DBClusterList {
                        items: clusters.iter().map(domain_cluster_to_wire).collect(),
                    }),
                    ..Default::default()
                })
            }
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_delete_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_d_b_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_cluster_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBClusterIdentifier",
            );
        }
        let mut st = state.write().await;
        match st.delete_db_cluster(&input.d_b_cluster_identifier) {
            Ok(cluster) => {
                wire::serialize_delete_d_b_cluster_response(&wire::DeleteDBClusterResult {
                    d_b_cluster: Some(domain_cluster_to_wire(&cluster)),
                })
            }
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_modify_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_d_b_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_cluster_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBClusterIdentifier",
            );
        }

        let serverless_v2_scaling_configuration = parse_serverless_v2_scaling_config(params);

        let mut st = state.write().await;
        match st.modify_db_cluster(
            &input.d_b_cluster_identifier,
            input.engine_version,
            input.d_b_cluster_parameter_group_name,
            input.backup_retention_period,
            input.deletion_protection,
            input.master_user_password,
            serverless_v2_scaling_configuration,
        ) {
            Ok(cluster) => {
                wire::serialize_modify_d_b_cluster_response(&wire::ModifyDBClusterResult {
                    d_b_cluster: Some(domain_cluster_to_wire(&cluster)),
                })
            }
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_start_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_start_d_b_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_cluster_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBClusterIdentifier",
            );
        }
        let identifier = &input.d_b_cluster_identifier;
        let st = state.read().await;
        match st.describe_db_clusters(Some(identifier)) {
            Ok(v) if !v.is_empty() => {
                wire::serialize_start_d_b_cluster_response(&wire::StartDBClusterResult {
                    d_b_cluster: Some(domain_cluster_to_wire(&v[0])),
                })
            }
            _ => neptune_error_response(&NeptuneError::NotFound {
                resource_type: "DBCluster".to_string(),
                name: identifier.to_string(),
            }),
        }
    }

    async fn handle_stop_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_stop_d_b_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_cluster_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBClusterIdentifier",
            );
        }
        let identifier = &input.d_b_cluster_identifier;
        let st = state.read().await;
        match st.describe_db_clusters(Some(identifier)) {
            Ok(v) if !v.is_empty() => {
                wire::serialize_stop_d_b_cluster_response(&wire::StopDBClusterResult {
                    d_b_cluster: Some(domain_cluster_to_wire(&v[0])),
                })
            }
            _ => neptune_error_response(&NeptuneError::NotFound {
                resource_type: "DBCluster".to_string(),
                name: identifier.to_string(),
            }),
        }
    }

    async fn handle_failover_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_failover_d_b_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = match &input.d_b_cluster_identifier {
            Some(v) if !v.is_empty() => v.as_str(),
            _ => {
                return MockResponse::error(
                    400,
                    "InvalidParameterValue",
                    "Missing DBClusterIdentifier",
                );
            }
        };
        let st = state.read().await;
        match st.describe_db_clusters(Some(identifier)) {
            Ok(v) if !v.is_empty() => {
                wire::serialize_failover_d_b_cluster_response(&wire::FailoverDBClusterResult {
                    d_b_cluster: Some(domain_cluster_to_wire(&v[0])),
                })
            }
            _ => neptune_error_response(&NeptuneError::NotFound {
                resource_type: "DBCluster".to_string(),
                name: identifier.to_string(),
            }),
        }
    }

    async fn handle_promote_read_replica_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_promote_read_replica_d_b_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_cluster_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBClusterIdentifier",
            );
        }
        let identifier = &input.d_b_cluster_identifier;
        let st = state.read().await;
        match st.describe_db_clusters(Some(identifier)) {
            Ok(v) if !v.is_empty() => wire::serialize_promote_read_replica_d_b_cluster_response(
                &wire::PromoteReadReplicaDBClusterResult {
                    d_b_cluster: Some(domain_cluster_to_wire(&v[0])),
                },
            ),
            _ => neptune_error_response(&NeptuneError::NotFound {
                resource_type: "DBCluster".to_string(),
                name: identifier.to_string(),
            }),
        }
    }

    async fn handle_restore_db_cluster_from_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_restore_d_b_cluster_from_snapshot_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.snapshot_identifier.is_empty() {
            return MockResponse::error(400, "InvalidParameterValue", "Missing SnapshotIdentifier");
        }
        if input.d_b_cluster_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBClusterIdentifier",
            );
        }
        let tags = wire_tags_to_domain(input.tags);
        let mut st = state.write().await;
        match st.restore_db_cluster_from_snapshot(
            &input.snapshot_identifier,
            input.d_b_cluster_identifier,
            tags,
            account_id,
            region,
        ) {
            Ok(cluster) => wire::serialize_restore_d_b_cluster_from_snapshot_response(
                &wire::RestoreDBClusterFromSnapshotResult {
                    d_b_cluster: Some(domain_cluster_to_wire(&cluster)),
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_restore_db_cluster_to_point_in_time(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_restore_d_b_cluster_to_point_in_time_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.source_d_b_cluster_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing SourceDBClusterIdentifier",
            );
        }
        if input.d_b_cluster_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBClusterIdentifier",
            );
        }
        let source_id = input.source_d_b_cluster_identifier;
        let target_id = input.d_b_cluster_identifier;
        let tags = wire_tags_to_domain(input.tags);

        let st_read = state.read().await;
        let source_cluster = match st_read.describe_db_clusters(Some(&source_id)) {
            Ok(v) if !v.is_empty() => v[0].clone(),
            _ => {
                return neptune_error_response(&NeptuneError::NotFound {
                    resource_type: "DBCluster".to_string(),
                    name: source_id.clone(),
                });
            }
        };
        drop(st_read);

        let arn = format!("arn:aws:neptune:{region}:{account_id}:cluster:{target_id}");
        let endpoint = Some(format!(
            "{target_id}.cluster-{account_id}.{region}.neptune.amazonaws.com"
        ));
        let reader_endpoint = Some(format!(
            "{target_id}.cluster-ro-{account_id}.{region}.neptune.amazonaws.com"
        ));
        let mut new_cluster = source_cluster;
        new_cluster.identifier = target_id.clone();
        new_cluster.arn = arn;
        new_cluster.endpoint = endpoint;
        new_cluster.reader_endpoint = reader_endpoint;
        new_cluster.status = "available".to_string();
        new_cluster.tags = tags;
        new_cluster.members = Vec::new();
        new_cluster.cluster_create_time =
            Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string());

        let mut st = state.write().await;
        if st.db_clusters.contains_key(&target_id) {
            return neptune_error_response(&NeptuneError::AlreadyExists {
                resource_type: "DBCluster".to_string(),
                name: target_id.clone(),
            });
        }
        st.db_clusters.insert(target_id, new_cluster.clone());
        wire::serialize_restore_d_b_cluster_to_point_in_time_response(
            &wire::RestoreDBClusterToPointInTimeResult {
                d_b_cluster: Some(domain_cluster_to_wire(&new_cluster)),
            },
        )
    }

    // -------------------------------------------------------------------------
    // DB Instances
    // -------------------------------------------------------------------------

    async fn handle_create_db_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_d_b_instance_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_instance_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBInstanceIdentifier",
            );
        }
        let db_instance_class = if input.d_b_instance_class.is_empty() {
            "db.r5.large".to_string()
        } else {
            input.d_b_instance_class
        };
        let engine = if input.engine.is_empty() {
            "neptune".to_string()
        } else {
            input.engine
        };
        let vpc_security_group_ids = input
            .vpc_security_group_ids
            .map(|l| l.items)
            .unwrap_or_default();
        let db_cluster_identifier = if input.d_b_cluster_identifier.is_empty() {
            None
        } else {
            Some(input.d_b_cluster_identifier)
        };
        let db_parameter_group_names = input
            .d_b_parameter_group_name
            .map(|n| vec![n])
            .unwrap_or_default();
        let tags = wire_tags_to_domain(input.tags);

        let mut st = state.write().await;
        match st.create_db_instance(
            input.d_b_instance_identifier,
            db_instance_class,
            engine,
            input.engine_version,
            input.d_b_subnet_group_name,
            vpc_security_group_ids,
            input.availability_zone,
            input.auto_minor_version_upgrade.unwrap_or(true),
            input.backup_retention_period.unwrap_or(1),
            db_cluster_identifier,
            input.storage_encrypted.unwrap_or(false),
            input.kms_key_id,
            input.publicly_accessible.unwrap_or(false),
            input.deletion_protection.unwrap_or(false),
            db_parameter_group_names,
            input.multi_a_z.unwrap_or(false),
            tags,
            account_id,
            region,
        ) {
            Ok(inst) => {
                wire::serialize_create_d_b_instance_response(&wire::CreateDBInstanceResult {
                    d_b_instance: Some(domain_instance_to_wire(&inst)),
                })
            }
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_describe_db_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_d_b_instances_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        match st.describe_db_instances(input.d_b_instance_identifier.as_deref()) {
            Ok(instances) => {
                wire::serialize_describe_d_b_instances_response(&wire::DBInstanceMessage {
                    d_b_instances: Some(wire::DBInstanceList {
                        items: instances.iter().map(domain_instance_to_wire).collect(),
                    }),
                    ..Default::default()
                })
            }
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_delete_db_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_d_b_instance_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_instance_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBInstanceIdentifier",
            );
        }
        let mut st = state.write().await;
        match st.delete_db_instance(&input.d_b_instance_identifier) {
            Ok(inst) => {
                wire::serialize_delete_d_b_instance_response(&wire::DeleteDBInstanceResult {
                    d_b_instance: Some(domain_instance_to_wire(&inst)),
                })
            }
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_modify_db_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_d_b_instance_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_instance_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBInstanceIdentifier",
            );
        }

        let mut st = state.write().await;
        match st.modify_db_instance(
            &input.d_b_instance_identifier,
            input.d_b_instance_class,
            input.engine_version,
            input.deletion_protection,
            input.auto_minor_version_upgrade,
        ) {
            Ok(inst) => {
                wire::serialize_modify_d_b_instance_response(&wire::ModifyDBInstanceResult {
                    d_b_instance: Some(domain_instance_to_wire(&inst)),
                })
            }
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_reboot_db_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_reboot_d_b_instance_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_instance_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBInstanceIdentifier",
            );
        }
        let identifier = &input.d_b_instance_identifier;
        let st = state.read().await;
        match st.describe_db_instances(Some(identifier)) {
            Ok(v) if !v.is_empty() => {
                wire::serialize_reboot_d_b_instance_response(&wire::RebootDBInstanceResult {
                    d_b_instance: Some(domain_instance_to_wire(&v[0])),
                })
            }
            _ => neptune_error_response(&NeptuneError::NotFound {
                resource_type: "DBInstance".to_string(),
                name: identifier.to_string(),
            }),
        }
    }

    // -------------------------------------------------------------------------
    // DB Subnet Groups
    // -------------------------------------------------------------------------

    async fn handle_create_db_subnet_group(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_d_b_subnet_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_subnet_group_name.is_empty() {
            return MockResponse::error(400, "InvalidParameterValue", "Missing DBSubnetGroupName");
        }
        let subnet_ids = input.subnet_ids.items;
        let tags = wire_tags_to_domain(input.tags);
        let mut st = state.write().await;
        match st.create_db_subnet_group(
            input.d_b_subnet_group_name,
            input.d_b_subnet_group_description,
            subnet_ids,
            tags,
            account_id,
            region,
        ) {
            Ok(sg) => {
                wire::serialize_create_d_b_subnet_group_response(&wire::CreateDBSubnetGroupResult {
                    d_b_subnet_group: Some(domain_subnet_group_to_wire(&sg)),
                })
            }
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_describe_db_subnet_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_d_b_subnet_groups_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        match st.describe_db_subnet_groups(input.d_b_subnet_group_name.as_deref()) {
            Ok(groups) => {
                wire::serialize_describe_d_b_subnet_groups_response(&wire::DBSubnetGroupMessage {
                    d_b_subnet_groups: Some(wire::DBSubnetGroups {
                        items: groups.iter().map(domain_subnet_group_to_wire).collect(),
                    }),
                    ..Default::default()
                })
            }
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_delete_db_subnet_group(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_d_b_subnet_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_subnet_group_name.is_empty() {
            return MockResponse::error(400, "InvalidParameterValue", "Missing DBSubnetGroupName");
        }
        let mut st = state.write().await;
        match st.delete_db_subnet_group(&input.d_b_subnet_group_name) {
            Ok(()) => wire::serialize_delete_d_b_subnet_group_response(),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_modify_db_subnet_group(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_d_b_subnet_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_subnet_group_name.is_empty() {
            return MockResponse::error(400, "InvalidParameterValue", "Missing DBSubnetGroupName");
        }
        let subnet_ids = if input.subnet_ids.items.is_empty() {
            None
        } else {
            Some(input.subnet_ids.items)
        };
        let mut st = state.write().await;
        match st.modify_db_subnet_group(
            &input.d_b_subnet_group_name,
            input.d_b_subnet_group_description,
            subnet_ids,
        ) {
            Ok(sg) => {
                wire::serialize_modify_d_b_subnet_group_response(&wire::ModifyDBSubnetGroupResult {
                    d_b_subnet_group: Some(domain_subnet_group_to_wire(&sg)),
                })
            }
            Err(e) => neptune_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // DB Parameter Groups
    // -------------------------------------------------------------------------

    async fn handle_create_db_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_d_b_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_parameter_group_name.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBParameterGroupName",
            );
        }
        let tags = wire_tags_to_domain(input.tags);
        let parameters = parse_parameters_from_params(params);
        let mut st = state.write().await;
        match st.create_db_parameter_group(
            input.d_b_parameter_group_name,
            input.d_b_parameter_group_family,
            input.description,
            parameters,
            tags,
            account_id,
            region,
        ) {
            Ok(pg) => wire::serialize_create_d_b_parameter_group_response(
                &wire::CreateDBParameterGroupResult {
                    d_b_parameter_group: Some(domain_parameter_group_to_wire(&pg)),
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_describe_db_parameter_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_d_b_parameter_groups_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        match st.describe_db_parameter_groups(input.d_b_parameter_group_name.as_deref()) {
            Ok(groups) => wire::serialize_describe_d_b_parameter_groups_response(
                &wire::DBParameterGroupsMessage {
                    d_b_parameter_groups: Some(wire::DBParameterGroupList {
                        items: groups.iter().map(domain_parameter_group_to_wire).collect(),
                    }),
                    ..Default::default()
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_delete_db_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_d_b_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_parameter_group_name.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBParameterGroupName",
            );
        }
        let mut st = state.write().await;
        match st.delete_db_parameter_group(&input.d_b_parameter_group_name) {
            Ok(()) => wire::serialize_delete_d_b_parameter_group_response(),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_modify_db_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_d_b_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_parameter_group_name.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBParameterGroupName",
            );
        }
        let st = state.read().await;
        match st.describe_db_parameter_groups(Some(&input.d_b_parameter_group_name)) {
            Ok(_) => wire::serialize_modify_d_b_parameter_group_response(
                &wire::DBParameterGroupNameMessage {
                    d_b_parameter_group_name: Some(input.d_b_parameter_group_name.clone()),
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_reset_db_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_reset_d_b_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_parameter_group_name.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBParameterGroupName",
            );
        }
        let st = state.read().await;
        match st.describe_db_parameter_groups(Some(&input.d_b_parameter_group_name)) {
            Ok(_) => wire::serialize_reset_d_b_parameter_group_response(
                &wire::DBParameterGroupNameMessage {
                    d_b_parameter_group_name: Some(input.d_b_parameter_group_name.clone()),
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_describe_db_parameters(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_d_b_parameters_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        match st.describe_db_parameter_groups(Some(&input.d_b_parameter_group_name)) {
            Ok(_) => {
                wire::serialize_describe_d_b_parameters_response(&wire::DBParameterGroupDetails {
                    ..Default::default()
                })
            }
            Err(e) => neptune_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // DB Cluster Parameter Groups
    // -------------------------------------------------------------------------

    async fn handle_create_db_cluster_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_d_b_cluster_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_cluster_parameter_group_name.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBClusterParameterGroupName",
            );
        }
        let tags = wire_tags_to_domain(input.tags);
        let parameters = parse_parameters_from_params(params);
        let mut st = state.write().await;
        match st.create_db_cluster_parameter_group(
            input.d_b_cluster_parameter_group_name,
            input.d_b_parameter_group_family,
            input.description,
            parameters,
            tags,
            account_id,
            region,
        ) {
            Ok(pg) => wire::serialize_create_d_b_cluster_parameter_group_response(
                &wire::CreateDBClusterParameterGroupResult {
                    d_b_cluster_parameter_group: Some(domain_cluster_parameter_group_to_wire(&pg)),
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_describe_db_cluster_parameter_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_d_b_cluster_parameter_groups_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        match st
            .describe_db_cluster_parameter_groups(input.d_b_cluster_parameter_group_name.as_deref())
        {
            Ok(groups) => wire::serialize_describe_d_b_cluster_parameter_groups_response(
                &wire::DBClusterParameterGroupsMessage {
                    d_b_cluster_parameter_groups: Some(wire::DBClusterParameterGroupList {
                        items: groups
                            .iter()
                            .map(domain_cluster_parameter_group_to_wire)
                            .collect(),
                    }),
                    ..Default::default()
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_delete_db_cluster_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_d_b_cluster_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_cluster_parameter_group_name.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBClusterParameterGroupName",
            );
        }
        let mut st = state.write().await;
        match st.delete_db_cluster_parameter_group(&input.d_b_cluster_parameter_group_name) {
            Ok(()) => wire::serialize_delete_d_b_cluster_parameter_group_response(),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_modify_db_cluster_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_d_b_cluster_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_cluster_parameter_group_name.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBClusterParameterGroupName",
            );
        }
        let st = state.read().await;
        match st.describe_db_cluster_parameter_groups(Some(&input.d_b_cluster_parameter_group_name))
        {
            Ok(_) => wire::serialize_modify_d_b_cluster_parameter_group_response(
                &wire::DBClusterParameterGroupNameMessage {
                    d_b_cluster_parameter_group_name: Some(
                        input.d_b_cluster_parameter_group_name.clone(),
                    ),
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_reset_db_cluster_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_reset_d_b_cluster_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_cluster_parameter_group_name.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBClusterParameterGroupName",
            );
        }
        let st = state.read().await;
        match st.describe_db_cluster_parameter_groups(Some(&input.d_b_cluster_parameter_group_name))
        {
            Ok(_) => wire::serialize_reset_d_b_cluster_parameter_group_response(
                &wire::DBClusterParameterGroupNameMessage {
                    d_b_cluster_parameter_group_name: Some(
                        input.d_b_cluster_parameter_group_name.clone(),
                    ),
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_describe_db_cluster_parameters(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_d_b_cluster_parameters_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        match st.describe_db_cluster_parameter_groups(Some(&input.d_b_cluster_parameter_group_name))
        {
            Ok(_) => wire::serialize_describe_d_b_cluster_parameters_response(
                &wire::DBClusterParameterGroupDetails {
                    ..Default::default()
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // DB Cluster Snapshots
    // -------------------------------------------------------------------------

    async fn handle_create_db_cluster_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_d_b_cluster_snapshot_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_cluster_snapshot_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBClusterSnapshotIdentifier",
            );
        }
        if input.d_b_cluster_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBClusterIdentifier",
            );
        }
        let tags = wire_tags_to_domain(input.tags);
        let mut st = state.write().await;
        match st.create_db_cluster_snapshot(
            input.d_b_cluster_snapshot_identifier,
            &input.d_b_cluster_identifier,
            tags,
            account_id,
            region,
        ) {
            Ok(snap) => wire::serialize_create_d_b_cluster_snapshot_response(
                &wire::CreateDBClusterSnapshotResult {
                    d_b_cluster_snapshot: Some(domain_snapshot_to_wire(&snap)),
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_describe_db_cluster_snapshots(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_d_b_cluster_snapshots_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        let snaps = st.describe_db_cluster_snapshots(
            input.d_b_cluster_snapshot_identifier.as_deref(),
            input.d_b_cluster_identifier.as_deref(),
        );
        wire::serialize_describe_d_b_cluster_snapshots_response(&wire::DBClusterSnapshotMessage {
            d_b_cluster_snapshots: Some(wire::DBClusterSnapshotList {
                items: snaps.iter().map(domain_snapshot_to_wire).collect(),
            }),
            ..Default::default()
        })
    }

    async fn handle_delete_db_cluster_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_d_b_cluster_snapshot_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_cluster_snapshot_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBClusterSnapshotIdentifier",
            );
        }
        let mut st = state.write().await;
        match st.delete_db_cluster_snapshot(&input.d_b_cluster_snapshot_identifier) {
            Ok(snap) => wire::serialize_delete_d_b_cluster_snapshot_response(
                &wire::DeleteDBClusterSnapshotResult {
                    d_b_cluster_snapshot: Some(domain_snapshot_to_wire(&snap)),
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Tags
    // -------------------------------------------------------------------------

    async fn handle_add_tags_to_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_add_tags_to_resource_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.resource_name.is_empty() {
            return MockResponse::error(400, "InvalidParameterValue", "Missing ResourceName");
        }
        let tags = wire_tags_to_domain(Some(input.tags));
        let mut st = state.write().await;
        st.add_tags_to_resource(&input.resource_name, tags);
        wire::serialize_add_tags_to_resource_response()
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.resource_name.is_empty() {
            return MockResponse::error(400, "InvalidParameterValue", "Missing ResourceName");
        }
        let st = state.read().await;
        let tags = st.list_tags_for_resource(&input.resource_name);
        wire::serialize_list_tags_for_resource_response(&wire::TagListMessage {
            tag_list: Some(domain_tags_to_wire(&tags)),
        })
    }

    async fn handle_remove_tags_from_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_remove_tags_from_resource_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.resource_name.is_empty() {
            return MockResponse::error(400, "InvalidParameterValue", "Missing ResourceName");
        }
        let mut st = state.write().await;
        st.remove_tags_from_resource(&input.resource_name, &input.tag_keys.items);
        wire::serialize_remove_tags_from_resource_response()
    }

    // -------------------------------------------------------------------------
    // Engine Versions / Orderable Options
    // -------------------------------------------------------------------------

    async fn handle_describe_db_engine_versions(
        &self,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_d_b_engine_versions_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let engine = input.engine.unwrap_or_else(|| "neptune".to_string());
        let engine_version = input
            .engine_version
            .unwrap_or_else(|| "1.2.1.0".to_string());
        wire::serialize_describe_d_b_engine_versions_response(&wire::DBEngineVersionMessage {
            d_b_engine_versions: Some(wire::DBEngineVersionList {
                items: vec![wire::DBEngineVersion {
                    engine: Some(engine.clone()),
                    engine_version: Some(engine_version.clone()),
                    d_b_engine_description: Some("Neptune graph database".to_string()),
                    d_b_engine_version_description: Some(format!("Neptune {engine_version}")),
                    ..Default::default()
                }],
            }),
            ..Default::default()
        })
    }

    async fn handle_describe_orderable_db_instance_options(
        &self,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_d_b_engine_versions_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let engine = input.engine.unwrap_or_else(|| "neptune".to_string());
        let engine_version = input
            .engine_version
            .unwrap_or_else(|| "1.2.1.0".to_string());
        wire::serialize_describe_orderable_d_b_instance_options_response(
            &wire::OrderableDBInstanceOptionsMessage {
                orderable_d_b_instance_options: Some(wire::OrderableDBInstanceOptionsList {
                    items: vec![wire::OrderableDBInstanceOption {
                        engine: Some(engine),
                        engine_version: Some(engine_version),
                        d_b_instance_class: Some("db.r5.large".to_string()),
                        license_model: Some("amazon-license".to_string()),
                        multi_a_z_capable: Some(true),
                        read_replica_capable: Some(false),
                        ..Default::default()
                    }],
                }),
                ..Default::default()
            },
        )
    }

    // -------------------------------------------------------------------------
    // Misc stubs
    // -------------------------------------------------------------------------

    async fn handle_describe_db_cluster_snapshot_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let snapshot_id = params
            .get("DBClusterSnapshotIdentifier")
            .cloned()
            .unwrap_or_default();
        let st = state.read().await;
        match st.describe_db_cluster_snapshot_attributes(&snapshot_id) {
            Ok(attrs) => wire::serialize_describe_d_b_cluster_snapshot_attributes_response(
                &wire::DescribeDBClusterSnapshotAttributesResult {
                    d_b_cluster_snapshot_attributes_result: Some(
                        domain_snapshot_attributes_to_wire(&snapshot_id, &attrs),
                    ),
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_modify_db_cluster_snapshot_attribute(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_d_b_cluster_snapshot_attribute_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let snapshot_id = input.d_b_cluster_snapshot_identifier;
        let attribute_name = input.attribute_name;
        let values_to_add = input.values_to_add.map(|l| l.items).unwrap_or_default();
        let values_to_remove = input.values_to_remove.map(|l| l.items).unwrap_or_default();
        let mut st = state.write().await;
        match st.modify_db_cluster_snapshot_attribute(
            &snapshot_id,
            &attribute_name,
            values_to_add,
            values_to_remove,
        ) {
            Ok(attrs) => wire::serialize_modify_d_b_cluster_snapshot_attribute_response(
                &wire::ModifyDBClusterSnapshotAttributeResult {
                    d_b_cluster_snapshot_attributes_result: Some(
                        domain_snapshot_attributes_to_wire(&snapshot_id, &attrs),
                    ),
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    // Engine default parameters are static metadata; returns empty parameter list.
    async fn handle_describe_engine_default_cluster_parameters(
        &self,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let family = params
            .get("DBParameterGroupFamily")
            .cloned()
            .unwrap_or_else(|| "neptune1".to_string());
        wire::serialize_describe_engine_default_cluster_parameters_response(
            &wire::DescribeEngineDefaultClusterParametersResult {
                engine_defaults: Some(wire::EngineDefaults {
                    d_b_parameter_group_family: Some(family),
                    ..Default::default()
                }),
            },
        )
    }

    // Engine default parameters are static metadata; returns empty parameter list.
    async fn handle_describe_engine_default_parameters(
        &self,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let family = params
            .get("DBParameterGroupFamily")
            .cloned()
            .unwrap_or_else(|| "neptune1".to_string());
        wire::serialize_describe_engine_default_parameters_response(
            &wire::DescribeEngineDefaultParametersResult {
                engine_defaults: Some(wire::EngineDefaults {
                    d_b_parameter_group_family: Some(family),
                    ..Default::default()
                }),
            },
        )
    }

    // STUB[no-telemetry]: Event category listings are driven by real infrastructure telemetry; returns empty list.
    async fn handle_describe_event_categories(&self) -> MockResponse {
        wire::serialize_describe_event_categories_response(&wire::EventCategoriesMessage {
            ..Default::default()
        })
    }

    // STUB[no-telemetry]: Events are driven by real infrastructure activity; returns empty list.
    async fn handle_describe_events(&self) -> MockResponse {
        wire::serialize_describe_events_response(&wire::EventsMessage {
            ..Default::default()
        })
    }

    // STUB[no-telemetry]: Pending maintenance actions are driven by real infrastructure state; returns empty list.
    async fn handle_describe_pending_maintenance_actions(&self) -> MockResponse {
        wire::serialize_describe_pending_maintenance_actions_response(
            &wire::PendingMaintenanceActionsMessage {
                ..Default::default()
            },
        )
    }

    // Valid modification options depend on real instance metadata; returns empty modifications message.
    async fn handle_describe_valid_db_instance_modifications(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_describe_valid_d_b_instance_modifications_request(params) {
                Ok(v) => v,
                Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
            };
        if input.d_b_instance_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBInstanceIdentifier",
            );
        }
        let st = state.read().await;
        match st.describe_db_instances(Some(&input.d_b_instance_identifier)) {
            Ok(_) => wire::serialize_describe_valid_d_b_instance_modifications_response(
                &wire::DescribeValidDBInstanceModificationsResult {
                    valid_d_b_instance_modifications_message: Some(
                        wire::ValidDBInstanceModificationsMessage {
                            ..Default::default()
                        },
                    ),
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_create_global_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_global_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let engine = input.engine.unwrap_or_else(|| "neptune".to_string());
        let mut st = state.write().await;
        match st.create_global_cluster(
            input.global_cluster_identifier,
            engine,
            input.engine_version,
            input.database_name,
            input.deletion_protection.unwrap_or(false),
            input.storage_encrypted.unwrap_or(false),
            input.source_d_b_cluster_identifier,
            account_id,
            region,
        ) {
            Ok(gc) => {
                wire::serialize_create_global_cluster_response(&wire::CreateGlobalClusterResult {
                    global_cluster: Some(domain_global_cluster_to_wire(&gc)),
                })
            }
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_describe_global_clusters(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_global_clusters_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        match st.describe_global_clusters(input.global_cluster_identifier.as_deref()) {
            Ok(clusters) => {
                wire::serialize_describe_global_clusters_response(&wire::GlobalClustersMessage {
                    global_clusters: Some(wire::GlobalClusterList {
                        items: clusters.iter().map(domain_global_cluster_to_wire).collect(),
                    }),
                    ..Default::default()
                })
            }
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_delete_global_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_global_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let mut st = state.write().await;
        match st.delete_global_cluster(&input.global_cluster_identifier) {
            Ok(gc) => {
                wire::serialize_delete_global_cluster_response(&wire::DeleteGlobalClusterResult {
                    global_cluster: Some(domain_global_cluster_to_wire(&gc)),
                })
            }
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_modify_global_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_global_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let mut st = state.write().await;
        match st.modify_global_cluster(
            &input.global_cluster_identifier,
            input.engine_version,
            input.deletion_protection,
            input.new_global_cluster_identifier,
        ) {
            Ok(gc) => {
                wire::serialize_modify_global_cluster_response(&wire::ModifyGlobalClusterResult {
                    global_cluster: Some(domain_global_cluster_to_wire(&gc)),
                })
            }
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_failover_global_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_failover_global_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        match st.failover_global_cluster(&input.global_cluster_identifier) {
            Ok(gc) => wire::serialize_failover_global_cluster_response(
                &wire::FailoverGlobalClusterResult {
                    global_cluster: Some(domain_global_cluster_to_wire(&gc)),
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_switchover_global_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_switchover_global_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        // Switchover is same as failover for mock purposes
        let st = state.read().await;
        match st.failover_global_cluster(&input.global_cluster_identifier) {
            Ok(gc) => wire::serialize_switchover_global_cluster_response(
                &wire::SwitchoverGlobalClusterResult {
                    global_cluster: Some(domain_global_cluster_to_wire(&gc)),
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_remove_from_global_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_remove_from_global_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        match st.remove_from_global_cluster(&input.global_cluster_identifier) {
            Ok(gc) => wire::serialize_remove_from_global_cluster_response(
                &wire::RemoveFromGlobalClusterResult {
                    global_cluster: Some(domain_global_cluster_to_wire(&gc)),
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_add_role_to_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_add_role_to_d_b_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_cluster_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBClusterIdentifier",
            );
        }
        let identifier = input.d_b_cluster_identifier;
        let role_arn = input.role_arn;
        let mut st = state.write().await;
        if let Some(cluster) = st.db_clusters.get_mut(&identifier) {
            if !cluster.associated_roles.contains(&role_arn) {
                cluster.associated_roles.push(role_arn);
            }
            wire::serialize_add_role_to_d_b_cluster_response()
        } else {
            neptune_error_response(&NeptuneError::NotFound {
                resource_type: "DBCluster".to_string(),
                name: identifier.clone(),
            })
        }
    }

    async fn handle_remove_role_from_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_remove_role_from_d_b_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_cluster_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBClusterIdentifier",
            );
        }
        let identifier = input.d_b_cluster_identifier;
        let role_arn = input.role_arn;
        let mut st = state.write().await;
        if let Some(cluster) = st.db_clusters.get_mut(&identifier) {
            cluster.associated_roles.retain(|r| *r != role_arn);
            wire::serialize_remove_role_from_d_b_cluster_response()
        } else {
            neptune_error_response(&NeptuneError::NotFound {
                resource_type: "DBCluster".to_string(),
                name: identifier.clone(),
            })
        }
    }

    async fn handle_create_db_cluster_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_d_b_cluster_endpoint_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_cluster_endpoint_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBClusterEndpointIdentifier",
            );
        }
        let static_members = input.static_members.map(|l| l.items).unwrap_or_default();
        let excluded_members = input.excluded_members.map(|l| l.items).unwrap_or_default();
        let mut st = state.write().await;
        match st.create_db_cluster_endpoint(
            input.d_b_cluster_endpoint_identifier,
            input.d_b_cluster_identifier,
            input.endpoint_type,
            static_members,
            excluded_members,
            account_id,
            region,
        ) {
            Ok(ep) => wire::serialize_create_d_b_cluster_endpoint_response(
                &domain_endpoint_to_wire_create(&ep),
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_delete_db_cluster_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_d_b_cluster_endpoint_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_cluster_endpoint_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBClusterEndpointIdentifier",
            );
        }
        let mut st = state.write().await;
        match st.delete_db_cluster_endpoint(&input.d_b_cluster_endpoint_identifier) {
            Ok(ep) => wire::serialize_delete_d_b_cluster_endpoint_response(
                &domain_endpoint_to_wire_delete(&ep),
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_describe_db_cluster_endpoints(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_d_b_cluster_endpoints_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        let eps = st.describe_db_cluster_endpoints(
            input.d_b_cluster_endpoint_identifier.as_deref(),
            input.d_b_cluster_identifier.as_deref(),
        );
        wire::serialize_describe_d_b_cluster_endpoints_response(&wire::DBClusterEndpointMessage {
            d_b_cluster_endpoints: Some(wire::DBClusterEndpointList {
                items: eps.iter().map(domain_endpoint_to_wire_describe).collect(),
            }),
            ..Default::default()
        })
    }

    async fn handle_modify_db_cluster_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_d_b_cluster_endpoint_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.d_b_cluster_endpoint_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBClusterEndpointIdentifier",
            );
        }
        let static_members = input.static_members.map(|l| l.items);
        let excluded_members = input.excluded_members.map(|l| l.items);
        let mut st = state.write().await;
        match st.modify_db_cluster_endpoint(
            &input.d_b_cluster_endpoint_identifier,
            input.endpoint_type,
            static_members,
            excluded_members,
        ) {
            Ok(ep) => wire::serialize_modify_d_b_cluster_endpoint_response(
                &domain_endpoint_to_wire_modify(&ep),
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_copy_db_cluster_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_copy_d_b_cluster_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input
            .target_d_b_cluster_parameter_group_identifier
            .is_empty()
        {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing TargetDBClusterParameterGroupIdentifier",
            );
        }
        let source = input.source_d_b_cluster_parameter_group_identifier;
        let target = input.target_d_b_cluster_parameter_group_identifier;
        let description = input.target_d_b_cluster_parameter_group_description;
        let tags = wire_tags_to_domain(input.tags);
        let st_read = state.read().await;
        let family = st_read
            .db_cluster_parameter_groups
            .get(&source)
            .map(|pg| pg.family.clone())
            .unwrap_or_default();
        let parameters = st_read
            .db_cluster_parameter_groups
            .get(&source)
            .map(|pg| pg.parameters.clone())
            .unwrap_or_default();
        drop(st_read);
        let mut st = state.write().await;
        match st.create_db_cluster_parameter_group(
            target,
            family,
            description,
            parameters,
            tags,
            account_id,
            region,
        ) {
            Ok(pg) => wire::serialize_copy_d_b_cluster_parameter_group_response(
                &wire::CopyDBClusterParameterGroupResult {
                    d_b_cluster_parameter_group: Some(domain_cluster_parameter_group_to_wire(&pg)),
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_copy_db_cluster_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_copy_d_b_cluster_snapshot_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.source_d_b_cluster_snapshot_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing SourceDBClusterSnapshotIdentifier",
            );
        }
        if input.target_d_b_cluster_snapshot_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing TargetDBClusterSnapshotIdentifier",
            );
        }
        let source_id = input.source_d_b_cluster_snapshot_identifier;
        let target_id = input.target_d_b_cluster_snapshot_identifier;
        let tags = wire_tags_to_domain(input.tags);
        let st_read = state.read().await;
        let source = match st_read.db_cluster_snapshots.get(&source_id) {
            Some(s) => s.clone(),
            None => {
                return neptune_error_response(&NeptuneError::NotFound {
                    resource_type: "DBClusterSnapshot".to_string(),
                    name: source_id.clone(),
                });
            }
        };
        drop(st_read);
        let arn = format!("arn:aws:neptune:{region}:{account_id}:cluster-snapshot:{target_id}");
        let snap = crate::types::DbClusterSnapshot {
            identifier: target_id.clone(),
            db_cluster_snapshot_arn: arn,
            tags: tags.clone(),
            snapshot_create_time: Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()),
            ..source
        };
        let mut st = state.write().await;
        if st.db_cluster_snapshots.contains_key(&target_id) {
            return neptune_error_response(&NeptuneError::AlreadyExists {
                resource_type: "DBClusterSnapshot".to_string(),
                name: target_id.clone(),
            });
        }
        st.db_cluster_snapshots.insert(target_id, snap.clone());
        wire::serialize_copy_d_b_cluster_snapshot_response(&wire::CopyDBClusterSnapshotResult {
            d_b_cluster_snapshot: Some(domain_snapshot_to_wire(&snap)),
        })
    }

    async fn handle_copy_db_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_copy_d_b_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.target_d_b_parameter_group_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing TargetDBParameterGroupIdentifier",
            );
        }
        let source = input.source_d_b_parameter_group_identifier;
        let target = input.target_d_b_parameter_group_identifier;
        let description = input.target_d_b_parameter_group_description;
        let tags = wire_tags_to_domain(input.tags);
        let st_read = state.read().await;
        let family = st_read
            .db_parameter_groups
            .get(&source)
            .map(|pg| pg.family.clone())
            .unwrap_or_default();
        let parameters = st_read
            .db_parameter_groups
            .get(&source)
            .map(|pg| pg.parameters.clone())
            .unwrap_or_default();
        drop(st_read);
        let mut st = state.write().await;
        match st.create_db_parameter_group(
            target,
            family,
            description,
            parameters,
            tags,
            account_id,
            region,
        ) {
            Ok(pg) => wire::serialize_copy_d_b_parameter_group_response(
                &wire::CopyDBParameterGroupResult {
                    d_b_parameter_group: Some(domain_parameter_group_to_wire(&pg)),
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_create_event_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_event_subscription_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.subscription_name.is_empty() {
            return MockResponse::error(400, "InvalidParameterValue", "Missing SubscriptionName");
        }
        let event_categories = input.event_categories.map(|l| l.items).unwrap_or_default();
        let source_ids = input.source_ids.map(|l| l.items).unwrap_or_default();
        let mut st = state.write().await;
        match st.create_event_subscription(
            input.subscription_name,
            input.sns_topic_arn,
            input.source_type,
            input.enabled.unwrap_or(true),
            event_categories,
            source_ids,
            account_id,
            region,
        ) {
            Ok(sub) => wire::serialize_create_event_subscription_response(
                &wire::CreateEventSubscriptionResult {
                    event_subscription: Some(domain_event_subscription_to_wire(&sub)),
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_delete_event_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_event_subscription_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.subscription_name.is_empty() {
            return MockResponse::error(400, "InvalidParameterValue", "Missing SubscriptionName");
        }
        let mut st = state.write().await;
        match st.delete_event_subscription(&input.subscription_name) {
            Ok(sub) => wire::serialize_delete_event_subscription_response(
                &wire::DeleteEventSubscriptionResult {
                    event_subscription: Some(domain_event_subscription_to_wire(&sub)),
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_describe_event_subscriptions(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_event_subscriptions_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        match st.describe_event_subscriptions(input.subscription_name.as_deref()) {
            Ok(subs) => wire::serialize_describe_event_subscriptions_response(
                &wire::EventSubscriptionsMessage {
                    event_subscriptions_list: Some(wire::EventSubscriptionsList {
                        items: subs.iter().map(domain_event_subscription_to_wire).collect(),
                    }),
                    ..Default::default()
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_modify_event_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_event_subscription_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.subscription_name.is_empty() {
            return MockResponse::error(400, "InvalidParameterValue", "Missing SubscriptionName");
        }
        let event_categories = input.event_categories.map(|l| l.items);
        let mut st = state.write().await;
        match st.modify_event_subscription(
            &input.subscription_name,
            input.sns_topic_arn,
            input.source_type,
            input.enabled,
            event_categories,
        ) {
            Ok(sub) => wire::serialize_modify_event_subscription_response(
                &wire::ModifyEventSubscriptionResult {
                    event_subscription: Some(domain_event_subscription_to_wire(&sub)),
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_add_source_identifier_to_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_add_source_identifier_to_subscription_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.subscription_name.is_empty() {
            return MockResponse::error(400, "InvalidParameterValue", "Missing SubscriptionName");
        }
        let mut st = state.write().await;
        match st.add_source_identifier_to_subscription(
            &input.subscription_name,
            &input.source_identifier,
        ) {
            Ok(sub) => wire::serialize_add_source_identifier_to_subscription_response(
                &wire::AddSourceIdentifierToSubscriptionResult {
                    event_subscription: Some(domain_event_subscription_to_wire(&sub)),
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    async fn handle_remove_source_identifier_from_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<NeptuneState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_remove_source_identifier_from_subscription_request(params) {
                Ok(v) => v,
                Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
            };
        if input.subscription_name.is_empty() {
            return MockResponse::error(400, "InvalidParameterValue", "Missing SubscriptionName");
        }
        let mut st = state.write().await;
        match st.remove_source_identifier_from_subscription(
            &input.subscription_name,
            &input.source_identifier,
        ) {
            Ok(sub) => wire::serialize_remove_source_identifier_from_subscription_response(
                &wire::RemoveSourceIdentifierFromSubscriptionResult {
                    event_subscription: Some(domain_event_subscription_to_wire(&sub)),
                },
            ),
            Err(e) => neptune_error_response(&e),
        }
    }

    // STUB[no-telemetry]: Pending maintenance actions are driven by real infrastructure state; returns empty result.
    async fn handle_apply_pending_maintenance_action(&self) -> MockResponse {
        wire::serialize_apply_pending_maintenance_action_response(
            &wire::ApplyPendingMaintenanceActionResult {
                ..Default::default()
            },
        )
    }
}

// -------------------------------------------------------------------------
// Wire-to-domain conversions
// -------------------------------------------------------------------------

/// Parse `ServerlessV2ScalingConfiguration.MinCapacity` / `MaxCapacity` from
/// the form-encoded parameters.
fn parse_serverless_v2_scaling_config(
    params: &HashMap<String, String>,
) -> Option<ServerlessV2ScalingConfiguration> {
    let min = params
        .get("ServerlessV2ScalingConfiguration.MinCapacity")
        .and_then(|v| v.parse::<f64>().ok());
    let max = params
        .get("ServerlessV2ScalingConfiguration.MaxCapacity")
        .and_then(|v| v.parse::<f64>().ok());
    match (min, max) {
        (Some(min_capacity), Some(max_capacity)) => Some(ServerlessV2ScalingConfiguration {
            min_capacity,
            max_capacity,
        }),
        _ => None,
    }
}

/// Parse `Parameters.member.N.ParameterName`, `Parameters.member.N.ParameterValue`,
/// and `Parameters.member.N.ApplyMethod` from form-encoded parameters.
fn parse_parameters_from_params(params: &HashMap<String, String>) -> Vec<Parameter> {
    let mut result = Vec::new();
    for i in 1.. {
        let name_key = format!("Parameters.member.{i}.ParameterName");
        let Some(name) = params.get(&name_key) else {
            break;
        };
        let value_key = format!("Parameters.member.{i}.ParameterValue");
        let value = params.get(&value_key).cloned().unwrap_or_default();
        let method_key = format!("Parameters.member.{i}.ApplyMethod");
        let apply_method = params.get(&method_key).cloned();
        result.push(Parameter {
            name: name.clone(),
            value,
            apply_method,
        });
    }
    result
}

fn wire_tags_to_domain(tags: Option<wire::TagList>) -> Vec<Tag> {
    tags.map(|tl| {
        tl.items
            .into_iter()
            .map(|t| Tag {
                key: t.key.unwrap_or_default(),
                value: t.value.unwrap_or_default(),
            })
            .collect()
    })
    .unwrap_or_default()
}

// -------------------------------------------------------------------------
// Error Response helper
// -------------------------------------------------------------------------

fn neptune_error_response(e: &NeptuneError) -> MockResponse {
    let (status, error_type) = match e {
        NeptuneError::AlreadyExists { resource_type, .. } => {
            (400u16, format!("{resource_type}AlreadyExists"))
        }
        NeptuneError::NotFound { resource_type, .. } => {
            (404u16, format!("{resource_type}NotFound"))
        }
        NeptuneError::InvalidParameter { .. } => (400u16, "InvalidParameterValue".to_string()),
    };
    MockResponse::error(status, &error_type, &e.to_string())
}

// -------------------------------------------------------------------------
// Domain-to-wire model conversions
// -------------------------------------------------------------------------

fn domain_tags_to_wire(tags: &[Tag]) -> wire::TagList {
    wire::TagList {
        items: tags
            .iter()
            .map(|t| wire::Tag {
                key: Some(t.key.clone()),
                value: Some(t.value.clone()),
            })
            .collect(),
    }
}

fn domain_cluster_to_wire(c: &crate::types::DbCluster) -> wire::DBCluster {
    wire::DBCluster {
        d_b_cluster_identifier: Some(c.identifier.clone()),
        engine: Some(c.engine.clone()),
        engine_version: c.engine_version.clone(),
        status: Some(c.status.clone()),
        endpoint: c.endpoint.clone(),
        reader_endpoint: c.reader_endpoint.clone(),
        port: c.port,
        master_username: c.master_username.clone(),
        database_name: c.database_name.clone(),
        d_b_subnet_group: c.db_subnet_group_name.clone(),
        vpc_security_groups: Some(wire::VpcSecurityGroupMembershipList {
            items: c
                .vpc_security_group_ids
                .iter()
                .map(|id| wire::VpcSecurityGroupMembership {
                    vpc_security_group_id: Some(id.clone()),
                    status: Some("active".to_string()),
                })
                .collect(),
        }),
        availability_zones: Some(wire::AvailabilityZones::from(c.availability_zones.clone())),
        d_b_cluster_arn: Some(c.arn.clone()),
        cluster_create_time: c.cluster_create_time.clone(),
        multi_a_z: Some(c.multi_az),
        backup_retention_period: Some(c.backup_retention_period),
        deletion_protection: Some(c.deletion_protection),
        storage_encrypted: Some(c.storage_encrypted),
        kms_key_id: c.kms_key_id.clone(),
        d_b_cluster_parameter_group: c.db_cluster_parameter_group.clone(),
        copy_tags_to_snapshot: Some(c.copy_tags_to_snapshot),
        d_b_cluster_members: Some(wire::DBClusterMemberList {
            items: c
                .members
                .iter()
                .map(|m| wire::DBClusterMember {
                    d_b_instance_identifier: Some(m.db_instance_identifier.clone()),
                    is_cluster_writer: Some(m.is_cluster_writer),
                    d_b_cluster_parameter_group_status: Some(
                        m.db_cluster_parameter_group_status.clone(),
                    ),
                    promotion_tier: m.promotion_tier,
                })
                .collect(),
        }),
        associated_roles: Some(wire::DBClusterRoles {
            items: c
                .associated_roles
                .iter()
                .map(|r| wire::DBClusterRole {
                    role_arn: Some(r.clone()),
                    status: Some("ACTIVE".to_string()),
                    ..Default::default()
                })
                .collect(),
        }),
        serverless_v2_scaling_configuration: c.serverless_v2_scaling_configuration.as_ref().map(
            |s| wire::ServerlessV2ScalingConfigurationInfo {
                min_capacity: Some(s.min_capacity),
                max_capacity: Some(s.max_capacity),
            },
        ),
        ..Default::default()
    }
}

fn domain_instance_to_wire(i: &crate::types::DbInstance) -> wire::DBInstance {
    wire::DBInstance {
        d_b_instance_identifier: Some(i.identifier.clone()),
        d_b_instance_class: Some(i.db_instance_class.clone()),
        engine: Some(i.engine.clone()),
        engine_version: Some(i.engine_version.clone()),
        d_b_instance_status: Some(i.status.clone()),
        endpoint: Some(wire::Endpoint {
            address: Some(i.endpoint_address.as_deref().unwrap_or("").to_string()),
            port: Some(i.port.unwrap_or(8182)),
            ..Default::default()
        }),
        d_b_subnet_group: i
            .db_subnet_group_name
            .as_ref()
            .map(|name| wire::DBSubnetGroup {
                d_b_subnet_group_name: Some(name.clone()),
                ..Default::default()
            }),
        vpc_security_groups: Some(wire::VpcSecurityGroupMembershipList {
            items: i
                .vpc_security_group_ids
                .iter()
                .map(|id| wire::VpcSecurityGroupMembership {
                    vpc_security_group_id: Some(id.clone()),
                    status: Some("active".to_string()),
                })
                .collect(),
        }),
        d_b_parameter_groups: Some(wire::DBParameterGroupStatusList {
            items: i
                .db_parameter_group_names
                .iter()
                .map(|n| wire::DBParameterGroupStatus {
                    d_b_parameter_group_name: Some(n.clone()),
                    parameter_apply_status: Some("in-sync".to_string()),
                })
                .collect(),
        }),
        availability_zone: i.availability_zone.clone(),
        auto_minor_version_upgrade: Some(i.auto_minor_version_upgrade),
        backup_retention_period: Some(i.backup_retention_period),
        d_b_cluster_identifier: i.db_cluster_identifier.clone(),
        d_b_instance_arn: Some(i.arn.clone()),
        instance_create_time: i.instance_create_time.clone(),
        storage_encrypted: Some(i.storage_encrypted),
        kms_key_id: i.kms_key_id.clone(),
        publicly_accessible: Some(i.publicly_accessible),
        deletion_protection: Some(i.deletion_protection),
        multi_a_z: Some(i.multi_az),
        ..Default::default()
    }
}

fn domain_subnet_group_to_wire(sg: &crate::types::DbSubnetGroup) -> wire::DBSubnetGroup {
    wire::DBSubnetGroup {
        d_b_subnet_group_name: Some(sg.name.clone()),
        d_b_subnet_group_description: Some(sg.description.clone()),
        vpc_id: sg.vpc_id.clone(),
        subnet_group_status: Some(sg.status.clone()),
        subnets: Some(wire::SubnetList {
            items: sg
                .subnet_ids
                .iter()
                .map(|id| wire::Subnet {
                    subnet_identifier: Some(id.clone()),
                    subnet_status: Some("Active".to_string()),
                    ..Default::default()
                })
                .collect(),
        }),
        d_b_subnet_group_arn: Some(sg.arn.clone()),
    }
}

fn domain_parameter_group_to_wire(pg: &crate::types::DbParameterGroup) -> wire::DBParameterGroup {
    wire::DBParameterGroup {
        d_b_parameter_group_name: Some(pg.name.clone()),
        d_b_parameter_group_family: Some(pg.family.clone()),
        description: Some(pg.description.clone()),
        d_b_parameter_group_arn: Some(pg.arn.clone()),
    }
}

fn domain_cluster_parameter_group_to_wire(
    pg: &crate::types::DbClusterParameterGroup,
) -> wire::DBClusterParameterGroup {
    wire::DBClusterParameterGroup {
        d_b_cluster_parameter_group_name: Some(pg.name.clone()),
        d_b_parameter_group_family: Some(pg.family.clone()),
        description: Some(pg.description.clone()),
        d_b_cluster_parameter_group_arn: Some(pg.arn.clone()),
    }
}

fn domain_snapshot_to_wire(s: &crate::types::DbClusterSnapshot) -> wire::DBClusterSnapshot {
    wire::DBClusterSnapshot {
        d_b_cluster_snapshot_identifier: Some(s.identifier.clone()),
        d_b_cluster_identifier: Some(s.db_cluster_identifier.clone()),
        engine: Some(s.engine.clone()),
        engine_version: s.engine_version.clone(),
        allocated_storage: Some(s.allocated_storage),
        status: Some(s.status.clone()),
        port: s.port,
        vpc_id: s.vpc_id.clone(),
        cluster_create_time: s.cluster_create_time.clone(),
        master_username: s.master_username.clone(),
        snapshot_type: Some(s.snapshot_type.clone()),
        percent_progress: Some(s.percent_progress),
        storage_encrypted: Some(s.storage_encrypted),
        kms_key_id: s.kms_key_id.clone(),
        d_b_cluster_snapshot_arn: Some(s.db_cluster_snapshot_arn.clone()),
        availability_zones: Some(wire::AvailabilityZones::from(s.availability_zones.clone())),
        snapshot_create_time: s.snapshot_create_time.clone(),
        ..Default::default()
    }
}

fn domain_global_cluster_to_wire(gc: &crate::types::GlobalCluster) -> wire::GlobalCluster {
    wire::GlobalCluster {
        global_cluster_identifier: Some(gc.identifier.clone()),
        engine: Some(gc.engine.clone()),
        engine_version: gc.engine_version.clone(),
        database_name: gc.database_name.clone(),
        global_cluster_arn: Some(gc.arn.clone()),
        status: Some(gc.status.clone()),
        deletion_protection: Some(gc.deletion_protection),
        storage_encrypted: Some(gc.storage_encrypted),
        ..Default::default()
    }
}

fn domain_endpoint_to_wire_create(
    ep: &crate::types::DbClusterEndpoint,
) -> wire::CreateDBClusterEndpointOutput {
    wire::CreateDBClusterEndpointOutput {
        d_b_cluster_endpoint_identifier: Some(ep.identifier.clone()),
        d_b_cluster_identifier: Some(ep.db_cluster_identifier.clone()),
        endpoint_type: Some(ep.endpoint_type.clone()),
        custom_endpoint_type: ep.custom_endpoint_type.clone(),
        endpoint: ep.endpoint.clone(),
        d_b_cluster_endpoint_arn: Some(ep.arn.clone()),
        d_b_cluster_endpoint_resource_identifier: Some(ep.resource_identifier.clone()),
        status: Some(ep.status.clone()),
        static_members: Some(wire::StringList::from(ep.static_members.clone())),
        excluded_members: Some(wire::StringList::from(ep.excluded_members.clone())),
    }
}

fn domain_endpoint_to_wire_delete(
    ep: &crate::types::DbClusterEndpoint,
) -> wire::DeleteDBClusterEndpointOutput {
    wire::DeleteDBClusterEndpointOutput {
        d_b_cluster_endpoint_identifier: Some(ep.identifier.clone()),
        d_b_cluster_identifier: Some(ep.db_cluster_identifier.clone()),
        endpoint_type: Some(ep.endpoint_type.clone()),
        custom_endpoint_type: ep.custom_endpoint_type.clone(),
        endpoint: ep.endpoint.clone(),
        d_b_cluster_endpoint_arn: Some(ep.arn.clone()),
        d_b_cluster_endpoint_resource_identifier: Some(ep.resource_identifier.clone()),
        status: Some(ep.status.clone()),
        static_members: Some(wire::StringList::from(ep.static_members.clone())),
        excluded_members: Some(wire::StringList::from(ep.excluded_members.clone())),
    }
}

fn domain_endpoint_to_wire_modify(
    ep: &crate::types::DbClusterEndpoint,
) -> wire::ModifyDBClusterEndpointOutput {
    wire::ModifyDBClusterEndpointOutput {
        d_b_cluster_endpoint_identifier: Some(ep.identifier.clone()),
        d_b_cluster_identifier: Some(ep.db_cluster_identifier.clone()),
        endpoint_type: Some(ep.endpoint_type.clone()),
        custom_endpoint_type: ep.custom_endpoint_type.clone(),
        endpoint: ep.endpoint.clone(),
        d_b_cluster_endpoint_arn: Some(ep.arn.clone()),
        d_b_cluster_endpoint_resource_identifier: Some(ep.resource_identifier.clone()),
        status: Some(ep.status.clone()),
        static_members: Some(wire::StringList::from(ep.static_members.clone())),
        excluded_members: Some(wire::StringList::from(ep.excluded_members.clone())),
    }
}

fn domain_endpoint_to_wire_describe(
    ep: &crate::types::DbClusterEndpoint,
) -> wire::DBClusterEndpoint {
    wire::DBClusterEndpoint {
        d_b_cluster_endpoint_identifier: Some(ep.identifier.clone()),
        d_b_cluster_identifier: Some(ep.db_cluster_identifier.clone()),
        endpoint_type: Some(ep.endpoint_type.clone()),
        custom_endpoint_type: ep.custom_endpoint_type.clone(),
        endpoint: ep.endpoint.clone(),
        d_b_cluster_endpoint_arn: Some(ep.arn.clone()),
        d_b_cluster_endpoint_resource_identifier: Some(ep.resource_identifier.clone()),
        status: Some(ep.status.clone()),
        static_members: Some(wire::StringList::from(ep.static_members.clone())),
        excluded_members: Some(wire::StringList::from(ep.excluded_members.clone())),
    }
}

fn domain_event_subscription_to_wire(
    sub: &crate::types::EventSubscription,
) -> wire::EventSubscription {
    wire::EventSubscription {
        cust_subscription_id: Some(sub.subscription_name.clone()),
        sns_topic_arn: Some(sub.sns_topic_arn.clone()),
        source_type: sub.source_type.clone(),
        enabled: Some(sub.enabled),
        event_categories_list: Some(wire::EventCategoriesList::from(
            sub.event_categories.clone(),
        )),
        source_ids_list: Some(wire::SourceIdsList::from(sub.source_ids.clone())),
        status: Some(sub.status.clone()),
        event_subscription_arn: Some(sub.arn.clone()),
        customer_aws_id: Some(sub.customer_aws_id.clone()),
        subscription_creation_time: sub.subscription_creation_time.clone(),
    }
}

fn domain_snapshot_attributes_to_wire(
    snapshot_id: &str,
    attrs: &[crate::types::SnapshotAttribute],
) -> wire::DBClusterSnapshotAttributesResult {
    wire::DBClusterSnapshotAttributesResult {
        d_b_cluster_snapshot_identifier: Some(snapshot_id.to_string()),
        d_b_cluster_snapshot_attributes: Some(wire::DBClusterSnapshotAttributeList {
            items: attrs
                .iter()
                .map(|a| wire::DBClusterSnapshotAttribute {
                    attribute_name: Some(a.attribute_name.clone()),
                    attribute_values: Some(wire::AttributeValueList::from(
                        a.attribute_values.clone(),
                    )),
                })
                .collect(),
        }),
    }
}

// -------------------------------------------------------------------------
// Query string parsing helpers
// -------------------------------------------------------------------------

fn parse_query_string(s: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for pair in s.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            let key = urldecode(key);
            let value = urldecode(value);
            map.insert(key, value);
        }
    }
    map
}

fn urldecode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.bytes();
    while let Some(b) = chars.next() {
        match b {
            b'+' => result.push(' '),
            b'%' => {
                let hi = chars.next().and_then(hex_val);
                let lo = chars.next().and_then(hex_val);
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    result.push((hi << 4 | lo) as char);
                }
            }
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
