//! Terraform converters for Route 53 resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_route53::Route53Service;
use winterbaume_route53::views::{
    DelegationSetView, HostedZoneView, ResourceRecordSetView, Route53StateView, VpcView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::route53 as route53_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_route53_zone
// ---------------------------------------------------------------------------

/// Converts `aws_route53_zone` Terraform resources to/from Route 53 state.
pub struct AwsRoute53ZoneConverter {
    service: Arc<Route53Service>,
}

impl AwsRoute53ZoneConverter {
    pub fn new(service: Arc<Route53Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53ZoneConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_zone"
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

impl AwsRoute53ZoneConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: route53_gen::HostedZoneTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_route53_zone", e))?;

        // Zone name may or may not have trailing dot
        let name = if model.name.ends_with('.') {
            model.name.clone()
        } else {
            format!("{}.", model.name)
        };

        let _tags_all = attrs.get("tags_all");
        let _force_destroy = attrs.get("force_destroy");
        let _delegation_set_id = attrs.get("delegation_set_id");

        // Parse vpc blocks
        let vpcs: Vec<VpcView> = attrs
            .get("vpc")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| {
                        let vpc_id = v.get("vpc_id")?.as_str()?.to_string();
                        let vpc_region = v
                            .get("vpc_region")
                            .and_then(|r| r.as_str())
                            .unwrap_or(&region)
                            .to_string();
                        Some(VpcView { vpc_id, vpc_region })
                    })
                    .collect()
            })
            .unwrap_or_default();

        // zone_id is the canonical field; fall back to id
        let raw_id = model
            .zone_id
            .or(model.id)
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()[..8].to_string());

        // Ensure the id has the /hostedzone/ prefix used as the map key
        let zone_id = if raw_id.starts_with("/hostedzone/") {
            raw_id
        } else {
            format!("/hostedzone/{}", raw_id)
        };

        let comment = model.comment;

        // Tags
        let mut tags: HashMap<String, String> = HashMap::new();
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

        let zone_view = HostedZoneView {
            id: zone_id.clone(),
            name,
            caller_reference: uuid::Uuid::new_v4().to_string(),
            resource_record_count: 0,
            records: vec![],
            vpcs,
            comment,
            delegation_set: DelegationSetView {
                id: String::new(),
                caller_reference: None,
                name_servers: vec![],
            },
            tags,
        };

        let mut state_view = minimal_route53_state_view();
        state_view.hosted_zones.insert(zone_id, zone_view);
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
        for zone in view.hosted_zones.values() {
            let vpcs_json: Vec<serde_json::Value> = zone
                .vpcs
                .iter()
                .map(|vpc| serde_json::json!({"vpc_id": vpc.vpc_id, "vpc_region": vpc.vpc_region}))
                .collect();
            let attrs = serde_json::json!({
                "id": zone.id,
                "zone_id": zone.id.trim_start_matches("/hostedzone/"),
                "name": zone.name,
                "comment": zone.comment,
                "name_servers": zone.delegation_set.name_servers,
                "tags": zone.tags,
                "tags_all": zone.tags,
                "vpc": vpcs_json,
            });
            results.push(ExtractedResource {
                name: zone.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_route53_record
// ---------------------------------------------------------------------------

/// Converts `aws_route53_record` Terraform resources to/from Route 53 state.
pub struct AwsRoute53RecordConverter {
    service: Arc<Route53Service>,
}

impl AwsRoute53RecordConverter {
    pub fn new(service: Arc<Route53Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53RecordConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_record"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_route53_zone"]
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

impl AwsRoute53RecordConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: route53_gen::ResourceRecordSetTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_route53_record", e))?;

        let raw_zone_id = model.zone_id.clone();
        let zone_id = if raw_zone_id.starts_with("/hostedzone/") {
            raw_zone_id
        } else {
            format!("/hostedzone/{}", raw_zone_id)
        };

        let rec_name = model.name.clone();
        let rec_type = model.record_type.clone();
        let _allow_overwrite = attrs.get("allow_overwrite");
        let alias = attrs
            .get("alias")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let weighted_routing_policy = attrs
            .get("weighted_routing_policy")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let failover_routing_policy = attrs
            .get("failover_routing_policy")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let geolocation_routing_policy = attrs
            .get("geolocation_routing_policy")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let latency_routing_policy = attrs
            .get("latency_routing_policy")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let multivalue_answer_routing_policy = attrs
            .get("multivalue_answer_routing_policy")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let cidr_routing_policy = attrs
            .get("cidr_routing_policy")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let set_identifier = model.set_identifier;
        let health_check_id = model.health_check_id;

        let ttl = attrs.get("ttl").and_then(|v| v.as_u64());

        // records is an array of strings
        let resource_records: Vec<String> = attrs
            .get("records")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let record_view = ResourceRecordSetView {
            name: rec_name.clone(),
            record_type: rec_type.clone(),
            ttl,
            resource_records,
            alias,
            weighted_routing_policy,
            failover_routing_policy,
            geolocation_routing_policy,
            latency_routing_policy,
            multivalue_answer_routing_policy,
            cidr_routing_policy,
            set_identifier,
            health_check_id,
        };

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        if let Some(zone) = state_view.hosted_zones.get_mut(&zone_id) {
            zone.records
                .retain(|r| !(r.name == rec_name && r.record_type == rec_type));
            zone.records.push(record_view);
            zone.resource_record_count = zone.records.len() as u64;
        } else {
            // Zone not yet injected — create placeholder zone
            let zone_view = HostedZoneView {
                id: zone_id.clone(),
                name: String::new(),
                caller_reference: uuid::Uuid::new_v4().to_string(),
                resource_record_count: 1,
                records: vec![record_view],
                vpcs: vec![],
                comment: None,
                delegation_set: DelegationSetView {
                    id: String::new(),
                    caller_reference: None,
                    name_servers: vec![],
                },
                tags: HashMap::new(),
            };
            state_view.hosted_zones.insert(zone_id, zone_view);
        }
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
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
        for zone in view.hosted_zones.values() {
            let zone_id_short = zone.id.trim_start_matches("/hostedzone/");
            for rec in &zone.records {
                let id = format!("{}_{}_{}", zone_id_short, rec.name, rec.record_type);
                // Compute fqdn: ensure the name is fully qualified with zone name
                let fqdn = if rec.name.ends_with('.') {
                    rec.name.clone()
                } else if !zone.name.is_empty() {
                    if rec
                        .name
                        .ends_with(&zone.name.trim_end_matches('.').to_string())
                    {
                        format!("{}.", rec.name)
                    } else {
                        format!("{}.{}", rec.name, zone.name)
                    }
                } else {
                    rec.name.clone()
                };
                let attrs = serde_json::json!({
                    "id": id,
                    "zone_id": zone_id_short,
                    "name": rec.name,
                    "type": rec.record_type,
                    "ttl": rec.ttl,
                    "records": rec.resource_records,
                    "fqdn": fqdn,
                    "alias": rec.alias,
                    "weighted_routing_policy": rec.weighted_routing_policy,
                    "failover_routing_policy": rec.failover_routing_policy,
                    "geolocation_routing_policy": rec.geolocation_routing_policy,
                    "latency_routing_policy": rec.latency_routing_policy,
                    "multivalue_answer_routing_policy": rec.multivalue_answer_routing_policy,
                    "cidr_routing_policy": rec.cidr_routing_policy,
                    "set_identifier": rec.set_identifier,
                    "health_check_id": rec.health_check_id,
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

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn minimal_route53_state_view() -> Route53StateView {
    Route53StateView::default()
}
