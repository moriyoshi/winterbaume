//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-dsql

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateClusterOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "deletionProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection_enabled: Option<bool>,
    #[serde(rename = "encryptionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_details: Option<EncryptionDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "multiRegionProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_properties: Option<MultiRegionProperties>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionDetails {
    #[serde(rename = "encryptionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_status: Option<String>,
    #[serde(rename = "encryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiRegionProperties {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<String>>,
    #[serde(rename = "witnessRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClusterOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClusterInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClusterPolicyInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClusterPolicyOutput {
    #[serde(rename = "policyVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVpcEndpointServiceNameInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVpcEndpointServiceNameOutput {
    #[serde(rename = "clusterVpcEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_vpc_endpoint: Option<String>,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListClustersInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetClusterOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "deletionProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection_enabled: Option<bool>,
    #[serde(rename = "encryptionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_details: Option<EncryptionDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "multiRegionProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_properties: Option<MultiRegionProperties>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateClusterInput {
    #[serde(rename = "bypassPolicyLockoutSafetyCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_policy_lockout_safety_check: Option<bool>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "deletionProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection_enabled: Option<bool>,
    #[serde(rename = "kmsEncryptionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_encryption_key: Option<String>,
    #[serde(rename = "multiRegionProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_properties: Option<MultiRegionProperties>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetClusterPolicyInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutClusterPolicyOutput {
    #[serde(rename = "policyVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetClusterPolicyOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "policyVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClusterOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutClusterPolicyInput {
    #[serde(rename = "bypassPolicyLockoutSafetyCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_policy_lockout_safety_check: Option<bool>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "expectedPolicyVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_policy_version: Option<String>,
    #[serde(default)]
    pub policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceInput {
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListClustersOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<ClusterSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClusterInput {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "deletionProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection_enabled: Option<bool>,
    #[serde(rename = "kmsEncryptionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_encryption_key: Option<String>,
    #[serde(rename = "multiRegionProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_properties: Option<MultiRegionProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetClusterInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceInput {}
