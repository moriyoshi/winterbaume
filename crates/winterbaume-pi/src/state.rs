use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Default)]
pub struct PiState {
    /// Performance analysis reports keyed by AnalysisReportId.
    pub reports: HashMap<String, ReportRecord>,
    /// Tags keyed by resource ARN.
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone)]
pub struct ReportRecord {
    pub analysis_report_id: String,
    pub identifier: String,
    pub service_type: String,
    pub start_time: f64,
    pub end_time: f64,
    pub status: String,
    pub create_time: f64,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Error)]
pub enum PiError {
    #[error("Performance analysis report {id} does not exist.")]
    ReportNotFound { id: String },

    #[error("{message}")]
    Validation { message: String },
}

impl PiState {
    pub fn create_report(&mut self, report: ReportRecord) -> &ReportRecord {
        let id = report.analysis_report_id.clone();
        self.reports.insert(id.clone(), report);
        self.reports.get(&id).unwrap()
    }

    pub fn get_report(&self, id: &str) -> Result<&ReportRecord, PiError> {
        self.reports
            .get(id)
            .ok_or(PiError::ReportNotFound { id: id.to_string() })
    }

    pub fn delete_report(&mut self, id: &str) -> Result<(), PiError> {
        self.reports
            .remove(id)
            .ok_or(PiError::ReportNotFound { id: id.to_string() })?;
        Ok(())
    }

    pub fn list_reports(&self, identifier: &str, service_type: &str) -> Vec<&ReportRecord> {
        let mut items: Vec<&ReportRecord> = self
            .reports
            .values()
            .filter(|r| r.identifier == identifier && r.service_type == service_type)
            .collect();
        items.sort_by(|a, b| a.analysis_report_id.cmp(&b.analysis_report_id));
        items
    }

    pub fn tag_resource(&mut self, arn: &str, tags: HashMap<String, String>) {
        let entry = self.tags.entry(arn.to_string()).or_default();
        for (k, v) in tags {
            entry.insert(k, v);
        }
    }

    pub fn untag_resource(&mut self, arn: &str, keys: &[String]) {
        if let Some(entry) = self.tags.get_mut(arn) {
            for k in keys {
                entry.remove(k);
            }
        }
    }

    pub fn list_tags(&self, arn: &str) -> HashMap<String, String> {
        self.tags.get(arn).cloned().unwrap_or_default()
    }
}
