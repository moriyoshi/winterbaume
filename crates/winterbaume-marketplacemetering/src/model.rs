//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-marketplacemetering

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchMeterUsageRequest {
    #[serde(rename = "ProductCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    #[serde(rename = "UsageRecords")]
    #[serde(default)]
    pub usage_records: Vec<UsageRecord>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsageRecord {
    #[serde(rename = "CustomerAWSAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_a_w_s_account_id: Option<String>,
    #[serde(rename = "CustomerIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_identifier: Option<String>,
    #[serde(rename = "Dimension")]
    #[serde(default)]
    pub dimension: String,
    #[serde(rename = "LicenseArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_arn: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    pub timestamp: f64,
    #[serde(rename = "UsageAllocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_allocations: Option<Vec<UsageAllocation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsageAllocation {
    #[serde(rename = "AllocatedUsageQuantity")]
    #[serde(default)]
    pub allocated_usage_quantity: i32,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchMeterUsageResult {
    #[serde(rename = "Results")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<UsageRecordResult>>,
    #[serde(rename = "UnprocessedRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_records: Option<Vec<UsageRecord>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsageRecordResult {
    #[serde(rename = "MeteringRecordId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metering_record_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UsageRecord")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_record: Option<UsageRecord>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MeterUsageRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "ProductCode")]
    #[serde(default)]
    pub product_code: String,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    pub timestamp: f64,
    #[serde(rename = "UsageAllocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_allocations: Option<Vec<UsageAllocation>>,
    #[serde(rename = "UsageDimension")]
    #[serde(default)]
    pub usage_dimension: String,
    #[serde(rename = "UsageQuantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MeterUsageResult {
    #[serde(rename = "MeteringRecordId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metering_record_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterUsageRequest {
    #[serde(rename = "Nonce")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(rename = "ProductCode")]
    #[serde(default)]
    pub product_code: String,
    #[serde(rename = "PublicKeyVersion")]
    #[serde(default)]
    pub public_key_version: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterUsageResult {
    #[serde(rename = "PublicKeyRotationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_rotation_timestamp: Option<f64>,
    #[serde(rename = "Signature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResolveCustomerRequest {
    #[serde(rename = "RegistrationToken")]
    #[serde(default)]
    pub registration_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResolveCustomerResult {
    #[serde(rename = "CustomerAWSAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_a_w_s_account_id: Option<String>,
    #[serde(rename = "CustomerIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_identifier: Option<String>,
    #[serde(rename = "LicenseArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_arn: Option<String>,
    #[serde(rename = "ProductCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
}
