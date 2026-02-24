use std::collections::HashMap;

use crate::types::{
    AuthorizedPrincipal, DataSource, DataSourceTypeKind, DirectQueryDataSource,
    DirectQueryDataSourceTypeKind, Domain, InboundConnection, OutboundConnection, Package,
    ReservedInstance, VpcEndpoint,
};

/// In-memory state for the OpenSearch service.
#[derive(Debug, Default)]
pub struct OpenSearchState {
    /// Domains keyed by domain name.
    pub domains: HashMap<String, Domain>,
    /// Counter for generating unique domain IDs.
    next_id: u64,
    /// Tags keyed by ARN -> (key -> value).
    pub tags: HashMap<String, HashMap<String, String>>,
    /// VPC endpoints keyed by endpoint ID.
    pub vpc_endpoints: HashMap<String, VpcEndpoint>,
    /// Authorized principals keyed by domain name -> list of principals.
    pub vpc_authorized_principals: HashMap<String, Vec<AuthorizedPrincipal>>,
    /// Data sources keyed by domain name -> (source name -> DataSource).
    pub data_sources: HashMap<String, HashMap<String, DataSource>>,
    /// Direct query data sources keyed by data source name.
    pub direct_query_data_sources: HashMap<String, DirectQueryDataSource>,
    /// Packages keyed by package ID.
    pub packages: HashMap<String, Package>,
    /// Outbound connections keyed by connection ID.
    pub outbound_connections: HashMap<String, OutboundConnection>,
    /// Inbound connections keyed by connection ID.
    pub inbound_connections: HashMap<String, InboundConnection>,
    /// Reserved instances keyed by reserved instance ID.
    pub reserved_instances: HashMap<String, ReservedInstance>,
}

/// Error type for OpenSearch operations.
#[derive(Debug, thiserror::Error)]
pub enum OpenSearchError {
    #[error(
        "domain [{domain_name}] already exists in region [{region}] for account [{account_id}]"
    )]
    DomainAlreadyExists {
        domain_name: String,
        region: String,
        account_id: String,
    },
    #[error("Domain not found: {domain_name}")]
    DomainNotFound { domain_name: String },
    #[error("VPC endpoint not found: {vpc_endpoint_id}")]
    VpcEndpointNotFound { vpc_endpoint_id: String },
    #[error("Either 'Account' or 'Service' must be provided")]
    MissingPrincipalIdentifier,
    #[error("Data source [{name}] already exists on domain [{domain_name}]")]
    DataSourceAlreadyExists { name: String, domain_name: String },
    #[error("Data source [{name}] not found on domain [{domain_name}]")]
    DataSourceNotFound { name: String, domain_name: String },
    #[error("Direct query data source [{name}] already exists")]
    DirectQueryDataSourceAlreadyExists { name: String },
    #[error("Direct query data source [{name}] not found")]
    DirectQueryDataSourceNotFound { name: String },
    #[error("Package [{package_name}] already exists")]
    PackageAlreadyExists { package_name: String },
    #[error("Package [{package_id}] not found")]
    PackageNotFound { package_id: String },
    #[error("Outbound connection [{connection_id}] not found")]
    OutboundConnectionNotFound { connection_id: String },
    #[error("Inbound connection [{connection_id}] not found")]
    InboundConnectionNotFound { connection_id: String },
}

