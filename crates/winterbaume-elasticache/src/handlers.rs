use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::model;
use crate::state::{ElastiCacheError, ElastiCacheState};
use crate::types::Tag;
use crate::views::ElastiCacheStateView;
use crate::wire;

/// ElastiCache service handler that processes awsQuery protocol requests.
pub struct ElastiCacheService {
    pub(crate) state: Arc<BackendState<ElastiCacheState>>,
    pub(crate) notifier: StateChangeNotifier<ElastiCacheStateView>,
}

impl ElastiCacheService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ElastiCacheService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ElastiCacheService {
    fn service_name(&self) -> &str {
        "elasticache"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://elasticache\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

const MUTATING_ACTIONS: &[&str] = &[
    "AddTagsToResource",
    "CreateCacheCluster",
    "CreateCacheParameterGroup",
    "CreateCacheSecurityGroup",
    "CreateCacheSubnetGroup",
    "CreateReplicationGroup",
    "CreateSnapshot",
    "CreateUser",
    "DeleteCacheCluster",
    "DeleteCacheParameterGroup",
    "DeleteCacheSecurityGroup",
    "DeleteCacheSubnetGroup",
    "DeleteReplicationGroup",
    "DeleteSnapshot",
    "DeleteUser",
    "ModifyCacheCluster",
    "ModifyCacheParameterGroup",
    "ModifyCacheSubnetGroup",
    "ModifyReplicationGroup",
    "RemoveTagsFromResource",
];

impl ElastiCacheService {
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
            "AddTagsToResource" => {
                self.handle_add_tags_to_resource(&state, &params, account_id, &region)
                    .await
            }
            "CreateCacheCluster" => {
                self.handle_create_cache_cluster(&state, &params, account_id, &region)
                    .await
            }
            "CreateCacheParameterGroup" => {
                self.handle_create_cache_parameter_group(&state, &params, account_id, &region)
                    .await
            }
            "CreateCacheSecurityGroup" => {
                self.handle_create_cache_security_group(&state, &params, account_id, &region)
                    .await
            }
            "CreateCacheSubnetGroup" => {
                self.handle_create_cache_subnet_group(&state, &params, account_id, &region)
                    .await
            }
            "CreateReplicationGroup" => {
                self.handle_create_replication_group(&state, &params, account_id, &region)
                    .await
            }
            "CreateSnapshot" => {
                self.handle_create_snapshot(&state, &params, account_id, &region)
                    .await
            }
            "CreateUser" => {
                self.handle_create_user(&state, &params, account_id, &region)
                    .await
            }
            "DeleteCacheCluster" => self.handle_delete_cache_cluster(&state, &params).await,
            "DeleteCacheParameterGroup" => {
                self.handle_delete_cache_parameter_group(&state, &params)
                    .await
            }
            "DeleteCacheSecurityGroup" => {
                self.handle_delete_cache_security_group(&state, &params)
                    .await
            }
            "DeleteCacheSubnetGroup" => {
                self.handle_delete_cache_subnet_group(&state, &params).await
            }
            "DeleteReplicationGroup" => self.handle_delete_replication_group(&state, &params).await,
            "DeleteSnapshot" => self.handle_delete_snapshot(&state, &params).await,
            "DeleteUser" => self.handle_delete_user(&state, &params).await,
            "DescribeCacheClusters" => self.handle_describe_cache_clusters(&state, &params).await,
            "DescribeCacheParameterGroups" => {
                self.handle_describe_cache_parameter_groups(&state, &params)
                    .await
            }
            "DescribeCacheSecurityGroups" => {
                self.handle_describe_cache_security_groups(&state, &params)
                    .await
            }
            "DescribeCacheSubnetGroups" => {
                self.handle_describe_cache_subnet_groups(&state, &params)
                    .await
            }
            "DescribeReplicationGroups" => {
                self.handle_describe_replication_groups(&state, &params)
                    .await
            }
            "DescribeSnapshots" => self.handle_describe_snapshots(&state, &params).await,
            "DescribeUsers" => self.handle_describe_users(&state, &params).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, &params).await,
            "RemoveTagsFromResource" => {
                self.handle_remove_tags_from_resource(&state, &params).await
            }
            // --- Unimplemented operations (auto-generated stubs) ---
            "AuthorizeCacheSecurityGroupIngress" => MockResponse::error(
                501,
                "NotImplementedError",
                "AuthorizeCacheSecurityGroupIngress is not yet implemented in winterbaume-elasticache",
            ),
            "BatchApplyUpdateAction" => MockResponse::error(
                501,
                "NotImplementedError",
                "BatchApplyUpdateAction is not yet implemented in winterbaume-elasticache",
            ),
            "BatchStopUpdateAction" => MockResponse::error(
                501,
                "NotImplementedError",
                "BatchStopUpdateAction is not yet implemented in winterbaume-elasticache",
            ),
            "CompleteMigration" => MockResponse::error(
                501,
                "NotImplementedError",
                "CompleteMigration is not yet implemented in winterbaume-elasticache",
            ),
            "CopyServerlessCacheSnapshot" => MockResponse::error(
                501,
                "NotImplementedError",
                "CopyServerlessCacheSnapshot is not yet implemented in winterbaume-elasticache",
            ),
            "CopySnapshot" => MockResponse::error(
                501,
                "NotImplementedError",
                "CopySnapshot is not yet implemented in winterbaume-elasticache",
            ),
            "CreateGlobalReplicationGroup" => MockResponse::error(
                501,
                "NotImplementedError",
                "CreateGlobalReplicationGroup is not yet implemented in winterbaume-elasticache",
            ),
            "CreateServerlessCache" => MockResponse::error(
                501,
                "NotImplementedError",
                "CreateServerlessCache is not yet implemented in winterbaume-elasticache",
            ),
            "CreateServerlessCacheSnapshot" => MockResponse::error(
                501,
                "NotImplementedError",
                "CreateServerlessCacheSnapshot is not yet implemented in winterbaume-elasticache",
            ),
            "CreateUserGroup" => MockResponse::error(
                501,
                "NotImplementedError",
                "CreateUserGroup is not yet implemented in winterbaume-elasticache",
            ),
            "DecreaseNodeGroupsInGlobalReplicationGroup" => MockResponse::error(
                501,
                "NotImplementedError",
                "DecreaseNodeGroupsInGlobalReplicationGroup is not yet implemented in winterbaume-elasticache",
            ),
            "DecreaseReplicaCount" => MockResponse::error(
                501,
                "NotImplementedError",
                "DecreaseReplicaCount is not yet implemented in winterbaume-elasticache",
            ),
            "DeleteGlobalReplicationGroup" => MockResponse::error(
                501,
                "NotImplementedError",
                "DeleteGlobalReplicationGroup is not yet implemented in winterbaume-elasticache",
            ),
            "DeleteServerlessCache" => MockResponse::error(
                501,
                "NotImplementedError",
                "DeleteServerlessCache is not yet implemented in winterbaume-elasticache",
            ),
            "DeleteServerlessCacheSnapshot" => MockResponse::error(
                501,
                "NotImplementedError",
                "DeleteServerlessCacheSnapshot is not yet implemented in winterbaume-elasticache",
            ),
            "DeleteUserGroup" => MockResponse::error(
                501,
                "NotImplementedError",
                "DeleteUserGroup is not yet implemented in winterbaume-elasticache",
            ),
            "DescribeCacheEngineVersions" => MockResponse::error(
                501,
                "NotImplementedError",
                "DescribeCacheEngineVersions is not yet implemented in winterbaume-elasticache",
            ),
            "DescribeCacheParameters" => MockResponse::error(
                501,
                "NotImplementedError",
                "DescribeCacheParameters is not yet implemented in winterbaume-elasticache",
            ),
            "DescribeEngineDefaultParameters" => MockResponse::error(
                501,
                "NotImplementedError",
                "DescribeEngineDefaultParameters is not yet implemented in winterbaume-elasticache",
            ),
            "DescribeEvents" => MockResponse::error(
                501,
                "NotImplementedError",
                "DescribeEvents is not yet implemented in winterbaume-elasticache",
            ),
            "DescribeGlobalReplicationGroups" => MockResponse::error(
                501,
                "NotImplementedError",
                "DescribeGlobalReplicationGroups is not yet implemented in winterbaume-elasticache",
            ),
            "DescribeReservedCacheNodes" => MockResponse::error(
                501,
                "NotImplementedError",
                "DescribeReservedCacheNodes is not yet implemented in winterbaume-elasticache",
            ),
            "DescribeReservedCacheNodesOfferings" => MockResponse::error(
                501,
                "NotImplementedError",
                "DescribeReservedCacheNodesOfferings is not yet implemented in winterbaume-elasticache",
            ),
            "DescribeServerlessCacheSnapshots" => MockResponse::error(
                501,
                "NotImplementedError",
                "DescribeServerlessCacheSnapshots is not yet implemented in winterbaume-elasticache",
            ),
            "DescribeServerlessCaches" => MockResponse::error(
                501,
                "NotImplementedError",
                "DescribeServerlessCaches is not yet implemented in winterbaume-elasticache",
            ),
            "DescribeServiceUpdates" => MockResponse::error(
                501,
                "NotImplementedError",
                "DescribeServiceUpdates is not yet implemented in winterbaume-elasticache",
            ),
            "DescribeUpdateActions" => MockResponse::error(
                501,
                "NotImplementedError",
                "DescribeUpdateActions is not yet implemented in winterbaume-elasticache",
            ),
            "DescribeUserGroups" => MockResponse::error(
                501,
                "NotImplementedError",
                "DescribeUserGroups is not yet implemented in winterbaume-elasticache",
            ),
            "DisassociateGlobalReplicationGroup" => MockResponse::error(
                501,
                "NotImplementedError",
                "DisassociateGlobalReplicationGroup is not yet implemented in winterbaume-elasticache",
            ),
            "ExportServerlessCacheSnapshot" => MockResponse::error(
                501,
                "NotImplementedError",
                "ExportServerlessCacheSnapshot is not yet implemented in winterbaume-elasticache",
            ),
            "FailoverGlobalReplicationGroup" => MockResponse::error(
                501,
                "NotImplementedError",
                "FailoverGlobalReplicationGroup is not yet implemented in winterbaume-elasticache",
            ),
            "IncreaseNodeGroupsInGlobalReplicationGroup" => MockResponse::error(
                501,
                "NotImplementedError",
                "IncreaseNodeGroupsInGlobalReplicationGroup is not yet implemented in winterbaume-elasticache",
            ),
            "IncreaseReplicaCount" => MockResponse::error(
                501,
                "NotImplementedError",
                "IncreaseReplicaCount is not yet implemented in winterbaume-elasticache",
            ),
            "ListAllowedNodeTypeModifications" => MockResponse::error(
                501,
                "NotImplementedError",
                "ListAllowedNodeTypeModifications is not yet implemented in winterbaume-elasticache",
            ),
            "ModifyCacheCluster" => MockResponse::error(
                501,
                "NotImplementedError",
                "ModifyCacheCluster is not yet implemented in winterbaume-elasticache",
            ),
            "ModifyCacheParameterGroup" => MockResponse::error(
                501,
                "NotImplementedError",
                "ModifyCacheParameterGroup is not yet implemented in winterbaume-elasticache",
            ),
            "ModifyCacheSubnetGroup" => MockResponse::error(
                501,
                "NotImplementedError",
                "ModifyCacheSubnetGroup is not yet implemented in winterbaume-elasticache",
            ),
            "ModifyGlobalReplicationGroup" => MockResponse::error(
                501,
                "NotImplementedError",
                "ModifyGlobalReplicationGroup is not yet implemented in winterbaume-elasticache",
            ),
            "ModifyReplicationGroup" => MockResponse::error(
                501,
                "NotImplementedError",
                "ModifyReplicationGroup is not yet implemented in winterbaume-elasticache",
            ),
            "ModifyReplicationGroupShardConfiguration" => MockResponse::error(
                501,
                "NotImplementedError",
                "ModifyReplicationGroupShardConfiguration is not yet implemented in winterbaume-elasticache",
            ),
            "ModifyServerlessCache" => MockResponse::error(
                501,
                "NotImplementedError",
                "ModifyServerlessCache is not yet implemented in winterbaume-elasticache",
            ),
            "ModifyUser" => MockResponse::error(
                501,
                "NotImplementedError",
                "ModifyUser is not yet implemented in winterbaume-elasticache",
            ),
            "ModifyUserGroup" => MockResponse::error(
                501,
                "NotImplementedError",
                "ModifyUserGroup is not yet implemented in winterbaume-elasticache",
            ),
            "PurchaseReservedCacheNodesOffering" => MockResponse::error(
                501,
                "NotImplementedError",
                "PurchaseReservedCacheNodesOffering is not yet implemented in winterbaume-elasticache",
            ),
            "RebalanceSlotsInGlobalReplicationGroup" => MockResponse::error(
                501,
                "NotImplementedError",
                "RebalanceSlotsInGlobalReplicationGroup is not yet implemented in winterbaume-elasticache",
            ),
            "RebootCacheCluster" => MockResponse::error(
                501,
                "NotImplementedError",
                "RebootCacheCluster is not yet implemented in winterbaume-elasticache",
            ),
            "ResetCacheParameterGroup" => MockResponse::error(
                501,
                "NotImplementedError",
                "ResetCacheParameterGroup is not yet implemented in winterbaume-elasticache",
            ),
            "RevokeCacheSecurityGroupIngress" => MockResponse::error(
                501,
                "NotImplementedError",
                "RevokeCacheSecurityGroupIngress is not yet implemented in winterbaume-elasticache",
            ),
            "StartMigration" => MockResponse::error(
                501,
                "NotImplementedError",
                "StartMigration is not yet implemented in winterbaume-elasticache",
            ),
            "TestFailover" => MockResponse::error(
                501,
                "NotImplementedError",
                "TestFailover is not yet implemented in winterbaume-elasticache",
            ),
            "TestMigration" => MockResponse::error(
                501,
                "NotImplementedError",
                "TestMigration is not yet implemented in winterbaume-elasticache",
            ),
            _ => MockResponse::error(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for ElastiCache"),
            ),
        };

