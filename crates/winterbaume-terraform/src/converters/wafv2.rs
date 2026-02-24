//! Terraform converters for WAFv2 resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;
use winterbaume_wafv2::WafV2Service;
use winterbaume_wafv2::views::{IpSetView, RuleGroupView, Wafv2StateView, WebAclView};

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_wafv2_web_acl
// ---------------------------------------------------------------------------

pub struct AwsWafv2WebAclConverter {
    service: Arc<WafV2Service>,
}

impl AwsWafv2WebAclConverter {
    pub fn new(service: Arc<WafV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsWafv2WebAclConverter {
    fn resource_type(&self) -> &str {
        "aws_wafv2_web_acl"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsWafv2WebAclConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_wafv2_web_acl")?;
        let scope = optional_str(attrs, "scope").unwrap_or_else(|| "REGIONAL".to_string());
        let key = format!("{}:{}", scope, name);

        let id = optional_str(attrs, "id").unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:wafv2:{}:{}:{}/webacl/{}/{}",
                region,
                ctx.default_account_id,
                scope.to_lowercase(),
                name,
                id
            )
        });
        let description = optional_str(attrs, "description").unwrap_or_default();

        let default_action_json = attrs
            .get("default_action")
            .cloned()
            .unwrap_or(serde_json::json!({"allow": {}}));
        let rules_json = attrs
            .get("rule")
            .cloned()
            .unwrap_or(serde_json::Value::Array(vec![]));
        let visibility_config_json = attrs
            .get("visibility_config")
            .cloned()
            .unwrap_or(serde_json::json!({}));

