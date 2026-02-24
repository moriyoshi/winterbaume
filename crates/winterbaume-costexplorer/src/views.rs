//! Serde-compatible view types for Cost Explorer state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CostExplorerService;
use crate::state::CostExplorerState;
use crate::types::{
    AnomalyMonitorRecord, AnomalySubscriptionRecord, CostAllocationTagBackfillRecord,
    CostAllocationTagRecord, CostCategoryDefinition, CostCategoryRule, SubscriberRecord,
};

/// Serializable view of the entire Cost Explorer state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CostExplorerStateView {
    /// Cost category definitions keyed by ARN.
    #[serde(default)]
    pub cost_categories: HashMap<String, CostCategoryDefinitionView>,
    /// Anomaly monitors keyed by ARN.
    #[serde(default)]
    pub anomaly_monitors: HashMap<String, AnomalyMonitorView>,
    /// Anomaly subscriptions keyed by ARN.
    #[serde(default)]
    pub anomaly_subscriptions: HashMap<String, AnomalySubscriptionView>,
    /// Resource tags keyed by ARN.
    #[serde(default)]
    pub resource_tags: HashMap<String, Vec<(String, String)>>,
    /// Cost allocation tags keyed by tag key.
    #[serde(default)]
    pub cost_allocation_tags: HashMap<String, CostAllocationTagRecord>,
    /// Cost allocation tag backfill jobs.
    #[serde(default)]
    pub cost_allocation_tag_backfills: Vec<CostAllocationTagBackfillRecord>,
    /// Anomaly feedback records keyed by anomaly ID.
    #[serde(default)]
    pub anomaly_feedback: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostCategoryDefinitionView {
    pub name: String,
    pub cost_category_arn: String,
    pub effective_start: String,
    pub rule_version: String,
    #[serde(default)]
    pub rules: Vec<CostCategoryRuleView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostCategoryRuleView {
    pub value: Option<String>,
    pub rule: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyMonitorView {
    pub monitor_arn: String,
    pub monitor_name: String,
    pub monitor_type: String,
    pub monitor_dimension: Option<String>,
    pub creation_date: String,
    pub last_updated_date: String,
    pub last_evaluated_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalySubscriptionView {
    pub subscription_arn: String,
    pub subscription_name: String,
    pub account_id: String,
    pub monitor_arn_list: Vec<String>,
    pub subscribers: Vec<SubscriberView>,
    pub frequency: String,
    pub threshold: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriberView {
    pub address: Option<String>,
    pub status: Option<String>,
    pub subscriber_type: Option<String>,
}

// --- From internal types to view types ---

impl From<&CostExplorerState> for CostExplorerStateView {
    fn from(state: &CostExplorerState) -> Self {
        CostExplorerStateView {
            cost_categories: state
                .cost_categories
                .iter()
                .map(|(k, v)| (k.clone(), CostCategoryDefinitionView::from(v)))
                .collect(),
            anomaly_monitors: state
                .anomaly_monitors
                .iter()
                .map(|(k, v)| (k.clone(), AnomalyMonitorView::from(v)))
                .collect(),
            anomaly_subscriptions: state
                .anomaly_subscriptions
                .iter()
                .map(|(k, v)| (k.clone(), AnomalySubscriptionView::from(v)))
                .collect(),
            resource_tags: state.resource_tags.clone(),
            cost_allocation_tags: state.cost_allocation_tags.clone(),
            cost_allocation_tag_backfills: state.cost_allocation_tag_backfills.clone(),
            anomaly_feedback: state.anomaly_feedback.clone(),
        }
    }
}

impl From<&CostCategoryDefinition> for CostCategoryDefinitionView {
    fn from(d: &CostCategoryDefinition) -> Self {
        CostCategoryDefinitionView {
            name: d.name.clone(),
            cost_category_arn: d.cost_category_arn.clone(),
            effective_start: d.effective_start.clone(),
            rule_version: d.rule_version.clone(),
            rules: d
                .rules
                .iter()
                .map(|r| CostCategoryRuleView {
                    value: r.value.clone(),
                    rule: r.rule.clone(),
                })
                .collect(),
        }
    }
}

impl From<&AnomalyMonitorRecord> for AnomalyMonitorView {
    fn from(m: &AnomalyMonitorRecord) -> Self {
        AnomalyMonitorView {
            monitor_arn: m.monitor_arn.clone(),
            monitor_name: m.monitor_name.clone(),
            monitor_type: m.monitor_type.clone(),
            monitor_dimension: m.monitor_dimension.clone(),
            creation_date: m.creation_date.clone(),
            last_updated_date: m.last_updated_date.clone(),
            last_evaluated_date: m.last_evaluated_date.clone(),
        }
    }
}

impl From<&AnomalySubscriptionRecord> for AnomalySubscriptionView {
    fn from(s: &AnomalySubscriptionRecord) -> Self {
        AnomalySubscriptionView {
            subscription_arn: s.subscription_arn.clone(),
            subscription_name: s.subscription_name.clone(),
            account_id: s.account_id.clone(),
            monitor_arn_list: s.monitor_arn_list.clone(),
            subscribers: s
                .subscribers
                .iter()
                .map(|sub| SubscriberView {
                    address: sub.address.clone(),
                    status: sub.status.clone(),
                    subscriber_type: sub.subscriber_type.clone(),
                })
                .collect(),
            frequency: s.frequency.clone(),
            threshold: s.threshold,
        }
    }
}

// --- From view types to internal types ---

impl From<CostExplorerStateView> for CostExplorerState {
    fn from(view: CostExplorerStateView) -> Self {
        let cost_categories = view
            .cost_categories
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    CostCategoryDefinition {
                        name: v.name,
                        cost_category_arn: v.cost_category_arn,
                        effective_start: v.effective_start,
                        rule_version: v.rule_version,
                        rules: v
                            .rules
                            .into_iter()
                            .map(|r| CostCategoryRule {
                                value: r.value,
                                rule: r.rule,
                            })
                            .collect(),
                    },
                )
            })
            .collect();

        let anomaly_monitors = view
            .anomaly_monitors
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    AnomalyMonitorRecord {
                        monitor_arn: v.monitor_arn,
                        monitor_name: v.monitor_name,
                        monitor_type: v.monitor_type,
                        monitor_dimension: v.monitor_dimension,
                        creation_date: v.creation_date,
                        last_updated_date: v.last_updated_date,
                        last_evaluated_date: v.last_evaluated_date,
                    },
                )
            })
            .collect();

        let anomaly_subscriptions = view
            .anomaly_subscriptions
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    AnomalySubscriptionRecord {
                        subscription_arn: v.subscription_arn,
                        subscription_name: v.subscription_name,
                        account_id: v.account_id,
                        monitor_arn_list: v.monitor_arn_list,
                        subscribers: v
                            .subscribers
                            .into_iter()
                            .map(|s| SubscriberRecord {
                                address: s.address,
                                status: s.status,
                                subscriber_type: s.subscriber_type,
                            })
                            .collect(),
                        frequency: v.frequency,
                        threshold: v.threshold,
                    },
                )
            })
            .collect();

        let mut state = CostExplorerState::default();
        state.cost_categories = cost_categories;
        state.anomaly_monitors = anomaly_monitors;
        state.anomaly_subscriptions = anomaly_subscriptions;
        state.resource_tags = view.resource_tags;
        state.cost_allocation_tags = view.cost_allocation_tags;
        state.cost_allocation_tag_backfills = view.cost_allocation_tag_backfills;
        state.anomaly_feedback = view.anomaly_feedback;
        state
    }
}

// --- StatefulService implementation ---

impl StatefulService for CostExplorerService {
    type StateView = CostExplorerStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        CostExplorerStateView::from(&*guard)
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
            *guard = CostExplorerState::from(view);
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
            let incoming = CostExplorerState::from(view);
            for (k, v) in incoming.cost_categories {
                guard.cost_categories.insert(k, v);
            }
            for (k, v) in incoming.anomaly_monitors {
                guard.anomaly_monitors.insert(k, v);
            }
            for (k, v) in incoming.anomaly_subscriptions {
                guard.anomaly_subscriptions.insert(k, v);
            }
            for (k, v) in incoming.resource_tags {
                guard.resource_tags.insert(k, v);
            }
            for (k, v) in incoming.cost_allocation_tags {
                guard.cost_allocation_tags.insert(k, v);
            }
            guard
                .cost_allocation_tag_backfills
                .extend(incoming.cost_allocation_tag_backfills);
            for (k, v) in incoming.anomaly_feedback {
                guard.anomaly_feedback.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
