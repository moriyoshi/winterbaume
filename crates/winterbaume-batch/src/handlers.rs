use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{BatchError, BatchState};
use crate::types::*;
use crate::views::BatchStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct BatchService {
    pub(crate) state: Arc<BackendState<BatchState>>,
    pub(crate) notifier: StateChangeNotifier<BatchStateView>,
}

impl BatchService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for BatchService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for BatchService {
    fn service_name(&self) -> &str {
        "batch"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://batch\..*\.amazonaws\.com",
            r"https?://batch\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl BatchService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();

        let body: Value = match serde_json::from_slice(&request.body) {
            Ok(v) => v,
            Err(_) => Value::Object(serde_json::Map::new()),
        };

        let response = match (method, path.as_str()) {
            // Existing operations
            ("POST", "/v1/createjobqueue") => {
                self.handle_create_job_queue(&state, &body, account_id, &region)
                    .await
            }
            ("POST", "/v1/describejobqueues") => {
                self.handle_describe_job_queues(&state, &body).await
            }
            ("POST", "/v1/deletejobqueue") => self.handle_delete_job_queue(&state, &body).await,
            ("POST", "/v1/updatejobqueue") => self.handle_update_job_queue(&state, &body).await,
            ("POST", "/v1/registerjobdefinition") => {
                self.handle_register_job_definition(&state, &body, account_id, &region)
                    .await
            }
            ("POST", "/v1/describejobdefinitions") => {
                self.handle_describe_job_definitions(&state, &body).await
            }
            ("POST", "/v1/deregisterjobdefinition") => {
                self.handle_deregister_job_definition(&state, &body).await
            }
            // Compute Environment operations
            ("POST", "/v1/createcomputeenvironment") => {
                self.handle_create_compute_environment(&state, &body, account_id, &region)
                    .await
            }
            ("POST", "/v1/describecomputeenvironments") => {
                self.handle_describe_compute_environments(&state, &body)
                    .await
            }
            ("POST", "/v1/deletecomputeenvironment") => {
                self.handle_delete_compute_environment(&state, &body).await
            }
            ("POST", "/v1/updatecomputeenvironment") => {
                self.handle_update_compute_environment(&state, &body).await
            }
            // Scheduling Policy operations
            ("POST", "/v1/createschedulingpolicy") => {
                self.handle_create_scheduling_policy(&state, &body, account_id, &region)
                    .await
            }
            ("POST", "/v1/describeschedulingpolicies") => {
                self.handle_describe_scheduling_policies(&state, &body)
                    .await
            }
            ("POST", "/v1/listschedulingpolicies") => {
                self.handle_list_scheduling_policies(&state).await
            }
            ("POST", "/v1/deleteschedulingpolicy") => {
                self.handle_delete_scheduling_policy(&state, &body).await
            }
            ("POST", "/v1/updateschedulingpolicy") => {
                self.handle_update_scheduling_policy(&state, &body).await
            }
            // Job operations
            ("POST", "/v1/submitjob") => {
                self.handle_submit_job(&state, &body, account_id, &region)
                    .await
            }
            ("POST", "/v1/describejobs") => self.handle_describe_jobs(&state, &body).await,
            ("POST", "/v1/listjobs") => self.handle_list_jobs(&state, &body).await,
            ("POST", "/v1/canceljob") => self.handle_cancel_job(&state, &body).await,
            ("POST", "/v1/terminatejob") => self.handle_terminate_job(&state, &body).await,
            // ConsumableResource operations
            ("POST", "/v1/createconsumableresource") => {
                self.handle_create_consumable_resource(&state, &body, account_id, &region)
                    .await
            }
            ("POST", "/v1/deleteconsumableresource") => {
                self.handle_delete_consumable_resource(&state, &body).await
            }
            ("POST", "/v1/describeconsumableresource") => {
                self.handle_describe_consumable_resource(&state, &body)
                    .await
            }
            ("POST", "/v1/updateconsumableresource") => {
                self.handle_update_consumable_resource(&state, &body).await
            }
            ("POST", "/v1/listconsumableresources") => {
                self.handle_list_consumable_resources(&state, &body).await
            }
            ("POST", "/v1/listjobsbyconsumableresource") => {
                self.handle_list_jobs_by_consumable_resource(&state, &body)
                    .await
            }
            // ServiceEnvironment operations
            ("POST", "/v1/createserviceenvironment") => {
                self.handle_create_service_environment(&state, &body, account_id, &region)
                    .await
            }
            ("POST", "/v1/deleteserviceenvironment") => {
                self.handle_delete_service_environment(&state, &body).await
            }
            ("POST", "/v1/describeserviceenvironments") => {
                self.handle_describe_service_environments(&state, &body)
                    .await
            }
            ("POST", "/v1/updateserviceenvironment") => {
                self.handle_update_service_environment(&state, &body).await
            }
            // ServiceJob operations
            ("POST", "/v1/submitservicejob") => {
                self.handle_submit_service_job(&state, &body, account_id, &region)
                    .await
            }
            ("POST", "/v1/describeservicejob") => {
                self.handle_describe_service_job(&state, &body).await
            }
            ("POST", "/v1/terminateservicejob") => {
                self.handle_terminate_service_job(&state, &body).await
            }
            ("POST", "/v1/listservicejobs") => self.handle_list_service_jobs(&state, &body).await,
            // GetJobQueueSnapshot
            ("POST", "/v1/getjobqueuesnapshot") => {
                self.handle_get_job_queue_snapshot(&state, &body).await
            }
            // Tag operations
            ("POST", p) if p.starts_with("/v1/tags/") => {
                let resource_arn = decode_resource_arn(p);
                self.handle_tag_resource(&state, &body, &resource_arn).await
            }
            ("GET", p) if p.starts_with("/v1/tags/") => {
                let resource_arn = decode_resource_arn(p);
                self.handle_list_tags_for_resource(&state, &resource_arn)
                    .await
            }
            ("DELETE", p) if p.starts_with("/v1/tags/") => {
                let resource_arn = decode_resource_arn(p);
                let tag_keys = extract_query_params(&request.uri, "tagKeys");
                self.handle_untag_resource(&state, &resource_arn, &tag_keys)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };
        if response.status >= 200 && response.status < 300 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_job_queue(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match body.get("jobQueueName").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => return rest_json_error(400, "ClientException", "jobQueueName is required"),
        };

        let queue_state = body
            .get("state")
            .and_then(|v| v.as_str())
            .unwrap_or("ENABLED");

        let priority = match body.get("priority").and_then(|v| v.as_i64()) {
            Some(p) => p as i32,
            None => return rest_json_error(400, "ClientException", "priority is required"),
        };

        let compute_env_order = parse_compute_environment_order(body);

        let tags: HashMap<String, String> = body
            .get("tags")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let scheduling_policy_arn = body
            .get("schedulingPolicyArn")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let mut state = state.write().await;
        match state.create_job_queue(
            name,
            queue_state,
            priority,
            compute_env_order,
            tags,
            account_id,
            region,
            scheduling_policy_arn,
        ) {
            Ok(jq) => wire::serialize_create_job_queue_response(&wire::CreateJobQueueResponse {
                job_queue_name: Some(jq.job_queue_name.clone()),
                job_queue_arn: Some(jq.job_queue_arn.clone()),
            }),
            Err(e) => batch_error_response(&e),
        }
    }

    async fn handle_describe_job_queues(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let names: Vec<&str> = body
            .get("jobQueues")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_default();

        let state = state.read().await;
        let queues = state.describe_job_queues(&names);

        let entries: Vec<wire::JobQueueDetail> = queues
            .iter()
            .map(|jq| wire::JobQueueDetail {
                job_queue_name: Some(jq.job_queue_name.clone()),
                job_queue_arn: Some(jq.job_queue_arn.clone()),
                state: Some(jq.state.clone()),
                status: Some(jq.status.clone()),
                status_reason: Some(jq.status_reason.clone()),
                priority: Some(jq.priority),
                compute_environment_order: Some(
                    jq.compute_environment_order
                        .iter()
                        .map(|ce| wire::ComputeEnvironmentOrder {
                            order: ce.order,
                            compute_environment: ce.compute_environment.clone(),
                        })
                        .collect(),
                ),
                tags: if jq.tags.is_empty() {
                    None
                } else {
                    Some(jq.tags.clone())
                },
                scheduling_policy_arn: jq.scheduling_policy_arn.clone(),
                ..Default::default()
            })
            .collect();

        wire::serialize_describe_job_queues_response(&wire::DescribeJobQueuesResponse {
            job_queues: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_delete_job_queue(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let queue = match body.get("jobQueue").and_then(|v| v.as_str()) {
            Some(q) => q,
            None => return rest_json_error(400, "ClientException", "jobQueue is required"),
        };

        let mut state = state.write().await;
        match state.delete_job_queue(queue) {
            Ok(()) => wire::serialize_delete_job_queue_response(&wire::DeleteJobQueueResponse {}),
            Err(e) => batch_error_response(&e),
        }
    }

    async fn handle_update_job_queue(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let queue = match body.get("jobQueue").and_then(|v| v.as_str()) {
            Some(q) => q,
            None => return rest_json_error(400, "ClientException", "jobQueue is required"),
        };

        let new_state = body.get("state").and_then(|v| v.as_str());
        let new_priority = body
            .get("priority")
            .and_then(|v| v.as_i64())
            .map(|p| p as i32);
        let new_ceo = if body.get("computeEnvironmentOrder").is_some() {
            Some(parse_compute_environment_order(body))
        } else {
            None
        };

        let mut state = state.write().await;
        match state.update_job_queue(queue, new_state, new_priority, new_ceo) {
            Ok(jq) => wire::serialize_update_job_queue_response(&wire::UpdateJobQueueResponse {
                job_queue_name: Some(jq.job_queue_name.clone()),
                job_queue_arn: Some(jq.job_queue_arn.clone()),
            }),
            Err(e) => batch_error_response(&e),
        }
    }

    async fn handle_register_job_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match body.get("jobDefinitionName").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return rest_json_error(400, "ClientException", "jobDefinitionName is required");
            }
        };

        let job_type = match body.get("type").and_then(|v| v.as_str()) {
            Some(t) => t,
            None => return rest_json_error(400, "ClientException", "type is required"),
        };

        let container_props = body.get("containerProperties").map(|cp| {
            let image = cp
                .get("image")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let command: Vec<String> = cp
                .get("command")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();
            let resource_requirements: Vec<ResourceRequirement> = cp
                .get("resourceRequirements")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .map(|rr| ResourceRequirement {
                            resource_type: rr
                                .get("type")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                            value: rr
                                .get("value")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                        })
                        .collect()
                })
                .unwrap_or_default();

            ContainerProperties {
                image,
                command,
                resource_requirements,
            }
        });

