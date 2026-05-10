//! Terraform converters for OpenSearch resources.
//!
//! `DomainTfModel` is generated from `specs/opensearch.toml`. The ARN
//! template, the synthesised `domain_id`, the `engine_version` default,
//! the nested `cluster_config` / `ebs_options` blocks, and the
//! `created`/`deleted`/`processing` constants are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_opensearch::OpenSearchService;
use winterbaume_opensearch::views::{DomainView, OpenSearchStateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::opensearch as opensearch_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_opensearch_domain
// ---------------------------------------------------------------------------

pub struct AwsOpensearchDomainConverter {
    service: Arc<OpenSearchService>,
}

impl AwsOpensearchDomainConverter {
    pub fn new(service: Arc<OpenSearchService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsOpensearchDomainConverter {
    fn resource_type(&self) -> &str {
        "aws_opensearch_domain"
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

impl AwsOpensearchDomainConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: opensearch_gen::DomainTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_opensearch_domain", e))?;

        let attrs = &instance.attributes;

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:es:{}:{}:domain/{}",
                region, ctx.default_account_id, model.domain_name
            )
        });
        let domain_id = model
            .domain_id
            .unwrap_or_else(|| format!("{}/{}", ctx.default_account_id, model.domain_name));
        let engine_version = model
            .engine_version
            .unwrap_or_else(|| "OpenSearch_2.11".to_string());

        // cluster_config block
        let instance_type = attrs
            .get("cluster_config")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|c| c.get("instance_type"))
            .and_then(|v| v.as_str())
            .unwrap_or("t3.small.search")
            .to_string();
        let instance_count = attrs
            .get("cluster_config")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|c| c.get("instance_count"))
            .and_then(|v| v.as_i64())
            .unwrap_or(1) as i32;

        // ebs_options block
        let ebs_enabled = attrs
            .get("ebs_options")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|e| e.get("ebs_enabled"))
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let ebs_volume_size = attrs
            .get("ebs_options")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|e| e.get("volume_size"))
            .and_then(|v| v.as_i64())
            .unwrap_or(10) as i32;
        let ebs_volume_type = attrs
            .get("ebs_options")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|e| e.get("volume_type"))
            .and_then(|v| v.as_str())
            .unwrap_or("gp2")
            .to_string();

        let dedicated_master_enabled = attrs
            .get("cluster_config")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|c| c.get("dedicated_master_enabled"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let zone_awareness_enabled = attrs
            .get("cluster_config")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|c| c.get("zone_awareness_enabled"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let access_policies = model.access_policies.unwrap_or_default();

        let domain_view = DomainView {
            domain_name: model.domain_name.clone(),
            arn,
            domain_id,
            endpoint: model.endpoint,
            engine_version,
            created: true,
            deleted: false,
            processing: false,
            instance_type,
            instance_count,
            dedicated_master_enabled,
            zone_awareness_enabled,
            ebs_enabled,
            ebs_volume_size,
            ebs_volume_type,
            access_policies,
            tags: model.tags,
        };

        let mut state_view = OpenSearchStateView {
            domains: HashMap::new(),
            tags: HashMap::new(),
            vpc_endpoints: HashMap::new(),
            vpc_authorized_principals: HashMap::new(),
            data_sources: HashMap::new(),
            direct_query_data_sources: HashMap::new(),
            packages: HashMap::new(),
            outbound_connections: HashMap::new(),
            inbound_connections: HashMap::new(),
            reserved_instances: HashMap::new(),
        };
        state_view.domains.insert(model.domain_name, domain_view);
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
        for domain in view.domains.values() {
            let attrs = serde_json::json!({
                "id": domain.arn,
                "domain_name": domain.domain_name,
                "arn": domain.arn,
                "domain_id": domain.domain_id,
                "endpoint": domain.endpoint,
                "engine_version": domain.engine_version,
                "cluster_config": [{
                    "instance_type": domain.instance_type,
                    "instance_count": domain.instance_count,
                    "dedicated_master_enabled": domain.dedicated_master_enabled,
                    "zone_awareness_enabled": domain.zone_awareness_enabled,
                }],
                "ebs_options": [{
                    "ebs_enabled": domain.ebs_enabled,
                    "volume_size": domain.ebs_volume_size,
                    "volume_type": domain.ebs_volume_type,
                }],
                "access_policies": domain.access_policies,
                "tags": domain.tags,
                "tags_all": domain.tags,
                "kibana_endpoint": "",
                "dashboard_endpoint": "",
                "advanced_options": {},
                "vpc_options": [],
                "node_to_node_encryption": [{"enabled": false}],
            });
            results.push(ExtractedResource {
                name: domain.domain_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
