use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;
use uuid::Uuid;

use crate::types::*;

#[derive(Debug, Default)]
pub struct AthenaState {
    pub work_groups: HashMap<String, WorkGroup>,
    pub capacity_reservations: HashMap<String, CapacityReservationData>,
    pub data_catalogs: HashMap<String, DataCatalogData>,
    pub named_queries: HashMap<String, NamedQueryData>,
    /// Key: (work_group_name, statement_name)
    pub prepared_statements: HashMap<(String, String), PreparedStatementData>,
    pub query_executions: HashMap<String, QueryExecutionData>,
    /// Resource ARN -> tags
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Error)]
pub enum AthenaError {
    #[error("WorkGroup {0} already exists.")]
    WorkGroupAlreadyExists(String),
    #[error("WorkGroup {0} not found.")]
    WorkGroupNotFound(String),
    #[error("The primary workgroup cannot be deleted.")]
    PrimaryWorkGroupCannotBeDeleted,
    #[error("Capacity reservation {0} already exists.")]
    CapacityReservationAlreadyExists(String),
    #[error("Capacity reservation {0} not found.")]
    CapacityReservationNotFound(String),
    #[error("Data catalog {0} already exists.")]
    DataCatalogAlreadyExists(String),
    #[error("Data catalog {0} not found.")]
    DataCatalogNotFound(String),
    #[error("Named query {0} not found.")]
    NamedQueryNotFound(String),
    #[error("Prepared statement {statement_name} already exists in work group {work_group}.")]
    PreparedStatementAlreadyExists {
        statement_name: String,
        work_group: String,
    },
    #[error("Prepared statement {statement_name} not found in work group {work_group}.")]
    PreparedStatementNotFound {
        statement_name: String,
        work_group: String,
    },
    #[error("Query execution {0} not found.")]
    QueryExecutionNotFound(String),
    #[error("Athena Resource, {0} Does Not Exist")]
    ResourceNotFound(String),
}

impl AthenaState {
    // ---- WorkGroup ----

    pub fn create_work_group(
        &mut self,
        name: &str,
        description: &str,
        output_location: &str,
        enforce: bool,
    ) -> Result<(), AthenaError> {
        if self.work_groups.contains_key(name) {
            return Err(AthenaError::WorkGroupAlreadyExists(name.to_string()));
        }

        let wg = WorkGroup {
            name: name.to_string(),
            state: "ENABLED".to_string(),
            description: description.to_string(),
            creation_time: Utc::now(),
            output_location: output_location.to_string(),
            enforce_work_group_configuration: enforce,
            tags: HashMap::new(),
        };

        self.work_groups.insert(name.to_string(), wg);
        Ok(())
    }

    pub fn get_work_group(&self, name: &str) -> Result<&WorkGroup, AthenaError> {
        self.work_groups
            .get(name)
            .ok_or_else(|| AthenaError::WorkGroupNotFound(name.to_string()))
    }

    pub fn delete_work_group(&mut self, name: &str) -> Result<(), AthenaError> {
        if name == "primary" {
            return Err(AthenaError::PrimaryWorkGroupCannotBeDeleted);
        }
        if self.work_groups.remove(name).is_none() {
            return Err(AthenaError::WorkGroupNotFound(name.to_string()));
        }
        Ok(())
    }

    pub fn list_work_groups(&self) -> Vec<&WorkGroup> {
        self.work_groups.values().collect()
    }

    // ---- CapacityReservation ----

    pub fn create_capacity_reservation(
        &mut self,
        name: &str,
        target_dpus: i32,
        tags: HashMap<String, String>,
    ) -> Result<(), AthenaError> {
        if self.capacity_reservations.contains_key(name) {
            return Err(AthenaError::CapacityReservationAlreadyExists(
                name.to_string(),
            ));
        }
        let cr = CapacityReservationData {
            name: name.to_string(),
            target_dpus,
            allocated_dpus: target_dpus,
            status: "ACTIVE".to_string(),
            creation_time: Utc::now(),
            tags,
        };
        self.capacity_reservations.insert(name.to_string(), cr);
        Ok(())
    }

