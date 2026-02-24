use std::collections::HashMap;

use thiserror::Error;
use uuid::Uuid;

use crate::types::*;

#[derive(Debug, Default)]
pub struct PersonalizeState {
    /// Keyed by datasetGroupArn
    pub dataset_groups: HashMap<String, DatasetGroup>,
    /// Maps name -> arn for duplicate detection
    pub dataset_group_names: HashMap<String, String>,
    /// Keyed by schemaArn
    pub schemas: HashMap<String, Schema>,
    /// Maps schema name -> arn for duplicate detection
    pub schema_names: HashMap<String, String>,
    /// Keyed by datasetArn
    pub datasets: HashMap<String, Dataset>,
    /// Maps name -> arn for duplicate detection
    pub dataset_names: HashMap<String, String>,
    /// Keyed by campaignArn
    pub campaigns: HashMap<String, Campaign>,
    /// Maps name -> arn for duplicate detection
    pub campaign_names: HashMap<String, String>,
    /// Keyed by eventTrackerArn
    pub event_trackers: HashMap<String, EventTracker>,
    /// Maps name -> arn for duplicate detection
    pub event_tracker_names: HashMap<String, String>,
    /// Keyed by filterArn
    pub filters: HashMap<String, Filter>,
    /// Maps name -> arn for duplicate detection
    pub filter_names: HashMap<String, String>,
    /// Keyed by batchInferenceJobArn
    pub batch_inference_jobs: HashMap<String, BatchInferenceJob>,
    /// Keyed by batchSegmentJobArn
    pub batch_segment_jobs: HashMap<String, BatchSegmentJob>,
    /// Keyed by dataDeletionJobArn
    pub data_deletion_jobs: HashMap<String, DataDeletionJob>,
    /// Keyed by datasetExportJobArn
    pub dataset_export_jobs: HashMap<String, DatasetExportJob>,
    /// Keyed by datasetImportJobArn
    pub dataset_import_jobs: HashMap<String, DatasetImportJob>,
    /// Keyed by metricAttributionArn
    pub metric_attributions: HashMap<String, MetricAttribution>,
    /// Maps name -> arn for duplicate detection
    pub metric_attribution_names: HashMap<String, String>,
    /// Keyed by recommenderArn
    pub recommenders: HashMap<String, Recommender>,
    /// Maps name -> arn for duplicate detection
    pub recommender_names: HashMap<String, String>,
    /// Keyed by resourceArn -> list of tags
    pub tags: HashMap<String, Vec<ResourceTag>>,
    /// Keyed by solutionArn
    pub solutions: HashMap<String, Solution>,
    /// Maps name -> arn for duplicate detection
    pub solution_names: HashMap<String, String>,
    /// Keyed by solutionVersionArn
    pub solution_versions: HashMap<String, SolutionVersionData>,
}

#[derive(Debug, Error)]
pub enum PersonalizeError {
    #[error("Resource already exists with arn: {arn}")]
    ResourceAlreadyExists { arn: String },

    #[error("Resource arn:aws:personalize not found: {arn}")]
    ResourceNotFound { arn: String },

    #[error("Missing X-Amz-Target header")]
    MissingAction,

    #[error("Invalid JSON body")]
    SerializationException,

    #[error("{field} is required")]
    InvalidInput { field: String },

    #[error("Unknown operation {action}")]
    InvalidAction { action: String },

    #[error("Resource {arn} is in use")]
    ResourceInUse { arn: String },
}

impl PersonalizeState {
    // ======================== Dataset Groups ========================

    pub fn create_dataset_group(
        &mut self,
        name: &str,
        role_arn: Option<&str>,
        kms_key_arn: Option<&str>,
        domain: Option<&str>,
        region: &str,
        account_id: &str,
    ) -> Result<&DatasetGroup, PersonalizeError> {
        if self.dataset_group_names.contains_key(name) {
            return Err(PersonalizeError::ResourceAlreadyExists {
                arn: self.dataset_group_names[name].clone(),
            });
        }

        let id = Uuid::new_v4().to_string();
        let arn = format!("arn:aws:personalize:{region}:{account_id}:dataset-group/{name}-{id}");
        let now = chrono::Utc::now().timestamp() as f64;

        let dg = DatasetGroup {
            name: name.to_string(),
            dataset_group_arn: arn.clone(),
            status: "ACTIVE".to_string(),
            role_arn: role_arn.map(|s| s.to_string()),
            kms_key_arn: kms_key_arn.map(|s| s.to_string()),
            domain: domain.map(|s| s.to_string()),
            creation_date_time: now,
            last_updated_date_time: now,
        };

        self.dataset_group_names
            .insert(name.to_string(), arn.clone());
        self.dataset_groups.insert(arn.clone(), dg);
        Ok(self.dataset_groups.get(&arn).unwrap())
    }

    pub fn describe_dataset_group(
        &self,
        dataset_group_arn: &str,
    ) -> Result<&DatasetGroup, PersonalizeError> {
        self.dataset_groups.get(dataset_group_arn).ok_or_else(|| {
            PersonalizeError::ResourceNotFound {
                arn: dataset_group_arn.to_string(),
            }
        })
    }

