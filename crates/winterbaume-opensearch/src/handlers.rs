use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::model;
use crate::state::{OpenSearchError, OpenSearchState};
use crate::types::{DataSourceTypeKind, DirectQueryDataSourceTypeKind};
use crate::views::OpenSearchStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

/// OpenSearch service handler that processes restJson1 protocol requests.
pub struct OpenSearchService {
    pub(crate) state: Arc<BackendState<OpenSearchState>>,
    pub(crate) notifier: StateChangeNotifier<OpenSearchStateView>,
}

impl OpenSearchService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for OpenSearchService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for OpenSearchService {
    fn service_name(&self) -> &str {
        "es"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://es\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl OpenSearchService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let (path, query_string) = extract_path_and_query(&request.uri);
        let method = request.method.as_str();

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        if segments.is_empty() || segments[0] != "2021-01-01" {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        #[allow(clippy::result_large_err)]
        let parse_body = |body: &[u8]| -> Result<Value, MockResponse> {
            if body.is_empty() {
                return Ok(Value::Object(Default::default()));
            }
            serde_json::from_slice(body)
                .map_err(|_| rest_json_error(400, "SerializationException", "Invalid JSON body"))
        };

        let response = match (method, segments.as_slice()) {
            // POST /2021-01-01/opensearch/domain - CreateDomain
            ("POST", ["2021-01-01", "opensearch", "domain"]) => match parse_body(&request.body) {
                Ok(body) => {
                    self.handle_create_domain(&state, &body, account_id, &region)
                        .await
                }
                Err(e) => e,
            },
            // GET /2021-01-01/opensearch/domain/{name} - DescribeDomain
            ("GET", ["2021-01-01", "opensearch", "domain", name]) => {
                self.handle_describe_domain(&state, name).await
            }
            // GET /2021-01-01/opensearch/domain/{name}/config - DescribeDomainConfig
            ("GET", ["2021-01-01", "opensearch", "domain", name, "config"]) => {
                self.handle_describe_domain_config(&state, name).await
            }
            // POST /2021-01-01/opensearch/domain/{name}/config - UpdateDomainConfig
            ("POST", ["2021-01-01", "opensearch", "domain", name, "config"]) => {
                match parse_body(&request.body) {
                    Ok(body) => self.handle_update_domain_config(&state, name, &body).await,
                    Err(e) => e,
                }
            }
            // POST /2021-01-01/opensearch/domain/{name}/config/cancel - CancelDomainConfigChange
            // STUB[no-state]: domain config-change cancellation requires a real
            // pending-change tracker; the mock has no such state and returns an
            // empty response.
            ("POST", ["2021-01-01", "opensearch", "domain", _, "config", "cancel"]) => {
                wire::serialize_cancel_domain_config_change_response(
                    &model::CancelDomainConfigChangeResponse {
                        ..Default::default()
                    },
                )
            }
            // DELETE /2021-01-01/opensearch/domain/{name} - DeleteDomain
            ("DELETE", ["2021-01-01", "opensearch", "domain", name]) => {
                self.handle_delete_domain(&state, name).await
            }
            // POST /2021-01-01/opensearch/domain-info - DescribeDomains
            ("POST", ["2021-01-01", "opensearch", "domain-info"]) => {
                match parse_body(&request.body) {
                    Ok(body) => {
                        self.handle_describe_domains(&state, &body, account_id, &region)
                            .await
                    }
                    Err(e) => e,
                }
            }
            // GET /2021-01-01/opensearch/compatibleVersions - GetCompatibleVersions
            ("GET", ["2021-01-01", "opensearch", "compatibleVersions"]) => {
                self.handle_get_compatible_versions(&state, &query_string)
                    .await
            }
            // GET /2021-01-01/domain - ListDomainNames
            ("GET", ["2021-01-01", "domain"]) => {
                self.handle_list_domain_names(&state, &query_string).await
            }
            // POST /2021-01-01/tags - AddTags
            ("POST", ["2021-01-01", "tags"]) => match parse_body(&request.body) {
                Ok(body) => self.handle_add_tags(&state, &body).await,
                Err(e) => e,
            },
            // GET /2021-01-01/tags?arn=... - ListTags
            ("GET", ["2021-01-01", "tags"]) => self.handle_list_tags(&state, &query_string).await,
            // POST /2021-01-01/tags-removal - RemoveTags
            ("POST", ["2021-01-01", "tags-removal"]) => match parse_body(&request.body) {
                Ok(body) => self.handle_remove_tags(&state, &body).await,
                Err(e) => e,
            },

            // ----------------------------------------------------------------
            // VPC Endpoints
            // ----------------------------------------------------------------

            // POST /2021-01-01/opensearch/vpcEndpoints - CreateVpcEndpoint
            ("POST", ["2021-01-01", "opensearch", "vpcEndpoints"]) => {
                match parse_body(&request.body) {
                    Ok(body) => {
                        self.handle_create_vpc_endpoint(&state, &body, account_id, &region)
                            .await
                    }
                    Err(e) => e,
                }
            }
            // POST /2021-01-01/opensearch/vpcEndpoints/describe - DescribeVpcEndpoints
            ("POST", ["2021-01-01", "opensearch", "vpcEndpoints", "describe"]) => {
                match parse_body(&request.body) {
                    Ok(body) => self.handle_describe_vpc_endpoints(&state, &body).await,
                    Err(e) => e,
                }
            }
            // GET /2021-01-01/opensearch/vpcEndpoints - ListVpcEndpoints
            ("GET", ["2021-01-01", "opensearch", "vpcEndpoints"]) => {
                self.handle_list_vpc_endpoints(&state).await
            }
            // GET /2021-01-01/opensearch/domain/{name}/vpcEndpoints - ListVpcEndpointsForDomain
            ("GET", ["2021-01-01", "opensearch", "domain", name, "vpcEndpoints"]) => {
                self.handle_list_vpc_endpoints_for_domain(&state, name, account_id, &region)
                    .await
            }
            // POST /2021-01-01/opensearch/vpcEndpoints/update - UpdateVpcEndpoint
            ("POST", ["2021-01-01", "opensearch", "vpcEndpoints", "update"]) => {
                match parse_body(&request.body) {
                    Ok(body) => self.handle_update_vpc_endpoint(&state, &body).await,
                    Err(e) => e,
                }
            }
            // DELETE /2021-01-01/opensearch/vpcEndpoints/{id} - DeleteVpcEndpoint
            ("DELETE", ["2021-01-01", "opensearch", "vpcEndpoints", id]) => {
                self.handle_delete_vpc_endpoint(&state, id).await
            }

            // ----------------------------------------------------------------
            // VPC Endpoint Access Authorization
            // ----------------------------------------------------------------

            // POST /2021-01-01/opensearch/domain/{name}/authorizeVpcEndpointAccess
            (
                "POST",
                [
                    "2021-01-01",
                    "opensearch",
                    "domain",
                    name,
                    "authorizeVpcEndpointAccess",
                ],
            ) => match parse_body(&request.body) {
                Ok(body) => {
                    self.handle_authorize_vpc_endpoint_access(&state, name, &body)
                        .await
                }
                Err(e) => e,
            },
            // GET /2021-01-01/opensearch/domain/{name}/listVpcEndpointAccess
            (
                "GET",
                [
                    "2021-01-01",
                    "opensearch",
                    "domain",
                    name,
                    "listVpcEndpointAccess",
                ],
            ) => self.handle_list_vpc_endpoint_access(&state, name).await,
            // POST /2021-01-01/opensearch/domain/{name}/revokeVpcEndpointAccess
            (
                "POST",
                [
                    "2021-01-01",
                    "opensearch",
                    "domain",
                    name,
                    "revokeVpcEndpointAccess",
                ],
            ) => match parse_body(&request.body) {
                Ok(body) => {
                    self.handle_revoke_vpc_endpoint_access(&state, name, &body)
                        .await
                }
                Err(e) => e,
            },

            // ----------------------------------------------------------------
            // Data Sources
            // ----------------------------------------------------------------

            // POST /2021-01-01/opensearch/domain/{name}/dataSource - AddDataSource
            (
                "POST",
                [
                    "2021-01-01",
                    "opensearch",
                    "domain",
                    domain_name,
                    "dataSource",
                ],
            ) => match parse_body(&request.body) {
                Ok(body) => {
                    self.handle_add_data_source(&state, domain_name, &body)
                        .await
                }
                Err(e) => e,
            },
            // GET /2021-01-01/opensearch/domain/{name}/dataSource/{src_name} - GetDataSource
            (
                "GET",
                [
                    "2021-01-01",
                    "opensearch",
                    "domain",
                    domain_name,
                    "dataSource",
                    src_name,
                ],
            ) => {
                self.handle_get_data_source(&state, domain_name, src_name)
                    .await
            }
            // GET /2021-01-01/opensearch/domain/{name}/dataSource - ListDataSources
            (
                "GET",
                [
                    "2021-01-01",
                    "opensearch",
                    "domain",
                    domain_name,
                    "dataSource",
                ],
            ) => self.handle_list_data_sources(&state, domain_name).await,
            // PUT /2021-01-01/opensearch/domain/{name}/dataSource/{src_name} - UpdateDataSource
            (
                "PUT",
                [
                    "2021-01-01",
                    "opensearch",
                    "domain",
                    domain_name,
                    "dataSource",
                    src_name,
                ],
            ) => match parse_body(&request.body) {
                Ok(body) => {
                    self.handle_update_data_source(&state, domain_name, src_name, &body)
                        .await
                }
                Err(e) => e,
            },
            // DELETE /2021-01-01/opensearch/domain/{name}/dataSource/{src_name} - DeleteDataSource
            (
                "DELETE",
                [
                    "2021-01-01",
                    "opensearch",
                    "domain",
                    domain_name,
                    "dataSource",
                    src_name,
                ],
            ) => {
                self.handle_delete_data_source(&state, domain_name, src_name)
                    .await
            }

            // ----------------------------------------------------------------
            // Direct Query Data Sources
            // ----------------------------------------------------------------

            // POST /2021-01-01/opensearch/directQueryDataSource - AddDirectQueryDataSource
            ("POST", ["2021-01-01", "opensearch", "directQueryDataSource"]) => {
                match parse_body(&request.body) {
                    Ok(body) => {
                        self.handle_add_direct_query_data_source(&state, &body, account_id, &region)
                            .await
                    }
                    Err(e) => e,
                }
            }
            // GET /2021-01-01/opensearch/directQueryDataSource/{name} - GetDirectQueryDataSource
            ("GET", ["2021-01-01", "opensearch", "directQueryDataSource", name]) => {
                self.handle_get_direct_query_data_source(&state, name).await
            }
            // GET /2021-01-01/opensearch/directQueryDataSource - ListDirectQueryDataSources
            ("GET", ["2021-01-01", "opensearch", "directQueryDataSource"]) => {
                self.handle_list_direct_query_data_sources(&state).await
            }
            // PUT /2021-01-01/opensearch/directQueryDataSource/{name} - UpdateDirectQueryDataSource
            ("PUT", ["2021-01-01", "opensearch", "directQueryDataSource", name]) => {
                match parse_body(&request.body) {
                    Ok(body) => {
                        self.handle_update_direct_query_data_source(&state, name, &body)
                            .await
                    }
                    Err(e) => e,
                }
            }
            // DELETE /2021-01-01/opensearch/directQueryDataSource/{name} - DeleteDirectQueryDataSource
            ("DELETE", ["2021-01-01", "opensearch", "directQueryDataSource", name]) => {
                self.handle_delete_direct_query_data_source(&state, name)
                    .await
            }

            // ----------------------------------------------------------------
            // Packages
            // ----------------------------------------------------------------

            // POST /2021-01-01/packages - CreatePackage
            ("POST", ["2021-01-01", "packages"]) => match parse_body(&request.body) {
                Ok(body) => {
                    self.handle_create_package(&state, &body, account_id, &region)
                        .await
                }
                Err(e) => e,
            },
            // POST /2021-01-01/packages/describe - DescribePackages
            ("POST", ["2021-01-01", "packages", "describe"]) => {
                self.handle_describe_packages(&state).await
            }
            // DELETE /2021-01-01/packages/{package_id} - DeletePackage
            ("DELETE", ["2021-01-01", "packages", package_id]) => {
                self.handle_delete_package(&state, package_id).await
            }
            // POST /2021-01-01/packages/associate/{package_id}/{domain_name} - AssociatePackage
            (
                "POST",
                [
                    "2021-01-01",
                    "packages",
                    "associate",
                    package_id,
                    domain_name,
                ],
            ) => {
                self.handle_associate_package(&state, package_id, domain_name)
                    .await
            }
            // POST /2021-01-01/packages/dissociate/{package_id}/{domain_name} - DissociatePackage
            (
                "POST",
                [
                    "2021-01-01",
                    "packages",
                    "dissociate",
                    package_id,
                    domain_name,
                ],
            ) => {
                self.handle_dissociate_package(&state, package_id, domain_name)
                    .await
            }
            // GET /2021-01-01/domain/{domain_name}/packages - ListPackagesForDomain
            ("GET", ["2021-01-01", "domain", domain_name, "packages"]) => {
                self.handle_list_packages_for_domain(&state, domain_name)
                    .await
            }
            // GET /2021-01-01/packages/{package_id}/domains - ListDomainsForPackage
            ("GET", ["2021-01-01", "packages", package_id, "domains"]) => {
                self.handle_list_domains_for_package(&state, package_id)
                    .await
            }

            // ----------------------------------------------------------------
            // Cross-Cluster Connections
            // ----------------------------------------------------------------

            // POST /2021-01-01/opensearch/cc/outboundConnection - CreateOutboundConnection
            ("POST", ["2021-01-01", "opensearch", "cc", "outboundConnection"]) => {
                match parse_body(&request.body) {
                    Ok(body) => {
                        self.handle_create_outbound_connection(&state, &body, account_id, &region)
                            .await
                    }
                    Err(e) => e,
                }
            }
            // POST /2021-01-01/opensearch/cc/outboundConnection/search - DescribeOutboundConnections
            (
                "POST",
                [
                    "2021-01-01",
                    "opensearch",
                    "cc",
                    "outboundConnection",
                    "search",
                ],
            ) => self.handle_describe_outbound_connections(&state).await,
            // DELETE /2021-01-01/opensearch/cc/outboundConnection/{id} - DeleteOutboundConnection
            ("DELETE", ["2021-01-01", "opensearch", "cc", "outboundConnection", id]) => {
                self.handle_delete_outbound_connection(&state, id).await
            }
            // POST /2021-01-01/opensearch/cc/inboundConnection/search - DescribeInboundConnections
            (
                "POST",
                [
                    "2021-01-01",
                    "opensearch",
                    "cc",
                    "inboundConnection",
                    "search",
                ],
            ) => self.handle_describe_inbound_connections(&state).await,
            // PUT /2021-01-01/opensearch/cc/inboundConnection/{id}/accept - AcceptInboundConnection
            (
                "PUT",
                [
                    "2021-01-01",
                    "opensearch",
                    "cc",
                    "inboundConnection",
                    id,
                    "accept",
                ],
            ) => self.handle_accept_inbound_connection(&state, id).await,
            // PUT /2021-01-01/opensearch/cc/inboundConnection/{id}/reject - RejectInboundConnection
            (
                "PUT",
                [
                    "2021-01-01",
                    "opensearch",
                    "cc",
                    "inboundConnection",
                    id,
                    "reject",
                ],
            ) => self.handle_reject_inbound_connection(&state, id).await,
            // DELETE /2021-01-01/opensearch/cc/inboundConnection/{id} - DeleteInboundConnection
            ("DELETE", ["2021-01-01", "opensearch", "cc", "inboundConnection", id]) => {
                self.handle_delete_inbound_connection(&state, id).await
            }

            // ----------------------------------------------------------------
            // Reserved Instances
            // ----------------------------------------------------------------

            // GET /2021-01-01/opensearch/reservedInstances - DescribeReservedInstances
            ("GET", ["2021-01-01", "opensearch", "reservedInstances"]) => {
                self.handle_describe_reserved_instances(&state).await
            }
            // POST /2021-01-01/opensearch/purchaseReservedInstanceOffering
            (
                "POST",
                [
                    "2021-01-01",
                    "opensearch",
                    "purchaseReservedInstanceOffering",
                ],
            ) => match parse_body(&request.body) {
                Ok(body) => {
                    self.handle_purchase_reserved_instance_offering(&state, &body)
                        .await
                }
                Err(e) => e,
            },
            // GET /2021-01-01/opensearch/reservedInstanceOfferings
            // STUB[no-state]: reserved-instance offerings are a service-wide
            // catalogue maintained by AWS; the mock has no such catalogue and
            // returns an empty offerings list.
            ("GET", ["2021-01-01", "opensearch", "reservedInstanceOfferings"]) => {
                wire::serialize_describe_reserved_instance_offerings_response(
                    &model::DescribeReservedInstanceOfferingsResponse {
                        reserved_instance_offerings: Some(vec![]),
                        ..Default::default()
                    },
                )
            }

            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        if matches!(method, "POST" | "PUT" | "DELETE") && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let domain_name = match body.get("DomainName").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => return rest_json_error(400, "ValidationException", "Missing 'DomainName'"),
        };

        let engine_version = body.get("EngineVersion").and_then(|v| v.as_str());

        // Extract cluster config
        let cluster_config = body.get("ClusterConfig");
        let instance_type = cluster_config
            .and_then(|c| c.get("InstanceType"))
            .and_then(|v| v.as_str());
        let instance_count = cluster_config
            .and_then(|c| c.get("InstanceCount"))
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);

        // Extract EBS options
        let ebs_options = body.get("EBSOptions");
        let ebs_enabled = ebs_options
            .and_then(|e| e.get("EBSEnabled"))
            .and_then(|v| v.as_bool());
        let ebs_volume_size = ebs_options
            .and_then(|e| e.get("VolumeSize"))
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);
        let ebs_volume_type = ebs_options
            .and_then(|e| e.get("VolumeType"))
            .and_then(|v| v.as_str());

