use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::{
    Canary, CanaryRunRecord as CanaryRun, CanaryRunRecordStatus, CanaryStatus, CreateCanaryInput,
    Group,
};

#[derive(Debug, Default)]
pub struct SyntheticsState {
    pub canaries: HashMap<String, Canary>,
    pub canary_runs: HashMap<String, Vec<CanaryRun>>,
    pub groups: HashMap<String, Group>,
}

#[derive(Debug, Error)]
pub enum SyntheticsError {
    #[error("{0} is required")]
    FieldRequired(&'static str),

    #[error("Canary with name {name} already exists in account {account_id} in region {region}")]
    CanaryAlreadyExists {
        name: String,
        account_id: String,
        region: String,
    },

    #[error("Group with name {0} already exists")]
    GroupAlreadyExists(String),

    #[error("Canary {0} not found")]
    CanaryNotFound(String),

    #[error("Group {0} not found")]
    GroupNotFound(String),

    #[error("Resource {0} not found")]
    ResourceNotFound(String),
}

impl SyntheticsState {
    pub fn create_canary(
        &mut self,
        input: &CreateCanaryInput,
        account_id: &str,
        region: &str,
    ) -> Result<&Canary, SyntheticsError> {
        let name = input
            .name
            .as_deref()
            .ok_or(SyntheticsError::FieldRequired("Name"))?;

        if self.canaries.contains_key(name) {
            return Err(SyntheticsError::CanaryAlreadyExists {
                name: name.to_string(),
                account_id: account_id.to_string(),
                region: region.to_string(),
            });
        }

        let code = input
            .code
            .as_ref()
            .ok_or(SyntheticsError::FieldRequired("Code"))?;

        let handler = code
            .handler
            .as_deref()
            .ok_or(SyntheticsError::FieldRequired("Code handler"))?;

        let artifact_s3_location = input
            .artifact_s3_location
            .as_deref()
            .ok_or(SyntheticsError::FieldRequired("ArtifactS3Location"))?;

        let execution_role_arn = input
            .execution_role_arn
            .as_deref()
            .ok_or(SyntheticsError::FieldRequired("ExecutionRoleArn"))?;

        let schedule = input
            .schedule
            .as_ref()
            .ok_or(SyntheticsError::FieldRequired("Schedule"))?;

        let expression = schedule
            .expression
            .as_deref()
            .ok_or(SyntheticsError::FieldRequired("Schedule expression"))?;

        let runtime_version = input
            .runtime_version
            .as_deref()
            .ok_or(SyntheticsError::FieldRequired("RuntimeVersion"))?;

        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:synthetics:{region}:{account_id}:canary:{name}");
        let now = Utc::now();

        let canary = Canary {
            name: name.to_string(),
            id,
            arn,
            artifact_s3_location: artifact_s3_location.to_string(),
            runtime_version: runtime_version.to_string(),
            handler: handler.to_string(),
            schedule_expression: expression.to_string(),
            schedule_duration_in_seconds: schedule.duration_in_seconds,
            success_retention_period_in_days: input.success_retention_period_in_days.unwrap_or(31),
            failure_retention_period_in_days: input.failure_retention_period_in_days.unwrap_or(31),
            status: CanaryStatus {
                state: "CREATING".to_string(),
                state_reason: None,
                state_reason_code: None,
            },
            created_at: now,
            last_modified: now,
            execution_role_arn: execution_role_arn.to_string(),
            s3_encryption_mode: None,
            tags: input.tags.clone().unwrap_or_default(),
        };

        self.canaries.insert(name.to_string(), canary);
        Ok(self.canaries.get(name).unwrap())
    }

    pub fn get_canary(&self, name: &str) -> Result<&Canary, SyntheticsError> {
        self.canaries
            .get(name)
            .ok_or_else(|| SyntheticsError::CanaryNotFound(name.to_string()))
    }

    pub fn delete_canary(&mut self, name: &str) -> Result<(), SyntheticsError> {
        if self.canaries.remove(name).is_none() {
            return Err(SyntheticsError::CanaryNotFound(name.to_string()));
        }
        Ok(())
    }

    pub fn describe_canaries(&self) -> Vec<&Canary> {
        self.canaries.values().collect()
    }

