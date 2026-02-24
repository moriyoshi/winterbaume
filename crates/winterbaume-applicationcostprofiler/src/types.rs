#[derive(Debug, Clone)]
pub struct S3Location {
    pub bucket: String,
    pub prefix: String,
}

#[derive(Debug, Clone)]
pub struct ReportDefinition {
    pub report_id: String,
    pub report_description: String,
    pub report_frequency: String,
    pub format: String,
    pub destination_s3_location: S3Location,
    pub created_at: i64,
    pub last_updated_at: i64,
}

#[derive(Debug, Clone)]
pub struct ImportJob {
    pub import_id: String,
    pub source_bucket: String,
    pub source_key: String,
    pub source_region: Option<String>,
    pub created_at: i64,
}
