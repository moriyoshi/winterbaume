use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::Value;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    json_error_response,
};

use crate::model::{BatchGetCollectionResponse, UpdateSecurityPolicyRequest};
use crate::model::{
    CollectionDetail, CollectionSummary, CreateCollectionDetail, CreateCollectionResponse,
    CreateSecurityPolicyResponse, CreateVpcEndpointDetail, CreateVpcEndpointResponse,
    DeleteCollectionDetail, DeleteCollectionResponse, GetSecurityPolicyResponse,
    ListCollectionsResponse, ListSecurityPoliciesResponse, ListTagsForResourceResponse,
    SecurityPolicyDetail, SecurityPolicySummary, Tag, TagResourceResponse, UntagResourceResponse,
    UpdateSecurityPolicyResponse,
};
use crate::state::{OpenSearchServerlessError, OpenSearchServerlessState};
use crate::types;
use crate::views::OpenSearchServerlessStateView;
use crate::wire;

pub struct OpenSearchServerlessService {
    pub(crate) state: Arc<BackendState<OpenSearchServerlessState>>,
    pub(crate) notifier: StateChangeNotifier<OpenSearchServerlessStateView>,
}

impl OpenSearchServerlessService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for OpenSearchServerlessService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for OpenSearchServerlessService {
    fn service_name(&self) -> &str {
        "aoss"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://aoss\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl OpenSearchServerlessService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

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

        // Validate that body parses as JSON; typed deserialisers re-parse per operation.
        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let mutating = matches!(
            action.as_str(),
            "CreateCollection"
                | "DeleteCollection"
                | "CreateSecurityPolicy"
                | "UpdateSecurityPolicy"
                | "CreateVpcEndpoint"
                | "TagResource"
                | "UntagResource"
        );

        let response = match action.as_str() {
            "CreateCollection" => {
                self.handle_create_collection(&state, body_bytes, &region, account_id)
                    .await
            }
            "DeleteCollection" => self.handle_delete_collection(&state, body_bytes).await,
            "ListCollections" => self.handle_list_collections(&state).await,
            "BatchGetCollection" => self.handle_batch_get_collection(&state, body_bytes).await,
            "CreateSecurityPolicy" => self.handle_create_security_policy(&state, body_bytes).await,
            "GetSecurityPolicy" => self.handle_get_security_policy(&state, body_bytes).await,
            "UpdateSecurityPolicy" => self.handle_update_security_policy(&state, body_bytes).await,
            "ListSecurityPolicies" => self.handle_list_security_policies(&state, body_bytes).await,
            "CreateVpcEndpoint" => self.handle_create_vpc_endpoint(&state, body_bytes).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            // --- Unimplemented operations ---
            "BatchGetCollectionGroup" => json_error_response(
                501,
                "NotImplementedError",
                "BatchGetCollectionGroup is not yet implemented in winterbaume-opensearchserverless",
            ),
            "BatchGetEffectiveLifecyclePolicy" => json_error_response(
                501,
                "NotImplementedError",
                "BatchGetEffectiveLifecyclePolicy is not yet implemented in winterbaume-opensearchserverless",
            ),
            "BatchGetLifecyclePolicy" => json_error_response(
                501,
                "NotImplementedError",
                "BatchGetLifecyclePolicy is not yet implemented in winterbaume-opensearchserverless",
            ),
            "BatchGetVpcEndpoint" => json_error_response(
                501,
                "NotImplementedError",
                "BatchGetVpcEndpoint is not yet implemented in winterbaume-opensearchserverless",
            ),
            "CreateAccessPolicy" => json_error_response(
                501,
                "NotImplementedError",
                "CreateAccessPolicy is not yet implemented in winterbaume-opensearchserverless",
            ),
            "CreateCollectionGroup" => json_error_response(
                501,
                "NotImplementedError",
                "CreateCollectionGroup is not yet implemented in winterbaume-opensearchserverless",
            ),
            "CreateIndex" => json_error_response(
                501,
                "NotImplementedError",
                "CreateIndex is not yet implemented in winterbaume-opensearchserverless",
            ),
            "CreateLifecyclePolicy" => json_error_response(
                501,
                "NotImplementedError",
                "CreateLifecyclePolicy is not yet implemented in winterbaume-opensearchserverless",
            ),
            "CreateSecurityConfig" => json_error_response(
                501,
                "NotImplementedError",
                "CreateSecurityConfig is not yet implemented in winterbaume-opensearchserverless",
            ),
            "DeleteAccessPolicy" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteAccessPolicy is not yet implemented in winterbaume-opensearchserverless",
            ),
            "DeleteCollectionGroup" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteCollectionGroup is not yet implemented in winterbaume-opensearchserverless",
            ),
            "DeleteIndex" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteIndex is not yet implemented in winterbaume-opensearchserverless",
            ),
            "DeleteLifecyclePolicy" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteLifecyclePolicy is not yet implemented in winterbaume-opensearchserverless",
            ),
            "DeleteSecurityConfig" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteSecurityConfig is not yet implemented in winterbaume-opensearchserverless",
            ),
            "DeleteSecurityPolicy" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteSecurityPolicy is not yet implemented in winterbaume-opensearchserverless",
            ),
            "DeleteVpcEndpoint" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteVpcEndpoint is not yet implemented in winterbaume-opensearchserverless",
            ),
            "GetAccessPolicy" => json_error_response(
                501,
                "NotImplementedError",
                "GetAccessPolicy is not yet implemented in winterbaume-opensearchserverless",
            ),
            "GetAccountSettings" => json_error_response(
                501,
                "NotImplementedError",
                "GetAccountSettings is not yet implemented in winterbaume-opensearchserverless",
            ),
            "GetIndex" => json_error_response(
                501,
                "NotImplementedError",
                "GetIndex is not yet implemented in winterbaume-opensearchserverless",
            ),
            "GetPoliciesStats" => json_error_response(
                501,
                "NotImplementedError",
                "GetPoliciesStats is not yet implemented in winterbaume-opensearchserverless",
            ),
            "GetSecurityConfig" => json_error_response(
                501,
                "NotImplementedError",
                "GetSecurityConfig is not yet implemented in winterbaume-opensearchserverless",
            ),
            "ListAccessPolicies" => json_error_response(
                501,
                "NotImplementedError",
                "ListAccessPolicies is not yet implemented in winterbaume-opensearchserverless",
            ),
            "ListCollectionGroups" => json_error_response(
                501,
                "NotImplementedError",
                "ListCollectionGroups is not yet implemented in winterbaume-opensearchserverless",
            ),
            "ListLifecyclePolicies" => json_error_response(
                501,
                "NotImplementedError",
                "ListLifecyclePolicies is not yet implemented in winterbaume-opensearchserverless",
            ),
            "ListSecurityConfigs" => json_error_response(
                501,
                "NotImplementedError",
                "ListSecurityConfigs is not yet implemented in winterbaume-opensearchserverless",
            ),
            "ListVpcEndpoints" => json_error_response(
                501,
                "NotImplementedError",
                "ListVpcEndpoints is not yet implemented in winterbaume-opensearchserverless",
            ),
            "UpdateAccessPolicy" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateAccessPolicy is not yet implemented in winterbaume-opensearchserverless",
            ),
            "UpdateAccountSettings" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateAccountSettings is not yet implemented in winterbaume-opensearchserverless",
            ),
            "UpdateCollection" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateCollection is not yet implemented in winterbaume-opensearchserverless",
            ),
            "UpdateCollectionGroup" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateCollectionGroup is not yet implemented in winterbaume-opensearchserverless",
            ),
            "UpdateIndex" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateIndex is not yet implemented in winterbaume-opensearchserverless",
            ),
            "UpdateLifecyclePolicy" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateLifecyclePolicy is not yet implemented in winterbaume-opensearchserverless",
            ),
            "UpdateSecurityConfig" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateSecurityConfig is not yet implemented in winterbaume-opensearchserverless",
            ),
            "UpdateVpcEndpoint" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateVpcEndpoint is not yet implemented in winterbaume-opensearchserverless",
            ),
            _ => json_error_response(400, "InvalidAction", &format!("Unknown action: {action}")),
        };

        if mutating && response.status / 100 == 2 {
            let state_arc = self.state.get(account_id, &region);
            let view = {
                let guard = state_arc.read().await;
                OpenSearchServerlessStateView::from(&*guard)
            };
            self.notifier.notify(account_id, &region, &view);
        }

        response
    }

    // ---- Collection handlers ----

    async fn handle_create_collection(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchServerlessState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_collection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'name'");
        }
        let name = input.name.as_str();
        let type_ = input.r#type.as_deref();
        let description = input.description.as_deref();
        let tags = wire_tags_to_types(input.tags.as_deref().unwrap_or(&[]));

        let mut st = state.write().await;
        match st.create_collection(name, type_, description, tags, region, account_id) {
            Ok(c) => wire::serialize_create_collection_response(&CreateCollectionResponse {
                create_collection_detail: Some(CreateCollectionDetail {
                    id: Some(c.id.clone()),
                    name: Some(c.name.clone()),
                    status: Some(c.status.clone()),
                    r#type: Some(c.type_.clone()),
                    description: c.description.clone(),
                    arn: Some(c.arn.clone()),
                    kms_key_arn: c.kms_key_arn.clone(),
                    created_date: Some(c.created_date),
                    last_modified_date: Some(c.last_modified_date),
                    ..Default::default()
                }),
            }),
            Err(e) => aoss_error_response(&e),
        }
    }

    async fn handle_delete_collection(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchServerlessState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_collection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'id'");
        }
        let id = input.id.as_str();
        let mut st = state.write().await;
        match st.delete_collection(id) {
            Ok(c) => wire::serialize_delete_collection_response(&DeleteCollectionResponse {
                delete_collection_detail: Some(DeleteCollectionDetail {
                    id: Some(c.id.clone()),
                    name: Some(c.name.clone()),
                    status: Some("DELETING".to_string()),
                }),
            }),
            Err(e) => aoss_error_response(&e),
        }
    }

    async fn handle_list_collections(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchServerlessState>>,
    ) -> MockResponse {
        let st = state.read().await;
        let collections: Vec<CollectionSummary> = st
            .list_collections()
            .into_iter()
            .map(|c| CollectionSummary {
                id: Some(c.id.clone()),
                name: Some(c.name.clone()),
                status: Some(c.status.clone()),
                arn: Some(c.arn.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_collections_response(&ListCollectionsResponse {
            collection_summaries: if collections.is_empty() {
                None
            } else {
                Some(collections)
            },
            next_token: None,
        })
    }

    async fn handle_batch_get_collection(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchServerlessState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_collection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let st = state.read().await;

        let collections: Vec<CollectionDetail> = if let Some(ref id_list) = input.ids {
            st.batch_get_collection_by_ids(id_list)
                .into_iter()
                .map(collection_to_detail)
                .collect()
        } else if let Some(ref name_list) = input.names {
            st.batch_get_collection_by_names(name_list)
                .into_iter()
                .map(collection_to_detail)
                .collect()
        } else {
            vec![]
        };

        wire::serialize_batch_get_collection_response(&BatchGetCollectionResponse {
            collection_details: if collections.is_empty() {
                None
            } else {
                Some(collections)
            },
            collection_error_details: None,
        })
    }

    // ---- SecurityPolicy handlers ----

    async fn handle_create_security_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchServerlessState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_security_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.r#type.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'type'");
        }
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'name'");
        }
        if input.policy.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'policy'");
        }
        let type_ = input.r#type.as_str();
        let name = input.name.as_str();
        // policy is a JSON document encoded as a string; try to parse, falling back to raw string.
        let policy: Value = serde_json::from_str(&input.policy)
            .unwrap_or_else(|_| Value::String(input.policy.clone()));
        let description = input.description.as_deref();

        let mut st = state.write().await;
        match st.create_security_policy(type_, name, policy, description) {
            Ok(sp) => {
                wire::serialize_create_security_policy_response(&CreateSecurityPolicyResponse {
                    security_policy_detail: Some(security_policy_to_detail(&sp)),
                })
            }
            Err(e) => aoss_error_response(&e),
        }
    }

    async fn handle_get_security_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchServerlessState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_security_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.r#type.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'type'");
        }
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'name'");
        }
        let type_ = input.r#type.as_str();
        let name = input.name.as_str();
        let st = state.read().await;
        match st.get_security_policy(type_, name) {
            Ok(sp) => wire::serialize_get_security_policy_response(&GetSecurityPolicyResponse {
                security_policy_detail: Some(security_policy_to_detail(sp)),
            }),
            Err(e) => aoss_error_response(&e),
        }
    }

    async fn handle_update_security_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchServerlessState>>,
        body: &[u8],
    ) -> MockResponse {
        let req: UpdateSecurityPolicyRequest =
            match wire::deserialize_update_security_policy_request(body) {
                Ok(r) => r,
                Err(e) => return json_error_response(400, "ValidationException", &e),
            };

        let type_ = req.r#type.clone();
        let name = req.name.clone();
        let policy_version = req.policy_version.clone();

        // Policy is Option<String> (JSON document as string)
        let policy = req
            .policy
            .as_ref()
            .map(|s| serde_json::from_str(s).unwrap_or_else(|_| Value::String(s.clone())));
        let description = req.description.as_deref();
        let mut st = state.write().await;
        match st.update_security_policy(&type_, &name, &policy_version, policy, description) {
            Ok(sp) => {
                wire::serialize_update_security_policy_response(&UpdateSecurityPolicyResponse {
                    security_policy_detail: Some(security_policy_to_detail(&sp)),
                })
            }
            Err(e) => aoss_error_response(&e),
        }
    }

    async fn handle_list_security_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchServerlessState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_security_policies_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.r#type.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'type'");
        }
        let type_ = input.r#type.as_str();
        let st = state.read().await;
        let summaries: Vec<SecurityPolicySummary> = st
            .list_security_policies(type_)
            .into_iter()
            .map(|sp| SecurityPolicySummary {
                name: Some(sp.name.clone()),
                r#type: Some(sp.type_.clone()),
                policy_version: Some(sp.policy_version.clone()),
                description: sp.description.clone(),
                created_date: Some(sp.created_date),
                last_modified_date: Some(sp.last_modified_date),
            })
            .collect();
        wire::serialize_list_security_policies_response(&ListSecurityPoliciesResponse {
            security_policy_summaries: if summaries.is_empty() {
                None
            } else {
                Some(summaries)
            },
            next_token: None,
        })
    }

    // ---- VpcEndpoint handlers ----

    async fn handle_create_vpc_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchServerlessState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_vpc_endpoint_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'name'");
        }
        if input.vpc_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'vpcId'");
        }
        let name = input.name.as_str();
        let vpc_id = input.vpc_id.as_str();
        let subnet_ids = input.subnet_ids;
        let security_group_ids = input.security_group_ids.unwrap_or_default();

        let mut st = state.write().await;
        match st.create_vpc_endpoint(name, vpc_id, subnet_ids, security_group_ids) {
            Ok(ep) => wire::serialize_create_vpc_endpoint_response(&CreateVpcEndpointResponse {
                create_vpc_endpoint_detail: Some(CreateVpcEndpointDetail {
                    id: Some(ep.id.clone()),
                    name: Some(ep.name.clone()),
                    status: Some(ep.status.clone()),
                }),
            }),
            Err(e) => aoss_error_response(&e),
        }
    }

    // ---- Tag handlers ----

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchServerlessState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'resourceArn'");
        }
        let resource_arn = input.resource_arn.as_str();
        let tags = wire_tags_to_types(&input.tags);
        let mut st = state.write().await;
        match st.tag_resource(resource_arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&TagResourceResponse {}),
            Err(e) => aoss_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchServerlessState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'resourceArn'");
        }
        let resource_arn = input.resource_arn.as_str();
        let tag_keys = input.tag_keys;
        let mut st = state.write().await;
        match st.untag_resource(resource_arn, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&UntagResourceResponse {}),
            Err(e) => aoss_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<OpenSearchServerlessState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'resourceArn'");
        }
        let resource_arn = input.resource_arn.as_str();
        let st = state.read().await;
        match st.list_tags_for_resource(resource_arn) {
            Ok(tags) => {
                let tag_list: Vec<Tag> = tags
                    .into_iter()
                    .map(|t| Tag {
                        key: t.key.clone(),
                        value: t.value.clone(),
                    })
                    .collect();
                wire::serialize_list_tags_for_resource_response(&ListTagsForResourceResponse {
                    tags: if tag_list.is_empty() {
                        None
                    } else {
                        Some(tag_list)
                    },
                })
            }
            Err(e) => aoss_error_response(&e),
        }
    }
}

