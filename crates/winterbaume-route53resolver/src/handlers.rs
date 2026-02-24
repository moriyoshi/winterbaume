use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService, json_error_response,
};

use crate::state::{IpAddressRequest, Route53ResolverError, Route53ResolverState};
use crate::types;
use crate::views::Route53ResolverStateView;
use crate::wire;

pub struct Route53ResolverService {
    pub(crate) state: Arc<BackendState<Route53ResolverState>>,
    pub(crate) notifier: StateChangeNotifier<Route53ResolverStateView>,
}

impl Route53ResolverService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for Route53ResolverService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for Route53ResolverService {
    fn service_name(&self) -> &str {
        "route53resolver"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://route53resolver\..*\.amazonaws\.com",
            r"https?://route53resolver\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl Route53ResolverService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        // Extract action from X-Amz-Target header
        // Format: "Route53Resolver.CreateResolverEndpoint"
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

        let response = match action.as_str() {
            "CreateResolverEndpoint" => {
                self.handle_create_resolver_endpoint(&state, body_bytes, account_id, &region)
                    .await
            }
            "GetResolverEndpoint" => self.handle_get_resolver_endpoint(&state, body_bytes).await,
            "DeleteResolverEndpoint" => {
                self.handle_delete_resolver_endpoint(&state, body_bytes)
                    .await
            }
            "ListResolverEndpoints" => self.handle_list_resolver_endpoints(&state).await,
            "UpdateResolverEndpoint" => {
                self.handle_update_resolver_endpoint(&state, body_bytes)
                    .await
            }
            "AssociateResolverEndpointIpAddress" => {
                self.handle_associate_resolver_endpoint_ip_address(&state, body_bytes)
                    .await
            }
            "DisassociateResolverEndpointIpAddress" => {
                self.handle_disassociate_resolver_endpoint_ip_address(&state, body_bytes)
                    .await
            }
            "ListResolverEndpointIpAddresses" => {
                self.handle_list_resolver_endpoint_ip_addresses(&state, body_bytes)
                    .await
            }
            "CreateResolverRule" => {
                self.handle_create_resolver_rule(&state, body_bytes, account_id, &region)
                    .await
            }
            "GetResolverRule" => self.handle_get_resolver_rule(&state, body_bytes).await,
            "DeleteResolverRule" => self.handle_delete_resolver_rule(&state, body_bytes).await,
            "ListResolverRules" => self.handle_list_resolver_rules(&state).await,
            "AssociateResolverRule" => {
                self.handle_associate_resolver_rule(&state, body_bytes)
                    .await
            }
            "DisassociateResolverRule" => {
                self.handle_disassociate_resolver_rule(&state, body_bytes)
                    .await
            }
            "GetResolverRuleAssociation" => {
                self.handle_get_resolver_rule_association(&state, body_bytes)
                    .await
            }
            "ListResolverRuleAssociations" => {
                self.handle_list_resolver_rule_associations(&state).await
            }
            "CreateResolverQueryLogConfig" => {
                self.handle_create_resolver_query_log_config(
                    &state, body_bytes, account_id, &region,
                )
                .await
            }
            "GetResolverQueryLogConfig" => {
                self.handle_get_resolver_query_log_config(&state, body_bytes)
                    .await
            }
            "ListResolverQueryLogConfigs" => {
                self.handle_list_resolver_query_log_configs(&state).await
            }
            "AssociateResolverQueryLogConfig" => {
                self.handle_associate_resolver_query_log_config(&state, body_bytes)
                    .await
            }
            "GetResolverQueryLogConfigAssociation" => {
                self.handle_get_resolver_query_log_config_association(&state, body_bytes)
                    .await
            }
            "ListResolverQueryLogConfigAssociations" => {
                self.handle_list_resolver_query_log_config_associations(&state)
                    .await
            }
            "GetResolverDnssecConfig" => {
                self.handle_get_resolver_dnssec_config(&state, body_bytes, account_id)
                    .await
            }
            "UpdateResolverDnssecConfig" => {
                self.handle_update_resolver_dnssec_config(&state, body_bytes, account_id)
                    .await
            }
            "ListResolverDnssecConfigs" => self.handle_list_resolver_dnssec_configs(&state).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            // --- Unimplemented operations (auto-generated stubs) ---
            "AssociateFirewallRuleGroup" => json_error_response(
                501,
                "NotImplementedError",
                "AssociateFirewallRuleGroup is not yet implemented in winterbaume-route53resolver",
            ),
            "CreateFirewallDomainList" => json_error_response(
                501,
                "NotImplementedError",
                "CreateFirewallDomainList is not yet implemented in winterbaume-route53resolver",
            ),
            "CreateFirewallRule" => json_error_response(
                501,
                "NotImplementedError",
                "CreateFirewallRule is not yet implemented in winterbaume-route53resolver",
            ),
            "CreateFirewallRuleGroup" => json_error_response(
                501,
                "NotImplementedError",
                "CreateFirewallRuleGroup is not yet implemented in winterbaume-route53resolver",
            ),
            "CreateOutpostResolver" => json_error_response(
                501,
                "NotImplementedError",
                "CreateOutpostResolver is not yet implemented in winterbaume-route53resolver",
            ),
            "DeleteFirewallDomainList" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteFirewallDomainList is not yet implemented in winterbaume-route53resolver",
            ),
            "DeleteFirewallRule" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteFirewallRule is not yet implemented in winterbaume-route53resolver",
            ),
            "DeleteFirewallRuleGroup" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteFirewallRuleGroup is not yet implemented in winterbaume-route53resolver",
            ),
            "DeleteOutpostResolver" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteOutpostResolver is not yet implemented in winterbaume-route53resolver",
            ),
            "DeleteResolverQueryLogConfig" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteResolverQueryLogConfig is not yet implemented in winterbaume-route53resolver",
            ),
            "DisassociateFirewallRuleGroup" => json_error_response(
                501,
                "NotImplementedError",
                "DisassociateFirewallRuleGroup is not yet implemented in winterbaume-route53resolver",
            ),
            "DisassociateResolverQueryLogConfig" => json_error_response(
                501,
                "NotImplementedError",
                "DisassociateResolverQueryLogConfig is not yet implemented in winterbaume-route53resolver",
            ),
            "GetFirewallConfig" => json_error_response(
                501,
                "NotImplementedError",
                "GetFirewallConfig is not yet implemented in winterbaume-route53resolver",
            ),
            "GetFirewallDomainList" => json_error_response(
                501,
                "NotImplementedError",
                "GetFirewallDomainList is not yet implemented in winterbaume-route53resolver",
            ),
            "GetFirewallRuleGroup" => json_error_response(
                501,
                "NotImplementedError",
                "GetFirewallRuleGroup is not yet implemented in winterbaume-route53resolver",
            ),
            "GetFirewallRuleGroupAssociation" => json_error_response(
                501,
                "NotImplementedError",
                "GetFirewallRuleGroupAssociation is not yet implemented in winterbaume-route53resolver",
            ),
            "GetFirewallRuleGroupPolicy" => json_error_response(
                501,
                "NotImplementedError",
                "GetFirewallRuleGroupPolicy is not yet implemented in winterbaume-route53resolver",
            ),
            "GetOutpostResolver" => json_error_response(
                501,
                "NotImplementedError",
                "GetOutpostResolver is not yet implemented in winterbaume-route53resolver",
            ),
            "GetResolverConfig" => json_error_response(
                501,
                "NotImplementedError",
                "GetResolverConfig is not yet implemented in winterbaume-route53resolver",
            ),
            "GetResolverQueryLogConfigPolicy" => json_error_response(
                501,
                "NotImplementedError",
                "GetResolverQueryLogConfigPolicy is not yet implemented in winterbaume-route53resolver",
            ),
            "GetResolverRulePolicy" => json_error_response(
                501,
                "NotImplementedError",
                "GetResolverRulePolicy is not yet implemented in winterbaume-route53resolver",
            ),
            "ImportFirewallDomains" => json_error_response(
                501,
                "NotImplementedError",
                "ImportFirewallDomains is not yet implemented in winterbaume-route53resolver",
            ),
            "ListFirewallConfigs" => json_error_response(
                501,
                "NotImplementedError",
                "ListFirewallConfigs is not yet implemented in winterbaume-route53resolver",
            ),
            "ListFirewallDomainLists" => json_error_response(
                501,
                "NotImplementedError",
                "ListFirewallDomainLists is not yet implemented in winterbaume-route53resolver",
            ),
            "ListFirewallDomains" => json_error_response(
                501,
                "NotImplementedError",
                "ListFirewallDomains is not yet implemented in winterbaume-route53resolver",
            ),
            "ListFirewallRuleGroupAssociations" => json_error_response(
                501,
                "NotImplementedError",
                "ListFirewallRuleGroupAssociations is not yet implemented in winterbaume-route53resolver",
            ),
            "ListFirewallRuleGroups" => json_error_response(
                501,
                "NotImplementedError",
                "ListFirewallRuleGroups is not yet implemented in winterbaume-route53resolver",
            ),
            "ListFirewallRules" => json_error_response(
                501,
                "NotImplementedError",
                "ListFirewallRules is not yet implemented in winterbaume-route53resolver",
            ),
            "ListOutpostResolvers" => json_error_response(
                501,
                "NotImplementedError",
                "ListOutpostResolvers is not yet implemented in winterbaume-route53resolver",
            ),
            "ListResolverConfigs" => json_error_response(
                501,
                "NotImplementedError",
                "ListResolverConfigs is not yet implemented in winterbaume-route53resolver",
            ),
            "PutFirewallRuleGroupPolicy" => json_error_response(
                501,
                "NotImplementedError",
                "PutFirewallRuleGroupPolicy is not yet implemented in winterbaume-route53resolver",
            ),
            "PutResolverQueryLogConfigPolicy" => json_error_response(
                501,
                "NotImplementedError",
                "PutResolverQueryLogConfigPolicy is not yet implemented in winterbaume-route53resolver",
            ),
            "PutResolverRulePolicy" => json_error_response(
                501,
                "NotImplementedError",
                "PutResolverRulePolicy is not yet implemented in winterbaume-route53resolver",
            ),
            "UpdateFirewallConfig" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateFirewallConfig is not yet implemented in winterbaume-route53resolver",
            ),
            "UpdateFirewallDomains" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateFirewallDomains is not yet implemented in winterbaume-route53resolver",
            ),
            "UpdateFirewallRule" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateFirewallRule is not yet implemented in winterbaume-route53resolver",
            ),
            "UpdateFirewallRuleGroupAssociation" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateFirewallRuleGroupAssociation is not yet implemented in winterbaume-route53resolver",
            ),
            "UpdateOutpostResolver" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateOutpostResolver is not yet implemented in winterbaume-route53resolver",
            ),
            "UpdateResolverConfig" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateResolverConfig is not yet implemented in winterbaume-route53resolver",
            ),
            "UpdateResolverRule" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateResolverRule is not yet implemented in winterbaume-route53resolver",
            ),
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for Route53Resolver"),
            ),
        };

        let is_mutating = !matches!(
            action.as_str(),
            "GetResolverEndpoint"
                | "ListResolverEndpoints"
                | "ListResolverEndpointIpAddresses"
                | "GetResolverRule"
                | "ListResolverRules"
                | "GetResolverRuleAssociation"
                | "ListResolverRuleAssociations"
                | "GetResolverQueryLogConfig"
                | "ListResolverQueryLogConfigs"
                | "GetResolverQueryLogConfigAssociation"
                | "ListResolverQueryLogConfigAssociations"
                | "GetResolverDnssecConfig"
                | "ListResolverDnssecConfigs"
                | "ListTagsForResource"
        );
        if is_mutating && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // --- Resolver Endpoint handlers ---

    async fn handle_create_resolver_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_resolver_endpoint_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterException", &e),
        };
        let creator_request_id = input.creator_request_id.as_str();

        if input.direction.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'Direction'");
        }
        let direction = input.direction.as_str();

        let name = input.name.as_deref().unwrap_or("");

        let security_group_ids: Vec<String> = input.security_group_ids;

        let ip_addresses: Vec<IpAddressRequest> = input
            .ip_addresses
            .into_iter()
            .map(|ip| IpAddressRequest {
                subnet_id: ip.subnet_id,
                ip: ip.ip,
            })
            .collect();

        let protocols: Vec<String> = input.protocols.unwrap_or_default();

        let resolver_endpoint_type = input.resolver_endpoint_type.as_deref().unwrap_or("");

        let mut state = state.write().await;
        match state.create_resolver_endpoint(
            name,
            creator_request_id,
            security_group_ids,
            direction,
            &ip_addresses,
            protocols,
            resolver_endpoint_type,
            account_id,
            region,
        ) {
            Ok(endpoint) => {
                let resp = wire::CreateResolverEndpointResponse {
                    resolver_endpoint: Some(resolver_endpoint_to_wire(endpoint)),
                };
                wire::serialize_create_resolver_endpoint_response(&resp)
            }
            Err(e) => resolver_error_response(&e),
        }
    }

    async fn handle_get_resolver_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_resolver_endpoint_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterException", &e),
        };
        if input.resolver_endpoint_id.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'ResolverEndpointId'",
            );
        }
        let endpoint_id = input.resolver_endpoint_id.as_str();

        let state = state.read().await;
        match state.get_resolver_endpoint(endpoint_id) {
            Ok(endpoint) => {
                let resp = wire::GetResolverEndpointResponse {
                    resolver_endpoint: Some(resolver_endpoint_to_wire(endpoint)),
                };
                wire::serialize_get_resolver_endpoint_response(&resp)
            }
            Err(e) => resolver_error_response(&e),
        }
    }

    async fn handle_delete_resolver_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_resolver_endpoint_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterException", &e),
        };
        if input.resolver_endpoint_id.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'ResolverEndpointId'",
            );
        }
        let endpoint_id = input.resolver_endpoint_id.as_str();

        let mut state = state.write().await;
        match state.delete_resolver_endpoint(endpoint_id) {
            Ok(endpoint) => {
                let resp = wire::DeleteResolverEndpointResponse {
                    resolver_endpoint: Some(resolver_endpoint_to_wire(&endpoint)),
                };
                wire::serialize_delete_resolver_endpoint_response(&resp)
            }
            Err(e) => resolver_error_response(&e),
        }
    }

    async fn handle_list_resolver_endpoints(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let endpoints = state.list_resolver_endpoints();
        let entries: Vec<wire::ResolverEndpoint> = endpoints
            .iter()
            .map(|e| resolver_endpoint_to_wire(e))
            .collect();

        let resp = wire::ListResolverEndpointsResponse {
            max_results: Some(10),
            resolver_endpoints: Some(entries),
            ..Default::default()
        };
        wire::serialize_list_resolver_endpoints_response(&resp)
    }

    async fn handle_update_resolver_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_resolver_endpoint_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterException", &e),
        };
        if input.resolver_endpoint_id.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'ResolverEndpointId'",
            );
        }
        let endpoint_id = input.resolver_endpoint_id.as_str();

        let name = input.name.as_deref();
        let protocols = input.protocols;
        let resolver_endpoint_type = input.resolver_endpoint_type.as_deref();

        let mut state = state.write().await;
        match state.update_resolver_endpoint(endpoint_id, name, protocols, resolver_endpoint_type) {
            Ok(endpoint) => {
                let resp = wire::UpdateResolverEndpointResponse {
                    resolver_endpoint: Some(resolver_endpoint_to_wire(endpoint)),
                };
                wire::serialize_update_resolver_endpoint_response(&resp)
            }
            Err(e) => resolver_error_response(&e),
        }
    }

    async fn handle_associate_resolver_endpoint_ip_address(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_associate_resolver_endpoint_ip_address_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterException", &e),
        };
        if input.resolver_endpoint_id.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'ResolverEndpointId'",
            );
        }
        let endpoint_id = input.resolver_endpoint_id.as_str();

        let req = IpAddressRequest {
            subnet_id: input.ip_address.subnet_id.unwrap_or_default(),
            ip: input.ip_address.ip,
        };

        let mut state = state.write().await;
        match state.associate_resolver_endpoint_ip_address(endpoint_id, &req) {
            Ok(endpoint) => {
                let resp = wire::AssociateResolverEndpointIpAddressResponse {
                    resolver_endpoint: Some(resolver_endpoint_to_wire(endpoint)),
                };
                wire::serialize_associate_resolver_endpoint_ip_address_response(&resp)
            }
            Err(e) => resolver_error_response(&e),
        }
    }

    async fn handle_disassociate_resolver_endpoint_ip_address(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_disassociate_resolver_endpoint_ip_address_request(body)
        {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterException", &e),
        };
        if input.resolver_endpoint_id.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'ResolverEndpointId'",
            );
        }
        let endpoint_id = input.resolver_endpoint_id.as_str();

        let ip = input.ip_address.ip.as_deref();
        let subnet_id = input.ip_address.subnet_id.as_deref();

        let mut state = state.write().await;
        match state.disassociate_resolver_endpoint_ip_address(endpoint_id, ip, subnet_id) {
            Ok(endpoint) => {
                let resp = wire::DisassociateResolverEndpointIpAddressResponse {
                    resolver_endpoint: Some(resolver_endpoint_to_wire(endpoint)),
                };
                wire::serialize_disassociate_resolver_endpoint_ip_address_response(&resp)
            }
            Err(e) => resolver_error_response(&e),
        }
    }

    async fn handle_list_resolver_endpoint_ip_addresses(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_resolver_endpoint_ip_addresses_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterException", &e),
        };
        if input.resolver_endpoint_id.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'ResolverEndpointId'",
            );
        }
        let endpoint_id = input.resolver_endpoint_id.as_str();

        let state = state.read().await;
        match state.list_resolver_endpoint_ip_addresses(endpoint_id) {
            Ok(entries) => {
                let ip_addrs: Vec<wire::IpAddressResponse> = entries
                    .iter()
                    .map(|e| wire::IpAddressResponse {
                        ip_id: Some(e.ip_id.clone()),
                        subnet_id: Some(e.subnet_id.clone()),
                        ip: e.ip.clone(),
                        status: Some(e.status.clone()),
                        status_message: Some(e.status_message.clone()),
                        creation_time: Some(
                            e.creation_time.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
                        ),
                        modification_time: Some(
                            e.modification_time
                                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                                .to_string(),
                        ),
                        ..Default::default()
                    })
                    .collect();
                let resp = wire::ListResolverEndpointIpAddressesResponse {
                    ip_addresses: Some(ip_addrs),
                    max_results: Some(10),
                    ..Default::default()
                };
                wire::serialize_list_resolver_endpoint_ip_addresses_response(&resp)
            }
            Err(e) => resolver_error_response(&e),
        }
    }

    // --- Resolver Rule handlers ---

    async fn handle_create_resolver_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_resolver_rule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterException", &e),
        };
        let creator_request_id = input.creator_request_id.as_str();

        let domain_name = match input.domain_name.as_deref() {
            Some(d) if !d.is_empty() => d,
            _ => {
                return json_error_response(
                    400,
                    "InvalidParameterException",
                    "Missing 'DomainName'",
                );
            }
        };

        if input.rule_type.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'RuleType'");
        }
        let rule_type = input.rule_type.as_str();

        let name = input.name.as_deref().unwrap_or("");
        let resolver_endpoint_id = input.resolver_endpoint_id.as_deref();

        let target_ips: Vec<types::TargetAddress> = input
            .target_ips
            .unwrap_or_default()
            .into_iter()
            .map(|t| types::TargetAddress {
                ip: t.ip,
                ipv6: t.ipv6,
                port: t.port,
                protocol: t.protocol,
            })
            .collect();

        let mut state = state.write().await;
        match state.create_resolver_rule(
            name,
            creator_request_id,
            domain_name,
            rule_type,
            resolver_endpoint_id,
            target_ips,
            account_id,
            region,
        ) {
            Ok(rule) => {
                let resp = wire::CreateResolverRuleResponse {
                    resolver_rule: Some(resolver_rule_to_wire(rule)),
                };
                wire::serialize_create_resolver_rule_response(&resp)
            }
            Err(e) => resolver_error_response(&e),
        }
    }

    async fn handle_get_resolver_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_resolver_rule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterException", &e),
        };
        if input.resolver_rule_id.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'ResolverRuleId'",
            );
        }
        let rule_id = input.resolver_rule_id.as_str();

        let state = state.read().await;
        match state.get_resolver_rule(rule_id) {
            Ok(rule) => {
                let resp = wire::GetResolverRuleResponse {
                    resolver_rule: Some(resolver_rule_to_wire(rule)),
                };
                wire::serialize_get_resolver_rule_response(&resp)
            }
            Err(e) => resolver_error_response(&e),
        }
    }

    async fn handle_delete_resolver_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_resolver_rule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterException", &e),
        };
        if input.resolver_rule_id.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'ResolverRuleId'",
            );
        }
        let rule_id = input.resolver_rule_id.as_str();

        let mut state = state.write().await;
        match state.delete_resolver_rule(rule_id) {
            Ok(rule) => {
                let resp = wire::DeleteResolverRuleResponse {
                    resolver_rule: Some(resolver_rule_to_wire(&rule)),
                };
                wire::serialize_delete_resolver_rule_response(&resp)
            }
            Err(e) => resolver_error_response(&e),
        }
    }

    async fn handle_list_resolver_rules(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let rules = state.list_resolver_rules();
        let entries: Vec<wire::ResolverRule> =
            rules.iter().map(|r| resolver_rule_to_wire(r)).collect();

        let resp = wire::ListResolverRulesResponse {
            max_results: Some(10),
            resolver_rules: Some(entries),
            ..Default::default()
        };
        wire::serialize_list_resolver_rules_response(&resp)
    }

    // --- Resolver Rule Association handlers ---

    async fn handle_associate_resolver_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_associate_resolver_rule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterException", &e),
        };
        if input.resolver_rule_id.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'ResolverRuleId'",
            );
        }
        let resolver_rule_id = input.resolver_rule_id.as_str();

        if input.v_p_c_id.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'VPCId'");
        }
        let vpc_id = input.v_p_c_id.as_str();

        let name = input.name.as_deref().unwrap_or("");

        let mut state = state.write().await;
        match state.associate_resolver_rule(resolver_rule_id, vpc_id, name) {
            Ok(assoc) => {
                let resp = wire::AssociateResolverRuleResponse {
                    resolver_rule_association: Some(rule_association_to_wire(assoc)),
                };
                wire::serialize_associate_resolver_rule_response(&resp)
            }
            Err(e) => resolver_error_response(&e),
        }
    }

    async fn handle_disassociate_resolver_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_disassociate_resolver_rule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterException", &e),
        };
        if input.resolver_rule_id.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'ResolverRuleId'",
            );
        }
        let resolver_rule_id = input.resolver_rule_id.as_str();

        if input.v_p_c_id.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'VPCId'");
        }
        let vpc_id = input.v_p_c_id.as_str();

        let mut state = state.write().await;
        match state.disassociate_resolver_rule(resolver_rule_id, vpc_id) {
            Ok(assoc) => {
                let resp = wire::DisassociateResolverRuleResponse {
                    resolver_rule_association: Some(rule_association_to_wire(&assoc)),
                };
                wire::serialize_disassociate_resolver_rule_response(&resp)
            }
            Err(e) => resolver_error_response(&e),
        }
    }

    async fn handle_get_resolver_rule_association(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_resolver_rule_association_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterException", &e),
        };
        if input.resolver_rule_association_id.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'ResolverRuleAssociationId'",
            );
        }
        let assoc_id = input.resolver_rule_association_id.as_str();

        let state = state.read().await;
        match state.get_resolver_rule_association(assoc_id) {
            Ok(assoc) => {
                let resp = wire::GetResolverRuleAssociationResponse {
                    resolver_rule_association: Some(rule_association_to_wire(assoc)),
                };
                wire::serialize_get_resolver_rule_association_response(&resp)
            }
            Err(e) => resolver_error_response(&e),
        }
    }

    async fn handle_list_resolver_rule_associations(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let assocs = state.list_resolver_rule_associations();
        let entries: Vec<wire::ResolverRuleAssociation> =
            assocs.iter().map(|a| rule_association_to_wire(a)).collect();

        let resp = wire::ListResolverRuleAssociationsResponse {
            max_results: Some(10),
            resolver_rule_associations: Some(entries),
            ..Default::default()
        };
        wire::serialize_list_resolver_rule_associations_response(&resp)
    }

    // --- Query Log Config handlers ---

    async fn handle_create_resolver_query_log_config(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_resolver_query_log_config_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'Name'");
        }
        let name = input.name.as_str();

        if input.destination_arn.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'DestinationArn'",
            );
        }
        let destination_arn = input.destination_arn.as_str();

        let creator_request_id = input.creator_request_id.as_str();

        let mut state = state.write().await;
        match state.create_resolver_query_log_config(
            name,
            destination_arn,
            creator_request_id,
            account_id,
            region,
        ) {
            Ok(config) => {
                let resp = wire::CreateResolverQueryLogConfigResponse {
                    resolver_query_log_config: Some(query_log_config_to_wire(config)),
                };
                wire::serialize_create_resolver_query_log_config_response(&resp)
            }
            Err(e) => resolver_error_response(&e),
        }
    }

    async fn handle_get_resolver_query_log_config(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_resolver_query_log_config_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterException", &e),
        };
        if input.resolver_query_log_config_id.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'ResolverQueryLogConfigId'",
            );
        }
        let config_id = input.resolver_query_log_config_id.as_str();

        let state = state.read().await;
        match state.get_resolver_query_log_config(config_id) {
            Ok(config) => {
                let resp = wire::GetResolverQueryLogConfigResponse {
                    resolver_query_log_config: Some(query_log_config_to_wire(config)),
                };
                wire::serialize_get_resolver_query_log_config_response(&resp)
            }
            Err(e) => resolver_error_response(&e),
        }
    }

    async fn handle_list_resolver_query_log_configs(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let configs = state.list_resolver_query_log_configs();
        let entries: Vec<wire::ResolverQueryLogConfig> = configs
            .iter()
            .map(|c| query_log_config_to_wire(c))
            .collect();
        let total = entries.len() as i32;

        let resp = wire::ListResolverQueryLogConfigsResponse {
            resolver_query_log_configs: Some(entries),
            total_count: Some(total),
            total_filtered_count: Some(total),
            ..Default::default()
        };
        wire::serialize_list_resolver_query_log_configs_response(&resp)
    }

    // --- Query Log Config Association handlers ---

    async fn handle_associate_resolver_query_log_config(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_associate_resolver_query_log_config_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterException", &e),
        };
        if input.resolver_query_log_config_id.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'ResolverQueryLogConfigId'",
            );
        }
        let config_id = input.resolver_query_log_config_id.as_str();

        if input.resource_id.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'ResourceId'");
        }
        let resource_id = input.resource_id.as_str();

        let mut state = state.write().await;
        match state.associate_resolver_query_log_config(config_id, resource_id) {
            Ok(assoc) => {
                let resp = wire::AssociateResolverQueryLogConfigResponse {
                    resolver_query_log_config_association: Some(
                        query_log_config_association_to_wire(assoc),
                    ),
                };
                wire::serialize_associate_resolver_query_log_config_response(&resp)
            }
            Err(e) => resolver_error_response(&e),
        }
    }

    async fn handle_get_resolver_query_log_config_association(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_resolver_query_log_config_association_request(body)
        {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterException", &e),
        };
        if input.resolver_query_log_config_association_id.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'ResolverQueryLogConfigAssociationId'",
            );
        }
        let assoc_id = input.resolver_query_log_config_association_id.as_str();

        let state = state.read().await;
        match state.get_resolver_query_log_config_association(assoc_id) {
            Ok(assoc) => {
                let resp = wire::GetResolverQueryLogConfigAssociationResponse {
                    resolver_query_log_config_association: Some(
                        query_log_config_association_to_wire(assoc),
                    ),
                };
                wire::serialize_get_resolver_query_log_config_association_response(&resp)
            }
            Err(e) => resolver_error_response(&e),
        }
    }

    async fn handle_list_resolver_query_log_config_associations(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let assocs = state.list_resolver_query_log_config_associations();
        let entries: Vec<wire::ResolverQueryLogConfigAssociation> = assocs
            .iter()
            .map(|a| query_log_config_association_to_wire(a))
            .collect();
        let total = entries.len() as i32;

        let resp = wire::ListResolverQueryLogConfigAssociationsResponse {
            resolver_query_log_config_associations: Some(entries),
            total_count: Some(total),
            total_filtered_count: Some(total),
            ..Default::default()
        };
        wire::serialize_list_resolver_query_log_config_associations_response(&resp)
    }

    // --- DNSSEC Config handlers ---

    async fn handle_get_resolver_dnssec_config(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_get_resolver_dnssec_config_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterException", &e),
        };
        if input.resource_id.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'ResourceId'");
        }
        let resource_id = input.resource_id.as_str();

        let mut state = state.write().await;
        let config = state.get_resolver_dnssec_config(resource_id, account_id);
        let resp = wire::GetResolverDnssecConfigResponse {
            resolver_d_n_s_s_e_c_config: Some(dnssec_config_to_wire(config)),
        };
        wire::serialize_get_resolver_dnssec_config_response(&resp)
    }

    async fn handle_update_resolver_dnssec_config(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_resolver_dnssec_config_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterException", &e),
        };
        if input.resource_id.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'ResourceId'");
        }
        let resource_id = input.resource_id.as_str();

        if input.validation.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'Validation'");
        }
        let validation = input.validation.as_str();

        let mut state = state.write().await;
        let config = state.update_resolver_dnssec_config(resource_id, validation, account_id);
        let resp = wire::UpdateResolverDnssecConfigResponse {
            resolver_d_n_s_s_e_c_config: Some(dnssec_config_to_wire(config)),
        };
        wire::serialize_update_resolver_dnssec_config_response(&resp)
    }

    async fn handle_list_resolver_dnssec_configs(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let configs = state.list_resolver_dnssec_configs();
        let entries: Vec<wire::ResolverDnssecConfig> =
            configs.iter().map(|c| dnssec_config_to_wire(c)).collect();

        let resp = wire::ListResolverDnssecConfigsResponse {
            resolver_dnssec_configs: Some(entries),
            ..Default::default()
        };
        wire::serialize_list_resolver_dnssec_configs_response(&resp)
    }

    // --- Tag handlers ---

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'ResourceArn'");
        }
        let resource_arn = input.resource_arn.as_str();

        let tags: HashMap<String, String> =
            input.tags.into_iter().map(|t| (t.key, t.value)).collect();

        let mut state = state.write().await;
        match state.tag_resource(resource_arn, tags) {
            Ok(()) => {
                let resp = wire::TagResourceResponse {};
                wire::serialize_tag_resource_response(&resp)
            }
            Err(e) => resolver_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'ResourceArn'");
        }
        let resource_arn = input.resource_arn.as_str();

        let tag_keys = input.tag_keys;

        let mut state = state.write().await;
        match state.untag_resource(resource_arn, &tag_keys) {
            Ok(()) => {
                let resp = wire::UntagResourceResponse {};
                wire::serialize_untag_resource_response(&resp)
            }
            Err(e) => resolver_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53ResolverState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParameterException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'ResourceArn'");
        }
        let resource_arn = input.resource_arn.as_str();

        let state = state.read().await;
        match state.list_tags_for_resource(resource_arn) {
            Ok(tags) => {
                let tag_list: Vec<wire::Tag> = tags
                    .iter()
                    .map(|(k, v)| wire::Tag {
                        key: k.clone(),
                        value: v.clone(),
                    })
                    .collect();
                let resp = wire::ListTagsForResourceResponse {
                    tags: Some(tag_list),
                    ..Default::default()
                };
                wire::serialize_list_tags_for_resource_response(&resp)
            }
            Err(e) => resolver_error_response(&e),
        }
    }
}

