use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::AppFlowService;
use crate::state::AppFlowState;
use crate::types::Flow;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppFlowStateView {
    #[serde(default)]
    pub flows: HashMap<String, FlowView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowView {
    pub flow_name: String,
    pub flow_arn: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub kms_arn: Option<String>,
    pub flow_status: String,
    #[serde(default)]
    pub flow_status_message: Option<String>,
    #[serde(default)]
    pub trigger_config: Value,
    #[serde(default)]
    pub source_flow_config: Value,
    #[serde(default)]
    pub destination_flow_config_list: Value,
    #[serde(default)]
    pub tasks: Value,
    #[serde(default)]
    pub metadata_catalog_config: Option<Value>,
    pub created_at: i64,
    pub last_updated_at: i64,
    pub created_by: String,
    pub last_updated_by: String,
    pub schema_version: i64,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub last_execution_id: Option<String>,
}

impl From<&Flow> for FlowView {
    fn from(f: &Flow) -> Self {
        Self {
            flow_name: f.flow_name.clone(),
            flow_arn: f.flow_arn.clone(),
            description: f.description.clone(),
            kms_arn: f.kms_arn.clone(),
            flow_status: f.flow_status.clone(),
            flow_status_message: f.flow_status_message.clone(),
            trigger_config: f.trigger_config.clone(),
            source_flow_config: f.source_flow_config.clone(),
            destination_flow_config_list: f.destination_flow_config_list.clone(),
            tasks: f.tasks.clone(),
            metadata_catalog_config: f.metadata_catalog_config.clone(),
            created_at: f.created_at,
            last_updated_at: f.last_updated_at,
            created_by: f.created_by.clone(),
            last_updated_by: f.last_updated_by.clone(),
            schema_version: f.schema_version,
            tags: f.tags.clone(),
            last_execution_id: f.last_execution_id.clone(),
        }
    }
}

impl From<FlowView> for Flow {
    fn from(v: FlowView) -> Self {
        Self {
            flow_name: v.flow_name,
            flow_arn: v.flow_arn,
            description: v.description,
            kms_arn: v.kms_arn,
            flow_status: v.flow_status,
            flow_status_message: v.flow_status_message,
            trigger_config: v.trigger_config,
            source_flow_config: v.source_flow_config,
            destination_flow_config_list: v.destination_flow_config_list,
            tasks: v.tasks,
            metadata_catalog_config: v.metadata_catalog_config,
            created_at: v.created_at,
            last_updated_at: v.last_updated_at,
            created_by: v.created_by,
            last_updated_by: v.last_updated_by,
            schema_version: v.schema_version,
            tags: v.tags,
            last_execution_id: v.last_execution_id,
        }
    }
}

impl From<&AppFlowState> for AppFlowStateView {
    fn from(state: &AppFlowState) -> Self {
        Self {
            flows: state
                .flows
                .iter()
                .map(|(k, v)| (k.clone(), FlowView::from(v)))
                .collect(),
        }
    }
}

impl From<AppFlowStateView> for AppFlowState {
    fn from(view: AppFlowStateView) -> Self {
        Self {
            flows: view
                .flows
                .into_iter()
                .map(|(k, v)| (k, Flow::from(v)))
                .collect(),
        }
    }
}

impl StatefulService for AppFlowService {
    type StateView = AppFlowStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        AppFlowStateView::from(&*guard)
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
            *guard = AppFlowState::from(view);
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
            for (k, v) in view.flows {
                guard.flows.insert(k, Flow::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
