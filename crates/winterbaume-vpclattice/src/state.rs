use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct VpcLatticeState {
    pub service_networks: HashMap<String, ServiceNetwork>,
    pub access_log_subscriptions: HashMap<String, AccessLogSubscription>,
    pub sn_service_associations: HashMap<String, ServiceNetworkServiceAssociation>,
    pub sn_vpc_associations: HashMap<String, ServiceNetworkVpcAssociation>,
    pub target_groups: HashMap<String, TargetGroup>,
    pub services: HashMap<String, Service>,
    pub auth_policies: HashMap<String, AuthPolicy>, // keyed by resource ARN/id
    pub resource_policies: HashMap<String, ResourcePolicy>, // keyed by resource ARN
    pub listeners: HashMap<String, Listener>,       // keyed by listener id
    pub rules: HashMap<String, Rule>,               // keyed by rule id
    pub resource_configurations: HashMap<String, ResourceConfiguration>,
    pub resource_gateways: HashMap<String, ResourceGateway>,
    pub sn_resource_associations: HashMap<String, ServiceNetworkResourceAssociation>,
    pub domain_verifications: HashMap<String, DomainVerification>,
}

#[derive(Debug, thiserror::Error)]
pub enum VpcLatticeError {
    #[error("A ServiceNetwork with name '{0}' already exists")]
    ServiceNetworkAlreadyExists(String),
    #[error("A TargetGroup with name '{0}' already exists")]
    TargetGroupAlreadyExists(String),
    #[error("A Service with name '{0}' already exists")]
    ServiceAlreadyExists(String),
    #[error("Association already exists")]
    AssociationAlreadyExists,
    #[error("ServiceNetwork '{0}' not found")]
    ServiceNetworkNotFound(String),
    #[error("Access Log Subscription {0} not found")]
    AccessLogSubscriptionNotFound(String),
    #[error("Resource {0} not found")]
    ResourceNotFound(String),
    #[error("ServiceNetworkServiceAssociation '{0}' not found")]
    ServiceNetworkServiceAssociationNotFound(String),
    #[error("ServiceNetworkVpcAssociation '{0}' not found")]
    ServiceNetworkVpcAssociationNotFound(String),
    #[error("TargetGroup '{0}' not found")]
    TargetGroupNotFound(String),
    #[error("Service '{0}' not found")]
    ServiceNotFound(String),
    #[error("Auth policy for '{0}' not found")]
    AuthPolicyNotFound(String),
    #[error("Resource policy for '{0}' not found")]
    ResourcePolicyNotFound(String),
    #[error("Resource '{0}' not found")]
    TagResourceNotFound(String),
    #[error("Listener '{0}' not found")]
    ListenerNotFound(String),
    #[error("Rule '{0}' not found")]
    RuleNotFound(String),
    #[error("ResourceConfiguration '{0}' not found")]
    ResourceConfigurationNotFound(String),
    #[error("ResourceGateway '{0}' not found")]
    ResourceGatewayNotFound(String),
    #[error("ServiceNetworkResourceAssociation '{0}' not found")]
    ServiceNetworkResourceAssociationNotFound(String),
    #[error("DomainVerification '{0}' not found")]
    DomainVerificationNotFound(String),
    #[error("Invalid parameter resourceIdentifier, must start with 'sn-' or 'svc-'")]
    InvalidResourceIdentifier,
}

impl VpcLatticeState {
    // ── ServiceNetwork ──────────────────────────────────────────────

