use std::collections::HashMap;

use crate::types::*;

#[derive(Debug, Default)]
pub struct BedrockState {
    pub customization_jobs: HashMap<String, ModelCustomizationJob>,
    pub custom_models: HashMap<String, CustomModel>,
    pub logging_config: Option<LoggingConfiguration>,
    pub tags: HashMap<String, Vec<ResourceTag>>,
    pub guardrails: HashMap<String, Guardrail>,
    pub provisioned_models: HashMap<String, ProvisionedModelThroughput>,
    pub inference_profiles: HashMap<String, InferenceProfile>,
    pub prompt_routers: HashMap<String, PromptRouter>,
    pub evaluation_jobs: HashMap<String, EvaluationJob>,
    pub model_invocation_jobs: HashMap<String, ModelInvocationJob>,
    pub model_import_jobs: HashMap<String, ModelImportJob>,
    pub model_copy_jobs: HashMap<String, ModelCopyJob>,
}

#[derive(Debug, thiserror::Error)]
pub enum BedrockError {
    // ResourceNotFoundException (404)
    #[error("Could not find model {0}")]
    ModelNotFound(String),
    #[error("Could not find job {0}")]
    CustomizationJobNotFound(String),
    #[error("Could not find custom model {0}")]
    CustomModelNotFound(String),
    #[error("Could not find guardrail {0}")]
    GuardrailNotFound(String),
    #[error("Could not find provisioned model {0}")]
    ProvisionedModelNotFound(String),
    #[error("Could not find inference profile {0}")]
    InferenceProfileNotFound(String),
    #[error("Could not find prompt router {0}")]
    PromptRouterNotFound(String),
    #[error("Could not find evaluation job {0}")]
    EvaluationJobNotFound(String),
    #[error("Could not find model invocation job {0}")]
    ModelInvocationJobNotFound(String),
    #[error("Could not find model import job {0}")]
    ModelImportJobNotFound(String),
    #[error("Could not find model copy job {0}")]
    ModelCopyJobNotFound(String),
    // ConflictException (400)
    #[error("Job with name {0} already exists")]
    CustomizationJobConflict(String),
    #[error("Job {} is not in progress", .0)]
    CustomizationJobNotInProgress(String),
    #[error("Guardrail with name {0} already exists")]
    GuardrailConflict(String),
    #[error("Provisioned model {0} already exists")]
    ProvisionedModelConflict(String),
    #[error("Inference profile {0} already exists")]
    InferenceProfileConflict(String),
    #[error("Prompt router {0} already exists")]
    PromptRouterConflict(String),
    #[error("Evaluation job {0} already exists")]
    EvaluationJobConflict(String),
    #[error("Model invocation job {0} already exists")]
    ModelInvocationJobConflict(String),
}

/// Pre-populated foundation models returned by the mock.
pub fn default_foundation_models(_account_id: &str, region: &str) -> Vec<FoundationModel> {
    vec![
        FoundationModel {
            model_id: "anthropic.claude-v2".to_string(),
            model_name: "Claude V2".to_string(),
            provider_name: "Anthropic".to_string(),
            model_arn: format!("arn:aws:bedrock:{region}::foundation-model/anthropic.claude-v2"),
            input_modalities: vec!["TEXT".to_string()],
            output_modalities: vec!["TEXT".to_string()],
            response_streaming_supported: true,
            customizations_supported: vec!["FINE_TUNING".to_string()],
            inference_types_supported: vec!["ON_DEMAND".to_string()],
            model_lifecycle_status: "ACTIVE".to_string(),
        },
        FoundationModel {
            model_id: "amazon.titan-text-express-v1".to_string(),
            model_name: "Titan Text G1 - Express".to_string(),
            provider_name: "Amazon".to_string(),
            model_arn: format!(
                "arn:aws:bedrock:{region}::foundation-model/amazon.titan-text-express-v1"
            ),
            input_modalities: vec!["TEXT".to_string()],
            output_modalities: vec!["TEXT".to_string()],
            response_streaming_supported: true,
            customizations_supported: vec!["FINE_TUNING".to_string()],
            inference_types_supported: vec!["ON_DEMAND".to_string(), "PROVISIONED".to_string()],
            model_lifecycle_status: "ACTIVE".to_string(),
        },
        FoundationModel {
            model_id: "anthropic.claude-3-sonnet-20240229-v1:0".to_string(),
            model_name: "Claude 3 Sonnet".to_string(),
            provider_name: "Anthropic".to_string(),
            model_arn: format!(
                "arn:aws:bedrock:{region}::foundation-model/anthropic.claude-3-sonnet-20240229-v1:0"
            ),
            input_modalities: vec!["TEXT".to_string(), "IMAGE".to_string()],
            output_modalities: vec!["TEXT".to_string()],
            response_streaming_supported: true,
            customizations_supported: vec![],
            inference_types_supported: vec!["ON_DEMAND".to_string()],
            model_lifecycle_status: "ACTIVE".to_string(),
        },
    ]
}

