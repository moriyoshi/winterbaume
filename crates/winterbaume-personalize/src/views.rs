//! Serde-compatible view types for Personalize state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::PersonalizeService;
use crate::state::PersonalizeState;

/// Serializable view of a dataset group.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatasetGroupView {
    pub name: String,
    pub dataset_group_arn: String,
    pub status: String,
    pub role_arn: Option<String>,
    pub kms_key_arn: Option<String>,
    pub domain: Option<String>,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

/// Serializable view of a schema.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SchemaView {
    pub name: String,
    pub schema_arn: String,
    pub schema: String,
    pub domain: Option<String>,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

/// Serializable view of a dataset.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatasetView {
    pub name: String,
    pub dataset_arn: String,
    pub dataset_group_arn: String,
    pub dataset_type: String,
    pub schema_arn: String,
    pub status: String,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

/// Serializable view of a campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CampaignView {
    pub name: String,
    pub campaign_arn: String,
    pub solution_version_arn: String,
    pub min_provisioned_tps: Option<i32>,
    pub status: String,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

/// Serializable view of an event tracker.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventTrackerView {
    pub name: String,
    pub event_tracker_arn: String,
    pub dataset_group_arn: String,
    pub tracking_id: String,
    pub account_id: String,
    pub status: String,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

/// Serializable view of a filter.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FilterView {
    pub name: String,
    pub filter_arn: String,
    pub dataset_group_arn: String,
    pub filter_expression: String,
    pub status: String,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

/// Serializable view of a batch inference job.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchInferenceJobView {
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

/// Serializable view of a batch segment job.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchSegmentJobView {
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

/// Serializable view of a data deletion job.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataDeletionJobView {
    pub job_name: String,
    pub data_deletion_job_arn: String,
    pub dataset_group_arn: String,
    pub data_source_location: String,
    pub role_arn: String,
    pub status: String,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

/// Serializable view of a dataset export job.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatasetExportJobView {
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

/// Serializable view of a dataset import job.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatasetImportJobView {
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

/// Serializable view of a metric attribute entry.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricAttributeEntryView {
    pub event_type: String,
    pub expression: String,
    pub metric_name: String,
}

/// Serializable view of a metric attribution.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricAttributionView {
    pub name: String,
    pub metric_attribution_arn: String,
    pub dataset_group_arn: String,
    pub metrics_output_role_arn: String,
    pub metrics_output_s3_path: Option<String>,
    #[serde(default)]
    pub metrics: Vec<MetricAttributeEntryView>,
    pub status: String,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

/// Serializable view of a recommender config.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecommenderConfigView {
    pub enable_metadata_with_recommendations: Option<bool>,
    pub item_exploration_config: Option<HashMap<String, String>>,
    pub min_recommendation_requests_per_second: Option<i32>,
}

/// Serializable view of a recommender.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecommenderView {
    pub name: String,
    pub recommender_arn: String,
    pub dataset_group_arn: String,
    pub recipe_arn: String,
    pub recommender_config: Option<RecommenderConfigView>,
    pub status: String,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

/// Serializable view of a resource tag.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceTagView {
    pub tag_key: String,
    pub tag_value: String,
}

/// Serializable view of a solution.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SolutionView {
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

/// Serializable view of a solution version.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SolutionVersionView {
    pub solution_version_arn: String,
    pub solution_arn: String,
    pub name: Option<String>,
    pub training_mode: Option<String>,
    pub status: String,
    pub creation_date_time: f64,
    pub last_updated_date_time: f64,
}

/// Serializable view of the entire Personalize state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonalizeStateView {
    #[serde(default)]
    pub dataset_groups: HashMap<String, DatasetGroupView>,
    #[serde(default)]
    pub schemas: HashMap<String, SchemaView>,
    #[serde(default)]
    pub datasets: HashMap<String, DatasetView>,
    #[serde(default)]
    pub campaigns: HashMap<String, CampaignView>,
    #[serde(default)]
    pub event_trackers: HashMap<String, EventTrackerView>,
    #[serde(default)]
    pub filters: HashMap<String, FilterView>,
    #[serde(default)]
    pub batch_inference_jobs: HashMap<String, BatchInferenceJobView>,
    #[serde(default)]
    pub batch_segment_jobs: HashMap<String, BatchSegmentJobView>,
    #[serde(default)]
    pub data_deletion_jobs: HashMap<String, DataDeletionJobView>,
    #[serde(default)]
    pub dataset_export_jobs: HashMap<String, DatasetExportJobView>,
    #[serde(default)]
    pub dataset_import_jobs: HashMap<String, DatasetImportJobView>,
    #[serde(default)]
    pub metric_attributions: HashMap<String, MetricAttributionView>,
    #[serde(default)]
    pub recommenders: HashMap<String, RecommenderView>,
    #[serde(default)]
    pub tags: HashMap<String, Vec<ResourceTagView>>,
    #[serde(default)]
    pub solutions: HashMap<String, SolutionView>,
    #[serde(default)]
    pub solution_versions: HashMap<String, SolutionVersionView>,
}

