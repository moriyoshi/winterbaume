use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct ConnectState {
    pub instances: HashMap<String, ConnectInstance>,
    /// Key: (instance_id, data_set_id)
    pub analytics_data_associations: HashMap<(String, String), AnalyticsDataAssociation>,
    /// Key: resource ARN -> tags
    pub resource_tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Error)]
pub enum ConnectError {
    #[error("Instance {instance_id} not found")]
    InstanceNotFound { instance_id: String },

    #[error("An instance with alias {alias} already exists in the account")]
    InstanceAliasConflict { alias: String },

    #[error("Analytics data set {data_set_id} is already associated with instance {instance_id}")]
    AnalyticsDataSetAlreadyAssociated {
        data_set_id: String,
        instance_id: String,
    },

    #[error("Analytics data set {data_set_id} is not associated with instance {instance_id}")]
    AnalyticsDataSetNotAssociated {
        data_set_id: String,
        instance_id: String,
    },
}

impl ConnectState {
    pub fn create_instance(
        &mut self,
        instance_id: &str,
        identity_management_type: &str,
        instance_alias: Option<&str>,
        inbound_calls_enabled: bool,
        outbound_calls_enabled: bool,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&ConnectInstance, ConnectError> {
        // Check for duplicate alias
        if let Some(alias) = instance_alias {
            let alias_exists = self
                .instances
                .values()
                .any(|inst| inst.instance_alias.as_deref() == Some(alias));
            if alias_exists {
                return Err(ConnectError::InstanceAliasConflict {
                    alias: alias.to_string(),
                });
            }
        }

        let arn = format!("arn:aws:connect:{region}:{account_id}:instance/{instance_id}");

        let instance = ConnectInstance {
            id: instance_id.to_string(),
            arn,
            identity_management_type: identity_management_type.to_string(),
            instance_alias: instance_alias.map(|s| s.to_string()),
            instance_status: "ACTIVE".to_string(),
            created_time: Utc::now(),
            inbound_calls_enabled,
            outbound_calls_enabled,
            tags,
        };

        self.instances.insert(instance_id.to_string(), instance);
        Ok(self.instances.get(instance_id).unwrap())
    }

    pub fn describe_instance(&self, instance_id: &str) -> Result<&ConnectInstance, ConnectError> {
        self.instances
            .get(instance_id)
            .ok_or_else(|| ConnectError::InstanceNotFound {
                instance_id: instance_id.to_string(),
            })
    }

    pub fn delete_instance(&mut self, instance_id: &str) -> Result<(), ConnectError> {
        if self.instances.remove(instance_id).is_none() {
            return Err(ConnectError::InstanceNotFound {
                instance_id: instance_id.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_instances(&self) -> Vec<&ConnectInstance> {
        self.instances.values().collect()
    }

    // --- Analytics Data Set Association operations ---

    pub fn associate_analytics_data_set(
        &mut self,
        instance_id: &str,
        data_set_id: &str,
        target_account_id: Option<&str>,
    ) -> Result<&AnalyticsDataAssociation, ConnectError> {
        if !self.instances.contains_key(instance_id) {
            return Err(ConnectError::InstanceNotFound {
                instance_id: instance_id.to_string(),
            });
        }

        let key = (instance_id.to_string(), data_set_id.to_string());
        if self.analytics_data_associations.contains_key(&key) {
            return Err(ConnectError::AnalyticsDataSetAlreadyAssociated {
                data_set_id: data_set_id.to_string(),
                instance_id: instance_id.to_string(),
            });
        }

        let resource_share_id = uuid::Uuid::new_v4().to_string();
        let resource_share_arn =
            format!("arn:aws:ram:us-east-1:123456789012:resource-share/{resource_share_id}");

        let assoc = AnalyticsDataAssociation {
            instance_id: instance_id.to_string(),
            data_set_id: data_set_id.to_string(),
            target_account_id: target_account_id.map(|s| s.to_string()),
            resource_share_id,
            resource_share_arn,
        };

        self.analytics_data_associations.insert(key.clone(), assoc);
        Ok(self.analytics_data_associations.get(&key).unwrap())
    }

    pub fn disassociate_analytics_data_set(
        &mut self,
        instance_id: &str,
        data_set_id: &str,
    ) -> Result<(), ConnectError> {
        if !self.instances.contains_key(instance_id) {
            return Err(ConnectError::InstanceNotFound {
                instance_id: instance_id.to_string(),
            });
        }

        let key = (instance_id.to_string(), data_set_id.to_string());
        if self.analytics_data_associations.remove(&key).is_none() {
            return Err(ConnectError::AnalyticsDataSetNotAssociated {
                data_set_id: data_set_id.to_string(),
                instance_id: instance_id.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_analytics_data_associations(
        &self,
        instance_id: &str,
    ) -> Result<Vec<&AnalyticsDataAssociation>, ConnectError> {
        if !self.instances.contains_key(instance_id) {
            return Err(ConnectError::InstanceNotFound {
                instance_id: instance_id.to_string(),
            });
        }

        let results: Vec<&AnalyticsDataAssociation> = self
            .analytics_data_associations
            .values()
            .filter(|a| a.instance_id == instance_id)
            .collect();
        Ok(results)
    }

    // --- Tag operations ---

    pub fn tag_resource(&mut self, arn: &str, tags: HashMap<String, String>) {
        let entry = self.resource_tags.entry(arn.to_string()).or_default();
        entry.extend(tags);
    }

    pub fn untag_resource(&mut self, arn: &str, tag_keys: &[String]) {
        if let Some(tags) = self.resource_tags.get_mut(arn) {
            for key in tag_keys {
                tags.remove(key);
            }
        }
    }

    pub fn list_tags_for_resource(&self, arn: &str) -> HashMap<String, String> {
        // First check if this is an instance ARN and return instance tags
        for inst in self.instances.values() {
            if inst.arn == arn {
                let mut tags = inst.tags.clone();
                // Merge any additional tags added via TagResource
                if let Some(extra) = self.resource_tags.get(arn) {
                    tags.extend(extra.clone());
                }
                return tags;
            }
        }
        self.resource_tags.get(arn).cloned().unwrap_or_default()
    }
}
