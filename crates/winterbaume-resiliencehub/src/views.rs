//! Serde-compatible view types for ResilienceHub state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ResilienceHubService;
use crate::state::ResilienceHubState;
use crate::types::{
    App, AppComponentData, AppVersion, AppVersionResource, AssessmentData, FailurePolicyData,
    ResiliencyPolicyData,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResilienceHubStateView {
    #[serde(default)]
    pub apps: HashMap<String, AppView>,
    #[serde(default)]
    pub policies: HashMap<String, ResiliencyPolicyView>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppView {
    pub app_arn: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub creation_time: DateTime<Utc>,
    pub compliance_status: String,
    pub policy_arn: String,
    pub assessment_schedule: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub versions: Vec<AppVersionView>,
    pub next_version_id: i64,
    #[serde(default)]
    pub resources: Vec<AppVersionResourceView>,
    pub app_template_body: String,
    #[serde(default)]
    pub app_components: Vec<AppComponentView>,
    #[serde(default)]
    pub assessments: Vec<AssessmentView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppVersionView {
    pub app_version: String,
    pub identifier: i64,
    pub creation_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppVersionResourceView {
    pub app_arn: String,
    pub resource_name: String,
    pub logical_resource_id: String,
    pub physical_resource_id: String,
    pub resource_type: String,
    #[serde(default)]
    pub app_components: Vec<String>,
    pub aws_region: String,
    pub aws_account_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppComponentView {
    pub id: String,
    pub name: String,
    pub component_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentView {
    pub assessment_arn: String,
    pub assessment_name: String,
    pub assessment_status: String,
    pub app_arn: String,
    pub app_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResiliencyPolicyView {
    pub policy_arn: String,
    pub policy_name: String,
    pub policy_description: String,
    pub data_location_constraint: String,
    pub tier: String,
    #[serde(default)]
    pub policy: HashMap<String, FailurePolicyView>,
    pub creation_time: DateTime<Utc>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailurePolicyView {
    pub rpo_in_secs: i32,
    pub rto_in_secs: i32,
}

impl From<&ResilienceHubState> for ResilienceHubStateView {
    fn from(state: &ResilienceHubState) -> Self {
        ResilienceHubStateView {
            apps: state
                .apps
                .iter()
                .map(|(k, v)| (k.clone(), AppView::from(v)))
                .collect(),
            policies: state
                .policies
                .iter()
                .map(|(k, v)| (k.clone(), ResiliencyPolicyView::from(v)))
                .collect(),
            tags: state.tags.clone(),
        }
    }
}

impl From<&App> for AppView {
    fn from(a: &App) -> Self {
        AppView {
            app_arn: a.app_arn.clone(),
            name: a.name.clone(),
            description: a.description.clone(),
            status: a.status.clone(),
            creation_time: a.creation_time,
            compliance_status: a.compliance_status.clone(),
            policy_arn: a.policy_arn.clone(),
            assessment_schedule: a.assessment_schedule.clone(),
            tags: a.tags.clone(),
            versions: a.versions.iter().map(AppVersionView::from).collect(),
            next_version_id: a.next_version_id,
            resources: a
                .resources
                .iter()
                .map(AppVersionResourceView::from)
                .collect(),
            app_template_body: a.app_template_body.clone(),
            app_components: a
                .app_components
                .iter()
                .map(AppComponentView::from)
                .collect(),
            assessments: a.assessments.iter().map(AssessmentView::from).collect(),
        }
    }
}

impl From<&AppVersion> for AppVersionView {
    fn from(v: &AppVersion) -> Self {
        AppVersionView {
            app_version: v.app_version.clone(),
            identifier: v.identifier,
            creation_time: v.creation_time,
        }
    }
}

impl From<&AppVersionResource> for AppVersionResourceView {
    fn from(r: &AppVersionResource) -> Self {
        AppVersionResourceView {
            app_arn: r.app_arn.clone(),
            resource_name: r.resource_name.clone(),
            logical_resource_id: r.logical_resource_id.clone(),
            physical_resource_id: r.physical_resource_id.clone(),
            resource_type: r.resource_type.clone(),
            app_components: r.app_components.clone(),
            aws_region: r.aws_region.clone(),
            aws_account_id: r.aws_account_id.clone(),
        }
    }
}

impl From<&AppComponentData> for AppComponentView {
    fn from(c: &AppComponentData) -> Self {
        AppComponentView {
            id: c.id.clone(),
            name: c.name.clone(),
            component_type: c.component_type.clone(),
        }
    }
}

impl From<&AssessmentData> for AssessmentView {
    fn from(a: &AssessmentData) -> Self {
        AssessmentView {
            assessment_arn: a.assessment_arn.clone(),
            assessment_name: a.assessment_name.clone(),
            assessment_status: a.assessment_status.clone(),
            app_arn: a.app_arn.clone(),
            app_version: a.app_version.clone(),
        }
    }
}

impl From<&ResiliencyPolicyData> for ResiliencyPolicyView {
    fn from(p: &ResiliencyPolicyData) -> Self {
        ResiliencyPolicyView {
            policy_arn: p.policy_arn.clone(),
            policy_name: p.policy_name.clone(),
            policy_description: p.policy_description.clone(),
            data_location_constraint: p.data_location_constraint.clone(),
            tier: p.tier.clone(),
            policy: p
                .policy
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        FailurePolicyView {
                            rpo_in_secs: v.rpo_in_secs,
                            rto_in_secs: v.rto_in_secs,
                        },
                    )
                })
                .collect(),
            creation_time: p.creation_time,
            tags: p.tags.clone(),
        }
    }
}

impl From<ResilienceHubStateView> for ResilienceHubState {
    fn from(view: ResilienceHubStateView) -> Self {
        ResilienceHubState {
            apps: view
                .apps
                .into_iter()
                .map(|(k, v)| (k, App::from(v)))
                .collect(),
            policies: view
                .policies
                .into_iter()
                .map(|(k, v)| (k, ResiliencyPolicyData::from(v)))
                .collect(),
            tags: view.tags,
        }
    }
}

impl From<AppView> for App {
    fn from(v: AppView) -> Self {
        App {
            app_arn: v.app_arn,
            name: v.name,
            description: v.description,
            status: v.status,
            creation_time: v.creation_time,
            compliance_status: v.compliance_status,
            policy_arn: v.policy_arn,
            assessment_schedule: v.assessment_schedule,
            tags: v.tags,
            versions: v
                .versions
                .into_iter()
                .map(|vv| AppVersion {
                    app_version: vv.app_version,
                    identifier: vv.identifier,
                    creation_time: vv.creation_time,
                })
                .collect(),
            next_version_id: v.next_version_id,
            resources: v
                .resources
                .into_iter()
                .map(|r| AppVersionResource {
                    app_arn: r.app_arn,
                    resource_name: r.resource_name,
                    logical_resource_id: r.logical_resource_id,
                    physical_resource_id: r.physical_resource_id,
                    resource_type: r.resource_type,
                    app_components: r.app_components,
                    aws_region: r.aws_region,
                    aws_account_id: r.aws_account_id,
                })
                .collect(),
            app_template_body: v.app_template_body,
            app_components: v
                .app_components
                .into_iter()
                .map(|c| AppComponentData {
                    id: c.id,
                    name: c.name,
                    component_type: c.component_type,
                })
                .collect(),
            assessments: v
                .assessments
                .into_iter()
                .map(|a| AssessmentData {
                    assessment_arn: a.assessment_arn,
                    assessment_name: a.assessment_name,
                    assessment_status: a.assessment_status,
                    app_arn: a.app_arn,
                    app_version: a.app_version,
                })
                .collect(),
        }
    }
}

impl From<ResiliencyPolicyView> for ResiliencyPolicyData {
    fn from(v: ResiliencyPolicyView) -> Self {
        ResiliencyPolicyData {
            policy_arn: v.policy_arn,
            policy_name: v.policy_name,
            policy_description: v.policy_description,
            data_location_constraint: v.data_location_constraint,
            tier: v.tier,
            policy: v
                .policy
                .into_iter()
                .map(|(k, fp)| {
                    (
                        k,
                        FailurePolicyData {
                            rpo_in_secs: fp.rpo_in_secs,
                            rto_in_secs: fp.rto_in_secs,
                        },
                    )
                })
                .collect(),
            creation_time: v.creation_time,
            tags: v.tags,
        }
    }
}

impl StatefulService for ResilienceHubService {
    type StateView = ResilienceHubStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ResilienceHubStateView::from(&*guard)
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
            *guard = ResilienceHubState::from(view);
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
            for (k, v) in view.apps {
                guard.apps.insert(k, App::from(v));
            }
            for (k, v) in view.policies {
                guard.policies.insert(k, ResiliencyPolicyData::from(v));
            }
            for (k, v) in view.tags {
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
