//! Serde-compatible view types for CloudTrail state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CloudTrailService;
use crate::state::CloudTrailState;

/// Serializable view of the entire CloudTrail state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudTrailStateView {
    /// Trails keyed by trail name.
    #[serde(default)]
    pub trails: HashMap<String, TrailView>,
    /// Event data stores keyed by ARN.
    #[serde(default)]
    pub event_data_stores: HashMap<String, EventDataStoreView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrailView {
    pub name: String,
    pub s3_bucket_name: String,
    pub s3_key_prefix: String,
    pub include_global_service_events: bool,
    pub is_multi_region_trail: bool,
    pub trail_arn: String,
    pub home_region: String,
    pub is_logging: bool,
    pub latest_delivery_time: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub event_selectors: Vec<EventSelectorView>,
    #[serde(default)]
    pub insight_selectors: Vec<InsightSelectorView>,
    #[serde(default)]
    pub advanced_event_selectors: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSelectorView {
    pub read_write_type: String,
    pub include_management_events: bool,
    #[serde(default)]
    pub data_resources: Vec<DataResourceView>,
    #[serde(default)]
    pub exclude_management_event_sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataResourceView {
    pub r#type: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsightSelectorView {
    pub insight_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventDataStoreView {
    pub event_data_store_arn: String,
    pub name: String,
    pub status: String,
    pub multi_region_enabled: bool,
    pub organization_enabled: bool,
    pub retention_period: i32,
    pub termination_protection_enabled: bool,
    /// RFC 3339 timestamp.
    pub created_timestamp: String,
    /// RFC 3339 timestamp.
    pub updated_timestamp: String,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
}

fn parse_datetime_opt(s: Option<&str>) -> Option<chrono::DateTime<chrono::Utc>> {
    s.and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc))
}

// --- From internal types to view types ---

impl From<&CloudTrailState> for CloudTrailStateView {
    fn from(state: &CloudTrailState) -> Self {
        CloudTrailStateView {
            trails: state
                .trails
                .iter()
                .map(|(k, t)| {
                    (
                        k.clone(),
                        TrailView {
                            name: t.name.clone(),
                            s3_bucket_name: t.s3_bucket_name.clone(),
                            s3_key_prefix: t.s3_key_prefix.clone(),
                            include_global_service_events: t.include_global_service_events,
                            is_multi_region_trail: t.is_multi_region_trail,
                            trail_arn: t.trail_arn.clone(),
                            home_region: t.home_region.clone(),
                            is_logging: t.is_logging,
                            latest_delivery_time: t.latest_delivery_time.map(|dt| dt.to_rfc3339()),
                            tags: t.tags.clone(),
                            event_selectors: t
                                .event_selectors
                                .iter()
                                .map(|es| EventSelectorView {
                                    read_write_type: es.read_write_type.clone(),
                                    include_management_events: es.include_management_events,
                                    data_resources: es
                                        .data_resources
                                        .iter()
                                        .map(|dr| DataResourceView {
                                            r#type: dr.r#type.clone(),
                                            values: dr.values.clone(),
                                        })
                                        .collect(),
                                    exclude_management_event_sources: es
                                        .exclude_management_event_sources
                                        .clone(),
                                })
                                .collect(),
                            insight_selectors: t
                                .insight_selectors
                                .iter()
                                .map(|is| InsightSelectorView {
                                    insight_type: is.insight_type.clone(),
                                })
                                .collect(),
                            advanced_event_selectors: None,
                        },
                    )
                })
                .collect(),
            event_data_stores: state
                .event_data_stores
                .iter()
                .map(|(k, eds)| {
                    (
                        k.clone(),
                        EventDataStoreView {
                            event_data_store_arn: eds.event_data_store_arn.clone(),
                            name: eds.name.clone(),
                            status: eds.status.clone(),
                            multi_region_enabled: eds.multi_region_enabled,
                            organization_enabled: eds.organization_enabled,
                            retention_period: eds.retention_period,
                            termination_protection_enabled: eds.termination_protection_enabled,
                            created_timestamp: eds.created_timestamp.to_rfc3339(),
                            updated_timestamp: eds.updated_timestamp.to_rfc3339(),
                            tags: eds.tags.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<CloudTrailStateView> for CloudTrailState {
    fn from(view: CloudTrailStateView) -> Self {
        CloudTrailState {
            trails: view
                .trails
                .into_iter()
                .map(|(k, t)| {
                    (
                        k,
                        crate::types::Trail {
                            name: t.name,
                            s3_bucket_name: t.s3_bucket_name,
                            s3_key_prefix: t.s3_key_prefix,
                            include_global_service_events: t.include_global_service_events,
                            is_multi_region_trail: t.is_multi_region_trail,
                            trail_arn: t.trail_arn,
                            home_region: t.home_region,
                            is_logging: t.is_logging,
                            latest_delivery_time: parse_datetime_opt(
                                t.latest_delivery_time.as_deref(),
                            ),
                            tags: t.tags,
                            event_selectors: t
                                .event_selectors
                                .into_iter()
                                .map(|es| crate::types::EventSelector {
                                    read_write_type: es.read_write_type,
                                    include_management_events: es.include_management_events,
                                    data_resources: es
                                        .data_resources
                                        .into_iter()
                                        .map(|dr| crate::types::DataResource {
                                            r#type: dr.r#type,
                                            values: dr.values,
                                        })
                                        .collect(),
                                    exclude_management_event_sources: es
                                        .exclude_management_event_sources,
                                })
                                .collect(),
                            insight_selectors: t
                                .insight_selectors
                                .into_iter()
                                .map(|is| crate::types::InsightSelector {
                                    insight_type: is.insight_type,
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
            event_data_stores: view
                .event_data_stores
                .into_iter()
                .map(|(k, eds)| {
                    (
                        k,
                        crate::types::EventDataStoreData {
                            event_data_store_arn: eds.event_data_store_arn,
                            name: eds.name,
                            status: eds.status,
                            multi_region_enabled: eds.multi_region_enabled,
                            organization_enabled: eds.organization_enabled,
                            retention_period: eds.retention_period,
                            termination_protection_enabled: eds.termination_protection_enabled,
                            created_timestamp: parse_datetime_opt(Some(&eds.created_timestamp))
                                .unwrap_or_else(chrono::Utc::now),
                            updated_timestamp: parse_datetime_opt(Some(&eds.updated_timestamp))
                                .unwrap_or_else(chrono::Utc::now),
                            tags: eds.tags,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for CloudTrailService {
    type StateView = CloudTrailStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        CloudTrailStateView::from(&*guard)
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
            *guard = CloudTrailState::from(view);
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
            let merged = CloudTrailState::from(view);
            for (k, v) in merged.trails {
                guard.trails.insert(k, v);
            }
            for (k, v) in merged.event_data_stores {
                guard.event_data_stores.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
