//! Terraform converters for WAFv2 resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;
use winterbaume_wafv2::WafV2Service;
use winterbaume_wafv2::views::{
    ApiKeyView, IpSetView, LoggingConfigView, RegexPatternSetView, RuleGroupView, Wafv2StateView,
    WebAclView,
};

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::wafv2 as wafv2_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let model: wafv2_gen::Wafv2WebAclTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_wafv2_web_acl", e))?;

        let name = model.name.clone();
        let scope = model.scope.unwrap_or_else(|| "REGIONAL".to_string());
        let key = format!("{}:{}", scope, name);

        let id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:wafv2:{}:{}:{}/webacl/{}/{}",
                region,
                ctx.default_account_id,
                scope.to_lowercase(),
                name,
                id
            )
        });
        let description = model.description.unwrap_or_default();

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
            name: name.clone(),
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
        let model: wafv2_gen::Wafv2IpSetTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_wafv2_ip_set", e))?;

        let name = model.name.clone();
        let scope = model.scope.unwrap_or_else(|| "REGIONAL".to_string());
        let key = format!("{}:{}", scope, name);

        let id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:wafv2:{}:{}:{}/ipset/{}/{}",
                region,
                ctx.default_account_id,
                scope.to_lowercase(),
                name,
                id
            )
        });
        let description = model.description.unwrap_or_default();
        let ip_address_version = model
            .ip_address_version
            .unwrap_or_else(|| "IPV4".to_string());

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
            name,
            id,
            arn,
            scope,
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
        let model: wafv2_gen::Wafv2RuleGroupTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_wafv2_rule_group", e))?;

        let name = model.name.clone();
        let scope = model.scope.unwrap_or_else(|| "REGIONAL".to_string());
        let key = format!("{}:{}", scope, name);

        let id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:wafv2:{}:{}:{}/rulegroup/{}/{}",
                region,
                ctx.default_account_id,
                scope.to_lowercase(),
                name,
                id
            )
        });
        let description = model.description.unwrap_or_default();
        let capacity = model.capacity;
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
            name: name.clone(),
            id,
            arn,
            scope,
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
// aws_wafv2_regex_pattern_set
// ---------------------------------------------------------------------------

pub struct AwsWafv2RegexPatternSetConverter {
    service: Arc<WafV2Service>,
}

impl AwsWafv2RegexPatternSetConverter {
    pub fn new(service: Arc<WafV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsWafv2RegexPatternSetConverter {
    fn resource_type(&self) -> &str {
        "aws_wafv2_regex_pattern_set"
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

impl AwsWafv2RegexPatternSetConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: wafv2_gen::Wafv2RegexPatternSetTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_wafv2_regex_pattern_set", e))?;

        let name = model.name.clone();
        let scope = model.scope.unwrap_or_else(|| "REGIONAL".to_string());
        let key = format!("{}:{}", scope, name);

        let id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:wafv2:{}:{}:{}/regexpatternset/{}/{}",
                region,
                ctx.default_account_id,
                scope.to_lowercase(),
                name,
                id
            )
        });

