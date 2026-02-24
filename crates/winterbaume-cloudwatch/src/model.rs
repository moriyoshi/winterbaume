//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cloudwatch

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

// These helpers are intentionally serialize-only. Generated rpc-v2-cbor
// request deserializers first convert CBOR Tag 1 values into plain JSON epoch
// numbers, then serde-deserialize those numbers into f64 fields.
#[derive(Serialize)]
struct CborTimestampSentinel {
    #[serde(rename = "__cbor_epoch_seconds")]
    epoch_seconds: f64,
}

fn serialize_cbor_timestamp<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    CborTimestampSentinel {
        epoch_seconds: *value,
    }
    .serialize(serializer)
}

fn serialize_cbor_timestamp_vec<S>(value: &[f64], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let sentinels: Vec<CborTimestampSentinel> = value
        .iter()
        .map(|epoch_seconds| CborTimestampSentinel {
            epoch_seconds: *epoch_seconds,
        })
        .collect();
    sentinels.serialize(serializer)
}

fn serialize_cbor_timestamp_vec_opt<S>(
    value: &Option<Vec<f64>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(values) => {
            let sentinels: Vec<CborTimestampSentinel> = values
                .iter()
                .map(|epoch_seconds| CborTimestampSentinel {
                    epoch_seconds: *epoch_seconds,
                })
                .collect();
            sentinels.serialize(serializer)
        }
        None => serializer.serialize_none(),
    }
}

