//! Serde-compatible view types for ELB Classic state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ElasticLoadBalancingService;
use crate::state::ElbState;
use crate::types::{ElbAttributes, ElbHealthCheck, ElbListener, ElbLoadBalancer, ElbPolicy};

/// Serializable view of the entire ELB Classic state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ElbStateView {
    /// Load balancers keyed by name.
    #[serde(default)]
    pub load_balancers: HashMap<String, ElbLoadBalancerView>,
}

/// Serializable view of a Classic ELB load balancer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElbLoadBalancerView {
    pub name: String,
    pub dns_name: String,
    pub scheme: String,
    pub availability_zones: Vec<String>,
    pub subnets: Vec<String>,
    pub security_groups: Vec<String>,
    pub instances: Vec<String>,
    pub listeners: Vec<ElbListenerView>,
    pub health_check: ElbHealthCheckView,
    pub tags: HashMap<String, String>,
    pub policies: Vec<ElbPolicyView>,
    pub attributes: ElbAttributesView,
    pub vpc_id: Option<String>,
}

/// Serializable view of a Classic ELB listener.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElbListenerView {
    pub load_balancer_port: i32,
    pub instance_port: i32,
    pub protocol: String,
    pub instance_protocol: String,
    pub ssl_certificate_id: Option<String>,
    pub policy_names: Vec<String>,
}

/// Serializable view of a Classic ELB health check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElbHealthCheckView {
    pub target: String,
    pub interval: i32,
    pub timeout: i32,
    pub unhealthy_threshold: i32,
    pub healthy_threshold: i32,
}

/// Serializable view of a Classic ELB policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElbPolicyView {
    pub policy_name: String,
    pub policy_type_name: String,
    pub attributes: Vec<(String, String)>,
}

/// Serializable view of Classic ELB attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElbAttributesView {
    pub cross_zone_load_balancing_enabled: bool,
    pub access_log_enabled: bool,
    pub access_log_emit_interval: Option<i32>,
    pub access_log_s3_bucket_name: Option<String>,
    pub access_log_s3_bucket_prefix: Option<String>,
    pub connection_draining_enabled: bool,
    pub connection_draining_timeout: Option<i32>,
    pub connection_idle_timeout: i32,
}

// --- From internal types to view types ---

impl From<&ElbState> for ElbStateView {
    fn from(state: &ElbState) -> Self {
        ElbStateView {
            load_balancers: state
                .load_balancers
                .iter()
                .map(|(k, v)| (k.clone(), ElbLoadBalancerView::from(v)))
                .collect(),
        }
    }
}

impl From<&ElbLoadBalancer> for ElbLoadBalancerView {
    fn from(lb: &ElbLoadBalancer) -> Self {
        ElbLoadBalancerView {
            name: lb.name.clone(),
            dns_name: lb.dns_name.clone(),
            scheme: lb.scheme.clone(),
            availability_zones: lb.availability_zones.clone(),
            subnets: lb.subnets.clone(),
            security_groups: lb.security_groups.clone(),
            instances: lb.instances.clone(),
            listeners: lb.listeners.iter().map(ElbListenerView::from).collect(),
            health_check: ElbHealthCheckView::from(&lb.health_check),
            tags: lb.tags.clone(),
            policies: lb.policies.iter().map(ElbPolicyView::from).collect(),
            attributes: ElbAttributesView::from(&lb.attributes),
            vpc_id: lb.vpc_id.clone(),
        }
    }
}

impl From<&ElbListener> for ElbListenerView {
    fn from(l: &ElbListener) -> Self {
        ElbListenerView {
            load_balancer_port: l.load_balancer_port,
            instance_port: l.instance_port,
            protocol: l.protocol.clone(),
            instance_protocol: l.instance_protocol.clone(),
            ssl_certificate_id: l.ssl_certificate_id.clone(),
            policy_names: l.policy_names.clone(),
        }
    }
}

impl From<&ElbHealthCheck> for ElbHealthCheckView {
    fn from(hc: &ElbHealthCheck) -> Self {
        ElbHealthCheckView {
            target: hc.target.clone(),
            interval: hc.interval,
            timeout: hc.timeout,
            unhealthy_threshold: hc.unhealthy_threshold,
            healthy_threshold: hc.healthy_threshold,
        }
    }
}

