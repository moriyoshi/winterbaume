use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Default)]
pub struct S3OutpostsState {
    /// Endpoints keyed by ARN.
    pub endpoints: HashMap<String, EndpointRecord>,
    /// Outposts catalogue (auto-seeded on first ListOutpostsWithS3 call to keep
    /// the state surface non-empty for callers).
    pub outposts: Vec<OutpostRecord>,
}

#[derive(Debug, Clone)]
pub struct EndpointRecord {
    pub arn: String,
    pub outpost_id: String,
    pub subnet_id: String,
    pub security_group_id: String,
    pub access_type: String,
    pub customer_owned_ipv4_pool: Option<String>,
    pub vpc_id: String,
    pub cidr_block: String,
    pub creation_time: f64,
    pub status: String,
    pub network_interface_ids: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct OutpostRecord {
    pub outpost_id: String,
    pub outpost_arn: String,
    pub s3_outpost_arn: String,
    pub owner_id: String,
    pub capacity_in_bytes: i64,
}

#[derive(Debug, Error)]
pub enum S3OutpostsError {
    #[error("Endpoint {arn} not found.")]
    EndpointNotFound { arn: String },

    #[error("{message}")]
    Validation { message: String },
}

impl S3OutpostsState {
    pub fn create_endpoint(&mut self, record: EndpointRecord) -> &EndpointRecord {
        let arn = record.arn.clone();
        self.endpoints.insert(arn.clone(), record);
        self.endpoints.get(&arn).unwrap()
    }

    pub fn delete_endpoint(&mut self, arn: &str) -> Result<(), S3OutpostsError> {
        self.endpoints
            .remove(arn)
            .ok_or(S3OutpostsError::EndpointNotFound {
                arn: arn.to_string(),
            })?;
        Ok(())
    }

    pub fn list_endpoints(&self) -> Vec<&EndpointRecord> {
        let mut v: Vec<&EndpointRecord> = self.endpoints.values().collect();
        v.sort_by(|a, b| a.arn.cmp(&b.arn));
        v
    }

    pub fn list_outposts(&self) -> &[OutpostRecord] {
        &self.outposts
    }

    pub fn ensure_default_outposts(&mut self, account_id: &str) {
        if !self.outposts.is_empty() {
            return;
        }
        self.outposts.push(OutpostRecord {
            outpost_id: "op-default".to_string(),
            outpost_arn: format!("arn:aws:outposts:us-east-1:{account_id}:outpost/op-default"),
            s3_outpost_arn: format!(
                "arn:aws:s3-outposts:us-east-1:{account_id}:outpost/op-default"
            ),
            owner_id: account_id.to_string(),
            capacity_in_bytes: 11_000_000_000_000,
        });
    }
}
