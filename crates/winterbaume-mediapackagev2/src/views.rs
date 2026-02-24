//! Serde-compatible view types for MediaPackageV2 state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::MediaPackageV2Service;
use crate::state::MediaPackageV2State;

/// Serializable view of a MediaPackage V2 channel.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChannelView {
    pub channel_name: String,
    pub channel_group_name: String,
    pub arn: String,
    pub description: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub created_at: String,
    pub modified_at: String,
    pub e_tag: String,
}

/// Serializable view of a MediaPackage V2 channel group.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChannelGroupView {
    pub channel_group_name: String,
    pub arn: String,
    pub egress_domain: String,
    pub description: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub created_at: String,
    pub modified_at: String,
    pub e_tag: String,
    #[serde(default)]
    pub channels: HashMap<String, ChannelView>,
}

/// Serializable view of the entire MediaPackageV2 state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MediaPackageV2StateView {
    #[serde(default)]
    pub channel_groups: HashMap<String, ChannelGroupView>,
}

// --- From internal types to view types ---

impl From<&crate::types::Channel> for ChannelView {
    fn from(ch: &crate::types::Channel) -> Self {
        ChannelView {
            channel_name: ch.channel_name.clone(),
            channel_group_name: ch.channel_group_name.clone(),
            arn: ch.arn.clone(),
            description: ch.description.clone(),
            tags: ch.tags.clone(),
            created_at: ch.created_at.clone(),
            modified_at: ch.modified_at.clone(),
            e_tag: ch.e_tag.clone(),
        }
    }
}

impl From<&crate::types::ChannelGroup> for ChannelGroupView {
    fn from(cg: &crate::types::ChannelGroup) -> Self {
        ChannelGroupView {
            channel_group_name: cg.channel_group_name.clone(),
            arn: cg.arn.clone(),
            egress_domain: cg.egress_domain.clone(),
            description: cg.description.clone(),
            tags: cg.tags.clone(),
            created_at: cg.created_at.clone(),
            modified_at: cg.modified_at.clone(),
            e_tag: cg.e_tag.clone(),
            channels: cg
                .channels
                .iter()
                .map(|(k, v)| (k.clone(), ChannelView::from(v)))
                .collect(),
        }
    }
}

impl From<&MediaPackageV2State> for MediaPackageV2StateView {
    fn from(state: &MediaPackageV2State) -> Self {
        MediaPackageV2StateView {
            channel_groups: state
                .channel_groups
                .iter()
                .map(|(k, v)| (k.clone(), ChannelGroupView::from(v)))
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<ChannelView> for crate::types::Channel {
    fn from(v: ChannelView) -> Self {
        crate::types::Channel {
            channel_name: v.channel_name,
            channel_group_name: v.channel_group_name,
            arn: v.arn,
            description: v.description,
            tags: v.tags,
            created_at: v.created_at,
            modified_at: v.modified_at,
            e_tag: v.e_tag,
        }
    }
}

impl From<ChannelGroupView> for crate::types::ChannelGroup {
    fn from(v: ChannelGroupView) -> Self {
        crate::types::ChannelGroup {
            channel_group_name: v.channel_group_name,
            arn: v.arn,
            egress_domain: v.egress_domain,
            description: v.description,
            tags: v.tags,
            created_at: v.created_at,
            modified_at: v.modified_at,
            e_tag: v.e_tag,
            channels: v
                .channels
                .into_iter()
                .map(|(k, cv)| (k, crate::types::Channel::from(cv)))
                .collect(),
        }
    }
}

impl From<MediaPackageV2StateView> for MediaPackageV2State {
    fn from(view: MediaPackageV2StateView) -> Self {
        MediaPackageV2State {
            channel_groups: view
                .channel_groups
                .into_iter()
                .map(|(k, v)| (k, crate::types::ChannelGroup::from(v)))
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for MediaPackageV2Service {
    type StateView = MediaPackageV2StateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        MediaPackageV2StateView::from(&*guard)
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
            *guard = MediaPackageV2State::from(view);
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
            for (name, cg_view) in view.channel_groups {
                guard
                    .channel_groups
                    .insert(name, crate::types::ChannelGroup::from(cg_view));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
