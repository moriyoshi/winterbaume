use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::FisService;
use crate::state::FisState;
use crate::types::{
    ExperimentTemplate, ExperimentTemplateAction, ExperimentTemplateStopCondition,
    ExperimentTemplateTarget, ExperimentTemplateTargetFilter,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FisStateView {
    #[serde(default)]
    pub experiment_templates: HashMap<String, ExperimentTemplateView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentTemplateView {
    pub id: String,
    pub arn: String,
    pub description: String,
    pub role_arn: String,
    #[serde(default)]
    pub targets: HashMap<String, ExperimentTemplateTargetView>,
    #[serde(default)]
    pub actions: HashMap<String, ExperimentTemplateActionView>,
    #[serde(default)]
    pub stop_conditions: Vec<ExperimentTemplateStopConditionView>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub creation_time: String,
    pub last_update_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentTemplateTargetView {
    pub resource_type: String,
    #[serde(default)]
    pub resource_arns: Vec<String>,
    #[serde(default)]
    pub resource_tags: HashMap<String, String>,
    #[serde(default)]
    pub filters: Vec<ExperimentTemplateTargetFilterView>,
    pub selection_mode: String,
    #[serde(default)]
    pub parameters: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentTemplateTargetFilterView {
    pub path: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentTemplateActionView {
    pub action_id: String,
    pub description: Option<String>,
    #[serde(default)]
    pub parameters: HashMap<String, String>,
    #[serde(default)]
    pub targets: HashMap<String, String>,
    #[serde(default)]
    pub start_after: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentTemplateStopConditionView {
    pub source: String,
    pub value: Option<String>,
}

// --- From internal types to view types ---

impl From<&FisState> for FisStateView {
    fn from(state: &FisState) -> Self {
        FisStateView {
            experiment_templates: state
                .experiment_templates
                .iter()
                .map(|(k, v)| (k.clone(), ExperimentTemplateView::from(v)))
                .collect(),
        }
    }
}

impl From<&ExperimentTemplate> for ExperimentTemplateView {
    fn from(t: &ExperimentTemplate) -> Self {
        ExperimentTemplateView {
            id: t.id.clone(),
            arn: t.arn.clone(),
            description: t.description.clone(),
            role_arn: t.role_arn.clone(),
            targets: t
                .targets
                .iter()
                .map(|(k, v)| (k.clone(), ExperimentTemplateTargetView::from(v)))
                .collect(),
            actions: t
                .actions
                .iter()
                .map(|(k, v)| (k.clone(), ExperimentTemplateActionView::from(v)))
                .collect(),
            stop_conditions: t
                .stop_conditions
                .iter()
                .map(ExperimentTemplateStopConditionView::from)
                .collect(),
            tags: t.tags.clone(),
            creation_time: t.creation_time.to_rfc3339(),
            last_update_time: t.last_update_time.to_rfc3339(),
        }
    }
}

impl From<&ExperimentTemplateTarget> for ExperimentTemplateTargetView {
    fn from(t: &ExperimentTemplateTarget) -> Self {
        ExperimentTemplateTargetView {
            resource_type: t.resource_type.clone(),
            resource_arns: t.resource_arns.clone(),
            resource_tags: t.resource_tags.clone(),
            filters: t
                .filters
                .iter()
                .map(ExperimentTemplateTargetFilterView::from)
                .collect(),
            selection_mode: t.selection_mode.clone(),
            parameters: t.parameters.clone(),
        }
    }
}

impl From<&ExperimentTemplateTargetFilter> for ExperimentTemplateTargetFilterView {
    fn from(f: &ExperimentTemplateTargetFilter) -> Self {
        ExperimentTemplateTargetFilterView {
            path: f.path.clone(),
            values: f.values.clone(),
        }
    }
}

impl From<&ExperimentTemplateAction> for ExperimentTemplateActionView {
    fn from(a: &ExperimentTemplateAction) -> Self {
        ExperimentTemplateActionView {
            action_id: a.action_id.clone(),
            description: a.description.clone(),
            parameters: a.parameters.clone(),
            targets: a.targets.clone(),
            start_after: a.start_after.clone(),
        }
    }
}

impl From<&ExperimentTemplateStopCondition> for ExperimentTemplateStopConditionView {
    fn from(sc: &ExperimentTemplateStopCondition) -> Self {
        ExperimentTemplateStopConditionView {
            source: sc.source.clone(),
            value: sc.value.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<FisStateView> for FisState {
    fn from(view: FisStateView) -> Self {
        FisState {
            experiment_templates: view
                .experiment_templates
                .into_iter()
                .map(|(k, v)| (k, ExperimentTemplate::from(v)))
                .collect(),
        }
    }
}

impl From<ExperimentTemplateView> for ExperimentTemplate {
    fn from(v: ExperimentTemplateView) -> Self {
        ExperimentTemplate {
            id: v.id,
            arn: v.arn,
            description: v.description,
            role_arn: v.role_arn,
            targets: v
                .targets
                .into_iter()
                .map(|(k, t)| (k, ExperimentTemplateTarget::from(t)))
                .collect(),
            actions: v
                .actions
                .into_iter()
                .map(|(k, a)| (k, ExperimentTemplateAction::from(a)))
                .collect(),
            stop_conditions: v
                .stop_conditions
                .into_iter()
                .map(ExperimentTemplateStopCondition::from)
                .collect(),
            tags: v.tags,
            creation_time: chrono::DateTime::parse_from_rfc3339(&v.creation_time)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            last_update_time: chrono::DateTime::parse_from_rfc3339(&v.last_update_time)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

impl From<ExperimentTemplateTargetView> for ExperimentTemplateTarget {
    fn from(v: ExperimentTemplateTargetView) -> Self {
        ExperimentTemplateTarget {
            resource_type: v.resource_type,
            resource_arns: v.resource_arns,
            resource_tags: v.resource_tags,
            filters: v
                .filters
                .into_iter()
                .map(ExperimentTemplateTargetFilter::from)
                .collect(),
            selection_mode: v.selection_mode,
            parameters: v.parameters,
        }
    }
}

impl From<ExperimentTemplateTargetFilterView> for ExperimentTemplateTargetFilter {
    fn from(v: ExperimentTemplateTargetFilterView) -> Self {
        ExperimentTemplateTargetFilter {
            path: v.path,
            values: v.values,
        }
    }
}

impl From<ExperimentTemplateActionView> for ExperimentTemplateAction {
    fn from(v: ExperimentTemplateActionView) -> Self {
        ExperimentTemplateAction {
            action_id: v.action_id,
            description: v.description,
            parameters: v.parameters,
            targets: v.targets,
            start_after: v.start_after,
        }
    }
}

impl From<ExperimentTemplateStopConditionView> for ExperimentTemplateStopCondition {
    fn from(v: ExperimentTemplateStopConditionView) -> Self {
        ExperimentTemplateStopCondition {
            source: v.source,
            value: v.value,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for FisService {
    type StateView = FisStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        FisStateView::from(&*guard)
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
            *guard = FisState::from(view);
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
            for (k, v) in view.experiment_templates {
                guard
                    .experiment_templates
                    .insert(k, ExperimentTemplate::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