// --- Wire conversion helpers ---

fn resolver_endpoint_to_wire(endpoint: &types::ResolverEndpoint) -> wire::ResolverEndpoint {
    wire::ResolverEndpoint {
        id: Some(endpoint.id.clone()),
        arn: Some(endpoint.arn.clone()),
        name: Some(endpoint.name.clone()),
        security_group_ids: Some(endpoint.security_group_ids.clone()),
        direction: Some(endpoint.direction.clone()),
        ip_address_count: Some(endpoint.ip_address_count),
        host_v_p_c_id: Some(endpoint.host_vpc_id.clone()),
        status: Some(endpoint.status.clone()),
        status_message: Some(endpoint.status_message.clone()),
        creation_time: Some(
            endpoint
                .creation_time
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        ),
        modification_time: Some(
            endpoint
                .modification_time
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        ),
        creator_request_id: Some(endpoint.creator_request_id.clone()),
        protocols: Some(endpoint.protocols.clone()),
        resolver_endpoint_type: Some(endpoint.resolver_endpoint_type.clone()),
        ..Default::default()
    }
}

fn resolver_rule_to_wire(rule: &types::ResolverRule) -> wire::ResolverRule {
    let target_ips: Vec<wire::TargetAddress> = rule
        .target_ips
        .iter()
        .map(|t| wire::TargetAddress {
            ip: t.ip.clone(),
            ipv6: t.ipv6.clone(),
            port: t.port,
            protocol: t.protocol.clone(),
            ..Default::default()
        })
        .collect();

    wire::ResolverRule {
        id: Some(rule.id.clone()),
        arn: Some(rule.arn.clone()),
        name: Some(rule.name.clone()),
        domain_name: Some(rule.domain_name.clone()),
        rule_type: Some(rule.rule_type.clone()),
        resolver_endpoint_id: rule.resolver_endpoint_id.clone(),
        target_ips: if target_ips.is_empty() {
            None
        } else {
            Some(target_ips)
        },
        status: Some(rule.status.clone()),
        status_message: Some(rule.status_message.clone()),
        owner_id: Some(rule.owner_id.clone()),
        share_status: Some(rule.share_status.clone()),
        creator_request_id: Some(rule.creator_request_id.clone()),
        creation_time: Some(
            rule.creation_time
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        ),
        modification_time: Some(
            rule.modification_time
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        ),
        ..Default::default()
    }
}

