//! Serde-compatible view types for WAFv2 state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::WafV2Service;
use crate::state::WafV2State;
use crate::types::{ApiKeyData, IpSet, LoggingConfigData, RegexPatternSet, RuleGroupData, WebAcl};

/// Serializable view of the entire WAFv2 state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Wafv2StateView {
    /// Web ACLs keyed by "{scope}:{name}".
    #[serde(default)]
    pub web_acls: HashMap<String, WebAclView>,
    /// Rule groups keyed by "{scope}:{name}".
    #[serde(default)]
    pub rule_groups: HashMap<String, RuleGroupView>,
    /// IP sets keyed by "{scope}:{name}".
    #[serde(default)]
    pub ip_sets: HashMap<String, IpSetView>,
    /// Regex pattern sets keyed by "{scope}:{name}".
    #[serde(default)]
    pub regex_pattern_sets: HashMap<String, RegexPatternSetView>,
    /// Logging configurations keyed by resource ARN.
    #[serde(default)]
    pub logging_configs: HashMap<String, LoggingConfigView>,
    /// Web ACL associations: resource_arn -> web_acl_arn.
    #[serde(default)]
    pub web_acl_associations: HashMap<String, String>,
    /// Permission policies: resource_arn -> policy JSON.
    #[serde(default)]
    pub permission_policies: HashMap<String, String>,
    /// API keys keyed by "{scope}:{api_key}".
    #[serde(default)]
    pub api_keys: HashMap<String, ApiKeyView>,
}

/// Serializable view of a single Web ACL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAclView {
    pub name: String,
    pub id: String,
    pub arn: String,
    pub scope: String,
    pub description: String,
    pub lock_token: String,
    pub default_action_json: serde_json::Value,
    pub visibility_config_json: serde_json::Value,
    pub rules_json: serde_json::Value,
    pub association_config_json: Option<serde_json::Value>,
    pub custom_response_bodies_json: Option<serde_json::Value>,
    pub captcha_config_json: Option<serde_json::Value>,
    pub challenge_config_json: Option<serde_json::Value>,
    pub token_domains: Option<Vec<String>>,
    pub label_namespace: String,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
}

/// Serializable view of a single rule group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleGroupView {
    pub name: String,
    pub id: String,
    pub arn: String,
    pub scope: String,
    pub description: String,
    pub lock_token: String,
    pub capacity: i64,
    pub rules_json: serde_json::Value,
    pub visibility_config_json: serde_json::Value,
    pub custom_response_bodies_json: Option<serde_json::Value>,
    pub label_namespace: String,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
}

/// Serializable view of a single IP set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpSetView {
    pub name: String,
    pub id: String,
    pub arn: String,
    pub scope: String,
    pub description: String,
    pub lock_token: String,
    pub ip_address_version: String,
    #[serde(default)]
    pub addresses: Vec<String>,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
}

/// Serializable view of a single regex pattern set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegexPatternSetView {
    pub name: String,
    pub id: String,
    pub arn: String,
    pub scope: String,
    pub description: String,
    pub lock_token: String,
    #[serde(default)]
    pub regular_expressions: Vec<String>,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
}

/// Serializable view of a single API key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyView {
    pub api_key: String,
    pub scope: String,
    #[serde(default)]
    pub token_domains: Vec<String>,
    pub creation_timestamp: f64,
}

/// Serializable view of a single logging configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfigView {
    pub resource_arn: String,
    #[serde(default)]
    pub log_destination_configs: Vec<String>,
    pub logging_filter_json: Option<serde_json::Value>,
    pub redacted_fields_json: Option<serde_json::Value>,
    pub log_scope: Option<String>,
    pub log_type: Option<String>,
}

// ---------------------------------------------------------------------------
// From conversions
// ---------------------------------------------------------------------------

impl From<&WebAcl> for WebAclView {
    fn from(a: &WebAcl) -> Self {
        Self {
            name: a.name.clone(),
            id: a.id.clone(),
            arn: a.arn.clone(),
            scope: a.scope.clone(),
            description: a.description.clone(),
            lock_token: a.lock_token.clone(),
            default_action_json: a.default_action_json.clone(),
            visibility_config_json: a.visibility_config_json.clone(),
            rules_json: a.rules_json.clone(),
            association_config_json: a.association_config_json.clone(),
            custom_response_bodies_json: a.custom_response_bodies_json.clone(),
            captcha_config_json: a.captcha_config_json.clone(),
            challenge_config_json: a.challenge_config_json.clone(),
            token_domains: a.token_domains.clone(),
            label_namespace: a.label_namespace.clone(),
            tags: a.tags.clone(),
        }
    }
}

