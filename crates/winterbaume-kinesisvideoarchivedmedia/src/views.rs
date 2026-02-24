//! Serde-compatible view types for KinesisVideoArchivedMedia state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::KinesisVideoArchivedMediaService;
use crate::state::KinesisVideoArchivedMediaState;
use crate::types::{Fragment, StreamData};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KinesisVideoArchivedMediaStateView {
    /// Stream data keyed by stream name.
    #[serde(default)]
    pub streams: HashMap<String, StreamDataView>,
    /// ARN to stream name mapping.
    #[serde(default)]
    pub arn_to_name: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamDataView {
    pub stream_name: String,
    pub stream_arn: String,
    #[serde(default)]
    pub fragments: Vec<FragmentView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FragmentView {
    pub fragment_number: String,
    pub fragment_size_in_bytes: i64,
    pub producer_timestamp: String,
    pub server_timestamp: String,
    pub fragment_length_in_milliseconds: i64,
}

fn parse_dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

// --- From internal types to view types ---

impl From<&KinesisVideoArchivedMediaState> for KinesisVideoArchivedMediaStateView {
    fn from(state: &KinesisVideoArchivedMediaState) -> Self {
        KinesisVideoArchivedMediaStateView {
            streams: state
                .streams
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        StreamDataView {
                            stream_name: v.stream_name.clone(),
                            stream_arn: v.stream_arn.clone(),
                            fragments: v
                                .fragments
                                .iter()
                                .map(|f| FragmentView {
                                    fragment_number: f.fragment_number.clone(),
                                    fragment_size_in_bytes: f.fragment_size_in_bytes,
                                    producer_timestamp: f.producer_timestamp.to_rfc3339(),
                                    server_timestamp: f.server_timestamp.to_rfc3339(),
                                    fragment_length_in_milliseconds: f
                                        .fragment_length_in_milliseconds,
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
            arn_to_name: state.arn_to_name.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<KinesisVideoArchivedMediaStateView> for KinesisVideoArchivedMediaState {
    fn from(view: KinesisVideoArchivedMediaStateView) -> Self {
        KinesisVideoArchivedMediaState {
            streams: view
                .streams
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        StreamData {
                            stream_name: v.stream_name,
                            stream_arn: v.stream_arn,
                            fragments: v
                                .fragments
                                .into_iter()
                                .map(|f| Fragment {
                                    fragment_number: f.fragment_number,
                                    fragment_size_in_bytes: f.fragment_size_in_bytes,
                                    producer_timestamp: parse_dt(&f.producer_timestamp),
                                    server_timestamp: parse_dt(&f.server_timestamp),
                                    fragment_length_in_milliseconds: f
                                        .fragment_length_in_milliseconds,
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
            arn_to_name: view.arn_to_name,
            sessions: HashMap::new(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for KinesisVideoArchivedMediaService {
    type StateView = KinesisVideoArchivedMediaStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        KinesisVideoArchivedMediaStateView::from(&*guard)
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
            *guard = KinesisVideoArchivedMediaState::from(view);
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
            let incoming = KinesisVideoArchivedMediaState::from(view);
            for (k, v) in incoming.streams {
                guard.streams.insert(k, v);
            }
            for (k, v) in incoming.arn_to_name {
                guard.arn_to_name.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
