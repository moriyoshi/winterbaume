use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// A virtual cluster.
#[derive(Debug, Clone)]
pub struct VirtualCluster {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub state: String,
    pub container_provider: ContainerProvider,
    pub created_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

/// Container provider information.
#[derive(Debug, Clone)]
pub struct ContainerProvider {
    pub provider_type: String,
    pub id: String,
    pub info: Option<ContainerInfo>,
}

/// Container info (currently only EKS).
#[derive(Debug, Clone)]
pub struct ContainerInfo {
    pub eks_info: Option<EksInfo>,
}

/// EKS cluster information.
#[derive(Debug, Clone)]
pub struct EksInfo {
    pub namespace: Option<String>,
}

/// A job run in a virtual cluster.
#[derive(Debug, Clone)]
pub struct JobRun {
    pub id: String,
    pub name: Option<String>,
    pub virtual_cluster_id: String,
    pub arn: String,
    pub state: String,
    pub execution_role_arn: String,
    pub release_label: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// A managed endpoint.
#[derive(Debug, Clone)]
pub struct ManagedEndpoint {
    pub id: String,
    pub name: String,
    pub virtual_cluster_id: String,
    pub arn: String,
    pub state: String,
    pub endpoint_type: String,
    pub release_label: String,
    pub execution_role_arn: String,
    pub certificate_arn: Option<String>,
    pub created_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

/// A job template.
#[derive(Debug, Clone)]
pub struct JobTemplate {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub created_at: DateTime<Utc>,
    pub kms_key_arn: Option<String>,
    pub tags: HashMap<String, String>,
    /// Raw job template data stored as JSON value for passthrough.
    pub job_template_data: serde_json::Value,
}

/// A security configuration.
#[derive(Debug, Clone)]
pub struct SecurityConfiguration {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub created_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
    /// Raw security configuration data stored as JSON value for passthrough.
    pub security_configuration_data: serde_json::Value,
}