        if MUTATING_ACTIONS.contains(&action.as_str()) && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }

        response
    }

    // ---- AddTagsToResource --------------------------------------------------

    async fn handle_add_tags_to_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
        _account_id: &str,
        _region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_add_tags_to_resource_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.resource_name.is_empty() {
            return MockResponse::error(400, "InvalidParameter", "Missing 'ResourceName'");
        }
        let resource_name = input.resource_name;

        let tags = wire_tags_to_domain(Some(input.tags));

        let mut state = state.write().await;
        match state.add_tags(&resource_name, tags) {
            Ok(tag_list) => {
                let msg = wire::TagListMessage {
                    tag_list: Some(tags_to_wire(tag_list)),
                };
                let inner = quick_xml::se::to_string(&msg).unwrap_or_default();
                serialize_with_fixed_element(
                    &inner,
                    "RemoveTagsFromResourceResult",
                    "AddTagsToResourceResult",
                    "AddTagsToResourceResponse",
                )
            }
            Err(e) => elasticache_error_response(&e),
        }
    }

    // ---- CreateCacheCluster -------------------------------------------------

    async fn handle_create_cache_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_cache_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cache_cluster_id.is_empty() {
            return MockResponse::error(400, "InvalidParameter", "Missing 'CacheClusterId'");
        }
        let cluster_id = input.cache_cluster_id;
        let engine = input.engine.unwrap_or_else(|| "redis".to_string());
        let engine_version = input.engine_version.unwrap_or_else(|| "7.0.12".to_string());
        let cache_node_type = input
            .cache_node_type
            .unwrap_or_else(|| "cache.t3.micro".to_string());
        let num_cache_nodes: i32 = input.num_cache_nodes.unwrap_or(1);
        let cache_subnet_group_name = input.cache_subnet_group_name;
        let replication_group_id = input.replication_group_id;
        let preferred_az = input
            .preferred_availability_zone
            .unwrap_or_else(|| format!("{region}a"));
        let tags = wire_tags_to_domain(input.tags);

        let mut state = state.write().await;
        match state.create_cache_cluster(
            cluster_id,
            engine,
            engine_version,
            cache_node_type,
            num_cache_nodes,
            cache_subnet_group_name,
            replication_group_id,
            preferred_az,
            tags,
            account_id,
            region,
        ) {
            Ok(cluster) => {
                wire::serialize_create_cache_cluster_response(&wire::CreateCacheClusterResult {
                    cache_cluster: Some(cache_cluster_to_wire(cluster)),
                })
            }
            Err(e) => elasticache_error_response(&e),
        }
    }

    // ---- DeleteCacheCluster -------------------------------------------------

    async fn handle_delete_cache_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_cache_cluster_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cache_cluster_id.is_empty() {
            return MockResponse::error(400, "InvalidParameter", "Missing 'CacheClusterId'");
        }
        let mut state = state.write().await;
        match state.delete_cache_cluster(&input.cache_cluster_id) {
            Ok(cluster) => {
                wire::serialize_delete_cache_cluster_response(&wire::DeleteCacheClusterResult {
                    cache_cluster: Some(cache_cluster_to_wire(&cluster)),
                })
            }
            Err(e) => elasticache_error_response(&e),
        }
    }

    // ---- DescribeCacheClusters ----------------------------------------------

    async fn handle_describe_cache_clusters(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_cache_clusters_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let state = state.read().await;
        match state.describe_cache_clusters(input.cache_cluster_id.as_deref()) {
            Ok(clusters) => {
                wire::serialize_describe_cache_clusters_response(&wire::CacheClusterMessage {
                    cache_clusters: Some(clusters.into_iter().map(cache_cluster_to_wire).collect()),
                    marker: None,
                })
            }
            Err(e) => elasticache_error_response(&e),
        }
    }

    // ---- CreateReplicationGroup ---------------------------------------------

    async fn handle_create_replication_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_replication_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.replication_group_id.is_empty() {
            return MockResponse::error(400, "InvalidParameter", "Missing 'ReplicationGroupId'");
        }
        let rg_id = input.replication_group_id;
        let description = input.replication_group_description;
        let engine = input.engine.unwrap_or_else(|| "redis".to_string());
        let cache_node_type = input
            .cache_node_type
            .unwrap_or_else(|| "cache.t3.micro".to_string());
        let num_cache_clusters: i32 = input.num_cache_clusters.unwrap_or(1);
        let automatic_failover = input
            .automatic_failover_enabled
            .map(|b| if b { "enabled" } else { "disabled" })
            .unwrap_or("disabled")
            .to_string();
        let multi_az = input
            .multi_a_z_enabled
            .map(|b| if b { "enabled" } else { "disabled" })
            .unwrap_or("disabled")
            .to_string();
        let tags = wire_tags_to_domain(input.tags);

        let mut state = state.write().await;
        match state.create_replication_group(
            rg_id,
            description,
            engine,
            cache_node_type,
            num_cache_clusters,
            automatic_failover,
            multi_az,
            tags,
            account_id,
            region,
        ) {
            Ok(rg) => wire::serialize_create_replication_group_response(
                &wire::CreateReplicationGroupResult {
                    replication_group: Some(replication_group_to_wire(rg)),
                },
            ),
            Err(e) => elasticache_error_response(&e),
        }
    }

    // ---- DeleteReplicationGroup ---------------------------------------------

    async fn handle_delete_replication_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_replication_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.replication_group_id.is_empty() {
            return MockResponse::error(400, "InvalidParameter", "Missing 'ReplicationGroupId'");
        }
        let mut state = state.write().await;
        match state.delete_replication_group(&input.replication_group_id) {
            Ok(rg) => wire::serialize_delete_replication_group_response(
                &wire::DeleteReplicationGroupResult {
                    replication_group: Some(replication_group_to_wire(&rg)),
                },
            ),
            Err(e) => elasticache_error_response(&e),
        }
    }

    // ---- DescribeReplicationGroups ------------------------------------------

    async fn handle_describe_replication_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_replication_groups_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let state = state.read().await;
        match state.describe_replication_groups(input.replication_group_id.as_deref()) {
            Ok(groups) => wire::serialize_describe_replication_groups_response(
                &wire::ReplicationGroupMessage {
                    replication_groups: Some(
                        groups.into_iter().map(replication_group_to_wire).collect(),
                    ),
                    marker: None,
                },
            ),
            Err(e) => elasticache_error_response(&e),
        }
    }

    // ---- CreateCacheSubnetGroup ---------------------------------------------

    async fn handle_create_cache_subnet_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_cache_subnet_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cache_subnet_group_name.is_empty() {
            return MockResponse::error(400, "InvalidParameter", "Missing 'CacheSubnetGroupName'");
        }
        let name = input.cache_subnet_group_name;
        let description = input.cache_subnet_group_description;
        let subnet_ids = input.subnet_ids.items;
        let tags = wire_tags_to_domain(input.tags);

        let mut state = state.write().await;
        match state.create_cache_subnet_group(
            name,
            description,
            subnet_ids,
            tags,
            account_id,
            region,
        ) {
            Ok(sg) => wire::serialize_create_cache_subnet_group_response(
                &wire::CreateCacheSubnetGroupResult {
                    cache_subnet_group: Some(cache_subnet_group_to_wire(sg)),
                },
            ),
            Err(e) => elasticache_error_response(&e),
        }
    }

    // ---- DeleteCacheSubnetGroup ---------------------------------------------

    async fn handle_delete_cache_subnet_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_cache_subnet_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cache_subnet_group_name.is_empty() {
            return MockResponse::error(400, "InvalidParameter", "Missing 'CacheSubnetGroupName'");
        }
        let mut state = state.write().await;
        match state.delete_cache_subnet_group(&input.cache_subnet_group_name) {
            Ok(()) => wire::serialize_delete_cache_subnet_group_response(),
            Err(e) => elasticache_error_response(&e),
        }
    }

    // ---- DescribeCacheSubnetGroups ------------------------------------------

    async fn handle_describe_cache_subnet_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_cache_subnet_groups_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let state = state.read().await;
        match state.describe_cache_subnet_groups(input.cache_subnet_group_name.as_deref()) {
            Ok(groups) => wire::serialize_describe_cache_subnet_groups_response(
                &wire::CacheSubnetGroupMessage {
                    cache_subnet_groups: Some(
                        groups.into_iter().map(cache_subnet_group_to_wire).collect(),
                    ),
                    marker: None,
                },
            ),
            Err(e) => elasticache_error_response(&e),
        }
    }

    // ---- CreateCacheParameterGroup ------------------------------------------

    async fn handle_create_cache_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_cache_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cache_parameter_group_name.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameter",
                "Missing 'CacheParameterGroupName'",
            );
        }
        if input.cache_parameter_group_family.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameter",
                "Missing 'CacheParameterGroupFamily'",
            );
        }
        let name = input.cache_parameter_group_name;
        let family = input.cache_parameter_group_family;
        let description = input.description;
        let tags = wire_tags_to_domain(input.tags);

        let mut state = state.write().await;
        match state.create_cache_parameter_group(
            name,
            family,
            description,
            tags,
            account_id,
            region,
        ) {
            Ok(pg) => wire::serialize_create_cache_parameter_group_response(
                &wire::CreateCacheParameterGroupResult {
                    cache_parameter_group: Some(cache_parameter_group_to_wire(pg)),
                },
            ),
            Err(e) => elasticache_error_response(&e),
        }
    }

    // ---- DeleteCacheParameterGroup ------------------------------------------

    async fn handle_delete_cache_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_cache_parameter_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cache_parameter_group_name.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameter",
                "Missing 'CacheParameterGroupName'",
            );
        }
        let mut state = state.write().await;
        match state.delete_cache_parameter_group(&input.cache_parameter_group_name) {
            Ok(()) => wire::serialize_delete_cache_parameter_group_response(),
            Err(e) => elasticache_error_response(&e),
        }
    }

    // ---- DescribeCacheParameterGroups ---------------------------------------

    async fn handle_describe_cache_parameter_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_cache_parameter_groups_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let state = state.read().await;
        match state.describe_cache_parameter_groups(input.cache_parameter_group_name.as_deref()) {
            Ok(groups) => wire::serialize_describe_cache_parameter_groups_response(
                &wire::CacheParameterGroupsMessage {
                    cache_parameter_groups: Some(
                        groups
                            .into_iter()
                            .map(cache_parameter_group_to_wire)
                            .collect(),
                    ),
                    marker: None,
                },
            ),
            Err(e) => elasticache_error_response(&e),
        }
    }

    // ---- CreateCacheSecurityGroup -------------------------------------------

    async fn handle_create_cache_security_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_cache_security_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cache_security_group_name.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameter",
                "Missing 'CacheSecurityGroupName'",
            );
        }
        if input.description.is_empty() {
            return MockResponse::error(400, "InvalidParameter", "Missing 'Description'");
        }
        let name = input.cache_security_group_name;
        let description = input.description;
        let tags = wire_tags_to_domain(input.tags);

        let mut state = state.write().await;
        match state.create_cache_security_group(name, description, tags, account_id, region) {
            Ok(sg) => wire::serialize_create_cache_security_group_response(
                &wire::CreateCacheSecurityGroupResult {
                    cache_security_group: Some(cache_security_group_to_wire(sg, account_id)),
                },
            ),
            Err(e) => elasticache_error_response(&e),
        }
    }

    // ---- DeleteCacheSecurityGroup -------------------------------------------

    async fn handle_delete_cache_security_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_cache_security_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.cache_security_group_name.is_empty() {
            return MockResponse::error(
                400,
                "InvalidParameter",
                "Missing 'CacheSecurityGroupName'",
            );
        }
        let mut state = state.write().await;
        match state.delete_cache_security_group(&input.cache_security_group_name) {
            Ok(()) => wire::serialize_delete_cache_security_group_response(),
            Err(e) => elasticache_error_response(&e),
        }
    }

    // ---- DescribeCacheSecurityGroups ----------------------------------------

    async fn handle_describe_cache_security_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_cache_security_groups_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let state = state.read().await;
        match state.describe_cache_security_groups(input.cache_security_group_name.as_deref()) {
            Ok(groups) => {
                // Use a fixed account id for wire conversion
                let account_id = default_account_id();
                wire::serialize_describe_cache_security_groups_response(
                    &wire::CacheSecurityGroupMessage {
                        cache_security_groups: Some(
                            groups
                                .into_iter()
                                .map(|sg| cache_security_group_to_wire(sg, account_id))
                                .collect(),
                        ),
                        marker: None,
                    },
                )
            }
            Err(e) => elasticache_error_response(&e),
        }
    }

    // ---- CreateSnapshot -----------------------------------------------------

    async fn handle_create_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_snapshot_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.snapshot_name.is_empty() {
            return MockResponse::error(400, "InvalidParameter", "Missing 'SnapshotName'");
        }
        let snapshot_name = input.snapshot_name;
        let cache_cluster_id = input.cache_cluster_id;
        let replication_group_id = input.replication_group_id;
        let tags = wire_tags_to_domain(input.tags);

        let mut state = state.write().await;
        match state.create_snapshot(
            snapshot_name,
            cache_cluster_id,
            replication_group_id,
            tags,
            account_id,
            region,
        ) {
            Ok(snap) => wire::serialize_create_snapshot_response(&wire::CreateSnapshotResult {
                snapshot: Some(snapshot_to_wire(snap)),
            }),
            Err(e) => elasticache_error_response(&e),
        }
    }

    // ---- DeleteSnapshot -----------------------------------------------------

    async fn handle_delete_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_snapshot_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.snapshot_name.is_empty() {
            return MockResponse::error(400, "InvalidParameter", "Missing 'SnapshotName'");
        }
        let mut state = state.write().await;
        match state.delete_snapshot(&input.snapshot_name) {
            Ok(snap) => wire::serialize_delete_snapshot_response(&wire::DeleteSnapshotResult {
                snapshot: Some(snapshot_to_wire(&snap)),
            }),
            Err(e) => elasticache_error_response(&e),
        }
    }

    // ---- DescribeSnapshots --------------------------------------------------

    async fn handle_describe_snapshots(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_snapshots_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let state = state.read().await;
        match state.describe_snapshots(input.snapshot_name.as_deref()) {
            Ok(snaps) => {
                wire::serialize_describe_snapshots_response(&wire::DescribeSnapshotsListMessage {
                    snapshots: Some(snaps.into_iter().map(snapshot_to_wire).collect()),
                    marker: None,
                })
            }
            Err(e) => elasticache_error_response(&e),
        }
    }

    // ---- CreateUser ---------------------------------------------------------

    async fn handle_create_user(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_user_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_id.is_empty() {
            return MockResponse::error(400, "InvalidParameter", "Missing 'UserId'");
        }
        if input.user_name.is_empty() {
            return MockResponse::error(400, "InvalidParameter", "Missing 'UserName'");
        }
        if input.engine.is_empty() {
            return MockResponse::error(400, "InvalidParameter", "Missing 'Engine'");
        }
        let user_id = input.user_id;
        let user_name = input.user_name;
        let engine = input.engine;
        let access_string = if input.access_string.is_empty() {
            "off".to_string()
        } else {
            input.access_string
        };
        let tags = wire_tags_to_domain(input.tags);

        let mut state = state.write().await;
        match state.create_user(
            user_id,
            user_name,
            engine,
            access_string,
            tags,
            account_id,
            region,
        ) {
            Ok(user) => {
                let user_wire = user_to_wire(user);
                let inner = quick_xml::se::to_string(&user_wire).unwrap_or_default();
                serialize_with_fixed_element(
                    &inner,
                    "ModifyUserResult",
                    "CreateUserResult",
                    "CreateUserResponse",
                )
            }
            Err(e) => elasticache_error_response(&e),
        }
    }

    // ---- DeleteUser ---------------------------------------------------------

    async fn handle_delete_user(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_user_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.user_id.is_empty() {
            return MockResponse::error(400, "InvalidParameter", "Missing 'UserId'");
        }
        let mut state = state.write().await;
        match state.delete_user(&input.user_id) {
            Ok(user) => {
                let user_wire = user_to_wire(&user);
                let inner = quick_xml::se::to_string(&user_wire).unwrap_or_default();
                serialize_with_fixed_element(
                    &inner,
                    "ModifyUserResult",
                    "DeleteUserResult",
                    "DeleteUserResponse",
                )
            }
            Err(e) => elasticache_error_response(&e),
        }
    }

    // ---- DescribeUsers ------------------------------------------------------

    async fn handle_describe_users(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_users_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let state = state.read().await;
        match state.describe_users(input.user_id.as_deref()) {
            Ok(users) => wire::serialize_describe_users_response(&wire::DescribeUsersResult {
                users: Some(users.into_iter().map(user_to_wire).collect()),
                marker: None,
            }),
            Err(e) => elasticache_error_response(&e),
        }
    }

    // ---- ListTagsForResource ------------------------------------------------

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.resource_name.is_empty() {
            return MockResponse::error(400, "InvalidParameter", "Missing 'ResourceName'");
        }
        let state = state.read().await;
        match state.list_tags(&input.resource_name) {
            Ok(tags) => {
                let msg = wire::TagListMessage {
                    tag_list: Some(tags_to_wire(tags)),
                };
                let inner = quick_xml::se::to_string(&msg).unwrap_or_default();
                serialize_with_fixed_element(
                    &inner,
                    "RemoveTagsFromResourceResult",
                    "ListTagsForResourceResult",
                    "ListTagsForResourceResponse",
                )
            }
            Err(e) => elasticache_error_response(&e),
        }
    }

    // ---- RemoveTagsFromResource ---------------------------------------------

    async fn handle_remove_tags_from_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ElastiCacheState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_remove_tags_from_resource_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.resource_name.is_empty() {
            return MockResponse::error(400, "InvalidParameter", "Missing 'ResourceName'");
        }

        let tag_keys = input.tag_keys.items;

        let mut state = state.write().await;
        match state.remove_tags(&input.resource_name, &tag_keys) {
            Ok(tags) => wire::serialize_remove_tags_from_resource_response(&wire::TagListMessage {
                tag_list: Some(tags_to_wire(tags)),
            }),
            Err(e) => elasticache_error_response(&e),
        }
    }
}