// --- From internal types to view types ---

impl From<&crate::types::DatasetGroup> for DatasetGroupView {
    fn from(dg: &crate::types::DatasetGroup) -> Self {
        DatasetGroupView {
            name: dg.name.clone(),
            dataset_group_arn: dg.dataset_group_arn.clone(),
            status: dg.status.clone(),
            role_arn: dg.role_arn.clone(),
            kms_key_arn: dg.kms_key_arn.clone(),
            domain: dg.domain.clone(),
            creation_date_time: dg.creation_date_time,
            last_updated_date_time: dg.last_updated_date_time,
        }
    }
}

impl From<&crate::types::Schema> for SchemaView {
    fn from(s: &crate::types::Schema) -> Self {
        SchemaView {
            name: s.name.clone(),
            schema_arn: s.schema_arn.clone(),
            schema: s.schema.clone(),
            domain: s.domain.clone(),
            creation_date_time: s.creation_date_time,
            last_updated_date_time: s.last_updated_date_time,
        }
    }
}

impl From<&crate::types::Dataset> for DatasetView {
    fn from(d: &crate::types::Dataset) -> Self {
        DatasetView {
            name: d.name.clone(),
            dataset_arn: d.dataset_arn.clone(),
            dataset_group_arn: d.dataset_group_arn.clone(),
            dataset_type: d.dataset_type.clone(),
            schema_arn: d.schema_arn.clone(),
            status: d.status.clone(),
            creation_date_time: d.creation_date_time,
            last_updated_date_time: d.last_updated_date_time,
        }
    }
}

impl From<&crate::types::Campaign> for CampaignView {
    fn from(c: &crate::types::Campaign) -> Self {
        CampaignView {
            name: c.name.clone(),
            campaign_arn: c.campaign_arn.clone(),
            solution_version_arn: c.solution_version_arn.clone(),
            min_provisioned_tps: c.min_provisioned_tps,
            status: c.status.clone(),
            creation_date_time: c.creation_date_time,
            last_updated_date_time: c.last_updated_date_time,
        }
    }
}

impl From<&crate::types::EventTracker> for EventTrackerView {
    fn from(et: &crate::types::EventTracker) -> Self {
        EventTrackerView {
            name: et.name.clone(),
            event_tracker_arn: et.event_tracker_arn.clone(),
            dataset_group_arn: et.dataset_group_arn.clone(),
            tracking_id: et.tracking_id.clone(),
            account_id: et.account_id.clone(),
            status: et.status.clone(),
            creation_date_time: et.creation_date_time,
            last_updated_date_time: et.last_updated_date_time,
        }
    }
}

impl From<&crate::types::Filter> for FilterView {
    fn from(f: &crate::types::Filter) -> Self {
        FilterView {
            name: f.name.clone(),
            filter_arn: f.filter_arn.clone(),
            dataset_group_arn: f.dataset_group_arn.clone(),
            filter_expression: f.filter_expression.clone(),
            status: f.status.clone(),
            creation_date_time: f.creation_date_time,
            last_updated_date_time: f.last_updated_date_time,
        }
    }
}

impl From<&crate::types::BatchInferenceJob> for BatchInferenceJobView {
    fn from(j: &crate::types::BatchInferenceJob) -> Self {
        BatchInferenceJobView {
            job_name: j.job_name.clone(),
            batch_inference_job_arn: j.batch_inference_job_arn.clone(),
            solution_version_arn: j.solution_version_arn.clone(),
            filter_arn: j.filter_arn.clone(),
            role_arn: j.role_arn.clone(),
            status: j.status.clone(),
            num_results: j.num_results,
            job_input_s3_path: j.job_input_s3_path.clone(),
            job_output_s3_path: j.job_output_s3_path.clone(),
            batch_inference_job_mode: j.batch_inference_job_mode.clone(),
            creation_date_time: j.creation_date_time,
            last_updated_date_time: j.last_updated_date_time,
        }
    }
}

