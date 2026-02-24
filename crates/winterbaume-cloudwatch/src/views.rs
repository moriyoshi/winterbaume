//! Serde-compatible view types for CloudWatch state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CloudWatchService;
use crate::state::CloudWatchState;
use crate::types::{
    AlarmMuteRule, AlarmState, AnomalyDetector, CompositeAlarm, Dashboard, Dimension, InsightRule,
    InsightRuleState, ManagedInsightRule, MetricAlarm, MetricDatum, MetricStream,
    MetricStreamFilter,
};

/// Serializable view of the entire CloudWatch state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CloudwatchStateView {
    /// Metric data points recorded by `PutMetricData` (newest at the end).
    #[serde(default)]
    pub metrics: Vec<MetricDatumView>,
    /// Metric alarms keyed by alarm name.
    #[serde(default)]
    pub alarms: HashMap<String, MetricAlarmView>,
    /// Composite alarms keyed by alarm name.
    #[serde(default)]
    pub composite_alarms: HashMap<String, CompositeAlarmView>,
    /// Dashboards keyed by dashboard name.
    #[serde(default)]
    pub dashboards: HashMap<String, DashboardView>,
    /// Insight rules keyed by rule name.
    #[serde(default)]
    pub insight_rules: HashMap<String, InsightRuleView>,
    /// Resource tags keyed by ARN.
    #[serde(default)]
    pub resource_tags: HashMap<String, HashMap<String, String>>,
    /// Anomaly detectors.
    #[serde(default)]
    pub anomaly_detectors: Vec<AnomalyDetectorView>,
    /// Metric streams keyed by name.
    #[serde(default)]
    pub metric_streams: HashMap<String, MetricStreamView>,
    /// Alarm mute rules keyed by name.
    #[serde(default)]
    pub alarm_mute_rules: HashMap<String, AlarmMuteRuleView>,
    /// Managed insight rules.
    #[serde(default)]
    pub managed_insight_rules: Vec<ManagedInsightRuleView>,
}

/// Serializable view of a CloudWatch metric data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDatumView {
    pub namespace: String,
    pub metric_name: String,
    #[serde(default)]
    pub dimensions: Vec<DimensionView>,
    pub value: Option<f64>,
    /// Timestamp in RFC 3339 format.
    pub timestamp: String,
    pub unit: Option<String>,
}

/// Serializable view of a metric alarm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricAlarmView {
    pub alarm_name: String,
    pub alarm_arn: String,
    pub metric_name: String,
    pub namespace: String,
    pub threshold: f64,
    pub comparison_operator: String,
    pub evaluation_periods: u32,
    pub period: u32,
    pub statistic: String,
    pub state_value: String,
    pub state_reason: String,
    pub actions_enabled: bool,
    pub alarm_description: Option<String>,
    #[serde(default)]
    pub alarm_actions: Vec<String>,
    #[serde(default)]
    pub ok_actions: Vec<String>,
    #[serde(default)]
    pub insufficient_data_actions: Vec<String>,
    #[serde(default)]
    pub dimensions: Vec<DimensionView>,
    pub unit: Option<String>,
}

/// Serializable view of a CloudWatch dimension.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionView {
    pub name: String,
    pub value: String,
}

/// Serializable view of a CloudWatch dashboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardView {
    pub dashboard_name: String,
    pub dashboard_body: String,
    pub dashboard_arn: String,
    pub last_modified: String,
}

/// Serializable view of a CloudWatch Contributor Insights rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsightRuleView {
    pub name: String,
    pub state: String,
    pub schema: String,
    pub definition: String,
}

/// Serializable view of a composite alarm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeAlarmView {
    pub alarm_name: String,
    pub alarm_arn: String,
    pub alarm_rule: String,
    pub alarm_description: Option<String>,
    pub actions_enabled: bool,
    #[serde(default)]
    pub alarm_actions: Vec<String>,
    #[serde(default)]
    pub ok_actions: Vec<String>,
    #[serde(default)]
    pub insufficient_data_actions: Vec<String>,
    pub state_value: String,
    pub state_reason: String,
}

