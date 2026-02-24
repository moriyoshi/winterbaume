use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use duckdb::Connection;
use winterbaume_redshiftdata::backend::{RedshiftQueryBackend, StatementResult};

pub struct DuckDbRedshiftQueryBackend {
    conn: Arc<Mutex<Connection>>,
}

impl DuckDbRedshiftQueryBackend {
    /// Create a backend that executes queries against the given shared DuckDB
    /// connection.  The caller retains an `Arc<Mutex<Connection>>` handle and
    /// can use it to seed the database or share it across multiple backends.
    ///
    /// Each query briefly locks the mutex to call [`Connection::try_clone`],
    /// which creates a lightweight handle to the same underlying database.
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

fn run_sql(conn: &Connection, sql: &str) -> StatementResult {
    let duckdb_sql =
        papera::transpile(sql, papera::SourceDialect::Redshift).unwrap_or_else(|_| sql.to_string());
    match crate::exec::execute_duckdb_sql(conn, &duckdb_sql) {
        Ok(r) => StatementResult {
            columns: r.columns,
            rows: r.rows,
            error: None,
        },
        Err(e) => StatementResult {
            columns: vec![],
            rows: vec![],
            error: Some(e),
        },
    }
}

impl RedshiftQueryBackend for DuckDbRedshiftQueryBackend {
    fn execute_statement(
        &self,
        sql: String,
    ) -> Pin<Box<dyn Future<Output = StatementResult> + Send>> {
        let conn = self
            .conn
            .lock()
            .expect("DuckDB connection mutex poisoned")
            .try_clone()
            .expect("failed to clone DuckDB connection");
        Box::pin(async move { run_sql(&conn, &sql) })
    }

    fn batch_execute(
        &self,
        sqls: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = StatementResult> + Send>> {
        let conn = self
            .conn
            .lock()
            .expect("DuckDB connection mutex poisoned")
            .try_clone()
            .expect("failed to clone DuckDB connection");
        Box::pin(async move {
            let mut last = StatementResult::default();
            for sql in &sqls {
                let result = run_sql(&conn, sql);
                if result.error.is_some() {
                    return result;
                }
                last = result;
            }
            last
        })
    }
}
