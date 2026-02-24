//! Serde-compatible view types for Elastic Beanstalk state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ElasticBeanstalkService;
use crate::state::ElasticBeanstalkState;
use crate::types::{Application, Environment};

/// Serializable view of the entire Elastic Beanstalk state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ElasticBeanstalkStateView {
    #[serde(default)]
    pub applications: HashMap<String, ApplicationView>,
    #[serde(default)]
    pub environments: HashMap<String, EnvironmentView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationView {
    pub application_name: String,
    pub description: Option<String>,
    pub tags: HashMap<String, String>,
    pub date_created: String,
    pub date_updated: String,
    pub arn: String,
    /// `appversion_lifecycle` nested block.
    #[serde(default)]
    pub appversion_lifecycle: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentView {
    pub environment_name: String,
    pub application_name: String,
    pub environment_id: String,
    pub description: Option<String>,
    pub status: String,
    pub tier_name: String,
    pub tier_type: String,
    pub health: String,
    pub solution_stack_name: Option<String>,
    pub tags: HashMap<String, String>,
    pub date_created: String,
    pub date_updated: String,
    pub arn: String,
    pub cname: Option<String>,
    pub endpoint_url: Option<String>,
    pub platform_arn: Option<String>,
    pub version_label: Option<String>,
    pub template_name: Option<String>,
    /// `setting` nested blocks.
    #[serde(default)]
    pub setting: Vec<JsonValue>,
}

impl From<&ElasticBeanstalkState> for ElasticBeanstalkStateView {
    fn from(state: &ElasticBeanstalkState) -> Self {
        ElasticBeanstalkStateView {
            applications: state
                .applications
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ApplicationView {
                            application_name: v.application_name.clone(),
                            description: v.description.clone(),
                            tags: v.tags.clone(),
                            date_created: v.date_created.clone(),
                            date_updated: v.date_updated.clone(),
                            arn: v.arn.clone(),
                            appversion_lifecycle: None,
                        },
                    )
                })
                .collect(),
            environments: state
                .environments
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        EnvironmentView {
                            environment_name: v.environment_name.clone(),
                            application_name: v.application_name.clone(),
                            environment_id: v.environment_id.clone(),
                            description: v.description.clone(),
                            status: v.status.clone(),
                            tier_name: v.tier_name.clone(),
                            tier_type: v.tier_type.clone(),
                            health: v.health.clone(),
                            solution_stack_name: v.solution_stack_name.clone(),
                            tags: v.tags.clone(),
                            date_created: v.date_created.clone(),
                            date_updated: v.date_updated.clone(),
                            arn: v.arn.clone(),
                            cname: v.cname.clone(),
                            endpoint_url: v.endpoint_url.clone(),
                            platform_arn: v.platform_arn.clone(),
                            version_label: v.version_label.clone(),
                            template_name: v.template_name.clone(),
                            setting: vec![],
                        },
                    )
                })
                .collect(),
        }
    }
}

impl From<ElasticBeanstalkStateView> for ElasticBeanstalkState {
    fn from(view: ElasticBeanstalkStateView) -> Self {
        ElasticBeanstalkState {
            applications: view
                .applications
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Application {
                            application_name: v.application_name,
                            description: v.description,
                            tags: v.tags,
                            date_created: v.date_created,
                            date_updated: v.date_updated,
                            arn: v.arn,
                        },
                    )
                })
                .collect(),
            environments: view
                .environments
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Environment {
                            environment_name: v.environment_name,
                            application_name: v.application_name,
                            environment_id: v.environment_id,
                            description: v.description,
                            status: v.status,
                            tier_name: v.tier_name,
                            tier_type: v.tier_type,
                            health: v.health,
                            solution_stack_name: v.solution_stack_name,
                            tags: v.tags,
                            date_created: v.date_created,
                            date_updated: v.date_updated,
                            arn: v.arn,
                            cname: v.cname,
                            endpoint_url: v.endpoint_url,
                            platform_arn: v.platform_arn,
                            version_label: v.version_label,
                            template_name: v.template_name,
                        },
                    )
                })
                .collect(),
        }
    }
}

impl StatefulService for ElasticBeanstalkService {
    type StateView = ElasticBeanstalkStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ElasticBeanstalkStateView::from(&*guard)
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
            *guard = ElasticBeanstalkState::from(view);
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
            for (k, v) in view.applications {
                guard.applications.insert(
                    k,
                    Application {
                        application_name: v.application_name,
                        description: v.description,
                        tags: v.tags,
                        date_created: v.date_created,
                        date_updated: v.date_updated,
                        arn: v.arn,
                    },
                );
            }
            for (k, v) in view.environments {
                guard.environments.insert(
                    k,
                    Environment {
                        environment_name: v.environment_name,
                        application_name: v.application_name,
                        environment_id: v.environment_id,
                        description: v.description,
                        status: v.status,
                        tier_name: v.tier_name,
                        tier_type: v.tier_type,
                        health: v.health,
                        solution_stack_name: v.solution_stack_name,
                        tags: v.tags,
                        date_created: v.date_created,
                        date_updated: v.date_updated,
                        arn: v.arn,
                        cname: v.cname,
                        endpoint_url: v.endpoint_url,
                        platform_arn: v.platform_arn,
                        version_label: v.version_label,
                        template_name: v.template_name,
                    },
                );
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
