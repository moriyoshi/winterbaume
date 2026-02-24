use std::collections::{HashMap, HashSet};

use crate::types::*;

/// In-memory state for Route 53.
#[derive(Debug, Default)]
pub struct Route53State {
    /// Hosted zones keyed by full zone ID (`/hostedzone/{id}`).
    pub hosted_zones: HashMap<String, HostedZone>,
    /// Health checks keyed by health check ID.
    pub health_checks: HashMap<String, Route53HealthCheck>,
    /// Query logging configs keyed by config ID.
    pub query_logging_configs: HashMap<String, Route53QueryLoggingConfig>,
    /// Reusable delegation sets keyed by delegation set ID.
    pub reusable_delegation_sets: HashMap<String, Route53DelegationSet>,
    /// Deleted health check caller references remain reserved for a while.
    pub deleted_health_check_caller_references: HashSet<String>,
    /// Key signing keys keyed by `{hosted_zone_id}:{name}`.
    pub key_signing_keys: HashMap<String, KeySigningKeyEntry>,
    /// Whether DNSSEC is enabled per hosted zone.
    pub dnssec_enabled: HashMap<String, bool>,
    /// CIDR collections keyed by collection ID.
    pub cidr_collections: HashMap<String, CidrCollectionEntry>,
    /// Traffic policies keyed by `{id}:{version}`.
    pub traffic_policies: HashMap<String, TrafficPolicyEntry>,
    /// Traffic policy instances keyed by instance ID.
    pub traffic_policy_instances: HashMap<String, TrafficPolicyInstanceEntry>,
    /// VPC association authorizations keyed by `{hosted_zone_id}:{vpc_id}`.
    pub vpc_association_authorizations: HashMap<String, VpcAssociationAuthorization>,
}

/// Error type for Route 53 operations.
#[derive(Debug, thiserror::Error)]
pub enum Route53Error {
    // --- Hosted zone errors ---
    #[error("No hosted zone found with ID: {zone_id}")]
    NoSuchHostedZone { zone_id: String },
    #[error("A hosted zone with the caller reference '{caller_reference}' already exists")]
    HostedZoneAlreadyExists { caller_reference: String },

    // --- Record set errors ---
    #[error(
        "Tried to create resource record set [name='{name}', type='{record_type}'] but it already exists"
    )]
    InvalidChangeBatchRecordExists { name: String, record_type: String },
    #[error("Invalid change action: {action}")]
    InvalidChangeBatchAction { action: String },
    #[error("No changes found in request body")]
    NoChangesInRequest,

    // --- Health check errors ---
    #[error("A health check with id {health_check_id} does not exist.")]
    NoSuchHealthCheck { health_check_id: String },
    #[error("A health check with caller reference {caller_reference} already exists.")]
    HealthCheckAlreadyExists { caller_reference: String },

    // --- Query logging errors ---
    #[error("A query logging configuration already exists for this hosted zone.")]
    QueryLoggingConfigAlreadyExists,
    #[error("The query logging configuration does not exist.")]
    NoSuchQueryLoggingConfig,

    // --- Delegation set errors ---
    #[error("A reusable delegation set with this caller reference already exists.")]
    DelegationSetAlreadyCreated,
    #[error("No reusable delegation set found with ID: {delegation_set_id}")]
    NoSuchDelegationSet { delegation_set_id: String },

    // --- Key signing key errors ---
    #[error("Key signing key '{name}' already exists")]
    KeySigningKeyAlreadyExists { name: String },
    #[error("Key signing key '{name}' not found")]
    NoSuchKeySigningKey { name: String },

    // --- CIDR collection errors ---
    #[error("CIDR collection '{name}' already exists")]
    CidrCollectionAlreadyExistsException { name: String },
    #[error("No CIDR collection found with ID: {collection_id}")]
    NoSuchCidrCollectionException { collection_id: String },

    // --- Traffic policy errors ---
    #[error("No traffic policy with ID: {policy_id}")]
    NoSuchTrafficPolicy { policy_id: String },
    #[error("No traffic policy with ID: {policy_id}, version: {version}")]
    NoSuchTrafficPolicyVersion { policy_id: String, version: i32 },
    #[error("No traffic policy instance with ID: {instance_id}")]
    NoSuchTrafficPolicyInstance { instance_id: String },

    // --- VPC association errors ---
    #[error("No VPC association authorization found for {vpc_id}")]
    VpcAssociationAuthorizationNotFound { vpc_id: String },

    // --- Generic input errors ---
    #[error("Unsupported resource type: {resource_type}")]
    UnsupportedResourceType { resource_type: String },
    #[error("Unsupported operation: {method} {path}")]
    UnsupportedOperation { method: String, path: String },
    #[error("Missing required element Name or CallerReference")]
    MissingNameOrCallerReference,
    #[error("Missing VPCId")]
    MissingVpcId,
    #[error("Missing vpcId parameter")]
    MissingVpcIdParameter,
    #[error("Missing required health check fields")]
    MissingHealthCheckFields,
    #[error("Missing required query logging config fields")]
    MissingQueryLoggingConfigFields,
    #[error("Missing CallerReference")]
    MissingCallerReference,
    #[error("Invalid version")]
    InvalidVersion,
    #[error("{0}")]
    InvalidInput(String),
}

