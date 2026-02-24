//! End-to-end integration test for the DuckDB-backed Athena and Redshift Data
//! services as wired by `winterbaume-server`.
//!
//! The server binary itself does not expose a library entry point, so this
//! test mirrors the relevant slice of `register_all_services` in
//! `crates/winterbaume-server/src/main.rs`: open a single in-memory DuckDB
//! database, share its `Arc<Mutex<Connection>>` between
//! `AthenaService::with_query_backend` and
//! `RedshiftDataService::with_query_backend`, then bring up an HTTP listener
//! on a free port and drive it via `aws-sdk-athena` / `aws-sdk-redshiftdata`.
//!
//! The round-trip (CREATE/INSERT via Athena, then SELECT via Redshift Data on
//! the same table) verifies that DDL/data written through one service is
//! visible to the other — i.e. the shared-connection injection performed by
//! the real `main.rs` is correct end-to-end.
//!
//! Gated on `backend-sqlengine-duckdb` via `required-features` in
//! `Cargo.toml`, so default builds do not pull DuckDB.

#![cfg(feature = "backend-sqlengine-duckdb")]

use std::sync::Arc;
use std::time::Duration;

use aws_config::BehaviorVersion;
use aws_credential_types::Credentials;
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use regex::Regex;
use tokio::net::TcpListener;
use winterbaume_core::service::{MockRequest, MockResponse, MockService};

// ---------------------------------------------------------------------------
// Minimal in-process HTTP harness — mirrors `winterbaume-server`'s router
// without dragging in the rest of the binary's service list.
// ---------------------------------------------------------------------------

struct ServiceRoute {
    pattern: Regex,
    service: Arc<dyn MockService>,
}

struct Router {
    routes: Vec<ServiceRoute>,
}

impl Router {
    fn new(services: Vec<Arc<dyn MockService>>) -> Self {
        let mut routes = Vec::new();
        for service in services {
            for pattern in service.url_patterns() {
                routes.push(ServiceRoute {
                    pattern: Regex::new(pattern)
                        .unwrap_or_else(|e| panic!("Invalid URL pattern '{pattern}': {e}")),
                    service: Arc::clone(&service),
                });
            }
        }
        Self { routes }
    }

    fn find_service(&self, uri: &str, headers: &http::HeaderMap) -> Option<Arc<dyn MockService>> {
        for route in &self.routes {
            if route.pattern.is_match(uri) {
                return Some(Arc::clone(&route.service));
            }
        }
        if let Some(name) = winterbaume_core::auth::extract_service_from_uri(uri) {
            if let Some(route) = self
                .routes
                .iter()
                .find(|r| r.service.service_name() == name)
            {
                return Some(Arc::clone(&route.service));
            }
        }
        if let Some(name) = winterbaume_core::auth::extract_service_from_headers(headers) {
            if let Some(route) = self
                .routes
                .iter()
                .find(|r| r.service.service_name() == name)
            {
                return Some(Arc::clone(&route.service));
            }
        }
        None
    }
}

async fn handle_request(
    router: Arc<Router>,
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let method = req.method().to_string();
    let host = req
        .headers()
        .get("host")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("localhost")
        .to_string();
    let path_and_query = req
        .uri()
        .path_and_query()
        .map(|pq| pq.as_str().to_string())
        .unwrap_or_else(|| "/".to_string());
    // Reconstruct the canonical AWS-shaped URL so the regex routes match
    // (they expect `https://athena[.region].amazonaws.com`). The aws-sdk
    // client sends a `host` header set to the configured endpoint host
    // (here `127.0.0.1:PORT`), so we synthesise a service host from the
    // signed Authorization header instead.
    let mut headers = http::HeaderMap::new();
    for (name, value) in req.headers() {
        headers.insert(name.clone(), value.clone());
    }

    let routing_uri =
        if let Some(name) = winterbaume_core::auth::extract_service_from_headers(&headers) {
            format!("https://{name}.us-east-1.amazonaws.com{path_and_query}")
        } else {
            format!("https://{host}{path_and_query}")
        };

    let body_bytes = req
        .into_body()
        .collect()
        .await
        .map(|c| c.to_bytes())
        .unwrap_or_default();

    let mock_request = MockRequest {
        method,
        uri: routing_uri.clone(),
        headers,
        body: body_bytes,
    };

    let mock_response = match router.find_service(&routing_uri, &mock_request.headers) {
        Some(service) => service.handle(mock_request).await,
        None => MockResponse::json(
            404,
            format!(
                r#"{{"__type":"UnknownService","message":"No service registered for: {routing_uri}"}}"#,
            ),
        ),
    };

    let mut response = Response::builder().status(mock_response.status);
    for (name, value) in &mock_response.headers {
        response = response.header(name, value);
    }
    Ok(response
        .body(Full::new(mock_response.body))
        .unwrap_or_else(|_| {
            Response::builder()
                .status(500)
                .body(Full::new(Bytes::from("Internal Server Error")))
                .unwrap()
        }))
}

