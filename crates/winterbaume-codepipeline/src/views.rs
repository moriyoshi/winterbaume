//! Serde-compatible view types for CodePipeline state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CodePipelineService;
use crate::state::CodePipelineState;
use crate::types::*;

/// Serializable view of the entire CodePipeline state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CodePipelineStateView {
    /// Pipelines keyed by pipeline name.
    #[serde(default)]
    pub pipelines: HashMap<String, PipelineView>,
    /// Custom action types keyed by "category/provider/version".
    #[serde(default)]
    pub custom_action_types: HashMap<String, CustomActionTypeView>,
    /// Webhooks keyed by webhook name.
    #[serde(default)]
    pub webhooks: HashMap<String, WebhookView>,
    /// Pipeline executions.
    #[serde(default)]
    pub pipeline_executions: Vec<PipelineExecutionView>,
    /// Jobs keyed by job ID.
    #[serde(default)]
    pub jobs: HashMap<String, ActionJobView>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PipelineView {
    pub name: String,
    pub arn: String,
    pub role_arn: String,
    pub stages: serde_json::Value,
    pub version: i32,
    pub created: String,
    pub updated: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub disabled_transitions: Vec<DisabledTransitionView>,
    /// Raw JSON for the `artifact_store` Terraform block(s).
    #[serde(default)]
    pub artifact_store: Option<serde_json::Value>,
    /// Raw JSON for the `trigger` Terraform block(s).
    #[serde(default)]
    pub trigger: Option<serde_json::Value>,
    /// Raw JSON for the `variable` Terraform block(s).
    #[serde(default)]
    pub variable: Option<serde_json::Value>,
    /// Pipeline execution mode (e.g. `QUEUED`, `SUPERSEDED`, `PARALLEL`).
    #[serde(default)]
    pub execution_mode: Option<String>,
    /// Pipeline type (e.g. `V1`, `V2`).
    #[serde(default)]
    pub pipeline_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisabledTransitionView {
    pub stage_name: String,
    pub transition_type: String,
    pub reason: String,
    pub last_changed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomActionTypeView {
    pub category: String,
    pub provider: String,
    pub version: String,
    #[serde(default)]
    pub settings: Option<serde_json::Value>,
    #[serde(default)]
    pub configuration_properties: Option<serde_json::Value>,
    pub input_artifact_min: i32,
    pub input_artifact_max: i32,
    pub output_artifact_min: i32,
    pub output_artifact_max: i32,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub created: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookView {
    pub name: String,
    pub arn: String,
    pub url: String,
    pub definition: serde_json::Value,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub registered_with_third_party: bool,
    pub created: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineExecutionView {
    pub pipeline_execution_id: String,
    pub pipeline_name: String,
    pub pipeline_version: i32,
    pub status: String,
    #[serde(default)]
    pub status_summary: Option<String>,
    pub start_time: String,
    pub last_update_time: String,
    #[serde(default)]
    pub trigger: Option<serde_json::Value>,
    #[serde(default)]
    pub source_revisions: Vec<serde_json::Value>,
    #[serde(default)]
    pub variables: Vec<serde_json::Value>,
    #[serde(default)]
    pub execution_mode: Option<String>,
    #[serde(default)]
    pub execution_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionJobView {
    pub id: String,
    pub nonce: String,
    pub status: String,
    pub account_id: String,
    pub action_type_id: serde_json::Value,
    #[serde(default)]
    pub pipeline_context: Option<serde_json::Value>,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}

fn parse_datetime(s: &str) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(|_| chrono::Utc::now())
}

fn parse_job_status(s: &str) -> JobStatus {
    match s {
        "InProgress" => JobStatus::InProgress,
        "Succeeded" => JobStatus::Succeeded,
        "Failed" => JobStatus::Failed,
        _ => JobStatus::Created,
    }
}

// --- From internal types to view types ---

impl From<&CodePipelineState> for CodePipelineStateView {
    fn from(state: &CodePipelineState) -> Self {
        CodePipelineStateView {
            pipelines: state
                .pipelines
                .iter()
                .map(|(k, p)| {
                    (
                        k.clone(),
                        PipelineView {
                            name: p.name.clone(),
                            arn: p.arn.clone(),
                            role_arn: p.role_arn.clone(),
                            stages: p.stages.clone(),
                            version: p.version,
                            created: p.created.to_rfc3339(),
                            updated: p.updated.to_rfc3339(),
                            tags: p.tags.clone(),
                            disabled_transitions: p
                                .disabled_transitions
                                .iter()
                                .map(|((sn, tt), dt)| DisabledTransitionView {
                                    stage_name: sn.clone(),
                                    transition_type: tt.clone(),
                                    reason: dt.reason.clone(),
                                    last_changed_at: dt.last_changed_at.to_rfc3339(),
                                })
                                .collect(),
                            artifact_store: p.artifact_store.clone(),
                            trigger: p.trigger.clone(),
                            variable: p.variable.clone(),
                            execution_mode: p.execution_mode.clone(),
                            pipeline_type: p.pipeline_type.clone(),
                        },
                    )
                })
                .collect(),
            custom_action_types: state
                .custom_action_types
                .iter()
                .map(|(k, a)| {
                    let view_key = format!("{}/{}/{}", k.category, k.provider, k.version);
                    (
                        view_key,
                        CustomActionTypeView {
                            category: a.category.clone(),
                            provider: a.provider.clone(),
                            version: a.version.clone(),
                            settings: a.settings.clone(),
                            configuration_properties: a.configuration_properties.clone(),
                            input_artifact_min: a.input_artifact_details.minimum_count,
                            input_artifact_max: a.input_artifact_details.maximum_count,
                            output_artifact_min: a.output_artifact_details.minimum_count,
                            output_artifact_max: a.output_artifact_details.maximum_count,
                            tags: a.tags.clone(),
                            created: a.created.to_rfc3339(),
                        },
                    )
                })
                .collect(),
            webhooks: state
                .webhooks
                .iter()
                .map(|(k, w)| {
                    (
                        k.clone(),
                        WebhookView {
                            name: w.name.clone(),
                            arn: w.arn.clone(),
                            url: w.url.clone(),
                            definition: w.definition.clone(),
                            tags: w.tags.clone(),
                            registered_with_third_party: w.registered_with_third_party,
                            created: w.created.to_rfc3339(),
                        },
                    )
                })
                .collect(),
            pipeline_executions: state
                .pipeline_executions
                .iter()
                .map(|e| PipelineExecutionView {
                    pipeline_execution_id: e.pipeline_execution_id.clone(),
                    pipeline_name: e.pipeline_name.clone(),
                    pipeline_version: e.pipeline_version,
                    status: e.status.clone(),
                    status_summary: e.status_summary.clone(),
                    start_time: e.start_time.to_rfc3339(),
                    last_update_time: e.last_update_time.to_rfc3339(),
                    trigger: e.trigger.clone(),
                    source_revisions: e.source_revisions.clone(),
                    variables: e.variables.clone(),
                    execution_mode: e.execution_mode.clone(),
                    execution_type: e.execution_type.clone(),
                })
                .collect(),
            jobs: state
                .jobs
                .iter()
                .map(|(k, j)| {
                    (
                        k.clone(),
                        ActionJobView {
                            id: j.id.clone(),
                            nonce: j.nonce.clone(),
                            status: j.status.to_string(),
                            account_id: j.account_id.clone(),
                            action_type_id: j.action_type_id.clone(),
                            pipeline_context: j.pipeline_context.clone(),
                            data: j.data.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<CodePipelineStateView> for CodePipelineState {
    fn from(view: CodePipelineStateView) -> Self {
        CodePipelineState {
            pipelines: view
                .pipelines
                .into_iter()
                .map(|(k, p)| {
                    let disabled_transitions = p
                        .disabled_transitions
                        .into_iter()
                        .map(|dt| {
                            (
                                (dt.stage_name, dt.transition_type),
                                DisabledTransition {
                                    reason: dt.reason,
                                    last_changed_at: parse_datetime(&dt.last_changed_at),
                                },
                            )
                        })
                        .collect();
                    (
                        k,
                        crate::types::Pipeline {
                            name: p.name,
                            arn: p.arn,
                            role_arn: p.role_arn,
                            stages: p.stages,
                            version: p.version,
                            created: parse_datetime(&p.created),
                            updated: parse_datetime(&p.updated),
                            tags: p.tags,
                            disabled_transitions,
                            artifact_store: p.artifact_store,
                            trigger: p.trigger,
                            variable: p.variable,
                            execution_mode: p.execution_mode,
                            pipeline_type: p.pipeline_type,
                        },
                    )
                })
                .collect(),
            custom_action_types: view
                .custom_action_types
                .into_values()
                .map(|a| {
                    let key = ActionTypeKey {
                        category: a.category.clone(),
                        provider: a.provider.clone(),
                        version: a.version.clone(),
                    };
                    (
                        key,
                        CustomActionType {
                            category: a.category,
                            provider: a.provider,
                            version: a.version,
                            settings: a.settings,
                            configuration_properties: a.configuration_properties,
                            input_artifact_details: ArtifactDetailsData {
                                minimum_count: a.input_artifact_min,
                                maximum_count: a.input_artifact_max,
                            },
                            output_artifact_details: ArtifactDetailsData {
                                minimum_count: a.output_artifact_min,
                                maximum_count: a.output_artifact_max,
                            },
                            tags: a.tags,
                            created: parse_datetime(&a.created),
                        },
                    )
                })
                .collect(),
            webhooks: view
                .webhooks
                .into_iter()
                .map(|(k, w)| {
                    (
                        k,
                        Webhook {
                            name: w.name,
                            arn: w.arn,
                            url: w.url,
                            definition: w.definition,
                            tags: w.tags,
                            registered_with_third_party: w.registered_with_third_party,
                            created: parse_datetime(&w.created),
                        },
                    )
                })
                .collect(),
            pipeline_executions: view
                .pipeline_executions
                .into_iter()
                .map(|e| PipelineExecution {
                    pipeline_execution_id: e.pipeline_execution_id,
                    pipeline_name: e.pipeline_name,
                    pipeline_version: e.pipeline_version,
                    status: e.status,
                    status_summary: e.status_summary,
                    start_time: parse_datetime(&e.start_time),
                    last_update_time: parse_datetime(&e.last_update_time),
                    trigger: e.trigger,
                    source_revisions: e.source_revisions,
                    variables: e.variables,
                    execution_mode: e.execution_mode,
                    execution_type: e.execution_type,
                })
                .collect(),
            jobs: view
                .jobs
                .into_iter()
                .map(|(k, j)| {
                    (
                        k,
                        ActionJob {
                            id: j.id,
                            nonce: j.nonce,
                            status: parse_job_status(&j.status),
                            account_id: j.account_id,
                            action_type_id: j.action_type_id,
                            pipeline_context: j.pipeline_context,
                            data: j.data,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for CodePipelineService {
    type StateView = CodePipelineStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        CodePipelineStateView::from(&*guard)
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
            *guard = CodePipelineState::from(view);
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
            let merged = CodePipelineState::from(view);
            for (k, v) in merged.pipelines {
                guard.pipelines.insert(k, v);
            }
            for (k, v) in merged.custom_action_types {
                guard.custom_action_types.insert(k, v);
            }
            for (k, v) in merged.webhooks {
                guard.webhooks.insert(k, v);
            }
            guard.pipeline_executions.extend(merged.pipeline_executions);
            for (k, v) in merged.jobs {
                guard.jobs.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
