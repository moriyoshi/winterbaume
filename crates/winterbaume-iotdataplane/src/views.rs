//! Serde-compatible view types for IoT Data Plane state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::IotDataPlaneService;
use crate::state::IotDataPlaneState;
use crate::types::{PublishedMessage, RetainedMessage, ShadowKey, ThingShadow};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IotDataPlaneStateView {
    /// Shadows keyed by "{thing_name}:{shadow_name}" or "{thing_name}:" for classic.
    #[serde(default)]
    pub shadows: HashMap<String, ThingShadowView>,
    /// Retained MQTT messages keyed by topic.
    #[serde(default)]
    pub retained_messages: HashMap<String, RetainedMessageView>,
    /// MQTT publish history (newest at the end).
    #[serde(default)]
    pub published_messages: Vec<PublishedMessageView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThingShadowView {
    pub thing_name: String,
    pub shadow_name: Option<String>,
    /// Payload stored as hex-encoded bytes.
    pub payload_hex: String,
    pub version: i64,
    pub last_modified: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetainedMessageView {
    pub topic: String,
    /// Payload stored as hex-encoded bytes.
    pub payload_hex: String,
    pub qos: i32,
    pub last_modified: String,
}

/// Serializable view of a single MQTT publish event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishedMessageView {
    pub topic: String,
    /// Payload stored as hex-encoded bytes.
    pub payload_hex: String,
    pub qos: i32,
    pub retain: bool,
    pub published_at: String,
}

fn shadow_key_str(key: &ShadowKey) -> String {
    format!(
        "{}:{}",
        key.thing_name,
        key.shadow_name.as_deref().unwrap_or("")
    )
}

fn parse_dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .filter_map(|i| u8::from_str_radix(&hex[i..i + 2], 16).ok())
        .collect()
}

// --- From internal types to view types ---

impl From<&IotDataPlaneState> for IotDataPlaneStateView {
    fn from(state: &IotDataPlaneState) -> Self {
        IotDataPlaneStateView {
            shadows: state
                .shadows
                .iter()
                .map(|(key, shadow)| {
                    (
                        shadow_key_str(key),
                        ThingShadowView {
                            thing_name: shadow.thing_name.clone(),
                            shadow_name: shadow.shadow_name.clone(),
                            payload_hex: bytes_to_hex(&shadow.payload),
                            version: shadow.version,
                            last_modified: shadow.last_modified.to_rfc3339(),
                        },
                    )
                })
                .collect(),
            retained_messages: state
                .retained_messages
                .iter()
                .map(|(topic, msg)| {
                    (
                        topic.clone(),
                        RetainedMessageView {
                            topic: msg.topic.clone(),
                            payload_hex: bytes_to_hex(&msg.payload),
                            qos: msg.qos,
                            last_modified: msg.last_modified.to_rfc3339(),
                        },
                    )
                })
                .collect(),
            published_messages: state
                .published_messages
                .iter()
                .map(|msg| PublishedMessageView {
                    topic: msg.topic.clone(),
                    payload_hex: bytes_to_hex(&msg.payload),
                    qos: msg.qos,
                    retain: msg.retain,
                    published_at: msg.published_at.to_rfc3339(),
                })
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<IotDataPlaneStateView> for IotDataPlaneState {
    fn from(view: IotDataPlaneStateView) -> Self {
        let shadows = view
            .shadows
            .into_values()
            .map(|sv| {
                let key = match sv.shadow_name.as_deref() {
                    Some(name) if !name.is_empty() => ShadowKey::named(&sv.thing_name, name),
                    _ => ShadowKey::classic(&sv.thing_name),
                };
                let payload = hex_to_bytes(&sv.payload_hex);
                let shadow = ThingShadow {
                    thing_name: sv.thing_name,
                    shadow_name: sv.shadow_name,
                    payload,
                    version: sv.version,
                    last_modified: parse_dt(&sv.last_modified),
                };
                (key, shadow)
            })
            .collect();

        let retained_messages = view
            .retained_messages
            .into_values()
            .map(|rv| {
                let msg = RetainedMessage {
                    topic: rv.topic.clone(),
                    payload: hex_to_bytes(&rv.payload_hex),
                    qos: rv.qos,
                    last_modified: parse_dt(&rv.last_modified),
                };
                (rv.topic, msg)
            })
            .collect();

        let published_messages = view
            .published_messages
            .into_iter()
            .map(|pv| PublishedMessage {
                topic: pv.topic,
                payload: hex_to_bytes(&pv.payload_hex),
                qos: pv.qos,
                retain: pv.retain,
                published_at: parse_dt(&pv.published_at),
            })
            .collect();

        IotDataPlaneState {
            shadows,
            published_messages,
            retained_messages,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for IotDataPlaneService {
    type StateView = IotDataPlaneStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        IotDataPlaneStateView::from(&*guard)
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
            *guard = IotDataPlaneState::from(view);
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
            let incoming = IotDataPlaneState::from(view);
            for (k, v) in incoming.shadows {
                guard.shadows.insert(k, v);
            }
            for (k, v) in incoming.retained_messages {
                guard.retained_messages.insert(k, v);
            }
            guard.published_messages.extend(incoming.published_messages);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