/// Spawn a hyper server on a free port and return the bound port.
async fn start_server(services: Vec<Arc<dyn MockService>>) -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let router = Arc::new(Router::new(services));

    tokio::spawn(async move {
        loop {
            let (stream, _) = match listener.accept().await {
                Ok(conn) => conn,
                Err(_) => break,
            };
            let io = TokioIo::new(stream);
            let router = Arc::clone(&router);
            tokio::spawn(async move {
                let service = service_fn(move |req| {
                    let router = Arc::clone(&router);
                    async move { handle_request(router, req).await }
                });
                let _ = http1::Builder::new().serve_connection(io, service).await;
            });
        }
    });

    port
}

// ---------------------------------------------------------------------------
// Test
// ---------------------------------------------------------------------------

fn test_credentials() -> Credentials {
    Credentials::for_tests()
}

/// Boot Athena + Redshift Data on a single shared in-memory DuckDB instance,
/// then drive a CREATE/INSERT/SELECT on Athena and a follow-up SELECT on
/// Redshift Data. The Redshift Data SELECT sees the table the Athena writer
/// created, proving the `Arc<Mutex<Connection>>` is genuinely shared between
/// the two services as wired by `winterbaume-server::register_all_services`.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn duckdb_athena_redshift_round_trip() {
    // Mirror the wiring in crates/winterbaume-server/src/main.rs: one DuckDB,
    // two backends sharing the same connection handle.
    let conn =
        winterbaume_sqlengine_duckdb::open_database(":memory:").expect("open in-memory DuckDB");

    let athena_service = winterbaume_athena::AthenaService::with_query_backend(Arc::new(
        winterbaume_sqlengine_duckdb::DuckDbAthenaQueryBackend::new(Arc::clone(&conn)),
    ));
    let redshift_service = winterbaume_redshiftdata::RedshiftDataService::with_query_backend(
        Arc::new(winterbaume_sqlengine_duckdb::DuckDbRedshiftQueryBackend::new(Arc::clone(&conn))),
    );

    let services: Vec<Arc<dyn MockService>> =
        vec![Arc::new(athena_service), Arc::new(redshift_service)];

    let port = start_server(services).await;
    let endpoint_url = format!("http://127.0.0.1:{port}");

    // ---- Athena client --------------------------------------------------
    let athena_config = aws_config::defaults(BehaviorVersion::latest())
        .region(aws_config::Region::new("us-east-1"))
        .endpoint_url(&endpoint_url)
        .credentials_provider(test_credentials())
        .load()
        .await;
    let athena = aws_sdk_athena::Client::new(&athena_config);

    // 1. CREATE TABLE via Athena.
    run_athena_query(
        &athena,
        "CREATE TABLE shared_demo (id INTEGER, name VARCHAR)",
    )
    .await;

    // 2. INSERT data via Athena.
    run_athena_query(
        &athena,
        "INSERT INTO shared_demo VALUES (1, 'alice'), (2, 'bob')",
    )
    .await;

    // 3. SELECT via Athena and assert the round-trip works inside one
    //    service.
    let select_id = run_athena_query(&athena, "SELECT id, name FROM shared_demo ORDER BY id").await;

    let athena_results = athena
        .get_query_results()
        .query_execution_id(select_id)
        .send()
        .await
        .expect("athena GetQueryResults should succeed");
    let result_set = athena_results
        .result_set()
        .expect("athena results should have result_set");
    let rows = result_set.rows();
    // Athena prepends a header row.
    assert!(
        rows.len() >= 3,
        "expected header + 2 data rows from Athena, got {}: {:?}",
        rows.len(),
        rows,
    );
    assert_eq!(rows[0].data()[0].var_char_value(), Some("id"));
    assert_eq!(rows[0].data()[1].var_char_value(), Some("name"));
    assert_eq!(rows[1].data()[0].var_char_value(), Some("1"));
    assert_eq!(rows[1].data()[1].var_char_value(), Some("alice"));
    assert_eq!(rows[2].data()[0].var_char_value(), Some("2"));
    assert_eq!(rows[2].data()[1].var_char_value(), Some("bob"));

    // ---- Redshift Data client ------------------------------------------
    //
    // The shared-connection contract is what we are actually verifying:
    // the table created by Athena above must be visible from this client.
    let redshift_config = aws_config::defaults(BehaviorVersion::latest())
        .region(aws_config::Region::new("us-east-1"))
        .endpoint_url(&endpoint_url)
        .credentials_provider(test_credentials())
        .load()
        .await;
    let redshift = aws_sdk_redshiftdata::Client::new(&redshift_config);

    let exec = redshift
        .execute_statement()
        .sql("SELECT name FROM shared_demo WHERE id = 1")
        .database("dev")
        .cluster_identifier("winterbaume")
        .send()
        .await
        .expect("redshift ExecuteStatement should succeed");
    let stmt_id = exec
        .id()
        .expect("redshift execute_statement should return id");

    // Synchronous backend, but be defensive against any future async path.
    let mut waited = Duration::ZERO;
    let timeout = Duration::from_secs(5);
    let step = Duration::from_millis(50);
    loop {
        let desc = redshift
            .describe_statement()
            .id(stmt_id)
            .send()
            .await
            .expect("redshift DescribeStatement should succeed");
        let status = desc.status();
        match status.map(|s| s.as_str()) {
            Some("FINISHED") => break,
            Some("FAILED") | Some("ABORTED") => panic!(
                "redshift statement {stmt_id} ended in {status:?}: error={:?}",
                desc.error()
            ),
            _ if waited >= timeout => panic!(
                "redshift statement {stmt_id} did not finish within {timeout:?} (last status: {:?})",
                status,
            ),
            _ => {
                tokio::time::sleep(step).await;
                waited += step;
            }
        }
    }

    let result = redshift
        .get_statement_result()
        .id(stmt_id)
        .send()
        .await
        .expect("redshift GetStatementResult should succeed");
    let records = result.records();
    assert_eq!(
        records.len(),
        1,
        "expected one row from cross-service SELECT, got {}: {records:?}",
        records.len(),
    );
    assert_eq!(
        records[0][0]
            .as_string_value()
            .expect("name should be string-typed")
            .as_str(),
        "alice",
        "Redshift Data did not see the row inserted via Athena — \
         the shared Arc<Mutex<Connection>> wiring is broken",
    );
}

