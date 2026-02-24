use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct GreengrassState {
    pub groups: HashMap<String, Group>,
    pub definitions: HashMap<(DefinitionType, String), Definition>,
    pub definition_versions: HashMap<(DefinitionType, String, String), DefinitionVersion>,
    pub group_versions: HashMap<(String, String), GroupVersionInfo>,
    pub deployments: HashMap<(String, String), DeploymentInfo>,
    pub role_associations: HashMap<String, RoleAssociation>,
}

#[derive(Debug, thiserror::Error)]
pub enum GreengrassError {
    #[error("{0}")]
    IdNotFound(String),
    #[error("{0}")]
    VersionNotFound(String),
    #[error("Input does not contain any attributes to be updated")]
    InvalidContainerDefinition,
    #[error("{0}")]
    InvalidInput(String),
    #[error("{0}")]
    ResourceNotFound(String),
    #[error("You need to attach an IAM role to this deployment group.")]
    RoleNotAssociated,
    #[error("Invalid JSON body")]
    BadRequest,
    #[error("Not found")]
    UnknownOperation,
}

impl GreengrassState {
    // ---- Groups ----

    pub fn create_group(
        &mut self,
        name: &str,
        region: &str,
        account_id: &str,
    ) -> Result<Group, GreengrassError> {
        let group_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let version_id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:greengrass:{region}:{account_id}:/greengrass/groups/{group_id}");
        let version_arn = format!(
            "arn:aws:greengrass:{region}:{account_id}:/greengrass/groups/{group_id}/versions/{version_id}"
        );

        let group = Group {
            id: group_id.clone(),
            name: name.to_string(),
            arn,
            creation_timestamp: now.clone(),
            last_updated_timestamp: now.clone(),
            latest_version: version_id.clone(),
            latest_version_arn: version_arn.clone(),
        };

        // Also store the initial group version
        let gv = GroupVersionInfo {
            arn: version_arn,
            id: group_id.clone(),
            version: version_id.clone(),
            creation_timestamp: now,
            group_id: group_id.clone(),
            core_definition_version_arn: None,
            device_definition_version_arn: None,
            function_definition_version_arn: None,
            resource_definition_version_arn: None,
            subscription_definition_version_arn: None,
        };
        self.group_versions
            .insert((group_id.clone(), version_id), gv);

        self.groups.insert(group_id.clone(), group.clone());
        Ok(group)
    }

    pub fn get_group(&self, group_id: &str) -> Result<&Group, GreengrassError> {
        self.groups.get(group_id).ok_or_else(|| {
            GreengrassError::IdNotFound("That Group Definition does not exist.".to_string())
        })
    }

    pub fn delete_group(&mut self, group_id: &str) -> Result<(), GreengrassError> {
        if self.groups.remove(group_id).is_none() {
            return Err(GreengrassError::IdNotFound(
                "That group definition does not exist.".to_string(),
            ));
        }
        Ok(())
    }

    pub fn list_groups(&self) -> Vec<&Group> {
        self.groups.values().collect()
    }

    pub fn update_group(&mut self, group_id: &str, name: &str) -> Result<(), GreengrassError> {
        if name.is_empty() {
            return Err(GreengrassError::InvalidContainerDefinition);
        }
        let group = self.groups.get_mut(group_id).ok_or_else(|| {
            GreengrassError::IdNotFound("That group definition does not exist.".to_string())
        })?;
        group.name = name.to_string();
        group.last_updated_timestamp = Utc::now().to_rfc3339();
        Ok(())
    }

    // ---- Group Versions ----

