use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    extract_path, percent_decode, rest_json_error,
};

use crate::state::{ManagerRecord, ServiceSettingsRecord, SsmQuickSetupError, SsmQuickSetupState};
use crate::views::SsmQuickSetupStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct SsmQuickSetupService {
    pub(crate) state: Arc<BackendState<SsmQuickSetupState>>,
    pub(crate) notifier: StateChangeNotifier<SsmQuickSetupStateView>,
}

impl SsmQuickSetupService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for SsmQuickSetupService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SsmQuickSetupService {
    fn service_name(&self) -> &str {
        "ssm-quicksetup"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://ssm-quicksetup\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<SsmQuickSetupState>>;

impl SsmQuickSetupService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
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
            ("POST", ["configurationManager"]) => {
                self.handle_create_manager(&state, account_id, &region, &body)
                    .await
            }
            ("GET", ["configurationManager", arn]) => self.handle_get_manager(&state, arn).await,
            ("DELETE", ["configurationManager", arn]) => {
                self.handle_delete_manager(&state, arn).await
            }
            ("PUT", ["configurationManager", arn]) => {
                self.handle_update_manager(&state, arn, &body).await
            }
            ("PUT", ["configurationDefinition", arn, definition_id]) => {
                self.handle_update_definition(&state, arn, definition_id, &body)
                    .await
            }
            ("GET", ["getConfiguration", id]) => self.handle_get_configuration(&state, id).await,
            ("POST", ["listConfigurationManagers"]) => {
                self.handle_list_managers(&state, &body).await
            }
            ("POST", ["listConfigurations"]) => {
                self.handle_list_configurations(&state, &body).await
            }
            ("GET", ["serviceSettings"]) => self.handle_get_service_settings(&state).await,
            ("PUT", ["serviceSettings"]) => {
                self.handle_update_service_settings(&state, &body).await
            }
            ("GET", ["listQuickSetupTypes"]) => self.handle_list_types().await,
            ("GET", ["tags", arn]) => self.handle_list_tags(&state, arn).await,
            ("PUT", ["tags", arn]) => self.handle_tag_resource(&state, arn, &body).await,
            ("DELETE", ["tags", arn]) => {
                self.handle_untag_resource(&state, arn, &request.uri).await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        let read_only = matches!(method.as_str(), "GET")
            || matches!(
                segs.as_slice(),
                ["listConfigurationManagers"] | ["listConfigurations"] | ["listQuickSetupTypes"]
            );
        if response.status / 100 == 2 && !read_only {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_manager(
        &self,
        state: &SharedState,
        account_id: &str,
        region: &str,
        body: &Value,
    ) -> MockResponse {
        let id = uuid::Uuid::new_v4().simple().to_string();
        let arn =
            format!("arn:aws:ssm-quicksetup:{region}:{account_id}:configuration-manager/{id}");
        let now = chrono::Utc::now().to_rfc3339();
        let mut definitions: Vec<Value> = body
            .get("ConfigurationDefinitions")
            .and_then(|v| v.as_array())
            .map(|a| a.to_vec())
            .unwrap_or_default();
        for def in &mut definitions {
            if let Some(obj) = def.as_object_mut() {
                let id = format!("def-{}", uuid::Uuid::new_v4().simple());
                obj.insert("Id".to_string(), Value::String(id));
            }
        }
        let tags = body
            .get("Tags")
            .and_then(|v| v.as_object())
            .map(|m| {
                m.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect::<HashMap<String, String>>()
            })
            .unwrap_or_default();
        let record = ManagerRecord {
            arn: arn.clone(),
            name: body.get("Name").and_then(|v| v.as_str()).map(String::from),
            description: body
                .get("Description")
                .and_then(|v| v.as_str())
                .map(String::from),
            created_at: now.clone(),
            last_modified_at: now,
            status_summary: "INITIALIZING".to_string(),
            definitions,
        };
        let mut state = state.write().await;
        state.create_manager(record);
        if !tags.is_empty() {
            state.tag_resource(&arn, tags);
        }
        wire::serialize_create_configuration_manager_response(
            &wire::CreateConfigurationManagerOutput {
                manager_arn: Some(arn),
            },
        )
    }

    async fn handle_get_manager(&self, state: &SharedState, arn: &str) -> MockResponse {
        let state = state.read().await;
        match state.get_manager(arn) {
            Ok(m) => {
                let definitions: Vec<wire::ConfigurationDefinition> = m
                    .definitions
                    .iter()
                    .filter_map(|v| serde_json::from_value(v.clone()).ok())
                    .collect();
                let tags = state.list_tags(arn);
                let tags_wire: Option<HashMap<String, String>> =
                    if tags.is_empty() { None } else { Some(tags) };
                wire::serialize_get_configuration_manager_response(
                    &wire::GetConfigurationManagerOutput {
                        configuration_definitions: Some(definitions),
                        created_at: Some(m.created_at.clone()),
                        description: m.description.clone(),
                        last_modified_at: Some(m.last_modified_at.clone()),
                        manager_arn: Some(m.arn.clone()),
                        name: m.name.clone(),
                        status_summaries: Some(vec![]),
                        tags: tags_wire,
                    },
                )
            }
            Err(e) => err_response(&e),
        }
    }

    async fn handle_delete_manager(&self, state: &SharedState, arn: &str) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_manager(arn) {
            Ok(()) => wire::serialize_delete_configuration_manager_response(),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_update_manager(
        &self,
        state: &SharedState,
        arn: &str,
        body: &Value,
    ) -> MockResponse {
        let new_name = body.get("Name").and_then(|v| v.as_str()).map(String::from);
        let new_desc = body
            .get("Description")
            .and_then(|v| v.as_str())
            .map(String::from);
        let mut state = state.write().await;
        let now = chrono::Utc::now().to_rfc3339();
        match state.get_manager_mut(arn) {
            Ok(m) => {
                if let Some(n) = new_name {
                    m.name = Some(n);
                }
                if let Some(d) = new_desc {
                    m.description = Some(d);
                }
                m.last_modified_at = now;
                wire::serialize_update_configuration_manager_response()
            }
            Err(e) => err_response(&e),
        }
    }

    async fn handle_update_definition(
        &self,
        state: &SharedState,
        arn: &str,
        definition_id: &str,
        body: &Value,
    ) -> MockResponse {
        let new_params: Option<HashMap<String, String>> =
            body.get("Parameters").and_then(|v| v.as_object()).map(|m| {
                m.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            });
        let mut state = state.write().await;
        let now = chrono::Utc::now().to_rfc3339();
        let m = match state.get_manager_mut(arn) {
            Ok(m) => m,
            Err(e) => return err_response(&e),
        };
        let def = m
            .definitions
            .iter_mut()
            .find(|d| d.get("Id").and_then(|v| v.as_str()) == Some(definition_id));
        match def {
            Some(d) => {
                if let Some(params) = new_params {
                    if let Some(obj) = d.as_object_mut() {
                        let params_value = serde_json::to_value(params).unwrap_or(json!({}));
                        obj.insert("Parameters".to_string(), params_value);
                    }
                }
                m.last_modified_at = now;
                wire::serialize_update_configuration_definition_response()
            }
            None => err_response(&SsmQuickSetupError::DefinitionNotFound {
                arn: arn.to_string(),
                id: definition_id.to_string(),
            }),
        }
    }

    async fn handle_get_configuration(&self, state: &SharedState, id: &str) -> MockResponse {
        let state = state.read().await;
        match state.find_definition_arn(id) {
            Some((manager_arn, def)) => {
                let cfg_type = def.get("Type").and_then(|v| v.as_str()).map(String::from);
                wire::serialize_get_configuration_response(&wire::GetConfigurationOutput {
                    account: Some(winterbaume_core::default_account_id().to_string()),
                    configuration_definition_id: Some(id.to_string()),
                    created_at: Some(chrono::Utc::now().to_rfc3339()),
                    id: Some(id.to_string()),
                    last_modified_at: Some(chrono::Utc::now().to_rfc3339()),
                    manager_arn: Some(manager_arn.to_string()),
                    parameters: def
                        .get("Parameters")
                        .and_then(|v| serde_json::from_value(v.clone()).ok()),
                    region: None,
                    status_summaries: Some(vec![]),
                    r#type: cfg_type,
                    type_version: def
                        .get("TypeVersion")
                        .and_then(|v| v.as_str())
                        .map(String::from),
                })
            }
            None => err_response(&SsmQuickSetupError::DefinitionNotFound {
                arn: String::new(),
                id: id.to_string(),
            }),
        }
    }

    async fn handle_list_managers(&self, state: &SharedState, _body: &Value) -> MockResponse {
        let state = state.read().await;
        let summaries: Vec<wire::ConfigurationManagerSummary> = state
            .list_managers()
            .into_iter()
            .map(|m| wire::ConfigurationManagerSummary {
                configuration_definition_summaries: Some(
                    m.definitions
                        .iter()
                        .filter_map(|d| serde_json::from_value(d.clone()).ok())
                        .collect(),
                ),
                description: m.description.clone(),
                manager_arn: Some(m.arn.clone()),
                name: m.name.clone(),
                status_summaries: Some(vec![]),
            })
            .collect();
        wire::serialize_list_configuration_managers_response(
            &wire::ListConfigurationManagersOutput {
                configuration_managers_list: Some(summaries),
                next_token: None,
            },
        )
    }

    async fn handle_list_configurations(&self, state: &SharedState, _body: &Value) -> MockResponse {
        let state = state.read().await;
        let summaries: Vec<wire::ConfigurationSummary> = state
            .list_all_definitions()
            .into_iter()
            .map(|d| wire::ConfigurationSummary {
                account: Some(winterbaume_core::default_account_id().to_string()),
                configuration_definition_id: d.get("Id").and_then(|v| v.as_str()).map(String::from),
                created_at: Some(chrono::Utc::now().to_rfc3339()),
                first_class_parameters: d
                    .get("Parameters")
                    .and_then(|v| serde_json::from_value(v.clone()).ok()),
                id: d.get("Id").and_then(|v| v.as_str()).map(String::from),
                manager_arn: None,
                region: None,
                status_summaries: Some(vec![]),
                r#type: d.get("Type").and_then(|v| v.as_str()).map(String::from),
                type_version: d
                    .get("TypeVersion")
                    .and_then(|v| v.as_str())
                    .map(String::from),
            })
            .collect();
        wire::serialize_list_configurations_response(&wire::ListConfigurationsOutput {
            configurations_list: Some(summaries),
            next_token: None,
        })
    }

    async fn handle_list_types(&self) -> MockResponse {
        wire::serialize_list_quick_setup_types_response(&wire::ListQuickSetupTypesOutput {
            quick_setup_type_list: Some(vec![
                wire::QuickSetupTypeOutput {
                    latest_version: Some("1.0".to_string()),
                    r#type: Some("AWSConfigSetup".to_string()),
                },
                wire::QuickSetupTypeOutput {
                    latest_version: Some("1.0".to_string()),
                    r#type: Some("AWSPatchManager".to_string()),
                },
            ]),
        })
    }

    async fn handle_get_service_settings(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        wire::serialize_get_service_settings_response(&wire::GetServiceSettingsOutput {
            service_settings: Some(wire::ServiceSettings {
                explorer_enabling_role_arn: state
                    .service_settings
                    .explorer_enabling_role_arn
                    .clone(),
            }),
        })
    }

    async fn handle_update_service_settings(
        &self,
        state: &SharedState,
        body: &Value,
    ) -> MockResponse {
        let role_arn = body
            .get("ExplorerEnablingRoleArn")
            .and_then(|v| v.as_str())
            .map(String::from);
        let mut state = state.write().await;
        state.service_settings = ServiceSettingsRecord {
            explorer_enabling_role_arn: role_arn,
        };
        wire::serialize_update_service_settings_response()
    }

    async fn handle_list_tags(&self, state: &SharedState, arn: &str) -> MockResponse {
        let state = state.read().await;
        let tags = state.list_tags(arn);
        let entries: Vec<wire::TagEntry> = tags
            .into_iter()
            .map(|(k, v)| wire::TagEntry {
                key: Some(k),
                value: Some(v),
            })
            .collect();
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: if entries.is_empty() {
                None
            } else {
                Some(entries)
            },
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &SharedState,
        arn: &str,
        body: &Value,
    ) -> MockResponse {
        let tags: HashMap<String, String> = body
            .get("Tags")
            .and_then(|v| v.as_object())
            .map(|m| {
                m.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();
        if tags.is_empty() {
            return rest_json_error(400, "ValidationException", "Tags is required");
        }
        let mut state = state.write().await;
        state.tag_resource(arn, tags);
        wire::serialize_tag_resource_response()
    }

    async fn handle_untag_resource(
        &self,
        state: &SharedState,
        arn: &str,
        uri: &str,
    ) -> MockResponse {
        let keys: Vec<String> = winterbaume_core::extract_query_string(uri)
            .split('&')
            .filter_map(|pair| pair.split_once('='))
            .filter(|(k, _)| *k == "tagKeys")
            .map(|(_, v)| winterbaume_core::percent_decode(v))
            .collect();
        let mut state = state.write().await;
        state.untag_resource(arn, &keys);
        wire::serialize_untag_resource_response()
    }
}

fn err_response(err: &SsmQuickSetupError) -> MockResponse {
    let (status, error_type) = match err {
        SsmQuickSetupError::ManagerNotFound { .. }
        | SsmQuickSetupError::DefinitionNotFound { .. } => (404, "ResourceNotFoundException"),
        SsmQuickSetupError::Validation { .. } => (400, "ValidationException"),
    };
    let body = json!({"Message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
