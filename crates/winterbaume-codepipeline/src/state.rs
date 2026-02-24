use std::collections::HashMap;

use chrono::Utc;
use serde_json::Value;
use thiserror::Error;
use uuid::Uuid;

use crate::types::*;

#[derive(Debug, Default)]
pub struct CodePipelineState {
    pub pipelines: HashMap<String, Pipeline>,
    pub custom_action_types: HashMap<ActionTypeKey, CustomActionType>,
    pub webhooks: HashMap<String, Webhook>,
    pub pipeline_executions: Vec<PipelineExecution>,
    pub jobs: HashMap<String, ActionJob>,
}

#[derive(Debug, Error)]
pub enum CodePipelineError {
    #[error("A pipeline with the name '{name}' already exists in account '{account_id}'")]
    PipelineAlreadyExists { name: String, account_id: String },

    #[error("Account '{account_id}' does not have a pipeline with name '{name}'")]
    PipelineNotFound { name: String, account_id: String },

    #[error(
        "The account with id '{account_id}' does not include a pipeline with the name '{name}'"
    )]
    ResourceNotFound { name: String, account_id: String },

    #[error("The specified action type already exists")]
    ActionTypeAlreadyExists,

    #[error("The specified action type cannot be found")]
    ActionTypeNotFound,

    #[error("The specified webhook was not found")]
    WebhookNotFound { name: String },

    #[error("A webhook with the name '{name}' already exists")]
    WebhookAlreadyExists { name: String },

    #[error("The job with id '{job_id}' was not found")]
    JobNotFound { job_id: String },

    #[error("The job nonce is invalid")]
    InvalidNonce,

    #[error("The pipeline execution was not found")]
    PipelineExecutionNotFound,

    #[error("The stage '{stage_name}' was not found in the pipeline '{pipeline_name}'")]
    StageNotFound {
        pipeline_name: String,
        stage_name: String,
    },
}

/// Extract pipeline name from a CodePipeline ARN.
/// Format: arn:aws:codepipeline:{region}:{account}:{name}
fn pipeline_name_from_arn(arn: &str) -> &str {
    arn.rsplit(':').next().unwrap_or(arn)
}

impl CodePipelineState {
    pub fn create_pipeline(
        &mut self,
        name: &str,
        role_arn: &str,
        stages: Value,
        account_id: &str,
        region: &str,
        initial_tags: HashMap<String, String>,
    ) -> Result<&Pipeline, CodePipelineError> {
        if self.pipelines.contains_key(name) {
            return Err(CodePipelineError::PipelineAlreadyExists {
                name: name.to_string(),
                account_id: account_id.to_string(),
            });
        }

        let arn = format!("arn:aws:codepipeline:{region}:{account_id}:{name}");
        let now = Utc::now();

        let pipeline = Pipeline {
            name: name.to_string(),
            arn,
            role_arn: role_arn.to_string(),
            stages,
            version: 1,
            created: now,
            updated: now,
            tags: initial_tags,
            disabled_transitions: HashMap::new(),
            artifact_store: None,
            trigger: None,
            variable: None,
            execution_mode: None,
            pipeline_type: None,
        };

        self.pipelines.insert(name.to_string(), pipeline);
        Ok(self.pipelines.get(name).unwrap())
    }

    pub fn get_pipeline(&self, name: &str) -> Result<&Pipeline, CodePipelineError> {
        self.pipelines
            .get(name)
            .ok_or_else(|| CodePipelineError::PipelineNotFound {
                name: name.to_string(),
                account_id: DEFAULT_ACCOUNT_ID_STR.to_string(),
            })
    }

    /// Delete a pipeline. Silently succeeds if the pipeline does not exist.
    pub fn delete_pipeline(&mut self, name: &str) {
        self.pipelines.remove(name);
    }

    pub fn list_pipelines(&self) -> Vec<&Pipeline> {
        self.pipelines.values().collect()
    }