/// Serializable view of an anomaly detector.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyDetectorView {
    pub namespace: String,
    pub metric_name: String,
    pub stat: String,
    #[serde(default)]
    pub dimensions: Vec<DimensionView>,
    pub state_value: String,
}

/// Serializable view of a metric stream filter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricStreamFilterView {
    pub namespace: String,
    #[serde(default)]
    pub metric_names: Vec<String>,
}

/// Serializable view of a metric stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricStreamView {
    pub name: String,
    pub arn: String,
    pub firehose_arn: String,
    pub role_arn: String,
    pub state: String,
    pub output_format: String,
    #[serde(default)]
    pub include_filters: Vec<MetricStreamFilterView>,
    #[serde(default)]
    pub exclude_filters: Vec<MetricStreamFilterView>,
    pub creation_date: i64,
    pub last_update_date: i64,
}

/// Serializable view of an alarm mute rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlarmMuteRuleView {
    pub name: String,
    pub arn: String,
    pub description: Option<String>,
    pub mute_type: String,
    #[serde(default)]
    pub alarm_names: Vec<String>,
    pub status: String,
    pub last_updated_timestamp: i64,
}

/// Serializable view of a managed insight rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedInsightRuleView {
    pub template_name: String,
    pub resource_arn: String,
    pub rule_name: String,
}

// --- From internal types to view types ---

impl From<&CloudWatchState> for CloudwatchStateView {
    fn from(state: &CloudWatchState) -> Self {
        CloudwatchStateView {
            metrics: state.metrics.iter().map(MetricDatumView::from).collect(),
            alarms: state
                .alarms
                .iter()
                .map(|(k, v)| (k.clone(), MetricAlarmView::from(v)))
                .collect(),
            composite_alarms: state
                .composite_alarms
                .iter()
                .map(|(k, v)| (k.clone(), CompositeAlarmView::from(v)))
                .collect(),
            dashboards: state
                .dashboards
                .iter()
                .map(|(k, v)| (k.clone(), DashboardView::from(v)))
                .collect(),
            insight_rules: state
                .insight_rules
                .iter()
                .map(|(k, v)| (k.clone(), InsightRuleView::from(v)))
                .collect(),
            resource_tags: state.resource_tags.clone(),
            anomaly_detectors: state
                .anomaly_detectors
                .iter()
                .map(AnomalyDetectorView::from)
                .collect(),
            metric_streams: state
                .metric_streams
                .iter()
                .map(|(k, v)| (k.clone(), MetricStreamView::from(v)))
                .collect(),
            alarm_mute_rules: state
                .alarm_mute_rules
                .iter()
                .map(|(k, v)| (k.clone(), AlarmMuteRuleView::from(v)))
                .collect(),
            managed_insight_rules: state
                .managed_insight_rules
                .iter()
                .map(ManagedInsightRuleView::from)
                .collect(),
        }
    }
}

impl From<&MetricDatum> for MetricDatumView {
    fn from(d: &MetricDatum) -> Self {
        MetricDatumView {
            namespace: d.namespace.clone(),
            metric_name: d.metric_name.clone(),
            dimensions: d.dimensions.iter().map(DimensionView::from).collect(),
            value: d.value,
            timestamp: d.timestamp.to_rfc3339(),
            unit: d.unit.clone(),
        }
    }
}

impl From<&MetricAlarm> for MetricAlarmView {
    fn from(alarm: &MetricAlarm) -> Self {
        MetricAlarmView {
            alarm_name: alarm.alarm_name.clone(),
            alarm_arn: alarm.alarm_arn.clone(),
            metric_name: alarm.metric_name.clone(),
            namespace: alarm.namespace.clone(),
            threshold: alarm.threshold,
            comparison_operator: alarm.comparison_operator.clone(),
            evaluation_periods: alarm.evaluation_periods,
            period: alarm.period,
            statistic: alarm.statistic.clone(),
            state_value: alarm.state_value.as_str().to_string(),
            state_reason: alarm.state_reason.clone(),
            actions_enabled: alarm.actions_enabled,
            alarm_description: alarm.alarm_description.clone(),
            alarm_actions: alarm.alarm_actions.clone(),
            ok_actions: alarm.ok_actions.clone(),
            insufficient_data_actions: alarm.insufficient_data_actions.clone(),
            dimensions: alarm.dimensions.iter().map(DimensionView::from).collect(),
            unit: alarm.unit.clone(),
        }
    }
}