impl From<WebAclView> for WebAcl {
    fn from(v: WebAclView) -> Self {
        Self {
            name: v.name,
            id: v.id,
            arn: v.arn,
            scope: v.scope,
            description: v.description,
            lock_token: v.lock_token,
            default_action_json: v.default_action_json,
            visibility_config_json: v.visibility_config_json,
            rules_json: v.rules_json,
            association_config_json: v.association_config_json,
            custom_response_bodies_json: v.custom_response_bodies_json,
            captcha_config_json: v.captcha_config_json,
            challenge_config_json: v.challenge_config_json,
            token_domains: v.token_domains,
            label_namespace: v.label_namespace,
            tags: v.tags,
        }
    }
}

impl From<&RuleGroupData> for RuleGroupView {
    fn from(r: &RuleGroupData) -> Self {
        Self {
            name: r.name.clone(),
            id: r.id.clone(),
            arn: r.arn.clone(),
            scope: r.scope.clone(),
            description: r.description.clone(),
            lock_token: r.lock_token.clone(),
            capacity: r.capacity,
            rules_json: r.rules_json.clone(),
            visibility_config_json: r.visibility_config_json.clone(),
            custom_response_bodies_json: r.custom_response_bodies_json.clone(),
            label_namespace: r.label_namespace.clone(),
            tags: r.tags.clone(),
        }
    }
}

impl From<RuleGroupView> for RuleGroupData {
    fn from(v: RuleGroupView) -> Self {
        Self {
            name: v.name,
            id: v.id,
            arn: v.arn,
            scope: v.scope,
            description: v.description,
            lock_token: v.lock_token,
            capacity: v.capacity,
            rules_json: v.rules_json,
            visibility_config_json: v.visibility_config_json,
            custom_response_bodies_json: v.custom_response_bodies_json,
            label_namespace: v.label_namespace,
            tags: v.tags,
        }
    }
}

impl From<&IpSet> for IpSetView {
    fn from(s: &IpSet) -> Self {
        Self {
            name: s.name.clone(),
            id: s.id.clone(),
            arn: s.arn.clone(),
            scope: s.scope.clone(),
            description: s.description.clone(),
            lock_token: s.lock_token.clone(),
            ip_address_version: s.ip_address_version.clone(),
            addresses: s.addresses.clone(),
            tags: s.tags.clone(),
        }
    }
}

impl From<IpSetView> for IpSet {
    fn from(v: IpSetView) -> Self {
        Self {
            name: v.name,
            id: v.id,
            arn: v.arn,
            scope: v.scope,
            description: v.description,
            lock_token: v.lock_token,
            ip_address_version: v.ip_address_version,
            addresses: v.addresses,
            tags: v.tags,
        }
    }
}

impl From<&RegexPatternSet> for RegexPatternSetView {
    fn from(s: &RegexPatternSet) -> Self {
        Self {
            name: s.name.clone(),
            id: s.id.clone(),
            arn: s.arn.clone(),
            scope: s.scope.clone(),
            description: s.description.clone(),
            lock_token: s.lock_token.clone(),
            regular_expressions: s.regular_expressions.clone(),
            tags: s.tags.clone(),
        }
    }
}

impl From<RegexPatternSetView> for RegexPatternSet {
    fn from(v: RegexPatternSetView) -> Self {
        Self {
            name: v.name,
            id: v.id,
            arn: v.arn,
            scope: v.scope,
            description: v.description,
            lock_token: v.lock_token,
            regular_expressions: v.regular_expressions,
            tags: v.tags,
        }
    }
}

impl From<&ApiKeyData> for ApiKeyView {
    fn from(k: &ApiKeyData) -> Self {
        Self {
            api_key: k.api_key.clone(),
            scope: k.scope.clone(),
            token_domains: k.token_domains.clone(),
            creation_timestamp: k.creation_timestamp,
        }
    }
}

impl From<ApiKeyView> for ApiKeyData {
    fn from(v: ApiKeyView) -> Self {
        Self {
            api_key: v.api_key,
            scope: v.scope,
            token_domains: v.token_domains,
            creation_timestamp: v.creation_timestamp,
        }
    }
}

