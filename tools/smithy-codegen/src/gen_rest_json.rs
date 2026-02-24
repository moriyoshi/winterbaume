//! Code generator for REST-JSON protocol services.

use crate::model::ServiceModel;

/// Generate a dispatch table for a REST-JSON service.
///
/// Produces match arms that route based on HTTP method + path segments.
pub fn generate_stub_arms(
    model: &ServiceModel,
    existing_ops: &[String],
    _crate_name: &str,
    _error_fn: &str,
) -> String {
    let mut lines = Vec::new();

    for op in &model.operations {
        if existing_ops.contains(&op.name) {
            continue;
        }
        if let Some(ref http) = op.http {
            lines.push(format!(
                "            // {} {} => {} (not implemented)",
                http.method, http.uri, op.name,
            ));
        }
    }

    if lines.is_empty() {
        return String::from("            // All operations are implemented!");
    }

    lines.push(String::new());
    lines.push(format!(
        "            // {} unimplemented operations above",
        lines.len() - 1,
    ));

    lines.join("\n")
}

/// Generate a complete route table for a REST-JSON service.
///
/// Each operation's HTTP trait is converted to a route entry.
#[allow(dead_code)]
pub fn generate_route_table(model: &ServiceModel, crate_name: &str) -> String {
    let mut routes = Vec::new();

    for op in &model.operations {
        let http = match &op.http {
            Some(h) => h,
            None => continue,
        };

        routes.push(format!(
            "    // {} {} => {}",
            http.method, http.uri, op.name,
        ));
    }

    let mut out = String::new();
    out.push_str(&format!("// REST-JSON route table for {crate_name}\n"));
    out.push_str(&format!(
        "// Total operations: {}\n\n",
        model.operations.len()
    ));

    for route in &routes {
        out.push_str(route);
        out.push('\n');
    }

    out
}

/// Generate a complete handlers.rs file for a REST-JSON service.
pub fn generate_handlers(
    model: &ServiceModel,
    crate_name: &str,
    service_struct: &str,
    state_type: &str,
) -> String {
    let endpoint_prefix = crate_name
        .strip_prefix("winterbaume-")
        .unwrap_or(crate_name);

    // Build route match arms from HTTP traits
    let mut route_arms = String::new();
    for op in &model.operations {
        let http = match &op.http {
            Some(h) => h,
            None => continue,
        };

        // Build a comment showing the route
        route_arms.push_str(&format!(
            "            // {} {} => {}\n",
            http.method, http.uri, op.name,
        ));
    }

    // Build catch-all 501 arms for unimplemented operations
    let mut dispatch_comments = String::new();
    for op in &model.operations {
        if let Some(ref http) = op.http {
            dispatch_comments.push_str(&format!(
                "        // {} {} => {}\n",
                http.method, http.uri, op.name,
            ));
        }
    }

    format!(
        r#"use winterbaume_core::{{extract_path, percent_decode, rest_json_error, BackendState, MockRequest, MockResponse, MockService, DEFAULT_ACCOUNT_ID}};
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::state::{state_type};

/// {service_struct} service handler (restJson1 protocol).
pub struct {service_struct} {{
    state: Arc<BackendState<{state_type}>>,
}}

impl {service_struct} {{
    pub fn new() -> Self {{
        Self {{
            state: Arc::new(BackendState::new()),
        }}
    }}
}}

impl Default for {service_struct} {{
    fn default() -> Self {{
        Self::new()
    }}
}}

impl MockService for {service_struct} {{
    fn service_name(&self) -> &str {{
        "{endpoint_prefix}"
    }}

    fn url_patterns(&self) -> Vec<&str> {{
        vec![
            r"https?://{endpoint_prefix}\..*\.amazonaws\.com",
            r"https?://{endpoint_prefix}\.amazonaws\.com",
        ]
    }}

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {{
        Box::pin(async move {{ self.dispatch(request) }})
    }}
}}

impl {service_struct} {{
    fn dispatch(&self, request: MockRequest) -> MockResponse {{
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let _state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();
        let _segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        // Route table:
{dispatch_comments}
        // Fallback: not found
        rest_json_error(404, "UnknownOperationException", "Not found")
    }}
}}
"#,
    )
}
