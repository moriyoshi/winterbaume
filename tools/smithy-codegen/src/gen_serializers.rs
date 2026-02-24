//! Generate typed serializer/deserializer helpers from Smithy shapes.
//!
//! For each operation with an output shape, generates:
//! - A serde-compatible struct matching the Smithy output shape
//! - A serializer function that takes the typed struct and returns a MockResponse
//!
//! For REST-XML operations with an input shape, also generates request
//! deserializers that decode XML request bodies and bind HTTP labels, query
//! parameters, and headers into generated input structs.
//!
//! JSON protocols use `serde_json` for serialization.
//! XML protocols use `quick_xml::se` for serialization.

use crate::features::{FeatureMap, ShapeFeatures, cfg_attr_for_feature, cfg_attr_for_shape};
use crate::model::{Protocol, ResolvedType, ServiceModel, ShapeMember};

/// Generate a complete `wire.rs` module with serializer functions for all operations.
/// This is the combined output (structs + serializers in one file), used with `--combined`.
pub fn generate_wire_module(
    model: &ServiceModel,
    crate_name: &str,
    feature_map: Option<&FeatureMap>,
    shape_features: Option<&ShapeFeatures>,
) -> String {
    let (structs_needed, input_shapes, rename_overrides) = collect_shapes(model);

    let mut out = String::new();

    out.push_str("//! Auto-generated wire helpers from Smithy models.\n");
    out.push_str("//! Do not edit manually. Regenerate with:\n");
    out.push_str(&format!(
        "//!   smithy-codegen gen-serializers --combined {crate_name}\n\n"
    ));

    out.push_str("#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]\n\n");

    // Imports - all protocols need serde for typed structs
    out.push_str("use serde::{Deserialize, Serialize};\n");
    out.push_str("use winterbaume_core::MockResponse;\n");
    emit_protocol_imports(&mut out, model);
    out.push('\n');

    // Generate per-operation helpers
    generate_serializer_functions(&mut out, model, feature_map);
    generate_request_deserializer_functions(&mut out, model, feature_map, shape_features);

    // Generate struct definitions for all collected shapes
    let mut visited = std::collections::HashSet::new();
    let mut shapes_to_generate: Vec<String> = structs_needed.iter().cloned().collect();
    shapes_to_generate.sort();
    for shape_fqn in &shapes_to_generate {
        generate_struct_recursive(
            &mut out,
            shape_fqn,
            model,
            &input_shapes,
            &mut visited,
            &rename_overrides,
            shape_features,
        );
    }

    out
}

/// Generate only the `model.rs` module with struct definitions.
pub fn generate_model_module(
    model: &ServiceModel,
    crate_name: &str,
    shape_features: Option<&ShapeFeatures>,
) -> String {
    let (structs_needed, input_shapes, rename_overrides) = collect_shapes(model);

    let mut out = String::new();

    out.push_str("//! Auto-generated types from Smithy models.\n");
    out.push_str("//! Do not edit manually. Regenerate with:\n");
    out.push_str(&format!(
        "//!   smithy-codegen gen-serializers {crate_name}\n\n"
    ));

    out.push_str("#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]\n\n");

    out.push_str("use serde::{Deserialize, Serialize};\n\n");
    emit_model_helpers(&mut out, model);

    let mut visited = std::collections::HashSet::new();
    let mut shapes_to_generate: Vec<String> = structs_needed.iter().cloned().collect();
    shapes_to_generate.sort();
    for shape_fqn in &shapes_to_generate {
        generate_struct_recursive(
            &mut out,
            shape_fqn,
            model,
            &input_shapes,
            &mut visited,
            &rename_overrides,
            shape_features,
        );
    }

    out
}

/// Emit helpers needed by generated model serde attributes.
fn emit_model_helpers(out: &mut String, model: &ServiceModel) {
    if !matches!(model.protocol, Protocol::RpcV2Cbor) {
        return;
    }

    out.push_str(
        r#"// These helpers are intentionally serialize-only. Generated rpc-v2-cbor
// request deserializers first convert CBOR Tag 1 values into plain JSON epoch
// numbers, then serde-deserialize those numbers into f64 fields.
#[derive(Serialize)]
struct CborTimestampSentinel {
    #[serde(rename = "__cbor_epoch_seconds")]
    epoch_seconds: f64,
}

fn serialize_cbor_timestamp<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    CborTimestampSentinel {
        epoch_seconds: *value,
    }
    .serialize(serializer)
}

fn serialize_cbor_timestamp_vec<S>(value: &[f64], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let sentinels: Vec<CborTimestampSentinel> = value
        .iter()
        .map(|epoch_seconds| CborTimestampSentinel {
            epoch_seconds: *epoch_seconds,
        })
        .collect();
    sentinels.serialize(serializer)
}

fn serialize_cbor_timestamp_vec_opt<S>(
    value: &Option<Vec<f64>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(values) => {
            let sentinels: Vec<CborTimestampSentinel> = values
                .iter()
                .map(|epoch_seconds| CborTimestampSentinel {
                    epoch_seconds: *epoch_seconds,
                })
                .collect();
            sentinels.serialize(serializer)
        }
        None => serializer.serialize_none(),
    }
}

fn serialize_cbor_timestamp_opt<S>(value: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(epoch_seconds) => CborTimestampSentinel {
            epoch_seconds: *epoch_seconds,
        }
        .serialize(serializer),
        None => serializer.serialize_none(),
    }
}

"#,
    );
}

/// Generate only the `wire.rs` module with serializer functions (importing types from model).
pub fn generate_wire_module_split(
    model: &ServiceModel,
    crate_name: &str,
    feature_map: Option<&FeatureMap>,
) -> String {
    let mut out = String::new();

    out.push_str("//! Auto-generated wire helpers from Smithy models.\n");
    out.push_str("//! Do not edit manually. Regenerate with:\n");
    out.push_str(&format!(
        "//!   smithy-codegen gen-serializers {crate_name}\n\n"
    ));

    out.push_str(
        "#![allow(\n    dead_code,\n    unused_variables,\n    clippy::let_and_return,\n    clippy::single_match\n)]\n\n",
    );
    out.push_str("pub use super::model::*;\n\n");
    out.push_str("use winterbaume_core::MockResponse;\n");
    emit_protocol_imports(&mut out, model);
    out.push('\n');

    generate_serializer_functions(&mut out, model, feature_map);
    // For wire.rs in split mode, the model.rs already gates structs by shape
    // features. The wire.rs only needs per-operation gating for the top-level
    // serialise/deserialise functions; nested struct deserialiser helpers
    // emitted by AwsQuery/Ec2Query also need shape-level gating, so we still
    // need shape_features. However, in split mode we don't have it here — we
    // only need it when the wire.rs emits sub-deserialisers (AwsQuery/Ec2Query).
    // Recompute on the fly when a feature_map is provided.
    let shape_features_owned =
        feature_map.map(|fm| crate::features::compute_shape_features(model, fm));
    generate_request_deserializer_functions(
        &mut out,
        model,
        feature_map,
        shape_features_owned.as_ref(),
    );

    out
}

/// Collect all struct shapes needed across all operations, plus input-shape
/// markers and rename overrides.
fn collect_shapes(
    model: &ServiceModel,
) -> (
    std::collections::HashSet<String>,
    std::collections::HashSet<String>,
    std::collections::HashMap<String, String>,
) {
    let mut structs_needed: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut input_shapes: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut rename_overrides: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();

    for op in &model.operations {
        if let Some(input_shape) = &op.input_shape {
            structs_needed.insert(input_shape.to_string());
            input_shapes.insert(input_shape.to_string());

            if matches!(model.protocol, Protocol::RestXml) {
                let members = model.get_members(input_shape);
                let payload_member = members.iter().find(|m| m.http_payload);
                if let Some(payload) = payload_member {
                    let resolved = model.resolve_type(&payload.target);
                    if let ResolvedType::Structure(ref s) = resolved {
                        structs_needed.insert(s.clone());
                    }
                }
            }
        }

        if let Some(output_shape) = &op.output_shape {
            structs_needed.insert(output_shape.to_string());

            if matches!(model.protocol, Protocol::AwsQuery | Protocol::Ec2Query) {
                rename_overrides.insert(output_shape.to_string(), format!("{}Result", op.name));
            }

            if matches!(model.protocol, Protocol::RestXml) {
                let members = model.get_members(output_shape);
                let payload_member = members.iter().find(|m| m.http_payload);
                if let Some(payload) = payload_member {
                    let resolved = model.resolve_type(&payload.target);
                    if let ResolvedType::Structure(ref s) = resolved {
                        structs_needed.insert(s.clone());
                    }
                }
            }
        }
    }

    // Compute transitive closure of all struct shapes reachable from operation
    // inputs.  Any struct NOT in this set is output-only and can have its
    // `@required` fields rendered as `Option<T>` (so `Default::default()`
    // produces an empty JSON object `{}`).
    let input_reachable = compute_input_reachable(&input_shapes, model);

    (structs_needed, input_reachable, rename_overrides)
}

/// BFS from every direct operation-input shape; returns every struct shape
/// (FQN) transitively reachable through member types.
fn compute_input_reachable(
    input_shapes: &std::collections::HashSet<String>,
    model: &ServiceModel,
) -> std::collections::HashSet<String> {
    let mut reachable: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut worklist: Vec<String> = input_shapes.iter().cloned().collect();

    while let Some(shape_fqn) = worklist.pop() {
        if !reachable.insert(shape_fqn.clone()) {
            continue;
        }
        for member in model.get_members(&shape_fqn) {
            collect_struct_fqns_from_resolved(&model.resolve_type(&member.target), &mut worklist);
        }
    }

    reachable
}

/// Push every `Structure` FQN reachable within `resolved` onto `out`.
fn collect_struct_fqns_from_resolved(resolved: &ResolvedType, out: &mut Vec<String>) {
    match resolved {
        ResolvedType::Structure(fqn) => out.push(fqn.clone()),
        ResolvedType::List(inner) => collect_struct_fqns_from_resolved(inner, out),
        ResolvedType::Map(k, v) => {
            collect_struct_fqns_from_resolved(k, out);
            collect_struct_fqns_from_resolved(v, out);
        }
        _ => {}
    }
}

/// Emit protocol-specific imports.
fn emit_protocol_imports(out: &mut String, model: &ServiceModel) {
    let mut emitted = std::collections::HashSet::new();
    emit_protocol_imports_for(out, model.protocol, &mut emitted);
    for &proto in &model.additional_protocols {
        emit_protocol_imports_for(out, proto, &mut emitted);
    }
}

fn emit_protocol_imports_for(
    out: &mut String,
    protocol: Protocol,
    emitted: &mut std::collections::HashSet<&'static str>,
) {
    match protocol {
        Protocol::AwsJson1_0 | Protocol::AwsJson1_1 => {}
        Protocol::RestJson1 => {
            if emitted.insert("rest_json_imports") {
                out.push_str("#[allow(unused_imports)]\n");
                out.push_str("use http::header::HeaderName;\n");
            }
        }
        Protocol::AwsQuery | Protocol::Ec2Query => {
            if emitted.insert("strip_outer_element") {
                out.push_str("fn strip_outer_element(xml: &str) -> &str {\n");
                out.push_str("    // Find the end of the opening tag\n");
                out.push_str("    if let Some(close_pos) = xml.find('>') {\n");
                out.push_str("        let inner_start = close_pos + 1;\n");
                out.push_str("        // Find the last closing tag\n");
                out.push_str("        if let Some(last_open) = xml.rfind('<') {\n");
                out.push_str("            if last_open >= inner_start {\n");
                out.push_str("                return &xml[inner_start..last_open];\n");
                out.push_str("            }\n");
                out.push_str("        }\n");
                out.push_str("    }\n");
                out.push_str("    xml\n");
                out.push_str("}\n");
            }
        }
        Protocol::RestXml => {}
        Protocol::RpcV2Cbor => {
            if emitted.insert("cbor_helpers") {
                emit_cbor_helpers(out);
            }
        }
    }
}

