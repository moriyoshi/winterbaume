//! Serde-compatible view types for MediaConnect state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::MediaConnectService;
use crate::state::MediaConnectState;
use crate::types::{Flow, FlowEntitlement, FlowOutput, FlowSource, FlowVpcInterface};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MediaConnectStateView {
    #[serde(default)]
    pub flows: HashMap<String, FlowView>,
    #[serde(default)]
    pub resource_tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowView {
    pub flow_arn: String,
    pub name: String,
    pub description: String,
    pub availability_zone: String,
    pub status: String,
    pub created_at: String,
    #[serde(default)]
    pub outputs: Vec<FlowOutputView>,
    #[serde(default)]
    pub sources: Vec<FlowSourceView>,
    #[serde(default)]
    pub vpc_interfaces: Vec<FlowVpcInterfaceView>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub entitlements: Vec<FlowEntitlementView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowEntitlementView {
    pub entitlement_arn: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub subscribers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowOutputView {
    pub output_arn: String,
    pub name: String,
    pub description: String,
    pub port: i32,
    pub protocol: String,
    pub destination: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowSourceView {
    pub source_arn: String,
    pub name: String,
    pub description: String,
    pub ingest_port: i32,
    pub protocol: String,
    pub whitelist_cidr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowVpcInterfaceView {
    pub name: String,
    pub role_arn: String,
    #[serde(default)]
    pub security_group_ids: Vec<String>,
    pub subnet_id: String,
    pub network_interface_type: String,
    #[serde(default)]
    pub network_interface_ids: Vec<String>,
}

fn parse_dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

// --- From internal types to view types ---

impl From<&MediaConnectState> for MediaConnectStateView {
    fn from(state: &MediaConnectState) -> Self {
        MediaConnectStateView {
            flows: state
                .flows
                .iter()
                .map(|(k, v)| (k.clone(), FlowView::from(v)))
                .collect(),
            resource_tags: state.resource_tags.clone(),
        }
    }
}

impl From<&Flow> for FlowView {
    fn from(v: &Flow) -> Self {
        FlowView {
            flow_arn: v.flow_arn.clone(),
            name: v.name.clone(),
            description: v.description.clone(),
            availability_zone: v.availability_zone.clone(),
            status: v.status.clone(),
            created_at: v.created_at.to_rfc3339(),
            outputs: v
                .outputs
                .iter()
                .map(|o| FlowOutputView {
                    output_arn: o.output_arn.clone(),
                    name: o.name.clone(),
                    description: o.description.clone(),
                    port: o.port,
                    protocol: o.protocol.clone(),
                    destination: o.destination.clone(),
                })
                .collect(),
            sources: v
                .sources
                .iter()
                .map(|s| FlowSourceView {
                    source_arn: s.source_arn.clone(),
                    name: s.name.clone(),
                    description: s.description.clone(),
                    ingest_port: s.ingest_port,
                    protocol: s.protocol.clone(),
                    whitelist_cidr: s.whitelist_cidr.clone(),
                })
                .collect(),
            vpc_interfaces: v
                .vpc_interfaces
                .iter()
                .map(|vpc| FlowVpcInterfaceView {
                    name: vpc.name.clone(),
                    role_arn: vpc.role_arn.clone(),
                    security_group_ids: vpc.security_group_ids.clone(),
                    subnet_id: vpc.subnet_id.clone(),
                    network_interface_type: vpc.network_interface_type.clone(),
                    network_interface_ids: vpc.network_interface_ids.clone(),
                })
                .collect(),
            tags: v.tags.clone(),
            entitlements: v
                .entitlements
                .iter()
                .map(|e| FlowEntitlementView {
                    entitlement_arn: e.entitlement_arn.clone(),
                    name: e.name.clone(),
                    description: e.description.clone(),
                    subscribers: e.subscribers.clone(),
                })
                .collect(),
        }
    }
}

fn flow_from_view(v: FlowView) -> Flow {
    Flow {
        flow_arn: v.flow_arn,
        name: v.name,
        description: v.description,
        availability_zone: v.availability_zone,
        status: v.status,
        created_at: parse_dt(&v.created_at),
        outputs: v
            .outputs
            .into_iter()
            .map(|o| FlowOutput {
                output_arn: o.output_arn,
                name: o.name,
                description: o.description,
                port: o.port,
                protocol: o.protocol,
                destination: o.destination,
            })
            .collect(),
        sources: v
            .sources
            .into_iter()
            .map(|s| FlowSource {
                source_arn: s.source_arn,
                name: s.name,
                description: s.description,
                ingest_port: s.ingest_port,
                protocol: s.protocol,
                whitelist_cidr: s.whitelist_cidr,
            })
            .collect(),
        vpc_interfaces: v
            .vpc_interfaces
            .into_iter()
            .map(|vpc| FlowVpcInterface {
                name: vpc.name,
                role_arn: vpc.role_arn,
                security_group_ids: vpc.security_group_ids,
                subnet_id: vpc.subnet_id,
                network_interface_type: vpc.network_interface_type,
                network_interface_ids: vpc.network_interface_ids,
            })
            .collect(),
        tags: v.tags,
        entitlements: v
            .entitlements
            .into_iter()
            .map(|e| FlowEntitlement {
                entitlement_arn: e.entitlement_arn,
                name: e.name,
                description: e.description,
                subscribers: e.subscribers,
            })
            .collect(),
    }
}

// --- From view types to internal types ---

impl From<MediaConnectStateView> for MediaConnectState {
    fn from(view: MediaConnectStateView) -> Self {
        MediaConnectState {
            flows: view
                .flows
                .into_iter()
                .map(|(k, v)| (k, flow_from_view(v)))
                .collect(),
            resource_tags: view.resource_tags,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for MediaConnectService {
    type StateView = MediaConnectStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        MediaConnectStateView::from(&*guard)
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
            *guard = MediaConnectState::from(view);
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
            for (k, v) in view.flows {
                guard.flows.insert(k, flow_from_view(v));
            }
            for (k, v) in view.resource_tags {
                guard.resource_tags.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
