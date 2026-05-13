use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::model::{
    AccessLog, AddAvailabilityZonesOutput, ApplySecurityGroupsToLoadBalancerOutput,
    AttachLoadBalancerToSubnetsOutput, AvailabilityZones, ConfigureHealthCheckOutput,
    ConnectionDraining, ConnectionSettings, CrossZoneLoadBalancing, DeregisterEndPointsOutput,
    DescribeAccessPointsOutput, DescribeEndPointStateOutput, DescribeLoadBalancerAttributesOutput,
    DescribeLoadBalancerPoliciesOutput, DescribeTagsOutput, HealthCheck, Instance, InstanceState,
    InstanceStates, Instances, Listener, ListenerDescription, ListenerDescriptions,
    LoadBalancerAttributes, LoadBalancerDescription, LoadBalancerDescriptions, PolicyDescription,
    PolicyDescriptions, RegisterEndPointsOutput, RemoveAvailabilityZonesOutput, SecurityGroups,
    Subnets, Tag, TagDescription, TagDescriptions, TagList,
};
use crate::state::{ElbError, ElbState};
use crate::types::{ElbAttributes, ElbHealthCheck, ElbListener};
use crate::views::ElbStateView;
use crate::wire;

/// Classic ELB service handler.
pub struct ElasticLoadBalancingService {
    pub(crate) state: Arc<BackendState<ElbState>>,
    pub(crate) notifier: StateChangeNotifier<ElbStateView>,
}

