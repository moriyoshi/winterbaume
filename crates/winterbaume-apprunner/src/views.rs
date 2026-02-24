//! Serde-compatible view types for App Runner state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::AppRunnerService as AppRunnerHandlerService;
use crate::state::AppRunnerState;
use crate::types::{AppRunnerService, AutoScalingConfig, Connection};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppRunnerStateView {
    #[serde(default)]
    pub services: HashMap<String, AppRunnerServiceView>,
    #[serde(default)]
    pub connections: HashMap<String, ConnectionView>,
    #[serde(default)]
    pub auto_scaling_configs: HashMap<String, AutoScalingConfigView>,
    /// Resource ARN -> tag key -> tag value
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppRunnerServiceView {
    pub service_id: String,
    pub service_name: String,
    pub service_arn: String,
    pub service_url: String,
    pub status: String,
    pub created_at: f64,
    pub updated_at: f64,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
    #[serde(default)]
    pub encryption_configuration: Option<serde_json::Value>,
    #[serde(default)]
    pub health_check_configuration: Option<serde_json::Value>,
    #[serde(default)]
    pub instance_configuration: Option<serde_json::Value>,
    #[serde(default)]
    pub network_configuration: Option<serde_json::Value>,
    #[serde(default)]
    pub observability_configuration: Option<serde_json::Value>,
    #[serde(default)]
    pub source_configuration: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionView {
    pub connection_name: String,
    pub connection_arn: String,
    pub provider_type: String,
    pub status: String,
    pub created_at: f64,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoScalingConfigView {
    pub name: String,
    pub arn: String,
    pub revision: i32,
    pub status: String,
    pub is_default: bool,
    pub latest: bool,
    pub min_size: i32,
    pub max_size: i32,
    pub max_concurrency: i32,
    pub created_at: f64,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
}

impl From<&AppRunnerState> for AppRunnerStateView {
    fn from(state: &AppRunnerState) -> Self {
        AppRunnerStateView {
            services: state
                .services
                .iter()
                .map(|(k, v)| (k.clone(), AppRunnerServiceView::from(v)))
                .collect(),
            connections: state
                .connections
                .iter()
                .map(|(k, v)| (k.clone(), ConnectionView::from(v)))
                .collect(),
            auto_scaling_configs: state
                .auto_scaling_configs
                .iter()
                .map(|(k, v)| (k.clone(), AutoScalingConfigView::from(v)))
                .collect(),
            tags: state.tags.clone(),
        }
    }
}

impl From<&AppRunnerService> for AppRunnerServiceView {
    fn from(s: &AppRunnerService) -> Self {
        AppRunnerServiceView {
            service_id: s.service_id.clone(),
            service_name: s.service_name.clone(),
            service_arn: s.service_arn.clone(),
            service_url: s.service_url.clone(),
            status: s.status.clone(),
            created_at: s.created_at,
            updated_at: s.updated_at,
            tags: s.tags.clone(),
            encryption_configuration: s.encryption_configuration.clone(),
            health_check_configuration: s.health_check_configuration.clone(),
            instance_configuration: s.instance_configuration.clone(),
            network_configuration: s.network_configuration.clone(),
            observability_configuration: s.observability_configuration.clone(),
            source_configuration: s.source_configuration.clone(),
        }
    }
}

impl From<&Connection> for ConnectionView {
    fn from(c: &Connection) -> Self {
        ConnectionView {
            connection_name: c.connection_name.clone(),
            connection_arn: c.connection_arn.clone(),
            provider_type: c.provider_type.clone(),
            status: c.status.clone(),
            created_at: c.created_at,
            tags: c.tags.clone(),
        }
    }
}

impl From<&AutoScalingConfig> for AutoScalingConfigView {
    fn from(c: &AutoScalingConfig) -> Self {
        AutoScalingConfigView {
            name: c.name.clone(),
            arn: c.arn.clone(),
            revision: c.revision,
            status: c.status.clone(),
            is_default: c.is_default,
            latest: c.latest,
            min_size: c.min_size,
            max_size: c.max_size,
            max_concurrency: c.max_concurrency,
            created_at: c.created_at,
            tags: c.tags.clone(),
        }
    }
}

impl From<AppRunnerStateView> for AppRunnerState {
    fn from(view: AppRunnerStateView) -> Self {
        AppRunnerState {
            services: view
                .services
                .into_iter()
                .map(|(k, v)| (k, AppRunnerService::from(v)))
                .collect(),
            connections: view
                .connections
                .into_iter()
                .map(|(k, v)| (k, Connection::from(v)))
                .collect(),
            auto_scaling_configs: view
                .auto_scaling_configs
                .into_iter()
                .map(|(k, v)| (k, AutoScalingConfig::from(v)))
                .collect(),
            tags: view.tags,
        }
    }
}

impl From<AppRunnerServiceView> for AppRunnerService {
    fn from(v: AppRunnerServiceView) -> Self {
        AppRunnerService {
            service_id: v.service_id,
            service_name: v.service_name,
            service_arn: v.service_arn,
            service_url: v.service_url,
            status: v.status,
            created_at: v.created_at,
            updated_at: v.updated_at,
            tags: v.tags,
            encryption_configuration: v.encryption_configuration,
            health_check_configuration: v.health_check_configuration,
            instance_configuration: v.instance_configuration,
            network_configuration: v.network_configuration,
            observability_configuration: v.observability_configuration,
            source_configuration: v.source_configuration,
        }
    }
}

impl From<ConnectionView> for Connection {
    fn from(v: ConnectionView) -> Self {
        Connection {
            connection_name: v.connection_name,
            connection_arn: v.connection_arn,
            provider_type: v.provider_type,
            status: v.status,
            created_at: v.created_at,
            tags: v.tags,
        }
    }
}

impl From<AutoScalingConfigView> for AutoScalingConfig {
    fn from(v: AutoScalingConfigView) -> Self {
        AutoScalingConfig {
            name: v.name,
            arn: v.arn,
            revision: v.revision,
            status: v.status,
            is_default: v.is_default,
            latest: v.latest,
            min_size: v.min_size,
            max_size: v.max_size,
            max_concurrency: v.max_concurrency,
            created_at: v.created_at,
            tags: v.tags,
        }
    }
}

impl StatefulService for AppRunnerHandlerService {
    type StateView = AppRunnerStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        AppRunnerStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = AppRunnerState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            let new_state = AppRunnerState::from(view);
            guard.services.extend(new_state.services);
            guard.connections.extend(new_state.connections);
            guard
                .auto_scaling_configs
                .extend(new_state.auto_scaling_configs);
            for (arn, new_tags) in new_state.tags {
                guard.tags.entry(arn).or_default().extend(new_tags);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
