use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct FisState {
    pub experiment_templates: HashMap<String, ExperimentTemplate>,
}

/// Domain-specific error enum. Contains no HTTP status codes or AWS error type strings —
/// those are mapped in the handler's error-shaping function.
#[derive(Debug, Error)]
pub enum FisError {
    #[error("ExperimentTemplate '{id}' not found.")]
    ExperimentTemplateNotFound { id: String },

    #[error("Resource '{arn}' not found.")]
    ResourceNotFound { arn: String },

    #[error("{message}")]
    Validation { message: String },
}

impl FisState {
    // -------------------------------------------------------------------------
    // Experiment Templates
    // -------------------------------------------------------------------------

    pub fn create_experiment_template(
        &mut self,
        description: &str,
        role_arn: &str,
        targets: HashMap<String, ExperimentTemplateTarget>,
        actions: HashMap<String, ExperimentTemplateAction>,
        stop_conditions: Vec<ExperimentTemplateStopCondition>,
        tags: HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> Result<&ExperimentTemplate, FisError> {
        let id = format!(
            "EXT{}",
            &uuid::Uuid::new_v4()
                .to_string()
                .replace('-', "")
                .chars()
                .take(16)
                .collect::<String>()
        );
        let arn = format!("arn:aws:fis:{region}:{account_id}:experiment-template/{id}");
        let now = Utc::now();

        let template = ExperimentTemplate {
            id: id.clone(),
            arn,
            description: description.to_string(),
            role_arn: role_arn.to_string(),
            targets,
            actions,
            stop_conditions,
            tags,
            creation_time: now,
            last_update_time: now,
        };

        self.experiment_templates.insert(id.clone(), template);
        Ok(self.experiment_templates.get(&id).unwrap())
    }

    pub fn get_experiment_template(&self, id: &str) -> Result<&ExperimentTemplate, FisError> {
        self.experiment_templates
            .get(id)
            .ok_or_else(|| FisError::ExperimentTemplateNotFound { id: id.to_string() })
    }

    pub fn delete_experiment_template(&mut self, id: &str) -> Result<ExperimentTemplate, FisError> {
        self.experiment_templates
            .remove(id)
            .ok_or_else(|| FisError::ExperimentTemplateNotFound { id: id.to_string() })
    }

    pub fn update_experiment_template(
        &mut self,
        id: &str,
        description: Option<&str>,
        role_arn: Option<&str>,
        targets: Option<HashMap<String, ExperimentTemplateTarget>>,
        actions: Option<HashMap<String, ExperimentTemplateAction>>,
        stop_conditions: Option<Vec<ExperimentTemplateStopCondition>>,
    ) -> Result<&ExperimentTemplate, FisError> {
        let template = self
            .experiment_templates
            .get_mut(id)
            .ok_or_else(|| FisError::ExperimentTemplateNotFound { id: id.to_string() })?;

        if let Some(d) = description {
            template.description = d.to_string();
        }
        if let Some(r) = role_arn {
            template.role_arn = r.to_string();
        }
        if let Some(t) = targets {
            template.targets = t;
        }
        if let Some(a) = actions {
            template.actions = a;
        }
        if let Some(sc) = stop_conditions {
            template.stop_conditions = sc;
        }
        template.last_update_time = Utc::now();

        Ok(self.experiment_templates.get(id).unwrap())
    }

    pub fn list_experiment_templates(&self) -> Vec<&ExperimentTemplate> {
        self.experiment_templates.values().collect()
    }

    // -------------------------------------------------------------------------
    // Tags
    // -------------------------------------------------------------------------

    pub fn get_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<HashMap<String, String>, FisError> {
        for template in self.experiment_templates.values() {
            if template.arn == resource_arn {
                return Ok(template.tags.clone());
            }
        }
        Err(FisError::ResourceNotFound {
            arn: resource_arn.to_string(),
        })
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), FisError> {
        for template in self.experiment_templates.values_mut() {
            if template.arn == resource_arn {
                template.tags.extend(tags);
                return Ok(());
            }
        }
        Err(FisError::ResourceNotFound {
            arn: resource_arn.to_string(),
        })
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), FisError> {
        for template in self.experiment_templates.values_mut() {
            if template.arn == resource_arn {
                for k in tag_keys {
                    template.tags.remove(k);
                }
                return Ok(());
            }
        }
        Err(FisError::ResourceNotFound {
            arn: resource_arn.to_string(),
        })
    }
}
