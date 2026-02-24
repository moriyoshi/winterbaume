//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cloudfrontkeyvaluestore

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

#[allow(unused_imports)]
use http::header::HeaderName;
use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for restJson protocol.
pub fn serialize_delete_key_response(result: &DeleteKeyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_key_value_store_response(
    result: &DescribeKeyValueStoreResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_get_key_response(result: &GetKeyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_keys_response(result: &ListKeysResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_key_response(result: &PutKeyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "etag": set by caller from e_tag field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_update_keys_response(result: &UpdateKeysResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "etag": set by caller from e_tag field
    resp
}
