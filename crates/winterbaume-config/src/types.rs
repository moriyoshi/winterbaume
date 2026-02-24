#[derive(Debug, Clone)]
pub struct ConfigurationRecorder {
    pub name: String,
    pub role_arn: String,
    pub recording_group: Option<RecordingGroup>,
    pub recording: bool,
    pub last_start_time: Option<f64>,
    pub last_stop_time: Option<f64>,
    pub recording_mode: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct RecordingGroup {
    pub all_supported: bool,
    pub include_global_resource_types: bool,
}

#[derive(Debug, Clone)]
pub struct DeliveryChannel {
    pub name: String,
    pub s3_bucket_name: String,
    pub s3_key_prefix: String,
    pub snapshot_delivery_properties: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct ConfigRule {
    pub config_rule_name: String,
    pub config_rule_arn: String,
    pub config_rule_id: String,
    pub config_rule_state: String,
    pub description: Option<String>,
    pub source_owner: String,
    pub source_identifier: Option<String>,
    pub input_parameters: Option<String>,
    pub maximum_execution_frequency: Option<String>,
    pub scope_resource_types: Option<Vec<String>>,
    pub evaluation_mode: Option<serde_json::Value>,
    pub scope: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct AggregationAuthorizationEntry {
    pub authorized_account_id: String,
    pub authorized_aws_region: String,
    pub aggregation_authorization_arn: String,
    pub creation_time: f64,
}

#[derive(Debug, Clone)]
pub struct ConfigurationAggregatorEntry {
    pub configuration_aggregator_name: String,
    pub configuration_aggregator_arn: String,
    pub account_aggregation_sources: Option<Vec<AccountAggregationSourceEntry>>,
    pub organization_aggregation_source: Option<OrganizationAggregationSourceEntry>,
    pub creation_time: f64,
    pub last_updated_time: f64,
}

#[derive(Debug, Clone)]
pub struct AccountAggregationSourceEntry {
    pub account_ids: Vec<String>,
    pub all_aws_regions: Option<bool>,
    pub aws_regions: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct OrganizationAggregationSourceEntry {
    pub role_arn: String,
    pub all_aws_regions: Option<bool>,
    pub aws_regions: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct RetentionConfigurationEntry {
    pub name: String,
    pub retention_period_in_days: i32,
}

#[derive(Debug, Clone)]
pub struct OrganizationConformancePackEntry {
    pub organization_conformance_pack_name: String,
    pub organization_conformance_pack_arn: String,
    pub delivery_s3_bucket: Option<String>,
    pub delivery_s3_key_prefix: Option<String>,
    pub excluded_accounts: Option<Vec<String>>,
    pub conformance_pack_input_parameters: Option<Vec<(String, String)>>,
    pub last_update_time: f64,
}

#[derive(Debug, Clone)]
pub struct ResourceConfigEntry {
    pub resource_type: String,
    pub resource_id: String,
    pub schema_version_id: String,
    pub configuration: String,
    pub resource_name: Option<String>,
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone)]
pub struct TagEntry {
    pub key: String,
    pub value: String,
}

/// A remediation configuration entry.
#[derive(Debug, Clone)]
pub struct RemediationConfigEntry {
    pub config_rule_name: String,
    pub target_type: String,
    pub target_id: String,
    pub target_version: Option<String>,
    pub automatic: Option<bool>,
    pub maximum_automatic_attempts: Option<i32>,
    pub retry_attempt_seconds: Option<i64>,
    pub resource_type: Option<String>,
    pub parameters: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// An organization config rule entry.
#[derive(Debug, Clone)]
pub struct OrganizationConfigRuleEntry {
    pub organization_config_rule_name: String,
    pub organization_config_rule_arn: String,
    pub excluded_accounts: Option<Vec<String>>,
    pub last_update_time: f64,
    /// Raw JSON metadata (OrganizationManagedRuleMetadata, OrganizationCustomRuleMetadata, or OrganizationCustomPolicyRuleMetadata)
    pub managed_rule_metadata: Option<serde_json::Value>,
    pub custom_rule_metadata: Option<serde_json::Value>,
    pub custom_policy_rule_metadata: Option<serde_json::Value>,
}
