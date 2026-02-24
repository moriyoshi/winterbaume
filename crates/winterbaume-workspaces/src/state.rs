use std::collections::HashMap;

use crate::types::*;

/// In-memory state for the WorkSpaces service.
#[derive(Debug, Default)]
pub struct WorkSpacesState {
    /// Workspaces keyed by workspace ID.
    pub workspaces: HashMap<String, Workspace>,
    /// Directories keyed by directory ID.
    pub directories: HashMap<String, WorkspaceDirectory>,
    /// Tags keyed by resource ARN.
    pub tags: HashMap<String, Vec<Tag>>,
    /// Workspace images keyed by image ID.
    pub images: HashMap<String, WorkspaceImage>,
    /// Client properties keyed by resource ID.
    pub client_properties: HashMap<String, ClientProperties>,
    /// Self-service permissions keyed by directory ID.
    pub selfservice_permissions: HashMap<String, SelfservicePermissions>,
    /// Workspace creation properties keyed by directory ID.
    pub workspace_creation_properties: HashMap<String, WorkspaceCreationProperties>,
    /// Image permissions: image_id -> Vec<shared_account_id>
    pub image_permissions: HashMap<String, Vec<String>>,
    /// IP groups keyed by group ID.
    pub ip_groups: HashMap<String, IpGroup>,
    /// Connection aliases keyed by alias ID.
    pub connection_aliases: HashMap<String, ConnectionAlias>,
    /// Connection alias permissions: alias_id -> Vec<ConnectionAliasPermission>
    pub connection_alias_permissions: HashMap<String, Vec<ConnectionAliasPermission>>,
    /// Workspace bundles keyed by bundle ID.
    pub bundles: HashMap<String, WorkspaceBundle>,
    /// WorkSpaces pools keyed by pool ID.
    pub pools: HashMap<String, WorkspacesPool>,
}

/// Error type for WorkSpaces operations.
#[derive(Debug, thiserror::Error)]
pub enum WorkSpacesError {
    #[error("The specified WorkSpace '{0}' was not found.")]
    WorkspaceNotFound(String),

    #[error("The specified image '{0}' was not found.")]
    ImageNotFound(String),

    #[error("The specified directory '{0}' was not found.")]
    DirectoryNotFound(String),

    #[error("The directory '{0}' is already registered.")]
    DirectoryAlreadyRegistered(String),

    #[error("The specified IP group '{0}' was not found.")]
    IpGroupNotFound(String),

    #[error("The specified connection alias '{0}' was not found.")]
    ConnectionAliasNotFound(String),

    #[error("The specified bundle '{0}' was not found.")]
    BundleNotFound(String),

    #[error("The specified pool '{0}' was not found.")]
    PoolNotFound(String),
}