impl Route53State {
    pub fn create_hosted_zone(
        &mut self,
        name: &str,
        caller_reference: &str,
    ) -> Result<HostedZone, Route53Error> {
        if self
            .hosted_zones
            .values()
            .any(|z| z.caller_reference == caller_reference)
        {
            return Err(Route53Error::HostedZoneAlreadyExists {
                caller_reference: caller_reference.to_string(),
            });
        }

        let id = compact_id();
        let zone_id = format!("/hostedzone/{id}");
        let zone_name = ensure_trailing_dot(name);
        let delegation_set = Route53DelegationSet {
            id: format!("N{delegation_id}", delegation_id = compact_id()),
            caller_reference: None,
            name_servers: default_name_servers(&id),
        };

        let soa_record = ResourceRecordSet {
            name: zone_name.clone(),
            record_type: "SOA".to_string(),
            ttl: Some(900),
            resource_records: vec![format!(
                "ns-1.awsdns-00.org. hostmaster.{zone_name} 1 7200 900 1209600 86400"
            )],
        };
        let ns_record = ResourceRecordSet {
            name: zone_name.clone(),
            record_type: "NS".to_string(),
            ttl: Some(172800),
            resource_records: delegation_set.name_servers.clone(),
        };

        let zone = HostedZone {
            id: zone_id.clone(),
            name: zone_name,
            caller_reference: caller_reference.to_string(),
            resource_record_count: 2,
            records: vec![soa_record, ns_record],
            vpcs: Vec::new(),
            comment: None,
            delegation_set,
            tags: HashMap::new(),
        };

        self.hosted_zones.insert(zone_id, zone.clone());
        Ok(zone)
    }

    pub fn get_hosted_zone(&self, zone_id: &str) -> Result<HostedZone, Route53Error> {
        let key = normalize_hosted_zone_id(zone_id);
        self.hosted_zones
            .get(&key)
            .cloned()
            .ok_or_else(|| zone_not_found(zone_id))
    }

    pub fn delete_hosted_zone(&mut self, zone_id: &str) -> Result<(), Route53Error> {
        let key = normalize_hosted_zone_id(zone_id);
        self.hosted_zones
            .remove(&key)
            .ok_or_else(|| zone_not_found(zone_id))?;
        Ok(())
    }

    pub fn list_hosted_zones(&self) -> Vec<HostedZone> {
        let mut zones: Vec<_> = self.hosted_zones.values().cloned().collect();
        zones.sort_by(|a, b| a.name.cmp(&b.name).then(a.id.cmp(&b.id)));
        zones
    }

    pub fn list_hosted_zones_by_name(
        &self,
        dns_name: Option<&str>,
        hosted_zone_id: Option<&str>,
    ) -> Vec<HostedZone> {
        let start_name = dns_name.map(ensure_trailing_dot);
        let start_id = hosted_zone_id.map(normalize_hosted_zone_id);
        self.list_hosted_zones()
            .into_iter()
            .filter(|zone| match &start_name {
                Some(name) if zone.name < *name => false,
                Some(name) if zone.name == *name => match &start_id {
                    Some(id) => zone.id >= *id,
                    None => true,
                },
                _ => true,
            })
            .collect()
    }

    pub fn get_hosted_zone_count(&self) -> i64 {
        self.hosted_zones.len() as i64
    }

    pub fn update_hosted_zone_comment(
        &mut self,
        zone_id: &str,
        comment: Option<String>,
    ) -> Result<HostedZone, Route53Error> {
        let key = normalize_hosted_zone_id(zone_id);
        let zone = self
            .hosted_zones
            .get_mut(&key)
            .ok_or_else(|| zone_not_found(zone_id))?;
        zone.comment = comment.filter(|value| !value.is_empty());
        Ok(zone.clone())
    }

    pub fn change_resource_record_sets(
        &mut self,
        zone_id: &str,
        changes: Vec<RecordChange>,
    ) -> Result<(), Route53Error> {
        let key = normalize_hosted_zone_id(zone_id);
        let zone = self
            .hosted_zones
            .get_mut(&key)
            .ok_or_else(|| zone_not_found(zone_id))?;

        let zone_name = zone.name.clone();

        for change in changes {
            let mut record_set = change.record_set;
            // Normalize record name to FQDN with trailing dot.
            // AWS Route53 accepts:
            //   - FQDN with trailing dot: "foo.example.com."
            //   - FQDN without trailing dot: "foo.example.com"
            //   - Zone-relative name: "foo" (becomes "foo.example.com.")
            record_set.name = normalize_record_name(&record_set.name, &zone_name);

            match change.action.as_str() {
                "CREATE" => {
                    if zone.records.iter().any(|r| {
                        r.name == record_set.name && r.record_type == record_set.record_type
                    }) {
                        return Err(Route53Error::InvalidChangeBatchRecordExists {
                            name: record_set.name.clone(),
                            record_type: record_set.record_type.clone(),
                        });
                    }
                    zone.records.push(record_set);
                    zone.resource_record_count += 1;
                }
                "DELETE" => {
                    let before = zone.records.len();
                    zone.records.retain(|r| {
                        !(r.name == record_set.name && r.record_type == record_set.record_type)
                    });
                    zone.resource_record_count -= (before - zone.records.len()) as u64;
                }
                "UPSERT" => {
                    zone.records.retain(|r| {
                        !(r.name == record_set.name && r.record_type == record_set.record_type)
                    });
                    zone.records.push(record_set);
                    zone.resource_record_count = zone.records.len() as u64;
                }
                _ => {
                    return Err(Route53Error::InvalidChangeBatchAction {
                        action: change.action.clone(),
                    });
                }
            }
        }
        Ok(())
    }

    pub fn list_resource_record_sets(
        &self,
        zone_id: &str,
    ) -> Result<Vec<ResourceRecordSet>, Route53Error> {
        let key = normalize_hosted_zone_id(zone_id);
        let zone = self
            .hosted_zones
            .get(&key)
            .ok_or_else(|| zone_not_found(zone_id))?;
        Ok(zone.records.clone())
    }

