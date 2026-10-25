//! RPC-v2 CBOR transport adapter for the existing generated AWS JSON shapes.
//!
//! Keyspaces has no blob members; its JSON and CBOR member names coincide.
//! Timestamps need tag 1 on CBOR output, and errors need absolute shape IDs.

use ciborium::value::Value as CborValue;
use serde_json::Value;
use winterbaume_core::{MockRequest, MockResponse, json_error_response};

pub(crate) fn decode_request(mut request: MockRequest) -> Result<MockRequest, MockResponse> {
    if request.method != "POST"
        || request.headers.contains_key("x-amz-target")
        || request.headers.contains_key("x-amzn-target")
    {
        return Err(error_response(
            "Invalid RPC-v2 request method or target header",
        ));
    }
    let uri = request
        .uri
        .parse::<http::Uri>()
        .map_err(|_| error_response("Invalid URI"))?;
    let mut path = uri.path().rsplit('/');
    let operation = path.next().unwrap_or_default();
    if operation.is_empty()
        || operation.contains('.')
        || path.next() != Some("operation")
        || !matches!(
            path.next(),
            Some("KeyspacesService" | "com.amazonaws.keyspaces.KeyspacesService")
        )
        || path.next() != Some("service")
    {
        return Err(error_response("Invalid RPC-v2 operation path"));
    }

    let mut bytes = request.body.as_ref();
    let mut body: CborValue = if bytes.is_empty() {
        CborValue::Map(Vec::new())
    } else {
        ciborium::from_reader(&mut bytes).map_err(|_| error_response("Invalid CBOR body"))?
    };
    if body.as_map().is_none() || !bytes.is_empty() {
        return Err(error_response("Expected a single CBOR structure"));
    }
    untag_timestamps(&mut body);
    let body: Value = body
        .deserialized()
        .map_err(|_| error_response("Invalid CBOR member value"))?;
    request.headers.insert(
        "x-amz-target",
        format!("KeyspacesService.{operation}")
            .parse()
            .map_err(|_| error_response("Invalid operation name"))?,
    );
    request.headers.remove("smithy-protocol");
    request.headers.insert(
        http::header::CONTENT_TYPE,
        "application/x-amz-json-1.0".parse().unwrap(),
    );
    request.headers.remove(http::header::CONTENT_LENGTH);
    request.body = serde_json::to_vec(&body)
        .expect("JSON-compatible CBOR input")
        .into();
    Ok(request)
}

fn error_response(message: &str) -> MockResponse {
    encode_response(json_error_response(400, "ValidationException", message))
}

fn untag_timestamps(value: &mut CborValue) {
    match value {
        CborValue::Tag(1, inner)
            if matches!(inner.as_ref(), CborValue::Integer(_) | CborValue::Float(_)) =>
        {
            *value = inner.as_ref().clone();
        }
        CborValue::Map(entries) => {
            for (_, value) in entries {
                untag_timestamps(value);
            }
        }
        CborValue::Array(values) => {
            for value in values {
                untag_timestamps(value);
            }
        }
        _ => {}
    }
}

pub(crate) fn encode_response(response: MockResponse) -> MockResponse {
    let mut body: Value = serde_json::from_slice(&response.body).expect("Keyspaces JSON response");
    if let Some(code) = body.get("__type").and_then(Value::as_str) {
        let shape = format!("com.amazonaws.keyspaces#{code}");
        body["__type"] = Value::String(shape);
    }
    let mut value = CborValue::serialized(&body).expect("JSON response is CBOR-compatible");
    tag_timestamps(&mut value);
    let mut bytes = Vec::new();
    ciborium::into_writer(&value, &mut bytes).expect("writing CBOR to a Vec cannot fail");
    MockResponse::cbor(response.status, bytes)
}

fn tag_timestamps(value: &mut CborValue) {
    match value {
        CborValue::Map(entries) => {
            for (key, value) in entries {
                if matches!(
                    key.as_text(),
                    Some(
                        "creationTimestamp"
                            | "lastModifiedTimestamp"
                            | "earliestRestorableTimestamp"
                            | "lastUpdateToPayPerRequestTimestamp"
                    )
                ) && matches!(value, CborValue::Integer(_) | CborValue::Float(_))
                {
                    *value = CborValue::Tag(1, Box::new(value.clone()));
                } else {
                    tag_timestamps(value);
                }
            }
        }
        CborValue::Array(values) => {
            for value in values {
                tag_timestamps(value);
            }
        }
        _ => {}
    }
}
