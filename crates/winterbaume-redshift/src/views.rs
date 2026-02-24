//! Serde-compatible view types for Redshift state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::RedshiftService;
use crate::state::RedshiftState;
use crate::types::{
    RedshiftAuthenticationProfile, RedshiftCluster, RedshiftEventSubscription,
    RedshiftHsmClientCertificate, RedshiftHsmConfiguration, RedshiftParameterGroup,
    RedshiftPartnerIntegration, RedshiftReservedNode, RedshiftResourcePolicy,
    RedshiftScheduledAction, RedshiftSecurityGroup, RedshiftSnapshot, RedshiftSnapshotCopyGrant,
    RedshiftSnapshotSchedule, RedshiftSubnetGroup, RedshiftTableRestoreStatus, RedshiftUsageLimit,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RedshiftStateView {
    #[serde(default)]
    pub clusters: HashMap<String, RedshiftCluster>,
    #[serde(default)]
    pub subnet_groups: HashMap<String, RedshiftSubnetGroup>,
    #[serde(default)]
    pub parameter_groups: HashMap<String, RedshiftParameterGroup>,
    #[serde(default)]
    pub security_groups: HashMap<String, RedshiftSecurityGroup>,
    #[serde(default)]
    pub snapshots: HashMap<String, RedshiftSnapshot>,
    #[serde(default)]
    pub snapshot_copy_grants: HashMap<String, RedshiftSnapshotCopyGrant>,
    #[serde(default)]
    pub tags_by_arn: HashMap<String, Vec<(String, String)>>,
    #[serde(default)]
    pub event_subscriptions: HashMap<String, RedshiftEventSubscription>,
    #[serde(default)]
    pub hsm_client_certificates: HashMap<String, RedshiftHsmClientCertificate>,
    #[serde(default)]
    pub hsm_configurations: HashMap<String, RedshiftHsmConfiguration>,
    #[serde(default)]
    pub authentication_profiles: HashMap<String, RedshiftAuthenticationProfile>,
    #[serde(default)]
    pub usage_limits: HashMap<String, RedshiftUsageLimit>,
    #[serde(default)]
    pub snapshot_schedules: HashMap<String, RedshiftSnapshotSchedule>,
    #[serde(default)]
    pub scheduled_actions: HashMap<String, RedshiftScheduledAction>,
    #[serde(default)]
    pub resource_policies: HashMap<String, RedshiftResourcePolicy>,
    #[serde(default)]
    pub partner_integrations: Vec<RedshiftPartnerIntegration>,
    #[serde(default)]
    pub aqua_configurations: HashMap<String, (String, String)>,
    #[serde(default)]
    pub reserved_nodes: HashMap<String, RedshiftReservedNode>,
    #[serde(default)]
    pub table_restore_statuses: HashMap<String, RedshiftTableRestoreStatus>,
}

impl From<&RedshiftState> for RedshiftStateView {
    fn from(state: &RedshiftState) -> Self {
        Self {
            clusters: state.clusters.clone(),
            subnet_groups: state.subnet_groups.clone(),
            parameter_groups: state.parameter_groups.clone(),
            security_groups: state.security_groups.clone(),
            snapshots: state.snapshots.clone(),
            snapshot_copy_grants: state.snapshot_copy_grants.clone(),
            tags_by_arn: state.tags_by_arn.clone(),
            event_subscriptions: state.event_subscriptions.clone(),
            hsm_client_certificates: state.hsm_client_certificates.clone(),
            hsm_configurations: state.hsm_configurations.clone(),
            authentication_profiles: state.authentication_profiles.clone(),
            usage_limits: state.usage_limits.clone(),
            snapshot_schedules: state.snapshot_schedules.clone(),
            scheduled_actions: state.scheduled_actions.clone(),
            resource_policies: state.resource_policies.clone(),
            partner_integrations: state.partner_integrations.clone(),
            aqua_configurations: state.aqua_configurations.clone(),
            reserved_nodes: state.reserved_nodes.clone(),
            table_restore_statuses: state.table_restore_statuses.clone(),
        }
    }
}

impl From<RedshiftStateView> for RedshiftState {
    fn from(view: RedshiftStateView) -> Self {
        Self {
            clusters: view.clusters,
            subnet_groups: view.subnet_groups,
            parameter_groups: view.parameter_groups,
            security_groups: view.security_groups,
            snapshots: view.snapshots,
            snapshot_copy_grants: view.snapshot_copy_grants,
            tags_by_arn: view.tags_by_arn,
            event_subscriptions: view.event_subscriptions,
            hsm_client_certificates: view.hsm_client_certificates,
            hsm_configurations: view.hsm_configurations,
            authentication_profiles: view.authentication_profiles,
            usage_limits: view.usage_limits,
            snapshot_schedules: view.snapshot_schedules,
            scheduled_actions: view.scheduled_actions,
            resource_policies: view.resource_policies,
            partner_integrations: view.partner_integrations,
            aqua_configurations: view.aqua_configurations,
            reserved_nodes: view.reserved_nodes,
            table_restore_statuses: view.table_restore_statuses,
        }
    }
}

// Re-export sub-view types needed by downstream code
pub use crate::types::{
    Ec2SecurityGroupIngress as Ec2SecurityGroupIngressView, IpRangeIngress as IpRangeIngressView,
    RedshiftAuthenticationProfile as RedshiftAuthenticationProfileView,
    RedshiftCluster as RedshiftClusterView,
    RedshiftEventSubscription as RedshiftEventSubscriptionView,
    RedshiftHsmClientCertificate as RedshiftHsmClientCertificateView,
    RedshiftHsmConfiguration as RedshiftHsmConfigurationView,
    RedshiftParameter as RedshiftParameterView,
    RedshiftParameterGroup as RedshiftParameterGroupView,
    RedshiftPartnerIntegration as RedshiftPartnerIntegrationView,
    RedshiftResourcePolicy as RedshiftResourcePolicyView,
    RedshiftScheduledAction as RedshiftScheduledActionView,
    RedshiftSecurityGroup as RedshiftSecurityGroupView, RedshiftSnapshot as RedshiftSnapshotView,
    RedshiftSnapshotCopyGrant as RedshiftSnapshotCopyGrantView,
    RedshiftSnapshotSchedule as RedshiftSnapshotScheduleView,
    RedshiftSubnetGroup as RedshiftSubnetGroupView,
    RedshiftTableRestoreStatus as RedshiftTableRestoreStatusView,
    RedshiftUsageLimit as RedshiftUsageLimitView,
};

impl StatefulService for RedshiftService {
    type StateView = RedshiftStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        RedshiftStateView::from(&*guard)
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
            *guard = RedshiftState::from(view);
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
            for (k, v) in view.clusters {
                guard.clusters.insert(k, v);
            }
            for (k, v) in view.subnet_groups {
                guard.subnet_groups.insert(k, v);
            }
            for (k, v) in view.parameter_groups {
                guard.parameter_groups.insert(k, v);
            }
            for (k, v) in view.security_groups {
                guard.security_groups.insert(k, v);
            }
            for (k, v) in view.snapshots {
                guard.snapshots.insert(k, v);
            }
            for (k, v) in view.snapshot_copy_grants {
                guard.snapshot_copy_grants.insert(k, v);
            }
            for (k, v) in view.tags_by_arn {
                guard.tags_by_arn.insert(k, v);
            }
            for (k, v) in view.event_subscriptions {
                guard.event_subscriptions.insert(k, v);
            }
            for (k, v) in view.hsm_client_certificates {
                guard.hsm_client_certificates.insert(k, v);
            }
            for (k, v) in view.hsm_configurations {
                guard.hsm_configurations.insert(k, v);
            }
            for (k, v) in view.authentication_profiles {
                guard.authentication_profiles.insert(k, v);
            }
            for (k, v) in view.usage_limits {
                guard.usage_limits.insert(k, v);
            }
            for (k, v) in view.snapshot_schedules {
                guard.snapshot_schedules.insert(k, v);
            }
            for (k, v) in view.scheduled_actions {
                guard.scheduled_actions.insert(k, v);
            }
            for (k, v) in view.resource_policies {
                guard.resource_policies.insert(k, v);
            }
            for pi in view.partner_integrations {
                guard.partner_integrations.push(pi);
            }
            for (k, v) in view.aqua_configurations {
                guard.aqua_configurations.insert(k, v);
            }
            for (k, v) in view.reserved_nodes {
                guard.reserved_nodes.insert(k, v);
            }
            for (k, v) in view.table_restore_statuses {
                guard.table_restore_statuses.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
