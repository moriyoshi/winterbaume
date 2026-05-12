use std::collections::HashMap;

use chrono::Utc;
use serde_json::Value;
use thiserror::Error;
use winterbaume_core::default_account_id;

use crate::types::*;

#[derive(Debug, Default)]
pub struct MediaStoreState {
    pub containers: HashMap<String, Container>,
}

#[derive(Debug, Error)]
pub enum MediaStoreError {
    #[error("Container with name {name} already exists")]
    ContainerInUse { name: String },

    #[error("The specified container does not exist")]
    ResourceNotFound,

    #[error("The specified container does not exist")]
    ContainerNotFound,

    #[error("The policy does not exist within the specfied container")]
    PolicyNotFound,
}

impl MediaStoreState {
    pub fn create_container(
        &mut self,
        name: &str,
        tags: Option<Vec<Tag>>,
    ) -> Result<&Container, MediaStoreError> {
        if self.containers.contains_key(name) {
            return Err(MediaStoreError::ContainerInUse {
                name: name.to_string(),
            });
        }
        // FIX(terraform-e2e): standard AWS MediaStore container ARN format is
        // `arn:aws:mediastore:<region>:<account-id>:container/<name>`. The previous
        // `arn:aws:mediastore:container:<name>` shape broke terraform's ListTagsForResource
        // round-trip (terraform sends the ARN back, and the handler parses the trailing
        // path component as the container name).
        let arn = format!(
            "arn:aws:mediastore:us-east-1:{account_id}:container/{name}",
            account_id = default_account_id()
        );
        let creation_time = Utc::now().timestamp().to_string();
        let container = Container {
            arn,
            name: name.to_string(),
            endpoint: format!("/{name}"),
            status: "CREATING".to_string(),
            creation_time,
            lifecycle_policy: None,
            policy: None,
            metric_policy: None,
            tags,
        };
        self.containers.insert(name.to_string(), container);
        Ok(self.containers.get(name).unwrap())
    }

    pub fn describe_container(&mut self, name: &str) -> Result<&Container, MediaStoreError> {
        if !self.containers.contains_key(name) {
            return Err(MediaStoreError::ResourceNotFound);
        }
        // Update status to ACTIVE on describe (matching moto behavior)
        if let Some(c) = self.containers.get_mut(name) {
            c.status = "ACTIVE".to_string();
        }
        Ok(self.containers.get(name).unwrap())
    }

    pub fn delete_container(&mut self, name: &str) -> Result<(), MediaStoreError> {
        if self.containers.remove(name).is_none() {
            return Err(MediaStoreError::ContainerNotFound);
        }
        Ok(())
    }

    pub fn list_containers(&self) -> Vec<&Container> {
        self.containers.values().collect()
    }

    pub fn list_tags_for_resource(&self, name: &str) -> Result<Option<&Vec<Tag>>, MediaStoreError> {
        let container = self
            .containers
            .get(name)
            .ok_or(MediaStoreError::ContainerNotFound)?;
        Ok(container.tags.as_ref())
    }

    pub fn put_lifecycle_policy(
        &mut self,
        container_name: &str,
        lifecycle_policy: &str,
    ) -> Result<(), MediaStoreError> {
        let container = self
            .containers
            .get_mut(container_name)
            .ok_or(MediaStoreError::ResourceNotFound)?;
        container.lifecycle_policy = Some(lifecycle_policy.to_string());
        Ok(())
    }

    pub fn get_lifecycle_policy(&self, container_name: &str) -> Result<&str, MediaStoreError> {
        let container = self
            .containers
            .get(container_name)
            .ok_or(MediaStoreError::ResourceNotFound)?;
        container
            .lifecycle_policy
            .as_deref()
            .ok_or(MediaStoreError::PolicyNotFound)
    }

    pub fn put_container_policy(
        &mut self,
        container_name: &str,
        policy: &str,
    ) -> Result<(), MediaStoreError> {
        let container = self
            .containers
            .get_mut(container_name)
            .ok_or(MediaStoreError::ResourceNotFound)?;
        container.policy = Some(policy.to_string());
        Ok(())
    }

    pub fn get_container_policy(&self, container_name: &str) -> Result<&str, MediaStoreError> {
        let container = self
            .containers
            .get(container_name)
            .ok_or(MediaStoreError::ResourceNotFound)?;
        container
            .policy
            .as_deref()
            .ok_or(MediaStoreError::PolicyNotFound)
    }

    pub fn put_metric_policy(
        &mut self,
        container_name: &str,
        metric_policy: Value,
    ) -> Result<(), MediaStoreError> {
        let container = self
            .containers
            .get_mut(container_name)
            .ok_or(MediaStoreError::ResourceNotFound)?;
        container.metric_policy = Some(metric_policy);
        Ok(())
    }

    pub fn get_metric_policy(&self, container_name: &str) -> Result<&Value, MediaStoreError> {
        let container = self
            .containers
            .get(container_name)
            .ok_or(MediaStoreError::ResourceNotFound)?;
        container
            .metric_policy
            .as_ref()
            .ok_or(MediaStoreError::PolicyNotFound)
    }
}
