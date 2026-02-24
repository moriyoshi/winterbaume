use std::collections::HashMap;

use thiserror::Error;

use crate::model;

/// State key tuple: (appId, environmentName, id).
type Key = (String, String, String);

#[derive(Debug, Default)]
pub struct AmplifyUiBuilderState {
    pub components: HashMap<Key, model::Component>,
    pub forms: HashMap<Key, model::Form>,
    pub themes: HashMap<Key, model::Theme>,
    pub codegen_jobs: HashMap<Key, model::CodegenJob>,
    /// Per-(appId, environmentName) feature flags.
    pub metadata: HashMap<(String, String), HashMap<String, String>>,
    /// Per-ARN tags (used by TagResource / UntagResource / ListTagsForResource).
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Error)]
pub enum AmplifyUiBuilderError {
    #[error("{resource_type} {id} does not exist.")]
    NotFound {
        resource_type: &'static str,
        id: String,
    },

    #[error("{message}")]
    Validation { message: String },
}

impl AmplifyUiBuilderState {
    pub fn create_component(
        &mut self,
        app_id: &str,
        env: &str,
        component: model::Component,
    ) -> &model::Component {
        let id = component.id.clone().unwrap_or_default();
        let key = (app_id.to_string(), env.to_string(), id);
        self.components.insert(key.clone(), component);
        self.components.get(&key).unwrap()
    }

    pub fn get_component(
        &self,
        app_id: &str,
        env: &str,
        id: &str,
    ) -> Result<&model::Component, AmplifyUiBuilderError> {
        self.components
            .get(&(app_id.to_string(), env.to_string(), id.to_string()))
            .ok_or(AmplifyUiBuilderError::NotFound {
                resource_type: "Component",
                id: id.to_string(),
            })
    }

    pub fn delete_component(
        &mut self,
        app_id: &str,
        env: &str,
        id: &str,
    ) -> Result<(), AmplifyUiBuilderError> {
        self.components
            .remove(&(app_id.to_string(), env.to_string(), id.to_string()))
            .ok_or(AmplifyUiBuilderError::NotFound {
                resource_type: "Component",
                id: id.to_string(),
            })?;
        Ok(())
    }

    pub fn list_components(&self, app_id: &str, env: &str) -> Vec<&model::Component> {
        self.components
            .iter()
            .filter(|((a, e, _), _)| a == app_id && e == env)
            .map(|(_, v)| v)
            .collect()
    }

    pub fn create_form(&mut self, app_id: &str, env: &str, form: model::Form) -> &model::Form {
        let id = form.id.clone().unwrap_or_default();
        let key = (app_id.to_string(), env.to_string(), id);
        self.forms.insert(key.clone(), form);
        self.forms.get(&key).unwrap()
    }

    pub fn get_form(
        &self,
        app_id: &str,
        env: &str,
        id: &str,
    ) -> Result<&model::Form, AmplifyUiBuilderError> {
        self.forms
            .get(&(app_id.to_string(), env.to_string(), id.to_string()))
            .ok_or(AmplifyUiBuilderError::NotFound {
                resource_type: "Form",
                id: id.to_string(),
            })
    }

    pub fn delete_form(
        &mut self,
        app_id: &str,
        env: &str,
        id: &str,
    ) -> Result<(), AmplifyUiBuilderError> {
        self.forms
            .remove(&(app_id.to_string(), env.to_string(), id.to_string()))
            .ok_or(AmplifyUiBuilderError::NotFound {
                resource_type: "Form",
                id: id.to_string(),
            })?;
        Ok(())
    }

    pub fn list_forms(&self, app_id: &str, env: &str) -> Vec<&model::Form> {
        self.forms
            .iter()
            .filter(|((a, e, _), _)| a == app_id && e == env)
            .map(|(_, v)| v)
            .collect()
    }

    pub fn create_theme(&mut self, app_id: &str, env: &str, theme: model::Theme) -> &model::Theme {
        let id = theme.id.clone().unwrap_or_default();
        let key = (app_id.to_string(), env.to_string(), id);
        self.themes.insert(key.clone(), theme);
        self.themes.get(&key).unwrap()
    }

    pub fn get_theme(
        &self,
        app_id: &str,
        env: &str,
        id: &str,
    ) -> Result<&model::Theme, AmplifyUiBuilderError> {
        self.themes
            .get(&(app_id.to_string(), env.to_string(), id.to_string()))
            .ok_or(AmplifyUiBuilderError::NotFound {
                resource_type: "Theme",
                id: id.to_string(),
            })
    }

    pub fn delete_theme(
        &mut self,
        app_id: &str,
        env: &str,
        id: &str,
    ) -> Result<(), AmplifyUiBuilderError> {
        self.themes
            .remove(&(app_id.to_string(), env.to_string(), id.to_string()))
            .ok_or(AmplifyUiBuilderError::NotFound {
                resource_type: "Theme",
                id: id.to_string(),
            })?;
        Ok(())
    }

    pub fn list_themes(&self, app_id: &str, env: &str) -> Vec<&model::Theme> {
        self.themes
            .iter()
            .filter(|((a, e, _), _)| a == app_id && e == env)
            .map(|(_, v)| v)
            .collect()
    }

    pub fn add_codegen_job(
        &mut self,
        app_id: &str,
        env: &str,
        job: model::CodegenJob,
    ) -> &model::CodegenJob {
        let id = job.id.clone().unwrap_or_default();
        let key = (app_id.to_string(), env.to_string(), id);
        self.codegen_jobs.insert(key.clone(), job);
        self.codegen_jobs.get(&key).unwrap()
    }

    pub fn get_codegen_job(
        &self,
        app_id: &str,
        env: &str,
        id: &str,
    ) -> Result<&model::CodegenJob, AmplifyUiBuilderError> {
        self.codegen_jobs
            .get(&(app_id.to_string(), env.to_string(), id.to_string()))
            .ok_or(AmplifyUiBuilderError::NotFound {
                resource_type: "CodegenJob",
                id: id.to_string(),
            })
    }

    pub fn list_codegen_jobs(&self, app_id: &str, env: &str) -> Vec<&model::CodegenJob> {
        self.codegen_jobs
            .iter()
            .filter(|((a, e, _), _)| a == app_id && e == env)
            .map(|(_, v)| v)
            .collect()
    }

    pub fn get_metadata(&self, app_id: &str, env: &str) -> HashMap<String, String> {
        self.metadata
            .get(&(app_id.to_string(), env.to_string()))
            .cloned()
            .unwrap_or_default()
    }

    pub fn put_metadata_flag(
        &mut self,
        app_id: &str,
        env: &str,
        feature_name: String,
        new_value: String,
    ) {
        let entry = self
            .metadata
            .entry((app_id.to_string(), env.to_string()))
            .or_default();
        entry.insert(feature_name, new_value);
    }

    pub fn tag_resource(&mut self, arn: &str, tags: HashMap<String, String>) {
        let entry = self.tags.entry(arn.to_string()).or_default();
        for (k, v) in tags {
            entry.insert(k, v);
        }
    }

    pub fn untag_resource(&mut self, arn: &str, keys: &[String]) {
        if let Some(entry) = self.tags.get_mut(arn) {
            for k in keys {
                entry.remove(k);
            }
        }
    }

    pub fn list_tags(&self, arn: &str) -> HashMap<String, String> {
        self.tags.get(arn).cloned().unwrap_or_default()
    }
}
