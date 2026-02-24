//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cognitosync

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BulkPublishRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BulkPublishResponse {
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDatasetRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDatasetResponse {
    #[serde(rename = "Dataset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<Dataset>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Dataset {
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "DataStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_storage: Option<i64>,
    #[serde(rename = "DatasetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    #[serde(rename = "IdentityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "NumRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_records: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatasetRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatasetResponse {
    #[serde(rename = "Dataset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<Dataset>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIdentityPoolUsageRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIdentityPoolUsageResponse {
    #[serde(rename = "IdentityPoolUsage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_usage: Option<IdentityPoolUsage>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentityPoolUsage {
    #[serde(rename = "DataStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_storage: Option<i64>,
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "SyncSessionsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_sessions_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIdentityUsageRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIdentityUsageResponse {
    #[serde(rename = "IdentityUsage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_usage: Option<IdentityUsage>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentityUsage {
    #[serde(rename = "DataStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_storage: Option<i64>,
    #[serde(rename = "DatasetCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_count: Option<i32>,
    #[serde(rename = "IdentityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBulkPublishDetailsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBulkPublishDetailsResponse {
    #[serde(rename = "BulkPublishCompleteTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_publish_complete_time: Option<f64>,
    #[serde(rename = "BulkPublishStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_publish_start_time: Option<f64>,
    #[serde(rename = "BulkPublishStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_publish_status: Option<String>,
    #[serde(rename = "FailureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCognitoEventsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCognitoEventsResponse {
    #[serde(rename = "Events")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIdentityPoolConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIdentityPoolConfigurationResponse {
    #[serde(rename = "CognitoStreams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_streams: Option<CognitoStreams>,
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<String>,
    #[serde(rename = "PushSync")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_sync: Option<PushSync>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CognitoStreams {
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    #[serde(rename = "StreamingStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PushSync {
    #[serde(rename = "ApplicationArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arns: Option<Vec<String>>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatasetsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatasetsResponse {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "Datasets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasets: Option<Vec<Dataset>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIdentityPoolUsageRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIdentityPoolUsageResponse {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "IdentityPoolUsages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_usages: Option<Vec<IdentityPoolUsage>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecordsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecordsResponse {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "DatasetDeletedAfterRequestedSyncCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_deleted_after_requested_sync_count: Option<bool>,
    #[serde(rename = "DatasetExists")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_exists: Option<bool>,
    #[serde(rename = "DatasetSyncCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_sync_count: Option<i64>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "MergedDatasetNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_dataset_names: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Records")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<Record>>,
    #[serde(rename = "SyncSessionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_session_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Record {
    #[serde(rename = "DeviceLastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_last_modified_date: Option<f64>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "SyncCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_count: Option<i64>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterDeviceRequest {
    #[serde(rename = "Platform")]
    #[serde(default)]
    pub platform: String,
    #[serde(rename = "Token")]
    #[serde(default)]
    pub token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterDeviceResponse {
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetCognitoEventsRequest {
    #[serde(rename = "Events")]
    #[serde(default)]
    pub events: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetIdentityPoolConfigurationRequest {
    #[serde(rename = "CognitoStreams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_streams: Option<CognitoStreams>,
    #[serde(rename = "PushSync")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_sync: Option<PushSync>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetIdentityPoolConfigurationResponse {
    #[serde(rename = "CognitoStreams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_streams: Option<CognitoStreams>,
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<String>,
    #[serde(rename = "PushSync")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_sync: Option<PushSync>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubscribeToDatasetRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubscribeToDatasetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnsubscribeFromDatasetRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnsubscribeFromDatasetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRecordsRequest {
    #[serde(rename = "ClientContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_context: Option<String>,
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "RecordPatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_patches: Option<Vec<RecordPatch>>,
    #[serde(rename = "SyncSessionToken")]
    #[serde(default)]
    pub sync_session_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecordPatch {
    #[serde(rename = "DeviceLastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_last_modified_date: Option<f64>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Op")]
    #[serde(default)]
    pub op: String,
    #[serde(rename = "SyncCount")]
    #[serde(default)]
    pub sync_count: i64,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRecordsResponse {
    #[serde(rename = "Records")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<Record>>,
}
