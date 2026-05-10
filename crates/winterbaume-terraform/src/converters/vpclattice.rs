//! Terraform converters for VPC Lattice resources.
//!
//! `VpcLatticeServiceTfModel`, `ServiceNetworkTfModel`,
//! `TargetGroupTfModel`, and `ListenerTfModel` are generated from
//! `specs/vpclattice.toml`. The ARN templates, the synthesised IDs,
//! the `auth_type = "NONE"` / `status = "ACTIVE"` /
//! `target_group_type = "INSTANCE"` / `protocol = "HTTP"` /
//! `number_of_associated_*` defaults, the `created_at` /
//! `last_updated_at` timestamps, and the nested config / default_action
//! blocks are wired up here.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;
use winterbaume_vpclattice::VpcLatticeService;
use winterbaume_vpclattice::views::{
    ListenerView, ServiceNetworkView, ServiceView, TargetGroupView, VpcLatticeStateView,
};

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::vpclattice as vpclattice_gen;
use crate::util::{classify_deserialize_error, extract_region, optional_i64};

// ---------------------------------------------------------------------------
// aws_vpclattice_service
// ---------------------------------------------------------------------------

/// Converts `aws_vpclattice_service` Terraform resources.
pub struct AwsVpcLatticeServiceConverter {
    service: Arc<VpcLatticeService>,
}

impl AwsVpcLatticeServiceConverter {
    pub fn new(service: Arc<VpcLatticeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcLatticeServiceConverter {
    fn resource_type(&self) -> &str {
        "aws_vpclattice_service"
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

impl AwsVpcLatticeServiceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: vpclattice_gen::VpcLatticeServiceTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpclattice_service", e))?;

        let id = model
            .id
            .unwrap_or_else(|| format!("svc-{}", uuid::Uuid::new_v4().simple()));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:vpc-lattice:{}:{}:service/{}",
                region, ctx.default_account_id, id
            )
        });
        let auth_type = model.auth_type.unwrap_or_else(|| "NONE".to_string());
        let status = model.status.unwrap_or_else(|| "ACTIVE".to_string());
        let now = chrono::Utc::now().to_rfc3339();

        let svc_view = ServiceView {
            id: id.clone(),
            name: model.name,
            arn,
            auth_type,
            status,
            created_at: now.clone(),
            last_updated_at: now,
            tags: model.tags,
        };

