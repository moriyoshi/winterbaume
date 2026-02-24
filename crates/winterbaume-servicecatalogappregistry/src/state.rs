use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct ServiceCatalogAppRegistryState {
    pub applications: HashMap<String, Application>,
    /// Index from application name to application id for uniqueness checks.
    pub application_names: HashMap<String, String>,

    pub attribute_groups: HashMap<String, AttributeGroup>,
    /// Index from attribute group name to attribute group id.
    pub attribute_group_names: HashMap<String, String>,

    /// application_id -> set of attribute_group_ids
    pub app_to_attr_groups: HashMap<String, Vec<String>>,

    /// application_id -> list of resource associations
    pub app_to_resources: HashMap<String, Vec<ResourceAssociation>>,

    /// Global configuration (not per-region in the real service, we store it here)
    pub configuration: AppRegistryConfig,
}

#[derive(Debug, thiserror::Error)]
pub enum AppRegistryError {
    #[error("Application with name '{0}' already exists.")]
    ApplicationAlreadyExists(String),

    #[error("Application '{0}' does not exist.")]
    ApplicationNotFound(String),

    #[error("AttributeGroup with name '{0}' already exists.")]
    AttributeGroupAlreadyExists(String),

    #[error("AttributeGroup '{0}' does not exist.")]
    AttributeGroupNotFound(String),

    #[error("Resource '{0}' already associated.")]
    ResourceAlreadyAssociated(String),

    #[error("Resource '{0}' not associated with application '{1}'.")]
    ResourceNotAssociated(String, String),

    #[error("Resource '{0}' does not exist.")]
    ResourceArnNotFound(String),
}

