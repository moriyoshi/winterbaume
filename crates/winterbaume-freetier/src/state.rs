use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug)]
pub struct FreeTierState {
    pub plan_type: String,
    pub plan_status: String,
    pub plan_expiration_date: Option<String>,
    pub remaining_credits_amount: f64,
    pub remaining_credits_unit: String,
    pub activities: HashMap<String, ActivityRecord>,
}

impl Default for FreeTierState {
    fn default() -> Self {
        Self {
            plan_type: "FREE_TIER".to_string(),
            plan_status: "ACTIVE".to_string(),
            plan_expiration_date: None,
            remaining_credits_amount: 100.0,
            remaining_credits_unit: "USD".to_string(),
            activities: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActivityRecord {
    pub activity_id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub expires_at: Option<String>,
    pub estimated_time_to_complete_in_minutes: Option<i32>,
    pub instructions_url: Option<String>,
    pub reward_amount: f64,
    pub reward_unit: String,
}

#[derive(Debug, Error)]
pub enum FreeTierError {
    #[error("Activity {id} not found.")]
    ActivityNotFound { id: String },

    #[error("{message}")]
    Validation { message: String },
}

impl FreeTierState {
    pub fn upsert_activity(&mut self, record: ActivityRecord) {
        self.activities.insert(record.activity_id.clone(), record);
    }

    pub fn get_activity(&self, id: &str) -> Result<&ActivityRecord, FreeTierError> {
        self.activities
            .get(id)
            .ok_or(FreeTierError::ActivityNotFound { id: id.to_string() })
    }

    pub fn list_activities(&self, statuses: Option<&[String]>) -> Vec<&ActivityRecord> {
        let mut items: Vec<&ActivityRecord> = self
            .activities
            .values()
            .filter(|a| statuses.is_none_or(|v| v.iter().any(|s| s == &a.status)))
            .collect();
        items.sort_by(|a, b| a.activity_id.cmp(&b.activity_id));
        items
    }

    pub fn upgrade_plan(&mut self, plan_type: String) {
        self.plan_type = plan_type;
        self.plan_status = "ACTIVE".to_string();
    }
}
