use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigningProfile {
    pub profile_name: String,
    pub profile_version: String,
    pub profile_version_arn: String,
    pub platform_id: String,
    pub status: String,
    pub arn: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub permissions: Vec<ProfilePermission>,
    #[serde(default)]
    pub revision_id: String,
    /// Signature validity period — serialised as `{"type": "DAYS", "value": 135}`.
    #[serde(default)]
    pub signature_validity_period: Option<serde_json::Value>,
    /// ARN of the signing certificate from a `signing_material` block.
    #[serde(default)]
    pub signing_material_certificate_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilePermission {
    pub statement_id: String,
    pub action: String,
    pub principal: String,
    pub profile_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigningJob {
    pub job_id: String,
    pub job_arn: String,
    pub job_owner: String,
    pub profile_name: String,
    pub profile_version: String,
    pub platform_id: String,
    pub status: String,
    pub created_at: f64,
    pub is_revoked: bool,
    pub revocation_reason: Option<String>,
    pub revoked_at: Option<f64>,
}
