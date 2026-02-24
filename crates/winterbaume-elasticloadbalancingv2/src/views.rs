//! Serde-compatible view types for ELBv2 state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ElasticLoadBalancingV2Service;
use crate::state::ElbV2State;
use crate::types::{
    AvailabilityZone, CapacityReservation, Certificate, IpamPool, Listener, ListenerAction,
    LoadBalancer, ResourcePolicy, Rule, RuleAction, RuleCondition, TargetDescription, TargetGroup,
    TrustStore, TrustStoreAssociation, TrustStoreRevocationEntry, ZonalCapacityState,
};

/// Serializable view of the entire ELBv2 state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Elbv2StateView {
    /// Load balancers keyed by ARN.
    #[serde(default)]
    pub load_balancers: HashMap<String, LoadBalancerView>,
    /// Target groups keyed by ARN.
    #[serde(default)]
    pub target_groups: HashMap<String, TargetGroupView>,
    /// Listeners keyed by ARN.
    #[serde(default)]
    pub listeners: HashMap<String, ListenerView>,
    /// Listener rules keyed by rule ARN.
    #[serde(default)]
    pub rules: HashMap<String, RuleView>,
    /// Resource tags keyed by resource ARN.
    #[serde(default)]
    pub resource_tags: HashMap<String, HashMap<String, String>>,
    /// Trust stores keyed by ARN.
    #[serde(default)]
    pub trust_stores: HashMap<String, TrustStoreView>,
    /// Trust store associations: trust_store_arn -> list of resource ARNs.
    #[serde(default)]
    pub trust_store_associations: HashMap<String, Vec<String>>,
    /// Capacity reservations keyed by LB ARN.
    #[serde(default)]
    pub capacity_reservations: HashMap<String, CapacityReservationView>,
    /// Resource-level policies keyed by resource ARN.
    #[serde(default)]
    pub resource_policies: HashMap<String, String>,
    /// IPAM pool assignments keyed by LB ARN.
    #[serde(default)]
    pub ipam_pools: HashMap<String, IpamPoolView>,
}

/// Serializable view of a load balancer (ALB/NLB/GWLB).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerView {
    pub arn: String,
    pub dns_name: String,
    pub name: String,
    pub scheme: String,
    pub state: String,
    pub lb_type: String,
    pub vpc_id: String,
    pub availability_zones: Vec<AvailabilityZoneView>,
    pub created_time: String,
    pub attributes: HashMap<String, String>,
    pub ip_address_type: String,
    pub security_groups: Vec<String>,
    pub subnets: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub connection_logs: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub subnet_mapping_tf: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub ipam_pools_tf: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub minimum_load_balancer_capacity: Vec<serde_json::Value>,
}

/// Serializable view of an availability zone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityZoneView {
    pub zone_name: String,
    pub subnet_id: String,
}

/// Serializable view of a target group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetGroupView {
    pub arn: String,
    pub name: String,
    pub protocol: String,
    pub port: i32,
    pub vpc_id: String,
    pub health_check_path: String,
    pub target_type: String,
    pub attributes: HashMap<String, String>,
    pub targets: Vec<TargetDescriptionView>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub target_failover: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub target_group_health: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub target_health_state: Vec<serde_json::Value>,
}

/// Serializable view of a target description.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetDescriptionView {
    pub id: String,
    pub port: Option<i32>,
    pub availability_zone: Option<String>,
}

/// Serializable view of a listener.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenerView {
    pub arn: String,
    pub load_balancer_arn: String,
    pub port: i32,
    pub protocol: String,
    pub default_actions: Vec<ListenerActionView>,
    pub certificates: Vec<CertificateView>,
    pub attributes: HashMap<String, String>,
}

/// Serializable view of a listener action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenerActionView {
    pub action_type: String,
    pub target_group_arn: String,
}

/// Serializable view of a certificate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateView {
    pub certificate_arn: String,
    pub is_default: Option<bool>,
}

/// Serializable view of a listener rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleView {
    pub rule_arn: String,
    pub priority: String,
    pub conditions: Vec<RuleConditionView>,
    pub actions: Vec<RuleActionView>,
    pub is_default: bool,
    pub listener_arn: String,
}

/// Serializable view of a rule condition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConditionView {
    pub field: String,
    pub values: Vec<String>,
}

/// Serializable view of a rule action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleActionView {
    pub action_type: String,
    pub target_group_arn: String,
}

