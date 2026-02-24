//! Serde-compatible view types for IVS state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::IvsService;
use crate::state::IvsState;
use crate::types::{
    Channel, PlaybackKeyPair, PlaybackRestrictionPolicy, RecordingConfiguration,
    RecordingDestination, StreamKey,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IvsStateView {
    #[serde(default)]
    pub channels: HashMap<String, ChannelView>,
    #[serde(default)]
    pub stream_keys: HashMap<String, StreamKeyView>,
    #[serde(default)]
    pub recording_configurations: HashMap<String, RecordingConfigurationView>,
    #[serde(default)]
    pub playback_key_pairs: HashMap<String, PlaybackKeyPairView>,
    #[serde(default)]
    pub playback_restriction_policies: HashMap<String, PlaybackRestrictionPolicyView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelView {
    pub arn: String,
    pub name: String,
    pub latency_mode: String,
    pub channel_type: String,
    pub authorized: bool,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamKeyView {
    pub arn: String,
    pub channel_arn: String,
    pub value: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingConfigurationView {
    pub arn: String,
    pub name: String,
    pub s3_bucket_name: Option<String>,
    pub state: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackKeyPairView {
    pub arn: String,
    pub name: String,
    pub fingerprint: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackRestrictionPolicyView {
    pub arn: String,
    pub name: String,
    pub allowed_countries: Vec<String>,
    pub allowed_origins: Vec<String>,
    pub enable_strict_origin_enforcement: bool,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

// --- From internal types to view types ---

impl From<&IvsState> for IvsStateView {
    fn from(state: &IvsState) -> Self {
        IvsStateView {
            channels: state
                .channels
                .iter()
                .map(|(k, v)| (k.clone(), ChannelView::from(v)))
                .collect(),
            stream_keys: state
                .stream_keys
                .iter()
                .map(|(k, v)| (k.clone(), StreamKeyView::from(v)))
                .collect(),
            recording_configurations: state
                .recording_configurations
                .iter()
                .map(|(k, v)| (k.clone(), RecordingConfigurationView::from(v)))
                .collect(),
            playback_key_pairs: state
                .playback_key_pairs
                .iter()
                .map(|(k, v)| (k.clone(), PlaybackKeyPairView::from(v)))
                .collect(),
            playback_restriction_policies: state
                .playback_restriction_policies
                .iter()
                .map(|(k, v)| (k.clone(), PlaybackRestrictionPolicyView::from(v)))
                .collect(),
        }
    }
}

impl From<&Channel> for ChannelView {
    fn from(c: &Channel) -> Self {
        ChannelView {
            arn: c.arn.clone(),
            name: c.name.clone(),
            latency_mode: c.latency_mode.clone(),
            channel_type: c.channel_type.clone(),
            authorized: c.authorized,
            tags: c.tags.clone(),
        }
    }
}

impl From<&StreamKey> for StreamKeyView {
    fn from(sk: &StreamKey) -> Self {
        StreamKeyView {
            arn: sk.arn.clone(),
            channel_arn: sk.channel_arn.clone(),
            value: sk.value.clone(),
            tags: sk.tags.clone(),
        }
    }
}

impl From<&RecordingConfiguration> for RecordingConfigurationView {
    fn from(rc: &RecordingConfiguration) -> Self {
        RecordingConfigurationView {
            arn: rc.arn.clone(),
            name: rc.name.clone(),
            s3_bucket_name: rc.destination_configuration.s3_bucket_name.clone(),
            state: rc.state.clone(),
            tags: rc.tags.clone(),
        }
    }
}

impl From<&PlaybackKeyPair> for PlaybackKeyPairView {
    fn from(kp: &PlaybackKeyPair) -> Self {
        PlaybackKeyPairView {
            arn: kp.arn.clone(),
            name: kp.name.clone(),
            fingerprint: kp.fingerprint.clone(),
            tags: kp.tags.clone(),
        }
    }
}

impl From<&PlaybackRestrictionPolicy> for PlaybackRestrictionPolicyView {
    fn from(p: &PlaybackRestrictionPolicy) -> Self {
        PlaybackRestrictionPolicyView {
            arn: p.arn.clone(),
            name: p.name.clone(),
            allowed_countries: p.allowed_countries.clone(),
            allowed_origins: p.allowed_origins.clone(),
            enable_strict_origin_enforcement: p.enable_strict_origin_enforcement,
            tags: p.tags.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<IvsStateView> for IvsState {
    fn from(view: IvsStateView) -> Self {
        IvsState {
            channels: view
                .channels
                .into_iter()
                .map(|(k, v)| (k, Channel::from(v)))
                .collect(),
            stream_keys: view
                .stream_keys
                .into_iter()
                .map(|(k, v)| (k, StreamKey::from(v)))
                .collect(),
            recording_configurations: view
                .recording_configurations
                .into_iter()
                .map(|(k, v)| (k, RecordingConfiguration::from(v)))
                .collect(),
            playback_key_pairs: view
                .playback_key_pairs
                .into_iter()
                .map(|(k, v)| (k, PlaybackKeyPair::from(v)))
                .collect(),
            playback_restriction_policies: view
                .playback_restriction_policies
                .into_iter()
                .map(|(k, v)| (k, PlaybackRestrictionPolicy::from(v)))
                .collect(),
            tags: HashMap::new(),
        }
    }
}

impl From<ChannelView> for Channel {
    fn from(v: ChannelView) -> Self {
        Channel {
            arn: v.arn,
            name: v.name,
            latency_mode: v.latency_mode,
            channel_type: v.channel_type,
            authorized: v.authorized,
            tags: v.tags,
        }
    }
}

impl From<StreamKeyView> for StreamKey {
    fn from(v: StreamKeyView) -> Self {
        StreamKey {
            arn: v.arn,
            channel_arn: v.channel_arn,
            value: v.value,
            tags: v.tags,
        }
    }
}

impl From<RecordingConfigurationView> for RecordingConfiguration {
    fn from(v: RecordingConfigurationView) -> Self {
        RecordingConfiguration {
            arn: v.arn,
            name: v.name,
            destination_configuration: RecordingDestination {
                s3_bucket_name: v.s3_bucket_name,
            },
            state: v.state,
            tags: v.tags,
        }
    }
}

impl From<PlaybackKeyPairView> for PlaybackKeyPair {
    fn from(v: PlaybackKeyPairView) -> Self {
        PlaybackKeyPair {
            arn: v.arn,
            name: v.name,
            fingerprint: v.fingerprint,
            tags: v.tags,
        }
    }
}

impl From<PlaybackRestrictionPolicyView> for PlaybackRestrictionPolicy {
    fn from(v: PlaybackRestrictionPolicyView) -> Self {
        PlaybackRestrictionPolicy {
            arn: v.arn,
            name: v.name,
            allowed_countries: v.allowed_countries,
            allowed_origins: v.allowed_origins,
            enable_strict_origin_enforcement: v.enable_strict_origin_enforcement,
            tags: v.tags,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for IvsService {
    type StateView = IvsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        IvsStateView::from(&*guard)
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
            *guard = IvsState::from(view);
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
            for (k, v) in view.channels {
                guard.channels.insert(k, Channel::from(v));
            }
            for (k, v) in view.stream_keys {
                guard.stream_keys.insert(k, StreamKey::from(v));
            }
            for (k, v) in view.recording_configurations {
                guard
                    .recording_configurations
                    .insert(k, RecordingConfiguration::from(v));
            }
            for (k, v) in view.playback_key_pairs {
                guard.playback_key_pairs.insert(k, PlaybackKeyPair::from(v));
            }
            for (k, v) in view.playback_restriction_policies {
                guard
                    .playback_restriction_policies
                    .insert(k, PlaybackRestrictionPolicy::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