    pub fn create_group_version(
        &mut self,
        group_id: &str,
        region: &str,
        account_id: &str,
        core_definition_version_arn: Option<String>,
        device_definition_version_arn: Option<String>,
        function_definition_version_arn: Option<String>,
        resource_definition_version_arn: Option<String>,
        subscription_definition_version_arn: Option<String>,
    ) -> Result<GroupVersionInfo, GreengrassError> {
        let group = self
            .groups
            .get(group_id)
            .ok_or_else(|| GreengrassError::IdNotFound("That group does not exist.".to_string()))?;

        let version_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let arn = format!(
            "arn:aws:greengrass:{region}:{account_id}:/greengrass/groups/{group_id}/versions/{version_id}"
        );

        let gv = GroupVersionInfo {
            arn: arn.clone(),
            id: group.id.clone(),
            version: version_id.clone(),
            creation_timestamp: now,
            group_id: group_id.to_string(),
            core_definition_version_arn,
            device_definition_version_arn,
            function_definition_version_arn,
            resource_definition_version_arn,
            subscription_definition_version_arn,
        };

        // Update group latest version
        let group = self.groups.get_mut(group_id).unwrap();
        group.latest_version = version_id.clone();
        group.latest_version_arn = arn;
        group.last_updated_timestamp = Utc::now().to_rfc3339();

        self.group_versions
            .insert((group_id.to_string(), version_id), gv.clone());
        Ok(gv)
    }

    pub fn get_group_version(
        &self,
        group_id: &str,
        group_version_id: &str,
    ) -> Result<&GroupVersionInfo, GreengrassError> {
        // First check group exists
        if !self.groups.contains_key(group_id) {
            return Err(GreengrassError::IdNotFound(
                "That group definition does not exist.".to_string(),
            ));
        }
        self.group_versions
            .get(&(group_id.to_string(), group_version_id.to_string()))
            .ok_or_else(|| {
                GreengrassError::VersionNotFound(format!(
                    "Version {group_version_id} of Group Definition {group_id} does not exist."
                ))
            })
    }

    pub fn list_group_versions(
        &self,
        group_id: &str,
    ) -> Result<Vec<&GroupVersionInfo>, GreengrassError> {
        // Verify group exists
        if !self.groups.contains_key(group_id) {
            return Err(GreengrassError::IdNotFound(
                "That group definition does not exist.".to_string(),
            ));
        }
        Ok(self
            .group_versions
            .values()
            .filter(|gv| gv.group_id == group_id)
            .collect())
    }

    // ---- Definitions (generic) ----

    pub fn create_definition(
        &mut self,
        def_type: DefinitionType,
        name: &str,
        region: &str,
        account_id: &str,
    ) -> Result<Definition, GreengrassError> {
        let def_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let version_id = uuid::Uuid::new_v4().to_string();
        let segment = def_type.path_segment();
        let arn = format!(
            "arn:aws:greengrass:{region}:{account_id}:/greengrass/definition/{segment}/{def_id}"
        );
        let version_arn = format!(
            "arn:aws:greengrass:{region}:{account_id}:/greengrass/definition/{segment}/{def_id}/versions/{version_id}"
        );

        let def = Definition {
            id: def_id.clone(),
            name: name.to_string(),
            arn,
            creation_timestamp: now.clone(),
            last_updated_timestamp: now.clone(),
            latest_version: version_id.clone(),
            latest_version_arn: version_arn.clone(),
            def_type,
        };

        // Also create the initial version
        let dv = DefinitionVersion {
            arn: version_arn,
            id: def_id.clone(),
            version: version_id.clone(),
            creation_timestamp: now,
            definition_id: def_id.clone(),
        };

        self.definitions
            .insert((def_type, def_id.clone()), def.clone());
        self.definition_versions
            .insert((def_type, def_id, version_id), dv);
        Ok(def)
    }

    pub fn get_definition(
        &self,
        def_type: DefinitionType,
        def_id: &str,
    ) -> Result<&Definition, GreengrassError> {
        self.definitions
            .get(&(def_type, def_id.to_string()))
            .ok_or_else(|| {
                GreengrassError::IdNotFound(format!(
                    "That {} definition does not exist.",
                    def_type.display_name()
                ))
            })
    }

    pub fn delete_definition(
        &mut self,
        def_type: DefinitionType,
        def_id: &str,
    ) -> Result<(), GreengrassError> {
        if self
            .definitions
            .remove(&(def_type, def_id.to_string()))
            .is_none()
        {
            return Err(GreengrassError::IdNotFound(format!(
                "That {} definition does not exist.",
                def_type.display_name()
            )));
        }
        // Also remove all versions
        self.definition_versions
            .retain(|k, _| !(k.0 == def_type && k.1 == def_id));
        Ok(())
    }

