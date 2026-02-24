//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-application-signals

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetServiceLevelObjectiveBudgetReportInput {
    #[serde(rename = "SloIds")]
    #[serde(default)]
    pub slo_ids: Vec<String>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    pub timestamp: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetServiceLevelObjectiveBudgetReportOutput {
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ServiceLevelObjectiveBudgetReportError>>,
    #[serde(rename = "Reports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports: Option<Vec<ServiceLevelObjectiveBudgetReport>>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceLevelObjectiveBudgetReportError {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceLevelObjectiveBudgetReport {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Attainment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attainment: Option<f64>,
    #[serde(rename = "BudgetRequestsRemaining")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_requests_remaining: Option<i32>,
    #[serde(rename = "BudgetSecondsRemaining")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_seconds_remaining: Option<i32>,
    #[serde(rename = "BudgetStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_status: Option<String>,
    #[serde(rename = "EvaluationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_type: Option<String>,
    #[serde(rename = "Goal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal: Option<Goal>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RequestBasedSli")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_based_sli: Option<RequestBasedServiceLevelIndicator>,
    #[serde(rename = "Sli")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sli: Option<ServiceLevelIndicator>,
    #[serde(rename = "TotalBudgetRequests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_budget_requests: Option<i32>,
    #[serde(rename = "TotalBudgetSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_budget_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Goal {
    #[serde(rename = "AttainmentGoal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attainment_goal: Option<f64>,
    #[serde(rename = "Interval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<Interval>,
    #[serde(rename = "WarningThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning_threshold: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Interval {
    #[serde(rename = "CalendarInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_interval: Option<CalendarInterval>,
    #[serde(rename = "RollingInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_interval: Option<RollingInterval>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CalendarInterval {
    #[serde(rename = "Duration")]
    #[serde(default)]
    pub duration: i32,
    #[serde(rename = "DurationUnit")]
    #[serde(default)]
    pub duration_unit: String,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RollingInterval {
    #[serde(rename = "Duration")]
    #[serde(default)]
    pub duration: i32,
    #[serde(rename = "DurationUnit")]
    #[serde(default)]
    pub duration_unit: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestBasedServiceLevelIndicator {
    #[serde(rename = "ComparisonOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    #[serde(rename = "MetricThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_threshold: Option<f64>,
    #[serde(rename = "RequestBasedSliMetric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_based_sli_metric: Option<RequestBasedServiceLevelIndicatorMetric>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestBasedServiceLevelIndicatorMetric {
    #[serde(rename = "DependencyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency_config: Option<DependencyConfig>,
    #[serde(rename = "KeyAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "MetricType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_type: Option<String>,
    #[serde(rename = "MonitoredRequestCountMetric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitored_request_count_metric: Option<MonitoredRequestCountMetricDataQueries>,
    #[serde(rename = "OperationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[serde(rename = "TotalRequestCountMetric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_request_count_metric: Option<Vec<MetricDataQuery>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DependencyConfig {
    #[serde(rename = "DependencyKeyAttributes")]
    #[serde(default)]
    pub dependency_key_attributes: std::collections::HashMap<String, String>,
    #[serde(rename = "DependencyOperationName")]
    #[serde(default)]
    pub dependency_operation_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MonitoredRequestCountMetricDataQueries {
    #[serde(rename = "BadCountMetric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bad_count_metric: Option<Vec<MetricDataQuery>>,
    #[serde(rename = "GoodCountMetric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub good_count_metric: Option<Vec<MetricDataQuery>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricDataQuery {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Expression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Label")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "MetricStat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_stat: Option<MetricStat>,
    #[serde(rename = "Period")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(rename = "ReturnData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_data: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricStat {
    #[serde(rename = "Metric")]
    #[serde(default)]
    pub metric: Metric,
    #[serde(rename = "Period")]
    #[serde(default)]
    pub period: i32,
    #[serde(rename = "Stat")]
    #[serde(default)]
    pub stat: String,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Metric {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<Dimension>>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Dimension {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceLevelIndicator {
    #[serde(rename = "ComparisonOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    #[serde(rename = "MetricThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_threshold: Option<f64>,
    #[serde(rename = "SliMetric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sli_metric: Option<ServiceLevelIndicatorMetric>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceLevelIndicatorMetric {
    #[serde(rename = "DependencyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency_config: Option<DependencyConfig>,
    #[serde(rename = "KeyAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "MetricDataQueries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_data_queries: Option<Vec<MetricDataQuery>>,
    #[serde(rename = "MetricType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_type: Option<String>,
    #[serde(rename = "OperationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateExclusionWindowsInput {
    #[serde(rename = "AddExclusionWindows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_exclusion_windows: Option<Vec<ExclusionWindow>>,
    #[serde(rename = "RemoveExclusionWindows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_exclusion_windows: Option<Vec<ExclusionWindow>>,
    #[serde(rename = "SloIds")]
    #[serde(default)]
    pub slo_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExclusionWindow {
    #[serde(rename = "Reason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "RecurrenceRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence_rule: Option<RecurrenceRule>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Window")]
    #[serde(default)]
    pub window: Window,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecurrenceRule {
    #[serde(rename = "Expression")]
    #[serde(default)]
    pub expression: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Window {
    #[serde(rename = "Duration")]
    #[serde(default)]
    pub duration: i32,
    #[serde(rename = "DurationUnit")]
    #[serde(default)]
    pub duration_unit: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateExclusionWindowsOutput {
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchUpdateExclusionWindowsError>>,
    #[serde(rename = "SloIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slo_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateExclusionWindowsError {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "SloId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slo_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateServiceLevelObjectiveInput {
    #[serde(rename = "BurnRateConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burn_rate_configurations: Option<Vec<BurnRateConfiguration>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Goal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal: Option<Goal>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RequestBasedSliConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_based_sli_config: Option<RequestBasedServiceLevelIndicatorConfig>,
    #[serde(rename = "SliConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sli_config: Option<ServiceLevelIndicatorConfig>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BurnRateConfiguration {
    #[serde(rename = "LookBackWindowMinutes")]
    #[serde(default)]
    pub look_back_window_minutes: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestBasedServiceLevelIndicatorConfig {
    #[serde(rename = "ComparisonOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    #[serde(rename = "MetricThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_threshold: Option<f64>,
    #[serde(rename = "RequestBasedSliMetricConfig")]
    #[serde(default)]
    pub request_based_sli_metric_config: RequestBasedServiceLevelIndicatorMetricConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestBasedServiceLevelIndicatorMetricConfig {
    #[serde(rename = "DependencyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency_config: Option<DependencyConfig>,
    #[serde(rename = "KeyAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "MetricType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_type: Option<String>,
    #[serde(rename = "MonitoredRequestCountMetric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitored_request_count_metric: Option<MonitoredRequestCountMetricDataQueries>,
    #[serde(rename = "OperationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[serde(rename = "TotalRequestCountMetric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_request_count_metric: Option<Vec<MetricDataQuery>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceLevelIndicatorConfig {
    #[serde(rename = "ComparisonOperator")]
    #[serde(default)]
    pub comparison_operator: String,
    #[serde(rename = "MetricThreshold")]
    #[serde(default)]
    pub metric_threshold: f64,
    #[serde(rename = "SliMetricConfig")]
    #[serde(default)]
    pub sli_metric_config: ServiceLevelIndicatorMetricConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceLevelIndicatorMetricConfig {
    #[serde(rename = "DependencyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency_config: Option<DependencyConfig>,
    #[serde(rename = "KeyAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "MetricDataQueries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_data_queries: Option<Vec<MetricDataQuery>>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "MetricType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_type: Option<String>,
    #[serde(rename = "OperationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[serde(rename = "PeriodSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_seconds: Option<i32>,
    #[serde(rename = "Statistic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
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
pub struct CreateServiceLevelObjectiveOutput {
    #[serde(rename = "Slo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slo: Option<ServiceLevelObjective>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceLevelObjective {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "BurnRateConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burn_rate_configurations: Option<Vec<BurnRateConfiguration>>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EvaluationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_type: Option<String>,
    #[serde(rename = "Goal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal: Option<Goal>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "MetricSourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_source_type: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RequestBasedSli")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_based_sli: Option<RequestBasedServiceLevelIndicator>,
    #[serde(rename = "Sli")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sli: Option<ServiceLevelIndicator>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGroupingConfigurationOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceLevelObjectiveInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceLevelObjectiveOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceInput {
    #[serde(rename = "KeyAttributes")]
    #[serde(default)]
    pub key_attributes: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceLevelObjectiveInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceLevelObjectiveOutput {
    #[serde(rename = "Slo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slo: Option<ServiceLevelObjective>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceOutput {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "LogGroupReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_references: Option<Vec<std::collections::HashMap<String, String>>>,
    #[serde(rename = "Service")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Service {
    #[serde(rename = "AttributeMaps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_maps: Option<Vec<std::collections::HashMap<String, String>>>,
    #[serde(rename = "KeyAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "LogGroupReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_references: Option<Vec<std::collections::HashMap<String, String>>>,
    #[serde(rename = "MetricReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_references: Option<Vec<MetricReference>>,
    #[serde(rename = "ServiceGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_groups: Option<Vec<ServiceGroup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricReference {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<Dimension>>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "MetricType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_type: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceGroup {
    #[serde(rename = "GroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "GroupSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_source: Option<String>,
    #[serde(rename = "GroupValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAuditFindingsInput {
    #[serde(rename = "AuditTargets")]
    #[serde(default)]
    pub audit_targets: Vec<AuditTarget>,
    #[serde(rename = "Auditors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auditors: Option<Vec<String>>,
    #[serde(rename = "DetailLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_level: Option<String>,
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
pub struct AuditTarget {
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: AuditTargetEntity,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditTargetEntity {
    #[serde(rename = "Canary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary: Option<CanaryEntity>,
    #[serde(rename = "Service")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceEntity>,
    #[serde(rename = "ServiceOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_operation: Option<ServiceOperationEntity>,
    #[serde(rename = "Slo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slo: Option<ServiceLevelObjectiveEntity>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CanaryEntity {
    #[serde(rename = "CanaryName")]
    #[serde(default)]
    pub canary_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceEntity {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(rename = "Environment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
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
pub struct ServiceOperationEntity {
    #[serde(rename = "MetricType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_type: Option<String>,
    #[serde(rename = "Operation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "Service")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceEntity>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceLevelObjectiveEntity {
    #[serde(rename = "SloArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slo_arn: Option<String>,
    #[serde(rename = "SloName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slo_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAuditFindingsOutput {
    #[serde(rename = "AuditFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_findings: Option<Vec<AuditFinding>>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditFinding {
    #[serde(rename = "AuditorResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auditor_results: Option<Vec<AuditorResult>>,
    #[serde(rename = "DependencyGraph")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency_graph: Option<DependencyGraph>,
    #[serde(rename = "KeyAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "MetricGraph")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_graph: Option<MetricGraph>,
    #[serde(rename = "Operation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditorResult {
    #[serde(rename = "Auditor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auditor: Option<String>,
    #[serde(rename = "Data")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DependencyGraph {
    #[serde(rename = "Edges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<Edge>>,
    #[serde(rename = "Nodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Node>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Edge {
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[serde(rename = "DestinationNodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_node_id: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(rename = "SourceNodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_node_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Node {
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(rename = "KeyAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "Operation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
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
pub struct MetricGraph {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "MetricDataQueries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_data_queries: Option<Vec<MetricDataQuery>>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEntityEventsInput {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "Entity")]
    #[serde(default)]
    pub entity: std::collections::HashMap<String, String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEntityEventsOutput {
    #[serde(rename = "ChangeEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_events: Option<Vec<ChangeEvent>>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChangeEvent {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "ChangeEventType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_event_type: Option<String>,
    #[serde(rename = "Entity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "EventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(rename = "EventName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGroupingAttributeDefinitionsInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGroupingAttributeDefinitionsOutput {
    #[serde(rename = "GroupingAttributeDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping_attribute_definitions: Option<Vec<GroupingAttributeDefinition>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupingAttributeDefinition {
    #[serde(rename = "DefaultGroupingValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_grouping_value: Option<String>,
    #[serde(rename = "GroupingName")]
    #[serde(default)]
    pub grouping_name: String,
    #[serde(rename = "GroupingSourceKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping_source_keys: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceDependenciesInput {
    #[serde(rename = "KeyAttributes")]
    #[serde(default)]
    pub key_attributes: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceDependenciesOutput {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceDependencies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_dependencies: Option<Vec<ServiceDependency>>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceDependency {
    #[serde(rename = "DependencyKeyAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency_key_attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "DependencyOperationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency_operation_name: Option<String>,
    #[serde(rename = "MetricReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_references: Option<Vec<MetricReference>>,
    #[serde(rename = "OperationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceDependentsInput {
    #[serde(rename = "KeyAttributes")]
    #[serde(default)]
    pub key_attributes: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceDependentsOutput {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceDependents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_dependents: Option<Vec<ServiceDependent>>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceDependent {
    #[serde(rename = "DependentKeyAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependent_key_attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "DependentOperationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependent_operation_name: Option<String>,
    #[serde(rename = "MetricReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_references: Option<Vec<MetricReference>>,
    #[serde(rename = "OperationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceLevelObjectiveExclusionWindowsInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceLevelObjectiveExclusionWindowsOutput {
    #[serde(rename = "ExclusionWindows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_windows: Option<Vec<ExclusionWindow>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceLevelObjectivesInput {
    #[serde(rename = "DependencyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency_config: Option<DependencyConfig>,
    #[serde(rename = "KeyAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "MetricSourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_source_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceLevelObjectivesOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SloSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slo_summaries: Option<Vec<ServiceLevelObjectiveSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceLevelObjectiveSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "DependencyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency_config: Option<DependencyConfig>,
    #[serde(rename = "EvaluationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_type: Option<String>,
    #[serde(rename = "KeyAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "MetricSourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_source_type: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OperationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceOperationsInput {
    #[serde(rename = "KeyAttributes")]
    #[serde(default)]
    pub key_attributes: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceOperationsOutput {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_operations: Option<Vec<ServiceOperation>>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceOperation {
    #[serde(rename = "MetricReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_references: Option<Vec<MetricReference>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceStatesInput {
    #[serde(rename = "AttributeFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_filters: Option<Vec<AttributeFilter>>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "IncludeLinkedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_linked_accounts: Option<bool>,
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
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttributeFilter {
    #[serde(rename = "AttributeFilterName")]
    #[serde(default)]
    pub attribute_filter_name: String,
    #[serde(rename = "AttributeFilterValues")]
    #[serde(default)]
    pub attribute_filter_values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceStatesOutput {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceStates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_states: Option<Vec<ServiceState>>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceState {
    #[serde(rename = "AttributeFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_filters: Option<Vec<AttributeFilter>>,
    #[serde(rename = "LatestChangeEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_change_events: Option<Vec<ChangeEvent>>,
    #[serde(rename = "Service")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServicesInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServicesOutput {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_summaries: Option<Vec<ServiceSummary>>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceSummary {
    #[serde(rename = "AttributeMaps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_maps: Option<Vec<std::collections::HashMap<String, String>>>,
    #[serde(rename = "KeyAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "MetricReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_references: Option<Vec<MetricReference>>,
    #[serde(rename = "ServiceGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_groups: Option<Vec<ServiceGroup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutGroupingConfigurationInput {
    #[serde(rename = "GroupingAttributeDefinitions")]
    #[serde(default)]
    pub grouping_attribute_definitions: Vec<GroupingAttributeDefinition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutGroupingConfigurationOutput {
    #[serde(rename = "GroupingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping_configuration: Option<GroupingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupingConfiguration {
    #[serde(rename = "GroupingAttributeDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping_attribute_definitions: Option<Vec<GroupingAttributeDefinition>>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDiscoveryInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDiscoveryOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServiceLevelObjectiveInput {
    #[serde(rename = "BurnRateConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burn_rate_configurations: Option<Vec<BurnRateConfiguration>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Goal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal: Option<Goal>,
    #[serde(rename = "RequestBasedSliConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_based_sli_config: Option<RequestBasedServiceLevelIndicatorConfig>,
    #[serde(rename = "SliConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sli_config: Option<ServiceLevelIndicatorConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServiceLevelObjectiveOutput {
    #[serde(rename = "Slo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slo: Option<ServiceLevelObjective>,
}