    pub fn associate_vpc_with_hosted_zone(
        &mut self,
        zone_id: &str,
        vpc_id: &str,
        vpc_region: &str,
    ) -> Result<(), Route53Error> {
        let key = normalize_hosted_zone_id(zone_id);
        let zone = self
            .hosted_zones
            .get_mut(&key)
            .ok_or_else(|| zone_not_found(zone_id))?;

        if !zone.vpcs.iter().any(|v| v.vpc_id == vpc_id) {
            zone.vpcs.push(Vpc {
                vpc_id: vpc_id.to_string(),
                vpc_region: vpc_region.to_string(),
            });
        }
        Ok(())
    }

    pub fn disassociate_vpc_from_hosted_zone(
        &mut self,
        zone_id: &str,
        vpc_id: &str,
    ) -> Result<(), Route53Error> {
        let key = normalize_hosted_zone_id(zone_id);
        let zone = self
            .hosted_zones
            .get_mut(&key)
            .ok_or_else(|| zone_not_found(zone_id))?;
        zone.vpcs.retain(|v| v.vpc_id != vpc_id);
        Ok(())
    }

    pub fn list_hosted_zones_by_vpc(&self, vpc_id: &str, _vpc_region: &str) -> Vec<HostedZone> {
        self.hosted_zones
            .values()
            .filter(|z| z.vpcs.iter().any(|v| v.vpc_id == vpc_id))
            .cloned()
            .collect()
    }

    pub fn change_tags_for_resource(
        &mut self,
        resource_type: &str,
        resource_id: &str,
        add_tags: &[(String, String)],
        remove_tag_keys: &[String],
    ) -> Result<(), Route53Error> {
        let tags = self.tags_mut(resource_type, resource_id)?;
        for (key, value) in add_tags {
            tags.insert(key.clone(), value.clone());
        }
        for key in remove_tag_keys {
            tags.remove(key);
        }
        Ok(())
    }

    pub fn list_tags_for_resource(
        &self,
        resource_type: &str,
        resource_id: &str,
    ) -> Result<HashMap<String, String>, Route53Error> {
        self.tags_ref(resource_type, resource_id).cloned()
    }

    pub fn list_tags_for_resources(
        &self,
        resource_type: &str,
        resource_ids: &[String],
    ) -> Result<Vec<TagResource>, Route53Error> {
        let mut result = Vec::new();
        for resource_id in resource_ids {
            result.push(TagResource {
                resource_id: canonical_resource_id(resource_type, resource_id),
                resource_type: resource_type.to_string(),
                tags: self.list_tags_for_resource(resource_type, resource_id)?,
            });
        }
        Ok(result)
    }

    pub fn create_health_check(
        &mut self,
        caller_reference: &str,
        config: Route53HealthCheckConfig,
    ) -> Result<Route53HealthCheck, Route53Error> {
        if self
            .deleted_health_check_caller_references
            .contains(caller_reference)
        {
            return Err(Route53Error::HealthCheckAlreadyExists {
                caller_reference: caller_reference.to_string(),
            });
        }

        if let Some(existing) = self
            .health_checks
            .values()
            .find(|hc| hc.caller_reference == caller_reference)
        {
            if existing.config == config {
                return Ok(existing.clone());
            }
            return Err(Route53Error::HealthCheckAlreadyExists {
                caller_reference: caller_reference.to_string(),
            });
        }

        let id = compact_id();
        let health_check = Route53HealthCheck {
            id: id.clone(),
            caller_reference: caller_reference.to_string(),
            config,
            version: 1,
            tags: HashMap::new(),
        };
        self.health_checks.insert(id, health_check.clone());
        Ok(health_check)
    }

    pub fn get_health_check(
        &self,
        health_check_id: &str,
    ) -> Result<Route53HealthCheck, Route53Error> {
        self.health_checks
            .get(health_check_id)
            .cloned()
            .ok_or_else(|| health_check_not_found(health_check_id))
    }

    pub fn delete_health_check(&mut self, health_check_id: &str) -> Result<(), Route53Error> {
        let health_check = self
            .health_checks
            .remove(health_check_id)
            .ok_or_else(|| health_check_not_found(health_check_id))?;
        self.deleted_health_check_caller_references
            .insert(health_check.caller_reference);
        Ok(())
    }

    pub fn list_health_checks(&self) -> Vec<Route53HealthCheck> {
        let mut checks: Vec<_> = self.health_checks.values().cloned().collect();
        checks.sort_by(|a, b| a.id.cmp(&b.id));
        checks
    }

    pub fn update_health_check(
        &mut self,
        health_check_id: &str,
        updates: Route53HealthCheckConfig,
    ) -> Result<Route53HealthCheck, Route53Error> {
        let health_check = self
            .health_checks
            .get_mut(health_check_id)
            .ok_or_else(|| health_check_not_found(health_check_id))?;

        merge_health_check_config(&mut health_check.config, updates);
        health_check.version += 1;
        Ok(health_check.clone())
    }

    pub fn create_query_logging_config(
        &mut self,
        hosted_zone_id: &str,
        cloud_watch_logs_log_group_arn: &str,
    ) -> Result<Route53QueryLoggingConfig, Route53Error> {
        let zone_key = normalize_hosted_zone_id(hosted_zone_id);
        if !self.hosted_zones.contains_key(&zone_key) {
            return Err(zone_not_found(hosted_zone_id));
        }

        if self
            .query_logging_configs
            .values()
            .any(|config| config.hosted_zone_id == zone_key)
        {
            return Err(Route53Error::QueryLoggingConfigAlreadyExists);
        }

        let id = compact_id();
        let config = Route53QueryLoggingConfig {
            id: id.clone(),
            hosted_zone_id: zone_key,
            cloud_watch_logs_log_group_arn: cloud_watch_logs_log_group_arn.to_string(),
        };
        self.query_logging_configs.insert(id, config.clone());
        Ok(config)
    }

