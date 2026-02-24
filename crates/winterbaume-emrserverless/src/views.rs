//! Serde-compatible view types for EMR Serverless state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::EmrServerlessService;
use crate::state::EmrServerlessState;
use crate::types::{
    Application, AutoStartConfig, AutoStopConfig, ConfigurationOverrides, InitialCapacityConfig,
    JobDriver, JobRun, MaximumCapacity, MonitoringConfiguration, NetworkConfiguration,
    S3MonitoringConfiguration, SparkSubmit, WorkerConfiguration,
};

/// Serializable view of the entire EMR Serverless state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmrServerlessStateView {
    /// Applications keyed by application ID.
    #[serde(default)]
    pub applications: HashMap<String, ApplicationView>,
    /// Job runs keyed by (application_id, job_run_id) encoded as "app_id:run_id".
    #[serde(default)]
    pub job_runs: HashMap<String, JobRunView>,
    /// Tags applied via TagResource keyed by resource ARN.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub resource_tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationView {
    pub application_id: String,
    pub name: String,
    pub arn: String,
    pub release_label: String,
    pub application_type: String,
    pub state: String,
    pub state_details: String,
    pub auto_start_configuration: AutoStartConfigView,
    pub auto_stop_configuration: AutoStopConfigView,
    pub initial_capacity: Option<HashMap<String, InitialCapacityConfigView>>,
    pub maximum_capacity: Option<MaximumCapacityView>,
    pub network_configuration: Option<NetworkConfigurationView>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub created_at: String,
    pub updated_at: String,
    /// `image_configuration` nested block.
    #[serde(default)]
    pub image_configuration: Option<ImageConfigurationView>,
    /// `interactive_configuration` nested block.
    #[serde(default)]
    pub interactive_configuration: Option<InteractiveConfigurationView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoStartConfigView {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoStopConfigView {
    pub enabled: bool,
    pub idle_timeout_minutes: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitialCapacityConfigView {
    pub worker_count: i64,
    pub worker_configuration: Option<WorkerConfigurationView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerConfigurationView {
    pub cpu: String,
    pub memory: String,
    pub disk: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaximumCapacityView {
    pub cpu: String,
    pub memory: String,
    pub disk: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfigurationView {
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    #[serde(default)]
    pub security_group_ids: Vec<String>,
}

/// Serializable view of an image configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfigurationView {
    /// ECR image URI for the application image.
    pub image_uri: String,
}

/// Serializable view of an interactive configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveConfigurationView {
    /// Whether the Jupyter notebook endpoint is enabled.
    #[serde(default)]
    pub studio_enabled: bool,
    /// Whether the LiveUI endpoint is enabled.
    #[serde(default)]
    pub livy_endpoint_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobRunView {
    pub application_id: String,
    pub job_run_id: String,
    pub name: String,
    pub arn: String,
    pub execution_role_arn: String,
    pub job_driver: JobDriverView,
    pub configuration_overrides: Option<ConfigurationOverridesView>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub state: String,
    pub state_details: String,
    pub created_at: String,
    pub updated_at: String,
    pub execution_timeout_minutes: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobDriverView {
    pub spark_submit: Option<SparkSubmitView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SparkSubmitView {
    pub entry_point: String,
    #[serde(default)]
    pub entry_point_arguments: Vec<String>,
    pub spark_submit_parameters: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationOverridesView {
    pub monitoring_configuration: Option<MonitoringConfigurationView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfigurationView {
    pub s3_monitoring_configuration: Option<S3MonitoringConfigurationView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3MonitoringConfigurationView {
    pub log_uri: String,
}

// --- From internal types to view types ---

impl From<&EmrServerlessState> for EmrServerlessStateView {
    fn from(state: &EmrServerlessState) -> Self {
        EmrServerlessStateView {
            applications: state
                .applications
                .iter()
                .map(|(k, v)| (k.clone(), ApplicationView::from(v)))
                .collect(),
            job_runs: state
                .job_runs
                .iter()
                .map(|((app_id, run_id), v)| {
                    (format!("{}:{}", app_id, run_id), JobRunView::from(v))
                })
                .collect(),
            resource_tags: state.resource_tags.clone(),
        }
    }
}

impl From<&Application> for ApplicationView {
    fn from(a: &Application) -> Self {
        ApplicationView {
            application_id: a.application_id.clone(),
            name: a.name.clone(),
            arn: a.arn.clone(),
            release_label: a.release_label.clone(),
            application_type: a.application_type.clone(),
            state: a.state.clone(),
            state_details: a.state_details.clone(),
            auto_start_configuration: AutoStartConfigView {
                enabled: a.auto_start_configuration.enabled,
            },
            auto_stop_configuration: AutoStopConfigView {
                enabled: a.auto_stop_configuration.enabled,
                idle_timeout_minutes: a.auto_stop_configuration.idle_timeout_minutes,
            },
            initial_capacity: a.initial_capacity.as_ref().map(|ic| {
                ic.iter()
                    .map(|(k, v)| {
                        (
                            k.clone(),
                            InitialCapacityConfigView {
                                worker_count: v.worker_count,
                                worker_configuration: v.worker_configuration.as_ref().map(|wc| {
                                    WorkerConfigurationView {
                                        cpu: wc.cpu.clone(),
                                        memory: wc.memory.clone(),
                                        disk: wc.disk.clone(),
                                    }
                                }),
                            },
                        )
                    })
                    .collect()
            }),
            maximum_capacity: a.maximum_capacity.as_ref().map(|mc| MaximumCapacityView {
                cpu: mc.cpu.clone(),
                memory: mc.memory.clone(),
                disk: mc.disk.clone(),
            }),
            network_configuration: a.network_configuration.as_ref().map(|nc| {
                NetworkConfigurationView {
                    subnet_ids: nc.subnet_ids.clone(),
                    security_group_ids: nc.security_group_ids.clone(),
                }
            }),
            tags: a.tags.clone(),
            created_at: a.created_at.clone(),
            updated_at: a.updated_at.clone(),
            image_configuration: None,
            interactive_configuration: None,
        }
    }
}

impl From<&JobRun> for JobRunView {
    fn from(jr: &JobRun) -> Self {
        JobRunView {
            application_id: jr.application_id.clone(),
            job_run_id: jr.job_run_id.clone(),
            name: jr.name.clone(),
            arn: jr.arn.clone(),
            execution_role_arn: jr.execution_role_arn.clone(),
            job_driver: JobDriverView {
                spark_submit: jr
                    .job_driver
                    .spark_submit
                    .as_ref()
                    .map(|ss| SparkSubmitView {
                        entry_point: ss.entry_point.clone(),
                        entry_point_arguments: ss.entry_point_arguments.clone(),
                        spark_submit_parameters: ss.spark_submit_parameters.clone(),
                    }),
            },
            configuration_overrides: jr.configuration_overrides.as_ref().map(|co| {
                ConfigurationOverridesView {
                    monitoring_configuration: co.monitoring_configuration.as_ref().map(|mc| {
                        MonitoringConfigurationView {
                            s3_monitoring_configuration: mc
                                .s3_monitoring_configuration
                                .as_ref()
                                .map(|s3| S3MonitoringConfigurationView {
                                    log_uri: s3.log_uri.clone(),
                                }),
                        }
                    }),
                }
            }),
            tags: jr.tags.clone(),
            state: jr.state.clone(),
            state_details: jr.state_details.clone(),
            created_at: jr.created_at.clone(),
            updated_at: jr.updated_at.clone(),
            execution_timeout_minutes: jr.execution_timeout_minutes,
        }
    }
}

// --- From view types to internal types ---

impl From<EmrServerlessStateView> for EmrServerlessState {
    fn from(view: EmrServerlessStateView) -> Self {
        let applications = view
            .applications
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    Application {
                        application_id: v.application_id,
                        name: v.name,
                        arn: v.arn,
                        release_label: v.release_label,
                        application_type: v.application_type,
                        state: v.state,
                        state_details: v.state_details,
                        auto_start_configuration: AutoStartConfig {
                            enabled: v.auto_start_configuration.enabled,
                        },
                        auto_stop_configuration: AutoStopConfig {
                            enabled: v.auto_stop_configuration.enabled,
                            idle_timeout_minutes: v.auto_stop_configuration.idle_timeout_minutes,
                        },
                        initial_capacity: v.initial_capacity.map(|ic| {
                            ic.into_iter()
                                .map(|(k, v)| {
                                    (
                                        k,
                                        InitialCapacityConfig {
                                            worker_count: v.worker_count,
                                            worker_configuration: v.worker_configuration.map(
                                                |wc| WorkerConfiguration {
                                                    cpu: wc.cpu,
                                                    memory: wc.memory,
                                                    disk: wc.disk,
                                                },
                                            ),
                                        },
                                    )
                                })
                                .collect()
                        }),
                        maximum_capacity: v.maximum_capacity.map(|mc| MaximumCapacity {
                            cpu: mc.cpu,
                            memory: mc.memory,
                            disk: mc.disk,
                        }),
                        network_configuration: v.network_configuration.map(|nc| {
                            NetworkConfiguration {
                                subnet_ids: nc.subnet_ids,
                                security_group_ids: nc.security_group_ids,
                            }
                        }),
                        tags: v.tags,
                        created_at: v.created_at,
                        updated_at: v.updated_at,
                    },
                )
            })
            .collect();

        let job_runs = view
            .job_runs
            .into_iter()
            .map(|(key, v)| {
                // key is "app_id:run_id"
                let app_id = v.application_id.clone();
                let run_id = v.job_run_id.clone();
                let jr = JobRun {
                    application_id: v.application_id,
                    job_run_id: v.job_run_id,
                    name: v.name,
                    arn: v.arn,
                    execution_role_arn: v.execution_role_arn,
                    job_driver: JobDriver {
                        spark_submit: v.job_driver.spark_submit.map(|ss| SparkSubmit {
                            entry_point: ss.entry_point,
                            entry_point_arguments: ss.entry_point_arguments,
                            spark_submit_parameters: ss.spark_submit_parameters,
                        }),
                    },
                    configuration_overrides: v.configuration_overrides.map(|co| {
                        ConfigurationOverrides {
                            monitoring_configuration: co.monitoring_configuration.map(|mc| {
                                MonitoringConfiguration {
                                    s3_monitoring_configuration: mc
                                        .s3_monitoring_configuration
                                        .map(|s3| S3MonitoringConfiguration {
                                            log_uri: s3.log_uri,
                                        }),
                                }
                            }),
                        }
                    }),
                    tags: v.tags,
                    state: v.state,
                    state_details: v.state_details,
                    created_at: v.created_at,
                    updated_at: v.updated_at,
                    execution_timeout_minutes: v.execution_timeout_minutes,
                };
                ((app_id, run_id), jr)
            })
            .collect();

        EmrServerlessState {
            applications,
            job_runs,
            resource_tags: view.resource_tags,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for EmrServerlessService {
    type StateView = EmrServerlessStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        EmrServerlessStateView::from(&*guard)
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
            *guard = EmrServerlessState::from(view);
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
            let incoming = EmrServerlessState::from(view);
            for (k, v) in incoming.applications {
                guard.applications.insert(k, v);
            }
            for (k, v) in incoming.job_runs {
                guard.job_runs.insert(k, v);
            }
            for (k, v) in incoming.resource_tags {
                guard.resource_tags.entry(k).or_default().extend(v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
