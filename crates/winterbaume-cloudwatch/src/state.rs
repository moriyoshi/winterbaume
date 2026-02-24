use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

/// In-memory state for CloudWatch.
#[derive(Debug, Default)]
pub struct CloudWatchState {
    /// Metric data points.
    pub metrics: Vec<MetricDatum>,
    /// Alarms keyed by alarm name.
    pub alarms: HashMap<String, MetricAlarm>,
    /// Composite alarms keyed by alarm name.
    pub composite_alarms: HashMap<String, CompositeAlarm>,
    /// Dashboards keyed by dashboard name.
    pub dashboards: HashMap<String, Dashboard>,
    /// Insight rules keyed by rule name.
    pub insight_rules: HashMap<String, InsightRule>,
    /// Resource tags keyed by ARN.
    pub resource_tags: HashMap<String, HashMap<String, String>>,
    /// Anomaly detectors stored as a list (keyed by namespace+metric+stat).
    pub anomaly_detectors: Vec<AnomalyDetector>,
    /// Metric streams keyed by name.
    pub metric_streams: HashMap<String, MetricStream>,
    /// Alarm mute rules keyed by name.
    pub alarm_mute_rules: HashMap<String, AlarmMuteRule>,
    /// Managed insight rules.
    pub managed_insight_rules: Vec<ManagedInsightRule>,
}

/// Error type for CloudWatch operations.
#[derive(Debug, thiserror::Error)]
pub enum CloudWatchError {
    #[error("The parameter Namespace must be specified.")]
    NamespaceRequired,

    #[error("Invalid state value: {value}")]
    InvalidStateValue { value: String },

    #[error("Dashboard {name} does not exist.")]
    DashboardNotFound { name: String },

    #[error("An alarm named '{name}' does not exist.")]
    AlarmNotFound { name: String },

    #[error("Rule {name} does not exist.")]
    InsightRuleNotFound { name: String },

    #[error("Resource not found")]
    ResourceNotFound,

    #[error("Anomaly detector not found for {namespace}/{metric_name}/{stat}")]
    AnomalyDetectorNotFound {
        namespace: String,
        metric_name: String,
        stat: String,
    },

    #[error("Metric stream '{name}' does not exist.")]
    MetricStreamNotFound { name: String },

    #[error("Alarm mute rule '{name}' does not exist.")]
    AlarmMuteRuleNotFound { name: String },
}

impl CloudWatchState {
    pub fn put_metric_data(
        &mut self,
        namespace: &str,
        metric_data: Vec<MetricDatum>,
    ) -> Result<(), CloudWatchError> {
        if namespace.is_empty() {
            return Err(CloudWatchError::NamespaceRequired);
        }
        for mut datum in metric_data {
            datum.namespace = namespace.to_string();
            self.metrics.push(datum);
        }
        Ok(())
    }

    pub fn get_metric_data(
        &self,
        namespace: Option<&str>,
        metric_name: Option<&str>,
    ) -> Vec<&MetricDatum> {
        self.metrics
            .iter()
            .filter(|m| namespace.is_none() || Some(m.namespace.as_str()) == namespace)
            .filter(|m| metric_name.is_none() || Some(m.metric_name.as_str()) == metric_name)
            .collect()
    }

    pub fn list_metrics(&self, namespace: Option<&str>, metric_name: Option<&str>) -> Vec<Metric> {
        let mut seen = HashMap::new();
        for m in &self.metrics {
            if let Some(ns) = namespace
                && m.namespace != ns
            {
                continue;
            }
            if let Some(mn) = metric_name
                && m.metric_name != mn
            {
                continue;
            }
            let key = format!("{}:{}", m.namespace, m.metric_name);
            seen.entry(key).or_insert_with(|| Metric {
                namespace: m.namespace.clone(),
                metric_name: m.metric_name.clone(),
                dimensions: m.dimensions.clone(),
            });
        }
        seen.into_values().collect()
    }

    pub fn put_metric_alarm(
        &mut self,
        input: PutMetricAlarmInput,
        account_id: &str,
        region: &str,
    ) -> Result<(), CloudWatchError> {
        let arn = format!(
            "arn:aws:cloudwatch:{region}:{account_id}:alarm:{}",
            input.alarm_name
        );

        let alarm = MetricAlarm {
            alarm_name: input.alarm_name.clone(),
            alarm_arn: arn.clone(),
            metric_name: input.metric_name,
            namespace: input.namespace,
            threshold: input.threshold,
            comparison_operator: input.comparison_operator,
            evaluation_periods: input.evaluation_periods,
            period: input.period,
            statistic: input.statistic,
            state_value: AlarmState::Ok,
            state_reason: "Unchecked: Initial alarm creation".to_string(),
            actions_enabled: input.actions_enabled,
            alarm_description: input.alarm_description,
            alarm_actions: input.alarm_actions,
            ok_actions: input.ok_actions,
            insufficient_data_actions: input.insufficient_data_actions,
            dimensions: input.dimensions,
            unit: input.unit,
        };

        self.alarms.insert(input.alarm_name.clone(), alarm);

        // Store tags if provided
        if !input.tags.is_empty() {
            self.resource_tags
                .entry(arn)
                .or_default()
                .extend(input.tags);
        }

        Ok(())
    }

