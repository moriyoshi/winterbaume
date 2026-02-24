use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Default)]
pub struct SavingsPlansState {
    pub plans: HashMap<String, PlanRecord>,
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone)]
pub struct PlanRecord {
    pub savings_plan_id: String,
    pub savings_plan_arn: String,
    pub offering_id: String,
    pub commitment: String,
    pub upfront_payment_amount: Option<String>,
    pub recurring_payment_amount: Option<String>,
    pub start: String,
    pub end: String,
    pub state: String,
    pub region: String,
    pub currency: String,
    pub savings_plan_type: String,
    pub payment_option: String,
    pub product_types: Vec<String>,
    pub term_duration_in_seconds: i64,
    pub returnable_until: Option<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Error)]
pub enum SavingsPlansError {
    #[error("Savings plan {id} does not exist.")]
    NotFound { id: String },

    #[error("Savings plan {id} is in state {state} and cannot be {action}.")]
    InvalidState {
        id: String,
        state: String,
        action: String,
    },

    #[error("{message}")]
    Validation { message: String },
}

impl SavingsPlansState {
    pub fn create_plan(&mut self, plan: PlanRecord) -> &PlanRecord {
        let id = plan.savings_plan_id.clone();
        self.plans.insert(id.clone(), plan);
        self.plans.get(&id).unwrap()
    }

    pub fn get_plan(&self, id: &str) -> Result<&PlanRecord, SavingsPlansError> {
        self.plans
            .get(id)
            .ok_or(SavingsPlansError::NotFound { id: id.to_string() })
    }

    pub fn delete_queued(&mut self, id: &str) -> Result<(), SavingsPlansError> {
        let plan = self
            .plans
            .get(id)
            .ok_or(SavingsPlansError::NotFound { id: id.to_string() })?;
        if plan.state != "queued" {
            return Err(SavingsPlansError::InvalidState {
                id: id.to_string(),
                state: plan.state.clone(),
                action: "deleted".to_string(),
            });
        }
        self.plans.remove(id);
        Ok(())
    }

    pub fn return_plan(&mut self, id: &str) -> Result<&PlanRecord, SavingsPlansError> {
        let plan = self
            .plans
            .get_mut(id)
            .ok_or(SavingsPlansError::NotFound { id: id.to_string() })?;
        if plan.state != "active" && plan.state != "payment-pending" {
            return Err(SavingsPlansError::InvalidState {
                id: id.to_string(),
                state: plan.state.clone(),
                action: "returned".to_string(),
            });
        }
        plan.state = "returned".to_string();
        Ok(plan)
    }

    pub fn list_plans<'a>(
        &'a self,
        ids: Option<&'a [String]>,
        arns: Option<&'a [String]>,
        states: Option<&'a [String]>,
    ) -> Vec<&'a PlanRecord> {
        let mut items: Vec<&PlanRecord> = self
            .plans
            .values()
            .filter(|p| ids.is_none_or(|v| v.iter().any(|x| x == &p.savings_plan_id)))
            .filter(|p| arns.is_none_or(|v| v.iter().any(|x| x == &p.savings_plan_arn)))
            .filter(|p| states.is_none_or(|v| v.iter().any(|x| x == &p.state)))
            .collect();
        items.sort_by(|a, b| a.savings_plan_id.cmp(&b.savings_plan_id));
        items
    }

    pub fn tag_resource(&mut self, arn: &str, tags: HashMap<String, String>) {
        let entry = self.tags.entry(arn.to_string()).or_default();
        for (k, v) in tags {
            entry.insert(k, v);
        }
        if let Some(plan) = self.plans.values_mut().find(|p| p.savings_plan_arn == arn) {
            plan.tags = self.tags.get(arn).cloned().unwrap_or_default();
        }
    }

    pub fn untag_resource(&mut self, arn: &str, keys: &[String]) {
        if let Some(entry) = self.tags.get_mut(arn) {
            for k in keys {
                entry.remove(k);
            }
        }
        let updated = self.tags.get(arn).cloned().unwrap_or_default();
        if let Some(plan) = self.plans.values_mut().find(|p| p.savings_plan_arn == arn) {
            plan.tags = updated;
        }
    }

    pub fn list_tags(&self, arn: &str) -> HashMap<String, String> {
        self.tags.get(arn).cloned().unwrap_or_default()
    }
}
