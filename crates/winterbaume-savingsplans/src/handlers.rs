use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    extract_path, rest_json_error,
};

use crate::state::{PlanRecord, SavingsPlansError, SavingsPlansState};
use crate::views::SavingsPlansStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct SavingsPlansService {
    pub(crate) state: Arc<BackendState<SavingsPlansState>>,
    pub(crate) notifier: StateChangeNotifier<SavingsPlansStateView>,
}

impl SavingsPlansService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for SavingsPlansService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SavingsPlansService {
    fn service_name(&self) -> &str {
        "savingsplans"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://savingsplans\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<SavingsPlansState>>;

impl SavingsPlansService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let path = path.trim_start_matches('/');
        let raw_query = extract_query(&request.uri);
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(&raw_query);

        let response = match path {
            "CreateSavingsPlan" => {
                self.handle_create_savings_plan(
                    &state,
                    account_id,
                    &region,
                    &request,
                    &[],
                    &query_map,
                )
                .await
            }
            "DeleteQueuedSavingsPlan" => {
                self.handle_delete_queued(&state, &request, &[], &query_map)
                    .await
            }
            "DescribeSavingsPlanRates" => {
                self.handle_describe_rates(&state, &request, &[], &query_map)
                    .await
            }
            "DescribeSavingsPlans" => {
                self.handle_describe_plans(&state, &request, &[], &query_map)
                    .await
            }
            "DescribeSavingsPlansOfferingRates" => {
                self.handle_describe_offering_rates(&request, &[], &query_map)
                    .await
            }
            "DescribeSavingsPlansOfferings" => {
                self.handle_describe_offerings(&request, &[], &query_map)
                    .await
            }
            "ListTagsForResource" => {
                self.handle_list_tags(&state, &request, &[], &query_map)
                    .await
            }
            "ReturnSavingsPlan" => {
                self.handle_return_plan(&state, &request, &[], &query_map)
                    .await
            }
            "TagResource" => {
                self.handle_tag_resource(&state, &request, &[], &query_map)
                    .await
            }
            "UntagResource" => {
                self.handle_untag_resource(&state, &request, &[], &query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2
            && !path.starts_with("Describe")
            && path != "ListTagsForResource"
        {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_savings_plan(
        &self,
        state: &SharedState,
        account_id: &str,
        region: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_savings_plan_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.savings_plan_offering_id.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "savingsPlanOfferingId is required",
            );
        }
        if input.commitment.is_empty() {
            return rest_json_error(400, "ValidationException", "commitment is required");
        }
        let id = format!("sp-{}", uuid::Uuid::new_v4().simple());
        let arn = format!("arn:aws:savingsplans::{}:savingsplan/{}", account_id, id);
        let now = chrono::Utc::now();
        let end = now + chrono::Duration::days(365);
        let returnable_until = Some((now + chrono::Duration::days(7)).to_rfc3339());
        let tags = input.tags.unwrap_or_default();
        let plan = PlanRecord {
            savings_plan_id: id.clone(),
            savings_plan_arn: arn.clone(),
            offering_id: input.savings_plan_offering_id,
            commitment: input.commitment,
            upfront_payment_amount: input.upfront_payment_amount,
            recurring_payment_amount: None,
            start: now.to_rfc3339(),
            end: end.to_rfc3339(),
            state: "payment-pending".to_string(),
            region: region.to_string(),
            currency: "USD".to_string(),
            savings_plan_type: "Compute".to_string(),
            payment_option: "All Upfront".to_string(),
            product_types: vec!["EC2".to_string()],
            term_duration_in_seconds: 31536000,
            returnable_until,
            tags,
        };
        let mut state = state.write().await;
        state.create_plan(plan);
        wire::serialize_create_savings_plan_response(&wire::CreateSavingsPlanResponse {
            savings_plan_id: Some(id),
        })
    }

    async fn handle_delete_queued(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_queued_savings_plan_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.savings_plan_id.is_empty() {
            return rest_json_error(400, "ValidationException", "savingsPlanId is required");
        }
        let mut state = state.write().await;
        match state.delete_queued(&input.savings_plan_id) {
            Ok(()) => wire::serialize_delete_queued_savings_plan_response(
                &wire::DeleteQueuedSavingsPlanResponse {},
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_return_plan(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_return_savings_plan_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.savings_plan_id.is_empty() {
            return rest_json_error(400, "ValidationException", "savingsPlanId is required");
        }
        let id = input.savings_plan_id;
        let mut state = state.write().await;
        match state.return_plan(&id) {
            Ok(_) => {
                wire::serialize_return_savings_plan_response(&wire::ReturnSavingsPlanResponse {
                    savings_plan_id: Some(id),
                })
            }
            Err(e) => err_response(&e),
        }
    }

    async fn handle_describe_plans(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_savings_plans_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let ids = input.savings_plan_ids;
        let arns = input.savings_plan_arns;
        let states = input.states;
        let state = state.read().await;
        let plans: Vec<wire::SavingsPlan> = state
            .list_plans(ids.as_deref(), arns.as_deref(), states.as_deref())
            .into_iter()
            .map(plan_to_wire)
            .collect();
        wire::serialize_describe_savings_plans_response(&wire::DescribeSavingsPlansResponse {
            next_token: None,
            savings_plans: Some(plans),
        })
    }

    async fn handle_describe_rates(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_describe_savings_plan_rates_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.savings_plan_id.is_empty() {
            return rest_json_error(400, "ValidationException", "savingsPlanId is required");
        }
        let id = input.savings_plan_id;
        let state = state.read().await;
        match state.get_plan(&id) {
            Ok(_) => wire::serialize_describe_savings_plan_rates_response(
                &wire::DescribeSavingsPlanRatesResponse {
                    next_token: None,
                    savings_plan_id: Some(id),
                    search_results: Some(vec![]),
                },
            ),
            Err(e) => err_response(&e),
        }
    }

    // STUB[no-state]: Savings Plans offerings are a static AWS catalog, not tracked in mock state.
    async fn handle_describe_offerings(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_describe_savings_plans_offerings_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        wire::serialize_describe_savings_plans_offerings_response(
            &wire::DescribeSavingsPlansOfferingsResponse {
                next_token: None,
                search_results: Some(vec![]),
            },
        )
    }

    // STUB[no-state]: Savings Plans offering rates are a static AWS catalog, not tracked in mock state.
    async fn handle_describe_offering_rates(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_describe_savings_plans_offering_rates_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        wire::serialize_describe_savings_plans_offering_rates_response(
            &wire::DescribeSavingsPlansOfferingRatesResponse {
                next_token: None,
                search_results: Some(vec![]),
            },
        )
    }

    async fn handle_list_tags(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "resourceArn is required");
        }
        let state = state.read().await;
        let tags_map = state.list_tags(&input.resource_arn);
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: if tags_map.is_empty() {
                None
            } else {
                Some(tags_map)
            },
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "resourceArn is required");
        }
        if input.tags.is_empty() {
            return rest_json_error(400, "ValidationException", "tags is required");
        }
        let mut state = state.write().await;
        state.tag_resource(&input.resource_arn, input.tags);
        wire::serialize_tag_resource_response(&wire::TagResourceResponse {})
    }

    async fn handle_untag_resource(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "resourceArn is required");
        }
        let mut state = state.write().await;
        state.untag_resource(&input.resource_arn, &input.tag_keys);
        wire::serialize_untag_resource_response(&wire::UntagResourceResponse {})
    }
}

