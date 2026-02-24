//! Serde-compatible view types for KinesisAnalyticsV2 state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::KinesisAnalyticsV2Service;
use crate::state::KinesisAnalyticsV2State;
use crate::types::{Application, Snapshot};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KinesisAnalyticsV2StateView {
    #[serde(default)]
    pub applications: HashMap<String, ApplicationView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationView {
    pub application_name: String,
    pub application_arn: String,
    pub application_status: String,
    pub application_version_id: i64,
    pub runtime_environment: String,
    pub service_execution_role: Option<String>,
    pub create_timestamp: String,
    pub last_update_timestamp: String,
    pub application_description: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub snapshots: Vec<SnapshotView>,
    #[serde(default)]
    pub application_configuration: Option<serde_json::Value>,
    #[serde(default)]
    pub cloudwatch_logging_options: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotView {
    pub snapshot_name: String,
    pub application_version_id: i64,
    pub runtime_environment: String,
    pub snapshot_creation_timestamp: String,
}

fn parse_dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

// --- From internal types to view types ---

impl From<&KinesisAnalyticsV2State> for KinesisAnalyticsV2StateView {
    fn from(state: &KinesisAnalyticsV2State) -> Self {
        KinesisAnalyticsV2StateView {
            applications: state
                .applications
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ApplicationView {
                            application_name: v.application_name.clone(),
                            application_arn: v.application_arn.clone(),
                            application_status: v.application_status.clone(),
                            application_version_id: v.application_version_id,
                            runtime_environment: v.runtime_environment.clone(),
                            service_execution_role: v.service_execution_role.clone(),
                            create_timestamp: v.create_timestamp.to_rfc3339(),
                            last_update_timestamp: v.last_update_timestamp.to_rfc3339(),
                            application_description: v.application_description.clone(),
                            tags: v.tags.clone(),
                            snapshots: v
                                .snapshots
                                .iter()
                                .map(|s| SnapshotView {
                                    snapshot_name: s.snapshot_name.clone(),
                                    application_version_id: s.application_version_id,
                                    runtime_environment: s.runtime_environment.clone(),
                                    snapshot_creation_timestamp: s
                                        .snapshot_creation_timestamp
                                        .to_rfc3339(),
                                })
                                .collect(),
                            application_configuration: v.application_configuration.clone(),
                            cloudwatch_logging_options: v.cloudwatch_logging_options.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<KinesisAnalyticsV2StateView> for KinesisAnalyticsV2State {
    fn from(view: KinesisAnalyticsV2StateView) -> Self {
        KinesisAnalyticsV2State {
            applications: view
                .applications
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Application {
                            application_name: v.application_name,
                            application_arn: v.application_arn,
                            application_status: v.application_status,
                            application_version_id: v.application_version_id,
                            runtime_environment: v.runtime_environment,
                            service_execution_role: v.service_execution_role,
                            create_timestamp: parse_dt(&v.create_timestamp),
                            last_update_timestamp: parse_dt(&v.last_update_timestamp),
                            application_description: v.application_description,
                            tags: v.tags,
                            snapshots: v
                                .snapshots
                                .into_iter()
                                .map(|s| Snapshot {
                                    snapshot_name: s.snapshot_name,
                                    application_version_id: s.application_version_id,
                                    runtime_environment: s.runtime_environment,
                                    snapshot_creation_timestamp: parse_dt(
                                        &s.snapshot_creation_timestamp,
                                    ),
                                })
                                .collect(),
                            application_configuration: v.application_configuration,
                            cloudwatch_logging_options: v.cloudwatch_logging_options,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for KinesisAnalyticsV2Service {
    type StateView = KinesisAnalyticsV2StateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        KinesisAnalyticsV2StateView::from(&*guard)
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
            *guard = KinesisAnalyticsV2State::from(view);
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
            for (k, v) in view.applications {
                guard.applications.insert(
                    k,
                    Application {
                        application_name: v.application_name,
                        application_arn: v.application_arn,
                        application_status: v.application_status,
                        application_version_id: v.application_version_id,
                        runtime_environment: v.runtime_environment,
                        service_execution_role: v.service_execution_role,
                        create_timestamp: parse_dt(&v.create_timestamp),
                        last_update_timestamp: parse_dt(&v.last_update_timestamp),
                        application_description: v.application_description,
                        tags: v.tags,
                        snapshots: v
                            .snapshots
                            .into_iter()
                            .map(|s| Snapshot {
                                snapshot_name: s.snapshot_name,
                                application_version_id: s.application_version_id,
                                runtime_environment: s.runtime_environment,
                                snapshot_creation_timestamp: parse_dt(
                                    &s.snapshot_creation_timestamp,
                                ),
                            })
                            .collect(),
                        application_configuration: v.application_configuration,
                        cloudwatch_logging_options: v.cloudwatch_logging_options,
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
