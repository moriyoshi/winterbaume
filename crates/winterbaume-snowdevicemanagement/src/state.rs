use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Default)]
pub struct SnowDeviceManagementState {
    /// Tasks keyed by taskId.
    pub tasks: HashMap<String, TaskRecord>,
    /// Devices keyed by managedDeviceId.
    pub devices: HashMap<String, DeviceRecord>,
    /// Executions keyed by (taskId, managedDeviceId).
    pub executions: HashMap<(String, String), ExecutionRecord>,
    /// Tags keyed by ARN.
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone)]
pub struct TaskRecord {
    pub task_id: String,
    pub task_arn: String,
    pub description: Option<String>,
    pub state: String,
    pub created_at: f64,
    pub last_updated_at: f64,
    pub completed_at: Option<f64>,
    pub command: serde_json::Value,
    pub targets: Vec<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct DeviceRecord {
    pub managed_device_id: String,
    pub managed_device_arn: String,
    pub job_id: Option<String>,
    pub state: String,
    pub device_type: String,
    pub last_reached_out_at: Option<f64>,
    pub last_updated_at: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct ExecutionRecord {
    pub task_id: String,
    pub managed_device_id: String,
    pub execution_id: String,
    pub state: String,
    pub last_updated_at: f64,
}

#[derive(Debug, Error)]
pub enum SnowDeviceManagementError {
    #[error("Task {id} not found.")]
    TaskNotFound { id: String },

    #[error("Device {id} not found.")]
    DeviceNotFound { id: String },

    #[error("Execution for task {task_id} device {device_id} not found.")]
    ExecutionNotFound { task_id: String, device_id: String },

    #[error("{message}")]
    Validation { message: String },
}

impl SnowDeviceManagementState {
    pub fn create_task(&mut self, task: TaskRecord) -> &TaskRecord {
        let id = task.task_id.clone();
        // Auto-create matching executions for each target.
        for device_id in &task.targets {
            self.executions.insert(
                (id.clone(), device_id.clone()),
                ExecutionRecord {
                    task_id: id.clone(),
                    managed_device_id: device_id.clone(),
                    execution_id: format!("exec-{}", uuid::Uuid::new_v4().simple()),
                    state: "QUEUED".to_string(),
                    last_updated_at: task.created_at,
                },
            );
        }
        self.tasks.insert(id.clone(), task);
        self.tasks.get(&id).unwrap()
    }

    pub fn get_task(&self, id: &str) -> Result<&TaskRecord, SnowDeviceManagementError> {
        self.tasks
            .get(id)
            .ok_or(SnowDeviceManagementError::TaskNotFound { id: id.to_string() })
    }

    pub fn cancel_task(&mut self, id: &str) -> Result<(), SnowDeviceManagementError> {
        let task = self
            .tasks
            .get_mut(id)
            .ok_or(SnowDeviceManagementError::TaskNotFound { id: id.to_string() })?;
        task.state = "CANCELLED".to_string();
        task.last_updated_at = chrono::Utc::now().timestamp() as f64;
        Ok(())
    }

    pub fn list_tasks<'a>(&'a self, state_filter: Option<&'a str>) -> Vec<&'a TaskRecord> {
        let mut v: Vec<&TaskRecord> = self
            .tasks
            .values()
            .filter(|t| state_filter.is_none_or(|s| s == t.state))
            .collect();
        v.sort_by(|a, b| a.task_id.cmp(&b.task_id));
        v
    }

    pub fn upsert_device(&mut self, device: DeviceRecord) -> &DeviceRecord {
        let id = device.managed_device_id.clone();
        self.devices.insert(id.clone(), device);
        self.devices.get(&id).unwrap()
    }

    pub fn get_device(&self, id: &str) -> Result<&DeviceRecord, SnowDeviceManagementError> {
        self.devices
            .get(id)
            .ok_or(SnowDeviceManagementError::DeviceNotFound { id: id.to_string() })
    }

    pub fn list_devices<'a>(&'a self, job_id: Option<&'a str>) -> Vec<&'a DeviceRecord> {
        let mut v: Vec<&DeviceRecord> = self
            .devices
            .values()
            .filter(|d| job_id.is_none_or(|j| d.job_id.as_deref() == Some(j)))
            .collect();
        v.sort_by(|a, b| a.managed_device_id.cmp(&b.managed_device_id));
        v
    }

    pub fn get_execution(
        &self,
        task_id: &str,
        device_id: &str,
    ) -> Result<&ExecutionRecord, SnowDeviceManagementError> {
        self.executions
            .get(&(task_id.to_string(), device_id.to_string()))
            .ok_or(SnowDeviceManagementError::ExecutionNotFound {
                task_id: task_id.to_string(),
                device_id: device_id.to_string(),
            })
    }

    pub fn list_executions_by_task<'a>(
        &'a self,
        task_id: Option<&'a str>,
        state_filter: Option<&'a str>,
    ) -> Vec<&'a ExecutionRecord> {
        let mut v: Vec<&ExecutionRecord> = self
            .executions
            .values()
            .filter(|e| task_id.is_none_or(|t| t == e.task_id))
            .filter(|e| state_filter.is_none_or(|s| s == e.state))
            .collect();
        v.sort_by(|a, b| {
            (a.task_id.as_str(), a.managed_device_id.as_str())
                .cmp(&(b.task_id.as_str(), b.managed_device_id.as_str()))
        });
        v
    }

    pub fn tag_resource(&mut self, arn: &str, tags: HashMap<String, String>) {
        let entry = self.tags.entry(arn.to_string()).or_default();
        for (k, v) in tags {
            entry.insert(k, v);
        }
    }

    pub fn untag_resource(&mut self, arn: &str, keys: &[String]) {
        if let Some(entry) = self.tags.get_mut(arn) {
            for k in keys {
                entry.remove(k);
            }
        }
    }

    pub fn list_tags(&self, arn: &str) -> HashMap<String, String> {
        self.tags.get(arn).cloned().unwrap_or_default()
    }

    pub fn ensure_seeded(&mut self, account_id: &str, region: &str) {
        if !self.devices.is_empty() {
            return;
        }
        let id = "smd-default";
        let arn =
            format!("arn:aws:snow-device-management:{region}:{account_id}:managed-device/{id}");
        self.devices.insert(
            id.to_string(),
            DeviceRecord {
                managed_device_id: id.to_string(),
                managed_device_arn: arn,
                job_id: Some("JID-DEFAULT".to_string()),
                state: "ACTIVATED".to_string(),
                device_type: "SNOWBALL".to_string(),
                last_reached_out_at: None,
                last_updated_at: None,
            },
        );
    }
}
