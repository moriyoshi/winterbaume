use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Export {
    pub export_arn: String,
    pub client_token: String,
    pub export_status: String,
    pub domain_name: String,
    pub requested_at: DateTime<Utc>,
    pub s3_bucket: String,
    pub s3_key_prefix: Option<String>,
    pub s3_sse_algorithm: Option<String>,
    pub s3_sse_kms_key_id: Option<String>,
    pub s3_bucket_owner: Option<String>,
    pub failure_code: Option<String>,
    pub failure_message: Option<String>,
    pub export_manifest: Option<String>,
    pub items_count: Option<i64>,
    pub export_data_cutoff_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct ExportSummary {
    pub export_arn: String,
    pub export_status: String,
    pub requested_at: DateTime<Utc>,
    pub domain_name: String,
}
