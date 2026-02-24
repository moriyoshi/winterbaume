use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

fn now_epoch_secs() -> String {
    let now = chrono::Utc::now();
    format!("{}.{}", now.timestamp(), now.timestamp_subsec_millis())
}

/// In-memory state for the EMR Serverless service.
#[derive(Debug, Default)]
pub struct EmrServerlessState {
    /// Applications keyed by application ID.
    pub applications: HashMap<String, Application>,
    /// Job runs keyed by (application_id, job_run_id).
    pub job_runs: HashMap<(String, String), JobRun>,
    /// Resource tags keyed by resource ARN.
    pub resource_tags: HashMap<String, HashMap<String, String>>,
}

/// Error type for EMR Serverless operations.
#[derive(Debug, Error)]
pub enum EmrServerlessError {
    #[error("Unsupported engine {engine}")]
    UnsupportedEngine { engine: String },

    #[error(
        "Type '{app_type}' is not supported for release label '{release_label}' or release label does not exist"
    )]
    UnsupportedReleaseLabel {
        app_type: String,
        release_label: String,
    },

    #[error("Application {application_id} does not exist")]
    ApplicationNotFound { application_id: String },

    #[error(
        "Application {application_id} must be in one of the following statuses [CREATED, STOPPED]. Current status: {current_status}"
    )]
    InvalidApplicationState {
        application_id: String,
        current_status: String,
    },

    #[error("Job run {job_run_id} does not exist")]
    JobRunNotFound { job_run_id: String },

    #[error("Cross-account pass role is not allowed.")]
    CrossAccountPassRole,

    #[error("RunTimeout must be at least 5 minutes.")]
    InvalidExecutionTimeout,
}

impl EmrServerlessState {
    fn generate_id() -> String {
        use rand::Rng;
        let mut rng = rand::rng();
        let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
        (0..16)
            .map(|_| chars[rng.random_range(0..chars.len())])
            .collect()
    }

    pub fn create_application(
        &mut self,
        name: &str,
        app_type: &str,
        release_label: &str,
        account_id: &str,
        region: &str,
        initial_capacity: Option<HashMap<String, InitialCapacityConfig>>,
        maximum_capacity: Option<MaximumCapacity>,
        network_configuration: Option<NetworkConfiguration>,
        tags: HashMap<String, String>,
    ) -> Result<Application, EmrServerlessError> {
        // Validate type
        if !VALID_TYPES.contains(&app_type) {
            return Err(EmrServerlessError::UnsupportedEngine {
                engine: app_type.to_string(),
            });
        }

        // Validate release label
        if !VALID_RELEASE_LABELS.contains(&release_label) {
            return Err(EmrServerlessError::UnsupportedReleaseLabel {
                app_type: app_type.to_string(),
                release_label: release_label.to_string(),
            });
        }

        let id = Self::generate_id();
        let arn = format!("arn:aws:emr-serverless:{region}:{account_id}:/applications/{id}");

        // Normalize type: SPARK -> Spark, HIVE -> Hive
        let normalized_type = match app_type {
            "SPARK" => "Spark".to_string(),
            "HIVE" => "Hive".to_string(),
            other => other.to_string(),
        };

        let now = now_epoch_secs();

        let app = Application {
            application_id: id.clone(),
            name: name.to_string(),
            arn: arn.clone(),
            release_label: release_label.to_string(),
            application_type: normalized_type,
            state: "STARTED".to_string(),
            state_details: String::new(),
            auto_start_configuration: AutoStartConfig::default(),
            auto_stop_configuration: AutoStopConfig::default(),
            initial_capacity,
            maximum_capacity,
            network_configuration,
            tags,
            created_at: now.clone(),
            updated_at: now,
        };

        self.applications.insert(id, app.clone());
        Ok(app)
    }

    pub fn get_application(
        &self,
        application_id: &str,
    ) -> Result<&Application, EmrServerlessError> {
        self.applications.get(application_id).ok_or_else(|| {
            EmrServerlessError::ApplicationNotFound {
                application_id: application_id.to_string(),
            }
        })
    }

