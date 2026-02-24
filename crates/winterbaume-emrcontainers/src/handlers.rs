use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json, to_value};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::state::{EmrContainersError, EmrContainersState};
use crate::types::{ContainerInfo, ContainerProvider, EksInfo};
use crate::views::EmrContainersStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct EmrContainersService {
    pub(crate) state: Arc<BackendState<EmrContainersState>>,
    pub(crate) notifier: StateChangeNotifier<EmrContainersStateView>,
}

impl EmrContainersService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for EmrContainersService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for EmrContainersService {
    fn service_name(&self) -> &str {
        "emr-containers"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://emr-containers\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl EmrContainersService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        // Extract query string for tag operations.
        let raw_query = if let Some(idx) = request.uri.find('?') {
            request.uri[idx + 1..].to_string()
        } else {
            String::new()
        };
        let query_map: HashMap<String, String> = parse_single_query(&raw_query);
        let multi_tag_keys: Vec<String> = parse_multi_tag_keys(&raw_query);

        // Routes from Smithy model:
        // POST   /virtualclusters           -> CreateVirtualCluster
        // GET    /virtualclusters           -> ListVirtualClusters
        // GET    /virtualclusters/{id}      -> DescribeVirtualCluster
        // DELETE /virtualclusters/{id}      -> DeleteVirtualCluster

        use winterbaume_core::StatefulService;
        let response = match (method, segments.as_slice()) {
            ("POST", ["virtualclusters"]) => {
                self.handle_create_virtual_cluster(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            ("GET", ["virtualclusters"]) => {
                self.handle_list_virtual_clusters(&state, &request, &[], &query_map)
                    .await
            }
            ("GET", ["virtualclusters", id]) => {
                let id = percent_decode(id);
                let labels: &[(&str, &str)] = &[("id", id.as_str())];
                self.handle_describe_virtual_cluster(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["virtualclusters", id]) => {
                let id = percent_decode(id);
                let labels: &[(&str, &str)] = &[("id", id.as_str())];
                self.handle_delete_virtual_cluster(&state, &request, labels, &query_map)
                    .await
            }
            // POST /virtualclusters/{id}/jobruns - StartJobRun
            ("POST", ["virtualclusters", vc_id, "jobruns"]) => {
                let vc_id = percent_decode(vc_id);
                let labels: &[(&str, &str)] = &[("virtualClusterId", vc_id.as_str())];
                self.handle_start_job_run(&state, &request, labels, &query_map, account_id, &region)
                    .await
            }
            // GET /virtualclusters/{id}/jobruns - ListJobRuns
            ("GET", ["virtualclusters", vc_id, "jobruns"]) => {
                let vc_id = percent_decode(vc_id);
                let labels: &[(&str, &str)] = &[("virtualClusterId", vc_id.as_str())];
                self.handle_list_job_runs(&state, &request, labels, &query_map)
                    .await
            }
            // GET /virtualclusters/{id}/jobruns/{jobRunId} - DescribeJobRun
            ("GET", ["virtualclusters", vc_id, "jobruns", jr_id]) => {
                let vc_id = percent_decode(vc_id);
                let jr_id = percent_decode(jr_id);
                let labels: &[(&str, &str)] =
                    &[("virtualClusterId", vc_id.as_str()), ("id", jr_id.as_str())];
                self.handle_describe_job_run(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /virtualclusters/{id}/jobruns/{jobRunId} - CancelJobRun
            ("DELETE", ["virtualclusters", vc_id, "jobruns", jr_id]) => {
                let vc_id = percent_decode(vc_id);
                let jr_id = percent_decode(jr_id);
                let labels: &[(&str, &str)] =
                    &[("virtualClusterId", vc_id.as_str()), ("id", jr_id.as_str())];
                self.handle_cancel_job_run(&state, &request, labels, &query_map)
                    .await
            }
            // POST /virtualclusters/{virtualClusterId}/endpoints - CreateManagedEndpoint
            ("POST", ["virtualclusters", vc_id, "endpoints"]) => {
                let vc_id = percent_decode(vc_id);
                let labels: &[(&str, &str)] = &[("virtualClusterId", vc_id.as_str())];
                self.handle_create_managed_endpoint(
                    &state, &request, labels, &query_map, account_id, &region,
                )
                .await
            }
            // GET /virtualclusters/{virtualClusterId}/endpoints - ListManagedEndpoints
            ("GET", ["virtualclusters", vc_id, "endpoints"]) => {
                let vc_id = percent_decode(vc_id);
                let labels: &[(&str, &str)] = &[("virtualClusterId", vc_id.as_str())];
                self.handle_list_managed_endpoints(&state, &request, labels, &query_map)
                    .await
            }
            // GET /virtualclusters/{virtualClusterId}/endpoints/{id} - DescribeManagedEndpoint
            ("GET", ["virtualclusters", vc_id, "endpoints", ep_id]) => {
                let vc_id = percent_decode(vc_id);
                let ep_id = percent_decode(ep_id);
                let labels: &[(&str, &str)] =
                    &[("virtualClusterId", vc_id.as_str()), ("id", ep_id.as_str())];
                self.handle_describe_managed_endpoint(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /virtualclusters/{virtualClusterId}/endpoints/{id} - DeleteManagedEndpoint
            ("DELETE", ["virtualclusters", vc_id, "endpoints", ep_id]) => {
                let vc_id = percent_decode(vc_id);
                let ep_id = percent_decode(ep_id);
                let labels: &[(&str, &str)] =
                    &[("virtualClusterId", vc_id.as_str()), ("id", ep_id.as_str())];
                self.handle_delete_managed_endpoint(&state, &request, labels, &query_map)
                    .await
            }
            // POST /virtualclusters/{virtualClusterIdentifier}/endpoints/{endpointIdentifier}/credentials
            ("POST", ["virtualclusters", vc_id, "endpoints", ep_id, "credentials"]) => {
                let vc_id = percent_decode(vc_id);
                let ep_id = percent_decode(ep_id);
                let labels: &[(&str, &str)] = &[
                    ("virtualClusterIdentifier", vc_id.as_str()),
                    ("endpointIdentifier", ep_id.as_str()),
                ];
                self.handle_get_managed_endpoint_session_credentials(
                    &state, &request, labels, &query_map,
                )
                .await
            }
            // POST /jobtemplates - CreateJobTemplate
            ("POST", ["jobtemplates"]) => {
                self.handle_create_job_template(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            // GET /jobtemplates - ListJobTemplates
            ("GET", ["jobtemplates"]) => {
                self.handle_list_job_templates(&state, &request, &[], &query_map)
                    .await
            }
            // GET /jobtemplates/{id} - DescribeJobTemplate
            ("GET", ["jobtemplates", id]) => {
                let id = percent_decode(id);
                let labels: &[(&str, &str)] = &[("id", id.as_str())];
                self.handle_describe_job_template(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /jobtemplates/{id} - DeleteJobTemplate
            ("DELETE", ["jobtemplates", id]) => {
                let id = percent_decode(id);
                let labels: &[(&str, &str)] = &[("id", id.as_str())];
                self.handle_delete_job_template(&state, &request, labels, &query_map)
                    .await
            }
            // POST /securityconfigurations - CreateSecurityConfiguration
            ("POST", ["securityconfigurations"]) => {
                self.handle_create_security_configuration(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            // GET /securityconfigurations - ListSecurityConfigurations
            ("GET", ["securityconfigurations"]) => {
                self.handle_list_security_configurations(&state, &request, &[], &query_map)
                    .await
            }
            // GET /securityconfigurations/{id} - DescribeSecurityConfiguration
            ("GET", ["securityconfigurations", id]) => {
                let id = percent_decode(id);
                let labels: &[(&str, &str)] = &[("id", id.as_str())];
                self.handle_describe_security_configuration(&state, &request, labels, &query_map)
                    .await
            }
            // GET /tags/{resourceArn} - ListTagsForResource
            ("GET", ["tags", resource_arn]) => {
                let resource_arn = percent_decode(resource_arn);
                let labels: &[(&str, &str)] = &[("resourceArn", resource_arn.as_str())];
                self.handle_list_tags_for_resource(&state, &request, labels, &query_map)
                    .await
            }
            // POST /tags/{resourceArn} - TagResource
            ("POST", ["tags", resource_arn]) => {
                let resource_arn = percent_decode(resource_arn);
                let labels: &[(&str, &str)] = &[("resourceArn", resource_arn.as_str())];
                self.handle_tag_resource(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /tags/{resourceArn} - UntagResource
            ("DELETE", ["tags", resource_arn]) => {
                let resource_arn = percent_decode(resource_arn);
                self.handle_untag_resource(&state, &resource_arn, &multi_tag_keys)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_create_virtual_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrContainersState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return rest_json_error(400, "ValidationException", "Invalid JSON body");
        }
        let input = match wire::deserialize_create_virtual_cluster_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'name'");
        }
        // ContainerProvider is required; the wire deserializer fills defaults so
        // we detect omission via the underlying body.
        if !body_has_field(&request.body, "containerProvider") {
            return rest_json_error(400, "ValidationException", "Missing 'containerProvider'");
        }
        if input.container_provider.id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'containerProvider.id'");
        }
        let provider_type = if input.container_provider.r#type.is_empty() {
            "EKS".to_string()
        } else {
            input.container_provider.r#type.clone()
        };
        let info = input.container_provider.info.as_ref().map(|info| {
            let eks_info = info.eks_info.as_ref().map(|ei| EksInfo {
                namespace: ei.namespace.clone(),
            });
            ContainerInfo { eks_info }
        });
        let container_provider = ContainerProvider {
            provider_type,
            id: input.container_provider.id.clone(),
            info,
        };

        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_virtual_cluster(
            &input.name,
            container_provider,
            tags,
            account_id,
            region,
        ) {
            Ok(cluster) => {
                let resp = wire::CreateVirtualClusterResponse {
                    id: Some(cluster.id.clone()),
                    name: Some(cluster.name.clone()),
                    arn: Some(cluster.arn.clone()),
                };
                wire::serialize_create_virtual_cluster_response(&resp)
            }
            Err(e) => emr_containers_error_response(&e),
        }
    }

    async fn handle_describe_virtual_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrContainersState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_virtual_cluster_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.describe_virtual_cluster(&input.id) {
            Ok(cluster) => {
                let resp = wire::DescribeVirtualClusterResponse {
                    virtual_cluster: Some(virtual_cluster_to_wire(cluster)),
                };
                wire::serialize_describe_virtual_cluster_response(&resp)
            }
            Err(e) => emr_containers_error_response(&e),
        }
    }

    async fn handle_delete_virtual_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrContainersState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_virtual_cluster_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_virtual_cluster(&input.id) {
            Ok(cluster) => {
                let resp = wire::DeleteVirtualClusterResponse {
                    id: Some(cluster.id.clone()),
                };
                wire::serialize_delete_virtual_cluster_response(&resp)
            }
            Err(e) => emr_containers_error_response(&e),
        }
    }

    async fn handle_list_virtual_clusters(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrContainersState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_list_virtual_clusters_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let clusters = state.list_virtual_clusters();
        let entries: Vec<wire::VirtualCluster> = clusters
            .iter()
            .map(|c| virtual_cluster_to_wire(c))
            .collect();
        let resp = wire::ListVirtualClustersResponse {
            virtual_clusters: Some(entries),
            next_token: None,
        };
        wire::serialize_list_virtual_clusters_response(&resp)
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_start_job_run(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrContainersState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return rest_json_error(400, "ValidationException", "Invalid JSON body");
        }
        let input = match wire::deserialize_start_job_run_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = input.name.as_deref();
        let execution_role_arn = input.execution_role_arn.as_deref().unwrap_or("");
        let release_label = input.release_label.as_deref().unwrap_or("emr-6.5.0-latest");

        let mut state = state.write().await;
        match state.start_job_run(
            &input.virtual_cluster_id,
            name,
            execution_role_arn,
            release_label,
            account_id,
            region,
        ) {
            Ok(jr) => {
                let resp = wire::StartJobRunResponse {
                    id: Some(jr.id.clone()),
                    name: jr.name.clone(),
                    arn: Some(jr.arn.clone()),
                    virtual_cluster_id: Some(jr.virtual_cluster_id.clone()),
                };
                wire::serialize_start_job_run_response(&resp)
            }
            Err(e) => emr_containers_error_response(&e),
        }
    }

    async fn handle_describe_job_run(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrContainersState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_job_run_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.describe_job_run(&input.virtual_cluster_id, &input.id) {
            Ok(jr) => {
                let resp = wire::DescribeJobRunResponse {
                    job_run: Some(job_run_to_wire(jr)),
                };
                wire::serialize_describe_job_run_response(&resp)
            }
            Err(e) => emr_containers_error_response(&e),
        }
    }

    async fn handle_cancel_job_run(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrContainersState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_job_run_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.cancel_job_run(&input.virtual_cluster_id, &input.id) {
            Ok(jr) => {
                let resp = wire::CancelJobRunResponse {
                    id: Some(jr.id.clone()),
                    virtual_cluster_id: Some(jr.virtual_cluster_id.clone()),
                };
                wire::serialize_cancel_job_run_response(&resp)
            }
            Err(e) => emr_containers_error_response(&e),
        }
    }

    async fn handle_list_job_runs(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrContainersState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_job_runs_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let job_runs = state.list_job_runs(&input.virtual_cluster_id);
        let entries: Vec<wire::JobRun> = job_runs.iter().map(|jr| job_run_to_wire(jr)).collect();
        let resp = wire::ListJobRunsResponse {
            job_runs: Some(entries),
            next_token: None,
        };
        wire::serialize_list_job_runs_response(&resp)
    }

    // ---- Managed Endpoints ----

    #[allow(clippy::too_many_arguments)]
    async fn handle_create_managed_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrContainersState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return rest_json_error(400, "ValidationException", "Invalid JSON body");
        }
        let input = match wire::deserialize_create_managed_endpoint_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'name'");
        }
        let endpoint_type = if input.r#type.is_empty() {
            "JUPYTER_ENTERPRISE_GATEWAY"
        } else {
            input.r#type.as_str()
        };
        let release_label = if input.release_label.is_empty() {
            "emr-6.5.0-latest"
        } else {
            input.release_label.as_str()
        };
        let execution_role_arn = input.execution_role_arn.as_str();
        let certificate_arn = input.certificate_arn.as_deref();
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_managed_endpoint(
            &input.virtual_cluster_id,
            &input.name,
            endpoint_type,
            release_label,
            execution_role_arn,
            certificate_arn,
            tags,
            account_id,
            region,
        ) {
            Ok(ep) => {
                let resp = wire::CreateManagedEndpointResponse {
                    id: Some(ep.id.clone()),
                    name: Some(ep.name.clone()),
                    arn: Some(ep.arn.clone()),
                    virtual_cluster_id: Some(ep.virtual_cluster_id.clone()),
                };
                wire::serialize_create_managed_endpoint_response(&resp)
            }
            Err(e) => emr_containers_error_response(&e),
        }
    }

    async fn handle_describe_managed_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrContainersState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_describe_managed_endpoint_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state = state.read().await;
        match state.describe_managed_endpoint(&input.virtual_cluster_id, &input.id) {
            Ok(ep) => {
                let resp = wire::DescribeManagedEndpointResponse {
                    endpoint: Some(endpoint_to_wire(ep)),
                };
                wire::serialize_describe_managed_endpoint_response(&resp)
            }
            Err(e) => emr_containers_error_response(&e),
        }
    }

    async fn handle_delete_managed_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrContainersState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_managed_endpoint_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_managed_endpoint(&input.virtual_cluster_id, &input.id) {
            Ok((id, vc_id)) => {
                let resp = wire::DeleteManagedEndpointResponse {
                    id: Some(id),
                    virtual_cluster_id: Some(vc_id),
                };
                wire::serialize_delete_managed_endpoint_response(&resp)
            }
            Err(e) => emr_containers_error_response(&e),
        }
    }

    async fn handle_list_managed_endpoints(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrContainersState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_managed_endpoints_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let endpoints = state.list_managed_endpoints(&input.virtual_cluster_id);
        let entries: Vec<wire::Endpoint> =
            endpoints.iter().map(|ep| endpoint_to_wire(ep)).collect();
        let resp = wire::ListManagedEndpointsResponse {
            endpoints: Some(entries),
            next_token: None,
        };
        wire::serialize_list_managed_endpoints_response(&resp)
    }

    async fn handle_get_managed_endpoint_session_credentials(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrContainersState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_managed_endpoint_session_credentials_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.describe_managed_endpoint(
            &input.virtual_cluster_identifier,
            &input.endpoint_identifier,
        ) {
            Ok(_) => {
                let token = format!("mock-session-token-{}", input.endpoint_identifier);
                let expires_at = (chrono::Utc::now() + chrono::Duration::hours(1)).to_rfc3339();
                let id = uuid::Uuid::new_v4()
                    .to_string()
                    .replace('-', "")
                    .chars()
                    .take(17)
                    .collect::<String>();
                let resp = wire::GetManagedEndpointSessionCredentialsResponse {
                    id: Some(id),
                    credentials: Some(wire::Credentials { token: Some(token) }),
                    expires_at: Some(expires_at),
                };
                wire::serialize_get_managed_endpoint_session_credentials_response(&resp)
            }
            Err(e) => emr_containers_error_response(&e),
        }
    }

    // ---- Job Templates ----

    #[allow(clippy::too_many_arguments)]
    async fn handle_create_job_template(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrContainersState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return rest_json_error(400, "ValidationException", "Invalid JSON body");
        }
        let input = match wire::deserialize_create_job_template_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'name'");
        }
        let kms_key_arn = input.kms_key_arn.as_deref();
        let job_template_data =
            to_value(&input.job_template_data).unwrap_or(Value::Object(serde_json::Map::new()));
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_job_template(
            &input.name,
            kms_key_arn,
            job_template_data,
            tags,
            account_id,
            region,
        ) {
            Ok(t) => {
                let resp = wire::CreateJobTemplateResponse {
                    id: Some(t.id.clone()),
                    name: Some(t.name.clone()),
                    arn: Some(t.arn.clone()),
                    created_at: Some(t.created_at.to_rfc3339()),
                };
                wire::serialize_create_job_template_response(&resp)
            }
            Err(e) => emr_containers_error_response(&e),
        }
    }

    async fn handle_describe_job_template(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrContainersState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_job_template_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.describe_job_template(&input.id) {
            Ok(t) => {
                let resp = wire::DescribeJobTemplateResponse {
                    job_template: Some(job_template_to_wire(t)),
                };
                wire::serialize_describe_job_template_response(&resp)
            }
            Err(e) => emr_containers_error_response(&e),
        }
    }

    async fn handle_delete_job_template(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrContainersState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_job_template_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_job_template(&input.id) {
            Ok(deleted_id) => {
                let resp = wire::DeleteJobTemplateResponse {
                    id: Some(deleted_id),
                };
                wire::serialize_delete_job_template_response(&resp)
            }
            Err(e) => emr_containers_error_response(&e),
        }
    }

    async fn handle_list_job_templates(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrContainersState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_list_job_templates_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let templates = state.list_job_templates();
        let entries: Vec<wire::JobTemplate> =
            templates.iter().map(|t| job_template_to_wire(t)).collect();
        let resp = wire::ListJobTemplatesResponse {
            templates: Some(entries),
            next_token: None,
        };
        wire::serialize_list_job_templates_response(&resp)
    }

    // ---- Security Configurations ----

    #[allow(clippy::too_many_arguments)]
    async fn handle_create_security_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrContainersState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return rest_json_error(400, "ValidationException", "Invalid JSON body");
        }
        let input =
            match wire::deserialize_create_security_configuration_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'name'");
        }
        let security_configuration_data = to_value(&input.security_configuration_data)
            .unwrap_or(Value::Object(serde_json::Map::new()));
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_security_configuration(
            &input.name,
            security_configuration_data,
            tags,
            account_id,
            region,
        ) {
            Ok(sc) => {
                let resp = wire::CreateSecurityConfigurationResponse {
                    id: Some(sc.id.clone()),
                    name: Some(sc.name.clone()),
                    arn: Some(sc.arn.clone()),
                };
                wire::serialize_create_security_configuration_response(&resp)
            }
            Err(e) => emr_containers_error_response(&e),
        }
    }

