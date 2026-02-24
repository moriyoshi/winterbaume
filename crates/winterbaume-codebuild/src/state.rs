use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct CodeBuildState {
    pub projects: HashMap<String, Project>,
    /// Builds keyed by build ID.
    pub builds: HashMap<String, Build>,
    /// Ordered list of build IDs (maintains insertion order).
    pub build_ids: Vec<String>,
    /// Next build number per project.
    pub build_counters: HashMap<String, i64>,
    /// Webhooks keyed by project name.
    pub webhooks: HashMap<String, crate::types::Webhook>,
    /// Source credentials keyed by ARN.
    pub source_credentials: HashMap<String, crate::types::SourceCredential>,
    /// Resource policies keyed by resource ARN.
    pub resource_policies: HashMap<String, String>,
    /// Report groups keyed by ARN.
    pub report_groups: HashMap<String, crate::types::ReportGroup>,
    /// Ordered list of report group ARNs (maintains insertion order).
    pub report_group_arns: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum CodeBuildError {
    #[error("Only alphanumeric characters, dash, and underscore are supported")]
    InvalidProjectName,
    #[error("Invalid service role: Service role account ID does not match caller's account")]
    InvalidServiceRole,
    #[error("Invalid build ID provided")]
    InvalidBuildId,
    #[error("Project already exists: arn:aws:codebuild:{region}:{account_id}:project/{name}")]
    ProjectAlreadyExists {
        region: String,
        account_id: String,
        name: String,
    },
    #[error("Project cannot be found: arn:aws:codebuild:{region}:{account_id}:project/{name}")]
    ProjectNotFound {
        region: String,
        account_id: String,
        name: String,
    },
    #[error("Build {build_id} does not exist")]
    BuildNotFound { build_id: String },
    #[error("Project {name} does not exist")]
    ProjectDoesNotExist { name: String },
    #[error("Webhook for project {project_name} already exists")]
    WebhookAlreadyExists { project_name: String },
    #[error("Webhook for project {project_name} does not exist")]
    WebhookNotFound { project_name: String },
    #[error("Source credentials {arn} do not exist")]
    SourceCredentialsNotFound { arn: String },
    #[error("Resource policy for {resource_arn} does not exist")]
    ResourcePolicyNotFound { resource_arn: String },
    #[error("Report group with name {name} already exists")]
    ReportGroupAlreadyExists { name: String },
    #[error("Report group {arn} does not exist")]
    ReportGroupNotFound { arn: String },
}

fn validate_project_name(name: &str) -> Result<(), CodeBuildError> {
    if name.len() >= 150 {
        return Err(CodeBuildError::InvalidProjectName);
    }

    // Must start with a letter, must not end with special characters
    let re = regex::Regex::new(r"^[A-Za-z].*[^!£$%^&*()\+=\|?`¬{}\@~#:;<>\\/\[\]]$").unwrap();
    if !re.is_match(name) {
        return Err(CodeBuildError::InvalidProjectName);
    }

    Ok(())
}

fn validate_service_role(account_id: &str, service_role: &str) -> Result<(), CodeBuildError> {
    let prefix = format!("arn:aws:iam::{account_id}:role/");
    if !service_role.starts_with(&prefix) {
        return Err(CodeBuildError::InvalidServiceRole);
    }
    Ok(())
}

fn validate_build_id(build_id: &str) -> Result<(), CodeBuildError> {
    if !build_id.contains(':') {
        return Err(CodeBuildError::InvalidBuildId);
    }
    Ok(())
}

impl CodeBuildState {
    pub fn create_project(
        &mut self,
        name: &str,
        description: &str,
        source_type: &str,
        source_location: &str,
        artifact_type: &str,
        artifact_location: Option<&str>,
        env_type: &str,
        env_image: &str,
        env_compute: &str,
        service_role: &str,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<&Project, CodeBuildError> {
        validate_project_name(name)?;
        validate_service_role(account_id, service_role)?;

        if self.projects.contains_key(name) {
            return Err(CodeBuildError::ProjectAlreadyExists {
                region: region.to_string(),
                account_id: account_id.to_string(),
                name: name.to_string(),
            });
        }

        let arn = format!("arn:aws:codebuild:{region}:{account_id}:project/{name}");
        let now = Utc::now();

        let project = Project {
            name: name.to_string(),
            arn,
            description: description.to_string(),
            source_type: source_type.to_string(),
            source_location: source_location.to_string(),
            artifact_type: artifact_type.to_string(),
            artifact_location: artifact_location.map(|s| s.to_string()),
            environment_type: env_type.to_string(),
            environment_image: env_image.to_string(),
            environment_compute_type: env_compute.to_string(),
            service_role: service_role.to_string(),
            tags,
            created: now,
            last_modified: now,
        };

        self.projects.insert(name.to_string(), project);
        Ok(self.projects.get(name).unwrap())
    }

    pub fn batch_get_projects(&self, names: &[String]) -> Vec<&Project> {
        names
            .iter()
            .filter_map(|n| {
                // Try by name first
                if let Some(p) = self.projects.get(n.as_str()) {
                    return Some(p);
                }
                // Try by ARN
                if n.starts_with("arn:") {
                    for project in self.projects.values() {
                        if project.arn == *n {
                            return Some(project);
                        }
                    }
                }
                None
            })
            .collect()
    }

    pub fn delete_project(&mut self, name: &str) -> Result<(), CodeBuildError> {
        // moto's delete_project does not raise an error if the project doesn't exist
        self.projects.remove(name);
        // Also remove build history for the project
        self.build_ids.retain(|id| {
            if let Some(build) = self.builds.get(id) {
                build.project_name != name
            } else {
                true
            }
        });
        self.builds.retain(|_, b| b.project_name != name);
        self.build_counters.remove(name);
        Ok(())
    }

    pub fn list_projects(&self) -> Vec<&str> {
        self.projects.keys().map(|s| s.as_str()).collect()
    }

    pub fn start_build(
        &mut self,
        project_name: &str,
        source_version: Option<&str>,
        account_id: &str,
        region: &str,
    ) -> Result<&Build, CodeBuildError> {
        let project = self
            .projects
            .get(project_name)
            .ok_or_else(|| CodeBuildError::ProjectNotFound {
                region: region.to_string(),
                account_id: account_id.to_string(),
                name: project_name.to_string(),
            })?
            .clone();

        let counter = self
            .build_counters
            .entry(project_name.to_string())
            .or_insert(0);
        *counter += 1;
        let build_number = *counter;

        let build_id = format!("{project_name}:{}", uuid::Uuid::new_v4());
        let arn = format!("arn:aws:codebuild:{region}:{account_id}:build/{build_id}");

        let now = Utc::now();
        let now_ts = now.timestamp() as f64;

        let resolved_source_version = source_version.unwrap_or("refs/heads/main").to_string();

        // Initial phases: SUBMITTED (complete) and QUEUED (in progress)
        let phases = vec![
            BuildPhase {
                phase_type: "SUBMITTED".to_string(),
                phase_status: Some("SUCCEEDED".to_string()),
                start_time: now_ts,
                end_time: Some(now_ts),
                duration_in_seconds: Some(0),
            },
            BuildPhase {
                phase_type: "QUEUED".to_string(),
                phase_status: None,
                start_time: now_ts,
                end_time: None,
                duration_in_seconds: None,
            },
        ];

        let build = Build {
            id: build_id.clone(),
            arn,
            project_name: project_name.to_string(),
            build_status: "IN_PROGRESS".to_string(),
            current_phase: "QUEUED".to_string(),
            source_type: project.source_type.clone(),
            source_location: project.source_location.clone(),
            source_version: resolved_source_version,
            artifact_type: project.artifact_type.clone(),
            artifact_location: project.artifact_location.clone(),
            environment_type: project.environment_type.clone(),
            environment_image: project.environment_image.clone(),
            environment_compute_type: project.environment_compute_type.clone(),
            service_role: project.service_role.clone(),
            start_time: now,
            end_time: None,
            build_number,
            phases,
        };

        self.builds.insert(build_id.clone(), build);
        self.build_ids.push(build_id.clone());
        Ok(self.builds.get(&build_id).unwrap())
    }

    pub fn stop_build(&mut self, build_id: &str) -> Result<&Build, CodeBuildError> {
        validate_build_id(build_id)?;

        let build = self
            .builds
            .get_mut(build_id)
            .ok_or_else(|| CodeBuildError::BuildNotFound {
                build_id: build_id.to_string(),
            })?;

        // Set completion phases
        set_completion_phases(&mut build.phases);
        build.build_status = "STOPPED".to_string();
        build.current_phase = "COMPLETED".to_string();
        build.end_time = Some(Utc::now());
        Ok(self.builds.get(build_id).unwrap())
    }

    pub fn batch_get_builds(&self, build_ids: &[String]) -> Result<Vec<Build>, CodeBuildError> {
        // Validate all IDs first
        for id in build_ids {
            validate_build_id(id)?;
        }

        let mut result = Vec::new();
        for id in build_ids {
            if let Some(build) = self.builds.get(id.as_str()) {
                let mut build = build.clone();
                // When retrieving builds, set them to COMPLETED/SUCCEEDED with all phases
                set_completion_phases(&mut build.phases);
                build.current_phase = "COMPLETED".to_string();
                build.build_status = "SUCCEEDED".to_string();
                if build.end_time.is_none() {
                    build.end_time = Some(Utc::now());
                }
                result.push(build);
            }
        }
        Ok(result)
    }

    pub fn list_builds(&self) -> Vec<&str> {
        self.build_ids.iter().map(|s| s.as_str()).collect()
    }

    pub fn list_builds_for_project(&self, project_name: &str) -> Vec<&str> {
        self.build_ids
            .iter()
            .filter(|id| {
                self.builds
                    .get(id.as_str())
                    .map(|b| b.project_name == project_name)
                    .unwrap_or(false)
            })
            .map(|s| s.as_str())
            .collect()
    }

    pub fn batch_delete_builds(&mut self, ids: &[String]) -> Vec<String> {
        let mut deleted = Vec::new();
        for id in ids {
            if self.builds.remove(id.as_str()).is_some() {
                self.build_ids.retain(|bid| bid != id);
                deleted.push(id.clone());
            }
        }
        deleted
    }

    pub fn update_project(
        &mut self,
        name: &str,
        description: Option<&str>,
        source_type: Option<&str>,
        source_location: Option<&str>,
        artifact_type: Option<&str>,
        artifact_location: Option<Option<&str>>,
        env_type: Option<&str>,
        env_image: Option<&str>,
        env_compute: Option<&str>,
        service_role: Option<&str>,
        tags: Option<Vec<crate::types::Tag>>,
        account_id: &str,
    ) -> Result<&Project, CodeBuildError> {
        let project =
            self.projects
                .get_mut(name)
                .ok_or_else(|| CodeBuildError::ProjectDoesNotExist {
                    name: name.to_string(),
                })?;
        if let Some(d) = description {
            project.description = d.to_string();
        }
        if let Some(t) = source_type {
            project.source_type = t.to_string();
        }
        if let Some(l) = source_location {
            project.source_location = l.to_string();
        }
        if let Some(t) = artifact_type {
            project.artifact_type = t.to_string();
        }
        if let Some(l) = artifact_location {
            project.artifact_location = l.map(|s| s.to_string());
        }
        if let Some(t) = env_type {
            project.environment_type = t.to_string();
        }
        if let Some(i) = env_image {
            project.environment_image = i.to_string();
        }
        if let Some(c) = env_compute {
            project.environment_compute_type = c.to_string();
        }
        if let Some(r) = service_role {
            // Validate service role
            let prefix = format!("arn:aws:iam::{account_id}:role/");
            if !r.starts_with(&prefix) {
                return Err(CodeBuildError::InvalidServiceRole);
            }
            project.service_role = r.to_string();
        }
        if let Some(t) = tags {
            project.tags = t;
        }
        project.last_modified = Utc::now();
        Ok(self.projects.get(name).unwrap())
    }

    pub fn retry_build(
        &mut self,
        build_id: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&Build, CodeBuildError> {
        validate_build_id(build_id)?;

        let original = self
            .builds
            .get(build_id)
            .ok_or_else(|| CodeBuildError::BuildNotFound {
                build_id: build_id.to_string(),
            })?
            .clone();

        let project_name = original.project_name.clone();
        let source_version = original.source_version.clone();

        self.start_build(&project_name, Some(&source_version), account_id, region)
    }

    // ── Webhook ──

    pub fn create_webhook(
        &mut self,
        project_name: &str,
        branch_filter: Option<&str>,
        build_type: Option<&str>,
        account_id: &str,
        region: &str,
    ) -> Result<&crate::types::Webhook, CodeBuildError> {
        if !self.projects.contains_key(project_name) {
            return Err(CodeBuildError::ProjectNotFound {
                region: region.to_string(),
                account_id: account_id.to_string(),
                name: project_name.to_string(),
            });
        }
        if self.webhooks.contains_key(project_name) {
            return Err(CodeBuildError::WebhookAlreadyExists {
                project_name: project_name.to_string(),
            });
        }
        let webhook = crate::types::Webhook {
            project_name: project_name.to_string(),
            url: format!("https://codebuild.{region}.amazonaws.com/webhooks/{project_name}"),
            branch_filter: branch_filter.map(|s| s.to_string()),
            build_type: build_type.map(|s| s.to_string()),
            secret: Some(uuid::Uuid::new_v4().to_string()),
        };
        self.webhooks.insert(project_name.to_string(), webhook);
        Ok(self.webhooks.get(project_name).unwrap())
    }

    pub fn update_webhook(
        &mut self,
        project_name: &str,
        branch_filter: Option<&str>,
        build_type: Option<&str>,
    ) -> Result<&crate::types::Webhook, CodeBuildError> {
        let webhook =
            self.webhooks
                .get_mut(project_name)
                .ok_or_else(|| CodeBuildError::WebhookNotFound {
                    project_name: project_name.to_string(),
                })?;
        if let Some(bf) = branch_filter {
            webhook.branch_filter = Some(bf.to_string());
        }
        if let Some(bt) = build_type {
            webhook.build_type = Some(bt.to_string());
        }
        Ok(self.webhooks.get(project_name).unwrap())
    }

    pub fn delete_webhook(&mut self, project_name: &str) -> Result<(), CodeBuildError> {
        match self.webhooks.remove(project_name) {
            Some(_) => Ok(()),
            None => Err(CodeBuildError::WebhookNotFound {
                project_name: project_name.to_string(),
            }),
        }
    }

    // ── Source Credentials ──

    pub fn import_source_credentials(
        &mut self,
        token: &str,
        server_type: &str,
        auth_type: &str,
        username: Option<&str>,
        account_id: &str,
        region: &str,
    ) -> Result<&crate::types::SourceCredential, CodeBuildError> {
        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:codebuild:{region}:{account_id}:token/{server_type}/{id}");
        let _ = token; // token is not stored in mock (security)
        let cred = crate::types::SourceCredential {
            arn: arn.clone(),
            server_type: server_type.to_string(),
            auth_type: auth_type.to_string(),
            resource: username.map(|s| s.to_string()),
        };
        self.source_credentials.insert(arn.clone(), cred);
        Ok(self.source_credentials.get(&arn).unwrap())
    }

    pub fn list_source_credentials(&self) -> Vec<&crate::types::SourceCredential> {
        self.source_credentials.values().collect()
    }

    pub fn delete_source_credentials(&mut self, arn: &str) -> Result<(), CodeBuildError> {
        match self.source_credentials.remove(arn) {
            Some(_) => Ok(()),
            None => Err(CodeBuildError::SourceCredentialsNotFound {
                arn: arn.to_string(),
            }),
        }
    }

    // ── Resource Policies ──

    pub fn put_resource_policy(
        &mut self,
        resource_arn: &str,
        policy: &str,
    ) -> Result<String, CodeBuildError> {
        self.resource_policies
            .insert(resource_arn.to_string(), policy.to_string());
        Ok(resource_arn.to_string())
    }

    pub fn get_resource_policy(&self, resource_arn: &str) -> Result<String, CodeBuildError> {
        self.resource_policies
            .get(resource_arn)
            .cloned()
            .ok_or_else(|| CodeBuildError::ResourcePolicyNotFound {
                resource_arn: resource_arn.to_string(),
            })
    }

    pub fn delete_resource_policy(&mut self, resource_arn: &str) -> Result<(), CodeBuildError> {
        self.resource_policies.remove(resource_arn);
        Ok(())
    }

    // ── Report Groups ──

    pub fn create_report_group(
        &mut self,
        name: &str,
        report_type: &str,
        export_config_type: Option<&str>,
        tags: Vec<crate::types::Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<&crate::types::ReportGroup, CodeBuildError> {
        // Check for duplicate name
        for rg in self.report_groups.values() {
            if rg.name == name {
                return Err(CodeBuildError::ReportGroupAlreadyExists {
                    name: name.to_string(),
                });
            }
        }

        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:codebuild:{region}:{account_id}:report-group/{name}-{id}");
        let now = Utc::now();

        let rg = crate::types::ReportGroup {
            arn: arn.clone(),
            name: name.to_string(),
            r#type: report_type.to_string(),
            export_config_type: export_config_type.map(|s| s.to_string()),
            tags,
            created: now,
            last_modified: now,
            status: "ACTIVE".to_string(),
        };

        self.report_groups.insert(arn.clone(), rg);
        self.report_group_arns.push(arn.clone());
        Ok(self.report_groups.get(&arn).unwrap())
    }

    pub fn batch_get_report_groups(&self, arns: &[String]) -> Vec<&crate::types::ReportGroup> {
        arns.iter()
            .filter_map(|arn| self.report_groups.get(arn.as_str()))
            .collect()
    }

    pub fn list_report_groups(&self) -> Vec<&str> {
        self.report_group_arns.iter().map(|s| s.as_str()).collect()
    }

    pub fn delete_report_group(&mut self, arn: &str) -> Result<(), CodeBuildError> {
        match self.report_groups.remove(arn) {
            Some(_) => {
                self.report_group_arns.retain(|a| a != arn);
                Ok(())
            }
            None => Err(CodeBuildError::ReportGroupNotFound {
                arn: arn.to_string(),
            }),
        }
    }

    pub fn update_report_group(
        &mut self,
        arn: &str,
        export_config_type: Option<&str>,
        tags: Option<Vec<crate::types::Tag>>,
    ) -> Result<&crate::types::ReportGroup, CodeBuildError> {
        let rg =
            self.report_groups
                .get_mut(arn)
                .ok_or_else(|| CodeBuildError::ReportGroupNotFound {
                    arn: arn.to_string(),
                })?;

        if let Some(ect) = export_config_type {
            rg.export_config_type = Some(ect.to_string());
        }
        if let Some(t) = tags {
            rg.tags = t;
        }
        rg.last_modified = Utc::now();
        Ok(self.report_groups.get(arn).unwrap())
    }
}

fn set_completion_phases(phases: &mut Vec<BuildPhase>) {
    let now_ts = Utc::now().timestamp() as f64;

    // Set QUEUED phase status to SUCCEEDED
    for phase in phases.iter_mut() {
        if phase.phase_type == "QUEUED" && phase.phase_status.is_none() {
            phase.phase_status = Some("SUCCEEDED".to_string());
        }
    }

    let additional_phases = [
        "PROVISIONING",
        "DOWNLOAD_SOURCE",
        "INSTALL",
        "PRE_BUILD",
        "BUILD",
        "POST_BUILD",
        "UPLOAD_ARTIFACTS",
        "FINALIZING",
        "COMPLETED",
    ];

    for phase_type in &additional_phases {
        // Only add if not already present
        if !phases.iter().any(|p| p.phase_type == *phase_type) {
            phases.push(BuildPhase {
                phase_type: phase_type.to_string(),
                phase_status: Some("SUCCEEDED".to_string()),
                start_time: now_ts,
                end_time: Some(now_ts),
                duration_in_seconds: Some(0),
            });
        }
    }
}
