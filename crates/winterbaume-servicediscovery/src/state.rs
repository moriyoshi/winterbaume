use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct ServiceDiscoveryState {
    pub namespaces: HashMap<String, Namespace>,
    pub services: HashMap<String, ServiceEntry>,
    pub instances: HashMap<String, HashMap<String, InstanceEntry>>,
    pub operations: HashMap<String, Operation>,
    pub instances_revision: i64,
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceDiscoveryError {
    #[error("A namespace with this VPC already exists: {vpc}")]
    ConflictingDomainExists { vpc: String },
    #[error("Namespace already exists with name: {name}")]
    NamespaceAlreadyExists { name: String },
    #[error("{id}")]
    NamespaceNotFound { id: String },
    #[error("Namespace still contains services")]
    NamespaceInUse,
    #[error("UpdateHttpNamespace can only be used with HTTP namespaces")]
    InvalidUpdateHttpNamespace,
    #[error("UpdatePrivateDnsNamespace can only be used with DNS_PRIVATE namespaces")]
    InvalidUpdatePrivateDnsNamespace,
    #[error("UpdatePublicDnsNamespace can only be used with DNS_PUBLIC namespaces")]
    InvalidUpdatePublicDnsNamespace,
    #[error("Service already exists with name: {name}")]
    ServiceAlreadyExists { name: String },
    #[error("{id}")]
    ServiceNotFound { id: String },
    #[error("Service still has registered instances")]
    ServiceInUse,
    #[error("{id}")]
    InstanceNotFound { id: String },
    #[error("{id}")]
    OperationNotFound { id: String },
    #[error("{arn}")]
    ResourceNotFound { arn: String },
}

impl ServiceDiscoveryState {
    fn create_operation(
        &mut self,
        operation_type: &str,
        target_key: &str,
        target_value: &str,
    ) -> &Operation {
        let op_id = format!("{}", uuid::Uuid::new_v4());
        let mut targets = HashMap::new();
        targets.insert(target_key.to_string(), target_value.to_string());
        let operation = Operation {
            id: op_id.clone(),
            operation_type: operation_type.to_string(),
            status: "SUCCESS".to_string(),
            targets,
            create_date: Utc::now(),
        };
        self.operations.insert(op_id.clone(), operation);
        self.operations.get(&op_id).unwrap()
    }

    pub fn create_private_dns_namespace(
        &mut self,
        name: &str,
        vpc: &str,
        description: Option<&str>,
        creator_request_id: Option<&str>,
        soa_ttl: Option<i64>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&Operation, ServiceDiscoveryError> {
        // Check for duplicate VPC (matches moto behavior: ConflictingDomainExists)
        for ns in self.namespaces.values() {
            if ns.namespace_type == "DNS_PRIVATE" && ns.vpc.as_deref() == Some(vpc) {
                return Err(ServiceDiscoveryError::ConflictingDomainExists {
                    vpc: vpc.to_string(),
                });
            }
        }

        let ns_id = format!("ns-{}", uuid::Uuid::new_v4().simple());
        let arn = format!("arn:aws:servicediscovery:{region}:{account_id}:namespace/{ns_id}");
        let hosted_zone_id = format!(
            "Z{}",
            &uuid::Uuid::new_v4().simple().to_string()[..13].to_uppercase()
        );

        let namespace = Namespace {
            id: ns_id.clone(),
            arn,
            name: name.to_string(),
            namespace_type: "DNS_PRIVATE".to_string(),
            description: description.map(|s| s.to_string()),
            creator_request_id: creator_request_id.map(|s| s.to_string()),
            vpc: Some(vpc.to_string()),
            hosted_zone_id: Some(hosted_zone_id),
            soa_ttl,
            service_count: 0,
            create_date: Utc::now(),
            tags,
        };

        self.namespaces.insert(ns_id.clone(), namespace);
        Ok(self.create_operation("CREATE_NAMESPACE", "NAMESPACE", &ns_id))
    }

    pub fn create_http_namespace(
        &mut self,
        name: &str,
        description: Option<&str>,
        creator_request_id: Option<&str>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&Operation, ServiceDiscoveryError> {
        for ns in self.namespaces.values() {
            if ns.name == name && ns.namespace_type == "HTTP" {
                return Err(ServiceDiscoveryError::NamespaceAlreadyExists {
                    name: name.to_string(),
                });
            }
        }

        let ns_id = format!("ns-{}", uuid::Uuid::new_v4().simple());
        let arn = format!("arn:aws:servicediscovery:{region}:{account_id}:namespace/{ns_id}");

        let namespace = Namespace {
            id: ns_id.clone(),
            arn,
            name: name.to_string(),
            namespace_type: "HTTP".to_string(),
            description: description.map(|s| s.to_string()),
            creator_request_id: creator_request_id.map(|s| s.to_string()),
            vpc: None,
            hosted_zone_id: None,
            soa_ttl: None,
            service_count: 0,
            create_date: Utc::now(),
            tags,
        };

        self.namespaces.insert(ns_id.clone(), namespace);
        Ok(self.create_operation("CREATE_NAMESPACE", "NAMESPACE", &ns_id))
    }

    pub fn create_public_dns_namespace(
        &mut self,
        name: &str,
        description: Option<&str>,
        creator_request_id: Option<&str>,
        soa_ttl: Option<i64>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&Operation, ServiceDiscoveryError> {
        for ns in self.namespaces.values() {
            if ns.name == name && ns.namespace_type == "DNS_PUBLIC" {
                return Err(ServiceDiscoveryError::NamespaceAlreadyExists {
                    name: name.to_string(),
                });
            }
        }

        let ns_id = format!("ns-{}", uuid::Uuid::new_v4().simple());
        let arn = format!("arn:aws:servicediscovery:{region}:{account_id}:namespace/{ns_id}");
        let hosted_zone_id = format!(
            "Z{}",
            &uuid::Uuid::new_v4().simple().to_string()[..13].to_uppercase()
        );

        let namespace = Namespace {
            id: ns_id.clone(),
            arn,
            name: name.to_string(),
            namespace_type: "DNS_PUBLIC".to_string(),
            description: description.map(|s| s.to_string()),
            creator_request_id: creator_request_id.map(|s| s.to_string()),
            vpc: None,
            hosted_zone_id: Some(hosted_zone_id),
            soa_ttl,
            service_count: 0,
            create_date: Utc::now(),
            tags,
        };

        self.namespaces.insert(ns_id.clone(), namespace);
        Ok(self.create_operation("CREATE_NAMESPACE", "NAMESPACE", &ns_id))
    }

    pub fn get_namespace(&self, id: &str) -> Result<&Namespace, ServiceDiscoveryError> {
        let ns = if id.starts_with("arn:") {
            self.namespaces.values().find(|ns| ns.arn == id)
        } else {
            self.namespaces.get(id)
        };

        ns.ok_or_else(|| ServiceDiscoveryError::NamespaceNotFound { id: id.to_string() })
    }

    pub fn delete_namespace(&mut self, id: &str) -> Result<String, ServiceDiscoveryError> {
        let ns_id = if id.starts_with("arn:") {
            self.namespaces
                .values()
                .find(|ns| ns.arn == id)
                .map(|ns| ns.id.clone())
        } else if self.namespaces.contains_key(id) {
            Some(id.to_string())
        } else {
            None
        };

        let ns_id =
            ns_id.ok_or_else(|| ServiceDiscoveryError::NamespaceNotFound { id: id.to_string() })?;

        let ns = self.namespaces.get(&ns_id).unwrap();
        if ns.service_count > 0 {
            return Err(ServiceDiscoveryError::NamespaceInUse);
        }

        self.namespaces.remove(&ns_id);
        // Clear all operations related to this namespace (matches moto behavior)
        self.operations.retain(|_, op| {
            op.targets.get("NAMESPACE").map(|v| v.as_str()) != Some(ns_id.as_str())
        });
        // Generate an operation ID to return but don't store it
        let op_id = format!("{}", uuid::Uuid::new_v4());
        Ok(op_id)
    }

    pub fn list_namespaces(&self) -> Vec<&Namespace> {
        self.namespaces.values().collect()
    }

    pub fn update_http_namespace(
        &mut self,
        id: &str,
        description: Option<&str>,
    ) -> Result<&Operation, ServiceDiscoveryError> {
        let ns = self
            .namespaces
            .get_mut(id)
            .ok_or_else(|| ServiceDiscoveryError::NamespaceNotFound { id: id.to_string() })?;
        if ns.namespace_type != "HTTP" {
            return Err(ServiceDiscoveryError::InvalidUpdateHttpNamespace);
        }
        if let Some(desc) = description {
            ns.description = Some(desc.to_string());
        }
        Ok(self.create_operation("UPDATE_NAMESPACE", "NAMESPACE", id))
    }

    pub fn update_private_dns_namespace(
        &mut self,
        id: &str,
        description: Option<&str>,
        soa_ttl: Option<i64>,
    ) -> Result<&Operation, ServiceDiscoveryError> {
        let ns = self
            .namespaces
            .get_mut(id)
            .ok_or_else(|| ServiceDiscoveryError::NamespaceNotFound { id: id.to_string() })?;
        if ns.namespace_type != "DNS_PRIVATE" {
            return Err(ServiceDiscoveryError::InvalidUpdatePrivateDnsNamespace);
        }
        if let Some(desc) = description {
            ns.description = Some(desc.to_string());
        }
        if let Some(ttl) = soa_ttl {
            ns.soa_ttl = Some(ttl);
        }
        Ok(self.create_operation("UPDATE_NAMESPACE", "NAMESPACE", id))
    }

    pub fn update_public_dns_namespace(
        &mut self,
        id: &str,
        description: Option<&str>,
        soa_ttl: Option<i64>,
    ) -> Result<&Operation, ServiceDiscoveryError> {
        let ns = self
            .namespaces
            .get_mut(id)
            .ok_or_else(|| ServiceDiscoveryError::NamespaceNotFound { id: id.to_string() })?;
        if ns.namespace_type != "DNS_PUBLIC" {
            return Err(ServiceDiscoveryError::InvalidUpdatePublicDnsNamespace);
        }
        if let Some(desc) = description {
            ns.description = Some(desc.to_string());
        }
        if let Some(ttl) = soa_ttl {
            ns.soa_ttl = Some(ttl);
        }
        Ok(self.create_operation("UPDATE_NAMESPACE", "NAMESPACE", id))
    }

    pub fn create_service(
        &mut self,
        name: &str,
        namespace_id: Option<&str>,
        description: Option<&str>,
        creator_request_id: Option<&str>,
        dns_config: Option<DnsConfigEntry>,
        health_check_config: Option<HealthCheckConfigEntry>,
        health_check_custom_config: Option<HealthCheckCustomConfigEntry>,
        service_type_override: Option<&str>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&ServiceEntry, ServiceDiscoveryError> {
        let ns_id = namespace_id.unwrap_or("");

        // Verify namespace exists if provided
        if !ns_id.is_empty() && !self.namespaces.contains_key(ns_id) {
            return Err(ServiceDiscoveryError::NamespaceNotFound {
                id: ns_id.to_string(),
            });
        }

        // Check for duplicate name in the same namespace
        if !ns_id.is_empty() {
            for svc in self.services.values() {
                if svc.name == name && svc.namespace_id == ns_id {
                    return Err(ServiceDiscoveryError::ServiceAlreadyExists {
                        name: name.to_string(),
                    });
                }
            }
        }

        let svc_id = format!("srv-{}", uuid::Uuid::new_v4().simple());
        let arn = format!("arn:aws:servicediscovery:{region}:{account_id}:service/{svc_id}");

        // Determine service type
        let service_type = if let Some(t) = service_type_override {
            Some(t.to_string())
        } else if !ns_id.is_empty() {
            let ns = self.namespaces.get(ns_id).unwrap();
            if dns_config.is_some() {
                if ns.namespace_type == "HTTP" {
                    None
                } else {
                    Some("DNS".to_string())
                }
            } else {
                Some("HTTP".to_string())
            }
        } else {
            None
        };

        // When DnsConfig is provided with NamespaceId, don't set top-level namespace_id
        let effective_ns_id =
            if dns_config.is_some() && dns_config.as_ref().unwrap().namespace_id.is_some() {
                // If DnsConfig has its own NamespaceId, the top-level NamespaceId is not set in response
                ns_id.to_string()
            } else {
                ns_id.to_string()
            };

        // Determine whether to include namespace_id in response
        // Moto: when DnsConfig is provided, NamespaceId is NOT in the create_service response
        let include_namespace_id = dns_config.is_none();

        let service = ServiceEntry {
            id: svc_id.clone(),
            arn,
            name: name.to_string(),
            namespace_id: effective_ns_id.clone(),
            description: description.map(|s| s.to_string()),
            creator_request_id: creator_request_id.map(|s| s.to_string()),
            dns_config,
            health_check_config,
            health_check_custom_config,
            instance_count: 0,
            create_date: Utc::now(),
            tags,
            service_type,
            include_namespace_id_in_response: include_namespace_id,
        };

        // Update namespace service count
        if !effective_ns_id.is_empty() {
            if let Some(ns) = self.namespaces.get_mut(&effective_ns_id) {
                ns.service_count += 1;
            }
        }

        self.services.insert(svc_id.clone(), service);
        self.instances.insert(svc_id.clone(), HashMap::new());
        Ok(self.services.get(&svc_id).unwrap())
    }

    pub fn get_service(&self, id: &str) -> Result<&ServiceEntry, ServiceDiscoveryError> {
        self.services
            .get(id)
            .ok_or_else(|| ServiceDiscoveryError::ServiceNotFound { id: id.to_string() })
    }

    pub fn delete_service(&mut self, id: &str) -> Result<(), ServiceDiscoveryError> {
        let svc = self
            .services
            .get(id)
            .ok_or_else(|| ServiceDiscoveryError::ServiceNotFound { id: id.to_string() })?;

        if svc.instance_count > 0 {
            return Err(ServiceDiscoveryError::ServiceInUse);
        }

        let namespace_id = svc.namespace_id.clone();
        self.services.remove(id);
        self.instances.remove(id);

        if let Some(ns) = self.namespaces.get_mut(&namespace_id) {
            ns.service_count -= 1;
        }

        Ok(())
    }

    pub fn list_services(&self) -> Vec<&ServiceEntry> {
        self.services.values().collect()
    }

    pub fn update_service(
        &mut self,
        id: &str,
        description: Option<&str>,
        dns_config_change: Option<Vec<DnsRecordEntry>>,
        has_dns_config: bool,
        health_check_config: Option<HealthCheckConfigEntry>,
    ) -> Result<&Operation, ServiceDiscoveryError> {
        let svc = self
            .services
            .get_mut(id)
            .ok_or_else(|| ServiceDiscoveryError::ServiceNotFound { id: id.to_string() })?;

        if let Some(desc) = description {
            svc.description = Some(desc.to_string());
        }

        // If DnsConfig was explicitly provided in the update:
        if has_dns_config {
            if let Some(records) = dns_config_change {
                if let Some(ref mut dns) = svc.dns_config {
                    dns.dns_records = records;
                }
            }
        } else {
            // If DnsConfig was NOT provided in update, delete it (AWS behavior)
            svc.dns_config = None;
        }

        if let Some(hcc) = health_check_config {
            svc.health_check_config = Some(hcc);
        }

        let svc_id = id.to_string();
        Ok(self.create_operation("UPDATE_SERVICE", "SERVICE", &svc_id))
    }

    pub fn register_instance(
        &mut self,
        service_id: &str,
        instance_id: &str,
        creator_request_id: Option<&str>,
        attributes: HashMap<String, String>,
    ) -> Result<&Operation, ServiceDiscoveryError> {
        if !self.services.contains_key(service_id) {
            return Err(ServiceDiscoveryError::ServiceNotFound {
                id: service_id.to_string(),
            });
        }

        let instance = InstanceEntry {
            id: instance_id.to_string(),
            service_id: service_id.to_string(),
            creator_request_id: creator_request_id.map(|s| s.to_string()),
            attributes,
            health_status: "HEALTHY".to_string(),
        };

        let instances = self.instances.entry(service_id.to_string()).or_default();
        let is_new = !instances.contains_key(instance_id);
        instances.insert(instance_id.to_string(), instance);

        if is_new {
            if let Some(svc) = self.services.get_mut(service_id) {
                svc.instance_count += 1;
            }
        }

        self.instances_revision += 1;

        let iid = instance_id.to_string();
        Ok(self.create_operation("REGISTER_INSTANCE", "INSTANCE", &iid))
    }

    pub fn deregister_instance(
        &mut self,
        service_id: &str,
        instance_id: &str,
    ) -> Result<&Operation, ServiceDiscoveryError> {
        if !self.services.contains_key(service_id) {
            return Err(ServiceDiscoveryError::ServiceNotFound {
                id: service_id.to_string(),
            });
        }

        let instances = self.instances.get_mut(service_id).ok_or_else(|| {
            ServiceDiscoveryError::InstanceNotFound {
                id: instance_id.to_string(),
            }
        })?;

        if instances.remove(instance_id).is_none() {
            return Err(ServiceDiscoveryError::InstanceNotFound {
                id: instance_id.to_string(),
            });
        }

        if let Some(svc) = self.services.get_mut(service_id) {
            svc.instance_count -= 1;
        }

        self.instances_revision += 1;

        let iid = instance_id.to_string();
        Ok(self.create_operation("DEREGISTER_INSTANCE", "INSTANCE", &iid))
    }

    pub fn get_instance(
        &self,
        service_id: &str,
        instance_id: &str,
    ) -> Result<&InstanceEntry, ServiceDiscoveryError> {
        if !self.services.contains_key(service_id) {
            return Err(ServiceDiscoveryError::ServiceNotFound {
                id: service_id.to_string(),
            });
        }

        self.instances
            .get(service_id)
            .and_then(|m| m.get(instance_id))
            .ok_or_else(|| ServiceDiscoveryError::InstanceNotFound {
                id: instance_id.to_string(),
            })
    }

    pub fn list_instances(
        &self,
        service_id: &str,
    ) -> Result<Vec<&InstanceEntry>, ServiceDiscoveryError> {
        if !self.services.contains_key(service_id) {
            return Err(ServiceDiscoveryError::ServiceNotFound {
                id: service_id.to_string(),
            });
        }

        Ok(self
            .instances
            .get(service_id)
            .map(|m| m.values().collect())
            .unwrap_or_default())
    }

    pub fn get_instances_health_status(
        &self,
        service_id: &str,
    ) -> Result<HashMap<String, String>, ServiceDiscoveryError> {
        if !self.services.contains_key(service_id) {
            return Err(ServiceDiscoveryError::ServiceNotFound {
                id: service_id.to_string(),
            });
        }

        let mut status = HashMap::new();
        if let Some(instances) = self.instances.get(service_id) {
            for (id, inst) in instances {
                status.insert(id.clone(), inst.health_status.clone());
            }
        }
        Ok(status)
    }

    pub fn update_instance_custom_health_status(
        &mut self,
        service_id: &str,
        instance_id: &str,
        status: &str,
    ) -> Result<(), ServiceDiscoveryError> {
        if !self.services.contains_key(service_id) {
            return Err(ServiceDiscoveryError::ServiceNotFound {
                id: service_id.to_string(),
            });
        }

        let instances = self.instances.get_mut(service_id).ok_or_else(|| {
            ServiceDiscoveryError::InstanceNotFound {
                id: instance_id.to_string(),
            }
        })?;

        let inst = instances.get_mut(instance_id).ok_or_else(|| {
            ServiceDiscoveryError::InstanceNotFound {
                id: instance_id.to_string(),
            }
        })?;

        inst.health_status = status.to_string();
        Ok(())
    }

    pub fn discover_instances(
        &self,
        namespace_name: &str,
        service_name: &str,
        health_status_filter: Option<&str>,
        query_parameters: Option<&HashMap<String, String>>,
        optional_parameters: Option<&HashMap<String, String>>,
        max_results: Option<usize>,
    ) -> Result<(Vec<(&InstanceEntry, String, String)>, i64), ServiceDiscoveryError> {
        // Find the namespace by name
        let ns = self
            .namespaces
            .values()
            .find(|ns| ns.name == namespace_name)
            .ok_or_else(|| ServiceDiscoveryError::NamespaceNotFound {
                id: namespace_name.to_string(),
            })?;

        // Find the service in that namespace
        let svc = self
            .services
            .values()
            .find(|s| s.namespace_id == ns.id && s.name == service_name)
            .ok_or_else(|| ServiceDiscoveryError::ServiceNotFound {
                id: service_name.to_string(),
            })?;

        let filter = health_status_filter.unwrap_or("ALL");

        let mut all_results: Vec<(&InstanceEntry, String, String)> = Vec::new();
        let mut healthy_results: Vec<(&InstanceEntry, String, String)> = Vec::new();

        if let Some(instances) = self.instances.get(&svc.id) {
            for inst in instances.values() {
                // Apply query parameter filters (required match)
                if let Some(qp) = query_parameters {
                    let mut matches_all = true;
                    for (k, v) in qp {
                        if inst.attributes.get(k) != Some(v) {
                            matches_all = false;
                            break;
                        }
                    }
                    if !matches_all {
                        continue;
                    }
                }

                // Apply optional parameter filters (prefer matches, but include non-matches)
                let matches_optional = if let Some(op) = optional_parameters {
                    op.iter().all(|(k, v)| inst.attributes.get(k) == Some(v))
                } else {
                    true
                };

                let health_match = match filter {
                    "HEALTHY" => inst.health_status == "HEALTHY",
                    "UNHEALTHY" => inst.health_status == "UNHEALTHY",
                    "ALL" | "HEALTHY_OR_ELSE_ALL" => true,
                    _ => true,
                };

                if health_match {
                    if inst.health_status == "HEALTHY" {
                        healthy_results.push((
                            inst,
                            namespace_name.to_string(),
                            service_name.to_string(),
                        ));
                    }
                    all_results.push((inst, namespace_name.to_string(), service_name.to_string()));
                }

                // If optional parameters, keep only those that match
                if optional_parameters.is_some() && !matches_optional {
                    // Remove the last added element from both lists if it doesn't match optional
                    all_results.pop();
                    if inst.health_status == "HEALTHY" {
                        healthy_results.pop();
                    }
                    // But we still want it in the result if no optional-matching instances exist
                    // We'll handle this below
                }
            }
        }

        // For optional parameters, we need a different approach: include all that match query params,
        // but sort optional-matching ones first. Actually, moto's behavior is:
        // - QueryParameters: required filter
        // - OptionalParameters: further filter, but if no instances match, return all query-matched instances
        // Let me re-implement this properly.
        let mut results = if let Some(op_params) = optional_parameters {
            // Re-collect with proper optional parameter handling
            let mut matching = Vec::new();
            let mut non_matching = Vec::new();
            if let Some(instances) = self.instances.get(&svc.id) {
                for inst in instances.values() {
                    // Apply query parameter filters
                    if let Some(qp) = query_parameters {
                        if !qp.iter().all(|(k, v)| inst.attributes.get(k) == Some(v)) {
                            continue;
                        }
                    }

                    let health_match = match filter {
                        "HEALTHY" => inst.health_status == "HEALTHY",
                        "UNHEALTHY" => inst.health_status == "UNHEALTHY",
                        "ALL" | "HEALTHY_OR_ELSE_ALL" => true,
                        _ => true,
                    };
                    if !health_match {
                        continue;
                    }

                    let op = op_params;
                    if op.iter().all(|(k, v)| inst.attributes.get(k) == Some(v)) {
                        matching.push((inst, namespace_name.to_string(), service_name.to_string()));
                    } else {
                        non_matching.push((
                            inst,
                            namespace_name.to_string(),
                            service_name.to_string(),
                        ));
                    }
                }
            }
            if matching.is_empty() {
                // Return all query-matched instances
                matching.extend(non_matching);
            }
            matching
        } else {
            // No optional parameters - just use query + health filter
            let mut results = Vec::new();
            if let Some(instances) = self.instances.get(&svc.id) {
                for inst in instances.values() {
                    if let Some(qp) = query_parameters {
                        if !qp.iter().all(|(k, v)| inst.attributes.get(k) == Some(v)) {
                            continue;
                        }
                    }

                    let health_match = match filter {
                        "HEALTHY" => inst.health_status == "HEALTHY",
                        "UNHEALTHY" => inst.health_status == "UNHEALTHY",
                        "ALL" => true,
                        "HEALTHY_OR_ELSE_ALL" => true, // handled below
                        _ => true,
                    };
                    if health_match {
                        results.push((inst, namespace_name.to_string(), service_name.to_string()));
                    }
                }
            }
            results
        };

        // Handle HEALTHY_OR_ELSE_ALL: if there are healthy instances, return only healthy ones
        if filter == "HEALTHY_OR_ELSE_ALL" {
            let healthy: Vec<_> = results
                .iter()
                .filter(|(inst, _, _)| inst.health_status == "HEALTHY")
                .cloned()
                .collect();
            if !healthy.is_empty() {
                results = healthy;
            }
            // else return all
        }

        // Apply MaxResults
        if let Some(max) = max_results {
            results.truncate(max);
        }

        Ok((results, self.instances_revision))
    }

    pub fn discover_instances_revision(
        &self,
        namespace_name: &str,
        service_name: &str,
    ) -> Result<i64, ServiceDiscoveryError> {
        // Verify namespace exists by name
        let ns = self
            .namespaces
            .values()
            .find(|ns| ns.name == namespace_name)
            .ok_or_else(|| ServiceDiscoveryError::NamespaceNotFound {
                id: namespace_name.to_string(),
            })?;

        // Verify service exists
        self.services
            .values()
            .find(|s| s.namespace_id == ns.id && s.name == service_name)
            .ok_or_else(|| ServiceDiscoveryError::ServiceNotFound {
                id: service_name.to_string(),
            })?;

        Ok(self.instances_revision)
    }

    pub fn get_operation(&self, id: &str) -> Result<&Operation, ServiceDiscoveryError> {
        self.operations
            .get(id)
            .ok_or_else(|| ServiceDiscoveryError::OperationNotFound { id: id.to_string() })
    }

    pub fn list_operations(&self) -> Vec<&Operation> {
        self.operations.values().collect()
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), ServiceDiscoveryError> {
        // Try namespaces
        for ns in self.namespaces.values_mut() {
            if ns.arn == resource_arn {
                ns.tags.extend(tags);
                return Ok(());
            }
        }
        // Try services
        for svc in self.services.values_mut() {
            if svc.arn == resource_arn {
                svc.tags.extend(tags);
                return Ok(());
            }
        }
        Err(ServiceDiscoveryError::ResourceNotFound {
            arn: resource_arn.to_string(),
        })
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), ServiceDiscoveryError> {
        // Try namespaces
        for ns in self.namespaces.values_mut() {
            if ns.arn == resource_arn {
                for key in tag_keys {
                    ns.tags.remove(key);
                }
                return Ok(());
            }
        }
        // Try services
        for svc in self.services.values_mut() {
            if svc.arn == resource_arn {
                for key in tag_keys {
                    svc.tags.remove(key);
                }
                return Ok(());
            }
        }
        Err(ServiceDiscoveryError::ResourceNotFound {
            arn: resource_arn.to_string(),
        })
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<&HashMap<String, String>, ServiceDiscoveryError> {
        // Try namespaces
        for ns in self.namespaces.values() {
            if ns.arn == resource_arn {
                return Ok(&ns.tags);
            }
        }
        // Try services
        for svc in self.services.values() {
            if svc.arn == resource_arn {
                return Ok(&svc.tags);
            }
        }
        Err(ServiceDiscoveryError::ResourceNotFound {
            arn: resource_arn.to_string(),
        })
    }
}