        let access_policies = body.get("AccessPolicies").and_then(|v| v.as_str());

        // Extract TagList
        let initial_tags: std::collections::HashMap<String, String> = body
            .get("TagList")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|t| {
                        let key = t.get("Key")?.as_str()?;
                        let value = t.get("Value")?.as_str()?;
                        Some((key.to_string(), value.to_string()))
                    })
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.create_domain(
            domain_name,
            account_id,
            region,
            engine_version,
            instance_type,
            instance_count,
            ebs_enabled,
            ebs_volume_size,
            ebs_volume_type,
            access_policies,
            initial_tags,
        ) {
            Ok(domain) => wire::serialize_create_domain_response(&model::CreateDomainResponse {
                domain_status: Some(domain_to_status(domain)),
            }),
            Err(e) => opensearch_error_response(&e),
        }
    }

    async fn handle_describe_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_domain(name) {
            Ok(domain) => {
                wire::serialize_describe_domain_response(&model::DescribeDomainResponse {
                    domain_status: Some(domain_to_status(domain)),
                })
            }
            Err(e) => opensearch_error_response(&e),
        }
    }

    async fn handle_describe_domain_config(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_domain(name) {
            Ok(domain) => wire::serialize_describe_domain_config_response(
                &model::DescribeDomainConfigResponse {
                    domain_config: Some(domain_to_config(domain)),
                },
            ),
            Err(e) => opensearch_error_response(&e),
        }
    }

    async fn handle_describe_domains(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        body: &Value,
        _account_id: &str,
        _region: &str,
    ) -> MockResponse {
        let domain_names: Vec<String> = body
            .get("DomainNames")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let state = state.read().await;
        let domains = state.describe_domains(&domain_names);
        let status_list: Vec<model::DomainStatus> =
            domains.iter().map(|d| domain_to_status(d)).collect();

        wire::serialize_describe_domains_response(&model::DescribeDomainsResponse {
            domain_status_list: Some(status_list),
        })
    }

    async fn handle_update_domain_config(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        name: &str,
        body: &Value,
    ) -> MockResponse {
        let engine_version = body.get("EngineVersion").and_then(|v| v.as_str());

        let cluster_config = body.get("ClusterConfig");
        let instance_type = cluster_config
            .and_then(|c| c.get("InstanceType"))
            .and_then(|v| v.as_str());
        let instance_count = cluster_config
            .and_then(|c| c.get("InstanceCount"))
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);

        let ebs_options = body.get("EBSOptions");
        let ebs_enabled = ebs_options
            .and_then(|e| e.get("EBSEnabled"))
            .and_then(|v| v.as_bool());
        let ebs_volume_size = ebs_options
            .and_then(|e| e.get("VolumeSize"))
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);
        let ebs_volume_type = ebs_options
            .and_then(|e| e.get("VolumeType"))
            .and_then(|v| v.as_str());

        let access_policies = body.get("AccessPolicies").and_then(|v| v.as_str());

        let mut state = state.write().await;
        match state.update_domain_config(
            name,
            engine_version,
            instance_type,
            instance_count,
            ebs_enabled,
            ebs_volume_size,
            ebs_volume_type,
            access_policies,
        ) {
            Ok(domain) => {
                wire::serialize_update_domain_config_response(&model::UpdateDomainConfigResponse {
                    domain_config: Some(domain_to_config(domain)),
                    dry_run_progress_status: None,
                    dry_run_results: None,
                })
            }
            Err(e) => opensearch_error_response(&e),
        }
    }

    async fn handle_delete_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_domain(name) {
            Ok(domain) => wire::serialize_delete_domain_response(&model::DeleteDomainResponse {
                domain_status: Some(domain_to_status(&domain)),
            }),
            Err(e) => opensearch_error_response(&e),
        }
    }

    async fn handle_list_domain_names(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        query_string: &str,
    ) -> MockResponse {
        let engine_type_filter = extract_query_param(query_string, "engineType");

        // Validate the engine type if provided
        if let Some(ref et) = engine_type_filter {
            let valid_types = ["OpenSearch", "Elasticsearch"];
            if !valid_types.contains(&et.as_str()) {
                return rest_json_error(
                    400,
                    "EngineTypeNotFoundException",
                    &format!("Engine Type not found: {et}"),
                );
            }
        }

        let state = state.read().await;
        let domains = state.list_domain_names();
        let entries: Vec<model::DomainInfo> = domains
            .iter()
            .filter(|d| {
                if let Some(ref et) = engine_type_filter {
                    &engine_type_from_version(&d.engine_version) == et
                } else {
                    true
                }
            })
            .map(|d| model::DomainInfo {
                domain_name: Some(d.domain_name.clone()),
                engine_type: Some(engine_type_from_version(&d.engine_version)),
            })
            .collect();
        wire::serialize_list_domain_names_response(&model::ListDomainNamesResponse {
            domain_names: Some(entries),
        })
    }

    async fn handle_get_compatible_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        query_string: &str,
    ) -> MockResponse {
        let domain_name = extract_query_param(query_string, "domainName");

        // If a domain name is specified, verify it exists
        if let Some(ref name) = domain_name {
            let state = state.read().await;
            if let Err(e) = state.describe_domain(name) {
                return opensearch_error_response(&e);
            }
        }

        // Return a static compatible versions map similar to moto
        let compatible_versions = vec![
            model::CompatibleVersionsMap {
                source_version: Some("OpenSearch_2.11".to_string()),
                target_versions: Some(vec![
                    "OpenSearch_2.13".to_string(),
                    "OpenSearch_2.15".to_string(),
                    "OpenSearch_2.17".to_string(),
                ]),
            },
            model::CompatibleVersionsMap {
                source_version: Some("OpenSearch_2.9".to_string()),
                target_versions: Some(vec![
                    "OpenSearch_2.11".to_string(),
                    "OpenSearch_2.13".to_string(),
                    "OpenSearch_2.15".to_string(),
                ]),
            },
            model::CompatibleVersionsMap {
                source_version: Some("OpenSearch_2.7".to_string()),
                target_versions: Some(vec![
                    "OpenSearch_2.9".to_string(),
                    "OpenSearch_2.11".to_string(),
                    "OpenSearch_2.13".to_string(),
                ]),
            },
            model::CompatibleVersionsMap {
                source_version: Some("OpenSearch_1.3".to_string()),
                target_versions: Some(vec![
                    "OpenSearch_2.7".to_string(),
                    "OpenSearch_2.9".to_string(),
                    "OpenSearch_2.11".to_string(),
                ]),
            },
        ];

        wire::serialize_get_compatible_versions_response(&model::GetCompatibleVersionsResponse {
            compatible_versions: Some(compatible_versions),
        })
    }

    async fn handle_add_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        body: &Value,
    ) -> MockResponse {
        let arn = match body.get("ARN").and_then(|v| v.as_str()) {
            Some(a) => a,
            None => return rest_json_error(400, "ValidationException", "Missing 'ARN'"),
        };

        let tag_list = match body.get("TagList").and_then(|v| v.as_array()) {
            Some(t) => t,
            None => return rest_json_error(400, "ValidationException", "Missing 'TagList'"),
        };

        let tags: Vec<(String, String)> = tag_list
            .iter()
            .filter_map(|t| {
                let key = t.get("Key")?.as_str()?;
                let value = t.get("Value")?.as_str()?;
                Some((key.to_string(), value.to_string()))
            })
            .collect();

        let mut state = state.write().await;
        state.add_tags(arn, &tags);

        wire::serialize_add_tags_response()
    }

    async fn handle_list_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        query_string: &str,
    ) -> MockResponse {
        let arn = match extract_query_param(query_string, "arn") {
            Some(a) => a,
            None => return rest_json_error(400, "ValidationException", "Missing 'arn' parameter"),
        };

        let state = state.read().await;
        let tags = state.list_tags(&arn);
        let tag_list: Vec<model::Tag> = tags
            .iter()
            .map(|(k, v)| model::Tag {
                key: k.clone(),
                value: v.clone(),
            })
            .collect();

        wire::serialize_list_tags_response(&model::ListTagsResponse {
            tag_list: Some(tag_list),
        })
    }

    async fn handle_remove_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        body: &Value,
    ) -> MockResponse {
        let arn = match body.get("ARN").and_then(|v| v.as_str()) {
            Some(a) => a,
            None => return rest_json_error(400, "ValidationException", "Missing 'ARN'"),
        };

        let tag_keys: Vec<String> = body
            .get("TagKeys")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        state.remove_tags(arn, &tag_keys);

        wire::serialize_remove_tags_response()
    }

    // -------------------------------------------------------------------------
    // VPC Endpoint handlers
    // -------------------------------------------------------------------------

    async fn handle_create_vpc_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let domain_arn = match body.get("DomainArn").and_then(|v| v.as_str()) {
            Some(a) => a,
            None => return rest_json_error(400, "ValidationException", "Missing 'DomainArn'"),
        };

        let vpc_options = body.get("VpcOptions");
        let subnet_ids = vpc_options
            .and_then(|o| o.get("SubnetIds"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let security_group_ids = vpc_options
            .and_then(|o| o.get("SecurityGroupIds"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.create_vpc_endpoint(
            account_id,
            region,
            domain_arn,
            subnet_ids,
            security_group_ids,
        ) {
            Ok(ep) => {
                wire::serialize_create_vpc_endpoint_response(&model::CreateVpcEndpointResponse {
                    vpc_endpoint: Some(vpc_endpoint_to_model(&ep)),
                })
            }
            Err(e) => opensearch_error_response(&e),
        }
    }

    async fn handle_describe_vpc_endpoints(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        body: &Value,
    ) -> MockResponse {
        let vpc_endpoint_ids: Vec<String> = body
            .get("VpcEndpointIds")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let state = state.read().await;
        let (found, not_found) = state.describe_vpc_endpoints(&vpc_endpoint_ids);
        let vpc_endpoints = found.iter().map(vpc_endpoint_to_model).collect();
        let errors: Vec<model::VpcEndpointError> = not_found
            .iter()
            .map(|id| model::VpcEndpointError {
                vpc_endpoint_id: Some(id.clone()),
                error_code: Some("ENDPOINT_NOT_FOUND".to_string()),
                error_message: Some(format!("VPC endpoint {id} not found")),
            })
            .collect();

        wire::serialize_describe_vpc_endpoints_response(&model::DescribeVpcEndpointsResponse {
            vpc_endpoints: Some(vpc_endpoints),
            vpc_endpoint_errors: if errors.is_empty() {
                None
            } else {
                Some(errors)
            },
        })
    }

    async fn handle_list_vpc_endpoints(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let endpoints = state.list_vpc_endpoints();
        let summary_list: Vec<model::VpcEndpointSummary> = endpoints
            .iter()
            .map(|ep| vpc_endpoint_to_summary(ep))
            .collect();
        wire::serialize_list_vpc_endpoints_response(&model::ListVpcEndpointsResponse {
            vpc_endpoint_summary_list: Some(summary_list),
            next_token: None,
        })
    }

    async fn handle_list_vpc_endpoints_for_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        domain_name: &str,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let state = state.read().await;
        // Get domain ARN for filtering
        let domain_arn = match state.describe_domain(domain_name) {
            Ok(d) => d.arn.clone(),
            Err(e) => return opensearch_error_response(&e),
        };
        let _ = (account_id, region);
        let endpoints = state.list_vpc_endpoints_for_domain(&domain_arn);
        let summary_list: Vec<model::VpcEndpointSummary> = endpoints
            .iter()
            .map(|ep| vpc_endpoint_to_summary(ep))
            .collect();
        wire::serialize_list_vpc_endpoints_for_domain_response(
            &model::ListVpcEndpointsForDomainResponse {
                vpc_endpoint_summary_list: Some(summary_list),
                next_token: None,
            },
        )
    }

    async fn handle_update_vpc_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        body: &Value,
    ) -> MockResponse {
        let vpc_endpoint_id = match body.get("VpcEndpointId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return rest_json_error(400, "ValidationException", "Missing 'VpcEndpointId'"),
        };

        let vpc_options = body.get("VpcOptions");
        let subnet_ids = vpc_options
            .and_then(|o| o.get("SubnetIds"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect::<Vec<_>>()
            });
        let security_group_ids = vpc_options
            .and_then(|o| o.get("SecurityGroupIds"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect::<Vec<_>>()
            });

        let mut state = state.write().await;
        match state.update_vpc_endpoint(vpc_endpoint_id, subnet_ids, security_group_ids) {
            Ok(ep) => {
                wire::serialize_update_vpc_endpoint_response(&model::UpdateVpcEndpointResponse {
                    vpc_endpoint: Some(vpc_endpoint_to_model(&ep)),
                })
            }
            Err(e) => opensearch_error_response(&e),
        }
    }

    async fn handle_delete_vpc_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        vpc_endpoint_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_vpc_endpoint(vpc_endpoint_id) {
            Ok(ep) => {
                wire::serialize_delete_vpc_endpoint_response(&model::DeleteVpcEndpointResponse {
                    vpc_endpoint_summary: Some(vpc_endpoint_to_summary(&ep)),
                })
            }
            Err(e) => opensearch_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // VPC Endpoint Access Authorization handlers
    // -------------------------------------------------------------------------

    async fn handle_authorize_vpc_endpoint_access(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        domain_name: &str,
        body: &Value,
    ) -> MockResponse {
        let account = body.get("Account").and_then(|v| v.as_str());
        let service = body.get("Service").and_then(|v| v.as_str());

        let mut state = state.write().await;
        match state.authorize_vpc_endpoint_access(domain_name, account, service) {
            Ok(ap) => wire::serialize_authorize_vpc_endpoint_access_response(
                &model::AuthorizeVpcEndpointAccessResponse {
                    authorized_principal: Some(model::AuthorizedPrincipal {
                        principal: Some(ap.principal),
                        principal_type: Some(ap.principal_type),
                    }),
                },
            ),
            Err(e) => opensearch_error_response(&e),
        }
    }

    async fn handle_list_vpc_endpoint_access(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        domain_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_vpc_endpoint_access(domain_name) {
            Ok(principals) => {
                let list: Vec<model::AuthorizedPrincipal> = principals
                    .iter()
                    .map(|ap| model::AuthorizedPrincipal {
                        principal: Some(ap.principal.clone()),
                        principal_type: Some(ap.principal_type.clone()),
                    })
                    .collect();
                wire::serialize_list_vpc_endpoint_access_response(
                    &model::ListVpcEndpointAccessResponse {
                        authorized_principal_list: Some(list),
                        next_token: None,
                    },
                )
            }
            Err(e) => opensearch_error_response(&e),
        }
    }

    async fn handle_revoke_vpc_endpoint_access(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        domain_name: &str,
        body: &Value,
    ) -> MockResponse {
        let account = body.get("Account").and_then(|v| v.as_str());
        let service = body.get("Service").and_then(|v| v.as_str());

        let mut state = state.write().await;
        match state.revoke_vpc_endpoint_access(domain_name, account, service) {
            Ok(()) => wire::serialize_revoke_vpc_endpoint_access_response(
                &model::RevokeVpcEndpointAccessResponse {},
            ),
            Err(e) => opensearch_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Data Source handlers
    // -------------------------------------------------------------------------

    async fn handle_add_data_source(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        domain_name: &str,
        body: &Value,
    ) -> MockResponse {
        let name = match body.get("Name").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => return rest_json_error(400, "ValidationException", "Missing 'Name'"),
        };
        let description = body.get("Description").and_then(|v| v.as_str());
        let ds_type = parse_data_source_type(body);

        let mut state = state.write().await;
        match state.add_data_source(domain_name, name, description, ds_type) {
            Ok(()) => wire::serialize_add_data_source_response(&model::AddDataSourceResponse {
                message: Some("Data source added successfully".to_string()),
            }),
            Err(e) => opensearch_error_response(&e),
        }
    }

    async fn handle_get_data_source(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        domain_name: &str,
        src_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_data_source(domain_name, src_name) {
            Ok(ds) => wire::serialize_get_data_source_response(&model::GetDataSourceResponse {
                name: Some(ds.name.clone()),
                description: ds.description.clone(),
                status: Some(ds.status.clone()),
                data_source_type: Some(data_source_type_to_model(&ds.data_source_type)),
            }),
            Err(e) => opensearch_error_response(&e),
        }
    }

    async fn handle_list_data_sources(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        domain_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_data_sources(domain_name) {
            Ok(sources) => {
                let list: Vec<model::DataSourceDetails> = sources
                    .iter()
                    .map(|ds| model::DataSourceDetails {
                        name: Some(ds.name.clone()),
                        description: ds.description.clone(),
                        status: Some(ds.status.clone()),
                        data_source_type: Some(data_source_type_to_model(&ds.data_source_type)),
                    })
                    .collect();
                wire::serialize_list_data_sources_response(&model::ListDataSourcesResponse {
                    data_sources: Some(list),
                })
            }
            Err(e) => opensearch_error_response(&e),
        }
    }

    async fn handle_update_data_source(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        domain_name: &str,
        src_name: &str,
        body: &Value,
    ) -> MockResponse {
        let description = body.get("Description").and_then(|v| v.as_str());
        let ds_type = if body.get("DataSourceType").is_some() {
            Some(parse_data_source_type(body))
        } else {
            None
        };

        let mut state = state.write().await;
        match state.update_data_source(domain_name, src_name, description, ds_type) {
            Ok(ds) => {
                wire::serialize_update_data_source_response(&model::UpdateDataSourceResponse {
                    message: Some(format!("Data source {} updated", ds.name)),
                })
            }
            Err(e) => opensearch_error_response(&e),
        }
    }

    async fn handle_delete_data_source(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        domain_name: &str,
        src_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_data_source(domain_name, src_name) {
            Ok(()) => {
                wire::serialize_delete_data_source_response(&model::DeleteDataSourceResponse {
                    message: Some("Data source deleted successfully".to_string()),
                })
            }
            Err(e) => opensearch_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Direct Query Data Source handlers
    // -------------------------------------------------------------------------

    async fn handle_add_direct_query_data_source(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match body.get("DataSourceName").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => return rest_json_error(400, "ValidationException", "Missing 'DataSourceName'"),
        };
        let description = body.get("Description").and_then(|v| v.as_str());
        let open_search_arns = body
            .get("OpenSearchArns")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let tags = body
            .get("TagList")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|t| {
                        let key = t.get("Key")?.as_str()?;
                        let value = t.get("Value")?.as_str()?;
                        Some((key.to_string(), value.to_string()))
                    })
                    .collect()
            })
            .unwrap_or_default();

        let ds_type = parse_direct_query_data_source_type(body);

        let mut state = state.write().await;
        match state.add_direct_query_data_source(
            account_id,
            region,
            name,
            description,
            ds_type,
            open_search_arns,
            tags,
        ) {
            Ok(ds) => wire::serialize_add_direct_query_data_source_response(
                &model::AddDirectQueryDataSourceResponse {
                    data_source_arn: Some(ds.data_source_arn.clone()),
                },
            ),
            Err(e) => opensearch_error_response(&e),
        }
    }

    async fn handle_get_direct_query_data_source(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_direct_query_data_source(name) {
            Ok(ds) => wire::serialize_get_direct_query_data_source_response(
                &model::GetDirectQueryDataSourceResponse {
                    data_source_name: Some(ds.data_source_name.clone()),
                    data_source_arn: Some(ds.data_source_arn.clone()),
                    description: ds.description.clone(),
                    open_search_arns: Some(ds.open_search_arns.clone()),
                    data_source_type: Some(direct_query_data_source_type_to_model(
                        &ds.data_source_type,
                    )),
                    data_source_access_policy: None,
                },
            ),
            Err(e) => opensearch_error_response(&e),
        }
    }

    async fn handle_list_direct_query_data_sources(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let sources = state.list_direct_query_data_sources();
        let list: Vec<model::DirectQueryDataSource> = sources
            .iter()
            .map(|ds| model::DirectQueryDataSource {
                data_source_name: Some(ds.data_source_name.clone()),
                data_source_arn: Some(ds.data_source_arn.clone()),
                description: ds.description.clone(),
                open_search_arns: Some(ds.open_search_arns.clone()),
                data_source_type: Some(direct_query_data_source_type_to_model(
                    &ds.data_source_type,
                )),
                tag_list: Some(
                    ds.tags
                        .iter()
                        .map(|(k, v)| model::Tag {
                            key: k.clone(),
                            value: v.clone(),
                        })
                        .collect(),
                ),
            })
            .collect();
        wire::serialize_list_direct_query_data_sources_response(
            &model::ListDirectQueryDataSourcesResponse {
                direct_query_data_sources: Some(list),
                next_token: None,
            },
        )
    }

    async fn handle_update_direct_query_data_source(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        name: &str,
        body: &Value,
    ) -> MockResponse {
        let description = body.get("Description").and_then(|v| v.as_str());
        let open_search_arns = body
            .get("OpenSearchArns")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect::<Vec<_>>()
            });
        let ds_type = if body.get("DataSourceType").is_some() {
            Some(parse_direct_query_data_source_type(body))
        } else {
            None
        };

        let mut state = state.write().await;
        match state.update_direct_query_data_source(name, description, ds_type, open_search_arns) {
            Ok(ds) => wire::serialize_update_direct_query_data_source_response(
                &model::UpdateDirectQueryDataSourceResponse {
                    data_source_arn: Some(ds.data_source_arn.clone()),
                },
            ),
            Err(e) => opensearch_error_response(&e),
        }
    }

    async fn handle_delete_direct_query_data_source(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_direct_query_data_source(name) {
            Ok(()) => wire::serialize_delete_direct_query_data_source_response(),
            Err(e) => opensearch_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Package handlers
    // -------------------------------------------------------------------------

    async fn handle_create_package(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let package_name = match body.get("PackageName").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => return rest_json_error(400, "ValidationException", "Missing 'PackageName'"),
        };
        let package_type = match body.get("PackageType").and_then(|v| v.as_str()) {
            Some(t) => t,
            None => return rest_json_error(400, "ValidationException", "Missing 'PackageType'"),
        };
        let description = body.get("PackageDescription").and_then(|v| v.as_str());
        let engine_version = body.get("EngineVersion").and_then(|v| v.as_str());

        let mut state = state.write().await;
        match state.create_package(
            account_id,
            region,
            package_name,
            package_type,
            description,
            engine_version,
        ) {
            Ok(pkg) => wire::serialize_create_package_response(&model::CreatePackageResponse {
                package_details: Some(package_to_model(pkg)),
            }),
            Err(e) => opensearch_error_response(&e),
        }
    }

    async fn handle_describe_packages(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let packages = state.describe_packages();
        let list: Vec<model::PackageDetails> =
            packages.iter().map(|p| package_to_model(p)).collect();
        wire::serialize_describe_packages_response(&model::DescribePackagesResponse {
            package_details_list: Some(list),
            next_token: None,
        })
    }

    async fn handle_delete_package(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        package_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_package(package_id) {
            Ok(pkg) => wire::serialize_delete_package_response(&model::DeletePackageResponse {
                package_details: Some(package_to_model(&pkg)),
            }),
            Err(e) => opensearch_error_response(&e),
        }
    }

    async fn handle_associate_package(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        package_id: &str,
        domain_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.associate_package(package_id, domain_name) {
            Ok(()) => {
                let pkg = state.packages.get(package_id).unwrap();
                wire::serialize_associate_package_response(&model::AssociatePackageResponse {
                    domain_package_details: Some(domain_package_details(pkg, domain_name)),
                })
            }
            Err(e) => opensearch_error_response(&e),
        }
    }

    async fn handle_dissociate_package(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        package_id: &str,
        domain_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.dissociate_package(package_id, domain_name) {
            Ok(()) => {
                let pkg = state.packages.get(package_id).unwrap();
                wire::serialize_dissociate_package_response(&model::DissociatePackageResponse {
                    domain_package_details: Some(domain_package_details(pkg, domain_name)),
                })
            }
            Err(e) => opensearch_error_response(&e),
        }
    }

    async fn handle_list_packages_for_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        domain_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let packages = state.list_packages_for_domain(domain_name);
        let list: Vec<model::DomainPackageDetails> = packages
            .iter()
            .map(|p| domain_package_details(p, domain_name))
            .collect();
        wire::serialize_list_packages_for_domain_response(&model::ListPackagesForDomainResponse {
            domain_package_details_list: Some(list),
            next_token: None,
        })
    }

    async fn handle_list_domains_for_package(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        package_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let pkg = match state.packages.get(package_id) {
            Some(p) => p,
            None => {
                return rest_json_error(
                    409,
                    "ResourceNotFoundException",
                    &format!("Package [{package_id}] not found"),
                );
            }
        };
        let domain_names = state.list_domains_for_package(package_id);
        let list: Vec<model::DomainPackageDetails> = domain_names
            .iter()
            .map(|dn| domain_package_details(pkg, dn))
            .collect();
        wire::serialize_list_domains_for_package_response(&model::ListDomainsForPackageResponse {
            domain_package_details_list: Some(list),
            next_token: None,
        })
    }

    // -------------------------------------------------------------------------
    // Cross-Cluster Connection handlers
    // -------------------------------------------------------------------------

    async fn handle_create_outbound_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let connection_alias = match body.get("ConnectionAlias").and_then(|v| v.as_str()) {
            Some(a) => a,
            None => {
                return rest_json_error(400, "ValidationException", "Missing 'ConnectionAlias'");
            }
        };
        let connection_mode = body.get("ConnectionMode").and_then(|v| v.as_str());

        let local_info = body.get("LocalDomainInfo");
        let local_domain_name = local_info
            .and_then(|i| i.get("AWSDomainInformation"))
            .and_then(|i| i.get("DomainName"))
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let local_owner_id = local_info
            .and_then(|i| i.get("AWSDomainInformation"))
            .and_then(|i| i.get("OwnerId"))
            .and_then(|v| v.as_str())
            .unwrap_or(account_id);
        let local_region_val = local_info
            .and_then(|i| i.get("AWSDomainInformation"))
            .and_then(|i| i.get("Region"))
            .and_then(|v| v.as_str());

        let remote_info = body.get("RemoteDomainInfo");
        let remote_domain_name = remote_info
            .and_then(|i| i.get("AWSDomainInformation"))
            .and_then(|i| i.get("DomainName"))
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let remote_owner_id = remote_info
            .and_then(|i| i.get("AWSDomainInformation"))
            .and_then(|i| i.get("OwnerId"))
            .and_then(|v| v.as_str());
        let remote_region_val = remote_info
            .and_then(|i| i.get("AWSDomainInformation"))
            .and_then(|i| i.get("Region"))
            .and_then(|v| v.as_str());

        let _ = region;
        let mut state = state.write().await;
        let conn = state.create_outbound_connection(
            connection_alias,
            connection_mode,
            local_domain_name,
            local_owner_id,
            local_region_val,
            remote_domain_name,
            remote_owner_id,
            remote_region_val,
        );
        wire::serialize_create_outbound_connection_response(
            &model::CreateOutboundConnectionResponse {
                connection_alias: Some(conn.connection_alias.clone()),
                connection_id: Some(conn.connection_id.clone()),
                connection_mode: conn.connection_mode.clone(),
                connection_status: Some(model::OutboundConnectionStatus {
                    status_code: Some(conn.status_code.clone()),
                    message: None,
                }),
                local_domain_info: Some(make_domain_info_container(
                    &conn.local_domain_name,
                    Some(&conn.local_owner_id),
                    conn.local_region.as_deref(),
                )),
                remote_domain_info: Some(make_domain_info_container(
                    &conn.remote_domain_name,
                    conn.remote_owner_id.as_deref(),
                    conn.remote_region.as_deref(),
                )),
                connection_properties: None,
            },
        )
    }

    async fn handle_describe_outbound_connections(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let connections = state.describe_outbound_connections();
        let list: Vec<model::OutboundConnection> = connections
            .iter()
            .map(|c| outbound_connection_to_model(c))
            .collect();
        wire::serialize_describe_outbound_connections_response(
            &model::DescribeOutboundConnectionsResponse {
                connections: Some(list),
                next_token: None,
            },
        )
    }

    async fn handle_delete_outbound_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        connection_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_outbound_connection(connection_id) {
            Ok(conn) => wire::serialize_delete_outbound_connection_response(
                &model::DeleteOutboundConnectionResponse {
                    connection: Some(outbound_connection_to_model(&conn)),
                },
            ),
            Err(e) => opensearch_error_response(&e),
        }
    }

    async fn handle_describe_inbound_connections(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let connections = state.describe_inbound_connections();
        let list: Vec<model::InboundConnection> = connections
            .iter()
            .map(|c| inbound_connection_to_model(c))
            .collect();
        wire::serialize_describe_inbound_connections_response(
            &model::DescribeInboundConnectionsResponse {
                connections: Some(list),
                next_token: None,
            },
        )
    }

    async fn handle_accept_inbound_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        connection_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.accept_inbound_connection(connection_id) {
            Ok(conn) => wire::serialize_accept_inbound_connection_response(
                &model::AcceptInboundConnectionResponse {
                    connection: Some(inbound_connection_to_model(conn)),
                },
            ),
            Err(e) => opensearch_error_response(&e),
        }
    }

    async fn handle_reject_inbound_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        connection_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.reject_inbound_connection(connection_id) {
            Ok(conn) => wire::serialize_reject_inbound_connection_response(
                &model::RejectInboundConnectionResponse {
                    connection: Some(inbound_connection_to_model(&conn)),
                },
            ),
            Err(e) => opensearch_error_response(&e),
        }
    }

    async fn handle_delete_inbound_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        connection_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_inbound_connection(connection_id) {
            Ok(conn) => wire::serialize_delete_inbound_connection_response(
                &model::DeleteInboundConnectionResponse {
                    connection: Some(inbound_connection_to_model(&conn)),
                },
            ),
            Err(e) => opensearch_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Reserved Instance handlers
    // -------------------------------------------------------------------------

    async fn handle_describe_reserved_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let instances = state.describe_reserved_instances();
        let list: Vec<model::ReservedInstance> = instances
            .iter()
            .map(|ri| reserved_instance_to_model(ri))
            .collect();
        wire::serialize_describe_reserved_instances_response(
            &model::DescribeReservedInstancesResponse {
                reserved_instances: Some(list),
                next_token: None,
            },
        )
    }

    async fn handle_purchase_reserved_instance_offering(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchState>>,
        body: &Value,
    ) -> MockResponse {
        let reservation_name = match body.get("ReservationName").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return rest_json_error(400, "ValidationException", "Missing 'ReservationName'");
            }
        };
        let reserved_instance_offering_id = match body
            .get("ReservedInstanceOfferingId")
            .and_then(|v| v.as_str())
        {
            Some(id) => id,
            None => {
                return rest_json_error(
                    400,
                    "ValidationException",
                    "Missing 'ReservedInstanceOfferingId'",
                );
            }
        };
        let instance_count = body
            .get("InstanceCount")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);

        let mut state = state.write().await;
        let ri = state.purchase_reserved_instance_offering(
            reservation_name,
            reserved_instance_offering_id,
            instance_count,
        );
        wire::serialize_purchase_reserved_instance_offering_response(
            &model::PurchaseReservedInstanceOfferingResponse {
                reservation_name: Some(ri.reservation_name),
                reserved_instance_id: Some(ri.reserved_instance_id),
            },
        )
    }
}

