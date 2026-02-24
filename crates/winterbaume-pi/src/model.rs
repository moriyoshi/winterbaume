//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-pi

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePerformanceAnalysisReportRequest {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "ServiceType")]
    #[serde(default)]
    pub service_type: String,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: f64,
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
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePerformanceAnalysisReportResponse {
    #[serde(rename = "AnalysisReportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_report_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePerformanceAnalysisReportRequest {
    #[serde(rename = "AnalysisReportId")]
    #[serde(default)]
    pub analysis_report_id: String,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "ServiceType")]
    #[serde(default)]
    pub service_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePerformanceAnalysisReportResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDimensionKeysRequest {
    #[serde(rename = "AdditionalMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_metrics: Option<Vec<String>>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "GroupBy")]
    #[serde(default)]
    pub group_by: DimensionGroup,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Metric")]
    #[serde(default)]
    pub metric: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PartitionBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_by: Option<DimensionGroup>,
    #[serde(rename = "PeriodInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_in_seconds: Option<i32>,
    #[serde(rename = "ServiceType")]
    #[serde(default)]
    pub service_type: String,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DimensionGroup {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<String>>,
    #[serde(rename = "Group")]
    #[serde(default)]
    pub group: String,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDimensionKeysResponse {
    #[serde(rename = "AlignedEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aligned_end_time: Option<f64>,
    #[serde(rename = "AlignedStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aligned_start_time: Option<f64>,
    #[serde(rename = "Keys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<DimensionKeyDescription>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PartitionKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_keys: Option<Vec<ResponsePartitionKey>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DimensionKeyDescription {
    #[serde(rename = "AdditionalMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_metrics: Option<std::collections::HashMap<String, f64>>,
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Partitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitions: Option<Vec<f64>>,
    #[serde(rename = "Total")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResponsePartitionKey {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDimensionKeyDetailsRequest {
    #[serde(rename = "Group")]
    #[serde(default)]
    pub group: String,
    #[serde(rename = "GroupIdentifier")]
    #[serde(default)]
    pub group_identifier: String,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "RequestedDimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_dimensions: Option<Vec<String>>,
    #[serde(rename = "ServiceType")]
    #[serde(default)]
    pub service_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDimensionKeyDetailsResponse {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<DimensionKeyDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DimensionKeyDetail {
    #[serde(rename = "Dimension")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPerformanceAnalysisReportRequest {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "AnalysisReportId")]
    #[serde(default)]
    pub analysis_report_id: String,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "ServiceType")]
    #[serde(default)]
    pub service_type: String,
    #[serde(rename = "TextFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_format: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPerformanceAnalysisReportResponse {
    #[serde(rename = "AnalysisReport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_report: Option<AnalysisReport>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalysisReport {
    #[serde(rename = "AnalysisReportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_report_id: Option<String>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "Insights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights: Option<Vec<Insight>>,
    #[serde(rename = "ServiceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Insight {
    #[serde(rename = "BaselineData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_data: Option<Vec<Data>>,
    #[serde(rename = "Context")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "InsightData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_data: Option<Vec<Data>>,
    #[serde(rename = "InsightId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_id: Option<String>,
    #[serde(rename = "InsightType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_type: Option<String>,
    #[serde(rename = "Recommendations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<Vec<Recommendation>>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "SupportingInsights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supporting_insights: Option<Vec<Insight>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Data {
    #[serde(rename = "PerformanceInsightsMetric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_metric: Option<PerformanceInsightsMetric>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PerformanceInsightsMetric {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Metric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Recommendation {
    #[serde(rename = "RecommendationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_description: Option<String>,
    #[serde(rename = "RecommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceMetadataRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "ServiceType")]
    #[serde(default)]
    pub service_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceMetadataResponse {
    #[serde(rename = "Features")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<std::collections::HashMap<String, FeatureMetadata>>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FeatureMetadata {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceMetricsRequest {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MetricQueries")]
    #[serde(default)]
    pub metric_queries: Vec<MetricQuery>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PeriodAlignment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_alignment: Option<String>,
    #[serde(rename = "PeriodInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_in_seconds: Option<i32>,
    #[serde(rename = "ServiceType")]
    #[serde(default)]
    pub service_type: String,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricQuery {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "GroupBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<DimensionGroup>,
    #[serde(rename = "Metric")]
    #[serde(default)]
    pub metric: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceMetricsResponse {
    #[serde(rename = "AlignedEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aligned_end_time: Option<f64>,
    #[serde(rename = "AlignedStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aligned_start_time: Option<f64>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "MetricList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_list: Option<Vec<MetricKeyDataPoints>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricKeyDataPoints {
    #[serde(rename = "DataPoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_points: Option<Vec<DataPoint>>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<ResponseResourceMetricKey>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataPoint {
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResponseResourceMetricKey {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Metric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAvailableResourceDimensionsRequest {
    #[serde(rename = "AuthorizedActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_actions: Option<Vec<String>>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    pub metrics: Vec<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceType")]
    #[serde(default)]
    pub service_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAvailableResourceDimensionsResponse {
    #[serde(rename = "MetricDimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_dimensions: Option<Vec<MetricDimensionGroups>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricDimensionGroups {
    #[serde(rename = "Groups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<DimensionGroupDetail>>,
    #[serde(rename = "Metric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DimensionGroupDetail {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<DimensionDetail>>,
    #[serde(rename = "Group")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DimensionDetail {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAvailableResourceMetricsRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MetricTypes")]
    #[serde(default)]
    pub metric_types: Vec<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceType")]
    #[serde(default)]
    pub service_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAvailableResourceMetricsResponse {
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<ResponseResourceMetric>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResponseResourceMetric {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Metric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPerformanceAnalysisReportsRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "ListTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_tags: Option<bool>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceType")]
    #[serde(default)]
    pub service_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPerformanceAnalysisReportsResponse {
    #[serde(rename = "AnalysisReports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_reports: Option<Vec<AnalysisReportSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalysisReportSummary {
    #[serde(rename = "AnalysisReportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_report_id: Option<String>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "ServiceType")]
    #[serde(default)]
    pub service_type: String,
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
    #[serde(rename = "ServiceType")]
    #[serde(default)]
    pub service_type: String,
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
    #[serde(rename = "ServiceType")]
    #[serde(default)]
    pub service_type: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}
