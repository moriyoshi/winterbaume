use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{KinesisAnalyticsV2Error, KinesisAnalyticsV2State};
use crate::views::KinesisAnalyticsV2StateView;
use crate::wire;

pub struct KinesisAnalyticsV2Service {
    pub(crate) state: Arc<BackendState<KinesisAnalyticsV2State>>,
    pub(crate) notifier: StateChangeNotifier<KinesisAnalyticsV2StateView>,
}

impl KinesisAnalyticsV2Service {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for KinesisAnalyticsV2Service {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for KinesisAnalyticsV2Service {
    fn service_name(&self) -> &str {
        "kinesisanalytics"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://kinesisanalytics\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl KinesisAnalyticsV2Service {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        // Extract action from X-Amz-Target header
        // Format: "KinesisAnalytics_20180523.CreateApplication"
        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.rsplit('.').next())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        // Validate the body is well-formed JSON up-front; the typed deserialisers in
        // `wire` re-parse the bytes per operation.
        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let mutating = matches!(
            action.as_str(),
            "CreateApplication"
                | "DeleteApplication"
                | "StartApplication"
                | "StopApplication"
                | "UpdateApplication"
                | "UpdateApplicationMaintenanceConfiguration"
                | "TagResource"
                | "UntagResource"
                | "CreateApplicationSnapshot"
                | "DeleteApplicationSnapshot"
                | "RollbackApplication"
                | "AddApplicationCloudWatchLoggingOption"
                | "DeleteApplicationCloudWatchLoggingOption"
                | "AddApplicationVpcConfiguration"
                | "DeleteApplicationVpcConfiguration"
                | "AddApplicationInput"
                | "AddApplicationInputProcessingConfiguration"
                | "DeleteApplicationInputProcessingConfiguration"
                | "AddApplicationOutput"
                | "DeleteApplicationOutput"
                | "AddApplicationReferenceDataSource"
                | "DeleteApplicationReferenceDataSource"
        );

        let response = match action.as_str() {
            "CreateApplication" => {
                self.handle_create_application(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeApplication" => self.handle_describe_application(&state, body_bytes).await,
            "DeleteApplication" => self.handle_delete_application(&state, body_bytes).await,
            "ListApplications" => self.handle_list_applications(&state).await,
            "StartApplication" => self.handle_start_application(&state, body_bytes).await,
            "StopApplication" => self.handle_stop_application(&state, body_bytes).await,
            "UpdateApplication" => self.handle_update_application(&state, body_bytes).await,
            "UpdateApplicationMaintenanceConfiguration" => {
                self.handle_update_application_maintenance_configuration(&state, body_bytes)
                    .await
            }
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            "CreateApplicationSnapshot" => {
                self.handle_create_application_snapshot(&state, body_bytes)
                    .await
            }
            "DeleteApplicationSnapshot" => {
                self.handle_delete_application_snapshot(&state, body_bytes)
                    .await
            }
            "DescribeApplicationSnapshot" => {
                self.handle_describe_application_snapshot(&state, body_bytes)
                    .await
            }
            "ListApplicationSnapshots" => {
                self.handle_list_application_snapshots(&state, body_bytes)
                    .await
            }
            "ListApplicationVersions" => {
                self.handle_list_application_versions(&state, body_bytes)
                    .await
            }
            "DescribeApplicationVersion" => {
                self.handle_describe_application_version(&state, body_bytes)
                    .await
            }
            "RollbackApplication" => self.handle_rollback_application(&state, body_bytes).await,
            "CreateApplicationPresignedUrl" => {
                self.handle_create_application_presigned_url(&state, body_bytes)
                    .await
            }
            "AddApplicationCloudWatchLoggingOption" => {
                self.handle_add_application_cloud_watch_logging_option(&state, body_bytes)
                    .await
            }
            "DeleteApplicationCloudWatchLoggingOption" => {
                self.handle_delete_application_cloud_watch_logging_option(&state, body_bytes)
                    .await
            }
            "AddApplicationVpcConfiguration" => {
                self.handle_add_application_vpc_configuration(&state, body_bytes)
                    .await
            }
            "DeleteApplicationVpcConfiguration" => {
                self.handle_delete_application_vpc_configuration(&state, body_bytes)
                    .await
            }
            "AddApplicationInput" => self.handle_add_application_input(&state, body_bytes).await,
            "AddApplicationInputProcessingConfiguration" => {
                self.handle_add_application_input_processing_configuration(&state, body_bytes)
                    .await
            }
            "DeleteApplicationInputProcessingConfiguration" => {
                self.handle_delete_application_input_processing_configuration(&state, body_bytes)
                    .await
            }
            "AddApplicationOutput" => self.handle_add_application_output(&state, body_bytes).await,
            "DeleteApplicationOutput" => {
                self.handle_delete_application_output(&state, body_bytes)
                    .await
            }
            "AddApplicationReferenceDataSource" => {
                self.handle_add_application_reference_data_source(&state, body_bytes)
                    .await
            }
            "DeleteApplicationReferenceDataSource" => {
                self.handle_delete_application_reference_data_source(&state, body_bytes)
                    .await
            }
            "DiscoverInputSchema" => self.handle_discover_input_schema(body_bytes).await,
            "DescribeApplicationOperation" => {
                self.handle_describe_application_operation(&state, body_bytes)
                    .await
            }
            "ListApplicationOperations" => {
                self.handle_list_application_operations(&state, body_bytes)
                    .await
            }
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for KinesisAnalyticsV2"),
            ),
        };

        if mutating && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }

