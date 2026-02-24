//! Code generator for awsQuery protocol services.

use crate::model::ServiceModel;

/// Generate dispatch stub arms for unimplemented awsQuery operations.
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

/// Generate a complete handlers.rs for an awsQuery service.
pub fn generate_handlers(
    model: &ServiceModel,
    crate_name: &str,
    service_struct: &str,
    state_type: &str,
) -> String {
    let endpoint_prefix = crate_name
        .strip_prefix("winterbaume-")
        .unwrap_or(crate_name);

    let xml_ns = model.xml_namespace.as_deref().unwrap_or("");

    let mut dispatch_arms = String::new();
    for op in &model.operations {
        dispatch_arms.push_str(&format!(
            "            \"{}\" => MockResponse::error(\n                501,\n                \"NotImplementedError\",\n                \"{} is not yet implemented in {}\",\n            ),\n",
            op.name, op.name, crate_name,
        ));
    }

    format!(
        r#"use winterbaume_core::{{parse_query_string, xml_escape, BackendState, MockRequest, MockResponse, MockService, DEFAULT_ACCOUNT_ID}};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::state::{state_type};

/// XML namespace for {service_struct} responses.
const XML_NAMESPACE: &str = "{xml_ns}";

/// {service_struct} service handler (awsQuery protocol).
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

        let body_str = std::str::from_utf8(&request.body).unwrap_or("");
        let params = parse_query_string(body_str);

        let action = match params.get("Action") {{
            Some(a) => a.as_str(),
            None => {{
                return MockResponse::error(400, "MissingAction", "Missing 'Action' parameter");
            }}
        }};

        let _state = self.state.get(account_id, &region);

        match action {{
{dispatch_arms}            _ => MockResponse::error(
                400,
                "InvalidAction",
                &format!("Could not find operation {{action}} for {sdk_id}"),
            ),
        }}
    }}
}}
"#,
        sdk_id = model.sdk_id,
    )
}