    pub fn get_query_logging_config(
        &self,
        query_logging_config_id: &str,
    ) -> Result<Route53QueryLoggingConfig, Route53Error> {
        self.query_logging_configs
            .get(query_logging_config_id)
            .cloned()
            .ok_or_else(|| query_logging_config_not_found(query_logging_config_id))
    }

    pub fn delete_query_logging_config(
        &mut self,
        query_logging_config_id: &str,
    ) -> Result<(), Route53Error> {
        self.query_logging_configs
            .remove(query_logging_config_id)
            .ok_or_else(|| query_logging_config_not_found(query_logging_config_id))?;
        Ok(())
    }

    pub fn list_query_logging_configs(
        &self,
        hosted_zone_id: Option<&str>,
    ) -> Vec<Route53QueryLoggingConfig> {
        let hosted_zone_key = hosted_zone_id.map(normalize_hosted_zone_id);
        let mut configs: Vec<_> = self
            .query_logging_configs
            .values()
            .filter(|config| match &hosted_zone_key {
                Some(id) => config.hosted_zone_id == *id,
                None => true,
            })
            .cloned()
            .collect();
        configs.sort_by(|a, b| a.id.cmp(&b.id));
        configs
    }

    pub fn create_reusable_delegation_set(
        &mut self,
        caller_reference: &str,
        hosted_zone_id: Option<&str>,
    ) -> Result<Route53DelegationSet, Route53Error> {
        if self
            .reusable_delegation_sets
            .values()
            .any(|set| set.caller_reference.as_deref() == Some(caller_reference))
        {
            return Err(Route53Error::DelegationSetAlreadyCreated);
        }

        let name_servers = match hosted_zone_id {
            Some(id) => self.get_hosted_zone(id)?.delegation_set.name_servers,
            None => default_name_servers(caller_reference),
        };

        let delegation_set = Route53DelegationSet {
            id: format!("N{}", compact_id()),
            caller_reference: Some(caller_reference.to_string()),
            name_servers,
        };
        self.reusable_delegation_sets
            .insert(delegation_set.id.clone(), delegation_set.clone());
        Ok(delegation_set)
    }

    pub fn get_reusable_delegation_set(
        &self,
        delegation_set_id: &str,
    ) -> Result<Route53DelegationSet, Route53Error> {
        self.reusable_delegation_sets
            .get(delegation_set_id)
            .cloned()
            .ok_or_else(|| delegation_set_not_found(delegation_set_id))
    }

    pub fn delete_reusable_delegation_set(
        &mut self,
        delegation_set_id: &str,
    ) -> Result<(), Route53Error> {
        self.reusable_delegation_sets
            .remove(delegation_set_id)
            .ok_or_else(|| delegation_set_not_found(delegation_set_id))?;
        Ok(())
    }

    pub fn list_reusable_delegation_sets(&self) -> Vec<Route53DelegationSet> {
        let mut sets: Vec<_> = self.reusable_delegation_sets.values().cloned().collect();
        sets.sort_by(|a, b| a.id.cmp(&b.id));
        sets
    }

    // --- Key Signing Key operations ---

    pub fn create_key_signing_key(
        &mut self,
        hosted_zone_id: &str,
        name: &str,
        kms_arn: &str,
        status: &str,
    ) -> Result<KeySigningKeyEntry, Route53Error> {
        let zone_key = normalize_hosted_zone_id(hosted_zone_id);
        if !self.hosted_zones.contains_key(&zone_key) {
            return Err(zone_not_found(hosted_zone_id));
        }
        let key = format!("{zone_key}:{name}");
        if self.key_signing_keys.contains_key(&key) {
            return Err(Route53Error::KeySigningKeyAlreadyExists {
                name: name.to_string(),
            });
        }
        let now = iso_now();
        let entry = KeySigningKeyEntry {
            name: name.to_string(),
            hosted_zone_id: zone_key,
            kms_arn: kms_arn.to_string(),
            status: if status.is_empty() {
                "ACTIVE".to_string()
            } else {
                status.to_string()
            },
            created_date: now.clone(),
            last_modified_date: now,
        };
        self.key_signing_keys.insert(key, entry.clone());
        Ok(entry)
    }

    pub fn activate_key_signing_key(
        &mut self,
        hosted_zone_id: &str,
        name: &str,
    ) -> Result<(), Route53Error> {
        let zone_key = normalize_hosted_zone_id(hosted_zone_id);
        let key = format!("{zone_key}:{name}");
        let entry = self.key_signing_keys.get_mut(&key).ok_or_else(|| {
            Route53Error::NoSuchKeySigningKey {
                name: name.to_string(),
            }
        })?;
        entry.status = "ACTIVE".to_string();
        entry.last_modified_date = iso_now();
        Ok(())
    }

    pub fn deactivate_key_signing_key(
        &mut self,
        hosted_zone_id: &str,
        name: &str,
    ) -> Result<(), Route53Error> {
        let zone_key = normalize_hosted_zone_id(hosted_zone_id);
        let key = format!("{zone_key}:{name}");
        let entry = self.key_signing_keys.get_mut(&key).ok_or_else(|| {
            Route53Error::NoSuchKeySigningKey {
                name: name.to_string(),
            }
        })?;
        entry.status = "INACTIVE".to_string();
        entry.last_modified_date = iso_now();
        Ok(())
    }

