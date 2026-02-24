use std::collections::HashMap;

use crate::types::*;

#[derive(Debug, Default)]
pub struct EcsState {
    pub clusters: HashMap<String, EcsCluster>,
    pub task_definitions: HashMap<String, TaskDefinition>,
    pub services: HashMap<String, EcsServiceDef>,
    pub capacity_providers: HashMap<String, EcsCapacityProvider>,
    pub container_instances: HashMap<String, EcsContainerInstance>,
    pub tasks: HashMap<String, EcsTask>,
    pub task_sets: HashMap<String, EcsTaskSet>,
    pub account_settings: HashMap<String, EcsAccountSetting>,
    pub attributes: Vec<EcsAttribute>,
    /// resource ARN -> Vec<EcsTag>
    pub resource_tags: HashMap<String, Vec<EcsTag>>,
    /// Task ARN -> EcsTaskProtection
    pub task_protections: HashMap<String, EcsTaskProtection>,
    /// Account-level default settings (name -> value)
    pub account_settings_defaults: HashMap<String, String>,
    /// Cluster-level settings keyed by "cluster_name:setting_name"
    pub cluster_settings: HashMap<String, Vec<EcsClusterSetting>>,
    /// ExpressGateway services keyed by service ARN
    pub express_gateway_services: HashMap<String, EcsExpressGatewayService>,
    /// Poll endpoints keyed by region
    pub poll_endpoints: HashMap<String, EcsPollEndpoint>,
    /// Attachment state change submissions (ECS agent internal API)
    pub attachment_state_changes: Vec<EcsAttachmentStateChange>,
    /// Container state change submissions (ECS agent internal API)
    pub container_state_changes: Vec<EcsContainerStateChange>,
    /// Task state change submissions (ECS agent internal API)
    pub task_state_changes: Vec<EcsTaskStateChange>,
    /// Service deployments keyed by deployment ARN
    pub service_deployments: HashMap<String, EcsServiceDeployment>,
    /// Service revisions keyed by revision ARN
    pub service_revisions: HashMap<String, EcsServiceRevision>,
}

#[derive(Debug, thiserror::Error)]
pub enum EcsError {
    #[error("Cluster not found: {cluster}")]
    ClusterNotFound { cluster: String },
    #[error("Cluster not found.")]
    ClusterNotFoundPlain,
    #[error("Unable to describe task definition: {task_definition}")]
    TaskDefinitionNotDescribable { task_definition: String },
    #[error("Task definition not found: {task_definition}")]
    TaskDefinitionNotFound { task_definition: String },
    #[error("Creation of service was not idempotent. Service {service_name} already exists.")]
    ServiceAlreadyExists { service_name: String },
    #[error("Service not found: {service_name}")]
    ServiceNotFound { service_name: String },
    #[error("Service not found.")]
    ServiceNotFoundPlain,
    #[error("A capacity provider with name {name} already exists.")]
    CapacityProviderAlreadyExists { name: String },
    #[error("The specified capacity provider does not exist: {name_or_arn}")]
    CapacityProviderNotFound { name_or_arn: String },
    #[error("Container instance not found: {container_instance}")]
    ContainerInstanceNotFound { container_instance: String },
    #[error("Task not found: {task_arn}")]
    TaskNotFound { task_arn: String },
    #[error("The specified task set does not exist: {task_set}")]
    TaskSetNotFound { task_set: String },
    #[error("The specified resource does not exist: {resource_arn}")]
    ResourceNotFound { resource_arn: String },
    #[error("Unable to describe task definition.")]
    TaskDefinitionResourceNotFound,
    #[error("Express gateway service {service_name} already exists.")]
    ExpressGatewayServiceAlreadyExists { service_name: String },
    #[error("Express gateway service not found: {service}")]
    ExpressGatewayServiceNotFound { service: String },
}

impl EcsState {
    // ========================================================================
    // Cluster operations
    // ========================================================================

