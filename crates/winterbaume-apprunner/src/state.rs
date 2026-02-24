use std::collections::HashMap;

use uuid::Uuid;

use crate::types::*;

#[derive(Debug, Default)]
pub struct AppRunnerState {
    pub services: HashMap<String, AppRunnerService>,
    pub connections: HashMap<String, Connection>,
    pub auto_scaling_configs: HashMap<String, AutoScalingConfig>,
    /// Resource ARN -> tag key -> tag value
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, thiserror::Error)]
pub enum AppRunnerError {
    #[error("Service {0} already exists.")]
    ServiceAlreadyExists(String),
    #[error("Connection {0} already exists.")]
    ConnectionAlreadyExists(String),
    #[error("Service {0} not found.")]
    ServiceNotFound(String),
    #[error("Connection {0} not found.")]
    ConnectionNotFound(String),
    #[error("AutoScalingConfiguration {0} not found.")]
    AutoScalingConfigNotFound(String),
}

impl AppRunnerState {
    fn now_f64() -> f64 {
        chrono::Utc::now().timestamp() as f64
    }

    // ---- Service CRUD ----

    pub fn create_service(
        &mut self,
        service_name: &str,
        tags: Vec<(String, String)>,
        account_id: &str,
        region: &str,
    ) -> Result<&AppRunnerService, AppRunnerError> {
        if self.services.contains_key(service_name) {
            return Err(AppRunnerError::ServiceAlreadyExists(
                service_name.to_string(),
            ));
        }
        let service_id = Uuid::new_v4().to_string();
        let service_arn =
            format!("arn:aws:apprunner:{region}:{account_id}:service/{service_name}/{service_id}");
        let now = Self::now_f64();
        let svc = AppRunnerService {
            service_id,
            service_name: service_name.to_string(),
            service_arn,
            service_url: format!("{service_name}.{region}.awsapprunner.com"),
            status: "RUNNING".to_string(),
            created_at: now,
            updated_at: now,
            tags,
            encryption_configuration: None,
            health_check_configuration: None,
            instance_configuration: None,
            network_configuration: None,
            observability_configuration: None,
            source_configuration: None,
        };
        self.services.insert(service_name.to_string(), svc);
        Ok(self.services.get(service_name).unwrap())
    }

    pub fn describe_service(&self, service_arn: &str) -> Result<&AppRunnerService, AppRunnerError> {
        self.services
            .values()
            .find(|s| s.service_arn == service_arn || s.service_name == service_arn)
            .ok_or_else(|| AppRunnerError::ServiceNotFound(service_arn.to_string()))
    }

    pub fn list_services(&self) -> Vec<&AppRunnerService> {
        self.services.values().collect()
    }

    pub fn delete_service(
        &mut self,
        service_arn: &str,
    ) -> Result<AppRunnerService, AppRunnerError> {
        let name = self
            .services
            .values()
            .find(|s| s.service_arn == service_arn)
            .map(|s| s.service_name.clone())
            .ok_or_else(|| AppRunnerError::ServiceNotFound(service_arn.to_string()))?;
        let mut svc = self.services.remove(&name).unwrap();
        svc.status = "DELETED".to_string();
        Ok(svc)
    }

    pub fn update_service(
        &mut self,
        service_arn: &str,
    ) -> Result<&AppRunnerService, AppRunnerError> {
        let svc = self
            .services
            .values_mut()
            .find(|s| s.service_arn == service_arn)
            .ok_or_else(|| AppRunnerError::ServiceNotFound(service_arn.to_string()))?;
        svc.updated_at = Self::now_f64();
        Ok(self
            .services
            .values()
            .find(|s| s.service_arn == service_arn)
            .unwrap())
    }

    pub fn pause_service(
        &mut self,
        service_arn: &str,
    ) -> Result<&AppRunnerService, AppRunnerError> {
        let svc = self
            .services
            .values_mut()
            .find(|s| s.service_arn == service_arn)
            .ok_or_else(|| AppRunnerError::ServiceNotFound(service_arn.to_string()))?;
        svc.status = "PAUSED".to_string();
        svc.updated_at = Self::now_f64();
        Ok(self
            .services
            .values()
            .find(|s| s.service_arn == service_arn)
            .unwrap())
    }

    pub fn resume_service(
        &mut self,
        service_arn: &str,
    ) -> Result<&AppRunnerService, AppRunnerError> {
        let svc = self
            .services
            .values_mut()
            .find(|s| s.service_arn == service_arn)
            .ok_or_else(|| AppRunnerError::ServiceNotFound(service_arn.to_string()))?;
        svc.status = "RUNNING".to_string();
        svc.updated_at = Self::now_f64();
        Ok(self
            .services
            .values()
            .find(|s| s.service_arn == service_arn)
            .unwrap())
    }

    // ---- Connection CRUD ----

