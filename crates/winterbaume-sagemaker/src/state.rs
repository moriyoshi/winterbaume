use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

/// Error type for SageMaker operations.
#[derive(Debug, thiserror::Error)]
pub enum SageMakerError {
    #[error("Could not find {resource_type} '{name}'.")]
    NotFound { resource_type: String, name: String },
    #[error("Cannot create a duplicate {resource_type} ({name})")]
    AlreadyExists { resource_type: String, name: String },
    #[error("{resource_type} '{name}' does not exist.")]
    ResourceNotFound { resource_type: String, name: String },
    #[error("Model Package Group does not exist.")]
    ModelPackageGroupNotFound,
}

fn not_found(resource_type: &str, name: &str) -> SageMakerError {
    SageMakerError::NotFound {
        resource_type: resource_type.to_string(),
        name: name.to_string(),
    }
}

fn already_exists(resource_type: &str, name: &str) -> SageMakerError {
    SageMakerError::AlreadyExists {
        resource_type: resource_type.to_string(),
        name: name.to_string(),
    }
}

fn resource_not_found(resource_type: &str, name: &str) -> SageMakerError {
    SageMakerError::ResourceNotFound {
        resource_type: resource_type.to_string(),
        name: name.to_string(),
    }
}

/// In-memory state for a single (account, region) pair.
#[derive(Default)]
pub struct SageMakerState {
    pub notebook_instances: HashMap<String, NotebookInstance>,
    pub models: HashMap<String, Model>,
    pub endpoint_configs: HashMap<String, EndpointConfig>,
    pub endpoints: HashMap<String, Endpoint>,
    pub training_jobs: HashMap<String, TrainingJob>,
    pub processing_jobs: HashMap<String, ProcessingJob>,
    pub transform_jobs: HashMap<String, TransformJob>,
    pub hyper_parameter_tuning_jobs: HashMap<String, HyperParameterTuningJob>,
    pub compilation_jobs: HashMap<String, CompilationJob>,
    pub auto_ml_jobs_v2: HashMap<String, AutoMLJobV2>,
    pub experiments: HashMap<String, Experiment>,
    pub trials: HashMap<String, Trial>,
    pub trial_components: HashMap<String, TrialComponent>,
    pub pipelines: HashMap<String, Pipeline>,
    pub feature_groups: HashMap<String, FeatureGroup>,
    pub domains: HashMap<String, Domain>,
    pub clusters: HashMap<String, Cluster>,
    pub cluster_nodes: HashMap<String, Vec<ClusterNode>>,
    pub data_quality_job_definitions: HashMap<String, JobDefinition>,
    pub model_quality_job_definitions: HashMap<String, JobDefinition>,
    pub model_bias_job_definitions: HashMap<String, JobDefinition>,
    pub model_explainability_job_definitions: HashMap<String, JobDefinition>,
    pub model_cards: HashMap<String, ModelCard>,
    pub model_packages: HashMap<String, ModelPackage>,
    pub model_package_groups: HashMap<String, ModelPackageGroup>,
    pub notebook_instance_lifecycle_configs: HashMap<String, NotebookInstanceLifecycleConfig>,
    pub user_profiles: HashMap<String, UserProfile>,
    pub spaces: HashMap<String, Space>,
    pub apps: HashMap<String, App>,
    pub tags: TagStore,
}

impl SageMakerState {
    // ============================================================
    // Notebook Instances
    // ============================================================

    pub fn create_notebook_instance(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
        instance_type: &str,
        role_arn: &str,
        volume_size_in_gb: Option<i64>,
        direct_internet_access: Option<&str>,
        root_access: Option<&str>,
    ) -> Result<&NotebookInstance, SageMakerError> {
        if self.notebook_instances.contains_key(name) {
            return Err(already_exists("Notebook Instance", name));
        }

        let now = Utc::now();
        let arn = format!("arn:aws:sagemaker:{region}:{account_id}:notebook-instance/{name}");
        let url = format!("{name}.notebook.{region}.sagemaker.aws");

        let instance = NotebookInstance {
            notebook_instance_name: name.to_string(),
            notebook_instance_arn: arn,
            notebook_instance_status: "Pending".to_string(),
            instance_type: instance_type.to_string(),
            role_arn: role_arn.to_string(),
            creation_time: now,
            last_modified_time: now,
            direct_internet_access: direct_internet_access.unwrap_or("Enabled").to_string(),
            volume_size_in_gb: volume_size_in_gb.unwrap_or(5),
            root_access: root_access.unwrap_or("Enabled").to_string(),
            url,
            instance_metadata_service_configuration: None,
        };

        self.notebook_instances.insert(name.to_string(), instance);
        Ok(self.notebook_instances.get(name).unwrap())
    }

    pub fn describe_notebook_instance(
        &self,
        name: &str,
    ) -> Result<&NotebookInstance, SageMakerError> {
        self.notebook_instances
            .get(name)
            .ok_or_else(|| not_found("Notebook instance", name))
    }

    pub fn delete_notebook_instance(&mut self, name: &str) -> Result<(), SageMakerError> {
        if self.notebook_instances.remove(name).is_none() {
            return Err(not_found("Notebook instance", name));
        }
        Ok(())
    }

    pub fn list_notebook_instances(&self) -> Vec<&NotebookInstance> {
        let mut instances: Vec<&NotebookInstance> = self.notebook_instances.values().collect();
        instances.sort_by(|a, b| a.notebook_instance_name.cmp(&b.notebook_instance_name));
        instances
    }

    pub fn start_notebook_instance(&mut self, name: &str) -> Result<(), SageMakerError> {
        let instance = self
            .notebook_instances
            .get_mut(name)
            .ok_or_else(|| not_found("Notebook instance", name))?;
        instance.notebook_instance_status = "InService".to_string();
        instance.last_modified_time = Utc::now();
        Ok(())
    }

    pub fn stop_notebook_instance(&mut self, name: &str) -> Result<(), SageMakerError> {
        let instance = self
            .notebook_instances
            .get_mut(name)
            .ok_or_else(|| not_found("Notebook instance", name))?;
        instance.notebook_instance_status = "Stopped".to_string();
        instance.last_modified_time = Utc::now();
        Ok(())
    }

    // ============================================================
    // Models
    // ============================================================

    pub fn create_model(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
        execution_role_arn: &str,
    ) -> Result<&Model, SageMakerError> {
        if self.models.contains_key(name) {
            return Err(already_exists("Model", name));
        }
        let arn = format!("arn:aws:sagemaker:{region}:{account_id}:model/{name}");
        let model = Model {
            model_name: name.to_string(),
            model_arn: arn,
            execution_role_arn: execution_role_arn.to_string(),
            creation_time: Utc::now(),
            tags: Vec::new(),
            container: None,
            primary_container: None,
            inference_execution_config: None,
            vpc_config: None,
        };
        self.models.insert(name.to_string(), model);
        Ok(self.models.get(name).unwrap())
    }

    pub fn describe_model(&self, name: &str) -> Result<&Model, SageMakerError> {
        self.models
            .get(name)
            .ok_or_else(|| not_found("Model", name))
    }

    pub fn delete_model(&mut self, name: &str) -> Result<(), SageMakerError> {
        self.models
            .remove(name)
            .ok_or_else(|| not_found("Model", name))?;
        Ok(())
    }

    pub fn list_models(&self) -> Vec<&Model> {
        let mut v: Vec<&Model> = self.models.values().collect();
        v.sort_by(|a, b| a.model_name.cmp(&b.model_name));
        v
    }

    // ============================================================
    // Endpoint Configs
    // ============================================================

    pub fn create_endpoint_config(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
    ) -> Result<&EndpointConfig, SageMakerError> {
        if self.endpoint_configs.contains_key(name) {
            return Err(already_exists("EndpointConfig", name));
        }
        let arn = format!("arn:aws:sagemaker:{region}:{account_id}:endpoint-config/{name}");
        let ec = EndpointConfig {
            endpoint_config_name: name.to_string(),
            endpoint_config_arn: arn,
            creation_time: Utc::now(),
            tags: Vec::new(),
            production_variants: None,
            async_inference_config: None,
            data_capture_config: None,
        };
        self.endpoint_configs.insert(name.to_string(), ec);
        Ok(self.endpoint_configs.get(name).unwrap())
    }