impl From<&Dimension> for DimensionView {
    fn from(dim: &Dimension) -> Self {
        DimensionView {
            name: dim.name.clone(),
            value: dim.value.clone(),
        }
    }
}

impl From<&Dashboard> for DashboardView {
    fn from(dash: &Dashboard) -> Self {
        DashboardView {
            dashboard_name: dash.dashboard_name.clone(),
            dashboard_body: dash.dashboard_body.clone(),
            dashboard_arn: dash.dashboard_arn.clone(),
            last_modified: dash.last_modified.to_rfc3339(),
        }
    }
}

impl From<&InsightRule> for InsightRuleView {
    fn from(rule: &InsightRule) -> Self {
        InsightRuleView {
            name: rule.name.clone(),
            state: rule.state.as_str().to_string(),
            schema: rule.schema.clone(),
            definition: rule.definition.clone(),
        }
    }
}

impl From<&CompositeAlarm> for CompositeAlarmView {
    fn from(alarm: &CompositeAlarm) -> Self {
        CompositeAlarmView {
            alarm_name: alarm.alarm_name.clone(),
            alarm_arn: alarm.alarm_arn.clone(),
            alarm_rule: alarm.alarm_rule.clone(),
            alarm_description: alarm.alarm_description.clone(),
            actions_enabled: alarm.actions_enabled,
            alarm_actions: alarm.alarm_actions.clone(),
            ok_actions: alarm.ok_actions.clone(),
            insufficient_data_actions: alarm.insufficient_data_actions.clone(),
            state_value: alarm.state_value.as_str().to_string(),
            state_reason: alarm.state_reason.clone(),
        }
    }
}

impl From<&AnomalyDetector> for AnomalyDetectorView {
    fn from(d: &AnomalyDetector) -> Self {
        AnomalyDetectorView {
            namespace: d.namespace.clone(),
            metric_name: d.metric_name.clone(),
            stat: d.stat.clone(),
            dimensions: d.dimensions.iter().map(DimensionView::from).collect(),
            state_value: d.state_value.clone(),
        }
    }
}

impl From<&MetricStreamFilter> for MetricStreamFilterView {
    fn from(f: &MetricStreamFilter) -> Self {
        MetricStreamFilterView {
            namespace: f.namespace.clone(),
            metric_names: f.metric_names.clone(),
        }
    }
}

impl From<&MetricStream> for MetricStreamView {
    fn from(s: &MetricStream) -> Self {
        MetricStreamView {
            name: s.name.clone(),
            arn: s.arn.clone(),
            firehose_arn: s.firehose_arn.clone(),
            role_arn: s.role_arn.clone(),
            state: s.state.clone(),
            output_format: s.output_format.clone(),
            include_filters: s
                .include_filters
                .iter()
                .map(MetricStreamFilterView::from)
                .collect(),
            exclude_filters: s
                .exclude_filters
                .iter()
                .map(MetricStreamFilterView::from)
                .collect(),
            creation_date: s.creation_date,
            last_update_date: s.last_update_date,
        }
    }
}

impl From<&AlarmMuteRule> for AlarmMuteRuleView {
    fn from(r: &AlarmMuteRule) -> Self {
        AlarmMuteRuleView {
            name: r.name.clone(),
            arn: r.arn.clone(),
            description: r.description.clone(),
            mute_type: r.mute_type.clone(),
            alarm_names: r.alarm_names.clone(),
            status: r.status.clone(),
            last_updated_timestamp: r.last_updated_timestamp,
        }
    }
}

impl From<&ManagedInsightRule> for ManagedInsightRuleView {
    fn from(r: &ManagedInsightRule) -> Self {
        ManagedInsightRuleView {
            template_name: r.template_name.clone(),
            resource_arn: r.resource_arn.clone(),
            rule_name: r.rule_name.clone(),
        }
    }
}

