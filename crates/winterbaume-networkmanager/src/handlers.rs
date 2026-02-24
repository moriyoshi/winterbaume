use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::state::{NetworkManagerError, NetworkManagerState};
use crate::types::{
    Attachment, ConnectPeer, ConnectPeerAssociation, Connection, CoreNetwork,
    CustomerGatewayAssociation, Device, GlobalNetwork, Link, LinkAssociation, RouteAnalysis, Site,
    TransitGatewayConnectPeerAssociation, TransitGatewayRegistration,
};
use crate::views::NetworkManagerStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct NetworkManagerService {
    pub(crate) state: Arc<BackendState<NetworkManagerState>>,
    pub(crate) notifier: StateChangeNotifier<NetworkManagerStateView>,
}

impl NetworkManagerService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for NetworkManagerService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for NetworkManagerService {
    fn service_name(&self) -> &str {
        "networkmanager"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://networkmanager\..*\.amazonaws\.com",
            r"https?://networkmanager\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl NetworkManagerService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let query = extract_query(&request.uri);
        let method = request.method.as_str();

        let raw_segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();
        let segments: Vec<String> = raw_segments.iter().map(|s| percent_decode(s)).collect();
        let segs: Vec<&str> = segments.iter().map(String::as_str).collect();

        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(&query);

        match (method, segs.as_slice()) {
            // ── Global Networks ──
            ("POST", ["global-networks"]) => {
                self.handle_create_global_network(&state, &request, &[], &query_map, account_id)
                    .await
            }
            ("GET", ["global-networks"]) => self.handle_describe_global_networks(&state).await,
            ("DELETE", ["global-networks", id]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", id)];
                self.handle_delete_global_network(&state, &request, labels, &query_map)
                    .await
            }
            ("PATCH", ["global-networks", id]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", id)];
                self.handle_update_global_network(&state, &request, labels, &query_map)
                    .await
            }

            // ── Core Networks ──
            ("POST", ["core-networks"]) => {
                self.handle_create_core_network(&state, &request, &[], &query_map, account_id)
                    .await
            }
            ("GET", ["core-networks"]) => self.handle_list_core_networks(&state, account_id).await,
            ("GET", ["core-networks", id]) => {
                let labels: &[(&str, &str)] = &[("CoreNetworkId", id)];
                self.handle_get_core_network(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["core-networks", id]) => {
                let labels: &[(&str, &str)] = &[("CoreNetworkId", id)];
                self.handle_delete_core_network(&state, &request, labels, &query_map)
                    .await
            }
            ("PATCH", ["core-networks", id]) => {
                let labels: &[(&str, &str)] = &[("CoreNetworkId", id)];
                self.handle_update_core_network(&state, &request, labels, &query_map)
                    .await
            }

            // ── Sites ──
            ("POST", ["global-networks", gn_id, "sites"]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id)];
                self.handle_create_site(&state, &request, labels, &query_map, account_id)
                    .await
            }
            ("GET", ["global-networks", gn_id, "sites"]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id)];
                self.handle_get_sites(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["global-networks", gn_id, "sites", site_id]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id), ("SiteId", site_id)];
                self.handle_delete_site(&state, &request, labels, &query_map)
                    .await
            }
            ("PATCH", ["global-networks", gn_id, "sites", site_id]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id), ("SiteId", site_id)];
                self.handle_update_site(&state, &request, labels, &query_map)
                    .await
            }

            // ── Links ──
            ("POST", ["global-networks", gn_id, "links"]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id)];
                self.handle_create_link(&state, &request, labels, &query_map, account_id)
                    .await
            }
            ("DELETE", ["global-networks", gn_id, "links", link_id]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id), ("LinkId", link_id)];
                self.handle_delete_link(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["global-networks", gn_id, "links"]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id)];
                self.handle_get_links(&state, &request, labels, &query_map)
                    .await
            }
            ("PATCH", ["global-networks", gn_id, "links", link_id]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id), ("LinkId", link_id)];
                self.handle_update_link(&state, &request, labels, &query_map)
                    .await
            }

            // ── Devices ──
            ("POST", ["global-networks", gn_id, "devices"]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id)];
                self.handle_create_device(&state, &request, labels, &query_map, account_id)
                    .await
            }
            ("GET", ["global-networks", gn_id, "devices"]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id)];
                self.handle_get_devices(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["global-networks", gn_id, "devices", device_id]) => {
                let labels: &[(&str, &str)] =
                    &[("GlobalNetworkId", gn_id), ("DeviceId", device_id)];
                self.handle_delete_device(&state, &request, labels, &query_map)
                    .await
            }
            ("PATCH", ["global-networks", gn_id, "devices", device_id]) => {
                let labels: &[(&str, &str)] =
                    &[("GlobalNetworkId", gn_id), ("DeviceId", device_id)];
                self.handle_update_device(&state, &request, labels, &query_map)
                    .await
            }

            // ── Tags ──
            ("GET", ["tags", rest @ ..]) if !rest.is_empty() => {
                let resource_arn = percent_decode(&rest.join("/"));
                let labels: &[(&str, &str)] = &[("ResourceArn", resource_arn.as_str())];
                self.handle_list_tags_for_resource(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", ["tags", rest @ ..]) if !rest.is_empty() => {
                let resource_arn = percent_decode(&rest.join("/"));
                let labels: &[(&str, &str)] = &[("ResourceArn", resource_arn.as_str())];
                self.handle_tag_resource(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["tags", rest @ ..]) if !rest.is_empty() => {
                let resource_arn = percent_decode(&rest.join("/"));
                let labels: &[(&str, &str)] = &[("ResourceArn", resource_arn.as_str())];
                self.handle_untag_resource(&state, &request, labels, &query_map)
                    .await
            }

            // ── Connections ──
            ("POST", ["global-networks", gn_id, "connections"]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id)];
                self.handle_create_connection(&state, &request, labels, &query_map, account_id)
                    .await
            }
            ("GET", ["global-networks", gn_id, "connections"]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id)];
                self.handle_get_connections(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["global-networks", gn_id, "connections", conn_id]) => {
                let labels: &[(&str, &str)] =
                    &[("GlobalNetworkId", gn_id), ("ConnectionId", conn_id)];
                self.handle_delete_connection(&state, &request, labels, &query_map)
                    .await
            }
            ("PATCH", ["global-networks", gn_id, "connections", conn_id]) => {
                let labels: &[(&str, &str)] =
                    &[("GlobalNetworkId", gn_id), ("ConnectionId", conn_id)];
                self.handle_update_connection(&state, &request, labels, &query_map)
                    .await
            }

            // ── Transit Gateway Registrations ──
            ("POST", ["global-networks", gn_id, "transit-gateway-registrations"]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id)];
                self.handle_register_transit_gateway(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["global-networks", gn_id, "transit-gateway-registrations"]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id)];
                self.handle_get_transit_gateway_registrations(&state, &request, labels, &query_map)
                    .await
            }
            (
                "DELETE",
                [
                    "global-networks",
                    gn_id,
                    "transit-gateway-registrations",
                    rest @ ..,
                ],
            ) if !rest.is_empty() => {
                let tgw_arn = percent_decode(&rest.join("/"));
                let labels: &[(&str, &str)] = &[
                    ("GlobalNetworkId", gn_id),
                    ("TransitGatewayArn", tgw_arn.as_str()),
                ];
                self.handle_deregister_transit_gateway(&state, &request, labels, &query_map)
                    .await
            }

            // ── VPC Attachments ──
            ("POST", ["vpc-attachments"]) => {
                self.handle_create_vpc_attachment(&state, &request, &[], &query_map, account_id)
                    .await
            }
            ("GET", ["vpc-attachments", att_id]) => {
                let labels: &[(&str, &str)] = &[("AttachmentId", att_id)];
                self.handle_get_vpc_attachment(&state, &request, labels, &query_map)
                    .await
            }
            ("PATCH", ["vpc-attachments", att_id]) => {
                let labels: &[(&str, &str)] = &[("AttachmentId", att_id)];
                self.handle_update_vpc_attachment(&state, &request, labels, &query_map)
                    .await
            }

            // ── Generic Attachments ──
            ("DELETE", ["attachments", att_id]) => {
                let labels: &[(&str, &str)] = &[("AttachmentId", att_id)];
                self.handle_delete_attachment(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", ["attachments", att_id, "accept"]) => {
                let labels: &[(&str, &str)] = &[("AttachmentId", att_id)];
                self.handle_accept_attachment(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", ["attachments", att_id, "reject"]) => {
                let labels: &[(&str, &str)] = &[("AttachmentId", att_id)];
                self.handle_reject_attachment(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["attachments"]) => {
                self.handle_list_attachments(&state, &request, &[], &query_map)
                    .await
            }

            // ── Connect Peers ──
            ("POST", ["connect-peers"]) => {
                self.handle_create_connect_peer(&state, &request, &[], &query_map)
                    .await
            }
            ("GET", ["connect-peers"]) => {
                self.handle_list_connect_peers(&state, &request, &[], &query_map)
                    .await
            }
            ("GET", ["connect-peers", cp_id]) => {
                let labels: &[(&str, &str)] = &[("ConnectPeerId", cp_id)];
                self.handle_get_connect_peer(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["connect-peers", cp_id]) => {
                let labels: &[(&str, &str)] = &[("ConnectPeerId", cp_id)];
                self.handle_delete_connect_peer(&state, &request, labels, &query_map)
                    .await
            }

            // ── Connect Peer Associations ──
            ("POST", ["global-networks", gn_id, "connect-peer-associations"]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id)];
                self.handle_associate_connect_peer(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["global-networks", gn_id, "connect-peer-associations"]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id)];
                self.handle_get_connect_peer_associations(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["global-networks", gn_id, "connect-peer-associations", cp_id]) => {
                let labels: &[(&str, &str)] =
                    &[("GlobalNetworkId", gn_id), ("ConnectPeerId", cp_id)];
                self.handle_disassociate_connect_peer(&state, &request, labels, &query_map)
                    .await
            }

            // ── Link Associations ──
            ("POST", ["global-networks", gn_id, "link-associations"]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id)];
                self.handle_associate_link(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["global-networks", gn_id, "link-associations"]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id)];
                self.handle_get_link_associations(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["global-networks", gn_id, "link-associations"]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id)];
                self.handle_disassociate_link(&state, &request, labels, &query_map)
                    .await
            }

            // ── Customer Gateway Associations ──
            ("POST", ["global-networks", gn_id, "customer-gateway-associations"]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id)];
                self.handle_associate_customer_gateway(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["global-networks", gn_id, "customer-gateway-associations"]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id)];
                self.handle_get_customer_gateway_associations(&state, &request, labels, &query_map)
                    .await
            }
            (
                "DELETE",
                [
                    "global-networks",
                    gn_id,
                    "customer-gateway-associations",
                    rest @ ..,
                ],
            ) if !rest.is_empty() => {
                let cgw_arn = percent_decode(&rest.join("/"));
                let labels: &[(&str, &str)] = &[
                    ("GlobalNetworkId", gn_id),
                    ("CustomerGatewayArn", cgw_arn.as_str()),
                ];
                self.handle_disassociate_customer_gateway(&state, &request, labels, &query_map)
                    .await
            }

            // ── Transit Gateway Connect Peer Associations ──
            (
                "POST",
                [
                    "global-networks",
                    gn_id,
                    "transit-gateway-connect-peer-associations",
                ],
            ) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id)];
                self.handle_associate_transit_gateway_connect_peer(
                    &state, &request, labels, &query_map,
                )
                .await
            }
            (
                "GET",
                [
                    "global-networks",
                    gn_id,
                    "transit-gateway-connect-peer-associations",
                ],
            ) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id)];
                self.handle_get_transit_gateway_connect_peer_associations(
                    &state, &request, labels, &query_map,
                )
                .await
            }
            (
                "DELETE",
                [
                    "global-networks",
                    gn_id,
                    "transit-gateway-connect-peer-associations",
                    rest @ ..,
                ],
            ) if !rest.is_empty() => {
                let tgw_cp_arn = percent_decode(&rest.join("/"));
                let labels: &[(&str, &str)] = &[
                    ("GlobalNetworkId", gn_id),
                    ("TransitGatewayConnectPeerArn", tgw_cp_arn.as_str()),
                ];
                self.handle_disassociate_transit_gateway_connect_peer(
                    &state, &request, labels, &query_map,
                )
                .await
            }

            // ── Route Analysis ──
            ("POST", ["global-networks", gn_id, "route-analyses"]) => {
                let labels: &[(&str, &str)] = &[("GlobalNetworkId", gn_id)];
                self.handle_start_route_analysis(&state, &request, labels, &query_map, account_id)
                    .await
            }
            ("GET", ["global-networks", gn_id, "route-analyses", ra_id]) => {
                let labels: &[(&str, &str)] =
                    &[("GlobalNetworkId", gn_id), ("RouteAnalysisId", ra_id)];
                self.handle_get_route_analysis(&state, &request, labels, &query_map)
                    .await
            }

            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ── Global Network handlers ──

    async fn handle_create_global_network(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_global_network_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let description = input.description.as_deref().unwrap_or("");
        let tags = tags_from_input(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_global_network(description, account_id, tags) {
            Ok(network) => {
                let resp = wire::CreateGlobalNetworkResponse {
                    global_network: Some(to_wire_global_network(network)),
                };
                wire::serialize_create_global_network_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_describe_global_networks(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let networks = state.describe_global_networks();
        let entries: Vec<wire::GlobalNetwork> =
            networks.iter().map(|n| to_wire_global_network(n)).collect();
        let resp = wire::DescribeGlobalNetworksResponse {
            global_networks: Some(entries),
            next_token: None,
        };
        wire::serialize_describe_global_networks_response(&resp)
    }

    async fn handle_delete_global_network(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_global_network_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_global_network(&input.global_network_id) {
            Ok(network) => {
                let mut net = network;
                net.state = "DELETING".to_string();
                let resp = wire::DeleteGlobalNetworkResponse {
                    global_network: Some(to_wire_global_network(&net)),
                };
                wire::serialize_delete_global_network_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_update_global_network(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_global_network_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let description = input.description.as_deref();

        let mut state = state.write().await;
        match state.update_global_network(&input.global_network_id, description) {
            Ok(network) => {
                let resp = wire::UpdateGlobalNetworkResponse {
                    global_network: Some(to_wire_global_network(network)),
                };
                wire::serialize_update_global_network_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    // ── Core Network handlers ──

    async fn handle_create_core_network(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_core_network_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.global_network_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'GlobalNetworkId'");
        }
        let description = input.description.as_deref().unwrap_or("");
        let tags = tags_from_input(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_core_network(&input.global_network_id, description, account_id, tags) {
            Ok(cn) => {
                let resp = wire::CreateCoreNetworkResponse {
                    core_network: Some(to_wire_core_network(cn)),
                };
                wire::serialize_create_core_network_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_get_core_network(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_core_network_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_core_network(&input.core_network_id) {
            Ok(cn) => {
                let resp = wire::GetCoreNetworkResponse {
                    core_network: Some(to_wire_core_network(cn)),
                };
                wire::serialize_get_core_network_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_delete_core_network(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_core_network_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_core_network(&input.core_network_id) {
            Ok(mut cn) => {
                cn.state = "DELETING".to_string();
                let resp = wire::DeleteCoreNetworkResponse {
                    core_network: Some(to_wire_core_network(&cn)),
                };
                wire::serialize_delete_core_network_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_update_core_network(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_core_network_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let description = input.description.as_deref();

        let mut state = state.write().await;
        match state.update_core_network(&input.core_network_id, description) {
            Ok(cn) => {
                let resp = wire::UpdateCoreNetworkResponse {
                    core_network: Some(to_wire_core_network(cn)),
                };
                wire::serialize_update_core_network_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_list_core_networks(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        account_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let networks = state.list_core_networks();
        let entries: Vec<wire::CoreNetworkSummary> = networks
            .iter()
            .map(|cn| wire::CoreNetworkSummary {
                core_network_id: Some(cn.core_network_id.clone()),
                core_network_arn: Some(cn.core_network_arn.clone()),
                global_network_id: Some(cn.global_network_id.clone()),
                description: Some(cn.description.clone()),
                owner_account_id: Some(account_id.to_string()),
                state: Some(cn.state.clone()),
                tags: Some(tags_to_wire(&cn.tags)),
                ..Default::default()
            })
            .collect();
        let resp = wire::ListCoreNetworksResponse {
            core_networks: Some(entries),
            next_token: None,
        };
        wire::serialize_list_core_networks_response(&resp)
    }

    // ── Site handlers ──

    async fn handle_create_site(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_site_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let description = input.description.as_deref().unwrap_or("");
        let tags = tags_from_input(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_site(&input.global_network_id, description, account_id, tags) {
            Ok(site) => {
                let resp = wire::CreateSiteResponse {
                    site: Some(to_wire_site(site)),
                };
                wire::serialize_create_site_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_get_sites(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_sites_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let site_ids = input.site_ids.clone().unwrap_or_default();
        let state = state.read().await;
        let sites = state.get_sites(&input.global_network_id);
        let entries: Vec<wire::Site> = sites
            .iter()
            .filter(|s| site_ids.is_empty() || site_ids.contains(&s.site_id))
            .map(|s| to_wire_site(s))
            .collect();
        let resp = wire::GetSitesResponse {
            sites: Some(entries),
            next_token: None,
        };
        wire::serialize_get_sites_response(&resp)
    }

    async fn handle_delete_site(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_site_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_site(&input.site_id) {
            Ok(mut site) => {
                site.state = "DELETING".to_string();
                let resp = wire::DeleteSiteResponse {
                    site: Some(to_wire_site(&site)),
                };
                wire::serialize_delete_site_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_update_site(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_site_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let description = input.description.as_deref();

        let mut state = state.write().await;
        match state.update_site(&input.site_id, description) {
            Ok(site) => {
                let resp = wire::UpdateSiteResponse {
                    site: Some(to_wire_site(site)),
                };
                wire::serialize_update_site_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    // ── Link handlers ──

    async fn handle_create_link(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_link_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.site_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'SiteId'");
        }
        let download_speed = input.bandwidth.download_speed.unwrap_or(0);
        let upload_speed = input.bandwidth.upload_speed.unwrap_or(0);
        let description = input.description.as_deref().unwrap_or("");
        let provider = input.provider.as_deref().unwrap_or("");
        let link_type = input.r#type.as_deref().unwrap_or("");
        let tags = tags_from_input(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_link(
            &input.global_network_id,
            &input.site_id,
            description,
            provider,
            link_type,
            download_speed,
            upload_speed,
            account_id,
            tags,
        ) {
            Ok(link) => {
                let resp = wire::CreateLinkResponse {
                    link: Some(to_wire_link(link)),
                };
                wire::serialize_create_link_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_delete_link(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_link_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_link(&input.link_id) {
            Ok(mut link) => {
                link.state = "DELETING".to_string();
                let resp = wire::DeleteLinkResponse {
                    link: Some(to_wire_link(&link)),
                };
                wire::serialize_delete_link_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    // ── Tag handlers ──

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
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
                    tag_list: Some(tags_to_wire(tags)),
                };
                wire::serialize_list_tags_for_resource_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let tags = tags_from_input(Some(input.tags.as_slice()));

        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, tags) {
            Ok(()) => {
                let resp = wire::TagResourceResponse {};
                wire::serialize_tag_resource_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, &input.tag_keys) {
            Ok(()) => {
                let resp = wire::UntagResourceResponse {};
                wire::serialize_untag_resource_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    // ── GetLinks handler ──

    async fn handle_get_links(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_links_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let link_ids = input.link_ids.clone().unwrap_or_default();
        let state = state.read().await;
        let links = state.get_links(&input.global_network_id);
        let wire_links: Vec<wire::Link> = links
            .iter()
            .filter(|l| link_ids.is_empty() || link_ids.contains(&l.link_id))
            .map(|l| to_wire_link(l))
            .collect();
        wire::serialize_get_links_response(&wire::GetLinksResponse {
            links: if wire_links.is_empty() {
                None
            } else {
                Some(wire_links)
            },
            next_token: None,
        })
    }

    // ── Device handlers ──

    async fn handle_create_device(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_device_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let description = input.description.as_deref().unwrap_or("");
        let site_id = input.site_id.as_deref();
        let model = input.model.as_deref();
        let serial_number = input.serial_number.as_deref();
        let device_type = input.r#type.as_deref();
        let vendor = input.vendor.as_deref();
        let tags = tags_from_input(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_device(
            &input.global_network_id,
            description,
            site_id,
            model,
            serial_number,
            device_type,
            vendor,
            account_id,
            tags,
        ) {
            Ok(device) => {
                let resp = wire::CreateDeviceResponse {
                    device: Some(to_wire_device(device)),
                };
                wire::serialize_create_device_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    // FIX(terraform-e2e): when the provider polls for a specific device after creation it sends
    //   deviceIds=[id] and expects EXACTLY one result; returning all devices in the GN caused
    //   tfresource.FindOneBy to see "too many results" and never mark the device as AVAILABLE.
    async fn handle_get_devices(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_devices_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let device_ids = input.device_ids.clone().unwrap_or_default();
        let state = state.read().await;
        let devices = state.get_devices(&input.global_network_id);
        let wire_devices: Vec<wire::Device> = devices
            .iter()
            .filter(|d| device_ids.is_empty() || device_ids.contains(&d.device_id))
            .map(|d| to_wire_device(d))
            .collect();
        wire::serialize_get_devices_response(&wire::GetDevicesResponse {
            devices: if wire_devices.is_empty() {
                None
            } else {
                Some(wire_devices)
            },
            next_token: None,
        })
    }

    async fn handle_delete_device(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_device_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_device(&input.device_id) {
            Ok(mut device) => {
                device.state = "DELETING".to_string();
                let resp = wire::DeleteDeviceResponse {
                    device: Some(to_wire_device(&device)),
                };
                wire::serialize_delete_device_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    // ── UpdateDevice handler ──

    async fn handle_update_device(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_device_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let description = input.description.as_deref();
        let site_id = input.site_id.as_deref();
        let model = input.model.as_deref();
        let serial_number = input.serial_number.as_deref();
        let device_type = input.r#type.as_deref();
        let vendor = input.vendor.as_deref();

        let mut state = state.write().await;
        match state.update_device(
            &input.device_id,
            description,
            site_id,
            model,
            serial_number,
            device_type,
            vendor,
        ) {
            Ok(device) => {
                let resp = wire::UpdateDeviceResponse {
                    device: Some(to_wire_device(device)),
                };
                wire::serialize_update_device_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    // ── Connection handlers ──

    async fn handle_create_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_connection_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.device_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'DeviceId'");
        }
        if input.connected_device_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'ConnectedDeviceId'");
        }
        let link_id = input.link_id.as_deref();
        let connected_link_id = input.connected_link_id.as_deref();
        let description = input.description.as_deref().unwrap_or("");
        let tags = tags_from_input(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_connection(
            &input.global_network_id,
            &input.device_id,
            &input.connected_device_id,
            link_id,
            connected_link_id,
            description,
            account_id,
            tags,
        ) {
            Ok(conn) => {
                let resp = wire::CreateConnectionResponse {
                    connection: Some(to_wire_connection(conn)),
                };
                wire::serialize_create_connection_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_get_connections(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_connections_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let connection_ids = input.connection_ids.clone().unwrap_or_default();
        let state = state.read().await;
        let connections = state.get_connections(&input.global_network_id);
        let wire_conns: Vec<wire::Connection> = connections
            .iter()
            .filter(|c| connection_ids.is_empty() || connection_ids.contains(&c.connection_id))
            .map(|c| to_wire_connection(c))
            .collect();
        wire::serialize_get_connections_response(&wire::GetConnectionsResponse {
            connections: if wire_conns.is_empty() {
                None
            } else {
                Some(wire_conns)
            },
            next_token: None,
        })
    }

    async fn handle_delete_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_connection_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_connection(&input.connection_id) {
            Ok(mut conn) => {
                conn.state = "DELETING".to_string();
                let resp = wire::DeleteConnectionResponse {
                    connection: Some(to_wire_connection(&conn)),
                };
                wire::serialize_delete_connection_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_update_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_connection_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let description = input.description.as_deref();
        let link_id = input.link_id.as_deref();
        let connected_link_id = input.connected_link_id.as_deref();

        let mut state = state.write().await;
        match state.update_connection(
            &input.connection_id,
            description,
            link_id,
            connected_link_id,
        ) {
            Ok(conn) => {
                let resp = wire::UpdateConnectionResponse {
                    connection: Some(to_wire_connection(conn)),
                };
                wire::serialize_update_connection_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    // ── UpdateLink handler ──

    async fn handle_update_link(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_link_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let description = input.description.as_deref();
        let provider = input.provider.as_deref();
        let link_type = input.r#type.as_deref();
        let download_speed = input.bandwidth.as_ref().and_then(|b| b.download_speed);
        let upload_speed = input.bandwidth.as_ref().and_then(|b| b.upload_speed);

        let mut state = state.write().await;
        match state.update_link(
            &input.link_id,
            description,
            provider,
            link_type,
            download_speed,
            upload_speed,
        ) {
            Ok(link) => {
                let resp = wire::UpdateLinkResponse {
                    link: Some(to_wire_link(link)),
                };
                wire::serialize_update_link_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    // ── Transit Gateway Registration handlers ──

    async fn handle_register_transit_gateway(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_register_transit_gateway_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.transit_gateway_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'TransitGatewayArn'");
        }

        let mut state = state.write().await;
        match state.register_transit_gateway(&input.global_network_id, &input.transit_gateway_arn) {
            Ok(reg) => {
                let resp = wire::RegisterTransitGatewayResponse {
                    transit_gateway_registration: Some(to_wire_tgw_registration(reg)),
                };
                wire::serialize_register_transit_gateway_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_deregister_transit_gateway(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_deregister_transit_gateway_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.deregister_transit_gateway(&input.global_network_id, &input.transit_gateway_arn)
        {
            Ok(mut reg) => {
                reg.state = "DELETING".to_string();
                let resp = wire::DeregisterTransitGatewayResponse {
                    transit_gateway_registration: Some(to_wire_tgw_registration(&reg)),
                };
                wire::serialize_deregister_transit_gateway_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_get_transit_gateway_registrations(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_transit_gateway_registrations_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let regs = state.get_transit_gateway_registrations(&input.global_network_id);
        let wire_regs: Vec<wire::TransitGatewayRegistration> =
            regs.iter().map(|r| to_wire_tgw_registration(r)).collect();
        wire::serialize_get_transit_gateway_registrations_response(
            &wire::GetTransitGatewayRegistrationsResponse {
                transit_gateway_registrations: if wire_regs.is_empty() {
                    None
                } else {
                    Some(wire_regs)
                },
                next_token: None,
            },
        )
    }

    // ── VPC Attachment handlers ──

    async fn handle_create_vpc_attachment(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_vpc_attachment_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.core_network_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'CoreNetworkId'");
        }
        if input.vpc_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'VpcArn'");
        }
        let subnet_arns = input.subnet_arns.clone();
        let tags = tags_from_input(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_vpc_attachment(
            &input.core_network_id,
            &input.vpc_arn,
            subnet_arns,
            account_id,
            tags,
        ) {
            Ok(att) => {
                let resp = wire::CreateVpcAttachmentResponse {
                    vpc_attachment: Some(to_wire_vpc_attachment(att)),
                };
                wire::serialize_create_vpc_attachment_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_get_vpc_attachment(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_vpc_attachment_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_attachment(&input.attachment_id) {
            Ok(att) => {
                let resp = wire::GetVpcAttachmentResponse {
                    vpc_attachment: Some(to_wire_vpc_attachment(att)),
                };
                wire::serialize_get_vpc_attachment_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_update_vpc_attachment(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_vpc_attachment_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_attachment(&input.attachment_id) {
            Ok(att) => {
                let resp = wire::UpdateVpcAttachmentResponse {
                    vpc_attachment: Some(to_wire_vpc_attachment(att)),
                };
                wire::serialize_update_vpc_attachment_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_delete_attachment(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_attachment_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_attachment(&input.attachment_id) {
            Ok(mut att) => {
                att.state = "DELETING".to_string();
                let resp = wire::DeleteAttachmentResponse {
                    attachment: Some(to_wire_attachment(&att)),
                };
                wire::serialize_delete_attachment_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_accept_attachment(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_accept_attachment_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.accept_attachment(&input.attachment_id) {
            Ok(att) => {
                let resp = wire::AcceptAttachmentResponse {
                    attachment: Some(to_wire_attachment(att)),
                };
                wire::serialize_accept_attachment_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_reject_attachment(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_reject_attachment_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.reject_attachment(&input.attachment_id) {
            Ok(att) => {
                let resp = wire::RejectAttachmentResponse {
                    attachment: Some(to_wire_attachment(att)),
                };
                wire::serialize_reject_attachment_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_list_attachments(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_list_attachments_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let attachments = state.list_attachments();
        let wire_atts: Vec<wire::Attachment> =
            attachments.iter().map(|a| to_wire_attachment(a)).collect();
        wire::serialize_list_attachments_response(&wire::ListAttachmentsResponse {
            attachments: if wire_atts.is_empty() {
                None
            } else {
                Some(wire_atts)
            },
            next_token: None,
        })
    }

    // ── Connect Peer handlers ──

    async fn handle_create_connect_peer(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_connect_peer_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.connect_attachment_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'ConnectAttachmentId'");
        }
        if input.peer_address.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'PeerAddress'");
        }
        let core_network_address = input.core_network_address.as_deref();
        let inside_cidr_blocks = input.inside_cidr_blocks.clone().unwrap_or_default();
        let tags = tags_from_input(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_connect_peer(
            &input.connect_attachment_id,
            &input.peer_address,
            core_network_address,
            inside_cidr_blocks,
            tags,
        ) {
            Ok(cp) => {
                let resp = wire::CreateConnectPeerResponse {
                    connect_peer: Some(to_wire_connect_peer(cp)),
                };
                wire::serialize_create_connect_peer_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_get_connect_peer(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_connect_peer_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_connect_peer(&input.connect_peer_id) {
            Ok(cp) => {
                let resp = wire::GetConnectPeerResponse {
                    connect_peer: Some(to_wire_connect_peer(cp)),
                };
                wire::serialize_get_connect_peer_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_delete_connect_peer(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_connect_peer_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_connect_peer(&input.connect_peer_id) {
            Ok(mut cp) => {
                cp.state = "DELETING".to_string();
                let resp = wire::DeleteConnectPeerResponse {
                    connect_peer: Some(to_wire_connect_peer(&cp)),
                };
                wire::serialize_delete_connect_peer_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_list_connect_peers(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_list_connect_peers_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let peers = state.list_connect_peers();
        let wire_peers: Vec<wire::ConnectPeerSummary> = peers
            .iter()
            .map(|cp| wire::ConnectPeerSummary {
                connect_attachment_id: Some(cp.connect_attachment_id.clone()),
                connect_peer_id: Some(cp.connect_peer_id.clone()),
                core_network_id: cp.core_network_id.clone(),
                edge_location: cp.edge_location.clone(),
                connect_peer_state: Some(cp.state.clone()),
                tags: if cp.tags.is_empty() {
                    None
                } else {
                    Some(tags_to_wire(&cp.tags))
                },
                ..Default::default()
            })
            .collect();
        wire::serialize_list_connect_peers_response(&wire::ListConnectPeersResponse {
            connect_peers: if wire_peers.is_empty() {
                None
            } else {
                Some(wire_peers)
            },
            next_token: None,
        })
    }

    // ── Connect Peer Association handlers ──

    async fn handle_associate_connect_peer(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_associate_connect_peer_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.connect_peer_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'ConnectPeerId'");
        }
        if input.device_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'DeviceId'");
        }
        let link_id = input.link_id.as_deref();

        let mut state = state.write().await;
        match state.associate_connect_peer(
            &input.global_network_id,
            &input.connect_peer_id,
            &input.device_id,
            link_id,
        ) {
            Ok(assoc) => {
                let resp = wire::AssociateConnectPeerResponse {
                    connect_peer_association: Some(to_wire_connect_peer_assoc(assoc)),
                };
                wire::serialize_associate_connect_peer_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_get_connect_peer_associations(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_connect_peer_associations_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state = state.read().await;
        let assocs = state.get_connect_peer_associations(&input.global_network_id);
        let wire_assocs: Vec<wire::ConnectPeerAssociation> = assocs
            .iter()
            .map(|a| to_wire_connect_peer_assoc(a))
            .collect();
        wire::serialize_get_connect_peer_associations_response(
            &wire::GetConnectPeerAssociationsResponse {
                connect_peer_associations: if wire_assocs.is_empty() {
                    None
                } else {
                    Some(wire_assocs)
                },
                next_token: None,
            },
        )
    }

    async fn handle_disassociate_connect_peer(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_disassociate_connect_peer_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.disassociate_connect_peer(&input.global_network_id, &input.connect_peer_id) {
            Ok(mut assoc) => {
                assoc.state = "DELETED".to_string();
                let resp = wire::DisassociateConnectPeerResponse {
                    connect_peer_association: Some(to_wire_connect_peer_assoc(&assoc)),
                };
                wire::serialize_disassociate_connect_peer_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    // ── Link Association handlers ──

    async fn handle_associate_link(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_associate_link_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.device_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'DeviceId'");
        }
        if input.link_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'LinkId'");
        }

        let mut state = state.write().await;
        match state.associate_link(&input.global_network_id, &input.device_id, &input.link_id) {
            Ok(assoc) => {
                let resp = wire::AssociateLinkResponse {
                    link_association: Some(to_wire_link_assoc(&assoc)),
                };
                wire::serialize_associate_link_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_get_link_associations(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_link_associations_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let assocs = state.get_link_associations(&input.global_network_id);
        let wire_assocs: Vec<wire::LinkAssociation> =
            assocs.iter().map(|a| to_wire_link_assoc(a)).collect();
        wire::serialize_get_link_associations_response(&wire::GetLinkAssociationsResponse {
            link_associations: if wire_assocs.is_empty() {
                None
            } else {
                Some(wire_assocs)
            },
            next_token: None,
        })
    }

    async fn handle_disassociate_link(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_disassociate_link_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.disassociate_link(&input.global_network_id, &input.device_id, &input.link_id) {
            Ok(mut assoc) => {
                assoc.state = "DELETED".to_string();
                let resp = wire::DisassociateLinkResponse {
                    link_association: Some(to_wire_link_assoc(&assoc)),
                };
                wire::serialize_disassociate_link_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    // ── Customer Gateway Association handlers ──

    async fn handle_associate_customer_gateway(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_associate_customer_gateway_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.customer_gateway_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'CustomerGatewayArn'");
        }
        if input.device_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'DeviceId'");
        }
        let link_id = input.link_id.as_deref();

        let mut state = state.write().await;
        match state.associate_customer_gateway(
            &input.global_network_id,
            &input.customer_gateway_arn,
            &input.device_id,
            link_id,
        ) {
            Ok(assoc) => {
                let resp = wire::AssociateCustomerGatewayResponse {
                    customer_gateway_association: Some(to_wire_cgw_assoc(assoc)),
                };
                wire::serialize_associate_customer_gateway_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_get_customer_gateway_associations(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_customer_gateway_associations_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let assocs = state.get_customer_gateway_associations(&input.global_network_id);
        let wire_assocs: Vec<wire::CustomerGatewayAssociation> =
            assocs.iter().map(|a| to_wire_cgw_assoc(a)).collect();
        wire::serialize_get_customer_gateway_associations_response(
            &wire::GetCustomerGatewayAssociationsResponse {
                customer_gateway_associations: if wire_assocs.is_empty() {
                    None
                } else {
                    Some(wire_assocs)
                },
                next_token: None,
            },
        )
    }

    async fn handle_disassociate_customer_gateway(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_disassociate_customer_gateway_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state
            .disassociate_customer_gateway(&input.global_network_id, &input.customer_gateway_arn)
        {
            Ok(mut assoc) => {
                assoc.state = "DELETED".to_string();
                let resp = wire::DisassociateCustomerGatewayResponse {
                    customer_gateway_association: Some(to_wire_cgw_assoc(&assoc)),
                };
                wire::serialize_disassociate_customer_gateway_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    // ── Transit Gateway Connect Peer Association handlers ──

    async fn handle_associate_transit_gateway_connect_peer(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_associate_transit_gateway_connect_peer_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.transit_gateway_connect_peer_arn.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing 'TransitGatewayConnectPeerArn'",
            );
        }
        if input.device_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'DeviceId'");
        }
        let link_id = input.link_id.as_deref();

        let mut state = state.write().await;
        match state.associate_transit_gateway_connect_peer(
            &input.global_network_id,
            &input.transit_gateway_connect_peer_arn,
            &input.device_id,
            link_id,
        ) {
            Ok(assoc) => {
                let resp = wire::AssociateTransitGatewayConnectPeerResponse {
                    transit_gateway_connect_peer_association: Some(to_wire_tgw_cp_assoc(assoc)),
                };
                wire::serialize_associate_transit_gateway_connect_peer_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_get_transit_gateway_connect_peer_associations(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_transit_gateway_connect_peer_associations_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let assocs = state.get_transit_gateway_connect_peer_associations(&input.global_network_id);
        let wire_assocs: Vec<wire::TransitGatewayConnectPeerAssociation> =
            assocs.iter().map(|a| to_wire_tgw_cp_assoc(a)).collect();
        wire::serialize_get_transit_gateway_connect_peer_associations_response(
            &wire::GetTransitGatewayConnectPeerAssociationsResponse {
                transit_gateway_connect_peer_associations: if wire_assocs.is_empty() {
                    None
                } else {
                    Some(wire_assocs)
                },
                next_token: None,
            },
        )
    }

    async fn handle_disassociate_transit_gateway_connect_peer(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_disassociate_transit_gateway_connect_peer_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.disassociate_transit_gateway_connect_peer(
            &input.global_network_id,
            &input.transit_gateway_connect_peer_arn,
        ) {
            Ok(mut assoc) => {
                assoc.state = "DELETED".to_string();
                let resp = wire::DisassociateTransitGatewayConnectPeerResponse {
                    transit_gateway_connect_peer_association: Some(to_wire_tgw_cp_assoc(&assoc)),
                };
                wire::serialize_disassociate_transit_gateway_connect_peer_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    // ── Route Analysis handlers ──

    async fn handle_start_route_analysis(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_start_route_analysis_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let source_tgw_att_arn = input.source.transit_gateway_attachment_arn.as_deref();
        let source_ip = input.source.ip_address.as_deref();
        let dest_tgw_att_arn = input.destination.transit_gateway_attachment_arn.as_deref();
        let dest_ip = input.destination.ip_address.as_deref();
        // Note: TransitGatewayArn was extracted from the JSON in the legacy handler, but the
        // generated `RouteAnalysisEndpointOptionsSpecification` model only has `IpAddress` and
        // `TransitGatewayAttachmentArn`. Pass None for the TGW ARN fields.
        let include_return_path = input.include_return_path.unwrap_or(false);
        let use_middleboxes = input.use_middleboxes.unwrap_or(false);

        let mut state = state.write().await;
        match state.start_route_analysis(
            &input.global_network_id,
            account_id,
            None,
            source_tgw_att_arn,
            source_ip,
            None,
            dest_tgw_att_arn,
            dest_ip,
            include_return_path,
            use_middleboxes,
        ) {
            Ok(ra) => {
                let resp = wire::StartRouteAnalysisResponse {
                    route_analysis: Some(to_wire_route_analysis(ra)),
                };
                wire::serialize_start_route_analysis_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }

    async fn handle_get_route_analysis(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkManagerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_route_analysis_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_route_analysis(&input.global_network_id, &input.route_analysis_id) {
            Ok(ra) => {
                let resp = wire::GetRouteAnalysisResponse {
                    route_analysis: Some(to_wire_route_analysis(ra)),
                };
                wire::serialize_get_route_analysis_response(&resp)
            }
            Err(e) => nm_error_response(&e),
        }
    }
}

// ── Wire conversion helpers ──

fn to_epoch_f64(dt: &chrono::DateTime<chrono::Utc>) -> f64 {
    dt.timestamp() as f64 + (dt.timestamp_subsec_millis() as f64 / 1000.0)
}

fn tags_to_wire(tags: &HashMap<String, String>) -> Vec<wire::Tag> {
    tags.iter()
        .map(|(k, v)| wire::Tag {
            key: Some(k.clone()),
            value: Some(v.clone()),
        })
        .collect()
}

/// Convert a slice of input `wire::Tag` values into the internal HashMap representation.
fn tags_from_input(tags: Option<&[wire::Tag]>) -> HashMap<String, String> {
    let mut out = HashMap::new();
    if let Some(arr) = tags {
        for t in arr {
            if let (Some(k), Some(v)) = (t.key.as_ref(), t.value.as_ref()) {
                out.insert(k.clone(), v.clone());
            }
        }
    }
    out
}

fn to_wire_global_network(network: &GlobalNetwork) -> wire::GlobalNetwork {
    wire::GlobalNetwork {
        global_network_id: Some(network.global_network_id.clone()),
        global_network_arn: Some(network.global_network_arn.clone()),
        description: Some(network.description.clone()),
        created_at: Some(to_epoch_f64(&network.created_at)),
        state: Some(network.state.clone()),
        tags: if network.tags.is_empty() {
            None
        } else {
            Some(tags_to_wire(&network.tags))
        },
    }
}

fn to_wire_core_network(cn: &CoreNetwork) -> wire::CoreNetwork {
    wire::CoreNetwork {
        core_network_id: Some(cn.core_network_id.clone()),
        core_network_arn: Some(cn.core_network_arn.clone()),
        global_network_id: Some(cn.global_network_id.clone()),
        description: Some(cn.description.clone()),
        created_at: Some(to_epoch_f64(&cn.created_at)),
        state: Some(cn.state.clone()),
        tags: if cn.tags.is_empty() {
            None
        } else {
            Some(tags_to_wire(&cn.tags))
        },
        ..Default::default()
    }
}

fn to_wire_site(site: &Site) -> wire::Site {
    wire::Site {
        site_id: Some(site.site_id.clone()),
        site_arn: Some(site.site_arn.clone()),
        global_network_id: Some(site.global_network_id.clone()),
        description: Some(site.description.clone()),
        created_at: Some(to_epoch_f64(&site.created_at)),
        state: Some(site.state.clone()),
        tags: if site.tags.is_empty() {
            None
        } else {
            Some(tags_to_wire(&site.tags))
        },
        ..Default::default()
    }
}

fn to_wire_device(device: &Device) -> wire::Device {
    wire::Device {
        device_id: Some(device.device_id.clone()),
        device_arn: Some(device.device_arn.clone()),
        global_network_id: Some(device.global_network_id.clone()),
        description: Some(device.description.clone()),
        site_id: device.site_id.clone(),
        model: device.model.clone(),
        serial_number: device.serial_number.clone(),
        r#type: device.device_type.clone(),
        vendor: device.vendor.clone(),
        created_at: Some(to_epoch_f64(&device.created_at)),
        state: Some(device.state.clone()),
        tags: if device.tags.is_empty() {
            None
        } else {
            Some(tags_to_wire(&device.tags))
        },
        ..Default::default()
    }
}

fn to_wire_link(link: &Link) -> wire::Link {
    wire::Link {
        link_id: Some(link.link_id.clone()),
        link_arn: Some(link.link_arn.clone()),
        global_network_id: Some(link.global_network_id.clone()),
        site_id: Some(link.site_id.clone()),
        description: Some(link.description.clone()),
        provider: Some(link.provider.clone()),
        r#type: Some(link.link_type.clone()),
        bandwidth: Some(wire::Bandwidth {
            download_speed: Some(link.bandwidth_download_speed),
            upload_speed: Some(link.bandwidth_upload_speed),
        }),
        created_at: Some(to_epoch_f64(&link.created_at)),
        state: Some(link.state.clone()),
        tags: if link.tags.is_empty() {
            None
        } else {
            Some(tags_to_wire(&link.tags))
        },
    }
}

fn to_wire_connection(conn: &Connection) -> wire::Connection {
    wire::Connection {
        connection_id: Some(conn.connection_id.clone()),
        connection_arn: Some(conn.connection_arn.clone()),
        global_network_id: Some(conn.global_network_id.clone()),
        device_id: Some(conn.device_id.clone()),
        connected_device_id: Some(conn.connected_device_id.clone()),
        link_id: conn.link_id.clone(),
        connected_link_id: conn.connected_link_id.clone(),
        description: Some(conn.description.clone()),
        created_at: Some(to_epoch_f64(&conn.created_at)),
        state: Some(conn.state.clone()),
        tags: if conn.tags.is_empty() {
            None
        } else {
            Some(tags_to_wire(&conn.tags))
        },
    }
}

fn to_wire_tgw_registration(reg: &TransitGatewayRegistration) -> wire::TransitGatewayRegistration {
    wire::TransitGatewayRegistration {
        global_network_id: Some(reg.global_network_id.clone()),
        transit_gateway_arn: Some(reg.transit_gateway_arn.clone()),
        state: Some(wire::TransitGatewayRegistrationStateReason {
            code: Some(reg.state.clone()),
            message: if reg.state_message.is_empty() {
                None
            } else {
                Some(reg.state_message.clone())
            },
        }),
    }
}

fn to_wire_attachment(att: &Attachment) -> wire::Attachment {
    wire::Attachment {
        attachment_id: Some(att.attachment_id.clone()),
        attachment_type: Some(att.attachment_type.clone()),
        core_network_id: Some(att.core_network_id.clone()),
        core_network_arn: Some(att.core_network_arn.clone()),
        owner_account_id: Some(att.owner_account_id.clone()),
        resource_arn: Some(att.resource_arn.clone()),
        edge_location: att.edge_location.clone(),
        state: Some(att.state.clone()),
        created_at: Some(to_epoch_f64(&att.created_at)),
        updated_at: Some(to_epoch_f64(&att.updated_at)),
        segment_name: att.segment_name.clone(),
        tags: if att.tags.is_empty() {
            None
        } else {
            Some(tags_to_wire(&att.tags))
        },
        ..Default::default()
    }
}

fn to_wire_vpc_attachment(att: &Attachment) -> wire::VpcAttachment {
    wire::VpcAttachment {
        attachment: Some(to_wire_attachment(att)),
        subnet_arns: if att.subnet_arns.is_empty() {
            None
        } else {
            Some(att.subnet_arns.clone())
        },
        options: None,
    }
}

fn to_wire_connect_peer(cp: &ConnectPeer) -> wire::ConnectPeer {
    wire::ConnectPeer {
        connect_peer_id: Some(cp.connect_peer_id.clone()),
        connect_attachment_id: Some(cp.connect_attachment_id.clone()),
        core_network_id: cp.core_network_id.clone(),
        edge_location: cp.edge_location.clone(),
        state: Some(cp.state.clone()),
        created_at: Some(to_epoch_f64(&cp.created_at)),
        tags: if cp.tags.is_empty() {
            None
        } else {
            Some(tags_to_wire(&cp.tags))
        },
        configuration: Some(wire::ConnectPeerConfiguration {
            peer_address: Some(cp.peer_address.clone()),
            core_network_address: cp.core_network_address.clone(),
            inside_cidr_blocks: if cp.inside_cidr_blocks.is_empty() {
                None
            } else {
                Some(cp.inside_cidr_blocks.clone())
            },
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn to_wire_connect_peer_assoc(assoc: &ConnectPeerAssociation) -> wire::ConnectPeerAssociation {
    wire::ConnectPeerAssociation {
        connect_peer_id: Some(assoc.connect_peer_id.clone()),
        global_network_id: Some(assoc.global_network_id.clone()),
        device_id: Some(assoc.device_id.clone()),
        link_id: assoc.link_id.clone(),
        state: Some(assoc.state.clone()),
    }
}

fn to_wire_link_assoc(assoc: &LinkAssociation) -> wire::LinkAssociation {
    wire::LinkAssociation {
        global_network_id: Some(assoc.global_network_id.clone()),
        device_id: Some(assoc.device_id.clone()),
        link_id: Some(assoc.link_id.clone()),
        link_association_state: Some(assoc.state.clone()),
    }
}

fn to_wire_cgw_assoc(assoc: &CustomerGatewayAssociation) -> wire::CustomerGatewayAssociation {
    wire::CustomerGatewayAssociation {
        customer_gateway_arn: Some(assoc.customer_gateway_arn.clone()),
        global_network_id: Some(assoc.global_network_id.clone()),
        device_id: Some(assoc.device_id.clone()),
        link_id: assoc.link_id.clone(),
        state: Some(assoc.state.clone()),
    }
}

fn to_wire_tgw_cp_assoc(
    assoc: &TransitGatewayConnectPeerAssociation,
) -> wire::TransitGatewayConnectPeerAssociation {
    wire::TransitGatewayConnectPeerAssociation {
        transit_gateway_connect_peer_arn: Some(assoc.transit_gateway_connect_peer_arn.clone()),
        global_network_id: Some(assoc.global_network_id.clone()),
        device_id: Some(assoc.device_id.clone()),
        link_id: assoc.link_id.clone(),
        state: Some(assoc.state.clone()),
    }
}

fn to_wire_route_analysis(ra: &RouteAnalysis) -> wire::RouteAnalysis {
    wire::RouteAnalysis {
        route_analysis_id: Some(ra.route_analysis_id.clone()),
        global_network_id: Some(ra.global_network_id.clone()),
        owner_account_id: Some(ra.owner_account_id.clone()),
        start_timestamp: Some(to_epoch_f64(&ra.started_at)),
        status: Some(ra.status.clone()),
        include_return_path: Some(ra.include_return_path),
        use_middleboxes: Some(ra.use_middleboxes),
        source: Some(wire::RouteAnalysisEndpointOptions {
            transit_gateway_arn: ra.source_transit_gateway_arn.clone(),
            transit_gateway_attachment_arn: ra.source_transit_gateway_attachment_arn.clone(),
            ip_address: ra.source_ip_address.clone(),
        }),
        destination: Some(wire::RouteAnalysisEndpointOptions {
            transit_gateway_arn: ra.destination_transit_gateway_arn.clone(),
            transit_gateway_attachment_arn: ra.destination_transit_gateway_attachment_arn.clone(),
            ip_address: ra.destination_ip_address.clone(),
        }),
        ..Default::default()
    }
}

fn nm_error_shape(err: &NetworkManagerError) -> (u16, &'static str) {
    match err {
        NetworkManagerError::GlobalNetworkNotFound { .. } => (404, "ResourceNotFoundException"),
        NetworkManagerError::CoreNetworkNotFound { .. } => (404, "ResourceNotFoundException"),
        NetworkManagerError::SiteNotFound { .. } => (404, "ResourceNotFoundException"),
        NetworkManagerError::LinkNotFound { .. } => (404, "ResourceNotFoundException"),
        NetworkManagerError::DeviceNotFound { .. } => (404, "ResourceNotFoundException"),
        NetworkManagerError::ConnectionNotFound { .. } => (404, "ResourceNotFoundException"),
        NetworkManagerError::ResourceNotFound { .. } => (404, "ResourceNotFoundException"),
        NetworkManagerError::TransitGatewayNotRegistered { .. } => {
            (404, "ResourceNotFoundException")
        }
        NetworkManagerError::AttachmentNotFound { .. } => (404, "ResourceNotFoundException"),
        NetworkManagerError::ConnectPeerNotFound { .. } => (404, "ResourceNotFoundException"),
        NetworkManagerError::ConnectPeerAssociationNotFound { .. } => {
            (404, "ResourceNotFoundException")
        }
        NetworkManagerError::LinkAssociationNotFound { .. } => (404, "ResourceNotFoundException"),
        NetworkManagerError::CustomerGatewayNotFound { .. } => (404, "ResourceNotFoundException"),
        NetworkManagerError::TransitGatewayConnectPeerAssociationNotFound { .. } => {
            (404, "ResourceNotFoundException")
        }
        NetworkManagerError::TransitGatewayConnectPeerAssociationNotFoundInNetwork { .. } => {
            (404, "ResourceNotFoundException")
        }
        NetworkManagerError::RouteAnalysisNotFound { .. } => (404, "ResourceNotFoundException"),
        NetworkManagerError::RouteAnalysisNotFoundInNetwork { .. } => {
            (404, "ResourceNotFoundException")
        }
    }
}

fn nm_error_response(err: &NetworkManagerError) -> MockResponse {
    let (status, error_type) = nm_error_shape(err);
    let body = json!({
        "Type": "User",
        "Message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

// FIX(terraform-e2e): old implementation only handled amazonaws.com URIs; localhost test URIs
//   produced segments like ["http:", "", "127.0.0.1:PORT", "global-networks"], causing every
//   route match to fall through to the 404 catch-all.
fn extract_path(uri: &str) -> String {
    let after_scheme = if let Some(idx) = uri.find("://") {
        &uri[idx + 3..]
    } else {
        uri
    };
    let path_start = after_scheme.find('/').unwrap_or(after_scheme.len());
    let path_and_query = &after_scheme[path_start..];
    match path_and_query.find('?') {
        Some(q) => path_and_query[..q].to_string(),
        None => path_and_query.to_string(),
    }
}

fn extract_query(uri: &str) -> String {
    if let Some(idx) = uri.find('?') {
        uri[idx + 1..].to_string()
    } else {
        String::new()
    }
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

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "Type": "User",
        "Message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
