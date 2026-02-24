//! Serde-compatible view types for EMR Containers state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::EmrContainersService;
use crate::state::EmrContainersState;
use crate::types::{
    ContainerInfo, ContainerProvider, EksInfo, JobRun, JobTemplate, ManagedEndpoint,
    SecurityConfiguration, VirtualCluster,
};

/// Serializable view of the entire EMR Containers state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmrContainersStateView {
    /// Virtual clusters keyed by cluster ID.
    #[serde(default)]
    pub virtual_clusters: HashMap<String, VirtualClusterView>,
    /// Job runs keyed by job run ID.
    #[serde(default)]
    pub job_runs: HashMap<String, JobRunView>,
    /// Managed endpoints keyed by endpoint ID.
    #[serde(default)]
    pub managed_endpoints: HashMap<String, ManagedEndpointView>,
    /// Job templates keyed by template ID.
    #[serde(default)]
    pub job_templates: HashMap<String, JobTemplateView>,
    /// Security configurations keyed by config ID.
    #[serde(default)]
    pub security_configurations: HashMap<String, SecurityConfigurationView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualClusterView {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub state: String,
    pub container_provider: ContainerProviderView,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerProviderView {
    pub provider_type: String,
    pub id: String,
    pub info: Option<ContainerInfoView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerInfoView {
    pub eks_info: Option<EksInfoView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EksInfoView {
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobRunView {
    pub id: String,
    pub name: Option<String>,
    pub virtual_cluster_id: String,
    pub arn: String,
    pub state: String,
    pub execution_role_arn: String,
    pub release_label: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedEndpointView {
    pub id: String,
    pub name: String,
    pub virtual_cluster_id: String,
    pub arn: String,
    pub state: String,
    pub endpoint_type: String,
    pub release_label: String,
    pub execution_role_arn: String,
    pub certificate_arn: Option<String>,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobTemplateView {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub created_at: DateTime<Utc>,
    pub kms_key_arn: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub job_template_data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfigurationView {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub security_configuration_data: serde_json::Value,
}

// --- From internal types to view types ---

impl From<&EmrContainersState> for EmrContainersStateView {
    fn from(state: &EmrContainersState) -> Self {
        EmrContainersStateView {
            virtual_clusters: state
                .virtual_clusters
                .iter()
                .map(|(k, v)| (k.clone(), VirtualClusterView::from(v)))
                .collect(),
            job_runs: state
                .job_runs
                .iter()
                .map(|(k, v)| (k.clone(), JobRunView::from(v)))
                .collect(),
            managed_endpoints: state
                .managed_endpoints
                .iter()
                .map(|(k, v)| (k.clone(), ManagedEndpointView::from(v)))
                .collect(),
            job_templates: state
                .job_templates
                .iter()
                .map(|(k, v)| (k.clone(), JobTemplateView::from(v)))
                .collect(),
            security_configurations: state
                .security_configurations
                .iter()
                .map(|(k, v)| (k.clone(), SecurityConfigurationView::from(v)))
                .collect(),
        }
    }
}

impl From<&VirtualCluster> for VirtualClusterView {
    fn from(vc: &VirtualCluster) -> Self {
        VirtualClusterView {
            id: vc.id.clone(),
            name: vc.name.clone(),
            arn: vc.arn.clone(),
            state: vc.state.clone(),
            container_provider: ContainerProviderView {
                provider_type: vc.container_provider.provider_type.clone(),
                id: vc.container_provider.id.clone(),
                info: vc
                    .container_provider
                    .info
                    .as_ref()
                    .map(|info| ContainerInfoView {
                        eks_info: info.eks_info.as_ref().map(|eks| EksInfoView {
                            namespace: eks.namespace.clone(),
                        }),
                    }),
            },
            created_at: vc.created_at,
            tags: vc.tags.clone(),
        }
    }
}

impl From<&JobRun> for JobRunView {
    fn from(jr: &JobRun) -> Self {
        JobRunView {
            id: jr.id.clone(),
            name: jr.name.clone(),
            virtual_cluster_id: jr.virtual_cluster_id.clone(),
            arn: jr.arn.clone(),
            state: jr.state.clone(),
            execution_role_arn: jr.execution_role_arn.clone(),
            release_label: jr.release_label.clone(),
            created_at: jr.created_at,
        }
    }
}

impl From<&ManagedEndpoint> for ManagedEndpointView {
    fn from(ep: &ManagedEndpoint) -> Self {
        ManagedEndpointView {
            id: ep.id.clone(),
            name: ep.name.clone(),
            virtual_cluster_id: ep.virtual_cluster_id.clone(),
            arn: ep.arn.clone(),
            state: ep.state.clone(),
            endpoint_type: ep.endpoint_type.clone(),
            release_label: ep.release_label.clone(),
            execution_role_arn: ep.execution_role_arn.clone(),
            certificate_arn: ep.certificate_arn.clone(),
            created_at: ep.created_at,
            tags: ep.tags.clone(),
        }
    }
}

impl From<&JobTemplate> for JobTemplateView {
    fn from(t: &JobTemplate) -> Self {
        JobTemplateView {
            id: t.id.clone(),
            name: t.name.clone(),
            arn: t.arn.clone(),
            created_at: t.created_at,
            kms_key_arn: t.kms_key_arn.clone(),
            tags: t.tags.clone(),
            job_template_data: t.job_template_data.clone(),
        }
    }
}

impl From<&SecurityConfiguration> for SecurityConfigurationView {
    fn from(sc: &SecurityConfiguration) -> Self {
        SecurityConfigurationView {
            id: sc.id.clone(),
            name: sc.name.clone(),
            arn: sc.arn.clone(),
            created_at: sc.created_at,
            tags: sc.tags.clone(),
            security_configuration_data: sc.security_configuration_data.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<EmrContainersStateView> for EmrContainersState {
    fn from(view: EmrContainersStateView) -> Self {
        EmrContainersState {
            virtual_clusters: view
                .virtual_clusters
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        VirtualCluster {
                            id: v.id,
                            name: v.name,
                            arn: v.arn,
                            state: v.state,
                            container_provider: ContainerProvider {
                                provider_type: v.container_provider.provider_type,
                                id: v.container_provider.id,
                                info: v.container_provider.info.map(|info| ContainerInfo {
                                    eks_info: info.eks_info.map(|eks| EksInfo {
                                        namespace: eks.namespace,
                                    }),
                                }),
                            },
                            created_at: v.created_at,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            job_runs: view
                .job_runs
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        JobRun {
                            id: v.id,
                            name: v.name,
                            virtual_cluster_id: v.virtual_cluster_id,
                            arn: v.arn,
                            state: v.state,
                            execution_role_arn: v.execution_role_arn,
                            release_label: v.release_label,
                            created_at: v.created_at,
                        },
                    )
                })
                .collect(),
            managed_endpoints: view
                .managed_endpoints
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        ManagedEndpoint {
                            id: v.id,
                            name: v.name,
                            virtual_cluster_id: v.virtual_cluster_id,
                            arn: v.arn,
                            state: v.state,
                            endpoint_type: v.endpoint_type,
                            release_label: v.release_label,
                            execution_role_arn: v.execution_role_arn,
                            certificate_arn: v.certificate_arn,
                            created_at: v.created_at,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            job_templates: view
                .job_templates
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        JobTemplate {
                            id: v.id,
                            name: v.name,
                            arn: v.arn,
                            created_at: v.created_at,
                            kms_key_arn: v.kms_key_arn,
                            tags: v.tags,
                            job_template_data: v.job_template_data,
                        },
                    )
                })
                .collect(),
            security_configurations: view
                .security_configurations
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        SecurityConfiguration {
                            id: v.id,
                            name: v.name,
                            arn: v.arn,
                            created_at: v.created_at,
                            tags: v.tags,
                            security_configuration_data: v.security_configuration_data,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for EmrContainersService {
    type StateView = EmrContainersStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        EmrContainersStateView::from(&*guard)
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
            *guard = EmrContainersState::from(view);
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
            let incoming = EmrContainersState::from(view);
            for (k, v) in incoming.virtual_clusters {
                guard.virtual_clusters.insert(k, v);
            }
            for (k, v) in incoming.job_runs {
                guard.job_runs.insert(k, v);
            }
            for (k, v) in incoming.managed_endpoints {
                guard.managed_endpoints.insert(k, v);
            }
            for (k, v) in incoming.job_templates {
                guard.job_templates.insert(k, v);
            }
            for (k, v) in incoming.security_configurations {
                guard.security_configurations.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
