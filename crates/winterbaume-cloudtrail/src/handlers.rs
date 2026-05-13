use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    json_error_response,
};

use crate::state::{CloudTrailError, CloudTrailState};
use crate::views::CloudTrailStateView;
use crate::wire;

pub struct CloudTrailService {
    pub(crate) state: Arc<BackendState<CloudTrailState>>,
    pub(crate) notifier: StateChangeNotifier<CloudTrailStateView>,
}

impl CloudTrailService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CloudTrailService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CloudTrailService {
    fn service_name(&self) -> &str {
        "cloudtrail"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://cloudtrail\..*\.amazonaws\.com",
            r"https?://cloudtrail\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl CloudTrailService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').next_back())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        match action.as_str() {
            "CreateTrail" => {
                self.handle_create_trail(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeTrails" => self.handle_describe_trails(&state, body_bytes).await,
            "DeleteTrail" => self.handle_delete_trail(&state, body_bytes).await,
            "GetTrail" => self.handle_get_trail(&state, body_bytes).await,
            "GetTrailStatus" => self.handle_get_trail_status(&state, body_bytes).await,
            "ListTrails" => self.handle_list_trails(&state).await,
            "UpdateTrail" => self.handle_update_trail(&state, body_bytes).await,
            "AddTags" => self.handle_add_tags(&state, body_bytes).await,
            "RemoveTags" => self.handle_remove_tags(&state, body_bytes).await,
            "ListTags" => self.handle_list_tags(&state, body_bytes).await,
            "GetEventSelectors" => self.handle_get_event_selectors(&state, body_bytes).await,
            "PutEventSelectors" => self.handle_put_event_selectors(&state, body_bytes).await,
            "GetInsightSelectors" => self.handle_get_insight_selectors(&state, body_bytes).await,
            "PutInsightSelectors" => self.handle_put_insight_selectors(&state, body_bytes).await,
            "StartLogging" => self.handle_start_logging(&state, body_bytes).await,
            "StopLogging" => self.handle_stop_logging(&state, body_bytes).await,
            "LookupEvents" => self.handle_lookup_events().await,
            "ListPublicKeys" => self.handle_list_public_keys().await,
            "CreateEventDataStore" => {
                self.handle_create_event_data_store(&state, body_bytes, account_id, &region)
                    .await
            }
            "GetEventDataStore" => self.handle_get_event_data_store(&state, body_bytes).await,
            "DeleteEventDataStore" => {
                self.handle_delete_event_data_store(&state, body_bytes)
                    .await
            }
            "ListEventDataStores" => self.handle_list_event_data_stores(&state).await,
            "UpdateEventDataStore" => {
                self.handle_update_event_data_store(&state, body_bytes)
                    .await
            }
            // --- Unimplemented operations (auto-generated stubs) ---
            "CancelQuery" => json_error_response(
                501,
                "NotImplementedError",
                "CancelQuery is not yet implemented in winterbaume-cloudtrail",
            ),
            "CreateChannel" => json_error_response(
                501,
                "NotImplementedError",
                "CreateChannel is not yet implemented in winterbaume-cloudtrail",
            ),
            "CreateDashboard" => json_error_response(
                501,
                "NotImplementedError",
                "CreateDashboard is not yet implemented in winterbaume-cloudtrail",
            ),
            "DeleteChannel" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteChannel is not yet implemented in winterbaume-cloudtrail",
            ),
            "DeleteDashboard" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteDashboard is not yet implemented in winterbaume-cloudtrail",
            ),
            "DeleteResourcePolicy" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteResourcePolicy is not yet implemented in winterbaume-cloudtrail",
            ),
            "DeregisterOrganizationDelegatedAdmin" => json_error_response(
                501,
                "NotImplementedError",
                "DeregisterOrganizationDelegatedAdmin is not yet implemented in winterbaume-cloudtrail",
            ),
            "DescribeQuery" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeQuery is not yet implemented in winterbaume-cloudtrail",
            ),
            "DisableFederation" => json_error_response(
                501,
                "NotImplementedError",
                "DisableFederation is not yet implemented in winterbaume-cloudtrail",
            ),
            "EnableFederation" => json_error_response(
                501,
                "NotImplementedError",
                "EnableFederation is not yet implemented in winterbaume-cloudtrail",
            ),
            "GenerateQuery" => json_error_response(
                501,
                "NotImplementedError",
                "GenerateQuery is not yet implemented in winterbaume-cloudtrail",
            ),
            "GetChannel" => json_error_response(
                501,
                "NotImplementedError",
                "GetChannel is not yet implemented in winterbaume-cloudtrail",
            ),
            "GetDashboard" => json_error_response(
                501,
                "NotImplementedError",
                "GetDashboard is not yet implemented in winterbaume-cloudtrail",
            ),
            "GetEventConfiguration" => json_error_response(
                501,
                "NotImplementedError",
                "GetEventConfiguration is not yet implemented in winterbaume-cloudtrail",
            ),
            "GetImport" => json_error_response(
                501,
                "NotImplementedError",
                "GetImport is not yet implemented in winterbaume-cloudtrail",
            ),
            "GetQueryResults" => json_error_response(
                501,
                "NotImplementedError",
                "GetQueryResults is not yet implemented in winterbaume-cloudtrail",
            ),
            "GetResourcePolicy" => json_error_response(
                501,
                "NotImplementedError",
                "GetResourcePolicy is not yet implemented in winterbaume-cloudtrail",
            ),
            "ListChannels" => json_error_response(
                501,
                "NotImplementedError",
                "ListChannels is not yet implemented in winterbaume-cloudtrail",
            ),
            "ListDashboards" => json_error_response(
                501,
                "NotImplementedError",
                "ListDashboards is not yet implemented in winterbaume-cloudtrail",
            ),
            "ListImportFailures" => json_error_response(
                501,
                "NotImplementedError",
                "ListImportFailures is not yet implemented in winterbaume-cloudtrail",
            ),
            "ListImports" => json_error_response(
                501,
                "NotImplementedError",
                "ListImports is not yet implemented in winterbaume-cloudtrail",
            ),
            "ListInsightsData" => json_error_response(
                501,
                "NotImplementedError",
                "ListInsightsData is not yet implemented in winterbaume-cloudtrail",
            ),
            "ListInsightsMetricData" => json_error_response(
                501,
                "NotImplementedError",
                "ListInsightsMetricData is not yet implemented in winterbaume-cloudtrail",
            ),
            "ListQueries" => json_error_response(
                501,
                "NotImplementedError",
                "ListQueries is not yet implemented in winterbaume-cloudtrail",
            ),
            "PutEventConfiguration" => json_error_response(
                501,
                "NotImplementedError",
                "PutEventConfiguration is not yet implemented in winterbaume-cloudtrail",
            ),
            "PutResourcePolicy" => json_error_response(
                501,
                "NotImplementedError",
                "PutResourcePolicy is not yet implemented in winterbaume-cloudtrail",
            ),
            "RegisterOrganizationDelegatedAdmin" => json_error_response(
                501,
                "NotImplementedError",
                "RegisterOrganizationDelegatedAdmin is not yet implemented in winterbaume-cloudtrail",
            ),
            "RestoreEventDataStore" => json_error_response(
                501,
                "NotImplementedError",
                "RestoreEventDataStore is not yet implemented in winterbaume-cloudtrail",
            ),
            "SearchSampleQueries" => json_error_response(
                501,
                "NotImplementedError",
                "SearchSampleQueries is not yet implemented in winterbaume-cloudtrail",
            ),
            "StartDashboardRefresh" => json_error_response(
                501,
                "NotImplementedError",
                "StartDashboardRefresh is not yet implemented in winterbaume-cloudtrail",
            ),
            "StartEventDataStoreIngestion" => json_error_response(
                501,
                "NotImplementedError",
                "StartEventDataStoreIngestion is not yet implemented in winterbaume-cloudtrail",
            ),
            "StartImport" => json_error_response(
                501,
                "NotImplementedError",
                "StartImport is not yet implemented in winterbaume-cloudtrail",
            ),
            "StartQuery" => json_error_response(
                501,
                "NotImplementedError",
                "StartQuery is not yet implemented in winterbaume-cloudtrail",
            ),
            "StopEventDataStoreIngestion" => json_error_response(
                501,
                "NotImplementedError",
                "StopEventDataStoreIngestion is not yet implemented in winterbaume-cloudtrail",
            ),
            "StopImport" => json_error_response(
                501,
                "NotImplementedError",
                "StopImport is not yet implemented in winterbaume-cloudtrail",
            ),
            "UpdateChannel" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateChannel is not yet implemented in winterbaume-cloudtrail",
            ),
            "UpdateDashboard" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateDashboard is not yet implemented in winterbaume-cloudtrail",
            ),
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for CloudTrail"),
            ),
        }
    }

    async fn handle_create_trail(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudTrailState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_trail_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidTrailNameException", "Trail name is required");
        }
        if input.s3_bucket_name.is_empty() {
            return json_error_response(
                400,
                "S3BucketDoesNotExistException",
                "S3BucketName is required",
            );
        }
        let name = input.name.as_str();
        let s3_bucket = input.s3_bucket_name.as_str();
        let s3_prefix = input.s3_key_prefix.as_deref().unwrap_or("");
        let include_global = input.include_global_service_events.unwrap_or(true);
        let is_multi_region = input.is_multi_region_trail.unwrap_or(false);

        let mut state = state.write().await;
        match state.create_trail(
            name,
            s3_bucket,
            s3_prefix,
            include_global,
            is_multi_region,
            account_id,
            region,
        ) {
            Ok(trail) => {
                let resp = wire::CreateTrailResponse {
                    name: Some(trail.name.clone()),
                    s3_bucket_name: Some(trail.s3_bucket_name.clone()),
                    s3_key_prefix: Some(trail.s3_key_prefix.clone()),
                    include_global_service_events: Some(trail.include_global_service_events),
                    is_multi_region_trail: Some(trail.is_multi_region_trail),
                    trail_a_r_n: Some(trail.trail_arn.clone()),
                    ..Default::default()
                };
                wire::serialize_create_trail_response(&resp)
            }
            Err(e) => cloudtrail_error_response(&e),
        }
    }

    async fn handle_describe_trails(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudTrailState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_trails_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let trail_names: Vec<String> = input.trail_name_list.unwrap_or_default();

        let state = state.read().await;
        let trails = state.describe_trails(&trail_names);
        let entries: Vec<wire::Trail> = trails.iter().map(|t| trail_to_wire(t)).collect();

        let resp = wire::DescribeTrailsResponse {
            trail_list: Some(entries),
        };
        wire::serialize_describe_trails_response(&resp)
    }

    async fn handle_delete_trail(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudTrailState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_trail_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidTrailNameException", "Trail name is required");
        }

        let mut state = state.write().await;
        match state.delete_trail(&input.name) {
            Ok(()) => wire::serialize_delete_trail_response(&wire::DeleteTrailResponse {}),
            Err(e) => cloudtrail_error_response(&e),
        }
    }

    async fn handle_get_trail_status(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudTrailState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_trail_status_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidTrailNameException", "Trail name is required");
        }

        let state = state.read().await;
        match state.get_trail_status(&input.name) {
            Ok(trail) => {
                let resp = wire::GetTrailStatusResponse {
                    is_logging: Some(trail.is_logging),
                    ..Default::default()
                };
                wire::serialize_get_trail_status_response(&resp)
            }
            Err(e) => cloudtrail_error_response(&e),
        }
    }

    async fn handle_start_logging(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudTrailState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_logging_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidTrailNameException", "Trail name is required");
        }

        let mut state = state.write().await;
        match state.start_logging(&input.name) {
            Ok(()) => wire::serialize_start_logging_response(&wire::StartLoggingResponse {}),
            Err(e) => cloudtrail_error_response(&e),
        }
    }

    async fn handle_stop_logging(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudTrailState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_logging_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidTrailNameException", "Trail name is required");
        }

        let mut state = state.write().await;
        match state.stop_logging(&input.name) {
            Ok(()) => wire::serialize_stop_logging_response(&wire::StopLoggingResponse {}),
            Err(e) => cloudtrail_error_response(&e),
        }
    }

    async fn handle_get_trail(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudTrailState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_trail_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidTrailNameException", "Trail name is required");
        }

        let state = state.read().await;
        match state.get_trail(&input.name) {
            Ok(trail) => {
                let resp = wire::GetTrailResponse {
                    trail: Some(trail_to_wire(trail)),
                };
                wire::serialize_get_trail_response(&resp)
            }
            Err(e) => cloudtrail_error_response(&e),
        }
    }

    async fn handle_list_trails(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudTrailState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let trails = state.list_trails();
        let entries: Vec<wire::TrailInfo> = trails
            .iter()
            .map(|t| wire::TrailInfo {
                name: Some(t.name.clone()),
                trail_a_r_n: Some(t.trail_arn.clone()),
                home_region: Some(t.home_region.clone()),
            })
            .collect();

        let resp = wire::ListTrailsResponse {
            trails: Some(entries),
            ..Default::default()
        };
        wire::serialize_list_trails_response(&resp)
    }

    async fn handle_update_trail(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudTrailState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_trail_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidTrailNameException", "Trail name is required");
        }
        let name = input.name.as_str();
        let s3_bucket = input.s3_bucket_name.as_deref();
        let s3_prefix = input.s3_key_prefix.as_deref();
        let include_global = input.include_global_service_events;
        let is_multi_region = input.is_multi_region_trail;

        let mut state = state.write().await;
        match state.update_trail(name, s3_bucket, s3_prefix, include_global, is_multi_region) {
            Ok(trail) => {
                let resp = wire::UpdateTrailResponse {
                    name: Some(trail.name.clone()),
                    s3_bucket_name: Some(trail.s3_bucket_name.clone()),
                    s3_key_prefix: Some(trail.s3_key_prefix.clone()),
                    include_global_service_events: Some(trail.include_global_service_events),
                    is_multi_region_trail: Some(trail.is_multi_region_trail),
                    trail_a_r_n: Some(trail.trail_arn.clone()),
                    ..Default::default()
                };
                wire::serialize_update_trail_response(&resp)
            }
            Err(e) => cloudtrail_error_response(&e),
        }
    }

    async fn handle_add_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudTrailState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_add_tags_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.resource_id.is_empty() {
            return json_error_response(400, "InvalidParameterException", "ResourceId is required");
        }

        let tags: Vec<(String, String)> = input
            .tags_list
            .into_iter()
            .map(|t| (t.key, t.value.unwrap_or_default()))
            .collect();

        let mut state = state.write().await;
        match state.add_tags(&input.resource_id, &tags) {
            Ok(()) => wire::serialize_add_tags_response(&wire::AddTagsResponse {}),
            Err(e) => cloudtrail_error_response(&e),
        }
    }

    async fn handle_remove_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudTrailState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_remove_tags_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.resource_id.is_empty() {
            return json_error_response(400, "InvalidParameterException", "ResourceId is required");
        }

        let tag_keys: Vec<String> = input.tags_list.into_iter().map(|t| t.key).collect();

        let mut state = state.write().await;
        match state.remove_tags(&input.resource_id, &tag_keys) {
            Ok(()) => wire::serialize_remove_tags_response(&wire::RemoveTagsResponse {}),
            Err(e) => cloudtrail_error_response(&e),
        }
    }

    async fn handle_list_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudTrailState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let resource_ids = input.resource_id_list;

        let state = state.read().await;
        match state.list_tags(&resource_ids) {
            Ok(results) => {
                let resource_tag_list: Vec<wire::ResourceTag> = results
                    .iter()
                    .map(|(trail, tags)| {
                        let tags_list: Vec<wire::Tag> = tags
                            .iter()
                            .map(|(k, v)| wire::Tag {
                                key: k.clone(),
                                value: Some(v.clone()),
                            })
                            .collect();
                        wire::ResourceTag {
                            resource_id: Some(trail.trail_arn.clone()),
                            tags_list: Some(tags_list),
                        }
                    })
                    .collect();

                let resp = wire::ListTagsResponse {
                    resource_tag_list: Some(resource_tag_list),
                    ..Default::default()
                };
                wire::serialize_list_tags_response(&resp)
            }
            Err(e) => cloudtrail_error_response(&e),
        }
    }

    async fn handle_get_event_selectors(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudTrailState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_event_selectors_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.trail_name.is_empty() {
            return json_error_response(400, "InvalidTrailNameException", "TrailName is required");
        }

        let state = state.read().await;
        match state.get_event_selectors(&input.trail_name) {
            Ok(trail) => {
                let selectors: Vec<wire::EventSelector> = trail
                    .event_selectors
                    .iter()
                    .map(|es| wire::EventSelector {
                        read_write_type: Some(es.read_write_type.clone()),
                        include_management_events: Some(es.include_management_events),
                        data_resources: Some(
                            es.data_resources
                                .iter()
                                .map(|dr| wire::DataResource {
                                    r#type: Some(dr.r#type.clone()),
                                    values: Some(dr.values.clone()),
                                })
                                .collect(),
                        ),
                        exclude_management_event_sources: Some(
                            es.exclude_management_event_sources.clone(),
                        ),
                    })
                    .collect();

                let resp = wire::GetEventSelectorsResponse {
                    trail_a_r_n: Some(trail.trail_arn.clone()),
                    event_selectors: Some(selectors),
                    ..Default::default()
                };
                wire::serialize_get_event_selectors_response(&resp)
            }
            Err(e) => cloudtrail_error_response(&e),
        }
    }

    async fn handle_put_event_selectors(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudTrailState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_event_selectors_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.trail_name.is_empty() {
            return json_error_response(400, "InvalidTrailNameException", "TrailName is required");
        }

        let event_selectors: Vec<crate::types::EventSelector> = input
            .event_selectors
            .unwrap_or_default()
            .into_iter()
            .map(|es| crate::types::EventSelector {
                read_write_type: es.read_write_type.unwrap_or_else(|| "All".to_string()),
                include_management_events: es.include_management_events.unwrap_or(true),
                data_resources: es
                    .data_resources
                    .unwrap_or_default()
                    .into_iter()
                    .map(|dr| crate::types::DataResource {
                        r#type: dr.r#type.unwrap_or_default(),
                        values: dr.values.unwrap_or_default(),
                    })
                    .collect(),
                exclude_management_event_sources: es
                    .exclude_management_event_sources
                    .unwrap_or_default(),
            })
            .collect();

        let mut state = state.write().await;
        match state.put_event_selectors(&input.trail_name, event_selectors) {
            Ok(trail) => {
                let selectors: Vec<wire::EventSelector> = trail
                    .event_selectors
                    .iter()
                    .map(|es| wire::EventSelector {
                        read_write_type: Some(es.read_write_type.clone()),
                        include_management_events: Some(es.include_management_events),
                        data_resources: Some(
                            es.data_resources
                                .iter()
                                .map(|dr| wire::DataResource {
                                    r#type: Some(dr.r#type.clone()),
                                    values: Some(dr.values.clone()),
                                })
                                .collect(),
                        ),
                        exclude_management_event_sources: Some(
                            es.exclude_management_event_sources.clone(),
                        ),
                    })
                    .collect();

                let resp = wire::PutEventSelectorsResponse {
                    trail_a_r_n: Some(trail.trail_arn.clone()),
                    event_selectors: Some(selectors),
                    ..Default::default()
                };
                wire::serialize_put_event_selectors_response(&resp)
            }
            Err(e) => cloudtrail_error_response(&e),
        }
    }

    async fn handle_get_insight_selectors(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudTrailState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_insight_selectors_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let name = match input.trail_name.as_deref().filter(|s| !s.is_empty()) {
            Some(n) => n,
            None => {
                return json_error_response(
                    400,
                    "InvalidTrailNameException",
                    "TrailName is required",
                );
            }
        };

        let state = state.read().await;
        match state.get_insight_selectors(name) {
            Ok(trail) => {
                let selectors: Vec<wire::InsightSelector> = trail
                    .insight_selectors
                    .iter()
                    .map(|is| wire::InsightSelector {
                        insight_type: Some(is.insight_type.clone()),
                        ..Default::default()
                    })
                    .collect();

                let resp = wire::GetInsightSelectorsResponse {
                    trail_a_r_n: Some(trail.trail_arn.clone()),
                    insight_selectors: Some(selectors),
                    ..Default::default()
                };
                wire::serialize_get_insight_selectors_response(&resp)
            }
            Err(e) => cloudtrail_error_response(&e),
        }
    }

    async fn handle_put_insight_selectors(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudTrailState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_insight_selectors_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let name = match input.trail_name.as_deref().filter(|s| !s.is_empty()) {
            Some(n) => n,
            None => {
                return json_error_response(
                    400,
                    "InvalidTrailNameException",
                    "TrailName is required",
                );
            }
        };

        let insight_selectors: Vec<crate::types::InsightSelector> = input
            .insight_selectors
            .into_iter()
            .map(|is| crate::types::InsightSelector {
                insight_type: is
                    .insight_type
                    .unwrap_or_else(|| "ApiCallRateInsight".to_string()),
            })
            .collect();

        let mut state = state.write().await;
        match state.put_insight_selectors(name, insight_selectors) {
            Ok(trail) => {
                let selectors: Vec<wire::InsightSelector> = trail
                    .insight_selectors
                    .iter()
                    .map(|is| wire::InsightSelector {
                        insight_type: Some(is.insight_type.clone()),
                        ..Default::default()
                    })
                    .collect();

                let resp = wire::PutInsightSelectorsResponse {
                    trail_a_r_n: Some(trail.trail_arn.clone()),
                    insight_selectors: Some(selectors),
                    ..Default::default()
                };
                wire::serialize_put_insight_selectors_response(&resp)
            }
            Err(e) => cloudtrail_error_response(&e),
        }
    }

    // STUB[no-telemetry]: LookupEvents returns real audit event history driven by
    //   infrastructure telemetry; the mock has no event ingestion pipeline.
    async fn handle_lookup_events(&self) -> MockResponse {
        let resp = wire::LookupEventsResponse {
            events: Some(vec![]),
            ..Default::default()
        };
        wire::serialize_lookup_events_response(&resp)
    }

    // STUB[no-telemetry]: ListPublicKeys returns the public keys used to validate
    //   CloudTrail log file integrity; the mock has no real signing infrastructure.
    async fn handle_list_public_keys(&self) -> MockResponse {
        let resp = wire::ListPublicKeysResponse {
            public_key_list: Some(vec![]),
            ..Default::default()
        };
        wire::serialize_list_public_keys_response(&resp)
    }

    async fn handle_create_event_data_store(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudTrailState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_event_data_store_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Name is required");
        }
        let name = input.name.as_str();
        let multi_region_enabled = input.multi_region_enabled.unwrap_or(true);
        let organization_enabled = input.organization_enabled.unwrap_or(false);
        let retention_period = input.retention_period.unwrap_or(2557);
        let termination_protection_enabled = input.termination_protection_enabled.unwrap_or(true);
        let tags: Vec<(String, String)> = input
            .tags_list
            .unwrap_or_default()
            .into_iter()
            .map(|t| (t.key, t.value.unwrap_or_default()))
            .collect();

        let mut state = state.write().await;
        match state.create_event_data_store(
            name,
            multi_region_enabled,
            organization_enabled,
            retention_period,
            termination_protection_enabled,
            tags,
            account_id,
            region,
        ) {
            Ok(eds) => {
                let created_ts = eds.created_timestamp.timestamp() as f64
                    + (eds.created_timestamp.timestamp_subsec_millis() as f64 / 1000.0);
                let updated_ts = eds.updated_timestamp.timestamp() as f64
                    + (eds.updated_timestamp.timestamp_subsec_millis() as f64 / 1000.0);
                let resp = wire::CreateEventDataStoreResponse {
                    event_data_store_arn: Some(eds.event_data_store_arn.clone()),
                    name: Some(eds.name.clone()),
                    status: Some(eds.status.clone()),
                    multi_region_enabled: Some(eds.multi_region_enabled),
                    organization_enabled: Some(eds.organization_enabled),
                    retention_period: Some(eds.retention_period),
                    termination_protection_enabled: Some(eds.termination_protection_enabled),
                    created_timestamp: Some(created_ts),
                    updated_timestamp: Some(updated_ts),
                    ..Default::default()
                };
                wire::serialize_create_event_data_store_response(&resp)
            }
            Err(e) => cloudtrail_error_response(&e),
        }
    }

    async fn handle_get_event_data_store(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudTrailState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_event_data_store_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.event_data_store.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "EventDataStore is required",
            );
        }

        let state = state.read().await;
        match state.get_event_data_store(&input.event_data_store) {
            Ok(eds) => {
                let created_ts = eds.created_timestamp.timestamp() as f64
                    + (eds.created_timestamp.timestamp_subsec_millis() as f64 / 1000.0);
                let updated_ts = eds.updated_timestamp.timestamp() as f64
                    + (eds.updated_timestamp.timestamp_subsec_millis() as f64 / 1000.0);
                let resp = wire::GetEventDataStoreResponse {
                    event_data_store_arn: Some(eds.event_data_store_arn.clone()),
                    name: Some(eds.name.clone()),
                    status: Some(eds.status.clone()),
                    multi_region_enabled: Some(eds.multi_region_enabled),
                    organization_enabled: Some(eds.organization_enabled),
                    retention_period: Some(eds.retention_period),
                    termination_protection_enabled: Some(eds.termination_protection_enabled),
                    created_timestamp: Some(created_ts),
                    updated_timestamp: Some(updated_ts),
                    ..Default::default()
                };
                wire::serialize_get_event_data_store_response(&resp)
            }
            Err(e) => cloudtrail_error_response(&e),
        }
    }

    async fn handle_delete_event_data_store(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudTrailState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_event_data_store_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.event_data_store.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "EventDataStore is required",
            );
        }

        let mut state = state.write().await;
        match state.delete_event_data_store(&input.event_data_store) {
            Ok(()) => wire::serialize_delete_event_data_store_response(
                &wire::DeleteEventDataStoreResponse {},
            ),
            Err(e) => cloudtrail_error_response(&e),
        }
    }

    async fn handle_list_event_data_stores(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudTrailState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let stores = state.list_event_data_stores();
        let entries: Vec<wire::EventDataStore> = stores
            .iter()
            .map(|eds| {
                let created_ts = eds.created_timestamp.timestamp() as f64
                    + (eds.created_timestamp.timestamp_subsec_millis() as f64 / 1000.0);
                let updated_ts = eds.updated_timestamp.timestamp() as f64
                    + (eds.updated_timestamp.timestamp_subsec_millis() as f64 / 1000.0);
                wire::EventDataStore {
                    event_data_store_arn: Some(eds.event_data_store_arn.clone()),
                    name: Some(eds.name.clone()),
                    status: Some(eds.status.clone()),
                    multi_region_enabled: Some(eds.multi_region_enabled),
                    organization_enabled: Some(eds.organization_enabled),
                    retention_period: Some(eds.retention_period),
                    termination_protection_enabled: Some(eds.termination_protection_enabled),
                    created_timestamp: Some(created_ts),
                    updated_timestamp: Some(updated_ts),
                    ..Default::default()
                }
            })
            .collect();
        let resp = wire::ListEventDataStoresResponse {
            event_data_stores: Some(entries),
            ..Default::default()
        };
        wire::serialize_list_event_data_stores_response(&resp)
    }

    async fn handle_update_event_data_store(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudTrailState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_event_data_store_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.event_data_store.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "EventDataStore is required",
            );
        }
        let arn_or_id = input.event_data_store.as_str();

        let name = input.name.as_deref();
        let multi_region_enabled = input.multi_region_enabled;
        let organization_enabled = input.organization_enabled;
        let retention_period = input.retention_period;
        let termination_protection_enabled = input.termination_protection_enabled;

        let mut state = state.write().await;
        match state.update_event_data_store(
            arn_or_id,
            name,
            multi_region_enabled,
            organization_enabled,
            retention_period,
            termination_protection_enabled,
        ) {
            Ok(eds) => {
                let created_ts = eds.created_timestamp.timestamp() as f64
                    + (eds.created_timestamp.timestamp_subsec_millis() as f64 / 1000.0);
                let updated_ts = eds.updated_timestamp.timestamp() as f64
                    + (eds.updated_timestamp.timestamp_subsec_millis() as f64 / 1000.0);
                let resp = wire::UpdateEventDataStoreResponse {
                    event_data_store_arn: Some(eds.event_data_store_arn.clone()),
                    name: Some(eds.name.clone()),
                    status: Some(eds.status.clone()),
                    multi_region_enabled: Some(eds.multi_region_enabled),
                    organization_enabled: Some(eds.organization_enabled),
                    retention_period: Some(eds.retention_period),
                    termination_protection_enabled: Some(eds.termination_protection_enabled),
                    created_timestamp: Some(created_ts),
                    updated_timestamp: Some(updated_ts),
                    ..Default::default()
                };
                wire::serialize_update_event_data_store_response(&resp)
            }
            Err(e) => cloudtrail_error_response(&e),
        }
    }
}