    async fn handle_describe_security_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrContainersState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_describe_security_configuration_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state = state.read().await;
        match state.describe_security_configuration(&input.id) {
            Ok(sc) => {
                let resp = wire::DescribeSecurityConfigurationResponse {
                    security_configuration: Some(security_config_to_wire(sc)),
                };
                wire::serialize_describe_security_configuration_response(&resp)
            }
            Err(e) => emr_containers_error_response(&e),
        }
    }

    async fn handle_list_security_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrContainersState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input =
            match wire::deserialize_list_security_configurations_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state = state.read().await;
        let configs = state.list_security_configurations();
        let entries: Vec<wire::SecurityConfiguration> = configs
            .iter()
            .map(|sc| security_config_to_wire(sc))
            .collect();
        let resp = wire::ListSecurityConfigurationsResponse {
            security_configurations: Some(entries),
            next_token: None,
        };
        wire::serialize_list_security_configurations_response(&resp)
    }

    // ---- Tags ----

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrContainersState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_tags_for_resource(&input.resource_arn) {
            Ok(tags) => {
                let resp = wire::ListTagsForResourceResponse {
                    tags: if tags.is_empty() { None } else { Some(tags) },
                };
                wire::serialize_list_tags_for_resource_response(&resp)
            }
            Err(e) => emr_containers_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrContainersState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return rest_json_error(400, "ValidationException", "Invalid JSON body");
        }
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };

        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, input.tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => emr_containers_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<EmrContainersState>>,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.untag_resource(resource_arn, tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => emr_containers_error_response(&e),
        }
    }
}

