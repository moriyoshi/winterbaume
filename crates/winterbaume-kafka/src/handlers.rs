use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::model;
use crate::state::{KafkaError, KafkaState};
use crate::types::*;
use crate::views::KafkaStateView;
use crate::wire;

const DEFAULT_KAFKA_VERSION: &str = "3.5.1";

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct KafkaService {
    pub(crate) state: Arc<BackendState<KafkaState>>,
    pub(crate) notifier: StateChangeNotifier<KafkaStateView>,
}

impl KafkaService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for KafkaService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for KafkaService {
    fn service_name(&self) -> &str {
        "kafka"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://kafka\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl KafkaService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let (path, query_string) = extract_path_and_query(&request.uri);
        let query_map: HashMap<String, String> =
            winterbaume_core::parse_query_string(&query_string);
        let method = request.method.as_str();

        tracing::debug!("MSK dispatch: {} {}", method, path);

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        // Handle /v1/tags/{resourceArn+} routes
        let response = if segments.len() >= 2 && segments[0] == "v1" && segments[1] == "tags" {
            let arn = percent_decode(&segments[2..].join("/"));
            match method {
                "GET" => self.handle_list_tags_for_resource(&state, &arn).await,
                "POST" => {
                    let labels: &[(&str, &str)] = &[("ResourceArn", arn.as_str())];
                    self.handle_tag_resource(&state, &request, labels, &query_map)
                        .await
                }
                "DELETE" => {
                    let tag_keys = extract_query_param_list(&query_string, "tagKeys");
                    self.handle_untag_resource(&state, &arn, &tag_keys).await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        } else if segments.len() >= 3
            && segments[0] == "api"
            && segments[1] == "v2"
            && segments[2] == "clusters"
        {
            // Handle /api/v2/clusters routes
            match (method, segments.len()) {
                // POST /api/v2/clusters - CreateClusterV2
                ("POST", 3) => {
                    self.handle_create_cluster_v2(
                        &state,
                        &request,
                        &[],
                        &query_map,
                        account_id,
                        &region,
                    )
                    .await
                }
                // GET /api/v2/clusters - ListClustersV2
                ("GET", 3) => self.handle_list_clusters_v2(&state).await,
                // GET /api/v2/clusters/{clusterArn+} - DescribeClusterV2
                ("GET", 4..) => {
                    let arn = percent_decode(&segments[3..].join("/"));
                    self.handle_describe_cluster_v2(&state, &arn).await
                }
                // DELETE /api/v2/clusters/{clusterArn+} - DeleteCluster (V2 path)
                ("DELETE", 4..) => {
                    let arn = percent_decode(&segments[3..].join("/"));
                    self.handle_delete_cluster(&state, &arn).await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        } else if segments.len() >= 2 && segments[0] == "v1" && segments[1] == "clusters" {
            // Handle /v1/clusters routes
            match (method, segments.len()) {
                // POST /v1/clusters - CreateCluster (V1)
                ("POST", 2) => {
                    self.handle_create_cluster_v1(
                        &state,
                        &request,
                        &[],
                        &query_map,
                        account_id,
                        &region,
                    )
                    .await
                }
                // GET /v1/clusters - ListClusters (V1)
                ("GET", 2) => self.handle_list_clusters_v1(&state).await,
                // GET /v1/clusters/{clusterArn+} - DescribeCluster (V1)
                ("GET", 3..) => {
                    let arn = percent_decode(&segments[2..].join("/"));
                    self.handle_describe_cluster_v1(&state, &arn).await
                }
                // DELETE /v1/clusters/{clusterArn+} - DeleteCluster
                ("DELETE", 3..) => {
                    let arn = percent_decode(&segments[2..].join("/"));
                    self.handle_delete_cluster(&state, &arn).await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        } else {
            rest_json_error(404, "UnknownOperationException", "Not found")
        };

        if matches!(method, "POST" | "PUT" | "DELETE") && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_cluster_v2(
        &self,
        state: &Arc<tokio::sync::RwLock<KafkaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_cluster_v2_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.cluster_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'clusterName'");
        }
        let cluster_name = input.cluster_name.as_str();

        let cluster_id = uuid::Uuid::new_v4().to_string();
        let cluster_arn =
            format!("arn:aws:kafka:{region}:{account_id}:cluster/{cluster_name}/{cluster_id}");

        let (cluster_type, provisioned, serverless) = if let Some(prov) = input.provisioned.as_ref()
        {
            let bng = &prov.broker_node_group_info;
            let instance_type = if bng.instance_type.is_empty() {
                "kafka.m5.large".to_string()
            } else {
                bng.instance_type.clone()
            };
            let kafka_ver = if prov.kafka_version.is_empty() {
                DEFAULT_KAFKA_VERSION.to_string()
            } else {
                prov.kafka_version.clone()
            };
            let num_brokers = if prov.number_of_broker_nodes == 0 {
                3
            } else {
                prov.number_of_broker_nodes
            };
            let client_subnets = bng.client_subnets.clone();
            let security_groups = bng.security_groups.clone().unwrap_or_default();
            (
                ClusterType::Provisioned,
                Some(ProvisionedClusterInfo {
                    kafka_version: kafka_ver,
                    number_of_broker_nodes: num_brokers,
                    instance_type,
                    client_subnets,
                    security_groups,
                }),
                None,
            )
        } else if let Some(sl) = input.serverless.as_ref() {
            let vpc_configs = sl
                .vpc_configs
                .iter()
                .map(|vc| VpcConfig {
                    subnet_ids: vc.subnet_ids.clone(),
                    security_group_ids: vc.security_group_ids.clone().unwrap_or_default(),
                })
                .collect();
            (
                ClusterType::Serverless,
                None,
                Some(ServerlessClusterInfo { vpc_configs }),
            )
        } else {
            return rest_json_error(
                400,
                "BadRequestException",
                "Either 'provisioned' or 'serverless' must be specified",
            );
        };

        let tags = input.tags.clone().unwrap_or_default();

        let mut state = state.write().await;
        match state.create_cluster(
            cluster_name,
            &cluster_arn,
            cluster_type,
            provisioned,
            serverless,
            tags,
        ) {
            Ok(cluster) => {
                let resp = model::CreateClusterV2Response {
                    cluster_arn: Some(cluster.cluster_arn.clone()),
                    cluster_name: Some(cluster.cluster_name.clone()),
                    state: Some(cluster.state.as_str().to_string()),
                    cluster_type: Some(cluster.cluster_type.as_str().to_string()),
                };
                wire::serialize_create_cluster_v2_response(&resp)
            }
            Err(e) => kafka_error_response(&e),
        }
    }

    async fn handle_describe_cluster_v2(
        &self,
        state: &Arc<tokio::sync::RwLock<KafkaState>>,
        cluster_arn: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_cluster(cluster_arn) {
            Ok(cluster) => {
                let cluster_info = cluster_to_model(cluster);
                let resp = model::DescribeClusterV2Response {
                    cluster_info: Some(cluster_info),
                };
                wire::serialize_describe_cluster_v2_response(&resp)
            }
            Err(e) => kafka_error_response(&e),
        }
    }

    async fn handle_delete_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<KafkaState>>,
        cluster_arn: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_cluster(cluster_arn) {
            Ok(cluster) => {
                let resp = model::DeleteClusterResponse {
                    cluster_arn: Some(cluster.cluster_arn.clone()),
                    state: Some("DELETING".to_string()),
                };
                wire::serialize_delete_cluster_response(&resp)
            }
            Err(e) => kafka_error_response(&e),
        }
    }

    async fn handle_list_clusters_v2(
        &self,
        state: &Arc<tokio::sync::RwLock<KafkaState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let clusters = state.list_clusters();
        let cluster_infos: Vec<model::Cluster> =
            clusters.iter().map(|c| cluster_to_model(c)).collect();
        let resp = model::ListClustersV2Response {
            cluster_info_list: Some(cluster_infos),
            next_token: None,
        };
        wire::serialize_list_clusters_v2_response(&resp)
    }

    async fn handle_create_cluster_v1(
        &self,
        state: &Arc<tokio::sync::RwLock<KafkaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_cluster_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.cluster_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'clusterName'");
        }
        let cluster_name = input.cluster_name.as_str();

        let kafka_version = if input.kafka_version.is_empty() {
            DEFAULT_KAFKA_VERSION.to_string()
        } else {
            input.kafka_version.clone()
        };
        let num_brokers = if input.number_of_broker_nodes == 0 {
            3
        } else {
            input.number_of_broker_nodes
        };

        let bng = &input.broker_node_group_info;
        let instance_type = if bng.instance_type.is_empty() {
            "kafka.m5.large".to_string()
        } else {
            bng.instance_type.clone()
        };
        let client_subnets = bng.client_subnets.clone();
        let security_groups = bng.security_groups.clone().unwrap_or_default();

        let cluster_id = uuid::Uuid::new_v4().to_string();
        let cluster_arn =
            format!("arn:aws:kafka:{region}:{account_id}:cluster/{cluster_name}/{cluster_id}");

        let tags = input.tags.clone().unwrap_or_default();

        let provisioned = Some(ProvisionedClusterInfo {
            kafka_version,
            number_of_broker_nodes: num_brokers,
            instance_type,
            client_subnets,
            security_groups,
        });

        let mut state = state.write().await;
        match state.create_cluster(
            cluster_name,
            &cluster_arn,
            ClusterType::Provisioned,
            provisioned,
            None,
            tags,
        ) {
            Ok(cluster) => {
                let resp = model::CreateClusterResponse {
                    cluster_arn: Some(cluster.cluster_arn.clone()),
                    cluster_name: Some(cluster.cluster_name.clone()),
                    state: Some(cluster.state.as_str().to_string()),
                };
                wire::serialize_create_cluster_response(&resp)
            }
            Err(e) => kafka_error_response(&e),
        }
    }

    async fn handle_describe_cluster_v1(
        &self,
        state: &Arc<tokio::sync::RwLock<KafkaState>>,
        cluster_arn: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_cluster(cluster_arn) {
            Ok(cluster) => {
                let cluster_info = cluster_to_v1_model(cluster);
                let resp = model::DescribeClusterResponse {
                    cluster_info: Some(cluster_info),
                };
                wire::serialize_describe_cluster_response(&resp)
            }
            Err(e) => kafka_error_response(&e),
        }
    }

    async fn handle_list_clusters_v1(
        &self,
        state: &Arc<tokio::sync::RwLock<KafkaState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let clusters = state.list_clusters();
        let cluster_infos: Vec<model::ClusterInfo> =
            clusters.iter().map(|c| cluster_to_v1_model(c)).collect();
        let resp = model::ListClustersResponse {
            cluster_info_list: Some(cluster_infos),
            next_token: None,
        };
        wire::serialize_list_clusters_response(&resp)
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<KafkaState>>,
        arn: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let tags = state.list_tags_for_resource(arn);
        let resp = model::ListTagsForResourceResponse { tags: Some(tags) };
        wire::serialize_list_tags_for_resource_response(&resp)
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<KafkaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        state.tag_resource(&input.resource_arn, &input.tags);
        wire::serialize_tag_resource_response()
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<KafkaState>>,
        arn: &str,
        tag_keys: &[String],
    ) -> MockResponse {
        let mut state = state.write().await;
        state.untag_resource(arn, tag_keys);
        wire::serialize_untag_resource_response()
    }
}

/// Convert a state `Cluster` to a wire `model::Cluster` (V2) for serialization.
fn cluster_to_model(cluster: &Cluster) -> model::Cluster {
    let provisioned = cluster.provisioned.as_ref().map(|prov| model::Provisioned {
        broker_node_group_info: Some(model::BrokerNodeGroupInfo {
            instance_type: prov.instance_type.clone(),
            client_subnets: prov.client_subnets.clone(),
            security_groups: Some(prov.security_groups.clone()),
            ..Default::default()
        }),
        current_broker_software_info: Some(model::BrokerSoftwareInfo {
            kafka_version: Some(prov.kafka_version.clone()),
            ..Default::default()
        }),
        number_of_broker_nodes: Some(prov.number_of_broker_nodes),
        ..Default::default()
    });

    let serverless = cluster.serverless.as_ref().map(|sl| model::Serverless {
        vpc_configs: Some(
            sl.vpc_configs
                .iter()
                .map(|vc| model::VpcConfig {
                    subnet_ids: vc.subnet_ids.clone(),
                    security_group_ids: Some(vc.security_group_ids.clone()),
                })
                .collect(),
        ),
        ..Default::default()
    });

    model::Cluster {
        active_operation_arn: None,
        cluster_arn: Some(cluster.cluster_arn.clone()),
        cluster_name: Some(cluster.cluster_name.clone()),
        cluster_type: Some(cluster.cluster_type.as_str().to_string()),
        creation_time: None,
        current_version: Some("K1".to_string()),
        state: Some(cluster.state.as_str().to_string()),
        state_info: Some(model::StateInfo {
            code: None,
            message: None,
        }),
        tags: Some(cluster.tags.clone()),
        provisioned,
        serverless,
    }
}

/// Convert a state `Cluster` to a wire `model::ClusterInfo` (V1) for serialization.
fn cluster_to_v1_model(cluster: &Cluster) -> model::ClusterInfo {
    let mut info = model::ClusterInfo {
        cluster_arn: Some(cluster.cluster_arn.clone()),
        cluster_name: Some(cluster.cluster_name.clone()),
        state: Some(cluster.state.as_str().to_string()),
        tags: Some(cluster.tags.clone()),
        ..Default::default()
    };

    if let Some(ref prov) = cluster.provisioned {
        info.broker_node_group_info = Some(model::BrokerNodeGroupInfo {
            instance_type: prov.instance_type.clone(),
            client_subnets: prov.client_subnets.clone(),
            security_groups: Some(prov.security_groups.clone()),
            ..Default::default()
        });
        info.current_broker_software_info = Some(model::BrokerSoftwareInfo {
            kafka_version: Some(prov.kafka_version.clone()),
            ..Default::default()
        });
        info.number_of_broker_nodes = Some(prov.number_of_broker_nodes);
    }

    info
}

fn extract_path_and_query(uri: &str) -> (String, String) {
    let path_and_query = if let Some(idx) = uri.find("amazonaws.com") {
        &uri[idx + "amazonaws.com".len()..]
    } else {
        uri
    };
    match path_and_query.find('?') {
        Some(q) => (
            path_and_query[..q].to_string(),
            path_and_query[q + 1..].to_string(),
        ),
        None => (path_and_query.to_string(), String::new()),
    }
}

fn extract_query_param_list(query_string: &str, param_name: &str) -> Vec<String> {
    if query_string.is_empty() {
        return Vec::new();
    }
    query_string
        .split('&')
        .filter_map(|part| {
            let (key, value) = part.split_once('=')?;
            if key == param_name {
                Some(percent_decode(value))
            } else {
                None
            }
        })
        .collect()
}

fn percent_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut bytes = s.bytes();
    while let Some(b) = bytes.next() {
        match b {
            b'%' => {
                let hi = bytes.next().and_then(hex_val);
                let lo = bytes.next().and_then(hex_val);
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    result.push((hi << 4 | lo) as char);
                }
            }
            b'+' => result.push(' '),
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

fn kafka_error_response(err: &KafkaError) -> MockResponse {
    let (status, error_type) = match err {
        KafkaError::ClusterAlreadyExists { .. } => (409, "ConflictException"),
        KafkaError::ClusterNotFound { .. } => (404, "NotFoundException"),
    };
    let body = json!({
        "message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
