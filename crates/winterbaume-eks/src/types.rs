use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Cluster {
    pub name: String,
    pub arn: String,
    pub endpoint: String,
    pub role_arn: String,
    pub status: String,
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub nodegroups: HashMap<String, Nodegroup>,
    pub fargate_profiles: HashMap<String, FargateProfile>,
    pub addons: HashMap<String, Addon>,
    pub access_entries: HashMap<String, AccessEntry>,
    pub identity_provider_configs: HashMap<String, IdentityProviderConfigStore>,
    pub pod_identity_associations: HashMap<String, PodIdentityAssociation>,
    pub capabilities: HashMap<String, CapabilityStore>,
    pub tags: HashMap<String, String>,
    pub updates: HashMap<String, UpdateRecord>,
}

#[derive(Debug, Clone)]
pub struct Addon {
    pub addon_name: String,
    pub addon_arn: String,
    pub cluster_name: String,
    pub addon_version: String,
    pub service_account_role_arn: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct AccessEntry {
    pub principal_arn: String,
    pub access_entry_arn: String,
    pub cluster_name: String,
    pub kubernetes_groups: Vec<String>,
    pub entry_type: String,
    pub username: Option<String>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
    pub associated_policies: HashMap<String, AssociatedPolicy>,
}

#[derive(Debug, Clone)]
pub struct AssociatedPolicy {
    pub policy_arn: String,
    pub access_scope_type: String,
    pub namespaces: Vec<String>,
    pub associated_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct IdentityProviderConfigStore {
    pub name: String,
    pub config_type: String,
    pub cluster_name: String,
    pub arn: String,
    pub issuer_url: String,
    pub client_id: String,
    pub status: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct PodIdentityAssociation {
    pub association_id: String,
    pub association_arn: String,
    pub cluster_name: String,
    pub namespace: String,
    pub service_account: String,
    pub role_arn: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct CapabilityStore {
    pub arn: String,
    pub capability_name: String,
    pub cluster_name: String,
    pub capability_type: String,
    pub role_arn: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct EksAnywhereSubscription {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub auto_renew: bool,
    pub license_quantity: i32,
    pub license_type: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Nodegroup {
    pub name: String,
    pub arn: String,
    pub cluster_name: String,
    pub node_role: String,
    pub scaling_config: ScalingConfig,
    pub status: String,
    pub tags: HashMap<String, String>,
    pub labels: HashMap<String, String>,
    pub taints: Vec<Taint>,
}

#[derive(Debug, Clone)]
pub struct Taint {
    pub key: String,
    pub value: Option<String>,
    pub effect: String,
}

#[derive(Debug, Clone)]
pub struct ScalingConfig {
    pub min_size: i32,
    pub max_size: i32,
    pub desired_size: i32,
}

#[derive(Debug, Clone)]
pub struct FargateProfile {
    pub name: String,
    pub arn: String,
    pub cluster_name: String,
    pub pod_execution_role_arn: String,
    pub selectors: Vec<FargateSelector>,
    pub status: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct FargateSelector {
    pub namespace: String,
    pub labels: HashMap<String, String>,
}

/// Tracks an update operation on a cluster.
#[derive(Debug, Clone)]
pub struct UpdateRecord {
    pub id: String,
    pub cluster_name: String,
    pub update_type: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}
