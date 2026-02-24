//! Serde-compatible view types for SecurityHub state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::SecurityHubService;
use crate::state::{AssocKey, SecurityHubState};
use crate::types::{
    ActionTargetInfo, AggregatorV2Info, AutomationRuleInfo, ConfigPolicyAssociation,
    ConfigPolicyInfo, ConnectorV2Info, Finding, FindingAggregatorInfo, FindingV2, HubInfo,
    HubV2Info, InsightInfo, Member, OrgAdminAccount, OrganizationConfig, ProductInfo,
    ProductSubscription, SecurityControlInfo, StandardInfo, StandardsControlAssociationInfo,
    StandardsControlInfo, StandardsSubscriptionInfo, TicketV2Info,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityHubStateView {
    /// Findings stored as raw JSON values.
    #[serde(default)]
    pub findings: Vec<FindingView>,
    #[serde(default)]
    pub hub: HubInfoView,
    #[serde(default)]
    pub members: HashMap<String, MemberView>,
    #[serde(default)]
    pub action_targets: HashMap<String, ActionTargetView>,
    #[serde(default)]
    pub finding_aggregators: HashMap<String, FindingAggregatorView>,
    #[serde(default)]
    pub enabled_standards: Vec<StandardsSubscriptionView>,
    #[serde(default)]
    pub org_config: OrgConfigView,
    pub org_admin_account: Option<OrgAdminAccountView>,
    #[serde(default)]
    pub automation_rules: HashMap<String, AutomationRuleView>,
    #[serde(default)]
    pub config_policies: HashMap<String, ConfigPolicyView>,
    #[serde(default)]
    pub connectors: HashMap<String, ConnectorV2View>,
    #[serde(default)]
    pub insights: HashMap<String, InsightView>,
    #[serde(default)]
    pub hub_v2: HubV2InfoView,
    #[serde(default)]
    pub aggregators_v2: HashMap<String, AggregatorV2View>,
    #[serde(default)]
    pub product_subscriptions: HashMap<String, ProductSubscriptionView>,
    #[serde(default)]
    pub policy_associations: HashMap<AssocKey, ConfigPolicyAssociationView>,
    #[serde(default)]
    pub auto_enable_controls: bool,
    #[serde(default)]
    pub control_finding_generator: String,
    #[serde(default)]
    pub security_controls: HashMap<String, SecurityControlView>,
    #[serde(default)]
    pub standards_control_associations: HashMap<String, StandardsControlAssociationView>,
    #[serde(default)]
    pub standards_controls: HashMap<String, StandardsControlView>,
    #[serde(default)]
    pub standards: HashMap<String, StandardView>,
    #[serde(default)]
    pub products: HashMap<String, ProductView>,
    #[serde(default)]
    pub tickets_v2: HashMap<String, TicketV2View>,
    #[serde(default)]
    pub findings_v2: HashMap<String, FindingV2View>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FindingView {
    pub id: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HubInfoView {
    pub enabled: bool,
    pub subscribed_at: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberView {
    pub account_id: String,
    pub email: String,
    pub member_status: String,
    pub invited_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionTargetView {
    pub arn: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindingAggregatorView {
    pub arn: String,
    pub region_linking_mode: String,
    #[serde(default)]
    pub regions: Vec<String>,
    pub finding_aggregation_region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StandardsSubscriptionView {
    pub standards_subscription_arn: String,
    pub standards_arn: String,
    pub standards_status: String,
    #[serde(default)]
    pub standards_input: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrgConfigView {
    pub auto_enable: bool,
    pub member_account_limit_reached: bool,
    pub auto_enable_standards: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgAdminAccountView {
    pub account_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRuleView {
    pub rule_arn: String,
    pub rule_id: Option<String>,
    pub rule_name: String,
    pub rule_order: i32,
    pub rule_status: String,
    pub description: Option<String>,
    pub is_terminal: bool,
    pub created_at: String,
    pub updated_at: String,
    pub raw: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigPolicyView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub configuration_policy: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorV2View {
    pub connector_id: String,
    pub connector_arn: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub last_updated_at: String,
    pub connector_status: String,
    pub provider_name: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub raw: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsightView {
    pub arn: String,
    pub name: String,
    pub group_by_attribute: String,
    pub filters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HubV2InfoView {
    pub enabled: bool,
    pub hub_v2_arn: Option<String>,
    pub subscribed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatorV2View {
    pub arn: String,
    pub aggregation_region: String,
    pub region_linking_mode: String,
    #[serde(default)]
    pub linked_regions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSubscriptionView {
    pub product_subscription_arn: String,
    pub product_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigPolicyAssociationView {
    pub configuration_policy_id: String,
    pub target_id: String,
    pub target_type: String,
    pub association_type: String,
    pub association_status: String,
    pub association_status_message: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityControlView {
    pub security_control_id: String,
    pub security_control_arn: String,
    pub title: String,
    pub description: String,
    pub remediation_url: String,
    pub severity_rating: String,
    pub security_control_status: String,
    pub current_region_availability: String,
    #[serde(default)]
    pub parameters: HashMap<String, serde_json::Value>,
    pub last_update_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardsControlAssociationView {
    pub security_control_id: String,
    pub standards_arn: String,
    pub association_status: String,
    pub updated_reason: Option<String>,
    pub security_control_arn: Option<String>,
    pub standards_control_title: Option<String>,
    pub standards_control_description: Option<String>,
    #[serde(default)]
    pub standards_control_arns: Vec<String>,
    #[serde(default)]
    pub related_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardsControlView {
    pub standards_control_arn: String,
    pub control_status: String,
    pub disabled_reason: Option<String>,
    pub control_status_updated_at: Option<String>,
    pub control_id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub remediation_url: Option<String>,
    pub severity_rating: Option<String>,
    #[serde(default)]
    pub related_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardView {
    pub standards_arn: String,
    pub name: String,
    pub description: String,
    pub enabled_by_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductView {
    pub product_arn: String,
    pub product_name: String,
    pub company_name: String,
    pub description: String,
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(default)]
    pub integration_types: Vec<String>,
    pub marketplace_url: Option<String>,
    pub activation_url: Option<String>,
    pub product_subscription_resource_policy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketV2View {
    pub ticket_id: String,
    pub ticket_src_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindingV2View {
    pub metadata_uid: String,
    pub data: serde_json::Value,
}

impl From<&SecurityHubState> for SecurityHubStateView {
    fn from(state: &SecurityHubState) -> Self {
        SecurityHubStateView {
            findings: state
                .findings
                .iter()
                .map(|f| FindingView {
                    id: f.id.clone(),
                    data: f.data.clone(),
                })
                .collect(),
            hub: HubInfoView {
                enabled: state.hub.enabled,
                subscribed_at: state.hub.subscribed_at.clone(),
                tags: state.hub.tags.clone(),
            },
            members: state
                .members
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        MemberView {
                            account_id: v.account_id.clone(),
                            email: v.email.clone(),
                            member_status: v.member_status.clone(),
                            invited_at: v.invited_at.clone(),
                            updated_at: v.updated_at.clone(),
                        },
                    )
                })
                .collect(),
            action_targets: state
                .action_targets
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ActionTargetView {
                            arn: v.arn.clone(),
                            name: v.name.clone(),
                            description: v.description.clone(),
                        },
                    )
                })
                .collect(),
            finding_aggregators: state
                .finding_aggregators
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        FindingAggregatorView {
                            arn: v.arn.clone(),
                            region_linking_mode: v.region_linking_mode.clone(),
                            regions: v.regions.clone(),
                            finding_aggregation_region: v.finding_aggregation_region.clone(),
                        },
                    )
                })
                .collect(),
            enabled_standards: state
                .enabled_standards
                .iter()
                .map(|s| StandardsSubscriptionView {
                    standards_subscription_arn: s.standards_subscription_arn.clone(),
                    standards_arn: s.standards_arn.clone(),
                    standards_status: s.standards_status.clone(),
                    standards_input: s.standards_input.clone(),
                })
                .collect(),
            org_config: OrgConfigView {
                auto_enable: state.org_config.auto_enable,
                member_account_limit_reached: state.org_config.member_account_limit_reached,
                auto_enable_standards: state.org_config.auto_enable_standards.clone(),
            },
            org_admin_account: state
                .org_admin_account
                .as_ref()
                .map(|a| OrgAdminAccountView {
                    account_id: a.account_id.clone(),
                    status: a.status.clone(),
                }),
            automation_rules: state
                .automation_rules
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        AutomationRuleView {
                            rule_arn: v.rule_arn.clone(),
                            rule_id: v.rule_id.clone(),
                            rule_name: v.rule_name.clone(),
                            rule_order: v.rule_order,
                            rule_status: v.rule_status.clone(),
                            description: v.description.clone(),
                            is_terminal: v.is_terminal,
                            created_at: v.created_at.clone(),
                            updated_at: v.updated_at.clone(),
                            raw: v.raw.clone(),
                        },
                    )
                })
                .collect(),
            config_policies: state
                .config_policies
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ConfigPolicyView {
                            id: v.id.clone(),
                            arn: v.arn.clone(),
                            name: v.name.clone(),
                            description: v.description.clone(),
                            created_at: v.created_at.clone(),
                            updated_at: v.updated_at.clone(),
                            configuration_policy: v.configuration_policy.clone(),
                        },
                    )
                })
                .collect(),
            connectors: state
                .connectors
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ConnectorV2View {
                            connector_id: v.connector_id.clone(),
                            connector_arn: v.connector_arn.clone(),
                            name: v.name.clone(),
                            description: v.description.clone(),
                            created_at: v.created_at.clone(),
                            last_updated_at: v.last_updated_at.clone(),
                            connector_status: v.connector_status.clone(),
                            provider_name: v.provider_name.clone(),
                            tags: v.tags.clone(),
                            raw: v.raw.clone(),
                        },
                    )
                })
                .collect(),
            insights: state
                .insights
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        InsightView {
                            arn: v.arn.clone(),
                            name: v.name.clone(),
                            group_by_attribute: v.group_by_attribute.clone(),
                            filters: v.filters.clone(),
                        },
                    )
                })
                .collect(),
            hub_v2: HubV2InfoView {
                enabled: state.hub_v2.enabled,
                hub_v2_arn: state.hub_v2.hub_v2_arn.clone(),
                subscribed_at: state.hub_v2.subscribed_at.clone(),
            },
            aggregators_v2: state
                .aggregators_v2
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        AggregatorV2View {
                            arn: v.arn.clone(),
                            aggregation_region: v.aggregation_region.clone(),
                            region_linking_mode: v.region_linking_mode.clone(),
                            linked_regions: v.linked_regions.clone(),
                        },
                    )
                })
                .collect(),
            product_subscriptions: state
                .product_subscriptions
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ProductSubscriptionView {
                            product_subscription_arn: v.product_subscription_arn.clone(),
                            product_arn: v.product_arn.clone(),
                        },
                    )
                })
                .collect(),
            policy_associations: state
                .policy_associations
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ConfigPolicyAssociationView {
                            configuration_policy_id: v.configuration_policy_id.clone(),
                            target_id: v.target_id.clone(),
                            target_type: v.target_type.clone(),
                            association_type: v.association_type.clone(),
                            association_status: v.association_status.clone(),
                            association_status_message: v.association_status_message.clone(),
                            updated_at: v.updated_at.clone(),
                        },
                    )
                })
                .collect(),
            auto_enable_controls: state.auto_enable_controls,
            control_finding_generator: state.control_finding_generator.clone(),
            security_controls: state
                .security_controls
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        SecurityControlView {
                            security_control_id: v.security_control_id.clone(),
                            security_control_arn: v.security_control_arn.clone(),
                            title: v.title.clone(),
                            description: v.description.clone(),
                            remediation_url: v.remediation_url.clone(),
                            severity_rating: v.severity_rating.clone(),
                            security_control_status: v.security_control_status.clone(),
                            current_region_availability: v.current_region_availability.clone(),
                            parameters: v.parameters.clone(),
                            last_update_reason: v.last_update_reason.clone(),
                        },
                    )
                })
                .collect(),
            standards_control_associations: state
                .standards_control_associations
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        StandardsControlAssociationView {
                            security_control_id: v.security_control_id.clone(),
                            standards_arn: v.standards_arn.clone(),
                            association_status: v.association_status.clone(),
                            updated_reason: v.updated_reason.clone(),
                            security_control_arn: v.security_control_arn.clone(),
                            standards_control_title: v.standards_control_title.clone(),
                            standards_control_description: v.standards_control_description.clone(),
                            standards_control_arns: v.standards_control_arns.clone(),
                            related_requirements: v.related_requirements.clone(),
                        },
                    )
                })
                .collect(),
            standards_controls: state
                .standards_controls
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        StandardsControlView {
                            standards_control_arn: v.standards_control_arn.clone(),
                            control_status: v.control_status.clone(),
                            disabled_reason: v.disabled_reason.clone(),
                            control_status_updated_at: v.control_status_updated_at.clone(),
                            control_id: v.control_id.clone(),
                            title: v.title.clone(),
                            description: v.description.clone(),
                            remediation_url: v.remediation_url.clone(),
                            severity_rating: v.severity_rating.clone(),
                            related_requirements: v.related_requirements.clone(),
                        },
                    )
                })
                .collect(),
            standards: state
                .standards
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        StandardView {
                            standards_arn: v.standards_arn.clone(),
                            name: v.name.clone(),
                            description: v.description.clone(),
                            enabled_by_default: v.enabled_by_default,
                        },
                    )
                })
                .collect(),
            products: state
                .products
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ProductView {
                            product_arn: v.product_arn.clone(),
                            product_name: v.product_name.clone(),
                            company_name: v.company_name.clone(),
                            description: v.description.clone(),
                            categories: v.categories.clone(),
                            integration_types: v.integration_types.clone(),
                            marketplace_url: v.marketplace_url.clone(),
                            activation_url: v.activation_url.clone(),
                            product_subscription_resource_policy: v
                                .product_subscription_resource_policy
                                .clone(),
                        },
                    )
                })
                .collect(),
            tickets_v2: state
                .tickets_v2
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        TicketV2View {
                            ticket_id: v.ticket_id.clone(),
                            ticket_src_url: v.ticket_src_url.clone(),
                        },
                    )
                })
                .collect(),
            findings_v2: state
                .findings_v2
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        FindingV2View {
                            metadata_uid: v.metadata_uid.clone(),
                            data: v.data.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

impl From<SecurityHubStateView> for SecurityHubState {
    fn from(view: SecurityHubStateView) -> Self {
        SecurityHubState {
            findings: view
                .findings
                .into_iter()
                .map(|f| Finding {
                    id: f.id,
                    data: f.data,
                })
                .collect(),
            hub: HubInfo {
                enabled: view.hub.enabled,
                subscribed_at: view.hub.subscribed_at,
                tags: view.hub.tags,
            },
            members: view
                .members
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Member {
                            account_id: v.account_id,
                            email: v.email,
                            member_status: v.member_status,
                            invited_at: v.invited_at,
                            updated_at: v.updated_at,
                        },
                    )
                })
                .collect(),
            action_targets: view
                .action_targets
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        ActionTargetInfo {
                            arn: v.arn,
                            name: v.name,
                            description: v.description,
                        },
                    )
                })
                .collect(),
            finding_aggregators: view
                .finding_aggregators
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        FindingAggregatorInfo {
                            arn: v.arn,
                            region_linking_mode: v.region_linking_mode,
                            regions: v.regions,
                            finding_aggregation_region: v.finding_aggregation_region,
                        },
                    )
                })
                .collect(),
            enabled_standards: view
                .enabled_standards
                .into_iter()
                .map(|s| StandardsSubscriptionInfo {
                    standards_subscription_arn: s.standards_subscription_arn,
                    standards_arn: s.standards_arn,
                    standards_status: s.standards_status,
                    standards_input: s.standards_input,
                })
                .collect(),
            org_config: OrganizationConfig {
                auto_enable: view.org_config.auto_enable,
                member_account_limit_reached: view.org_config.member_account_limit_reached,
                auto_enable_standards: view.org_config.auto_enable_standards,
            },
            org_admin_account: view.org_admin_account.map(|a| OrgAdminAccount {
                account_id: a.account_id,
                status: a.status,
            }),
            automation_rules: view
                .automation_rules
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        AutomationRuleInfo {
                            rule_arn: v.rule_arn,
                            rule_id: v.rule_id,
                            rule_name: v.rule_name,
                            rule_order: v.rule_order,
                            rule_status: v.rule_status,
                            description: v.description,
                            is_terminal: v.is_terminal,
                            created_at: v.created_at,
                            updated_at: v.updated_at,
                            raw: v.raw,
                        },
                    )
                })
                .collect(),
            config_policies: view
                .config_policies
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        ConfigPolicyInfo {
                            id: v.id,
                            arn: v.arn,
                            name: v.name,
                            description: v.description,
                            created_at: v.created_at,
                            updated_at: v.updated_at,
                            configuration_policy: v.configuration_policy,
                        },
                    )
                })
                .collect(),
            connectors: view
                .connectors
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        ConnectorV2Info {
                            connector_id: v.connector_id,
                            connector_arn: v.connector_arn,
                            name: v.name,
                            description: v.description,
                            created_at: v.created_at,
                            last_updated_at: v.last_updated_at,
                            connector_status: v.connector_status,
                            provider_name: v.provider_name,
                            tags: v.tags,
                            raw: v.raw,
                        },
                    )
                })
                .collect(),
            insights: view
                .insights
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        InsightInfo {
                            arn: v.arn,
                            name: v.name,
                            group_by_attribute: v.group_by_attribute,
                            filters: v.filters,
                        },
                    )
                })
                .collect(),
            hub_v2: HubV2Info {
                enabled: view.hub_v2.enabled,
                hub_v2_arn: view.hub_v2.hub_v2_arn,
                subscribed_at: view.hub_v2.subscribed_at,
            },
            aggregators_v2: view
                .aggregators_v2
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        AggregatorV2Info {
                            arn: v.arn,
                            aggregation_region: v.aggregation_region,
                            region_linking_mode: v.region_linking_mode,
                            linked_regions: v.linked_regions,
                        },
                    )
                })
                .collect(),
            product_subscriptions: view
                .product_subscriptions
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        ProductSubscription {
                            product_subscription_arn: v.product_subscription_arn,
                            product_arn: v.product_arn,
                        },
                    )
                })
                .collect(),
            policy_associations: view
                .policy_associations
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        ConfigPolicyAssociation {
                            configuration_policy_id: v.configuration_policy_id,
                            target_id: v.target_id,
                            target_type: v.target_type,
                            association_type: v.association_type,
                            association_status: v.association_status,
                            association_status_message: v.association_status_message,
                            updated_at: v.updated_at,
                        },
                    )
                })
                .collect(),
            auto_enable_controls: view.auto_enable_controls,
            control_finding_generator: view.control_finding_generator,
            security_controls: view
                .security_controls
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        SecurityControlInfo {
                            security_control_id: v.security_control_id,
                            security_control_arn: v.security_control_arn,
                            title: v.title,
                            description: v.description,
                            remediation_url: v.remediation_url,
                            severity_rating: v.severity_rating,
                            security_control_status: v.security_control_status,
                            current_region_availability: v.current_region_availability,
                            parameters: v.parameters,
                            last_update_reason: v.last_update_reason,
                        },
                    )
                })
                .collect(),
            standards_control_associations: view
                .standards_control_associations
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        StandardsControlAssociationInfo {
                            security_control_id: v.security_control_id,
                            standards_arn: v.standards_arn,
                            association_status: v.association_status,
                            updated_reason: v.updated_reason,
                            security_control_arn: v.security_control_arn,
                            standards_control_title: v.standards_control_title,
                            standards_control_description: v.standards_control_description,
                            standards_control_arns: v.standards_control_arns,
                            related_requirements: v.related_requirements,
                        },
                    )
                })
                .collect(),
            standards_controls: view
                .standards_controls
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        StandardsControlInfo {
                            standards_control_arn: v.standards_control_arn,
                            control_status: v.control_status,
                            disabled_reason: v.disabled_reason,
                            control_status_updated_at: v.control_status_updated_at,
                            control_id: v.control_id,
                            title: v.title,
                            description: v.description,
                            remediation_url: v.remediation_url,
                            severity_rating: v.severity_rating,
                            related_requirements: v.related_requirements,
                        },
                    )
                })
                .collect(),
            standards: view
                .standards
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        StandardInfo {
                            standards_arn: v.standards_arn,
                            name: v.name,
                            description: v.description,
                            enabled_by_default: v.enabled_by_default,
                        },
                    )
                })
                .collect(),
            products: view
                .products
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        ProductInfo {
                            product_arn: v.product_arn,
                            product_name: v.product_name,
                            company_name: v.company_name,
                            description: v.description,
                            categories: v.categories,
                            integration_types: v.integration_types,
                            marketplace_url: v.marketplace_url,
                            activation_url: v.activation_url,
                            product_subscription_resource_policy: v
                                .product_subscription_resource_policy,
                        },
                    )
                })
                .collect(),
            tickets_v2: view
                .tickets_v2
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        TicketV2Info {
                            ticket_id: v.ticket_id,
                            ticket_src_url: v.ticket_src_url,
                        },
                    )
                })
                .collect(),
            findings_v2: view
                .findings_v2
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        FindingV2 {
                            metadata_uid: v.metadata_uid,
                            data: v.data,
                        },
                    )
                })
                .collect(),
        }
    }
}

