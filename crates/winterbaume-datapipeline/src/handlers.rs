use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::state::{DataPipelineError, DataPipelineState};
use crate::types;
use crate::views::DataPipelineStateView;
use crate::wire;

pub struct DataPipelineService {
    pub(crate) state: Arc<BackendState<DataPipelineState>>,
    pub(crate) notifier: StateChangeNotifier<DataPipelineStateView>,
}

impl DataPipelineService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for DataPipelineService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for DataPipelineService {
    fn service_name(&self) -> &str {
        "datapipeline"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://datapipeline\..*\.amazonaws\.com",
            r"https?://datapipeline\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl DataPipelineService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        // Extract action from X-Amz-Target header
        // Format: "DataPipeline.CreatePipeline"
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
        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        use winterbaume_core::StatefulService;
        let response = match action.as_str() {
            "ActivatePipeline" => self.handle_activate_pipeline(&state, body_bytes).await,
            "AddTags" => self.handle_add_tags(&state, body_bytes).await,
            "CreatePipeline" => self.handle_create_pipeline(&state, body_bytes).await,
            "DeactivatePipeline" => self.handle_deactivate_pipeline(&state, body_bytes).await,
            "DeletePipeline" => self.handle_delete_pipeline(&state, body_bytes).await,
            "DescribeObjects" => self.handle_describe_objects(&state, body_bytes).await,
            "DescribePipelines" => self.handle_describe_pipelines(&state, body_bytes).await,
            "EvaluateExpression" => self.handle_evaluate_expression(&state, body_bytes).await,
            "GetPipelineDefinition" => {
                self.handle_get_pipeline_definition(&state, body_bytes)
                    .await
            }
            "ListPipelines" => self.handle_list_pipelines(&state).await,
            "PollForTask" => self.handle_poll_for_task(&state, body_bytes).await,
            "PutPipelineDefinition" => {
                self.handle_put_pipeline_definition(&state, body_bytes)
                    .await
            }
            "QueryObjects" => self.handle_query_objects(&state, body_bytes).await,
            "RemoveTags" => self.handle_remove_tags(&state, body_bytes).await,
            "ReportTaskProgress" => self.handle_report_task_progress(&state, body_bytes).await,
            "ReportTaskRunnerHeartbeat" => {
                self.handle_report_task_runner_heartbeat(&state, body_bytes)
                    .await
            }
            "SetStatus" => self.handle_set_status(&state, body_bytes).await,
            "SetTaskStatus" => self.handle_set_task_status(&state, body_bytes).await,
            "ValidatePipelineDefinition" => {
                self.handle_validate_pipeline_definition(&state, body_bytes)
                    .await
            }
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for DataPipeline"),
            ),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_pipeline(
        &self,
        state: &Arc<tokio::sync::RwLock<DataPipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_pipeline_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'name'");
        }
        if input.unique_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'uniqueId'");
        }
        let name = input.name.as_str();
        let unique_id = input.unique_id.as_str();
        let description = input.description.as_deref();

        let tags: Vec<types::PipelineTag> = input
            .tags
            .unwrap_or_default()
            .into_iter()
            .map(|t| types::PipelineTag {
                key: t.key,
                value: t.value,
            })
            .collect();

