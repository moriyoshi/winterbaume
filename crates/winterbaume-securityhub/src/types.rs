use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct OrganizationConfig {
    pub auto_enable: bool,
    pub member_account_limit_reached: bool,
    pub auto_enable_standards: String,
}

#[derive(Debug, Clone, Default)]
pub struct OrgAdminAccount {
    pub account_id: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct Finding {
    pub id: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct Member {
    pub account_id: String,
    pub email: String,
    pub member_status: String,
    pub invited_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Default)]
pub struct HubInfo {
    pub enabled: bool,
    pub subscribed_at: Option<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ActionTargetInfo {
    pub arn: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct FindingAggregatorInfo {
    pub arn: String,
    pub region_linking_mode: String,
    pub regions: Vec<String>,
    pub finding_aggregation_region: String,
}

#[derive(Debug, Clone)]
pub struct StandardsSubscriptionInfo {
    pub standards_subscription_arn: String,
    pub standards_arn: String,
    pub standards_status: String,
    pub standards_input: HashMap<String, String>,
}

/// Automation rule (v1 and v2), keyed by ARN.
#[derive(Debug, Clone)]
pub struct AutomationRuleInfo {
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

/// Configuration policy.
#[derive(Debug, Clone)]
pub struct ConfigPolicyInfo {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub configuration_policy: Option<serde_json::Value>,
}

/// Connector (v2).
#[derive(Debug, Clone)]
pub struct ConnectorV2Info {
    pub connector_id: String,
    pub connector_arn: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub last_updated_at: String,
    pub connector_status: String,
    pub provider_name: Option<String>,
    pub tags: HashMap<String, String>,
    pub raw: serde_json::Value,
}

/// Insight.
#[derive(Debug, Clone)]
pub struct InsightInfo {
    pub arn: String,
    pub name: String,
    pub group_by_attribute: String,
    pub filters: serde_json::Value,
}

/// Hub v2 (Security Hub v2).
#[derive(Debug, Clone, Default)]
pub struct HubV2Info {
    pub enabled: bool,
    pub hub_v2_arn: Option<String>,
    pub subscribed_at: Option<String>,
}

/// AggregatorV2 (cross-region aggregator v2).
#[derive(Debug, Clone)]
pub struct AggregatorV2Info {
    pub arn: String,
    pub aggregation_region: String,
    pub region_linking_mode: String,
    pub linked_regions: Vec<String>,
}

/// Product subscription (EnableImportFindingsForProduct).
#[derive(Debug, Clone)]
pub struct ProductSubscription {
    pub product_subscription_arn: String,
    pub product_arn: String,
}

/// Configuration policy association.
#[derive(Debug, Clone)]
pub struct ConfigPolicyAssociation {
    pub configuration_policy_id: String,
    pub target_id: String,
    pub target_type: String,
    pub association_type: String,
    pub association_status: String,
    pub association_status_message: Option<String>,
    pub updated_at: String,
}

/// Security control definition (catalog).
#[derive(Debug, Clone)]
pub struct SecurityControlInfo {
    pub security_control_id: String,
    pub security_control_arn: String,
    pub title: String,
    pub description: String,
    pub remediation_url: String,
    pub severity_rating: String,
    pub security_control_status: String,
    pub current_region_availability: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub last_update_reason: Option<String>,
}

/// Standards control association keyed by (security_control_id, standards_arn).
#[derive(Debug, Clone)]
pub struct StandardsControlAssociationInfo {
    pub security_control_id: String,
    pub standards_arn: String,
    pub association_status: String,
    pub updated_reason: Option<String>,
    pub security_control_arn: Option<String>,
    pub standards_control_title: Option<String>,
    pub standards_control_description: Option<String>,
    pub standards_control_arns: Vec<String>,
    pub related_requirements: Vec<String>,
}

/// Standards control (legacy v1 control) keyed by standards_control_arn.
#[derive(Debug, Clone)]
pub struct StandardsControlInfo {
    pub standards_control_arn: String,
    pub control_status: String,
    pub disabled_reason: Option<String>,
    pub control_status_updated_at: Option<String>,
    pub control_id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub remediation_url: Option<String>,
    pub severity_rating: Option<String>,
    pub related_requirements: Vec<String>,
}

/// Standard definition (catalog entry).
#[derive(Debug, Clone)]
pub struct StandardInfo {
    pub standards_arn: String,
    pub name: String,
    pub description: String,
    pub enabled_by_default: bool,
}

/// Product integration (catalog entry).
#[derive(Debug, Clone)]
pub struct ProductInfo {
    pub product_arn: String,
    pub product_name: String,
    pub company_name: String,
    pub description: String,
    pub categories: Vec<String>,
    pub integration_types: Vec<String>,
    pub marketplace_url: Option<String>,
    pub activation_url: Option<String>,
    pub product_subscription_resource_policy: Option<String>,
}

/// Ticket v2.
#[derive(Debug, Clone)]
pub struct TicketV2Info {
    pub ticket_id: String,
    pub ticket_src_url: Option<String>,
}

/// Finding v2 (OCSF-based) stored as raw JSON.
#[derive(Debug, Clone)]
pub struct FindingV2 {
    pub metadata_uid: String,
    pub data: serde_json::Value,
}