    pub fn describe_endpoint_config(&self, name: &str) -> Result<&EndpointConfig, SageMakerError> {
        self.endpoint_configs
            .get(name)
            .ok_or_else(|| not_found("EndpointConfig", name))
    }

    pub fn delete_endpoint_config(&mut self, name: &str) -> Result<(), SageMakerError> {
        self.endpoint_configs
            .remove(name)
            .ok_or_else(|| not_found("EndpointConfig", name))?;
        Ok(())
    }

    pub fn list_endpoint_configs(&self) -> Vec<&EndpointConfig> {
        let mut v: Vec<&EndpointConfig> = self.endpoint_configs.values().collect();
        v.sort_by(|a, b| a.endpoint_config_name.cmp(&b.endpoint_config_name));
        v
    }

    // ============================================================
    // Endpoints
    // ============================================================

    pub fn create_endpoint(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
        endpoint_config_name: &str,
    ) -> Result<&Endpoint, SageMakerError> {
        if self.endpoints.contains_key(name) {
            return Err(already_exists("Endpoint", name));
        }
        let now = Utc::now();
        let arn = format!("arn:aws:sagemaker:{region}:{account_id}:endpoint/{name}");
        let ep = Endpoint {
            endpoint_name: name.to_string(),
            endpoint_arn: arn,
            endpoint_config_name: endpoint_config_name.to_string(),
            endpoint_status: "Creating".to_string(),
            creation_time: now,
            last_modified_time: now,
            tags: Vec::new(),
        };
        self.endpoints.insert(name.to_string(), ep);
        Ok(self.endpoints.get(name).unwrap())
    }

    pub fn describe_endpoint(&self, name: &str) -> Result<&Endpoint, SageMakerError> {
        self.endpoints
            .get(name)
            .ok_or_else(|| not_found("Endpoint", name))
    }

    pub fn delete_endpoint(&mut self, name: &str) -> Result<(), SageMakerError> {
        self.endpoints
            .remove(name)
            .ok_or_else(|| not_found("Endpoint", name))?;
        Ok(())
    }

    pub fn list_endpoints(&self) -> Vec<&Endpoint> {
        let mut v: Vec<&Endpoint> = self.endpoints.values().collect();
        v.sort_by(|a, b| a.endpoint_name.cmp(&b.endpoint_name));
        v
    }

    pub fn update_endpoint_weights_and_capacities(
        &self,
        name: &str,
    ) -> Result<&Endpoint, SageMakerError> {
        self.endpoints
            .get(name)
            .ok_or_else(|| not_found("Endpoint", name))
    }

    // ============================================================
    // Training Jobs
    // ============================================================

    pub fn create_training_job(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
        role_arn: &str,
    ) -> Result<&TrainingJob, SageMakerError> {
        if self.training_jobs.contains_key(name) {
            return Err(already_exists("TrainingJob", name));
        }
        let now = Utc::now();
        let arn = format!("arn:aws:sagemaker:{region}:{account_id}:training-job/{name}");
        let job = TrainingJob {
            training_job_name: name.to_string(),
            training_job_arn: arn,
            training_job_status: "InProgress".to_string(),
            secondary_status: "Starting".to_string(),
            role_arn: role_arn.to_string(),
            creation_time: now,
            last_modified_time: now,
            tags: Vec::new(),
        };
        self.training_jobs.insert(name.to_string(), job);
        Ok(self.training_jobs.get(name).unwrap())
    }

    pub fn describe_training_job(&self, name: &str) -> Result<&TrainingJob, SageMakerError> {
        self.training_jobs
            .get(name)
            .ok_or_else(|| not_found("TrainingJob", name))
    }

    pub fn list_training_jobs(&self) -> Vec<&TrainingJob> {
        let mut v: Vec<&TrainingJob> = self.training_jobs.values().collect();
        v.sort_by(|a, b| a.training_job_name.cmp(&b.training_job_name));
        v
    }

    // ============================================================
    // Processing Jobs
    // ============================================================

    pub fn create_processing_job(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
        role_arn: &str,
    ) -> Result<&ProcessingJob, SageMakerError> {
        if self.processing_jobs.contains_key(name) {
            return Err(already_exists("ProcessingJob", name));
        }
        let now = Utc::now();
        let arn = format!("arn:aws:sagemaker:{region}:{account_id}:processing-job/{name}");
        let job = ProcessingJob {
            processing_job_name: name.to_string(),
            processing_job_arn: arn,
            processing_job_status: "InProgress".to_string(),
            role_arn: role_arn.to_string(),
            creation_time: now,
            last_modified_time: now,
            tags: Vec::new(),
        };
        self.processing_jobs.insert(name.to_string(), job);
        Ok(self.processing_jobs.get(name).unwrap())
    }

    pub fn describe_processing_job(&self, name: &str) -> Result<&ProcessingJob, SageMakerError> {
        self.processing_jobs
            .get(name)
            .ok_or_else(|| not_found("ProcessingJob", name))
    }

    pub fn list_processing_jobs(&self) -> Vec<&ProcessingJob> {
        let mut v: Vec<&ProcessingJob> = self.processing_jobs.values().collect();
        v.sort_by(|a, b| a.processing_job_name.cmp(&b.processing_job_name));
        v
    }

    // ============================================================
    // Transform Jobs
    // ============================================================

    pub fn create_transform_job(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
        model_name: &str,
    ) -> Result<&TransformJob, SageMakerError> {
        if self.transform_jobs.contains_key(name) {
            return Err(already_exists("TransformJob", name));
        }
        let now = Utc::now();
        let arn = format!("arn:aws:sagemaker:{region}:{account_id}:transform-job/{name}");
        let job = TransformJob {
            transform_job_name: name.to_string(),
            transform_job_arn: arn,
            transform_job_status: "InProgress".to_string(),
            model_name: model_name.to_string(),
            creation_time: now,
            last_modified_time: now,
            tags: Vec::new(),
        };
        self.transform_jobs.insert(name.to_string(), job);
        Ok(self.transform_jobs.get(name).unwrap())
    }

    pub fn describe_transform_job(&self, name: &str) -> Result<&TransformJob, SageMakerError> {
        self.transform_jobs
            .get(name)
            .ok_or_else(|| not_found("TransformJob", name))
    }

    pub fn list_transform_jobs(&self) -> Vec<&TransformJob> {
        let mut v: Vec<&TransformJob> = self.transform_jobs.values().collect();
        v.sort_by(|a, b| a.transform_job_name.cmp(&b.transform_job_name));
        v
    }

    // ============================================================
    // Hyper Parameter Tuning Jobs
    // ============================================================

    pub fn create_hyper_parameter_tuning_job(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
    ) -> Result<&HyperParameterTuningJob, SageMakerError> {
        if self.hyper_parameter_tuning_jobs.contains_key(name) {
            return Err(already_exists("HyperParameterTuningJob", name));
        }
        let now = Utc::now();
        let arn =
            format!("arn:aws:sagemaker:{region}:{account_id}:hyper-parameter-tuning-job/{name}");
        let job = HyperParameterTuningJob {
            job_name: name.to_string(),
            job_arn: arn,
            job_status: "InProgress".to_string(),
            creation_time: now,
            last_modified_time: now,
            tags: Vec::new(),
        };
        self.hyper_parameter_tuning_jobs
            .insert(name.to_string(), job);
        Ok(self.hyper_parameter_tuning_jobs.get(name).unwrap())
    }

    pub fn describe_hyper_parameter_tuning_job(
        &self,
        name: &str,
    ) -> Result<&HyperParameterTuningJob, SageMakerError> {
        self.hyper_parameter_tuning_jobs
            .get(name)
            .ok_or_else(|| not_found("HyperParameterTuningJob", name))
    }

