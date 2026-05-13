use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{TimestreamInfluxDbState, TsInfluxError};
use crate::types::{DbCluster, DbInstance, DbParameterGroup};
use crate::views::TimestreamInfluxDbStateView;
use crate::wire;

/// Timestream for InfluxDB service handler that processes awsJson1.0 protocol requests.
pub struct TimestreamInfluxDbService {
    pub(crate) state: Arc<BackendState<TimestreamInfluxDbState>>,
    pub(crate) notifier: StateChangeNotifier<TimestreamInfluxDbStateView>,
}

impl TimestreamInfluxDbService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for TimestreamInfluxDbService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for TimestreamInfluxDbService {
    fn service_name(&self) -> &str {
        "timestream-influxdb"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://timestream-influxdb\.(.+)\.amazonaws\.com",
            r"https?://timestream-influxdb\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl TimestreamInfluxDbService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        // Extract action from X-Amz-Target header
        // Format: "AmazonTimestreamInfluxDB.CreateDbInstance"
        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.rsplit('.').next())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        // Validate the body is well-formed JSON up-front; the typed deserialisers in
        // `wire` re-parse the bytes per operation.
        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "ValidationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreateDbInstance" => {
                self.handle_create_db_instance(&state, body_bytes, account_id, &region)
                    .await
            }
            "GetDbInstance" => self.handle_get_db_instance(&state, body_bytes).await,
            "DeleteDbInstance" => self.handle_delete_db_instance(&state, body_bytes).await,
            "ListDbInstances" => self.handle_list_db_instances(&state).await,
            "CreateDbCluster" => {
                self.handle_create_db_cluster(&state, body_bytes, account_id, &region)
                    .await
            }
            "GetDbCluster" => self.handle_get_db_cluster(&state, body_bytes).await,
            "DeleteDbCluster" => self.handle_delete_db_cluster(&state, body_bytes).await,
            "ListDbClusters" => self.handle_list_db_clusters(&state).await,
            "ListDbInstancesForCluster" => {
                self.handle_list_db_instances_for_cluster(&state, body_bytes)
                    .await
            }
            "RebootDbCluster" => self.handle_reboot_db_cluster(&state, body_bytes).await,
            "RebootDbInstance" => self.handle_reboot_db_instance(&state, body_bytes).await,
            "UpdateDbCluster" => self.handle_update_db_cluster(&state, body_bytes).await,
            "UpdateDbInstance" => self.handle_update_db_instance(&state, body_bytes).await,
            "CreateDbParameterGroup" => {
                self.handle_create_db_parameter_group(&state, body_bytes, account_id, &region)
                    .await
            }
            "GetDbParameterGroup" => self.handle_get_db_parameter_group(&state, body_bytes).await,
            "ListDbParameterGroups" => self.handle_list_db_parameter_groups(&state).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            _ => json_error_response(
                400,
                "ValidationException",
                &format!("Could not find operation {action} for TimestreamInfluxDB"),
            ),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_db_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamInfluxDbState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_db_instance_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'name'",
            );
        }
        if input.password.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'password'",
            );
        }
        if input.db_instance_type.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'dbInstanceType'",
            );
        }
        // The Smithy `allocatedStorage` member is required, but it deserialises to 0 by default.
        // Treat zero as "not provided" to match prior behaviour.
        if input.allocated_storage == 0 {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'allocatedStorage'",
            );
        }
        if input.vpc_subnet_ids.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'vpcSubnetIds'",
            );
        }
        if input.vpc_security_group_ids.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'vpcSecurityGroupIds'",
            );
        }

        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_db_instance(
            &input.name,
            &input.db_instance_type,
            input.allocated_storage,
            input.vpc_subnet_ids,
            input.vpc_security_group_ids,
            input.publicly_accessible,
            input.db_storage_type.as_deref(),
            input.db_parameter_group_identifier.as_deref(),
            input.deployment_type.as_deref(),
            input.port,
            tags,
            account_id,
            region,
        ) {
            Ok(instance) => {
                wire::serialize_create_db_instance_response(&db_instance_to_wire(instance))
            }
            Err(e) => ts_error_response(&e),
        }
    }

    async fn handle_get_db_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamInfluxDbState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_db_instance_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identifier.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'identifier'",
            );
        }

        let state = state.read().await;
        match state.get_db_instance(&input.identifier) {
            Ok(instance) => {
                let resp = wire::GetDbInstanceOutput {
                    id: Some(instance.id.clone()),
                    name: Some(instance.name.clone()),
                    arn: Some(instance.arn.clone()),
                    status: Some(instance.status.clone()),
                    endpoint: instance.endpoint.clone(),
                    port: instance.port,
                    db_instance_type: Some(instance.db_instance_type.clone()),
                    db_storage_type: instance.db_storage_type.clone(),
                    allocated_storage: Some(instance.allocated_storage),
                    deployment_type: instance.deployment_type.clone(),
                    vpc_subnet_ids: Some(instance.vpc_subnet_ids.clone()),
                    vpc_security_group_ids: Some(instance.vpc_security_group_ids.clone()),
                    publicly_accessible: instance.publicly_accessible,
                    db_parameter_group_identifier: instance.db_parameter_group_identifier.clone(),
                    availability_zone: instance.availability_zone.clone(),
                    ..Default::default()
                };
                wire::serialize_get_db_instance_response(&resp)
            }
            Err(e) => ts_error_response(&e),
        }
    }

    async fn handle_delete_db_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamInfluxDbState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_db_instance_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identifier.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'identifier'",
            );
        }

        let mut state = state.write().await;
        match state.delete_db_instance(&input.identifier) {
            Ok(ref instance) => {
                let resp = wire::DeleteDbInstanceOutput {
                    id: Some(instance.id.clone()),
                    name: Some(instance.name.clone()),
                    arn: Some(instance.arn.clone()),
                    status: Some(instance.status.clone()),
                    endpoint: instance.endpoint.clone(),
                    port: instance.port,
                    db_instance_type: Some(instance.db_instance_type.clone()),
                    db_storage_type: instance.db_storage_type.clone(),
                    allocated_storage: Some(instance.allocated_storage),
                    deployment_type: instance.deployment_type.clone(),
                    vpc_subnet_ids: Some(instance.vpc_subnet_ids.clone()),
                    vpc_security_group_ids: Some(instance.vpc_security_group_ids.clone()),
                    publicly_accessible: instance.publicly_accessible,
                    db_parameter_group_identifier: instance.db_parameter_group_identifier.clone(),
                    availability_zone: instance.availability_zone.clone(),
                    ..Default::default()
                };
                wire::serialize_delete_db_instance_response(&resp)
            }
            Err(e) => ts_error_response(&e),
        }
    }

    async fn handle_list_db_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamInfluxDbState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let summaries = state.list_db_instances();
        let items: Vec<wire::DbInstanceSummary> = summaries
            .iter()
            .map(|s| wire::DbInstanceSummary {
                id: Some(s.id.clone()),
                name: Some(s.name.clone()),
                arn: Some(s.arn.clone()),
                status: s.status.clone(),
                endpoint: s.endpoint.clone(),
                port: s.port,
                db_instance_type: s.db_instance_type.clone(),
                db_storage_type: s.db_storage_type.clone(),
                allocated_storage: s.allocated_storage,
                deployment_type: s.deployment_type.clone(),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_db_instances_response(&wire::ListDbInstancesOutput {
            items: Some(items),
            next_token: None,
        })
    }
    // --- DbCluster handlers ---

    async fn handle_create_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamInfluxDbState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_db_cluster_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'name'",
            );
        }
        if input.password.is_none() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'password'",
            );
        }

        let db_instance_type = if input.db_instance_type.is_empty() {
            None
        } else {
            Some(input.db_instance_type.as_str())
        };

        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_db_cluster(
            &input.name,
            db_instance_type,
            input.allocated_storage,
            input.vpc_subnet_ids,
            input.vpc_security_group_ids,
            input.publicly_accessible,
            input.db_storage_type.as_deref(),
            input.db_parameter_group_identifier.as_deref(),
            input.deployment_type.as_deref(),
            input.network_type.as_deref(),
            input.failover_mode.as_deref(),
            input.port,
            tags,
            account_id,
            region,
        ) {
            Ok(cluster) => {
                let resp = wire::CreateDbClusterOutput {
                    db_cluster_id: Some(cluster.id.clone()),
                    db_cluster_status: Some(cluster.status.clone()),
                    ..Default::default()
                };
                wire::serialize_create_db_cluster_response(&resp)
            }
            Err(e) => ts_error_response(&e),
        }
    }

    async fn handle_get_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamInfluxDbState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_db_cluster_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.db_cluster_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'dbClusterId'",
            );
        }

        let state = state.read().await;
        match state.get_db_cluster(&input.db_cluster_id) {
            Ok(cluster) => {
                wire::serialize_get_db_cluster_response(&db_cluster_to_get_wire(cluster))
            }
            Err(e) => ts_error_response(&e),
        }
    }

    async fn handle_list_db_clusters(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamInfluxDbState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let summaries = state.list_db_clusters();
        let items: Vec<wire::DbClusterSummary> = summaries
            .iter()
            .map(|s| wire::DbClusterSummary {
                id: Some(s.id.clone()),
                name: Some(s.name.clone()),
                arn: Some(s.arn.clone()),
                status: s.status.clone(),
                endpoint: s.endpoint.clone(),
                reader_endpoint: s.reader_endpoint.clone(),
                port: s.port,
                deployment_type: s.deployment_type.clone(),
                db_instance_type: s.db_instance_type.clone(),
                network_type: s.network_type.clone(),
                db_storage_type: s.db_storage_type.clone(),
                allocated_storage: s.allocated_storage,
                ..Default::default()
            })
            .collect();

        wire::serialize_list_db_clusters_response(&wire::ListDbClustersOutput {
            items: Some(items),
            next_token: None,
        })
    }

    async fn handle_delete_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamInfluxDbState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_db_cluster_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.db_cluster_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'dbClusterId'",
            );
        }

        let mut state = state.write().await;
        match state.delete_db_cluster(&input.db_cluster_id) {
            Ok(cluster) => {
                let resp = wire::DeleteDbClusterOutput {
                    db_cluster_status: Some(cluster.status.clone()),
                };
                wire::serialize_delete_db_cluster_response(&resp)
            }
            Err(e) => ts_error_response(&e),
        }
    }

    async fn handle_list_db_instances_for_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamInfluxDbState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_db_instances_for_cluster_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.db_cluster_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'dbClusterId'",
            );
        }

        let state = state.read().await;
        match state.list_db_instances_for_cluster(&input.db_cluster_id) {
            Ok(summaries) => {
                let items: Vec<wire::DbInstanceForClusterSummary> = summaries
                    .iter()
                    .map(|s| wire::DbInstanceForClusterSummary {
                        id: Some(s.id.clone()),
                        name: Some(s.name.clone()),
                        arn: Some(s.arn.clone()),
                        status: s.status.clone(),
                        endpoint: s.endpoint.clone(),
                        port: s.port,
                        db_instance_type: s.db_instance_type.clone(),
                        db_storage_type: s.db_storage_type.clone(),
                        allocated_storage: s.allocated_storage,
                        deployment_type: s.deployment_type.clone(),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_db_instances_for_cluster_response(
                    &wire::ListDbInstancesForClusterOutput {
                        items: Some(items),
                        next_token: None,
                    },
                )
            }
            Err(e) => ts_error_response(&e),
        }
    }

    async fn handle_reboot_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamInfluxDbState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_reboot_db_cluster_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.db_cluster_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'dbClusterId'",
            );
        }

        let mut state = state.write().await;
        match state.reboot_db_cluster(&input.db_cluster_id) {
            Ok(cluster) => {
                let resp = wire::RebootDbClusterOutput {
                    db_cluster_status: Some(cluster.status.clone()),
                };
                wire::serialize_reboot_db_cluster_response(&resp)
            }
            Err(e) => ts_error_response(&e),
        }
    }

    async fn handle_reboot_db_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamInfluxDbState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_reboot_db_instance_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identifier.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'identifier'",
            );
        }

        let mut state = state.write().await;
        match state.reboot_db_instance(&input.identifier) {
            Ok(instance) => {
                let resp = wire::RebootDbInstanceOutput {
                    id: Some(instance.id.clone()),
                    name: Some(instance.name.clone()),
                    arn: Some(instance.arn.clone()),
                    status: Some(instance.status.clone()),
                    endpoint: instance.endpoint.clone(),
                    port: instance.port,
                    db_instance_type: Some(instance.db_instance_type.clone()),
                    db_storage_type: instance.db_storage_type.clone(),
                    allocated_storage: Some(instance.allocated_storage),
                    deployment_type: instance.deployment_type.clone(),
                    vpc_subnet_ids: Some(instance.vpc_subnet_ids.clone()),
                    vpc_security_group_ids: Some(instance.vpc_security_group_ids.clone()),
                    publicly_accessible: instance.publicly_accessible,
                    db_parameter_group_identifier: instance.db_parameter_group_identifier.clone(),
                    availability_zone: instance.availability_zone.clone(),
                    ..Default::default()
                };
                wire::serialize_reboot_db_instance_response(&resp)
            }
            Err(e) => ts_error_response(&e),
        }
    }

    async fn handle_update_db_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamInfluxDbState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_db_cluster_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.db_cluster_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'dbClusterId'",
            );
        }

        let mut state = state.write().await;
        match state.update_db_cluster(
            &input.db_cluster_id,
            input.db_parameter_group_identifier.as_deref(),
            input.failover_mode.as_deref(),
            input.port,
        ) {
            Ok(cluster) => {
                let resp = wire::UpdateDbClusterOutput {
                    db_cluster_status: Some(cluster.status.clone()),
                };
                wire::serialize_update_db_cluster_response(&resp)
            }
            Err(e) => ts_error_response(&e),
        }
    }

    async fn handle_update_db_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamInfluxDbState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_db_instance_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identifier.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'identifier'",
            );
        }

        let mut state = state.write().await;
        match state.update_db_instance(
            &input.identifier,
            input.db_parameter_group_identifier.as_deref(),
            input.db_storage_type.as_deref(),
            input.allocated_storage,
            input.deployment_type.as_deref(),
            input.port,
        ) {
            Ok(instance) => {
                let resp = wire::UpdateDbInstanceOutput {
                    id: Some(instance.id.clone()),
                    name: Some(instance.name.clone()),
                    arn: Some(instance.arn.clone()),
                    status: Some(instance.status.clone()),
                    endpoint: instance.endpoint.clone(),
                    port: instance.port,
                    db_instance_type: Some(instance.db_instance_type.clone()),
                    db_storage_type: instance.db_storage_type.clone(),
                    allocated_storage: Some(instance.allocated_storage),
                    deployment_type: instance.deployment_type.clone(),
                    vpc_subnet_ids: Some(instance.vpc_subnet_ids.clone()),
                    vpc_security_group_ids: Some(instance.vpc_security_group_ids.clone()),
                    publicly_accessible: instance.publicly_accessible,
                    db_parameter_group_identifier: instance.db_parameter_group_identifier.clone(),
                    availability_zone: instance.availability_zone.clone(),
                    ..Default::default()
                };
                wire::serialize_update_db_instance_response(&resp)
            }
            Err(e) => ts_error_response(&e),
        }
    }

    // --- DbParameterGroup handlers ---

    async fn handle_create_db_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamInfluxDbState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_db_parameter_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'name'",
            );
        }

        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_db_parameter_group(
            &input.name,
            input.description.as_deref(),
            tags,
            account_id,
            region,
        ) {
            Ok(pg) => {
                let resp = db_parameter_group_to_create_wire(pg);
                wire::serialize_create_db_parameter_group_response(&resp)
            }
            Err(e) => ts_error_response(&e),
        }
    }

    async fn handle_get_db_parameter_group(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamInfluxDbState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_db_parameter_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identifier.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'identifier'",
            );
        }

        let state = state.read().await;
        match state.get_db_parameter_group(&input.identifier) {
            Ok(pg) => {
                let resp = wire::GetDbParameterGroupOutput {
                    id: Some(pg.id.clone()),
                    name: Some(pg.name.clone()),
                    arn: Some(pg.arn.clone()),
                    description: pg.description.clone(),
                    ..Default::default()
                };
                wire::serialize_get_db_parameter_group_response(&resp)
            }
            Err(e) => ts_error_response(&e),
        }
    }

    async fn handle_list_db_parameter_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamInfluxDbState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let summaries = state.list_db_parameter_groups();
        let items: Vec<wire::DbParameterGroupSummary> = summaries
            .iter()
            .map(|s| wire::DbParameterGroupSummary {
                id: Some(s.id.clone()),
                name: Some(s.name.clone()),
                arn: Some(s.arn.clone()),
                description: s.description.clone(),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_db_parameter_groups_response(&wire::ListDbParameterGroupsOutput {
            items: Some(items),
            next_token: None,
        })
    }

    // --- Tag handlers ---

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamInfluxDbState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'resourceArn'",
            );
        }

        let state = state.read().await;
        match state.find_resource_tags(&input.resource_arn) {
            Ok(tags) => {
                let resp = wire::ListTagsForResourceResponse {
                    tags: Some(tags.clone()),
                };
                wire::serialize_list_tags_for_resource_response(&resp)
            }
            Err(e) => ts_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamInfluxDbState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'resourceArn'",
            );
        }

        let mut state = state.write().await;
        match state.find_resource_tags_mut(&input.resource_arn) {
            Ok(tags) => {
                tags.extend(input.tags);
                wire::serialize_tag_resource_response()
            }
            Err(e) => ts_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamInfluxDbState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing required field 'resourceArn'",
            );
        }

        let mut state = state.write().await;
        match state.find_resource_tags_mut(&input.resource_arn) {
            Ok(tags) => {
                for key in &input.tag_keys {
                    tags.remove(key);
                }
                wire::serialize_untag_resource_response()
            }
            Err(e) => ts_error_response(&e),
        }
    }
}

