//! Serde-compatible view types for SageMakerMetrics state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::SageMakerMetricsService;
use crate::state::SageMakerMetricsState;
use crate::types::StoredMetric;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SageMakerMetricsStateView {
    /// Metrics stored by trial component name.
    #[serde(default)]
    pub metrics: HashMap<String, Vec<StoredMetricView>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMetricView {
    pub trial_component_name: String,
    pub metric_name: String,
    pub timestamp: f64,
    pub step: Option<i64>,
    pub value: f64,
    pub ingested_at: DateTime<Utc>,
}

impl From<&SageMakerMetricsState> for SageMakerMetricsStateView {
    fn from(state: &SageMakerMetricsState) -> Self {
        SageMakerMetricsStateView {
            metrics: state
                .metrics
                .iter()
                .map(|(k, v)| (k.clone(), v.iter().map(StoredMetricView::from).collect()))
                .collect(),
        }
    }
}

impl From<&StoredMetric> for StoredMetricView {
    fn from(m: &StoredMetric) -> Self {
        StoredMetricView {
            trial_component_name: m.trial_component_name.clone(),
            metric_name: m.metric_name.clone(),
            timestamp: m.timestamp,
            step: m.step,
            value: m.value,
            ingested_at: m.ingested_at,
        }
    }
}

impl From<SageMakerMetricsStateView> for SageMakerMetricsState {
    fn from(view: SageMakerMetricsStateView) -> Self {
        SageMakerMetricsState {
            metrics: view
                .metrics
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(StoredMetric::from).collect()))
                .collect(),
        }
    }
}

impl From<StoredMetricView> for StoredMetric {
    fn from(v: StoredMetricView) -> Self {
        StoredMetric {
            trial_component_name: v.trial_component_name,
            metric_name: v.metric_name,
            timestamp: v.timestamp,
            step: v.step,
            value: v.value,
            ingested_at: v.ingested_at,
        }
    }
}

impl StatefulService for SageMakerMetricsService {
    type StateView = SageMakerMetricsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        SageMakerMetricsStateView::from(&*guard)
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
            *guard = SageMakerMetricsState::from(view);
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
            for (k, v) in view.metrics {
                let metrics = guard.metrics.entry(k).or_default();
                for m in v {
                    metrics.push(StoredMetric::from(m));
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