    pub fn delete_hyper_parameter_tuning_job(&mut self, name: &str) -> Result<(), SageMakerError> {
        self.hyper_parameter_tuning_jobs
            .remove(name)
            .ok_or_else(|| not_found("HyperParameterTuningJob", name))?;
        Ok(())
    }

    pub fn list_hyper_parameter_tuning_jobs(&self) -> Vec<&HyperParameterTuningJob> {
        let mut v: Vec<&HyperParameterTuningJob> =
            self.hyper_parameter_tuning_jobs.values().collect();
        v.sort_by(|a, b| a.job_name.cmp(&b.job_name));
        v
    }

    // ============================================================
    // Compilation Jobs
    // ============================================================

    pub fn create_compilation_job(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
    ) -> Result<&CompilationJob, SageMakerError> {
        if self.compilation_jobs.contains_key(name) {
            return Err(already_exists("CompilationJob", name));
        }
        let now = Utc::now();
        let arn = format!("arn:aws:sagemaker:{region}:{account_id}:compilation-job/{name}");
        let job = CompilationJob {
            compilation_job_name: name.to_string(),
            compilation_job_arn: arn,
            compilation_job_status: "INPROGRESS".to_string(),
            creation_time: now,
            last_modified_time: now,
            tags: Vec::new(),
        };
        self.compilation_jobs.insert(name.to_string(), job);
        Ok(self.compilation_jobs.get(name).unwrap())
    }

    pub fn describe_compilation_job(&self, name: &str) -> Result<&CompilationJob, SageMakerError> {
        self.compilation_jobs
            .get(name)
            .ok_or_else(|| not_found("CompilationJob", name))
    }

    pub fn delete_compilation_job(&mut self, name: &str) -> Result<(), SageMakerError> {
        self.compilation_jobs
            .remove(name)
            .ok_or_else(|| not_found("CompilationJob", name))?;
        Ok(())
    }

    pub fn list_compilation_jobs(&self) -> Vec<&CompilationJob> {
        let mut v: Vec<&CompilationJob> = self.compilation_jobs.values().collect();
        v.sort_by(|a, b| a.compilation_job_name.cmp(&b.compilation_job_name));
        v
    }

    // ============================================================
    // AutoML Jobs V2
    // ============================================================

    pub fn create_auto_ml_job_v2(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
    ) -> Result<&AutoMLJobV2, SageMakerError> {
        if self.auto_ml_jobs_v2.contains_key(name) {
            return Err(already_exists("AutoMLJob", name));
        }
        let now = Utc::now();
        let arn = format!("arn:aws:sagemaker:{region}:{account_id}:automl-job/{name}");
        let job = AutoMLJobV2 {
            auto_ml_job_name: name.to_string(),
            auto_ml_job_arn: arn,
            auto_ml_job_status: "InProgress".to_string(),
            creation_time: now,
            last_modified_time: now,
            tags: Vec::new(),
        };
        self.auto_ml_jobs_v2.insert(name.to_string(), job);
        Ok(self.auto_ml_jobs_v2.get(name).unwrap())
    }

    pub fn describe_auto_ml_job_v2(&self, name: &str) -> Result<&AutoMLJobV2, SageMakerError> {
        self.auto_ml_jobs_v2
            .get(name)
            .ok_or_else(|| not_found("AutoMLJob", name))
    }

    pub fn stop_auto_ml_job(&mut self, name: &str) -> Result<(), SageMakerError> {
        let job = self
            .auto_ml_jobs_v2
            .get_mut(name)
            .ok_or_else(|| not_found("AutoMLJob", name))?;
        job.auto_ml_job_status = "Stopped".to_string();
        job.last_modified_time = Utc::now();
        Ok(())
    }

    pub fn list_auto_ml_jobs(&self) -> Vec<&AutoMLJobV2> {
        let mut v: Vec<&AutoMLJobV2> = self.auto_ml_jobs_v2.values().collect();
        v.sort_by(|a, b| a.auto_ml_job_name.cmp(&b.auto_ml_job_name));
        v
    }

    // ============================================================
    // Experiments
    // ============================================================

    pub fn create_experiment(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
        display_name: Option<&str>,
        description: Option<&str>,
    ) -> Result<&Experiment, SageMakerError> {
        if self.experiments.contains_key(name) {
            return Err(already_exists("Experiment", name));
        }
        let now = Utc::now();
        let arn = format!("arn:aws:sagemaker:{region}:{account_id}:experiment/{name}");
        let exp = Experiment {
            experiment_name: name.to_string(),
            experiment_arn: arn,
            display_name: display_name.map(|s| s.to_string()),
            description: description.map(|s| s.to_string()),
            creation_time: now,
            last_modified_time: now,
            tags: Vec::new(),
        };
        self.experiments.insert(name.to_string(), exp);
        Ok(self.experiments.get(name).unwrap())
    }

    pub fn describe_experiment(&self, name: &str) -> Result<&Experiment, SageMakerError> {
        self.experiments
            .get(name)
            .ok_or_else(|| resource_not_found("Experiment", name))
    }

    pub fn delete_experiment(&mut self, name: &str) -> Result<String, SageMakerError> {
        let exp = self
            .experiments
            .remove(name)
            .ok_or_else(|| resource_not_found("Experiment", name))?;
        Ok(exp.experiment_arn)
    }

    pub fn list_experiments(&self) -> Vec<&Experiment> {
        let mut v: Vec<&Experiment> = self.experiments.values().collect();
        v.sort_by(|a, b| a.experiment_name.cmp(&b.experiment_name));
        v
    }

    // ============================================================
    // Trials
    // ============================================================

    pub fn create_trial(
        &mut self,
        account_id: &str,
        region: &str,
        trial_name: &str,
        experiment_name: &str,
        display_name: Option<&str>,
    ) -> Result<&Trial, SageMakerError> {
        if self.trials.contains_key(trial_name) {
            return Err(already_exists("Trial", trial_name));
        }
        let now = Utc::now();
        let arn = format!("arn:aws:sagemaker:{region}:{account_id}:experiment-trial/{trial_name}");
        let trial = Trial {
            trial_name: trial_name.to_string(),
            trial_arn: arn,
            experiment_name: experiment_name.to_string(),
            display_name: display_name.map(|s| s.to_string()),
            creation_time: now,
            last_modified_time: now,
            trial_component_names: Vec::new(),
            tags: Vec::new(),
        };
        self.trials.insert(trial_name.to_string(), trial);
        Ok(self.trials.get(trial_name).unwrap())
    }

    pub fn describe_trial(&self, name: &str) -> Result<&Trial, SageMakerError> {
        self.trials
            .get(name)
            .ok_or_else(|| resource_not_found("Trial", name))
    }

    pub fn delete_trial(&mut self, name: &str) -> Result<String, SageMakerError> {
        let trial = self
            .trials
            .remove(name)
            .ok_or_else(|| resource_not_found("Trial", name))?;
        Ok(trial.trial_arn)
    }

    pub fn list_trials(&self) -> Vec<&Trial> {
        let mut v: Vec<&Trial> = self.trials.values().collect();
        v.sort_by(|a, b| a.trial_name.cmp(&b.trial_name));
        v
    }

    /// List trials, optionally filtered by trial component name (via associations).
    pub fn list_trials_filtered(&self, tc_name: Option<&str>) -> Vec<&Trial> {
        match tc_name {
            None => self.list_trials(),
            Some(tcn) => {
                let mut v: Vec<&Trial> = self
                    .trials
                    .values()
                    .filter(|t| t.trial_component_names.iter().any(|n| n == tcn))
                    .collect();
                v.sort_by(|a, b| a.trial_name.cmp(&b.trial_name));
                v
            }
        }
    }

    // ============================================================
    // Trial Components
    // ============================================================

    pub fn create_trial_component(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
        display_name: Option<&str>,
    ) -> Result<&TrialComponent, SageMakerError> {
        if self.trial_components.contains_key(name) {
            return Err(already_exists("TrialComponent", name));
        }
        let now = Utc::now();
        let arn =
            format!("arn:aws:sagemaker:{region}:{account_id}:experiment-trial-component/{name}");
        let tc = TrialComponent {
            trial_component_name: name.to_string(),
            trial_component_arn: arn,
            display_name: display_name.map(|s| s.to_string()),
            status: "InProgress".to_string(),
            creation_time: now,
            last_modified_time: now,
            tags: Vec::new(),
        };
        self.trial_components.insert(name.to_string(), tc);
        Ok(self.trial_components.get(name).unwrap())
    }