fn rule_association_to_wire(
    assoc: &types::ResolverRuleAssociation,
) -> wire::ResolverRuleAssociation {
    wire::ResolverRuleAssociation {
        id: Some(assoc.id.clone()),
        resolver_rule_id: Some(assoc.resolver_rule_id.clone()),
        name: Some(assoc.name.clone()),
        v_p_c_id: Some(assoc.vpc_id.clone()),
        status: Some(assoc.status.clone()),
        status_message: Some(assoc.status_message.clone()),
    }
}

fn query_log_config_to_wire(
    config: &types::ResolverQueryLogConfig,
) -> wire::ResolverQueryLogConfig {
    wire::ResolverQueryLogConfig {
        id: Some(config.id.clone()),
        arn: Some(config.arn.clone()),
        name: Some(config.name.clone()),
        destination_arn: Some(config.destination_arn.clone()),
        owner_id: Some(config.owner_id.clone()),
        status: Some(config.status.clone()),
        share_status: Some(config.share_status.clone()),
        association_count: Some(config.association_count),
        creator_request_id: Some(config.creator_request_id.clone()),
        creation_time: Some(
            config
                .creation_time
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        ),
    }
}

fn query_log_config_association_to_wire(
    assoc: &types::ResolverQueryLogConfigAssociation,
) -> wire::ResolverQueryLogConfigAssociation {
    wire::ResolverQueryLogConfigAssociation {
        id: Some(assoc.id.clone()),
        resolver_query_log_config_id: Some(assoc.resolver_query_log_config_id.clone()),
        resource_id: Some(assoc.resource_id.clone()),
        status: Some(assoc.status.clone()),
        error: Some(assoc.error.clone()),
        error_message: Some(assoc.error_message.clone()),
        creation_time: Some(
            assoc
                .creation_time
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        ),
    }
}

