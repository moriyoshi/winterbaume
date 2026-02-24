use std::collections::HashMap;

use thiserror::Error;
use uuid::Uuid;

use crate::types::*;

#[derive(Debug, Default)]
pub struct ArtifactState {
    pub account_settings: AccountSettings,
    /// Reports keyed by reportId.
    pub reports: HashMap<String, Report>,
    /// Customer agreements keyed by their id.
    pub customer_agreements: HashMap<String, CustomerAgreement>,
    /// Issued term tokens that authorise GetReport for a particular reportId.
    pub term_tokens: HashMap<String, String>,
}

#[derive(Debug, Error)]
pub enum ArtifactError {
    #[error("Report {report_id} not found")]
    ReportNotFound { report_id: String },

    #[error("Term token {token} is invalid for report {report_id}")]
    InvalidTermToken { token: String, report_id: String },

    #[error("{message}")]
    Validation { message: String },
}

impl ArtifactState {
    pub fn put_account_settings(&mut self, status: Option<String>) {
        self.account_settings.notification_subscription_status = status;
    }

    pub fn get_report(&self, report_id: &str, token: &str) -> Result<&Report, ArtifactError> {
        let report = self
            .reports
            .get(report_id)
            .ok_or_else(|| ArtifactError::ReportNotFound {
                report_id: report_id.to_string(),
            })?;
        match self.term_tokens.get(report_id) {
            Some(t) if t == token => Ok(report),
            _ => Err(ArtifactError::InvalidTermToken {
                token: token.to_string(),
                report_id: report_id.to_string(),
            }),
        }
    }

    pub fn get_report_metadata(&self, report_id: &str) -> Result<&Report, ArtifactError> {
        self.reports
            .get(report_id)
            .ok_or(ArtifactError::ReportNotFound {
                report_id: report_id.to_string(),
            })
    }

    /// Issue a term token for a report. Real AWS Artifact requires the caller
    /// to fetch and accept terms before downloading; we return a token that
    /// must be passed back via GetReport.
    pub fn issue_term_token(&mut self, report_id: &str) -> Result<(String, String), ArtifactError> {
        let report = self
            .reports
            .get(report_id)
            .ok_or_else(|| ArtifactError::ReportNotFound {
                report_id: report_id.to_string(),
            })?;
        let token = Uuid::new_v4().to_string();
        let url = format!(
            "https://artifact-mock.example/terms/{}/{}",
            report_id, token
        );
        self.term_tokens
            .insert(report_id.to_string(), token.clone());
        let _ = report; // suppress unused warning
        Ok((url, token))
    }

    pub fn list_reports(&self) -> Vec<&Report> {
        let mut all: Vec<&Report> = self.reports.values().collect();
        all.sort_by(|a, b| a.id.cmp(&b.id));
        all
    }

    pub fn list_report_versions(&self, report_id: &str) -> Vec<&Report> {
        // Mock: each report has a single version, so just return the matching report.
        self.reports
            .values()
            .filter(|r| r.id == report_id)
            .collect()
    }

    pub fn list_customer_agreements(&self) -> Vec<&CustomerAgreement> {
        let mut all: Vec<&CustomerAgreement> = self.customer_agreements.values().collect();
        all.sort_by(|a, b| a.id.cmp(&b.id));
        all
    }
}
