use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct AppIntegrationsState {
    pub applications: HashMap<String, Application>,
    pub data_integrations: HashMap<String, DataIntegration>,
    pub data_integration_associations: HashMap<String, DataIntegrationAssociation>,
    pub event_integrations: HashMap<String, EventIntegration>,
    pub event_integration_associations: HashMap<String, EventIntegrationAssociationRecord>,
    pub application_associations: HashMap<String, ApplicationAssociation>,
}

#[derive(Debug, Error)]
pub enum AppIntegrationsError {
    #[error("{resource_type} {identifier} does not exist.")]
    NotFound {
        resource_type: &'static str,
        identifier: String,
    },

    #[error("{resource_type} {identifier} already exists.")]
    AlreadyExists {
        resource_type: &'static str,
        identifier: String,
    },

    #[error(
        "{resource_type} {identifier} could not be deleted because dependent associations exist."
    )]
    InUse {
        resource_type: &'static str,
        identifier: String,
    },

    #[error("{message}")]
    Validation { message: String },
}

impl AppIntegrationsState {
    fn application_by_id_or_arn(&self, identifier: &str) -> Option<&Application> {
        if let Some(a) = self.applications.get(identifier) {
            return Some(a);
        }
        self.applications.values().find(|a| a.arn == identifier)
    }

    fn application_id_by_id_or_arn(&self, identifier: &str) -> Option<String> {
        self.application_by_id_or_arn(identifier)
            .map(|a| a.id.clone())
    }

    fn data_integration_by_id_or_arn(&self, identifier: &str) -> Option<&DataIntegration> {
        if let Some(d) = self.data_integrations.get(identifier) {
            return Some(d);
        }
        self.data_integrations
            .values()
            .find(|d| d.arn == identifier)
    }

    fn data_integration_id_by_id_or_arn(&self, identifier: &str) -> Option<String> {
        self.data_integration_by_id_or_arn(identifier)
            .map(|d| d.id.clone())
    }

    pub fn create_application(
        &mut self,
        application: Application,
    ) -> Result<&Application, AppIntegrationsError> {
        if self
            .applications
            .values()
            .any(|a| a.name == application.name)
        {
            return Err(AppIntegrationsError::AlreadyExists {
                resource_type: "Application",
                identifier: application.name.clone(),
            });
        }
        let id = application.id.clone();
        self.applications.insert(id.clone(), application);
        Ok(self.applications.get(&id).unwrap())
    }

    pub fn get_application(&self, identifier: &str) -> Result<&Application, AppIntegrationsError> {
        self.application_by_id_or_arn(identifier)
            .ok_or_else(|| AppIntegrationsError::NotFound {
                resource_type: "Application",
                identifier: identifier.to_string(),
            })
    }

    pub fn update_application(
        &mut self,
        identifier: &str,
        update: impl FnOnce(&mut Application),
    ) -> Result<&Application, AppIntegrationsError> {
        let id = self
            .application_id_by_id_or_arn(identifier)
            .ok_or_else(|| AppIntegrationsError::NotFound {
                resource_type: "Application",
                identifier: identifier.to_string(),
            })?;
        let app = self.applications.get_mut(&id).unwrap();
        update(app);
        app.last_modified_time = chrono::Utc::now().timestamp();
        Ok(app)
    }

    pub fn delete_application(&mut self, identifier: &str) -> Result<(), AppIntegrationsError> {
        let id = self
            .application_id_by_id_or_arn(identifier)
            .ok_or_else(|| AppIntegrationsError::NotFound {
                resource_type: "Application",
                identifier: identifier.to_string(),
            })?;
        self.applications.remove(&id);
        Ok(())
    }

    pub fn list_applications(&self) -> Vec<&Application> {
        let mut items: Vec<&Application> = self.applications.values().collect();
        items.sort_by(|a, b| a.name.cmp(&b.name));
        items
    }

    pub fn create_data_integration(
        &mut self,
        di: DataIntegration,
    ) -> Result<&DataIntegration, AppIntegrationsError> {
        if self.data_integrations.values().any(|d| d.name == di.name) {
            return Err(AppIntegrationsError::AlreadyExists {
                resource_type: "DataIntegration",
                identifier: di.name.clone(),
            });
        }
        let id = di.id.clone();
        self.data_integrations.insert(id.clone(), di);
        Ok(self.data_integrations.get(&id).unwrap())
    }

    pub fn get_data_integration(
        &self,
        identifier: &str,
    ) -> Result<&DataIntegration, AppIntegrationsError> {
        self.data_integration_by_id_or_arn(identifier)
            .ok_or_else(|| AppIntegrationsError::NotFound {
                resource_type: "DataIntegration",
                identifier: identifier.to_string(),
            })
    }

