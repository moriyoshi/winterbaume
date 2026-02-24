//! Serde-compatible view types for NetworkFirewall state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::NetworkFirewallService;
use crate::state::NetworkFirewallState;

/// Serializable view of a subnet mapping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubnetMappingView {
    pub subnet_id: String,
}

/// Serializable view of a firewall.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FirewallView {
    pub firewall_name: String,
    pub firewall_arn: String,
    pub firewall_id: String,
    pub firewall_policy_arn: String,
    pub vpc_id: String,
    #[serde(default)]
    pub subnet_mappings: Vec<SubnetMappingView>,
    pub delete_protection: bool,
    #[serde(default)]
    pub subnet_change_protection: bool,
    #[serde(default)]
    pub firewall_policy_change_protection: bool,
    #[serde(default)]
    pub availability_zone_change_protection: bool,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
    /// Encryption configuration stored as `{"key_id": "...", "type": "..."}`.
    #[serde(default)]
    pub encryption_configuration: Option<serde_json::Value>,
}

/// Serializable view of a rule group.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuleGroupView {
    pub rule_group_name: String,
    pub rule_group_arn: String,
    pub rule_group_id: String,
    pub rule_group_type: String,
    pub capacity: i32,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
    pub rule_group_body: Option<serde_json::Value>,
    pub rules: Option<String>,
    /// Encryption configuration stored as `{"key_id": "...", "type": "..."}`.
    #[serde(default)]
    pub encryption_configuration: Option<serde_json::Value>,
}

/// Serializable view of a firewall policy.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FirewallPolicyView {
    pub firewall_policy_name: String,
    pub firewall_policy_arn: String,
    pub firewall_policy_id: String,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
    pub firewall_policy_body: serde_json::Value,
    /// Encryption configuration stored as `{"key_id": "...", "type": "..."}`.
    #[serde(default)]
    pub encryption_configuration: Option<serde_json::Value>,
}

/// Serializable view of a resource policy.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcePolicyView {
    pub resource_arn: String,
    pub policy: String,
}

/// Serializable view of a TLS inspection configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TlsInspectionConfigurationView {
    pub name: String,
    pub arn: String,
    pub id: String,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
    pub body: serde_json::Value,
}

/// Serializable view of a VPC endpoint association.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VpcEndpointAssociationView {
    pub vpc_endpoint_association_arn: String,
    pub vpc_endpoint_association_id: String,
    pub firewall_arn: String,
    pub vpc_id: String,
    pub subnet_id: String,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
}

/// Serializable view of an availability zone mapping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityZoneMappingView {
    pub availability_zone: String,
}

/// Serializable view of a transit gateway attachment.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransitGatewayAttachmentView {
    pub transit_gateway_attachment_id: String,
    pub status: String,
}

/// Serializable view of a proxy.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NfwProxyView {
    pub proxy_name: String,
    pub proxy_arn: String,
    pub nat_gateway_id: String,
    pub proxy_configuration_arn: Option<String>,
    pub proxy_configuration_name: Option<String>,
    pub proxy_state: String,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
    pub body: serde_json::Value,
}

/// Serializable view of a proxy configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NfwProxyConfigurationView {
    pub proxy_configuration_name: String,
    pub proxy_configuration_arn: String,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
    pub body: serde_json::Value,
}

/// Serializable view of a proxy rule group.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NfwProxyRuleGroupView {
    pub proxy_rule_group_name: String,
    pub proxy_rule_group_arn: String,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
    pub body: serde_json::Value,
}

/// Serializable view of a flow operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FlowOperationView {
    pub flow_operation_id: String,
    pub firewall_arn: String,
    pub flow_operation_type: String,
    pub flow_operation_status: String,
    pub body: serde_json::Value,
}

/// Serializable view of an analysis report.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnalysisReportView {
    pub analysis_report_id: String,
    pub firewall_arn: String,
    pub analysis_type: String,
    pub status: String,
}

/// Serializable view of an encryption configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EncryptionConfigView {
    pub key_id: Option<String>,
    pub config_type: String,
}

