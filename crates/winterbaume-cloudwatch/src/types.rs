use chrono::{DateTime, Utc};

/// A CloudWatch metric data point.
#[derive(Debug, Clone)]
pub struct MetricDatum {
    pub namespace: String,
    pub metric_name: String,
    pub dimensions: Vec<Dimension>,
    pub value: Option<f64>,
    pub timestamp: DateTime<Utc>,
    pub unit: Option<String>,
}

/// A CloudWatch dimension.
#[derive(Debug, Clone)]
pub struct Dimension {
    pub name: String,
    pub value: String,
}

/// A CloudWatch metric alarm.
#[derive(Debug, Clone)]
pub struct MetricAlarm {
    pub alarm_name: String,
    pub alarm_arn: String,
    /// Absent for alarms defined through `Metrics` (metric math, anomaly detection).
    pub metric_name: Option<String>,
    pub namespace: Option<String>,
    /// Absent for anomaly-detection alarms, which use `ThresholdMetricId`.
    pub threshold: Option<f64>,
    pub comparison_operator: String,
    pub evaluation_periods: u32,
    pub period: Option<u32>,
    pub statistic: Option<String>,
    pub extended_statistic: Option<String>,
    pub datapoints_to_alarm: Option<u32>,
    /// Stored only when the caller sent it; AWS applies `missing` otherwise.
    pub treat_missing_data: Option<String>,
    pub evaluate_low_sample_count_percentile: Option<String>,
    pub threshold_metric_id: Option<String>,
    pub metrics: Vec<MetricDataQuery>,
    pub state_value: AlarmState,
    pub state_reason: String,
    pub state_reason_data: Option<String>,
    pub actions_enabled: bool,
    pub alarm_description: Option<String>,
    pub alarm_actions: Vec<String>,
    pub ok_actions: Vec<String>,
    pub insufficient_data_actions: Vec<String>,
    pub dimensions: Vec<Dimension>,
    pub unit: Option<String>,
    /// Last `PutMetricAlarm` that created or updated the configuration.
    pub alarm_configuration_updated_timestamp: DateTime<Utc>,
    /// Last update of the alarm state record ( `StateValue`, reason ).
    pub state_updated_timestamp: DateTime<Utc>,
    /// Last time `StateValue` changed.
    pub state_transitioned_timestamp: DateTime<Utc>,
}

/// One entry of a metric alarm's `Metrics` array ( `MetricDataQuery` ).
#[derive(Debug, Clone, Default)]
pub struct MetricDataQuery {
    pub id: String,
    pub metric_stat: Option<MetricStat>,
    pub expression: Option<String>,
    pub label: Option<String>,
    pub return_data: Option<bool>,
    pub period: Option<u32>,
    pub account_id: Option<String>,
}

/// The metric, period and statistic of a `MetricDataQuery`.
#[derive(Debug, Clone, Default)]
pub struct MetricStat {
    pub metric: Metric,
    pub period: u32,
    pub stat: String,
    pub unit: Option<String>,
}

/// Alarm state enum.
#[derive(Debug, Clone, PartialEq)]
pub enum AlarmState {
    Ok,
    Alarm,
    InsufficientData,
}

impl AlarmState {
    pub fn as_str(&self) -> &str {
        match self {
            AlarmState::Ok => "OK",
            AlarmState::Alarm => "ALARM",
            AlarmState::InsufficientData => "INSUFFICIENT_DATA",
        }
    }
}

/// Input parameters for the PutMetricAlarm operation.
pub struct PutMetricAlarmInput {
    pub alarm_name: String,
    pub metric_name: Option<String>,
    pub namespace: Option<String>,
    pub threshold: Option<f64>,
    pub comparison_operator: String,
    pub evaluation_periods: u32,
    pub period: Option<u32>,
    pub statistic: Option<String>,
    pub extended_statistic: Option<String>,
    pub datapoints_to_alarm: Option<u32>,
    pub treat_missing_data: Option<String>,
    pub evaluate_low_sample_count_percentile: Option<String>,
    pub threshold_metric_id: Option<String>,
    pub metrics: Vec<MetricDataQuery>,
    pub alarm_description: Option<String>,
    pub alarm_actions: Vec<String>,
    pub ok_actions: Vec<String>,
    pub insufficient_data_actions: Vec<String>,
    pub actions_enabled: bool,
    pub dimensions: Vec<Dimension>,
    pub unit: Option<String>,
    pub tags: std::collections::HashMap<String, String>,
}

/// A metric for ListMetrics.
#[derive(Debug, Clone, Default)]
pub struct Metric {
    pub namespace: String,
    pub metric_name: String,
    pub dimensions: Vec<Dimension>,
}

/// A CloudWatch dashboard.
#[derive(Debug, Clone)]
pub struct Dashboard {
    pub dashboard_name: String,
    pub dashboard_body: String,
    pub dashboard_arn: String,
    pub last_modified: DateTime<Utc>,
}

/// Insight rule state.
#[derive(Debug, Clone, PartialEq)]
pub enum InsightRuleState {
    Enabled,
    Disabled,
}

impl InsightRuleState {
    pub fn as_str(&self) -> &str {
        match self {
            InsightRuleState::Enabled => "ENABLED",
            InsightRuleState::Disabled => "DISABLED",
        }
    }
}

/// A CloudWatch Contributor Insights rule.
#[derive(Debug, Clone)]
pub struct InsightRule {
    pub name: String,
    pub state: InsightRuleState,
    pub schema: String,
    pub definition: String,
}

/// A CloudWatch composite alarm.
#[derive(Debug, Clone)]
pub struct CompositeAlarm {
    pub alarm_name: String,
    pub alarm_arn: String,
    pub alarm_rule: String,
    pub alarm_description: Option<String>,
    pub actions_enabled: bool,
    pub alarm_actions: Vec<String>,
    pub ok_actions: Vec<String>,
    pub insufficient_data_actions: Vec<String>,
    pub state_value: AlarmState,
    pub state_reason: String,
}

/// A CloudWatch anomaly detector.
#[derive(Debug, Clone)]
pub struct AnomalyDetector {
    pub namespace: String,
    pub metric_name: String,
    pub stat: String,
    pub dimensions: Vec<Dimension>,
    pub state_value: String,
}

/// A CloudWatch metric stream.
#[derive(Debug, Clone)]
pub struct MetricStream {
    pub name: String,
    pub arn: String,
    pub firehose_arn: String,
    pub role_arn: String,
    pub state: String,
    pub output_format: String,
    pub include_filters: Vec<MetricStreamFilter>,
    pub exclude_filters: Vec<MetricStreamFilter>,
    pub creation_date: i64,
    pub last_update_date: i64,
}

/// A CloudWatch metric stream filter.
#[derive(Debug, Clone)]
pub struct MetricStreamFilter {
    pub namespace: String,
    pub metric_names: Vec<String>,
}

/// A CloudWatch alarm mute rule.
#[derive(Debug, Clone)]
pub struct AlarmMuteRule {
    pub name: String,
    pub arn: String,
    pub description: Option<String>,
    pub mute_type: String,
    pub alarm_names: Vec<String>,
    pub status: String,
    pub last_updated_timestamp: i64,
}

/// A managed CloudWatch Insight Rule.
#[derive(Debug, Clone)]
pub struct ManagedInsightRule {
    pub template_name: String,
    pub resource_arn: String,
    pub rule_name: String,
}
