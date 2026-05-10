//! Terraform converter for Network Firewall resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_networkfirewall::NetworkFirewallService;
use winterbaume_networkfirewall::views::{
    FirewallPolicyView, FirewallView, NetworkFirewallStateView, RuleGroupView, SubnetMappingView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::networkfirewall as networkfirewall_gen;
use crate::util::{classify_deserialize_error, extract_region};

/// Extract tags as `Vec<(String, String)>` from Terraform attributes.
fn extract_tags_vec(attrs: &serde_json::Value) -> Vec<(String, String)> {
    let mut tags = HashMap::new();
    if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
        for (k, v) in obj {
            if let Some(s) = v.as_str() {
                tags.insert(k.clone(), s.to_string());
            }
        }
    }
    if let Some(obj) = attrs.get("tags").and_then(|v| v.as_object()) {
        for (k, v) in obj {
            if let Some(s) = v.as_str() {
                tags.insert(k.clone(), s.to_string());
            }
        }
    }
    tags.into_iter().collect()
}

/// Parse `encryption_configuration` TF JSON array block into a normalised JSON value.
/// Returns `None` when no block (or an empty array) is present.
fn parse_encryption_config(attrs: &serde_json::Value) -> Option<serde_json::Value> {
    attrs
        .get("encryption_configuration")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .map(|b| {
            serde_json::json!({
                "key_id": b.get("key_id").and_then(|v| v.as_str()).unwrap_or(""),
                "type": b.get("type").and_then(|v| v.as_str()).unwrap_or("AWS_OWNED_KMS_KEY"),
            })
        })
}

/// Emit an `encryption_configuration` block as a TF JSON array (zero or one element).
fn emit_encryption_config(enc: &Option<serde_json::Value>) -> Vec<serde_json::Value> {
    enc.as_ref().map(|v| vec![v.clone()]).unwrap_or_default()
}

// ---------------------------------------------------------------------------
// aws_networkfirewall_firewall
// ---------------------------------------------------------------------------

/// Converts `aws_networkfirewall_firewall` Terraform resources.
pub struct AwsNetworkFirewallFirewallConverter {
    service: Arc<NetworkFirewallService>,
}

impl AwsNetworkFirewallFirewallConverter {
    pub fn new(service: Arc<NetworkFirewallService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkFirewallFirewallConverter {
    fn resource_type(&self) -> &str {
        "aws_networkfirewall_firewall"
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

impl AwsNetworkFirewallFirewallConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: networkfirewall_gen::FirewallTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_networkfirewall_firewall", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let firewall_id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let firewall_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:network-firewall:{}:{}:firewall/{}",
                region, ctx.default_account_id, name
            )
        });
        let firewall_policy_arn = model.firewall_policy_arn.unwrap_or_default();
        let vpc_id = model.vpc_id.unwrap_or_default();
        let _firewall_status = attrs.get("firewall_status");