impl ElasticLoadBalancingService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ElasticLoadBalancingService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ElasticLoadBalancingService {
    fn service_name(&self) -> &str {
        "elasticloadbalancing"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://elasticloadbalancing\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

const MUTATING_ACTIONS: &[&str] = &[
    "CreateLoadBalancer",
    "DeleteLoadBalancer",
    "RegisterInstancesWithLoadBalancer",
    "DeregisterInstancesFromLoadBalancer",
    "ConfigureHealthCheck",
    "CreateLoadBalancerListeners",
    "DeleteLoadBalancerListeners",
    "AttachLoadBalancerToSubnets",
    "DetachLoadBalancerFromSubnets",
    "ApplySecurityGroupsToLoadBalancer",
    "AddTags",
    "RemoveTags",
    "EnableAvailabilityZonesForLoadBalancer",
    "DisableAvailabilityZonesForLoadBalancer",
    "SetLoadBalancerPoliciesOfListener",
    "CreateLoadBalancerPolicy",
    "DeleteLoadBalancerPolicy",
    "ModifyLoadBalancerAttributes",
    "SetLoadBalancerListenerSSLCertificate",
    "SetLoadBalancerPoliciesForBackendServer",
    "CreateAppCookieStickinessPolicy",
    "CreateLBCookieStickinessPolicy",
];

impl ElasticLoadBalancingService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        let body_str = std::str::from_utf8(&request.body).unwrap_or("");
        let params = parse_query_string(body_str);

        let action = match params.get("Action") {
            Some(a) => a.clone(),
            None => {
                return MockResponse::error(400, "MissingAction", "Missing 'Action' parameter");
            }
        };

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreateLoadBalancer" => {
                self.handle_create_load_balancer(&state, &params, &region)
                    .await
            }
            "DeleteLoadBalancer" => self.handle_delete_load_balancer(&state, &params).await,
            "DescribeLoadBalancers" => self.handle_describe_load_balancers(&state, &params).await,
            "RegisterInstancesWithLoadBalancer" => {
                self.handle_register_instances(&state, &params).await
            }
            "DeregisterInstancesFromLoadBalancer" => {
                self.handle_deregister_instances(&state, &params).await
            }
            "DescribeInstanceHealth" => self.handle_describe_instance_health(&state, &params).await,
            "ConfigureHealthCheck" => self.handle_configure_health_check(&state, &params).await,
            "CreateLoadBalancerListeners" => {
                self.handle_create_load_balancer_listeners(&state, &params)
                    .await
            }
            "DeleteLoadBalancerListeners" => {
                self.handle_delete_load_balancer_listeners(&state, &params)
                    .await
            }
            "DescribeLoadBalancerPolicies" => {
                self.handle_describe_load_balancer_policies(&state, &params)
                    .await
            }
            "AttachLoadBalancerToSubnets" => {
                self.handle_attach_load_balancer_to_subnets(&state, &params)
                    .await
            }
            "DetachLoadBalancerFromSubnets" => {
                self.handle_detach_load_balancer_from_subnets(&state, &params)
                    .await
            }
            "ApplySecurityGroupsToLoadBalancer" => {
                self.handle_apply_security_groups(&state, &params).await
            }
            "AddTags" => self.handle_add_tags(&state, &params).await,
            "DescribeTags" => self.handle_describe_tags(&state, &params).await,
            "RemoveTags" => self.handle_remove_tags(&state, &params).await,
            "EnableAvailabilityZonesForLoadBalancer" => {
                self.handle_enable_availability_zones(&state, &params).await
            }
            "DisableAvailabilityZonesForLoadBalancer" => {
                self.handle_disable_availability_zones(&state, &params)
                    .await
            }
            "SetLoadBalancerPoliciesOfListener" => {
                self.handle_set_load_balancer_policies_of_listener(&state, &params)
                    .await
            }
            "CreateLoadBalancerPolicy" => {
                self.handle_create_load_balancer_policy(&state, &params)
                    .await
            }
            "DeleteLoadBalancerPolicy" => {
                self.handle_delete_load_balancer_policy(&state, &params)
                    .await
            }
            "ModifyLoadBalancerAttributes" => {
                self.handle_modify_load_balancer_attributes(&state, &params)
                    .await
            }
            "DescribeLoadBalancerAttributes" => {
                self.handle_describe_load_balancer_attributes(&state, &params)
                    .await
            }
            "DescribeAccountLimits" => self.handle_describe_account_limits(&state).await,
            "SetLoadBalancerListenerSSLCertificate" => {
                self.handle_set_load_balancer_listener_ssl_certificate(&state, &params)
                    .await
            }
            "SetLoadBalancerPoliciesForBackendServer" => {
                self.handle_set_load_balancer_policies_for_backend_server(&state, &params)
                    .await
            }
            "CreateAppCookieStickinessPolicy" => {
                self.handle_create_app_cookie_stickiness_policy(&state, &params)
                    .await
            }
            "CreateLBCookieStickinessPolicy" => {
                self.handle_create_lb_cookie_stickiness_policy(&state, &params)
                    .await
            }
            "DescribeLoadBalancerPolicyTypes" => {
                self.handle_describe_load_balancer_policy_types().await
            }
            _ => MockResponse::error(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for Elastic Load Balancing"),
            ),
        };

        if MUTATING_ACTIONS.contains(&action.as_str()) && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // --- CreateLoadBalancer ---

    async fn handle_create_load_balancer(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_load_balancer_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "LoadBalancerName is required.".to_string(),
            ));
        }
        let name = input.load_balancer_name;

        let scheme = input
            .scheme
            .unwrap_or_else(|| "internet-facing".to_string());

        let availability_zones = input
            .availability_zones
            .map(|l| l.items)
            .unwrap_or_default();
        let subnets = input.subnets.map(|l| l.items).unwrap_or_default();
        let security_groups = input.security_groups.map(|l| l.items).unwrap_or_default();
        let listeners: Vec<ElbListener> = input
            .listeners
            .items
            .iter()
            .map(|l| ElbListener {
                load_balancer_port: l.load_balancer_port,
                instance_port: l.instance_port,
                protocol: l.protocol.clone(),
                instance_protocol: l
                    .instance_protocol
                    .clone()
                    .unwrap_or_else(|| l.protocol.clone()),
                ssl_certificate_id: l.s_s_l_certificate_id.clone(),
                policy_names: Vec::new(),
            })
            .collect();

        let mut state_guard = state.write().await;
        match state_guard.create_load_balancer(
            &name,
            &scheme,
            availability_zones,
            subnets,
            security_groups,
            listeners,
            region,
        ) {
            Ok(lb) => {
                let dns_name = lb.dns_name.clone();
                wire::serialize_create_load_balancer_response(
                    &crate::model::CreateAccessPointOutput {
                        d_n_s_name: Some(dns_name),
                    },
                )
            }
            Err(e) => elb_error_response(&e),
        }
    }

    // --- DeleteLoadBalancer ---

    async fn handle_delete_load_balancer(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_load_balancer_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "LoadBalancerName is required.".to_string(),
            ));
        }
        let name = input.load_balancer_name;
        let mut state_guard = state.write().await;
        match state_guard.delete_load_balancer(&name) {
            Ok(()) => wire::serialize_delete_load_balancer_response(
                &crate::model::DeleteAccessPointOutput {},
            ),
            Err(e) => elb_error_response(&e),
        }
    }

    // --- DescribeLoadBalancers ---

    async fn handle_describe_load_balancers(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_load_balancers_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let names = input
            .load_balancer_names
            .map(|l| l.items)
            .unwrap_or_default();
        let state_guard = state.read().await;
        match state_guard.describe_load_balancers(&names) {
            Ok(lbs) => {
                let items: Vec<LoadBalancerDescription> =
                    lbs.iter().map(|lb| lb_to_description(lb)).collect();
                wire::serialize_describe_load_balancers_response(&DescribeAccessPointsOutput {
                    load_balancer_descriptions: Some(LoadBalancerDescriptions { items }),
                    next_marker: None,
                })
            }
            Err(e) => elb_error_response(&e),
        }
    }

    // --- RegisterInstancesWithLoadBalancer ---

    async fn handle_register_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_register_instances_with_load_balancer_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "LoadBalancerName is required.".to_string(),
            ));
        }
        let name = input.load_balancer_name;
        let instance_ids: Vec<String> = input
            .instances
            .items
            .iter()
            .filter_map(|inst| inst.instance_id.clone())
            .collect();
        let mut state_guard = state.write().await;
        match state_guard.register_instances(&name, &instance_ids) {
            Ok(instances) => wire::serialize_register_instances_with_load_balancer_response(
                &RegisterEndPointsOutput {
                    instances: Some(Instances {
                        items: instances
                            .iter()
                            .map(|id| Instance {
                                instance_id: Some(id.clone()),
                            })
                            .collect(),
                    }),
                },
            ),
            Err(e) => elb_error_response(&e),
        }
    }

    // --- DeregisterInstancesFromLoadBalancer ---

    async fn handle_deregister_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_deregister_instances_from_load_balancer_request(params)
        {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "LoadBalancerName is required.".to_string(),
            ));
        }
        let name = input.load_balancer_name;
        let instance_ids: Vec<String> = input
            .instances
            .items
            .iter()
            .filter_map(|inst| inst.instance_id.clone())
            .collect();
        let mut state_guard = state.write().await;
        match state_guard.deregister_instances(&name, &instance_ids) {
            Ok(instances) => wire::serialize_deregister_instances_from_load_balancer_response(
                &DeregisterEndPointsOutput {
                    instances: Some(Instances {
                        items: instances
                            .iter()
                            .map(|id| Instance {
                                instance_id: Some(id.clone()),
                            })
                            .collect(),
                    }),
                },
            ),
            Err(e) => elb_error_response(&e),
        }
    }

    // --- DescribeInstanceHealth ---

    async fn handle_describe_instance_health(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_instance_health_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "LoadBalancerName is required.".to_string(),
            ));
        }
        let name = input.load_balancer_name;
        let instance_ids: Vec<String> = input
            .instances
            .map(|l| l.items)
            .unwrap_or_default()
            .iter()
            .filter_map(|inst| inst.instance_id.clone())
            .collect();
        let state_guard = state.read().await;
        match state_guard.describe_instance_health(&name, &instance_ids) {
            Ok(instances) => {
                wire::serialize_describe_instance_health_response(&DescribeEndPointStateOutput {
                    instance_states: Some(InstanceStates {
                        items: instances
                            .iter()
                            .map(|id| InstanceState {
                                instance_id: Some(id.clone()),
                                state: Some("InService".to_string()),
                                reason_code: Some("N/A".to_string()),
                                description: Some("N/A".to_string()),
                            })
                            .collect(),
                    }),
                })
            }
            Err(e) => elb_error_response(&e),
        }
    }

    // --- ConfigureHealthCheck ---

    async fn handle_configure_health_check(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_configure_health_check_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "LoadBalancerName is required.".to_string(),
            ));
        }
        let name = input.load_balancer_name;
        let wire_hc = input.health_check;
        let hc = ElbHealthCheck {
            target: if wire_hc.target.is_empty() {
                "TCP:80".to_string()
            } else {
                wire_hc.target
            },
            interval: wire_hc.interval,
            timeout: wire_hc.timeout,
            unhealthy_threshold: wire_hc.unhealthy_threshold,
            healthy_threshold: wire_hc.healthy_threshold,
        };
        let mut state_guard = state.write().await;
        match state_guard.configure_health_check(&name, hc) {
            Ok(hc) => {
                wire::serialize_configure_health_check_response(&ConfigureHealthCheckOutput {
                    health_check: Some(HealthCheck {
                        target: hc.target,
                        interval: hc.interval,
                        timeout: hc.timeout,
                        unhealthy_threshold: hc.unhealthy_threshold,
                        healthy_threshold: hc.healthy_threshold,
                    }),
                })
            }
            Err(e) => elb_error_response(&e),
        }
    }

    // --- CreateLoadBalancerListeners ---

    async fn handle_create_load_balancer_listeners(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_load_balancer_listeners_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "LoadBalancerName is required.".to_string(),
            ));
        }
        let name = input.load_balancer_name;
        let listeners: Vec<ElbListener> = input
            .listeners
            .items
            .iter()
            .map(|l| ElbListener {
                load_balancer_port: l.load_balancer_port,
                instance_port: l.instance_port,
                protocol: l.protocol.clone(),
                instance_protocol: l
                    .instance_protocol
                    .clone()
                    .unwrap_or_else(|| l.protocol.clone()),
                ssl_certificate_id: l.s_s_l_certificate_id.clone(),
                policy_names: Vec::new(),
            })
            .collect();
        let mut state_guard = state.write().await;
        match state_guard.create_load_balancer_listeners(&name, listeners) {
            Ok(()) => wire::serialize_create_load_balancer_listeners_response(
                &crate::model::CreateLoadBalancerListenerOutput {},
            ),
            Err(e) => elb_error_response(&e),
        }
    }

    // --- DeleteLoadBalancerListeners ---

    async fn handle_delete_load_balancer_listeners(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_load_balancer_listeners_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "LoadBalancerName is required.".to_string(),
            ));
        }
        let name = input.load_balancer_name;
        let ports = input.load_balancer_ports.items;
        let mut state_guard = state.write().await;
        match state_guard.delete_load_balancer_listeners(&name, &ports) {
            Ok(()) => wire::serialize_delete_load_balancer_listeners_response(
                &crate::model::DeleteLoadBalancerListenerOutput {},
            ),
            Err(e) => elb_error_response(&e),
        }
    }

    // --- DescribeLoadBalancerPolicies ---

    async fn handle_describe_load_balancer_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_load_balancer_policies_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let lb_name = input.load_balancer_name.as_deref();
        let policy_names = input.policy_names.map(|l| l.items).unwrap_or_default();
        let state_guard = state.read().await;
        match state_guard.describe_load_balancer_policies(lb_name, &policy_names) {
            Ok(policies) => {
                let items: Vec<PolicyDescription> = policies
                    .iter()
                    .map(|p| PolicyDescription {
                        policy_name: Some(p.policy_name.clone()),
                        policy_type_name: Some(p.policy_type_name.clone()),
                        policy_attribute_descriptions: None,
                    })
                    .collect();
                wire::serialize_describe_load_balancer_policies_response(
                    &DescribeLoadBalancerPoliciesOutput {
                        policy_descriptions: Some(PolicyDescriptions { items }),
                    },
                )
            }
            Err(e) => elb_error_response(&e),
        }
    }

    // --- AttachLoadBalancerToSubnets ---

    async fn handle_attach_load_balancer_to_subnets(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_attach_load_balancer_to_subnets_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "LoadBalancerName is required.".to_string(),
            ));
        }
        let name = input.load_balancer_name;
        let subnets = input.subnets.items;
        let mut state_guard = state.write().await;
        match state_guard.attach_load_balancer_to_subnets(&name, &subnets) {
            Ok(current_subnets) => wire::serialize_attach_load_balancer_to_subnets_response(
                &AttachLoadBalancerToSubnetsOutput {
                    subnets: Some(Subnets {
                        items: current_subnets,
                    }),
                },
            ),
            Err(e) => elb_error_response(&e),
        }
    }

    // --- DetachLoadBalancerFromSubnets ---

    async fn handle_detach_load_balancer_from_subnets(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_detach_load_balancer_from_subnets_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "LoadBalancerName is required.".to_string(),
            ));
        }
        let name = input.load_balancer_name;
        let subnets = input.subnets.items;
        let mut state_guard = state.write().await;
        match state_guard.detach_load_balancer_from_subnets(&name, &subnets) {
            Ok(current_subnets) => wire::serialize_detach_load_balancer_from_subnets_response(
                &crate::model::DetachLoadBalancerFromSubnetsOutput {
                    subnets: Some(Subnets {
                        items: current_subnets,
                    }),
                },
            ),
            Err(e) => elb_error_response(&e),
        }
    }

    // --- ApplySecurityGroupsToLoadBalancer ---

    async fn handle_apply_security_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_apply_security_groups_to_load_balancer_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "LoadBalancerName is required.".to_string(),
            ));
        }
        let name = input.load_balancer_name;
        let sgs = input.security_groups.items;
        let mut state_guard = state.write().await;
        match state_guard.apply_security_groups(&name, sgs) {
            Ok(current_sgs) => wire::serialize_apply_security_groups_to_load_balancer_response(
                &ApplySecurityGroupsToLoadBalancerOutput {
                    security_groups: Some(SecurityGroups { items: current_sgs }),
                },
            ),
            Err(e) => elb_error_response(&e),
        }
    }

    // --- AddTags ---

    async fn handle_add_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_add_tags_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let lb_names = input.load_balancer_names.items;
        let tags: Vec<(String, String)> = input
            .tags
            .items
            .iter()
            .map(|t| (t.key.clone(), t.value.clone().unwrap_or_default()))
            .collect();
        let mut state_guard = state.write().await;
        match state_guard.add_tags(&lb_names, tags) {
            Ok(()) => wire::serialize_add_tags_response(&crate::model::AddTagsOutput {}),
            Err(e) => elb_error_response(&e),
        }
    }

    // --- DescribeTags ---

    async fn handle_describe_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_tags_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let lb_names = input.load_balancer_names.items;
        let state_guard = state.read().await;
        match state_guard.describe_tags(&lb_names) {
            Ok(tag_list) => {
                let items: Vec<TagDescription> = tag_list
                    .iter()
                    .map(|(name, tags)| TagDescription {
                        load_balancer_name: Some(name.to_string()),
                        tags: Some(TagList {
                            items: tags
                                .iter()
                                .map(|(k, v)| Tag {
                                    key: k.clone(),
                                    value: Some(v.clone()),
                                })
                                .collect(),
                        }),
                    })
                    .collect();
                wire::serialize_describe_tags_response(&DescribeTagsOutput {
                    tag_descriptions: Some(TagDescriptions { items }),
                })
            }
            Err(e) => elb_error_response(&e),
        }
    }

    // --- RemoveTags ---

    async fn handle_remove_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_remove_tags_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let lb_names = input.load_balancer_names.items;
        let tag_keys: Vec<String> = input
            .tags
            .items
            .iter()
            .filter_map(|t| t.key.clone())
            .collect();
        let mut state_guard = state.write().await;
        match state_guard.remove_tags(&lb_names, &tag_keys) {
            Ok(()) => wire::serialize_remove_tags_response(&crate::model::RemoveTagsOutput {}),
            Err(e) => elb_error_response(&e),
        }
    }

    // --- EnableAvailabilityZonesForLoadBalancer ---

    async fn handle_enable_availability_zones(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_enable_availability_zones_for_load_balancer_request(params) {
                Ok(v) => v,
                Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
            };
        if input.load_balancer_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "LoadBalancerName is required.".to_string(),
            ));
        }
        let name = input.load_balancer_name;
        let zones = input.availability_zones.items;
        let mut state_guard = state.write().await;
        match state_guard.enable_availability_zones(&name, &zones) {
            Ok(current_zones) => {
                wire::serialize_enable_availability_zones_for_load_balancer_response(
                    &AddAvailabilityZonesOutput {
                        availability_zones: Some(AvailabilityZones {
                            items: current_zones,
                        }),
                    },
                )
            }
            Err(e) => elb_error_response(&e),
        }
    }

    // --- DisableAvailabilityZonesForLoadBalancer ---

    async fn handle_disable_availability_zones(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_disable_availability_zones_for_load_balancer_request(params) {
                Ok(v) => v,
                Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
            };
        if input.load_balancer_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "LoadBalancerName is required.".to_string(),
            ));
        }
        let name = input.load_balancer_name;
        let zones = input.availability_zones.items;
        let mut state_guard = state.write().await;
        match state_guard.disable_availability_zones(&name, &zones) {
            Ok(current_zones) => {
                wire::serialize_disable_availability_zones_for_load_balancer_response(
                    &RemoveAvailabilityZonesOutput {
                        availability_zones: Some(AvailabilityZones {
                            items: current_zones,
                        }),
                    },
                )
            }
            Err(e) => elb_error_response(&e),
        }
    }

    // --- SetLoadBalancerPoliciesOfListener ---

    async fn handle_set_load_balancer_policies_of_listener(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_set_load_balancer_policies_of_listener_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "LoadBalancerName is required.".to_string(),
            ));
        }
        let name = input.load_balancer_name;
        let port = input.load_balancer_port;
        let policy_names = input.policy_names.items;
        let mut state_guard = state.write().await;
        match state_guard.set_load_balancer_policies_of_listener(&name, port, policy_names) {
            Ok(()) => wire::serialize_set_load_balancer_policies_of_listener_response(
                &crate::model::SetLoadBalancerPoliciesOfListenerOutput {},
            ),
            Err(e) => elb_error_response(&e),
        }
    }

    // --- CreateLoadBalancerPolicy ---

    async fn handle_create_load_balancer_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_load_balancer_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "LoadBalancerName is required.".to_string(),
            ));
        }
        if input.policy_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "PolicyName is required.".to_string(),
            ));
        }
        if input.policy_type_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "PolicyTypeName is required.".to_string(),
            ));
        }
        let name = input.load_balancer_name;
        let policy_name = input.policy_name;
        let policy_type_name = input.policy_type_name;
        let attributes: Vec<(String, String)> = input
            .policy_attributes
            .map(|l| l.items)
            .unwrap_or_default()
            .iter()
            .map(|a| {
                (
                    a.attribute_name.clone().unwrap_or_default(),
                    a.attribute_value.clone().unwrap_or_default(),
                )
            })
            .collect();
        let mut state_guard = state.write().await;
        match state_guard.create_load_balancer_policy(
            &name,
            &policy_name,
            &policy_type_name,
            attributes,
        ) {
            Ok(()) => wire::serialize_create_load_balancer_policy_response(
                &crate::model::CreateLoadBalancerPolicyOutput {},
            ),
            Err(e) => elb_error_response(&e),
        }
    }

    // --- DeleteLoadBalancerPolicy ---

    async fn handle_delete_load_balancer_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_load_balancer_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "LoadBalancerName is required.".to_string(),
            ));
        }
        if input.policy_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "PolicyName is required.".to_string(),
            ));
        }
        let name = input.load_balancer_name;
        let policy_name = input.policy_name;
        let mut state_guard = state.write().await;
        if let Some(lb) = state_guard.load_balancers.get_mut(&name) {
            lb.policies.retain(|p| p.policy_name != policy_name);
        }
        wire::serialize_delete_load_balancer_policy_response(
            &crate::model::DeleteLoadBalancerPolicyOutput {},
        )
    }

    // --- ModifyLoadBalancerAttributes ---

    async fn handle_modify_load_balancer_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_modify_load_balancer_attributes_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "LoadBalancerName is required.".to_string(),
            ));
        }
        let name = input.load_balancer_name;
        let lb_attrs = input.load_balancer_attributes;

        let cross_zone = lb_attrs
            .cross_zone_load_balancing
            .as_ref()
            .map(|c| c.enabled)
            .unwrap_or(false);
        let conn_draining = lb_attrs
            .connection_draining
            .as_ref()
            .map(|c| c.enabled)
            .unwrap_or(false);
        let conn_draining_timeout = lb_attrs
            .connection_draining
            .as_ref()
            .and_then(|c| c.timeout);
        let idle_timeout = lb_attrs
            .connection_settings
            .as_ref()
            .map(|c| c.idle_timeout)
            .filter(|t| *t > 0)
            .unwrap_or(60);
        let access_log_enabled = lb_attrs
            .access_log
            .as_ref()
            .map(|a| a.enabled)
            .unwrap_or(false);
        let access_log_emit_interval = lb_attrs.access_log.as_ref().and_then(|a| a.emit_interval);
        let access_log_s3_bucket_name = lb_attrs
            .access_log
            .as_ref()
            .and_then(|a| a.s3_bucket_name.clone());
        let access_log_s3_bucket_prefix = lb_attrs
            .access_log
            .as_ref()
            .and_then(|a| a.s3_bucket_prefix.clone());

        let attrs = ElbAttributes {
            cross_zone_load_balancing_enabled: cross_zone,
            access_log_enabled,
            access_log_emit_interval,
            access_log_s3_bucket_name,
            access_log_s3_bucket_prefix,
            connection_draining_enabled: conn_draining,
            connection_draining_timeout: conn_draining_timeout,
            connection_idle_timeout: idle_timeout,
        };

        let mut state_guard = state.write().await;
        match state_guard.modify_load_balancer_attributes(&name, attrs) {
            Ok(attrs) => wire::serialize_modify_load_balancer_attributes_response(
                &crate::model::ModifyLoadBalancerAttributesOutput {
                    load_balancer_name: Some(name),
                    load_balancer_attributes: Some(build_lb_attributes(&attrs)),
                },
            ),
            Err(e) => elb_error_response(&e),
        }
    }

    // --- DescribeLoadBalancerAttributes ---

    async fn handle_describe_load_balancer_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_load_balancer_attributes_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "LoadBalancerName is required.".to_string(),
            ));
        }
        let name = input.load_balancer_name;
        let state_guard = state.read().await;
        match state_guard.describe_load_balancer_attributes(&name) {
            Ok(attrs) => wire::serialize_describe_load_balancer_attributes_response(
                &DescribeLoadBalancerAttributesOutput {
                    load_balancer_attributes: Some(build_lb_attributes(attrs)),
                },
            ),
            Err(e) => elb_error_response(&e),
        }
    }

    // --- DescribeAccountLimits ---

    async fn handle_describe_account_limits(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let lb_count = state.load_balancers.len() as i64;
        let limits = vec![
            crate::model::Limit {
                name: Some("classic-load-balancers".to_string()),
                max: Some("20".to_string()),
            },
            crate::model::Limit {
                name: Some("classic-listeners".to_string()),
                max: Some("100".to_string()),
            },
            crate::model::Limit {
                name: Some("classic-registered-instances".to_string()),
                max: Some("1000".to_string()),
            },
        ];
        let _ = lb_count;
        wire::serialize_describe_account_limits_response(
            &crate::model::DescribeAccountLimitsOutput {
                limits: Some(crate::model::Limits { items: limits }),
                next_marker: None,
            },
        )
    }

    // --- SetLoadBalancerListenerSSLCertificate ---

    async fn handle_set_load_balancer_listener_ssl_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_set_load_balancer_listener_s_s_l_certificate_request(params) {
                Ok(v) => v,
                Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
            };
        if input.load_balancer_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "LoadBalancerName is required.".to_string(),
            ));
        }
        let name = input.load_balancer_name;
        let port = input.load_balancer_port;
        let cert_id = input.s_s_l_certificate_id;
        let mut state_guard = state.write().await;
        if let Some(lb) = state_guard.load_balancers.get_mut(&name) {
            for listener in lb.listeners.iter_mut() {
                if listener.load_balancer_port == port {
                    listener.ssl_certificate_id = Some(cert_id);
                    break;
                }
            }
        }
        wire::serialize_set_load_balancer_listener_s_s_l_certificate_response(
            &crate::model::SetLoadBalancerListenerSSLCertificateOutput {},
        )
    }

    // --- SetLoadBalancerPoliciesForBackendServer ---

    async fn handle_set_load_balancer_policies_for_backend_server(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_set_load_balancer_policies_for_backend_server_request(params) {
                Ok(v) => v,
                Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
            };
        let name = input.load_balancer_name;
        let _instance_port = input.instance_port;
        let _policy_names = input.policy_names.items;
        let state_guard = state.read().await;
        if !state_guard.load_balancers.contains_key(&name) {
            return elb_error_response(&ElbError::LoadBalancerNotFound(name));
        }
        wire::serialize_set_load_balancer_policies_for_backend_server_response(
            &crate::model::SetLoadBalancerPoliciesForBackendServerOutput {},
        )
    }

    // --- CreateAppCookieStickinessPolicy ---

    async fn handle_create_app_cookie_stickiness_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_app_cookie_stickiness_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "LoadBalancerName is required.".to_string(),
            ));
        }
        let name = input.load_balancer_name;
        let policy_name = input.policy_name;
        let cookie_name = input.cookie_name;
        let mut state_guard = state.write().await;
        match state_guard.create_load_balancer_policy(
            &name,
            &policy_name,
            "AppCookieStickinessPolicyType",
            vec![("CookieName".to_string(), cookie_name)],
        ) {
            Ok(()) => wire::serialize_create_app_cookie_stickiness_policy_response(
                &crate::model::CreateAppCookieStickinessPolicyOutput {},
            ),
            Err(e) => elb_error_response(&e),
        }
    }

    // --- CreateLBCookieStickinessPolicy ---

    async fn handle_create_lb_cookie_stickiness_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<ElbState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_l_b_cookie_stickiness_policy_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.load_balancer_name.is_empty() {
            return elb_error_response(&ElbError::ValidationError(
                "LoadBalancerName is required.".to_string(),
            ));
        }
        let name = input.load_balancer_name;
        let policy_name = input.policy_name;
        let expiry = input
            .cookie_expiration_period
            .map(|v| v.to_string())
            .unwrap_or_default();
        let mut state_guard = state.write().await;
        match state_guard.create_load_balancer_policy(
            &name,
            &policy_name,
            "LBCookieStickinessPolicyType",
            vec![("CookieExpirationPeriod".to_string(), expiry)],
        ) {
            Ok(()) => wire::serialize_create_l_b_cookie_stickiness_policy_response(
                &crate::model::CreateLBCookieStickinessPolicyOutput {},
            ),
            Err(e) => elb_error_response(&e),
        }
    }

    // --- DescribeLoadBalancerPolicyTypes ---

    async fn handle_describe_load_balancer_policy_types(&self) -> MockResponse {
        let policy_types = vec![
            crate::model::PolicyTypeDescription {
                policy_type_name: Some("AppCookieStickinessPolicyType".to_string()),
                description: Some("Stickiness policy with app-controlled cookie.".to_string()),
                policy_attribute_type_descriptions: None,
            },
            crate::model::PolicyTypeDescription {
                policy_type_name: Some("LBCookieStickinessPolicyType".to_string()),
                description: Some("Stickiness policy with ELB-controlled cookie.".to_string()),
                policy_attribute_type_descriptions: None,
            },
            crate::model::PolicyTypeDescription {
                policy_type_name: Some("SSLNegotiationPolicyType".to_string()),
                description: Some("SSL negotiation policy.".to_string()),
                policy_attribute_type_descriptions: None,
            },
        ];
        wire::serialize_describe_load_balancer_policy_types_response(
            &crate::model::DescribeLoadBalancerPolicyTypesOutput {
                policy_type_descriptions: Some(crate::model::PolicyTypeDescriptions {
                    items: policy_types,
                }),
            },
        )
    }
}

