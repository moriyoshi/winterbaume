use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct BcmDashboardsState {
    pub dashboards: HashMap<String, Dashboard>,
}

#[derive(Debug, Error)]
pub enum BcmDashboardsError {
    #[error("Dashboard {arn} does not exist.")]
    NotFound { arn: String },

    #[error("Dashboard with name {name} already exists.")]
    AlreadyExists { name: String },

    #[error("{message}")]
    Validation { message: String },
}

impl BcmDashboardsState {
    pub fn create_dashboard(
        &mut self,
        dashboard: Dashboard,
    ) -> Result<&Dashboard, BcmDashboardsError> {
        if self.dashboards.values().any(|d| d.name == dashboard.name) {
            return Err(BcmDashboardsError::AlreadyExists {
                name: dashboard.name.clone(),
            });
        }
        let arn = dashboard.arn.clone();
        self.dashboards.insert(arn.clone(), dashboard);
        Ok(self.dashboards.get(&arn).unwrap())
    }

    pub fn get_dashboard(&self, arn: &str) -> Result<&Dashboard, BcmDashboardsError> {
        self.dashboards
            .get(arn)
            .ok_or_else(|| BcmDashboardsError::NotFound {
                arn: arn.to_string(),
            })
    }

    pub fn update_dashboard(
        &mut self,
        arn: &str,
        update: impl FnOnce(&mut Dashboard),
    ) -> Result<&Dashboard, BcmDashboardsError> {
        let d = self
            .dashboards
            .get_mut(arn)
            .ok_or_else(|| BcmDashboardsError::NotFound {
                arn: arn.to_string(),
            })?;
        update(d);
        d.updated_at = chrono::Utc::now().timestamp();
        Ok(d)
    }

    pub fn delete_dashboard(&mut self, arn: &str) -> Result<(), BcmDashboardsError> {
        self.dashboards
            .remove(arn)
            .ok_or_else(|| BcmDashboardsError::NotFound {
                arn: arn.to_string(),
            })?;
        Ok(())
    }

    pub fn list_dashboards(&self) -> Vec<&Dashboard> {
        let mut items: Vec<&Dashboard> = self.dashboards.values().collect();
        items.sort_by(|a, b| a.name.cmp(&b.name));
        items
    }

    pub fn tag_resource(
        &mut self,
        arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), BcmDashboardsError> {
        let d = self
            .dashboards
            .get_mut(arn)
            .ok_or_else(|| BcmDashboardsError::NotFound {
                arn: arn.to_string(),
            })?;
        for (k, v) in tags {
            d.tags.insert(k, v);
        }
        Ok(())
    }

    pub fn untag_resource(&mut self, arn: &str, keys: &[String]) -> Result<(), BcmDashboardsError> {
        let d = self
            .dashboards
            .get_mut(arn)
            .ok_or_else(|| BcmDashboardsError::NotFound {
                arn: arn.to_string(),
            })?;
        for k in keys {
            d.tags.remove(k);
        }
        Ok(())
    }

    pub fn list_tags(&self, arn: &str) -> Result<HashMap<String, String>, BcmDashboardsError> {
        Ok(self.get_dashboard(arn)?.tags.clone())
    }
}
