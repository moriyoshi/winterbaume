//! Serde-compatible view types for CodeDeploy state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CodeDeployService;
use crate::state::CodeDeployState;

/// Serializable view of the entire CodeDeploy state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeDeployStateView {
    #[serde(default)]
    pub applications: HashMap<String, ApplicationView>,
    /// Deployment groups as a list (tuple key `(app_name, group_name)` flattened).
    #[serde(default)]
    pub deployment_groups: Vec<DeploymentGroupView>,
    #[serde(default)]
    pub deployments: HashMap<String, DeploymentView>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationView {
    pub application_id: String,
    pub application_name: String,
    pub compute_platform: String,
    pub create_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentGroupView {
    pub deployment_group_id: String,
    pub deployment_group_name: String,
    pub application_name: String,
    pub service_role_arn: String,
    pub deployment_config_name: String,
    pub compute_platform: String,
    pub create_time: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub alarm_configuration: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub blue_green_deployment_config: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub ec2_tag_filter: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub ec2_tag_set: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub ecs_service: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub load_balancer_info: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub trigger_configuration: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentView {
    pub deployment_id: String,
    pub application_name: String,
    pub deployment_group_name: String,
    pub deployment_config_name: String,
    pub description: String,
    pub revision_type: Option<String>,
    pub revision_s3_bucket: Option<String>,
    pub revision_s3_key: Option<String>,
    pub revision_s3_bundle_type: Option<String>,
    pub revision_github_repository: Option<String>,
    pub revision_github_commit_id: Option<String>,
    pub status: String,
    pub create_time: String,
    pub file_exists_behavior: Option<String>,
    pub ignore_application_stop_failures: bool,
}

fn parse_datetime(s: &str) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(|_| chrono::Utc::now())
}

// --- From internal types to view types ---

impl From<&CodeDeployState> for CodeDeployStateView {
    fn from(state: &CodeDeployState) -> Self {
        CodeDeployStateView {
            applications: state
                .applications
                .iter()
                .map(|(k, a)| {
                    (
                        k.clone(),
                        ApplicationView {
                            application_id: a.application_id.clone(),
                            application_name: a.application_name.clone(),
                            compute_platform: a.compute_platform.clone(),
                            create_time: a.create_time.to_rfc3339(),
                        },
                    )
                })
                .collect(),
            deployment_groups: state
                .deployment_groups
                .values()
                .map(|dg| DeploymentGroupView {
                    deployment_group_id: dg.deployment_group_id.clone(),
                    deployment_group_name: dg.deployment_group_name.clone(),
                    application_name: dg.application_name.clone(),
                    service_role_arn: dg.service_role_arn.clone(),
                    deployment_config_name: dg.deployment_config_name.clone(),
                    compute_platform: dg.compute_platform.clone(),
                    create_time: dg.create_time.to_rfc3339(),
                    alarm_configuration: None,
                    blue_green_deployment_config: None,
                    ec2_tag_filter: vec![],
                    ec2_tag_set: vec![],
                    ecs_service: vec![],
                    load_balancer_info: vec![],
                    trigger_configuration: vec![],
                })
                .collect(),
            deployments: state
                .deployments
                .iter()
                .map(|(k, d)| {
                    (
                        k.clone(),
                        DeploymentView {
                            deployment_id: d.deployment_id.clone(),
                            application_name: d.application_name.clone(),
                            deployment_group_name: d.deployment_group_name.clone(),
                            deployment_config_name: d.deployment_config_name.clone(),
                            description: d.description.clone(),
                            revision_type: d.revision_type.clone(),
                            revision_s3_bucket: d.revision_s3_bucket.clone(),
                            revision_s3_key: d.revision_s3_key.clone(),
                            revision_s3_bundle_type: d.revision_s3_bundle_type.clone(),
                            revision_github_repository: d.revision_github_repository.clone(),
                            revision_github_commit_id: d.revision_github_commit_id.clone(),
                            status: d.status.clone(),
                            create_time: d.create_time.to_rfc3339(),
                            file_exists_behavior: d.file_exists_behavior.clone(),
                            ignore_application_stop_failures: d.ignore_application_stop_failures,
                        },
                    )
                })
                .collect(),
            tags: state.tags.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<CodeDeployStateView> for CodeDeployState {
    fn from(view: CodeDeployStateView) -> Self {
        let mut state = CodeDeployState::default();
        for (k, a) in view.applications {
            state.applications.insert(
                k,
                crate::types::Application {
                    application_id: a.application_id,
                    application_name: a.application_name,
                    compute_platform: a.compute_platform,
                    create_time: parse_datetime(&a.create_time),
                },
            );
        }
        for dg in view.deployment_groups {
            let key = (
                dg.application_name.clone(),
                dg.deployment_group_name.clone(),
            );
            state.deployment_groups.insert(
                key,
                crate::types::DeploymentGroup {
                    deployment_group_id: dg.deployment_group_id,
                    deployment_group_name: dg.deployment_group_name,
                    application_name: dg.application_name,
                    service_role_arn: dg.service_role_arn,
                    deployment_config_name: dg.deployment_config_name,
                    compute_platform: dg.compute_platform,
                    create_time: parse_datetime(&dg.create_time),
                },
            );
        }
        for (k, d) in view.deployments {
            state.deployments.insert(
                k,
                crate::types::Deployment {
                    deployment_id: d.deployment_id,
                    application_name: d.application_name,
                    deployment_group_name: d.deployment_group_name,
                    deployment_config_name: d.deployment_config_name,
                    description: d.description,
                    revision_type: d.revision_type,
                    revision_s3_bucket: d.revision_s3_bucket,
                    revision_s3_key: d.revision_s3_key,
                    revision_s3_bundle_type: d.revision_s3_bundle_type,
                    revision_github_repository: d.revision_github_repository,
                    revision_github_commit_id: d.revision_github_commit_id,
                    status: d.status,
                    create_time: parse_datetime(&d.create_time),
                    file_exists_behavior: d.file_exists_behavior,
                    ignore_application_stop_failures: d.ignore_application_stop_failures,
                },
            );
        }
        state.tags = view.tags;
        state
    }
}

// --- StatefulService implementation ---

impl StatefulService for CodeDeployService {
    type StateView = CodeDeployStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        CodeDeployStateView::from(&*guard)
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
            *guard = CodeDeployState::from(view);
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
            let merged = CodeDeployState::from(view);
            for (k, v) in merged.applications {
                guard.applications.insert(k, v);
            }
            for (k, v) in merged.deployment_groups {
                guard.deployment_groups.insert(k, v);
            }
            for (k, v) in merged.deployments {
                guard.deployments.insert(k, v);
            }
            for (k, v) in merged.tags {
                guard.tags.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