impl BedrockState {
    pub fn get_foundation_model(
        &self,
        model_id: &str,
        _account_id: &str,
        region: &str,
    ) -> Result<FoundationModel, BedrockError> {
        let models = default_foundation_models(_account_id, region);
        models
            .into_iter()
            .find(|m| m.model_id == model_id)
            .ok_or_else(|| BedrockError::ModelNotFound(model_id.to_string()))
    }

    pub fn list_foundation_models(&self, _account_id: &str, region: &str) -> Vec<FoundationModel> {
        default_foundation_models(_account_id, region)
    }

    pub fn create_model_customization_job(
        &mut self,
        job_name: &str,
        base_model_identifier: &str,
        custom_model_name: &str,
        customization_type: &str,
        role_arn: &str,
        training_data_config: TrainingDataConfig,
        output_data_config: OutputDataConfig,
        hyper_parameters: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&ModelCustomizationJob, BedrockError> {
        // Check for duplicate job name
        if self
            .customization_jobs
            .values()
            .any(|j| j.job_name == job_name)
        {
            return Err(BedrockError::CustomizationJobConflict(job_name.to_string()));
        }

        let job_arn =
            format!("arn:aws:bedrock:{region}:{account_id}:model-customization-job/{job_name}");
        let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

        // Also create the custom model (simulating immediate completion)
        let custom_model_arn =
            format!("arn:aws:bedrock:{region}:{account_id}:custom-model/{custom_model_name}");
        let custom_model = CustomModel {
            model_arn: custom_model_arn.clone(),
            model_name: custom_model_name.to_string(),
            base_model_arn: base_model_identifier.to_string(),
            customization_type: customization_type.to_string(),
            creation_time: now.clone(),
            job_arn: job_arn.clone(),
            job_name: job_name.to_string(),
            training_data_config: TrainingDataConfig {
                s3_uri: training_data_config.s3_uri.clone(),
            },
            output_data_config: OutputDataConfig {
                s3_uri: output_data_config.s3_uri.clone(),
            },
            hyper_parameters: hyper_parameters.clone(),
        };
        self.custom_models.insert(custom_model_arn, custom_model);

        let job = ModelCustomizationJob {
            job_arn: job_arn.clone(),
            job_name: job_name.to_string(),
            base_model_identifier: base_model_identifier.to_string(),
            custom_model_name: custom_model_name.to_string(),
            customization_type: customization_type.to_string(),
            role_arn: role_arn.to_string(),
            status: "InProgress".to_string(),
            creation_time: now.clone(),
            last_modified_time: now,
            training_data_config,
            output_data_config,
            hyper_parameters,
        };

        self.customization_jobs.insert(job_arn.clone(), job);
        Ok(self.customization_jobs.get(&job_arn).unwrap())
    }

    pub fn list_model_customization_jobs(&self) -> Vec<&ModelCustomizationJob> {
        self.customization_jobs.values().collect()
    }

    pub fn get_model_customization_job(
        &self,
        job_identifier: &str,
    ) -> Result<&ModelCustomizationJob, BedrockError> {
        // Look up by ARN first, then by name
        if let Some(job) = self.customization_jobs.get(job_identifier) {
            return Ok(job);
        }
        self.customization_jobs
            .values()
            .find(|j| j.job_name == job_identifier)
            .ok_or_else(|| BedrockError::CustomizationJobNotFound(job_identifier.to_string()))
    }

    pub fn stop_model_customization_job(
        &mut self,
        job_identifier: &str,
    ) -> Result<(), BedrockError> {
        // Find the job ARN
        let job_arn = if self.customization_jobs.contains_key(job_identifier) {
            job_identifier.to_string()
        } else {
            self.customization_jobs
                .values()
                .find(|j| j.job_name == job_identifier)
                .map(|j| j.job_arn.clone())
                .ok_or_else(|| BedrockError::CustomizationJobNotFound(job_identifier.to_string()))?
        };

        let job = self.customization_jobs.get_mut(&job_arn).unwrap();
        if job.status != "InProgress" {
            return Err(BedrockError::CustomizationJobNotInProgress(
                job_identifier.to_string(),
            ));
        }
        let now = chrono::Utc::now().to_rfc3339();
        job.status = "Stopped".to_string();
        job.last_modified_time = now;
        Ok(())
    }

    pub fn get_custom_model(&self, model_identifier: &str) -> Result<&CustomModel, BedrockError> {
        if let Some(model) = self.custom_models.get(model_identifier) {
            return Ok(model);
        }
        self.custom_models
            .values()
            .find(|m| m.model_name == model_identifier)
            .ok_or_else(|| BedrockError::CustomModelNotFound(model_identifier.to_string()))
    }

    pub fn list_custom_models(&self) -> Vec<&CustomModel> {
        self.custom_models.values().collect()
    }

    pub fn delete_custom_model(&mut self, model_identifier: &str) -> Result<(), BedrockError> {
        // Try by ARN first
        if self.custom_models.remove(model_identifier).is_some() {
            return Ok(());
        }
        // Try by name
        let arn = self
            .custom_models
            .values()
            .find(|m| m.model_name == model_identifier)
            .map(|m| m.model_arn.clone());
        match arn {
            Some(arn) => {
                self.custom_models.remove(&arn);
                Ok(())
            }
            None => Err(BedrockError::CustomModelNotFound(
                model_identifier.to_string(),
            )),
        }
    }

    pub fn get_logging_config(&self) -> Option<&LoggingConfiguration> {
        self.logging_config.as_ref()
    }

    pub fn put_logging_config(&mut self, config: LoggingConfiguration) {
        self.logging_config = Some(config);
    }

    pub fn delete_logging_config(&mut self) {
        self.logging_config = None;
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<Vec<ResourceTag>, BedrockError> {
        Ok(self.tags.get(resource_arn).cloned().unwrap_or_default())
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: Vec<ResourceTag>,
    ) -> Result<(), BedrockError> {
        let entry = self.tags.entry(resource_arn.to_string()).or_default();
        for tag in tags {
            if let Some(existing) = entry.iter_mut().find(|t| t.key == tag.key) {
                existing.value = tag.value;
            } else {
                entry.push(tag);
            }
        }
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: Vec<String>,
    ) -> Result<(), BedrockError> {
        if let Some(tags) = self.tags.get_mut(resource_arn) {
            tags.retain(|t| !tag_keys.contains(&t.key));
        }
        Ok(())
    }

    // ---- Guardrails ----

    pub fn create_guardrail(
        &mut self,
        name: &str,
        description: Option<String>,
        blocked_input_messaging: &str,
        blocked_outputs_messaging: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&Guardrail, BedrockError> {
        if self.guardrails.values().any(|g| g.name == name) {
            return Err(BedrockError::GuardrailConflict(name.to_string()));
        }
        let id = uuid::Uuid::new_v4().to_string().replace('-', "")[..10].to_string();
        let arn = format!("arn:aws:bedrock:{region}:{account_id}:guardrail/{id}");
        let now = chrono::Utc::now().to_rfc3339();
        let g = Guardrail {
            guardrail_id: id.clone(),
            guardrail_arn: arn.clone(),
            name: name.to_string(),
            description,
            status: "READY".to_string(),
            version: "DRAFT".to_string(),
            created_at: now.clone(),
            updated_at: now,
            blocked_input_messaging: blocked_input_messaging.to_string(),
            blocked_outputs_messaging: blocked_outputs_messaging.to_string(),
            content_policy_config: None,
            contextual_grounding_policy_config: None,
            sensitive_information_policy_config: None,
            topic_policy_config: None,
            word_policy_config: None,
        };
        self.guardrails.insert(arn.clone(), g);
        Ok(self.guardrails.get(&arn).unwrap())
    }

    pub fn get_guardrail(&self, identifier: &str) -> Result<&Guardrail, BedrockError> {
        if let Some(g) = self.guardrails.get(identifier) {
            return Ok(g);
        }
        self.guardrails
            .values()
            .find(|g| g.guardrail_id == identifier || g.name == identifier)
            .ok_or_else(|| BedrockError::GuardrailNotFound(identifier.to_string()))
    }

    pub fn list_guardrails(&self) -> Vec<&Guardrail> {
        self.guardrails.values().collect()
    }

    pub fn update_guardrail(
        &mut self,
        identifier: &str,
        name: Option<&str>,
        description: Option<String>,
        blocked_input_messaging: Option<&str>,
        blocked_outputs_messaging: Option<&str>,
    ) -> Result<String, BedrockError> {
        let arn = if self.guardrails.contains_key(identifier) {
            identifier.to_string()
        } else {
            self.guardrails
                .values()
                .find(|g| g.guardrail_id == identifier || g.name == identifier)
                .map(|g| g.guardrail_arn.clone())
                .ok_or_else(|| BedrockError::GuardrailNotFound(identifier.to_string()))?
        };
        let g = self.guardrails.get_mut(&arn).unwrap();
        if let Some(n) = name {
            g.name = n.to_string();
        }
        if let Some(d) = description {
            g.description = Some(d);
        }
        if let Some(m) = blocked_input_messaging {
            g.blocked_input_messaging = m.to_string();
        }
        if let Some(m) = blocked_outputs_messaging {
            g.blocked_outputs_messaging = m.to_string();
        }
        g.updated_at = chrono::Utc::now().to_rfc3339();
        Ok(g.guardrail_id.clone())
    }

    pub fn delete_guardrail(&mut self, identifier: &str) -> Result<(), BedrockError> {
        if self.guardrails.remove(identifier).is_some() {
            return Ok(());
        }
        let arn = self
            .guardrails
            .values()
            .find(|g| g.guardrail_id == identifier || g.name == identifier)
            .map(|g| g.guardrail_arn.clone());
        match arn {
            Some(a) => {
                self.guardrails.remove(&a);
                Ok(())
            }
            None => Err(BedrockError::GuardrailNotFound(identifier.to_string())),
        }
    }

    pub fn create_guardrail_version(&mut self, identifier: &str) -> Result<String, BedrockError> {
        let arn = if self.guardrails.contains_key(identifier) {
            identifier.to_string()
        } else {
            self.guardrails
                .values()
                .find(|g| g.guardrail_id == identifier || g.name == identifier)
                .map(|g| g.guardrail_arn.clone())
                .ok_or_else(|| BedrockError::GuardrailNotFound(identifier.to_string()))?
        };
        let g = self.guardrails.get(&arn).unwrap();
        Ok(g.guardrail_id.clone())
    }

    // ---- Provisioned Model Throughput ----

    pub fn create_provisioned_model_throughput(
        &mut self,
        model_id: &str,
        provisioned_model_name: &str,
        model_units: i32,
        commitment_duration: Option<String>,
        account_id: &str,
        region: &str,
    ) -> Result<&ProvisionedModelThroughput, BedrockError> {
        if self
            .provisioned_models
            .values()
            .any(|p| p.provisioned_model_name == provisioned_model_name)
        {
            return Err(BedrockError::ProvisionedModelConflict(
                provisioned_model_name.to_string(),
            ));
        }
        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:bedrock:{region}:{account_id}:provisioned-model/{id}");
        let now = chrono::Utc::now().to_rfc3339();
        let p = ProvisionedModelThroughput {
            provisioned_model_arn: arn.clone(),
            provisioned_model_name: provisioned_model_name.to_string(),
            model_arn: model_id.to_string(),
            model_units,
            status: "InService".to_string(),
            commitment_duration,
            creation_time: now.clone(),
            last_modified_time: now,
        };
        self.provisioned_models.insert(arn.clone(), p);
        Ok(self.provisioned_models.get(&arn).unwrap())
    }

    pub fn get_provisioned_model_throughput(
        &self,
        identifier: &str,
    ) -> Result<&ProvisionedModelThroughput, BedrockError> {
        if let Some(p) = self.provisioned_models.get(identifier) {
            return Ok(p);
        }
        self.provisioned_models
            .values()
            .find(|p| p.provisioned_model_name == identifier)
            .ok_or_else(|| BedrockError::ProvisionedModelNotFound(identifier.to_string()))
    }

    pub fn list_provisioned_model_throughputs(&self) -> Vec<&ProvisionedModelThroughput> {
        self.provisioned_models.values().collect()
    }

    pub fn update_provisioned_model_throughput(
        &mut self,
        identifier: &str,
        desired_model_id: Option<&str>,
        desired_provisioned_model_name: Option<&str>,
    ) -> Result<(), BedrockError> {
        let arn = if self.provisioned_models.contains_key(identifier) {
            identifier.to_string()
        } else {
            self.provisioned_models
                .values()
                .find(|p| p.provisioned_model_name == identifier)
                .map(|p| p.provisioned_model_arn.clone())
                .ok_or_else(|| BedrockError::ProvisionedModelNotFound(identifier.to_string()))?
        };
        let p = self.provisioned_models.get_mut(&arn).unwrap();
        if let Some(m) = desired_model_id {
            p.model_arn = m.to_string();
        }
        if let Some(n) = desired_provisioned_model_name {
            p.provisioned_model_name = n.to_string();
        }
        p.last_modified_time = chrono::Utc::now().to_rfc3339();
        Ok(())
    }

    pub fn delete_provisioned_model_throughput(
        &mut self,
        identifier: &str,
    ) -> Result<(), BedrockError> {
        if self.provisioned_models.remove(identifier).is_some() {
            return Ok(());
        }
        let arn = self
            .provisioned_models
            .values()
            .find(|p| p.provisioned_model_name == identifier)
            .map(|p| p.provisioned_model_arn.clone());
        match arn {
            Some(a) => {
                self.provisioned_models.remove(&a);
                Ok(())
            }
            None => Err(BedrockError::ProvisionedModelNotFound(
                identifier.to_string(),
            )),
        }
    }

    // ---- Inference Profiles ----

    pub fn create_inference_profile(
        &mut self,
        name: &str,
        description: Option<String>,
        model_source_arn: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&InferenceProfile, BedrockError> {
        if self
            .inference_profiles
            .values()
            .any(|p| p.inference_profile_name == name)
        {
            return Err(BedrockError::InferenceProfileConflict(name.to_string()));
        }
        let id = format!("{region}.{name}");
        let arn =
            format!("arn:aws:bedrock:{region}:{account_id}:application-inference-profile/{id}");
        let now = chrono::Utc::now().to_rfc3339();
        let p = InferenceProfile {
            inference_profile_arn: arn.clone(),
            inference_profile_id: id,
            inference_profile_name: name.to_string(),
            description,
            status: "ACTIVE".to_string(),
            profile_type: "APPLICATION".to_string(),
            model_arn: model_source_arn.to_string(),
            created_at: now.clone(),
            updated_at: now,
        };
        self.inference_profiles.insert(arn.clone(), p);
        Ok(self.inference_profiles.get(&arn).unwrap())
    }

    pub fn get_inference_profile(
        &self,
        identifier: &str,
    ) -> Result<&InferenceProfile, BedrockError> {
        if let Some(p) = self.inference_profiles.get(identifier) {
            return Ok(p);
        }
        self.inference_profiles
            .values()
            .find(|p| {
                p.inference_profile_id == identifier || p.inference_profile_name == identifier
            })
            .ok_or_else(|| BedrockError::InferenceProfileNotFound(identifier.to_string()))
    }

    pub fn list_inference_profiles(&self) -> Vec<&InferenceProfile> {
        self.inference_profiles.values().collect()
    }

    pub fn delete_inference_profile(&mut self, identifier: &str) -> Result<(), BedrockError> {
        if self.inference_profiles.remove(identifier).is_some() {
            return Ok(());
        }
        let arn = self
            .inference_profiles
            .values()
            .find(|p| {
                p.inference_profile_id == identifier || p.inference_profile_name == identifier
            })
            .map(|p| p.inference_profile_arn.clone());
        match arn {
            Some(a) => {
                self.inference_profiles.remove(&a);
                Ok(())
            }
            None => Err(BedrockError::InferenceProfileNotFound(
                identifier.to_string(),
            )),
        }
    }

    // ---- Prompt Routers ----

    pub fn create_prompt_router(
        &mut self,
        name: &str,
        description: Option<String>,
        fallback_model_arn: &str,
        models: Vec<String>,
        routing_criteria_response_quality_difference: f64,
        account_id: &str,
        region: &str,
    ) -> Result<&PromptRouter, BedrockError> {
        if self
            .prompt_routers
            .values()
            .any(|r| r.prompt_router_name == name)
        {
            return Err(BedrockError::PromptRouterConflict(name.to_string()));
        }
        let arn = format!("arn:aws:bedrock:{region}:{account_id}:prompt-router/{name}");
        let now = chrono::Utc::now().to_rfc3339();
        let r = PromptRouter {
            prompt_router_arn: arn.clone(),
            prompt_router_name: name.to_string(),
            description,
            status: "AVAILABLE".to_string(),
            router_type: "custom".to_string(),
            fallback_model_arn: fallback_model_arn.to_string(),
            models,
            routing_criteria_response_quality_difference,
            created_at: now.clone(),
            updated_at: now,
        };
        self.prompt_routers.insert(arn.clone(), r);
        Ok(self.prompt_routers.get(&arn).unwrap())
    }

    pub fn get_prompt_router(&self, identifier: &str) -> Result<&PromptRouter, BedrockError> {
        if let Some(r) = self.prompt_routers.get(identifier) {
            return Ok(r);
        }
        self.prompt_routers
            .values()
            .find(|r| r.prompt_router_name == identifier)
            .ok_or_else(|| BedrockError::PromptRouterNotFound(identifier.to_string()))
    }

    pub fn list_prompt_routers(&self) -> Vec<&PromptRouter> {
        self.prompt_routers.values().collect()
    }

    pub fn delete_prompt_router(&mut self, identifier: &str) -> Result<(), BedrockError> {
        if self.prompt_routers.remove(identifier).is_some() {
            return Ok(());
        }
        let arn = self
            .prompt_routers
            .values()
            .find(|r| r.prompt_router_name == identifier)
            .map(|r| r.prompt_router_arn.clone());
        match arn {
            Some(a) => {
                self.prompt_routers.remove(&a);
                Ok(())
            }
            None => Err(BedrockError::PromptRouterNotFound(identifier.to_string())),
        }
    }

    // ---- Evaluation Jobs ----

    pub fn create_evaluation_job(
        &mut self,
        job_name: &str,
        job_description: Option<String>,
        account_id: &str,
        region: &str,
    ) -> Result<&EvaluationJob, BedrockError> {
        if self
            .evaluation_jobs
            .values()
            .any(|j| j.job_name == job_name)
        {
            return Err(BedrockError::EvaluationJobConflict(job_name.to_string()));
        }
        let arn = format!("arn:aws:bedrock:{region}:{account_id}:evaluation-job/{job_name}");
        let now = chrono::Utc::now().to_rfc3339();
        let j = EvaluationJob {
            job_arn: arn.clone(),
            job_name: job_name.to_string(),
            job_description,
            status: "InProgress".to_string(),
            creation_time: now.clone(),
            last_modified_time: now,
        };
        self.evaluation_jobs.insert(arn.clone(), j);
        Ok(self.evaluation_jobs.get(&arn).unwrap())
    }

    pub fn get_evaluation_job(&self, identifier: &str) -> Result<&EvaluationJob, BedrockError> {
        if let Some(j) = self.evaluation_jobs.get(identifier) {
            return Ok(j);
        }
        self.evaluation_jobs
            .values()
            .find(|j| j.job_name == identifier)
            .ok_or_else(|| BedrockError::EvaluationJobNotFound(identifier.to_string()))
    }

    pub fn list_evaluation_jobs(&self) -> Vec<&EvaluationJob> {
        self.evaluation_jobs.values().collect()
    }

    pub fn stop_evaluation_job(&mut self, identifier: &str) -> Result<(), BedrockError> {
        let arn = if self.evaluation_jobs.contains_key(identifier) {
            identifier.to_string()
        } else {
            self.evaluation_jobs
                .values()
                .find(|j| j.job_name == identifier)
                .map(|j| j.job_arn.clone())
                .ok_or_else(|| BedrockError::EvaluationJobNotFound(identifier.to_string()))?
        };
        let j = self.evaluation_jobs.get_mut(&arn).unwrap();
        j.status = "Stopped".to_string();
        j.last_modified_time = chrono::Utc::now().to_rfc3339();
        Ok(())
    }

    // ---- Model Invocation Jobs ----

    pub fn create_model_invocation_job(
        &mut self,
        job_name: &str,
        model_id: &str,
        role_arn: &str,
        input_s3_uri: &str,
        output_s3_uri: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&ModelInvocationJob, BedrockError> {
        if self
            .model_invocation_jobs
            .values()
            .any(|j| j.job_name == job_name)
        {
            return Err(BedrockError::ModelInvocationJobConflict(
                job_name.to_string(),
            ));
        }
        let arn = format!("arn:aws:bedrock:{region}:{account_id}:model-invocation-job/{job_name}");
        let now = chrono::Utc::now().to_rfc3339();
        let j = ModelInvocationJob {
            job_arn: arn.clone(),
            job_name: job_name.to_string(),
            model_id: model_id.to_string(),
            role_arn: role_arn.to_string(),
            input_s3_uri: input_s3_uri.to_string(),
            output_s3_uri: output_s3_uri.to_string(),
            status: "Submitted".to_string(),
            submit_time: now.clone(),
            last_modified_time: now,
        };
        self.model_invocation_jobs.insert(arn.clone(), j);
        Ok(self.model_invocation_jobs.get(&arn).unwrap())
    }

    pub fn get_model_invocation_job(
        &self,
        identifier: &str,
    ) -> Result<&ModelInvocationJob, BedrockError> {
        if let Some(j) = self.model_invocation_jobs.get(identifier) {
            return Ok(j);
        }
        self.model_invocation_jobs
            .values()
            .find(|j| j.job_name == identifier)
            .ok_or_else(|| BedrockError::ModelInvocationJobNotFound(identifier.to_string()))
    }

    pub fn list_model_invocation_jobs(&self) -> Vec<&ModelInvocationJob> {
        self.model_invocation_jobs.values().collect()
    }

    pub fn stop_model_invocation_job(&mut self, identifier: &str) -> Result<(), BedrockError> {
        let arn = if self.model_invocation_jobs.contains_key(identifier) {
            identifier.to_string()
        } else {
            self.model_invocation_jobs
                .values()
                .find(|j| j.job_name == identifier)
                .map(|j| j.job_arn.clone())
                .ok_or_else(|| BedrockError::ModelInvocationJobNotFound(identifier.to_string()))?
        };
        let j = self.model_invocation_jobs.get_mut(&arn).unwrap();
        j.status = "Stopping".to_string();
        j.last_modified_time = chrono::Utc::now().to_rfc3339();
        Ok(())
    }

    // ---- Model Import Jobs ----

    pub fn create_model_import_job(
        &mut self,
        job_name: &str,
        imported_model_name: &str,
        role_arn: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&ModelImportJob, BedrockError> {
        let arn = format!("arn:aws:bedrock:{region}:{account_id}:model-import-job/{job_name}");
        let model_arn =
            format!("arn:aws:bedrock:{region}:{account_id}:imported-model/{imported_model_name}");
        let now = chrono::Utc::now().to_rfc3339();
        let j = ModelImportJob {
            job_arn: arn.clone(),
            job_name: job_name.to_string(),
            imported_model_name: imported_model_name.to_string(),
            imported_model_arn: model_arn,
            role_arn: role_arn.to_string(),
            status: "InProgress".to_string(),
            creation_time: now.clone(),
            last_modified_time: now,
        };
        self.model_import_jobs.insert(arn.clone(), j);
        Ok(self.model_import_jobs.get(&arn).unwrap())
    }

    pub fn get_model_import_job(&self, identifier: &str) -> Result<&ModelImportJob, BedrockError> {
        if let Some(j) = self.model_import_jobs.get(identifier) {
            return Ok(j);
        }
        self.model_import_jobs
            .values()
            .find(|j| j.job_name == identifier)
            .ok_or_else(|| BedrockError::ModelImportJobNotFound(identifier.to_string()))
    }

    pub fn list_model_import_jobs(&self) -> Vec<&ModelImportJob> {
        self.model_import_jobs.values().collect()
    }

    // ---- Model Copy Jobs ----

    pub fn create_model_copy_job(
        &mut self,
        source_model_arn: &str,
        target_model_name: &str,
        source_account_id: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&ModelCopyJob, BedrockError> {
        let arn =
            format!("arn:aws:bedrock:{region}:{account_id}:model-copy-job/{target_model_name}");
        let target_model_arn =
            format!("arn:aws:bedrock:{region}:{account_id}:custom-model/{target_model_name}");
        let now = chrono::Utc::now().to_rfc3339();
        let j = ModelCopyJob {
            job_arn: arn.clone(),
            source_model_arn: source_model_arn.to_string(),
            source_account_id: source_account_id.to_string(),
            target_model_arn,
            target_model_name: target_model_name.to_string(),
            status: "InProgress".to_string(),
            creation_time: now,
        };
        self.model_copy_jobs.insert(arn.clone(), j);
        Ok(self.model_copy_jobs.get(&arn).unwrap())
    }

    pub fn get_model_copy_job(&self, identifier: &str) -> Result<&ModelCopyJob, BedrockError> {
        if let Some(j) = self.model_copy_jobs.get(identifier) {
            return Ok(j);
        }
        self.model_copy_jobs
            .values()
            .find(|j| j.job_arn == identifier)
            .ok_or_else(|| BedrockError::ModelCopyJobNotFound(identifier.to_string()))
    }

    pub fn list_model_copy_jobs(&self) -> Vec<&ModelCopyJob> {
        self.model_copy_jobs.values().collect()
    }
}
