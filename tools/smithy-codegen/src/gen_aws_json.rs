//! Code generator for awsJson1.0 and awsJson1.1 protocol services.

use crate::model::ServiceModel;

/// Generate the dispatch match table for an awsJson service.
///
/// Returns the body of the `match action.as_str() { ... }` block.
#[allow(dead_code)]
pub fn generate_dispatch_table(
    model: &ServiceModel,
    implemented_ops: &[String],
    crate_name: &str,
) -> String {
    let mut lines = Vec::new();

    for op in &model.operations {
        if implemented_ops.contains(&op.name) {
            // Preserve existing handler call — user will keep their handler
            let handler_name = format!("handle_{}", to_snake_case(&op.name));
            lines.push(format!(
                "            // TODO: wire to existing self.{handler_name}()",
            ));
            lines.push(format!(
                "            \"{}\" => todo!(\"wire {}\"),",
                op.name, op.name,
            ));
        } else {
            lines.push(format!(
                "            \"{}\" => json_error_response(501, \"NotImplementedError\", \"{} is not yet implemented in {}\"),",
                op.name, op.name, crate_name,
            ));
        }
    }

    lines.join("\n")
}

/// Generate a complete handlers.rs file for an awsJson service.
pub fn generate_handlers(
    model: &ServiceModel,
    crate_name: &str,
    service_struct: &str,
    state_type: &str,
) -> String {
    let target_prefix = &model.service_name;

    let mut dispatch_arms = String::new();
    for op in &model.operations {
        dispatch_arms.push_str(&format!(
            "            \"{}\" => json_error_response(\n                501,\n                \"NotImplementedError\",\n                \"{} is not yet implemented in {}\",\n            ),\n",
            op.name, op.name, crate_name,
        ));
    }

    format!(
        r#"use winterbaume_core::{{json_error_response, BackendState, MockRequest, MockResponse, MockService, DEFAULT_ACCOUNT_ID}};
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::state::{state_type};

/// {service_struct} service handler ({protocol} protocol).
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
        "{crate_short}"
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

        // Extract action from X-Amz-Target header
        // Format: "{target_prefix}.OperationName"
        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').last())
            .map(|s| s.to_string());

        let action = match action {{
            Some(a) => a,
            None => {{
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }}
        }};

        let body: Value = match serde_json::from_slice(&request.body) {{
            Ok(v) => v,
            Err(_) => {{
                return json_error_response(400, "SerializationException", "Invalid JSON body");
            }}
        }};

        let _state = self.state.get(account_id, &region);

        match action.as_str() {{
{dispatch_arms}            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {{action}} for {sdk_id}"),
            ),
        }}
    }}
}}
"#,
        protocol = model.protocol.as_str(),
        crate_short = crate_name
            .strip_prefix("winterbaume-")
            .unwrap_or(crate_name),
        endpoint_prefix = crate_name
            .strip_prefix("winterbaume-")
            .unwrap_or(crate_name),
        sdk_id = model.sdk_id,
    )
}

/// Generate a dispatch table snippet that can be merged into an existing handlers.rs.
///
/// This produces only the match arms for operations that are NOT in `existing_ops`.
pub fn generate_stub_arms(
    model: &ServiceModel,
    existing_ops: &[String],
    crate_name: &str,
    error_fn: &str,
) -> String {
    let mut lines = Vec::new();

    for op in &model.operations {
        if !existing_ops.contains(&op.name) {
            lines.push(format!(
                "            \"{}\" => {error_fn}(501, \"NotImplementedError\", \"{} is not yet implemented in {}\"),",
                op.name, op.name, crate_name,
            ));
        }
    }

    lines.join("\n")
}

#[allow(dead_code)]
fn to_snake_case(s: &str) -> String {
    heck::AsSnakeCase(s).to_string()
}
