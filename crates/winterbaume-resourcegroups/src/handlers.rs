use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService, extract_path, rest_json_error,
};

use crate::state::{ResourceGroupsError, ResourceGroupsState};
use crate::views::ResourceGroupsStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct ResourceGroupsService {
    pub(crate) state: Arc<BackendState<ResourceGroupsState>>,
    pub(crate) notifier: StateChangeNotifier<ResourceGroupsStateView>,
}

impl ResourceGroupsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ResourceGroupsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ResourceGroupsService {
    fn service_name(&self) -> &str {
        "resource-groups"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://resource-groups(-fips)?\..*\.amazonaws\.com",
            r"https?://resource-groups\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl ResourceGroupsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();
        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        let query_string = if let Some(pos) = request.uri.find('?') {
            &request.uri[pos + 1..]
        } else {
            ""
        };
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(query_string);

        // Handle /resources/{Arn}/tags routes where Arn is a greedy label
        if segments.first() == Some(&"resources")
            && segments.last() == Some(&"tags")
            && segments.len() >= 3
        {
            let arn_parts = &segments[1..segments.len() - 1];
            let arn_encoded = arn_parts.join("/");
            let arn_decoded = percent_decode(&arn_encoded);
            let labels: &[(&str, &str)] = &[("Arn", arn_decoded.as_str())];
            let response = match method {
                "GET" => {
                    self.handle_get_tags(&state, &request, labels, &query_map)
                        .await
                }
                "PUT" => self.handle_tag(&state, &request, labels, &query_map).await,
                "PATCH" => {
                    self.handle_untag(&state, &request, labels, &query_map)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            };
            if response.status / 100 == 2 {
                self.notify_state_changed(account_id, &region).await;
            }
            return response;
        }

        let response = match (method, segments.as_slice()) {
            // POST /groups - CreateGroup
            ("POST", ["groups"]) => {
                self.handle_create_group(&state, &request, &[], &query_map, account_id, &region)
                    .await
            }
            // POST /get-group - GetGroup
            ("POST", ["get-group"]) => {
                self.handle_get_group(&state, &request, &[], &query_map)
                    .await
            }
            // POST /delete-group - DeleteGroup
            ("POST", ["delete-group"]) => {
                self.handle_delete_group(&state, &request, &[], &query_map)
                    .await
            }
            // POST /groups-list - ListGroups
            ("POST", ["groups-list"]) => {
                self.handle_list_groups(&state, &request, &[], &query_map)
                    .await
            }
            // POST /update-group - UpdateGroup
            ("POST", ["update-group"]) => {
                self.handle_update_group(&state, &request, &[], &query_map)
                    .await
            }
            // POST /update-group-query - UpdateGroupQuery
            ("POST", ["update-group-query"]) => {
                self.handle_update_group_query(&state, &request, &[], &query_map)
                    .await
            }
            // POST /get-group-configuration - GetGroupConfiguration
            ("POST", ["get-group-configuration"]) => {
                self.handle_get_group_configuration(&state, &request, &[], &query_map)
                    .await
            }
            // POST /put-group-configuration - PutGroupConfiguration
            ("POST", ["put-group-configuration"]) => {
                self.handle_put_group_configuration(&state, &request, &[], &query_map)
                    .await
            }
            // POST /start-tag-sync-task - StartTagSyncTask
            ("POST", ["start-tag-sync-task"]) => {
                self.handle_start_tag_sync_task(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            // POST /get-tag-sync-task - GetTagSyncTask
            ("POST", ["get-tag-sync-task"]) => {
                self.handle_get_tag_sync_task(&state, &request, &[], &query_map)
                    .await
            }
            // POST /list-tag-sync-tasks - ListTagSyncTasks
            ("POST", ["list-tag-sync-tasks"]) => {
                self.handle_list_tag_sync_tasks(&state, &request, &[], &query_map)
                    .await
            }
            // POST /cancel-tag-sync-task - CancelTagSyncTask
            ("POST", ["cancel-tag-sync-task"]) => {
                self.handle_cancel_tag_sync_task(&state, &request, &[], &query_map)
                    .await
            }
            // POST /get-account-settings - GetAccountSettings
            ("POST", ["get-account-settings"]) => self.handle_get_account_settings(&state).await,
            // POST /update-account-settings - UpdateAccountSettings
            ("POST", ["update-account-settings"]) => {
                self.handle_update_account_settings(&state, &request, &[], &query_map)
                    .await
            }
            // POST /get-group-query - GetGroupQuery
            ("POST", ["get-group-query"]) => {
                self.handle_get_group_query(&state, &request, &[], &query_map)
                    .await
            }
            // POST /group-resources - GroupResources
            ("POST", ["group-resources"]) => {
                self.handle_group_resources(&state, &request, &[], &query_map)
                    .await
            }
            // POST /ungroup-resources - UngroupResources
            ("POST", ["ungroup-resources"]) => {
                self.handle_ungroup_resources(&state, &request, &[], &query_map)
                    .await
            }
            // POST /list-group-resources - ListGroupResources
            ("POST", ["list-group-resources"]) => {
                self.handle_list_group_resources(&state, &request, &[], &query_map)
                    .await
            }
            // POST /list-grouping-statuses - ListGroupingStatuses
            ("POST", ["list-grouping-statuses"]) => {
                self.handle_list_grouping_statuses(&state, &request, &[], &query_map)
                    .await
            }
            // POST /resources/search - SearchResources
            ("POST", ["resources", "search"]) => {
                self.handle_search_resources(&state, &request, &[], &query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ResourceGroupsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Name is required");
        }
        let description = input.description.unwrap_or_default();
        let (query_type, query_query) = input
            .resource_query
            .as_ref()
            .map(|q| (q.r#type.clone(), q.query.clone()))
            .unwrap_or_else(|| ("TAG_FILTERS_1_0".to_string(), "{}".to_string()));
        let tags = input.tags.unwrap_or_default();

        let mut s = state.write().await;
        match s.create_group(
            &input.name,
            &description,
            &query_type,
            &query_query,
            tags,
            account_id,
            region,
        ) {
            Ok(group) => wire::serialize_create_group_response(&wire::CreateGroupOutput {
                group: Some(wire::Group {
                    group_arn: Some(group.arn.clone()),
                    name: Some(group.name.clone()),
                    description: Some(group.description.clone()),
                    ..Default::default()
                }),
                resource_query: Some(wire::ResourceQuery {
                    r#type: group.resource_query_type.clone(),
                    query: group.resource_query_query.clone(),
                }),
                ..Default::default()
            }),
            Err(e) => rg_error_response(&e),
        }
    }

    async fn handle_get_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ResourceGroupsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = input.group.or(input.group_name).unwrap_or_default();
        let s = state.read().await;
        match s.get_group(&name) {
            Ok(group) => wire::serialize_get_group_response(&wire::GetGroupOutput {
                group: Some(wire::Group {
                    group_arn: Some(group.arn.clone()),
                    name: Some(group.name.clone()),
                    description: Some(group.description.clone()),
                    ..Default::default()
                }),
            }),
            Err(e) => rg_error_response(&e),
        }
    }

    async fn handle_delete_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ResourceGroupsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = input.group.or(input.group_name).unwrap_or_default();
        let mut s = state.write().await;
        match s.delete_group(&name) {
            Ok(group) => wire::serialize_delete_group_response(&wire::DeleteGroupOutput {
                group: Some(wire::Group {
                    group_arn: Some(group.arn.clone()),
                    name: Some(group.name.clone()),
                    description: Some(group.description.clone()),
                    ..Default::default()
                }),
            }),
            Err(e) => rg_error_response(&e),
        }
    }

    async fn handle_list_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<ResourceGroupsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_list_groups_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        let groups = s.list_groups();

        let group_identifiers: Vec<wire::GroupIdentifier> = groups
            .iter()
            .map(|g| wire::GroupIdentifier {
                group_name: Some(g.name.clone()),
                group_arn: Some(g.arn.clone()),
                ..Default::default()
            })
            .collect();

        let group_entries: Vec<wire::Group> = groups
            .iter()
            .map(|g| wire::Group {
                group_arn: Some(g.arn.clone()),
                name: Some(g.name.clone()),
                description: Some(g.description.clone()),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_groups_response(&wire::ListGroupsOutput {
            group_identifiers: Some(group_identifiers),
            groups: Some(group_entries),
            ..Default::default()
        })
    }

    async fn handle_update_group(
        &self,
        state: &Arc<tokio::sync::RwLock<ResourceGroupsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = match input.group.or(input.group_name) {
            Some(n) if !n.is_empty() => n,
            _ => return rest_json_error(400, "BadRequestException", "Group name is required"),
        };

        let mut s = state.write().await;
        match s.update_group(&name, input.description.as_deref()) {
            Ok(group) => wire::serialize_update_group_response(&wire::UpdateGroupOutput {
                group: Some(wire::Group {
                    group_arn: Some(group.arn.clone()),
                    name: Some(group.name.clone()),
                    description: Some(group.description.clone()),
                    ..Default::default()
                }),
            }),
            Err(e) => rg_error_response(&e),
        }
    }

    async fn handle_update_group_query(
        &self,
        state: &Arc<tokio::sync::RwLock<ResourceGroupsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_group_query_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = match input.group.or(input.group_name) {
            Some(n) if !n.is_empty() => n,
            _ => return rest_json_error(400, "BadRequestException", "Group name is required"),
        };
        let query_type = if input.resource_query.r#type.is_empty() {
            "TAG_FILTERS_1_0".to_string()
        } else {
            input.resource_query.r#type.clone()
        };
        let query_query = if input.resource_query.query.is_empty() {
            "{}".to_string()
        } else {
            input.resource_query.query.clone()
        };

        let mut s = state.write().await;
        match s.update_group_query(&name, &query_type, &query_query) {
            Ok(group) => {
                wire::serialize_update_group_query_response(&wire::UpdateGroupQueryOutput {
                    group_query: Some(wire::GroupQuery {
                        group_name: Some(group.name.clone()),
                        resource_query: Some(wire::ResourceQuery {
                            r#type: group.resource_query_type.clone(),
                            query: group.resource_query_query.clone(),
                        }),
                    }),
                })
            }
            Err(e) => rg_error_response(&e),
        }
    }

    async fn handle_get_group_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<ResourceGroupsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_group_configuration_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = input.group.unwrap_or_default();

        let s = state.read().await;
        match s.get_group_configuration(&name) {
            Ok(group) => {
                let config = group
                    .configuration
                    .as_ref()
                    .map(|items| wire::GroupConfiguration {
                        configuration: Some(
                            items
                                .iter()
                                .map(|item| wire::GroupConfigurationItem {
                                    r#type: item.config_type.clone(),
                                    parameters: Some(
                                        item.parameters
                                            .iter()
                                            .map(|p| wire::GroupConfigurationParameter {
                                                name: p.name.clone(),
                                                values: Some(p.values.clone()),
                                            })
                                            .collect(),
                                    ),
                                })
                                .collect(),
                        ),
                        status: Some("UPDATE_COMPLETE".to_string()),
                        ..Default::default()
                    });
                wire::serialize_get_group_configuration_response(
                    &wire::GetGroupConfigurationOutput {
                        group_configuration: config,
                    },
                )
            }
            Err(e) => rg_error_response(&e),
        }
    }

    async fn handle_put_group_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<ResourceGroupsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_group_configuration_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = match input.group {
            Some(n) if !n.is_empty() => n,
            _ => return rest_json_error(400, "BadRequestException", "Group name is required"),
        };

        let configuration = input.configuration.map(|items| {
            items
                .into_iter()
                .map(|item| crate::types::GroupConfigurationEntry {
                    config_type: item.r#type,
                    parameters: item
                        .parameters
                        .unwrap_or_default()
                        .into_iter()
                        .map(|p| crate::types::GroupConfigParam {
                            name: p.name,
                            values: p.values.unwrap_or_default(),
                        })
                        .collect(),
                })
                .collect()
        });

        let mut s = state.write().await;
        match s.put_group_configuration(&name, configuration) {
            Ok(()) => wire::serialize_put_group_configuration_response(
                &wire::PutGroupConfigurationOutput {},
            ),
            Err(e) => rg_error_response(&e),
        }
    }

    async fn handle_get_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<ResourceGroupsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_tags_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.get_tags(&input.arn) {
            Ok(group) => wire::serialize_get_tags_response(&wire::GetTagsOutput {
                arn: Some(group.arn.clone()),
                tags: Some(group.tags.clone()),
            }),
            Err(e) => rg_error_response(&e),
        }
    }

    async fn handle_tag(
        &self,
        state: &Arc<tokio::sync::RwLock<ResourceGroupsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };

        let mut s = state.write().await;
        match s.tag_resource(&input.arn, &input.tags) {
            Ok(group) => wire::serialize_tag_response(&wire::TagOutput {
                arn: Some(group.arn.clone()),
                tags: Some(input.tags),
            }),
            Err(e) => rg_error_response(&e),
        }
    }

    async fn handle_untag(
        &self,
        state: &Arc<tokio::sync::RwLock<ResourceGroupsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };

        let mut s = state.write().await;
        match s.untag_resource(&input.arn, &input.keys) {
            Ok(group) => wire::serialize_untag_response(&wire::UntagOutput {
                arn: Some(group.arn.clone()),
                keys: Some(input.keys),
            }),
            Err(e) => rg_error_response(&e),
        }
    }

