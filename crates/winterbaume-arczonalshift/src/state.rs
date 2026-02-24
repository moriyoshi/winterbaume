use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug)]
pub struct ArcZonalShiftState {
    /// Active and historical zonal shifts keyed by zonalShiftId.
    pub zonal_shifts: HashMap<String, ZonalShift>,
    /// Practice run configurations keyed by resource identifier (ARN).
    pub practice_run_configurations: HashMap<String, PracticeRunConfiguration>,
    /// Managed resource metadata keyed by resource identifier (ARN).
    pub managed_resources: HashMap<String, ManagedResource>,
    /// Account-level autoshift observer notification status. Defaults to "DISABLED".
    pub autoshift_observer_notification_status: String,
}

impl Default for ArcZonalShiftState {
    fn default() -> Self {
        Self {
            zonal_shifts: HashMap::new(),
            practice_run_configurations: HashMap::new(),
            managed_resources: HashMap::new(),
            autoshift_observer_notification_status: "DISABLED".to_string(),
        }
    }
}

#[derive(Debug, Error)]
pub enum ArcZonalShiftError {
    #[error("Zonal shift {id} not found")]
    ZonalShiftNotFound { id: String },

    #[error("Practice run configuration for resource {resource_identifier} not found")]
    PracticeRunConfigurationNotFound { resource_identifier: String },

    #[error("Managed resource {resource_identifier} not found")]
    ManagedResourceNotFound { resource_identifier: String },

    #[error("Practice run configuration already exists for resource {resource_identifier}")]
    PracticeRunConfigurationAlreadyExists { resource_identifier: String },

    #[error("An active zonal shift already exists for resource {resource_identifier}")]
    ActiveZonalShiftExists { resource_identifier: String },

    #[error("Zonal shift {id} is not active and cannot be modified")]
    ZonalShiftNotActive { id: String },

    #[error("Zonal autoshift cannot be enabled without a practice run configuration")]
    AutoshiftRequiresPracticeRun,

    #[error("{message}")]
    Validation { message: String },
}

impl ArcZonalShiftState {
    pub fn new() -> Self {
        Self {
            autoshift_observer_notification_status: "DISABLED".to_string(),
            ..Default::default()
        }
    }

    fn ensure_managed_resource(&mut self, resource_identifier: &str) -> &mut ManagedResource {
        self.managed_resources
            .entry(resource_identifier.to_string())
            .or_insert_with(|| ManagedResource::new(resource_identifier))
    }

    pub fn start_zonal_shift(
        &mut self,
        resource_identifier: String,
        away_from: String,
        expires_in: String,
        comment: String,
        shift_type: &str,
    ) -> Result<ZonalShift, ArcZonalShiftError> {
        let duration_seconds = parse_duration(&expires_in)?;
        if self
            .zonal_shifts
            .values()
            .any(|s| s.resource_identifier == resource_identifier && s.status == "ACTIVE")
        {
            return Err(ArcZonalShiftError::ActiveZonalShiftExists {
                resource_identifier,
            });
        }

        let now = chrono::Utc::now().timestamp();
        let zonal_shift_id = uuid::Uuid::new_v4().to_string();
        let shift = ZonalShift {
            zonal_shift_id: zonal_shift_id.clone(),
            resource_identifier: resource_identifier.clone(),
            away_from: away_from.clone(),
            comment,
            start_time: now,
            expiry_time: now + duration_seconds,
            status: "ACTIVE".to_string(),
            shift_type: shift_type.to_string(),
            practice_run_outcome: None,
        };

        self.zonal_shifts.insert(zonal_shift_id, shift.clone());
        let resource = self.ensure_managed_resource(&resource_identifier);
        resource.applied_weights.insert(away_from.clone(), 0.0);
        Ok(shift)
    }

