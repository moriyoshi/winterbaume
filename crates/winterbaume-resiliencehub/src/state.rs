use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct ResilienceHubState {
    pub apps: HashMap<String, App>,
    pub policies: HashMap<String, ResiliencyPolicyData>,
    /// Tags stored by resource ARN (covers apps, policies, etc.)
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Error)]
pub enum ResilienceHubError {
    #[error("App with name '{name}' already exists.")]
    AppConflict { name: String },

    #[error("App not found: {app_arn}")]
    AppNotFound { app_arn: String },

    #[error("Policy not found: {policy_arn}")]
    PolicyNotFound { policy_arn: String },

    #[error("Resource not found: {resource_arn}")]
    ResourceNotFound { resource_arn: String },
}

impl ResilienceHubState {
    pub fn create_app(
        &mut self,
        name: &str,
        description: &str,
        policy_arn: &str,
        assessment_schedule: &str,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&App, ResilienceHubError> {
        // Check for duplicate name
        if self.apps.values().any(|a| a.name == name) {
            return Err(ResilienceHubError::AppConflict {
                name: name.to_string(),
            });
        }

        let id = uuid::Uuid::new_v4().to_string();
        let app_arn = format!("arn:aws:resiliencehub:{region}:{account_id}:app/{id}");

        let app = App {
            app_arn: app_arn.clone(),
            name: name.to_string(),
            description: description.to_string(),
            status: "Active".to_string(),
            creation_time: Utc::now(),
            compliance_status: "NotAssessed".to_string(),
            policy_arn: policy_arn.to_string(),
            assessment_schedule: assessment_schedule.to_string(),
            tags: tags.clone(),
            versions: Vec::new(),
            next_version_id: 1,
            resources: Vec::new(),
            app_template_body: "{}".to_string(),
            app_components: Vec::new(),
            assessments: Vec::new(),
        };

        if !tags.is_empty() {
            self.tags.insert(app_arn.clone(), tags);
        }

        self.apps.insert(app_arn.clone(), app);
        Ok(self.apps.get(&app_arn).unwrap())
    }

    pub fn describe_app(&self, app_arn: &str) -> Result<&App, ResilienceHubError> {
        self.apps
            .get(app_arn)
            .ok_or_else(|| ResilienceHubError::AppNotFound {
                app_arn: app_arn.to_string(),
            })
    }

    pub fn delete_app(&mut self, app_arn: &str) -> Result<String, ResilienceHubError> {
        if self.apps.remove(app_arn).is_none() {
            return Err(ResilienceHubError::AppNotFound {
                app_arn: app_arn.to_string(),
            });
        }
        self.tags.remove(app_arn);
        Ok(app_arn.to_string())
    }

    pub fn list_apps(
        &self,
        name_filter: Option<&str>,
        app_arn_filter: Option<&str>,
    ) -> Vec<AppSummary> {
        let mut summaries: Vec<AppSummary> = self
            .apps
            .values()
            .filter(|app| {
                if let Some(name) = name_filter {
                    if app.name != name {
                        return false;
                    }
                }
                if let Some(arn) = app_arn_filter {
                    if app.app_arn != arn {
                        return false;
                    }
                }
                true
            })
            .map(|app| AppSummary {
                app_arn: app.app_arn.clone(),
                name: app.name.clone(),
                description: app.description.clone(),
                status: app.status.clone(),
                creation_time: app.creation_time,
                compliance_status: app.compliance_status.clone(),
                assessment_schedule: app.assessment_schedule.clone(),
            })
            .collect();
        // Sort by creation time for stable ordering
        summaries.sort_by_key(|a| a.creation_time);
        summaries
    }

    // --- Resiliency Policy operations ---

    pub fn create_resiliency_policy(
        &mut self,
        policy_name: &str,
        policy_description: &str,
        data_location_constraint: &str,
        tier: &str,
        policy: HashMap<String, FailurePolicyData>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&ResiliencyPolicyData, ResilienceHubError> {
        let id = uuid::Uuid::new_v4().to_string();
        let policy_arn =
            format!("arn:aws:resiliencehub:{region}:{account_id}:resiliency-policy/{id}");

        let data = ResiliencyPolicyData {
            policy_arn: policy_arn.clone(),
            policy_name: policy_name.to_string(),
            policy_description: policy_description.to_string(),
            data_location_constraint: data_location_constraint.to_string(),
            tier: tier.to_string(),
            policy,
            creation_time: Utc::now(),
            tags: tags.clone(),
        };

        if !tags.is_empty() {
            self.tags.insert(policy_arn.clone(), tags);
        }

        self.policies.insert(policy_arn.clone(), data);
        Ok(self.policies.get(&policy_arn).unwrap())
    }

    pub fn describe_resiliency_policy(
        &self,
        policy_arn: &str,
    ) -> Result<&ResiliencyPolicyData, ResilienceHubError> {
        self.policies
            .get(policy_arn)
            .ok_or_else(|| ResilienceHubError::PolicyNotFound {
                policy_arn: policy_arn.to_string(),
            })
    }

    pub fn delete_resiliency_policy(
        &mut self,
        policy_arn: &str,
    ) -> Result<String, ResilienceHubError> {
        if self.policies.remove(policy_arn).is_none() {
            return Err(ResilienceHubError::PolicyNotFound {
                policy_arn: policy_arn.to_string(),
            });
        }
        self.tags.remove(policy_arn);
        Ok(policy_arn.to_string())
    }

    pub fn update_resiliency_policy(
        &mut self,
        policy_arn: &str,
        policy_name: Option<&str>,
        policy_description: Option<&str>,
        data_location_constraint: Option<&str>,
        tier: Option<&str>,
        policy: Option<HashMap<String, FailurePolicyData>>,
    ) -> Result<&ResiliencyPolicyData, ResilienceHubError> {
        let data = self.policies.get_mut(policy_arn).ok_or_else(|| {
            ResilienceHubError::PolicyNotFound {
                policy_arn: policy_arn.to_string(),
            }
        })?;

        if let Some(n) = policy_name {
            data.policy_name = n.to_string();
        }
        if let Some(d) = policy_description {
            data.policy_description = d.to_string();
        }
        if let Some(c) = data_location_constraint {
            data.data_location_constraint = c.to_string();
        }
        if let Some(t) = tier {
            data.tier = t.to_string();
        }
        if let Some(p) = policy {
            data.policy = p;
        }

        Ok(self.policies.get(policy_arn).unwrap())
    }

    pub fn list_resiliency_policies(&self) -> Vec<&ResiliencyPolicyData> {
        self.policies.values().collect()
    }

    pub fn list_suggested_resiliency_policies(&self) -> Vec<ResiliencyPolicyData> {
        // Return a static set of suggested policies (like moto does)
        Vec::new()
    }

    // --- Tag operations ---

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), ResilienceHubError> {
        // Verify resource exists
        if !self.apps.contains_key(resource_arn) && !self.policies.contains_key(resource_arn) {
            return Err(ResilienceHubError::ResourceNotFound {
                resource_arn: resource_arn.to_string(),
            });
        }
        let entry = self.tags.entry(resource_arn.to_string()).or_default();
        for (k, v) in tags {
            entry.insert(k, v);
        }
        // Also update tags on the resource itself (for policies)
        if let Some(policy) = self.policies.get_mut(resource_arn) {
            policy.tags = entry.clone();
        }
        if let Some(app) = self.apps.get_mut(resource_arn) {
            app.tags = entry.clone();
        }
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), ResilienceHubError> {
        if !self.apps.contains_key(resource_arn) && !self.policies.contains_key(resource_arn) {
            return Err(ResilienceHubError::ResourceNotFound {
                resource_arn: resource_arn.to_string(),
            });
        }
        if let Some(entry) = self.tags.get_mut(resource_arn) {
            for key in tag_keys {
                entry.remove(key);
            }
            // Update inline tags
            if let Some(policy) = self.policies.get_mut(resource_arn) {
                policy.tags = entry.clone();
            }
            if let Some(app) = self.apps.get_mut(resource_arn) {
                app.tags = entry.clone();
            }
        }
        Ok(())
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<HashMap<String, String>, ResilienceHubError> {
        if !self.apps.contains_key(resource_arn) && !self.policies.contains_key(resource_arn) {
            return Err(ResilienceHubError::ResourceNotFound {
                resource_arn: resource_arn.to_string(),
            });
        }
        Ok(self.tags.get(resource_arn).cloned().unwrap_or_default())
    }

    // --- App version operations ---

    pub fn publish_app_version(
        &mut self,
        app_arn: &str,
        version_name: Option<&str>,
    ) -> Result<(String, String, i64), ResilienceHubError> {
        let app = self
            .apps
            .get_mut(app_arn)
            .ok_or_else(|| ResilienceHubError::AppNotFound {
                app_arn: app_arn.to_string(),
            })?;

        let id = app.next_version_id;
        app.next_version_id += 1;
        let ver_str = version_name.unwrap_or("release").to_string();

        let version = AppVersion {
            app_version: ver_str.clone(),
            identifier: id,
            creation_time: Utc::now(),
        };
        app.versions.push(version);

        Ok((app_arn.to_string(), ver_str, id))
    }

    pub fn list_app_versions(&self, app_arn: &str) -> Result<Vec<&AppVersion>, ResilienceHubError> {
        let app = self
            .apps
            .get(app_arn)
            .ok_or_else(|| ResilienceHubError::AppNotFound {
                app_arn: app_arn.to_string(),
            })?;

        // Always include draft version
        Ok(app.versions.iter().collect())
    }

    pub fn describe_app_version_template(
        &self,
        app_arn: &str,
        app_version: &str,
    ) -> Result<(String, String, String), ResilienceHubError> {
        let app = self
            .apps
            .get(app_arn)
            .ok_or_else(|| ResilienceHubError::AppNotFound {
                app_arn: app_arn.to_string(),
            })?;

        Ok((
            app_arn.to_string(),
            app_version.to_string(),
            app.app_template_body.clone(),
        ))
    }

    // --- App version resource operations ---

    pub fn create_app_version_resource(
        &mut self,
        app_arn: &str,
        resource_name: &str,
        logical_resource_id: &str,
        physical_resource_id: &str,
        resource_type: &str,
        app_components: Vec<String>,
        aws_region: &str,
        aws_account_id: &str,
    ) -> Result<&AppVersionResource, ResilienceHubError> {
        if !self.apps.contains_key(app_arn) {
            return Err(ResilienceHubError::AppNotFound {
                app_arn: app_arn.to_string(),
            });
        }

        let resource = AppVersionResource {
            app_arn: app_arn.to_string(),
            resource_name: resource_name.to_string(),
            logical_resource_id: logical_resource_id.to_string(),
            physical_resource_id: physical_resource_id.to_string(),
            resource_type: resource_type.to_string(),
            app_components,
            aws_region: aws_region.to_string(),
            aws_account_id: aws_account_id.to_string(),
        };

        let app = self.apps.get_mut(app_arn).unwrap();
        app.resources.push(resource);
        Ok(app.resources.last().unwrap())
    }

    pub fn list_app_version_resources(
        &self,
        app_arn: &str,
        _app_version: &str,
    ) -> Result<Vec<&AppVersionResource>, ResilienceHubError> {
        let app = self
            .apps
            .get(app_arn)
            .ok_or_else(|| ResilienceHubError::AppNotFound {
                app_arn: app_arn.to_string(),
            })?;

        Ok(app.resources.iter().collect())
    }

    // --- App version app component operations ---

    pub fn create_app_version_app_component(
        &mut self,
        app_arn: &str,
        name: &str,
        component_type: &str,
        account_id: &str,
    ) -> Result<AppComponentData, ResilienceHubError> {
        let app = self
            .apps
            .get_mut(app_arn)
            .ok_or_else(|| ResilienceHubError::AppNotFound {
                app_arn: app_arn.to_string(),
            })?;

        let id = format!("{account_id}-{name}");
        let comp = AppComponentData {
            id: id.clone(),
            name: name.to_string(),
            component_type: component_type.to_string(),
        };
        app.app_components.push(comp.clone());
        Ok(comp)
    }

    pub fn list_app_version_app_components(
        &self,
        app_arn: &str,
        _app_version: &str,
    ) -> Result<Vec<&AppComponentData>, ResilienceHubError> {
        let app = self
            .apps
            .get(app_arn)
            .ok_or_else(|| ResilienceHubError::AppNotFound {
                app_arn: app_arn.to_string(),
            })?;

        Ok(app.app_components.iter().collect())
    }

    // --- Import resources ---

    pub fn import_resources_to_draft_app_version(
        &mut self,
        app_arn: &str,
        source_arns: Vec<String>,
    ) -> Result<(String, String, Vec<String>), ResilienceHubError> {
        if !self.apps.contains_key(app_arn) {
            return Err(ResilienceHubError::AppNotFound {
                app_arn: app_arn.to_string(),
            });
        }
        Ok((app_arn.to_string(), "draft".to_string(), source_arns))
    }

    // --- List app assessments ---

    pub fn list_app_assessments(&self, app_arn_filter: Option<&str>) -> Vec<&AssessmentData> {
        self.apps
            .values()
            .filter(|app| {
                if let Some(arn) = app_arn_filter {
                    app.app_arn == arn
                } else {
                    true
                }
            })
            .flat_map(|app| app.assessments.iter())
            .collect()
    }
}