    pub fn delete_dataset_group(
        &mut self,
        dataset_group_arn: &str,
    ) -> Result<(), PersonalizeError> {
        match self.dataset_groups.remove(dataset_group_arn) {
            Some(dg) => {
                self.dataset_group_names.remove(&dg.name);
                Ok(())
            }
            None => Err(PersonalizeError::ResourceNotFound {
                arn: dataset_group_arn.to_string(),
            }),
        }
    }

    pub fn list_dataset_groups(&self) -> Vec<&DatasetGroup> {
        self.dataset_groups.values().collect()
    }

    // ======================== Schemas ========================

    pub fn create_schema(
        &mut self,
        name: &str,
        schema: &str,
        domain: Option<&str>,
        region: &str,
        account_id: &str,
    ) -> Result<&Schema, PersonalizeError> {
        if self.schema_names.contains_key(name) {
            return Err(PersonalizeError::ResourceAlreadyExists {
                arn: self.schema_names[name].clone(),
            });
        }

        let id = Uuid::new_v4().to_string();
        let arn = format!("arn:aws:personalize:{region}:{account_id}:schema/{name}-{id}");
        let now = chrono::Utc::now().timestamp() as f64;

        let s = Schema {
            name: name.to_string(),
            schema_arn: arn.clone(),
            schema: schema.to_string(),
            domain: domain.map(|d| d.to_string()),
            creation_date_time: now,
            last_updated_date_time: now,
        };

        self.schema_names.insert(name.to_string(), arn.clone());
        self.schemas.insert(arn.clone(), s);
        Ok(self.schemas.get(&arn).unwrap())
    }

    pub fn describe_schema(&self, schema_arn: &str) -> Result<&Schema, PersonalizeError> {
        self.schemas
            .get(schema_arn)
            .ok_or_else(|| PersonalizeError::ResourceNotFound {
                arn: schema_arn.to_string(),
            })
    }

    pub fn delete_schema(&mut self, schema_arn: &str) -> Result<(), PersonalizeError> {
        match self.schemas.remove(schema_arn) {
            Some(s) => {
                self.schema_names.remove(&s.name);
                Ok(())
            }
            None => Err(PersonalizeError::ResourceNotFound {
                arn: schema_arn.to_string(),
            }),
        }
    }

    pub fn list_schemas(&self) -> Vec<&Schema> {
        self.schemas.values().collect()
    }

    // ======================== Datasets ========================

    pub fn create_dataset(
        &mut self,
        name: &str,
        dataset_group_arn: &str,
        dataset_type: &str,
        schema_arn: &str,
        region: &str,
        account_id: &str,
    ) -> Result<&Dataset, PersonalizeError> {
        // Verify dataset group exists
        if !self.dataset_groups.contains_key(dataset_group_arn) {
            return Err(PersonalizeError::ResourceNotFound {
                arn: dataset_group_arn.to_string(),
            });
        }
        if self.dataset_names.contains_key(name) {
            return Err(PersonalizeError::ResourceAlreadyExists {
                arn: self.dataset_names[name].clone(),
            });
        }

        let id = Uuid::new_v4().to_string();
        let arn = format!("arn:aws:personalize:{region}:{account_id}:dataset/{name}-{id}");
        let now = chrono::Utc::now().timestamp() as f64;

        let ds = Dataset {
            name: name.to_string(),
            dataset_arn: arn.clone(),
            dataset_group_arn: dataset_group_arn.to_string(),
            dataset_type: dataset_type.to_string(),
            schema_arn: schema_arn.to_string(),
            status: "ACTIVE".to_string(),
            creation_date_time: now,
            last_updated_date_time: now,
        };

        self.dataset_names.insert(name.to_string(), arn.clone());
        self.datasets.insert(arn.clone(), ds);
        Ok(self.datasets.get(&arn).unwrap())
    }

    pub fn describe_dataset(&self, dataset_arn: &str) -> Result<&Dataset, PersonalizeError> {
        self.datasets
            .get(dataset_arn)
            .ok_or_else(|| PersonalizeError::ResourceNotFound {
                arn: dataset_arn.to_string(),
            })
    }

    pub fn delete_dataset(&mut self, dataset_arn: &str) -> Result<(), PersonalizeError> {
        match self.datasets.remove(dataset_arn) {
            Some(ds) => {
                self.dataset_names.remove(&ds.name);
                Ok(())
            }
            None => Err(PersonalizeError::ResourceNotFound {
                arn: dataset_arn.to_string(),
            }),
        }
    }

    pub fn list_datasets(&self, dataset_group_arn: Option<&str>) -> Vec<&Dataset> {
        self.datasets
            .values()
            .filter(|ds| {
                dataset_group_arn
                    .map(|g| ds.dataset_group_arn == g)
                    .unwrap_or(true)
            })
            .collect()
    }

    pub fn update_dataset(
        &mut self,
        dataset_arn: &str,
        schema_arn: &str,
    ) -> Result<&Dataset, PersonalizeError> {
        let ds = self.datasets.get_mut(dataset_arn).ok_or_else(|| {
            PersonalizeError::ResourceNotFound {
                arn: dataset_arn.to_string(),
            }
        })?;
        ds.schema_arn = schema_arn.to_string();
        ds.last_updated_date_time = chrono::Utc::now().timestamp() as f64;
        Ok(ds)
    }

    // ======================== Campaigns ========================