    pub fn create_cluster(
        &mut self,
        name: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&EcsCluster, EcsError> {
        if self.clusters.contains_key(name) {
            // ECS CreateCluster is idempotent - return existing
            return Ok(self.clusters.get(name).unwrap());
        }

        let arn = format!("arn:aws:ecs:{region}:{account_id}:cluster/{name}");
        let cluster = EcsCluster {
            name: name.to_string(),
            arn,
            status: "ACTIVE".to_string(),
            registered_container_instances_count: 0,
            running_tasks_count: 0,
            capacity_providers: Vec::new(),
            default_capacity_provider_strategy: Vec::new(),
            tags: Vec::new(),
        };

        self.clusters.insert(name.to_string(), cluster);
        Ok(self.clusters.get(name).unwrap())
    }

    pub fn delete_cluster(&mut self, cluster: &str) -> Result<&EcsCluster, EcsError> {
        let resolved = self.resolve_cluster_name(cluster);
        let mut c = self
            .clusters
            .remove(&resolved)
            .ok_or_else(|| EcsError::ClusterNotFound {
                cluster: cluster.to_string(),
            })?;
        c.status = "INACTIVE".to_string();
        self.clusters.insert(resolved.clone(), c);
        Ok(self.clusters.get(&resolved).unwrap())
    }

    pub fn describe_clusters(
        &self,
        names: &[&str],
        account_id: &str,
        region: &str,
    ) -> (Vec<&EcsCluster>, Vec<String>) {
        let mut found = Vec::new();
        let mut failures = Vec::new();
        for name in names {
            let resolved = self.resolve_cluster_name(name);
            match self.clusters.get(&resolved) {
                Some(c) => found.push(c),
                None => {
                    let arn = if name.starts_with("arn:aws:ecs:") {
                        name.to_string()
                    } else {
                        format!("arn:aws:ecs:{region}:{account_id}:cluster/{name}")
                    };
                    failures.push(arn);
                }
            }
        }
        (found, failures)
    }

    pub fn list_clusters(&self) -> Vec<&str> {
        self.clusters.values().map(|c| c.arn.as_str()).collect()
    }

    pub fn update_cluster(&mut self, cluster: &str) -> Result<&EcsCluster, EcsError> {
        let resolved = self.resolve_cluster_name(cluster);
        self.clusters
            .get(&resolved)
            .ok_or_else(|| EcsError::ClusterNotFound {
                cluster: cluster.to_string(),
            })
    }

    pub fn put_cluster_capacity_providers(
        &mut self,
        cluster: &str,
        capacity_providers: Vec<String>,
        default_strategy: Vec<CapacityProviderStrategyItem>,
    ) -> Result<&EcsCluster, EcsError> {
        let resolved = self.resolve_cluster_name(cluster);
        let c = self
            .clusters
            .get_mut(&resolved)
            .ok_or_else(|| EcsError::ClusterNotFound {
                cluster: cluster.to_string(),
            })?;
        c.capacity_providers = capacity_providers;
        c.default_capacity_provider_strategy = default_strategy;
        Ok(c)
    }

    // ========================================================================
    // Task Definition operations
    // ========================================================================

    pub fn register_task_definition(
        &mut self,
        family: &str,
        container_definitions: Vec<ContainerDefinition>,
        account_id: &str,
        region: &str,
        task_role_arn: Option<&str>,
        execution_role_arn: Option<&str>,
        requires_compatibilities: Vec<String>,
        cpu: Option<&str>,
        memory: Option<&str>,
    ) -> Result<&TaskDefinition, EcsError> {
        let revision = self
            .task_definitions
            .values()
            .filter(|td| td.family == family)
            .map(|td| td.revision)
            .max()
            .unwrap_or(0)
            + 1;

        let key = format!("{family}:{revision}");
        let arn = format!("arn:aws:ecs:{region}:{account_id}:task-definition/{key}");

        let network_mode = if requires_compatibilities.contains(&"FARGATE".to_string()) {
            "awsvpc".to_string()
        } else {
            "bridge".to_string()
        };

        let td = TaskDefinition {
            family: family.to_string(),
            revision,
            arn,
            container_definitions,
            status: "ACTIVE".to_string(),
            network_mode,
            task_role_arn: task_role_arn.map(|s| s.to_string()),
            execution_role_arn: execution_role_arn.map(|s| s.to_string()),
            requires_compatibilities,
            cpu: cpu.map(|s| s.to_string()),
            memory: memory.map(|s| s.to_string()),
        };

        self.task_definitions.insert(key.clone(), td);
        Ok(self.task_definitions.get(&key).unwrap())
    }

    pub fn describe_task_definition(
        &self,
        task_definition: &str,
    ) -> Result<&TaskDefinition, EcsError> {
        // FIX(terraform-e2e): terraform calls DescribeTaskDefinition with the
        // full ARN (e.g., arn:aws:ecs:...:task-definition/family:1) but state
        // is keyed by "family:revision". Normalize ARN to key before lookup.
        let key = if let Some(rest) = task_definition.split(":task-definition/").nth(1) {
            rest
        } else {
            task_definition
        };

        if let Some(td) = self.task_definitions.get(key) {
            return Ok(td);
        }

        let latest = self
            .task_definitions
            .values()
            .filter(|td| td.family == key)
            .max_by_key(|td| td.revision);

        latest.ok_or_else(|| EcsError::TaskDefinitionNotDescribable {
            task_definition: task_definition.to_string(),
        })
    }

    pub fn deregister_task_definition(
        &mut self,
        task_definition: &str,
    ) -> Result<&TaskDefinition, EcsError> {
        // FIX(terraform-e2e): terraform calls DeregisterTaskDefinition with the
        // full ARN (e.g., arn:aws:ecs:...:task-definition/family:1) but state
        // is keyed by "family:revision". Normalize ARN to key before lookup.
        let task_definition = if let Some(rest) = task_definition.split(":task-definition/").nth(1)
        {
            rest
        } else {
            task_definition
        };
        // Find the task definition key
        let key = if self.task_definitions.contains_key(task_definition) {
            task_definition.to_string()
        } else {
            // Try as family name - find latest revision
            let found = self
                .task_definitions
                .iter()
                .filter(|(_, td)| td.family == task_definition)
                .max_by_key(|(_, td)| td.revision)
                .map(|(k, _)| k.clone());
            found.ok_or_else(|| EcsError::TaskDefinitionNotDescribable {
                task_definition: task_definition.to_string(),
            })?
        };

        let td = self.task_definitions.get_mut(&key).unwrap();
        td.status = "INACTIVE".to_string();
        Ok(self.task_definitions.get(&key).unwrap())
    }

    pub fn delete_task_definitions(
        &mut self,
        task_definitions: &[&str],
    ) -> Result<Vec<&TaskDefinition>, EcsError> {
        // FIX(terraform-e2e): normalize ARNs to "family:revision" keys before lookup.
        let keys: Vec<String> = task_definitions
            .iter()
            .map(|td| {
                if let Some(rest) = td.split(":task-definition/").nth(1) {
                    rest.to_string()
                } else {
                    td.to_string()
                }
            })
            .collect();
        let mut results = Vec::new();
        for key in &keys {
            if let Some(td) = self.task_definitions.get_mut(key.as_str()) {
                td.status = "DELETE_IN_PROGRESS".to_string();
            }
        }
        for key in &keys {
            if let Some(td) = self.task_definitions.get(key.as_str()) {
                results.push(td);
            }
        }
        Ok(results)
    }

    pub fn list_task_definitions(&self) -> Vec<&str> {
        self.task_definitions
            .values()
            .filter(|td| td.status == "ACTIVE")
            .map(|td| td.arn.as_str())
            .collect()
    }

    pub fn list_task_definition_families(&self) -> Vec<String> {
        let mut families: Vec<String> = self
            .task_definitions
            .values()
            .filter(|td| td.status == "ACTIVE")
            .map(|td| td.family.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        families.sort();
        families
    }

    // ========================================================================
    // Service operations
    // ========================================================================

    pub fn create_service(
        &mut self,
        cluster: &str,
        service_name: &str,
        task_definition: &str,
        desired_count: i32,
        account_id: &str,
        region: &str,
        launch_type: &str,
        scheduling_strategy: &str,
        tags: Vec<EcsTag>,
        load_balancers: Vec<EcsLoadBalancer>,
        deployment_controller_type: Option<&str>,
    ) -> Result<&EcsServiceDef, EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        let cluster_arn = match self.clusters.get(&cluster_name) {
            Some(c) => c.arn.clone(),
            None => {
                return Err(EcsError::ClusterNotFoundPlain);
            }
        };

        let service_key = format!("{cluster_name}/{service_name}");
        if self.services.contains_key(&service_key) {
            return Err(EcsError::ServiceAlreadyExists {
                service_name: service_name.to_string(),
            });
        }

        let resolved_task_def =
            self.resolve_task_definition_arn(task_definition, account_id, region);

        let arn =
            format!("arn:aws:ecs:{region}:{account_id}:service/{cluster_name}/{service_name}");

        let svc_tags = tags;
        // Also store tags in resource_tags for list_tags_for_resource
        let svc_arn = arn.clone();

        let svc = EcsServiceDef {
            name: service_name.to_string(),
            arn,
            cluster_arn,
            task_definition: resolved_task_def,
            desired_count,
            running_count: 0,
            pending_count: desired_count,
            status: "ACTIVE".to_string(),
            scheduling_strategy: scheduling_strategy.to_string(),
            launch_type: launch_type.to_string(),
            tags: svc_tags.clone(),
            load_balancers,
            deployment_controller_type: deployment_controller_type.unwrap_or("ECS").to_string(),
        };

        // Store tags in resource_tags for list_tags_for_resource
        if !svc_tags.is_empty() {
            self.resource_tags.insert(svc_arn, svc_tags);
        }

        self.services.insert(service_key.clone(), svc);
        Ok(self.services.get(&service_key).unwrap())
    }

    pub fn describe_services(
        &self,
        cluster: &str,
        service_names: &[&str],
        account_id: &str,
        region: &str,
    ) -> Result<(Vec<&EcsServiceDef>, Vec<String>), EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        if !self.clusters.contains_key(&cluster_name) {
            return Err(EcsError::ClusterNotFoundPlain);
        }
        let mut found = Vec::new();
        let mut failures = Vec::new();
        for svc_name_or_arn in service_names {
            let svc_name = if svc_name_or_arn.starts_with("arn:aws:ecs:") {
                svc_name_or_arn
                    .rsplit('/')
                    .next()
                    .unwrap_or(svc_name_or_arn)
            } else {
                svc_name_or_arn
            };
            let key = format!("{cluster_name}/{svc_name}");
            match self.services.get(&key) {
                Some(s) => found.push(s),
                None => {
                    let arn = if svc_name_or_arn.starts_with("arn:aws:ecs:") {
                        svc_name_or_arn.to_string()
                    } else {
                        format!("arn:aws:ecs:{region}:{account_id}:service/{svc_name}")
                    };
                    failures.push(arn);
                }
            }
        }
        Ok((found, failures))
    }

