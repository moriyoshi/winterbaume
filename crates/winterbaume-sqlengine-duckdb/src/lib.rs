mod athena;
mod exec;
mod redshift;

use std::sync::{Arc, Mutex};

pub use athena::DuckDbAthenaQueryBackend;
pub use duckdb::{Connection, Error as DuckDbError, Result as DuckDbResult};
pub use redshift::DuckDbRedshiftQueryBackend;

/// Open a DuckDB database and wrap it in the `Arc<Mutex<Connection>>` shape
/// that [`DuckDbAthenaQueryBackend`] and [`DuckDbRedshiftQueryBackend`] both
/// take, so callers can share a single database between the two services.
///
/// `path` is either `:memory:` (or empty) for an in-memory database, or a
/// filesystem path to a DuckDB file (created if missing).
pub fn open_database(path: &str) -> DuckDbResult<Arc<Mutex<Connection>>> {
    let conn = if path.is_empty() || path == ":memory:" {
        Connection::open_in_memory()?
    } else {
        Connection::open(path)?
    };
    Ok(Arc::new(Mutex::new(conn)))
}
