//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-sagemakermetrics

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetMetricsResponse {
    #[serde(rename = "MetricQueryResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_query_results: Option<Vec<MetricQueryResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricQueryResult {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "MetricValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_values: Option<Vec<f64>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "XAxisValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_axis_values: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchPutMetricsResponse {
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchPutMetricsError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchPutMetricsError {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "MetricIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_index: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetMetricsRequest {
    #[serde(rename = "MetricQueries")]
    #[serde(default)]
    pub metric_queries: Vec<MetricQuery>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricQuery {
    #[serde(rename = "End")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    pub metric_name: String,
    #[serde(rename = "MetricStat")]
    #[serde(default)]
    pub metric_stat: String,
    #[serde(rename = "Period")]
    #[serde(default)]
    pub period: String,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Start")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[serde(rename = "XAxisType")]
    #[serde(default)]
    pub x_axis_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchPutMetricsRequest {
    #[serde(rename = "MetricData")]
    #[serde(default)]
    pub metric_data: Vec<RawMetricData>,
    #[serde(rename = "TrialComponentName")]
    #[serde(default)]
    pub trial_component_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RawMetricData {
    #[serde(rename = "MetricName")]
    #[serde(default)]
    pub metric_name: String,
    #[serde(rename = "Step")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<i32>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    pub timestamp: f64,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: f64,
}