fn trail_to_wire(trail: &crate::types::Trail) -> wire::Trail {
    wire::Trail {
        name: Some(trail.name.clone()),
        s3_bucket_name: Some(trail.s3_bucket_name.clone()),
        s3_key_prefix: Some(trail.s3_key_prefix.clone()),
        include_global_service_events: Some(trail.include_global_service_events),
        is_multi_region_trail: Some(trail.is_multi_region_trail),
        trail_a_r_n: Some(trail.trail_arn.clone()),
        home_region: Some(trail.home_region.clone()),
        has_custom_event_selectors: Some(!trail.event_selectors.is_empty()),
        has_insight_selectors: Some(!trail.insight_selectors.is_empty()),
        ..Default::default()
    }
}

fn cloudtrail_error_response(err: &CloudTrailError) -> MockResponse {
    let (status, error_type) = match err {
        CloudTrailError::TrailNotFound(_) => (400u16, "TrailNotFoundException"),
        CloudTrailError::TrailAlreadyExists(_) => (400u16, "TrailAlreadyExistsException"),
        CloudTrailError::EventDataStoreNotFound(_) => (404u16, "EventDataStoreNotFoundException"),
        CloudTrailError::TerminationProtectionEnabled => (400u16, "InvalidParameterException"),
    };
    let body = json!({
        "__type": error_type,
        "message": err.to_string(),
    });
    MockResponse::json(status, body.to_string())
}
