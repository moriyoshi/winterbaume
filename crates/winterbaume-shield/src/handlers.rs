use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{ShieldError, ShieldState};
use crate::views::ShieldStateView;
use crate::wire;

/// Shield service handler that processes awsJson1.1 protocol requests.
pub struct ShieldService {
    pub(crate) state: Arc<BackendState<ShieldState>>,
    pub(crate) notifier: StateChangeNotifier<ShieldStateView>,
}

impl ShieldService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ShieldService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ShieldService {
    fn service_name(&self) -> &str {
        "shield"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://shield\..*\.amazonaws\.com",
            r"https?://shield\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl ShieldService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        // Extract action from X-Amz-Target header
        // Format: "AWSShield_20160616.CreateSubscription"
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

        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "InvalidInput", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreateSubscription" => {
                self.handle_create_subscription(&state, account_id, &region)
                    .await
            }
            "DescribeSubscription" => self.handle_describe_subscription(&state).await,
            "CreateProtection" => {
                self.handle_create_protection(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeProtection" => self.handle_describe_protection(&state, body_bytes).await,
            "DeleteProtection" => self.handle_delete_protection(&state, body_bytes).await,
            "ListProtections" => self.handle_list_protections(&state).await,
            // --- Unimplemented operations (auto-generated stubs) ---
            "AssociateDRTLogBucket" => json_error_response(
                501,
                "NotImplementedError",
                "AssociateDRTLogBucket is not yet implemented in winterbaume-shield",
            ),
            "AssociateDRTRole" => json_error_response(
                501,
                "NotImplementedError",
                "AssociateDRTRole is not yet implemented in winterbaume-shield",
            ),
            "AssociateHealthCheck" => json_error_response(
                501,
                "NotImplementedError",
                "AssociateHealthCheck is not yet implemented in winterbaume-shield",
            ),
            "AssociateProactiveEngagementDetails" => json_error_response(
                501,
                "NotImplementedError",
                "AssociateProactiveEngagementDetails is not yet implemented in winterbaume-shield",
            ),
            "CreateProtectionGroup" => json_error_response(
                501,
                "NotImplementedError",
                "CreateProtectionGroup is not yet implemented in winterbaume-shield",
            ),
            "DeleteProtectionGroup" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteProtectionGroup is not yet implemented in winterbaume-shield",
            ),
            "DeleteSubscription" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteSubscription is not yet implemented in winterbaume-shield",
            ),
            "DescribeAttack" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeAttack is not yet implemented in winterbaume-shield",
            ),
            "DescribeAttackStatistics" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeAttackStatistics is not yet implemented in winterbaume-shield",
            ),
            "DescribeDRTAccess" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeDRTAccess is not yet implemented in winterbaume-shield",
            ),
            "DescribeEmergencyContactSettings" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeEmergencyContactSettings is not yet implemented in winterbaume-shield",
            ),
            "DescribeProtectionGroup" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeProtectionGroup is not yet implemented in winterbaume-shield",
            ),
            "DisableApplicationLayerAutomaticResponse" => json_error_response(
                501,
                "NotImplementedError",
                "DisableApplicationLayerAutomaticResponse is not yet implemented in winterbaume-shield",
            ),
            "DisableProactiveEngagement" => json_error_response(
                501,
                "NotImplementedError",
                "DisableProactiveEngagement is not yet implemented in winterbaume-shield",
            ),
            "DisassociateDRTLogBucket" => json_error_response(
                501,
                "NotImplementedError",
                "DisassociateDRTLogBucket is not yet implemented in winterbaume-shield",
            ),
            "DisassociateDRTRole" => json_error_response(
                501,
                "NotImplementedError",
                "DisassociateDRTRole is not yet implemented in winterbaume-shield",
            ),
            "DisassociateHealthCheck" => json_error_response(
                501,
                "NotImplementedError",
                "DisassociateHealthCheck is not yet implemented in winterbaume-shield",
            ),
            "EnableApplicationLayerAutomaticResponse" => json_error_response(
                501,
                "NotImplementedError",
                "EnableApplicationLayerAutomaticResponse is not yet implemented in winterbaume-shield",
            ),
            "EnableProactiveEngagement" => json_error_response(
                501,
                "NotImplementedError",
                "EnableProactiveEngagement is not yet implemented in winterbaume-shield",
            ),
            "GetSubscriptionState" => json_error_response(
                501,
                "NotImplementedError",
                "GetSubscriptionState is not yet implemented in winterbaume-shield",
            ),
            "ListAttacks" => json_error_response(
                501,
                "NotImplementedError",
                "ListAttacks is not yet implemented in winterbaume-shield",
            ),
            "ListProtectionGroups" => json_error_response(
                501,
                "NotImplementedError",
                "ListProtectionGroups is not yet implemented in winterbaume-shield",
            ),
            "ListResourcesInProtectionGroup" => json_error_response(
                501,
                "NotImplementedError",
                "ListResourcesInProtectionGroup is not yet implemented in winterbaume-shield",
            ),
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "UpdateApplicationLayerAutomaticResponse" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateApplicationLayerAutomaticResponse is not yet implemented in winterbaume-shield",
            ),
            "UpdateEmergencyContactSettings" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateEmergencyContactSettings is not yet implemented in winterbaume-shield",
            ),
            "UpdateProtectionGroup" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateProtectionGroup is not yet implemented in winterbaume-shield",
            ),
            "UpdateSubscription" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateSubscription is not yet implemented in winterbaume-shield",
            ),
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for Shield"),
            ),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<ShieldState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.create_subscription(account_id, region) {
            Ok(_) => wire::serialize_create_subscription_response(
                &crate::model::CreateSubscriptionResponse {},
            ),
            Err(e) => shield_error_response(&e),
        }
    }

    async fn handle_describe_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<ShieldState>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_subscription() {
            Ok(sub) => wire::serialize_describe_subscription_response(
                &crate::model::DescribeSubscriptionResponse {
                    subscription: Some(subscription_to_model(sub)),
                },
            ),
            Err(e) => shield_error_response(&e),
        }
    }

    async fn handle_create_protection(
        &self,
        state: &Arc<tokio::sync::RwLock<ShieldState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_protection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'Name'");
        }
        if input.resource_arn.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'ResourceArn'");
        }
        let name = input.name.as_str();
        let resource_arn = input.resource_arn.as_str();

        let mut state = state.write().await;
        match state.create_protection(name, resource_arn, account_id, region) {
            Ok(protection) => wire::serialize_create_protection_response(
                &crate::model::CreateProtectionResponse {
                    protection_id: Some(protection.id.clone()),
                },
            ),
            Err(e) => shield_error_response(&e),
        }
    }

    async fn handle_describe_protection(
        &self,
        state: &Arc<tokio::sync::RwLock<ShieldState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_protection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let protection_id = input.protection_id.as_deref();
        let resource_arn = input.resource_arn.as_deref();

        let state = state.read().await;
        match state.describe_protection(protection_id, resource_arn) {
            Ok(protection) => wire::serialize_describe_protection_response(
                &crate::model::DescribeProtectionResponse {
                    protection: Some(protection_to_model(protection)),
                },
            ),
            Err(e) => shield_error_response(&e),
        }
    }

    async fn handle_delete_protection(
        &self,
        state: &Arc<tokio::sync::RwLock<ShieldState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_protection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.protection_id.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'ProtectionId'");
        }
        let protection_id = input.protection_id.as_str();

        let mut state = state.write().await;
        match state.delete_protection(protection_id) {
            Ok(()) => wire::serialize_delete_protection_response(
                &crate::model::DeleteProtectionResponse {},
            ),
            Err(e) => shield_error_response(&e),
        }
    }

    async fn handle_list_protections(
        &self,
        state: &Arc<tokio::sync::RwLock<ShieldState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let protections = state.list_protections();
        let entries: Vec<crate::model::Protection> =
            protections.iter().map(|p| protection_to_model(p)).collect();

        wire::serialize_list_protections_response(&crate::model::ListProtectionsResponse {
            next_token: None,
            protections: Some(entries),
        })
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ShieldState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'ResourceARN'");
        }
        let resource_arn = input.resource_a_r_n.as_str();

        let state = state.read().await;
        let tags = state.tags.get(resource_arn).cloned().unwrap_or_default();
        let tag_entries: Vec<crate::model::Tag> = tags.iter().map(tag_to_model).collect();

        wire::serialize_list_tags_for_resource_response(
            &crate::model::ListTagsForResourceResponse {
                tags: Some(tag_entries),
            },
        )
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ShieldState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'ResourceARN'");
        }
        let resource_arn = input.resource_a_r_n.as_str();
        let new_tags: Vec<crate::types::Tag> = input
            .tags
            .into_iter()
            .filter_map(|t| {
                let key = t.key?;
                let value = t.value?;
                Some(crate::types::Tag { key, value })
            })
            .collect();

        let mut state = state.write().await;
        let tags = state.tags.entry(resource_arn.to_string()).or_default();
        for new_tag in new_tags {
            if let Some(existing) = tags.iter_mut().find(|t| t.key == new_tag.key) {
                existing.value = new_tag.value;
            } else {
                tags.push(new_tag);
            }
        }

        wire::serialize_tag_resource_response(&crate::model::TagResourceResponse {})
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ShieldState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'ResourceARN'");
        }
        let resource_arn = input.resource_a_r_n.as_str();
        let tag_keys = input.tag_keys;

        let mut state = state.write().await;
        if let Some(tags) = state.tags.get_mut(resource_arn) {
            tags.retain(|t| !tag_keys.contains(&t.key));
        }

        wire::serialize_untag_resource_response(&crate::model::UntagResourceResponse {})
    }
}

