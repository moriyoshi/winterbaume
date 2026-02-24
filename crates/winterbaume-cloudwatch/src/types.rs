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
    pub metric_name: String,
    pub namespace: String,
    pub threshold: f64,
    pub comparison_operator: String,
    pub evaluation_periods: u32,
    pub period: u32,
    pub statistic: String,
    pub state_value: AlarmState,
    pub state_reason: String,
    pub actions_enabled: bool,
    pub alarm_description: Option<String>,
    pub alarm_actions: Vec<String>,
    pub ok_actions: Vec<String>,
    pub insufficient_data_actions: Vec<String>,
    pub dimensions: Vec<Dimension>,
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
    pub metric_name: String,
    pub namespace: String,
    pub threshold: f64,
    pub comparison_operator: String,
    pub evaluation_periods: u32,
    pub period: u32,
    pub statistic: String,
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
#[derive(Debug, Clone)]
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