    pub fn describe_trial_component(&self, name: &str) -> Result<&TrialComponent, SageMakerError> {
        self.trial_components
            .get(name)
            .ok_or_else(|| resource_not_found("TrialComponent", name))
    }

    pub fn delete_trial_component(&mut self, name: &str) -> Result<String, SageMakerError> {
        let tc = self
            .trial_components
            .remove(name)
            .ok_or_else(|| resource_not_found("TrialComponent", name))?;
        // Remove from any trials
        for trial in self.trials.values_mut() {
            trial.trial_component_names.retain(|n| n != name);
        }
        Ok(tc.trial_component_arn)
    }

    pub fn update_trial_component(
        &mut self,
        name: &str,
        display_name: Option<&str>,
    ) -> Result<&TrialComponent, SageMakerError> {
        let tc = self
            .trial_components
            .get_mut(name)
            .ok_or_else(|| resource_not_found("TrialComponent", name))?;
        if let Some(dn) = display_name {
            tc.display_name = Some(dn.to_string());
        }
        tc.last_modified_time = Utc::now();
        Ok(self.trial_components.get(name).unwrap())
    }

    pub fn list_trial_components(&self) -> Vec<&TrialComponent> {
        let mut v: Vec<&TrialComponent> = self.trial_components.values().collect();
        v.sort_by(|a, b| a.trial_component_name.cmp(&b.trial_component_name));
        v
    }

    /// List trial components, optionally filtered by trial name (via associations).
    pub fn list_trial_components_filtered(&self, trial_name: Option<&str>) -> Vec<&TrialComponent> {
        match trial_name {
            None => self.list_trial_components(),
            Some(tname) => {
                // Find which trial components are associated with this trial.
                let associated: std::collections::HashSet<&str> = self
                    .trials
                    .get(tname)
                    .map(|t| t.trial_component_names.iter().map(|s| s.as_str()).collect())
                    .unwrap_or_default();
                let mut v: Vec<&TrialComponent> = self
                    .trial_components
                    .values()
                    .filter(|tc| associated.contains(tc.trial_component_name.as_str()))
                    .collect();
                v.sort_by(|a, b| a.trial_component_name.cmp(&b.trial_component_name));
                v
            }
        }
    }

    pub fn associate_trial_component(
        &mut self,
        trial_name: &str,
        trial_component_name: &str,
    ) -> Result<(String, String), SageMakerError> {
        let trial = self
            .trials
            .get_mut(trial_name)
            .ok_or_else(|| resource_not_found("Trial", trial_name))?;
        let trial_arn = trial.trial_arn.clone();
        if !trial
            .trial_component_names
            .contains(&trial_component_name.to_string())
        {
            trial
                .trial_component_names
                .push(trial_component_name.to_string());
        }
        let tc_arn = self
            .trial_components
            .get(trial_component_name)
            .ok_or_else(|| resource_not_found("TrialComponent", trial_component_name))?
            .trial_component_arn
            .clone();
        Ok((trial_arn, tc_arn))
    }

    pub fn disassociate_trial_component(&mut self, trial_name: &str, trial_component_name: &str) {
        // Idempotent: silently does nothing if trial or component doesn't exist.
        if let Some(trial) = self.trials.get_mut(trial_name) {
            trial
                .trial_component_names
                .retain(|n| n != trial_component_name);
        }
    }

    // ============================================================
    // Pipelines
    // ============================================================

    pub fn create_pipeline(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
        role_arn: &str,
        pipeline_definition: Option<&str>,
        display_name: Option<&str>,
        description: Option<&str>,
    ) -> Result<&Pipeline, SageMakerError> {
        if self.pipelines.contains_key(name) {
            return Err(already_exists("Pipeline", name));
        }
        let now = Utc::now();
        let arn = format!("arn:aws:sagemaker:{region}:{account_id}:pipeline/{name}");
        let pipeline = Pipeline {
            pipeline_name: name.to_string(),
            pipeline_arn: arn,
            pipeline_display_name: display_name.map(|s| s.to_string()),
            pipeline_description: description.map(|s| s.to_string()),
            pipeline_definition: pipeline_definition.map(|s| s.to_string()),
            role_arn: role_arn.to_string(),
            creation_time: now,
            last_modified_time: now,
            executions: Vec::new(),
            tags: Vec::new(),
        };
        self.pipelines.insert(name.to_string(), pipeline);
        Ok(self.pipelines.get(name).unwrap())
    }

    pub fn describe_pipeline(&self, name: &str) -> Result<&Pipeline, SageMakerError> {
        self.pipelines
            .get(name)
            .ok_or_else(|| resource_not_found("Pipeline", name))
    }

    pub fn describe_pipeline_definition_for_execution(
        &self,
        pipeline_execution_arn: &str,
    ) -> Result<Option<String>, SageMakerError> {
        for pipeline in self.pipelines.values() {
            for exec in &pipeline.executions {
                if exec.pipeline_execution_arn == pipeline_execution_arn {
                    return Ok(pipeline.pipeline_definition.clone());
                }
            }
        }
        Err(resource_not_found(
            "PipelineExecution",
            pipeline_execution_arn,
        ))
    }

    pub fn update_pipeline(
        &mut self,
        name: &str,
        pipeline_definition: Option<&str>,
        description: Option<&str>,
        role_arn: Option<&str>,
    ) -> Result<&Pipeline, SageMakerError> {
        let pipeline = self
            .pipelines
            .get_mut(name)
            .ok_or_else(|| resource_not_found("Pipeline", name))?;
        if let Some(d) = pipeline_definition {
            pipeline.pipeline_definition = Some(d.to_string());
        }
        if let Some(d) = description {
            pipeline.pipeline_description = Some(d.to_string());
        }
        if let Some(r) = role_arn {
            pipeline.role_arn = r.to_string();
        }
        pipeline.last_modified_time = Utc::now();
        Ok(self.pipelines.get(name).unwrap())
    }

    pub fn delete_pipeline(&mut self, name: &str) -> Result<String, SageMakerError> {
        let pipeline = self
            .pipelines
            .remove(name)
            .ok_or_else(|| resource_not_found("Pipeline", name))?;
        Ok(pipeline.pipeline_arn)
    }

    pub fn list_pipelines(&self) -> Vec<&Pipeline> {
        let mut v: Vec<&Pipeline> = self.pipelines.values().collect();
        v.sort_by(|a, b| a.pipeline_name.cmp(&b.pipeline_name));
        v
    }

    pub fn start_pipeline_execution(
        &mut self,
        pipeline_name: &str,
        account_id: &str,
        region: &str,
        parameters: Vec<PipelineParameter>,
    ) -> Result<String, SageMakerError> {
        let pipeline = self
            .pipelines
            .get_mut(pipeline_name)
            .ok_or_else(|| resource_not_found("Pipeline", pipeline_name))?;
        let now = Utc::now();
        let exec_id = uuid::Uuid::new_v4().to_string();
        let exec_arn = format!(
            "arn:aws:sagemaker:{region}:{account_id}:pipeline/{pipeline_name}/execution/{exec_id}"
        );
        let execution = PipelineExecution {
            pipeline_execution_arn: exec_arn.clone(),
            pipeline_execution_status: "Executing".to_string(),
            pipeline_execution_description: None,
            creation_time: now,
            last_modified_time: now,
            parameters,
        };
        pipeline.executions.push(execution);
        Ok(exec_arn)
    }

    pub fn describe_pipeline_execution(
        &self,
        pipeline_execution_arn: &str,
    ) -> Result<(&Pipeline, &PipelineExecution), SageMakerError> {
        for pipeline in self.pipelines.values() {
            for exec in &pipeline.executions {
                if exec.pipeline_execution_arn == pipeline_execution_arn {
                    return Ok((pipeline, exec));
                }
            }
        }
        Err(resource_not_found(
            "PipelineExecution",
            pipeline_execution_arn,
        ))
    }

