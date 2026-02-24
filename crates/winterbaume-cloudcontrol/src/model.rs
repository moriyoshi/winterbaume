//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cloudcontrol

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelResourceRequestInput {
    #[serde(rename = "RequestToken")]
    #[serde(default)]
    pub request_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelResourceRequestOutput {
    #[serde(rename = "ProgressEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_event: Option<ProgressEvent>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProgressEvent {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "EventTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<f64>,
    #[serde(rename = "HooksRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks_request_token: Option<String>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "Operation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "OperationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_status: Option<String>,
    #[serde(rename = "RequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_token: Option<String>,
    #[serde(rename = "ResourceModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_model: Option<String>,
    #[serde(rename = "RetryAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<f64>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResourceInput {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "DesiredState")]
    #[serde(default)]
    pub desired_state: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    pub type_name: String,
    #[serde(rename = "TypeVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResourceOutput {
    #[serde(rename = "ProgressEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_event: Option<ProgressEvent>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourceInput {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    pub type_name: String,
    #[serde(rename = "TypeVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourceOutput {
    #[serde(rename = "ProgressEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_event: Option<ProgressEvent>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceInput {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    pub type_name: String,
    #[serde(rename = "TypeVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceOutput {
    #[serde(rename = "ResourceDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_description: Option<ResourceDescription>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceDescription {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceRequestStatusInput {
    #[serde(rename = "RequestToken")]
    #[serde(default)]
    pub request_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceRequestStatusOutput {
    #[serde(rename = "HooksProgressEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks_progress_event: Option<Vec<HookProgressEvent>>,
    #[serde(rename = "ProgressEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_event: Option<ProgressEvent>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HookProgressEvent {
    #[serde(rename = "FailureMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_mode: Option<String>,
    #[serde(rename = "HookEventTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_event_time: Option<f64>,
    #[serde(rename = "HookStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_status: Option<String>,
    #[serde(rename = "HookStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_status_message: Option<String>,
    #[serde(rename = "HookTypeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_type_arn: Option<String>,
    #[serde(rename = "HookTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_type_name: Option<String>,
    #[serde(rename = "HookTypeVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_type_version_id: Option<String>,
    #[serde(rename = "InvocationPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_point: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceRequestsInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceRequestStatusFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_request_status_filter: Option<ResourceRequestStatusFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceRequestStatusFilter {
    #[serde(rename = "OperationStatuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_statuses: Option<Vec<String>>,
    #[serde(rename = "Operations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceRequestsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceRequestStatusSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_request_status_summaries: Option<Vec<ProgressEvent>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourcesInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_model: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    pub type_name: String,
    #[serde(rename = "TypeVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourcesOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_descriptions: Option<Vec<ResourceDescription>>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResourceInput {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "PatchDocument")]
    #[serde(default)]
    pub patch_document: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    pub type_name: String,
    #[serde(rename = "TypeVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResourceOutput {
    #[serde(rename = "ProgressEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_event: Option<ProgressEvent>,
}