    pub fn describe_canaries_last_run(&self) -> Vec<(&str, Option<()>)> {
        // Returns a list of (canary_name, None) — we don't track run history; return empty last runs.
        self.canaries.keys().map(|k| (k.as_str(), None)).collect()
    }

    pub fn get_canary_runs(&self, name: &str) -> Result<&[CanaryRun], SyntheticsError> {
        if !self.canaries.contains_key(name) {
            return Err(SyntheticsError::CanaryNotFound(name.to_string()));
        }
        Ok(self
            .canary_runs
            .get(name)
            .map(|v| v.as_slice())
            .unwrap_or(&[]))
    }

    pub fn start_canary(&mut self, name: &str) -> Result<(), SyntheticsError> {
        let canary = self
            .canaries
            .get_mut(name)
            .ok_or_else(|| SyntheticsError::CanaryNotFound(name.to_string()))?;
        canary.status.state = "RUNNING".to_string();
        canary.status.state_reason = None;
        canary.last_modified = Utc::now();
        let artifact_s3_location = canary.artifact_s3_location.clone();
        let run = CanaryRun {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            artifact_s3_location,
            status: CanaryRunRecordStatus {
                state: "RUNNING".to_string(),
                state_reason: None,
                state_reason_code: None,
                test_result: None,
            },
            started_at: Utc::now(),
            completed_at: None,
        };
        self.canary_runs
            .entry(name.to_string())
            .or_default()
            .push(run);
        Ok(())
    }

    pub fn stop_canary(&mut self, name: &str) -> Result<(), SyntheticsError> {
        let canary = self
            .canaries
            .get_mut(name)
            .ok_or_else(|| SyntheticsError::CanaryNotFound(name.to_string()))?;
        canary.status.state = "STOPPED".to_string();
        canary.last_modified = Utc::now();
        Ok(())
    }

    pub fn update_canary(
        &mut self,
        name: &str,
        runtime_version: Option<&str>,
        execution_role_arn: Option<&str>,
        schedule_expression: Option<&str>,
        schedule_duration: Option<i64>,
        success_retention: Option<i32>,
        failure_retention: Option<i32>,
        handler: Option<&str>,
        artifact_s3_location: Option<&str>,
    ) -> Result<(), SyntheticsError> {
        let canary = self
            .canaries
            .get_mut(name)
            .ok_or_else(|| SyntheticsError::CanaryNotFound(name.to_string()))?;
        if let Some(v) = runtime_version {
            canary.runtime_version = v.to_string();
        }
        if let Some(v) = execution_role_arn {
            canary.execution_role_arn = v.to_string();
        }
        if let Some(v) = schedule_expression {
            canary.schedule_expression = v.to_string();
        }
        if let Some(v) = schedule_duration {
            canary.schedule_duration_in_seconds = Some(v);
        }
        if let Some(v) = success_retention {
            canary.success_retention_period_in_days = v;
        }
        if let Some(v) = failure_retention {
            canary.failure_retention_period_in_days = v;
        }
        if let Some(v) = handler {
            canary.handler = v.to_string();
        }
        if let Some(v) = artifact_s3_location {
            canary.artifact_s3_location = v.to_string();
        }
        canary.last_modified = Utc::now();
        Ok(())
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), SyntheticsError> {
        // Try canary first
        if let Some(canary) = self.canaries.values_mut().find(|c| c.arn == resource_arn) {
            canary.tags.extend(tags);
            return Ok(());
        }
        // Try group
        if let Some(group) = self.groups.values_mut().find(|g| g.arn == resource_arn) {
            group.tags.extend(tags);
            return Ok(());
        }
        Err(SyntheticsError::ResourceNotFound(resource_arn.to_string()))
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), SyntheticsError> {
        // Try canary first
        if let Some(canary) = self.canaries.values_mut().find(|c| c.arn == resource_arn) {
            for key in tag_keys {
                canary.tags.remove(key);
            }
            return Ok(());
        }
        // Try group
        if let Some(group) = self.groups.values_mut().find(|g| g.arn == resource_arn) {
            for key in tag_keys {
                group.tags.remove(key);
            }
            return Ok(());
        }
        Err(SyntheticsError::ResourceNotFound(resource_arn.to_string()))
    }