        let mut state = state.write().await;
        match state.create_pipeline(name, unique_id, description, tags) {
            Ok(pipeline) => wire::serialize_create_pipeline_response(&wire::CreatePipelineOutput {
                pipeline_id: Some(pipeline.pipeline_id.clone()),
            }),
            Err(e) => datapipeline_error_response(&e),
        }
    }

    async fn handle_delete_pipeline(
        &self,
        state: &Arc<tokio::sync::RwLock<DataPipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_pipeline_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.pipeline_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'pipelineId'");
        }

        let mut state = state.write().await;
        match state.delete_pipeline(&input.pipeline_id) {
            Ok(()) => wire::serialize_delete_pipeline_response(),
            Err(e) => datapipeline_error_response(&e),
        }
    }

    async fn handle_describe_pipelines(
        &self,
        state: &Arc<tokio::sync::RwLock<DataPipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_pipelines_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let pipeline_ids = input.pipeline_ids;

        if pipeline_ids.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'pipelineIds'");
        }

        let state = state.read().await;
        match state.describe_pipelines(&pipeline_ids) {
            Ok(pipelines) => {
                let pipeline_descriptions: Vec<wire::PipelineDescription> = pipelines
                    .iter()
                    .map(|p| pipeline_to_description(p))
                    .collect();

                wire::serialize_describe_pipelines_response(&wire::DescribePipelinesOutput {
                    pipeline_description_list: Some(pipeline_descriptions),
                })
            }
            Err(e) => datapipeline_error_response(&e),
        }
    }

    async fn handle_list_pipelines(
        &self,
        state: &Arc<tokio::sync::RwLock<DataPipelineState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let pipelines = state.list_pipelines();

        let pipeline_list: Vec<wire::PipelineIdName> = pipelines
            .iter()
            .map(|p| wire::PipelineIdName {
                id: Some(p.pipeline_id.clone()),
                name: Some(p.name.clone()),
            })
            .collect();

        wire::serialize_list_pipelines_response(&wire::ListPipelinesOutput {
            pipeline_id_list: Some(pipeline_list),
            has_more_results: Some(false),
            marker: None,
        })
    }

    async fn handle_get_pipeline_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<DataPipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_pipeline_definition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.pipeline_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'pipelineId'");
        }

        let state = state.read().await;
        match state.get_pipeline_definition(&input.pipeline_id) {
            Ok(pipeline) => {
                let objects: Vec<wire::PipelineObject> = pipeline
                    .pipeline_objects
                    .iter()
                    .map(pipeline_object_to_wire)
                    .collect();

                let parameter_objects = values_to_parameter_objects(&pipeline.parameter_objects);
                let parameter_values = values_to_parameter_values(&pipeline.parameter_values);

                wire::serialize_get_pipeline_definition_response(
                    &wire::GetPipelineDefinitionOutput {
                        pipeline_objects: if objects.is_empty() {
                            None
                        } else {
                            Some(objects)
                        },
                        parameter_objects,
                        parameter_values,
                    },
                )
            }
            Err(e) => datapipeline_error_response(&e),
        }
    }

    async fn handle_put_pipeline_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<DataPipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_pipeline_definition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.pipeline_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'pipelineId'");
        }

        let pipeline_objects: Vec<types::PipelineObject> = input
            .pipeline_objects
            .into_iter()
            .map(wire_pipeline_object_to_types)
            .collect();

        let parameter_objects: Vec<Value> = input
            .parameter_objects
            .unwrap_or_default()
            .iter()
            .filter_map(|p| serde_json::to_value(p).ok())
            .collect();

        let parameter_values: Vec<Value> = input
            .parameter_values
            .unwrap_or_default()
            .iter()
            .filter_map(|p| serde_json::to_value(p).ok())
            .collect();

        let mut state = state.write().await;
        match state.put_pipeline_definition(
            &input.pipeline_id,
            pipeline_objects,
            parameter_objects,
            parameter_values,
        ) {
            Ok(()) => wire::serialize_put_pipeline_definition_response(
                &wire::PutPipelineDefinitionOutput {
                    errored: Some(false),
                    validation_errors: None,
                    validation_warnings: None,
                },
            ),
            Err(e) => datapipeline_error_response(&e),
        }
    }

    async fn handle_activate_pipeline(
        &self,
        state: &Arc<tokio::sync::RwLock<DataPipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_activate_pipeline_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.pipeline_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'pipelineId'");
        }
        let mut state = state.write().await;
        match state.activate_pipeline(&input.pipeline_id) {
            Ok(()) => wire::serialize_activate_pipeline_response(&wire::ActivatePipelineOutput {}),
            Err(e) => datapipeline_error_response(&e),
        }
    }

    async fn handle_deactivate_pipeline(
        &self,
        state: &Arc<tokio::sync::RwLock<DataPipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_deactivate_pipeline_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.pipeline_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'pipelineId'");
        }
        let mut state = state.write().await;
        match state.deactivate_pipeline(&input.pipeline_id) {
            Ok(()) => {
                wire::serialize_deactivate_pipeline_response(&wire::DeactivatePipelineOutput {})
            }
            Err(e) => datapipeline_error_response(&e),
        }
    }

    async fn handle_add_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<DataPipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_add_tags_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.pipeline_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'pipelineId'");
        }
        let tags: Vec<types::PipelineTag> = input
            .tags
            .into_iter()
            .map(|t| types::PipelineTag {
                key: t.key,
                value: t.value,
            })
            .collect();

        let mut state = state.write().await;
        match state.add_tags(&input.pipeline_id, tags) {
            Ok(()) => wire::serialize_add_tags_response(&wire::AddTagsOutput {}),
            Err(e) => datapipeline_error_response(&e),
        }
    }

    async fn handle_remove_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<DataPipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_remove_tags_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.pipeline_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'pipelineId'");
        }

        let mut state = state.write().await;
        match state.remove_tags(&input.pipeline_id, &input.tag_keys) {
            Ok(()) => wire::serialize_remove_tags_response(&wire::RemoveTagsOutput {}),
            Err(e) => datapipeline_error_response(&e),
        }
    }

    async fn handle_describe_objects(
        &self,
        state: &Arc<tokio::sync::RwLock<DataPipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_objects_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.pipeline_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'pipelineId'");
        }

        let state = state.read().await;
        match state.describe_objects(&input.pipeline_id, &input.object_ids) {
            Ok(objects) => {
                let pipeline_objects: Vec<wire::PipelineObject> =
                    objects.iter().map(|o| pipeline_object_to_wire(o)).collect();
                wire::serialize_describe_objects_response(&wire::DescribeObjectsOutput {
                    pipeline_objects: if pipeline_objects.is_empty() {
                        None
                    } else {
                        Some(pipeline_objects)
                    },
                    has_more_results: Some(false),
                    marker: None,
                })
            }
            Err(e) => datapipeline_error_response(&e),
        }
    }

    async fn handle_query_objects(
        &self,
        state: &Arc<tokio::sync::RwLock<DataPipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_query_objects_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.pipeline_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'pipelineId'");
        }
        let sphere = if input.sphere.is_empty() {
            "COMPONENT"
        } else {
            input.sphere.as_str()
        };

        let state = state.read().await;
        match state.query_objects(&input.pipeline_id, sphere) {
            Ok(ids) => wire::serialize_query_objects_response(&wire::QueryObjectsOutput {
                ids: if ids.is_empty() { None } else { Some(ids) },
                has_more_results: Some(false),
                marker: None,
            }),
            Err(e) => datapipeline_error_response(&e),
        }
    }

    async fn handle_set_status(
        &self,
        state: &Arc<tokio::sync::RwLock<DataPipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_set_status_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.pipeline_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'pipelineId'");
        }
        if input.status.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'status'");
        }

        let mut state = state.write().await;
        match state.set_status(&input.pipeline_id, &input.object_ids, &input.status) {
            Ok(()) => wire::serialize_set_status_response(),
            Err(e) => datapipeline_error_response(&e),
        }
    }

    async fn handle_poll_for_task(
        &self,
        state: &Arc<tokio::sync::RwLock<DataPipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_poll_for_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let worker_group = input.worker_group.as_str();

        let mut state = state.write().await;
        let task = state.poll_for_task(worker_group);
        match task {
            Some((task_id, pipeline_id, attempt_id)) => {
                wire::serialize_poll_for_task_response(&wire::PollForTaskOutput {
                    task_object: Some(wire::TaskObject {
                        task_id: Some(task_id),
                        pipeline_id: Some(pipeline_id),
                        attempt_id: Some(attempt_id),
                        objects: None,
                    }),
                })
            }
            None => wire::serialize_poll_for_task_response(&wire::PollForTaskOutput {
                task_object: None,
            }),
        }
    }

    async fn handle_report_task_progress(
        &self,
        state: &Arc<tokio::sync::RwLock<DataPipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_report_task_progress_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.task_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'taskId'");
        }
        let mut state = state.write().await;
        match state.report_task_progress(&input.task_id) {
            Ok(canceled) => {
                wire::serialize_report_task_progress_response(&wire::ReportTaskProgressOutput {
                    canceled: Some(canceled),
                })
            }
            Err(e) => datapipeline_error_response(&e),
        }
    }

    async fn handle_report_task_runner_heartbeat(
        &self,
        state: &Arc<tokio::sync::RwLock<DataPipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_report_task_runner_heartbeat_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.taskrunner_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'taskrunnerId'");
        }
        let state = state.read().await;
        let terminate = state.report_task_runner_heartbeat(&input.taskrunner_id);
        wire::serialize_report_task_runner_heartbeat_response(
            &wire::ReportTaskRunnerHeartbeatOutput {
                terminate: Some(terminate),
            },
        )
    }

    async fn handle_set_task_status(
        &self,
        state: &Arc<tokio::sync::RwLock<DataPipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_set_task_status_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.task_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'taskId'");
        }
        if input.task_status.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'taskStatus'");
        }
        let mut state = state.write().await;
        match state.set_task_status(&input.task_id, &input.task_status) {
            Ok(()) => wire::serialize_set_task_status_response(&wire::SetTaskStatusOutput {}),
            Err(e) => datapipeline_error_response(&e),
        }
    }

    async fn handle_evaluate_expression(
        &self,
        state: &Arc<tokio::sync::RwLock<DataPipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_evaluate_expression_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.pipeline_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'pipelineId'");
        }
        if input.object_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'objectId'");
        }
        if input.expression.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'expression'");
        }
        let state = state.read().await;
        match state.evaluate_expression(&input.pipeline_id, &input.object_id, &input.expression) {
            Ok(evaluated) => {
                wire::serialize_evaluate_expression_response(&wire::EvaluateExpressionOutput {
                    evaluated_expression: Some(evaluated),
                })
            }
            Err(e) => datapipeline_error_response(&e),
        }
    }

    async fn handle_validate_pipeline_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<DataPipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_validate_pipeline_definition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.pipeline_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'pipelineId'");
        }

        let pipeline_objects: Vec<types::PipelineObject> = input
            .pipeline_objects
            .into_iter()
            .map(wire_pipeline_object_to_types)
            .collect();
        let _ = pipeline_objects; // consumed for validation but not actually validated in mock

        let state = state.read().await;
        match state.validate_pipeline_definition(&input.pipeline_id) {
            Ok(()) => wire::serialize_validate_pipeline_definition_response(
                &wire::ValidatePipelineDefinitionOutput {
                    errored: Some(false),
                    validation_errors: None,
                    validation_warnings: None,
                },
            ),
            Err(e) => datapipeline_error_response(&e),
        }
    }
}