impl From<ManagedInsightRuleView> for ManagedInsightRule {
    fn from(v: ManagedInsightRuleView) -> Self {
        ManagedInsightRule {
            template_name: v.template_name,
            resource_arn: v.resource_arn,
            rule_name: v.rule_name,
        }
    }
}

// --- From view types to internal types ---

impl From<CloudwatchStateView> for CloudWatchState {
    fn from(view: CloudwatchStateView) -> Self {
        CloudWatchState {
            metrics: view.metrics.into_iter().map(MetricDatum::from).collect(),
            alarms: view
                .alarms
                .into_iter()
                .map(|(k, v)| (k, MetricAlarm::from(v)))
                .collect(),
            composite_alarms: view
                .composite_alarms
                .into_iter()
                .map(|(k, v)| (k, CompositeAlarm::from(v)))
                .collect(),
            dashboards: view
                .dashboards
                .into_iter()
                .map(|(k, v)| (k, Dashboard::from(v)))
                .collect(),
            insight_rules: view
                .insight_rules
                .into_iter()
                .map(|(k, v)| (k, InsightRule::from(v)))
                .collect(),
            resource_tags: view.resource_tags,
            anomaly_detectors: view
                .anomaly_detectors
                .into_iter()
                .map(AnomalyDetector::from)
                .collect(),
            metric_streams: view
                .metric_streams
                .into_iter()
                .map(|(k, v)| (k, MetricStream::from(v)))
                .collect(),
            alarm_mute_rules: view
                .alarm_mute_rules
                .into_iter()
                .map(|(k, v)| (k, AlarmMuteRule::from(v)))
                .collect(),
            managed_insight_rules: view
                .managed_insight_rules
                .into_iter()
                .map(ManagedInsightRule::from)
                .collect(),
        }
    }
}

impl From<MetricDatumView> for MetricDatum {
    fn from(view: MetricDatumView) -> Self {
        let timestamp = view
            .timestamp
            .parse::<DateTime<Utc>>()
            .unwrap_or_else(|_| Utc::now());
        MetricDatum {
            namespace: view.namespace,
            metric_name: view.metric_name,
            dimensions: view.dimensions.into_iter().map(Dimension::from).collect(),
            value: view.value,
            timestamp,
            unit: view.unit,
        }
    }
}

impl From<MetricAlarmView> for MetricAlarm {
    fn from(view: MetricAlarmView) -> Self {
        MetricAlarm {
            alarm_name: view.alarm_name,
            alarm_arn: view.alarm_arn,
            metric_name: view.metric_name,
            namespace: view.namespace,
            threshold: view.threshold,
            comparison_operator: view.comparison_operator,
            evaluation_periods: view.evaluation_periods,
            period: view.period,
            statistic: view.statistic,
            state_value: match view.state_value.as_str() {
                "ALARM" => AlarmState::Alarm,
                "INSUFFICIENT_DATA" => AlarmState::InsufficientData,
                _ => AlarmState::Ok,
            },
            state_reason: view.state_reason,
            actions_enabled: view.actions_enabled,
            alarm_description: view.alarm_description,
            alarm_actions: view.alarm_actions,
            ok_actions: view.ok_actions,
            insufficient_data_actions: view.insufficient_data_actions,
            dimensions: view.dimensions.into_iter().map(Dimension::from).collect(),
            unit: view.unit,
        }
    }
}

impl From<DimensionView> for Dimension {
    fn from(view: DimensionView) -> Self {
        Dimension {
            name: view.name,
            value: view.value,
        }
    }
}

impl From<DashboardView> for Dashboard {
    fn from(view: DashboardView) -> Self {
        let last_modified = view
            .last_modified
            .parse::<DateTime<Utc>>()
            .unwrap_or_else(|_| Utc::now());
        Dashboard {
            dashboard_name: view.dashboard_name,
            dashboard_body: view.dashboard_body,
            dashboard_arn: view.dashboard_arn,
            last_modified,
        }
    }
}

impl From<InsightRuleView> for InsightRule {
    fn from(view: InsightRuleView) -> Self {
        InsightRule {
            name: view.name,
            state: match view.state.as_str() {
                "ENABLED" => InsightRuleState::Enabled,
                _ => InsightRuleState::Disabled,
            },
            schema: view.schema,
            definition: view.definition,
        }
    }
}

