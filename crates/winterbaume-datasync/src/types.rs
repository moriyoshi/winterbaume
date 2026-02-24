use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct DataSyncTask {
    pub task_arn: String,
    pub name: Option<String>,
    pub status: String,
    pub source_location_arn: String,
    pub destination_location_arn: String,
    pub cloud_watch_log_group_arn: Option<String>,
    pub creation_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct DataSyncLocation {
    pub location_arn: String,
    pub location_uri: String,
    pub creation_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct TaskExecution {
    pub task_execution_arn: String,
    pub task_arn: String,
    pub status: String,
    pub start_time: DateTime<Utc>,
}
