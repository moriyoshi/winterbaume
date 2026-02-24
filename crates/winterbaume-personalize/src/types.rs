#[derive(Debug, Clone)]
pub struct DatasetGroup {
    pub name: String,
    pub dataset_group_arn: String,
    pub status: String,
    pub role_arn: Option<String>,
    pub kms_key_arn: Option<String>,
    pub domain: Option<String>,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

#[derive(Debug, Clone)]
pub struct Schema {
    pub name: String,
    pub schema_arn: String,
    pub schema: String,
    pub domain: Option<String>,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

#[derive(Debug, Clone)]
pub struct Dataset {
    pub name: String,
    pub dataset_arn: String,
    pub dataset_group_arn: String,
    pub dataset_type: String,
    pub schema_arn: String,
    pub status: String,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

#[derive(Debug, Clone)]
pub struct Campaign {
    pub name: String,
    pub campaign_arn: String,
    pub solution_version_arn: String,
    pub min_provisioned_tps: Option<i32>,
    pub status: String,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

#[derive(Debug, Clone)]
pub struct EventTracker {
    pub name: String,
    pub event_tracker_arn: String,
    pub dataset_group_arn: String,
    pub tracking_id: String,
    pub account_id: String,
    pub status: String,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub name: String,
    pub filter_arn: String,
    pub dataset_group_arn: String,
    pub filter_expression: String,
    pub status: String,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

#[derive(Debug, Clone)]
pub struct BatchInferenceJob {
    pub job_name: String,
    pub batch_inference_job_arn: String,
    pub solution_version_arn: String,
    pub filter_arn: Option<String>,
    pub role_arn: String,
    pub status: String,
    pub num_results: Option<i32>,
    pub job_input_s3_path: String,
    pub job_output_s3_path: String,
    pub batch_inference_job_mode: Option<String>,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

#[derive(Debug, Clone)]
pub struct BatchSegmentJob {
    pub job_name: String,
    pub batch_segment_job_arn: String,
    pub solution_version_arn: String,
    pub filter_arn: Option<String>,
    pub role_arn: String,
    pub num_results: Option<i32>,
    pub job_input_s3_path: String,
    pub job_output_s3_path: String,
    pub status: String,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

#[derive(Debug, Clone)]
pub struct DataDeletionJob {
    pub job_name: String,
    pub data_deletion_job_arn: String,
    pub dataset_group_arn: String,
    pub data_source_location: String,
    pub role_arn: String,
    pub status: String,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

#[derive(Debug, Clone)]
pub struct DatasetExportJob {
    pub job_name: String,
    pub dataset_export_job_arn: String,
    pub dataset_arn: String,
    pub role_arn: String,
    pub ingestion_mode: Option<String>,
    pub job_output_s3_path: String,
    pub status: String,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

#[derive(Debug, Clone)]
pub struct DatasetImportJob {
    pub job_name: String,
    pub dataset_import_job_arn: String,
    pub dataset_arn: String,
    pub data_source_location: String,
    pub role_arn: Option<String>,
    pub import_mode: Option<String>,
    pub status: String,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

#[derive(Debug, Clone)]
pub struct MetricAttribution {
    pub name: String,
    pub metric_attribution_arn: String,
    pub dataset_group_arn: String,
    pub metrics_output_role_arn: String,
    pub metrics_output_s3_path: Option<String>,
    pub metrics: Vec<MetricAttributeEntry>,
    pub status: String,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

#[derive(Debug, Clone)]
pub struct MetricAttributeEntry {
    pub event_type: String,
    pub expression: String,
    pub metric_name: String,
}

#[derive(Debug, Clone)]
pub struct Recommender {
    pub name: String,
    pub recommender_arn: String,
    pub dataset_group_arn: String,
    pub recipe_arn: String,
    pub recommender_config: Option<RecommenderConfigData>,
    pub status: String,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

#[derive(Debug, Clone)]
pub struct RecommenderConfigData {
    pub enable_metadata_with_recommendations: Option<bool>,
    pub item_exploration_config: Option<std::collections::HashMap<String, String>>,
    pub min_recommendation_requests_per_second: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct ResourceTag {
    pub tag_key: String,
    pub tag_value: String,
}

#[derive(Debug, Clone)]
pub struct Solution {
    pub name: String,
    pub solution_arn: String,
    pub dataset_group_arn: String,
    pub recipe_arn: Option<String>,
    pub event_type: Option<String>,
    pub perform_auto_ml: Option<bool>,
    pub perform_auto_training: Option<bool>,
    pub perform_hpo: Option<bool>,
    pub status: String,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

#[derive(Debug, Clone)]
pub struct SolutionVersionData {
    pub solution_version_arn: String,
    pub solution_arn: String,
    pub name: Option<String>,
    pub training_mode: Option<String>,
    pub status: String,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}
