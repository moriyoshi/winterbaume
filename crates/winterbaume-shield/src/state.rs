use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

/// In-memory state for the Shield service.
#[derive(Debug, Default)]
pub struct ShieldState {
    /// The Shield Advanced subscription, if any.
    pub subscription: Option<Subscription>,
    /// Protections keyed by protection ID.
    pub protections: HashMap<String, Protection>,
    /// Tags keyed by resource ARN.
    pub tags: HashMap<String, Vec<Tag>>,
}

/// Error type for Shield operations.
#[derive(Debug, Error)]
pub enum ShieldError {
    #[error("Subscription already exists")]
    SubscriptionAlreadyExists,

    #[error("Subscription does not exist")]
    SubscriptionNotFound,

    #[error("A protection already exists for resource: {resource_arn}")]
    ProtectionAlreadyExists { resource_arn: String },

    #[error("Protection not found: '{id}'")]
    ProtectionNotFound { id: String },

    #[error("No protection found for resource: {arn}")]
    ProtectionNotFoundForResource { arn: String },

    #[error("Either ProtectionId or ResourceArn must be provided")]
    MissingProtectionIdentifier,
}

impl ShieldState {
    pub fn create_subscription(
        &mut self,
        account_id: &str,
        region: &str,
    ) -> Result<&Subscription, ShieldError> {
        if self.subscription.is_some() {
            return Err(ShieldError::SubscriptionAlreadyExists);
        }

        let subscription_arn = format!("arn:aws:shield:{region}:{account_id}:subscription");

        let sub = Subscription {
            start_time: Utc::now(),
            end_time: None,
            time_commitment_in_seconds: 31536000, // 1 year
            auto_renew: AutoRenew::Enabled,
            subscription_arn,
        };

        self.subscription = Some(sub);
        Ok(self.subscription.as_ref().unwrap())
    }

    pub fn describe_subscription(&self) -> Result<&Subscription, ShieldError> {
        self.subscription
            .as_ref()
            .ok_or(ShieldError::SubscriptionNotFound)
    }

    pub fn create_protection(
        &mut self,
        name: &str,
        resource_arn: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&Protection, ShieldError> {
        // Check if a protection already exists for this resource
        if self
            .protections
            .values()
            .any(|p| p.resource_arn == resource_arn)
        {
            return Err(ShieldError::ProtectionAlreadyExists {
                resource_arn: resource_arn.to_string(),
            });
        }

        let protection_id = uuid::Uuid::new_v4().to_string();
        let protection_arn =
            format!("arn:aws:shield:{region}:{account_id}:protection/{protection_id}");

        let protection = Protection {
            id: protection_id.clone(),
            name: name.to_string(),
            resource_arn: resource_arn.to_string(),
            protection_arn,
            health_check_ids: Vec::new(),
            tags: Vec::new(),
        };

        self.protections.insert(protection_id.clone(), protection);
        Ok(self.protections.get(&protection_id).unwrap())
    }

    pub fn describe_protection(
        &self,
        protection_id: Option<&str>,
        resource_arn: Option<&str>,
    ) -> Result<&Protection, ShieldError> {
        if let Some(id) = protection_id {
            return self
                .protections
                .get(id)
                .ok_or_else(|| protection_not_found(id));
        }

        if let Some(arn) = resource_arn {
            return self
                .protections
                .values()
                .find(|p| p.resource_arn == arn)
                .ok_or_else(|| ShieldError::ProtectionNotFoundForResource {
                    arn: arn.to_string(),
                });
        }

        Err(ShieldError::MissingProtectionIdentifier)
    }

    pub fn delete_protection(&mut self, protection_id: &str) -> Result<(), ShieldError> {
        self.protections
            .remove(protection_id)
            .ok_or_else(|| protection_not_found(protection_id))?;
        Ok(())
    }

    pub fn list_protections(&self) -> Vec<&Protection> {
        self.protections.values().collect()
    }
}

fn protection_not_found(id: &str) -> ShieldError {
    ShieldError::ProtectionNotFound { id: id.to_string() }
}