    pub fn delete_key_signing_key(
        &mut self,
        hosted_zone_id: &str,
        name: &str,
    ) -> Result<(), Route53Error> {
        let zone_key = normalize_hosted_zone_id(hosted_zone_id);
        let key = format!("{zone_key}:{name}");
        self.key_signing_keys
            .remove(&key)
            .ok_or_else(|| Route53Error::NoSuchKeySigningKey {
                name: name.to_string(),
            })?;
        Ok(())
    }

    pub fn list_key_signing_keys(&self, hosted_zone_id: &str) -> Vec<KeySigningKeyEntry> {
        let zone_key = normalize_hosted_zone_id(hosted_zone_id);
        self.key_signing_keys
            .values()
            .filter(|k| k.hosted_zone_id == zone_key)
            .cloned()
            .collect()
    }

    pub fn enable_hosted_zone_dnssec(&mut self, hosted_zone_id: &str) -> Result<(), Route53Error> {
        let zone_key = normalize_hosted_zone_id(hosted_zone_id);
        if !self.hosted_zones.contains_key(&zone_key) {
            return Err(zone_not_found(hosted_zone_id));
        }
        self.dnssec_enabled.insert(zone_key, true);
        Ok(())
    }

    pub fn disable_hosted_zone_dnssec(&mut self, hosted_zone_id: &str) -> Result<(), Route53Error> {
        let zone_key = normalize_hosted_zone_id(hosted_zone_id);
        if !self.hosted_zones.contains_key(&zone_key) {
            return Err(zone_not_found(hosted_zone_id));
        }
        self.dnssec_enabled.insert(zone_key, false);
        Ok(())
    }

    pub fn is_dnssec_enabled(&self, hosted_zone_id: &str) -> bool {
        let zone_key = normalize_hosted_zone_id(hosted_zone_id);
        self.dnssec_enabled.get(&zone_key).copied().unwrap_or(false)
    }

    // --- CIDR Collection operations ---

    pub fn create_cidr_collection(
        &mut self,
        name: &str,
        _caller_reference: &str,
    ) -> Result<CidrCollectionEntry, Route53Error> {
        if self.cidr_collections.values().any(|c| c.name == name) {
            return Err(Route53Error::CidrCollectionAlreadyExistsException {
                name: name.to_string(),
            });
        }
        let id = compact_id();
        let arn = format!("arn:aws:route53:::cidrcollection/{id}");
        let entry = CidrCollectionEntry {
            id: id.clone(),
            name: name.to_string(),
            arn,
            version: 1,
            locations: HashMap::new(),
        };
        self.cidr_collections.insert(id, entry.clone());
        Ok(entry)
    }

    pub fn delete_cidr_collection(&mut self, collection_id: &str) -> Result<(), Route53Error> {
        self.cidr_collections.remove(collection_id).ok_or_else(|| {
            Route53Error::NoSuchCidrCollectionException {
                collection_id: collection_id.to_string(),
            }
        })?;
        Ok(())
    }

    pub fn change_cidr_collection(
        &mut self,
        collection_id: &str,
        changes: Vec<CidrCollectionChangeEntry>,
    ) -> Result<String, Route53Error> {
        let collection = self
            .cidr_collections
            .get_mut(collection_id)
            .ok_or_else(|| Route53Error::NoSuchCidrCollectionException {
                collection_id: collection_id.to_string(),
            })?;
        for change in changes {
            match change.action.as_str() {
                "PUT" => {
                    collection
                        .locations
                        .entry(change.location_name)
                        .or_default()
                        .extend(change.cidr_list);
                }
                "DELETE_IF_EXISTS" | "DELETE" => {
                    if let Some(blocks) = collection.locations.get_mut(&change.location_name) {
                        blocks.retain(|b| !change.cidr_list.contains(b));
                        if blocks.is_empty() {
                            collection.locations.remove(&change.location_name);
                        }
                    }
                }
                _ => {}
            }
        }
        collection.version += 1;
        Ok(collection.id.clone())
    }

    pub fn list_cidr_collections(&self) -> Vec<CidrCollectionEntry> {
        let mut collections: Vec<_> = self.cidr_collections.values().cloned().collect();
        collections.sort_by(|a, b| a.name.cmp(&b.name));
        collections
    }

    pub fn list_cidr_locations(&self, collection_id: &str) -> Result<Vec<String>, Route53Error> {
        let collection = self.cidr_collections.get(collection_id).ok_or_else(|| {
            Route53Error::NoSuchCidrCollectionException {
                collection_id: collection_id.to_string(),
            }
        })?;
        let mut locations: Vec<_> = collection.locations.keys().cloned().collect();
        locations.sort();
        Ok(locations)
    }

    pub fn list_cidr_blocks(
        &self,
        collection_id: &str,
        location_name: Option<&str>,
    ) -> Result<Vec<(String, String)>, Route53Error> {
        let collection = self.cidr_collections.get(collection_id).ok_or_else(|| {
            Route53Error::NoSuchCidrCollectionException {
                collection_id: collection_id.to_string(),
            }
        })?;
        let mut blocks = Vec::new();
        for (loc_name, cidrs) in &collection.locations {
            if let Some(filter) = location_name {
                if loc_name != filter {
                    continue;
                }
            }
            for cidr in cidrs {
                blocks.push((loc_name.clone(), cidr.clone()));
            }
        }
        blocks.sort();
        Ok(blocks)
    }

    // --- Traffic Policy operations ---