impl StatefulService for SecurityHubService {
    type StateView = SecurityHubStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        SecurityHubStateView::from(&*guard)
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
            *guard = SecurityHubState::from(view);
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
            for f in view.findings {
                if let Some(existing) = guard.findings.iter_mut().find(|e| e.id == f.id) {
                    existing.data = f.data;
                } else {
                    guard.findings.push(Finding {
                        id: f.id,
                        data: f.data,
                    });
                }
            }
            for (k, v) in view.members {
                guard.members.insert(
                    k,
                    Member {
                        account_id: v.account_id,
                        email: v.email,
                        member_status: v.member_status,
                        invited_at: v.invited_at,
                        updated_at: v.updated_at,
                    },
                );
            }
            for (k, v) in view.action_targets {
                guard.action_targets.insert(
                    k,
                    ActionTargetInfo {
                        arn: v.arn,
                        name: v.name,
                        description: v.description,
                    },
                );
            }
            for sub in view.enabled_standards {
                let sub_data = crate::types::StandardsSubscriptionInfo {
                    standards_subscription_arn: sub.standards_subscription_arn.clone(),
                    standards_arn: sub.standards_arn,
                    standards_status: sub.standards_status,
                    standards_input: sub.standards_input,
                };
                if !guard
                    .enabled_standards
                    .iter()
                    .any(|s| s.standards_subscription_arn == sub_data.standards_subscription_arn)
                {
                    guard.enabled_standards.push(sub_data);
                }
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
