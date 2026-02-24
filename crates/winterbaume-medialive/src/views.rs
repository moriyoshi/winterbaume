//! Serde-compatible view types for MediaLive state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::MediaLiveService;
use crate::state::MediaLiveState;
use crate::types::{MediaLiveChannel, MediaLiveInput};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MediaLiveStateView {
    #[serde(default)]
    pub channels: Vec<MediaLiveChannelView>,
    #[serde(default)]
    pub inputs: Vec<MediaLiveInputView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaLiveChannelView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub state: String,
    pub channel_class: String,
    pub pipelines_running_count: i32,
    pub role_arn: String,
    pub input_attachments: Value,
    pub destinations: Value,
    pub encoder_settings: Value,
    pub input_specification: Value,
    pub log_level: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    /// `cdi_input_specification` nested block.
    #[serde(default)]
    pub cdi_input_specification: Option<Value>,
    /// `maintenance` nested block.
    #[serde(default)]
    pub maintenance: Option<Value>,
    /// `vpc` nested block.
    #[serde(default)]
    pub vpc: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaLiveInputView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub state: String,
    pub input_class: String,
    pub input_source_type: String,
    pub input_type: String,
    pub role_arn: String,
    #[serde(default)]
    pub attached_channels: Vec<String>,
    pub destinations: Value,
    pub input_devices: Value,
    pub media_connect_flows: Value,
    #[serde(default)]
    pub security_groups: Vec<String>,
    pub sources: Value,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    /// `vpc` nested block for medialive input.
    #[serde(default)]
    pub vpc: Option<Value>,
}

// --- From internal types to view types ---

impl From<&MediaLiveState> for MediaLiveStateView {
    fn from(state: &MediaLiveState) -> Self {
        MediaLiveStateView {
            channels: state
                .channels
                .iter()
                .map(|ch| MediaLiveChannelView {
                    id: ch.id.clone(),
                    arn: ch.arn.clone(),
                    name: ch.name.clone(),
                    state: ch.state.clone(),
                    channel_class: ch.channel_class.clone(),
                    pipelines_running_count: ch.pipelines_running_count,
                    role_arn: ch.role_arn.clone(),
                    input_attachments: ch.input_attachments.clone(),
                    destinations: ch.destinations.clone(),
                    encoder_settings: ch.encoder_settings.clone(),
                    input_specification: ch.input_specification.clone(),
                    log_level: ch.log_level.clone(),
                    tags: ch.tags.clone(),
                    cdi_input_specification: None,
                    maintenance: None,
                    vpc: None,
                })
                .collect(),
            inputs: state
                .inputs
                .iter()
                .map(|inp| MediaLiveInputView {
                    id: inp.id.clone(),
                    arn: inp.arn.clone(),
                    name: inp.name.clone(),
                    state: inp.state.clone(),
                    input_class: inp.input_class.clone(),
                    input_source_type: inp.input_source_type.clone(),
                    input_type: inp.input_type.clone(),
                    role_arn: inp.role_arn.clone(),
                    attached_channels: inp.attached_channels.clone(),
                    destinations: inp.destinations.clone(),
                    input_devices: inp.input_devices.clone(),
                    media_connect_flows: inp.media_connect_flows.clone(),
                    security_groups: inp.security_groups.clone(),
                    sources: inp.sources.clone(),
                    tags: inp.tags.clone(),
                    vpc: None,
                })
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<MediaLiveStateView> for MediaLiveState {
    fn from(view: MediaLiveStateView) -> Self {
        MediaLiveState {
            channels: view
                .channels
                .into_iter()
                .map(|ch| MediaLiveChannel {
                    id: ch.id,
                    arn: ch.arn,
                    name: ch.name,
                    state: ch.state,
                    channel_class: ch.channel_class,
                    pipelines_running_count: ch.pipelines_running_count,
                    role_arn: ch.role_arn,
                    input_attachments: ch.input_attachments,
                    destinations: ch.destinations,
                    encoder_settings: ch.encoder_settings,
                    input_specification: ch.input_specification,
                    log_level: ch.log_level,
                    tags: ch.tags,
                })
                .collect(),
            inputs: view
                .inputs
                .into_iter()
                .map(|inp| MediaLiveInput {
                    id: inp.id,
                    arn: inp.arn,
                    name: inp.name,
                    state: inp.state,
                    input_class: inp.input_class,
                    input_source_type: inp.input_source_type,
                    input_type: inp.input_type,
                    role_arn: inp.role_arn,
                    attached_channels: inp.attached_channels,
                    destinations: inp.destinations,
                    input_devices: inp.input_devices,
                    media_connect_flows: inp.media_connect_flows,
                    security_groups: inp.security_groups,
                    sources: inp.sources,
                    tags: inp.tags,
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for MediaLiveService {
    type StateView = MediaLiveStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        MediaLiveStateView::from(&*guard)
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
            *guard = MediaLiveState::from(view);
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
            let incoming = MediaLiveState::from(view);
            for ch in incoming.channels {
                if let Some(existing) = guard.channels.iter_mut().find(|c| c.id == ch.id) {
                    *existing = ch;
                } else {
                    guard.channels.push(ch);
                }
            }
            for inp in incoming.inputs {
                if let Some(existing) = guard.inputs.iter_mut().find(|i| i.id == inp.id) {
                    *existing = inp;
                } else {
                    guard.inputs.push(inp);
                }
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
