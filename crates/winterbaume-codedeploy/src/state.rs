use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;
use uuid::Uuid;

use crate::types::*;

#[derive(Debug, Default)]
pub struct CodeDeployState {
    pub applications: HashMap<String, Application>,
    /// deployment_group key: (application_name, deployment_group_name)
    pub deployment_groups: HashMap<(String, String), DeploymentGroup>,
    /// deployment_id -> Deployment
    pub deployments: HashMap<String, Deployment>,
    /// ARN -> tag key -> tag value
    pub tags: TagMap,
}

#[derive(Debug, Error)]
pub enum CodeDeployError {
    #[error("Application already exists with the name: {name}")]
    ApplicationAlreadyExists { name: String },

    #[error("No application found for name: {name}")]
    ApplicationDoesNotExist { name: String },

    #[error("Deployment group already exists with the name: {name}")]
    DeploymentGroupAlreadyExists { name: String },

    #[error("No deployment group found for name: {name}")]
    DeploymentGroupDoesNotExist { name: String },

    #[error("No deployment found for id: {id}")]
    DeploymentDoesNotExist { id: String },
}

impl CodeDeployState {
    // ---- Application operations ----

    pub fn create_application(
        &mut self,
        application_name: &str,
        compute_platform: &str,
    ) -> Result<String, CodeDeployError> {
        if self.applications.contains_key(application_name) {
            return Err(CodeDeployError::ApplicationAlreadyExists {
                name: application_name.to_string(),
            });
        }

        let application_id = Uuid::new_v4().to_string();
        let app = Application {
            application_id: application_id.clone(),
            application_name: application_name.to_string(),
            compute_platform: compute_platform.to_string(),
            create_time: Utc::now(),
        };

        self.applications.insert(application_name.to_string(), app);
        Ok(application_id)
    }

    pub fn get_application(&self, application_name: &str) -> Result<&Application, CodeDeployError> {
        self.applications.get(application_name).ok_or_else(|| {
            CodeDeployError::ApplicationDoesNotExist {
                name: application_name.to_string(),
            }
        })
    }

    pub fn delete_application(&mut self, application_name: &str) -> Result<(), CodeDeployError> {
        if self.applications.remove(application_name).is_none() {
            return Err(CodeDeployError::ApplicationDoesNotExist {
                name: application_name.to_string(),
            });
        }
        // Also remove all deployment groups for this application
        self.deployment_groups
            .retain(|(app, _), _| app != application_name);
        // Also remove deployments for this application
        self.deployments
            .retain(|_, d| d.application_name != application_name);
        Ok(())
    }

    pub fn list_applications(&self) -> Vec<&str> {
        self.applications.keys().map(|k| k.as_str()).collect()
    }

    pub fn batch_get_applications(&self, names: &[String]) -> Vec<&Application> {
        names
            .iter()
            .filter_map(|n| self.applications.get(n))
            .collect()
    }

    // ---- DeploymentGroup operations ----

    pub fn create_deployment_group(
        &mut self,
        application_name: &str,
        deployment_group_name: &str,
        service_role_arn: &str,
        deployment_config_name: Option<&str>,
    ) -> Result<String, CodeDeployError> {
        // Application must exist
        let app = self.applications.get(application_name).ok_or_else(|| {
            CodeDeployError::ApplicationDoesNotExist {
                name: application_name.to_string(),
            }
        })?;
        let compute_platform = app.compute_platform.clone();

        let key = (
            application_name.to_string(),
            deployment_group_name.to_string(),
        );
        if self.deployment_groups.contains_key(&key) {
            return Err(CodeDeployError::DeploymentGroupAlreadyExists {
                name: deployment_group_name.to_string(),
            });
        }

        let deployment_group_id = Uuid::new_v4().to_string();
        let dg = DeploymentGroup {
            deployment_group_id: deployment_group_id.clone(),
            deployment_group_name: deployment_group_name.to_string(),
            application_name: application_name.to_string(),
            service_role_arn: service_role_arn.to_string(),
            deployment_config_name: deployment_config_name
                .unwrap_or("CodeDeployDefault.OneAtATime")
                .to_string(),
            compute_platform,
            create_time: Utc::now(),
        };
        self.deployment_groups.insert(key, dg);
        Ok(deployment_group_id)
    }

    pub fn get_deployment_group(
        &self,
        application_name: &str,
        deployment_group_name: &str,
    ) -> Result<&DeploymentGroup, CodeDeployError> {
        // Application must exist
        if !self.applications.contains_key(application_name) {
            return Err(CodeDeployError::ApplicationDoesNotExist {
                name: application_name.to_string(),
            });
        }
        let key = (
            application_name.to_string(),
            deployment_group_name.to_string(),
        );
        self.deployment_groups.get(&key).ok_or_else(|| {
            CodeDeployError::DeploymentGroupDoesNotExist {
                name: deployment_group_name.to_string(),
            }
        })
    }