    pub fn create_connection(
        &mut self,
        connection_name: &str,
        provider_type: &str,
        tags: Vec<(String, String)>,
        account_id: &str,
        region: &str,
    ) -> Result<&Connection, AppRunnerError> {
        if self.connections.contains_key(connection_name) {
            return Err(AppRunnerError::ConnectionAlreadyExists(
                connection_name.to_string(),
            ));
        }
        let connection_arn =
            format!("arn:aws:apprunner:{region}:{account_id}:connection/{connection_name}");
        let now = Self::now_f64();
        let conn = Connection {
            connection_name: connection_name.to_string(),
            connection_arn,
            provider_type: provider_type.to_string(),
            status: "PENDING_HANDSHAKE".to_string(),
            created_at: now,
            tags,
        };
        self.connections.insert(connection_name.to_string(), conn);
        Ok(self.connections.get(connection_name).unwrap())
    }

    pub fn list_connections(&self) -> Vec<&Connection> {
        self.connections.values().collect()
    }

    pub fn delete_connection(
        &mut self,
        connection_arn: &str,
    ) -> Result<Connection, AppRunnerError> {
        let name = self
            .connections
            .values()
            .find(|c| c.connection_arn == connection_arn)
            .map(|c| c.connection_name.clone())
            .ok_or_else(|| AppRunnerError::ConnectionNotFound(connection_arn.to_string()))?;
        Ok(self.connections.remove(&name).unwrap())
    }

    // ---- AutoScalingConfiguration CRUD ----

    #[allow(clippy::too_many_arguments)]
    pub fn create_auto_scaling_configuration(
        &mut self,
        name: &str,
        min_size: i32,
        max_size: i32,
        max_concurrency: i32,
        tags: Vec<(String, String)>,
        account_id: &str,
        region: &str,
    ) -> Result<&AutoScalingConfig, AppRunnerError> {
        let arn =
            format!("arn:aws:apprunner:{region}:{account_id}:autoscalingconfiguration/{name}/1");
        let now = Self::now_f64();
        let config = AutoScalingConfig {
            name: name.to_string(),
            arn: arn.clone(),
            revision: 1,
            status: "ACTIVE".to_string(),
            is_default: false,
            latest: true,
            min_size,
            max_size,
            max_concurrency,
            created_at: now,
            tags,
        };
        self.auto_scaling_configs.insert(arn.clone(), config);
        Ok(self.auto_scaling_configs.get(&arn).unwrap())
    }

    pub fn describe_auto_scaling_configuration(
        &self,
        arn: &str,
    ) -> Result<&AutoScalingConfig, AppRunnerError> {
        self.auto_scaling_configs
            .values()
            .find(|c| c.arn == arn || c.name == arn)
            .ok_or_else(|| AppRunnerError::AutoScalingConfigNotFound(arn.to_string()))
    }

    pub fn list_auto_scaling_configurations(&self) -> Vec<&AutoScalingConfig> {
        self.auto_scaling_configs.values().collect()
    }

    pub fn delete_auto_scaling_configuration(
        &mut self,
        arn: &str,
    ) -> Result<AutoScalingConfig, AppRunnerError> {
        let key = self
            .auto_scaling_configs
            .keys()
            .find(|k| k.as_str() == arn || self.auto_scaling_configs[*k].name == arn)
            .cloned()
            .ok_or_else(|| AppRunnerError::AutoScalingConfigNotFound(arn.to_string()))?;
        Ok(self.auto_scaling_configs.remove(&key).unwrap())
    }

    pub fn update_default_auto_scaling_configuration(
        &mut self,
        arn: &str,
    ) -> Result<&AutoScalingConfig, AppRunnerError> {
        // Clear existing defaults
        for cfg in self.auto_scaling_configs.values_mut() {
            cfg.is_default = false;
        }
        // Set the new default
        self.auto_scaling_configs
            .values_mut()
            .find(|c| c.arn == arn || c.name == arn)
            .ok_or_else(|| AppRunnerError::AutoScalingConfigNotFound(arn.to_string()))?
            .is_default = true;
        Ok(self
            .auto_scaling_configs
            .values()
            .find(|c| c.arn == arn || c.name == arn)
            .unwrap())
    }

    // ---- Tag operations ----

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: Vec<(String, String)>,
    ) -> Result<(), AppRunnerError> {
        let entry = self.tags.entry(resource_arn.to_string()).or_default();
        for (k, v) in tags {
            entry.insert(k, v);
        }
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), AppRunnerError> {
        if let Some(entry) = self.tags.get_mut(resource_arn) {
            for key in tag_keys {
                entry.remove(key);
            }
        }
        Ok(())
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<Vec<(String, String)>, AppRunnerError> {
        let tags = self
            .tags
            .get(resource_arn)
            .map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            .unwrap_or_default();
        Ok(tags)
    }
}
