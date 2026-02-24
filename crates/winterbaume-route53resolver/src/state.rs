use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct Route53ResolverState {
    pub endpoints: HashMap<String, ResolverEndpoint>,
    pub resolver_rules: HashMap<String, ResolverRule>,
    pub rule_associations: HashMap<String, ResolverRuleAssociation>,
    pub query_log_configs: HashMap<String, ResolverQueryLogConfig>,
    pub query_log_config_associations: HashMap<String, ResolverQueryLogConfigAssociation>,
    pub dnssec_configs: HashMap<String, ResolverDnssecConfig>,
}

#[derive(Debug, thiserror::Error)]
pub enum Route53ResolverError {
    #[error("Invalid direction: {direction}. Must be INBOUND or OUTBOUND.")]
    InvalidDirection { direction: String },
    #[error("SecurityGroupIds must not be empty.")]
    EmptySecurityGroupIds,
    #[error("Resolver endpoint with ID '{endpoint_id}' does not exist.")]
    EndpointNotFound { endpoint_id: String },
    #[error("Cannot remove IP address: minimum 2 IP addresses required.")]
    MinimumIpAddressesRequired,
    #[error("The specified IP address was not found.")]
    IpAddressNotFound,
    #[error("Invalid RuleType: {rule_type}.")]
    InvalidRuleType { rule_type: String },
    #[error("Resolver rule with ID '{rule_id}' does not exist.")]
    RuleNotFound { rule_id: String },
    #[error(
        "Resolver rule '{rule_id}' is associated with one or more VPCs. Disassociate before deleting."
    )]
    RuleInUse { rule_id: String },
    #[error("Resolver rule '{resolver_rule_id}' is already associated with VPC '{vpc_id}'.")]
    RuleAlreadyAssociated {
        resolver_rule_id: String,
        vpc_id: String,
    },
    #[error("No association found for resolver rule '{resolver_rule_id}' and VPC '{vpc_id}'.")]
    RuleAssociationNotFoundByRuleAndVpc {
        resolver_rule_id: String,
        vpc_id: String,
    },
    #[error("Resolver rule association with ID '{assoc_id}' does not exist.")]
    RuleAssociationNotFound { assoc_id: String },
    #[error("Resolver query log config with ID '{config_id}' does not exist.")]
    QueryLogConfigNotFound { config_id: String },
    #[error("Resolver query log config association with ID '{assoc_id}' does not exist.")]
    QueryLogConfigAssociationNotFound { assoc_id: String },
    #[error("Resource with ARN '{resource_arn}' does not exist.")]
    ResourceNotFoundByArn { resource_arn: String },
}

#[derive(Debug)]
pub struct IpAddressRequest {
    pub subnet_id: String,
    pub ip: Option<String>,
}

impl Route53ResolverState {
    // --- Resolver Endpoint operations ---