    pub fn update_data_integration(
        &mut self,
        identifier: &str,
        update: impl FnOnce(&mut DataIntegration),
    ) -> Result<&DataIntegration, AppIntegrationsError> {
        let id = self
            .data_integration_id_by_id_or_arn(identifier)
            .ok_or_else(|| AppIntegrationsError::NotFound {
                resource_type: "DataIntegration",
                identifier: identifier.to_string(),
            })?;
        let di = self.data_integrations.get_mut(&id).unwrap();
        update(di);
        Ok(di)
    }

    pub fn delete_data_integration(
        &mut self,
        identifier: &str,
    ) -> Result<(), AppIntegrationsError> {
        let id = self
            .data_integration_id_by_id_or_arn(identifier)
            .ok_or_else(|| AppIntegrationsError::NotFound {
                resource_type: "DataIntegration",
                identifier: identifier.to_string(),
            })?;
        if self
            .data_integration_associations
            .values()
            .any(|a| a.data_integration_id == id)
        {
            return Err(AppIntegrationsError::InUse {
                resource_type: "DataIntegration",
                identifier: identifier.to_string(),
            });
        }
        self.data_integrations.remove(&id);
        Ok(())
    }

    pub fn list_data_integrations(&self) -> Vec<&DataIntegration> {
        let mut items: Vec<&DataIntegration> = self.data_integrations.values().collect();
        items.sort_by(|a, b| a.name.cmp(&b.name));
        items
    }

    pub fn create_data_integration_association(
        &mut self,
        identifier: &str,
        association: DataIntegrationAssociation,
    ) -> Result<&DataIntegrationAssociation, AppIntegrationsError> {
        if self.data_integration_by_id_or_arn(identifier).is_none() {
            return Err(AppIntegrationsError::NotFound {
                resource_type: "DataIntegration",
                identifier: identifier.to_string(),
            });
        }
        let id = association.id.clone();
        self.data_integration_associations
            .insert(id.clone(), association);
        Ok(self.data_integration_associations.get(&id).unwrap())
    }

    pub fn update_data_integration_association(
        &mut self,
        identifier: &str,
        association_id: &str,
        last_execution_status: Option<String>,
    ) -> Result<&DataIntegrationAssociation, AppIntegrationsError> {
        let di_id = self
            .data_integration_id_by_id_or_arn(identifier)
            .ok_or_else(|| AppIntegrationsError::NotFound {
                resource_type: "DataIntegration",
                identifier: identifier.to_string(),
            })?;
        let assoc = self
            .data_integration_associations
            .get_mut(association_id)
            .ok_or_else(|| AppIntegrationsError::NotFound {
                resource_type: "DataIntegrationAssociation",
                identifier: association_id.to_string(),
            })?;
        if assoc.data_integration_id != di_id {
            return Err(AppIntegrationsError::Validation {
                message: format!(
                    "DataIntegrationAssociation {association_id} does not belong to DataIntegration {identifier}"
                ),
            });
        }
        if let Some(s) = last_execution_status {
            assoc.last_execution_status = Some(s);
        }
        assoc.last_modified_time = chrono::Utc::now().timestamp();
        Ok(assoc)
    }

    pub fn list_data_integration_associations(
        &self,
        identifier: &str,
    ) -> Result<Vec<&DataIntegrationAssociation>, AppIntegrationsError> {
        let id = self
            .data_integration_id_by_id_or_arn(identifier)
            .ok_or_else(|| AppIntegrationsError::NotFound {
                resource_type: "DataIntegration",
                identifier: identifier.to_string(),
            })?;
        let mut items: Vec<&DataIntegrationAssociation> = self
            .data_integration_associations
            .values()
            .filter(|a| a.data_integration_id == id)
            .collect();
        items.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(items)
    }

    pub fn create_event_integration(
        &mut self,
        ei: EventIntegration,
    ) -> Result<&EventIntegration, AppIntegrationsError> {
        if self.event_integrations.contains_key(&ei.name) {
            return Err(AppIntegrationsError::AlreadyExists {
                resource_type: "EventIntegration",
                identifier: ei.name.clone(),
            });
        }
        let name = ei.name.clone();
        self.event_integrations.insert(name.clone(), ei);
        Ok(self.event_integrations.get(&name).unwrap())
    }

    pub fn get_event_integration(
        &self,
        name: &str,
    ) -> Result<&EventIntegration, AppIntegrationsError> {
        self.event_integrations
            .get(name)
            .ok_or_else(|| AppIntegrationsError::NotFound {
                resource_type: "EventIntegration",
                identifier: name.to_string(),
            })
    }