impl From<&ElbPolicy> for ElbPolicyView {
    fn from(p: &ElbPolicy) -> Self {
        ElbPolicyView {
            policy_name: p.policy_name.clone(),
            policy_type_name: p.policy_type_name.clone(),
            attributes: p.attributes.clone(),
        }
    }
}

impl From<&ElbAttributes> for ElbAttributesView {
    fn from(a: &ElbAttributes) -> Self {
        ElbAttributesView {
            cross_zone_load_balancing_enabled: a.cross_zone_load_balancing_enabled,
            access_log_enabled: a.access_log_enabled,
            access_log_emit_interval: a.access_log_emit_interval,
            access_log_s3_bucket_name: a.access_log_s3_bucket_name.clone(),
            access_log_s3_bucket_prefix: a.access_log_s3_bucket_prefix.clone(),
            connection_draining_enabled: a.connection_draining_enabled,
            connection_draining_timeout: a.connection_draining_timeout,
            connection_idle_timeout: a.connection_idle_timeout,
        }
    }
}

// --- From view types to internal types ---

impl From<ElbStateView> for ElbState {
    fn from(view: ElbStateView) -> Self {
        ElbState {
            load_balancers: view
                .load_balancers
                .into_iter()
                .map(|(k, v)| (k, ElbLoadBalancer::from(v)))
                .collect(),
        }
    }
}

impl From<ElbLoadBalancerView> for ElbLoadBalancer {
    fn from(view: ElbLoadBalancerView) -> Self {
        ElbLoadBalancer {
            name: view.name,
            dns_name: view.dns_name,
            scheme: view.scheme,
            availability_zones: view.availability_zones,
            subnets: view.subnets,
            security_groups: view.security_groups,
            instances: view.instances,
            listeners: view.listeners.into_iter().map(ElbListener::from).collect(),
            health_check: ElbHealthCheck::from(view.health_check),
            tags: view.tags,
            policies: view.policies.into_iter().map(ElbPolicy::from).collect(),
            attributes: ElbAttributes::from(view.attributes),
            vpc_id: view.vpc_id,
        }
    }
}

impl From<ElbListenerView> for ElbListener {
    fn from(view: ElbListenerView) -> Self {
        ElbListener {
            load_balancer_port: view.load_balancer_port,
            instance_port: view.instance_port,
            protocol: view.protocol,
            instance_protocol: view.instance_protocol,
            ssl_certificate_id: view.ssl_certificate_id,
            policy_names: view.policy_names,
        }
    }
}

impl From<ElbHealthCheckView> for ElbHealthCheck {
    fn from(view: ElbHealthCheckView) -> Self {
        ElbHealthCheck {
            target: view.target,
            interval: view.interval,
            timeout: view.timeout,
            unhealthy_threshold: view.unhealthy_threshold,
            healthy_threshold: view.healthy_threshold,
        }
    }
}

impl From<ElbPolicyView> for ElbPolicy {
    fn from(view: ElbPolicyView) -> Self {
        ElbPolicy {
            policy_name: view.policy_name,
            policy_type_name: view.policy_type_name,
            attributes: view.attributes,
        }
    }
}

impl From<ElbAttributesView> for ElbAttributes {
    fn from(view: ElbAttributesView) -> Self {
        ElbAttributes {
            cross_zone_load_balancing_enabled: view.cross_zone_load_balancing_enabled,
            access_log_enabled: view.access_log_enabled,
            access_log_emit_interval: view.access_log_emit_interval,
            access_log_s3_bucket_name: view.access_log_s3_bucket_name,
            access_log_s3_bucket_prefix: view.access_log_s3_bucket_prefix,
            connection_draining_enabled: view.connection_draining_enabled,
            connection_draining_timeout: view.connection_draining_timeout,
            connection_idle_timeout: view.connection_idle_timeout,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for ElasticLoadBalancingService {
    type StateView = ElbStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ElbStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = ElbState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (name, lb_view) in view.load_balancers {
                guard
                    .load_balancers
                    .insert(name, ElbLoadBalancer::from(lb_view));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
