use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    extract_path, extract_query_string, percent_decode, rest_json_error,
};

use crate::state::{MediaConnectError, MediaConnectState};
use crate::types::{Flow, FlowOutput, FlowSource, FlowVpcInterface};
use crate::views::MediaConnectStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct MediaConnectService {
    pub(crate) state: Arc<BackendState<MediaConnectState>>,
    pub(crate) notifier: StateChangeNotifier<MediaConnectStateView>,
}

impl MediaConnectService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for MediaConnectService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for MediaConnectService {
    fn service_name(&self) -> &str {
        "mediaconnect"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://mediaconnect\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl MediaConnectService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let raw_query = extract_query_string(&request.uri);
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(raw_query);
        let method = request.method.as_str();

        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        // Handle /tags/{resourceArn+} routes
        // GET    /tags/{ResourceArn} - ListTagsForResource
        // POST   /tags/{ResourceArn} - TagResource
        // DELETE /tags/{ResourceArn} - UntagResource
        if !segments.is_empty() && segments[0] == "tags" && segments.len() >= 2 {
            let resource_arn = percent_decode(&segments[1..].join("/"));
            let labels: &[(&str, &str)] = &[("ResourceArn", resource_arn.as_str())];
            return match method {
                "GET" => {
                    self.handle_list_tags_for_resource(&state, &request, labels, &query_map)
                        .await
                }
                "POST" => {
                    self.handle_tag_resource(&state, &request, labels, &query_map)
                        .await
                }
                "DELETE" => {
                    self.handle_untag_resource(&state, &request, labels, &query_map)
                        .await
                }
                _ => rest_json_error(404, "NotFoundException", "Not found"),
            };
        }

        if segments.len() < 2 || segments[0] != "v1" || segments[1] != "flows" {
            return rest_json_error(404, "NotFoundException", "Not found");
        }

        // Routes under /v1/flows/...
        match (method, segments.as_slice()) {
            // POST /v1/flows - CreateFlow
            ("POST", ["v1", "flows"]) => {
                self.handle_create_flow(&state, &request, &[], &query_map, account_id, &region)
                    .await
            }
            // GET /v1/flows - ListFlows
            ("GET", ["v1", "flows"]) => {
                self.handle_list_flows(&state, &request, &[], &query_map)
                    .await
            }

            // POST /v1/flows/start/{flowArn...} - StartFlow
            ("POST", ["v1", "flows", "start", rest @ ..]) if !rest.is_empty() => {
                let arn_part = rest.join("/");
                let flow_arn = percent_decode(&arn_part);
                let labels: &[(&str, &str)] = &[("FlowArn", flow_arn.as_str())];
                self.handle_start_flow(&state, &request, labels, &query_map)
                    .await
            }
            // POST /v1/flows/stop/{flowArn...} - StopFlow
            ("POST", ["v1", "flows", "stop", rest @ ..]) if !rest.is_empty() => {
                let arn_part = rest.join("/");
                let flow_arn = percent_decode(&arn_part);
                let labels: &[(&str, &str)] = &[("FlowArn", flow_arn.as_str())];
                self.handle_stop_flow(&state, &request, labels, &query_map)
                    .await
            }

            // POST /v1/flows/{flowArn...}/outputs - AddFlowOutputs
            ("POST", _) if segments.len() >= 4 && *segments.last().unwrap() == "outputs" => {
                let arn_part = segments[2..segments.len() - 1].join("/");
                let flow_arn = percent_decode(&arn_part);
                let labels: &[(&str, &str)] = &[("FlowArn", flow_arn.as_str())];
                self.handle_add_flow_outputs(
                    &state, &request, labels, &query_map, account_id, &region,
                )
                .await
            }
            // POST /v1/flows/{flowArn...}/source - AddFlowSources
            ("POST", _) if segments.len() >= 4 && *segments.last().unwrap() == "source" => {
                let arn_part = segments[2..segments.len() - 1].join("/");
                let flow_arn = percent_decode(&arn_part);
                let labels: &[(&str, &str)] = &[("FlowArn", flow_arn.as_str())];
                self.handle_add_flow_sources(
                    &state, &request, labels, &query_map, account_id, &region,
                )
                .await
            }
            // POST /v1/flows/{flowArn...}/vpcInterfaces - AddFlowVpcInterfaces
            ("POST", _) if segments.len() >= 4 && *segments.last().unwrap() == "vpcInterfaces" => {
                let arn_part = segments[2..segments.len() - 1].join("/");
                let flow_arn = percent_decode(&arn_part);
                let labels: &[(&str, &str)] = &[("FlowArn", flow_arn.as_str())];
                self.handle_add_flow_vpc_interfaces(&state, &request, labels, &query_map)
                    .await
            }

            // POST /v1/flows/{flowArn...}/entitlements - GrantFlowEntitlements
            ("POST", _) if segments.len() >= 4 && *segments.last().unwrap() == "entitlements" => {
                let arn_part = segments[2..segments.len() - 1].join("/");
                let flow_arn = percent_decode(&arn_part);
                let labels: &[(&str, &str)] = &[("FlowArn", flow_arn.as_str())];
                self.handle_grant_flow_entitlements(
                    &state, &request, labels, &query_map, account_id, &region,
                )
                .await
            }

            // PUT /v1/flows/{flowArn...}/outputs/{outputArn...} - UpdateFlowOutput
            ("PUT", _) if segments.len() >= 5 && contains_segment(&segments, "outputs") => {
                let (flow_arn, output_arn) = split_at_segment(&segments[2..], "outputs");
                let labels: &[(&str, &str)] = &[
                    ("FlowArn", flow_arn.as_str()),
                    ("OutputArn", output_arn.as_str()),
                ];
                self.handle_update_flow_output(&state, &request, labels, &query_map)
                    .await
            }
            // PUT /v1/flows/{flowArn...}/entitlements/{entitlementArn...} - UpdateFlowEntitlement
            ("PUT", _) if segments.len() >= 5 && contains_segment(&segments, "entitlements") => {
                let (flow_arn, entitlement_arn) = split_at_segment(&segments[2..], "entitlements");
                let labels: &[(&str, &str)] = &[
                    ("FlowArn", flow_arn.as_str()),
                    ("EntitlementArn", entitlement_arn.as_str()),
                ];
                self.handle_update_flow_entitlement(&state, &request, labels, &query_map)
                    .await
            }
            // PUT /v1/flows/{flowArn...}/source/{sourceArn...} - UpdateFlowSource
            ("PUT", _) if segments.len() >= 5 && contains_segment(&segments, "source") => {
                let (flow_arn, source_arn) = split_at_segment(&segments[2..], "source");
                let labels: &[(&str, &str)] = &[
                    ("FlowArn", flow_arn.as_str()),
                    ("SourceArn", source_arn.as_str()),
                ];
                self.handle_update_flow_source(&state, &request, labels, &query_map)
                    .await
            }
            // PUT /v1/flows/{flowArn...} - UpdateFlow
            ("PUT", _) if segments.len() >= 3 => {
                let arn_part = segments[2..].join("/");
                let flow_arn = percent_decode(&arn_part);
                let labels: &[(&str, &str)] = &[("FlowArn", flow_arn.as_str())];
                self.handle_update_flow(&state, &request, labels, &query_map)
                    .await
            }

            // DELETE /v1/flows/{flowArn...}/outputs/{outputArn...} - RemoveFlowOutput
            ("DELETE", _) if segments.len() >= 5 && contains_segment(&segments, "outputs") => {
                let (flow_arn, output_arn) = split_at_segment(&segments[2..], "outputs");
                let labels: &[(&str, &str)] = &[
                    ("FlowArn", flow_arn.as_str()),
                    ("OutputArn", output_arn.as_str()),
                ];
                self.handle_remove_flow_output(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /v1/flows/{flowArn...}/entitlements/{entitlementArn...} - RevokeFlowEntitlement
            ("DELETE", _) if segments.len() >= 5 && contains_segment(&segments, "entitlements") => {
                let (flow_arn, entitlement_arn) = split_at_segment(&segments[2..], "entitlements");
                let labels: &[(&str, &str)] = &[
                    ("FlowArn", flow_arn.as_str()),
                    ("EntitlementArn", entitlement_arn.as_str()),
                ];
                self.handle_revoke_flow_entitlement(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /v1/flows/{flowArn...}/source/{sourceArn...} - RemoveFlowSource
            ("DELETE", _) if segments.len() >= 5 && contains_segment(&segments, "source") => {
                let (flow_arn, source_arn) = split_at_segment(&segments[2..], "source");
                let labels: &[(&str, &str)] = &[
                    ("FlowArn", flow_arn.as_str()),
                    ("SourceArn", source_arn.as_str()),
                ];
                self.handle_remove_flow_source(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /v1/flows/{flowArn...}/vpcInterfaces/{vpcInterfaceName} - RemoveFlowVpcInterface
            ("DELETE", _)
                if segments.len() >= 5 && contains_segment(&segments, "vpcInterfaces") =>
            {
                let (flow_arn, vpc_interface_name) =
                    split_at_segment(&segments[2..], "vpcInterfaces");
                let labels: &[(&str, &str)] = &[
                    ("FlowArn", flow_arn.as_str()),
                    ("VpcInterfaceName", vpc_interface_name.as_str()),
                ];
                self.handle_remove_flow_vpc_interface(&state, &request, labels, &query_map)
                    .await
            }

            // GET /v1/flows/{flowArn...} - DescribeFlow
            ("GET", _) if segments.len() >= 3 => {
                let arn_part = segments[2..].join("/");
                let flow_arn = percent_decode(&arn_part);
                let labels: &[(&str, &str)] = &[("FlowArn", flow_arn.as_str())];
                self.handle_describe_flow(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /v1/flows/{flowArn...} - DeleteFlow
            ("DELETE", _) if segments.len() >= 3 => {
                let arn_part = segments[2..].join("/");
                let flow_arn = percent_decode(&arn_part);
                let labels: &[(&str, &str)] = &[("FlowArn", flow_arn.as_str())];
                self.handle_delete_flow(&state, &request, labels, &query_map)
                    .await
            }
            _ => rest_json_error(404, "NotFoundException", "Not found"),
        }
    }

    async fn handle_create_flow(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_flow_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'name'");
        }
        let availability_zone = input.availability_zone.as_deref().unwrap_or("");
        let description = ""; // Smithy CreateFlowRequest has no top-level description field
        let mut state = state.write().await;
        match state.create_flow(
            &input.name,
            availability_zone,
            description,
            account_id,
            region,
        ) {
            Ok(flow) => wire::serialize_create_flow_response(&wire::CreateFlowResponse {
                flow: Some(flow_to_detail(flow)),
            }),
            Err(e) => mediaconnect_error_response(&e),
        }
    }

    async fn handle_describe_flow(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_flow_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.describe_flow(&input.flow_arn) {
            Ok(flow) => wire::serialize_describe_flow_response(&wire::DescribeFlowResponse {
                flow: Some(flow_to_detail(flow)),
                messages: Some(wire::Messages::default()),
            }),
            Err(e) => mediaconnect_error_response(&e),
        }
    }

    async fn handle_delete_flow(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_flow_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.delete_flow(&input.flow_arn) {
            Ok(flow) => wire::serialize_delete_flow_response(&wire::DeleteFlowResponse {
                flow_arn: Some(flow.flow_arn),
                status: Some("DELETING".to_string()),
            }),
            Err(e) => mediaconnect_error_response(&e),
        }
    }

    async fn handle_list_flows(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_flows_request(request, labels, query) {
            return rest_json_error(400, "BadRequestException", &e);
        }
        let state = state.read().await;
        let flows = state.list_flows();
        let entries: Vec<wire::ListedFlow> = flows
            .iter()
            .map(|f| wire::ListedFlow {
                flow_arn: Some(f.flow_arn.clone()),
                name: Some(f.name.clone()),
                description: Some(f.description.clone()),
                availability_zone: Some(f.availability_zone.clone()),
                status: Some(f.status.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_flows_response(&wire::ListFlowsResponse {
            flows: Some(entries),
            next_token: None,
        })
    }

    async fn handle_start_flow(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_start_flow_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.start_flow(&input.flow_arn) {
            Ok(flow) => wire::serialize_start_flow_response(&wire::StartFlowResponse {
                flow_arn: Some(flow.flow_arn.clone()),
                status: Some(flow.status.clone()),
            }),
            Err(e) => mediaconnect_error_response(&e),
        }
    }

    async fn handle_stop_flow(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_stop_flow_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.stop_flow(&input.flow_arn) {
            Ok(flow) => wire::serialize_stop_flow_response(&wire::StopFlowResponse {
                flow_arn: Some(flow.flow_arn.clone()),
                status: Some(flow.status.clone()),
            }),
            Err(e) => mediaconnect_error_response(&e),
        }
    }

    async fn handle_update_flow(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_update_flow_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        // The Smithy UpdateFlowRequest does not have a description field.
        // Preserve previous behaviour by parsing the body for "description".
        let description: Option<String> = if !request.body.is_empty() {
            serde_json::from_slice::<serde_json::Value>(&request.body)
                .ok()
                .and_then(|v| {
                    v.get("description")
                        .and_then(|d| d.as_str())
                        .map(|s| s.to_string())
                })
        } else {
            None
        };
        let mut state = state.write().await;
        match state.update_flow(&_input.flow_arn, description.as_deref()) {
            Ok(flow) => wire::serialize_update_flow_response(&wire::UpdateFlowResponse {
                flow: Some(flow_to_detail(flow)),
            }),
            Err(e) => mediaconnect_error_response(&e),
        }
    }

    async fn handle_add_flow_outputs(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_add_flow_outputs_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let outputs: Vec<FlowOutput> = input
            .outputs
            .into_iter()
            .map(|o| {
                let name = o.name.unwrap_or_else(|| "output".to_string());
                let output_arn = format!(
                    "arn:aws:mediaconnect:{region}:{account_id}:output:{id}:{name}",
                    id = uuid::Uuid::new_v4(),
                );
                FlowOutput {
                    output_arn,
                    name,
                    description: o.description.unwrap_or_default(),
                    port: o.port.unwrap_or(0),
                    protocol: o.protocol.unwrap_or_else(|| "srt-listener".to_string()),
                    destination: o.destination.unwrap_or_default(),
                }
            })
            .collect();

        let mut state = state.write().await;
        match state.add_flow_outputs(&input.flow_arn, outputs) {
            Ok((_flow, added)) => {
                let wire_outputs: Vec<wire::Output> = added.iter().map(output_to_wire).collect();
                wire::serialize_add_flow_outputs_response(&wire::AddFlowOutputsResponse {
                    flow_arn: Some(input.flow_arn.clone()),
                    outputs: Some(wire_outputs),
                })
            }
            Err(e) => mediaconnect_error_response(&e),
        }
    }

    async fn handle_remove_flow_output(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_remove_flow_output_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.remove_flow_output(&input.flow_arn, &input.output_arn) {
            Ok(_flow) => {
                wire::serialize_remove_flow_output_response(&wire::RemoveFlowOutputResponse {
                    flow_arn: Some(input.flow_arn.clone()),
                    output_arn: Some(input.output_arn.clone()),
                })
            }
            Err(e) => mediaconnect_error_response(&e),
        }
    }

    async fn handle_update_flow_output(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_flow_output_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let description = input.description.as_deref();
        let port = input.port;
        let protocol = input.protocol.as_deref();
        let destination = input.destination.as_deref();

        let mut state = state.write().await;
        match state.update_flow_output(
            &input.flow_arn,
            &input.output_arn,
            description,
            port,
            protocol,
            destination,
        ) {
            Ok((_flow, output)) => {
                wire::serialize_update_flow_output_response(&wire::UpdateFlowOutputResponse {
                    flow_arn: Some(input.flow_arn.clone()),
                    output: Some(output_to_wire(output)),
                })
            }
            Err(e) => mediaconnect_error_response(&e),
        }
    }

    async fn handle_add_flow_sources(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_add_flow_sources_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        // Re-parse body for whitelistCidr — Smithy SetSourceRequest does not include it.
        let body_val: serde_json::Value = if request.body.is_empty() {
            serde_json::Value::Null
        } else {
            serde_json::from_slice(&request.body).unwrap_or(serde_json::Value::Null)
        };
        let whitelist_cidrs: Vec<String> = body_val
            .get("sources")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|s| {
                        s.get("whitelistCidr")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string()
                    })
                    .collect()
            })
            .unwrap_or_default();

        let sources: Vec<FlowSource> = input
            .sources
            .into_iter()
            .enumerate()
            .map(|(i, s)| {
                let name = s.name.unwrap_or_else(|| "source".to_string());
                let source_arn = format!(
                    "arn:aws:mediaconnect:{region}:{account_id}:source:{id}:{name}",
                    id = uuid::Uuid::new_v4(),
                );
                FlowSource {
                    source_arn,
                    name,
                    description: s.description.unwrap_or_default(),
                    ingest_port: s.ingest_port.unwrap_or(0),
                    protocol: s.protocol.unwrap_or_else(|| "srt-listener".to_string()),
                    whitelist_cidr: whitelist_cidrs.get(i).cloned().unwrap_or_default(),
                }
            })
            .collect();

        let mut state = state.write().await;
        match state.add_flow_sources(&input.flow_arn, sources) {
            Ok((_flow, added)) => {
                let wire_sources: Vec<wire::Source> = added.iter().map(source_to_wire).collect();
                wire::serialize_add_flow_sources_response(&wire::AddFlowSourcesResponse {
                    flow_arn: Some(input.flow_arn.clone()),
                    sources: Some(wire_sources),
                })
            }
            Err(e) => mediaconnect_error_response(&e),
        }
    }

    async fn handle_remove_flow_source(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_remove_flow_source_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.remove_flow_source(&input.flow_arn, &input.source_arn) {
            Ok(_flow) => {
                wire::serialize_remove_flow_source_response(&wire::RemoveFlowSourceResponse {
                    flow_arn: Some(input.flow_arn.clone()),
                    source_arn: Some(input.source_arn.clone()),
                })
            }
            Err(e) => mediaconnect_error_response(&e),
        }
    }

    async fn handle_update_flow_source(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_flow_source_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        // Smithy UpdateFlowSourceRequest has no whitelistCidr field — preserve via body parse.
        let whitelist_cidr_owned: Option<String> = if !request.body.is_empty() {
            serde_json::from_slice::<serde_json::Value>(&request.body)
                .ok()
                .and_then(|v| {
                    v.get("whitelistCidr")
                        .and_then(|s| s.as_str())
                        .map(|s| s.to_string())
                })
        } else {
            None
        };

        let mut state = state.write().await;
        match state.update_flow_source(
            &input.flow_arn,
            &input.source_arn,
            input.description.as_deref(),
            input.protocol.as_deref(),
            whitelist_cidr_owned.as_deref(),
        ) {
            Ok((_flow, source)) => {
                wire::serialize_update_flow_source_response(&wire::UpdateFlowSourceResponse {
                    flow_arn: Some(input.flow_arn.clone()),
                    source: Some(source_to_wire(source)),
                })
            }
            Err(e) => mediaconnect_error_response(&e),
        }
    }

    async fn handle_add_flow_vpc_interfaces(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_add_flow_vpc_interfaces_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let vpc_interfaces: Vec<FlowVpcInterface> = input
            .vpc_interfaces
            .into_iter()
            .map(|v| {
                let name = if v.name.is_empty() {
                    "vpc-interface".to_string()
                } else {
                    v.name
                };
                FlowVpcInterface {
                    name,
                    role_arn: v.role_arn,
                    security_group_ids: v.security_group_ids,
                    subnet_id: v.subnet_id,
                    network_interface_type: v
                        .network_interface_type
                        .unwrap_or_else(|| "ena".to_string()),
                    network_interface_ids: vec![format!("eni-{}", uuid::Uuid::new_v4())],
                }
            })
            .collect();

        let mut state = state.write().await;
        match state.add_flow_vpc_interfaces(&input.flow_arn, vpc_interfaces) {
            Ok((_flow, added)) => {
                let wire_vpcs: Vec<wire::VpcInterface> =
                    added.iter().map(vpc_interface_to_wire).collect();
                wire::serialize_add_flow_vpc_interfaces_response(
                    &wire::AddFlowVpcInterfacesResponse {
                        flow_arn: Some(input.flow_arn.clone()),
                        vpc_interfaces: Some(wire_vpcs),
                    },
                )
            }
            Err(e) => mediaconnect_error_response(&e),
        }
    }

    async fn handle_remove_flow_vpc_interface(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_remove_flow_vpc_interface_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "BadRequestException", &e),
            };
        let mut state = state.write().await;
        match state.remove_flow_vpc_interface(&input.flow_arn, &input.vpc_interface_name) {
            Ok(_flow) => wire::serialize_remove_flow_vpc_interface_response(
                &wire::RemoveFlowVpcInterfaceResponse {
                    flow_arn: Some(input.flow_arn.clone()),
                    vpc_interface_name: Some(input.vpc_interface_name.clone()),
                    non_deleted_network_interface_ids: Some(vec![]),
                },
            ),
            Err(e) => mediaconnect_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        let tags = state.list_tags_for_resource(&input.resource_arn);
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: Some(tags),
        })
    }

    async fn handle_grant_flow_entitlements(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_grant_flow_entitlements_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };

        let parsed: Vec<(String, String, Vec<String>)> = input
            .entitlements
            .into_iter()
            .map(|ent| {
                (
                    ent.name.unwrap_or_default(),
                    ent.description.unwrap_or_default(),
                    ent.subscribers,
                )
            })
            .collect();

        let mut state = state.write().await;
        match state.grant_flow_entitlements(&input.flow_arn, parsed, account_id, region) {
            Ok(ents) => {
                let wire_ents: Vec<wire::Entitlement> = ents.iter().map(ent_to_wire).collect();
                wire::serialize_grant_flow_entitlements_response(
                    &wire::GrantFlowEntitlementsResponse {
                        entitlements: Some(wire_ents),
                        flow_arn: Some(input.flow_arn.clone()),
                    },
                )
            }
            Err(e) => mediaconnect_error_response(&e),
        }
    }

    async fn handle_revoke_flow_entitlement(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_revoke_flow_entitlement_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.revoke_flow_entitlement(&input.flow_arn, &input.entitlement_arn) {
            Ok((flow, ent)) => wire::serialize_revoke_flow_entitlement_response(
                &wire::RevokeFlowEntitlementResponse {
                    flow_arn: Some(flow),
                    entitlement_arn: Some(ent),
                },
            ),
            Err(e) => mediaconnect_error_response(&e),
        }
    }

    async fn handle_update_flow_entitlement(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_flow_entitlement_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let description = input.description.as_deref();
        let subscribers = input.subscribers;

        let mut state = state.write().await;
        match state.update_flow_entitlement(
            &input.flow_arn,
            &input.entitlement_arn,
            description,
            subscribers,
        ) {
            Ok(ent) => wire::serialize_update_flow_entitlement_response(
                &wire::UpdateFlowEntitlementResponse {
                    entitlement: Some(ent_to_wire(&ent)),
                    flow_arn: Some(input.flow_arn.clone()),
                },
            ),
            Err(e) => mediaconnect_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        state.tag_resource(&input.resource_arn, input.tags);
        wire::serialize_tag_resource_response()
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaConnectState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        state.untag_resource(&input.resource_arn, &input.tag_keys);
        wire::serialize_untag_resource_response()
    }
}

fn ent_to_wire(e: &crate::types::FlowEntitlement) -> wire::Entitlement {
    wire::Entitlement {
        entitlement_arn: Some(e.entitlement_arn.clone()),
        name: Some(e.name.clone()),
        description: Some(e.description.clone()),
        subscribers: Some(e.subscribers.clone()),
        ..Default::default()
    }
}

/// Convert a state `Flow` to a wire `Flow` with empty collection defaults.
fn flow_to_detail(flow: &Flow) -> wire::Flow {
    wire::Flow {
        flow_arn: Some(flow.flow_arn.clone()),
        name: Some(flow.name.clone()),
        description: Some(flow.description.clone()),
        availability_zone: Some(flow.availability_zone.clone()),
        status: Some(flow.status.clone()),
        outputs: Some(flow.outputs.iter().map(output_to_wire).collect()),
        sources: Some(flow.sources.iter().map(source_to_wire).collect()),
        entitlements: Some(flow.entitlements.iter().map(ent_to_wire).collect()),
        media_streams: Some(vec![]),
        vpc_interfaces: Some(
            flow.vpc_interfaces
                .iter()
                .map(vpc_interface_to_wire)
                .collect(),
        ),
        ..Default::default()
    }
}

fn output_to_wire(output: &FlowOutput) -> wire::Output {
    wire::Output {
        output_arn: Some(output.output_arn.clone()),
        name: Some(output.name.clone()),
        description: Some(output.description.clone()),
        port: Some(output.port),
        destination: Some(output.destination.clone()),
        ..Default::default()
    }
}

fn source_to_wire(source: &FlowSource) -> wire::Source {
    wire::Source {
        source_arn: Some(source.source_arn.clone()),
        name: Some(source.name.clone()),
        description: Some(source.description.clone()),
        ingest_port: Some(source.ingest_port),
        whitelist_cidr: Some(source.whitelist_cidr.clone()),
        ..Default::default()
    }
}

fn vpc_interface_to_wire(vpc: &FlowVpcInterface) -> wire::VpcInterface {
    wire::VpcInterface {
        name: Some(vpc.name.clone()),
        role_arn: Some(vpc.role_arn.clone()),
        security_group_ids: Some(vpc.security_group_ids.clone()),
        subnet_id: Some(vpc.subnet_id.clone()),
        network_interface_type: Some(vpc.network_interface_type.clone()),
        network_interface_ids: Some(vpc.network_interface_ids.clone()),
    }
}

fn mediaconnect_error_response(err: &MediaConnectError) -> MockResponse {
    let (status, error_type) = match err {
        MediaConnectError::DuplicateFlowName { .. } => (420u16, "CreateFlow420Exception"),
        MediaConnectError::FlowNotFound { .. }
        | MediaConnectError::FlowNotFoundShort { .. }
        | MediaConnectError::OutputNotFound { .. }
        | MediaConnectError::SourceNotFound { .. }
        | MediaConnectError::VpcInterfaceNotFound { .. }
        | MediaConnectError::EntitlementNotFound { .. } => (404u16, "NotFoundException"),
    };
    let body = json!({
        "message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

/// Helper to check if a segment exists in the path segments (after the first 2).
fn contains_segment(segments: &[&str], target: &str) -> bool {
    segments[2..].contains(&target)
}

/// Split segments at a target keyword, returning (left_arn, right_arn) with percent decoding.
fn split_at_segment(segments: &[&str], target: &str) -> (String, String) {
    let pos = segments.iter().position(|&s| s == target).unwrap_or(0);
    let left = percent_decode(&segments[..pos].join("/"));
    let right = percent_decode(&segments[pos + 1..].join("/"));
    (left, right)
}
