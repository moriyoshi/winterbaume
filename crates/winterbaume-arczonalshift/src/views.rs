//! Serde-compatible view types for ARC Zonal Shift state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ArcZonalShiftService;
use crate::state::ArcZonalShiftState;
use crate::types::{ControlCondition, ManagedResource, PracticeRunConfiguration, ZonalShift};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArcZonalShiftStateView {
    #[serde(default)]
    pub zonal_shifts: HashMap<String, ZonalShiftView>,
    #[serde(default)]
    pub practice_run_configurations: HashMap<String, PracticeRunConfigurationView>,
    #[serde(default)]
    pub managed_resources: HashMap<String, ManagedResourceView>,
    #[serde(default)]
    pub autoshift_observer_notification_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZonalShiftView {
    pub zonal_shift_id: String,
    pub resource_identifier: String,
    pub away_from: String,
    pub comment: String,
    pub start_time: i64,
    pub expiry_time: i64,
    pub status: String,
    pub shift_type: String,
    #[serde(default)]
    pub practice_run_outcome: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PracticeRunConfigurationView {
    pub resource_identifier: String,
    #[serde(default)]
    pub blocking_alarms: Vec<ControlConditionView>,
    #[serde(default)]
    pub outcome_alarms: Vec<ControlConditionView>,
    #[serde(default)]
    pub blocked_windows: Vec<String>,
    #[serde(default)]
    pub allowed_windows: Vec<String>,
    #[serde(default)]
    pub blocked_dates: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlConditionView {
    pub alarm_identifier: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedResourceView {
    pub resource_identifier: String,
    pub name: String,
    pub arn: String,
    #[serde(default)]
    pub availability_zones: Vec<String>,
    #[serde(default)]
    pub applied_weights: HashMap<String, f32>,
    pub zonal_autoshift_status: String,
}

impl From<&ZonalShift> for ZonalShiftView {
    fn from(s: &ZonalShift) -> Self {
        Self {
            zonal_shift_id: s.zonal_shift_id.clone(),
            resource_identifier: s.resource_identifier.clone(),
            away_from: s.away_from.clone(),
            comment: s.comment.clone(),
            start_time: s.start_time,
            expiry_time: s.expiry_time,
            status: s.status.clone(),
            shift_type: s.shift_type.clone(),
            practice_run_outcome: s.practice_run_outcome.clone(),
        }
    }
}

impl From<ZonalShiftView> for ZonalShift {
    fn from(v: ZonalShiftView) -> Self {
        Self {
            zonal_shift_id: v.zonal_shift_id,
            resource_identifier: v.resource_identifier,
            away_from: v.away_from,
            comment: v.comment,
            start_time: v.start_time,
            expiry_time: v.expiry_time,
            status: v.status,
            shift_type: v.shift_type,
            practice_run_outcome: v.practice_run_outcome,
        }
    }
}

impl From<&ControlCondition> for ControlConditionView {
    fn from(c: &ControlCondition) -> Self {
        Self {
            alarm_identifier: c.alarm_identifier.clone(),
            r#type: c.r#type.clone(),
        }
    }
}

impl From<ControlConditionView> for ControlCondition {
    fn from(v: ControlConditionView) -> Self {
        Self {
            alarm_identifier: v.alarm_identifier,
            r#type: v.r#type,
        }
    }
}

impl From<&PracticeRunConfiguration> for PracticeRunConfigurationView {
    fn from(c: &PracticeRunConfiguration) -> Self {
        Self {
            resource_identifier: c.resource_identifier.clone(),
            blocking_alarms: c.blocking_alarms.iter().map(Into::into).collect(),
            outcome_alarms: c.outcome_alarms.iter().map(Into::into).collect(),
            blocked_windows: c.blocked_windows.clone(),
            allowed_windows: c.allowed_windows.clone(),
            blocked_dates: c.blocked_dates.clone(),
        }
    }
}

impl From<PracticeRunConfigurationView> for PracticeRunConfiguration {
    fn from(v: PracticeRunConfigurationView) -> Self {
        Self {
            resource_identifier: v.resource_identifier,
            blocking_alarms: v.blocking_alarms.into_iter().map(Into::into).collect(),
            outcome_alarms: v.outcome_alarms.into_iter().map(Into::into).collect(),
            blocked_windows: v.blocked_windows,
            allowed_windows: v.allowed_windows,
            blocked_dates: v.blocked_dates,
        }
    }
}

impl From<&ManagedResource> for ManagedResourceView {
    fn from(r: &ManagedResource) -> Self {
        Self {
            resource_identifier: r.resource_identifier.clone(),
            name: r.name.clone(),
            arn: r.arn.clone(),
            availability_zones: r.availability_zones.clone(),
            applied_weights: r.applied_weights.clone(),
            zonal_autoshift_status: r.zonal_autoshift_status.clone(),
        }
    }
}

impl From<ManagedResourceView> for ManagedResource {
    fn from(v: ManagedResourceView) -> Self {
        Self {
            resource_identifier: v.resource_identifier,
            name: v.name,
            arn: v.arn,
            availability_zones: v.availability_zones,
            applied_weights: v.applied_weights,
            zonal_autoshift_status: v.zonal_autoshift_status,
        }
    }
}

impl From<&ArcZonalShiftState> for ArcZonalShiftStateView {
    fn from(state: &ArcZonalShiftState) -> Self {
        Self {
            zonal_shifts: state
                .zonal_shifts
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            practice_run_configurations: state
                .practice_run_configurations
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            managed_resources: state
                .managed_resources
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            autoshift_observer_notification_status: state
                .autoshift_observer_notification_status
                .clone(),
        }
    }
}

impl From<ArcZonalShiftStateView> for ArcZonalShiftState {
    fn from(view: ArcZonalShiftStateView) -> Self {
        Self {
            zonal_shifts: view
                .zonal_shifts
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            practice_run_configurations: view
                .practice_run_configurations
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            managed_resources: view
                .managed_resources
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            autoshift_observer_notification_status: if view
                .autoshift_observer_notification_status
                .is_empty()
            {
                "DISABLED".to_string()
            } else {
                view.autoshift_observer_notification_status
            },
        }
    }
}

impl StatefulService for ArcZonalShiftService {
    type StateView = ArcZonalShiftStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ArcZonalShiftStateView::from(&*guard)
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
            *guard = ArcZonalShiftState::from(view);
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
            for (k, v) in view.zonal_shifts {
                guard.zonal_shifts.insert(k, v.into());
            }
            for (k, v) in view.practice_run_configurations {
                guard.practice_run_configurations.insert(k, v.into());
            }
            for (k, v) in view.managed_resources {
                guard.managed_resources.insert(k, v.into());
            }
            if !view.autoshift_observer_notification_status.is_empty() {
                guard.autoshift_observer_notification_status =
                    view.autoshift_observer_notification_status;
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