    pub fn create_campaign(
        &mut self,
        name: &str,
        solution_version_arn: &str,
        min_provisioned_tps: Option<i32>,
        region: &str,
        account_id: &str,
    ) -> Result<&Campaign, PersonalizeError> {
        if self.campaign_names.contains_key(name) {
            return Err(PersonalizeError::ResourceAlreadyExists {
                arn: self.campaign_names[name].clone(),
            });
        }

        let id = Uuid::new_v4().to_string();
        let arn = format!("arn:aws:personalize:{region}:{account_id}:campaign/{name}-{id}");
        let now = chrono::Utc::now().timestamp() as f64;

        let c = Campaign {
            name: name.to_string(),
            campaign_arn: arn.clone(),
            solution_version_arn: solution_version_arn.to_string(),
            min_provisioned_tps,
            status: "ACTIVE".to_string(),
            creation_date_time: now,
            last_updated_date_time: now,
        };

        self.campaign_names.insert(name.to_string(), arn.clone());
        self.campaigns.insert(arn.clone(), c);
        Ok(self.campaigns.get(&arn).unwrap())
    }

    pub fn describe_campaign(&self, campaign_arn: &str) -> Result<&Campaign, PersonalizeError> {
        self.campaigns
            .get(campaign_arn)
            .ok_or_else(|| PersonalizeError::ResourceNotFound {
                arn: campaign_arn.to_string(),
            })
    }

    pub fn delete_campaign(&mut self, campaign_arn: &str) -> Result<(), PersonalizeError> {
        match self.campaigns.remove(campaign_arn) {
            Some(c) => {
                self.campaign_names.remove(&c.name);
                Ok(())
            }
            None => Err(PersonalizeError::ResourceNotFound {
                arn: campaign_arn.to_string(),
            }),
        }
    }

    pub fn list_campaigns(&self, solution_arn: Option<&str>) -> Vec<&Campaign> {
        self.campaigns
            .values()
            .filter(|c| {
                solution_arn
                    .map(|sa| c.solution_version_arn.contains(sa))
                    .unwrap_or(true)
            })
            .collect()
    }

    pub fn update_campaign(
        &mut self,
        campaign_arn: &str,
        solution_version_arn: Option<&str>,
        min_provisioned_tps: Option<i32>,
    ) -> Result<&Campaign, PersonalizeError> {
        let c = self.campaigns.get_mut(campaign_arn).ok_or_else(|| {
            PersonalizeError::ResourceNotFound {
                arn: campaign_arn.to_string(),
            }
        })?;
        if let Some(sv) = solution_version_arn {
            c.solution_version_arn = sv.to_string();
        }
        if let Some(tps) = min_provisioned_tps {
            c.min_provisioned_tps = Some(tps);
        }
        c.last_updated_date_time = chrono::Utc::now().timestamp() as f64;
        Ok(c)
    }

    // ======================== Event Trackers ========================

    pub fn create_event_tracker(
        &mut self,
        name: &str,
        dataset_group_arn: &str,
        region: &str,
        account_id: &str,
    ) -> Result<&EventTracker, PersonalizeError> {
        if !self.dataset_groups.contains_key(dataset_group_arn) {
            return Err(PersonalizeError::ResourceNotFound {
                arn: dataset_group_arn.to_string(),
            });
        }
        if self.event_tracker_names.contains_key(name) {
            return Err(PersonalizeError::ResourceAlreadyExists {
                arn: self.event_tracker_names[name].clone(),
            });
        }

        let id = Uuid::new_v4().to_string();
        let arn = format!("arn:aws:personalize:{region}:{account_id}:event-tracker/{name}-{id}");
        let tracking_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp() as f64;

        let et = EventTracker {
            name: name.to_string(),
            event_tracker_arn: arn.clone(),
            dataset_group_arn: dataset_group_arn.to_string(),
            tracking_id,
            account_id: account_id.to_string(),
            status: "ACTIVE".to_string(),
            creation_date_time: now,
            last_updated_date_time: now,
        };

        self.event_tracker_names
            .insert(name.to_string(), arn.clone());
        self.event_trackers.insert(arn.clone(), et);
        Ok(self.event_trackers.get(&arn).unwrap())
    }

    pub fn describe_event_tracker(
        &self,
        event_tracker_arn: &str,
    ) -> Result<&EventTracker, PersonalizeError> {
        self.event_trackers.get(event_tracker_arn).ok_or_else(|| {
            PersonalizeError::ResourceNotFound {
                arn: event_tracker_arn.to_string(),
            }
        })
    }

    pub fn delete_event_tracker(
        &mut self,
        event_tracker_arn: &str,
    ) -> Result<(), PersonalizeError> {
        match self.event_trackers.remove(event_tracker_arn) {
            Some(et) => {
                self.event_tracker_names.remove(&et.name);
                Ok(())
            }
            None => Err(PersonalizeError::ResourceNotFound {
                arn: event_tracker_arn.to_string(),
            }),
        }
    }

    pub fn list_event_trackers(&self, dataset_group_arn: Option<&str>) -> Vec<&EventTracker> {
        self.event_trackers
            .values()
            .filter(|et| {
                dataset_group_arn
                    .map(|g| et.dataset_group_arn == g)
                    .unwrap_or(true)
            })
            .collect()
    }