    async fn handle_start_tag_sync_task(
        &self,
        state: &Arc<tokio::sync::RwLock<ResourceGroupsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_start_tag_sync_task_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.group.is_empty() {
            return rest_json_error(400, "BadRequestException", "Group is required");
        }
        let tag_key = match input.tag_key.as_deref() {
            Some(k) if !k.is_empty() => k,
            _ => return rest_json_error(400, "BadRequestException", "TagKey is required"),
        };
        let tag_value = input.tag_value.as_deref().unwrap_or("");
        if input.role_arn.is_empty() {
            return rest_json_error(400, "BadRequestException", "RoleArn is required");
        }

        let mut s = state.write().await;
        match s.start_tag_sync_task(
            &input.group,
            tag_key,
            tag_value,
            &input.role_arn,
            account_id,
            region,
        ) {
            Ok(task) => {
                wire::serialize_start_tag_sync_task_response(&wire::StartTagSyncTaskOutput {
                    group_arn: Some(task.group_arn.clone()),
                    group_name: Some(task.group_name.clone()),
                    tag_key: Some(task.tag_key.clone()),
                    tag_value: Some(task.tag_value.clone()),
                    role_arn: Some(task.role_arn.clone()),
                    task_arn: Some(task.task_arn.clone()),
                    ..Default::default()
                })
            }
            Err(e) => rg_error_response(&e),
        }
    }