    pub fn cancel_zonal_shift(&mut self, id: &str) -> Result<ZonalShift, ArcZonalShiftError> {
        let shift = self
            .zonal_shifts
            .get_mut(id)
            .ok_or_else(|| ArcZonalShiftError::ZonalShiftNotFound { id: id.to_string() })?;
        if shift.status != "ACTIVE" {
            return Err(ArcZonalShiftError::ZonalShiftNotActive { id: id.to_string() });
        }
        shift.status = "CANCELED".to_string();
        let cloned = shift.clone();
        let resource_identifier = cloned.resource_identifier.clone();
        let away_from = cloned.away_from.clone();
        if let Some(resource) = self.managed_resources.get_mut(&resource_identifier) {
            resource.applied_weights.remove(&away_from);
        }
        Ok(cloned)
    }

    pub fn update_zonal_shift(
        &mut self,
        id: &str,
        comment: Option<String>,
        expires_in: Option<String>,
    ) -> Result<ZonalShift, ArcZonalShiftError> {
        let extra_seconds = match expires_in.as_deref() {
            Some(v) => Some(parse_duration(v)?),
            None => None,
        };
        let shift = self
            .zonal_shifts
            .get_mut(id)
            .ok_or_else(|| ArcZonalShiftError::ZonalShiftNotFound { id: id.to_string() })?;
        if shift.status != "ACTIVE" {
            return Err(ArcZonalShiftError::ZonalShiftNotActive { id: id.to_string() });
        }
        if let Some(c) = comment {
            shift.comment = c;
        }
        if let Some(seconds) = extra_seconds {
            let now = chrono::Utc::now().timestamp();
            shift.expiry_time = now + seconds;
        }
        Ok(shift.clone())
    }

    pub fn list_zonal_shifts(&self, status: Option<&str>) -> Vec<&ZonalShift> {
        let mut shifts: Vec<&ZonalShift> = self
            .zonal_shifts
            .values()
            .filter(|s| status.is_none_or(|st| s.status == st))
            .collect();
        shifts.sort_by_key(|b| std::cmp::Reverse(b.start_time));
        shifts
    }

    pub fn create_practice_run_configuration(
        &mut self,
        config: PracticeRunConfiguration,
    ) -> Result<&ManagedResource, ArcZonalShiftError> {
        if config.outcome_alarms.is_empty() {
            return Err(ArcZonalShiftError::Validation {
                message: "outcomeAlarms must contain at least one alarm".to_string(),
            });
        }
        if self
            .practice_run_configurations
            .contains_key(&config.resource_identifier)
        {
            return Err(ArcZonalShiftError::PracticeRunConfigurationAlreadyExists {
                resource_identifier: config.resource_identifier,
            });
        }
        let resource_identifier = config.resource_identifier.clone();
        self.practice_run_configurations
            .insert(resource_identifier.clone(), config);
        let resource = self.ensure_managed_resource(&resource_identifier);
        if resource.zonal_autoshift_status.is_empty() {
            resource.zonal_autoshift_status = "DISABLED".to_string();
        }
        Ok(resource)
    }

    pub fn update_practice_run_configuration(
        &mut self,
        resource_identifier: &str,
        update: impl FnOnce(&mut PracticeRunConfiguration),
    ) -> Result<&ManagedResource, ArcZonalShiftError> {
        let config = self
            .practice_run_configurations
            .get_mut(resource_identifier)
            .ok_or_else(|| ArcZonalShiftError::PracticeRunConfigurationNotFound {
                resource_identifier: resource_identifier.to_string(),
            })?;
        update(config);
        if config.outcome_alarms.is_empty() {
            return Err(ArcZonalShiftError::Validation {
                message: "outcomeAlarms must contain at least one alarm".to_string(),
            });
        }
        let resource = self.managed_resources.get(resource_identifier).ok_or(
            ArcZonalShiftError::ManagedResourceNotFound {
                resource_identifier: resource_identifier.to_string(),
            },
        )?;
        Ok(resource)
    }

