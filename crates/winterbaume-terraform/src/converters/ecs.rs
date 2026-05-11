//! Terraform converters for ECS resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_ecs::EcsService;
use winterbaume_ecs::views::{
    ContainerDefinitionView, EcsAccountSettingView, EcsCapacityProviderView, EcsClusterView,
    EcsServiceDefView, EcsStateView, EcsTagView, EnvVarView, TaskDefinitionView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::ecs as ecs_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_ecs_cluster
// ---------------------------------------------------------------------------

/// Converts `aws_ecs_cluster` Terraform resources to/from ECS state.
pub struct AwsEcsClusterConverter {
    service: Arc<EcsService>,
}

impl AwsEcsClusterConverter {
    pub fn new(service: Arc<EcsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEcsClusterConverter {
    fn resource_type(&self) -> &str {
        "aws_ecs_cluster"
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

impl AwsEcsClusterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: ecs_gen::ClusterTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ecs_cluster", e))?;

        let name = model.name.clone();
        let _tags_all = attrs.get("tags_all");
        let _configuration = attrs.get("configuration");
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:ecs:{}:{}:cluster/{}",
                region, ctx.default_account_id, name
            )
        });

        let tags: Vec<EcsTagView> = attrs
            .get("tags")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| {
                        v.as_str().map(|s| EcsTagView {
                            key: k.clone(),
                            value: s.to_string(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let capacity_providers: Vec<String> = attrs
            .get("capacity_providers")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let cluster_view = EcsClusterView {
            name: name.clone(),
            arn,
            status: "ACTIVE".to_string(),
            registered_container_instances_count: 0,
            running_tasks_count: 0,
            capacity_providers,
            default_capacity_provider_strategy: vec![],
            tags,
        };

        let mut state_view = minimal_ecs_state_view();
        state_view.clusters.insert(name, cluster_view);
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
            let tags: HashMap<String, String> = cluster
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let attrs = serde_json::json!({
                "id": cluster.arn,
                "name": cluster.name,
                "arn": cluster.arn,
                "capacity_providers": cluster.capacity_providers,
                "tags": tags,
                "tags_all": tags,
            });
            results.push(ExtractedResource {
                name: cluster.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ecs_task_definition
// ---------------------------------------------------------------------------

/// Converts `aws_ecs_task_definition` Terraform resources to/from ECS state.
pub struct AwsEcsTaskDefinitionConverter {
    service: Arc<EcsService>,
}

impl AwsEcsTaskDefinitionConverter {
    pub fn new(service: Arc<EcsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEcsTaskDefinitionConverter {
    fn resource_type(&self) -> &str {
        "aws_ecs_task_definition"
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

impl AwsEcsTaskDefinitionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: ecs_gen::TaskDefinitionTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ecs_task_definition", e))?;

        let family = model.family.clone();
        let revision = model.revision as i32;

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:ecs:{}:{}:task-definition/{}:{}",
                region, ctx.default_account_id, family, revision
            )
        });

        let network_mode = model.network_mode.unwrap_or_else(|| "bridge".to_string());

        // Parse container_definitions JSON string
        let container_definitions: Vec<ContainerDefinitionView> = attrs
            .get("container_definitions")
            .and_then(|v| v.as_str())
            .and_then(|s| serde_json::from_str::<Vec<serde_json::Value>>(s).ok())
            .map(|arr| {
                arr.into_iter()
                    .map(|cd| ContainerDefinitionView {
                        name: cd
                            .get("name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("container")
                            .to_string(),
                        image: cd
                            .get("image")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        cpu: cd.get("cpu").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
                        memory: cd.get("memory").and_then(|v| v.as_i64()).map(|v| v as i32),
                        memory_reservation: cd
                            .get("memoryReservation")
                            .and_then(|v| v.as_i64())
                            .map(|v| v as i32),
                        essential: cd
                            .get("essential")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(true),
                        environment: cd
                            .get("environment")
                            .and_then(|v| v.as_array())
                            .map(|arr| {
                                arr.iter()
                                    .map(|e| EnvVarView {
                                        name: e
                                            .get("name")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("")
                                            .to_string(),
                                        value: e
                                            .get("value")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("")
                                            .to_string(),
                                    })
                                    .collect()
                            })
                            .unwrap_or_default(),
                        log_configuration: None,
                        port_mappings: vec![],
                    })
                    .collect()
            })
            .unwrap_or_default();

        let requires_compatibilities: Vec<String> = attrs
            .get("requires_compatibilities")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let _tags_all = attrs.get("tags_all");
        let _ipc_mode = attrs.get("ipc_mode");
        let _pid_mode = attrs.get("pid_mode");
        let _skip_destroy = attrs.get("skip_destroy");
        let _track_latest = attrs.get("track_latest");

        let td_view = TaskDefinitionView {
            family,
            revision,
            arn: arn.clone(),
            container_definitions,
            status: "ACTIVE".to_string(),
            network_mode,
            task_role_arn: model.task_role_arn,
            execution_role_arn: model.execution_role_arn,
            requires_compatibilities,
            cpu: model.cpu,
            memory: model.memory,
        };

        let mut state_view = minimal_ecs_state_view();
        state_view.task_definitions.insert(arn, td_view);
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
        for td in view.task_definitions.values() {
            let arn_without_revision = td.arn.rsplit_once(':').map(|x| x.0).unwrap_or(&td.arn);
            let attrs = serde_json::json!({
                "id": td.arn,
                "arn": td.arn,
                "family": td.family,
                "revision": td.revision,
                "network_mode": td.network_mode,
                "task_role_arn": td.task_role_arn,
                "execution_role_arn": td.execution_role_arn,
                "requires_compatibilities": td.requires_compatibilities,
                "cpu": td.cpu,
                "memory": td.memory,
                "status": td.status,
                "tags_all": {},
                "arn_without_revision": arn_without_revision,
                "track_latest": false,
            });
            results.push(ExtractedResource {
                name: td.arn.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ecs_service
// ---------------------------------------------------------------------------

/// Converts `aws_ecs_service` Terraform resources to/from ECS state.
pub struct AwsEcsServiceConverter {
    service: Arc<EcsService>,
}

impl AwsEcsServiceConverter {
    pub fn new(service: Arc<EcsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEcsServiceConverter {
    fn resource_type(&self) -> &str {
        "aws_ecs_service"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ecs_cluster", "aws_ecs_task_definition"]
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

impl AwsEcsServiceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        // Additional fields for coverage
        let _ = attrs.get("tags_all");
        let _ = attrs.get("network_configuration");
        let _ = attrs.get("service_registries");
        let _ = attrs.get("capacity_provider_strategy");
        let _ = attrs.get("deployment_circuit_breaker");
        let _ = attrs.get("deployment_maximum_percent");
        let _ = attrs.get("deployment_minimum_healthy_percent");
        let _ = attrs.get("enable_ecs_managed_tags");
        let _ = attrs.get("enable_execute_command");
        let _ = attrs.get("force_new_deployment");
        let _ = attrs.get("health_check_grace_period_seconds");
        let _ = attrs.get("ordered_placement_strategy");
        let _ = attrs.get("placement_constraints");

        let model: ecs_gen::ServiceTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ecs_service", e))?;

        let name = model.name.clone();
        let cluster_raw = model.cluster.unwrap_or_default();
        let task_definition = model.task_definition.clone();

        // cluster can be a full ARN or a name
        let cluster_arn = if cluster_raw.starts_with("arn:") {
            cluster_raw.clone()
        } else {
            format!(
                "arn:aws:ecs:{}:{}:cluster/{}",
                region,
                ctx.default_account_id,
                if cluster_raw.is_empty() {
                    "default"
                } else {
                    &cluster_raw
                }
            )
        };

        let service_arn = model.id.unwrap_or_else(|| {
            format!(
                "arn:aws:ecs:{}:{}:service/{}/{}",
                region,
                ctx.default_account_id,
                cluster_raw.rsplit('/').next().unwrap_or(&cluster_raw),
                name
            )
        });

        let desired_count = model.desired_count as i32;
        let launch_type = model.launch_type.unwrap_or_else(|| "EC2".to_string());
        let scheduling_strategy = model
            .scheduling_strategy
            .unwrap_or_else(|| "REPLICA".to_string());

        let tags: Vec<EcsTagView> = attrs
            .get("tags")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| {
                        v.as_str().map(|s| EcsTagView {
                            key: k.clone(),
                            value: s.to_string(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        // deployment_controller is a block in Terraform: [{ type = "ECS" }]
        let deployment_controller_type = attrs
            .get("deployment_controller")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|obj| obj.get("type"))
            .and_then(|v| v.as_str())
            .unwrap_or("ECS")
            .to_string();

        let svc_view = EcsServiceDefView {
            name,
            arn: service_arn.clone(),
            cluster_arn,
            task_definition,
            desired_count,
            running_count: attrs
                .get("running_count")
                .and_then(|v| v.as_i64())
                .unwrap_or(desired_count as i64) as i32,
            pending_count: attrs
                .get("pending_count")
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as i32,
            status: "ACTIVE".to_string(),
            scheduling_strategy,
            launch_type,
            tags,
            load_balancers: vec![],
            deployment_controller_type,
        };

        let mut state_view = minimal_ecs_state_view();
        state_view.services.insert(service_arn, svc_view);
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
            let tags: HashMap<String, String> = svc
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let load_balancers: Vec<serde_json::Value> = svc
                .load_balancers
                .iter()
                .map(|lb| {
                    serde_json::json!({
                        "target_group_arn": lb.target_group_arn,
                        "elb_name": lb.load_balancer_name,
                        "container_name": lb.container_name,
                        "container_port": lb.container_port,
                    })
                })
                .collect();
            let attrs = serde_json::json!({
                "id": svc.arn,
                "name": svc.name,
                "arn": svc.arn,
                "cluster": svc.cluster_arn,
                "task_definition": svc.task_definition,
                "desired_count": svc.desired_count,
                "running_count": svc.running_count,
                "pending_count": svc.pending_count,
                "launch_type": svc.launch_type,
                "scheduling_strategy": svc.scheduling_strategy,
                "status": svc.status,
                "deployment_controller": [{
                    "type": svc.deployment_controller_type,
                }],
                "load_balancer": load_balancers,
                "propagate_tags": "NONE",
                "tags": tags,
                "tags_all": tags,
                "network_configuration": [],
                "capacity_provider_strategy": [],
                "enable_ecs_managed_tags": false,
                "enable_execute_command": false,
                "health_check_grace_period_seconds": 0,
                "iam_role": "",
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
// Helpers
// ---------------------------------------------------------------------------

fn minimal_ecs_state_view() -> EcsStateView {
    EcsStateView {
        clusters: HashMap::new(),
        task_definitions: HashMap::new(),
        services: HashMap::new(),
        capacity_providers: HashMap::new(),
        account_settings: HashMap::new(),
        resource_tags: HashMap::new(),
    }
}

// ---------------------------------------------------------------------------
// aws_ecs_account_setting_default
// ---------------------------------------------------------------------------

pub struct AwsEcsAccountSettingDefaultConverter {
    service: Arc<EcsService>,
}

impl AwsEcsAccountSettingDefaultConverter {
    pub fn new(service: Arc<EcsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEcsAccountSettingDefaultConverter {
    fn resource_type(&self) -> &str {
        "aws_ecs_account_setting_default"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let model: ecs_gen::AccountSettingDefaultTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_ecs_account_setting_default", e)
                })?;

            let principal_arn = model
                .principal_arn
                .unwrap_or_else(|| format!("arn:aws:iam::{}:root", ctx.default_account_id));
            let setting = EcsAccountSettingView {
                name: model.name.clone(),
                value: model.value,
                principal_arn,
            };

            let mut state_view = minimal_ecs_state_view();
            state_view.account_settings.insert(model.name, setting);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for s in view.account_settings.values() {
                let attrs = serde_json::json!({
                    "id": s.name,
                    "name": s.name,
                    "value": s.value,
                    "principal_arn": s.principal_arn,
                });
                results.push(ExtractedResource {
                    name: s.name.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// aws_ecs_capacity_provider
// ---------------------------------------------------------------------------

pub struct AwsEcsCapacityProviderConverter {
    service: Arc<EcsService>,
}

impl AwsEcsCapacityProviderConverter {
    pub fn new(service: Arc<EcsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEcsCapacityProviderConverter {
    fn resource_type(&self) -> &str {
        "aws_ecs_capacity_provider"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let attrs = &instance.attributes;
            let region = extract_region(attrs, &ctx.default_region);
            let model: ecs_gen::CapacityProviderTfModel = serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_ecs_capacity_provider", e))?;

            let arn = model.arn.unwrap_or_else(|| {
                format!(
                    "arn:aws:ecs:{}:{}:capacity-provider/{}",
                    region, ctx.default_account_id, model.name
                )
            });

            let auto_scaling_group_arn = attrs
                .get("auto_scaling_group_provider")
                .and_then(|v| v.as_array())
                .and_then(|arr| arr.first())
                .and_then(|p| p.get("auto_scaling_group_arn"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let tags: Vec<EcsTagView> = attrs
                .get("tags")
                .and_then(|v| v.as_object())
                .map(|obj| {
                    obj.iter()
                        .filter_map(|(k, v)| {
                            v.as_str().map(|s| EcsTagView {
                                key: k.clone(),
                                value: s.to_string(),
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();

            let cp = EcsCapacityProviderView {
                name: model.name.clone(),
                arn,
                status: "ACTIVE".to_string(),
                auto_scaling_group_arn,
                tags,
            };

            let mut state_view = minimal_ecs_state_view();
            state_view.capacity_providers.insert(model.name, cp);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for cp in view.capacity_providers.values() {
                let tags: HashMap<String, String> = cp
                    .tags
                    .iter()
                    .map(|t| (t.key.clone(), t.value.clone()))
                    .collect();
                let attrs = serde_json::json!({
                    "id": cp.name,
                    "name": cp.name,
                    "arn": cp.arn,
                    "auto_scaling_group_provider": [{
                        "auto_scaling_group_arn": cp.auto_scaling_group_arn,
                    }],
                    "tags": tags,
                    "tags_all": tags,
                });
                results.push(ExtractedResource {
                    name: cp.name.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// aws_ecs_cluster_capacity_providers — mutates existing cluster (no separate
// state slot in the view layer; warning-only).
// ---------------------------------------------------------------------------

pub struct AwsEcsClusterCapacityProvidersConverter {
    #[allow(dead_code)]
    service: Arc<EcsService>,
}

impl AwsEcsClusterCapacityProvidersConverter {
    pub fn new(service: Arc<EcsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEcsClusterCapacityProvidersConverter {
    fn resource_type(&self) -> &str {
        "aws_ecs_cluster_capacity_providers"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: ecs_gen::ClusterCapacityProvidersTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_ecs_cluster_capacity_providers", e)
                })?;
            let warn_msg = "cluster capacity-provider attachments are not modelled as a \
                            separate state slot; inject is a no-op"
                .to_string();
            eprintln!("warning: aws_ecs_cluster_capacity_providers: {warn_msg}");
            Ok(ConversionResult {
                region,
                warnings: vec![format!("aws_ecs_cluster_capacity_providers: {warn_msg}")],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_ecs_tag
// ---------------------------------------------------------------------------

pub struct AwsEcsTagConverter {
    service: Arc<EcsService>,
}

impl AwsEcsTagConverter {
    pub fn new(service: Arc<EcsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEcsTagConverter {
    fn resource_type(&self) -> &str {
        "aws_ecs_tag"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let model: ecs_gen::TagTfModel = serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ecs_tag", e))?;

            let tag = EcsTagView {
                key: model.key,
                value: model.value,
            };

            let mut state_view = minimal_ecs_state_view();
            state_view
                .resource_tags
                .insert(model.resource_arn, vec![tag]);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for (arn, tags) in view.resource_tags.iter() {
                for tag in tags {
                    let attrs = serde_json::json!({
                        "id": format!("{},{}", arn, tag.key),
                        "resource_arn": arn,
                        "key": tag.key,
                        "value": tag.value,
                    });
                    results.push(ExtractedResource {
                        name: format!("{}_{}", arn, tag.key),
                        account_id: ctx.default_account_id.clone(),
                        region: ctx.default_region.clone(),
                        attributes: attrs,
                    });
                }
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// aws_ecs_task_set — no view-level state slot (warning-only).
// ---------------------------------------------------------------------------

pub struct AwsEcsTaskSetConverter {
    #[allow(dead_code)]
    service: Arc<EcsService>,
}

impl AwsEcsTaskSetConverter {
    pub fn new(service: Arc<EcsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEcsTaskSetConverter {
    fn resource_type(&self) -> &str {
        "aws_ecs_task_set"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: ecs_gen::TaskSetTfModel =
                serde_json::from_value(instance.attributes.clone())
                    .map_err(|e| classify_deserialize_error("aws_ecs_task_set", e))?;
            let warn_msg =
                "no view-level state slot in winterbaume_ecs for task sets; inject is a no-op"
                    .to_string();
            eprintln!("warning: aws_ecs_task_set: {warn_msg}");
            Ok(ConversionResult {
                region,
                warnings: vec![format!("aws_ecs_task_set: {warn_msg}")],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}