// ---------------------------------------------------------------------------
// Wire conversion helpers
// ---------------------------------------------------------------------------

fn cache_cluster_to_wire(c: &crate::types::CacheCluster) -> model::CacheCluster {
    model::CacheCluster {
        cache_cluster_id: Some(c.cache_cluster_id.clone()),
        cache_cluster_status: Some(c.status.clone()),
        engine: Some(c.engine.clone()),
        engine_version: Some(c.engine_version.clone()),
        cache_node_type: Some(c.cache_node_type.clone()),
        num_cache_nodes: Some(c.num_cache_nodes),
        preferred_availability_zone: Some(c.preferred_availability_zone.clone()),
        cache_subnet_group_name: c.cache_subnet_group_name.clone(),
        replication_group_id: c.replication_group_id.clone(),
        a_r_n: Some(c.arn.clone()),
        ..Default::default()
    }
}

fn replication_group_to_wire(r: &crate::types::ReplicationGroup) -> model::ReplicationGroup {
    model::ReplicationGroup {
        replication_group_id: Some(r.replication_group_id.clone()),
        description: Some(r.description.clone()),
        status: Some(r.status.clone()),
        cache_node_type: Some(r.cache_node_type.clone()),
        engine: Some(r.engine.clone()),
        automatic_failover: Some(r.automatic_failover.clone()),
        multi_a_z: Some(r.multi_az.clone()),
        a_r_n: Some(r.arn.clone()),
        member_clusters: if r.member_clusters.is_empty() {
            None
        } else {
            Some(r.member_clusters.iter().cloned().collect())
        },
        ..Default::default()
    }
}