impl OpenSearchState {
    pub fn create_domain(
        &mut self,
        domain_name: &str,
        account_id: &str,
        region: &str,
        engine_version: Option<&str>,
        instance_type: Option<&str>,
        instance_count: Option<i32>,
        ebs_enabled: Option<bool>,
        ebs_volume_size: Option<i32>,
        ebs_volume_type: Option<&str>,
        access_policies: Option<&str>,
        initial_tags: HashMap<String, String>,
    ) -> Result<&Domain, OpenSearchError> {
        if self.domains.contains_key(domain_name) {
            return Err(OpenSearchError::DomainAlreadyExists {
                domain_name: domain_name.to_string(),
                region: region.to_string(),
                account_id: account_id.to_string(),
            });
        }

        self.next_id += 1;
        let domain_id = format!("{account_id}/{domain_name}");
        let domain_arn = format!("arn:aws:es:{region}:{account_id}:domain/{domain_name}");
        let endpoint = Some(format!(
            "search-{domain_name}-{id:016x}.{region}.es.amazonaws.com",
            id = self.next_id,
        ));

        let engine = engine_version.unwrap_or("OpenSearch_2.5").to_string();
        let inst_type = instance_type.unwrap_or("m5.large.search").to_string();
        let inst_count = instance_count.unwrap_or(1);
        let ebs = ebs_enabled.unwrap_or(true);
        let vol_size = ebs_volume_size.unwrap_or(10);
        let vol_type = ebs_volume_type.unwrap_or("gp3").to_string();
        let policies = access_policies.unwrap_or("").to_string();

        let domain = Domain {
            domain_name: domain_name.to_string(),
            arn: domain_arn.clone(),
            domain_id,
            endpoint,
            engine_version: engine,
            created: true,
            deleted: false,
            processing: false,
            instance_type: inst_type,
            instance_count: inst_count,
            dedicated_master_enabled: false,
            zone_awareness_enabled: false,
            ebs_enabled: ebs,
            ebs_volume_size: vol_size,
            ebs_volume_type: vol_type,
            access_policies: policies,
            tags: initial_tags.clone(),
        };

        // Store tags under the domain ARN
        if !initial_tags.is_empty() {
            self.tags.insert(domain_arn, initial_tags);
        }

        self.domains.insert(domain_name.to_string(), domain);
        Ok(self.domains.get(domain_name).unwrap())
    }

    pub fn describe_domain(&self, domain_name: &str) -> Result<&Domain, OpenSearchError> {
        self.domains
            .get(domain_name)
            .ok_or_else(|| not_found_error(domain_name))
    }

    pub fn describe_domains(&self, domain_names: &[String]) -> Vec<&Domain> {
        domain_names
            .iter()
            .filter_map(|name| self.domains.get(name))
            .collect()
    }

    pub fn delete_domain(&mut self, domain_name: &str) -> Result<Domain, OpenSearchError> {
        let mut domain = self
            .domains
            .remove(domain_name)
            .ok_or_else(|| not_found_error(domain_name))?;
        domain.deleted = true;
        Ok(domain)
    }

    pub fn list_domain_names(&self) -> Vec<&Domain> {
        self.domains.values().collect()
    }

    pub fn update_domain_config(
        &mut self,
        domain_name: &str,
        engine_version: Option<&str>,
        instance_type: Option<&str>,
        instance_count: Option<i32>,
        ebs_enabled: Option<bool>,
        ebs_volume_size: Option<i32>,
        ebs_volume_type: Option<&str>,
        access_policies: Option<&str>,
    ) -> Result<&Domain, OpenSearchError> {
        let domain = self
            .domains
            .get_mut(domain_name)
            .ok_or_else(|| not_found_error(domain_name))?;

        if let Some(v) = engine_version {
            domain.engine_version = v.to_string();
        }
        if let Some(v) = instance_type {
            domain.instance_type = v.to_string();
        }
        if let Some(v) = instance_count {
            domain.instance_count = v;
        }
        if let Some(v) = ebs_enabled {
            domain.ebs_enabled = v;
        }
        if let Some(v) = ebs_volume_size {
            domain.ebs_volume_size = v;
        }
        if let Some(v) = ebs_volume_type {
            domain.ebs_volume_type = v.to_string();
        }
        if let Some(v) = access_policies {
            domain.access_policies = v.to_string();
        }

        Ok(self.domains.get(domain_name).unwrap())
    }