// ---- Helper functions ----

fn collection_to_detail(c: &types::Collection) -> CollectionDetail {
    CollectionDetail {
        id: Some(c.id.clone()),
        name: Some(c.name.clone()),
        status: Some(c.status.clone()),
        r#type: Some(c.type_.clone()),
        description: c.description.clone(),
        arn: Some(c.arn.clone()),
        kms_key_arn: c.kms_key_arn.clone(),
        created_date: Some(c.created_date),
        last_modified_date: Some(c.last_modified_date),
        ..Default::default()
    }
}

fn security_policy_to_detail(sp: &types::SecurityPolicy) -> SecurityPolicyDetail {
    SecurityPolicyDetail {
        name: Some(sp.name.clone()),
        r#type: Some(sp.type_.clone()),
        policy_version: Some(sp.policy_version.clone()),
        description: sp.description.clone(),
        policy: Some(sp.policy.clone()),
        created_date: Some(sp.created_date),
        last_modified_date: Some(sp.last_modified_date),
    }
}

fn wire_tags_to_types(tags: &[Tag]) -> Vec<types::Tag> {
    tags.iter()
        .filter(|t| !t.key.is_empty())
        .map(|t| types::Tag {
            key: t.key.clone(),
            value: t.value.clone(),
        })
        .collect()
}

fn aoss_error_response(e: &OpenSearchServerlessError) -> MockResponse {
    let (status, error_type) = match e {
        OpenSearchServerlessError::CollectionAlreadyExists { .. } => (409, "ConflictException"),
        OpenSearchServerlessError::SecurityPolicyAlreadyExists { .. } => (409, "ConflictException"),
        OpenSearchServerlessError::VpcEndpointAlreadyExists { .. } => (409, "ConflictException"),
        OpenSearchServerlessError::PolicyVersionMismatch { .. } => (409, "ConflictException"),
        OpenSearchServerlessError::CollectionNotFound { .. } => (404, "ResourceNotFoundException"),
        OpenSearchServerlessError::SecurityPolicyNotFound { .. } => {
            (404, "ResourceNotFoundException")
        }
        OpenSearchServerlessError::ResourceNotFound { .. } => (404, "ResourceNotFoundException"),
    };
    json_error_response(status, error_type, &e.to_string())
}
