//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-ebs

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
pub fn serialize_complete_snapshot_response(result: &CompleteSnapshotResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_snapshot_block_response(result: &GetSnapshotBlockResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "x-amz-checksum": set by caller from checksum field
    // Header "x-amz-checksum-algorithm": set by caller from checksum_algorithm field
    // Header "x-amz-data-length": set by caller from data_length field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_list_changed_blocks_response(result: &ListChangedBlocksResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_snapshot_blocks_response(
    result: &ListSnapshotBlocksResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_snapshot_block_response(result: &PutSnapshotBlockResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "x-amz-checksum": set by caller from checksum field
    // Header "x-amz-checksum-algorithm": set by caller from checksum_algorithm field
    resp
}

/// Serialize response for restJson protocol.
pub fn serialize_start_snapshot_response(result: &StartSnapshotResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}
