//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-costexplorer

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAnomalyMonitorRequest {
    #[serde(rename = "AnomalyMonitor")]
    #[serde(default)]
    pub anomaly_monitor: AnomalyMonitor,
    #[serde(rename = "ResourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<ResourceTag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnomalyMonitor {
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "DimensionalValueCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensional_value_count: Option<i32>,
    #[serde(rename = "LastEvaluatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_date: Option<String>,
    #[serde(rename = "LastUpdatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<String>,
    #[serde(rename = "MonitorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_arn: Option<String>,
    #[serde(rename = "MonitorDimension")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_dimension: Option<String>,
    #[serde(rename = "MonitorName")]
    #[serde(default)]
    pub monitor_name: String,
    #[serde(rename = "MonitorSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_specification: Option<Expression>,
    #[serde(rename = "MonitorType")]
    #[serde(default)]
    pub monitor_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Expression {
    #[serde(rename = "And")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<Expression>>,
    #[serde(rename = "CostCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_categories: Option<CostCategoryValues>,
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<DimensionValues>,
    #[serde(rename = "Not")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<Expression>>,
    #[serde(rename = "Or")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Expression>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagValues>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CostCategoryValues {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "MatchOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_options: Option<Vec<String>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DimensionValues {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "MatchOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_options: Option<Vec<String>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagValues {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "MatchOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_options: Option<Vec<String>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceTag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAnomalyMonitorResponse {
    #[serde(rename = "MonitorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAnomalySubscriptionRequest {
    #[serde(rename = "AnomalySubscription")]
    #[serde(default)]
    pub anomaly_subscription: AnomalySubscription,
    #[serde(rename = "ResourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<ResourceTag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnomalySubscription {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Frequency")]
    #[serde(default)]
    pub frequency: String,
    #[serde(rename = "MonitorArnList")]
    #[serde(default)]
    pub monitor_arn_list: Vec<String>,
    #[serde(rename = "Subscribers")]
    #[serde(default)]
    pub subscribers: Vec<Subscriber>,
    #[serde(rename = "SubscriptionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_arn: Option<String>,
    #[serde(rename = "SubscriptionName")]
    #[serde(default)]
    pub subscription_name: String,
    #[serde(rename = "Threshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[serde(rename = "ThresholdExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_expression: Option<Expression>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Subscriber {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
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
pub struct CreateAnomalySubscriptionResponse {
    #[serde(rename = "SubscriptionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCostCategoryDefinitionRequest {
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "EffectiveStart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_start: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ResourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<ResourceTag>>,
    #[serde(rename = "RuleVersion")]
    #[serde(default)]
    pub rule_version: String,
    #[serde(rename = "Rules")]
    #[serde(default)]
    pub rules: Vec<CostCategoryRule>,
    #[serde(rename = "SplitChargeRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_charge_rules: Option<Vec<CostCategorySplitChargeRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CostCategoryRule {
    #[serde(rename = "InheritedValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited_value: Option<CostCategoryInheritedValueDimension>,
    #[serde(rename = "Rule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Expression>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CostCategoryInheritedValueDimension {
    #[serde(rename = "DimensionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_key: Option<String>,
    #[serde(rename = "DimensionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CostCategorySplitChargeRule {
    #[serde(rename = "Method")]
    #[serde(default)]
    pub method: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<CostCategorySplitChargeRuleParameter>>,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: String,
    #[serde(rename = "Targets")]
    #[serde(default)]
    pub targets: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CostCategorySplitChargeRuleParameter {
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCostCategoryDefinitionResponse {
    #[serde(rename = "CostCategoryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category_arn: Option<String>,
    #[serde(rename = "EffectiveStart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_start: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAnomalyMonitorRequest {
    #[serde(rename = "MonitorArn")]
    #[serde(default)]
    pub monitor_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAnomalyMonitorResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAnomalySubscriptionRequest {
    #[serde(rename = "SubscriptionArn")]
    #[serde(default)]
    pub subscription_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAnomalySubscriptionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCostCategoryDefinitionRequest {
    #[serde(rename = "CostCategoryArn")]
    #[serde(default)]
    pub cost_category_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCostCategoryDefinitionResponse {
    #[serde(rename = "CostCategoryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category_arn: Option<String>,
    #[serde(rename = "EffectiveEnd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_end: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCostCategoryDefinitionRequest {
    #[serde(rename = "CostCategoryArn")]
    #[serde(default)]
    pub cost_category_arn: String,
    #[serde(rename = "EffectiveOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_on: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCostCategoryDefinitionResponse {
    #[serde(rename = "CostCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category: Option<CostCategory>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CostCategory {
    #[serde(rename = "CostCategoryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category_arn: Option<String>,
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "EffectiveEnd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_end: Option<String>,
    #[serde(rename = "EffectiveStart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_start: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ProcessingStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_status: Option<Vec<CostCategoryProcessingStatus>>,
    #[serde(rename = "RuleVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_version: Option<String>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<CostCategoryRule>>,
    #[serde(rename = "SplitChargeRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_charge_rules: Option<Vec<CostCategorySplitChargeRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CostCategoryProcessingStatus {
    #[serde(rename = "Component")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAnomaliesRequest {
    #[serde(rename = "DateInterval")]
    #[serde(default)]
    pub date_interval: AnomalyDateInterval,
    #[serde(rename = "Feedback")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MonitorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_arn: Option<String>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "TotalImpact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_impact: Option<TotalImpactFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnomalyDateInterval {
    #[serde(rename = "EndDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    pub start_date: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TotalImpactFilter {
    #[serde(rename = "EndValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_value: Option<f64>,
    #[serde(rename = "NumericOperator")]
    #[serde(default)]
    pub numeric_operator: String,
    #[serde(rename = "StartValue")]
    #[serde(default)]
    pub start_value: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAnomaliesResponse {
    #[serde(rename = "Anomalies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomalies: Option<Vec<Anomaly>>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Anomaly {
    #[serde(rename = "AnomalyEndDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_end_date: Option<String>,
    #[serde(rename = "AnomalyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_id: Option<String>,
    #[serde(rename = "AnomalyScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_score: Option<AnomalyScore>,
    #[serde(rename = "AnomalyStartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_start_date: Option<String>,
    #[serde(rename = "DimensionValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_value: Option<String>,
    #[serde(rename = "Feedback")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<String>,
    #[serde(rename = "Impact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: Option<Impact>,
    #[serde(rename = "MonitorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_arn: Option<String>,
    #[serde(rename = "RootCauses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_causes: Option<Vec<RootCause>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnomalyScore {
    #[serde(rename = "CurrentScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_score: Option<f64>,
    #[serde(rename = "MaxScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_score: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Impact {
    #[serde(rename = "MaxImpact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_impact: Option<f64>,
    #[serde(rename = "TotalActualSpend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_actual_spend: Option<f64>,
    #[serde(rename = "TotalExpectedSpend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_expected_spend: Option<f64>,
    #[serde(rename = "TotalImpact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_impact: Option<f64>,
    #[serde(rename = "TotalImpactPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_impact_percentage: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RootCause {
    #[serde(rename = "Impact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: Option<RootCauseImpact>,
    #[serde(rename = "LinkedAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_account: Option<String>,
    #[serde(rename = "LinkedAccountName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_account_name: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "Service")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(rename = "UsageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RootCauseImpact {
    #[serde(rename = "Contribution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contribution: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAnomalyMonitorsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MonitorArnList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_arn_list: Option<Vec<String>>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAnomalyMonitorsResponse {
    #[serde(rename = "AnomalyMonitors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_monitors: Option<Vec<AnomalyMonitor>>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAnomalySubscriptionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MonitorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_arn: Option<String>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "SubscriptionArnList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_arn_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAnomalySubscriptionsResponse {
    #[serde(rename = "AnomalySubscriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_subscriptions: Option<Vec<AnomalySubscription>>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApproximateUsageRecordsRequest {
    #[serde(rename = "ApproximationDimension")]
    #[serde(default)]
    pub approximation_dimension: String,
    #[serde(rename = "Granularity")]
    #[serde(default)]
    pub granularity: String,
    #[serde(rename = "Services")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApproximateUsageRecordsResponse {
    #[serde(rename = "LookbackPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookback_period: Option<DateInterval>,
    #[serde(rename = "Services")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<std::collections::HashMap<String, i64>>,
    #[serde(rename = "TotalRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_records: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateInterval {
    #[serde(rename = "End")]
    #[serde(default)]
    pub end: String,
    #[serde(rename = "Start")]
    #[serde(default)]
    pub start: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCommitmentPurchaseAnalysisRequest {
    #[serde(rename = "AnalysisId")]
    #[serde(default)]
    pub analysis_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCommitmentPurchaseAnalysisResponse {
    #[serde(rename = "AnalysisCompletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_completion_time: Option<String>,
    #[serde(rename = "AnalysisDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_details: Option<AnalysisDetails>,
    #[serde(rename = "AnalysisId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_id: Option<String>,
    #[serde(rename = "AnalysisStartedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_started_time: Option<String>,
    #[serde(rename = "AnalysisStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_status: Option<String>,
    #[serde(rename = "CommitmentPurchaseAnalysisConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment_purchase_analysis_configuration: Option<CommitmentPurchaseAnalysisConfiguration>,
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "EstimatedCompletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_completion_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalysisDetails {
    #[serde(rename = "SavingsPlansPurchaseAnalysisDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_purchase_analysis_details: Option<SavingsPlansPurchaseAnalysisDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlansPurchaseAnalysisDetails {
    #[serde(rename = "AdditionalMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_metadata: Option<String>,
    #[serde(rename = "CurrencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "CurrentAverageCoverage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_average_coverage: Option<String>,
    #[serde(rename = "CurrentAverageHourlyOnDemandSpend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_average_hourly_on_demand_spend: Option<String>,
    #[serde(rename = "CurrentMaximumHourlyOnDemandSpend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_maximum_hourly_on_demand_spend: Option<String>,
    #[serde(rename = "CurrentMinimumHourlyOnDemandSpend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_minimum_hourly_on_demand_spend: Option<String>,
    #[serde(rename = "CurrentOnDemandSpend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_on_demand_spend: Option<String>,
    #[serde(rename = "EstimatedAverageCoverage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_average_coverage: Option<String>,
    #[serde(rename = "EstimatedAverageUtilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_average_utilization: Option<String>,
    #[serde(rename = "EstimatedCommitmentCost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_commitment_cost: Option<String>,
    #[serde(rename = "EstimatedMonthlySavingsAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_savings_amount: Option<String>,
    #[serde(rename = "EstimatedOnDemandCost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_on_demand_cost: Option<String>,
    #[serde(rename = "EstimatedOnDemandCostWithCurrentCommitment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_on_demand_cost_with_current_commitment: Option<String>,
    #[serde(rename = "EstimatedROI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_r_o_i: Option<String>,
    #[serde(rename = "EstimatedSavingsAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_savings_amount: Option<String>,
    #[serde(rename = "EstimatedSavingsPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_savings_percentage: Option<String>,
    #[serde(rename = "ExistingHourlyCommitment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing_hourly_commitment: Option<String>,
    #[serde(rename = "HourlyCommitmentToPurchase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly_commitment_to_purchase: Option<String>,
    #[serde(rename = "LatestUsageTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_usage_timestamp: Option<String>,
    #[serde(rename = "LookbackPeriodInHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookback_period_in_hours: Option<String>,
    #[serde(rename = "MetricsOverLookbackPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_over_lookback_period: Option<Vec<RecommendationDetailHourlyMetrics>>,
    #[serde(rename = "UpfrontCost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upfront_cost: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendationDetailHourlyMetrics {
    #[serde(rename = "CurrentCoverage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_coverage: Option<String>,
    #[serde(rename = "EstimatedCoverage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_coverage: Option<String>,
    #[serde(rename = "EstimatedNewCommitmentUtilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_new_commitment_utilization: Option<String>,
    #[serde(rename = "EstimatedOnDemandCost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_on_demand_cost: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommitmentPurchaseAnalysisConfiguration {
    #[serde(rename = "SavingsPlansPurchaseAnalysisConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_purchase_analysis_configuration:
        Option<SavingsPlansPurchaseAnalysisConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlansPurchaseAnalysisConfiguration {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AccountScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_scope: Option<String>,
    #[serde(rename = "AnalysisType")]
    #[serde(default)]
    pub analysis_type: String,
    #[serde(rename = "LookBackTimePeriod")]
    #[serde(default)]
    pub look_back_time_period: DateInterval,
    #[serde(rename = "SavingsPlansToAdd")]
    #[serde(default)]
    pub savings_plans_to_add: Vec<SavingsPlans>,
    #[serde(rename = "SavingsPlansToExclude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_to_exclude: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlans {
    #[serde(rename = "InstanceFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_family: Option<String>,
    #[serde(rename = "OfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_id: Option<String>,
    #[serde(rename = "PaymentOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "SavingsPlansCommitment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_commitment: Option<f64>,
    #[serde(rename = "SavingsPlansType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_type: Option<String>,
    #[serde(rename = "TermInYears")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_in_years: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCostAndUsageComparisonsRequest {
    #[serde(rename = "BaselineTimePeriod")]
    #[serde(default)]
    pub baseline_time_period: DateInterval,
    #[serde(rename = "BillingViewArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_view_arn: Option<String>,
    #[serde(rename = "ComparisonTimePeriod")]
    #[serde(default)]
    pub comparison_time_period: DateInterval,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    #[serde(rename = "GroupBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<GroupDefinition>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MetricForComparison")]
    #[serde(default)]
    pub metric_for_comparison: String,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupDefinition {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCostAndUsageComparisonsResponse {
    #[serde(rename = "CostAndUsageComparisons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_and_usage_comparisons: Option<Vec<CostAndUsageComparison>>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "TotalCostAndUsage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cost_and_usage: Option<std::collections::HashMap<String, ComparisonMetricValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CostAndUsageComparison {
    #[serde(rename = "CostAndUsageSelector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_and_usage_selector: Option<Expression>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<std::collections::HashMap<String, ComparisonMetricValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComparisonMetricValue {
    #[serde(rename = "BaselineTimePeriodAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_time_period_amount: Option<String>,
    #[serde(rename = "ComparisonTimePeriodAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_time_period_amount: Option<String>,
    #[serde(rename = "Difference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difference: Option<String>,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCostAndUsageRequest {
    #[serde(rename = "BillingViewArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_view_arn: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    #[serde(rename = "Granularity")]
    #[serde(default)]
    pub granularity: String,
    #[serde(rename = "GroupBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<GroupDefinition>>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    pub metrics: Vec<String>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "TimePeriod")]
    #[serde(default)]
    pub time_period: DateInterval,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCostAndUsageResponse {
    #[serde(rename = "DimensionValueAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_value_attributes: Option<Vec<DimensionValuesWithAttributes>>,
    #[serde(rename = "GroupDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_definitions: Option<Vec<GroupDefinition>>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "ResultsByTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results_by_time: Option<Vec<ResultByTime>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DimensionValuesWithAttributes {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResultByTime {
    #[serde(rename = "Estimated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated: Option<bool>,
    #[serde(rename = "Groups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<Group>>,
    #[serde(rename = "TimePeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<DateInterval>,
    #[serde(rename = "Total")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<std::collections::HashMap<String, MetricValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Group {
    #[serde(rename = "Keys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<std::collections::HashMap<String, MetricValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricValue {
    #[serde(rename = "Amount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCostAndUsageWithResourcesRequest {
    #[serde(rename = "BillingViewArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_view_arn: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    pub filter: Expression,
    #[serde(rename = "Granularity")]
    #[serde(default)]
    pub granularity: String,
    #[serde(rename = "GroupBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<GroupDefinition>>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<String>>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "TimePeriod")]
    #[serde(default)]
    pub time_period: DateInterval,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCostAndUsageWithResourcesResponse {
    #[serde(rename = "DimensionValueAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_value_attributes: Option<Vec<DimensionValuesWithAttributes>>,
    #[serde(rename = "GroupDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_definitions: Option<Vec<GroupDefinition>>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "ResultsByTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results_by_time: Option<Vec<ResultByTime>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCostCategoriesRequest {
    #[serde(rename = "BillingViewArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_view_arn: Option<String>,
    #[serde(rename = "CostCategoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category_name: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "SearchString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_string: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<Vec<SortDefinition>>,
    #[serde(rename = "TimePeriod")]
    #[serde(default)]
    pub time_period: DateInterval,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SortDefinition {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCostCategoriesResponse {
    #[serde(rename = "CostCategoryNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category_names: Option<Vec<String>>,
    #[serde(rename = "CostCategoryValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category_values: Option<Vec<String>>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "ReturnSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_size: Option<i32>,
    #[serde(rename = "TotalSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCostComparisonDriversRequest {
    #[serde(rename = "BaselineTimePeriod")]
    #[serde(default)]
    pub baseline_time_period: DateInterval,
    #[serde(rename = "BillingViewArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_view_arn: Option<String>,
    #[serde(rename = "ComparisonTimePeriod")]
    #[serde(default)]
    pub comparison_time_period: DateInterval,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    #[serde(rename = "GroupBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<GroupDefinition>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MetricForComparison")]
    #[serde(default)]
    pub metric_for_comparison: String,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCostComparisonDriversResponse {
    #[serde(rename = "CostComparisonDrivers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_comparison_drivers: Option<Vec<CostComparisonDriver>>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CostComparisonDriver {
    #[serde(rename = "CostDrivers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_drivers: Option<Vec<CostDriver>>,
    #[serde(rename = "CostSelector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_selector: Option<Expression>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<std::collections::HashMap<String, ComparisonMetricValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CostDriver {
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<std::collections::HashMap<String, ComparisonMetricValue>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCostForecastRequest {
    #[serde(rename = "BillingViewArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_view_arn: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    #[serde(rename = "Granularity")]
    #[serde(default)]
    pub granularity: String,
    #[serde(rename = "Metric")]
    #[serde(default)]
    pub metric: String,
    #[serde(rename = "PredictionIntervalLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction_interval_level: Option<i32>,
    #[serde(rename = "TimePeriod")]
    #[serde(default)]
    pub time_period: DateInterval,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCostForecastResponse {
    #[serde(rename = "ForecastResultsByTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_results_by_time: Option<Vec<ForecastResult>>,
    #[serde(rename = "Total")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<MetricValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ForecastResult {
    #[serde(rename = "MeanValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean_value: Option<String>,
    #[serde(rename = "PredictionIntervalLowerBound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction_interval_lower_bound: Option<String>,
    #[serde(rename = "PredictionIntervalUpperBound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction_interval_upper_bound: Option<String>,
    #[serde(rename = "TimePeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<DateInterval>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDimensionValuesRequest {
    #[serde(rename = "BillingViewArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_view_arn: Option<String>,
    #[serde(rename = "Context")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "Dimension")]
    #[serde(default)]
    pub dimension: String,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "SearchString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_string: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<Vec<SortDefinition>>,
    #[serde(rename = "TimePeriod")]
    #[serde(default)]
    pub time_period: DateInterval,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDimensionValuesResponse {
    #[serde(rename = "DimensionValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_values: Option<Vec<DimensionValuesWithAttributes>>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "ReturnSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_size: Option<i32>,
    #[serde(rename = "TotalSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetReservationCoverageRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    #[serde(rename = "Granularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    #[serde(rename = "GroupBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<GroupDefinition>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<String>>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<SortDefinition>,
    #[serde(rename = "TimePeriod")]
    #[serde(default)]
    pub time_period: DateInterval,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetReservationCoverageResponse {
    #[serde(rename = "CoveragesByTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverages_by_time: Option<Vec<CoverageByTime>>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "Total")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<Coverage>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoverageByTime {
    #[serde(rename = "Groups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ReservationCoverageGroup>>,
    #[serde(rename = "TimePeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<DateInterval>,
    #[serde(rename = "Total")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<Coverage>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservationCoverageGroup {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Coverage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage: Option<Coverage>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Coverage {
    #[serde(rename = "CoverageCost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_cost: Option<CoverageCost>,
    #[serde(rename = "CoverageHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_hours: Option<CoverageHours>,
    #[serde(rename = "CoverageNormalizedUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_normalized_units: Option<CoverageNormalizedUnits>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoverageCost {
    #[serde(rename = "OnDemandCost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_cost: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoverageHours {
    #[serde(rename = "CoverageHoursPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_hours_percentage: Option<String>,
    #[serde(rename = "OnDemandHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_hours: Option<String>,
    #[serde(rename = "ReservedHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_hours: Option<String>,
    #[serde(rename = "TotalRunningHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_running_hours: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoverageNormalizedUnits {
    #[serde(rename = "CoverageNormalizedUnitsPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_normalized_units_percentage: Option<String>,
    #[serde(rename = "OnDemandNormalizedUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_normalized_units: Option<String>,
    #[serde(rename = "ReservedNormalizedUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_normalized_units: Option<String>,
    #[serde(rename = "TotalRunningNormalizedUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_running_normalized_units: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetReservationPurchaseRecommendationRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AccountScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_scope: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    #[serde(rename = "LookbackPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookback_period_in_days: Option<String>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PaymentOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    #[serde(rename = "Service")]
    #[serde(default)]
    pub service: String,
    #[serde(rename = "ServiceSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
    #[serde(rename = "TermInYears")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_in_years: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceSpecification {
    #[serde(rename = "EC2Specification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_specification: Option<EC2Specification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EC2Specification {
    #[serde(rename = "OfferingClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_class: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetReservationPurchaseRecommendationResponse {
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ReservationPurchaseRecommendationMetadata>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "Recommendations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<Vec<ReservationPurchaseRecommendation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservationPurchaseRecommendationMetadata {
    #[serde(rename = "AdditionalMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_metadata: Option<String>,
    #[serde(rename = "GenerationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_timestamp: Option<String>,
    #[serde(rename = "RecommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservationPurchaseRecommendation {
    #[serde(rename = "AccountScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_scope: Option<String>,
    #[serde(rename = "LookbackPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookback_period_in_days: Option<String>,
    #[serde(rename = "PaymentOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    #[serde(rename = "RecommendationDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_details: Option<Vec<ReservationPurchaseRecommendationDetail>>,
    #[serde(rename = "RecommendationSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_summary: Option<ReservationPurchaseRecommendationSummary>,
    #[serde(rename = "ServiceSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
    #[serde(rename = "TermInYears")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_in_years: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservationPurchaseRecommendationDetail {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AverageNormalizedUnitsUsedPerHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_normalized_units_used_per_hour: Option<String>,
    #[serde(rename = "AverageNumberOfCapacityUnitsUsedPerHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_number_of_capacity_units_used_per_hour: Option<String>,
    #[serde(rename = "AverageNumberOfInstancesUsedPerHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_number_of_instances_used_per_hour: Option<String>,
    #[serde(rename = "AverageUtilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_utilization: Option<String>,
    #[serde(rename = "CurrencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "EstimatedBreakEvenInMonths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_break_even_in_months: Option<String>,
    #[serde(rename = "EstimatedMonthlyOnDemandCost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_on_demand_cost: Option<String>,
    #[serde(rename = "EstimatedMonthlySavingsAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_savings_amount: Option<String>,
    #[serde(rename = "EstimatedMonthlySavingsPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_savings_percentage: Option<String>,
    #[serde(rename = "EstimatedReservationCostForLookbackPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_reservation_cost_for_lookback_period: Option<String>,
    #[serde(rename = "InstanceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_details: Option<InstanceDetails>,
    #[serde(rename = "MaximumNormalizedUnitsUsedPerHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_normalized_units_used_per_hour: Option<String>,
    #[serde(rename = "MaximumNumberOfCapacityUnitsUsedPerHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_number_of_capacity_units_used_per_hour: Option<String>,
    #[serde(rename = "MaximumNumberOfInstancesUsedPerHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_number_of_instances_used_per_hour: Option<String>,
    #[serde(rename = "MinimumNormalizedUnitsUsedPerHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_normalized_units_used_per_hour: Option<String>,
    #[serde(rename = "MinimumNumberOfCapacityUnitsUsedPerHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_number_of_capacity_units_used_per_hour: Option<String>,
    #[serde(rename = "MinimumNumberOfInstancesUsedPerHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_number_of_instances_used_per_hour: Option<String>,
    #[serde(rename = "RecommendedNormalizedUnitsToPurchase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_normalized_units_to_purchase: Option<String>,
    #[serde(rename = "RecommendedNumberOfCapacityUnitsToPurchase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_number_of_capacity_units_to_purchase: Option<String>,
    #[serde(rename = "RecommendedNumberOfInstancesToPurchase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_number_of_instances_to_purchase: Option<String>,
    #[serde(rename = "RecurringStandardMonthlyCost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_standard_monthly_cost: Option<String>,
    #[serde(rename = "ReservedCapacityDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_capacity_details: Option<ReservedCapacityDetails>,
    #[serde(rename = "UpfrontCost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upfront_cost: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceDetails {
    #[serde(rename = "EC2InstanceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_instance_details: Option<EC2InstanceDetails>,
    #[serde(rename = "ESInstanceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_s_instance_details: Option<ESInstanceDetails>,
    #[serde(rename = "ElastiCacheInstanceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasti_cache_instance_details: Option<ElastiCacheInstanceDetails>,
    #[serde(rename = "MemoryDBInstanceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_d_b_instance_details: Option<MemoryDBInstanceDetails>,
    #[serde(rename = "RDSInstanceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r_d_s_instance_details: Option<RDSInstanceDetails>,
    #[serde(rename = "RedshiftInstanceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_instance_details: Option<RedshiftInstanceDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EC2InstanceDetails {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "CurrentGeneration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_generation: Option<bool>,
    #[serde(rename = "Family")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "Platform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "SizeFlexEligible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_flex_eligible: Option<bool>,
    #[serde(rename = "Tenancy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenancy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ESInstanceDetails {
    #[serde(rename = "CurrentGeneration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_generation: Option<bool>,
    #[serde(rename = "InstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_class: Option<String>,
    #[serde(rename = "InstanceSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_size: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "SizeFlexEligible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_flex_eligible: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ElastiCacheInstanceDetails {
    #[serde(rename = "CurrentGeneration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_generation: Option<bool>,
    #[serde(rename = "Family")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "ProductDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "SizeFlexEligible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_flex_eligible: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MemoryDBInstanceDetails {
    #[serde(rename = "CurrentGeneration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_generation: Option<bool>,
    #[serde(rename = "Family")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "SizeFlexEligible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_flex_eligible: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RDSInstanceDetails {
    #[serde(rename = "CurrentGeneration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_generation: Option<bool>,
    #[serde(rename = "DatabaseEdition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_edition: Option<String>,
    #[serde(rename = "DatabaseEngine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_engine: Option<String>,
    #[serde(rename = "DeploymentModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_model: Option<String>,
    #[serde(rename = "DeploymentOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_option: Option<String>,
    #[serde(rename = "Family")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "SizeFlexEligible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_flex_eligible: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftInstanceDetails {
    #[serde(rename = "CurrentGeneration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_generation: Option<bool>,
    #[serde(rename = "Family")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "SizeFlexEligible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_flex_eligible: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservedCapacityDetails {
    #[serde(rename = "DynamoDBCapacityDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_d_b_capacity_details: Option<DynamoDBCapacityDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DynamoDBCapacityDetails {
    #[serde(rename = "CapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_units: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservationPurchaseRecommendationSummary {
    #[serde(rename = "CurrencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "TotalEstimatedMonthlySavingsAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_estimated_monthly_savings_amount: Option<String>,
    #[serde(rename = "TotalEstimatedMonthlySavingsPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_estimated_monthly_savings_percentage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetReservationUtilizationRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    #[serde(rename = "Granularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    #[serde(rename = "GroupBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<GroupDefinition>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<SortDefinition>,
    #[serde(rename = "TimePeriod")]
    #[serde(default)]
    pub time_period: DateInterval,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetReservationUtilizationResponse {
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "Total")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<ReservationAggregates>,
    #[serde(rename = "UtilizationsByTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilizations_by_time: Option<Vec<UtilizationByTime>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservationAggregates {
    #[serde(rename = "AmortizedRecurringFee")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amortized_recurring_fee: Option<String>,
    #[serde(rename = "AmortizedUpfrontFee")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amortized_upfront_fee: Option<String>,
    #[serde(rename = "NetRISavings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_r_i_savings: Option<String>,
    #[serde(rename = "OnDemandCostOfRIHoursUsed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_cost_of_r_i_hours_used: Option<String>,
    #[serde(rename = "PurchasedHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchased_hours: Option<String>,
    #[serde(rename = "PurchasedUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchased_units: Option<String>,
    #[serde(rename = "RICostForUnusedHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r_i_cost_for_unused_hours: Option<String>,
    #[serde(rename = "RealizedSavings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realized_savings: Option<String>,
    #[serde(rename = "TotalActualHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_actual_hours: Option<String>,
    #[serde(rename = "TotalActualUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_actual_units: Option<String>,
    #[serde(rename = "TotalAmortizedFee")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amortized_fee: Option<String>,
    #[serde(rename = "TotalPotentialRISavings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_potential_r_i_savings: Option<String>,
    #[serde(rename = "UnrealizedSavings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unrealized_savings: Option<String>,
    #[serde(rename = "UnusedHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_hours: Option<String>,
    #[serde(rename = "UnusedUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_units: Option<String>,
    #[serde(rename = "UtilizationPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization_percentage: Option<String>,
    #[serde(rename = "UtilizationPercentageInUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization_percentage_in_units: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UtilizationByTime {
    #[serde(rename = "Groups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ReservationUtilizationGroup>>,
    #[serde(rename = "TimePeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<DateInterval>,
    #[serde(rename = "Total")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<ReservationAggregates>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservationUtilizationGroup {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Utilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization: Option<ReservationAggregates>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRightsizingRecommendationRequest {
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<RightsizingRecommendationConfiguration>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "Service")]
    #[serde(default)]
    pub service: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RightsizingRecommendationConfiguration {
    #[serde(rename = "BenefitsConsidered")]
    #[serde(default)]
    pub benefits_considered: bool,
    #[serde(rename = "RecommendationTarget")]
    #[serde(default)]
    pub recommendation_target: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRightsizingRecommendationResponse {
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<RightsizingRecommendationConfiguration>,
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<RightsizingRecommendationMetadata>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "RightsizingRecommendations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rightsizing_recommendations: Option<Vec<RightsizingRecommendation>>,
    #[serde(rename = "Summary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<RightsizingRecommendationSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RightsizingRecommendationMetadata {
    #[serde(rename = "AdditionalMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_metadata: Option<String>,
    #[serde(rename = "GenerationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_timestamp: Option<String>,
    #[serde(rename = "LookbackPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookback_period_in_days: Option<String>,
    #[serde(rename = "RecommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RightsizingRecommendation {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "CurrentInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_instance: Option<CurrentInstance>,
    #[serde(rename = "FindingReasonCodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_reason_codes: Option<Vec<String>>,
    #[serde(rename = "ModifyRecommendationDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_recommendation_detail: Option<ModifyRecommendationDetail>,
    #[serde(rename = "RightsizingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rightsizing_type: Option<String>,
    #[serde(rename = "TerminateRecommendationDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_recommendation_detail: Option<TerminateRecommendationDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CurrentInstance {
    #[serde(rename = "CurrencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "InstanceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[serde(rename = "MonthlyCost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_cost: Option<String>,
    #[serde(rename = "OnDemandHoursInLookbackPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_hours_in_lookback_period: Option<String>,
    #[serde(rename = "ReservationCoveredHoursInLookbackPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_covered_hours_in_lookback_period: Option<String>,
    #[serde(rename = "ResourceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_details: Option<ResourceDetails>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ResourceUtilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_utilization: Option<ResourceUtilization>,
    #[serde(rename = "SavingsPlansCoveredHoursInLookbackPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_covered_hours_in_lookback_period: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagValues>>,
    #[serde(rename = "TotalRunningHoursInLookbackPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_running_hours_in_lookback_period: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceDetails {
    #[serde(rename = "EC2ResourceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_resource_details: Option<EC2ResourceDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EC2ResourceDetails {
    #[serde(rename = "HourlyOnDemandRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly_on_demand_rate: Option<String>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "Memory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    #[serde(rename = "NetworkPerformance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_performance: Option<String>,
    #[serde(rename = "Platform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "Sku")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(rename = "Storage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
    #[serde(rename = "Vcpu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpu: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceUtilization {
    #[serde(rename = "EC2ResourceUtilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_resource_utilization: Option<EC2ResourceUtilization>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EC2ResourceUtilization {
    #[serde(rename = "DiskResourceUtilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_resource_utilization: Option<DiskResourceUtilization>,
    #[serde(rename = "EBSResourceUtilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_b_s_resource_utilization: Option<EBSResourceUtilization>,
    #[serde(rename = "MaxCpuUtilizationPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cpu_utilization_percentage: Option<String>,
    #[serde(rename = "MaxMemoryUtilizationPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_memory_utilization_percentage: Option<String>,
    #[serde(rename = "MaxStorageUtilizationPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_utilization_percentage: Option<String>,
    #[serde(rename = "NetworkResourceUtilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_resource_utilization: Option<NetworkResourceUtilization>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DiskResourceUtilization {
    #[serde(rename = "DiskReadBytesPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_read_bytes_per_second: Option<String>,
    #[serde(rename = "DiskReadOpsPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_read_ops_per_second: Option<String>,
    #[serde(rename = "DiskWriteBytesPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_write_bytes_per_second: Option<String>,
    #[serde(rename = "DiskWriteOpsPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_write_ops_per_second: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EBSResourceUtilization {
    #[serde(rename = "EbsReadBytesPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_read_bytes_per_second: Option<String>,
    #[serde(rename = "EbsReadOpsPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_read_ops_per_second: Option<String>,
    #[serde(rename = "EbsWriteBytesPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_write_bytes_per_second: Option<String>,
    #[serde(rename = "EbsWriteOpsPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_write_ops_per_second: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkResourceUtilization {
    #[serde(rename = "NetworkInBytesPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_in_bytes_per_second: Option<String>,
    #[serde(rename = "NetworkOutBytesPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_out_bytes_per_second: Option<String>,
    #[serde(rename = "NetworkPacketsInPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_packets_in_per_second: Option<String>,
    #[serde(rename = "NetworkPacketsOutPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_packets_out_per_second: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyRecommendationDetail {
    #[serde(rename = "TargetInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_instances: Option<Vec<TargetInstance>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetInstance {
    #[serde(rename = "CurrencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "DefaultTargetInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_target_instance: Option<bool>,
    #[serde(rename = "EstimatedMonthlyCost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_cost: Option<String>,
    #[serde(rename = "EstimatedMonthlySavings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_savings: Option<String>,
    #[serde(rename = "ExpectedResourceUtilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_resource_utilization: Option<ResourceUtilization>,
    #[serde(rename = "PlatformDifferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_differences: Option<Vec<String>>,
    #[serde(rename = "ResourceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_details: Option<ResourceDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminateRecommendationDetail {
    #[serde(rename = "CurrencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "EstimatedMonthlySavings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_savings: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RightsizingRecommendationSummary {
    #[serde(rename = "EstimatedTotalMonthlySavingsAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_total_monthly_savings_amount: Option<String>,
    #[serde(rename = "SavingsCurrencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_currency_code: Option<String>,
    #[serde(rename = "SavingsPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_percentage: Option<String>,
    #[serde(rename = "TotalRecommendationCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_recommendation_count: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSavingsPlanPurchaseRecommendationDetailsRequest {
    #[serde(rename = "RecommendationDetailId")]
    #[serde(default)]
    pub recommendation_detail_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSavingsPlanPurchaseRecommendationDetailsResponse {
    #[serde(rename = "RecommendationDetailData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_detail_data: Option<RecommendationDetailData>,
    #[serde(rename = "RecommendationDetailId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_detail_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendationDetailData {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AccountScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_scope: Option<String>,
    #[serde(rename = "CurrencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "CurrentAverageCoverage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_average_coverage: Option<String>,
    #[serde(rename = "CurrentAverageHourlyOnDemandSpend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_average_hourly_on_demand_spend: Option<String>,
    #[serde(rename = "CurrentMaximumHourlyOnDemandSpend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_maximum_hourly_on_demand_spend: Option<String>,
    #[serde(rename = "CurrentMinimumHourlyOnDemandSpend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_minimum_hourly_on_demand_spend: Option<String>,
    #[serde(rename = "EstimatedAverageCoverage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_average_coverage: Option<String>,
    #[serde(rename = "EstimatedAverageUtilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_average_utilization: Option<String>,
    #[serde(rename = "EstimatedMonthlySavingsAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_savings_amount: Option<String>,
    #[serde(rename = "EstimatedOnDemandCost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_on_demand_cost: Option<String>,
    #[serde(rename = "EstimatedOnDemandCostWithCurrentCommitment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_on_demand_cost_with_current_commitment: Option<String>,
    #[serde(rename = "EstimatedROI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_r_o_i: Option<String>,
    #[serde(rename = "EstimatedSPCost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_s_p_cost: Option<String>,
    #[serde(rename = "EstimatedSavingsAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_savings_amount: Option<String>,
    #[serde(rename = "EstimatedSavingsPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_savings_percentage: Option<String>,
    #[serde(rename = "ExistingHourlyCommitment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing_hourly_commitment: Option<String>,
    #[serde(rename = "GenerationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_timestamp: Option<String>,
    #[serde(rename = "HourlyCommitmentToPurchase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly_commitment_to_purchase: Option<String>,
    #[serde(rename = "InstanceFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_family: Option<String>,
    #[serde(rename = "LatestUsageTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_usage_timestamp: Option<String>,
    #[serde(rename = "LookbackPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookback_period_in_days: Option<String>,
    #[serde(rename = "MetricsOverLookbackPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_over_lookback_period: Option<Vec<RecommendationDetailHourlyMetrics>>,
    #[serde(rename = "OfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_id: Option<String>,
    #[serde(rename = "PaymentOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "SavingsPlansType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_type: Option<String>,
    #[serde(rename = "TermInYears")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_in_years: Option<String>,
    #[serde(rename = "UpfrontCost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upfront_cost: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSavingsPlansCoverageRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    #[serde(rename = "Granularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    #[serde(rename = "GroupBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<GroupDefinition>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<SortDefinition>,
    #[serde(rename = "TimePeriod")]
    #[serde(default)]
    pub time_period: DateInterval,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSavingsPlansCoverageResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SavingsPlansCoverages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_coverages: Option<Vec<SavingsPlansCoverage>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlansCoverage {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Coverage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage: Option<SavingsPlansCoverageData>,
    #[serde(rename = "TimePeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<DateInterval>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlansCoverageData {
    #[serde(rename = "CoveragePercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_percentage: Option<String>,
    #[serde(rename = "OnDemandCost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_cost: Option<String>,
    #[serde(rename = "SpendCoveredBySavingsPlans")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spend_covered_by_savings_plans: Option<String>,
    #[serde(rename = "TotalCost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cost: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSavingsPlansPurchaseRecommendationRequest {
    #[serde(rename = "AccountScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_scope: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    #[serde(rename = "LookbackPeriodInDays")]
    #[serde(default)]
    pub lookback_period_in_days: String,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PaymentOption")]
    #[serde(default)]
    pub payment_option: String,
    #[serde(rename = "SavingsPlansType")]
    #[serde(default)]
    pub savings_plans_type: String,
    #[serde(rename = "TermInYears")]
    #[serde(default)]
    pub term_in_years: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSavingsPlansPurchaseRecommendationResponse {
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SavingsPlansPurchaseRecommendationMetadata>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "SavingsPlansPurchaseRecommendation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_purchase_recommendation: Option<SavingsPlansPurchaseRecommendation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlansPurchaseRecommendationMetadata {
    #[serde(rename = "AdditionalMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_metadata: Option<String>,
    #[serde(rename = "GenerationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_timestamp: Option<String>,
    #[serde(rename = "RecommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlansPurchaseRecommendation {
    #[serde(rename = "AccountScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_scope: Option<String>,
    #[serde(rename = "LookbackPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookback_period_in_days: Option<String>,
    #[serde(rename = "PaymentOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    #[serde(rename = "SavingsPlansPurchaseRecommendationDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_purchase_recommendation_details:
        Option<Vec<SavingsPlansPurchaseRecommendationDetail>>,
    #[serde(rename = "SavingsPlansPurchaseRecommendationSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_purchase_recommendation_summary:
        Option<SavingsPlansPurchaseRecommendationSummary>,
    #[serde(rename = "SavingsPlansType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_type: Option<String>,
    #[serde(rename = "TermInYears")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_in_years: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlansPurchaseRecommendationDetail {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "CurrencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "CurrentAverageHourlyOnDemandSpend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_average_hourly_on_demand_spend: Option<String>,
    #[serde(rename = "CurrentMaximumHourlyOnDemandSpend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_maximum_hourly_on_demand_spend: Option<String>,
    #[serde(rename = "CurrentMinimumHourlyOnDemandSpend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_minimum_hourly_on_demand_spend: Option<String>,
    #[serde(rename = "EstimatedAverageUtilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_average_utilization: Option<String>,
    #[serde(rename = "EstimatedMonthlySavingsAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_savings_amount: Option<String>,
    #[serde(rename = "EstimatedOnDemandCost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_on_demand_cost: Option<String>,
    #[serde(rename = "EstimatedOnDemandCostWithCurrentCommitment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_on_demand_cost_with_current_commitment: Option<String>,
    #[serde(rename = "EstimatedROI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_r_o_i: Option<String>,
    #[serde(rename = "EstimatedSPCost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_s_p_cost: Option<String>,
    #[serde(rename = "EstimatedSavingsAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_savings_amount: Option<String>,
    #[serde(rename = "EstimatedSavingsPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_savings_percentage: Option<String>,
    #[serde(rename = "HourlyCommitmentToPurchase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly_commitment_to_purchase: Option<String>,
    #[serde(rename = "RecommendationDetailId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_detail_id: Option<String>,
    #[serde(rename = "SavingsPlansDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_details: Option<SavingsPlansDetails>,
    #[serde(rename = "UpfrontCost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upfront_cost: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlansDetails {
    #[serde(rename = "InstanceFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_family: Option<String>,
    #[serde(rename = "OfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_id: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlansPurchaseRecommendationSummary {
    #[serde(rename = "CurrencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "CurrentOnDemandSpend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_on_demand_spend: Option<String>,
    #[serde(rename = "DailyCommitmentToPurchase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_commitment_to_purchase: Option<String>,
    #[serde(rename = "EstimatedMonthlySavingsAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_savings_amount: Option<String>,
    #[serde(rename = "EstimatedOnDemandCostWithCurrentCommitment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_on_demand_cost_with_current_commitment: Option<String>,
    #[serde(rename = "EstimatedROI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_r_o_i: Option<String>,
    #[serde(rename = "EstimatedSavingsAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_savings_amount: Option<String>,
    #[serde(rename = "EstimatedSavingsPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_savings_percentage: Option<String>,
    #[serde(rename = "EstimatedTotalCost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_total_cost: Option<String>,
    #[serde(rename = "HourlyCommitmentToPurchase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly_commitment_to_purchase: Option<String>,
    #[serde(rename = "TotalRecommendationCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_recommendation_count: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSavingsPlansUtilizationDetailsRequest {
    #[serde(rename = "DataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<Vec<String>>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<SortDefinition>,
    #[serde(rename = "TimePeriod")]
    #[serde(default)]
    pub time_period: DateInterval,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSavingsPlansUtilizationDetailsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SavingsPlansUtilizationDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_utilization_details: Option<Vec<SavingsPlansUtilizationDetail>>,
    #[serde(rename = "TimePeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<DateInterval>,
    #[serde(rename = "Total")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<SavingsPlansUtilizationAggregates>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlansUtilizationDetail {
    #[serde(rename = "AmortizedCommitment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amortized_commitment: Option<SavingsPlansAmortizedCommitment>,
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Savings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings: Option<SavingsPlansSavings>,
    #[serde(rename = "SavingsPlanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_arn: Option<String>,
    #[serde(rename = "Utilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization: Option<SavingsPlansUtilization>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlansAmortizedCommitment {
    #[serde(rename = "AmortizedRecurringCommitment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amortized_recurring_commitment: Option<String>,
    #[serde(rename = "AmortizedUpfrontCommitment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amortized_upfront_commitment: Option<String>,
    #[serde(rename = "TotalAmortizedCommitment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amortized_commitment: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlansSavings {
    #[serde(rename = "NetSavings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_savings: Option<String>,
    #[serde(rename = "OnDemandCostEquivalent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_cost_equivalent: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlansUtilization {
    #[serde(rename = "TotalCommitment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_commitment: Option<String>,
    #[serde(rename = "UnusedCommitment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_commitment: Option<String>,
    #[serde(rename = "UsedCommitment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_commitment: Option<String>,
    #[serde(rename = "UtilizationPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization_percentage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlansUtilizationAggregates {
    #[serde(rename = "AmortizedCommitment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amortized_commitment: Option<SavingsPlansAmortizedCommitment>,
    #[serde(rename = "Savings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings: Option<SavingsPlansSavings>,
    #[serde(rename = "Utilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization: Option<SavingsPlansUtilization>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSavingsPlansUtilizationRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    #[serde(rename = "Granularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<SortDefinition>,
    #[serde(rename = "TimePeriod")]
    #[serde(default)]
    pub time_period: DateInterval,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSavingsPlansUtilizationResponse {
    #[serde(rename = "SavingsPlansUtilizationsByTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_utilizations_by_time: Option<Vec<SavingsPlansUtilizationByTime>>,
    #[serde(rename = "Total")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<SavingsPlansUtilizationAggregates>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlansUtilizationByTime {
    #[serde(rename = "AmortizedCommitment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amortized_commitment: Option<SavingsPlansAmortizedCommitment>,
    #[serde(rename = "Savings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings: Option<SavingsPlansSavings>,
    #[serde(rename = "TimePeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<DateInterval>,
    #[serde(rename = "Utilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization: Option<SavingsPlansUtilization>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTagsRequest {
    #[serde(rename = "BillingViewArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_view_arn: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "SearchString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_string: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<Vec<SortDefinition>>,
    #[serde(rename = "TagKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    #[serde(rename = "TimePeriod")]
    #[serde(default)]
    pub time_period: DateInterval,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTagsResponse {
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "ReturnSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_size: Option<i32>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "TotalSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUsageForecastRequest {
    #[serde(rename = "BillingViewArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_view_arn: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    #[serde(rename = "Granularity")]
    #[serde(default)]
    pub granularity: String,
    #[serde(rename = "Metric")]
    #[serde(default)]
    pub metric: String,
    #[serde(rename = "PredictionIntervalLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction_interval_level: Option<i32>,
    #[serde(rename = "TimePeriod")]
    #[serde(default)]
    pub time_period: DateInterval,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUsageForecastResponse {
    #[serde(rename = "ForecastResultsByTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_results_by_time: Option<Vec<ForecastResult>>,
    #[serde(rename = "Total")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<MetricValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCommitmentPurchaseAnalysesRequest {
    #[serde(rename = "AnalysisIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_ids: Option<Vec<String>>,
    #[serde(rename = "AnalysisStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_status: Option<String>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCommitmentPurchaseAnalysesResponse {
    #[serde(rename = "AnalysisSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_summary_list: Option<Vec<AnalysisSummary>>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalysisSummary {
    #[serde(rename = "AnalysisCompletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_completion_time: Option<String>,
    #[serde(rename = "AnalysisId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_id: Option<String>,
    #[serde(rename = "AnalysisStartedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_started_time: Option<String>,
    #[serde(rename = "AnalysisStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_status: Option<String>,
    #[serde(rename = "CommitmentPurchaseAnalysisConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment_purchase_analysis_configuration: Option<CommitmentPurchaseAnalysisConfiguration>,
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "EstimatedCompletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_completion_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCostAllocationTagBackfillHistoryRequest {
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
pub struct ListCostAllocationTagBackfillHistoryResponse {
    #[serde(rename = "BackfillRequests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backfill_requests: Option<Vec<CostAllocationTagBackfillRequest>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CostAllocationTagBackfillRequest {
    #[serde(rename = "BackfillFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backfill_from: Option<String>,
    #[serde(rename = "BackfillStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backfill_status: Option<String>,
    #[serde(rename = "CompletedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(rename = "LastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(rename = "RequestedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCostAllocationTagsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCostAllocationTagsResponse {
    #[serde(rename = "CostAllocationTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_allocation_tags: Option<Vec<CostAllocationTag>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CostAllocationTag {
    #[serde(rename = "LastUpdatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<String>,
    #[serde(rename = "LastUsedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_date: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TagKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCostCategoryDefinitionsRequest {
    #[serde(rename = "EffectiveOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_on: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SupportedResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_resource_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCostCategoryDefinitionsResponse {
    #[serde(rename = "CostCategoryReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category_references: Option<Vec<CostCategoryReference>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CostCategoryReference {
    #[serde(rename = "CostCategoryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category_arn: Option<String>,
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "EffectiveEnd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_end: Option<String>,
    #[serde(rename = "EffectiveStart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_start: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NumberOfRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_rules: Option<i32>,
    #[serde(rename = "ProcessingStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_status: Option<Vec<CostCategoryProcessingStatus>>,
    #[serde(rename = "SupportedResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_resource_types: Option<Vec<String>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCostCategoryResourceAssociationsRequest {
    #[serde(rename = "CostCategoryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category_arn: Option<String>,
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
pub struct ListCostCategoryResourceAssociationsResponse {
    #[serde(rename = "CostCategoryResourceAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category_resource_associations: Option<Vec<CostCategoryResourceAssociation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CostCategoryResourceAssociation {
    #[serde(rename = "CostCategoryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category_arn: Option<String>,
    #[serde(rename = "CostCategoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category_name: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSavingsPlansPurchaseRecommendationGenerationRequest {
    #[serde(rename = "GenerationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_status: Option<String>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "RecommendationIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSavingsPlansPurchaseRecommendationGenerationResponse {
    #[serde(rename = "GenerationSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_summary_list: Option<Vec<GenerationSummary>>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerationSummary {
    #[serde(rename = "EstimatedCompletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_completion_time: Option<String>,
    #[serde(rename = "GenerationCompletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_completion_time: Option<String>,
    #[serde(rename = "GenerationStartedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_started_time: Option<String>,
    #[serde(rename = "GenerationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_status: Option<String>,
    #[serde(rename = "RecommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "ResourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<ResourceTag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvideAnomalyFeedbackRequest {
    #[serde(rename = "AnomalyId")]
    #[serde(default)]
    pub anomaly_id: String,
    #[serde(rename = "Feedback")]
    #[serde(default)]
    pub feedback: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvideAnomalyFeedbackResponse {
    #[serde(rename = "AnomalyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCommitmentPurchaseAnalysisRequest {
    #[serde(rename = "CommitmentPurchaseAnalysisConfiguration")]
    #[serde(default)]
    pub commitment_purchase_analysis_configuration: CommitmentPurchaseAnalysisConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCommitmentPurchaseAnalysisResponse {
    #[serde(rename = "AnalysisId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_id: Option<String>,
    #[serde(rename = "AnalysisStartedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_started_time: Option<String>,
    #[serde(rename = "EstimatedCompletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_completion_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCostAllocationTagBackfillRequest {
    #[serde(rename = "BackfillFrom")]
    #[serde(default)]
    pub backfill_from: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCostAllocationTagBackfillResponse {
    #[serde(rename = "BackfillRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backfill_request: Option<CostAllocationTagBackfillRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSavingsPlansPurchaseRecommendationGenerationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSavingsPlansPurchaseRecommendationGenerationResponse {
    #[serde(rename = "EstimatedCompletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_completion_time: Option<String>,
    #[serde(rename = "GenerationStartedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_started_time: Option<String>,
    #[serde(rename = "RecommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "ResourceTags")]
    #[serde(default)]
    pub resource_tags: Vec<ResourceTag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "ResourceTagKeys")]
    #[serde(default)]
    pub resource_tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAnomalyMonitorRequest {
    #[serde(rename = "MonitorArn")]
    #[serde(default)]
    pub monitor_arn: String,
    #[serde(rename = "MonitorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAnomalyMonitorResponse {
    #[serde(rename = "MonitorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAnomalySubscriptionRequest {
    #[serde(rename = "Frequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    #[serde(rename = "MonitorArnList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_arn_list: Option<Vec<String>>,
    #[serde(rename = "Subscribers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<Subscriber>>,
    #[serde(rename = "SubscriptionArn")]
    #[serde(default)]
    pub subscription_arn: String,
    #[serde(rename = "SubscriptionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
    #[serde(rename = "Threshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[serde(rename = "ThresholdExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_expression: Option<Expression>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAnomalySubscriptionResponse {
    #[serde(rename = "SubscriptionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCostAllocationTagsStatusRequest {
    #[serde(rename = "CostAllocationTagsStatus")]
    #[serde(default)]
    pub cost_allocation_tags_status: Vec<CostAllocationTagStatusEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CostAllocationTagStatusEntry {
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    #[serde(rename = "TagKey")]
    #[serde(default)]
    pub tag_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCostAllocationTagsStatusResponse {
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<UpdateCostAllocationTagsStatusError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCostAllocationTagsStatusError {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "TagKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCostCategoryDefinitionRequest {
    #[serde(rename = "CostCategoryArn")]
    #[serde(default)]
    pub cost_category_arn: String,
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "EffectiveStart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_start: Option<String>,
    #[serde(rename = "RuleVersion")]
    #[serde(default)]
    pub rule_version: String,
    #[serde(rename = "Rules")]
    #[serde(default)]
    pub rules: Vec<CostCategoryRule>,
    #[serde(rename = "SplitChargeRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_charge_rules: Option<Vec<CostCategorySplitChargeRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCostCategoryDefinitionResponse {
    #[serde(rename = "CostCategoryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_category_arn: Option<String>,
    #[serde(rename = "EffectiveStart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_start: Option<String>,
}