/// Emit the CBOR helper functions (`json_to_cbor`, `cbor_to_json`, `cbor_error_response`)
/// into the generated wire.rs for rpc-v2-cbor protocol services.
fn emit_cbor_helpers(out: &mut String) {
    out.push_str(
        r#"/// Convert `serde_json::Value` to `ciborium::Value` for CBOR encoding.
///
/// Recognises the `{"__cbor_epoch_seconds": N}` sentinel and encodes it as
/// CBOR Tag 1 (epoch-based date/time) per the rpc-v2-cbor protocol.
pub fn json_to_cbor(json: &serde_json::Value) -> ciborium::Value {
    match json {
        serde_json::Value::Null => ciborium::Value::Null,
        serde_json::Value::Bool(b) => ciborium::Value::Bool(*b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                ciborium::Value::Integer(i.into())
            } else if let Some(u) = n.as_u64() {
                ciborium::Value::Integer(u.into())
            } else if let Some(f) = n.as_f64() {
                ciborium::Value::Float(f)
            } else {
                ciborium::Value::Null
            }
        }
        serde_json::Value::String(s) => ciborium::Value::Text(s.clone()),
        serde_json::Value::Array(arr) => {
            ciborium::Value::Array(arr.iter().map(json_to_cbor).collect())
        }
        serde_json::Value::Object(obj) => {
            // Detect the cbor_timestamp sentinel.
            if obj.len() == 1 {
                if let Some(serde_json::Value::Number(n)) = obj.get("__cbor_epoch_seconds") {
                    let inner = if let Some(i) = n.as_i64() {
                        ciborium::Value::Integer(i.into())
                    } else if let Some(f) = n.as_f64() {
                        ciborium::Value::Float(f)
                    } else {
                        ciborium::Value::Null
                    };
                    return ciborium::Value::Tag(1, Box::new(inner));
                }
            }
            let map: Vec<(ciborium::Value, ciborium::Value)> = obj
                .iter()
                .map(|(k, v)| (ciborium::Value::Text(k.clone()), json_to_cbor(v)))
                .collect();
            ciborium::Value::Map(map)
        }
    }
}

/// Convert `ciborium::Value` to `serde_json::Value` for request decoding.
pub fn cbor_to_json(cbor: ciborium::Value) -> serde_json::Value {
    match cbor {
        ciborium::Value::Null => serde_json::Value::Null,
        ciborium::Value::Bool(b) => serde_json::Value::Bool(b),
        ciborium::Value::Integer(i) => {
            let n: i128 = i.into();
            if let Ok(v) = i64::try_from(n) {
                serde_json::json!(v)
            } else {
                serde_json::json!(n as f64)
            }
        }
        ciborium::Value::Float(f) => serde_json::json!(f),
        ciborium::Value::Text(s) => serde_json::Value::String(s),
        ciborium::Value::Bytes(b) => {
            use base64::Engine;
            serde_json::Value::String(base64::engine::general_purpose::STANDARD.encode(b))
        }
        ciborium::Value::Array(arr) => {
            serde_json::Value::Array(arr.into_iter().map(cbor_to_json).collect())
        }
        ciborium::Value::Map(map) => {
            let obj = map
                .into_iter()
                .filter_map(|(k, v)| {
                    let key = match k {
                        ciborium::Value::Text(s) => s,
                        _ => return None,
                    };
                    Some((key, cbor_to_json(v)))
                })
                .collect();
            serde_json::Value::Object(obj)
        }
        ciborium::Value::Tag(_, inner) => cbor_to_json(*inner),
        _ => serde_json::Value::Null,
    }
}

/// Build a CBOR-encoded error response.
pub fn cbor_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = serde_json::json!({
        "__type": code,
        "message": message,
    });
    let cbor_val = json_to_cbor(&body);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(status, buf)
}

"#,
    );
}

/// Generate all serializer functions (without struct definitions).
fn generate_serializer_functions(
    out: &mut String,
    model: &ServiceModel,
    feature_map: Option<&FeatureMap>,
) {
    generate_serializer_functions_for_protocol(out, model, model.protocol, "", feature_map);

    // Generate additional protocol serializers with protocol-specific suffixes
    for &proto in &model.additional_protocols {
        let suffix = protocol_fn_suffix(proto);
        generate_serializer_functions_for_protocol(out, model, proto, suffix, feature_map);
    }
}

fn generate_serializer_functions_for_protocol(
    out: &mut String,
    model: &ServiceModel,
    protocol: Protocol,
    fn_suffix: &str,
    feature_map: Option<&FeatureMap>,
) {
    for op in &model.operations {
        let fn_name = format!("serialize_{}_response{fn_suffix}", to_snake_case(&op.name));
        let http_code = op.http.as_ref().map(|h| h.code).unwrap_or(200);

        if let Some(fm) = feature_map {
            out.push_str(&cfg_attr_for_feature(fm.feature_for(&op.name)));
        }

        let output_shape = match &op.output_shape {
            Some(s) => s,
            None => {
                generate_void_serializer_for_protocol(
                    out, &fn_name, &op.name, http_code, model, protocol,
                );
                continue;
            }
        };

        let members = model.get_members(output_shape);
        let struct_name = escape_type_name(ServiceModel::short_name(output_shape));

        match protocol {
            Protocol::AwsJson1_0 | Protocol::AwsJson1_1 => {
                generate_aws_json_serializer(out, &fn_name, http_code, &struct_name);
            }
            Protocol::RestJson1 => {
                generate_rest_json_serializer(
                    out,
                    &fn_name,
                    http_code,
                    &members,
                    &struct_name,
                    model,
                );
            }
            Protocol::AwsQuery => {
                let ns = model.xml_namespace.as_deref().unwrap_or("");
                generate_aws_query_serializer(out, &fn_name, &op.name, ns, &struct_name);
            }
            Protocol::Ec2Query => {
                let ns = model.xml_namespace.as_deref().unwrap_or("");
                generate_ec2_query_serializer(out, &fn_name, &op.name, ns, &struct_name);
            }
            Protocol::RestXml => {
                let mut structs_needed = std::collections::HashSet::new();
                generate_rest_xml_serializer(
                    out,
                    &fn_name,
                    http_code,
                    output_shape,
                    &members,
                    model,
                    &struct_name,
                    &mut structs_needed,
                );
            }
            Protocol::RpcV2Cbor => {
                generate_rpcv2_cbor_serializer(out, &fn_name, http_code, &struct_name);
            }
        }
    }
}

// --- rpc-v2-cbor serializer ---

fn generate_rpcv2_cbor_serializer(
    out: &mut String,
    fn_name: &str,
    http_code: u16,
    struct_name: &str,
) {
    out.push_str(&format!(
        r#"/// Serialize response for rpc-v2-cbor protocol.
pub fn {fn_name}(result: &{struct_name}) -> MockResponse {{
    let json_val = serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor({http_code}, buf)
}}

"#
    ));
}

/// Generate request deserializers for protocols that need them.
fn generate_request_deserializer_functions(
    out: &mut String,
    model: &ServiceModel,
    feature_map: Option<&FeatureMap>,
    shape_features: Option<&ShapeFeatures>,
) {
    generate_request_deserializers_for_protocol(
        out,
        model,
        model.protocol,
        "",
        feature_map,
        shape_features,
    );

    for &proto in &model.additional_protocols {
        let suffix = protocol_fn_suffix(proto);
        generate_request_deserializers_for_protocol(
            out,
            model,
            proto,
            suffix,
            feature_map,
            shape_features,
        );
    }
}

fn generate_request_deserializers_for_protocol(
    out: &mut String,
    model: &ServiceModel,
    protocol: Protocol,
    fn_suffix: &str,
    feature_map: Option<&FeatureMap>,
    shape_features: Option<&ShapeFeatures>,
) {
    match protocol {
        Protocol::RestXml => {
            for op in &model.operations {
                let Some(input_shape) = &op.input_shape else {
                    continue;
                };
                if let Some(fm) = feature_map {
                    out.push_str(&cfg_attr_for_feature(fm.feature_for(&op.name)));
                }
                let op_name_suffixed = format!("{}{}", op.name, fn_suffix_to_pascal(fn_suffix));
                generate_rest_xml_request_deserializer(out, &op_name_suffixed, input_shape, model);
            }
        }
        Protocol::AwsQuery | Protocol::Ec2Query => {
            // Collect all structure shapes reachable from input shapes to generate sub-deserializers
            let mut struct_deserializers_needed: std::collections::HashSet<String> =
                std::collections::HashSet::new();
            for op in &model.operations {
                let Some(input_shape) = &op.input_shape else {
                    continue;
                };
                collect_struct_deserializers(input_shape, model, &mut struct_deserializers_needed);
            }
            // Generate sub-deserializers for nested structures.
            // When this protocol is the primary (fn_suffix is empty), always emit them.
            // When this is an additional protocol, only emit them if the primary
            // protocol is NOT also awsQuery/ec2Query (which would have already
            // generated them).
            let primary_is_query =
                matches!(model.protocol, Protocol::AwsQuery | Protocol::Ec2Query);
            if fn_suffix.is_empty() || !primary_is_query {
                for shape_fqn in &struct_deserializers_needed {
                    if let Some(sf) = shape_features {
                        out.push_str(&cfg_attr_for_shape(sf.get(shape_fqn)));
                    }
                    generate_aws_query_struct_deserializer(out, shape_fqn, model);
                }
            }
            // Generate top-level operation deserializers
            for op in &model.operations {
                let Some(input_shape) = &op.input_shape else {
                    continue;
                };
                if let Some(fm) = feature_map {
                    out.push_str(&cfg_attr_for_feature(fm.feature_for(&op.name)));
                }
                generate_aws_query_request_deserializer_with_suffix(
                    out,
                    &op.name,
                    input_shape,
                    model,
                    fn_suffix,
                );
            }
        }
        Protocol::RpcV2Cbor => {
            for op in &model.operations {
                let Some(input_shape) = &op.input_shape else {
                    continue;
                };
                if let Some(fm) = feature_map {
                    out.push_str(&cfg_attr_for_feature(fm.feature_for(&op.name)));
                }
                let op_name_suffixed = format!("{}{}", op.name, fn_suffix_to_pascal(fn_suffix));
                generate_rpcv2_cbor_request_deserializer(
                    out,
                    &op_name_suffixed,
                    input_shape,
                    model,
                );
            }
        }
        Protocol::AwsJson1_0 | Protocol::AwsJson1_1 => {
            for op in &model.operations {
                let Some(input_shape) = &op.input_shape else {
                    continue;
                };
                if let Some(fm) = feature_map {
                    out.push_str(&cfg_attr_for_feature(fm.feature_for(&op.name)));
                }
                let op_name_suffixed = format!("{}{}", op.name, fn_suffix_to_pascal(fn_suffix));
                generate_aws_json_request_deserializer(out, &op_name_suffixed, input_shape, model);
            }
        }
        Protocol::RestJson1 => {
            for op in &model.operations {
                let Some(input_shape) = &op.input_shape else {
                    continue;
                };
                if let Some(fm) = feature_map {
                    out.push_str(&cfg_attr_for_feature(fm.feature_for(&op.name)));
                }
                let op_name_suffixed = format!("{}{}", op.name, fn_suffix_to_pascal(fn_suffix));
                generate_rest_json_request_deserializer(out, &op_name_suffixed, input_shape, model);
            }
        }
    }
}

