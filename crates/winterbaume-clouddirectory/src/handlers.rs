use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{CloudDirectoryError, CloudDirectoryState};
use crate::views::CloudDirectoryStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct CloudDirectoryService {
    pub(crate) state: Arc<BackendState<CloudDirectoryState>>,
    pub(crate) notifier: StateChangeNotifier<CloudDirectoryStateView>,
}

impl CloudDirectoryService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CloudDirectoryService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CloudDirectoryService {
    fn service_name(&self) -> &str {
        "clouddirectory"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://clouddirectory\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<CloudDirectoryState>>;

impl CloudDirectoryService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let (path, query) = extract_path_and_query(&request.uri);
        let method = request.method.as_str();
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(&query);

        // Cloud Directory uses /amazonclouddirectory/2017-01-11/ prefix
        let path_no_prefix = path.trim_start_matches("/amazonclouddirectory/2017-01-11");

        let labels: &[(&str, &str)] = &[];
        let response = match (method, path_no_prefix) {
            // CreateDirectory: PUT /directory/create
            ("PUT", "/directory/create") => {
                self.handle_create_directory(
                    &state, &request, labels, &query_map, account_id, &region,
                )
                .await
            }
            // DeleteDirectory: PUT /directory
            ("PUT", "/directory") => {
                self.handle_delete_directory(&state, &request, labels, &query_map)
                    .await
            }
            // GetDirectory: POST /directory/get
            ("POST", "/directory/get") => {
                self.handle_get_directory(&state, &request, labels, &query_map)
                    .await
            }
            // ListDirectories: POST /directory/list
            ("POST", "/directory/list") => {
                self.handle_list_directories(&state, &request, labels, &query_map)
                    .await
            }
            // CreateSchema: PUT /schema/create
            ("PUT", "/schema/create") => {
                self.handle_create_schema(&state, &request, labels, &query_map, account_id, &region)
                    .await
            }
            // DeleteSchema: PUT /schema
            ("PUT", "/schema") => {
                self.handle_delete_schema(&state, &request, labels, &query_map)
                    .await
            }
            // ApplySchema: PUT /schema/apply
            ("PUT", "/schema/apply") => {
                self.handle_apply_schema(&state, &request, labels, &query_map)
                    .await
            }
            // ListDevelopmentSchemaArns: POST /schema/development
            ("POST", "/schema/development") => {
                self.handle_list_development_schema_arns(&state).await
            }
            // ListPublishedSchemaArns: POST /schema/published
            ("POST", "/schema/published") => self.handle_list_published_schema_arns(&state).await,
            // PublishSchema: PUT /schema/publish
            ("PUT", "/schema/publish") => {
                self.handle_publish_schema(
                    &state, &request, labels, &query_map, account_id, &region,
                )
                .await
            }
            // ListTagsForResource: POST /tags
            ("POST", "/tags") => {
                self.handle_list_tags_for_resource(&state, &request, labels, &query_map)
                    .await
            }
            // TagResource: PUT /tags/add
            ("PUT", "/tags/add") => {
                self.handle_tag_resource(&state, &request, labels, &query_map)
                    .await
            }
            // UntagResource: PUT /tags/remove
            ("PUT", "/tags/remove") => {
                self.handle_untag_resource(&state, &request, labels, &query_map)
                    .await
            }
            _ => clouddirectory_error(404, "UnknownOperationException", "Not found"),
        };

        if matches!(method, "PUT" | "POST" | "DELETE") && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_directory(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_directory_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return clouddirectory_error(400, "SerializationException", &e),
        };
        if input.name.is_empty() {
            return clouddirectory_error(400, "ValidationException", "Missing 'Name'");
        }
        if input.schema_arn.is_empty() {
            return clouddirectory_error(
                400,
                "ValidationException",
                "Missing 'x-amz-data-partition' header (SchemaArn)",
            );
        }
        let mut state = state.write().await;
        match state.create_directory(&input.name, &input.schema_arn, account_id, region) {
            Ok(dir) => {
                let resp = wire::CreateDirectoryResponse {
                    directory_arn: Some(dir.directory_arn.clone()),
                    name: Some(dir.name.clone()),
                    object_identifier: Some("$ObjectIdentifier".to_string()),
                    applied_schema_arn: Some(dir.schema_arn.clone()),
                    ..Default::default()
                };
                wire::serialize_create_directory_response(&resp)
            }
            Err(e) => clouddirectory_error_from(&e),
        }
    }

