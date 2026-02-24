//! Serde-compatible view types for Firehose state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::FirehoseService;
use crate::state::FirehoseState;
use crate::types::{DeliveryStream, FirehoseRecord};

/// Serializable view of the entire Firehose state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirehoseStateView {
    #[serde(default)]
    pub streams: HashMap<String, DeliveryStreamView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryStreamView {
    pub name: String,
    pub arn: String,
    pub status: String,
    pub destination_type: String,
    pub destination_id: String,
    pub created_timestamp: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub encryption_status: String,
    pub version_id: String,
}

// ---------------------------------------------------------------------------
// From conversions
// ---------------------------------------------------------------------------

impl From<&DeliveryStream> for DeliveryStreamView {
    fn from(ds: &DeliveryStream) -> Self {
        DeliveryStreamView {
            name: ds.name.clone(),
            arn: ds.arn.clone(),
            status: ds.status.clone(),
            destination_type: ds.destination_type.clone(),
            destination_id: ds.destination_id.clone(),
            created_timestamp: Some(ds.created_timestamp.to_rfc3339()),
            tags: ds.tags.clone(),
            encryption_status: ds.encryption_status.clone(),
            version_id: ds.version_id.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for FirehoseService {
    type StateView = FirehoseStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;

        let streams = guard
            .streams
            .values()
            .map(|ds| (ds.name.clone(), DeliveryStreamView::from(ds)))
            .collect();

        FirehoseStateView { streams }
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let mut new_state = FirehoseState::default();

        for (name, dsv) in view.streams {
            let created_timestamp = dsv
                .created_timestamp
                .as_deref()
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(Utc::now);
            new_state.streams.insert(
                name,
                DeliveryStream {
                    name: dsv.name,
                    arn: dsv.arn,
                    status: dsv.status,
                    destination_type: dsv.destination_type,
                    destination_id: dsv.destination_id,
                    created_timestamp,
                    records: Vec::<FirehoseRecord>::new(),
                    tags: dsv.tags,
                    encryption_status: dsv.encryption_status,
                    version_id: dsv.version_id,
                },
            );
        }

        {
            let state = self.state.get(account_id, region);
            *state.write().await = new_state;
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

            for (name, dsv) in view.streams {
                let created_timestamp = dsv
                    .created_timestamp
                    .as_deref()
                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(Utc::now);
                guard.streams.insert(
                    name,
                    DeliveryStream {
                        name: dsv.name,
                        arn: dsv.arn,
                        status: dsv.status,
                        destination_type: dsv.destination_type,
                        destination_id: dsv.destination_id,
                        created_timestamp,
                        records: Vec::<FirehoseRecord>::new(),
                        tags: dsv.tags,
                        encryption_status: dsv.encryption_status,
                        version_id: dsv.version_id,
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