fn fn_suffix_to_pascal(suffix: &str) -> &str {
    match suffix {
        "_query" => "Query",
        "_json" => "Json",
        "_cbor" => "Cbor",
        _ => "",
    }
}

/// Map a protocol to the function name suffix used for additional protocol variants.
fn protocol_fn_suffix(protocol: Protocol) -> &'static str {
    match protocol {
        Protocol::AwsQuery | Protocol::Ec2Query => "_query",
        Protocol::AwsJson1_0 | Protocol::AwsJson1_1 => "_json",
        Protocol::RestJson1 => "_json",
        Protocol::RestXml => "_xml",
        Protocol::RpcV2Cbor => "_cbor",
    }
}

// --- rpc-v2-cbor request deserializer ---

fn generate_rpcv2_cbor_request_deserializer(
    out: &mut String,
    op_name: &str,
    input_shape: &str,
    _model: &ServiceModel,
) {
    let fn_name = format!("deserialize_{}_request", to_snake_case(op_name));
    let struct_name = escape_type_name(ServiceModel::short_name(input_shape));
    out.push_str(&format!(
        r#"/// Deserialize request for rpc-v2-cbor protocol.
pub fn {fn_name}(body: &[u8]) -> Result<{struct_name}, String> {{
    if body.is_empty() {{
        return Ok({struct_name}::default());
    }}
    let cbor_val: ciborium::Value = ciborium::from_reader(body)
        .map_err(|e| format!("CBOR decode error: {{e}}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize {op_name} request: {{e}}"))
}}

"#
    ));
}

// --- awsJson request deserializer ---

fn generate_aws_json_request_deserializer(
    out: &mut String,
    op_name: &str,
    input_shape: &str,
    _model: &ServiceModel,
) {
    let fn_name = format!("deserialize_{}_request", to_snake_case(op_name));
    let struct_name = escape_type_name(ServiceModel::short_name(input_shape));
    out.push_str(&format!(
        r#"/// Deserialize request for awsJson protocol.
pub fn {fn_name}(body: &[u8]) -> Result<{struct_name}, String> {{
    if body.is_empty() {{
        return Ok({struct_name}::default());
    }}
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize {op_name} request: {{e}}"))
}}

"#
    ));
}

// --- restJson request deserializer ---

fn generate_rest_json_request_deserializer(
    out: &mut String,
    op_name: &str,
    input_shape: &str,
    model: &ServiceModel,
) {
    let fn_name = format!("deserialize_{}_request", to_snake_case(op_name));
    let struct_name = escape_type_name(ServiceModel::short_name(input_shape));
    let members = model.get_members(input_shape);
    let payload_member = members.iter().find(|m| m.http_payload);
    let body_members: Vec<&ShapeMember> = members
        .iter()
        .filter(|m| {
            m.http_header.is_none()
                && !m.http_label
                && m.http_query.is_none()
                && !m.http_payload
                && !m.http_response_code
        })
        .collect();
    let label_members: Vec<&ShapeMember> = members.iter().filter(|m| m.http_label).collect();
    let query_members: Vec<&ShapeMember> =
        members.iter().filter(|m| m.http_query.is_some()).collect();
    let header_members: Vec<&ShapeMember> =
        members.iter().filter(|m| m.http_header.is_some()).collect();

    out.push_str("/// Deserialize request for restJson protocol.\n");
    out.push_str(&format!(
        "pub fn {fn_name}(\n    request: &winterbaume_core::MockRequest,\n    labels: &[(&str, &str)],\n    query: &std::collections::HashMap<String, String>,\n) -> Result<{struct_name}, String> {{\n"
    ));
    if members.is_empty() {
        out.push_str(&format!("    let input = {struct_name} {{}};\n"));
    } else {
        out.push_str(&format!("    let mut input = {struct_name}::default();\n"));
    }

    if let Some(payload) = payload_member {
        let resolved = model.resolve_type(&payload.target);
        let field_name = to_snake_case(&payload.name);
        match resolved {
            ResolvedType::Structure(ref s) => {
                let payload_struct = escape_type_name(ServiceModel::short_name(s));
                out.push_str("    if !request.body.is_empty() {\n");
                out.push_str(&format!(
                    "        let payload = serde_json::from_slice::<{payload_struct}>(&request.body)\n            .map_err(|err| format!(\"failed to deserialize {op_name} payload: {{err}}\"))?;\n"
                ));
                if payload.required {
                    out.push_str(&format!("        input.{field_name} = payload;\n"));
                } else {
                    out.push_str(&format!("        input.{field_name} = Some(payload);\n"));
                }
                out.push_str("    }\n");
            }
            _ => {
                let value_expr = binding_value_expr_for_member(payload, model, "body");
                out.push_str("    if !request.body.is_empty() {\n");
                out.push_str("        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;\n");
                out.push_str(&format!(
                    "        {};\n",
                    assign_member_value("input", &field_name, payload.required, &value_expr)
                ));
                out.push_str("    }\n");
            }
        }
    } else if !body_members.is_empty() {
        out.push_str("    if !request.body.is_empty() {\n");
        out.push_str(&format!(
            "        input = serde_json::from_slice::<{struct_name}>(&request.body)\n            .map_err(|err| format!(\"failed to deserialize {op_name} request: {{err}}\"))?;\n"
        ));
        out.push_str("    }\n");
    }

    if !label_members.is_empty() {
        out.push_str("    for (name, value) in labels {\n");
        out.push_str("        match *name {\n");
        for member in &label_members {
            let field_name = to_snake_case(&member.name);
            let value_expr = binding_value_expr_for_member(member, model, "value");
            out.push_str(&format!(
                "            \"{}\" => {{ {};}},\n",
                member.name,
                assign_member_value("input", &field_name, member.required, &value_expr)
            ));
        }
        out.push_str("            _ => {}\n");
        out.push_str("        }\n");
        out.push_str("    }\n");
    }

    for member in &query_members {
        let field_name = to_snake_case(&member.name);
        let value_expr = binding_value_expr_for_member(member, model, "value");
        let query_name = member.http_query.as_ref().unwrap();
        out.push_str(&format!(
            "    if let Some(value) = query.get(\"{query_name}\") {{ {};\n    }}\n",
            assign_member_value("input", &field_name, member.required, &value_expr)
        ));
    }

    for member in &header_members {
        let field_name = to_snake_case(&member.name);
        let value_expr = binding_value_expr_for_member(member, model, "value");
        let header_name = member.http_header.as_ref().unwrap();
        out.push_str(&format!(
            "    if let Some(value) = request.headers.get(\"{header_name}\").and_then(|value| value.to_str().ok()) {{ {};\n    }}\n",
            assign_member_value("input", &field_name, member.required, &value_expr)
        ));
    }

    out.push_str("    Ok(input)\n");
    out.push_str("}\n\n");
}

// --- Void serializer (operations with no output shape) ---

#[allow(dead_code)]
fn generate_void_serializer(out: &mut String, fn_name: &str, http_code: u16, model: &ServiceModel) {
    generate_void_serializer_for_protocol(out, fn_name, "", http_code, model, model.protocol);
}

fn generate_void_serializer_for_protocol(
    out: &mut String,
    fn_name: &str,
    op_name: &str,
    http_code: u16,
    model: &ServiceModel,
    protocol: Protocol,
) {
    match protocol {
        Protocol::AwsJson1_0 | Protocol::AwsJson1_1 => {
            out.push_str(&format!(
                "/// Serialize void response for awsJson protocol.\npub fn {fn_name}() -> MockResponse {{\n    MockResponse::json({http_code}, \"{{}}\")\n}}\n\n"
            ));
        }
        Protocol::RestJson1 => {
            out.push_str(&format!(
                "/// Serialize void response for restJson protocol.\npub fn {fn_name}() -> MockResponse {{\n    MockResponse::rest_json({http_code}, \"{{}}\")\n}}\n\n"
            ));
        }
        Protocol::AwsQuery => {
            let ns = model.xml_namespace.as_deref().unwrap_or("");
            out.push_str(&format!(
                "/// Serialize void response for awsQuery protocol.\npub fn {fn_name}() -> MockResponse {{\n    let request_id = uuid::Uuid::new_v4();\n    let xml = format!(\n        r#\"<{op_name}Response xmlns=\"{ns}\">\n  <ResponseMetadata>\n    <RequestId>{{request_id}}</RequestId>\n  </ResponseMetadata>\n</{op_name}Response>\"#\n    );\n    MockResponse::xml(200, xml)\n}}\n\n"
            ));
        }
        Protocol::Ec2Query => {
            let ns = model.xml_namespace.as_deref().unwrap_or("");
            out.push_str(&format!(
                "/// Serialize void response for ec2Query protocol.\npub fn {fn_name}() -> MockResponse {{\n    let request_id = uuid::Uuid::new_v4();\n    let xml = format!(r#\"<{op_name}Response xmlns=\"{ns}\">\\n  <requestId>{{request_id}}</requestId>\\n  <return>true</return>\\n</{op_name}Response>\"#);\n    MockResponse::xml(200, xml)\n}}\n\n"
            ));
        }
        Protocol::RestXml => {
            out.push_str(&format!(
                "/// Serialize void response for restXml protocol.\npub fn {fn_name}() -> MockResponse {{\n    MockResponse::xml({http_code}, \"\")\n}}\n\n"
            ));
        }
        Protocol::RpcV2Cbor => {
            out.push_str(&format!(
                "/// Serialize void response for rpc-v2-cbor protocol.\npub fn {fn_name}() -> MockResponse {{\n    let mut buf = Vec::new();\n    ciborium::into_writer(&ciborium::Value::Map(vec![]), &mut buf).unwrap();\n    MockResponse::cbor({http_code}, buf)\n}}\n\n"
            ));
        }
    }
}

// --- awsJson serializer ---

fn generate_aws_json_serializer(
    out: &mut String,
    fn_name: &str,
    http_code: u16,
    struct_name: &str,
) {
    out.push_str(&format!(
        r#"/// Serialize response for awsJson protocol.
pub fn {fn_name}(result: &{struct_name}) -> MockResponse {{
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{{}}".to_string());
    MockResponse::json({http_code}, body)
}}

"#
    ));
}

// --- restJson serializer ---

