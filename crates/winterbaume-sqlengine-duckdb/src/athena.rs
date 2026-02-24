use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use duckdb::Connection;
use winterbaume_athena::backend::{AthenaQueryBackend, QueryResult};

pub struct DuckDbAthenaQueryBackend {
    conn: Arc<Mutex<Connection>>,
}

impl DuckDbAthenaQueryBackend {
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

impl AthenaQueryBackend for DuckDbAthenaQueryBackend {
    fn execute_query(&self, sql: String) -> Pin<Box<dyn Future<Output = QueryResult> + Send>> {
        let conn = self
            .conn
            .lock()
            .expect("DuckDB connection mutex poisoned")
            .try_clone()
            .expect("failed to clone DuckDB connection");
        Box::pin(async move {
            let duckdb_sql = papera::transpile(&sql, papera::SourceDialect::Trino).unwrap_or(sql);
            match crate::exec::execute_duckdb_sql(&conn, &duckdb_sql) {
                Ok(r) => QueryResult {
                    columns: r.columns,
                    rows: r.rows,
                    error: None,
                },
                Err(e) => QueryResult {
                    columns: vec![],
                    rows: vec![],
                    error: Some(e),
                },
            }
        })
    }
}