/// Serializable view of a trust store revocation entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustStoreRevocationEntryView {
    pub revocation_id: i64,
    pub revocation_type: String,
    pub number_of_revoked_entries: i64,
}

/// Serializable view of a trust store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustStoreView {
    pub arn: String,
    pub name: String,
    pub status: String,
    pub number_of_ca_certificates: i32,
    pub total_revoked_entries: i64,
    pub revocations: HashMap<String, TrustStoreRevocationEntryView>,
    pub next_revocation_id: i64,
}

/// Serializable view of a capacity reservation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityReservationView {
    pub load_balancer_arn: String,
    pub minimum_capacity_units: i32,
    pub availability_zone_states: Vec<ZonalCapacityStateView>,
    pub decrease_requests_remaining: i32,
}

/// Serializable view of per-AZ capacity state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZonalCapacityStateView {
    pub availability_zone: String,
    pub effective_capacity_units: f64,
    pub status_code: String,
}

/// Serializable view of an IPAM pool assignment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamPoolView {
    pub ipv4_ipam_pool_id: Option<String>,
    pub ipv6_ipam_pool_id: Option<String>,
}

// --- From internal types to view types ---

impl From<&ElbV2State> for Elbv2StateView {
    fn from(state: &ElbV2State) -> Self {
        Elbv2StateView {
            load_balancers: state
                .load_balancers
                .iter()
                .map(|(k, v)| (k.clone(), LoadBalancerView::from(v)))
                .collect(),
            target_groups: state
                .target_groups
                .iter()
                .map(|(k, v)| (k.clone(), TargetGroupView::from(v)))
                .collect(),
            listeners: state
                .listeners
                .iter()
                .map(|(k, v)| (k.clone(), ListenerView::from(v)))
                .collect(),
            rules: state
                .rules
                .iter()
                .map(|(k, v)| (k.clone(), RuleView::from(v)))
                .collect(),
            resource_tags: state.resource_tags.clone(),
            trust_stores: state
                .trust_stores
                .iter()
                .map(|(k, v)| (k.clone(), TrustStoreView::from(v)))
                .collect(),
            trust_store_associations: state
                .trust_store_associations
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v.iter().map(|a| a.resource_arn.clone()).collect(),
                    )
                })
                .collect(),
            capacity_reservations: state
                .capacity_reservations
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        CapacityReservationView {
                            load_balancer_arn: v.load_balancer_arn.clone(),
                            minimum_capacity_units: v.minimum_capacity_units,
                            availability_zone_states: v
                                .availability_zone_states
                                .iter()
                                .map(|z| ZonalCapacityStateView {
                                    availability_zone: z.availability_zone.clone(),
                                    effective_capacity_units: z.effective_capacity_units,
                                    status_code: z.status_code.clone(),
                                })
                                .collect(),
                            decrease_requests_remaining: v.decrease_requests_remaining,
                        },
                    )
                })
                .collect(),
            resource_policies: state
                .resource_policies
                .iter()
                .map(|(k, v)| (k.clone(), v.policy.clone()))
                .collect(),
            ipam_pools: state
                .ipam_pools
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        IpamPoolView {
                            ipv4_ipam_pool_id: v.ipv4_ipam_pool_id.clone(),
                            ipv6_ipam_pool_id: v.ipv6_ipam_pool_id.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

impl From<&LoadBalancer> for LoadBalancerView {
    fn from(lb: &LoadBalancer) -> Self {
        LoadBalancerView {
            arn: lb.arn.clone(),
            dns_name: lb.dns_name.clone(),
            name: lb.name.clone(),
            scheme: lb.scheme.clone(),
            state: lb.state.clone(),
            lb_type: lb.lb_type.clone(),
            vpc_id: lb.vpc_id.clone(),
            availability_zones: lb
                .availability_zones
                .iter()
                .map(AvailabilityZoneView::from)
                .collect(),
            created_time: lb.created_time.to_rfc3339(),
            attributes: lb.attributes.clone(),
            ip_address_type: lb.ip_address_type.clone(),
            security_groups: lb.security_groups.clone(),
            subnets: lb.subnets.clone(),
            connection_logs: None,
            subnet_mapping_tf: vec![],
            ipam_pools_tf: vec![],
            minimum_load_balancer_capacity: vec![],
        }
    }
}

impl From<&AvailabilityZone> for AvailabilityZoneView {
    fn from(az: &AvailabilityZone) -> Self {
        AvailabilityZoneView {
            zone_name: az.zone_name.clone(),
            subnet_id: az.subnet_id.clone(),
        }
    }
}

impl From<&TargetGroup> for TargetGroupView {
    fn from(tg: &TargetGroup) -> Self {
        TargetGroupView {
            arn: tg.arn.clone(),
            name: tg.name.clone(),
            protocol: tg.protocol.clone(),
            port: tg.port,
            vpc_id: tg.vpc_id.clone(),
            health_check_path: tg.health_check_path.clone(),
            target_type: tg.target_type.clone(),
            attributes: tg.attributes.clone(),
            targets: tg.targets.iter().map(TargetDescriptionView::from).collect(),
            target_failover: vec![],
            target_group_health: vec![],
            target_health_state: vec![],
        }
    }
}

impl From<&TargetDescription> for TargetDescriptionView {
    fn from(t: &TargetDescription) -> Self {
        TargetDescriptionView {
            id: t.id.clone(),
            port: t.port,
            availability_zone: t.availability_zone.clone(),
        }
    }
}

impl From<&Listener> for ListenerView {
    fn from(l: &Listener) -> Self {
        ListenerView {
            arn: l.arn.clone(),
            load_balancer_arn: l.load_balancer_arn.clone(),
            port: l.port,
            protocol: l.protocol.clone(),
            default_actions: l
                .default_actions
                .iter()
                .map(ListenerActionView::from)
                .collect(),
            certificates: l.certificates.iter().map(CertificateView::from).collect(),
            attributes: l.attributes.clone(),
        }
    }
}

impl From<&ListenerAction> for ListenerActionView {
    fn from(a: &ListenerAction) -> Self {
        ListenerActionView {
            action_type: a.action_type.clone(),
            target_group_arn: a.target_group_arn.clone(),
        }
    }
}

impl From<&Certificate> for CertificateView {
    fn from(c: &Certificate) -> Self {
        CertificateView {
            certificate_arn: c.certificate_arn.clone(),
            is_default: c.is_default,
        }
    }
}

impl From<&Rule> for RuleView {
    fn from(r: &Rule) -> Self {
        RuleView {
            rule_arn: r.rule_arn.clone(),
            priority: r.priority.clone(),
            conditions: r.conditions.iter().map(RuleConditionView::from).collect(),
            actions: r.actions.iter().map(RuleActionView::from).collect(),
            is_default: r.is_default,
            listener_arn: r.listener_arn.clone(),
        }
    }
}

impl From<&RuleCondition> for RuleConditionView {
    fn from(c: &RuleCondition) -> Self {
        RuleConditionView {
            field: c.field.clone(),
            values: c.values.clone(),
        }
    }
}

impl From<&RuleAction> for RuleActionView {
    fn from(a: &RuleAction) -> Self {
        RuleActionView {
            action_type: a.action_type.clone(),
            target_group_arn: a.target_group_arn.clone(),
        }
    }
}

impl From<&TrustStore> for TrustStoreView {
    fn from(ts: &TrustStore) -> Self {
        TrustStoreView {
            arn: ts.arn.clone(),
            name: ts.name.clone(),
            status: ts.status.clone(),
            number_of_ca_certificates: ts.number_of_ca_certificates,
            total_revoked_entries: ts.total_revoked_entries,
            revocations: ts
                .revocations
                .iter()
                .map(|(k, v)| (k.to_string(), TrustStoreRevocationEntryView::from(v)))
                .collect(),
            next_revocation_id: ts.next_revocation_id,
        }
    }
}

impl From<&TrustStoreRevocationEntry> for TrustStoreRevocationEntryView {
    fn from(e: &TrustStoreRevocationEntry) -> Self {
        TrustStoreRevocationEntryView {
            revocation_id: e.revocation_id,
            revocation_type: e.revocation_type.clone(),
            number_of_revoked_entries: e.number_of_revoked_entries,
        }
    }
}

// --- From view types to internal types ---

impl From<Elbv2StateView> for ElbV2State {
    fn from(view: Elbv2StateView) -> Self {
        ElbV2State {
            load_balancers: view
                .load_balancers
                .into_iter()
                .map(|(k, v)| (k, LoadBalancer::from(v)))
                .collect(),
            target_groups: view
                .target_groups
                .into_iter()
                .map(|(k, v)| (k, TargetGroup::from(v)))
                .collect(),
            listeners: view
                .listeners
                .into_iter()
                .map(|(k, v)| (k, Listener::from(v)))
                .collect(),
            rules: view
                .rules
                .into_iter()
                .map(|(k, v)| (k, Rule::from(v)))
                .collect(),
            resource_tags: view.resource_tags,
            trust_stores: view
                .trust_stores
                .into_iter()
                .map(|(k, v)| (k, TrustStore::from(v)))
                .collect(),
            trust_store_associations: view
                .trust_store_associations
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        v.into_iter()
                            .map(|arn| TrustStoreAssociation { resource_arn: arn })
                            .collect(),
                    )
                })
                .collect(),
            capacity_reservations: view
                .capacity_reservations
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        CapacityReservation {
                            load_balancer_arn: v.load_balancer_arn,
                            minimum_capacity_units: v.minimum_capacity_units,
                            availability_zone_states: v
                                .availability_zone_states
                                .into_iter()
                                .map(|z| ZonalCapacityState {
                                    availability_zone: z.availability_zone,
                                    effective_capacity_units: z.effective_capacity_units,
                                    status_code: z.status_code,
                                })
                                .collect(),
                            decrease_requests_remaining: v.decrease_requests_remaining,
                        },
                    )
                })
                .collect(),
            resource_policies: view
                .resource_policies
                .into_iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ResourcePolicy {
                            resource_arn: k,
                            policy: v,
                        },
                    )
                })
                .collect(),
            ipam_pools: view
                .ipam_pools
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        IpamPool {
                            ipv4_ipam_pool_id: v.ipv4_ipam_pool_id,
                            ipv6_ipam_pool_id: v.ipv6_ipam_pool_id,
                        },
                    )
                })
                .collect(),
        }
    }
}

