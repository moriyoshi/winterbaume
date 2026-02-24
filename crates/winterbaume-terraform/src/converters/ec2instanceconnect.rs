//! Terraform converter for EC2 Instance Connect resources.

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
use crate::util::{extract_region, extract_tags, optional_str, require_str};

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
        let attrs = &instance.attributes;
        let subnet_id = require_str(attrs, "subnet_id", "aws_ec2_instance_connect_endpoint")?;
        let region = extract_region(attrs, &ctx.default_region);

        let endpoint_id = optional_str(attrs, "id")
            .or_else(|| optional_str(attrs, "instance_connect_endpoint_id"))
            .unwrap_or_else(|| {
                format!("eice-{}", &uuid::Uuid::new_v4().simple().to_string()[..17])
            });

        let vpc_id = optional_str(attrs, "vpc_id").unwrap_or_default();
        let security_group_ids: Vec<String> = attrs
            .get("security_group_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let state = optional_str(attrs, "state").unwrap_or_else(|| "create-complete".to_string());
        let dns_name = optional_str(attrs, "dns_name");
        let fips_dns_name = optional_str(attrs, "fips_dns_name");
        let network_interface_ids: Vec<String> = attrs
            .get("network_interface_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let owner_id =
            optional_str(attrs, "owner_id").unwrap_or_else(|| ctx.default_account_id.clone());
        let availability_zone =
            optional_str(attrs, "availability_zone").unwrap_or_else(|| format!("{}a", region));
        let created_at =
            optional_str(attrs, "created_at").unwrap_or_else(|| "2024-01-01T00:00:00Z".to_string());
        let tags = extract_tags(attrs);

        let endpoint_view = EndpointView {
            instance_connect_endpoint_id: endpoint_id.clone(),
            subnet_id: subnet_id.to_string(),
            vpc_id,
            security_group_ids,
            state,
            dns_name,
            fips_dns_name,
            network_interface_ids,
            owner_id,
            availability_zone,
            created_at,
            tags,
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
