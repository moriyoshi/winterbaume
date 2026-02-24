use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct ResourceGroupsState {
    pub groups: HashMap<String, ResourceGroup>,
    pub tag_sync_tasks: HashMap<String, TagSyncTask>,
    pub(crate) next_task_id: u64,
    pub account_settings: AccountSettings,
}

#[derive(Debug, Error)]
pub enum ResourceGroupsError {
    #[error("Group {name} already exists.")]
    GroupAlreadyExists { name: String },

    #[error("Group {name} not found.")]
    GroupNotFound { name: String },

    #[error("Resource {arn} not found.")]
    ResourceNotFound { arn: String },

    #[error("Tag sync task {task_arn} not found.")]
    TagSyncTaskNotFound { task_arn: String },
}

impl ResourceGroupsState {
    pub fn create_group(
        &mut self,
        name: &str,
        description: &str,
        query_type: &str,
        query: &str,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&ResourceGroup, ResourceGroupsError> {
        if self.groups.contains_key(name) {
            return Err(ResourceGroupsError::GroupAlreadyExists {
                name: name.to_string(),
            });
        }

        let arn = format!("arn:aws:resource-groups:{region}:{account_id}:group/{name}");

        let group = ResourceGroup {
            name: name.to_string(),
            arn,
            description: description.to_string(),
            resource_query_type: query_type.to_string(),
            resource_query_query: query.to_string(),
            tags,
            configuration: None,
            resource_arns: Vec::new(),
        };

        self.groups.insert(name.to_string(), group);
        Ok(self.groups.get(name).unwrap())
    }

    pub fn get_group(&self, name: &str) -> Result<&ResourceGroup, ResourceGroupsError> {
        self.groups
            .get(name)
            .ok_or_else(|| ResourceGroupsError::GroupNotFound {
                name: name.to_string(),
            })
    }

    pub fn delete_group(&mut self, name: &str) -> Result<ResourceGroup, ResourceGroupsError> {
        self.groups
            .remove(name)
            .ok_or_else(|| ResourceGroupsError::GroupNotFound {
                name: name.to_string(),
            })
    }

    pub fn list_groups(&self) -> Vec<&ResourceGroup> {
        self.groups.values().collect()
    }

    pub fn update_group(
        &mut self,
        name: &str,
        description: Option<&str>,
    ) -> Result<&ResourceGroup, ResourceGroupsError> {
        let group =
            self.groups
                .get_mut(name)
                .ok_or_else(|| ResourceGroupsError::GroupNotFound {
                    name: name.to_string(),
                })?;
        if let Some(desc) = description {
            group.description = desc.to_string();
        }
        Ok(group)
    }

    pub fn update_group_query(
        &mut self,
        name: &str,
        query_type: &str,
        query: &str,
    ) -> Result<&ResourceGroup, ResourceGroupsError> {
        let group =
            self.groups
                .get_mut(name)
                .ok_or_else(|| ResourceGroupsError::GroupNotFound {
                    name: name.to_string(),
                })?;
        group.resource_query_type = query_type.to_string();
        group.resource_query_query = query.to_string();
        Ok(group)
    }

    pub fn get_group_configuration(
        &self,
        name: &str,
    ) -> Result<&ResourceGroup, ResourceGroupsError> {
        self.get_group(name)
    }

    pub fn put_group_configuration(
        &mut self,
        name: &str,
        configuration: Option<Vec<GroupConfigurationEntry>>,
    ) -> Result<(), ResourceGroupsError> {
        let group =
            self.groups
                .get_mut(name)
                .ok_or_else(|| ResourceGroupsError::GroupNotFound {
                    name: name.to_string(),
                })?;
        group.configuration = configuration;
        Ok(())
    }

    pub fn get_tags(&self, arn: &str) -> Result<&ResourceGroup, ResourceGroupsError> {
        self.groups.values().find(|g| g.arn == arn).ok_or_else(|| {
            ResourceGroupsError::ResourceNotFound {
                arn: arn.to_string(),
            }
        })
    }

    pub fn tag_resource(
        &mut self,
        arn: &str,
        tags: &HashMap<String, String>,
    ) -> Result<&ResourceGroup, ResourceGroupsError> {
        let group = self
            .groups
            .values_mut()
            .find(|g| g.arn == arn)
            .ok_or_else(|| ResourceGroupsError::ResourceNotFound {
                arn: arn.to_string(),
            })?;
        for (k, v) in tags {
            group.tags.insert(k.clone(), v.clone());
        }
        Ok(group)
    }

    pub fn untag_resource(
        &mut self,
        arn: &str,
        keys: &[String],
    ) -> Result<&ResourceGroup, ResourceGroupsError> {
        let group = self
            .groups
            .values_mut()
            .find(|g| g.arn == arn)
            .ok_or_else(|| ResourceGroupsError::ResourceNotFound {
                arn: arn.to_string(),
            })?;
        for key in keys {
            group.tags.remove(key);
        }
        Ok(group)
    }

    pub fn start_tag_sync_task(
        &mut self,
        group_name: &str,
        tag_key: &str,
        tag_value: &str,
        role_arn: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&TagSyncTask, ResourceGroupsError> {
        let group =
            self.groups
                .get(group_name)
                .ok_or_else(|| ResourceGroupsError::GroupNotFound {
                    name: group_name.to_string(),
                })?;
        let group_arn = group.arn.clone();

        self.next_task_id += 1;
        let task_id = format!("task-{:08x}", self.next_task_id);
        let task_arn =
            format!("arn:aws:resource-groups:{region}:{account_id}:tag-sync-task/{task_id}");

        let task = TagSyncTask {
            task_arn: task_arn.clone(),
            group_name: group_name.to_string(),
            group_arn,
            tag_key: tag_key.to_string(),
            tag_value: tag_value.to_string(),
            role_arn: role_arn.to_string(),
            status: "ACTIVE".to_string(),
            created_at: 1700000000.0,
        };

        self.tag_sync_tasks.insert(task_arn.clone(), task);
        Ok(self.tag_sync_tasks.get(&task_arn).unwrap())
    }

    pub fn get_tag_sync_task(&self, task_arn: &str) -> Result<&TagSyncTask, ResourceGroupsError> {
        self.tag_sync_tasks
            .get(task_arn)
            .ok_or_else(|| ResourceGroupsError::TagSyncTaskNotFound {
                task_arn: task_arn.to_string(),
            })
    }

    pub fn list_tag_sync_tasks(&self, group_filter: Option<&str>) -> Vec<&TagSyncTask> {
        self.tag_sync_tasks
            .values()
            .filter(|t| {
                group_filter
                    .map(|f| t.group_name == f || t.group_arn == f)
                    .unwrap_or(true)
            })
            .collect()
    }

    pub fn cancel_tag_sync_task(&mut self, task_arn: &str) -> Result<(), ResourceGroupsError> {
        let task = self.tag_sync_tasks.get_mut(task_arn).ok_or_else(|| {
            ResourceGroupsError::TagSyncTaskNotFound {
                task_arn: task_arn.to_string(),
            }
        })?;
        task.status = "CANCELLED".to_string();
        Ok(())
    }

    pub fn get_account_settings(&self) -> &AccountSettings {
        &self.account_settings
    }

    pub fn update_account_settings(&mut self, desired_status: Option<&str>) -> &AccountSettings {
        if let Some(status) = desired_status {
            self.account_settings.group_lifecycle_events_desired_status = status.to_string();
        }
        &self.account_settings
    }

    pub fn get_group_query(&self, name: &str) -> Result<&ResourceGroup, ResourceGroupsError> {
        self.get_group(name)
    }

    pub fn group_resources(
        &mut self,
        name: &str,
        resource_arns: Vec<String>,
    ) -> Result<Vec<String>, ResourceGroupsError> {
        let group =
            self.groups
                .get_mut(name)
                .ok_or_else(|| ResourceGroupsError::GroupNotFound {
                    name: name.to_string(),
                })?;
        let mut succeeded = Vec::new();
        for arn in resource_arns {
            if !group.resource_arns.contains(&arn) {
                group.resource_arns.push(arn.clone());
            }
            succeeded.push(arn);
        }
        Ok(succeeded)
    }

    pub fn ungroup_resources(
        &mut self,
        name: &str,
        resource_arns: Vec<String>,
    ) -> Result<Vec<String>, ResourceGroupsError> {
        let group =
            self.groups
                .get_mut(name)
                .ok_or_else(|| ResourceGroupsError::GroupNotFound {
                    name: name.to_string(),
                })?;
        let mut succeeded = Vec::new();
        for arn in resource_arns {
            group.resource_arns.retain(|a| a != &arn);
            succeeded.push(arn);
        }
        Ok(succeeded)
    }

    pub fn list_group_resources(&self, name: &str) -> Result<Vec<String>, ResourceGroupsError> {
        let group = self.get_group(name)?;
        Ok(group.resource_arns.clone())
    }
}
