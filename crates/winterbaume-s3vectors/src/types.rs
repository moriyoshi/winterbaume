use std::collections::HashMap;

/// A vector bucket.
#[derive(Debug, Clone)]
pub struct VectorBucket {
    pub name: String,
    pub arn: String,
    pub creation_time: String,
}

/// An index within a vector bucket.
#[derive(Debug, Clone)]
pub struct Index {
    pub bucket_name: String,
    pub name: String,
    pub arn: String,
    pub creation_time: String,
    pub data_type: String,
    pub dimension: i32,
    pub distance_metric: String,
    pub non_filterable_metadata_keys: Vec<String>,
}

/// A stored vector.
#[derive(Debug, Clone)]
pub struct Vector {
    pub key: String,
    pub data: Vec<f32>,
    pub metadata: Option<serde_json::Value>,
}

/// Policy on a vector bucket.
#[derive(Debug, Clone)]
pub struct VectorBucketPolicy {
    pub policy: String,
}

/// Tags keyed by ARN.
pub type TagsMap = HashMap<String, String>;
