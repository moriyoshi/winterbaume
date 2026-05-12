use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::model;
use crate::state::{LakeFormationError, LakeFormationState};
use crate::types;
use crate::views::LakeFormationStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct LakeFormationService {
    pub(crate) state: Arc<BackendState<LakeFormationState>>,
    pub(crate) notifier: StateChangeNotifier<LakeFormationStateView>,
}

impl LakeFormationService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for LakeFormationService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for LakeFormationService {
    fn service_name(&self) -> &str {
        "lakeformation"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://lakeformation\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl LakeFormationService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();
        let labels: &[(&str, &str)] = &[];
        let query: HashMap<String, String> = HashMap::new();

        match (method, path.as_str()) {
            ("POST", "/RegisterResource") => {
                self.handle_register_resource(&state, &request, labels, &query)
                    .await
            }
            ("POST", "/DeregisterResource") => {
                self.handle_deregister_resource(&state, &request, labels, &query)
                    .await
            }
            ("POST", "/DescribeResource") => {
                self.handle_describe_resource(&state, &request, labels, &query)
                    .await
            }
            ("POST", "/ListResources") => self.handle_list_resources(&state).await,
            ("POST", "/GetDataLakeSettings") => self.handle_get_data_lake_settings(&state).await,
            ("POST", "/PutDataLakeSettings") => {
                self.handle_put_data_lake_settings(&state, &request, labels, &query)
                    .await
            }
            ("POST", "/CreateLFTag") => {
                self.handle_create_lf_tag(&state, &request, labels, &query)
                    .await
            }
            ("POST", "/DeleteLFTag") => {
                self.handle_delete_lf_tag(&state, &request, labels, &query)
                    .await
            }
            ("POST", "/GetLFTag") => {
                self.handle_get_lf_tag(&state, &request, labels, &query)
                    .await
            }
            ("POST", "/ListLFTags") => self.handle_list_lf_tags(&state).await,
            ("POST", "/AddLFTagsToResource") => {
                self.handle_add_lf_tags_to_resource(&state, &request, labels, &query)
                    .await
            }
            ("POST", "/RemoveLFTagsFromResource") => {
                self.handle_remove_lf_tags_from_resource(&state, &request, labels, &query)
                    .await
            }
            ("POST", "/UpdateLFTag") => {
                self.handle_update_lf_tag(&state, &request, labels, &query)
                    .await
            }
            ("POST", "/GetResourceLFTags") => {
                self.handle_get_resource_lf_tags(&state, &request, labels, &query)
                    .await
            }
            ("POST", "/GrantPermissions") => {
                self.handle_grant_permissions(&state, &request, labels, &query)
                    .await
            }
            ("POST", "/RevokePermissions") => {
                self.handle_revoke_permissions(&state, &request, labels, &query)
                    .await
            }
            ("POST", "/BatchGrantPermissions") => {
                self.handle_batch_grant_permissions(&state, &request, labels, &query)
                    .await
            }
            ("POST", "/BatchRevokePermissions") => {
                self.handle_batch_revoke_permissions(&state, &request, labels, &query)
                    .await
            }
            ("POST", "/ListPermissions") => {
                self.handle_list_permissions(&state, &request, labels, &query)
                    .await
            }
            ("POST", "/ListDataCellsFilter") => self.handle_list_data_cells_filter().await,
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn handle_register_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<LakeFormationState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_register_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return rest_json_error(400, "InvalidInputException", "Missing 'ResourceArn'");
        }

        let mut state = state.write().await;
        match state.register_resource(
            &input.resource_arn,
            input.role_arn.as_deref(),
            input.use_service_linked_role.unwrap_or(false),
        ) {
            Ok(()) => {
                wire::serialize_register_resource_response(&model::RegisterResourceResponse {})
            }
            Err(e) => lf_error_response(&e),
        }
    }