    pub fn get_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<HashMap<String, String>, SyntheticsError> {
        if let Some(canary) = self.canaries.values().find(|c| c.arn == resource_arn) {
            return Ok(canary.tags.clone());
        }
        if let Some(group) = self.groups.values().find(|g| g.arn == resource_arn) {
            return Ok(group.tags.clone());
        }
        Err(SyntheticsError::ResourceNotFound(resource_arn.to_string()))
    }

    // --- Group operations ---

    pub fn create_group(
        &mut self,
        name: &str,
        tags: Option<HashMap<String, String>>,
        account_id: &str,
        region: &str,
    ) -> Result<&Group, SyntheticsError> {
        if self.groups.contains_key(name) {
            return Err(SyntheticsError::GroupAlreadyExists(name.to_string()));
        }
        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:synthetics:{region}:{account_id}:group:{name}");
        let now = Utc::now();
        let group = Group {
            name: name.to_string(),
            id,
            arn,
            resource_arns: Default::default(),
            created_time: now,
            last_modified_time: now,
            tags: tags.unwrap_or_default(),
        };
        self.groups.insert(name.to_string(), group);
        Ok(self.groups.get(name).unwrap())
    }

    pub fn get_group(&self, identifier: &str) -> Result<&Group, SyntheticsError> {
        // identifier can be name or id
        if let Some(g) = self.groups.get(identifier) {
            return Ok(g);
        }
        if let Some(g) = self.groups.values().find(|g| g.id == identifier) {
            return Ok(g);
        }
        Err(SyntheticsError::GroupNotFound(identifier.to_string()))
    }

    pub fn delete_group(&mut self, identifier: &str) -> Result<(), SyntheticsError> {
        // identifier can be name or id
        let name = if self.groups.contains_key(identifier) {
            identifier.to_string()
        } else if let Some(n) = self
            .groups
            .iter()
            .find(|(_, g)| g.id == identifier)
            .map(|(k, _)| k.clone())
        {
            n
        } else {
            return Err(SyntheticsError::GroupNotFound(identifier.to_string()));
        };
        self.groups.remove(&name);
        Ok(())
    }

    pub fn list_groups(&self) -> Vec<&Group> {
        self.groups.values().collect()
    }

    pub fn associate_resource(
        &mut self,
        group_identifier: &str,
        resource_arn: &str,
    ) -> Result<(), SyntheticsError> {
        let group = if self.groups.contains_key(group_identifier) {
            self.groups.get_mut(group_identifier).unwrap()
        } else if let Some(k) = self
            .groups
            .iter()
            .find(|(_, g)| g.id == group_identifier)
            .map(|(k, _)| k.clone())
        {
            self.groups.get_mut(&k).unwrap()
        } else {
            return Err(SyntheticsError::GroupNotFound(group_identifier.to_string()));
        };
        group.resource_arns.insert(resource_arn.to_string());
        Ok(())
    }

    pub fn disassociate_resource(
        &mut self,
        group_identifier: &str,
        resource_arn: &str,
    ) -> Result<(), SyntheticsError> {
        let group = if self.groups.contains_key(group_identifier) {
            self.groups.get_mut(group_identifier).unwrap()
        } else if let Some(k) = self
            .groups
            .iter()
            .find(|(_, g)| g.id == group_identifier)
            .map(|(k, _)| k.clone())
        {
            self.groups.get_mut(&k).unwrap()
        } else {
            return Err(SyntheticsError::GroupNotFound(group_identifier.to_string()));
        };
        group.resource_arns.remove(resource_arn);
        Ok(())
    }

    pub fn list_group_resources(
        &self,
        group_identifier: &str,
    ) -> Result<Vec<String>, SyntheticsError> {
        let group = if let Some(g) = self.groups.get(group_identifier) {
            g
        } else if let Some(g) = self.groups.values().find(|g| g.id == group_identifier) {
            g
        } else {
            return Err(SyntheticsError::GroupNotFound(group_identifier.to_string()));
        };
        Ok(group.resource_arns.iter().cloned().collect())
    }

    pub fn list_associated_groups(&self, resource_arn: &str) -> Vec<&Group> {
        self.groups
            .values()
            .filter(|g| g.resource_arns.contains(resource_arn))
            .collect()
    }
}
