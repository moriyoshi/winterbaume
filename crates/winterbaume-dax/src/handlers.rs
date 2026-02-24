use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    json_error_response,
};

use crate::state::{DaxError, DaxState};
use crate::views::DaxStateView;
use crate::wire;

pub struct DaxService {
    pub(crate) state: Arc<BackendState<DaxState>>,
    pub(crate) notifier: StateChangeNotifier<DaxStateView>,
}

impl DaxService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for DaxService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for DaxService {
    fn service_name(&self) -> &str {
        "dax"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://dax\..*\.amazonaws\.com",
            r"https?://dax\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl DaxService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

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

        use winterbaume_core::StatefulService;
        let response = match action.as_str() {
            "CreateCluster" => {
                self.handle_create_cluster(&state, body_bytes, &region, account_id)
                    .await
            }
            "DescribeClusters" => self.handle_describe_clusters(&state, body_bytes).await,
            "DeleteCluster" => self.handle_delete_cluster(&state, body_bytes).await,
            // --- Unimplemented operations (auto-generated stubs) ---
            "CreateParameterGroup" => json_error_response(
                501,
                "NotImplementedError",
                "CreateParameterGroup is not yet implemented in winterbaume-dax",
            ),
            "CreateSubnetGroup" => json_error_response(
                501,
                "NotImplementedError",
                "CreateSubnetGroup is not yet implemented in winterbaume-dax",
            ),
            "DecreaseReplicationFactor" => {
                self.handle_decrease_replication_factor(&state, body_bytes)
                    .await
            }
            "DeleteParameterGroup" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteParameterGroup is not yet implemented in winterbaume-dax",
            ),
            "DeleteSubnetGroup" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteSubnetGroup is not yet implemented in winterbaume-dax",
            ),
            "DescribeDefaultParameters" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeDefaultParameters is not yet implemented in winterbaume-dax",
            ),
            "DescribeEvents" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeEvents is not yet implemented in winterbaume-dax",
            ),
            "DescribeParameterGroups" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeParameterGroups is not yet implemented in winterbaume-dax",
            ),
            "DescribeParameters" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeParameters is not yet implemented in winterbaume-dax",
            ),
            "DescribeSubnetGroups" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeSubnetGroups is not yet implemented in winterbaume-dax",
            ),
            "IncreaseReplicationFactor" => {
                self.handle_increase_replication_factor(&state, body_bytes)
                    .await
            }
            "ListTags" => self.handle_list_tags(&state, body_bytes).await,
            "RebootNode" => json_error_response(
                501,
                "NotImplementedError",
                "RebootNode is not yet implemented in winterbaume-dax",
            ),
            "TagResource" => json_error_response(
                501,
                "NotImplementedError",
                "TagResource is not yet implemented in winterbaume-dax",
            ),
            "UntagResource" => json_error_response(
                501,
                "NotImplementedError",
                "UntagResource is not yet implemented in winterbaume-dax",
            ),
            "UpdateCluster" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateCluster is not yet implemented in winterbaume-dax",
            ),
            "UpdateParameterGroup" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateParameterGroup is not yet implemented in winterbaume-dax",
            ),
            "UpdateSubnetGroup" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateSubnetGroup is not yet implemented in winterbaume-dax",
            ),
            _ => json_error_response(400, "InvalidAction", &format!("Unknown operation {action}")),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<DaxState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        // Parse the raw body separately so we can reproduce previous defaults
        // for fields the typed deserialiser fills with `Default::default()`
        // (e.g. `ReplicationFactor` defaulted to 3, `NodeType` to dax.r5.large).
        let raw: Value = match serde_json::from_slice(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(400, "SerializationException", &e.to_string());
            }
        };
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
        let cluster_name = input.cluster_name.as_str();
        let node_type = if raw.get("NodeType").is_some() && !input.node_type.is_empty() {
            input.node_type.as_str()
        } else {
            "dax.r5.large"
        };
        let replication_factor = if raw.get("ReplicationFactor").is_some() {
            input.replication_factor
        } else {
            3
        };
        let iam_role_arn = input.iam_role_arn.as_str();
        let description = input.description.as_deref().unwrap_or("");
        let sse_enabled = input
            .s_s_e_specification
            .as_ref()
            .map(|s| s.enabled)
            .unwrap_or(false);
        let cluster_endpoint_encryption_type = input
            .cluster_endpoint_encryption_type
            .clone()
            .unwrap_or_else(|| "NONE".to_string());
        let tags: Vec<crate::types::DaxTag> = input
            .tags
            .as_deref()
            .map(|arr| {
                arr.iter()
                    .filter_map(|t| {
                        let key = t.key.as_ref()?.clone();
                        let value = t.value.as_ref()?.clone();
                        Some(crate::types::DaxTag { key, value })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.create_cluster(
            cluster_name,
            node_type,
            replication_factor,
            iam_role_arn,
            description,
            sse_enabled,
            cluster_endpoint_encryption_type,
            tags,
            region,
            account_id,
        ) {
            Ok(cluster) => wire::serialize_create_cluster_response(&wire::CreateClusterResponse {
                cluster: Some(cluster_to_wire(cluster)),
            }),
            Err(e) => dax_error_response(&e),
        }
    }

    /// Single-page response — pagination not implemented; full result set
    /// returned in one call. The `MaxResults` and `NextToken` request
    /// parameters from the AWS contract are intentionally ignored, and
    /// `NextToken` is always returned as absent in the response.
    async fn handle_describe_clusters(
        &self,
        state: &Arc<tokio::sync::RwLock<DaxState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_clusters_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterValueException", &e),
        };
        let cluster_names: Option<Vec<String>> = input.cluster_names.clone();

        let state = state.read().await;
        match state.describe_clusters(cluster_names.as_deref()) {
            Ok(clusters) => {
                wire::serialize_describe_clusters_response(&wire::DescribeClustersResponse {
                    clusters: Some(clusters.iter().map(|c| cluster_to_wire(c)).collect()),
                    next_token: None,
                })
            }
            Err(e) => dax_error_response(&e),
        }
    }

    async fn handle_delete_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<DaxState>>,
        body: &[u8],
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
        let cluster_name = input.cluster_name.as_str();

        let mut state = state.write().await;
        match state.delete_cluster(cluster_name) {
            Ok(cluster) => wire::serialize_delete_cluster_response(&wire::DeleteClusterResponse {
                cluster: Some(cluster_to_wire(&cluster)),
            }),
            Err(e) => dax_error_response(&e),
        }
    }

    async fn handle_decrease_replication_factor(
        &self,
        state: &Arc<tokio::sync::RwLock<DaxState>>,
        body: &[u8],
    ) -> MockResponse {
        let raw: Value = match serde_json::from_slice(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(400, "SerializationException", &e.to_string());
            }
        };
        let input = match wire::deserialize_decrease_replication_factor_request(body) {
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
        let cluster_name = input.cluster_name.as_str();
        // The original handler treated absence of NewReplicationFactor as an
        // error. Detect it by looking at the raw JSON since the typed struct
        // defaults to 0.
        if raw.get("NewReplicationFactor").is_none() {
            return json_error_response(
                400,
                "InvalidParameterValueException",
                "NewReplicationFactor is required",
            );
        }
        let new_replication_factor = input.new_replication_factor;

        let mut state = state.write().await;
        match state.clusters.get_mut(cluster_name) {
            Some(cluster) => {
                if new_replication_factor >= cluster.replication_factor {
                    return dax_error_response(&DaxError::InvalidParameterValue {
                        message:
                            "NewReplicationFactor must be less than current replication factor"
                                .to_string(),
                    });
                }
                if new_replication_factor < 1 {
                    return dax_error_response(&DaxError::InvalidParameterValue {
                        message: "NewReplicationFactor must be at least 1".to_string(),
                    });
                }
                cluster.replication_factor = new_replication_factor;
                wire::serialize_decrease_replication_factor_response(
                    &wire::DecreaseReplicationFactorResponse {
                        cluster: Some(cluster_to_wire(cluster)),
                    },
                )
            }
            None => dax_error_response(&DaxError::ClusterNotFound {
                message: format!("Cluster {cluster_name} not found."),
            }),
        }
    }

    async fn handle_increase_replication_factor(
        &self,
        state: &Arc<tokio::sync::RwLock<DaxState>>,
        body: &[u8],
    ) -> MockResponse {
        let raw: Value = match serde_json::from_slice(body) {
            Ok(v) => v,
            Err(e) => {
                return json_error_response(400, "SerializationException", &e.to_string());
            }
        };
        let input = match wire::deserialize_increase_replication_factor_request(body) {
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
        let cluster_name = input.cluster_name.as_str();
        if raw.get("NewReplicationFactor").is_none() {
            return json_error_response(
                400,
                "InvalidParameterValueException",
                "NewReplicationFactor is required",
            );
        }
        let new_replication_factor = input.new_replication_factor;

        let mut state = state.write().await;
        match state.clusters.get_mut(cluster_name) {
            Some(cluster) => {
                if new_replication_factor <= cluster.replication_factor {
                    return dax_error_response(&DaxError::InvalidParameterValue {
                        message:
                            "NewReplicationFactor must be greater than current replication factor"
                                .to_string(),
                    });
                }
                cluster.replication_factor = new_replication_factor;
                wire::serialize_increase_replication_factor_response(
                    &wire::IncreaseReplicationFactorResponse {
                        cluster: Some(cluster_to_wire(cluster)),
                    },
                )
            }
            None => dax_error_response(&DaxError::ClusterNotFound {
                message: format!("Cluster {cluster_name} not found."),
            }),
        }
    }

    /// Single-page response — pagination not implemented; full result set
    /// returned in one call. The `NextToken` request parameter is intentionally
    /// ignored, and `NextToken` is always returned as absent in the response.
    async fn handle_list_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<DaxState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterValueException", &e),
        };
        if input.resource_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterValueException",
                "ResourceName is required",
            );
        }
        let resource_name = input.resource_name.as_str();

        let state = state.read().await;
        // Find by ARN or by cluster name
        let cluster = state
            .clusters
            .values()
            .find(|c| c.cluster_arn == resource_name || c.cluster_name == resource_name);

        match cluster {
            Some(c) => wire::serialize_list_tags_response(&wire::ListTagsResponse {
                tags: Some(tags_to_wire(&c.tags)),
                next_token: None,
            }),
            None => dax_error_response(&DaxError::ClusterNotFound {
                message: format!("Resource {resource_name} not found."),
            }),
        }
    }
}