fn generate_rest_json_serializer(
    out: &mut String,
    fn_name: &str,
    http_code: u16,
    members: &[ShapeMember],
    struct_name: &str,
    model: &ServiceModel,
) {
    let header_members: Vec<&ShapeMember> =
        members.iter().filter(|m| m.http_header.is_some()).collect();
    let response_code_member = members.iter().find(|m| m.http_response_code);
    let payload_member = members.iter().find(|m| m.http_payload);

    // When an output member carries `@httpPayload`, the restJson1 wire format
    // places the value of that member directly in the response body instead
    // of wrapping it inside the response struct.  Detect the case where the
    // payload is a structure and emit the field's contents as the body.
    // For non-structure payloads (string/blob/document), fall back to the
    // generic struct serialisation; those shapes are uncommon and the
    // existing behaviour is preserved.
    let payload_struct_field: Option<(String, bool)> =
        payload_member.and_then(|p| match model.resolve_type(&p.target) {
            ResolvedType::Structure(_) => Some((to_snake_case(&p.name), p.required)),
            _ => None,
        });

    out.push_str("/// Serialize response for restJson protocol.\n");
    out.push_str(&format!(
        "pub fn {fn_name}(result: &{struct_name}) -> MockResponse {{\n"
    ));

    // Determine status code
    if let Some(rc_member) = response_code_member {
        let field_name = to_snake_case(&rc_member.name);
        out.push_str(&format!(
            "    let status = result.{field_name}.unwrap_or({http_code}) as u16;\n"
        ));
    } else {
        out.push_str(&format!("    let status = {http_code}_u16;\n"));
    }

    if let Some((field_name, required)) = &payload_struct_field {
        if *required {
            out.push_str(&format!(
                "    let body = serde_json::to_string(&result.{field_name}).unwrap_or_else(|_| \"{{}}\".to_string());\n"
            ));
        } else {
            out.push_str(&format!("    let body = match &result.{field_name} {{\n"));
            out.push_str(
                "        Some(v) => serde_json::to_string(v).unwrap_or_else(|_| \"{}\".to_string()),\n",
            );
            out.push_str("        None => String::new(),\n");
            out.push_str("    };\n");
        }
    } else {
        out.push_str(
            "    let body = serde_json::to_string(result).unwrap_or_else(|_| \"{}\".to_string());\n",
        );
    }

    if header_members.is_empty() {
        out.push_str("    MockResponse::rest_json(status, body)\n");
        out.push_str("}\n\n");
        return;
    }

    out.push_str("    let resp = MockResponse::rest_json(status, body);\n");

    // Set headers from struct fields
    for hm in &header_members {
        let header_name = hm.http_header.as_ref().unwrap();
        let field_name = to_snake_case(&hm.name);
        out.push_str(&format!(
            "    // Header \"{}\": set by caller from {field_name} field\n",
            header_name.to_lowercase()
        ));
    }

    out.push_str("    resp\n");
    out.push_str("}\n\n");
}

// --- awsQuery XML serializer (serde + quick-xml) ---

fn generate_aws_query_serializer(
    out: &mut String,
    fn_name: &str,
    op_name: &str,
    namespace: &str,
    struct_name: &str,
) {
    out.push_str("/// Serialize response for awsQuery protocol.\n");
    out.push_str(&format!(
        "pub fn {fn_name}(\n    result: &{struct_name},\n) -> MockResponse {{\n"
    ));
    // Serialize the struct, then strip the outer element (whose name comes from the struct's
    // serde rename, which may not match the expected operation-specific Result wrapper name).
    // Re-wrap with the correct <{op_name}Result> element.
    out.push_str(
        "    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();\n",
    );
    out.push_str(
        "    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.\n",
    );
    out.push_str("    let inner_xml = strip_outer_element(&result_xml_wrapped);\n");
    out.push_str(&format!(
        "    let result_xml = format!(\n        \"<{op_name}Result>{{inner_xml}}</{op_name}Result>\"\n    );\n"
    ));
    out.push_str("    let request_id = uuid::Uuid::new_v4();\n");
    out.push_str(&format!(
        "    let xml = format!(\n        r#\"<{op_name}Response xmlns=\"{namespace}\">\n  {{result_xml}}\n  <ResponseMetadata>\n    <RequestId>{{request_id}}</RequestId>\n  </ResponseMetadata>\n</{op_name}Response>\"#\n    );\n"
    ));
    out.push_str("    MockResponse::xml(200, xml)\n");
    out.push_str("}\n\n");
}

// --- ec2Query XML serializer (serde + quick-xml, no Result wrapper) ---

fn generate_ec2_query_serializer(
    out: &mut String,
    fn_name: &str,
    op_name: &str,
    namespace: &str,
    struct_name: &str,
) {
    out.push_str("/// Serialize response for ec2Query protocol.\n");
    out.push_str(&format!(
        "pub fn {fn_name}(result: &{struct_name}) -> MockResponse {{\n"
    ));
    // Serialize the struct and strip the outer element (e.g. <DescribeVpcsResult>...</DescribeVpcsResult>)
    // so that the inner fields are placed directly inside the Response element.
    out.push_str(
        "    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();\n",
    );
    out.push_str(
        "    // Strip the outer wrapper element (e.g. <OpNameResult>...</OpNameResult>)\n",
    );
    out.push_str("    let result_xml = strip_outer_element(&result_xml_wrapped);\n");
    out.push_str("    let request_id = uuid::Uuid::new_v4();\n");
    out.push_str(&format!(
        "    let xml = format!(r#\"<{op_name}Response xmlns=\"{namespace}\">\n  <requestId>{{request_id}}</requestId>\n  {{result_xml}}\n</{op_name}Response>\"#);\n"
    ));
    out.push_str("    MockResponse::xml(200, xml)\n");
    out.push_str("}\n\n");
}

// --- restXml serializer (serde + quick-xml) ---

#[allow(clippy::too_many_arguments)]
fn generate_rest_xml_serializer(
    out: &mut String,
    fn_name: &str,
    http_code: u16,
    _output_shape: &str,
    members: &[ShapeMember],
    model: &ServiceModel,
    struct_name: &str,
    structs_needed: &mut std::collections::HashSet<String>,
) {
    let header_members: Vec<&ShapeMember> =
        members.iter().filter(|m| m.http_header.is_some()).collect();
    let payload_member = members.iter().find(|m| m.http_payload);

    // For restXml, if there's an httpPayload member pointing to a structure,
    // that structure is the XML body.
    let body_shape = if let Some(payload) = payload_member {
        let resolved = model.resolve_type(&payload.target);
        match resolved {
            ResolvedType::Structure(ref s) => {
                structs_needed.insert(s.clone());
                let payload_struct = escape_type_name(ServiceModel::short_name(s));
                Some((payload.name.clone(), payload_struct.to_string()))
            }
            _ => None,
        }
    } else {
        let body_members: Vec<&ShapeMember> = members
            .iter()
            .filter(|m| {
                m.http_header.is_none()
                    && !m.http_response_code
                    && !m.http_label
                    && m.http_query.is_none()
                    && !m.http_payload
            })
            .collect();
        if !body_members.is_empty() {
            Some(("".to_string(), struct_name.to_string()))
        } else {
            None
        }
    };

    out.push_str("/// Serialize response for restXml protocol.\n");

    match &body_shape {
        Some((field_name, type_name)) if !field_name.is_empty() => {
            out.push_str(&format!(
                "pub fn {fn_name}(result: &{type_name}) -> MockResponse {{\n"
            ));
            out.push_str("    let body = quick_xml::se::to_string(result).unwrap_or_default();\n");
        }
        Some((_, _)) => {
            out.push_str(&format!(
                "pub fn {fn_name}(result: &{struct_name}) -> MockResponse {{\n"
            ));
            out.push_str("    let body = quick_xml::se::to_string(result).unwrap_or_default();\n");
        }
        None => {
            out.push_str(&format!("pub fn {fn_name}() -> MockResponse {{\n"));
            out.push_str("    let body = String::new();\n");
        }
    }

    if header_members.is_empty() {
        out.push_str(&format!("    MockResponse::xml({http_code}, body)\n"));
        out.push_str("}\n\n");
        return;
    }

    out.push_str(&format!(
        "    let resp = MockResponse::xml({http_code}, body);\n"
    ));

    for hm in &header_members {
        let header_name = hm.http_header.as_ref().unwrap();
        let field_name = to_snake_case(&hm.name);
        out.push_str(&format!(
            "    // Header \"{}\": set by caller from {field_name} field\n",
            header_name.to_lowercase()
        ));
    }

    out.push_str("    resp\n");
    out.push_str("}\n\n");
}

fn generate_rest_xml_request_deserializer(
    out: &mut String,
    op_name: &str,
    input_shape: &str,
    model: &ServiceModel,
) {
    let fn_name = format!("deserialize_{}_request", to_snake_case(op_name));
    let struct_name = escape_type_name(ServiceModel::short_name(input_shape));
    let members = model.get_members(input_shape);
    let payload_member = members.iter().find(|m| m.http_payload);
    let body_members: Vec<&ShapeMember> = members
        .iter()
        .filter(|m| {
            m.http_header.is_none()
                && !m.http_label
                && m.http_query.is_none()
                && !m.http_payload
                && !m.http_response_code
        })
        .collect();
    let label_members: Vec<&ShapeMember> = members.iter().filter(|m| m.http_label).collect();
    let query_members: Vec<&ShapeMember> =
        members.iter().filter(|m| m.http_query.is_some()).collect();
    let header_members: Vec<&ShapeMember> =
        members.iter().filter(|m| m.http_header.is_some()).collect();

    out.push_str("/// Deserialize request for restXml protocol.\n");
    out.push_str(&format!(
        "pub fn {fn_name}(\n    request: &winterbaume_core::MockRequest,\n    labels: &[(&str, &str)],\n    query: &std::collections::HashMap<String, String>,\n) -> Result<{struct_name}, String> {{\n"
    ));
    if members.is_empty() {
        out.push_str(&format!("    let input = {struct_name} {{}};\n"));
    } else {
        out.push_str(&format!("    let mut input = {struct_name}::default();\n"));
    }

    if let Some(payload) = payload_member {
        let resolved = model.resolve_type(&payload.target);
        let field_name = to_snake_case(&payload.name);
        match resolved {
            ResolvedType::Structure(ref s) => {
                let payload_struct = escape_type_name(ServiceModel::short_name(s));
                out.push_str("    if !request.body.is_empty() {\n");
                out.push_str("        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;\n");
                out.push_str(&format!(
                    "        let payload = quick_xml::de::from_str::<{payload_struct}>(body)\n            .map_err(|err| format!(\"failed to deserialize {op_name} payload: {{err}}\"))?;\n"
                ));
                if payload.required {
                    out.push_str(&format!("        input.{field_name} = payload;\n"));
                } else {
                    out.push_str(&format!("        input.{field_name} = Some(payload);\n"));
                }
                out.push_str("    }\n");
            }
            _ => {
                let value_expr = binding_value_expr_for_member(payload, model, "body");
                out.push_str("    if !request.body.is_empty() {\n");
                out.push_str("        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;\n");
                out.push_str(&format!(
                    "        {};\n",
                    assign_member_value("input", &field_name, payload.required, &value_expr)
                ));
                out.push_str("    }\n");
            }
        }
    } else if !body_members.is_empty() {
        out.push_str("    if !request.body.is_empty() {\n");
        out.push_str("        let body = std::str::from_utf8(&request.body).map_err(|err| err.to_string())?;\n");
        out.push_str(&format!(
            "        input = quick_xml::de::from_str::<{struct_name}>(body)\n            .map_err(|err| format!(\"failed to deserialize {op_name} request: {{err}}\"))?;\n"
        ));
        out.push_str("    }\n");
    }

    if !label_members.is_empty() {
        out.push_str("    for (name, value) in labels {\n");
        out.push_str("        match *name {\n");
        for member in &label_members {
            let field_name = to_snake_case(&member.name);
            let value_expr = binding_value_expr_for_member(member, model, "value");
            out.push_str(&format!(
                "            \"{}\" => {{ {};}},\n",
                member.name,
                assign_member_value("input", &field_name, member.required, &value_expr)
            ));
        }
        out.push_str("            _ => {}\n");
        out.push_str("        }\n");
        out.push_str("    }\n");
    }

    for member in &query_members {
        let field_name = to_snake_case(&member.name);
        let value_expr = binding_value_expr_for_member(member, model, "value");
        let query_name = member.http_query.as_ref().unwrap();
        out.push_str(&format!(
            "    if let Some(value) = query.get(\"{query_name}\") {{ {};\n    }}\n",
            assign_member_value("input", &field_name, member.required, &value_expr)
        ));
    }

    for member in &header_members {
        let field_name = to_snake_case(&member.name);
        let value_expr = binding_value_expr_for_member(member, model, "value");
        let header_name = member.http_header.as_ref().unwrap();
        out.push_str(&format!(
            "    if let Some(value) = request.headers.get(\"{header_name}\").and_then(|value| value.to_str().ok()) {{ {};\n    }}\n",
            assign_member_value("input", &field_name, member.required, &value_expr)
        ));
    }

    out.push_str("    Ok(input)\n");
    out.push_str("}\n\n");
}

