//! Serde-compatible view types for MediaPackage state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::MediaPackageService;
use crate::state::MediaPackageState;
use crate::types::{Channel, OriginEndpoint};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MediaPackageStateView {
    #[serde(default)]
    pub channels: HashMap<String, ChannelView>,
    #[serde(default)]
    pub origin_endpoints: HashMap<String, OriginEndpointView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelView {
    pub arn: String,
    pub id: String,
    pub description: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginEndpointView {
    pub arn: String,
    pub id: String,
    pub channel_id: String,
    pub description: String,
    pub manifest_name: String,
    pub startover_window_seconds: i32,
    pub time_delay_seconds: i32,
    pub url: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

fn parse_dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

// --- From internal types to view types ---

impl From<&MediaPackageState> for MediaPackageStateView {
    fn from(state: &MediaPackageState) -> Self {
        MediaPackageStateView {
            channels: state
                .channels
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ChannelView {
                            arn: v.arn.clone(),
                            id: v.id.clone(),
                            description: v.description.clone(),
                            tags: v.tags.clone(),
                            created_at: v.created_at.to_rfc3339(),
                        },
                    )
                })
                .collect(),
            origin_endpoints: state
                .origin_endpoints
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        OriginEndpointView {
                            arn: v.arn.clone(),
                            id: v.id.clone(),
                            channel_id: v.channel_id.clone(),
                            description: v.description.clone(),
                            manifest_name: v.manifest_name.clone(),
                            startover_window_seconds: v.startover_window_seconds,
                            time_delay_seconds: v.time_delay_seconds,
                            url: v.url.clone(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<MediaPackageStateView> for MediaPackageState {
    fn from(view: MediaPackageStateView) -> Self {
        MediaPackageState {
            channels: view
                .channels
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Channel {
                            arn: v.arn,
                            id: v.id,
                            description: v.description,
                            tags: v.tags,
                            created_at: parse_dt(&v.created_at),
                        },
                    )
                })
                .collect(),
            origin_endpoints: view
                .origin_endpoints
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        OriginEndpoint {
                            arn: v.arn,
                            id: v.id,
                            channel_id: v.channel_id,
                            description: v.description,
                            manifest_name: v.manifest_name,
                            startover_window_seconds: v.startover_window_seconds,
                            time_delay_seconds: v.time_delay_seconds,
                            url: v.url,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for MediaPackageService {
    type StateView = MediaPackageStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        MediaPackageStateView::from(&*guard)
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
            *guard = MediaPackageState::from(view);
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
                guard.channels.insert(
                    k,
                    Channel {
                        arn: v.arn,
                        id: v.id,
                        description: v.description,
                        tags: v.tags,
                        created_at: parse_dt(&v.created_at),
                    },
                );
            }
            for (k, v) in view.origin_endpoints {
                guard.origin_endpoints.insert(
                    k,
                    OriginEndpoint {
                        arn: v.arn,
                        id: v.id,
                        channel_id: v.channel_id,
                        description: v.description,
                        manifest_name: v.manifest_name,
                        startover_window_seconds: v.startover_window_seconds,
                        time_delay_seconds: v.time_delay_seconds,
                        url: v.url,
                        tags: v.tags,
                    },
                );
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