    async fn handle_get_tag_sync_task(
        &self,
        state: &Arc<tokio::sync::RwLock<ResourceGroupsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_tag_sync_task_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.task_arn.is_empty() {
            return rest_json_error(400, "BadRequestException", "TaskArn is required");
        }

        let s = state.read().await;
        match s.get_tag_sync_task(&input.task_arn) {
            Ok(task) => wire::serialize_get_tag_sync_task_response(&wire::GetTagSyncTaskOutput {
                task_arn: Some(task.task_arn.clone()),
                group_arn: Some(task.group_arn.clone()),
                group_name: Some(task.group_name.clone()),
                tag_key: Some(task.tag_key.clone()),
                tag_value: Some(task.tag_value.clone()),
                role_arn: Some(task.role_arn.clone()),
                status: Some(task.status.clone()),
                created_at: Some(task.created_at),
                ..Default::default()
            }),
            Err(e) => rg_error_response(&e),
        }
    }

    async fn handle_list_tag_sync_tasks(
        &self,
        state: &Arc<tokio::sync::RwLock<ResourceGroupsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tag_sync_tasks_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let group_filter = input.filters.as_ref().and_then(|filters| {
            filters
                .iter()
                .find_map(|f| f.group_arn.clone().or_else(|| f.group_name.clone()))
        });

        let s = state.read().await;
        let tasks = s.list_tag_sync_tasks(group_filter.as_deref());

        let items: Vec<wire::TagSyncTaskItem> = tasks
            .iter()
            .map(|t| wire::TagSyncTaskItem {
                task_arn: Some(t.task_arn.clone()),
                group_arn: Some(t.group_arn.clone()),
                group_name: Some(t.group_name.clone()),
                tag_key: Some(t.tag_key.clone()),
                tag_value: Some(t.tag_value.clone()),
                role_arn: Some(t.role_arn.clone()),
                status: Some(t.status.clone()),
                created_at: Some(t.created_at),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_tag_sync_tasks_response(&wire::ListTagSyncTasksOutput {
            tag_sync_tasks: Some(items),
            ..Default::default()
        })
    }

    async fn handle_cancel_tag_sync_task(
        &self,
        state: &Arc<tokio::sync::RwLock<ResourceGroupsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_tag_sync_task_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.task_arn.is_empty() {
            return rest_json_error(400, "BadRequestException", "TaskArn is required");
        }

        let mut s = state.write().await;
        match s.cancel_tag_sync_task(&input.task_arn) {
            Ok(()) => wire::serialize_cancel_tag_sync_task_response(),
            Err(e) => rg_error_response(&e),
        }
    }

    async fn handle_get_account_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<ResourceGroupsState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let settings = s.get_account_settings();
        let desired = if settings.group_lifecycle_events_desired_status.is_empty() {
            "ACTIVE".to_string()
        } else {
            settings.group_lifecycle_events_desired_status.clone()
        };
        wire::serialize_get_account_settings_response(&wire::GetAccountSettingsOutput {
            account_settings: Some(wire::AccountSettings {
                group_lifecycle_events_desired_status: Some(desired),
                ..Default::default()
            }),
        })
    }

    async fn handle_update_account_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<ResourceGroupsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_account_settings_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let desired_status = input.group_lifecycle_events_desired_status.as_deref();

        let mut s = state.write().await;
        let settings = s.update_account_settings(desired_status);
        let desired = if settings.group_lifecycle_events_desired_status.is_empty() {
            "ACTIVE".to_string()
        } else {
            settings.group_lifecycle_events_desired_status.clone()
        };
        wire::serialize_update_account_settings_response(&wire::UpdateAccountSettingsOutput {
            account_settings: Some(wire::AccountSettings {
                group_lifecycle_events_desired_status: Some(desired),
                ..Default::default()
            }),
        })
    }

