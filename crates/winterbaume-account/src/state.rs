use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct AccountState {
    pub alternate_contacts: HashMap<String, AlternateContact>,
    pub contact_information: Option<ContactInformation>,
    pub account_name: Option<String>,
    pub primary_email: Option<String>,
    /// Pending email update: (target_email, otp)
    pub pending_email_update: Option<(String, String)>,
    /// Per-region opt status overrides (only for opt-in regions that have been explicitly toggled)
    pub region_overrides: HashMap<String, RegionOptStatus>,
}

#[derive(Debug, Error)]
pub enum AccountError {
    #[error(
        "Invalid AlternateContactType: {contact_type}. Must be one of: BILLING, OPERATIONS, SECURITY"
    )]
    InvalidAlternateContactType { contact_type: String },

    #[error("Alternate contact of type {contact_type} not found")]
    AlternateContactNotFound { contact_type: String },

    #[error("Contact information not found")]
    ContactInformationNotFound,

    #[error("OTP or email mismatch")]
    OtpOrEmailMismatch,

    #[error("No pending email update found")]
    NoPendingEmailUpdate,

    #[error("Region {region_name} is enabled by default and cannot be opted in")]
    RegionEnabledByDefaultCannotOptIn { region_name: String },

    #[error("Region {region_name} is enabled by default and cannot be disabled")]
    RegionEnabledByDefaultCannotDisable { region_name: String },
}

impl AccountState {
    pub fn put_alternate_contact(
        &mut self,
        contact_type: &str,
        email_address: &str,
        name: &str,
        phone_number: &str,
        title: &str,
    ) -> Result<(), AccountError> {
        let valid_types = ["BILLING", "OPERATIONS", "SECURITY"];
        if !valid_types.contains(&contact_type) {
            return Err(AccountError::InvalidAlternateContactType {
                contact_type: contact_type.to_string(),
            });
        }

        let contact = AlternateContact {
            alternate_contact_type: contact_type.to_string(),
            email_address: email_address.to_string(),
            name: name.to_string(),
            phone_number: phone_number.to_string(),
            title: title.to_string(),
        };

        self.alternate_contacts
            .insert(contact_type.to_string(), contact);
        Ok(())
    }

    pub fn get_alternate_contact(
        &self,
        contact_type: &str,
    ) -> Result<&AlternateContact, AccountError> {
        self.alternate_contacts.get(contact_type).ok_or_else(|| {
            AccountError::AlternateContactNotFound {
                contact_type: contact_type.to_string(),
            }
        })
    }

    pub fn delete_alternate_contact(&mut self, contact_type: &str) -> Result<(), AccountError> {
        if self.alternate_contacts.remove(contact_type).is_none() {
            return Err(AccountError::AlternateContactNotFound {
                contact_type: contact_type.to_string(),
            });
        }
        Ok(())
    }

    pub fn put_contact_information(
        &mut self,
        info: ContactInformation,
    ) -> Result<(), AccountError> {
        self.contact_information = Some(info);
        Ok(())
    }

    pub fn get_contact_information(&self) -> Result<&ContactInformation, AccountError> {
        self.contact_information
            .as_ref()
            .ok_or(AccountError::ContactInformationNotFound)
    }

    pub fn put_account_name(&mut self, name: &str) -> Result<(), AccountError> {
        self.account_name = Some(name.to_string());
        Ok(())
    }

    pub fn get_account_information(&self, account_id: &str) -> (String, Option<String>) {
        // Return (account_id, account_name)
        (account_id.to_string(), self.account_name.clone())
    }

    pub fn get_primary_email(&self) -> Option<&str> {
        self.primary_email.as_deref()
    }

    /// Start a primary email update: store the pending email + OTP.
    pub fn start_primary_email_update(&mut self, new_email: &str) -> String {
        let otp = "123456".to_string(); // deterministic OTP for mock
        self.pending_email_update = Some((new_email.to_string(), otp.clone()));
        otp
    }

    /// Accept a primary email update using the OTP.
    pub fn accept_primary_email_update(
        &mut self,
        new_email: &str,
        otp: &str,
    ) -> Result<(), AccountError> {
        match &self.pending_email_update {
            Some((pending_email, pending_otp)) => {
                if pending_email != new_email || pending_otp != otp {
                    return Err(AccountError::OtpOrEmailMismatch);
                }
                let email = pending_email.clone();
                self.primary_email = Some(email);
                self.pending_email_update = None;
                Ok(())
            }
            None => Err(AccountError::NoPendingEmailUpdate),
        }
    }

    pub fn get_region_opt_status(&self, region_name: &str) -> RegionOptStatus {
        if let Some(override_status) = self.region_overrides.get(region_name) {
            return override_status.clone();
        }
        if DEFAULT_ENABLED_REGIONS.contains(&region_name) {
            RegionOptStatus::EnabledByDefault
        } else {
            RegionOptStatus::Disabled
        }
    }

    pub fn enable_region(&mut self, region_name: &str) -> Result<(), AccountError> {
        if DEFAULT_ENABLED_REGIONS.contains(&region_name) {
            return Err(AccountError::RegionEnabledByDefaultCannotOptIn {
                region_name: region_name.to_string(),
            });
        }
        self.region_overrides
            .insert(region_name.to_string(), RegionOptStatus::Enabled);
        Ok(())
    }

    pub fn disable_region(&mut self, region_name: &str) -> Result<(), AccountError> {
        if DEFAULT_ENABLED_REGIONS.contains(&region_name) {
            return Err(AccountError::RegionEnabledByDefaultCannotDisable {
                region_name: region_name.to_string(),
            });
        }
        self.region_overrides
            .insert(region_name.to_string(), RegionOptStatus::Disabled);
        Ok(())
    }

    pub fn list_regions(
        &self,
        filter: Option<&[String]>,
        max_results: Option<i32>,
        next_token: Option<&str>,
    ) -> (Vec<(String, RegionOptStatus)>, Option<String>) {
        let mut regions: Vec<(String, RegionOptStatus)> = ALL_REGIONS
            .iter()
            .map(|r| {
                let status = self.get_region_opt_status(r);
                (r.to_string(), status)
            })
            .collect();

        // Apply filter
        if let Some(statuses) = filter {
            regions.retain(|(_, status)| statuses.iter().any(|s| s == status.as_str()));
        }

        // Apply pagination
        let start = if let Some(token) = next_token {
            token.parse::<usize>().unwrap_or(0)
        } else {
            0
        };

        let page_size = max_results.unwrap_or(50).min(50) as usize;
        let end = (start + page_size).min(regions.len());
        let page = regions[start..end].to_vec();

        let new_next_token = if end < regions.len() {
            Some(end.to_string())
        } else {
            None
        };

        (page, new_next_token)
    }
}