        let mut state = state.write().await;
        match state.register_job_definition(name, job_type, container_props, account_id, region) {
            Ok(jd) => wire::serialize_register_job_definition_response(
                &wire::RegisterJobDefinitionResponse {
                    job_definition_name: Some(jd.job_definition_name.clone()),
                    job_definition_arn: Some(jd.job_definition_arn.clone()),
                    revision: Some(jd.revision),
                },
            ),
            Err(e) => batch_error_response(&e),
        }
    }

    async fn handle_describe_job_definitions(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let names: Vec<&str> = body
            .get("jobDefinitions")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_default();

        let job_definition_name = body.get("jobDefinitionName").and_then(|v| v.as_str());

        let status = body.get("status").and_then(|v| v.as_str());

        let state = state.read().await;
        let defs = state.describe_job_definitions(&names, job_definition_name, status);

        let entries: Vec<wire::JobDefinition> = defs
            .iter()
            .map(|jd| {
                let container_properties =
                    jd.container_properties
                        .as_ref()
                        .map(|cp| wire::ContainerProperties {
                            image: Some(cp.image.clone()),
                            command: Some(cp.command.clone()),
                            resource_requirements: Some(
                                cp.resource_requirements
                                    .iter()
                                    .map(|rr| wire::ResourceRequirement {
                                        r#type: rr.resource_type.clone(),
                                        value: rr.value.clone(),
                                    })
                                    .collect(),
                            ),
                            ..Default::default()
                        });
                wire::JobDefinition {
                    job_definition_name: Some(jd.job_definition_name.clone()),
                    job_definition_arn: Some(jd.job_definition_arn.clone()),
                    revision: Some(jd.revision),
                    status: Some(jd.status.clone()),
                    r#type: Some(jd.job_definition_type.clone()),
                    container_properties,
                    tags: if jd.tags.is_empty() {
                        None
                    } else {
                        Some(jd.tags.clone())
                    },
                    ..Default::default()
                }
            })
            .collect();

        wire::serialize_describe_job_definitions_response(&wire::DescribeJobDefinitionsResponse {
            job_definitions: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_deregister_job_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let job_def = match body.get("jobDefinition").and_then(|v| v.as_str()) {
            Some(jd) => jd,
            None => return rest_json_error(400, "ClientException", "jobDefinition is required"),
        };

        let mut state = state.write().await;
        match state.deregister_job_definition(job_def) {
            Ok(()) => wire::serialize_deregister_job_definition_response(
                &wire::DeregisterJobDefinitionResponse {},
            ),
            Err(e) => batch_error_response(&e),
        }
    }

    // --- Compute Environment handlers ---

    async fn handle_create_compute_environment(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match body.get("computeEnvironmentName").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return rest_json_error(
                    400,
                    "ClientException",
                    "computeEnvironmentName is required",
                );
            }
        };

        let ce_type = match body.get("type").and_then(|v| v.as_str()) {
            Some(t) => t,
            None => return rest_json_error(400, "ClientException", "type is required"),
        };

        let ce_state = body.get("state").and_then(|v| v.as_str());
        let service_role = body.get("serviceRole").and_then(|v| v.as_str());
        let tags = parse_tags(body);

        let mut state = state.write().await;
        match state.create_compute_environment(
            name,
            ce_type,
            ce_state,
            service_role,
            tags,
            account_id,
            region,
        ) {
            Ok(ce) => wire::serialize_create_compute_environment_response(
                &wire::CreateComputeEnvironmentResponse {
                    compute_environment_name: Some(ce.compute_environment_name.clone()),
                    compute_environment_arn: Some(ce.compute_environment_arn.clone()),
                },
            ),
            Err(e) => batch_error_response(&e),
        }
    }

    async fn handle_describe_compute_environments(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let names: Vec<&str> = body
            .get("computeEnvironments")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_default();

        let state = state.read().await;
        let envs = state.describe_compute_environments(&names);

        let entries: Vec<wire::ComputeEnvironmentDetail> = envs
            .iter()
            .map(|ce| wire::ComputeEnvironmentDetail {
                compute_environment_name: Some(ce.compute_environment_name.clone()),
                compute_environment_arn: Some(ce.compute_environment_arn.clone()),
                r#type: Some(ce.ce_type.clone()),
                state: Some(ce.state.clone()),
                status: Some(ce.status.clone()),
                status_reason: Some(ce.status_reason.clone()),
                service_role: ce.service_role.clone(),
                tags: if ce.tags.is_empty() {
                    None
                } else {
                    Some(ce.tags.clone())
                },
                ..Default::default()
            })
            .collect();

        wire::serialize_describe_compute_environments_response(
            &wire::DescribeComputeEnvironmentsResponse {
                compute_environments: Some(entries),
                ..Default::default()
            },
        )
    }

    async fn handle_delete_compute_environment(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let ce = match body.get("computeEnvironment").and_then(|v| v.as_str()) {
            Some(c) => c,
            None => {
                return rest_json_error(400, "ClientException", "computeEnvironment is required");
            }
        };

        let mut state = state.write().await;
        match state.delete_compute_environment(ce) {
            Ok(()) => wire::serialize_delete_compute_environment_response(
                &wire::DeleteComputeEnvironmentResponse {},
            ),
            Err(e) => batch_error_response(&e),
        }
    }

    async fn handle_update_compute_environment(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let ce = match body.get("computeEnvironment").and_then(|v| v.as_str()) {
            Some(c) => c,
            None => {
                return rest_json_error(400, "ClientException", "computeEnvironment is required");
            }
        };

        let new_state = body.get("state").and_then(|v| v.as_str());
        let service_role = body.get("serviceRole").and_then(|v| v.as_str());

        let mut state = state.write().await;
        match state.update_compute_environment(ce, new_state, service_role) {
            Ok(ce) => wire::serialize_update_compute_environment_response(
                &wire::UpdateComputeEnvironmentResponse {
                    compute_environment_name: Some(ce.compute_environment_name.clone()),
                    compute_environment_arn: Some(ce.compute_environment_arn.clone()),
                },
            ),
            Err(e) => batch_error_response(&e),
        }
    }

    // --- Scheduling Policy handlers ---

    async fn handle_create_scheduling_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match body.get("name").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => return rest_json_error(400, "ClientException", "name is required"),
        };

        let fairshare_policy = parse_fairshare_policy(body);
        let tags = parse_tags(body);

        let mut state = state.write().await;
        match state.create_scheduling_policy(name, fairshare_policy, tags, account_id, region) {
            Ok(sp) => wire::serialize_create_scheduling_policy_response(
                &wire::CreateSchedulingPolicyResponse {
                    name: Some(sp.name.clone()),
                    arn: Some(sp.arn.clone()),
                },
            ),
            Err(e) => batch_error_response(&e),
        }
    }

    async fn handle_describe_scheduling_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let arns: Vec<&str> = body
            .get("arns")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_default();

        let state = state.read().await;
        let policies = state.describe_scheduling_policies(&arns);

        let entries: Vec<wire::SchedulingPolicyDetail> = policies
            .iter()
            .map(|sp| wire::SchedulingPolicyDetail {
                name: Some(sp.name.clone()),
                arn: Some(sp.arn.clone()),
                fairshare_policy: sp
                    .fairshare_policy
                    .as_ref()
                    .map(|fp| wire::FairsharePolicy {
                        compute_reservation: fp.compute_reservation,
                        share_decay_seconds: fp.share_decay_seconds,
                        share_distribution: Some(
                            fp.share_distribution
                                .iter()
                                .map(|sa| wire::ShareAttributes {
                                    share_identifier: sa.share_identifier.clone(),
                                    weight_factor: sa.weight_factor,
                                })
                                .collect(),
                        ),
                    }),
                tags: if sp.tags.is_empty() {
                    None
                } else {
                    Some(sp.tags.clone())
                },
            })
            .collect();

        wire::serialize_describe_scheduling_policies_response(
            &wire::DescribeSchedulingPoliciesResponse {
                scheduling_policies: Some(entries),
            },
        )
    }

    async fn handle_list_scheduling_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let policies = state.list_scheduling_policies();

        let entries: Vec<wire::SchedulingPolicyListingDetail> = policies
            .iter()
            .map(|sp| wire::SchedulingPolicyListingDetail {
                arn: Some(sp.arn.clone()),
            })
            .collect();

        wire::serialize_list_scheduling_policies_response(&wire::ListSchedulingPoliciesResponse {
            scheduling_policies: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_delete_scheduling_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let arn = match body.get("arn").and_then(|v| v.as_str()) {
            Some(a) => a,
            None => return rest_json_error(400, "ClientException", "arn is required"),
        };

        let mut state = state.write().await;
        match state.delete_scheduling_policy(arn) {
            Ok(()) => wire::serialize_delete_scheduling_policy_response(
                &wire::DeleteSchedulingPolicyResponse {},
            ),
            Err(e) => batch_error_response(&e),
        }
    }

    async fn handle_update_scheduling_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let arn = match body.get("arn").and_then(|v| v.as_str()) {
            Some(a) => a,
            None => return rest_json_error(400, "ClientException", "arn is required"),
        };

        let fairshare_policy = parse_fairshare_policy(body);

        let mut state = state.write().await;
        match state.update_scheduling_policy(arn, fairshare_policy) {
            Ok(()) => wire::serialize_update_scheduling_policy_response(
                &wire::UpdateSchedulingPolicyResponse {},
            ),
            Err(e) => batch_error_response(&e),
        }
    }

    // --- Job handlers ---

    async fn handle_submit_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let job_name = match body.get("jobName").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => return rest_json_error(400, "ClientException", "jobName is required"),
        };
        let job_queue = match body.get("jobQueue").and_then(|v| v.as_str()) {
            Some(q) => q,
            None => return rest_json_error(400, "ClientException", "jobQueue is required"),
        };
        let job_definition = match body.get("jobDefinition").and_then(|v| v.as_str()) {
            Some(d) => d,
            None => return rest_json_error(400, "ClientException", "jobDefinition is required"),
        };
        let tags = parse_tags(body);

        let mut state = state.write().await;
        match state.submit_job(
            job_name,
            job_queue,
            job_definition,
            tags,
            account_id,
            region,
        ) {
            Ok(job) => wire::serialize_submit_job_response(&wire::SubmitJobResponse {
                job_id: Some(job.job_id.clone()),
                job_name: Some(job.job_name.clone()),
                job_arn: Some(job.job_arn.clone()),
            }),
            Err(e) => batch_error_response(&e),
        }
    }

    async fn handle_describe_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let job_ids: Vec<&str> = body
            .get("jobs")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_default();

        let state = state.read().await;
        let jobs = state.describe_jobs(&job_ids);

        let entries: Vec<wire::JobDetail> = jobs
            .iter()
            .map(|j| wire::JobDetail {
                job_id: Some(j.job_id.clone()),
                job_name: Some(j.job_name.clone()),
                job_arn: Some(j.job_arn.clone()),
                job_queue: Some(j.job_queue.clone()),
                job_definition: Some(j.job_definition.clone()),
                status: Some(j.status.clone()),
                status_reason: j.status_reason.clone(),
                created_at: Some(j.created_at),
                started_at: Some(j.started_at),
                tags: if j.tags.is_empty() {
                    None
                } else {
                    Some(j.tags.clone())
                },
                ..Default::default()
            })
            .collect();

        wire::serialize_describe_jobs_response(&wire::DescribeJobsResponse {
            jobs: Some(entries),
        })
    }

    async fn handle_list_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let job_queue = body.get("jobQueue").and_then(|v| v.as_str());
        let job_status = body.get("jobStatus").and_then(|v| v.as_str());

        let state = state.read().await;
        let jobs = state.list_jobs(job_queue, job_status);

        let entries: Vec<wire::JobSummary> = jobs
            .iter()
            .map(|j| wire::JobSummary {
                job_id: Some(j.job_id.clone()),
                job_name: Some(j.job_name.clone()),
                job_arn: Some(j.job_arn.clone()),
                job_definition: Some(j.job_definition.clone()),
                status: Some(j.status.clone()),
                status_reason: j.status_reason.clone(),
                created_at: Some(j.created_at),
                started_at: Some(j.started_at),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_jobs_response(&wire::ListJobsResponse {
            job_summary_list: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_cancel_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let job_id = match body.get("jobId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return rest_json_error(400, "ClientException", "jobId is required"),
        };
        let reason = match body.get("reason").and_then(|v| v.as_str()) {
            Some(r) => r,
            None => return rest_json_error(400, "ClientException", "reason is required"),
        };

        let mut state = state.write().await;
        match state.cancel_job(job_id, reason) {
            Ok(()) => wire::serialize_cancel_job_response(&wire::CancelJobResponse {}),
            Err(e) => batch_error_response(&e),
        }
    }

    async fn handle_terminate_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let job_id = match body.get("jobId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return rest_json_error(400, "ClientException", "jobId is required"),
        };
        let reason = match body.get("reason").and_then(|v| v.as_str()) {
            Some(r) => r,
            None => return rest_json_error(400, "ClientException", "reason is required"),
        };

        let mut state = state.write().await;
        match state.terminate_job(job_id, reason) {
            Ok(()) => wire::serialize_terminate_job_response(&wire::TerminateJobResponse {}),
            Err(e) => batch_error_response(&e),
        }
    }

    // --- Tag handlers ---

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
        resource_arn: &str,
    ) -> MockResponse {
        let tags = parse_tags(body);
        let mut state = state.write().await;
        match state.tag_resource(resource_arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => batch_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        resource_arn: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_tags_for_resource(resource_arn) {
            Ok(tags) => wire::serialize_list_tags_for_resource_response(
                &wire::ListTagsForResourceResponse {
                    tags: if tags.is_empty() { None } else { Some(tags) },
                },
            ),
            Err(e) => batch_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> MockResponse {
        let keys: Vec<&str> = tag_keys.iter().map(|s| s.as_str()).collect();
        let mut state = state.write().await;
        match state.untag_resource(resource_arn, &keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => batch_error_response(&e),
        }
    }

    // --- ConsumableResource handlers ---

    async fn handle_create_consumable_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match body.get("consumableResourceName").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return rest_json_error(
                    400,
                    "ClientException",
                    "consumableResourceName is required",
                );
            }
        };
        let total_quantity = body
            .get("totalQuantity")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        let resource_type = body
            .get("resourceType")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let tags = parse_tags(body);

        let mut state = state.write().await;
        match state.create_consumable_resource(
            name,
            total_quantity,
            resource_type,
            tags,
            account_id,
            region,
        ) {
            Ok(cr) => wire::serialize_create_consumable_resource_response(
                &wire::CreateConsumableResourceResponse {
                    consumable_resource_name: Some(cr.consumable_resource_name.clone()),
                    consumable_resource_arn: Some(cr.consumable_resource_arn.clone()),
                },
            ),
            Err(e) => batch_error_response(&e),
        }
    }

    async fn handle_delete_consumable_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let name_or_arn = match body.get("consumableResource").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return rest_json_error(400, "ClientException", "consumableResource is required");
            }
        };
        let mut state = state.write().await;
        match state.delete_consumable_resource(name_or_arn) {
            Ok(()) => wire::serialize_delete_consumable_resource_response(
                &wire::DeleteConsumableResourceResponse {},
            ),
            Err(e) => batch_error_response(&e),
        }
    }

    async fn handle_describe_consumable_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let name_or_arn = match body.get("consumableResource").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return rest_json_error(400, "ClientException", "consumableResource is required");
            }
        };
        let state = state.read().await;
        match state.describe_consumable_resource(name_or_arn) {
            Ok(cr) => {
                let available = cr.total_quantity - cr.in_use_quantity;
                wire::serialize_describe_consumable_resource_response(
                    &wire::DescribeConsumableResourceResponse {
                        consumable_resource_name: Some(cr.consumable_resource_name.clone()),
                        consumable_resource_arn: Some(cr.consumable_resource_arn.clone()),
                        total_quantity: Some(cr.total_quantity),
                        in_use_quantity: Some(cr.in_use_quantity),
                        available_quantity: Some(available),
                        resource_type: cr.resource_type.clone(),
                        created_at: Some(cr.created_at),
                        tags: if cr.tags.is_empty() {
                            None
                        } else {
                            Some(cr.tags.clone())
                        },
                    },
                )
            }
            Err(e) => batch_error_response(&e),
        }
    }

    async fn handle_update_consumable_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let name_or_arn = match body.get("consumableResource").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return rest_json_error(400, "ClientException", "consumableResource is required");
            }
        };
        let quantity = body.get("quantity").and_then(|v| v.as_i64());

        let mut state = state.write().await;
        match state.update_consumable_resource(name_or_arn, quantity) {
            Ok(cr) => wire::serialize_update_consumable_resource_response(
                &wire::UpdateConsumableResourceResponse {
                    consumable_resource_name: Some(cr.consumable_resource_name.clone()),
                    consumable_resource_arn: Some(cr.consumable_resource_arn.clone()),
                    total_quantity: Some(cr.total_quantity),
                },
            ),
            Err(e) => batch_error_response(&e),
        }
    }

    async fn handle_list_consumable_resources(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let name_filter = body.get("consumableResourceName").and_then(|v| v.as_str());

        let state = state.read().await;
        let resources = state.list_consumable_resources(name_filter);

        let entries: Vec<wire::ConsumableResourceSummary> = resources
            .iter()
            .map(|cr| wire::ConsumableResourceSummary {
                consumable_resource_name: Some(cr.consumable_resource_name.clone()),
                consumable_resource_arn: Some(cr.consumable_resource_arn.clone()),
                total_quantity: Some(cr.total_quantity),
                in_use_quantity: Some(cr.in_use_quantity),
                resource_type: cr.resource_type.clone(),
            })
            .collect();

        wire::serialize_list_consumable_resources_response(&wire::ListConsumableResourcesResponse {
            consumable_resources: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_list_jobs_by_consumable_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let consumable_resource = match body.get("consumableResource").and_then(|v| v.as_str()) {
            Some(cr) => cr,
            None => {
                return rest_json_error(400, "ClientException", "consumableResource is required");
            }
        };
        let state = state.read().await;
        // Validate the consumable resource exists
        if !state.consumable_resources.contains_key(consumable_resource)
            && !state
                .consumable_resources
                .values()
                .any(|cr| cr.consumable_resource_arn == consumable_resource)
        {
            return rest_json_error(
                404,
                "ClientException",
                &format!("Consumable resource {consumable_resource} does not exist"),
            );
        }
        // Return jobs that reference this consumable resource (none in mock)
        wire::serialize_list_jobs_by_consumable_resource_response(
            &wire::ListJobsByConsumableResourceResponse {
                jobs: Some(vec![]),
                ..Default::default()
            },
        )
    }

    // --- ServiceEnvironment handlers ---

    async fn handle_create_service_environment(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match body.get("serviceEnvironmentName").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return rest_json_error(
                    400,
                    "ClientException",
                    "serviceEnvironmentName is required",
                );
            }
        };
        let se_type = body
            .get("serviceEnvironmentType")
            .and_then(|v| v.as_str())
            .unwrap_or("MANAGED");
        let state_val = body.get("state").and_then(|v| v.as_str());
        let tags = parse_tags(body);

        let mut state = state.write().await;
        match state.create_service_environment(name, se_type, state_val, tags, account_id, region) {
            Ok(se) => wire::serialize_create_service_environment_response(
                &wire::CreateServiceEnvironmentResponse {
                    service_environment_name: Some(se.service_environment_name.clone()),
                    service_environment_arn: Some(se.service_environment_arn.clone()),
                },
            ),
            Err(e) => batch_error_response(&e),
        }
    }

    async fn handle_delete_service_environment(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let name_or_arn = match body.get("serviceEnvironment").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return rest_json_error(400, "ClientException", "serviceEnvironment is required");
            }
        };
        let mut state = state.write().await;
        match state.delete_service_environment(name_or_arn) {
            Ok(()) => wire::serialize_delete_service_environment_response(
                &wire::DeleteServiceEnvironmentResponse {},
            ),
            Err(e) => batch_error_response(&e),
        }
    }

    async fn handle_describe_service_environments(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let names: Vec<&str> = body
            .get("serviceEnvironments")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_default();

        let state = state.read().await;
        let envs = state.describe_service_environments(&names);

        let entries: Vec<wire::ServiceEnvironmentDetail> = envs
            .iter()
            .map(|se| wire::ServiceEnvironmentDetail {
                service_environment_name: Some(se.service_environment_name.clone()),
                service_environment_arn: Some(se.service_environment_arn.clone()),
                service_environment_type: Some(se.service_environment_type.clone()),
                state: Some(se.state.clone()),
                status: Some(se.status.clone()),
                tags: if se.tags.is_empty() {
                    None
                } else {
                    Some(se.tags.clone())
                },
                ..Default::default()
            })
            .collect();

        wire::serialize_describe_service_environments_response(
            &wire::DescribeServiceEnvironmentsResponse {
                service_environments: Some(entries),
                ..Default::default()
            },
        )
    }

    async fn handle_update_service_environment(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let name_or_arn = match body.get("serviceEnvironment").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return rest_json_error(400, "ClientException", "serviceEnvironment is required");
            }
        };
        let new_state = body.get("state").and_then(|v| v.as_str());

        let mut state = state.write().await;
        match state.update_service_environment(name_or_arn, new_state) {
            Ok(se) => wire::serialize_update_service_environment_response(
                &wire::UpdateServiceEnvironmentResponse {
                    service_environment_name: Some(se.service_environment_name.clone()),
                    service_environment_arn: Some(se.service_environment_arn.clone()),
                },
            ),
            Err(e) => batch_error_response(&e),
        }
    }

    // --- ServiceJob handlers ---

    async fn handle_submit_service_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let job_name = match body.get("jobName").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => return rest_json_error(400, "ClientException", "jobName is required"),
        };
        let job_queue = match body.get("jobQueue").and_then(|v| v.as_str()) {
            Some(q) => q,
            None => return rest_json_error(400, "ClientException", "jobQueue is required"),
        };
        let tags = parse_tags(body);

        let mut state = state.write().await;
        match state.submit_service_job(job_name, job_queue, tags, account_id, region) {
            Ok(sj) => {
                wire::serialize_submit_service_job_response(&wire::SubmitServiceJobResponse {
                    job_id: Some(sj.job_id.clone()),
                    job_arn: Some(sj.job_arn.clone()),
                    job_name: Some(sj.job_name.clone()),
                })
            }
            Err(e) => batch_error_response(&e),
        }
    }

    async fn handle_describe_service_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let job_id = match body.get("jobId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return rest_json_error(400, "ClientException", "jobId is required"),
        };
        let state = state.read().await;
        match state.describe_service_job(job_id) {
            Ok(sj) => {
                wire::serialize_describe_service_job_response(&wire::DescribeServiceJobResponse {
                    job_id: Some(sj.job_id.clone()),
                    job_arn: Some(sj.job_arn.clone()),
                    job_name: Some(sj.job_name.clone()),
                    job_queue: Some(sj.job_queue.clone()),
                    status: Some(sj.status.clone()),
                    created_at: Some(sj.created_at),
                    started_at: sj.started_at,
                    stopped_at: sj.stopped_at,
                    is_terminated: Some(sj.is_terminated),
                    ..Default::default()
                })
            }
            Err(e) => batch_error_response(&e),
        }
    }

    async fn handle_terminate_service_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let job_id = match body.get("jobId").and_then(|v| v.as_str()) {
            Some(id) => id,
            None => return rest_json_error(400, "ClientException", "jobId is required"),
        };
        let mut state = state.write().await;
        match state.terminate_service_job(job_id) {
            Ok(()) => wire::serialize_terminate_service_job_response(
                &wire::TerminateServiceJobResponse {},
            ),
            Err(e) => batch_error_response(&e),
        }
    }

    async fn handle_list_service_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let job_queue = body.get("jobQueue").and_then(|v| v.as_str());

        let state = state.read().await;
        let jobs = state.list_service_jobs(job_queue);

        let entries: Vec<wire::ServiceJobSummary> = jobs
            .iter()
            .map(|sj| wire::ServiceJobSummary {
                job_id: Some(sj.job_id.clone()),
                job_arn: Some(sj.job_arn.clone()),
                job_name: Some(sj.job_name.clone()),
                status: Some(sj.status.clone()),
                created_at: Some(sj.created_at),
                started_at: sj.started_at,
                stopped_at: sj.stopped_at,
                ..Default::default()
            })
            .collect();

        wire::serialize_list_service_jobs_response(&wire::ListServiceJobsResponse {
            job_summary_list: Some(entries),
            ..Default::default()
        })
    }

    // --- GetJobQueueSnapshot handler ---

    async fn handle_get_job_queue_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<BatchState>>,
        body: &Value,
    ) -> MockResponse {
        let job_queue = match body.get("jobQueue").and_then(|v| v.as_str()) {
            Some(jq) => jq,
            None => {
                return rest_json_error(400, "ClientException", "jobQueue is required");
            }
        };
        let state = state.read().await;
        // Validate the job queue exists
        if !state.job_queues.contains_key(job_queue)
            && !state
                .job_queues
                .values()
                .any(|jq| jq.job_queue_arn == job_queue)
        {
            return rest_json_error(
                404,
                "ClientException",
                &format!("Job queue {job_queue} does not exist"),
            );
        }
        let now = chrono::Utc::now().timestamp();
        wire::serialize_get_job_queue_snapshot_response(&wire::GetJobQueueSnapshotResponse {
            front_of_queue: Some(wire::FrontOfQueueDetail {
                jobs: Some(vec![]),
                last_updated_at: Some(now),
            }),
            ..Default::default()
        })
    }
}