/// Serializable view of the entire NetworkFirewall state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkFirewallStateView {
    #[serde(default)]
    pub firewalls: HashMap<String, FirewallView>,
    /// Logging configurations stored as raw JSON strings, keyed by firewall ARN.
    #[serde(default)]
    pub logging_configs: HashMap<String, String>,
    #[serde(default)]
    pub rule_groups: HashMap<String, RuleGroupView>,
    #[serde(default)]
    pub firewall_policies: HashMap<String, FirewallPolicyView>,
    #[serde(default)]
    pub resource_policies: HashMap<String, ResourcePolicyView>,
    #[serde(default)]
    pub tls_inspection_configurations: HashMap<String, TlsInspectionConfigurationView>,
    #[serde(default)]
    pub vpc_endpoint_associations: HashMap<String, VpcEndpointAssociationView>,
    #[serde(default)]
    pub availability_zone_mappings: HashMap<String, Vec<AvailabilityZoneMappingView>>,
    #[serde(default)]
    pub transit_gateway_attachments: HashMap<String, TransitGatewayAttachmentView>,
    #[serde(default)]
    pub proxies: HashMap<String, NfwProxyView>,
    #[serde(default)]
    pub proxy_configurations: HashMap<String, NfwProxyConfigurationView>,
    #[serde(default)]
    pub proxy_rule_groups: HashMap<String, NfwProxyRuleGroupView>,
    #[serde(default)]
    pub flow_operations: HashMap<String, FlowOperationView>,
    #[serde(default)]
    pub analysis_reports: HashMap<String, AnalysisReportView>,
    #[serde(default)]
    pub encryption_configs: HashMap<String, EncryptionConfigView>,
    #[serde(default)]
    pub analysis_settings: HashMap<String, Vec<String>>,
}

// --- From internal types to view types ---

impl From<&crate::types::Firewall> for FirewallView {
    fn from(fw: &crate::types::Firewall) -> Self {
        FirewallView {
            firewall_name: fw.firewall_name.clone(),
            firewall_arn: fw.firewall_arn.clone(),
            firewall_id: fw.firewall_id.clone(),
            firewall_policy_arn: fw.firewall_policy_arn.clone(),
            vpc_id: fw.vpc_id.clone(),
            subnet_mappings: fw
                .subnet_mappings
                .iter()
                .map(|sm| SubnetMappingView {
                    subnet_id: sm.subnet_id.clone(),
                })
                .collect(),
            delete_protection: fw.delete_protection,
            subnet_change_protection: fw.subnet_change_protection,
            firewall_policy_change_protection: fw.firewall_policy_change_protection,
            availability_zone_change_protection: fw.availability_zone_change_protection,
            description: fw.description.clone(),
            tags: fw.tags.clone(),
            encryption_configuration: fw.encryption_configuration.clone(),
        }
    }
}

impl From<&crate::types::RuleGroup> for RuleGroupView {
    fn from(rg: &crate::types::RuleGroup) -> Self {
        RuleGroupView {
            rule_group_name: rg.rule_group_name.clone(),
            rule_group_arn: rg.rule_group_arn.clone(),
            rule_group_id: rg.rule_group_id.clone(),
            rule_group_type: rg.rule_group_type.clone(),
            capacity: rg.capacity,
            description: rg.description.clone(),
            tags: rg.tags.clone(),
            rule_group_body: rg.rule_group_body.clone(),
            rules: rg.rules.clone(),
            encryption_configuration: rg.encryption_configuration.clone(),
        }
    }
}

impl From<&crate::types::FirewallPolicy> for FirewallPolicyView {
    fn from(fp: &crate::types::FirewallPolicy) -> Self {
        FirewallPolicyView {
            firewall_policy_name: fp.firewall_policy_name.clone(),
            firewall_policy_arn: fp.firewall_policy_arn.clone(),
            firewall_policy_id: fp.firewall_policy_id.clone(),
            description: fp.description.clone(),
            tags: fp.tags.clone(),
            firewall_policy_body: fp.firewall_policy_body.clone(),
            encryption_configuration: fp.encryption_configuration.clone(),
        }
    }
}

impl From<&crate::types::TlsInspectionConfiguration> for TlsInspectionConfigurationView {
    fn from(tls: &crate::types::TlsInspectionConfiguration) -> Self {
        TlsInspectionConfigurationView {
            name: tls.name.clone(),
            arn: tls.arn.clone(),
            id: tls.id.clone(),
            description: tls.description.clone(),
            tags: tls.tags.clone(),
            body: tls.body.clone(),
        }
    }
}