// State-to-wire conversion helpers

fn pipeline_to_description(p: &types::Pipeline) -> wire::PipelineDescription {
    let tags: Vec<wire::Tag> = p
        .tags
        .iter()
        .map(|t| wire::Tag {
            key: t.key.clone(),
            value: t.value.clone(),
        })
        .collect();

    let mut fields: Vec<wire::Field> = vec![
        wire::Field {
            key: "@pipelineState".to_string(),
            string_value: Some(p.status.as_str().to_string()),
            ref_value: None,
        },
        wire::Field {
            key: "name".to_string(),
            string_value: Some(p.name.clone()),
            ref_value: None,
        },
        wire::Field {
            key: "uniqueId".to_string(),
            string_value: Some(p.unique_id.clone()),
            ref_value: None,
        },
    ];

    if !p.description.is_empty() {
        fields.push(wire::Field {
            key: "description".to_string(),
            string_value: Some(p.description.clone()),
            ref_value: None,
        });
    }

    wire::PipelineDescription {
        pipeline_id: Some(p.pipeline_id.clone()),
        name: Some(p.name.clone()),
        description: if p.description.is_empty() {
            None
        } else {
            Some(p.description.clone())
        },
        fields: Some(fields),
        tags: if tags.is_empty() { None } else { Some(tags) },
    }
}