    pub fn list_pipeline_executions(
        &self,
        pipeline_name: &str,
    ) -> Result<Vec<&PipelineExecution>, SageMakerError> {
        let pipeline = self
            .pipelines
            .get(pipeline_name)
            .ok_or_else(|| resource_not_found("Pipeline", pipeline_name))?;
        Ok(pipeline.executions.iter().collect())
    }

    pub fn list_pipeline_parameters_for_execution(
        &self,
        pipeline_execution_arn: &str,
    ) -> Result<Vec<&PipelineParameter>, SageMakerError> {
        for pipeline in self.pipelines.values() {
            for exec in &pipeline.executions {
                if exec.pipeline_execution_arn == pipeline_execution_arn {
                    return Ok(exec.parameters.iter().collect());
                }
            }
        }
        Err(resource_not_found(
            "PipelineExecution",
            pipeline_execution_arn,
        ))
    }

    // ============================================================
    // Feature Groups
    // ============================================================

    pub fn create_feature_group(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
    ) -> Result<&FeatureGroup, SageMakerError> {
        if self.feature_groups.contains_key(name) {
            return Err(already_exists("FeatureGroup", name));
        }
        let now = Utc::now();
        let arn = format!("arn:aws:sagemaker:{region}:{account_id}:feature-group/{name}");
        let fg = FeatureGroup {
            feature_group_name: name.to_string(),
            feature_group_arn: arn,
            feature_group_status: "Created".to_string(),
            creation_time: now,
            last_modified_time: now,
            tags: Vec::new(),
        };
        self.feature_groups.insert(name.to_string(), fg);
        Ok(self.feature_groups.get(name).unwrap())
    }

    pub fn describe_feature_group(&self, name: &str) -> Result<&FeatureGroup, SageMakerError> {
        self.feature_groups
            .get(name)
            .ok_or_else(|| resource_not_found("FeatureGroup", name))
    }

    // ============================================================
    // Domains
    // ============================================================

    pub fn create_domain(
        &mut self,
        account_id: &str,
        region: &str,
        domain_name: &str,
    ) -> Result<&Domain, SageMakerError> {
        let now = Utc::now();
        let domain_id = format!("d-{}", &uuid::Uuid::new_v4().to_string()[..12]);
        let arn = format!("arn:aws:sagemaker:{region}:{account_id}:domain/{domain_id}");
        let domain = Domain {
            domain_id: domain_id.clone(),
            domain_name: domain_name.to_string(),
            domain_arn: arn,
            status: "InService".to_string(),
            creation_time: now,
            last_modified_time: now,
            tags: Vec::new(),
            vpc_id: None,
            subnet_ids: Vec::new(),
            app_network_access_type: None,
            auth_mode: None,
            kms_key_id: None,
            home_efs_file_system_id: None,
            security_group_ids: Vec::new(),
            url: None,
            default_space_settings: None,
            domain_settings: None,
            retention_policy: None,
        };
        self.domains.insert(domain_id.clone(), domain);
        Ok(self.domains.get(&domain_id).unwrap())
    }

    pub fn describe_domain(&self, domain_id: &str) -> Result<&Domain, SageMakerError> {
        self.domains
            .get(domain_id)
            .ok_or_else(|| resource_not_found("Domain", domain_id))
    }

    pub fn delete_domain(&mut self, domain_id: &str) -> Result<(), SageMakerError> {
        self.domains
            .remove(domain_id)
            .ok_or_else(|| resource_not_found("Domain", domain_id))?;
        Ok(())
    }

    pub fn list_domains(&self) -> Vec<&Domain> {
        let mut v: Vec<&Domain> = self.domains.values().collect();
        v.sort_by(|a, b| a.domain_name.cmp(&b.domain_name));
        v
    }

    // ============================================================
    // Clusters
    // ============================================================

    pub fn create_cluster(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
    ) -> Result<&Cluster, SageMakerError> {
        if self.clusters.contains_key(name) {
            return Err(already_exists("Cluster", name));
        }
        let now = Utc::now();
        let arn = format!("arn:aws:sagemaker:{region}:{account_id}:cluster/{name}");
        let cluster = Cluster {
            cluster_name: name.to_string(),
            cluster_arn: arn,
            cluster_status: "InService".to_string(),
            creation_time: now,
            tags: Vec::new(),
        };
        // Create a default node
        let node = ClusterNode {
            instance_group_name: "default-group".to_string(),
            instance_id: format!("i-{}", &uuid::Uuid::new_v4().to_string()[..12]),
            instance_status: "Running".to_string(),
            instance_type: "ml.m5.xlarge".to_string(),
            launch_time: now,
        };
        self.cluster_nodes.insert(name.to_string(), vec![node]);
        self.clusters.insert(name.to_string(), cluster);
        Ok(self.clusters.get(name).unwrap())
    }

    pub fn describe_cluster(&self, name: &str) -> Result<&Cluster, SageMakerError> {
        self.clusters
            .get(name)
            .ok_or_else(|| resource_not_found("Cluster", name))
    }

    pub fn delete_cluster(&mut self, name: &str) -> Result<(), SageMakerError> {
        self.clusters
            .remove(name)
            .ok_or_else(|| resource_not_found("Cluster", name))?;
        self.cluster_nodes.remove(name);
        Ok(())
    }

    pub fn list_clusters(&self) -> Vec<&Cluster> {
        let mut v: Vec<&Cluster> = self.clusters.values().collect();
        v.sort_by(|a, b| a.cluster_name.cmp(&b.cluster_name));
        v
    }

    pub fn describe_cluster_node(
        &self,
        cluster_name: &str,
        node_id: &str,
    ) -> Result<&ClusterNode, SageMakerError> {
        let nodes = self
            .cluster_nodes
            .get(cluster_name)
            .ok_or_else(|| resource_not_found("Cluster", cluster_name))?;
        nodes
            .iter()
            .find(|n| n.instance_id == node_id)
            .ok_or_else(|| resource_not_found("ClusterNode", node_id))
    }

    pub fn list_cluster_nodes(
        &self,
        cluster_name: &str,
    ) -> Result<Vec<&ClusterNode>, SageMakerError> {
        let nodes = self
            .cluster_nodes
            .get(cluster_name)
            .ok_or_else(|| resource_not_found("Cluster", cluster_name))?;
        Ok(nodes.iter().collect())
    }

    // ============================================================
    // Job Definitions (Data Quality, Model Quality, Model Bias, Model Explainability)
    // ============================================================

    pub fn create_job_definition(
        &mut self,
        store: JobDefinitionType,
        account_id: &str,
        region: &str,
        name: &str,
        role_arn: &str,
    ) -> Result<String, SageMakerError> {
        let map = self.get_job_def_map_mut(store);
        if map.contains_key(name) {
            return Err(already_exists("JobDefinition", name));
        }
        let resource_type = match store {
            JobDefinitionType::DataQuality => "data-quality-job-definition",
            JobDefinitionType::ModelQuality => "model-quality-job-definition",
            JobDefinitionType::ModelBias => "model-bias-job-definition",
            JobDefinitionType::ModelExplainability => "model-explainability-job-definition",
        };
        let arn = format!("arn:aws:sagemaker:{region}:{account_id}:{resource_type}/{name}");
        let jd = JobDefinition {
            job_definition_name: name.to_string(),
            job_definition_arn: arn.clone(),
            role_arn: role_arn.to_string(),
            creation_time: Utc::now(),
            tags: Vec::new(),
        };
        map.insert(name.to_string(), jd);
        Ok(arn)
    }

    pub fn describe_job_definition(
        &self,
        store: JobDefinitionType,
        name: &str,
    ) -> Result<&JobDefinition, SageMakerError> {
        let map = self.get_job_def_map(store);
        map.get(name)
            .ok_or_else(|| resource_not_found("JobDefinition", name))
    }

