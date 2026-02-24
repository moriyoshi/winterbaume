use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ApplicationSignalsService;
use crate::state::ApplicationSignalsState;
use crate::types::{GroupingConfiguration, ServiceEntry, ServiceLevelObjective};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationSignalsStateView {
    #[serde(default)]
    pub slos: HashMap<String, ServiceLevelObjectiveView>,
    #[serde(default)]
    pub grouping_configuration: Option<GroupingConfigurationView>,
    #[serde(default)]
    pub services: Vec<ServiceEntryView>,
    #[serde(default)]
    pub discovery_started: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServiceLevelObjectiveView {
    pub id: String,
    pub arn: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub evaluation_type: String,
    pub metric_source_type: String,
    pub created_time: i64,
    pub last_updated_time: i64,
    #[serde(default)]
    pub sli: Option<Value>,
    #[serde(default)]
    pub request_based_sli: Option<Value>,
    #[serde(default)]
    pub goal: Option<Value>,
    #[serde(default)]
    pub burn_rate_configurations: Vec<Value>,
    #[serde(default)]
    pub exclusion_windows: Vec<Value>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupingConfigurationView {
    #[serde(default)]
    pub grouping_attribute_definitions: Vec<Value>,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServiceEntryView {
    #[serde(default)]
    pub key_attributes: HashMap<String, String>,
    #[serde(default)]
    pub attribute_maps: Vec<HashMap<String, String>>,
    #[serde(default)]
    pub metric_references: Vec<Value>,
    #[serde(default)]
    pub log_group_references: Vec<HashMap<String, String>>,
    #[serde(default)]
    pub service_groups: Vec<Value>,
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
    ServiceLevelObjectiveView,
    ServiceLevelObjective {
        id,
        arn,
        name,
        description,
        evaluation_type,
        metric_source_type,
        created_time,
        last_updated_time,
        sli,
        request_based_sli,
        goal,
        burn_rate_configurations,
        exclusion_windows,
        tags
    }
);

basic_from!(
    GroupingConfigurationView,
    GroupingConfiguration {
        grouping_attribute_definitions,
        updated_at
    }
);

basic_from!(
    ServiceEntryView,
    ServiceEntry {
        key_attributes,
        attribute_maps,
        metric_references,
        log_group_references,
        service_groups
    }
);

impl From<&ApplicationSignalsState> for ApplicationSignalsStateView {
    fn from(state: &ApplicationSignalsState) -> Self {
        Self {
            slos: state
                .slos
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            grouping_configuration: state.grouping_configuration.as_ref().map(Into::into),
            services: state.services.iter().map(Into::into).collect(),
            discovery_started: state.discovery_started,
        }
    }
}

impl From<ApplicationSignalsStateView> for ApplicationSignalsState {
    fn from(view: ApplicationSignalsStateView) -> Self {
        Self {
            slos: view.slos.into_iter().map(|(k, v)| (k, v.into())).collect(),
            grouping_configuration: view.grouping_configuration.map(Into::into),
            services: view.services.into_iter().map(Into::into).collect(),
            discovery_started: view.discovery_started,
        }
    }
}

impl StatefulService for ApplicationSignalsService {
    type StateView = ApplicationSignalsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ApplicationSignalsStateView::from(&*guard)
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
            *guard = ApplicationSignalsState::from(view);
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
            for (k, v) in view.slos {
                guard.slos.insert(k, v.into());
            }
            if let Some(g) = view.grouping_configuration {
                guard.grouping_configuration = Some(g.into());
            }
            for s in view.services {
                guard.services.push(s.into());
            }
            if view.discovery_started {
                guard.discovery_started = true;
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