        response
    }

    async fn handle_create_application(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_application_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }
        if input.runtime_environment.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'RuntimeEnvironment'");
        }
        let service_execution_role = if input.service_execution_role.is_empty() {
            None
        } else {
            Some(input.service_execution_role.as_str())
        };
        let application_description = input.application_description.as_deref();

        let mut state = state.write().await;
        match state.create_application(
            &input.application_name,
            &input.runtime_environment,
            service_execution_role,
            application_description,
            account_id,
            region,
        ) {
            Ok(app) => wire::serialize_create_application_response(
                &crate::model::CreateApplicationResponse {
                    application_detail: Some(application_to_wire_detail(app)),
                },
            ),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_describe_application(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_application_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }

        let state = state.read().await;
        match state.describe_application(&input.application_name) {
            Ok(app) => wire::serialize_describe_application_response(
                &crate::model::DescribeApplicationResponse {
                    application_detail: Some(application_to_wire_detail(app)),
                },
            ),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_delete_application(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_application_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }

        let mut state = state.write().await;
        match state.delete_application(&input.application_name, Some(input.create_timestamp)) {
            Ok(()) => wire::serialize_delete_application_response(
                &crate::model::DeleteApplicationResponse {},
            ),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_list_applications(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
    ) -> MockResponse {
        let state = state.read().await;
        let apps = state.list_applications();

        let summaries: Vec<crate::model::ApplicationSummary> = apps
            .iter()
            .map(|app| application_to_wire_summary(app))
            .collect();

        wire::serialize_list_applications_response(&crate::model::ListApplicationsResponse {
            application_summaries: Some(summaries),
            next_token: None,
        })
    }

    async fn handle_start_application(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_application_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }

        let mut state = state.write().await;
        match state.start_application(&input.application_name) {
            Ok(()) => wire::serialize_start_application_response(
                &crate::model::StartApplicationResponse::default(),
            ),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_stop_application(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_application_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }

        let mut state = state.write().await;
        match state.stop_application(&input.application_name) {
            Ok(()) => wire::serialize_stop_application_response(
                &crate::model::StopApplicationResponse::default(),
            ),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_update_application(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_application_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }

        let runtime_environment_update = input.runtime_environment_update.as_deref();
        let service_execution_role_update = input.service_execution_role_update.as_deref();

        let mut state = state.write().await;
        match state.update_application(
            &input.application_name,
            runtime_environment_update,
            service_execution_role_update,
        ) {
            Ok(app) => wire::serialize_update_application_response(
                &crate::model::UpdateApplicationResponse {
                    application_detail: Some(application_to_wire_detail(app)),
                    ..Default::default()
                },
            ),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_update_application_maintenance_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_application_maintenance_configuration_request(body) {
                Ok(v) => v,
                Err(e) => return json_error_response(400, "ValidationException", &e),
            };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }

        let mut state = state.write().await;
        match state.update_application_maintenance_configuration(&input.application_name) {
            Ok(app) => wire::serialize_update_application_maintenance_configuration_response(
                &crate::model::UpdateApplicationMaintenanceConfigurationResponse {
                    application_a_r_n: Some(app.application_arn.clone()),
                    ..Default::default()
                },
            ),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceARN'");
        }

        let tags: Vec<(String, String)> = input
            .tags
            .into_iter()
            .map(|t| (t.key, t.value.unwrap_or_default()))
            .collect();

        let mut state = state.write().await;
        match state.tag_resource(&input.resource_a_r_n, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&crate::model::TagResourceResponse {}),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceARN'");
        }

        let mut state = state.write().await;
        match state.untag_resource(&input.resource_a_r_n, &input.tag_keys) {
            Ok(()) => {
                wire::serialize_untag_resource_response(&crate::model::UntagResourceResponse {})
            }
            Err(e) => error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceARN'");
        }

        let state = state.read().await;
        match state.list_tags_for_resource(&input.resource_a_r_n) {
            Ok(tags) => {
                let wire_tags: Vec<crate::model::Tag> = tags
                    .into_iter()
                    .map(|(k, v)| crate::model::Tag {
                        key: k,
                        value: Some(v),
                    })
                    .collect();
                wire::serialize_list_tags_for_resource_response(
                    &crate::model::ListTagsForResourceResponse {
                        tags: Some(wire_tags),
                    },
                )
            }
            Err(e) => error_response(&e),
        }
    }

    async fn handle_create_application_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_application_snapshot_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }
        if input.snapshot_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SnapshotName'");
        }

        let mut state = state.write().await;
        match state.create_application_snapshot(&input.application_name, &input.snapshot_name) {
            Ok(()) => wire::serialize_create_application_snapshot_response(
                &crate::model::CreateApplicationSnapshotResponse {},
            ),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_delete_application_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_application_snapshot_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }
        if input.snapshot_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SnapshotName'");
        }

