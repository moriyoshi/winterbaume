use std::collections::HashMap;

use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct TaxSettingsState {
    /// Primary tax registration per account, keyed by account_id.
    pub registrations: HashMap<String, Value>,
    /// Supplemental tax registrations per (account_id, authority_id).
    pub supplemental: HashMap<(String, String), Value>,
    /// Tax exemption details per account_id.
    pub exemptions: HashMap<String, Value>,
    /// Tax inheritance setting (single global flag).
    pub inheritance_state: String,
}

#[derive(Debug, Error)]
pub enum TaxSettingsError {
    #[error("Tax registration for account {account_id} does not exist.")]
    RegistrationNotFound { account_id: String },

    #[error("Supplemental tax registration {authority_id} for account {account_id} not found.")]
    SupplementalNotFound {
        account_id: String,
        authority_id: String,
    },

    #[error("{message}")]
    Validation { message: String },
}

impl TaxSettingsState {
    pub fn put_registration(&mut self, account_id: &str, value: Value) {
        self.registrations.insert(account_id.to_string(), value);
    }

    pub fn get_registration(&self, account_id: &str) -> Result<&Value, TaxSettingsError> {
        self.registrations
            .get(account_id)
            .ok_or(TaxSettingsError::RegistrationNotFound {
                account_id: account_id.to_string(),
            })
    }

    pub fn delete_registration(&mut self, account_id: &str) -> Result<(), TaxSettingsError> {
        self.registrations
            .remove(account_id)
            .ok_or(TaxSettingsError::RegistrationNotFound {
                account_id: account_id.to_string(),
            })?;
        Ok(())
    }

    pub fn list_registrations(&self) -> Vec<(&String, &Value)> {
        let mut v: Vec<(&String, &Value)> = self.registrations.iter().collect();
        v.sort_by(|a, b| a.0.cmp(b.0));
        v
    }

    pub fn put_supplemental(&mut self, account_id: &str, authority_id: &str, value: Value) {
        self.supplemental
            .insert((account_id.to_string(), authority_id.to_string()), value);
    }

    pub fn delete_supplemental(
        &mut self,
        account_id: &str,
        authority_id: &str,
    ) -> Result<(), TaxSettingsError> {
        self.supplemental
            .remove(&(account_id.to_string(), authority_id.to_string()))
            .ok_or(TaxSettingsError::SupplementalNotFound {
                account_id: account_id.to_string(),
                authority_id: authority_id.to_string(),
            })?;
        Ok(())
    }

    pub fn list_supplemental(&self, account_id: &str) -> Vec<&Value> {
        let mut v: Vec<&Value> = self
            .supplemental
            .iter()
            .filter(|((a, _), _)| a == account_id)
            .map(|(_, v)| v)
            .collect();
        // Sort by authorityId for stable output.
        v.sort_by(|a, b| {
            a.get("authorityId")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .cmp(b.get("authorityId").and_then(|v| v.as_str()).unwrap_or(""))
        });
        v
    }

    pub fn put_exemption(&mut self, account_id: &str, value: Value) {
        self.exemptions.insert(account_id.to_string(), value);
    }

    pub fn get_exemption(&self, account_id: &str) -> Option<&Value> {
        self.exemptions.get(account_id)
    }

    pub fn list_exemptions(&self) -> Vec<(&String, &Value)> {
        let mut v: Vec<(&String, &Value)> = self.exemptions.iter().collect();
        v.sort_by(|a, b| a.0.cmp(b.0));
        v
    }

    pub fn set_inheritance(&mut self, state: String) {
        self.inheritance_state = state;
    }
}
