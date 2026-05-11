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
    AccessLogSubscriptionView, AuthPolicyView, ListenerView, ResourceConfigurationView,
    ResourceGatewayView, ResourcePolicyView, RuleView, ServiceNetworkView, ServiceView,
    SnResourceAssociationView, SnServiceAssociationView, SnVpcAssociationView, TargetEntryView,
    TargetGroupView, VpcLatticeStateView,
};

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::vpclattice as vpclattice_gen;
use crate::util::{classify_deserialize_error, extract_region, optional_i64, optional_str};

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

// ---------------------------------------------------------------------------
// aws_vpclattice_access_log_subscription
// ---------------------------------------------------------------------------

/// Converts `aws_vpclattice_access_log_subscription` Terraform resources.
pub struct AwsVpcLatticeAccessLogSubscriptionConverter {
    service: Arc<VpcLatticeService>,
}

impl AwsVpcLatticeAccessLogSubscriptionConverter {
    pub fn new(service: Arc<VpcLatticeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcLatticeAccessLogSubscriptionConverter {
    fn resource_type(&self) -> &str {
        "aws_vpclattice_access_log_subscription"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpclattice_service", "aws_vpclattice_service_network"]
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

impl AwsVpcLatticeAccessLogSubscriptionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: vpclattice_gen::AccessLogSubscriptionTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_vpclattice_access_log_subscription", e)
            })?;

        let id = model
            .id
            .unwrap_or_else(|| format!("als-{}", uuid::Uuid::new_v4().simple()));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:vpc-lattice:{}:{}:accesslogsubscription/{}",
                region, ctx.default_account_id, id
            )
        });

        // Resolve the parent resource (service-network or service) by looking
        // it up in the current snapshot. If we can't find it, fall back to
        // synthesising an ARN from the identifier itself.
        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let (resource_arn, resource_id) = resolve_log_target(
            &snapshot,
            &model.resource_identifier,
            &ctx.default_account_id,
            &region,
            model.resource_arn.as_deref(),
        );

        let now = chrono::Utc::now().to_rfc3339();
        let als_view = AccessLogSubscriptionView {
            id: id.clone(),
            arn,
            resource_arn,
            resource_id,
            destination_arn: model.destination_arn,
            created_at: now.clone(),
            last_updated_at: now,
            tags: model.tags,
        };

        let mut state_view = VpcLatticeStateView::default();
        state_view.access_log_subscriptions.insert(id, als_view);
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
        for als in view.access_log_subscriptions.values() {
            let attrs = serde_json::json!({
                "id": als.id,
                "arn": als.arn,
                "resource_arn": als.resource_arn,
                "resource_identifier": als.resource_id,
                "destination_arn": als.destination_arn,
                "tags": als.tags,
            });
            results.push(ExtractedResource {
                name: als.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

fn resolve_log_target(
    view: &VpcLatticeStateView,
    identifier: &str,
    account_id: &str,
    region: &str,
    fallback_arn: Option<&str>,
) -> (String, String) {
    // Service network lookup first.
    if let Some(sn) = view
        .service_networks
        .values()
        .find(|s| s.id == identifier || s.arn == identifier || s.name == identifier)
    {
        return (sn.arn.clone(), sn.id.clone());
    }
    if let Some(svc) = view
        .services
        .values()
        .find(|s| s.id == identifier || s.arn == identifier || s.name == identifier)
    {
        return (svc.arn.clone(), svc.id.clone());
    }
    if let Some(arn) = fallback_arn {
        return (arn.to_string(), identifier.to_string());
    }
    if identifier.starts_with("arn:") {
        let id_part = identifier.rsplit('/').next().unwrap_or(identifier);
        return (identifier.to_string(), id_part.to_string());
    }
    let arn = if identifier.starts_with("sn-") {
        format!(
            "arn:aws:vpc-lattice:{}:{}:servicenetwork/{}",
            region, account_id, identifier
        )
    } else if identifier.starts_with("svc-") {
        format!(
            "arn:aws:vpc-lattice:{}:{}:service/{}",
            region, account_id, identifier
        )
    } else {
        format!(
            "arn:aws:vpc-lattice:{}:{}:resource/{}",
            region, account_id, identifier
        )
    };
    (arn, identifier.to_string())
}

// ---------------------------------------------------------------------------
// aws_vpclattice_auth_policy
// ---------------------------------------------------------------------------

/// Converts `aws_vpclattice_auth_policy` Terraform resources.
pub struct AwsVpcLatticeAuthPolicyConverter {
    service: Arc<VpcLatticeService>,
}

impl AwsVpcLatticeAuthPolicyConverter {
    pub fn new(service: Arc<VpcLatticeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcLatticeAuthPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_vpclattice_auth_policy"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpclattice_service", "aws_vpclattice_service_network"]
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

impl AwsVpcLatticeAuthPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: vpclattice_gen::AuthPolicyTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpclattice_auth_policy", e))?;

        let now = chrono::Utc::now().to_rfc3339();
        let policy_view = AuthPolicyView {
            policy: model.policy,
            state: model.state.unwrap_or_else(|| "Active".to_string()),
            created_at: now.clone(),
            last_updated_at: now,
        };

        let mut state_view = VpcLatticeStateView::default();
        state_view
            .auth_policies
            .insert(model.resource_identifier, policy_view);
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
        for (resource_identifier, policy) in &view.auth_policies {
            let attrs = serde_json::json!({
                "resource_identifier": resource_identifier,
                "policy": policy.policy,
                "state": policy.state,
            });
            results.push(ExtractedResource {
                name: resource_identifier.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpclattice_listener_rule
// ---------------------------------------------------------------------------

/// Converts `aws_vpclattice_listener_rule` Terraform resources.
pub struct AwsVpcLatticeListenerRuleConverter {
    service: Arc<VpcLatticeService>,
}

impl AwsVpcLatticeListenerRuleConverter {
    pub fn new(service: Arc<VpcLatticeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcLatticeListenerRuleConverter {
    fn resource_type(&self) -> &str {
        "aws_vpclattice_listener_rule"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpclattice_service", "aws_vpclattice_listener"]
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

impl AwsVpcLatticeListenerRuleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: vpclattice_gen::ListenerRuleTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpclattice_listener_rule", e))?;

        // Resolve the parent service + listener so the rule ARN points at the
        // real listener/service IDs when possible. TF's listener_identifier
        // may take the form "{service_id}/{listener_id}" or a bare listener
        // id; strip the prefix for lookup.
        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let svc_id = snapshot
            .services
            .values()
            .find(|s| {
                s.id == model.service_identifier
                    || s.arn == model.service_identifier
                    || s.name == model.service_identifier
            })
            .map(|s| s.id.clone())
            .unwrap_or_else(|| model.service_identifier.clone());

        let plain_listener = model
            .listener_identifier
            .split_once('/')
            .map(|x| x.1)
            .unwrap_or(&model.listener_identifier)
            .to_string();
        let listener_id = snapshot
            .listeners
            .values()
            .find(|l| {
                l.id == plain_listener
                    || l.arn == plain_listener
                    || l.name == plain_listener
                    || l.arn == model.listener_identifier
            })
            .map(|l| l.id.clone())
            .unwrap_or(plain_listener);

        let id = model
            .id
            .or(model.rule_id)
            .unwrap_or_else(|| format!("rule-{}", uuid::Uuid::new_v4().simple()));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:vpc-lattice:{}:{}:service/{}/listener/{}/rule/{}",
                region, ctx.default_account_id, svc_id, listener_id, id
            )
        });

        // Parse the nested `action` and `match` blocks from raw attrs.
        let (fixed_status, forward_tgs) = parse_rule_action(attrs);
        let (m_method, m_exact, m_prefix) = parse_rule_match(attrs);

        let now = chrono::Utc::now().to_rfc3339();
        let rule_view = RuleView {
            id: id.clone(),
            arn,
            name: model.name,
            listener_id,
            service_id: svc_id,
            priority: model.priority as i32,
            is_default: false,
            action_fixed_response_status_code: fixed_status,
            action_forward_target_groups: forward_tgs,
            match_method: m_method,
            match_path_exact: m_exact,
            match_path_prefix: m_prefix,
            created_at: now.clone(),
            last_updated_at: now,
        };

        let mut state_view = VpcLatticeStateView::default();
        state_view.rules.insert(id, rule_view);
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
        for r in view.rules.values() {
            let attrs = serde_json::json!({
                "id": r.id,
                "rule_id": r.id,
                "arn": r.arn,
                "name": r.name,
                "service_identifier": r.service_id,
                "listener_identifier": r.listener_id,
                "priority": r.priority,
            });
            results.push(ExtractedResource {
                name: r.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

fn first_block(v: &serde_json::Value) -> Option<&serde_json::Value> {
    if v.is_array() {
        v.as_array().and_then(|a| a.first())
    } else {
        Some(v)
    }
}

fn parse_rule_action(attrs: &serde_json::Value) -> (Option<i32>, Vec<(String, Option<i32>)>) {
    let action = attrs.get("action").and_then(first_block);
    let fixed = action
        .and_then(|a| a.get("fixed_response"))
        .and_then(first_block)
        .and_then(|fr| fr.get("status_code"))
        .and_then(|s| s.as_i64())
        .map(|s| s as i32);
    let forward: Vec<(String, Option<i32>)> = action
        .and_then(|a| a.get("forward"))
        .and_then(first_block)
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
    (fixed, forward)
}

fn parse_rule_match(attrs: &serde_json::Value) -> (Option<String>, Option<String>, Option<String>) {
    let m = attrs.get("match").and_then(first_block);
    let http = m.and_then(|x| x.get("http_match")).and_then(first_block);
    let method = http
        .and_then(|h| h.get("method"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let path_match = http.and_then(|h| h.get("path_match")).and_then(first_block);
    let match_block = path_match
        .and_then(|p| p.get("match"))
        .and_then(first_block);
    let path_exact = match_block
        .and_then(|x| x.get("exact"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let path_prefix = match_block
        .and_then(|x| x.get("prefix"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    (method, path_exact, path_prefix)
}

// ---------------------------------------------------------------------------
// aws_vpclattice_resource_configuration
// ---------------------------------------------------------------------------

/// Converts `aws_vpclattice_resource_configuration` Terraform resources.
pub struct AwsVpcLatticeResourceConfigurationConverter {
    service: Arc<VpcLatticeService>,
}

impl AwsVpcLatticeResourceConfigurationConverter {
    pub fn new(service: Arc<VpcLatticeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcLatticeResourceConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_vpclattice_resource_configuration"
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

impl AwsVpcLatticeResourceConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: vpclattice_gen::ResourceConfigurationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_vpclattice_resource_configuration", e)
            })?;

        let id = model
            .id
            .unwrap_or_else(|| format!("rcfg-{}", uuid::Uuid::new_v4().simple()));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:vpc-lattice:{}:{}:resourceconfiguration/{}",
                region, ctx.default_account_id, id
            )
        });
        let rc_type = model
            .resource_configuration_type
            .unwrap_or_else(|| "SINGLE".to_string());

        // port_ranges is a list-of-strings argument in the TF schema; parse
        // it from raw attrs so we don't lose it via the spec.
        let port_ranges: Vec<String> = attrs
            .get("port_ranges")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let now = chrono::Utc::now().to_rfc3339();
        let rc_view = ResourceConfigurationView {
            id: id.clone(),
            arn,
            name: model.name,
            resource_configuration_type: rc_type,
            status: "ACTIVE".to_string(),
            resource_gateway_id: model.resource_gateway_identifier,
            port_ranges,
            protocol: model.protocol,
            created_at: now.clone(),
            last_updated_at: now,
            tags: model.tags,
        };

        let mut state_view = VpcLatticeStateView::default();
        state_view.resource_configurations.insert(id, rc_view);
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
        for rc in view.resource_configurations.values() {
            let attrs = serde_json::json!({
                "id": rc.id,
                "arn": rc.arn,
                "name": rc.name,
                "type": rc.resource_configuration_type,
                "resource_gateway_identifier": rc.resource_gateway_id,
                "port_ranges": rc.port_ranges,
                "protocol": rc.protocol,
                "tags": rc.tags,
            });
            results.push(ExtractedResource {
                name: rc.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpclattice_resource_gateway
// ---------------------------------------------------------------------------

/// Converts `aws_vpclattice_resource_gateway` Terraform resources.
pub struct AwsVpcLatticeResourceGatewayConverter {
    service: Arc<VpcLatticeService>,
}

impl AwsVpcLatticeResourceGatewayConverter {
    pub fn new(service: Arc<VpcLatticeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcLatticeResourceGatewayConverter {
    fn resource_type(&self) -> &str {
        "aws_vpclattice_resource_gateway"
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

impl AwsVpcLatticeResourceGatewayConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: vpclattice_gen::ResourceGatewayTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_vpclattice_resource_gateway", e))?;

        let id = model
            .id
            .unwrap_or_else(|| format!("rgw-{}", uuid::Uuid::new_v4().simple()));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:vpc-lattice:{}:{}:resourcegateway/{}",
                region, ctx.default_account_id, id
            )
        });

        let subnet_ids: Vec<String> = attrs
            .get("subnet_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let security_group_ids: Vec<String> = attrs
            .get("security_group_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let now = chrono::Utc::now().to_rfc3339();
        let gw_view = ResourceGatewayView {
            id: id.clone(),
            arn,
            name: model.name,
            status: model.status.unwrap_or_else(|| "ACTIVE".to_string()),
            vpc_id: model.vpc_id,
            subnet_ids,
            security_group_ids,
            ip_address_type: model.ip_address_type,
            created_at: now.clone(),
            last_updated_at: now,
            tags: model.tags,
        };

        let mut state_view = VpcLatticeStateView::default();
        state_view.resource_gateways.insert(id, gw_view);
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
        for gw in view.resource_gateways.values() {
            let attrs = serde_json::json!({
                "id": gw.id,
                "arn": gw.arn,
                "name": gw.name,
                "status": gw.status,
                "vpc_id": gw.vpc_id,
                "subnet_ids": gw.subnet_ids,
                "security_group_ids": gw.security_group_ids,
                "ip_address_type": gw.ip_address_type,
                "tags": gw.tags,
            });
            results.push(ExtractedResource {
                name: gw.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpclattice_resource_policy
// ---------------------------------------------------------------------------

/// Converts `aws_vpclattice_resource_policy` Terraform resources.
pub struct AwsVpcLatticeResourcePolicyConverter {
    service: Arc<VpcLatticeService>,
}

impl AwsVpcLatticeResourcePolicyConverter {
    pub fn new(service: Arc<VpcLatticeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcLatticeResourcePolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_vpclattice_resource_policy"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpclattice_service", "aws_vpclattice_service_network"]
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

impl AwsVpcLatticeResourcePolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: vpclattice_gen::ResourcePolicyTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpclattice_resource_policy", e))?;

        let policy_view = ResourcePolicyView {
            policy: model.policy,
        };

        let mut state_view = VpcLatticeStateView::default();
        state_view
            .resource_policies
            .insert(model.resource_arn, policy_view);
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
        for (resource_arn, policy) in &view.resource_policies {
            let attrs = serde_json::json!({
                "resource_arn": resource_arn,
                "policy": policy.policy,
            });
            results.push(ExtractedResource {
                name: resource_arn.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpclattice_service_network_resource_association
// ---------------------------------------------------------------------------

/// Converts `aws_vpclattice_service_network_resource_association` Terraform resources.
pub struct AwsVpcLatticeServiceNetworkResourceAssociationConverter {
    service: Arc<VpcLatticeService>,
}

impl AwsVpcLatticeServiceNetworkResourceAssociationConverter {
    pub fn new(service: Arc<VpcLatticeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcLatticeServiceNetworkResourceAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_vpclattice_service_network_resource_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec![
            "aws_vpclattice_service_network",
            "aws_vpclattice_resource_configuration",
        ]
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

impl AwsVpcLatticeServiceNetworkResourceAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: vpclattice_gen::ServiceNetworkResourceAssociationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_vpclattice_service_network_resource_association", e)
            })?;

        let id = model
            .id
            .unwrap_or_else(|| format!("snra-{}", uuid::Uuid::new_v4().simple()));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:vpc-lattice:{}:{}:servicenetworkresourceassociation/{}",
                region, ctx.default_account_id, id
            )
        });

        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let (sn_id, sn_arn, sn_name) = resolve_sn(
            &snapshot,
            &model.service_network_identifier,
            &ctx.default_account_id,
            &region,
        );
        let (rc_id, rc_arn, rc_name) = resolve_rc(
            &snapshot,
            &model.resource_configuration_identifier,
            &ctx.default_account_id,
            &region,
        );

        let now = chrono::Utc::now().to_rfc3339();
        let assoc_view = SnResourceAssociationView {
            id: id.clone(),
            arn,
            service_network_identifier: model.service_network_identifier,
            resource_configuration_identifier: model.resource_configuration_identifier,
            service_network_id: sn_id,
            service_network_arn: sn_arn,
            service_network_name: sn_name,
            resource_configuration_id: rc_id,
            resource_configuration_arn: rc_arn,
            resource_configuration_name: rc_name,
            status: "ACTIVE".to_string(),
            created_at: now,
            tags: model.tags,
        };

        let mut state_view = VpcLatticeStateView::default();
        state_view.sn_resource_associations.insert(id, assoc_view);
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
        for a in view.sn_resource_associations.values() {
            let attrs = serde_json::json!({
                "id": a.id,
                "arn": a.arn,
                "service_network_identifier": a.service_network_identifier,
                "resource_configuration_identifier": a.resource_configuration_identifier,
                "status": a.status,
                "tags": a.tags,
            });
            results.push(ExtractedResource {
                name: a.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

fn resolve_sn(
    view: &VpcLatticeStateView,
    identifier: &str,
    account_id: &str,
    region: &str,
) -> (String, String, String) {
    if let Some(sn) = view
        .service_networks
        .values()
        .find(|s| s.id == identifier || s.arn == identifier || s.name == identifier)
    {
        return (sn.id.clone(), sn.arn.clone(), sn.name.clone());
    }
    let arn = format!(
        "arn:aws:vpc-lattice:{}:{}:servicenetwork/{}",
        region, account_id, identifier
    );
    (identifier.to_string(), arn, identifier.to_string())
}

fn resolve_svc(
    view: &VpcLatticeStateView,
    identifier: &str,
    account_id: &str,
    region: &str,
) -> (String, String, String) {
    if let Some(s) = view
        .services
        .values()
        .find(|s| s.id == identifier || s.arn == identifier || s.name == identifier)
    {
        return (s.id.clone(), s.arn.clone(), s.name.clone());
    }
    let arn = format!(
        "arn:aws:vpc-lattice:{}:{}:service/{}",
        region, account_id, identifier
    );
    (identifier.to_string(), arn, identifier.to_string())
}

fn resolve_rc(
    view: &VpcLatticeStateView,
    identifier: &str,
    account_id: &str,
    region: &str,
) -> (String, String, String) {
    if let Some(rc) = view
        .resource_configurations
        .values()
        .find(|r| r.id == identifier || r.arn == identifier || r.name == identifier)
    {
        return (rc.id.clone(), rc.arn.clone(), rc.name.clone());
    }
    let arn = format!(
        "arn:aws:vpc-lattice:{}:{}:resourceconfiguration/{}",
        region, account_id, identifier
    );
    (identifier.to_string(), arn, identifier.to_string())
}

// ---------------------------------------------------------------------------
// aws_vpclattice_service_network_service_association
// ---------------------------------------------------------------------------

/// Converts `aws_vpclattice_service_network_service_association` Terraform resources.
pub struct AwsVpcLatticeServiceNetworkServiceAssociationConverter {
    service: Arc<VpcLatticeService>,
}

impl AwsVpcLatticeServiceNetworkServiceAssociationConverter {
    pub fn new(service: Arc<VpcLatticeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcLatticeServiceNetworkServiceAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_vpclattice_service_network_service_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpclattice_service_network", "aws_vpclattice_service"]
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

impl AwsVpcLatticeServiceNetworkServiceAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: vpclattice_gen::ServiceNetworkServiceAssociationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_vpclattice_service_network_service_association", e)
            })?;

        let id = model
            .id
            .unwrap_or_else(|| format!("snsa-{}", uuid::Uuid::new_v4().simple()));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:vpc-lattice:{}:{}:servicenetworkserviceassociation/{}",
                region, ctx.default_account_id, id
            )
        });

        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let (sn_id, sn_arn, sn_name) = resolve_sn(
            &snapshot,
            &model.service_network_identifier,
            &ctx.default_account_id,
            &region,
        );
        let (svc_id, svc_arn, svc_name) = resolve_svc(
            &snapshot,
            &model.service_identifier,
            &ctx.default_account_id,
            &region,
        );

        let now = chrono::Utc::now().to_rfc3339();
        let assoc_view = SnServiceAssociationView {
            id: id.clone(),
            arn,
            service_network_identifier: model.service_network_identifier,
            service_identifier: model.service_identifier,
            service_network_id: sn_id,
            service_network_arn: sn_arn,
            service_network_name: sn_name,
            service_id: svc_id,
            service_arn: svc_arn,
            service_name: svc_name,
            status: "ACTIVE".to_string(),
            created_at: now,
            tags: model.tags,
        };

        let mut state_view = VpcLatticeStateView::default();
        state_view.sn_service_associations.insert(id, assoc_view);
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
        for a in view.sn_service_associations.values() {
            let attrs = serde_json::json!({
                "id": a.id,
                "arn": a.arn,
                "service_network_identifier": a.service_network_identifier,
                "service_identifier": a.service_identifier,
                "status": a.status,
                "tags": a.tags,
            });
            results.push(ExtractedResource {
                name: a.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpclattice_service_network_vpc_association
// ---------------------------------------------------------------------------

/// Converts `aws_vpclattice_service_network_vpc_association` Terraform resources.
pub struct AwsVpcLatticeServiceNetworkVpcAssociationConverter {
    service: Arc<VpcLatticeService>,
}

impl AwsVpcLatticeServiceNetworkVpcAssociationConverter {
    pub fn new(service: Arc<VpcLatticeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcLatticeServiceNetworkVpcAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_vpclattice_service_network_vpc_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpclattice_service_network"]
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

impl AwsVpcLatticeServiceNetworkVpcAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: vpclattice_gen::ServiceNetworkVpcAssociationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_vpclattice_service_network_vpc_association", e)
            })?;

        let id = model
            .id
            .unwrap_or_else(|| format!("snva-{}", uuid::Uuid::new_v4().simple()));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:vpc-lattice:{}:{}:servicenetworkvpcassociation/{}",
                region, ctx.default_account_id, id
            )
        });

        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let (sn_id, sn_arn, sn_name) = resolve_sn(
            &snapshot,
            &model.service_network_identifier,
            &ctx.default_account_id,
            &region,
        );

        let security_group_ids: Vec<String> = attrs
            .get("security_group_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let now = chrono::Utc::now().to_rfc3339();
        let assoc_view = SnVpcAssociationView {
            id: id.clone(),
            arn,
            service_network_identifier: model.service_network_identifier,
            vpc_identifier: model.vpc_identifier.clone(),
            service_network_id: sn_id,
            service_network_arn: sn_arn,
            service_network_name: sn_name,
            vpc_id: model.vpc_identifier,
            status: "ACTIVE".to_string(),
            security_group_ids,
            created_at: now.clone(),
            last_updated_at: now,
            tags: model.tags,
        };

        let mut state_view = VpcLatticeStateView::default();
        state_view.sn_vpc_associations.insert(id, assoc_view);
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
        for a in view.sn_vpc_associations.values() {
            let attrs = serde_json::json!({
                "id": a.id,
                "arn": a.arn,
                "service_network_identifier": a.service_network_identifier,
                "vpc_identifier": a.vpc_identifier,
                "status": a.status,
                "security_group_ids": a.security_group_ids,
                "tags": a.tags,
            });
            results.push(ExtractedResource {
                name: a.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpclattice_target_group_attachment
// (modifier; appends a TargetEntry to the parent TargetGroupView.targets)
// ---------------------------------------------------------------------------

/// Converts `aws_vpclattice_target_group_attachment` Terraform resources by
/// appending the registered target to the parent target group's `targets`
/// list. If the target group is not yet in state, the resource is accepted
/// with a warning.
pub struct AwsVpcLatticeTargetGroupAttachmentConverter {
    service: Arc<VpcLatticeService>,
}

impl AwsVpcLatticeTargetGroupAttachmentConverter {
    pub fn new(service: Arc<VpcLatticeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcLatticeTargetGroupAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_vpclattice_target_group_attachment"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpclattice_target_group"]
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

impl AwsVpcLatticeTargetGroupAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: vpclattice_gen::TargetGroupAttachmentTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_vpclattice_target_group_attachment", e)
            })?;

        // TF's `target` is a single nested block with `id` (required) and
        // `port` (optional). Parse it from raw attrs.
        let target_block = attrs.get("target").and_then(first_block);
        let target_id = target_block
            .and_then(|t| t.get("id"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .or_else(|| optional_str(attrs, "id"));
        let target_id = match target_id {
            Some(s) => s,
            None => {
                return Err(ConversionError::MissingAttribute {
                    resource_type: "aws_vpclattice_target_group_attachment".to_string(),
                    attribute: "target.id".to_string(),
                });
            }
        };
        let port = target_block
            .and_then(|t| t.get("port"))
            .and_then(|v| v.as_i64())
            .or_else(|| optional_i64(attrs, "port"))
            .filter(|p| *p > 0)
            .map(|p| p as i32);

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];

        // Find the target group by id, arn, or name.
        let tg_key = state_view
            .target_groups
            .iter()
            .find(|(_, tg)| {
                tg.id == model.target_group_identifier
                    || tg.arn == model.target_group_identifier
                    || tg.name == model.target_group_identifier
            })
            .map(|(k, _)| k.clone());

        if let Some(key) = tg_key {
            if let Some(tg) = state_view.target_groups.get_mut(&key) {
                let exists = tg
                    .targets
                    .iter()
                    .any(|t| t.id == target_id && t.port == port);
                if !exists {
                    tg.targets.push(TargetEntryView {
                        id: target_id,
                        port,
                        status: "HEALTHY".to_string(),
                    });
                }
            }
        } else {
            warnings.push(format!(
                "target group '{}' not found in state; target '{}' attachment skipped",
                model.target_group_identifier, target_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult { region, warnings })
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
            for t in &tg.targets {
                let port_part = t.port.map(|p| p.to_string()).unwrap_or_default();
                let id = format!("{}/{}/{}", tg.id, t.id, port_part);
                let target_block = serde_json::json!({
                    "id": t.id,
                    "port": t.port,
                });
                let attrs = serde_json::json!({
                    "id": id,
                    "target_group_identifier": tg.id,
                    "target": [target_block],
                });
                results.push(ExtractedResource {
                    name: id,
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}
