use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use tokio::sync::RwLock;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::model;
use crate::state::{LexError, LexState};
use crate::views::LexStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

/// Convert an RFC 3339 timestamp string into the wire's `f64` epoch seconds.
/// Returns `None` if the string is empty or unparseable, mirroring AWS behaviour
/// where the wire field is optional.
fn rfc3339_to_epoch(s: &str) -> Option<f64> {
    chrono::DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|dt| dt.timestamp() as f64)
}

pub struct LexModelsV2Service {
    pub(crate) state: Arc<BackendState<LexState>>,
    pub(crate) notifier: StateChangeNotifier<LexStateView>,
}

impl LexModelsV2Service {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for LexModelsV2Service {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for LexModelsV2Service {
    fn service_name(&self) -> &str {
        "lex"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://models\.lex\..*\.amazonaws\.com",
            r"https?://models-v2-lex\..*\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

fn rest_json_error(status: u16, error_type: &str, message: &str) -> MockResponse {
    let body = json!({ "message": message });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn service_error_response(e: &LexError) -> MockResponse {
    let (status, error_type) = match e {
        LexError::ResourceNotFound { .. } => (404, "ResourceNotFoundException"),
        LexError::Conflict(_) => (409, "ConflictException"),
        LexError::Validation(_) => (400, "ValidationException"),
    };
    rest_json_error(status, error_type, &e.to_string())
}

fn extract_path(uri: &str) -> String {
    let without_scheme = if let Some(pos) = uri.find("://") {
        &uri[pos + 3..]
    } else {
        uri
    };
    let path_start = without_scheme.find('/').unwrap_or(without_scheme.len());
    let path = &without_scheme[path_start..];
    let query_start = path.find('?').unwrap_or(path.len());
    path[..query_start].to_string()
}

fn percent_decode(s: &str) -> String {
    urlencoding::decode(s)
        .unwrap_or_else(|_| s.into())
        .into_owned()
}

impl LexModelsV2Service {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();

        let query_string = if let Some(pos) = request.uri.find('?') {
            &request.uri[pos + 1..]
        } else {
            ""
        };
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(query_string);

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        let response = self
            .route(
                method, &segments, &request, &query_map, &state, account_id, &region,
            )
            .await;

        if matches!(method, "PUT" | "POST" | "DELETE") && response.status / 100 == 2 {
            use winterbaume_core::StatefulService;
            self.notify_state_changed(account_id, &region).await;
        }

        response
    }

    #[allow(clippy::too_many_arguments)]
    async fn route(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &HashMap<String, String>,
        state: &Arc<RwLock<LexState>>,
        _account_id: &str,
        _region: &str,
    ) -> MockResponse {
        // Handle /policy/{resourceArn} where resourceArn may contain slashes
        if segments.first() == Some(&"policy") && segments.len() >= 2 {
            let resource_arn = percent_decode(&segments[1..].join("/"));
            let labels: &[(&str, &str)] = &[("resourceArn", resource_arn.as_str())];
            return match method {
                "POST" => {
                    self.handle_create_resource_policy(state, request, labels, query)
                        .await
                }
                "PUT" => {
                    self.handle_update_resource_policy(state, request, labels, query)
                        .await
                }
                "GET" => {
                    self.handle_describe_resource_policy(state, request, labels, query)
                        .await
                }
                "DELETE" => {
                    self.handle_delete_resource_policy(state, request, labels, query)
                        .await
                }
                _ => rest_json_error(405, "MethodNotAllowedException", "Method not allowed"),
            };
        }

        match (method, segments) {
            // PUT /bots - CreateBot
            ("PUT", ["bots"]) => self.handle_create_bot(state, request, &[], query).await,
            // POST /bots - ListBots
            ("POST", ["bots"]) => self.handle_list_bots(state).await,
            // GET /bots/{botId} - DescribeBot
            ("GET", ["bots", bot_id]) => {
                let bot_id = percent_decode(bot_id);
                let labels: &[(&str, &str)] = &[("botId", bot_id.as_str())];
                self.handle_describe_bot(state, request, labels, query)
                    .await
            }
            // DELETE /bots/{botId} - DeleteBot
            ("DELETE", ["bots", bot_id]) => {
                let bot_id = percent_decode(bot_id);
                let labels: &[(&str, &str)] = &[("botId", bot_id.as_str())];
                self.handle_delete_bot(state, request, labels, query).await
            }
            // PUT /bots/{botId} - UpdateBot
            ("PUT", ["bots", bot_id]) => {
                let bot_id = percent_decode(bot_id);
                let labels: &[(&str, &str)] = &[("botId", bot_id.as_str())];
                self.handle_update_bot(state, request, labels, query).await
            }
            // PUT /bots/{botId}/botaliases - CreateBotAlias
            ("PUT", ["bots", bot_id, "botaliases"]) => {
                let bot_id = percent_decode(bot_id);
                let labels: &[(&str, &str)] = &[("botId", bot_id.as_str())];
                self.handle_create_bot_alias(state, request, labels, query)
                    .await
            }
            // POST /bots/{botId}/botaliases - ListBotAliases
            ("POST", ["bots", bot_id, "botaliases"]) => {
                let bot_id = percent_decode(bot_id);
                let labels: &[(&str, &str)] = &[("botId", bot_id.as_str())];
                self.handle_list_bot_aliases(state, request, labels, query)
                    .await
            }
            // GET /bots/{botId}/botaliases/{botAliasId} - DescribeBotAlias
            ("GET", ["bots", bot_id, "botaliases", alias_id]) => {
                let bot_id = percent_decode(bot_id);
                let alias_id = percent_decode(alias_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botAliasId", alias_id.as_str()),
                ];
                self.handle_describe_bot_alias(state, request, labels, query)
                    .await
            }
            // DELETE /bots/{botId}/botaliases/{botAliasId} - DeleteBotAlias
            ("DELETE", ["bots", bot_id, "botaliases", alias_id]) => {
                let bot_id = percent_decode(bot_id);
                let alias_id = percent_decode(alias_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botAliasId", alias_id.as_str()),
                ];
                self.handle_delete_bot_alias(state, request, labels, query)
                    .await
            }
            // PUT /bots/{botId}/botaliases/{botAliasId} - UpdateBotAlias
            ("PUT", ["bots", bot_id, "botaliases", alias_id]) => {
                let bot_id = percent_decode(bot_id);
                let alias_id = percent_decode(alias_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botAliasId", alias_id.as_str()),
                ];
                self.handle_update_bot_alias(state, request, labels, query)
                    .await
            }
            // PUT /bots/{botId}/botversions/{botVersion}/botlocales - CreateBotLocale
            ("PUT", ["bots", bot_id, "botversions", bot_version, "botlocales"]) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                ];
                self.handle_create_bot_locale(state, request, labels, query)
                    .await
            }
            // POST /bots/{botId}/botversions/{botVersion}/botlocales - ListBotLocales
            ("POST", ["bots", bot_id, "botversions", bot_version, "botlocales"]) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                ];
                self.handle_list_bot_locales(state, request, labels, query)
                    .await
            }
            // GET /bots/{botId}/botversions/{botVersion}/botlocales/{localeId} - DescribeBotLocale
            (
                "GET",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                ];
                self.handle_describe_bot_locale(state, request, labels, query)
                    .await
            }
            // DELETE /bots/{botId}/botversions/{botVersion}/botlocales/{localeId} - DeleteBotLocale
            (
                "DELETE",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                ];
                self.handle_delete_bot_locale(state, request, labels, query)
                    .await
            }
            // PUT /bots/{botId}/botversions/{botVersion}/botlocales/{localeId} - UpdateBotLocale
            (
                "PUT",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                ];
                self.handle_update_bot_locale(state, request, labels, query)
                    .await
            }
            // POST /bots/{botId}/botversions/{botVersion}/botlocales/{localeId} - BuildBotLocale
            (
                "POST",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                ];
                self.handle_build_bot_locale(state, request, labels, query)
                    .await
            }
            // PUT /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/intents - CreateIntent
            (
                "PUT",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                    "intents",
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                ];
                self.handle_create_intent(state, request, labels, query)
                    .await
            }
            // POST /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/intents - ListIntents
            (
                "POST",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                    "intents",
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                ];
                self.handle_list_intents(state, request, labels, query)
                    .await
            }
            // GET /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/intents/{intentId} - DescribeIntent
            (
                "GET",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                    "intents",
                    intent_id,
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let intent_id = percent_decode(intent_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                    ("intentId", intent_id.as_str()),
                ];
                self.handle_describe_intent(state, request, labels, query)
                    .await
            }
            // DELETE /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/intents/{intentId} - DeleteIntent
            (
                "DELETE",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                    "intents",
                    intent_id,
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let intent_id = percent_decode(intent_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                    ("intentId", intent_id.as_str()),
                ];
                self.handle_delete_intent(state, request, labels, query)
                    .await
            }
            // PUT /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/intents/{intentId} - UpdateIntent
            (
                "PUT",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                    "intents",
                    intent_id,
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let intent_id = percent_decode(intent_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                    ("intentId", intent_id.as_str()),
                ];
                self.handle_update_intent(state, request, labels, query)
                    .await
            }
            // PUT /bots/{botId}/botversions - CreateBotVersion
            ("PUT", ["bots", bot_id, "botversions"]) => {
                let bot_id = percent_decode(bot_id);
                let labels: &[(&str, &str)] = &[("botId", bot_id.as_str())];
                self.handle_create_bot_version(state, request, labels, query)
                    .await
            }
            // POST /bots/{botId}/botversions - ListBotVersions
            ("POST", ["bots", bot_id, "botversions"]) => {
                let bot_id = percent_decode(bot_id);
                let labels: &[(&str, &str)] = &[("botId", bot_id.as_str())];
                self.handle_list_bot_versions(state, request, labels, query)
                    .await
            }
            // GET /bots/{botId}/botversions/{botVersion} - DescribeBotVersion
            ("GET", ["bots", bot_id, "botversions", version]) => {
                let bot_id = percent_decode(bot_id);
                let version = percent_decode(version);
                let labels: &[(&str, &str)] =
                    &[("botId", bot_id.as_str()), ("botVersion", version.as_str())];
                self.handle_describe_bot_version(state, request, labels, query)
                    .await
            }
            // DELETE /bots/{botId}/botversions/{botVersion} - DeleteBotVersion
            ("DELETE", ["bots", bot_id, "botversions", version]) => {
                let bot_id = percent_decode(bot_id);
                let version = percent_decode(version);
                let labels: &[(&str, &str)] =
                    &[("botId", bot_id.as_str()), ("botVersion", version.as_str())];
                self.handle_delete_bot_version(state, request, labels, query)
                    .await
            }
            // GET /tags/{resourceARN} - ListTagsForResource
            ("GET", ["tags", resource_arn]) => {
                let resource_arn = percent_decode(resource_arn);
                let labels: &[(&str, &str)] = &[("resourceARN", resource_arn.as_str())];
                self.handle_list_tags_for_resource(state, request, labels, query)
                    .await
            }
            // POST /tags/{resourceARN} - TagResource
            ("POST", ["tags", resource_arn]) => {
                let resource_arn = percent_decode(resource_arn);
                let labels: &[(&str, &str)] = &[("resourceARN", resource_arn.as_str())];
                self.handle_tag_resource(state, request, labels, query)
                    .await
            }
            // DELETE /tags/{resourceARN} - UntagResource (tag keys come from query string)
            ("DELETE", ["tags", resource_arn]) => {
                let resource_arn = percent_decode(resource_arn);
                let labels: &[(&str, &str)] = &[("resourceARN", resource_arn.as_str())];
                self.handle_untag_resource(state, request, labels, query)
                    .await
            }
            // ---- Slot routes ----
            // PUT /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/intents/{intentId}/slots
            (
                "PUT",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                    "intents",
                    intent_id,
                    "slots",
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let intent_id = percent_decode(intent_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                    ("intentId", intent_id.as_str()),
                ];
                self.handle_create_slot(state, request, labels, query).await
            }
            // POST /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/intents/{intentId}/slots
            (
                "POST",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                    "intents",
                    intent_id,
                    "slots",
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let intent_id = percent_decode(intent_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                    ("intentId", intent_id.as_str()),
                ];
                self.handle_list_slots(state, request, labels, query).await
            }
            // GET /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/intents/{intentId}/slots/{slotId}
            (
                "GET",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                    "intents",
                    intent_id,
                    "slots",
                    slot_id,
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let intent_id = percent_decode(intent_id);
                let slot_id = percent_decode(slot_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                    ("intentId", intent_id.as_str()),
                    ("slotId", slot_id.as_str()),
                ];
                self.handle_describe_slot(state, request, labels, query)
                    .await
            }
            // DELETE /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/intents/{intentId}/slots/{slotId}
            (
                "DELETE",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                    "intents",
                    intent_id,
                    "slots",
                    slot_id,
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let intent_id = percent_decode(intent_id);
                let slot_id = percent_decode(slot_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                    ("intentId", intent_id.as_str()),
                    ("slotId", slot_id.as_str()),
                ];
                self.handle_delete_slot(state, request, labels, query).await
            }
            // PUT /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/intents/{intentId}/slots/{slotId}
            (
                "PUT",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                    "intents",
                    intent_id,
                    "slots",
                    slot_id,
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let intent_id = percent_decode(intent_id);
                let slot_id = percent_decode(slot_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                    ("intentId", intent_id.as_str()),
                    ("slotId", slot_id.as_str()),
                ];
                self.handle_update_slot(state, request, labels, query).await
            }
            // ---- SlotType routes ----
            // PUT /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/slottypes
            (
                "PUT",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                    "slottypes",
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                ];
                self.handle_create_slot_type(state, request, labels, query)
                    .await
            }
            // POST /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/slottypes
            (
                "POST",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                    "slottypes",
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                ];
                self.handle_list_slot_types(state, request, labels, query)
                    .await
            }
            // GET /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/slottypes/{slotTypeId}
            (
                "GET",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                    "slottypes",
                    slot_type_id,
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let slot_type_id = percent_decode(slot_type_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                    ("slotTypeId", slot_type_id.as_str()),
                ];
                self.handle_describe_slot_type(state, request, labels, query)
                    .await
            }
            // DELETE /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/slottypes/{slotTypeId}
            (
                "DELETE",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                    "slottypes",
                    slot_type_id,
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let slot_type_id = percent_decode(slot_type_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                    ("slotTypeId", slot_type_id.as_str()),
                ];
                self.handle_delete_slot_type(state, request, labels, query)
                    .await
            }
            // PUT /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/slottypes/{slotTypeId}
            (
                "PUT",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                    "slottypes",
                    slot_type_id,
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let slot_type_id = percent_decode(slot_type_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                    ("slotTypeId", slot_type_id.as_str()),
                ];
                self.handle_update_slot_type(state, request, labels, query)
                    .await
            }
            // ---- CustomVocabulary routes ----
            // PUT /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/customvocabulary/DEFAULT/batchcreate
            (
                "PUT",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                    "customvocabulary",
                    "DEFAULT",
                    "batchcreate",
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                ];
                self.handle_batch_create_custom_vocabulary_item(state, request, labels, query)
                    .await
            }
            // POST /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/customvocabulary/DEFAULT/batchdelete
            (
                "POST",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                    "customvocabulary",
                    "DEFAULT",
                    "batchdelete",
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                ];
                self.handle_batch_delete_custom_vocabulary_item(state, request, labels, query)
                    .await
            }
            // PUT /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/customvocabulary/DEFAULT/batchupdate
            (
                "PUT",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                    "customvocabulary",
                    "DEFAULT",
                    "batchupdate",
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                ];
                self.handle_batch_update_custom_vocabulary_item(state, request, labels, query)
                    .await
            }
            // POST /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/customvocabulary/DEFAULT/list
            (
                "POST",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                    "customvocabulary",
                    "DEFAULT",
                    "list",
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                ];
                self.handle_list_custom_vocabulary_items(state, request, labels, query)
                    .await
            }
            // GET /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/customvocabulary/DEFAULT/metadata
            (
                "GET",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                    "customvocabulary",
                    "DEFAULT",
                    "metadata",
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                ];
                self.handle_describe_custom_vocabulary_metadata(state, request, labels, query)
                    .await
            }
            // DELETE /bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/customvocabulary
            (
                "DELETE",
                [
                    "bots",
                    bot_id,
                    "botversions",
                    bot_version,
                    "botlocales",
                    locale_id,
                    "customvocabulary",
                ],
            ) => {
                let bot_id = percent_decode(bot_id);
                let bot_version = percent_decode(bot_version);
                let locale_id = percent_decode(locale_id);
                let labels: &[(&str, &str)] = &[
                    ("botId", bot_id.as_str()),
                    ("botVersion", bot_version.as_str()),
                    ("localeId", locale_id.as_str()),
                ];
                self.handle_delete_custom_vocabulary(state, request, labels, query)
                    .await
            }
            // ---- Export routes ----
            // PUT /exports - CreateExport
            ("PUT", ["exports"]) => self.handle_create_export(state, request, &[], query).await,
            // POST /exports - ListExports
            ("POST", ["exports"]) => self.handle_list_exports(state).await,
            // GET /exports/{exportId} - DescribeExport
            ("GET", ["exports", export_id]) => {
                let export_id = percent_decode(export_id);
                let labels: &[(&str, &str)] = &[("exportId", export_id.as_str())];
                self.handle_describe_export(state, request, labels, query)
                    .await
            }
            // DELETE /exports/{exportId} - DeleteExport
            ("DELETE", ["exports", export_id]) => {
                let export_id = percent_decode(export_id);
                let labels: &[(&str, &str)] = &[("exportId", export_id.as_str())];
                self.handle_delete_export(state, request, labels, query)
                    .await
            }
            // PUT /exports/{exportId} - UpdateExport
            ("PUT", ["exports", export_id]) => {
                let export_id = percent_decode(export_id);
                let labels: &[(&str, &str)] = &[("exportId", export_id.as_str())];
                self.handle_update_export(state, request, labels, query)
                    .await
            }
            // ---- Import routes ----
            // PUT /imports - StartImport
            ("PUT", ["imports"]) => self.handle_start_import(state, request, &[], query).await,
            // POST /imports - ListImports
            ("POST", ["imports"]) => self.handle_list_imports(state).await,
            // GET /imports/{importId} - DescribeImport
            ("GET", ["imports", import_id]) => {
                let import_id = percent_decode(import_id);
                let labels: &[(&str, &str)] = &[("importId", import_id.as_str())];
                self.handle_describe_import(state, request, labels, query)
                    .await
            }
            // DELETE /imports/{importId} - DeleteImport
            ("DELETE", ["imports", import_id]) => {
                let import_id = percent_decode(import_id);
                let labels: &[(&str, &str)] = &[("importId", import_id.as_str())];
                self.handle_delete_import(state, request, labels, query)
                    .await
            }
            // ---- BuiltIn routes ----
            // POST /builtins/locales/{localeId}/intents - ListBuiltInIntents
            ("POST", ["builtins", "locales", locale_id, "intents"]) => {
                let locale_id = percent_decode(locale_id);
                let labels: &[(&str, &str)] = &[("localeId", locale_id.as_str())];
                self.handle_list_built_in_intents(request, labels, query)
                    .await
            }
            // POST /builtins/locales/{localeId}/slottypes - ListBuiltInSlotTypes
            ("POST", ["builtins", "locales", locale_id, "slottypes"]) => {
                let locale_id = percent_decode(locale_id);
                let labels: &[(&str, &str)] = &[("localeId", locale_id.as_str())];
                self.handle_list_built_in_slot_types(request, labels, query)
                    .await
            }
            // ---- Utterances routes ----
            // DELETE /bots/{botId}/utterances - DeleteUtterances
            ("DELETE", ["bots", bot_id, "utterances"]) => {
                let bot_id = percent_decode(bot_id);
                let labels: &[(&str, &str)] = &[("botId", bot_id.as_str())];
                self.handle_delete_utterances(state, request, labels, query)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ---- Bot handlers ----

    // PUT /bots - CreateBot
    async fn handle_create_bot(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_bot_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.bot_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'botName'");
        }
        if input.role_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'roleArn'");
        }
        let child_directed = input.data_privacy.child_directed;
        let ttl = if input.idle_session_t_t_l_in_seconds == 0 {
            300
        } else {
            input.idle_session_t_t_l_in_seconds
        };
        let bot_tags = input.bot_tags;

        let mut st = state.write().await;
        match st.create_bot(
            input.bot_name,
            input.role_arn,
            child_directed,
            ttl,
            input.description,
            bot_tags,
        ) {
            Ok(bot) => wire::serialize_create_bot_response(&model::CreateBotResponse {
                bot_id: Some(bot.bot_id),
                bot_name: Some(bot.bot_name),
                bot_status: Some(bot.bot_status),
                role_arn: Some(bot.role_arn),
                data_privacy: Some(model::DataPrivacy {
                    child_directed: bot.data_privacy_child_directed,
                }),
                idle_session_t_t_l_in_seconds: Some(bot.idle_session_ttl_in_seconds),
                description: bot.description,
                creation_date_time: rfc3339_to_epoch(&bot.creation_date_time),
                ..Default::default()
            }),
            Err(e) => service_error_response(&e),
        }
    }

    // POST /bots - ListBots
    async fn handle_list_bots(&self, state: &Arc<RwLock<LexState>>) -> MockResponse {
        let st = state.read().await;
        let bots = st.list_bots();
        let summaries: Vec<model::BotSummary> = bots
            .into_iter()
            .map(|b| model::BotSummary {
                bot_id: Some(b.bot_id.clone()),
                bot_name: Some(b.bot_name.clone()),
                bot_status: Some(b.bot_status.clone()),
                description: b.description.clone(),
                last_updated_date_time: rfc3339_to_epoch(&b.last_updated_date_time),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_bots_response(&model::ListBotsResponse {
            bot_summaries: Some(summaries),
            ..Default::default()
        })
    }

    // GET /bots/{botId} - DescribeBot
    async fn handle_describe_bot(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_bot_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        match st.describe_bot(&input.bot_id) {
            Ok(bot) => wire::serialize_describe_bot_response(&model::DescribeBotResponse {
                bot_id: Some(bot.bot_id.clone()),
                bot_name: Some(bot.bot_name.clone()),
                bot_status: Some(bot.bot_status.clone()),
                role_arn: Some(bot.role_arn.clone()),
                data_privacy: Some(model::DataPrivacy {
                    child_directed: bot.data_privacy_child_directed,
                }),
                idle_session_t_t_l_in_seconds: Some(bot.idle_session_ttl_in_seconds),
                description: bot.description.clone(),
                creation_date_time: rfc3339_to_epoch(&bot.creation_date_time),
                last_updated_date_time: rfc3339_to_epoch(&bot.last_updated_date_time),
                ..Default::default()
            }),
            Err(e) => service_error_response(&e),
        }
    }

    // DELETE /bots/{botId} - DeleteBot
    async fn handle_delete_bot(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_bot_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.delete_bot(&input.bot_id) {
            Ok(id) => wire::serialize_delete_bot_response(&model::DeleteBotResponse {
                bot_id: Some(id),
                bot_status: Some("Deleting".to_string()),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    // PUT /bots/{botId} - UpdateBot
    async fn handle_update_bot(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_bot_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.bot_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'botName'");
        }
        if input.role_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'roleArn'");
        }
        let child_directed = input.data_privacy.child_directed;
        let ttl = if input.idle_session_t_t_l_in_seconds == 0 {
            300
        } else {
            input.idle_session_t_t_l_in_seconds
        };

        let mut st = state.write().await;
        match st.update_bot(
            &input.bot_id,
            input.bot_name,
            input.role_arn,
            child_directed,
            ttl,
            input.description,
        ) {
            Ok(bot) => wire::serialize_update_bot_response(&model::UpdateBotResponse {
                bot_id: Some(bot.bot_id),
                bot_name: Some(bot.bot_name),
                bot_status: Some(bot.bot_status),
                role_arn: Some(bot.role_arn),
                data_privacy: Some(model::DataPrivacy {
                    child_directed: bot.data_privacy_child_directed,
                }),
                idle_session_t_t_l_in_seconds: Some(bot.idle_session_ttl_in_seconds),
                description: bot.description,
                creation_date_time: rfc3339_to_epoch(&bot.creation_date_time),
                last_updated_date_time: rfc3339_to_epoch(&bot.last_updated_date_time),
                ..Default::default()
            }),
            Err(e) => service_error_response(&e),
        }
    }

    // ---- BotAlias handlers ----

    // PUT /bots/{botId}/botaliases - CreateBotAlias
    async fn handle_create_bot_alias(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_bot_alias_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.bot_alias_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'botAliasName'");
        }

        let mut st = state.write().await;
        match st.create_bot_alias(
            &input.bot_id,
            input.bot_alias_name,
            input.bot_version,
            input.description,
        ) {
            Ok(alias) => {
                wire::serialize_create_bot_alias_response(&model::CreateBotAliasResponse {
                    bot_alias_id: Some(alias.bot_alias_id),
                    bot_alias_name: Some(alias.bot_alias_name),
                    bot_alias_status: Some(alias.bot_alias_status),
                    bot_id: Some(alias.bot_id),
                    bot_version: alias.bot_version,
                    description: alias.description,
                    creation_date_time: rfc3339_to_epoch(&alias.creation_date_time),
                    ..Default::default()
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    // POST /bots/{botId}/botaliases - ListBotAliases
    async fn handle_list_bot_aliases(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_bot_aliases_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        let aliases = st.list_bot_aliases(&input.bot_id);
        let summaries: Vec<model::BotAliasSummary> = aliases
            .into_iter()
            .map(|a| model::BotAliasSummary {
                bot_alias_id: Some(a.bot_alias_id.clone()),
                bot_alias_name: Some(a.bot_alias_name.clone()),
                bot_alias_status: Some(a.bot_alias_status.clone()),
                bot_version: a.bot_version.clone(),
                description: a.description.clone(),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_bot_aliases_response(&model::ListBotAliasesResponse {
            bot_id: Some(input.bot_id),
            bot_alias_summaries: Some(summaries),
            ..Default::default()
        })
    }

    // GET /bots/{botId}/botaliases/{botAliasId} - DescribeBotAlias
    async fn handle_describe_bot_alias(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_bot_alias_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        match st.describe_bot_alias(&input.bot_id, &input.bot_alias_id) {
            Ok(alias) => {
                wire::serialize_describe_bot_alias_response(&model::DescribeBotAliasResponse {
                    bot_alias_id: Some(alias.bot_alias_id.clone()),
                    bot_alias_name: Some(alias.bot_alias_name.clone()),
                    bot_alias_status: Some(alias.bot_alias_status.clone()),
                    bot_id: Some(alias.bot_id.clone()),
                    bot_version: alias.bot_version.clone(),
                    description: alias.description.clone(),
                    creation_date_time: rfc3339_to_epoch(&alias.creation_date_time),
                    last_updated_date_time: rfc3339_to_epoch(&alias.last_updated_date_time),
                    ..Default::default()
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    // DELETE /bots/{botId}/botaliases/{botAliasId} - DeleteBotAlias
    async fn handle_delete_bot_alias(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_bot_alias_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.delete_bot_alias(&input.bot_id, &input.bot_alias_id) {
            Ok((bid, aid)) => {
                wire::serialize_delete_bot_alias_response(&model::DeleteBotAliasResponse {
                    bot_alias_id: Some(aid),
                    bot_alias_status: Some("Deleting".to_string()),
                    bot_id: Some(bid),
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    // PUT /bots/{botId}/botaliases/{botAliasId} - UpdateBotAlias
    async fn handle_update_bot_alias(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_bot_alias_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.bot_alias_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'botAliasName'");
        }

        let mut st = state.write().await;
        match st.update_bot_alias(
            &input.bot_id,
            &input.bot_alias_id,
            input.bot_alias_name,
            input.bot_version,
            input.description,
        ) {
            Ok(alias) => {
                wire::serialize_update_bot_alias_response(&model::UpdateBotAliasResponse {
                    bot_alias_id: Some(alias.bot_alias_id),
                    bot_alias_name: Some(alias.bot_alias_name),
                    bot_alias_status: Some(alias.bot_alias_status),
                    bot_id: Some(alias.bot_id),
                    bot_version: alias.bot_version,
                    description: alias.description,
                    creation_date_time: rfc3339_to_epoch(&alias.creation_date_time),
                    last_updated_date_time: rfc3339_to_epoch(&alias.last_updated_date_time),
                    ..Default::default()
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    // ---- BotLocale handlers ----

    // PUT /bots/{botId}/botversions/{botVersion}/botlocales - CreateBotLocale
    async fn handle_create_bot_locale(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_bot_locale_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.locale_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'localeId'");
        }
        let threshold = if input.nlu_intent_confidence_threshold == 0.0 {
            0.4
        } else {
            input.nlu_intent_confidence_threshold
        };

        let mut st = state.write().await;
        match st.create_bot_locale(&input.bot_id, input.locale_id, threshold, input.description) {
            Ok(locale) => {
                wire::serialize_create_bot_locale_response(&model::CreateBotLocaleResponse {
                    bot_id: Some(locale.bot_id),
                    bot_version: Some(locale.bot_version),
                    locale_id: Some(locale.locale_id),
                    locale_name: Some(locale.locale_name),
                    bot_locale_status: Some(locale.bot_locale_status),
                    nlu_intent_confidence_threshold: Some(locale.nlu_intent_confidence_threshold),
                    description: locale.description,
                    creation_date_time: rfc3339_to_epoch(&locale.creation_date_time),
                    ..Default::default()
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    // PUT /bots/{botId}/botversions/{botVersion}/botlocales/{localeId} - UpdateBotLocale
    async fn handle_update_bot_locale(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_bot_locale_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let threshold = if input.nlu_intent_confidence_threshold == 0.0 {
            0.4
        } else {
            input.nlu_intent_confidence_threshold
        };

        let mut st = state.write().await;
        match st.update_bot_locale(
            &input.bot_id,
            &input.locale_id,
            threshold,
            input.description,
        ) {
            Ok(locale) => {
                wire::serialize_update_bot_locale_response(&model::UpdateBotLocaleResponse {
                    bot_id: Some(locale.bot_id),
                    bot_version: Some(locale.bot_version),
                    locale_id: Some(locale.locale_id),
                    locale_name: Some(locale.locale_name),
                    bot_locale_status: Some(locale.bot_locale_status),
                    nlu_intent_confidence_threshold: Some(locale.nlu_intent_confidence_threshold),
                    description: locale.description,
                    creation_date_time: rfc3339_to_epoch(&locale.creation_date_time),
                    last_updated_date_time: rfc3339_to_epoch(&locale.last_updated_date_time),
                    ..Default::default()
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    // POST /bots/{botId}/botversions/{botVersion}/botlocales - ListBotLocales
    async fn handle_list_bot_locales(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_bot_locales_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        let locales = st.list_bot_locales(&input.bot_id);
        let summaries: Vec<model::BotLocaleSummary> = locales
            .into_iter()
            .map(|l| model::BotLocaleSummary {
                locale_id: Some(l.locale_id.clone()),
                locale_name: Some(l.locale_name.clone()),
                bot_locale_status: Some(l.bot_locale_status.clone()),
                description: l.description.clone(),
                last_updated_date_time: rfc3339_to_epoch(&l.last_updated_date_time),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_bot_locales_response(&model::ListBotLocalesResponse {
            bot_id: Some(input.bot_id),
            bot_version: Some("DRAFT".to_string()),
            bot_locale_summaries: Some(summaries),
            ..Default::default()
        })
    }

    // GET /bots/{botId}/botlocales/{localeId} - DescribeBotLocale
    async fn handle_describe_bot_locale(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_bot_locale_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        match st.describe_bot_locale(&input.bot_id, &input.locale_id) {
            Ok(locale) => {
                wire::serialize_describe_bot_locale_response(&model::DescribeBotLocaleResponse {
                    bot_id: Some(locale.bot_id.clone()),
                    bot_version: Some(locale.bot_version.clone()),
                    locale_id: Some(locale.locale_id.clone()),
                    locale_name: Some(locale.locale_name.clone()),
                    bot_locale_status: Some(locale.bot_locale_status.clone()),
                    nlu_intent_confidence_threshold: Some(locale.nlu_intent_confidence_threshold),
                    description: locale.description.clone(),
                    creation_date_time: rfc3339_to_epoch(&locale.creation_date_time),
                    last_updated_date_time: rfc3339_to_epoch(&locale.last_updated_date_time),
                    ..Default::default()
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    // DELETE /bots/{botId}/botlocales/{localeId} - DeleteBotLocale
    async fn handle_delete_bot_locale(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_bot_locale_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.delete_bot_locale(&input.bot_id, &input.locale_id) {
            Ok((bid, lid)) => {
                wire::serialize_delete_bot_locale_response(&model::DeleteBotLocaleResponse {
                    bot_id: Some(bid),
                    bot_version: Some("DRAFT".to_string()),
                    locale_id: Some(lid),
                    bot_locale_status: Some("Deleting".to_string()),
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    // POST /bots/{botId}/botlocales/{localeId}/build - BuildBotLocale
    async fn handle_build_bot_locale(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_build_bot_locale_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.build_bot_locale(&input.bot_id, &input.locale_id) {
            Ok((bid, ver, lid)) => {
                wire::serialize_build_bot_locale_response(&model::BuildBotLocaleResponse {
                    bot_id: Some(bid),
                    bot_version: Some(ver),
                    locale_id: Some(lid),
                    bot_locale_status: Some("Built".to_string()),
                    ..Default::default()
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    // ---- Intent handlers ----

    // POST /bots/{botId}/botlocales/{localeId}/intents - CreateIntent
    async fn handle_create_intent(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_intent_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.intent_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'intentName'");
        }

        let mut st = state.write().await;
        match st.create_intent(
            &input.bot_id,
            &input.locale_id,
            input.intent_name,
            input.description,
        ) {
            Ok(intent) => wire::serialize_create_intent_response(&model::CreateIntentResponse {
                intent_id: Some(intent.intent_id),
                intent_name: Some(intent.intent_name),
                bot_id: Some(intent.bot_id),
                bot_version: Some(intent.bot_version),
                locale_id: Some(intent.locale_id),
                description: intent.description,
                creation_date_time: rfc3339_to_epoch(&intent.creation_date_time),
                ..Default::default()
            }),
            Err(e) => service_error_response(&e),
        }
    }

    // GET /bots/{botId}/botlocales/{localeId}/intents - ListIntents
    async fn handle_list_intents(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_intents_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        let intents = st.list_intents(&input.bot_id, &input.locale_id);
        let summaries: Vec<model::IntentSummary> = intents
            .into_iter()
            .map(|i| model::IntentSummary {
                intent_id: Some(i.intent_id.clone()),
                intent_name: Some(i.intent_name.clone()),
                description: i.description.clone(),
                last_updated_date_time: rfc3339_to_epoch(&i.last_updated_date_time),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_intents_response(&model::ListIntentsResponse {
            bot_id: Some(input.bot_id),
            bot_version: Some("DRAFT".to_string()),
            locale_id: Some(input.locale_id),
            intent_summaries: Some(summaries),
            ..Default::default()
        })
    }

    // GET /bots/{botId}/botlocales/{localeId}/intents/{intentId} - DescribeIntent
    async fn handle_describe_intent(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_intent_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        match st.describe_intent(&input.bot_id, &input.locale_id, &input.intent_id) {
            Ok(intent) => {
                wire::serialize_describe_intent_response(&model::DescribeIntentResponse {
                    intent_id: Some(intent.intent_id.clone()),
                    intent_name: Some(intent.intent_name.clone()),
                    bot_id: Some(intent.bot_id.clone()),
                    bot_version: Some(intent.bot_version.clone()),
                    locale_id: Some(intent.locale_id.clone()),
                    description: intent.description.clone(),
                    creation_date_time: rfc3339_to_epoch(&intent.creation_date_time),
                    last_updated_date_time: rfc3339_to_epoch(&intent.last_updated_date_time),
                    ..Default::default()
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    // DELETE /bots/{botId}/botlocales/{localeId}/intents/{intentId} - DeleteIntent
    async fn handle_delete_intent(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_intent_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.delete_intent(&input.bot_id, &input.locale_id, &input.intent_id) {
            Ok(()) => wire::serialize_delete_intent_response(),
            Err(e) => service_error_response(&e),
        }
    }

    // PUT /bots/{botId}/botlocales/{localeId}/intents/{intentId} - UpdateIntent
    async fn handle_update_intent(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_intent_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.intent_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'intentName'");
        }

        let mut st = state.write().await;
        match st.update_intent(
            &input.bot_id,
            &input.locale_id,
            &input.intent_id,
            input.intent_name,
            input.description,
        ) {
            Ok(intent) => wire::serialize_update_intent_response(&model::UpdateIntentResponse {
                intent_id: Some(intent.intent_id),
                intent_name: Some(intent.intent_name),
                bot_id: Some(intent.bot_id),
                bot_version: Some(intent.bot_version),
                locale_id: Some(intent.locale_id),
                description: intent.description,
                creation_date_time: rfc3339_to_epoch(&intent.creation_date_time),
                last_updated_date_time: rfc3339_to_epoch(&intent.last_updated_date_time),
                ..Default::default()
            }),
            Err(e) => service_error_response(&e),
        }
    }

    // ---- BotVersion handlers ----

    // POST /bots/{botId}/botversions - CreateBotVersion
    async fn handle_create_bot_version(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_bot_version_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };

        let mut st = state.write().await;
        match st.create_bot_version(&input.bot_id, input.description) {
            Ok(bv) => {
                wire::serialize_create_bot_version_response(&model::CreateBotVersionResponse {
                    bot_id: Some(bv.bot_id),
                    bot_version: Some(bv.bot_version),
                    bot_status: Some(bv.bot_status),
                    description: bv.description,
                    creation_date_time: rfc3339_to_epoch(&bv.creation_date_time),
                    ..Default::default()
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    // GET /bots/{botId}/botversions - ListBotVersions
    async fn handle_list_bot_versions(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_bot_versions_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        let versions = st.list_bot_versions(&input.bot_id);
        let summaries: Vec<model::BotVersionSummary> = versions
            .into_iter()
            .map(|v| model::BotVersionSummary {
                bot_version: Some(v.bot_version.clone()),
                bot_status: Some(v.bot_status.clone()),
                description: v.description.clone(),
                creation_date_time: rfc3339_to_epoch(&v.creation_date_time),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_bot_versions_response(&model::ListBotVersionsResponse {
            bot_id: Some(input.bot_id),
            bot_version_summaries: Some(summaries),
            ..Default::default()
        })
    }

    // GET /bots/{botId}/botversions/{botVersion} - DescribeBotVersion
    async fn handle_describe_bot_version(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_bot_version_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        match st.describe_bot_version(&input.bot_id, &input.bot_version) {
            Ok(bv) => {
                wire::serialize_describe_bot_version_response(&model::DescribeBotVersionResponse {
                    bot_id: Some(bv.bot_id.clone()),
                    bot_version: Some(bv.bot_version.clone()),
                    bot_status: Some(bv.bot_status.clone()),
                    description: bv.description.clone(),
                    creation_date_time: rfc3339_to_epoch(&bv.creation_date_time),
                    ..Default::default()
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    // DELETE /bots/{botId}/botversions/{botVersion} - DeleteBotVersion
    async fn handle_delete_bot_version(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_bot_version_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.delete_bot_version(&input.bot_id, &input.bot_version) {
            Ok((bid, ver)) => {
                wire::serialize_delete_bot_version_response(&model::DeleteBotVersionResponse {
                    bot_id: Some(bid),
                    bot_version: Some(ver),
                    bot_status: Some("Deleting".to_string()),
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    // ---- Tag handlers ----

    // GET /tags/{resourceARN} - ListTagsForResource
    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        let tags = st.list_tags_for_resource(&input.resource_a_r_n);
        let tags_opt = if tags.is_empty() { None } else { Some(tags) };
        wire::serialize_list_tags_for_resource_response(&model::ListTagsForResourceResponse {
            tags: tags_opt,
        })
    }

    // POST /tags/{resourceARN} - TagResource
    async fn handle_tag_resource(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };

        let mut st = state.write().await;
        match st.tag_resource(&input.resource_a_r_n, input.tags) {
            Ok(()) => wire::serialize_tag_resource_response(&model::TagResourceResponse {}),
            Err(e) => service_error_response(&e),
        }
    }

    // DELETE /tags/{resourceARN} - UntagResource
    async fn handle_untag_resource(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };

        let mut st = state.write().await;
        match st.untag_resource(&input.resource_a_r_n, input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&model::UntagResourceResponse {}),
            Err(e) => service_error_response(&e),
        }
    }

    // ---- Resource Policy handlers ----

    // PUT /policy/{resourceArn} (create) - CreateResourcePolicy
    async fn handle_create_resource_policy(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_resource_policy_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.policy.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'policy'");
        }
        let mut st = state.write().await;
        match st.create_resource_policy(&input.resource_arn, input.policy) {
            Ok(rp) => wire::serialize_create_resource_policy_response(
                &model::CreateResourcePolicyResponse {
                    resource_arn: Some(rp.resource_arn),
                    revision_id: Some(rp.revision_id),
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    // GET /policy/{resourceArn} - DescribeResourcePolicy
    async fn handle_describe_resource_policy(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_resource_policy_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        match st.describe_resource_policy(&input.resource_arn) {
            Ok(rp) => wire::serialize_describe_resource_policy_response(
                &model::DescribeResourcePolicyResponse {
                    resource_arn: Some(rp.resource_arn.clone()),
                    policy: Some(rp.policy.clone()),
                    revision_id: Some(rp.revision_id.clone()),
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    // PUT /policy/{resourceArn} (update) - UpdateResourcePolicy
    async fn handle_update_resource_policy(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_resource_policy_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.policy.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'policy'");
        }
        let mut st = state.write().await;
        match st.update_resource_policy(&input.resource_arn, input.policy) {
            Ok(rp) => wire::serialize_update_resource_policy_response(
                &model::UpdateResourcePolicyResponse {
                    resource_arn: Some(rp.resource_arn),
                    revision_id: Some(rp.revision_id),
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    // DELETE /policy/{resourceArn} - DeleteResourcePolicy
    async fn handle_delete_resource_policy(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_resource_policy_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.delete_resource_policy(&input.resource_arn) {
            Ok(rp) => wire::serialize_delete_resource_policy_response(
                &model::DeleteResourcePolicyResponse {
                    resource_arn: Some(rp.resource_arn),
                    revision_id: Some(rp.revision_id),
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    // ---- Slot handlers ----

    async fn handle_create_slot(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_slot_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.slot_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'slotName'");
        }

        let mut st = state.write().await;
        match st.create_slot(
            &input.bot_id,
            &input.locale_id,
            &input.intent_id,
            input.slot_name,
            input.slot_type_id,
            input.description,
            input.value_elicitation_setting,
            input.multiple_values_setting,
            input.obfuscation_setting,
            input.sub_slot_setting,
        ) {
            Ok(slot) => wire::serialize_create_slot_response(&model::CreateSlotResponse {
                slot_id: Some(slot.slot_id),
                slot_name: Some(slot.slot_name),
                bot_id: Some(slot.bot_id),
                bot_version: Some(slot.bot_version),
                locale_id: Some(slot.locale_id),
                intent_id: Some(slot.intent_id),
                slot_type_id: slot.slot_type_id,
                description: slot.description,
                value_elicitation_setting: Some(slot.value_elicitation_setting),
                multiple_values_setting: slot.multiple_values_setting,
                obfuscation_setting: slot.obfuscation_setting,
                sub_slot_setting: slot.sub_slot_setting,
                creation_date_time: rfc3339_to_epoch(&slot.creation_date_time),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_describe_slot(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_slot_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        match st.describe_slot(
            &input.bot_id,
            &input.locale_id,
            &input.intent_id,
            &input.slot_id,
        ) {
            Ok(slot) => wire::serialize_describe_slot_response(&model::DescribeSlotResponse {
                slot_id: Some(slot.slot_id.clone()),
                slot_name: Some(slot.slot_name.clone()),
                bot_id: Some(slot.bot_id.clone()),
                bot_version: Some(slot.bot_version.clone()),
                locale_id: Some(slot.locale_id.clone()),
                intent_id: Some(slot.intent_id.clone()),
                slot_type_id: slot.slot_type_id.clone(),
                description: slot.description.clone(),
                value_elicitation_setting: Some(slot.value_elicitation_setting.clone()),
                multiple_values_setting: slot.multiple_values_setting.clone(),
                obfuscation_setting: slot.obfuscation_setting.clone(),
                sub_slot_setting: slot.sub_slot_setting.clone(),
                creation_date_time: rfc3339_to_epoch(&slot.creation_date_time),
                last_updated_date_time: rfc3339_to_epoch(&slot.last_updated_date_time),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_slots(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_slots_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        let slots = st.list_slots(&input.bot_id, &input.locale_id, &input.intent_id);
        let summaries: Vec<model::SlotSummary> = slots
            .into_iter()
            .map(|s| model::SlotSummary {
                slot_id: Some(s.slot_id.clone()),
                slot_name: Some(s.slot_name.clone()),
                slot_type_id: s.slot_type_id.clone(),
                description: s.description.clone(),
                last_updated_date_time: rfc3339_to_epoch(&s.last_updated_date_time),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_slots_response(&model::ListSlotsResponse {
            bot_id: Some(input.bot_id),
            bot_version: Some("DRAFT".to_string()),
            locale_id: Some(input.locale_id),
            intent_id: Some(input.intent_id),
            slot_summaries: Some(summaries),
            ..Default::default()
        })
    }

    async fn handle_delete_slot(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_slot_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.delete_slot(
            &input.bot_id,
            &input.locale_id,
            &input.intent_id,
            &input.slot_id,
        ) {
            Ok(()) => wire::serialize_delete_slot_response(),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_update_slot(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_slot_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.slot_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'slotName'");
        }

        let mut st = state.write().await;
        match st.update_slot(
            &input.bot_id,
            &input.locale_id,
            &input.intent_id,
            &input.slot_id,
            input.slot_name,
            input.slot_type_id,
            input.description,
            input.value_elicitation_setting,
            input.multiple_values_setting,
            input.obfuscation_setting,
            input.sub_slot_setting,
        ) {
            Ok(slot) => wire::serialize_update_slot_response(&model::UpdateSlotResponse {
                slot_id: Some(slot.slot_id),
                slot_name: Some(slot.slot_name),
                bot_id: Some(slot.bot_id),
                bot_version: Some(slot.bot_version),
                locale_id: Some(slot.locale_id),
                intent_id: Some(slot.intent_id),
                slot_type_id: slot.slot_type_id,
                description: slot.description,
                value_elicitation_setting: Some(slot.value_elicitation_setting),
                multiple_values_setting: slot.multiple_values_setting,
                obfuscation_setting: slot.obfuscation_setting,
                sub_slot_setting: slot.sub_slot_setting,
                creation_date_time: rfc3339_to_epoch(&slot.creation_date_time),
                last_updated_date_time: rfc3339_to_epoch(&slot.last_updated_date_time),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    // ---- SlotType handlers ----

    async fn handle_create_slot_type(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_slot_type_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.slot_type_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'slotTypeName'");
        }

        let mut st = state.write().await;
        match st.create_slot_type(
            &input.bot_id,
            &input.locale_id,
            input.slot_type_name,
            input.description,
            input.parent_slot_type_signature,
            input.slot_type_values,
            input.value_selection_setting,
            input.external_source_setting,
            input.composite_slot_type_setting,
        ) {
            Ok(stype) => {
                wire::serialize_create_slot_type_response(&model::CreateSlotTypeResponse {
                    slot_type_id: Some(stype.slot_type_id),
                    slot_type_name: Some(stype.slot_type_name),
                    bot_id: Some(stype.bot_id),
                    bot_version: Some(stype.bot_version),
                    locale_id: Some(stype.locale_id),
                    description: stype.description,
                    parent_slot_type_signature: stype.parent_slot_type_signature,
                    slot_type_values: stype.slot_type_values,
                    value_selection_setting: stype.value_selection_setting,
                    external_source_setting: stype.external_source_setting,
                    composite_slot_type_setting: stype.composite_slot_type_setting,
                    creation_date_time: rfc3339_to_epoch(&stype.creation_date_time),
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_describe_slot_type(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_slot_type_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        match st.describe_slot_type(&input.bot_id, &input.locale_id, &input.slot_type_id) {
            Ok(stype) => {
                wire::serialize_describe_slot_type_response(&model::DescribeSlotTypeResponse {
                    slot_type_id: Some(stype.slot_type_id.clone()),
                    slot_type_name: Some(stype.slot_type_name.clone()),
                    bot_id: Some(stype.bot_id.clone()),
                    bot_version: Some(stype.bot_version.clone()),
                    locale_id: Some(stype.locale_id.clone()),
                    description: stype.description.clone(),
                    parent_slot_type_signature: stype.parent_slot_type_signature.clone(),
                    slot_type_values: stype.slot_type_values.clone(),
                    value_selection_setting: stype.value_selection_setting.clone(),
                    external_source_setting: stype.external_source_setting.clone(),
                    composite_slot_type_setting: stype.composite_slot_type_setting.clone(),
                    creation_date_time: rfc3339_to_epoch(&stype.creation_date_time),
                    last_updated_date_time: rfc3339_to_epoch(&stype.last_updated_date_time),
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_slot_types(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_slot_types_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        let types = st.list_slot_types(&input.bot_id, &input.locale_id);
        let summaries: Vec<model::SlotTypeSummary> = types
            .into_iter()
            .map(|s| model::SlotTypeSummary {
                slot_type_id: Some(s.slot_type_id.clone()),
                slot_type_name: Some(s.slot_type_name.clone()),
                description: s.description.clone(),
                parent_slot_type_signature: s.parent_slot_type_signature.clone(),
                last_updated_date_time: rfc3339_to_epoch(&s.last_updated_date_time),
                slot_type_category: Some("Custom".to_string()),
            })
            .collect();
        wire::serialize_list_slot_types_response(&model::ListSlotTypesResponse {
            bot_id: Some(input.bot_id),
            bot_version: Some("DRAFT".to_string()),
            locale_id: Some(input.locale_id),
            slot_type_summaries: Some(summaries),
            ..Default::default()
        })
    }

    async fn handle_delete_slot_type(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_slot_type_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.delete_slot_type(&input.bot_id, &input.locale_id, &input.slot_type_id) {
            Ok(()) => wire::serialize_delete_slot_type_response(),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_update_slot_type(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_slot_type_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.slot_type_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'slotTypeName'");
        }

        let mut st = state.write().await;
        match st.update_slot_type(
            &input.bot_id,
            &input.locale_id,
            &input.slot_type_id,
            input.slot_type_name,
            input.description,
            input.parent_slot_type_signature,
            input.slot_type_values,
            input.value_selection_setting,
            input.external_source_setting,
            input.composite_slot_type_setting,
        ) {
            Ok(stype) => {
                wire::serialize_update_slot_type_response(&model::UpdateSlotTypeResponse {
                    slot_type_id: Some(stype.slot_type_id),
                    slot_type_name: Some(stype.slot_type_name),
                    bot_id: Some(stype.bot_id),
                    bot_version: Some(stype.bot_version),
                    locale_id: Some(stype.locale_id),
                    description: stype.description,
                    parent_slot_type_signature: stype.parent_slot_type_signature,
                    slot_type_values: stype.slot_type_values,
                    value_selection_setting: stype.value_selection_setting,
                    external_source_setting: stype.external_source_setting,
                    composite_slot_type_setting: stype.composite_slot_type_setting,
                    creation_date_time: rfc3339_to_epoch(&stype.creation_date_time),
                    last_updated_date_time: rfc3339_to_epoch(&stype.last_updated_date_time),
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    // ---- CustomVocabulary handlers ----

    async fn handle_batch_create_custom_vocabulary_item(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_create_custom_vocabulary_item_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // The state operates on `model::CustomVocabularyItem` (with `item_id`); the
        // create-item shape lacks `item_id` so we synthesize empty strings (the state
        // assigns UUIDs when the field is empty).
        let items: Vec<model::CustomVocabularyItem> = input
            .custom_vocabulary_item_list
            .into_iter()
            .map(|n| model::CustomVocabularyItem {
                item_id: String::new(),
                phrase: n.phrase,
                display_as: n.display_as,
                weight: n.weight,
            })
            .collect();

        let mut st = state.write().await;
        match st.batch_create_custom_vocabulary_items(&input.bot_id, &input.locale_id, items) {
            Ok(created) => wire::serialize_batch_create_custom_vocabulary_item_response(
                &model::BatchCreateCustomVocabularyItemResponse {
                    bot_id: Some(input.bot_id),
                    bot_version: Some("DRAFT".to_string()),
                    locale_id: Some(input.locale_id),
                    resources: Some(created),
                    ..Default::default()
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_batch_delete_custom_vocabulary_item(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_delete_custom_vocabulary_item_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let item_ids: Vec<String> = input
            .custom_vocabulary_item_list
            .into_iter()
            .map(|i| i.item_id)
            .collect();

        let mut st = state.write().await;
        st.batch_delete_custom_vocabulary_items(&input.bot_id, &input.locale_id, item_ids);
        wire::serialize_batch_delete_custom_vocabulary_item_response(
            &model::BatchDeleteCustomVocabularyItemResponse {
                bot_id: Some(input.bot_id),
                bot_version: Some("DRAFT".to_string()),
                locale_id: Some(input.locale_id),
                ..Default::default()
            },
        )
    }

    async fn handle_batch_update_custom_vocabulary_item(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_update_custom_vocabulary_item_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };

        let mut st = state.write().await;
        match st.batch_update_custom_vocabulary_items(
            &input.bot_id,
            &input.locale_id,
            input.custom_vocabulary_item_list,
        ) {
            Ok(updated) => wire::serialize_batch_update_custom_vocabulary_item_response(
                &model::BatchUpdateCustomVocabularyItemResponse {
                    bot_id: Some(input.bot_id),
                    bot_version: Some("DRAFT".to_string()),
                    locale_id: Some(input.locale_id),
                    resources: Some(updated),
                    ..Default::default()
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_custom_vocabulary_items(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_list_custom_vocabulary_items_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let st = state.read().await;
        let items = st.list_custom_vocabulary_items(&input.bot_id, &input.locale_id);
        let vocab_items: Vec<model::CustomVocabularyItem> = items
            .into_iter()
            .map(|e| model::CustomVocabularyItem {
                item_id: e.item_id.clone(),
                phrase: e.phrase.clone(),
                display_as: e.display_as.clone(),
                weight: e.weight,
            })
            .collect();
        wire::serialize_list_custom_vocabulary_items_response(
            &model::ListCustomVocabularyItemsResponse {
                bot_id: Some(input.bot_id),
                bot_version: Some("DRAFT".to_string()),
                locale_id: Some(input.locale_id),
                custom_vocabulary_items: Some(vocab_items),
                ..Default::default()
            },
        )
    }

    async fn handle_describe_custom_vocabulary_metadata(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_custom_vocabulary_metadata_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        match st.describe_custom_vocabulary_metadata(&input.bot_id, &input.locale_id) {
            Ok(meta) => wire::serialize_describe_custom_vocabulary_metadata_response(
                &model::DescribeCustomVocabularyMetadataResponse {
                    bot_id: Some(meta.bot_id.clone()),
                    bot_version: Some(meta.bot_version.clone()),
                    locale_id: Some(meta.locale_id.clone()),
                    custom_vocabulary_status: Some(meta.status.clone()),
                    creation_date_time: rfc3339_to_epoch(&meta.creation_date_time),
                    last_updated_date_time: rfc3339_to_epoch(&meta.last_updated_date_time),
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_delete_custom_vocabulary(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_custom_vocabulary_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.delete_custom_vocabulary(&input.bot_id, &input.locale_id) {
            Ok(()) => wire::serialize_delete_custom_vocabulary_response(
                &model::DeleteCustomVocabularyResponse {
                    bot_id: Some(input.bot_id),
                    bot_version: Some("DRAFT".to_string()),
                    locale_id: Some(input.locale_id),
                    custom_vocabulary_status: Some("Deleting".to_string()),
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    // ---- Export handlers ----

    async fn handle_create_export(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_export_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let file_format = if input.file_format.is_empty() {
            "LexJson".to_string()
        } else {
            input.file_format
        };

        let mut st = state.write().await;
        match st.create_export(file_format, Some(input.resource_specification)) {
            Ok(job) => wire::serialize_create_export_response(&model::CreateExportResponse {
                export_id: Some(job.export_id),
                file_format: Some(job.file_format),
                export_status: Some(job.export_status),
                resource_specification: job.resource_specification,
                creation_date_time: rfc3339_to_epoch(&job.creation_date_time),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_describe_export(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_export_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        match st.describe_export(&input.export_id) {
            Ok(job) => wire::serialize_describe_export_response(&model::DescribeExportResponse {
                export_id: Some(job.export_id.clone()),
                file_format: Some(job.file_format.clone()),
                export_status: Some(job.export_status.clone()),
                resource_specification: job.resource_specification.clone(),
                creation_date_time: rfc3339_to_epoch(&job.creation_date_time),
                last_updated_date_time: rfc3339_to_epoch(&job.last_updated_date_time),
                ..Default::default()
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_delete_export(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_export_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.delete_export(&input.export_id) {
            Ok(job) => wire::serialize_delete_export_response(&model::DeleteExportResponse {
                export_id: Some(job.export_id),
                export_status: Some("Deleting".to_string()),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_exports(&self, state: &Arc<RwLock<LexState>>) -> MockResponse {
        let st = state.read().await;
        let exports = st.list_exports();
        let summaries: Vec<model::ExportSummary> = exports
            .into_iter()
            .map(|j| model::ExportSummary {
                export_id: Some(j.export_id.clone()),
                file_format: Some(j.file_format.clone()),
                export_status: Some(j.export_status.clone()),
                resource_specification: j.resource_specification.clone(),
                creation_date_time: rfc3339_to_epoch(&j.creation_date_time),
                last_updated_date_time: rfc3339_to_epoch(&j.last_updated_date_time),
            })
            .collect();
        wire::serialize_list_exports_response(&model::ListExportsResponse {
            export_summaries: Some(summaries),
            ..Default::default()
        })
    }

    async fn handle_update_export(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_export_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // The Smithy model for UpdateExport carries only `exportId` and
        // `filePassword`, but the historical handler accepts a flat body containing
        // `fileFormat`. Re-parse the body for that field, mirroring the apigateway
        // pattern.
        let file_format = serde_json::from_slice::<serde_json::Value>(&request.body)
            .ok()
            .and_then(|v| {
                v.get("fileFormat")
                    .and_then(|f| f.as_str())
                    .map(str::to_string)
            })
            .unwrap_or_else(|| "LexJson".to_string());

        let mut st = state.write().await;
        match st.update_export(&input.export_id, file_format) {
            Ok(job) => wire::serialize_update_export_response(&model::UpdateExportResponse {
                export_id: Some(job.export_id),
                file_format: Some(job.file_format),
                export_status: Some(job.export_status),
                resource_specification: job.resource_specification,
                creation_date_time: rfc3339_to_epoch(&job.creation_date_time),
                last_updated_date_time: rfc3339_to_epoch(&job.last_updated_date_time),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    // ---- Import handlers ----

    async fn handle_start_import(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_start_import_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let merge_strategy = if input.merge_strategy.is_empty() {
            "FailOnConflict".to_string()
        } else {
            input.merge_strategy
        };

        let mut st = state.write().await;
        match st.start_import(
            input.import_id,
            merge_strategy,
            Some(input.resource_specification),
        ) {
            Ok(job) => wire::serialize_start_import_response(&model::StartImportResponse {
                import_id: Some(job.import_id),
                import_status: Some(job.import_status),
                merge_strategy: Some(job.merge_strategy),
                resource_specification: job.resource_specification,
                creation_date_time: rfc3339_to_epoch(&job.creation_date_time),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_describe_import(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_import_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let st = state.read().await;
        match st.describe_import(&input.import_id) {
            Ok(job) => wire::serialize_describe_import_response(&model::DescribeImportResponse {
                import_id: Some(job.import_id.clone()),
                import_status: Some(job.import_status.clone()),
                merge_strategy: Some(job.merge_strategy.clone()),
                resource_specification: job.resource_specification.clone(),
                creation_date_time: rfc3339_to_epoch(&job.creation_date_time),
                last_updated_date_time: rfc3339_to_epoch(&job.last_updated_date_time),
                imported_resource_id: job.imported_resource_id.clone(),
                imported_resource_name: job.imported_resource_name.clone(),
                ..Default::default()
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_delete_import(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_import_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.delete_import(&input.import_id) {
            Ok(job) => wire::serialize_delete_import_response(&model::DeleteImportResponse {
                import_id: Some(job.import_id),
                import_status: Some("Deleting".to_string()),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_imports(&self, state: &Arc<RwLock<LexState>>) -> MockResponse {
        let st = state.read().await;
        let imports = st.list_imports();
        let summaries: Vec<model::ImportSummary> = imports
            .into_iter()
            .map(|j| model::ImportSummary {
                import_id: Some(j.import_id.clone()),
                import_status: Some(j.import_status.clone()),
                merge_strategy: Some(j.merge_strategy.clone()),
                imported_resource_id: j.imported_resource_id.clone(),
                imported_resource_name: j.imported_resource_name.clone(),
                creation_date_time: rfc3339_to_epoch(&j.creation_date_time),
                last_updated_date_time: rfc3339_to_epoch(&j.last_updated_date_time),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_imports_response(&model::ListImportsResponse {
            import_summaries: Some(summaries),
            ..Default::default()
        })
    }

    // ---- BuiltIn handlers ----

    // STUB[no-engine]: Built-in intents are defined by the Lex service corpus; the mock
    //   returns a small fixed set of well-known AMAZON.* signatures.
    async fn handle_list_built_in_intents(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_built_in_intents_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let summaries = vec![
            model::BuiltInIntentSummary {
                intent_signature: Some("AMAZON.CancelIntent".to_string()),
                description: Some("Cancel the current action.".to_string()),
            },
            model::BuiltInIntentSummary {
                intent_signature: Some("AMAZON.HelpIntent".to_string()),
                description: Some("Provide help about how to use the application.".to_string()),
            },
            model::BuiltInIntentSummary {
                intent_signature: Some("AMAZON.StopIntent".to_string()),
                description: Some("Stop the current action.".to_string()),
            },
            model::BuiltInIntentSummary {
                intent_signature: Some("AMAZON.FallbackIntent".to_string()),
                description: Some("Fallback when no other intents match.".to_string()),
            },
        ];
        wire::serialize_list_built_in_intents_response(&model::ListBuiltInIntentsResponse {
            locale_id: Some(input.locale_id),
            built_in_intent_summaries: Some(summaries),
            ..Default::default()
        })
    }

    // STUB[no-engine]: Built-in slot types are defined by the Lex service corpus; the mock
    //   returns a small fixed set of well-known AMAZON.* signatures.
    async fn handle_list_built_in_slot_types(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_built_in_slot_types_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let summaries = vec![
            model::BuiltInSlotTypeSummary {
                slot_type_signature: Some("AMAZON.AlphaNumeric".to_string()),
                description: Some("Alphanumeric values.".to_string()),
            },
            model::BuiltInSlotTypeSummary {
                slot_type_signature: Some("AMAZON.Date".to_string()),
                description: Some("Date values.".to_string()),
            },
            model::BuiltInSlotTypeSummary {
                slot_type_signature: Some("AMAZON.Number".to_string()),
                description: Some("Numeric values.".to_string()),
            },
            model::BuiltInSlotTypeSummary {
                slot_type_signature: Some("AMAZON.Time".to_string()),
                description: Some("Time values.".to_string()),
            },
        ];
        wire::serialize_list_built_in_slot_types_response(&model::ListBuiltInSlotTypesResponse {
            locale_id: Some(input.locale_id),
            built_in_slot_type_summaries: Some(summaries),
            ..Default::default()
        })
    }

    // ---- Utterances handler ----

    async fn handle_delete_utterances(
        &self,
        state: &Arc<RwLock<LexState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_utterances_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut st = state.write().await;
        match st.delete_utterances(&input.bot_id) {
            Ok(()) => {
                wire::serialize_delete_utterances_response(&model::DeleteUtterancesResponse {})
            }
            Err(e) => service_error_response(&e),
        }
    }
}
