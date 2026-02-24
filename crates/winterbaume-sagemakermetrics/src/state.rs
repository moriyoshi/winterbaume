use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct SageMakerMetricsState {
    /// Metrics stored by trial component name, then metric name.
    pub metrics: HashMap<String, Vec<StoredMetric>>,
}

#[derive(Debug, Error)]
pub enum SageMakerMetricsError {
    #[error("TrialComponentName must not be empty.")]
    EmptyTrialComponentName,
}

impl SageMakerMetricsState {
    /// Store a batch of metric data points for a trial component.
    /// Returns a list of indices that failed (empty on full success).
    pub fn batch_put_metrics(
        &mut self,
        trial_component_name: &str,
        metric_data: &[RawMetricData],
    ) -> Result<Vec<usize>, SageMakerMetricsError> {
        if trial_component_name.is_empty() {
            return Err(SageMakerMetricsError::EmptyTrialComponentName);
        }

        let now = Utc::now();
        for raw in metric_data.iter() {
            let stored = StoredMetric {
                trial_component_name: trial_component_name.to_string(),
                metric_name: raw.metric_name.clone(),
                timestamp: raw.timestamp,
                step: raw.step,
                value: raw.value,
                ingested_at: now,
            };
            self.metrics
                .entry(trial_component_name.to_string())
                .or_default()
                .push(stored);
        }

        // In the mock, all metrics succeed.
        Ok(vec![])
    }

    /// Query metrics for BatchGetMetrics. Returns results for each query.
    pub fn batch_get_metrics(&self, queries: &[MetricQuery]) -> Vec<MetricQueryResult> {
        queries
            .iter()
            .map(|query| {
                // Extract trial component name from ResourceArn.
                // ARN format: arn:aws:sagemaker:<region>:<account>:experiment-trial-component/<name>
                let trial_component_name = query.resource_arn.rsplit('/').next().unwrap_or("");

                let x_axis_type = query.x_axis_type.as_deref().unwrap_or("Timestamp");

                if let Some(stored_metrics) = self.metrics.get(trial_component_name) {
                    let matching: Vec<&StoredMetric> = stored_metrics
                        .iter()
                        .filter(|m| m.metric_name == query.metric_name)
                        .filter(|m| {
                            if let Some(start) = query.start {
                                m.timestamp >= start as f64
                            } else {
                                true
                            }
                        })
                        .filter(|m| {
                            if let Some(end) = query.end {
                                m.timestamp <= end as f64
                            } else {
                                true
                            }
                        })
                        .collect();

                    if matching.is_empty() {
                        MetricQueryResult {
                            status: "Complete".to_string(),
                            x_axis_values: vec![],
                            metric_values: vec![],
                            message: None,
                        }
                    } else {
                        let x_axis_values: Vec<f64> = matching
                            .iter()
                            .map(|m| {
                                if x_axis_type == "IterationNumber" || x_axis_type == "Step" {
                                    m.step.unwrap_or(0) as f64
                                } else {
                                    m.timestamp
                                }
                            })
                            .collect();
                        let metric_values: Vec<f64> = matching.iter().map(|m| m.value).collect();

                        // For aggregation (Avg, Min, Max, Count, etc.),
                        // the mock just returns raw values. A real implementation
                        // would aggregate based on metric_stat and period.
                        MetricQueryResult {
                            status: "Complete".to_string(),
                            x_axis_values,
                            metric_values,
                            message: None,
                        }
                    }
                } else {
                    MetricQueryResult {
                        status: "Complete".to_string(),
                        x_axis_values: vec![],
                        metric_values: vec![],
                        message: None,
                    }
                }
            })
            .collect()
    }
}
