use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;
use uuid::Uuid;

use crate::types::*;

const TRUSTED_ADVISOR_STATUSES: &[&str] =
    &["none", "enqueued", "processing", "success", "abandoned"];

#[derive(Debug, Default)]
pub struct SupportState {
    pub cases: HashMap<String, SupportCase>,
    next_display_id: u64,
    /// Tracks how many times each check ID has been refreshed, for cycling statuses.
    pub refresh_call_counts: HashMap<String, usize>,
}

impl SupportState {
    pub fn next_refresh_status(&mut self, check_id: &str) -> &str {
        let count = self
            .refresh_call_counts
            .entry(check_id.to_string())
            .or_insert(0);
        let status = TRUSTED_ADVISOR_STATUSES[*count % TRUSTED_ADVISOR_STATUSES.len()];
        *count += 1;
        status
    }
}

#[derive(Debug, Error)]
pub enum SupportError {
    #[error("The requested CaseId could not be located.")]
    CaseIdNotFound,
}

impl SupportState {
    pub fn create_case(
        &mut self,
        subject: &str,
        communication_body: &str,
        service_code: Option<&str>,
        severity_code: Option<&str>,
        category_code: Option<&str>,
        cc_email_addresses: Vec<String>,
        language: Option<&str>,
    ) -> Result<&SupportCase, SupportError> {
        let case_uuid = Uuid::new_v4().to_string().replace('-', "");
        let case_id = format!(
            "case-{}-{}-{}",
            &case_uuid[..12],
            Utc::now().format("%Y"),
            &case_uuid[12..27]
        );

        self.next_display_id += 1;
        let display_id = format!("{}", self.next_display_id);

        let support_case = SupportCase {
            case_id: case_id.clone(),
            display_id,
            subject: subject.to_string(),
            status: "opened".to_string(),
            service_code: service_code.unwrap_or("general-info").to_string(),
            category_code: category_code.unwrap_or("other").to_string(),
            severity_code: severity_code.unwrap_or("normal").to_string(),
            communication_body: communication_body.to_string(),
            submitted_by: "user@example.com".to_string(),
            time_created: Utc::now().to_rfc3339(),
            cc_email_addresses,
            language: language.unwrap_or("en").to_string(),
        };

        self.cases.insert(case_id.clone(), support_case);
        Ok(self.cases.get(&case_id).unwrap())
    }

    pub fn describe_cases(
        &self,
        case_id_list: Option<&[String]>,
        include_resolved: bool,
    ) -> Vec<&SupportCase> {
        self.cases
            .values()
            .filter(|c| {
                if !include_resolved && c.status == "resolved" {
                    return false;
                }
                if let Some(ids) = case_id_list
                    && !ids.is_empty()
                {
                    return ids.iter().any(|id| id == &c.case_id);
                }
                true
            })
            .collect()
    }

    pub fn resolve_case(
        &mut self,
        case_id: Option<&str>,
    ) -> Result<(String, String), SupportError> {
        let case_id = match case_id {
            Some(id) => id.to_string(),
            None => {
                // If no caseId, resolve the most recent case
                match self
                    .cases
                    .values()
                    .filter(|c| c.status != "resolved")
                    .last()
                {
                    Some(c) => c.case_id.clone(),
                    None => {
                        return Err(SupportError::CaseIdNotFound);
                    }
                }
            }
        };

        match self.cases.get_mut(&case_id) {
            Some(c) => {
                let initial_status = c.status.clone();
                c.status = "resolved".to_string();
                Ok((initial_status, "resolved".to_string()))
            }
            None => Err(SupportError::CaseIdNotFound),
        }
    }

    pub fn describe_services(&self, service_code_list: Option<&[String]>) -> Vec<SupportService> {
        let all_services = default_services();
        match service_code_list {
            Some(codes) if !codes.is_empty() => all_services
                .into_iter()
                .filter(|s| codes.iter().any(|c| c == &s.code))
                .collect(),
            _ => all_services,
        }
    }
}

fn default_services() -> Vec<SupportService> {
    vec![
        SupportService {
            code: "amazon-elastic-compute-cloud-linux".to_string(),
            name: "Amazon Elastic Compute Cloud (Linux)".to_string(),
            categories: vec![
                SupportCategory {
                    code: "general-info".to_string(),
                    name: "General Info".to_string(),
                },
                SupportCategory {
                    code: "instance-issue".to_string(),
                    name: "Instance Issue".to_string(),
                },
            ],
        },
        SupportService {
            code: "amazon-simple-storage-service".to_string(),
            name: "Amazon Simple Storage Service".to_string(),
            categories: vec![SupportCategory {
                code: "general-info".to_string(),
                name: "General Info".to_string(),
            }],
        },
        SupportService {
            code: "general-info".to_string(),
            name: "General Info".to_string(),
            categories: vec![SupportCategory {
                code: "other".to_string(),
                name: "Other".to_string(),
            }],
        },
    ]
}
