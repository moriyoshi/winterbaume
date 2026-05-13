use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
    protocol::common::{extract_path, extract_query_string, percent_decode},
};

use crate::model;
use crate::state::{CodeGuruReviewerError, CodeGuruReviewerState};
use crate::views::CodeGuruReviewerStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct CodeGuruReviewerService {
    pub(crate) state: Arc<BackendState<CodeGuruReviewerState>>,
    pub(crate) notifier: StateChangeNotifier<CodeGuruReviewerStateView>,
}

impl CodeGuruReviewerService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CodeGuruReviewerService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CodeGuruReviewerService {
    fn service_name(&self) -> &str {
        "codeguru-reviewer"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://codeguru-reviewer\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<CodeGuruReviewerState>>;

/// Convert a stored `serde_json::Value` blob to a typed model struct.
/// Values are constructed with JSON field names matching the Smithy model
/// `#[serde(rename = "...")]` attributes, so `from_value` round-trips cleanly.
/// Falls back to `Default::default()` on the (unexpected) deserialisation error.
fn from_value_or_default<T: serde::de::DeserializeOwned + Default>(v: Value) -> T {
    serde_json::from_value(v).unwrap_or_default()
}

impl CodeGuruReviewerService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let raw_segments: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(percent_decode)
            .collect();
        let segments: Vec<&str> = raw_segments.iter().map(|s| s.as_str()).collect();
        let raw_query = extract_query_string(&request.uri);
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(raw_query);
        let method = request.method.as_str().to_string();

        let (response, mutating) = match (method.as_str(), segments.as_slice()) {
            ("POST", ["associations"]) => (
                self.handle_associate_repository(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    &region,
                    account_id,
                )
                .await,
                true,
            ),
            ("GET", ["associations"]) => (
                self.handle_list_associations(&state, &request, &[], &query_map)
                    .await,
                false,
            ),
            ("GET", ["associations", arn]) => {
                let labels: &[(&str, &str)] = &[("AssociationArn", arn)];
                (
                    self.handle_describe_repository_association(
                        &state, &request, labels, &query_map,
                    )
                    .await,
                    false,
                )
            }
            ("DELETE", ["associations", arn]) => {
                let labels: &[(&str, &str)] = &[("AssociationArn", arn)];
                (
                    self.handle_disassociate_repository(&state, &request, labels, &query_map)
                        .await,
                    true,
                )
            }
            ("POST", ["codereviews"]) => (
                self.handle_create_code_review(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    &region,
                    account_id,
                )
                .await,
                true,
            ),
            ("GET", ["codereviews"]) => (
                self.handle_list_code_reviews(&state, &request, &[], &query_map)
                    .await,
                false,
            ),
            ("GET", ["codereviews", arn]) => {
                let labels: &[(&str, &str)] = &[("CodeReviewArn", arn)];
                (
                    self.handle_describe_code_review(&state, &request, labels, &query_map)
                        .await,
                    false,
                )
            }
            ("GET", ["codereviews", arn, "Recommendations"]) => {
                let labels: &[(&str, &str)] = &[("CodeReviewArn", arn)];
                (
                    self.handle_list_recommendations(&state, &request, labels, &query_map)
                        .await,
                    false,
                )
            }
            ("PUT", ["feedback"]) => (
                self.handle_put_feedback(&state, &request, &[], &query_map)
                    .await,
                true,
            ),
            ("GET", ["feedback", code_review_arn]) => {
                let labels: &[(&str, &str)] = &[("CodeReviewArn", code_review_arn)];
                (
                    self.handle_describe_feedback(&state, &request, labels, &query_map)
                        .await,
                    false,
                )
            }
            ("GET", ["feedback", code_review_arn, "RecommendationFeedback"]) => {
                let labels: &[(&str, &str)] = &[("CodeReviewArn", code_review_arn)];
                (
                    self.handle_list_feedback(&state, &request, labels, &query_map)
                        .await,
                    false,
                )
            }
            ("GET", ["tags", arn]) => {
                let labels: &[(&str, &str)] = &[("resourceArn", arn)];
                (
                    self.handle_list_tags(&state, &request, labels, &query_map)
                        .await,
                    false,
                )
            }
            ("POST", ["tags", arn]) => {
                let labels: &[(&str, &str)] = &[("resourceArn", arn)];
                (
                    self.handle_tag_resource(&state, &request, labels, &query_map)
                        .await,
                    true,
                )
            }
            ("DELETE", ["tags", arn]) => {
                let labels: &[(&str, &str)] = &[("resourceArn", arn)];
                (
                    self.handle_untag_resource(&state, &request, labels, &query_map, raw_query)
                        .await,
                    true,
                )
            }
            _ => (
                rest_json_error(404, "ResourceNotFoundException", "No route matches"),
                false,
            ),
        };

        if response.status / 100 == 2 && mutating {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_associate_repository(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_associate_repository_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let repository = serde_json::to_value(&input.repository).unwrap_or(Value::Null);
        let kms_details = input
            .k_m_s_key_details
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok())
            .unwrap_or(Value::Null);
        let (provider, name, owner, connection_arn) = repo_meta(&repository);
        let association_id = uuid::Uuid::new_v4().to_string();
        let arn =
            format!("arn:aws:codeguru-reviewer:{region}:{account_id}:association:{association_id}");
        let now = chrono::Utc::now().timestamp() as f64;
        let association = json!({
            "AssociationArn": arn,
            "AssociationId": association_id,
            "ConnectionArn": connection_arn,
            "Name": name,
            "Owner": owner,
            "ProviderType": provider,
            "State": "Associated",
            "CreatedTimeStamp": now,
            "LastUpdatedTimeStamp": now,
            "KMSKeyDetails": kms_details,
        });
        let tag_map: HashMap<String, String> = input.tags.unwrap_or_default();
        let mut state_guard = state.write().await;
        state_guard.put_association(arn.clone(), association.clone());
        if !tag_map.is_empty() {
            state_guard.tag_resource(&arn, tag_map.clone());
        }
        drop(state_guard);
        let typed_assoc: model::RepositoryAssociation = from_value_or_default(association);
        wire::serialize_associate_repository_response(&model::AssociateRepositoryResponse {
            repository_association: Some(typed_assoc),
            tags: if tag_map.is_empty() {
                None
            } else {
                Some(tag_map)
            },
        })
    }

    async fn handle_list_associations(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_list_repository_associations_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let summaries: Vec<model::RepositoryAssociationSummary> = state
            .list_associations()
            .into_iter()
            .map(|a| from_value_or_default::<model::RepositoryAssociationSummary>(a.clone()))
            .collect();
        wire::serialize_list_repository_associations_response(
            &model::ListRepositoryAssociationsResponse {
                repository_association_summaries: Some(summaries),
                next_token: None,
            },
        )
    }

    async fn handle_describe_repository_association(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_describe_repository_association_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let arn = &input.association_arn;
        let state = state.read().await;
        match state.associations.get(arn) {
            Some(a) => {
                let tags = state.list_tags(arn);
                let typed_assoc: model::RepositoryAssociation = from_value_or_default(a.clone());
                wire::serialize_describe_repository_association_response(
                    &model::DescribeRepositoryAssociationResponse {
                        repository_association: Some(typed_assoc),
                        tags: Some(tags),
                    },
                )
            }
            None => err_response(&CodeGuruReviewerError::NotFound(format!(
                "association {arn}"
            ))),
        }
    }

    async fn handle_disassociate_repository(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_disassociate_repository_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let arn = &input.association_arn;
        let mut state = state.write().await;
        match state.associations.remove(arn) {
            Some(a) => {
                let tags = state.list_tags(arn);
                state.tags.remove(arn);
                let typed_assoc: model::RepositoryAssociation = from_value_or_default(a);
                wire::serialize_disassociate_repository_response(
                    &model::DisassociateRepositoryResponse {
                        repository_association: Some(typed_assoc),
                        tags: Some(tags),
                    },
                )
            }
            None => err_response(&CodeGuruReviewerError::NotFound(format!(
                "association {arn}"
            ))),
        }
    }

    async fn handle_create_code_review(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_code_review_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = if input.name.is_empty() {
            "review".to_string()
        } else {
            input.name
        };
        let association_arn = input.repository_association_arn;
        let review_id = uuid::Uuid::new_v4().to_string();
        let arn =
            format!("arn:aws:codeguru-reviewer:{region}:{account_id}:code-review:{review_id}");
        let now = chrono::Utc::now().timestamp() as f64;
        let association_meta = {
            let state = state.read().await;
            state
                .associations
                .get(&association_arn)
                .cloned()
                .unwrap_or(Value::Null)
        };
        let provider_type = pluck(&association_meta, "ProviderType");
        let owner = pluck(&association_meta, "Owner");
        let repo_name = match pluck(&association_meta, "Name") {
            Value::Null => json!("repo"),
            other => other,
        };
        // Re-serialize the typed CodeReviewType to JSON to access nested fields without
        // touching the original request body.
        let r#type_value = serde_json::to_value(&input.r#type).unwrap_or(Value::Null);
        let source_code_type = pluck(&r#type_value, "RepositoryAnalysis");
        let source_code_type = pluck(&source_code_type, "SourceCodeType");
        let analysis_types = match pluck(&r#type_value, "AnalysisTypes") {
            Value::Null => json!([]),
            other => other,
        };
        let code_review = json!({
            "Name": name,
            "CodeReviewArn": arn,
            "RepositoryName": repo_name,
            "Owner": owner,
            "ProviderType": provider_type,
            "State": "Completed",
            "StateReason": Value::Null,
            "CreatedTimeStamp": now,
            "LastUpdatedTimeStamp": now,
            "Type": "RepositoryAnalysis",
            "PullRequestId": Value::Null,
            "SourceCodeType": source_code_type,
            "AssociationArn": association_arn,
            "Metrics": {"MeteredLinesOfCodeCount": 0, "FindingsCount": 0},
            "AnalysisTypes": analysis_types,
            "ConfigFileState": "Absent",
        });
        let mut state = state.write().await;
        state.code_reviews.insert(arn.clone(), code_review.clone());
        drop(state);
        let typed_cr: model::CodeReview = from_value_or_default(code_review);
        wire::serialize_create_code_review_response(&model::CreateCodeReviewResponse {
            code_review: Some(typed_cr),
        })
    }

    async fn handle_list_code_reviews(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_code_reviews_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let summaries: Vec<model::CodeReviewSummary> = state
            .list_code_reviews()
            .into_iter()
            .map(|cr| from_value_or_default::<model::CodeReviewSummary>(cr.clone()))
            .collect();
        wire::serialize_list_code_reviews_response(&model::ListCodeReviewsResponse {
            code_review_summaries: Some(summaries),
            next_token: None,
        })
    }

    async fn handle_describe_code_review(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_code_review_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let arn = &input.code_review_arn;
        let state = state.read().await;
        match state.code_reviews.get(arn) {
            Some(cr) => {
                let typed_cr: model::CodeReview = from_value_or_default(cr.clone());
                wire::serialize_describe_code_review_response(&model::DescribeCodeReviewResponse {
                    code_review: Some(typed_cr),
                })
            }
            None => err_response(&CodeGuruReviewerError::NotFound(format!(
                "code review {arn}"
            ))),
        }
    }

    async fn handle_list_recommendations(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_recommendations_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let arn = &input.code_review_arn;
        let state = state.read().await;
        let recs: Vec<model::RecommendationSummary> = state
            .recommendations
            .get(arn)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(from_value_or_default::<model::RecommendationSummary>)
            .collect();
        wire::serialize_list_recommendations_response(&model::ListRecommendationsResponse {
            recommendation_summaries: Some(recs),
            next_token: None,
        })
    }

    async fn handle_put_feedback(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_put_recommendation_feedback_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let arn = input.code_review_arn;
        let recommendation_id = input.recommendation_id;
        let reactions = serde_json::to_value(&input.reactions).unwrap_or(json!([]));
        let user_id = "AROADGYTBVKRYBI3RDSDIE7DCOMR1AJ8".to_string();
        let now = chrono::Utc::now().timestamp() as f64;
        let feedback = json!({
            "CodeReviewArn": arn,
            "RecommendationId": recommendation_id,
            "Reactions": reactions,
            "UserId": user_id,
            "CreatedTimeStamp": now,
            "LastUpdatedTimeStamp": now,
        });
        let mut state = state.write().await;
        state
            .feedback
            .insert((arn, recommendation_id, user_id), feedback);
        wire::serialize_put_recommendation_feedback_response(
            &model::PutRecommendationFeedbackResponse {},
        )
    }

    async fn handle_describe_feedback(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_recommendation_feedback_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let code_review_arn = input.code_review_arn;
        let recommendation_id = input.recommendation_id;
        let state = state.read().await;
        let user_id = match input.user_id.as_deref() {
            Some(u) if !u.is_empty() => u.to_string(),
            _ => "AROADGYTBVKRYBI3RDSDIE7DCOMR1AJ8".to_string(),
        };
        let key = (code_review_arn, recommendation_id.clone(), user_id);
        match state.feedback.get(&key) {
            Some(f) => {
                let typed_fb: model::RecommendationFeedback = from_value_or_default(f.clone());
                wire::serialize_describe_recommendation_feedback_response(
                    &model::DescribeRecommendationFeedbackResponse {
                        recommendation_feedback: Some(typed_fb),
                    },
                )
            }
            None => err_response(&CodeGuruReviewerError::NotFound(format!(
                "feedback for {recommendation_id}"
            ))),
        }
    }

    async fn handle_list_feedback(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_list_recommendation_feedback_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let code_review_arn = input.code_review_arn;
        let state = state.read().await;
        let summaries: Vec<model::RecommendationFeedbackSummary> = state
            .feedback
            .iter()
            .filter(|((arn, _, _), _)| arn == &code_review_arn)
            .map(|(_, f)| from_value_or_default::<model::RecommendationFeedbackSummary>(f.clone()))
            .collect();
        wire::serialize_list_recommendation_feedback_response(
            &model::ListRecommendationFeedbackResponse {
                recommendation_feedback_summaries: Some(summaries),
                next_token: None,
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
        let state = state.read().await;
        let tags = state.list_tags(&input.resource_arn);
        wire::serialize_list_tags_for_resource_response(&model::ListTagsForResourceResponse {
            tags: Some(tags),
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
        let mut state = state.write().await;
        state.tag_resource(&input.resource_arn, input.tags);
        wire::serialize_tag_resource_response(&model::TagResourceResponse {})
    }

    async fn handle_untag_resource(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        raw_query: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // The AWS SDK sends tag keys as repeated query params (?tagKeys=k1&tagKeys=k2),
        // but parse_query_string keeps only the last value, so parse them from the raw
        // query string here. Fall back to the wire deserializer's parsed value (which
        // assumes a comma-separated single param) if no repeated params were found.
        let mut keys: Vec<String> = url_query_get_all(raw_query, "tagKeys")
            .into_iter()
            .map(percent_decode)
            .collect();
        if keys.is_empty() {
            keys = input.tag_keys;
        }
        let mut state = state.write().await;
        state.untag_resource(&input.resource_arn, &keys);
        wire::serialize_untag_resource_response(&model::UntagResourceResponse {})
    }
}

/// Helper to extract a nested field from a JSON value as a Value, returning Value::Null on miss.
fn pluck(v: &Value, key: &str) -> Value {
    match v {
        Value::Object(m) => m.get(key).cloned().unwrap_or(Value::Null),
        _ => Value::Null,
    }
}

fn repo_meta(repo: &Value) -> (Value, Value, Value, Value) {
    if let Value::Object(m) = repo {
        if let Some(cc) = m.get("CodeCommit") {
            return (
                json!("CodeCommit"),
                pluck(cc, "Name"),
                Value::Null,
                Value::Null,
            );
        }
        if let Some(s3) = m.get("S3Bucket") {
            return (
                json!("S3Bucket"),
                pluck(s3, "Name"),
                Value::Null,
                Value::Null,
            );
        }
        for (provider, key) in &[
            ("Bitbucket", "Bitbucket"),
            ("GitHubEnterpriseServer", "GitHubEnterpriseServer"),
        ] {
            if let Some(t) = m.get(*key) {
                return (
                    json!(provider),
                    pluck(t, "Name"),
                    pluck(t, "Owner"),
                    pluck(t, "ConnectionArn"),
                );
            }
        }
    }
    (Value::Null, Value::Null, Value::Null, Value::Null)
}

fn url_query_get_all<'a>(query: &'a str, key: &str) -> Vec<&'a str> {
    query
        .split('&')
        .filter_map(|kv| {
            let (k, v) = kv.split_once('=').unwrap_or((kv, ""));
            if k == key { Some(v) } else { None }
        })
        .collect()
}

fn rest_json_error(status: u16, error_type: &str, message: &str) -> MockResponse {
    let body = json!({"__type": error_type, "Message": message});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn err_response(err: &CodeGuruReviewerError) -> MockResponse {
    let (status, error_type) = match err {
        CodeGuruReviewerError::NotFound(_) => (404, "NotFoundException"),
        CodeGuruReviewerError::Validation(_) => (400, "ValidationException"),
    };
    rest_json_error(status, error_type, &err.to_string())
}
