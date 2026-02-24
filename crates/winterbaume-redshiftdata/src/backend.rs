//! Pluggable query execution backend for the Redshift Data mock service.
//!
//! The [`RedshiftQueryBackend`] trait abstracts SQL execution so that
//! alternative implementations (e.g. DuckDB-backed) can be swapped in without
//! touching the protocol layer.
//!
//! The built-in [`InMemoryRedshiftQueryBackend`] is the default; it returns
//! the same hardcoded three-row mock result (`Number`/`Street`/`City`) that
//! the service used before the backend abstraction was introduced, preserving
//! backward-compatible behavior for existing tests.
//!
//! # Object safety and async
//!
//! Uses the same `Pin<Box<dyn Future>>` pattern as `winterbaume-sqs`/`winterbaume-sns`
//! so that `Arc<dyn RedshiftQueryBackend>` is object-safe without the
//! `async-trait` crate.

use std::future::Future;
use std::pin::Pin;

/// Result of executing a single SQL statement.
#[derive(Debug, Clone, Default)]
pub struct StatementResult {
    /// Column metadata as `(name, type_str)` pairs.
    /// `type_str` uses lowercase DuckDB type names: `"varchar"`, `"integer"`,
    /// `"bigint"`, `"double"`, `"boolean"`, etc.
    pub columns: Vec<(String, String)>,
    /// Row data.  Each inner `Option<String>` is `None` for SQL NULL.
    pub rows: Vec<Vec<Option<String>>>,
    /// Non-`None` when execution failed.  Causes the statement status to be
    /// set to `Failed`.
    pub error: Option<String>,
}

/// Pluggable backend for Redshift Data query execution.
pub trait RedshiftQueryBackend: Send + Sync {
    /// Execute a single SQL statement and return the result asynchronously.
    fn execute_statement(
        &self,
        sql: String,
    ) -> Pin<Box<dyn Future<Output = StatementResult> + Send>>;

    /// Execute a batch of SQL statements sequentially.
    /// Batch executions typically do not return a result set.
    fn batch_execute(
        &self,
        sqls: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = StatementResult> + Send>>;
}

/// Default in-memory backend: returns the hardcoded three-row mock result
/// (`Number`/`Street`/`City`) for single statements, and an empty result for
/// batch executions.  Statements are stored with status `Finished`.
pub struct InMemoryRedshiftQueryBackend;

impl RedshiftQueryBackend for InMemoryRedshiftQueryBackend {
    fn execute_statement(
        &self,
        _sql: String,
    ) -> Pin<Box<dyn Future<Output = StatementResult> + Send>> {
        Box::pin(async move {
            StatementResult {
                columns: vec![
                    ("Number".to_string(), "integer".to_string()),
                    ("Street".to_string(), "varchar".to_string()),
                    ("City".to_string(), "varchar".to_string()),
                ],
                rows: vec![
                    vec![
                        Some("10".to_string()),
                        Some("Alpha st".to_string()),
                        Some("Portland".to_string()),
                    ],
                    vec![
                        Some("20".to_string()),
                        Some("Beta st".to_string()),
                        Some("Bellevue".to_string()),
                    ],
                    vec![
                        Some("30".to_string()),
                        Some("Gamma st".to_string()),
                        Some("Seattle".to_string()),
                    ],
                ],
                error: None,
            }
        })
    }

    fn batch_execute(
        &self,
        _sqls: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = StatementResult> + Send>> {
        Box::pin(async move { StatementResult::default() })
    }
}