impl WorkSpacesState {
    /// Create workspaces from a list of requests.
    /// Returns (successful_workspaces, failed_requests).
    pub fn create_workspaces(
        &mut self,
        requests: &[WorkspaceRequest],
        account_id: &str,
        region: &str,
    ) -> (Vec<Workspace>, Vec<FailedCreateWorkspaceRequest>) {
        let mut pending = Vec::new();
        let failed = Vec::new();

        for req in requests {
            let workspace_id = format!(
                "ws-{}",
                &uuid::Uuid::new_v4().to_string().replace('-', "")[..25]
            );
            let props = req.workspace_properties.as_ref();

            let workspace = Workspace {
                workspace_id: workspace_id.clone(),
                directory_id: req.directory_id.clone(),
                user_name: req.user_name.clone(),
                bundle_id: req.bundle_id.clone(),
                state: "AVAILABLE".to_string(),
                ip_address: "10.0.0.1".to_string(),
                computer_name: format!("WSAMZN-{}", &workspace_id[3..10]).to_uppercase(),
                subnet_id: format!(
                    "subnet-{}",
                    &uuid::Uuid::new_v4().to_string().replace('-', "")[..8]
                ),
                root_volume_size_gib: props.and_then(|p| p.root_volume_size_gib).unwrap_or(80),
                user_volume_size_gib: props.and_then(|p| p.user_volume_size_gib).unwrap_or(50),
                volume_encryption_key: req.volume_encryption_key.clone(),
                user_volume_encryption_enabled: req.user_volume_encryption_enabled.unwrap_or(false),
                root_volume_encryption_enabled: req.root_volume_encryption_enabled.unwrap_or(false),
                running_mode: props
                    .and_then(|p| p.running_mode.clone())
                    .unwrap_or_else(|| "AUTO_STOP".to_string()),
                running_mode_auto_stop_timeout_in_minutes: props
                    .and_then(|p| p.running_mode_auto_stop_timeout_in_minutes)
                    .unwrap_or(60),
            };

            // Ensure the directory exists (auto-create a stub directory if not present)
            if !self.directories.contains_key(&req.directory_id) {
                let dir = WorkspaceDirectory {
                    directory_id: req.directory_id.clone(),
                    directory_name: format!("corp.{}.com", region),
                    directory_type: "SIMPLE_AD".to_string(),
                    alias: req.directory_id.clone(),
                    state: "REGISTERED".to_string(),
                    registration_code: format!("SLiad+{}", &uuid::Uuid::new_v4().to_string()[..8]),
                    workspace_security_group_id: format!(
                        "sg-{}",
                        &uuid::Uuid::new_v4().to_string().replace('-', "")[..8]
                    ),
                    iam_role_id: format!("arn:aws:iam::{}:role/workspaces_DefaultRole", account_id),
                };
                self.directories.insert(req.directory_id.clone(), dir);
            }

            self.workspaces
                .insert(workspace_id.clone(), workspace.clone());
            pending.push(workspace);
        }

        (pending, failed)
    }

