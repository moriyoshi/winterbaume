use std::collections::HashMap;

use chrono::Utc;
use uuid::Uuid;

use crate::types::{EncryptionConfig, Namespace, StorageClassConfig, Table, TableBucket};

#[derive(Debug, Default)]
pub struct S3TablesState {
    /// Keyed by table bucket ARN
    pub table_buckets: HashMap<String, TableBucket>,
    /// Keyed by (table_bucket_arn, namespace_name)
    pub namespaces: HashMap<(String, String), Namespace>,
    /// Keyed by (table_bucket_arn, namespace_name, table_name)
    pub tables: HashMap<(String, String, String), Table>,
    /// Tags keyed by resource ARN
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, thiserror::Error)]
pub enum S3TablesError {
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Conflict(String),
}

impl S3TablesState {
    fn not_found(msg: impl Into<String>) -> S3TablesError {
        S3TablesError::NotFound(msg.into())
    }

    fn conflict(msg: impl Into<String>) -> S3TablesError {
        S3TablesError::Conflict(msg.into())
    }

    // --- Table Bucket operations ---

    pub fn create_table_bucket(
        &mut self,
        name: &str,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<String, S3TablesError> {
        let arn = format!("arn:aws:s3tables:{region}:{account_id}:bucket/{name}");
        if self.table_buckets.contains_key(&arn) {
            return Err(Self::conflict(format!(
                "Table bucket {name} already exists."
            )));
        }
        let bucket = TableBucket {
            name: name.to_string(),
            arn: arn.clone(),
            owner_account_id: account_id.to_string(),
            created_at: Utc::now(),
            tags: tags.clone(),
            encryption: None,
            maintenance_config: HashMap::new(),
            metrics_config: None,
            policy: None,
            storage_class: None,
            replication_config: None,
        };
        self.table_buckets.insert(arn.clone(), bucket);
        self.tags.insert(arn.clone(), tags);
        Ok(arn)
    }

    pub fn get_table_bucket(&self, arn: &str) -> Result<&TableBucket, S3TablesError> {
        self.table_buckets
            .get(arn)
            .ok_or_else(|| Self::not_found(format!("Table bucket {arn} not found.")))
    }

    pub fn delete_table_bucket(&mut self, arn: &str) -> Result<(), S3TablesError> {
        if self.table_buckets.remove(arn).is_none() {
            return Err(Self::not_found(format!("Table bucket {arn} not found.")));
        }
        self.tags.remove(arn);
        Ok(())
    }

    pub fn list_table_buckets(&self, prefix: Option<&str>) -> Vec<&TableBucket> {
        let mut buckets: Vec<&TableBucket> = self
            .table_buckets
            .values()
            .filter(|b| {
                if let Some(p) = prefix {
                    b.name.starts_with(p)
                } else {
                    true
                }
            })
            .collect();
        buckets.sort_by(|a, b| a.name.cmp(&b.name));
        buckets
    }

    // --- Namespace operations ---

    pub fn create_namespace(
        &mut self,
        table_bucket_arn: &str,
        namespace_parts: Vec<String>,
        account_id: &str,
    ) -> Result<(), S3TablesError> {
        if !self.table_buckets.contains_key(table_bucket_arn) {
            return Err(Self::not_found(format!(
                "Table bucket {table_bucket_arn} not found."
            )));
        }
        let ns_name = namespace_parts.join(".");
        let key = (table_bucket_arn.to_string(), ns_name.clone());
        if self.namespaces.contains_key(&key) {
            return Err(Self::conflict(format!(
                "Namespace {ns_name} already exists."
            )));
        }
        let ns = Namespace {
            table_bucket_arn: table_bucket_arn.to_string(),
            namespace: namespace_parts,
            name: ns_name.clone(),
            owner_account_id: account_id.to_string(),
            created_at: Utc::now(),
            created_by: account_id.to_string(),
            tags: HashMap::new(),
        };
        self.namespaces.insert(key, ns);
        Ok(())
    }

    pub fn get_namespace(
        &self,
        table_bucket_arn: &str,
        namespace: &str,
    ) -> Result<&Namespace, S3TablesError> {
        let key = (table_bucket_arn.to_string(), namespace.to_string());
        self.namespaces
            .get(&key)
            .ok_or_else(|| Self::not_found(format!("Namespace {namespace} not found.")))
    }

    pub fn delete_namespace(
        &mut self,
        table_bucket_arn: &str,
        namespace: &str,
    ) -> Result<(), S3TablesError> {
        let key = (table_bucket_arn.to_string(), namespace.to_string());
        if self.namespaces.remove(&key).is_none() {
            return Err(Self::not_found(format!("Namespace {namespace} not found.")));
        }
        Ok(())
    }

    pub fn list_namespaces(&self, table_bucket_arn: &str, prefix: Option<&str>) -> Vec<&Namespace> {
        let mut ns: Vec<&Namespace> = self
            .namespaces
            .iter()
            .filter(|((arn, ns_name), _)| {
                arn == table_bucket_arn && prefix.is_none_or(|p| ns_name.starts_with(p))
            })
            .map(|(_, v)| v)
            .collect();
        ns.sort_by(|a, b| a.name.cmp(&b.name));
        ns
    }

    // --- Table operations ---

    #[allow(clippy::too_many_arguments)]
    pub fn create_table(
        &mut self,
        table_bucket_arn: &str,
        namespace: &str,
        name: &str,
        format: &str,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<(String, String), S3TablesError> {
        // Verify namespace exists
        let ns_key = (table_bucket_arn.to_string(), namespace.to_string());
        if !self.namespaces.contains_key(&ns_key) {
            return Err(Self::not_found(format!("Namespace {namespace} not found.")));
        }
        let tbl_key = (
            table_bucket_arn.to_string(),
            namespace.to_string(),
            name.to_string(),
        );
        if self.tables.contains_key(&tbl_key) {
            return Err(Self::conflict(format!("Table {name} already exists.")));
        }

        // Extract bucket name from ARN for the warehouse location
        let bucket_name = table_bucket_arn
            .rsplit('/')
            .next()
            .unwrap_or(table_bucket_arn);

        let table_arn = format!(
            "arn:aws:s3tables:{region}:{account_id}:bucket/{bucket_name}/table/{namespace}/{name}"
        );
        let version_token = Uuid::new_v4().to_string();
        let warehouse_location = format!("s3://{bucket_name}/{namespace}/{name}/");

        let now = Utc::now();
        let table = Table {
            table_bucket_arn: table_bucket_arn.to_string(),
            namespace: namespace.to_string(),
            name: name.to_string(),
            arn: table_arn.clone(),
            version_token: version_token.clone(),
            format: format.to_string(),
            warehouse_location,
            owner_account_id: account_id.to_string(),
            created_at: now,
            created_by: account_id.to_string(),
            modified_at: now,
            modified_by: account_id.to_string(),
            tags: tags.clone(),
            policy: None,
            metadata_location: None,
            maintenance_config: HashMap::new(),
            storage_class: None,
            replication_config: None,
            record_expiration_config: None,
        };
        self.tables.insert(tbl_key, table);
        self.tags.insert(table_arn.clone(), tags);
        Ok((table_arn, version_token))
    }

    pub fn get_table(
        &self,
        table_bucket_arn: &str,
        namespace: &str,
        name: &str,
    ) -> Result<&Table, S3TablesError> {
        let key = (
            table_bucket_arn.to_string(),
            namespace.to_string(),
            name.to_string(),
        );
        self.tables
            .get(&key)
            .ok_or_else(|| Self::not_found(format!("Table {name} not found.")))
    }

    pub fn get_table_by_arn(&self, arn: &str) -> Result<&Table, S3TablesError> {
        self.tables
            .values()
            .find(|t| t.arn == arn)
            .ok_or_else(|| Self::not_found(format!("Table {arn} not found.")))
    }

    pub fn delete_table(
        &mut self,
        table_bucket_arn: &str,
        namespace: &str,
        name: &str,
    ) -> Result<(), S3TablesError> {
        let key = (
            table_bucket_arn.to_string(),
            namespace.to_string(),
            name.to_string(),
        );
        let table = self
            .tables
            .remove(&key)
            .ok_or_else(|| Self::not_found(format!("Table {name} not found.")))?;
        self.tags.remove(&table.arn);
        Ok(())
    }

    pub fn list_tables(
        &self,
        table_bucket_arn: &str,
        namespace: Option<&str>,
        prefix: Option<&str>,
    ) -> Vec<&Table> {
        let mut tables: Vec<&Table> = self
            .tables
            .iter()
            .filter(|((arn, ns, tbl_name), _)| {
                arn == table_bucket_arn
                    && namespace.is_none_or(|n| ns == n)
                    && prefix.is_none_or(|p| tbl_name.starts_with(p))
            })
            .map(|(_, v)| v)
            .collect();
        tables.sort_by(|a, b| a.name.cmp(&b.name));
        tables
    }

    // --- Table Bucket extended operations ---

    pub fn get_table_bucket_mut(&mut self, arn: &str) -> Result<&mut TableBucket, S3TablesError> {
        self.table_buckets
            .get_mut(arn)
            .ok_or_else(|| Self::not_found(format!("Table bucket {arn} not found.")))
    }

    pub fn put_table_bucket_encryption(
        &mut self,
        arn: &str,
        sse_algorithm: String,
        kms_key_arn: Option<String>,
    ) -> Result<(), S3TablesError> {
        let bucket = self.get_table_bucket_mut(arn)?;
        bucket.encryption = Some(EncryptionConfig {
            sse_algorithm,
            kms_key_arn,
        });
        Ok(())
    }

    pub fn delete_table_bucket_encryption(&mut self, arn: &str) -> Result<(), S3TablesError> {
        let bucket = self.get_table_bucket_mut(arn)?;
        bucket.encryption = None;
        Ok(())
    }

    pub fn put_table_bucket_policy(
        &mut self,
        arn: &str,
        policy: String,
    ) -> Result<(), S3TablesError> {
        let bucket = self.get_table_bucket_mut(arn)?;
        bucket.policy = Some(policy);
        Ok(())
    }

    pub fn delete_table_bucket_policy(&mut self, arn: &str) -> Result<(), S3TablesError> {
        let bucket = self.get_table_bucket_mut(arn)?;
        bucket.policy = None;
        Ok(())
    }

    pub fn put_table_bucket_storage_class(
        &mut self,
        arn: &str,
        storage_class: String,
    ) -> Result<(), S3TablesError> {
        let bucket = self.get_table_bucket_mut(arn)?;
        bucket.storage_class = Some(StorageClassConfig { storage_class });
        Ok(())
    }

    pub fn put_table_bucket_maintenance(
        &mut self,
        arn: &str,
        type_key: String,
        value_json: String,
    ) -> Result<(), S3TablesError> {
        let bucket = self.get_table_bucket_mut(arn)?;
        bucket.maintenance_config.insert(type_key, value_json);
        Ok(())
    }

    pub fn put_table_bucket_metrics(
        &mut self,
        arn: &str,
        metrics_json: String,
    ) -> Result<(), S3TablesError> {
        let bucket = self.get_table_bucket_mut(arn)?;
        bucket.metrics_config = Some(metrics_json);
        Ok(())
    }

    pub fn delete_table_bucket_metrics(&mut self, arn: &str) -> Result<(), S3TablesError> {
        let bucket = self.get_table_bucket_mut(arn)?;
        bucket.metrics_config = None;
        Ok(())
    }

    pub fn put_table_bucket_replication(
        &mut self,
        arn: &str,
        config_json: String,
    ) -> Result<(), S3TablesError> {
        let bucket = self.get_table_bucket_mut(arn)?;
        bucket.replication_config = Some(config_json);
        Ok(())
    }

    pub fn delete_table_bucket_replication(&mut self, arn: &str) -> Result<(), S3TablesError> {
        let bucket = self.get_table_bucket_mut(arn)?;
        bucket.replication_config = None;
        Ok(())
    }

    // --- Table extended operations ---

    fn get_table_mut(
        &mut self,
        table_bucket_arn: &str,
        namespace: &str,
        name: &str,
    ) -> Result<&mut Table, S3TablesError> {
        let key = (
            table_bucket_arn.to_string(),
            namespace.to_string(),
            name.to_string(),
        );
        self.tables
            .get_mut(&key)
            .ok_or_else(|| Self::not_found(format!("Table {name} not found.")))
    }

    pub fn put_table_policy(
        &mut self,
        table_bucket_arn: &str,
        namespace: &str,
        name: &str,
        policy: String,
    ) -> Result<(), S3TablesError> {
        let t = self.get_table_mut(table_bucket_arn, namespace, name)?;
        t.policy = Some(policy);
        Ok(())
    }

    pub fn delete_table_policy(
        &mut self,
        table_bucket_arn: &str,
        namespace: &str,
        name: &str,
    ) -> Result<(), S3TablesError> {
        let t = self.get_table_mut(table_bucket_arn, namespace, name)?;
        t.policy = None;
        Ok(())
    }

    pub fn put_table_maintenance(
        &mut self,
        table_bucket_arn: &str,
        namespace: &str,
        name: &str,
        type_key: String,
        value_json: String,
    ) -> Result<(), S3TablesError> {
        let t = self.get_table_mut(table_bucket_arn, namespace, name)?;
        t.maintenance_config.insert(type_key, value_json);
        Ok(())
    }

    pub fn update_table_metadata_location(
        &mut self,
        table_bucket_arn: &str,
        namespace: &str,
        name: &str,
        metadata_location: String,
        version_token: &str,
    ) -> Result<String, S3TablesError> {
        let t = self.get_table_mut(table_bucket_arn, namespace, name)?;
        if t.version_token != version_token {
            return Err(S3TablesError::Conflict(
                "Version token mismatch.".to_string(),
            ));
        }
        t.metadata_location = Some(metadata_location);
        t.version_token = uuid::Uuid::new_v4().to_string();
        t.modified_at = Utc::now();
        Ok(t.version_token.clone())
    }

    pub fn rename_table(
        &mut self,
        table_bucket_arn: &str,
        namespace: &str,
        name: &str,
        new_name: Option<String>,
        new_namespace: Option<String>,
        version_token: Option<String>,
    ) -> Result<(), S3TablesError> {
        let key = (
            table_bucket_arn.to_string(),
            namespace.to_string(),
            name.to_string(),
        );
        let mut table = self
            .tables
            .remove(&key)
            .ok_or_else(|| Self::not_found(format!("Table {name} not found.")))?;

        if let Some(ref vt) = version_token
            && &table.version_token != vt
        {
            // put back
            self.tables.insert(key, table);
            return Err(S3TablesError::Conflict(
                "Version token mismatch.".to_string(),
            ));
        }

        let target_namespace = new_namespace.unwrap_or_else(|| namespace.to_string());
        let target_name = new_name.unwrap_or_else(|| name.to_string());

        // Verify new namespace exists if changed
        if target_namespace != namespace {
            let ns_key = (table_bucket_arn.to_string(), target_namespace.clone());
            if !self.namespaces.contains_key(&ns_key) {
                self.tables.insert(key, table);
                return Err(Self::not_found(format!(
                    "Namespace {target_namespace} not found."
                )));
            }
        }

        let new_key = (
            table_bucket_arn.to_string(),
            target_namespace.clone(),
            target_name.clone(),
        );
        if self.tables.contains_key(&new_key) {
            self.tables.insert(key, table);
            return Err(Self::conflict(format!(
                "Table {target_name} already exists."
            )));
        }

        table.namespace = target_namespace;
        table.name = target_name.clone();
        table.modified_at = Utc::now();
        table.version_token = uuid::Uuid::new_v4().to_string();
        self.tables.insert(new_key, table);
        Ok(())
    }

    pub fn put_table_storage_class(
        &mut self,
        table_bucket_arn: &str,
        namespace: &str,
        name: &str,
        storage_class: String,
    ) -> Result<(), S3TablesError> {
        let t = self.get_table_mut(table_bucket_arn, namespace, name)?;
        t.storage_class = Some(StorageClassConfig { storage_class });
        Ok(())
    }

    pub fn put_table_replication(
        &mut self,
        table_arn: &str,
        config_json: String,
    ) -> Result<(), S3TablesError> {
        let t = self
            .tables
            .values_mut()
            .find(|t| t.arn == table_arn)
            .ok_or_else(|| Self::not_found(format!("Table {table_arn} not found.")))?;
        t.replication_config = Some(config_json);
        Ok(())
    }

    pub fn delete_table_replication(&mut self, table_arn: &str) -> Result<(), S3TablesError> {
        let t = self
            .tables
            .values_mut()
            .find(|t| t.arn == table_arn)
            .ok_or_else(|| Self::not_found(format!("Table {table_arn} not found.")))?;
        t.replication_config = None;
        Ok(())
    }

    pub fn put_table_record_expiration(
        &mut self,
        table_arn: &str,
        config_json: String,
    ) -> Result<(), S3TablesError> {
        let t = self
            .tables
            .values_mut()
            .find(|t| t.arn == table_arn)
            .ok_or_else(|| Self::not_found(format!("Table {table_arn} not found.")))?;
        t.record_expiration_config = Some(config_json);
        Ok(())
    }

    pub fn get_table_by_arn_ref(&self, arn: &str) -> Result<&Table, S3TablesError> {
        self.tables
            .values()
            .find(|t| t.arn == arn)
            .ok_or_else(|| Self::not_found(format!("Table {arn} not found.")))
    }

    // --- Tag operations ---

    pub fn list_tags(&self, resource_arn: &str) -> HashMap<String, String> {
        self.tags.get(resource_arn).cloned().unwrap_or_default()
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        new_tags: HashMap<String, String>,
    ) -> Result<(), S3TablesError> {
        if !self.table_buckets.contains_key(resource_arn)
            && !self.tables.values().any(|t| t.arn == resource_arn)
        {
            return Err(Self::not_found(format!(
                "Resource {resource_arn} not found."
            )));
        }
        let tags = self.tags.entry(resource_arn.to_string()).or_default();
        tags.extend(new_tags);
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        keys: &[String],
    ) -> Result<(), S3TablesError> {
        if !self.table_buckets.contains_key(resource_arn)
            && !self.tables.values().any(|t| t.arn == resource_arn)
        {
            return Err(Self::not_found(format!(
                "Resource {resource_arn} not found."
            )));
        }
        if let Some(tags) = self.tags.get_mut(resource_arn) {
            for k in keys {
                tags.remove(k);
            }
        }
        Ok(())
    }
}
