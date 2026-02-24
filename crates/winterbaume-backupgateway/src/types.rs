use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Gateway {
    pub arn: String,
    pub display_name: String,
    pub gateway_type: String,
    pub hypervisor_arn: Option<String>,
    pub last_seen_time: i64,
    pub software_version: String,
    pub vpc_endpoint: Option<String>,
    pub maintenance_start_time: Option<MaintenanceStartTime>,
    pub bandwidth_rate_limit_intervals: Vec<BandwidthRateLimitInterval>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Default)]
pub struct MaintenanceStartTime {
    pub day_of_month: Option<i32>,
    pub day_of_week: Option<i32>,
    pub hour_of_day: i32,
    pub minute_of_hour: i32,
}

#[derive(Debug, Clone, Default)]
pub struct BandwidthRateLimitInterval {
    pub average_upload_rate_limit_in_bits_per_sec: Option<i64>,
    pub days_of_week: Vec<i32>,
    pub start_hour_of_day: i32,
    pub start_minute_of_hour: i32,
    pub end_hour_of_day: i32,
    pub end_minute_of_hour: i32,
}

#[derive(Debug, Clone)]
pub struct Hypervisor {
    pub arn: String,
    pub host: String,
    pub name: String,
    pub state: String,
    pub kms_key_arn: Option<String>,
    pub log_group_arn: Option<String>,
    pub iam_role_arn: Option<String>,
    pub property_mappings: Vec<TagMapping>,
    pub last_metadata_sync_status: String,
    pub last_metadata_sync_time: Option<i64>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct TagMapping {
    pub aws_tag_key: String,
    pub aws_tag_value: String,
    pub vmware_category: String,
    pub vmware_tag_name: String,
}

#[derive(Debug, Clone)]
pub struct VirtualMachine {
    pub resource_arn: String,
    pub host_name: String,
    pub hypervisor_arn: String,
    pub name: String,
    pub path: String,
    pub last_backup_date: Option<i64>,
}
