//! Terraform converters for MSK (Kafka) resources.
//!
//! `ClusterTfModel` is generated from `specs/kafka.toml`. The cluster
//! ARN template (with synthesised UUID suffix), the
//! `broker_node_group_info` nested-block parsing (instance_type,
//! client_subnets, security_groups), the i32 `number_of_broker_nodes`
//! raw read, the opaque `client_authentication` /
//! `configuration_info` / `encryption_info` / `logging_info` /
//! `open_monitoring` JSON capture, and the `state` / `cluster_type` /
//! `creation_time` / `kafka_version` defaults are wired up here.
//!
//! `aws_msk_serverless_cluster` reuses the shared `ClusterView` slot via
//! `cluster_type = "SERVERLESS"`, populating `ServerlessClusterInfoView`
//! from the `vpc_config` nested-block array.
//!
//! The remaining five MSK resources (`aws_msk_cluster_policy`,
//! `aws_msk_configuration`, `aws_msk_replicator`,
//! `aws_msk_scram_secret_association`, `aws_msk_single_scram_secret_association`,
//! `aws_msk_vpc_connection`) have no dedicated state slot in
//! `winterbaume_kafka` and are wired up as validation-only converters that
//! deserialize the TfModel, emit a warning, and otherwise no-op on inject;
//! extract returns an empty vector.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::StatefulService;
use winterbaume_kafka::KafkaService;
use winterbaume_kafka::views::{
    ClusterView, KafkaStateView, ProvisionedClusterInfoView, ServerlessClusterInfoView,
    VpcConfigView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::kafka as kafka_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_msk_cluster
// ---------------------------------------------------------------------------

pub struct AwsMskClusterConverter {
    service: Arc<KafkaService>,
}

impl AwsMskClusterConverter {
    pub fn new(service: Arc<KafkaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMskClusterConverter {
    fn resource_type(&self) -> &str {
        "aws_msk_cluster"
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

impl AwsMskClusterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: kafka_gen::ClusterTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_msk_cluster", e))?;

        let attrs = &instance.attributes;

        let cluster_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:kafka:{}:{}:cluster/{}/{}",
                region,
                ctx.default_account_id,
                model.cluster_name,
                uuid::Uuid::new_v4()
            )
        });

        let kafka_version = model.kafka_version.unwrap_or_else(|| "3.5.1".to_string());

        // i32 not in spec vocabulary -- read raw.
        let number_of_broker_nodes = attrs
            .get("number_of_broker_nodes")
            .and_then(|v| v.as_i64())
            .unwrap_or(3) as i32;

        // broker_node_group_info is a nested-block array.
        let bng = attrs
            .get("broker_node_group_info")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first());

        let instance_type = bng
            .and_then(|b| b.get("instance_type"))
            .and_then(|v| v.as_str())
            .unwrap_or("kafka.m5.large")
            .to_string();

        let client_subnets: Vec<String> = bng
            .and_then(|b| b.get("client_subnets"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let security_groups: Vec<String> = bng
            .and_then(|b| b.get("security_groups"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        // Capture nested blocks as opaque JSON.
        let client_authentication = attrs.get("client_authentication").cloned();
        let configuration_info = attrs.get("configuration_info").cloned();
        let encryption_info = attrs.get("encryption_info").cloned();
        let logging_info = attrs.get("logging_info").cloned();
        let open_monitoring = attrs.get("open_monitoring").cloned();

        let cluster_view = ClusterView {
            cluster_name: model.cluster_name.clone(),
            cluster_arn: cluster_arn.clone(),
            state: "ACTIVE".to_string(),
            cluster_type: "PROVISIONED".to_string(),
            creation_time: Utc::now().to_rfc3339(),
            provisioned: Some(ProvisionedClusterInfoView {
                kafka_version,
                number_of_broker_nodes,
                instance_type,
                client_subnets,
                security_groups,
            }),
            serverless: None,
            tags: model.tags,
            client_authentication,
            configuration_info,
            encryption_info,
            logging_info,
            open_monitoring,
        };

        let mut state_view = KafkaStateView {
            clusters: HashMap::new(),
            tags: HashMap::new(),
        };
        state_view.clusters.insert(cluster_arn, cluster_view);
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
        for cluster in view.clusters.values() {
            // Skip serverless clusters; those belong to aws_msk_serverless_cluster.
            if cluster.cluster_type == "SERVERLESS" {
                continue;
            }
            // Extract cluster_uuid from ARN (last segment after /)
            let cluster_uuid = cluster.cluster_arn.rsplit('/').next().unwrap_or("");
            let mut attrs = serde_json::json!({
                "id": cluster.cluster_arn,
                "cluster_name": cluster.cluster_name,
                "arn": cluster.cluster_arn,
                "cluster_type": cluster.cluster_type,
                "creation_time": cluster.creation_time,
                "state": cluster.state,
                "tags": cluster.tags,
                "tags_all": cluster.tags,
                "kafka_version": "",
                "number_of_broker_nodes": 0,
                "enhanced_monitoring": "DEFAULT",
                "cluster_uuid": cluster_uuid,
                "client_authentication": cluster.client_authentication,
                "configuration_info": cluster.configuration_info,
                "encryption_info": cluster.encryption_info,
                "logging_info": cluster.logging_info,
                "open_monitoring": cluster.open_monitoring,
            });
            if let Some(ref prov) = cluster.provisioned {
                // Override kafka_version and number_of_broker_nodes with actual values
                let obj = attrs.as_object_mut().unwrap();
                obj.insert(
                    "kafka_version".to_string(),
                    serde_json::json!(prov.kafka_version),
                );
                obj.insert(
                    "number_of_broker_nodes".to_string(),
                    serde_json::json!(prov.number_of_broker_nodes),
                );
                obj.insert(
                    "broker_node_group_info".to_string(),
                    serde_json::json!([{
                        "instance_type": prov.instance_type,
                        "client_subnets": prov.client_subnets,
                        "security_groups": prov.security_groups,
                    }]),
                );
                // Synthesize bootstrap_brokers from cluster info
                let brokers: Vec<String> = (0..prov.number_of_broker_nodes)
                    .map(|i| {
                        format!(
                            "b-{}.{}.kafka.{}.amazonaws.com:9092",
                            i + 1,
                            cluster.cluster_name,
                            &ctx.default_region
                        )
                    })
                    .collect();
                obj.insert(
                    "bootstrap_brokers".to_string(),
                    serde_json::json!(brokers.join(",")),
                );
                let tls_brokers: Vec<String> = (0..prov.number_of_broker_nodes)
                    .map(|i| {
                        format!(
                            "b-{}.{}.kafka.{}.amazonaws.com:9094",
                            i + 1,
                            cluster.cluster_name,
                            &ctx.default_region
                        )
                    })
                    .collect();
                obj.insert(
                    "bootstrap_brokers_tls".to_string(),
                    serde_json::json!(tls_brokers.join(",")),
                );
                // Synthesize zookeeper_connect_string
                let zk: Vec<String> = (0..3)
                    .map(|i| {
                        format!(
                            "z-{}.{}.kafka.{}.amazonaws.com:2181",
                            i + 1,
                            cluster.cluster_name,
                            &ctx.default_region
                        )
                    })
                    .collect();
                obj.insert(
                    "zookeeper_connect_string".to_string(),
                    serde_json::json!(zk.join(",")),
                );
                // current_version is a hash-like string used for updates
                obj.insert(
                    "current_version".to_string(),
                    serde_json::json!(format!("K{}", prov.kafka_version.replace('.', ""))),
                );
                obj.insert(
                    "bootstrap_brokers_vpc_connectivity_tls".to_string(),
                    serde_json::json!(""),
                );
                obj.insert(
                    "zookeeper_connect_string_tls".to_string(),
                    serde_json::json!(""),
                );
            }
            results.push(ExtractedResource {
                name: cluster.cluster_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_msk_cluster_policy — no state slot
// ---------------------------------------------------------------------------

/// Converts `aws_msk_cluster_policy` resources (validation-only; no backing
/// state slot in `winterbaume_kafka` for resource-based cluster policies).
pub struct AwsMskClusterPolicyConverter {
    #[allow(dead_code)]
    service: Arc<KafkaService>,
}

impl AwsMskClusterPolicyConverter {
    pub fn new(service: Arc<KafkaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMskClusterPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_msk_cluster_policy"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsMskClusterPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: kafka_gen::ClusterPolicyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_msk_cluster_policy", e))?;
        let warn_msg = "no state slot in winterbaume_kafka for cluster policies; inject is a no-op"
            .to_string();
        eprintln!("warning: aws_msk_cluster_policy: {warn_msg}");
        Ok(ConversionResult {
            region,
            warnings: vec![format!("aws_msk_cluster_policy: {warn_msg}")],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_msk_configuration — no state slot
// ---------------------------------------------------------------------------

/// Converts `aws_msk_configuration` resources (validation-only; no backing
/// state slot in `winterbaume_kafka` for standalone MSK configurations).
pub struct AwsMskConfigurationConverter {
    #[allow(dead_code)]
    service: Arc<KafkaService>,
}

impl AwsMskConfigurationConverter {
    pub fn new(service: Arc<KafkaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMskConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_msk_configuration"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsMskConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: kafka_gen::ConfigurationTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_msk_configuration", e))?;
        let warn_msg =
            "no state slot in winterbaume_kafka for standalone MSK configurations; inject is a \
             no-op"
                .to_string();
        eprintln!("warning: aws_msk_configuration: {warn_msg}");
        Ok(ConversionResult {
            region,
            warnings: vec![format!("aws_msk_configuration: {warn_msg}")],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_msk_replicator — no state slot
// ---------------------------------------------------------------------------

/// Converts `aws_msk_replicator` resources (validation-only; no backing state
/// slot in `winterbaume_kafka` for MSK replicators).
pub struct AwsMskReplicatorConverter {
    #[allow(dead_code)]
    service: Arc<KafkaService>,
}

impl AwsMskReplicatorConverter {
    pub fn new(service: Arc<KafkaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMskReplicatorConverter {
    fn resource_type(&self) -> &str {
        "aws_msk_replicator"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsMskReplicatorConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: kafka_gen::ReplicatorTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_msk_replicator", e))?;
        let warn_msg =
            "no state slot in winterbaume_kafka for replicators; inject is a no-op".to_string();
        eprintln!("warning: aws_msk_replicator: {warn_msg}");
        Ok(ConversionResult {
            region,
            warnings: vec![format!("aws_msk_replicator: {warn_msg}")],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_msk_scram_secret_association — no state slot
// ---------------------------------------------------------------------------

/// Converts `aws_msk_scram_secret_association` resources (validation-only;
/// SCRAM secret associations have no backing state slot in `winterbaume_kafka`).
pub struct AwsMskScramSecretAssociationConverter {
    #[allow(dead_code)]
    service: Arc<KafkaService>,
}

impl AwsMskScramSecretAssociationConverter {
    pub fn new(service: Arc<KafkaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMskScramSecretAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_msk_scram_secret_association"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsMskScramSecretAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: kafka_gen::ScramSecretAssociationTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_msk_scram_secret_association", e))?;
        let warn_msg =
            "no state slot in winterbaume_kafka for SCRAM secret associations; inject is a no-op"
                .to_string();
        eprintln!("warning: aws_msk_scram_secret_association: {warn_msg}");
        Ok(ConversionResult {
            region,
            warnings: vec![format!("aws_msk_scram_secret_association: {warn_msg}")],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_msk_serverless_cluster
// ---------------------------------------------------------------------------

pub struct AwsMskServerlessClusterConverter {
    service: Arc<KafkaService>,
}

impl AwsMskServerlessClusterConverter {
    pub fn new(service: Arc<KafkaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMskServerlessClusterConverter {
    fn resource_type(&self) -> &str {
        "aws_msk_serverless_cluster"
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

impl AwsMskServerlessClusterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: kafka_gen::ServerlessClusterTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_msk_serverless_cluster", e))?;

        let attrs = &instance.attributes;

        let cluster_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:kafka:{}:{}:cluster/{}/{}",
                region,
                ctx.default_account_id,
                model.cluster_name,
                uuid::Uuid::new_v4()
            )
        });

        // vpc_config is a nested-block array.
        let vpc_configs: Vec<VpcConfigView> = attrs
            .get("vpc_config")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|block| {
                        let subnet_ids: Vec<String> = block
                            .get("subnet_ids")
                            .and_then(|v| v.as_array())
                            .map(|a| {
                                a.iter()
                                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                    .collect()
                            })
                            .unwrap_or_default();
                        let security_group_ids: Vec<String> = block
                            .get("security_group_ids")
                            .and_then(|v| v.as_array())
                            .map(|a| {
                                a.iter()
                                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                    .collect()
                            })
                            .unwrap_or_default();
                        VpcConfigView {
                            subnet_ids,
                            security_group_ids,
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Capture the client_authentication block as opaque JSON.
        let client_authentication = attrs.get("client_authentication").cloned();

        let cluster_view = ClusterView {
            cluster_name: model.cluster_name.clone(),
            cluster_arn: cluster_arn.clone(),
            state: "ACTIVE".to_string(),
            cluster_type: "SERVERLESS".to_string(),
            creation_time: Utc::now().to_rfc3339(),
            provisioned: None,
            serverless: Some(ServerlessClusterInfoView { vpc_configs }),
            tags: model.tags,
            client_authentication,
            configuration_info: None,
            encryption_info: None,
            logging_info: None,
            open_monitoring: None,
        };

        let mut state_view = KafkaStateView {
            clusters: HashMap::new(),
            tags: HashMap::new(),
        };
        state_view.clusters.insert(cluster_arn, cluster_view);
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
        for cluster in view.clusters.values() {
            if cluster.cluster_type != "SERVERLESS" {
                continue;
            }
            let cluster_uuid = cluster.cluster_arn.rsplit('/').next().unwrap_or("");
            let mut attrs = serde_json::json!({
                "id": cluster.cluster_arn,
                "cluster_name": cluster.cluster_name,
                "arn": cluster.cluster_arn,
                "tags": cluster.tags,
                "tags_all": cluster.tags,
                "cluster_uuid": cluster_uuid,
                "bootstrap_brokers_sasl_iam": format!(
                    "boot-{}.c1.kafka-serverless.{}.amazonaws.com:9098",
                    cluster_uuid, &ctx.default_region
                ),
                "client_authentication": cluster.client_authentication,
            });
            if let Some(ref sv) = cluster.serverless {
                let vpc_config_blocks: Vec<serde_json::Value> = sv
                    .vpc_configs
                    .iter()
                    .map(|c| {
                        serde_json::json!({
                            "subnet_ids": c.subnet_ids,
                            "security_group_ids": c.security_group_ids,
                        })
                    })
                    .collect();
                let obj = attrs.as_object_mut().unwrap();
                obj.insert(
                    "vpc_config".to_string(),
                    serde_json::json!(vpc_config_blocks),
                );
            }
            results.push(ExtractedResource {
                name: cluster.cluster_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_msk_single_scram_secret_association — no state slot
// ---------------------------------------------------------------------------

/// Converts `aws_msk_single_scram_secret_association` resources
/// (validation-only; SCRAM secret associations have no backing state slot in
/// `winterbaume_kafka`).
pub struct AwsMskSingleScramSecretAssociationConverter {
    #[allow(dead_code)]
    service: Arc<KafkaService>,
}

impl AwsMskSingleScramSecretAssociationConverter {
    pub fn new(service: Arc<KafkaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMskSingleScramSecretAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_msk_single_scram_secret_association"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsMskSingleScramSecretAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: kafka_gen::SingleScramSecretAssociationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_msk_single_scram_secret_association", e)
            })?;
        let warn_msg =
            "no state slot in winterbaume_kafka for single SCRAM secret associations; inject is \
             a no-op"
                .to_string();
        eprintln!("warning: aws_msk_single_scram_secret_association: {warn_msg}");
        Ok(ConversionResult {
            region,
            warnings: vec![format!(
                "aws_msk_single_scram_secret_association: {warn_msg}"
            )],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_msk_vpc_connection — no state slot
// ---------------------------------------------------------------------------

/// Converts `aws_msk_vpc_connection` resources (validation-only; VPC
/// connections have no backing state slot in `winterbaume_kafka`).
pub struct AwsMskVpcConnectionConverter {
    #[allow(dead_code)]
    service: Arc<KafkaService>,
}

impl AwsMskVpcConnectionConverter {
    pub fn new(service: Arc<KafkaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMskVpcConnectionConverter {
    fn resource_type(&self) -> &str {
        "aws_msk_vpc_connection"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsMskVpcConnectionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: kafka_gen::VpcConnectionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_msk_vpc_connection", e))?;
        let warn_msg =
            "no state slot in winterbaume_kafka for VPC connections; inject is a no-op".to_string();
        eprintln!("warning: aws_msk_vpc_connection: {warn_msg}");
        Ok(ConversionResult {
            region,
            warnings: vec![format!("aws_msk_vpc_connection: {warn_msg}")],
        })
    }
}
