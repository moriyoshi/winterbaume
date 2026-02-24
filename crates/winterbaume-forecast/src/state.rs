use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct ForecastState {
    pub dataset_groups: HashMap<String, DatasetGroup>,
}

#[derive(Debug, Error)]
pub enum ForecastError {
    #[error("A dataset group already exists with the ARN: {arn}")]
    ResourceAlreadyExists { arn: String },

    #[error("No resource found with ARN: {arn}")]
    ResourceNotFound { arn: String },
}

impl ForecastState {
    pub fn create_dataset_group(
        &mut self,
        dataset_group_name: &str,
        domain: &str,
        dataset_arns: Option<Vec<String>>,
        account_id: &str,
        region: &str,
    ) -> Result<String, ForecastError> {
        let arn =
            format!("arn:aws:forecast:{region}:{account_id}:dataset-group/{dataset_group_name}");

        if self.dataset_groups.contains_key(&arn) {
            return Err(ForecastError::ResourceAlreadyExists { arn });
        }

        let now = Utc::now();
        let dataset_group = DatasetGroup {
            dataset_group_name: dataset_group_name.to_string(),
            dataset_group_arn: arn.clone(),
            domain: domain.to_string(),
            dataset_arns: dataset_arns.unwrap_or_default(),
            status: "ACTIVE".to_string(),
            creation_time: now,
            last_modification_time: now,
        };

        self.dataset_groups.insert(arn.clone(), dataset_group);
        Ok(arn)
    }

    pub fn describe_dataset_group(
        &self,
        dataset_group_arn: &str,
    ) -> Result<&DatasetGroup, ForecastError> {
        self.dataset_groups
            .get(dataset_group_arn)
            .ok_or_else(|| ForecastError::ResourceNotFound {
                arn: dataset_group_arn.to_string(),
            })
    }

    pub fn delete_dataset_group(&mut self, dataset_group_arn: &str) -> Result<(), ForecastError> {
        if self.dataset_groups.remove(dataset_group_arn).is_none() {
            return Err(ForecastError::ResourceNotFound {
                arn: dataset_group_arn.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_dataset_groups(&self) -> Vec<&DatasetGroup> {
        self.dataset_groups.values().collect()
    }

    pub fn update_dataset_group(
        &mut self,
        dataset_group_arn: &str,
        dataset_arns: Vec<String>,
    ) -> Result<(), ForecastError> {
        let dg = self
            .dataset_groups
            .get_mut(dataset_group_arn)
            .ok_or_else(|| ForecastError::ResourceNotFound {
                arn: dataset_group_arn.to_string(),
            })?;
        dg.dataset_arns = dataset_arns;
        dg.last_modification_time = Utc::now();
        Ok(())
    }
}