    async fn handle_get_group_query(
        &self,
        state: &Arc<tokio::sync::RwLock<ResourceGroupsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_group_query_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = match input.group.or(input.group_name) {
            Some(n) if !n.is_empty() => n,
            _ => return rest_json_error(400, "BadRequestException", "Group name is required"),
        };

        let s = state.read().await;
        match s.get_group_query(&name) {
            Ok(group) => wire::serialize_get_group_query_response(&wire::GetGroupQueryOutput {
                group_query: Some(wire::GroupQuery {
                    group_name: Some(group.name.clone()),
                    resource_query: Some(wire::ResourceQuery {
                        r#type: group.resource_query_type.clone(),
                        query: group.resource_query_query.clone(),
                    }),
                }),
            }),
            Err(e) => rg_error_response(&e),
        }
    }

    async fn handle_group_resources(
        &self,
        state: &Arc<tokio::sync::RwLock<ResourceGroupsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_group_resources_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.group.is_empty() {
            return rest_json_error(400, "BadRequestException", "Group name is required");
        }

        let mut s = state.write().await;
        match s.group_resources(&input.group, input.resource_arns) {
            Ok(succeeded) => {
                wire::serialize_group_resources_response(&wire::GroupResourcesOutput {
                    succeeded: if succeeded.is_empty() {
                        None
                    } else {
                        Some(succeeded)
                    },
                    failed: None,
                    pending: None,
                })
            }
            Err(e) => rg_error_response(&e),
        }
    }