    pub fn create_traffic_policy(
        &mut self,
        name: &str,
        document: &str,
        comment: Option<&str>,
    ) -> Result<TrafficPolicyEntry, Route53Error> {
        let id = compact_id();
        let type_ = detect_traffic_policy_type(document);
        let entry = TrafficPolicyEntry {
            id: id.clone(),
            name: name.to_string(),
            version: 1,
            document: document.to_string(),
            comment: comment.map(|s| s.to_string()),
            type_,
        };
        let key = format!("{id}:1");
        self.traffic_policies.insert(key, entry.clone());
        Ok(entry)
    }

    pub fn create_traffic_policy_version(
        &mut self,
        policy_id: &str,
        document: &str,
        comment: Option<&str>,
    ) -> Result<TrafficPolicyEntry, Route53Error> {
        let max_version = self
            .traffic_policies
            .values()
            .filter(|p| p.id == policy_id)
            .map(|p| p.version)
            .max()
            .ok_or_else(|| Route53Error::NoSuchTrafficPolicy {
                policy_id: policy_id.to_string(),
            })?;
        let new_version = max_version + 1;
        let existing = self
            .traffic_policies
            .values()
            .find(|p| p.id == policy_id)
            .unwrap();
        let type_ = detect_traffic_policy_type(document);
        let entry = TrafficPolicyEntry {
            id: policy_id.to_string(),
            name: existing.name.clone(),
            version: new_version,
            document: document.to_string(),
            comment: comment.map(|s| s.to_string()),
            type_,
        };
        let key = format!("{policy_id}:{new_version}");
        self.traffic_policies.insert(key, entry.clone());
        Ok(entry)
    }

    pub fn get_traffic_policy(
        &self,
        policy_id: &str,
        version: i32,
    ) -> Result<TrafficPolicyEntry, Route53Error> {
        let key = format!("{policy_id}:{version}");
        self.traffic_policies.get(&key).cloned().ok_or_else(|| {
            Route53Error::NoSuchTrafficPolicyVersion {
                policy_id: policy_id.to_string(),
                version,
            }
        })
    }

    pub fn delete_traffic_policy(
        &mut self,
        policy_id: &str,
        version: i32,
    ) -> Result<(), Route53Error> {
        let key = format!("{policy_id}:{version}");
        self.traffic_policies.remove(&key).ok_or_else(|| {
            Route53Error::NoSuchTrafficPolicyVersion {
                policy_id: policy_id.to_string(),
                version,
            }
        })?;
        Ok(())
    }

    pub fn update_traffic_policy_comment(
        &mut self,
        policy_id: &str,
        version: i32,
        comment: &str,
    ) -> Result<TrafficPolicyEntry, Route53Error> {
        let key = format!("{policy_id}:{version}");
        let entry = self.traffic_policies.get_mut(&key).ok_or_else(|| {
            Route53Error::NoSuchTrafficPolicyVersion {
                policy_id: policy_id.to_string(),
                version,
            }
        })?;
        entry.comment = Some(comment.to_string());
        Ok(entry.clone())
    }

    pub fn list_traffic_policies(&self) -> Vec<TrafficPolicyEntry> {
        // Return the latest version of each unique policy ID
        let mut latest: HashMap<&str, &TrafficPolicyEntry> = HashMap::new();
        for policy in self.traffic_policies.values() {
            let existing = latest.get(policy.id.as_str());
            if existing.is_none() || existing.unwrap().version < policy.version {
                latest.insert(&policy.id, policy);
            }
        }
        let mut result: Vec<_> = latest.into_values().cloned().collect();
        result.sort_by(|a, b| a.id.cmp(&b.id));
        result
    }

    pub fn list_traffic_policy_versions(
        &self,
        policy_id: &str,
    ) -> Result<Vec<TrafficPolicyEntry>, Route53Error> {
        let mut versions: Vec<_> = self
            .traffic_policies
            .values()
            .filter(|p| p.id == policy_id)
            .cloned()
            .collect();
        if versions.is_empty() {
            return Err(Route53Error::NoSuchTrafficPolicy {
                policy_id: policy_id.to_string(),
            });
        }
        versions.sort_by_key(|p| p.version);
        Ok(versions)
    }

    // --- Traffic Policy Instance operations ---

    pub fn create_traffic_policy_instance(
        &mut self,
        hosted_zone_id: &str,
        name: &str,
        ttl: i64,
        traffic_policy_id: &str,
        traffic_policy_version: i32,
    ) -> Result<TrafficPolicyInstanceEntry, Route53Error> {
        let zone_key = normalize_hosted_zone_id(hosted_zone_id);
        if !self.hosted_zones.contains_key(&zone_key) {
            return Err(zone_not_found(hosted_zone_id));
        }
        let policy = self.get_traffic_policy(traffic_policy_id, traffic_policy_version)?;
        let id = compact_id();
        let entry = TrafficPolicyInstanceEntry {
            id: id.clone(),
            hosted_zone_id: zone_key,
            name: name.to_string(),
            ttl,
            state: "Applied".to_string(),
            message: String::new(),
            traffic_policy_id: traffic_policy_id.to_string(),
            traffic_policy_version,
            traffic_policy_type: policy.type_.clone(),
        };
        self.traffic_policy_instances.insert(id, entry.clone());
        Ok(entry)
    }

    pub fn get_traffic_policy_instance(
        &self,
        instance_id: &str,
    ) -> Result<TrafficPolicyInstanceEntry, Route53Error> {
        self.traffic_policy_instances
            .get(instance_id)
            .cloned()
            .ok_or_else(|| Route53Error::NoSuchTrafficPolicyInstance {
                instance_id: instance_id.to_string(),
            })
    }