fn endpoint_to_wire(ep: &crate::types::ManagedEndpoint) -> wire::Endpoint {
    wire::Endpoint {
        id: Some(ep.id.clone()),
        name: Some(ep.name.clone()),
        virtual_cluster_id: Some(ep.virtual_cluster_id.clone()),
        arn: Some(ep.arn.clone()),
        state: Some(ep.state.clone()),
        r#type: Some(ep.endpoint_type.clone()),
        release_label: Some(ep.release_label.clone()),
        execution_role_arn: Some(ep.execution_role_arn.clone()),
        certificate_arn: ep.certificate_arn.clone(),
        created_at: Some(ep.created_at.to_rfc3339()),
        tags: if ep.tags.is_empty() {
            None
        } else {
            Some(ep.tags.clone())
        },
        ..Default::default()
    }
}

fn job_template_to_wire(t: &crate::types::JobTemplate) -> wire::JobTemplate {
    // Convert stored job_template_data Value back to wire::JobTemplateData
    let job_template_data = serde_json::from_value(t.job_template_data.clone()).ok();
    wire::JobTemplate {
        id: Some(t.id.clone()),
        name: Some(t.name.clone()),
        arn: Some(t.arn.clone()),
        created_at: Some(t.created_at.to_rfc3339()),
        kms_key_arn: t.kms_key_arn.clone(),
        job_template_data,
        tags: if t.tags.is_empty() {
            None
        } else {
            Some(t.tags.clone())
        },
        ..Default::default()
    }
}