        let regular_expressions: Vec<String> = attrs
            .get("regular_expression")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|e| {
                        e.get("regex_string")
                            .and_then(|s| s.as_str().map(|s| s.to_string()))
                    })
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

        let view = RegexPatternSetView {
            name: name.clone(),
            id,
            arn,
            scope,
            description: model.description.unwrap_or_default(),
            lock_token: uuid::Uuid::new_v4().to_string(),
            regular_expressions,
            tags,
        };

        let mut state_view = minimal_wafv2_state_view();
        state_view.regex_pattern_sets.insert(key, view);
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
        for rps in view.regex_pattern_sets.values() {
            let tags: serde_json::Map<String, serde_json::Value> = rps
                .tags
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect();
            let attrs = serde_json::json!({
                "id": rps.id,
                "name": rps.name,
                "arn": rps.arn,
                "scope": rps.scope,
                "description": rps.description,
                "lock_token": rps.lock_token,
                "tags": tags,
                "tags_all": tags,
                "regular_expression": rps
                    .regular_expressions
                    .iter()
                    .map(|e| serde_json::json!({"regex_string": e}))
                    .collect::<Vec<_>>(),
            });
            results.push(ExtractedResource {
                name: rps.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_wafv2_api_key
// ---------------------------------------------------------------------------

pub struct AwsWafv2ApiKeyConverter {
    service: Arc<WafV2Service>,
}

impl AwsWafv2ApiKeyConverter {
    pub fn new(service: Arc<WafV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsWafv2ApiKeyConverter {
    fn resource_type(&self) -> &str {
        "aws_wafv2_api_key"
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

impl AwsWafv2ApiKeyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: wafv2_gen::Wafv2ApiKeyTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_wafv2_api_key", e))?;

        let api_key = model
            .api_key
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let scope = model.scope;
        let key = format!("{}:{}", scope, api_key);

        let token_domains: Vec<String> = attrs
            .get("token_domains")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let view = ApiKeyView {
            api_key: api_key.clone(),
            scope,
            token_domains,
            creation_timestamp: 0.0,
        };

        let mut state_view = minimal_wafv2_state_view();
        state_view.api_keys.insert(key, view);
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
        for k in view.api_keys.values() {
            let attrs = serde_json::json!({
                "id": k.api_key,
                "api_key": k.api_key,
                "scope": k.scope,
                "token_domains": k.token_domains,
            });
            results.push(ExtractedResource {
                name: k.api_key.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_wafv2_web_acl_association
// ---------------------------------------------------------------------------

pub struct AwsWafv2WebAclAssociationConverter {
    service: Arc<WafV2Service>,
}

impl AwsWafv2WebAclAssociationConverter {
    pub fn new(service: Arc<WafV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsWafv2WebAclAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_wafv2_web_acl_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_wafv2_web_acl"]
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

impl AwsWafv2WebAclAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: wafv2_gen::Wafv2WebAclAssociationTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_wafv2_web_acl_association", e))?;

        let mut state_view = minimal_wafv2_state_view();
        state_view
            .web_acl_associations
            .insert(model.resource_arn, model.web_acl_arn);
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
        for (resource_arn, web_acl_arn) in &view.web_acl_associations {
            let id = format!("{},{}", web_acl_arn, resource_arn);
            let attrs = serde_json::json!({
                "id": id,
                "web_acl_arn": web_acl_arn,
                "resource_arn": resource_arn,
            });
            results.push(ExtractedResource {
                name: id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_wafv2_web_acl_logging_configuration
// ---------------------------------------------------------------------------

pub struct AwsWafv2WebAclLoggingConfigurationConverter {
    service: Arc<WafV2Service>,
}

impl AwsWafv2WebAclLoggingConfigurationConverter {
    pub fn new(service: Arc<WafV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsWafv2WebAclLoggingConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_wafv2_web_acl_logging_configuration"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_wafv2_web_acl"]
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

impl AwsWafv2WebAclLoggingConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: wafv2_gen::Wafv2WebAclLoggingConfigurationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_wafv2_web_acl_logging_configuration", e)
            })?;

        let log_destination_configs: Vec<String> = attrs
            .get("log_destination_configs")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let view = LoggingConfigView {
            resource_arn: model.resource_arn.clone(),
            log_destination_configs,
            logging_filter_json: attrs.get("logging_filter").cloned(),
            redacted_fields_json: attrs.get("redacted_fields").cloned(),
            log_scope: model.log_scope,
            log_type: model.log_type,
        };

        let mut state_view = minimal_wafv2_state_view();
        state_view.logging_configs.insert(model.resource_arn, view);
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
        for lc in view.logging_configs.values() {
            let attrs = serde_json::json!({
                "id": lc.resource_arn,
                "resource_arn": lc.resource_arn,
                "log_destination_configs": lc.log_destination_configs,
                "logging_filter": lc.logging_filter_json,
                "redacted_fields": lc.redacted_fields_json,
                "log_scope": lc.log_scope,
                "log_type": lc.log_type,
            });
            results.push(ExtractedResource {
                name: lc.resource_arn.clone(),
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