        let mut state_view = VpcLatticeStateView::default();
        state_view.services.insert(id, svc_view);
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
        for svc in view.services.values() {
            let attrs = serde_json::json!({
                "id": svc.id,
                "name": svc.name,
                "arn": svc.arn,
                "auth_type": svc.auth_type,
                "status": svc.status,
                "created_at": svc.created_at,
                "last_updated_at": svc.last_updated_at,
                "tags": svc.tags,
            });
            results.push(ExtractedResource {
                name: svc.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpclattice_service_network
// ---------------------------------------------------------------------------

/// Converts `aws_vpclattice_service_network` Terraform resources.
pub struct AwsVpcLatticeServiceNetworkConverter {
    service: Arc<VpcLatticeService>,
}

impl AwsVpcLatticeServiceNetworkConverter {
    pub fn new(service: Arc<VpcLatticeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcLatticeServiceNetworkConverter {
    fn resource_type(&self) -> &str {
        "aws_vpclattice_service_network"
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

impl AwsVpcLatticeServiceNetworkConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: vpclattice_gen::ServiceNetworkTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpclattice_service_network", e))?;

        let id = model
            .id
            .unwrap_or_else(|| format!("sn-{}", uuid::Uuid::new_v4().simple()));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:vpc-lattice:{}:{}:servicenetwork/{}",
                region, ctx.default_account_id, id
            )
        });
        let auth_type = model.auth_type.unwrap_or_else(|| "NONE".to_string());
        let now = chrono::Utc::now().to_rfc3339();

        let sn_view = ServiceNetworkView {
            id: id.clone(),
            name: model.name,
            arn,
            auth_type,
            created_at: now.clone(),
            last_updated_at: now,
            number_of_associated_services: 0,
            number_of_associated_v_p_cs: 0,
            tags: model.tags,
        };

        let mut state_view = VpcLatticeStateView::default();
        state_view.service_networks.insert(id, sn_view);
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
        for sn in view.service_networks.values() {
            let attrs = serde_json::json!({
                "id": sn.id,
                "name": sn.name,
                "arn": sn.arn,
                "auth_type": sn.auth_type,
                "created_at": sn.created_at,
                "last_updated_at": sn.last_updated_at,
                "number_of_associated_services": sn.number_of_associated_services,
                "number_of_associated_v_p_cs": sn.number_of_associated_v_p_cs,
                "tags": sn.tags,
            });
            results.push(ExtractedResource {
                name: sn.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpclattice_target_group
// ---------------------------------------------------------------------------

/// Converts `aws_vpclattice_target_group` Terraform resources.
pub struct AwsVpcLatticeTargetGroupConverter {
    service: Arc<VpcLatticeService>,
}

impl AwsVpcLatticeTargetGroupConverter {
    pub fn new(service: Arc<VpcLatticeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcLatticeTargetGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_vpclattice_target_group"
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

impl AwsVpcLatticeTargetGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: vpclattice_gen::TargetGroupTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpclattice_target_group", e))?;

        let id = model
            .id
            .unwrap_or_else(|| format!("tg-{}", uuid::Uuid::new_v4().simple()));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:vpc-lattice:{}:{}:targetgroup/{}",
                region, ctx.default_account_id, id
            )
        });
        let target_group_type = model
            .target_group_type
            .unwrap_or_else(|| "INSTANCE".to_string());
        let status = model.status.unwrap_or_else(|| "ACTIVE".to_string());
        let now = chrono::Utc::now().to_rfc3339();

        // Parse config block
        let config_block = attrs.get("config").and_then(|v| {
            if v.is_array() {
                v.as_array().and_then(|a| a.first())
            } else {
                Some(v)
            }
        });
        let config_port = config_block
            .and_then(|c| c.get("port"))
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);
        let config_protocol = config_block
            .and_then(|c| c.get("protocol"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let config_vpc_identifier = config_block
            .and_then(|c| c.get("vpc_identifier"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let config_ip_address_type = config_block
            .and_then(|c| c.get("ip_address_type"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let config_protocol_version = config_block
            .and_then(|c| c.get("protocol_version"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let config_lambda_event_structure_version = config_block
            .and_then(|c| c.get("lambda_event_structure_version"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let tg_view = TargetGroupView {
            id: id.clone(),
            arn,
            name: model.name,
            target_group_type,
            config_port,
            config_protocol,
            config_vpc_identifier,
            config_ip_address_type,
            config_protocol_version,
            config_lambda_event_structure_version,
            status,
            created_at: now.clone(),
            last_updated_at: now,
            tags: model.tags,
            targets: vec![],
        };

        let mut state_view = VpcLatticeStateView::default();
        state_view.target_groups.insert(id, tg_view);
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
        for tg in view.target_groups.values() {
            let config = serde_json::json!({
                "port": tg.config_port,
                "protocol": tg.config_protocol,
                "vpc_identifier": tg.config_vpc_identifier,
                "ip_address_type": tg.config_ip_address_type,
                "protocol_version": tg.config_protocol_version,
                "lambda_event_structure_version": tg.config_lambda_event_structure_version,
            });
            let attrs = serde_json::json!({
                "id": tg.id,
                "name": tg.name,
                "arn": tg.arn,
                "type": tg.target_group_type,
                "config": [config],
                "status": tg.status,
                "created_at": tg.created_at,
                "last_updated_at": tg.last_updated_at,
                "tags": tg.tags,
            });
            results.push(ExtractedResource {
                name: tg.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpclattice_listener
// ---------------------------------------------------------------------------

/// Converts `aws_vpclattice_listener` Terraform resources.
pub struct AwsVpcLatticeListenerConverter {
    service: Arc<VpcLatticeService>,
}

impl AwsVpcLatticeListenerConverter {
    pub fn new(service: Arc<VpcLatticeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcLatticeListenerConverter {
    fn resource_type(&self) -> &str {
        "aws_vpclattice_listener"
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

impl AwsVpcLatticeListenerConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: vpclattice_gen::ListenerTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpclattice_listener", e))?;

        let id = model
            .id
            .unwrap_or_else(|| format!("listener-{}", uuid::Uuid::new_v4().simple()));
        let service_id = model.service_identifier.unwrap_or_default();
        let service_arn = model.service_arn.unwrap_or_default();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:vpc-lattice:{}:{}:service/{}/listener/{}",
                region, ctx.default_account_id, service_id, id
            )
        });
        let protocol = model.protocol.unwrap_or_else(|| "HTTP".to_string());
        let port = optional_i64(attrs, "port").map(|p| p as i32);
        let now = chrono::Utc::now().to_rfc3339();

        // Parse default_action
        let default_action = attrs.get("default_action").and_then(|v| {
            if v.is_array() {
                v.as_array().and_then(|a| a.first())
            } else {
                Some(v)
            }
        });
        let fixed_response_status_code = default_action
            .and_then(|a| a.get("fixed_response"))
            .and_then(|v| {
                if v.is_array() {
                    v.as_array().and_then(|a| a.first())
                } else {
                    Some(v)
                }
            })
            .and_then(|fr| fr.get("status_code"))
            .and_then(|s| s.as_i64())
            .map(|s| s as i32);

        let forward_target_groups: Vec<(String, Option<i32>)> = default_action
            .and_then(|a| a.get("forward"))
            .and_then(|v| {
                if v.is_array() {
                    v.as_array().and_then(|a| a.first())
                } else {
                    Some(v)
                }
            })
            .and_then(|fwd| fwd.get("target_groups"))
            .and_then(|tgs| tgs.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|tg| {
                        let tg_id = tg.get("target_group_identifier").and_then(|v| v.as_str())?;
                        let weight = tg.get("weight").and_then(|w| w.as_i64()).map(|w| w as i32);
                        Some((tg_id.to_string(), weight))
                    })
                    .collect()
            })
            .unwrap_or_default();

        let listener_view = ListenerView {
            id: id.clone(),
            arn,
            name: model.name,
            service_id,
            service_arn,
            port,
            protocol,
            default_action_fixed_response_status_code: fixed_response_status_code,
            default_action_forward_target_groups: forward_target_groups,
            created_at: now.clone(),
            last_updated_at: now,
            tags: model.tags,
        };

        let mut state_view = VpcLatticeStateView::default();
        state_view.listeners.insert(id, listener_view);
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
        for l in view.listeners.values() {
            let forward_tgs: Vec<serde_json::Value> = l
                .default_action_forward_target_groups
                .iter()
                .map(|(tg_id, weight)| {
                    serde_json::json!({
                        "target_group_identifier": tg_id,
                        "weight": weight,
                    })
                })
                .collect();
            let attrs = serde_json::json!({
                "id": l.id,
                "name": l.name,
                "arn": l.arn,
                "service_identifier": l.service_id,
                "service_arn": l.service_arn,
                "port": l.port,
                "protocol": l.protocol,
                "default_action": [{
                    "fixed_response": if l.default_action_fixed_response_status_code.is_some() {
                        serde_json::json!([{"status_code": l.default_action_fixed_response_status_code}])
                    } else {
                        serde_json::json!(null)
                    },
                    "forward": if !forward_tgs.is_empty() {
                        serde_json::json!([{"target_groups": forward_tgs}])
                    } else {
                        serde_json::json!(null)
                    },
                }],
                "created_at": l.created_at,
                "last_updated_at": l.last_updated_at,
                "tags": l.tags,
            });
            results.push(ExtractedResource {
                name: l.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