fn cache_subnet_group_to_wire(s: &crate::types::CacheSubnetGroup) -> model::CacheSubnetGroup {
    model::CacheSubnetGroup {
        cache_subnet_group_name: Some(s.name.clone()),
        cache_subnet_group_description: Some(s.description.clone()),
        vpc_id: Some(s.vpc_id.clone()),
        a_r_n: Some(s.arn.clone()),
        subnets: Some(
            s.subnet_ids
                .iter()
                .map(|id| model::Subnet {
                    subnet_identifier: Some(id.clone()),
                    ..Default::default()
                })
                .collect(),
        ),
        ..Default::default()
    }
}

fn cache_parameter_group_to_wire(
    p: &crate::types::CacheParameterGroup,
) -> model::CacheParameterGroup {
    model::CacheParameterGroup {
        cache_parameter_group_name: Some(p.name.clone()),
        cache_parameter_group_family: Some(p.family.clone()),
        description: Some(p.description.clone()),
        a_r_n: Some(p.arn.clone()),
        ..Default::default()
    }
}

fn cache_security_group_to_wire(
    s: &crate::types::CacheSecurityGroup,
    account_id: &str,
) -> model::CacheSecurityGroup {
    model::CacheSecurityGroup {
        cache_security_group_name: Some(s.name.clone()),
        description: Some(s.description.clone()),
        owner_id: Some(account_id.to_string()),
        a_r_n: Some(s.arn.clone()),
        ..Default::default()
    }
}

