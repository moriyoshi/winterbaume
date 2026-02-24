use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{ElbV2Error, ElbV2State};
use crate::types::ListenerAction;
use crate::views::Elbv2StateView;
use crate::wire;

pub struct ElasticLoadBalancingV2Service {
    pub(crate) state: Arc<BackendState<ElbV2State>>,
    pub(crate) notifier: StateChangeNotifier<Elbv2StateView>,
}

impl ElasticLoadBalancingV2Service {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ElasticLoadBalancingV2Service {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ElasticLoadBalancingV2Service {
    fn service_name(&self) -> &str {
        "elasticloadbalancing"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://elasticloadbalancing\..*\.amazonaws\.com",
            r"https?://elasticloadbalancing\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl ElasticLoadBalancingV2Service {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        let body_str = std::str::from_utf8(&request.body).unwrap_or("");
        let params = parse_query_string(body_str);

        let action = match params.get("Action") {
            Some(a) => a.as_str(),
            None => {
                return MockResponse::error(400, "MissingAction", "Missing 'Action' parameter");
            }
        };

        let state = self.state.get(account_id, &region);

        let response = match action {
            "CreateLoadBalancer" => {
                self.handle_create_load_balancer(&state, &params, account_id, &region)
                    .await
            }
            "DeleteLoadBalancer" => self.handle_delete_load_balancer(&state, &params).await,
            "DescribeLoadBalancers" => self.handle_describe_load_balancers(&state, &params).await,
            "CreateTargetGroup" => {
                self.handle_create_target_group(&state, &params, account_id, &region)
                    .await
            }
            "DeleteTargetGroup" => self.handle_delete_target_group(&state, &params).await,
            "DescribeTargetGroups" => self.handle_describe_target_groups(&state, &params).await,
            "CreateListener" => {
                self.handle_create_listener(&state, &params, account_id, &region)
                    .await
            }
            "DescribeListeners" => self.handle_describe_listeners(&state, &params).await,
            "DeleteListener" => self.handle_delete_listener(&state, &params).await,
            "ModifyListener" => self.handle_modify_listener(&state, &params).await,
            "CreateRule" => {
                self.handle_create_rule(&state, &params, account_id, &region)
                    .await
            }
            "DeleteRule" => self.handle_delete_rule(&state, &params).await,
            "DescribeRules" => self.handle_describe_rules(&state, &params).await,
            "ModifyRule" => self.handle_modify_rule(&state, &params).await,
            "RegisterTargets" => self.handle_register_targets(&state, &params).await,
            "DeregisterTargets" => self.handle_deregister_targets(&state, &params).await,
            "DescribeTargetHealth" => self.handle_describe_target_health(&state, &params).await,
            "ModifyTargetGroup" => self.handle_modify_target_group(&state, &params).await,
            "ModifyTargetGroupAttributes" => {
                self.handle_modify_target_group_attributes(&state, &params)
                    .await
            }
            "AddTags" => self.handle_add_tags(&state, &params).await,
            "RemoveTags" => self.handle_remove_tags(&state, &params).await,
            "DescribeTags" => self.handle_describe_tags(&state, &params).await,
            "DescribeLoadBalancerAttributes" => {
                self.handle_describe_load_balancer_attributes(&state, &params)
                    .await
            }
            "ModifyLoadBalancerAttributes" => {
                self.handle_modify_load_balancer_attributes(&state, &params)
                    .await
            }
            "DescribeListenerAttributes" => {
                self.handle_describe_listener_attributes(&state, &params)
                    .await
            }
            "ModifyListenerAttributes" => {
                self.handle_modify_listener_attributes(&state, &params)
                    .await
            }
            "DescribeListenerCertificates" => {
                self.handle_describe_listener_certificates(&state, &params)
                    .await
            }
            "AddListenerCertificates" => {
                self.handle_add_listener_certificates(&state, &params).await
            }
            "RemoveListenerCertificates" => {
                self.handle_remove_listener_certificates(&state, &params)
                    .await
            }
            "SetRulePriorities" => self.handle_set_rule_priorities(&state, &params).await,
            "SetIpAddressType" => self.handle_set_ip_address_type(&state, &params).await,
            "SetSecurityGroups" => self.handle_set_security_groups(&state, &params).await,
            "SetSubnets" => self.handle_set_subnets(&state, &params, &region).await,
            // TrustStore operations
            "CreateTrustStore" => {
                self.handle_create_trust_store(&state, &params, account_id, &region)
                    .await
            }
            "DeleteTrustStore" => self.handle_delete_trust_store(&state, &params).await,
            "DescribeTrustStores" => self.handle_describe_trust_stores(&state, &params).await,
            "ModifyTrustStore" => self.handle_modify_trust_store(&state, &params).await,
            "AddTrustStoreRevocations" => {
                self.handle_add_trust_store_revocations(&state, &params)
                    .await
            }
            "RemoveTrustStoreRevocations" => {
                self.handle_remove_trust_store_revocations(&state, &params)
                    .await
            }
            "DescribeTrustStoreRevocations" => {
                self.handle_describe_trust_store_revocations(&state, &params)
                    .await
            }
            "DescribeTrustStoreAssociations" => {
                self.handle_describe_trust_store_associations(&state, &params)
                    .await
            }
            "DeleteSharedTrustStoreAssociation" => {
                self.handle_delete_shared_trust_store_association(&state, &params)
                    .await
            }
            "GetTrustStoreCaCertificatesBundle" => {
                self.handle_get_trust_store_ca_certificates_bundle(&state, &params)
                    .await
            }
            "GetTrustStoreRevocationContent" => {
                self.handle_get_trust_store_revocation_content(&state, &params)
                    .await
            }
            // Account / capacity / SSL / misc
            "DescribeAccountLimits" => self.handle_describe_account_limits().await,
            "DescribeCapacityReservation" => {
                self.handle_describe_capacity_reservation(&state, &params)
                    .await
            }
            "ModifyCapacityReservation" => {
                self.handle_modify_capacity_reservation(&state, &params)
                    .await
            }
            "DescribeSSLPolicies" => self.handle_describe_ssl_policies().await,
            "DescribeTargetGroupAttributes" => {
                self.handle_describe_target_group_attributes(&state, &params)
                    .await
            }
            "GetResourcePolicy" => self.handle_get_resource_policy(&state, &params).await,
            "ModifyIpPools" => self.handle_modify_ip_pools(&state, &params).await,
            _ => MockResponse::error(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for Elastic Load Balancing"),
            ),
        };

        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // --- Load Balancer operations ---

    async fn handle_create_load_balancer(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_load_balancer_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'Name'");
        }
        let name = &input.name;
        let scheme = input.scheme.as_deref().unwrap_or("internet-facing");
        let lb_type = input.r#type.as_deref().unwrap_or("application");

        let mut state = state.write().await;
        match state.create_load_balancer(name, scheme, lb_type, account_id, region) {
            Ok(lb) => {
                wire::serialize_create_load_balancer_response(&wire::CreateLoadBalancerOutput {
                    load_balancers: Some(vec![lb_to_wire(lb)].into()),
                })
            }
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_delete_load_balancer(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_load_balancer_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'LoadBalancerArn'");
        }
        let arn = &input.load_balancer_arn;

        let mut state = state.write().await;
        match state.delete_load_balancer(arn) {
            Ok(()) => {
                wire::serialize_delete_load_balancer_response(&wire::DeleteLoadBalancerOutput {})
            }
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_describe_load_balancers(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_load_balancers_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let lb_arns = input
            .load_balancer_arns
            .map(|l| l.items)
            .unwrap_or_default();
        let names = input.names.map(|l| l.items).unwrap_or_default();

        let state = state.read().await;

        // If filtering by ARNs, validate they exist
        if !lb_arns.is_empty() {
            for arn in &lb_arns {
                if !state.load_balancers.contains_key(arn) {
                    return elbv2_error_response(&ElbV2Error::LoadBalancerNotFoundList(
                        arn.clone(),
                    ));
                }
            }
        }

        // If filtering by names, validate they exist
        if !names.is_empty() {
            for name in &names {
                if !state.load_balancers.values().any(|lb| &lb.name == name) {
                    return elbv2_error_response(&ElbV2Error::LoadBalancerNotFoundList(
                        name.clone(),
                    ));
                }
            }
        }

        let mut lbs: Vec<&_> = state.describe_load_balancers();
        if !lb_arns.is_empty() {
            lbs.retain(|lb| lb_arns.contains(&lb.arn));
        }
        if !names.is_empty() {
            lbs.retain(|lb| names.contains(&lb.name));
        }

        wire::serialize_describe_load_balancers_response(&wire::DescribeLoadBalancersOutput {
            load_balancers: Some(
                lbs.iter()
                    .map(|lb| lb_to_wire(lb))
                    .collect::<Vec<_>>()
                    .into(),
            ),
            next_marker: None,
        })
    }

    async fn handle_describe_load_balancer_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_load_balancer_attributes_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'LoadBalancerArn'");
        }
        let lb_arn = &input.load_balancer_arn;

        let state = state.read().await;
        match state.describe_load_balancer_attributes(lb_arn) {
            Ok(attrs) => wire::serialize_describe_load_balancer_attributes_response(
                &wire::DescribeLoadBalancerAttributesOutput {
                    attributes: Some(
                        attrs
                            .iter()
                            .map(|(k, v)| wire::LoadBalancerAttribute {
                                key: Some(k.clone()),
                                value: Some(v.clone()),
                            })
                            .collect::<Vec<_>>()
                            .into(),
                    ),
                },
            ),
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_modify_load_balancer_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_load_balancer_attributes_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'LoadBalancerArn'");
        }
        let lb_arn = &input.load_balancer_arn;

        let attributes: Vec<(String, String)> = input
            .attributes
            .items
            .iter()
            .map(|a| {
                (
                    a.key.clone().unwrap_or_default(),
                    a.value.clone().unwrap_or_default(),
                )
            })
            .collect();

        let mut state = state.write().await;
        match state.modify_load_balancer_attributes(lb_arn, &attributes) {
            Ok(attrs) => wire::serialize_modify_load_balancer_attributes_response(
                &wire::ModifyLoadBalancerAttributesOutput {
                    attributes: Some(
                        attrs
                            .iter()
                            .map(|(k, v)| wire::LoadBalancerAttribute {
                                key: Some(k.clone()),
                                value: Some(v.clone()),
                            })
                            .collect::<Vec<_>>()
                            .into(),
                    ),
                },
            ),
            Err(e) => elbv2_error_response(&e),
        }
    }

    // --- Target Group operations ---

    async fn handle_create_target_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_target_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'Name'");
        }
        let name = &input.name;
        let protocol = input.protocol.as_deref().unwrap_or("HTTP");
        let port: i32 = input.port.unwrap_or(80);
        let vpc_id = input.vpc_id.as_deref().unwrap_or("vpc-12345678");
        let target_type = input.target_type.as_deref().unwrap_or("instance");
        let health_check_path = input.health_check_path.as_deref().unwrap_or("/");