        let mut state = state.write().await;
        match state.delete_application_snapshot(&input.application_name, &input.snapshot_name) {
            Ok(()) => wire::serialize_delete_application_snapshot_response(
                &crate::model::DeleteApplicationSnapshotResponse {},
            ),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_describe_application_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_application_snapshot_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }
        if input.snapshot_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SnapshotName'");
        }

        let state = state.read().await;
        match state.describe_application_snapshot(&input.application_name, &input.snapshot_name) {
            Ok(snapshot) => wire::serialize_describe_application_snapshot_response(
                &crate::model::DescribeApplicationSnapshotResponse {
                    snapshot_details: Some(snapshot_to_wire(snapshot)),
                },
            ),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_list_application_snapshots(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_application_snapshots_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }

        let state = state.read().await;
        match state.list_application_snapshots(&input.application_name) {
            Ok(snapshots) => {
                let wire_snapshots: Vec<crate::model::SnapshotDetails> =
                    snapshots.iter().map(|s| snapshot_to_wire(s)).collect();
                wire::serialize_list_application_snapshots_response(
                    &crate::model::ListApplicationSnapshotsResponse {
                        snapshot_summaries: Some(wire_snapshots),
                        next_token: None,
                    },
                )
            }
            Err(e) => error_response(&e),
        }
    }

    async fn handle_list_application_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_application_versions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }

        let state = state.read().await;
        match state.list_application_versions(&input.application_name) {
            Ok(versions) => {
                let app = state.applications.get(&input.application_name).unwrap();
                let summaries: Vec<crate::model::ApplicationVersionSummary> = versions
                    .into_iter()
                    .map(|v| crate::model::ApplicationVersionSummary {
                        application_version_id: Some(v),
                        application_status: Some(app.application_status.clone()),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_application_versions_response(
                    &crate::model::ListApplicationVersionsResponse {
                        application_version_summaries: Some(summaries),
                        next_token: None,
                    },
                )
            }
            Err(e) => error_response(&e),
        }
    }

    async fn handle_describe_application_version(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_application_version_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }
        let application_version_id = if input.application_version_id == 0 {
            1
        } else {
            input.application_version_id
        };

        let state = state.read().await;
        match state.describe_application_version(&input.application_name, application_version_id) {
            Ok(app) => wire::serialize_describe_application_version_response(
                &crate::model::DescribeApplicationVersionResponse {
                    application_version_detail: Some(application_to_wire_detail(app)),
                },
            ),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_rollback_application(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_rollback_application_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }

        let mut state = state.write().await;
        match state.rollback_application(&input.application_name) {
            Ok(app) => wire::serialize_rollback_application_response(
                &crate::model::RollbackApplicationResponse {
                    application_detail: Some(application_to_wire_detail(app)),
                    ..Default::default()
                },
            ),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_create_application_presigned_url(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_application_presigned_url_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }

        let state = state.read().await;
        match state.describe_application(&input.application_name) {
            Ok(app) => {
                let url = format!(
                    "https://kinesisanalytics.us-east-1.amazonaws.com/presigned/{}",
                    app.application_name
                );
                wire::serialize_create_application_presigned_url_response(
                    &crate::model::CreateApplicationPresignedUrlResponse {
                        authorized_url: Some(url),
                    },
                )
            }
            Err(e) => error_response(&e),
        }
    }

    async fn handle_add_application_cloud_watch_logging_option(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_add_application_cloud_watch_logging_option_request(body)
        {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }

        let mut state = state.write().await;
        match state.add_application_cloud_watch_logging_option(&input.application_name) {
            Ok(app) => wire::serialize_add_application_cloud_watch_logging_option_response(
                &crate::model::AddApplicationCloudWatchLoggingOptionResponse {
                    application_a_r_n: Some(app.application_arn.clone()),
                    application_version_id: Some(app.application_version_id),
                    ..Default::default()
                },
            ),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_delete_application_cloud_watch_logging_option(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_application_cloud_watch_logging_option_request(body) {
                Ok(v) => v,
                Err(e) => return json_error_response(400, "ValidationException", &e),
            };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }

        let mut state = state.write().await;
        match state.delete_application_cloud_watch_logging_option(&input.application_name) {
            Ok(app) => wire::serialize_delete_application_cloud_watch_logging_option_response(
                &crate::model::DeleteApplicationCloudWatchLoggingOptionResponse {
                    application_a_r_n: Some(app.application_arn.clone()),
                    application_version_id: Some(app.application_version_id),
                    ..Default::default()
                },
            ),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_add_application_vpc_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_add_application_vpc_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }

        let mut state = state.write().await;
        match state.add_application_vpc_configuration(&input.application_name) {
            Ok(app) => wire::serialize_add_application_vpc_configuration_response(
                &crate::model::AddApplicationVpcConfigurationResponse {
                    application_a_r_n: Some(app.application_arn.clone()),
                    application_version_id: Some(app.application_version_id),
                    ..Default::default()
                },
            ),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_delete_application_vpc_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_application_vpc_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }

        let mut state = state.write().await;
        match state.delete_application_vpc_configuration(&input.application_name) {
            Ok(app) => wire::serialize_delete_application_vpc_configuration_response(
                &crate::model::DeleteApplicationVpcConfigurationResponse {
                    application_a_r_n: Some(app.application_arn.clone()),
                    application_version_id: Some(app.application_version_id),
                    ..Default::default()
                },
            ),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_add_application_input(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_add_application_input_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }

        let mut state = state.write().await;
        match state.add_application_input(&input.application_name) {
            Ok(app) => wire::serialize_add_application_input_response(
                &crate::model::AddApplicationInputResponse {
                    application_a_r_n: Some(app.application_arn.clone()),
                    application_version_id: Some(app.application_version_id),
                    ..Default::default()
                },
            ),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_add_application_input_processing_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input =
            match wire::deserialize_add_application_input_processing_configuration_request(body) {
                Ok(v) => v,
                Err(e) => return json_error_response(400, "ValidationException", &e),
            };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }

        let mut state = state.write().await;
        match state.add_application_input_processing_configuration(&input.application_name) {
            Ok(app) => wire::serialize_add_application_input_processing_configuration_response(
                &crate::model::AddApplicationInputProcessingConfigurationResponse {
                    application_a_r_n: Some(app.application_arn.clone()),
                    application_version_id: Some(app.application_version_id),
                    ..Default::default()
                },
            ),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_delete_application_input_processing_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_application_input_processing_configuration_request(body)
            {
                Ok(v) => v,
                Err(e) => return json_error_response(400, "ValidationException", &e),
            };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }

        let mut state = state.write().await;
        match state.delete_application_input_processing_configuration(&input.application_name) {
            Ok(app) => wire::serialize_delete_application_input_processing_configuration_response(
                &crate::model::DeleteApplicationInputProcessingConfigurationResponse {
                    application_a_r_n: Some(app.application_arn.clone()),
                    application_version_id: Some(app.application_version_id),
                },
            ),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_add_application_output(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_add_application_output_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }

        let mut state = state.write().await;
        match state.add_application_output(&input.application_name) {
            Ok(app) => wire::serialize_add_application_output_response(
                &crate::model::AddApplicationOutputResponse {
                    application_a_r_n: Some(app.application_arn.clone()),
                    application_version_id: Some(app.application_version_id),
                    ..Default::default()
                },
            ),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_delete_application_output(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_application_output_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }

        let mut state = state.write().await;
        match state.delete_application_output(&input.application_name) {
            Ok(app) => wire::serialize_delete_application_output_response(
                &crate::model::DeleteApplicationOutputResponse {
                    application_a_r_n: Some(app.application_arn.clone()),
                    application_version_id: Some(app.application_version_id),
                },
            ),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_add_application_reference_data_source(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_add_application_reference_data_source_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }

        let mut state = state.write().await;
        match state.add_application_reference_data_source(&input.application_name) {
            Ok(app) => wire::serialize_add_application_reference_data_source_response(
                &crate::model::AddApplicationReferenceDataSourceResponse {
                    application_a_r_n: Some(app.application_arn.clone()),
                    application_version_id: Some(app.application_version_id),
                    ..Default::default()
                },
            ),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_delete_application_reference_data_source(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_application_reference_data_source_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }

        let mut state = state.write().await;
        match state.delete_application_reference_data_source(&input.application_name) {
            Ok(app) => wire::serialize_delete_application_reference_data_source_response(
                &crate::model::DeleteApplicationReferenceDataSourceResponse {
                    application_a_r_n: Some(app.application_arn.clone()),
                    application_version_id: Some(app.application_version_id),
                },
            ),
            Err(e) => error_response(&e),
        }
    }

    // STUB[no-engine]: DiscoverInputSchema requires a real data stream and schema inference engine; always returns empty.
    async fn handle_discover_input_schema(&self, _body: &[u8]) -> MockResponse {
        wire::serialize_discover_input_schema_response(&crate::model::DiscoverInputSchemaResponse {
            ..Default::default()
        })
    }

    async fn handle_describe_application_operation(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_application_operation_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }

        let state = state.read().await;
        match state.describe_application_operation(&input.application_name) {
            Ok(_) => wire::serialize_describe_application_operation_response(
                &crate::model::DescribeApplicationOperationResponse {
                    ..Default::default()
                },
            ),
            Err(e) => error_response(&e),
        }
    }

    async fn handle_list_application_operations(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisAnalyticsV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_application_operations_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.application_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ApplicationName'");
        }

        let state = state.read().await;
        match state.list_application_operations(&input.application_name) {
            Ok(()) => wire::serialize_list_application_operations_response(
                &crate::model::ListApplicationOperationsResponse {
                    application_operation_info_list: Some(vec![]),
                    next_token: None,
                },
            ),
            Err(e) => error_response(&e),
        }
    }
}

fn application_to_wire_detail(app: &crate::types::Application) -> crate::model::ApplicationDetail {
    crate::model::ApplicationDetail {
        application_a_r_n: Some(app.application_arn.clone()),
        application_name: Some(app.application_name.clone()),
        application_status: Some(app.application_status.clone()),
        application_version_id: Some(app.application_version_id),
        runtime_environment: Some(app.runtime_environment.clone()),
        create_timestamp: Some(app.create_timestamp.timestamp() as f64),
        last_update_timestamp: Some(app.last_update_timestamp.timestamp() as f64),
        application_configuration_description: Some(
            crate::model::ApplicationConfigurationDescription::default(),
        ),
        service_execution_role: app.service_execution_role.clone(),
        application_description: app.application_description.clone(),
        ..Default::default()
    }
}

fn application_to_wire_summary(
    app: &crate::types::Application,
) -> crate::model::ApplicationSummary {
    crate::model::ApplicationSummary {
        application_a_r_n: Some(app.application_arn.clone()),
        application_name: Some(app.application_name.clone()),
        application_status: Some(app.application_status.clone()),
        application_version_id: Some(app.application_version_id),
        runtime_environment: Some(app.runtime_environment.clone()),
        ..Default::default()
    }
}

fn snapshot_to_wire(snapshot: &crate::types::Snapshot) -> crate::model::SnapshotDetails {
    crate::model::SnapshotDetails {
        snapshot_name: Some(snapshot.snapshot_name.clone()),
        application_version_id: Some(snapshot.application_version_id),
        runtime_environment: Some(snapshot.runtime_environment.clone()),
        snapshot_creation_timestamp: Some(snapshot.snapshot_creation_timestamp.timestamp() as f64),
        ..Default::default()
    }
}

fn error_response(err: &KinesisAnalyticsV2Error) -> MockResponse {
    let (status, error_type) = match err {
        KinesisAnalyticsV2Error::ApplicationAlreadyExists(_) => (409, "ResourceInUseException"),
        KinesisAnalyticsV2Error::ApplicationNotFound(_) => (400, "ResourceNotFoundException"),
        KinesisAnalyticsV2Error::ApplicationArnNotFound(_) => (400, "ResourceNotFoundException"),
        KinesisAnalyticsV2Error::SnapshotAlreadyExists(_) => (409, "ResourceInUseException"),
        KinesisAnalyticsV2Error::SnapshotNotFound(_) => (400, "ResourceNotFoundException"),
    };
    let body = json!({
        "__type": error_type,
        "message": err.to_string(),
    });
    MockResponse::json(status, body.to_string())
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}
