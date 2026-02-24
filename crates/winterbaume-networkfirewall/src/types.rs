/// A subnet mapping for a firewall.
#[derive(Debug, Clone)]
pub struct SubnetMapping {
    pub subnet_id: String,
}

/// Sync state for a firewall attachment.
#[derive(Debug, Clone)]
pub struct SyncState {
    pub subnet_id: String,
    pub status: String,
}

/// Firewall status.
#[derive(Debug, Clone)]
pub struct FirewallStatus {
    pub status: String,
    pub configuration_sync_state_summary: String,
}

impl Default for FirewallStatus {
    fn default() -> Self {
        Self {
            status: "READY".to_string(),
            configuration_sync_state_summary: "IN_SYNC".to_string(),
        }
    }
}

/// A Network Firewall firewall resource.
#[derive(Debug, Clone)]
pub struct Firewall {
    pub firewall_name: String,
    pub firewall_arn: String,
    pub firewall_id: String,
    pub firewall_policy_arn: String,
    pub vpc_id: String,
    pub subnet_mappings: Vec<SubnetMapping>,
    pub delete_protection: bool,
    pub subnet_change_protection: bool,
    pub firewall_policy_change_protection: bool,
    pub availability_zone_change_protection: bool,
    pub description: Option<String>,
    pub tags: Vec<(String, String)>,
    pub encryption_configuration: Option<serde_json::Value>,
}

/// Metadata for listing firewalls.
#[derive(Debug, Clone)]
pub struct FirewallMetadata {
    pub firewall_name: String,
    pub firewall_arn: String,
}

/// A Network Firewall rule group resource.
#[derive(Debug, Clone)]
pub struct RuleGroup {
    pub rule_group_name: String,
    pub rule_group_arn: String,
    pub rule_group_id: String,
    pub rule_group_type: String,
    pub capacity: i32,
    pub description: Option<String>,
    pub tags: Vec<(String, String)>,
    /// Raw rule group body stored as JSON value.
    pub rule_group_body: Option<serde_json::Value>,
    /// Raw rules string (Suricata format).
    pub rules: Option<String>,
    pub encryption_configuration: Option<serde_json::Value>,
}

/// Metadata for listing rule groups.
#[derive(Debug, Clone)]
pub struct RuleGroupMetadata {
    pub name: String,
    pub arn: String,
}

/// A Network Firewall firewall policy resource.
#[derive(Debug, Clone)]
pub struct FirewallPolicy {
    pub firewall_policy_name: String,
    pub firewall_policy_arn: String,
    pub firewall_policy_id: String,
    pub description: Option<String>,
    pub tags: Vec<(String, String)>,
    /// Raw firewall policy body stored as JSON value.
    pub firewall_policy_body: serde_json::Value,
    pub encryption_configuration: Option<serde_json::Value>,
}

/// Metadata for listing firewall policies.
#[derive(Debug, Clone)]
pub struct FirewallPolicyMetadata {
    pub name: String,
    pub arn: String,
}

/// A resource policy (used for sharing rule groups / policies cross-account).
#[derive(Debug, Clone)]
pub struct ResourcePolicy {
    pub resource_arn: String,
    pub policy: String,
}

/// A TLS inspection configuration resource.
#[derive(Debug, Clone)]
pub struct TlsInspectionConfiguration {
    pub name: String,
    pub arn: String,
    pub id: String,
    pub description: Option<String>,
    pub tags: Vec<(String, String)>,
    /// Raw body stored as JSON value.
    pub body: serde_json::Value,
}

/// Metadata for listing TLS inspection configurations.
#[derive(Debug, Clone)]
pub struct TlsInspectionConfigurationMetadata {
    pub name: String,
    pub arn: String,
}

/// A VPC endpoint association resource.
#[derive(Debug, Clone)]
pub struct VpcEndpointAssociation {
    pub vpc_endpoint_association_arn: String,
    pub vpc_endpoint_association_id: String,
    pub firewall_arn: String,
    pub vpc_id: String,
    pub subnet_id: String,
    pub description: Option<String>,
    pub tags: Vec<(String, String)>,
}

/// An availability zone mapping for a firewall.
#[derive(Debug, Clone)]
pub struct AvailabilityZoneMapping {
    pub availability_zone: String,
}

/// A transit gateway attachment tracked by the firewall.
#[derive(Debug, Clone)]
pub struct TransitGatewayAttachment {
    pub transit_gateway_attachment_id: String,
    pub status: String,
}

/// A proxy resource.
#[derive(Debug, Clone)]
pub struct NfwProxy {
    pub proxy_name: String,
    pub proxy_arn: String,
    pub nat_gateway_id: String,
    pub proxy_configuration_arn: Option<String>,
    pub proxy_configuration_name: Option<String>,
    pub proxy_state: String,
    pub tags: Vec<(String, String)>,
    /// Raw body stored as JSON value.
    pub body: serde_json::Value,
}

/// A proxy configuration resource.
#[derive(Debug, Clone)]
pub struct NfwProxyConfiguration {
    pub proxy_configuration_name: String,
    pub proxy_configuration_arn: String,
    pub description: Option<String>,
    pub tags: Vec<(String, String)>,
    /// Raw body stored as JSON value.
    pub body: serde_json::Value,
}

/// A proxy rule group resource.
#[derive(Debug, Clone)]
pub struct NfwProxyRuleGroup {
    pub proxy_rule_group_name: String,
    pub proxy_rule_group_arn: String,
    pub description: Option<String>,
    pub tags: Vec<(String, String)>,
    /// Raw body stored as JSON value.
    pub body: serde_json::Value,
}

/// A flow operation resource.
#[derive(Debug, Clone)]
pub struct FlowOperation {
    pub flow_operation_id: String,
    pub firewall_arn: String,
    pub flow_operation_type: String,
    pub flow_operation_status: String,
    /// Raw body stored as JSON value.
    pub body: serde_json::Value,
}

/// An analysis report resource.
#[derive(Debug, Clone)]
pub struct AnalysisReport {
    pub analysis_report_id: String,
    pub firewall_arn: String,
    pub analysis_type: String,
    pub status: String,
}

/// Encryption configuration stored on a firewall.
#[derive(Debug, Clone)]
pub struct EncryptionConfig {
    pub key_id: Option<String>,
    pub config_type: String,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            key_id: None,
            config_type: "AWS_OWNED_KMS_KEY".to_string(),
        }
    }
}
