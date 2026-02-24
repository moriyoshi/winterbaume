use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Collection {
    pub collection_id: String,
    pub collection_arn: String,
    pub face_count: u64,
    pub face_model_version: String,
    pub creation_timestamp: f64,
    pub user_count: u64,
    pub tags: HashMap<String, String>,
}

/// Represents an async video analysis job.
#[derive(Debug, Clone)]
pub struct VideoJob {
    pub job_id: String,
    pub job_tag: Option<String>,
    pub job_type: VideoJobType,
    pub collection_id: Option<String>,
    /// Deterministic seed derived from job inputs, used to generate varied mock results.
    pub seed: u64,
}

#[derive(Debug, Clone)]
pub enum VideoJobType {
    FaceSearch,
    TextDetection,
}