    pub fn describe_alarms(
        &self,
        alarm_names: Option<&[String]>,
        state_value: Option<&str>,
    ) -> Vec<&MetricAlarm> {
        self.alarms
            .values()
            .filter(|a| match alarm_names {
                Some(names) => names.iter().any(|n| n == &a.alarm_name),
                None => true,
            })
            .filter(|a| match state_value {
                Some(sv) => a.state_value.as_str() == sv,
                None => true,
            })
            .collect()
    }

    pub fn describe_alarms_for_metric(
        &self,
        metric_name: &str,
        namespace: &str,
    ) -> Vec<&MetricAlarm> {
        self.alarms
            .values()
            .filter(|a| a.metric_name == metric_name && a.namespace == namespace)
            .collect()
    }

    pub fn delete_alarms(&mut self, alarm_names: &[String]) -> Result<(), CloudWatchError> {
        for name in alarm_names {
            self.alarms.remove(name);
        }
        Ok(())
    }

    // --- Dashboard operations ---

    pub fn put_dashboard(
        &mut self,
        dashboard_name: &str,
        dashboard_body: &str,
        account_id: &str,
        region: &str,
    ) -> Result<(), CloudWatchError> {
        let arn = format!("arn:aws:cloudwatch:{region}:{account_id}:dashboard/{dashboard_name}");
        let dashboard = Dashboard {
            dashboard_name: dashboard_name.to_string(),
            dashboard_body: dashboard_body.to_string(),
            dashboard_arn: arn,
            last_modified: Utc::now(),
        };
        self.dashboards
            .insert(dashboard_name.to_string(), dashboard);
        Ok(())
    }

    pub fn get_dashboard(&self, dashboard_name: &str) -> Result<&Dashboard, CloudWatchError> {
        self.dashboards
            .get(dashboard_name)
            .ok_or(CloudWatchError::DashboardNotFound {
                name: dashboard_name.to_string(),
            })
    }

    pub fn list_dashboards(&self, prefix: Option<&str>) -> Vec<&Dashboard> {
        self.dashboards
            .values()
            .filter(|d| match prefix {
                Some(p) => d.dashboard_name.starts_with(p),
                None => true,
            })
            .collect()
    }

    pub fn delete_dashboards(&mut self, dashboard_names: &[String]) -> Result<(), CloudWatchError> {
        for name in dashboard_names {
            if !self.dashboards.contains_key(name) {
                return Err(CloudWatchError::DashboardNotFound {
                    name: name.to_string(),
                });
            }
        }
        for name in dashboard_names {
            self.dashboards.remove(name);
        }
        Ok(())
    }

    // --- Insight Rule operations ---

    pub fn put_insight_rule(
        &mut self,
        name: &str,
        schema: &str,
        definition: &str,
    ) -> Result<(), CloudWatchError> {
        let rule = InsightRule {
            name: name.to_string(),
            state: InsightRuleState::Disabled,
            schema: schema.to_string(),
            definition: definition.to_string(),
        };
        self.insight_rules.insert(name.to_string(), rule);
        Ok(())
    }

    pub fn describe_insight_rules(&self) -> Vec<&InsightRule> {
        self.insight_rules.values().collect()
    }

    pub fn enable_insight_rules(&mut self, rule_names: &[String]) -> Result<(), CloudWatchError> {
        for name in rule_names {
            match self.insight_rules.get_mut(name) {
                Some(rule) => rule.state = InsightRuleState::Enabled,
                None => {
                    return Err(CloudWatchError::InsightRuleNotFound {
                        name: name.to_string(),
                    });
                }
            }
        }
        Ok(())
    }

    pub fn disable_insight_rules(&mut self, rule_names: &[String]) -> Result<(), CloudWatchError> {
        for name in rule_names {
            match self.insight_rules.get_mut(name) {
                Some(rule) => rule.state = InsightRuleState::Disabled,
                None => {
                    return Err(CloudWatchError::InsightRuleNotFound {
                        name: name.to_string(),
                    });
                }
            }
        }
        Ok(())
    }