    // ======================== Filters ========================

    pub fn create_filter(
        &mut self,
        name: &str,
        dataset_group_arn: &str,
        filter_expression: &str,
        region: &str,
        account_id: &str,
    ) -> Result<&Filter, PersonalizeError> {
        if !self.dataset_groups.contains_key(dataset_group_arn) {
            return Err(PersonalizeError::ResourceNotFound {
                arn: dataset_group_arn.to_string(),
            });
        }
        if self.filter_names.contains_key(name) {
            return Err(PersonalizeError::ResourceAlreadyExists {
                arn: self.filter_names[name].clone(),
            });
        }

        let id = Uuid::new_v4().to_string();
        let arn = format!("arn:aws:personalize:{region}:{account_id}:filter/{name}-{id}");
        let now = chrono::Utc::now().timestamp() as f64;

        let f = Filter {
            name: name.to_string(),
            filter_arn: arn.clone(),
            dataset_group_arn: dataset_group_arn.to_string(),
            filter_expression: filter_expression.to_string(),
            status: "ACTIVE".to_string(),
            creation_date_time: now,
            last_updated_date_time: now,
        };

        self.filter_names.insert(name.to_string(), arn.clone());
        self.filters.insert(arn.clone(), f);
        Ok(self.filters.get(&arn).unwrap())
    }

    pub fn describe_filter(&self, filter_arn: &str) -> Result<&Filter, PersonalizeError> {
        self.filters
            .get(filter_arn)
            .ok_or_else(|| PersonalizeError::ResourceNotFound {
                arn: filter_arn.to_string(),
            })
    }

    pub fn delete_filter(&mut self, filter_arn: &str) -> Result<(), PersonalizeError> {
        match self.filters.remove(filter_arn) {
            Some(f) => {
                self.filter_names.remove(&f.name);
                Ok(())
            }
            None => Err(PersonalizeError::ResourceNotFound {
                arn: filter_arn.to_string(),
            }),
        }
    }

    pub fn list_filters(&self, dataset_group_arn: Option<&str>) -> Vec<&Filter> {
        self.filters
            .values()
            .filter(|f| {
                dataset_group_arn
                    .map(|g| f.dataset_group_arn == g)
                    .unwrap_or(true)
            })
            .collect()
    }

    // ======================== Batch Inference Jobs ========================

    pub fn create_batch_inference_job(
        &mut self,
        job_name: &str,
        solution_version_arn: &str,
        filter_arn: Option<&str>,
        role_arn: &str,
        num_results: Option<i32>,
        job_input_s3_path: &str,
        job_output_s3_path: &str,
        batch_inference_job_mode: Option<&str>,
        region: &str,
        account_id: &str,
    ) -> Result<&BatchInferenceJob, PersonalizeError> {
        let id = Uuid::new_v4().to_string();
        let arn = format!(
            "arn:aws:personalize:{region}:{account_id}:batch-inference-job/{job_name}-{id}"
        );
        let now = chrono::Utc::now().timestamp() as f64;

        let job = BatchInferenceJob {
            job_name: job_name.to_string(),
            batch_inference_job_arn: arn.clone(),
            solution_version_arn: solution_version_arn.to_string(),
            filter_arn: filter_arn.map(|s| s.to_string()),
            role_arn: role_arn.to_string(),
            status: "ACTIVE".to_string(),
            num_results,
            job_input_s3_path: job_input_s3_path.to_string(),
            job_output_s3_path: job_output_s3_path.to_string(),
            batch_inference_job_mode: batch_inference_job_mode.map(|s| s.to_string()),
            creation_date_time: now,
            last_updated_date_time: now,
        };

        self.batch_inference_jobs.insert(arn.clone(), job);
        Ok(self.batch_inference_jobs.get(&arn).unwrap())
    }