impl From<&crate::types::BatchSegmentJob> for BatchSegmentJobView {
    fn from(j: &crate::types::BatchSegmentJob) -> Self {
        BatchSegmentJobView {
            job_name: j.job_name.clone(),
            batch_segment_job_arn: j.batch_segment_job_arn.clone(),
            solution_version_arn: j.solution_version_arn.clone(),
            filter_arn: j.filter_arn.clone(),
            role_arn: j.role_arn.clone(),
            num_results: j.num_results,
            job_input_s3_path: j.job_input_s3_path.clone(),
            job_output_s3_path: j.job_output_s3_path.clone(),
            status: j.status.clone(),
            creation_date_time: j.creation_date_time,
            last_updated_date_time: j.last_updated_date_time,
        }
    }
}

impl From<&crate::types::DataDeletionJob> for DataDeletionJobView {
    fn from(j: &crate::types::DataDeletionJob) -> Self {
        DataDeletionJobView {
            job_name: j.job_name.clone(),
            data_deletion_job_arn: j.data_deletion_job_arn.clone(),
            dataset_group_arn: j.dataset_group_arn.clone(),
            data_source_location: j.data_source_location.clone(),
            role_arn: j.role_arn.clone(),
            status: j.status.clone(),
            creation_date_time: j.creation_date_time,
            last_updated_date_time: j.last_updated_date_time,
        }
    }
}

impl From<&crate::types::DatasetExportJob> for DatasetExportJobView {
    fn from(j: &crate::types::DatasetExportJob) -> Self {
        DatasetExportJobView {
            job_name: j.job_name.clone(),
            dataset_export_job_arn: j.dataset_export_job_arn.clone(),
            dataset_arn: j.dataset_arn.clone(),
            role_arn: j.role_arn.clone(),
            ingestion_mode: j.ingestion_mode.clone(),
            job_output_s3_path: j.job_output_s3_path.clone(),
            status: j.status.clone(),
            creation_date_time: j.creation_date_time,
            last_updated_date_time: j.last_updated_date_time,
        }
    }
}

impl From<&crate::types::DatasetImportJob> for DatasetImportJobView {
    fn from(j: &crate::types::DatasetImportJob) -> Self {
        DatasetImportJobView {
            job_name: j.job_name.clone(),
            dataset_import_job_arn: j.dataset_import_job_arn.clone(),
            dataset_arn: j.dataset_arn.clone(),
            data_source_location: j.data_source_location.clone(),
            role_arn: j.role_arn.clone(),
            import_mode: j.import_mode.clone(),
            status: j.status.clone(),
            creation_date_time: j.creation_date_time,
            last_updated_date_time: j.last_updated_date_time,
        }
    }
}

impl From<&crate::types::MetricAttributeEntry> for MetricAttributeEntryView {
    fn from(m: &crate::types::MetricAttributeEntry) -> Self {
        MetricAttributeEntryView {
            event_type: m.event_type.clone(),
            expression: m.expression.clone(),
            metric_name: m.metric_name.clone(),
        }
    }
}

impl From<&crate::types::MetricAttribution> for MetricAttributionView {
    fn from(ma: &crate::types::MetricAttribution) -> Self {
        MetricAttributionView {
            name: ma.name.clone(),
            metric_attribution_arn: ma.metric_attribution_arn.clone(),
            dataset_group_arn: ma.dataset_group_arn.clone(),
            metrics_output_role_arn: ma.metrics_output_role_arn.clone(),
            metrics_output_s3_path: ma.metrics_output_s3_path.clone(),
            metrics: ma
                .metrics
                .iter()
                .map(MetricAttributeEntryView::from)
                .collect(),
            status: ma.status.clone(),
            creation_date_time: ma.creation_date_time,
            last_updated_date_time: ma.last_updated_date_time,
        }
    }
}