impl From<&crate::types::VpcEndpointAssociation> for VpcEndpointAssociationView {
    fn from(assoc: &crate::types::VpcEndpointAssociation) -> Self {
        VpcEndpointAssociationView {
            vpc_endpoint_association_arn: assoc.vpc_endpoint_association_arn.clone(),
            vpc_endpoint_association_id: assoc.vpc_endpoint_association_id.clone(),
            firewall_arn: assoc.firewall_arn.clone(),
            vpc_id: assoc.vpc_id.clone(),
            subnet_id: assoc.subnet_id.clone(),
            description: assoc.description.clone(),
            tags: assoc.tags.clone(),
        }
    }
}

impl From<&NetworkFirewallState> for NetworkFirewallStateView {
    fn from(state: &NetworkFirewallState) -> Self {
        NetworkFirewallStateView {
            firewalls: state
                .firewalls
                .iter()
                .map(|(k, v)| (k.clone(), FirewallView::from(v)))
                .collect(),
            logging_configs: state
                .logging_configs
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::to_string(v).unwrap_or_default()))
                .collect(),
            rule_groups: state
                .rule_groups
                .iter()
                .map(|(k, v)| (k.clone(), RuleGroupView::from(v)))
                .collect(),
            firewall_policies: state
                .firewall_policies
                .iter()
                .map(|(k, v)| (k.clone(), FirewallPolicyView::from(v)))
                .collect(),
            resource_policies: state
                .resource_policies
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ResourcePolicyView {
                            resource_arn: v.resource_arn.clone(),
                            policy: v.policy.clone(),
                        },
                    )
                })
                .collect(),
            tls_inspection_configurations: state
                .tls_inspection_configurations
                .iter()
                .map(|(k, v)| (k.clone(), TlsInspectionConfigurationView::from(v)))
                .collect(),
            vpc_endpoint_associations: state
                .vpc_endpoint_associations
                .iter()
                .map(|(k, v)| (k.clone(), VpcEndpointAssociationView::from(v)))
                .collect(),
            availability_zone_mappings: state
                .availability_zone_mappings
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v.iter()
                            .map(|m| AvailabilityZoneMappingView {
                                availability_zone: m.availability_zone.clone(),
                            })
                            .collect(),
                    )
                })
                .collect(),
            transit_gateway_attachments: state
                .transit_gateway_attachments
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        TransitGatewayAttachmentView {
                            transit_gateway_attachment_id: v.transit_gateway_attachment_id.clone(),
                            status: v.status.clone(),
                        },
                    )
                })
                .collect(),
            proxies: state
                .proxies
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        NfwProxyView {
                            proxy_name: v.proxy_name.clone(),
                            proxy_arn: v.proxy_arn.clone(),
                            nat_gateway_id: v.nat_gateway_id.clone(),
                            proxy_configuration_arn: v.proxy_configuration_arn.clone(),
                            proxy_configuration_name: v.proxy_configuration_name.clone(),
                            proxy_state: v.proxy_state.clone(),
                            tags: v.tags.clone(),
                            body: v.body.clone(),
                        },
                    )
                })
                .collect(),
            proxy_configurations: state
                .proxy_configurations
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        NfwProxyConfigurationView {
                            proxy_configuration_name: v.proxy_configuration_name.clone(),
                            proxy_configuration_arn: v.proxy_configuration_arn.clone(),
                            description: v.description.clone(),
                            tags: v.tags.clone(),
                            body: v.body.clone(),
                        },
                    )
                })
                .collect(),
            proxy_rule_groups: state
                .proxy_rule_groups
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        NfwProxyRuleGroupView {
                            proxy_rule_group_name: v.proxy_rule_group_name.clone(),
                            proxy_rule_group_arn: v.proxy_rule_group_arn.clone(),
                            description: v.description.clone(),
                            tags: v.tags.clone(),
                            body: v.body.clone(),
                        },
                    )
                })
                .collect(),
            flow_operations: state
                .flow_operations
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        FlowOperationView {
                            flow_operation_id: v.flow_operation_id.clone(),
                            firewall_arn: v.firewall_arn.clone(),
                            flow_operation_type: v.flow_operation_type.clone(),
                            flow_operation_status: v.flow_operation_status.clone(),
                            body: v.body.clone(),
                        },
                    )
                })
                .collect(),
            analysis_reports: state
                .analysis_reports
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        AnalysisReportView {
                            analysis_report_id: v.analysis_report_id.clone(),
                            firewall_arn: v.firewall_arn.clone(),
                            analysis_type: v.analysis_type.clone(),
                            status: v.status.clone(),
                        },
                    )
                })
                .collect(),
            encryption_configs: state
                .encryption_configs
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        EncryptionConfigView {
                            key_id: v.key_id.clone(),
                            config_type: v.config_type.clone(),
                        },
                    )
                })
                .collect(),
            analysis_settings: state.analysis_settings.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<FirewallView> for crate::types::Firewall {
    fn from(v: FirewallView) -> Self {
        crate::types::Firewall {
            firewall_name: v.firewall_name,
            firewall_arn: v.firewall_arn,
            firewall_id: v.firewall_id,
            firewall_policy_arn: v.firewall_policy_arn,
            vpc_id: v.vpc_id,
            subnet_mappings: v
                .subnet_mappings
                .into_iter()
                .map(|sm| crate::types::SubnetMapping {
                    subnet_id: sm.subnet_id,
                })
                .collect(),
            delete_protection: v.delete_protection,
            subnet_change_protection: v.subnet_change_protection,
            firewall_policy_change_protection: v.firewall_policy_change_protection,
            availability_zone_change_protection: v.availability_zone_change_protection,
            description: v.description,
            tags: v.tags,
            encryption_configuration: v.encryption_configuration,
        }
    }
}