    pub fn describe_batch_inference_job(
        &self,
        arn: &str,
    ) -> Result<&BatchInferenceJob, PersonalizeError> {
        self.batch_inference_jobs
            .get(arn)
            .ok_or_else(|| PersonalizeError::ResourceNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn list_batch_inference_jobs(
        &self,
        solution_version_arn: Option<&str>,
    ) -> Vec<&BatchInferenceJob> {
        self.batch_inference_jobs
            .values()
            .filter(|j| {
                solution_version_arn
                    .map(|sv| j.solution_version_arn == sv)
                    .unwrap_or(true)
            })
            .collect()
    }

    // ======================== Batch Segment Jobs ========================

    pub fn create_batch_segment_job(
        &mut self,
        job_name: &str,
        solution_version_arn: &str,
        filter_arn: Option<&str>,
        role_arn: &str,
        num_results: Option<i32>,
        job_input_s3_path: &str,
        job_output_s3_path: &str,
        region: &str,
        account_id: &str,
    ) -> Result<&BatchSegmentJob, PersonalizeError> {
        let id = Uuid::new_v4().to_string();
        let arn =
            format!("arn:aws:personalize:{region}:{account_id}:batch-segment-job/{job_name}-{id}");
        let now = chrono::Utc::now().timestamp() as f64;

        let job = BatchSegmentJob {
            job_name: job_name.to_string(),
            batch_segment_job_arn: arn.clone(),
            solution_version_arn: solution_version_arn.to_string(),
            filter_arn: filter_arn.map(|s| s.to_string()),
            role_arn: role_arn.to_string(),
            num_results,
            job_input_s3_path: job_input_s3_path.to_string(),
            job_output_s3_path: job_output_s3_path.to_string(),
            status: "ACTIVE".to_string(),
            creation_date_time: now,
            last_updated_date_time: now,
        };

        self.batch_segment_jobs.insert(arn.clone(), job);
        Ok(self.batch_segment_jobs.get(&arn).unwrap())
    }

    pub fn describe_batch_segment_job(
        &self,
        arn: &str,
    ) -> Result<&BatchSegmentJob, PersonalizeError> {
        self.batch_segment_jobs
            .get(arn)
            .ok_or_else(|| PersonalizeError::ResourceNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn list_batch_segment_jobs(
        &self,
        solution_version_arn: Option<&str>,
    ) -> Vec<&BatchSegmentJob> {
        self.batch_segment_jobs
            .values()
            .filter(|j| {
                solution_version_arn
                    .map(|sv| j.solution_version_arn == sv)
                    .unwrap_or(true)
            })
            .collect()
    }

    // ======================== Data Deletion Jobs ========================

    pub fn create_data_deletion_job(
        &mut self,
        job_name: &str,
        dataset_group_arn: &str,
        data_source_location: &str,
        role_arn: &str,
        region: &str,
        account_id: &str,
    ) -> Result<&DataDeletionJob, PersonalizeError> {
        if !self.dataset_groups.contains_key(dataset_group_arn) {
            return Err(PersonalizeError::ResourceNotFound {
                arn: dataset_group_arn.to_string(),
            });
        }

        let id = Uuid::new_v4().to_string();
        let arn =
            format!("arn:aws:personalize:{region}:{account_id}:data-deletion-job/{job_name}-{id}");
        let now = chrono::Utc::now().timestamp() as f64;

        let job = DataDeletionJob {
            job_name: job_name.to_string(),
            data_deletion_job_arn: arn.clone(),
            dataset_group_arn: dataset_group_arn.to_string(),
            data_source_location: data_source_location.to_string(),
            role_arn: role_arn.to_string(),
            status: "ACTIVE".to_string(),
            creation_date_time: now,
            last_updated_date_time: now,
        };

        self.data_deletion_jobs.insert(arn.clone(), job);
        Ok(self.data_deletion_jobs.get(&arn).unwrap())
    }

    pub fn describe_data_deletion_job(
        &self,
        arn: &str,
    ) -> Result<&DataDeletionJob, PersonalizeError> {
        self.data_deletion_jobs
            .get(arn)
            .ok_or_else(|| PersonalizeError::ResourceNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn list_data_deletion_jobs(
        &self,
        dataset_group_arn: Option<&str>,
    ) -> Vec<&DataDeletionJob> {
        self.data_deletion_jobs
            .values()
            .filter(|j| {
                dataset_group_arn
                    .map(|g| j.dataset_group_arn == g)
                    .unwrap_or(true)
            })
            .collect()
    }

    // ======================== Dataset Export Jobs ========================

    pub fn create_dataset_export_job(
        &mut self,
        job_name: &str,
        dataset_arn: &str,
        role_arn: &str,
        ingestion_mode: Option<&str>,
        job_output_s3_path: &str,
        region: &str,
        account_id: &str,
    ) -> Result<&DatasetExportJob, PersonalizeError> {
        if !self.datasets.contains_key(dataset_arn) {
            return Err(PersonalizeError::ResourceNotFound {
                arn: dataset_arn.to_string(),
            });
        }

        let id = Uuid::new_v4().to_string();
        let arn =
            format!("arn:aws:personalize:{region}:{account_id}:dataset-export-job/{job_name}-{id}");
        let now = chrono::Utc::now().timestamp() as f64;

        let job = DatasetExportJob {
            job_name: job_name.to_string(),
            dataset_export_job_arn: arn.clone(),
            dataset_arn: dataset_arn.to_string(),
            role_arn: role_arn.to_string(),
            ingestion_mode: ingestion_mode.map(|s| s.to_string()),
            job_output_s3_path: job_output_s3_path.to_string(),
            status: "ACTIVE".to_string(),
            creation_date_time: now,
            last_updated_date_time: now,
        };

        self.dataset_export_jobs.insert(arn.clone(), job);
        Ok(self.dataset_export_jobs.get(&arn).unwrap())
    }

    pub fn describe_dataset_export_job(
        &self,
        arn: &str,
    ) -> Result<&DatasetExportJob, PersonalizeError> {
        self.dataset_export_jobs
            .get(arn)
            .ok_or_else(|| PersonalizeError::ResourceNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn list_dataset_export_jobs(&self, dataset_arn: Option<&str>) -> Vec<&DatasetExportJob> {
        self.dataset_export_jobs
            .values()
            .filter(|j| dataset_arn.map(|da| j.dataset_arn == da).unwrap_or(true))
            .collect()
    }

    // ======================== Dataset Import Jobs ========================

    pub fn create_dataset_import_job(
        &mut self,
        job_name: &str,
        dataset_arn: &str,
        data_source_location: &str,
        role_arn: Option<&str>,
        import_mode: Option<&str>,
        region: &str,
        account_id: &str,
    ) -> Result<&DatasetImportJob, PersonalizeError> {
        if !self.datasets.contains_key(dataset_arn) {
            return Err(PersonalizeError::ResourceNotFound {
                arn: dataset_arn.to_string(),
            });
        }

        let id = Uuid::new_v4().to_string();
        let arn =
            format!("arn:aws:personalize:{region}:{account_id}:dataset-import-job/{job_name}-{id}");
        let now = chrono::Utc::now().timestamp() as f64;

        let job = DatasetImportJob {
            job_name: job_name.to_string(),
            dataset_import_job_arn: arn.clone(),
            dataset_arn: dataset_arn.to_string(),
            data_source_location: data_source_location.to_string(),
            role_arn: role_arn.map(|s| s.to_string()),
            import_mode: import_mode.map(|s| s.to_string()),
            status: "ACTIVE".to_string(),
            creation_date_time: now,
            last_updated_date_time: now,
        };

        self.dataset_import_jobs.insert(arn.clone(), job);
        Ok(self.dataset_import_jobs.get(&arn).unwrap())
    }

    pub fn describe_dataset_import_job(
        &self,
        arn: &str,
    ) -> Result<&DatasetImportJob, PersonalizeError> {
        self.dataset_import_jobs
            .get(arn)
            .ok_or_else(|| PersonalizeError::ResourceNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn list_dataset_import_jobs(&self, dataset_arn: Option<&str>) -> Vec<&DatasetImportJob> {
        self.dataset_import_jobs
            .values()
            .filter(|j| dataset_arn.map(|da| j.dataset_arn == da).unwrap_or(true))
            .collect()
    }

    // ======================== Metric Attributions ========================

    pub fn create_metric_attribution(
        &mut self,
        name: &str,
        dataset_group_arn: &str,
        metrics: Vec<MetricAttributeEntry>,
        metrics_output_role_arn: &str,
        metrics_output_s3_path: Option<&str>,
        region: &str,
        account_id: &str,
    ) -> Result<&MetricAttribution, PersonalizeError> {
        if !self.dataset_groups.contains_key(dataset_group_arn) {
            return Err(PersonalizeError::ResourceNotFound {
                arn: dataset_group_arn.to_string(),
            });
        }
        if self.metric_attribution_names.contains_key(name) {
            return Err(PersonalizeError::ResourceAlreadyExists {
                arn: self.metric_attribution_names[name].clone(),
            });
        }

        let id = Uuid::new_v4().to_string();
        let arn =
            format!("arn:aws:personalize:{region}:{account_id}:metric-attribution/{name}-{id}");
        let now = chrono::Utc::now().timestamp() as f64;

        let ma = MetricAttribution {
            name: name.to_string(),
            metric_attribution_arn: arn.clone(),
            dataset_group_arn: dataset_group_arn.to_string(),
            metrics_output_role_arn: metrics_output_role_arn.to_string(),
            metrics_output_s3_path: metrics_output_s3_path.map(|s| s.to_string()),
            metrics,
            status: "ACTIVE".to_string(),
            creation_date_time: now,
            last_updated_date_time: now,
        };

        self.metric_attribution_names
            .insert(name.to_string(), arn.clone());
        self.metric_attributions.insert(arn.clone(), ma);
        Ok(self.metric_attributions.get(&arn).unwrap())
    }

    pub fn describe_metric_attribution(
        &self,
        arn: &str,
    ) -> Result<&MetricAttribution, PersonalizeError> {
        self.metric_attributions
            .get(arn)
            .ok_or_else(|| PersonalizeError::ResourceNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn delete_metric_attribution(&mut self, arn: &str) -> Result<(), PersonalizeError> {
        match self.metric_attributions.remove(arn) {
            Some(ma) => {
                self.metric_attribution_names.remove(&ma.name);
                Ok(())
            }
            None => Err(PersonalizeError::ResourceNotFound {
                arn: arn.to_string(),
            }),
        }
    }

    pub fn list_metric_attributions(
        &self,
        dataset_group_arn: Option<&str>,
    ) -> Vec<&MetricAttribution> {
        self.metric_attributions
            .values()
            .filter(|ma| {
                dataset_group_arn
                    .map(|g| ma.dataset_group_arn == g)
                    .unwrap_or(true)
            })
            .collect()
    }

    pub fn update_metric_attribution(
        &mut self,
        arn: &str,
        add_metrics: Option<Vec<MetricAttributeEntry>>,
        remove_metrics: Option<Vec<String>>,
        metrics_output_role_arn: Option<&str>,
        metrics_output_s3_path: Option<&str>,
    ) -> Result<&MetricAttribution, PersonalizeError> {
        let ma = self.metric_attributions.get_mut(arn).ok_or_else(|| {
            PersonalizeError::ResourceNotFound {
                arn: arn.to_string(),
            }
        })?;
        if let Some(remove) = remove_metrics {
            ma.metrics.retain(|m| !remove.contains(&m.metric_name));
        }
        if let Some(add) = add_metrics {
            ma.metrics.extend(add);
        }
        if let Some(role) = metrics_output_role_arn {
            ma.metrics_output_role_arn = role.to_string();
        }
        if let Some(path) = metrics_output_s3_path {
            ma.metrics_output_s3_path = Some(path.to_string());
        }
        ma.last_updated_date_time = chrono::Utc::now().timestamp() as f64;
        Ok(ma)
    }

    pub fn list_metric_attribution_metrics(
        &self,
        metric_attribution_arn: Option<&str>,
    ) -> Result<Vec<&MetricAttributeEntry>, PersonalizeError> {
        match metric_attribution_arn {
            Some(arn) => {
                let ma = self.metric_attributions.get(arn).ok_or_else(|| {
                    PersonalizeError::ResourceNotFound {
                        arn: arn.to_string(),
                    }
                })?;
                Ok(ma.metrics.iter().collect())
            }
            None => Ok(self
                .metric_attributions
                .values()
                .flat_map(|ma| ma.metrics.iter())
                .collect()),
        }
    }

    // ======================== Recommenders ========================

    pub fn create_recommender(
        &mut self,
        name: &str,
        dataset_group_arn: &str,
        recipe_arn: &str,
        config: Option<RecommenderConfigData>,
        region: &str,
        account_id: &str,
    ) -> Result<&Recommender, PersonalizeError> {
        if !self.dataset_groups.contains_key(dataset_group_arn) {
            return Err(PersonalizeError::ResourceNotFound {
                arn: dataset_group_arn.to_string(),
            });
        }
        if self.recommender_names.contains_key(name) {
            return Err(PersonalizeError::ResourceAlreadyExists {
                arn: self.recommender_names[name].clone(),
            });
        }

        let id = Uuid::new_v4().to_string();
        let arn = format!("arn:aws:personalize:{region}:{account_id}:recommender/{name}-{id}");
        let now = chrono::Utc::now().timestamp() as f64;

        let r = Recommender {
            name: name.to_string(),
            recommender_arn: arn.clone(),
            dataset_group_arn: dataset_group_arn.to_string(),
            recipe_arn: recipe_arn.to_string(),
            recommender_config: config,
            status: "ACTIVE".to_string(),
            creation_date_time: now,
            last_updated_date_time: now,
        };

        self.recommender_names.insert(name.to_string(), arn.clone());
        self.recommenders.insert(arn.clone(), r);
        Ok(self.recommenders.get(&arn).unwrap())
    }

    pub fn describe_recommender(&self, arn: &str) -> Result<&Recommender, PersonalizeError> {
        self.recommenders
            .get(arn)
            .ok_or_else(|| PersonalizeError::ResourceNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn delete_recommender(&mut self, arn: &str) -> Result<(), PersonalizeError> {
        match self.recommenders.remove(arn) {
            Some(r) => {
                self.recommender_names.remove(&r.name);
                Ok(())
            }
            None => Err(PersonalizeError::ResourceNotFound {
                arn: arn.to_string(),
            }),
        }
    }

    pub fn list_recommenders(&self, dataset_group_arn: Option<&str>) -> Vec<&Recommender> {
        self.recommenders
            .values()
            .filter(|r| {
                dataset_group_arn
                    .map(|g| r.dataset_group_arn == g)
                    .unwrap_or(true)
            })
            .collect()
    }

    pub fn update_recommender(
        &mut self,
        arn: &str,
        config: RecommenderConfigData,
    ) -> Result<&Recommender, PersonalizeError> {
        let r =
            self.recommenders
                .get_mut(arn)
                .ok_or_else(|| PersonalizeError::ResourceNotFound {
                    arn: arn.to_string(),
                })?;
        r.recommender_config = Some(config);
        r.last_updated_date_time = chrono::Utc::now().timestamp() as f64;
        Ok(r)
    }

    pub fn start_recommender(&mut self, arn: &str) -> Result<&Recommender, PersonalizeError> {
        let r =
            self.recommenders
                .get_mut(arn)
                .ok_or_else(|| PersonalizeError::ResourceNotFound {
                    arn: arn.to_string(),
                })?;
        r.status = "ACTIVE".to_string();
        r.last_updated_date_time = chrono::Utc::now().timestamp() as f64;
        Ok(r)
    }

    pub fn stop_recommender(&mut self, arn: &str) -> Result<&Recommender, PersonalizeError> {
        let r =
            self.recommenders
                .get_mut(arn)
                .ok_or_else(|| PersonalizeError::ResourceNotFound {
                    arn: arn.to_string(),
                })?;
        r.status = "INACTIVE".to_string();
        r.last_updated_date_time = chrono::Utc::now().timestamp() as f64;
        Ok(r)
    }

    // ======================== Tags ========================

    pub fn tag_resource(&mut self, resource_arn: &str, new_tags: Vec<ResourceTag>) {
        let tags = self.tags.entry(resource_arn.to_string()).or_default();
        for new_tag in new_tags {
            if let Some(existing) = tags.iter_mut().find(|t| t.tag_key == new_tag.tag_key) {
                existing.tag_value = new_tag.tag_value;
            } else {
                tags.push(new_tag);
            }
        }
    }

    pub fn untag_resource(&mut self, resource_arn: &str, tag_keys: &[String]) {
        if let Some(tags) = self.tags.get_mut(resource_arn) {
            tags.retain(|t| !tag_keys.contains(&t.tag_key));
        }
    }

    pub fn list_tags_for_resource(&self, resource_arn: &str) -> Vec<&ResourceTag> {
        self.tags
            .get(resource_arn)
            .map(|tags| tags.iter().collect())
            .unwrap_or_default()
    }

    // ======================== Solutions ========================

    pub fn create_solution(
        &mut self,
        name: &str,
        dataset_group_arn: &str,
        recipe_arn: Option<&str>,
        event_type: Option<&str>,
        perform_auto_ml: Option<bool>,
        perform_auto_training: Option<bool>,
        perform_hpo: Option<bool>,
        region: &str,
        account_id: &str,
    ) -> Result<&Solution, PersonalizeError> {
        if !self.dataset_groups.contains_key(dataset_group_arn) {
            return Err(PersonalizeError::ResourceNotFound {
                arn: dataset_group_arn.to_string(),
            });
        }
        if self.solution_names.contains_key(name) {
            return Err(PersonalizeError::ResourceAlreadyExists {
                arn: self.solution_names[name].clone(),
            });
        }

        let id = Uuid::new_v4().to_string();
        let arn = format!("arn:aws:personalize:{region}:{account_id}:solution/{name}-{id}");
        let now = chrono::Utc::now().timestamp() as f64;

        let s = Solution {
            name: name.to_string(),
            solution_arn: arn.clone(),
            dataset_group_arn: dataset_group_arn.to_string(),
            recipe_arn: recipe_arn.map(|s| s.to_string()),
            event_type: event_type.map(|s| s.to_string()),
            perform_auto_ml,
            perform_auto_training,
            perform_hpo,
            status: "ACTIVE".to_string(),
            creation_date_time: now,
            last_updated_date_time: now,
        };

        self.solution_names.insert(name.to_string(), arn.clone());
        self.solutions.insert(arn.clone(), s);
        Ok(self.solutions.get(&arn).unwrap())
    }

    pub fn describe_solution(&self, arn: &str) -> Result<&Solution, PersonalizeError> {
        self.solutions
            .get(arn)
            .ok_or_else(|| PersonalizeError::ResourceNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn delete_solution(&mut self, arn: &str) -> Result<(), PersonalizeError> {
        match self.solutions.remove(arn) {
            Some(s) => {
                self.solution_names.remove(&s.name);
                Ok(())
            }
            None => Err(PersonalizeError::ResourceNotFound {
                arn: arn.to_string(),
            }),
        }
    }

    pub fn list_solutions(&self, dataset_group_arn: Option<&str>) -> Vec<&Solution> {
        self.solutions
            .values()
            .filter(|s| {
                dataset_group_arn
                    .map(|g| s.dataset_group_arn == g)
                    .unwrap_or(true)
            })
            .collect()
    }

    pub fn update_solution(
        &mut self,
        arn: &str,
        perform_auto_training: Option<bool>,
    ) -> Result<&Solution, PersonalizeError> {
        let s = self
            .solutions
            .get_mut(arn)
            .ok_or_else(|| PersonalizeError::ResourceNotFound {
                arn: arn.to_string(),
            })?;
        if let Some(v) = perform_auto_training {
            s.perform_auto_training = Some(v);
        }
        s.last_updated_date_time = chrono::Utc::now().timestamp() as f64;
        Ok(s)
    }

    // ======================== Solution Versions ========================

    pub fn create_solution_version(
        &mut self,
        solution_arn: &str,
        name: Option<&str>,
        training_mode: Option<&str>,
        region: &str,
        account_id: &str,
    ) -> Result<&SolutionVersionData, PersonalizeError> {
        if !self.solutions.contains_key(solution_arn) {
            return Err(PersonalizeError::ResourceNotFound {
                arn: solution_arn.to_string(),
            });
        }

        let id = Uuid::new_v4().to_string();
        let arn = format!("arn:aws:personalize:{region}:{account_id}:solution-version/{id}");
        let now = chrono::Utc::now().timestamp() as f64;

        let sv = SolutionVersionData {
            solution_version_arn: arn.clone(),
            solution_arn: solution_arn.to_string(),
            name: name.map(|s| s.to_string()),
            training_mode: training_mode.map(|s| s.to_string()),
            status: "ACTIVE".to_string(),
            creation_date_time: now,
            last_updated_date_time: now,
        };

        self.solution_versions.insert(arn.clone(), sv);
        Ok(self.solution_versions.get(&arn).unwrap())
    }

    pub fn describe_solution_version(
        &self,
        arn: &str,
    ) -> Result<&SolutionVersionData, PersonalizeError> {
        self.solution_versions
            .get(arn)
            .ok_or_else(|| PersonalizeError::ResourceNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn list_solution_versions(&self, solution_arn: Option<&str>) -> Vec<&SolutionVersionData> {
        self.solution_versions
            .values()
            .filter(|sv| solution_arn.map(|sa| sv.solution_arn == sa).unwrap_or(true))
            .collect()
    }

    pub fn stop_solution_version_creation(&mut self, arn: &str) -> Result<(), PersonalizeError> {
        let sv = self.solution_versions.get_mut(arn).ok_or_else(|| {
            PersonalizeError::ResourceNotFound {
                arn: arn.to_string(),
            }
        })?;
        sv.status = "STOPPED".to_string();
        sv.last_updated_date_time = chrono::Utc::now().timestamp() as f64;
        Ok(())
    }
}