impl From<&crate::types::RecommenderConfigData> for RecommenderConfigView {
    fn from(c: &crate::types::RecommenderConfigData) -> Self {
        RecommenderConfigView {
            enable_metadata_with_recommendations: c.enable_metadata_with_recommendations,
            item_exploration_config: c.item_exploration_config.clone(),
            min_recommendation_requests_per_second: c.min_recommendation_requests_per_second,
        }
    }
}

impl From<&crate::types::Recommender> for RecommenderView {
    fn from(r: &crate::types::Recommender) -> Self {
        RecommenderView {
            name: r.name.clone(),
            recommender_arn: r.recommender_arn.clone(),
            dataset_group_arn: r.dataset_group_arn.clone(),
            recipe_arn: r.recipe_arn.clone(),
            recommender_config: r
                .recommender_config
                .as_ref()
                .map(RecommenderConfigView::from),
            status: r.status.clone(),
            creation_date_time: r.creation_date_time,
            last_updated_date_time: r.last_updated_date_time,
        }
    }
}

impl From<&crate::types::ResourceTag> for ResourceTagView {
    fn from(t: &crate::types::ResourceTag) -> Self {
        ResourceTagView {
            tag_key: t.tag_key.clone(),
            tag_value: t.tag_value.clone(),
        }
    }
}

impl From<&crate::types::Solution> for SolutionView {
    fn from(s: &crate::types::Solution) -> Self {
        SolutionView {
            name: s.name.clone(),
            solution_arn: s.solution_arn.clone(),
            dataset_group_arn: s.dataset_group_arn.clone(),
            recipe_arn: s.recipe_arn.clone(),
            event_type: s.event_type.clone(),
            perform_auto_ml: s.perform_auto_ml,
            perform_auto_training: s.perform_auto_training,
            perform_hpo: s.perform_hpo,
            status: s.status.clone(),
            creation_date_time: s.creation_date_time,
            last_updated_date_time: s.last_updated_date_time,
        }
    }
}

impl From<&crate::types::SolutionVersionData> for SolutionVersionView {
    fn from(sv: &crate::types::SolutionVersionData) -> Self {
        SolutionVersionView {
            solution_version_arn: sv.solution_version_arn.clone(),
            solution_arn: sv.solution_arn.clone(),
            name: sv.name.clone(),
            training_mode: sv.training_mode.clone(),
            status: sv.status.clone(),
            creation_date_time: sv.creation_date_time,
            last_updated_date_time: sv.last_updated_date_time,
        }
    }
}

// --- From view types to internal types ---

impl From<DatasetGroupView> for crate::types::DatasetGroup {
    fn from(v: DatasetGroupView) -> Self {
        crate::types::DatasetGroup {
            name: v.name,
            dataset_group_arn: v.dataset_group_arn,
            status: v.status,
            role_arn: v.role_arn,
            kms_key_arn: v.kms_key_arn,
            domain: v.domain,
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
        }
    }
}

impl From<SchemaView> for crate::types::Schema {
    fn from(v: SchemaView) -> Self {
        crate::types::Schema {
            name: v.name,
            schema_arn: v.schema_arn,
            schema: v.schema,
            domain: v.domain,
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
        }
    }
}

impl From<DatasetView> for crate::types::Dataset {
    fn from(v: DatasetView) -> Self {
        crate::types::Dataset {
            name: v.name,
            dataset_arn: v.dataset_arn,
            dataset_group_arn: v.dataset_group_arn,
            dataset_type: v.dataset_type,
            schema_arn: v.schema_arn,
            status: v.status,
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
        }
    }
}

impl From<CampaignView> for crate::types::Campaign {
    fn from(v: CampaignView) -> Self {
        crate::types::Campaign {
            name: v.name,
            campaign_arn: v.campaign_arn,
            solution_version_arn: v.solution_version_arn,
            min_provisioned_tps: v.min_provisioned_tps,
            status: v.status,
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
        }
    }
}

impl From<EventTrackerView> for crate::types::EventTracker {
    fn from(v: EventTrackerView) -> Self {
        crate::types::EventTracker {
            name: v.name,
            event_tracker_arn: v.event_tracker_arn,
            dataset_group_arn: v.dataset_group_arn,
            tracking_id: v.tracking_id,
            account_id: v.account_id,
            status: v.status,
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
        }
    }
}

impl From<FilterView> for crate::types::Filter {
    fn from(v: FilterView) -> Self {
        crate::types::Filter {
            name: v.name,
            filter_arn: v.filter_arn,
            dataset_group_arn: v.dataset_group_arn,
            filter_expression: v.filter_expression,
            status: v.status,
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
        }
    }
}