    async fn handle_ungroup_resources(
        &self,
        state: &Arc<tokio::sync::RwLock<ResourceGroupsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_ungroup_resources_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.group.is_empty() {
            return rest_json_error(400, "BadRequestException", "Group name is required");
        }

        let mut s = state.write().await;
        match s.ungroup_resources(&input.group, input.resource_arns) {
            Ok(succeeded) => {
                wire::serialize_ungroup_resources_response(&wire::UngroupResourcesOutput {
                    succeeded: if succeeded.is_empty() {
                        None
                    } else {
                        Some(succeeded)
                    },
                    failed: None,
                    pending: None,
                })
            }
            Err(e) => rg_error_response(&e),
        }
    }

    async fn handle_list_group_resources(
        &self,
        state: &Arc<tokio::sync::RwLock<ResourceGroupsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_group_resources_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = match input.group.or(input.group_name) {
            Some(n) if !n.is_empty() => n,
            _ => return rest_json_error(400, "BadRequestException", "Group name is required"),
        };

        let s = state.read().await;
        match s.list_group_resources(&name) {
            Ok(arns) => {
                let resource_identifiers: Vec<wire::ResourceIdentifier> = arns
                    .iter()
                    .map(|arn| wire::ResourceIdentifier {
                        resource_arn: Some(arn.clone()),
                        resource_type: None,
                    })
                    .collect();
                let resources: Vec<wire::ListGroupResourcesItem> = arns
                    .iter()
                    .map(|arn| wire::ListGroupResourcesItem {
                        identifier: Some(wire::ResourceIdentifier {
                            resource_arn: Some(arn.clone()),
                            resource_type: None,
                        }),
                        status: None,
                    })
                    .collect();
                wire::serialize_list_group_resources_response(&wire::ListGroupResourcesOutput {
                    resource_identifiers: if resource_identifiers.is_empty() {
                        None
                    } else {
                        Some(resource_identifiers)
                    },
                    resources: if resources.is_empty() {
                        None
                    } else {
                        Some(resources)
                    },
                    ..Default::default()
                })
            }
            Err(e) => rg_error_response(&e),
        }
    }

