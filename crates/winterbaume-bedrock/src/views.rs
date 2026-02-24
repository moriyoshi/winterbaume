//! Serde-compatible view types for Bedrock state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::BedrockService;
use crate::state::BedrockState;
use crate::types::{
    CustomModel, EvaluationJob, Guardrail, InferenceProfile, LoggingConfiguration, ModelCopyJob,
    ModelCustomizationJob, ModelImportJob, ModelInvocationJob, PromptRouter,
    ProvisionedModelThroughput, ResourceTag,
};

/// Serializable view of the entire Bedrock state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BedrockStateView {
    #[serde(default)]
    pub customization_jobs: HashMap<String, ModelCustomizationJob>,
    #[serde(default)]
    pub custom_models: HashMap<String, CustomModel>,
    pub logging_config: Option<LoggingConfiguration>,
    #[serde(default)]
    pub tags: HashMap<String, Vec<ResourceTag>>,
    #[serde(default)]
    pub guardrails: HashMap<String, Guardrail>,
    #[serde(default)]
    pub provisioned_models: HashMap<String, ProvisionedModelThroughput>,
    #[serde(default)]
    pub inference_profiles: HashMap<String, InferenceProfile>,
    #[serde(default)]
    pub prompt_routers: HashMap<String, PromptRouter>,
    #[serde(default)]
    pub evaluation_jobs: HashMap<String, EvaluationJob>,
    #[serde(default)]
    pub model_invocation_jobs: HashMap<String, ModelInvocationJob>,
    #[serde(default)]
    pub model_import_jobs: HashMap<String, ModelImportJob>,
    #[serde(default)]
    pub model_copy_jobs: HashMap<String, ModelCopyJob>,
}

// --- From internal types to view types ---

impl From<&BedrockState> for BedrockStateView {
    fn from(state: &BedrockState) -> Self {
        BedrockStateView {
            customization_jobs: state.customization_jobs.clone(),
            custom_models: state.custom_models.clone(),
            logging_config: state.logging_config.clone(),
            tags: state.tags.clone(),
            guardrails: state.guardrails.clone(),
            provisioned_models: state.provisioned_models.clone(),
            inference_profiles: state.inference_profiles.clone(),
            prompt_routers: state.prompt_routers.clone(),
            evaluation_jobs: state.evaluation_jobs.clone(),
            model_invocation_jobs: state.model_invocation_jobs.clone(),
            model_import_jobs: state.model_import_jobs.clone(),
            model_copy_jobs: state.model_copy_jobs.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<BedrockStateView> for BedrockState {
    fn from(view: BedrockStateView) -> Self {
        BedrockState {
            customization_jobs: view.customization_jobs,
            custom_models: view.custom_models,
            logging_config: view.logging_config,
            tags: view.tags,
            guardrails: view.guardrails,
            provisioned_models: view.provisioned_models,
            inference_profiles: view.inference_profiles,
            prompt_routers: view.prompt_routers,
            evaluation_jobs: view.evaluation_jobs,
            model_invocation_jobs: view.model_invocation_jobs,
            model_import_jobs: view.model_import_jobs,
            model_copy_jobs: view.model_copy_jobs,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for BedrockService {
    type StateView = BedrockStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        BedrockStateView::from(&*guard)
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
            *guard = BedrockState::from(view);
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
            for (k, v) in view.customization_jobs {
                guard.customization_jobs.insert(k, v);
            }
            for (k, v) in view.custom_models {
                guard.custom_models.insert(k, v);
            }
            if let Some(lc) = view.logging_config {
                guard.logging_config = Some(lc);
            }
            for (k, v) in view.tags {
                guard.tags.insert(k, v);
            }
            for (k, v) in view.guardrails {
                guard.guardrails.insert(k, v);
            }
            for (k, v) in view.provisioned_models {
                guard.provisioned_models.insert(k, v);
            }
            for (k, v) in view.inference_profiles {
                guard.inference_profiles.insert(k, v);
            }
            for (k, v) in view.prompt_routers {
                guard.prompt_routers.insert(k, v);
            }
            for (k, v) in view.evaluation_jobs {
                guard.evaluation_jobs.insert(k, v);
            }
            for (k, v) in view.model_invocation_jobs {
                guard.model_invocation_jobs.insert(k, v);
            }
            for (k, v) in view.model_import_jobs {
                guard.model_import_jobs.insert(k, v);
            }
            for (k, v) in view.model_copy_jobs {
                guard.model_copy_jobs.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
