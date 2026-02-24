//! Serde-compatible view types for Account state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::AccountService;
use crate::state::AccountState;
use crate::types::{ContactInformation, RegionOptStatus};

/// Serializable view of the entire Account state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountStateView {
    /// Alternate contacts keyed by contact type.
    #[serde(default)]
    pub alternate_contacts: HashMap<String, AlternateContactView>,
    #[serde(default)]
    pub contact_information: Option<ContactInformationView>,
    #[serde(default)]
    pub account_name: Option<String>,
    #[serde(default)]
    pub primary_email: Option<String>,
    #[serde(default)]
    pub region_overrides: HashMap<String, String>,
}

/// Serializable view of an alternate contact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternateContactView {
    pub alternate_contact_type: String,
    pub name: String,
    pub email_address: String,
    pub phone_number: String,
    pub title: String,
}

/// Serializable view of contact information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInformationView {
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub address_line3: Option<String>,
    pub city: String,
    pub company_name: Option<String>,
    pub country_code: String,
    pub district_or_county: Option<String>,
    pub full_name: String,
    pub phone_number: String,
    pub postal_code: String,
    pub state_or_region: Option<String>,
    pub website_url: Option<String>,
}

impl From<&ContactInformation> for ContactInformationView {
    fn from(c: &ContactInformation) -> Self {
        Self {
            address_line1: c.address_line1.clone(),
            address_line2: c.address_line2.clone(),
            address_line3: c.address_line3.clone(),
            city: c.city.clone(),
            company_name: c.company_name.clone(),
            country_code: c.country_code.clone(),
            district_or_county: c.district_or_county.clone(),
            full_name: c.full_name.clone(),
            phone_number: c.phone_number.clone(),
            postal_code: c.postal_code.clone(),
            state_or_region: c.state_or_region.clone(),
            website_url: c.website_url.clone(),
        }
    }
}

impl From<ContactInformationView> for ContactInformation {
    fn from(v: ContactInformationView) -> Self {
        Self {
            address_line1: v.address_line1,
            address_line2: v.address_line2,
            address_line3: v.address_line3,
            city: v.city,
            company_name: v.company_name,
            country_code: v.country_code,
            district_or_county: v.district_or_county,
            full_name: v.full_name,
            phone_number: v.phone_number,
            postal_code: v.postal_code,
            state_or_region: v.state_or_region,
            website_url: v.website_url,
        }
    }
}

// --- From internal types to view types ---

impl From<&AccountState> for AccountStateView {
    fn from(state: &AccountState) -> Self {
        AccountStateView {
            alternate_contacts: state
                .alternate_contacts
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        AlternateContactView {
                            alternate_contact_type: v.alternate_contact_type.clone(),
                            name: v.name.clone(),
                            email_address: v.email_address.clone(),
                            phone_number: v.phone_number.clone(),
                            title: v.title.clone(),
                        },
                    )
                })
                .collect(),
            contact_information: state
                .contact_information
                .as_ref()
                .map(ContactInformationView::from),
            account_name: state.account_name.clone(),
            primary_email: state.primary_email.clone(),
            region_overrides: state
                .region_overrides
                .iter()
                .map(|(k, v)| (k.clone(), v.as_str().to_string()))
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<AccountStateView> for AccountState {
    fn from(view: AccountStateView) -> Self {
        AccountState {
            alternate_contacts: view
                .alternate_contacts
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        crate::types::AlternateContact {
                            alternate_contact_type: v.alternate_contact_type,
                            name: v.name,
                            email_address: v.email_address,
                            phone_number: v.phone_number,
                            title: v.title,
                        },
                    )
                })
                .collect(),
            contact_information: view.contact_information.map(ContactInformation::from),
            account_name: view.account_name,
            primary_email: view.primary_email,
            pending_email_update: None,
            region_overrides: view
                .region_overrides
                .into_iter()
                .map(|(k, v)| {
                    let status = match v.as_str() {
                        "ENABLED" => RegionOptStatus::Enabled,
                        "ENABLED_BY_DEFAULT" => RegionOptStatus::EnabledByDefault,
                        "DISABLED" => RegionOptStatus::Disabled,
                        "ENABLING" => RegionOptStatus::Enabling,
                        "DISABLING" => RegionOptStatus::Disabling,
                        _ => RegionOptStatus::Disabled,
                    };
                    (k, status)
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for AccountService {
    type StateView = AccountStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        AccountStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = AccountState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in view.alternate_contacts {
                guard.alternate_contacts.insert(
                    k,
                    crate::types::AlternateContact {
                        alternate_contact_type: v.alternate_contact_type,
                        name: v.name,
                        email_address: v.email_address,
                        phone_number: v.phone_number,
                        title: v.title,
                    },
                );
            }
            if let Some(ci) = view.contact_information {
                guard.contact_information = Some(ContactInformation::from(ci));
            }
            if let Some(name) = view.account_name {
                guard.account_name = Some(name);
            }
            if let Some(email) = view.primary_email {
                guard.primary_email = Some(email);
            }
            for (k, v) in view.region_overrides {
                let status = match v.as_str() {
                    "ENABLED" => RegionOptStatus::Enabled,
                    "ENABLED_BY_DEFAULT" => RegionOptStatus::EnabledByDefault,
                    "DISABLED" => RegionOptStatus::Disabled,
                    "ENABLING" => RegionOptStatus::Enabling,
                    "DISABLING" => RegionOptStatus::Disabling,
                    _ => RegionOptStatus::Disabled,
                };
                guard.region_overrides.insert(k, status);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
