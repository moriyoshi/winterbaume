use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct ApplicationDiscoveryState {
    pub applications: HashMap<String, Application>,
    pub tags: Vec<ConfigurationTag>,
    pub import_tasks: HashMap<String, ImportTask>,
    pub export_tasks: HashMap<String, ExportTask>,
    pub continuous_exports: HashMap<String, ContinuousExport>,
    pub batch_delete_tasks: HashMap<String, BatchDeleteTask>,
    /// Application -> configurationItem associations.
    pub associations: HashMap<String, Vec<String>>,
}

#[derive(Debug, Error)]
pub enum ApplicationDiscoveryError {
    #[error("{resource_type} {id} does not exist.")]
    NotFound {
        resource_type: &'static str,
        id: String,
    },

    #[error("{message}")]
    Validation { message: String },
}

impl ApplicationDiscoveryState {
    pub fn create_application(&mut self, app: Application) -> &Application {
        let id = app.configuration_id.clone();
        self.applications.insert(id.clone(), app);
        self.applications.get(&id).unwrap()
    }

    pub fn update_application(
        &mut self,
        id: &str,
        update: impl FnOnce(&mut Application),
    ) -> Result<&Application, ApplicationDiscoveryError> {
        let a =
            self.applications
                .get_mut(id)
                .ok_or_else(|| ApplicationDiscoveryError::NotFound {
                    resource_type: "Application",
                    id: id.to_string(),
                })?;
        update(a);
        a.last_modified_time_stamp = chrono::Utc::now().timestamp();
        Ok(a)
    }

    pub fn delete_applications(&mut self, ids: &[String]) {
        for id in ids {
            self.applications.remove(id);
            self.associations.remove(id);
            self.tags.retain(|t| t.configuration_id != *id);
        }
    }

    pub fn list_applications(&self) -> Vec<&Application> {
        let mut items: Vec<&Application> = self.applications.values().collect();
        items.sort_by(|a, b| a.name.cmp(&b.name));
        items
    }

    pub fn create_tags(&mut self, configuration_ids: &[String], tags: Vec<(String, String)>) {
        let now = chrono::Utc::now().timestamp();
        for cid in configuration_ids {
            for (k, v) in &tags {
                let configuration_type = if self.applications.contains_key(cid) {
                    "APPLICATION".to_string()
                } else {
                    "OTHER".to_string()
                };
                self.tags
                    .retain(|t| !(t.configuration_id == *cid && t.key == *k));
                self.tags.push(ConfigurationTag {
                    configuration_id: cid.clone(),
                    configuration_type,
                    key: k.clone(),
                    value: v.clone(),
                    time_of_creation: now,
                });
            }
        }
    }

    pub fn delete_tags(&mut self, configuration_ids: &[String], keys: &[String]) {
        self.tags.retain(|t| {
            !(configuration_ids.contains(&t.configuration_id) && keys.contains(&t.key))
        });
    }

    pub fn list_tags(&self, configuration_id: Option<&str>) -> Vec<&ConfigurationTag> {
        self.tags
            .iter()
            .filter(|t| configuration_id.is_none_or(|c| t.configuration_id == c))
            .collect()
    }

    pub fn add_associations(&mut self, application_id: &str, configuration_ids: &[String]) {
        let entry = self
            .associations
            .entry(application_id.to_string())
            .or_default();
        for cid in configuration_ids {
            if !entry.contains(cid) {
                entry.push(cid.clone());
            }
        }
    }

    pub fn remove_associations(&mut self, application_id: &str, configuration_ids: &[String]) {
        if let Some(entry) = self.associations.get_mut(application_id) {
            entry.retain(|c| !configuration_ids.contains(c));
        }
    }

    pub fn add_import_task(&mut self, task: ImportTask) -> &ImportTask {
        let id = task.id.clone();
        self.import_tasks.insert(id.clone(), task);
        self.import_tasks.get(&id).unwrap()
    }

    pub fn batch_delete_import_data(
        &mut self,
        ids: &[String],
        delete_history: bool,
    ) -> Vec<String> {
        let mut errors = vec![];
        for id in ids {
            match self.import_tasks.get_mut(id) {
                Some(t) => {
                    if delete_history {
                        self.import_tasks.remove(id);
                    } else {
                        t.import_deleted_time = Some(chrono::Utc::now().timestamp());
                    }
                }
                None => errors.push(id.clone()),
            }
        }
        errors
    }

    pub fn list_import_tasks(&self) -> Vec<&ImportTask> {
        let mut items: Vec<&ImportTask> = self.import_tasks.values().collect();
        items.sort_by_key(|b| std::cmp::Reverse(b.import_request_time));
        items
    }

    pub fn add_export_task(&mut self, task: ExportTask) -> &ExportTask {
        let id = task.id.clone();
        self.export_tasks.insert(id.clone(), task);
        self.export_tasks.get(&id).unwrap()
    }

    pub fn list_export_tasks(&self) -> Vec<&ExportTask> {
        let mut items: Vec<&ExportTask> = self.export_tasks.values().collect();
        items.sort_by_key(|b| std::cmp::Reverse(b.export_request_time));
        items
    }

    pub fn add_continuous_export(&mut self, export: ContinuousExport) -> &ContinuousExport {
        let id = export.export_id.clone();
        self.continuous_exports.insert(id.clone(), export);
        self.continuous_exports.get(&id).unwrap()
    }

    pub fn stop_continuous_export(
        &mut self,
        export_id: &str,
    ) -> Result<&ContinuousExport, ApplicationDiscoveryError> {
        let e = self.continuous_exports.get_mut(export_id).ok_or_else(|| {
            ApplicationDiscoveryError::NotFound {
                resource_type: "ContinuousExport",
                id: export_id.to_string(),
            }
        })?;
        e.status = "INACTIVE".to_string();
        e.stop_time = Some(chrono::Utc::now().timestamp());
        Ok(e)
    }

    pub fn list_continuous_exports(&self) -> Vec<&ContinuousExport> {
        let mut items: Vec<&ContinuousExport> = self.continuous_exports.values().collect();
        items.sort_by(|a, b| a.export_id.cmp(&b.export_id));
        items
    }

    pub fn add_batch_delete_task(&mut self, task: BatchDeleteTask) -> &BatchDeleteTask {
        let id = task.id.clone();
        self.batch_delete_tasks.insert(id.clone(), task);
        self.batch_delete_tasks.get(&id).unwrap()
    }

    pub fn get_batch_delete_task(
        &self,
        id: &str,
    ) -> Result<&BatchDeleteTask, ApplicationDiscoveryError> {
        self.batch_delete_tasks
            .get(id)
            .ok_or(ApplicationDiscoveryError::NotFound {
                resource_type: "BatchDeleteConfigurationTask",
                id: id.to_string(),
            })
    }
}
