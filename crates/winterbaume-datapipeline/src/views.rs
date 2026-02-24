//! Serde-compatible view types for Data Pipeline state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::DataPipelineService;
use crate::state::DataPipelineState;
use crate::types::{Pipeline, PipelineField, PipelineObject, PipelineStatus, PipelineTag};

/// Serializable view of the entire Data Pipeline state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataPipelineStateView {
    /// Pipelines keyed by pipeline ID.
    #[serde(default)]
    pub pipelines: HashMap<String, PipelineView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineView {
    pub pipeline_id: String,
    pub name: String,
    pub description: String,
    pub unique_id: String,
    #[serde(default)]
    pub tags: Vec<PipelineTagView>,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub pipeline_objects: Vec<PipelineObjectView>,
    #[serde(default)]
    pub parameter_objects: Vec<Value>,
    #[serde(default)]
    pub parameter_values: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineTagView {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineObjectView {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub fields: Vec<PipelineFieldView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineFieldView {
    pub key: String,
    pub string_value: Option<String>,
    pub ref_value: Option<String>,
}

// --- From internal types to view types ---

impl From<&DataPipelineState> for DataPipelineStateView {
    fn from(state: &DataPipelineState) -> Self {
        DataPipelineStateView {
            pipelines: state
                .pipelines
                .iter()
                .map(|(k, v)| (k.clone(), PipelineView::from(v)))
                .collect(),
        }
    }
}

impl From<&Pipeline> for PipelineView {
    fn from(p: &Pipeline) -> Self {
        PipelineView {
            pipeline_id: p.pipeline_id.clone(),
            name: p.name.clone(),
            description: p.description.clone(),
            unique_id: p.unique_id.clone(),
            tags: p
                .tags
                .iter()
                .map(|t| PipelineTagView {
                    key: t.key.clone(),
                    value: t.value.clone(),
                })
                .collect(),
            created_at: p.created_at,
            status: p.status.as_str().to_string(),
            pipeline_objects: p
                .pipeline_objects
                .iter()
                .map(|o| PipelineObjectView {
                    id: o.id.clone(),
                    name: o.name.clone(),
                    fields: o
                        .fields
                        .iter()
                        .map(|f| PipelineFieldView {
                            key: f.key.clone(),
                            string_value: f.string_value.clone(),
                            ref_value: f.ref_value.clone(),
                        })
                        .collect(),
                })
                .collect(),
            parameter_objects: p.parameter_objects.clone(),
            parameter_values: p.parameter_values.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<DataPipelineStateView> for DataPipelineState {
    fn from(view: DataPipelineStateView) -> Self {
        let pipelines = view
            .pipelines
            .into_iter()
            .map(|(k, v)| {
                let status = match v.status.as_str() {
                    "ACTIVE" => PipelineStatus::Active,
                    "ACTIVATING" => PipelineStatus::Activating,
                    "DEACTIVATING" => PipelineStatus::Deactivating,
                    "INACTIVE" => PipelineStatus::Inactive,
                    "DELETING" => PipelineStatus::Deleting,
                    _ => PipelineStatus::Pending,
                };
                (
                    k,
                    Pipeline {
                        pipeline_id: v.pipeline_id,
                        name: v.name,
                        description: v.description,
                        unique_id: v.unique_id,
                        tags: v
                            .tags
                            .into_iter()
                            .map(|t| PipelineTag {
                                key: t.key,
                                value: t.value,
                            })
                            .collect(),
                        created_at: v.created_at,
                        status,
                        pipeline_objects: v
                            .pipeline_objects
                            .into_iter()
                            .map(|o| PipelineObject {
                                id: o.id,
                                name: o.name,
                                fields: o
                                    .fields
                                    .into_iter()
                                    .map(|f| PipelineField {
                                        key: f.key,
                                        string_value: f.string_value,
                                        ref_value: f.ref_value,
                                    })
                                    .collect(),
                            })
                            .collect(),
                        parameter_objects: v.parameter_objects,
                        parameter_values: v.parameter_values,
                    },
                )
            })
            .collect();

        let mut state = DataPipelineState::default();
        state.pipelines = pipelines;
        state
    }
}

// --- StatefulService implementation ---

impl StatefulService for DataPipelineService {
    type StateView = DataPipelineStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        DataPipelineStateView::from(&*guard)
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
            *guard = DataPipelineState::from(view);
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
            let incoming = DataPipelineState::from(view);
            for (k, v) in incoming.pipelines {
                guard.pipelines.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