fn extract_query(uri: &str) -> String {
    match uri.find('?') {
        Some(idx) => uri[idx + 1..].to_string(),
        None => String::new(),
    }
}

fn plan_to_wire(p: &PlanRecord) -> wire::SavingsPlan {
    wire::SavingsPlan {
        commitment: Some(p.commitment.clone()),
        currency: Some(p.currency.clone()),
        description: None,
        ec2_instance_family: None,
        end: Some(p.end.clone()),
        offering_id: Some(p.offering_id.clone()),
        payment_option: Some(p.payment_option.clone()),
        product_types: Some(p.product_types.clone()),
        recurring_payment_amount: p.recurring_payment_amount.clone(),
        region: Some(p.region.clone()),
        returnable_until: p.returnable_until.clone(),
        savings_plan_arn: Some(p.savings_plan_arn.clone()),
        savings_plan_id: Some(p.savings_plan_id.clone()),
        savings_plan_type: Some(p.savings_plan_type.clone()),
        start: Some(p.start.clone()),
        state: Some(p.state.clone()),
        tags: if p.tags.is_empty() {
            None
        } else {
            Some(p.tags.clone())
        },
        term_duration_in_seconds: Some(p.term_duration_in_seconds),
        upfront_payment_amount: p.upfront_payment_amount.clone(),
    }
}

fn err_response(err: &SavingsPlansError) -> MockResponse {
    let (status, error_type) = match err {
        SavingsPlansError::NotFound { .. } => (404, "ResourceNotFoundException"),
        SavingsPlansError::InvalidState { .. } => (400, "ValidationException"),
        SavingsPlansError::Validation { .. } => (400, "ValidationException"),
    };
    let body = json!({"message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