// ---------------------------------------------------------------------------
// Helper conversion functions
// ---------------------------------------------------------------------------

/// Convert a state Domain to a wire DomainStatus model.
fn domain_to_status(domain: &crate::types::Domain) -> model::DomainStatus {
    model::DomainStatus {
        domain_id: Some(domain.domain_id.clone()),
        domain_name: Some(domain.domain_name.clone()),
        a_r_n: Some(domain.arn.clone()),
        created: Some(domain.created),
        deleted: Some(domain.deleted),
        endpoint: domain.endpoint.clone(),
        processing: Some(domain.processing),
        engine_version: Some(domain.engine_version.clone()),
        cluster_config: Some(model::ClusterConfig {
            instance_type: Some(domain.instance_type.clone()),
            instance_count: Some(domain.instance_count),
            dedicated_master_enabled: Some(domain.dedicated_master_enabled),
            zone_awareness_enabled: Some(domain.zone_awareness_enabled),
            ..Default::default()
        }),
        e_b_s_options: Some(model::EBSOptions {
            e_b_s_enabled: Some(domain.ebs_enabled),
            volume_size: Some(domain.ebs_volume_size),
            volume_type: Some(domain.ebs_volume_type.clone()),
            ..Default::default()
        }),
        access_policies: Some(domain.access_policies.clone()),
        advanced_security_options: Some(model::AdvancedSecurityOptions {
            enabled: Some(false),
            internal_user_database_enabled: Some(false),
            ..Default::default()
        }),
        advanced_options: Some(std::collections::HashMap::new()),
        ..Default::default()
    }
}

