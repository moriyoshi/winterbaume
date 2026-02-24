use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct AmplifyBackendState {
    /// Backends keyed by `(app_id, environment_name)`.
    pub backends: HashMap<(String, String), Backend>,
}

#[derive(Debug, Error)]
pub enum AmplifyBackendError {
    #[error("Backend environment {environment} not found for app {app_id}")]
    NotFound { app_id: String, environment: String },

    #[error("Backend environment {environment} already exists for app {app_id}")]
    AlreadyExists { app_id: String, environment: String },

    #[error("{message}")]
    Validation { message: String },
}

impl AmplifyBackendState {
    pub fn create_backend(
        &mut self,
        app_id: &str,
        app_name: &str,
        environment: &str,
        resource_name: Option<String>,
    ) -> Result<&Backend, AmplifyBackendError> {
        let key = (app_id.to_string(), environment.to_string());
        if self.backends.contains_key(&key) {
            return Err(AmplifyBackendError::AlreadyExists {
                app_id: app_id.to_string(),
                environment: environment.to_string(),
            });
        }
        let backend = Backend {
            app_id: app_id.to_string(),
            app_name: app_name.to_string(),
            backend_environment_name: environment.to_string(),
            resource_name,
            amplify_meta_config: Some("{}".to_string()),
            amplify_feature_flags: None,
        };
        self.backends.insert(key.clone(), backend);
        Ok(self.backends.get(&key).unwrap())
    }

    pub fn get_backend(
        &self,
        app_id: &str,
        environment: &str,
    ) -> Result<&Backend, AmplifyBackendError> {
        self.backends
            .get(&(app_id.to_string(), environment.to_string()))
            .ok_or_else(|| AmplifyBackendError::NotFound {
                app_id: app_id.to_string(),
                environment: environment.to_string(),
            })
    }

    pub fn delete_backend(
        &mut self,
        app_id: &str,
        environment: &str,
    ) -> Result<(), AmplifyBackendError> {
        self.backends
            .remove(&(app_id.to_string(), environment.to_string()))
            .ok_or_else(|| AmplifyBackendError::NotFound {
                app_id: app_id.to_string(),
                environment: environment.to_string(),
            })?;
        Ok(())
    }

    pub fn clone_backend(
        &mut self,
        app_id: &str,
        environment: &str,
        target_environment: &str,
    ) -> Result<&Backend, AmplifyBackendError> {
        let source = self.get_backend(app_id, environment)?.clone();
        let target_key = (app_id.to_string(), target_environment.to_string());
        if self.backends.contains_key(&target_key) {
            return Err(AmplifyBackendError::AlreadyExists {
                app_id: app_id.to_string(),
                environment: target_environment.to_string(),
            });
        }
        let cloned = Backend {
            backend_environment_name: target_environment.to_string(),
            ..source
        };
        self.backends.insert(target_key.clone(), cloned);
        Ok(self.backends.get(&target_key).unwrap())
    }

    pub fn list_environments_for_app(&self, app_id: &str) -> Vec<String> {
        let mut envs: Vec<String> = self
            .backends
            .keys()
            .filter(|(a, _)| a == app_id)
            .map(|(_, e)| e.clone())
            .collect();
        envs.sort();
        envs
    }
}