    pub fn delete_application(&mut self, application_id: &str) -> Result<(), EmrServerlessError> {
        let app = self.applications.get(application_id).ok_or_else(|| {
            EmrServerlessError::ApplicationNotFound {
                application_id: application_id.to_string(),
            }
        })?;

        // Only CREATED and STOPPED can be deleted
        let allowed = ["CREATED", "STOPPED"];
        if !allowed.contains(&app.state.as_str()) {
            return Err(EmrServerlessError::InvalidApplicationState {
                application_id: application_id.to_string(),
                current_status: app.state.clone(),
            });
        }

        self.applications.remove(application_id);
        Ok(())
    }

    pub fn update_application(
        &mut self,
        application_id: &str,
        initial_capacity: Option<HashMap<String, InitialCapacityConfig>>,
        maximum_capacity: Option<MaximumCapacity>,
        auto_start_configuration: Option<AutoStartConfig>,
        auto_stop_configuration: Option<AutoStopConfig>,
        network_configuration: Option<NetworkConfiguration>,
    ) -> Result<&Application, EmrServerlessError> {
        // First check if application exists and validate state
        let app = self.applications.get(application_id).ok_or_else(|| {
            EmrServerlessError::ApplicationNotFound {
                application_id: application_id.to_string(),
            }
        })?;

        let allowed = ["CREATED", "STOPPED"];
        if !allowed.contains(&app.state.as_str()) {
            return Err(EmrServerlessError::InvalidApplicationState {
                application_id: application_id.to_string(),
                current_status: app.state.clone(),
            });
        }

        let app = self.applications.get_mut(application_id).unwrap();
        if let Some(ic) = initial_capacity {
            app.initial_capacity = Some(ic);
        }
        if let Some(mc) = maximum_capacity {
            app.maximum_capacity = Some(mc);
        }
        if let Some(asc) = auto_start_configuration {
            app.auto_start_configuration = asc;
        }
        if let Some(asc) = auto_stop_configuration {
            app.auto_stop_configuration = asc;
        }
        if let Some(nc) = network_configuration {
            app.network_configuration = Some(nc);
        }
        app.updated_at = now_epoch_secs();

        Ok(self.applications.get(application_id).unwrap())
    }

    pub fn list_applications(
        &self,
        states: Option<&[String]>,
        max_results: Option<usize>,
        next_token: Option<&str>,
    ) -> (Vec<&Application>, Option<String>) {
        let mut apps: Vec<&Application> = self
            .applications
            .values()
            .filter(|a| match states {
                Some(states) => states.contains(&a.state),
                None => true,
            })
            .collect();
        apps.sort_by(|a, b| a.application_id.cmp(&b.application_id));

        let start = if let Some(token) = next_token {
            token.parse::<usize>().unwrap_or(0)
        } else {
            0
        };

        if start > 0 {
            apps = apps.into_iter().skip(start).collect();
        }

        let token = if let Some(max) = max_results {
            if apps.len() > max {
                apps.truncate(max);
                Some((start + max).to_string())
            } else {
                None
            }
        } else {
            None
        };

        (apps, token)
    }

    pub fn start_application(&mut self, application_id: &str) -> Result<(), EmrServerlessError> {
        let app = self.applications.get_mut(application_id).ok_or_else(|| {
            EmrServerlessError::ApplicationNotFound {
                application_id: application_id.to_string(),
            }
        })?;
        app.state = "STARTED".to_string();
        Ok(())
    }

    pub fn stop_application(&mut self, application_id: &str) -> Result<(), EmrServerlessError> {
        let app = self.applications.get_mut(application_id).ok_or_else(|| {
            EmrServerlessError::ApplicationNotFound {
                application_id: application_id.to_string(),
            }
        })?;
        app.state = "STOPPED".to_string();
        Ok(())
    }

