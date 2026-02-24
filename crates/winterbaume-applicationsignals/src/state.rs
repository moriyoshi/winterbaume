use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct ApplicationSignalsState {
    pub slos: HashMap<String, ServiceLevelObjective>,
    pub grouping_configuration: Option<GroupingConfiguration>,
    /// Service inventory ( read-only mock; default empty, seedable via state view ).
    pub services: Vec<ServiceEntry>,
    pub discovery_started: bool,
}

#[derive(Debug, Error)]
pub enum ApplicationSignalsError {
    #[error("ServiceLevelObjective {id} does not exist.")]
    SloNotFound { id: String },

    #[error("ServiceLevelObjective with name {name} already exists.")]
    SloAlreadyExists { name: String },

    #[error("Resource {arn} does not exist.")]
    ResourceNotFound { arn: String },

    #[error("{message}")]
    Validation { message: String },
}

impl ApplicationSignalsState {
    pub fn create_slo(
        &mut self,
        slo: ServiceLevelObjective,
    ) -> Result<&ServiceLevelObjective, ApplicationSignalsError> {
        if self.slos.values().any(|s| s.name == slo.name) {
            return Err(ApplicationSignalsError::SloAlreadyExists {
                name: slo.name.clone(),
            });
        }
        let id = slo.id.clone();
        self.slos.insert(id.clone(), slo);
        Ok(self.slos.get(&id).unwrap())
    }

    fn id_by_id_or_arn(&self, identifier: &str) -> Option<String> {
        if self.slos.contains_key(identifier) {
            return Some(identifier.to_string());
        }
        self.slos
            .values()
            .find(|s| s.arn == identifier || s.name == identifier)
            .map(|s| s.id.clone())
    }

    pub fn get_slo(
        &self,
        identifier: &str,
    ) -> Result<&ServiceLevelObjective, ApplicationSignalsError> {
        let id = self.id_by_id_or_arn(identifier).ok_or_else(|| {
            ApplicationSignalsError::SloNotFound {
                id: identifier.to_string(),
            }
        })?;
        self.slos
            .get(&id)
            .ok_or(ApplicationSignalsError::SloNotFound { id })
    }

    pub fn update_slo(
        &mut self,
        identifier: &str,
        update: impl FnOnce(&mut ServiceLevelObjective),
    ) -> Result<&ServiceLevelObjective, ApplicationSignalsError> {
        let id = self.id_by_id_or_arn(identifier).ok_or_else(|| {
            ApplicationSignalsError::SloNotFound {
                id: identifier.to_string(),
            }
        })?;
        let s = self.slos.get_mut(&id).unwrap();
        update(s);
        s.last_updated_time = chrono::Utc::now().timestamp();
        Ok(s)
    }

    pub fn delete_slo(&mut self, identifier: &str) -> Result<(), ApplicationSignalsError> {
        let id = self.id_by_id_or_arn(identifier).ok_or_else(|| {
            ApplicationSignalsError::SloNotFound {
                id: identifier.to_string(),
            }
        })?;
        self.slos.remove(&id);
        Ok(())
    }

    pub fn list_slos(&self) -> Vec<&ServiceLevelObjective> {
        let mut items: Vec<&ServiceLevelObjective> = self.slos.values().collect();
        items.sort_by(|a, b| a.name.cmp(&b.name));
        items
    }

    pub fn tag_resource(
        &mut self,
        arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), ApplicationSignalsError> {
        let s = self
            .slos
            .values_mut()
            .find(|s| s.arn == arn)
            .ok_or_else(|| ApplicationSignalsError::ResourceNotFound {
                arn: arn.to_string(),
            })?;
        for (k, v) in tags {
            s.tags.insert(k, v);
        }
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        arn: &str,
        keys: &[String],
    ) -> Result<(), ApplicationSignalsError> {
        let s = self
            .slos
            .values_mut()
            .find(|s| s.arn == arn)
            .ok_or_else(|| ApplicationSignalsError::ResourceNotFound {
                arn: arn.to_string(),
            })?;
        for k in keys {
            s.tags.remove(k);
        }
        Ok(())
    }

    pub fn list_tags(&self, arn: &str) -> Result<HashMap<String, String>, ApplicationSignalsError> {
        self.slos
            .values()
            .find(|s| s.arn == arn)
            .map(|s| s.tags.clone())
            .ok_or_else(|| ApplicationSignalsError::ResourceNotFound {
                arn: arn.to_string(),
            })
    }
}