fn dnssec_config_to_wire(config: &types::ResolverDnssecConfig) -> wire::ResolverDnssecConfig {
    wire::ResolverDnssecConfig {
        id: Some(config.id.clone()),
        owner_id: Some(config.owner_id.clone()),
        resource_id: Some(config.resource_id.clone()),
        validation_status: Some(config.validation_status.clone()),
    }
}

fn resolver_error_response(err: &Route53ResolverError) -> MockResponse {
    use Route53ResolverError::*;
    let (status, error_type) = match err {
        InvalidDirection { .. } => (400, "InvalidParameterException"),
        EmptySecurityGroupIds => (400, "InvalidRequestException"),
        EndpointNotFound { .. } => (400, "ResourceNotFoundException"),
        MinimumIpAddressesRequired => (400, "InvalidRequestException"),
        IpAddressNotFound => (400, "ResourceNotFoundException"),
        InvalidRuleType { .. } => (400, "InvalidParameterException"),
        RuleNotFound { .. } => (400, "ResourceNotFoundException"),
        RuleInUse { .. } => (400, "ResourceInUseException"),
        RuleAlreadyAssociated { .. } => (400, "InvalidRequestException"),
        RuleAssociationNotFoundByRuleAndVpc { .. } => (400, "ResourceNotFoundException"),
        RuleAssociationNotFound { .. } => (400, "ResourceNotFoundException"),
        QueryLogConfigNotFound { .. } => (400, "ResourceNotFoundException"),
        QueryLogConfigAssociationNotFound { .. } => (400, "ResourceNotFoundException"),
        ResourceNotFoundByArn { .. } => (400, "ResourceNotFoundException"),
    };
    let body = json!({
        "__type": error_type,
        "message": err.to_string(),
    });
    MockResponse::json(status, body.to_string())
}