        let mut state = state.write().await;
        match state.create_target_group(
            name,
            protocol,
            port,
            vpc_id,
            target_type,
            health_check_path,
            account_id,
            region,
        ) {
            Ok(tg) => {
                wire::serialize_create_target_group_response(&wire::CreateTargetGroupOutput {
                    target_groups: Some(vec![tg_to_wire(tg)].into()),
                })
            }
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_delete_target_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_target_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.target_group_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TargetGroupArn'");
        }
        let arn = &input.target_group_arn;

        let mut state = state.write().await;
        match state.delete_target_group(arn) {
            Ok(()) => {
                wire::serialize_delete_target_group_response(&wire::DeleteTargetGroupOutput {})
            }
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_describe_target_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_target_groups_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let state = state.read().await;
        let all_tgs = state.describe_target_groups();

        // FIX(terraform-e2e): apply ARN and name filters so the provider's waiter
        // gets back exactly the TG it requested rather than all TGs.  Without this
        // the provider detects drift because TargetGroups[0] may be the wrong TG.
        let arns = input.target_group_arns.map(|l| l.items).unwrap_or_default();
        let names = input.names.map(|l| l.items).unwrap_or_default();
        let tgs: Vec<_> = all_tgs
            .into_iter()
            .filter(|tg| {
                (arns.is_empty() || arns.iter().any(|a| a == &tg.arn))
                    && (names.is_empty() || names.iter().any(|n| n == &tg.name))
            })
            .collect();

        wire::serialize_describe_target_groups_response(&wire::DescribeTargetGroupsOutput {
            target_groups: Some(
                tgs.iter()
                    .map(|tg| tg_to_wire(tg))
                    .collect::<Vec<_>>()
                    .into(),
            ),
            next_marker: None,
        })
    }

    async fn handle_modify_target_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_target_group_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.target_group_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TargetGroupArn'");
        }
        let tg_arn = &input.target_group_arn;
        let health_check_path = input.health_check_path.as_deref();

        let mut state = state.write().await;
        match state.modify_target_group(tg_arn, health_check_path) {
            Ok(tg) => {
                wire::serialize_modify_target_group_response(&wire::ModifyTargetGroupOutput {
                    target_groups: Some(vec![tg_to_wire(tg)].into()),
                })
            }
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_modify_target_group_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_target_group_attributes_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.target_group_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TargetGroupArn'");
        }
        let tg_arn = &input.target_group_arn;

        let attributes: Vec<(String, String)> = input
            .attributes
            .items
            .iter()
            .map(|a| {
                (
                    a.key.clone().unwrap_or_default(),
                    a.value.clone().unwrap_or_default(),
                )
            })
            .collect();

        let mut state = state.write().await;
        match state.modify_target_group_attributes(tg_arn, &attributes) {
            Ok(attrs) => wire::serialize_modify_target_group_attributes_response(
                &wire::ModifyTargetGroupAttributesOutput {
                    attributes: Some(
                        attrs
                            .iter()
                            .map(|(k, v)| wire::TargetGroupAttribute {
                                key: Some(k.clone()),
                                value: Some(v.clone()),
                            })
                            .collect::<Vec<_>>()
                            .into(),
                    ),
                },
            ),
            Err(e) => elbv2_error_response(&e),
        }
    }

