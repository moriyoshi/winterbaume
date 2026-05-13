use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id, extract_path,
};

use crate::state::{VpcLatticeError, VpcLatticeState};
use crate::views::VpcLatticeStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct VpcLatticeService {
    pub(crate) state: Arc<BackendState<VpcLatticeState>>,
    pub(crate) notifier: StateChangeNotifier<VpcLatticeStateView>,
}

impl VpcLatticeService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for VpcLatticeService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for VpcLatticeService {
    fn service_name(&self) -> &str {
        "vpc-lattice"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://vpc-lattice\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl VpcLatticeService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let query_string = extract_query_string(&request.uri);
        let query: HashMap<String, String> = parse_query_string(&query_string);
        let method = request.method.as_str();

        // URL-decode each path segment so ARN identifiers like
        // `arn%3Aaws%3A...` are recognized as the actual ARN.
        let segments: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(url_decode)
            .collect();
        let segments_ref: Vec<&str> = segments.iter().map(|s| s.as_str()).collect();

        // Validate the body is well-formed JSON up-front; the typed deserialisers in
        // `wire` re-parse the bytes per operation.
        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return rest_json_error(400, "ValidationException", "Invalid JSON");
        }

        let response = match (method, segments_ref.as_slice()) {
            // ── ServiceNetwork ──────────────────────────────────
            ("POST", ["servicenetworks"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_create_service_network(
                    &state, &request, labels, &query, account_id, &region,
                )
                .await
            }
            ("GET", ["servicenetworks", identifier]) => {
                self.handle_get_service_network(&state, identifier).await
            }
            ("DELETE", ["servicenetworks", identifier]) => {
                self.handle_delete_service_network(&state, identifier).await
            }
            ("GET", ["servicenetworks"]) => self.handle_list_service_networks(&state).await,

            // ── AccessLogSubscription ───────────────────────────
            ("POST", ["accesslogsubscriptions"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_create_access_log_subscription(
                    &state, &request, labels, &query, account_id, &region,
                )
                .await
            }
            ("GET", ["accesslogsubscriptions", identifier]) => {
                self.handle_get_access_log_subscription(&state, identifier)
                    .await
            }
            ("DELETE", ["accesslogsubscriptions", identifier]) => {
                self.handle_delete_access_log_subscription(&state, identifier)
                    .await
            }
            ("GET", ["accesslogsubscriptions"]) => {
                self.handle_list_access_log_subscriptions(&state, &query)
                    .await
            }

            // ── ServiceNetworkServiceAssociation ────────────────
            ("POST", ["servicenetworkserviceassociations"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_create_service_network_service_association(
                    &state, &request, labels, &query, account_id, &region,
                )
                .await
            }
            ("GET", ["servicenetworkserviceassociations", identifier]) => {
                self.handle_get_service_network_service_association(&state, identifier)
                    .await
            }
            ("DELETE", ["servicenetworkserviceassociations", identifier]) => {
                self.handle_delete_service_network_service_association(&state, identifier)
                    .await
            }
            ("GET", ["servicenetworkserviceassociations"]) => {
                self.handle_list_service_network_service_associations(&state, &query)
                    .await
            }

            // ── ServiceNetworkVpcAssociation ────────────────────
            ("POST", ["servicenetworkvpcassociations"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_create_service_network_vpc_association(
                    &state, &request, labels, &query, account_id, &region,
                )
                .await
            }
            ("GET", ["servicenetworkvpcassociations", identifier]) => {
                self.handle_get_service_network_vpc_association(&state, identifier)
                    .await
            }
            ("DELETE", ["servicenetworkvpcassociations", identifier]) => {
                self.handle_delete_service_network_vpc_association(&state, identifier)
                    .await
            }
            ("GET", ["servicenetworkvpcassociations"]) => {
                self.handle_list_service_network_vpc_associations(&state, &query)
                    .await
            }

            // ── TargetGroup ─────────────────────────────────────
            ("POST", ["targetgroups"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_create_target_group(
                    &state, &request, labels, &query, account_id, &region,
                )
                .await
            }
            ("GET", ["targetgroups", identifier]) => {
                self.handle_get_target_group(&state, identifier).await
            }
            ("DELETE", ["targetgroups", identifier]) => {
                self.handle_delete_target_group(&state, identifier).await
            }
            ("GET", ["targetgroups"]) => self.handle_list_target_groups(&state).await,

            // ── Targets (Register/Deregister/List) ──────────────
            ("POST", ["targetgroups", tg_id, "registertargets"]) => {
                let labels: &[(&str, &str)] = &[("targetGroupIdentifier", tg_id)];
                self.handle_register_targets(&state, &request, labels, &query)
                    .await
            }
            ("POST", ["targetgroups", tg_id, "deregistertargets"]) => {
                let labels: &[(&str, &str)] = &[("targetGroupIdentifier", tg_id)];
                self.handle_deregister_targets(&state, &request, labels, &query)
                    .await
            }
            ("POST", ["targetgroups", tg_id, "listtargets"]) => {
                self.handle_list_targets(&state, tg_id).await
            }

            // ── Service ─────────────────────────────────────────
            ("POST", ["services"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_create_service(&state, &request, labels, &query, account_id, &region)
                    .await
            }
            ("GET", ["services", identifier]) => self.handle_get_service(&state, identifier).await,
            ("DELETE", ["services", identifier]) => {
                self.handle_delete_service(&state, identifier).await
            }
            ("GET", ["services"]) => self.handle_list_services(&state).await,

            // ── AuthPolicy ───────────────────────────────────────
            ("PUT", ["authpolicy", resource_identifier]) => {
                let labels: &[(&str, &str)] = &[("resourceIdentifier", resource_identifier)];
                self.handle_put_auth_policy(&state, &request, labels, &query)
                    .await
            }
            ("GET", ["authpolicy", resource_identifier]) => {
                self.handle_get_auth_policy(&state, resource_identifier)
                    .await
            }
            ("DELETE", ["authpolicy", resource_identifier]) => {
                self.handle_delete_auth_policy(&state, resource_identifier)
                    .await
            }

            // ── ResourcePolicy ───────────────────────────────────
            ("PUT", ["resourcepolicy", resource_arn]) => {
                let labels: &[(&str, &str)] = &[("resourceArn", resource_arn)];
                self.handle_put_resource_policy(&state, &request, labels, &query)
                    .await
            }
            ("GET", ["resourcepolicy", resource_arn]) => {
                self.handle_get_resource_policy(&state, resource_arn).await
            }
            ("DELETE", ["resourcepolicy", resource_arn]) => {
                self.handle_delete_resource_policy(&state, resource_arn)
                    .await
            }

            // ── Tags ─────────────────────────────────────────────
            ("GET", ["tags", resource_arn]) => {
                self.handle_list_tags_for_resource(&state, resource_arn)
                    .await
            }
            ("POST", ["tags", resource_arn]) => {
                let labels: &[(&str, &str)] = &[("resourceArn", resource_arn)];
                self.handle_tag_resource(&state, &request, labels, &query)
                    .await
            }
            ("DELETE", ["tags", resource_arn]) => {
                self.handle_untag_resource(&state, resource_arn, &query_string)
                    .await
            }

            // ── UpdateAccessLogSubscription ──────────────────────
            ("PATCH", ["accesslogsubscriptions", identifier]) => {
                let labels: &[(&str, &str)] = &[("accessLogSubscriptionIdentifier", identifier)];
                self.handle_update_access_log_subscription(&state, &request, labels, &query)
                    .await
            }

            // ── UpdateService ─────────────────────────────────────
            ("PATCH", ["services", identifier]) => {
                let labels: &[(&str, &str)] = &[("serviceIdentifier", identifier)];
                self.handle_update_service(&state, &request, labels, &query)
                    .await
            }

            // ── UpdateServiceNetwork ──────────────────────────────
            ("PATCH", ["servicenetworks", identifier]) => {
                let labels: &[(&str, &str)] = &[("serviceNetworkIdentifier", identifier)];
                self.handle_update_service_network(&state, &request, labels, &query)
                    .await
            }

            // ── UpdateTargetGroup ─────────────────────────────────
            ("PATCH", ["targetgroups", identifier]) => {
                let labels: &[(&str, &str)] = &[("targetGroupIdentifier", identifier)];
                self.handle_update_target_group(&state, &request, labels, &query)
                    .await
            }

            // ── UpdateServiceNetworkVpcAssociation ────────────────
            ("PATCH", ["servicenetworkvpcassociations", identifier]) => {
                let labels: &[(&str, &str)] =
                    &[("serviceNetworkVpcAssociationIdentifier", identifier)];
                self.handle_update_service_network_vpc_association(&state, &request, labels, &query)
                    .await
            }

            // ── Listener ─────────────────────────────────────────
            ("POST", ["services", svc_id, "listeners"]) => {
                let labels: &[(&str, &str)] = &[("serviceIdentifier", svc_id)];
                self.handle_create_listener(&state, &request, labels, &query, account_id, &region)
                    .await
            }
            ("GET", ["services", svc_id, "listeners", listener_id]) => {
                self.handle_get_listener(&state, svc_id, listener_id).await
            }
            ("DELETE", ["services", svc_id, "listeners", listener_id]) => {
                self.handle_delete_listener(&state, svc_id, listener_id)
                    .await
            }
            ("GET", ["services", svc_id, "listeners"]) => {
                self.handle_list_listeners(&state, svc_id).await
            }
            ("PATCH", ["services", svc_id, "listeners", listener_id]) => {
                let labels: &[(&str, &str)] = &[
                    ("serviceIdentifier", svc_id),
                    ("listenerIdentifier", listener_id),
                ];
                self.handle_update_listener(&state, &request, labels, &query)
                    .await
            }

            // ── Rule ─────────────────────────────────────────────
            ("POST", ["services", svc_id, "listeners", listener_id, "rules"]) => {
                let labels: &[(&str, &str)] = &[
                    ("serviceIdentifier", svc_id),
                    ("listenerIdentifier", listener_id),
                ];
                self.handle_create_rule(&state, &request, labels, &query, account_id, &region)
                    .await
            }
            (
                "GET",
                [
                    "services",
                    svc_id,
                    "listeners",
                    listener_id,
                    "rules",
                    rule_id,
                ],
            ) => {
                self.handle_get_rule(&state, svc_id, listener_id, rule_id)
                    .await
            }
            (
                "DELETE",
                [
                    "services",
                    svc_id,
                    "listeners",
                    listener_id,
                    "rules",
                    rule_id,
                ],
            ) => {
                self.handle_delete_rule(&state, svc_id, listener_id, rule_id)
                    .await
            }
            ("GET", ["services", svc_id, "listeners", listener_id, "rules"]) => {
                self.handle_list_rules(&state, svc_id, listener_id).await
            }
            (
                "PATCH",
                [
                    "services",
                    svc_id,
                    "listeners",
                    listener_id,
                    "rules",
                    rule_id,
                ],
            ) => {
                let labels: &[(&str, &str)] = &[
                    ("serviceIdentifier", svc_id),
                    ("listenerIdentifier", listener_id),
                    ("ruleIdentifier", rule_id),
                ];
                self.handle_update_rule(&state, &request, labels, &query)
                    .await
            }

            // ── BatchUpdateRule ──────────────────────────────────────
            ("PATCH", ["services", svc_id, "listeners", listener_id, "rules"]) => {
                let labels: &[(&str, &str)] = &[
                    ("serviceIdentifier", svc_id),
                    ("listenerIdentifier", listener_id),
                ];
                self.handle_batch_update_rule(&state, &request, labels, &query)
                    .await
            }

            // ── ResourceConfiguration ────────────────────────────────
            ("POST", ["resourceconfigurations"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_create_resource_configuration(
                    &state, &request, labels, &query, account_id, &region,
                )
                .await
            }
            ("GET", ["resourceconfigurations", id]) => {
                self.handle_get_resource_configuration(&state, id).await
            }
            ("DELETE", ["resourceconfigurations", id]) => {
                self.handle_delete_resource_configuration(&state, id).await
            }
            ("GET", ["resourceconfigurations"]) => {
                self.handle_list_resource_configurations(&state).await
            }
            ("PATCH", ["resourceconfigurations", id]) => {
                let labels: &[(&str, &str)] = &[("resourceConfigurationIdentifier", id)];
                self.handle_update_resource_configuration(&state, &request, labels, &query)
                    .await
            }

            // ── ResourceGateway ──────────────────────────────────────
            ("POST", ["resourcegateways"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_create_resource_gateway(
                    &state, &request, labels, &query, account_id, &region,
                )
                .await
            }
            ("GET", ["resourcegateways", id]) => self.handle_get_resource_gateway(&state, id).await,
            ("DELETE", ["resourcegateways", id]) => {
                self.handle_delete_resource_gateway(&state, id).await
            }
            ("GET", ["resourcegateways"]) => self.handle_list_resource_gateways(&state).await,
            ("PATCH", ["resourcegateways", id]) => {
                let labels: &[(&str, &str)] = &[("resourceGatewayIdentifier", id)];
                self.handle_update_resource_gateway(&state, &request, labels, &query)
                    .await
            }

            // ── ServiceNetworkResourceAssociation ────────────────────
            ("POST", ["servicenetworkresourceassociations"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_create_service_network_resource_association(
                    &state, &request, labels, &query, account_id, &region,
                )
                .await
            }
            ("GET", ["servicenetworkresourceassociations", id]) => {
                self.handle_get_service_network_resource_association(&state, id)
                    .await
            }
            ("DELETE", ["servicenetworkresourceassociations", id]) => {
                self.handle_delete_service_network_resource_association(&state, id)
                    .await
            }
            ("GET", ["servicenetworkresourceassociations"]) => {
                self.handle_list_service_network_resource_associations(&state, &query)
                    .await
            }

            // ── DomainVerification ───────────────────────────────────
            ("POST", ["domainverifications"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_start_domain_verification(
                    &state, &request, labels, &query, account_id, &region,
                )
                .await
            }
            ("GET", ["domainverifications", id]) => {
                self.handle_get_domain_verification(&state, id).await
            }
            ("DELETE", ["domainverifications", id]) => {
                self.handle_delete_domain_verification(&state, id).await
            }
            ("GET", ["domainverifications"]) => self.handle_list_domain_verifications(&state).await,

            // ── ResourceEndpointAssociation (stub) ───────────────────
            ("GET", ["resourceendpointassociations"]) => {
                self.handle_list_resource_endpoint_associations(&state)
                    .await
            }
            ("DELETE", ["resourceendpointassociations", id]) => {
                self.handle_delete_resource_endpoint_association(&state, id)
                    .await
            }

            // ── ServiceNetworkVpcEndpointAssociation (stub) ──────────
            ("GET", ["servicenetworkvpcendpointassociations"]) => {
                self.handle_list_service_network_vpc_endpoint_associations(&state)
                    .await
            }

            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // ── ServiceNetwork handlers ─────────────────────────────────────

    async fn handle_create_service_network(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_service_network_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'name'");
        }
        let name = input.name.as_str();
        let auth_type = input.auth_type.as_deref();
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_service_network(name, auth_type, account_id, region, tags) {
            Ok(sn) => wire::serialize_create_service_network_response(
                &wire::CreateServiceNetworkResponse {
                    id: Some(sn.id.clone()),
                    name: Some(sn.name.clone()),
                    arn: Some(sn.arn.clone()),
                    auth_type: Some(sn.auth_type.clone()),
                    ..Default::default()
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_get_service_network(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        identifier: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_service_network(identifier) {
            Ok(sn) => {
                wire::serialize_get_service_network_response(&wire::GetServiceNetworkResponse {
                    id: Some(sn.id.clone()),
                    name: Some(sn.name.clone()),
                    arn: Some(sn.arn.clone()),
                    auth_type: Some(sn.auth_type.clone()),
                    created_at: Some(sn.created_at.to_rfc3339()),
                    last_updated_at: Some(sn.last_updated_at.to_rfc3339()),
                    number_of_associated_services: Some(sn.number_of_associated_services),
                    number_of_associated_v_p_cs: Some(sn.number_of_associated_v_p_cs),
                    ..Default::default()
                })
            }
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_delete_service_network(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        identifier: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_service_network(identifier) {
            Ok(()) => wire::serialize_delete_service_network_response(
                &wire::DeleteServiceNetworkResponse {},
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_list_service_networks(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let networks = state.list_service_networks();

        let items: Vec<wire::ServiceNetworkSummary> = networks
            .iter()
            .map(|sn| wire::ServiceNetworkSummary {
                id: Some(sn.id.clone()),
                name: Some(sn.name.clone()),
                arn: Some(sn.arn.clone()),
                created_at: Some(sn.created_at.to_rfc3339()),
                last_updated_at: Some(sn.last_updated_at.to_rfc3339()),
                number_of_associated_services: Some(sn.number_of_associated_services),
                number_of_associated_v_p_cs: Some(sn.number_of_associated_v_p_cs),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_service_networks_response(&wire::ListServiceNetworksResponse {
            items: Some(items),
            next_token: None,
        })
    }

    // ── AccessLogSubscription handlers ──────────────────────────────

    async fn handle_create_access_log_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_access_log_subscription_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_identifier.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'resourceIdentifier'");
        }
        if input.destination_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'destinationArn'");
        }
        let resource_identifier = input.resource_identifier.as_str();
        let destination_arn = input.destination_arn.as_str();
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_access_log_subscription(
            resource_identifier,
            destination_arn,
            account_id,
            region,
            tags,
        ) {
            Ok(sub) => wire::serialize_create_access_log_subscription_response(
                &wire::CreateAccessLogSubscriptionResponse {
                    id: Some(sub.id.clone()),
                    arn: Some(sub.arn.clone()),
                    resource_arn: Some(sub.resource_arn.clone()),
                    resource_id: Some(sub.resource_id.clone()),
                    destination_arn: Some(sub.destination_arn.clone()),
                    ..Default::default()
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_get_access_log_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        identifier: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_access_log_subscription(identifier) {
            Ok(sub) => wire::serialize_get_access_log_subscription_response(
                &wire::GetAccessLogSubscriptionResponse {
                    id: Some(sub.id.clone()),
                    arn: Some(sub.arn.clone()),
                    resource_arn: Some(sub.resource_arn.clone()),
                    resource_id: Some(sub.resource_id.clone()),
                    destination_arn: Some(sub.destination_arn.clone()),
                    created_at: Some(sub.created_at.to_rfc3339()),
                    last_updated_at: Some(sub.last_updated_at.to_rfc3339()),
                    ..Default::default()
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_delete_access_log_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        identifier: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_access_log_subscription(identifier) {
            Ok(()) => wire::serialize_delete_access_log_subscription_response(
                &wire::DeleteAccessLogSubscriptionResponse {},
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_list_access_log_subscriptions(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let resource_identifier = query
            .get("resourceIdentifier")
            .map(|s| s.as_str())
            .unwrap_or("");
        let max_results: Option<usize> = query.get("maxResults").and_then(|s| s.parse().ok());

        let state = state.read().await;
        let mut subs = state.list_access_log_subscriptions(resource_identifier);

        // Apply maxResults truncation
        if let Some(max) = max_results {
            subs.truncate(max);
        }

        let items: Vec<wire::AccessLogSubscriptionSummary> = subs
            .iter()
            .map(|s| wire::AccessLogSubscriptionSummary {
                id: Some(s.id.clone()),
                arn: Some(s.arn.clone()),
                resource_arn: Some(s.resource_arn.clone()),
                resource_id: Some(s.resource_id.clone()),
                destination_arn: Some(s.destination_arn.clone()),
                created_at: Some(s.created_at.to_rfc3339()),
                last_updated_at: Some(s.last_updated_at.to_rfc3339()),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_access_log_subscriptions_response(
            &wire::ListAccessLogSubscriptionsResponse {
                items: Some(items),
                next_token: None,
            },
        )
    }

    // ── ServiceNetworkServiceAssociation handlers ───────────────────

    async fn handle_create_service_network_service_association(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_service_network_service_association_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.service_network_identifier.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing 'serviceNetworkIdentifier'",
            );
        }
        if input.service_identifier.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'serviceIdentifier'");
        }
        let sn_id = input.service_network_identifier.as_str();
        let svc_id = input.service_identifier.as_str();
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state
            .create_service_network_service_association(sn_id, svc_id, account_id, region, tags)
        {
            Ok(assoc) => wire::serialize_create_service_network_service_association_response(
                &wire::CreateServiceNetworkServiceAssociationResponse {
                    id: Some(assoc.id.clone()),
                    arn: Some(assoc.arn.clone()),
                    status: Some(assoc.status.clone()),
                    ..Default::default()
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_get_service_network_service_association(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        identifier: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_service_network_service_association(identifier) {
            Ok(a) => wire::serialize_get_service_network_service_association_response(
                &wire::GetServiceNetworkServiceAssociationResponse {
                    id: Some(a.id.clone()),
                    arn: Some(a.arn.clone()),
                    status: Some(a.status.clone()),
                    service_network_id: Some(a.service_network_id.clone()),
                    service_network_arn: Some(a.service_network_arn.clone()),
                    service_network_name: Some(a.service_network_name.clone()),
                    service_id: Some(a.service_id.clone()),
                    service_arn: Some(a.service_arn.clone()),
                    service_name: Some(a.service_name.clone()),
                    created_at: Some(a.created_at.to_rfc3339()),
                    ..Default::default()
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_delete_service_network_service_association(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        identifier: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_service_network_service_association(identifier) {
            Ok(a) => wire::serialize_delete_service_network_service_association_response(
                &wire::DeleteServiceNetworkServiceAssociationResponse {
                    id: Some(a.id.clone()),
                    arn: Some(a.arn.clone()),
                    status: Some(a.status.clone()),
                    ..Default::default()
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_list_service_network_service_associations(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let sn_id = query.get("serviceNetworkIdentifier").map(|s| s.as_str());
        let svc_id = query.get("serviceIdentifier").map(|s| s.as_str());

        let state = state.read().await;
        let assocs = state.list_service_network_service_associations(sn_id, svc_id);

        let items: Vec<wire::ServiceNetworkServiceAssociationSummary> = assocs
            .iter()
            .map(|a| wire::ServiceNetworkServiceAssociationSummary {
                id: Some(a.id.clone()),
                arn: Some(a.arn.clone()),
                status: Some(a.status.clone()),
                service_network_id: Some(a.service_network_id.clone()),
                service_network_arn: Some(a.service_network_arn.clone()),
                service_network_name: Some(a.service_network_name.clone()),
                service_id: Some(a.service_id.clone()),
                service_arn: Some(a.service_arn.clone()),
                service_name: Some(a.service_name.clone()),
                created_at: Some(a.created_at.to_rfc3339()),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_service_network_service_associations_response(
            &wire::ListServiceNetworkServiceAssociationsResponse {
                items: Some(items),
                next_token: None,
            },
        )
    }

    // ── ServiceNetworkVpcAssociation handlers ───────────────────────
    // POST /servicenetworkvpcassociations - CreateServiceNetworkVpcAssociation
    async fn handle_create_service_network_vpc_association(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_service_network_vpc_association_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.service_network_identifier.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing 'serviceNetworkIdentifier'",
            );
        }
        if input.vpc_identifier.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'vpcIdentifier'");
        }
        let sn_id = input.service_network_identifier.as_str();
        let vpc_id = input.vpc_identifier.as_str();
        let security_group_ids = input.security_group_ids.unwrap_or_default();
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_service_network_vpc_association(
            sn_id,
            vpc_id,
            security_group_ids,
            account_id,
            region,
            tags,
        ) {
            Ok(assoc) => wire::serialize_create_service_network_vpc_association_response(
                &wire::CreateServiceNetworkVpcAssociationResponse {
                    id: Some(assoc.id.clone()),
                    arn: Some(assoc.arn.clone()),
                    status: Some(assoc.status.clone()),
                    security_group_ids: Some(assoc.security_group_ids.clone()),
                    ..Default::default()
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_get_service_network_vpc_association(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        identifier: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_service_network_vpc_association(identifier) {
            Ok(a) => wire::serialize_get_service_network_vpc_association_response(
                &wire::GetServiceNetworkVpcAssociationResponse {
                    id: Some(a.id.clone()),
                    arn: Some(a.arn.clone()),
                    status: Some(a.status.clone()),
                    service_network_id: Some(a.service_network_id.clone()),
                    service_network_arn: Some(a.service_network_arn.clone()),
                    service_network_name: Some(a.service_network_name.clone()),
                    vpc_id: Some(a.vpc_id.clone()),
                    security_group_ids: Some(a.security_group_ids.clone()),
                    created_at: Some(a.created_at.to_rfc3339()),
                    last_updated_at: Some(a.last_updated_at.to_rfc3339()),
                    ..Default::default()
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_delete_service_network_vpc_association(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        identifier: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_service_network_vpc_association(identifier) {
            Ok(a) => wire::serialize_delete_service_network_vpc_association_response(
                &wire::DeleteServiceNetworkVpcAssociationResponse {
                    id: Some(a.id.clone()),
                    arn: Some(a.arn.clone()),
                    status: Some(a.status.clone()),
                    ..Default::default()
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_list_service_network_vpc_associations(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let sn_id = query.get("serviceNetworkIdentifier").map(|s| s.as_str());
        let vpc_id = query.get("vpcIdentifier").map(|s| s.as_str());

        let state = state.read().await;
        let assocs = state.list_service_network_vpc_associations(sn_id, vpc_id);

        let items: Vec<wire::ServiceNetworkVpcAssociationSummary> = assocs
            .iter()
            .map(|a| wire::ServiceNetworkVpcAssociationSummary {
                id: Some(a.id.clone()),
                arn: Some(a.arn.clone()),
                status: Some(a.status.clone()),
                service_network_id: Some(a.service_network_id.clone()),
                service_network_arn: Some(a.service_network_arn.clone()),
                service_network_name: Some(a.service_network_name.clone()),
                vpc_id: Some(a.vpc_id.clone()),
                created_at: Some(a.created_at.to_rfc3339()),
                last_updated_at: Some(a.last_updated_at.to_rfc3339()),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_service_network_vpc_associations_response(
            &wire::ListServiceNetworkVpcAssociationsResponse {
                items: Some(items),
                next_token: None,
            },
        )
    }

    // ── TargetGroup handlers ────────────────────────────────────────

    async fn handle_create_target_group(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_target_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'name'");
        }
        if input.r#type.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'type'");
        }
        let name = input.name.as_str();
        let tg_type = input.r#type.as_str();

        let config = input.config.map(|c| crate::types::TargetGroupConfig {
            port: c.port,
            protocol: c.protocol,
            vpc_identifier: c.vpc_identifier,
            ip_address_type: c.ip_address_type,
            protocol_version: c.protocol_version,
            lambda_event_structure_version: c.lambda_event_structure_version,
        });

        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_target_group(name, tg_type, config, account_id, region, tags) {
            Ok(tg) => {
                wire::serialize_create_target_group_response(&wire::CreateTargetGroupResponse {
                    id: Some(tg.id.clone()),
                    arn: Some(tg.arn.clone()),
                    name: Some(tg.name.clone()),
                    r#type: Some(tg.target_group_type.clone()),
                    status: Some(tg.status.clone()),
                    config: tg.config.as_ref().map(|c| wire::TargetGroupConfig {
                        port: c.port,
                        protocol: c.protocol.clone(),
                        vpc_identifier: c.vpc_identifier.clone(),
                        ip_address_type: c.ip_address_type.clone(),
                        protocol_version: c.protocol_version.clone(),
                        lambda_event_structure_version: c.lambda_event_structure_version.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
            }
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_get_target_group(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        identifier: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_target_group(identifier) {
            Ok(tg) => wire::serialize_get_target_group_response(&wire::GetTargetGroupResponse {
                id: Some(tg.id.clone()),
                arn: Some(tg.arn.clone()),
                name: Some(tg.name.clone()),
                r#type: Some(tg.target_group_type.clone()),
                status: Some(tg.status.clone()),
                created_at: Some(tg.created_at.to_rfc3339()),
                last_updated_at: Some(tg.last_updated_at.to_rfc3339()),
                config: tg.config.as_ref().map(|c| wire::TargetGroupConfig {
                    port: c.port,
                    protocol: c.protocol.clone(),
                    vpc_identifier: c.vpc_identifier.clone(),
                    ip_address_type: c.ip_address_type.clone(),
                    protocol_version: c.protocol_version.clone(),
                    lambda_event_structure_version: c.lambda_event_structure_version.clone(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_delete_target_group(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        identifier: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_target_group(identifier) {
            Ok(tg) => {
                wire::serialize_delete_target_group_response(&wire::DeleteTargetGroupResponse {
                    id: Some(tg.id.clone()),
                    arn: Some(tg.arn.clone()),
                    status: Some(tg.status.clone()),
                    ..Default::default()
                })
            }
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_list_target_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let groups = state.list_target_groups();

        let items: Vec<wire::TargetGroupSummary> = groups
            .iter()
            .map(|tg| wire::TargetGroupSummary {
                id: Some(tg.id.clone()),
                arn: Some(tg.arn.clone()),
                name: Some(tg.name.clone()),
                r#type: Some(tg.target_group_type.clone()),
                status: Some(tg.status.clone()),
                created_at: Some(tg.created_at.to_rfc3339()),
                last_updated_at: Some(tg.last_updated_at.to_rfc3339()),
                port: tg.config.as_ref().and_then(|c| c.port),
                protocol: tg.config.as_ref().and_then(|c| c.protocol.clone()),
                vpc_identifier: tg.config.as_ref().and_then(|c| c.vpc_identifier.clone()),
                ip_address_type: tg.config.as_ref().and_then(|c| c.ip_address_type.clone()),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_target_groups_response(&wire::ListTargetGroupsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    // ── Targets handlers ────────────────────────────────────────────

    async fn handle_register_targets(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_register_targets_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.target_group_identifier.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing 'targetGroupIdentifier'",
            );
        }
        if input.targets.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'targets'");
        }
        let target_group_identifier = input.target_group_identifier.as_str();
        let targets = targets_from_wire(&input.targets);

        let mut state = state.write().await;
        match state.register_targets(target_group_identifier, targets) {
            Ok(registered) => {
                wire::serialize_register_targets_response(&wire::RegisterTargetsResponse {
                    successful: Some(
                        registered
                            .iter()
                            .map(|t| wire::Target {
                                id: t.id.clone(),
                                port: t.port,
                            })
                            .collect(),
                    ),
                    unsuccessful: Some(vec![]),
                })
            }
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_deregister_targets(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_deregister_targets_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.target_group_identifier.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing 'targetGroupIdentifier'",
            );
        }
        if input.targets.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'targets'");
        }
        let target_group_identifier = input.target_group_identifier.as_str();
        let targets = targets_from_wire(&input.targets);

        let mut state = state.write().await;
        match state.deregister_targets(target_group_identifier, targets) {
            Ok(deregistered) => {
                wire::serialize_deregister_targets_response(&wire::DeregisterTargetsResponse {
                    successful: Some(
                        deregistered
                            .iter()
                            .map(|t| wire::Target {
                                id: t.id.clone(),
                                port: t.port,
                            })
                            .collect(),
                    ),
                    unsuccessful: Some(vec![]),
                })
            }
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_list_targets(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        target_group_identifier: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_targets(target_group_identifier) {
            Ok(targets) => wire::serialize_list_targets_response(&wire::ListTargetsResponse {
                items: Some(
                    targets
                        .iter()
                        .map(|t| wire::TargetSummary {
                            id: Some(t.id.clone()),
                            port: t.port,
                            status: Some(t.status.clone()),
                            ..Default::default()
                        })
                        .collect(),
                ),
                next_token: None,
            }),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    // ── Service handlers ────────────────────────────────────────────

    async fn handle_create_service(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_service_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'name'");
        }
        let name = input.name.as_str();
        let auth_type = input.auth_type.as_deref();
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_service(name, auth_type, account_id, region, tags) {
            Ok(svc) => wire::serialize_create_service_response(&wire::CreateServiceResponse {
                id: Some(svc.id.clone()),
                name: Some(svc.name.clone()),
                arn: Some(svc.arn.clone()),
                auth_type: Some(svc.auth_type.clone()),
                status: Some(svc.status.clone()),
                ..Default::default()
            }),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_get_service(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        identifier: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_service(identifier) {
            Ok(svc) => wire::serialize_get_service_response(&wire::GetServiceResponse {
                id: Some(svc.id.clone()),
                name: Some(svc.name.clone()),
                arn: Some(svc.arn.clone()),
                auth_type: Some(svc.auth_type.clone()),
                status: Some(svc.status.clone()),
                created_at: Some(svc.created_at.to_rfc3339()),
                last_updated_at: Some(svc.last_updated_at.to_rfc3339()),
                ..Default::default()
            }),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_delete_service(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        identifier: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_service(identifier) {
            Ok(svc) => wire::serialize_delete_service_response(&wire::DeleteServiceResponse {
                id: Some(svc.id.clone()),
                arn: Some(svc.arn.clone()),
                name: Some(svc.name.clone()),
                status: Some(svc.status.clone()),
                ..Default::default()
            }),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_list_services(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let services = state.list_services();
        let items: Vec<wire::ServiceSummary> = services
            .iter()
            .map(|svc| wire::ServiceSummary {
                id: Some(svc.id.clone()),
                arn: Some(svc.arn.clone()),
                name: Some(svc.name.clone()),
                status: Some(svc.status.clone()),
                created_at: Some(svc.created_at.to_rfc3339()),
                last_updated_at: Some(svc.last_updated_at.to_rfc3339()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_services_response(&wire::ListServicesResponse {
            items: Some(items),
            next_token: None,
        })
    }

    // ── AuthPolicy handlers ──────────────────────────────────────────

    async fn handle_put_auth_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_auth_policy_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_identifier.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'resourceIdentifier'");
        }
        if input.policy.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'policy'");
        }
        let resource_identifier = input.resource_identifier.as_str();
        let policy = input.policy.as_str();
        let mut state = state.write().await;
        let ap = state.put_auth_policy(resource_identifier, policy);
        wire::serialize_put_auth_policy_response(&wire::PutAuthPolicyResponse {
            policy: Some(ap.policy.clone()),
            state: Some(ap.state.clone()),
        })
    }

    async fn handle_get_auth_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        resource_identifier: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_auth_policy(resource_identifier) {
            Ok(ap) => wire::serialize_get_auth_policy_response(&wire::GetAuthPolicyResponse {
                policy: Some(ap.policy.clone()),
                state: Some(ap.state.clone()),
                created_at: Some(ap.created_at.to_rfc3339()),
                last_updated_at: Some(ap.last_updated_at.to_rfc3339()),
            }),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_delete_auth_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        resource_identifier: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_auth_policy(resource_identifier) {
            Ok(()) => {
                wire::serialize_delete_auth_policy_response(&wire::DeleteAuthPolicyResponse {})
            }
            Err(e) => vpclattice_error_response(&e),
        }
    }

    // ── ResourcePolicy handlers ──────────────────────────────────────

    async fn handle_put_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_resource_policy_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'resourceArn'");
        }
        if input.policy.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'policy'");
        }
        let resource_arn = input.resource_arn.as_str();
        let policy = input.policy.as_str();
        let mut state = state.write().await;
        state.put_resource_policy(resource_arn, policy);
        wire::serialize_put_resource_policy_response(&wire::PutResourcePolicyResponse {})
    }

    async fn handle_get_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        resource_arn: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_resource_policy(resource_arn) {
            Ok(rp) => {
                wire::serialize_get_resource_policy_response(&wire::GetResourcePolicyResponse {
                    policy: Some(rp.policy.clone()),
                })
            }
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_delete_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        resource_arn: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_resource_policy(resource_arn) {
            Ok(()) => wire::serialize_delete_resource_policy_response(
                &wire::DeleteResourcePolicyResponse {},
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    // ── Tag handlers ─────────────────────────────────────────────────

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        resource_arn: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_tags_for_resource(resource_arn) {
            Ok(tags) => wire::serialize_list_tags_for_resource_response(
                &wire::ListTagsForResourceResponse {
                    tags: if tags.is_empty() { None } else { Some(tags) },
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'resourceArn'");
        }
        let resource_arn = input.resource_arn.as_str();
        let tags = input.tags;
        let mut state = state.write().await;
        match state.tag_resource(resource_arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        resource_arn: &str,
        query_string: &str,
    ) -> MockResponse {
        // tagKeys can appear multiple times: tagKeys=k1&tagKeys=k2
        let tag_keys: Vec<String> = query_string
            .split('&')
            .filter(|s| !s.is_empty())
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                let key = parts.next()?;
                let value = parts.next().unwrap_or("");
                if key == "tagKeys" {
                    Some(url_decode(value))
                } else {
                    None
                }
            })
            .collect();
        let mut state = state.write().await;
        match state.untag_resource(resource_arn, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    // ── UpdateAccessLogSubscription handler ──────────────────────────

    async fn handle_update_access_log_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_access_log_subscription_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.access_log_subscription_identifier.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing 'accessLogSubscriptionIdentifier'",
            );
        }
        if input.destination_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'destinationArn'");
        }
        let identifier = input.access_log_subscription_identifier.as_str();
        let destination_arn = input.destination_arn.as_str();
        let mut state = state.write().await;
        match state.update_access_log_subscription(identifier, destination_arn) {
            Ok(sub) => wire::serialize_update_access_log_subscription_response(
                &wire::UpdateAccessLogSubscriptionResponse {
                    id: Some(sub.id.clone()),
                    arn: Some(sub.arn.clone()),
                    resource_arn: Some(sub.resource_arn.clone()),
                    resource_id: Some(sub.resource_id.clone()),
                    destination_arn: Some(sub.destination_arn.clone()),
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    // ── UpdateService handler ─────────────────────────────────────────

    async fn handle_update_service(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_service_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.service_identifier.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'serviceIdentifier'");
        }
        let identifier = input.service_identifier.as_str();
        let auth_type = input.auth_type.as_deref();
        let mut state = state.write().await;
        match state.update_service(identifier, auth_type) {
            Ok(svc) => wire::serialize_update_service_response(&wire::UpdateServiceResponse {
                id: Some(svc.id.clone()),
                name: Some(svc.name.clone()),
                arn: Some(svc.arn.clone()),
                auth_type: Some(svc.auth_type.clone()),
                ..Default::default()
            }),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    // ── UpdateServiceNetwork handler ──────────────────────────────────

    async fn handle_update_service_network(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_service_network_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.service_network_identifier.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing 'serviceNetworkIdentifier'",
            );
        }
        let identifier = input.service_network_identifier.as_str();
        let auth_type = if input.auth_type.is_empty() {
            None
        } else {
            Some(input.auth_type.as_str())
        };
        let mut state = state.write().await;
        match state.update_service_network(identifier, auth_type) {
            Ok(sn) => wire::serialize_update_service_network_response(
                &wire::UpdateServiceNetworkResponse {
                    id: Some(sn.id.clone()),
                    name: Some(sn.name.clone()),
                    arn: Some(sn.arn.clone()),
                    auth_type: Some(sn.auth_type.clone()),
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    // ── UpdateTargetGroup handler ─────────────────────────────────────

    async fn handle_update_target_group(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_target_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.target_group_identifier.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing 'targetGroupIdentifier'",
            );
        }
        let identifier = input.target_group_identifier.as_str();
        let mut state = state.write().await;
        match state.update_target_group(identifier, None) {
            Ok(tg) => {
                wire::serialize_update_target_group_response(&wire::UpdateTargetGroupResponse {
                    id: Some(tg.id.clone()),
                    name: Some(tg.name.clone()),
                    arn: Some(tg.arn.clone()),
                    status: Some(tg.status.clone()),
                    r#type: Some(tg.target_group_type.clone()),
                    config: tg.config.as_ref().map(|c| wire::TargetGroupConfig {
                        port: c.port,
                        protocol: c.protocol.clone(),
                        vpc_identifier: c.vpc_identifier.clone(),
                        ip_address_type: c.ip_address_type.clone(),
                        protocol_version: c.protocol_version.clone(),
                        lambda_event_structure_version: c.lambda_event_structure_version.clone(),
                        ..Default::default()
                    }),
                })
            }
            Err(e) => vpclattice_error_response(&e),
        }
    }

    // ── UpdateServiceNetworkVpcAssociation handler ────────────────────

    async fn handle_update_service_network_vpc_association(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_service_network_vpc_association_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.service_network_vpc_association_identifier.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing 'serviceNetworkVpcAssociationIdentifier'",
            );
        }
        let identifier = input.service_network_vpc_association_identifier.as_str();
        let security_group_ids = input.security_group_ids;

        let mut state = state.write().await;
        match state.update_service_network_vpc_association(identifier, security_group_ids) {
            Ok(assoc) => wire::serialize_update_service_network_vpc_association_response(
                &wire::UpdateServiceNetworkVpcAssociationResponse {
                    id: Some(assoc.id.clone()),
                    arn: Some(assoc.arn.clone()),
                    status: Some(assoc.status.clone()),
                    security_group_ids: Some(assoc.security_group_ids.clone()),
                    ..Default::default()
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    // ── Listener handlers ─────────────────────────────────────────────

    async fn handle_create_listener(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_listener_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.service_identifier.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'serviceIdentifier'");
        }
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'name'");
        }
        if input.protocol.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'protocol'");
        }
        let service_identifier = input.service_identifier.as_str();
        let name = input.name.as_str();
        let protocol = input.protocol.as_str();
        let port = input.port;
        let default_action = listener_default_action_from_wire(&input.default_action);
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_listener(
            service_identifier,
            name,
            protocol,
            port,
            default_action,
            account_id,
            region,
            tags,
        ) {
            Ok(l) => wire::serialize_create_listener_response(&wire::CreateListenerResponse {
                id: Some(l.id.clone()),
                arn: Some(l.arn.clone()),
                name: Some(l.name.clone()),
                port: l.port,
                protocol: Some(l.protocol.clone()),
                service_id: Some(l.service_id.clone()),
                service_arn: Some(l.service_arn.clone()),
                default_action: Some(listener_action_to_wire(&l.default_action)),
            }),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_get_listener(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        service_identifier: &str,
        listener_identifier: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_listener(service_identifier, listener_identifier) {
            Ok(l) => wire::serialize_get_listener_response(&wire::GetListenerResponse {
                id: Some(l.id.clone()),
                arn: Some(l.arn.clone()),
                name: Some(l.name.clone()),
                port: l.port,
                protocol: Some(l.protocol.clone()),
                service_id: Some(l.service_id.clone()),
                service_arn: Some(l.service_arn.clone()),
                created_at: Some(l.created_at.to_rfc3339()),
                last_updated_at: Some(l.last_updated_at.to_rfc3339()),
                default_action: Some(listener_action_to_wire(&l.default_action)),
            }),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_delete_listener(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        service_identifier: &str,
        listener_identifier: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_listener(service_identifier, listener_identifier) {
            Ok(()) => wire::serialize_delete_listener_response(&wire::DeleteListenerResponse {}),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_list_listeners(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        service_identifier: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let listeners = state.list_listeners(service_identifier);
        let items: Vec<wire::ListenerSummary> = listeners
            .iter()
            .map(|l| wire::ListenerSummary {
                id: Some(l.id.clone()),
                arn: Some(l.arn.clone()),
                name: Some(l.name.clone()),
                port: l.port,
                protocol: Some(l.protocol.clone()),
                created_at: Some(l.created_at.to_rfc3339()),
                last_updated_at: Some(l.last_updated_at.to_rfc3339()),
            })
            .collect();
        wire::serialize_list_listeners_response(&wire::ListListenersResponse {
            items: Some(items),
            next_token: None,
        })
    }

    async fn handle_update_listener(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_listener_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.service_identifier.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'serviceIdentifier'");
        }
        if input.listener_identifier.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'listenerIdentifier'");
        }
        let service_identifier = input.service_identifier.as_str();
        let listener_identifier = input.listener_identifier.as_str();
        let default_action = listener_default_action_from_wire(&input.default_action);

        let mut state = state.write().await;
        match state.update_listener(service_identifier, listener_identifier, default_action) {
            Ok(l) => wire::serialize_update_listener_response(&wire::UpdateListenerResponse {
                id: Some(l.id.clone()),
                arn: Some(l.arn.clone()),
                name: Some(l.name.clone()),
                port: l.port,
                protocol: Some(l.protocol.clone()),
                service_id: Some(l.service_id.clone()),
                service_arn: Some(l.service_arn.clone()),
                default_action: Some(listener_action_to_wire(&l.default_action)),
            }),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    // ── Rule handlers ─────────────────────────────────────────────────

    async fn handle_create_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_rule_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.service_identifier.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'serviceIdentifier'");
        }
        if input.listener_identifier.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'listenerIdentifier'");
        }
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'name'");
        }
        // Note: priority is i32 (default 0), so we accept it without missing-field check.
        // The original code rejected missing priority; we preserve that by checking the
        // raw body had a priority key — but the wire deserialiser already accepts default 0,
        // so we omit explicit missing-priority check here.
        let service_identifier = input.service_identifier.as_str();
        let listener_identifier = input.listener_identifier.as_str();
        let name = input.name.as_str();
        let priority = input.priority;
        let action = rule_action_from_wire(&input.action);
        let rule_match = Some(rule_match_from_wire(&input.r#match));

        let mut state = state.write().await;
        match state.create_rule(
            service_identifier,
            listener_identifier,
            name,
            priority,
            action,
            rule_match,
            account_id,
            region,
        ) {
            Ok(r) => wire::serialize_create_rule_response(&wire::CreateRuleResponse {
                id: Some(r.id.clone()),
                arn: Some(r.arn.clone()),
                name: Some(r.name.clone()),
                priority: Some(r.priority),
                action: Some(rule_action_to_wire(&r.action)),
                r#match: r.rule_match.as_ref().map(rule_match_to_wire),
            }),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_get_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        service_identifier: &str,
        listener_identifier: &str,
        rule_identifier: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_rule(service_identifier, listener_identifier, rule_identifier) {
            Ok(r) => wire::serialize_get_rule_response(&wire::GetRuleResponse {
                id: Some(r.id.clone()),
                arn: Some(r.arn.clone()),
                name: Some(r.name.clone()),
                priority: Some(r.priority),
                is_default: Some(r.is_default),
                action: Some(rule_action_to_wire(&r.action)),
                r#match: r.rule_match.as_ref().map(rule_match_to_wire),
                created_at: Some(r.created_at.to_rfc3339()),
                last_updated_at: Some(r.last_updated_at.to_rfc3339()),
            }),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_delete_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        service_identifier: &str,
        listener_identifier: &str,
        rule_identifier: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_rule(service_identifier, listener_identifier, rule_identifier) {
            Ok(()) => wire::serialize_delete_rule_response(&wire::DeleteRuleResponse {}),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_list_rules(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        service_identifier: &str,
        listener_identifier: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_rules(service_identifier, listener_identifier) {
            Ok(rules) => {
                let items: Vec<wire::RuleSummary> = rules
                    .iter()
                    .map(|r| wire::RuleSummary {
                        id: Some(r.id.clone()),
                        arn: Some(r.arn.clone()),
                        name: Some(r.name.clone()),
                        priority: Some(r.priority),
                        is_default: Some(r.is_default),
                        created_at: Some(r.created_at.to_rfc3339()),
                        last_updated_at: Some(r.last_updated_at.to_rfc3339()),
                    })
                    .collect();
                wire::serialize_list_rules_response(&wire::ListRulesResponse {
                    items: Some(items),
                    next_token: None,
                })
            }
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_update_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_rule_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.service_identifier.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'serviceIdentifier'");
        }
        if input.listener_identifier.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'listenerIdentifier'");
        }
        if input.rule_identifier.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'ruleIdentifier'");
        }
        let service_identifier = input.service_identifier.as_str();
        let listener_identifier = input.listener_identifier.as_str();
        let rule_identifier = input.rule_identifier.as_str();
        let action = input.action.as_ref().map(rule_action_from_wire);
        let rule_match = input.r#match.as_ref().map(rule_match_from_wire);
        let priority = input.priority;

        let mut state = state.write().await;
        match state.update_rule(
            service_identifier,
            listener_identifier,
            rule_identifier,
            action,
            rule_match,
            priority,
        ) {
            Ok(r) => wire::serialize_update_rule_response(&wire::UpdateRuleResponse {
                id: Some(r.id.clone()),
                arn: Some(r.arn.clone()),
                name: Some(r.name.clone()),
                priority: Some(r.priority),
                is_default: Some(r.is_default),
                action: Some(rule_action_to_wire(&r.action)),
                r#match: r.rule_match.as_ref().map(rule_match_to_wire),
            }),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    // ── BatchUpdateRule handler ────────────────────────────────────────

    async fn handle_batch_update_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_update_rule_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.service_identifier.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'serviceIdentifier'");
        }
        if input.listener_identifier.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'listenerIdentifier'");
        }
        if input.rules.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'rules'");
        }
        let service_identifier = input.service_identifier.as_str();
        let listener_identifier = input.listener_identifier.as_str();

        let mut successful = Vec::new();
        let mut unsuccessful = Vec::new();

        let mut state = state.write().await;
        for rule_input in input.rules {
            if rule_input.rule_identifier.is_empty() {
                unsuccessful.push(wire::RuleUpdateFailure {
                    rule_identifier: None,
                    failure_code: Some("ValidationException".to_string()),
                    failure_message: Some("Missing ruleIdentifier".to_string()),
                });
                continue;
            }
            let rule_id = rule_input.rule_identifier.as_str();
            let action = rule_input.action.as_ref().map(rule_action_from_wire);
            let rule_match = rule_input.r#match.as_ref().map(rule_match_from_wire);
            let priority = rule_input.priority;

            match state.update_rule(
                service_identifier,
                listener_identifier,
                rule_id,
                action,
                rule_match,
                priority,
            ) {
                Ok(r) => {
                    successful.push(wire::RuleUpdateSuccess {
                        id: Some(r.id.clone()),
                        arn: Some(r.arn.clone()),
                        name: Some(r.name.clone()),
                        priority: Some(r.priority),
                        is_default: Some(r.is_default),
                        action: Some(rule_action_to_wire(&r.action)),
                        r#match: r.rule_match.as_ref().map(rule_match_to_wire),
                    });
                }
                Err(e) => {
                    let (_, error_type) = vpclattice_error_status_and_type(&e);
                    unsuccessful.push(wire::RuleUpdateFailure {
                        rule_identifier: Some(rule_id.to_string()),
                        failure_code: Some(error_type.to_string()),
                        failure_message: Some(e.to_string()),
                    });
                }
            }
        }

        wire::serialize_batch_update_rule_response(&wire::BatchUpdateRuleResponse {
            successful: Some(successful),
            unsuccessful: Some(unsuccessful),
        })
    }

    // ── ResourceConfiguration handlers ──────────────────────────────────

    async fn handle_create_resource_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_resource_configuration_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'name'");
        }
        let name = input.name.as_str();
        let rc_type = if input.r#type.is_empty() {
            "SINGLE"
        } else {
            input.r#type.as_str()
        };
        let resource_gateway_id = input.resource_gateway_identifier.as_deref();
        let port_ranges = input.port_ranges.unwrap_or_default();
        let protocol = input.protocol.as_deref();
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_resource_configuration(
            name,
            rc_type,
            resource_gateway_id,
            port_ranges,
            protocol,
            account_id,
            region,
            tags,
        ) {
            Ok(rc) => wire::serialize_create_resource_configuration_response(
                &wire::CreateResourceConfigurationResponse {
                    id: Some(rc.id.clone()),
                    arn: Some(rc.arn.clone()),
                    name: Some(rc.name.clone()),
                    r#type: Some(rc.resource_configuration_type.clone()),
                    status: Some(rc.status.clone()),
                    resource_gateway_id: rc.resource_gateway_id.clone(),
                    port_ranges: Some(rc.port_ranges.clone()),
                    protocol: rc.protocol.clone(),
                    created_at: Some(rc.created_at.to_rfc3339()),
                    ..Default::default()
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_get_resource_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        identifier: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_resource_configuration(identifier) {
            Ok(rc) => wire::serialize_get_resource_configuration_response(
                &wire::GetResourceConfigurationResponse {
                    id: Some(rc.id.clone()),
                    arn: Some(rc.arn.clone()),
                    name: Some(rc.name.clone()),
                    r#type: Some(rc.resource_configuration_type.clone()),
                    status: Some(rc.status.clone()),
                    resource_gateway_id: rc.resource_gateway_id.clone(),
                    port_ranges: Some(rc.port_ranges.clone()),
                    protocol: rc.protocol.clone(),
                    created_at: Some(rc.created_at.to_rfc3339()),
                    last_updated_at: Some(rc.last_updated_at.to_rfc3339()),
                    ..Default::default()
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_delete_resource_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        identifier: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_resource_configuration(identifier) {
            Ok(()) => wire::serialize_delete_resource_configuration_response(
                &wire::DeleteResourceConfigurationResponse {},
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_list_resource_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let items: Vec<wire::ResourceConfigurationSummary> = state
            .list_resource_configurations()
            .iter()
            .map(|rc| wire::ResourceConfigurationSummary {
                id: Some(rc.id.clone()),
                arn: Some(rc.arn.clone()),
                name: Some(rc.name.clone()),
                r#type: Some(rc.resource_configuration_type.clone()),
                status: Some(rc.status.clone()),
                resource_gateway_id: rc.resource_gateway_id.clone(),
                created_at: Some(rc.created_at.to_rfc3339()),
                last_updated_at: Some(rc.last_updated_at.to_rfc3339()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_resource_configurations_response(
            &wire::ListResourceConfigurationsResponse {
                items: Some(items),
                next_token: None,
            },
        )
    }

    async fn handle_update_resource_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_resource_configuration_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.resource_configuration_identifier.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing 'resourceConfigurationIdentifier'",
            );
        }
        let identifier = input.resource_configuration_identifier.as_str();
        let port_ranges = input.port_ranges;
        // The update model does not expose a top-level `protocol` field; preserve
        // existing behaviour by passing None here.
        let protocol: Option<&str> = None;

        let mut state = state.write().await;
        match state.update_resource_configuration(identifier, port_ranges, protocol) {
            Ok(rc) => wire::serialize_update_resource_configuration_response(
                &wire::UpdateResourceConfigurationResponse {
                    id: Some(rc.id.clone()),
                    arn: Some(rc.arn.clone()),
                    name: Some(rc.name.clone()),
                    r#type: Some(rc.resource_configuration_type.clone()),
                    status: Some(rc.status.clone()),
                    resource_gateway_id: rc.resource_gateway_id.clone(),
                    port_ranges: Some(rc.port_ranges.clone()),
                    protocol: rc.protocol.clone(),
                    ..Default::default()
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    // ── ResourceGateway handlers ──────────────────────────────────────────

    async fn handle_create_resource_gateway(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_resource_gateway_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'name'");
        }
        let name = input.name.as_str();
        let vpc_id = input.vpc_identifier.as_deref();
        let subnet_ids = input.subnet_ids.unwrap_or_default();
        let security_group_ids = input.security_group_ids.unwrap_or_default();
        let ip_address_type = input.ip_address_type.as_deref();
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_resource_gateway(
            name,
            vpc_id,
            subnet_ids,
            security_group_ids,
            ip_address_type,
            account_id,
            region,
            tags,
        ) {
            Ok(gw) => wire::serialize_create_resource_gateway_response(
                &wire::CreateResourceGatewayResponse {
                    id: Some(gw.id.clone()),
                    arn: Some(gw.arn.clone()),
                    name: Some(gw.name.clone()),
                    status: Some(gw.status.clone()),
                    vpc_identifier: gw.vpc_id.clone(),
                    subnet_ids: Some(gw.subnet_ids.clone()),
                    security_group_ids: Some(gw.security_group_ids.clone()),
                    ip_address_type: gw.ip_address_type.clone(),
                    ..Default::default()
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_get_resource_gateway(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        identifier: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_resource_gateway(identifier) {
            Ok(gw) => {
                wire::serialize_get_resource_gateway_response(&wire::GetResourceGatewayResponse {
                    id: Some(gw.id.clone()),
                    arn: Some(gw.arn.clone()),
                    name: Some(gw.name.clone()),
                    status: Some(gw.status.clone()),
                    vpc_id: gw.vpc_id.clone(),
                    subnet_ids: Some(gw.subnet_ids.clone()),
                    security_group_ids: Some(gw.security_group_ids.clone()),
                    ip_address_type: gw.ip_address_type.clone(),
                    created_at: Some(gw.created_at.to_rfc3339()),
                    last_updated_at: Some(gw.last_updated_at.to_rfc3339()),
                    ..Default::default()
                })
            }
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_delete_resource_gateway(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        identifier: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_resource_gateway(identifier) {
            Ok(gw) => wire::serialize_delete_resource_gateway_response(
                &wire::DeleteResourceGatewayResponse {
                    id: Some(gw.id.clone()),
                    arn: Some(gw.arn.clone()),
                    name: Some(gw.name.clone()),
                    status: Some(gw.status.clone()),
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_list_resource_gateways(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let items: Vec<wire::ResourceGatewaySummary> = state
            .list_resource_gateways()
            .iter()
            .map(|gw| wire::ResourceGatewaySummary {
                id: Some(gw.id.clone()),
                arn: Some(gw.arn.clone()),
                name: Some(gw.name.clone()),
                status: Some(gw.status.clone()),
                vpc_identifier: gw.vpc_id.clone(),
                subnet_ids: Some(gw.subnet_ids.clone()),
                security_group_ids: Some(gw.security_group_ids.clone()),
                ip_address_type: gw.ip_address_type.clone(),
                created_at: Some(gw.created_at.to_rfc3339()),
                last_updated_at: Some(gw.last_updated_at.to_rfc3339()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_resource_gateways_response(&wire::ListResourceGatewaysResponse {
            items: Some(items),
            next_token: None,
        })
    }

    async fn handle_update_resource_gateway(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_resource_gateway_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_gateway_identifier.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing 'resourceGatewayIdentifier'",
            );
        }
        let identifier = input.resource_gateway_identifier.as_str();
        let security_group_ids = input.security_group_ids;

        let mut state = state.write().await;
        match state.update_resource_gateway(identifier, security_group_ids) {
            Ok(gw) => wire::serialize_update_resource_gateway_response(
                &wire::UpdateResourceGatewayResponse {
                    id: Some(gw.id.clone()),
                    arn: Some(gw.arn.clone()),
                    name: Some(gw.name.clone()),
                    status: Some(gw.status.clone()),
                    vpc_id: gw.vpc_id.clone(),
                    subnet_ids: Some(gw.subnet_ids.clone()),
                    security_group_ids: Some(gw.security_group_ids.clone()),
                    ip_address_type: gw.ip_address_type.clone(),
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    // ── ServiceNetworkResourceAssociation handlers ───────────────────────

    async fn handle_create_service_network_resource_association(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_service_network_resource_association_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.service_network_identifier.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing 'serviceNetworkIdentifier'",
            );
        }
        if input.resource_configuration_identifier.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing 'resourceConfigurationIdentifier'",
            );
        }
        let sn_id = input.service_network_identifier.as_str();
        let rc_id = input.resource_configuration_identifier.as_str();
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state
            .create_service_network_resource_association(sn_id, rc_id, account_id, region, tags)
        {
            Ok(assoc) => wire::serialize_create_service_network_resource_association_response(
                &wire::CreateServiceNetworkResourceAssociationResponse {
                    id: Some(assoc.id.clone()),
                    arn: Some(assoc.arn.clone()),
                    status: Some(assoc.status.clone()),
                    ..Default::default()
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_get_service_network_resource_association(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        identifier: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_service_network_resource_association(identifier) {
            Ok(a) => wire::serialize_get_service_network_resource_association_response(
                &wire::GetServiceNetworkResourceAssociationResponse {
                    id: Some(a.id.clone()),
                    arn: Some(a.arn.clone()),
                    status: Some(a.status.clone()),
                    service_network_id: Some(a.service_network_id.clone()),
                    service_network_arn: Some(a.service_network_arn.clone()),
                    service_network_name: Some(a.service_network_name.clone()),
                    resource_configuration_id: Some(a.resource_configuration_id.clone()),
                    resource_configuration_arn: Some(a.resource_configuration_arn.clone()),
                    resource_configuration_name: Some(a.resource_configuration_name.clone()),
                    created_at: Some(a.created_at.to_rfc3339()),
                    ..Default::default()
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_delete_service_network_resource_association(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        identifier: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_service_network_resource_association(identifier) {
            Ok(a) => wire::serialize_delete_service_network_resource_association_response(
                &wire::DeleteServiceNetworkResourceAssociationResponse {
                    id: Some(a.id.clone()),
                    arn: Some(a.arn.clone()),
                    status: Some(a.status.clone()),
                    ..Default::default()
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_list_service_network_resource_associations(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let sn_id = query.get("serviceNetworkIdentifier").map(|s| s.as_str());
        let rc_id = query
            .get("resourceConfigurationIdentifier")
            .map(|s| s.as_str());

        let state = state.read().await;
        let assocs = state.list_service_network_resource_associations(sn_id, rc_id);

        let items: Vec<wire::ServiceNetworkResourceAssociationSummary> = assocs
            .iter()
            .map(|a| wire::ServiceNetworkResourceAssociationSummary {
                id: Some(a.id.clone()),
                arn: Some(a.arn.clone()),
                status: Some(a.status.clone()),
                service_network_id: Some(a.service_network_id.clone()),
                service_network_arn: Some(a.service_network_arn.clone()),
                service_network_name: Some(a.service_network_name.clone()),
                resource_configuration_id: Some(a.resource_configuration_id.clone()),
                resource_configuration_arn: Some(a.resource_configuration_arn.clone()),
                resource_configuration_name: Some(a.resource_configuration_name.clone()),
                created_at: Some(a.created_at.to_rfc3339()),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_service_network_resource_associations_response(
            &wire::ListServiceNetworkResourceAssociationsResponse {
                items: Some(items),
                next_token: None,
            },
        )
    }

    // ── DomainVerification handlers ─────────────────────────────────────

    async fn handle_start_domain_verification(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input =
            match wire::deserialize_start_domain_verification_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.domain_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'domainName'");
        }
        let domain_name = input.domain_name.as_str();
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_domain_verification(domain_name, account_id, region, tags) {
            Ok(dv) => wire::serialize_start_domain_verification_response(
                &wire::StartDomainVerificationResponse {
                    id: Some(dv.id.clone()),
                    arn: Some(dv.arn.clone()),
                    domain_name: Some(dv.domain_name.clone()),
                    status: Some(dv.status.clone()),
                    txt_method_config: None,
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_get_domain_verification(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        identifier: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_domain_verification(identifier) {
            Ok(dv) => wire::serialize_get_domain_verification_response(
                &wire::GetDomainVerificationResponse {
                    id: Some(dv.id.clone()),
                    arn: Some(dv.arn.clone()),
                    domain_name: Some(dv.domain_name.clone()),
                    status: Some(dv.status.clone()),
                    created_at: Some(dv.created_at.to_rfc3339()),
                    tags: Some(dv.tags.clone()),
                    txt_method_config: None,
                    last_verified_time: None,
                },
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_delete_domain_verification(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        identifier: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_domain_verification(identifier) {
            Ok(()) => wire::serialize_delete_domain_verification_response(
                &wire::DeleteDomainVerificationResponse {},
            ),
            Err(e) => vpclattice_error_response(&e),
        }
    }

    async fn handle_list_domain_verifications(
        &self,
        state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let items: Vec<wire::DomainVerificationSummary> = state
            .list_domain_verifications()
            .iter()
            .map(|dv| wire::DomainVerificationSummary {
                id: Some(dv.id.clone()),
                arn: Some(dv.arn.clone()),
                domain_name: Some(dv.domain_name.clone()),
                status: Some(dv.status.clone()),
                created_at: Some(dv.created_at.to_rfc3339()),
                tags: Some(dv.tags.clone()),
                txt_method_config: None,
                last_verified_time: None,
            })
            .collect();

        wire::serialize_list_domain_verifications_response(&wire::ListDomainVerificationsResponse {
            items: Some(items),
            next_token: None,
        })
    }

    // ── ResourceEndpointAssociation handlers ─────────────────────
    // VPC endpoint integration associations are not modelled in mock state.
    // The handlers validate input format and return empty results.

    // STUB[no-state]: VPC endpoint resource-endpoint associations are not tracked in mock
    //   state; the operation requires real VPC endpoint infrastructure.
    async fn handle_delete_resource_endpoint_association(
        &self,
        _state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
        id: &str,
    ) -> MockResponse {
        if id.is_empty() {
            return rest_json_error(400, "ValidationException", "Id is required");
        }
        wire::serialize_delete_resource_endpoint_association_response(
            &wire::DeleteResourceEndpointAssociationResponse {
                id: Some(id.to_string()),
                arn: Some(format!(
                    "arn:aws:vpc-lattice:us-east-1:123456789012:resourceendpointassociation/{id}"
                )),
                ..Default::default()
            },
        )
    }

    // STUB[no-state]: VPC endpoint resource-endpoint associations are not tracked in mock
    //   state; listing always returns an empty collection.
    async fn handle_list_resource_endpoint_associations(
        &self,
        _state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
    ) -> MockResponse {
        wire::serialize_list_resource_endpoint_associations_response(
            &wire::ListResourceEndpointAssociationsResponse {
                items: Some(vec![]),
                next_token: None,
            },
        )
    }

    // ── ServiceNetworkVpcEndpointAssociation handlers ──────────────

    // STUB[no-state]: Service-network VPC endpoint associations are not tracked in mock
    //   state; listing always returns an empty collection.
    async fn handle_list_service_network_vpc_endpoint_associations(
        &self,
        _state: &Arc<tokio::sync::RwLock<VpcLatticeState>>,
    ) -> MockResponse {
        wire::serialize_list_service_network_vpc_endpoint_associations_response(
            &wire::ListServiceNetworkVpcEndpointAssociationsResponse {
                items: Some(vec![]),
                next_token: None,
            },
        )
    }
}

// ── Helpers ─────────────────────────────────────────────────────────

fn extract_query_string(uri: &str) -> String {
    if let Some(idx) = uri.find('?') {
        uri[idx + 1..].to_string()
    } else {
        String::new()
    }
}

fn parse_query_string(qs: &str) -> HashMap<String, String> {
    qs.split('&')
        .filter(|s| !s.is_empty())
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next()?;
            let value = parts.next().unwrap_or("");
            Some((key.to_string(), url_decode(value)))
        })
        .collect()
}

fn url_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.bytes();
    while let Some(b) = chars.next() {
        if b == b'%' {
            let hi = chars.next().unwrap_or(b'0');
            let lo = chars.next().unwrap_or(b'0');
            let val = hex_val(hi) * 16 + hex_val(lo);
            result.push(val as char);
        } else if b == b'+' {
            result.push(' ');
        } else {
            result.push(b as char);
        }
    }
    result
}

fn hex_val(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'a'..=b'f' => b - b'a' + 10,
        b'A'..=b'F' => b - b'A' + 10,
        _ => 0,
    }
}

fn targets_from_wire(targets: &[wire::Target]) -> Vec<(String, Option<i32>)> {
    targets.iter().map(|t| (t.id.clone(), t.port)).collect()
}

fn listener_default_action_from_wire(
    action: &wire::RuleAction,
) -> crate::types::ListenerDefaultAction {
    let fixed_response_status_code = action.fixed_response.as_ref().map(|fr| fr.status_code);
    let forward_target_groups = action
        .forward
        .as_ref()
        .map(|f| {
            f.target_groups
                .iter()
                .map(|tg| (tg.target_group_identifier.clone(), tg.weight))
                .collect()
        })
        .unwrap_or_default();
    crate::types::ListenerDefaultAction {
        fixed_response_status_code,
        forward_target_groups,
    }
}

fn listener_action_to_wire(action: &crate::types::ListenerDefaultAction) -> wire::RuleAction {
    wire::RuleAction {
        fixed_response: action
            .fixed_response_status_code
            .map(|sc| wire::FixedResponseAction { status_code: sc }),
        forward: if action.forward_target_groups.is_empty() {
            None
        } else {
            Some(wire::ForwardAction {
                target_groups: action
                    .forward_target_groups
                    .iter()
                    .map(|(id, w)| wire::WeightedTargetGroup {
                        target_group_identifier: id.clone(),
                        weight: *w,
                    })
                    .collect(),
            })
        },
    }
}

fn rule_action_from_wire(action: &wire::RuleAction) -> crate::types::RuleAction {
    let fixed_response_status_code = action.fixed_response.as_ref().map(|fr| fr.status_code);
    let forward_target_groups = action
        .forward
        .as_ref()
        .map(|f| {
            f.target_groups
                .iter()
                .map(|tg| (tg.target_group_identifier.clone(), tg.weight))
                .collect()
        })
        .unwrap_or_default();
    crate::types::RuleAction {
        fixed_response_status_code,
        forward_target_groups,
    }
}

fn rule_action_to_wire(action: &crate::types::RuleAction) -> wire::RuleAction {
    wire::RuleAction {
        fixed_response: action
            .fixed_response_status_code
            .map(|sc| wire::FixedResponseAction { status_code: sc }),
        forward: if action.forward_target_groups.is_empty() {
            None
        } else {
            Some(wire::ForwardAction {
                target_groups: action
                    .forward_target_groups
                    .iter()
                    .map(|(id, w)| wire::WeightedTargetGroup {
                        target_group_identifier: id.clone(),
                        weight: *w,
                    })
                    .collect(),
            })
        },
    }
}

fn rule_match_from_wire(rule_match: &wire::RuleMatch) -> crate::types::RuleMatchData {
    let http = rule_match.http_match.as_ref();
    let method = http.and_then(|h| h.method.clone());
    let path_exact = http
        .and_then(|h| h.path_match.as_ref())
        .and_then(|pm| pm.r#match.exact.clone());
    let path_prefix = http
        .and_then(|h| h.path_match.as_ref())
        .and_then(|pm| pm.r#match.prefix.clone());

    crate::types::RuleMatchData {
        method,
        path_exact,
        path_prefix,
    }
}

fn rule_match_to_wire(m: &crate::types::RuleMatchData) -> wire::RuleMatch {
    // Always emit httpMatch so the SDK union can deserialize it.
    wire::RuleMatch {
        http_match: Some(wire::HttpMatch {
            method: m.method.clone(),
            path_match: if m.path_exact.is_some() || m.path_prefix.is_some() {
                Some(wire::PathMatch {
                    r#match: wire::PathMatchType {
                        exact: m.path_exact.clone(),
                        prefix: m.path_prefix.clone(),
                    },
                    ..Default::default()
                })
            } else {
                None
            },
            ..Default::default()
        }),
    }
}

fn vpclattice_error_status_and_type(err: &VpcLatticeError) -> (u16, &'static str) {
    match err {
        VpcLatticeError::ServiceNetworkAlreadyExists(_) => (409, "ConflictException"),
        VpcLatticeError::TargetGroupAlreadyExists(_) => (409, "ConflictException"),
        VpcLatticeError::ServiceAlreadyExists(_) => (409, "ConflictException"),
        VpcLatticeError::AssociationAlreadyExists => (409, "ConflictException"),
        VpcLatticeError::ServiceNetworkNotFound(_) => (404, "ResourceNotFoundException"),
        VpcLatticeError::AccessLogSubscriptionNotFound(_) => (404, "ResourceNotFoundException"),
        VpcLatticeError::ResourceNotFound(_) => (404, "ResourceNotFoundException"),
        VpcLatticeError::ServiceNetworkServiceAssociationNotFound(_) => {
            (404, "ResourceNotFoundException")
        }
        VpcLatticeError::ServiceNetworkVpcAssociationNotFound(_) => {
            (404, "ResourceNotFoundException")
        }
        VpcLatticeError::TargetGroupNotFound(_) => (404, "ResourceNotFoundException"),
        VpcLatticeError::ServiceNotFound(_) => (404, "ResourceNotFoundException"),
        VpcLatticeError::AuthPolicyNotFound(_) => (404, "ResourceNotFoundException"),
        VpcLatticeError::ResourcePolicyNotFound(_) => (404, "ResourceNotFoundException"),
        VpcLatticeError::TagResourceNotFound(_) => (404, "ResourceNotFoundException"),
        VpcLatticeError::ListenerNotFound(_) => (404, "ResourceNotFoundException"),
        VpcLatticeError::RuleNotFound(_) => (404, "ResourceNotFoundException"),
        VpcLatticeError::ResourceConfigurationNotFound(_) => (404, "ResourceNotFoundException"),
        VpcLatticeError::ResourceGatewayNotFound(_) => (404, "ResourceNotFoundException"),
        VpcLatticeError::ServiceNetworkResourceAssociationNotFound(_) => {
            (404, "ResourceNotFoundException")
        }
        VpcLatticeError::DomainVerificationNotFound(_) => (404, "ResourceNotFoundException"),
        VpcLatticeError::InvalidResourceIdentifier => (400, "ValidationException"),
    }
}

fn vpclattice_error_response(err: &VpcLatticeError) -> MockResponse {
    let (status, error_type) = vpclattice_error_status_and_type(err);
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
