use std::collections::HashMap;

use chrono::{Duration, Utc};
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct Route53DomainsState {
    pub domains: HashMap<String, DomainRegistration>,
}

#[derive(Debug, Error)]
pub enum Route53DomainsError {
    #[error("Domain {domain_name} is already registered.")]
    DuplicateRequest { domain_name: String },

    #[error("Domain {domain_name} not found.")]
    DomainNotFound { domain_name: String },
}

impl Route53DomainsState {
    pub fn register_domain(
        &mut self,
        domain_name: &str,
        duration_in_years: i32,
        auto_renew: bool,
        admin_contact: ContactDetail,
        registrant_contact: ContactDetail,
        tech_contact: ContactDetail,
        privacy_protect_admin: bool,
        privacy_protect_registrant: bool,
        privacy_protect_tech: bool,
    ) -> Result<String, Route53DomainsError> {
        if self.domains.contains_key(domain_name) {
            return Err(Route53DomainsError::DuplicateRequest {
                domain_name: domain_name.to_string(),
            });
        }

        let now = Utc::now();
        let expiration = now + Duration::days(365 * duration_in_years as i64);

        let registration = DomainRegistration {
            domain_name: domain_name.to_string(),
            auto_renew,
            admin_contact,
            registrant_contact,
            tech_contact,
            admin_privacy: privacy_protect_admin,
            registrant_privacy: privacy_protect_registrant,
            tech_privacy: privacy_protect_tech,
            creation_date: now,
            expiration_date: expiration,
            updated_date: now,
            transfer_lock: true,
            status_list: vec!["clientTransferProhibited".to_string()],
            nameservers: vec![
                "ns-1.awsdns-00.org".to_string(),
                "ns-2.awsdns-00.co.uk".to_string(),
            ],
        };

        let operation_id = uuid::Uuid::new_v4().to_string();
        self.domains.insert(domain_name.to_string(), registration);
        Ok(operation_id)
    }

    pub fn get_domain_detail(
        &self,
        domain_name: &str,
    ) -> Result<&DomainRegistration, Route53DomainsError> {
        self.domains
            .get(domain_name)
            .ok_or_else(|| Route53DomainsError::DomainNotFound {
                domain_name: domain_name.to_string(),
            })
    }

    pub fn list_domains(&self) -> Vec<&DomainRegistration> {
        self.domains.values().collect()
    }

    pub fn check_domain_availability(&self, domain_name: &str) -> String {
        if self.domains.contains_key(domain_name) {
            "UNAVAILABLE".to_string()
        } else {
            "AVAILABLE".to_string()
        }
    }

    pub fn delete_domain(&mut self, domain_name: &str) -> Result<String, Route53DomainsError> {
        if self.domains.remove(domain_name).is_none() {
            return Err(Route53DomainsError::DomainNotFound {
                domain_name: domain_name.to_string(),
            });
        }
        let operation_id = uuid::Uuid::new_v4().to_string();
        Ok(operation_id)
    }
}
