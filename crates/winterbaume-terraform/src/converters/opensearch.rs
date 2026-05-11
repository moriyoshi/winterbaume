//! Terraform converters for OpenSearch resources.
//!
//! The `*TfModel` types are generated from `specs/opensearch.toml`. The
//! ARN template, the synthesised `domain_id`, the `engine_version`
//! default, the nested `cluster_config` / `ebs_options` blocks, and
//! the `created`/`deleted`/`processing` constants for the domain
//! converter are wired up here. For the eight additional resources
//! (authorize_vpc_endpoint_access, domain_policy, domain_saml_options,
//! inbound_connection_accepter, outbound_connection, package,
//! package_association, vpc_endpoint) the service-specific shapes
//! (principal/principal_type derivation, access_policies overlay on an
//! existing domain, package id synthesis, etc.) are wired up here. The
//! `domain_saml_options` converter is a no-op state-wise because SAML
//! options are not modelled in `OpenSearchState`.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_opensearch::OpenSearchService;
use winterbaume_opensearch::views::{
    AuthorizedPrincipalView, DomainView, InboundConnectionView, OpenSearchStateView,
    OutboundConnectionView, PackageView, VpcEndpointView,
};
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
        let model =
            serde_json::from_value::<opensearch_gen::DomainTfModel>(instance.attributes.clone())
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

        let mut state_view = OpenSearchStateView::default();
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

// ---------------------------------------------------------------------------
// aws_opensearch_authorize_vpc_endpoint_access
// ---------------------------------------------------------------------------

pub struct AwsOpensearchAuthorizeVpcEndpointAccessConverter {
    service: Arc<OpenSearchService>,
}

