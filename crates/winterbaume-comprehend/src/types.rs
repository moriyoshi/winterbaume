use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DocumentClassifier {
    pub arn: String,
    pub name: String,
    pub language_code: String,
    pub data_access_role_arn: String,
    pub input_data_config_s3_uri: String,
    pub status: String,
    pub submit_time: f64,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct EntityRecognizer {
    pub arn: String,
    pub name: String,
    pub language_code: String,
    pub data_access_role_arn: String,
    pub input_data_config_s3_uri: String,
    pub entity_types: Vec<String>,
    pub status: String,
    pub submit_time: f64,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Endpoint {
    pub arn: String,
    pub name: String,
    pub model_arn: String,
    pub desired_model_arn: String,
    pub desired_inference_units: i32,
    pub current_inference_units: i32,
    pub status: String,
    pub creation_time: f64,
    pub last_modified_time: f64,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Flywheel {
    pub arn: String,
    pub name: String,
    pub data_access_role_arn: String,
    pub data_lake_s3_uri: String,
    pub active_model_arn: String,
    pub model_type: String,
    pub status: String,
    pub creation_time: f64,
    pub last_modified_time: f64,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ComprehendJob {
    pub job_id: String,
    pub job_arn: String,
    pub job_name: Option<String>,
    pub job_status: String,
    pub submit_time: f64,
    pub data_access_role_arn: String,
    pub input_s3_uri: String,
    pub output_s3_uri: String,
    pub language_code: Option<String>,
    pub job_type: JobType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JobType {
    DocumentClassification,
    DominantLanguageDetection,
    EntitiesDetection,
    EventsDetection,
    KeyPhrasesDetection,
    PiiEntitiesDetection,
    SentimentDetection,
    TargetedSentimentDetection,
    TopicsDetection,
}

#[derive(Debug, Clone)]
pub struct ResourcePolicy {
    pub policy: String,
    pub revision_id: String,
    pub creation_time: f64,
    pub last_modified_time: f64,
}