/// Convert a state Domain to a wire DomainConfig model.
fn domain_to_config(domain: &crate::types::Domain) -> model::DomainConfig {
    let status = model::OptionStatus {
        creation_date: Some(0.0),
        pending_deletion: Some(false),
        state: Some("Active".to_string()),
        update_date: Some(0.0),
        ..Default::default()
    };

    model::DomainConfig {
        engine_version: Some(model::VersionStatus {
            options: Some(domain.engine_version.clone()),
            status: Some(status.clone()),
        }),
        cluster_config: Some(model::ClusterConfigStatus {
            options: Some(model::ClusterConfig {
                instance_type: Some(domain.instance_type.clone()),
                instance_count: Some(domain.instance_count),
                dedicated_master_enabled: Some(domain.dedicated_master_enabled),
                zone_awareness_enabled: Some(domain.zone_awareness_enabled),
                ..Default::default()
            }),
            status: Some(status.clone()),
        }),
        e_b_s_options: Some(model::EBSOptionsStatus {
            options: Some(model::EBSOptions {
                e_b_s_enabled: Some(domain.ebs_enabled),
                volume_size: Some(domain.ebs_volume_size),
                volume_type: Some(domain.ebs_volume_type.clone()),
                ..Default::default()
            }),
            status: Some(status.clone()),
        }),
        access_policies: Some(model::AccessPoliciesStatus {
            options: Some(domain.access_policies.clone()),
            status: Some(status),
        }),
        ..Default::default()
    }
}