    // --- Listener operations ---

    async fn handle_create_listener(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_listener_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'LoadBalancerArn'");
        }
        let lb_arn = &input.load_balancer_arn;
        let port: i32 = input.port.unwrap_or(80);
        let protocol = input.protocol.as_deref().unwrap_or("HTTP");

        let default_actions: Vec<ListenerAction> = input
            .default_actions
            .items
            .iter()
            .map(|a| ListenerAction {
                action_type: a.r#type.clone(),
                target_group_arn: a.target_group_arn.clone().unwrap_or_default(),
            })
            .collect();

        let mut state = state.write().await;
        match state.create_listener(lb_arn, port, protocol, default_actions, account_id, region) {
            Ok(listener) => wire::serialize_create_listener_response(&wire::CreateListenerOutput {
                listeners: Some(vec![listener_to_wire(listener)].into()),
            }),
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_delete_listener(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_listener_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.listener_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ListenerArn'");
        }
        let arn = &input.listener_arn;

        let mut state = state.write().await;
        match state.delete_listener(arn) {
            Ok(()) => wire::serialize_delete_listener_response(&wire::DeleteListenerOutput {}),
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_describe_listeners(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_listeners_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let lb_arn_owned = input.load_balancer_arn;
        let lb_arn = if lb_arn_owned.as_deref().is_none_or(|s| s.is_empty()) {
            None
        } else {
            lb_arn_owned.as_deref()
        };
        let listener_arns = input.listener_arns.map(|l| l.items).unwrap_or_default();

        if lb_arn.is_none() && listener_arns.is_empty() {
            return elbv2_error_response(&ElbV2Error::ValidationError);
        }

        let state = state.read().await;

        let listeners: Vec<&_> = if !listener_arns.is_empty() {
            // Validate all listener ARNs exist
            for arn in &listener_arns {
                if !state.listeners.contains_key(arn) {
                    return elbv2_error_response(&ElbV2Error::ListenerNotFoundList);
                }
            }
            state
                .listeners
                .values()
                .filter(|l| listener_arns.contains(&l.arn))
                .collect()
        } else {
            state.describe_listeners(lb_arn.unwrap())
        };

        wire::serialize_describe_listeners_response(&wire::DescribeListenersOutput {
            listeners: Some(
                listeners
                    .iter()
                    .map(|l| listener_to_wire(l))
                    .collect::<Vec<_>>()
                    .into(),
            ),
            next_marker: None,
        })
    }

    async fn handle_modify_listener(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_listener_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.listener_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ListenerArn'");
        }
        let listener_arn = &input.listener_arn;
        let port = input.port;
        let protocol = input.protocol.as_deref();
        let default_actions = input.default_actions.map(|actions| {
            actions
                .items
                .iter()
                .map(|a| ListenerAction {
                    action_type: a.r#type.clone(),
                    target_group_arn: a.target_group_arn.clone().unwrap_or_default(),
                })
                .collect()
        });

        let mut state = state.write().await;
        match state.modify_listener(listener_arn, port, protocol, default_actions) {
            Ok(listener) => wire::serialize_modify_listener_response(&wire::ModifyListenerOutput {
                listeners: Some(vec![listener_to_wire(listener)].into()),
            }),
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_describe_listener_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_listener_attributes_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.listener_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ListenerArn'");
        }
        let listener_arn = &input.listener_arn;

        let state = state.read().await;
        match state.describe_listener_attributes(listener_arn) {
            Ok(attrs) => wire::serialize_describe_listener_attributes_response(
                &wire::DescribeListenerAttributesOutput {
                    attributes: Some(
                        attrs
                            .iter()
                            .map(|(k, v)| wire::ListenerAttribute {
                                key: Some(k.clone()),
                                value: Some(v.clone()),
                            })
                            .collect::<Vec<_>>()
                            .into(),
                    ),
                },
            ),
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_modify_listener_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_listener_attributes_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.listener_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ListenerArn'");
        }
        let listener_arn = &input.listener_arn;

        let attributes: Vec<(String, String)> = input
            .attributes
            .items
            .iter()
            .map(|a| {
                (
                    a.key.clone().unwrap_or_default(),
                    a.value.clone().unwrap_or_default(),
                )
            })
            .collect();

        let mut state = state.write().await;
        match state.modify_listener_attributes(listener_arn, &attributes) {
            Ok(attrs) => wire::serialize_modify_listener_attributes_response(
                &wire::ModifyListenerAttributesOutput {
                    attributes: Some(
                        attrs
                            .iter()
                            .map(|(k, v)| wire::ListenerAttribute {
                                key: Some(k.clone()),
                                value: Some(v.clone()),
                            })
                            .collect::<Vec<_>>()
                            .into(),
                    ),
                },
            ),
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_describe_listener_certificates(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_listener_certificates_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.listener_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ListenerArn'");
        }
        let listener_arn = &input.listener_arn;

        let state = state.read().await;
        match state.describe_listener_certificates(listener_arn) {
            Ok(certs) => wire::serialize_describe_listener_certificates_response(
                &wire::DescribeListenerCertificatesOutput {
                    certificates: Some(
                        certs
                            .iter()
                            .map(|c| wire::Certificate {
                                certificate_arn: Some(c.certificate_arn.clone()),
                                is_default: c.is_default,
                            })
                            .collect::<Vec<_>>()
                            .into(),
                    ),
                    next_marker: None,
                },
            ),
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_add_listener_certificates(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_add_listener_certificates_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.listener_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ListenerArn'");
        }
        let listener_arn = &input.listener_arn;

        let certificates: Vec<crate::types::Certificate> = input
            .certificates
            .items
            .iter()
            .map(|c| crate::types::Certificate {
                certificate_arn: c.certificate_arn.clone().unwrap_or_default(),
                is_default: c.is_default,
            })
            .collect();

        let mut state = state.write().await;
        match state.add_listener_certificates(listener_arn, certificates) {
            Ok(certs) => wire::serialize_add_listener_certificates_response(
                &wire::AddListenerCertificatesOutput {
                    certificates: Some(
                        certs
                            .iter()
                            .map(|c| wire::Certificate {
                                certificate_arn: Some(c.certificate_arn.clone()),
                                is_default: c.is_default,
                            })
                            .collect::<Vec<_>>()
                            .into(),
                    ),
                },
            ),
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_remove_listener_certificates(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_remove_listener_certificates_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.listener_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ListenerArn'");
        }
        let listener_arn = &input.listener_arn;

        let cert_arns: Vec<String> = input
            .certificates
            .items
            .iter()
            .filter_map(|c| c.certificate_arn.clone())
            .collect();

        let mut state = state.write().await;
        match state.remove_listener_certificates(listener_arn, &cert_arns) {
            Ok(()) => wire::serialize_remove_listener_certificates_response(
                &wire::RemoveListenerCertificatesOutput {},
            ),
            Err(e) => elbv2_error_response(&e),
        }
    }

    // --- Rule operations ---

    async fn handle_create_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_rule_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.listener_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ListenerArn'");
        }
        let listener_arn = &input.listener_arn;
        let priority_str = if input.priority == 0 {
            "100".to_string()
        } else {
            input.priority.to_string()
        };
        let priority = &priority_str;

        let conditions: Vec<crate::types::RuleCondition> = input
            .conditions
            .items
            .iter()
            .map(|c| {
                let values = c
                    .values
                    .as_ref()
                    .map(|l| l.items.clone())
                    .unwrap_or_default();
                crate::types::RuleCondition {
                    field: c.field.clone().unwrap_or_default(),
                    values,
                }
            })
            .collect();
        let actions: Vec<crate::types::RuleAction> = input
            .actions
            .items
            .iter()
            .map(|a| crate::types::RuleAction {
                action_type: a.r#type.clone(),
                target_group_arn: a.target_group_arn.clone().unwrap_or_default(),
            })
            .collect();

        let mut state = state.write().await;
        match state.create_rule(
            listener_arn,
            priority,
            conditions,
            actions,
            account_id,
            region,
        ) {
            Ok(rule) => {
                let wire_rule = types_rule_to_wire(rule);
                wire::serialize_create_rule_response(&wire::CreateRuleOutput {
                    rules: Some(wire::Rules::from(vec![wire_rule])),
                })
            }
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_delete_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_rule_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.rule_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RuleArn'");
        }
        let rule_arn = &input.rule_arn;

        let mut state = state.write().await;
        match state.delete_rule(rule_arn) {
            Ok(()) => wire::serialize_delete_rule_response(&wire::DeleteRuleOutput {}),
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_describe_rules(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_rules_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let listener_arn_owned = input.listener_arn;
        let listener_arn = listener_arn_owned.as_deref().filter(|s| !s.is_empty());
        let rule_arns = input.rule_arns.map(|l| l.items).unwrap_or_default();

        let state = state.read().await;
        let rules = state.describe_rules(listener_arn, &rule_arns);

        let wire_rules: Vec<wire::Rule> = rules.iter().map(|r| types_rule_to_wire(r)).collect();
        wire::serialize_describe_rules_response(&wire::DescribeRulesOutput {
            rules: Some(wire::Rules::from(wire_rules)),
            next_marker: None,
        })
    }

    async fn handle_modify_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_rule_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.rule_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'RuleArn'");
        }
        let rule_arn = &input.rule_arn;

        let conditions = input.conditions.map(|conds| {
            conds
                .items
                .iter()
                .map(|c| {
                    let values = c
                        .values
                        .as_ref()
                        .map(|l| l.items.clone())
                        .unwrap_or_default();
                    crate::types::RuleCondition {
                        field: c.field.clone().unwrap_or_default(),
                        values,
                    }
                })
                .collect()
        });
        let actions = input.actions.map(|acts| {
            acts.items
                .iter()
                .map(|a| crate::types::RuleAction {
                    action_type: a.r#type.clone(),
                    target_group_arn: a.target_group_arn.clone().unwrap_or_default(),
                })
                .collect()
        });

        let mut state = state.write().await;
        match state.modify_rule(rule_arn, conditions, actions) {
            Ok(rule) => {
                let wire_rule = types_rule_to_wire(rule);
                wire::serialize_modify_rule_response(&wire::ModifyRuleOutput {
                    rules: Some(wire::Rules::from(vec![wire_rule])),
                })
            }
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_set_rule_priorities(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_set_rule_priorities_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let rule_priorities: Vec<(String, String)> = input
            .rule_priorities
            .items
            .iter()
            .filter_map(|rp| {
                let arn = rp.rule_arn.clone()?;
                let prio = rp.priority.map(|p| p.to_string())?;
                Some((arn, prio))
            })
            .collect();

        let mut state = state.write().await;
        match state.set_rule_priorities(&rule_priorities) {
            Ok(rules) => {
                let wire_rules: Vec<wire::Rule> =
                    rules.iter().map(|r| types_rule_to_wire(r)).collect();
                wire::serialize_set_rule_priorities_response(&wire::SetRulePrioritiesOutput {
                    rules: Some(wire::Rules::from(wire_rules)),
                })
            }
            Err(e) => elbv2_error_response(&e),
        }
    }

    // --- Target operations ---

    async fn handle_register_targets(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_register_targets_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.target_group_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TargetGroupArn'");
        }
        let tg_arn = &input.target_group_arn;

        let targets: Vec<crate::types::TargetDescription> = input
            .targets
            .items
            .iter()
            .map(|t| crate::types::TargetDescription {
                id: t.id.clone(),
                port: t.port,
                availability_zone: t.availability_zone.clone(),
            })
            .collect();

        let mut state = state.write().await;
        match state.register_targets(tg_arn, targets) {
            Ok(()) => wire::serialize_register_targets_response(&wire::RegisterTargetsOutput {}),
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_deregister_targets(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_deregister_targets_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.target_group_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TargetGroupArn'");
        }
        let tg_arn = &input.target_group_arn;

        let targets: Vec<crate::types::TargetDescription> = input
            .targets
            .items
            .iter()
            .map(|t| crate::types::TargetDescription {
                id: t.id.clone(),
                port: t.port,
                availability_zone: t.availability_zone.clone(),
            })
            .collect();

        let mut state = state.write().await;
        match state.deregister_targets(tg_arn, &targets) {
            Ok(()) => {
                wire::serialize_deregister_targets_response(&wire::DeregisterTargetsOutput {})
            }
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_describe_target_health(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_target_health_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.target_group_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TargetGroupArn'");
        }
        let tg_arn = &input.target_group_arn;

        let state = state.read().await;
        match state.describe_target_health(tg_arn) {
            Ok(targets) => {
                let descs: Vec<wire::TargetHealthDescription> = targets
                    .iter()
                    .map(|t| wire::TargetHealthDescription {
                        target: Some(wire::TargetDescription {
                            id: t.id.clone(),
                            port: t.port,
                            availability_zone: t.availability_zone.clone(),
                            quic_server_id: None,
                        }),
                        target_health: Some(wire::TargetHealth {
                            state: Some("healthy".to_string()),
                            reason: None,
                            description: None,
                        }),
                        health_check_port: None,
                        administrative_override: None,
                        anomaly_detection: None,
                    })
                    .collect();
                wire::serialize_describe_target_health_response(&wire::DescribeTargetHealthOutput {
                    target_health_descriptions: Some(descs.into()),
                })
            }
            Err(e) => elbv2_error_response(&e),
        }
    }

    // --- Tag operations ---

    async fn handle_add_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_add_tags_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let resource_arns = input.resource_arns.items;
        if resource_arns.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ResourceArns'");
        }

        let tags: Vec<(String, String)> = input
            .tags
            .items
            .iter()
            .map(|t| (t.key.clone(), t.value.clone().unwrap_or_default()))
            .collect();

        let mut state = state.write().await;
        match state.add_tags(&resource_arns, &tags) {
            Ok(()) => wire::serialize_add_tags_response(&wire::AddTagsOutput {}),
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_remove_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_remove_tags_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let resource_arns = input.resource_arns.items;
        if resource_arns.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ResourceArns'");
        }

        let tag_keys = input.tag_keys.items;

        let mut state = state.write().await;
        match state.remove_tags(&resource_arns, &tag_keys) {
            Ok(()) => wire::serialize_remove_tags_response(&wire::RemoveTagsOutput {}),
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_describe_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_tags_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let resource_arns = input.resource_arns.items;

        let state = state.read().await;
        let tag_descs = state.describe_tags(&resource_arns);

        let empty_tags: HashMap<String, String> = HashMap::new();
        let tag_descriptions: Vec<wire::TagDescription> = tag_descs
            .iter()
            .map(|(arn, tags_opt)| {
                let tags = tags_opt.unwrap_or(&empty_tags);
                wire::TagDescription {
                    resource_arn: Some((*arn).to_string()),
                    tags: Some(
                        tags.iter()
                            .map(|(k, v)| wire::Tag {
                                key: k.clone(),
                                value: Some(v.clone()),
                            })
                            .collect::<Vec<_>>()
                            .into(),
                    ),
                }
            })
            .collect();

        wire::serialize_describe_tags_response(&wire::DescribeTagsOutput {
            tag_descriptions: Some(tag_descriptions.into()),
        })
    }

    // --- Set operations ---

    async fn handle_set_ip_address_type(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_set_ip_address_type_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'LoadBalancerArn'");
        }
        if input.ip_address_type.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'IpAddressType'");
        }
        let lb_arn = &input.load_balancer_arn;
        let ip_type = &input.ip_address_type;

        let mut state = state.write().await;
        match state.set_ip_address_type(lb_arn, ip_type) {
            Ok(ip_address_type) => {
                wire::serialize_set_ip_address_type_response(&wire::SetIpAddressTypeOutput {
                    ip_address_type: Some(ip_address_type.to_string()),
                })
            }
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_set_security_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_set_security_groups_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'LoadBalancerArn'");
        }
        let lb_arn = &input.load_balancer_arn;

        let security_groups = input.security_groups.items;

        let mut state = state.write().await;
        match state.set_security_groups(lb_arn, security_groups) {
            Ok(sgs) => {
                wire::serialize_set_security_groups_response(&wire::SetSecurityGroupsOutput {
                    security_group_ids: Some(sgs.to_vec().into()),
                    enforce_security_group_inbound_rules_on_private_link_traffic: None,
                })
            }
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_set_subnets(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_set_subnets_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'LoadBalancerArn'");
        }
        let lb_arn = &input.load_balancer_arn;

        let subnets = input.subnets.map(|l| l.items).unwrap_or_default();

        let mut state = state.write().await;
        match state.set_subnets(lb_arn, subnets, region) {
            Ok(azs) => {
                let wire_azs: Vec<wire::AvailabilityZone> = azs
                    .iter()
                    .map(|az| wire::AvailabilityZone {
                        zone_name: Some(az.zone_name.clone()),
                        subnet_id: Some(az.subnet_id.clone()),
                        load_balancer_addresses: None,
                        outpost_id: None,
                        source_nat_ipv6_prefixes: None,
                    })
                    .collect();
                wire::serialize_set_subnets_response(&wire::SetSubnetsOutput {
                    availability_zones: Some(wire_azs.into()),
                    enable_prefix_for_ipv6_source_nat: None,
                    ip_address_type: None,
                })
            }
            Err(e) => elbv2_error_response(&e),
        }
    }

    // --- TrustStore operations ---

    async fn handle_create_trust_store(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_trust_store_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'Name'");
        }
        let name = &input.name;
        let mut st = state.write().await;
        match st.create_trust_store(name, account_id, region) {
            Ok(ts) => wire::serialize_create_trust_store_response(&wire::CreateTrustStoreOutput {
                trust_stores: Some(vec![ts_to_wire(ts)].into()),
            }),
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_delete_trust_store(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_trust_store_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.trust_store_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TrustStoreArn'");
        }
        let arn = &input.trust_store_arn;
        let mut st = state.write().await;
        match st.delete_trust_store(arn) {
            Ok(()) => wire::serialize_delete_trust_store_response(&wire::DeleteTrustStoreOutput {}),
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_describe_trust_stores(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_trust_stores_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let arns = input.trust_store_arns.map(|l| l.items).unwrap_or_default();
        let names = input.names.map(|l| l.items).unwrap_or_default();
        let st = state.read().await;
        let stores = st.describe_trust_stores(&arns, &names);

        wire::serialize_describe_trust_stores_response(&wire::DescribeTrustStoresOutput {
            trust_stores: Some(
                stores
                    .iter()
                    .map(|ts| ts_to_wire(ts))
                    .collect::<Vec<_>>()
                    .into(),
            ),
            next_marker: None,
        })
    }

    async fn handle_modify_trust_store(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_trust_store_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.trust_store_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TrustStoreArn'");
        }
        let arn = &input.trust_store_arn;
        let mut st = state.write().await;
        match st.modify_trust_store(arn) {
            Ok(ts) => wire::serialize_modify_trust_store_response(&wire::ModifyTrustStoreOutput {
                trust_stores: Some(vec![ts_to_wire(ts)].into()),
            }),
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_add_trust_store_revocations(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_add_trust_store_revocations_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.trust_store_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TrustStoreArn'");
        }
        let arn = &input.trust_store_arn;
        // Extract from the first revocation content entry
        let first_rev = input
            .revocation_contents
            .map(|l| l.items)
            .unwrap_or_default();
        let number: i64 = params
            .get("RevocationContents.member.1.NumberOfRevokedEntries")
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);
        let rev_type_owned = first_rev
            .first()
            .and_then(|r| r.revocation_type.clone())
            .unwrap_or_else(|| "CRL".to_string());
        let rev_type = rev_type_owned.as_str();

        let mut st = state.write().await;
        match st.add_trust_store_revocations(arn, number, rev_type) {
            Ok(entries) => {
                let arn_clone = arn.clone();
                let revocations: Vec<wire::TrustStoreRevocation> = entries
                    .iter()
                    .map(|(rev_id, num, rtype)| wire::TrustStoreRevocation {
                        trust_store_arn: Some(arn_clone.clone()),
                        revocation_id: Some(*rev_id),
                        revocation_type: Some(rtype.clone()),
                        number_of_revoked_entries: Some(*num),
                    })
                    .collect();
                wire::serialize_add_trust_store_revocations_response(
                    &wire::AddTrustStoreRevocationsOutput {
                        trust_store_revocations: Some(revocations.into()),
                    },
                )
            }
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_remove_trust_store_revocations(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_remove_trust_store_revocations_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.trust_store_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TrustStoreArn'");
        }
        let arn = &input.trust_store_arn;
        let rev_ids: Vec<i64> = input.revocation_ids.items;

        let mut st = state.write().await;
        match st.remove_trust_store_revocations(arn, &rev_ids) {
            Ok(()) => wire::serialize_remove_trust_store_revocations_response(
                &wire::RemoveTrustStoreRevocationsOutput {},
            ),
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_describe_trust_store_revocations(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_trust_store_revocations_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.trust_store_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TrustStoreArn'");
        }
        let arn = &input.trust_store_arn;
        let st = state.read().await;
        match st.describe_trust_store_revocations(arn) {
            Ok(entries) => {
                let arn_clone = arn.clone();
                let revocations: Vec<wire::DescribeTrustStoreRevocation> = entries
                    .iter()
                    .map(|(rev_id, num, rtype)| wire::DescribeTrustStoreRevocation {
                        trust_store_arn: Some(arn_clone.clone()),
                        revocation_id: Some(*rev_id),
                        revocation_type: Some(rtype.clone()),
                        number_of_revoked_entries: Some(*num),
                    })
                    .collect();
                wire::serialize_describe_trust_store_revocations_response(
                    &wire::DescribeTrustStoreRevocationsOutput {
                        trust_store_revocations: Some(revocations.into()),
                        next_marker: None,
                    },
                )
            }
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_describe_trust_store_associations(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_trust_store_associations_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.trust_store_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TrustStoreArn'");
        }
        let trust_store_arn = input.trust_store_arn.as_str();
        let state = state.read().await;
        match state.describe_trust_store_associations(trust_store_arn) {
            Ok(assocs) => {
                let items: Vec<wire::TrustStoreAssociation> = assocs
                    .iter()
                    .map(|a| wire::TrustStoreAssociation {
                        resource_arn: Some(a.resource_arn.clone()),
                    })
                    .collect();
                wire::serialize_describe_trust_store_associations_response(
                    &wire::DescribeTrustStoreAssociationsOutput {
                        trust_store_associations: Some(items.into()),
                        next_marker: None,
                    },
                )
            }
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_delete_shared_trust_store_association(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_shared_trust_store_association_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.trust_store_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TrustStoreArn'");
        }
        if input.resource_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'ResourceArn'");
        }
        let trust_store_arn = input.trust_store_arn.as_str();
        let resource_arn = input.resource_arn.as_str();
        let mut state = state.write().await;
        match state.delete_shared_trust_store_association(trust_store_arn, resource_arn) {
            Ok(()) => wire::serialize_delete_shared_trust_store_association_response(
                &wire::DeleteSharedTrustStoreAssociationOutput {},
            ),
            Err(e) => elbv2_error_response(&e),
        }
    }

    // Trust store CA certificates bundle: validates trust store exists, returns placeholder URL.
    async fn handle_get_trust_store_ca_certificates_bundle(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_trust_store_ca_certificates_bundle_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.trust_store_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TrustStoreArn'");
        }
        let trust_store_arn = input.trust_store_arn.as_str();
        let state = state.read().await;
        if !state.trust_stores.contains_key(trust_store_arn) {
            return elbv2_error_response(&ElbV2Error::TrustStoreNotFound(
                trust_store_arn.to_string(),
            ));
        }
        wire::serialize_get_trust_store_ca_certificates_bundle_response(
            &wire::GetTrustStoreCaCertificatesBundleOutput {
                location: Some(
                    "https://s3.amazonaws.com/mock-bucket/mock-ca-bundle.pem".to_string(),
                ),
            },
        )
    }

    // Trust store revocation content: validates trust store exists, returns placeholder URL.
    async fn handle_get_trust_store_revocation_content(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_trust_store_revocation_content_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.trust_store_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TrustStoreArn'");
        }
        let trust_store_arn = input.trust_store_arn.as_str();
        let state = state.read().await;
        if !state.trust_stores.contains_key(trust_store_arn) {
            return elbv2_error_response(&ElbV2Error::TrustStoreNotFound(
                trust_store_arn.to_string(),
            ));
        }
        wire::serialize_get_trust_store_revocation_content_response(
            &wire::GetTrustStoreRevocationContentOutput {
                location: Some(
                    "https://s3.amazonaws.com/mock-bucket/mock-revocation.crl".to_string(),
                ),
            },
        )
    }

    // --- Account / capacity / SSL / misc ---

    // STUB[no-telemetry]: DescribeAccountLimits returns hardcoded limit values; real limits are driven by account-level configuration that the mock does not track
    async fn handle_describe_account_limits(&self) -> MockResponse {
        let limits: Vec<wire::Limit> = [
            ("application-load-balancers", "20"),
            ("target-groups", "3000"),
            ("listeners-per-application-load-balancer", "50"),
            ("listeners-per-network-load-balancer", "50"),
            ("rules-per-application-load-balancer", "100"),
            ("network-load-balancers", "20"),
            ("certificates-per-application-load-balancer", "25"),
            ("targets-per-application-load-balancer", "1000"),
            ("targets-per-network-load-balancer", "3000"),
            ("listeners", "50"),
        ]
        .iter()
        .map(|(name, max)| wire::Limit {
            name: Some(name.to_string()),
            max: Some(max.to_string()),
        })
        .collect();
        wire::serialize_describe_account_limits_response(&wire::DescribeAccountLimitsOutput {
            limits: Some(limits.into()),
            next_marker: None,
        })
    }

    async fn handle_describe_capacity_reservation(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_capacity_reservation_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let lb_arn = if input.load_balancer_arn.is_empty() {
            None
        } else {
            Some(input.load_balancer_arn.as_str())
        };
        let state = state.read().await;
        let (cap_states, remaining, min_cap) = if let Some(arn) = lb_arn {
            if let Some(reservation) = state.get_capacity_reservation(arn) {
                let zonal: Vec<wire::ZonalCapacityReservationState> = reservation
                    .availability_zone_states
                    .iter()
                    .map(|z| wire::ZonalCapacityReservationState {
                        availability_zone: Some(z.availability_zone.clone()),
                        effective_capacity_units: Some(z.effective_capacity_units),
                        state: Some(wire::CapacityReservationStatus {
                            code: Some(z.status_code.clone()),
                            reason: None,
                        }),
                    })
                    .collect();
                (
                    zonal,
                    reservation.decrease_requests_remaining,
                    reservation.minimum_capacity_units,
                )
            } else {
                (
                    vec![wire::ZonalCapacityReservationState {
                        availability_zone: Some("us-east-1a".to_string()),
                        effective_capacity_units: Some(0.0),
                        state: Some(wire::CapacityReservationStatus {
                            code: Some("provisioned".to_string()),
                            reason: None,
                        }),
                    }],
                    10,
                    0,
                )
            }
        } else {
            (
                vec![wire::ZonalCapacityReservationState {
                    availability_zone: Some("us-east-1a".to_string()),
                    effective_capacity_units: Some(0.0),
                    state: Some(wire::CapacityReservationStatus {
                        code: Some("provisioned".to_string()),
                        reason: None,
                    }),
                }],
                10,
                0,
            )
        };
        wire::serialize_describe_capacity_reservation_response(
            &wire::DescribeCapacityReservationOutput {
                capacity_reservation_state: Some(cap_states.into()),
                decrease_requests_remaining: Some(remaining),
                last_modified_time: None,
                minimum_load_balancer_capacity: Some(wire::MinimumLoadBalancerCapacity {
                    capacity_units: Some(min_cap),
                }),
            },
        )
    }

    async fn handle_modify_capacity_reservation(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_capacity_reservation_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'LoadBalancerArn'");
        }
        let lb_arn = input.load_balancer_arn.as_str();
        let min_cap = input
            .minimum_load_balancer_capacity
            .and_then(|m| m.capacity_units);

        let mut state = state.write().await;
        match state.modify_capacity_reservation(lb_arn, min_cap) {
            Ok(reservation) => {
                let zonal: Vec<wire::ZonalCapacityReservationState> = reservation
                    .availability_zone_states
                    .iter()
                    .map(|z| wire::ZonalCapacityReservationState {
                        availability_zone: Some(z.availability_zone.clone()),
                        effective_capacity_units: Some(z.effective_capacity_units),
                        state: Some(wire::CapacityReservationStatus {
                            code: Some(z.status_code.clone()),
                            reason: None,
                        }),
                    })
                    .collect();
                let zonal = if zonal.is_empty() {
                    vec![wire::ZonalCapacityReservationState {
                        availability_zone: Some("us-east-1a".to_string()),
                        effective_capacity_units: Some(0.0),
                        state: Some(wire::CapacityReservationStatus {
                            code: Some("provisioned".to_string()),
                            reason: None,
                        }),
                    }]
                } else {
                    zonal
                };
                wire::serialize_modify_capacity_reservation_response(
                    &wire::ModifyCapacityReservationOutput {
                        capacity_reservation_state: Some(zonal.into()),
                        decrease_requests_remaining: Some(reservation.decrease_requests_remaining),
                        last_modified_time: None,
                        minimum_load_balancer_capacity: Some(wire::MinimumLoadBalancerCapacity {
                            capacity_units: Some(reservation.minimum_capacity_units),
                        }),
                    },
                )
            }
            Err(e) => elbv2_error_response(&e),
        }
    }

    // SSL policies are reference data; hardcoded defaults are served from state.
    async fn handle_describe_ssl_policies(&self) -> MockResponse {
        // SSL policies are static reference data. Return commonly used policies.
        wire::serialize_describe_s_s_l_policies_response(&wire::DescribeSSLPoliciesOutput {
            ssl_policies: Some(
                vec![wire::SslPolicy {
                    name: Some("ELBSecurityPolicy-2016-08".to_string()),
                    ssl_protocols: Some(vec!["TLSv1.2".to_string(), "TLSv1.1".to_string()].into()),
                    ciphers: Some(
                        vec![
                            wire::Cipher {
                                name: Some("ECDHE-ECDSA-AES128-GCM-SHA256".to_string()),
                                priority: Some(1),
                            },
                            wire::Cipher {
                                name: Some("ECDHE-RSA-AES128-GCM-SHA256".to_string()),
                                priority: Some(2),
                            },
                        ]
                        .into(),
                    ),
                    supported_load_balancer_types: Some(
                        vec!["application".to_string(), "network".to_string()].into(),
                    ),
                }]
                .into(),
            ),
            next_marker: None,
        })
    }

    async fn handle_describe_target_group_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_target_group_attributes_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.target_group_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'TargetGroupArn'");
        }

        let st = state.read().await;
        match st.describe_target_group_attributes(&input.target_group_arn) {
            Ok(attrs) => {
                // Return default attributes merged with stored ones.
                let mut effective: std::collections::HashMap<String, String> = [
                    ("deregistration_delay.timeout_seconds", "300"),
                    ("stickiness.enabled", "false"),
                    ("stickiness.type", "lb_cookie"),
                    ("load_balancing.algorithm.type", "round_robin"),
                ]
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
                for (k, v) in attrs {
                    effective.insert(k.clone(), v.clone());
                }
                wire::serialize_describe_target_group_attributes_response(
                    &wire::DescribeTargetGroupAttributesOutput {
                        attributes: Some(
                            effective
                                .iter()
                                .map(|(k, v)| wire::TargetGroupAttribute {
                                    key: Some(k.clone()),
                                    value: Some(v.clone()),
                                })
                                .collect::<Vec<_>>()
                                .into(),
                        ),
                    },
                )
            }
            Err(e) => elbv2_error_response(&e),
        }
    }

    async fn handle_get_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_resource_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let resource_arn = if input.resource_arn.is_empty() {
            None
        } else {
            Some(input.resource_arn.as_str())
        };
        let state = state.read().await;
        let policy = resource_arn
            .and_then(|arn| state.get_resource_policy(arn))
            .map(|p| p.policy.clone());
        wire::serialize_get_resource_policy_response(&wire::GetResourcePolicyOutput { policy })
    }

    async fn handle_modify_ip_pools(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbV2State>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_ip_pools_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "Missing 'LoadBalancerArn'");
        }
        let lb_arn = input.load_balancer_arn.as_str();
        let ipv4_pool_id = input
            .ipam_pools
            .as_ref()
            .and_then(|p| p.ipv4_ipam_pool_id.as_deref());

