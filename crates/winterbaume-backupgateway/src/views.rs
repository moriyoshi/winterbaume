use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::BackupGatewayService;
use crate::state::BackupGatewayState;
use crate::types::{
    BandwidthRateLimitInterval, Gateway, Hypervisor, MaintenanceStartTime, TagMapping,
    VirtualMachine,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackupGatewayStateView {
    #[serde(default)]
    pub gateways: HashMap<String, GatewayView>,
    #[serde(default)]
    pub hypervisors: HashMap<String, HypervisorView>,
    #[serde(default)]
    pub virtual_machines: HashMap<String, VirtualMachineView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GatewayView {
    pub arn: String,
    pub display_name: String,
    pub gateway_type: String,
    #[serde(default)]
    pub hypervisor_arn: Option<String>,
    pub last_seen_time: i64,
    pub software_version: String,
    #[serde(default)]
    pub vpc_endpoint: Option<String>,
    #[serde(default)]
    pub maintenance_start_time: Option<MaintenanceStartTimeView>,
    #[serde(default)]
    pub bandwidth_rate_limit_intervals: Vec<BandwidthRateLimitIntervalView>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MaintenanceStartTimeView {
    #[serde(default)]
    pub day_of_month: Option<i32>,
    #[serde(default)]
    pub day_of_week: Option<i32>,
    pub hour_of_day: i32,
    pub minute_of_hour: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BandwidthRateLimitIntervalView {
    #[serde(default)]
    pub average_upload_rate_limit_in_bits_per_sec: Option<i64>,
    #[serde(default)]
    pub days_of_week: Vec<i32>,
    pub start_hour_of_day: i32,
    pub start_minute_of_hour: i32,
    pub end_hour_of_day: i32,
    pub end_minute_of_hour: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HypervisorView {
    pub arn: String,
    pub host: String,
    pub name: String,
    pub state: String,
    #[serde(default)]
    pub kms_key_arn: Option<String>,
    #[serde(default)]
    pub log_group_arn: Option<String>,
    #[serde(default)]
    pub iam_role_arn: Option<String>,
    #[serde(default)]
    pub property_mappings: Vec<TagMappingView>,
    pub last_metadata_sync_status: String,
    #[serde(default)]
    pub last_metadata_sync_time: Option<i64>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagMappingView {
    pub aws_tag_key: String,
    pub aws_tag_value: String,
    pub vmware_category: String,
    pub vmware_tag_name: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VirtualMachineView {
    pub resource_arn: String,
    pub host_name: String,
    pub hypervisor_arn: String,
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub last_backup_date: Option<i64>,
}

macro_rules! basic_from {
    ($view:ident, $domain:ident { $($field:ident),* $(,)? }) => {
        impl From<&$domain> for $view {
            fn from(s: &$domain) -> Self { Self { $($field: s.$field.clone(),)* } }
        }
        impl From<$view> for $domain {
            fn from(v: $view) -> Self { Self { $($field: v.$field,)* } }
        }
    };
}

basic_from!(
    MaintenanceStartTimeView,
    MaintenanceStartTime {
        day_of_month,
        day_of_week,
        hour_of_day,
        minute_of_hour
    }
);

basic_from!(
    BandwidthRateLimitIntervalView,
    BandwidthRateLimitInterval {
        average_upload_rate_limit_in_bits_per_sec,
        days_of_week,
        start_hour_of_day,
        start_minute_of_hour,
        end_hour_of_day,
        end_minute_of_hour
    }
);

basic_from!(
    TagMappingView,
    TagMapping {
        aws_tag_key,
        aws_tag_value,
        vmware_category,
        vmware_tag_name
    }
);

basic_from!(
    VirtualMachineView,
    VirtualMachine {
        resource_arn,
        host_name,
        hypervisor_arn,
        name,
        path,
        last_backup_date
    }
);

impl From<&Gateway> for GatewayView {
    fn from(g: &Gateway) -> Self {
        Self {
            arn: g.arn.clone(),
            display_name: g.display_name.clone(),
            gateway_type: g.gateway_type.clone(),
            hypervisor_arn: g.hypervisor_arn.clone(),
            last_seen_time: g.last_seen_time,
            software_version: g.software_version.clone(),
            vpc_endpoint: g.vpc_endpoint.clone(),
            maintenance_start_time: g.maintenance_start_time.as_ref().map(Into::into),
            bandwidth_rate_limit_intervals: g
                .bandwidth_rate_limit_intervals
                .iter()
                .map(Into::into)
                .collect(),
            tags: g.tags.clone(),
        }
    }
}

impl From<GatewayView> for Gateway {
    fn from(v: GatewayView) -> Self {
        Self {
            arn: v.arn,
            display_name: v.display_name,
            gateway_type: v.gateway_type,
            hypervisor_arn: v.hypervisor_arn,
            last_seen_time: v.last_seen_time,
            software_version: v.software_version,
            vpc_endpoint: v.vpc_endpoint,
            maintenance_start_time: v.maintenance_start_time.map(Into::into),
            bandwidth_rate_limit_intervals: v
                .bandwidth_rate_limit_intervals
                .into_iter()
                .map(Into::into)
                .collect(),
            tags: v.tags,
        }
    }
}

impl From<&Hypervisor> for HypervisorView {
    fn from(h: &Hypervisor) -> Self {
        Self {
            arn: h.arn.clone(),
            host: h.host.clone(),
            name: h.name.clone(),
            state: h.state.clone(),
            kms_key_arn: h.kms_key_arn.clone(),
            log_group_arn: h.log_group_arn.clone(),
            iam_role_arn: h.iam_role_arn.clone(),
            property_mappings: h.property_mappings.iter().map(Into::into).collect(),
            last_metadata_sync_status: h.last_metadata_sync_status.clone(),
            last_metadata_sync_time: h.last_metadata_sync_time,
            tags: h.tags.clone(),
        }
    }
}

impl From<HypervisorView> for Hypervisor {
    fn from(v: HypervisorView) -> Self {
        Self {
            arn: v.arn,
            host: v.host,
            name: v.name,
            state: v.state,
            kms_key_arn: v.kms_key_arn,
            log_group_arn: v.log_group_arn,
            iam_role_arn: v.iam_role_arn,
            property_mappings: v.property_mappings.into_iter().map(Into::into).collect(),
            last_metadata_sync_status: v.last_metadata_sync_status,
            last_metadata_sync_time: v.last_metadata_sync_time,
            tags: v.tags,
        }
    }
}

impl From<&BackupGatewayState> for BackupGatewayStateView {
    fn from(state: &BackupGatewayState) -> Self {
        Self {
            gateways: state
                .gateways
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            hypervisors: state
                .hypervisors
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            virtual_machines: state
                .virtual_machines
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
        }
    }
}

impl From<BackupGatewayStateView> for BackupGatewayState {
    fn from(view: BackupGatewayStateView) -> Self {
        Self {
            gateways: view
                .gateways
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            hypervisors: view
                .hypervisors
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            virtual_machines: view
                .virtual_machines
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

impl StatefulService for BackupGatewayService {
    type StateView = BackupGatewayStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        BackupGatewayStateView::from(&*guard)
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
            *guard = BackupGatewayState::from(view);
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
            for (k, v) in view.gateways {
                guard.gateways.insert(k, v.into());
            }
            for (k, v) in view.hypervisors {
                guard.hypervisors.insert(k, v.into());
            }
            for (k, v) in view.virtual_machines {
                guard.virtual_machines.insert(k, v.into());
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