fn vpc_endpoint_to_model(ep: &crate::types::VpcEndpoint) -> model::VpcEndpoint {
    model::VpcEndpoint {
        vpc_endpoint_id: Some(ep.vpc_endpoint_id.clone()),
        vpc_endpoint_owner: Some(ep.vpc_endpoint_owner.clone()),
        domain_arn: Some(ep.domain_arn.clone()),
        endpoint: ep.endpoint.clone(),
        status: Some(ep.status.clone()),
        vpc_options: Some(model::VPCDerivedInfo {
            subnet_ids: Some(ep.subnet_ids.clone()),
            security_group_ids: Some(ep.security_group_ids.clone()),
            ..Default::default()
        }),
    }
}

fn vpc_endpoint_to_summary(ep: &crate::types::VpcEndpoint) -> model::VpcEndpointSummary {
    model::VpcEndpointSummary {
        vpc_endpoint_id: Some(ep.vpc_endpoint_id.clone()),
        vpc_endpoint_owner: Some(ep.vpc_endpoint_owner.clone()),
        domain_arn: Some(ep.domain_arn.clone()),
        status: Some(ep.status.clone()),
    }
}

fn data_source_type_to_model(t: &DataSourceTypeKind) -> model::DataSourceType {
    match t {
        DataSourceTypeKind::S3GlueDataCatalog { role_arn } => model::DataSourceType {
            s3_glue_data_catalog: Some(model::S3GlueDataCatalog {
                role_arn: role_arn.clone(),
            }),
        },
    }
}