        let _tags_all = attrs.get("tags_all");
        let token_domains: Option<Vec<String>> = attrs
            .get("token_domains")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            });
        let association_config_json = attrs.get("association_config").cloned();
        let challenge_config_json = attrs.get("challenge_config").cloned();
        let captcha_config_json = attrs.get("captcha_config").cloned();
        let custom_response_bodies_json = attrs.get("custom_response_body").cloned();
        let data_protection_config = attrs.get("data_protection_config").cloned();
        let _ = data_protection_config; // stored in rules_json blob; no separate view field yet

        let tags: Vec<(String, String)> = attrs
            .get("tags")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let acl_view = WebAclView {
            name: name.to_string(),
            id,
            arn,
            scope: scope.clone(),
            description,
            lock_token: uuid::Uuid::new_v4().to_string(),
            default_action_json,
            visibility_config_json,
            rules_json,
            association_config_json,
            custom_response_bodies_json,
            captcha_config_json,
            challenge_config_json,
            token_domains,
            label_namespace: format!("awswaf:{}:webacl:{}:", ctx.default_account_id, name),
            tags,
        };

        let mut state_view = minimal_wafv2_state_view();
        state_view.web_acls.insert(key, acl_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for acl in view.web_acls.values() {
            let tags: HashMap<String, String> = acl
                .tags
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            let attrs = serde_json::json!({
                "id": acl.id,
                "name": acl.name,
                "arn": acl.arn,
                "scope": acl.scope,
                "description": acl.description,
                "tags": tags,
                "tags_all": tags,
                "capacity": 0,
                "lock_token": acl.lock_token,
                "application_integration_url": null,
                "default_action": acl.default_action_json,
                "visibility_config": acl.visibility_config_json,
                "rule": acl.rules_json,
                "captcha_config": acl.captcha_config_json,
                "challenge_config": acl.challenge_config_json,
                "association_config": acl.association_config_json,
                "custom_response_body": acl.custom_response_bodies_json,
                "token_domains": acl.token_domains,
            });
            results.push(ExtractedResource {
                name: acl.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_wafv2_ip_set
// ---------------------------------------------------------------------------

pub struct AwsWafv2IpSetConverter {
    service: Arc<WafV2Service>,
}

impl AwsWafv2IpSetConverter {
    pub fn new(service: Arc<WafV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsWafv2IpSetConverter {
    fn resource_type(&self) -> &str {
        "aws_wafv2_ip_set"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsWafv2IpSetConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_wafv2_ip_set")?;
        let scope = optional_str(attrs, "scope").unwrap_or_else(|| "REGIONAL".to_string());
        let key = format!("{}:{}", scope, name);

        let id = optional_str(attrs, "id").unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:wafv2:{}:{}:{}/ipset/{}/{}",
                region,
                ctx.default_account_id,
                scope.to_lowercase(),
                name,
                id
            )
        });
        let description = optional_str(attrs, "description").unwrap_or_default();
        let ip_address_version =
            optional_str(attrs, "ip_address_version").unwrap_or_else(|| "IPV4".to_string());

        let addresses: Vec<String> = attrs
            .get("addresses")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let tags: Vec<(String, String)> = attrs
            .get("tags")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let ip_set_view = IpSetView {
            name: name.to_string(),
            id,
            arn,
            scope: scope.clone(),
            description,
            lock_token: uuid::Uuid::new_v4().to_string(),
            ip_address_version,
            addresses,
            tags,
        };

        let mut state_view = minimal_wafv2_state_view();
        state_view.ip_sets.insert(key, ip_set_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for ip_set in view.ip_sets.values() {
            let tags: HashMap<String, String> = ip_set
                .tags
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            let attrs = serde_json::json!({
                "id": ip_set.id,
                "name": ip_set.name,
                "arn": ip_set.arn,
                "scope": ip_set.scope,
                "ip_address_version": ip_set.ip_address_version,
                "addresses": ip_set.addresses,
                "tags": tags,
            });
            results.push(ExtractedResource {
                name: ip_set.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_wafv2_rule_group
// ---------------------------------------------------------------------------

pub struct AwsWafv2RuleGroupConverter {
    service: Arc<WafV2Service>,
}

impl AwsWafv2RuleGroupConverter {
    pub fn new(service: Arc<WafV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsWafv2RuleGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_wafv2_rule_group"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsWafv2RuleGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_wafv2_rule_group")?;
        let scope = optional_str(attrs, "scope").unwrap_or_else(|| "REGIONAL".to_string());
        let key = format!("{}:{}", scope, name);

        let id = optional_str(attrs, "id").unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:wafv2:{}:{}:{}/rulegroup/{}/{}",
                region,
                ctx.default_account_id,
                scope.to_lowercase(),
                name,
                id
            )
        });
        let description = optional_str(attrs, "description").unwrap_or_default();
        let capacity = attrs
            .get("capacity")
            .and_then(|v| v.as_i64())
            .unwrap_or(100);
        let rules_json = attrs
            .get("rule")
            .cloned()
            .unwrap_or(serde_json::Value::Array(vec![]));
        let visibility_config_json = attrs
            .get("visibility_config")
            .cloned()
            .unwrap_or(serde_json::json!({}));
        let custom_response_bodies_json = attrs.get("custom_response_body").cloned();

        let tags: Vec<(String, String)> = attrs
            .get("tags")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let rg_view = RuleGroupView {
            name: name.to_string(),
            id,
            arn,
            scope: scope.clone(),
            description,
            lock_token: uuid::Uuid::new_v4().to_string(),
            capacity,
            rules_json,
            visibility_config_json,
            custom_response_bodies_json,
            label_namespace: format!("awswaf:{}:rulegroup:{}:", ctx.default_account_id, name),
            tags,
        };

        let mut state_view = minimal_wafv2_state_view();
        state_view.rule_groups.insert(key, rg_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for rg in view.rule_groups.values() {
            let tags: HashMap<String, String> = rg
                .tags
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            let attrs = serde_json::json!({
                "id": rg.id,
                "name": rg.name,
                "arn": rg.arn,
                "scope": rg.scope,
                "capacity": rg.capacity,
                "description": rg.description,
                "tags": tags,
                "tags_all": tags,
                "lock_token": rg.lock_token,
                "rule": rg.rules_json,
                "visibility_config": rg.visibility_config_json,
                "custom_response_body": rg.custom_response_bodies_json,
            });
            results.push(ExtractedResource {
                name: rg.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn minimal_wafv2_state_view() -> Wafv2StateView {
    Wafv2StateView::default()
}