// --- Conversion helpers ---

fn protection_to_model(p: &crate::types::Protection) -> crate::model::Protection {
    crate::model::Protection {
        application_layer_automatic_response_configuration: None,
        health_check_ids: Some(p.health_check_ids.clone()),
        id: Some(p.id.clone()),
        name: Some(p.name.clone()),
        protection_arn: Some(p.protection_arn.clone()),
        resource_arn: Some(p.resource_arn.clone()),
    }
}

fn subscription_to_model(sub: &crate::types::Subscription) -> crate::model::Subscription {
    crate::model::Subscription {
        auto_renew: Some(sub.auto_renew.as_str().to_string()),
        end_time: sub.end_time.map(|t| t.timestamp() as f64),
        limits: None,
        proactive_engagement_status: None,
        start_time: Some(sub.start_time.timestamp() as f64),
        subscription_arn: Some(sub.subscription_arn.clone()),
        subscription_limits: Some(crate::model::SubscriptionLimits {
            protection_group_limits: Some(crate::model::ProtectionGroupLimits {
                max_protection_groups: Some(5),
                pattern_type_limits: Some(crate::model::ProtectionGroupPatternTypeLimits {
                    arbitrary_pattern_limits: Some(
                        crate::model::ProtectionGroupArbitraryPatternLimits {
                            max_members: Some(10000),
                        },
                    ),
                }),
            }),
            protection_limits: Some(crate::model::ProtectionLimits {
                protected_resource_type_limits: Some(vec![]),
            }),
        }),
        time_commitment_in_seconds: Some(sub.time_commitment_in_seconds),
    }
}

fn tag_to_model(t: &crate::types::Tag) -> crate::model::Tag {
    crate::model::Tag {
        key: Some(t.key.clone()),
        value: Some(t.value.clone()),
    }
}

// --- Error utility functions ---

fn shield_error_response(err: &ShieldError) -> MockResponse {
    let (status, error_type) = match err {
        ShieldError::SubscriptionAlreadyExists => (400, "ResourceAlreadyExistsException"),
        ShieldError::SubscriptionNotFound => (400, "ResourceNotFoundException"),
        ShieldError::ProtectionAlreadyExists { .. } => (400, "ResourceAlreadyExistsException"),
        ShieldError::ProtectionNotFound { .. } => (400, "ResourceNotFoundException"),
        ShieldError::ProtectionNotFoundForResource { .. } => (400, "ResourceNotFoundException"),
        ShieldError::MissingProtectionIdentifier => (400, "InvalidParameterException"),
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