impl From<RuleGroupView> for crate::types::RuleGroup {
    fn from(v: RuleGroupView) -> Self {
        crate::types::RuleGroup {
            rule_group_name: v.rule_group_name,
            rule_group_arn: v.rule_group_arn,
            rule_group_id: v.rule_group_id,
            rule_group_type: v.rule_group_type,
            capacity: v.capacity,
            description: v.description,
            tags: v.tags,
            rule_group_body: v.rule_group_body,
            rules: v.rules,
            encryption_configuration: v.encryption_configuration,
        }
    }
}

impl From<FirewallPolicyView> for crate::types::FirewallPolicy {
    fn from(v: FirewallPolicyView) -> Self {
        crate::types::FirewallPolicy {
            firewall_policy_name: v.firewall_policy_name,
            firewall_policy_arn: v.firewall_policy_arn,
            firewall_policy_id: v.firewall_policy_id,
            description: v.description,
            tags: v.tags,
            firewall_policy_body: v.firewall_policy_body,
            encryption_configuration: v.encryption_configuration,
        }
    }
}

impl From<TlsInspectionConfigurationView> for crate::types::TlsInspectionConfiguration {
    fn from(v: TlsInspectionConfigurationView) -> Self {
        crate::types::TlsInspectionConfiguration {
            name: v.name,
            arn: v.arn,
            id: v.id,
            description: v.description,
            tags: v.tags,
            body: v.body,
        }
    }
}

impl From<VpcEndpointAssociationView> for crate::types::VpcEndpointAssociation {
    fn from(v: VpcEndpointAssociationView) -> Self {
        crate::types::VpcEndpointAssociation {
            vpc_endpoint_association_arn: v.vpc_endpoint_association_arn,
            vpc_endpoint_association_id: v.vpc_endpoint_association_id,
            firewall_arn: v.firewall_arn,
            vpc_id: v.vpc_id,
            subnet_id: v.subnet_id,
            description: v.description,
            tags: v.tags,
        }
    }
}

