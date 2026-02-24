use std::collections::HashMap;

use crate::types::{Application, Environment};

#[derive(Debug, Default)]
pub struct ElasticBeanstalkState {
    pub applications: HashMap<String, Application>,
    pub environments: HashMap<String, Environment>,
}

#[derive(Debug, thiserror::Error)]
pub enum ElasticBeanstalkError {
    #[error("Application {name} already exists.")]
    ApplicationAlreadyExists { name: String },
    #[error("Application {name} not found.")]
    ApplicationNotFound { name: String },
    #[error("Environment {name} not found.")]
    EnvironmentNotFound { name: String },
}

impl ElasticBeanstalkState {
    pub fn create_application(
        &mut self,
        application_name: String,
        description: Option<String>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&Application, ElasticBeanstalkError> {
        if self.applications.contains_key(&application_name) {
            return Err(ElasticBeanstalkError::ApplicationAlreadyExists {
                name: application_name,
            });
        }
        let now = current_timestamp_str();
        let arn = format!(
            "arn:aws:elasticbeanstalk:{}:{}:application/{}",
            region, account_id, application_name
        );
        let app = Application {
            application_name: application_name.clone(),
            description,
            tags,
            date_created: now.clone(),
            date_updated: now,
            arn,
        };
        self.applications.insert(application_name.clone(), app);
        Ok(self.applications.get(&application_name).unwrap())
    }

    pub fn delete_application(
        &mut self,
        application_name: &str,
    ) -> Result<(), ElasticBeanstalkError> {
        if self.applications.remove(application_name).is_none() {
            return Err(ElasticBeanstalkError::ApplicationNotFound {
                name: application_name.to_string(),
            });
        }
        Ok(())
    }

    pub fn create_environment(
        &mut self,
        environment_name: String,
        application_name: String,
        description: Option<String>,
        solution_stack_name: Option<String>,
        tier_name: String,
        tier_type: String,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&Environment, ElasticBeanstalkError> {
        if !self.applications.contains_key(&application_name) {
            return Err(ElasticBeanstalkError::ApplicationNotFound {
                name: application_name,
            });
        }
        let env_id = format!("e-{}", &uuid::Uuid::new_v4().to_string()[..12]);
        let now = current_timestamp_str();
        let arn = format!(
            "arn:aws:elasticbeanstalk:{}:{}:environment/{}/{}",
            region, account_id, application_name, environment_name
        );
        let env = Environment {
            environment_name: environment_name.clone(),
            application_name,
            environment_id: env_id,
            description,
            status: "Launching".to_string(),
            tier_name,
            tier_type,
            health: "Grey".to_string(),
            solution_stack_name,
            tags,
            date_created: now.clone(),
            date_updated: now,
            arn,
            cname: None,
            endpoint_url: None,
            platform_arn: None,
            version_label: None,
            template_name: None,
        };
        self.environments.insert(environment_name.clone(), env);
        Ok(self.environments.get(&environment_name).unwrap())
    }

    pub fn describe_environments(
        &self,
        application_name: Option<&str>,
        environment_names: &[String],
    ) -> Vec<&Environment> {
        let mut envs: Vec<&Environment> = self.environments.values().collect();

        if let Some(app) = application_name {
            envs.retain(|e| e.application_name == app);
        }

        if !environment_names.is_empty() {
            envs.retain(|e| environment_names.contains(&e.environment_name));
        }

        envs.sort_by(|a, b| a.environment_name.cmp(&b.environment_name));
        envs
    }

    pub fn get_environment_tags(
        &self,
        environment_name: &str,
    ) -> Result<&HashMap<String, String>, ElasticBeanstalkError> {
        self.environments
            .get(environment_name)
            .map(|e| &e.tags)
            .ok_or_else(|| ElasticBeanstalkError::EnvironmentNotFound {
                name: environment_name.to_string(),
            })
    }

    pub fn update_environment_tags(
        &mut self,
        environment_name: &str,
        tags_to_add: HashMap<String, String>,
        tags_to_remove: Vec<String>,
    ) -> Result<(), ElasticBeanstalkError> {
        let env = self.environments.get_mut(environment_name).ok_or_else(|| {
            ElasticBeanstalkError::EnvironmentNotFound {
                name: environment_name.to_string(),
            }
        })?;
        for key in &tags_to_remove {
            env.tags.remove(key);
        }
        for (k, v) in tags_to_add {
            env.tags.insert(k, v);
        }
        env.date_updated = current_timestamp_str();
        Ok(())
    }
}

fn current_timestamp_str() -> String {
    use chrono::{SecondsFormat, Utc};
    Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)
}
