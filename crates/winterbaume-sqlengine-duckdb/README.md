# winterbaume-sqlengine-duckdb

DuckDB-backed SQL engine backend for winterbaume Athena and Redshift Data.

This crate is part of the [winterbaume](https://github.com/moriyoshi/winterbaume) workspace — a suite of in-process AWS service mocks for Rust. Use the umbrella [`winterbaume`](https://crates.io/crates/winterbaume) crate to pull in all services at once, or depend on this crate directly.

## Overview

This crate provides two backend implementations that execute SQL queries against an in-memory [DuckDB](https://duckdb.org/) instance:

- **`DuckDbAthenaQueryBackend`** — implements `AthenaQueryBackend` (Trino dialect)
- **`DuckDbRedshiftQueryBackend`** — implements `RedshiftQueryBackend` (Redshift dialect)

SQL is transparently transpiled from the source dialect (Trino or Redshift) to DuckDB-compatible SQL via the [`papera`](../papera) crate before execution.

## Usage

### Basic (empty database)

```rust
use std::sync::{Arc, Mutex};
use duckdb::Connection;
use aws_sdk_athena::config::BehaviorVersion;
use winterbaume_athena::AthenaService;
use winterbaume_core::MockAws;
use winterbaume_sqlengine_duckdb::DuckDbAthenaQueryBackend;

#[tokio::main]
async fn main() {
    let conn = Arc::new(Mutex::new(
        Connection::open_in_memory().expect("open DuckDB"),
    ));
    let mock = MockAws::builder()
        .with_service(AthenaService::with_query_backend(Arc::new(
            DuckDbAthenaQueryBackend::new(conn),
        )))
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_athena::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_athena::Client::new(&config);

    let resp = client
        .start_query_execution()
        .query_string("SELECT 1 AS n")
        .send()
        .await
        .expect("start_query_execution should succeed");

    println!("Query execution ID: {}", resp.query_execution_id().unwrap());
}
```

### Seeding the database

Because the `Connection` is injected from outside, you can seed it with tables and data before handing it to the backend:

```rust
use std::sync::{Arc, Mutex};
use duckdb::Connection;
use winterbaume_sqlengine_duckdb::DuckDbAthenaQueryBackend;

let conn = Arc::new(Mutex::new(
    Connection::open_in_memory().expect("open DuckDB"),
));

// Seed the database while you still hold a handle.
conn.lock().unwrap().execute_batch(
    "CREATE TABLE users (id INTEGER, name VARCHAR);
     INSERT INTO users VALUES (1, 'Alice'), (2, 'Bob');"
).expect("seed database");

let backend = DuckDbAthenaQueryBackend::new(Arc::clone(&conn));
// Queries executed through this backend can now SELECT from the `users` table.
// You can also keep seeding via `conn` after construction.
```

The same pattern works for `DuckDbRedshiftQueryBackend`. You can also share a single connection across both backends:

```rust
use std::sync::{Arc, Mutex};
use duckdb::Connection;
use winterbaume_sqlengine_duckdb::{DuckDbAthenaQueryBackend, DuckDbRedshiftQueryBackend};

let conn = Arc::new(Mutex::new(
    Connection::open_in_memory().expect("open DuckDB"),
));

conn.lock().unwrap().execute_batch(
    "CREATE TABLE events (ts TIMESTAMP, payload VARCHAR);
     INSERT INTO events VALUES ('2026-01-01 00:00:00', '{\"key\": \"value\"}');"
).expect("seed database");

let athena_backend = DuckDbAthenaQueryBackend::new(Arc::clone(&conn));
let redshift_backend = DuckDbRedshiftQueryBackend::new(Arc::clone(&conn));
// Both backends query the same underlying database.
```

### Loading from a file

You can also point at a pre-populated DuckDB database file:

```rust
use std::sync::{Arc, Mutex};
use duckdb::Connection;
use winterbaume_sqlengine_duckdb::DuckDbAthenaQueryBackend;

let conn = Arc::new(Mutex::new(
    Connection::open("test_fixtures/analytics.duckdb").expect("open DuckDB file"),
));
let backend = DuckDbAthenaQueryBackend::new(conn);
```

## How it works

Each backend struct holds a shared `Arc<Mutex<Connection>>`. When a query arrives:

1. The mutex is locked briefly to call `Connection::try_clone()`, which creates a lightweight handle to the **same underlying DuckDB database**.
2. The SQL is transpiled from the source dialect (Trino or Redshift) to DuckDB-compatible SQL.
3. The query is executed on the cloned connection and results are returned.

This design means the database is shared across all queries while avoiding contention — the mutex is never held during query execution.

## License

Licensed under Apache-2.0. See [LICENSE](https://github.com/moriyoshi/winterbaume/blob/main/LICENSE) for details.