        let mut state = state.write().await;
        match state.modify_ip_pools(lb_arn, ipv4_pool_id, None) {
            Ok(pool) => {
                let ipam_pools = pool.ipv4_ipam_pool_id.as_ref().map(|id| wire::IpamPools {
                    ipv4_ipam_pool_id: Some(id.clone()),
                });
                wire::serialize_modify_ip_pools_response(&wire::ModifyIpPoolsOutput { ipam_pools })
            }
            Err(e) => elbv2_error_response(&e),
        }
    }
}

// --- Utility functions ---

fn elbv2_error_response(err: &ElbV2Error) -> MockResponse {
    let (status, code) = match err {
        ElbV2Error::DuplicateLoadBalancerName(_) => (400, "DuplicateLoadBalancerName"),
        ElbV2Error::LoadBalancerNotFound(_) => (400, "LoadBalancerNotFound"),
        ElbV2Error::LoadBalancerNotFoundList(_) => (400, "LoadBalancerNotFound"),
        ElbV2Error::DuplicateTargetGroupName(_) => (400, "DuplicateTargetGroupName"),
        ElbV2Error::TargetGroupNotFound(_) => (400, "TargetGroupNotFound"),
        ElbV2Error::ListenerNotFound(_) => (400, "ListenerNotFound"),
        ElbV2Error::ListenerNotFoundList => (400, "ListenerNotFound"),
        ElbV2Error::PriorityInUse => (400, "PriorityInUse"),
        ElbV2Error::RuleNotFound(_) => (400, "RuleNotFound"),
        ElbV2Error::OperationNotPermitted => (400, "OperationNotPermitted"),
        ElbV2Error::DuplicateTrustStoreName(_) => (400, "DuplicateTrustStoreName"),
        ElbV2Error::TrustStoreNotFound(_) => (400, "TrustStoreNotFound"),
        ElbV2Error::ValidationError => (400, "ValidationError"),
    };
    let body = format!(
        r#"<ErrorResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2015-12-01/">
  <Error>
    <Type>Sender</Type>
    <Code>{code}</Code>
    <Message>{message}</Message>
  </Error>
  <RequestId>{request_id}</RequestId>
</ErrorResponse>"#,
        code = xml_escape(code),
        message = xml_escape(&err.to_string()),
        request_id = uuid::Uuid::new_v4(),
    );
    MockResponse::xml(status, body)
}