impl From<NetworkFirewallStateView> for NetworkFirewallState {
    fn from(view: NetworkFirewallStateView) -> Self {
        NetworkFirewallState {
            firewalls: view
                .firewalls
                .into_iter()
                .map(|(k, v)| (k, crate::types::Firewall::from(v)))
                .collect(),
            logging_configs: view
                .logging_configs
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        serde_json::from_str(&v).unwrap_or(serde_json::Value::Null),
                    )
                })
                .collect(),
            rule_groups: view
                .rule_groups
                .into_iter()
                .map(|(k, v)| (k, crate::types::RuleGroup::from(v)))
                .collect(),
            firewall_policies: view
                .firewall_policies
                .into_iter()
                .map(|(k, v)| (k, crate::types::FirewallPolicy::from(v)))
                .collect(),
            resource_policies: view
                .resource_policies
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        crate::types::ResourcePolicy {
                            resource_arn: v.resource_arn,
                            policy: v.policy,
                        },
                    )
                })
                .collect(),
            tls_inspection_configurations: view
                .tls_inspection_configurations
                .into_iter()
                .map(|(k, v)| (k, crate::types::TlsInspectionConfiguration::from(v)))
                .collect(),
            vpc_endpoint_associations: view
                .vpc_endpoint_associations
                .into_iter()
                .map(|(k, v)| (k, crate::types::VpcEndpointAssociation::from(v)))
                .collect(),
            availability_zone_mappings: view
                .availability_zone_mappings
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        v.into_iter()
                            .map(|m| crate::types::AvailabilityZoneMapping {
                                availability_zone: m.availability_zone,
                            })
                            .collect(),
                    )
                })
                .collect(),
            transit_gateway_attachments: view
                .transit_gateway_attachments
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        crate::types::TransitGatewayAttachment {
                            transit_gateway_attachment_id: v.transit_gateway_attachment_id,
                            status: v.status,
                        },
                    )
                })
                .collect(),
            proxies: view
                .proxies
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        crate::types::NfwProxy {
                            proxy_name: v.proxy_name,
                            proxy_arn: v.proxy_arn,
                            nat_gateway_id: v.nat_gateway_id,
                            proxy_configuration_arn: v.proxy_configuration_arn,
                            proxy_configuration_name: v.proxy_configuration_name,
                            proxy_state: v.proxy_state,
                            tags: v.tags,
                            body: v.body,
                        },
                    )
                })
                .collect(),
            proxy_configurations: view
                .proxy_configurations
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        crate::types::NfwProxyConfiguration {
                            proxy_configuration_name: v.proxy_configuration_name,
                            proxy_configuration_arn: v.proxy_configuration_arn,
                            description: v.description,
                            tags: v.tags,
                            body: v.body,
                        },
                    )
                })
                .collect(),
            proxy_rule_groups: view
                .proxy_rule_groups
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        crate::types::NfwProxyRuleGroup {
                            proxy_rule_group_name: v.proxy_rule_group_name,
                            proxy_rule_group_arn: v.proxy_rule_group_arn,
                            description: v.description,
                            tags: v.tags,
                            body: v.body,
                        },
                    )
                })
                .collect(),
            flow_operations: view
                .flow_operations
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        crate::types::FlowOperation {
                            flow_operation_id: v.flow_operation_id,
                            firewall_arn: v.firewall_arn,
                            flow_operation_type: v.flow_operation_type,
                            flow_operation_status: v.flow_operation_status,
                            body: v.body,
                        },
                    )
                })
                .collect(),
            analysis_reports: view
                .analysis_reports
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        crate::types::AnalysisReport {
                            analysis_report_id: v.analysis_report_id,
                            firewall_arn: v.firewall_arn,
                            analysis_type: v.analysis_type,
                            status: v.status,
                        },
                    )
                })
                .collect(),
            encryption_configs: view
                .encryption_configs
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        crate::types::EncryptionConfig {
                            key_id: v.key_id,
                            config_type: v.config_type,
                        },
                    )
                })
                .collect(),
            analysis_settings: view.analysis_settings,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for NetworkFirewallService {
    type StateView = NetworkFirewallStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        NetworkFirewallStateView::from(&*guard)
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
            *guard = NetworkFirewallState::from(view);
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
            for (arn, fw_view) in view.firewalls {
                guard
                    .firewalls
                    .insert(arn, crate::types::Firewall::from(fw_view));
            }
            for (arn, cfg_str) in view.logging_configs {
                if let Ok(v) = serde_json::from_str(&cfg_str) {
                    guard.logging_configs.insert(arn, v);
                }
            }
            for (arn, rg_view) in view.rule_groups {
                guard
                    .rule_groups
                    .insert(arn, crate::types::RuleGroup::from(rg_view));
            }
            for (arn, fp_view) in view.firewall_policies {
                guard
                    .firewall_policies
                    .insert(arn, crate::types::FirewallPolicy::from(fp_view));
            }
            for (arn, rp_view) in view.resource_policies {
                guard.resource_policies.insert(
                    arn,
                    crate::types::ResourcePolicy {
                        resource_arn: rp_view.resource_arn,
                        policy: rp_view.policy,
                    },
                );
            }
            for (arn, tls_view) in view.tls_inspection_configurations {
                guard.tls_inspection_configurations.insert(
                    arn,
                    crate::types::TlsInspectionConfiguration::from(tls_view),
                );
            }
            for (arn, assoc_view) in view.vpc_endpoint_associations {
                guard
                    .vpc_endpoint_associations
                    .insert(arn, crate::types::VpcEndpointAssociation::from(assoc_view));
            }
            for (arn, az_views) in view.availability_zone_mappings {
                guard.availability_zone_mappings.insert(
                    arn,
                    az_views
                        .into_iter()
                        .map(|m| crate::types::AvailabilityZoneMapping {
                            availability_zone: m.availability_zone,
                        })
                        .collect(),
                );
            }
            for (id, att_view) in view.transit_gateway_attachments {
                guard.transit_gateway_attachments.insert(
                    id,
                    crate::types::TransitGatewayAttachment {
                        transit_gateway_attachment_id: att_view.transit_gateway_attachment_id,
                        status: att_view.status,
                    },
                );
            }
            for (arn, proxy_view) in view.proxies {
                guard.proxies.insert(
                    arn,
                    crate::types::NfwProxy {
                        proxy_name: proxy_view.proxy_name,
                        proxy_arn: proxy_view.proxy_arn,
                        nat_gateway_id: proxy_view.nat_gateway_id,
                        proxy_configuration_arn: proxy_view.proxy_configuration_arn,
                        proxy_configuration_name: proxy_view.proxy_configuration_name,
                        proxy_state: proxy_view.proxy_state,
                        tags: proxy_view.tags,
                        body: proxy_view.body,
                    },
                );
            }
            for (arn, config_view) in view.proxy_configurations {
                guard.proxy_configurations.insert(
                    arn,
                    crate::types::NfwProxyConfiguration {
                        proxy_configuration_name: config_view.proxy_configuration_name,
                        proxy_configuration_arn: config_view.proxy_configuration_arn,
                        description: config_view.description,
                        tags: config_view.tags,
                        body: config_view.body,
                    },
                );
            }
            for (arn, group_view) in view.proxy_rule_groups {
                guard.proxy_rule_groups.insert(
                    arn,
                    crate::types::NfwProxyRuleGroup {
                        proxy_rule_group_name: group_view.proxy_rule_group_name,
                        proxy_rule_group_arn: group_view.proxy_rule_group_arn,
                        description: group_view.description,
                        tags: group_view.tags,
                        body: group_view.body,
                    },
                );
            }
            for (id, op_view) in view.flow_operations {
                guard.flow_operations.insert(
                    id,
                    crate::types::FlowOperation {
                        flow_operation_id: op_view.flow_operation_id,
                        firewall_arn: op_view.firewall_arn,
                        flow_operation_type: op_view.flow_operation_type,
                        flow_operation_status: op_view.flow_operation_status,
                        body: op_view.body,
                    },
                );
            }
            for (id, report_view) in view.analysis_reports {
                guard.analysis_reports.insert(
                    id,
                    crate::types::AnalysisReport {
                        analysis_report_id: report_view.analysis_report_id,
                        firewall_arn: report_view.firewall_arn,
                        analysis_type: report_view.analysis_type,
                        status: report_view.status,
                    },
                );
            }
            for (arn, enc_view) in view.encryption_configs {
                guard.encryption_configs.insert(
                    arn,
                    crate::types::EncryptionConfig {
                        key_id: enc_view.key_id,
                        config_type: enc_view.config_type,
                    },
                );
            }
            for (arn, types) in view.analysis_settings {
                guard.analysis_settings.insert(arn, types);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
