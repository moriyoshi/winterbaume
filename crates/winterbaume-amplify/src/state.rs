use std::collections::HashMap;

use uuid::Uuid;

use crate::types::*;

#[derive(Debug, Default)]
pub struct AmplifyState {
    /// app_id -> App
    pub apps: HashMap<String, AmplifyApp>,
    /// (app_id, branch_name) -> Branch
    pub branches: HashMap<(String, String), AmplifyBranch>,
    /// (app_id, domain_name) -> DomainAssociation
    pub domain_associations: HashMap<(String, String), AmplifyDomainAssociation>,
    /// (app_id, branch_name, job_id) -> Job
    pub jobs: HashMap<(String, String, String), AmplifyJob>,
    /// resource_arn -> tag_key -> tag_value
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, thiserror::Error)]
pub enum AmplifyError {
    #[error("App '{0}' not found.")]
    AppNotFound(String),
    #[error("Branch '{0}' not found in app '{1}'.")]
    BranchNotFound(String, String),
    #[error("Branch '{0}' already exists in app '{1}'.")]
    BranchAlreadyExists(String, String),
    #[error("Domain '{0}' not associated with app '{1}'.")]
    DomainNotFound(String, String),
    #[error("Domain '{0}' already associated with app '{1}'.")]
    DomainAlreadyExists(String, String),
    #[error("Job '{0}' not found.")]
    JobNotFound(String),
}

impl AmplifyState {
    fn now() -> f64 {
        chrono::Utc::now().timestamp() as f64
    }

    // ---- App CRUD ----

