use std::collections::HashMap;

/// An IoT thing.
#[derive(Debug, Clone)]
pub struct Thing {
    pub thing_name: String,
    pub thing_id: String,
    pub thing_type_name: Option<String>,
    pub attributes: HashMap<String, String>,
    pub version: i64,
    pub thing_arn: String,
    pub billing_group_name: Option<String>,
    pub principals: Vec<String>,
    pub thing_groups: Vec<String>,
}

/// An IoT thing type.
#[derive(Debug, Clone)]
pub struct ThingType {
    pub thing_type_name: String,
    pub thing_type_id: String,
    pub thing_type_arn: String,
    pub thing_type_description: Option<String>,
    pub searchable_attributes: Option<Vec<String>>,
    pub creation_date: f64,
    pub deprecated: bool,
    pub deprecation_date: Option<f64>,
}

/// An IoT thing group.
#[derive(Debug, Clone)]
pub struct ThingGroup {
    pub thing_group_name: String,
    pub thing_group_id: String,
    pub thing_group_arn: String,
    pub parent_group_name: Option<String>,
    pub thing_group_description: Option<String>,
    pub attributes: HashMap<String, String>,
    pub version: i64,
    pub creation_date: f64,
    pub things: Vec<String>,
}

/// An IoT billing group.
#[derive(Debug, Clone)]
pub struct BillingGroup {
    pub billing_group_name: String,
    pub billing_group_id: String,
    pub billing_group_arn: String,
    pub billing_group_description: Option<String>,
    pub version: i64,
    pub creation_date: f64,
    pub things: Vec<String>,
}

/// An IoT certificate.
#[derive(Debug, Clone)]
pub struct Certificate {
    pub certificate_id: String,
    pub certificate_arn: String,
    pub certificate_pem: String,
    pub status: String,
    pub creation_date: f64,
    pub ca_certificate_id: Option<String>,
    pub owned_by: String,
    pub mode: String,
}

/// A CA certificate.
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct CACertificate {
    pub certificate_id: String,
    pub certificate_arn: String,
    pub certificate_pem: String,
    pub status: String,
    pub auto_registration_status: String,
    pub creation_date: f64,
    pub owned_by: String,
    pub mode: String,
}

/// An IoT policy.
#[derive(Debug, Clone)]
pub struct IotPolicy {
    pub policy_name: String,
    pub policy_arn: String,
    pub policy_document: String,
    pub creation_date: f64,
    pub last_modified_date: f64,
    pub generation_id: String,
    pub versions: Vec<PolicyVersionData>,
    pub default_version_id: String,
    pub targets: Vec<String>,
    pub principals: Vec<String>,
}

/// A policy version.
#[derive(Debug, Clone)]
pub struct PolicyVersionData {
    pub version_id: String,
    pub policy_document: String,
    pub is_default: bool,
    pub create_date: f64,
}

/// An IoT role alias.
#[derive(Debug, Clone)]
pub struct RoleAlias {
    pub role_alias: String,
    pub role_alias_arn: String,
    pub role_arn: String,
    pub credential_duration_seconds: i32,
    pub creation_date: f64,
    pub last_modified_date: f64,
    pub owner: String,
}

/// An IoT domain configuration.
#[derive(Debug, Clone)]
pub struct DomainConfiguration {
    pub domain_configuration_name: String,
    pub domain_configuration_arn: String,
    pub domain_name: Option<String>,
    pub domain_configuration_status: String,
    pub service_type: Option<String>,
    pub creation_date: f64,
}

/// An IoT job.
#[derive(Debug, Clone)]
pub struct IotJob {
    pub job_id: String,
    pub job_arn: String,
    pub description: Option<String>,
    pub targets: Vec<String>,
    pub document: Option<String>,
    pub document_source: Option<String>,
    pub status: String,
    pub target_selection: Option<String>,
    pub creation_date: f64,
    pub last_updated_date: f64,
    pub completed_date: Option<f64>,
    pub comment: Option<String>,
    pub reason_code: Option<String>,
    pub force_canceled: Option<bool>,
    pub job_template_arn: Option<String>,
    /// Maps thing_name -> job execution
    pub executions: HashMap<String, JobExecData>,
}

/// A job execution.
#[derive(Debug, Clone)]
pub struct JobExecData {
    pub status: String,
    pub queued_at: f64,
    pub started_at: Option<f64>,
    pub last_updated_at: f64,
    pub execution_number: i64,
    pub version_number: i64,
}

/// An IoT job template.
#[derive(Debug, Clone)]
pub struct JobTemplate {
    pub job_template_id: String,
    pub job_template_arn: String,
    pub description: String,
    pub document: Option<String>,
    pub document_source: Option<String>,
    pub creation_date: f64,
}

/// An IoT topic rule.
#[derive(Debug, Clone)]
pub struct TopicRule {
    pub rule_name: String,
    pub rule_arn: String,
    pub sql: String,
    pub description: Option<String>,
    pub rule_disabled: bool,
    pub creation_date: f64,
    pub actions_json: serde_json::Value,
    pub error_action_json: Option<serde_json::Value>,
    pub aws_iot_sql_version: Option<String>,
}
