use chrono::{DateTime, Utc};

/// A usage record for batch meter usage.
#[derive(Debug, Clone)]
pub struct UsageRecord {
    pub timestamp: DateTime<Utc>,
    pub customer_identifier: String,
    pub dimension: String,
    pub quantity: i32,
    pub usage_allocations: Vec<UsageAllocation>,
}

/// Allocation within a usage record.
#[derive(Debug, Clone)]
pub struct UsageAllocation {
    pub allocated_usage_quantity: i32,
    pub tags: Vec<UsageTag>,
}

/// A tag associated with a usage allocation.
#[derive(Debug, Clone)]
pub struct UsageTag {
    pub key: String,
    pub value: String,
}

/// Result of processing a single usage record.
#[derive(Debug, Clone)]
pub struct UsageRecordResult {
    pub usage_record: UsageRecord,
    pub metering_record_id: String,
    pub status: UsageRecordResultStatus,
}

/// Status of a processed usage record.
#[derive(Debug, Clone, PartialEq)]
pub enum UsageRecordResultStatus {
    Success,
    CustomerNotSubscribed,
    DuplicateRecord,
}

impl UsageRecordResultStatus {
    pub fn as_str(&self) -> &str {
        match self {
            UsageRecordResultStatus::Success => "Success",
            UsageRecordResultStatus::CustomerNotSubscribed => "CustomerNotSubscribed",
            UsageRecordResultStatus::DuplicateRecord => "DuplicateRecord",
        }
    }
}

/// Registration result for RegisterUsage.
#[derive(Debug, Clone)]
pub struct Registration {
    pub product_code: String,
    pub public_key_rotation_timestamp: DateTime<Utc>,
    pub signature: String,
}

/// A resolved customer mapping.
#[derive(Debug, Clone)]
pub struct ResolvedCustomer {
    pub customer_identifier: String,
    pub product_code: String,
    pub customer_aws_account_id: String,
}

/// A metering record from MeterUsage.
#[derive(Debug, Clone)]
pub struct MeteringRecord {
    pub product_code: String,
    pub timestamp: DateTime<Utc>,
    pub usage_dimension: String,
    pub usage_quantity: i32,
    pub dry_run: bool,
    pub metering_record_id: String,
    pub usage_allocations: Vec<UsageAllocation>,
}