fn snapshot_to_wire(s: &crate::types::Snapshot) -> model::Snapshot {
    model::Snapshot {
        snapshot_name: Some(s.snapshot_name.clone()),
        cache_cluster_id: s.cache_cluster_id.clone(),
        replication_group_id: s.replication_group_id.clone(),
        snapshot_status: Some(s.status.clone()),
        engine: Some(s.engine.clone()),
        engine_version: Some(s.engine_version.clone()),
        cache_node_type: Some(s.cache_node_type.clone()),
        cache_subnet_group_name: s.cache_subnet_group_name.clone(),
        a_r_n: Some(s.arn.clone()),
        ..Default::default()
    }
}

fn user_to_wire(u: &crate::types::User) -> model::User {
    model::User {
        user_id: Some(u.user_id.clone()),
        user_name: Some(u.user_name.clone()),
        engine: Some(u.engine.clone()),
        status: Some(u.status.clone()),
        access_string: Some(u.access_string.clone()),
        a_r_n: Some(u.arn.clone()),
        ..Default::default()
    }
}

/// Build an XML response where the inner result element name differs from the model's
/// #[serde(rename = "...")] annotation. Replaces the first occurrence of `wrong_name`
/// with `correct_name` in the serialized XML.
fn serialize_with_fixed_element(
    inner_xml: &str,
    wrong_name: &str,
    correct_name: &str,
    outer_tag: &str,
) -> MockResponse {
    let fixed = inner_xml
        .replacen(&format!("<{wrong_name}>"), &format!("<{correct_name}>"), 1)
        .replacen(
            &format!("</{wrong_name}>"),
            &format!("</{correct_name}>"),
            1,
        );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<{outer_tag} xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  {fixed}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</{outer_tag}>"#
    );
    MockResponse::xml(200, xml)
}