fn security_config_to_wire(
    sc: &crate::types::SecurityConfiguration,
) -> wire::SecurityConfiguration {
    let security_configuration_data =
        serde_json::from_value(sc.security_configuration_data.clone()).ok();
    wire::SecurityConfiguration {
        id: Some(sc.id.clone()),
        name: Some(sc.name.clone()),
        arn: Some(sc.arn.clone()),
        created_at: Some(sc.created_at.to_rfc3339()),
        security_configuration_data,
        tags: if sc.tags.is_empty() {
            None
        } else {
            Some(sc.tags.clone())
        },
        ..Default::default()
    }
}

fn job_run_to_wire(jr: &crate::types::JobRun) -> wire::JobRun {
    wire::JobRun {
        id: Some(jr.id.clone()),
        virtual_cluster_id: Some(jr.virtual_cluster_id.clone()),
        arn: Some(jr.arn.clone()),
        state: Some(jr.state.clone()),
        execution_role_arn: Some(jr.execution_role_arn.clone()),
        release_label: Some(jr.release_label.clone()),
        created_at: Some(jr.created_at.to_rfc3339()),
        name: jr.name.clone(),
        ..Default::default()
    }
}

fn virtual_cluster_to_wire(cluster: &crate::types::VirtualCluster) -> wire::VirtualCluster {
    let cp = wire::ContainerProvider {
        r#type: cluster.container_provider.provider_type.clone(),
        id: cluster.container_provider.id.clone(),
        info: cluster
            .container_provider
            .info
            .as_ref()
            .map(|info| wire::ContainerInfo {
                eks_info: info.eks_info.as_ref().map(|eks| wire::EksInfo {
                    namespace: eks.namespace.clone(),
                    ..Default::default()
                }),
            }),
    };

    let tags = if cluster.tags.is_empty() {
        None
    } else {
        Some(cluster.tags.clone())
    };

    wire::VirtualCluster {
        id: Some(cluster.id.clone()),
        name: Some(cluster.name.clone()),
        arn: Some(cluster.arn.clone()),
        state: Some(cluster.state.clone()),
        container_provider: Some(cp),
        created_at: Some(cluster.created_at.to_rfc3339()),
        tags,
        ..Default::default()
    }
}