// --- awsQuery/ec2Query request deserializer ---

/// Collect all nested structure types that need sub-deserializer functions.
fn collect_struct_deserializers(
    shape_fqn: &str,
    model: &ServiceModel,
    needed: &mut std::collections::HashSet<String>,
) {
    let members = model.get_members(shape_fqn);
    for member in &members {
        let resolved = model.resolve_type(&member.target);
        match &resolved {
            ResolvedType::List(inner) => {
                if let ResolvedType::Structure(s) = inner.as_ref()
                    && needed.insert(s.clone())
                {
                    collect_struct_deserializers(s, model, needed);
                }
            }
            ResolvedType::Structure(s) if needed.insert(s.clone()) => {
                // Only collect if it's not the top-level input shape itself
                collect_struct_deserializers(s, model, needed);
            }
            _ => {}
        }
    }
}

/// Generate a sub-deserializer for a nested structure in awsQuery format.
/// These are called as `deserialize_{short_name}_from_query(params, prefix)`.
fn generate_aws_query_struct_deserializer(out: &mut String, shape_fqn: &str, model: &ServiceModel) {
    let short = ServiceModel::short_name(shape_fqn);
    let struct_name = escape_type_name(short);
    let fn_name = format!("deserialize_{}_from_query", to_snake_case(short));
    let members = model.get_members(shape_fqn);

    out.push_str(&format!(
        "fn {fn_name}(\n    params: &std::collections::HashMap<String, String>,\n    prefix: &str,\n) -> Result<Option<{struct_name}>, String> {{\n"
    ));
    out.push_str(&format!(
        "    let mut item = {struct_name}::default();\n    let mut found = false;\n"
    ));

    for member in &members {
        let field_name = to_snake_case(&member.name);
        let query_key = &member.name;
        let resolved = model.resolve_type(&member.target);
        generate_aws_query_member_deser(
            out,
            &field_name,
            query_key,
            &resolved,
            member,
            model,
            "prefix",
        );
    }

    out.push_str("    Ok(if found { Some(item) } else { None })\n");
    out.push_str("}\n\n");
}

/// Generate the top-level awsQuery request deserializer for an operation.
#[allow(dead_code)]
fn generate_aws_query_request_deserializer(
    out: &mut String,
    op_name: &str,
    input_shape: &str,
    model: &ServiceModel,
) {
    generate_aws_query_request_deserializer_with_suffix(out, op_name, input_shape, model, "");
}

fn generate_aws_query_request_deserializer_with_suffix(
    out: &mut String,
    op_name: &str,
    input_shape: &str,
    model: &ServiceModel,
    fn_suffix: &str,
) {
    let fn_name = format!("deserialize_{}_request{fn_suffix}", to_snake_case(op_name));
    let struct_name = escape_type_name(ServiceModel::short_name(input_shape));
    let members = model.get_members(input_shape);

    out.push_str(&format!(
        "/// Deserialize awsQuery request for {op_name}.\n"
    ));
    out.push_str(&format!(
        "pub fn {fn_name}(\n    params: &std::collections::HashMap<String, String>,\n) -> Result<{struct_name}, String> {{\n"
    ));
    if members.is_empty() {
        out.push_str(&format!("    let input = {struct_name} {{}};\n"));
    } else {
        out.push_str(&format!("    let mut input = {struct_name}::default();\n"));
    }

    for member in &members {
        let field_name = to_snake_case(&member.name);
        let query_key = &member.name;
        let resolved = model.resolve_type(&member.target);
        generate_aws_query_member_deser_top(out, &field_name, query_key, &resolved, member, model);
    }

    out.push_str("    Ok(input)\n");
    out.push_str("}\n\n");
}

/// Generate deserialization code for a single member in an awsQuery top-level input.
fn generate_aws_query_member_deser_top(
    out: &mut String,
    field_name: &str,
    query_key: &str,
    resolved: &ResolvedType,
    member: &ShapeMember,
    model: &ServiceModel,
) {
    match resolved {
        ResolvedType::String | ResolvedType::Enum(_) => {
            out.push_str(&format!(
                "    if let Some(value) = params.get(\"{query_key}\") {{\n"
            ));
            if member.required {
                out.push_str(&format!(
                    "        input.{field_name} = value.to_string();\n"
                ));
            } else {
                out.push_str(&format!(
                    "        input.{field_name} = Some(value.to_string());\n"
                ));
            }
            out.push_str("    }\n");
        }
        ResolvedType::Timestamp => {
            // For XML-primary protocols, timestamps are ISO 8601 strings;
            // for non-XML protocols they are epoch-seconds f64 values.
            out.push_str(&format!(
                "    if let Some(value) = params.get(\"{query_key}\") {{\n"
            ));
            if model.is_xml_protocol() {
                if member.required {
                    out.push_str(&format!(
                        "        input.{field_name} = value.to_string();\n"
                    ));
                } else {
                    out.push_str(&format!(
                        "        input.{field_name} = Some(value.to_string());\n"
                    ));
                }
            } else if member.required {
                out.push_str(&format!(
                    "        input.{field_name} = value\n            .parse::<f64>()\n            .map_err(|e| format!(\"failed to parse {query_key}: {{e}}\"))?;\n"
                ));
            } else {
                out.push_str(&format!(
                    "        input.{field_name} = Some(\n            value\n                .parse::<f64>()\n                .map_err(|e| format!(\"failed to parse {query_key}: {{e}}\"))?,\n        );\n"
                ));
            }
            out.push_str("    }\n");
        }
        ResolvedType::Boolean => {
            out.push_str(&format!(
                "    if let Some(value) = params.get(\"{query_key}\") {{\n"
            ));
            if member.required {
                out.push_str(&format!(
                    "        input.{field_name} = value.parse::<bool>().map_err(|e| format!(\"failed to parse {query_key}: {{e}}\"))?;\n"
                ));
            } else {
                out.push_str(&format!(
                    "        input.{field_name} = Some(value.parse::<bool>().map_err(|e| format!(\"failed to parse {query_key}: {{e}}\"))?);\n"
                ));
            }
            out.push_str("    }\n");
        }
        ResolvedType::Integer => {
            out.push_str(&format!(
                "    if let Some(value) = params.get(\"{query_key}\") {{\n"
            ));
            if member.required {
                out.push_str(&format!(
                    "        input.{field_name} = value\n            .parse::<i32>()\n            .map_err(|e| format!(\"failed to parse {query_key}: {{e}}\"))?;\n"
                ));
            } else {
                out.push_str(&format!(
                    "        input.{field_name} = Some(\n            value\n                .parse::<i32>()\n                .map_err(|e| format!(\"failed to parse {query_key}: {{e}}\"))?,\n        );\n"
                ));
            }
            out.push_str("    }\n");
        }
        ResolvedType::Long => {
            out.push_str(&format!(
                "    if let Some(value) = params.get(\"{query_key}\") {{\n"
            ));
            if member.required {
                out.push_str(&format!(
                    "        input.{field_name} = value\n            .parse::<i64>()\n            .map_err(|e| format!(\"failed to parse {query_key}: {{e}}\"))?;\n"
                ));
            } else {
                out.push_str(&format!(
                    "        input.{field_name} = Some(\n            value\n                .parse::<i64>()\n                .map_err(|e| format!(\"failed to parse {query_key}: {{e}}\"))?,\n        );\n"
                ));
            }
            out.push_str("    }\n");
        }
        ResolvedType::Float | ResolvedType::Double => {
            out.push_str(&format!(
                "    if let Some(value) = params.get(\"{query_key}\") {{\n"
            ));
            if member.required {
                out.push_str(&format!(
                    "        input.{field_name} = value\n            .parse::<f64>()\n            .map_err(|e| format!(\"failed to parse {query_key}: {{e}}\"))?;\n"
                ));
            } else {
                out.push_str(&format!(
                    "        input.{field_name} = Some(\n            value\n                .parse::<f64>()\n                .map_err(|e| format!(\"failed to parse {query_key}: {{e}}\"))?,\n        );\n"
                ));
            }
            out.push_str("    }\n");
        }
        ResolvedType::List(inner) => {
            // For XML-primary protocols, list fields use named wrapper structs
            // (e.g. `AlarmNames { items: Vec<String> }`).  For non-XML protocols
            // the model emits plain `Vec<T>`, so we must not reference wrappers.
            let use_wrapper = model.is_xml_protocol();
            let wrapper_name = if use_wrapper {
                Some(escape_type_name(ServiceModel::short_name(&member.target)))
            } else {
                None
            };
            // Determine the item key name from the list member's xmlName
            let item_name = model
                .get_list_member_xml_name(&member.target)
                .unwrap_or_else(|| "member".to_string());

            match inner.as_ref() {
                ResolvedType::Structure(s) => {
                    let deser_fn = format!(
                        "deserialize_{}_from_query",
                        to_snake_case(ServiceModel::short_name(s))
                    );
                    out.push_str("    {\n");
                    out.push_str("        let mut items = Vec::new();\n");
                    out.push_str(&format!(
                        "        let list_prefix = \"{query_key}\".to_string();\n"
                    ));
                    out.push_str("        for i in 1.. {\n");
                    out.push_str(&format!(
                        "            let item_key = format!(\"{{list_prefix}}.{item_name}.{{i}}\");\n"
                    ));
                    out.push_str(&format!(
                        "            match {deser_fn}(params, &item_key)? {{\n"
                    ));
                    out.push_str("                Some(item) => items.push(item),\n");
                    out.push_str("                None => break,\n");
                    out.push_str("            }\n");
                    out.push_str("        }\n");
                    if member.required {
                        out.push_str("        if !items.is_empty() {\n");
                        if let Some(ref w) = wrapper_name {
                            out.push_str(&format!(
                                "            input.{field_name} = {w} {{ items }};\n"
                            ));
                        } else {
                            out.push_str(&format!("            input.{field_name} = items;\n"));
                        }
                        out.push_str("        }\n");
                    } else {
                        out.push_str("        if !items.is_empty() {\n");
                        if let Some(ref w) = wrapper_name {
                            out.push_str(&format!(
                                "            input.{field_name} = Some({w} {{ items }});\n"
                            ));
                        } else {
                            out.push_str(&format!(
                                "            input.{field_name} = Some(items);\n"
                            ));
                        }
                        out.push_str("        }\n");
                    }
                    out.push_str("    }\n");
                }
                _ => {
                    // Simple list (list of strings, ints, etc.)
                    let item_expr = match inner.as_ref() {
                        ResolvedType::Integer => "value.parse::<i32>().map_err(|e| format!(\"failed to parse list item: {e}\"))?".to_string(),
                        ResolvedType::Long => "value.parse::<i64>().map_err(|e| format!(\"failed to parse list item: {e}\"))?".to_string(),
                        ResolvedType::Float => "value.parse::<f32>().map_err(|e| format!(\"failed to parse list item: {e}\"))?".to_string(),
                        ResolvedType::Double => "value.parse::<f64>().map_err(|e| format!(\"failed to parse list item: {e}\"))?".to_string(),
                        ResolvedType::Boolean => "value.parse::<bool>().map_err(|e| format!(\"failed to parse list item: {e}\"))?".to_string(),
                        _ => "value.to_string()".to_string(),
                    };
                    out.push_str("    {\n");
                    out.push_str("        let mut items = Vec::new();\n");
                    out.push_str(&format!(
                        "        let list_prefix = \"{query_key}\".to_string();\n"
                    ));
                    out.push_str("        for i in 1.. {\n");
                    out.push_str(&format!(
                        "            let item_key = format!(\"{{list_prefix}}.{item_name}.{{i}}\");\n"
                    ));
                    out.push_str("            match params.get(&item_key) {\n");
                    out.push_str("                Some(value) => items.push(");
                    out.push_str(&item_expr);
                    out.push_str("),\n");
                    out.push_str("                None => break,\n");
                    out.push_str("            }\n");
                    out.push_str("        }\n");
                    if member.required {
                        out.push_str("        if !items.is_empty() {\n");
                        if let Some(ref w) = wrapper_name {
                            out.push_str(&format!(
                                "            input.{field_name} = {w} {{ items }};\n"
                            ));
                        } else {
                            out.push_str(&format!("            input.{field_name} = items;\n"));
                        }
                        out.push_str("        }\n");
                    } else {
                        out.push_str("        if !items.is_empty() {\n");
                        if let Some(ref w) = wrapper_name {
                            out.push_str(&format!(
                                "            input.{field_name} = Some({w} {{ items }});\n"
                            ));
                        } else {
                            out.push_str(&format!(
                                "            input.{field_name} = Some(items);\n"
                            ));
                        }
                        out.push_str("        }\n");
                    }
                    out.push_str("    }\n");
                }
            }
        }
        ResolvedType::Structure(s) => {
            let deser_fn = format!(
                "deserialize_{}_from_query",
                to_snake_case(ServiceModel::short_name(s))
            );
            if member.required {
                out.push_str(&format!(
                    "    if let Some(val) = {deser_fn}(params, \"{query_key}\")? {{\n        input.{field_name} = val;\n    }}\n"
                ));
            } else {
                out.push_str(&format!(
                    "    if let Some(val) = {deser_fn}(params, \"{query_key}\")? {{\n        input.{field_name} = Some(val);\n    }}\n"
                ));
            }
        }
        _ => {
            // Blob, Document, Map, Unknown - skip for now
        }
    }
}

