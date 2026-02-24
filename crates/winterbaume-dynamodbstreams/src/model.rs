//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-dynamodbstreams

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStreamInput {
    #[serde(rename = "ExclusiveStartShardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_shard_id: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "ShardFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_filter: Option<ShardFilter>,
    #[serde(rename = "StreamArn")]
    #[serde(default)]
    pub stream_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ShardFilter {
    #[serde(rename = "ShardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStreamOutput {
    #[serde(rename = "StreamDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_description: Option<StreamDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamDescription {
    #[serde(rename = "CreationRequestDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_request_date_time: Option<f64>,
    #[serde(rename = "KeySchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<Vec<KeySchemaElement>>,
    #[serde(rename = "LastEvaluatedShardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_shard_id: Option<String>,
    #[serde(rename = "Shards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shards: Option<Vec<Shard>>,
    #[serde(rename = "StreamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    #[serde(rename = "StreamLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_label: Option<String>,
    #[serde(rename = "StreamStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_status: Option<String>,
    #[serde(rename = "StreamViewType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_view_type: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeySchemaElement {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "KeyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Shard {
    #[serde(rename = "ParentShardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_shard_id: Option<String>,
    #[serde(rename = "SequenceNumberRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number_range: Option<SequenceNumberRange>,
    #[serde(rename = "ShardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SequenceNumberRange {
    #[serde(rename = "EndingSequenceNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_sequence_number: Option<String>,
    #[serde(rename = "StartingSequenceNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_sequence_number: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRecordsInput {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "ShardIterator")]
    #[serde(default)]
    pub shard_iterator: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRecordsOutput {
    #[serde(rename = "NextShardIterator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_shard_iterator: Option<String>,
    #[serde(rename = "Records")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<Record>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Record {
    #[serde(rename = "awsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamodb: Option<StreamRecord>,
    #[serde(rename = "eventID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_i_d: Option<String>,
    #[serde(rename = "eventName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(rename = "eventSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source: Option<String>,
    #[serde(rename = "eventVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_version: Option<String>,
    #[serde(rename = "userIdentity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_identity: Option<Identity>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamRecord {
    #[serde(rename = "ApproximateCreationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_creation_date_time: Option<f64>,
    #[serde(rename = "Keys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "NewImage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_image: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "OldImage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_image: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "SequenceNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
    #[serde(rename = "SizeBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
    #[serde(rename = "StreamViewType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_view_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttributeValue {
    #[serde(rename = "B")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<String>,
    #[serde(rename = "BOOL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_o_o_l: Option<bool>,
    #[serde(rename = "BS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_s: Option<Vec<String>>,
    #[serde(rename = "L")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l: Option<Vec<AttributeValue>>,
    #[serde(rename = "M")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m: Option<std::collections::HashMap<String, AttributeValue>>,
    #[serde(rename = "N")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,
    #[serde(rename = "NS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_s: Option<Vec<String>>,
    #[serde(rename = "NULL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_u_l_l: Option<bool>,
    #[serde(rename = "S")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(rename = "SS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Identity {
    #[serde(rename = "PrincipalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetShardIteratorInput {
    #[serde(rename = "SequenceNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
    #[serde(rename = "ShardId")]
    #[serde(default)]
    pub shard_id: String,
    #[serde(rename = "ShardIteratorType")]
    #[serde(default)]
    pub shard_iterator_type: String,
    #[serde(rename = "StreamArn")]
    #[serde(default)]
    pub stream_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetShardIteratorOutput {
    #[serde(rename = "ShardIterator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_iterator: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStreamsInput {
    #[serde(rename = "ExclusiveStartStreamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_stream_arn: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStreamsOutput {
    #[serde(rename = "LastEvaluatedStreamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_stream_arn: Option<String>,
    #[serde(rename = "Streams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streams: Option<Vec<Stream>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Stream {
    #[serde(rename = "StreamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    #[serde(rename = "StreamLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_label: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}
