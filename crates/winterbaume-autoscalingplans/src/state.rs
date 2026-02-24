use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct AutoScalingPlansState {
    /// ScalingPlans keyed by `(name, version)`.
    pub plans: HashMap<(String, i64), ScalingPlan>,
}

#[derive(Debug, Error)]
pub enum AutoScalingPlansError {
    #[error("Scaling plan {name} version {version} not found")]
    PlanNotFound { name: String, version: i64 },

    #[error("Scaling plan {name} version {version} already exists")]
    PlanAlreadyExists { name: String, version: i64 },

    #[error("{message}")]
    Validation { message: String },
}

impl AutoScalingPlansState {
    pub fn create_plan(&mut self, plan: ScalingPlan) -> Result<i64, AutoScalingPlansError> {
        let key = (plan.scaling_plan_name.clone(), plan.scaling_plan_version);
        if self.plans.contains_key(&key) {
            return Err(AutoScalingPlansError::PlanAlreadyExists {
                name: key.0.clone(),
                version: key.1,
            });
        }
        let version = plan.scaling_plan_version;
        self.plans.insert(key, plan);
        Ok(version)
    }

    pub fn get_plan(
        &self,
        name: &str,
        version: i64,
    ) -> Result<&ScalingPlan, AutoScalingPlansError> {
        self.plans.get(&(name.to_string(), version)).ok_or_else(|| {
            AutoScalingPlansError::PlanNotFound {
                name: name.to_string(),
                version,
            }
        })
    }

    pub fn update_plan(
        &mut self,
        name: &str,
        version: i64,
        application_source: Option<serde_json::Value>,
        scaling_instructions: Option<Vec<serde_json::Value>>,
    ) -> Result<(), AutoScalingPlansError> {
        let plan = self
            .plans
            .get_mut(&(name.to_string(), version))
            .ok_or_else(|| AutoScalingPlansError::PlanNotFound {
                name: name.to_string(),
                version,
            })?;
        if let Some(s) = application_source {
            plan.application_source = s;
        }
        if let Some(s) = scaling_instructions {
            plan.scaling_instructions = s;
        }
        plan.status_start_time = chrono::Utc::now().timestamp();
        Ok(())
    }

    pub fn delete_plan(&mut self, name: &str, version: i64) -> Result<(), AutoScalingPlansError> {
        self.plans.remove(&(name.to_string(), version)).ok_or(
            AutoScalingPlansError::PlanNotFound {
                name: name.to_string(),
                version,
            },
        )?;
        Ok(())
    }

    /// List plans, optionally filtering by name list.
    pub fn list_plans(&self, names: Option<&[String]>) -> Vec<&ScalingPlan> {
        let mut all: Vec<&ScalingPlan> = self
            .plans
            .values()
            .filter(|p| match names {
                Some(filter) => filter.iter().any(|n| n == &p.scaling_plan_name),
                None => true,
            })
            .collect();
        all.sort_by(|a, b| {
            (a.scaling_plan_name.as_str(), a.scaling_plan_version)
                .cmp(&(b.scaling_plan_name.as_str(), b.scaling_plan_version))
        });
        all
    }
}
