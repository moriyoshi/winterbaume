use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A raw metric data point, as submitted by BatchPutMetrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawMetricData {
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    #[serde(rename = "Timestamp")]
    pub timestamp: f64,
    #[serde(rename = "Step")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<i64>,
    #[serde(rename = "Value")]
    pub value: f64,
}

/// A stored metric data point with additional metadata.
#[derive(Debug, Clone)]
pub struct StoredMetric {
    pub trial_component_name: String,
    pub metric_name: String,
    pub timestamp: f64,
    pub step: Option<i64>,
    pub value: f64,
    pub ingested_at: DateTime<Utc>,
}

/// A metric query for BatchGetMetrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricQuery {
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    #[serde(rename = "MetricStat")]
    pub metric_stat: String,
    #[serde(rename = "Period")]
    pub period: String,
    #[serde(rename = "Start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[serde(rename = "End")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
    #[serde(rename = "XAxisType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_axis_type: Option<String>,
}

/// Result of a metric query in BatchGetMetrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricQueryResult {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "XAxisValues")]
    pub x_axis_values: Vec<f64>,
    #[serde(rename = "MetricValues")]
    pub metric_values: Vec<f64>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