fn parse_data_source_type(body: &Value) -> DataSourceTypeKind {
    let role_arn = body
        .get("DataSourceType")
        .and_then(|t| t.get("S3GlueDataCatalog"))
        .and_then(|s| s.get("RoleArn"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    DataSourceTypeKind::S3GlueDataCatalog { role_arn }
}

fn direct_query_data_source_type_to_model(
    t: &DirectQueryDataSourceTypeKind,
) -> model::DirectQueryDataSourceType {
    match t {
        DirectQueryDataSourceTypeKind::CloudWatchLog { role_arn } => {
            model::DirectQueryDataSourceType {
                cloud_watch_log: Some(model::CloudWatchDirectQueryDataSource {
                    role_arn: role_arn.clone(),
                }),
                security_lake: None,
            }
        }
        DirectQueryDataSourceTypeKind::SecurityLake { role_arn } => {
            model::DirectQueryDataSourceType {
                cloud_watch_log: None,
                security_lake: Some(model::SecurityLakeDirectQueryDataSource {
                    role_arn: role_arn.clone(),
                }),
            }
        }
    }
}

fn parse_direct_query_data_source_type(body: &Value) -> DirectQueryDataSourceTypeKind {
    let ds_type = body.get("DataSourceType");
    if let Some(cw) = ds_type.and_then(|t| t.get("CloudWatchLog")) {
        let role_arn = cw
            .get("RoleArn")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        DirectQueryDataSourceTypeKind::CloudWatchLog { role_arn }
    } else {
        let role_arn = ds_type
            .and_then(|t| t.get("SecurityLake"))
            .and_then(|s| s.get("RoleArn"))
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        DirectQueryDataSourceTypeKind::SecurityLake { role_arn }
    }
}

fn package_to_model(pkg: &crate::types::Package) -> model::PackageDetails {
    model::PackageDetails {
        package_i_d: Some(pkg.package_id.clone()),
        package_name: Some(pkg.package_name.clone()),
        package_type: Some(pkg.package_type.clone()),
        package_description: pkg.package_description.clone(),
        engine_version: pkg.engine_version.clone(),
        package_status: Some(pkg.status.clone()),
        created_at: Some(pkg.created_at),
        last_updated_at: Some(pkg.last_updated_at),
        ..Default::default()
    }
}

fn domain_package_details(
    pkg: &crate::types::Package,
    domain_name: &str,
) -> model::DomainPackageDetails {
    model::DomainPackageDetails {
        package_i_d: Some(pkg.package_id.clone()),
        package_name: Some(pkg.package_name.clone()),
        package_type: Some(pkg.package_type.clone()),
        domain_name: Some(domain_name.to_string()),
        domain_package_status: Some("ACTIVE".to_string()),
        ..Default::default()
    }
}

fn outbound_connection_to_model(c: &crate::types::OutboundConnection) -> model::OutboundConnection {
    model::OutboundConnection {
        connection_id: Some(c.connection_id.clone()),
        connection_alias: Some(c.connection_alias.clone()),
        connection_mode: c.connection_mode.clone(),
        connection_status: Some(model::OutboundConnectionStatus {
            status_code: Some(c.status_code.clone()),
            message: None,
        }),
        local_domain_info: Some(make_domain_info_container(
            &c.local_domain_name,
            Some(&c.local_owner_id),
            c.local_region.as_deref(),
        )),
        remote_domain_info: Some(make_domain_info_container(
            &c.remote_domain_name,
            c.remote_owner_id.as_deref(),
            c.remote_region.as_deref(),
        )),
        connection_properties: None,
    }
}

fn inbound_connection_to_model(c: &crate::types::InboundConnection) -> model::InboundConnection {
    model::InboundConnection {
        connection_id: Some(c.connection_id.clone()),
        connection_mode: c.connection_mode.clone(),
        connection_status: Some(model::InboundConnectionStatus {
            status_code: Some(c.status_code.clone()),
            message: None,
        }),
        local_domain_info: Some(make_domain_info_container(
            &c.local_domain_name,
            Some(&c.local_owner_id),
            c.local_region.as_deref(),
        )),
        remote_domain_info: Some(make_domain_info_container(
            &c.remote_domain_name,
            c.remote_owner_id.as_deref(),
            c.remote_region.as_deref(),
        )),
    }
}

fn make_domain_info_container(
    domain_name: &str,
    owner_id: Option<&str>,
    region: Option<&str>,
) -> model::DomainInformationContainer {
    model::DomainInformationContainer {
        a_w_s_domain_information: Some(model::AWSDomainInformation {
            domain_name: domain_name.to_string(),
            owner_id: owner_id.map(|s| s.to_string()),
            region: region.map(|s| s.to_string()),
        }),
    }
}

fn reserved_instance_to_model(ri: &crate::types::ReservedInstance) -> model::ReservedInstance {
    model::ReservedInstance {
        reserved_instance_id: Some(ri.reserved_instance_id.clone()),
        reservation_name: Some(ri.reservation_name.clone()),
        reserved_instance_offering_id: Some(ri.reserved_instance_offering_id.clone()),
        instance_type: Some(ri.instance_type.clone()),
        instance_count: Some(ri.instance_count),
        duration: Some(ri.duration),
        fixed_price: Some(ri.fixed_price),
        usage_price: Some(ri.usage_price),
        currency_code: Some(ri.currency_code.clone()),
        payment_option: Some(ri.payment_option.clone()),
        state: Some(ri.state.clone()),
        ..Default::default()
    }
}

/// Derive engine type from engine version string.
/// "OpenSearch_2.5" -> "OpenSearch", "Elasticsearch_7.10" -> "Elasticsearch"
fn engine_type_from_version(engine_version: &str) -> String {
    if engine_version.starts_with("Elasticsearch") {
        "Elasticsearch".to_string()
    } else {
        "OpenSearch".to_string()
    }
}

fn extract_path_and_query(uri: &str) -> (String, String) {
    let after_scheme = if let Some(idx) = uri.find("://") {
        &uri[idx + 3..]
    } else {
        uri
    };
    let path_start = after_scheme.find('/').unwrap_or(after_scheme.len());
    let path_and_query = &after_scheme[path_start..];
    match path_and_query.find('?') {
        Some(q) => (
            path_and_query[..q].to_string(),
            path_and_query[q + 1..].to_string(),
        ),
        None => (path_and_query.to_string(), String::new()),
    }
}

fn extract_query_param(query_string: &str, param_name: &str) -> Option<String> {
    if query_string.is_empty() {
        return None;
    }
    for part in query_string.split('&') {
        if let Some((key, value)) = part.split_once('=')
            && key == param_name
        {
            // URL-decode the value (basic: handle %3A for colons, %2F for slashes)
            let decoded = value
                .replace("%3A", ":")
                .replace("%3a", ":")
                .replace("%2F", "/")
                .replace("%2f", "/");
            return Some(decoded);
        }
    }
    None
}

fn opensearch_error_response(err: &OpenSearchError) -> MockResponse {
    let (status, error_type) = match err {
        OpenSearchError::DomainAlreadyExists { .. } => (409, "ResourceAlreadyExistsException"),
        OpenSearchError::DomainNotFound { .. } => (409, "ResourceNotFoundException"),
        OpenSearchError::VpcEndpointNotFound { .. } => (409, "ResourceNotFoundException"),
        OpenSearchError::MissingPrincipalIdentifier => (400, "ValidationException"),
        OpenSearchError::DataSourceAlreadyExists { .. } => (409, "ResourceAlreadyExistsException"),
        OpenSearchError::DataSourceNotFound { .. } => (409, "ResourceNotFoundException"),
        OpenSearchError::DirectQueryDataSourceAlreadyExists { .. } => {
            (409, "ResourceAlreadyExistsException")
        }
        OpenSearchError::DirectQueryDataSourceNotFound { .. } => (409, "ResourceNotFoundException"),
        OpenSearchError::PackageAlreadyExists { .. } => (409, "ResourceAlreadyExistsException"),
        OpenSearchError::PackageNotFound { .. } => (409, "ResourceNotFoundException"),
        OpenSearchError::OutboundConnectionNotFound { .. } => (409, "ResourceNotFoundException"),
        OpenSearchError::InboundConnectionNotFound { .. } => (409, "ResourceNotFoundException"),
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
