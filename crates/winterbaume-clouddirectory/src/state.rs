use std::collections::HashMap;

use crate::types::{Directory, Schema, SchemaType};

#[derive(Debug, Default)]
pub struct CloudDirectoryState {
    pub directories: HashMap<String, Directory>,
    pub schemas: HashMap<String, Schema>,
    /// Tags for resources (ARN -> tags)
    pub resource_tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, thiserror::Error)]
pub enum CloudDirectoryError {
    #[error("Directory with name {0} already exists.")]
    DirectoryAlreadyExists(String),
    #[error("Directory must be disabled before deleting.")]
    DirectoryNotDisabled,
    #[error("Schema {0} already exists.")]
    SchemaAlreadyExists(String),
    #[error("Directory {0} not found.")]
    DirectoryNotFound(String),
    #[error("Schema {0} not found.")]
    SchemaNotFound(String),
    #[error("Resource {0} not found.")]
    ResourceNotFound(String),
}

impl CloudDirectoryState {
    pub fn create_directory(
        &mut self,
        name: &str,
        schema_arn: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&Directory, CloudDirectoryError> {
        // Check that name is unique
        if self.directories.values().any(|d| d.name == name) {
            return Err(CloudDirectoryError::DirectoryAlreadyExists(
                name.to_string(),
            ));
        }

        let dir_id = uuid::Uuid::new_v4().to_string();
        let directory_arn =
            format!("arn:aws:clouddirectory:{region}:{account_id}:directory/{dir_id}");

        let now = chrono::Utc::now().timestamp() as f64;

        let dir = Directory {
            directory_arn: directory_arn.clone(),
            name: name.to_string(),
            schema_arn: schema_arn.to_string(),
            state: "ENABLED".to_string(),
            created_date_time: now,
            tags: HashMap::new(),
        };

        self.directories.insert(directory_arn.clone(), dir);
        Ok(self.directories.get(&directory_arn).unwrap())
    }

    pub fn delete_directory(&mut self, directory_arn: &str) -> Result<String, CloudDirectoryError> {
        let dir = self
            .directories
            .get_mut(directory_arn)
            .ok_or_else(|| CloudDirectoryError::DirectoryNotFound(directory_arn.to_string()))?;

        if dir.state == "ENABLED" {
            return Err(CloudDirectoryError::DirectoryNotDisabled);
        }

        dir.state = "DELETED".to_string();
        let arn = dir.directory_arn.clone();
        Ok(arn)
    }

    pub fn get_directory(&self, directory_arn: &str) -> Result<&Directory, CloudDirectoryError> {
        self.directories
            .get(directory_arn)
            .ok_or_else(|| CloudDirectoryError::DirectoryNotFound(directory_arn.to_string()))
    }

    pub fn list_directories(&self, state_filter: Option<&str>) -> Vec<&Directory> {
        self.directories
            .values()
            .filter(|d| {
                if let Some(s) = state_filter {
                    d.state == s
                } else {
                    true
                }
            })
            .collect()
    }

    pub fn create_schema(
        &mut self,
        name: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&Schema, CloudDirectoryError> {
        let schema_arn =
            format!("arn:aws:clouddirectory:{region}:{account_id}:schema/development/{name}");

        if self.schemas.contains_key(&schema_arn) {
            return Err(CloudDirectoryError::SchemaAlreadyExists(name.to_string()));
        }

        let schema = Schema {
            schema_arn: schema_arn.clone(),
            name: name.to_string(),
            type_: SchemaType::Development,
            published_arns: Vec::new(),
        };

        self.schemas.insert(schema_arn.clone(), schema);
        Ok(self.schemas.get(&schema_arn).unwrap())
    }

    pub fn delete_schema(&mut self, schema_arn: &str) -> Result<String, CloudDirectoryError> {
        self.schemas
            .remove(schema_arn)
            .ok_or_else(|| CloudDirectoryError::SchemaNotFound(schema_arn.to_string()))?;
        Ok(schema_arn.to_string())
    }

    pub fn apply_schema(
        &mut self,
        directory_arn: &str,
        published_schema_arn: &str,
    ) -> Result<(&str, &str), CloudDirectoryError> {
        // Verify directory exists
        if !self.directories.contains_key(directory_arn) {
            return Err(CloudDirectoryError::DirectoryNotFound(
                directory_arn.to_string(),
            ));
        }

        // Verify schema exists
        if !self.schemas.contains_key(published_schema_arn) {
            return Err(CloudDirectoryError::SchemaNotFound(
                published_schema_arn.to_string(),
            ));
        }

        // Update directory's schema
        let dir = self.directories.get_mut(directory_arn).unwrap();
        dir.schema_arn = published_schema_arn.to_string();

        let dir_ref = self.directories.get(directory_arn).unwrap();
        let schema_ref = self.schemas.get(&dir_ref.schema_arn).unwrap();
        // Return static references to owned strings via a different approach
        let _ = schema_ref;
        Ok(("", ""))
    }

    pub fn apply_schema_arns(
        &mut self,
        directory_arn: &str,
        published_schema_arn: &str,
    ) -> Result<(String, String), CloudDirectoryError> {
        // Verify directory exists
        if !self.directories.contains_key(directory_arn) {
            return Err(CloudDirectoryError::DirectoryNotFound(
                directory_arn.to_string(),
            ));
        }

        // Verify schema exists
        if !self.schemas.contains_key(published_schema_arn) {
            return Err(CloudDirectoryError::SchemaNotFound(
                published_schema_arn.to_string(),
            ));
        }

        // Update directory's schema
        let dir = self.directories.get_mut(directory_arn).unwrap();
        dir.schema_arn = published_schema_arn.to_string();

        Ok((published_schema_arn.to_string(), directory_arn.to_string()))
    }

    pub fn list_development_schema_arns(&self) -> Vec<&str> {
        self.schemas
            .values()
            .filter(|s| s.type_ == SchemaType::Development)
            .map(|s| s.schema_arn.as_str())
            .collect()
    }

    pub fn list_published_schema_arns(&self) -> Vec<String> {
        self.schemas
            .values()
            .filter(|s| s.type_ == SchemaType::Published)
            .map(|s| s.schema_arn.clone())
            .collect()
    }

    pub fn publish_schema(
        &mut self,
        development_schema_arn: &str,
        version: &str,
        account_id: &str,
        region: &str,
    ) -> Result<String, CloudDirectoryError> {
        let dev_schema = self.schemas.get(development_schema_arn).ok_or_else(|| {
            CloudDirectoryError::SchemaNotFound(development_schema_arn.to_string())
        })?;

        // Extract name from dev schema ARN: .../schema/development/{name}
        let name = dev_schema.name.clone();
        let published_arn = format!(
            "arn:aws:clouddirectory:{region}:{account_id}:schema/published/{name}/{version}"
        );

        let published_schema = Schema {
            schema_arn: published_arn.clone(),
            name: name.clone(),
            type_: SchemaType::Published,
            published_arns: Vec::new(),
        };

        // Track the published ARN on the dev schema
        self.schemas
            .get_mut(development_schema_arn)
            .unwrap()
            .published_arns
            .push(published_arn.clone());

        self.schemas.insert(published_arn.clone(), published_schema);
        Ok(published_arn)
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<Vec<(String, String)>, CloudDirectoryError> {
        // Resources can be directories or schemas - find them
        let tags = if let Some(dir) = self.directories.get(resource_arn) {
            dir.tags
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect()
        } else if self.schemas.contains_key(resource_arn) {
            self.resource_tags
                .get(resource_arn)
                .map(|t| t.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                .unwrap_or_default()
        } else {
            return Err(CloudDirectoryError::ResourceNotFound(
                resource_arn.to_string(),
            ));
        };
        Ok(tags)
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: Vec<(String, String)>,
    ) -> Result<(), CloudDirectoryError> {
        if let Some(dir) = self.directories.get_mut(resource_arn) {
            for (k, v) in tags {
                dir.tags.insert(k, v);
            }
        } else if self.schemas.contains_key(resource_arn) {
            let entry = self
                .resource_tags
                .entry(resource_arn.to_string())
                .or_default();
            for (k, v) in tags {
                entry.insert(k, v);
            }
        } else {
            return Err(CloudDirectoryError::ResourceNotFound(
                resource_arn.to_string(),
            ));
        }
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: Vec<String>,
    ) -> Result<(), CloudDirectoryError> {
        if let Some(dir) = self.directories.get_mut(resource_arn) {
            for k in &tag_keys {
                dir.tags.remove(k);
            }
        } else if self.schemas.contains_key(resource_arn) {
            if let Some(tags) = self.resource_tags.get_mut(resource_arn) {
                for k in &tag_keys {
                    tags.remove(k);
                }
            }
        } else {
            return Err(CloudDirectoryError::ResourceNotFound(
                resource_arn.to_string(),
            ));
        }
        Ok(())
    }
}