impl From<BatchInferenceJobView> for crate::types::BatchInferenceJob {
    fn from(v: BatchInferenceJobView) -> Self {
        crate::types::BatchInferenceJob {
            job_name: v.job_name,
            batch_inference_job_arn: v.batch_inference_job_arn,
            solution_version_arn: v.solution_version_arn,
            filter_arn: v.filter_arn,
            role_arn: v.role_arn,
            status: v.status,
            num_results: v.num_results,
            job_input_s3_path: v.job_input_s3_path,
            job_output_s3_path: v.job_output_s3_path,
            batch_inference_job_mode: v.batch_inference_job_mode,
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
        }
    }
}

impl From<BatchSegmentJobView> for crate::types::BatchSegmentJob {
    fn from(v: BatchSegmentJobView) -> Self {
        crate::types::BatchSegmentJob {
            job_name: v.job_name,
            batch_segment_job_arn: v.batch_segment_job_arn,
            solution_version_arn: v.solution_version_arn,
            filter_arn: v.filter_arn,
            role_arn: v.role_arn,
            num_results: v.num_results,
            job_input_s3_path: v.job_input_s3_path,
            job_output_s3_path: v.job_output_s3_path,
            status: v.status,
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
        }
    }
}

impl From<DataDeletionJobView> for crate::types::DataDeletionJob {
    fn from(v: DataDeletionJobView) -> Self {
        crate::types::DataDeletionJob {
            job_name: v.job_name,
            data_deletion_job_arn: v.data_deletion_job_arn,
            dataset_group_arn: v.dataset_group_arn,
            data_source_location: v.data_source_location,
            role_arn: v.role_arn,
            status: v.status,
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
        }
    }
}

impl From<DatasetExportJobView> for crate::types::DatasetExportJob {
    fn from(v: DatasetExportJobView) -> Self {
        crate::types::DatasetExportJob {
            job_name: v.job_name,
            dataset_export_job_arn: v.dataset_export_job_arn,
            dataset_arn: v.dataset_arn,
            role_arn: v.role_arn,
            ingestion_mode: v.ingestion_mode,
            job_output_s3_path: v.job_output_s3_path,
            status: v.status,
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
        }
    }
}

impl From<DatasetImportJobView> for crate::types::DatasetImportJob {
    fn from(v: DatasetImportJobView) -> Self {
        crate::types::DatasetImportJob {
            job_name: v.job_name,
            dataset_import_job_arn: v.dataset_import_job_arn,
            dataset_arn: v.dataset_arn,
            data_source_location: v.data_source_location,
            role_arn: v.role_arn,
            import_mode: v.import_mode,
            status: v.status,
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
        }
    }
}

impl From<MetricAttributeEntryView> for crate::types::MetricAttributeEntry {
    fn from(v: MetricAttributeEntryView) -> Self {
        crate::types::MetricAttributeEntry {
            event_type: v.event_type,
            expression: v.expression,
            metric_name: v.metric_name,
        }
    }
}

impl From<MetricAttributionView> for crate::types::MetricAttribution {
    fn from(v: MetricAttributionView) -> Self {
        crate::types::MetricAttribution {
            name: v.name,
            metric_attribution_arn: v.metric_attribution_arn,
            dataset_group_arn: v.dataset_group_arn,
            metrics_output_role_arn: v.metrics_output_role_arn,
            metrics_output_s3_path: v.metrics_output_s3_path,
            metrics: v
                .metrics
                .into_iter()
                .map(crate::types::MetricAttributeEntry::from)
                .collect(),
            status: v.status,
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
        }
    }
}

impl From<RecommenderConfigView> for crate::types::RecommenderConfigData {
    fn from(v: RecommenderConfigView) -> Self {
        crate::types::RecommenderConfigData {
            enable_metadata_with_recommendations: v.enable_metadata_with_recommendations,
            item_exploration_config: v.item_exploration_config,
            min_recommendation_requests_per_second: v.min_recommendation_requests_per_second,
        }
    }
}