    pub fn delete_insight_rules(&mut self, rule_names: &[String]) -> Result<(), CloudWatchError> {
        for name in rule_names {
            if !self.insight_rules.contains_key(name) {
                return Err(CloudWatchError::InsightRuleNotFound {
                    name: name.to_string(),
                });
            }
        }
        for name in rule_names {
            self.insight_rules.remove(name);
        }
        Ok(())
    }

    // --- Alarm state ---

    pub fn set_alarm_state(
        &mut self,
        alarm_name: &str,
        state_value: &str,
        state_reason: &str,
    ) -> Result<(), CloudWatchError> {
        let alarm = self
            .alarms
            .get_mut(alarm_name)
            .ok_or(CloudWatchError::AlarmNotFound {
                name: alarm_name.to_string(),
            })?;
        alarm.state_value = match state_value {
            "OK" => AlarmState::Ok,
            "ALARM" => AlarmState::Alarm,
            "INSUFFICIENT_DATA" => AlarmState::InsufficientData,
            _ => {
                return Err(CloudWatchError::InvalidStateValue {
                    value: state_value.to_string(),
                });
            }
        };
        alarm.state_reason = state_reason.to_string();
        Ok(())
    }

    // --- Tag operations ---

    fn resource_exists(&self, arn: &str) -> bool {
        // Check if the ARN matches any alarm or insight rule
        self.alarms.values().any(|a| a.alarm_arn == arn)
            || self.insight_rules.values().any(|_r| {
                // Insight rule ARNs are constructed like:
                // arn:aws:cloudwatch:<region>:<account>:insight-rule/<name>
                arn.contains("insight-rule/")
            })
            || self.resource_tags.contains_key(arn)
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), CloudWatchError> {
        if !self.resource_exists(resource_arn) {
            return Err(CloudWatchError::ResourceNotFound);
        }
        let entry = self
            .resource_tags
            .entry(resource_arn.to_string())
            .or_default();
        entry.extend(tags);
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), CloudWatchError> {
        if !self.resource_exists(resource_arn) {
            return Err(CloudWatchError::ResourceNotFound);
        }
        if let Some(tags) = self.resource_tags.get_mut(resource_arn) {
            for key in tag_keys {
                tags.remove(key);
            }
        }
        Ok(())
    }

    pub fn list_tags_for_resource(&self, resource_arn: &str) -> HashMap<String, String> {
        self.resource_tags
            .get(resource_arn)
            .cloned()
            .unwrap_or_default()
    }

    // --- Alarm actions enable/disable ---

    pub fn disable_alarm_actions(&mut self, alarm_names: &[String]) {
        for name in alarm_names {
            if let Some(alarm) = self.alarms.get_mut(name) {
                alarm.actions_enabled = false;
            }
            if let Some(alarm) = self.composite_alarms.get_mut(name) {
                alarm.actions_enabled = false;
            }
        }
    }

    pub fn enable_alarm_actions(&mut self, alarm_names: &[String]) {
        for name in alarm_names {
            if let Some(alarm) = self.alarms.get_mut(name) {
                alarm.actions_enabled = true;
            }
            if let Some(alarm) = self.composite_alarms.get_mut(name) {
                alarm.actions_enabled = true;
            }
        }
    }

    // --- Composite alarm ---

    pub fn put_composite_alarm(
        &mut self,
        alarm_name: &str,
        alarm_rule: &str,
        account_id: &str,
        region: &str,
        alarm_description: Option<String>,
        actions_enabled: bool,
        alarm_actions: Vec<String>,
        ok_actions: Vec<String>,
        insufficient_data_actions: Vec<String>,
    ) {
        let arn = format!("arn:aws:cloudwatch:{region}:{account_id}:alarm:{alarm_name}");
        let alarm = CompositeAlarm {
            alarm_name: alarm_name.to_string(),
            alarm_arn: arn,
            alarm_rule: alarm_rule.to_string(),
            alarm_description,
            actions_enabled,
            alarm_actions,
            ok_actions,
            insufficient_data_actions,
            state_value: AlarmState::Ok,
            state_reason: "Unchecked: Initial alarm creation".to_string(),
        };
        self.composite_alarms.insert(alarm_name.to_string(), alarm);
    }

    // --- Anomaly detector ---

    pub fn put_anomaly_detector(&mut self, detector: AnomalyDetector) {
        // Remove any existing detector with same namespace+metric_name+stat
        self.anomaly_detectors.retain(|d| {
            !(d.namespace == detector.namespace
                && d.metric_name == detector.metric_name
                && d.stat == detector.stat)
        });
        self.anomaly_detectors.push(detector);
    }