    pub fn update_definition(
        &mut self,
        def_type: DefinitionType,
        def_id: &str,
        name: &str,
    ) -> Result<(), GreengrassError> {
        if name.is_empty() {
            return Err(GreengrassError::InvalidContainerDefinition);
        }
        let def = self
            .definitions
            .get_mut(&(def_type, def_id.to_string()))
            .ok_or_else(|| {
                GreengrassError::IdNotFound(format!(
                    "That {} definition does not exist.",
                    def_type.display_name()
                ))
            })?;
        def.name = name.to_string();
        def.last_updated_timestamp = Utc::now().to_rfc3339();
        Ok(())
    }

    pub fn list_definitions(&self, def_type: DefinitionType) -> Vec<&Definition> {
        self.definitions
            .values()
            .filter(|d| d.def_type == def_type)
            .collect()
    }

    pub fn create_definition_version(
        &mut self,
        def_type: DefinitionType,
        def_id: &str,
        region: &str,
        account_id: &str,
    ) -> Result<DefinitionVersion, GreengrassError> {
        let def = self
            .definitions
            .get(&(def_type, def_id.to_string()))
            .ok_or_else(|| {
                GreengrassError::IdNotFound(format!(
                    "That {} does not exist.",
                    def_type.display_name()
                ))
            })?;

        let version_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let segment = def_type.path_segment();
        let version_arn = format!(
            "arn:aws:greengrass:{region}:{account_id}:/greengrass/definition/{segment}/{def_id}/versions/{version_id}"
        );

        let dv = DefinitionVersion {
            arn: version_arn.clone(),
            id: def.id.clone(),
            version: version_id.clone(),
            creation_timestamp: now,
            definition_id: def_id.to_string(),
        };

        // Update the definition's latest version
        let def = self
            .definitions
            .get_mut(&(def_type, def_id.to_string()))
            .unwrap();
        def.latest_version = version_id.clone();
        def.latest_version_arn = version_arn;
        def.last_updated_timestamp = Utc::now().to_rfc3339();

        self.definition_versions
            .insert((def_type, def_id.to_string(), version_id), dv.clone());
        Ok(dv)
    }

    pub fn get_definition_version(
        &self,
        def_type: DefinitionType,
        def_id: &str,
        version_id: &str,
    ) -> Result<&DefinitionVersion, GreengrassError> {
        // First check definition exists
        if !self
            .definitions
            .contains_key(&(def_type, def_id.to_string()))
        {
            return Err(GreengrassError::IdNotFound(format!(
                "That {} definition does not exist.",
                def_type.display_name()
            )));
        }
        self.definition_versions
            .get(&(def_type, def_id.to_string(), version_id.to_string()))
            .ok_or_else(|| {
                GreengrassError::VersionNotFound(format!(
                    "Version {} of {} Definition {} does not exist.",
                    version_id,
                    def_type.version_display_name(),
                    def_id
                ))
            })
    }

    pub fn list_definition_versions(
        &self,
        def_type: DefinitionType,
        def_id: &str,
    ) -> Result<Vec<&DefinitionVersion>, GreengrassError> {
        // Verify definition exists
        self.get_definition(def_type, def_id)?;
        Ok(self
            .definition_versions
            .values()
            .filter(|dv| dv.definition_id == def_id)
            .collect())
    }

    // ---- Deployments ----

