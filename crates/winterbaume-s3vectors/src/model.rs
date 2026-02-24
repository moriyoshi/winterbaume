//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-s3vectors

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIndexInput {
    #[serde(rename = "dataType")]
    #[serde(default)]
    pub data_type: String,
    #[serde(default)]
    pub dimension: i32,
    #[serde(rename = "distanceMetric")]
    #[serde(default)]
    pub distance_metric: String,
    #[serde(rename = "encryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "indexName")]
    #[serde(default)]
    pub index_name: String,
    #[serde(rename = "metadataConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_configuration: Option<MetadataConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "vectorBucketArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_arn: Option<String>,
    #[serde(rename = "vectorBucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionConfiguration {
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "sseType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetadataConfiguration {
    #[serde(rename = "nonFilterableMetadataKeys")]
    #[serde(default)]
    pub non_filterable_metadata_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIndexOutput {
    #[serde(rename = "indexArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVectorBucketInput {
    #[serde(rename = "encryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "vectorBucketName")]
    #[serde(default)]
    pub vector_bucket_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVectorBucketOutput {
    #[serde(rename = "vectorBucketArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIndexInput {
    #[serde(rename = "indexArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_arn: Option<String>,
    #[serde(rename = "indexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "vectorBucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIndexOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVectorBucketInput {
    #[serde(rename = "vectorBucketArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_arn: Option<String>,
    #[serde(rename = "vectorBucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVectorBucketOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVectorBucketPolicyInput {
    #[serde(rename = "vectorBucketArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_arn: Option<String>,
    #[serde(rename = "vectorBucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVectorBucketPolicyOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVectorsInput {
    #[serde(rename = "indexArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_arn: Option<String>,
    #[serde(rename = "indexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(default)]
    pub keys: Vec<String>,
    #[serde(rename = "vectorBucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVectorsOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIndexInput {
    #[serde(rename = "indexArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_arn: Option<String>,
    #[serde(rename = "indexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "vectorBucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIndexOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Index>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Index {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "dataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension: Option<i32>,
    #[serde(rename = "distanceMetric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_metric: Option<String>,
    #[serde(rename = "encryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "indexArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_arn: Option<String>,
    #[serde(rename = "indexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "metadataConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_configuration: Option<MetadataConfiguration>,
    #[serde(rename = "vectorBucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVectorBucketInput {
    #[serde(rename = "vectorBucketArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_arn: Option<String>,
    #[serde(rename = "vectorBucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVectorBucketOutput {
    #[serde(rename = "vectorBucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket: Option<VectorBucket>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VectorBucket {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "encryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "vectorBucketArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_arn: Option<String>,
    #[serde(rename = "vectorBucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVectorBucketPolicyInput {
    #[serde(rename = "vectorBucketArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_arn: Option<String>,
    #[serde(rename = "vectorBucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVectorBucketPolicyOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVectorsInput {
    #[serde(rename = "indexArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_arn: Option<String>,
    #[serde(rename = "indexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(default)]
    pub keys: Vec<String>,
    #[serde(rename = "returnData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_data: Option<bool>,
    #[serde(rename = "returnMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_metadata: Option<bool>,
    #[serde(rename = "vectorBucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVectorsOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vectors: Option<Vec<GetOutputVector>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOutputVector {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<VectorData>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VectorData {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float32: Option<Vec<f32>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIndexesInput {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "vectorBucketArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_arn: Option<String>,
    #[serde(rename = "vectorBucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIndexesOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexes: Option<Vec<IndexSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IndexSummary {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "indexArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_arn: Option<String>,
    #[serde(rename = "indexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "vectorBucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceInput {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVectorBucketsInput {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVectorBucketsOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "vectorBuckets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_buckets: Option<Vec<VectorBucketSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VectorBucketSummary {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "vectorBucketArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_arn: Option<String>,
    #[serde(rename = "vectorBucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVectorsInput {
    #[serde(rename = "indexArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_arn: Option<String>,
    #[serde(rename = "indexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "returnData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_data: Option<bool>,
    #[serde(rename = "returnMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_metadata: Option<bool>,
    #[serde(rename = "segmentCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_count: Option<i32>,
    #[serde(rename = "segmentIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_index: Option<i32>,
    #[serde(rename = "vectorBucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVectorsOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vectors: Option<Vec<ListOutputVector>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOutputVector {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<VectorData>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutVectorBucketPolicyInput {
    #[serde(default)]
    pub policy: String,
    #[serde(rename = "vectorBucketArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_arn: Option<String>,
    #[serde(rename = "vectorBucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutVectorBucketPolicyOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutVectorsInput {
    #[serde(rename = "indexArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_arn: Option<String>,
    #[serde(rename = "indexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "vectorBucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_name: Option<String>,
    #[serde(default)]
    pub vectors: Vec<PutInputVector>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutInputVector {
    #[serde(default)]
    pub data: VectorData,
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutVectorsOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryVectorsInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
    #[serde(rename = "indexArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_arn: Option<String>,
    #[serde(rename = "indexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "queryVector")]
    #[serde(default)]
    pub query_vector: VectorData,
    #[serde(rename = "returnDistance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_distance: Option<bool>,
    #[serde(rename = "returnMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_metadata: Option<bool>,
    #[serde(rename = "topK")]
    #[serde(default)]
    pub top_k: i32,
    #[serde(rename = "vectorBucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryVectorsOutput {
    #[serde(rename = "distanceMetric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_metric: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vectors: Option<Vec<QueryOutputVector>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryOutputVector {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<f32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceInput {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceInput {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceOutput {}