    async fn handle_list_grouping_statuses(
        &self,
        state: &Arc<tokio::sync::RwLock<ResourceGroupsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_grouping_statuses_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.group.is_empty() {
            return rest_json_error(400, "BadRequestException", "Group name is required");
        }

        let s = state.read().await;
        match s.get_group(&input.group) {
            Ok(_) => {
                wire::serialize_list_grouping_statuses_response(&wire::ListGroupingStatusesOutput {
                    group: Some(input.group.clone()),
                    grouping_statuses: Some(Vec::new()),
                    next_token: None,
                })
            }
            Err(e) => rg_error_response(&e),
        }
    }

    // STUB[no-engine]: SearchResources queries tag-based filters across all AWS resource types
    //   in the account; the mock has no cross-service resource index to satisfy these queries.
    async fn handle_search_resources(
        &self,
        _state: &Arc<tokio::sync::RwLock<ResourceGroupsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_search_resources_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        wire::serialize_search_resources_response(&wire::SearchResourcesOutput {
            resource_identifiers: Some(Vec::new()),
            ..Default::default()
        })
    }
}

fn percent_decode(s: &str) -> String {
    urlencoding::decode(s)
        .unwrap_or_else(|_| s.into())
        .into_owned()
}

fn rg_error_response(err: &ResourceGroupsError) -> MockResponse {
    let (status, error_type) = match err {
        ResourceGroupsError::GroupAlreadyExists { .. } => (400, "BadRequestException"),
        ResourceGroupsError::GroupNotFound { .. } => (404, "NotFoundException"),
        ResourceGroupsError::ResourceNotFound { .. } => (404, "NotFoundException"),
        ResourceGroupsError::TagSyncTaskNotFound { .. } => (404, "NotFoundException"),
    };
    let body = json!({
        "Message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