/// Run an Athena `StartQueryExecution` and poll `GetQueryExecution` until the
/// query reaches `SUCCEEDED`. Returns the QueryExecutionId on success.
async fn run_athena_query(client: &aws_sdk_athena::Client, sql: &str) -> String {
    let start = client
        .start_query_execution()
        .query_string(sql)
        .send()
        .await
        .unwrap_or_else(|e| panic!("athena StartQueryExecution(\"{sql}\") failed: {e:?}"));
    let id = start
        .query_execution_id()
        .unwrap_or_else(|| panic!("athena StartQueryExecution returned no id for \"{sql}\""))
        .to_string();

    let timeout = Duration::from_secs(5);
    let step = Duration::from_millis(50);
    let mut waited = Duration::ZERO;
    loop {
        let desc = client
            .get_query_execution()
            .query_execution_id(&id)
            .send()
            .await
            .unwrap_or_else(|e| {
                panic!("athena GetQueryExecution({id}) for \"{sql}\" failed: {e:?}")
            });
        let state = desc
            .query_execution()
            .and_then(|qe| qe.status())
            .and_then(|s| s.state())
            .map(|s| s.as_str().to_string());
        match state.as_deref() {
            Some("SUCCEEDED") => return id,
            Some("FAILED") | Some("CANCELLED") => {
                let reason = desc
                    .query_execution()
                    .and_then(|qe| qe.status())
                    .and_then(|s| s.state_change_reason())
                    .unwrap_or("(no reason)")
                    .to_string();
                panic!("athena query \"{sql}\" ended in {state:?}: reason={reason}",);
            }
            _ if waited >= timeout => panic!(
                "athena query \"{sql}\" did not finish within {timeout:?} (last state: {state:?})",
            ),
            _ => {
                tokio::time::sleep(step).await;
                waited += step;
            }
        }
    }
}