    pub fn delete_job_definition(
        &mut self,
        store: JobDefinitionType,
        name: &str,
    ) -> Result<(), SageMakerError> {
        let map = self.get_job_def_map_mut(store);
        map.remove(name)
            .ok_or_else(|| resource_not_found("JobDefinition", name))?;
        Ok(())
    }

    pub fn list_job_definitions(&self, store: JobDefinitionType) -> Vec<&JobDefinition> {
        let map = self.get_job_def_map(store);
        let mut v: Vec<&JobDefinition> = map.values().collect();
        v.sort_by(|a, b| a.job_definition_name.cmp(&b.job_definition_name));
        v
    }

    fn get_job_def_map(&self, store: JobDefinitionType) -> &HashMap<String, JobDefinition> {
        match store {
            JobDefinitionType::DataQuality => &self.data_quality_job_definitions,
            JobDefinitionType::ModelQuality => &self.model_quality_job_definitions,
            JobDefinitionType::ModelBias => &self.model_bias_job_definitions,
            JobDefinitionType::ModelExplainability => &self.model_explainability_job_definitions,
        }
    }

    fn get_job_def_map_mut(
        &mut self,
        store: JobDefinitionType,
    ) -> &mut HashMap<String, JobDefinition> {
        match store {
            JobDefinitionType::DataQuality => &mut self.data_quality_job_definitions,
            JobDefinitionType::ModelQuality => &mut self.model_quality_job_definitions,
            JobDefinitionType::ModelBias => &mut self.model_bias_job_definitions,
            JobDefinitionType::ModelExplainability => {
                &mut self.model_explainability_job_definitions
            }
        }
    }

    // ============================================================
    // Model Cards
    // ============================================================

    pub fn create_model_card(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
        content: &str,
        model_card_status: &str,
    ) -> Result<&ModelCard, SageMakerError> {
        if self.model_cards.contains_key(name) {
            return Err(already_exists("ModelCard", name));
        }
        let now = Utc::now();
        let arn = format!("arn:aws:sagemaker:{region}:{account_id}:model-card/{name}");
        let mc = ModelCard {
            model_card_name: name.to_string(),
            model_card_arn: arn,
            model_card_status: model_card_status.to_string(),
            content: content.to_string(),
            model_card_version: 1,
            creation_time: now,
            last_modified_time: now,
            tags: Vec::new(),
        };
        self.model_cards.insert(name.to_string(), mc);
        Ok(self.model_cards.get(name).unwrap())
    }

    pub fn describe_model_card(&self, name: &str) -> Result<&ModelCard, SageMakerError> {
        self.model_cards
            .get(name)
            .ok_or_else(|| resource_not_found("ModelCard", name))
    }

    pub fn update_model_card(
        &mut self,
        name: &str,
        content: Option<&str>,
        model_card_status: Option<&str>,
    ) -> Result<&ModelCard, SageMakerError> {
        let mc = self
            .model_cards
            .get_mut(name)
            .ok_or_else(|| resource_not_found("ModelCard", name))?;
        if let Some(c) = content {
            mc.content = c.to_string();
        }
        if let Some(s) = model_card_status {
            mc.model_card_status = s.to_string();
        }
        mc.model_card_version += 1;
        mc.last_modified_time = Utc::now();
        Ok(self.model_cards.get(name).unwrap())
    }

    pub fn delete_model_card(&mut self, name: &str) -> Result<(), SageMakerError> {
        self.model_cards
            .remove(name)
            .ok_or_else(|| resource_not_found("ModelCard", name))?;
        Ok(())
    }

    pub fn list_model_cards(&self) -> Vec<&ModelCard> {
        let mut v: Vec<&ModelCard> = self.model_cards.values().collect();
        v.sort_by(|a, b| a.model_card_name.cmp(&b.model_card_name));
        v
    }

    // ============================================================
    // Model Packages
    // ============================================================

    pub fn create_model_package(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
        description: Option<&str>,
        model_approval_status: Option<&str>,
    ) -> Result<&ModelPackage, SageMakerError> {
        if self.model_packages.contains_key(name) {
            return Err(already_exists("ModelPackage", name));
        }
        let now = Utc::now();
        let arn = format!(
            "arn:aws:sagemaker:{region}:{account_id}:model-package/{}",
            name.to_lowercase()
        );
        let mp = ModelPackage {
            model_package_name: name.to_string(),
            model_package_arn: arn,
            model_package_status: "Completed".to_string(),
            model_package_description: description.map(|s| s.to_string()),
            model_approval_status: model_approval_status.map(|s| s.to_string()),
            creation_time: now,
            last_modified_time: now,
            tags: Vec::new(),
        };
        self.model_packages.insert(name.to_string(), mp);
        Ok(self.model_packages.get(name).unwrap())
    }

    /// Create a versioned model package within a model package group.
    pub fn create_versioned_model_package(
        &mut self,
        account_id: &str,
        region: &str,
        group_name: &str,
        description: Option<&str>,
        model_approval_status: Option<&str>,
    ) -> Result<&ModelPackage, SageMakerError> {
        // Ensure the group exists
        if !self.model_package_groups.contains_key(group_name) {
            return Err(SageMakerError::ModelPackageGroupNotFound);
        }
        let version = {
            let group = self.model_package_groups.get_mut(group_name).unwrap();
            let v = group.next_version;
            group.next_version += 1;
            v
        };
        let key = format!("{group_name}/{version}");
        let now = Utc::now();
        let arn =
            format!("arn:aws:sagemaker:{region}:{account_id}:model-package/{group_name}/{version}");
        let mp = ModelPackage {
            model_package_name: key.clone(),
            model_package_arn: arn,
            model_package_status: "Completed".to_string(),
            model_package_description: description.map(|s| s.to_string()),
            model_approval_status: model_approval_status.map(|s| s.to_string()),
            creation_time: now,
            last_modified_time: now,
            tags: Vec::new(),
        };
        self.model_packages.insert(key.clone(), mp);
        Ok(self.model_packages.get(&key).unwrap())
    }

    pub fn describe_model_package(&self, name: &str) -> Result<&ModelPackage, SageMakerError> {
        self.model_packages
            .get(name)
            .ok_or_else(|| resource_not_found("ModelPackage", name))
    }

    pub fn update_model_package(
        &mut self,
        name: &str,
        model_approval_status: Option<&str>,
    ) -> Result<&ModelPackage, SageMakerError> {
        let mp = self
            .model_packages
            .get_mut(name)
            .ok_or_else(|| resource_not_found("ModelPackage", name))?;
        if let Some(s) = model_approval_status {
            mp.model_approval_status = Some(s.to_string());
        }
        mp.last_modified_time = Utc::now();
        Ok(self.model_packages.get(name).unwrap())
    }

    pub fn list_model_packages(&self) -> Vec<&ModelPackage> {
        let mut v: Vec<&ModelPackage> = self.model_packages.values().collect();
        v.sort_by(|a, b| a.model_package_name.cmp(&b.model_package_name));
        v
    }

    // ============================================================
    // Model Package Groups
    // ============================================================

    pub fn create_model_package_group(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
        description: Option<&str>,
    ) -> Result<&ModelPackageGroup, SageMakerError> {
        if self.model_package_groups.contains_key(name) {
            return Err(already_exists("ModelPackageGroup", name));
        }
        let now = Utc::now();
        let arn = format!("arn:aws:sagemaker:{region}:{account_id}:model-package-group/{name}");
        let mpg = ModelPackageGroup {
            model_package_group_name: name.to_string(),
            model_package_group_arn: arn,
            model_package_group_status: "Completed".to_string(),
            model_package_group_description: description.map(|s| s.to_string()),
            creation_time: now,
            tags: Vec::new(),
            next_version: 1,
        };
        self.model_package_groups.insert(name.to_string(), mpg);
        Ok(self.model_package_groups.get(name).unwrap())
    }

    pub fn describe_model_package_group(
        &self,
        name: &str,
    ) -> Result<&ModelPackageGroup, SageMakerError> {
        self.model_package_groups
            .get(name)
            .ok_or_else(|| resource_not_found("ModelPackageGroup", name))
    }

