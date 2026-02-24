//! Serde-compatible view types for Directory Service state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::DirectoryService;
use crate::state::DsState;
use crate::types::{
    Directory, DirectoryConnectSettings, DirectoryStage, DirectoryType, DirectoryVpcSettings,
};

/// Serializable view of the entire Directory Service state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DsStateView {
    /// Directories keyed by directory ID.
    #[serde(default)]
    pub directories: HashMap<String, DirectoryView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryView {
    pub directory_id: String,
    pub name: String,
    pub short_name: Option<String>,
    pub description: Option<String>,
    pub size: String,
    /// Directory type as string: "SimpleAD", "ADConnector", "MicrosoftAD"
    pub directory_type: String,
    pub alias: String,
    pub access_url: String,
    /// Directory stage as string: "Active", "Deleted", etc.
    pub stage: String,
    pub launch_time: DateTime<Utc>,
    pub stage_last_updated_date_time: DateTime<Utc>,
    #[serde(default)]
    pub dns_ip_addrs: Vec<String>,
    pub vpc_settings: Option<DirectoryVpcSettingsView>,
    pub connect_settings: Option<DirectoryConnectSettingsView>,
    pub ssoid_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryVpcSettingsView {
    pub vpc_id: String,
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    pub security_group_id: String,
    #[serde(default)]
    pub availability_zones: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryConnectSettingsView {
    pub vpc_id: String,
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    #[serde(default)]
    pub customer_dns_ips: Vec<String>,
    pub customer_user_name: String,
    pub security_group_id: String,
    #[serde(default)]
    pub availability_zones: Vec<String>,
    #[serde(default)]
    pub connect_ips: Vec<String>,
}

// --- From internal types to view types ---

impl From<&DsState> for DsStateView {
    fn from(state: &DsState) -> Self {
        DsStateView {
            directories: state
                .directories
                .iter()
                .map(|(k, v)| (k.clone(), DirectoryView::from(v)))
                .collect(),
        }
    }
}

impl From<&Directory> for DirectoryView {
    fn from(d: &Directory) -> Self {
        DirectoryView {
            directory_id: d.directory_id.clone(),
            name: d.name.clone(),
            short_name: d.short_name.clone(),
            description: d.description.clone(),
            size: d.size.clone(),
            directory_type: d.directory_type.as_str().to_string(),
            alias: d.alias.clone(),
            access_url: d.access_url.clone(),
            stage: d.stage.as_str().to_string(),
            launch_time: d.launch_time,
            stage_last_updated_date_time: d.stage_last_updated_date_time,
            dns_ip_addrs: d.dns_ip_addrs.clone(),
            vpc_settings: d.vpc_settings.as_ref().map(|vs| DirectoryVpcSettingsView {
                vpc_id: vs.vpc_id.clone(),
                subnet_ids: vs.subnet_ids.clone(),
                security_group_id: vs.security_group_id.clone(),
                availability_zones: vs.availability_zones.clone(),
            }),
            connect_settings: d
                .connect_settings
                .as_ref()
                .map(|cs| DirectoryConnectSettingsView {
                    vpc_id: cs.vpc_id.clone(),
                    subnet_ids: cs.subnet_ids.clone(),
                    customer_dns_ips: cs.customer_dns_ips.clone(),
                    customer_user_name: cs.customer_user_name.clone(),
                    security_group_id: cs.security_group_id.clone(),
                    availability_zones: cs.availability_zones.clone(),
                    connect_ips: cs.connect_ips.clone(),
                }),
            ssoid_enabled: d.ssoid_enabled,
        }
    }
}

fn parse_directory_type(s: &str) -> DirectoryType {
    match s {
        "ADConnector" => DirectoryType::ADConnector,
        "MicrosoftAD" => DirectoryType::MicrosoftAD,
        _ => DirectoryType::SimpleAD,
    }
}

fn parse_directory_stage(s: &str) -> DirectoryStage {
    match s {
        "Requested" => DirectoryStage::Requested,
        "Creating" => DirectoryStage::Creating,
        "Created" => DirectoryStage::Created,
        "Active" => DirectoryStage::Active,
        "Inoperable" => DirectoryStage::Inoperable,
        "Impaired" => DirectoryStage::Impaired,
        "Restoring" => DirectoryStage::Restoring,
        "RestoreFailed" => DirectoryStage::RestoreFailed,
        "Deleting" => DirectoryStage::Deleting,
        "Deleted" => DirectoryStage::Deleted,
        _ => DirectoryStage::Failed,
    }
}

// --- From view types to internal types ---

impl From<DsStateView> for DsState {
    fn from(view: DsStateView) -> Self {
        DsState {
            directories: view
                .directories
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Directory {
                            directory_id: v.directory_id,
                            name: v.name,
                            short_name: v.short_name,
                            description: v.description,
                            size: v.size,
                            directory_type: parse_directory_type(&v.directory_type),
                            alias: v.alias,
                            access_url: v.access_url,
                            stage: parse_directory_stage(&v.stage),
                            launch_time: v.launch_time,
                            stage_last_updated_date_time: v.stage_last_updated_date_time,
                            dns_ip_addrs: v.dns_ip_addrs,
                            vpc_settings: v.vpc_settings.map(|vs| DirectoryVpcSettings {
                                vpc_id: vs.vpc_id,
                                subnet_ids: vs.subnet_ids,
                                security_group_id: vs.security_group_id,
                                availability_zones: vs.availability_zones,
                            }),
                            connect_settings: v.connect_settings.map(|cs| {
                                DirectoryConnectSettings {
                                    vpc_id: cs.vpc_id,
                                    subnet_ids: cs.subnet_ids,
                                    customer_dns_ips: cs.customer_dns_ips,
                                    customer_user_name: cs.customer_user_name,
                                    security_group_id: cs.security_group_id,
                                    availability_zones: cs.availability_zones,
                                    connect_ips: cs.connect_ips,
                                }
                            }),
                            ssoid_enabled: v.ssoid_enabled,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for DirectoryService {
    type StateView = DsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        DsStateView::from(&*guard)
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
            *guard = DsState::from(view);
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
            let incoming = DsState::from(view);
            for (k, v) in incoming.directories {
                guard.directories.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