    pub fn list_deployment_groups(
        &self,
        application_name: &str,
    ) -> Result<Vec<&str>, CodeDeployError> {
        if !self.applications.contains_key(application_name) {
            return Err(CodeDeployError::ApplicationDoesNotExist {
                name: application_name.to_string(),
            });
        }
        Ok(self
            .deployment_groups
            .iter()
            .filter(|((app, _), _)| app == application_name)
            .map(|((_, _), dg)| dg.deployment_group_name.as_str())
            .collect())
    }

    // ---- Deployment operations ----

    pub fn create_deployment(
        &mut self,
        application_name: &str,
        deployment_group_name: &str,
        description: Option<&str>,
        revision_type: Option<&str>,
        revision_s3_bucket: Option<&str>,
        revision_s3_key: Option<&str>,
        revision_s3_bundle_type: Option<&str>,
        revision_github_repository: Option<&str>,
        revision_github_commit_id: Option<&str>,
        file_exists_behavior: Option<&str>,
        ignore_application_stop_failures: bool,
    ) -> Result<String, CodeDeployError> {
        // Application must exist
        if !self.applications.contains_key(application_name) {
            return Err(CodeDeployError::ApplicationDoesNotExist {
                name: application_name.to_string(),
            });
        }

        let key = (
            application_name.to_string(),
            deployment_group_name.to_string(),
        );
        let dg = self.deployment_groups.get(&key).ok_or_else(|| {
            CodeDeployError::DeploymentGroupDoesNotExist {
                name: deployment_group_name.to_string(),
            }
        })?;
        let deployment_config_name = dg.deployment_config_name.clone();

        let deployment_id = format!("d-{}", &Uuid::new_v4().to_string()[..9].to_uppercase());
        let deployment = Deployment {
            deployment_id: deployment_id.clone(),
            application_name: application_name.to_string(),
            deployment_group_name: deployment_group_name.to_string(),
            deployment_config_name,
            description: description.unwrap_or("").to_string(),
            revision_type: revision_type.map(|s| s.to_string()),
            revision_s3_bucket: revision_s3_bucket.map(|s| s.to_string()),
            revision_s3_key: revision_s3_key.map(|s| s.to_string()),
            revision_s3_bundle_type: revision_s3_bundle_type.map(|s| s.to_string()),
            revision_github_repository: revision_github_repository.map(|s| s.to_string()),
            revision_github_commit_id: revision_github_commit_id.map(|s| s.to_string()),
            status: "Created".to_string(),
            create_time: Utc::now(),
            file_exists_behavior: file_exists_behavior.map(|s| s.to_string()),
            ignore_application_stop_failures,
        };
        self.deployments.insert(deployment_id.clone(), deployment);
        Ok(deployment_id)
    }

    pub fn get_deployment(&self, deployment_id: &str) -> Result<&Deployment, CodeDeployError> {
        self.deployments
            .get(deployment_id)
            .ok_or_else(|| CodeDeployError::DeploymentDoesNotExist {
                id: deployment_id.to_string(),
            })
    }

    pub fn batch_get_deployments(&self, ids: &[String]) -> Vec<&Deployment> {
        ids.iter()
            .filter_map(|id| self.deployments.get(id))
            .collect()
    }

    pub fn list_deployments(
        &self,
        application_name: Option<&str>,
        deployment_group_name: Option<&str>,
    ) -> Vec<&str> {
        self.deployments
            .values()
            .filter(|d| {
                if let Some(app) = application_name
                    && d.application_name != app
                {
                    return false;
                }
                if let Some(dg) = deployment_group_name
                    && d.deployment_group_name != dg
                {
                    return false;
                }
                true
            })
            .map(|d| d.deployment_id.as_str())
            .collect()
    }

    // ---- Tag operations ----

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: &[(String, String)],
    ) -> Result<(), CodeDeployError> {
        let entry = self.tags.entry(resource_arn.to_string()).or_default();
        for (k, v) in tags {
            entry.insert(k.clone(), v.clone());
        }
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), CodeDeployError> {
        if let Some(entry) = self.tags.get_mut(resource_arn) {
            for k in tag_keys {
                entry.remove(k);
            }
        }
        Ok(())
    }

    pub fn list_tags_for_resource(&self, resource_arn: &str) -> Vec<(String, String)> {
        self.tags
            .get(resource_arn)
            .map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            .unwrap_or_default()
    }
}