    pub fn add_tags(&mut self, arn: &str, tags: &[(String, String)]) {
        let tag_map = self.tags.entry(arn.to_string()).or_default();
        for (key, value) in tags {
            tag_map.insert(key.clone(), value.clone());
        }
    }

    pub fn remove_tags(&mut self, arn: &str, tag_keys: &[String]) {
        if let Some(tag_map) = self.tags.get_mut(arn) {
            for key in tag_keys {
                tag_map.remove(key);
            }
        }
    }

    pub fn list_tags(&self, arn: &str) -> Vec<(String, String)> {
        self.tags
            .get(arn)
            .map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            .unwrap_or_default()
    }

    // -------------------------------------------------------------------------
    // VPC Endpoints
    // -------------------------------------------------------------------------

    pub fn create_vpc_endpoint(
        &mut self,
        account_id: &str,
        region: &str,
        domain_arn: &str,
        subnet_ids: Vec<String>,
        security_group_ids: Vec<String>,
    ) -> Result<VpcEndpoint, OpenSearchError> {
        self.next_id += 1;
        let vpc_endpoint_id = format!("aos-{:016x}", self.next_id);
        let endpoint_url = format!("{vpc_endpoint_id}.{region}.es.amazonaws.com");
        let ep = VpcEndpoint {
            vpc_endpoint_id: vpc_endpoint_id.clone(),
            vpc_endpoint_owner: account_id.to_string(),
            domain_arn: domain_arn.to_string(),
            subnet_ids,
            security_group_ids,
            status: "ACTIVE".to_string(),
            endpoint: Some(endpoint_url),
        };
        self.vpc_endpoints.insert(vpc_endpoint_id, ep.clone());
        Ok(ep)
    }

    pub fn describe_vpc_endpoints(
        &self,
        vpc_endpoint_ids: &[String],
    ) -> (Vec<VpcEndpoint>, Vec<String>) {
        let mut found = Vec::new();
        let mut not_found = Vec::new();
        for id in vpc_endpoint_ids {
            if let Some(ep) = self.vpc_endpoints.get(id) {
                found.push(ep.clone());
            } else {
                not_found.push(id.clone());
            }
        }
        (found, not_found)
    }

    pub fn list_vpc_endpoints(&self) -> Vec<&VpcEndpoint> {
        self.vpc_endpoints.values().collect()
    }

    pub fn list_vpc_endpoints_for_domain(&self, domain_arn: &str) -> Vec<&VpcEndpoint> {
        self.vpc_endpoints
            .values()
            .filter(|ep| ep.domain_arn == domain_arn)
            .collect()
    }

    pub fn update_vpc_endpoint(
        &mut self,
        vpc_endpoint_id: &str,
        subnet_ids: Option<Vec<String>>,
        security_group_ids: Option<Vec<String>>,
    ) -> Result<VpcEndpoint, OpenSearchError> {
        let ep = self.vpc_endpoints.get_mut(vpc_endpoint_id).ok_or_else(|| {
            OpenSearchError::VpcEndpointNotFound {
                vpc_endpoint_id: vpc_endpoint_id.to_string(),
            }
        })?;
        if let Some(sids) = subnet_ids {
            ep.subnet_ids = sids;
        }
        if let Some(sgids) = security_group_ids {
            ep.security_group_ids = sgids;
        }
        Ok(ep.clone())
    }

    pub fn delete_vpc_endpoint(
        &mut self,
        vpc_endpoint_id: &str,
    ) -> Result<VpcEndpoint, OpenSearchError> {
        self.vpc_endpoints.remove(vpc_endpoint_id).ok_or_else(|| {
            OpenSearchError::VpcEndpointNotFound {
                vpc_endpoint_id: vpc_endpoint_id.to_string(),
            }
        })
    }

    // -------------------------------------------------------------------------
    // VPC Endpoint Access Authorization
    // -------------------------------------------------------------------------

