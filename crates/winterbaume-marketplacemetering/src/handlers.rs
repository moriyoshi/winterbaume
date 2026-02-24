use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::state::{MeteringError, MeteringMarketplaceState};
use crate::types::*;
use crate::views::MeteringMarketplaceStateView;
use crate::wire;

/// AWS Marketplace Metering service handler that processes awsJson1.1 protocol requests.
pub struct MarketplaceMeteringService {
    pub(crate) state: Arc<BackendState<MeteringMarketplaceState>>,
    pub(crate) notifier: StateChangeNotifier<MeteringMarketplaceStateView>,
}

impl MarketplaceMeteringService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for MarketplaceMeteringService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for MarketplaceMeteringService {
    fn service_name(&self) -> &str {
        "meteringmarketplace"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://metering\.marketplace\.(.+)\.amazonaws\.com",
            r"https?://aws-marketplace\.(.+)\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl MarketplaceMeteringService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        // Extract action from X-Amz-Target header
        // Format: "AWSMPMeteringService.MeterUsage"
        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.rsplit('.').next())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "InvalidInput", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        match action.as_str() {
            "MeterUsage" => self.handle_meter_usage(&state, body_bytes).await,
            "RegisterUsage" => self.handle_register_usage(&state, body_bytes).await,
            "BatchMeterUsage" => self.handle_batch_meter_usage(&state, body_bytes).await,
            "ResolveCustomer" => self.handle_resolve_customer(&state, body_bytes).await,
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for AWSMPMeteringService"),
            ),
        }
    }

    async fn handle_meter_usage(
        &self,
        state: &Arc<tokio::sync::RwLock<MeteringMarketplaceState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_meter_usage_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.product_code.is_empty() {
            return json_error_response(
                400,
                "InvalidProductCodeException",
                "Missing 'ProductCode'",
            );
        }
        if input.usage_dimension.is_empty() {
            return json_error_response(
                400,
                "InvalidUsageDimensionException",
                "Missing 'UsageDimension'",
            );
        }

        let product_code = input.product_code.as_str();
        let usage_dimension = input.usage_dimension.as_str();
        let timestamp = if input.timestamp == 0.0 {
            chrono::Utc::now()
        } else {
            chrono::DateTime::from_timestamp(input.timestamp as i64, 0)
                .unwrap_or_else(chrono::Utc::now)
        };

        let usage_quantity = input.usage_quantity;
        let dry_run = input.dry_run.unwrap_or(false);

        let usage_allocations = wire_allocations_to_state(input.usage_allocations.as_deref());

        let mut state = state.write().await;
        match state.meter_usage(
            product_code,
            timestamp,
            usage_dimension,
            usage_quantity,
            dry_run,
            usage_allocations,
        ) {
            Ok(metering_record_id) => {
                wire::serialize_meter_usage_response(&wire::MeterUsageResult {
                    metering_record_id: Some(metering_record_id),
                })
            }
            Err(e) => metering_error_response(&e),
        }
    }

    async fn handle_register_usage(
        &self,
        state: &Arc<tokio::sync::RwLock<MeteringMarketplaceState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_register_usage_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.product_code.is_empty() {
            return json_error_response(
                400,
                "InvalidProductCodeException",
                "Missing 'ProductCode'",
            );
        }
        let product_code = input.product_code.as_str();
        let public_key_version = input.public_key_version;

        let mut state = state.write().await;
        match state.register_usage(product_code, public_key_version) {
            Ok(reg) => wire::serialize_register_usage_response(&wire::RegisterUsageResult {
                public_key_rotation_timestamp: Some(
                    reg.public_key_rotation_timestamp.timestamp() as f64
                ),
                signature: Some(reg.signature.clone()),
            }),
            Err(e) => metering_error_response(&e),
        }
    }

    async fn handle_batch_meter_usage(
        &self,
        state: &Arc<tokio::sync::RwLock<MeteringMarketplaceState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_meter_usage_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let product_code = match input.product_code.as_deref() {
            Some(p) if !p.is_empty() => p,
            _ => {
                return json_error_response(
                    400,
                    "InvalidProductCodeException",
                    "Missing 'ProductCode'",
                );
            }
        };

        let usage_records: Vec<UsageRecord> = input
            .usage_records
            .iter()
            .map(|record| {
                let timestamp = if record.timestamp == 0.0 {
                    chrono::Utc::now()
                } else {
                    chrono::DateTime::from_timestamp(record.timestamp as i64, 0)
                        .unwrap_or_else(chrono::Utc::now)
                };
                let customer_identifier = record.customer_identifier.clone().unwrap_or_default();
                let dimension = record.dimension.clone();
                let quantity = record.quantity.unwrap_or(0);
                let usage_allocations =
                    wire_allocations_to_state(record.usage_allocations.as_deref());

                UsageRecord {
                    timestamp,
                    customer_identifier,
                    dimension,
                    quantity,
                    usage_allocations,
                }
            })
            .collect();

        let mut state = state.write().await;
        match state.batch_meter_usage(product_code, usage_records) {
            Ok(results) => {
                let result_entries: Vec<wire::UsageRecordResult> = results
                    .iter()
                    .map(|r| wire::UsageRecordResult {
                        usage_record: Some(usage_record_to_wire(&r.usage_record)),
                        metering_record_id: Some(r.metering_record_id.clone()),
                        status: Some(r.status.as_str().to_string()),
                    })
                    .collect();

                wire::serialize_batch_meter_usage_response(&wire::BatchMeterUsageResult {
                    results: Some(result_entries),
                    unprocessed_records: Some(Vec::new()),
                })
            }
            Err(e) => metering_error_response(&e),
        }
    }

    async fn handle_resolve_customer(
        &self,
        state: &Arc<tokio::sync::RwLock<MeteringMarketplaceState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_resolve_customer_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.registration_token.is_empty() {
            return json_error_response(
                400,
                "InvalidTokenException",
                "Missing 'RegistrationToken'",
            );
        }
        let registration_token = input.registration_token.as_str();

        let mut state = state.write().await;
        match state.resolve_customer(registration_token) {
            Ok(customer) => {
                wire::serialize_resolve_customer_response(&wire::ResolveCustomerResult {
                    customer_identifier: Some(customer.customer_identifier.clone()),
                    product_code: Some(customer.product_code.clone()),
                    customer_a_w_s_account_id: Some(customer.customer_aws_account_id.clone()),
                    license_arn: None,
                })
            }
            Err(e) => metering_error_response(&e),
        }
    }
}

