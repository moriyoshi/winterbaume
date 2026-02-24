//! Serde-compatible view types for VPC Lattice state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::VpcLatticeService;
use crate::state::VpcLatticeState;
use crate::types::{
    AccessLogSubscription, AuthPolicy, DomainVerification, Listener, ListenerDefaultAction,
    ResourceConfiguration, ResourceGateway, ResourcePolicy, Rule, RuleAction, RuleMatchData,
    Service, ServiceNetwork, ServiceNetworkResourceAssociation, ServiceNetworkServiceAssociation,
    ServiceNetworkVpcAssociation, TargetEntry, TargetGroup, TargetGroupConfig,
};

/// Serializable view of the entire VPC Lattice state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VpcLatticeStateView {
    #[serde(default)]
    pub service_networks: HashMap<String, ServiceNetworkView>,
    #[serde(default)]
    pub access_log_subscriptions: HashMap<String, AccessLogSubscriptionView>,
    #[serde(default)]
    pub sn_service_associations: HashMap<String, SnServiceAssociationView>,
    #[serde(default)]
    pub sn_vpc_associations: HashMap<String, SnVpcAssociationView>,
    #[serde(default)]
    pub target_groups: HashMap<String, TargetGroupView>,
    #[serde(default)]
    pub services: HashMap<String, ServiceView>,
    #[serde(default)]
    pub auth_policies: HashMap<String, AuthPolicyView>,
    #[serde(default)]
    pub resource_policies: HashMap<String, ResourcePolicyView>,
    #[serde(default)]
    pub listeners: HashMap<String, ListenerView>,
    #[serde(default)]
    pub rules: HashMap<String, RuleView>,
    #[serde(default)]
    pub resource_configurations: HashMap<String, ResourceConfigurationView>,
    #[serde(default)]
    pub resource_gateways: HashMap<String, ResourceGatewayView>,
    #[serde(default)]
    pub sn_resource_associations: HashMap<String, SnResourceAssociationView>,
    #[serde(default)]
    pub domain_verifications: HashMap<String, DomainVerificationView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceNetworkView {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub auth_type: String,
    pub created_at: String,
    pub last_updated_at: String,
    pub number_of_associated_services: i64,
    pub number_of_associated_v_p_cs: i64,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessLogSubscriptionView {
    pub id: String,
    pub arn: String,
    pub resource_arn: String,
    pub resource_id: String,
    pub destination_arn: String,
    pub created_at: String,
    pub last_updated_at: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnServiceAssociationView {
    pub id: String,
    pub arn: String,
    pub service_network_identifier: String,
    pub service_identifier: String,
    pub service_network_id: String,
    pub service_network_arn: String,
    pub service_network_name: String,
    pub service_id: String,
    pub service_arn: String,
    pub service_name: String,
    pub status: String,
    pub created_at: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnVpcAssociationView {
    pub id: String,
    pub arn: String,
    pub service_network_identifier: String,
    pub vpc_identifier: String,
    pub service_network_id: String,
    pub service_network_arn: String,
    pub service_network_name: String,
    pub vpc_id: String,
    pub status: String,
    #[serde(default)]
    pub security_group_ids: Vec<String>,
    pub created_at: String,
    pub last_updated_at: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetGroupView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub target_group_type: String,
    pub config_port: Option<i32>,
    pub config_protocol: Option<String>,
    pub config_vpc_identifier: Option<String>,
    pub config_ip_address_type: Option<String>,
    pub config_protocol_version: Option<String>,
    pub config_lambda_event_structure_version: Option<String>,
    pub status: String,
    pub created_at: String,
    pub last_updated_at: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub targets: Vec<TargetEntryView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetEntryView {
    pub id: String,
    pub port: Option<i32>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceView {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub auth_type: String,
    pub status: String,
    pub created_at: String,
    pub last_updated_at: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthPolicyView {
    pub policy: String,
    pub state: String,
    pub created_at: String,
    pub last_updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePolicyView {
    pub policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenerView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub service_id: String,
    pub service_arn: String,
    pub port: Option<i32>,
    pub protocol: String,
    pub default_action_fixed_response_status_code: Option<i32>,
    #[serde(default)]
    pub default_action_forward_target_groups: Vec<(String, Option<i32>)>,
    pub created_at: String,
    pub last_updated_at: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub listener_id: String,
    pub service_id: String,
    pub priority: i32,
    pub is_default: bool,
    pub action_fixed_response_status_code: Option<i32>,
    #[serde(default)]
    pub action_forward_target_groups: Vec<(String, Option<i32>)>,
    pub match_method: Option<String>,
    pub match_path_exact: Option<String>,
    pub match_path_prefix: Option<String>,
    pub created_at: String,
    pub last_updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConfigurationView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub resource_configuration_type: String,
    pub status: String,
    pub resource_gateway_id: Option<String>,
    #[serde(default)]
    pub port_ranges: Vec<String>,
    pub protocol: Option<String>,
    pub created_at: String,
    pub last_updated_at: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceGatewayView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub status: String,
    pub vpc_id: Option<String>,
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    #[serde(default)]
    pub security_group_ids: Vec<String>,
    pub ip_address_type: Option<String>,
    pub created_at: String,
    pub last_updated_at: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnResourceAssociationView {
    pub id: String,
    pub arn: String,
    pub service_network_identifier: String,
    pub resource_configuration_identifier: String,
    pub service_network_id: String,
    pub service_network_arn: String,
    pub service_network_name: String,
    pub resource_configuration_id: String,
    pub resource_configuration_arn: String,
    pub resource_configuration_name: String,
    pub status: String,
    pub created_at: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainVerificationView {
    pub id: String,
    pub arn: String,
    pub domain_name: String,
    pub status: String,
    pub created_at: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

fn dt_to_str(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

fn parse_dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

// --- From internal types to view types ---

impl From<&VpcLatticeState> for VpcLatticeStateView {
    fn from(state: &VpcLatticeState) -> Self {
        VpcLatticeStateView {
            service_networks: state
                .service_networks
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ServiceNetworkView {
                            id: v.id.clone(),
                            name: v.name.clone(),
                            arn: v.arn.clone(),
                            auth_type: v.auth_type.clone(),
                            created_at: dt_to_str(&v.created_at),
                            last_updated_at: dt_to_str(&v.last_updated_at),
                            number_of_associated_services: v.number_of_associated_services,
                            number_of_associated_v_p_cs: v.number_of_associated_v_p_cs,
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            access_log_subscriptions: state
                .access_log_subscriptions
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        AccessLogSubscriptionView {
                            id: v.id.clone(),
                            arn: v.arn.clone(),
                            resource_arn: v.resource_arn.clone(),
                            resource_id: v.resource_id.clone(),
                            destination_arn: v.destination_arn.clone(),
                            created_at: dt_to_str(&v.created_at),
                            last_updated_at: dt_to_str(&v.last_updated_at),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            sn_service_associations: state
                .sn_service_associations
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        SnServiceAssociationView {
                            id: v.id.clone(),
                            arn: v.arn.clone(),
                            service_network_identifier: v.service_network_identifier.clone(),
                            service_identifier: v.service_identifier.clone(),
                            service_network_id: v.service_network_id.clone(),
                            service_network_arn: v.service_network_arn.clone(),
                            service_network_name: v.service_network_name.clone(),
                            service_id: v.service_id.clone(),
                            service_arn: v.service_arn.clone(),
                            service_name: v.service_name.clone(),
                            status: v.status.clone(),
                            created_at: dt_to_str(&v.created_at),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            sn_vpc_associations: state
                .sn_vpc_associations
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        SnVpcAssociationView {
                            id: v.id.clone(),
                            arn: v.arn.clone(),
                            service_network_identifier: v.service_network_identifier.clone(),
                            vpc_identifier: v.vpc_identifier.clone(),
                            service_network_id: v.service_network_id.clone(),
                            service_network_arn: v.service_network_arn.clone(),
                            service_network_name: v.service_network_name.clone(),
                            vpc_id: v.vpc_id.clone(),
                            status: v.status.clone(),
                            security_group_ids: v.security_group_ids.clone(),
                            created_at: dt_to_str(&v.created_at),
                            last_updated_at: dt_to_str(&v.last_updated_at),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            target_groups: state
                .target_groups
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        TargetGroupView {
                            id: v.id.clone(),
                            arn: v.arn.clone(),
                            name: v.name.clone(),
                            target_group_type: v.target_group_type.clone(),
                            config_port: v.config.as_ref().and_then(|c| c.port),
                            config_protocol: v.config.as_ref().and_then(|c| c.protocol.clone()),
                            config_vpc_identifier: v
                                .config
                                .as_ref()
                                .and_then(|c| c.vpc_identifier.clone()),
                            config_ip_address_type: v
                                .config
                                .as_ref()
                                .and_then(|c| c.ip_address_type.clone()),
                            config_protocol_version: v
                                .config
                                .as_ref()
                                .and_then(|c| c.protocol_version.clone()),
                            config_lambda_event_structure_version: v
                                .config
                                .as_ref()
                                .and_then(|c| c.lambda_event_structure_version.clone()),
                            status: v.status.clone(),
                            created_at: dt_to_str(&v.created_at),
                            last_updated_at: dt_to_str(&v.last_updated_at),
                            tags: v.tags.clone(),
                            targets: v
                                .targets
                                .iter()
                                .map(|t| TargetEntryView {
                                    id: t.id.clone(),
                                    port: t.port,
                                    status: t.status.clone(),
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
            services: state
                .services
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ServiceView {
                            id: v.id.clone(),
                            name: v.name.clone(),
                            arn: v.arn.clone(),
                            auth_type: v.auth_type.clone(),
                            status: v.status.clone(),
                            created_at: dt_to_str(&v.created_at),
                            last_updated_at: dt_to_str(&v.last_updated_at),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            auth_policies: state
                .auth_policies
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        AuthPolicyView {
                            policy: v.policy.clone(),
                            state: v.state.clone(),
                            created_at: dt_to_str(&v.created_at),
                            last_updated_at: dt_to_str(&v.last_updated_at),
                        },
                    )
                })
                .collect(),
            resource_policies: state
                .resource_policies
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ResourcePolicyView {
                            policy: v.policy.clone(),
                        },
                    )
                })
                .collect(),
            listeners: state
                .listeners
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ListenerView {
                            id: v.id.clone(),
                            arn: v.arn.clone(),
                            name: v.name.clone(),
                            service_id: v.service_id.clone(),
                            service_arn: v.service_arn.clone(),
                            port: v.port,
                            protocol: v.protocol.clone(),
                            default_action_fixed_response_status_code: v
                                .default_action
                                .fixed_response_status_code,
                            default_action_forward_target_groups: v
                                .default_action
                                .forward_target_groups
                                .clone(),
                            created_at: dt_to_str(&v.created_at),
                            last_updated_at: dt_to_str(&v.last_updated_at),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            rules: state
                .rules
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        RuleView {
                            id: v.id.clone(),
                            arn: v.arn.clone(),
                            name: v.name.clone(),
                            listener_id: v.listener_id.clone(),
                            service_id: v.service_id.clone(),
                            priority: v.priority,
                            is_default: v.is_default,
                            action_fixed_response_status_code: v.action.fixed_response_status_code,
                            action_forward_target_groups: v.action.forward_target_groups.clone(),
                            match_method: v.rule_match.as_ref().and_then(|m| m.method.clone()),
                            match_path_exact: v
                                .rule_match
                                .as_ref()
                                .and_then(|m| m.path_exact.clone()),
                            match_path_prefix: v
                                .rule_match
                                .as_ref()
                                .and_then(|m| m.path_prefix.clone()),
                            created_at: dt_to_str(&v.created_at),
                            last_updated_at: dt_to_str(&v.last_updated_at),
                        },
                    )
                })
                .collect(),
            resource_configurations: state
                .resource_configurations
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ResourceConfigurationView {
                            id: v.id.clone(),
                            arn: v.arn.clone(),
                            name: v.name.clone(),
                            resource_configuration_type: v.resource_configuration_type.clone(),
                            status: v.status.clone(),
                            resource_gateway_id: v.resource_gateway_id.clone(),
                            port_ranges: v.port_ranges.clone(),
                            protocol: v.protocol.clone(),
                            created_at: dt_to_str(&v.created_at),
                            last_updated_at: dt_to_str(&v.last_updated_at),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            resource_gateways: state
                .resource_gateways
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ResourceGatewayView {
                            id: v.id.clone(),
                            arn: v.arn.clone(),
                            name: v.name.clone(),
                            status: v.status.clone(),
                            vpc_id: v.vpc_id.clone(),
                            subnet_ids: v.subnet_ids.clone(),
                            security_group_ids: v.security_group_ids.clone(),
                            ip_address_type: v.ip_address_type.clone(),
                            created_at: dt_to_str(&v.created_at),
                            last_updated_at: dt_to_str(&v.last_updated_at),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            sn_resource_associations: state
                .sn_resource_associations
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        SnResourceAssociationView {
                            id: v.id.clone(),
                            arn: v.arn.clone(),
                            service_network_identifier: v.service_network_identifier.clone(),
                            resource_configuration_identifier: v
                                .resource_configuration_identifier
                                .clone(),
                            service_network_id: v.service_network_id.clone(),
                            service_network_arn: v.service_network_arn.clone(),
                            service_network_name: v.service_network_name.clone(),
                            resource_configuration_id: v.resource_configuration_id.clone(),
                            resource_configuration_arn: v.resource_configuration_arn.clone(),
                            resource_configuration_name: v.resource_configuration_name.clone(),
                            status: v.status.clone(),
                            created_at: dt_to_str(&v.created_at),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            domain_verifications: state
                .domain_verifications
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        DomainVerificationView {
                            id: v.id.clone(),
                            arn: v.arn.clone(),
                            domain_name: v.domain_name.clone(),
                            status: v.status.clone(),
                            created_at: dt_to_str(&v.created_at),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<VpcLatticeStateView> for VpcLatticeState {
    fn from(view: VpcLatticeStateView) -> Self {
        VpcLatticeState {
            service_networks: view
                .service_networks
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        ServiceNetwork {
                            id: v.id,
                            name: v.name,
                            arn: v.arn,
                            auth_type: v.auth_type,
                            created_at: parse_dt(&v.created_at),
                            last_updated_at: parse_dt(&v.last_updated_at),
                            number_of_associated_services: v.number_of_associated_services,
                            number_of_associated_v_p_cs: v.number_of_associated_v_p_cs,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            access_log_subscriptions: view
                .access_log_subscriptions
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        AccessLogSubscription {
                            id: v.id,
                            arn: v.arn,
                            resource_arn: v.resource_arn,
                            resource_id: v.resource_id,
                            destination_arn: v.destination_arn,
                            created_at: parse_dt(&v.created_at),
                            last_updated_at: parse_dt(&v.last_updated_at),
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            sn_service_associations: view
                .sn_service_associations
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        ServiceNetworkServiceAssociation {
                            id: v.id,
                            arn: v.arn,
                            service_network_identifier: v.service_network_identifier,
                            service_identifier: v.service_identifier,
                            service_network_id: v.service_network_id,
                            service_network_arn: v.service_network_arn,
                            service_network_name: v.service_network_name,
                            service_id: v.service_id,
                            service_arn: v.service_arn,
                            service_name: v.service_name,
                            status: v.status,
                            created_at: parse_dt(&v.created_at),
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            sn_vpc_associations: view
                .sn_vpc_associations
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        ServiceNetworkVpcAssociation {
                            id: v.id,
                            arn: v.arn,
                            service_network_identifier: v.service_network_identifier,
                            vpc_identifier: v.vpc_identifier,
                            service_network_id: v.service_network_id,
                            service_network_arn: v.service_network_arn,
                            service_network_name: v.service_network_name,
                            vpc_id: v.vpc_id,
                            status: v.status,
                            security_group_ids: v.security_group_ids,
                            created_at: parse_dt(&v.created_at),
                            last_updated_at: parse_dt(&v.last_updated_at),
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            target_groups: view
                .target_groups
                .into_iter()
                .map(|(k, v)| {
                    let config = if v.config_port.is_some()
                        || v.config_protocol.is_some()
                        || v.config_vpc_identifier.is_some()
                    {
                        Some(TargetGroupConfig {
                            port: v.config_port,
                            protocol: v.config_protocol,
                            vpc_identifier: v.config_vpc_identifier,
                            ip_address_type: v.config_ip_address_type,
                            protocol_version: v.config_protocol_version,
                            lambda_event_structure_version: v.config_lambda_event_structure_version,
                        })
                    } else {
                        None
                    };
                    (
                        k,
                        TargetGroup {
                            id: v.id,
                            arn: v.arn,
                            name: v.name,
                            target_group_type: v.target_group_type,
                            config,
                            status: v.status,
                            created_at: parse_dt(&v.created_at),
                            last_updated_at: parse_dt(&v.last_updated_at),
                            tags: v.tags,
                            targets: v
                                .targets
                                .into_iter()
                                .map(|t| TargetEntry {
                                    id: t.id,
                                    port: t.port,
                                    status: t.status,
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
            services: view
                .services
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Service {
                            id: v.id,
                            name: v.name,
                            arn: v.arn,
                            auth_type: v.auth_type,
                            status: v.status,
                            created_at: parse_dt(&v.created_at),
                            last_updated_at: parse_dt(&v.last_updated_at),
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            auth_policies: view
                .auth_policies
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        AuthPolicy {
                            policy: v.policy,
                            state: v.state,
                            created_at: parse_dt(&v.created_at),
                            last_updated_at: parse_dt(&v.last_updated_at),
                        },
                    )
                })
                .collect(),
            resource_policies: view
                .resource_policies
                .into_iter()
                .map(|(k, v)| (k, ResourcePolicy { policy: v.policy }))
                .collect(),
            listeners: view
                .listeners
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Listener {
                            id: v.id,
                            arn: v.arn,
                            name: v.name,
                            service_id: v.service_id,
                            service_arn: v.service_arn,
                            port: v.port,
                            protocol: v.protocol,
                            default_action: ListenerDefaultAction {
                                fixed_response_status_code: v
                                    .default_action_fixed_response_status_code,
                                forward_target_groups: v.default_action_forward_target_groups,
                            },
                            created_at: parse_dt(&v.created_at),
                            last_updated_at: parse_dt(&v.last_updated_at),
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            rules: view
                .rules
                .into_iter()
                .map(|(k, v)| {
                    let rule_match = if v.match_method.is_some()
                        || v.match_path_exact.is_some()
                        || v.match_path_prefix.is_some()
                    {
                        Some(RuleMatchData {
                            method: v.match_method,
                            path_exact: v.match_path_exact,
                            path_prefix: v.match_path_prefix,
                        })
                    } else {
                        None
                    };
                    (
                        k,
                        Rule {
                            id: v.id,
                            arn: v.arn,
                            name: v.name,
                            listener_id: v.listener_id,
                            service_id: v.service_id,
                            priority: v.priority,
                            is_default: v.is_default,
                            action: RuleAction {
                                fixed_response_status_code: v.action_fixed_response_status_code,
                                forward_target_groups: v.action_forward_target_groups,
                            },
                            rule_match,
                            created_at: parse_dt(&v.created_at),
                            last_updated_at: parse_dt(&v.last_updated_at),
                        },
                    )
                })
                .collect(),
            resource_configurations: view
                .resource_configurations
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        ResourceConfiguration {
                            id: v.id,
                            arn: v.arn,
                            name: v.name,
                            resource_configuration_type: v.resource_configuration_type,
                            status: v.status,
                            resource_gateway_id: v.resource_gateway_id,
                            port_ranges: v.port_ranges,
                            protocol: v.protocol,
                            created_at: parse_dt(&v.created_at),
                            last_updated_at: parse_dt(&v.last_updated_at),
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            resource_gateways: view
                .resource_gateways
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        ResourceGateway {
                            id: v.id,
                            arn: v.arn,
                            name: v.name,
                            status: v.status,
                            vpc_id: v.vpc_id,
                            subnet_ids: v.subnet_ids,
                            security_group_ids: v.security_group_ids,
                            ip_address_type: v.ip_address_type,
                            created_at: parse_dt(&v.created_at),
                            last_updated_at: parse_dt(&v.last_updated_at),
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            sn_resource_associations: view
                .sn_resource_associations
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        ServiceNetworkResourceAssociation {
                            id: v.id,
                            arn: v.arn,
                            service_network_identifier: v.service_network_identifier,
                            resource_configuration_identifier: v.resource_configuration_identifier,
                            service_network_id: v.service_network_id,
                            service_network_arn: v.service_network_arn,
                            service_network_name: v.service_network_name,
                            resource_configuration_id: v.resource_configuration_id,
                            resource_configuration_arn: v.resource_configuration_arn,
                            resource_configuration_name: v.resource_configuration_name,
                            status: v.status,
                            created_at: parse_dt(&v.created_at),
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            domain_verifications: view
                .domain_verifications
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        DomainVerification {
                            id: v.id,
                            arn: v.arn,
                            domain_name: v.domain_name,
                            status: v.status,
                            created_at: parse_dt(&v.created_at),
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for VpcLatticeService {
    type StateView = VpcLatticeStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        VpcLatticeStateView::from(&*guard)
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
            *guard = VpcLatticeState::from(view);
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
            let new_state = VpcLatticeState::from(view);
            guard.service_networks.extend(new_state.service_networks);
            guard
                .access_log_subscriptions
                .extend(new_state.access_log_subscriptions);
            guard
                .sn_service_associations
                .extend(new_state.sn_service_associations);
            guard
                .sn_vpc_associations
                .extend(new_state.sn_vpc_associations);
            guard.target_groups.extend(new_state.target_groups);
            guard.services.extend(new_state.services);
            guard.auth_policies.extend(new_state.auth_policies);
            guard.resource_policies.extend(new_state.resource_policies);
            guard.listeners.extend(new_state.listeners);
            guard.rules.extend(new_state.rules);
            guard
                .resource_configurations
                .extend(new_state.resource_configurations);
            guard.resource_gateways.extend(new_state.resource_gateways);
            guard
                .sn_resource_associations
                .extend(new_state.sn_resource_associations);
            guard
                .domain_verifications
                .extend(new_state.domain_verifications);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
