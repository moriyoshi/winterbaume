//! Terraform converter for Resource Groups resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_resourcegroups::ResourceGroupsService;
use winterbaume_resourcegroups::views::{
    AccountSettingsView, GroupConfigEntryView, GroupConfigParamView, ResourceGroupView,
    ResourceGroupsStateView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_resourcegroups_group
// ---------------------------------------------------------------------------

/// Converts `aws_resourcegroups_group` Terraform resources to/from Resource Groups state.
pub struct AwsResourcegroupsGroupConverter {
    service: Arc<ResourceGroupsService>,
}

impl AwsResourcegroupsGroupConverter {
    pub fn new(service: Arc<ResourceGroupsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsResourcegroupsGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_resourcegroups_group"
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

impl AwsResourcegroupsGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_resourcegroups_group")?;
        let region = extract_region(attrs, &ctx.default_region);
        let description = optional_str(attrs, "description").unwrap_or_default();

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:resource-groups:{}:{}:group/{}",
                region, ctx.default_account_id, name
            )
        });

        // Parse resource_query block
        let rq = attrs.get("resource_query");
        let (query_type, query_query) = if let Some(rq_val) = rq {
            let rq_obj = rq_val.as_array().and_then(|a| a.first()).unwrap_or(rq_val);
            let qt = rq_obj
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("TAG_FILTERS_1_0")
                .to_string();
            let qq = rq_obj
                .get("query")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            (qt, qq)
        } else {
            ("TAG_FILTERS_1_0".to_string(), String::new())
        };

        // Parse configuration blocks
        let configuration = attrs.get("configuration").and_then(|v| {
            v.as_array().map(|arr| {
                arr.iter()
                    .map(|c| {
                        let config_type = c
                            .get("type")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        let parameters = c
                            .get("parameters")
                            .and_then(|v| v.as_array())
                            .map(|params| {
                                params
                                    .iter()
                                    .map(|p| GroupConfigParamView {
                                        name: p
                                            .get("name")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("")
                                            .to_string(),
                                        values: p
                                            .get("values")
                                            .and_then(|v| v.as_array())
                                            .map(|vals| {
                                                vals.iter()
                                                    .filter_map(|v| {
                                                        v.as_str().map(|s| s.to_string())
                                                    })
                                                    .collect()
                                            })
                                            .unwrap_or_default(),
                                    })
                                    .collect()
                            })
                            .unwrap_or_default();
                        GroupConfigEntryView {
                            config_type,
                            parameters,
                        }
                    })
                    .collect()
            })
        });

        let group_view = ResourceGroupView {
            name: name.to_string(),
            arn,
            description,
            resource_query_type: query_type,
            resource_query_query: query_query,
            tags: extract_tags(attrs),
            configuration,
            resource_arns: vec![],
        };

        let mut state_view = ResourceGroupsStateView {
            groups: HashMap::new(),
            tag_sync_tasks: HashMap::new(),
            next_task_id: 0,
            account_settings: AccountSettingsView {
                group_lifecycle_events_desired_status: String::new(),
            },
        };
        state_view.groups.insert(name.to_string(), group_view);
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
        for group in view.groups.values() {
            let mut attrs = serde_json::json!({
                "id": group.name,
                "name": group.name,
                "arn": group.arn,
                "description": group.description,
                "resource_query": [{
                    "type": group.resource_query_type,
                    "query": group.resource_query_query,
                }],
                "tags": group.tags,
            });
            if let Some(configs) = &group.configuration {
                let config_arr: Vec<serde_json::Value> = configs
                    .iter()
                    .map(|c| {
                        let params: Vec<serde_json::Value> = c
                            .parameters
                            .iter()
                            .map(|p| {
                                serde_json::json!({
                                    "name": p.name,
                                    "values": p.values,
                                })
                            })
                            .collect();
                        serde_json::json!({
                            "type": c.config_type,
                            "parameters": params,
                        })
                    })
                    .collect();
                attrs["configuration"] = serde_json::Value::Array(config_arr);
            }
            results.push(ExtractedResource {
                name: group.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
