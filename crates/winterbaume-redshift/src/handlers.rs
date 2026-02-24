use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{RedshiftError, RedshiftState};
use crate::views::RedshiftStateView;
use crate::wire;

/// Redshift service handler that processes awsQuery protocol requests.
pub struct RedshiftService {
    pub(crate) state: Arc<BackendState<RedshiftState>>,
    pub(crate) notifier: StateChangeNotifier<RedshiftStateView>,
}

impl RedshiftService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for RedshiftService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for RedshiftService {
    fn service_name(&self) -> &str {
        "redshift"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://redshift\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

const MUTATING_ACTIONS: &[&str] = &[
    "CreateCluster",
    "DeleteCluster",
    "ModifyCluster",
    "PauseCluster",
    "ResumeCluster",
    "RebootCluster",
    "ResizeCluster",
    "RestoreFromClusterSnapshot",
    "CreateClusterSubnetGroup",
    "ModifyClusterSubnetGroup",
    "DeleteClusterSubnetGroup",
    "CreateClusterParameterGroup",
    "ModifyClusterParameterGroup",
    "ResetClusterParameterGroup",
    "DeleteClusterParameterGroup",
    "CreateClusterSecurityGroup",
    "DeleteClusterSecurityGroup",
    "AuthorizeClusterSecurityGroupIngress",
    "RevokeClusterSecurityGroupIngress",
    "CreateClusterSnapshot",
    "DeleteClusterSnapshot",
    "CopyClusterSnapshot",
    "BatchDeleteClusterSnapshots",
    "CreateSnapshotCopyGrant",
    "DeleteSnapshotCopyGrant",
    "EnableSnapshotCopy",
    "DisableSnapshotCopy",
    "ModifySnapshotCopyRetentionPeriod",
    "EnableLogging",
    "DisableLogging",
    "CreateTags",
    "DeleteTags",
    "CreateEventSubscription",
    "DeleteEventSubscription",
    "CreateHsmClientCertificate",
    "DeleteHsmClientCertificate",
    "CreateHsmConfiguration",
    "DeleteHsmConfiguration",
    "CreateAuthenticationProfile",
    "ModifyAuthenticationProfile",
    "DeleteAuthenticationProfile",
    "CreateUsageLimit",
    "ModifyUsageLimit",
    "DeleteUsageLimit",
    "CreateSnapshotSchedule",
    "ModifySnapshotSchedule",
    "DeleteSnapshotSchedule",
    "CreateScheduledAction",
    "ModifyScheduledAction",
    "DeleteScheduledAction",
    "AuthorizeSnapshotAccess",
    "RevokeSnapshotAccess",
    "ModifyClusterSnapshot",
    "BatchModifyClusterSnapshots",
    "ModifyClusterIamRoles",
    "ModifyClusterMaintenance",
    "RotateEncryptionKey",
    "FailoverPrimaryCompute",
    "ModifyEventSubscription",
    "PutResourcePolicy",
    "DeleteResourcePolicy",
    "AddPartner",
    "DeletePartner",
    "UpdatePartnerStatus",
    "ModifyClusterSnapshotSchedule",
    "ModifyAquaConfiguration",
    "ModifyClusterSnapshotSchedule",
];

impl RedshiftService {
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
            "CreateCluster" => {
                self.handle_create_cluster(&state, &params, account_id, &region)
                    .await
            }
            "DescribeClusters" => self.handle_describe_clusters(&state, &params).await,
            "DeleteCluster" => {
                self.handle_delete_cluster(&state, &params, account_id, &region)
                    .await
            }
            "ModifyCluster" => self.handle_modify_cluster(&state, &params).await,
            "PauseCluster" => self.handle_pause_cluster(&state, &params).await,
            "ResumeCluster" => self.handle_resume_cluster(&state, &params).await,
            "RestoreFromClusterSnapshot" => {
                self.handle_restore_from_cluster_snapshot(&state, &params, account_id, &region)
                    .await
            }
            "CreateClusterSubnetGroup" => {
                self.handle_create_cluster_subnet_group(&state, &params, account_id, &region)
                    .await
            }
            "DescribeClusterSubnetGroups" => {
                self.handle_describe_cluster_subnet_groups(&state, &params)
                    .await
            }
            "DeleteClusterSubnetGroup" => {
                self.handle_delete_cluster_subnet_group(&state, &params)
                    .await
            }
            "CreateClusterParameterGroup" => {
                self.handle_create_cluster_parameter_group(&state, &params, account_id, &region)
                    .await
            }
            "DescribeClusterParameterGroups" => {
                self.handle_describe_cluster_parameter_groups(&state, &params)
                    .await
            }
            "DescribeClusterParameters" => {
                self.handle_describe_cluster_parameters(&state, &params)
                    .await
            }
            "DescribeDefaultClusterParameters" => {
                self.handle_describe_default_cluster_parameters(&state, &params)
                    .await
            }
            "DeleteClusterParameterGroup" => {
                self.handle_delete_cluster_parameter_group(&state, &params)
                    .await
            }
            "CreateClusterSecurityGroup" => {
                self.handle_create_cluster_security_group(&state, &params, account_id, &region)
                    .await
            }
            "DescribeClusterSecurityGroups" => {
                self.handle_describe_cluster_security_groups(&state, &params)
                    .await
            }
            "DeleteClusterSecurityGroup" => {
                self.handle_delete_cluster_security_group(&state, &params)
                    .await
            }
            "AuthorizeClusterSecurityGroupIngress" => {
                self.handle_authorize_cluster_security_group_ingress(&state, &params)
                    .await
            }
            "CreateClusterSnapshot" => {
                self.handle_create_cluster_snapshot(&state, &params, account_id, &region)
                    .await
            }
            "DescribeClusterSnapshots" => {
                self.handle_describe_cluster_snapshots(&state, &params)
                    .await
            }
            "DeleteClusterSnapshot" => self.handle_delete_cluster_snapshot(&state, &params).await,
            "CreateSnapshotCopyGrant" => {
                self.handle_create_snapshot_copy_grant(&state, &params, account_id, &region)
                    .await
            }
            "DeleteSnapshotCopyGrant" => {
                self.handle_delete_snapshot_copy_grant(&state, &params)
                    .await
            }
            "DescribeSnapshotCopyGrants" => {
                self.handle_describe_snapshot_copy_grants(&state, &params)
                    .await
            }
            "EnableSnapshotCopy" => self.handle_enable_snapshot_copy(&state, &params).await,
            "DisableSnapshotCopy" => self.handle_disable_snapshot_copy(&state, &params).await,
            "ModifySnapshotCopyRetentionPeriod" => {
                self.handle_modify_snapshot_copy_retention_period(&state, &params)
                    .await
            }
            "EnableLogging" => self.handle_enable_logging(&state, &params).await,
            "DisableLogging" => self.handle_disable_logging(&state, &params).await,
            "DescribeLoggingStatus" => self.handle_describe_logging_status(&state, &params).await,
            "CreateTags" => self.handle_create_tags(&state, &params).await,
            "DeleteTags" => self.handle_delete_tags(&state, &params).await,
            "DescribeTags" => self.handle_describe_tags(&state, &params).await,
            "GetClusterCredentials" => {
                self.handle_get_cluster_credentials(&state, &params, account_id, &region)
                    .await
            }
            "RebootCluster" => self.handle_reboot_cluster(&state, &params).await,
            "ResizeCluster" => self.handle_resize_cluster(&state, &params).await,
            "ModifyClusterParameterGroup" => {
                self.handle_modify_cluster_parameter_group(&state, &params)
                    .await
            }
            "ResetClusterParameterGroup" => {
                self.handle_reset_cluster_parameter_group(&state, &params)
                    .await
            }
            "CopyClusterSnapshot" => {
                self.handle_copy_cluster_snapshot(&state, &params, account_id, &region)
                    .await
            }
            "RevokeClusterSecurityGroupIngress" => {
                self.handle_revoke_cluster_security_group_ingress(&state, &params)
                    .await
            }
            "ModifyClusterSubnetGroup" => {
                self.handle_modify_cluster_subnet_group(&state, &params)
                    .await
            }
            "BatchDeleteClusterSnapshots" => {
                self.handle_batch_delete_cluster_snapshots(&state, &params)
                    .await
            }
            "CreateEventSubscription" => {
                self.handle_create_event_subscription(&state, &params, account_id)
                    .await
            }
            "DescribeEventSubscriptions" => {
                self.handle_describe_event_subscriptions(&state, &params)
                    .await
            }
            "DeleteEventSubscription" => {
                self.handle_delete_event_subscription(&state, &params).await
            }
            "DescribeEvents" => self.handle_describe_events(&state, &params).await,
            "DescribeClusterVersions" => self.handle_describe_cluster_versions().await,
            "DescribeOrderableClusterOptions" => {
                self.handle_describe_orderable_cluster_options().await
            }
            "DescribeEventCategories" => {
                self.handle_describe_event_categories(&state, &params).await
            }
            "CreateHsmClientCertificate" => {
                self.handle_create_hsm_client_certificate(&state, &params, account_id, &region)
                    .await
            }
            "DescribeHsmClientCertificates" => {
                self.handle_describe_hsm_client_certificates(&state, &params)
                    .await
            }
            "DeleteHsmClientCertificate" => {
                self.handle_delete_hsm_client_certificate(&state, &params)
                    .await
            }
            "CreateHsmConfiguration" => {
                self.handle_create_hsm_configuration(&state, &params, account_id, &region)
                    .await
            }
            "DescribeHsmConfigurations" => {
                self.handle_describe_hsm_configurations(&state, &params)
                    .await
            }
            "DeleteHsmConfiguration" => self.handle_delete_hsm_configuration(&state, &params).await,
            "CreateAuthenticationProfile" => {
                self.handle_create_authentication_profile(&state, &params)
                    .await
            }
            "DescribeAuthenticationProfiles" => {
                self.handle_describe_authentication_profiles(&state, &params)
                    .await
            }
            "ModifyAuthenticationProfile" => {
                self.handle_modify_authentication_profile(&state, &params)
                    .await
            }
            "DeleteAuthenticationProfile" => {
                self.handle_delete_authentication_profile(&state, &params)
                    .await
            }
            "CreateUsageLimit" => self.handle_create_usage_limit(&state, &params).await,
            "DescribeUsageLimits" => self.handle_describe_usage_limits(&state, &params).await,
            "ModifyUsageLimit" => self.handle_modify_usage_limit(&state, &params).await,
            "DeleteUsageLimit" => self.handle_delete_usage_limit(&state, &params).await,
            "CreateSnapshotSchedule" => self.handle_create_snapshot_schedule(&state, &params).await,
            "DescribeSnapshotSchedules" => {
                self.handle_describe_snapshot_schedules(&state, &params)
                    .await
            }
            "ModifySnapshotSchedule" => self.handle_modify_snapshot_schedule(&state, &params).await,
            "DeleteSnapshotSchedule" => self.handle_delete_snapshot_schedule(&state, &params).await,
            "CreateScheduledAction" => self.handle_create_scheduled_action(&state, &params).await,
            "DescribeScheduledActions" => {
                self.handle_describe_scheduled_actions(&state, &params)
                    .await
            }
            "ModifyScheduledAction" => self.handle_modify_scheduled_action(&state, &params).await,
            "DeleteScheduledAction" => self.handle_delete_scheduled_action(&state, &params).await,
            "AuthorizeSnapshotAccess" => {
                self.handle_authorize_snapshot_access(&state, &params).await
            }
            "RevokeSnapshotAccess" => self.handle_revoke_snapshot_access(&state, &params).await,
            "ModifyClusterSnapshot" => self.handle_modify_cluster_snapshot(&state, &params).await,
            "BatchModifyClusterSnapshots" => {
                self.handle_batch_modify_cluster_snapshots(&state, &params)
                    .await
            }
            "ModifyClusterIamRoles" => self.handle_modify_cluster_iam_roles(&state, &params).await,
            "ModifyClusterMaintenance" => {
                self.handle_modify_cluster_maintenance(&state, &params)
                    .await
            }
            "RotateEncryptionKey" => self.handle_rotate_encryption_key(&state, &params).await,
            "FailoverPrimaryCompute" => self.handle_failover_primary_compute(&state, &params).await,
            "DescribeAccountAttributes" => self.handle_describe_account_attributes(&state).await,
            "DescribeStorage" => self.handle_describe_storage().await,
            "DescribeClusterTracks" => self.handle_describe_cluster_tracks(&state).await,
            "DescribeResize" => self.handle_describe_resize(&state, &params).await,
            "CancelResize" => self.handle_cancel_resize(&state, &params).await,
            "GetClusterCredentialsWithIAM" => {
                self.handle_get_cluster_credentials_with_iam(&state, &params, account_id, &region)
                    .await
            }
            "ModifyClusterDbRevision" => {
                self.handle_modify_cluster_db_revision(&state, &params)
                    .await
            }
            "DescribeClusterDbRevisions" => {
                self.handle_describe_cluster_db_revisions(&state, &params)
                    .await
            }
            "ModifyEventSubscription" => {
                self.handle_modify_event_subscription(&state, &params).await
            }
            "PutResourcePolicy" => self.handle_put_resource_policy(&state, &params).await,
            "GetResourcePolicy" => self.handle_get_resource_policy(&state, &params).await,
            "DeleteResourcePolicy" => self.handle_delete_resource_policy(&state, &params).await,
            "AddPartner" => self.handle_add_partner(&state, &params).await,
            "DescribePartners" => self.handle_describe_partners(&state, &params).await,
            "DeletePartner" => self.handle_delete_partner(&state, &params).await,
            "UpdatePartnerStatus" => self.handle_update_partner_status(&state, &params).await,
            "ModifyClusterSnapshotSchedule" => {
                self.handle_modify_cluster_snapshot_schedule(&state, &params)
                    .await
            }
            "ModifyAquaConfiguration" => {
                self.handle_modify_aqua_configuration(&state, &params).await
            }
            "DescribeNodeConfigurationOptions" => {
                self.handle_describe_node_configuration_options(&state)
                    .await
            }
            "DescribeReservedNodes" => self.handle_describe_reserved_nodes(&state).await,
            "DescribeReservedNodeOfferings" => {
                self.handle_describe_reserved_node_offerings(&state).await
            }
            "DescribeTableRestoreStatus" => {
                self.handle_describe_table_restore_status(&state, &params)
                    .await
            }
            "ListRecommendations" => self.handle_list_recommendations().await,
            _ => MockResponse::error(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for Redshift"),
            ),
        };

        if MUTATING_ACTIONS.contains(&action.as_str()) && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // ---- Clusters ----

    async fn handle_create_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        if input.node_type.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'NodeType'");
        }
        if input.master_username.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'MasterUsername'");
        }
        let cluster_identifier = input.cluster_identifier;
        let node_type = input.node_type;
        let master_username = input.master_username;
        let db_name = input.d_b_name.unwrap_or_else(|| "dev".to_string());
        let number_of_nodes: i32 = input.number_of_nodes.unwrap_or(1);
        let subnet_group_name = input.cluster_subnet_group_name;
        let publicly_accessible = input.publicly_accessible.unwrap_or(true);
        let encrypted = input.encrypted.unwrap_or(false);
        let availability_zone = input.availability_zone;
        let availability_zone_relocation = input.availability_zone_relocation.unwrap_or(false);
        let preferred_mw = input.preferred_maintenance_window;
        let automated_retention: i32 = input.automated_snapshot_retention_period.unwrap_or(1);
        let tags = wire_tags_to_domain(input.tags);

        let mut state_guard = state.write().await;
        match state_guard.create_cluster(
            cluster_identifier,
            node_type,
            master_username,
            db_name,
            region,
            account_id,
            number_of_nodes,
            subnet_group_name,
            tags,
            publicly_accessible,
            encrypted,
            availability_zone,
            availability_zone_relocation,
            preferred_mw,
            automated_retention,
        ) {
            Ok(cluster) => wire::serialize_create_cluster_response(&wire::CreateClusterResult {
                cluster: Some(cluster_to_wire(&cluster)),
            }),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_describe_clusters(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_clusters_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let cluster_identifier = input.cluster_identifier.as_deref();
        let state_guard = state.read().await;
        match state_guard.describe_clusters(cluster_identifier) {
            Ok(clusters) => wire::serialize_describe_clusters_response(&wire::ClustersMessage {
                clusters: Some(wire::ClusterList {
                    items: clusters.iter().map(cluster_to_wire).collect(),
                }),
                marker: None,
            }),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_delete_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let skip_final = input.skip_final_cluster_snapshot.unwrap_or(false);
        let final_snapshot_id = if skip_final {
            None
        } else {
            input.final_cluster_snapshot_identifier.as_deref()
        };

        let mut state_guard = state.write().await;
        match state_guard.delete_cluster(&cluster_identifier, final_snapshot_id, region, account_id)
        {
            Ok(cluster) => wire::serialize_delete_cluster_response(&wire::DeleteClusterResult {
                cluster: Some(cluster_to_wire(&cluster)),
            }),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_modify_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let new_node_type = input.node_type;
        let number_of_nodes = input.number_of_nodes;
        let new_cluster_identifier = input.new_cluster_identifier;
        let master_user_password = input.master_user_password;
        let publicly_accessible = input.publicly_accessible;
        let encrypted = input.encrypted;
        let availability_zone_relocation = input.availability_zone_relocation;
        let preferred_mw = input.preferred_maintenance_window;
        let automated_retention = input.automated_snapshot_retention_period;

        let mut state_guard = state.write().await;
        match state_guard.modify_cluster(
            &cluster_identifier,
            new_node_type,
            number_of_nodes,
            new_cluster_identifier,
            master_user_password,
            publicly_accessible,
            encrypted,
            availability_zone_relocation,
            preferred_mw,
            automated_retention,
        ) {
            Ok(cluster) => wire::serialize_modify_cluster_response(&wire::ModifyClusterResult {
                cluster: Some(cluster_to_wire(&cluster)),
            }),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_pause_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_pause_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let mut state_guard = state.write().await;
        match state_guard.pause_cluster(&cluster_identifier) {
            Ok(cluster) => wire::serialize_pause_cluster_response(&wire::PauseClusterResult {
                cluster: Some(cluster_to_wire(&cluster)),
            }),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_resume_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_resume_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let mut state_guard = state.write().await;
        match state_guard.resume_cluster(&cluster_identifier) {
            Ok(cluster) => wire::serialize_resume_cluster_response(&wire::ResumeClusterResult {
                cluster: Some(cluster_to_wire(&cluster)),
            }),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_restore_from_cluster_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_restore_from_cluster_snapshot_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let snapshot_identifier = match input.snapshot_identifier {
            Some(ref v) if !v.is_empty() => v.clone(),
            _ => {
                return MockResponse::error(
                    400,
                    "MissingParameter",
                    "Missing 'SnapshotIdentifier'",
                );
            }
        };
        let cluster_identifier = input.cluster_identifier;
        let subnet_group_name = input.cluster_subnet_group_name;
        let publicly_accessible = input.publicly_accessible.unwrap_or(true);
        let availability_zone_relocation = input.availability_zone_relocation.unwrap_or(false);

        let mut state_guard = state.write().await;
        match state_guard.restore_from_cluster_snapshot(
            cluster_identifier,
            &snapshot_identifier,
            region,
            account_id,
            subnet_group_name,
            publicly_accessible,
            availability_zone_relocation,
        ) {
            Ok(cluster) => wire::serialize_restore_from_cluster_snapshot_response(
                &wire::RestoreFromClusterSnapshotResult {
                    cluster: Some(cluster_to_wire(&cluster)),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Subnet Groups ----

    async fn handle_create_cluster_subnet_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_cluster_subnet_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_subnet_group_name.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'ClusterSubnetGroupName'",
            );
        }
        let name = input.cluster_subnet_group_name;
        let description = input.description;
        // VpcId is not part of the Smithy model; parse manually if present
        let vpc_id = params.get("VpcId").cloned().unwrap_or_default();
        let subnet_ids = input.subnet_ids.items;
        let tags = wire_tags_to_domain(input.tags);

        let mut state_guard = state.write().await;
        match state_guard.create_cluster_subnet_group(
            name,
            description,
            vpc_id,
            subnet_ids,
            tags,
            region,
            account_id,
        ) {
            Ok(sg) => wire::serialize_create_cluster_subnet_group_response(
                &wire::CreateClusterSubnetGroupResult {
                    cluster_subnet_group: Some(subnet_group_to_wire(&sg)),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_describe_cluster_subnet_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_cluster_subnet_groups_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.cluster_subnet_group_name.as_deref();
        let state_guard = state.read().await;
        match state_guard.describe_cluster_subnet_groups(name) {
            Ok(groups) => wire::serialize_describe_cluster_subnet_groups_response(
                &wire::ClusterSubnetGroupMessage {
                    cluster_subnet_groups: Some(wire::ClusterSubnetGroups {
                        items: groups.iter().map(subnet_group_to_wire).collect(),
                    }),
                    marker: None,
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_delete_cluster_subnet_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_cluster_subnet_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_subnet_group_name.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'ClusterSubnetGroupName'",
            );
        }
        let name = input.cluster_subnet_group_name;
        let mut state_guard = state.write().await;
        match state_guard.delete_cluster_subnet_group(&name) {
            Ok(()) => wire::serialize_delete_cluster_subnet_group_response(),
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Parameter Groups ----

    async fn handle_create_cluster_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_cluster_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.parameter_group_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ParameterGroupName'");
        }
        if input.parameter_group_family.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ParameterGroupFamily'");
        }
        let name = input.parameter_group_name;
        let family = input.parameter_group_family;
        let description = input.description;
        let tags = wire_tags_to_domain(input.tags);

        let mut state_guard = state.write().await;
        match state_guard.create_cluster_parameter_group(
            name,
            family,
            description,
            tags,
            region,
            account_id,
        ) {
            Ok(pg) => wire::serialize_create_cluster_parameter_group_response(
                &wire::CreateClusterParameterGroupResult {
                    cluster_parameter_group: Some(parameter_group_to_wire(&pg)),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_describe_cluster_parameter_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_cluster_parameter_groups_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.parameter_group_name.as_deref();
        let state_guard = state.read().await;
        match state_guard.describe_cluster_parameter_groups(name) {
            Ok(groups) => wire::serialize_describe_cluster_parameter_groups_response(
                &wire::ClusterParameterGroupsMessage {
                    parameter_groups: Some(wire::ParameterGroupList {
                        items: groups.iter().map(parameter_group_to_wire).collect(),
                    }),
                    marker: None,
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_describe_cluster_parameters(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_cluster_parameters_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.parameter_group_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ParameterGroupName'");
        }
        let name = &input.parameter_group_name;
        let state_guard = state.read().await;
        match state_guard.describe_cluster_parameters(name) {
            Ok(params_list) => wire::serialize_describe_cluster_parameters_response(
                &wire::ClusterParameterGroupDetails {
                    parameters: Some(wire::ParametersList {
                        items: params_list.iter().map(param_to_wire).collect(),
                    }),
                    marker: None,
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_describe_default_cluster_parameters(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_default_cluster_parameters_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let family = if input.parameter_group_family.is_empty() {
            "redshift-1.0".to_string()
        } else {
            input.parameter_group_family
        };
        let state_guard = state.read().await;
        let params_list = state_guard.describe_default_cluster_parameters(&family);
        wire::serialize_describe_default_cluster_parameters_response(
            &wire::DescribeDefaultClusterParametersResult {
                default_cluster_parameters: Some(wire::DefaultClusterParameters {
                    parameter_group_family: Some(family),
                    parameters: Some(wire::ParametersList {
                        items: params_list.iter().map(param_to_wire).collect(),
                    }),
                    marker: None,
                }),
            },
        )
    }

    async fn handle_delete_cluster_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_cluster_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.parameter_group_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ParameterGroupName'");
        }
        let name = input.parameter_group_name;
        let mut state_guard = state.write().await;
        match state_guard.delete_cluster_parameter_group(&name) {
            Ok(()) => wire::serialize_delete_cluster_parameter_group_response(),
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Security Groups ----

    async fn handle_create_cluster_security_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_cluster_security_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_security_group_name.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'ClusterSecurityGroupName'",
            );
        }
        let name = input.cluster_security_group_name;
        let description = input.description;
        let tags = wire_tags_to_domain(input.tags);

        let mut state_guard = state.write().await;
        match state_guard.create_cluster_security_group(name, description, tags, region, account_id)
        {
            Ok(sg) => wire::serialize_create_cluster_security_group_response(
                &wire::CreateClusterSecurityGroupResult {
                    cluster_security_group: Some(security_group_to_wire(&sg)),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_describe_cluster_security_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_cluster_security_groups_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.cluster_security_group_name.as_deref();
        let state_guard = state.read().await;
        match state_guard.describe_cluster_security_groups(name) {
            Ok(groups) => wire::serialize_describe_cluster_security_groups_response(
                &wire::ClusterSecurityGroupMessage {
                    cluster_security_groups: Some(wire::ClusterSecurityGroups {
                        items: groups.iter().map(security_group_to_wire).collect(),
                    }),
                    marker: None,
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_delete_cluster_security_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_cluster_security_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_security_group_name.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'ClusterSecurityGroupName'",
            );
        }
        let name = input.cluster_security_group_name;
        let mut state_guard = state.write().await;
        match state_guard.delete_cluster_security_group(&name) {
            Ok(()) => wire::serialize_delete_cluster_security_group_response(),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_authorize_cluster_security_group_ingress(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_authorize_cluster_security_group_ingress_request(params)
        {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_security_group_name.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'ClusterSecurityGroupName'",
            );
        }
        let name = input.cluster_security_group_name;
        let ec2_sg_name = input.e_c2_security_group_name;
        let ec2_sg_owner = input.e_c2_security_group_owner_id;
        let cidrip = input.c_i_d_r_i_p;

        let mut state_guard = state.write().await;
        match state_guard.authorize_cluster_security_group_ingress(
            &name,
            ec2_sg_name,
            ec2_sg_owner,
            cidrip,
        ) {
            Ok(sg) => wire::serialize_authorize_cluster_security_group_ingress_response(
                &wire::AuthorizeClusterSecurityGroupIngressResult {
                    cluster_security_group: Some(security_group_to_wire(&sg)),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Snapshots ----

    async fn handle_create_cluster_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_cluster_snapshot_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.snapshot_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SnapshotIdentifier'");
        }
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let snapshot_identifier = input.snapshot_identifier;
        let cluster_identifier = input.cluster_identifier;
        let tags = wire_tags_to_domain(input.tags);

        let mut state_guard = state.write().await;
        match state_guard.create_cluster_snapshot(
            snapshot_identifier,
            &cluster_identifier,
            tags,
            region,
            account_id,
        ) {
            Ok(snap) => wire::serialize_create_cluster_snapshot_response(
                &wire::CreateClusterSnapshotResult {
                    snapshot: Some(snapshot_to_wire(&snap)),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_describe_cluster_snapshots(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_cluster_snapshots_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let snapshot_identifier = input.snapshot_identifier.as_deref();
        let cluster_identifier = input.cluster_identifier.as_deref();
        let state_guard = state.read().await;
        match state_guard.describe_cluster_snapshots(snapshot_identifier, cluster_identifier) {
            Ok(snaps) => {
                wire::serialize_describe_cluster_snapshots_response(&wire::SnapshotMessage {
                    snapshots: Some(wire::SnapshotList {
                        items: snaps.iter().map(snapshot_to_wire).collect(),
                    }),
                    marker: None,
                })
            }
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_delete_cluster_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_cluster_snapshot_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.snapshot_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SnapshotIdentifier'");
        }
        let snapshot_identifier = input.snapshot_identifier;
        let mut state_guard = state.write().await;
        match state_guard.delete_cluster_snapshot(&snapshot_identifier) {
            Ok(snap) => wire::serialize_delete_cluster_snapshot_response(
                &wire::DeleteClusterSnapshotResult {
                    snapshot: Some(snapshot_to_wire(&snap)),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Snapshot Copy Grants ----

    async fn handle_create_snapshot_copy_grant(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_snapshot_copy_grant_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.snapshot_copy_grant_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SnapshotCopyGrantName'");
        }
        let name = input.snapshot_copy_grant_name;
        let kms_key_id = input.kms_key_id;
        let tags = wire_tags_to_domain(input.tags);

        let mut state_guard = state.write().await;
        match state_guard.create_snapshot_copy_grant(name, kms_key_id, tags, region, account_id) {
            Ok(grant) => wire::serialize_create_snapshot_copy_grant_response(
                &wire::CreateSnapshotCopyGrantResult {
                    snapshot_copy_grant: Some(wire::SnapshotCopyGrant {
                        snapshot_copy_grant_name: Some(grant.name.clone()),
                        kms_key_id: grant.kms_key_id.clone(),
                        tags: Some(wire::TagList {
                            items: grant
                                .tags
                                .iter()
                                .map(|(k, v)| wire::Tag {
                                    key: Some(k.clone()),
                                    value: Some(v.clone()),
                                })
                                .collect(),
                        }),
                    }),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_delete_snapshot_copy_grant(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_snapshot_copy_grant_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.snapshot_copy_grant_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SnapshotCopyGrantName'");
        }
        let name = input.snapshot_copy_grant_name;
        let mut state_guard = state.write().await;
        match state_guard.delete_snapshot_copy_grant(&name) {
            Ok(()) => wire::serialize_delete_snapshot_copy_grant_response(),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_describe_snapshot_copy_grants(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_snapshot_copy_grants_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.snapshot_copy_grant_name.as_deref();
        let state_guard = state.read().await;
        match state_guard.describe_snapshot_copy_grants(name) {
            Ok(grants) => wire::serialize_describe_snapshot_copy_grants_response(
                &wire::SnapshotCopyGrantMessage {
                    snapshot_copy_grants: Some(wire::SnapshotCopyGrantList {
                        items: grants
                            .iter()
                            .map(|g| wire::SnapshotCopyGrant {
                                snapshot_copy_grant_name: Some(g.name.clone()),
                                kms_key_id: g.kms_key_id.clone(),
                                tags: Some(wire::TagList {
                                    items: g
                                        .tags
                                        .iter()
                                        .map(|(k, v)| wire::Tag {
                                            key: Some(k.clone()),
                                            value: Some(v.clone()),
                                        })
                                        .collect(),
                                }),
                            })
                            .collect(),
                    }),
                    marker: None,
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Snapshot Copy ----

    async fn handle_enable_snapshot_copy(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_enable_snapshot_copy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        if input.destination_region.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'DestinationRegion'");
        }
        let cluster_identifier = input.cluster_identifier;
        let destination_region = input.destination_region;
        let retention_period: i32 = input.retention_period.unwrap_or(7);
        let grant_name = input.snapshot_copy_grant_name;

        let mut state_guard = state.write().await;
        match state_guard.enable_snapshot_copy(
            &cluster_identifier,
            destination_region,
            retention_period,
            grant_name,
        ) {
            Ok(cluster) => {
                wire::serialize_enable_snapshot_copy_response(&wire::EnableSnapshotCopyResult {
                    cluster: Some(cluster_to_wire(&cluster)),
                })
            }
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_disable_snapshot_copy(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_disable_snapshot_copy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let mut state_guard = state.write().await;
        match state_guard.disable_snapshot_copy(&cluster_identifier) {
            Ok(cluster) => {
                wire::serialize_disable_snapshot_copy_response(&wire::DisableSnapshotCopyResult {
                    cluster: Some(cluster_to_wire(&cluster)),
                })
            }
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_modify_snapshot_copy_retention_period(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_snapshot_copy_retention_period_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let retention_period: i32 = input.retention_period;
        let mut state_guard = state.write().await;
        match state_guard
            .modify_snapshot_copy_retention_period(&cluster_identifier, retention_period)
        {
            Ok(cluster) => wire::serialize_modify_snapshot_copy_retention_period_response(
                &wire::ModifySnapshotCopyRetentionPeriodResult {
                    cluster: Some(cluster_to_wire(&cluster)),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Logging ----

    async fn handle_enable_logging(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_enable_logging_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let bucket_name = match input.bucket_name {
            Some(v) if !v.is_empty() => v,
            _ => return MockResponse::error(400, "MissingParameter", "Missing 'BucketName'"),
        };
        let s3_key_prefix = input.s3_key_prefix;

        let mut state_guard = state.write().await;
        match state_guard.enable_logging(&cluster_identifier, bucket_name, s3_key_prefix) {
            Ok((enabled, bucket, prefix)) => {
                wire::serialize_enable_logging_response(&wire::LoggingStatus {
                    logging_enabled: Some(enabled),
                    bucket_name: bucket,
                    s3_key_prefix: prefix,
                    ..Default::default()
                })
            }
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_disable_logging(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_disable_logging_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let mut state_guard = state.write().await;
        match state_guard.disable_logging(&cluster_identifier) {
            Ok((enabled, bucket, prefix)) => {
                wire::serialize_disable_logging_response(&wire::LoggingStatus {
                    logging_enabled: Some(enabled),
                    bucket_name: bucket,
                    s3_key_prefix: prefix,
                    ..Default::default()
                })
            }
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_describe_logging_status(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_logging_status_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let state_guard = state.read().await;
        match state_guard.describe_logging_status(&cluster_identifier) {
            Ok((enabled, bucket, prefix)) => {
                wire::serialize_describe_logging_status_response(&wire::LoggingStatus {
                    logging_enabled: Some(enabled),
                    bucket_name: bucket,
                    s3_key_prefix: prefix,
                    ..Default::default()
                })
            }
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Tags ----

    async fn handle_create_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_tags_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.resource_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ResourceName'");
        }
        let resource_name = input.resource_name;
        let tags = wire_tags_to_domain(Some(input.tags));
        let mut state_guard = state.write().await;
        state_guard.create_tags(&resource_name, tags);
        wire::serialize_create_tags_response()
    }

    async fn handle_delete_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_tags_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.resource_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ResourceName'");
        }
        let resource_name = input.resource_name;
        let tag_keys = input.tag_keys.items;
        let mut state_guard = state.write().await;
        state_guard.delete_tags(&resource_name, tag_keys);
        wire::serialize_delete_tags_response()
    }

    async fn handle_describe_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_tags_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let resource_name = input.resource_name.as_deref();
        let resource_type = input.resource_type.as_deref();
        let state_guard = state.read().await;
        let tagged = state_guard.describe_tags(resource_name, resource_type);
        wire::serialize_describe_tags_response(&wire::TaggedResourceListMessage {
            tagged_resources: Some(wire::TaggedResourceList {
                items: tagged
                    .iter()
                    .map(|(arn, k, v)| wire::TaggedResource {
                        resource_name: Some(arn.clone()),
                        resource_type: Some(extract_resource_type_from_arn(arn).to_string()),
                        tag: Some(wire::Tag {
                            key: Some(k.clone()),
                            value: Some(v.clone()),
                        }),
                    })
                    .collect(),
            }),
            marker: None,
        })
    }

    // ---- Reboot / Resize ----

    async fn handle_reboot_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_reboot_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let mut state_guard = state.write().await;
        match state_guard.reboot_cluster(&cluster_identifier) {
            Ok(cluster) => wire::serialize_reboot_cluster_response(&wire::RebootClusterResult {
                cluster: Some(cluster_to_wire(&cluster)),
            }),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_resize_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_resize_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let node_type = input.node_type;
        let number_of_nodes = input.number_of_nodes;
        let cluster_type = input.cluster_type;

        let mut state_guard = state.write().await;
        match state_guard.resize_cluster(
            &cluster_identifier,
            node_type,
            number_of_nodes,
            cluster_type,
        ) {
            Ok(cluster) => wire::serialize_resize_cluster_response(&wire::ResizeClusterResult {
                cluster: Some(cluster_to_wire(&cluster)),
            }),
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Modify/Reset Parameter Group ----

    async fn handle_modify_cluster_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_cluster_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.parameter_group_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ParameterGroupName'");
        }
        let name = input.parameter_group_name;
        let parameters: Vec<(String, String)> = input
            .parameters
            .items
            .iter()
            .filter_map(|p| {
                let pn = p.parameter_name.clone()?;
                let pv = p.parameter_value.clone().unwrap_or_default();
                Some((pn, pv))
            })
            .collect();
        let mut state_guard = state.write().await;
        match state_guard.modify_cluster_parameter_group(&name, parameters) {
            Ok(()) => wire::serialize_modify_cluster_parameter_group_response(
                &wire::ClusterParameterGroupNameMessage {
                    parameter_group_name: Some(name),
                    parameter_group_status: Some(
                        "Your parameter group has been updated".to_string(),
                    ),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_reset_cluster_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_reset_cluster_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.parameter_group_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ParameterGroupName'");
        }
        let name = input.parameter_group_name;
        let reset_all = input.reset_all_parameters.unwrap_or(false);
        let parameter_names: Vec<String> = input
            .parameters
            .map(|pl| {
                pl.items
                    .iter()
                    .filter_map(|p| p.parameter_name.clone())
                    .collect()
            })
            .unwrap_or_default();
        let mut state_guard = state.write().await;
        match state_guard.reset_cluster_parameter_group(&name, reset_all, parameter_names) {
            Ok(()) => wire::serialize_reset_cluster_parameter_group_response(
                &wire::ClusterParameterGroupNameMessage {
                    parameter_group_name: Some(name),
                    parameter_group_status: Some(
                        "Your parameter group has been updated".to_string(),
                    ),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Copy Cluster Snapshot ----

    async fn handle_copy_cluster_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_copy_cluster_snapshot_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.source_snapshot_identifier.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'SourceSnapshotIdentifier'",
            );
        }
        if input.target_snapshot_identifier.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'TargetSnapshotIdentifier'",
            );
        }
        let source_identifier = input.source_snapshot_identifier;
        let target_identifier = input.target_snapshot_identifier;
        let tags = collect_tags(params);
        let mut state_guard = state.write().await;
        match state_guard.copy_cluster_snapshot(
            &source_identifier,
            target_identifier,
            tags,
            region,
            account_id,
        ) {
            Ok(snap) => {
                wire::serialize_copy_cluster_snapshot_response(&wire::CopyClusterSnapshotResult {
                    snapshot: Some(snapshot_to_wire(&snap)),
                })
            }
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Revoke Security Group Ingress ----

    async fn handle_revoke_cluster_security_group_ingress(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_revoke_cluster_security_group_ingress_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_security_group_name.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'ClusterSecurityGroupName'",
            );
        }
        let name = input.cluster_security_group_name;
        let ec2_sg_name = input.e_c2_security_group_name;
        let ec2_sg_owner = input.e_c2_security_group_owner_id;
        let cidrip = input.c_i_d_r_i_p;
        let mut state_guard = state.write().await;
        match state_guard.revoke_cluster_security_group_ingress(
            &name,
            ec2_sg_name,
            ec2_sg_owner,
            cidrip,
        ) {
            Ok(sg) => wire::serialize_revoke_cluster_security_group_ingress_response(
                &wire::RevokeClusterSecurityGroupIngressResult {
                    cluster_security_group: Some(security_group_to_wire(&sg)),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Modify Cluster Subnet Group ----

    async fn handle_modify_cluster_subnet_group(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_cluster_subnet_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_subnet_group_name.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'ClusterSubnetGroupName'",
            );
        }
        let name = input.cluster_subnet_group_name;
        let description = input.description;
        let subnet_ids = input.subnet_ids.items;
        let mut state_guard = state.write().await;
        match state_guard.modify_cluster_subnet_group(&name, description, subnet_ids) {
            Ok(sg) => wire::serialize_modify_cluster_subnet_group_response(
                &wire::ModifyClusterSubnetGroupResult {
                    cluster_subnet_group: Some(subnet_group_to_wire(&sg)),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Batch Delete Cluster Snapshots ----

    async fn handle_batch_delete_cluster_snapshots(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_delete_cluster_snapshots_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let snapshot_ids: Vec<String> = input
            .identifiers
            .items
            .iter()
            .map(|msg| msg.snapshot_identifier.clone())
            .collect();
        let mut state_guard = state.write().await;
        let (deleted, errors) = state_guard.batch_delete_cluster_snapshots(snapshot_ids);
        wire::serialize_batch_delete_cluster_snapshots_response(
            &wire::BatchDeleteClusterSnapshotsResult {
                resources: Some(wire::SnapshotIdentifierList { items: deleted }),
                errors: if errors.is_empty() {
                    None
                } else {
                    Some(wire::BatchSnapshotOperationErrorList {
                        items: errors
                            .iter()
                            .map(|(id, code)| wire::SnapshotErrorMessage {
                                snapshot_identifier: Some(id.clone()),
                                snapshot_cluster_identifier: None,
                                failure_code: Some(code.clone()),
                                failure_reason: Some(format!("Snapshot '{id}' not found")),
                            })
                            .collect(),
                    })
                },
            },
        )
    }

    // ---- Event Subscriptions ----

    async fn handle_create_event_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_event_subscription_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.subscription_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SubscriptionName'");
        }
        if input.sns_topic_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SnsTopicArn'");
        }
        let subscription_name = input.subscription_name;
        let sns_topic_arn = input.sns_topic_arn;
        let source_type = input.source_type;
        let enabled = input.enabled.unwrap_or(true);
        let severity = input.severity;
        let source_ids = input.source_ids.map(|l| l.items).unwrap_or_default();
        let event_categories = input.event_categories.map(|l| l.items).unwrap_or_default();
        let tags = wire_tags_to_domain(input.tags);

        let mut state_guard = state.write().await;
        match state_guard.create_event_subscription(
            subscription_name,
            sns_topic_arn,
            source_type,
            source_ids,
            event_categories,
            severity,
            enabled,
            tags,
            account_id,
        ) {
            Ok(sub) => wire::serialize_create_event_subscription_response(
                &wire::CreateEventSubscriptionResult {
                    event_subscription: Some(event_subscription_to_wire(&sub)),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_describe_event_subscriptions(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_event_subscriptions_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let subscription_name = input.subscription_name.as_deref();
        let state_guard = state.read().await;
        match state_guard.describe_event_subscriptions(subscription_name) {
            Ok(subs) => wire::serialize_describe_event_subscriptions_response(
                &wire::EventSubscriptionsMessage {
                    event_subscriptions_list: Some(wire::EventSubscriptionsList {
                        items: subs.iter().map(event_subscription_to_wire).collect(),
                    }),
                    marker: None,
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_delete_event_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_event_subscription_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.subscription_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SubscriptionName'");
        }
        let subscription_name = input.subscription_name;
        let mut state_guard = state.write().await;
        match state_guard.delete_event_subscription(&subscription_name) {
            Ok(()) => wire::serialize_delete_event_subscription_response(),
            Err(e) => redshift_error_response(&e),
        }
    }

    // STUB[no-telemetry]: Events are driven by real infrastructure telemetry; the mock has no event history.
    async fn handle_describe_events(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        _params: &HashMap<String, String>,
    ) -> MockResponse {
        let _state_guard = state.read().await;
        wire::serialize_describe_events_response(&wire::EventsMessage {
            events: Some(wire::EventList { items: vec![] }),
            marker: None,
        })
    }

    async fn handle_describe_cluster_versions(&self) -> MockResponse {
        wire::serialize_describe_cluster_versions_response(&wire::ClusterVersionsMessage {
            cluster_versions: Some(wire::ClusterVersionList {
                items: vec![wire::ClusterVersion {
                    cluster_version: Some("1.0".to_string()),
                    cluster_parameter_group_family: Some("redshift-1.0".to_string()),
                    description: Some("Amazon Redshift 1.0".to_string()),
                }],
            }),
            marker: None,
        })
    }

    async fn handle_describe_orderable_cluster_options(&self) -> MockResponse {
        wire::serialize_describe_orderable_cluster_options_response(
            &wire::OrderableClusterOptionsMessage {
                orderable_cluster_options: Some(wire::OrderableClusterOptionsList {
                    items: vec![
                        wire::OrderableClusterOption {
                            node_type: Some("dc2.large".to_string()),
                            cluster_type: Some("multi-node".to_string()),
                            cluster_version: Some("1.0".to_string()),
                            availability_zones: None,
                        },
                        wire::OrderableClusterOption {
                            node_type: Some("dc2.8xlarge".to_string()),
                            cluster_type: Some("multi-node".to_string()),
                            cluster_version: Some("1.0".to_string()),
                            availability_zones: None,
                        },
                        wire::OrderableClusterOption {
                            node_type: Some("ra3.xlplus".to_string()),
                            cluster_type: Some("multi-node".to_string()),
                            cluster_version: Some("1.0".to_string()),
                            availability_zones: None,
                        },
                    ],
                }),
                marker: None,
            },
        )
    }

    async fn handle_describe_event_categories(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        _params: &HashMap<String, String>,
    ) -> MockResponse {
        // Read state to allow future population of event categories.
        let _state_guard = state.read().await;
        wire::serialize_describe_event_categories_response(&wire::EventCategoriesMessage {
            event_categories_map_list: Some(wire::EventCategoriesMapList { items: vec![] }),
        })
    }

    // ---- Credentials ----

    async fn handle_get_cluster_credentials(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
        _account_id: &str,
        _region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_get_cluster_credentials_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let cluster_identifier = match input.cluster_identifier {
            Some(ref v) if !v.is_empty() => v.clone(),
            _ => {
                return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
            }
        };
        if input.db_user.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'DbUser'");
        }
        let db_user = input.db_user;
        let db_name = input.db_name.as_deref();
        let state_guard = state.read().await;
        match state_guard.get_cluster_credentials(&cluster_identifier, &db_user, db_name) {
            Ok((user, password, expiration)) => {
                wire::serialize_get_cluster_credentials_response(&wire::ClusterCredentials {
                    db_user: Some(user),
                    db_password: Some(password),
                    expiration: Some(expiration),
                })
            }
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- HSM Client Certificates ----

    async fn handle_create_hsm_client_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_hsm_client_certificate_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.hsm_client_certificate_identifier.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'HsmClientCertificateIdentifier'",
            );
        }
        let identifier = input.hsm_client_certificate_identifier;
        let tags = wire_tags_to_domain(input.tags);
        let mut state_guard = state.write().await;
        match state_guard.create_hsm_client_certificate(identifier, tags, region, account_id) {
            Ok(cert) => wire::serialize_create_hsm_client_certificate_response(
                &wire::CreateHsmClientCertificateResult {
                    hsm_client_certificate: Some(wire::HsmClientCertificate {
                        hsm_client_certificate_identifier: Some(cert.identifier.clone()),
                        hsm_client_certificate_public_key: Some(cert.public_key.clone()),
                        tags: Some(wire::TagList {
                            items: cert
                                .tags
                                .iter()
                                .map(|(k, v)| wire::Tag {
                                    key: Some(k.clone()),
                                    value: Some(v.clone()),
                                })
                                .collect(),
                        }),
                    }),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_describe_hsm_client_certificates(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_hsm_client_certificates_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.hsm_client_certificate_identifier.as_deref();
        let state_guard = state.read().await;
        match state_guard.describe_hsm_client_certificates(identifier) {
            Ok(certs) => wire::serialize_describe_hsm_client_certificates_response(
                &wire::HsmClientCertificateMessage {
                    hsm_client_certificates: Some(wire::HsmClientCertificateList {
                        items: certs
                            .iter()
                            .map(|c| wire::HsmClientCertificate {
                                hsm_client_certificate_identifier: Some(c.identifier.clone()),
                                hsm_client_certificate_public_key: Some(c.public_key.clone()),
                                tags: Some(wire::TagList {
                                    items: c
                                        .tags
                                        .iter()
                                        .map(|(k, v)| wire::Tag {
                                            key: Some(k.clone()),
                                            value: Some(v.clone()),
                                        })
                                        .collect(),
                                }),
                            })
                            .collect(),
                    }),
                    marker: None,
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_delete_hsm_client_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_hsm_client_certificate_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.hsm_client_certificate_identifier.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'HsmClientCertificateIdentifier'",
            );
        }
        let identifier = input.hsm_client_certificate_identifier;
        let mut state_guard = state.write().await;
        match state_guard.delete_hsm_client_certificate(&identifier) {
            Ok(()) => wire::serialize_delete_hsm_client_certificate_response(),
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- HSM Configurations ----

    async fn handle_create_hsm_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_hsm_configuration_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.hsm_configuration_identifier.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'HsmConfigurationIdentifier'",
            );
        }
        let identifier = input.hsm_configuration_identifier;
        let description = input.description;
        let hsm_ip_address = input.hsm_ip_address;
        let hsm_partition_name = input.hsm_partition_name;
        let tags = wire_tags_to_domain(input.tags);
        let mut state_guard = state.write().await;
        match state_guard.create_hsm_configuration(
            identifier,
            description,
            hsm_ip_address,
            hsm_partition_name,
            tags,
            region,
            account_id,
        ) {
            Ok(config) => wire::serialize_create_hsm_configuration_response(
                &wire::CreateHsmConfigurationResult {
                    hsm_configuration: Some(wire::HsmConfiguration {
                        hsm_configuration_identifier: Some(config.identifier.clone()),
                        description: Some(config.description.clone()),
                        hsm_ip_address: Some(config.hsm_ip_address.clone()),
                        hsm_partition_name: Some(config.hsm_partition_name.clone()),
                        tags: Some(wire::TagList {
                            items: config
                                .tags
                                .iter()
                                .map(|(k, v)| wire::Tag {
                                    key: Some(k.clone()),
                                    value: Some(v.clone()),
                                })
                                .collect(),
                        }),
                    }),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_describe_hsm_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_hsm_configurations_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let identifier = input.hsm_configuration_identifier.as_deref();
        let state_guard = state.read().await;
        match state_guard.describe_hsm_configurations(identifier) {
            Ok(configs) => wire::serialize_describe_hsm_configurations_response(
                &wire::HsmConfigurationMessage {
                    hsm_configurations: Some(wire::HsmConfigurationList {
                        items: configs
                            .iter()
                            .map(|c| wire::HsmConfiguration {
                                hsm_configuration_identifier: Some(c.identifier.clone()),
                                description: Some(c.description.clone()),
                                hsm_ip_address: Some(c.hsm_ip_address.clone()),
                                hsm_partition_name: Some(c.hsm_partition_name.clone()),
                                tags: Some(wire::TagList {
                                    items: c
                                        .tags
                                        .iter()
                                        .map(|(k, v)| wire::Tag {
                                            key: Some(k.clone()),
                                            value: Some(v.clone()),
                                        })
                                        .collect(),
                                }),
                            })
                            .collect(),
                    }),
                    marker: None,
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_delete_hsm_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_hsm_configuration_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.hsm_configuration_identifier.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'HsmConfigurationIdentifier'",
            );
        }
        let identifier = input.hsm_configuration_identifier;
        let mut state_guard = state.write().await;
        match state_guard.delete_hsm_configuration(&identifier) {
            Ok(()) => wire::serialize_delete_hsm_configuration_response(),
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Authentication Profiles ----

    async fn handle_create_authentication_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_authentication_profile_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.authentication_profile_name.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'AuthenticationProfileName'",
            );
        }
        let name = input.authentication_profile_name;
        let content = input.authentication_profile_content;
        let mut state_guard = state.write().await;
        match state_guard.create_authentication_profile(name, content) {
            Ok(profile) => wire::serialize_create_authentication_profile_response(
                &wire::CreateAuthenticationProfileResult {
                    authentication_profile_name: Some(profile.name.clone()),
                    authentication_profile_content: Some(profile.content.clone()),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_describe_authentication_profiles(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_authentication_profiles_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.authentication_profile_name.as_deref();
        let state_guard = state.read().await;
        match state_guard.describe_authentication_profiles(name) {
            Ok(profiles) => wire::serialize_describe_authentication_profiles_response(
                &wire::DescribeAuthenticationProfilesResult {
                    authentication_profiles: Some(wire::AuthenticationProfileList {
                        items: profiles
                            .iter()
                            .map(|p| wire::AuthenticationProfile {
                                authentication_profile_name: Some(p.name.clone()),
                                authentication_profile_content: Some(p.content.clone()),
                            })
                            .collect(),
                    }),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_modify_authentication_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_authentication_profile_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.authentication_profile_name.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'AuthenticationProfileName'",
            );
        }
        let name = input.authentication_profile_name;
        let content = input.authentication_profile_content;
        let mut state_guard = state.write().await;
        match state_guard.modify_authentication_profile(&name, content) {
            Ok(profile) => wire::serialize_modify_authentication_profile_response(
                &wire::ModifyAuthenticationProfileResult {
                    authentication_profile_name: Some(profile.name.clone()),
                    authentication_profile_content: Some(profile.content.clone()),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_delete_authentication_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_authentication_profile_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.authentication_profile_name.is_empty() {
            return MockResponse::error(
                400,
                "MissingParameter",
                "Missing 'AuthenticationProfileName'",
            );
        }
        let name = input.authentication_profile_name;
        let mut state_guard = state.write().await;
        match state_guard.delete_authentication_profile(&name) {
            Ok(()) => wire::serialize_delete_authentication_profile_response(
                &wire::DeleteAuthenticationProfileResult {
                    authentication_profile_name: Some(name),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Usage Limits ----

    async fn handle_create_usage_limit(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_usage_limit_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let feature_type = input.feature_type;
        let limit_type = input.limit_type;
        let amount: i64 = input.amount;
        let period = input.period;
        let breach_action = input.breach_action;
        let tags = wire_tags_to_domain(input.tags);
        let mut state_guard = state.write().await;
        match state_guard.create_usage_limit(
            cluster_identifier,
            feature_type,
            limit_type,
            amount,
            period,
            breach_action,
            tags,
        ) {
            Ok(limit) => wire::serialize_create_usage_limit_response(&usage_limit_to_wire(&limit)),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_describe_usage_limits(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_usage_limits_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let usage_limit_id = input.usage_limit_id.as_deref();
        let cluster_identifier = input.cluster_identifier.as_deref();
        let state_guard = state.read().await;
        let limits = state_guard.describe_usage_limits(usage_limit_id, cluster_identifier);
        wire::serialize_describe_usage_limits_response(&wire::UsageLimitList {
            usage_limits: Some(wire::UsageLimits {
                items: limits.iter().map(usage_limit_to_wire).collect(),
            }),
            marker: None,
        })
    }

    async fn handle_modify_usage_limit(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_usage_limit_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.usage_limit_id.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UsageLimitId'");
        }
        let usage_limit_id = input.usage_limit_id;
        let amount = input.amount;
        let breach_action = input.breach_action;
        let mut state_guard = state.write().await;
        match state_guard.modify_usage_limit(&usage_limit_id, amount, breach_action) {
            Ok(limit) => wire::serialize_modify_usage_limit_response(&usage_limit_to_wire(&limit)),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_delete_usage_limit(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_usage_limit_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.usage_limit_id.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'UsageLimitId'");
        }
        let usage_limit_id = input.usage_limit_id;
        let mut state_guard = state.write().await;
        match state_guard.delete_usage_limit(&usage_limit_id) {
            Ok(()) => wire::serialize_delete_usage_limit_response(),
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Snapshot Schedules ----

    async fn handle_create_snapshot_schedule(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_snapshot_schedule_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let schedule_identifier = input.schedule_identifier;
        let description = input.schedule_description;
        let tags = wire_tags_to_domain(input.tags);
        let definitions = input
            .schedule_definitions
            .map(|l| l.items)
            .unwrap_or_default();
        let mut state_guard = state.write().await;
        match state_guard.create_snapshot_schedule(
            schedule_identifier,
            definitions,
            description,
            tags,
        ) {
            Ok(sched) => wire::serialize_create_snapshot_schedule_response(
                &snapshot_schedule_to_wire(&sched),
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_describe_snapshot_schedules(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_snapshot_schedules_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let schedule_identifier = input.schedule_identifier.as_deref();
        let state_guard = state.read().await;
        let schedules = state_guard.describe_snapshot_schedules(schedule_identifier);
        wire::serialize_describe_snapshot_schedules_response(
            &wire::DescribeSnapshotSchedulesOutputMessage {
                snapshot_schedules: Some(wire::SnapshotScheduleList {
                    items: schedules.iter().map(snapshot_schedule_to_wire).collect(),
                }),
                marker: None,
            },
        )
    }

    async fn handle_modify_snapshot_schedule(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_snapshot_schedule_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.schedule_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ScheduleIdentifier'");
        }
        let schedule_identifier = input.schedule_identifier;
        let definitions = input.schedule_definitions.items;
        let mut state_guard = state.write().await;
        match state_guard.modify_snapshot_schedule(&schedule_identifier, definitions) {
            Ok(sched) => wire::serialize_modify_snapshot_schedule_response(
                &snapshot_schedule_to_wire(&sched),
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_delete_snapshot_schedule(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_snapshot_schedule_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.schedule_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ScheduleIdentifier'");
        }
        let schedule_identifier = input.schedule_identifier;
        let mut state_guard = state.write().await;
        match state_guard.delete_snapshot_schedule(&schedule_identifier) {
            Ok(()) => wire::serialize_delete_snapshot_schedule_response(),
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Scheduled Actions ----

    async fn handle_create_scheduled_action(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_scheduled_action_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.scheduled_action_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ScheduledActionName'");
        }
        let name = input.scheduled_action_name;
        let schedule = if input.schedule.is_empty() {
            None
        } else {
            Some(input.schedule)
        };
        let iam_role = if input.iam_role.is_empty() {
            None
        } else {
            Some(input.iam_role)
        };
        let description = input.scheduled_action_description;
        let start_time = input.start_time;
        let end_time = input.end_time;
        let mut state_guard = state.write().await;
        match state_guard.create_scheduled_action(
            name,
            schedule,
            iam_role,
            description,
            start_time,
            end_time,
        ) {
            Ok(action) => {
                wire::serialize_create_scheduled_action_response(&scheduled_action_to_wire(&action))
            }
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_describe_scheduled_actions(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_scheduled_actions_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let name = input.scheduled_action_name.as_deref();
        let state_guard = state.read().await;
        let actions = state_guard.describe_scheduled_actions(name);
        wire::serialize_describe_scheduled_actions_response(&wire::ScheduledActionsMessage {
            scheduled_actions: Some(wire::ScheduledActionList {
                items: actions.iter().map(scheduled_action_to_wire).collect(),
            }),
            marker: None,
        })
    }

    async fn handle_modify_scheduled_action(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_scheduled_action_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.scheduled_action_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ScheduledActionName'");
        }
        let name = input.scheduled_action_name;
        let schedule = input.schedule;
        let iam_role = input.iam_role;
        let description = input.scheduled_action_description;
        let start_time = input.start_time;
        let end_time = input.end_time;
        let mut state_guard = state.write().await;
        match state_guard.modify_scheduled_action(
            &name,
            schedule,
            iam_role,
            description,
            start_time,
            end_time,
        ) {
            Ok(action) => {
                wire::serialize_modify_scheduled_action_response(&scheduled_action_to_wire(&action))
            }
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_delete_scheduled_action(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_scheduled_action_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.scheduled_action_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ScheduledActionName'");
        }
        let name = input.scheduled_action_name;
        let mut state_guard = state.write().await;
        match state_guard.delete_scheduled_action(&name) {
            Ok(()) => wire::serialize_delete_scheduled_action_response(),
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Snapshot Access ----

    async fn handle_authorize_snapshot_access(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_authorize_snapshot_access_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let snapshot_identifier = match input.snapshot_identifier {
            Some(ref v) if !v.is_empty() => v.clone(),
            _ => {
                return MockResponse::error(
                    400,
                    "MissingParameter",
                    "Missing 'SnapshotIdentifier'",
                );
            }
        };
        let account_with_restore_access = input.account_with_restore_access;
        let mut state_guard = state.write().await;
        match state_guard
            .authorize_snapshot_access(&snapshot_identifier, account_with_restore_access)
        {
            Ok(snap) => wire::serialize_authorize_snapshot_access_response(
                &wire::AuthorizeSnapshotAccessResult {
                    snapshot: Some(snapshot_to_wire(&snap)),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_revoke_snapshot_access(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_revoke_snapshot_access_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let snapshot_identifier = match input.snapshot_identifier {
            Some(ref v) if !v.is_empty() => v.clone(),
            _ => {
                return MockResponse::error(
                    400,
                    "MissingParameter",
                    "Missing 'SnapshotIdentifier'",
                );
            }
        };
        let mut state_guard = state.write().await;
        match state_guard.revoke_snapshot_access(&snapshot_identifier) {
            Ok(snap) => {
                wire::serialize_revoke_snapshot_access_response(&wire::RevokeSnapshotAccessResult {
                    snapshot: Some(snapshot_to_wire(&snap)),
                })
            }
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_modify_cluster_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_cluster_snapshot_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.snapshot_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SnapshotIdentifier'");
        }
        let snapshot_identifier = input.snapshot_identifier;
        let state_guard = state.read().await;
        match state_guard.describe_cluster_snapshots(Some(&snapshot_identifier), None) {
            Ok(snaps) => wire::serialize_modify_cluster_snapshot_response(
                &wire::ModifyClusterSnapshotResult {
                    snapshot: snaps.first().map(snapshot_to_wire),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_batch_modify_cluster_snapshots(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        _params: &HashMap<String, String>,
    ) -> MockResponse {
        let _state_guard = state.read().await;
        wire::serialize_batch_modify_cluster_snapshots_response(
            &wire::BatchModifyClusterSnapshotsOutputMessage {
                resources: Some(wire::SnapshotIdentifierList { items: vec![] }),
                errors: None,
            },
        )
    }

    // ---- Cluster IAM Roles ----

    async fn handle_modify_cluster_iam_roles(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_cluster_iam_roles_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let mut state_guard = state.write().await;
        match state_guard.modify_cluster_iam_roles(&cluster_identifier) {
            Ok(cluster) => wire::serialize_modify_cluster_iam_roles_response(
                &wire::ModifyClusterIamRolesResult {
                    cluster: Some(cluster_to_wire(&cluster)),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Cluster Maintenance ----

    async fn handle_modify_cluster_maintenance(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_cluster_maintenance_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let mut state_guard = state.write().await;
        match state_guard.modify_cluster_maintenance(&cluster_identifier) {
            Ok(cluster) => wire::serialize_modify_cluster_maintenance_response(
                &wire::ModifyClusterMaintenanceResult {
                    cluster: Some(cluster_to_wire(&cluster)),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Rotate Encryption Key ----

    async fn handle_rotate_encryption_key(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_rotate_encryption_key_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let mut state_guard = state.write().await;
        match state_guard.rotate_encryption_key(&cluster_identifier) {
            Ok(cluster) => {
                wire::serialize_rotate_encryption_key_response(&wire::RotateEncryptionKeyResult {
                    cluster: Some(cluster_to_wire(&cluster)),
                })
            }
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Failover Primary Compute ----

    async fn handle_failover_primary_compute(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_failover_primary_compute_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let mut state_guard = state.write().await;
        match state_guard.failover_primary_compute(&cluster_identifier) {
            Ok(cluster) => wire::serialize_failover_primary_compute_response(
                &wire::FailoverPrimaryComputeResult {
                    cluster: Some(cluster_to_wire(&cluster)),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Static / Info endpoints ----

    async fn handle_describe_account_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
    ) -> MockResponse {
        // Read state to allow future population of account attributes.
        let _state_guard = state.read().await;
        wire::serialize_describe_account_attributes_response(&wire::AccountAttributeList {
            account_attributes: Some(wire::AttributeList { items: vec![] }),
        })
    }

    // STUB[no-telemetry]: Storage usage figures are driven by real infrastructure; the mock returns zero values.
    async fn handle_describe_storage(&self) -> MockResponse {
        wire::serialize_describe_storage_response(&wire::CustomerStorageMessage {
            total_backup_size_in_mega_bytes: Some(0.0),
            total_provisioned_storage_in_mega_bytes: Some(0.0),
        })
    }

    async fn handle_describe_cluster_tracks(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
    ) -> MockResponse {
        let _state_guard = state.read().await;
        wire::serialize_describe_cluster_tracks_response(&wire::TrackListMessage {
            maintenance_tracks: Some(wire::TrackList {
                items: vec![wire::MaintenanceTrack {
                    maintenance_track_name: Some("current".to_string()),
                    database_version: Some("1.0".to_string()),
                    ..Default::default()
                }],
            }),
            marker: None,
        })
    }

    async fn handle_describe_resize(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_resize_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let state_guard = state.read().await;
        match state_guard.describe_clusters(Some(&cluster_identifier)) {
            Ok(_) => wire::serialize_describe_resize_response(&wire::ResizeProgressMessage {
                status: Some("NONE".to_string()),
                ..Default::default()
            }),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_cancel_resize(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_resize_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let state_guard = state.read().await;
        match state_guard.describe_clusters(Some(&cluster_identifier)) {
            Ok(_) => wire::serialize_cancel_resize_response(&wire::ResizeProgressMessage {
                status: Some("CANCELLED".to_string()),
                ..Default::default()
            }),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_get_cluster_credentials_with_iam(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
        _account_id: &str,
        _region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_get_cluster_credentials_with_i_a_m_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let cluster_identifier = match input.cluster_identifier {
            Some(ref v) if !v.is_empty() => v.clone(),
            _ => {
                return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
            }
        };
        let state_guard = state.read().await;
        match state_guard.describe_clusters(Some(&cluster_identifier)) {
            Ok(_) => {
                let expiration = chrono::Utc::now()
                    .checked_add_signed(chrono::Duration::seconds(900))
                    .map(|t| t.to_rfc3339())
                    .unwrap_or_default();
                wire::serialize_get_cluster_credentials_with_i_a_m_response(
                    &wire::ClusterExtendedCredentials {
                        db_user: Some(format!("IAM:{cluster_identifier}-user")),
                        db_password: Some("IamPassword123!".to_string()),
                        expiration: Some(expiration),
                        next_refresh_time: None,
                    },
                )
            }
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_modify_cluster_db_revision(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_cluster_db_revision_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let state_guard = state.read().await;
        match state_guard.describe_clusters(Some(&cluster_identifier)) {
            Ok(clusters) => wire::serialize_modify_cluster_db_revision_response(
                &wire::ModifyClusterDbRevisionResult {
                    cluster: clusters.first().map(cluster_to_wire),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_describe_cluster_db_revisions(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_cluster_db_revisions_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let cluster_identifier = input.cluster_identifier.as_deref();
        let state_guard = state.read().await;
        let clusters = if let Some(id) = cluster_identifier {
            match state_guard.describe_clusters(Some(id)) {
                Ok(c) => c,
                Err(e) => return redshift_error_response(&e),
            }
        } else {
            state_guard.clusters.values().cloned().collect()
        };
        wire::serialize_describe_cluster_db_revisions_response(&wire::ClusterDbRevisionsMessage {
            cluster_db_revisions: Some(wire::ClusterDbRevisionsList {
                items: clusters
                    .iter()
                    .map(|c| wire::ClusterDbRevision {
                        cluster_identifier: Some(c.cluster_identifier.clone()),
                        current_database_revision: Some("1.0".to_string()),
                        ..Default::default()
                    })
                    .collect(),
            }),
            marker: None,
        })
    }

    // ---- Modify Event Subscription ----

    async fn handle_modify_event_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_event_subscription_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.subscription_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'SubscriptionName'");
        }
        let subscription_name = input.subscription_name;
        let sns_topic_arn = input.sns_topic_arn;
        let source_type = input.source_type;
        let enabled = input.enabled;
        let severity = input.severity;
        let source_ids_opt = input.source_ids.map(|l| l.items);
        let event_cats_opt = input.event_categories.map(|l| l.items);
        let mut state_guard = state.write().await;
        match state_guard.modify_event_subscription(
            &subscription_name,
            sns_topic_arn,
            source_type,
            source_ids_opt,
            event_cats_opt,
            severity,
            enabled,
        ) {
            Ok(sub) => wire::serialize_modify_event_subscription_response(
                &wire::ModifyEventSubscriptionResult {
                    event_subscription: Some(event_subscription_to_wire(&sub)),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Resource Policies ----

    async fn handle_put_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_resource_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.resource_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ResourceArn'");
        }
        let resource_arn = input.resource_arn;
        let policy = input.policy;
        let mut state_guard = state.write().await;
        let rp = state_guard.put_resource_policy(resource_arn, policy);
        wire::serialize_put_resource_policy_response(&wire::PutResourcePolicyResult {
            resource_policy: Some(wire::ResourcePolicy {
                resource_arn: Some(rp.resource_arn.clone()),
                policy: Some(rp.policy.clone()),
            }),
        })
    }

    async fn handle_get_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_resource_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.resource_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ResourceArn'");
        }
        let resource_arn = input.resource_arn;
        let state_guard = state.read().await;
        match state_guard.get_resource_policy(&resource_arn) {
            Some(rp) => {
                wire::serialize_get_resource_policy_response(&wire::GetResourcePolicyResult {
                    resource_policy: Some(wire::ResourcePolicy {
                        resource_arn: Some(rp.resource_arn.clone()),
                        policy: Some(rp.policy.clone()),
                    }),
                })
            }
            None => MockResponse::error(
                404,
                "ResourceNotFoundFault",
                &format!("Resource policy for ARN '{resource_arn}' not found"),
            ),
        }
    }

    async fn handle_delete_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_resource_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.resource_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ResourceArn'");
        }
        let resource_arn = input.resource_arn;
        let mut state_guard = state.write().await;
        state_guard.delete_resource_policy(&resource_arn);
        wire::serialize_delete_resource_policy_response()
    }

    // ---- Partner Integrations ----

    async fn handle_add_partner(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_add_partner_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let database_name = input.database_name;
        let partner_name = input.partner_name;
        let mut state_guard = state.write().await;
        match state_guard.add_partner(cluster_identifier, database_name, partner_name) {
            Ok(pi) => {
                wire::serialize_add_partner_response(&wire::PartnerIntegrationOutputMessage {
                    database_name: Some(pi.database_name.clone()),
                    partner_name: Some(pi.partner_name.clone()),
                })
            }
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_describe_partners(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_partners_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let database_name = input.database_name.as_deref();
        let partner_name = input.partner_name.as_deref();
        let state_guard = state.read().await;
        let partners =
            state_guard.describe_partners(&cluster_identifier, database_name, partner_name);
        wire::serialize_describe_partners_response(&wire::DescribePartnersOutputMessage {
            partner_integration_info_list: Some(wire::PartnerIntegrationInfoList {
                items: partners
                    .iter()
                    .map(|p| wire::PartnerIntegrationInfo {
                        partner_name: Some(p.partner_name.clone()),
                        database_name: Some(p.database_name.clone()),
                        status: Some(p.status.clone()),
                        status_message: p.status_message.clone(),
                        ..Default::default()
                    })
                    .collect(),
            }),
        })
    }

    async fn handle_delete_partner(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_partner_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let database_name = input.database_name;
        let partner_name = input.partner_name;
        let mut state_guard = state.write().await;
        match state_guard.delete_partner(&cluster_identifier, &database_name, &partner_name) {
            Ok(pi) => {
                wire::serialize_delete_partner_response(&wire::PartnerIntegrationOutputMessage {
                    database_name: Some(pi.database_name.clone()),
                    partner_name: Some(pi.partner_name.clone()),
                })
            }
            Err(e) => redshift_error_response(&e),
        }
    }

    async fn handle_update_partner_status(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_partner_status_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let database_name = input.database_name;
        let partner_name = input.partner_name;
        let status = if input.status.is_empty() {
            "Active".to_string()
        } else {
            input.status
        };
        let status_message = input.status_message;
        let mut state_guard = state.write().await;
        match state_guard.update_partner_status(
            &cluster_identifier,
            &database_name,
            &partner_name,
            status,
            status_message,
        ) {
            Ok(pi) => wire::serialize_update_partner_status_response(
                &wire::PartnerIntegrationOutputMessage {
                    database_name: Some(pi.database_name.clone()),
                    partner_name: Some(pi.partner_name.clone()),
                },
            ),
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- ModifyClusterSnapshotSchedule ----

    async fn handle_modify_cluster_snapshot_schedule(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_cluster_snapshot_schedule_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let state_guard = state.read().await;
        match state_guard.describe_clusters(Some(&cluster_identifier)) {
            Ok(_) => wire::serialize_modify_cluster_snapshot_schedule_response(),
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- ModifyAquaConfiguration ----

    async fn handle_modify_aqua_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_aqua_configuration_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cluster_identifier.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ClusterIdentifier'");
        }
        let cluster_identifier = input.cluster_identifier;
        let mut state_guard = state.write().await;
        match state_guard.describe_clusters(Some(&cluster_identifier)) {
            Ok(_) => {
                let aqua_config_status =
                    input.aqua_configuration_status.as_deref().unwrap_or("auto");
                let (config_status, aqua_status) =
                    state_guard.set_aqua_configuration(&cluster_identifier, aqua_config_status);
                wire::serialize_modify_aqua_configuration_response(&wire::ModifyAquaOutputMessage {
                    aqua_configuration: Some(wire::AquaConfiguration {
                        aqua_configuration_status: Some(config_status),
                        aqua_status: Some(aqua_status),
                    }),
                })
            }
            Err(e) => redshift_error_response(&e),
        }
    }

    // ---- Read-only list endpoints ----

    async fn handle_describe_node_configuration_options(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
    ) -> MockResponse {
        let _state_guard = state.read().await;
        wire::serialize_describe_node_configuration_options_response(
            &wire::NodeConfigurationOptionsMessage {
                node_configuration_option_list: None,
                marker: None,
            },
        )
    }

    async fn handle_describe_reserved_nodes(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
    ) -> MockResponse {
        let state_guard = state.read().await;
        let _nodes = state_guard.list_reserved_nodes();
        wire::serialize_describe_reserved_nodes_response(&wire::ReservedNodesMessage {
            reserved_nodes: None,
            marker: None,
        })
    }

    async fn handle_describe_reserved_node_offerings(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
    ) -> MockResponse {
        let _state_guard = state.read().await;
        wire::serialize_describe_reserved_node_offerings_response(
            &wire::ReservedNodeOfferingsMessage {
                reserved_node_offerings: None,
                marker: None,
            },
        )
    }

    async fn handle_describe_table_restore_status(
        &self,
        state: &Arc<tokio::sync::RwLock<RedshiftState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_table_restore_status_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let cluster_identifier = input.cluster_identifier.as_deref();
        let state_guard = state.read().await;
        let _statuses = state_guard.list_table_restore_statuses(cluster_identifier);
        wire::serialize_describe_table_restore_status_response(&wire::TableRestoreStatusMessage {
            table_restore_status_details: None,
            marker: None,
        })
    }

    // STUB[no-engine]: Advisor recommendations require real telemetry and analysis engines; always returns empty.
    async fn handle_list_recommendations(&self) -> MockResponse {
        wire::serialize_list_recommendations_response(&wire::ListRecommendationsResult {
            recommendations: None,
            marker: None,
        })
    }
}

// ---- Wire conversion helpers ----

fn cluster_to_wire(c: &crate::types::RedshiftCluster) -> wire::Cluster {
    wire::Cluster {
        cluster_identifier: Some(c.cluster_identifier.clone()),
        node_type: Some(c.node_type.clone()),
        cluster_status: Some(c.cluster_status.as_str().to_owned()),
        cluster_availability_status: c.cluster_status.availability_status().map(str::to_owned),
        master_username: Some(c.master_username.clone()),
        d_b_name: Some(c.db_name.clone()),
        cluster_subnet_group_name: c.cluster_subnet_group_name.clone(),
        vpc_id: c.vpc_id.clone(),
        availability_zone: c.availability_zone.clone(),
        number_of_nodes: Some(c.number_of_nodes),
        cluster_namespace_arn: Some(c.arn.clone()),
        endpoint: c.endpoint_address.as_ref().map(|addr| wire::Endpoint {
            address: Some(addr.clone()),
            port: c.endpoint_port,
            ..Default::default()
        }),
        cluster_version: Some(c.cluster_version.clone()),
        encrypted: Some(c.encrypted),
        publicly_accessible: Some(c.publicly_accessible),
        tags: Some(wire::TagList {
            items: c
                .tags
                .iter()
                .map(|(k, v)| wire::Tag {
                    key: Some(k.clone()),
                    value: Some(v.clone()),
                })
                .collect(),
        }),
        cluster_snapshot_copy_status: c.snapshot_copy.as_ref().map(|(dest, ret, grant)| {
            wire::ClusterSnapshotCopyStatus {
                destination_region: Some(dest.clone()),
                retention_period: Some(*ret as i64),
                snapshot_copy_grant_name: grant.clone(),
                ..Default::default()
            }
        }),
        preferred_maintenance_window: c.preferred_maintenance_window.clone(),
        automated_snapshot_retention_period: Some(c.automated_snapshot_retention_period),
        availability_zone_relocation_status: Some(
            if c.availability_zone_relocation {
                "enabled"
            } else {
                "disabled"
            }
            .to_owned(),
        ),
        // FIX(terraform-e2e): terraform-provider-aws accesses ClusterNodes[0] in
        //   resourceClusterRead (cluster.go:647) without a length guard, causing a
        //   panic when the slice is empty.  Always return at least one node so the
        //   provider can read back the cluster it just created.
        cluster_nodes: {
            let addr = c.endpoint_address.as_deref().unwrap_or("127.0.0.1");
            let nodes: Vec<wire::ClusterNode> = if c.number_of_nodes > 1 {
                let mut v = vec![wire::ClusterNode {
                    node_role: Some("LEADER".to_string()),
                    private_i_p_address: Some(addr.to_string()),
                    public_i_p_address: Some(addr.to_string()),
                }];
                for _ in 0..c.number_of_nodes - 1 {
                    v.push(wire::ClusterNode {
                        node_role: Some("COMPUTE".to_string()),
                        private_i_p_address: Some(addr.to_string()),
                        public_i_p_address: Some(addr.to_string()),
                    });
                }
                v
            } else {
                vec![wire::ClusterNode {
                    node_role: Some("SHARED".to_string()),
                    private_i_p_address: Some(addr.to_string()),
                    public_i_p_address: Some(addr.to_string()),
                }]
            };
            Some(wire::ClusterNodesList { items: nodes })
        },
        // FIX(terraform-e2e): terraform-provider-aws accesses ClusterParameterGroups[0]
        //   in resourceClusterRead (cluster.go:647) without a length guard, panicking
        //   when the list is empty.  Always return the default parameter group.
        cluster_parameter_groups: Some(wire::ClusterParameterGroupStatusList {
            items: vec![wire::ClusterParameterGroupStatus {
                parameter_group_name: Some("default.redshift-1.0".to_string()),
                parameter_apply_status: Some("in-sync".to_string()),
                cluster_parameter_status_list: None,
            }],
        }),
        // FIX(terraform-e2e): terraform-provider-aws rejects an empty MultiAZ string
        //   with "unexpected MultiAZ value".  Return "Disabled" as the default.
        multi_a_z: Some("Disabled".to_string()),
        ..Default::default()
    }
}

fn subnet_group_to_wire(sg: &crate::types::RedshiftSubnetGroup) -> wire::ClusterSubnetGroup {
    wire::ClusterSubnetGroup {
        cluster_subnet_group_name: Some(sg.name.clone()),
        description: Some(sg.description.clone()),
        vpc_id: Some(sg.vpc_id.clone()),
        subnet_group_status: Some("Complete".to_string()),
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
        tags: Some(wire::TagList {
            items: sg
                .tags
                .iter()
                .map(|(k, v)| wire::Tag {
                    key: Some(k.clone()),
                    value: Some(v.clone()),
                })
                .collect(),
        }),
        ..Default::default()
    }
}

fn parameter_group_to_wire(
    pg: &crate::types::RedshiftParameterGroup,
) -> wire::ClusterParameterGroup {
    wire::ClusterParameterGroup {
        description: Some(pg.description.clone()),
        parameter_group_family: Some(pg.family.clone()),
        parameter_group_name: Some(pg.name.clone()),
        tags: Some(wire::TagList {
            items: pg
                .tags
                .iter()
                .map(|(k, v)| wire::Tag {
                    key: Some(k.clone()),
                    value: Some(v.clone()),
                })
                .collect(),
        }),
    }
}

fn security_group_to_wire(sg: &crate::types::RedshiftSecurityGroup) -> wire::ClusterSecurityGroup {
    wire::ClusterSecurityGroup {
        cluster_security_group_name: Some(sg.name.clone()),
        description: Some(sg.description.clone()),
        e_c2_security_groups: Some(wire::EC2SecurityGroupList {
            items: sg
                .ec2_security_groups
                .iter()
                .map(|e| wire::EC2SecurityGroup {
                    e_c2_security_group_name: Some(e.ec2_security_group_name.clone()),
                    e_c2_security_group_owner_id: e.ec2_security_group_owner_id.clone(),
                    status: Some(e.status.clone()),
                    ..Default::default()
                })
                .collect(),
        }),
        i_p_ranges: Some(wire::IPRangeList {
            items: sg
                .ip_ranges
                .iter()
                .map(|ip| wire::IPRange {
                    c_i_d_r_i_p: Some(ip.cidrip.clone()),
                    status: Some(ip.status.clone()),
                    ..Default::default()
                })
                .collect(),
        }),
        tags: Some(wire::TagList {
            items: sg
                .tags
                .iter()
                .map(|(k, v)| wire::Tag {
                    key: Some(k.clone()),
                    value: Some(v.clone()),
                })
                .collect(),
        }),
    }
}

fn snapshot_to_wire(snap: &crate::types::RedshiftSnapshot) -> wire::Snapshot {
    wire::Snapshot {
        snapshot_identifier: Some(snap.snapshot_identifier.clone()),
        cluster_identifier: Some(snap.cluster_identifier.clone()),
        status: Some(snap.status.clone()),
        master_username: Some(snap.master_username.clone()),
        d_b_name: Some(snap.db_name.clone()),
        node_type: Some(snap.node_type.clone()),
        number_of_nodes: Some(snap.number_of_nodes),
        cluster_version: Some(snap.cluster_version.clone()),
        tags: Some(wire::TagList {
            items: snap
                .tags
                .iter()
                .map(|(k, v)| wire::Tag {
                    key: Some(k.clone()),
                    value: Some(v.clone()),
                })
                .collect(),
        }),
        ..Default::default()
    }
}

fn param_to_wire(p: &crate::types::RedshiftParameter) -> wire::Parameter {
    wire::Parameter {
        parameter_name: Some(p.name.clone()),
        parameter_value: Some(p.value.clone()),
        description: Some(p.description.clone()),
        is_modifiable: Some(p.is_modifiable),
        apply_type: Some(p.apply_type.clone()),
        ..Default::default()
    }
}

fn event_subscription_to_wire(
    sub: &crate::types::RedshiftEventSubscription,
) -> wire::EventSubscription {
    wire::EventSubscription {
        cust_subscription_id: Some(sub.subscription_name.clone()),
        customer_aws_id: Some(sub.customer_aws_id.clone()),
        enabled: Some(sub.enabled),
        event_categories_list: Some(wire::EventCategoriesList {
            items: sub.event_categories.clone(),
        }),
        severity: sub.severity.clone(),
        sns_topic_arn: Some(sub.sns_topic_arn.clone()),
        source_ids_list: Some(wire::SourceIdsList {
            items: sub.source_ids.clone(),
        }),
        source_type: sub.source_type.clone(),
        status: Some(sub.status.clone()),
        subscription_creation_time: Some(sub.creation_time.clone()),
        tags: Some(wire::TagList {
            items: sub
                .tags
                .iter()
                .map(|(k, v)| wire::Tag {
                    key: Some(k.clone()),
                    value: Some(v.clone()),
                })
                .collect(),
        }),
    }
}

fn extract_resource_type_from_arn(arn: &str) -> &str {
    // arn:aws:redshift:{region}:{account}:{type}:{name}
    let parts: Vec<&str> = arn.splitn(7, ':').collect();
    if parts.len() >= 6 { parts[5] } else { "" }
}

fn usage_limit_to_wire(l: &crate::types::RedshiftUsageLimit) -> wire::UsageLimit {
    wire::UsageLimit {
        usage_limit_id: Some(l.usage_limit_id.clone()),
        cluster_identifier: Some(l.cluster_identifier.clone()),
        feature_type: Some(l.feature_type.clone()),
        limit_type: Some(l.limit_type.clone()),
        amount: Some(l.amount),
        period: l.period.clone(),
        breach_action: l.breach_action.clone(),
        tags: Some(wire::TagList {
            items: l
                .tags
                .iter()
                .map(|(k, v)| wire::Tag {
                    key: Some(k.clone()),
                    value: Some(v.clone()),
                })
                .collect(),
        }),
    }
}

fn snapshot_schedule_to_wire(s: &crate::types::RedshiftSnapshotSchedule) -> wire::SnapshotSchedule {
    wire::SnapshotSchedule {
        schedule_identifier: Some(s.schedule_identifier.clone()),
        schedule_description: s.schedule_description.clone(),
        schedule_definitions: Some(wire::ScheduleDefinitionList {
            items: s.schedule_definitions.clone(),
        }),
        tags: Some(wire::TagList {
            items: s
                .tags
                .iter()
                .map(|(k, v)| wire::Tag {
                    key: Some(k.clone()),
                    value: Some(v.clone()),
                })
                .collect(),
        }),
        ..Default::default()
    }
}

fn scheduled_action_to_wire(a: &crate::types::RedshiftScheduledAction) -> wire::ScheduledAction {
    wire::ScheduledAction {
        scheduled_action_name: Some(a.name.clone()),
        schedule: a.schedule.clone(),
        iam_role: a.iam_role.clone(),
        scheduled_action_description: a.description.clone(),
        start_time: a.start_time.clone(),
        end_time: a.end_time.clone(),
        state: Some(a.state.clone()),
        ..Default::default()
    }
}

fn redshift_error_response(e: &RedshiftError) -> MockResponse {
    let (status, error_type) = match e {
        RedshiftError::ClusterNotFound(_) => (404, "ClusterNotFound"),
        RedshiftError::ClusterAlreadyExists(_) => (400, "ClusterAlreadyExists"),
        RedshiftError::InvalidParameterValue(_) => (400, "InvalidParameterValue"),
        RedshiftError::ClusterSubnetGroupAlreadyExists(_) => {
            (400, "ClusterSubnetGroupAlreadyExists")
        }
        RedshiftError::ClusterSubnetGroupNotFoundFault(_) => {
            (404, "ClusterSubnetGroupNotFoundFault")
        }
        RedshiftError::ClusterParameterGroupAlreadyExists(_) => {
            (400, "ClusterParameterGroupAlreadyExists")
        }
        RedshiftError::ClusterParameterGroupNotFound(_) => (404, "ClusterParameterGroupNotFound"),
        RedshiftError::ClusterSecurityGroupAlreadyExists(_) => {
            (400, "ClusterSecurityGroupAlreadyExists")
        }
        RedshiftError::ClusterSecurityGroupNotFound(_) => (404, "ClusterSecurityGroupNotFound"),
        RedshiftError::ClusterSnapshotAlreadyExists(_) => (400, "ClusterSnapshotAlreadyExists"),
        RedshiftError::ClusterSnapshotNotFound(_) => (404, "ClusterSnapshotNotFound"),
        RedshiftError::SnapshotCopyGrantAlreadyExistsFault(_) => {
            (400, "SnapshotCopyGrantAlreadyExistsFault")
        }
        RedshiftError::SnapshotCopyGrantNotFoundFault(_) => (404, "SnapshotCopyGrantNotFoundFault"),
        RedshiftError::SnapshotCopyDisabledFault(_) => (400, "SnapshotCopyDisabledFault"),
        RedshiftError::SubscriptionAlreadyExistFault(_) => (400, "SubscriptionAlreadyExistFault"),
        RedshiftError::SubscriptionNotFoundFault(_) => (404, "SubscriptionNotFoundFault"),
        RedshiftError::HsmClientCertificateAlreadyExistsFault(_) => {
            (400, "HsmClientCertificateAlreadyExistsFault")
        }
        RedshiftError::HsmClientCertificateNotFoundFault(_) => {
            (404, "HsmClientCertificateNotFoundFault")
        }
        RedshiftError::HsmConfigurationAlreadyExistsFault(_) => {
            (400, "HsmConfigurationAlreadyExistsFault")
        }
        RedshiftError::HsmConfigurationNotFoundFault(_) => (404, "HsmConfigurationNotFoundFault"),
        RedshiftError::AuthenticationProfileAlreadyExistsFault(_) => {
            (400, "AuthenticationProfileAlreadyExistsFault")
        }
        RedshiftError::AuthenticationProfileNotFoundFault(_) => {
            (404, "AuthenticationProfileNotFoundFault")
        }
        RedshiftError::UsageLimitNotFoundFault(_) => (404, "UsageLimitNotFoundFault"),
        RedshiftError::SnapshotScheduleAlreadyExistsFault(_) => {
            (400, "SnapshotScheduleAlreadyExistsFault")
        }
        RedshiftError::SnapshotScheduleNotFoundFault(_) => (404, "SnapshotScheduleNotFoundFault"),
        RedshiftError::ScheduledActionAlreadyExistsFault(_) => {
            (400, "ScheduledActionAlreadyExistsFault")
        }
        RedshiftError::ScheduledActionNotFoundFault(_) => (404, "ScheduledActionNotFoundFault"),
        RedshiftError::PartnerNotFoundFault(_) => (404, "PartnerNotFoundFault"),
    };
    MockResponse::error(status, error_type, &e.to_string())
}

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
    let mut bytes = s.bytes();
    while let Some(b) = bytes.next() {
        match b {
            b'+' => result.push(' '),
            b'%' => {
                let hi = bytes.next().and_then(hex_val);
                let lo = bytes.next().and_then(hex_val);
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

/// Convert wire Tag list to domain (key, value) pairs.
fn wire_tags_to_domain(tags: Option<wire::TagList>) -> Vec<(String, String)> {
    tags.map(|tl| {
        tl.items
            .iter()
            .filter_map(|t| {
                let k = t.key.clone()?;
                let v = t.value.clone().unwrap_or_default();
                Some((k, v))
            })
            .collect()
    })
    .unwrap_or_default()
}

/// Collect tags from awsQuery params: Tags.Tag.N.Key / Tags.Tag.N.Value
fn collect_tags(params: &HashMap<String, String>) -> Vec<(String, String)> {
    let mut tags = Vec::new();
    let mut i = 1;
    loop {
        let key_param = format!("Tags.Tag.{i}.Key");
        let value_param = format!("Tags.Tag.{i}.Value");
        match (params.get(&key_param), params.get(&value_param)) {
            (Some(k), Some(v)) => {
                tags.push((k.clone(), v.clone()));
                i += 1;
            }
            _ => break,
        }
    }
    tags
}
