//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-xray

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetTracesRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TraceIds")]
    #[serde(default)]
    pub trace_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetTracesResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Traces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traces: Option<Vec<Trace>>,
    #[serde(rename = "UnprocessedTraceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_trace_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Trace {
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LimitExceeded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_exceeded: Option<bool>,
    #[serde(rename = "Segments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<Segment>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Segment {
    #[serde(rename = "Document")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelTraceRetrievalRequest {
    #[serde(rename = "RetrievalToken")]
    #[serde(default)]
    pub retrieval_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelTraceRetrievalResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGroupRequest {
    #[serde(rename = "FilterExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "InsightsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights_configuration: Option<InsightsConfiguration>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InsightsConfiguration {
    #[serde(rename = "InsightsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights_enabled: Option<bool>,
    #[serde(rename = "NotificationsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications_enabled: Option<bool>,
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
pub struct CreateGroupResult {
    #[serde(rename = "Group")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Group {
    #[serde(rename = "FilterExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    #[serde(rename = "GroupARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_a_r_n: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "InsightsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights_configuration: Option<InsightsConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSamplingRuleRequest {
    #[serde(rename = "SamplingRule")]
    #[serde(default)]
    pub sampling_rule: SamplingRule,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SamplingRule {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "FixedRate")]
    #[serde(default)]
    pub fixed_rate: f64,
    #[serde(rename = "HTTPMethod")]
    #[serde(default)]
    pub h_t_t_p_method: String,
    #[serde(rename = "Host")]
    #[serde(default)]
    pub host: String,
    #[serde(rename = "Priority")]
    #[serde(default)]
    pub priority: i32,
    #[serde(rename = "ReservoirSize")]
    #[serde(default)]
    pub reservoir_size: i32,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "RuleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_a_r_n: Option<String>,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[serde(rename = "SamplingRateBoost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rate_boost: Option<SamplingRateBoost>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    pub service_name: String,
    #[serde(rename = "ServiceType")]
    #[serde(default)]
    pub service_type: String,
    #[serde(rename = "URLPath")]
    #[serde(default)]
    pub u_r_l_path: String,
    #[serde(rename = "Version")]
    #[serde(default)]
    pub version: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SamplingRateBoost {
    #[serde(rename = "CooldownWindowMinutes")]
    #[serde(default)]
    pub cooldown_window_minutes: i32,
    #[serde(rename = "MaxRate")]
    #[serde(default)]
    pub max_rate: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSamplingRuleResult {
    #[serde(rename = "SamplingRuleRecord")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rule_record: Option<SamplingRuleRecord>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SamplingRuleRecord {
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "ModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(rename = "SamplingRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rule: Option<SamplingRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGroupRequest {
    #[serde(rename = "GroupARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_a_r_n: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGroupResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyRequest {
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(rename = "PolicyRevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSamplingRuleRequest {
    #[serde(rename = "RuleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_a_r_n: Option<String>,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSamplingRuleResult {
    #[serde(rename = "SamplingRuleRecord")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rule_record: Option<SamplingRuleRecord>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEncryptionConfigRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEncryptionConfigResult {
    #[serde(rename = "EncryptionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionConfig {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGroupRequest {
    #[serde(rename = "GroupARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_a_r_n: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGroupResult {
    #[serde(rename = "Group")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGroupsRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGroupsResult {
    #[serde(rename = "Groups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupSummary {
    #[serde(rename = "FilterExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    #[serde(rename = "GroupARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_a_r_n: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "InsightsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights_configuration: Option<InsightsConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIndexingRulesRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIndexingRulesResult {
    #[serde(rename = "IndexingRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexing_rules: Option<Vec<IndexingRule>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IndexingRule {
    #[serde(rename = "ModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Rule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<IndexingRuleValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IndexingRuleValue {
    #[serde(rename = "Probabilistic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probabilistic: Option<ProbabilisticRuleValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProbabilisticRuleValue {
    #[serde(rename = "ActualSamplingPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_sampling_percentage: Option<f64>,
    #[serde(rename = "DesiredSamplingPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_sampling_percentage: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInsightEventsRequest {
    #[serde(rename = "InsightId")]
    #[serde(default)]
    pub insight_id: String,
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
pub struct GetInsightEventsResult {
    #[serde(rename = "InsightEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_events: Option<Vec<InsightEvent>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InsightEvent {
    #[serde(rename = "ClientRequestImpactStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_impact_statistics: Option<RequestImpactStatistics>,
    #[serde(rename = "EventTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<f64>,
    #[serde(rename = "RootCauseServiceRequestImpactStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_cause_service_request_impact_statistics: Option<RequestImpactStatistics>,
    #[serde(rename = "Summary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "TopAnomalousServices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_anomalous_services: Option<Vec<AnomalousService>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestImpactStatistics {
    #[serde(rename = "FaultCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fault_count: Option<i64>,
    #[serde(rename = "OkCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ok_count: Option<i64>,
    #[serde(rename = "TotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnomalousService {
    #[serde(rename = "ServiceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<ServiceId>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceId {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Names")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInsightImpactGraphRequest {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "InsightId")]
    #[serde(default)]
    pub insight_id: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInsightImpactGraphResult {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "InsightId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_id: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceGraphEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_graph_end_time: Option<f64>,
    #[serde(rename = "ServiceGraphStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_graph_start_time: Option<f64>,
    #[serde(rename = "Services")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<InsightImpactGraphService>>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InsightImpactGraphService {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Edges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<InsightImpactGraphEdge>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Names")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    #[serde(rename = "ReferenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InsightImpactGraphEdge {
    #[serde(rename = "ReferenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInsightRequest {
    #[serde(rename = "InsightId")]
    #[serde(default)]
    pub insight_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInsightResult {
    #[serde(rename = "Insight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight: Option<Insight>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Insight {
    #[serde(rename = "Categories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    #[serde(rename = "ClientRequestImpactStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_impact_statistics: Option<RequestImpactStatistics>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "GroupARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_a_r_n: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "InsightId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_id: Option<String>,
    #[serde(rename = "RootCauseServiceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_cause_service_id: Option<ServiceId>,
    #[serde(rename = "RootCauseServiceRequestImpactStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_cause_service_request_impact_statistics: Option<RequestImpactStatistics>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Summary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "TopAnomalousServices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_anomalous_services: Option<Vec<AnomalousService>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInsightSummariesRequest {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "GroupARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_a_r_n: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: f64,
    #[serde(rename = "States")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInsightSummariesResult {
    #[serde(rename = "InsightSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_summaries: Option<Vec<InsightSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InsightSummary {
    #[serde(rename = "Categories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    #[serde(rename = "ClientRequestImpactStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_impact_statistics: Option<RequestImpactStatistics>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "GroupARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_a_r_n: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "InsightId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_id: Option<String>,
    #[serde(rename = "LastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    #[serde(rename = "RootCauseServiceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_cause_service_id: Option<ServiceId>,
    #[serde(rename = "RootCauseServiceRequestImpactStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_cause_service_request_impact_statistics: Option<RequestImpactStatistics>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Summary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "TopAnomalousServices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_anomalous_services: Option<Vec<AnomalousService>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRetrievedTracesGraphRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RetrievalToken")]
    #[serde(default)]
    pub retrieval_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRetrievedTracesGraphResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RetrievalStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_status: Option<String>,
    #[serde(rename = "Services")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<RetrievedService>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetrievedService {
    #[serde(rename = "Links")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<GraphLink>>,
    #[serde(rename = "Service")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GraphLink {
    #[serde(rename = "DestinationTraceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_trace_ids: Option<Vec<String>>,
    #[serde(rename = "ReferenceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<String>,
    #[serde(rename = "SourceTraceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_trace_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Service {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "DurationHistogram")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_histogram: Option<Vec<HistogramEntry>>,
    #[serde(rename = "Edges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<Edge>>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Names")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    #[serde(rename = "ReferenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<i32>,
    #[serde(rename = "ResponseTimeHistogram")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time_histogram: Option<Vec<HistogramEntry>>,
    #[serde(rename = "Root")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<bool>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "SummaryStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_statistics: Option<ServiceStatistics>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HistogramEntry {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Edge {
    #[serde(rename = "Aliases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<Alias>>,
    #[serde(rename = "EdgeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_type: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "ReceivedEventAgeHistogram")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_event_age_histogram: Option<Vec<HistogramEntry>>,
    #[serde(rename = "ReferenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<i32>,
    #[serde(rename = "ResponseTimeHistogram")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time_histogram: Option<Vec<HistogramEntry>>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "SummaryStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_statistics: Option<EdgeStatistics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Alias {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Names")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EdgeStatistics {
    #[serde(rename = "ErrorStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_statistics: Option<ErrorStatistics>,
    #[serde(rename = "FaultStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fault_statistics: Option<FaultStatistics>,
    #[serde(rename = "OkCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ok_count: Option<i64>,
    #[serde(rename = "TotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[serde(rename = "TotalResponseTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_response_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorStatistics {
    #[serde(rename = "OtherCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_count: Option<i64>,
    #[serde(rename = "ThrottleCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_count: Option<i64>,
    #[serde(rename = "TotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FaultStatistics {
    #[serde(rename = "OtherCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_count: Option<i64>,
    #[serde(rename = "TotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceStatistics {
    #[serde(rename = "ErrorStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_statistics: Option<ErrorStatistics>,
    #[serde(rename = "FaultStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fault_statistics: Option<FaultStatistics>,
    #[serde(rename = "OkCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ok_count: Option<i64>,
    #[serde(rename = "TotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[serde(rename = "TotalResponseTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_response_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSamplingRulesRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSamplingRulesResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SamplingRuleRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rule_records: Option<Vec<SamplingRuleRecord>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSamplingStatisticSummariesRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSamplingStatisticSummariesResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SamplingStatisticSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_statistic_summaries: Option<Vec<SamplingStatisticSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SamplingStatisticSummary {
    #[serde(rename = "BorrowCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_count: Option<i32>,
    #[serde(rename = "RequestCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_count: Option<i32>,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[serde(rename = "SampledCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampled_count: Option<i32>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSamplingTargetsRequest {
    #[serde(rename = "SamplingBoostStatisticsDocuments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_boost_statistics_documents: Option<Vec<SamplingBoostStatisticsDocument>>,
    #[serde(rename = "SamplingStatisticsDocuments")]
    #[serde(default)]
    pub sampling_statistics_documents: Vec<SamplingStatisticsDocument>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SamplingBoostStatisticsDocument {
    #[serde(rename = "AnomalyCount")]
    #[serde(default)]
    pub anomaly_count: i32,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    pub rule_name: String,
    #[serde(rename = "SampledAnomalyCount")]
    #[serde(default)]
    pub sampled_anomaly_count: i32,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    pub service_name: String,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    pub timestamp: f64,
    #[serde(rename = "TotalCount")]
    #[serde(default)]
    pub total_count: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SamplingStatisticsDocument {
    #[serde(rename = "BorrowCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_count: Option<i32>,
    #[serde(rename = "ClientID")]
    #[serde(default)]
    pub client_i_d: String,
    #[serde(rename = "RequestCount")]
    #[serde(default)]
    pub request_count: i32,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    pub rule_name: String,
    #[serde(rename = "SampledCount")]
    #[serde(default)]
    pub sampled_count: i32,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    pub timestamp: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSamplingTargetsResult {
    #[serde(rename = "LastRuleModification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_rule_modification: Option<f64>,
    #[serde(rename = "SamplingTargetDocuments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_target_documents: Option<Vec<SamplingTargetDocument>>,
    #[serde(rename = "UnprocessedBoostStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_boost_statistics: Option<Vec<UnprocessedStatistics>>,
    #[serde(rename = "UnprocessedStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_statistics: Option<Vec<UnprocessedStatistics>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SamplingTargetDocument {
    #[serde(rename = "FixedRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_rate: Option<f64>,
    #[serde(rename = "Interval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[serde(rename = "ReservoirQuota")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservoir_quota: Option<i32>,
    #[serde(rename = "ReservoirQuotaTTL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservoir_quota_t_t_l: Option<f64>,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[serde(rename = "SamplingBoost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_boost: Option<SamplingBoost>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SamplingBoost {
    #[serde(rename = "BoostRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_rate: Option<f64>,
    #[serde(rename = "BoostRateTTL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_rate_t_t_l: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnprocessedStatistics {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceGraphRequest {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "GroupARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_a_r_n: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceGraphResult {
    #[serde(rename = "ContainsOldGroupVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_old_group_versions: Option<bool>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Services")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTimeSeriesServiceStatisticsRequest {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "EntitySelectorExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_selector_expression: Option<String>,
    #[serde(rename = "ForecastStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_statistics: Option<bool>,
    #[serde(rename = "GroupARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_a_r_n: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Period")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTimeSeriesServiceStatisticsResult {
    #[serde(rename = "ContainsOldGroupVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_old_group_versions: Option<bool>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TimeSeriesServiceStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_series_service_statistics: Option<Vec<TimeSeriesServiceStatistics>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeSeriesServiceStatistics {
    #[serde(rename = "EdgeSummaryStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_summary_statistics: Option<EdgeStatistics>,
    #[serde(rename = "ResponseTimeHistogram")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time_histogram: Option<Vec<HistogramEntry>>,
    #[serde(rename = "ServiceForecastStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_forecast_statistics: Option<ForecastStatistics>,
    #[serde(rename = "ServiceSummaryStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_summary_statistics: Option<ServiceStatistics>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ForecastStatistics {
    #[serde(rename = "FaultCountHigh")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fault_count_high: Option<i64>,
    #[serde(rename = "FaultCountLow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fault_count_low: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTraceGraphRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TraceIds")]
    #[serde(default)]
    pub trace_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTraceGraphResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Services")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTraceSegmentDestinationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTraceSegmentDestinationResult {
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTraceSummariesRequest {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "FilterExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Sampling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<bool>,
    #[serde(rename = "SamplingStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_strategy: Option<SamplingStrategy>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: f64,
    #[serde(rename = "TimeRangeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SamplingStrategy {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTraceSummariesResult {
    #[serde(rename = "ApproximateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_time: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TraceSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_summaries: Option<Vec<TraceSummary>>,
    #[serde(rename = "TracesProcessedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traces_processed_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TraceSummary {
    #[serde(rename = "Annotations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, Vec<ValueWithServiceIds>>>,
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<AvailabilityZoneDetail>>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(rename = "EntryPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<ServiceId>,
    #[serde(rename = "ErrorRootCauses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_root_causes: Option<Vec<ErrorRootCause>>,
    #[serde(rename = "FaultRootCauses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fault_root_causes: Option<Vec<FaultRootCause>>,
    #[serde(rename = "HasError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_error: Option<bool>,
    #[serde(rename = "HasFault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_fault: Option<bool>,
    #[serde(rename = "HasThrottle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_throttle: Option<bool>,
    #[serde(rename = "Http")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<Http>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InstanceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<InstanceIdDetail>>,
    #[serde(rename = "IsPartial")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_partial: Option<bool>,
    #[serde(rename = "MatchedEventTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_event_time: Option<f64>,
    #[serde(rename = "ResourceARNs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_ns: Option<Vec<ResourceARNDetail>>,
    #[serde(rename = "ResponseTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time: Option<f64>,
    #[serde(rename = "ResponseTimeRootCauses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time_root_causes: Option<Vec<ResponseTimeRootCause>>,
    #[serde(rename = "Revision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[serde(rename = "ServiceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_ids: Option<Vec<ServiceId>>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Users")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<TraceUser>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValueWithServiceIds {
    #[serde(rename = "AnnotationValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_value: Option<AnnotationValue>,
    #[serde(rename = "ServiceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_ids: Option<Vec<ServiceId>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnnotationValue {
    #[serde(rename = "BooleanValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_value: Option<bool>,
    #[serde(rename = "NumberValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_value: Option<f64>,
    #[serde(rename = "StringValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailabilityZoneDetail {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorRootCause {
    #[serde(rename = "ClientImpacting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_impacting: Option<bool>,
    #[serde(rename = "Services")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ErrorRootCauseService>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorRootCauseService {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "EntityPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_path: Option<Vec<ErrorRootCauseEntity>>,
    #[serde(rename = "Inferred")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inferred: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Names")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorRootCauseEntity {
    #[serde(rename = "Exceptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptions: Option<Vec<RootCauseException>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Remote")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RootCauseException {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FaultRootCause {
    #[serde(rename = "ClientImpacting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_impacting: Option<bool>,
    #[serde(rename = "Services")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<FaultRootCauseService>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FaultRootCauseService {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "EntityPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_path: Option<Vec<FaultRootCauseEntity>>,
    #[serde(rename = "Inferred")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inferred: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Names")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FaultRootCauseEntity {
    #[serde(rename = "Exceptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptions: Option<Vec<RootCauseException>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Remote")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Http {
    #[serde(rename = "ClientIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
    #[serde(rename = "HttpMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    #[serde(rename = "HttpStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status: Option<i32>,
    #[serde(rename = "HttpURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_u_r_l: Option<String>,
    #[serde(rename = "UserAgent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceIdDetail {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceARNDetail {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResponseTimeRootCause {
    #[serde(rename = "ClientImpacting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_impacting: Option<bool>,
    #[serde(rename = "Services")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ResponseTimeRootCauseService>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResponseTimeRootCauseService {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "EntityPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_path: Option<Vec<ResponseTimeRootCauseEntity>>,
    #[serde(rename = "Inferred")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inferred: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Names")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResponseTimeRootCauseEntity {
    #[serde(rename = "Coverage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Remote")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TraceUser {
    #[serde(rename = "ServiceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_ids: Option<Vec<ServiceId>>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourcePoliciesRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourcePoliciesResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourcePolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policies: Option<Vec<ResourcePolicy>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourcePolicy {
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "PolicyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "PolicyRevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRetrievedTracesRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RetrievalToken")]
    #[serde(default)]
    pub retrieval_token: String,
    #[serde(rename = "TraceFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_format: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRetrievedTracesResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RetrievalStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_status: Option<String>,
    #[serde(rename = "TraceFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_format: Option<String>,
    #[serde(rename = "Traces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traces: Option<Vec<RetrievedTrace>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetrievedTrace {
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Spans")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spans: Option<Vec<Span>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Span {
    #[serde(rename = "Document")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEncryptionConfigRequest {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEncryptionConfigResult {
    #[serde(rename = "EncryptionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyRequest {
    #[serde(rename = "BypassPolicyLockoutCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_policy_lockout_check: Option<bool>,
    #[serde(rename = "PolicyDocument")]
    #[serde(default)]
    pub policy_document: String,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(rename = "PolicyRevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyResult {
    #[serde(rename = "ResourcePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policy: Option<ResourcePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTelemetryRecordsRequest {
    #[serde(rename = "EC2InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_instance_id: Option<String>,
    #[serde(rename = "Hostname")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
    #[serde(rename = "TelemetryRecords")]
    #[serde(default)]
    pub telemetry_records: Vec<TelemetryRecord>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TelemetryRecord {
    #[serde(rename = "BackendConnectionErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_connection_errors: Option<BackendConnectionErrors>,
    #[serde(rename = "SegmentsReceivedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments_received_count: Option<i32>,
    #[serde(rename = "SegmentsRejectedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments_rejected_count: Option<i32>,
    #[serde(rename = "SegmentsSentCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments_sent_count: Option<i32>,
    #[serde(rename = "SegmentsSpilloverCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments_spillover_count: Option<i32>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    pub timestamp: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackendConnectionErrors {
    #[serde(rename = "ConnectionRefusedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_refused_count: Option<i32>,
    #[serde(rename = "HTTPCode4XXCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_t_t_p_code4_x_x_count: Option<i32>,
    #[serde(rename = "HTTPCode5XXCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_t_t_p_code5_x_x_count: Option<i32>,
    #[serde(rename = "OtherCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_count: Option<i32>,
    #[serde(rename = "TimeoutCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_count: Option<i32>,
    #[serde(rename = "UnknownHostCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_host_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTelemetryRecordsResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTraceSegmentsRequest {
    #[serde(rename = "TraceSegmentDocuments")]
    #[serde(default)]
    pub trace_segment_documents: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTraceSegmentsResult {
    #[serde(rename = "UnprocessedTraceSegments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_trace_segments: Option<Vec<UnprocessedTraceSegment>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnprocessedTraceSegment {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTraceRetrievalRequest {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: f64,
    #[serde(rename = "TraceIds")]
    #[serde(default)]
    pub trace_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTraceRetrievalResult {
    #[serde(rename = "RetrievalToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_token: Option<String>,
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
pub struct UpdateGroupRequest {
    #[serde(rename = "FilterExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    #[serde(rename = "GroupARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_a_r_n: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "InsightsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights_configuration: Option<InsightsConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGroupResult {
    #[serde(rename = "Group")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIndexingRuleRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Rule")]
    #[serde(default)]
    pub rule: IndexingRuleValueUpdate,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IndexingRuleValueUpdate {
    #[serde(rename = "Probabilistic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probabilistic: Option<ProbabilisticRuleValueUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProbabilisticRuleValueUpdate {
    #[serde(rename = "DesiredSamplingPercentage")]
    #[serde(default)]
    pub desired_sampling_percentage: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIndexingRuleResult {
    #[serde(rename = "IndexingRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexing_rule: Option<IndexingRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSamplingRuleRequest {
    #[serde(rename = "SamplingRuleUpdate")]
    #[serde(default)]
    pub sampling_rule_update: SamplingRuleUpdate,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SamplingRuleUpdate {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "FixedRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_rate: Option<f64>,
    #[serde(rename = "HTTPMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_t_t_p_method: Option<String>,
    #[serde(rename = "Host")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "ReservoirSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservoir_size: Option<i32>,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
    #[serde(rename = "RuleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_a_r_n: Option<String>,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[serde(rename = "SamplingRateBoost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rate_boost: Option<SamplingRateBoost>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "ServiceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    #[serde(rename = "URLPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_r_l_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSamplingRuleResult {
    #[serde(rename = "SamplingRuleRecord")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rule_record: Option<SamplingRuleRecord>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTraceSegmentDestinationRequest {
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTraceSegmentDestinationResult {
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