    async fn handle_delete_directory(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_directory_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return clouddirectory_error(400, "SerializationException", &e),
        };
        if input.directory_arn.is_empty() {
            return clouddirectory_error(
                400,
                "ValidationException",
                "Missing 'x-amz-data-partition' header (DirectoryArn)",
            );
        }
        let mut state = state.write().await;
        match state.delete_directory(&input.directory_arn) {
            Ok(arn) => {
                let resp = wire::DeleteDirectoryResponse {
                    directory_arn: Some(arn),
                    ..Default::default()
                };
                wire::serialize_delete_directory_response(&resp)
            }
            Err(e) => clouddirectory_error_from(&e),
        }
    }

    async fn handle_get_directory(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_directory_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return clouddirectory_error(400, "SerializationException", &e),
        };
        if input.directory_arn.is_empty() {
            return clouddirectory_error(
                400,
                "ValidationException",
                "Missing 'x-amz-data-partition' header (DirectoryArn)",
            );
        }
        let state = state.read().await;
        match state.get_directory(&input.directory_arn) {
            Ok(dir) => {
                let resp = wire::GetDirectoryResponse {
                    directory: Some(wire::Directory {
                        directory_arn: Some(dir.directory_arn.clone()),
                        name: Some(dir.name.clone()),
                        state: Some(dir.state.clone()),
                        creation_date_time: Some(dir.created_date_time),
                        ..Default::default()
                    }),
                    ..Default::default()
                };
                wire::serialize_get_directory_response(&resp)
            }
            Err(e) => clouddirectory_error_from(&e),
        }
    }

    async fn handle_list_directories(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_directories_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return clouddirectory_error(400, "SerializationException", &e),
        };
        let state_filter = input.state.as_deref();
        let state = state.read().await;
        let dirs = state.list_directories(state_filter);

        let directories: Vec<wire::Directory> = dirs
            .into_iter()
            .map(|d| wire::Directory {
                directory_arn: Some(d.directory_arn.clone()),
                name: Some(d.name.clone()),
                state: Some(d.state.clone()),
                creation_date_time: Some(d.created_date_time),
                ..Default::default()
            })
            .collect();

        let resp = wire::ListDirectoriesResponse {
            directories: Some(directories),
            next_token: None,
            ..Default::default()
        };
        wire::serialize_list_directories_response(&resp)
    }

    async fn handle_create_schema(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_schema_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return clouddirectory_error(400, "SerializationException", &e),
        };
        if input.name.is_empty() {
            return clouddirectory_error(400, "ValidationException", "Missing 'Name'");
        }
        let mut state = state.write().await;
        match state.create_schema(&input.name, account_id, region) {
            Ok(schema) => {
                let resp = wire::CreateSchemaResponse {
                    schema_arn: Some(schema.schema_arn.clone()),
                    ..Default::default()
                };
                wire::serialize_create_schema_response(&resp)
            }
            Err(e) => clouddirectory_error_from(&e),
        }
    }

    async fn handle_delete_schema(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_schema_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return clouddirectory_error(400, "SerializationException", &e),
        };
        if input.schema_arn.is_empty() {
            return clouddirectory_error(
                400,
                "ValidationException",
                "Missing 'x-amz-data-partition' header (SchemaArn)",
            );
        }
        let mut state = state.write().await;
        match state.delete_schema(&input.schema_arn) {
            Ok(arn) => {
                let resp = wire::DeleteSchemaResponse {
                    schema_arn: Some(arn),
                    ..Default::default()
                };
                wire::serialize_delete_schema_response(&resp)
            }
            Err(e) => clouddirectory_error_from(&e),
        }
    }

    async fn handle_apply_schema(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_apply_schema_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return clouddirectory_error(400, "SerializationException", &e),
        };
        if input.directory_arn.is_empty() {
            return clouddirectory_error(
                400,
                "ValidationException",
                "Missing 'x-amz-data-partition' header (DirectoryArn)",
            );
        }
        if input.published_schema_arn.is_empty() {
            return clouddirectory_error(
                400,
                "ValidationException",
                "Missing 'PublishedSchemaArn'",
            );
        }
        let mut state = state.write().await;
        match state.apply_schema_arns(&input.directory_arn, &input.published_schema_arn) {
            Ok((applied_schema_arn, dir_arn)) => {
                let resp = wire::ApplySchemaResponse {
                    applied_schema_arn: Some(applied_schema_arn),
                    directory_arn: Some(dir_arn),
                    ..Default::default()
                };
                wire::serialize_apply_schema_response(&resp)
            }
            Err(e) => clouddirectory_error_from(&e),
        }
    }

    async fn handle_list_development_schema_arns(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let arns: Vec<String> = state
            .list_development_schema_arns()
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        let resp = wire::ListDevelopmentSchemaArnsResponse {
            schema_arns: Some(arns),
            next_token: None,
            ..Default::default()
        };
        wire::serialize_list_development_schema_arns_response(&resp)
    }

    async fn handle_list_published_schema_arns(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let arns = state.list_published_schema_arns();

        let resp = wire::ListPublishedSchemaArnsResponse {
            schema_arns: Some(arns),
            next_token: None,
            ..Default::default()
        };
        wire::serialize_list_published_schema_arns_response(&resp)
    }

    async fn handle_publish_schema(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_publish_schema_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return clouddirectory_error(400, "SerializationException", &e),
        };
        if input.development_schema_arn.is_empty() {
            return clouddirectory_error(
                400,
                "ValidationException",
                "Missing 'x-amz-data-partition' header (DevelopmentSchemaArn)",
            );
        }
        if input.version.is_empty() {
            return clouddirectory_error(400, "ValidationException", "Missing 'Version'");
        }
        let mut state = state.write().await;
        match state.publish_schema(
            &input.development_schema_arn,
            &input.version,
            account_id,
            region,
        ) {
            Ok(published_arn) => {
                let resp = wire::PublishSchemaResponse {
                    published_schema_arn: Some(published_arn),
                    ..Default::default()
                };
                wire::serialize_publish_schema_response(&resp)
            }
            Err(e) => clouddirectory_error_from(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return clouddirectory_error(400, "SerializationException", &e),
        };
        if input.resource_arn.is_empty() {
            return clouddirectory_error(400, "ValidationException", "Missing 'ResourceArn'");
        }
        let state = state.read().await;
        match state.list_tags_for_resource(&input.resource_arn) {
            Ok(tags) => {
                let wire_tags: Vec<wire::Tag> = tags
                    .into_iter()
                    .map(|(k, v)| wire::Tag {
                        key: Some(k),
                        value: Some(v),
                        ..Default::default()
                    })
                    .collect();
                let resp = wire::ListTagsForResourceResponse {
                    tags: Some(wire_tags),
                    next_token: None,
                    ..Default::default()
                };
                wire::serialize_list_tags_for_resource_response(&resp)
            }
            Err(e) => clouddirectory_error_from(&e),
        }
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
            Err(e) => return clouddirectory_error(400, "SerializationException", &e),
        };
        if input.resource_arn.is_empty() {
            return clouddirectory_error(400, "ValidationException", "Missing 'ResourceArn'");
        }
        let tags: Vec<(String, String)> = input
            .tags
            .iter()
            .filter_map(|t| {
                let key = t.key.clone()?;
                let value = t.value.clone().unwrap_or_default();
                Some((key, value))
            })
            .collect();

        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, tags) {
            Ok(()) => {
                let resp = wire::TagResourceResponse {};
                wire::serialize_tag_resource_response(&resp)
            }
            Err(e) => clouddirectory_error_from(&e),
        }
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
            Err(e) => return clouddirectory_error(400, "SerializationException", &e),
        };
        if input.resource_arn.is_empty() {
            return clouddirectory_error(400, "ValidationException", "Missing 'ResourceArn'");
        }
        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, input.tag_keys.clone()) {
            Ok(()) => {
                let resp = wire::UntagResourceResponse {};
                wire::serialize_untag_resource_response(&resp)
            }
            Err(e) => clouddirectory_error_from(&e),
        }
    }
}

// ===================== Utility functions =====================

fn extract_path_and_query(uri: &str) -> (String, String) {
    let relevant = if let Some(idx) = uri.find("amazonaws.com") {
        &uri[idx + "amazonaws.com".len()..]
    } else {
        uri
    };
    if let Some(q) = relevant.find('?') {
        (relevant[..q].to_string(), relevant[q + 1..].to_string())
    } else {
        (relevant.to_string(), String::new())
    }
}

fn clouddirectory_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "Type": "User",
        "Message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}

fn clouddirectory_error_from(err: &CloudDirectoryError) -> MockResponse {
    let (status, error_type) = match err {
        CloudDirectoryError::DirectoryAlreadyExists(_) => (400, "DirectoryAlreadyExistsException"),
        CloudDirectoryError::DirectoryNotDisabled => (400, "DirectoryNotDisabledException"),
        CloudDirectoryError::SchemaAlreadyExists(_) => (400, "SchemaAlreadyExistsException"),
        CloudDirectoryError::DirectoryNotFound(_)
        | CloudDirectoryError::SchemaNotFound(_)
        | CloudDirectoryError::ResourceNotFound(_) => (400, "ResourceNotFoundException"),
    };
    clouddirectory_error(status, error_type, &err.to_string())
}
