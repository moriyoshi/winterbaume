//! Serde-compatible view types for Redshift Data state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::RedshiftDataService;
use crate::state::RedshiftDataState;
use crate::types::StatementStatus;

/// Serializable view of a statement parameter.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatementParameterView {
    pub name: String,
    pub value: String,
}

/// Serializable view of a statement.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatementView {
    pub id: String,
    pub sql: String,
    pub cluster_identifier: Option<String>,
    pub workgroup_name: Option<String>,
    pub database: String,
    pub db_user: Option<String>,
    pub secret_arn: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub result_rows: i64,
    pub result_size: i64,
    pub has_result_set: bool,
    pub query_string: String,
    #[serde(default)]
    pub parameters: Vec<StatementParameterView>,
    #[serde(default)]
    pub result_columns: Vec<(String, String)>,
    #[serde(default)]
    pub result_data: Vec<Vec<Option<String>>>,
    #[serde(default)]
    pub error_message: Option<String>,
}

/// Serializable view of the entire Redshift Data state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RedshiftDataStateView {
    #[serde(default)]
    pub statements: HashMap<String, StatementView>,
    #[serde(default)]
    pub databases: Vec<String>,
    #[serde(default)]
    pub schemas: Vec<String>,
    #[serde(default)]
    pub table_names: Vec<String>,
    #[serde(default)]
    pub table_columns: HashMap<String, Vec<(String, String)>>,
}

// --- From internal types to view types ---

fn dt_to_string(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

fn parse_dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

impl From<&crate::types::StatementParameter> for StatementParameterView {
    fn from(p: &crate::types::StatementParameter) -> Self {
        StatementParameterView {
            name: p.name.clone(),
            value: p.value.clone(),
        }
    }
}

impl From<&crate::types::Statement> for StatementView {
    fn from(s: &crate::types::Statement) -> Self {
        StatementView {
            id: s.id.clone(),
            sql: s.sql.clone(),
            cluster_identifier: s.cluster_identifier.clone(),
            workgroup_name: s.workgroup_name.clone(),
            database: s.database.clone(),
            db_user: s.db_user.clone(),
            secret_arn: s.secret_arn.clone(),
            status: s.status.as_str().to_string(),
            created_at: dt_to_string(&s.created_at),
            updated_at: dt_to_string(&s.updated_at),
            result_rows: s.result_rows,
            result_size: s.result_size,
            has_result_set: s.has_result_set,
            query_string: s.query_string.clone(),
            parameters: s
                .parameters
                .iter()
                .map(StatementParameterView::from)
                .collect(),
            result_columns: s.result_columns.clone(),
            result_data: s.result_data.clone(),
            error_message: s.error_message.clone(),
        }
    }
}

impl From<&RedshiftDataState> for RedshiftDataStateView {
    fn from(state: &RedshiftDataState) -> Self {
        RedshiftDataStateView {
            statements: state
                .statements
                .iter()
                .map(|(k, v)| (k.clone(), StatementView::from(v)))
                .collect(),
            databases: state.databases.clone(),
            schemas: state.schemas.clone(),
            table_names: state.table_names.clone(),
            table_columns: state.table_columns.clone(),
        }
    }
}

// --- From view types to internal types ---

fn parse_status(s: &str) -> StatementStatus {
    match s {
        "SUBMITTED" => StatementStatus::Submitted,
        "STARTED" => StatementStatus::Started,
        "FINISHED" => StatementStatus::Finished,
        "FAILED" => StatementStatus::Failed,
        "ABORTED" => StatementStatus::Aborted,
        _ => StatementStatus::Finished,
    }
}

impl From<StatementParameterView> for crate::types::StatementParameter {
    fn from(v: StatementParameterView) -> Self {
        crate::types::StatementParameter {
            name: v.name,
            value: v.value,
        }
    }
}

impl From<StatementView> for crate::types::Statement {
    fn from(v: StatementView) -> Self {
        crate::types::Statement {
            id: v.id,
            sql: v.sql,
            cluster_identifier: v.cluster_identifier,
            workgroup_name: v.workgroup_name,
            database: v.database,
            db_user: v.db_user,
            secret_arn: v.secret_arn,
            status: parse_status(&v.status),
            created_at: parse_dt(&v.created_at),
            updated_at: parse_dt(&v.updated_at),
            result_rows: v.result_rows,
            result_size: v.result_size,
            has_result_set: v.has_result_set,
            query_string: v.query_string,
            parameters: v
                .parameters
                .into_iter()
                .map(crate::types::StatementParameter::from)
                .collect(),
            sqls: vec![],
            statement_name: None,
            is_batch: false,
            result_columns: v.result_columns,
            result_data: v.result_data,
            error_message: v.error_message,
        }
    }
}

impl From<RedshiftDataStateView> for RedshiftDataState {
    fn from(view: RedshiftDataStateView) -> Self {
        RedshiftDataState {
            statements: view
                .statements
                .into_iter()
                .map(|(k, v)| (k, crate::types::Statement::from(v)))
                .collect(),
            databases: view.databases,
            schemas: view.schemas,
            table_names: view.table_names,
            table_columns: view.table_columns,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for RedshiftDataService {
    type StateView = RedshiftDataStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        RedshiftDataStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = RedshiftDataState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (id, stmt_view) in view.statements {
                guard
                    .statements
                    .insert(id, crate::types::Statement::from(stmt_view));
            }
            if !view.databases.is_empty() {
                guard.databases = view.databases;
            }
            if !view.schemas.is_empty() {
                guard.schemas = view.schemas;
            }
            if !view.table_names.is_empty() {
                guard.table_names = view.table_names;
            }
            for (table_name, columns) in view.table_columns {
                guard.table_columns.insert(table_name, columns);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
