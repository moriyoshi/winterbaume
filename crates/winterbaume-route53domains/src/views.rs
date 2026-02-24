//! Serde-compatible view types for Route53Domains state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::Route53DomainsService;
use crate::state::Route53DomainsState;
use crate::types::{ContactDetail, DomainRegistration};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Route53DomainsStateView {
    #[serde(default)]
    pub domains: HashMap<String, DomainRegistrationView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainRegistrationView {
    pub domain_name: String,
    pub auto_renew: bool,
    pub admin_contact: ContactDetailView,
    pub registrant_contact: ContactDetailView,
    pub tech_contact: ContactDetailView,
    pub admin_privacy: bool,
    pub registrant_privacy: bool,
    pub tech_privacy: bool,
    pub creation_date: DateTime<Utc>,
    pub expiration_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
    pub transfer_lock: bool,
    #[serde(default)]
    pub status_list: Vec<String>,
    #[serde(default)]
    pub nameservers: Vec<String>,
    /// `name_server` nested blocks (structured nameserver records with glue IPs).
    #[serde(default)]
    pub name_server: Vec<NameserverRecord>,
    /// `billing_contact` nested block.
    #[serde(default)]
    pub billing_contact: Option<ContactDetailView>,
    /// `billing_privacy` flag.
    #[serde(default)]
    pub billing_privacy: bool,
}

/// A structured nameserver record with optional glue IPs.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NameserverRecord {
    pub name: String,
    #[serde(default)]
    pub glue_ips: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContactDetailView {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub organization_name: Option<String>,
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country_code: Option<String>,
    pub zip_code: Option<String>,
    pub contact_type: Option<String>,
}

impl From<&ContactDetail> for ContactDetailView {
    fn from(c: &ContactDetail) -> Self {
        ContactDetailView {
            first_name: c.first_name.clone(),
            last_name: c.last_name.clone(),
            email: c.email.clone(),
            phone_number: c.phone_number.clone(),
            organization_name: c.organization_name.clone(),
            address_line_1: c.address_line_1.clone(),
            address_line_2: c.address_line_2.clone(),
            city: c.city.clone(),
            state: c.state.clone(),
            country_code: c.country_code.clone(),
            zip_code: c.zip_code.clone(),
            contact_type: c.contact_type.clone(),
        }
    }
}

impl From<ContactDetailView> for ContactDetail {
    fn from(v: ContactDetailView) -> Self {
        ContactDetail {
            first_name: v.first_name,
            last_name: v.last_name,
            email: v.email,
            phone_number: v.phone_number,
            organization_name: v.organization_name,
            address_line_1: v.address_line_1,
            address_line_2: v.address_line_2,
            city: v.city,
            state: v.state,
            country_code: v.country_code,
            zip_code: v.zip_code,
            contact_type: v.contact_type,
        }
    }
}

impl From<&Route53DomainsState> for Route53DomainsStateView {
    fn from(state: &Route53DomainsState) -> Self {
        Route53DomainsStateView {
            domains: state
                .domains
                .iter()
                .map(|(k, v)| (k.clone(), DomainRegistrationView::from(v)))
                .collect(),
        }
    }
}

impl From<&DomainRegistration> for DomainRegistrationView {
    fn from(d: &DomainRegistration) -> Self {
        DomainRegistrationView {
            domain_name: d.domain_name.clone(),
            auto_renew: d.auto_renew,
            admin_contact: ContactDetailView::from(&d.admin_contact),
            registrant_contact: ContactDetailView::from(&d.registrant_contact),
            tech_contact: ContactDetailView::from(&d.tech_contact),
            admin_privacy: d.admin_privacy,
            registrant_privacy: d.registrant_privacy,
            tech_privacy: d.tech_privacy,
            creation_date: d.creation_date,
            expiration_date: d.expiration_date,
            updated_date: d.updated_date,
            transfer_lock: d.transfer_lock,
            status_list: d.status_list.clone(),
            nameservers: d.nameservers.clone(),
            name_server: vec![],
            billing_contact: None,
            billing_privacy: false,
        }
    }
}

impl From<Route53DomainsStateView> for Route53DomainsState {
    fn from(view: Route53DomainsStateView) -> Self {
        Route53DomainsState {
            domains: view
                .domains
                .into_iter()
                .map(|(k, v)| (k, DomainRegistration::from(v)))
                .collect(),
        }
    }
}

impl From<DomainRegistrationView> for DomainRegistration {
    fn from(v: DomainRegistrationView) -> Self {
        DomainRegistration {
            domain_name: v.domain_name,
            auto_renew: v.auto_renew,
            admin_contact: ContactDetail::from(v.admin_contact),
            registrant_contact: ContactDetail::from(v.registrant_contact),
            tech_contact: ContactDetail::from(v.tech_contact),
            admin_privacy: v.admin_privacy,
            registrant_privacy: v.registrant_privacy,
            tech_privacy: v.tech_privacy,
            creation_date: v.creation_date,
            expiration_date: v.expiration_date,
            updated_date: v.updated_date,
            transfer_lock: v.transfer_lock,
            status_list: v.status_list,
            nameservers: v.nameservers,
        }
    }
}

impl StatefulService for Route53DomainsService {
    type StateView = Route53DomainsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        Route53DomainsStateView::from(&*guard)
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
            *guard = Route53DomainsState::from(view);
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
            for (k, v) in view.domains {
                guard.domains.insert(k, DomainRegistration::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