    pub fn authorize_vpc_endpoint_access(
        &mut self,
        domain_name: &str,
        account: Option<&str>,
        service: Option<&str>,
    ) -> Result<AuthorizedPrincipal, OpenSearchError> {
        if !self.domains.contains_key(domain_name) {
            return Err(not_found_error(domain_name));
        }
        let (principal, principal_type) = if let Some(a) = account {
            (a.to_string(), "AWS_ACCOUNT".to_string())
        } else if let Some(s) = service {
            (s.to_string(), "AWS_SERVICE".to_string())
        } else {
            return Err(OpenSearchError::MissingPrincipalIdentifier);
        };
        let ap = AuthorizedPrincipal {
            principal: principal.clone(),
            principal_type: principal_type.clone(),
        };
        let list = self
            .vpc_authorized_principals
            .entry(domain_name.to_string())
            .or_default();
        if !list.iter().any(|p| p.principal == principal) {
            list.push(ap.clone());
        }
        Ok(ap)
    }

    pub fn list_vpc_endpoint_access(
        &self,
        domain_name: &str,
    ) -> Result<Vec<&AuthorizedPrincipal>, OpenSearchError> {
        if !self.domains.contains_key(domain_name) {
            return Err(not_found_error(domain_name));
        }
        Ok(self
            .vpc_authorized_principals
            .get(domain_name)
            .map(|v| v.iter().collect())
            .unwrap_or_default())
    }