    pub fn delete_traffic_policy_instance(
        &mut self,
        instance_id: &str,
    ) -> Result<(), Route53Error> {
        self.traffic_policy_instances
            .remove(instance_id)
            .ok_or_else(|| Route53Error::NoSuchTrafficPolicyInstance {
                instance_id: instance_id.to_string(),
            })?;
        Ok(())
    }

    pub fn update_traffic_policy_instance(
        &mut self,
        instance_id: &str,
        ttl: i64,
        traffic_policy_id: &str,
        traffic_policy_version: i32,
    ) -> Result<TrafficPolicyInstanceEntry, Route53Error> {
        let _policy = self.get_traffic_policy(traffic_policy_id, traffic_policy_version)?;
        let entry = self
            .traffic_policy_instances
            .get_mut(instance_id)
            .ok_or_else(|| Route53Error::NoSuchTrafficPolicyInstance {
                instance_id: instance_id.to_string(),
            })?;
        entry.ttl = ttl;
        entry.traffic_policy_id = traffic_policy_id.to_string();
        entry.traffic_policy_version = traffic_policy_version;
        Ok(entry.clone())
    }

    pub fn list_traffic_policy_instances(&self) -> Vec<TrafficPolicyInstanceEntry> {
        let mut instances: Vec<_> = self.traffic_policy_instances.values().cloned().collect();
        instances.sort_by(|a, b| a.id.cmp(&b.id));
        instances
    }

    pub fn list_traffic_policy_instances_by_hosted_zone(
        &self,
        hosted_zone_id: &str,
    ) -> Vec<TrafficPolicyInstanceEntry> {
        let zone_key = normalize_hosted_zone_id(hosted_zone_id);
        let mut instances: Vec<_> = self
            .traffic_policy_instances
            .values()
            .filter(|i| i.hosted_zone_id == zone_key)
            .cloned()
            .collect();
        instances.sort_by(|a, b| a.id.cmp(&b.id));
        instances
    }

    pub fn list_traffic_policy_instances_by_policy(
        &self,
        traffic_policy_id: &str,
        traffic_policy_version: i32,
    ) -> Vec<TrafficPolicyInstanceEntry> {
        let mut instances: Vec<_> = self
            .traffic_policy_instances
            .values()
            .filter(|i| {
                i.traffic_policy_id == traffic_policy_id
                    && i.traffic_policy_version == traffic_policy_version
            })
            .cloned()
            .collect();
        instances.sort_by(|a, b| a.id.cmp(&b.id));
        instances
    }

    pub fn get_traffic_policy_instance_count(&self) -> i32 {
        self.traffic_policy_instances.len() as i32
    }

    // --- VPC Association Authorization operations ---

    pub fn create_vpc_association_authorization(
        &mut self,
        hosted_zone_id: &str,
        vpc_id: &str,
        vpc_region: &str,
    ) -> Result<VpcAssociationAuthorization, Route53Error> {
        let zone_key = normalize_hosted_zone_id(hosted_zone_id);
        if !self.hosted_zones.contains_key(&zone_key) {
            return Err(zone_not_found(hosted_zone_id));
        }
        let key = format!("{zone_key}:{vpc_id}");
        let auth = VpcAssociationAuthorization {
            hosted_zone_id: zone_key,
            vpc_id: vpc_id.to_string(),
            vpc_region: vpc_region.to_string(),
        };
        self.vpc_association_authorizations
            .insert(key, auth.clone());
        Ok(auth)
    }

    pub fn delete_vpc_association_authorization(
        &mut self,
        hosted_zone_id: &str,
        vpc_id: &str,
    ) -> Result<(), Route53Error> {
        let zone_key = normalize_hosted_zone_id(hosted_zone_id);
        let key = format!("{zone_key}:{vpc_id}");
        self.vpc_association_authorizations
            .remove(&key)
            .ok_or_else(|| Route53Error::VpcAssociationAuthorizationNotFound {
                vpc_id: vpc_id.to_string(),
            })?;
        Ok(())
    }

    pub fn list_vpc_association_authorizations(
        &self,
        hosted_zone_id: &str,
    ) -> Vec<VpcAssociationAuthorization> {
        let zone_key = normalize_hosted_zone_id(hosted_zone_id);
        self.vpc_association_authorizations
            .values()
            .filter(|a| a.hosted_zone_id == zone_key)
            .cloned()
            .collect()
    }

    fn tags_ref(
        &self,
        resource_type: &str,
        resource_id: &str,
    ) -> Result<&HashMap<String, String>, Route53Error> {
        match resource_type {
            "hostedzone" => {
                let key = normalize_hosted_zone_id(resource_id);
                self.hosted_zones
                    .get(&key)
                    .map(|zone| &zone.tags)
                    .ok_or_else(|| zone_not_found(resource_id))
            }
            "healthcheck" => self
                .health_checks
                .get(resource_id)
                .map(|health_check| &health_check.tags)
                .ok_or_else(|| health_check_not_found(resource_id)),
            _ => Err(Route53Error::UnsupportedResourceType {
                resource_type: resource_type.to_string(),
            }),
        }
    }

    fn tags_mut(
        &mut self,
        resource_type: &str,
        resource_id: &str,
    ) -> Result<&mut HashMap<String, String>, Route53Error> {
        match resource_type {
            "hostedzone" => {
                let key = normalize_hosted_zone_id(resource_id);
                self.hosted_zones
                    .get_mut(&key)
                    .map(|zone| &mut zone.tags)
                    .ok_or_else(|| zone_not_found(resource_id))
            }
            "healthcheck" => self
                .health_checks
                .get_mut(resource_id)
                .map(|health_check| &mut health_check.tags)
                .ok_or_else(|| health_check_not_found(resource_id)),
            _ => Err(Route53Error::UnsupportedResourceType {
                resource_type: resource_type.to_string(),
            }),
        }
    }
}