// --- State-to-wire conversion helpers ---

fn db_instance_to_wire(instance: &DbInstance) -> wire::CreateDbInstanceOutput {
    wire::CreateDbInstanceOutput {
        id: Some(instance.id.clone()),
        name: Some(instance.name.clone()),
        arn: Some(instance.arn.clone()),
        status: Some(instance.status.clone()),
        endpoint: instance.endpoint.clone(),
        port: instance.port,
        db_instance_type: Some(instance.db_instance_type.clone()),
        db_storage_type: instance.db_storage_type.clone(),
        allocated_storage: Some(instance.allocated_storage),
        deployment_type: instance.deployment_type.clone(),
        vpc_subnet_ids: Some(instance.vpc_subnet_ids.clone()),
        vpc_security_group_ids: Some(instance.vpc_security_group_ids.clone()),
        publicly_accessible: instance.publicly_accessible,
        db_parameter_group_identifier: instance.db_parameter_group_identifier.clone(),
        availability_zone: instance.availability_zone.clone(),
        ..Default::default()
    }
}

fn db_cluster_to_get_wire(cluster: &DbCluster) -> wire::GetDbClusterOutput {
    wire::GetDbClusterOutput {
        id: Some(cluster.id.clone()),
        name: Some(cluster.name.clone()),
        arn: Some(cluster.arn.clone()),
        status: Some(cluster.status.clone()),
        endpoint: cluster.endpoint.clone(),
        reader_endpoint: cluster.reader_endpoint.clone(),
        port: cluster.port,
        deployment_type: cluster.deployment_type.clone(),
        db_instance_type: cluster.db_instance_type.clone(),
        network_type: cluster.network_type.clone(),
        db_storage_type: cluster.db_storage_type.clone(),
        allocated_storage: cluster.allocated_storage,
        publicly_accessible: cluster.publicly_accessible,
        db_parameter_group_identifier: cluster.db_parameter_group_identifier.clone(),
        vpc_subnet_ids: Some(cluster.vpc_subnet_ids.clone()),
        vpc_security_group_ids: Some(cluster.vpc_security_group_ids.clone()),
        failover_mode: cluster.failover_mode.clone(),
        ..Default::default()
    }
}

fn db_parameter_group_to_create_wire(pg: &DbParameterGroup) -> wire::CreateDbParameterGroupOutput {
    wire::CreateDbParameterGroupOutput {
        id: Some(pg.id.clone()),
        name: Some(pg.name.clone()),
        arn: Some(pg.arn.clone()),
        description: pg.description.clone(),
        ..Default::default()
    }
}

// --- Error utility functions ---

fn ts_error_response(err: &TsInfluxError) -> MockResponse {
    let (status, error_type) = match err {
        TsInfluxError::DbInstanceNameConflict { .. } => (409, "ConflictException"),
        TsInfluxError::DbClusterNameConflict { .. } => (409, "ConflictException"),
        TsInfluxError::DbParameterGroupNameConflict { .. } => (409, "ConflictException"),
        TsInfluxError::DbInstanceNotFound { .. } => (404, "ResourceNotFoundException"),
        TsInfluxError::DbClusterNotFound { .. } => (404, "ResourceNotFoundException"),
        TsInfluxError::DbParameterGroupNotFound { .. } => (404, "ResourceNotFoundException"),
        TsInfluxError::ResourceArnNotFound { .. } => (404, "ResourceNotFoundException"),
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