impl AwsOpensearchAuthorizeVpcEndpointAccessConverter {
    pub fn new(service: Arc<OpenSearchService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsOpensearchAuthorizeVpcEndpointAccessConverter {
    fn resource_type(&self) -> &str {
        "aws_opensearch_authorize_vpc_endpoint_access"
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

impl AwsOpensearchAuthorizeVpcEndpointAccessConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model = serde_json::from_value::<opensearch_gen::AuthorizeVpcEndpointAccessTfModel>(
            instance.attributes.clone(),
        )
        .map_err(|e| {
            classify_deserialize_error("aws_opensearch_authorize_vpc_endpoint_access", e)
        })?;

        let account = model
            .account
            .clone()
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let principal = format!("arn:aws:iam::{account}:root");

        let view = AuthorizedPrincipalView {
            principal,
            principal_type: "AWS_ACCOUNT".to_string(),
        };

        let mut state_view = OpenSearchStateView::default();
        state_view
            .vpc_authorized_principals
            .insert(model.domain_name, vec![view]);
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
        for (domain_name, principals) in view.vpc_authorized_principals.iter() {
            for principal in principals {
                let account = principal
                    .principal
                    .strip_prefix("arn:aws:iam::")
                    .and_then(|rest| rest.split(':').next())
                    .unwrap_or("")
                    .to_string();
                let id = format!("{domain_name}/{account}");
                let attrs = serde_json::json!({
                    "id": id,
                    "domain_name": domain_name,
                    "account": account,
                    "authorized_principal": [{
                        "principal": principal.principal,
                        "principal_type": principal.principal_type,
                    }],
                });
                results.push(ExtractedResource {
                    name: id.clone(),
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
// aws_opensearch_domain_policy
// ---------------------------------------------------------------------------

pub struct AwsOpensearchDomainPolicyConverter {
    service: Arc<OpenSearchService>,
}

impl AwsOpensearchDomainPolicyConverter {
    pub fn new(service: Arc<OpenSearchService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsOpensearchDomainPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_opensearch_domain_policy"
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

impl AwsOpensearchDomainPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model = serde_json::from_value::<opensearch_gen::DomainPolicyTfModel>(
            instance.attributes.clone(),
        )
        .map_err(|e| classify_deserialize_error("aws_opensearch_domain_policy", e))?;

        // Merge access_policies onto an existing domain if there is one;
        // otherwise synthesise a minimal placeholder domain view so the
        // policy resource has somewhere to live after re-extraction.
        let existing = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await
            .domains
            .get(&model.domain_name)
            .cloned();

        let access_policies = model.access_policies.unwrap_or_default();
        let domain_view = match existing {
            Some(current) => DomainView {
                access_policies,
                ..current
            },
            None => DomainView {
                domain_name: model.domain_name.clone(),
                arn: format!(
                    "arn:aws:es:{}:{}:domain/{}",
                    region, ctx.default_account_id, model.domain_name
                ),
                domain_id: format!("{}/{}", ctx.default_account_id, model.domain_name),
                endpoint: None,
                engine_version: "OpenSearch_2.11".to_string(),
                created: true,
                deleted: false,
                processing: false,
                instance_type: "t3.small.search".to_string(),
                instance_count: 1,
                dedicated_master_enabled: false,
                zone_awareness_enabled: false,
                ebs_enabled: true,
                ebs_volume_size: 10,
                ebs_volume_type: "gp2".to_string(),
                access_policies,
                tags: HashMap::new(),
            },
        };

        let mut state_view = OpenSearchStateView::default();
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
            if domain.access_policies.is_empty() {
                continue;
            }
            let attrs = serde_json::json!({
                "id": format!("esd-policy-{}", domain.domain_name),
                "domain_name": domain.domain_name,
                "access_policies": domain.access_policies,
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

// ---------------------------------------------------------------------------
// aws_opensearch_domain_saml_options
// ---------------------------------------------------------------------------

pub struct AwsOpensearchDomainSamlOptionsConverter {
    service: Arc<OpenSearchService>,
}

impl AwsOpensearchDomainSamlOptionsConverter {
    pub fn new(service: Arc<OpenSearchService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsOpensearchDomainSamlOptionsConverter {
    fn resource_type(&self) -> &str {
        "aws_opensearch_domain_saml_options"
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

impl AwsOpensearchDomainSamlOptionsConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        // SAML options are not modelled in OpenSearchState, so this converter
        // only validates the TF shape and emits a warning. The state mutation
        // is a no-op apart from a placeholder domain entry if the referenced
        // domain has not been seen yet.
        let model = serde_json::from_value::<opensearch_gen::DomainSamlOptionsTfModel>(
            instance.attributes.clone(),
        )
        .map_err(|e| classify_deserialize_error("aws_opensearch_domain_saml_options", e))?;

        let existing = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await
            .domains
            .get(&model.domain_name)
            .cloned();

        if existing.is_none() {
            let domain_view = DomainView {
                domain_name: model.domain_name.clone(),
                arn: format!(
                    "arn:aws:es:{}:{}:domain/{}",
                    region, ctx.default_account_id, model.domain_name
                ),
                domain_id: format!("{}/{}", ctx.default_account_id, model.domain_name),
                endpoint: None,
                engine_version: "OpenSearch_2.11".to_string(),
                created: true,
                deleted: false,
                processing: false,
                instance_type: "t3.small.search".to_string(),
                instance_count: 1,
                dedicated_master_enabled: false,
                zone_awareness_enabled: false,
                ebs_enabled: true,
                ebs_volume_size: 10,
                ebs_volume_type: "gp2".to_string(),
                access_policies: String::new(),
                tags: HashMap::new(),
            };
            let mut state_view = OpenSearchStateView::default();
            state_view.domains.insert(model.domain_name, domain_view);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;
        }

        Ok(ConversionResult {
            region,
            warnings: vec![
                "aws_opensearch_domain_saml_options: SAML options are not persisted in winterbaume state"
                    .to_string(),
            ],
        })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // SAML options are not stored in state, so nothing to extract.
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_opensearch_inbound_connection_accepter
// ---------------------------------------------------------------------------

pub struct AwsOpensearchInboundConnectionAccepterConverter {
    service: Arc<OpenSearchService>,
}

impl AwsOpensearchInboundConnectionAccepterConverter {
    pub fn new(service: Arc<OpenSearchService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsOpensearchInboundConnectionAccepterConverter {
    fn resource_type(&self) -> &str {
        "aws_opensearch_inbound_connection_accepter"
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

impl AwsOpensearchInboundConnectionAccepterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model = serde_json::from_value::<opensearch_gen::InboundConnectionAccepterTfModel>(
            instance.attributes.clone(),
        )
        .map_err(|e| classify_deserialize_error("aws_opensearch_inbound_connection_accepter", e))?;

        // Look up the existing inbound connection if any, then re-insert it
        // with status ACTIVE. If there is no matching connection, synthesise
        // a minimal placeholder so the resource id survives a round trip.
        let existing = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await
            .inbound_connections
            .get(&model.connection_id)
            .cloned();

        let view = match existing {
            Some(current) => InboundConnectionView {
                status_code: "ACTIVE".to_string(),
                ..current
            },
            None => InboundConnectionView {
                connection_id: model.connection_id.clone(),
                connection_mode: None,
                status_code: "ACTIVE".to_string(),
                local_domain_name: String::new(),
                local_owner_id: ctx.default_account_id.clone(),
                local_region: Some(region.clone()),
                remote_domain_name: String::new(),
                remote_owner_id: None,
                remote_region: None,
            },
        };

        let mut state_view = OpenSearchStateView::default();
        state_view
            .inbound_connections
            .insert(model.connection_id, view);
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
        for conn in view.inbound_connections.values() {
            if conn.status_code != "ACTIVE" {
                continue;
            }
            let attrs = serde_json::json!({
                "id": conn.connection_id,
                "connection_id": conn.connection_id,
                "connection_status": conn.status_code,
            });
            results.push(ExtractedResource {
                name: conn.connection_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_opensearch_outbound_connection
// ---------------------------------------------------------------------------

pub struct AwsOpensearchOutboundConnectionConverter {
    service: Arc<OpenSearchService>,
}

impl AwsOpensearchOutboundConnectionConverter {
    pub fn new(service: Arc<OpenSearchService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsOpensearchOutboundConnectionConverter {
    fn resource_type(&self) -> &str {
        "aws_opensearch_outbound_connection"
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

impl AwsOpensearchOutboundConnectionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model =
            serde_json::from_value::<opensearch_gen::OutboundConnectionTfModel>(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_opensearch_outbound_connection", e))?;

        // connection_id can be supplied by Terraform after creation; if absent,
        // synthesise a deterministic id from the alias so re-injection is idempotent.
        let connection_id = attrs
            .get("connection_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("cc-{}", model.connection_alias));

        // local_domain_info / remote_domain_info nested blocks
        let local_info = attrs.get("local_domain_info").and_then(|v| v.as_array());
        let local_domain_name = local_info
            .and_then(|arr| arr.first())
            .and_then(|i| i.get("domain_name"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let local_owner_id = local_info
            .and_then(|arr| arr.first())
            .and_then(|i| i.get("owner_id"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let local_region = local_info
            .and_then(|arr| arr.first())
            .and_then(|i| i.get("region"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .or_else(|| Some(region.clone()));

        let remote_info = attrs.get("remote_domain_info").and_then(|v| v.as_array());
        let remote_domain_name = remote_info
            .and_then(|arr| arr.first())
            .and_then(|i| i.get("domain_name"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let remote_owner_id = remote_info
            .and_then(|arr| arr.first())
            .and_then(|i| i.get("owner_id"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let remote_region = remote_info
            .and_then(|arr| arr.first())
            .and_then(|i| i.get("region"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let status_code = attrs
            .get("connection_status")
            .and_then(|v| v.as_str())
            .unwrap_or("PENDING_ACCEPTANCE")
            .to_string();

        let view = OutboundConnectionView {
            connection_id: connection_id.clone(),
            connection_alias: model.connection_alias,
            connection_mode: model.connection_mode,
            status_code,
            local_domain_name,
            local_owner_id,
            local_region,
            remote_domain_name,
            remote_owner_id,
            remote_region,
        };

        let mut state_view = OpenSearchStateView::default();
        state_view.outbound_connections.insert(connection_id, view);
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
        for conn in view.outbound_connections.values() {
            let attrs = serde_json::json!({
                "id": conn.connection_id,
                "connection_id": conn.connection_id,
                "connection_alias": conn.connection_alias,
                "connection_mode": conn.connection_mode,
                "connection_status": conn.status_code,
                "local_domain_info": [{
                    "domain_name": conn.local_domain_name,
                    "owner_id": conn.local_owner_id,
                    "region": conn.local_region,
                }],
                "remote_domain_info": [{
                    "domain_name": conn.remote_domain_name,
                    "owner_id": conn.remote_owner_id,
                    "region": conn.remote_region,
                }],
            });
            results.push(ExtractedResource {
                name: conn.connection_alias.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_opensearch_package
// ---------------------------------------------------------------------------

pub struct AwsOpensearchPackageConverter {
    service: Arc<OpenSearchService>,
}

impl AwsOpensearchPackageConverter {
    pub fn new(service: Arc<OpenSearchService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsOpensearchPackageConverter {
    fn resource_type(&self) -> &str {
        "aws_opensearch_package"
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

impl AwsOpensearchPackageConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model = serde_json::from_value::<opensearch_gen::PackageTfModel>(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_opensearch_package", e))?;

        // package_id can be supplied by Terraform after creation; if absent,
        // synthesise a deterministic id so re-injection is idempotent.
        let package_id = attrs
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("pkg-{}", model.package_name));

        let view = PackageView {
            package_id: package_id.clone(),
            package_name: model.package_name,
            package_type: model.package_type,
            package_description: model.package_description,
            engine_version: model.engine_version,
            status: "AVAILABLE".to_string(),
            created_at: 0.0,
            last_updated_at: 0.0,
            associated_domains: vec![],
        };

        let mut state_view = OpenSearchStateView::default();
        state_view.packages.insert(package_id, view);
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
        for pkg in view.packages.values() {
            let attrs = serde_json::json!({
                "id": pkg.package_id,
                "package_id": pkg.package_id,
                "package_name": pkg.package_name,
                "package_type": pkg.package_type,
                "package_description": pkg.package_description,
                "engine_version": pkg.engine_version,
                "available_package_version": "",
            });
            results.push(ExtractedResource {
                name: pkg.package_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_opensearch_package_association
// ---------------------------------------------------------------------------

pub struct AwsOpensearchPackageAssociationConverter {
    service: Arc<OpenSearchService>,
}

impl AwsOpensearchPackageAssociationConverter {
    pub fn new(service: Arc<OpenSearchService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsOpensearchPackageAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_opensearch_package_association"
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

impl AwsOpensearchPackageAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model =
            serde_json::from_value::<opensearch_gen::PackageAssociationTfModel>(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_opensearch_package_association", e))?;

        // Append the domain to the existing package's associated_domains list,
        // synthesising a minimal package entry if no such package exists yet.
        let existing = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await
            .packages
            .get(&model.package_id)
            .cloned();

        let view = match existing {
            Some(current) => {
                let mut domains = current.associated_domains.clone();
                if !domains.contains(&model.domain_name) {
                    domains.push(model.domain_name.clone());
                }
                PackageView {
                    associated_domains: domains,
                    ..current
                }
            }
            None => PackageView {
                package_id: model.package_id.clone(),
                package_name: model.package_id.clone(),
                package_type: "TXT-DICTIONARY".to_string(),
                package_description: None,
                engine_version: None,
                status: "AVAILABLE".to_string(),
                created_at: 0.0,
                last_updated_at: 0.0,
                associated_domains: vec![model.domain_name.clone()],
            },
        };

        let mut state_view = OpenSearchStateView::default();
        state_view.packages.insert(model.package_id, view);
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
        for pkg in view.packages.values() {
            for domain_name in &pkg.associated_domains {
                let id = format!("{}-{}", pkg.package_id, domain_name);
                let attrs = serde_json::json!({
                    "id": id,
                    "package_id": pkg.package_id,
                    "domain_name": domain_name,
                });
                results.push(ExtractedResource {
                    name: id.clone(),
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
// aws_opensearch_vpc_endpoint
// ---------------------------------------------------------------------------

pub struct AwsOpensearchVpcEndpointConverter {
    service: Arc<OpenSearchService>,
}

impl AwsOpensearchVpcEndpointConverter {
    pub fn new(service: Arc<OpenSearchService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsOpensearchVpcEndpointConverter {
    fn resource_type(&self) -> &str {
        "aws_opensearch_vpc_endpoint"
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

impl AwsOpensearchVpcEndpointConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model = serde_json::from_value::<opensearch_gen::VpcEndpointTfModel>(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_opensearch_vpc_endpoint", e))?;

        let vpc_endpoint_id = attrs
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                // Synthesise deterministic id from domain ARN suffix.
                let suffix = model.domain_arn.rsplit('/').next().unwrap_or("unknown");
                format!("aos-{suffix}")
            });

        // vpc_options nested block
        let vpc_options = attrs.get("vpc_options").and_then(|v| v.as_array());
        let subnet_ids = vpc_options
            .and_then(|arr| arr.first())
            .and_then(|o| o.get("subnet_ids"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let security_group_ids = vpc_options
            .and_then(|arr| arr.first())
            .and_then(|o| o.get("security_group_ids"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let endpoint = attrs
            .get("endpoint")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let view = VpcEndpointView {
            vpc_endpoint_id: vpc_endpoint_id.clone(),
            vpc_endpoint_owner: ctx.default_account_id.clone(),
            domain_arn: model.domain_arn,
            subnet_ids,
            security_group_ids,
            status: "ACTIVE".to_string(),
            endpoint,
        };

        let mut state_view = OpenSearchStateView::default();
        state_view.vpc_endpoints.insert(vpc_endpoint_id, view);
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
        for ep in view.vpc_endpoints.values() {
            let attrs = serde_json::json!({
                "id": ep.vpc_endpoint_id,
                "vpc_endpoint_id": ep.vpc_endpoint_id,
                "vpc_endpoint_owner": ep.vpc_endpoint_owner,
                "domain_arn": ep.domain_arn,
                "endpoint": ep.endpoint,
                "vpc_options": [{
                    "subnet_ids": ep.subnet_ids,
                    "security_group_ids": ep.security_group_ids,
                }],
            });
            results.push(ExtractedResource {
                name: ep.vpc_endpoint_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
