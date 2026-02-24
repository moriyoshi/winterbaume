use std::collections::HashMap;

use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct ControlCatalogState {
    /// Controls keyed by ARN.
    pub controls: HashMap<String, Value>,
    /// Domains keyed by ARN.
    pub domains: HashMap<String, Value>,
    /// Objectives keyed by ARN.
    pub objectives: HashMap<String, Value>,
    /// Common controls keyed by ARN.
    pub common_controls: HashMap<String, Value>,
}

#[derive(Debug, Error)]
pub enum ControlCatalogError {
    #[error("Control {arn} not found.")]
    ControlNotFound { arn: String },
}

impl ControlCatalogState {
    pub fn ensure_seeded(&mut self) {
        if !self.controls.is_empty() {
            return;
        }
        // Seed a small canonical catalogue mirroring the public AWS catalogue's shape.
        self.domains.insert(
            "arn:aws:controlcatalog:::domain/SECURITY".to_string(),
            serde_json::json!({
                "Arn": "arn:aws:controlcatalog:::domain/SECURITY",
                "CreateTime": 1_700_000_000.0,
                "Description": "Manage security and compliance.",
                "LastUpdateTime": 1_700_000_000.0,
                "Name": "Security"
            }),
        );
        self.objectives.insert(
            "arn:aws:controlcatalog:::objective/AC".to_string(),
            serde_json::json!({
                "Arn": "arn:aws:controlcatalog:::objective/AC",
                "CreateTime": 1_700_000_000.0,
                "Description": "Maintain access control.",
                "Domain": {"Arn": "arn:aws:controlcatalog:::domain/SECURITY"},
                "LastUpdateTime": 1_700_000_000.0,
                "Name": "Access control"
            }),
        );
        self.common_controls.insert(
            "arn:aws:controlcatalog:::common-control/MFA-REQUIRED".to_string(),
            serde_json::json!({
                "Arn": "arn:aws:controlcatalog:::common-control/MFA-REQUIRED",
                "CreateTime": 1_700_000_000.0,
                "Description": "Require multi-factor authentication.",
                "Domain": {"Arn": "arn:aws:controlcatalog:::domain/SECURITY"},
                "LastUpdateTime": 1_700_000_000.0,
                "Name": "Require MFA",
                "Objective": {"Arn": "arn:aws:controlcatalog:::objective/AC"}
            }),
        );
        self.controls.insert(
            "arn:aws:controlcatalog:::control/AWS-GR_IAM_USER_NO_POLICIES_CHECK".to_string(),
            serde_json::json!({
                "Aliases": ["CT.IAM.PR.1"],
                "Arn": "arn:aws:controlcatalog:::control/AWS-GR_IAM_USER_NO_POLICIES_CHECK",
                "Behavior": "DETECTIVE",
                "CreateTime": 1_700_000_000.0,
                "Description": "Checks that none of your IAM users have policies attached.",
                "Implementation": {"Type": "AWS::Config::ConfigRule", "Identifier": "IAM_USER_NO_POLICIES_CHECK"},
                "Name": "[CT.IAM.PR.1] Require IAM users to attach permissions only via groups",
                "RegionConfiguration": {
                    "Scope": "GLOBAL",
                    "DeployableRegions": []
                },
                "Severity": "MEDIUM"
            }),
        );
    }

    pub fn get_control(&self, arn: &str) -> Result<&Value, ControlCatalogError> {
        self.controls
            .get(arn)
            .ok_or(ControlCatalogError::ControlNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn list_controls(&self) -> Vec<&Value> {
        let mut v: Vec<&Value> = self.controls.values().collect();
        v.sort_by(|a, b| {
            a.get("Arn")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .cmp(b.get("Arn").and_then(|v| v.as_str()).unwrap_or(""))
        });
        v
    }

    pub fn list_domains(&self) -> Vec<&Value> {
        let mut v: Vec<&Value> = self.domains.values().collect();
        v.sort_by(|a, b| {
            a.get("Arn")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .cmp(b.get("Arn").and_then(|v| v.as_str()).unwrap_or(""))
        });
        v
    }

    pub fn list_objectives(&self) -> Vec<&Value> {
        let mut v: Vec<&Value> = self.objectives.values().collect();
        v.sort_by(|a, b| {
            a.get("Arn")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .cmp(b.get("Arn").and_then(|v| v.as_str()).unwrap_or(""))
        });
        v
    }

    pub fn list_common_controls(&self) -> Vec<&Value> {
        let mut v: Vec<&Value> = self.common_controls.values().collect();
        v.sort_by(|a, b| {
            a.get("Arn")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .cmp(b.get("Arn").and_then(|v| v.as_str()).unwrap_or(""))
        });
        v
    }
}
