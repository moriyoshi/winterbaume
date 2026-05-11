//! Terraform converters for Route 53 resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_route53::Route53Service;
use winterbaume_route53::views::{
    CidrCollectionView, DelegationSetView, HealthCheckConfigView, HealthCheckView, HostedZoneView,
    KeySigningKeyView, QueryLoggingConfigView, ResourceRecordSetView, Route53StateView,
    TrafficPolicyInstanceView, TrafficPolicyView, VpcAssociationAuthorizationView, VpcView,
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
// aws_route53_cidr_collection
// ---------------------------------------------------------------------------

/// Converts `aws_route53_cidr_collection` Terraform resources.
pub struct AwsRoute53CidrCollectionConverter {
    service: Arc<Route53Service>,
}

impl AwsRoute53CidrCollectionConverter {
    pub fn new(service: Arc<Route53Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53CidrCollectionConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_cidr_collection"
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

impl AwsRoute53CidrCollectionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: route53_gen::CidrCollectionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_route53_cidr_collection", e))?;

        let collection_id = model
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let arn = format!(
            "arn:aws:route53::{}:cidrcollection/{}",
            ctx.default_account_id, collection_id
        );

        let view = CidrCollectionView {
            id: collection_id.clone(),
            name: model.name,
            arn,
            version: 1,
            locations: HashMap::new(),
        };

        let mut state_view = minimal_route53_state_view();
        state_view.cidr_collections.insert(collection_id, view);
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
        for c in view.cidr_collections.values() {
            let attrs = serde_json::json!({
                "id": c.id,
                "name": c.name,
                "arn": c.arn,
                "version": c.version,
            });
            results.push(ExtractedResource {
                name: c.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_route53_cidr_location
// ---------------------------------------------------------------------------

/// Converts `aws_route53_cidr_location` Terraform resources.
///
/// Sub-resource modifier: snapshots the parent collection and inserts the
/// location's `cidr_blocks` list under the location `name`. `cidr_blocks`
/// is a `Vec<String>` read directly from `instance.attributes`.
pub struct AwsRoute53CidrLocationConverter {
    service: Arc<Route53Service>,
}

impl AwsRoute53CidrLocationConverter {
    pub fn new(service: Arc<Route53Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53CidrLocationConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_cidr_location"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_route53_cidr_collection"]
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

impl AwsRoute53CidrLocationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: route53_gen::CidrLocationTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_route53_cidr_location", e))?;

        let cidr_blocks: Vec<String> = attrs
            .get("cidr_blocks")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(collection) = state_view
            .cidr_collections
            .get_mut(&model.cidr_collection_id)
        {
            collection.locations.insert(model.name.clone(), cidr_blocks);
            collection.version += 1;
        } else {
            warnings.push(format!(
                "cidr collection '{}' not found in state; location skipped",
                model.cidr_collection_id
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
        for c in view.cidr_collections.values() {
            for (loc_name, blocks) in &c.locations {
                let id = format!("{},{}", c.id, loc_name);
                let attrs = serde_json::json!({
                    "id": id,
                    "cidr_collection_id": c.id,
                    "name": loc_name,
                    "cidr_blocks": blocks,
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
// aws_route53_delegation_set
// ---------------------------------------------------------------------------

/// Converts `aws_route53_delegation_set` Terraform resources.
pub struct AwsRoute53DelegationSetConverter {
    service: Arc<Route53Service>,
}

impl AwsRoute53DelegationSetConverter {
    pub fn new(service: Arc<Route53Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53DelegationSetConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_delegation_set"
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

impl AwsRoute53DelegationSetConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: route53_gen::DelegationSetTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_route53_delegation_set", e))?;

        let id = model
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let view = DelegationSetView {
            id: id.clone(),
            caller_reference: model.reference_name,
            name_servers: vec![],
        };

        let mut state_view = minimal_route53_state_view();
        state_view.reusable_delegation_sets.insert(id, view);
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
        for ds in view.reusable_delegation_sets.values() {
            let attrs = serde_json::json!({
                "id": ds.id,
                "reference_name": ds.caller_reference,
                "name_servers": ds.name_servers,
            });
            results.push(ExtractedResource {
                name: ds.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_route53_health_check
// ---------------------------------------------------------------------------

/// Converts `aws_route53_health_check` Terraform resources.
///
/// Several optional integer and boolean fields, plus the `Vec<String>` lists
/// `child_health_checks` and `regions`, are read directly from
/// `instance.attributes` because the spec format does not currently express
/// those types.
pub struct AwsRoute53HealthCheckConverter {
    service: Arc<Route53Service>,
}

impl AwsRoute53HealthCheckConverter {
    pub fn new(service: Arc<Route53Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53HealthCheckConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_health_check"
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

impl AwsRoute53HealthCheckConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: route53_gen::HealthCheckTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_route53_health_check", e))?;

        let id = attrs
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let caller_reference = model
            .reference_name
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let port = attrs.get("port").and_then(|v| v.as_i64()).map(|n| n as i32);
        let failure_threshold = attrs
            .get("failure_threshold")
            .and_then(|v| v.as_i64())
            .map(|n| n as i32);
        let request_interval = attrs
            .get("request_interval")
            .and_then(|v| v.as_i64())
            .map(|n| n as i32);
        let inverted = attrs.get("inverted").and_then(|v| v.as_bool());
        let disabled = attrs.get("disabled").and_then(|v| v.as_bool());
        let measure_latency = attrs.get("measure_latency").and_then(|v| v.as_bool());
        let enable_sni = attrs.get("enable_sni").and_then(|v| v.as_bool());
        let health_threshold = attrs
            .get("child_healthchecks_threshold")
            .and_then(|v| v.as_i64())
            .map(|n| n as i32);

        let regions: Vec<String> = attrs
            .get("regions")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let child_health_checks: Vec<String> = attrs
            .get("child_health_checks")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

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

        let config = HealthCheckConfigView {
            type_: model.type_,
            ip_address: model.ip_address,
            port,
            resource_path: model.resource_path,
            fully_qualified_domain_name: model.fqdn,
            failure_threshold,
            request_interval,
            inverted,
            disabled,
            measure_latency,
            search_string: model.search_string,
            enable_sni,
            health_threshold,
            insufficient_data_health_status: model.insufficient_data_health_status,
            regions,
            child_health_checks,
        };

        let view = HealthCheckView {
            id: id.clone(),
            caller_reference,
            config,
            version: 1,
            tags,
        };

        let mut state_view = minimal_route53_state_view();
        state_view.health_checks.insert(id, view);
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
        for hc in view.health_checks.values() {
            let cfg = &hc.config;
            let attrs = serde_json::json!({
                "id": hc.id,
                "type": cfg.type_,
                "ip_address": cfg.ip_address,
                "fqdn": cfg.fully_qualified_domain_name,
                "port": cfg.port,
                "resource_path": cfg.resource_path,
                "failure_threshold": cfg.failure_threshold,
                "request_interval": cfg.request_interval,
                "inverted": cfg.inverted,
                "disabled": cfg.disabled,
                "measure_latency": cfg.measure_latency,
                "search_string": cfg.search_string,
                "enable_sni": cfg.enable_sni,
                "child_healthchecks_threshold": cfg.health_threshold,
                "insufficient_data_health_status": cfg.insufficient_data_health_status,
                "regions": cfg.regions,
                "child_health_checks": cfg.child_health_checks,
                "reference_name": hc.caller_reference,
                "tags": hc.tags,
                "tags_all": hc.tags,
            });
            results.push(ExtractedResource {
                name: hc.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_route53_hosted_zone_dnssec
// ---------------------------------------------------------------------------

/// Converts `aws_route53_hosted_zone_dnssec` Terraform resources.
///
/// Sets the `dnssec_enabled` flag for the given hosted zone based on
/// `signing_status` ("SIGNING" → true, otherwise false).
pub struct AwsRoute53HostedZoneDnssecConverter {
    service: Arc<Route53Service>,
}

impl AwsRoute53HostedZoneDnssecConverter {
    pub fn new(service: Arc<Route53Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53HostedZoneDnssecConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_hosted_zone_dnssec"
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

impl AwsRoute53HostedZoneDnssecConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: route53_gen::HostedZoneDnssecTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_route53_hosted_zone_dnssec", e))?;

        let enabled = model.signing_status == "SIGNING";

        let mut state_view = minimal_route53_state_view();
        state_view
            .dnssec_enabled
            .insert(model.hosted_zone_id, enabled);
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
        for (zone_id, enabled) in &view.dnssec_enabled {
            let status = if *enabled { "SIGNING" } else { "NOT_SIGNING" };
            let attrs = serde_json::json!({
                "id": zone_id,
                "hosted_zone_id": zone_id,
                "signing_status": status,
            });
            results.push(ExtractedResource {
                name: zone_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_route53_key_signing_key
// ---------------------------------------------------------------------------

/// Converts `aws_route53_key_signing_key` Terraform resources.
///
/// Keyed by `{hosted_zone_id}/{name}` (the natural TF id format).
pub struct AwsRoute53KeySigningKeyConverter {
    service: Arc<Route53Service>,
}

impl AwsRoute53KeySigningKeyConverter {
    pub fn new(service: Arc<Route53Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53KeySigningKeyConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_key_signing_key"
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

impl AwsRoute53KeySigningKeyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: route53_gen::KeySigningKeyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_route53_key_signing_key", e))?;

        let key = format!("{}/{}", model.hosted_zone_id, model.name);
        let view = KeySigningKeyView {
            name: model.name,
            hosted_zone_id: model.hosted_zone_id,
            kms_arn: model.key_management_service_arn,
            status: model.status,
            created_date: String::new(),
            last_modified_date: String::new(),
        };

        let mut state_view = minimal_route53_state_view();
        state_view.key_signing_keys.insert(key, view);
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
        for ksk in view.key_signing_keys.values() {
            let id = format!("{}/{}", ksk.hosted_zone_id, ksk.name);
            let attrs = serde_json::json!({
                "id": id,
                "hosted_zone_id": ksk.hosted_zone_id,
                "name": ksk.name,
                "key_management_service_arn": ksk.kms_arn,
                "status": ksk.status,
            });
            results.push(ExtractedResource {
                name: id,
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_route53_query_log
// ---------------------------------------------------------------------------

/// Converts `aws_route53_query_log` Terraform resources.
pub struct AwsRoute53QueryLogConverter {
    service: Arc<Route53Service>,
}

impl AwsRoute53QueryLogConverter {
    pub fn new(service: Arc<Route53Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53QueryLogConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_query_log"
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

impl AwsRoute53QueryLogConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: route53_gen::QueryLogTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_route53_query_log", e))?;

        let id = attrs
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let view = QueryLoggingConfigView {
            id: id.clone(),
            hosted_zone_id: model.zone_id,
            cloud_watch_logs_log_group_arn: model.cloudwatch_log_group_arn,
        };

        let mut state_view = minimal_route53_state_view();
        state_view.query_logging_configs.insert(id, view);
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
        for q in view.query_logging_configs.values() {
            let attrs = serde_json::json!({
                "id": q.id,
                "zone_id": q.hosted_zone_id,
                "cloudwatch_log_group_arn": q.cloud_watch_logs_log_group_arn,
            });
            results.push(ExtractedResource {
                name: q.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_route53_traffic_policy
// ---------------------------------------------------------------------------

/// Converts `aws_route53_traffic_policy` Terraform resources.
pub struct AwsRoute53TrafficPolicyConverter {
    service: Arc<Route53Service>,
}

impl AwsRoute53TrafficPolicyConverter {
    pub fn new(service: Arc<Route53Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53TrafficPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_traffic_policy"
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

impl AwsRoute53TrafficPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: route53_gen::TrafficPolicyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_route53_traffic_policy", e))?;

        let id = attrs
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let version: i32 = attrs
            .get("version")
            .and_then(|v| v.as_i64())
            .map(|n| n as i32)
            .unwrap_or(1);
        let type_ = model.type_.unwrap_or_else(|| "A".to_string());

        let key = format!("{}:{}", id, version);
        let view = TrafficPolicyView {
            id: id.clone(),
            name: model.name,
            version,
            document: model.document,
            comment: model.comment,
            type_,
        };

        let mut state_view = minimal_route53_state_view();
        state_view.traffic_policies.insert(key, view);
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
        for tp in view.traffic_policies.values() {
            let attrs = serde_json::json!({
                "id": tp.id,
                "name": tp.name,
                "version": tp.version,
                "document": tp.document,
                "comment": tp.comment,
                "type": tp.type_,
            });
            results.push(ExtractedResource {
                name: tp.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_route53_traffic_policy_instance
// ---------------------------------------------------------------------------

/// Converts `aws_route53_traffic_policy_instance` Terraform resources.
pub struct AwsRoute53TrafficPolicyInstanceConverter {
    service: Arc<Route53Service>,
}

impl AwsRoute53TrafficPolicyInstanceConverter {
    pub fn new(service: Arc<Route53Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53TrafficPolicyInstanceConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_traffic_policy_instance"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_route53_zone", "aws_route53_traffic_policy"]
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

impl AwsRoute53TrafficPolicyInstanceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: route53_gen::TrafficPolicyInstanceTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_route53_traffic_policy_instance", e)
            })?;

        let id = attrs
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let view = TrafficPolicyInstanceView {
            id: id.clone(),
            hosted_zone_id: model.hosted_zone_id,
            name: model.name,
            ttl: model.ttl,
            state: "Applied".to_string(),
            message: String::new(),
            traffic_policy_id: model.traffic_policy_id,
            traffic_policy_version: model.traffic_policy_version as i32,
            traffic_policy_type: "A".to_string(),
        };

        let mut state_view = minimal_route53_state_view();
        state_view.traffic_policy_instances.insert(id, view);
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
        for inst in view.traffic_policy_instances.values() {
            let attrs = serde_json::json!({
                "id": inst.id,
                "hosted_zone_id": inst.hosted_zone_id,
                "name": inst.name,
                "ttl": inst.ttl,
                "traffic_policy_id": inst.traffic_policy_id,
                "traffic_policy_version": inst.traffic_policy_version,
                "traffic_policy_type": inst.traffic_policy_type,
            });
            results.push(ExtractedResource {
                name: inst.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_route53_vpc_association_authorization
// ---------------------------------------------------------------------------

/// Converts `aws_route53_vpc_association_authorization` Terraform resources.
///
/// Keyed by `{zone_id}/{vpc_id}`.
pub struct AwsRoute53VpcAssociationAuthorizationConverter {
    service: Arc<Route53Service>,
}

impl AwsRoute53VpcAssociationAuthorizationConverter {
    pub fn new(service: Arc<Route53Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53VpcAssociationAuthorizationConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_vpc_association_authorization"
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

impl AwsRoute53VpcAssociationAuthorizationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: route53_gen::VpcAssociationAuthorizationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_route53_vpc_association_authorization", e)
            })?;

        let vpc_region = model.vpc_region.clone().unwrap_or_else(|| region.clone());
        let key = format!("{}/{}", model.zone_id, model.vpc_id);
        let view = VpcAssociationAuthorizationView {
            hosted_zone_id: model.zone_id,
            vpc_id: model.vpc_id,
            vpc_region,
        };

        let mut state_view = minimal_route53_state_view();
        state_view.vpc_association_authorizations.insert(key, view);
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
        for auth in view.vpc_association_authorizations.values() {
            let id = format!("{}/{}", auth.hosted_zone_id, auth.vpc_id);
            let attrs = serde_json::json!({
                "id": id,
                "zone_id": auth.hosted_zone_id,
                "vpc_id": auth.vpc_id,
                "vpc_region": auth.vpc_region,
            });
            results.push(ExtractedResource {
                name: id,
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_route53_zone_association
// ---------------------------------------------------------------------------

/// Converts `aws_route53_zone_association` Terraform resources.
///
/// Sub-resource modifier: snapshots the named hosted zone, pushes a fresh
/// `VpcView` onto its `vpcs` Vec (deduplicated), and restores. The TF id is
/// `{zone_id}:{vpc_id}`.
pub struct AwsRoute53ZoneAssociationConverter {
    service: Arc<Route53Service>,
}

impl AwsRoute53ZoneAssociationConverter {
    pub fn new(service: Arc<Route53Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53ZoneAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_zone_association"
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

impl AwsRoute53ZoneAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: route53_gen::ZoneAssociationTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_route53_zone_association", e))?;

        let raw_zone_id = model.zone_id.clone();
        let zone_id = if raw_zone_id.starts_with("/hostedzone/") {
            raw_zone_id
        } else {
            format!("/hostedzone/{}", raw_zone_id)
        };
        let vpc_region = model.vpc_region.clone().unwrap_or_else(|| region.clone());

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(zone) = state_view.hosted_zones.get_mut(&zone_id) {
            if !zone.vpcs.iter().any(|v| v.vpc_id == model.vpc_id) {
                zone.vpcs.push(VpcView {
                    vpc_id: model.vpc_id,
                    vpc_region,
                });
            }
        } else {
            warnings.push(format!(
                "hosted zone '{}' not found in state; association skipped",
                zone_id
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
        for zone in view.hosted_zones.values() {
            let zone_id_short = zone.id.trim_start_matches("/hostedzone/");
            for vpc in &zone.vpcs {
                let id = format!("{}:{}", zone_id_short, vpc.vpc_id);
                let attrs = serde_json::json!({
                    "id": id,
                    "zone_id": zone_id_short,
                    "vpc_id": vpc.vpc_id,
                    "vpc_region": vpc.vpc_region,
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
