//! Serde-compatible view types for X-Ray state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::XRayService;
use crate::state::XRayState;
use crate::types::{
    EncryptionConfig, Group, IndexingRuleData, InsightsConfiguration, ResourcePolicy,
    SamplingRuleData, TraceSegmentDestination,
};

/// Serializable view of the entire X-Ray state for one account/region.
///
/// Transient runtime state intentionally omitted from snapshots: `trace_segments`,
/// `telemetry_records`, `trace_retrieval_jobs`, `sampling_statistic_summaries`. These are
/// per-request observation buffers that have no durable meaning across a server restart
/// and are repopulated by client traffic.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[allow(clippy::upper_case_acronyms)]
pub struct XRayStateView {
    #[serde(default)]
    pub groups: HashMap<String, GroupView>,
    /// Resource-based policies keyed by policy_name.
    #[serde(default)]
    pub resource_policies: HashMap<String, ResourcePolicyView>,
    /// Sampling rules keyed by rule_name.
    #[serde(default)]
    pub sampling_rules: HashMap<String, SamplingRuleView>,
    /// Encryption configuration for traces.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfigView>,
    /// Per-resource tags keyed by resource ARN.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub resource_tags: HashMap<String, HashMap<String, String>>,
    /// Trace segment destination configuration (durable account-level setting).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_segment_destination: Option<TraceSegmentDestinationView>,
    /// Indexing rules keyed by rule name.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub indexing_rules: HashMap<String, IndexingRuleView>,
    // trace_segments, telemetry_records, trace_retrieval_jobs, and
    // sampling_statistic_summaries are transient and excluded from snapshots.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePolicyView {
    pub policy_name: String,
    pub policy_document: String,
    pub policy_revision_id: String,
    pub last_updated_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplingRuleView {
    pub rule_name: String,
    pub rule_arn: String,
    pub resource_arn: String,
    pub priority: i32,
    pub fixed_rate: f64,
    pub reservoir_size: i32,
    pub service_name: String,
    pub service_type: String,
    pub host: String,
    pub http_method: String,
    pub url_path: String,
    pub version: i32,
    pub created_at: f64,
    pub modified_at: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupView {
    pub group_name: String,
    pub group_arn: String,
    pub filter_expression: String,
    pub insights_enabled: Option<bool>,
    pub notifications_enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfigView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    pub status: String,
    pub encryption_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceSegmentDestinationView {
    pub destination: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingRuleView {
    pub name: String,
    pub modified_at: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desired_sampling_percentage: Option<f64>,
}

// --- From internal types to view types ---

impl From<&ResourcePolicy> for ResourcePolicyView {
    fn from(p: &ResourcePolicy) -> Self {
        ResourcePolicyView {
            policy_name: p.policy_name.clone(),
            policy_document: p.policy_document.clone(),
            policy_revision_id: p.policy_revision_id.clone(),
            last_updated_time: p.last_updated_time,
        }
    }
}

impl From<&EncryptionConfig> for EncryptionConfigView {
    fn from(c: &EncryptionConfig) -> Self {
        EncryptionConfigView {
            key_id: c.key_id.clone(),
            status: c.status.clone(),
            encryption_type: c.encryption_type.clone(),
        }
    }
}

impl From<EncryptionConfigView> for EncryptionConfig {
    fn from(v: EncryptionConfigView) -> Self {
        EncryptionConfig {
            key_id: v.key_id,
            status: v.status,
            encryption_type: v.encryption_type,
        }
    }
}

impl From<&TraceSegmentDestination> for TraceSegmentDestinationView {
    fn from(d: &TraceSegmentDestination) -> Self {
        TraceSegmentDestinationView {
            destination: d.destination.clone(),
        }
    }
}

impl From<TraceSegmentDestinationView> for TraceSegmentDestination {
    fn from(v: TraceSegmentDestinationView) -> Self {
        TraceSegmentDestination {
            destination: v.destination,
        }
    }
}

impl From<&IndexingRuleData> for IndexingRuleView {
    fn from(r: &IndexingRuleData) -> Self {
        IndexingRuleView {
            name: r.name.clone(),
            modified_at: r.modified_at,
            desired_sampling_percentage: r.desired_sampling_percentage,
        }
    }
}

impl From<IndexingRuleView> for IndexingRuleData {
    fn from(v: IndexingRuleView) -> Self {
        IndexingRuleData {
            name: v.name,
            modified_at: v.modified_at,
            desired_sampling_percentage: v.desired_sampling_percentage,
        }
    }
}

impl From<&SamplingRuleData> for SamplingRuleView {
    fn from(r: &SamplingRuleData) -> Self {
        SamplingRuleView {
            rule_name: r.rule_name.clone(),
            rule_arn: r.rule_arn.clone(),
            resource_arn: r.resource_arn.clone(),
            priority: r.priority,
            fixed_rate: r.fixed_rate,
            reservoir_size: r.reservoir_size,
            service_name: r.service_name.clone(),
            service_type: r.service_type.clone(),
            host: r.host.clone(),
            http_method: r.http_method.clone(),
            url_path: r.url_path.clone(),
            version: r.version,
            created_at: r.created_at,
            modified_at: r.modified_at,
        }
    }
}

impl From<&XRayState> for XRayStateView {
    fn from(state: &XRayState) -> Self {
        XRayStateView {
            groups: state
                .groups
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        GroupView {
                            group_name: v.group_name.clone(),
                            group_arn: v.group_arn.clone(),
                            filter_expression: v.filter_expression.clone(),
                            insights_enabled: v
                                .insights_configuration
                                .as_ref()
                                .map(|ic| ic.insights_enabled),
                            notifications_enabled: v
                                .insights_configuration
                                .as_ref()
                                .map(|ic| ic.notifications_enabled),
                        },
                    )
                })
                .collect(),
            resource_policies: state
                .resource_policies
                .iter()
                .map(|(k, v)| (k.clone(), ResourcePolicyView::from(v)))
                .collect(),
            sampling_rules: state
                .sampling_rules
                .iter()
                .map(|(k, v)| (k.clone(), SamplingRuleView::from(v)))
                .collect(),
            encryption_config: Some(EncryptionConfigView::from(&state.encryption_config)),
            resource_tags: state.resource_tags.clone(),
            trace_segment_destination: Some(TraceSegmentDestinationView::from(
                &state.trace_segment_destination,
            )),
            indexing_rules: state
                .indexing_rules
                .iter()
                .map(|(k, v)| (k.clone(), IndexingRuleView::from(v)))
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<XRayStateView> for XRayState {
    fn from(view: XRayStateView) -> Self {
        XRayState {
            groups: view
                .groups
                .into_iter()
                .map(|(k, v)| {
                    let insights_configuration = match (v.insights_enabled, v.notifications_enabled)
                    {
                        (Some(ie), Some(ne)) => Some(InsightsConfiguration {
                            insights_enabled: ie,
                            notifications_enabled: ne,
                        }),
                        (Some(ie), None) => Some(InsightsConfiguration {
                            insights_enabled: ie,
                            notifications_enabled: false,
                        }),
                        (None, Some(ne)) => Some(InsightsConfiguration {
                            insights_enabled: false,
                            notifications_enabled: ne,
                        }),
                        (None, None) => None,
                    };
                    (
                        k,
                        Group {
                            group_name: v.group_name,
                            group_arn: v.group_arn,
                            filter_expression: v.filter_expression,
                            insights_configuration,
                        },
                    )
                })
                .collect(),
            resource_policies: view
                .resource_policies
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        ResourcePolicy {
                            policy_name: v.policy_name,
                            policy_document: v.policy_document,
                            policy_revision_id: v.policy_revision_id,
                            last_updated_time: v.last_updated_time,
                        },
                    )
                })
                .collect(),
            sampling_rules: view
                .sampling_rules
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        SamplingRuleData {
                            rule_name: v.rule_name,
                            rule_arn: v.rule_arn,
                            resource_arn: v.resource_arn,
                            priority: v.priority,
                            fixed_rate: v.fixed_rate,
                            reservoir_size: v.reservoir_size,
                            service_name: v.service_name,
                            service_type: v.service_type,
                            host: v.host,
                            http_method: v.http_method,
                            url_path: v.url_path,
                            version: v.version,
                            created_at: v.created_at,
                            modified_at: v.modified_at,
                        },
                    )
                })
                .collect(),
            encryption_config: view
                .encryption_config
                .map(EncryptionConfig::from)
                .unwrap_or_default(),
            resource_tags: view.resource_tags,
            trace_segment_destination: view
                .trace_segment_destination
                .map(TraceSegmentDestination::from)
                .unwrap_or_default(),
            indexing_rules: view
                .indexing_rules
                .into_iter()
                .map(|(k, v)| (k, IndexingRuleData::from(v)))
                .collect(),
            trace_segments: Default::default(), // transient; not persisted
            telemetry_records: Default::default(), // transient
            trace_retrieval_jobs: Default::default(), // transient
            sampling_statistic_summaries: Default::default(), // transient
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for XRayService {
    type StateView = XRayStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        XRayStateView::from(&*guard)
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
            *guard = XRayState::from(view);
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
            // Account-level singletons are only overwritten when the supplied view
            // explicitly carries a value, preserving the existing setting otherwise.
            // Map-shaped fields merge per-key.
            let supplied_encryption = view.encryption_config.clone();
            let supplied_destination = view.trace_segment_destination.clone();
            let new_state = XRayState::from(view);
            guard.groups.extend(new_state.groups);
            guard.resource_policies.extend(new_state.resource_policies);
            guard.sampling_rules.extend(new_state.sampling_rules);
            if let Some(ec) = supplied_encryption {
                guard.encryption_config = EncryptionConfig::from(ec);
            }
            if let Some(d) = supplied_destination {
                guard.trace_segment_destination = TraceSegmentDestination::from(d);
            }
            for (arn, tags) in new_state.resource_tags {
                guard.resource_tags.entry(arn).or_default().extend(tags);
            }
            guard.indexing_rules.extend(new_state.indexing_rules);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
