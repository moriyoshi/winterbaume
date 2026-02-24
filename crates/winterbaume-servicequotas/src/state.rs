use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct ServiceQuotasState {
    /// Key: "{service_code}:{quota_code}"
    pub quotas: HashMap<String, ServiceQuotaEntry>,
    /// Key: service_code
    pub services: HashMap<String, ServiceEntry>,
}

#[derive(Debug, Error)]
pub enum ServiceQuotasError {
    #[error("No quota found for service code {service_code} and quota code {quota_code}.")]
    NoSuchResource {
        service_code: String,
        quota_code: String,
    },
}

impl ServiceQuotasState {
    pub fn add_service(&mut self, service_code: &str, service_name: &str) {
        self.services.insert(
            service_code.to_string(),
            ServiceEntry {
                service_code: service_code.to_string(),
                service_name: service_name.to_string(),
            },
        );
    }

    pub fn add_quota(
        &mut self,
        service_code: &str,
        service_name: &str,
        quota_code: &str,
        quota_name: &str,
        value: f64,
        unit: &str,
        adjustable: bool,
        global_quota: bool,
        description: &str,
        region: &str,
        account_id: &str,
    ) {
        let key = format!("{service_code}:{quota_code}");
        let quota_arn =
            format!("arn:aws:servicequotas:{region}:{account_id}:{service_code}/{quota_code}");

        // Ensure service is registered
        if !self.services.contains_key(service_code) {
            self.add_service(service_code, service_name);
        }

        self.quotas.insert(
            key,
            ServiceQuotaEntry {
                service_code: service_code.to_string(),
                service_name: service_name.to_string(),
                quota_code: quota_code.to_string(),
                quota_name: quota_name.to_string(),
                quota_arn,
                value,
                unit: unit.to_string(),
                adjustable,
                global_quota,
                description: description.to_string(),
            },
        );
    }

    pub fn get_quota(
        &self,
        service_code: &str,
        quota_code: &str,
    ) -> Result<&ServiceQuotaEntry, ServiceQuotasError> {
        let key = format!("{service_code}:{quota_code}");
        self.quotas
            .get(&key)
            .ok_or_else(|| ServiceQuotasError::NoSuchResource {
                service_code: service_code.to_string(),
                quota_code: quota_code.to_string(),
            })
    }

    pub fn list_quotas(&self, service_code: &str) -> Vec<&ServiceQuotaEntry> {
        self.quotas
            .values()
            .filter(|q| q.service_code == service_code)
            .collect()
    }

    pub fn list_services(&self) -> Vec<&ServiceEntry> {
        self.services.values().collect()
    }
}
