//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-bcmdashboards

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDashboardRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "resourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<ResourceTag>>,
    #[serde(default)]
    pub widgets: Vec<Widget>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceTag {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Widget {
    #[serde(default)]
    pub configs: Vec<WidgetConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename = "horizontalOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_offset: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WidgetConfig {
    #[serde(rename = "displayConfig")]
    #[serde(default)]
    pub display_config: DisplayConfig,
    #[serde(rename = "queryParameters")]
    #[serde(default)]
    pub query_parameters: QueryParameters,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisplayConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph: Option<std::collections::HashMap<String, GraphDisplayConfig>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<TableDisplayConfigStruct>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GraphDisplayConfig {
    #[serde(rename = "visualType")]
    #[serde(default)]
    pub visual_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableDisplayConfigStruct {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryParameters {
    #[serde(rename = "costAndUsage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_and_usage: Option<CostAndUsageQuery>,
    #[serde(rename = "reservationCoverage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_coverage: Option<ReservationCoverageQuery>,
    #[serde(rename = "reservationUtilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_utilization: Option<ReservationUtilizationQuery>,
    #[serde(rename = "savingsPlansCoverage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_coverage: Option<SavingsPlansCoverageQuery>,
    #[serde(rename = "savingsPlansUtilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans_utilization: Option<SavingsPlansUtilizationQuery>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CostAndUsageQuery {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    #[serde(default)]
    pub granularity: String,
    #[serde(rename = "groupBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<GroupDefinition>>,
    #[serde(default)]
    pub metrics: Vec<String>,
    #[serde(rename = "timeRange")]
    #[serde(default)]
    pub time_range: DateTimeRange,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Expression {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<Expression>>,
    #[serde(rename = "costCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_categories: Option<CostCategoryValues>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<DimensionValues>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<Expression>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Expression>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagValues>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CostCategoryValues {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "matchOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_options: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DimensionValues {
    #[serde(default)]
    pub key: String,
    #[serde(rename = "matchOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_options: Option<Vec<String>>,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagValues {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "matchOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_options: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupDefinition {
    #[serde(default)]
    pub key: String,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateTimeRange {
    #[serde(rename = "endTime")]
    #[serde(default)]
    pub end_time: DateTimeValue,
    #[serde(rename = "startTime")]
    #[serde(default)]
    pub start_time: DateTimeValue,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateTimeValue {
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservationCoverageQuery {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    #[serde(rename = "groupBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<GroupDefinition>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<String>>,
    #[serde(rename = "timeRange")]
    #[serde(default)]
    pub time_range: DateTimeRange,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservationUtilizationQuery {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    #[serde(rename = "groupBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<GroupDefinition>>,
    #[serde(rename = "timeRange")]
    #[serde(default)]
    pub time_range: DateTimeRange,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlansCoverageQuery {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    #[serde(rename = "groupBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<GroupDefinition>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<String>>,
    #[serde(rename = "timeRange")]
    #[serde(default)]
    pub time_range: DateTimeRange,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlansUtilizationQuery {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    #[serde(rename = "timeRange")]
    #[serde(default)]
    pub time_range: DateTimeRange,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDashboardResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateScheduledReportRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "resourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<ResourceTag>>,
    #[serde(rename = "scheduledReport")]
    #[serde(default)]
    pub scheduled_report: ScheduledReportInput,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledReportInput {
    #[serde(rename = "dashboardArn")]
    #[serde(default)]
    pub dashboard_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "scheduleConfig")]
    #[serde(default)]
    pub schedule_config: ScheduleConfig,
    #[serde(rename = "scheduledReportExecutionRoleArn")]
    #[serde(default)]
    pub scheduled_report_execution_role_arn: String,
    #[serde(rename = "widgetDateRangeOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_date_range_override: Option<DateTimeRange>,
    #[serde(rename = "widgetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduleConfig {
    #[serde(rename = "scheduleExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "scheduleExpressionTimeZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression_time_zone: Option<String>,
    #[serde(rename = "schedulePeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_period: Option<SchedulePeriod>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchedulePeriod {
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateScheduledReportResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDashboardRequest {
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDashboardResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScheduledReportRequest {
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScheduledReportResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteScheduledReportRequest {
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "dryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteScheduledReportResponse {
    #[serde(rename = "executionTriggered")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_triggered: Option<bool>,
    #[serde(rename = "healthStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<HealthStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HealthStatus {
    #[serde(rename = "lastRefreshedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refreshed_at: Option<f64>,
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "statusReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reasons: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDashboardRequest {
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDashboardResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widgets: Option<Vec<Widget>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyResponse {
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetScheduledReportRequest {
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetScheduledReportResponse {
    #[serde(rename = "scheduledReport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_report: Option<ScheduledReport>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledReport {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "dashboardArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "healthStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<HealthStatus>,
    #[serde(rename = "lastExecutionAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execution_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "scheduleConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_config: Option<ScheduleConfig>,
    #[serde(rename = "scheduledReportExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_report_execution_role_arn: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
    #[serde(rename = "widgetDateRangeOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_date_range_override: Option<DateTimeRange>,
    #[serde(rename = "widgetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDashboardsRequest {
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
pub struct ListDashboardsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboards: Option<Vec<DashboardReference>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashboardReference {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListScheduledReportsRequest {
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
pub struct ListScheduledReportsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "scheduledReports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_reports: Option<Vec<ScheduledReportSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledReportSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "dashboardArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_arn: Option<String>,
    #[serde(rename = "healthStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<HealthStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "scheduleExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "scheduleExpressionTimeZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression_time_zone: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "widgetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "resourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<ResourceTag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "resourceTags")]
    #[serde(default)]
    pub resource_tags: Vec<ResourceTag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "resourceTagKeys")]
    #[serde(default)]
    pub resource_tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDashboardRequest {
    #[serde(default)]
    pub arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widgets: Option<Vec<Widget>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDashboardResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateScheduledReportRequest {
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "clearWidgetDateRangeOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_widget_date_range_override: Option<bool>,
    #[serde(rename = "clearWidgetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_widget_ids: Option<bool>,
    #[serde(rename = "dashboardArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "scheduleConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_config: Option<ScheduleConfig>,
    #[serde(rename = "scheduledReportExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_report_execution_role_arn: Option<String>,
    #[serde(rename = "widgetDateRangeOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_date_range_override: Option<DateTimeRange>,
    #[serde(rename = "widgetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateScheduledReportResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}