impl From<TrustStoreView> for TrustStore {
    fn from(view: TrustStoreView) -> Self {
        TrustStore {
            arn: view.arn,
            name: view.name,
            status: view.status,
            number_of_ca_certificates: view.number_of_ca_certificates,
            total_revoked_entries: view.total_revoked_entries,
            revocations: view
                .revocations
                .into_iter()
                .filter_map(|(k, v)| {
                    k.parse::<i64>()
                        .ok()
                        .map(|id| (id, TrustStoreRevocationEntry::from(v)))
                })
                .collect(),
            next_revocation_id: view.next_revocation_id,
        }
    }
}

impl From<TrustStoreRevocationEntryView> for TrustStoreRevocationEntry {
    fn from(view: TrustStoreRevocationEntryView) -> Self {
        TrustStoreRevocationEntry {
            revocation_id: view.revocation_id,
            revocation_type: view.revocation_type,
            number_of_revoked_entries: view.number_of_revoked_entries,
        }
    }
}

impl From<LoadBalancerView> for LoadBalancer {
    fn from(view: LoadBalancerView) -> Self {
        let created_time = DateTime::parse_from_rfc3339(&view.created_time)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        LoadBalancer {
            arn: view.arn,
            dns_name: view.dns_name,
            name: view.name,
            scheme: view.scheme,
            state: view.state,
            lb_type: view.lb_type,
            vpc_id: view.vpc_id,
            availability_zones: view
                .availability_zones
                .into_iter()
                .map(AvailabilityZone::from)
                .collect(),
            created_time,
            attributes: view.attributes,
            ip_address_type: view.ip_address_type,
            security_groups: view.security_groups,
            subnets: view.subnets,
        }
    }
}