/// Generate deserialization code for a single member in an awsQuery nested structure.
/// Uses `prefix` variable for composing the param key.
fn generate_aws_query_member_deser(
    out: &mut String,
    field_name: &str,
    query_key: &str,
    resolved: &ResolvedType,
    member: &ShapeMember,
    model: &ServiceModel,
    prefix_var: &str,
) {
    let wrap_some = !member.required;
    match resolved {
        ResolvedType::String | ResolvedType::Enum(_) => {
            out.push_str(&format!(
                "    if let Some(value) = params.get(&format!(\"{{{prefix_var}}}.{query_key}\")) {{\n"
            ));
            if wrap_some {
                out.push_str(&format!(
                    "        item.{field_name} = Some(value.to_string());\n        found = true;\n"
                ));
            } else {
                out.push_str(&format!(
                    "        item.{field_name} = value.to_string();\n        found = true;\n"
                ));
            }
            out.push_str("    }\n");
        }
        ResolvedType::Timestamp => {
            out.push_str(&format!(
                "    if let Some(value) = params.get(&format!(\"{{{prefix_var}}}.{query_key}\")) {{\n"
            ));
            if model.is_xml_protocol() {
                if wrap_some {
                    out.push_str(&format!(
                        "        item.{field_name} = Some(value.to_string());\n        found = true;\n"
                    ));
                } else {
                    out.push_str(&format!(
                        "        item.{field_name} = value.to_string();\n        found = true;\n"
                    ));
                }
            } else if wrap_some {
                out.push_str(&format!(
                    "        item.{field_name} = Some(\n            value\n                .parse::<f64>()\n                .map_err(|e| format!(\"failed to parse {query_key}: {{e}}\"))?,\n        );\n        found = true;\n"
                ));
            } else {
                out.push_str(&format!(
                    "        item.{field_name} = value\n            .parse::<f64>()\n            .map_err(|e| format!(\"failed to parse {query_key}: {{e}}\"))?;\n        found = true;\n"
                ));
            }
            out.push_str("    }\n");
        }
        ResolvedType::Boolean => {
            out.push_str(&format!(
                "    if let Some(value) = params.get(&format!(\"{{{prefix_var}}}.{query_key}\")) {{\n"
            ));
            if wrap_some {
                out.push_str(&format!(
                    "        item.{field_name} = Some(value.parse::<bool>().map_err(|e| format!(\"failed to parse {query_key}: {{e}}\"))?);\n        found = true;\n"
                ));
            } else {
                out.push_str(&format!(
                    "        item.{field_name} = value.parse::<bool>().map_err(|e| format!(\"failed to parse {query_key}: {{e}}\"))?;\n        found = true;\n"
                ));
            }
            out.push_str("    }\n");
        }
        ResolvedType::Integer => {
            out.push_str(&format!(
                "    if let Some(value) = params.get(&format!(\"{{{prefix_var}}}.{query_key}\")) {{\n"
            ));
            if wrap_some {
                out.push_str(&format!(
                    "        item.{field_name} = Some(\n            value\n                .parse::<i32>()\n                .map_err(|e| format!(\"failed to parse {query_key}: {{e}}\"))?,\n        );\n        found = true;\n"
                ));
            } else {
                out.push_str(&format!(
                    "        item.{field_name} = value\n            .parse::<i32>()\n            .map_err(|e| format!(\"failed to parse {query_key}: {{e}}\"))?;\n        found = true;\n"
                ));
            }
            out.push_str("    }\n");
        }
        ResolvedType::Long => {
            out.push_str(&format!(
                "    if let Some(value) = params.get(&format!(\"{{{prefix_var}}}.{query_key}\")) {{\n"
            ));
            if wrap_some {
                out.push_str(&format!(
                    "        item.{field_name} = Some(\n            value\n                .parse::<i64>()\n                .map_err(|e| format!(\"failed to parse {query_key}: {{e}}\"))?,\n        );\n        found = true;\n"
                ));
            } else {
                out.push_str(&format!(
                    "        item.{field_name} = value\n            .parse::<i64>()\n            .map_err(|e| format!(\"failed to parse {query_key}: {{e}}\"))?;\n        found = true;\n"
                ));
            }
            out.push_str("    }\n");
        }
        ResolvedType::Float | ResolvedType::Double => {
            out.push_str(&format!(
                "    if let Some(value) = params.get(&format!(\"{{{prefix_var}}}.{query_key}\")) {{\n"
            ));
            if wrap_some {
                out.push_str(&format!(
                    "        item.{field_name} = Some(\n            value\n                .parse::<f64>()\n                .map_err(|e| format!(\"failed to parse {query_key}: {{e}}\"))?,\n        );\n        found = true;\n"
                ));
            } else {
                out.push_str(&format!(
                    "        item.{field_name} = value\n            .parse::<f64>()\n            .map_err(|e| format!(\"failed to parse {query_key}: {{e}}\"))?;\n        found = true;\n"
                ));
            }
            out.push_str("    }\n");
        }
        ResolvedType::List(inner) => {
            // List inside a nested struct.
            // For XML-primary protocols, list fields use named wrapper structs;
            // for non-XML protocols the model emits plain `Vec<T>`.
            let use_wrapper = model.is_xml_protocol();
            let wrapper_name = if use_wrapper {
                Some(escape_type_name(ServiceModel::short_name(&member.target)))
            } else {
                None
            };
            let item_name = model
                .get_list_member_xml_name(&member.target)
                .unwrap_or_else(|| "member".to_string());
            match inner.as_ref() {
                ResolvedType::Structure(s) => {
                    let deser_fn = format!(
                        "deserialize_{}_from_query",
                        to_snake_case(ServiceModel::short_name(s))
                    );
                    out.push_str("    {\n");
                    out.push_str("        let mut sub_items = Vec::new();\n");
                    out.push_str(&format!(
                        "        let sub_list_prefix = format!(\"{{{prefix_var}}}.{query_key}\");\n"
                    ));
                    out.push_str("        for i in 1.. {\n");
                    out.push_str(&format!(
                        "            let item_key = format!(\"{{sub_list_prefix}}.{item_name}.{{i}}\");\n"
                    ));
                    out.push_str(&format!(
                        "            match {deser_fn}(params, &item_key)? {{\n"
                    ));
                    out.push_str("                Some(sub_item) => sub_items.push(sub_item),\n");
                    out.push_str("                None => break,\n");
                    out.push_str("            }\n");
                    out.push_str("        }\n");
                    out.push_str("        if !sub_items.is_empty() {\n");
                    if wrap_some {
                        if let Some(ref w) = wrapper_name {
                            out.push_str(&format!(
                                "            item.{field_name} = Some({w} {{ items: sub_items }});\n"
                            ));
                        } else {
                            out.push_str(&format!(
                                "            item.{field_name} = Some(sub_items);\n"
                            ));
                        }
                    } else if let Some(ref w) = wrapper_name {
                        out.push_str(&format!(
                            "            item.{field_name} = {w} {{ items: sub_items }};\n"
                        ));
                    } else {
                        out.push_str(&format!("            item.{field_name} = sub_items;\n"));
                    }
                    out.push_str("            found = true;\n");
                    out.push_str("        }\n");
                    out.push_str("    }\n");
                }
                _ => {
                    let item_expr = match inner.as_ref() {
                        ResolvedType::Integer => "value.parse::<i32>().map_err(|e| format!(\"failed to parse list item: {e}\"))?".to_string(),
                        ResolvedType::Long => "value.parse::<i64>().map_err(|e| format!(\"failed to parse list item: {e}\"))?".to_string(),
                        ResolvedType::Float => "value.parse::<f32>().map_err(|e| format!(\"failed to parse list item: {e}\"))?".to_string(),
                        ResolvedType::Double => "value.parse::<f64>().map_err(|e| format!(\"failed to parse list item: {e}\"))?".to_string(),
                        ResolvedType::Boolean => "value.parse::<bool>().map_err(|e| format!(\"failed to parse list item: {e}\"))?".to_string(),
                        _ => "value.to_string()".to_string(),
                    };
                    out.push_str("    {\n");
                    out.push_str("        let mut sub_items = Vec::new();\n");
                    out.push_str(&format!(
                        "        let sub_list_prefix = format!(\"{{{prefix_var}}}.{query_key}\");\n"
                    ));
                    out.push_str("        for i in 1.. {\n");
                    out.push_str(&format!(
                        "            let item_key = format!(\"{{sub_list_prefix}}.{item_name}.{{i}}\");\n"
                    ));
                    out.push_str("            match params.get(&item_key) {\n");
                    out.push_str("                Some(value) => sub_items.push(");
                    out.push_str(&item_expr);
                    out.push_str("),\n");
                    out.push_str("                None => break,\n");
                    out.push_str("            }\n");
                    out.push_str("        }\n");
                    out.push_str("        if !sub_items.is_empty() {\n");
                    if wrap_some {
                        if let Some(ref w) = wrapper_name {
                            out.push_str(&format!(
                                "            item.{field_name} = Some({w} {{ items: sub_items }});\n"
                            ));
                        } else {
                            out.push_str(&format!(
                                "            item.{field_name} = Some(sub_items);\n"
                            ));
                        }
                    } else if let Some(ref w) = wrapper_name {
                        out.push_str(&format!(
                            "            item.{field_name} = {w} {{ items: sub_items }};\n"
                        ));
                    } else {
                        out.push_str(&format!("            item.{field_name} = sub_items;\n"));
                    }
                    out.push_str("            found = true;\n");
                    out.push_str("        }\n");
                    out.push_str("    }\n");
                }
            }
        }
        ResolvedType::Structure(s) => {
            let deser_fn = format!(
                "deserialize_{}_from_query",
                to_snake_case(ServiceModel::short_name(s))
            );
            out.push_str("    {\n");
            out.push_str(&format!(
                "        let sub_prefix = format!(\"{{{prefix_var}}}.{query_key}\");\n"
            ));
            out.push_str(&format!(
                "        if let Some(val) = {deser_fn}(params, &sub_prefix)? {{\n"
            ));
            if wrap_some {
                out.push_str(&format!("            item.{field_name} = Some(val);\n"));
            } else {
                out.push_str(&format!("            item.{field_name} = val;\n"));
            }
            out.push_str("            found = true;\n");
            out.push_str("        }\n");
            out.push_str("    }\n");
        }
        _ => {
            // Blob, Document, Map, Unknown - skip for now
        }
    }
}

