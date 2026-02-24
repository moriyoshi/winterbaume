use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct BackupSearchState {
    pub search_jobs: HashMap<String, SearchJob>,
    pub export_jobs: HashMap<String, SearchResultExportJob>,
}

#[derive(Debug, Error)]
pub enum BackupSearchError {
    #[error("Search job {identifier} does not exist.")]
    SearchJobNotFound { identifier: String },

    #[error("Export job {identifier} does not exist.")]
    ExportJobNotFound { identifier: String },

    #[error("Resource {arn} does not exist.")]
    ResourceNotFound { arn: String },

    #[error("Search job {identifier} is in status {status} and cannot be stopped.")]
    SearchJobNotRunning { identifier: String, status: String },

    #[error("{message}")]
    Validation { message: String },
}

impl BackupSearchState {
    pub fn create_search_job(&mut self, job: SearchJob) -> &SearchJob {
        let id = job.identifier.clone();
        self.search_jobs.insert(id.clone(), job);
        self.search_jobs.get(&id).unwrap()
    }

    pub fn get_search_job(&self, id: &str) -> Result<&SearchJob, BackupSearchError> {
        self.search_jobs
            .get(id)
            .ok_or_else(|| BackupSearchError::SearchJobNotFound {
                identifier: id.to_string(),
            })
    }

    pub fn stop_search_job(&mut self, id: &str) -> Result<&SearchJob, BackupSearchError> {
        let job =
            self.search_jobs
                .get_mut(id)
                .ok_or_else(|| BackupSearchError::SearchJobNotFound {
                    identifier: id.to_string(),
                })?;
        if job.status != "RUNNING" {
            return Err(BackupSearchError::SearchJobNotRunning {
                identifier: id.to_string(),
                status: job.status.clone(),
            });
        }
        job.status = "STOPPED".to_string();
        job.completion_time = Some(chrono::Utc::now().timestamp());
        Ok(job)
    }

    pub fn list_search_jobs(&self) -> Vec<&SearchJob> {
        let mut items: Vec<&SearchJob> = self.search_jobs.values().collect();
        items.sort_by_key(|j| std::cmp::Reverse(j.creation_time));
        items
    }

    pub fn create_export_job(
        &mut self,
        job: SearchResultExportJob,
    ) -> Result<&SearchResultExportJob, BackupSearchError> {
        if !self.search_jobs.contains_key(&job.search_job_identifier) {
            return Err(BackupSearchError::SearchJobNotFound {
                identifier: job.search_job_identifier.clone(),
            });
        }
        let id = job.identifier.clone();
        self.export_jobs.insert(id.clone(), job);
        Ok(self.export_jobs.get(&id).unwrap())
    }

    pub fn get_export_job(&self, id: &str) -> Result<&SearchResultExportJob, BackupSearchError> {
        self.export_jobs
            .get(id)
            .ok_or_else(|| BackupSearchError::ExportJobNotFound {
                identifier: id.to_string(),
            })
    }

    pub fn list_export_jobs(&self) -> Vec<&SearchResultExportJob> {
        let mut items: Vec<&SearchResultExportJob> = self.export_jobs.values().collect();
        items.sort_by_key(|j| std::cmp::Reverse(j.creation_time));
        items
    }

    fn locate_tags_mut(&mut self, arn: &str) -> Option<&mut HashMap<String, String>> {
        if let Some(j) = self.search_jobs.values_mut().find(|j| j.arn == arn) {
            return Some(&mut j.tags);
        }
        if let Some(j) = self.export_jobs.values_mut().find(|j| j.arn == arn) {
            return Some(&mut j.tags);
        }
        None
    }

    fn locate_tags(&self, arn: &str) -> Option<&HashMap<String, String>> {
        if let Some(j) = self.search_jobs.values().find(|j| j.arn == arn) {
            return Some(&j.tags);
        }
        if let Some(j) = self.export_jobs.values().find(|j| j.arn == arn) {
            return Some(&j.tags);
        }
        None
    }

    pub fn tag_resource(
        &mut self,
        arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), BackupSearchError> {
        let bag = self
            .locate_tags_mut(arn)
            .ok_or_else(|| BackupSearchError::ResourceNotFound {
                arn: arn.to_string(),
            })?;
        for (k, v) in tags {
            bag.insert(k, v);
        }
        Ok(())
    }

    pub fn untag_resource(&mut self, arn: &str, keys: &[String]) -> Result<(), BackupSearchError> {
        let bag = self
            .locate_tags_mut(arn)
            .ok_or_else(|| BackupSearchError::ResourceNotFound {
                arn: arn.to_string(),
            })?;
        for k in keys {
            bag.remove(k);
        }
        Ok(())
    }

    pub fn list_tags(&self, arn: &str) -> Result<HashMap<String, String>, BackupSearchError> {
        self.locate_tags(arn)
            .cloned()
            .ok_or_else(|| BackupSearchError::ResourceNotFound {
                arn: arn.to_string(),
            })
    }
}
