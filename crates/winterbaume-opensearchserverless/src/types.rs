use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub type_: String,
    pub status: String,
    pub description: Option<String>,
    pub kms_key_arn: Option<String>,
    pub tags: Vec<Tag>,
    pub created_date: i64,
    pub last_modified_date: i64,
}

#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub name: String,
    pub type_: String,
    pub policy: serde_json::Value,
    pub policy_version: String,
    pub description: Option<String>,
    pub created_date: i64,
    pub last_modified_date: i64,
}

#[derive(Debug, Clone)]
pub struct VpcEndpoint {
    pub id: String,
    pub name: String,
    pub vpc_id: String,
    pub subnet_ids: Vec<String>,
    pub security_group_ids: Vec<String>,
    pub status: String,
}

/// Tag key/value pair.
#[derive(Debug, Clone)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

/// Per-resource tag store (ARN -> tags map).
pub type TagStore = HashMap<String, Vec<Tag>>;
