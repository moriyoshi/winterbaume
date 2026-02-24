use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Default)]
pub struct RecoveryClusterState {
    /// Routing controls keyed by ARN.
    pub routing_controls: HashMap<String, RoutingControlRecord>,
}

#[derive(Debug, Clone)]
pub struct RoutingControlRecord {
    pub arn: String,
    pub name: String,
    pub control_panel_arn: String,
    pub control_panel_name: String,
    pub owner: String,
    pub state: String,
}

#[derive(Debug, Error)]
pub enum RecoveryClusterError {
    #[error("Routing control {arn} not found.")]
    NotFound { arn: String },

    #[error("{message}")]
    Validation { message: String },
}

impl RecoveryClusterState {
    pub fn upsert(&mut self, record: RoutingControlRecord) {
        self.routing_controls.insert(record.arn.clone(), record);
    }

    pub fn get(&self, arn: &str) -> Result<&RoutingControlRecord, RecoveryClusterError> {
        self.routing_controls
            .get(arn)
            .ok_or(RecoveryClusterError::NotFound {
                arn: arn.to_string(),
            })
    }

    pub fn set_state(
        &mut self,
        arn: &str,
        state: String,
    ) -> Result<&RoutingControlRecord, RecoveryClusterError> {
        let entry = self
            .routing_controls
            .get_mut(arn)
            .ok_or(RecoveryClusterError::NotFound {
                arn: arn.to_string(),
            })?;
        entry.state = state;
        Ok(entry)
    }

    pub fn list<'a>(&'a self, control_panel_arn: Option<&'a str>) -> Vec<&'a RoutingControlRecord> {
        let mut v: Vec<&RoutingControlRecord> = self
            .routing_controls
            .values()
            .filter(|c| control_panel_arn.is_none_or(|p| p == c.control_panel_arn))
            .collect();
        v.sort_by(|a, b| a.arn.cmp(&b.arn));
        v
    }

    /// Auto-seed a default control-panel + routing controls so the read APIs
    /// return useful data even without prior `restore` calls. The data-plane
    /// service does not have create-RoutingControl operations of its own; in
    /// real AWS those are created via the route53-recovery-control-config
    /// service.
    pub fn ensure_seeded(&mut self, account_id: &str) {
        if !self.routing_controls.is_empty() {
            return;
        }
        let cp_arn =
            format!("arn:aws:route53-recovery-control::{account_id}:controlpanel/default-cp");
        for name in ["primary", "secondary"] {
            let arn = format!(
                "arn:aws:route53-recovery-control::{account_id}:controlpanel/default-cp/routingcontrol/{name}"
            );
            self.routing_controls.insert(
                arn.clone(),
                RoutingControlRecord {
                    arn,
                    name: name.to_string(),
                    control_panel_arn: cp_arn.clone(),
                    control_panel_name: "default-cp".to_string(),
                    owner: account_id.to_string(),
                    state: if name == "primary" {
                        "On".to_string()
                    } else {
                        "Off".to_string()
                    },
                },
            );
        }
    }
}
