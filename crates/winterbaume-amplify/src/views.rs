//! Serde-compatible view types for Amplify state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::AmplifyService;
use crate::state::AmplifyState;
use crate::types::{AmplifyApp, AmplifyBranch, AmplifyDomainAssociation, AmplifyJob, SubDomain};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AmplifyStateView {
    #[serde(default)]
    pub apps: HashMap<String, AmplifyAppView>,
    /// "(app_id, branch_name)" -> Branch
    #[serde(default)]
    pub branches: HashMap<String, AmplifyBranchView>,
    /// "(app_id, domain_name)" -> DomainAssociation
    #[serde(default)]
    pub domain_associations: HashMap<String, AmplifyDomainAssociationView>,
    /// "(app_id, branch_name, job_id)" -> Job
    #[serde(default)]
    pub jobs: HashMap<String, AmplifyJobView>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmplifyAppView {
    pub app_id: String,
    pub app_arn: String,
    pub name: String,
    pub description: Option<String>,
    pub repository: Option<String>,
    pub platform: Option<String>,
    pub create_time: f64,
    pub update_time: f64,
    pub iam_service_role_arn: Option<String>,
    #[serde(default)]
    pub environment_variables: HashMap<String, String>,
    pub default_domain: String,
    pub enable_branch_auto_build: bool,
    pub enable_branch_auto_deletion: bool,
    pub enable_basic_auth: bool,
    pub build_spec: Option<String>,
    pub custom_headers: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    /// `auto_branch_creation_config` block stored as JSON for Terraform round-trip.
    #[serde(default)]
    pub auto_branch_creation_config: Option<serde_json::Value>,
    /// `cache_config` block stored as JSON for Terraform round-trip.
    #[serde(default)]
    pub cache_config: Option<serde_json::Value>,
    /// `custom_rule` blocks stored as JSON array for Terraform round-trip.
    #[serde(default)]
    pub custom_rules: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmplifyBranchView {
    pub app_id: String,
    pub branch_arn: String,
    pub branch_name: String,
    pub description: Option<String>,
    pub stage: Option<String>,
    pub display_name: Option<String>,
    pub enable_auto_build: bool,
    pub enable_basic_auth: bool,
    pub enable_notification: bool,
    pub enable_performance_mode: bool,
    pub enable_pull_request_preview: bool,
    #[serde(default)]
    pub environment_variables: HashMap<String, String>,
    pub framework: Option<String>,
    pub ttl: Option<String>,
    pub create_time: f64,
    pub update_time: f64,
    pub total_number_of_jobs: String,
    pub active_job_id: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmplifyDomainAssociationView {
    pub app_id: String,
    pub domain_association_arn: String,
    pub domain_name: String,
    pub enable_auto_sub_domain: bool,
    pub domain_status: String,
    pub status_reason: String,
    #[serde(default)]
    pub sub_domains: Vec<SubDomainView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubDomainView {
    pub prefix: String,
    pub branch_name: String,
    pub dns_record: Option<String>,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmplifyJobView {
    pub app_id: String,
    pub branch_name: String,
    pub job_id: String,
    pub job_arn: String,
    pub job_type: String,
    pub status: String,
    pub start_time: f64,
    pub end_time: Option<f64>,
    pub commit_id: Option<String>,
    pub commit_message: Option<String>,
    pub commit_time: Option<f64>,
}

impl From<&AmplifyState> for AmplifyStateView {
    fn from(state: &AmplifyState) -> Self {
        AmplifyStateView {
            apps: state
                .apps
                .iter()
                .map(|(k, v)| (k.clone(), AmplifyAppView::from(v)))
                .collect(),
            branches: state
                .branches
                .iter()
                .map(|(k, v)| {
                    let key = format!("{}\x00{}", k.0, k.1);
                    (
                        key,
                        AmplifyBranchView {
                            app_id: k.0.clone(),
                            branch_arn: v.branch_arn.clone(),
                            branch_name: v.branch_name.clone(),
                            description: v.description.clone(),
                            stage: v.stage.clone(),
                            display_name: v.display_name.clone(),
                            enable_auto_build: v.enable_auto_build,
                            enable_basic_auth: v.enable_basic_auth,
                            enable_notification: v.enable_notification,
                            enable_performance_mode: v.enable_performance_mode,
                            enable_pull_request_preview: v.enable_pull_request_preview,
                            environment_variables: v.environment_variables.clone(),
                            framework: v.framework.clone(),
                            ttl: v.ttl.clone(),
                            create_time: v.create_time,
                            update_time: v.update_time,
                            total_number_of_jobs: v.total_number_of_jobs.clone(),
                            active_job_id: v.active_job_id.clone(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            domain_associations: state
                .domain_associations
                .iter()
                .map(|(k, v)| {
                    let key = format!("{}\x00{}", k.0, k.1);
                    (
                        key,
                        AmplifyDomainAssociationView {
                            app_id: k.0.clone(),
                            domain_association_arn: v.domain_association_arn.clone(),
                            domain_name: v.domain_name.clone(),
                            enable_auto_sub_domain: v.enable_auto_sub_domain,
                            domain_status: v.domain_status.clone(),
                            status_reason: v.status_reason.clone(),
                            sub_domains: v
                                .sub_domains
                                .iter()
                                .map(|s| SubDomainView {
                                    prefix: s.prefix.clone(),
                                    branch_name: s.branch_name.clone(),
                                    dns_record: s.dns_record.clone(),
                                    verified: s.verified,
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
            jobs: state
                .jobs
                .iter()
                .map(|(k, v)| {
                    let key = format!("{}\x00{}\x00{}", k.0, k.1, k.2);
                    (
                        key,
                        AmplifyJobView {
                            app_id: k.0.clone(),
                            branch_name: k.1.clone(),
                            job_id: v.job_id.clone(),
                            job_arn: v.job_arn.clone(),
                            job_type: v.job_type.clone(),
                            status: v.status.clone(),
                            start_time: v.start_time,
                            end_time: v.end_time,
                            commit_id: v.commit_id.clone(),
                            commit_message: v.commit_message.clone(),
                            commit_time: v.commit_time,
                        },
                    )
                })
                .collect(),
            tags: state.tags.clone(),
        }
    }
}

impl From<&AmplifyApp> for AmplifyAppView {
    fn from(a: &AmplifyApp) -> Self {
        AmplifyAppView {
            app_id: a.app_id.clone(),
            app_arn: a.app_arn.clone(),
            name: a.name.clone(),
            description: a.description.clone(),
            repository: a.repository.clone(),
            platform: a.platform.clone(),
            create_time: a.create_time,
            update_time: a.update_time,
            iam_service_role_arn: a.iam_service_role_arn.clone(),
            environment_variables: a.environment_variables.clone(),
            default_domain: a.default_domain.clone(),
            enable_branch_auto_build: a.enable_branch_auto_build,
            enable_branch_auto_deletion: a.enable_branch_auto_deletion,
            enable_basic_auth: a.enable_basic_auth,
            build_spec: a.build_spec.clone(),
            custom_headers: a.custom_headers.clone(),
            tags: a.tags.clone(),
            auto_branch_creation_config: a.auto_branch_creation_config.clone(),
            cache_config: a.cache_config.clone(),
            custom_rules: a.custom_rules.clone(),
        }
    }
}

impl From<AmplifyStateView> for AmplifyState {
    fn from(view: AmplifyStateView) -> Self {
        let apps = view
            .apps
            .into_iter()
            .map(|(k, v)| (k, AmplifyApp::from(v)))
            .collect();
        let mut state = AmplifyState {
            apps,
            ..Default::default()
        };
        for (key_str, v) in view.branches {
            let mut parts = key_str.splitn(2, '\x00');
            let app_id = parts.next().unwrap_or("").to_string();
            let branch_name = parts.next().unwrap_or("").to_string();
            let branch = AmplifyBranch {
                branch_arn: v.branch_arn,
                branch_name: v.branch_name,
                description: v.description,
                stage: v.stage,
                display_name: v.display_name,
                enable_auto_build: v.enable_auto_build,
                enable_basic_auth: v.enable_basic_auth,
                enable_notification: v.enable_notification,
                enable_performance_mode: v.enable_performance_mode,
                enable_pull_request_preview: v.enable_pull_request_preview,
                environment_variables: v.environment_variables,
                framework: v.framework,
                ttl: v.ttl,
                create_time: v.create_time,
                update_time: v.update_time,
                total_number_of_jobs: v.total_number_of_jobs,
                active_job_id: v.active_job_id,
                tags: v.tags,
            };
            state.branches.insert((app_id, branch_name), branch);
        }
        for (key_str, v) in view.domain_associations {
            let mut parts = key_str.splitn(2, '\x00');
            let app_id = parts.next().unwrap_or("").to_string();
            let domain_name = parts.next().unwrap_or("").to_string();
            let domain = AmplifyDomainAssociation {
                domain_association_arn: v.domain_association_arn,
                domain_name: v.domain_name,
                enable_auto_sub_domain: v.enable_auto_sub_domain,
                domain_status: v.domain_status,
                status_reason: v.status_reason,
                sub_domains: v
                    .sub_domains
                    .into_iter()
                    .map(|s| SubDomain {
                        prefix: s.prefix,
                        branch_name: s.branch_name,
                        dns_record: s.dns_record,
                        verified: s.verified,
                    })
                    .collect(),
            };
            state
                .domain_associations
                .insert((app_id, domain_name), domain);
        }
        for (key_str, v) in view.jobs {
            let mut parts = key_str.splitn(3, '\x00');
            let app_id = parts.next().unwrap_or("").to_string();
            let branch_name = parts.next().unwrap_or("").to_string();
            let job_id = parts.next().unwrap_or("").to_string();
            let job = AmplifyJob {
                job_id: v.job_id,
                job_arn: v.job_arn,
                job_type: v.job_type,
                status: v.status,
                start_time: v.start_time,
                end_time: v.end_time,
                commit_id: v.commit_id,
                commit_message: v.commit_message,
                commit_time: v.commit_time,
            };
            state.jobs.insert((app_id, branch_name, job_id), job);
        }
        state.tags = view.tags;
        state
    }
}

impl From<AmplifyAppView> for AmplifyApp {
    fn from(v: AmplifyAppView) -> Self {
        AmplifyApp {
            app_id: v.app_id,
            app_arn: v.app_arn,
            name: v.name,
            description: v.description,
            repository: v.repository,
            platform: v.platform,
            create_time: v.create_time,
            update_time: v.update_time,
            iam_service_role_arn: v.iam_service_role_arn,
            environment_variables: v.environment_variables,
            default_domain: v.default_domain,
            enable_branch_auto_build: v.enable_branch_auto_build,
            enable_branch_auto_deletion: v.enable_branch_auto_deletion,
            enable_basic_auth: v.enable_basic_auth,
            build_spec: v.build_spec,
            custom_headers: v.custom_headers,
            tags: v.tags,
            auto_branch_creation_config: v.auto_branch_creation_config,
            cache_config: v.cache_config,
            custom_rules: v.custom_rules,
        }
    }
}

impl StatefulService for AmplifyService {
    type StateView = AmplifyStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        AmplifyStateView::from(&*guard)
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
            *guard = AmplifyState::from(view);
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
            let new_state = AmplifyState::from(view);
            guard.apps.extend(new_state.apps);
            guard.branches.extend(new_state.branches);
            guard
                .domain_associations
                .extend(new_state.domain_associations);
            guard.jobs.extend(new_state.jobs);
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
