use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    extract_path, extract_query_string, parse_query_string, percent_decode, rest_json_error,
};

use crate::model;
use crate::state::{AmplifyUiBuilderError, AmplifyUiBuilderState};
use crate::views::AmplifyUiBuilderStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct AmplifyUiBuilderService {
    pub(crate) state: Arc<BackendState<AmplifyUiBuilderState>>,
    pub(crate) notifier: StateChangeNotifier<AmplifyUiBuilderStateView>,
}

impl AmplifyUiBuilderService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for AmplifyUiBuilderService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for AmplifyUiBuilderService {
    fn service_name(&self) -> &str {
        "amplifyuibuilder"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://amplifyuibuilder\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<AmplifyUiBuilderState>>;

impl AmplifyUiBuilderService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str().to_uppercase();

        let body: Value = if request.body.is_empty() {
            json!({})
        } else {
            match serde_json::from_slice(&request.body) {
                Ok(v) => v,
                Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
            }
        };

        let segments: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(percent_decode)
            .collect();
        let segs: Vec<&str> = segments.iter().map(|s| s.as_str()).collect();

        let response = match (method.as_str(), segs.as_slice()) {
            ("POST", ["app", app_id, "environment", env, "components"]) => {
                self.handle_create_component(&state, app_id, env, &body)
                    .await
            }
            ("GET", ["app", app_id, "environment", env, "components"]) => {
                self.handle_list_components(&state, app_id, env, &request.uri)
                    .await
            }
            ("GET", ["app", app_id, "environment", env, "components", id]) => {
                self.handle_get_component(&state, app_id, env, id).await
            }
            ("PATCH", ["app", app_id, "environment", env, "components", id]) => {
                self.handle_update_component(&state, app_id, env, id, &body)
                    .await
            }
            ("DELETE", ["app", app_id, "environment", env, "components", id]) => {
                self.handle_delete_component(&state, app_id, env, id).await
            }
            ("POST", ["app", app_id, "environment", env, "forms"]) => {
                self.handle_create_form(&state, app_id, env, &body).await
            }
            ("GET", ["app", app_id, "environment", env, "forms"]) => {
                self.handle_list_forms(&state, app_id, env).await
            }
            ("GET", ["app", app_id, "environment", env, "forms", id]) => {
                self.handle_get_form(&state, app_id, env, id).await
            }
            ("PATCH", ["app", app_id, "environment", env, "forms", id]) => {
                self.handle_update_form(&state, app_id, env, id, &body)
                    .await
            }
            ("DELETE", ["app", app_id, "environment", env, "forms", id]) => {
                self.handle_delete_form(&state, app_id, env, id).await
            }
            ("POST", ["app", app_id, "environment", env, "themes"]) => {
                self.handle_create_theme(&state, app_id, env, &body).await
            }
            ("GET", ["app", app_id, "environment", env, "themes"]) => {
                self.handle_list_themes(&state, app_id, env).await
            }
            ("GET", ["app", app_id, "environment", env, "themes", id]) => {
                self.handle_get_theme(&state, app_id, env, id).await
            }
            ("PATCH", ["app", app_id, "environment", env, "themes", id]) => {
                self.handle_update_theme(&state, app_id, env, id, &body)
                    .await
            }
            ("DELETE", ["app", app_id, "environment", env, "themes", id]) => {
                self.handle_delete_theme(&state, app_id, env, id).await
            }
            ("GET", ["export", "app", app_id, "environment", env, "components"]) => {
                self.handle_export_components(&state, app_id, env).await
            }
            ("GET", ["export", "app", app_id, "environment", env, "forms"]) => {
                self.handle_export_forms(&state, app_id, env).await
            }
            ("GET", ["export", "app", app_id, "environment", env, "themes"]) => {
                self.handle_export_themes(&state, app_id, env).await
            }
            ("POST", ["app", app_id, "environment", env, "codegen-jobs"]) => {
                self.handle_start_codegen_job(&state, app_id, env, &body)
                    .await
            }
            ("GET", ["app", app_id, "environment", env, "codegen-jobs"]) => {
                self.handle_list_codegen_jobs(&state, app_id, env).await
            }
            ("GET", ["app", app_id, "environment", env, "codegen-jobs", id]) => {
                self.handle_get_codegen_job(&state, app_id, env, id).await
            }
            ("GET", ["app", app_id, "environment", env, "metadata"]) => {
                self.handle_get_metadata(&state, app_id, env).await
            }
            (
                "PUT",
                [
                    "app",
                    app_id,
                    "environment",
                    env,
                    "metadata",
                    "features",
                    feature,
                ],
            ) => {
                self.handle_put_metadata_flag(&state, app_id, env, feature, &body)
                    .await
            }
            ("POST", ["tokens", provider]) => {
                self.handle_exchange_code_for_token(provider, &body).await
            }
            ("POST", ["tokens", provider, "refresh"]) => {
                self.handle_refresh_token(provider, &body).await
            }
            ("GET", ["tags", arn]) => self.handle_list_tags_for_resource(&state, arn).await,
            ("POST", ["tags", arn]) => self.handle_tag_resource(&state, arn, &body).await,
            ("DELETE", ["tags", arn]) => {
                self.handle_untag_resource(&state, arn, &request.uri).await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2
            && matches!(method.as_str(), "POST" | "PATCH" | "DELETE" | "PUT")
        {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_component(
        &self,
        state: &SharedState,
        app_id: &str,
        env: &str,
        body: &Value,
    ) -> MockResponse {
        let now = chrono::Utc::now().to_rfc3339();
        let id = uuid::Uuid::new_v4().to_string();
        // Body has shape { ClientToken, EntityToCreate: { ... fields ... } }
        let entity = body.clone();
        let mut component: model::Component = serde_json::from_value(entity).unwrap_or_default();
        component.app_id = Some(app_id.to_string());
        component.environment_name = Some(env.to_string());
        component.id = Some(id.clone());
        component.created_at = Some(now.clone());
        component.modified_at = Some(now);

        let mut state = state.write().await;
        let c = state.create_component(app_id, env, component);
        wire::serialize_create_component_response(&wire::CreateComponentResponse {
            entity: Some(c.clone()),
        })
    }

    async fn handle_get_component(
        &self,
        state: &SharedState,
        app_id: &str,
        env: &str,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_component(app_id, env, id) {
            Ok(c) => wire::serialize_get_component_response(&wire::GetComponentResponse {
                component: Some(c.clone()),
            }),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_update_component(
        &self,
        state: &SharedState,
        app_id: &str,
        env: &str,
        id: &str,
        body: &Value,
    ) -> MockResponse {
        let entity = body.clone();
        let mut state = state.write().await;
        if state.get_component(app_id, env, id).is_err() {
            return err_response(&AmplifyUiBuilderError::NotFound {
                resource_type: "Component",
                id: id.to_string(),
            });
        }
        let existing = state
            .components
            .get(&(app_id.to_string(), env.to_string(), id.to_string()))
            .cloned()
            .unwrap();
        let mut updated: model::Component = serde_json::from_value(entity).unwrap_or_default();
        updated.id = Some(id.to_string());
        updated.app_id = Some(app_id.to_string());
        updated.environment_name = Some(env.to_string());
        updated.created_at = existing.created_at;
        updated.modified_at = Some(chrono::Utc::now().to_rfc3339());
        let c = state.create_component(app_id, env, updated);
        wire::serialize_update_component_response(&wire::UpdateComponentResponse {
            entity: Some(c.clone()),
        })
    }

    async fn handle_delete_component(
        &self,
        state: &SharedState,
        app_id: &str,
        env: &str,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_component(app_id, env, id) {
            Ok(()) => wire::serialize_delete_component_response(),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_list_components(
        &self,
        state: &SharedState,
        app_id: &str,
        env: &str,
        _uri: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let summaries: Vec<wire::ComponentSummary> = state
            .list_components(app_id, env)
            .into_iter()
            .map(|c| wire::ComponentSummary {
                app_id: c.app_id.clone(),
                component_type: c.component_type.clone(),
                environment_name: c.environment_name.clone(),
                id: c.id.clone(),
                name: c.name.clone(),
            })
            .collect();
        wire::serialize_list_components_response(&wire::ListComponentsResponse {
            entities: Some(summaries),
            next_token: None,
        })
    }

    async fn handle_export_components(
        &self,
        state: &SharedState,
        app_id: &str,
        env: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let entities: Vec<model::Component> = state
            .list_components(app_id, env)
            .into_iter()
            .cloned()
            .collect();
        wire::serialize_export_components_response(&wire::ExportComponentsResponse {
            entities: Some(entities),
            next_token: None,
        })
    }

    async fn handle_create_form(
        &self,
        state: &SharedState,
        app_id: &str,
        env: &str,
        body: &Value,
    ) -> MockResponse {
        let id = uuid::Uuid::new_v4().to_string();
        let entity = body.clone();
        let mut form: model::Form = serde_json::from_value(entity).unwrap_or_default();
        form.app_id = Some(app_id.to_string());
        form.environment_name = Some(env.to_string());
        form.id = Some(id);
        let mut state = state.write().await;
        let f = state.create_form(app_id, env, form);
        wire::serialize_create_form_response(&wire::CreateFormResponse {
            entity: Some(f.clone()),
        })
    }

    async fn handle_get_form(
        &self,
        state: &SharedState,
        app_id: &str,
        env: &str,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_form(app_id, env, id) {
            Ok(f) => wire::serialize_get_form_response(&wire::GetFormResponse {
                form: Some(f.clone()),
            }),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_update_form(
        &self,
        state: &SharedState,
        app_id: &str,
        env: &str,
        id: &str,
        body: &Value,
    ) -> MockResponse {
        let entity = body.clone();
        let mut state = state.write().await;
        if state.get_form(app_id, env, id).is_err() {
            return err_response(&AmplifyUiBuilderError::NotFound {
                resource_type: "Form",
                id: id.to_string(),
            });
        }
        let mut updated: model::Form = serde_json::from_value(entity).unwrap_or_default();
        updated.id = Some(id.to_string());
        updated.app_id = Some(app_id.to_string());
        updated.environment_name = Some(env.to_string());
        let f = state.create_form(app_id, env, updated);
        wire::serialize_update_form_response(&wire::UpdateFormResponse {
            entity: Some(f.clone()),
        })
    }

    async fn handle_delete_form(
        &self,
        state: &SharedState,
        app_id: &str,
        env: &str,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_form(app_id, env, id) {
            Ok(()) => wire::serialize_delete_form_response(),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_list_forms(
        &self,
        state: &SharedState,
        app_id: &str,
        env: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let summaries: Vec<wire::FormSummary> = state
            .list_forms(app_id, env)
            .into_iter()
            .map(|f| wire::FormSummary {
                app_id: f.app_id.clone(),
                data_type: f.data_type.clone(),
                environment_name: f.environment_name.clone(),
                form_action_type: f.form_action_type.clone(),
                id: f.id.clone(),
                name: f.name.clone(),
            })
            .collect();
        wire::serialize_list_forms_response(&wire::ListFormsResponse {
            entities: Some(summaries),
            next_token: None,
        })
    }

    async fn handle_export_forms(
        &self,
        state: &SharedState,
        app_id: &str,
        env: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let entities: Vec<model::Form> =
            state.list_forms(app_id, env).into_iter().cloned().collect();
        wire::serialize_export_forms_response(&wire::ExportFormsResponse {
            entities: Some(entities),
            next_token: None,
        })
    }

    async fn handle_create_theme(
        &self,
        state: &SharedState,
        app_id: &str,
        env: &str,
        body: &Value,
    ) -> MockResponse {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let entity = body.clone();
        let mut theme: model::Theme = serde_json::from_value(entity).unwrap_or_default();
        theme.app_id = Some(app_id.to_string());
        theme.environment_name = Some(env.to_string());
        theme.id = Some(id);
        theme.created_at = Some(now.clone());
        theme.modified_at = Some(now);
        let mut state = state.write().await;
        let t = state.create_theme(app_id, env, theme);
        wire::serialize_create_theme_response(&wire::CreateThemeResponse {
            entity: Some(t.clone()),
        })
    }

    async fn handle_get_theme(
        &self,
        state: &SharedState,
        app_id: &str,
        env: &str,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_theme(app_id, env, id) {
            Ok(t) => wire::serialize_get_theme_response(&wire::GetThemeResponse {
                theme: Some(t.clone()),
            }),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_update_theme(
        &self,
        state: &SharedState,
        app_id: &str,
        env: &str,
        id: &str,
        body: &Value,
    ) -> MockResponse {
        let entity = body.clone();
        let mut state = state.write().await;
        if state.get_theme(app_id, env, id).is_err() {
            return err_response(&AmplifyUiBuilderError::NotFound {
                resource_type: "Theme",
                id: id.to_string(),
            });
        }
        let existing = state
            .themes
            .get(&(app_id.to_string(), env.to_string(), id.to_string()))
            .cloned()
            .unwrap();
        let mut updated: model::Theme = serde_json::from_value(entity).unwrap_or_default();
        updated.id = Some(id.to_string());
        updated.app_id = Some(app_id.to_string());
        updated.environment_name = Some(env.to_string());
        updated.created_at = existing.created_at;
        updated.modified_at = Some(chrono::Utc::now().to_rfc3339());
        let t = state.create_theme(app_id, env, updated);
        wire::serialize_update_theme_response(&wire::UpdateThemeResponse {
            entity: Some(t.clone()),
        })
    }

    async fn handle_delete_theme(
        &self,
        state: &SharedState,
        app_id: &str,
        env: &str,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_theme(app_id, env, id) {
            Ok(()) => wire::serialize_delete_theme_response(),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_list_themes(
        &self,
        state: &SharedState,
        app_id: &str,
        env: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let summaries: Vec<wire::ThemeSummary> = state
            .list_themes(app_id, env)
            .into_iter()
            .map(|t| wire::ThemeSummary {
                app_id: t.app_id.clone(),
                environment_name: t.environment_name.clone(),
                id: t.id.clone(),
                name: t.name.clone(),
            })
            .collect();
        wire::serialize_list_themes_response(&wire::ListThemesResponse {
            entities: Some(summaries),
            next_token: None,
        })
    }

    async fn handle_export_themes(
        &self,
        state: &SharedState,
        app_id: &str,
        env: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let entities: Vec<model::Theme> = state
            .list_themes(app_id, env)
            .into_iter()
            .cloned()
            .collect();
        wire::serialize_export_themes_response(&wire::ExportThemesResponse {
            entities: Some(entities),
            next_token: None,
        })
    }

    async fn handle_start_codegen_job(
        &self,
        state: &SharedState,
        app_id: &str,
        env: &str,
        body: &Value,
    ) -> MockResponse {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let entity = body.clone();
        let mut job: model::CodegenJob = serde_json::from_value(entity).unwrap_or_default();
        job.app_id = Some(app_id.to_string());
        job.environment_name = Some(env.to_string());
        job.id = Some(id);
        job.created_at = Some(now.clone());
        job.modified_at = Some(now);
        job.status = Some("succeeded".to_string());
        let mut state = state.write().await;
        let j = state.add_codegen_job(app_id, env, job);
        wire::serialize_start_codegen_job_response(&wire::StartCodegenJobResponse {
            entity: Some(j.clone()),
        })
    }

    async fn handle_get_codegen_job(
        &self,
        state: &SharedState,
        app_id: &str,
        env: &str,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_codegen_job(app_id, env, id) {
            Ok(j) => wire::serialize_get_codegen_job_response(&wire::GetCodegenJobResponse {
                job: Some(j.clone()),
            }),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_list_codegen_jobs(
        &self,
        state: &SharedState,
        app_id: &str,
        env: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let summaries: Vec<wire::CodegenJobSummary> = state
            .list_codegen_jobs(app_id, env)
            .into_iter()
            .map(|j| wire::CodegenJobSummary {
                app_id: j.app_id.clone(),
                created_at: j.created_at.clone(),
                environment_name: j.environment_name.clone(),
                id: j.id.clone(),
                modified_at: j.modified_at.clone(),
            })
            .collect();
        wire::serialize_list_codegen_jobs_response(&wire::ListCodegenJobsResponse {
            entities: Some(summaries),
            next_token: None,
        })
    }

    async fn handle_get_metadata(
        &self,
        state: &SharedState,
        app_id: &str,
        env: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let features = state.get_metadata(app_id, env);
        wire::serialize_get_metadata_response(&wire::GetMetadataResponse {
            features: Some(features),
        })
    }

    async fn handle_put_metadata_flag(
        &self,
        state: &SharedState,
        app_id: &str,
        env: &str,
        feature: &str,
        body: &Value,
    ) -> MockResponse {
        let new_value = body
            .get("body")
            .and_then(|v| v.get("newValue"))
            .and_then(|v| v.as_str())
            .or_else(|| body.get("newValue").and_then(|v| v.as_str()))
            .unwrap_or("disabled")
            .to_string();
        let mut state = state.write().await;
        state.put_metadata_flag(app_id, env, feature.to_string(), new_value);
        wire::serialize_put_metadata_flag_response()
    }

    async fn handle_exchange_code_for_token(&self, provider: &str, body: &Value) -> MockResponse {
        let code = body
            .get("code")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if code.is_empty() {
            return rest_json_error(400, "ValidationException", "code is required");
        }
        wire::serialize_exchange_code_for_token_response(&wire::ExchangeCodeForTokenResponse {
            access_token: Some(format!("mock-access-token-{provider}")),
            expires_in: Some(3600),
            refresh_token: Some(format!("mock-refresh-token-{provider}")),
        })
    }

    async fn handle_refresh_token(&self, provider: &str, body: &Value) -> MockResponse {
        let token = body
            .get("token")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if token.is_empty() {
            return rest_json_error(400, "ValidationException", "token is required");
        }
        wire::serialize_refresh_token_response(&wire::RefreshTokenResponse {
            access_token: Some(format!("mock-access-token-{provider}")),
            expires_in: Some(3600),
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &SharedState,
        arn: &str,
        body: &Value,
    ) -> MockResponse {
        let tags = parse_tag_map(body.get("tags"));
        if tags.is_empty() {
            return rest_json_error(400, "ValidationException", "tags is required");
        }
        let mut state = state.write().await;
        state.tag_resource(arn, tags);
        wire::serialize_tag_resource_response(&wire::TagResourceResponse {})
    }

    async fn handle_untag_resource(
        &self,
        state: &SharedState,
        arn: &str,
        uri: &str,
    ) -> MockResponse {
        let qs = parse_query_string(extract_query_string(uri));
        let keys: Vec<String> = qs
            .iter()
            .filter(|(k, _)| *k == "tagKeys")
            .map(|(_, v)| v.clone())
            .collect();
        let mut state = state.write().await;
        state.untag_resource(arn, &keys);
        wire::serialize_untag_resource_response(&wire::UntagResourceResponse {})
    }

    async fn handle_list_tags_for_resource(&self, state: &SharedState, arn: &str) -> MockResponse {
        let state = state.read().await;
        let tags = state.list_tags(arn);
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: if tags.is_empty() { None } else { Some(tags) },
        })
    }
}

fn parse_tag_map(v: Option<&Value>) -> HashMap<String, String> {
    v.and_then(|v| v.as_object())
        .map(|m| {
            m.iter()
                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                .collect()
        })
        .unwrap_or_default()
}

fn err_response(err: &AmplifyUiBuilderError) -> MockResponse {
    let (status, error_type) = match err {
        AmplifyUiBuilderError::NotFound { .. } => (404, "ResourceNotFoundException"),
        AmplifyUiBuilderError::Validation { .. } => (400, "ValidationException"),
    };
    let body = json!({"message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
