//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-mediastore-data

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
pub fn serialize_delete_object_response(result: &DeleteObjectResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_object_response(result: &DescribeObjectResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "cache-control": set by caller from cache_control field
    // Header "content-length": set by caller from content_length field
    // Header "content-type": set by caller from content_type field
    // Header "etag": set by caller from e_tag field
    // Header "last-modified": set by caller from last_modified field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_get_object_response(result: &GetObjectResponse) -> MockResponse {
    let status = result.status_code.unwrap_or(200) as u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "cache-control": set by caller from cache_control field
    // Header "content-length": set by caller from content_length field
    // Header "content-range": set by caller from content_range field
    // Header "content-type": set by caller from content_type field
    // Header "etag": set by caller from e_tag field
    // Header "last-modified": set by caller from last_modified field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_list_items_response(result: &ListItemsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_object_response(result: &PutObjectResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}