// --- Recursive struct generation using serde ---

fn generate_struct_recursive(
    out: &mut String,
    shape_fqn: &str,
    model: &ServiceModel,
    input_shapes: &std::collections::HashSet<String>,
    visited: &mut std::collections::HashSet<String>,
    rename_overrides: &std::collections::HashMap<String, String>,
    shape_features: Option<&ShapeFeatures>,
) {
    if visited.contains(shape_fqn) {
        return;
    }
    visited.insert(shape_fqn.to_string());

    let raw_name = ServiceModel::short_name(shape_fqn);
    let short_name = escape_type_name(raw_name);
    let members = model.get_members(shape_fqn);

    let include_http_bindings = input_shapes.contains(shape_fqn);

    let cfg_attr = shape_features
        .map(|sf| cfg_attr_for_shape(sf.get(shape_fqn)))
        .unwrap_or_default();

    if members.is_empty() {
        // Use brace-form `{}` rather than unit-struct `;` so that serde can
        // deserialise the JSON representation `{}` that AWS SDKs emit for
        // empty-body shapes (e.g. QueryString, AllQueryArguments, Method).
        // A unit struct `Foo;` would fail to deserialise from `{}`.
        out.push_str(&cfg_attr);
        out.push_str("#[derive(Debug, Clone, Default, Deserialize, Serialize)]\n");
        out.push_str(&format!("pub struct {short_name} {{}}\n\n"));
        return;
    }

    let is_xml = model.is_xml_protocol();

    // For XML protocols, determine the XML root element name
    if is_xml {
        let xml_root_name = rename_overrides
            .get(shape_fqn)
            .cloned()
            .or_else(|| model.get_shape_xml_name(shape_fqn))
            .unwrap_or_else(|| short_name.to_string());

        out.push_str(&cfg_attr);
        out.push_str("#[derive(Debug, Clone, Default, Deserialize, Serialize)]\n");
        out.push_str(&format!("#[serde(rename = \"{xml_root_name}\")]\n"));
    } else {
        // JSON protocols don't need root rename
        out.push_str(&cfg_attr);
        out.push_str("#[derive(Debug, Clone, Default, Deserialize, Serialize)]\n");
    }
    out.push_str(&format!("pub struct {short_name} {{\n"));

    for m in &members {
        // Skip HTTP binding members for XML protocols
        if is_xml
            && !include_http_bindings
            && (m.http_header.is_some()
                || m.http_label
                || m.http_query.is_some()
                || m.http_response_code)
        {
            continue;
        }

        // For JSON protocols, skip httpLabel and httpQuery on non-input shapes
        // (they're in the URL, not body). On input shapes, keep them so the
        // generated request deserializer can bind them from labels/query.
        if !is_xml && !include_http_bindings && (m.http_label || m.http_query.is_some()) {
            continue;
        }

        let field_name = to_snake_case(&m.name);
        let resolved = model.resolve_type(&m.target);
        // Determine timestamp format: per-member override > protocol default
        let ts_format = m
            .timestamp_format
            .as_deref()
            .unwrap_or_else(|| model.default_timestamp_format());
        let epoch_seconds = ts_format == "epoch-seconds";
        let rpc_cbor_timestamp_list = matches!(model.protocol, Protocol::RpcV2Cbor)
            && epoch_seconds
            && is_timestamp_list_shape(model, &m.target);
        let rpc_cbor_timestamp_scalar = matches!(model.protocol, Protocol::RpcV2Cbor)
            && epoch_seconds
            && matches!(resolved, ResolvedType::Timestamp);

        // For XML non-flattened lists, use a member-wrapper container type instead of Vec<T>.
        // quick_xml serializes Vec items directly into the parent element without a wrapper tag;
        // a container struct with a field named "member" produces the required <member> elements.
        let rust_type = if is_xml && !m.xml_flattened && model.is_list_shape(&m.target) {
            escape_type_name(ServiceModel::short_name(&m.target))
        } else if is_xml && !m.xml_flattened {
            xml_aware_rust_type(&resolved, epoch_seconds)
        } else {
            resolved_type_to_rust_serde(&resolved, epoch_seconds)
        };

        // Box struct-typed fields that could form cycles (infinite-size types).
        let needs_box = needs_boxing(&resolved, shape_fqn, model);

        // Determine the wire name for serde rename
        let wire_name = if is_xml {
            m.xml_name.as_deref().unwrap_or(&m.name)
        } else {
            m.json_name.as_deref().unwrap_or(&m.name)
        };

        // Add serde rename if the wire name differs from the field name
        if wire_name != field_name {
            out.push_str(&format!("    #[serde(rename = \"{wire_name}\")]\n"));
        }

        let required_timestamp_serializer = if rpc_cbor_timestamp_list {
            Some("serialize_cbor_timestamp_vec")
        } else if rpc_cbor_timestamp_scalar {
            Some("serialize_cbor_timestamp")
        } else {
            None
        };

        // For shapes reachable from operation inputs, `@required` fields must
        // be non-optional so that serde deserialization of request bodies
        // yields concrete values.  For output-only shapes (not reachable from
        // any input), `@required` fields are also emitted as `Option<T>` with
        // `skip_serializing_if`: this makes `Default::default()` serialize to
        // `{}` instead of zero-value noise, which is needed when a handler
        // wants to return a legitimately empty nested object.
        if m.required && input_shapes.contains(shape_fqn) {
            out.push_str("    #[serde(default)]\n");
            if let Some(serializer) = required_timestamp_serializer {
                out.push_str(&format!(
                    "    #[serde(serialize_with = \"{serializer}\")]\n"
                ));
            }
            if needs_box {
                out.push_str(&format!("    pub {field_name}: Box<{rust_type}>,\n"));
            } else {
                out.push_str(&format!("    pub {field_name}: {rust_type},\n"));
            }
        } else {
            out.push_str("    #[serde(default)]\n");
            out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
            if rpc_cbor_timestamp_list {
                out.push_str(
                    "    #[serde(serialize_with = \"serialize_cbor_timestamp_vec_opt\")]\n",
                );
            } else if rpc_cbor_timestamp_scalar {
                out.push_str("    #[serde(serialize_with = \"serialize_cbor_timestamp_opt\")]\n");
            }
            if needs_box {
                out.push_str(&format!(
                    "    pub {field_name}: Option<Box<{rust_type}>>,\n"
                ));
            } else {
                out.push_str(&format!("    pub {field_name}: Option<{rust_type}>,\n"));
            }
        }
    }

    out.push_str("}\n\n");

    // Recursively generate structs for nested structures.
    // For XML protocol, also emit member-wrapper container structs for non-flattened lists.
    for m in &members {
        if is_xml && !m.xml_flattened && model.is_list_shape(&m.target) {
            generate_named_xml_list_wrapper(out, &m.target, model, visited, shape_features);
        }
        let resolved = model.resolve_type(&m.target);
        collect_nested_structs(
            &resolved,
            model,
            out,
            input_shapes,
            visited,
            rename_overrides,
            is_xml,
            m.xml_flattened,
            shape_features,
        );
    }
}

fn is_timestamp_list_shape(model: &ServiceModel, shape_fqn: &str) -> bool {
    let Some(inner_target) = model.get_list_member_target(shape_fqn) else {
        return false;
    };
    matches!(model.resolve_type(&inner_target), ResolvedType::Timestamp)
}

fn generate_named_xml_list_wrapper(
    out: &mut String,
    list_shape_fqn: &str,
    model: &ServiceModel,
    visited: &mut std::collections::HashSet<String>,
    shape_features: Option<&ShapeFeatures>,
) {
    let key = format!("__named_list_shape_{list_shape_fqn}");
    if !visited.insert(key) {
        return;
    }

    let Some(inner_target) = model.get_list_member_target(list_shape_fqn) else {
        return;
    };
    let inner_resolved = model.resolve_type(&inner_target);
    let inner_name = resolved_type_to_rust_serde(&inner_resolved, false);
    let wrapper_name = escape_type_name(ServiceModel::short_name(list_shape_fqn));
    let member_name = model
        .get_list_member_xml_name(list_shape_fqn)
        .unwrap_or_else(|| "member".to_string());

    let cfg_attr = shape_features
        .map(|sf| cfg_attr_for_shape(sf.get(list_shape_fqn)))
        .unwrap_or_default();

    out.push_str(&format!(
        concat!(
            "{cfg_attr}",
            "#[derive(Debug, Clone, Default, Deserialize, Serialize)]\n",
            "pub struct {wrapper_name} {{\n",
            "    #[serde(\n",
            "        rename = \"{member_name}\",\n",
            "        default,\n",
            "        skip_serializing_if = \"Vec::is_empty\"\n",
            "    )]\n",
            "    pub items: Vec<{inner_name}>,\n",
            "}}\n",
            "{cfg_attr}",
            "impl From<Vec<{inner_name}>> for {wrapper_name} {{\n",
            "    fn from(v: Vec<{inner_name}>) -> Self {{\n",
            "        Self {{\n",
            "            items: v,\n",
            "        }}\n",
            "    }}\n",
            "}}\n",
            "{cfg_attr}",
            "impl FromIterator<{inner_name}> for {wrapper_name} {{\n",
            "    fn from_iter<I: IntoIterator<Item = {inner_name}>>(iter: I) -> Self {{\n",
            "        Self {{\n",
            "            items: iter.into_iter().collect(),\n",
            "        }}\n",
            "    }}\n",
            "}}\n\n",
        ),
        cfg_attr = cfg_attr,
        wrapper_name = wrapper_name,
        member_name = member_name,
        inner_name = inner_name,
    ));
}