impl From<AvailabilityZoneView> for AvailabilityZone {
    fn from(view: AvailabilityZoneView) -> Self {
        AvailabilityZone {
            zone_name: view.zone_name,
            subnet_id: view.subnet_id,
        }
    }
}

impl From<TargetGroupView> for TargetGroup {
    fn from(view: TargetGroupView) -> Self {
        TargetGroup {
            arn: view.arn,
            name: view.name,
            protocol: view.protocol,
            port: view.port,
            vpc_id: view.vpc_id,
            health_check_path: view.health_check_path,
            target_type: view.target_type,
            attributes: view.attributes,
            targets: view
                .targets
                .into_iter()
                .map(TargetDescription::from)
                .collect(),
        }
    }
}

impl From<TargetDescriptionView> for TargetDescription {
    fn from(view: TargetDescriptionView) -> Self {
        TargetDescription {
            id: view.id,
            port: view.port,
            availability_zone: view.availability_zone,
        }
    }
}

impl From<ListenerView> for Listener {
    fn from(view: ListenerView) -> Self {
        Listener {
            arn: view.arn,
            load_balancer_arn: view.load_balancer_arn,
            port: view.port,
            protocol: view.protocol,
            default_actions: view
                .default_actions
                .into_iter()
                .map(ListenerAction::from)
                .collect(),
            certificates: view
                .certificates
                .into_iter()
                .map(Certificate::from)
                .collect(),
            attributes: view.attributes,
        }
    }
}