fn parse_query_string(s: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for pair in s.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            let key = urldecode(key);
            let value = urldecode(value);
            map.insert(key, value);
        }
    }
    map
}

fn urldecode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.bytes();
    while let Some(b) = chars.next() {
        match b {
            b'+' => result.push(' '),
            b'%' => {
                let hi = chars.next().and_then(hex_val);
                let lo = chars.next().and_then(hex_val);
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    result.push((hi << 4 | lo) as char);
                }
            }
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

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn parse_member_list(params: &HashMap<String, String>, prefix: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut i = 1;
    loop {
        let key = format!("{prefix}.member.{i}");
        match params.get(&key) {
            Some(v) => {
                result.push(v.clone());
                i += 1;
            }
            None => break,
        }
    }
    result
}

/// Convert an internal `types::Rule` to the wire `model::Rule` for serialization.
fn types_rule_to_wire(rule: &crate::types::Rule) -> wire::Rule {
    let conditions: Vec<wire::RuleCondition> = rule
        .conditions
        .iter()
        .map(|c| {
            let values_list = wire::ListOfString::from(c.values.clone());
            let mut wc = wire::RuleCondition {
                field: Some(c.field.clone()),
                values: Some(values_list.clone()),
                ..Default::default()
            };
            // The terraform-provider-aws ListenerRule reader dereferences the
            // matching `*Config` block based on the condition field and panics
            // if it is nil, so populate the block alongside the legacy `Values`.
            match c.field.as_str() {
                "host-header" => {
                    wc.host_header_config = Some(wire::HostHeaderConditionConfig {
                        values: Some(values_list),
                        ..Default::default()
                    });
                }
                "path-pattern" => {
                    wc.path_pattern_config = Some(wire::PathPatternConditionConfig {
                        values: Some(values_list),
                        ..Default::default()
                    });
                }
                "http-request-method" => {
                    wc.http_request_method_config = Some(wire::HttpRequestMethodConditionConfig {
                        values: Some(values_list),
                    });
                }
                "source-ip" => {
                    wc.source_ip_config = Some(wire::SourceIpConditionConfig {
                        values: Some(values_list),
                    });
                }
                _ => {}
            }
            wc
        })
        .collect();

    let actions: Vec<wire::Action> = rule
        .actions
        .iter()
        .enumerate()
        .map(|(i, a)| wire::Action {
            r#type: a.action_type.clone(),
            target_group_arn: Some(a.target_group_arn.clone()),
            order: Some((i as i32) + 1),
            ..Default::default()
        })
        .collect();

    wire::Rule {
        rule_arn: Some(rule.rule_arn.clone()),
        priority: Some(rule.priority.clone()),
        is_default: Some(rule.is_default),
        conditions: Some(wire::RuleConditionList::from(conditions)),
        actions: Some(wire::Actions::from(actions)),
        transforms: None,
    }
}

fn lb_to_wire(lb: &crate::types::LoadBalancer) -> wire::LoadBalancer {
    let azs: Vec<wire::AvailabilityZone> = lb
        .availability_zones
        .iter()
        .map(|az| wire::AvailabilityZone {
            zone_name: Some(az.zone_name.clone()),
            subnet_id: Some(az.subnet_id.clone()),
            load_balancer_addresses: None,
            outpost_id: None,
            source_nat_ipv6_prefixes: None,
        })
        .collect();
    wire::LoadBalancer {
        load_balancer_arn: Some(lb.arn.clone()),
        d_n_s_name: Some(lb.dns_name.clone()),
        load_balancer_name: Some(lb.name.clone()),
        scheme: Some(lb.scheme.clone()),
        vpc_id: Some(lb.vpc_id.clone()),
        r#type: Some(lb.lb_type.clone()),
        state: Some(wire::LoadBalancerState {
            code: Some(lb.state.clone()),
            reason: None,
        }),
        created_time: Some(lb.created_time.to_rfc3339()),
        ip_address_type: Some(lb.ip_address_type.clone()),
        security_groups: Some(lb.security_groups.clone().into()),
        availability_zones: Some(azs.into()),
        canonical_hosted_zone_id: None,
        customer_owned_ipv4_pool: None,
        enable_prefix_for_ipv6_source_nat: None,
        enforce_security_group_inbound_rules_on_private_link_traffic: None,
        ipam_pools: None,
    }
}

fn tg_to_wire(tg: &crate::types::TargetGroup) -> wire::TargetGroup {
    wire::TargetGroup {
        target_group_arn: Some(tg.arn.clone()),
        target_group_name: Some(tg.name.clone()),
        protocol: Some(tg.protocol.clone()),
        port: Some(tg.port),
        vpc_id: Some(tg.vpc_id.clone()),
        health_check_path: Some(tg.health_check_path.clone()),
        target_type: Some(tg.target_type.clone()),
        health_check_enabled: Some(true),
        health_check_protocol: Some(tg.protocol.clone()),
        health_check_port: Some("traffic-port".to_string()),
        health_check_interval_seconds: Some(30),
        health_check_timeout_seconds: Some(5),
        healthy_threshold_count: Some(5),
        unhealthy_threshold_count: Some(2),
        matcher: Some(wire::Matcher {
            http_code: Some("200".to_string()),
            grpc_code: None,
        }),
        load_balancer_arns: None,
        ip_address_type: None,
        protocol_version: None,
        target_control_port: None,
    }
}

fn listener_to_wire(l: &crate::types::Listener) -> wire::Listener {
    let actions: Vec<wire::Action> = l
        .default_actions
        .iter()
        .enumerate()
        .map(|(i, a)| wire::Action {
            r#type: a.action_type.clone(),
            target_group_arn: Some(a.target_group_arn.clone()),
            order: Some((i as i32) + 1),
            ..Default::default()
        })
        .collect();
    let certs: Vec<wire::Certificate> = l
        .certificates
        .iter()
        .map(|c| wire::Certificate {
            certificate_arn: Some(c.certificate_arn.clone()),
            is_default: c.is_default,
        })
        .collect();
    wire::Listener {
        listener_arn: Some(l.arn.clone()),
        load_balancer_arn: Some(l.load_balancer_arn.clone()),
        port: Some(l.port),
        protocol: Some(l.protocol.clone()),
        default_actions: if actions.is_empty() {
            None
        } else {
            Some(wire::Actions::from(actions))
        },
        certificates: if certs.is_empty() {
            None
        } else {
            Some(wire::CertificateList::from(certs))
        },
        alpn_policy: None,
        mutual_authentication: None,
        ssl_policy: None,
    }
}

fn ts_to_wire(ts: &crate::types::TrustStore) -> wire::TrustStore {
    wire::TrustStore {
        trust_store_arn: Some(ts.arn.clone()),
        name: Some(ts.name.clone()),
        status: Some(ts.status.clone()),
        number_of_ca_certificates: Some(ts.number_of_ca_certificates),
        total_revoked_entries: Some(ts.total_revoked_entries),
    }
}