// --- Helpers ---

fn lb_to_description(lb: &crate::types::ElbLoadBalancer) -> LoadBalancerDescription {
    LoadBalancerDescription {
        load_balancer_name: Some(lb.name.clone()),
        d_n_s_name: Some(lb.dns_name.clone()),
        scheme: Some(lb.scheme.clone()),
        availability_zones: Some(AvailabilityZones {
            items: lb.availability_zones.clone(),
        }),
        subnets: Some(Subnets {
            items: lb.subnets.clone(),
        }),
        security_groups: Some(SecurityGroups {
            items: lb.security_groups.clone(),
        }),
        instances: Some(Instances {
            items: lb
                .instances
                .iter()
                .map(|id| Instance {
                    instance_id: Some(id.clone()),
                })
                .collect(),
        }),
        listener_descriptions: Some(ListenerDescriptions {
            items: lb
                .listeners
                .iter()
                .map(|l| ListenerDescription {
                    listener: Some(Listener {
                        load_balancer_port: l.load_balancer_port,
                        instance_port: l.instance_port,
                        protocol: l.protocol.clone(),
                        instance_protocol: Some(l.instance_protocol.clone()),
                        s_s_l_certificate_id: l.ssl_certificate_id.clone(),
                    }),
                    policy_names: if l.policy_names.is_empty() {
                        None
                    } else {
                        Some(crate::model::PolicyNames {
                            items: l.policy_names.clone(),
                        })
                    },
                })
                .collect(),
        }),
        health_check: Some(HealthCheck {
            target: lb.health_check.target.clone(),
            interval: lb.health_check.interval,
            timeout: lb.health_check.timeout,
            unhealthy_threshold: lb.health_check.unhealthy_threshold,
            healthy_threshold: lb.health_check.healthy_threshold,
        }),
        v_p_c_id: lb.vpc_id.clone(),
        canonical_hosted_zone_name: None,
        canonical_hosted_zone_name_i_d: None,
        created_time: None,
        policies: None,
        backend_server_descriptions: None,
        source_security_group: None,
    }
}

