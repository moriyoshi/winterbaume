use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;
use uuid::Uuid;

use crate::types::{DataSyncLocation, DataSyncTask, TaskExecution};

#[derive(Debug, Default)]
pub struct DataSyncState {
    /// Tasks keyed by TaskArn.
    pub tasks: HashMap<String, DataSyncTask>,
    /// Locations keyed by LocationArn.
    pub locations: HashMap<String, DataSyncLocation>,
    /// Task executions keyed by TaskExecutionArn.
    pub task_executions: HashMap<String, TaskExecution>,
}

#[derive(Debug, Error)]
pub enum DataSyncError {
    #[error("Task {task_arn} is not found.")]
    TaskNotFound { task_arn: String },

    #[error("Location {location_arn} is not found.")]
    LocationNotFound { location_arn: String },

    #[error("TaskExecution {task_execution_arn} is not found.")]
    TaskExecutionNotFound { task_execution_arn: String },
}

impl DataSyncState {
    pub fn create_task(
        &mut self,
        source_location_arn: &str,
        destination_location_arn: &str,
        cloud_watch_log_group_arn: Option<&str>,
        name: Option<&str>,
        region: &str,
        account_id: &str,
    ) -> Result<&DataSyncTask, DataSyncError> {
        let task_id = format!("task-{}", hex_id());
        let task_arn = format!("arn:aws:datasync:{region}:{account_id}:task/{task_id}");

        let task = DataSyncTask {
            task_arn: task_arn.clone(),
            name: name.map(|n| n.to_string()),
            status: "AVAILABLE".to_string(),
            source_location_arn: source_location_arn.to_string(),
            destination_location_arn: destination_location_arn.to_string(),
            cloud_watch_log_group_arn: cloud_watch_log_group_arn.map(|s| s.to_string()),
            creation_time: Utc::now(),
        };

        self.tasks.insert(task_arn.clone(), task);
        Ok(self.tasks.get(&task_arn).unwrap())
    }

    pub fn describe_task(&self, task_arn: &str) -> Result<&DataSyncTask, DataSyncError> {
        self.tasks
            .get(task_arn)
            .ok_or_else(|| DataSyncError::TaskNotFound {
                task_arn: task_arn.to_string(),
            })
    }

    pub fn delete_task(&mut self, task_arn: &str) -> Result<(), DataSyncError> {
        self.tasks
            .remove(task_arn)
            .ok_or_else(|| DataSyncError::TaskNotFound {
                task_arn: task_arn.to_string(),
            })?;
        Ok(())
    }

    pub fn list_tasks(&self) -> Vec<&DataSyncTask> {
        self.tasks.values().collect()
    }

    pub fn update_task(&mut self, task_arn: &str, name: Option<&str>) -> Result<(), DataSyncError> {
        let task = self
            .tasks
            .get_mut(task_arn)
            .ok_or_else(|| DataSyncError::TaskNotFound {
                task_arn: task_arn.to_string(),
            })?;
        if let Some(n) = name {
            task.name = Some(n.to_string());
        }
        Ok(())
    }

    pub fn create_location_s3(
        &mut self,
        s3_bucket_arn: &str,
        region: &str,
        account_id: &str,
    ) -> Result<&DataSyncLocation, DataSyncError> {
        let loc_id = format!("loc-{}", hex_id());
        let location_arn = format!("arn:aws:datasync:{region}:{account_id}:location/{loc_id}");
        let location_uri = format!(
            "s3://{}",
            s3_bucket_arn.split(':').next_back().unwrap_or("bucket")
        );
        let location = DataSyncLocation {
            location_arn: location_arn.clone(),
            location_uri,
            creation_time: Utc::now(),
        };
        self.locations.insert(location_arn.clone(), location);
        Ok(self.locations.get(&location_arn).unwrap())
    }

    pub fn delete_location(&mut self, location_arn: &str) -> Result<(), DataSyncError> {
        self.locations
            .remove(location_arn)
            .ok_or_else(|| DataSyncError::LocationNotFound {
                location_arn: location_arn.to_string(),
            })?;
        Ok(())
    }

    pub fn start_task_execution(
        &mut self,
        task_arn: &str,
        region: &str,
        account_id: &str,
    ) -> Result<&TaskExecution, DataSyncError> {
        if !self.tasks.contains_key(task_arn) {
            return Err(DataSyncError::TaskNotFound {
                task_arn: task_arn.to_string(),
            });
        }

        let exec_id = format!("exec-{}", hex_id());
        let task_execution_arn =
            format!("arn:aws:datasync:{region}:{account_id}:task/execution/{exec_id}");

        let execution = TaskExecution {
            task_execution_arn: task_execution_arn.clone(),
            task_arn: task_arn.to_string(),
            status: "LAUNCHING".to_string(),
            start_time: Utc::now(),
        };

        self.task_executions
            .insert(task_execution_arn.clone(), execution);
        Ok(self.task_executions.get(&task_execution_arn).unwrap())
    }

    pub fn cancel_task_execution(&mut self, task_execution_arn: &str) -> Result<(), DataSyncError> {
        let exec = self
            .task_executions
            .get_mut(task_execution_arn)
            .ok_or_else(|| DataSyncError::TaskExecutionNotFound {
                task_execution_arn: task_execution_arn.to_string(),
            })?;
        exec.status = "ERROR".to_string();
        Ok(())
    }
}

/// Generate a 17-character hex ID (matching the AWS datasync task ID pattern).
fn hex_id() -> String {
    let uuid = Uuid::new_v4();
    let hex = format!("{:032x}", uuid.as_u128());
    hex[..17].to_string()
}
