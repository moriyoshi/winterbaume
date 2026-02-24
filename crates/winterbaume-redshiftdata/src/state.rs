use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct RedshiftDataState {
    pub statements: HashMap<String, Statement>,
    /// Databases available in the mock cluster catalogue.
    pub databases: Vec<String>,
    /// Schemas available in the mock cluster catalogue.
    pub schemas: Vec<String>,
    /// Table names available in the mock cluster catalogue.
    pub table_names: Vec<String>,
    /// Column metadata keyed by table name.
    pub table_columns: HashMap<String, Vec<(String, String)>>,
}

#[derive(Debug, Error)]
pub enum RedshiftDataError {
    #[error("Sql is required")]
    SqlRequired,

    #[error("Sqls is required")]
    SqlsRequired,

    #[error(
        "id must satisfy regex pattern: ^[a-z0-9]{{8}}(-[a-z0-9]{{4}}){{3}}-[a-z0-9]{{12}}(:\\d+)?$"
    )]
    InvalidStatementId,

    #[error("Query does not exist.")]
    StatementNotFound,
}

/// Check if an id looks like a valid UUID (lowercase hex with dashes).
/// Pattern: [a-z0-9]{8}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{12}
fn is_valid_statement_id(id: &str) -> bool {
    let parts: Vec<&str> = id.split(':').collect();
    let uuid_part = parts[0];
    let segments: Vec<&str> = uuid_part.split('-').collect();
    if segments.len() != 5 {
        return false;
    }
    let expected_lengths = [8, 4, 4, 4, 12];
    for (seg, &len) in segments.iter().zip(expected_lengths.iter()) {
        if seg.len() != len {
            return false;
        }
        if !seg
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_uppercase())
        {
            return false;
        }
    }
    true
}

impl RedshiftDataState {
    pub fn execute_statement(
        &mut self,
        sql: &str,
        database: &str,
        cluster_identifier: Option<&str>,
        workgroup_name: Option<&str>,
        db_user: Option<&str>,
        secret_arn: Option<&str>,
        parameters: Vec<StatementParameter>,
        result: crate::backend::StatementResult,
    ) -> Result<String, RedshiftDataError> {
        if sql.is_empty() {
            return Err(RedshiftDataError::SqlRequired);
        }

        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        let status = if result.error.is_some() {
            StatementStatus::Failed
        } else {
            StatementStatus::Finished
        };
        let result_rows = result.rows.len() as i64;
        let has_result_set = !result.columns.is_empty();

        let statement = Statement {
            id: id.clone(),
            sql: sql.to_string(),
            cluster_identifier: cluster_identifier.map(String::from),
            workgroup_name: workgroup_name.map(String::from),
            database: database.to_string(),
            db_user: db_user.map(String::from),
            secret_arn: secret_arn.map(String::from),
            status,
            created_at: now,
            updated_at: now,
            result_rows,
            result_size: 0,
            has_result_set,
            query_string: sql.to_string(),
            parameters,
            sqls: vec![],
            statement_name: None,
            is_batch: false,
            result_columns: result.columns,
            result_data: result.rows,
            error_message: result.error,
        };

        self.statements.insert(id.clone(), statement);
        Ok(id)
    }

    pub fn describe_statement(&self, id: &str) -> Result<&Statement, RedshiftDataError> {
        if !is_valid_statement_id(id) {
            return Err(RedshiftDataError::InvalidStatementId);
        }
        self.statements
            .get(id)
            .ok_or(RedshiftDataError::StatementNotFound)
    }

    pub fn cancel_statement(&mut self, id: &str) -> Result<bool, RedshiftDataError> {
        if !is_valid_statement_id(id) {
            return Err(RedshiftDataError::InvalidStatementId);
        }
        let statement = self
            .statements
            .get_mut(id)
            .ok_or(RedshiftDataError::StatementNotFound)?;

        match statement.status {
            StatementStatus::Submitted | StatementStatus::Started => {
                statement.status = StatementStatus::Aborted;
                statement.updated_at = Utc::now();
                Ok(true)
            }
            _ => {
                // Already finished, failed, or aborted - cancel returns false
                Ok(false)
            }
        }
    }

    pub fn list_statements(&self) -> Vec<&Statement> {
        let mut stmts: Vec<&Statement> = self.statements.values().collect();
        stmts.sort_by_key(|s| std::cmp::Reverse(s.created_at));
        stmts
    }

    pub fn batch_execute_statement(
        &mut self,
        sqls: Vec<String>,
        database: &str,
        cluster_identifier: Option<&str>,
        workgroup_name: Option<&str>,
        db_user: Option<&str>,
        secret_arn: Option<&str>,
        statement_name: Option<&str>,
        result: crate::backend::StatementResult,
    ) -> Result<String, RedshiftDataError> {
        if sqls.is_empty() {
            return Err(RedshiftDataError::SqlsRequired);
        }

        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        let query_string = sqls.first().cloned().unwrap_or_default();
        let status = if result.error.is_some() {
            StatementStatus::Failed
        } else {
            StatementStatus::Finished
        };

        let statement = Statement {
            id: id.clone(),
            sql: query_string.clone(),
            cluster_identifier: cluster_identifier.map(String::from),
            workgroup_name: workgroup_name.map(String::from),
            database: database.to_string(),
            db_user: db_user.map(String::from),
            secret_arn: secret_arn.map(String::from),
            status,
            created_at: now,
            updated_at: now,
            result_rows: 0,
            result_size: 0,
            has_result_set: false,
            query_string,
            parameters: vec![],
            sqls,
            statement_name: statement_name.map(String::from),
            is_batch: true,
            result_columns: vec![],
            result_data: vec![],
            error_message: result.error,
        };

        self.statements.insert(id.clone(), statement);
        Ok(id)
    }

    // --- Catalogue operations ---

    pub fn list_databases(&self) -> Vec<String> {
        if self.databases.is_empty() {
            vec!["dev".to_string(), "prod".to_string()]
        } else {
            self.databases.clone()
        }
    }

    pub fn list_schemas(&self) -> Vec<String> {
        if self.schemas.is_empty() {
            vec!["public".to_string(), "information_schema".to_string()]
        } else {
            self.schemas.clone()
        }
    }

    pub fn list_tables(&self) -> Vec<String> {
        self.table_names.clone()
    }

    pub fn describe_table(&self, table: Option<&str>) -> Vec<(String, String)> {
        if let Some(name) = table {
            self.table_columns.get(name).cloned().unwrap_or_default()
        } else {
            vec![]
        }
    }
}