    pub fn list_model_package_groups(&self) -> Vec<&ModelPackageGroup> {
        let mut v: Vec<&ModelPackageGroup> = self.model_package_groups.values().collect();
        v.sort_by(|a, b| a.model_package_group_name.cmp(&b.model_package_group_name));
        v
    }

    // ============================================================
    // Notebook Instance Lifecycle Configs
    // ============================================================

    pub fn create_notebook_instance_lifecycle_config(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
    ) -> Result<&NotebookInstanceLifecycleConfig, SageMakerError> {
        if self.notebook_instance_lifecycle_configs.contains_key(name) {
            return Err(already_exists("NotebookInstanceLifecycleConfig", name));
        }
        let now = Utc::now();
        let arn = format!(
            "arn:aws:sagemaker:{region}:{account_id}:notebook-instance-lifecycle-config/{name}"
        );
        let config = NotebookInstanceLifecycleConfig {
            name: name.to_string(),
            arn,
            on_create: Vec::new(),
            on_start: Vec::new(),
            creation_time: now,
            last_modified_time: now,
        };
        self.notebook_instance_lifecycle_configs
            .insert(name.to_string(), config);
        Ok(self.notebook_instance_lifecycle_configs.get(name).unwrap())
    }

    pub fn describe_notebook_instance_lifecycle_config(
        &self,
        name: &str,
    ) -> Result<&NotebookInstanceLifecycleConfig, SageMakerError> {
        self.notebook_instance_lifecycle_configs
            .get(name)
            .ok_or_else(|| resource_not_found("NotebookInstanceLifecycleConfig", name))
    }

    pub fn delete_notebook_instance_lifecycle_config(
        &mut self,
        name: &str,
    ) -> Result<(), SageMakerError> {
        self.notebook_instance_lifecycle_configs
            .remove(name)
            .ok_or_else(|| resource_not_found("NotebookInstanceLifecycleConfig", name))?;
        Ok(())
    }

    // ============================================================
    // User Profiles
    // ============================================================

    pub fn create_user_profile(
        &mut self,
        account_id: &str,
        region: &str,
        domain_id: &str,
        user_profile_name: &str,
    ) -> Result<&UserProfile, SageMakerError> {
        let key = format!("{domain_id}/{user_profile_name}");
        if self.user_profiles.contains_key(&key) {
            return Err(already_exists("UserProfile", user_profile_name));
        }
        let now = Utc::now();
        let arn = format!(
            "arn:aws:sagemaker:{region}:{account_id}:user-profile/{domain_id}/{user_profile_name}"
        );
        let up = UserProfile {
            domain_id: domain_id.to_string(),
            user_profile_name: user_profile_name.to_string(),
            user_profile_arn: arn,
            status: "InService".to_string(),
            creation_time: now,
            last_modified_time: now,
            tags: Vec::new(),
        };
        self.user_profiles.insert(key.clone(), up);
        Ok(self.user_profiles.get(&key).unwrap())
    }

    pub fn describe_user_profile(
        &self,
        domain_id: &str,
        user_profile_name: &str,
    ) -> Result<&UserProfile, SageMakerError> {
        let key = format!("{domain_id}/{user_profile_name}");
        self.user_profiles
            .get(&key)
            .ok_or_else(|| resource_not_found("UserProfile", user_profile_name))
    }

    pub fn update_user_profile(
        &mut self,
        domain_id: &str,
        user_profile_name: &str,
    ) -> Result<&UserProfile, SageMakerError> {
        let key = format!("{domain_id}/{user_profile_name}");
        let up = self
            .user_profiles
            .get_mut(&key)
            .ok_or_else(|| resource_not_found("UserProfile", user_profile_name))?;
        up.last_modified_time = Utc::now();
        Ok(self.user_profiles.get(&key).unwrap())
    }

    pub fn delete_user_profile(
        &mut self,
        domain_id: &str,
        user_profile_name: &str,
    ) -> Result<(), SageMakerError> {
        let key = format!("{domain_id}/{user_profile_name}");
        self.user_profiles
            .remove(&key)
            .ok_or_else(|| resource_not_found("UserProfile", user_profile_name))?;
        Ok(())
    }

    pub fn list_user_profiles(&self) -> Vec<&UserProfile> {
        let mut v: Vec<&UserProfile> = self.user_profiles.values().collect();
        v.sort_by(|a, b| a.user_profile_name.cmp(&b.user_profile_name));
        v
    }

    // ============================================================
    // Spaces
    // ============================================================

    pub fn create_space(
        &mut self,
        account_id: &str,
        region: &str,
        domain_id: &str,
        space_name: &str,
    ) -> Result<&Space, SageMakerError> {
        let key = format!("{domain_id}/{space_name}");
        if self.spaces.contains_key(&key) {
            return Err(already_exists("Space", space_name));
        }
        let now = Utc::now();
        let arn = format!("arn:aws:sagemaker:{region}:{account_id}:space/{domain_id}/{space_name}");
        let space = Space {
            domain_id: domain_id.to_string(),
            space_name: space_name.to_string(),
            space_arn: arn,
            status: "InService".to_string(),
            creation_time: now,
            last_modified_time: now,
            tags: Vec::new(),
        };
        self.spaces.insert(key.clone(), space);
        Ok(self.spaces.get(&key).unwrap())
    }

    pub fn describe_space(
        &self,
        domain_id: &str,
        space_name: &str,
    ) -> Result<&Space, SageMakerError> {
        let key = format!("{domain_id}/{space_name}");
        self.spaces
            .get(&key)
            .ok_or_else(|| resource_not_found("Space", space_name))
    }

    pub fn update_space(
        &mut self,
        domain_id: &str,
        space_name: &str,
    ) -> Result<&Space, SageMakerError> {
        let key = format!("{domain_id}/{space_name}");
        let space = self
            .spaces
            .get_mut(&key)
            .ok_or_else(|| resource_not_found("Space", space_name))?;
        space.last_modified_time = Utc::now();
        Ok(self.spaces.get(&key).unwrap())
    }

    pub fn delete_space(
        &mut self,
        domain_id: &str,
        space_name: &str,
    ) -> Result<(), SageMakerError> {
        let key = format!("{domain_id}/{space_name}");
        self.spaces
            .remove(&key)
            .ok_or_else(|| resource_not_found("Space", space_name))?;
        Ok(())
    }

    pub fn list_spaces(&self) -> Vec<&Space> {
        let mut v: Vec<&Space> = self.spaces.values().collect();
        v.sort_by(|a, b| a.space_name.cmp(&b.space_name));
        v
    }

    // ============================================================
    // Apps
    // ============================================================

    pub fn create_app(
        &mut self,
        account_id: &str,
        region: &str,
        domain_id: &str,
        user_profile_name: Option<&str>,
        space_name: Option<&str>,
        app_type: &str,
        app_name: &str,
    ) -> Result<&App, SageMakerError> {
        let owner = user_profile_name.or(space_name).unwrap_or("default");
        let key = format!("{domain_id}/{owner}/{app_type}/{app_name}");
        if self.apps.contains_key(&key) {
            return Err(already_exists("App", app_name));
        }
        let now = Utc::now();
        let arn = format!(
            "arn:aws:sagemaker:{region}:{account_id}:app/{domain_id}/{owner}/{app_type}/{app_name}"
        );
        let app = App {
            domain_id: domain_id.to_string(),
            user_profile_name: user_profile_name.map(|s| s.to_string()),
            space_name: space_name.map(|s| s.to_string()),
            app_type: app_type.to_string(),
            app_name: app_name.to_string(),
            app_arn: arn,
            status: "InService".to_string(),
            creation_time: now,
            tags: Vec::new(),
        };
        self.apps.insert(key.clone(), app);
        Ok(self.apps.get(&key).unwrap())
    }

    pub fn describe_app(
        &self,
        domain_id: &str,
        user_profile_name: Option<&str>,
        space_name: Option<&str>,
        app_type: &str,
        app_name: &str,
    ) -> Result<&App, SageMakerError> {
        let owner = user_profile_name.or(space_name).unwrap_or("default");
        let key = format!("{domain_id}/{owner}/{app_type}/{app_name}");
        self.apps
            .get(&key)
            .ok_or_else(|| resource_not_found("App", app_name))
    }