fn cluster_to_wire(cluster: &crate::types::DaxCluster) -> wire::Cluster {
    wire::Cluster {
        cluster_name: Some(cluster.cluster_name.clone()),
        cluster_arn: Some(cluster.cluster_arn.clone()),
        node_type: Some(cluster.node_type.clone()),
        status: Some(cluster.status.clone()),
        description: Some(cluster.description.clone()),
        iam_role_arn: Some(cluster.iam_role_arn.clone()),
        total_nodes: Some(cluster.replication_factor),
        active_nodes: Some(0),
        preferred_maintenance_window: Some("thu:23:30-fri:00:30".to_string()),
        subnet_group: Some("default".to_string()),
        security_groups: Some(vec![wire::SecurityGroupMembership {
            security_group_identifier: Some("sg-00000001".to_string()),
            status: Some("active".to_string()),
        }]),
        parameter_group: Some(wire::ParameterGroupStatus {
            parameter_group_name: Some("default.dax1.0".to_string()),
            parameter_apply_status: Some("in-sync".to_string()),
            node_ids_to_reboot: None,
        }),
        s_s_e_description: Some(wire::SSEDescription {
            status: Some(if cluster.sse_enabled {
                "ENABLED".to_string()
            } else {
                "DISABLED".to_string()
            }),
        }),
        cluster_endpoint_encryption_type: Some(cluster.cluster_endpoint_encryption_type.clone()),
        cluster_discovery_endpoint: Some(wire::Endpoint {
            port: Some(8111),
            address: None,
            u_r_l: None,
        }),
        ..Default::default()
    }
}