    pub fn create_service_network(
        &mut self,
        name: &str,
        auth_type: Option<&str>,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&ServiceNetwork, VpcLatticeError> {
        if self.service_networks.values().any(|sn| sn.name == name) {
            return Err(VpcLatticeError::ServiceNetworkAlreadyExists(
                name.to_string(),
            ));
        }

        let id = format!("sn-{}", &uuid::Uuid::new_v4().to_string()[..17]);
        let arn = format!("arn:aws:vpc-lattice:{region}:{account_id}:servicenetwork/{id}");
        let now = Utc::now();

        let sn = ServiceNetwork {
            id: id.clone(),
            name: name.to_string(),
            arn,
            auth_type: auth_type.unwrap_or("NONE").to_string(),
            created_at: now,
            last_updated_at: now,
            number_of_associated_services: 0,
            number_of_associated_v_p_cs: 0,
            tags,
        };

        self.service_networks.insert(id.clone(), sn);
        Ok(self.service_networks.get(&id).unwrap())
    }

    pub fn get_service_network(
        &self,
        identifier: &str,
    ) -> Result<&ServiceNetwork, VpcLatticeError> {
        self.resolve_service_network(identifier)
            .ok_or_else(|| VpcLatticeError::ServiceNetworkNotFound(identifier.to_string()))
    }

    pub fn delete_service_network(&mut self, identifier: &str) -> Result<(), VpcLatticeError> {
        let id = self
            .resolve_service_network(identifier)
            .map(|sn| sn.id.clone())
            .ok_or_else(|| VpcLatticeError::ServiceNetworkNotFound(identifier.to_string()))?;
        self.service_networks.remove(&id);
        Ok(())
    }

    pub fn list_service_networks(&self) -> Vec<&ServiceNetwork> {
        self.service_networks.values().collect()
    }

    fn resolve_service_network(&self, identifier: &str) -> Option<&ServiceNetwork> {
        if let Some(sn) = self.service_networks.get(identifier) {
            return Some(sn);
        }
        if let Some(sn) = self
            .service_networks
            .values()
            .find(|sn| sn.arn == identifier)
        {
            return Some(sn);
        }
        self.service_networks
            .values()
            .find(|sn| sn.name == identifier)
    }

    // ── AccessLogSubscription ───────────────────────────────────────

    pub fn create_access_log_subscription(
        &mut self,
        resource_identifier: &str,
        destination_arn: &str,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&AccessLogSubscription, VpcLatticeError> {
        let id = format!("als-{}", &uuid::Uuid::new_v4().to_string()[..17]);
        let arn = format!("arn:aws:vpc-lattice:{region}:{account_id}:accesslogsubscription/{id}");
        let now = Utc::now();

        // Resolve resource ARN/ID
        let (resource_arn, resource_id) =
            self.resolve_resource_for_log(resource_identifier, account_id, region)?;

        let sub = AccessLogSubscription {
            id: id.clone(),
            arn,
            resource_arn,
            resource_id,
            destination_arn: destination_arn.to_string(),
            created_at: now,
            last_updated_at: now,
            tags,
        };

        self.access_log_subscriptions.insert(id.clone(), sub);
        Ok(self.access_log_subscriptions.get(&id).unwrap())
    }

    pub fn get_access_log_subscription(
        &self,
        identifier: &str,
    ) -> Result<&AccessLogSubscription, VpcLatticeError> {
        self.resolve_access_log_subscription(identifier)
            .ok_or_else(|| VpcLatticeError::AccessLogSubscriptionNotFound(identifier.to_string()))
    }

    pub fn delete_access_log_subscription(
        &mut self,
        identifier: &str,
    ) -> Result<(), VpcLatticeError> {
        let id = self
            .resolve_access_log_subscription(identifier)
            .map(|s| s.id.clone())
            .ok_or_else(|| {
                VpcLatticeError::AccessLogSubscriptionNotFound(identifier.to_string())
            })?;
        self.access_log_subscriptions.remove(&id);
        Ok(())
    }

    pub fn list_access_log_subscriptions(
        &self,
        resource_identifier: &str,
    ) -> Vec<&AccessLogSubscription> {
        self.access_log_subscriptions
            .values()
            .filter(|s| {
                s.resource_arn == resource_identifier || s.resource_id == resource_identifier
            })
            .collect()
    }

    fn resolve_access_log_subscription(&self, identifier: &str) -> Option<&AccessLogSubscription> {
        if let Some(s) = self.access_log_subscriptions.get(identifier) {
            return Some(s);
        }
        self.access_log_subscriptions
            .values()
            .find(|s| s.arn == identifier)
    }

    fn resolve_resource_for_log(
        &self,
        identifier: &str,
        account_id: &str,
        region: &str,
    ) -> Result<(String, String), VpcLatticeError> {
        // Try service network (by id or arn)
        if let Some(sn) = self.resolve_service_network(identifier) {
            return Ok((sn.arn.clone(), sn.id.clone()));
        }
        // If identifier looks like an ARN, extract resource type and id
        if identifier.starts_with("arn:") {
            // Extract id from ARN
            let id_part = identifier.rsplit('/').next().unwrap_or(identifier);
            // Try to resolve it as service network or service
            if let Some(sn) = self.resolve_service_network(id_part) {
                return Ok((sn.arn.clone(), sn.id.clone()));
            }
            return Ok((identifier.to_string(), id_part.to_string()));
        }
        // Validate the prefix: must start with 'sn-' or 'svc-'
        if identifier.starts_with("sn-") {
            // It's a service-network ID, but not found
            return Err(VpcLatticeError::ResourceNotFound(identifier.to_string()));
        }
        if identifier.starts_with("svc-") {
            // Try service by id
            if let Some(svc) = self.resolve_service(identifier) {
                return Ok((svc.arn.clone(), svc.id.clone()));
            }
            return Err(VpcLatticeError::ResourceNotFound(identifier.to_string()));
        }
        // Invalid prefix
        Err(VpcLatticeError::InvalidResourceIdentifier)
    }

    // ── ServiceNetworkServiceAssociation ────────────────────────────

    pub fn create_service_network_service_association(
        &mut self,
        service_network_identifier: &str,
        service_identifier: &str,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&ServiceNetworkServiceAssociation, VpcLatticeError> {
        // Check for duplicate
        if self.sn_service_associations.values().any(|a| {
            (a.service_network_identifier == service_network_identifier
                || a.service_network_id == service_network_identifier
                || a.service_network_arn == service_network_identifier)
                && (a.service_identifier == service_identifier
                    || a.service_id == service_identifier
                    || a.service_arn == service_identifier)
        }) {
            return Err(VpcLatticeError::AssociationAlreadyExists);
        }

        let id = format!("snsa-{}", &uuid::Uuid::new_v4().to_string()[..17]);
        let arn = format!(
            "arn:aws:vpc-lattice:{region}:{account_id}:servicenetworkserviceassociation/{id}"
        );
        let now = Utc::now();

        // Resolve service network
        let (sn_id, sn_arn, sn_name) = if let Some(sn) =
            self.resolve_service_network(service_network_identifier)
        {
            (sn.id.clone(), sn.arn.clone(), sn.name.clone())
        } else {
            (
                service_network_identifier.to_string(),
                format!(
                    "arn:aws:vpc-lattice:{region}:{account_id}:servicenetwork/{service_network_identifier}"
                ),
                service_network_identifier.to_string(),
            )
        };

        // For service, use the identifier as-is (we don't track services in state yet)
        let svc_id = service_identifier.to_string();
        let svc_arn = if service_identifier.starts_with("arn:") {
            service_identifier.to_string()
        } else {
            format!("arn:aws:vpc-lattice:{region}:{account_id}:service/{service_identifier}")
        };
        let svc_name = service_identifier.to_string();

        let assoc = ServiceNetworkServiceAssociation {
            id: id.clone(),
            arn,
            service_network_identifier: service_network_identifier.to_string(),
            service_identifier: service_identifier.to_string(),
            service_network_id: sn_id,
            service_network_arn: sn_arn,
            service_network_name: sn_name,
            service_id: svc_id,
            service_arn: svc_arn,
            service_name: svc_name,
            status: "ACTIVE".to_string(),
            created_at: now,
            tags,
        };

        // Increment associated services count on the service network
        if let Some(sn) = self.service_networks.values_mut().find(|sn| {
            sn.id == assoc.service_network_id
                || sn.arn == assoc.service_network_arn
                || sn.name == service_network_identifier
        }) {
            sn.number_of_associated_services += 1;
        }

        self.sn_service_associations.insert(id.clone(), assoc);
        Ok(self.sn_service_associations.get(&id).unwrap())
    }

    pub fn get_service_network_service_association(
        &self,
        identifier: &str,
    ) -> Result<&ServiceNetworkServiceAssociation, VpcLatticeError> {
        self.resolve_sn_service_association(identifier)
            .ok_or_else(|| {
                VpcLatticeError::ServiceNetworkServiceAssociationNotFound(identifier.to_string())
            })
    }

    pub fn delete_service_network_service_association(
        &mut self,
        identifier: &str,
    ) -> Result<ServiceNetworkServiceAssociation, VpcLatticeError> {
        let id = self
            .resolve_sn_service_association(identifier)
            .map(|a| a.id.clone())
            .ok_or_else(|| {
                VpcLatticeError::ServiceNetworkServiceAssociationNotFound(identifier.to_string())
            })?;

        let mut assoc = self.sn_service_associations.remove(&id).unwrap();
        assoc.status = "DELETE_IN_PROGRESS".to_string();

        // Decrement count
        if let Some(sn) = self
            .service_networks
            .values_mut()
            .find(|sn| sn.id == assoc.service_network_id)
        {
            sn.number_of_associated_services = (sn.number_of_associated_services - 1).max(0);
        }

        Ok(assoc)
    }

    pub fn list_service_network_service_associations(
        &self,
        service_network_identifier: Option<&str>,
        service_identifier: Option<&str>,
    ) -> Vec<&ServiceNetworkServiceAssociation> {
        self.sn_service_associations
            .values()
            .filter(|a| {
                if let Some(sni) = service_network_identifier
                    && a.service_network_id != sni
                    && a.service_network_arn != sni
                    && a.service_network_name != sni
                    && a.service_network_identifier != sni
                {
                    return false;
                }
                if let Some(si) = service_identifier
                    && a.service_id != si
                    && a.service_arn != si
                    && a.service_name != si
                    && a.service_identifier != si
                {
                    return false;
                }
                true
            })
            .collect()
    }

    fn resolve_sn_service_association(
        &self,
        identifier: &str,
    ) -> Option<&ServiceNetworkServiceAssociation> {
        if let Some(a) = self.sn_service_associations.get(identifier) {
            return Some(a);
        }
        self.sn_service_associations
            .values()
            .find(|a| a.arn == identifier)
    }

    // ── ServiceNetworkVpcAssociation ────────────────────────────────

    pub fn create_service_network_vpc_association(
        &mut self,
        service_network_identifier: &str,
        vpc_identifier: &str,
        security_group_ids: Vec<String>,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&ServiceNetworkVpcAssociation, VpcLatticeError> {
        // Check for duplicate
        if self.sn_vpc_associations.values().any(|a| {
            (a.service_network_identifier == service_network_identifier
                || a.service_network_id == service_network_identifier
                || a.service_network_arn == service_network_identifier)
                && a.vpc_id == vpc_identifier
        }) {
            return Err(VpcLatticeError::AssociationAlreadyExists);
        }

        let id = format!("snva-{}", &uuid::Uuid::new_v4().to_string()[..17]);
        let arn =
            format!("arn:aws:vpc-lattice:{region}:{account_id}:servicenetworkvpcassociation/{id}");
        let now = Utc::now();

        let (sn_id, sn_arn, sn_name) = if let Some(sn) =
            self.resolve_service_network(service_network_identifier)
        {
            (sn.id.clone(), sn.arn.clone(), sn.name.clone())
        } else {
            (
                service_network_identifier.to_string(),
                format!(
                    "arn:aws:vpc-lattice:{region}:{account_id}:servicenetwork/{service_network_identifier}"
                ),
                service_network_identifier.to_string(),
            )
        };

        let assoc = ServiceNetworkVpcAssociation {
            id: id.clone(),
            arn,
            service_network_identifier: service_network_identifier.to_string(),
            vpc_identifier: vpc_identifier.to_string(),
            service_network_id: sn_id,
            service_network_arn: sn_arn,
            service_network_name: sn_name,
            vpc_id: vpc_identifier.to_string(),
            status: "ACTIVE".to_string(),
            security_group_ids,
            created_at: now,
            last_updated_at: now,
            tags,
        };

        // Increment VPC count on service network
        if let Some(sn) = self.service_networks.values_mut().find(|sn| {
            sn.id == assoc.service_network_id
                || sn.arn == assoc.service_network_arn
                || sn.name == service_network_identifier
        }) {
            sn.number_of_associated_v_p_cs += 1;
        }

        self.sn_vpc_associations.insert(id.clone(), assoc);
        Ok(self.sn_vpc_associations.get(&id).unwrap())
    }

    pub fn get_service_network_vpc_association(
        &self,
        identifier: &str,
    ) -> Result<&ServiceNetworkVpcAssociation, VpcLatticeError> {
        self.resolve_sn_vpc_association(identifier).ok_or_else(|| {
            VpcLatticeError::ServiceNetworkVpcAssociationNotFound(identifier.to_string())
        })
    }

    pub fn delete_service_network_vpc_association(
        &mut self,
        identifier: &str,
    ) -> Result<ServiceNetworkVpcAssociation, VpcLatticeError> {
        let id = self
            .resolve_sn_vpc_association(identifier)
            .map(|a| a.id.clone())
            .ok_or_else(|| {
                VpcLatticeError::ServiceNetworkVpcAssociationNotFound(identifier.to_string())
            })?;

        let mut assoc = self.sn_vpc_associations.remove(&id).unwrap();
        assoc.status = "DELETE_IN_PROGRESS".to_string();

        if let Some(sn) = self
            .service_networks
            .values_mut()
            .find(|sn| sn.id == assoc.service_network_id)
        {
            sn.number_of_associated_v_p_cs = (sn.number_of_associated_v_p_cs - 1).max(0);
        }

        Ok(assoc)
    }

    pub fn list_service_network_vpc_associations(
        &self,
        service_network_identifier: Option<&str>,
        vpc_identifier: Option<&str>,
    ) -> Vec<&ServiceNetworkVpcAssociation> {
        self.sn_vpc_associations
            .values()
            .filter(|a| {
                if let Some(sni) = service_network_identifier
                    && a.service_network_id != sni
                    && a.service_network_arn != sni
                    && a.service_network_name != sni
                    && a.service_network_identifier != sni
                {
                    return false;
                }
                if let Some(vi) = vpc_identifier
                    && a.vpc_id != vi
                    && a.vpc_identifier != vi
                {
                    return false;
                }
                true
            })
            .collect()
    }

    fn resolve_sn_vpc_association(
        &self,
        identifier: &str,
    ) -> Option<&ServiceNetworkVpcAssociation> {
        if let Some(a) = self.sn_vpc_associations.get(identifier) {
            return Some(a);
        }
        self.sn_vpc_associations
            .values()
            .find(|a| a.arn == identifier)
    }

    // ── TargetGroup ─────────────────────────────────────────────────

    pub fn create_target_group(
        &mut self,
        name: &str,
        target_group_type: &str,
        config: Option<TargetGroupConfig>,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&TargetGroup, VpcLatticeError> {
        if self.target_groups.values().any(|tg| tg.name == name) {
            return Err(VpcLatticeError::TargetGroupAlreadyExists(name.to_string()));
        }

        let id = format!("tg-{}", &uuid::Uuid::new_v4().to_string()[..17]);
        let arn = format!("arn:aws:vpc-lattice:{region}:{account_id}:targetgroup/{id}");
        let now = Utc::now();

        let tg = TargetGroup {
            id: id.clone(),
            arn,
            name: name.to_string(),
            target_group_type: target_group_type.to_string(),
            config,
            status: "ACTIVE".to_string(),
            created_at: now,
            last_updated_at: now,
            tags,
            targets: Vec::new(),
        };

        self.target_groups.insert(id.clone(), tg);
        Ok(self.target_groups.get(&id).unwrap())
    }

    pub fn get_target_group(&self, identifier: &str) -> Result<&TargetGroup, VpcLatticeError> {
        self.resolve_target_group(identifier)
            .ok_or_else(|| VpcLatticeError::TargetGroupNotFound(identifier.to_string()))
    }

    pub fn delete_target_group(
        &mut self,
        identifier: &str,
    ) -> Result<TargetGroup, VpcLatticeError> {
        let id = self
            .resolve_target_group(identifier)
            .map(|tg| tg.id.clone())
            .ok_or_else(|| VpcLatticeError::TargetGroupNotFound(identifier.to_string()))?;
        let mut tg = self.target_groups.remove(&id).unwrap();
        tg.status = "DELETE_IN_PROGRESS".to_string();
        Ok(tg)
    }

    pub fn list_target_groups(&self) -> Vec<&TargetGroup> {
        self.target_groups.values().collect()
    }

    pub fn register_targets(
        &mut self,
        target_group_identifier: &str,
        targets: Vec<(String, Option<i32>)>,
    ) -> Result<Vec<TargetEntry>, VpcLatticeError> {
        let id = self
            .resolve_target_group(target_group_identifier)
            .map(|tg| tg.id.clone())
            .ok_or_else(|| {
                VpcLatticeError::TargetGroupNotFound(target_group_identifier.to_string())
            })?;

        let tg = self.target_groups.get_mut(&id).unwrap();
        let mut registered = Vec::new();

        for (target_id, port) in targets {
            // Remove existing target with same id+port if present
            tg.targets
                .retain(|t| !(t.id == target_id && t.port == port));

            let entry = TargetEntry {
                id: target_id,
                port,
                status: "HEALTHY".to_string(),
            };
            tg.targets.push(entry.clone());
            registered.push(entry);
        }

        Ok(registered)
    }

    pub fn deregister_targets(
        &mut self,
        target_group_identifier: &str,
        targets: Vec<(String, Option<i32>)>,
    ) -> Result<Vec<TargetEntry>, VpcLatticeError> {
        let id = self
            .resolve_target_group(target_group_identifier)
            .map(|tg| tg.id.clone())
            .ok_or_else(|| {
                VpcLatticeError::TargetGroupNotFound(target_group_identifier.to_string())
            })?;

        let tg = self.target_groups.get_mut(&id).unwrap();
        let mut deregistered = Vec::new();

        for (target_id, port) in targets {
            tg.targets
                .retain(|t| !(t.id == target_id && t.port == port));
            deregistered.push(TargetEntry {
                id: target_id,
                port,
                status: "DRAINING".to_string(),
            });
        }

        Ok(deregistered)
    }

    pub fn list_targets(
        &self,
        target_group_identifier: &str,
    ) -> Result<Vec<&TargetEntry>, VpcLatticeError> {
        let tg = self
            .resolve_target_group(target_group_identifier)
            .ok_or_else(|| {
                VpcLatticeError::TargetGroupNotFound(target_group_identifier.to_string())
            })?;
        Ok(tg.targets.iter().collect())
    }

    fn resolve_target_group(&self, identifier: &str) -> Option<&TargetGroup> {
        if let Some(tg) = self.target_groups.get(identifier) {
            return Some(tg);
        }
        if let Some(tg) = self.target_groups.values().find(|tg| tg.arn == identifier) {
            return Some(tg);
        }
        self.target_groups.values().find(|tg| tg.name == identifier)
    }

    // ── Service ──────────────────────────────────────────────────────

    pub fn create_service(
        &mut self,
        name: &str,
        auth_type: Option<&str>,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&Service, VpcLatticeError> {
        if self.services.values().any(|s| s.name == name) {
            return Err(VpcLatticeError::ServiceAlreadyExists(name.to_string()));
        }

        let id = format!("svc-{}", &uuid::Uuid::new_v4().to_string()[..17]);
        let arn = format!("arn:aws:vpc-lattice:{region}:{account_id}:service/{id}");
        let now = Utc::now();

        let svc = Service {
            id: id.clone(),
            name: name.to_string(),
            arn,
            auth_type: auth_type.unwrap_or("NONE").to_string(),
            status: "ACTIVE".to_string(),
            created_at: now,
            last_updated_at: now,
            tags,
        };

        self.services.insert(id.clone(), svc);
        Ok(self.services.get(&id).unwrap())
    }

    pub fn get_service(&self, identifier: &str) -> Result<&Service, VpcLatticeError> {
        self.resolve_service(identifier)
            .ok_or_else(|| VpcLatticeError::ServiceNotFound(identifier.to_string()))
    }

    pub fn delete_service(&mut self, identifier: &str) -> Result<Service, VpcLatticeError> {
        let id = self
            .resolve_service(identifier)
            .map(|s| s.id.clone())
            .ok_or_else(|| VpcLatticeError::ServiceNotFound(identifier.to_string()))?;
        let mut svc = self.services.remove(&id).unwrap();
        svc.status = "DELETE_IN_PROGRESS".to_string();
        Ok(svc)
    }

    pub fn list_services(&self) -> Vec<&Service> {
        self.services.values().collect()
    }

    fn resolve_service(&self, identifier: &str) -> Option<&Service> {
        if let Some(s) = self.services.get(identifier) {
            return Some(s);
        }
        if let Some(s) = self.services.values().find(|s| s.arn == identifier) {
            return Some(s);
        }
        self.services.values().find(|s| s.name == identifier)
    }

    // ── AuthPolicy ───────────────────────────────────────────────────

    pub fn put_auth_policy(&mut self, resource_identifier: &str, policy: &str) -> &AuthPolicy {
        let now = Utc::now();
        let entry = self
            .auth_policies
            .entry(resource_identifier.to_string())
            .or_insert_with(|| AuthPolicy {
                policy: policy.to_string(),
                state: "Active".to_string(),
                created_at: now,
                last_updated_at: now,
            });
        entry.policy = policy.to_string();
        entry.last_updated_at = now;
        // Return the inserted/updated entry
        self.auth_policies.get(resource_identifier).unwrap()
    }

    pub fn get_auth_policy(
        &self,
        resource_identifier: &str,
    ) -> Result<&AuthPolicy, VpcLatticeError> {
        self.auth_policies
            .get(resource_identifier)
            .ok_or_else(|| VpcLatticeError::AuthPolicyNotFound(resource_identifier.to_string()))
    }

    pub fn delete_auth_policy(&mut self, resource_identifier: &str) -> Result<(), VpcLatticeError> {
        self.auth_policies.remove(resource_identifier);
        Ok(())
    }

    // ── ResourcePolicy ───────────────────────────────────────────────

    pub fn put_resource_policy(&mut self, resource_arn: &str, policy: &str) {
        self.resource_policies.insert(
            resource_arn.to_string(),
            ResourcePolicy {
                policy: policy.to_string(),
            },
        );
    }

    pub fn get_resource_policy(
        &self,
        resource_arn: &str,
    ) -> Result<&ResourcePolicy, VpcLatticeError> {
        self.resource_policies
            .get(resource_arn)
            .ok_or_else(|| VpcLatticeError::ResourcePolicyNotFound(resource_arn.to_string()))
    }

    pub fn delete_resource_policy(&mut self, resource_arn: &str) -> Result<(), VpcLatticeError> {
        self.resource_policies.remove(resource_arn);
        Ok(())
    }

    // ── Tags (generic - keyed by ARN) ────────────────────────────────

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), VpcLatticeError> {
        // FIX(terraform-e2e): listener/association ARNs were not handled, causing
        //   tag operations on those resource types to return TagResourceNotFound.
        if let Some(sn) = self
            .service_networks
            .values_mut()
            .find(|r| r.arn == resource_arn)
        {
            sn.tags.extend(tags);
            return Ok(());
        }
        if let Some(svc) = self.services.values_mut().find(|r| r.arn == resource_arn) {
            svc.tags.extend(tags);
            return Ok(());
        }
        if let Some(tg) = self
            .target_groups
            .values_mut()
            .find(|r| r.arn == resource_arn)
        {
            tg.tags.extend(tags);
            return Ok(());
        }
        if let Some(s) = self
            .access_log_subscriptions
            .values_mut()
            .find(|r| r.arn == resource_arn)
        {
            s.tags.extend(tags);
            return Ok(());
        }
        if let Some(l) = self.listeners.values_mut().find(|r| r.arn == resource_arn) {
            l.tags.extend(tags);
            return Ok(());
        }
        if let Some(a) = self
            .sn_service_associations
            .values_mut()
            .find(|r| r.arn == resource_arn)
        {
            a.tags.extend(tags);
            return Ok(());
        }
        if let Some(a) = self
            .sn_vpc_associations
            .values_mut()
            .find(|r| r.arn == resource_arn)
        {
            a.tags.extend(tags);
            return Ok(());
        }
        if self.rules.values().any(|r| r.arn == resource_arn) {
            return Ok(()); // rules do not carry tags; silently accept the tag call
        }
        Err(VpcLatticeError::TagResourceNotFound(
            resource_arn.to_string(),
        ))
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), VpcLatticeError> {
        if let Some(sn) = self
            .service_networks
            .values_mut()
            .find(|r| r.arn == resource_arn)
        {
            for k in tag_keys {
                sn.tags.remove(k);
            }
            return Ok(());
        }
        if let Some(svc) = self.services.values_mut().find(|r| r.arn == resource_arn) {
            for k in tag_keys {
                svc.tags.remove(k);
            }
            return Ok(());
        }
        if let Some(tg) = self
            .target_groups
            .values_mut()
            .find(|r| r.arn == resource_arn)
        {
            for k in tag_keys {
                tg.tags.remove(k);
            }
            return Ok(());
        }
        if let Some(s) = self
            .access_log_subscriptions
            .values_mut()
            .find(|r| r.arn == resource_arn)
        {
            for k in tag_keys {
                s.tags.remove(k);
            }
            return Ok(());
        }
        if let Some(l) = self.listeners.values_mut().find(|r| r.arn == resource_arn) {
            for k in tag_keys {
                l.tags.remove(k);
            }
            return Ok(());
        }
        if let Some(a) = self
            .sn_service_associations
            .values_mut()
            .find(|r| r.arn == resource_arn)
        {
            for k in tag_keys {
                a.tags.remove(k);
            }
            return Ok(());
        }
        if let Some(a) = self
            .sn_vpc_associations
            .values_mut()
            .find(|r| r.arn == resource_arn)
        {
            for k in tag_keys {
                a.tags.remove(k);
            }
            return Ok(());
        }
        if self.rules.values().any(|r| r.arn == resource_arn) {
            return Ok(()); // rules do not carry tags
        }
        Err(VpcLatticeError::TagResourceNotFound(
            resource_arn.to_string(),
        ))
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<HashMap<String, String>, VpcLatticeError> {
        // FIX(terraform-e2e): listener/association ARNs were not handled, causing
        //   ListTagsForResource to return 404 when provider refreshes those resources.
        if let Some(sn) = self
            .service_networks
            .values()
            .find(|r| r.arn == resource_arn)
        {
            return Ok(sn.tags.clone());
        }
        if let Some(svc) = self.services.values().find(|r| r.arn == resource_arn) {
            return Ok(svc.tags.clone());
        }
        if let Some(tg) = self.target_groups.values().find(|r| r.arn == resource_arn) {
            return Ok(tg.tags.clone());
        }
        if let Some(s) = self
            .access_log_subscriptions
            .values()
            .find(|r| r.arn == resource_arn)
        {
            return Ok(s.tags.clone());
        }
        if let Some(l) = self.listeners.values().find(|r| r.arn == resource_arn) {
            return Ok(l.tags.clone());
        }
        if let Some(a) = self
            .sn_service_associations
            .values()
            .find(|r| r.arn == resource_arn)
        {
            return Ok(a.tags.clone());
        }
        if let Some(a) = self
            .sn_vpc_associations
            .values()
            .find(|r| r.arn == resource_arn)
        {
            return Ok(a.tags.clone());
        }
        // FIX(terraform-e2e): rules do not carry tags in the mock model but the provider
        //   calls ListTagsForResource for rule ARNs after create. Return empty tags so the
        //   provider doesn't fail with 404 ResourceNotFoundException.
        if self.rules.values().any(|r| r.arn == resource_arn) {
            return Ok(HashMap::new());
        }
        Err(VpcLatticeError::TagResourceNotFound(
            resource_arn.to_string(),
        ))
    }

    // ── UpdateAccessLogSubscription ──────────────────────────────────

    pub fn update_access_log_subscription(
        &mut self,
        identifier: &str,
        destination_arn: &str,
    ) -> Result<&AccessLogSubscription, VpcLatticeError> {
        let id = self
            .resolve_access_log_subscription(identifier)
            .map(|s| s.id.clone())
            .ok_or_else(|| {
                VpcLatticeError::AccessLogSubscriptionNotFound(identifier.to_string())
            })?;

        let sub = self.access_log_subscriptions.get_mut(&id).unwrap();
        sub.destination_arn = destination_arn.to_string();
        sub.last_updated_at = Utc::now();
        Ok(self.access_log_subscriptions.get(&id).unwrap())
    }

    // ── UpdateService ────────────────────────────────────────────────

    pub fn update_service(
        &mut self,
        identifier: &str,
        auth_type: Option<&str>,
    ) -> Result<&Service, VpcLatticeError> {
        let id = self
            .resolve_service(identifier)
            .map(|s| s.id.clone())
            .ok_or_else(|| VpcLatticeError::ServiceNotFound(identifier.to_string()))?;

        let svc = self.services.get_mut(&id).unwrap();
        if let Some(at) = auth_type {
            svc.auth_type = at.to_string();
        }
        svc.last_updated_at = Utc::now();
        Ok(self.services.get(&id).unwrap())
    }

    // ── UpdateServiceNetwork ─────────────────────────────────────────

    pub fn update_service_network(
        &mut self,
        identifier: &str,
        auth_type: Option<&str>,
    ) -> Result<&ServiceNetwork, VpcLatticeError> {
        let id = self
            .resolve_service_network(identifier)
            .map(|s| s.id.clone())
            .ok_or_else(|| VpcLatticeError::ServiceNetworkNotFound(identifier.to_string()))?;

        let sn = self.service_networks.get_mut(&id).unwrap();
        if let Some(at) = auth_type {
            sn.auth_type = at.to_string();
        }
        sn.last_updated_at = Utc::now();
        Ok(self.service_networks.get(&id).unwrap())
    }

    // ── UpdateTargetGroup ────────────────────────────────────────────

    pub fn update_target_group(
        &mut self,
        identifier: &str,
        health_check: Option<()>,
    ) -> Result<&TargetGroup, VpcLatticeError> {
        let id = self
            .resolve_target_group(identifier)
            .map(|tg| tg.id.clone())
            .ok_or_else(|| VpcLatticeError::TargetGroupNotFound(identifier.to_string()))?;

        let tg = self.target_groups.get_mut(&id).unwrap();
        let _ = health_check; // health check config not stored in state
        tg.last_updated_at = Utc::now();
        Ok(self.target_groups.get(&id).unwrap())
    }

    // ── UpdateServiceNetworkVpcAssociation ───────────────────────────

    pub fn update_service_network_vpc_association(
        &mut self,
        identifier: &str,
        security_group_ids: Vec<String>,
    ) -> Result<&ServiceNetworkVpcAssociation, VpcLatticeError> {
        let id = self
            .resolve_sn_vpc_association(identifier)
            .map(|a| a.id.clone())
            .ok_or_else(|| {
                VpcLatticeError::ServiceNetworkVpcAssociationNotFound(identifier.to_string())
            })?;

        let assoc = self.sn_vpc_associations.get_mut(&id).unwrap();
        assoc.security_group_ids = security_group_ids;
        assoc.last_updated_at = Utc::now();
        Ok(self.sn_vpc_associations.get(&id).unwrap())
    }

    // ── Listener ─────────────────────────────────────────────────────

    pub fn create_listener(
        &mut self,
        service_identifier: &str,
        name: &str,
        protocol: &str,
        port: Option<i32>,
        default_action: ListenerDefaultAction,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&Listener, VpcLatticeError> {
        let svc = self
            .resolve_service(service_identifier)
            .ok_or_else(|| VpcLatticeError::ServiceNotFound(service_identifier.to_string()))?;
        let (svc_id, svc_arn) = (svc.id.clone(), svc.arn.clone());

        let id = format!("listener-{}", &uuid::Uuid::new_v4().to_string()[..17]);
        let arn =
            format!("arn:aws:vpc-lattice:{region}:{account_id}:service/{svc_id}/listener/{id}");
        let now = Utc::now();

        let listener = Listener {
            id: id.clone(),
            arn,
            name: name.to_string(),
            service_id: svc_id,
            service_arn: svc_arn,
            port,
            protocol: protocol.to_string(),
            default_action,
            created_at: now,
            last_updated_at: now,
            tags,
        };

        self.listeners.insert(id.clone(), listener);
        Ok(self.listeners.get(&id).unwrap())
    }

    pub fn get_listener(
        &self,
        service_identifier: &str,
        listener_identifier: &str,
    ) -> Result<&Listener, VpcLatticeError> {
        self.resolve_listener(service_identifier, listener_identifier)
            .ok_or_else(|| VpcLatticeError::ListenerNotFound(listener_identifier.to_string()))
    }

    pub fn delete_listener(
        &mut self,
        service_identifier: &str,
        listener_identifier: &str,
    ) -> Result<(), VpcLatticeError> {
        let id = self
            .resolve_listener(service_identifier, listener_identifier)
            .map(|l| l.id.clone())
            .ok_or_else(|| VpcLatticeError::ListenerNotFound(listener_identifier.to_string()))?;
        // Also remove all rules for this listener
        self.rules.retain(|_, r| r.listener_id != id);
        self.listeners.remove(&id);
        Ok(())
    }

    pub fn list_listeners(&self, service_identifier: &str) -> Vec<&Listener> {
        let svc_id = self
            .resolve_service(service_identifier)
            .map(|s| s.id.clone())
            .unwrap_or_else(|| service_identifier.to_string());
        self.listeners
            .values()
            .filter(|l| l.service_id == svc_id || l.service_arn == service_identifier)
            .collect()
    }

    pub fn update_listener(
        &mut self,
        service_identifier: &str,
        listener_identifier: &str,
        default_action: ListenerDefaultAction,
    ) -> Result<&Listener, VpcLatticeError> {
        let id = self
            .resolve_listener(service_identifier, listener_identifier)
            .map(|l| l.id.clone())
            .ok_or_else(|| VpcLatticeError::ListenerNotFound(listener_identifier.to_string()))?;

        let listener = self.listeners.get_mut(&id).unwrap();
        listener.default_action = default_action;
        listener.last_updated_at = Utc::now();
        Ok(self.listeners.get(&id).unwrap())
    }

    fn resolve_listener(
        &self,
        service_identifier: &str,
        listener_identifier: &str,
    ) -> Option<&Listener> {
        // FIX(terraform-e2e): Terraform AWS provider sets listener resource id as
        //   "{service_id}/{listener_id}" (composite). When used as listener_identifier
        //   in aws_vpclattice_listener_rule, the provider URL-encodes the "/" and we
        //   decode it back, so listener_identifier arrives as "svc-xxx/listener-yyy".
        //   Strip the service prefix if present so the plain listener ID can be looked up.
        let plain_listener_id = if listener_identifier.contains('/') {
            listener_identifier
                .split_once('/')
                .map(|x| x.1)
                .unwrap_or(listener_identifier)
        } else {
            listener_identifier
        };

        let svc_id = self
            .resolve_service(service_identifier)
            .map(|s| s.id.clone())
            .unwrap_or_else(|| service_identifier.to_string());

        if let Some(l) = self.listeners.get(plain_listener_id) {
            if l.service_id == svc_id || l.service_arn == service_identifier {
                return Some(l);
            }
        }
        self.listeners.values().find(|l| {
            (l.service_id == svc_id || l.service_arn == service_identifier)
                && (l.arn == plain_listener_id
                    || l.name == plain_listener_id
                    || l.arn == listener_identifier)
        })
    }

    // ── Rule ─────────────────────────────────────────────────────────

    pub fn create_rule(
        &mut self,
        service_identifier: &str,
        listener_identifier: &str,
        name: &str,
        priority: i32,
        action: RuleAction,
        rule_match: Option<RuleMatchData>,
        account_id: &str,
        region: &str,
    ) -> Result<&Rule, VpcLatticeError> {
        let listener = self
            .resolve_listener(service_identifier, listener_identifier)
            .ok_or_else(|| VpcLatticeError::ListenerNotFound(listener_identifier.to_string()))?;
        let (listener_id, svc_id) = (listener.id.clone(), listener.service_id.clone());

        let id = format!("rule-{}", &uuid::Uuid::new_v4().to_string()[..17]);
        let arn = format!(
            "arn:aws:vpc-lattice:{region}:{account_id}:service/{svc_id}/listener/{listener_id}/rule/{id}"
        );
        let now = Utc::now();

        let rule = Rule {
            id: id.clone(),
            arn,
            name: name.to_string(),
            listener_id,
            service_id: svc_id,
            priority,
            is_default: false,
            action,
            rule_match,
            created_at: now,
            last_updated_at: now,
        };

        self.rules.insert(id.clone(), rule);
        Ok(self.rules.get(&id).unwrap())
    }

    pub fn get_rule(
        &self,
        service_identifier: &str,
        listener_identifier: &str,
        rule_identifier: &str,
    ) -> Result<&Rule, VpcLatticeError> {
        self.resolve_rule(service_identifier, listener_identifier, rule_identifier)
            .ok_or_else(|| VpcLatticeError::RuleNotFound(rule_identifier.to_string()))
    }

    pub fn delete_rule(
        &mut self,
        service_identifier: &str,
        listener_identifier: &str,
        rule_identifier: &str,
    ) -> Result<(), VpcLatticeError> {
        let id = self
            .resolve_rule(service_identifier, listener_identifier, rule_identifier)
            .map(|r| r.id.clone())
            .ok_or_else(|| VpcLatticeError::RuleNotFound(rule_identifier.to_string()))?;
        self.rules.remove(&id);
        Ok(())
    }

    pub fn list_rules(
        &self,
        service_identifier: &str,
        listener_identifier: &str,
    ) -> Result<Vec<&Rule>, VpcLatticeError> {
        let listener = self
            .resolve_listener(service_identifier, listener_identifier)
            .ok_or_else(|| VpcLatticeError::ListenerNotFound(listener_identifier.to_string()))?;
        let lid = listener.id.clone();
        Ok(self
            .rules
            .values()
            .filter(|r| r.listener_id == lid)
            .collect())
    }

    pub fn update_rule(
        &mut self,
        service_identifier: &str,
        listener_identifier: &str,
        rule_identifier: &str,
        action: Option<RuleAction>,
        rule_match: Option<RuleMatchData>,
        priority: Option<i32>,
    ) -> Result<&Rule, VpcLatticeError> {
        let id = self
            .resolve_rule(service_identifier, listener_identifier, rule_identifier)
            .map(|r| r.id.clone())
            .ok_or_else(|| VpcLatticeError::RuleNotFound(rule_identifier.to_string()))?;

        let rule = self.rules.get_mut(&id).unwrap();
        if let Some(a) = action {
            rule.action = a;
        }
        if let Some(m) = rule_match {
            rule.rule_match = Some(m);
        }
        if let Some(p) = priority {
            rule.priority = p;
        }
        rule.last_updated_at = Utc::now();
        Ok(self.rules.get(&id).unwrap())
    }

    fn resolve_rule(
        &self,
        service_identifier: &str,
        listener_identifier: &str,
        rule_identifier: &str,
    ) -> Option<&Rule> {
        let listener = self.resolve_listener(service_identifier, listener_identifier)?;
        let lid = &listener.id;

        if let Some(r) = self.rules.get(rule_identifier) {
            if &r.listener_id == lid {
                return Some(r);
            }
        }
        self.rules.values().find(|r| {
            &r.listener_id == lid && (r.arn == rule_identifier || r.name == rule_identifier)
        })
    }

    // ── ResourceConfiguration ────────────────────────────────────────

    pub fn create_resource_configuration(
        &mut self,
        name: &str,
        rc_type: &str,
        resource_gateway_id: Option<&str>,
        port_ranges: Vec<String>,
        protocol: Option<&str>,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&ResourceConfiguration, VpcLatticeError> {
        let id = format!("rcfg-{}", &uuid::Uuid::new_v4().to_string()[..17]);
        let arn = format!("arn:aws:vpc-lattice:{region}:{account_id}:resourceconfiguration/{id}");
        let now = Utc::now();

        let rc = ResourceConfiguration {
            id: id.clone(),
            arn,
            name: name.to_string(),
            resource_configuration_type: rc_type.to_string(),
            status: "ACTIVE".to_string(),
            resource_gateway_id: resource_gateway_id.map(|s| s.to_string()),
            port_ranges,
            protocol: protocol.map(|s| s.to_string()),
            created_at: now,
            last_updated_at: now,
            tags,
        };

        self.resource_configurations.insert(id.clone(), rc);
        Ok(self.resource_configurations.get(&id).unwrap())
    }

    pub fn get_resource_configuration(
        &self,
        identifier: &str,
    ) -> Result<&ResourceConfiguration, VpcLatticeError> {
        self.resolve_resource_configuration(identifier)
            .ok_or_else(|| VpcLatticeError::ResourceConfigurationNotFound(identifier.to_string()))
    }

    pub fn delete_resource_configuration(
        &mut self,
        identifier: &str,
    ) -> Result<(), VpcLatticeError> {
        let id = self
            .resolve_resource_configuration(identifier)
            .map(|r| r.id.clone())
            .ok_or_else(|| {
                VpcLatticeError::ResourceConfigurationNotFound(identifier.to_string())
            })?;
        self.resource_configurations.remove(&id);
        Ok(())
    }

    pub fn list_resource_configurations(&self) -> Vec<&ResourceConfiguration> {
        self.resource_configurations.values().collect()
    }

    pub fn update_resource_configuration(
        &mut self,
        identifier: &str,
        port_ranges: Option<Vec<String>>,
        protocol: Option<&str>,
    ) -> Result<&ResourceConfiguration, VpcLatticeError> {
        let id = self
            .resolve_resource_configuration(identifier)
            .map(|r| r.id.clone())
            .ok_or_else(|| {
                VpcLatticeError::ResourceConfigurationNotFound(identifier.to_string())
            })?;
        let rc = self.resource_configurations.get_mut(&id).unwrap();
        if let Some(pr) = port_ranges {
            rc.port_ranges = pr;
        }
        if let Some(p) = protocol {
            rc.protocol = Some(p.to_string());
        }
        rc.last_updated_at = Utc::now();
        Ok(self.resource_configurations.get(&id).unwrap())
    }

    fn resolve_resource_configuration(&self, identifier: &str) -> Option<&ResourceConfiguration> {
        if let Some(r) = self.resource_configurations.get(identifier) {
            return Some(r);
        }
        if let Some(r) = self
            .resource_configurations
            .values()
            .find(|r| r.arn == identifier)
        {
            return Some(r);
        }
        self.resource_configurations
            .values()
            .find(|r| r.name == identifier)
    }

    // ── ResourceGateway ──────────────────────────────────────────────

    pub fn create_resource_gateway(
        &mut self,
        name: &str,
        vpc_id: Option<&str>,
        subnet_ids: Vec<String>,
        security_group_ids: Vec<String>,
        ip_address_type: Option<&str>,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&ResourceGateway, VpcLatticeError> {
        let id = format!("rgw-{}", &uuid::Uuid::new_v4().to_string()[..17]);
        let arn = format!("arn:aws:vpc-lattice:{region}:{account_id}:resourcegateway/{id}");
        let now = Utc::now();

        let gw = ResourceGateway {
            id: id.clone(),
            arn,
            name: name.to_string(),
            status: "ACTIVE".to_string(),
            vpc_id: vpc_id.map(|s| s.to_string()),
            subnet_ids,
            security_group_ids,
            ip_address_type: ip_address_type.map(|s| s.to_string()),
            created_at: now,
            last_updated_at: now,
            tags,
        };

        self.resource_gateways.insert(id.clone(), gw);
        Ok(self.resource_gateways.get(&id).unwrap())
    }

    pub fn get_resource_gateway(
        &self,
        identifier: &str,
    ) -> Result<&ResourceGateway, VpcLatticeError> {
        self.resolve_resource_gateway(identifier)
            .ok_or_else(|| VpcLatticeError::ResourceGatewayNotFound(identifier.to_string()))
    }

    pub fn delete_resource_gateway(
        &mut self,
        identifier: &str,
    ) -> Result<ResourceGateway, VpcLatticeError> {
        let id = self
            .resolve_resource_gateway(identifier)
            .map(|r| r.id.clone())
            .ok_or_else(|| VpcLatticeError::ResourceGatewayNotFound(identifier.to_string()))?;
        let mut gw = self.resource_gateways.remove(&id).unwrap();
        gw.status = "DELETING".to_string();
        Ok(gw)
    }

    pub fn list_resource_gateways(&self) -> Vec<&ResourceGateway> {
        self.resource_gateways.values().collect()
    }

    pub fn update_resource_gateway(
        &mut self,
        identifier: &str,
        security_group_ids: Option<Vec<String>>,
    ) -> Result<&ResourceGateway, VpcLatticeError> {
        let id = self
            .resolve_resource_gateway(identifier)
            .map(|r| r.id.clone())
            .ok_or_else(|| VpcLatticeError::ResourceGatewayNotFound(identifier.to_string()))?;
        let gw = self.resource_gateways.get_mut(&id).unwrap();
        if let Some(sgs) = security_group_ids {
            gw.security_group_ids = sgs;
        }
        gw.last_updated_at = Utc::now();
        Ok(self.resource_gateways.get(&id).unwrap())
    }

    fn resolve_resource_gateway(&self, identifier: &str) -> Option<&ResourceGateway> {
        if let Some(r) = self.resource_gateways.get(identifier) {
            return Some(r);
        }
        if let Some(r) = self
            .resource_gateways
            .values()
            .find(|r| r.arn == identifier)
        {
            return Some(r);
        }
        self.resource_gateways
            .values()
            .find(|r| r.name == identifier)
    }

    // ── ServiceNetworkResourceAssociation ────────────────────────────

    pub fn create_service_network_resource_association(
        &mut self,
        service_network_identifier: &str,
        resource_configuration_identifier: &str,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&ServiceNetworkResourceAssociation, VpcLatticeError> {
        let id = format!("snra-{}", &uuid::Uuid::new_v4().to_string()[..17]);
        let arn = format!(
            "arn:aws:vpc-lattice:{region}:{account_id}:servicenetworkresourceassociation/{id}"
        );
        let now = Utc::now();

        let (sn_id, sn_arn, sn_name) = if let Some(sn) =
            self.resolve_service_network(service_network_identifier)
        {
            (sn.id.clone(), sn.arn.clone(), sn.name.clone())
        } else {
            (
                service_network_identifier.to_string(),
                format!(
                    "arn:aws:vpc-lattice:{region}:{account_id}:servicenetwork/{service_network_identifier}"
                ),
                service_network_identifier.to_string(),
            )
        };

        let (rc_id, rc_arn, rc_name) = if let Some(rc) =
            self.resolve_resource_configuration(resource_configuration_identifier)
        {
            (rc.id.clone(), rc.arn.clone(), rc.name.clone())
        } else {
            (
                resource_configuration_identifier.to_string(),
                format!(
                    "arn:aws:vpc-lattice:{region}:{account_id}:resourceconfiguration/{resource_configuration_identifier}"
                ),
                resource_configuration_identifier.to_string(),
            )
        };

        let assoc = ServiceNetworkResourceAssociation {
            id: id.clone(),
            arn,
            service_network_identifier: service_network_identifier.to_string(),
            resource_configuration_identifier: resource_configuration_identifier.to_string(),
            service_network_id: sn_id,
            service_network_arn: sn_arn,
            service_network_name: sn_name,
            resource_configuration_id: rc_id,
            resource_configuration_arn: rc_arn,
            resource_configuration_name: rc_name,
            status: "ACTIVE".to_string(),
            created_at: now,
            tags,
        };

        self.sn_resource_associations.insert(id.clone(), assoc);
        Ok(self.sn_resource_associations.get(&id).unwrap())
    }

    pub fn get_service_network_resource_association(
        &self,
        identifier: &str,
    ) -> Result<&ServiceNetworkResourceAssociation, VpcLatticeError> {
        self.resolve_sn_resource_association(identifier)
            .ok_or_else(|| {
                VpcLatticeError::ServiceNetworkResourceAssociationNotFound(identifier.to_string())
            })
    }

    pub fn delete_service_network_resource_association(
        &mut self,
        identifier: &str,
    ) -> Result<ServiceNetworkResourceAssociation, VpcLatticeError> {
        let id = self
            .resolve_sn_resource_association(identifier)
            .map(|a| a.id.clone())
            .ok_or_else(|| {
                VpcLatticeError::ServiceNetworkResourceAssociationNotFound(identifier.to_string())
            })?;
        let mut assoc = self.sn_resource_associations.remove(&id).unwrap();
        assoc.status = "DELETE_IN_PROGRESS".to_string();
        Ok(assoc)
    }

    pub fn list_service_network_resource_associations(
        &self,
        service_network_identifier: Option<&str>,
        resource_configuration_identifier: Option<&str>,
    ) -> Vec<&ServiceNetworkResourceAssociation> {
        self.sn_resource_associations
            .values()
            .filter(|a| {
                if let Some(sni) = service_network_identifier
                    && a.service_network_id != sni
                    && a.service_network_arn != sni
                    && a.service_network_name != sni
                    && a.service_network_identifier != sni
                {
                    return false;
                }
                if let Some(rci) = resource_configuration_identifier
                    && a.resource_configuration_id != rci
                    && a.resource_configuration_arn != rci
                    && a.resource_configuration_name != rci
                    && a.resource_configuration_identifier != rci
                {
                    return false;
                }
                true
            })
            .collect()
    }

    fn resolve_sn_resource_association(
        &self,
        identifier: &str,
    ) -> Option<&ServiceNetworkResourceAssociation> {
        if let Some(a) = self.sn_resource_associations.get(identifier) {
            return Some(a);
        }
        self.sn_resource_associations
            .values()
            .find(|a| a.arn == identifier)
    }

    // ── DomainVerification ───────────────────────────────────────────

    pub fn create_domain_verification(
        &mut self,
        domain_name: &str,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&DomainVerification, VpcLatticeError> {
        let id = format!("dv-{}", &uuid::Uuid::new_v4().to_string()[..17]);
        let arn = format!("arn:aws:vpc-lattice:{region}:{account_id}:domainverification/{id}");
        let now = Utc::now();

        let dv = DomainVerification {
            id: id.clone(),
            arn,
            domain_name: domain_name.to_string(),
            status: "PENDING".to_string(),
            created_at: now,
            tags,
        };

        self.domain_verifications.insert(id.clone(), dv);
        Ok(self.domain_verifications.get(&id).unwrap())
    }

    pub fn get_domain_verification(
        &self,
        identifier: &str,
    ) -> Result<&DomainVerification, VpcLatticeError> {
        self.resolve_domain_verification(identifier)
            .ok_or_else(|| VpcLatticeError::DomainVerificationNotFound(identifier.to_string()))
    }

    pub fn delete_domain_verification(&mut self, identifier: &str) -> Result<(), VpcLatticeError> {
        let id = self
            .resolve_domain_verification(identifier)
            .map(|d| d.id.clone())
            .ok_or_else(|| VpcLatticeError::DomainVerificationNotFound(identifier.to_string()))?;
        self.domain_verifications.remove(&id);
        Ok(())
    }

    pub fn list_domain_verifications(&self) -> Vec<&DomainVerification> {
        self.domain_verifications.values().collect()
    }

    fn resolve_domain_verification(&self, identifier: &str) -> Option<&DomainVerification> {
        if let Some(d) = self.domain_verifications.get(identifier) {
            return Some(d);
        }
        if let Some(d) = self
            .domain_verifications
            .values()
            .find(|d| d.arn == identifier)
        {
            return Some(d);
        }
        self.domain_verifications
            .values()
            .find(|d| d.domain_name == identifier)
    }
}
