use std::collections::HashMap;

/// An EMR Serverless application.
#[derive(Debug, Clone)]
pub struct Application {
    pub application_id: String,
    pub name: String,
    pub arn: String,
    pub release_label: String,
    pub application_type: String,
    pub state: String,
    pub state_details: String,
    pub auto_start_configuration: AutoStartConfig,
    pub auto_stop_configuration: AutoStopConfig,
    pub initial_capacity: Option<HashMap<String, InitialCapacityConfig>>,
    pub maximum_capacity: Option<MaximumCapacity>,
    pub network_configuration: Option<NetworkConfiguration>,
    pub tags: HashMap<String, String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct AutoStartConfig {
    pub enabled: bool,
}

impl Default for AutoStartConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Debug, Clone)]
pub struct AutoStopConfig {
    pub enabled: bool,
    pub idle_timeout_minutes: i64,
}

impl Default for AutoStopConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            idle_timeout_minutes: 15,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InitialCapacityConfig {
    pub worker_count: i64,
    pub worker_configuration: Option<WorkerConfiguration>,
}

#[derive(Debug, Clone)]
pub struct WorkerConfiguration {
    pub cpu: String,
    pub memory: String,
    pub disk: String,
}

#[derive(Debug, Clone)]
pub struct MaximumCapacity {
    pub cpu: String,
    pub memory: String,
    pub disk: String,
}

#[derive(Debug, Clone)]
pub struct NetworkConfiguration {
    pub subnet_ids: Vec<String>,
    pub security_group_ids: Vec<String>,
}

/// A job run.
#[derive(Debug, Clone)]
pub struct JobRun {
    pub application_id: String,
    pub job_run_id: String,
    pub name: String,
    pub arn: String,
    pub execution_role_arn: String,
    pub job_driver: JobDriver,
    pub configuration_overrides: Option<ConfigurationOverrides>,
    pub tags: HashMap<String, String>,
    pub state: String,
    pub state_details: String,
    pub created_at: String,
    pub updated_at: String,
    pub execution_timeout_minutes: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct JobDriver {
    pub spark_submit: Option<SparkSubmit>,
}

#[derive(Debug, Clone)]
pub struct SparkSubmit {
    pub entry_point: String,
    pub entry_point_arguments: Vec<String>,
    pub spark_submit_parameters: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ConfigurationOverrides {
    pub monitoring_configuration: Option<MonitoringConfiguration>,
}

#[derive(Debug, Clone)]
pub struct MonitoringConfiguration {
    pub s3_monitoring_configuration: Option<S3MonitoringConfiguration>,
}

#[derive(Debug, Clone)]
pub struct S3MonitoringConfiguration {
    pub log_uri: String,
}

/// Valid release labels
pub const VALID_RELEASE_LABELS: &[&str] = &[
    "emr-6.6.0",
    "emr-6.7.0",
    "emr-6.8.0",
    "emr-6.9.0",
    "emr-6.10.0",
    "emr-6.11.0",
    "emr-6.12.0",
    "emr-6.13.0",
    "emr-6.14.0",
    "emr-6.15.0",
    "emr-7.0.0",
    "emr-7.1.0",
    "emr-7.2.0",
    "emr-7.3.0",
    "emr-7.4.0",
    "emr-7.5.0",
    "emr-7.6.0",
    "emr-7.7.0",
];

/// Default release label
pub const DEFAULT_RELEASE_LABEL: &str = "emr-6.6.0";

/// Default region
pub const DEFAULT_REGION: &str = "us-east-1";

/// Valid application types
pub const VALID_TYPES: &[&str] = &["SPARK", "HIVE"];
