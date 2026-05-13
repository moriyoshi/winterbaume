use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::state::{DataBrewError, DataBrewState};
use crate::views::DataBrewStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct DataBrewService {
    pub(crate) state: Arc<BackendState<DataBrewState>>,
    pub(crate) notifier: StateChangeNotifier<DataBrewStateView>,
}

impl DataBrewService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for DataBrewService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for DataBrewService {
    fn service_name(&self) -> &str {
        "databrew"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://databrew\..*\.amazonaws\.com",
            r"https?://databrew\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl DataBrewService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let query_string = extract_query_string(&request.uri);
        let query_map: HashMap<String, String> =
            winterbaume_core::parse_query_string(&query_string);
        let method = request.method.as_str();

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        if segments.is_empty() {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        use winterbaume_core::StatefulService;
        let response = match segments[0] {
            "datasets" => {
                self.dispatch_datasets(
                    method, &segments, &request, &query_map, &state, account_id, &region,
                )
                .await
            }
            "recipes" => {
                self.dispatch_recipes(
                    method, &segments, &request, &query_map, &state, account_id, &region,
                )
                .await
            }
            "recipeVersions" => {
                self.dispatch_recipe_versions(method, &request, &query_map, &state)
                    .await
            }
            "rulesets" => {
                self.dispatch_rulesets(
                    method, &segments, &request, &query_map, &state, account_id, &region,
                )
                .await
            }
            "schedules" => {
                self.dispatch_schedules(
                    method, &segments, &request, &query_map, &state, account_id, &region,
                )
                .await
            }
            "profileJobs" => {
                self.dispatch_profile_jobs(
                    method, &segments, &request, &query_map, &state, account_id, &region,
                )
                .await
            }
            "recipeJobs" => {
                self.dispatch_recipe_jobs(
                    method, &segments, &request, &query_map, &state, account_id, &region,
                )
                .await
            }
            "jobs" => {
                self.dispatch_jobs(method, &segments, &request, &query_map, &state)
                    .await
            }
            "tags" => {
                self.dispatch_tags(method, &segments, &request, &query_map, &state)
                    .await
            }
            "projects" => {
                self.dispatch_projects(method, &segments, &request, &query_map, &state)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_datasets(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        match (method, segments.len()) {
            ("POST", 1) => {
                self.handle_create_dataset(state, request, &[], query, account_id, region)
                    .await
            }
            ("GET", 1) => self.handle_list_datasets(state).await,
            ("GET", 2) => {
                let name = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_describe_dataset(state, request, labels, query)
                    .await
            }
            ("PUT", 2) => {
                let name = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_update_dataset(state, request, labels, query, account_id)
                    .await
            }
            ("DELETE", 2) => {
                let name = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_delete_dataset(state, request, labels, query)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_recipes(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        match (method, segments) {
            // POST /recipes - CreateRecipe
            ("POST", ["recipes"]) => {
                self.handle_create_recipe(state, request, &[], query, account_id, region)
                    .await
            }
            // GET /recipes - ListRecipes
            ("GET", ["recipes"]) => self.handle_list_recipes(state).await,
            // POST /recipes/{name}/publishRecipe - PublishRecipe
            ("POST", ["recipes", name, "publishRecipe"]) => {
                let name = percent_decode(name);
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_publish_recipe(state, request, labels, query, account_id)
                    .await
            }
            // GET /recipes/{name} - DescribeRecipe
            ("GET", ["recipes", name]) => {
                let name = percent_decode(name);
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_describe_recipe(state, request, labels, query)
                    .await
            }
            // PUT /recipes/{name} - UpdateRecipe
            ("PUT", ["recipes", name]) => {
                let name = percent_decode(name);
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_update_recipe(state, request, labels, query, account_id)
                    .await
            }
            // DELETE /recipes/{name}/recipeVersion/{version} - DeleteRecipeVersion
            ("DELETE", ["recipes", name, "recipeVersion", version]) => {
                let name = percent_decode(name);
                let version = percent_decode(version);
                let labels: &[(&str, &str)] =
                    &[("Name", name.as_str()), ("RecipeVersion", version.as_str())];
                self.handle_delete_recipe_version(state, request, labels, query)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn dispatch_recipe_versions(
        &self,
        method: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
    ) -> MockResponse {
        // GET /recipeVersions?name={name} - ListRecipeVersions
        if method == "GET" {
            return self
                .handle_list_recipe_versions(state, request, &[], query)
                .await;
        }
        rest_json_error(404, "UnknownOperationException", "Not found")
    }

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_rulesets(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        match (method, segments.len()) {
            ("POST", 1) => {
                self.handle_create_ruleset(state, request, &[], query, account_id, region)
                    .await
            }
            ("GET", 1) => self.handle_list_rulesets(state).await,
            ("GET", 2) => {
                let name = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_describe_ruleset(state, request, labels, query)
                    .await
            }
            ("PUT", 2) => {
                let name = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_update_ruleset(state, request, labels, query, account_id)
                    .await
            }
            ("DELETE", 2) => {
                let name = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_delete_ruleset(state, request, labels, query)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_schedules(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        match (method, segments.len()) {
            ("POST", 1) => {
                self.handle_create_schedule(state, request, &[], query, account_id, region)
                    .await
            }
            ("GET", 1) => self.handle_list_schedules(state).await,
            ("GET", 2) => {
                let name = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_describe_schedule(state, request, labels, query)
                    .await
            }
            ("PUT", 2) => {
                let name = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_update_schedule(state, request, labels, query, account_id)
                    .await
            }
            ("DELETE", 2) => {
                let name = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_delete_schedule(state, request, labels, query)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn dispatch_tags(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
    ) -> MockResponse {
        // Tags routes: /tags/{resourceArn+}
        if segments.len() < 2 {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        let arn_parts: Vec<String> = segments[1..].iter().map(|s| percent_decode(s)).collect();
        let resource_arn = arn_parts.join("/");
        let labels: &[(&str, &str)] = &[("ResourceArn", resource_arn.as_str())];

        match method {
            "GET" => {
                self.handle_list_tags_for_resource(state, request, labels, query)
                    .await
            }
            "POST" => {
                self.handle_tag_resource(state, request, labels, query)
                    .await
            }
            "DELETE" => {
                self.handle_untag_resource(state, request, labels, query)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_profile_jobs(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        match (method, segments.len()) {
            ("POST", 1) => {
                self.handle_create_profile_job(state, request, &[], query, account_id, region)
                    .await
            }
            ("PUT", 2) => {
                let name = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_update_profile_job(state, request, labels, query, account_id)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_recipe_jobs(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        match (method, segments.len()) {
            ("POST", 1) => {
                self.handle_create_recipe_job(state, request, &[], query, account_id, region)
                    .await
            }
            ("PUT", 2) => {
                let name = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_update_recipe_job(state, request, labels, query, account_id)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn dispatch_jobs(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
    ) -> MockResponse {
        match (method, segments.len()) {
            ("GET", 1) => self.handle_list_jobs(state, request, &[], query).await,
            ("GET", 2) => {
                let name = percent_decode(segments[1]);
                if name.len() > 240 {
                    return rest_json_error(
                        400,
                        "ValidationException",
                        &format!(
                            "1 validation error detected: Value '{}' at 'name' failed to satisfy constraint: Member must have length less than or equal to 240",
                            name
                        ),
                    );
                }
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_describe_job(state, request, labels, query)
                    .await
            }
            ("DELETE", 2) => {
                let name = percent_decode(segments[1]);
                if name.len() > 240 {
                    return rest_json_error(
                        400,
                        "ValidationException",
                        &format!(
                            "1 validation error detected: Value '{}' at 'name' failed to satisfy constraint: Member must have length less than or equal to 240",
                            name
                        ),
                    );
                }
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_delete_job(state, request, labels, query).await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn dispatch_projects(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
    ) -> MockResponse {
        // PUT /projects/{name}/sendProjectSessionAction - SendProjectSessionAction
        if method == "PUT" && segments.len() == 3 && segments[2] == "sendProjectSessionAction" {
            let name = percent_decode(segments[1]);
            let labels: &[(&str, &str)] = &[("Name", name.as_str())];
            return self
                .handle_send_project_session_action(state, request, labels, query)
                .await;
        }
        rest_json_error(404, "UnknownOperationException", "Not found")
    }

    // ── Dataset handlers ────────────────────────────────────────────

    async fn handle_create_dataset(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_dataset_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'Name'");
        }
        let name = input.name.clone();
        let input_value = serde_json::to_value(&input.input).unwrap_or(Value::Null);
        let format = input.format.as_deref();
        let format_options = input
            .format_options
            .as_ref()
            .and_then(|fo| serde_json::to_value(fo).ok());
        let tags = input.tags;
        let path_options = input
            .path_options
            .as_ref()
            .and_then(|po| serde_json::to_value(po).ok());

        let mut state = state.write().await;
        match state.create_dataset(
            &name,
            input_value,
            format,
            format_options,
            tags,
            path_options,
            account_id,
            region,
        ) {
            Ok(dataset) => wire::serialize_create_dataset_response(&wire::CreateDatasetResponse {
                name: Some(dataset.name.clone()),
            }),
            Err(e) => databrew_error_response(&e),
        }
    }

    async fn handle_describe_dataset(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_dataset_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.describe_dataset(&input.name) {
            Ok(dataset) => {
                wire::serialize_describe_dataset_response(&dataset_to_describe_response(dataset))
            }
            Err(e) => databrew_error_response(&e),
        }
    }

    async fn handle_delete_dataset(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_dataset_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_dataset(&input.name) {
            Ok(deleted_name) => {
                wire::serialize_delete_dataset_response(&wire::DeleteDatasetResponse {
                    name: Some(deleted_name),
                })
            }
            Err(e) => databrew_error_response(&e),
        }
    }

    async fn handle_list_datasets(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let datasets: Vec<wire::Dataset> = state
            .list_datasets()
            .iter()
            .map(|d| dataset_to_wire(d))
            .collect();
        wire::serialize_list_datasets_response(&wire::ListDatasetsResponse {
            datasets: Some(datasets),
            next_token: None,
        })
    }

    // ── Recipe handlers ─────────────────────────────────────────────

    async fn handle_create_recipe(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_recipe_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'Name'");
        }
        let name = input.name;
        let steps: Vec<Value> = input
            .steps
            .iter()
            .filter_map(|s| serde_json::to_value(s).ok())
            .collect();
        let description = input.description.as_deref();
        let tags = input.tags;

        let mut state = state.write().await;
        match state.create_recipe(&name, description, steps, tags, account_id, region) {
            Ok(recipe) => wire::serialize_create_recipe_response(&wire::CreateRecipeResponse {
                name: Some(recipe.name.clone()),
            }),
            Err(e) => databrew_error_response(&e),
        }
    }

    async fn handle_describe_recipe(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_recipe_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.describe_recipe(&input.name, input.recipe_version.as_deref()) {
            Ok(version) => {
                wire::serialize_describe_recipe_response(&wire::DescribeRecipeResponse {
                    name: Some(version.name.clone()),
                    description: version.description.clone(),
                    recipe_version: Some(version.version.clone()),
                    steps: Some(steps_to_wire(&version.steps)),
                    project_name: version.project_name.clone(),
                    tags: version.tags.clone(),
                    created_by: Some(version.created_by.clone()),
                    create_date: Some(version.create_date.timestamp() as f64),
                    last_modified_by: Some(version.last_modified_by.clone()),
                    last_modified_date: Some(version.last_modified_date.timestamp() as f64),
                    resource_arn: Some(version.resource_arn.clone()),
                    published_by: version.published_by.clone(),
                    published_date: version.published_date.map(|d| d.timestamp() as f64),
                })
            }
            Err(e) => databrew_error_response(&e),
        }
    }

    async fn handle_update_recipe(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_recipe_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = input.name;
        let description = input.description.as_deref();
        let steps: Option<Vec<Value>> = input.steps.map(|s| {
            s.iter()
                .filter_map(|step| serde_json::to_value(step).ok())
                .collect()
        });

        let mut state = state.write().await;
        match state.update_recipe(&name, description, steps, account_id) {
            Ok(recipe_name) => {
                wire::serialize_update_recipe_response(&wire::UpdateRecipeResponse {
                    name: Some(recipe_name),
                })
            }
            Err(e) => databrew_error_response(&e),
        }
    }

    async fn handle_publish_recipe(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_publish_recipe_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.publish_recipe(&input.name, input.description.as_deref(), account_id) {
            Ok(recipe_name) => {
                wire::serialize_publish_recipe_response(&wire::PublishRecipeResponse {
                    name: Some(recipe_name),
                })
            }
            Err(e) => databrew_error_response(&e),
        }
    }

    async fn handle_delete_recipe_version(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_recipe_version_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_recipe_version(&input.name, &input.recipe_version) {
            Ok((rname, rversion)) => {
                wire::serialize_delete_recipe_version_response(&wire::DeleteRecipeVersionResponse {
                    name: Some(rname),
                    recipe_version: Some(rversion),
                })
            }
            Err(e) => databrew_error_response(&e),
        }
    }

    async fn handle_list_recipes(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let recipes: Vec<wire::Recipe> = state
            .list_recipes()
            .iter()
            .map(|r| recipe_to_wire(r))
            .collect();
        wire::serialize_list_recipes_response(&wire::ListRecipesResponse {
            recipes: Some(recipes),
            next_token: None,
        })
    }

    async fn handle_list_recipe_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_recipe_versions_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'name' query parameter");
        }
        let state = state.read().await;
        let versions = state.list_recipe_versions(&input.name);
        let recipes: Vec<wire::Recipe> =
            versions.iter().map(|v| recipe_version_to_wire(v)).collect();
        wire::serialize_list_recipe_versions_response(&wire::ListRecipeVersionsResponse {
            recipes: Some(recipes),
            next_token: None,
        })
    }

    // ── Ruleset handlers ────────────────────────────────────────────

    async fn handle_create_ruleset(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_ruleset_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'Name'");
        }
        if input.target_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'TargetArn'");
        }
        let name = input.name;
        let target_arn = input.target_arn;
        let rules: Vec<Value> = input
            .rules
            .iter()
            .filter_map(|r| serde_json::to_value(r).ok())
            .collect();
        let description = input.description.as_deref();
        let tags = input.tags;

        let mut state = state.write().await;
        match state.create_ruleset(
            &name,
            description,
            &target_arn,
            rules,
            tags,
            account_id,
            region,
        ) {
            Ok(ruleset) => wire::serialize_create_ruleset_response(&wire::CreateRulesetResponse {
                name: Some(ruleset.name.clone()),
            }),
            Err(e) => databrew_error_response(&e),
        }
    }

    async fn handle_describe_ruleset(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_ruleset_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.describe_ruleset(&input.name) {
            Ok(ruleset) => {
                wire::serialize_describe_ruleset_response(&wire::DescribeRulesetResponse {
                    name: Some(ruleset.name.clone()),
                    description: ruleset.description.clone(),
                    target_arn: Some(ruleset.target_arn.clone()),
                    rules: Some(rules_to_wire(&ruleset.rules)),
                    tags: ruleset.tags.clone(),
                    created_by: Some(ruleset.created_by.clone()),
                    create_date: Some(ruleset.create_date.timestamp() as f64),
                    last_modified_by: Some(ruleset.last_modified_by.clone()),
                    last_modified_date: Some(ruleset.last_modified_date.timestamp() as f64),
                    resource_arn: Some(ruleset.resource_arn.clone()),
                })
            }
            Err(e) => databrew_error_response(&e),
        }
    }

    async fn handle_delete_ruleset(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_ruleset_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_ruleset(&input.name) {
            Ok(deleted_name) => {
                wire::serialize_delete_ruleset_response(&wire::DeleteRulesetResponse {
                    name: Some(deleted_name),
                })
            }
            Err(e) => databrew_error_response(&e),
        }
    }

    async fn handle_list_rulesets(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let rulesets: Vec<wire::RulesetItem> = state
            .list_rulesets()
            .iter()
            .map(|r| ruleset_to_wire(r))
            .collect();
        wire::serialize_list_rulesets_response(&wire::ListRulesetsResponse {
            rulesets: Some(rulesets),
            next_token: None,
        })
    }

    // ── Schedule handlers ───────────────────────────────────────────

    async fn handle_create_schedule(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_schedule_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'Name'");
        }
        let name = input.name;
        let cron_expression = if input.cron_expression.is_empty() {
            None
        } else {
            Some(input.cron_expression.as_str())
        };
        let job_names = input.job_names;
        let tags = input.tags;

        let mut state = state.write().await;
        match state.create_schedule(&name, cron_expression, job_names, tags, account_id, region) {
            Ok(schedule) => {
                wire::serialize_create_schedule_response(&wire::CreateScheduleResponse {
                    name: Some(schedule.name.clone()),
                })
            }
            Err(e) => databrew_error_response(&e),
        }
    }

    async fn handle_describe_schedule(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_schedule_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.describe_schedule(&input.name) {
            Ok(schedule) => {
                wire::serialize_describe_schedule_response(&wire::DescribeScheduleResponse {
                    name: Some(schedule.name.clone()),
                    cron_expression: schedule.cron_expression.clone(),
                    job_names: schedule.job_names.clone(),
                    tags: schedule.tags.clone(),
                    created_by: Some(schedule.created_by.clone()),
                    create_date: Some(schedule.create_date.timestamp() as f64),
                    last_modified_by: Some(schedule.last_modified_by.clone()),
                    last_modified_date: Some(schedule.last_modified_date.timestamp() as f64),
                    resource_arn: Some(schedule.resource_arn.clone()),
                })
            }
            Err(e) => databrew_error_response(&e),
        }
    }

    async fn handle_update_schedule(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_schedule_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let cron_expression = if input.cron_expression.is_empty() {
            None
        } else {
            Some(input.cron_expression.as_str())
        };
        let job_names = input.job_names;

        let mut state = state.write().await;
        match state.update_schedule(&input.name, cron_expression, job_names, account_id) {
            Ok(schedule_name) => {
                wire::serialize_update_schedule_response(&wire::UpdateScheduleResponse {
                    name: Some(schedule_name),
                })
            }
            Err(e) => databrew_error_response(&e),
        }
    }

    async fn handle_delete_schedule(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_schedule_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_schedule(&input.name) {
            Ok(deleted_name) => {
                wire::serialize_delete_schedule_response(&wire::DeleteScheduleResponse {
                    name: Some(deleted_name),
                })
            }
            Err(e) => databrew_error_response(&e),
        }
    }

    async fn handle_list_schedules(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let schedules: Vec<wire::Schedule> = state
            .list_schedules()
            .iter()
            .map(|s| schedule_to_wire(s))
            .collect();
        wire::serialize_list_schedules_response(&wire::ListSchedulesResponse {
            schedules: Some(schedules),
            next_token: None,
        })
    }

    // ── Tag handlers ────────────────────────────────────────────────

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_tags_for_resource(&input.resource_arn) {
            Ok(tags) => wire::serialize_list_tags_for_resource_response(
                &wire::ListTagsForResourceResponse { tags },
            ),
            Err(e) => databrew_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.tags.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'Tags'");
        }
        let resource_arn = input.resource_arn;
        let tags = input.tags;

        let mut state = state.write().await;
        match state.tag_resource(&resource_arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => databrew_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, &input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => databrew_error_response(&e),
        }
    }

    // ── Dataset update handler ───────────────────────────────────────

    async fn handle_update_dataset(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_dataset_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = input.name;
        let format = input.format.as_deref();
        let format_options = input
            .format_options
            .as_ref()
            .and_then(|fo| serde_json::to_value(fo).ok());
        let input_value = serde_json::to_value(&input.input).ok();
        let path_options = input
            .path_options
            .as_ref()
            .and_then(|po| serde_json::to_value(po).ok());

        let mut state = state.write().await;
        match state.update_dataset(
            &name,
            format,
            format_options,
            input_value,
            path_options,
            account_id,
        ) {
            Ok(dataset_name) => {
                wire::serialize_update_dataset_response(&wire::UpdateDatasetResponse {
                    name: Some(dataset_name),
                })
            }
            Err(e) => databrew_error_response(&e),
        }
    }

    // ── Ruleset update handler ─────────────────────────────────────

    async fn handle_update_ruleset(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_ruleset_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let description = input.description.as_deref();
        let rules: Option<Vec<Value>> = if input.rules.is_empty() {
            None
        } else {
            Some(
                input
                    .rules
                    .iter()
                    .filter_map(|r| serde_json::to_value(r).ok())
                    .collect(),
            )
        };

        let mut state = state.write().await;
        match state.update_ruleset(&input.name, description, rules, account_id) {
            Ok(ruleset_name) => {
                wire::serialize_update_ruleset_response(&wire::UpdateRulesetResponse {
                    name: Some(ruleset_name),
                })
            }
            Err(e) => databrew_error_response(&e),
        }
    }

    // ── Job handlers ──────────────────────────────────────────────

    async fn handle_create_profile_job(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_profile_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'Name'");
        }
        if input.dataset_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'DatasetName'");
        }
        if input.role_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'RoleArn'");
        }
        let output_location = serde_json::to_value(&input.output_location).unwrap_or(Value::Null);
        let tags = input.tags;

        let mut state = state.write().await;
        match state.create_profile_job(
            &input.name,
            &input.dataset_name,
            &input.role_arn,
            output_location,
            tags,
            account_id,
            region,
        ) {
            Ok(job) => {
                wire::serialize_create_profile_job_response(&wire::CreateProfileJobResponse {
                    name: Some(job.name.clone()),
                })
            }
            Err(e) => databrew_error_response(&e),
        }
    }

    async fn handle_create_recipe_job(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_recipe_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'Name'");
        }
        if input.role_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'RoleArn'");
        }
        let dataset_name = input.dataset_name.as_deref();
        let project_name = input.project_name.as_deref();
        let encryption_mode = input.encryption_mode.as_deref();
        let log_subscription = input.log_subscription.as_deref();
        let tags = input.tags;

        let mut state = state.write().await;
        match state.create_recipe_job(
            &input.name,
            &input.role_arn,
            dataset_name,
            project_name,
            encryption_mode,
            log_subscription,
            tags,
            account_id,
            region,
        ) {
            Ok(job) => wire::serialize_create_recipe_job_response(&wire::CreateRecipeJobResponse {
                name: Some(job.name.clone()),
            }),
            Err(e) => databrew_error_response(&e),
        }
    }

    async fn handle_describe_job(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.describe_job(&input.name) {
            Ok(job) => wire::serialize_describe_job_response(&job_to_describe_response(job)),
            Err(e) => databrew_error_response(&e),
        }
    }

    async fn handle_update_profile_job(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_profile_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let role_arn = if input.role_arn.is_empty() {
            None
        } else {
            Some(input.role_arn.as_str())
        };
        let output_location = serde_json::to_value(&input.output_location).ok();

        let mut state = state.write().await;
        match state.update_profile_job(&input.name, role_arn, output_location, account_id) {
            Ok(job_name) => {
                wire::serialize_update_profile_job_response(&wire::UpdateProfileJobResponse {
                    name: Some(job_name),
                })
            }
            Err(e) => databrew_error_response(&e),
        }
    }

    async fn handle_update_recipe_job(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_recipe_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let role_arn = if input.role_arn.is_empty() {
            None
        } else {
            Some(input.role_arn.as_str())
        };

        let mut state = state.write().await;
        match state.update_recipe_job(&input.name, role_arn, account_id) {
            Ok(job_name) => {
                wire::serialize_update_recipe_job_response(&wire::UpdateRecipeJobResponse {
                    name: Some(job_name),
                })
            }
            Err(e) => databrew_error_response(&e),
        }
    }

    async fn handle_delete_job(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_job(&input.name) {
            Ok(deleted_name) => wire::serialize_delete_job_response(&wire::DeleteJobResponse {
                name: Some(deleted_name),
            }),
            Err(e) => databrew_error_response(&e),
        }
    }

    async fn handle_list_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_jobs_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let jobs: Vec<wire::Job> = state
            .list_jobs(input.dataset_name.as_deref(), input.project_name.as_deref())
            .iter()
            .map(|j| job_to_wire(j))
            .collect();
        wire::serialize_list_jobs_response(&wire::ListJobsResponse {
            jobs: Some(jobs),
            next_token: None,
        })
    }

    // ── Project session handler ─────────────────────────────────────

    // STUB[no-engine]: SendProjectSessionAction requires a live interactive session engine;
    //   the mock returns a minimal success response with no real transformation applied.
    async fn handle_send_project_session_action(
        &self,
        _state: &Arc<tokio::sync::RwLock<DataBrewState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_send_project_session_action_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        wire::serialize_send_project_session_action_response(
            &wire::SendProjectSessionActionResponse {
                name: Some(input.name),
                action_id: Some(1),
                result: None,
            },
        )
    }
}

// ── Conversion helpers ──────────────────────────────────────────────

/// Convert a `Value` (from state) to a wire `Input` struct.
fn value_to_input(v: &Value) -> wire::Input {
    let s3 = v.get("S3InputDefinition").and_then(|s| {
        Some(wire::S3Location {
            bucket: s.get("Bucket")?.as_str()?.to_string(),
            bucket_owner: s
                .get("BucketOwner")
                .and_then(|b| b.as_str().map(|s| s.to_string())),
            key: s.get("Key").and_then(|k| k.as_str().map(|s| s.to_string())),
        })
    });
    let database = v.get("DatabaseInputDefinition").and_then(|d| {
        Some(wire::DatabaseInputDefinition {
            glue_connection_name: d.get("GlueConnectionName")?.as_str()?.to_string(),
            database_table_name: d
                .get("DatabaseTableName")
                .and_then(|t| t.as_str().map(|s| s.to_string())),
            query_string: d
                .get("QueryString")
                .and_then(|q| q.as_str().map(|s| s.to_string())),
            temp_directory: d.get("TempDirectory").and_then(|t| {
                Some(wire::S3Location {
                    bucket: t.get("Bucket")?.as_str()?.to_string(),
                    bucket_owner: t
                        .get("BucketOwner")
                        .and_then(|b| b.as_str().map(|s| s.to_string())),
                    key: t.get("Key").and_then(|k| k.as_str().map(|s| s.to_string())),
                })
            }),
        })
    });
    let data_catalog = v.get("DataCatalogInputDefinition").and_then(|d| {
        Some(wire::DataCatalogInputDefinition {
            catalog_id: d
                .get("CatalogId")
                .and_then(|c| c.as_str().map(|s| s.to_string())),
            database_name: d.get("DatabaseName")?.as_str()?.to_string(),
            table_name: d.get("TableName")?.as_str()?.to_string(),
            temp_directory: d.get("TempDirectory").and_then(|t| {
                Some(wire::S3Location {
                    bucket: t.get("Bucket")?.as_str()?.to_string(),
                    bucket_owner: t
                        .get("BucketOwner")
                        .and_then(|b| b.as_str().map(|s| s.to_string())),
                    key: t.get("Key").and_then(|k| k.as_str().map(|s| s.to_string())),
                })
            }),
        })
    });
    let metadata = v.get("Metadata").map(|m| wire::Metadata {
        source_arn: m
            .get("SourceArn")
            .and_then(|s| s.as_str().map(|s| s.to_string())),
    });
    wire::Input {
        s3_input_definition: s3,
        database_input_definition: database,
        data_catalog_input_definition: data_catalog,
        metadata,
    }
}

/// Convert a state `Dataset` to a wire `wire::Dataset`.
fn dataset_to_wire(dataset: &crate::types::Dataset) -> wire::Dataset {
    wire::Dataset {
        name: Some(dataset.name.clone()),
        account_id: Some(dataset.account_id.clone()),
        created_by: Some(dataset.created_by.clone()),
        create_date: Some(dataset.create_date.timestamp() as f64),
        last_modified_by: Some(dataset.last_modified_by.clone()),
        last_modified_date: Some(dataset.last_modified_date.timestamp() as f64),
        source: Some(dataset.source.clone()),
        input: Some(value_to_input(&dataset.input)),
        resource_arn: Some(dataset.resource_arn.clone()),
        format: dataset.format.clone(),
        format_options: value_to_format_options(dataset.format_options.as_ref()),
        tags: dataset.tags.clone(),
        path_options: value_to_path_options(dataset.path_options.as_ref()),
    }
}

/// Convert a state `Dataset` to a wire `DescribeDatasetResponse`.
fn dataset_to_describe_response(dataset: &crate::types::Dataset) -> wire::DescribeDatasetResponse {
    wire::DescribeDatasetResponse {
        name: Some(dataset.name.clone()),
        created_by: Some(dataset.created_by.clone()),
        create_date: Some(dataset.create_date.timestamp() as f64),
        last_modified_by: Some(dataset.last_modified_by.clone()),
        last_modified_date: Some(dataset.last_modified_date.timestamp() as f64),
        source: Some(dataset.source.clone()),
        input: Some(value_to_input(&dataset.input)),
        resource_arn: Some(dataset.resource_arn.clone()),
        format: dataset.format.clone(),
        format_options: value_to_format_options(dataset.format_options.as_ref()),
        tags: dataset.tags.clone(),
        path_options: value_to_path_options(dataset.path_options.as_ref()),
    }
}

/// Convert steps from Value to wire RecipeStep.
fn steps_to_wire(steps: &[Value]) -> Vec<wire::RecipeStep> {
    steps
        .iter()
        .filter_map(|s| {
            let action = s.get("Action")?;
            let operation = action.get("Operation")?.as_str()?.to_string();
            let parameters: Option<HashMap<String, String>> = action
                .get("Parameters")
                .and_then(|p| serde_json::from_value(p.clone()).ok());
            let condition_expressions = s.get("ConditionExpressions").and_then(|ce| {
                ce.as_array().map(|arr| {
                    arr.iter()
                        .filter_map(|c| {
                            Some(wire::ConditionExpression {
                                condition: c.get("Condition")?.as_str()?.to_string(),
                                target_column: c.get("TargetColumn")?.as_str()?.to_string(),
                                value: c
                                    .get("Value")
                                    .and_then(|v| v.as_str().map(|s| s.to_string())),
                            })
                        })
                        .collect()
                })
            });
            Some(wire::RecipeStep {
                action: wire::RecipeAction {
                    operation,
                    parameters,
                },
                condition_expressions,
            })
        })
        .collect()
}

/// Convert a state Recipe to wire Recipe (latest version).
fn recipe_to_wire(recipe: &crate::types::Recipe) -> wire::Recipe {
    wire::Recipe {
        name: Some(recipe.name.clone()),
        description: recipe.description.clone(),
        recipe_version: recipe.versions.keys().last().cloned(),
        steps: Some(steps_to_wire(&recipe.steps)),
        project_name: recipe.project_name.clone(),
        tags: recipe.tags.clone(),
        created_by: Some(recipe.created_by.clone()),
        create_date: Some(recipe.create_date.timestamp() as f64),
        last_modified_by: Some(recipe.last_modified_by.clone()),
        last_modified_date: Some(recipe.last_modified_date.timestamp() as f64),
        resource_arn: Some(recipe.resource_arn.clone()),
        published_by: None,
        published_date: None,
    }
}

/// Convert a RecipeVersion to wire Recipe.
fn recipe_version_to_wire(version: &crate::types::RecipeVersion) -> wire::Recipe {
    wire::Recipe {
        name: Some(version.name.clone()),
        description: version.description.clone(),
        recipe_version: Some(version.version.clone()),
        steps: Some(steps_to_wire(&version.steps)),
        project_name: version.project_name.clone(),
        tags: version.tags.clone(),
        created_by: Some(version.created_by.clone()),
        create_date: Some(version.create_date.timestamp() as f64),
        last_modified_by: Some(version.last_modified_by.clone()),
        last_modified_date: Some(version.last_modified_date.timestamp() as f64),
        resource_arn: Some(version.resource_arn.clone()),
        published_by: version.published_by.clone(),
        published_date: version.published_date.map(|d| d.timestamp() as f64),
    }
}

/// Convert rules from Value to wire Rule.
fn rules_to_wire(rules: &[Value]) -> Vec<wire::Rule> {
    rules
        .iter()
        .filter_map(|r| {
            let check_expression = r.get("CheckExpression")?.as_str()?.to_string();
            let name = r.get("Name")?.as_str()?.to_string();
            let disabled = r.get("Disabled").and_then(|d| d.as_bool());
            let substitution_map: Option<HashMap<String, String>> = r
                .get("SubstitutionMap")
                .and_then(|s| serde_json::from_value(s.clone()).ok());
            let column_selectors = r.get("ColumnSelectors").and_then(|cs| {
                cs.as_array().map(|arr| {
                    arr.iter()
                        .map(|c| wire::ColumnSelector {
                            name: c
                                .get("Name")
                                .and_then(|n| n.as_str().map(|s| s.to_string())),
                            regex: c
                                .get("Regex")
                                .and_then(|r| r.as_str().map(|s| s.to_string())),
                        })
                        .collect()
                })
            });
            let threshold = r.get("Threshold").and_then(|t| {
                Some(wire::Threshold {
                    value: t.get("Value")?.as_f64()?,
                    r#type: t
                        .get("Type")
                        .and_then(|ty| ty.as_str().map(|s| s.to_string())),
                    unit: t
                        .get("Unit")
                        .and_then(|u| u.as_str().map(|s| s.to_string())),
                })
            });
            Some(wire::Rule {
                check_expression,
                name,
                disabled,
                substitution_map,
                column_selectors,
                threshold,
            })
        })
        .collect()
}

/// Convert a state Ruleset to wire RulesetItem.
fn ruleset_to_wire(ruleset: &crate::types::Ruleset) -> wire::RulesetItem {
    wire::RulesetItem {
        name: Some(ruleset.name.clone()),
        description: ruleset.description.clone(),
        target_arn: Some(ruleset.target_arn.clone()),
        rule_count: Some(ruleset.rules.len() as i32),
        tags: ruleset.tags.clone(),
        account_id: Some(ruleset.account_id.clone()),
        created_by: Some(ruleset.created_by.clone()),
        create_date: Some(ruleset.create_date.timestamp() as f64),
        last_modified_by: Some(ruleset.last_modified_by.clone()),
        last_modified_date: Some(ruleset.last_modified_date.timestamp() as f64),
        resource_arn: Some(ruleset.resource_arn.clone()),
    }
}

/// Convert a state Schedule to wire Schedule.
fn schedule_to_wire(schedule: &crate::types::Schedule) -> wire::Schedule {
    wire::Schedule {
        name: Some(schedule.name.clone()),
        cron_expression: schedule.cron_expression.clone(),
        job_names: schedule.job_names.clone(),
        tags: schedule.tags.clone(),
        account_id: Some(schedule.account_id.clone()),
        created_by: Some(schedule.created_by.clone()),
        create_date: Some(schedule.create_date.timestamp() as f64),
        last_modified_by: Some(schedule.last_modified_by.clone()),
        last_modified_date: Some(schedule.last_modified_date.timestamp() as f64),
        resource_arn: Some(schedule.resource_arn.clone()),
    }
}

/// Convert a `Value` to a wire `FormatOptions`, if present.
fn value_to_format_options(v: Option<&Value>) -> Option<wire::FormatOptions> {
    let v = v?;
    Some(wire::FormatOptions {
        csv: v.get("Csv").map(|c| wire::CsvOptions {
            delimiter: c
                .get("Delimiter")
                .and_then(|d| d.as_str().map(|s| s.to_string())),
            header_row: c.get("HeaderRow").and_then(|h| h.as_bool()),
        }),
        excel: v.get("Excel").map(|e| wire::ExcelOptions {
            header_row: e.get("HeaderRow").and_then(|h| h.as_bool()),
            sheet_indexes: e.get("SheetIndexes").and_then(|s| {
                s.as_array().map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_i64().map(|n| n as i32))
                        .collect()
                })
            }),
            sheet_names: e.get("SheetNames").and_then(|s| {
                s.as_array().map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
            }),
        }),
        json: v.get("Json").map(|j| wire::JsonOptions {
            multi_line: j.get("MultiLine").and_then(|m| m.as_bool()),
        }),
    })
}

/// Convert a `Value` to a wire `PathOptions`, if present.
fn value_to_path_options(v: Option<&Value>) -> Option<wire::PathOptions> {
    let v = v?;
    Some(wire::PathOptions {
        files_limit: v.get("FilesLimit").and_then(|f| {
            Some(wire::FilesLimit {
                max_files: f.get("MaxFiles")?.as_i64()? as i32,
                order: f
                    .get("Order")
                    .and_then(|o| o.as_str().map(|s| s.to_string())),
                ordered_by: f
                    .get("OrderedBy")
                    .and_then(|o| o.as_str().map(|s| s.to_string())),
            })
        }),
        last_modified_date_condition: v.get("LastModifiedDateCondition").and_then(|l| {
            Some(wire::FilterExpression {
                expression: l.get("Expression")?.as_str()?.to_string(),
                values_map: l
                    .get("ValuesMap")
                    .and_then(|vm| {
                        vm.as_object().map(|m| {
                            m.iter()
                                .map(|(k, v)| {
                                    (k.clone(), v.as_str().unwrap_or_default().to_string())
                                })
                                .collect()
                        })
                    })
                    .unwrap_or_default(),
            })
        }),
        parameters: None, // Complex nested type, pass through None for now
    })
}

/// Convert a state Job to wire DescribeJobResponse.
fn job_to_describe_response(job: &crate::types::Job) -> wire::DescribeJobResponse {
    wire::DescribeJobResponse {
        name: Some(job.name.clone()),
        created_by: Some(job.created_by.clone()),
        create_date: Some(job.create_date.timestamp() as f64),
        last_modified_by: Some(job.last_modified_by.clone()),
        last_modified_date: Some(job.last_modified_date.timestamp() as f64),
        resource_arn: Some(job.resource_arn.clone()),
        dataset_name: job.dataset_name.clone(),
        project_name: job.project_name.clone(),
        role_arn: Some(job.role_arn.clone()),
        r#type: Some(job.job_type.clone()),
        encryption_mode: job.encryption_mode.clone(),
        encryption_key_arn: job.encryption_key_arn.clone(),
        log_subscription: job.log_subscription.clone(),
        tags: job.tags.clone(),
        max_capacity: job.max_capacity,
        max_retries: job.max_retries,
        timeout: job.timeout,
        ..Default::default()
    }
}

/// Convert a state Job to wire Job.
fn job_to_wire(job: &crate::types::Job) -> wire::Job {
    wire::Job {
        name: Some(job.name.clone()),
        account_id: Some(job.account_id.clone()),
        created_by: Some(job.created_by.clone()),
        create_date: Some(job.create_date.timestamp() as f64),
        last_modified_by: Some(job.last_modified_by.clone()),
        last_modified_date: Some(job.last_modified_date.timestamp() as f64),
        resource_arn: Some(job.resource_arn.clone()),
        dataset_name: job.dataset_name.clone(),
        project_name: job.project_name.clone(),
        role_arn: Some(job.role_arn.clone()),
        r#type: Some(job.job_type.clone()),
        encryption_mode: job.encryption_mode.clone(),
        encryption_key_arn: job.encryption_key_arn.clone(),
        log_subscription: job.log_subscription.clone(),
        tags: job.tags.clone(),
        max_capacity: job.max_capacity,
        max_retries: job.max_retries,
        timeout: job.timeout,
        ..Default::default()
    }
}

fn extract_path(uri: &str) -> String {
    if let Some(idx) = uri.find("amazonaws.com") {
        let after_host = &uri[idx + "amazonaws.com".len()..];
        if let Some(q) = after_host.find('?') {
            after_host[..q].to_string()
        } else {
            after_host.to_string()
        }
    } else {
        let path_part = uri.split('?').next().unwrap_or(uri);
        path_part.to_string()
    }
}

fn extract_query_string(uri: &str) -> String {
    if let Some(idx) = uri.find('?') {
        uri[idx + 1..].to_string()
    } else {
        String::new()
    }
}

fn percent_decode(s: &str) -> String {
    urlencoding_decode(s)
}

fn urlencoding_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.bytes();
    while let Some(b) = chars.next() {
        if b == b'%' {
            let hi = chars.next().unwrap_or(b'0');
            let lo = chars.next().unwrap_or(b'0');
            let byte = hex_val(hi) * 16 + hex_val(lo);
            result.push(byte as char);
        } else if b == b'+' {
            result.push(' ');
        } else {
            result.push(b as char);
        }
    }
    result
}

fn hex_val(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'a'..=b'f' => b - b'a' + 10,
        b'A'..=b'F' => b - b'A' + 10,
        _ => 0,
    }
}

fn databrew_error_response(err: &DataBrewError) -> MockResponse {
    let (status, error_type) = match err {
        DataBrewError::DatasetAlreadyExists { .. } => (409, "AlreadyExistsException"),
        DataBrewError::DatasetNotFound => (404, "ResourceNotFoundException"),
        DataBrewError::RecipeAlreadyExists { .. } => (409, "ConflictException"),
        DataBrewError::RecipeNotFoundForVersion { .. } => (404, "ResourceNotFoundException"),
        DataBrewError::RecipeLatestPublishedNotFound { .. } => (404, "ResourceNotFoundException"),
        DataBrewError::RecipeLatestWorkingNotFound { .. } => (404, "ResourceNotFoundException"),
        DataBrewError::RecipeVersionNotFound { .. } => (404, "ResourceNotFoundException"),
        DataBrewError::RecipeNotFound { .. } => (404, "ResourceNotFoundException"),
        DataBrewError::PublishRecipeNotFound { .. } => (404, "ResourceNotFoundException"),
        DataBrewError::RecipeLatestWorkingCannotDelete => (400, "ValidationException"),
        DataBrewError::RecipeVersionCannotDelete { .. } => (400, "ValidationException"),
        DataBrewError::RulesetAlreadyExists => (409, "AlreadyExistsException"),
        DataBrewError::RulesetNotFound { .. } => (404, "EntityNotFoundException"),
        DataBrewError::ScheduleAlreadyExists { .. } => (409, "ConflictException"),
        DataBrewError::ScheduleNotFound { .. } => (404, "ResourceNotFoundException"),
        DataBrewError::JobAlreadyExists { .. } => (409, "ConflictException"),
        DataBrewError::InvalidEncryptionMode { .. } => (400, "ValidationException"),
        DataBrewError::InvalidLogSubscription { .. } => (400, "ValidationException"),
        DataBrewError::JobDescribeNotFound { .. } => (404, "ResourceNotFoundException"),
        DataBrewError::JobUpdateNotFound { .. } => (404, "ResourceNotFoundException"),
        DataBrewError::JobDeleteNotFound { .. } => (404, "ResourceNotFoundException"),
        DataBrewError::TagResourceNotFound { .. } => (404, "ResourceNotFoundException"),
    };
    let body = json!({
        "Type": "User",
        "Message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "Type": "User",
        "Message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
