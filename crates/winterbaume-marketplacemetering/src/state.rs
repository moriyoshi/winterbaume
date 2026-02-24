use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

/// In-memory state for the AWS Marketplace Metering service.
#[derive(Debug, Default)]
pub struct MeteringMarketplaceState {
    /// Metering records keyed by metering_record_id.
    pub metering_records: Vec<MeteringRecord>,
    /// Batch usage record results.
    pub batch_results: Vec<UsageRecordResult>,
    /// Registered products keyed by product_code.
    pub registrations: HashMap<String, Registration>,
    /// Registration tokens mapped to resolved customers.
    pub resolved_customers: HashMap<String, ResolvedCustomer>,
    /// Pre-configured registration tokens for ResolveCustomer.
    pub registration_tokens: HashMap<String, ResolvedCustomer>,
}

/// Error type for metering marketplace operations.
#[derive(Debug, Error)]
pub enum MeteringError {
    #[error("Product code is invalid or empty.")]
    InvalidProductCode,

    #[error("Usage dimension is invalid or empty.")]
    InvalidUsageDimension,

    #[error("Public key version is invalid.")]
    InvalidPublicKeyVersion,

    #[error("UsageRecords must not be empty.")]
    EmptyUsageRecords,

    #[error("Registration token is invalid or empty.")]
    InvalidToken,
}

impl MeteringMarketplaceState {
    /// Record usage via MeterUsage.
    pub fn meter_usage(
        &mut self,
        product_code: &str,
        timestamp: chrono::DateTime<chrono::Utc>,
        usage_dimension: &str,
        usage_quantity: Option<i32>,
        dry_run: bool,
        usage_allocations: Vec<UsageAllocation>,
    ) -> Result<String, MeteringError> {
        if product_code.is_empty() {
            return Err(MeteringError::InvalidProductCode);
        }

        if usage_dimension.is_empty() {
            return Err(MeteringError::InvalidUsageDimension);
        }

        let quantity = usage_quantity.unwrap_or(0);
        let metering_record_id = uuid::Uuid::new_v4().to_string();

        if !dry_run {
            self.metering_records.push(MeteringRecord {
                product_code: product_code.to_string(),
                timestamp,
                usage_dimension: usage_dimension.to_string(),
                usage_quantity: quantity,
                dry_run,
                metering_record_id: metering_record_id.clone(),
                usage_allocations,
            });
        }

        Ok(metering_record_id)
    }

    /// Register usage via RegisterUsage.
    pub fn register_usage(
        &mut self,
        product_code: &str,
        public_key_version: i32,
    ) -> Result<&Registration, MeteringError> {
        if product_code.is_empty() {
            return Err(MeteringError::InvalidProductCode);
        }

        if public_key_version < 1 {
            return Err(MeteringError::InvalidPublicKeyVersion);
        }

        let registration = Registration {
            product_code: product_code.to_string(),
            public_key_rotation_timestamp: Utc::now(),
            signature: format!("mock-signature-{}", uuid::Uuid::new_v4()),
        };

        self.registrations
            .insert(product_code.to_string(), registration);
        Ok(self.registrations.get(product_code).unwrap())
    }

    /// Process batch meter usage records.
    pub fn batch_meter_usage(
        &mut self,
        product_code: &str,
        usage_records: Vec<UsageRecord>,
    ) -> Result<Vec<UsageRecordResult>, MeteringError> {
        if product_code.is_empty() {
            return Err(MeteringError::InvalidProductCode);
        }

        if usage_records.is_empty() {
            return Err(MeteringError::EmptyUsageRecords);
        }

        let mut results = Vec::new();

        for record in usage_records {
            let metering_record_id = uuid::Uuid::new_v4().to_string();
            let result = UsageRecordResult {
                usage_record: record,
                metering_record_id,
                status: UsageRecordResultStatus::Success,
            };
            results.push(result);
        }

        self.batch_results.extend(results.clone());
        Ok(results)
    }

    /// Resolve a registration token to a customer.
    pub fn resolve_customer(
        &mut self,
        registration_token: &str,
    ) -> Result<ResolvedCustomer, MeteringError> {
        if registration_token.is_empty() {
            return Err(MeteringError::InvalidToken);
        }

        // Check pre-configured tokens first
        if let Some(customer) = self.registration_tokens.get(registration_token) {
            let resolved = customer.clone();
            self.resolved_customers
                .insert(registration_token.to_string(), resolved.clone());
            return Ok(resolved);
        }

        // Generate a mock resolution for any token
        let customer = ResolvedCustomer {
            customer_identifier: format!(
                "customer-{}",
                &registration_token[..8.min(registration_token.len())]
            ),
            product_code: format!(
                "product-{}",
                &registration_token[..8.min(registration_token.len())]
            ),
            customer_aws_account_id: "123456789012".to_string(),
        };

        self.resolved_customers
            .insert(registration_token.to_string(), customer.clone());
        Ok(customer)
    }
}
