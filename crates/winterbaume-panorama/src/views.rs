//! Serde-compatible view types for Panorama state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::PanoramaService;
use crate::state::PanoramaState;
use crate::types::{ApplicationInstance, Device, NodeFromTemplateJob};

/// Serializable view of the entire Panorama state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PanoramaStateView {
    #[serde(default)]
    pub application_instances: HashMap<String, ApplicationInstanceView>,
    #[serde(default)]
    pub devices: HashMap<String, DeviceView>,
    #[serde(default)]
    pub node_from_template_jobs: HashMap<String, NodeFromTemplateJobView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationInstanceView {
    pub application_instance_id: String,
    pub name: String,
    pub status: String,
    pub description: Option<String>,
    pub default_runtime_context_device: String,
    pub application_instance_id_to_replace: Option<String>,
    pub tags: HashMap<String, String>,
    pub arn: String,
    pub created_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceView {
    pub device_id: String,
    pub name: String,
    pub arn: String,
    pub status: String,
    pub description: Option<String>,
    pub tags: HashMap<String, String>,
    pub created_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeFromTemplateJobView {
    pub job_id: String,
    pub node_name: String,
    pub status: String,
    pub template_type: String,
    pub created_time: String,
}

impl From<&PanoramaState> for PanoramaStateView {
    fn from(state: &PanoramaState) -> Self {
        PanoramaStateView {
            application_instances: state
                .application_instances
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ApplicationInstanceView {
                            application_instance_id: v.application_instance_id.clone(),
                            name: v.name.clone(),
                            status: v.status.clone(),
                            description: v.description.clone(),
                            default_runtime_context_device: v
                                .default_runtime_context_device
                                .clone(),
                            application_instance_id_to_replace: v
                                .application_instance_id_to_replace
                                .clone(),
                            tags: v.tags.clone(),
                            arn: v.arn.clone(),
                            created_time: v.created_time.clone(),
                        },
                    )
                })
                .collect(),
            devices: state
                .devices
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        DeviceView {
                            device_id: v.device_id.clone(),
                            name: v.name.clone(),
                            arn: v.arn.clone(),
                            status: v.status.clone(),
                            description: v.description.clone(),
                            tags: v.tags.clone(),
                            created_time: v.created_time.clone(),
                        },
                    )
                })
                .collect(),
            node_from_template_jobs: state
                .node_from_template_jobs
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        NodeFromTemplateJobView {
                            job_id: v.job_id.clone(),
                            node_name: v.node_name.clone(),
                            status: v.status.clone(),
                            template_type: v.template_type.clone(),
                            created_time: v.created_time.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

impl From<PanoramaStateView> for PanoramaState {
    fn from(view: PanoramaStateView) -> Self {
        PanoramaState {
            application_instances: view
                .application_instances
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        ApplicationInstance {
                            application_instance_id: v.application_instance_id,
                            name: v.name,
                            status: v.status,
                            description: v.description,
                            default_runtime_context_device: v.default_runtime_context_device,
                            application_instance_id_to_replace: v
                                .application_instance_id_to_replace,
                            tags: v.tags,
                            arn: v.arn,
                            created_time: v.created_time.clone(),
                        },
                    )
                })
                .collect(),
            devices: view
                .devices
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Device {
                            device_id: v.device_id,
                            name: v.name,
                            arn: v.arn,
                            status: v.status,
                            description: v.description,
                            tags: v.tags,
                            created_time: v.created_time.clone(),
                        },
                    )
                })
                .collect(),
            node_from_template_jobs: view
                .node_from_template_jobs
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        NodeFromTemplateJob {
                            job_id: v.job_id,
                            node_name: v.node_name,
                            status: v.status,
                            template_type: v.template_type,
                            created_time: v.created_time.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

impl StatefulService for PanoramaService {
    type StateView = PanoramaStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        PanoramaStateView::from(&*guard)
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
            *guard = PanoramaState::from(view);
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
            for (k, v) in view.application_instances {
                guard.application_instances.insert(
                    k,
                    ApplicationInstance {
                        application_instance_id: v.application_instance_id,
                        name: v.name,
                        status: v.status,
                        description: v.description,
                        default_runtime_context_device: v.default_runtime_context_device,
                        application_instance_id_to_replace: v.application_instance_id_to_replace,
                        tags: v.tags,
                        arn: v.arn,
                        created_time: v.created_time,
                    },
                );
            }
            for (k, v) in view.devices {
                guard.devices.insert(
                    k,
                    Device {
                        device_id: v.device_id,
                        name: v.name,
                        arn: v.arn,
                        status: v.status,
                        description: v.description,
                        tags: v.tags,
                        created_time: v.created_time,
                    },
                );
            }
            for (k, v) in view.node_from_template_jobs {
                guard.node_from_template_jobs.insert(
                    k,
                    NodeFromTemplateJob {
                        job_id: v.job_id,
                        node_name: v.node_name,
                        status: v.status,
                        template_type: v.template_type,
                        created_time: v.created_time,
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