    pub fn create_deployment(
        &mut self,
        group_id: &str,
        deployment_type: &str,
        region: &str,
        account_id: &str,
    ) -> Result<DeploymentInfo, GreengrassError> {
        let group = self.groups.get(group_id).ok_or_else(|| {
            GreengrassError::ResourceNotFound("That group definition does not exist.".to_string())
        })?;

        let deployment_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let deployment_arn = format!(
            "arn:aws:greengrass:{region}:{account_id}:/greengrass/groups/{group_id}/deployments/{deployment_id}"
        );

        let deployment = DeploymentInfo {
            deployment_id: deployment_id.clone(),
            deployment_arn,
            deployment_type: deployment_type.to_string(),
            group_arn: group.arn.clone(),
            created_at: now.clone(),
            deployment_status: "InProgress".to_string(),
            updated_at: now,
        };

        self.deployments
            .insert((group_id.to_string(), deployment_id), deployment.clone());
        Ok(deployment)
    }

    pub fn get_deployment_status(
        &self,
        group_id: &str,
        deployment_id: &str,
    ) -> Result<&DeploymentInfo, GreengrassError> {
        self.deployments
            .get(&(group_id.to_string(), deployment_id.to_string()))
            .ok_or_else(|| {
                GreengrassError::InvalidInput(format!(
                    "Deployment '{}' does not exist.",
                    deployment_id
                ))
            })
    }

    pub fn list_deployments(
        &self,
        group_id: &str,
    ) -> Result<Vec<&DeploymentInfo>, GreengrassError> {
        self.get_group(group_id)?;
        Ok(self
            .deployments
            .values()
            .filter(|d| {
                d.group_arn
                    .contains(&format!("/greengrass/groups/{group_id}"))
            })
            .collect())
    }

    pub fn reset_deployments(
        &mut self,
        group_id: &str,
        region: &str,
        account_id: &str,
    ) -> Result<DeploymentInfo, GreengrassError> {
        if !self.groups.contains_key(group_id) {
            return Err(GreengrassError::ResourceNotFound(
                "That Group Definition does not exist.".to_string(),
            ));
        }

        // Check there are non-reset deployments for this group
        let has_non_reset_deployments = self.deployments.values().any(|d| {
            d.group_arn
                .contains(&format!("/greengrass/groups/{group_id}"))
                && d.deployment_type != "ResetDeployment"
        });
        if !has_non_reset_deployments {
            return Err(GreengrassError::ResourceNotFound(format!(
                "Group id: {group_id} has not been deployed or has already been reset."
            )));
        }

        // Remove the non-reset deployments (keep reset ones)
        self.deployments.retain(|_, d| {
            !(d.group_arn
                .contains(&format!("/greengrass/groups/{group_id}"))
                && d.deployment_type != "ResetDeployment")
        });

        // Create a reset deployment record
        let deployment_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let group = self.groups.get(group_id).unwrap();
        let deployment_arn = format!(
            "arn:aws:greengrass:{region}:{account_id}:/greengrass/groups/{group_id}/deployments/{deployment_id}"
        );

        let deployment = DeploymentInfo {
            deployment_id: deployment_id.clone(),
            deployment_arn,
            deployment_type: "ResetDeployment".to_string(),
            group_arn: group.arn.clone(),
            created_at: now.clone(),
            deployment_status: "Success".to_string(),
            updated_at: now,
        };

        self.deployments
            .insert((group_id.to_string(), deployment_id), deployment.clone());
        Ok(deployment)
    }

    // ---- Role Association ----

    pub fn associate_role_to_group(
        &mut self,
        group_id: &str,
        role_arn: &str,
    ) -> Result<String, GreengrassError> {
        // Moto does not require the group to exist for role association
        let now = Utc::now().to_rfc3339();
        self.role_associations.insert(
            group_id.to_string(),
            RoleAssociation {
                role_arn: role_arn.to_string(),
                associated_at: now.clone(),
            },
        );
        Ok(now)
    }

    pub fn disassociate_role_from_group(
        &mut self,
        group_id: &str,
    ) -> Result<String, GreengrassError> {
        // Moto returns success even if no role was associated
        self.role_associations.remove(group_id);
        Ok(Utc::now().to_rfc3339())
    }

    pub fn get_associated_role(&self, group_id: &str) -> Result<&RoleAssociation, GreengrassError> {
        self.role_associations
            .get(group_id)
            .ok_or(GreengrassError::RoleNotAssociated)
    }
}
