//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-mediastore

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateContainerOutput {
    #[serde(rename = "Container")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<Container>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Container {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "AccessLoggingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_logging_enabled: Option<bool>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAccessLoggingOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutMetricPolicyOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutMetricPolicyInput {
    #[serde(rename = "ContainerName")]
    #[serde(default)]
    pub container_name: String,
    #[serde(rename = "MetricPolicy")]
    #[serde(default)]
    pub metric_policy: MetricPolicy,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricPolicy {
    #[serde(rename = "ContainerLevelMetrics")]
    #[serde(default)]
    pub container_level_metrics: String,
    #[serde(rename = "MetricPolicyRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_policy_rules: Option<Vec<MetricPolicyRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricPolicyRule {
    #[serde(rename = "ObjectGroup")]
    #[serde(default)]
    pub object_group: String,
    #[serde(rename = "ObjectGroupName")]
    #[serde(default)]
    pub object_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceOutput {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopAccessLoggingInput {
    #[serde(rename = "ContainerName")]
    #[serde(default)]
    pub container_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutContainerPolicyOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutCorsPolicyOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetContainerPolicyInput {
    #[serde(rename = "ContainerName")]
    #[serde(default)]
    pub container_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceInput {
    #[serde(rename = "Resource")]
    #[serde(default)]
    pub resource: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteContainerPolicyOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLifecyclePolicyOutput {
    #[serde(rename = "LifecyclePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceInput {
    #[serde(rename = "Resource")]
    #[serde(default)]
    pub resource: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLifecyclePolicyOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCorsPolicyOutput {
    #[serde(rename = "CorsPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_policy: Option<Vec<CorsRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CorsRule {
    #[serde(rename = "AllowedHeaders")]
    #[serde(default)]
    pub allowed_headers: Vec<String>,
    #[serde(rename = "AllowedMethods")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_methods: Option<Vec<String>>,
    #[serde(rename = "AllowedOrigins")]
    #[serde(default)]
    pub allowed_origins: Vec<String>,
    #[serde(rename = "ExposeHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expose_headers: Option<Vec<String>>,
    #[serde(rename = "MaxAgeSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutLifecyclePolicyInput {
    #[serde(rename = "ContainerName")]
    #[serde(default)]
    pub container_name: String,
    #[serde(rename = "LifecyclePolicy")]
    #[serde(default)]
    pub lifecycle_policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutContainerPolicyInput {
    #[serde(rename = "ContainerName")]
    #[serde(default)]
    pub container_name: String,
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListContainersInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateContainerInput {
    #[serde(rename = "ContainerName")]
    #[serde(default)]
    pub container_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceInput {
    #[serde(rename = "Resource")]
    #[serde(default)]
    pub resource: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeContainerOutput {
    #[serde(rename = "Container")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<Container>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteContainerPolicyInput {
    #[serde(rename = "ContainerName")]
    #[serde(default)]
    pub container_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMetricPolicyOutput {
    #[serde(rename = "MetricPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_policy: Option<MetricPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutCorsPolicyInput {
    #[serde(rename = "ContainerName")]
    #[serde(default)]
    pub container_name: String,
    #[serde(rename = "CorsPolicy")]
    #[serde(default)]
    pub cors_policy: Vec<CorsRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteContainerOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMetricPolicyInput {
    #[serde(rename = "ContainerName")]
    #[serde(default)]
    pub container_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMetricPolicyOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLifecyclePolicyInput {
    #[serde(rename = "ContainerName")]
    #[serde(default)]
    pub container_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCorsPolicyOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeContainerInput {
    #[serde(rename = "ContainerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopAccessLoggingOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCorsPolicyInput {
    #[serde(rename = "ContainerName")]
    #[serde(default)]
    pub container_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListContainersOutput {
    #[serde(rename = "Containers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<Container>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCorsPolicyInput {
    #[serde(rename = "ContainerName")]
    #[serde(default)]
    pub container_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLifecyclePolicyInput {
    #[serde(rename = "ContainerName")]
    #[serde(default)]
    pub container_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutLifecyclePolicyOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAccessLoggingInput {
    #[serde(rename = "ContainerName")]
    #[serde(default)]
    pub container_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteContainerInput {
    #[serde(rename = "ContainerName")]
    #[serde(default)]
    pub container_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMetricPolicyInput {
    #[serde(rename = "ContainerName")]
    #[serde(default)]
    pub container_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetContainerPolicyOutput {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}