    pub fn start_job_run(
        &mut self,
        application_id: &str,
        name: Option<&str>,
        execution_role_arn: &str,
        job_driver: JobDriver,
        configuration_overrides: Option<ConfigurationOverrides>,
        tags: HashMap<String, String>,
        execution_timeout_minutes: Option<i64>,
        account_id: &str,
        region: &str,
    ) -> Result<JobRun, EmrServerlessError> {
        // Check application exists
        let _app = self.applications.get(application_id).ok_or_else(|| {
            EmrServerlessError::ApplicationNotFound {
                application_id: application_id.to_string(),
            }
        })?;

        // Check cross-account role
        let expected_account_prefix = format!("arn:aws:iam::{account_id}:");
        if !execution_role_arn.starts_with(&expected_account_prefix) {
            return Err(EmrServerlessError::CrossAccountPassRole);
        }

        // Check execution timeout
        if let Some(timeout) = execution_timeout_minutes
            && timeout < 5
        {
            return Err(EmrServerlessError::InvalidExecutionTimeout);
        }

        let job_run_id = Self::generate_id();
        let arn = format!(
            "arn:aws:emr-serverless:{region}:{account_id}:/applications/{application_id}/jobruns/{job_run_id}"
        );
        let now = now_epoch_secs();

        let job_run = JobRun {
            application_id: application_id.to_string(),
            job_run_id: job_run_id.clone(),
            name: name.unwrap_or("").to_string(),
            arn: arn.clone(),
            execution_role_arn: execution_role_arn.to_string(),
            job_driver,
            configuration_overrides,
            tags,
            state: "RUNNING".to_string(),
            state_details: String::new(),
            created_at: now.clone(),
            updated_at: now,
            execution_timeout_minutes,
        };

        let key = (application_id.to_string(), job_run_id.clone());
        self.job_runs.insert(key, job_run.clone());
        Ok(job_run)
    }

    pub fn get_job_run(
        &self,
        application_id: &str,
        job_run_id: &str,
    ) -> Result<&JobRun, EmrServerlessError> {
        // Check application exists
        if !self.applications.contains_key(application_id) {
            return Err(EmrServerlessError::ApplicationNotFound {
                application_id: application_id.to_string(),
            });
        }

        let key = (application_id.to_string(), job_run_id.to_string());
        self.job_runs
            .get(&key)
            .ok_or_else(|| EmrServerlessError::JobRunNotFound {
                job_run_id: job_run_id.to_string(),
            })
    }

    pub fn list_job_runs(
        &self,
        application_id: &str,
        states: Option<&[String]>,
        max_results: Option<usize>,
        next_token: Option<&str>,
    ) -> Result<(Vec<&JobRun>, Option<String>), EmrServerlessError> {
        // Check application exists
        if !self.applications.contains_key(application_id) {
            return Err(EmrServerlessError::ApplicationNotFound {
                application_id: application_id.to_string(),
            });
        }

        let mut runs: Vec<&JobRun> = self
            .job_runs
            .values()
            .filter(|r| r.application_id == application_id)
            .filter(|r| match states {
                Some(states) => states.contains(&r.state),
                None => true,
            })
            .collect();
        runs.sort_by(|a, b| a.job_run_id.cmp(&b.job_run_id));

        let start = if let Some(token) = next_token {
            token.parse::<usize>().unwrap_or(0)
        } else {
            0
        };

        if start > 0 {
            runs = runs.into_iter().skip(start).collect();
        }

        let token = if let Some(max) = max_results {
            if runs.len() > max {
                runs.truncate(max);
                Some((start + max).to_string())
            } else {
                None
            }
        } else {
            None
        };

        Ok((runs, token))
    }

    pub fn cancel_job_run(
        &mut self,
        application_id: &str,
        job_run_id: &str,
    ) -> Result<(), EmrServerlessError> {
        // Check application exists
        if !self.applications.contains_key(application_id) {
            return Err(EmrServerlessError::ApplicationNotFound {
                application_id: application_id.to_string(),
            });
        }

        let key = (application_id.to_string(), job_run_id.to_string());
        let run =
            self.job_runs
                .get_mut(&key)
                .ok_or_else(|| EmrServerlessError::JobRunNotFound {
                    job_run_id: job_run_id.to_string(),
                })?;

        run.state = "CANCELLED".to_string();
        Ok(())
    }

    pub fn list_tags_for_resource(&self, resource_arn: &str) -> HashMap<String, String> {
        self.resource_tags
            .get(resource_arn)
            .cloned()
            .unwrap_or_default()
    }

    pub fn tag_resource(&mut self, resource_arn: &str, tags: HashMap<String, String>) {
        let entry = self
            .resource_tags
            .entry(resource_arn.to_string())
            .or_default();
        entry.extend(tags);
    }

    pub fn untag_resource(&mut self, resource_arn: &str, tag_keys: &[String]) {
        if let Some(tags) = self.resource_tags.get_mut(resource_arn) {
            for key in tag_keys {
                tags.remove(key);
            }
        }
    }
}