    /// Describe workspaces, optionally filtered by workspace IDs, directory ID, or user name.
    pub fn describe_workspaces(
        &self,
        workspace_ids: Option<&[String]>,
        directory_id: Option<&str>,
        user_name: Option<&str>,
    ) -> Vec<&Workspace> {
        self.workspaces
            .values()
            .filter(|ws| {
                if let Some(ids) = workspace_ids {
                    if !ids.contains(&ws.workspace_id) {
                        return false;
                    }
                }
                if let Some(dir_id) = directory_id {
                    if ws.directory_id != dir_id {
                        return false;
                    }
                }
                if let Some(user) = user_name {
                    if ws.user_name != user {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    /// Terminate workspaces by IDs.
    /// Returns a list of failed termination requests.
    pub fn terminate_workspaces(
        &mut self,
        requests: &[TerminateRequest],
    ) -> Vec<FailedTerminateWorkspaceRequest> {
        let mut failed = Vec::new();

        for req in requests {
            if self.workspaces.contains_key(&req.workspace_id) {
                if let Some(ws) = self.workspaces.get_mut(&req.workspace_id) {
                    ws.state = "TERMINATING".to_string();
                }
                // Remove from the map to simulate termination
                self.workspaces.remove(&req.workspace_id);
            } else {
                failed.push(FailedTerminateWorkspaceRequest {
                    workspace_id: req.workspace_id.clone(),
                    error_code: "ResourceNotFoundException".to_string(),
                    error_message: format!(
                        "The specified WorkSpace '{}' was not found.",
                        req.workspace_id
                    ),
                });
            }
        }

        failed
    }

    /// Describe workspace directories.
    pub fn describe_workspace_directories(
        &self,
        directory_ids: Option<&[String]>,
    ) -> Vec<&WorkspaceDirectory> {
        self.directories
            .values()
            .filter(|dir| {
                if let Some(ids) = directory_ids {
                    if !ids.contains(&dir.directory_id) {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    /// Create tags on a resource.
    pub fn create_tags(&mut self, resource_id: &str, tags: &[Tag]) {
        let entry = self.tags.entry(resource_id.to_string()).or_default();
        for tag in tags {
            // Replace existing tag with same key
            entry.retain(|t| t.key != tag.key);
            entry.push(tag.clone());
        }
    }

    /// Describe tags for a resource.
    pub fn describe_tags(&self, resource_id: &str) -> Vec<&Tag> {
        self.tags
            .get(resource_id)
            .map(|tags| tags.iter().collect())
            .unwrap_or_default()
    }

    /// Create a workspace image from a workspace.
    pub fn create_workspace_image(
        &mut self,
        name: &str,
        description: &str,
        workspace_id: &str,
        account_id: &str,
        tags: Option<&[Tag]>,
    ) -> Result<WorkspaceImage, WorkSpacesError> {
        if !self.workspaces.contains_key(workspace_id) {
            return Err(WorkSpacesError::WorkspaceNotFound(workspace_id.to_string()));
        }

        let image_id = format!(
            "wsi-{}",
            &uuid::Uuid::new_v4().to_string().replace('-', "")[..25]
        );

        let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

        let image = WorkspaceImage {
            image_id: image_id.clone(),
            name: name.to_string(),
            description: description.to_string(),
            state: "AVAILABLE".to_string(),
            operating_system_type: Some("WINDOWS".to_string()),
            owner_account_id: account_id.to_string(),
            required_tenancy: "DEFAULT".to_string(),
            created: now,
        };

        self.images.insert(image_id.clone(), image.clone());

        if let Some(tags) = tags {
            self.create_tags(&image_id, tags);
        }

        Ok(image)
    }

    /// Describe workspace images.
    pub fn describe_workspace_images(&self, image_ids: Option<&[String]>) -> Vec<&WorkspaceImage> {
        self.images
            .values()
            .filter(|img| {
                if let Some(ids) = image_ids {
                    if !ids.contains(&img.image_id) {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    /// Describe workspace image permissions.
    pub fn describe_workspace_image_permissions(
        &self,
        image_id: &str,
    ) -> Result<Vec<String>, WorkSpacesError> {
        if !self.images.contains_key(image_id) {
            return Err(WorkSpacesError::ImageNotFound(image_id.to_string()));
        }
        Ok(self
            .image_permissions
            .get(image_id)
            .cloned()
            .unwrap_or_default())
    }

    /// Update workspace image permission.
    pub fn update_workspace_image_permission(
        &mut self,
        image_id: &str,
        shared_account_id: &str,
        allow_copy_image: bool,
    ) -> Result<(), WorkSpacesError> {
        if !self.images.contains_key(image_id) {
            return Err(WorkSpacesError::ImageNotFound(image_id.to_string()));
        }
        let perms = self
            .image_permissions
            .entry(image_id.to_string())
            .or_default();
        if allow_copy_image {
            if !perms.contains(&shared_account_id.to_string()) {
                perms.push(shared_account_id.to_string());
            }
        } else {
            perms.retain(|id| id != shared_account_id);
        }
        Ok(())
    }

    /// Describe client properties.
    pub fn describe_client_properties(
        &self,
        resource_ids: &[String],
    ) -> Vec<(String, ClientProperties)> {
        resource_ids
            .iter()
            .filter_map(|id| {
                self.client_properties
                    .get(id)
                    .map(|cp| (id.clone(), cp.clone()))
            })
            .collect()
    }

    /// Modify client properties for a resource.
    pub fn modify_client_properties(&mut self, resource_id: &str, props: &ClientProperties) {
        let entry = self
            .client_properties
            .entry(resource_id.to_string())
            .or_insert_with(|| ClientProperties {
                reconnect_enabled: None,
                log_upload_enabled: None,
            });
        if let Some(ref v) = props.reconnect_enabled {
            entry.reconnect_enabled = Some(v.clone());
        }
        if let Some(ref v) = props.log_upload_enabled {
            entry.log_upload_enabled = Some(v.clone());
        }
    }

    /// Modify self-service permissions for a directory.
    pub fn modify_selfservice_permissions(
        &mut self,
        directory_id: &str,
        perms: &SelfservicePermissions,
    ) -> Result<(), WorkSpacesError> {
        if !self.directories.contains_key(directory_id) {
            return Err(WorkSpacesError::DirectoryNotFound(directory_id.to_string()));
        }
        self.selfservice_permissions
            .insert(directory_id.to_string(), perms.clone());
        Ok(())
    }

    /// Modify workspace creation properties for a directory.
    pub fn modify_workspace_creation_properties(
        &mut self,
        directory_id: &str,
        props: &WorkspaceCreationProperties,
    ) -> Result<(), WorkSpacesError> {
        if !self.directories.contains_key(directory_id) {
            return Err(WorkSpacesError::DirectoryNotFound(directory_id.to_string()));
        }
        self.workspace_creation_properties
            .insert(directory_id.to_string(), props.clone());
        Ok(())
    }

    /// Register a workspace directory.
    pub fn register_workspace_directory(
        &mut self,
        directory_id: &str,
        account_id: &str,
        region: &str,
    ) -> Result<WorkspaceDirectory, WorkSpacesError> {
        if self.directories.contains_key(directory_id) {
            return Err(WorkSpacesError::DirectoryAlreadyRegistered(
                directory_id.to_string(),
            ));
        }

        let dir = WorkspaceDirectory {
            directory_id: directory_id.to_string(),
            directory_name: format!("corp.{}.com", region),
            directory_type: "SIMPLE_AD".to_string(),
            alias: directory_id.to_string(),
            state: "REGISTERED".to_string(),
            registration_code: format!("SLiad+{}", &uuid::Uuid::new_v4().to_string()[..8]),
            workspace_security_group_id: format!(
                "sg-{}",
                &uuid::Uuid::new_v4().to_string().replace('-', "")[..8]
            ),
            iam_role_id: format!("arn:aws:iam::{}:role/workspaces_DefaultRole", account_id),
        };

        self.directories
            .insert(directory_id.to_string(), dir.clone());
        Ok(dir)
    }

    /// Deregister a workspace directory.
    pub fn deregister_workspace_directory(
        &mut self,
        directory_id: &str,
    ) -> Result<(), WorkSpacesError> {
        if self.directories.remove(directory_id).is_none() {
            return Err(WorkSpacesError::DirectoryNotFound(directory_id.to_string()));
        }
        Ok(())
    }

    /// Delete tags from a resource.
    pub fn delete_tags(&mut self, resource_id: &str, tag_keys: &[String]) {
        if let Some(tags) = self.tags.get_mut(resource_id) {
            tags.retain(|t| !tag_keys.contains(&t.key));
        }
    }

    /// Start workspaces.
    pub fn start_workspaces(
        &mut self,
        requests: &[StartRequest],
    ) -> Vec<crate::types::FailedTerminateWorkspaceRequest> {
        let mut failed = Vec::new();
        for req in requests {
            if let Some(ws) = self.workspaces.get_mut(&req.workspace_id) {
                ws.state = "AVAILABLE".to_string();
            } else {
                failed.push(crate::types::FailedTerminateWorkspaceRequest {
                    workspace_id: req.workspace_id.clone(),
                    error_code: "ResourceNotFoundException".to_string(),
                    error_message: format!(
                        "The specified WorkSpace '{}' was not found.",
                        req.workspace_id
                    ),
                });
            }
        }
        failed
    }

    /// Stop workspaces.
    pub fn stop_workspaces(
        &mut self,
        requests: &[StopRequest],
    ) -> Vec<crate::types::FailedTerminateWorkspaceRequest> {
        let mut failed = Vec::new();
        for req in requests {
            if let Some(ws) = self.workspaces.get_mut(&req.workspace_id) {
                ws.state = "STOPPED".to_string();
            } else {
                failed.push(crate::types::FailedTerminateWorkspaceRequest {
                    workspace_id: req.workspace_id.clone(),
                    error_code: "ResourceNotFoundException".to_string(),
                    error_message: format!(
                        "The specified WorkSpace '{}' was not found.",
                        req.workspace_id
                    ),
                });
            }
        }
        failed
    }

    /// Reboot workspaces.
    pub fn reboot_workspaces(
        &mut self,
        requests: &[RebootRequest],
    ) -> Vec<crate::types::FailedTerminateWorkspaceRequest> {
        let mut failed = Vec::new();
        for req in requests {
            if !self.workspaces.contains_key(&req.workspace_id) {
                failed.push(crate::types::FailedTerminateWorkspaceRequest {
                    workspace_id: req.workspace_id.clone(),
                    error_code: "ResourceNotFoundException".to_string(),
                    error_message: format!(
                        "The specified WorkSpace '{}' was not found.",
                        req.workspace_id
                    ),
                });
            }
        }
        failed
    }

    /// Rebuild workspaces.
    pub fn rebuild_workspaces(
        &mut self,
        requests: &[RebuildRequest],
    ) -> Vec<crate::types::FailedTerminateWorkspaceRequest> {
        let mut failed = Vec::new();
        for req in requests {
            if !self.workspaces.contains_key(&req.workspace_id) {
                failed.push(crate::types::FailedTerminateWorkspaceRequest {
                    workspace_id: req.workspace_id.clone(),
                    error_code: "ResourceNotFoundException".to_string(),
                    error_message: format!(
                        "The specified WorkSpace '{}' was not found.",
                        req.workspace_id
                    ),
                });
            }
        }
        failed
    }

    /// Restore workspace.
    pub fn restore_workspace(&mut self, workspace_id: &str) -> Result<(), WorkSpacesError> {
        if !self.workspaces.contains_key(workspace_id) {
            return Err(WorkSpacesError::WorkspaceNotFound(workspace_id.to_string()));
        }
        Ok(())
    }

    /// Modify workspace properties.
    pub fn modify_workspace_properties(
        &mut self,
        workspace_id: &str,
        running_mode: Option<&str>,
        root_volume_size_gib: Option<i32>,
        user_volume_size_gib: Option<i32>,
        compute_type_name: Option<&str>,
        running_mode_auto_stop_timeout_in_minutes: Option<i32>,
    ) -> Result<(), WorkSpacesError> {
        let ws = self
            .workspaces
            .get_mut(workspace_id)
            .ok_or_else(|| WorkSpacesError::WorkspaceNotFound(workspace_id.to_string()))?;
        if let Some(v) = running_mode {
            ws.running_mode = v.to_string();
        }
        if let Some(v) = root_volume_size_gib {
            ws.root_volume_size_gib = v;
        }
        if let Some(v) = user_volume_size_gib {
            ws.user_volume_size_gib = v;
        }
        let _ = compute_type_name; // stored in workspace properties
        if let Some(v) = running_mode_auto_stop_timeout_in_minutes {
            ws.running_mode_auto_stop_timeout_in_minutes = v;
        }
        Ok(())
    }

    /// Modify workspace state.
    pub fn modify_workspace_state(
        &mut self,
        workspace_id: &str,
        workspace_state: &str,
    ) -> Result<(), WorkSpacesError> {
        let ws = self
            .workspaces
            .get_mut(workspace_id)
            .ok_or_else(|| WorkSpacesError::WorkspaceNotFound(workspace_id.to_string()))?;
        ws.state = workspace_state.to_string();
        Ok(())
    }

    /// Describe workspaces connection status.
    pub fn describe_workspaces_connection_status(
        &self,
        workspace_ids: Option<&[String]>,
    ) -> Vec<(&str, &str)> {
        self.workspaces
            .values()
            .filter(|ws| {
                if let Some(ids) = workspace_ids {
                    ids.contains(&ws.workspace_id)
                } else {
                    true
                }
            })
            .map(|ws| (ws.workspace_id.as_str(), ws.state.as_str()))
            .collect()
    }

    // --- IP Groups ---

    /// Create an IP group.
    pub fn create_ip_group(
        &mut self,
        group_name: &str,
        group_desc: Option<&str>,
        user_rules: Vec<IpRule>,
        account_id: &str,
    ) -> IpGroup {
        let group_id = format!(
            "wsipg-{}",
            &uuid::Uuid::new_v4().to_string().replace('-', "")[..25]
        );
        let _ = account_id;
        let group = IpGroup {
            group_id: group_id.clone(),
            group_name: group_name.to_string(),
            group_desc: group_desc.map(|s| s.to_string()),
            user_rules,
        };
        self.ip_groups.insert(group_id, group.clone());
        group
    }

    /// Describe IP groups.
    pub fn describe_ip_groups(&self, group_ids: Option<&[String]>) -> Vec<&IpGroup> {
        self.ip_groups
            .values()
            .filter(|g| {
                if let Some(ids) = group_ids {
                    ids.contains(&g.group_id)
                } else {
                    true
                }
            })
            .collect()
    }

    /// Delete an IP group.
    pub fn delete_ip_group(&mut self, group_id: &str) -> Result<(), WorkSpacesError> {
        if self.ip_groups.remove(group_id).is_none() {
            return Err(WorkSpacesError::IpGroupNotFound(group_id.to_string()));
        }
        Ok(())
    }

    /// Authorize IP rules to an IP group.
    pub fn authorize_ip_rules(
        &mut self,
        group_id: &str,
        user_rules: Vec<IpRule>,
    ) -> Result<(), WorkSpacesError> {
        let group = self
            .ip_groups
            .get_mut(group_id)
            .ok_or_else(|| WorkSpacesError::IpGroupNotFound(group_id.to_string()))?;
        for rule in user_rules {
            // Avoid duplicates
            if !group.user_rules.iter().any(|r| r.ip_rule == rule.ip_rule) {
                group.user_rules.push(rule);
            }
        }
        Ok(())
    }

    /// Revoke IP rules from an IP group.
    pub fn revoke_ip_rules(
        &mut self,
        group_id: &str,
        user_rules: &[String],
    ) -> Result<(), WorkSpacesError> {
        let group = self
            .ip_groups
            .get_mut(group_id)
            .ok_or_else(|| WorkSpacesError::IpGroupNotFound(group_id.to_string()))?;
        group
            .user_rules
            .retain(|r| !user_rules.contains(&r.ip_rule));
        Ok(())
    }

    /// Update rules of an IP group (replace all rules).
    pub fn update_rules_of_ip_group(
        &mut self,
        group_id: &str,
        user_rules: Vec<IpRule>,
    ) -> Result<(), WorkSpacesError> {
        let group = self
            .ip_groups
            .get_mut(group_id)
            .ok_or_else(|| WorkSpacesError::IpGroupNotFound(group_id.to_string()))?;
        group.user_rules = user_rules;
        Ok(())
    }

    /// Associate IP groups with a directory.
    pub fn associate_ip_groups(
        &mut self,
        directory_id: &str,
        group_ids: &[String],
    ) -> Result<(), WorkSpacesError> {
        if !self.directories.contains_key(directory_id) {
            return Err(WorkSpacesError::DirectoryNotFound(directory_id.to_string()));
        }
        for group_id in group_ids {
            if !self.ip_groups.contains_key(group_id.as_str()) {
                return Err(WorkSpacesError::IpGroupNotFound(group_id.to_string()));
            }
        }
        Ok(())
    }

    /// Disassociate IP groups from a directory.
    pub fn disassociate_ip_groups(
        &mut self,
        directory_id: &str,
        group_ids: &[String],
    ) -> Result<(), WorkSpacesError> {
        if !self.directories.contains_key(directory_id) {
            return Err(WorkSpacesError::DirectoryNotFound(directory_id.to_string()));
        }
        let _ = group_ids;
        Ok(())
    }

    // --- Connection Aliases ---

    /// Create a connection alias.
    pub fn create_connection_alias(
        &mut self,
        connection_string: &str,
        account_id: &str,
    ) -> ConnectionAlias {
        let alias_id = format!(
            "wsca-{}",
            &uuid::Uuid::new_v4().to_string().replace('-', "")[..25]
        );
        let alias = ConnectionAlias {
            alias_id: alias_id.clone(),
            connection_string: connection_string.to_string(),
            owner_account_id: account_id.to_string(),
            state: "CREATED".to_string(),
            associations: Vec::new(),
        };
        self.connection_aliases.insert(alias_id, alias.clone());
        alias
    }

    /// Describe connection aliases.
    pub fn describe_connection_aliases(
        &self,
        alias_ids: Option<&[String]>,
        resource_id: Option<&str>,
    ) -> Vec<&ConnectionAlias> {
        self.connection_aliases
            .values()
            .filter(|a| {
                if let Some(ids) = alias_ids {
                    if !ids.contains(&a.alias_id) {
                        return false;
                    }
                }
                if let Some(rid) = resource_id {
                    if !a
                        .associations
                        .iter()
                        .any(|assoc| assoc.resource_id.as_deref() == Some(rid))
                    {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    /// Delete a connection alias.
    pub fn delete_connection_alias(&mut self, alias_id: &str) -> Result<(), WorkSpacesError> {
        if self.connection_aliases.remove(alias_id).is_none() {
            return Err(WorkSpacesError::ConnectionAliasNotFound(
                alias_id.to_string(),
            ));
        }
        self.connection_alias_permissions.remove(alias_id);
        Ok(())
    }

    /// Associate a connection alias with a resource.
    pub fn associate_connection_alias(
        &mut self,
        alias_id: &str,
        resource_id: &str,
    ) -> Result<String, WorkSpacesError> {
        let alias = self
            .connection_aliases
            .get_mut(alias_id)
            .ok_or_else(|| WorkSpacesError::ConnectionAliasNotFound(alias_id.to_string()))?;
        let connection_identifier = uuid::Uuid::new_v4().to_string();
        alias.associations.push(ConnectionAliasAssociation {
            connection_identifier: connection_identifier.clone(),
            resource_id: Some(resource_id.to_string()),
            associated_account_id: None,
        });
        alias.state = "ASSOCIATED".to_string();
        Ok(connection_identifier)
    }

    /// Disassociate a connection alias.
    pub fn disassociate_connection_alias(&mut self, alias_id: &str) -> Result<(), WorkSpacesError> {
        let alias = self
            .connection_aliases
            .get_mut(alias_id)
            .ok_or_else(|| WorkSpacesError::ConnectionAliasNotFound(alias_id.to_string()))?;
        alias.associations.clear();
        alias.state = "CREATED".to_string();
        Ok(())
    }

    /// Describe connection alias permissions.
    pub fn describe_connection_alias_permissions(
        &self,
        alias_id: &str,
    ) -> Result<Vec<&ConnectionAliasPermission>, WorkSpacesError> {
        if !self.connection_aliases.contains_key(alias_id) {
            return Err(WorkSpacesError::ConnectionAliasNotFound(
                alias_id.to_string(),
            ));
        }
        Ok(self
            .connection_alias_permissions
            .get(alias_id)
            .map(|v| v.iter().collect())
            .unwrap_or_default())
    }

    /// Update connection alias permission.
    pub fn update_connection_alias_permission(
        &mut self,
        alias_id: &str,
        shared_account_id: &str,
        allow_association: bool,
    ) -> Result<(), WorkSpacesError> {
        if !self.connection_aliases.contains_key(alias_id) {
            return Err(WorkSpacesError::ConnectionAliasNotFound(
                alias_id.to_string(),
            ));
        }
        let perms = self
            .connection_alias_permissions
            .entry(alias_id.to_string())
            .or_default();
        if allow_association {
            if !perms
                .iter()
                .any(|p| p.shared_account_id == shared_account_id)
            {
                perms.push(ConnectionAliasPermission {
                    shared_account_id: shared_account_id.to_string(),
                    allow_association,
                });
            }
        } else {
            perms.retain(|p| p.shared_account_id != shared_account_id);
        }
        Ok(())
    }

    // --- Bundles ---

    /// Describe workspace bundles.
    #[allow(clippy::too_many_arguments)]
    pub fn describe_workspace_bundles(
        &self,
        bundle_ids: Option<&[String]>,
        owner: Option<&str>,
    ) -> Vec<&WorkspaceBundle> {
        self.bundles
            .values()
            .filter(|b| {
                if let Some(ids) = bundle_ids {
                    if !ids.contains(&b.bundle_id) {
                        return false;
                    }
                }
                if let Some(o) = owner {
                    let bundle_owner = b.owner.as_deref().unwrap_or("");
                    if bundle_owner != o {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    /// Create a workspace bundle.
    pub fn create_workspace_bundle(
        &mut self,
        bundle_name: &str,
        bundle_description: Option<&str>,
        image_id: &str,
        compute_type: &str,
        user_storage: i32,
        root_storage: Option<i32>,
        account_id: &str,
    ) -> Result<WorkspaceBundle, WorkSpacesError> {
        if !self.images.contains_key(image_id) {
            return Err(WorkSpacesError::ImageNotFound(image_id.to_string()));
        }
        let bundle_id = format!(
            "wsb-{}",
            &uuid::Uuid::new_v4().to_string().replace('-', "")[..25]
        );
        let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let bundle = WorkspaceBundle {
            bundle_id: bundle_id.clone(),
            name: bundle_name.to_string(),
            owner: Some(account_id.to_string()),
            description: bundle_description.map(|s| s.to_string()),
            bundle_type: None,
            compute_type_name: Some(compute_type.to_string()),
            root_storage_capacity: root_storage,
            user_storage_capacity: Some(user_storage),
            creation_time: now,
        };
        self.bundles.insert(bundle_id, bundle.clone());
        Ok(bundle)
    }

    /// Delete a workspace bundle.
    pub fn delete_workspace_bundle(&mut self, bundle_id: &str) -> Result<(), WorkSpacesError> {
        if self.bundles.remove(bundle_id).is_none() {
            return Err(WorkSpacesError::BundleNotFound(bundle_id.to_string()));
        }
        Ok(())
    }

    /// Delete a workspace image.
    pub fn delete_workspace_image(&mut self, image_id: &str) -> Result<(), WorkSpacesError> {
        if self.images.remove(image_id).is_none() {
            return Err(WorkSpacesError::ImageNotFound(image_id.to_string()));
        }
        self.image_permissions.remove(image_id);
        Ok(())
    }

    // --- Pools ---

    /// Create a WorkSpaces pool.
    pub fn create_workspaces_pool(
        &mut self,
        pool_name: &str,
        description: Option<&str>,
        bundle_id: &str,
        directory_id: &str,
        account_id: &str,
        region: &str,
    ) -> WorkspacesPool {
        let pool_id = format!(
            "wspool-{}",
            &uuid::Uuid::new_v4().to_string().replace('-', "")[..25]
        );
        let pool_arn = format!(
            "arn:aws:workspaces:{}:{}:workspacespool/{}",
            region, account_id, pool_id
        );
        let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let pool = WorkspacesPool {
            pool_id: pool_id.clone(),
            pool_arn,
            pool_name: pool_name.to_string(),
            description: description.map(|s| s.to_string()),
            state: "RUNNING".to_string(),
            bundle_id: bundle_id.to_string(),
            directory_id: directory_id.to_string(),
            created_at: now,
        };
        self.pools.insert(pool_id, pool.clone());
        pool
    }

    /// Describe WorkSpaces pools.
    pub fn describe_workspaces_pools(&self, pool_ids: Option<&[String]>) -> Vec<&WorkspacesPool> {
        self.pools
            .values()
            .filter(|p| {
                if let Some(ids) = pool_ids {
                    ids.contains(&p.pool_id)
                } else {
                    true
                }
            })
            .collect()
    }

    /// Terminate a WorkSpaces pool.
    pub fn terminate_workspaces_pool(
        &mut self,
        pool_id: &str,
    ) -> Result<WorkspacesPool, WorkSpacesError> {
        let mut pool = self
            .pools
            .remove(pool_id)
            .ok_or_else(|| WorkSpacesError::PoolNotFound(pool_id.to_string()))?;
        pool.state = "TERMINATING".to_string();
        Ok(pool)
    }

    /// Update a WorkSpaces pool.
    pub fn update_workspaces_pool(
        &mut self,
        pool_id: &str,
        description: Option<&str>,
        bundle_id: Option<&str>,
        directory_id: Option<&str>,
    ) -> Result<WorkspacesPool, WorkSpacesError> {
        let pool = self
            .pools
            .get_mut(pool_id)
            .ok_or_else(|| WorkSpacesError::PoolNotFound(pool_id.to_string()))?;
        if let Some(v) = description {
            pool.description = Some(v.to_string());
        }
        if let Some(v) = bundle_id {
            pool.bundle_id = v.to_string();
        }
        if let Some(v) = directory_id {
            pool.directory_id = v.to_string();
        }
        Ok(pool.clone())
    }

    /// Start a WorkSpaces pool.
    pub fn start_workspaces_pool(&mut self, pool_id: &str) -> Result<(), WorkSpacesError> {
        let pool = self
            .pools
            .get_mut(pool_id)
            .ok_or_else(|| WorkSpacesError::PoolNotFound(pool_id.to_string()))?;
        pool.state = "RUNNING".to_string();
        Ok(())
    }

    /// Stop a WorkSpaces pool.
    pub fn stop_workspaces_pool(&mut self, pool_id: &str) -> Result<(), WorkSpacesError> {
        let pool = self
            .pools
            .get_mut(pool_id)
            .ok_or_else(|| WorkSpacesError::PoolNotFound(pool_id.to_string()))?;
        pool.state = "STOPPED".to_string();
        Ok(())
    }
}