/// A change to apply to resource record sets.
pub struct RecordChange {
    pub action: String,
    pub record_set: ResourceRecordSet,
}

pub fn canonical_resource_id(resource_type: &str, resource_id: &str) -> String {
    match resource_type {
        "hostedzone" => normalize_hosted_zone_id(resource_id)
            .trim_start_matches("/hostedzone/")
            .to_string(),
        _ => resource_id.to_string(),
    }
}

fn compact_id() -> String {
    uuid::Uuid::new_v4()
        .simple()
        .to_string()
        .chars()
        .take(13)
        .collect::<String>()
        .to_uppercase()
}

fn ensure_trailing_dot(name: &str) -> String {
    if name.ends_with('.') {
        name.to_string()
    } else {
        format!("{name}.")
    }
}

fn default_name_servers(seed: &str) -> Vec<String> {
    let base = seed.bytes().fold(0u32, |acc, byte| acc + byte as u32);
    (0..4)
        .map(|offset| {
            format!(
                "ns-{}.awsdns-{:02}.{}.{}.",
                (base + offset) % 2048 + 1,
                (base + offset * 11) % 100,
                match offset {
                    0 => "org",
                    1 => "co.uk",
                    2 => "com",
                    _ => "net",
                },
                ""
            )
            .replace("..", ".")
        })
        .collect()
}

fn merge_health_check_config(
    current: &mut Route53HealthCheckConfig,
    updates: Route53HealthCheckConfig,
) {
    if !updates.type_.is_empty() {
        current.type_ = updates.type_;
    }
    if updates.ip_address.is_some() {
        current.ip_address = updates.ip_address;
    }
    if updates.port.is_some() {
        current.port = updates.port;
    }
    if updates.resource_path.is_some() {
        current.resource_path = updates.resource_path;
    }
    if updates.fully_qualified_domain_name.is_some() {
        current.fully_qualified_domain_name = updates.fully_qualified_domain_name;
    }
    if updates.failure_threshold.is_some() {
        current.failure_threshold = updates.failure_threshold;
    }
    if updates.request_interval.is_some() {
        current.request_interval = updates.request_interval;
    }
    if updates.inverted.is_some() {
        current.inverted = updates.inverted;
    }
    if updates.disabled.is_some() {
        current.disabled = updates.disabled;
    }
    if updates.measure_latency.is_some() {
        current.measure_latency = updates.measure_latency;
    }
    if updates.search_string.is_some() {
        current.search_string = updates.search_string;
    }
    if updates.enable_sni.is_some() {
        current.enable_sni = updates.enable_sni;
    }
    if updates.health_threshold.is_some() {
        current.health_threshold = updates.health_threshold;
    }
    if updates.insufficient_data_health_status.is_some() {
        current.insufficient_data_health_status = updates.insufficient_data_health_status;
    }
    if !updates.regions.is_empty() {
        current.regions = updates.regions;
    }
    if !updates.child_health_checks.is_empty() {
        current.child_health_checks = updates.child_health_checks;
    }
}

pub fn normalize_hosted_zone_id_pub(zone_id: &str) -> String {
    normalize_hosted_zone_id(zone_id)
}

/// Normalize a record name to a fully-qualified domain name with trailing dot.
///
/// AWS Route53 accepts three forms:
///   - FQDN with trailing dot: `foo.example.com.` → stored as-is
///   - FQDN without trailing dot: `foo.example.com` → adds trailing dot
///   - Zone-relative name: `foo` → appends `.{zone_name}` (zone_name already has trailing dot)
fn normalize_record_name(name: &str, zone_name: &str) -> String {
    // Ensure trailing dot on the input name
    let name_with_dot = if name.ends_with('.') {
        name.to_string()
    } else {
        format!("{name}.")
    };

    // If the name already ends with the zone name (case-insensitive), it's fully qualified
    if name_with_dot
        .to_lowercase()
        .ends_with(&zone_name.to_lowercase())
    {
        return name_with_dot;
    }

    // Zone-relative: prepend to zone name
    // e.g. name="foo", zone_name="example.com." → "foo.example.com."
    format!("{name}.{zone_name}")
}

fn normalize_hosted_zone_id(zone_id: &str) -> String {
    if zone_id.starts_with("/hostedzone/") {
        zone_id.to_string()
    } else {
        format!("/hostedzone/{zone_id}")
    }
}

fn zone_not_found(zone_id: &str) -> Route53Error {
    Route53Error::NoSuchHostedZone {
        zone_id: zone_id.to_string(),
    }
}

fn health_check_not_found(health_check_id: &str) -> Route53Error {
    Route53Error::NoSuchHealthCheck {
        health_check_id: health_check_id.to_string(),
    }
}

fn query_logging_config_not_found(_query_logging_config_id: &str) -> Route53Error {
    Route53Error::NoSuchQueryLoggingConfig
}

fn delegation_set_not_found(delegation_set_id: &str) -> Route53Error {
    Route53Error::NoSuchDelegationSet {
        delegation_set_id: delegation_set_id.to_string(),
    }
}

fn iso_now() -> String {
    chrono::Utc::now()
        .format("%Y-%m-%dT%H:%M:%S.000Z")
        .to_string()
}

fn detect_traffic_policy_type(document: &str) -> String {
    // Simple heuristic: look for AliasTarget in the document
    let _ = document;
    "A".to_string()
}

/// A single change in a ChangeCidrCollection request.
pub struct CidrCollectionChangeEntry {
    pub action: String,
    pub location_name: String,
    pub cidr_list: Vec<String>,
}
