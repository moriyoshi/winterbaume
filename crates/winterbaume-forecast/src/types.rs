use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct DatasetGroup {
    pub dataset_group_name: String,
    pub dataset_group_arn: String,
    pub domain: String,
    pub dataset_arns: Vec<String>,
    pub status: String,
    pub creation_time: DateTime<Utc>,
    pub last_modification_time: DateTime<Utc>,
}
