use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct BcmDataExportsState {
    pub exports: HashMap<String, Export>,
    pub executions: HashMap<String, Execution>,
    /// Available CUR table catalogue. Empty by default; tests can seed via state view.
    pub tables: HashMap<String, TableCatalogueEntry>,
}

#[derive(Debug, Clone)]
pub struct TableCatalogueEntry {
    pub name: String,
    pub description: Option<String>,
    pub schema: Vec<TableColumn>,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct TableColumn {
    pub name: String,
    pub r#type: String,
    pub description: Option<String>,
}

#[derive(Debug, Error)]
pub enum BcmDataExportsError {
    #[error("Export {arn} does not exist.")]
    ExportNotFound { arn: String },

    #[error("Export with name {name} already exists.")]
    ExportAlreadyExists { name: String },

    #[error("Execution {id} does not exist.")]
    ExecutionNotFound { id: String },

    #[error("Table {name} does not exist.")]
    TableNotFound { name: String },

    #[error("{message}")]
    Validation { message: String },
}

impl BcmDataExportsState {
    pub fn create_export(&mut self, export: Export) -> Result<&Export, BcmDataExportsError> {
        if self.exports.values().any(|e| e.name == export.name) {
            return Err(BcmDataExportsError::ExportAlreadyExists {
                name: export.name.clone(),
            });
        }
        let arn = export.arn.clone();
        self.exports.insert(arn.clone(), export);
        Ok(self.exports.get(&arn).unwrap())
    }

    pub fn get_export(&self, arn: &str) -> Result<&Export, BcmDataExportsError> {
        self.exports
            .get(arn)
            .ok_or_else(|| BcmDataExportsError::ExportNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn update_export(
        &mut self,
        arn: &str,
        update: impl FnOnce(&mut Export),
    ) -> Result<&Export, BcmDataExportsError> {
        let e = self
            .exports
            .get_mut(arn)
            .ok_or_else(|| BcmDataExportsError::ExportNotFound {
                arn: arn.to_string(),
            })?;
        update(e);
        e.last_updated_at = chrono::Utc::now().to_rfc3339();
        Ok(e)
    }

    pub fn delete_export(&mut self, arn: &str) -> Result<(), BcmDataExportsError> {
        self.exports
            .remove(arn)
            .ok_or_else(|| BcmDataExportsError::ExportNotFound {
                arn: arn.to_string(),
            })?;
        // Also delete associated executions
        self.executions.retain(|_, e| e.export_arn != arn);
        Ok(())
    }

    pub fn list_exports(&self) -> Vec<&Export> {
        let mut items: Vec<&Export> = self.exports.values().collect();
        items.sort_by(|a, b| a.name.cmp(&b.name));
        items
    }

    pub fn list_executions(&self, export_arn: &str) -> Vec<&Execution> {
        let mut items: Vec<&Execution> = self
            .executions
            .values()
            .filter(|e| e.export_arn == export_arn)
            .collect();
        items.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        items
    }

    pub fn get_execution(&self, id: &str) -> Result<&Execution, BcmDataExportsError> {
        self.executions
            .get(id)
            .ok_or_else(|| BcmDataExportsError::ExecutionNotFound { id: id.to_string() })
    }

    pub fn get_table(&self, name: &str) -> Result<&TableCatalogueEntry, BcmDataExportsError> {
        self.tables
            .get(name)
            .ok_or_else(|| BcmDataExportsError::TableNotFound {
                name: name.to_string(),
            })
    }

    pub fn list_tables(&self) -> Vec<&TableCatalogueEntry> {
        let mut items: Vec<&TableCatalogueEntry> = self.tables.values().collect();
        items.sort_by(|a, b| a.name.cmp(&b.name));
        items
    }

    pub fn tag_resource(
        &mut self,
        arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), BcmDataExportsError> {
        let e = self
            .exports
            .get_mut(arn)
            .ok_or_else(|| BcmDataExportsError::ExportNotFound {
                arn: arn.to_string(),
            })?;
        for (k, v) in tags {
            e.tags.insert(k, v);
        }
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        arn: &str,
        keys: &[String],
    ) -> Result<(), BcmDataExportsError> {
        let e = self
            .exports
            .get_mut(arn)
            .ok_or_else(|| BcmDataExportsError::ExportNotFound {
                arn: arn.to_string(),
            })?;
        for k in keys {
            e.tags.remove(k);
        }
        Ok(())
    }

    pub fn list_tags(&self, arn: &str) -> Result<HashMap<String, String>, BcmDataExportsError> {
        Ok(self.get_export(arn)?.tags.clone())
    }
}
