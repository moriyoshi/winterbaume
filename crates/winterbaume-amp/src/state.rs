use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct AmpState {
    pub workspaces: HashMap<String, Workspace>,
    /// Logging configurations keyed by workspace_id
    pub logging_configs: HashMap<String, LoggingConfiguration>,
    /// Rule groups namespaces keyed by (workspace_id, name)
    pub rule_groups_namespaces: HashMap<(String, String), RuleGroupsNamespace>,
}

#[derive(Debug, Error)]
pub enum AmpError {
    #[error("Workspace {workspace_id} not found")]
    WorkspaceNotFound { workspace_id: String },

    #[error("Logging configuration not found for workspace {workspace_id}")]
    LoggingConfigNotFound { workspace_id: String },

    #[error("Logging configuration already exists for workspace {workspace_id}")]
    LoggingConfigAlreadyExists { workspace_id: String },

    #[error("Rule groups namespace {name} not found")]
    RuleGroupsNamespaceNotFound { name: String },

    #[error("Rule groups namespace {name} already exists")]
    RuleGroupsNamespaceAlreadyExists { name: String },

    #[error("Resource {resource_arn} not found")]
    ResourceNotFound { resource_arn: String },
}

impl AmpState {
    pub fn create_workspace(
        &mut self,
        alias: Option<&str>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&Workspace, AmpError> {
        let workspace_id = format!("ws-{}", uuid::Uuid::new_v4().simple());
        let arn = format!("arn:aws:aps:{region}:{account_id}:workspace/{workspace_id}");
        let prometheus_endpoint =
            format!("https://aps-workspaces.{region}.amazonaws.com/workspaces/{workspace_id}/");

        let workspace = Workspace {
            workspace_id: workspace_id.clone(),
            arn,
            alias: alias.map(|s| s.to_string()),
            status: WorkspaceStatus {
                status_code: "ACTIVE".to_string(),
            },
            prometheus_endpoint,
            created_at: Utc::now(),
            tags,
        };

        self.workspaces.insert(workspace_id.clone(), workspace);
        Ok(self.workspaces.get(&workspace_id).unwrap())
    }

    pub fn describe_workspace(&self, workspace_id: &str) -> Result<&Workspace, AmpError> {
        self.workspaces
            .get(workspace_id)
            .ok_or_else(|| AmpError::WorkspaceNotFound {
                workspace_id: workspace_id.to_string(),
            })
    }

    pub fn delete_workspace(&mut self, workspace_id: &str) -> Result<(), AmpError> {
        self.workspaces
            .remove(workspace_id)
            .ok_or_else(|| AmpError::WorkspaceNotFound {
                workspace_id: workspace_id.to_string(),
            })?;
        Ok(())
    }

    pub fn list_workspaces(&self, alias: Option<&str>) -> Vec<WorkspaceSummary> {
        self.workspaces
            .values()
            .filter(|ws| {
                if let Some(filter_alias) = alias {
                    ws.alias.as_deref() == Some(filter_alias)
                } else {
                    true
                }
            })
            .map(|ws| WorkspaceSummary {
                workspace_id: ws.workspace_id.clone(),
                arn: ws.arn.clone(),
                alias: ws.alias.clone(),
                status: ws.status.clone(),
                created_at: ws.created_at,
                tags: ws.tags.clone(),
            })
            .collect()
    }

    pub fn update_workspace_alias(
        &mut self,
        workspace_id: &str,
        alias: Option<&str>,
    ) -> Result<(), AmpError> {
        let ws =
            self.workspaces
                .get_mut(workspace_id)
                .ok_or_else(|| AmpError::WorkspaceNotFound {
                    workspace_id: workspace_id.to_string(),
                })?;
        ws.alias = alias.map(|s| s.to_string());
        Ok(())
    }

    // --- Logging Configuration ---

    pub fn create_logging_configuration(
        &mut self,
        workspace_id: &str,
        log_group_arn: &str,
    ) -> Result<&LoggingConfiguration, AmpError> {
        if !self.workspaces.contains_key(workspace_id) {
            return Err(AmpError::WorkspaceNotFound {
                workspace_id: workspace_id.to_string(),
            });
        }
        if self.logging_configs.contains_key(workspace_id) {
            return Err(AmpError::LoggingConfigAlreadyExists {
                workspace_id: workspace_id.to_string(),
            });
        }

        let now = Utc::now();
        let config = LoggingConfiguration {
            workspace_id: workspace_id.to_string(),
            log_group_arn: log_group_arn.to_string(),
            status: LoggingConfigurationStatus {
                status_code: "ACTIVE".to_string(),
            },
            created_at: now,
            modified_at: now,
        };
        self.logging_configs
            .insert(workspace_id.to_string(), config);
        Ok(self.logging_configs.get(workspace_id).unwrap())
    }

    pub fn describe_logging_configuration(
        &self,
        workspace_id: &str,
    ) -> Result<Option<&LoggingConfiguration>, AmpError> {
        if !self.workspaces.contains_key(workspace_id) {
            return Err(AmpError::WorkspaceNotFound {
                workspace_id: workspace_id.to_string(),
            });
        }
        Ok(self.logging_configs.get(workspace_id))
    }

    pub fn update_logging_configuration(
        &mut self,
        workspace_id: &str,
        log_group_arn: &str,
    ) -> Result<&LoggingConfiguration, AmpError> {
        if !self.workspaces.contains_key(workspace_id) {
            return Err(AmpError::WorkspaceNotFound {
                workspace_id: workspace_id.to_string(),
            });
        }
        let config = self.logging_configs.get_mut(workspace_id).ok_or_else(|| {
            AmpError::LoggingConfigNotFound {
                workspace_id: workspace_id.to_string(),
            }
        })?;
        config.log_group_arn = log_group_arn.to_string();
        config.modified_at = Utc::now();
        Ok(self.logging_configs.get(workspace_id).unwrap())
    }

    pub fn delete_logging_configuration(&mut self, workspace_id: &str) -> Result<(), AmpError> {
        if !self.workspaces.contains_key(workspace_id) {
            return Err(AmpError::WorkspaceNotFound {
                workspace_id: workspace_id.to_string(),
            });
        }
        self.logging_configs.remove(workspace_id).ok_or_else(|| {
            AmpError::LoggingConfigNotFound {
                workspace_id: workspace_id.to_string(),
            }
        })?;
        Ok(())
    }

    // --- Rule Groups Namespaces ---

    pub fn create_rule_groups_namespace(
        &mut self,
        workspace_id: &str,
        name: &str,
        data: &str,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&RuleGroupsNamespace, AmpError> {
        if !self.workspaces.contains_key(workspace_id) {
            return Err(AmpError::WorkspaceNotFound {
                workspace_id: workspace_id.to_string(),
            });
        }
        let key = (workspace_id.to_string(), name.to_string());
        if self.rule_groups_namespaces.contains_key(&key) {
            return Err(AmpError::RuleGroupsNamespaceAlreadyExists {
                name: name.to_string(),
            });
        }

        let arn =
            format!("arn:aws:aps:{region}:{account_id}:rulegroupsnamespace/{workspace_id}/{name}");
        let now = Utc::now();
        let ns = RuleGroupsNamespace {
            name: name.to_string(),
            arn,
            workspace_id: workspace_id.to_string(),
            data: data.to_string(),
            status: RuleGroupsNamespaceStatus {
                status_code: "ACTIVE".to_string(),
            },
            created_at: now,
            modified_at: now,
            tags,
        };
        self.rule_groups_namespaces.insert(key.clone(), ns);
        Ok(self.rule_groups_namespaces.get(&key).unwrap())
    }

    pub fn describe_rule_groups_namespace(
        &self,
        workspace_id: &str,
        name: &str,
    ) -> Result<&RuleGroupsNamespace, AmpError> {
        if !self.workspaces.contains_key(workspace_id) {
            return Err(AmpError::WorkspaceNotFound {
                workspace_id: workspace_id.to_string(),
            });
        }
        let key = (workspace_id.to_string(), name.to_string());
        self.rule_groups_namespaces
            .get(&key)
            .ok_or_else(|| AmpError::RuleGroupsNamespaceNotFound {
                name: name.to_string(),
            })
    }

    pub fn put_rule_groups_namespace(
        &mut self,
        workspace_id: &str,
        name: &str,
        data: &str,
    ) -> Result<&RuleGroupsNamespace, AmpError> {
        if !self.workspaces.contains_key(workspace_id) {
            return Err(AmpError::WorkspaceNotFound {
                workspace_id: workspace_id.to_string(),
            });
        }
        let key = (workspace_id.to_string(), name.to_string());
        let ns = self.rule_groups_namespaces.get_mut(&key).ok_or_else(|| {
            AmpError::RuleGroupsNamespaceNotFound {
                name: name.to_string(),
            }
        })?;
        ns.data = data.to_string();
        ns.modified_at = Utc::now();
        Ok(self.rule_groups_namespaces.get(&key).unwrap())
    }

    pub fn delete_rule_groups_namespace(
        &mut self,
        workspace_id: &str,
        name: &str,
    ) -> Result<(), AmpError> {
        if !self.workspaces.contains_key(workspace_id) {
            return Err(AmpError::WorkspaceNotFound {
                workspace_id: workspace_id.to_string(),
            });
        }
        let key = (workspace_id.to_string(), name.to_string());
        self.rule_groups_namespaces.remove(&key).ok_or_else(|| {
            AmpError::RuleGroupsNamespaceNotFound {
                name: name.to_string(),
            }
        })?;
        Ok(())
    }

    pub fn list_rule_groups_namespaces(
        &self,
        workspace_id: &str,
        name_filter: Option<&str>,
    ) -> Result<Vec<&RuleGroupsNamespace>, AmpError> {
        if !self.workspaces.contains_key(workspace_id) {
            return Err(AmpError::WorkspaceNotFound {
                workspace_id: workspace_id.to_string(),
            });
        }
        Ok(self
            .rule_groups_namespaces
            .values()
            .filter(|ns| ns.workspace_id == workspace_id)
            .filter(|ns| {
                if let Some(prefix) = name_filter {
                    ns.name.starts_with(prefix)
                } else {
                    true
                }
            })
            .collect())
    }

    // --- Tags ---

    pub fn find_resource_tags(
        &self,
        resource_arn: &str,
    ) -> Result<&HashMap<String, String>, AmpError> {
        // Check workspaces
        for ws in self.workspaces.values() {
            if ws.arn == resource_arn {
                return Ok(&ws.tags);
            }
        }
        // Check rule groups namespaces
        for ns in self.rule_groups_namespaces.values() {
            if ns.arn == resource_arn {
                return Ok(&ns.tags);
            }
        }
        Err(AmpError::ResourceNotFound {
            resource_arn: resource_arn.to_string(),
        })
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), AmpError> {
        // Check workspaces
        for ws in self.workspaces.values_mut() {
            if ws.arn == resource_arn {
                ws.tags.extend(tags);
                return Ok(());
            }
        }
        // Check rule groups namespaces
        for ns in self.rule_groups_namespaces.values_mut() {
            if ns.arn == resource_arn {
                ns.tags.extend(tags);
                return Ok(());
            }
        }
        Err(AmpError::ResourceNotFound {
            resource_arn: resource_arn.to_string(),
        })
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), AmpError> {
        // Check workspaces
        for ws in self.workspaces.values_mut() {
            if ws.arn == resource_arn {
                for key in tag_keys {
                    ws.tags.remove(key);
                }
                return Ok(());
            }
        }
        // Check rule groups namespaces
        for ns in self.rule_groups_namespaces.values_mut() {
            if ns.arn == resource_arn {
                for key in tag_keys {
                    ns.tags.remove(key);
                }
                return Ok(());
            }
        }
        Err(AmpError::ResourceNotFound {
            resource_arn: resource_arn.to_string(),
        })
    }
}