fn serialize_cbor_timestamp_opt<S>(value: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(epoch_seconds) => CborTimestampSentinel {
            epoch_seconds: *epoch_seconds,
        }
        .serialize(serializer),
        None => serializer.serialize_none(),
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAlarmMuteRuleInput {
    #[serde(rename = "AlarmMuteRuleName")]
    #[serde(default)]
    pub alarm_mute_rule_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAlarmsInput {
    #[serde(rename = "AlarmNames")]
    #[serde(default)]
    pub alarm_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAnomalyDetectorInput {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<Dimension>>,
    #[serde(rename = "MetricMathAnomalyDetector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_math_anomaly_detector: Option<MetricMathAnomalyDetector>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "SingleMetricAnomalyDetector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_metric_anomaly_detector: Option<SingleMetricAnomalyDetector>,
    #[serde(rename = "Stat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stat: Option<String>,
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
pub struct MetricMathAnomalyDetector {
    #[serde(rename = "MetricDataQueries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_data_queries: Option<Vec<MetricDataQuery>>,
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
pub struct SingleMetricAnomalyDetector {
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
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "Stat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stat: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAnomalyDetectorOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDashboardsInput {
    #[serde(rename = "DashboardNames")]
    #[serde(default)]
    pub dashboard_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDashboardsOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInsightRulesInput {
    #[serde(rename = "RuleNames")]
    #[serde(default)]
    pub rule_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInsightRulesOutput {
    #[serde(rename = "Failures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<PartialFailure>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PartialFailure {
    #[serde(rename = "ExceptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception_type: Option<String>,
    #[serde(rename = "FailureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(rename = "FailureDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_description: Option<String>,
    #[serde(rename = "FailureResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_resource: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMetricStreamInput {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMetricStreamOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAlarmContributorsInput {
    #[serde(rename = "AlarmName")]
    #[serde(default)]
    pub alarm_name: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAlarmContributorsOutput {
    #[serde(rename = "AlarmContributors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_contributors: Option<Vec<AlarmContributor>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AlarmContributor {
    #[serde(rename = "ContributorAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor_attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ContributorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor_id: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(rename = "StateTransitionedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub state_transitioned_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAlarmHistoryInput {
    #[serde(rename = "AlarmContributorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_contributor_id: Option<String>,
    #[serde(rename = "AlarmName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_name: Option<String>,
    #[serde(rename = "AlarmTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_types: Option<Vec<String>>,
    #[serde(rename = "EndDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub end_date: Option<f64>,
    #[serde(rename = "HistoryItemType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_item_type: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ScanBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_by: Option<String>,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub start_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAlarmHistoryOutput {
    #[serde(rename = "AlarmHistoryItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_history_items: Option<Vec<AlarmHistoryItem>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AlarmHistoryItem {
    #[serde(rename = "AlarmContributorAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_contributor_attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "AlarmContributorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_contributor_id: Option<String>,
    #[serde(rename = "AlarmName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_name: Option<String>,
    #[serde(rename = "AlarmType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_type: Option<String>,
    #[serde(rename = "HistoryData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_data: Option<String>,
    #[serde(rename = "HistoryItemType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_item_type: Option<String>,
    #[serde(rename = "HistorySummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_summary: Option<String>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAlarmsForMetricInput {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<Dimension>>,
    #[serde(rename = "ExtendedStatistic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_statistic: Option<String>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    pub metric_name: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "Period")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(rename = "Statistic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAlarmsForMetricOutput {
    #[serde(rename = "MetricAlarms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_alarms: Option<Vec<MetricAlarm>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricAlarm {
    #[serde(rename = "ActionsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_enabled: Option<bool>,
    #[serde(rename = "AlarmActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_actions: Option<Vec<String>>,
    #[serde(rename = "AlarmArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_arn: Option<String>,
    #[serde(rename = "AlarmConfigurationUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub alarm_configuration_updated_timestamp: Option<f64>,
    #[serde(rename = "AlarmDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_description: Option<String>,
    #[serde(rename = "AlarmName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_name: Option<String>,
    #[serde(rename = "ComparisonOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    #[serde(rename = "DatapointsToAlarm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datapoints_to_alarm: Option<i32>,
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<Dimension>>,
    #[serde(rename = "EvaluateLowSampleCountPercentile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluate_low_sample_count_percentile: Option<String>,
    #[serde(rename = "EvaluationPeriods")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_periods: Option<i32>,
    #[serde(rename = "EvaluationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_state: Option<String>,
    #[serde(rename = "ExtendedStatistic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_statistic: Option<String>,
    #[serde(rename = "InsufficientDataActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insufficient_data_actions: Option<Vec<String>>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<MetricDataQuery>>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "OKActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_k_actions: Option<Vec<String>>,
    #[serde(rename = "Period")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(rename = "StateReasonData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason_data: Option<String>,
    #[serde(rename = "StateTransitionedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub state_transitioned_timestamp: Option<f64>,
    #[serde(rename = "StateUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub state_updated_timestamp: Option<f64>,
    #[serde(rename = "StateValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_value: Option<String>,
    #[serde(rename = "Statistic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
    #[serde(rename = "Threshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[serde(rename = "ThresholdMetricId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_metric_id: Option<String>,
    #[serde(rename = "TreatMissingData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treat_missing_data: Option<String>,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAlarmsInput {
    #[serde(rename = "ActionPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_prefix: Option<String>,
    #[serde(rename = "AlarmNamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_name_prefix: Option<String>,
    #[serde(rename = "AlarmNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_names: Option<Vec<String>>,
    #[serde(rename = "AlarmTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_types: Option<Vec<String>>,
    #[serde(rename = "ChildrenOfAlarmName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children_of_alarm_name: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ParentsOfAlarmName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents_of_alarm_name: Option<String>,
    #[serde(rename = "StateValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAlarmsOutput {
    #[serde(rename = "CompositeAlarms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_alarms: Option<Vec<CompositeAlarm>>,
    #[serde(rename = "MetricAlarms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_alarms: Option<Vec<MetricAlarm>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompositeAlarm {
    #[serde(rename = "ActionsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_enabled: Option<bool>,
    #[serde(rename = "ActionsSuppressedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_suppressed_by: Option<String>,
    #[serde(rename = "ActionsSuppressedReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_suppressed_reason: Option<String>,
    #[serde(rename = "ActionsSuppressor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_suppressor: Option<String>,
    #[serde(rename = "ActionsSuppressorExtensionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_suppressor_extension_period: Option<i32>,
    #[serde(rename = "ActionsSuppressorWaitPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_suppressor_wait_period: Option<i32>,
    #[serde(rename = "AlarmActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_actions: Option<Vec<String>>,
    #[serde(rename = "AlarmArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_arn: Option<String>,
    #[serde(rename = "AlarmConfigurationUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub alarm_configuration_updated_timestamp: Option<f64>,
    #[serde(rename = "AlarmDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_description: Option<String>,
    #[serde(rename = "AlarmName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_name: Option<String>,
    #[serde(rename = "AlarmRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_rule: Option<String>,
    #[serde(rename = "InsufficientDataActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insufficient_data_actions: Option<Vec<String>>,
    #[serde(rename = "OKActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_k_actions: Option<Vec<String>>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(rename = "StateReasonData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason_data: Option<String>,
    #[serde(rename = "StateTransitionedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub state_transitioned_timestamp: Option<f64>,
    #[serde(rename = "StateUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub state_updated_timestamp: Option<f64>,
    #[serde(rename = "StateValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAnomalyDetectorsInput {
    #[serde(rename = "AnomalyDetectorTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_detector_types: Option<Vec<String>>,
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<Dimension>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAnomalyDetectorsOutput {
    #[serde(rename = "AnomalyDetectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_detectors: Option<Vec<AnomalyDetector>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnomalyDetector {
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<AnomalyDetectorConfiguration>,
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<Dimension>>,
    #[serde(rename = "MetricCharacteristics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_characteristics: Option<MetricCharacteristics>,
    #[serde(rename = "MetricMathAnomalyDetector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_math_anomaly_detector: Option<MetricMathAnomalyDetector>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "SingleMetricAnomalyDetector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_metric_anomaly_detector: Option<SingleMetricAnomalyDetector>,
    #[serde(rename = "Stat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stat: Option<String>,
    #[serde(rename = "StateValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnomalyDetectorConfiguration {
    #[serde(rename = "ExcludedTimeRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_time_ranges: Option<Vec<Range>>,
    #[serde(rename = "MetricTimezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_timezone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Range {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(serialize_with = "serialize_cbor_timestamp")]
    pub end_time: f64,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(serialize_with = "serialize_cbor_timestamp")]
    pub start_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricCharacteristics {
    #[serde(rename = "PeriodicSpikes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub periodic_spikes: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInsightRulesInput {
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
pub struct DescribeInsightRulesOutput {
    #[serde(rename = "InsightRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_rules: Option<Vec<InsightRule>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InsightRule {
    #[serde(rename = "ApplyOnTransformedLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_on_transformed_logs: Option<bool>,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    #[serde(rename = "ManagedRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_rule: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Schema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableAlarmActionsInput {
    #[serde(rename = "AlarmNames")]
    #[serde(default)]
    pub alarm_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableInsightRulesInput {
    #[serde(rename = "RuleNames")]
    #[serde(default)]
    pub rule_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableInsightRulesOutput {
    #[serde(rename = "Failures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<PartialFailure>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableAlarmActionsInput {
    #[serde(rename = "AlarmNames")]
    #[serde(default)]
    pub alarm_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableInsightRulesInput {
    #[serde(rename = "RuleNames")]
    #[serde(default)]
    pub rule_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableInsightRulesOutput {
    #[serde(rename = "Failures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<PartialFailure>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAlarmMuteRuleInput {
    #[serde(rename = "AlarmMuteRuleName")]
    #[serde(default)]
    pub alarm_mute_rule_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAlarmMuteRuleOutput {
    #[serde(rename = "AlarmMuteRuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_mute_rule_arn: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExpireDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub expire_date: Option<f64>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub last_updated_timestamp: Option<f64>,
    #[serde(rename = "MuteTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute_targets: Option<MuteTargets>,
    #[serde(rename = "MuteType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute_type: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Rule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Rule>,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub start_date: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MuteTargets {
    #[serde(rename = "AlarmNames")]
    #[serde(default)]
    pub alarm_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Rule {
    #[serde(rename = "Schedule")]
    #[serde(default)]
    pub schedule: Schedule,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Schedule {
    #[serde(rename = "Duration")]
    #[serde(default)]
    pub duration: String,
    #[serde(rename = "Expression")]
    #[serde(default)]
    pub expression: String,
    #[serde(rename = "Timezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDashboardInput {
    #[serde(rename = "DashboardName")]
    #[serde(default)]
    pub dashboard_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDashboardOutput {
    #[serde(rename = "DashboardArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_arn: Option<String>,
    #[serde(rename = "DashboardBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_body: Option<String>,
    #[serde(rename = "DashboardName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInsightRuleReportInput {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(serialize_with = "serialize_cbor_timestamp")]
    pub end_time: f64,
    #[serde(rename = "MaxContributorCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_contributor_count: Option<i32>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<String>>,
    #[serde(rename = "OrderBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(rename = "Period")]
    #[serde(default)]
    pub period: i32,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    pub rule_name: String,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(serialize_with = "serialize_cbor_timestamp")]
    pub start_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInsightRuleReportOutput {
    #[serde(rename = "AggregateValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_value: Option<f64>,
    #[serde(rename = "AggregationStatistic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_statistic: Option<String>,
    #[serde(rename = "ApproximateUniqueCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_unique_count: Option<i64>,
    #[serde(rename = "Contributors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributors: Option<Vec<InsightRuleContributor>>,
    #[serde(rename = "KeyLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_labels: Option<Vec<String>>,
    #[serde(rename = "MetricDatapoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_datapoints: Option<Vec<InsightRuleMetricDatapoint>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InsightRuleContributor {
    #[serde(rename = "ApproximateAggregateValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_aggregate_value: Option<f64>,
    #[serde(rename = "Datapoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datapoints: Option<Vec<InsightRuleContributorDatapoint>>,
    #[serde(rename = "Keys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InsightRuleContributorDatapoint {
    #[serde(rename = "ApproximateValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_value: Option<f64>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InsightRuleMetricDatapoint {
    #[serde(rename = "Average")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average: Option<f64>,
    #[serde(rename = "MaxContributorValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_contributor_value: Option<f64>,
    #[serde(rename = "Maximum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(rename = "Minimum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(rename = "SampleCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_count: Option<f64>,
    #[serde(rename = "Sum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum: Option<f64>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub timestamp: Option<f64>,
    #[serde(rename = "UniqueContributors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_contributors: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMetricDataInput {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(serialize_with = "serialize_cbor_timestamp")]
    pub end_time: f64,
    #[serde(rename = "LabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_options: Option<LabelOptions>,
    #[serde(rename = "MaxDatapoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_datapoints: Option<i32>,
    #[serde(rename = "MetricDataQueries")]
    #[serde(default)]
    pub metric_data_queries: Vec<MetricDataQuery>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ScanBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_by: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(serialize_with = "serialize_cbor_timestamp")]
    pub start_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LabelOptions {
    #[serde(rename = "Timezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMetricDataOutput {
    #[serde(rename = "Messages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<MessageData>>,
    #[serde(rename = "MetricDataResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_data_results: Option<Vec<MetricDataResult>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MessageData {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricDataResult {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Label")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "Messages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<MessageData>>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "Timestamps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_vec_opt")]
    pub timestamps: Option<Vec<f64>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMetricStatisticsInput {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<Dimension>>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(serialize_with = "serialize_cbor_timestamp")]
    pub end_time: f64,
    #[serde(rename = "ExtendedStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_statistics: Option<Vec<String>>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    pub metric_name: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "Period")]
    #[serde(default)]
    pub period: i32,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(serialize_with = "serialize_cbor_timestamp")]
    pub start_time: f64,
    #[serde(rename = "Statistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Vec<String>>,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMetricStatisticsOutput {
    #[serde(rename = "Datapoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datapoints: Option<Vec<Datapoint>>,
    #[serde(rename = "Label")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Datapoint {
    #[serde(rename = "Average")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average: Option<f64>,
    #[serde(rename = "ExtendedStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_statistics: Option<std::collections::HashMap<String, f64>>,
    #[serde(rename = "Maximum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(rename = "Minimum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(rename = "SampleCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_count: Option<f64>,
    #[serde(rename = "Sum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum: Option<f64>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub timestamp: Option<f64>,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMetricStreamInput {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMetricStreamOutput {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub creation_date: Option<f64>,
    #[serde(rename = "ExcludeFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_filters: Option<Vec<MetricStreamFilter>>,
    #[serde(rename = "FirehoseArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firehose_arn: Option<String>,
    #[serde(rename = "IncludeFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_filters: Option<Vec<MetricStreamFilter>>,
    #[serde(rename = "IncludeLinkedAccountsMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_linked_accounts_metrics: Option<bool>,
    #[serde(rename = "LastUpdateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub last_update_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OutputFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StatisticsConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics_configurations: Option<Vec<MetricStreamStatisticsConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricStreamFilter {
    #[serde(rename = "MetricNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_names: Option<Vec<String>>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricStreamStatisticsConfiguration {
    #[serde(rename = "AdditionalStatistics")]
    #[serde(default)]
    pub additional_statistics: Vec<String>,
    #[serde(rename = "IncludeMetrics")]
    #[serde(default)]
    pub include_metrics: Vec<MetricStreamStatisticsMetric>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricStreamStatisticsMetric {
    #[serde(rename = "MetricName")]
    #[serde(default)]
    pub metric_name: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMetricWidgetImageInput {
    #[serde(rename = "MetricWidget")]
    #[serde(default)]
    pub metric_widget: String,
    #[serde(rename = "OutputFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMetricWidgetImageOutput {
    #[serde(rename = "MetricWidgetImage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_widget_image: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAlarmMuteRulesInput {
    #[serde(rename = "AlarmName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_name: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Statuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAlarmMuteRulesOutput {
    #[serde(rename = "AlarmMuteRuleSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_mute_rule_summaries: Option<Vec<AlarmMuteRuleSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AlarmMuteRuleSummary {
    #[serde(rename = "AlarmMuteRuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_mute_rule_arn: Option<String>,
    #[serde(rename = "ExpireDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub expire_date: Option<f64>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub last_updated_timestamp: Option<f64>,
    #[serde(rename = "MuteType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDashboardsInput {
    #[serde(rename = "DashboardNamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_name_prefix: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDashboardsOutput {
    #[serde(rename = "DashboardEntries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_entries: Option<Vec<DashboardEntry>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashboardEntry {
    #[serde(rename = "DashboardArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_arn: Option<String>,
    #[serde(rename = "DashboardName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_name: Option<String>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub last_modified: Option<f64>,
    #[serde(rename = "Size")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListManagedInsightRulesInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListManagedInsightRulesOutput {
    #[serde(rename = "ManagedRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_rules: Option<Vec<ManagedRuleDescription>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedRuleDescription {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
    #[serde(rename = "RuleState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_state: Option<ManagedRuleState>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedRuleState {
    #[serde(rename = "RuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMetricStreamsInput {
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
pub struct ListMetricStreamsOutput {
    #[serde(rename = "Entries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<MetricStreamEntry>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricStreamEntry {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub creation_date: Option<f64>,
    #[serde(rename = "FirehoseArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firehose_arn: Option<String>,
    #[serde(rename = "LastUpdateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub last_update_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OutputFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMetricsInput {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<DimensionFilter>>,
    #[serde(rename = "IncludeLinkedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_linked_accounts: Option<bool>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OwningAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owning_account: Option<String>,
    #[serde(rename = "RecentlyActive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recently_active: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DimensionFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMetricsOutput {
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<Metric>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OwningAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owning_accounts: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceInput {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceOutput {
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
pub struct PutAlarmMuteRuleInput {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExpireDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub expire_date: Option<f64>,
    #[serde(rename = "MuteTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute_targets: Option<MuteTargets>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Rule")]
    #[serde(default)]
    pub rule: Rule,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub start_date: Option<f64>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAnomalyDetectorInput {
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<AnomalyDetectorConfiguration>,
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<Dimension>>,
    #[serde(rename = "MetricCharacteristics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_characteristics: Option<MetricCharacteristics>,
    #[serde(rename = "MetricMathAnomalyDetector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_math_anomaly_detector: Option<MetricMathAnomalyDetector>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "SingleMetricAnomalyDetector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_metric_anomaly_detector: Option<SingleMetricAnomalyDetector>,
    #[serde(rename = "Stat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stat: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAnomalyDetectorOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutCompositeAlarmInput {
    #[serde(rename = "ActionsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_enabled: Option<bool>,
    #[serde(rename = "ActionsSuppressor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_suppressor: Option<String>,
    #[serde(rename = "ActionsSuppressorExtensionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_suppressor_extension_period: Option<i32>,
    #[serde(rename = "ActionsSuppressorWaitPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_suppressor_wait_period: Option<i32>,
    #[serde(rename = "AlarmActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_actions: Option<Vec<String>>,
    #[serde(rename = "AlarmDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_description: Option<String>,
    #[serde(rename = "AlarmName")]
    #[serde(default)]
    pub alarm_name: String,
    #[serde(rename = "AlarmRule")]
    #[serde(default)]
    pub alarm_rule: String,
    #[serde(rename = "InsufficientDataActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insufficient_data_actions: Option<Vec<String>>,
    #[serde(rename = "OKActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_k_actions: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDashboardInput {
    #[serde(rename = "DashboardBody")]
    #[serde(default)]
    pub dashboard_body: String,
    #[serde(rename = "DashboardName")]
    #[serde(default)]
    pub dashboard_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDashboardOutput {
    #[serde(rename = "DashboardValidationMessages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_validation_messages: Option<Vec<DashboardValidationMessage>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashboardValidationMessage {
    #[serde(rename = "DataPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_path: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutInsightRuleInput {
    #[serde(rename = "ApplyOnTransformedLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_on_transformed_logs: Option<bool>,
    #[serde(rename = "RuleDefinition")]
    #[serde(default)]
    pub rule_definition: String,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    pub rule_name: String,
    #[serde(rename = "RuleState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_state: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutInsightRuleOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutManagedInsightRulesInput {
    #[serde(rename = "ManagedRules")]
    #[serde(default)]
    pub managed_rules: Vec<ManagedRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedRule {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutManagedInsightRulesOutput {
    #[serde(rename = "Failures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<PartialFailure>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutMetricAlarmInput {
    #[serde(rename = "ActionsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_enabled: Option<bool>,
    #[serde(rename = "AlarmActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_actions: Option<Vec<String>>,
    #[serde(rename = "AlarmDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_description: Option<String>,
    #[serde(rename = "AlarmName")]
    #[serde(default)]
    pub alarm_name: String,
    #[serde(rename = "ComparisonOperator")]
    #[serde(default)]
    pub comparison_operator: String,
    #[serde(rename = "DatapointsToAlarm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datapoints_to_alarm: Option<i32>,
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<Dimension>>,
    #[serde(rename = "EvaluateLowSampleCountPercentile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluate_low_sample_count_percentile: Option<String>,
    #[serde(rename = "EvaluationPeriods")]
    #[serde(default)]
    pub evaluation_periods: i32,
    #[serde(rename = "ExtendedStatistic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_statistic: Option<String>,
    #[serde(rename = "InsufficientDataActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insufficient_data_actions: Option<Vec<String>>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<MetricDataQuery>>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "OKActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_k_actions: Option<Vec<String>>,
    #[serde(rename = "Period")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(rename = "Statistic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Threshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[serde(rename = "ThresholdMetricId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_metric_id: Option<String>,
    #[serde(rename = "TreatMissingData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treat_missing_data: Option<String>,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutMetricDataInput {
    #[serde(rename = "EntityMetricData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_metric_data: Option<Vec<EntityMetricData>>,
    #[serde(rename = "MetricData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_data: Option<Vec<MetricDatum>>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "StrictEntityValidation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict_entity_validation: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EntityMetricData {
    #[serde(rename = "Entity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Entity>,
    #[serde(rename = "MetricData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_data: Option<Vec<MetricDatum>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Entity {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "KeyAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_attributes: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricDatum {
    #[serde(rename = "Counts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts: Option<Vec<f64>>,
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<Dimension>>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    pub metric_name: String,
    #[serde(rename = "StatisticValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic_values: Option<StatisticSet>,
    #[serde(rename = "StorageResolution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_resolution: Option<i32>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_cbor_timestamp_opt")]
    pub timestamp: Option<f64>,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatisticSet {
    #[serde(rename = "Maximum")]
    #[serde(default)]
    pub maximum: f64,
    #[serde(rename = "Minimum")]
    #[serde(default)]
    pub minimum: f64,
    #[serde(rename = "SampleCount")]
    #[serde(default)]
    pub sample_count: f64,
    #[serde(rename = "Sum")]
    #[serde(default)]
    pub sum: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutMetricStreamInput {
    #[serde(rename = "ExcludeFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_filters: Option<Vec<MetricStreamFilter>>,
    #[serde(rename = "FirehoseArn")]
    #[serde(default)]
    pub firehose_arn: String,
    #[serde(rename = "IncludeFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_filters: Option<Vec<MetricStreamFilter>>,
    #[serde(rename = "IncludeLinkedAccountsMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_linked_accounts_metrics: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputFormat")]
    #[serde(default)]
    pub output_format: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "StatisticsConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics_configurations: Option<Vec<MetricStreamStatisticsConfiguration>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutMetricStreamOutput {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetAlarmStateInput {
    #[serde(rename = "AlarmName")]
    #[serde(default)]
    pub alarm_name: String,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    pub state_reason: String,
    #[serde(rename = "StateReasonData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason_data: Option<String>,
    #[serde(rename = "StateValue")]
    #[serde(default)]
    pub state_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMetricStreamsInput {
    #[serde(rename = "Names")]
    #[serde(default)]
    pub names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMetricStreamsOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopMetricStreamsInput {
    #[serde(rename = "Names")]
    #[serde(default)]
    pub names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopMetricStreamsOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceInput {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceInput {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceOutput {}
