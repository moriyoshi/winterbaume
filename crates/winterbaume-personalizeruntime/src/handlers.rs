use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    extract_path, rest_json_error,
};

use crate::state::PersonalizeRuntimeState;
use crate::views::PersonalizeRuntimeStateView;
use crate::wire;

pub struct PersonalizeRuntimeService {
    pub(crate) state: Arc<BackendState<PersonalizeRuntimeState>>,
    pub(crate) notifier: StateChangeNotifier<PersonalizeRuntimeStateView>,
}

impl PersonalizeRuntimeService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for PersonalizeRuntimeService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for PersonalizeRuntimeService {
    fn service_name(&self) -> &str {
        "personalize-runtime"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://personalize-runtime\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<PersonalizeRuntimeState>>;

impl PersonalizeRuntimeService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let path = path.trim_start_matches('/').trim_end_matches('/');
        let raw_query = extract_query(&request.uri);
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(&raw_query);

        let response = match path {
            "recommendations" => {
                self.handle_get_recommendations(&state, &request, &[], &query_map)
                    .await
            }
            "personalize-ranking" => {
                self.handle_get_personalized_ranking(&state, &request, &[], &query_map)
                    .await
            }
            "action-recommendations" => {
                self.handle_get_action_recommendations(&state, &request, &[], &query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_get_recommendations(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_recommendations_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let arn = input
            .campaign_arn
            .as_deref()
            .or(input.recommender_arn.as_deref())
            .unwrap_or("");
        if arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidInputException",
                "campaignArn or recommenderArn is required",
            );
        }
        let num = input.num_results.unwrap_or(25).clamp(1, 500) as usize;
        let item_id = input.item_id.as_deref();
        let user_id = input.user_id.as_deref();
        let mut state = state.write().await;
        state.record_call(arn);
        let items: Vec<wire::PredictedItem> = (0..num)
            .map(|i| wire::PredictedItem {
                item_id: Some(generate_item_id(item_id, user_id, i)),
                metadata: None,
                promotion_name: None,
                reason: None,
                score: Some(score_for(i, num)),
            })
            .collect();
        wire::serialize_get_recommendations_response(&wire::GetRecommendationsResponse {
            item_list: Some(items),
            recommendation_id: Some(format!("rec-{}", uuid::Uuid::new_v4().simple())),
        })
    }

    async fn handle_get_personalized_ranking(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_personalized_ranking_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.campaign_arn.is_empty() {
            return rest_json_error(400, "InvalidInputException", "campaignArn is required");
        }
        if input.user_id.is_empty() {
            return rest_json_error(400, "InvalidInputException", "userId is required");
        }
        let arn = input.campaign_arn.clone();
        let mut state = state.write().await;
        state.record_call(&arn);
        let n = input.input_list.len().max(1);
        let ranked: Vec<wire::PredictedItem> = input
            .input_list
            .into_iter()
            .enumerate()
            .map(|(i, id)| wire::PredictedItem {
                item_id: Some(id),
                metadata: None,
                promotion_name: None,
                reason: None,
                score: Some(score_for(i, n)),
            })
            .collect();
        wire::serialize_get_personalized_ranking_response(&wire::GetPersonalizedRankingResponse {
            personalized_ranking: Some(ranked),
            recommendation_id: Some(format!("rec-{}", uuid::Uuid::new_v4().simple())),
        })
    }

    async fn handle_get_action_recommendations(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_action_recommendations_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "BadRequestException", &e),
            };
        let arn = input.campaign_arn.as_deref().unwrap_or("");
        if arn.is_empty() {
            return rest_json_error(400, "InvalidInputException", "campaignArn is required");
        }
        let num = input.num_results.unwrap_or(5).clamp(1, 100) as usize;
        let user_id = input.user_id.as_deref();
        let arn_owned = arn.to_string();
        let mut state = state.write().await;
        state.record_call(&arn_owned);
        let actions: Vec<wire::PredictedAction> = (0..num)
            .map(|i| wire::PredictedAction {
                action_id: Some(generate_item_id(None, user_id, i)),
                score: Some(score_for(i, num)),
            })
            .collect();
        wire::serialize_get_action_recommendations_response(
            &wire::GetActionRecommendationsResponse {
                action_list: Some(actions),
                recommendation_id: Some(format!("rec-{}", uuid::Uuid::new_v4().simple())),
            },
        )
    }
}

fn extract_query(uri: &str) -> String {
    match uri.find('?') {
        Some(idx) => uri[idx + 1..].to_string(),
        None => String::new(),
    }
}

fn score_for(i: usize, total: usize) -> f64 {
    let denom = total.max(1) as f64;
    (1.0 - i as f64 / denom).max(0.0)
}

fn generate_item_id(item_id: Option<&str>, user_id: Option<&str>, i: usize) -> String {
    if let Some(item) = item_id {
        format!("{item}-related-{i}")
    } else if let Some(user) = user_id {
        format!("{user}-rec-{i}")
    } else {
        format!("item-{i}")
    }
}
