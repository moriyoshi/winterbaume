use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Default)]
pub struct CostAndUsageReportState {
    pub reports: HashMap<String, ReportRecord>,
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone)]
pub struct ReportRecord {
    pub report_name: String,
    pub time_unit: String,
    pub format: String,
    pub compression: String,
    pub additional_schema_elements: Vec<String>,
    pub s3_bucket: String,
    pub s3_prefix: String,
    pub s3_region: String,
    pub additional_artifacts: Vec<String>,
    pub refresh_closed_reports: Option<bool>,
    pub report_versioning: Option<String>,
    pub billing_view_arn: Option<String>,
}

#[derive(Debug, Error)]
pub enum CurError {
    #[error("Report definition {name} does not exist.")]
    ReportNotFound { name: String },

    #[error("Report definition {name} already exists.")]
    DuplicateReport { name: String },

    #[error("{message}")]
    Validation { message: String },
}

impl CostAndUsageReportState {
    pub fn put_report(&mut self, report: ReportRecord) -> Result<(), CurError> {
        if self.reports.contains_key(&report.report_name) {
            return Err(CurError::DuplicateReport {
                name: report.report_name,
            });
        }
        self.reports.insert(report.report_name.clone(), report);
        Ok(())
    }

    pub fn modify_report(&mut self, report: ReportRecord) -> Result<(), CurError> {
        if !self.reports.contains_key(&report.report_name) {
            return Err(CurError::ReportNotFound {
                name: report.report_name,
            });
        }
        self.reports.insert(report.report_name.clone(), report);
        Ok(())
    }

    pub fn delete_report(&mut self, name: &str) -> bool {
        self.reports.remove(name).is_some()
    }

    pub fn list_reports(&self) -> Vec<&ReportRecord> {
        let mut items: Vec<&ReportRecord> = self.reports.values().collect();
        items.sort_by(|a, b| a.report_name.cmp(&b.report_name));
        items
    }

    pub fn tag_resource(&mut self, name: &str, tags: HashMap<String, String>) {
        let entry = self.tags.entry(name.to_string()).or_default();
        for (k, v) in tags {
            entry.insert(k, v);
        }
    }

    pub fn untag_resource(&mut self, name: &str, keys: &[String]) {
        if let Some(entry) = self.tags.get_mut(name) {
            for k in keys {
                entry.remove(k);
            }
        }
    }

    pub fn list_tags(&self, name: &str) -> HashMap<String, String> {
        self.tags.get(name).cloned().unwrap_or_default()
    }
}