    pub fn update_pipeline(
        &mut self,
        name: &str,
        role_arn: Option<&str>,
        stages: Option<Value>,
        account_id: &str,
    ) -> Result<&Pipeline, CodePipelineError> {
        let pipeline =
            self.pipelines
                .get_mut(name)
                .ok_or_else(|| CodePipelineError::ResourceNotFound {
                    name: name.to_string(),
                    account_id: account_id.to_string(),
                })?;
        if let Some(r) = role_arn {
            pipeline.role_arn = r.to_string();
        }
        if let Some(s) = stages {
            pipeline.stages = s;
        }
        pipeline.version += 1;
        pipeline.updated = Utc::now();
        Ok(pipeline)
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
        account_id: &str,
    ) -> Result<(), CodePipelineError> {
        let pipeline_name = pipeline_name_from_arn(resource_arn).to_string();
        let pipeline = self
            .pipelines
            .values_mut()
            .find(|p| p.arn == resource_arn)
            .ok_or_else(|| CodePipelineError::ResourceNotFound {
                name: pipeline_name.clone(),
                account_id: account_id.to_string(),
            })?;
        pipeline.tags.extend(tags);
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
        account_id: &str,
    ) -> Result<(), CodePipelineError> {
        let pipeline_name = pipeline_name_from_arn(resource_arn).to_string();
        let pipeline = self
            .pipelines
            .values_mut()
            .find(|p| p.arn == resource_arn)
            .ok_or_else(|| CodePipelineError::ResourceNotFound {
                name: pipeline_name.clone(),
                account_id: account_id.to_string(),
            })?;
        for key in tag_keys {
            pipeline.tags.remove(key);
        }
        Ok(())
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
        account_id: &str,
    ) -> Result<&HashMap<String, String>, CodePipelineError> {
        let pipeline_name = pipeline_name_from_arn(resource_arn).to_string();
        let pipeline = self
            .pipelines
            .values()
            .find(|p| p.arn == resource_arn)
            .ok_or_else(|| CodePipelineError::ResourceNotFound {
                name: pipeline_name.clone(),
                account_id: account_id.to_string(),
            })?;
        Ok(&pipeline.tags)
    }

    // ---- Custom Action Types ----

    pub fn create_custom_action_type(
        &mut self,
        category: &str,
        provider: &str,
        version: &str,
        settings: Option<Value>,
        configuration_properties: Option<Value>,
        input_artifact_details: ArtifactDetailsData,
        output_artifact_details: ArtifactDetailsData,
        tags: HashMap<String, String>,
    ) -> Result<&CustomActionType, CodePipelineError> {
        let key = ActionTypeKey {
            category: category.to_string(),
            provider: provider.to_string(),
            version: version.to_string(),
        };
        if self.custom_action_types.contains_key(&key) {
            // AWS actually returns the existing one without error; we do the same but
            // overwrite settings/tags to match AWS behaviour.
        }
        let action_type = CustomActionType {
            category: category.to_string(),
            provider: provider.to_string(),
            version: version.to_string(),
            settings,
            configuration_properties,
            input_artifact_details,
            output_artifact_details,
            tags,
            created: Utc::now(),
        };
        self.custom_action_types.insert(key.clone(), action_type);
        Ok(self.custom_action_types.get(&key).unwrap())
    }

    pub fn delete_custom_action_type(
        &mut self,
        category: &str,
        provider: &str,
        version: &str,
    ) -> Result<(), CodePipelineError> {
        let key = ActionTypeKey {
            category: category.to_string(),
            provider: provider.to_string(),
            version: version.to_string(),
        };
        self.custom_action_types
            .remove(&key)
            .ok_or(CodePipelineError::ActionTypeNotFound)?;
        Ok(())
    }

    pub fn get_custom_action_type(
        &self,
        category: &str,
        provider: &str,
        version: &str,
    ) -> Result<&CustomActionType, CodePipelineError> {
        let key = ActionTypeKey {
            category: category.to_string(),
            provider: provider.to_string(),
            version: version.to_string(),
        };
        self.custom_action_types
            .get(&key)
            .ok_or(CodePipelineError::ActionTypeNotFound)
    }

    pub fn list_custom_action_types(&self, owner_filter: Option<&str>) -> Vec<&CustomActionType> {
        self.custom_action_types
            .values()
            .filter(|_a| {
                // Custom action types always have owner "Custom"; if filter is specified
                // and is not "Custom", return empty.
                match owner_filter {
                    Some(f) => f == "Custom",
                    None => true,
                }
            })
            .collect()
    }