        // Parse subnet_mapping
        let subnet_mappings: Vec<SubnetMappingView> = attrs
            .get("subnet_mapping")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|sm| {
                        sm.get("subnet_id")
                            .and_then(|v| v.as_str())
                            .map(|s| SubnetMappingView {
                                subnet_id: s.to_string(),
                            })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let fw_view = FirewallView {
            firewall_name: name,
            firewall_arn: firewall_arn.clone(),
            firewall_id,
            firewall_policy_arn,
            vpc_id,
            subnet_mappings,
            delete_protection: model.delete_protection,
            subnet_change_protection: model.subnet_change_protection,
            firewall_policy_change_protection: model.firewall_policy_change_protection,
            availability_zone_change_protection: false,
            description: model.description,
            tags: extract_tags_vec(attrs),
            encryption_configuration: parse_encryption_config(attrs),
        };

        let mut state_view = NetworkFirewallStateView::default();
        state_view.firewalls.insert(firewall_arn, fw_view);
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
        for fw in view.firewalls.values() {
            let tags: HashMap<String, String> = fw
                .tags
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            let tags_all = tags.clone();
            let subnet_mappings: Vec<serde_json::Value> = fw
                .subnet_mappings
                .iter()
                .map(|sm| serde_json::json!({"subnet_id": sm.subnet_id}))
                .collect();
            let attrs = serde_json::json!({
                "id": fw.firewall_id,
                "arn": fw.firewall_arn,
                "name": fw.firewall_name,
                "firewall_policy_arn": fw.firewall_policy_arn,
                "vpc_id": fw.vpc_id,
                "subnet_mapping": subnet_mappings,
                "delete_protection": fw.delete_protection,
                "subnet_change_protection": fw.subnet_change_protection,
                "firewall_policy_change_protection": fw.firewall_policy_change_protection,
                "description": fw.description,
                "encryption_configuration": emit_encryption_config(&fw.encryption_configuration),
                "tags": tags,
                "tags_all": tags_all,
            });
            results.push(ExtractedResource {
                name: fw.firewall_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_networkfirewall_firewall_policy
// ---------------------------------------------------------------------------

/// Converts `aws_networkfirewall_firewall_policy` Terraform resources.
pub struct AwsNetworkFirewallFirewallPolicyConverter {
    service: Arc<NetworkFirewallService>,
}

impl AwsNetworkFirewallFirewallPolicyConverter {
    pub fn new(service: Arc<NetworkFirewallService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkFirewallFirewallPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_networkfirewall_firewall_policy"
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

impl AwsNetworkFirewallFirewallPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: networkfirewall_gen::FirewallPolicyTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_networkfirewall_firewall_policy", e)
            })?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let policy_id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let policy_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:network-firewall:{}:{}:firewall-policy/{}",
                region, ctx.default_account_id, name
            )
        });

        // The firewall_policy block is typically a JSON structure
        let firewall_policy_body = attrs
            .get("firewall_policy")
            .cloned()
            .unwrap_or(serde_json::json!({}));

        let fp_view = FirewallPolicyView {
            firewall_policy_name: name,
            firewall_policy_arn: policy_arn.clone(),
            firewall_policy_id: policy_id,
            description: model.description,
            tags: extract_tags_vec(attrs),
            firewall_policy_body,
            encryption_configuration: parse_encryption_config(attrs),
        };

        let mut state_view = NetworkFirewallStateView::default();
        state_view.firewall_policies.insert(policy_arn, fp_view);
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
        for fp in view.firewall_policies.values() {
            let tags: HashMap<String, String> = fp
                .tags
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            let tags_all = tags.clone();
            let attrs = serde_json::json!({
                "id": fp.firewall_policy_id,
                "arn": fp.firewall_policy_arn,
                "name": fp.firewall_policy_name,
                "description": fp.description,
                "firewall_policy": fp.firewall_policy_body,
                "encryption_configuration": emit_encryption_config(&fp.encryption_configuration),
                "tags": tags,
                "tags_all": tags_all,
            });
            results.push(ExtractedResource {
                name: fp.firewall_policy_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_networkfirewall_rule_group
// ---------------------------------------------------------------------------

/// Converts `aws_networkfirewall_rule_group` Terraform resources.
pub struct AwsNetworkFirewallRuleGroupConverter {
    service: Arc<NetworkFirewallService>,
}

impl AwsNetworkFirewallRuleGroupConverter {
    pub fn new(service: Arc<NetworkFirewallService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkFirewallRuleGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_networkfirewall_rule_group"
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

impl AwsNetworkFirewallRuleGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: networkfirewall_gen::RuleGroupTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_networkfirewall_rule_group", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let rule_group_type = model.rule_group_type.clone();
        let capacity = model.capacity as i32;

        let rule_group_id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let rule_group_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:network-firewall:{}:{}:{}rule-group/{}",
                region,
                ctx.default_account_id,
                if rule_group_type == "STATEFUL" {
                    "stateful-"
                } else {
                    "stateless-"
                },
                name
            )
        });

        // The rule_group block is typically a JSON structure
        let rule_group_body = attrs.get("rule_group").cloned();

        let rg_view = RuleGroupView {
            rule_group_name: name,
            rule_group_arn: rule_group_arn.clone(),
            rule_group_id,
            rule_group_type,
            capacity,
            description: model.description,
            tags: extract_tags_vec(attrs),
            rule_group_body,
            rules: model.rules,
            encryption_configuration: parse_encryption_config(attrs),
        };

        let mut state_view = NetworkFirewallStateView::default();
        state_view.rule_groups.insert(rule_group_arn, rg_view);
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
                "id": rg.rule_group_id,
                "arn": rg.rule_group_arn,
                "name": rg.rule_group_name,
                "type": rg.rule_group_type,
                "capacity": rg.capacity,
                "description": rg.description,
                "rule_group": rg.rule_group_body,
                "rules": rg.rules,
                "encryption_configuration": emit_encryption_config(&rg.encryption_configuration),
                "tags": tags,
            });
            results.push(ExtractedResource {
                name: rg.rule_group_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
