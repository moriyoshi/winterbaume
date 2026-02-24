//! Code generator for rpc-v2-cbor protocol services.

use crate::model::ServiceModel;

/// Generate a complete handlers.rs file for an rpc-v2-cbor service.
pub fn generate_handlers(
    model: &ServiceModel,
    crate_name: &str,
    service_struct: &str,
    state_type: &str,
) -> String {
    let mut dispatch_arms = String::new();
    for op in &model.operations {
        dispatch_arms.push_str(&format!(
            "            \"{}\" => cbor_error_response(\n                501,\n                \"NotImplementedError\",\n                \"{} is not yet implemented in {}\",\n            ),\n",
            op.name, op.name, crate_name,
        ));
    }

    format!(
        r#"use winterbaume_core::{{BackendState, MockRequest, MockResponse, MockService, DEFAULT_ACCOUNT_ID}};
use serde_json::{{Value, json}};
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

        // Extract action from URL path
        // Format: /service/<ServiceName>/operation/{{Action}}
        let action = extract_action_from_path(&request.uri);
        let action = match action {{
            Some(a) => a,
            None => {{
                return cbor_error_response(400, "MissingAction", "Could not determine action from URL path");
            }}
        }};

        // Decode CBOR body into serde_json::Value
        let body: Value = if request.body.is_empty() {{
            json!({{}})
        }} else {{
            match ciborium::from_reader::<ciborium::Value, _>(&request.body[..]) {{
                Ok(cbor_val) => cbor_to_json(cbor_val),
                Err(_) => {{
                    return cbor_error_response(400, "SerializationException", "Invalid CBOR body");
                }}
            }}
        }};

        let _state = self.state.get(account_id, &region);

        match action.as_str() {{
{dispatch_arms}            _ => cbor_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {{action}} for {sdk_id}"),
            ),
        }}
    }}
}}

fn extract_action_from_path(uri: &str) -> Option<String> {{
    let path = {{
        let after_scheme = uri
            .strip_prefix("https://")
            .or_else(|| uri.strip_prefix("http://"))
            .unwrap_or(uri);
        let slash_pos = after_scheme.find('/').unwrap_or(after_scheme.len());
        &after_scheme[slash_pos..]
    }};
    let idx = path.find("/operation/")?;
    let action = &path[idx + "/operation/".len()..];
    let action = action.split('?').next().unwrap_or(action);
    if action.is_empty() {{
        None
    }} else {{
        Some(action.to_string())
    }}
}}

fn cbor_to_json(cbor: ciborium::Value) -> Value {{
    match cbor {{
        ciborium::Value::Null => Value::Null,
        ciborium::Value::Bool(b) => Value::Bool(b),
        ciborium::Value::Integer(i) => {{
            let n: i128 = i.into();
            if let Ok(v) = i64::try_from(n) {{
                json!(v)
            }} else {{
                json!(n as f64)
            }}
        }}
        ciborium::Value::Float(f) => json!(f),
        ciborium::Value::Text(s) => Value::String(s),
        ciborium::Value::Bytes(b) => {{
            use base64::Engine;
            Value::String(base64::engine::general_purpose::STANDARD.encode(b))
        }}
        ciborium::Value::Array(arr) => Value::Array(arr.into_iter().map(cbor_to_json).collect()),
        ciborium::Value::Map(map) => {{
            let obj = map
                .into_iter()
                .filter_map(|(k, v)| {{
                    let key = match k {{
                        ciborium::Value::Text(s) => s,
                        _ => return None,
                    }};
                    Some((key, cbor_to_json(v)))
                }})
                .collect();
            Value::Object(obj)
        }}
        ciborium::Value::Tag(_, inner) => cbor_to_json(*inner),
        _ => Value::Null,
    }}
}}

fn json_to_cbor(json: &Value) -> ciborium::Value {{
    match json {{
        Value::Null => ciborium::Value::Null,
        Value::Bool(b) => ciborium::Value::Bool(*b),
        Value::Number(n) => {{
            if let Some(i) = n.as_i64() {{
                ciborium::Value::Integer(i.into())
            }} else if let Some(u) = n.as_u64() {{
                ciborium::Value::Integer(u.into())
            }} else if let Some(f) = n.as_f64() {{
                ciborium::Value::Float(f)
            }} else {{
                ciborium::Value::Null
            }}
        }}
        Value::String(s) => ciborium::Value::Text(s.clone()),
        Value::Array(arr) => ciborium::Value::Array(arr.iter().map(json_to_cbor).collect()),
        Value::Object(obj) => {{
            if obj.len() == 1 {{
                if let Some(Value::Number(n)) = obj.get("__cbor_epoch_seconds") {{
                    let inner = if let Some(i) = n.as_i64() {{
                        ciborium::Value::Integer(i.into())
                    }} else if let Some(f) = n.as_f64() {{
                        ciborium::Value::Float(f)
                    }} else {{
                        ciborium::Value::Null
                    }};
                    return ciborium::Value::Tag(1, Box::new(inner));
                }}
            }}
            let map: Vec<(ciborium::Value, ciborium::Value)> = obj
                .iter()
                .map(|(k, v)| (ciborium::Value::Text(k.clone()), json_to_cbor(v)))
                .collect();
            ciborium::Value::Map(map)
        }}
    }}
}}

fn cbor_response(status: u16, body: Value) -> MockResponse {{
    let cbor_val = json_to_cbor(&body);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(status, buf)
}}

fn cbor_error_response(status: u16, code: &str, message: &str) -> MockResponse {{
    cbor_response(
        status,
        json!({{
            "__type": code,
            "message": message,
        }}),
    )
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

/// Generate stub match arms for unimplemented operations.
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
