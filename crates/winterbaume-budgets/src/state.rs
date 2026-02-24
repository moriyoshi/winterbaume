use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct BudgetsState {
    pub budgets: HashMap<String, Budget>,
}

#[derive(Debug, Error)]
pub enum BudgetsError {
    #[error("Budget {name} already exists.")]
    DuplicateBudget { name: String },

    #[error("Budget {name} not found.")]
    BudgetNotFound { name: String },

    #[error(
        "Error: Duplicate notification - a notification with the same type, \
         comparison operator, and threshold already exists for budget {budget_name}."
    )]
    DuplicateNotification { budget_name: String },

    #[error("Notification not found.")]
    NotificationNotFound,
}

impl BudgetsState {
    pub fn create_budget(
        &mut self,
        name: &str,
        budget_type: &str,
        limit_amount: &str,
        limit_unit: &str,
        time_unit: &str,
    ) -> Result<(), BudgetsError> {
        if self.budgets.contains_key(name) {
            return Err(BudgetsError::DuplicateBudget {
                name: name.to_string(),
            });
        }

        let budget = Budget {
            budget_name: name.to_string(),
            budget_type: budget_type.to_string(),
            budget_limit_amount: limit_amount.to_string(),
            budget_limit_unit: limit_unit.to_string(),
            time_unit: time_unit.to_string(),
            notifications: Vec::new(),
        };

        self.budgets.insert(name.to_string(), budget);
        Ok(())
    }

    pub fn describe_budget(&self, name: &str) -> Result<&Budget, BudgetsError> {
        self.budgets
            .get(name)
            .ok_or_else(|| BudgetsError::BudgetNotFound {
                name: name.to_string(),
            })
    }

    pub fn describe_budgets(&self) -> Vec<&Budget> {
        self.budgets.values().collect()
    }

    pub fn delete_budget(&mut self, name: &str) -> Result<(), BudgetsError> {
        if self.budgets.remove(name).is_none() {
            return Err(BudgetsError::BudgetNotFound {
                name: name.to_string(),
            });
        }
        Ok(())
    }
}