    pub fn update_event_integration(
        &mut self,
        name: &str,
        description: Option<String>,
    ) -> Result<&EventIntegration, AppIntegrationsError> {
        let ei = self.event_integrations.get_mut(name).ok_or_else(|| {
            AppIntegrationsError::NotFound {
                resource_type: "EventIntegration",
                identifier: name.to_string(),
            }
        })?;
        if let Some(d) = description {
            ei.description = Some(d);
        }
        Ok(ei)
    }

    pub fn delete_event_integration(&mut self, name: &str) -> Result<(), AppIntegrationsError> {
        if !self.event_integrations.contains_key(name) {
            return Err(AppIntegrationsError::NotFound {
                resource_type: "EventIntegration",
                identifier: name.to_string(),
            });
        }
        if self
            .event_integration_associations
            .values()
            .any(|a| a.event_integration_name == name)
        {
            return Err(AppIntegrationsError::InUse {
                resource_type: "EventIntegration",
                identifier: name.to_string(),
            });
        }
        self.event_integrations.remove(name);
        Ok(())
    }

    pub fn list_event_integrations(&self) -> Vec<&EventIntegration> {
        let mut items: Vec<&EventIntegration> = self.event_integrations.values().collect();
        items.sort_by(|a, b| a.name.cmp(&b.name));
        items
    }

    pub fn list_event_integration_associations(
        &self,
        name: &str,
    ) -> Result<Vec<&EventIntegrationAssociationRecord>, AppIntegrationsError> {
        if !self.event_integrations.contains_key(name) {
            return Err(AppIntegrationsError::NotFound {
                resource_type: "EventIntegration",
                identifier: name.to_string(),
            });
        }
        let mut items: Vec<&EventIntegrationAssociationRecord> = self
            .event_integration_associations
            .values()
            .filter(|a| a.event_integration_name == name)
            .collect();
        items.sort_by(|a, b| {
            a.event_integration_association_id
                .cmp(&b.event_integration_association_id)
        });
        Ok(items)
    }

    pub fn list_application_associations(
        &self,
        identifier: &str,
    ) -> Result<Vec<&ApplicationAssociation>, AppIntegrationsError> {
        let id = self
            .application_id_by_id_or_arn(identifier)
            .ok_or_else(|| AppIntegrationsError::NotFound {
                resource_type: "Application",
                identifier: identifier.to_string(),
            })?;
        let mut items: Vec<&ApplicationAssociation> = self
            .application_associations
            .values()
            .filter(|a| a.application_id == id)
            .collect();
        items.sort_by(|a, b| a.client_id.cmp(&b.client_id));
        Ok(items)
    }

    fn locate_tags_mut(&mut self, arn: &str) -> Option<&mut HashMap<String, String>> {
        if let Some(app) = self.applications.values_mut().find(|a| a.arn == arn) {
            return Some(&mut app.tags);
        }
        if let Some(di) = self.data_integrations.values_mut().find(|d| d.arn == arn) {
            return Some(&mut di.tags);
        }
        if let Some(ei) = self.event_integrations.values_mut().find(|e| e.arn == arn) {
            return Some(&mut ei.tags);
        }
        None
    }

    fn locate_tags(&self, arn: &str) -> Option<&HashMap<String, String>> {
        if let Some(app) = self.applications.values().find(|a| a.arn == arn) {
            return Some(&app.tags);
        }
        if let Some(di) = self.data_integrations.values().find(|d| d.arn == arn) {
            return Some(&di.tags);
        }
        if let Some(ei) = self.event_integrations.values().find(|e| e.arn == arn) {
            return Some(&ei.tags);
        }
        None
    }

    pub fn tag_resource(
        &mut self,
        arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), AppIntegrationsError> {
        let bag = self
            .locate_tags_mut(arn)
            .ok_or_else(|| AppIntegrationsError::NotFound {
                resource_type: "Resource",
                identifier: arn.to_string(),
            })?;
        for (k, v) in tags {
            bag.insert(k, v);
        }
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        arn: &str,
        keys: &[String],
    ) -> Result<(), AppIntegrationsError> {
        let bag = self
            .locate_tags_mut(arn)
            .ok_or_else(|| AppIntegrationsError::NotFound {
                resource_type: "Resource",
                identifier: arn.to_string(),
            })?;
        for k in keys {
            bag.remove(k);
        }
        Ok(())
    }

    pub fn list_tags(&self, arn: &str) -> Result<HashMap<String, String>, AppIntegrationsError> {
        self.locate_tags(arn)
            .cloned()
            .ok_or_else(|| AppIntegrationsError::NotFound {
                resource_type: "Resource",
                identifier: arn.to_string(),
            })
    }
}