impl From<CompositeAlarmView> for CompositeAlarm {
    fn from(view: CompositeAlarmView) -> Self {
        CompositeAlarm {
            alarm_name: view.alarm_name,
            alarm_arn: view.alarm_arn,
            alarm_rule: view.alarm_rule,
            alarm_description: view.alarm_description,
            actions_enabled: view.actions_enabled,
            alarm_actions: view.alarm_actions,
            ok_actions: view.ok_actions,
            insufficient_data_actions: view.insufficient_data_actions,
            state_value: match view.state_value.as_str() {
                "ALARM" => AlarmState::Alarm,
                "INSUFFICIENT_DATA" => AlarmState::InsufficientData,
                _ => AlarmState::Ok,
            },
            state_reason: view.state_reason,
        }
    }
}

impl From<AnomalyDetectorView> for AnomalyDetector {
    fn from(view: AnomalyDetectorView) -> Self {
        AnomalyDetector {
            namespace: view.namespace,
            metric_name: view.metric_name,
            stat: view.stat,
            dimensions: view.dimensions.into_iter().map(Dimension::from).collect(),
            state_value: view.state_value,
        }
    }
}

impl From<MetricStreamFilterView> for MetricStreamFilter {
    fn from(view: MetricStreamFilterView) -> Self {
        MetricStreamFilter {
            namespace: view.namespace,
            metric_names: view.metric_names,
        }
    }
}

impl From<MetricStreamView> for MetricStream {
    fn from(view: MetricStreamView) -> Self {
        MetricStream {
            name: view.name,
            arn: view.arn,
            firehose_arn: view.firehose_arn,
            role_arn: view.role_arn,
            state: view.state,
            output_format: view.output_format,
            include_filters: view
                .include_filters
                .into_iter()
                .map(MetricStreamFilter::from)
                .collect(),
            exclude_filters: view
                .exclude_filters
                .into_iter()
                .map(MetricStreamFilter::from)
                .collect(),
            creation_date: view.creation_date,
            last_update_date: view.last_update_date,
        }
    }
}

impl From<AlarmMuteRuleView> for AlarmMuteRule {
    fn from(view: AlarmMuteRuleView) -> Self {
        AlarmMuteRule {
            name: view.name,
            arn: view.arn,
            description: view.description,
            mute_type: view.mute_type,
            alarm_names: view.alarm_names,
            status: view.status,
            last_updated_timestamp: view.last_updated_timestamp,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for CloudWatchService {
    type StateView = CloudwatchStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        CloudwatchStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = CloudWatchState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for datum_view in view.metrics {
                guard.metrics.push(MetricDatum::from(datum_view));
            }
            for (name, alarm_view) in view.alarms {
                guard.alarms.insert(name, MetricAlarm::from(alarm_view));
            }
            for (name, alarm_view) in view.composite_alarms {
                guard
                    .composite_alarms
                    .insert(name, CompositeAlarm::from(alarm_view));
            }
            for (name, dashboard_view) in view.dashboards {
                guard
                    .dashboards
                    .insert(name, Dashboard::from(dashboard_view));
            }
            for (name, rule_view) in view.insight_rules {
                guard
                    .insight_rules
                    .insert(name, InsightRule::from(rule_view));
            }
            for (arn, tags) in view.resource_tags {
                guard.resource_tags.entry(arn).or_default().extend(tags);
            }
            for detector_view in view.anomaly_detectors {
                guard
                    .anomaly_detectors
                    .push(AnomalyDetector::from(detector_view));
            }
            for (name, stream_view) in view.metric_streams {
                guard
                    .metric_streams
                    .insert(name, MetricStream::from(stream_view));
            }
            for (name, rule_view) in view.alarm_mute_rules {
                guard
                    .alarm_mute_rules
                    .insert(name, AlarmMuteRule::from(rule_view));
            }
            for rule_view in view.managed_insight_rules {
                guard
                    .managed_insight_rules
                    .push(ManagedInsightRule::from(rule_view));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
