//! Serde-compatible view types for EC2 Instance Connect state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::Ec2InstanceConnectService;
use crate::state::{Ec2InstanceConnectEndpoint, Ec2InstanceConnectState};

/// Serializable view of the EC2 Instance Connect state.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Ec2InstanceConnectStateView {
    /// Endpoints keyed by instance connect endpoint ID.
    #[serde(default)]
    pub endpoints: HashMap<String, EndpointView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointView {
    pub instance_connect_endpoint_id: String,
    pub subnet_id: String,
    pub vpc_id: String,
    pub security_group_ids: Vec<String>,
    pub state: String,
    #[serde(default)]
    pub dns_name: Option<String>,
    #[serde(default)]
    pub fips_dns_name: Option<String>,
    pub network_interface_ids: Vec<String>,
    pub owner_id: String,
    pub availability_zone: String,
    pub created_at: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

// --- From internal types to view types ---

impl From<&Ec2InstanceConnectState> for Ec2InstanceConnectStateView {
    fn from(state: &Ec2InstanceConnectState) -> Self {
        Ec2InstanceConnectStateView {
            endpoints: state
                .endpoints
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        EndpointView {
                            instance_connect_endpoint_id: v.instance_connect_endpoint_id.clone(),
                            subnet_id: v.subnet_id.clone(),
                            vpc_id: v.vpc_id.clone(),
                            security_group_ids: v.security_group_ids.clone(),
                            state: v.state.clone(),
                            dns_name: v.dns_name.clone(),
                            fips_dns_name: v.fips_dns_name.clone(),
                            network_interface_ids: v.network_interface_ids.clone(),
                            owner_id: v.owner_id.clone(),
                            availability_zone: v.availability_zone.clone(),
                            created_at: v.created_at.clone(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<Ec2InstanceConnectStateView> for Ec2InstanceConnectState {
    fn from(view: Ec2InstanceConnectStateView) -> Self {
        Ec2InstanceConnectState {
            endpoints: view
                .endpoints
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Ec2InstanceConnectEndpoint {
                            instance_connect_endpoint_id: v.instance_connect_endpoint_id,
                            subnet_id: v.subnet_id,
                            vpc_id: v.vpc_id,
                            security_group_ids: v.security_group_ids,
                            state: v.state,
                            dns_name: v.dns_name,
                            fips_dns_name: v.fips_dns_name,
                            network_interface_ids: v.network_interface_ids,
                            owner_id: v.owner_id,
                            availability_zone: v.availability_zone,
                            created_at: v.created_at,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for Ec2InstanceConnectService {
    type StateView = Ec2InstanceConnectStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        Ec2InstanceConnectStateView::from(&*guard)
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
            *guard = Ec2InstanceConnectState::from(view);
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
            let incoming = Ec2InstanceConnectState::from(view);
            for (k, v) in incoming.endpoints {
                guard.endpoints.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
