use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService, json_error_response,
};

use crate::model;
use crate::state::{WafV2Error, WafV2State};
use crate::types::*;
use crate::views::Wafv2StateView;
use crate::wire;

fn tags_from_wire(tags: Option<Vec<model::Tag>>) -> Vec<(String, String)> {
    tags.unwrap_or_default()
        .into_iter()
        .map(|t| (t.key, t.value))
        .collect()
}

fn to_value<T: serde::Serialize>(v: &T) -> Value {
    serde_json::to_value(v).unwrap_or(Value::Null)
}

fn to_value_opt<T: serde::Serialize>(v: &Option<T>) -> Option<Value> {
    v.as_ref().and_then(|x| serde_json::to_value(x).ok())
}

pub struct WafV2Service {
    pub(crate) state: Arc<BackendState<WafV2State>>,
    pub(crate) notifier: StateChangeNotifier<Wafv2StateView>,
}

impl WafV2Service {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for WafV2Service {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for WafV2Service {
    fn service_name(&self) -> &str {
        "wafv2"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://wafv2\..*\.amazonaws\.com",
            r"https?://wafv2\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

// ── Helpers ──

fn wafv2_error_response(err: &WafV2Error) -> MockResponse {
    let (status, error_type) = match err {
        WafV2Error::NonexistentItem => (400, "WAFNonexistentItemException"),
        WafV2Error::DuplicateItem => (400, "WAFDuplicateItemException"),
        WafV2Error::OptimisticLock => (400, "WAFOptimisticLockException"),
    };
    MockResponse::json(
        status,
        json!({"__type": error_type, "message": err.to_string()}).to_string(),
    )
}

fn to_wire_web_acl_summary(acl: &WebAcl) -> wire::WebACLSummary {
    wire::WebACLSummary {
        name: Some(acl.name.clone()),
        id: Some(acl.id.clone()),
        a_r_n: Some(acl.arn.clone()),
        lock_token: Some(acl.lock_token.clone()),
        description: Some(acl.description.clone()),
    }
}

fn json_to_wire_default_action(v: &serde_json::Value) -> wire::DefaultAction {
    serde_json::from_value(v.clone()).unwrap_or_default()
}

fn json_to_wire_visibility_config(v: &serde_json::Value) -> wire::VisibilityConfig {
    serde_json::from_value(v.clone()).unwrap_or_default()
}

fn json_to_wire_rules(v: &serde_json::Value) -> Option<Vec<wire::Rule>> {
    serde_json::from_value(v.clone()).ok()
}

fn json_to_wire_association_config(v: &serde_json::Value) -> Option<wire::AssociationConfig> {
    serde_json::from_value(v.clone()).ok()
}

fn json_to_wire_custom_response_bodies(
    v: &serde_json::Value,
) -> Option<std::collections::HashMap<String, wire::CustomResponseBody>> {
    serde_json::from_value(v.clone()).ok()
}

fn json_to_wire_captcha_config(v: &serde_json::Value) -> Option<wire::CaptchaConfig> {
    serde_json::from_value(v.clone()).ok()
}

fn json_to_wire_challenge_config(v: &serde_json::Value) -> Option<wire::ChallengeConfig> {
    serde_json::from_value(v.clone()).ok()
}

fn to_wire_web_acl(acl: &WebAcl) -> wire::WebACL {
    wire::WebACL {
        name: Some(acl.name.clone()),
        id: Some(acl.id.clone()),
        a_r_n: Some(acl.arn.clone()),
        description: Some(acl.description.clone()),
        default_action: Some(json_to_wire_default_action(&acl.default_action_json)),
        visibility_config: Some(json_to_wire_visibility_config(&acl.visibility_config_json)),
        rules: json_to_wire_rules(&acl.rules_json),
        association_config: acl
            .association_config_json
            .as_ref()
            .and_then(json_to_wire_association_config),
        custom_response_bodies: acl
            .custom_response_bodies_json
            .as_ref()
            .and_then(json_to_wire_custom_response_bodies),
        captcha_config: acl
            .captcha_config_json
            .as_ref()
            .and_then(json_to_wire_captcha_config),
        challenge_config: acl
            .challenge_config_json
            .as_ref()
            .and_then(json_to_wire_challenge_config),
        token_domains: acl.token_domains.clone(),
        label_namespace: Some(acl.label_namespace.clone()),
        ..Default::default()
    }
}

fn to_wire_ip_set_summary(s: &IpSet) -> wire::IPSetSummary {
    wire::IPSetSummary {
        name: Some(s.name.clone()),
        id: Some(s.id.clone()),
        a_r_n: Some(s.arn.clone()),
        lock_token: Some(s.lock_token.clone()),
        description: Some(s.description.clone()),
    }
}

fn to_wire_ip_set(s: &IpSet) -> wire::IPSet {
    wire::IPSet {
        name: Some(s.name.clone()),
        id: Some(s.id.clone()),
        a_r_n: Some(s.arn.clone()),
        description: Some(s.description.clone()),
        i_p_address_version: Some(s.ip_address_version.clone()),
        addresses: Some(s.addresses.clone()),
    }
}

fn to_wire_regex_pattern_set_summary(s: &RegexPatternSet) -> wire::RegexPatternSetSummary {
    wire::RegexPatternSetSummary {
        name: Some(s.name.clone()),
        id: Some(s.id.clone()),
        a_r_n: Some(s.arn.clone()),
        lock_token: Some(s.lock_token.clone()),
        description: Some(s.description.clone()),
    }
}

fn to_wire_regex_pattern_set(s: &RegexPatternSet) -> wire::RegexPatternSet {
    wire::RegexPatternSet {
        name: Some(s.name.clone()),
        id: Some(s.id.clone()),
        a_r_n: Some(s.arn.clone()),
        description: Some(s.description.clone()),
        regular_expression_list: Some(
            s.regular_expressions
                .iter()
                .map(|r| wire::Regex {
                    regex_string: Some(r.clone()),
                })
                .collect(),
        ),
    }
}

fn to_wire_rule_group_summary(s: &RuleGroupData) -> wire::RuleGroupSummary {
    wire::RuleGroupSummary {
        name: Some(s.name.clone()),
        id: Some(s.id.clone()),
        a_r_n: Some(s.arn.clone()),
        lock_token: Some(s.lock_token.clone()),
        description: Some(s.description.clone()),
    }
}

fn to_wire_rule_group(s: &RuleGroupData) -> wire::RuleGroup {
    wire::RuleGroup {
        name: Some(s.name.clone()),
        id: Some(s.id.clone()),
        a_r_n: Some(s.arn.clone()),
        description: Some(s.description.clone()),
        capacity: Some(s.capacity),
        visibility_config: Some(json_to_wire_visibility_config(&s.visibility_config_json)),
        rules: json_to_wire_rules(&s.rules_json),
        custom_response_bodies: s
            .custom_response_bodies_json
            .as_ref()
            .and_then(json_to_wire_custom_response_bodies),
        label_namespace: Some(s.label_namespace.clone()),
        ..Default::default()
    }
}

fn to_wire_logging_configuration(lc: &LoggingConfigData) -> wire::LoggingConfiguration {
    wire::LoggingConfiguration {
        resource_arn: lc.resource_arn.clone(),
        log_destination_configs: lc.log_destination_configs.clone(),
        log_scope: lc.log_scope.clone(),
        log_type: lc.log_type.clone(),
        ..Default::default()
    }
}

async fn to_wire_tags(tags: &[(String, String)]) -> Vec<wire::Tag> {
    tags.iter()
        .map(|(k, v)| wire::Tag {
            key: k.clone(),
            value: v.clone(),
        })
        .collect()
}

// ── Dispatch ──

impl WafV2Service {
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

        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            // ── WebACL ──
            "CreateWebACL" => {
                self.handle_create_web_acl(&state, body, &region, account_id)
                    .await
            }
            "GetWebACL" => self.handle_get_web_acl(&state, body).await,
            "DeleteWebACL" => self.handle_delete_web_acl(&state, body).await,
            "ListWebACLs" => self.handle_list_web_acls(&state, body).await,
            "UpdateWebACL" => self.handle_update_web_acl(&state, body).await,
            "GetWebACLForResource" => self.handle_get_web_acl_for_resource(&state, body).await,
            "AssociateWebACL" => self.handle_associate_web_acl(&state, body).await,
            "DisassociateWebACL" => self.handle_disassociate_web_acl(&state, body).await,

            // ── IPSet ──
            "CreateIPSet" => {
                self.handle_create_ip_set(&state, body, &region, account_id)
                    .await
            }
            "GetIPSet" => self.handle_get_ip_set(&state, body).await,
            "DeleteIPSet" => self.handle_delete_ip_set(&state, body).await,
            "UpdateIPSet" => self.handle_update_ip_set(&state, body).await,
            "ListIPSets" => self.handle_list_ip_sets(&state, body).await,

            // ── RegexPatternSet ──
            "CreateRegexPatternSet" => {
                self.handle_create_regex_pattern_set(&state, body, &region, account_id)
                    .await
            }
            "GetRegexPatternSet" => self.handle_get_regex_pattern_set(&state, body).await,
            "DeleteRegexPatternSet" => self.handle_delete_regex_pattern_set(&state, body).await,
            "UpdateRegexPatternSet" => self.handle_update_regex_pattern_set(&state, body).await,
            "ListRegexPatternSets" => self.handle_list_regex_pattern_sets(&state, body).await,

            // ── RuleGroup ──
            "CreateRuleGroup" => {
                self.handle_create_rule_group(&state, body, &region, account_id)
                    .await
            }
            "GetRuleGroup" => self.handle_get_rule_group(&state, body).await,
            "DeleteRuleGroup" => self.handle_delete_rule_group(&state, body).await,
            "UpdateRuleGroup" => self.handle_update_rule_group(&state, body).await,
            "ListRuleGroups" => self.handle_list_rule_groups(&state, body).await,

            // ── LoggingConfiguration ──
            "PutLoggingConfiguration" => self.handle_put_logging_configuration(&state, body).await,
            "GetLoggingConfiguration" => self.handle_get_logging_configuration(&state, body).await,
            "DeleteLoggingConfiguration" => {
                self.handle_delete_logging_configuration(&state, body).await
            }
            "ListLoggingConfigurations" => {
                self.handle_list_logging_configurations(&state, body).await
            }

            // ── Tags ──
            "TagResource" => self.handle_tag_resource(&state, body).await,
            "UntagResource" => self.handle_untag_resource(&state, body).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body).await,

            // ── Permission policies ──
            "PutPermissionPolicy" => self.handle_put_permission_policy(&state, body).await,
            "GetPermissionPolicy" => self.handle_get_permission_policy(&state, body).await,
            "DeletePermissionPolicy" => self.handle_delete_permission_policy(&state, body).await,

            // ── API Keys ──
            "CreateAPIKey" => self.handle_create_api_key(&state, body).await,
            "DeleteAPIKey" => self.handle_delete_api_key(&state, body).await,
            "ListAPIKeys" => self.handle_list_api_keys(&state, body).await,

            // ── CheckCapacity ──
            "CheckCapacity" => self.handle_check_capacity(&state, body).await,

            // ── ListResourcesForWebACL ──
            "ListResourcesForWebACL" => self.handle_list_resources_for_web_acl(&state, body).await,

            // --- Unimplemented operations (auto-generated stubs) ---
            "DeleteFirewallManagerRuleGroups" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteFirewallManagerRuleGroups is not yet implemented in winterbaume-wafv2",
            ),
            "DescribeAllManagedProducts" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeAllManagedProducts is not yet implemented in winterbaume-wafv2",
            ),
            "DescribeManagedProductsByVendor" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeManagedProductsByVendor is not yet implemented in winterbaume-wafv2",
            ),
            "DescribeManagedRuleGroup" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeManagedRuleGroup is not yet implemented in winterbaume-wafv2",
            ),
            "GenerateMobileSdkReleaseUrl" => json_error_response(
                501,
                "NotImplementedError",
                "GenerateMobileSdkReleaseUrl is not yet implemented in winterbaume-wafv2",
            ),
            "GetDecryptedAPIKey" => json_error_response(
                501,
                "NotImplementedError",
                "GetDecryptedAPIKey is not yet implemented in winterbaume-wafv2",
            ),
            "GetManagedRuleSet" => json_error_response(
                501,
                "NotImplementedError",
                "GetManagedRuleSet is not yet implemented in winterbaume-wafv2",
            ),
            "GetMobileSdkRelease" => json_error_response(
                501,
                "NotImplementedError",
                "GetMobileSdkRelease is not yet implemented in winterbaume-wafv2",
            ),
            "GetRateBasedStatementManagedKeys" => json_error_response(
                501,
                "NotImplementedError",
                "GetRateBasedStatementManagedKeys is not yet implemented in winterbaume-wafv2",
            ),
            "GetSampledRequests" => json_error_response(
                501,
                "NotImplementedError",
                "GetSampledRequests is not yet implemented in winterbaume-wafv2",
            ),
            "GetTopPathStatisticsByTraffic" => json_error_response(
                501,
                "NotImplementedError",
                "GetTopPathStatisticsByTraffic is not yet implemented in winterbaume-wafv2",
            ),
            "ListAvailableManagedRuleGroupVersions" => json_error_response(
                501,
                "NotImplementedError",
                "ListAvailableManagedRuleGroupVersions is not yet implemented in winterbaume-wafv2",
            ),
            "ListAvailableManagedRuleGroups" => json_error_response(
                501,
                "NotImplementedError",
                "ListAvailableManagedRuleGroups is not yet implemented in winterbaume-wafv2",
            ),
            "ListManagedRuleSets" => json_error_response(
                501,
                "NotImplementedError",
                "ListManagedRuleSets is not yet implemented in winterbaume-wafv2",
            ),
            "ListMobileSdkReleases" => json_error_response(
                501,
                "NotImplementedError",
                "ListMobileSdkReleases is not yet implemented in winterbaume-wafv2",
            ),
            "PutManagedRuleSetVersions" => json_error_response(
                501,
                "NotImplementedError",
                "PutManagedRuleSetVersions is not yet implemented in winterbaume-wafv2",
            ),
            "UpdateManagedRuleSetVersionExpiryDate" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateManagedRuleSetVersionExpiryDate is not yet implemented in winterbaume-wafv2",
            ),
            _ => json_error_response(400, "InvalidAction", &format!("Unknown operation {action}")),
        };

        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // ── WebACL handlers ──

    async fn handle_create_web_acl(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_web_a_c_l_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Name is required");
        }
        if input.scope.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Scope is required");
        }
        let name = input.name.as_str();
        let scope = input.scope.as_str();
        let description = input.description.as_deref().unwrap_or("");
        let default_action_json = serde_json::to_value(&input.default_action)
            .ok()
            .filter(|v| !v.is_null())
            .unwrap_or(json!({"Allow": {}}));
        let visibility_config_json =
            serde_json::to_value(&input.visibility_config).unwrap_or(json!({}));
        let rules_json = input
            .rules
            .as_ref()
            .map(|r| serde_json::to_value(r).unwrap_or(json!([])))
            .unwrap_or(json!([]));
        let association_config_json = to_value_opt(&input.association_config);
        let custom_response_bodies_json = to_value_opt(&input.custom_response_bodies);
        let captcha_config_json = to_value_opt(&input.captcha_config);
        let challenge_config_json = to_value_opt(&input.challenge_config);
        let token_domains = input.token_domains;
        let tags = tags_from_wire(input.tags);

        let mut state = state.write().await;
        match state.create_web_acl(
            name,
            scope,
            description,
            default_action_json,
            visibility_config_json,
            region,
            account_id,
            rules_json,
            association_config_json,
            custom_response_bodies_json,
            captcha_config_json,
            challenge_config_json,
            token_domains,
            tags,
        ) {
            Ok(acl) => wire::serialize_create_web_a_c_l_response(&wire::CreateWebACLResponse {
                summary: Some(to_wire_web_acl_summary(acl)),
            }),
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_get_web_acl(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_web_a_c_l_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        let name = match input.name.as_deref() {
            Some(s) if !s.is_empty() => s,
            _ => {
                return json_error_response(
                    400,
                    "WAFInvalidParameterException",
                    "Name is required",
                );
            }
        };
        let scope = match input.scope.as_deref() {
            Some(s) if !s.is_empty() => s,
            _ => {
                return json_error_response(
                    400,
                    "WAFInvalidParameterException",
                    "Scope is required",
                );
            }
        };
        let id = match input.id.as_deref() {
            Some(s) if !s.is_empty() => s,
            _ => return json_error_response(400, "WAFInvalidParameterException", "Id is required"),
        };

        let state = state.read().await;
        match state.get_web_acl(name, scope, id) {
            Ok(acl) => wire::serialize_get_web_a_c_l_response(&wire::GetWebACLResponse {
                lock_token: Some(acl.lock_token.clone()),
                web_a_c_l: Some(to_wire_web_acl(acl)),
                application_integration_u_r_l: None,
            }),
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_delete_web_acl(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_web_a_c_l_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Name is required");
        }
        if input.scope.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Scope is required");
        }
        if input.id.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Id is required");
        }
        if input.lock_token.is_empty() {
            return json_error_response(
                400,
                "WAFInvalidParameterException",
                "LockToken is required",
            );
        }

        let mut state = state.write().await;
        match state.delete_web_acl(&input.name, &input.scope, &input.id, &input.lock_token) {
            Ok(()) => wire::serialize_delete_web_a_c_l_response(&wire::DeleteWebACLResponse {}),
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_list_web_acls(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_web_a_c_ls_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.scope.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Scope is required");
        }

        let state = state.read().await;
        let acls = state.list_web_acls(&input.scope);
        let entries: Vec<wire::WebACLSummary> = acls
            .iter()
            .map(|acl| to_wire_web_acl_summary(acl))
            .collect();

        wire::serialize_list_web_a_c_ls_response(&wire::ListWebACLsResponse {
            web_a_c_ls: Some(entries),
            next_marker: None,
        })
    }

    async fn handle_update_web_acl(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_web_a_c_l_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Name is required");
        }
        if input.scope.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Scope is required");
        }
        if input.id.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Id is required");
        }
        if input.lock_token.is_empty() {
            return json_error_response(
                400,
                "WAFInvalidParameterException",
                "LockToken is required",
            );
        }
        let description = input.description.as_deref();
        // For UpdateWebACL, default_action and visibility_config are required, so always pass them.
        let default_action_json = Some(to_value(&input.default_action));
        let visibility_config_json = Some(to_value(&input.visibility_config));
        let rules_json = input.rules.as_ref().map(to_value);

        let mut state = state.write().await;
        match state.update_web_acl(
            &input.name,
            &input.scope,
            &input.id,
            &input.lock_token,
            description,
            default_action_json,
            visibility_config_json,
            rules_json,
        ) {
            Ok(next_lock_token) => {
                wire::serialize_update_web_a_c_l_response(&wire::UpdateWebACLResponse {
                    next_lock_token: Some(next_lock_token),
                })
            }
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_get_web_acl_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_web_a_c_l_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(
                400,
                "WAFInvalidParameterException",
                "ResourceArn is required",
            );
        }

        let state = state.read().await;
        let web_acl = state
            .get_web_acl_for_resource(&input.resource_arn)
            .map(to_wire_web_acl);

        wire::serialize_get_web_a_c_l_for_resource_response(&wire::GetWebACLForResourceResponse {
            web_a_c_l: web_acl,
        })
    }

    async fn handle_associate_web_acl(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_associate_web_a_c_l_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.web_a_c_l_arn.is_empty() {
            return json_error_response(
                400,
                "WAFInvalidParameterException",
                "WebACLArn is required",
            );
        }
        if input.resource_arn.is_empty() {
            return json_error_response(
                400,
                "WAFInvalidParameterException",
                "ResourceArn is required",
            );
        }

        let mut state = state.write().await;
        match state.associate_web_acl(&input.web_a_c_l_arn, &input.resource_arn) {
            Ok(()) => {
                wire::serialize_associate_web_a_c_l_response(&wire::AssociateWebACLResponse {})
            }
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_disassociate_web_acl(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_disassociate_web_a_c_l_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(
                400,
                "WAFInvalidParameterException",
                "ResourceArn is required",
            );
        }

        let mut state = state.write().await;
        match state.disassociate_web_acl(&input.resource_arn) {
            Ok(()) => wire::serialize_disassociate_web_a_c_l_response(
                &wire::DisassociateWebACLResponse {},
            ),
            Err(e) => wafv2_error_response(&e),
        }
    }

    // ── IPSet handlers ──

    async fn handle_create_ip_set(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_i_p_set_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Name is required");
        }
        if input.scope.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Scope is required");
        }
        if input.i_p_address_version.is_empty() {
            return json_error_response(
                400,
                "WAFInvalidParameterException",
                "IPAddressVersion is required",
            );
        }
        let description = input.description.as_deref().unwrap_or("");
        let addresses = input.addresses;
        let tags = tags_from_wire(input.tags);

        let mut state = state.write().await;
        match state.create_ip_set(
            &input.name,
            &input.scope,
            description,
            &input.i_p_address_version,
            addresses,
            region,
            account_id,
            tags,
        ) {
            Ok(s) => wire::serialize_create_i_p_set_response(&wire::CreateIPSetResponse {
                summary: Some(to_wire_ip_set_summary(s)),
            }),
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_get_ip_set(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_i_p_set_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Name is required");
        }
        if input.scope.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Scope is required");
        }
        if input.id.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Id is required");
        }

        let state = state.read().await;
        match state.get_ip_set(&input.name, &input.scope, &input.id) {
            Ok(s) => wire::serialize_get_i_p_set_response(&wire::GetIPSetResponse {
                i_p_set: Some(to_wire_ip_set(s)),
                lock_token: Some(s.lock_token.clone()),
            }),
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_delete_ip_set(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_i_p_set_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Name is required");
        }
        if input.scope.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Scope is required");
        }
        if input.id.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Id is required");
        }
        if input.lock_token.is_empty() {
            return json_error_response(
                400,
                "WAFInvalidParameterException",
                "LockToken is required",
            );
        }

        let mut state = state.write().await;
        match state.delete_ip_set(&input.name, &input.scope, &input.id, &input.lock_token) {
            Ok(()) => wire::serialize_delete_i_p_set_response(&wire::DeleteIPSetResponse {}),
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_update_ip_set(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_i_p_set_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Name is required");
        }
        if input.scope.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Scope is required");
        }
        if input.id.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Id is required");
        }
        if input.lock_token.is_empty() {
            return json_error_response(
                400,
                "WAFInvalidParameterException",
                "LockToken is required",
            );
        }
        let description = input.description.as_deref();
        let addresses = input.addresses;

        let mut state = state.write().await;
        match state.update_ip_set(
            &input.name,
            &input.scope,
            &input.id,
            &input.lock_token,
            description,
            addresses,
        ) {
            Ok(next_lock_token) => {
                wire::serialize_update_i_p_set_response(&wire::UpdateIPSetResponse {
                    next_lock_token: Some(next_lock_token),
                })
            }
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_list_ip_sets(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_i_p_sets_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.scope.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Scope is required");
        }

        let state = state.read().await;
        let sets = state.list_ip_sets(&input.scope);
        let entries: Vec<wire::IPSetSummary> =
            sets.iter().map(|s| to_wire_ip_set_summary(s)).collect();

        wire::serialize_list_i_p_sets_response(&wire::ListIPSetsResponse {
            i_p_sets: Some(entries),
            next_marker: None,
        })
    }

    // ── RegexPatternSet handlers ──

    async fn handle_create_regex_pattern_set(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_regex_pattern_set_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Name is required");
        }
        if input.scope.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Scope is required");
        }
        let description = input.description.as_deref().unwrap_or("");
        let regular_expressions: Vec<String> = input
            .regular_expression_list
            .into_iter()
            .filter_map(|r| r.regex_string)
            .collect();
        let tags = tags_from_wire(input.tags);

        let mut state = state.write().await;
        match state.create_regex_pattern_set(
            &input.name,
            &input.scope,
            description,
            regular_expressions,
            region,
            account_id,
            tags,
        ) {
            Ok(s) => wire::serialize_create_regex_pattern_set_response(
                &wire::CreateRegexPatternSetResponse {
                    summary: Some(to_wire_regex_pattern_set_summary(s)),
                },
            ),
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_get_regex_pattern_set(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_regex_pattern_set_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Name is required");
        }
        if input.scope.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Scope is required");
        }
        if input.id.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Id is required");
        }

        let state = state.read().await;
        match state.get_regex_pattern_set(&input.name, &input.scope, &input.id) {
            Ok(s) => {
                wire::serialize_get_regex_pattern_set_response(&wire::GetRegexPatternSetResponse {
                    regex_pattern_set: Some(to_wire_regex_pattern_set(s)),
                    lock_token: Some(s.lock_token.clone()),
                })
            }
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_delete_regex_pattern_set(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_regex_pattern_set_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Name is required");
        }
        if input.scope.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Scope is required");
        }
        if input.id.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Id is required");
        }
        if input.lock_token.is_empty() {
            return json_error_response(
                400,
                "WAFInvalidParameterException",
                "LockToken is required",
            );
        }

        let mut state = state.write().await;
        match state.delete_regex_pattern_set(
            &input.name,
            &input.scope,
            &input.id,
            &input.lock_token,
        ) {
            Ok(()) => wire::serialize_delete_regex_pattern_set_response(
                &wire::DeleteRegexPatternSetResponse {},
            ),
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_update_regex_pattern_set(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_regex_pattern_set_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Name is required");
        }
        if input.scope.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Scope is required");
        }
        if input.id.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Id is required");
        }
        if input.lock_token.is_empty() {
            return json_error_response(
                400,
                "WAFInvalidParameterException",
                "LockToken is required",
            );
        }
        let description = input.description.as_deref();
        let regular_expressions: Vec<String> = input
            .regular_expression_list
            .into_iter()
            .filter_map(|r| r.regex_string)
            .collect();

        let mut state = state.write().await;
        match state.update_regex_pattern_set(
            &input.name,
            &input.scope,
            &input.id,
            &input.lock_token,
            description,
            regular_expressions,
        ) {
            Ok(next_lock_token) => wire::serialize_update_regex_pattern_set_response(
                &wire::UpdateRegexPatternSetResponse {
                    next_lock_token: Some(next_lock_token),
                },
            ),
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_list_regex_pattern_sets(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_regex_pattern_sets_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.scope.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Scope is required");
        }

        let state = state.read().await;
        let sets = state.list_regex_pattern_sets(&input.scope);
        let entries: Vec<wire::RegexPatternSetSummary> = sets
            .iter()
            .map(|s| to_wire_regex_pattern_set_summary(s))
            .collect();

        wire::serialize_list_regex_pattern_sets_response(&wire::ListRegexPatternSetsResponse {
            regex_pattern_sets: Some(entries),
            next_marker: None,
        })
    }

    // ── RuleGroup handlers ──

    async fn handle_create_rule_group(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_rule_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Name is required");
        }
        if input.scope.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Scope is required");
        }
        let capacity = if input.capacity == 0 {
            100
        } else {
            input.capacity
        };
        let description = input.description.as_deref().unwrap_or("");
        let rules_json = input.rules.as_ref().map(to_value).unwrap_or(json!([]));
        let visibility_config_json = to_value(&input.visibility_config);
        let custom_response_bodies_json = to_value_opt(&input.custom_response_bodies);
        let tags = tags_from_wire(input.tags);

        let mut state = state.write().await;
        match state.create_rule_group(
            &input.name,
            &input.scope,
            description,
            capacity,
            rules_json,
            visibility_config_json,
            custom_response_bodies_json,
            region,
            account_id,
            tags,
        ) {
            Ok(s) => wire::serialize_create_rule_group_response(&wire::CreateRuleGroupResponse {
                summary: Some(to_wire_rule_group_summary(s)),
            }),
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_get_rule_group(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_rule_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        let name = match input.name.as_deref() {
            Some(s) if !s.is_empty() => s,
            _ => {
                return json_error_response(
                    400,
                    "WAFInvalidParameterException",
                    "Name is required",
                );
            }
        };
        let scope = match input.scope.as_deref() {
            Some(s) if !s.is_empty() => s,
            _ => {
                return json_error_response(
                    400,
                    "WAFInvalidParameterException",
                    "Scope is required",
                );
            }
        };
        let id = match input.id.as_deref() {
            Some(s) if !s.is_empty() => s,
            _ => return json_error_response(400, "WAFInvalidParameterException", "Id is required"),
        };

        let state = state.read().await;
        match state.get_rule_group(name, scope, id) {
            Ok(s) => wire::serialize_get_rule_group_response(&wire::GetRuleGroupResponse {
                rule_group: Some(to_wire_rule_group(s)),
                lock_token: Some(s.lock_token.clone()),
            }),
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_delete_rule_group(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_rule_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Name is required");
        }
        if input.scope.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Scope is required");
        }
        if input.id.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Id is required");
        }
        if input.lock_token.is_empty() {
            return json_error_response(
                400,
                "WAFInvalidParameterException",
                "LockToken is required",
            );
        }

        let mut state = state.write().await;
        match state.delete_rule_group(&input.name, &input.scope, &input.id, &input.lock_token) {
            Ok(()) => wire::serialize_delete_rule_group_response(&wire::DeleteRuleGroupResponse {}),
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_update_rule_group(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_rule_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Name is required");
        }
        if input.scope.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Scope is required");
        }
        if input.id.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Id is required");
        }
        if input.lock_token.is_empty() {
            return json_error_response(
                400,
                "WAFInvalidParameterException",
                "LockToken is required",
            );
        }
        let description = input.description.as_deref();
        let rules_json = input.rules.as_ref().map(to_value);
        let visibility_config_json = Some(to_value(&input.visibility_config));
        let custom_response_bodies_json = to_value_opt(&input.custom_response_bodies);

        let mut state = state.write().await;
        match state.update_rule_group(
            &input.name,
            &input.scope,
            &input.id,
            &input.lock_token,
            description,
            rules_json,
            visibility_config_json,
            custom_response_bodies_json,
        ) {
            Ok(next_lock_token) => {
                wire::serialize_update_rule_group_response(&wire::UpdateRuleGroupResponse {
                    next_lock_token: Some(next_lock_token),
                })
            }
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_list_rule_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_rule_groups_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.scope.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Scope is required");
        }

        let state = state.read().await;
        let groups = state.list_rule_groups(&input.scope);
        let entries: Vec<wire::RuleGroupSummary> = groups
            .iter()
            .map(|s| to_wire_rule_group_summary(s))
            .collect();

        wire::serialize_list_rule_groups_response(&wire::ListRuleGroupsResponse {
            rule_groups: Some(entries),
            next_marker: None,
        })
    }

    // ── LoggingConfiguration handlers ──

    async fn handle_put_logging_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_logging_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        let lc = input.logging_configuration;
        if lc.resource_arn.is_empty() {
            return json_error_response(
                400,
                "WAFInvalidParameterException",
                "ResourceArn is required",
            );
        }
        let log_destination_configs = lc.log_destination_configs;
        let logging_filter_json = to_value_opt(&lc.logging_filter);
        let redacted_fields_json = to_value_opt(&lc.redacted_fields);
        let log_scope = lc.log_scope;
        let log_type = lc.log_type;

        let mut state = state.write().await;
        match state.put_logging_configuration(
            &lc.resource_arn,
            log_destination_configs,
            logging_filter_json,
            redacted_fields_json,
            log_scope,
            log_type,
        ) {
            Ok(lc) => wire::serialize_put_logging_configuration_response(
                &wire::PutLoggingConfigurationResponse {
                    logging_configuration: Some(to_wire_logging_configuration(lc)),
                },
            ),
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_get_logging_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_logging_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(
                400,
                "WAFInvalidParameterException",
                "ResourceArn is required",
            );
        }

        let state = state.read().await;
        match state.get_logging_configuration(&input.resource_arn) {
            Ok(lc) => wire::serialize_get_logging_configuration_response(
                &wire::GetLoggingConfigurationResponse {
                    logging_configuration: Some(to_wire_logging_configuration(lc)),
                },
            ),
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_delete_logging_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_logging_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(
                400,
                "WAFInvalidParameterException",
                "ResourceArn is required",
            );
        }

        let mut state = state.write().await;
        match state.delete_logging_configuration(&input.resource_arn) {
            Ok(()) => wire::serialize_delete_logging_configuration_response(
                &wire::DeleteLoggingConfigurationResponse {},
            ),
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_list_logging_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_logging_configurations_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        let scope = if input.scope.is_empty() {
            "REGIONAL"
        } else {
            input.scope.as_str()
        };

        let state = state.read().await;
        let configs = state.list_logging_configurations(scope);
        let entries: Vec<wire::LoggingConfiguration> = configs
            .iter()
            .map(|lc| to_wire_logging_configuration(lc))
            .collect();

        wire::serialize_list_logging_configurations_response(
            &wire::ListLoggingConfigurationsResponse {
                logging_configurations: Some(entries),
                next_marker: None,
            },
        )
    }

    // ── Tag handlers ──

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Missing ResourceARN");
        }
        let tags = tags_from_wire(Some(input.tags));

        let mut state = state.write().await;
        match state.tag_resource(&input.resource_a_r_n, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Missing ResourceARN");
        }
        let tag_keys = input.tag_keys;

        let mut state = state.write().await;
        match state.untag_resource(&input.resource_a_r_n, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Missing ResourceARN");
        }

        let state = state.read().await;
        match state.list_tags_for_resource(&input.resource_a_r_n) {
            Ok(tags) => wire::serialize_list_tags_for_resource_response(
                &wire::ListTagsForResourceResponse {
                    tag_info_for_resource: Some(wire::TagInfoForResource {
                        resource_a_r_n: Some(input.resource_a_r_n.clone()),
                        tag_list: Some(to_wire_tags(&tags).await),
                    }),
                    next_marker: None,
                },
            ),
            Err(e) => wafv2_error_response(&e),
        }
    }

    // ── Permission Policy handlers ──

    async fn handle_put_permission_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_permission_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Missing ResourceArn");
        }
        if input.policy.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Missing Policy");
        }

        let mut state = state.write().await;
        match state.put_permission_policy(&input.resource_arn, &input.policy) {
            Ok(()) => wire::serialize_put_permission_policy_response(
                &wire::PutPermissionPolicyResponse {},
            ),
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_get_permission_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_permission_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Missing ResourceArn");
        }

        let state = state.read().await;
        match state.get_permission_policy(&input.resource_arn) {
            Ok(policy) => {
                wire::serialize_get_permission_policy_response(&wire::GetPermissionPolicyResponse {
                    policy: Some(policy.to_string()),
                })
            }
            Err(e) => wafv2_error_response(&e),
        }
    }

    async fn handle_delete_permission_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_permission_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Missing ResourceArn");
        }

        let mut state = state.write().await;
        match state.delete_permission_policy(&input.resource_arn) {
            Ok(()) => wire::serialize_delete_permission_policy_response(
                &wire::DeletePermissionPolicyResponse {},
            ),
            Err(e) => wafv2_error_response(&e),
        }
    }

    // ── API Key handlers ──

    async fn handle_create_api_key(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_a_p_i_key_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.scope.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Missing Scope");
        }
        let token_domains = input.token_domains;

        let mut state = state.write().await;
        let key_data = state.create_api_key(&input.scope, token_domains);
        wire::serialize_create_a_p_i_key_response(&wire::CreateAPIKeyResponse {
            a_p_i_key: Some(key_data.api_key.clone()),
        })
    }

    async fn handle_list_api_keys(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_a_p_i_keys_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.scope.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Missing Scope");
        }

        let state = state.read().await;
        let keys = state.list_api_keys(&input.scope);
        let summaries: Vec<wire::APIKeySummary> = keys
            .iter()
            .map(|k| wire::APIKeySummary {
                a_p_i_key: Some(k.api_key.clone()),
                token_domains: Some(k.token_domains.clone()),
                creation_timestamp: Some(k.creation_timestamp),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_a_p_i_keys_response(&wire::ListAPIKeysResponse {
            a_p_i_key_summaries: Some(summaries),
            next_marker: None,
            application_integration_u_r_l: None,
        })
    }

    async fn handle_delete_api_key(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_a_p_i_key_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.scope.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Missing Scope");
        }
        if input.a_p_i_key.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Missing APIKey");
        }

        let mut state = state.write().await;
        match state.delete_api_key(&input.scope, &input.a_p_i_key) {
            Ok(()) => wire::serialize_delete_a_p_i_key_response(&wire::DeleteAPIKeyResponse {}),
            Err(e) => wafv2_error_response(&e),
        }
    }

    // ── ListResourcesForWebACL handler ──

    async fn handle_list_resources_for_web_acl(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_resources_for_web_a_c_l_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.web_a_c_l_arn.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Missing WebACLArn");
        }

        let state = state.read().await;
        let resource_arns = state.list_resources_for_web_acl(&input.web_a_c_l_arn);

        wire::serialize_list_resources_for_web_a_c_l_response(
            &wire::ListResourcesForWebACLResponse {
                resource_arns: Some(resource_arns),
            },
        )
    }

    async fn handle_check_capacity(
        &self,
        state: &Arc<tokio::sync::RwLock<WafV2State>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_check_capacity_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "WAFInvalidParameterException", &e),
        };
        if input.scope.is_empty() {
            return json_error_response(400, "WAFInvalidParameterException", "Missing Scope");
        }
        let rules_value = serde_json::to_value(&input.rules).unwrap_or(json!([]));
        let rules_json = &rules_value;

        let parsed_rules = match winterbaume_wafv2_webacl_rule_parser::parse_rules(rules_json) {
            Ok(r) => r,
            Err(e) => {
                return json_error_response(
                    400,
                    "WAFInvalidParameterException",
                    &format!("Invalid rule definition: {e}"),
                );
            }
        };

        // Resolve customer-managed rule group ARNs to their capacity from state.
        let mut resolved_rule_groups = std::collections::HashMap::new();
        {
            let st = state.read().await;
            Self::collect_rule_group_arns(&parsed_rules, &mut resolved_rule_groups, &st);
        }

        let wcu = match winterbaume_wafv2_wcu_calculator::calculate_capacity(
            &parsed_rules,
            &resolved_rule_groups,
        ) {
            Ok(c) => c,
            Err(e) => {
                return json_error_response(
                    400,
                    "WAFInvalidParameterException",
                    &format!("Capacity calculation error: {e}"),
                );
            }
        };

        wire::serialize_check_capacity_response(&wire::CheckCapacityResponse {
            capacity: Some(wcu as i64),
        })
    }

    /// Walk parsed rules to find RuleGroupReference ARNs and resolve them
    /// against state.
    fn collect_rule_group_arns(
        rules: &[winterbaume_wafv2_webacl_rule_parser::Rule],
        resolved: &mut std::collections::HashMap<String, u64>,
        state: &WafV2State,
    ) {
        for rule in rules {
            Self::collect_statement_rule_group_arns(&rule.statement, resolved, state);
        }
    }

    fn collect_statement_rule_group_arns(
        stmt: &winterbaume_wafv2_webacl_rule_parser::Statement,
        resolved: &mut std::collections::HashMap<String, u64>,
        state: &WafV2State,
    ) {
        use winterbaume_wafv2_webacl_rule_parser::Statement;
        match stmt {
            Statement::RuleGroupReference { arn, .. } if !resolved.contains_key(arn) => {
                // Look up by ARN across all rule groups in state.
                if let Some(rg) = state.rule_groups.values().find(|rg| rg.arn == *arn) {
                    resolved.insert(arn.clone(), rg.capacity as u64);
                }
            }
            Statement::RuleGroupReference { .. } => {}

            Statement::And { statements } | Statement::Or { statements } => {
                for s in statements {
                    Self::collect_statement_rule_group_arns(s, resolved, state);
                }
            }
            Statement::Not { statement } => {
                Self::collect_statement_rule_group_arns(statement, resolved, state);
            }
            Statement::RateBased {
                scope_down_statement,
                ..
            }
            | Statement::ManagedRuleGroup {
                scope_down_statement,
                ..
            } => {
                if let Some(s) = scope_down_statement {
                    Self::collect_statement_rule_group_arns(s, resolved, state);
                }
            }
            _ => {}
        }
    }
}
