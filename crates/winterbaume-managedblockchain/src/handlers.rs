use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::state::{ManagedBlockchainError, ManagedBlockchainState};
use crate::views::ManagedBlockchainStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct ManagedBlockchainService {
    pub(crate) state: Arc<BackendState<ManagedBlockchainState>>,
    pub(crate) notifier: StateChangeNotifier<ManagedBlockchainStateView>,
}

impl ManagedBlockchainService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ManagedBlockchainService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ManagedBlockchainService {
    fn service_name(&self) -> &str {
        "managedblockchain"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://managedblockchain\.(.+)\.amazonaws\.com",
            r"https?://managedblockchain\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl ManagedBlockchainService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let (path, query_string) = extract_path_and_query(&request.uri);
        let method = request.method.as_str();

        // Validate JSON body up-front; the typed deserialisers in `wire` re-parse the bytes
        // per operation.
        if !request.body.is_empty()
            && serde_json::from_slice::<serde_json::Value>(&request.body).is_err()
        {
            return rest_json_error(400, "InvalidRequestException", "Invalid JSON");
        }

        let segments_owned: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(urldecode)
            .collect();
        let segments: Vec<&str> = segments_owned.iter().map(String::as_str).collect();

        let query: HashMap<String, String> = parse_query_string(&query_string);

        if segments.is_empty() {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        match segments[0] {
            "networks" => {
                self.dispatch_networks(
                    method, &segments, &request, &query, &state, account_id, &region,
                )
                .await
            }
            "accessors" => {
                self.dispatch_accessors(
                    method, &segments, &request, &query, &state, account_id, &region,
                )
                .await
            }
            "invitations" => {
                self.dispatch_invitations(method, &segments, &request, &query, &state)
                    .await
            }
            "tags" => {
                self.dispatch_tags(method, &segments, &request, &query, &state)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_networks(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        match (method, segments.len()) {
            // POST /networks - CreateNetwork
            ("POST", 1) => {
                self.handle_create_network(state, request, &[], query, account_id, region)
                    .await
            }
            // GET /networks - ListNetworks
            ("GET", 1) => self.handle_list_networks(state, request, &[], query).await,
            // GET /networks/{networkId} - GetNetwork
            ("GET", 2) => {
                let labels: &[(&str, &str)] = &[("NetworkId", segments[1])];
                self.handle_get_network(state, request, labels, query).await
            }
            // DELETE /networks/{networkId} - DeleteNetwork (no typed input — synthetic)
            ("DELETE", 2) => self.handle_delete_network(state, segments[1]).await,
            // Routes with 3 segments: /networks/{networkId}/{sub}
            (_, 3) => {
                let network_id = segments[1];
                let labels: &[(&str, &str)] = &[("NetworkId", network_id)];
                match (method, segments[2]) {
                    // GET /networks/{networkId}/members - ListMembers
                    ("GET", "members") => {
                        self.handle_list_members(state, request, labels, query)
                            .await
                    }
                    // POST /networks/{networkId}/members - CreateMember
                    ("POST", "members") => {
                        self.handle_create_member(state, request, labels, query, account_id, region)
                            .await
                    }
                    // POST /networks/{networkId}/proposals - CreateProposal
                    ("POST", "proposals") => {
                        self.handle_create_proposal(
                            state, request, labels, query, account_id, region,
                        )
                        .await
                    }
                    // GET /networks/{networkId}/proposals - ListProposals
                    ("GET", "proposals") => {
                        self.handle_list_proposals(state, request, labels, query)
                            .await
                    }
                    // POST /networks/{networkId}/nodes - CreateNode
                    ("POST", "nodes") => {
                        self.handle_create_node(state, request, labels, query, account_id, region)
                            .await
                    }
                    // GET /networks/{networkId}/nodes - ListNodes
                    ("GET", "nodes") => self.handle_list_nodes(state, request, labels, query).await,
                    _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                }
            }
            // Routes with 4 segments: /networks/{networkId}/{sub}/{id}
            (_, 4) => {
                let network_id = segments[1];
                match (method, segments[2]) {
                    // GET /networks/{networkId}/members/{memberId} - GetMember
                    ("GET", "members") => {
                        let labels: &[(&str, &str)] =
                            &[("NetworkId", network_id), ("MemberId", segments[3])];
                        self.handle_get_member(state, request, labels, query).await
                    }
                    // DELETE /networks/{networkId}/members/{memberId} - DeleteMember
                    ("DELETE", "members") => {
                        let labels: &[(&str, &str)] =
                            &[("NetworkId", network_id), ("MemberId", segments[3])];
                        self.handle_delete_member(state, request, labels, query)
                            .await
                    }
                    // PATCH /networks/{networkId}/members/{memberId} - UpdateMember
                    ("PATCH", "members") => {
                        let labels: &[(&str, &str)] =
                            &[("NetworkId", network_id), ("MemberId", segments[3])];
                        self.handle_update_member(state, request, labels, query)
                            .await
                    }
                    // GET /networks/{networkId}/proposals/{proposalId} - GetProposal
                    ("GET", "proposals") => {
                        let labels: &[(&str, &str)] =
                            &[("NetworkId", network_id), ("ProposalId", segments[3])];
                        self.handle_get_proposal(state, request, labels, query)
                            .await
                    }
                    // GET /networks/{networkId}/nodes/{nodeId} - GetNode
                    ("GET", "nodes") => {
                        let labels: &[(&str, &str)] =
                            &[("NetworkId", network_id), ("NodeId", segments[3])];
                        self.handle_get_node(state, request, labels, query).await
                    }
                    // DELETE /networks/{networkId}/nodes/{nodeId} - DeleteNode
                    ("DELETE", "nodes") => {
                        let labels: &[(&str, &str)] =
                            &[("NetworkId", network_id), ("NodeId", segments[3])];
                        self.handle_delete_node(state, request, labels, query).await
                    }
                    // PATCH /networks/{networkId}/nodes/{nodeId} - UpdateNode
                    ("PATCH", "nodes") => {
                        let labels: &[(&str, &str)] =
                            &[("NetworkId", network_id), ("NodeId", segments[3])];
                        self.handle_update_node(state, request, labels, query).await
                    }
                    _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                }
            }
            // Routes with 5 segments: /networks/{networkId}/proposals/{proposalId}/votes
            (_, 5) => {
                let network_id = segments[1];
                if segments[2] == "proposals" && segments[4] == "votes" {
                    let proposal_id = segments[3];
                    let labels: &[(&str, &str)] =
                        &[("NetworkId", network_id), ("ProposalId", proposal_id)];
                    match method {
                        "POST" => {
                            self.handle_vote_on_proposal(state, request, labels, query)
                                .await
                        }
                        "GET" => {
                            self.handle_list_proposal_votes(state, request, labels, query)
                                .await
                        }
                        _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                    }
                } else {
                    rest_json_error(404, "UnknownOperationException", "Not found")
                }
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_accessors(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        match (method, segments.len()) {
            // POST /accessors - CreateAccessor
            ("POST", 1) => {
                self.handle_create_accessor(state, request, &[], query, account_id, region)
                    .await
            }
            // GET /accessors - ListAccessors
            ("GET", 1) => self.handle_list_accessors(state, request, &[], query).await,
            // GET /accessors/{accessorId} - GetAccessor
            ("GET", 2) => {
                let labels: &[(&str, &str)] = &[("AccessorId", segments[1])];
                self.handle_get_accessor(state, request, labels, query)
                    .await
            }
            // DELETE /accessors/{accessorId} - DeleteAccessor
            ("DELETE", 2) => {
                let labels: &[(&str, &str)] = &[("AccessorId", segments[1])];
                self.handle_delete_accessor(state, request, labels, query)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn dispatch_invitations(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
    ) -> MockResponse {
        match (method, segments.len()) {
            // GET /invitations - ListInvitations
            ("GET", 1) => {
                self.handle_list_invitations(state, request, &[], query)
                    .await
            }
            // DELETE /invitations/{invitationId} - RejectInvitation
            ("DELETE", 2) => {
                let labels: &[(&str, &str)] = &[("InvitationId", segments[1])];
                self.handle_reject_invitation(state, request, labels, query)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn dispatch_tags(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
    ) -> MockResponse {
        // /tags/{ResourceArn} - the ARN is everything after /tags/
        if segments.len() < 2 {
            return rest_json_error(400, "InvalidRequestException", "Missing ResourceArn");
        }
        let resource_arn = segments[1..].join("/");
        let labels: &[(&str, &str)] = &[("ResourceArn", resource_arn.as_str())];

        match method {
            "GET" => {
                self.handle_list_tags_for_resource(state, request, labels, query)
                    .await
            }
            "POST" => {
                self.handle_tag_resource(state, request, labels, query)
                    .await
            }
            "DELETE" => {
                self.handle_untag_resource(state, request, labels, query)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // --- Network handlers ---

    async fn handle_create_network(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_network_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "InvalidRequestException", "Missing 'Name'");
        }
        if input.framework.is_empty() {
            return rest_json_error(400, "InvalidRequestException", "Missing 'Framework'");
        }
        if input.framework_version.is_empty() {
            return rest_json_error(400, "InvalidRequestException", "Missing 'FrameworkVersion'");
        }
        let mut state = state.write().await;
        match state.create_network(
            &input.name,
            input.description.as_deref(),
            &input.framework,
            &input.framework_version,
            account_id,
            region,
        ) {
            Ok((network, member)) => {
                wire::serialize_create_network_response(&wire::CreateNetworkOutput {
                    network_id: Some(network.id.clone()),
                    member_id: Some(member.id.clone()),
                })
            }
            Err(e) => mb_error_response(&e),
        }
    }

    async fn handle_get_network(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_network_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let state = state.read().await;
        match state.get_network(&input.network_id) {
            Ok(network) => wire::serialize_get_network_response(&wire::GetNetworkOutput {
                network: Some(network_to_wire(network)),
            }),
            Err(e) => mb_error_response(&e),
        }
    }

    async fn handle_list_networks(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_networks_request(request, labels, query) {
            return rest_json_error(400, "InvalidRequestException", &e);
        }
        let state = state.read().await;
        let networks = state.list_networks();
        let summaries: Vec<wire::NetworkSummary> =
            networks.iter().map(|n| network_to_summary(n)).collect();
        wire::serialize_list_networks_response(&wire::ListNetworksOutput {
            networks: Some(summaries),
            next_token: None,
        })
    }

    /// DeleteNetwork is not modelled in the Smithy schema as a wire operation here; call
    /// state directly by path label.
    async fn handle_delete_network(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        network_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_network(network_id) {
            Ok(()) => MockResponse::rest_json(200, "{}".to_string()),
            Err(e) => mb_error_response(&e),
        }
    }

    async fn handle_get_member(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_member_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let state = state.read().await;
        match state.get_member(&input.network_id, &input.member_id) {
            Ok(member) => wire::serialize_get_member_response(&wire::GetMemberOutput {
                member: Some(member_to_wire(member)),
            }),
            Err(e) => mb_error_response(&e),
        }
    }

    async fn handle_list_members(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_members_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let state = state.read().await;
        match state.list_members(&input.network_id) {
            Ok(members) => {
                let summaries: Vec<wire::MemberSummary> =
                    members.iter().map(|m| member_to_summary(m)).collect();
                wire::serialize_list_members_response(&wire::ListMembersOutput {
                    members: Some(summaries),
                    next_token: None,
                })
            }
            Err(e) => mb_error_response(&e),
        }
    }

    async fn handle_create_member(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_member_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        if input.invitation_id.is_empty() {
            return rest_json_error(400, "InvalidRequestException", "Missing 'InvitationId'");
        }
        let name = if input.member_configuration.name.is_empty() {
            "unnamed"
        } else {
            input.member_configuration.name.as_str()
        };
        let description = input.member_configuration.description.as_deref();
        // CreateMemberInput in this Smithy model does not carry top-level Tags;
        // tags on the member come through MemberConfiguration.tags.
        let tags = input.member_configuration.tags.clone().unwrap_or_default();

        let mut state = state.write().await;
        match state.create_member(
            &input.network_id,
            &input.invitation_id,
            name,
            description,
            tags,
            account_id,
            region,
        ) {
            Ok(member) => wire::serialize_create_member_response(&wire::CreateMemberOutput {
                member_id: Some(member.id.clone()),
            }),
            Err(e) => mb_error_response(&e),
        }
    }

    async fn handle_delete_member(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_member_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let mut state = state.write().await;
        match state.delete_member(&input.network_id, &input.member_id) {
            Ok(()) => wire::serialize_delete_member_response(&wire::DeleteMemberOutput {}),
            Err(e) => mb_error_response(&e),
        }
    }

    async fn handle_list_proposal_votes(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_proposal_votes_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let state = state.read().await;
        match state.list_proposal_votes(&input.network_id, &input.proposal_id) {
            Ok(votes) => {
                let summaries: Vec<wire::VoteSummary> = votes
                    .iter()
                    .map(|v| wire::VoteSummary {
                        member_id: Some(v.member_id.clone()),
                        member_name: Some(v.member_name.clone()),
                        vote: Some(v.vote.clone()),
                    })
                    .collect();
                wire::serialize_list_proposal_votes_response(&wire::ListProposalVotesOutput {
                    proposal_votes: Some(summaries),
                    next_token: None,
                })
            }
            Err(e) => mb_error_response(&e),
        }
    }

    // --- Node handlers ---
    // Note: The AWS SDK for managed blockchain uses these URL patterns:
    //   POST   /networks/{networkId}/nodes             (memberId in request body)
    //   GET    /networks/{networkId}/nodes             (memberId in query param "memberId")
    //   GET    /networks/{networkId}/nodes/{nodeId}    (memberId in query param "memberId")
    //   DELETE /networks/{networkId}/nodes/{nodeId}    (memberId in query param "memberId")
    //   PATCH  /networks/{networkId}/nodes/{nodeId}    (memberId in query param "memberId")

    async fn handle_create_node(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_node_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let member_id = match input.member_id.as_deref() {
            Some(m) if !m.is_empty() => m,
            _ => return rest_json_error(400, "InvalidRequestException", "Missing 'MemberId'"),
        };
        let instance_type = if input.node_configuration.instance_type.is_empty() {
            "bc.t3.small"
        } else {
            input.node_configuration.instance_type.as_str()
        };
        let availability_zone = input.node_configuration.availability_zone.as_deref();
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_node(
            &input.network_id,
            member_id,
            instance_type,
            availability_zone,
            tags,
            account_id,
            region,
        ) {
            Ok(node) => wire::serialize_create_node_response(&wire::CreateNodeOutput {
                node_id: Some(node.id.clone()),
            }),
            Err(e) => mb_error_response(&e),
        }
    }

    async fn handle_get_node(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_node_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let member_id = input.member_id.unwrap_or_default();
        let state = state.read().await;
        match state.get_node(&input.network_id, &member_id, &input.node_id) {
            Ok(node) => wire::serialize_get_node_response(&wire::GetNodeOutput {
                node: Some(node_to_wire(node)),
            }),
            Err(e) => mb_error_response(&e),
        }
    }

    async fn handle_delete_node(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_node_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let member_id = input.member_id.unwrap_or_default();
        let mut state = state.write().await;
        match state.delete_node(&input.network_id, &member_id, &input.node_id) {
            Ok(()) => wire::serialize_delete_node_response(&wire::DeleteNodeOutput {}),
            Err(e) => mb_error_response(&e),
        }
    }

    async fn handle_list_nodes(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_nodes_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let member_id = input.member_id.unwrap_or_default();
        let state = state.read().await;
        match state.list_nodes(&input.network_id, &member_id) {
            Ok(nodes) => {
                let summaries: Vec<wire::NodeSummary> = nodes
                    .iter()
                    .map(|n| wire::NodeSummary {
                        id: Some(n.id.clone()),
                        arn: Some(n.arn.clone()),
                        status: Some(n.status.clone()),
                        instance_type: Some(n.instance_type.clone()),
                        availability_zone: n.availability_zone.clone(),
                        creation_date: Some(n.creation_date.to_rfc3339()),
                    })
                    .collect();
                wire::serialize_list_nodes_response(&wire::ListNodesOutput {
                    nodes: Some(summaries),
                    next_token: None,
                })
            }
            Err(e) => mb_error_response(&e),
        }
    }

    async fn handle_update_member(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_member_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let mut state = state.write().await;
        match state.update_member(&input.network_id, &input.member_id) {
            Ok(()) => wire::serialize_update_member_response(&wire::UpdateMemberOutput {}),
            Err(e) => mb_error_response(&e),
        }
    }

    async fn handle_update_node(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_node_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let member_id = input.member_id.unwrap_or_default();
        let mut state = state.write().await;
        match state.update_node(&input.network_id, &member_id, &input.node_id) {
            Ok(()) => wire::serialize_update_node_response(&wire::UpdateNodeOutput {}),
            Err(e) => mb_error_response(&e),
        }
    }

    // --- Accessor handlers ---

    async fn handle_create_accessor(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_accessor_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        if input.accessor_type.is_empty() {
            return rest_json_error(400, "InvalidRequestException", "Missing 'AccessorType'");
        }
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_accessor(
            &input.accessor_type,
            input.network_type.as_deref(),
            tags,
            account_id,
            region,
        ) {
            Ok(accessor) => wire::serialize_create_accessor_response(&wire::CreateAccessorOutput {
                accessor_id: Some(accessor.id.clone()),
                billing_token: Some(accessor.billing_token.clone()),
                network_type: Some(accessor.network_type.clone()),
            }),
            Err(e) => mb_error_response(&e),
        }
    }

    async fn handle_get_accessor(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_accessor_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let state = state.read().await;
        match state.get_accessor(&input.accessor_id) {
            Ok(accessor) => wire::serialize_get_accessor_response(&wire::GetAccessorOutput {
                accessor: Some(accessor_to_wire(accessor)),
            }),
            Err(e) => mb_error_response(&e),
        }
    }

    async fn handle_delete_accessor(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_accessor_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let mut state = state.write().await;
        match state.delete_accessor(&input.accessor_id) {
            Ok(()) => wire::serialize_delete_accessor_response(&wire::DeleteAccessorOutput {}),
            Err(e) => mb_error_response(&e),
        }
    }

    async fn handle_list_accessors(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_accessors_request(request, labels, query) {
            return rest_json_error(400, "InvalidRequestException", &e);
        }
        let state = state.read().await;
        let accessors = state.list_accessors();
        let summaries: Vec<wire::AccessorSummary> = accessors
            .iter()
            .map(|a| wire::AccessorSummary {
                id: Some(a.id.clone()),
                arn: Some(a.arn.clone()),
                status: Some(a.status.clone()),
                r#type: Some(a.accessor_type.clone()),
                network_type: Some(a.network_type.clone()),
                creation_date: Some(a.creation_date.to_rfc3339()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_accessors_response(&wire::ListAccessorsOutput {
            accessors: Some(summaries),
            next_token: None,
        })
    }

    // --- Proposal handlers ---

    async fn handle_create_proposal(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_proposal_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        if input.member_id.is_empty() {
            return rest_json_error(400, "InvalidRequestException", "Missing 'MemberId'");
        }
        let tags = input.tags.unwrap_or_default();
        let actions = wire_actions_to_state(&input.actions);

        let mut state = state.write().await;
        match state.create_proposal(
            &input.network_id,
            &input.member_id,
            input.description.as_deref(),
            actions,
            tags,
            account_id,
            region,
        ) {
            Ok(proposal) => wire::serialize_create_proposal_response(&wire::CreateProposalOutput {
                proposal_id: Some(proposal.id.clone()),
            }),
            Err(e) => mb_error_response(&e),
        }
    }

    async fn handle_get_proposal(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_proposal_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let state = state.read().await;
        match state.get_proposal(&input.network_id, &input.proposal_id) {
            Ok(proposal) => wire::serialize_get_proposal_response(&wire::GetProposalOutput {
                proposal: Some(proposal_to_wire(proposal)),
            }),
            Err(e) => mb_error_response(&e),
        }
    }

    async fn handle_list_proposals(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_proposals_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let state = state.read().await;
        match state.list_proposals(&input.network_id) {
            Ok(proposals) => {
                let summaries: Vec<wire::ProposalSummary> = proposals
                    .iter()
                    .map(|p| wire::ProposalSummary {
                        proposal_id: Some(p.id.clone()),
                        description: p.description.clone(),
                        proposed_by_member_id: Some(p.member_id.clone()),
                        proposed_by_member_name: Some(p.member_name.clone()),
                        status: Some(p.status.clone()),
                        creation_date: Some(p.creation_date.to_rfc3339()),
                        expiration_date: Some(p.expiration_date.to_rfc3339()),
                        arn: Some(p.arn.clone()),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_proposals_response(&wire::ListProposalsOutput {
                    proposals: Some(summaries),
                    next_token: None,
                })
            }
            Err(e) => mb_error_response(&e),
        }
    }

    async fn handle_vote_on_proposal(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_vote_on_proposal_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        if input.voter_member_id.is_empty() {
            return rest_json_error(400, "InvalidRequestException", "Missing 'VoterMemberId'");
        }
        if input.vote.is_empty() {
            return rest_json_error(400, "InvalidRequestException", "Missing 'Vote'");
        }

        let mut state = state.write().await;
        match state.vote_on_proposal(
            &input.network_id,
            &input.proposal_id,
            &input.voter_member_id,
            &input.vote,
        ) {
            Ok(()) => wire::serialize_vote_on_proposal_response(&wire::VoteOnProposalOutput {}),
            Err(e) => mb_error_response(&e),
        }
    }

    // --- Invitation handlers ---

    async fn handle_list_invitations(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_invitations_request(request, labels, query) {
            return rest_json_error(400, "InvalidRequestException", &e);
        }
        let state = state.read().await;
        let invitations = state.list_invitations();
        let items: Vec<wire::Invitation> = invitations
            .iter()
            .map(|i| {
                // Find the network for this invitation to build the summary
                let network_summary =
                    state
                        .networks
                        .get(&i.network_id)
                        .map(|n| wire::NetworkSummary {
                            id: Some(n.id.clone()),
                            name: Some(n.name.clone()),
                            framework: Some(n.framework.clone()),
                            framework_version: Some(n.framework_version.clone()),
                            status: Some(n.status.clone()),
                            creation_date: Some(n.creation_date.to_rfc3339()),
                            arn: Some(n.arn.clone()),
                            ..Default::default()
                        });
                wire::Invitation {
                    invitation_id: Some(i.invitation_id.clone()),
                    status: Some(i.status.clone()),
                    creation_date: Some(i.creation_date.to_rfc3339()),
                    expiration_date: Some(i.expiration_date.to_rfc3339()),
                    arn: Some(i.arn.clone()),
                    network_summary,
                    ..Default::default()
                }
            })
            .collect();
        wire::serialize_list_invitations_response(&wire::ListInvitationsOutput {
            invitations: Some(items),
            next_token: None,
        })
    }

    async fn handle_reject_invitation(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_reject_invitation_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let mut state = state.write().await;
        match state.reject_invitation(&input.invitation_id) {
            Ok(()) => wire::serialize_reject_invitation_response(&wire::RejectInvitationOutput {}),
            Err(e) => mb_error_response(&e),
        }
    }

    // --- Tag handlers ---

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let state = state.read().await;
        match state.list_tags_for_resource(&input.resource_arn) {
            Ok(tags) => {
                let tags_opt = if tags.is_empty() { None } else { Some(tags) };
                wire::serialize_list_tags_for_resource_response(
                    &wire::ListTagsForResourceResponse { tags: tags_opt },
                )
            }
            Err(e) => mb_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, input.tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => mb_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ManagedBlockchainState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, &input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => mb_error_response(&e),
        }
    }
}

// --- Wire conversion helpers ---

fn network_to_wire(network: &crate::types::Network) -> wire::Network {
    wire::Network {
        id: Some(network.id.clone()),
        name: Some(network.name.clone()),
        description: network.description.clone(),
        framework: Some(network.framework.clone()),
        framework_version: Some(network.framework_version.clone()),
        status: Some(network.status.clone()),
        creation_date: Some(network.creation_date.to_rfc3339()),
        arn: Some(network.arn.clone()),
        tags: if network.tags.is_empty() {
            None
        } else {
            Some(network.tags.clone())
        },
        ..Default::default()
    }
}

fn network_to_summary(network: &crate::types::Network) -> wire::NetworkSummary {
    wire::NetworkSummary {
        id: Some(network.id.clone()),
        name: Some(network.name.clone()),
        framework: Some(network.framework.clone()),
        framework_version: Some(network.framework_version.clone()),
        status: Some(network.status.clone()),
        creation_date: Some(network.creation_date.to_rfc3339()),
        arn: Some(network.arn.clone()),
        ..Default::default()
    }
}

fn member_to_wire(member: &crate::types::Member) -> wire::Member {
    wire::Member {
        id: Some(member.id.clone()),
        name: Some(member.name.clone()),
        description: member.description.clone(),
        network_id: Some(member.network_id.clone()),
        status: Some(member.status.clone()),
        creation_date: Some(member.creation_date.to_rfc3339()),
        arn: Some(member.arn.clone()),
        tags: if member.tags.is_empty() {
            None
        } else {
            Some(member.tags.clone())
        },
        ..Default::default()
    }
}

fn member_to_summary(member: &crate::types::Member) -> wire::MemberSummary {
    wire::MemberSummary {
        id: Some(member.id.clone()),
        name: Some(member.name.clone()),
        status: Some(member.status.clone()),
        creation_date: Some(member.creation_date.to_rfc3339()),
        arn: Some(member.arn.clone()),
        ..Default::default()
    }
}

fn node_to_wire(node: &crate::types::Node) -> wire::Node {
    wire::Node {
        id: Some(node.id.clone()),
        network_id: Some(node.network_id.clone()),
        member_id: Some(node.member_id.clone()),
        instance_type: Some(node.instance_type.clone()),
        availability_zone: node.availability_zone.clone(),
        status: Some(node.status.clone()),
        creation_date: Some(node.creation_date.to_rfc3339()),
        arn: Some(node.arn.clone()),
        tags: if node.tags.is_empty() {
            None
        } else {
            Some(node.tags.clone())
        },
        ..Default::default()
    }
}

fn accessor_to_wire(accessor: &crate::types::Accessor) -> wire::Accessor {
    wire::Accessor {
        id: Some(accessor.id.clone()),
        arn: Some(accessor.arn.clone()),
        billing_token: Some(accessor.billing_token.clone()),
        status: Some(accessor.status.clone()),
        r#type: Some(accessor.accessor_type.clone()),
        network_type: Some(accessor.network_type.clone()),
        creation_date: Some(accessor.creation_date.to_rfc3339()),
        tags: if accessor.tags.is_empty() {
            None
        } else {
            Some(accessor.tags.clone())
        },
    }
}

fn proposal_to_wire(proposal: &crate::types::Proposal) -> wire::Proposal {
    let actions = wire::ProposalActions {
        invitations: if proposal.actions.invitations.is_empty() {
            None
        } else {
            Some(
                proposal
                    .actions
                    .invitations
                    .iter()
                    .map(|i| wire::InviteAction {
                        principal: i.principal.clone(),
                    })
                    .collect(),
            )
        },
        removals: if proposal.actions.removals.is_empty() {
            None
        } else {
            Some(
                proposal
                    .actions
                    .removals
                    .iter()
                    .map(|r| wire::RemoveAction {
                        member_id: r.member_id.clone(),
                    })
                    .collect(),
            )
        },
    };

    wire::Proposal {
        proposal_id: Some(proposal.id.clone()),
        network_id: Some(proposal.network_id.clone()),
        description: proposal.description.clone(),
        proposed_by_member_id: Some(proposal.member_id.clone()),
        proposed_by_member_name: Some(proposal.member_name.clone()),
        status: Some(proposal.status.clone()),
        creation_date: Some(proposal.creation_date.to_rfc3339()),
        expiration_date: Some(proposal.expiration_date.to_rfc3339()),
        yes_vote_count: Some(proposal.yes_vote_count),
        no_vote_count: Some(proposal.no_vote_count),
        outstanding_vote_count: Some(proposal.outstanding_vote_count),
        arn: Some(proposal.arn.clone()),
        actions: Some(actions),
        tags: if proposal.tags.is_empty() {
            None
        } else {
            Some(proposal.tags.clone())
        },
    }
}

fn wire_actions_to_state(actions: &wire::ProposalActions) -> crate::types::ProposalActions {
    let mut out = crate::types::ProposalActions::default();
    if let Some(invitations) = actions.invitations.as_ref() {
        for inv in invitations {
            out.invitations.push(crate::types::InviteAction {
                principal: inv.principal.clone(),
            });
        }
    }
    if let Some(removals) = actions.removals.as_ref() {
        for rem in removals {
            out.removals.push(crate::types::RemoveAction {
                member_id: rem.member_id.clone(),
            });
        }
    }
    out
}

// --- Utility functions ---

fn extract_path_and_query(uri: &str) -> (String, String) {
    let after_host = if let Some(idx) = uri.find("amazonaws.com") {
        &uri[idx + "amazonaws.com".len()..]
    } else {
        uri
    };

    if let Some(q) = after_host.find('?') {
        (after_host[..q].to_string(), after_host[q + 1..].to_string())
    } else {
        (after_host.to_string(), String::new())
    }
}

fn parse_query_string(query: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for param in query.split('&').filter(|s| !s.is_empty()) {
        if let Some((k, v)) = param.split_once('=') {
            map.insert(urldecode(k), urldecode(v));
        } else {
            map.insert(urldecode(param), String::new());
        }
    }
    map
}

fn urldecode(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                result.push(byte as char);
            } else {
                result.push('%');
                result.push_str(&hex);
            }
        } else if c == '+' {
            result.push(' ');
        } else {
            result.push(c);
        }
    }
    result
}

fn mb_error_response(err: &ManagedBlockchainError) -> MockResponse {
    let (status, error_type) = match err {
        ManagedBlockchainError::NetworkNotFound { .. } => (404, "ResourceNotFoundException"),
        ManagedBlockchainError::MemberNotFound { .. } => (404, "ResourceNotFoundException"),
        ManagedBlockchainError::NodeNotFound { .. } => (404, "ResourceNotFoundException"),
        ManagedBlockchainError::AccessorNotFound { .. } => (404, "ResourceNotFoundException"),
        ManagedBlockchainError::InvitationNotFound { .. } => (404, "ResourceNotFoundException"),
        ManagedBlockchainError::ProposalNotFound { .. } => (404, "ResourceNotFoundException"),
        ManagedBlockchainError::ResourceNotFound { .. } => (404, "ResourceNotFoundException"),
        ManagedBlockchainError::MemberAlreadyVoted { .. } => (400, "InvalidRequestException"),
        ManagedBlockchainError::InvitationNotPending { .. } => (400, "InvalidRequestException"),
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