impl From<ListenerActionView> for ListenerAction {
    fn from(view: ListenerActionView) -> Self {
        ListenerAction {
            action_type: view.action_type,
            target_group_arn: view.target_group_arn,
        }
    }
}

impl From<CertificateView> for Certificate {
    fn from(view: CertificateView) -> Self {
        Certificate {
            certificate_arn: view.certificate_arn,
            is_default: view.is_default,
        }
    }
}

impl From<RuleView> for Rule {
    fn from(view: RuleView) -> Self {
        Rule {
            rule_arn: view.rule_arn,
            priority: view.priority,
            conditions: view
                .conditions
                .into_iter()
                .map(RuleCondition::from)
                .collect(),
            actions: view.actions.into_iter().map(RuleAction::from).collect(),
            is_default: view.is_default,
            listener_arn: view.listener_arn,
        }
    }
}

impl From<RuleConditionView> for RuleCondition {
    fn from(view: RuleConditionView) -> Self {
        RuleCondition {
            field: view.field,
            values: view.values,
        }
    }
}

impl From<RuleActionView> for RuleAction {
    fn from(view: RuleActionView) -> Self {
        RuleAction {
            action_type: view.action_type,
            target_group_arn: view.target_group_arn,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for ElasticLoadBalancingV2Service {
    type StateView = Elbv2StateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        Elbv2StateView::from(&*guard)
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
            *guard = ElbV2State::from(view);
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
            for (arn, lb_view) in view.load_balancers {
                guard
                    .load_balancers
                    .insert(arn, LoadBalancer::from(lb_view));
            }
            for (arn, tg_view) in view.target_groups {
                guard.target_groups.insert(arn, TargetGroup::from(tg_view));
            }
            for (arn, listener_view) in view.listeners {
                guard.listeners.insert(arn, Listener::from(listener_view));
            }
            for (arn, rule_view) in view.rules {
                guard.rules.insert(arn, Rule::from(rule_view));
            }
            for (arn, tags) in view.resource_tags {
                guard.resource_tags.insert(arn, tags);
            }
            for (arn, ts_view) in view.trust_stores {
                guard.trust_stores.insert(arn, TrustStore::from(ts_view));
            }
            for (arn, assocs) in view.trust_store_associations {
                let entries: Vec<TrustStoreAssociation> = assocs
                    .into_iter()
                    .map(|r| TrustStoreAssociation { resource_arn: r })
                    .collect();
                guard
                    .trust_store_associations
                    .entry(arn)
                    .or_default()
                    .extend(entries);
            }
            for (arn, cap_view) in view.capacity_reservations {
                guard.capacity_reservations.insert(
                    arn,
                    CapacityReservation {
                        load_balancer_arn: cap_view.load_balancer_arn,
                        minimum_capacity_units: cap_view.minimum_capacity_units,
                        availability_zone_states: cap_view
                            .availability_zone_states
                            .into_iter()
                            .map(|z| ZonalCapacityState {
                                availability_zone: z.availability_zone,
                                effective_capacity_units: z.effective_capacity_units,
                                status_code: z.status_code,
                            })
                            .collect(),
                        decrease_requests_remaining: cap_view.decrease_requests_remaining,
                    },
                );
            }
            for (arn, policy) in view.resource_policies {
                guard.resource_policies.insert(
                    arn.clone(),
                    ResourcePolicy {
                        resource_arn: arn,
                        policy,
                    },
                );
            }
            for (arn, pool_view) in view.ipam_pools {
                guard.ipam_pools.insert(
                    arn,
                    IpamPool {
                        ipv4_ipam_pool_id: pool_view.ipv4_ipam_pool_id,
                        ipv6_ipam_pool_id: pool_view.ipv6_ipam_pool_id,
                    },
                );
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
