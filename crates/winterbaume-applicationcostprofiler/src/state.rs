use std::collections::HashMap;

use thiserror::Error;
use uuid::Uuid;

use crate::types::*;

#[derive(Debug, Default)]
pub struct ApplicationCostProfilerState {
    /// Report definitions keyed by reportId.
    pub reports: HashMap<String, ReportDefinition>,
    /// Import jobs keyed by importId.
    pub import_jobs: HashMap<String, ImportJob>,
}

#[derive(Debug, Error)]
pub enum ApplicationCostProfilerError {
    #[error("Report definition {report_id} not found")]
    ReportNotFound { report_id: String },

    #[error("Report definition {report_id} already exists")]
    ReportAlreadyExists { report_id: String },

    #[error("{message}")]
    Validation { message: String },
}

impl ApplicationCostProfilerState {
    pub fn put_report(
        &mut self,
        report: ReportDefinition,
    ) -> Result<(), ApplicationCostProfilerError> {
        if self.reports.contains_key(&report.report_id) {
            return Err(ApplicationCostProfilerError::ReportAlreadyExists {
                report_id: report.report_id.clone(),
            });
        }
        self.reports.insert(report.report_id.clone(), report);
        Ok(())
    }

    pub fn get_report(
        &self,
        report_id: &str,
    ) -> Result<&ReportDefinition, ApplicationCostProfilerError> {
        self.reports
            .get(report_id)
            .ok_or_else(|| ApplicationCostProfilerError::ReportNotFound {
                report_id: report_id.to_string(),
            })
    }

    pub fn update_report(
        &mut self,
        report_id: &str,
        description: String,
        frequency: String,
        format: String,
        destination: S3Location,
    ) -> Result<(), ApplicationCostProfilerError> {
        let report = self.reports.get_mut(report_id).ok_or_else(|| {
            ApplicationCostProfilerError::ReportNotFound {
                report_id: report_id.to_string(),
            }
        })?;
        report.report_description = description;
        report.report_frequency = frequency;
        report.format = format;
        report.destination_s3_location = destination;
        report.last_updated_at = chrono::Utc::now().timestamp();
        Ok(())
    }

    pub fn delete_report(&mut self, report_id: &str) -> Result<(), ApplicationCostProfilerError> {
        self.reports.remove(report_id).ok_or_else(|| {
            ApplicationCostProfilerError::ReportNotFound {
                report_id: report_id.to_string(),
            }
        })?;
        Ok(())
    }

    pub fn list_reports(&self) -> Vec<&ReportDefinition> {
        let mut all: Vec<&ReportDefinition> = self.reports.values().collect();
        all.sort_by(|a, b| a.report_id.cmp(&b.report_id));
        all
    }

    pub fn import_application_usage(
        &mut self,
        bucket: &str,
        key: &str,
        region: Option<String>,
    ) -> &ImportJob {
        let import_id = Uuid::new_v4().to_string();
        let job = ImportJob {
            import_id: import_id.clone(),
            source_bucket: bucket.to_string(),
            source_key: key.to_string(),
            source_region: region,
            created_at: chrono::Utc::now().timestamp(),
        };
        self.import_jobs.insert(import_id.clone(), job);
        self.import_jobs.get(&import_id).unwrap()
    }
}