impl From<&LoggingConfigData> for LoggingConfigView {
    fn from(lc: &LoggingConfigData) -> Self {
        Self {
            resource_arn: lc.resource_arn.clone(),
            log_destination_configs: lc.log_destination_configs.clone(),
            logging_filter_json: lc.logging_filter_json.clone(),
            redacted_fields_json: lc.redacted_fields_json.clone(),
            log_scope: lc.log_scope.clone(),
            log_type: lc.log_type.clone(),
        }
    }
}

impl From<LoggingConfigView> for LoggingConfigData {
    fn from(v: LoggingConfigView) -> Self {
        Self {
            resource_arn: v.resource_arn,
            log_destination_configs: v.log_destination_configs,
            logging_filter_json: v.logging_filter_json,
            redacted_fields_json: v.redacted_fields_json,
            log_scope: v.log_scope,
            log_type: v.log_type,
        }
    }
}

impl From<&WafV2State> for Wafv2StateView {
    fn from(s: &WafV2State) -> Self {
        Self {
            web_acls: s
                .web_acls
                .iter()
                .map(|(k, v)| (k.clone(), WebAclView::from(v)))
                .collect(),
            rule_groups: s
                .rule_groups
                .iter()
                .map(|(k, v)| (k.clone(), RuleGroupView::from(v)))
                .collect(),
            ip_sets: s
                .ip_sets
                .iter()
                .map(|(k, v)| (k.clone(), IpSetView::from(v)))
                .collect(),
            regex_pattern_sets: s
                .regex_pattern_sets
                .iter()
                .map(|(k, v)| (k.clone(), RegexPatternSetView::from(v)))
                .collect(),
            logging_configs: s
                .logging_configs
                .iter()
                .map(|(k, v)| (k.clone(), LoggingConfigView::from(v)))
                .collect(),
            web_acl_associations: s.web_acl_associations.clone(),
            permission_policies: s.permission_policies.clone(),
            api_keys: s
                .api_keys
                .iter()
                .map(|(k, v)| (k.clone(), ApiKeyView::from(v)))
                .collect(),
        }
    }
}

impl From<Wafv2StateView> for WafV2State {
    fn from(v: Wafv2StateView) -> Self {
        Self {
            web_acls: v
                .web_acls
                .into_iter()
                .map(|(k, vv)| (k, WebAcl::from(vv)))
                .collect(),
            rule_groups: v
                .rule_groups
                .into_iter()
                .map(|(k, vv)| (k, RuleGroupData::from(vv)))
                .collect(),
            ip_sets: v
                .ip_sets
                .into_iter()
                .map(|(k, vv)| (k, IpSet::from(vv)))
                .collect(),
            regex_pattern_sets: v
                .regex_pattern_sets
                .into_iter()
                .map(|(k, vv)| (k, RegexPatternSet::from(vv)))
                .collect(),
            logging_configs: v
                .logging_configs
                .into_iter()
                .map(|(k, vv)| (k, LoggingConfigData::from(vv)))
                .collect(),
            web_acl_associations: v.web_acl_associations,
            permission_policies: v.permission_policies,
            api_keys: v
                .api_keys
                .into_iter()
                .map(|(k, vv)| (k, ApiKeyData::from(vv)))
                .collect(),
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for WafV2Service {
    type StateView = Wafv2StateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        Wafv2StateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let new_state = WafV2State::from(view);
        {
            let state = self.state.get(account_id, region);
            *state.write().await = new_state;
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            let incoming = WafV2State::from(view);
            for (k, v) in incoming.web_acls {
                guard.web_acls.insert(k, v);
            }
            for (k, v) in incoming.rule_groups {
                guard.rule_groups.insert(k, v);
            }
            for (k, v) in incoming.ip_sets {
                guard.ip_sets.insert(k, v);
            }
            for (k, v) in incoming.regex_pattern_sets {
                guard.regex_pattern_sets.insert(k, v);
            }
            for (k, v) in incoming.logging_configs {
                guard.logging_configs.insert(k, v);
            }
            for (k, v) in incoming.web_acl_associations {
                guard.web_acl_associations.insert(k, v);
            }
            for (k, v) in incoming.permission_policies {
                guard.permission_policies.insert(k, v);
            }
            for (k, v) in incoming.api_keys {
                guard.api_keys.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