    pub fn delete_service(
        &mut self,
        cluster: &str,
        service_name: &str,
    ) -> Result<&EcsServiceDef, EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        if !self.clusters.contains_key(&cluster_name) {
            return Err(EcsError::ClusterNotFoundPlain);
        }

        let resolved_name = if service_name.starts_with("arn:aws:ecs:") {
            service_name.rsplit('/').next().unwrap_or(service_name)
        } else {
            service_name
        };

        let key = format!("{cluster_name}/{resolved_name}");
        let svc = self
            .services
            .get_mut(&key)
            .ok_or_else(|| EcsError::ServiceNotFound {
                service_name: service_name.to_string(),
            })?;
        svc.status = "INACTIVE".to_string();
        Ok(self.services.get(&key).unwrap())
    }

    pub fn update_service(
        &mut self,
        cluster: &str,
        service_name: &str,
        task_definition: Option<&str>,
        desired_count: Option<i32>,
        account_id: &str,
        region: &str,
    ) -> Result<&EcsServiceDef, EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        if !self.clusters.contains_key(&cluster_name) {
            return Err(EcsError::ClusterNotFoundPlain);
        }

        let resolved_name = if service_name.starts_with("arn:aws:ecs:") {
            service_name.rsplit('/').next().unwrap_or(service_name)
        } else {
            service_name
        };

        let key = format!("{cluster_name}/{resolved_name}");
        if !self.services.contains_key(&key) {
            return Err(EcsError::ServiceNotFound {
                service_name: service_name.to_string(),
            });
        }

        // Resolve task definition ARN before borrowing services mutably
        let resolved_td =
            task_definition.map(|td| self.resolve_task_definition_arn(td, account_id, region));

        let svc = self.services.get_mut(&key).unwrap();
        if let Some(td_arn) = resolved_td {
            svc.task_definition = td_arn;
        }
        if let Some(count) = desired_count {
            svc.desired_count = count;
            svc.pending_count = count;
        }

        Ok(self.services.get(&key).unwrap())
    }

    pub fn list_services(&self, cluster: &str) -> Result<Vec<&str>, EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        if !self.clusters.contains_key(&cluster_name) {
            return Err(EcsError::ClusterNotFoundPlain);
        }

        let prefix = format!("{cluster_name}/");
        let arns: Vec<&str> = self
            .services
            .iter()
            .filter(|(k, s)| k.starts_with(&prefix) && s.status == "ACTIVE")
            .map(|(_, s)| s.arn.as_str())
            .collect();
        Ok(arns)
    }

    // ========================================================================
    // Capacity Provider operations
    // ========================================================================

    pub fn create_capacity_provider(
        &mut self,
        name: &str,
        auto_scaling_group_arn: &str,
        account_id: &str,
        region: &str,
        tags: Vec<EcsTag>,
    ) -> Result<&EcsCapacityProvider, EcsError> {
        if self.capacity_providers.contains_key(name) {
            return Err(EcsError::CapacityProviderAlreadyExists {
                name: name.to_string(),
            });
        }

        let arn = format!("arn:aws:ecs:{region}:{account_id}:capacity-provider/{name}");
        let cp = EcsCapacityProvider {
            name: name.to_string(),
            arn,
            status: "ACTIVE".to_string(),
            auto_scaling_group_arn: auto_scaling_group_arn.to_string(),
            tags,
        };
        self.capacity_providers.insert(name.to_string(), cp);
        Ok(self.capacity_providers.get(name).unwrap())
    }

    pub fn delete_capacity_provider(
        &mut self,
        name_or_arn: &str,
    ) -> Result<&EcsCapacityProvider, EcsError> {
        let name = self.resolve_capacity_provider_name(name_or_arn);
        let cp = self.capacity_providers.get_mut(&name).ok_or_else(|| {
            EcsError::CapacityProviderNotFound {
                name_or_arn: name_or_arn.to_string(),
            }
        })?;
        cp.status = "INACTIVE".to_string();
        Ok(self.capacity_providers.get(&name).unwrap())
    }

    pub fn describe_capacity_providers(
        &self,
        names: &[&str],
    ) -> (Vec<&EcsCapacityProvider>, Vec<String>) {
        if names.is_empty() {
            let all: Vec<&EcsCapacityProvider> = self.capacity_providers.values().collect();
            return (all, Vec::new());
        }
        let mut found = Vec::new();
        let mut failures = Vec::new();
        for name in names {
            let resolved = self.resolve_capacity_provider_name(name);
            match self.capacity_providers.get(&resolved) {
                Some(cp) => found.push(cp),
                None => failures.push(name.to_string()),
            }
        }
        (found, failures)
    }

    pub fn update_capacity_provider(
        &mut self,
        name_or_arn: &str,
    ) -> Result<&EcsCapacityProvider, EcsError> {
        let name = self.resolve_capacity_provider_name(name_or_arn);
        self.capacity_providers
            .get(&name)
            .ok_or_else(|| EcsError::CapacityProviderNotFound {
                name_or_arn: name_or_arn.to_string(),
            })
    }

    // ========================================================================
    // Container Instance operations
    // ========================================================================

    pub fn register_container_instance(
        &mut self,
        cluster: &str,
        instance_id: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&EcsContainerInstance, EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        let cluster_arn = match self.clusters.get(&cluster_name) {
            Some(c) => c.arn.clone(),
            None => {
                return Err(EcsError::ClusterNotFoundPlain);
            }
        };

        let ci_id = uuid::Uuid::new_v4().to_string();
        let arn =
            format!("arn:aws:ecs:{region}:{account_id}:container-instance/{cluster_name}/{ci_id}");

        let ci = EcsContainerInstance {
            arn: arn.clone(),
            ec2_instance_id: instance_id.to_string(),
            cluster_arn: cluster_arn.clone(),
            status: "ACTIVE".to_string(),
            running_tasks_count: 0,
            pending_tasks_count: 0,
            agent_connected: true,
            version: 1,
            tags: Vec::new(),
        };

        // Update cluster's registered count
        if let Some(c) = self.clusters.get_mut(&cluster_name) {
            c.registered_container_instances_count += 1;
        }

        self.container_instances.insert(arn.clone(), ci);
        Ok(self.container_instances.get(&arn).unwrap())
    }

    pub fn deregister_container_instance(
        &mut self,
        cluster: &str,
        container_instance: &str,
    ) -> Result<&EcsContainerInstance, EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        if !self.clusters.contains_key(&cluster_name) {
            return Err(EcsError::ClusterNotFoundPlain);
        }

        let ci_arn = self.resolve_container_instance_arn(container_instance, &cluster_name);
        let ci = self.container_instances.get_mut(&ci_arn).ok_or_else(|| {
            EcsError::ContainerInstanceNotFound {
                container_instance: container_instance.to_string(),
            }
        })?;
        ci.status = "INACTIVE".to_string();

        if let Some(c) = self.clusters.get_mut(&cluster_name) {
            c.registered_container_instances_count =
                (c.registered_container_instances_count - 1).max(0);
        }

        Ok(self.container_instances.get(&ci_arn).unwrap())
    }

    pub fn describe_container_instances(
        &self,
        cluster: &str,
        container_instances: &[&str],
    ) -> Result<(Vec<&EcsContainerInstance>, Vec<String>), EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        if !self.clusters.contains_key(&cluster_name) {
            return Err(EcsError::ClusterNotFoundPlain);
        }

        let mut found = Vec::new();
        let mut failures = Vec::new();
        for ci_ref in container_instances {
            let ci_arn = self.resolve_container_instance_arn(ci_ref, &cluster_name);
            match self.container_instances.get(&ci_arn) {
                Some(ci) if ci.cluster_arn.ends_with(&format!("/{cluster_name}")) => {
                    found.push(ci);
                }
                _ => {
                    failures.push(ci_arn);
                }
            }
        }
        Ok((found, failures))
    }

    pub fn list_container_instances(&self, cluster: &str) -> Result<Vec<&str>, EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        if !self.clusters.contains_key(&cluster_name) {
            return Err(EcsError::ClusterNotFoundPlain);
        }

        let arns: Vec<&str> = self
            .container_instances
            .values()
            .filter(|ci| {
                ci.cluster_arn.ends_with(&format!("/{cluster_name}")) && ci.status == "ACTIVE"
            })
            .map(|ci| ci.arn.as_str())
            .collect();
        Ok(arns)
    }

    pub fn update_container_instances_state(
        &mut self,
        cluster: &str,
        container_instances: &[&str],
        status: &str,
    ) -> Result<(Vec<String>, Vec<String>), EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        if !self.clusters.contains_key(&cluster_name) {
            return Err(EcsError::ClusterNotFoundPlain);
        }

        let mut updated_arns = Vec::new();
        let mut failures = Vec::new();
        for ci_ref in container_instances {
            let ci_arn = self.resolve_container_instance_arn(ci_ref, &cluster_name);
            match self.container_instances.get_mut(&ci_arn) {
                Some(ci) if ci.cluster_arn.ends_with(&format!("/{cluster_name}")) => {
                    ci.status = status.to_string();
                    updated_arns.push(ci_arn);
                }
                _ => {
                    failures.push(ci_arn);
                }
            }
        }
        Ok((updated_arns, failures))
    }

    // ========================================================================
    // Task operations
    // ========================================================================

    pub fn run_task(
        &mut self,
        cluster: &str,
        task_definition: &str,
        count: i32,
        launch_type: &str,
        started_by: Option<&str>,
        group: Option<&str>,
        overrides: Option<serde_json::Value>,
        account_id: &str,
        region: &str,
        tags: Vec<EcsTag>,
    ) -> Result<Vec<&EcsTask>, EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        let cluster_arn = match self.clusters.get(&cluster_name) {
            Some(c) => c.arn.clone(),
            None => {
                return Err(EcsError::ClusterNotFoundPlain);
            }
        };

        let td = self
            .describe_task_definition(task_definition)
            .map_err(|_| EcsError::TaskDefinitionNotFound {
                task_definition: task_definition.to_string(),
            })?;
        let td_arn = td.arn.clone();
        let td_family = td.family.clone();
        let container_defs = td.container_definitions.clone();

        let default_group = format!("family:{td_family}");
        let effective_group = group.map(|s| s.to_string()).unwrap_or(default_group);

        let mut task_arns = Vec::new();
        for _ in 0..count {
            let task_id = uuid::Uuid::new_v4().to_string();
            let task_arn =
                format!("arn:aws:ecs:{region}:{account_id}:task/{cluster_name}/{task_id}");

            let containers: Vec<EcsContainer> = container_defs
                .iter()
                .map(|cd| {
                    let container_id = uuid::Uuid::new_v4().to_string();
                    EcsContainer {
                        container_arn: format!(
                            "arn:aws:ecs:{region}:{account_id}:container/{container_id}"
                        ),
                        name: cd.name.clone(),
                        last_status: "PENDING".to_string(),
                        task_arn: task_arn.clone(),
                    }
                })
                .collect();

            let task = EcsTask {
                task_arn: task_arn.clone(),
                task_definition_arn: td_arn.clone(),
                cluster_arn: cluster_arn.clone(),
                container_instance_arn: None,
                last_status: "RUNNING".to_string(),
                desired_status: "RUNNING".to_string(),
                started_by: started_by.map(|s| s.to_string()),
                group: Some(effective_group.clone()),
                launch_type: launch_type.to_string(),
                containers,
                overrides: overrides.clone(),
                tags: tags.clone(),
                stopped_reason: Some(String::new()),
            };

            self.tasks.insert(task_arn.clone(), task);
            // Store tags in resource_tags for list_tags_for_resource
            if !tags.is_empty() {
                self.resource_tags.insert(task_arn.clone(), tags.clone());
            }
            task_arns.push(task_arn);
        }

        // Update cluster running tasks count
        if let Some(c) = self.clusters.get_mut(&cluster_name) {
            c.running_tasks_count += count;
        }

        let tasks: Vec<&EcsTask> = task_arns
            .iter()
            .map(|arn| self.tasks.get(arn).unwrap())
            .collect();
        Ok(tasks)
    }

    pub fn start_task(
        &mut self,
        cluster: &str,
        task_definition: &str,
        container_instances: &[&str],
        started_by: Option<&str>,
        group: Option<&str>,
        overrides: Option<serde_json::Value>,
        account_id: &str,
        region: &str,
        tags: Vec<EcsTag>,
    ) -> Result<(Vec<String>, Vec<String>), EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        let cluster_arn = match self.clusters.get(&cluster_name) {
            Some(c) => c.arn.clone(),
            None => {
                return Err(EcsError::ClusterNotFoundPlain);
            }
        };

        let td = self
            .describe_task_definition(task_definition)
            .map_err(|_| EcsError::TaskDefinitionNotFound {
                task_definition: task_definition.to_string(),
            })?;
        let td_arn = td.arn.clone();
        let td_family = td.family.clone();
        let container_defs = td.container_definitions.clone();

        let default_group = format!("family:{td_family}");
        let effective_group = group.map(|s| s.to_string()).unwrap_or(default_group);

        let mut task_arns = Vec::new();
        let mut failures = Vec::new();

        for ci_ref in container_instances {
            let ci_arn = self.resolve_container_instance_arn(ci_ref, &cluster_name);
            if !self.container_instances.contains_key(&ci_arn) {
                failures.push(ci_arn);
                continue;
            }

            let task_id = uuid::Uuid::new_v4().to_string();
            let task_arn =
                format!("arn:aws:ecs:{region}:{account_id}:task/{cluster_name}/{task_id}");

            let containers: Vec<EcsContainer> = container_defs
                .iter()
                .map(|cd| {
                    let container_id = uuid::Uuid::new_v4().to_string();
                    EcsContainer {
                        container_arn: format!(
                            "arn:aws:ecs:{region}:{account_id}:container/{container_id}"
                        ),
                        name: cd.name.clone(),
                        last_status: "PENDING".to_string(),
                        task_arn: task_arn.clone(),
                    }
                })
                .collect();

            let task = EcsTask {
                task_arn: task_arn.clone(),
                task_definition_arn: td_arn.clone(),
                cluster_arn: cluster_arn.clone(),
                container_instance_arn: Some(ci_arn),
                last_status: "RUNNING".to_string(),
                desired_status: "RUNNING".to_string(),
                started_by: started_by.map(|s| s.to_string()),
                group: Some(effective_group.clone()),
                launch_type: "EC2".to_string(),
                containers,
                overrides: overrides.clone(),
                tags: tags.clone(),
                stopped_reason: Some(String::new()),
            };

            self.tasks.insert(task_arn.clone(), task);
            // Store tags in resource_tags for list_tags_for_resource
            if !tags.is_empty() {
                self.resource_tags.insert(task_arn.clone(), tags.clone());
            }
            task_arns.push(task_arn);
        }

        // Update cluster running tasks count
        if let Some(c) = self.clusters.get_mut(&cluster_name) {
            c.running_tasks_count += task_arns.len() as i32;
        }

        Ok((task_arns, failures))
    }

    pub fn describe_tasks(
        &self,
        cluster: &str,
        task_arns: &[&str],
    ) -> Result<(Vec<&EcsTask>, Vec<String>), EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        if !self.clusters.contains_key(&cluster_name) {
            return Err(EcsError::ClusterNotFoundPlain);
        }

        let cluster_arn_suffix = format!("/{cluster_name}");
        let mut found = Vec::new();
        let mut failures = Vec::new();
        for task_ref in task_arns {
            // Try to match by full ARN or task ID
            let task = if task_ref.starts_with("arn:aws:ecs:") {
                self.tasks.get(*task_ref)
            } else {
                // Search by task ID suffix
                self.tasks.values().find(|t| t.task_arn.ends_with(task_ref))
            };
            match task {
                Some(t) if t.cluster_arn.contains(&cluster_arn_suffix) => found.push(t),
                _ => failures.push(task_ref.to_string()),
            }
        }
        Ok((found, failures))
    }

    pub fn stop_task(
        &mut self,
        cluster: &str,
        task_arn: &str,
        reason: Option<&str>,
    ) -> Result<&EcsTask, EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        if !self.clusters.contains_key(&cluster_name) {
            return Err(EcsError::ClusterNotFoundPlain);
        }

        // Find task
        let resolved_arn = if task_arn.starts_with("arn:aws:ecs:") {
            task_arn.to_string()
        } else {
            self.tasks
                .keys()
                .find(|k| k.ends_with(task_arn))
                .cloned()
                .unwrap_or_default()
        };

        let task = self
            .tasks
            .get_mut(&resolved_arn)
            .ok_or_else(|| EcsError::TaskNotFound {
                task_arn: task_arn.to_string(),
            })?;
        let was_running = task.last_status == "RUNNING";
        let task_cluster_arn = task.cluster_arn.clone();
        task.last_status = "STOPPED".to_string();
        task.desired_status = "STOPPED".to_string();
        task.stopped_reason = reason.map(|s| s.to_string());
        for c in &mut task.containers {
            c.last_status = "STOPPED".to_string();
        }

        // Decrement cluster running tasks count
        if was_running {
            let cluster_name_from_arn = task_cluster_arn
                .rsplit('/')
                .next()
                .unwrap_or("")
                .to_string();
            if let Some(c) = self.clusters.get_mut(&cluster_name_from_arn) {
                c.running_tasks_count = (c.running_tasks_count - 1).max(0);
            }
        }

        Ok(self.tasks.get(&resolved_arn).unwrap())
    }

    pub fn list_tasks(
        &self,
        cluster: &str,
        family: Option<&str>,
        service_name: Option<&str>,
        desired_status: Option<&str>,
        started_by: Option<&str>,
        container_instance: Option<&str>,
    ) -> Result<Vec<&str>, EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        if !self.clusters.contains_key(&cluster_name) {
            return Err(EcsError::ClusterNotFoundPlain);
        }

        let resolved_ci =
            container_instance.map(|ci| self.resolve_container_instance_arn(ci, &cluster_name));

        let cluster_arn_suffix = format!("/{cluster_name}");
        let arns: Vec<&str> = self
            .tasks
            .values()
            .filter(|t| {
                if !t.cluster_arn.contains(&cluster_arn_suffix) {
                    return false;
                }
                if let Some(f) = family
                    && !t.task_definition_arn.contains(&format!("/{f}:"))
                {
                    return false;
                }
                if let Some(_sn) = service_name {
                    if let Some(ref g) = t.group {
                        if !g.contains(_sn) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                if let Some(ds) = desired_status
                    && t.desired_status.to_uppercase() != ds.to_uppercase()
                {
                    return false;
                }
                if let Some(ref sb) = started_by {
                    match &t.started_by {
                        Some(tsb) if tsb == sb => {}
                        _ => return false,
                    }
                }
                if let Some(ref ci_arn) = resolved_ci {
                    match &t.container_instance_arn {
                        Some(tci) if tci == ci_arn => {}
                        _ => return false,
                    }
                }
                true
            })
            .map(|t| t.task_arn.as_str())
            .collect();
        Ok(arns)
    }

    // ========================================================================
    // Task Set operations
    // ========================================================================

    pub fn create_task_set(
        &mut self,
        cluster: &str,
        service: &str,
        task_definition: &str,
        launch_type: &str,
        scale_value: Option<f64>,
        account_id: &str,
        region: &str,
        tags: Vec<EcsTag>,
    ) -> Result<&EcsTaskSet, EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        let cluster_arn = match self.clusters.get(&cluster_name) {
            Some(c) => c.arn.clone(),
            None => {
                return Err(EcsError::ClusterNotFoundPlain);
            }
        };

        let service_name = if service.starts_with("arn:aws:ecs:") {
            service.rsplit('/').next().unwrap_or(service)
        } else {
            service
        };
        let service_key = format!("{cluster_name}/{service_name}");
        let service_arn = match self.services.get(&service_key) {
            Some(s) => s.arn.clone(),
            None => {
                return Err(EcsError::ServiceNotFound {
                    service_name: service.to_string(),
                });
            }
        };

        let td_arn = self.resolve_task_definition_arn(task_definition, account_id, region);
        let ts_id = format!(
            "ecs-svc/{}",
            &uuid::Uuid::new_v4().to_string().replace('-', "")[..16]
        );
        let ts_arn = format!(
            "arn:aws:ecs:{region}:{account_id}:task-set/{cluster_name}/{service_name}/{ts_id}"
        );

        let ts = EcsTaskSet {
            id: ts_id.clone(),
            task_set_arn: ts_arn.clone(),
            service_arn,
            cluster_arn,
            task_definition: td_arn,
            status: "ACTIVE".to_string(),
            scale_value: scale_value.unwrap_or(100.0),
            scale_unit: "PERCENT".to_string(),
            running_count: 0,
            pending_count: 0,
            launch_type: launch_type.to_string(),
            tags: tags.clone(),
        };

        // Store tags in resource_tags for list_tags_for_resource
        if !tags.is_empty() {
            self.resource_tags.insert(ts_arn.clone(), tags);
        }
        self.task_sets.insert(ts_arn.clone(), ts);
        Ok(self.task_sets.get(&ts_arn).unwrap())
    }

    pub fn delete_task_set(
        &mut self,
        cluster: &str,
        service: &str,
        task_set: &str,
    ) -> Result<&EcsTaskSet, EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        if !self.clusters.contains_key(&cluster_name) {
            return Err(EcsError::ClusterNotFoundPlain);
        }

        let _service_name = if service.starts_with("arn:aws:ecs:") {
            service.rsplit('/').next().unwrap_or(service)
        } else {
            service
        };

        let ts = self
            .task_sets
            .get_mut(task_set)
            .ok_or_else(|| EcsError::TaskSetNotFound {
                task_set: task_set.to_string(),
            })?;
        ts.status = "INACTIVE".to_string();
        Ok(self.task_sets.get(task_set).unwrap())
    }

    pub fn describe_task_sets(
        &self,
        cluster: &str,
        service: &str,
        task_sets: Option<&[&str]>,
    ) -> Result<Vec<&EcsTaskSet>, EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        if !self.clusters.contains_key(&cluster_name) {
            return Err(EcsError::ClusterNotFoundPlain);
        }

        let service_name = if service.starts_with("arn:aws:ecs:") {
            service.rsplit('/').next().unwrap_or(service)
        } else {
            service
        };
        let service_key = format!("{cluster_name}/{service_name}");
        let service_arn = match self.services.get(&service_key) {
            Some(s) => s.arn.clone(),
            None => {
                return Err(EcsError::ServiceNotFound {
                    service_name: service.to_string(),
                });
            }
        };

        let result: Vec<&EcsTaskSet> = match task_sets {
            Some(arns) => self
                .task_sets
                .values()
                .filter(|ts| {
                    ts.service_arn == service_arn && arns.contains(&ts.task_set_arn.as_str())
                })
                .collect(),
            None => self
                .task_sets
                .values()
                .filter(|ts| ts.service_arn == service_arn)
                .collect(),
        };
        Ok(result)
    }

    pub fn update_task_set(
        &mut self,
        cluster: &str,
        service: &str,
        task_set: &str,
        scale_value: f64,
    ) -> Result<&EcsTaskSet, EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        if !self.clusters.contains_key(&cluster_name) {
            return Err(EcsError::ClusterNotFoundPlain);
        }

        let _service_name = if service.starts_with("arn:aws:ecs:") {
            service.rsplit('/').next().unwrap_or(service)
        } else {
            service
        };

        let ts = self
            .task_sets
            .get_mut(task_set)
            .ok_or_else(|| EcsError::TaskSetNotFound {
                task_set: task_set.to_string(),
            })?;
        ts.scale_value = scale_value;
        Ok(self.task_sets.get(task_set).unwrap())
    }

    pub fn update_service_primary_task_set(
        &mut self,
        cluster: &str,
        service: &str,
        primary_task_set: &str,
    ) -> Result<&EcsTaskSet, EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        if !self.clusters.contains_key(&cluster_name) {
            return Err(EcsError::ClusterNotFoundPlain);
        }

        let _service_name = if service.starts_with("arn:aws:ecs:") {
            service.rsplit('/').next().unwrap_or(service)
        } else {
            service
        };

        self.task_sets
            .get(primary_task_set)
            .ok_or_else(|| EcsError::TaskSetNotFound {
                task_set: primary_task_set.to_string(),
            })
    }

    // ========================================================================
    // Account Setting operations
    // ========================================================================

    pub fn put_account_setting(
        &mut self,
        name: &str,
        value: &str,
        principal_arn: &str,
    ) -> &EcsAccountSetting {
        let setting = EcsAccountSetting {
            name: name.to_string(),
            value: value.to_string(),
            principal_arn: principal_arn.to_string(),
        };
        let key = format!("{name}:{principal_arn}");
        self.account_settings.insert(key.clone(), setting);
        self.account_settings.get(&key).unwrap()
    }

    pub fn delete_account_setting(
        &mut self,
        name: &str,
        principal_arn: &str,
    ) -> Result<EcsAccountSetting, EcsError> {
        let key = format!("{name}:{principal_arn}");
        match self.account_settings.remove(&key) {
            Some(setting) => Ok(setting),
            None => {
                // Return a default setting even if not found
                Ok(EcsAccountSetting {
                    name: name.to_string(),
                    value: String::new(),
                    principal_arn: principal_arn.to_string(),
                })
            }
        }
    }

    pub fn list_account_settings(&self, name: Option<&str>) -> Vec<&EcsAccountSetting> {
        self.account_settings
            .values()
            .filter(|s| {
                if let Some(n) = name {
                    s.name == n
                } else {
                    true
                }
            })
            .collect()
    }

    // ========================================================================
    // Attribute operations
    // ========================================================================

    pub fn put_attributes(&mut self, attributes: Vec<EcsAttribute>) -> Vec<&EcsAttribute> {
        let start_idx = self.attributes.len();
        for attr in attributes {
            // Remove existing attribute with same name and target
            self.attributes.retain(|a| {
                !(a.name == attr.name
                    && a.target_id == attr.target_id
                    && a.target_type == attr.target_type)
            });
            self.attributes.push(attr);
        }
        // Return the just-added ones (from start_idx to end, but because of retain the indices changed)
        // Just return all attributes that were touched
        let _end_idx = self.attributes.len();
        self.attributes
            .iter()
            .skip(start_idx.min(self.attributes.len()))
            .collect()
    }

    pub fn delete_attributes(
        &mut self,
        attributes: &[(String, Option<String>, Option<String>)],
    ) -> Vec<EcsAttribute> {
        let mut deleted = Vec::new();
        for (name, target_type, target_id) in attributes {
            let idx = self.attributes.iter().position(|a| {
                a.name == *name
                    && a.target_type.as_deref() == target_type.as_deref()
                    && a.target_id.as_deref() == target_id.as_deref()
            });
            if let Some(i) = idx {
                deleted.push(self.attributes.remove(i));
            }
        }
        deleted
    }

    pub fn list_attributes(&self, target_type: Option<&str>) -> Vec<&EcsAttribute> {
        self.attributes
            .iter()
            .filter(|a| {
                if let Some(tt) = target_type {
                    a.target_type.as_deref() == Some(tt)
                } else {
                    true
                }
            })
            .collect()
    }

    // ========================================================================
    // Tag operations
    // ========================================================================

    pub fn tag_resource(&mut self, resource_arn: &str, tags: Vec<EcsTag>) -> Result<(), EcsError> {
        // Verify the resource exists
        if !self.resource_exists(resource_arn) {
            return Err(EcsError::ResourceNotFound {
                resource_arn: resource_arn.to_string(),
            });
        }

        // Also store tags on the resource types that have a tags field
        self.update_resource_tags(resource_arn, &tags, false);

        let entry = self
            .resource_tags
            .entry(resource_arn.to_string())
            .or_default();
        for tag in tags {
            // Remove existing tag with same key
            entry.retain(|t| t.key != tag.key);
            entry.push(tag);
        }
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[&str],
    ) -> Result<(), EcsError> {
        if !self.resource_exists(resource_arn) {
            return Err(EcsError::ResourceNotFound {
                resource_arn: resource_arn.to_string(),
            });
        }

        if let Some(tags) = self.resource_tags.get_mut(resource_arn) {
            tags.retain(|t| !tag_keys.contains(&t.key.as_str()));
        }

        // Also update tags on the resource types
        self.remove_resource_tags(resource_arn, tag_keys);
        Ok(())
    }

    pub fn list_tags_for_resource(&self, resource_arn: &str) -> Result<Vec<&EcsTag>, EcsError> {
        if !self.resource_exists(resource_arn) {
            // Return appropriate error type based on resource type
            let err = if resource_arn.contains(":service/") {
                EcsError::ServiceNotFoundPlain
            } else if resource_arn.contains(":task-definition/") {
                EcsError::TaskDefinitionResourceNotFound
            } else {
                EcsError::ResourceNotFound {
                    resource_arn: resource_arn.to_string(),
                }
            };
            return Err(err);
        }

        let tags: Vec<&EcsTag> = self
            .resource_tags
            .get(resource_arn)
            .map(|tags| tags.iter().collect())
            .unwrap_or_default();
        Ok(tags)
    }

    // ========================================================================
    // Helper methods
    // ========================================================================

    fn resolve_task_definition_arn(
        &self,
        task_def: &str,
        account_id: &str,
        region: &str,
    ) -> String {
        if task_def.starts_with("arn:aws:ecs:") {
            return task_def.to_string();
        }
        if task_def.contains(':')
            && let Some(td) = self.task_definitions.get(task_def)
        {
            return td.arn.clone();
        }
        if let Some(td) = self
            .task_definitions
            .values()
            .filter(|td| td.family == task_def)
            .max_by_key(|td| td.revision)
        {
            return td.arn.clone();
        }
        format!("arn:aws:ecs:{region}:{account_id}:task-definition/{task_def}:1")
    }

    fn resolve_cluster_name(&self, name_or_arn: &str) -> String {
        if name_or_arn.starts_with("arn:aws:ecs:") {
            name_or_arn
                .rsplit('/')
                .next()
                .unwrap_or(name_or_arn)
                .to_string()
        } else {
            name_or_arn.to_string()
        }
    }

    fn resolve_capacity_provider_name(&self, name_or_arn: &str) -> String {
        if name_or_arn.starts_with("arn:aws:ecs:") {
            name_or_arn
                .rsplit('/')
                .next()
                .unwrap_or(name_or_arn)
                .to_string()
        } else {
            name_or_arn.to_string()
        }
    }

    fn resolve_container_instance_arn(&self, ci_ref: &str, _cluster_name: &str) -> String {
        if ci_ref.starts_with("arn:aws:ecs:") {
            ci_ref.to_string()
        } else {
            // Try to find by partial match
            self.container_instances
                .keys()
                .find(|k| k.ends_with(ci_ref))
                .cloned()
                .unwrap_or_else(|| ci_ref.to_string())
        }
    }

    fn resource_exists(&self, arn: &str) -> bool {
        if arn.contains(":cluster/") {
            return self.clusters.values().any(|c| c.arn == arn);
        }
        if arn.contains(":task-definition/") {
            return self.task_definitions.values().any(|td| td.arn == arn);
        }
        if arn.contains(":service/") {
            return self.services.values().any(|s| s.arn == arn);
        }
        if arn.contains(":task/") {
            return self.tasks.contains_key(arn);
        }
        if arn.contains(":container-instance/") {
            return self.container_instances.contains_key(arn);
        }
        if arn.contains(":capacity-provider/") {
            return self.capacity_providers.values().any(|cp| cp.arn == arn);
        }
        if arn.contains(":task-set/") {
            return self.task_sets.contains_key(arn);
        }
        false
    }

    fn update_resource_tags(&mut self, arn: &str, tags: &[EcsTag], _replace: bool) {
        if arn.contains(":service/")
            && let Some(svc) = self.services.values_mut().find(|s| s.arn == arn)
        {
            for tag in tags {
                svc.tags.retain(|t| t.key != tag.key);
                svc.tags.push(tag.clone());
            }
        }
        if arn.contains(":cluster/")
            && let Some(c) = self.clusters.values_mut().find(|c| c.arn == arn)
        {
            for tag in tags {
                c.tags.retain(|t| t.key != tag.key);
                c.tags.push(tag.clone());
            }
        }
    }

    fn remove_resource_tags(&mut self, arn: &str, tag_keys: &[&str]) {
        if arn.contains(":service/")
            && let Some(svc) = self.services.values_mut().find(|s| s.arn == arn)
        {
            svc.tags.retain(|t| !tag_keys.contains(&t.key.as_str()));
        }
        if arn.contains(":cluster/")
            && let Some(c) = self.clusters.values_mut().find(|c| c.arn == arn)
        {
            c.tags.retain(|t| !tag_keys.contains(&t.key.as_str()));
        }
    }

    // ========================================================================
    // Task Protection operations
    // ========================================================================

    pub fn get_task_protection(
        &self,
        cluster: &str,
        tasks: &[&str],
    ) -> Result<Vec<&EcsTaskProtection>, EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        if !self.clusters.contains_key(&cluster_name) {
            return Err(EcsError::ClusterNotFoundPlain);
        }
        if tasks.is_empty() {
            // Return all protected tasks in the cluster
            let result: Vec<&EcsTaskProtection> = self
                .task_protections
                .values()
                .filter(|p| {
                    self.tasks
                        .get(&p.task_arn)
                        .map(|t| t.cluster_arn.ends_with(&format!("/{cluster_name}")))
                        .unwrap_or(false)
                })
                .collect();
            return Ok(result);
        }
        let result: Vec<&EcsTaskProtection> = tasks
            .iter()
            .filter_map(|t| {
                let arn = if t.starts_with("arn:aws:ecs:") {
                    t.to_string()
                } else {
                    self.tasks
                        .keys()
                        .find(|k| k.ends_with(t))
                        .cloned()
                        .unwrap_or_else(|| t.to_string())
                };
                self.task_protections.get(&arn)
            })
            .collect();
        Ok(result)
    }

    pub fn update_task_protection(
        &mut self,
        cluster: &str,
        tasks: &[&str],
        protection_enabled: bool,
        expires_in_minutes: Option<i64>,
    ) -> Result<Vec<&EcsTaskProtection>, EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        if !self.clusters.contains_key(&cluster_name) {
            return Err(EcsError::ClusterNotFoundPlain);
        }

        let expiration_date = expires_in_minutes.map(|m| {
            // Current time as unix timestamp + m minutes in seconds
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs_f64();
            now + (m as f64) * 60.0
        });

        let mut arns = Vec::new();
        for task_ref in tasks {
            let arn = if task_ref.starts_with("arn:aws:ecs:") {
                task_ref.to_string()
            } else {
                self.tasks
                    .keys()
                    .find(|k| k.ends_with(task_ref))
                    .cloned()
                    .unwrap_or_else(|| task_ref.to_string())
            };
            let protection = EcsTaskProtection {
                task_arn: arn.clone(),
                protection_enabled,
                expiration_date,
            };
            self.task_protections.insert(arn.clone(), protection);
            arns.push(arn);
        }
        let result: Vec<&EcsTaskProtection> = arns
            .iter()
            .filter_map(|a| self.task_protections.get(a))
            .collect();
        Ok(result)
    }

    // ========================================================================
    // Account Setting Default operations
    // ========================================================================

    pub fn put_account_setting_default(&mut self, name: &str, value: &str) -> (String, String) {
        self.account_settings_defaults
            .insert(name.to_string(), value.to_string());
        (name.to_string(), value.to_string())
    }

    // ========================================================================
    // Cluster Settings operations
    // ========================================================================

    pub fn update_cluster_settings(
        &mut self,
        cluster: &str,
        settings: Vec<EcsClusterSetting>,
    ) -> Result<&EcsCluster, EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        if !self.clusters.contains_key(&cluster_name) {
            return Err(EcsError::ClusterNotFound {
                cluster: cluster.to_string(),
            });
        }
        self.cluster_settings.insert(cluster_name.clone(), settings);
        Ok(self.clusters.get(&cluster_name).unwrap())
    }

    // ========================================================================
    // ListServicesByNamespace operations
    // ========================================================================

    pub fn list_services_by_namespace(&self, namespace: &str) -> Vec<&str> {
        // Services don't currently track namespace, so we return all services
        // whose ARN contains the namespace (conventional naming) or return empty.
        // In practice namespace would be stored on the service; for now return empty.
        let _ = namespace;
        Vec::new()
    }

    // ========================================================================
    // ExpressGatewayService operations
    // ========================================================================

    pub fn create_express_gateway_service(
        &mut self,
        service_name: &str,
        cluster: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&EcsExpressGatewayService, EcsError> {
        let cluster_name = self.resolve_cluster_name(cluster);
        if !self.clusters.contains_key(&cluster_name) {
            return Err(EcsError::ClusterNotFoundPlain);
        }

        let service_arn = format!(
            "arn:aws:ecs:{region}:{account_id}:express-gateway-service/{cluster_name}/{service_name}"
        );

        if self.express_gateway_services.contains_key(&service_arn) {
            return Err(EcsError::ExpressGatewayServiceAlreadyExists {
                service_name: service_name.to_string(),
            });
        }

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        let svc = EcsExpressGatewayService {
            service_arn: service_arn.clone(),
            service_name: service_name.to_string(),
            cluster: cluster_name,
            created_at: now,
            updated_at: now,
        };

        self.express_gateway_services
            .insert(service_arn.clone(), svc);
        Ok(self.express_gateway_services.get(&service_arn).unwrap())
    }

    pub fn delete_express_gateway_service(
        &mut self,
        service: &str,
    ) -> Result<EcsExpressGatewayService, EcsError> {
        let arn = self.resolve_express_gateway_service_arn(service);
        self.express_gateway_services.remove(&arn).ok_or_else(|| {
            EcsError::ExpressGatewayServiceNotFound {
                service: service.to_string(),
            }
        })
    }

    pub fn describe_express_gateway_service(
        &self,
        service: &str,
    ) -> Result<&EcsExpressGatewayService, EcsError> {
        let arn = self.resolve_express_gateway_service_arn(service);
        self.express_gateway_services.get(&arn).ok_or_else(|| {
            EcsError::ExpressGatewayServiceNotFound {
                service: service.to_string(),
            }
        })
    }

    pub fn update_express_gateway_service(
        &mut self,
        service: &str,
    ) -> Result<&EcsExpressGatewayService, EcsError> {
        let arn = self.resolve_express_gateway_service_arn(service);
        let svc = self.express_gateway_services.get_mut(&arn).ok_or_else(|| {
            EcsError::ExpressGatewayServiceNotFound {
                service: service.to_string(),
            }
        })?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();
        svc.updated_at = now;
        Ok(self.express_gateway_services.get(&arn).unwrap())
    }

    // ========================================================================
    // Poll Endpoint operations
    // ========================================================================

    pub fn get_or_create_poll_endpoint(&mut self, region: &str) -> &EcsPollEndpoint {
        self.poll_endpoints
            .entry(region.to_string())
            .or_insert_with(|| EcsPollEndpoint {
                endpoint: format!("https://ecs-a.{region}.amazonaws.com"),
                telemetry_endpoint: format!("https://ecs-t.{region}.amazonaws.com"),
            })
    }

    // ========================================================================
    // Submit state change operations (ECS agent internal APIs)
    // ========================================================================

    pub fn submit_attachment_state_change(&mut self) -> String {
        let ack = format!("ACK_{:020}", self.attachment_state_changes.len());
        self.attachment_state_changes
            .push(EcsAttachmentStateChange {
                acknowledgment: ack.clone(),
            });
        ack
    }

    pub fn submit_container_state_change(&mut self) -> String {
        let ack = format!("ACK_{:020}", self.container_state_changes.len());
        self.container_state_changes.push(EcsContainerStateChange {
            acknowledgment: ack.clone(),
        });
        ack
    }

    pub fn submit_task_state_change(&mut self) -> String {
        let ack = format!("ACK_{:020}", self.task_state_changes.len());
        self.task_state_changes.push(EcsTaskStateChange {
            acknowledgment: ack.clone(),
        });
        ack
    }

    // ========================================================================
    // Service Deployment operations
    // ========================================================================

    pub fn list_service_deployments(
        &self,
        service_arn: Option<&str>,
    ) -> Vec<&EcsServiceDeployment> {
        self.service_deployments
            .values()
            .filter(|d| {
                if let Some(arn) = service_arn {
                    d.service_arn == arn
                } else {
                    true
                }
            })
            .collect()
    }

    pub fn describe_service_deployments(
        &self,
        deployment_arns: &[&str],
    ) -> (Vec<&EcsServiceDeployment>, Vec<String>) {
        let mut found = Vec::new();
        let mut failures = Vec::new();
        for arn in deployment_arns {
            match self.service_deployments.get(*arn) {
                Some(d) => found.push(d),
                None => failures.push(arn.to_string()),
            }
        }
        (found, failures)
    }

    pub fn stop_service_deployment(
        &mut self,
        deployment_arn: &str,
    ) -> Result<&EcsServiceDeployment, EcsError> {
        let deployment = self
            .service_deployments
            .get_mut(deployment_arn)
            .ok_or_else(|| EcsError::ResourceNotFound {
                resource_arn: deployment_arn.to_string(),
            })?;
        deployment.status = "STOP_REQUESTED".to_string();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();
        deployment.updated_at = now;
        Ok(self.service_deployments.get(deployment_arn).unwrap())
    }

    // ========================================================================
    // Service Revision operations
    // ========================================================================

    pub fn describe_service_revisions(
        &self,
        revision_arns: &[&str],
    ) -> (Vec<&EcsServiceRevision>, Vec<String>) {
        let mut found = Vec::new();
        let mut failures = Vec::new();
        for arn in revision_arns {
            match self.service_revisions.get(*arn) {
                Some(r) => found.push(r),
                None => failures.push(arn.to_string()),
            }
        }
        (found, failures)
    }

    fn resolve_express_gateway_service_arn(&self, service: &str) -> String {
        if service.starts_with("arn:aws:ecs:") {
            return service.to_string();
        }
        // Try to find by service name
        self.express_gateway_services
            .keys()
            .find(|k| k.ends_with(&format!("/{service}")))
            .cloned()
            .unwrap_or_else(|| service.to_string())
    }
}
