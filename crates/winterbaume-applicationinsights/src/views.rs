use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ApplicationInsightsService;
use crate::state::{
    ApplicationInsightsState, ApplicationRecord, ComponentRecord, LogPatternRecord, ProblemRecord,
    WorkloadRecord,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationInsightsStateView {
    #[serde(default)]
    pub applications: HashMap<String, ApplicationRecordView>,
    /// Stored as a flat map, keyed by `"{resource_group_name}::{component_name}"`.
    #[serde(default)]
    pub components: HashMap<String, ComponentRecordView>,
    /// Keyed by `"{resource_group_name}::{pattern_set_name}::{pattern_name}"`.
    #[serde(default)]
    pub log_patterns: HashMap<String, LogPatternRecordView>,
    /// Keyed by `"{resource_group_name}::{workload_id}"`.
    #[serde(default)]
    pub workloads: HashMap<String, WorkloadRecordView>,
    #[serde(default)]
    pub problems: Vec<ProblemRecordView>,
    #[serde(default)]
    pub observations: HashMap<String, Value>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
    #[serde(default)]
    pub configuration_history: Vec<Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationRecordView {
    pub resource_group_name: String,
    pub account_id: String,
    #[serde(default)]
    pub remarks: Option<String>,
    pub life_cycle: String,
    #[serde(default)]
    pub ops_item_sns_topic_arn: Option<String>,
    #[serde(default)]
    pub sns_notification_arn: Option<String>,
    #[serde(default)]
    pub ops_center_enabled: Option<bool>,
    #[serde(default)]
    pub cwe_monitor_enabled: Option<bool>,
    #[serde(default)]
    pub auto_config_enabled: Option<bool>,
    #[serde(default)]
    pub attach_missing_permission: Option<bool>,
    #[serde(default)]
    pub discovery_type: Option<String>,
    #[serde(default)]
    pub component_configurations: HashMap<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComponentRecordView {
    pub component_name: String,
    #[serde(default)]
    pub resource_type: Option<String>,
    #[serde(default)]
    pub os_type: Option<String>,
    #[serde(default)]
    pub tier: Option<String>,
    pub monitor: bool,
    #[serde(default)]
    pub component_remarks: Option<String>,
    #[serde(default)]
    pub configuration: Option<Value>,
    pub auto_configuration_enabled: bool,
    #[serde(default)]
    pub resource_list: Vec<String>,
    #[serde(default)]
    pub workload_id_per_resource: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogPatternRecordView {
    pub pattern_set_name: String,
    pub pattern_name: String,
    pub pattern: String,
    pub rank: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkloadRecordView {
    pub workload_id: String,
    pub component_name: String,
    pub workload_name: String,
    pub tier: String,
    #[serde(default)]
    pub workload_remarks: Option<String>,
    #[serde(default)]
    pub workload_configuration: Option<Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProblemRecordView {
    pub id: String,
    pub resource_group_name: String,
    pub title: String,
    pub status: String,
    #[serde(default)]
    pub severity: Option<String>,
    #[serde(default)]
    pub start_time: Option<i64>,
    #[serde(default)]
    pub end_time: Option<i64>,
    #[serde(default)]
    pub affected_resource: Option<String>,
    #[serde(default)]
    pub feedback: HashMap<String, String>,
}

macro_rules! basic_from {
    ($view:ident, $domain:ident { $($field:ident),* $(,)? }) => {
        impl From<&$domain> for $view {
            fn from(s: &$domain) -> Self { Self { $($field: s.$field.clone(),)* } }
        }
        impl From<$view> for $domain {
            fn from(v: $view) -> Self { Self { $($field: v.$field,)* } }
        }
    };
}

basic_from!(
    ApplicationRecordView,
    ApplicationRecord {
        resource_group_name,
        account_id,
        remarks,
        life_cycle,
        ops_item_sns_topic_arn,
        sns_notification_arn,
        ops_center_enabled,
        cwe_monitor_enabled,
        auto_config_enabled,
        attach_missing_permission,
        discovery_type,
        component_configurations
    }
);

basic_from!(
    ComponentRecordView,
    ComponentRecord {
        component_name,
        resource_type,
        os_type,
        tier,
        monitor,
        component_remarks,
        configuration,
        auto_configuration_enabled,
        resource_list,
        workload_id_per_resource
    }
);

basic_from!(
    LogPatternRecordView,
    LogPatternRecord {
        pattern_set_name,
        pattern_name,
        pattern,
        rank
    }
);

basic_from!(
    WorkloadRecordView,
    WorkloadRecord {
        workload_id,
        component_name,
        workload_name,
        tier,
        workload_remarks,
        workload_configuration
    }
);

basic_from!(
    ProblemRecordView,
    ProblemRecord {
        id,
        resource_group_name,
        title,
        status,
        severity,
        start_time,
        end_time,
        affected_resource,
        feedback
    }
);

fn join2(a: &str, b: &str) -> String {
    format!("{a}::{b}")
}

fn join3(a: &str, b: &str, c: &str) -> String {
    format!("{a}::{b}::{c}")
}

fn split2(s: &str) -> (String, String) {
    let mut parts = s.splitn(2, "::");
    let a = parts.next().unwrap_or("").to_string();
    let b = parts.next().unwrap_or("").to_string();
    (a, b)
}

fn split3(s: &str) -> (String, String, String) {
    let mut parts = s.splitn(3, "::");
    let a = parts.next().unwrap_or("").to_string();
    let b = parts.next().unwrap_or("").to_string();
    let c = parts.next().unwrap_or("").to_string();
    (a, b, c)
}

impl From<&ApplicationInsightsState> for ApplicationInsightsStateView {
    fn from(state: &ApplicationInsightsState) -> Self {
        Self {
            applications: state
                .applications
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            components: state
                .components
                .iter()
                .map(|((rg, name), v)| (join2(rg, name), v.into()))
                .collect(),
            log_patterns: state
                .log_patterns
                .iter()
                .map(|((rg, set, name), v)| (join3(rg, set, name), v.into()))
                .collect(),
            workloads: state
                .workloads
                .iter()
                .map(|((rg, id), v)| (join2(rg, id), v.into()))
                .collect(),
            problems: state.problems.iter().map(Into::into).collect(),
            observations: state.observations.clone(),
            tags: state.tags.clone(),
            configuration_history: state.configuration_history.clone(),
        }
    }
}

impl From<ApplicationInsightsStateView> for ApplicationInsightsState {
    fn from(view: ApplicationInsightsStateView) -> Self {
        Self {
            applications: view
                .applications
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            components: view
                .components
                .into_iter()
                .map(|(k, v)| (split2(&k), v.into()))
                .collect(),
            log_patterns: view
                .log_patterns
                .into_iter()
                .map(|(k, v)| (split3(&k), v.into()))
                .collect(),
            workloads: view
                .workloads
                .into_iter()
                .map(|(k, v)| (split2(&k), v.into()))
                .collect(),
            problems: view.problems.into_iter().map(Into::into).collect(),
            observations: view.observations,
            tags: view.tags,
            configuration_history: view.configuration_history,
        }
    }
}

impl StatefulService for ApplicationInsightsService {
    type StateView = ApplicationInsightsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ApplicationInsightsStateView::from(&*guard)
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
            *guard = ApplicationInsightsState::from(view);
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
            for (k, v) in view.applications {
                guard.applications.insert(k, v.into());
            }
            for (k, v) in view.components {
                guard.components.insert(split2(&k), v.into());
            }
            for (k, v) in view.log_patterns {
                guard.log_patterns.insert(split3(&k), v.into());
            }
            for (k, v) in view.workloads {
                guard.workloads.insert(split2(&k), v.into());
            }
            for p in view.problems {
                guard.problems.push(p.into());
            }
            for (k, v) in view.observations {
                guard.observations.insert(k, v);
            }
            for (k, v) in view.tags {
                guard.tags.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
