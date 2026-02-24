use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::model;
use crate::state::{DirectConnectError, DirectConnectState};
use crate::types;
use crate::views::DirectConnectStateView;
use crate::wire;

/// Direct Connect service handler that processes awsJson1.1 protocol requests.
pub struct DirectConnectService {
    pub(crate) state: Arc<BackendState<DirectConnectState>>,
    pub(crate) notifier: StateChangeNotifier<DirectConnectStateView>,
}

impl DirectConnectService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for DirectConnectService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for DirectConnectService {
    fn service_name(&self) -> &str {
        "directconnect"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://directconnect\..*\.amazonaws\.com",
            r"https?://directconnect\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl DirectConnectService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        // Extract action from X-Amz-Target header
        // Format: "OvertureService.CreateConnection"
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

        // Validate the body is well-formed JSON up-front; the typed deserialisers in
        // `wire` re-parse the bytes per operation.
        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        use winterbaume_core::StatefulService;
        let response = match action.as_str() {
            "CreateConnection" => {
                self.handle_create_connection(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeConnections" => self.handle_describe_connections(&state, body_bytes).await,
            "DeleteConnection" => self.handle_delete_connection(&state, body_bytes).await,
            "DescribeLocations" => self.handle_describe_locations(&state, &region).await,
            // --- Unimplemented operations (auto-generated stubs) ---
            "AcceptDirectConnectGatewayAssociationProposal" => json_error_response(
                501,
                "NotImplementedError",
                "AcceptDirectConnectGatewayAssociationProposal is not yet implemented in winterbaume-directconnect",
            ),
            "AllocateConnectionOnInterconnect" => json_error_response(
                501,
                "NotImplementedError",
                "AllocateConnectionOnInterconnect is not yet implemented in winterbaume-directconnect",
            ),
            "AllocateHostedConnection" => json_error_response(
                501,
                "NotImplementedError",
                "AllocateHostedConnection is not yet implemented in winterbaume-directconnect",
            ),
            "AllocatePrivateVirtualInterface" => json_error_response(
                501,
                "NotImplementedError",
                "AllocatePrivateVirtualInterface is not yet implemented in winterbaume-directconnect",
            ),
            "AllocatePublicVirtualInterface" => json_error_response(
                501,
                "NotImplementedError",
                "AllocatePublicVirtualInterface is not yet implemented in winterbaume-directconnect",
            ),
            "AllocateTransitVirtualInterface" => json_error_response(
                501,
                "NotImplementedError",
                "AllocateTransitVirtualInterface is not yet implemented in winterbaume-directconnect",
            ),
            "AssociateConnectionWithLag" => json_error_response(
                501,
                "NotImplementedError",
                "AssociateConnectionWithLag is not yet implemented in winterbaume-directconnect",
            ),
            "AssociateHostedConnection" => json_error_response(
                501,
                "NotImplementedError",
                "AssociateHostedConnection is not yet implemented in winterbaume-directconnect",
            ),
            "AssociateMacSecKey" => json_error_response(
                501,
                "NotImplementedError",
                "AssociateMacSecKey is not yet implemented in winterbaume-directconnect",
            ),
            "AssociateVirtualInterface" => json_error_response(
                501,
                "NotImplementedError",
                "AssociateVirtualInterface is not yet implemented in winterbaume-directconnect",
            ),
            "ConfirmConnection" => json_error_response(
                501,
                "NotImplementedError",
                "ConfirmConnection is not yet implemented in winterbaume-directconnect",
            ),
            "ConfirmCustomerAgreement" => json_error_response(
                501,
                "NotImplementedError",
                "ConfirmCustomerAgreement is not yet implemented in winterbaume-directconnect",
            ),
            "ConfirmPrivateVirtualInterface" => json_error_response(
                501,
                "NotImplementedError",
                "ConfirmPrivateVirtualInterface is not yet implemented in winterbaume-directconnect",
            ),
            "ConfirmPublicVirtualInterface" => json_error_response(
                501,
                "NotImplementedError",
                "ConfirmPublicVirtualInterface is not yet implemented in winterbaume-directconnect",
            ),
            "ConfirmTransitVirtualInterface" => json_error_response(
                501,
                "NotImplementedError",
                "ConfirmTransitVirtualInterface is not yet implemented in winterbaume-directconnect",
            ),
            "CreateBGPPeer" => json_error_response(
                501,
                "NotImplementedError",
                "CreateBGPPeer is not yet implemented in winterbaume-directconnect",
            ),
            "CreateDirectConnectGateway" => json_error_response(
                501,
                "NotImplementedError",
                "CreateDirectConnectGateway is not yet implemented in winterbaume-directconnect",
            ),
            "CreateDirectConnectGatewayAssociation" => json_error_response(
                501,
                "NotImplementedError",
                "CreateDirectConnectGatewayAssociation is not yet implemented in winterbaume-directconnect",
            ),
            "CreateDirectConnectGatewayAssociationProposal" => json_error_response(
                501,
                "NotImplementedError",
                "CreateDirectConnectGatewayAssociationProposal is not yet implemented in winterbaume-directconnect",
            ),
            "CreateInterconnect" => json_error_response(
                501,
                "NotImplementedError",
                "CreateInterconnect is not yet implemented in winterbaume-directconnect",
            ),
            "CreateLag" => json_error_response(
                501,
                "NotImplementedError",
                "CreateLag is not yet implemented in winterbaume-directconnect",
            ),
            "CreatePrivateVirtualInterface" => json_error_response(
                501,
                "NotImplementedError",
                "CreatePrivateVirtualInterface is not yet implemented in winterbaume-directconnect",
            ),
            "CreatePublicVirtualInterface" => json_error_response(
                501,
                "NotImplementedError",
                "CreatePublicVirtualInterface is not yet implemented in winterbaume-directconnect",
            ),
            "CreateTransitVirtualInterface" => json_error_response(
                501,
                "NotImplementedError",
                "CreateTransitVirtualInterface is not yet implemented in winterbaume-directconnect",
            ),
            "DeleteBGPPeer" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteBGPPeer is not yet implemented in winterbaume-directconnect",
            ),
            "DeleteDirectConnectGateway" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteDirectConnectGateway is not yet implemented in winterbaume-directconnect",
            ),
            "DeleteDirectConnectGatewayAssociation" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteDirectConnectGatewayAssociation is not yet implemented in winterbaume-directconnect",
            ),
            "DeleteDirectConnectGatewayAssociationProposal" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteDirectConnectGatewayAssociationProposal is not yet implemented in winterbaume-directconnect",
            ),
            "DeleteInterconnect" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteInterconnect is not yet implemented in winterbaume-directconnect",
            ),
            "DeleteLag" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteLag is not yet implemented in winterbaume-directconnect",
            ),
            "DeleteVirtualInterface" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteVirtualInterface is not yet implemented in winterbaume-directconnect",
            ),
            "DescribeConnectionLoa" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeConnectionLoa is not yet implemented in winterbaume-directconnect",
            ),
            "DescribeConnectionsOnInterconnect" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeConnectionsOnInterconnect is not yet implemented in winterbaume-directconnect",
            ),
            "DescribeCustomerMetadata" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeCustomerMetadata is not yet implemented in winterbaume-directconnect",
            ),
            "DescribeDirectConnectGatewayAssociationProposals" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeDirectConnectGatewayAssociationProposals is not yet implemented in winterbaume-directconnect",
            ),
            "DescribeDirectConnectGatewayAssociations" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeDirectConnectGatewayAssociations is not yet implemented in winterbaume-directconnect",
            ),
            "DescribeDirectConnectGatewayAttachments" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeDirectConnectGatewayAttachments is not yet implemented in winterbaume-directconnect",
            ),
            "DescribeDirectConnectGateways" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeDirectConnectGateways is not yet implemented in winterbaume-directconnect",
            ),
            "DescribeHostedConnections" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeHostedConnections is not yet implemented in winterbaume-directconnect",
            ),
            "DescribeInterconnectLoa" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeInterconnectLoa is not yet implemented in winterbaume-directconnect",
            ),
            "DescribeInterconnects" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeInterconnects is not yet implemented in winterbaume-directconnect",
            ),
            "DescribeLags" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeLags is not yet implemented in winterbaume-directconnect",
            ),
            "DescribeLoa" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeLoa is not yet implemented in winterbaume-directconnect",
            ),
            "DescribeRouterConfiguration" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeRouterConfiguration is not yet implemented in winterbaume-directconnect",
            ),
            "DescribeTags" => self.handle_describe_tags(&state, body_bytes).await,
            "DescribeVirtualGateways" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeVirtualGateways is not yet implemented in winterbaume-directconnect",
            ),
            "DescribeVirtualInterfaces" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeVirtualInterfaces is not yet implemented in winterbaume-directconnect",
            ),
            "DisassociateConnectionFromLag" => json_error_response(
                501,
                "NotImplementedError",
                "DisassociateConnectionFromLag is not yet implemented in winterbaume-directconnect",
            ),
            "DisassociateMacSecKey" => json_error_response(
                501,
                "NotImplementedError",
                "DisassociateMacSecKey is not yet implemented in winterbaume-directconnect",
            ),
            "ListVirtualInterfaceTestHistory" => json_error_response(
                501,
                "NotImplementedError",
                "ListVirtualInterfaceTestHistory is not yet implemented in winterbaume-directconnect",
            ),
            "StartBgpFailoverTest" => json_error_response(
                501,
                "NotImplementedError",
                "StartBgpFailoverTest is not yet implemented in winterbaume-directconnect",
            ),
            "StopBgpFailoverTest" => json_error_response(
                501,
                "NotImplementedError",
                "StopBgpFailoverTest is not yet implemented in winterbaume-directconnect",
            ),
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "UpdateConnection" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateConnection is not yet implemented in winterbaume-directconnect",
            ),
            "UpdateDirectConnectGateway" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateDirectConnectGateway is not yet implemented in winterbaume-directconnect",
            ),
            "UpdateDirectConnectGatewayAssociation" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateDirectConnectGatewayAssociation is not yet implemented in winterbaume-directconnect",
            ),
            "UpdateLag" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateLag is not yet implemented in winterbaume-directconnect",
            ),
            "UpdateVirtualInterfaceAttributes" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateVirtualInterfaceAttributes is not yet implemented in winterbaume-directconnect",
            ),
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for DirectConnect"),
            ),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<DirectConnectState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_connection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "DirectConnectClientException", &e),
        };
        if input.connection_name.is_empty() {
            return json_error_response(
                400,
                "DirectConnectClientException",
                "connectionName is required",
            );
        }
        if input.location.is_empty() {
            return json_error_response(
                400,
                "DirectConnectClientException",
                "location is required",
            );
        }
        if input.bandwidth.is_empty() {
            return json_error_response(
                400,
                "DirectConnectClientException",
                "bandwidth is required",
            );
        }
        let connection_name = input.connection_name.as_str();
        let location = input.location.as_str();
        let bandwidth = input.bandwidth.as_str();
        let tags = tags_from_wire(&input.tags);

        let mut state = state.write().await;
        match state.create_connection(
            connection_name,
            location,
            bandwidth,
            account_id,
            region,
            tags,
        ) {
            Ok(conn) => wire::serialize_create_connection_response(&connection_to_wire(conn)),
            Err(e) => dc_error_response(&e),
        }
    }

    async fn handle_describe_connections(
        &self,
        state: &Arc<tokio::sync::RwLock<DirectConnectState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_connections_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "DirectConnectClientException", &e),
        };
        let connection_id = input.connection_id.as_deref();

        let state = state.read().await;
        match state.describe_connections(connection_id) {
            Ok(connections) => {
                let entries: Vec<model::Connection> =
                    connections.iter().map(|c| connection_to_wire(c)).collect();
                let result = model::Connections {
                    connections: Some(entries),
                    ..Default::default()
                };
                wire::serialize_describe_connections_response(&result)
            }
            Err(e) => dc_error_response(&e),
        }
    }

    async fn handle_delete_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<DirectConnectState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_connection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "DirectConnectClientException", &e),
        };
        if input.connection_id.is_empty() {
            return json_error_response(
                400,
                "DirectConnectClientException",
                "connectionId is required",
            );
        }
        let connection_id = input.connection_id.as_str();

        let mut state = state.write().await;
        match state.delete_connection(connection_id) {
            Ok(conn) => wire::serialize_delete_connection_response(&connection_to_wire(&conn)),
            Err(e) => dc_error_response(&e),
        }
    }

    async fn handle_describe_locations(
        &self,
        state: &Arc<tokio::sync::RwLock<DirectConnectState>>,
        region: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let locations = state.describe_locations(region);
        let entries: Vec<model::Location> = locations.iter().map(location_to_wire).collect();
        let result = model::Locations {
            locations: Some(entries),
        };
        wire::serialize_describe_locations_response(&result)
    }

    async fn handle_describe_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<DirectConnectState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_tags_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "DirectConnectClientException", &e),
        };
        let resource_arns: Vec<String> = input.resource_arns;

        let state = state.read().await;
        let entries = state.describe_tags(&resource_arns);

        let resource_tags: Vec<model::ResourceTag> = entries
            .into_iter()
            .filter_map(|(arn, tags_opt)| {
                tags_opt.map(|tags| {
                    let tag_list: Vec<model::Tag> = tags
                        .into_iter()
                        .map(|(k, v)| model::Tag {
                            key: k,
                            value: Some(v),
                        })
                        .collect();
                    model::ResourceTag {
                        resource_arn: Some(arn),
                        tags: if tag_list.is_empty() {
                            None
                        } else {
                            Some(tag_list)
                        },
                    }
                })
            })
            .collect();

        let result = model::DescribeTagsResponse {
            resource_tags: if resource_tags.is_empty() {
                None
            } else {
                Some(resource_tags)
            },
        };
        wire::serialize_describe_tags_response(&result)
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<DirectConnectState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "DirectConnectClientException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(
                400,
                "DirectConnectClientException",
                "resourceArn is required",
            );
        }
        let resource_arn = input.resource_arn.clone();
        let tags = tags_from_vec(&input.tags);

        let mut state = state.write().await;
        match state.tag_resource(&resource_arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&model::TagResourceResponse {}),
            Err(e) => dc_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<DirectConnectState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "DirectConnectClientException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(
                400,
                "DirectConnectClientException",
                "resourceArn is required",
            );
        }
        let resource_arn = input.resource_arn.clone();
        let tag_keys: Vec<String> = input.tag_keys;

        let mut state = state.write().await;
        match state.untag_resource(&resource_arn, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&model::UntagResourceResponse {}),
            Err(e) => dc_error_response(&e),
        }
    }
}