fn body_has_field(body: &[u8], field: &str) -> bool {
    if body.is_empty() {
        return false;
    }
    let parsed: Result<Value, _> = serde_json::from_slice(body);
    matches!(parsed, Ok(Value::Object(map)) if map.contains_key(field))
}

fn extract_path(uri: &str) -> String {
    if let Some(idx) = uri.find("amazonaws.com") {
        let after_host = &uri[idx + "amazonaws.com".len()..];
        if let Some(q) = after_host.find('?') {
            after_host[..q].to_string()
        } else {
            after_host.to_string()
        }
    } else {
        uri.split('?').next().unwrap_or(uri).to_string()
    }
}

fn parse_single_query(query: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    if query.is_empty() {
        return map;
    }
    for pair in query.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            map.insert(percent_decode(k), percent_decode(v));
        }
    }
    map
}

fn parse_multi_tag_keys(query: &str) -> Vec<String> {
    query
        .split('&')
        .filter_map(|pair| {
            let (key, value) = pair.split_once('=')?;
            if key == "tagKeys" {
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

fn emr_containers_error_response(err: &EmrContainersError) -> MockResponse {
    let (status, error_type) = match err {
        EmrContainersError::VirtualClusterNamespaceConflict => (400, "ValidationException"),
        EmrContainersError::VirtualClusterNotFound(_) => (400, "ValidationException"),
        EmrContainersError::VirtualClusterDoesNotExist => (400, "ValidationException"),
        EmrContainersError::StartJobRunVirtualClusterNotFound(_) => {
            (400, "ResourceNotFoundException")
        }
        EmrContainersError::ReleaseLabelNotFound(_) => (400, "ResourceNotFoundException"),
        EmrContainersError::InvalidJobRunId => (400, "ValidationException"),
        EmrContainersError::JobRunNotFound(_) => (400, "ResourceNotFoundException"),
        EmrContainersError::JobRunNotCancellable(_) => (400, "ValidationException"),
        EmrContainersError::ManagedEndpointVirtualClusterNotFound(_) => {
            (400, "ResourceNotFoundException")
        }
        EmrContainersError::ManagedEndpointNotFound(_) => (400, "ResourceNotFoundException"),
        EmrContainersError::JobTemplateNotFound(_) => (400, "ResourceNotFoundException"),
        EmrContainersError::SecurityConfigurationNotFound(_) => (400, "ResourceNotFoundException"),
        EmrContainersError::ResourceNotFound(_) => (400, "ResourceNotFoundException"),
    };
    let body = json!({
        "Type": "User",
        "Message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "Type": "User",
        "Message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
