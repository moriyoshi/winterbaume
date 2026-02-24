//! Serde-compatible view types for MeteringMarketplace state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::MarketplaceMeteringService;
use crate::state::MeteringMarketplaceState;

/// Serializable view of a usage allocation tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageTagView {
    pub key: String,
    pub value: String,
}

/// Serializable view of a usage allocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageAllocationView {
    pub allocated_usage_quantity: i32,
    #[serde(default)]
    pub tags: Vec<UsageTagView>,
}

/// Serializable view of a metering record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeteringRecordView {
    pub product_code: String,
    pub timestamp: String,
    pub usage_dimension: String,
    pub usage_quantity: i32,
    pub dry_run: bool,
    pub metering_record_id: String,
    #[serde(default)]
    pub usage_allocations: Vec<UsageAllocationView>,
}

/// Serializable view of a registration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationView {
    pub product_code: String,
    pub public_key_rotation_timestamp: String,
    pub signature: String,
}

/// Serializable view of a resolved customer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedCustomerView {
    pub customer_identifier: String,
    pub product_code: String,
    pub customer_aws_account_id: String,
}

/// Serializable view of the entire MeteringMarketplace state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MeteringMarketplaceStateView {
    #[serde(default)]
    pub metering_records: Vec<MeteringRecordView>,
    #[serde(default)]
    pub registrations: HashMap<String, RegistrationView>,
    #[serde(default)]
    pub resolved_customers: HashMap<String, ResolvedCustomerView>,
    #[serde(default)]
    pub registration_tokens: HashMap<String, ResolvedCustomerView>,
}

// --- From internal types to view types ---

impl From<&crate::types::MeteringRecord> for MeteringRecordView {
    fn from(r: &crate::types::MeteringRecord) -> Self {
        MeteringRecordView {
            product_code: r.product_code.clone(),
            timestamp: r.timestamp.to_rfc3339(),
            usage_dimension: r.usage_dimension.clone(),
            usage_quantity: r.usage_quantity,
            dry_run: r.dry_run,
            metering_record_id: r.metering_record_id.clone(),
            usage_allocations: r
                .usage_allocations
                .iter()
                .map(|a| UsageAllocationView {
                    allocated_usage_quantity: a.allocated_usage_quantity,
                    tags: a
                        .tags
                        .iter()
                        .map(|t| UsageTagView {
                            key: t.key.clone(),
                            value: t.value.clone(),
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}

impl From<&crate::types::Registration> for RegistrationView {
    fn from(r: &crate::types::Registration) -> Self {
        RegistrationView {
            product_code: r.product_code.clone(),
            public_key_rotation_timestamp: r.public_key_rotation_timestamp.to_rfc3339(),
            signature: r.signature.clone(),
        }
    }
}

impl From<&crate::types::ResolvedCustomer> for ResolvedCustomerView {
    fn from(c: &crate::types::ResolvedCustomer) -> Self {
        ResolvedCustomerView {
            customer_identifier: c.customer_identifier.clone(),
            product_code: c.product_code.clone(),
            customer_aws_account_id: c.customer_aws_account_id.clone(),
        }
    }
}

impl From<&MeteringMarketplaceState> for MeteringMarketplaceStateView {
    fn from(state: &MeteringMarketplaceState) -> Self {
        MeteringMarketplaceStateView {
            metering_records: state
                .metering_records
                .iter()
                .map(MeteringRecordView::from)
                .collect(),
            registrations: state
                .registrations
                .iter()
                .map(|(k, v)| (k.clone(), RegistrationView::from(v)))
                .collect(),
            resolved_customers: state
                .resolved_customers
                .iter()
                .map(|(k, v)| (k.clone(), ResolvedCustomerView::from(v)))
                .collect(),
            registration_tokens: state
                .registration_tokens
                .iter()
                .map(|(k, v)| (k.clone(), ResolvedCustomerView::from(v)))
                .collect(),
        }
    }
}

// --- From view types to internal types ---

fn parse_dt(s: &str) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(|_| chrono::Utc::now())
}

impl From<MeteringRecordView> for crate::types::MeteringRecord {
    fn from(v: MeteringRecordView) -> Self {
        crate::types::MeteringRecord {
            product_code: v.product_code,
            timestamp: parse_dt(&v.timestamp),
            usage_dimension: v.usage_dimension,
            usage_quantity: v.usage_quantity,
            dry_run: v.dry_run,
            metering_record_id: v.metering_record_id,
            usage_allocations: v
                .usage_allocations
                .into_iter()
                .map(|a| crate::types::UsageAllocation {
                    allocated_usage_quantity: a.allocated_usage_quantity,
                    tags: a
                        .tags
                        .into_iter()
                        .map(|t| crate::types::UsageTag {
                            key: t.key,
                            value: t.value,
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}

impl From<RegistrationView> for crate::types::Registration {
    fn from(v: RegistrationView) -> Self {
        crate::types::Registration {
            product_code: v.product_code,
            public_key_rotation_timestamp: parse_dt(&v.public_key_rotation_timestamp),
            signature: v.signature,
        }
    }
}

impl From<ResolvedCustomerView> for crate::types::ResolvedCustomer {
    fn from(v: ResolvedCustomerView) -> Self {
        crate::types::ResolvedCustomer {
            customer_identifier: v.customer_identifier,
            product_code: v.product_code,
            customer_aws_account_id: v.customer_aws_account_id,
        }
    }
}

impl From<MeteringMarketplaceStateView> for MeteringMarketplaceState {
    fn from(view: MeteringMarketplaceStateView) -> Self {
        MeteringMarketplaceState {
            metering_records: view
                .metering_records
                .into_iter()
                .map(crate::types::MeteringRecord::from)
                .collect(),
            batch_results: Vec::new(),
            registrations: view
                .registrations
                .into_iter()
                .map(|(k, v)| (k, crate::types::Registration::from(v)))
                .collect(),
            resolved_customers: view
                .resolved_customers
                .into_iter()
                .map(|(k, v)| (k, crate::types::ResolvedCustomer::from(v)))
                .collect(),
            registration_tokens: view
                .registration_tokens
                .into_iter()
                .map(|(k, v)| (k, crate::types::ResolvedCustomer::from(v)))
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for MarketplaceMeteringService {
    type StateView = MeteringMarketplaceStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        MeteringMarketplaceStateView::from(&*guard)
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
            *guard = MeteringMarketplaceState::from(view);
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
            for record in view.metering_records {
                guard
                    .metering_records
                    .push(crate::types::MeteringRecord::from(record));
            }
            for (k, v) in view.registrations {
                guard
                    .registrations
                    .insert(k, crate::types::Registration::from(v));
            }
            for (k, v) in view.resolved_customers {
                guard
                    .resolved_customers
                    .insert(k, crate::types::ResolvedCustomer::from(v));
            }
            for (k, v) in view.registration_tokens {
                guard
                    .registration_tokens
                    .insert(k, crate::types::ResolvedCustomer::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