fn build_lb_attributes(attrs: &ElbAttributes) -> LoadBalancerAttributes {
    LoadBalancerAttributes {
        cross_zone_load_balancing: Some(CrossZoneLoadBalancing {
            enabled: attrs.cross_zone_load_balancing_enabled,
        }),
        access_log: Some(AccessLog {
            enabled: attrs.access_log_enabled,
            emit_interval: attrs.access_log_emit_interval,
            s3_bucket_name: attrs.access_log_s3_bucket_name.clone(),
            s3_bucket_prefix: attrs.access_log_s3_bucket_prefix.clone(),
        }),
        connection_draining: Some(ConnectionDraining {
            enabled: attrs.connection_draining_enabled,
            timeout: attrs.connection_draining_timeout,
        }),
        connection_settings: Some(ConnectionSettings {
            idle_timeout: attrs.connection_idle_timeout,
        }),
        additional_attributes: None,
    }
}

fn elb_error_response(err: &ElbError) -> MockResponse {
    let (status, error_type) = match err {
        ElbError::DuplicateLoadBalancerName(_) => (400u16, "DuplicateLoadBalancerName"),
        ElbError::LoadBalancerNotFound(_) => (400, "LoadBalancerNotFound"),
        ElbError::ListenerNotFound { .. } => (400, "ListenerNotFound"),
        ElbError::ValidationError(_) => (400, "ValidationError"),
    };
    let body = format!(
        r#"<ErrorResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
  <Error>
    <Type>Sender</Type>
    <Code>{code}</Code>
    <Message>{message}</Message>
  </Error>
  <RequestId>{request_id}</RequestId>
</ErrorResponse>"#,
        code = xml_escape(error_type),
        message = xml_escape(&err.to_string()),
        request_id = uuid::Uuid::new_v4(),
    );
    MockResponse::xml(status, body)
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
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
