//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cloud9

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEnvironmentEC2Request {
    #[serde(rename = "automaticStopTimeMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_stop_time_minutes: Option<i32>,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "connectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "dryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "imageId")]
    #[serde(default)]
    pub image_id: String,
    #[serde(rename = "instanceType")]
    #[serde(default)]
    pub instance_type: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ownerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_arn: Option<String>,
    #[serde(rename = "subnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
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
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEnvironmentEC2Result {
    #[serde(rename = "environmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEnvironmentMembershipRequest {
    #[serde(rename = "environmentId")]
    #[serde(default)]
    pub environment_id: String,
    #[serde(default)]
    pub permissions: String,
    #[serde(rename = "userArn")]
    #[serde(default)]
    pub user_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEnvironmentMembershipResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership: Option<EnvironmentMember>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnvironmentMember {
    #[serde(rename = "environmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[serde(rename = "lastAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_access: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    #[serde(rename = "userArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
    #[serde(rename = "userId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEnvironmentMembershipRequest {
    #[serde(rename = "environmentId")]
    #[serde(default)]
    pub environment_id: String,
    #[serde(rename = "userArn")]
    #[serde(default)]
    pub user_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEnvironmentMembershipResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEnvironmentRequest {
    #[serde(rename = "environmentId")]
    #[serde(default)]
    pub environment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEnvironmentResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEnvironmentMembershipsRequest {
    #[serde(rename = "environmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "userArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEnvironmentMembershipsResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memberships: Option<Vec<EnvironmentMember>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEnvironmentStatusRequest {
    #[serde(rename = "environmentId")]
    #[serde(default)]
    pub environment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEnvironmentStatusResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEnvironmentsRequest {
    #[serde(rename = "environmentIds")]
    #[serde(default)]
    pub environment_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEnvironmentsResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environments: Option<Vec<Environment>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Environment {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "connectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<EnvironmentLifecycle>,
    #[serde(rename = "managedCredentialsStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_credentials_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ownerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_arn: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnvironmentLifecycle {
    #[serde(rename = "failureResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_resource: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEnvironmentsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEnvironmentsResult {
    #[serde(rename = "environmentIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_ids: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEnvironmentMembershipRequest {
    #[serde(rename = "environmentId")]
    #[serde(default)]
    pub environment_id: String,
    #[serde(default)]
    pub permissions: String,
    #[serde(rename = "userArn")]
    #[serde(default)]
    pub user_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEnvironmentMembershipResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership: Option<EnvironmentMember>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEnvironmentRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "environmentId")]
    #[serde(default)]
    pub environment_id: String,
    #[serde(rename = "managedCredentialsAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_credentials_action: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEnvironmentResult {}