    pub fn delete_practice_run_configuration(
        &mut self,
        resource_identifier: &str,
    ) -> Result<&ManagedResource, ArcZonalShiftError> {
        if let Some(resource) = self.managed_resources.get(resource_identifier) {
            if resource.zonal_autoshift_status == "ENABLED" {
                return Err(ArcZonalShiftError::Validation {
                    message: format!(
                        "Cannot delete practice run configuration while zonal autoshift is enabled for resource {resource_identifier}"
                    ),
                });
            }
        }
        self.practice_run_configurations
            .remove(resource_identifier)
            .ok_or_else(|| ArcZonalShiftError::PracticeRunConfigurationNotFound {
                resource_identifier: resource_identifier.to_string(),
            })?;
        let resource = self.managed_resources.get_mut(resource_identifier).ok_or(
            ArcZonalShiftError::ManagedResourceNotFound {
                resource_identifier: resource_identifier.to_string(),
            },
        )?;
        resource.zonal_autoshift_status = "DISABLED".to_string();
        Ok(resource)
    }

    pub fn get_managed_resource(
        &self,
        resource_identifier: &str,
    ) -> Result<&ManagedResource, ArcZonalShiftError> {
        self.managed_resources.get(resource_identifier).ok_or(
            ArcZonalShiftError::ManagedResourceNotFound {
                resource_identifier: resource_identifier.to_string(),
            },
        )
    }

    pub fn list_managed_resources(&self) -> Vec<&ManagedResource> {
        let mut items: Vec<&ManagedResource> = self.managed_resources.values().collect();
        items.sort_by(|a, b| a.resource_identifier.cmp(&b.resource_identifier));
        items
    }

    pub fn update_zonal_autoshift_configuration(
        &mut self,
        resource_identifier: &str,
        status: &str,
    ) -> Result<&ManagedResource, ArcZonalShiftError> {
        if status == "ENABLED"
            && !self
                .practice_run_configurations
                .contains_key(resource_identifier)
        {
            return Err(ArcZonalShiftError::AutoshiftRequiresPracticeRun);
        }
        let resource = self.ensure_managed_resource(resource_identifier);
        resource.zonal_autoshift_status = status.to_string();
        Ok(resource)
    }

    pub fn start_practice_run(
        &mut self,
        resource_identifier: String,
        away_from: String,
        comment: String,
    ) -> Result<ZonalShift, ArcZonalShiftError> {
        if !self
            .practice_run_configurations
            .contains_key(&resource_identifier)
        {
            return Err(ArcZonalShiftError::PracticeRunConfigurationNotFound {
                resource_identifier,
            });
        }
        // Default practice run window is 30 minutes.
        self.start_zonal_shift(
            resource_identifier,
            away_from,
            "30m".to_string(),
            comment,
            "PRACTICE",
        )
    }

    pub fn cancel_practice_run(&mut self, id: &str) -> Result<ZonalShift, ArcZonalShiftError> {
        let shift = self
            .zonal_shifts
            .get(id)
            .ok_or_else(|| ArcZonalShiftError::ZonalShiftNotFound { id: id.to_string() })?;
        if shift.shift_type != "PRACTICE" {
            return Err(ArcZonalShiftError::Validation {
                message: format!("Zonal shift {id} is not a practice run"),
            });
        }
        self.cancel_zonal_shift(id)
    }

    pub fn get_observer_notification_status(&self) -> &str {
        &self.autoshift_observer_notification_status
    }

    pub fn set_observer_notification_status(&mut self, status: &str) {
        self.autoshift_observer_notification_status = status.to_string();
    }
}

/// Parse an `expiresIn` duration string like `20h` or `120m` into seconds.
fn parse_duration(input: &str) -> Result<i64, ArcZonalShiftError> {
    if input.is_empty() {
        return Err(ArcZonalShiftError::Validation {
            message: "expiresIn must not be empty".to_string(),
        });
    }
    let (num_part, unit) = input.split_at(input.len() - 1);
    let value: i64 = num_part
        .parse()
        .map_err(|_| ArcZonalShiftError::Validation {
            message: format!("Invalid expiresIn value: {input}"),
        })?;
    let seconds = match unit {
        "m" | "M" => value * 60,
        "h" | "H" => value * 3600,
        _ => {
            return Err(ArcZonalShiftError::Validation {
                message: format!("Invalid expiresIn unit in {input}; expected 'm' or 'h'"),
            });
        }
    };
    if seconds <= 0 {
        return Err(ArcZonalShiftError::Validation {
            message: format!("expiresIn must be positive: {input}"),
        });
    }
    Ok(seconds)
}