fn tags_to_wire(tags: &[crate::types::DaxTag]) -> Vec<wire::Tag> {
    tags.iter()
        .map(|t| wire::Tag {
            key: Some(t.key.clone()),
            value: Some(t.value.clone()),
        })
        .collect()
}

fn dax_error_response(err: &DaxError) -> MockResponse {
    let (status, error_type) = match err {
        DaxError::InvalidClusterName => (400, "InvalidParameterValueException"),
        DaxError::InvalidArnMissingPrefix { .. } => (400, "InvalidParameterValueException"),
        DaxError::InvalidArnMissingPartition { .. } => (400, "InvalidParameterValueException"),
        DaxError::InvalidArnMissingVendor { .. } => (400, "InvalidParameterValueException"),
        DaxError::InvalidArnMissingRegionDelimiter { .. } => {
            (400, "InvalidParameterValueException")
        }
        DaxError::InvalidArnMissingNamespaceDelimiter { .. } => {
            (400, "InvalidParameterValueException")
        }
        DaxError::ClusterAlreadyExists { .. } => (400, "ClusterAlreadyExistsFault"),
        DaxError::ClusterNotFound { .. } => (400, "ClusterNotFoundFault"),
        DaxError::InvalidParameterValue { .. } => (400, "InvalidParameterValueException"),
    };
    MockResponse::json(
        status,
        json!({"__type": error_type, "message": err.to_string()}).to_string(),
    )
}
