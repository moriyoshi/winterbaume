//! Pluggable query execution backend for the Athena mock service.
//!
//! The [`AthenaQueryBackend`] trait abstracts SQL execution so that alternative
//! implementations (e.g. DuckDB-backed) can be swapped in without touching the
//! protocol layer.
//!
//! The built-in [`InMemoryAthenaQueryBackend`] is the default; it returns empty
//! results immediately, preserving backward-compatible behavior for tests that
//! do not care about query output.
//!
//! # Object safety and async
//!
//! Uses the same `Pin<Box<dyn Future>>` pattern as `winterbaume-sqs`/`winterbaume-sns`
//! so that `Arc<dyn AthenaQueryBackend>` is object-safe without the `async-trait`
//! crate.

use std::future::Future;
use std::pin::Pin;

/// Result of executing a single SQL query.
#[derive(Debug, Clone, Default)]
pub struct QueryResult {
    /// Column metadata as `(name, type_str)` pairs.
    /// `type_str` uses lowercase DuckDB type names: `"varchar"`, `"integer"`,
    /// `"bigint"`, `"double"`, `"boolean"`, etc.
    pub columns: Vec<(String, String)>,
    /// Row data.  Each inner `Option<String>` is `None` for SQL NULL.
    pub rows: Vec<Vec<Option<String>>>,
    /// Non-`None` when execution failed.  Causes the query execution status
    /// to be set to `"FAILED"`.
    pub error: Option<String>,
}

/// Pluggable backend for Athena query execution.
pub trait AthenaQueryBackend: Send + Sync {
    /// Execute `sql` and return the result asynchronously.
    fn execute_query(&self, sql: String) -> Pin<Box<dyn Future<Output = QueryResult> + Send>>;
}

/// Default in-memory backend: returns an empty successful result without
/// executing anything.  Query executions are stored with status `"SUCCEEDED"`
/// and zero rows.
pub struct InMemoryAthenaQueryBackend;

impl AthenaQueryBackend for InMemoryAthenaQueryBackend {
    fn execute_query(&self, _sql: String) -> Pin<Box<dyn Future<Output = QueryResult> + Send>> {
        Box::pin(async move { QueryResult::default() })
    }
}