#[allow(clippy::too_many_arguments)]
fn collect_nested_structs(
    resolved: &ResolvedType,
    model: &ServiceModel,
    out: &mut String,
    input_shapes: &std::collections::HashSet<String>,
    visited: &mut std::collections::HashSet<String>,
    rename_overrides: &std::collections::HashMap<String, String>,
    is_xml: bool,
    xml_flattened: bool,
    shape_features: Option<&ShapeFeatures>,
) {
    match resolved {
        ResolvedType::Structure(fqn) => {
            generate_struct_recursive(
                out,
                fqn,
                model,
                input_shapes,
                visited,
                rename_overrides,
                shape_features,
            );
        }
        ResolvedType::List(inner) => {
            if is_xml && !xml_flattened {
                // Emit a member-wrapper container struct, e.g.:
                //   pub struct XmlAttachedPolicyList { pub member: Vec<AttachedPolicy> }
                // The field name "member" causes quick_xml to emit <member>…</member> per item.
                let inner_name = resolved_type_to_rust_serde(inner, false);
                let container_name = format!("Xml{inner_name}List");
                let key = format!("__list_container_xml_{inner_name}");
                if !visited.contains(&key) {
                    visited.insert(key);
                    // Gate the wrapper struct (and its impls) with the same
                    // features as the inner element type. If the inner type is
                    // a primitive, it has no features → no cfg.
                    let inner_cfg = match (shape_features, inner.as_ref()) {
                        (Some(sf), ResolvedType::Structure(fqn)) => cfg_attr_for_shape(sf.get(fqn)),
                        _ => String::new(),
                    };
                    out.push_str(&format!(
                        concat!(
                            "/// awsQuery member-wrapper for `Vec<{inner_name}>`.\n",
                            "{cfg}",
                            "#[derive(Debug, Clone, Default, Deserialize, Serialize)]\n",
                            "pub struct {container_name} {{\n",
                            "    #[serde(default, skip_serializing_if = \"Vec::is_empty\")]\n",
                            "    pub member: Vec<{inner_name}>,\n",
                            "}}\n\n",
                            "{cfg}",
                            "impl From<Vec<{inner_name}>> for {container_name} {{\n",
                            "    fn from(v: Vec<{inner_name}>) -> Self {{\n",
                            "        Self {{\n",
                            "            member: v,\n",
                            "        }}\n",
                            "    }}\n",
                            "}}\n\n",
                            "{cfg}",
                            "impl FromIterator<{inner_name}> for {container_name} {{\n",
                            "    fn from_iter<I: IntoIterator<Item = {inner_name}>>(iter: I) -> Self {{\n",
                            "        Self {{\n",
                            "            member: iter.into_iter().collect(),\n",
                            "        }}\n",
                            "    }}\n",
                            "}}\n\n",
                        ),
                        cfg = inner_cfg,
                        inner_name = inner_name,
                        container_name = container_name,
                    ));
                }
            }
            // Recurse into the inner type (inner lists are always non-XML-wrapped at this level)
            collect_nested_structs(
                inner,
                model,
                out,
                input_shapes,
                visited,
                rename_overrides,
                false,
                false,
                shape_features,
            );
        }
        ResolvedType::Map(k, v) => {
            collect_nested_structs(
                k,
                model,
                out,
                input_shapes,
                visited,
                rename_overrides,
                false,
                false,
                shape_features,
            );
            collect_nested_structs(
                v,
                model,
                out,
                input_shapes,
                visited,
                rename_overrides,
                false,
                false,
                shape_features,
            );
        }
        _ => {}
    }
}

// --- Utility functions ---

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(ch.to_lowercase().next().unwrap());
        } else {
            result.push(ch);
        }
    }
    escape_rust_keyword(&result)
}

/// Check if a field's type needs Box to break recursive type cycles.
/// Returns true if the resolved type is a struct that can reach back to
/// `parent_shape` through its member graph, forming a cycle.
fn needs_boxing(resolved: &ResolvedType, parent_shape: &str, model: &ServiceModel) -> bool {
    match resolved {
        ResolvedType::Structure(fqn) => {
            // Check if this struct can reach back to parent_shape
            let mut visited = std::collections::HashSet::new();
            can_reach(fqn, parent_shape, model, &mut visited)
        }
        // Vec and HashMap are already heap-allocated, so they break recursive cycles
        // without needing an additional Box wrapper (which would trigger clippy::box_collection).
        ResolvedType::List(_) | ResolvedType::Map(_, _) => false,
        _ => false,
    }
}

/// Check if `from_shape` can reach `target_shape` through its member graph.
fn can_reach(
    from_shape: &str,
    target_shape: &str,
    model: &ServiceModel,
    visited: &mut std::collections::HashSet<String>,
) -> bool {
    if from_shape == target_shape {
        return true;
    }
    if !visited.insert(from_shape.to_string()) {
        return false; // Already visited, avoid infinite loop
    }
    let members = model.get_members(from_shape);
    for m in &members {
        let resolved = model.resolve_type(&m.target);
        if can_reach_resolved(&resolved, target_shape, model, visited) {
            return true;
        }
    }
    false
}

fn can_reach_resolved(
    resolved: &ResolvedType,
    target_shape: &str,
    model: &ServiceModel,
    visited: &mut std::collections::HashSet<String>,
) -> bool {
    match resolved {
        ResolvedType::Structure(fqn) => can_reach(fqn, target_shape, model, visited),
        ResolvedType::List(inner) => can_reach_resolved(inner, target_shape, model, visited),
        ResolvedType::Map(k, v) => {
            can_reach_resolved(k, target_shape, model, visited)
                || can_reach_resolved(v, target_shape, model, visited)
        }
        _ => false,
    }
}

/// Convert snake_case to PascalCase (for operation names in void serializers).
#[allow(dead_code)]
fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

/// Escape type names that shadow Rust builtin types.
fn escape_type_name(name: &str) -> String {
    const RESERVED: &[&str] = &[
        "Option", "Result", "Box", "String", "Vec", "HashMap", "Self",
    ];
    if RESERVED.contains(&name) {
        format!("{name}_")
    } else {
        name.to_string()
    }
}

/// Escape Rust keywords by prefixing with `r#`.
fn escape_rust_keyword(s: &str) -> String {
    const KEYWORDS: &[&str] = &[
        "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
        "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move",
        "mut", "override", "pub", "ref", "return", "self", "Self", "static", "struct", "super",
        "trait", "true", "type", "unsafe", "use", "where", "while", "yield", "abstract", "become",
        "box", "do", "final", "macro", "priv", "try", "typeof", "unsized", "virtual",
    ];
    if KEYWORDS.contains(&s) {
        format!("r#{s}")
    } else {
        s.to_string()
    }
}

fn binding_value_expr_for_member(
    member: &ShapeMember,
    model: &ServiceModel,
    source_expr: &str,
) -> String {
    let resolved = model.resolve_type(&member.target);
    // For timestamp http-bindings, the model emits `f64` only when the resolved
    // timestamp format is `epoch-seconds`; other formats (`date-time`,
    // `http-date`) emit `String`. JSON protocols default to `epoch-seconds`
    // but per-member `@timestampFormat` can override. Match the field's actual
    // Rust type when emitting the binding expression so the generated code
    // type-checks regardless of which format the model uses.
    if matches!(resolved, ResolvedType::Timestamp) {
        let ts_format = member
            .timestamp_format
            .as_deref()
            .unwrap_or_else(|| model.default_timestamp_format());
        if ts_format == "epoch-seconds" {
            return format!(
                "{source_expr}.parse::<f64>().ok().or_else(|| chrono::DateTime::parse_from_rfc3339({source_expr}).ok().map(|dt| dt.timestamp() as f64)).ok_or_else(|| format!(\"failed to parse timestamp: {{}}\", {source_expr}))?"
            );
        }
        // Non-epoch-seconds formats: field is `String`, just store the raw value.
        return format!("{source_expr}.to_string()");
    }
    if model.is_list_shape(&member.target) {
        let inner_expr = if let Some(inner_target) = model.get_list_member_target(&member.target) {
            let inner_resolved = model.resolve_type(&inner_target);
            scalar_binding_value_expr(&inner_resolved, "item.trim()")
        } else {
            "item.trim().to_string()".to_string()
        };
        if model.is_xml_protocol() {
            let wrapper_name = escape_type_name(ServiceModel::short_name(&member.target));
            return format!(
                "{wrapper_name} {{ items: {source_expr}.split(',').filter(|item| !item.trim().is_empty()).map(|item| Ok({inner_expr})).collect::<Result<Vec<_>, String>>()? }}"
            );
        }
        return format!(
            "{source_expr}.split(',').filter(|item| !item.trim().is_empty()).map(|item| Ok({inner_expr})).collect::<Result<Vec<_>, String>>()?"
        );
    }

    scalar_binding_value_expr(&resolved, source_expr)
}

fn scalar_binding_value_expr(resolved: &ResolvedType, source_expr: &str) -> String {
    match resolved {
        ResolvedType::String
        | ResolvedType::Enum(_)
        | ResolvedType::Blob
        | ResolvedType::Timestamp
        | ResolvedType::Unknown(_) => format!("{source_expr}.to_string()"),
        ResolvedType::Boolean => format!(
            "{source_expr}.parse::<bool>().map_err(|err| format!(\"failed to parse bool: {{err}}\"))?"
        ),
        ResolvedType::Integer => format!(
            "{source_expr}.parse::<i32>().map_err(|err| format!(\"failed to parse integer: {{err}}\"))?"
        ),
        ResolvedType::Long => format!(
            "{source_expr}.parse::<i64>().map_err(|err| format!(\"failed to parse long: {{err}}\"))?"
        ),
        ResolvedType::Float => format!(
            "{source_expr}.parse::<f32>().map_err(|err| format!(\"failed to parse float: {{err}}\"))?"
        ),
        ResolvedType::Double => format!(
            "{source_expr}.parse::<f64>().map_err(|err| format!(\"failed to parse double: {{err}}\"))?"
        ),
        ResolvedType::List(_)
        | ResolvedType::Map(_, _)
        | ResolvedType::Structure(_)
        | ResolvedType::Document => format!("{source_expr}.to_string()"),
    }
}

fn assign_member_value(target: &str, field_name: &str, required: bool, value_expr: &str) -> String {
    if required {
        format!("{target}.{field_name} = {value_expr}")
    } else {
        format!("{target}.{field_name} = Some({value_expr})")
    }
}

/// Like `resolved_type_to_rust_serde` but for XML non-flattened fields: replaces
/// `Vec<T>` with an anonymous member-wrapper container type so a wrapper element
/// is emitted without colliding with real Smithy list shape names.
fn xml_aware_rust_type(resolved: &ResolvedType, epoch_seconds: bool) -> String {
    match resolved {
        ResolvedType::List(inner) => {
            let inner_name = resolved_type_to_rust_serde(inner, epoch_seconds);
            format!("Xml{inner_name}List")
        }
        _ => resolved_type_to_rust_serde(resolved, epoch_seconds),
    }
}

/// Map resolved types to Rust types suitable for serde serialization.
/// `epoch_seconds`: if true, timestamps serialize as f64 (epoch seconds);
/// if false, timestamps serialize as String (ISO 8601 date-time).
fn resolved_type_to_rust_serde(resolved: &ResolvedType, epoch_seconds: bool) -> String {
    match resolved {
        ResolvedType::String | ResolvedType::Enum(_) => "String".to_string(),
        ResolvedType::Boolean => "bool".to_string(),
        ResolvedType::Integer => "i32".to_string(),
        ResolvedType::Long => "i64".to_string(),
        ResolvedType::Float => "f32".to_string(),
        ResolvedType::Double => "f64".to_string(),
        ResolvedType::Blob => "String".to_string(), // base64-encoded
        ResolvedType::Timestamp => {
            if epoch_seconds {
                "f64".to_string() // epoch seconds for JSON protocols
            } else {
                "String".to_string() // ISO 8601 for XML protocols
            }
        }
        ResolvedType::List(inner) => {
            format!("Vec<{}>", resolved_type_to_rust_serde(inner, epoch_seconds))
        }
        ResolvedType::Map(k, v) => format!(
            "std::collections::HashMap<{}, {}>",
            resolved_type_to_rust_serde(k, epoch_seconds),
            resolved_type_to_rust_serde(v, epoch_seconds)
        ),
        ResolvedType::Structure(name) => escape_type_name(ServiceModel::short_name(name)),
        ResolvedType::Document => "serde_json::Value".to_string(),
        ResolvedType::Unknown(_) => "String".to_string(),
    }
}