    pub fn get_capacity_reservation(
        &self,
        name: &str,
    ) -> Result<&CapacityReservationData, AthenaError> {
        self.capacity_reservations
            .get(name)
            .ok_or_else(|| AthenaError::CapacityReservationNotFound(name.to_string()))
    }

    pub fn list_capacity_reservations(&self) -> Vec<&CapacityReservationData> {
        self.capacity_reservations.values().collect()
    }

    pub fn update_capacity_reservation(
        &mut self,
        name: &str,
        target_dpus: i32,
    ) -> Result<(), AthenaError> {
        let cr = self
            .capacity_reservations
            .get_mut(name)
            .ok_or_else(|| AthenaError::CapacityReservationNotFound(name.to_string()))?;
        cr.target_dpus = target_dpus;
        cr.allocated_dpus = target_dpus;
        Ok(())
    }

    // ---- DataCatalog ----

    pub fn create_data_catalog(
        &mut self,
        name: &str,
        catalog_type: &str,
        description: &str,
        parameters: HashMap<String, String>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&DataCatalogData, AthenaError> {
        if self.data_catalogs.contains_key(name) {
            return Err(AthenaError::DataCatalogAlreadyExists(name.to_string()));
        }

        // Store tags under the resource ARN
        if !tags.is_empty() {
            let arn = format!("arn:aws:athena:{region}:{account_id}:datacatalog/{name}");
            self.tags.insert(arn, tags.clone());
        }

        let dc = DataCatalogData {
            name: name.to_string(),
            catalog_type: catalog_type.to_string(),
            description: description.to_string(),
            parameters,
            tags,
        };
        self.data_catalogs.insert(name.to_string(), dc);
        Ok(self.data_catalogs.get(name).unwrap())
    }

    pub fn get_data_catalog(&self, name: &str) -> Result<&DataCatalogData, AthenaError> {
        self.data_catalogs
            .get(name)
            .ok_or_else(|| AthenaError::DataCatalogNotFound(name.to_string()))
    }

    pub fn list_data_catalogs(&self) -> Vec<&DataCatalogData> {
        self.data_catalogs.values().collect()
    }

    // ---- NamedQuery ----

    pub fn create_named_query(
        &mut self,
        name: &str,
        description: &str,
        database: &str,
        query_string: &str,
        work_group: &str,
    ) -> String {
        let id = Uuid::new_v4().to_string();
        let nq = NamedQueryData {
            id: id.clone(),
            name: name.to_string(),
            description: description.to_string(),
            database: database.to_string(),
            query_string: query_string.to_string(),
            work_group: work_group.to_string(),
        };
        self.named_queries.insert(id.clone(), nq);
        id
    }

    pub fn get_named_query(&self, id: &str) -> Result<&NamedQueryData, AthenaError> {
        self.named_queries
            .get(id)
            .ok_or_else(|| AthenaError::NamedQueryNotFound(id.to_string()))
    }

    pub fn list_named_queries(&self) -> Vec<String> {
        self.named_queries.keys().cloned().collect()
    }

    // ---- PreparedStatement ----

    pub fn create_prepared_statement(
        &mut self,
        statement_name: &str,
        work_group: &str,
        query_statement: &str,
        description: &str,
    ) -> Result<(), AthenaError> {
        let key = (work_group.to_string(), statement_name.to_string());
        if self.prepared_statements.contains_key(&key) {
            return Err(AthenaError::PreparedStatementAlreadyExists {
                statement_name: statement_name.to_string(),
                work_group: work_group.to_string(),
            });
        }
        let ps = PreparedStatementData {
            statement_name: statement_name.to_string(),
            work_group_name: work_group.to_string(),
            query_statement: query_statement.to_string(),
            description: description.to_string(),
            last_modified_time: Utc::now(),
        };
        self.prepared_statements.insert(key, ps);
        Ok(())
    }

    pub fn get_prepared_statement(
        &self,
        statement_name: &str,
        work_group: &str,
    ) -> Result<&PreparedStatementData, AthenaError> {
        let key = (work_group.to_string(), statement_name.to_string());
        self.prepared_statements
            .get(&key)
            .ok_or_else(|| AthenaError::PreparedStatementNotFound {
                statement_name: statement_name.to_string(),
                work_group: work_group.to_string(),
            })
    }

    // ---- QueryExecution ----

    pub fn start_query_execution(
        &mut self,
        query: &str,
        work_group: &str,
        database: Option<&str>,
        catalog: Option<&str>,
        output_location: Option<&str>,
        result: crate::backend::QueryResult,
    ) -> String {
        let id = Uuid::new_v4().to_string();
        let status = if result.error.is_some() {
            "FAILED".to_string()
        } else {
            "SUCCEEDED".to_string()
        };
        let qe = QueryExecutionData {
            query_execution_id: id.clone(),
            query: query.to_string(),
            work_group: work_group.to_string(),
            database: database.map(|s| s.to_string()),
            catalog: catalog.map(|s| s.to_string()),
            output_location: output_location.map(|s| s.to_string()),
            status,
            submission_time: Utc::now(),
            completion_time: Some(Utc::now()),
            result_columns: result.columns,
            result_rows: result.rows,
            error_message: result.error,
        };
        self.query_executions.insert(id.clone(), qe);
        id
    }

    pub fn get_query_execution(&self, id: &str) -> Result<&QueryExecutionData, AthenaError> {
        self.query_executions
            .get(id)
            .ok_or_else(|| AthenaError::QueryExecutionNotFound(id.to_string()))
    }

    pub fn stop_query_execution(&mut self, id: &str) -> Result<(), AthenaError> {
        let qe = self
            .query_executions
            .get_mut(id)
            .ok_or_else(|| AthenaError::QueryExecutionNotFound(id.to_string()))?;
        qe.status = "CANCELLED".to_string();
        qe.completion_time = Some(Utc::now());
        Ok(())
    }

    pub fn list_query_executions(&self) -> Vec<String> {
        self.query_executions.keys().cloned().collect()
    }

    // ---- Tags ----

    pub fn list_tags_for_resource(&self, resource_arn: &str) -> HashMap<String, String> {
        self.tags.get(resource_arn).cloned().unwrap_or_default()
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), AthenaError> {
        if !self.resource_exists(resource_arn) {
            return Err(AthenaError::ResourceNotFound(resource_arn.to_string()));
        }
        let entry = self.tags.entry(resource_arn.to_string()).or_default();
        entry.extend(tags);
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), AthenaError> {
        if !self.resource_exists(resource_arn) {
            return Err(AthenaError::ResourceNotFound(resource_arn.to_string()));
        }
        if let Some(entry) = self.tags.get_mut(resource_arn) {
            for key in tag_keys {
                entry.remove(key);
            }
        }
        Ok(())
    }

    /// Check if a resource ARN corresponds to a known resource.
    pub fn resource_exists(&self, resource_arn: &str) -> bool {
        // Check if it's a workgroup ARN
        if resource_arn.contains(":workgroup/")
            && let Some(name) = resource_arn.split(":workgroup/").last()
        {
            // "primary" workgroup always exists implicitly
            return name == "primary" || self.work_groups.contains_key(name);
        }
        // Check if it's a datacatalog ARN
        if resource_arn.contains(":datacatalog/")
            && let Some(name) = resource_arn.split(":datacatalog/").last()
        {
            return self.data_catalogs.contains_key(name);
        }
        // Check tags map directly
        if self.tags.contains_key(resource_arn) {
            return true;
        }
        // If the ARN doesn't match a known pattern, it's not a valid Athena resource
        false
    }
}