fn parse_compute_environment_order(body: &Value) -> Vec<ComputeEnvironmentOrder> {
    body.get("computeEnvironmentOrder")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .map(|ce| ComputeEnvironmentOrder {
                    order: ce.get("order").and_then(|v| v.as_i64()).unwrap_or(1) as i32,
                    compute_environment: ce
                        .get("computeEnvironment")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                })
                .collect()
        })
        .unwrap_or_default()
}

fn parse_tags(body: &Value) -> HashMap<String, String> {
    body.get("tags")
        .and_then(|v| v.as_object())
        .map(|obj| {
            obj.iter()
                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                .collect()
        })
        .unwrap_or_default()
}

fn parse_fairshare_policy(body: &Value) -> Option<FairsharePolicy> {
    body.get("fairsharePolicy").map(|fp| {
        let compute_reservation = fp
            .get("computeReservation")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);
        let share_decay_seconds = fp
            .get("shareDecaySeconds")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);
        let share_distribution = fp
            .get("shareDistribution")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|sa| ShareAttributes {
                        share_identifier: sa
                            .get("shareIdentifier")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        weight_factor: sa
                            .get("weightFactor")
                            .and_then(|v| v.as_f64())
                            .map(|f| f as f32),
                    })
                    .collect()
            })
            .unwrap_or_default();

        FairsharePolicy {
            compute_reservation,
            share_decay_seconds,
            share_distribution,
        }
    })
}