    pub fn update_action_type(
        &mut self,
        category: &str,
        provider: &str,
        version: &str,
        settings: Option<Value>,
        configuration_properties: Option<Value>,
        input_artifact_details: ArtifactDetailsData,
        output_artifact_details: ArtifactDetailsData,
    ) -> Result<(), CodePipelineError> {
        let key = ActionTypeKey {
            category: category.to_string(),
            provider: provider.to_string(),
            version: version.to_string(),
        };
        let entry = self
            .custom_action_types
            .get_mut(&key)
            .ok_or(CodePipelineError::ActionTypeNotFound)?;
        entry.settings = settings;
        entry.configuration_properties = configuration_properties;
        entry.input_artifact_details = input_artifact_details;
        entry.output_artifact_details = output_artifact_details;
        Ok(())
    }

    // ---- Webhooks ----

    pub fn put_webhook(
        &mut self,
        name: &str,
        definition: Value,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> &Webhook {
        let arn = format!("arn:aws:codepipeline:{region}:{account_id}:webhook:{name}");
        let url = format!("https://webhooks.codepipeline.{region}.amazonaws.com/{name}");
        let webhook = Webhook {
            name: name.to_string(),
            arn,
            url,
            definition,
            tags,
            registered_with_third_party: false,
            created: Utc::now(),
        };
        self.webhooks.insert(name.to_string(), webhook);
        self.webhooks.get(name).unwrap()
    }

    pub fn delete_webhook(&mut self, name: &str) -> Result<(), CodePipelineError> {
        self.webhooks
            .remove(name)
            .ok_or(CodePipelineError::WebhookNotFound {
                name: name.to_string(),
            })?;
        Ok(())
    }

    pub fn list_webhooks(&self) -> Vec<&Webhook> {
        self.webhooks.values().collect()
    }

    pub fn register_webhook_with_third_party(
        &mut self,
        name: &str,
    ) -> Result<(), CodePipelineError> {
        let wh = self
            .webhooks
            .get_mut(name)
            .ok_or(CodePipelineError::WebhookNotFound {
                name: name.to_string(),
            })?;
        wh.registered_with_third_party = true;
        Ok(())
    }

    pub fn deregister_webhook_with_third_party(
        &mut self,
        name: &str,
    ) -> Result<(), CodePipelineError> {
        let wh = self
            .webhooks
            .get_mut(name)
            .ok_or(CodePipelineError::WebhookNotFound {
                name: name.to_string(),
            })?;
        wh.registered_with_third_party = false;
        Ok(())
    }

    // ---- Stage Transitions ----

    pub fn disable_stage_transition(
        &mut self,
        pipeline_name: &str,
        stage_name: &str,
        transition_type: &str,
        reason: &str,
    ) -> Result<(), CodePipelineError> {
        let pipeline = self.pipelines.get_mut(pipeline_name).ok_or_else(|| {
            CodePipelineError::PipelineNotFound {
                name: pipeline_name.to_string(),
                account_id: DEFAULT_ACCOUNT_ID_STR.to_string(),
            }
        })?;
        pipeline.disabled_transitions.insert(
            (stage_name.to_string(), transition_type.to_string()),
            DisabledTransition {
                reason: reason.to_string(),
                last_changed_at: Utc::now(),
            },
        );
        Ok(())
    }

    pub fn enable_stage_transition(
        &mut self,
        pipeline_name: &str,
        stage_name: &str,
        transition_type: &str,
    ) -> Result<(), CodePipelineError> {
        let pipeline = self.pipelines.get_mut(pipeline_name).ok_or_else(|| {
            CodePipelineError::PipelineNotFound {
                name: pipeline_name.to_string(),
                account_id: DEFAULT_ACCOUNT_ID_STR.to_string(),
            }
        })?;
        pipeline
            .disabled_transitions
            .remove(&(stage_name.to_string(), transition_type.to_string()));
        Ok(())
    }

    // ---- Pipeline Execution ----

    pub fn start_pipeline_execution(
        &mut self,
        pipeline_name: &str,
        variables: Vec<Value>,
        source_revisions: Vec<Value>,
        execution_mode: Option<String>,
        execution_type: Option<String>,
    ) -> Result<String, CodePipelineError> {
        let pipeline = self.pipelines.get(pipeline_name).ok_or_else(|| {
            CodePipelineError::PipelineNotFound {
                name: pipeline_name.to_string(),
                account_id: DEFAULT_ACCOUNT_ID_STR.to_string(),
            }
        })?;
        let execution_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let execution = PipelineExecution {
            pipeline_execution_id: execution_id.clone(),
            pipeline_name: pipeline_name.to_string(),
            pipeline_version: pipeline.version,
            status: "InProgress".to_string(),
            status_summary: None,
            start_time: now,
            last_update_time: now,
            trigger: None,
            source_revisions,
            variables,
            execution_mode,
            execution_type,
        };
        self.pipeline_executions.push(execution);
        Ok(execution_id)
    }

    pub fn stop_pipeline_execution(
        &mut self,
        pipeline_name: &str,
        pipeline_execution_id: &str,
        abandon: bool,
        reason: Option<&str>,
    ) -> Result<String, CodePipelineError> {
        // Verify pipeline exists.
        if !self.pipelines.contains_key(pipeline_name) {
            return Err(CodePipelineError::PipelineNotFound {
                name: pipeline_name.to_string(),
                account_id: DEFAULT_ACCOUNT_ID_STR.to_string(),
            });
        }
        let exec = self
            .pipeline_executions
            .iter_mut()
            .find(|e| {
                e.pipeline_name == pipeline_name && e.pipeline_execution_id == pipeline_execution_id
            })
            .ok_or(CodePipelineError::PipelineExecutionNotFound)?;
        let new_status = if abandon { "Abandoned" } else { "Stopping" };
        exec.status = new_status.to_string();
        exec.status_summary = reason.map(String::from);
        exec.last_update_time = Utc::now();
        Ok(pipeline_execution_id.to_string())
    }

    pub fn get_pipeline_execution(
        &self,
        pipeline_name: &str,
        pipeline_execution_id: &str,
    ) -> Result<&PipelineExecution, CodePipelineError> {
        if !self.pipelines.contains_key(pipeline_name) {
            return Err(CodePipelineError::PipelineNotFound {
                name: pipeline_name.to_string(),
                account_id: DEFAULT_ACCOUNT_ID_STR.to_string(),
            });
        }
        self.pipeline_executions
            .iter()
            .find(|e| {
                e.pipeline_name == pipeline_name && e.pipeline_execution_id == pipeline_execution_id
            })
            .ok_or(CodePipelineError::PipelineExecutionNotFound)
    }

    pub fn list_pipeline_executions(
        &self,
        pipeline_name: &str,
    ) -> Result<Vec<&PipelineExecution>, CodePipelineError> {
        if !self.pipelines.contains_key(pipeline_name) {
            return Err(CodePipelineError::PipelineNotFound {
                name: pipeline_name.to_string(),
                account_id: DEFAULT_ACCOUNT_ID_STR.to_string(),
            });
        }
        Ok(self
            .pipeline_executions
            .iter()
            .filter(|e| e.pipeline_name == pipeline_name)
            .collect())
    }

    pub fn retry_stage_execution(
        &mut self,
        pipeline_name: &str,
        pipeline_execution_id: &str,
        _stage_name: &str,
        _retry_mode: &str,
    ) -> Result<String, CodePipelineError> {
        if !self.pipelines.contains_key(pipeline_name) {
            return Err(CodePipelineError::PipelineNotFound {
                name: pipeline_name.to_string(),
                account_id: DEFAULT_ACCOUNT_ID_STR.to_string(),
            });
        }
        let _exec = self
            .pipeline_executions
            .iter()
            .find(|e| {
                e.pipeline_name == pipeline_name && e.pipeline_execution_id == pipeline_execution_id
            })
            .ok_or(CodePipelineError::PipelineExecutionNotFound)?;
        // In a real engine this would restart the stage; we just return the execution ID.
        Ok(pipeline_execution_id.to_string())
    }

    pub fn rollback_stage(
        &mut self,
        pipeline_name: &str,
        _stage_name: &str,
        target_pipeline_execution_id: &str,
    ) -> Result<String, CodePipelineError> {
        if !self.pipelines.contains_key(pipeline_name) {
            return Err(CodePipelineError::PipelineNotFound {
                name: pipeline_name.to_string(),
                account_id: DEFAULT_ACCOUNT_ID_STR.to_string(),
            });
        }
        let _exec = self
            .pipeline_executions
            .iter()
            .find(|e| {
                e.pipeline_name == pipeline_name
                    && e.pipeline_execution_id == target_pipeline_execution_id
            })
            .ok_or(CodePipelineError::PipelineExecutionNotFound)?;
        // Generate a new execution ID for the rollback.
        let new_execution_id = Uuid::new_v4().to_string();
        Ok(new_execution_id)
    }

    pub fn override_stage_condition(
        &mut self,
        pipeline_name: &str,
        pipeline_execution_id: &str,
        _stage_name: &str,
        _condition_type: &str,
    ) -> Result<(), CodePipelineError> {
        if !self.pipelines.contains_key(pipeline_name) {
            return Err(CodePipelineError::PipelineNotFound {
                name: pipeline_name.to_string(),
                account_id: DEFAULT_ACCOUNT_ID_STR.to_string(),
            });
        }
        let _exec = self
            .pipeline_executions
            .iter()
            .find(|e| {
                e.pipeline_name == pipeline_name && e.pipeline_execution_id == pipeline_execution_id
            })
            .ok_or(CodePipelineError::PipelineExecutionNotFound)?;
        Ok(())
    }

    // ---- Jobs ----

    pub fn acknowledge_job(
        &mut self,
        job_id: &str,
        nonce: &str,
    ) -> Result<&str, CodePipelineError> {
        let job = self
            .jobs
            .get_mut(job_id)
            .ok_or(CodePipelineError::JobNotFound {
                job_id: job_id.to_string(),
            })?;
        if job.nonce != nonce {
            return Err(CodePipelineError::InvalidNonce);
        }
        job.status = JobStatus::InProgress;
        Ok(match job.status {
            JobStatus::InProgress => "InProgress",
            _ => "InProgress",
        })
    }

    pub fn get_job_details(&self, job_id: &str) -> Result<&ActionJob, CodePipelineError> {
        self.jobs.get(job_id).ok_or(CodePipelineError::JobNotFound {
            job_id: job_id.to_string(),
        })
    }

    pub fn poll_for_jobs(&self, _action_type_id: &Value) -> Vec<&ActionJob> {
        // Return all jobs in Created status.
        self.jobs
            .values()
            .filter(|j| j.status == JobStatus::Created)
            .collect()
    }

    pub fn put_job_success_result(&mut self, job_id: &str) -> Result<(), CodePipelineError> {
        let job = self
            .jobs
            .get_mut(job_id)
            .ok_or(CodePipelineError::JobNotFound {
                job_id: job_id.to_string(),
            })?;
        job.status = JobStatus::Succeeded;
        Ok(())
    }

    pub fn put_job_failure_result(&mut self, job_id: &str) -> Result<(), CodePipelineError> {
        let job = self
            .jobs
            .get_mut(job_id)
            .ok_or(CodePipelineError::JobNotFound {
                job_id: job_id.to_string(),
            })?;
        job.status = JobStatus::Failed;
        Ok(())
    }

    pub fn put_action_revision(
        &mut self,
        pipeline_name: &str,
        _stage_name: &str,
        _action_name: &str,
    ) -> Result<Option<String>, CodePipelineError> {
        if !self.pipelines.contains_key(pipeline_name) {
            return Err(CodePipelineError::PipelineNotFound {
                name: pipeline_name.to_string(),
                account_id: DEFAULT_ACCOUNT_ID_STR.to_string(),
            });
        }
        // In a real engine this triggers a new execution; we return a generated
        // execution ID to signal a new revision was accepted.
        let execution_id = Uuid::new_v4().to_string();
        Ok(Some(execution_id))
    }

    pub fn put_approval_result(
        &mut self,
        pipeline_name: &str,
        _stage_name: &str,
        _action_name: &str,
    ) -> Result<(), CodePipelineError> {
        if !self.pipelines.contains_key(pipeline_name) {
            return Err(CodePipelineError::PipelineNotFound {
                name: pipeline_name.to_string(),
                account_id: DEFAULT_ACCOUNT_ID_STR.to_string(),
            });
        }
        Ok(())
    }
}

// For use in state.rs itself
pub(crate) const DEFAULT_ACCOUNT_ID_STR: &str = "123456789012";