// --- State-to-wire helpers ---

/// Convert a state UsageRecord to a wire UsageRecord.
fn usage_record_to_wire(record: &UsageRecord) -> wire::UsageRecord {
    let wire_allocations = if record.usage_allocations.is_empty() {
        None
    } else {
        Some(
            record
                .usage_allocations
                .iter()
                .map(|a| wire::UsageAllocation {
                    allocated_usage_quantity: a.allocated_usage_quantity,
                    tags: if a.tags.is_empty() {
                        None
                    } else {
                        Some(
                            a.tags
                                .iter()
                                .map(|t| wire::Tag {
                                    key: t.key.clone(),
                                    value: t.value.clone(),
                                })
                                .collect(),
                        )
                    },
                })
                .collect(),
        )
    };

    wire::UsageRecord {
        timestamp: record.timestamp.timestamp() as f64,
        customer_identifier: Some(record.customer_identifier.clone()),
        dimension: record.dimension.clone(),
        quantity: Some(record.quantity),
        usage_allocations: wire_allocations,
        customer_a_w_s_account_id: None,
        license_arn: None,
    }
}

// --- Utility functions ---

fn wire_allocations_to_state(val: Option<&[wire::UsageAllocation]>) -> Vec<UsageAllocation> {
    let Some(arr) = val else {
        return Vec::new();
    };
    arr.iter()
        .map(|a| {
            let tags = a
                .tags
                .as_ref()
                .map(|tags| {
                    tags.iter()
                        .map(|t| UsageTag {
                            key: t.key.clone(),
                            value: t.value.clone(),
                        })
                        .collect()
                })
                .unwrap_or_default();
            UsageAllocation {
                allocated_usage_quantity: a.allocated_usage_quantity,
                tags,
            }
        })
        .collect()
}

fn metering_error_response(err: &MeteringError) -> MockResponse {
    let (status, error_type) = match err {
        MeteringError::InvalidProductCode => (400, "InvalidProductCodeException"),
        MeteringError::InvalidUsageDimension => (400, "InvalidUsageDimensionException"),
        MeteringError::InvalidPublicKeyVersion => (400, "InvalidPublicKeyVersionException"),
        MeteringError::EmptyUsageRecords => (400, "ValidationException"),
        MeteringError::InvalidToken => (400, "InvalidTokenException"),
    };
    let body = json!({
        "__type": error_type,
        "message": err.to_string(),
    });
    MockResponse::json(status, body.to_string())
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}