    pub fn create_app(
        &mut self,
        name: &str,
        description: Option<String>,
        repository: Option<String>,
        platform: Option<String>,
        iam_service_role_arn: Option<String>,
        environment_variables: HashMap<String, String>,
        enable_branch_auto_build: bool,
        enable_branch_auto_deletion: bool,
        enable_basic_auth: bool,
        build_spec: Option<String>,
        custom_headers: Option<String>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&AmplifyApp, AmplifyError> {
        let app_id = Uuid::new_v4()
            .to_string()
            .replace('-', "")
            .chars()
            .take(12)
            .collect::<String>();
        let app_arn = format!("arn:aws:amplify:{region}:{account_id}:apps/{app_id}");
        let default_domain = format!("{app_id}.amplifyapp.com");
        let now = Self::now();
        let app = AmplifyApp {
            app_id: app_id.clone(),
            app_arn,
            name: name.to_string(),
            description,
            repository,
            platform,
            create_time: now,
            update_time: now,
            iam_service_role_arn,
            environment_variables,
            default_domain,
            enable_branch_auto_build,
            enable_branch_auto_deletion,
            enable_basic_auth,
            build_spec,
            custom_headers,
            tags,
            auto_branch_creation_config: None,
            cache_config: None,
            custom_rules: vec![],
        };
        self.apps.insert(app_id.clone(), app);
        Ok(self.apps.get(&app_id).unwrap())
    }

    pub fn get_app(&self, app_id: &str) -> Result<&AmplifyApp, AmplifyError> {
        self.apps
            .get(app_id)
            .ok_or_else(|| AmplifyError::AppNotFound(app_id.to_string()))
    }

    pub fn list_apps(&self) -> Vec<&AmplifyApp> {
        self.apps.values().collect()
    }

    pub fn update_app(
        &mut self,
        app_id: &str,
        name: Option<String>,
        description: Option<String>,
        repository: Option<String>,
        platform: Option<String>,
        iam_service_role_arn: Option<String>,
        environment_variables: Option<HashMap<String, String>>,
        enable_branch_auto_build: Option<bool>,
        enable_branch_auto_deletion: Option<bool>,
        enable_basic_auth: Option<bool>,
        build_spec: Option<String>,
        custom_headers: Option<String>,
    ) -> Result<&AmplifyApp, AmplifyError> {
        let app = self
            .apps
            .get_mut(app_id)
            .ok_or_else(|| AmplifyError::AppNotFound(app_id.to_string()))?;
        if let Some(v) = name {
            app.name = v;
        }
        if let Some(v) = description {
            app.description = Some(v);
        }
        if let Some(v) = repository {
            app.repository = Some(v);
        }
        if let Some(v) = platform {
            app.platform = Some(v);
        }
        if let Some(v) = iam_service_role_arn {
            app.iam_service_role_arn = Some(v);
        }
        if let Some(v) = environment_variables {
            app.environment_variables = v;
        }
        if let Some(v) = enable_branch_auto_build {
            app.enable_branch_auto_build = v;
        }
        if let Some(v) = enable_branch_auto_deletion {
            app.enable_branch_auto_deletion = v;
        }
        if let Some(v) = enable_basic_auth {
            app.enable_basic_auth = v;
        }
        if let Some(v) = build_spec {
            app.build_spec = Some(v);
        }
        if let Some(v) = custom_headers {
            app.custom_headers = Some(v);
        }
        app.update_time = Self::now();
        Ok(self.apps.get(app_id).unwrap())
    }

    pub fn delete_app(&mut self, app_id: &str) -> Result<AmplifyApp, AmplifyError> {
        self.apps
            .remove(app_id)
            .ok_or_else(|| AmplifyError::AppNotFound(app_id.to_string()))
    }

    // ---- Branch CRUD ----

    pub fn create_branch(
        &mut self,
        app_id: &str,
        branch_name: &str,
        description: Option<String>,
        stage: Option<String>,
        display_name: Option<String>,
        enable_auto_build: bool,
        enable_basic_auth: bool,
        enable_notification: bool,
        enable_performance_mode: bool,
        enable_pull_request_preview: bool,
        environment_variables: HashMap<String, String>,
        framework: Option<String>,
        ttl: Option<String>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&AmplifyBranch, AmplifyError> {
        // Make sure the app exists
        if !self.apps.contains_key(app_id) {
            return Err(AmplifyError::AppNotFound(app_id.to_string()));
        }
        let key = (app_id.to_string(), branch_name.to_string());
        if self.branches.contains_key(&key) {
            return Err(AmplifyError::BranchAlreadyExists(
                branch_name.to_string(),
                app_id.to_string(),
            ));
        }
        let branch_arn =
            format!("arn:aws:amplify:{region}:{account_id}:apps/{app_id}/branches/{branch_name}");
        let now = Self::now();
        let branch = AmplifyBranch {
            branch_arn,
            branch_name: branch_name.to_string(),
            description,
            stage,
            display_name,
            enable_auto_build,
            enable_basic_auth,
            enable_notification,
            enable_performance_mode,
            enable_pull_request_preview,
            environment_variables,
            framework,
            ttl,
            create_time: now,
            update_time: now,
            total_number_of_jobs: "0".to_string(),
            active_job_id: None,
            tags,
        };
        self.branches.insert(key.clone(), branch);
        Ok(self.branches.get(&key).unwrap())
    }

    pub fn get_branch(
        &self,
        app_id: &str,
        branch_name: &str,
    ) -> Result<&AmplifyBranch, AmplifyError> {
        let key = (app_id.to_string(), branch_name.to_string());
        self.branches.get(&key).ok_or_else(|| {
            AmplifyError::BranchNotFound(branch_name.to_string(), app_id.to_string())
        })
    }

    pub fn list_branches(&self, app_id: &str) -> Vec<&AmplifyBranch> {
        self.branches
            .iter()
            .filter_map(|(k, v)| if k.0 == app_id { Some(v) } else { None })
            .collect()
    }

    pub fn update_branch(
        &mut self,
        app_id: &str,
        branch_name: &str,
        description: Option<String>,
        stage: Option<String>,
        framework: Option<String>,
        enable_auto_build: Option<bool>,
        enable_notification: Option<bool>,
        enable_performance_mode: Option<bool>,
        enable_pull_request_preview: Option<bool>,
        environment_variables: Option<HashMap<String, String>>,
        ttl: Option<String>,
    ) -> Result<&AmplifyBranch, AmplifyError> {
        let key = (app_id.to_string(), branch_name.to_string());
        let branch = self.branches.get_mut(&key).ok_or_else(|| {
            AmplifyError::BranchNotFound(branch_name.to_string(), app_id.to_string())
        })?;
        if let Some(v) = description {
            branch.description = Some(v);
        }
        if let Some(v) = stage {
            branch.stage = Some(v);
        }
        if let Some(v) = framework {
            branch.framework = Some(v);
        }
        if let Some(v) = enable_auto_build {
            branch.enable_auto_build = v;
        }
        if let Some(v) = enable_notification {
            branch.enable_notification = v;
        }
        if let Some(v) = enable_performance_mode {
            branch.enable_performance_mode = v;
        }
        if let Some(v) = enable_pull_request_preview {
            branch.enable_pull_request_preview = v;
        }
        if let Some(v) = environment_variables {
            branch.environment_variables = v;
        }
        if let Some(v) = ttl {
            branch.ttl = Some(v);
        }
        branch.update_time = Self::now();
        Ok(self.branches.get(&key).unwrap())
    }

    pub fn delete_branch(
        &mut self,
        app_id: &str,
        branch_name: &str,
    ) -> Result<AmplifyBranch, AmplifyError> {
        let key = (app_id.to_string(), branch_name.to_string());
        self.branches.remove(&key).ok_or_else(|| {
            AmplifyError::BranchNotFound(branch_name.to_string(), app_id.to_string())
        })
    }

    // ---- Domain Association CRUD ----

    pub fn create_domain_association(
        &mut self,
        app_id: &str,
        domain_name: &str,
        enable_auto_sub_domain: bool,
        sub_domains: Vec<SubDomain>,
        account_id: &str,
        region: &str,
    ) -> Result<&AmplifyDomainAssociation, AmplifyError> {
        if !self.apps.contains_key(app_id) {
            return Err(AmplifyError::AppNotFound(app_id.to_string()));
        }
        let key = (app_id.to_string(), domain_name.to_string());
        if self.domain_associations.contains_key(&key) {
            return Err(AmplifyError::DomainAlreadyExists(
                domain_name.to_string(),
                app_id.to_string(),
            ));
        }
        let domain_association_arn =
            format!("arn:aws:amplify:{region}:{account_id}:apps/{app_id}/domains/{domain_name}");
        let domain = AmplifyDomainAssociation {
            domain_association_arn,
            domain_name: domain_name.to_string(),
            enable_auto_sub_domain,
            domain_status: "PENDING_VERIFICATION".to_string(),
            status_reason: String::new(),
            sub_domains,
        };
        self.domain_associations.insert(key.clone(), domain);
        Ok(self.domain_associations.get(&key).unwrap())
    }

    pub fn get_domain_association(
        &self,
        app_id: &str,
        domain_name: &str,
    ) -> Result<&AmplifyDomainAssociation, AmplifyError> {
        let key = (app_id.to_string(), domain_name.to_string());
        self.domain_associations.get(&key).ok_or_else(|| {
            AmplifyError::DomainNotFound(domain_name.to_string(), app_id.to_string())
        })
    }

    pub fn list_domain_associations(&self, app_id: &str) -> Vec<&AmplifyDomainAssociation> {
        self.domain_associations
            .iter()
            .filter_map(|(k, v)| if k.0 == app_id { Some(v) } else { None })
            .collect()
    }

    pub fn delete_domain_association(
        &mut self,
        app_id: &str,
        domain_name: &str,
    ) -> Result<AmplifyDomainAssociation, AmplifyError> {
        let key = (app_id.to_string(), domain_name.to_string());
        self.domain_associations.remove(&key).ok_or_else(|| {
            AmplifyError::DomainNotFound(domain_name.to_string(), app_id.to_string())
        })
    }

    /// Update an existing domain association, optionally replacing
    /// `enable_auto_sub_domain` and/or the `sub_domains` list. `None` arguments
    /// leave the corresponding field untouched.
    pub fn update_domain_association(
        &mut self,
        app_id: &str,
        domain_name: &str,
        enable_auto_sub_domain: Option<bool>,
        sub_domains: Option<Vec<SubDomain>>,
    ) -> Result<&AmplifyDomainAssociation, AmplifyError> {
        let key = (app_id.to_string(), domain_name.to_string());
        let domain = self.domain_associations.get_mut(&key).ok_or_else(|| {
            AmplifyError::DomainNotFound(domain_name.to_string(), app_id.to_string())
        })?;
        if let Some(flag) = enable_auto_sub_domain {
            domain.enable_auto_sub_domain = flag;
        }
        if let Some(subs) = sub_domains {
            domain.sub_domains = subs;
        }
        Ok(self.domain_associations.get(&key).unwrap())
    }

    // ---- Job CRUD ----

    pub fn start_job(
        &mut self,
        app_id: &str,
        branch_name: &str,
        job_type: &str,
        commit_id: Option<String>,
        commit_message: Option<String>,
        commit_time: Option<f64>,
        account_id: &str,
        region: &str,
    ) -> Result<&AmplifyJob, AmplifyError> {
        if !self.apps.contains_key(app_id) {
            return Err(AmplifyError::AppNotFound(app_id.to_string()));
        }
        let branch_key = (app_id.to_string(), branch_name.to_string());
        if !self.branches.contains_key(&branch_key) {
            return Err(AmplifyError::BranchNotFound(
                branch_name.to_string(),
                app_id.to_string(),
            ));
        }
        let job_id = Uuid::new_v4().to_string();
        let job_arn = format!(
            "arn:aws:amplify:{region}:{account_id}:apps/{app_id}/branches/{branch_name}/jobs/{job_id}"
        );
        let now = Self::now();
        let job = AmplifyJob {
            job_id: job_id.clone(),
            job_arn,
            job_type: job_type.to_string(),
            status: "PENDING".to_string(),
            start_time: now,
            end_time: None,
            commit_id,
            commit_message,
            commit_time,
        };
        // Update branch total job count
        if let Some(branch) = self.branches.get_mut(&branch_key) {
            let count: u64 = branch.total_number_of_jobs.parse().unwrap_or(0) + 1;
            branch.total_number_of_jobs = count.to_string();
            branch.active_job_id = Some(job_id.clone());
        }
        let key = (app_id.to_string(), branch_name.to_string(), job_id.clone());
        self.jobs.insert(key.clone(), job);
        Ok(self.jobs.get(&key).unwrap())
    }

    pub fn get_job(
        &self,
        app_id: &str,
        branch_name: &str,
        job_id: &str,
    ) -> Result<&AmplifyJob, AmplifyError> {
        let key = (
            app_id.to_string(),
            branch_name.to_string(),
            job_id.to_string(),
        );
        self.jobs
            .get(&key)
            .ok_or_else(|| AmplifyError::JobNotFound(job_id.to_string()))
    }

    pub fn list_jobs(&self, app_id: &str, branch_name: &str) -> Vec<&AmplifyJob> {
        self.jobs
            .iter()
            .filter_map(|(k, v)| {
                if k.0 == app_id && k.1 == branch_name {
                    Some(v)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn stop_job(
        &mut self,
        app_id: &str,
        branch_name: &str,
        job_id: &str,
    ) -> Result<&AmplifyJob, AmplifyError> {
        let key = (
            app_id.to_string(),
            branch_name.to_string(),
            job_id.to_string(),
        );
        let job = self
            .jobs
            .get_mut(&key)
            .ok_or_else(|| AmplifyError::JobNotFound(job_id.to_string()))?;
        job.status = "CANCELLED".to_string();
        job.end_time = Some(Self::now());
        Ok(self.jobs.get(&key).unwrap())
    }

    // ---- Tag operations ----

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<HashMap<String, String>, AmplifyError> {
        Ok(self.tags.get(resource_arn).cloned().unwrap_or_default())
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), AmplifyError> {
        self.tags
            .entry(resource_arn.to_string())
            .or_default()
            .extend(tags);
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), AmplifyError> {
        if let Some(entry) = self.tags.get_mut(resource_arn) {
            for key in tag_keys {
                entry.remove(key);
            }
        }
        Ok(())
    }
}