    pub fn create_resolver_endpoint(
        &mut self,
        name: &str,
        creator_request_id: &str,
        security_group_ids: Vec<String>,
        direction: &str,
        ip_addresses: &[IpAddressRequest],
        protocols: Vec<String>,
        resolver_endpoint_type: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&ResolverEndpoint, Route53ResolverError> {
        if direction != "INBOUND" && direction != "OUTBOUND" {
            return Err(Route53ResolverError::InvalidDirection {
                direction: direction.to_string(),
            });
        }

        if security_group_ids.is_empty() {
            return Err(Route53ResolverError::EmptySecurityGroupIds);
        }

        let endpoint_id = format!(
            "rslvr-{}-{}",
            if direction == "INBOUND" { "in" } else { "out" },
            &uuid::Uuid::new_v4().to_string()[..17]
        );
        let arn = format!(
            "arn:aws:route53resolver:{region}:{account_id}:resolver-endpoint/{endpoint_id}"
        );

        let host_vpc_id = format!("vpc-{}", &uuid::Uuid::new_v4().to_string()[..8]);

        let now = Utc::now();

        let ip_entries: Vec<IpAddressEntry> = ip_addresses
            .iter()
            .map(|req| {
                let ip_id = format!("rni-{}", &uuid::Uuid::new_v4().to_string()[..8]);
                IpAddressEntry {
                    ip_id,
                    subnet_id: req.subnet_id.clone(),
                    ip: req
                        .ip
                        .clone()
                        .or_else(|| Some(format!("10.0.{}.{}", rand_u8(), rand_u8()))),
                    status: "ATTACHED".to_string(),
                    status_message: "This IP address is operational.".to_string(),
                    creation_time: now,
                    modification_time: now,
                }
            })
            .collect();

        let endpoint = ResolverEndpoint {
            id: endpoint_id.clone(),
            arn,
            name: name.to_string(),
            security_group_ids,
            direction: direction.to_string(),
            ip_address_count: ip_entries.len() as i32,
            host_vpc_id,
            status: "OPERATIONAL".to_string(),
            status_message: "This Resolver Endpoint is operational.".to_string(),
            creation_time: now,
            modification_time: now,
            creator_request_id: creator_request_id.to_string(),
            protocols: if protocols.is_empty() {
                vec!["Do53".to_string()]
            } else {
                protocols
            },
            resolver_endpoint_type: if resolver_endpoint_type.is_empty() {
                "IPV4".to_string()
            } else {
                resolver_endpoint_type.to_string()
            },
            ip_addresses: ip_entries,
            tags: HashMap::new(),
        };

        self.endpoints.insert(endpoint_id.clone(), endpoint);
        Ok(self.endpoints.get(&endpoint_id).unwrap())
    }

    pub fn get_resolver_endpoint(
        &self,
        endpoint_id: &str,
    ) -> Result<&ResolverEndpoint, Route53ResolverError> {
        self.endpoints
            .get(endpoint_id)
            .ok_or_else(|| Route53ResolverError::EndpointNotFound {
                endpoint_id: endpoint_id.to_string(),
            })
    }

    pub fn delete_resolver_endpoint(
        &mut self,
        endpoint_id: &str,
    ) -> Result<ResolverEndpoint, Route53ResolverError> {
        match self.endpoints.remove(endpoint_id) {
            Some(mut endpoint) => {
                endpoint.status = "DELETING".to_string();
                endpoint.status_message = "This Resolver Endpoint is being deleted.".to_string();
                Ok(endpoint)
            }
            None => Err(Route53ResolverError::EndpointNotFound {
                endpoint_id: endpoint_id.to_string(),
            }),
        }
    }

    pub fn list_resolver_endpoints(&self) -> Vec<&ResolverEndpoint> {
        self.endpoints.values().collect()
    }

    pub fn update_resolver_endpoint(
        &mut self,
        endpoint_id: &str,
        name: Option<&str>,
        protocols: Option<Vec<String>>,
        resolver_endpoint_type: Option<&str>,
    ) -> Result<&ResolverEndpoint, Route53ResolverError> {
        let endpoint = self.endpoints.get_mut(endpoint_id).ok_or_else(|| {
            Route53ResolverError::EndpointNotFound {
                endpoint_id: endpoint_id.to_string(),
            }
        })?;

        if let Some(n) = name {
            endpoint.name = n.to_string();
        }
        if let Some(p) = protocols {
            endpoint.protocols = p;
        }
        if let Some(t) = resolver_endpoint_type {
            endpoint.resolver_endpoint_type = t.to_string();
        }
        endpoint.modification_time = Utc::now();

        Ok(self.endpoints.get(endpoint_id).unwrap())
    }

    pub fn associate_resolver_endpoint_ip_address(
        &mut self,
        endpoint_id: &str,
        ip_address: &IpAddressRequest,
    ) -> Result<&ResolverEndpoint, Route53ResolverError> {
        let endpoint = self.endpoints.get_mut(endpoint_id).ok_or_else(|| {
            Route53ResolverError::EndpointNotFound {
                endpoint_id: endpoint_id.to_string(),
            }
        })?;

        let now = Utc::now();
        let ip_id = format!("rni-{}", &uuid::Uuid::new_v4().to_string()[..8]);
        let entry = IpAddressEntry {
            ip_id,
            subnet_id: ip_address.subnet_id.clone(),
            ip: ip_address
                .ip
                .clone()
                .or_else(|| Some(format!("10.0.{}.{}", rand_u8(), rand_u8()))),
            status: "ATTACHED".to_string(),
            status_message: "This IP address is operational.".to_string(),
            creation_time: now,
            modification_time: now,
        };
        endpoint.ip_addresses.push(entry);
        endpoint.ip_address_count = endpoint.ip_addresses.len() as i32;
        endpoint.modification_time = now;

        Ok(self.endpoints.get(endpoint_id).unwrap())
    }

    pub fn disassociate_resolver_endpoint_ip_address(
        &mut self,
        endpoint_id: &str,
        ip_address_ip: Option<&str>,
        subnet_id: Option<&str>,
    ) -> Result<&ResolverEndpoint, Route53ResolverError> {
        let endpoint = self.endpoints.get_mut(endpoint_id).ok_or_else(|| {
            Route53ResolverError::EndpointNotFound {
                endpoint_id: endpoint_id.to_string(),
            }
        })?;

        if endpoint.ip_addresses.len() <= 2 {
            return Err(Route53ResolverError::MinimumIpAddressesRequired);
        }

        let idx = endpoint.ip_addresses.iter().position(|e| {
            if let Some(ip) = ip_address_ip
                && e.ip.as_deref() == Some(ip)
            {
                return true;
            }
            if let Some(sid) = subnet_id
                && e.subnet_id == sid
            {
                return true;
            }
            false
        });

        match idx {
            Some(i) => {
                endpoint.ip_addresses.remove(i);
                endpoint.ip_address_count = endpoint.ip_addresses.len() as i32;
                endpoint.modification_time = Utc::now();
                Ok(self.endpoints.get(endpoint_id).unwrap())
            }
            None => Err(Route53ResolverError::IpAddressNotFound),
        }
    }

    pub fn list_resolver_endpoint_ip_addresses(
        &self,
        endpoint_id: &str,
    ) -> Result<Vec<&IpAddressEntry>, Route53ResolverError> {
        let endpoint = self.endpoints.get(endpoint_id).ok_or_else(|| {
            Route53ResolverError::EndpointNotFound {
                endpoint_id: endpoint_id.to_string(),
            }
        })?;
        Ok(endpoint.ip_addresses.iter().collect())
    }

    // --- Resolver Rule operations ---

    pub fn create_resolver_rule(
        &mut self,
        name: &str,
        creator_request_id: &str,
        domain_name: &str,
        rule_type: &str,
        resolver_endpoint_id: Option<&str>,
        target_ips: Vec<TargetAddress>,
        account_id: &str,
        region: &str,
    ) -> Result<&ResolverRule, Route53ResolverError> {
        if rule_type != "FORWARD" && rule_type != "SYSTEM" && rule_type != "RECURSIVE" {
            return Err(Route53ResolverError::InvalidRuleType {
                rule_type: rule_type.to_string(),
            });
        }

        let rule_id = format!("rslvr-rr-{}", &uuid::Uuid::new_v4().to_string()[..17]);
        let arn = format!("arn:aws:route53resolver:{region}:{account_id}:resolver-rule/{rule_id}");

        let now = Utc::now();
        let rule = ResolverRule {
            id: rule_id.clone(),
            arn,
            name: name.to_string(),
            domain_name: domain_name.to_string(),
            rule_type: rule_type.to_string(),
            resolver_endpoint_id: resolver_endpoint_id.map(|s| s.to_string()),
            target_ips,
            status: "COMPLETE".to_string(),
            status_message: "".to_string(),
            owner_id: account_id.to_string(),
            share_status: "NOT_SHARED".to_string(),
            creator_request_id: creator_request_id.to_string(),
            creation_time: now,
            modification_time: now,
            tags: HashMap::new(),
        };

        self.resolver_rules.insert(rule_id.clone(), rule);
        Ok(self.resolver_rules.get(&rule_id).unwrap())
    }

    pub fn get_resolver_rule(&self, rule_id: &str) -> Result<&ResolverRule, Route53ResolverError> {
        self.resolver_rules
            .get(rule_id)
            .ok_or_else(|| Route53ResolverError::RuleNotFound {
                rule_id: rule_id.to_string(),
            })
    }

    pub fn delete_resolver_rule(
        &mut self,
        rule_id: &str,
    ) -> Result<ResolverRule, Route53ResolverError> {
        // Check for active associations
        let has_associations = self
            .rule_associations
            .values()
            .any(|a| a.resolver_rule_id == rule_id && a.status == "COMPLETE");
        if has_associations {
            return Err(Route53ResolverError::RuleInUse {
                rule_id: rule_id.to_string(),
            });
        }

        match self.resolver_rules.remove(rule_id) {
            Some(mut rule) => {
                rule.status = "DELETING".to_string();
                Ok(rule)
            }
            None => Err(Route53ResolverError::RuleNotFound {
                rule_id: rule_id.to_string(),
            }),
        }
    }

    pub fn list_resolver_rules(&self) -> Vec<&ResolverRule> {
        self.resolver_rules.values().collect()
    }

    // --- Resolver Rule Association operations ---

    pub fn associate_resolver_rule(
        &mut self,
        resolver_rule_id: &str,
        vpc_id: &str,
        name: &str,
    ) -> Result<&ResolverRuleAssociation, Route53ResolverError> {
        // Verify rule exists
        if !self.resolver_rules.contains_key(resolver_rule_id) {
            return Err(Route53ResolverError::RuleNotFound {
                rule_id: resolver_rule_id.to_string(),
            });
        }

        // Check for duplicate
        let duplicate = self
            .rule_associations
            .values()
            .any(|a| a.resolver_rule_id == resolver_rule_id && a.vpc_id == vpc_id);
        if duplicate {
            return Err(Route53ResolverError::RuleAlreadyAssociated {
                resolver_rule_id: resolver_rule_id.to_string(),
                vpc_id: vpc_id.to_string(),
            });
        }

        let assoc_id = format!("rslvr-rrassoc-{}", &uuid::Uuid::new_v4().to_string()[..17]);
        let assoc = ResolverRuleAssociation {
            id: assoc_id.clone(),
            resolver_rule_id: resolver_rule_id.to_string(),
            name: name.to_string(),
            vpc_id: vpc_id.to_string(),
            status: "COMPLETE".to_string(),
            status_message: "".to_string(),
        };

        self.rule_associations.insert(assoc_id.clone(), assoc);
        Ok(self.rule_associations.get(&assoc_id).unwrap())
    }

    pub fn disassociate_resolver_rule(
        &mut self,
        resolver_rule_id: &str,
        vpc_id: &str,
    ) -> Result<ResolverRuleAssociation, Route53ResolverError> {
        let assoc_id = self
            .rule_associations
            .iter()
            .find(|(_, a)| a.resolver_rule_id == resolver_rule_id && a.vpc_id == vpc_id)
            .map(|(id, _)| id.clone());

        match assoc_id {
            Some(id) => {
                let mut assoc = self.rule_associations.remove(&id).unwrap();
                assoc.status = "DELETING".to_string();
                Ok(assoc)
            }
            None => Err(Route53ResolverError::RuleAssociationNotFoundByRuleAndVpc {
                resolver_rule_id: resolver_rule_id.to_string(),
                vpc_id: vpc_id.to_string(),
            }),
        }
    }

    pub fn get_resolver_rule_association(
        &self,
        assoc_id: &str,
    ) -> Result<&ResolverRuleAssociation, Route53ResolverError> {
        self.rule_associations.get(assoc_id).ok_or_else(|| {
            Route53ResolverError::RuleAssociationNotFound {
                assoc_id: assoc_id.to_string(),
            }
        })
    }

    pub fn list_resolver_rule_associations(&self) -> Vec<&ResolverRuleAssociation> {
        self.rule_associations.values().collect()
    }

    // --- Resolver Query Log Config operations ---

    pub fn create_resolver_query_log_config(
        &mut self,
        name: &str,
        destination_arn: &str,
        creator_request_id: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&ResolverQueryLogConfig, Route53ResolverError> {
        let config_id = format!("rqlc-{}", &uuid::Uuid::new_v4().to_string()[..17]);
        let arn = format!(
            "arn:aws:route53resolver:{region}:{account_id}:resolver-query-log-config/{config_id}"
        );

        let now = Utc::now();
        let config = ResolverQueryLogConfig {
            id: config_id.clone(),
            arn,
            name: name.to_string(),
            destination_arn: destination_arn.to_string(),
            owner_id: account_id.to_string(),
            status: "CREATED".to_string(),
            share_status: "NOT_SHARED".to_string(),
            association_count: 0,
            creator_request_id: creator_request_id.to_string(),
            creation_time: now,
            tags: HashMap::new(),
        };

        self.query_log_configs.insert(config_id.clone(), config);
        Ok(self.query_log_configs.get(&config_id).unwrap())
    }

    pub fn get_resolver_query_log_config(
        &self,
        config_id: &str,
    ) -> Result<&ResolverQueryLogConfig, Route53ResolverError> {
        self.query_log_configs.get(config_id).ok_or_else(|| {
            Route53ResolverError::QueryLogConfigNotFound {
                config_id: config_id.to_string(),
            }
        })
    }

    pub fn list_resolver_query_log_configs(&self) -> Vec<&ResolverQueryLogConfig> {
        self.query_log_configs.values().collect()
    }

    // --- Query Log Config Association operations ---

    pub fn associate_resolver_query_log_config(
        &mut self,
        resolver_query_log_config_id: &str,
        resource_id: &str,
    ) -> Result<&ResolverQueryLogConfigAssociation, Route53ResolverError> {
        if !self
            .query_log_configs
            .contains_key(resolver_query_log_config_id)
        {
            return Err(Route53ResolverError::QueryLogConfigNotFound {
                config_id: resolver_query_log_config_id.to_string(),
            });
        }

        let assoc_id = format!("rqlca-{}", &uuid::Uuid::new_v4().to_string()[..17]);
        let now = Utc::now();
        let assoc = ResolverQueryLogConfigAssociation {
            id: assoc_id.clone(),
            resolver_query_log_config_id: resolver_query_log_config_id.to_string(),
            resource_id: resource_id.to_string(),
            status: "ACTIVE".to_string(),
            error: "NONE".to_string(),
            error_message: "".to_string(),
            creation_time: now,
        };

        // Increment association count
        if let Some(config) = self.query_log_configs.get_mut(resolver_query_log_config_id) {
            config.association_count += 1;
        }

        self.query_log_config_associations
            .insert(assoc_id.clone(), assoc);
        Ok(self.query_log_config_associations.get(&assoc_id).unwrap())
    }

    pub fn get_resolver_query_log_config_association(
        &self,
        assoc_id: &str,
    ) -> Result<&ResolverQueryLogConfigAssociation, Route53ResolverError> {
        self.query_log_config_associations
            .get(assoc_id)
            .ok_or_else(|| Route53ResolverError::QueryLogConfigAssociationNotFound {
                assoc_id: assoc_id.to_string(),
            })
    }

    pub fn list_resolver_query_log_config_associations(
        &self,
    ) -> Vec<&ResolverQueryLogConfigAssociation> {
        self.query_log_config_associations.values().collect()
    }

    // --- DNSSEC Config operations ---

    pub fn get_resolver_dnssec_config(
        &mut self,
        resource_id: &str,
        account_id: &str,
    ) -> &ResolverDnssecConfig {
        if !self.dnssec_configs.contains_key(resource_id) {
            let config_id = format!("rdsc-{}", &uuid::Uuid::new_v4().to_string()[..17]);
            let config = ResolverDnssecConfig {
                id: config_id,
                owner_id: account_id.to_string(),
                resource_id: resource_id.to_string(),
                validation_status: "DISABLED".to_string(),
            };
            self.dnssec_configs.insert(resource_id.to_string(), config);
        }
        self.dnssec_configs.get(resource_id).unwrap()
    }

    pub fn update_resolver_dnssec_config(
        &mut self,
        resource_id: &str,
        validation: &str,
        account_id: &str,
    ) -> &ResolverDnssecConfig {
        if !self.dnssec_configs.contains_key(resource_id) {
            let config_id = format!("rdsc-{}", &uuid::Uuid::new_v4().to_string()[..17]);
            let config = ResolverDnssecConfig {
                id: config_id,
                owner_id: account_id.to_string(),
                resource_id: resource_id.to_string(),
                validation_status: "DISABLED".to_string(),
            };
            self.dnssec_configs.insert(resource_id.to_string(), config);
        }

        let config = self.dnssec_configs.get_mut(resource_id).unwrap();
        config.validation_status = match validation {
            "ENABLE" => "ENABLED".to_string(),
            "DISABLE" => "DISABLED".to_string(),
            _ => validation.to_string(),
        };
        self.dnssec_configs.get(resource_id).unwrap()
    }

    pub fn list_resolver_dnssec_configs(&self) -> Vec<&ResolverDnssecConfig> {
        self.dnssec_configs.values().collect()
    }

    // --- Tag operations ---

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), Route53ResolverError> {
        // Try to find the resource by ARN
        if let Some(ep) = self.endpoints.values_mut().find(|e| e.arn == resource_arn) {
            ep.tags.extend(tags);
            return Ok(());
        }
        if let Some(rule) = self
            .resolver_rules
            .values_mut()
            .find(|r| r.arn == resource_arn)
        {
            rule.tags.extend(tags);
            return Ok(());
        }
        if let Some(config) = self
            .query_log_configs
            .values_mut()
            .find(|c| c.arn == resource_arn)
        {
            config.tags.extend(tags);
            return Ok(());
        }
        Err(Route53ResolverError::ResourceNotFoundByArn {
            resource_arn: resource_arn.to_string(),
        })
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), Route53ResolverError> {
        if let Some(ep) = self.endpoints.values_mut().find(|e| e.arn == resource_arn) {
            for key in tag_keys {
                ep.tags.remove(key);
            }
            return Ok(());
        }
        if let Some(rule) = self
            .resolver_rules
            .values_mut()
            .find(|r| r.arn == resource_arn)
        {
            for key in tag_keys {
                rule.tags.remove(key);
            }
            return Ok(());
        }
        if let Some(config) = self
            .query_log_configs
            .values_mut()
            .find(|c| c.arn == resource_arn)
        {
            for key in tag_keys {
                config.tags.remove(key);
            }
            return Ok(());
        }
        Err(Route53ResolverError::ResourceNotFoundByArn {
            resource_arn: resource_arn.to_string(),
        })
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<&HashMap<String, String>, Route53ResolverError> {
        if let Some(ep) = self.endpoints.values().find(|e| e.arn == resource_arn) {
            return Ok(&ep.tags);
        }
        if let Some(rule) = self.resolver_rules.values().find(|r| r.arn == resource_arn) {
            return Ok(&rule.tags);
        }
        if let Some(config) = self
            .query_log_configs
            .values()
            .find(|c| c.arn == resource_arn)
        {
            return Ok(&config.tags);
        }
        Err(Route53ResolverError::ResourceNotFoundByArn {
            resource_arn: resource_arn.to_string(),
        })
    }
}

fn rand_u8() -> u8 {
    (uuid::Uuid::new_v4().as_bytes()[0] % 254) + 1
}
