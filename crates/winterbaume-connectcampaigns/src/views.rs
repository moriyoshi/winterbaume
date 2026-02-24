//! Serde-compatible view types for Connect Campaigns state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ConnectCampaignsService;
use crate::state::ConnectCampaignsState;

/// Serializable view of the entire Connect Campaigns state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectCampaignsStateView {
    /// Campaigns keyed by campaign ID.
    #[serde(default)]
    pub campaigns: HashMap<String, CampaignView>,
    /// Instance configs keyed by connect instance ID.
    #[serde(default)]
    pub instance_configs: HashMap<String, InstanceConfigView>,
    /// Instance onboarding jobs keyed by connect instance ID.
    #[serde(default)]
    pub instance_onboarding_jobs: HashMap<String, InstanceOnboardingJobView>,
    /// Tags keyed by ARN.
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub connect_instance_id: String,
    pub dialer_config: serde_json::Value,
    pub outbound_call_config: serde_json::Value,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceConfigView {
    pub connect_instance_id: String,
    pub encryption_enabled: bool,
    pub encryption_type: Option<String>,
    pub key_arn: Option<String>,
    pub service_linked_role_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceOnboardingJobView {
    pub connect_instance_id: String,
    pub status: String,
}

// --- From internal types to view types ---

impl From<&ConnectCampaignsState> for ConnectCampaignsStateView {
    fn from(state: &ConnectCampaignsState) -> Self {
        ConnectCampaignsStateView {
            campaigns: state
                .campaigns
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        CampaignView {
                            id: v.id.clone(),
                            arn: v.arn.clone(),
                            name: v.name.clone(),
                            connect_instance_id: v.connect_instance_id.clone(),
                            dialer_config: v.dialer_config.clone(),
                            outbound_call_config: v.outbound_call_config.clone(),
                            tags: v.tags.clone(),
                            state: v.state.as_str().to_string(),
                        },
                    )
                })
                .collect(),
            instance_configs: state
                .instance_configs
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        InstanceConfigView {
                            connect_instance_id: v.connect_instance_id.clone(),
                            encryption_enabled: v.encryption_enabled,
                            encryption_type: v.encryption_type.clone(),
                            key_arn: v.key_arn.clone(),
                            service_linked_role_arn: v.service_linked_role_arn.clone(),
                        },
                    )
                })
                .collect(),
            instance_onboarding_jobs: state
                .instance_onboarding_jobs
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        InstanceOnboardingJobView {
                            connect_instance_id: v.connect_instance_id.clone(),
                            status: v.status.clone(),
                        },
                    )
                })
                .collect(),
            tags: state.tags.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<ConnectCampaignsStateView> for ConnectCampaignsState {
    fn from(view: ConnectCampaignsStateView) -> Self {
        use crate::types::{Campaign, CampaignState, InstanceConfig, InstanceOnboardingJob};

        let campaigns = view
            .campaigns
            .into_iter()
            .map(|(k, v)| {
                let state = match v.state.as_str() {
                    "Running" => CampaignState::Running,
                    "Paused" => CampaignState::Paused,
                    "Stopped" => CampaignState::Stopped,
                    _ => CampaignState::Initialized,
                };
                (
                    k,
                    Campaign {
                        id: v.id,
                        arn: v.arn,
                        name: v.name,
                        connect_instance_id: v.connect_instance_id,
                        dialer_config: v.dialer_config,
                        outbound_call_config: v.outbound_call_config,
                        tags: v.tags,
                        state,
                    },
                )
            })
            .collect();

        let instance_configs = view
            .instance_configs
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    InstanceConfig {
                        connect_instance_id: v.connect_instance_id,
                        encryption_enabled: v.encryption_enabled,
                        encryption_type: v.encryption_type,
                        key_arn: v.key_arn,
                        service_linked_role_arn: v.service_linked_role_arn,
                    },
                )
            })
            .collect();

        let instance_onboarding_jobs = view
            .instance_onboarding_jobs
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    InstanceOnboardingJob {
                        connect_instance_id: v.connect_instance_id,
                        status: v.status,
                    },
                )
            })
            .collect();

        ConnectCampaignsState {
            campaigns,
            instance_configs,
            instance_onboarding_jobs,
            tags: view.tags,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for ConnectCampaignsService {
    type StateView = ConnectCampaignsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ConnectCampaignsStateView::from(&*guard)
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
            *guard = ConnectCampaignsState::from(view);
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
            let incoming = ConnectCampaignsState::from(view);
            for (k, v) in incoming.campaigns {
                guard.campaigns.insert(k, v);
            }
            for (k, v) in incoming.instance_configs {
                guard.instance_configs.insert(k, v);
            }
            for (k, v) in incoming.instance_onboarding_jobs {
                guard.instance_onboarding_jobs.insert(k, v);
            }
            for (k, v) in incoming.tags {
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