    pub fn delete_app(
        &mut self,
        domain_id: &str,
        user_profile_name: Option<&str>,
        space_name: Option<&str>,
        app_type: &str,
        app_name: &str,
    ) -> Result<(), SageMakerError> {
        let owner = user_profile_name.or(space_name).unwrap_or("default");
        let key = format!("{domain_id}/{owner}/{app_type}/{app_name}");
        self.apps
            .remove(&key)
            .ok_or_else(|| resource_not_found("App", app_name))?;
        Ok(())
    }

    pub fn list_apps(&self) -> Vec<&App> {
        let mut v: Vec<&App> = self.apps.values().collect();
        v.sort_by(|a, b| a.app_name.cmp(&b.app_name));
        v
    }

    // ============================================================
    // Feature Groups (additional methods)
    // ============================================================

    pub fn delete_feature_group(&mut self, name: &str) -> Result<(), SageMakerError> {
        self.feature_groups
            .remove(name)
            .ok_or_else(|| resource_not_found("FeatureGroup", name))?;
        Ok(())
    }

    pub fn list_feature_groups(&self) -> Vec<&FeatureGroup> {
        let mut v: Vec<&FeatureGroup> = self.feature_groups.values().collect();
        v.sort_by(|a, b| a.feature_group_name.cmp(&b.feature_group_name));
        v
    }

    // ============================================================
    // Model Package (additional methods)
    // ============================================================

    pub fn delete_model_package(&mut self, name: &str) -> Result<(), SageMakerError> {
        self.model_packages
            .remove(name)
            .ok_or_else(|| resource_not_found("ModelPackage", name))?;
        Ok(())
    }

    // ============================================================
    // Model Package Group (additional methods)
    // ============================================================

    pub fn delete_model_package_group(&mut self, name: &str) -> Result<(), SageMakerError> {
        self.model_package_groups
            .remove(name)
            .ok_or_else(|| resource_not_found("ModelPackageGroup", name))?;
        Ok(())
    }

    // ============================================================
    // Training/Processing/Transform Jobs (additional methods)
    // ============================================================

    pub fn stop_training_job(&mut self, name: &str) -> Result<(), SageMakerError> {
        let job = self
            .training_jobs
            .get_mut(name)
            .ok_or_else(|| not_found("TrainingJob", name))?;
        job.training_job_status = "Stopping".to_string();
        job.last_modified_time = Utc::now();
        Ok(())
    }

    pub fn delete_training_job(&mut self, name: &str) -> Result<(), SageMakerError> {
        self.training_jobs
            .remove(name)
            .ok_or_else(|| not_found("TrainingJob", name))?;
        Ok(())
    }

    pub fn stop_processing_job(&mut self, name: &str) -> Result<(), SageMakerError> {
        let job = self
            .processing_jobs
            .get_mut(name)
            .ok_or_else(|| not_found("ProcessingJob", name))?;
        job.processing_job_status = "Stopping".to_string();
        job.last_modified_time = Utc::now();
        Ok(())
    }

    pub fn delete_processing_job(&mut self, name: &str) -> Result<(), SageMakerError> {
        self.processing_jobs
            .remove(name)
            .ok_or_else(|| not_found("ProcessingJob", name))?;
        Ok(())
    }

    pub fn stop_transform_job(&mut self, name: &str) -> Result<(), SageMakerError> {
        let job = self
            .transform_jobs
            .get_mut(name)
            .ok_or_else(|| not_found("TransformJob", name))?;
        job.transform_job_status = "Stopping".to_string();
        job.last_modified_time = Utc::now();
        Ok(())
    }

    // ============================================================
    // Experiments/Trials (additional update methods)
    // ============================================================

    pub fn update_experiment(
        &mut self,
        name: &str,
        display_name: Option<&str>,
        description: Option<&str>,
    ) -> Result<&Experiment, SageMakerError> {
        let exp = self
            .experiments
            .get_mut(name)
            .ok_or_else(|| resource_not_found("Experiment", name))?;
        if let Some(d) = display_name {
            exp.display_name = Some(d.to_string());
        }
        if let Some(desc) = description {
            exp.description = Some(desc.to_string());
        }
        exp.last_modified_time = Utc::now();
        Ok(self.experiments.get(name).unwrap())
    }

    pub fn update_trial(
        &mut self,
        name: &str,
        display_name: Option<&str>,
    ) -> Result<&Trial, SageMakerError> {
        let trial = self
            .trials
            .get_mut(name)
            .ok_or_else(|| resource_not_found("Trial", name))?;
        if let Some(d) = display_name {
            trial.display_name = Some(d.to_string());
        }
        trial.last_modified_time = Utc::now();
        Ok(self.trials.get(name).unwrap())
    }

    // ============================================================
    // Domain (additional update method)
    // ============================================================

    pub fn update_domain(&mut self, domain_id: &str) -> Result<&Domain, SageMakerError> {
        let domain = self
            .domains
            .get_mut(domain_id)
            .ok_or_else(|| resource_not_found("Domain", domain_id))?;
        domain.last_modified_time = Utc::now();
        Ok(self.domains.get(domain_id).unwrap())
    }

    // ============================================================
    // Endpoint (additional update method)
    // ============================================================

    pub fn update_endpoint(
        &mut self,
        name: &str,
        endpoint_config_name: &str,
    ) -> Result<&Endpoint, SageMakerError> {
        let ep = self
            .endpoints
            .get_mut(name)
            .ok_or_else(|| not_found("Endpoint", name))?;
        ep.endpoint_config_name = endpoint_config_name.to_string();
        ep.last_modified_time = Utc::now();
        Ok(self.endpoints.get(name).unwrap())
    }

    // ============================================================
    // Notebook Instance Lifecycle Configs (list method)
    // ============================================================

    pub fn list_notebook_instance_lifecycle_configs(
        &self,
    ) -> Vec<&NotebookInstanceLifecycleConfig> {
        let mut v: Vec<&NotebookInstanceLifecycleConfig> =
            self.notebook_instance_lifecycle_configs.values().collect();
        v.sort_by(|a, b| a.name.cmp(&b.name));
        v
    }

    pub fn update_notebook_instance(
        &mut self,
        name: &str,
        instance_type: Option<&str>,
        role_arn: Option<&str>,
    ) -> Result<(), SageMakerError> {
        let instance = self
            .notebook_instances
            .get_mut(name)
            .ok_or_else(|| not_found("Notebook instance", name))?;
        if let Some(it) = instance_type {
            instance.instance_type = it.to_string();
        }
        if let Some(r) = role_arn {
            instance.role_arn = r.to_string();
        }
        instance.last_modified_time = Utc::now();
        Ok(())
    }

    // ============================================================
    // Tags
    // ============================================================

    pub fn add_tags(&mut self, resource_arn: &str, new_tags: Vec<TagPair>) -> Vec<TagPair> {
        let tags = self.tags.entry(resource_arn.to_string()).or_default();
        for t in &new_tags {
            if let Some(existing) = tags.iter_mut().find(|et| et.key == t.key) {
                existing.value = t.value.clone();
            } else {
                tags.push(t.clone());
            }
        }
        tags.clone()
    }

    pub fn delete_tags(&mut self, resource_arn: &str, tag_keys: &[String]) {
        if let Some(tags) = self.tags.get_mut(resource_arn) {
            tags.retain(|t| !tag_keys.contains(&t.key));
        }
    }

    pub fn list_tags(&self, resource_arn: &str) -> Vec<TagPair> {
        self.tags.get(resource_arn).cloned().unwrap_or_default()
    }

    // ============================================================
    // Search (minimal implementation)
    // ============================================================

    pub fn search(&self, _resource: &str) -> Vec<String> {
        // Minimal: returns empty results. Full search is complex.
        Vec::new()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum JobDefinitionType {
    DataQuality,
    ModelQuality,
    ModelBias,
    ModelExplainability,
}
