//! Serde-compatible view types for AIOps state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::AIOpsService;
use crate::state::AIOpsState;
use crate::types::{CrossAccountConfiguration, EncryptionConfiguration, InvestigationGroup};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AIOpsStateView {
    #[serde(default)]
    pub investigation_groups: HashMap<String, InvestigationGroupView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestigationGroupView {
    pub name: String,
    pub arn: String,
    pub role_arn: String,
    #[serde(default)]
    pub encryption_configuration: Option<EncryptionConfigurationView>,
    #[serde(default)]
    pub retention_in_days: Option<i64>,
    #[serde(default)]
    pub chatbot_notification_channel: Option<HashMap<String, Vec<String>>>,
    #[serde(default)]
    pub tag_key_boundaries: Option<Vec<String>>,
    #[serde(default)]
    pub is_cloud_trail_event_history_enabled: Option<bool>,
    #[serde(default)]
    pub cross_account_configurations: Option<Vec<CrossAccountConfigurationView>>,
    pub created_by: String,
    pub created_at: i64,
    pub last_modified_by: String,
    pub last_modified_at: i64,
    #[serde(default)]
    pub policy: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfigurationView {
    #[serde(default, rename = "type")]
    pub r#type: Option<String>,
    #[serde(default)]
    pub kms_key_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossAccountConfigurationView {
    #[serde(default)]
    pub source_role_arn: Option<String>,
}

impl From<&InvestigationGroup> for InvestigationGroupView {
    fn from(g: &InvestigationGroup) -> Self {
        Self {
            name: g.name.clone(),
            arn: g.arn.clone(),
            role_arn: g.role_arn.clone(),
            encryption_configuration: g.encryption_configuration.as_ref().map(|e| {
                EncryptionConfigurationView {
                    r#type: e.r#type.clone(),
                    kms_key_id: e.kms_key_id.clone(),
                }
            }),
            retention_in_days: g.retention_in_days,
            chatbot_notification_channel: g.chatbot_notification_channel.clone(),
            tag_key_boundaries: g.tag_key_boundaries.clone(),
            is_cloud_trail_event_history_enabled: g.is_cloud_trail_event_history_enabled,
            cross_account_configurations: g.cross_account_configurations.as_ref().map(|cs| {
                cs.iter()
                    .map(|c| CrossAccountConfigurationView {
                        source_role_arn: c.source_role_arn.clone(),
                    })
                    .collect()
            }),
            created_by: g.created_by.clone(),
            created_at: g.created_at,
            last_modified_by: g.last_modified_by.clone(),
            last_modified_at: g.last_modified_at,
            policy: g.policy.clone(),
            tags: g.tags.clone(),
        }
    }
}

impl From<InvestigationGroupView> for InvestigationGroup {
    fn from(v: InvestigationGroupView) -> Self {
        Self {
            name: v.name,
            arn: v.arn,
            role_arn: v.role_arn,
            encryption_configuration: v.encryption_configuration.map(|e| EncryptionConfiguration {
                r#type: e.r#type,
                kms_key_id: e.kms_key_id,
            }),
            retention_in_days: v.retention_in_days,
            chatbot_notification_channel: v.chatbot_notification_channel,
            tag_key_boundaries: v.tag_key_boundaries,
            is_cloud_trail_event_history_enabled: v.is_cloud_trail_event_history_enabled,
            cross_account_configurations: v.cross_account_configurations.map(|cs| {
                cs.into_iter()
                    .map(|c| CrossAccountConfiguration {
                        source_role_arn: c.source_role_arn,
                    })
                    .collect()
            }),
            created_by: v.created_by,
            created_at: v.created_at,
            last_modified_by: v.last_modified_by,
            last_modified_at: v.last_modified_at,
            policy: v.policy,
            tags: v.tags,
        }
    }
}

impl From<&AIOpsState> for AIOpsStateView {
    fn from(state: &AIOpsState) -> Self {
        AIOpsStateView {
            investigation_groups: state
                .investigation_groups
                .iter()
                .map(|(k, v)| (k.clone(), InvestigationGroupView::from(v)))
                .collect(),
        }
    }
}

impl From<AIOpsStateView> for AIOpsState {
    fn from(view: AIOpsStateView) -> Self {
        AIOpsState {
            investigation_groups: view
                .investigation_groups
                .into_iter()
                .map(|(k, v)| (k, InvestigationGroup::from(v)))
                .collect(),
        }
    }
}

impl StatefulService for AIOpsService {
    type StateView = AIOpsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        AIOpsStateView::from(&*guard)
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
            *guard = AIOpsState::from(view);
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
            for (k, v) in view.investigation_groups {
                guard
                    .investigation_groups
                    .insert(k, InvestigationGroup::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