fn tags_to_wire(tags: Vec<Tag>) -> model::TagList {
    tags.into_iter()
        .map(|t| model::Tag {
            key: Some(t.key),
            value: Some(t.value),
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Utility functions
// ---------------------------------------------------------------------------

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

fn elasticache_error_response(e: &ElastiCacheError) -> MockResponse {
    let (status, code) = match e {
        ElastiCacheError::CacheClusterAlreadyExists(_) => (400, "CacheClusterAlreadyExists"),
        ElastiCacheError::CacheClusterNotFound(_) => (404, "CacheClusterNotFound"),
        ElastiCacheError::ReplicationGroupAlreadyExists(_) => {
            (400, "ReplicationGroupAlreadyExists")
        }
        ElastiCacheError::ReplicationGroupNotFound(_) => (404, "ReplicationGroupNotFound"),
        ElastiCacheError::CacheSubnetGroupAlreadyExists(_) => {
            (400, "CacheSubnetGroupAlreadyExists")
        }
        ElastiCacheError::CacheSubnetGroupNotFound(_) => (400, "CacheSubnetGroupNotFound"),
        ElastiCacheError::CacheParameterGroupAlreadyExists(_) => {
            (400, "CacheParameterGroupAlreadyExists")
        }
        ElastiCacheError::CacheParameterGroupNotFound(_) => (400, "CacheParameterGroupNotFound"),
        ElastiCacheError::CacheSecurityGroupAlreadyExists(_) => {
            (400, "CacheSecurityGroupAlreadyExists")
        }
        ElastiCacheError::CacheSecurityGroupNotFound(_) => (400, "CacheSecurityGroupNotFound"),
        ElastiCacheError::SnapshotAlreadyExistsFault(_) => (400, "SnapshotAlreadyExistsFault"),
        ElastiCacheError::SnapshotNotFound(_) => (404, "SnapshotNotFound"),
        ElastiCacheError::UserAlreadyExists(_) => (400, "UserAlreadyExists"),
        ElastiCacheError::UserNotFound(_) => (404, "UserNotFound"),
        ElastiCacheError::InvalidARN(_) => (400, "InvalidARN"),
    };
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ErrorResponse xmlns="http://elasticache.amazonaws.com/doc/2015-02-02/">
  <Error>
    <Type>Sender</Type>
    <Code>{code}</Code>
    <Message>{message}</Message>
  </Error>
  <RequestId>{request_id}</RequestId>
</ErrorResponse>"#,
        message = xml_escape(&e.to_string()),
    );
    MockResponse::xml(status, xml)
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Convert wire tag list to domain tags.
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
