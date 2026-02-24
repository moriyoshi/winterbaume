//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-codestarnotifications

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNotificationRuleRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DetailType")]
    #[serde(default)]
    pub detail_type: String,
    #[serde(rename = "EventTypeIds")]
    #[serde(default)]
    pub event_type_ids: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Resource")]
    #[serde(default)]
    pub resource: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    pub targets: Vec<Target>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Target {
    #[serde(rename = "TargetAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_address: Option<String>,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNotificationRuleResult {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNotificationRuleRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNotificationRuleResult {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTargetRequest {
    #[serde(rename = "ForceUnsubscribeAll")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_unsubscribe_all: Option<bool>,
    #[serde(rename = "TargetAddress")]
    #[serde(default)]
    pub target_address: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTargetResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeNotificationRuleRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeNotificationRuleResult {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "DetailType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_type: Option<String>,
    #[serde(rename = "EventTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<EventTypeSummary>>,
    #[serde(rename = "LastModifiedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_timestamp: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Resource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<TargetSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventTypeSummary {
    #[serde(rename = "EventTypeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_id: Option<String>,
    #[serde(rename = "EventTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_name: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetSummary {
    #[serde(rename = "TargetAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_address: Option<String>,
    #[serde(rename = "TargetStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_status: Option<String>,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEventTypesRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ListEventTypesFilter>>,
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
pub struct ListEventTypesFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEventTypesResult {
    #[serde(rename = "EventTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<EventTypeSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNotificationRulesRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ListNotificationRulesFilter>>,
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
pub struct ListNotificationRulesFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNotificationRulesResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "NotificationRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_rules: Option<Vec<NotificationRuleSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotificationRuleSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResult {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTargetsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ListTargetsFilter>>,
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
pub struct ListTargetsFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTargetsResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<TargetSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubscribeRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "Target")]
    #[serde(default)]
    pub target: Target,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubscribeResult {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResult {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnsubscribeRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "TargetAddress")]
    #[serde(default)]
    pub target_address: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnsubscribeResult {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNotificationRuleRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "DetailType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_type: Option<String>,
    #[serde(rename = "EventTypeIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type_ids: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNotificationRuleResult {}