    async fn handle_deregister_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<LakeFormationState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_deregister_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return rest_json_error(400, "InvalidInputException", "Missing 'ResourceArn'");
        }

        let mut state = state.write().await;
        match state.deregister_resource(&input.resource_arn) {
            Ok(()) => {
                wire::serialize_deregister_resource_response(&model::DeregisterResourceResponse {})
            }
            Err(e) => lf_error_response(&e),
        }
    }

    async fn handle_describe_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<LakeFormationState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return rest_json_error(400, "InvalidInputException", "Missing 'ResourceArn'");
        }

        let state = state.read().await;
        match state.describe_resource(&input.resource_arn) {
            Ok(r) => wire::serialize_describe_resource_response(&model::DescribeResourceResponse {
                resource_info: Some(model::ResourceInfo {
                    resource_arn: Some(r.resource_arn.clone()),
                    role_arn: r.role_arn.clone(),
                    ..Default::default()
                }),
            }),
            Err(e) => lf_error_response(&e),
        }
    }

    async fn handle_list_resources(
        &self,
        state: &Arc<tokio::sync::RwLock<LakeFormationState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let resources = state.list_resources();
        let entries: Vec<model::ResourceInfo> = resources
            .iter()
            .map(|r| model::ResourceInfo {
                resource_arn: Some(r.resource_arn.clone()),
                role_arn: r.role_arn.clone(),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_resources_response(&model::ListResourcesResponse {
            resource_info_list: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_get_data_lake_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<LakeFormationState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let settings = state.get_data_lake_settings();

        let admins: Vec<model::DataLakePrincipal> = settings
            .data_lake_admins
            .iter()
            .map(|a| model::DataLakePrincipal {
                data_lake_principal_identifier: Some(a.data_lake_principal_identifier.clone()),
            })
            .collect();

        let create_db_perms: Vec<model::PrincipalPermissions> = settings
            .create_database_default_permissions
            .iter()
            .map(|p| model::PrincipalPermissions {
                principal: Some(model::DataLakePrincipal {
                    data_lake_principal_identifier: Some(
                        p.principal.data_lake_principal_identifier.clone(),
                    ),
                }),
                permissions: Some(p.permissions.clone()),
            })
            .collect();

        let create_table_perms: Vec<model::PrincipalPermissions> = settings
            .create_table_default_permissions
            .iter()
            .map(|p| model::PrincipalPermissions {
                principal: Some(model::DataLakePrincipal {
                    data_lake_principal_identifier: Some(
                        p.principal.data_lake_principal_identifier.clone(),
                    ),
                }),
                permissions: Some(p.permissions.clone()),
            })
            .collect();

        wire::serialize_get_data_lake_settings_response(&model::GetDataLakeSettingsResponse {
            data_lake_settings: Some(model::DataLakeSettings {
                data_lake_admins: Some(admins),
                allow_external_data_filtering: Some(settings.allow_external_data_filtering),
                authorized_session_tag_value_list: Some(
                    settings.authorized_session_tag_value_list.clone(),
                ),
                create_database_default_permissions: Some(create_db_perms),
                create_table_default_permissions: Some(create_table_perms),
                ..Default::default()
            }),
        })
    }

    async fn handle_put_data_lake_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<LakeFormationState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_data_lake_settings_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let dls = input.data_lake_settings;

        let admins: Vec<types::DataLakePrincipal> = dls
            .data_lake_admins
            .unwrap_or_default()
            .into_iter()
            .filter_map(|p| {
                p.data_lake_principal_identifier
                    .map(|id| types::DataLakePrincipal {
                        data_lake_principal_identifier: id,
                    })
            })
            .collect();

        let allow_ext = dls.allow_external_data_filtering.unwrap_or(false);
        let auth_tags = dls.authorized_session_tag_value_list.unwrap_or_default();
        let create_db_perms = principal_permissions_from_model(
            dls.create_database_default_permissions.unwrap_or_default(),
        );
        let create_table_perms = principal_permissions_from_model(
            dls.create_table_default_permissions.unwrap_or_default(),
        );

        let settings = types::DataLakeSettings {
            data_lake_admins: admins,
            allow_external_data_filtering: allow_ext,
            authorized_session_tag_value_list: auth_tags,
            create_database_default_permissions: create_db_perms,
            create_table_default_permissions: create_table_perms,
        };

        let mut state = state.write().await;
        state.put_data_lake_settings(settings);

        wire::serialize_put_data_lake_settings_response(&model::PutDataLakeSettingsResponse {})
    }

    async fn handle_create_lf_tag(
        &self,
        state: &Arc<tokio::sync::RwLock<LakeFormationState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_l_f_tag_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.tag_key.is_empty() {
            return rest_json_error(400, "InvalidInputException", "Missing 'TagKey'");
        }
        if input.tag_values.is_empty() {
            return rest_json_error(400, "InvalidInputException", "Missing 'TagValues'");
        }

        let mut state = state.write().await;
        match state.create_lf_tag(&input.tag_key, input.tag_values, input.catalog_id) {
            Ok(()) => wire::serialize_create_l_f_tag_response(&model::CreateLFTagResponse {}),
            Err(e) => lf_error_response(&e),
        }
    }

    async fn handle_delete_lf_tag(
        &self,
        state: &Arc<tokio::sync::RwLock<LakeFormationState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_l_f_tag_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.tag_key.is_empty() {
            return rest_json_error(400, "InvalidInputException", "Missing 'TagKey'");
        }

        let mut state = state.write().await;
        match state.delete_lf_tag(&input.tag_key) {
            Ok(()) => wire::serialize_delete_l_f_tag_response(&model::DeleteLFTagResponse {}),
            Err(e) => lf_error_response(&e),
        }
    }

    async fn handle_get_lf_tag(
        &self,
        state: &Arc<tokio::sync::RwLock<LakeFormationState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_l_f_tag_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.tag_key.is_empty() {
            return rest_json_error(400, "InvalidInputException", "Missing 'TagKey'");
        }

        let state = state.read().await;
        match state.get_lf_tag(&input.tag_key) {
            Ok(tag) => wire::serialize_get_l_f_tag_response(&model::GetLFTagResponse {
                catalog_id: tag.catalog_id.clone(),
                tag_key: Some(tag.tag_key.clone()),
                tag_values: Some(tag.tag_values.clone()),
            }),
            Err(e) => lf_error_response(&e),
        }
    }

    async fn handle_list_lf_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<LakeFormationState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let tags = state.list_lf_tags();
        let entries: Vec<model::LFTagPair> = tags
            .iter()
            .map(|t| model::LFTagPair {
                catalog_id: t.catalog_id.clone(),
                tag_key: t.tag_key.clone(),
                tag_values: t.tag_values.clone(),
            })
            .collect();
        wire::serialize_list_l_f_tags_response(&model::ListLFTagsResponse {
            l_f_tags: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_add_lf_tags_to_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<LakeFormationState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_add_l_f_tags_to_resource_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.l_f_tags.is_empty() {
            return rest_json_error(400, "InvalidInputException", "Missing 'LFTags'");
        }

        let resource_key = extract_resource_key_from_model(&input.resource);
        let lf_tags: Vec<types::LFTagPair> = input
            .l_f_tags
            .iter()
            .filter_map(|t| {
                let tag_key = t.tag_key.clone();
                if tag_key.is_empty() {
                    return None;
                }
                Some(types::LFTagPair {
                    catalog_id: t.catalog_id.clone(),
                    tag_key,
                    tag_values: t.tag_values.clone(),
                })
            })
            .collect();

        let mut state = state.write().await;
        state.add_lf_tags_to_resource(&resource_key, lf_tags);

        wire::serialize_add_l_f_tags_to_resource_response(&model::AddLFTagsToResourceResponse {
            failures: Some(vec![]),
            ..Default::default()
        })
    }

    async fn handle_update_lf_tag(
        &self,
        state: &Arc<tokio::sync::RwLock<LakeFormationState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_l_f_tag_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.tag_key.is_empty() {
            return rest_json_error(400, "InvalidInputException", "Missing 'TagKey'");
        }
        let tags_to_add = input.tag_values_to_add.unwrap_or_default();
        let tags_to_delete = input.tag_values_to_delete.unwrap_or_default();

        let mut state = state.write().await;
        match state.update_lf_tag(&input.tag_key, tags_to_add, tags_to_delete) {
            Ok(()) => wire::serialize_update_l_f_tag_response(&model::UpdateLFTagResponse {}),
            Err(e) => lf_error_response(&e),
        }
    }

    async fn handle_remove_lf_tags_from_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<LakeFormationState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_remove_l_f_tags_from_resource_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };

        let resource_key = extract_resource_key_from_model(&input.resource);
        let lf_tags: Vec<String> = input
            .l_f_tags
            .iter()
            .filter_map(|t| {
                if t.tag_key.is_empty() {
                    None
                } else {
                    Some(t.tag_key.clone())
                }
            })
            .collect();

        let mut state = state.write().await;
        state.remove_lf_tags_from_resource(&resource_key, lf_tags);
        wire::serialize_remove_l_f_tags_from_resource_response(
            &model::RemoveLFTagsFromResourceResponse {
                failures: Some(vec![]),
                ..Default::default()
            },
        )
    }

    async fn handle_get_resource_lf_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<LakeFormationState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_resource_l_f_tags_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let resource_key = extract_resource_key_from_model(&input.resource);

        let state = state.read().await;
        let tags = state.get_resource_lf_tags(&resource_key);

        // For simplicity, return tags on database level
        let db_tags: Vec<model::LFTagPair> = tags
            .iter()
            .map(|t| model::LFTagPair {
                catalog_id: t.catalog_id.clone(),
                tag_key: t.tag_key.clone(),
                tag_values: t.tag_values.clone(),
            })
            .collect();

        wire::serialize_get_resource_l_f_tags_response(&model::GetResourceLFTagsResponse {
            l_f_tag_on_database: Some(db_tags),
            ..Default::default()
        })
    }

    async fn handle_grant_permissions(
        &self,
        state: &Arc<tokio::sync::RwLock<LakeFormationState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_grant_permissions_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };

        let principal_id = input.principal.data_lake_principal_identifier.as_deref();
        let principal = match principal_id {
            Some(p) if !p.is_empty() => p,
            _ => return rest_json_error(400, "InvalidInputException", "Missing 'Principal'"),
        };

        let resource = serde_json::to_value(&input.resource).unwrap_or_else(|_| json!({}));
        let permissions = input.permissions;
        let permissions_with_grant_option = input.permissions_with_grant_option.unwrap_or_default();

        let mut state = state.write().await;
        state.grant_permissions(
            principal,
            resource,
            permissions,
            permissions_with_grant_option,
        );

        wire::serialize_grant_permissions_response(&model::GrantPermissionsResponse {})
    }

    async fn handle_revoke_permissions(
        &self,
        state: &Arc<tokio::sync::RwLock<LakeFormationState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_revoke_permissions_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };

        let principal_id = input.principal.data_lake_principal_identifier.as_deref();
        let principal = match principal_id {
            Some(p) if !p.is_empty() => p,
            _ => return rest_json_error(400, "InvalidInputException", "Missing 'Principal'"),
        };

        let resource = serde_json::to_value(&input.resource).unwrap_or_else(|_| json!({}));
        let permissions = input.permissions;

        let mut state = state.write().await;
        state.revoke_permissions(principal, &resource, &permissions);

        wire::serialize_revoke_permissions_response(&model::RevokePermissionsResponse {})
    }

    async fn handle_batch_grant_permissions(
        &self,
        state: &Arc<tokio::sync::RwLock<LakeFormationState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_grant_permissions_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };

        let mut state = state.write().await;
        for entry in &input.entries {
            let principal = entry
                .principal
                .as_ref()
                .and_then(|p| p.data_lake_principal_identifier.as_deref())
                .unwrap_or("");
            let resource = entry
                .resource
                .as_ref()
                .map(|r| serde_json::to_value(r).unwrap_or_else(|_| json!({})))
                .unwrap_or_else(|| json!({}));
            let permissions = entry.permissions.clone().unwrap_or_default();
            let perms_with_grant = entry
                .permissions_with_grant_option
                .clone()
                .unwrap_or_default();
            state.grant_permissions(principal, resource, permissions, perms_with_grant);
        }

        wire::serialize_batch_grant_permissions_response(&model::BatchGrantPermissionsResponse {
            failures: Some(vec![]),
            ..Default::default()
        })
    }

    async fn handle_batch_revoke_permissions(
        &self,
        state: &Arc<tokio::sync::RwLock<LakeFormationState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_revoke_permissions_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };

        let mut state = state.write().await;
        for entry in &input.entries {
            let principal = entry
                .principal
                .as_ref()
                .and_then(|p| p.data_lake_principal_identifier.as_deref())
                .unwrap_or("");
            let resource = entry
                .resource
                .as_ref()
                .map(|r| serde_json::to_value(r).unwrap_or_else(|_| json!({})))
                .unwrap_or_else(|| json!({}));
            let permissions = entry.permissions.clone().unwrap_or_default();
            state.revoke_permissions(principal, &resource, &permissions);
        }

        wire::serialize_batch_revoke_permissions_response(&model::BatchRevokePermissionsResponse {
            failures: Some(vec![]),
            ..Default::default()
        })
    }

    async fn handle_list_permissions(
        &self,
        state: &Arc<tokio::sync::RwLock<LakeFormationState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_permissions_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };

        let principal = input
            .principal
            .as_ref()
            .and_then(|p| p.data_lake_principal_identifier.as_deref());

        let state = state.read().await;
        let grants = state.list_permissions(principal);

        let entries: Vec<model::PrincipalResourcePermissions> = grants
            .iter()
            .map(|g| model::PrincipalResourcePermissions {
                principal: Some(model::DataLakePrincipal {
                    data_lake_principal_identifier: Some(g.principal.clone()),
                }),
                permissions: Some(g.permissions.clone()),
                permissions_with_grant_option: Some(g.permissions_with_grant_option.clone()),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_permissions_response(&model::ListPermissionsResponse {
            principal_resource_permissions: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_list_data_cells_filter(&self) -> MockResponse {
        wire::serialize_list_data_cells_filter_response(&model::ListDataCellsFilterResponse {
            data_cells_filters: Some(vec![]),
            ..Default::default()
        })
    }
}

/// Build a stable resource key from a typed `Resource` shape.
fn extract_resource_key_from_model(resource: &model::Resource) -> String {
    if let Some(db) = &resource.database {
        let name = db.name.as_str();
        return format!("database:{}", name);
    }
    if let Some(table) = &resource.table {
        let db = table.database_name.as_str();
        let name = table.name.as_deref().unwrap_or("");
        return format!("table:{}:{}", db, name);
    }
    "unknown".to_string()
}

fn principal_permissions_from_model(
    perms: Vec<model::PrincipalPermissions>,
) -> Vec<types::PrincipalPermissions> {
    perms
        .into_iter()
        .filter_map(|p| {
            let principal_id = p.principal?.data_lake_principal_identifier?;
            let permissions = p.permissions.unwrap_or_default();
            Some(types::PrincipalPermissions {
                principal: types::DataLakePrincipal {
                    data_lake_principal_identifier: principal_id,
                },
                permissions,
            })
        })
        .collect()
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
        uri.split('?').next().unwrap_or(uri).to_string()
    }
}

fn lf_error_response(err: &LakeFormationError) -> MockResponse {
    let (status, error_type) = match err {
        LakeFormationError::ResourceAlreadyExists { .. } => (409, "AlreadyExistsException"),
        LakeFormationError::ResourceNotFound { .. } => (400, "EntityNotFoundException"),
        LakeFormationError::TagAlreadyExists { .. } => (409, "AlreadyExistsException"),
        LakeFormationError::TagNotFound { .. } => (400, "EntityNotFoundException"),
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