    pub fn revoke_vpc_endpoint_access(
        &mut self,
        domain_name: &str,
        account: Option<&str>,
        service: Option<&str>,
    ) -> Result<(), OpenSearchError> {
        if !self.domains.contains_key(domain_name) {
            return Err(not_found_error(domain_name));
        }
        let principal = account
            .or(service)
            .ok_or(OpenSearchError::MissingPrincipalIdentifier)?;
        if let Some(list) = self.vpc_authorized_principals.get_mut(domain_name) {
            list.retain(|p| p.principal != principal);
        }
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Data Sources
    // -------------------------------------------------------------------------

    pub fn add_data_source(
        &mut self,
        domain_name: &str,
        name: &str,
        description: Option<&str>,
        data_source_type: DataSourceTypeKind,
    ) -> Result<(), OpenSearchError> {
        if !self.domains.contains_key(domain_name) {
            return Err(not_found_error(domain_name));
        }
        let ds_map = self
            .data_sources
            .entry(domain_name.to_string())
            .or_default();
        if ds_map.contains_key(name) {
            return Err(OpenSearchError::DataSourceAlreadyExists {
                name: name.to_string(),
                domain_name: domain_name.to_string(),
            });
        }
        ds_map.insert(
            name.to_string(),
            DataSource {
                name: name.to_string(),
                description: description.map(|s| s.to_string()),
                data_source_type,
                status: "ACTIVE".to_string(),
            },
        );
        Ok(())
    }

    pub fn get_data_source(
        &self,
        domain_name: &str,
        name: &str,
    ) -> Result<&DataSource, OpenSearchError> {
        if !self.domains.contains_key(domain_name) {
            return Err(not_found_error(domain_name));
        }
        self.data_sources
            .get(domain_name)
            .and_then(|m| m.get(name))
            .ok_or_else(|| OpenSearchError::DataSourceNotFound {
                name: name.to_string(),
                domain_name: domain_name.to_string(),
            })
    }

    pub fn list_data_sources(
        &self,
        domain_name: &str,
    ) -> Result<Vec<&DataSource>, OpenSearchError> {
        if !self.domains.contains_key(domain_name) {
            return Err(not_found_error(domain_name));
        }
        Ok(self
            .data_sources
            .get(domain_name)
            .map(|m| m.values().collect())
            .unwrap_or_default())
    }

    pub fn update_data_source(
        &mut self,
        domain_name: &str,
        name: &str,
        description: Option<&str>,
        data_source_type: Option<DataSourceTypeKind>,
    ) -> Result<&DataSource, OpenSearchError> {
        if !self.domains.contains_key(domain_name) {
            return Err(not_found_error(domain_name));
        }
        let ds = self
            .data_sources
            .get_mut(domain_name)
            .and_then(|m| m.get_mut(name))
            .ok_or_else(|| OpenSearchError::DataSourceNotFound {
                name: name.to_string(),
                domain_name: domain_name.to_string(),
            })?;
        if let Some(d) = description {
            ds.description = Some(d.to_string());
        }
        if let Some(t) = data_source_type {
            ds.data_source_type = t;
        }
        Ok(self
            .data_sources
            .get(domain_name)
            .unwrap()
            .get(name)
            .unwrap())
    }

    pub fn delete_data_source(
        &mut self,
        domain_name: &str,
        name: &str,
    ) -> Result<(), OpenSearchError> {
        if !self.domains.contains_key(domain_name) {
            return Err(not_found_error(domain_name));
        }
        let removed = self
            .data_sources
            .get_mut(domain_name)
            .and_then(|m| m.remove(name));
        if removed.is_none() {
            return Err(OpenSearchError::DataSourceNotFound {
                name: name.to_string(),
                domain_name: domain_name.to_string(),
            });
        }
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Direct Query Data Sources
    // -------------------------------------------------------------------------

    pub fn add_direct_query_data_source(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
        description: Option<&str>,
        data_source_type: DirectQueryDataSourceTypeKind,
        open_search_arns: Vec<String>,
        tags: Vec<(String, String)>,
    ) -> Result<&DirectQueryDataSource, OpenSearchError> {
        if self.direct_query_data_sources.contains_key(name) {
            return Err(OpenSearchError::DirectQueryDataSourceAlreadyExists {
                name: name.to_string(),
            });
        }
        self.next_id += 1;
        let arn = format!("arn:aws:opensearch:{region}:{account_id}:datasource/{name}");
        self.direct_query_data_sources.insert(
            name.to_string(),
            DirectQueryDataSource {
                data_source_name: name.to_string(),
                data_source_arn: arn,
                description: description.map(|s| s.to_string()),
                data_source_type,
                open_search_arns,
                tags,
            },
        );
        Ok(self.direct_query_data_sources.get(name).unwrap())
    }

    pub fn get_direct_query_data_source(
        &self,
        name: &str,
    ) -> Result<&DirectQueryDataSource, OpenSearchError> {
        self.direct_query_data_sources.get(name).ok_or_else(|| {
            OpenSearchError::DirectQueryDataSourceNotFound {
                name: name.to_string(),
            }
        })
    }

    pub fn list_direct_query_data_sources(&self) -> Vec<&DirectQueryDataSource> {
        self.direct_query_data_sources.values().collect()
    }

    pub fn update_direct_query_data_source(
        &mut self,
        name: &str,
        description: Option<&str>,
        data_source_type: Option<DirectQueryDataSourceTypeKind>,
        open_search_arns: Option<Vec<String>>,
    ) -> Result<&DirectQueryDataSource, OpenSearchError> {
        let ds = self
            .direct_query_data_sources
            .get_mut(name)
            .ok_or_else(|| OpenSearchError::DirectQueryDataSourceNotFound {
                name: name.to_string(),
            })?;
        if let Some(d) = description {
            ds.description = Some(d.to_string());
        }
        if let Some(t) = data_source_type {
            ds.data_source_type = t;
        }
        if let Some(arns) = open_search_arns {
            ds.open_search_arns = arns;
        }
        Ok(self.direct_query_data_sources.get(name).unwrap())
    }

    pub fn delete_direct_query_data_source(&mut self, name: &str) -> Result<(), OpenSearchError> {
        self.direct_query_data_sources.remove(name).ok_or_else(|| {
            OpenSearchError::DirectQueryDataSourceNotFound {
                name: name.to_string(),
            }
        })?;
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Packages
    // -------------------------------------------------------------------------

    pub fn create_package(
        &mut self,
        account_id: &str,
        region: &str,
        package_name: &str,
        package_type: &str,
        package_description: Option<&str>,
        engine_version: Option<&str>,
    ) -> Result<&Package, OpenSearchError> {
        // Check for duplicate name
        if self
            .packages
            .values()
            .any(|p| p.package_name == package_name)
        {
            return Err(OpenSearchError::PackageAlreadyExists {
                package_name: package_name.to_string(),
            });
        }
        self.next_id += 1;
        let package_id = format!("F{:011}", self.next_id);
        let now = 0.0_f64;
        let pkg = Package {
            package_id: package_id.clone(),
            package_name: package_name.to_string(),
            package_type: package_type.to_string(),
            package_description: package_description.map(|s| s.to_string()),
            engine_version: engine_version.map(|s| s.to_string()),
            status: "AVAILABLE".to_string(),
            created_at: now,
            last_updated_at: now,
            associated_domains: Vec::new(),
        };
        let _ = (account_id, region);
        self.packages.insert(package_id, pkg);
        // Return by looking up - we need package_id to index but we just moved it
        // Find by name since we can't use the moved package_id easily
        Ok(self
            .packages
            .values()
            .find(|p| p.package_name == package_name)
            .unwrap())
    }

    pub fn describe_packages(&self) -> Vec<&Package> {
        self.packages.values().collect()
    }

    pub fn delete_package(&mut self, package_id: &str) -> Result<Package, OpenSearchError> {
        self.packages
            .remove(package_id)
            .ok_or_else(|| OpenSearchError::PackageNotFound {
                package_id: package_id.to_string(),
            })
    }

    pub fn associate_package(
        &mut self,
        package_id: &str,
        domain_name: &str,
    ) -> Result<(), OpenSearchError> {
        if !self.domains.contains_key(domain_name) {
            return Err(not_found_error(domain_name));
        }
        let pkg =
            self.packages
                .get_mut(package_id)
                .ok_or_else(|| OpenSearchError::PackageNotFound {
                    package_id: package_id.to_string(),
                })?;
        if !pkg.associated_domains.contains(&domain_name.to_string()) {
            pkg.associated_domains.push(domain_name.to_string());
        }
        Ok(())
    }

    pub fn dissociate_package(
        &mut self,
        package_id: &str,
        domain_name: &str,
    ) -> Result<(), OpenSearchError> {
        if !self.domains.contains_key(domain_name) {
            return Err(not_found_error(domain_name));
        }
        let pkg =
            self.packages
                .get_mut(package_id)
                .ok_or_else(|| OpenSearchError::PackageNotFound {
                    package_id: package_id.to_string(),
                })?;
        pkg.associated_domains.retain(|d| d != domain_name);
        Ok(())
    }

    pub fn list_packages_for_domain(&self, domain_name: &str) -> Vec<&Package> {
        self.packages
            .values()
            .filter(|p| p.associated_domains.contains(&domain_name.to_string()))
            .collect()
    }

    pub fn list_domains_for_package(&self, package_id: &str) -> Vec<&str> {
        self.packages
            .get(package_id)
            .map(|p| p.associated_domains.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    // -------------------------------------------------------------------------
    // Cross-Cluster Connections (Outbound)
    // -------------------------------------------------------------------------

    pub fn create_outbound_connection(
        &mut self,
        connection_alias: &str,
        connection_mode: Option<&str>,
        local_domain_name: &str,
        local_owner_id: &str,
        local_region: Option<&str>,
        remote_domain_name: &str,
        remote_owner_id: Option<&str>,
        remote_region: Option<&str>,
    ) -> OutboundConnection {
        self.next_id += 1;
        let connection_id = format!("cc-{:016x}", self.next_id);
        let conn = OutboundConnection {
            connection_id: connection_id.clone(),
            connection_alias: connection_alias.to_string(),
            connection_mode: connection_mode.map(|s| s.to_string()),
            status_code: "VALIDATING".to_string(),
            local_domain_name: local_domain_name.to_string(),
            local_owner_id: local_owner_id.to_string(),
            local_region: local_region.map(|s| s.to_string()),
            remote_domain_name: remote_domain_name.to_string(),
            remote_owner_id: remote_owner_id.map(|s| s.to_string()),
            remote_region: remote_region.map(|s| s.to_string()),
        };
        self.outbound_connections
            .insert(connection_id, conn.clone());
        conn
    }

    pub fn describe_outbound_connections(&self) -> Vec<&OutboundConnection> {
        self.outbound_connections.values().collect()
    }

    pub fn delete_outbound_connection(
        &mut self,
        connection_id: &str,
    ) -> Result<OutboundConnection, OpenSearchError> {
        self.outbound_connections
            .remove(connection_id)
            .ok_or_else(|| OpenSearchError::OutboundConnectionNotFound {
                connection_id: connection_id.to_string(),
            })
    }

    // -------------------------------------------------------------------------
    // Cross-Cluster Connections (Inbound)
    // -------------------------------------------------------------------------

    pub fn describe_inbound_connections(&self) -> Vec<&InboundConnection> {
        self.inbound_connections.values().collect()
    }

    pub fn accept_inbound_connection(
        &mut self,
        connection_id: &str,
    ) -> Result<&InboundConnection, OpenSearchError> {
        let conn = self
            .inbound_connections
            .get_mut(connection_id)
            .ok_or_else(|| OpenSearchError::InboundConnectionNotFound {
                connection_id: connection_id.to_string(),
            })?;
        conn.status_code = "ACTIVE".to_string();
        Ok(self.inbound_connections.get(connection_id).unwrap())
    }

    pub fn reject_inbound_connection(
        &mut self,
        connection_id: &str,
    ) -> Result<InboundConnection, OpenSearchError> {
        let mut conn = self
            .inbound_connections
            .remove(connection_id)
            .ok_or_else(|| OpenSearchError::InboundConnectionNotFound {
                connection_id: connection_id.to_string(),
            })?;
        conn.status_code = "REJECTED".to_string();
        Ok(conn)
    }

    pub fn delete_inbound_connection(
        &mut self,
        connection_id: &str,
    ) -> Result<InboundConnection, OpenSearchError> {
        self.inbound_connections
            .remove(connection_id)
            .ok_or_else(|| OpenSearchError::InboundConnectionNotFound {
                connection_id: connection_id.to_string(),
            })
    }

    // -------------------------------------------------------------------------
    // Reserved Instances
    // -------------------------------------------------------------------------

    pub fn describe_reserved_instances(&self) -> Vec<&ReservedInstance> {
        self.reserved_instances.values().collect()
    }

    pub fn purchase_reserved_instance_offering(
        &mut self,
        reservation_name: &str,
        reserved_instance_offering_id: &str,
        instance_count: Option<i32>,
    ) -> ReservedInstance {
        self.next_id += 1;
        let reserved_instance_id = format!("ri-{:016x}", self.next_id);
        let ri = ReservedInstance {
            reserved_instance_id: reserved_instance_id.clone(),
            reservation_name: reservation_name.to_string(),
            reserved_instance_offering_id: reserved_instance_offering_id.to_string(),
            instance_type: "m5.large.search".to_string(),
            instance_count: instance_count.unwrap_or(1),
            duration: 31536000, // 1 year in seconds
            fixed_price: 0.0,
            usage_price: 0.0,
            currency_code: "USD".to_string(),
            payment_option: "NO_UPFRONT".to_string(),
            state: "payment-pending".to_string(),
        };
        self.reserved_instances
            .insert(reserved_instance_id, ri.clone());
        ri
    }
}

fn not_found_error(domain_name: &str) -> OpenSearchError {
    OpenSearchError::DomainNotFound {
        domain_name: domain_name.to_string(),
    }
}
