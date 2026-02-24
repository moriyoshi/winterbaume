//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cloudfrontkeyvaluestore

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteKeyRequest {
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    pub if_match: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteKeyResponse {
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "ItemCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i32>,
    #[serde(rename = "TotalSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size_in_bytes: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeKeyValueStoreRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeKeyValueStoreResponse {
    #[serde(rename = "Created")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "ItemCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i32>,
    #[serde(rename = "KvsARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kvs_a_r_n: Option<String>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TotalSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size_in_bytes: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetKeyRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetKeyResponse {
    #[serde(rename = "ItemCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i32>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "TotalSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size_in_bytes: Option<i64>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListKeysRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListKeysResponse {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ListKeysResponseListItem>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListKeysResponseListItem {
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
pub struct PutKeyRequest {
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    pub if_match: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutKeyResponse {
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "ItemCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i32>,
    #[serde(rename = "TotalSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size_in_bytes: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateKeysRequest {
    #[serde(rename = "Deletes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletes: Option<Vec<DeleteKeyRequestListItem>>,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    pub if_match: String,
    #[serde(rename = "Puts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub puts: Option<Vec<PutKeyRequestListItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteKeyRequestListItem {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutKeyRequestListItem {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateKeysResponse {
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "ItemCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i32>,
    #[serde(rename = "TotalSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size_in_bytes: Option<i64>,
}