fn decode_resource_arn(path: &str) -> String {
    // path is like /v1/tags/arn%3Aaws%3Abatch%3A...
    let prefix = "/v1/tags/";
    let encoded = &path[prefix.len()..];
    urlencoding::decode(encoded)
        .unwrap_or_else(|_| encoded.into())
        .to_string()
}

fn extract_query_params(uri: &str, key: &str) -> Vec<String> {
    let query_start = match uri.find('?') {
        Some(idx) => idx + 1,
        None => return vec![],
    };
    let query = &uri[query_start..];
    query
        .split('&')
        .filter_map(|pair| {
            let (k, v) = pair.split_once('=')?;
            if k == key || k == format!("{key}[]").as_str() {
                Some(
                    urlencoding::decode(v)
                        .unwrap_or_else(|_| v.into())
                        .to_string(),
                )
            } else {
                None
            }
        })
        .collect()
}

fn extract_path(uri: &str) -> String {
    let after_scheme = if let Some(idx) = uri.find("://") {
        &uri[idx + 3..]
    } else {
        uri
    };
    let path_start = after_scheme.find('/').unwrap_or(after_scheme.len());
    let path_and_query = &after_scheme[path_start..];
    match path_and_query.find('?') {
        Some(q) => path_and_query[..q].to_string(),
        None => path_and_query.to_string(),
    }
}

