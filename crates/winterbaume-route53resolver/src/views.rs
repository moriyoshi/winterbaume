//! Serde-compatible view types for Route53Resolver state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::Route53ResolverService;
use crate::state::Route53ResolverState;
use crate::types::{
    IpAddressEntry, ResolverDnssecConfig, ResolverEndpoint, ResolverQueryLogConfig,
    ResolverQueryLogConfigAssociation, ResolverRule, ResolverRuleAssociation, TargetAddress,
};

/// Serializable view of the entire Route53Resolver state.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Route53ResolverStateView {
    /// Resolver endpoints keyed by endpoint ID.
    #[serde(default)]
    pub endpoints: HashMap<String, ResolverEndpointView>,
    /// Resolver rules keyed by rule ID.
    #[serde(default)]
    pub resolver_rules: HashMap<String, ResolverRuleView>,
    /// Rule associations keyed by association ID.
    #[serde(default)]
    pub rule_associations: HashMap<String, ResolverRuleAssociationView>,
    /// Query log configs keyed by config ID.
    #[serde(default)]
    pub query_log_configs: HashMap<String, ResolverQueryLogConfigView>,
    /// Query log config associations keyed by association ID.
    #[serde(default)]
    pub query_log_config_associations: HashMap<String, ResolverQueryLogConfigAssociationView>,
    /// DNSSEC configs keyed by resource ID.
    #[serde(default)]
    pub dnssec_configs: HashMap<String, ResolverDnssecConfigView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolverEndpointView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub security_group_ids: Vec<String>,
    pub direction: String,
    pub ip_address_count: i32,
    pub host_vpc_id: String,
    pub status: String,
    pub status_message: String,
    pub creation_time: String,
    pub modification_time: String,
    pub creator_request_id: String,
    pub protocols: Vec<String>,
    pub resolver_endpoint_type: String,
    pub ip_addresses: Vec<IpAddressEntryView>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpAddressEntryView {
    pub ip_id: String,
    pub subnet_id: String,
    pub ip: Option<String>,
    pub status: String,
    pub status_message: String,
    pub creation_time: String,
    pub modification_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolverRuleView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub domain_name: String,
    pub rule_type: String,
    pub resolver_endpoint_id: Option<String>,
    pub target_ips: Vec<TargetAddressView>,
    pub status: String,
    pub status_message: String,
    pub owner_id: String,
    pub share_status: String,
    pub creator_request_id: String,
    pub creation_time: String,
    pub modification_time: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetAddressView {
    pub ip: Option<String>,
    pub ipv6: Option<String>,
    pub port: Option<i32>,
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolverRuleAssociationView {
    pub id: String,
    pub resolver_rule_id: String,
    pub name: String,
    pub vpc_id: String,
    pub status: String,
    pub status_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolverQueryLogConfigView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub destination_arn: String,
    pub owner_id: String,
    pub status: String,
    pub share_status: String,
    pub association_count: i32,
    pub creator_request_id: String,
    pub creation_time: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolverQueryLogConfigAssociationView {
    pub id: String,
    pub resolver_query_log_config_id: String,
    pub resource_id: String,
    pub status: String,
    pub error: String,
    pub error_message: String,
    pub creation_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolverDnssecConfigView {
    pub id: String,
    pub owner_id: String,
    pub resource_id: String,
    pub validation_status: String,
}

// ---------------------------------------------------------------------------
// From conversions
// ---------------------------------------------------------------------------

impl From<&IpAddressEntry> for IpAddressEntryView {
    fn from(e: &IpAddressEntry) -> Self {
        Self {
            ip_id: e.ip_id.clone(),
            subnet_id: e.subnet_id.clone(),
            ip: e.ip.clone(),
            status: e.status.clone(),
            status_message: e.status_message.clone(),
            creation_time: e.creation_time.to_rfc3339(),
            modification_time: e.modification_time.to_rfc3339(),
        }
    }
}

impl From<&ResolverEndpoint> for ResolverEndpointView {
    fn from(e: &ResolverEndpoint) -> Self {
        Self {
            id: e.id.clone(),
            arn: e.arn.clone(),
            name: e.name.clone(),
            security_group_ids: e.security_group_ids.clone(),
            direction: e.direction.clone(),
            ip_address_count: e.ip_address_count,
            host_vpc_id: e.host_vpc_id.clone(),
            status: e.status.clone(),
            status_message: e.status_message.clone(),
            creation_time: e.creation_time.to_rfc3339(),
            modification_time: e.modification_time.to_rfc3339(),
            creator_request_id: e.creator_request_id.clone(),
            protocols: e.protocols.clone(),
            resolver_endpoint_type: e.resolver_endpoint_type.clone(),
            ip_addresses: e
                .ip_addresses
                .iter()
                .map(IpAddressEntryView::from)
                .collect(),
            tags: e.tags.clone(),
        }
    }
}

impl From<&TargetAddress> for TargetAddressView {
    fn from(t: &TargetAddress) -> Self {
        Self {
            ip: t.ip.clone(),
            ipv6: t.ipv6.clone(),
            port: t.port,
            protocol: t.protocol.clone(),
        }
    }
}

impl From<&ResolverRule> for ResolverRuleView {
    fn from(r: &ResolverRule) -> Self {
        Self {
            id: r.id.clone(),
            arn: r.arn.clone(),
            name: r.name.clone(),
            domain_name: r.domain_name.clone(),
            rule_type: r.rule_type.clone(),
            resolver_endpoint_id: r.resolver_endpoint_id.clone(),
            target_ips: r.target_ips.iter().map(TargetAddressView::from).collect(),
            status: r.status.clone(),
            status_message: r.status_message.clone(),
            owner_id: r.owner_id.clone(),
            share_status: r.share_status.clone(),
            creator_request_id: r.creator_request_id.clone(),
            creation_time: r.creation_time.to_rfc3339(),
            modification_time: r.modification_time.to_rfc3339(),
            tags: r.tags.clone(),
        }
    }
}

impl From<&ResolverRuleAssociation> for ResolverRuleAssociationView {
    fn from(a: &ResolverRuleAssociation) -> Self {
        Self {
            id: a.id.clone(),
            resolver_rule_id: a.resolver_rule_id.clone(),
            name: a.name.clone(),
            vpc_id: a.vpc_id.clone(),
            status: a.status.clone(),
            status_message: a.status_message.clone(),
        }
    }
}

impl From<&ResolverQueryLogConfig> for ResolverQueryLogConfigView {
    fn from(c: &ResolverQueryLogConfig) -> Self {
        Self {
            id: c.id.clone(),
            arn: c.arn.clone(),
            name: c.name.clone(),
            destination_arn: c.destination_arn.clone(),
            owner_id: c.owner_id.clone(),
            status: c.status.clone(),
            share_status: c.share_status.clone(),
            association_count: c.association_count,
            creator_request_id: c.creator_request_id.clone(),
            creation_time: c.creation_time.to_rfc3339(),
            tags: c.tags.clone(),
        }
    }
}

impl From<&ResolverQueryLogConfigAssociation> for ResolverQueryLogConfigAssociationView {
    fn from(a: &ResolverQueryLogConfigAssociation) -> Self {
        Self {
            id: a.id.clone(),
            resolver_query_log_config_id: a.resolver_query_log_config_id.clone(),
            resource_id: a.resource_id.clone(),
            status: a.status.clone(),
            error: a.error.clone(),
            error_message: a.error_message.clone(),
            creation_time: a.creation_time.to_rfc3339(),
        }
    }
}

impl From<&ResolverDnssecConfig> for ResolverDnssecConfigView {
    fn from(c: &ResolverDnssecConfig) -> Self {
        Self {
            id: c.id.clone(),
            owner_id: c.owner_id.clone(),
            resource_id: c.resource_id.clone(),
            validation_status: c.validation_status.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for Route53ResolverService {
    type StateView = Route53ResolverStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;

        Route53ResolverStateView {
            endpoints: guard
                .endpoints
                .iter()
                .map(|(k, v)| (k.clone(), ResolverEndpointView::from(v)))
                .collect(),
            resolver_rules: guard
                .resolver_rules
                .iter()
                .map(|(k, v)| (k.clone(), ResolverRuleView::from(v)))
                .collect(),
            rule_associations: guard
                .rule_associations
                .iter()
                .map(|(k, v)| (k.clone(), ResolverRuleAssociationView::from(v)))
                .collect(),
            query_log_configs: guard
                .query_log_configs
                .iter()
                .map(|(k, v)| (k.clone(), ResolverQueryLogConfigView::from(v)))
                .collect(),
            query_log_config_associations: guard
                .query_log_config_associations
                .iter()
                .map(|(k, v)| (k.clone(), ResolverQueryLogConfigAssociationView::from(v)))
                .collect(),
            dnssec_configs: guard
                .dnssec_configs
                .iter()
                .map(|(k, v)| (k.clone(), ResolverDnssecConfigView::from(v)))
                .collect(),
        }
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let new_state = state_from_view(view);
        {
            let state = self.state.get(account_id, region);
            *state.write().await = new_state;
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
        let incoming = state_from_view(view);
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            guard.endpoints.extend(incoming.endpoints);
            guard.resolver_rules.extend(incoming.resolver_rules);
            guard.rule_associations.extend(incoming.rule_associations);
            guard.query_log_configs.extend(incoming.query_log_configs);
            guard
                .query_log_config_associations
                .extend(incoming.query_log_config_associations);
            guard.dnssec_configs.extend(incoming.dnssec_configs);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

fn parse_dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

fn state_from_view(view: Route53ResolverStateView) -> Route53ResolverState {
    let mut state = Route53ResolverState::default();

    for (id, ev) in view.endpoints {
        let ip_addresses = ev
            .ip_addresses
            .into_iter()
            .map(|e| IpAddressEntry {
                ip_id: e.ip_id,
                subnet_id: e.subnet_id,
                ip: e.ip,
                status: e.status,
                status_message: e.status_message,
                creation_time: parse_dt(&e.creation_time),
                modification_time: parse_dt(&e.modification_time),
            })
            .collect();
        state.endpoints.insert(
            id,
            ResolverEndpoint {
                id: ev.id,
                arn: ev.arn,
                name: ev.name,
                security_group_ids: ev.security_group_ids,
                direction: ev.direction,
                ip_address_count: ev.ip_address_count,
                host_vpc_id: ev.host_vpc_id,
                status: ev.status,
                status_message: ev.status_message,
                creation_time: parse_dt(&ev.creation_time),
                modification_time: parse_dt(&ev.modification_time),
                creator_request_id: ev.creator_request_id,
                protocols: ev.protocols,
                resolver_endpoint_type: ev.resolver_endpoint_type,
                ip_addresses,
                tags: ev.tags,
            },
        );
    }

    for (id, rv) in view.resolver_rules {
        let target_ips = rv
            .target_ips
            .into_iter()
            .map(|t| TargetAddress {
                ip: t.ip,
                ipv6: t.ipv6,
                port: t.port,
                protocol: t.protocol,
            })
            .collect();
        state.resolver_rules.insert(
            id,
            ResolverRule {
                id: rv.id,
                arn: rv.arn,
                name: rv.name,
                domain_name: rv.domain_name,
                rule_type: rv.rule_type,
                resolver_endpoint_id: rv.resolver_endpoint_id,
                target_ips,
                status: rv.status,
                status_message: rv.status_message,
                owner_id: rv.owner_id,
                share_status: rv.share_status,
                creator_request_id: rv.creator_request_id,
                creation_time: parse_dt(&rv.creation_time),
                modification_time: parse_dt(&rv.modification_time),
                tags: rv.tags,
            },
        );
    }

    for (id, av) in view.rule_associations {
        state.rule_associations.insert(
            id,
            ResolverRuleAssociation {
                id: av.id,
                resolver_rule_id: av.resolver_rule_id,
                name: av.name,
                vpc_id: av.vpc_id,
                status: av.status,
                status_message: av.status_message,
            },
        );
    }

    for (id, cv) in view.query_log_configs {
        state.query_log_configs.insert(
            id,
            ResolverQueryLogConfig {
                id: cv.id,
                arn: cv.arn,
                name: cv.name,
                destination_arn: cv.destination_arn,
                owner_id: cv.owner_id,
                status: cv.status,
                share_status: cv.share_status,
                association_count: cv.association_count,
                creator_request_id: cv.creator_request_id,
                creation_time: parse_dt(&cv.creation_time),
                tags: cv.tags,
            },
        );
    }

    for (id, av) in view.query_log_config_associations {
        state.query_log_config_associations.insert(
            id,
            ResolverQueryLogConfigAssociation {
                id: av.id,
                resolver_query_log_config_id: av.resolver_query_log_config_id,
                resource_id: av.resource_id,
                status: av.status,
                error: av.error,
                error_message: av.error_message,
                creation_time: parse_dt(&av.creation_time),
            },
        );
    }

    for (id, dv) in view.dnssec_configs {
        state.dnssec_configs.insert(
            id,
            ResolverDnssecConfig {
                id: dv.id,
                owner_id: dv.owner_id,
                resource_id: dv.resource_id,
                validation_status: dv.validation_status,
            },
        );
    }

    state
}
