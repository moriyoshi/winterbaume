//! Terraform converter for EC2 Instance Connect resources.
//!
//! `EndpointTfModel` is generated from `specs/ec2instanceconnect.toml`. The
//! synthesised endpoint id (`eice-...`), the `state` / `owner_id` /
//! `availability_zone` / `created_at` fallbacks, and the
//! `security_group_ids` / `network_interface_ids` Vec<String> projections
//! are wired up here.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_ec2instanceconnect::Ec2InstanceConnectService;
use winterbaume_ec2instanceconnect::views::{Ec2InstanceConnectStateView, EndpointView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::ec2instanceconnect as ec2instanceconnect_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_ec2_instance_connect_endpoint
// ---------------------------------------------------------------------------

/// Converts `aws_ec2_instance_connect_endpoint` Terraform resources.
pub struct AwsEc2InstanceConnectEndpointConverter {
    service: Arc<Ec2InstanceConnectService>,
}

impl AwsEc2InstanceConnectEndpointConverter {
    pub fn new(service: Arc<Ec2InstanceConnectService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2InstanceConnectEndpointConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_instance_connect_endpoint"
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

impl AwsEc2InstanceConnectEndpointConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ec2instanceconnect_gen::EndpointTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ec2_instance_connect_endpoint", e))?;

        let endpoint_id = model
            .id
            .clone()
            .or_else(|| model.instance_connect_endpoint_id.clone())
            .unwrap_or_else(|| {
                format!("eice-{}", &uuid::Uuid::new_v4().simple().to_string()[..17])
            });

        let security_group_ids: Vec<String> = instance
            .attributes
            .get("security_group_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let network_interface_ids: Vec<String> = instance
            .attributes
            .get("network_interface_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let endpoint_view = EndpointView {
            instance_connect_endpoint_id: endpoint_id.clone(),
            subnet_id: model.subnet_id,
            vpc_id: model.vpc_id.unwrap_or_default(),
            security_group_ids,
            state: model.state.unwrap_or_else(|| "create-complete".to_string()),
            dns_name: model.dns_name,
            fips_dns_name: model.fips_dns_name,
            network_interface_ids,
            owner_id: model
                .owner_id
                .unwrap_or_else(|| ctx.default_account_id.clone()),
            availability_zone: model
                .availability_zone
                .unwrap_or_else(|| format!("{}a", region)),
            created_at: model
                .created_at
                .unwrap_or_else(|| "2024-01-01T00:00:00Z".to_string()),
            tags: model.tags,
        };

        let mut state_view = Ec2InstanceConnectStateView::default();
        state_view.endpoints.insert(endpoint_id, endpoint_view);
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
        for ep in view.endpoints.values() {
            let attrs = serde_json::json!({
                "id": ep.instance_connect_endpoint_id,
                "instance_connect_endpoint_id": ep.instance_connect_endpoint_id,
                "subnet_id": ep.subnet_id,
                "vpc_id": ep.vpc_id,
                "security_group_ids": ep.security_group_ids,
                "state": ep.state,
                "dns_name": ep.dns_name,
                "fips_dns_name": ep.fips_dns_name,
                "network_interface_ids": ep.network_interface_ids,
                "owner_id": ep.owner_id,
                "availability_zone": ep.availability_zone,
                "created_at": ep.created_at,
                "tags": ep.tags,
                "tags_all": ep.tags,
            });
            results.push(ExtractedResource {
                name: ep.instance_connect_endpoint_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