// --- State-to-wire conversion helpers ---

fn connection_to_wire(conn: &types::Connection) -> model::Connection {
    let tags = if conn.tags.is_empty() {
        None
    } else {
        Some(
            conn.tags
                .iter()
                .map(|(k, v)| model::Tag {
                    key: k.clone(),
                    value: Some(v.clone()),
                })
                .collect(),
        )
    };

    model::Connection {
        connection_id: Some(conn.connection_id.clone()),
        connection_name: Some(conn.connection_name.clone()),
        connection_state: Some(conn.connection_state.as_str().to_string()),
        region: Some(conn.region.clone()),
        location: Some(conn.location.clone()),
        bandwidth: Some(conn.bandwidth.clone()),
        owner_account: Some(conn.owner_account.clone()),
        vlan: Some(conn.vlan),
        partner_name: conn.partner_name.clone(),
        tags,
        ..Default::default()
    }
}

fn location_to_wire(loc: &types::Location) -> model::Location {
    model::Location {
        location_code: Some(loc.location_code.clone()),
        location_name: Some(loc.location_name.clone()),
        region: Some(loc.region.clone()),
        available_port_speeds: Some(loc.available_port_speeds.clone()),
        available_providers: Some(loc.available_providers.clone()),
        available_mac_sec_port_speeds: Some(loc.available_mac_sec_port_speeds.clone()),
    }
}

fn tags_from_wire(tags: &Option<Vec<model::Tag>>) -> HashMap<String, String> {
    match tags {
        Some(arr) => tags_from_vec(arr),
        None => HashMap::new(),
    }
}

fn tags_from_vec(tags: &[model::Tag]) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for tag in tags {
        if let Some(value) = tag.value.as_ref() {
            if !tag.key.is_empty() {
                out.insert(tag.key.clone(), value.clone());
            }
        }
    }
    out
}

fn dc_error_response(err: &DirectConnectError) -> MockResponse {
    match err {
        DirectConnectError::ConnectionNameEmpty => {
            json_error_response(400, "DirectConnectClientException", &err.to_string())
        }
        DirectConnectError::LocationEmpty => {
            json_error_response(400, "DirectConnectClientException", &err.to_string())
        }
        DirectConnectError::BandwidthEmpty => {
            json_error_response(400, "DirectConnectClientException", &err.to_string())
        }
        DirectConnectError::ConnectionNotFound { .. } => {
            json_error_response(400, "DirectConnectClientException", &err.to_string())
        }
        DirectConnectError::InvalidResourceArn { .. } => {
            json_error_response(400, "DirectConnectClientException", &err.to_string())
        }
    }
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}