impl From<RecommenderView> for crate::types::Recommender {
    fn from(v: RecommenderView) -> Self {
        crate::types::Recommender {
            name: v.name,
            recommender_arn: v.recommender_arn,
            dataset_group_arn: v.dataset_group_arn,
            recipe_arn: v.recipe_arn,
            recommender_config: v
                .recommender_config
                .map(crate::types::RecommenderConfigData::from),
            status: v.status,
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
        }
    }
}

impl From<ResourceTagView> for crate::types::ResourceTag {
    fn from(v: ResourceTagView) -> Self {
        crate::types::ResourceTag {
            tag_key: v.tag_key,
            tag_value: v.tag_value,
        }
    }
}

impl From<SolutionView> for crate::types::Solution {
    fn from(v: SolutionView) -> Self {
        crate::types::Solution {
            name: v.name,
            solution_arn: v.solution_arn,
            dataset_group_arn: v.dataset_group_arn,
            recipe_arn: v.recipe_arn,
            event_type: v.event_type,
            perform_auto_ml: v.perform_auto_ml,
            perform_auto_training: v.perform_auto_training,
            perform_hpo: v.perform_hpo,
            status: v.status,
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
        }
    }
}

impl From<SolutionVersionView> for crate::types::SolutionVersionData {
    fn from(v: SolutionVersionView) -> Self {
        crate::types::SolutionVersionData {
            solution_version_arn: v.solution_version_arn,
            solution_arn: v.solution_arn,
            name: v.name,
            training_mode: v.training_mode,
            status: v.status,
            creation_date_time: v.creation_date_time,
            last_updated_date_time: v.last_updated_date_time,
        }
    }
}

impl From<&PersonalizeState> for PersonalizeStateView {
    fn from(state: &PersonalizeState) -> Self {
        PersonalizeStateView {
            dataset_groups: state
                .dataset_groups
                .iter()
                .map(|(k, v)| (k.clone(), DatasetGroupView::from(v)))
                .collect(),
            schemas: state
                .schemas
                .iter()
                .map(|(k, v)| (k.clone(), SchemaView::from(v)))
                .collect(),
            datasets: state
                .datasets
                .iter()
                .map(|(k, v)| (k.clone(), DatasetView::from(v)))
                .collect(),
            campaigns: state
                .campaigns
                .iter()
                .map(|(k, v)| (k.clone(), CampaignView::from(v)))
                .collect(),
            event_trackers: state
                .event_trackers
                .iter()
                .map(|(k, v)| (k.clone(), EventTrackerView::from(v)))
                .collect(),
            filters: state
                .filters
                .iter()
                .map(|(k, v)| (k.clone(), FilterView::from(v)))
                .collect(),
            batch_inference_jobs: state
                .batch_inference_jobs
                .iter()
                .map(|(k, v)| (k.clone(), BatchInferenceJobView::from(v)))
                .collect(),
            batch_segment_jobs: state
                .batch_segment_jobs
                .iter()
                .map(|(k, v)| (k.clone(), BatchSegmentJobView::from(v)))
                .collect(),
            data_deletion_jobs: state
                .data_deletion_jobs
                .iter()
                .map(|(k, v)| (k.clone(), DataDeletionJobView::from(v)))
                .collect(),
            dataset_export_jobs: state
                .dataset_export_jobs
                .iter()
                .map(|(k, v)| (k.clone(), DatasetExportJobView::from(v)))
                .collect(),
            dataset_import_jobs: state
                .dataset_import_jobs
                .iter()
                .map(|(k, v)| (k.clone(), DatasetImportJobView::from(v)))
                .collect(),
            metric_attributions: state
                .metric_attributions
                .iter()
                .map(|(k, v)| (k.clone(), MetricAttributionView::from(v)))
                .collect(),
            recommenders: state
                .recommenders
                .iter()
                .map(|(k, v)| (k.clone(), RecommenderView::from(v)))
                .collect(),
            tags: state
                .tags
                .iter()
                .map(|(k, v)| (k.clone(), v.iter().map(ResourceTagView::from).collect()))
                .collect(),
            solutions: state
                .solutions
                .iter()
                .map(|(k, v)| (k.clone(), SolutionView::from(v)))
                .collect(),
            solution_versions: state
                .solution_versions
                .iter()
                .map(|(k, v)| (k.clone(), SolutionVersionView::from(v)))
                .collect(),
        }
    }
}

// --- From view types to state ---