fn batch_error_response(err: &BatchError) -> MockResponse {
    let (status, error_type) = match err {
        BatchError::JobQueueAlreadyExists(_) => (400u16, "ClientException"),
        BatchError::JobQueueNotFound(_) => (404, "ClientException"),
        BatchError::JobDefinitionNotFound(_) => (404, "ClientException"),
        BatchError::ComputeEnvironmentAlreadyExists(_) => (400, "ClientException"),
        BatchError::ComputeEnvironmentNotFound(_) => (404, "ClientException"),
        BatchError::SchedulingPolicyAlreadyExists(_) => (400, "ClientException"),
        BatchError::SchedulingPolicyNotFound(_) => (404, "ClientException"),
        BatchError::JobNotFound(_) => (404, "ClientException"),
        BatchError::ResourceNotFound(_) => (404, "ClientException"),
        BatchError::ConsumableResourceAlreadyExists(_) => (400, "ClientException"),
        BatchError::ConsumableResourceNotFound(_) => (404, "ClientException"),
        BatchError::ServiceEnvironmentAlreadyExists(_) => (400, "ClientException"),
        BatchError::ServiceEnvironmentNotFound(_) => (404, "ClientException"),
        BatchError::ServiceJobNotFound(_) => (404, "ClientException"),
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
