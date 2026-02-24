use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct SdbState {
    /// Exports keyed by export ARN.
    pub exports: HashMap<String, Export>,
    /// Known domain names. Domains are implicitly created when an export is started.
    pub domains: std::collections::HashSet<String>,
    pub(crate) next_export_id: u64,
}

#[derive(Debug, Error)]
pub enum SdbError {
    #[error("The domain '{domain_name}' does not exist.")]
    NoSuchDomain { domain_name: String },

    #[error("The export '{export_arn}' does not exist.")]
    NoSuchExport { export_arn: String },

    #[error("A request with the same client token but different parameters was already submitted.")]
    Conflict,
}

impl SdbState {
    /// Register a domain as existing (for mock purposes).
    pub fn add_domain(&mut self, name: &str) {
        self.domains.insert(name.to_string());
    }

    pub fn start_domain_export(
        &mut self,
        domain_name: &str,
        s3_bucket: &str,
        s3_key_prefix: Option<&str>,
        s3_sse_algorithm: Option<&str>,
        s3_sse_kms_key_id: Option<&str>,
        s3_bucket_owner: Option<&str>,
        client_token: Option<&str>,
        account_id: &str,
        region: &str,
    ) -> Result<&Export, SdbError> {
        if !self.domains.contains(domain_name) {
            return Err(SdbError::NoSuchDomain {
                domain_name: domain_name.to_string(),
            });
        }

        // Idempotency: if a client_token matches an existing export, return it
        if let Some(token) = client_token
            && let Some(existing) = self.exports.values().find(|e| e.client_token == token)
        {
            // Check for conflicts: same token but different parameters
            if existing.domain_name != domain_name || existing.s3_bucket != s3_bucket {
                return Err(SdbError::Conflict);
            }
            let arn = existing.export_arn.clone();
            return Ok(self.exports.get(&arn).unwrap());
        }

        self.next_export_id += 1;
        let export_id = format!("{:08x}", self.next_export_id);
        let export_arn =
            format!("arn:aws:sdb:{region}:{account_id}:domain/{domain_name}/export/{export_id}");

        let token = client_token
            .map(|t| t.to_string())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let now = Utc::now();
        let export = Export {
            export_arn: export_arn.clone(),
            client_token: token,
            export_status: "SUCCEEDED".to_string(),
            domain_name: domain_name.to_string(),
            requested_at: now,
            s3_bucket: s3_bucket.to_string(),
            s3_key_prefix: s3_key_prefix.map(|s| s.to_string()),
            s3_sse_algorithm: s3_sse_algorithm.map(|s| s.to_string()),
            s3_sse_kms_key_id: s3_sse_kms_key_id.map(|s| s.to_string()),
            s3_bucket_owner: s3_bucket_owner.map(|s| s.to_string()),
            failure_code: None,
            failure_message: None,
            export_manifest: Some(format!("{domain_name}-manifest.json")),
            items_count: Some(0),
            export_data_cutoff_time: Some(now),
        };

        self.exports.insert(export_arn.clone(), export);
        Ok(self.exports.get(&export_arn).unwrap())
    }

    pub fn get_export(&self, export_arn: &str) -> Result<&Export, SdbError> {
        self.exports
            .get(export_arn)
            .ok_or_else(|| SdbError::NoSuchExport {
                export_arn: export_arn.to_string(),
            })
    }

    pub fn list_exports(
        &self,
        domain_name: Option<&str>,
        max_results: Option<i32>,
        _next_token: Option<&str>,
    ) -> Result<(Vec<ExportSummary>, Option<String>), SdbError> {
        // Validate domain if specified
        if let Some(domain) = domain_name
            && !self.domains.contains(domain)
        {
            return Err(SdbError::NoSuchDomain {
                domain_name: domain.to_string(),
            });
        }

        let max = max_results.unwrap_or(100) as usize;

        let summaries: Vec<ExportSummary> = self
            .exports
            .values()
            .filter(|e| domain_name.map(|d| e.domain_name == d).unwrap_or(true))
            .take(max)
            .map(|e| ExportSummary {
                export_arn: e.export_arn.clone(),
                export_status: e.export_status.clone(),
                requested_at: e.requested_at,
                domain_name: e.domain_name.clone(),
            })
            .collect();

        // Simple mock: no real pagination
        Ok((summaries, None))
    }
}