impl ServiceCatalogAppRegistryState {
    pub fn create_application(
        &mut self,
        name: &str,
        description: Option<&str>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&Application, AppRegistryError> {
        if self.application_names.contains_key(name) {
            return Err(AppRegistryError::ApplicationAlreadyExists(name.to_string()));
        }

        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        let arn = format!("arn:aws:servicecatalog:{region}:{account_id}:/applications/{id}");

        let app = Application {
            id: id.clone(),
            arn,
            name: name.to_string(),
            description: description.map(String::from),
            creation_time: now,
            last_update_time: now,
            tags,
        };

        self.application_names.insert(name.to_string(), id.clone());
        self.applications.insert(id.clone(), app);
        Ok(self.applications.get(&id).unwrap())
    }

    pub fn get_application(&self, id_or_name: &str) -> Result<&Application, AppRegistryError> {
        // Try by ID first, then by name
        if let Some(app) = self.applications.get(id_or_name) {
            return Ok(app);
        }
        if let Some(id) = self.application_names.get(id_or_name)
            && let Some(app) = self.applications.get(id)
        {
            return Ok(app);
        }
        Err(AppRegistryError::ApplicationNotFound(
            id_or_name.to_string(),
        ))
    }

    /// Resolve application id_or_name to an id string.
    fn resolve_application_id(&self, id_or_name: &str) -> Result<String, AppRegistryError> {
        if self.applications.contains_key(id_or_name) {
            return Ok(id_or_name.to_string());
        }
        if let Some(id) = self.application_names.get(id_or_name) {
            return Ok(id.clone());
        }
        Err(AppRegistryError::ApplicationNotFound(
            id_or_name.to_string(),
        ))
    }

    pub fn update_application(
        &mut self,
        id_or_name: &str,
        new_name: Option<&str>,
        description: Option<&str>,
    ) -> Result<&Application, AppRegistryError> {
        let id = self.resolve_application_id(id_or_name)?;

        // Check name conflict
        if let Some(n) = new_name {
            if let Some(existing_id) = self.application_names.get(n) {
                if *existing_id != id {
                    return Err(AppRegistryError::ApplicationAlreadyExists(n.to_string()));
                }
            }
        }

        let app = self.applications.get_mut(&id).unwrap();
        let old_name = app.name.clone();

        if let Some(n) = new_name {
            app.name = n.to_string();
        }
        if let Some(d) = description {
            app.description = Some(d.to_string());
        }
        app.last_update_time = Utc::now();

        // Update name index if name changed
        if let Some(n) = new_name {
            if n != old_name {
                self.application_names.remove(&old_name);
                self.application_names.insert(n.to_string(), id.clone());
            }
        }

        Ok(self.applications.get(&id).unwrap())
    }

    pub fn delete_application(&mut self, id_or_name: &str) -> Result<(), AppRegistryError> {
        // Resolve to ID
        let id = if self.applications.contains_key(id_or_name) {
            id_or_name.to_string()
        } else if let Some(id) = self.application_names.get(id_or_name) {
            id.clone()
        } else {
            return Err(AppRegistryError::ApplicationNotFound(
                id_or_name.to_string(),
            ));
        };

        if let Some(app) = self.applications.remove(&id) {
            self.application_names.remove(&app.name);
            self.app_to_attr_groups.remove(&id);
            self.app_to_resources.remove(&id);
            Ok(())
        } else {
            Err(AppRegistryError::ApplicationNotFound(
                id_or_name.to_string(),
            ))
        }
    }

    pub fn list_applications(&self) -> Vec<&Application> {
        self.applications.values().collect()
    }

    // ---- Attribute Groups ----

    pub fn create_attribute_group(
        &mut self,
        name: &str,
        description: Option<&str>,
        attributes: &str,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&AttributeGroup, AppRegistryError> {
        if self.attribute_group_names.contains_key(name) {
            return Err(AppRegistryError::AttributeGroupAlreadyExists(
                name.to_string(),
            ));
        }

        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        let arn = format!("arn:aws:servicecatalog:{region}:{account_id}:/attribute-groups/{id}");

        let ag = AttributeGroup {
            id: id.clone(),
            arn,
            name: name.to_string(),
            description: description.map(String::from),
            attributes: attributes.to_string(),
            creation_time: now,
            last_update_time: now,
            tags,
        };

        self.attribute_group_names
            .insert(name.to_string(), id.clone());
        self.attribute_groups.insert(id.clone(), ag);
        Ok(self.attribute_groups.get(&id).unwrap())
    }

    pub fn get_attribute_group(
        &self,
        id_or_name: &str,
    ) -> Result<&AttributeGroup, AppRegistryError> {
        if let Some(ag) = self.attribute_groups.get(id_or_name) {
            return Ok(ag);
        }
        if let Some(id) = self.attribute_group_names.get(id_or_name)
            && let Some(ag) = self.attribute_groups.get(id)
        {
            return Ok(ag);
        }
        Err(AppRegistryError::AttributeGroupNotFound(
            id_or_name.to_string(),
        ))
    }

    fn resolve_attribute_group_id(&self, id_or_name: &str) -> Result<String, AppRegistryError> {
        if self.attribute_groups.contains_key(id_or_name) {
            return Ok(id_or_name.to_string());
        }
        if let Some(id) = self.attribute_group_names.get(id_or_name) {
            return Ok(id.clone());
        }
        Err(AppRegistryError::AttributeGroupNotFound(
            id_or_name.to_string(),
        ))
    }

    pub fn update_attribute_group(
        &mut self,
        id_or_name: &str,
        new_name: Option<&str>,
        description: Option<&str>,
        attributes: Option<&str>,
    ) -> Result<&AttributeGroup, AppRegistryError> {
        let id = self.resolve_attribute_group_id(id_or_name)?;

        if let Some(n) = new_name {
            if let Some(existing_id) = self.attribute_group_names.get(n) {
                if *existing_id != id {
                    return Err(AppRegistryError::AttributeGroupAlreadyExists(n.to_string()));
                }
            }
        }

        let ag = self.attribute_groups.get_mut(&id).unwrap();
        let old_name = ag.name.clone();

        if let Some(n) = new_name {
            ag.name = n.to_string();
        }
        if let Some(d) = description {
            ag.description = Some(d.to_string());
        }
        if let Some(a) = attributes {
            ag.attributes = a.to_string();
        }
        ag.last_update_time = Utc::now();

        if let Some(n) = new_name {
            if n != old_name {
                self.attribute_group_names.remove(&old_name);
                self.attribute_group_names.insert(n.to_string(), id.clone());
            }
        }

        Ok(self.attribute_groups.get(&id).unwrap())
    }

    pub fn delete_attribute_group(
        &mut self,
        id_or_name: &str,
    ) -> Result<AttributeGroup, AppRegistryError> {
        let id = if self.attribute_groups.contains_key(id_or_name) {
            id_or_name.to_string()
        } else if let Some(id) = self.attribute_group_names.get(id_or_name) {
            id.clone()
        } else {
            return Err(AppRegistryError::AttributeGroupNotFound(
                id_or_name.to_string(),
            ));
        };

        if let Some(ag) = self.attribute_groups.remove(&id) {
            self.attribute_group_names.remove(&ag.name);
            // Remove associations
            for assocs in self.app_to_attr_groups.values_mut() {
                assocs.retain(|ag_id| ag_id != &id);
            }
            Ok(ag)
        } else {
            Err(AppRegistryError::AttributeGroupNotFound(
                id_or_name.to_string(),
            ))
        }
    }

    pub fn list_attribute_groups(&self) -> Vec<&AttributeGroup> {
        self.attribute_groups.values().collect()
    }

    // ---- Associations ----

    pub fn associate_attribute_group(
        &mut self,
        application: &str,
        attribute_group: &str,
    ) -> Result<(String, String), AppRegistryError> {
        let app_id = self.resolve_application_id(application)?;
        let ag_id = self.resolve_attribute_group_id(attribute_group)?;

        let app_arn = self.applications[&app_id].arn.clone();
        let ag_arn = self.attribute_groups[&ag_id].arn.clone();

        let assocs = self.app_to_attr_groups.entry(app_id).or_default();
        if !assocs.contains(&ag_id) {
            assocs.push(ag_id);
        }
        Ok((app_arn, ag_arn))
    }

    pub fn disassociate_attribute_group(
        &mut self,
        application: &str,
        attribute_group: &str,
    ) -> Result<(String, String), AppRegistryError> {
        let app_id = self.resolve_application_id(application)?;
        let ag_id = self.resolve_attribute_group_id(attribute_group)?;

        let app_arn = self.applications[&app_id].arn.clone();
        let ag_arn = self.attribute_groups[&ag_id].arn.clone();

        if let Some(assocs) = self.app_to_attr_groups.get_mut(&app_id) {
            assocs.retain(|id| id != &ag_id);
        }
        Ok((app_arn, ag_arn))
    }

    pub fn list_associated_attribute_groups(
        &self,
        application: &str,
    ) -> Result<Vec<String>, AppRegistryError> {
        let app_id = self.resolve_application_id(application)?;
        let ag_ids = self
            .app_to_attr_groups
            .get(&app_id)
            .cloned()
            .unwrap_or_default();
        // Return ARNs
        let arns = ag_ids
            .iter()
            .filter_map(|id| self.attribute_groups.get(id))
            .map(|ag| ag.arn.clone())
            .collect();
        Ok(arns)
    }

    pub fn list_attribute_groups_for_application(
        &self,
        application: &str,
    ) -> Result<Vec<&AttributeGroup>, AppRegistryError> {
        let app_id = self.resolve_application_id(application)?;
        let ag_ids = self
            .app_to_attr_groups
            .get(&app_id)
            .cloned()
            .unwrap_or_default();
        let groups = ag_ids
            .iter()
            .filter_map(|id| self.attribute_groups.get(id))
            .collect();
        Ok(groups)
    }

    pub fn associate_resource(
        &mut self,
        application: &str,
        resource_type: &str,
        resource: &str,
        options: Vec<String>,
    ) -> Result<(String, String), AppRegistryError> {
        let app_id = self.resolve_application_id(application)?;
        let app_arn = self.applications[&app_id].arn.clone();
        // We use the resource name as the ARN for simplicity in the mock
        let resource_arn = format!("arn:aws:{}:::{}", resource_type.to_lowercase(), resource);

        let assocs = self.app_to_resources.entry(app_id.clone()).or_default();
        // Check for duplicate
        let existing = assocs
            .iter()
            .position(|a| a.resource_type == resource_type && a.resource == resource);
        if existing.is_some() {
            return Err(AppRegistryError::ResourceAlreadyAssociated(
                resource.to_string(),
            ));
        }
        assocs.push(ResourceAssociation {
            application_id: app_id.clone(),
            resource_type: resource_type.to_string(),
            resource: resource.to_string(),
            resource_arn: Some(resource_arn.clone()),
            options,
        });
        Ok((app_arn, resource_arn))
    }

    pub fn disassociate_resource(
        &mut self,
        application: &str,
        resource_type: &str,
        resource: &str,
    ) -> Result<(String, String), AppRegistryError> {
        let app_id = self.resolve_application_id(application)?;
        let app_arn = self.applications[&app_id].arn.clone();
        let resource_arn = format!("arn:aws:{}:::{}", resource_type.to_lowercase(), resource);

        if let Some(assocs) = self.app_to_resources.get_mut(&app_id) {
            assocs.retain(|a| !(a.resource_type == resource_type && a.resource == resource));
        }
        Ok((app_arn, resource_arn))
    }

    pub fn get_associated_resource(
        &self,
        application: &str,
        resource_type: &str,
        resource: &str,
    ) -> Result<&ResourceAssociation, AppRegistryError> {
        let app_id = self.resolve_application_id(application)?;
        if let Some(assocs) = self.app_to_resources.get(&app_id) {
            if let Some(assoc) = assocs
                .iter()
                .find(|a| a.resource_type == resource_type && a.resource == resource)
            {
                return Ok(assoc);
            }
        }
        Err(AppRegistryError::ResourceNotAssociated(
            resource.to_string(),
            application.to_string(),
        ))
    }

    pub fn list_associated_resources(
        &self,
        application: &str,
    ) -> Result<Vec<&ResourceAssociation>, AppRegistryError> {
        let app_id = self.resolve_application_id(application)?;
        let assocs = self
            .app_to_resources
            .get(&app_id)
            .map(|v| v.iter().collect())
            .unwrap_or_default();
        Ok(assocs)
    }

    // ---- Tags ----

    /// Tag an application or attribute group by ARN.
    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), AppRegistryError> {
        // Find application by ARN
        if let Some(app) = self
            .applications
            .values_mut()
            .find(|a| a.arn == resource_arn)
        {
            app.tags.extend(tags);
            app.last_update_time = Utc::now();
            return Ok(());
        }
        // Find attribute group by ARN
        if let Some(ag) = self
            .attribute_groups
            .values_mut()
            .find(|a| a.arn == resource_arn)
        {
            ag.tags.extend(tags);
            ag.last_update_time = Utc::now();
            return Ok(());
        }
        Err(AppRegistryError::ResourceArnNotFound(
            resource_arn.to_string(),
        ))
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), AppRegistryError> {
        if let Some(app) = self
            .applications
            .values_mut()
            .find(|a| a.arn == resource_arn)
        {
            for key in tag_keys {
                app.tags.remove(key);
            }
            app.last_update_time = Utc::now();
            return Ok(());
        }
        if let Some(ag) = self
            .attribute_groups
            .values_mut()
            .find(|a| a.arn == resource_arn)
        {
            for key in tag_keys {
                ag.tags.remove(key);
            }
            ag.last_update_time = Utc::now();
            return Ok(());
        }
        Err(AppRegistryError::ResourceArnNotFound(
            resource_arn.to_string(),
        ))
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<HashMap<String, String>, AppRegistryError> {
        if let Some(app) = self.applications.values().find(|a| a.arn == resource_arn) {
            return Ok(app.tags.clone());
        }
        if let Some(ag) = self
            .attribute_groups
            .values()
            .find(|a| a.arn == resource_arn)
        {
            return Ok(ag.tags.clone());
        }
        Err(AppRegistryError::ResourceArnNotFound(
            resource_arn.to_string(),
        ))
    }

    // ---- Configuration ----

    pub fn get_configuration(&self) -> &AppRegistryConfig {
        &self.configuration
    }

    pub fn put_configuration(&mut self, tag_key: Option<String>) {
        self.configuration.tag_key = tag_key;
    }
}