fn pipeline_object_to_wire(obj: &types::PipelineObject) -> wire::PipelineObject {
    let fields: Vec<wire::Field> = obj
        .fields
        .iter()
        .map(|f| wire::Field {
            key: f.key.clone(),
            string_value: f.string_value.clone(),
            ref_value: f.ref_value.clone(),
        })
        .collect();

    wire::PipelineObject {
        id: obj.id.clone(),
        name: obj.name.clone(),
        fields,
    }
}

fn wire_pipeline_object_to_types(obj: wire::PipelineObject) -> types::PipelineObject {
    let fields: Vec<types::PipelineField> = obj
        .fields
        .into_iter()
        .map(|f| types::PipelineField {
            key: f.key,
            string_value: f.string_value,
            ref_value: f.ref_value,
        })
        .collect();
    types::PipelineObject {
        id: obj.id,
        name: obj.name,
        fields,
    }
}

fn values_to_parameter_objects(values: &[Value]) -> Option<Vec<wire::ParameterObject>> {
    if values.is_empty() {
        return None;
    }
    let objs: Vec<wire::ParameterObject> = values
        .iter()
        .filter_map(|v| {
            let id = v.get("id")?.as_str()?.to_string();
            let attributes = v
                .get("attributes")
                .and_then(|a| a.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|a| {
                            Some(wire::ParameterAttribute {
                                key: a.get("key")?.as_str()?.to_string(),
                                string_value: a.get("stringValue")?.as_str()?.to_string(),
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();
            Some(wire::ParameterObject { id, attributes })
        })
        .collect();
    if objs.is_empty() { None } else { Some(objs) }
}

fn values_to_parameter_values(values: &[Value]) -> Option<Vec<wire::ParameterValue>> {
    if values.is_empty() {
        return None;
    }
    let vals: Vec<wire::ParameterValue> = values
        .iter()
        .filter_map(|v| {
            Some(wire::ParameterValue {
                id: v.get("id")?.as_str()?.to_string(),
                string_value: v.get("stringValue")?.as_str()?.to_string(),
            })
        })
        .collect();
    if vals.is_empty() { None } else { Some(vals) }
}

fn datapipeline_error_response(err: &DataPipelineError) -> MockResponse {
    let (status, error_type) = match err {
        DataPipelineError::PipelineNotFound { .. } => (400, "PipelineNotFoundException"),
        DataPipelineError::DuplicateUniqueId { .. } => (400, "InvalidRequestException"),
        DataPipelineError::TaskNotFound { .. } => (400, "TaskNotFoundException"),
        DataPipelineError::InvalidTaskStatus { .. } => (400, "InvalidRequestException"),
        DataPipelineError::ObjectNotFound { .. } => (400, "ObjectNotFoundException"),
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