impl From<PersonalizeStateView> for PersonalizeState {
    fn from(view: PersonalizeStateView) -> Self {
        let mut dataset_group_names = HashMap::new();
        let mut schema_names = HashMap::new();
        let mut dataset_names = HashMap::new();
        let mut campaign_names = HashMap::new();
        let mut event_tracker_names = HashMap::new();
        let mut filter_names = HashMap::new();
        let mut metric_attribution_names = HashMap::new();
        let mut recommender_names = HashMap::new();
        let mut solution_names = HashMap::new();

        let dataset_groups: HashMap<String, crate::types::DatasetGroup> = view
            .dataset_groups
            .into_iter()
            .map(|(k, v)| {
                dataset_group_names.insert(v.name.clone(), k.clone());
                (k, crate::types::DatasetGroup::from(v))
            })
            .collect();

        let schemas: HashMap<String, crate::types::Schema> = view
            .schemas
            .into_iter()
            .map(|(k, v)| {
                schema_names.insert(v.name.clone(), k.clone());
                (k, crate::types::Schema::from(v))
            })
            .collect();

        let datasets: HashMap<String, crate::types::Dataset> = view
            .datasets
            .into_iter()
            .map(|(k, v)| {
                dataset_names.insert(v.name.clone(), k.clone());
                (k, crate::types::Dataset::from(v))
            })
            .collect();

        let campaigns: HashMap<String, crate::types::Campaign> = view
            .campaigns
            .into_iter()
            .map(|(k, v)| {
                campaign_names.insert(v.name.clone(), k.clone());
                (k, crate::types::Campaign::from(v))
            })
            .collect();

        let event_trackers: HashMap<String, crate::types::EventTracker> = view
            .event_trackers
            .into_iter()
            .map(|(k, v)| {
                event_tracker_names.insert(v.name.clone(), k.clone());
                (k, crate::types::EventTracker::from(v))
            })
            .collect();

        let filters: HashMap<String, crate::types::Filter> = view
            .filters
            .into_iter()
            .map(|(k, v)| {
                filter_names.insert(v.name.clone(), k.clone());
                (k, crate::types::Filter::from(v))
            })
            .collect();

        let batch_inference_jobs: HashMap<String, crate::types::BatchInferenceJob> = view
            .batch_inference_jobs
            .into_iter()
            .map(|(k, v)| (k, crate::types::BatchInferenceJob::from(v)))
            .collect();

        let batch_segment_jobs: HashMap<String, crate::types::BatchSegmentJob> = view
            .batch_segment_jobs
            .into_iter()
            .map(|(k, v)| (k, crate::types::BatchSegmentJob::from(v)))
            .collect();

        let data_deletion_jobs: HashMap<String, crate::types::DataDeletionJob> = view
            .data_deletion_jobs
            .into_iter()
            .map(|(k, v)| (k, crate::types::DataDeletionJob::from(v)))
            .collect();

        let dataset_export_jobs: HashMap<String, crate::types::DatasetExportJob> = view
            .dataset_export_jobs
            .into_iter()
            .map(|(k, v)| (k, crate::types::DatasetExportJob::from(v)))
            .collect();

        let dataset_import_jobs: HashMap<String, crate::types::DatasetImportJob> = view
            .dataset_import_jobs
            .into_iter()
            .map(|(k, v)| (k, crate::types::DatasetImportJob::from(v)))
            .collect();

        let metric_attributions: HashMap<String, crate::types::MetricAttribution> = view
            .metric_attributions
            .into_iter()
            .map(|(k, v)| {
                metric_attribution_names.insert(v.name.clone(), k.clone());
                (k, crate::types::MetricAttribution::from(v))
            })
            .collect();

        let recommenders: HashMap<String, crate::types::Recommender> = view
            .recommenders
            .into_iter()
            .map(|(k, v)| {
                recommender_names.insert(v.name.clone(), k.clone());
                (k, crate::types::Recommender::from(v))
            })
            .collect();

        let tags: HashMap<String, Vec<crate::types::ResourceTag>> = view
            .tags
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    v.into_iter().map(crate::types::ResourceTag::from).collect(),
                )
            })
            .collect();

        let solutions: HashMap<String, crate::types::Solution> = view
            .solutions
            .into_iter()
            .map(|(k, v)| {
                solution_names.insert(v.name.clone(), k.clone());
                (k, crate::types::Solution::from(v))
            })
            .collect();

        let solution_versions: HashMap<String, crate::types::SolutionVersionData> = view
            .solution_versions
            .into_iter()
            .map(|(k, v)| (k, crate::types::SolutionVersionData::from(v)))
            .collect();

        PersonalizeState {
            dataset_groups,
            dataset_group_names,
            schemas,
            schema_names,
            datasets,
            dataset_names,
            campaigns,
            campaign_names,
            event_trackers,
            event_tracker_names,
            filters,
            filter_names,
            batch_inference_jobs,
            batch_segment_jobs,
            data_deletion_jobs,
            dataset_export_jobs,
            dataset_import_jobs,
            metric_attributions,
            metric_attribution_names,
            recommenders,
            recommender_names,
            tags,
            solutions,
            solution_names,
            solution_versions,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for PersonalizeService {
    type StateView = PersonalizeStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        PersonalizeStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = PersonalizeState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (arn, dg_view) in view.dataset_groups {
                guard
                    .dataset_group_names
                    .insert(dg_view.name.clone(), arn.clone());
                guard
                    .dataset_groups
                    .insert(arn, crate::types::DatasetGroup::from(dg_view));
            }
            for (arn, schema_view) in view.schemas {
                guard
                    .schema_names
                    .insert(schema_view.name.clone(), arn.clone());
                guard
                    .schemas
                    .insert(arn, crate::types::Schema::from(schema_view));
            }
            for (arn, ds_view) in view.datasets {
                guard
                    .dataset_names
                    .insert(ds_view.name.clone(), arn.clone());
                guard
                    .datasets
                    .insert(arn, crate::types::Dataset::from(ds_view));
            }
            for (arn, c_view) in view.campaigns {
                guard
                    .campaign_names
                    .insert(c_view.name.clone(), arn.clone());
                guard
                    .campaigns
                    .insert(arn, crate::types::Campaign::from(c_view));
            }
            for (arn, et_view) in view.event_trackers {
                guard
                    .event_tracker_names
                    .insert(et_view.name.clone(), arn.clone());
                guard
                    .event_trackers
                    .insert(arn, crate::types::EventTracker::from(et_view));
            }
            for (arn, f_view) in view.filters {
                guard.filter_names.insert(f_view.name.clone(), arn.clone());
                guard
                    .filters
                    .insert(arn, crate::types::Filter::from(f_view));
            }
            for (arn, j_view) in view.batch_inference_jobs {
                guard
                    .batch_inference_jobs
                    .insert(arn, crate::types::BatchInferenceJob::from(j_view));
            }
            for (arn, j_view) in view.batch_segment_jobs {
                guard
                    .batch_segment_jobs
                    .insert(arn, crate::types::BatchSegmentJob::from(j_view));
            }
            for (arn, j_view) in view.data_deletion_jobs {
                guard
                    .data_deletion_jobs
                    .insert(arn, crate::types::DataDeletionJob::from(j_view));
            }
            for (arn, j_view) in view.dataset_export_jobs {
                guard
                    .dataset_export_jobs
                    .insert(arn, crate::types::DatasetExportJob::from(j_view));
            }
            for (arn, j_view) in view.dataset_import_jobs {
                guard
                    .dataset_import_jobs
                    .insert(arn, crate::types::DatasetImportJob::from(j_view));
            }
            for (arn, ma_view) in view.metric_attributions {
                guard
                    .metric_attribution_names
                    .insert(ma_view.name.clone(), arn.clone());
                guard
                    .metric_attributions
                    .insert(arn, crate::types::MetricAttribution::from(ma_view));
            }
            for (arn, r_view) in view.recommenders {
                guard
                    .recommender_names
                    .insert(r_view.name.clone(), arn.clone());
                guard
                    .recommenders
                    .insert(arn, crate::types::Recommender::from(r_view));
            }
            for (arn, t_view) in view.tags {
                guard.tags.insert(
                    arn,
                    t_view
                        .into_iter()
                        .map(crate::types::ResourceTag::from)
                        .collect(),
                );
            }
            for (arn, s_view) in view.solutions {
                guard
                    .solution_names
                    .insert(s_view.name.clone(), arn.clone());
                guard
                    .solutions
                    .insert(arn, crate::types::Solution::from(s_view));
            }
            for (arn, sv_view) in view.solution_versions {
                guard
                    .solution_versions
                    .insert(arn, crate::types::SolutionVersionData::from(sv_view));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
