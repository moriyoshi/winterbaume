use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id, parse_query_string,
};

use crate::state::{RdsError, RdsState};
use crate::types::Tag;
use crate::views::RdsStateView;
use crate::wire;

/// RDS service handler.
pub struct RdsService {
    pub(crate) state: Arc<BackendState<RdsState>>,
    pub(crate) notifier: StateChangeNotifier<RdsStateView>,
}

impl RdsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }

    /// Returns sorted `(account_id, region)` pairs that have state.
    pub fn scopes_with_state(&self) -> Vec<(String, String)> {
        self.state.scopes_with_state()
    }
}

impl Default for RdsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for RdsService {
    fn service_name(&self) -> &str {
        "rds"
    }

    fn url_patterns(&self) -> Vec<&str> {
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
    "CreateDBInstance",
    "DeleteDBInstance",
    "ModifyDBInstance",
    "RebootDBInstance",
    "StartDBInstance",
    "StopDBInstance",
    "CreateDBInstanceReadReplica",
    "PromoteReadReplica",
    "RestoreDBInstanceFromDBSnapshot",
    "RestoreDBInstanceToPointInTime",
    "CreateDBCluster",
    "DeleteDBCluster",
    "ModifyDBCluster",
    "StartDBCluster",
    "StopDBCluster",
    "FailoverDBCluster",
    "PromoteReadReplicaDBCluster",
    "RestoreDBClusterFromSnapshot",
    "RestoreDBClusterToPointInTime",
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
    "CreateDBSnapshot",
    "DeleteDBSnapshot",
    "CopyDBSnapshot",
    "CreateDBClusterSnapshot",
    "DeleteDBClusterSnapshot",
    "CopyDBClusterSnapshot",
    "CreateDBSecurityGroup",
    "DeleteDBSecurityGroup",
    "CreateOptionGroup",
    "DeleteOptionGroup",
    "AddTagsToResource",
    "RemoveTagsFromResource",
    "CreateEventSubscription",
    "DeleteEventSubscription",
    "ModifyEventSubscription",
    "CreateGlobalCluster",
    "DeleteGlobalCluster",
    "StartExportTask",
    "CancelExportTask",
    "CreateDBProxy",
    "DeleteDBProxy",
    "ModifyDBProxy",
    "RegisterDBProxyTargets",
    "DeregisterDBProxyTargets",
    "ModifyDBProxyTargetGroup",
    "AddRoleToDBCluster",
    "RemoveRoleFromDBCluster",
    "AddRoleToDBInstance",
    "RemoveRoleFromDBInstance",
    "ModifyDBSnapshotAttribute",
    "ModifyDBClusterSnapshotAttribute",
    "CopyDBParameterGroup",
    "ModifyOptionGroup",
    "CreateBlueGreenDeployment",
    "DeleteBlueGreenDeployment",
    "SwitchoverBlueGreenDeployment",
    "CreateDBShardGroup",
    "DeleteDBShardGroup",
    "CreateDBClusterEndpoint",
    "ModifyDBClusterEndpoint",
    "DeleteDBClusterEndpoint",
    "CreateDBProxyEndpoint",
    "ModifyDBProxyEndpoint",
    "DeleteDBProxyEndpoint",
    "CopyDBClusterParameterGroup",
    "CopyOptionGroup",
    "AddSourceIdentifierToSubscription",
    "RemoveSourceIdentifierFromSubscription",
    "AuthorizeDBSecurityGroupIngress",
    "RevokeDBSecurityGroupIngress",
    "ModifyGlobalCluster",
    "FailoverGlobalCluster",
    "SwitchoverGlobalCluster",
    "RebootDBCluster",
    "ModifyDBProxy",
    "ModifyDBSnapshot",
    "SwitchoverReadReplica",
    "ApplyPendingMaintenanceAction",
    "ModifyDBShardGroup",
    "RebootDBShardGroup",
    "BacktrackDBCluster",
    "EnableHttpEndpoint",
    "DisableHttpEndpoint",
    "ModifyCurrentDBClusterCapacity",
    "RestoreDBClusterFromS3",
    "RestoreDBInstanceFromS3",
    "StartActivityStream",
    "StopActivityStream",
    "ModifyActivityStream",
    "StartDBInstanceAutomatedBackupsReplication",
    "StopDBInstanceAutomatedBackupsReplication",
    "DeleteDBInstanceAutomatedBackup",
    "DeleteDBClusterAutomatedBackup",
];

impl RdsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

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
            // DB Instances
            "CreateDBInstance" => {
                self.handle_create_db_instance(&state, &params, account_id, &region)
                    .await
            }
            "DescribeDBInstances" => self.handle_describe_db_instances(&state, &params).await,
            "DeleteDBInstance" => self.handle_delete_db_instance(&state, &params).await,
            "ModifyDBInstance" => self.handle_modify_db_instance(&state, &params).await,
            "RebootDBInstance" => self.handle_reboot_db_instance(&state, &params).await,
            "StartDBInstance" => self.handle_start_db_instance(&state, &params).await,
            "StopDBInstance" => self.handle_stop_db_instance(&state, &params).await,
            "CreateDBInstanceReadReplica" => {
                self.handle_create_db_instance_read_replica(&state, &params, account_id, &region)
                    .await
            }
            "PromoteReadReplica" => self.handle_promote_read_replica(&state, &params).await,
            "RestoreDBInstanceFromDBSnapshot" => {
                self.handle_restore_db_instance_from_db_snapshot(
                    &state, &params, account_id, &region,
                )
                .await
            }
            "RestoreDBInstanceToPointInTime" => {
                self.handle_restore_db_instance_to_point_in_time(
                    &state, &params, account_id, &region,
                )
                .await
            }
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
            "DescribeDBParameters" => self.handle_describe_db_parameters(&params).await,
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
                self.handle_describe_db_cluster_parameters(&params).await
            }
            // DB Snapshots
            "CreateDBSnapshot" => {
                self.handle_create_db_snapshot(&state, &params, account_id, &region)
                    .await
            }
            "DescribeDBSnapshots" => self.handle_describe_db_snapshots(&state, &params).await,
            "DeleteDBSnapshot" => self.handle_delete_db_snapshot(&state, &params).await,
            "CopyDBSnapshot" => {
                self.handle_copy_db_snapshot(&state, &params, account_id, &region)
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
            "CopyDBClusterSnapshot" => {
                self.handle_copy_db_cluster_snapshot(&state, &params, account_id, &region)
                    .await
            }
            // DB Security Groups
            "CreateDBSecurityGroup" => {
                self.handle_create_db_security_group(&state, &params, account_id, &region)
                    .await
            }
            "DescribeDBSecurityGroups" => {
                self.handle_describe_db_security_groups(&state, &params)
                    .await
            }
            "DeleteDBSecurityGroup" => self.handle_delete_db_security_group(&state, &params).await,
            // Tags
            "AddTagsToResource" => self.handle_add_tags_to_resource(&state, &params).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, &params).await,
            "RemoveTagsFromResource" => {
                self.handle_remove_tags_from_resource(&state, &params).await
            }
            // Event Subscriptions
            "CreateEventSubscription" => {
                self.handle_create_event_subscription(&state, &params, account_id, &region)
                    .await
            }
            "DescribeEventSubscriptions" => {
                self.handle_describe_event_subscriptions(&state, &params)
                    .await
            }
            "DeleteEventSubscription" => {
                self.handle_delete_event_subscription(&state, &params).await
            }
            "ModifyEventSubscription" => {
                self.handle_modify_event_subscription(&state, &params).await
            }
            // Events
            "DescribeEvents" => self.handle_describe_events().await,
            // Option Groups
            "CreateOptionGroup" => {
                self.handle_create_option_group(&state, &params, account_id, &region)
                    .await
            }
            "DescribeOptionGroups" => self.handle_describe_option_groups(&state, &params).await,
            "DeleteOptionGroup" => self.handle_delete_option_group(&state, &params).await,
            "DescribeOptionGroupOptions" => {
                self.handle_describe_option_group_options(&params).await
            }
            // Engine Versions / Orderable Options
            "DescribeDBEngineVersions" => self.handle_describe_db_engine_versions(&params).await,
            "DescribeOrderableDBInstanceOptions" => {
                self.handle_describe_orderable_db_instance_options(&params)
                    .await
            }
            "DescribeValidDBInstanceModifications" => {
                self.handle_describe_valid_db_instance_modifications(&state, &params)
                    .await
            }
            // Global Clusters
            "CreateGlobalCluster" => {
                self.handle_create_global_cluster(&state, &params, account_id, &region)
                    .await
            }
            "DescribeGlobalClusters" => self.handle_describe_global_clusters(&state, &params).await,
            "DeleteGlobalCluster" => self.handle_delete_global_cluster(&state, &params).await,
            "RemoveFromGlobalCluster" => {
                self.handle_remove_from_global_cluster(&state, &params)
                    .await
            }
            // Export Tasks
            "StartExportTask" => self.handle_start_export_task(&state, &params).await,
            "CancelExportTask" => self.handle_cancel_export_task(&state, &params).await,
            "DescribeExportTasks" => self.handle_describe_export_tasks(&state, &params).await,
            // DB Proxies
            "CreateDBProxy" => {
                self.handle_create_db_proxy(&state, &params, account_id, &region)
                    .await
            }
            "DescribeDBProxies" => self.handle_describe_db_proxies(&state, &params).await,
            "DeleteDBProxy" => self.handle_delete_db_proxy(&state, &params).await,
            "RegisterDBProxyTargets" => {
                self.handle_register_db_proxy_targets(&state, &params, account_id, &region)
                    .await
            }
            "DeregisterDBProxyTargets" => {
                self.handle_deregister_db_proxy_targets(&state, &params)
                    .await
            }
            "DescribeDBProxyTargets" => {
                self.handle_describe_db_proxy_targets(&state, &params).await
            }
            "DescribeDBProxyTargetGroups" => {
                self.handle_describe_db_proxy_target_groups(&state, &params, account_id, &region)
                    .await
            }
            "ModifyDBProxyTargetGroup" => {
                self.handle_modify_db_proxy_target_group(&state, &params, account_id, &region)
                    .await
            }
            // Role associations
            "AddRoleToDBCluster" => self.handle_add_role_to_db_cluster(&state, &params).await,
            "RemoveRoleFromDBCluster" => {
                self.handle_remove_role_from_db_cluster(&state, &params)
                    .await
            }
            "AddRoleToDBInstance" => self.handle_add_role_to_db_instance(&state, &params).await,
            "RemoveRoleFromDBInstance" => {
                self.handle_remove_role_from_db_instance(&state, &params)
                    .await
            }
            // Snapshot attributes
            "DescribeDBSnapshotAttributes" => {
                self.handle_describe_db_snapshot_attributes(&state, &params)
                    .await
            }
            "ModifyDBSnapshotAttribute" => {
                self.handle_modify_db_snapshot_attribute(&state, &params)
                    .await
            }
            "DescribeDBClusterSnapshotAttributes" => {
                self.handle_describe_db_cluster_snapshot_attributes(&state, &params)
                    .await
            }
            "ModifyDBClusterSnapshotAttribute" => {
                self.handle_modify_db_cluster_snapshot_attribute(&state, &params)
                    .await
            }
            // Parameter group copy
            "CopyDBParameterGroup" => {
                self.handle_copy_db_parameter_group(&state, &params, account_id, &region)
                    .await
            }
            // Automated backups
            "DescribeDBInstanceAutomatedBackups" => {
                self.handle_describe_db_instance_automated_backups(&state, &params)
                    .await
            }
            // Option group modification
            "ModifyOptionGroup" => self.handle_modify_option_group(&state, &params).await,
            // Blue/Green deployments
            "CreateBlueGreenDeployment" => {
                self.handle_create_blue_green_deployment(&state, &params)
                    .await
            }
            "DeleteBlueGreenDeployment" => {
                self.handle_delete_blue_green_deployment(&state, &params)
                    .await
            }
            "DescribeBlueGreenDeployments" => {
                self.handle_describe_blue_green_deployments(&state, &params)
                    .await
            }
            "SwitchoverBlueGreenDeployment" => {
                self.handle_switchover_blue_green_deployment(&state, &params)
                    .await
            }
            // DB Shard Groups
            "CreateDBShardGroup" => {
                self.handle_create_db_shard_group(&state, &params, account_id, &region)
                    .await
            }
            "DescribeDBShardGroups" => self.handle_describe_db_shard_groups(&state, &params).await,
            "DeleteDBShardGroup" => self.handle_delete_db_shard_group(&state, &params).await,
            "ModifyDBShardGroup" => self.handle_modify_db_shard_group(&state, &params).await,
            "RebootDBShardGroup" => self.handle_reboot_db_shard_group(&state, &params).await,
            // DB Log Files
            "DescribeDBLogFiles" => self.handle_describe_db_log_files(&state, &params).await,
            "DownloadDBLogFilePortion" => {
                self.handle_download_db_log_file_portion(&state, &params)
                    .await
            }
            // Account attributes
            "DescribeAccountAttributes" => self.handle_describe_account_attributes().await,
            "DescribeCertificates" => self.handle_describe_certificates().await,
            // DB Cluster Endpoints
            "CreateDBClusterEndpoint" => {
                self.handle_create_db_cluster_endpoint(&state, &params, account_id, &region)
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
            "DeleteDBClusterEndpoint" => {
                self.handle_delete_db_cluster_endpoint(&state, &params)
                    .await
            }
            // DB Proxy Endpoints
            "CreateDBProxyEndpoint" => {
                self.handle_create_db_proxy_endpoint(&state, &params, account_id, &region)
                    .await
            }
            "DescribeDBProxyEndpoints" => {
                self.handle_describe_db_proxy_endpoints(&state, &params)
                    .await
            }
            "ModifyDBProxyEndpoint" => self.handle_modify_db_proxy_endpoint(&state, &params).await,
            "DeleteDBProxyEndpoint" => self.handle_delete_db_proxy_endpoint(&state, &params).await,
            // Copy operations
            "CopyDBClusterParameterGroup" => {
                self.handle_copy_db_cluster_parameter_group(&state, &params, account_id, &region)
                    .await
            }
            "CopyOptionGroup" => {
                self.handle_copy_option_group(&state, &params, account_id, &region)
                    .await
            }
            // Event subscription source identifiers
            "AddSourceIdentifierToSubscription" => {
                self.handle_add_source_identifier_to_subscription(&state, &params)
                    .await
            }
            "RemoveSourceIdentifierFromSubscription" => {
                self.handle_remove_source_identifier_from_subscription(&state, &params)
                    .await
            }
            // DB Security Group ingress
            "AuthorizeDBSecurityGroupIngress" => {
                self.handle_authorize_db_security_group_ingress(&state, &params)
                    .await
            }
            "RevokeDBSecurityGroupIngress" => {
                self.handle_revoke_db_security_group_ingress(&state, &params)
                    .await
            }
            // Global Cluster operations
            "ModifyGlobalCluster" => self.handle_modify_global_cluster(&state, &params).await,
            "FailoverGlobalCluster" => self.handle_failover_global_cluster(&state, &params).await,
            "SwitchoverGlobalCluster" => {
                self.handle_switchover_global_cluster(&state, &params).await
            }
            // DB Cluster reboot
            "RebootDBCluster" => self.handle_reboot_db_cluster(&state, &params).await,
            // Modify DB Proxy
            "ModifyDBProxy" => self.handle_modify_db_proxy(&state, &params).await,
            // Modify DB Snapshot
            "ModifyDBSnapshot" => self.handle_modify_db_snapshot(&state, &params).await,
            // Switchover Read Replica
            "SwitchoverReadReplica" => self.handle_switchover_read_replica(&state, &params).await,
            // Apply Pending Maintenance Action
            "ApplyPendingMaintenanceAction" => {
                self.handle_apply_pending_maintenance_action(&state, &params)
                    .await
            }
            // Describe-only operations
            "DescribeEngineDefaultClusterParameters" => {
                self.handle_describe_engine_default_cluster_parameters(&params)
                    .await
            }
            "DescribeEngineDefaultParameters" => {
                self.handle_describe_engine_default_parameters(&params)
                    .await
            }
            "DescribeEventCategories" => self.handle_describe_event_categories(&params).await,
            "DescribePendingMaintenanceActions" => {
                self.handle_describe_pending_maintenance_actions(&params)
                    .await
            }
            "DescribeSourceRegions" => self.handle_describe_source_regions(&params).await,
            // Backtrack DB Cluster
            "BacktrackDBCluster" => self.handle_backtrack_db_cluster(&state, &params).await,
            // HTTP endpoint (Aurora Serverless)
            "EnableHttpEndpoint" => self.handle_enable_http_endpoint(&state, &params).await,
            "DisableHttpEndpoint" => self.handle_disable_http_endpoint(&state, &params).await,
            // Modify current DB cluster capacity
            "ModifyCurrentDBClusterCapacity" => {
                self.handle_modify_current_db_cluster_capacity(&state, &params)
                    .await
            }
            // Restore from S3
            "RestoreDBClusterFromS3" => {
                self.handle_restore_db_cluster_from_s3(&state, &params, account_id, &region)
                    .await
            }
            "RestoreDBInstanceFromS3" => {
                self.handle_restore_db_instance_from_s3(&state, &params, account_id, &region)
                    .await
            }
            // Describe operations returning empty lists
            "DescribeDBClusterAutomatedBackups" => {
                self.handle_describe_db_cluster_automated_backups(&params)
                    .await
            }
            "DescribeDBClusterBacktracks" => {
                self.handle_describe_db_cluster_backtracks(&params).await
            }
            "DescribeDBMajorEngineVersions" => {
                self.handle_describe_db_major_engine_versions(&params).await
            }
            "DescribeReservedDBInstances" => {
                self.handle_describe_reserved_db_instances(&params).await
            }
            "DescribeReservedDBInstancesOfferings" => {
                self.handle_describe_reserved_db_instances_offerings(&params)
                    .await
            }
            "DescribeDBRecommendations" => self.handle_describe_db_recommendations(&params).await,
            "DescribeDBSnapshotTenantDatabases" => {
                self.handle_describe_db_snapshot_tenant_databases(&params)
                    .await
            }
            // Activity streams
            "StartActivityStream" => self.handle_start_activity_stream(&state, &params).await,
            "StopActivityStream" => self.handle_stop_activity_stream(&state, &params).await,
            "ModifyActivityStream" => self.handle_modify_activity_stream(&state, &params).await,
            // Automated backup replication
            "StartDBInstanceAutomatedBackupsReplication" => {
                self.handle_start_db_instance_automated_backups_replication(
                    &state, &params, account_id, &region,
                )
                .await
            }
            "StopDBInstanceAutomatedBackupsReplication" => {
                self.handle_stop_db_instance_automated_backups_replication(&state, &params)
                    .await
            }
            "DeleteDBInstanceAutomatedBackup" => {
                self.handle_delete_db_instance_automated_backup(&state, &params)
                    .await
            }
            "DeleteDBClusterAutomatedBackup" => {
                self.handle_delete_db_cluster_automated_backup(&state, &params)
                    .await
            }
            // Modify certificates
            "ModifyCertificates" => self.handle_modify_certificates(&params).await,
            _ => MockResponse::error(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for RDS"),
            ),
        };

        if MUTATING_ACTIONS.contains(&action.as_str()) && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // -------------------------------------------------------------------------
    // DB Instances
    // -------------------------------------------------------------------------

    async fn handle_create_db_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_d_b_instance_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = if input.d_b_instance_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DBInstanceIdentifier",
            );
        } else {
            input.d_b_instance_identifier
        };
        let db_instance_class = if input.d_b_instance_class.is_empty() {
            "db.t3.micro".to_string()
        } else {
            input.d_b_instance_class
        };
        let engine = if input.engine.is_empty() {
            "mysql".to_string()
        } else {
            input.engine
        };
        let engine_version = input.engine_version;
        let master_username = input.master_username;
        let db_name = input.d_b_name;
        let port = input.port;
        let multi_az = input.multi_a_z.unwrap_or(false);
        let storage_type = input.storage_type;
        let allocated_storage = input.allocated_storage.unwrap_or(20);
        let db_subnet_group_name = input.d_b_subnet_group_name;
        let vpc_security_group_ids = input
            .vpc_security_group_ids
            .map(|l| l.items)
            .unwrap_or_default();
        let db_parameter_group_names = input.d_b_parameter_group_name.into_iter().collect();
        let availability_zone = input.availability_zone;
        let publicly_accessible = input.publicly_accessible.unwrap_or(false);
        let auto_minor_version_upgrade = input.auto_minor_version_upgrade.unwrap_or(true);
        let backup_retention_period = input.backup_retention_period.unwrap_or(1);
        let db_cluster_identifier = input.d_b_cluster_identifier;
        let license_model = input.license_model;
        let iops = input.iops;
        let deletion_protection = input.deletion_protection.unwrap_or(false);
        let copy_tags_to_snapshot = input.copy_tags_to_snapshot.unwrap_or(false);
        let monitoring_interval = input.monitoring_interval;
        let performance_insights_enabled = input.enable_performance_insights.unwrap_or(false);
        let storage_encrypted = input.storage_encrypted.unwrap_or(false);
        let kms_key_id = input.kms_key_id;
        let ca_certificate_identifier = input.c_a_certificate_identifier;
        let tags = wire_tags_to_domain(input.tags);

        let mut st = state.write().await;
        match st.create_db_instance(
            identifier,
            db_instance_class,
            engine,
            engine_version,
            master_username,
            db_name,
            port,
            multi_az,
            storage_type,
            allocated_storage,
            db_subnet_group_name,
            vpc_security_group_ids,
            db_parameter_group_names,
            availability_zone,
            publicly_accessible,
            auto_minor_version_upgrade,
            backup_retention_period,
            db_cluster_identifier,
            license_model,
            iops,
            deletion_protection,
            copy_tags_to_snapshot,
            monitoring_interval,
            performance_insights_enabled,
            storage_encrypted,
            kms_key_id,
            ca_certificate_identifier,
            tags,
            account_id,
            region,
        ) {
            Ok(inst) => {
                wire::serialize_create_d_b_instance_response(&wire::CreateDBInstanceResult {
                    d_b_instance: Some(db_instance_to_model(&inst)),
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_describe_db_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_d_b_instances_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_instance_identifier;
        let st = state.read().await;
        match st.describe_db_instances(identifier.as_deref()) {
            Ok(instances) => {
                wire::serialize_describe_d_b_instances_response(&wire::DBInstanceMessage {
                    d_b_instances: Some(wire::DBInstanceList::from(
                        instances
                            .iter()
                            .map(|i| db_instance_to_model(i))
                            .collect::<Vec<_>>(),
                    )),
                    marker: None,
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_delete_db_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_d_b_instance_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_instance_identifier;
        let mut st = state.write().await;
        match st.delete_db_instance(&identifier) {
            Ok(inst) => {
                wire::serialize_delete_d_b_instance_response(&wire::DeleteDBInstanceResult {
                    d_b_instance: Some(db_instance_to_model(&inst)),
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_modify_db_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_d_b_instance_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_instance_identifier;

        let mut st = state.write().await;
        match st.modify_db_instance(
            &identifier,
            input.d_b_instance_class,
            input.engine_version,
            input.multi_a_z,
            input.storage_type,
            input.allocated_storage,
            input.backup_retention_period,
            input.iops,
            input.deletion_protection,
            input.auto_minor_version_upgrade,
            input.new_d_b_instance_identifier,
            input.master_user_password,
        ) {
            Ok(inst) => {
                wire::serialize_modify_d_b_instance_response(&wire::ModifyDBInstanceResult {
                    d_b_instance: Some(db_instance_to_model(&inst)),
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_reboot_db_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_reboot_d_b_instance_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        match st.reboot_db_instance(&input.d_b_instance_identifier) {
            Ok(inst) => {
                wire::serialize_reboot_d_b_instance_response(&wire::RebootDBInstanceResult {
                    d_b_instance: Some(db_instance_to_model(&inst)),
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_start_db_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_start_d_b_instance_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_instance_identifier;
        let mut st = state.write().await;
        match st.start_db_instance(&identifier) {
            Ok(inst) => wire::serialize_start_d_b_instance_response(&wire::StartDBInstanceResult {
                d_b_instance: Some(db_instance_to_model(&inst)),
            }),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_stop_db_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_stop_d_b_instance_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_instance_identifier;
        let mut st = state.write().await;
        match st.stop_db_instance(&identifier) {
            Ok(inst) => wire::serialize_stop_d_b_instance_response(&wire::StopDBInstanceResult {
                d_b_instance: Some(db_instance_to_model(&inst)),
            }),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_create_db_instance_read_replica(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_d_b_instance_read_replica_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let replica_id = input.d_b_instance_identifier;
        let source_id = match input.source_d_b_instance_identifier {
            Some(v) => v,
            None => {
                return MockResponse::error(
                    400,
                    "InvalidParameterValue",
                    "Missing SourceDBInstanceIdentifier",
                );
            }
        };
        let tags = wire_tags_to_domain(input.tags);
        let mut st = state.write().await;
        match st.create_db_instance_read_replica(replica_id, &source_id, tags, account_id, region) {
            Ok(inst) => wire::serialize_create_d_b_instance_read_replica_response(
                &wire::CreateDBInstanceReadReplicaResult {
                    d_b_instance: Some(db_instance_to_model(&inst)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_promote_read_replica(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_promote_read_replica_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_instance_identifier;
        let mut st = state.write().await;
        match st.promote_read_replica(&identifier, input.backup_retention_period) {
            Ok(inst) => {
                wire::serialize_promote_read_replica_response(&wire::PromoteReadReplicaResult {
                    d_b_instance: Some(db_instance_to_model(&inst)),
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_restore_db_instance_from_db_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_restore_d_b_instance_from_d_b_snapshot_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let new_identifier = input.d_b_instance_identifier;
        let snapshot_id = match input.d_b_snapshot_identifier {
            Some(v) => v,
            None => {
                return MockResponse::error(
                    400,
                    "InvalidParameterValue",
                    "Missing DBSnapshotIdentifier",
                );
            }
        };
        let tags = wire_tags_to_domain(input.tags);
        let mut st = state.write().await;
        match st.restore_db_instance_from_db_snapshot(
            &snapshot_id,
            new_identifier,
            tags,
            account_id,
            region,
        ) {
            Ok(inst) => wire::serialize_restore_d_b_instance_from_d_b_snapshot_response(
                &wire::RestoreDBInstanceFromDBSnapshotResult {
                    d_b_instance: Some(db_instance_to_model(&inst)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_restore_db_instance_to_point_in_time(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_restore_d_b_instance_to_point_in_time_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let source_id = match input.source_d_b_instance_identifier {
            Some(v) => v,
            None => {
                return MockResponse::error(
                    400,
                    "InvalidParameterValue",
                    "Missing SourceDBInstanceIdentifier",
                );
            }
        };
        let target_id = input.target_d_b_instance_identifier;
        let tags = wire_tags_to_domain(input.tags);
        let mut st = state.write().await;
        match st
            .restore_db_instance_to_point_in_time(&source_id, target_id, tags, account_id, region)
        {
            Ok(inst) => wire::serialize_restore_d_b_instance_to_point_in_time_response(
                &wire::RestoreDBInstanceToPointInTimeResult {
                    d_b_instance: Some(db_instance_to_model(&inst)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // DB Clusters
    // -------------------------------------------------------------------------

    async fn handle_create_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_d_b_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_cluster_identifier;
        let engine = if input.engine.is_empty() {
            "aurora-mysql".to_string()
        } else {
            input.engine
        };
        let engine_version = input.engine_version;
        let master_username = input.master_username;
        let database_name = input.database_name;
        let port = input.port;
        let db_subnet_group_name = input.d_b_subnet_group_name;
        let vpc_security_group_ids = input
            .vpc_security_group_ids
            .map(|l| l.items)
            .unwrap_or_default();
        let availability_zones = input
            .availability_zones
            .map(|l| l.items)
            .unwrap_or_default();
        let backup_retention_period = input.backup_retention_period.unwrap_or(1);
        let deletion_protection = input.deletion_protection.unwrap_or(false);
        let storage_encrypted = input.storage_encrypted.unwrap_or(false);
        let kms_key_id = input.kms_key_id;
        let db_cluster_parameter_group = input.d_b_cluster_parameter_group_name;
        let engine_mode = input.engine_mode;
        let copy_tags_to_snapshot = input.copy_tags_to_snapshot.unwrap_or(false);
        let tags = wire_tags_to_domain(input.tags);

        let mut st = state.write().await;
        match st.create_db_cluster(
            identifier,
            engine,
            engine_version,
            master_username,
            database_name,
            port,
            db_subnet_group_name,
            vpc_security_group_ids,
            availability_zones,
            backup_retention_period,
            deletion_protection,
            storage_encrypted,
            kms_key_id,
            db_cluster_parameter_group,
            engine_mode,
            copy_tags_to_snapshot,
            tags,
            account_id,
            region,
        ) {
            Ok(cluster) => {
                wire::serialize_create_d_b_cluster_response(&wire::CreateDBClusterResult {
                    d_b_cluster: Some(db_cluster_to_model(&cluster)),
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_describe_db_clusters(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
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
                    d_b_clusters: Some(wire::DBClusterList::from(
                        clusters
                            .iter()
                            .map(|c| db_cluster_to_model(c))
                            .collect::<Vec<_>>(),
                    )),
                    marker: None,
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_delete_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_d_b_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_cluster_identifier;
        let mut st = state.write().await;
        match st.delete_db_cluster(&identifier) {
            Ok(cluster) => {
                wire::serialize_delete_d_b_cluster_response(&wire::DeleteDBClusterResult {
                    d_b_cluster: Some(db_cluster_to_model(&cluster)),
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_modify_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_d_b_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_cluster_identifier;
        let vpc_security_group_ids = input.vpc_security_group_ids.map(|l| l.items);

        let mut st = state.write().await;
        match st.modify_db_cluster(
            &identifier,
            input.engine_version,
            input.master_user_password,
            input.backup_retention_period,
            input.deletion_protection,
            input.new_d_b_cluster_identifier,
            vpc_security_group_ids,
            input.d_b_cluster_parameter_group_name,
        ) {
            Ok(cluster) => {
                wire::serialize_modify_d_b_cluster_response(&wire::ModifyDBClusterResult {
                    d_b_cluster: Some(db_cluster_to_model(&cluster)),
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_start_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_start_d_b_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_cluster_identifier;
        let mut st = state.write().await;
        match st.start_db_cluster(&identifier) {
            Ok(cluster) => {
                wire::serialize_start_d_b_cluster_response(&wire::StartDBClusterResult {
                    d_b_cluster: Some(db_cluster_to_model(&cluster)),
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_stop_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_stop_d_b_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_cluster_identifier;
        let mut st = state.write().await;
        match st.stop_db_cluster(&identifier) {
            Ok(cluster) => wire::serialize_stop_d_b_cluster_response(&wire::StopDBClusterResult {
                d_b_cluster: Some(db_cluster_to_model(&cluster)),
            }),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_failover_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_failover_d_b_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_cluster_identifier;
        let st = state.read().await;
        match st.failover_db_cluster(&identifier) {
            Ok(cluster) => {
                wire::serialize_failover_d_b_cluster_response(&wire::FailoverDBClusterResult {
                    d_b_cluster: Some(db_cluster_to_model(&cluster)),
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_promote_read_replica_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_promote_read_replica_d_b_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        match st.promote_read_replica_db_cluster(&input.d_b_cluster_identifier) {
            Ok(cluster) => wire::serialize_promote_read_replica_d_b_cluster_response(
                &wire::PromoteReadReplicaDBClusterResult {
                    d_b_cluster: Some(db_cluster_to_model(&cluster)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_restore_db_cluster_from_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_restore_d_b_cluster_from_snapshot_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let snapshot_id = input.snapshot_identifier;
        let new_cluster_id = input.d_b_cluster_identifier;
        let tags = wire_tags_to_domain(input.tags);
        let mut st = state.write().await;
        match st.restore_db_cluster_from_snapshot(
            &snapshot_id,
            new_cluster_id,
            tags,
            account_id,
            region,
        ) {
            Ok(cluster) => wire::serialize_restore_d_b_cluster_from_snapshot_response(
                &wire::RestoreDBClusterFromSnapshotResult {
                    d_b_cluster: Some(db_cluster_to_model(&cluster)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_restore_db_cluster_to_point_in_time(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_restore_d_b_cluster_to_point_in_time_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let source_id = match input.source_d_b_cluster_identifier {
            Some(v) => v,
            None => {
                return MockResponse::error(
                    400,
                    "InvalidParameterValue",
                    "Missing SourceDBClusterIdentifier",
                );
            }
        };
        let target_id = input.d_b_cluster_identifier;
        let tags = wire_tags_to_domain(input.tags);
        // Reuse restore_db_cluster_from_snapshot logic via creating a temp snapshot
        // For simplicity, create cluster from source cluster data
        let st_read = state.read().await;
        let source_cluster = match st_read.describe_db_clusters(Some(&source_id)) {
            Ok(v) if !v.is_empty() => v[0].clone(),
            _ => return rds_error_response(&RdsError::not_found("DBCluster", &source_id)),
        };
        drop(st_read);

        let arn = format!("arn:aws:rds:{region}:{account_id}:cluster:{target_id}");
        let endpoint = Some(format!(
            "{target_id}.cluster-{account_id}.{region}.rds.amazonaws.com"
        ));
        let reader_endpoint = Some(format!(
            "{target_id}.cluster-ro-{account_id}.{region}.rds.amazonaws.com"
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
            return rds_error_response(&RdsError::already_exists("DBCluster", &target_id));
        }
        st.db_clusters.insert(target_id, new_cluster.clone());
        wire::serialize_restore_d_b_cluster_to_point_in_time_response(
            &wire::RestoreDBClusterToPointInTimeResult {
                d_b_cluster: Some(db_cluster_to_model(&new_cluster)),
            },
        )
    }

    // -------------------------------------------------------------------------
    // DB Subnet Groups
    // -------------------------------------------------------------------------

    async fn handle_create_db_subnet_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_d_b_subnet_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.d_b_subnet_group_name;
        let description = input.d_b_subnet_group_description;
        let subnet_ids = input.subnet_ids.items;
        let tags = wire_tags_to_domain(input.tags);
        let mut st = state.write().await;
        match st.create_db_subnet_group(name, description, subnet_ids, tags, account_id, region) {
            Ok(sg) => {
                wire::serialize_create_d_b_subnet_group_response(&wire::CreateDBSubnetGroupResult {
                    d_b_subnet_group: Some(db_subnet_group_to_model(&sg)),
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_describe_db_subnet_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
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
                    d_b_subnet_groups: Some(wire::DBSubnetGroups::from(
                        groups
                            .iter()
                            .map(|g| db_subnet_group_to_model(g))
                            .collect::<Vec<_>>(),
                    )),
                    marker: None,
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_delete_db_subnet_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_d_b_subnet_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.d_b_subnet_group_name;
        let mut st = state.write().await;
        match st.delete_db_subnet_group(&name) {
            Ok(()) => wire::serialize_delete_d_b_subnet_group_response(),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_modify_db_subnet_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_d_b_subnet_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.d_b_subnet_group_name;
        let description = input.d_b_subnet_group_description;
        let subnet_ids_items = input.subnet_ids.items;
        let subnet_ids = if subnet_ids_items.is_empty() {
            None
        } else {
            Some(subnet_ids_items)
        };
        let mut st = state.write().await;
        match st.modify_db_subnet_group(&name, description, subnet_ids) {
            Ok(sg) => {
                wire::serialize_modify_d_b_subnet_group_response(&wire::ModifyDBSubnetGroupResult {
                    d_b_subnet_group: Some(db_subnet_group_to_model(&sg)),
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // DB Parameter Groups
    // -------------------------------------------------------------------------

    async fn handle_create_db_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_d_b_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.d_b_parameter_group_name;
        let family = input.d_b_parameter_group_family;
        let description = input.description;
        let tags = wire_tags_to_domain(input.tags);
        let mut st = state.write().await;
        match st.create_db_parameter_group(name, family, description, tags, account_id, region) {
            Ok(pg) => wire::serialize_create_d_b_parameter_group_response(
                &wire::CreateDBParameterGroupResult {
                    d_b_parameter_group: Some(db_parameter_group_to_model(&pg)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_describe_db_parameter_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
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
                    d_b_parameter_groups: Some(wire::DBParameterGroupList::from(
                        groups
                            .iter()
                            .map(|g| db_parameter_group_to_model(g))
                            .collect::<Vec<_>>(),
                    )),
                    marker: None,
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_delete_db_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_d_b_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.d_b_parameter_group_name;
        let mut st = state.write().await;
        match st.delete_db_parameter_group(&name) {
            Ok(()) => wire::serialize_delete_d_b_parameter_group_response(),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_modify_db_parameter_group(
        &self,
        _state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_d_b_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        wire::serialize_modify_d_b_parameter_group_response(&wire::DBParameterGroupNameMessage {
            d_b_parameter_group_name: Some(input.d_b_parameter_group_name),
        })
    }

    async fn handle_reset_db_parameter_group(
        &self,
        _state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_reset_d_b_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        wire::serialize_reset_d_b_parameter_group_response(&wire::DBParameterGroupNameMessage {
            d_b_parameter_group_name: Some(input.d_b_parameter_group_name),
        })
    }

    async fn handle_describe_db_parameters(
        &self,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = wire::deserialize_describe_d_b_parameters_request(params);
        wire::serialize_describe_d_b_parameters_response(&wire::DBParameterGroupDetails::default())
    }

    // -------------------------------------------------------------------------
    // DB Cluster Parameter Groups
    // -------------------------------------------------------------------------

    async fn handle_create_db_cluster_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_d_b_cluster_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.d_b_cluster_parameter_group_name;
        let family = input.d_b_parameter_group_family;
        let description = input.description;
        let tags = wire_tags_to_domain(input.tags);
        let mut st = state.write().await;
        match st.create_db_cluster_parameter_group(
            name,
            family,
            description,
            tags,
            account_id,
            region,
        ) {
            Ok(pg) => wire::serialize_create_d_b_cluster_parameter_group_response(
                &wire::CreateDBClusterParameterGroupResult {
                    d_b_cluster_parameter_group: Some(db_cluster_parameter_group_to_model(&pg)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_describe_db_cluster_parameter_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
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
                    d_b_cluster_parameter_groups: Some(wire::DBClusterParameterGroupList::from(
                        groups
                            .iter()
                            .map(|g| db_cluster_parameter_group_to_model(g))
                            .collect::<Vec<_>>(),
                    )),
                    marker: None,
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_delete_db_cluster_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_d_b_cluster_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.d_b_cluster_parameter_group_name;
        let mut st = state.write().await;
        match st.delete_db_cluster_parameter_group(&name) {
            Ok(()) => wire::serialize_delete_d_b_cluster_parameter_group_response(),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_modify_db_cluster_parameter_group(
        &self,
        _state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_d_b_cluster_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        wire::serialize_modify_d_b_cluster_parameter_group_response(
            &wire::DBClusterParameterGroupNameMessage {
                d_b_cluster_parameter_group_name: Some(input.d_b_cluster_parameter_group_name),
            },
        )
    }

    async fn handle_reset_db_cluster_parameter_group(
        &self,
        _state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_reset_d_b_cluster_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        wire::serialize_reset_d_b_cluster_parameter_group_response(
            &wire::DBClusterParameterGroupNameMessage {
                d_b_cluster_parameter_group_name: Some(input.d_b_cluster_parameter_group_name),
            },
        )
    }

    async fn handle_describe_db_cluster_parameters(
        &self,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = wire::deserialize_describe_d_b_cluster_parameters_request(params);
        wire::serialize_describe_d_b_cluster_parameters_response(
            &wire::DBClusterParameterGroupDetails::default(),
        )
    }

    // -------------------------------------------------------------------------
    // DB Snapshots
    // -------------------------------------------------------------------------

    async fn handle_create_db_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_d_b_snapshot_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let snapshot_id = input.d_b_snapshot_identifier;
        let db_instance_id = input.d_b_instance_identifier;
        let tags = wire_tags_to_domain(input.tags);
        let mut st = state.write().await;
        match st.create_db_snapshot(snapshot_id, &db_instance_id, tags, account_id, region) {
            Ok(snap) => {
                wire::serialize_create_d_b_snapshot_response(&wire::CreateDBSnapshotResult {
                    d_b_snapshot: Some(db_snapshot_to_model(&snap)),
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_describe_db_snapshots(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_d_b_snapshots_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        let snaps = st.describe_db_snapshots(
            input.d_b_snapshot_identifier.as_deref(),
            input.d_b_instance_identifier.as_deref(),
        );
        wire::serialize_describe_d_b_snapshots_response(&wire::DBSnapshotMessage {
            d_b_snapshots: Some(wire::DBSnapshotList::from(
                snaps
                    .iter()
                    .map(|s| db_snapshot_to_model(s))
                    .collect::<Vec<_>>(),
            )),
            marker: None,
        })
    }

    async fn handle_delete_db_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_d_b_snapshot_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let snapshot_id = input.d_b_snapshot_identifier;
        let mut st = state.write().await;
        match st.delete_db_snapshot(&snapshot_id) {
            Ok(snap) => {
                wire::serialize_delete_d_b_snapshot_response(&wire::DeleteDBSnapshotResult {
                    d_b_snapshot: Some(db_snapshot_to_model(&snap)),
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_copy_db_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_copy_d_b_snapshot_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let source_id = input.source_d_b_snapshot_identifier;
        let target_id = input.target_d_b_snapshot_identifier;
        let tags = wire_tags_to_domain(input.tags);
        let mut st = state.write().await;
        match st.copy_db_snapshot(&source_id, target_id, tags, account_id, region) {
            Ok(snap) => wire::serialize_copy_d_b_snapshot_response(&wire::CopyDBSnapshotResult {
                d_b_snapshot: Some(db_snapshot_to_model(&snap)),
            }),
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // DB Cluster Snapshots
    // -------------------------------------------------------------------------

    async fn handle_create_db_cluster_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_d_b_cluster_snapshot_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let snapshot_id = input.d_b_cluster_snapshot_identifier;
        let cluster_id = input.d_b_cluster_identifier;
        let tags = wire_tags_to_domain(input.tags);
        let mut st = state.write().await;
        match st.create_db_cluster_snapshot(snapshot_id, &cluster_id, tags, account_id, region) {
            Ok(snap) => wire::serialize_create_d_b_cluster_snapshot_response(
                &wire::CreateDBClusterSnapshotResult {
                    d_b_cluster_snapshot: Some(db_cluster_snapshot_to_model(&snap)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_describe_db_cluster_snapshots(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
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
            d_b_cluster_snapshots: Some(wire::DBClusterSnapshotList::from(
                snaps
                    .iter()
                    .map(|s| db_cluster_snapshot_to_model(s))
                    .collect::<Vec<_>>(),
            )),
            marker: None,
        })
    }

    async fn handle_delete_db_cluster_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_d_b_cluster_snapshot_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let snapshot_id = input.d_b_cluster_snapshot_identifier;
        let mut st = state.write().await;
        match st.delete_db_cluster_snapshot(&snapshot_id) {
            Ok(snap) => wire::serialize_delete_d_b_cluster_snapshot_response(
                &wire::DeleteDBClusterSnapshotResult {
                    d_b_cluster_snapshot: Some(db_cluster_snapshot_to_model(&snap)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_copy_db_cluster_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_copy_d_b_cluster_snapshot_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let source_id = input.source_d_b_cluster_snapshot_identifier;
        let target_id = input.target_d_b_cluster_snapshot_identifier;
        let tags = wire_tags_to_domain(input.tags);
        let mut st = state.write().await;
        match st.copy_db_cluster_snapshot(&source_id, target_id, tags, account_id, region) {
            Ok(snap) => wire::serialize_copy_d_b_cluster_snapshot_response(
                &wire::CopyDBClusterSnapshotResult {
                    d_b_cluster_snapshot: Some(db_cluster_snapshot_to_model(&snap)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // DB Security Groups
    // -------------------------------------------------------------------------

    async fn handle_create_db_security_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_d_b_security_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.d_b_security_group_name;
        let description = input.d_b_security_group_description;
        let tags = wire_tags_to_domain(input.tags);
        let mut st = state.write().await;
        match st.create_db_security_group(name, description, tags, account_id, region) {
            Ok(sg) => wire::serialize_create_d_b_security_group_response(
                &wire::CreateDBSecurityGroupResult {
                    d_b_security_group: Some(db_security_group_to_model(&sg)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_describe_db_security_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_d_b_security_groups_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        let groups = st.describe_db_security_groups(input.d_b_security_group_name.as_deref());
        wire::serialize_describe_d_b_security_groups_response(&wire::DBSecurityGroupMessage {
            d_b_security_groups: Some(wire::DBSecurityGroups::from(
                groups
                    .iter()
                    .map(|g| db_security_group_to_model(g))
                    .collect::<Vec<_>>(),
            )),
            marker: None,
        })
    }

    async fn handle_delete_db_security_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_d_b_security_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.d_b_security_group_name;
        let mut st = state.write().await;
        match st.delete_db_security_group(&name) {
            Ok(()) => wire::serialize_delete_d_b_security_group_response(),
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Tags
    // -------------------------------------------------------------------------

    async fn handle_add_tags_to_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_add_tags_to_resource_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let resource_arn = input.resource_name;
        let tags = wire_tags_to_domain(Some(input.tags));
        let mut st = state.write().await;
        st.add_tags_to_resource(&resource_arn, tags);
        wire::serialize_add_tags_to_resource_response()
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let resource_arn = input.resource_name;
        let st = state.read().await;
        let tags = st.list_tags_for_resource(&resource_arn);
        wire::serialize_list_tags_for_resource_response(&wire::TagListMessage {
            tag_list: if tags.is_empty() {
                None
            } else {
                Some(wire::TagList::from(
                    tags.iter().map(|t| tag_to_model(t)).collect::<Vec<_>>(),
                ))
            },
        })
    }

    async fn handle_remove_tags_from_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_remove_tags_from_resource_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let resource_arn = input.resource_name;
        let tag_keys = input.tag_keys.items;
        let mut st = state.write().await;
        st.remove_tags_from_resource(&resource_arn, &tag_keys);
        wire::serialize_remove_tags_from_resource_response()
    }

    // -------------------------------------------------------------------------
    // Event Subscriptions
    // -------------------------------------------------------------------------

    async fn handle_create_event_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_event_subscription_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.subscription_name;
        let sns_topic_arn = input.sns_topic_arn;
        let source_type = input.source_type;
        let source_ids = input.source_ids.map(|l| l.items).unwrap_or_default();
        let event_categories = input.event_categories.map(|l| l.items).unwrap_or_default();
        let enabled = input.enabled.unwrap_or(true);

        let mut st = state.write().await;
        match st.create_event_subscription(
            name,
            sns_topic_arn,
            source_type,
            source_ids,
            event_categories,
            enabled,
            account_id,
            region,
        ) {
            Ok(sub) => wire::serialize_create_event_subscription_response(
                &wire::CreateEventSubscriptionResult {
                    event_subscription: Some(event_subscription_to_model(&sub)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_describe_event_subscriptions(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_event_subscriptions_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        let subs = st.describe_event_subscriptions(input.subscription_name.as_deref());
        wire::serialize_describe_event_subscriptions_response(&wire::EventSubscriptionsMessage {
            event_subscriptions_list: Some(wire::EventSubscriptionsList::from(
                subs.iter()
                    .map(|s| event_subscription_to_model(s))
                    .collect::<Vec<_>>(),
            )),
            marker: None,
        })
    }

    async fn handle_delete_event_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_event_subscription_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.subscription_name;
        let mut st = state.write().await;
        match st.delete_event_subscription(&name) {
            Ok(sub) => wire::serialize_delete_event_subscription_response(
                &wire::DeleteEventSubscriptionResult {
                    event_subscription: Some(event_subscription_to_model(&sub)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_modify_event_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_event_subscription_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.subscription_name;
        let st = state.read().await;
        let subs = st.describe_event_subscriptions(Some(&name));
        match subs.first() {
            Some(sub) => wire::serialize_modify_event_subscription_response(
                &wire::ModifyEventSubscriptionResult {
                    event_subscription: Some(event_subscription_to_model(sub)),
                },
            ),
            None => rds_error_response(&RdsError::not_found("SubscriptionNotFound", &name)),
        }
    }

    async fn handle_describe_events(&self) -> MockResponse {
        wire::serialize_describe_events_response(&wire::EventsMessage::default())
    }

    // -------------------------------------------------------------------------
    // Option Groups
    // -------------------------------------------------------------------------

    async fn handle_create_option_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_option_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.option_group_name;
        let engine_name = input.engine_name;
        let major_engine_version = input.major_engine_version;
        let description = input.option_group_description;
        let tags = wire_tags_to_domain(input.tags);
        let mut st = state.write().await;
        match st.create_option_group(
            name,
            engine_name,
            major_engine_version,
            description,
            tags,
            account_id,
            region,
        ) {
            Ok(og) => {
                wire::serialize_create_option_group_response(&wire::CreateOptionGroupResult {
                    option_group: Some(option_group_to_model(&og)),
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_describe_option_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_option_groups_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        let groups = st.describe_option_groups(input.option_group_name.as_deref());
        wire::serialize_describe_option_groups_response(&wire::OptionGroups {
            option_groups_list: Some(wire::OptionGroupsList::from(
                groups
                    .iter()
                    .map(|g| option_group_to_model(g))
                    .collect::<Vec<_>>(),
            )),
            marker: None,
        })
    }

    async fn handle_delete_option_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_option_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.option_group_name;
        let mut st = state.write().await;
        match st.delete_option_group(&name) {
            Ok(()) => wire::serialize_delete_option_group_response(),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_describe_option_group_options(
        &self,
        _params: &HashMap<String, String>,
    ) -> MockResponse {
        wire::serialize_describe_option_group_options_response(
            &wire::OptionGroupOptionsMessage::default(),
        )
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
        let engine = input.engine.unwrap_or_else(|| "mysql".to_string());
        let engine_version = input.engine_version.unwrap_or_else(|| "8.0".to_string());
        wire::serialize_describe_d_b_engine_versions_response(&wire::DBEngineVersionMessage {
            d_b_engine_versions: Some(wire::DBEngineVersionList::from(vec![
                wire::DBEngineVersion {
                    engine: Some(engine.clone()),
                    engine_version: Some(engine_version.clone()),
                    d_b_engine_description: Some(format!("{engine} Community Edition")),
                    d_b_engine_version_description: Some(format!("{engine} {engine_version}")),
                    ..Default::default()
                },
            ])),
            marker: None,
        })
    }

    async fn handle_describe_orderable_db_instance_options(
        &self,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_orderable_d_b_instance_options_request(params)
        {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let engine = if input.engine.is_empty() {
            "mysql".to_string()
        } else {
            input.engine
        };
        let engine_version = input.engine_version.unwrap_or_else(|| "8.0".to_string());
        wire::serialize_describe_orderable_d_b_instance_options_response(
            &wire::OrderableDBInstanceOptionsMessage {
                orderable_d_b_instance_options: Some(wire::OrderableDBInstanceOptionsList::from(
                    vec![wire::OrderableDBInstanceOption {
                        engine: Some(engine),
                        engine_version: Some(engine_version),
                        d_b_instance_class: Some("db.t3.micro".to_string()),
                        license_model: Some("general-public-license".to_string()),
                        multi_a_z_capable: Some(true),
                        read_replica_capable: Some(true),
                        ..Default::default()
                    }],
                )),
                marker: None,
            },
        )
    }

    async fn handle_describe_valid_db_instance_modifications(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_describe_valid_d_b_instance_modifications_request(params) {
                Ok(v) => v,
                Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
            };
        let st = state.read().await;
        match st.describe_db_instances(Some(&input.d_b_instance_identifier)) {
            Ok(_) => wire::serialize_describe_valid_d_b_instance_modifications_response(
                &wire::DescribeValidDBInstanceModificationsResult::default(),
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Global Clusters
    // -------------------------------------------------------------------------

    async fn handle_create_global_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_global_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.global_cluster_identifier;
        let engine = input.engine;
        let engine_version = input.engine_version;
        let database_name = input.database_name;
        let deletion_protection = input.deletion_protection.unwrap_or(false);
        let storage_encrypted = input.storage_encrypted.unwrap_or(false);
        let mut st = state.write().await;
        match st.create_global_cluster(
            identifier,
            engine,
            engine_version,
            database_name,
            deletion_protection,
            storage_encrypted,
            account_id,
            region,
        ) {
            Ok(gc) => {
                wire::serialize_create_global_cluster_response(&wire::CreateGlobalClusterResult {
                    global_cluster: Some(global_cluster_to_model(&gc)),
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_describe_global_clusters(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_global_clusters_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        let clusters = st.describe_global_clusters(input.global_cluster_identifier.as_deref());
        wire::serialize_describe_global_clusters_response(&wire::GlobalClustersMessage {
            global_clusters: Some(wire::GlobalClusterList::from(
                clusters
                    .iter()
                    .map(|gc| global_cluster_to_model(gc))
                    .collect::<Vec<_>>(),
            )),
            marker: None,
        })
    }

    async fn handle_delete_global_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_global_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.global_cluster_identifier;
        let mut st = state.write().await;
        match st.delete_global_cluster(&identifier) {
            Ok(gc) => {
                wire::serialize_delete_global_cluster_response(&wire::DeleteGlobalClusterResult {
                    global_cluster: Some(global_cluster_to_model(&gc)),
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_remove_from_global_cluster(
        &self,
        _state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_remove_from_global_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.global_cluster_identifier;
        wire::serialize_remove_from_global_cluster_response(&wire::RemoveFromGlobalClusterResult {
            global_cluster: Some(wire::GlobalCluster {
                global_cluster_identifier: Some(identifier),
                ..Default::default()
            }),
        })
    }

    // -------------------------------------------------------------------------
    // Export Tasks
    // -------------------------------------------------------------------------

    async fn handle_start_export_task(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_start_export_task_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let task_id = input.export_task_identifier;
        let source_arn = input.source_arn;
        let s3_bucket = input.s3_bucket_name;
        let s3_prefix = input.s3_prefix;
        let iam_role_arn = input.iam_role_arn;
        let kms_key_id = input.kms_key_id;
        let export_only = input.export_only.map(|l| l.items).unwrap_or_default();
        let mut st = state.write().await;
        match st.start_export_task(
            task_id,
            source_arn,
            s3_bucket,
            s3_prefix,
            iam_role_arn,
            kms_key_id,
            export_only,
        ) {
            Ok(task) => wire::serialize_start_export_task_response(&export_task_to_model(&task)),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_cancel_export_task(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_export_task_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let task_id = input.export_task_identifier;
        let mut st = state.write().await;
        match st.cancel_export_task(&task_id) {
            Ok(task) => wire::serialize_cancel_export_task_response(&export_task_to_model(&task)),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_describe_export_tasks(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_export_tasks_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        let tasks = st.describe_export_tasks(input.export_task_identifier.as_deref());
        wire::serialize_describe_export_tasks_response(&wire::ExportTasksMessage {
            export_tasks: Some(wire::ExportTasksList::from(
                tasks
                    .iter()
                    .map(|t| export_task_to_model(t))
                    .collect::<Vec<_>>(),
            )),
            marker: None,
        })
    }

    // -------------------------------------------------------------------------
    // DB Proxies
    // -------------------------------------------------------------------------

    async fn handle_create_db_proxy(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_d_b_proxy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.d_b_proxy_name;
        let engine_family = if input.engine_family.is_empty() {
            "MYSQL".to_string()
        } else {
            input.engine_family
        };
        let vpc_subnet_ids = input.vpc_subnet_ids.items;
        let vpc_security_group_ids = input
            .vpc_security_group_ids
            .map(|l| l.items)
            .unwrap_or_default();
        let role_arn = input.role_arn;
        let require_tls = input.require_t_l_s.unwrap_or(false);
        let idle_client_timeout = input.idle_client_timeout.unwrap_or(1800);
        let debug_logging = input.debug_logging.unwrap_or(false);
        let tags = wire_tags_to_domain(input.tags);
        let mut st = state.write().await;
        match st.create_db_proxy(
            name,
            engine_family,
            vpc_subnet_ids,
            vpc_security_group_ids,
            role_arn,
            require_tls,
            idle_client_timeout,
            debug_logging,
            tags,
            account_id,
            region,
        ) {
            Ok(proxy) => wire::serialize_create_d_b_proxy_response(&wire::CreateDBProxyResponse {
                d_b_proxy: Some(db_proxy_to_model(&proxy)),
            }),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_describe_db_proxies(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_d_b_proxies_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.d_b_proxy_name;
        let st = state.read().await;
        let proxies: Vec<&crate::types::DbProxy> = if let Some(n) = name.as_deref() {
            st.db_proxies.get(n).into_iter().collect()
        } else {
            st.db_proxies.values().collect()
        };
        wire::serialize_describe_d_b_proxies_response(&wire::DescribeDBProxiesResponse {
            d_b_proxies: Some(wire::DBProxyList::from(
                proxies
                    .iter()
                    .map(|p| db_proxy_to_model(p))
                    .collect::<Vec<_>>(),
            )),
            marker: None,
        })
    }

    async fn handle_delete_db_proxy(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_d_b_proxy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.d_b_proxy_name;
        let mut st = state.write().await;
        match st.db_proxies.remove(&name) {
            Some(proxy) => {
                wire::serialize_delete_d_b_proxy_response(&wire::DeleteDBProxyResponse {
                    d_b_proxy: Some(db_proxy_to_model(&proxy)),
                })
            }
            None => rds_error_response(&RdsError::not_found("DBProxy", &name)),
        }
    }

    // -------------------------------------------------------------------------
    // DB Log Files
    // -------------------------------------------------------------------------

    async fn handle_describe_db_log_files(
        &self,
        _state: &Arc<tokio::sync::RwLock<RdsState>>,
        _params: &HashMap<String, String>,
    ) -> MockResponse {
        wire::serialize_describe_d_b_log_files_response(&wire::DescribeDBLogFilesResponse::default())
    }

    async fn handle_download_db_log_file_portion(
        &self,
        _state: &Arc<tokio::sync::RwLock<RdsState>>,
        _params: &HashMap<String, String>,
    ) -> MockResponse {
        wire::serialize_download_d_b_log_file_portion_response(
            &wire::DownloadDBLogFilePortionDetails {
                additional_data_pending: Some(false),
                ..Default::default()
            },
        )
    }

    // -------------------------------------------------------------------------
    // Account Attributes / Certificates
    // -------------------------------------------------------------------------

    async fn handle_describe_account_attributes(&self) -> MockResponse {
        wire::serialize_describe_account_attributes_response(&wire::AccountAttributesMessage {
            account_quotas: Some(wire::AccountQuotaList::from(vec![wire::AccountQuota {
                account_quota_name: Some("DBInstances".to_string()),
                used: Some(0),
                max: Some(40),
            }])),
        })
    }

    async fn handle_describe_certificates(&self) -> MockResponse {
        wire::serialize_describe_certificates_response(&wire::CertificateMessage {
            certificates: Some(wire::CertificateList::from(vec![wire::Certificate {
                certificate_identifier: Some("rds-ca-2019".to_string()),
                certificate_type: Some("CA".to_string()),
                valid_from: Some("2019-09-19T17:10:40Z".to_string()),
                valid_till: Some("2024-08-22T17:08:50Z".to_string()),
                ..Default::default()
            }])),
            ..Default::default()
        })
    }

    // -------------------------------------------------------------------------
    // Role Associations
    // -------------------------------------------------------------------------

    async fn handle_add_role_to_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_add_role_to_d_b_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_cluster_identifier;
        let role_arn = input.role_arn;
        let mut st = state.write().await;
        match st.add_role_to_db_cluster(&identifier, role_arn) {
            Ok(()) => wire::serialize_add_role_to_d_b_cluster_response(),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_remove_role_from_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_remove_role_from_d_b_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_cluster_identifier;
        let role_arn = input.role_arn;
        let mut st = state.write().await;
        match st.remove_role_from_db_cluster(&identifier, &role_arn) {
            Ok(()) => wire::serialize_remove_role_from_d_b_cluster_response(),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_add_role_to_db_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_add_role_to_d_b_instance_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_instance_identifier;
        let role_arn = input.role_arn;
        let mut st = state.write().await;
        match st.add_role_to_db_instance(&identifier, role_arn) {
            Ok(()) => wire::serialize_add_role_to_d_b_instance_response(),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_remove_role_from_db_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_remove_role_from_d_b_instance_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_instance_identifier;
        let role_arn = input.role_arn;
        let mut st = state.write().await;
        match st.remove_role_from_db_instance(&identifier, &role_arn) {
            Ok(()) => wire::serialize_remove_role_from_d_b_instance_response(),
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Snapshot Attributes
    // -------------------------------------------------------------------------

    async fn handle_describe_db_snapshot_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_d_b_snapshot_attributes_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let snapshot_id = input.d_b_snapshot_identifier;
        let st = state.read().await;
        if !st.db_snapshots.contains_key(&snapshot_id) {
            return rds_error_response(&RdsError::not_found("DBSnapshot", &snapshot_id));
        }
        wire::serialize_describe_d_b_snapshot_attributes_response(
            &wire::DescribeDBSnapshotAttributesResult {
                d_b_snapshot_attributes_result: Some(wire::DBSnapshotAttributesResult {
                    d_b_snapshot_identifier: Some(snapshot_id),
                    ..Default::default()
                }),
            },
        )
    }

    async fn handle_modify_db_snapshot_attribute(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_d_b_snapshot_attribute_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let snapshot_id = input.d_b_snapshot_identifier;
        let st = state.read().await;
        if !st.db_snapshots.contains_key(&snapshot_id) {
            return rds_error_response(&RdsError::not_found("DBSnapshot", &snapshot_id));
        }
        wire::serialize_modify_d_b_snapshot_attribute_response(
            &wire::ModifyDBSnapshotAttributeResult {
                d_b_snapshot_attributes_result: Some(wire::DBSnapshotAttributesResult {
                    d_b_snapshot_identifier: Some(snapshot_id),
                    ..Default::default()
                }),
            },
        )
    }

    async fn handle_describe_db_cluster_snapshot_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_d_b_cluster_snapshot_attributes_request(params)
        {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let snapshot_id = input.d_b_cluster_snapshot_identifier;
        let st = state.read().await;
        if !st.db_cluster_snapshots.contains_key(&snapshot_id) {
            return rds_error_response(&RdsError::not_found("DBClusterSnapshot", &snapshot_id));
        }
        wire::serialize_describe_d_b_cluster_snapshot_attributes_response(
            &wire::DescribeDBClusterSnapshotAttributesResult {
                d_b_cluster_snapshot_attributes_result: Some(
                    wire::DBClusterSnapshotAttributesResult {
                        d_b_cluster_snapshot_identifier: Some(snapshot_id),
                        ..Default::default()
                    },
                ),
            },
        )
    }

    async fn handle_modify_db_cluster_snapshot_attribute(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_d_b_cluster_snapshot_attribute_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let snapshot_id = input.d_b_cluster_snapshot_identifier;
        let st = state.read().await;
        if !st.db_cluster_snapshots.contains_key(&snapshot_id) {
            return rds_error_response(&RdsError::not_found("DBClusterSnapshot", &snapshot_id));
        }
        wire::serialize_modify_d_b_cluster_snapshot_attribute_response(
            &wire::ModifyDBClusterSnapshotAttributeResult {
                d_b_cluster_snapshot_attributes_result: Some(
                    wire::DBClusterSnapshotAttributesResult {
                        d_b_cluster_snapshot_identifier: Some(snapshot_id),
                        ..Default::default()
                    },
                ),
            },
        )
    }

    // -------------------------------------------------------------------------
    // CopyDBParameterGroup
    // -------------------------------------------------------------------------

    async fn handle_copy_db_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_copy_d_b_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let source_name = input.source_d_b_parameter_group_identifier;
        let target_name = input.target_d_b_parameter_group_identifier;
        let target_description = input.target_d_b_parameter_group_description;
        let tags = wire_tags_to_domain(input.tags);
        let mut st = state.write().await;
        match st.copy_db_parameter_group(
            &source_name,
            target_name,
            target_description,
            tags,
            account_id,
            region,
        ) {
            Ok(pg) => wire::serialize_copy_d_b_parameter_group_response(
                &wire::CopyDBParameterGroupResult {
                    d_b_parameter_group: Some(db_parameter_group_to_model(&pg)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // DescribeDBInstanceAutomatedBackups
    // -------------------------------------------------------------------------

    async fn handle_describe_db_instance_automated_backups(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        _params: &HashMap<String, String>,
    ) -> MockResponse {
        let _st = state.read().await;
        wire::serialize_describe_d_b_instance_automated_backups_response(
            &wire::DBInstanceAutomatedBackupMessage::default(),
        )
    }

    // -------------------------------------------------------------------------
    // ModifyOptionGroup
    // -------------------------------------------------------------------------

    async fn handle_modify_option_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_option_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.option_group_name;
        let st = state.read().await;
        match st.option_groups.get(&name) {
            Some(og) => {
                wire::serialize_modify_option_group_response(&wire::ModifyOptionGroupResult {
                    option_group: Some(option_group_to_model(og)),
                })
            }
            None => rds_error_response(&RdsError::not_found("OptionGroup", &name)),
        }
    }

    // -------------------------------------------------------------------------
    // Blue/Green Deployments
    // -------------------------------------------------------------------------

    async fn handle_create_blue_green_deployment(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_blue_green_deployment_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.blue_green_deployment_name;
        let source = if input.source.is_empty() {
            None
        } else {
            Some(input.source)
        };
        let tags = wire_tags_to_domain(input.tags);
        let mut st = state.write().await;
        match st.create_blue_green_deployment(name, source, tags) {
            Ok(d) => wire::serialize_create_blue_green_deployment_response(
                &wire::CreateBlueGreenDeploymentResponse {
                    blue_green_deployment: Some(blue_green_deployment_to_model(&d)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_delete_blue_green_deployment(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_blue_green_deployment_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let id = input.blue_green_deployment_identifier;
        let mut st = state.write().await;
        match st.delete_blue_green_deployment(&id) {
            Ok(d) => wire::serialize_delete_blue_green_deployment_response(
                &wire::DeleteBlueGreenDeploymentResponse {
                    blue_green_deployment: Some(blue_green_deployment_to_model(&d)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_describe_blue_green_deployments(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_blue_green_deployments_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        let deployments =
            st.describe_blue_green_deployments(input.blue_green_deployment_identifier.as_deref());
        wire::serialize_describe_blue_green_deployments_response(
            &wire::DescribeBlueGreenDeploymentsResponse {
                blue_green_deployments: Some(wire::BlueGreenDeploymentList::from(
                    deployments
                        .iter()
                        .map(|d| blue_green_deployment_to_model(d))
                        .collect::<Vec<_>>(),
                )),
                marker: None,
            },
        )
    }

    async fn handle_switchover_blue_green_deployment(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_switchover_blue_green_deployment_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let id = input.blue_green_deployment_identifier;
        let mut st = state.write().await;
        match st.switchover_blue_green_deployment(&id) {
            Ok(d) => wire::serialize_switchover_blue_green_deployment_response(
                &wire::SwitchoverBlueGreenDeploymentResponse {
                    blue_green_deployment: Some(blue_green_deployment_to_model(&d)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // DB Shard Groups
    // -------------------------------------------------------------------------

    async fn handle_create_db_shard_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_d_b_shard_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_shard_group_identifier;
        let db_cluster_identifier = input.d_b_cluster_identifier;
        let max_acu = input.max_a_c_u;
        let min_acu = input.min_a_c_u;
        let publicly_accessible = input.publicly_accessible.unwrap_or(false);
        let tags = wire_tags_to_domain(input.tags);
        let mut st = state.write().await;
        match st.create_db_shard_group(
            identifier,
            db_cluster_identifier,
            max_acu,
            min_acu,
            publicly_accessible,
            tags,
            account_id,
            region,
        ) {
            Ok(sg) => {
                wire::serialize_create_d_b_shard_group_response(&db_shard_group_to_model(&sg))
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_describe_db_shard_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_d_b_shard_groups_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        let groups = st.describe_db_shard_groups(input.d_b_shard_group_identifier.as_deref());
        wire::serialize_describe_d_b_shard_groups_response(&wire::DescribeDBShardGroupsResponse {
            d_b_shard_groups: Some(wire::DBShardGroupsList::from(
                groups
                    .iter()
                    .map(|g| db_shard_group_to_model(g))
                    .collect::<Vec<_>>(),
            )),
            marker: None,
        })
    }

    // -------------------------------------------------------------------------
    // DB Proxy Targets and Target Groups
    // -------------------------------------------------------------------------

    async fn handle_register_db_proxy_targets(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_register_d_b_proxy_targets_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let db_proxy_name = input.d_b_proxy_name;
        let target_group_name = input
            .target_group_name
            .unwrap_or_else(|| "default".to_string());
        let db_instance_identifiers = input
            .d_b_instance_identifiers
            .map(|l| l.items)
            .unwrap_or_default();
        let db_cluster_identifiers = input
            .d_b_cluster_identifiers
            .map(|l| l.items)
            .unwrap_or_default();
        let mut st = state.write().await;
        match st.register_db_proxy_targets(
            &db_proxy_name,
            &target_group_name,
            db_instance_identifiers,
            db_cluster_identifiers,
            account_id,
            region,
        ) {
            Ok(targets) => wire::serialize_register_d_b_proxy_targets_response(
                &wire::RegisterDBProxyTargetsResponse {
                    d_b_proxy_targets: Some(wire::TargetList::from(
                        targets
                            .iter()
                            .map(db_proxy_target_to_model)
                            .collect::<Vec<_>>(),
                    )),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_deregister_db_proxy_targets(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_deregister_d_b_proxy_targets_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let db_proxy_name = input.d_b_proxy_name;
        let db_instance_identifiers = input
            .d_b_instance_identifiers
            .map(|l| l.items)
            .unwrap_or_default();
        let db_cluster_identifiers = input
            .d_b_cluster_identifiers
            .map(|l| l.items)
            .unwrap_or_default();
        let mut st = state.write().await;
        match st.deregister_db_proxy_targets(
            &db_proxy_name,
            db_instance_identifiers,
            db_cluster_identifiers,
        ) {
            Ok(()) => wire::serialize_deregister_d_b_proxy_targets_response(
                &wire::DeregisterDBProxyTargetsResponse {},
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_describe_db_proxy_targets(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_d_b_proxy_targets_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let db_proxy_name = input.d_b_proxy_name;
        let st = state.read().await;
        match st.describe_db_proxy_targets(&db_proxy_name) {
            Ok(targets) => wire::serialize_describe_d_b_proxy_targets_response(
                &wire::DescribeDBProxyTargetsResponse {
                    targets: Some(wire::TargetList::from(
                        targets
                            .iter()
                            .map(|t| db_proxy_target_to_model(t))
                            .collect::<Vec<_>>(),
                    )),
                    marker: None,
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_describe_db_proxy_target_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_d_b_proxy_target_groups_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let db_proxy_name = input.d_b_proxy_name;
        let st = state.read().await;
        match st.describe_db_proxy_target_groups(
            &db_proxy_name,
            input.target_group_name.as_deref(),
            account_id,
            region,
        ) {
            Ok(groups) => wire::serialize_describe_d_b_proxy_target_groups_response(
                &wire::DescribeDBProxyTargetGroupsResponse {
                    target_groups: Some(wire::TargetGroupList::from(
                        groups
                            .iter()
                            .map(db_proxy_target_group_to_model)
                            .collect::<Vec<_>>(),
                    )),
                    marker: None,
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_modify_db_proxy_target_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_d_b_proxy_target_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let db_proxy_name = input.d_b_proxy_name;
        let target_group_name = input.target_group_name;
        let mut st = state.write().await;
        match st.modify_db_proxy_target_group(
            &db_proxy_name,
            &target_group_name,
            account_id,
            region,
        ) {
            Ok(group) => wire::serialize_modify_d_b_proxy_target_group_response(
                &wire::ModifyDBProxyTargetGroupResponse {
                    d_b_proxy_target_group: Some(db_proxy_target_group_to_model(&group)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // DB Shard Group delete/modify/reboot
    // -------------------------------------------------------------------------

    async fn handle_delete_db_shard_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_d_b_shard_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_shard_group_identifier;
        let mut st = state.write().await;
        match st.delete_db_shard_group(&identifier) {
            Ok(sg) => {
                wire::serialize_delete_d_b_shard_group_response(&db_shard_group_to_model(&sg))
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_modify_db_shard_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_d_b_shard_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_shard_group_identifier;
        let mut st = state.write().await;
        match st.modify_db_shard_group(&identifier, input.max_a_c_u, input.min_a_c_u) {
            Ok(sg) => {
                wire::serialize_modify_d_b_shard_group_response(&db_shard_group_to_model(&sg))
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_reboot_db_shard_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_reboot_d_b_shard_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_shard_group_identifier;
        let st = state.read().await;
        match st.reboot_db_shard_group(&identifier) {
            Ok(sg) => {
                wire::serialize_reboot_d_b_shard_group_response(&db_shard_group_to_model(&sg))
            }
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // DB Cluster Endpoints
    // -------------------------------------------------------------------------

    async fn handle_create_db_cluster_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_d_b_cluster_endpoint_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_cluster_endpoint_identifier;
        let db_cluster_identifier = input.d_b_cluster_identifier;
        let endpoint_type = if input.endpoint_type.is_empty() {
            "READER".to_string()
        } else {
            input.endpoint_type
        };
        let static_members = input.static_members.map(|l| l.items).unwrap_or_default();
        let excluded_members = input.excluded_members.map(|l| l.items).unwrap_or_default();
        let mut st = state.write().await;
        match st.create_db_cluster_endpoint(
            identifier,
            db_cluster_identifier,
            endpoint_type,
            static_members,
            excluded_members,
            account_id,
            region,
        ) {
            Ok(ep) => wire::serialize_create_d_b_cluster_endpoint_response(
                &db_cluster_endpoint_to_model(&ep),
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_describe_db_cluster_endpoints(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_d_b_cluster_endpoints_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        let endpoints = st.describe_db_cluster_endpoints(
            input.d_b_cluster_identifier.as_deref(),
            input.d_b_cluster_endpoint_identifier.as_deref(),
        );
        wire::serialize_describe_d_b_cluster_endpoints_response(&wire::DBClusterEndpointMessage {
            d_b_cluster_endpoints: Some(wire::DBClusterEndpointList::from(
                endpoints
                    .iter()
                    .map(|ep| db_cluster_endpoint_to_model(ep))
                    .collect::<Vec<_>>(),
            )),
            marker: None,
        })
    }

    async fn handle_modify_db_cluster_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_d_b_cluster_endpoint_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_cluster_endpoint_identifier;
        let static_members = input.static_members.map(|l| l.items);
        let excluded_members = input.excluded_members.map(|l| l.items);
        let mut st = state.write().await;
        match st.modify_db_cluster_endpoint(&identifier, static_members, excluded_members) {
            Ok(ep) => wire::serialize_modify_d_b_cluster_endpoint_response(
                &db_cluster_endpoint_to_model(&ep),
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_delete_db_cluster_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_d_b_cluster_endpoint_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_cluster_endpoint_identifier;
        let mut st = state.write().await;
        match st.delete_db_cluster_endpoint(&identifier) {
            Ok(ep) => wire::serialize_delete_d_b_cluster_endpoint_response(
                &db_cluster_endpoint_to_model(&ep),
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // DB Proxy Endpoints
    // -------------------------------------------------------------------------

    async fn handle_create_db_proxy_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_d_b_proxy_endpoint_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.d_b_proxy_endpoint_name;
        let db_proxy_name = input.d_b_proxy_name;
        let vpc_subnet_ids = input.vpc_subnet_ids.items;
        let vpc_security_group_ids = input
            .vpc_security_group_ids
            .map(|l| l.items)
            .unwrap_or_default();
        let target_role = input.target_role;
        let mut st = state.write().await;
        match st.create_db_proxy_endpoint(
            name,
            db_proxy_name,
            vpc_subnet_ids,
            vpc_security_group_ids,
            target_role,
            account_id,
            region,
        ) {
            Ok(ep) => wire::serialize_create_d_b_proxy_endpoint_response(
                &wire::CreateDBProxyEndpointResponse {
                    d_b_proxy_endpoint: Some(db_proxy_endpoint_to_model(&ep)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_describe_db_proxy_endpoints(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_d_b_proxy_endpoints_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let st = state.read().await;
        let endpoints = st.describe_db_proxy_endpoints(
            input.d_b_proxy_name.as_deref(),
            input.d_b_proxy_endpoint_name.as_deref(),
        );
        wire::serialize_describe_d_b_proxy_endpoints_response(
            &wire::DescribeDBProxyEndpointsResponse {
                d_b_proxy_endpoints: Some(wire::DBProxyEndpointList::from(
                    endpoints
                        .iter()
                        .map(|ep| db_proxy_endpoint_to_model(ep))
                        .collect::<Vec<_>>(),
                )),
                marker: None,
            },
        )
    }

    async fn handle_modify_db_proxy_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_d_b_proxy_endpoint_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.d_b_proxy_endpoint_name;
        let vpc_security_group_ids = input.vpc_security_group_ids.map(|l| l.items);
        let mut st = state.write().await;
        match st.modify_db_proxy_endpoint(&name, vpc_security_group_ids) {
            Ok(ep) => wire::serialize_modify_d_b_proxy_endpoint_response(
                &wire::ModifyDBProxyEndpointResponse {
                    d_b_proxy_endpoint: Some(db_proxy_endpoint_to_model(&ep)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_delete_db_proxy_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_d_b_proxy_endpoint_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.d_b_proxy_endpoint_name;
        let mut st = state.write().await;
        match st.delete_db_proxy_endpoint(&name) {
            Ok(ep) => wire::serialize_delete_d_b_proxy_endpoint_response(
                &wire::DeleteDBProxyEndpointResponse {
                    d_b_proxy_endpoint: Some(db_proxy_endpoint_to_model(&ep)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Copy operations
    // -------------------------------------------------------------------------

    async fn handle_copy_db_cluster_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_copy_d_b_cluster_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let source_name = input.source_d_b_cluster_parameter_group_identifier;
        let target_name = input.target_d_b_cluster_parameter_group_identifier;
        let target_description = input.target_d_b_cluster_parameter_group_description;
        let tags = wire_tags_to_domain(input.tags);
        let mut st = state.write().await;
        match st.copy_db_cluster_parameter_group(
            &source_name,
            target_name,
            target_description,
            tags,
            account_id,
            region,
        ) {
            Ok(pg) => wire::serialize_copy_d_b_cluster_parameter_group_response(
                &wire::CopyDBClusterParameterGroupResult {
                    d_b_cluster_parameter_group: Some(db_cluster_parameter_group_to_model(&pg)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_copy_option_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_copy_option_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let source_name = input.source_option_group_identifier;
        let target_name = input.target_option_group_identifier;
        let target_description = input.target_option_group_description;
        let tags = wire_tags_to_domain(input.tags);
        let mut st = state.write().await;
        match st.copy_option_group(
            &source_name,
            target_name,
            target_description,
            tags,
            account_id,
            region,
        ) {
            Ok(og) => wire::serialize_copy_option_group_response(&wire::CopyOptionGroupResult {
                option_group: Some(option_group_to_model(&og)),
            }),
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Event subscription source identifiers
    // -------------------------------------------------------------------------

    async fn handle_add_source_identifier_to_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_add_source_identifier_to_subscription_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let subscription_name = input.subscription_name;
        let source_identifier = input.source_identifier;
        let mut st = state.write().await;
        match st.add_source_identifier_to_subscription(&subscription_name, source_identifier) {
            Ok(sub) => wire::serialize_add_source_identifier_to_subscription_response(
                &wire::AddSourceIdentifierToSubscriptionResult {
                    event_subscription: Some(event_subscription_to_model(&sub)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_remove_source_identifier_from_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_remove_source_identifier_from_subscription_request(params) {
                Ok(v) => v,
                Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
            };
        let subscription_name = input.subscription_name;
        let source_identifier = input.source_identifier;
        let mut st = state.write().await;
        match st.remove_source_identifier_from_subscription(&subscription_name, &source_identifier)
        {
            Ok(sub) => wire::serialize_remove_source_identifier_from_subscription_response(
                &wire::RemoveSourceIdentifierFromSubscriptionResult {
                    event_subscription: Some(event_subscription_to_model(&sub)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // DB Security Group ingress
    // -------------------------------------------------------------------------

    async fn handle_authorize_db_security_group_ingress(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_authorize_d_b_security_group_ingress_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.d_b_security_group_name;
        let mut st = state.write().await;
        match st.authorize_db_security_group_ingress(
            &name,
            input.c_i_d_r_i_p,
            input.e_c2_security_group_name,
            input.e_c2_security_group_id,
            input.e_c2_security_group_owner_id,
        ) {
            Ok(sg) => wire::serialize_authorize_d_b_security_group_ingress_response(
                &wire::AuthorizeDBSecurityGroupIngressResult {
                    d_b_security_group: Some(db_security_group_to_model(&sg)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_revoke_db_security_group_ingress(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_revoke_d_b_security_group_ingress_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.d_b_security_group_name;
        let mut st = state.write().await;
        match st.revoke_db_security_group_ingress(
            &name,
            input.c_i_d_r_i_p.as_deref(),
            input.e_c2_security_group_name.as_deref(),
            input.e_c2_security_group_id.as_deref(),
        ) {
            Ok(sg) => wire::serialize_revoke_d_b_security_group_ingress_response(
                &wire::RevokeDBSecurityGroupIngressResult {
                    d_b_security_group: Some(db_security_group_to_model(&sg)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Global Cluster operations
    // -------------------------------------------------------------------------

    async fn handle_modify_global_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_global_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.global_cluster_identifier;
        let mut st = state.write().await;
        match st.modify_global_cluster(
            &identifier,
            input.new_global_cluster_identifier,
            input.deletion_protection,
            input.engine_version,
        ) {
            Ok(gc) => {
                wire::serialize_modify_global_cluster_response(&wire::ModifyGlobalClusterResult {
                    global_cluster: Some(global_cluster_to_model(&gc)),
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_failover_global_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_failover_global_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.global_cluster_identifier;
        let st = state.read().await;
        match st.failover_global_cluster(&identifier) {
            Ok(gc) => wire::serialize_failover_global_cluster_response(
                &wire::FailoverGlobalClusterResult {
                    global_cluster: Some(global_cluster_to_model(&gc)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_switchover_global_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_switchover_global_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.global_cluster_identifier;
        let st = state.read().await;
        match st.switchover_global_cluster(&identifier) {
            Ok(gc) => wire::serialize_switchover_global_cluster_response(
                &wire::SwitchoverGlobalClusterResult {
                    global_cluster: Some(global_cluster_to_model(&gc)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // DB Cluster reboot
    // -------------------------------------------------------------------------

    async fn handle_reboot_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_reboot_d_b_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_cluster_identifier;
        let st = state.read().await;
        match st.reboot_db_cluster(&identifier) {
            Ok(cluster) => {
                wire::serialize_reboot_d_b_cluster_response(&wire::RebootDBClusterResult {
                    d_b_cluster: Some(db_cluster_to_model(&cluster)),
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Modify DB Proxy
    // -------------------------------------------------------------------------

    async fn handle_modify_db_proxy(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_d_b_proxy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.d_b_proxy_name;
        let security_groups = input.security_groups.map(|l| l.items);
        let mut st = state.write().await;
        match st.modify_db_proxy(
            &name,
            input.new_d_b_proxy_name,
            input.require_t_l_s,
            input.idle_client_timeout,
            input.debug_logging,
            input.role_arn,
            security_groups,
        ) {
            Ok(proxy) => wire::serialize_modify_d_b_proxy_response(&wire::ModifyDBProxyResponse {
                d_b_proxy: Some(db_proxy_to_model(&proxy)),
            }),
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Modify DB Snapshot
    // -------------------------------------------------------------------------

    async fn handle_modify_db_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_d_b_snapshot_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let snapshot_id = input.d_b_snapshot_identifier;
        let mut st = state.write().await;
        match st.modify_db_snapshot(&snapshot_id, input.engine_version, input.option_group_name) {
            Ok(snap) => {
                wire::serialize_modify_d_b_snapshot_response(&wire::ModifyDBSnapshotResult {
                    d_b_snapshot: Some(db_snapshot_to_model(&snap)),
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Switchover Read Replica
    // -------------------------------------------------------------------------

    async fn handle_switchover_read_replica(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_switchover_read_replica_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_instance_identifier;
        let st = state.read().await;
        match st.switchover_read_replica(&identifier) {
            Ok(inst) => wire::serialize_switchover_read_replica_response(
                &wire::SwitchoverReadReplicaResult {
                    d_b_instance: Some(db_instance_to_model(&inst)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Apply Pending Maintenance Action
    // -------------------------------------------------------------------------

    async fn handle_apply_pending_maintenance_action(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_apply_pending_maintenance_action_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let resource_identifier = input.resource_identifier;
        let apply_action = input.apply_action;
        let opt_in_type = input.opt_in_type;
        let st = state.read().await;
        match st.apply_pending_maintenance_action(&resource_identifier) {
            Ok(_) => {
                let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
                wire::serialize_apply_pending_maintenance_action_response(
                    &wire::ApplyPendingMaintenanceActionResult {
                        resource_pending_maintenance_actions: Some(
                            wire::ResourcePendingMaintenanceActions {
                                resource_identifier: Some(resource_identifier),
                                pending_maintenance_action_details: Some(
                                    wire::PendingMaintenanceActionDetails::from(vec![
                                        wire::PendingMaintenanceAction {
                                            action: Some(apply_action),
                                            opt_in_status: Some(opt_in_type),
                                            current_apply_date: Some(now),
                                            ..Default::default()
                                        },
                                    ]),
                                ),
                            },
                        ),
                    },
                )
            }
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Describe-only operations
    // -------------------------------------------------------------------------

    async fn handle_describe_engine_default_cluster_parameters(
        &self,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_describe_engine_default_cluster_parameters_request(params) {
                Ok(v) => v,
                Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
            };
        let db_parameter_group_family = if input.d_b_parameter_group_family.is_empty() {
            "aurora-mysql8.0".to_string()
        } else {
            input.d_b_parameter_group_family
        };
        wire::serialize_describe_engine_default_cluster_parameters_response(
            &wire::DescribeEngineDefaultClusterParametersResult {
                engine_defaults: Some(wire::EngineDefaults {
                    d_b_parameter_group_family: Some(db_parameter_group_family),
                    ..Default::default()
                }),
            },
        )
    }

    async fn handle_describe_engine_default_parameters(
        &self,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_engine_default_parameters_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let db_parameter_group_family = if input.d_b_parameter_group_family.is_empty() {
            "mysql8.0".to_string()
        } else {
            input.d_b_parameter_group_family
        };
        wire::serialize_describe_engine_default_parameters_response(
            &wire::DescribeEngineDefaultParametersResult {
                engine_defaults: Some(wire::EngineDefaults {
                    d_b_parameter_group_family: Some(db_parameter_group_family),
                    ..Default::default()
                }),
            },
        )
    }

    async fn handle_describe_event_categories(
        &self,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_event_categories_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let source_type = input.source_type;
        let categories = match source_type.as_deref() {
            Some("db-instance") | None => vec![wire::EventCategoriesMap {
                source_type: Some("db-instance".to_string()),
                event_categories: Some(wire::EventCategoriesList::from(vec![
                    "availability".to_string(),
                    "backup".to_string(),
                    "configuration change".to_string(),
                    "creation".to_string(),
                    "deletion".to_string(),
                    "failover".to_string(),
                    "failure".to_string(),
                    "maintenance".to_string(),
                    "notification".to_string(),
                    "read replica".to_string(),
                    "recovery".to_string(),
                    "restoration".to_string(),
                ])),
            }],
            _ => vec![],
        };
        wire::serialize_describe_event_categories_response(&wire::EventCategoriesMessage {
            event_categories_map_list: Some(wire::EventCategoriesMapList::from(categories)),
        })
    }

    async fn handle_describe_pending_maintenance_actions(
        &self,
        _params: &HashMap<String, String>,
    ) -> MockResponse {
        wire::serialize_describe_pending_maintenance_actions_response(
            &wire::PendingMaintenanceActionsMessage::default(),
        )
    }

    async fn handle_describe_source_regions(
        &self,
        _params: &HashMap<String, String>,
    ) -> MockResponse {
        let regions = [
            "us-east-1",
            "us-east-2",
            "us-west-1",
            "us-west-2",
            "eu-west-1",
            "eu-west-2",
            "eu-west-3",
            "eu-central-1",
            "ap-southeast-1",
            "ap-southeast-2",
            "ap-northeast-1",
            "ap-northeast-2",
            "ap-south-1",
            "sa-east-1",
            "ca-central-1",
        ];
        wire::serialize_describe_source_regions_response(&wire::SourceRegionMessage {
            source_regions: Some(wire::SourceRegionList::from(
                regions
                    .iter()
                    .map(|r| wire::SourceRegion {
                        region_name: Some(r.to_string()),
                        endpoint: Some(format!("https://rds.{r}.amazonaws.com")),
                        status: Some("available".to_string()),
                        ..Default::default()
                    })
                    .collect::<Vec<_>>(),
            )),
            marker: None,
        })
    }

    // -------------------------------------------------------------------------
    // Backtrack DB Cluster
    // -------------------------------------------------------------------------

    async fn handle_backtrack_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_backtrack_d_b_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_cluster_identifier;
        let st = state.read().await;
        match st.backtrack_db_cluster(&identifier) {
            Ok(_cluster) => {
                let backtrack_id = uuid::Uuid::new_v4().to_string();
                let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
                wire::serialize_backtrack_d_b_cluster_response(&wire::DBClusterBacktrack {
                    backtrack_identifier: Some(backtrack_id),
                    d_b_cluster_identifier: Some(identifier),
                    backtrack_to: Some(now.clone()),
                    backtracked_from: Some(now.clone()),
                    backtrack_request_creation_time: Some(now),
                    status: Some("applying".to_string()),
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // HTTP endpoint (Aurora Serverless)
    // -------------------------------------------------------------------------

    async fn handle_enable_http_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_enable_http_endpoint_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let resource_arn = input.resource_arn;
        let identifier = resource_arn
            .split(':')
            .next_back()
            .unwrap_or(&resource_arn)
            .to_string();
        let mut st = state.write().await;
        match st.enable_http_endpoint(&identifier) {
            Ok(_) => {
                wire::serialize_enable_http_endpoint_response(&wire::EnableHttpEndpointResponse {
                    resource_arn: Some(resource_arn),
                    http_endpoint_enabled: Some(true),
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_disable_http_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_disable_http_endpoint_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let resource_arn = input.resource_arn;
        let identifier = resource_arn
            .split(':')
            .next_back()
            .unwrap_or(&resource_arn)
            .to_string();
        let mut st = state.write().await;
        match st.disable_http_endpoint(&identifier) {
            Ok(_) => {
                wire::serialize_disable_http_endpoint_response(&wire::DisableHttpEndpointResponse {
                    resource_arn: Some(resource_arn),
                    http_endpoint_enabled: Some(false),
                })
            }
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Modify current DB cluster capacity
    // -------------------------------------------------------------------------

    async fn handle_modify_current_db_cluster_capacity(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_current_d_b_cluster_capacity_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_cluster_identifier;
        let capacity = input.capacity.unwrap_or(1);
        let st = state.read().await;
        match st.modify_current_db_cluster_capacity(&identifier) {
            Ok(_) => wire::serialize_modify_current_d_b_cluster_capacity_response(
                &wire::DBClusterCapacityInfo {
                    d_b_cluster_identifier: Some(identifier),
                    pending_capacity: Some(capacity),
                    current_capacity: Some(capacity),
                    seconds_before_timeout: Some(300),
                    timeout_action: Some("ForceApplyCapacityChange".to_string()),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Restore from S3
    // -------------------------------------------------------------------------

    async fn handle_restore_db_cluster_from_s3(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_restore_d_b_cluster_from_s3_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_cluster_identifier;
        let engine = if input.engine.is_empty() {
            "aurora-mysql".to_string()
        } else {
            input.engine
        };
        let master_username = if input.master_username.is_empty() {
            None
        } else {
            Some(input.master_username)
        };
        let mut st = state.write().await;
        match st.restore_db_cluster_from_s3(identifier, engine, master_username, account_id, region)
        {
            Ok(cluster) => wire::serialize_restore_d_b_cluster_from_s3_response(
                &wire::RestoreDBClusterFromS3Result {
                    d_b_cluster: Some(db_cluster_to_model(&cluster)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    async fn handle_restore_db_instance_from_s3(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_restore_d_b_instance_from_s3_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.d_b_instance_identifier;
        let db_instance_class = if input.d_b_instance_class.is_empty() {
            "db.t3.micro".to_string()
        } else {
            input.d_b_instance_class
        };
        let engine = if input.engine.is_empty() {
            "mysql".to_string()
        } else {
            input.engine
        };
        let master_username = input.master_username;
        let mut st = state.write().await;
        match st.restore_db_instance_from_s3(
            identifier,
            db_instance_class,
            engine,
            master_username,
            account_id,
            region,
        ) {
            Ok(inst) => wire::serialize_restore_d_b_instance_from_s3_response(
                &wire::RestoreDBInstanceFromS3Result {
                    d_b_instance: Some(db_instance_to_model(&inst)),
                },
            ),
            Err(e) => rds_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Describe operations returning empty lists
    // -------------------------------------------------------------------------

    async fn handle_describe_db_cluster_automated_backups(
        &self,
        _params: &HashMap<String, String>,
    ) -> MockResponse {
        wire::serialize_describe_d_b_cluster_automated_backups_response(
            &wire::DBClusterAutomatedBackupMessage::default(),
        )
    }

    async fn handle_describe_db_cluster_backtracks(
        &self,
        _params: &HashMap<String, String>,
    ) -> MockResponse {
        wire::serialize_describe_d_b_cluster_backtracks_response(
            &wire::DBClusterBacktrackMessage::default(),
        )
    }

    async fn handle_describe_db_major_engine_versions(
        &self,
        _params: &HashMap<String, String>,
    ) -> MockResponse {
        wire::serialize_describe_d_b_major_engine_versions_response(
            &wire::DescribeDBMajorEngineVersionsResponse::default(),
        )
    }

    async fn handle_describe_reserved_db_instances(
        &self,
        _params: &HashMap<String, String>,
    ) -> MockResponse {
        wire::serialize_describe_reserved_d_b_instances_response(
            &wire::ReservedDBInstanceMessage::default(),
        )
    }

    async fn handle_describe_reserved_db_instances_offerings(
        &self,
        _params: &HashMap<String, String>,
    ) -> MockResponse {
        wire::serialize_describe_reserved_d_b_instances_offerings_response(
            &wire::ReservedDBInstancesOfferingMessage::default(),
        )
    }

    async fn handle_describe_db_recommendations(
        &self,
        _params: &HashMap<String, String>,
    ) -> MockResponse {
        wire::serialize_describe_d_b_recommendations_response(
            &wire::DBRecommendationsMessage::default(),
        )
    }

    async fn handle_describe_db_snapshot_tenant_databases(
        &self,
        _params: &HashMap<String, String>,
    ) -> MockResponse {
        wire::serialize_describe_d_b_snapshot_tenant_databases_response(
            &wire::DBSnapshotTenantDatabasesMessage::default(),
        )
    }

    // -------------------------------------------------------------------------
    // Activity streams
    // -------------------------------------------------------------------------

    async fn handle_start_activity_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_start_activity_stream_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let resource_arn = input.resource_arn;
        let kms_key_id = input.kms_key_id;
        let mode = input.mode;
        let identifier = resource_arn
            .split(':')
            .next_back()
            .unwrap_or("")
            .to_string();
        let st = state.read().await;
        if !st.db_clusters.contains_key(&identifier) && !st.db_instances.contains_key(&identifier) {
            return rds_error_response(&RdsError::not_found("Resource", &identifier));
        }
        let kinesis_stream = format!("aws-rds-das-{identifier}");
        wire::serialize_start_activity_stream_response(&wire::StartActivityStreamResponse {
            kinesis_stream_name: Some(kinesis_stream),
            kms_key_id: Some(kms_key_id),
            status: Some("starting".to_string()),
            mode: Some(mode),
            apply_immediately: Some(true),
            ..Default::default()
        })
    }

    async fn handle_stop_activity_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_stop_activity_stream_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let resource_arn = input.resource_arn;
        let identifier = resource_arn
            .split(':')
            .next_back()
            .unwrap_or("")
            .to_string();
        let st = state.read().await;
        if !st.db_clusters.contains_key(&identifier) && !st.db_instances.contains_key(&identifier) {
            return rds_error_response(&RdsError::not_found("Resource", &identifier));
        }
        let kinesis_stream = format!("aws-rds-das-{identifier}");
        wire::serialize_stop_activity_stream_response(&wire::StopActivityStreamResponse {
            kinesis_stream_name: Some(kinesis_stream),
            status: Some("stopping".to_string()),
            ..Default::default()
        })
    }

    async fn handle_modify_activity_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_activity_stream_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let resource_arn = match input.resource_arn {
            Some(v) => v,
            None => {
                return MockResponse::error(400, "InvalidParameterValue", "Missing ResourceArn");
            }
        };
        let identifier = resource_arn
            .split(':')
            .next_back()
            .unwrap_or("")
            .to_string();
        let st = state.read().await;
        if !st.db_clusters.contains_key(&identifier) && !st.db_instances.contains_key(&identifier) {
            return rds_error_response(&RdsError::not_found("Resource", &identifier));
        }
        let kinesis_stream = format!("aws-rds-das-{identifier}");
        wire::serialize_modify_activity_stream_response(&wire::ModifyActivityStreamResponse {
            kinesis_stream_name: Some(kinesis_stream),
            status: Some("started".to_string()),
            mode: Some("async".to_string()),
            ..Default::default()
        })
    }

    // -------------------------------------------------------------------------
    // Automated backup replication
    // -------------------------------------------------------------------------

    async fn handle_start_db_instance_automated_backups_replication(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_start_d_b_instance_automated_backups_replication_request(
            params,
        ) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let source_db_instance_arn = input.source_d_b_instance_arn;
        let backup_retention_period = input.backup_retention_period.unwrap_or(7);
        let identifier = source_db_instance_arn
            .split(':')
            .next_back()
            .unwrap_or("")
            .to_string();
        let st = state.read().await;
        let inst = st
            .db_instances
            .get(&identifier)
            .cloned()
            .unwrap_or_else(|| crate::types::DbInstance {
                identifier: identifier.clone(),
                db_instance_class: "db.t3.micro".to_string(),
                engine: "mysql".to_string(),
                engine_version: "8.0".to_string(),
                status: "replicating".to_string(),
                master_username: None,
                db_name: None,
                endpoint_address: None,
                port: Some(3306),
                multi_az: false,
                storage_type: Some("gp2".to_string()),
                allocated_storage: 20,
                db_subnet_group_name: None,
                vpc_security_group_ids: vec![],
                db_parameter_group_names: vec![],
                availability_zone: None,
                publicly_accessible: false,
                auto_minor_version_upgrade: true,
                backup_retention_period,
                db_cluster_identifier: None,
                arn: format!("arn:aws:rds:{region}:{account_id}:db:{identifier}"),
                tags: vec![],
                instance_create_time: None,
                license_model: None,
                iops: None,
                deletion_protection: false,
                copy_tags_to_snapshot: false,
                monitoring_interval: None,
                performance_insights_enabled: false,
                storage_encrypted: false,
                kms_key_id: None,
                ca_certificate_identifier: None,
                secondary_availability_zone: None,
                associated_roles: vec![],
            });
        wire::serialize_start_d_b_instance_automated_backups_replication_response(
            &wire::StartDBInstanceAutomatedBackupsReplicationResult {
                d_b_instance_automated_backup: Some(db_instance_to_automated_backup_model(
                    &inst,
                    &source_db_instance_arn,
                )),
            },
        )
    }

    async fn handle_stop_db_instance_automated_backups_replication(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_stop_d_b_instance_automated_backups_replication_request(params)
            {
                Ok(v) => v,
                Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
            };
        let source_db_instance_arn = input.source_d_b_instance_arn;
        let identifier = source_db_instance_arn
            .split(':')
            .next_back()
            .unwrap_or("")
            .to_string();
        let st = state.read().await;
        if let Some(inst) = st.db_instances.get(&identifier) {
            wire::serialize_stop_d_b_instance_automated_backups_replication_response(
                &wire::StopDBInstanceAutomatedBackupsReplicationResult {
                    d_b_instance_automated_backup: Some(db_instance_to_automated_backup_model(
                        inst,
                        &source_db_instance_arn,
                    )),
                },
            )
        } else {
            rds_error_response(&RdsError::not_found("DBInstance", &identifier))
        }
    }

    async fn handle_delete_db_instance_automated_backup(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_d_b_instance_automated_backup_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let dbi_resource_id = match input.dbi_resource_id {
            Some(v) => v,
            None => {
                return MockResponse::error(400, "InvalidParameterValue", "Missing DbiResourceId");
            }
        };
        let st = state.read().await;
        let inst = st
            .db_instances
            .values()
            .find(|i| i.identifier == dbi_resource_id || i.arn.contains(&dbi_resource_id))
            .cloned();
        if let Some(inst) = inst {
            let arn = inst.arn.replace(":db:", ":auto-backup:");
            wire::serialize_delete_d_b_instance_automated_backup_response(
                &wire::DeleteDBInstanceAutomatedBackupResult {
                    d_b_instance_automated_backup: Some(db_instance_to_automated_backup_model(
                        &inst, &arn,
                    )),
                },
            )
        } else {
            wire::serialize_delete_d_b_instance_automated_backup_response(
                &wire::DeleteDBInstanceAutomatedBackupResult {
                    d_b_instance_automated_backup: Some(wire::DBInstanceAutomatedBackup {
                        dbi_resource_id: Some(dbi_resource_id),
                        status: Some("deleted".to_string()),
                        ..Default::default()
                    }),
                },
            )
        }
    }

    async fn handle_delete_db_cluster_automated_backup(
        &self,
        state: &Arc<tokio::sync::RwLock<RdsState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_d_b_cluster_automated_backup_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let db_cluster_identifier = input.db_cluster_resource_id;
        if db_cluster_identifier.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameterValue",
                "Missing DbClusterResourceId or DBClusterIdentifier",
            );
        }
        let st = state.read().await;
        let cluster = st.db_clusters.get(&db_cluster_identifier).cloned();
        if let Some(cluster) = cluster {
            wire::serialize_delete_d_b_cluster_automated_backup_response(
                &wire::DeleteDBClusterAutomatedBackupResult {
                    d_b_cluster_automated_backup: Some(wire::DBClusterAutomatedBackup {
                        d_b_cluster_identifier: Some(cluster.identifier),
                        d_b_cluster_arn: Some(cluster.arn),
                        status: Some("deleted".to_string()),
                        engine: Some(cluster.engine),
                        ..Default::default()
                    }),
                },
            )
        } else {
            wire::serialize_delete_d_b_cluster_automated_backup_response(
                &wire::DeleteDBClusterAutomatedBackupResult {
                    d_b_cluster_automated_backup: Some(wire::DBClusterAutomatedBackup {
                        d_b_cluster_identifier: Some(db_cluster_identifier),
                        status: Some("deleted".to_string()),
                        ..Default::default()
                    }),
                },
            )
        }
    }

    // -------------------------------------------------------------------------
    // Modify Certificates
    // -------------------------------------------------------------------------

    async fn handle_modify_certificates(&self, params: &HashMap<String, String>) -> MockResponse {
        let input = match wire::deserialize_modify_certificates_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let certificate_identifier = input
            .certificate_identifier
            .unwrap_or_else(|| "rds-ca-rsa2048-g1".to_string());
        wire::serialize_modify_certificates_response(&wire::ModifyCertificatesResult {
            certificate: Some(wire::Certificate {
                certificate_identifier: Some(certificate_identifier.clone()),
                certificate_type: Some("CA".to_string()),
                thumbprint: Some("aaaa1234".to_string()),
                valid_from: Some("2024-01-01T00:00:00Z".to_string()),
                valid_till: Some("2028-01-01T00:00:00Z".to_string()),
                certificate_arn: Some(format!("arn:aws:rds:::{certificate_identifier}")),
                customer_override: Some(false),
                ..Default::default()
            }),
        })
    }
}

// -------------------------------------------------------------------------
// Error helper
// -------------------------------------------------------------------------

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

fn rds_error_response(e: &RdsError) -> MockResponse {
    let (status, error_type) = match e {
        RdsError::AlreadyExists { resource_type, .. } => {
            (400, format!("{resource_type}AlreadyExists"))
        }
        RdsError::NotFound { resource_type, .. } => (404, format!("{resource_type}NotFound")),
        RdsError::InvalidParameter(_) => (400, "InvalidParameterValue".to_string()),
        RdsError::InvalidState(_) => (400, "InvalidDBInstanceState".to_string()),
    };
    MockResponse::error(status, &error_type, &e.to_string())
}

// -------------------------------------------------------------------------
// Conversion functions: domain types → wire model types
// -------------------------------------------------------------------------

fn tag_to_model(t: &Tag) -> wire::Tag {
    wire::Tag {
        key: Some(t.key.clone()),
        value: Some(t.value.clone()),
    }
}

fn tags_to_model(tags: &[Tag]) -> wire::TagList {
    wire::TagList::from(tags.iter().map(tag_to_model).collect::<Vec<_>>())
}

fn db_instance_to_model(i: &crate::types::DbInstance) -> wire::DBInstance {
    wire::DBInstance {
        d_b_instance_identifier: Some(i.identifier.clone()),
        d_b_instance_class: Some(i.db_instance_class.clone()),
        engine: Some(i.engine.clone()),
        engine_version: Some(i.engine_version.clone()),
        d_b_instance_status: Some(i.status.clone()),
        master_username: i.master_username.clone(),
        d_b_name: i.db_name.clone(),
        endpoint: Some(wire::Endpoint {
            address: i.endpoint_address.clone(),
            port: i.port,
            ..Default::default()
        }),
        multi_a_z: Some(i.multi_az),
        storage_type: i.storage_type.clone(),
        allocated_storage: Some(i.allocated_storage),
        d_b_subnet_group: i
            .db_subnet_group_name
            .as_ref()
            .map(|name| wire::DBSubnetGroup {
                d_b_subnet_group_name: Some(name.clone()),
                ..Default::default()
            }),
        vpc_security_groups: Some(wire::VpcSecurityGroupMembershipList::from(
            i.vpc_security_group_ids
                .iter()
                .map(|id| wire::VpcSecurityGroupMembership {
                    vpc_security_group_id: Some(id.clone()),
                    status: Some("active".to_string()),
                })
                .collect::<Vec<_>>(),
        )),
        d_b_parameter_groups: Some(wire::DBParameterGroupStatusList::from(
            i.db_parameter_group_names
                .iter()
                .map(|n| wire::DBParameterGroupStatus {
                    d_b_parameter_group_name: Some(n.clone()),
                    parameter_apply_status: Some("in-sync".to_string()),
                })
                .collect::<Vec<_>>(),
        )),
        availability_zone: i.availability_zone.clone(),
        publicly_accessible: Some(i.publicly_accessible),
        auto_minor_version_upgrade: Some(i.auto_minor_version_upgrade),
        backup_retention_period: Some(i.backup_retention_period),
        d_b_cluster_identifier: i.db_cluster_identifier.clone(),
        d_b_instance_arn: Some(i.arn.clone()),
        instance_create_time: i.instance_create_time.clone(),
        license_model: i.license_model.clone(),
        iops: i.iops,
        deletion_protection: Some(i.deletion_protection),
        copy_tags_to_snapshot: Some(i.copy_tags_to_snapshot),
        storage_encrypted: Some(i.storage_encrypted),
        kms_key_id: i.kms_key_id.clone(),
        c_a_certificate_identifier: i.ca_certificate_identifier.clone(),
        tag_list: if i.tags.is_empty() {
            None
        } else {
            Some(tags_to_model(&i.tags))
        },
        associated_roles: if i.associated_roles.is_empty() {
            None
        } else {
            Some(wire::DBInstanceRoles::from(
                i.associated_roles
                    .iter()
                    .map(|r| wire::DBInstanceRole {
                        role_arn: Some(r.clone()),
                        status: Some("active".to_string()),
                        ..Default::default()
                    })
                    .collect::<Vec<_>>(),
            ))
        },
        ..Default::default()
    }
}

fn db_cluster_to_model(c: &crate::types::DbCluster) -> wire::DBCluster {
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
        vpc_security_groups: Some(wire::VpcSecurityGroupMembershipList::from(
            c.vpc_security_group_ids
                .iter()
                .map(|id| wire::VpcSecurityGroupMembership {
                    vpc_security_group_id: Some(id.clone()),
                    status: Some("active".to_string()),
                })
                .collect::<Vec<_>>(),
        )),
        availability_zones: Some(wire::AvailabilityZones::from(c.availability_zones.clone())),
        d_b_cluster_arn: Some(c.arn.clone()),
        cluster_create_time: c.cluster_create_time.clone(),
        multi_a_z: Some(c.multi_az),
        storage_type: c.storage_type.clone(),
        allocated_storage: c.allocated_storage,
        backup_retention_period: Some(c.backup_retention_period),
        deletion_protection: Some(c.deletion_protection),
        storage_encrypted: Some(c.storage_encrypted),
        kms_key_id: c.kms_key_id.clone(),
        d_b_cluster_parameter_group: c.db_cluster_parameter_group.clone(),
        engine_mode: c.engine_mode.clone(),
        copy_tags_to_snapshot: Some(c.copy_tags_to_snapshot),
        d_b_cluster_members: Some(wire::DBClusterMemberList::from(
            c.members
                .iter()
                .map(db_cluster_member_to_model)
                .collect::<Vec<_>>(),
        )),
        tag_list: if c.tags.is_empty() {
            None
        } else {
            Some(tags_to_model(&c.tags))
        },
        associated_roles: if c.associated_roles.is_empty() {
            None
        } else {
            Some(wire::DBClusterRoles::from(
                c.associated_roles
                    .iter()
                    .map(|r| wire::DBClusterRole {
                        role_arn: Some(r.clone()),
                        status: Some("active".to_string()),
                        ..Default::default()
                    })
                    .collect::<Vec<_>>(),
            ))
        },
        ..Default::default()
    }
}

fn db_cluster_member_to_model(m: &crate::types::DbClusterMember) -> wire::DBClusterMember {
    wire::DBClusterMember {
        d_b_instance_identifier: Some(m.db_instance_identifier.clone()),
        is_cluster_writer: Some(m.is_cluster_writer),
        d_b_cluster_parameter_group_status: Some(m.db_cluster_parameter_group_status.clone()),
        promotion_tier: m.promotion_tier,
    }
}

fn db_subnet_group_to_model(sg: &crate::types::DbSubnetGroup) -> wire::DBSubnetGroup {
    wire::DBSubnetGroup {
        d_b_subnet_group_name: Some(sg.name.clone()),
        d_b_subnet_group_description: Some(sg.description.clone()),
        vpc_id: sg.vpc_id.clone(),
        subnet_group_status: Some(sg.status.clone()),
        subnets: Some(wire::SubnetList::from(
            sg.subnet_ids
                .iter()
                .map(|id| wire::Subnet {
                    subnet_identifier: Some(id.clone()),
                    ..Default::default()
                })
                .collect::<Vec<_>>(),
        )),
        d_b_subnet_group_arn: Some(sg.arn.clone()),
        ..Default::default()
    }
}

fn db_parameter_group_to_model(pg: &crate::types::DbParameterGroup) -> wire::DBParameterGroup {
    wire::DBParameterGroup {
        d_b_parameter_group_name: Some(pg.name.clone()),
        d_b_parameter_group_family: Some(pg.family.clone()),
        description: Some(pg.description.clone()),
        d_b_parameter_group_arn: Some(pg.arn.clone()),
    }
}

fn db_cluster_parameter_group_to_model(
    pg: &crate::types::DbClusterParameterGroup,
) -> wire::DBClusterParameterGroup {
    wire::DBClusterParameterGroup {
        d_b_cluster_parameter_group_name: Some(pg.name.clone()),
        d_b_parameter_group_family: Some(pg.family.clone()),
        description: Some(pg.description.clone()),
        d_b_cluster_parameter_group_arn: Some(pg.arn.clone()),
    }
}

fn db_snapshot_to_model(s: &crate::types::DbSnapshot) -> wire::DBSnapshot {
    wire::DBSnapshot {
        d_b_snapshot_identifier: Some(s.identifier.clone()),
        d_b_instance_identifier: Some(s.db_instance_identifier.clone()),
        engine: Some(s.engine.clone()),
        engine_version: s.engine_version.clone(),
        allocated_storage: Some(s.allocated_storage),
        status: Some(s.status.clone()),
        port: s.port,
        availability_zone: s.availability_zone.clone(),
        vpc_id: s.vpc_id.clone(),
        instance_create_time: s.instance_create_time.clone(),
        master_username: s.master_username.clone(),
        snapshot_type: Some(s.snapshot_type.clone()),
        iops: s.iops,
        option_group_name: s.option_group_name.clone(),
        percent_progress: Some(s.percent_progress),
        source_region: s.source_region.clone(),
        source_d_b_snapshot_identifier: s.source_db_snapshot_identifier.clone(),
        storage_type: s.storage_type.clone(),
        tde_credential_arn: s.tde_credential_arn.clone(),
        encrypted: Some(s.encrypted),
        kms_key_id: s.kms_key_id.clone(),
        d_b_snapshot_arn: Some(s.db_snapshot_arn.clone()),
        timezone: s.timezone.clone(),
        snapshot_create_time: s.snapshot_create_time.clone(),
        tag_list: if s.tags.is_empty() {
            None
        } else {
            Some(tags_to_model(&s.tags))
        },
        ..Default::default()
    }
}

fn db_cluster_snapshot_to_model(s: &crate::types::DbClusterSnapshot) -> wire::DBClusterSnapshot {
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
        source_d_b_cluster_snapshot_arn: s.source_db_cluster_snapshot_arn.clone(),
        availability_zones: Some(wire::AvailabilityZones::from(s.availability_zones.clone())),
        snapshot_create_time: s.snapshot_create_time.clone(),
        storage_type: s.storage_type.clone(),
        tag_list: if s.tags.is_empty() {
            None
        } else {
            Some(tags_to_model(&s.tags))
        },
        ..Default::default()
    }
}

fn db_security_group_to_model(sg: &crate::types::DbSecurityGroup) -> wire::DBSecurityGroup {
    wire::DBSecurityGroup {
        d_b_security_group_name: Some(sg.name.clone()),
        d_b_security_group_description: Some(sg.description.clone()),
        vpc_id: sg.vpc_id.clone(),
        owner_id: Some(sg.owner_id.clone()),
        d_b_security_group_arn: Some(sg.arn.clone()),
        ..Default::default()
    }
}

fn event_subscription_to_model(s: &crate::types::EventSubscription) -> wire::EventSubscription {
    wire::EventSubscription {
        cust_subscription_id: Some(s.subscription_name.clone()),
        sns_topic_arn: Some(s.sns_topic_arn.clone()),
        source_type: s.source_type.clone(),
        source_ids_list: Some(wire::SourceIdsList::from(s.source_ids.clone())),
        event_categories_list: Some(wire::EventCategoriesList::from(s.event_categories.clone())),
        enabled: Some(s.enabled),
        status: Some(s.status.clone()),
        event_subscription_arn: Some(s.arn.clone()),
        customer_aws_id: Some(s.customer_aws_id.clone()),
        subscription_creation_time: Some(s.subscription_creation_time.clone()),
        ..Default::default()
    }
}

fn option_group_to_model(og: &crate::types::OptionGroup) -> wire::OptionGroup {
    wire::OptionGroup {
        option_group_name: Some(og.name.clone()),
        engine_name: Some(og.engine_name.clone()),
        major_engine_version: Some(og.major_engine_version.clone()),
        option_group_description: Some(og.description.clone()),
        allows_vpc_and_non_vpc_instance_memberships: Some(
            og.allows_vpc_and_non_vpc_instance_memberships,
        ),
        vpc_id: og.vpc_id.clone(),
        option_group_arn: Some(og.arn.clone()),
        ..Default::default()
    }
}

fn global_cluster_to_model(gc: &crate::types::GlobalCluster) -> wire::GlobalCluster {
    wire::GlobalCluster {
        global_cluster_identifier: Some(gc.global_cluster_identifier.clone()),
        global_cluster_resource_id: Some(gc.global_cluster_resource_id.clone()),
        global_cluster_arn: Some(gc.global_cluster_arn.clone()),
        status: Some(gc.status.clone()),
        engine: gc.engine.clone(),
        engine_version: gc.engine_version.clone(),
        database_name: gc.database_name.clone(),
        storage_encrypted: Some(gc.storage_encrypted),
        deletion_protection: Some(gc.deletion_protection),
        ..Default::default()
    }
}

fn export_task_to_model(t: &crate::types::ExportTask) -> wire::ExportTask {
    wire::ExportTask {
        export_task_identifier: Some(t.export_task_identifier.clone()),
        source_arn: Some(t.source_arn.clone()),
        s3_bucket: Some(t.s3_bucket.clone()),
        s3_prefix: t.s3_prefix.clone(),
        iam_role_arn: Some(t.iam_role_arn.clone()),
        kms_key_id: Some(t.kms_key_id.clone()),
        status: Some(t.status.clone()),
        percent_progress: Some(t.percent_progress),
        export_only: Some(wire::StringList::from(t.export_only.clone())),
        task_start_time: t.task_start_time.clone(),
        task_end_time: t.task_end_time.clone(),
        total_extracted_data_in_g_b: t.total_extracted_data_in_gb,
        source_type: t.source_type.clone(),
        snapshot_time: t.snapshot_time.clone(),
        failure_cause: t.failure_cause.clone(),
        warning_message: t.warning_message.clone(),
    }
}

fn db_proxy_to_model(p: &crate::types::DbProxy) -> wire::DBProxy {
    wire::DBProxy {
        d_b_proxy_name: Some(p.db_proxy_name.clone()),
        d_b_proxy_arn: Some(p.db_proxy_arn.clone()),
        status: Some(p.status.clone()),
        engine_family: Some(p.engine_family.clone()),
        vpc_id: p.vpc_id.clone(),
        vpc_security_group_ids: Some(wire::StringList::from(p.vpc_security_group_ids.clone())),
        vpc_subnet_ids: Some(wire::StringList::from(p.vpc_subnet_ids.clone())),
        endpoint: Some(p.endpoint.clone()),
        require_t_l_s: Some(p.require_tls),
        idle_client_timeout: Some(p.idle_client_timeout),
        debug_logging: Some(p.debug_logging),
        role_arn: Some(p.role_arn.clone()),
        created_date: p.created_date.clone(),
        updated_date: p.updated_date.clone(),
        ..Default::default()
    }
}

fn blue_green_deployment_to_model(
    d: &crate::types::BlueGreenDeployment,
) -> wire::BlueGreenDeployment {
    wire::BlueGreenDeployment {
        blue_green_deployment_identifier: Some(d.blue_green_deployment_identifier.clone()),
        blue_green_deployment_name: Some(d.blue_green_deployment_name.clone()),
        source: d.source.clone(),
        target: d.target.clone(),
        status: Some(d.status.clone()),
        status_details: d.status_details.clone(),
        create_time: d.create_time.clone(),
        delete_time: d.delete_time.clone(),
        tag_list: if d.tags.is_empty() {
            None
        } else {
            Some(tags_to_model(&d.tags))
        },
        ..Default::default()
    }
}

fn db_shard_group_to_model(sg: &crate::types::DbShardGroup) -> wire::DBShardGroup {
    wire::DBShardGroup {
        d_b_shard_group_identifier: Some(sg.db_shard_group_identifier.clone()),
        d_b_shard_group_resource_id: sg.db_shard_group_resource_id.clone(),
        d_b_cluster_identifier: Some(sg.db_cluster_identifier.clone()),
        max_a_c_u: Some(sg.max_acu),
        min_a_c_u: sg.min_acu,
        publicly_accessible: Some(sg.publicly_accessible),
        status: Some(sg.status.clone()),
        endpoint: sg.endpoint.clone(),
        d_b_shard_group_arn: sg.db_shard_group_arn.clone(),
        tag_list: if sg.tag_list.is_empty() {
            None
        } else {
            Some(tags_to_model(&sg.tag_list))
        },
        ..Default::default()
    }
}

fn db_proxy_target_to_model(t: &crate::types::DbProxyTarget) -> wire::DBProxyTarget {
    wire::DBProxyTarget {
        target_arn: Some(t.target_arn.clone()),
        endpoint: t.endpoint.clone(),
        tracked_cluster_id: t.tracked_cluster_id.clone(),
        rds_resource_id: t.rds_resource_id.clone(),
        port: t.port,
        r#type: t.type_.clone(),
        role: t.role.clone(),
        ..Default::default()
    }
}

fn db_proxy_target_group_to_model(
    g: &crate::types::DbProxyTargetGroup,
) -> wire::DBProxyTargetGroup {
    wire::DBProxyTargetGroup {
        target_group_name: Some(g.target_group_name.clone()),
        d_b_proxy_name: Some(g.db_proxy_name.clone()),
        target_group_arn: Some(g.target_group_arn.clone()),
        is_default: Some(g.is_default),
        status: Some(g.status.clone()),
        connection_pool_config: None,
        created_date: g.created_date.clone(),
        updated_date: g.updated_date.clone(),
    }
}

fn db_cluster_endpoint_to_model(ep: &crate::types::DbClusterEndpoint) -> wire::DBClusterEndpoint {
    wire::DBClusterEndpoint {
        d_b_cluster_endpoint_identifier: Some(ep.db_cluster_endpoint_identifier.clone()),
        d_b_cluster_identifier: Some(ep.db_cluster_identifier.clone()),
        d_b_cluster_endpoint_arn: Some(ep.db_cluster_endpoint_arn.clone()),
        endpoint: Some(ep.endpoint.clone()),
        status: Some(ep.status.clone()),
        endpoint_type: Some(ep.endpoint_type.clone()),
        custom_endpoint_type: ep.custom_endpoint_type.clone(),
        static_members: Some(wire::StringList::from(ep.static_members.clone())),
        excluded_members: Some(wire::StringList::from(ep.excluded_members.clone())),
        ..Default::default()
    }
}

fn db_proxy_endpoint_to_model(ep: &crate::types::DbProxyEndpoint) -> wire::DBProxyEndpoint {
    wire::DBProxyEndpoint {
        d_b_proxy_endpoint_name: Some(ep.db_proxy_endpoint_name.clone()),
        d_b_proxy_endpoint_arn: Some(ep.db_proxy_endpoint_arn.clone()),
        d_b_proxy_name: Some(ep.db_proxy_name.clone()),
        status: Some(ep.status.clone()),
        vpc_id: ep.vpc_id.clone(),
        vpc_security_group_ids: Some(wire::StringList::from(ep.vpc_security_group_ids.clone())),
        vpc_subnet_ids: Some(wire::StringList::from(ep.vpc_subnet_ids.clone())),
        endpoint: Some(ep.endpoint.clone()),
        endpoint_network_type: None,
        is_default: Some(ep.is_default),
        target_role: Some(ep.target_role.clone()),
        created_date: ep.created_date.clone(),
    }
}

fn db_instance_to_automated_backup_model(
    i: &crate::types::DbInstance,
    source_arn: &str,
) -> wire::DBInstanceAutomatedBackup {
    wire::DBInstanceAutomatedBackup {
        d_b_instance_identifier: Some(i.identifier.clone()),
        d_b_instance_arn: Some(i.arn.clone()),
        engine: Some(i.engine.clone()),
        engine_version: Some(i.engine_version.clone()),
        allocated_storage: Some(i.allocated_storage),
        backup_retention_period: Some(i.backup_retention_period),
        status: Some(i.status.clone()),
        d_b_instance_automated_backups_arn: Some(source_arn.to_string()),
        ..Default::default()
    }
}
