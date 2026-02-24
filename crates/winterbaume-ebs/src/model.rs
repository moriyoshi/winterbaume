//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-ebs

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSnapshotResponse {
    #[serde(rename = "BlockSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_size: Option<i32>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "ParentSnapshotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_snapshot_id: Option<String>,
    #[serde(rename = "SnapshotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "SseType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_type: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VolumeSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompleteSnapshotResponse {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSnapshotRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Encrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "ParentSnapshotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_snapshot_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "VolumeSize")]
    #[serde(default)]
    pub volume_size: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListChangedBlocksRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompleteSnapshotRequest {
    #[serde(rename = "ChangedBlocksCount")]
    #[serde(default)]
    pub changed_blocks_count: i32,
    #[serde(rename = "Checksum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(rename = "ChecksumAggregationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_aggregation_method: Option<String>,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSnapshotBlockRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSnapshotBlockResponse {
    #[serde(rename = "BlockData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_data: Option<String>,
    #[serde(rename = "Checksum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "DataLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_length: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSnapshotBlocksRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSnapshotBlocksResponse {
    #[serde(rename = "BlockSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_size: Option<i32>,
    #[serde(rename = "Blocks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<Block>>,
    #[serde(rename = "ExpiryTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "VolumeSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Block {
    #[serde(rename = "BlockIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_index: Option<i32>,
    #[serde(rename = "BlockToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListChangedBlocksResponse {
    #[serde(rename = "BlockSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_size: Option<i32>,
    #[serde(rename = "ChangedBlocks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changed_blocks: Option<Vec<ChangedBlock>>,
    #[serde(rename = "ExpiryTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "VolumeSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChangedBlock {
    #[serde(rename = "BlockIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_index: Option<i32>,
    #[serde(rename = "FirstBlockToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_block_token: Option<String>,
    #[serde(rename = "SecondBlockToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_block_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutSnapshotBlockRequest {
    #[serde(rename = "BlockData")]
    #[serde(default)]
    pub block_data: String,
    #[serde(rename = "Checksum")]
    #[serde(default)]
    pub checksum: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    pub checksum_algorithm: String,
    #[serde(rename = "DataLength")]
    #[serde(default)]
    pub data_length: i32,
    #[serde(rename = "Progress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutSnapshotBlockResponse {
    #[serde(rename = "Checksum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
}