    pub fn delete_anomaly_detector(
        &mut self,
        namespace: &str,
        metric_name: &str,
        stat: &str,
    ) -> Result<(), CloudWatchError> {
        let before = self.anomaly_detectors.len();
        self.anomaly_detectors.retain(|d| {
            !(d.namespace == namespace && d.metric_name == metric_name && d.stat == stat)
        });
        if self.anomaly_detectors.len() == before {
            return Err(CloudWatchError::AnomalyDetectorNotFound {
                namespace: namespace.to_string(),
                metric_name: metric_name.to_string(),
                stat: stat.to_string(),
            });
        }
        Ok(())
    }

    pub fn describe_anomaly_detectors(&self) -> &[AnomalyDetector] {
        &self.anomaly_detectors
    }

    // --- Metric stream ---

    pub fn put_metric_stream(
        &mut self,
        name: &str,
        firehose_arn: &str,
        role_arn: &str,
        output_format: &str,
        include_filters: Vec<MetricStreamFilter>,
        exclude_filters: Vec<MetricStreamFilter>,
        account_id: &str,
        region: &str,
    ) -> String {
        let arn = format!("arn:aws:cloudwatch:{region}:{account_id}:metric-stream/{name}");
        let now = Utc::now().timestamp();
        let existing = self.metric_streams.get(name);
        let creation_date = existing.map(|s| s.creation_date).unwrap_or(now);
        let stream = MetricStream {
            name: name.to_string(),
            arn: arn.clone(),
            firehose_arn: firehose_arn.to_string(),
            role_arn: role_arn.to_string(),
            state: "running".to_string(),
            output_format: output_format.to_string(),
            include_filters,
            exclude_filters,
            creation_date,
            last_update_date: now,
        };
        self.metric_streams.insert(name.to_string(), stream);
        arn
    }

    pub fn delete_metric_stream(&mut self, name: &str) -> Result<(), CloudWatchError> {
        if self.metric_streams.remove(name).is_none() {
            return Err(CloudWatchError::MetricStreamNotFound {
                name: name.to_string(),
            });
        }
        Ok(())
    }

    pub fn get_metric_stream(&self, name: &str) -> Result<&MetricStream, CloudWatchError> {
        self.metric_streams
            .get(name)
            .ok_or(CloudWatchError::MetricStreamNotFound {
                name: name.to_string(),
            })
    }

    pub fn list_metric_streams(&self) -> Vec<&MetricStream> {
        self.metric_streams.values().collect()
    }

    pub fn start_metric_streams(&mut self, names: &[String]) -> Result<(), CloudWatchError> {
        for name in names {
            match self.metric_streams.get_mut(name) {
                Some(s) => s.state = "running".to_string(),
                None => {
                    return Err(CloudWatchError::MetricStreamNotFound {
                        name: name.to_string(),
                    });
                }
            }
        }
        Ok(())
    }

    pub fn stop_metric_streams(&mut self, names: &[String]) -> Result<(), CloudWatchError> {
        for name in names {
            match self.metric_streams.get_mut(name) {
                Some(s) => s.state = "stopped".to_string(),
                None => {
                    return Err(CloudWatchError::MetricStreamNotFound {
                        name: name.to_string(),
                    });
                }
            }
        }
        Ok(())
    }

    // --- Alarm mute rules ---

    pub fn put_alarm_mute_rule(
        &mut self,
        name: &str,
        mute_type: &str,
        alarm_names: Vec<String>,
        description: Option<String>,
        account_id: &str,
        region: &str,
    ) -> String {
        let arn = format!("arn:aws:cloudwatch:{region}:{account_id}:alarm-mute-rule/{name}");
        let now = Utc::now().timestamp();
        let rule = AlarmMuteRule {
            name: name.to_string(),
            arn: arn.clone(),
            description,
            mute_type: mute_type.to_string(),
            alarm_names,
            status: "ACTIVE".to_string(),
            last_updated_timestamp: now,
        };
        self.alarm_mute_rules.insert(name.to_string(), rule);
        arn
    }

    pub fn get_alarm_mute_rule(&self, name: &str) -> Result<&AlarmMuteRule, CloudWatchError> {
        self.alarm_mute_rules
            .get(name)
            .ok_or(CloudWatchError::AlarmMuteRuleNotFound {
                name: name.to_string(),
            })
    }

    pub fn delete_alarm_mute_rule(&mut self, name: &str) -> Result<(), CloudWatchError> {
        if self.alarm_mute_rules.remove(name).is_none() {
            return Err(CloudWatchError::AlarmMuteRuleNotFound {
                name: name.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_alarm_mute_rules(&self) -> Vec<&AlarmMuteRule> {
        self.alarm_mute_rules.values().collect()
    }
}
