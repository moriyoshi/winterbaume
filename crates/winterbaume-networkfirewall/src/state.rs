use std::collections::HashMap;

use crate::types::*;

/// In-memory state for the Network Firewall service.
#[derive(Debug, Default)]
pub struct NetworkFirewallState {
    /// Firewalls keyed by ARN.
    pub firewalls: HashMap<String, Firewall>,
    /// Logging configurations keyed by firewall ARN.
    pub logging_configs: HashMap<String, serde_json::Value>,
    /// Rule groups keyed by ARN.
    pub rule_groups: HashMap<String, RuleGroup>,
    /// Firewall policies keyed by ARN.
    pub firewall_policies: HashMap<String, FirewallPolicy>,
    /// Resource policies keyed by resource ARN.
    pub resource_policies: HashMap<String, ResourcePolicy>,
    /// TLS inspection configurations keyed by ARN.
    pub tls_inspection_configurations: HashMap<String, TlsInspectionConfiguration>,
    /// VPC endpoint associations keyed by ARN.
    pub vpc_endpoint_associations: HashMap<String, VpcEndpointAssociation>,
    /// Availability zone mappings keyed by firewall ARN.
    pub availability_zone_mappings: HashMap<String, Vec<AvailabilityZoneMapping>>,
    /// Transit gateway attachments keyed by attachment ID.
    pub transit_gateway_attachments: HashMap<String, TransitGatewayAttachment>,
    /// Proxies keyed by ARN.
    pub proxies: HashMap<String, NfwProxy>,
    /// Proxy configurations keyed by ARN.
    pub proxy_configurations: HashMap<String, NfwProxyConfiguration>,
    /// Proxy rule groups keyed by ARN.
    pub proxy_rule_groups: HashMap<String, NfwProxyRuleGroup>,
    /// Flow operations keyed by operation ID.
    pub flow_operations: HashMap<String, FlowOperation>,
    /// Analysis reports keyed by report ID.
    pub analysis_reports: HashMap<String, AnalysisReport>,
    /// Encryption configurations keyed by firewall ARN.
    pub encryption_configs: HashMap<String, EncryptionConfig>,
    /// Enabled analysis types keyed by firewall ARN.
    pub analysis_settings: HashMap<String, Vec<String>>,
}

/// Error type for Network Firewall operations.
#[derive(Debug, thiserror::Error)]
pub enum NfwError {
    #[error("A resource with the specified name already exists: {name}")]
    InvalidRequestDuplicateName { name: String },
    #[error("Either FirewallArn or FirewallName must be specified")]
    InvalidRequestFirewallIdentifier,
    #[error("Either RuleGroupArn or RuleGroupName must be specified")]
    InvalidRequestRuleGroupIdentifier,
    #[error("Either FirewallPolicyArn or FirewallPolicyName must be specified")]
    InvalidRequestFirewallPolicyIdentifier,
    #[error(
        "Either TLSInspectionConfigurationArn or TLSInspectionConfigurationName must be specified"
    )]
    InvalidRequestTlsInspectionConfigIdentifier,
    #[error("Either ProxyArn or ProxyName must be specified")]
    InvalidRequestProxyIdentifier,
    #[error("Either ProxyConfigurationArn or ProxyConfigurationName must be specified")]
    InvalidRequestProxyConfigIdentifier,
    #[error("Either ProxyRuleGroupArn or ProxyRuleGroupName must be specified")]
    InvalidRequestProxyRuleGroupIdentifier,
    #[error("Resource '{identifier}' not found")]
    ResourceNotFound { identifier: String },
}

impl NetworkFirewallState {
    pub fn create_firewall(
        &mut self,
        firewall_name: &str,
        firewall_policy_arn: &str,
        vpc_id: &str,
        subnet_mappings: Vec<SubnetMapping>,
        description: Option<&str>,
        delete_protection: bool,
        tags: Vec<(String, String)>,
        account_id: &str,
        region: &str,
    ) -> Result<(&Firewall, FirewallStatus), NfwError> {
        if self
            .firewalls
            .values()
            .any(|f| f.firewall_name == firewall_name)
        {
            return Err(NfwError::InvalidRequestDuplicateName {
                name: firewall_name.to_string(),
            });
        }

        let firewall_id = uuid::Uuid::new_v4().to_string();
        let arn =
            format!("arn:aws:network-firewall:{region}:{account_id}:firewall/{firewall_name}");

        let firewall = Firewall {
            firewall_name: firewall_name.to_string(),
            firewall_arn: arn.clone(),
            firewall_id,
            firewall_policy_arn: firewall_policy_arn.to_string(),
            vpc_id: vpc_id.to_string(),
            subnet_mappings,
            delete_protection,
            subnet_change_protection: false,
            firewall_policy_change_protection: false,
            availability_zone_change_protection: false,
            description: description.map(|s| s.to_string()),
            tags,
            encryption_configuration: None,
        };

        self.firewalls.insert(arn.clone(), firewall);
        let fw = self.firewalls.get(&arn).unwrap();
        Ok((fw, FirewallStatus::default()))
    }

    pub fn describe_firewall(
        &self,
        firewall_name: Option<&str>,
        firewall_arn: Option<&str>,
    ) -> Result<(&Firewall, FirewallStatus), NfwError> {
        let fw = if let Some(arn) = firewall_arn {
            self.firewalls.get(arn)
        } else if let Some(name) = firewall_name {
            self.firewalls.values().find(|f| f.firewall_name == name)
        } else {
            return Err(NfwError::InvalidRequestFirewallIdentifier);
        };

        match fw {
            Some(f) => Ok((f, FirewallStatus::default())),
            None => Err(resource_not_found_error(
                firewall_arn.or(firewall_name).unwrap_or("unknown"),
            )),
        }
    }

    pub fn delete_firewall(
        &mut self,
        firewall_name: Option<&str>,
        firewall_arn: Option<&str>,
    ) -> Result<(Firewall, FirewallStatus), NfwError> {
        let arn = if let Some(arn) = firewall_arn {
            arn.to_string()
        } else if let Some(name) = firewall_name {
            match self.firewalls.values().find(|f| f.firewall_name == name) {
                Some(f) => f.firewall_arn.clone(),
                None => return Err(resource_not_found_error(name)),
            }
        } else {
            return Err(NfwError::InvalidRequestFirewallIdentifier);
        };

        match self.firewalls.remove(&arn) {
            Some(fw) => Ok((
                fw,
                FirewallStatus {
                    status: "DELETING".to_string(),
                    configuration_sync_state_summary: "IN_SYNC".to_string(),
                },
            )),
            None => Err(resource_not_found_error(&arn)),
        }
    }

    pub fn list_firewalls(&self) -> Vec<FirewallMetadata> {
        self.firewalls
            .values()
            .map(|f| FirewallMetadata {
                firewall_name: f.firewall_name.clone(),
                firewall_arn: f.firewall_arn.clone(),
            })
            .collect()
    }

    // ---------- Rule Groups ----------

    #[allow(clippy::too_many_arguments)]
    pub fn create_rule_group(
        &mut self,
        rule_group_name: &str,
        rule_group_type: &str,
        capacity: i32,
        description: Option<&str>,
        tags: Vec<(String, String)>,
        rule_group_body: Option<serde_json::Value>,
        rules: Option<&str>,
        account_id: &str,
        region: &str,
    ) -> Result<&RuleGroup, NfwError> {
        if self
            .rule_groups
            .values()
            .any(|rg| rg.rule_group_name == rule_group_name)
        {
            return Err(NfwError::InvalidRequestDuplicateName {
                name: rule_group_name.to_string(),
            });
        }

        let rule_group_id = uuid::Uuid::new_v4().to_string();
        let arn = format!(
            "arn:aws:network-firewall:{region}:{account_id}:stateful-rulegroup/{rule_group_name}"
        );

        let rg = RuleGroup {
            rule_group_name: rule_group_name.to_string(),
            rule_group_arn: arn.clone(),
            rule_group_id,
            rule_group_type: rule_group_type.to_string(),
            capacity,
            description: description.map(|s| s.to_string()),
            tags,
            rule_group_body,
            rules: rules.map(|s| s.to_string()),
            encryption_configuration: None,
        };

        self.rule_groups.insert(arn.clone(), rg);
        Ok(self.rule_groups.get(&arn).unwrap())
    }

    pub fn describe_rule_group(
        &self,
        rule_group_name: Option<&str>,
        rule_group_arn: Option<&str>,
    ) -> Result<&RuleGroup, NfwError> {
        let rg = if let Some(arn) = rule_group_arn {
            self.rule_groups.get(arn)
        } else if let Some(name) = rule_group_name {
            self.rule_groups
                .values()
                .find(|rg| rg.rule_group_name == name)
        } else {
            return Err(NfwError::InvalidRequestRuleGroupIdentifier);
        };

        match rg {
            Some(rg) => Ok(rg),
            None => Err(resource_not_found_error(
                rule_group_arn.or(rule_group_name).unwrap_or("unknown"),
            )),
        }
    }

    pub fn delete_rule_group(
        &mut self,
        rule_group_name: Option<&str>,
        rule_group_arn: Option<&str>,
    ) -> Result<RuleGroup, NfwError> {
        let arn = if let Some(arn) = rule_group_arn {
            arn.to_string()
        } else if let Some(name) = rule_group_name {
            match self
                .rule_groups
                .values()
                .find(|rg| rg.rule_group_name == name)
            {
                Some(rg) => rg.rule_group_arn.clone(),
                None => return Err(resource_not_found_error(name)),
            }
        } else {
            return Err(NfwError::InvalidRequestRuleGroupIdentifier);
        };

        match self.rule_groups.remove(&arn) {
            Some(rg) => Ok(rg),
            None => Err(resource_not_found_error(&arn)),
        }
    }

    pub fn list_rule_groups(&self) -> Vec<RuleGroupMetadata> {
        self.rule_groups
            .values()
            .map(|rg| RuleGroupMetadata {
                name: rg.rule_group_name.clone(),
                arn: rg.rule_group_arn.clone(),
            })
            .collect()
    }

    pub fn update_rule_group(
        &mut self,
        rule_group_name: Option<&str>,
        rule_group_arn: Option<&str>,
        description: Option<&str>,
        rule_group_body: Option<serde_json::Value>,
        rules: Option<&str>,
    ) -> Result<&RuleGroup, NfwError> {
        let arn = if let Some(arn) = rule_group_arn {
            arn.to_string()
        } else if let Some(name) = rule_group_name {
            match self
                .rule_groups
                .values()
                .find(|rg| rg.rule_group_name == name)
            {
                Some(rg) => rg.rule_group_arn.clone(),
                None => return Err(resource_not_found_error(name)),
            }
        } else {
            return Err(NfwError::InvalidRequestRuleGroupIdentifier);
        };

        let rg = match self.rule_groups.get_mut(&arn) {
            Some(rg) => rg,
            None => return Err(resource_not_found_error(&arn)),
        };

        if let Some(desc) = description {
            rg.description = Some(desc.to_string());
        }
        if let Some(body) = rule_group_body {
            rg.rule_group_body = Some(body);
        }
        if let Some(r) = rules {
            rg.rules = Some(r.to_string());
        }

        Ok(self.rule_groups.get(&arn).unwrap())
    }

    // ---------- Firewall Policies ----------

    pub fn create_firewall_policy(
        &mut self,
        firewall_policy_name: &str,
        firewall_policy_body: serde_json::Value,
        description: Option<&str>,
        tags: Vec<(String, String)>,
        account_id: &str,
        region: &str,
    ) -> Result<&FirewallPolicy, NfwError> {
        if self
            .firewall_policies
            .values()
            .any(|fp| fp.firewall_policy_name == firewall_policy_name)
        {
            return Err(NfwError::InvalidRequestDuplicateName {
                name: firewall_policy_name.to_string(),
            });
        }

        let firewall_policy_id = uuid::Uuid::new_v4().to_string();
        let arn = format!(
            "arn:aws:network-firewall:{region}:{account_id}:firewall-policy/{firewall_policy_name}"
        );

        let fp = FirewallPolicy {
            firewall_policy_name: firewall_policy_name.to_string(),
            firewall_policy_arn: arn.clone(),
            firewall_policy_id,
            description: description.map(|s| s.to_string()),
            tags,
            firewall_policy_body,
            encryption_configuration: None,
        };

        self.firewall_policies.insert(arn.clone(), fp);
        Ok(self.firewall_policies.get(&arn).unwrap())
    }

    pub fn describe_firewall_policy(
        &self,
        firewall_policy_name: Option<&str>,
        firewall_policy_arn: Option<&str>,
    ) -> Result<&FirewallPolicy, NfwError> {
        let fp = if let Some(arn) = firewall_policy_arn {
            self.firewall_policies.get(arn)
        } else if let Some(name) = firewall_policy_name {
            self.firewall_policies
                .values()
                .find(|fp| fp.firewall_policy_name == name)
        } else {
            return Err(NfwError::InvalidRequestFirewallPolicyIdentifier);
        };

        match fp {
            Some(fp) => Ok(fp),
            None => Err(resource_not_found_error(
                firewall_policy_arn
                    .or(firewall_policy_name)
                    .unwrap_or("unknown"),
            )),
        }
    }

    pub fn delete_firewall_policy(
        &mut self,
        firewall_policy_name: Option<&str>,
        firewall_policy_arn: Option<&str>,
    ) -> Result<FirewallPolicy, NfwError> {
        let arn = if let Some(arn) = firewall_policy_arn {
            arn.to_string()
        } else if let Some(name) = firewall_policy_name {
            match self
                .firewall_policies
                .values()
                .find(|fp| fp.firewall_policy_name == name)
            {
                Some(fp) => fp.firewall_policy_arn.clone(),
                None => return Err(resource_not_found_error(name)),
            }
        } else {
            return Err(NfwError::InvalidRequestFirewallPolicyIdentifier);
        };

        match self.firewall_policies.remove(&arn) {
            Some(fp) => Ok(fp),
            None => Err(resource_not_found_error(&arn)),
        }
    }

    pub fn list_firewall_policies(&self) -> Vec<FirewallPolicyMetadata> {
        self.firewall_policies
            .values()
            .map(|fp| FirewallPolicyMetadata {
                name: fp.firewall_policy_name.clone(),
                arn: fp.firewall_policy_arn.clone(),
            })
            .collect()
    }

    pub fn update_firewall_policy(
        &mut self,
        firewall_policy_name: Option<&str>,
        firewall_policy_arn: Option<&str>,
        firewall_policy_body: serde_json::Value,
        description: Option<&str>,
    ) -> Result<&FirewallPolicy, NfwError> {
        let arn = if let Some(arn) = firewall_policy_arn {
            arn.to_string()
        } else if let Some(name) = firewall_policy_name {
            match self
                .firewall_policies
                .values()
                .find(|fp| fp.firewall_policy_name == name)
            {
                Some(fp) => fp.firewall_policy_arn.clone(),
                None => return Err(resource_not_found_error(name)),
            }
        } else {
            return Err(NfwError::InvalidRequestFirewallPolicyIdentifier);
        };

        let fp = match self.firewall_policies.get_mut(&arn) {
            Some(fp) => fp,
            None => return Err(resource_not_found_error(&arn)),
        };

        fp.firewall_policy_body = firewall_policy_body;
        if let Some(desc) = description {
            fp.description = Some(desc.to_string());
        }

        Ok(self.firewall_policies.get(&arn).unwrap())
    }

    // ---------- Tags ----------

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<Vec<(String, String)>, NfwError> {
        if let Some(fw) = self.firewalls.get(resource_arn) {
            return Ok(fw.tags.clone());
        }
        if let Some(rg) = self.rule_groups.get(resource_arn) {
            return Ok(rg.tags.clone());
        }
        if let Some(fp) = self.firewall_policies.get(resource_arn) {
            return Ok(fp.tags.clone());
        }
        Err(resource_not_found_error(resource_arn))
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: Vec<(String, String)>,
    ) -> Result<(), NfwError> {
        if let Some(fw) = self.firewalls.get_mut(resource_arn) {
            for (k, v) in tags {
                fw.tags.retain(|(ek, _)| *ek != k);
                fw.tags.push((k, v));
            }
            return Ok(());
        }
        if let Some(rg) = self.rule_groups.get_mut(resource_arn) {
            for (k, v) in tags {
                rg.tags.retain(|(ek, _)| *ek != k);
                rg.tags.push((k, v));
            }
            return Ok(());
        }
        if let Some(fp) = self.firewall_policies.get_mut(resource_arn) {
            for (k, v) in tags {
                fp.tags.retain(|(ek, _)| *ek != k);
                fp.tags.push((k, v));
            }
            return Ok(());
        }
        Err(resource_not_found_error(resource_arn))
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: Vec<&str>,
    ) -> Result<(), NfwError> {
        if let Some(fw) = self.firewalls.get_mut(resource_arn) {
            fw.tags.retain(|(k, _)| !tag_keys.contains(&k.as_str()));
            return Ok(());
        }
        if let Some(rg) = self.rule_groups.get_mut(resource_arn) {
            rg.tags.retain(|(k, _)| !tag_keys.contains(&k.as_str()));
            return Ok(());
        }
        if let Some(fp) = self.firewall_policies.get_mut(resource_arn) {
            fp.tags.retain(|(k, _)| !tag_keys.contains(&k.as_str()));
            return Ok(());
        }
        Err(resource_not_found_error(resource_arn))
    }

    // ---------- Resource Policies ----------

    pub fn put_resource_policy(
        &mut self,
        resource_arn: &str,
        policy: &str,
    ) -> Result<(), NfwError> {
        self.resource_policies.insert(
            resource_arn.to_string(),
            ResourcePolicy {
                resource_arn: resource_arn.to_string(),
                policy: policy.to_string(),
            },
        );
        Ok(())
    }

    pub fn describe_resource_policy(&self, resource_arn: &str) -> Result<&str, NfwError> {
        match self.resource_policies.get(resource_arn) {
            Some(rp) => Ok(&rp.policy),
            None => Err(resource_not_found_error(resource_arn)),
        }
    }

    pub fn delete_resource_policy(&mut self, resource_arn: &str) -> Result<(), NfwError> {
        match self.resource_policies.remove(resource_arn) {
            Some(_) => Ok(()),
            None => Err(resource_not_found_error(resource_arn)),
        }
    }

    // ---------- Firewall update operations ----------

    pub fn update_firewall_description(
        &mut self,
        firewall_name: Option<&str>,
        firewall_arn: Option<&str>,
        description: Option<&str>,
    ) -> Result<(&Firewall, FirewallStatus), NfwError> {
        let arn = self.resolve_firewall_arn(firewall_name, firewall_arn)?;
        let fw = self.firewalls.get_mut(&arn).unwrap();
        fw.description = description.map(|s| s.to_string());
        Ok((self.firewalls.get(&arn).unwrap(), FirewallStatus::default()))
    }

    pub fn update_firewall_delete_protection(
        &mut self,
        firewall_name: Option<&str>,
        firewall_arn: Option<&str>,
        delete_protection: bool,
    ) -> Result<(&Firewall, FirewallStatus), NfwError> {
        let arn = self.resolve_firewall_arn(firewall_name, firewall_arn)?;
        let fw = self.firewalls.get_mut(&arn).unwrap();
        fw.delete_protection = delete_protection;
        Ok((self.firewalls.get(&arn).unwrap(), FirewallStatus::default()))
    }

    pub fn associate_firewall_policy(
        &mut self,
        firewall_name: Option<&str>,
        firewall_arn: Option<&str>,
        firewall_policy_arn: &str,
    ) -> Result<(&Firewall, FirewallStatus), NfwError> {
        let arn = self.resolve_firewall_arn(firewall_name, firewall_arn)?;
        let fw = self.firewalls.get_mut(&arn).unwrap();
        fw.firewall_policy_arn = firewall_policy_arn.to_string();
        Ok((self.firewalls.get(&arn).unwrap(), FirewallStatus::default()))
    }

    pub fn associate_subnets(
        &mut self,
        firewall_name: Option<&str>,
        firewall_arn: Option<&str>,
        subnet_mappings: Vec<SubnetMapping>,
    ) -> Result<(&Firewall, FirewallStatus), NfwError> {
        let arn = self.resolve_firewall_arn(firewall_name, firewall_arn)?;
        let fw = self.firewalls.get_mut(&arn).unwrap();
        for sm in subnet_mappings {
            if !fw
                .subnet_mappings
                .iter()
                .any(|e| e.subnet_id == sm.subnet_id)
            {
                fw.subnet_mappings.push(sm);
            }
        }
        Ok((self.firewalls.get(&arn).unwrap(), FirewallStatus::default()))
    }

    pub fn disassociate_subnets(
        &mut self,
        firewall_name: Option<&str>,
        firewall_arn: Option<&str>,
        subnet_ids: Vec<&str>,
    ) -> Result<(&Firewall, FirewallStatus), NfwError> {
        let arn = self.resolve_firewall_arn(firewall_name, firewall_arn)?;
        let fw = self.firewalls.get_mut(&arn).unwrap();
        fw.subnet_mappings
            .retain(|sm| !subnet_ids.contains(&sm.subnet_id.as_str()));
        Ok((self.firewalls.get(&arn).unwrap(), FirewallStatus::default()))
    }

    pub fn update_subnet_change_protection(
        &mut self,
        firewall_name: Option<&str>,
        firewall_arn: Option<&str>,
        subnet_change_protection: bool,
    ) -> Result<(&Firewall, FirewallStatus), NfwError> {
        let arn = self.resolve_firewall_arn(firewall_name, firewall_arn)?;
        let fw = self.firewalls.get_mut(&arn).unwrap();
        fw.subnet_change_protection = subnet_change_protection;
        Ok((self.firewalls.get(&arn).unwrap(), FirewallStatus::default()))
    }

    pub fn update_firewall_policy_change_protection(
        &mut self,
        firewall_name: Option<&str>,
        firewall_arn: Option<&str>,
        firewall_policy_change_protection: bool,
    ) -> Result<(&Firewall, FirewallStatus), NfwError> {
        let arn = self.resolve_firewall_arn(firewall_name, firewall_arn)?;
        let fw = self.firewalls.get_mut(&arn).unwrap();
        fw.firewall_policy_change_protection = firewall_policy_change_protection;
        Ok((self.firewalls.get(&arn).unwrap(), FirewallStatus::default()))
    }

    pub fn update_availability_zone_change_protection(
        &mut self,
        firewall_name: Option<&str>,
        firewall_arn: Option<&str>,
        availability_zone_change_protection: bool,
    ) -> Result<(&Firewall, FirewallStatus), NfwError> {
        let arn = self.resolve_firewall_arn(firewall_name, firewall_arn)?;
        let fw = self.firewalls.get_mut(&arn).unwrap();
        fw.availability_zone_change_protection = availability_zone_change_protection;
        Ok((self.firewalls.get(&arn).unwrap(), FirewallStatus::default()))
    }

    // ---------- TLS Inspection Configurations ----------

    pub fn create_tls_inspection_configuration(
        &mut self,
        name: &str,
        body: serde_json::Value,
        description: Option<&str>,
        tags: Vec<(String, String)>,
        account_id: &str,
        region: &str,
    ) -> Result<&TlsInspectionConfiguration, NfwError> {
        if self
            .tls_inspection_configurations
            .values()
            .any(|t| t.name == name)
        {
            return Err(NfwError::InvalidRequestDuplicateName {
                name: name.to_string(),
            });
        }

        let id = uuid::Uuid::new_v4().to_string();
        let arn =
            format!("arn:aws:network-firewall:{region}:{account_id}:tls-configuration/{name}");

        let tls = TlsInspectionConfiguration {
            name: name.to_string(),
            arn: arn.clone(),
            id,
            description: description.map(|s| s.to_string()),
            tags,
            body,
        };

        self.tls_inspection_configurations.insert(arn.clone(), tls);
        Ok(self.tls_inspection_configurations.get(&arn).unwrap())
    }

    pub fn describe_tls_inspection_configuration(
        &self,
        name: Option<&str>,
        arn: Option<&str>,
    ) -> Result<&TlsInspectionConfiguration, NfwError> {
        let tls = if let Some(a) = arn {
            self.tls_inspection_configurations.get(a)
        } else if let Some(n) = name {
            self.tls_inspection_configurations
                .values()
                .find(|t| t.name == n)
        } else {
            return Err(NfwError::InvalidRequestTlsInspectionConfigIdentifier);
        };

        match tls {
            Some(t) => Ok(t),
            None => Err(resource_not_found_error(arn.or(name).unwrap_or("unknown"))),
        }
    }

    pub fn delete_tls_inspection_configuration(
        &mut self,
        name: Option<&str>,
        arn: Option<&str>,
    ) -> Result<TlsInspectionConfiguration, NfwError> {
        let resolved_arn = if let Some(a) = arn {
            a.to_string()
        } else if let Some(n) = name {
            match self
                .tls_inspection_configurations
                .values()
                .find(|t| t.name == n)
            {
                Some(t) => t.arn.clone(),
                None => return Err(resource_not_found_error(n)),
            }
        } else {
            return Err(NfwError::InvalidRequestTlsInspectionConfigIdentifier);
        };

        match self.tls_inspection_configurations.remove(&resolved_arn) {
            Some(t) => Ok(t),
            None => Err(resource_not_found_error(&resolved_arn)),
        }
    }

    pub fn update_tls_inspection_configuration(
        &mut self,
        name: Option<&str>,
        arn: Option<&str>,
        body: serde_json::Value,
        description: Option<&str>,
    ) -> Result<&TlsInspectionConfiguration, NfwError> {
        let resolved_arn = if let Some(a) = arn {
            a.to_string()
        } else if let Some(n) = name {
            match self
                .tls_inspection_configurations
                .values()
                .find(|t| t.name == n)
            {
                Some(t) => t.arn.clone(),
                None => return Err(resource_not_found_error(n)),
            }
        } else {
            return Err(NfwError::InvalidRequestTlsInspectionConfigIdentifier);
        };

        let tls = match self.tls_inspection_configurations.get_mut(&resolved_arn) {
            Some(t) => t,
            None => return Err(resource_not_found_error(&resolved_arn)),
        };

        tls.body = body;
        if let Some(desc) = description {
            tls.description = Some(desc.to_string());
        }

        Ok(self
            .tls_inspection_configurations
            .get(&resolved_arn)
            .unwrap())
    }

    pub fn list_tls_inspection_configurations(&self) -> Vec<TlsInspectionConfigurationMetadata> {
        self.tls_inspection_configurations
            .values()
            .map(|t| TlsInspectionConfigurationMetadata {
                name: t.name.clone(),
                arn: t.arn.clone(),
            })
            .collect()
    }

    // ---------- VPC Endpoint Associations ----------

    pub fn create_vpc_endpoint_association(
        &mut self,
        firewall_arn: &str,
        vpc_id: &str,
        subnet_id: &str,
        description: Option<&str>,
        tags: Vec<(String, String)>,
        account_id: &str,
        region: &str,
    ) -> Result<&VpcEndpointAssociation, NfwError> {
        let id = uuid::Uuid::new_v4().to_string();
        let arn =
            format!("arn:aws:network-firewall:{region}:{account_id}:vpc-endpoint-association/{id}");

        let assoc = VpcEndpointAssociation {
            vpc_endpoint_association_arn: arn.clone(),
            vpc_endpoint_association_id: id,
            firewall_arn: firewall_arn.to_string(),
            vpc_id: vpc_id.to_string(),
            subnet_id: subnet_id.to_string(),
            description: description.map(|s| s.to_string()),
            tags,
        };

        self.vpc_endpoint_associations.insert(arn.clone(), assoc);
        Ok(self.vpc_endpoint_associations.get(&arn).unwrap())
    }

    pub fn describe_vpc_endpoint_association(
        &self,
        arn: &str,
    ) -> Result<&VpcEndpointAssociation, NfwError> {
        match self.vpc_endpoint_associations.get(arn) {
            Some(a) => Ok(a),
            None => Err(resource_not_found_error(arn)),
        }
    }

    pub fn delete_vpc_endpoint_association(
        &mut self,
        arn: &str,
    ) -> Result<VpcEndpointAssociation, NfwError> {
        match self.vpc_endpoint_associations.remove(arn) {
            Some(a) => Ok(a),
            None => Err(resource_not_found_error(arn)),
        }
    }

    pub fn list_vpc_endpoint_associations(
        &self,
        firewall_arn: Option<&str>,
    ) -> Vec<&VpcEndpointAssociation> {
        self.vpc_endpoint_associations
            .values()
            .filter(|a| {
                if let Some(fw_arn) = firewall_arn {
                    a.firewall_arn == fw_arn
                } else {
                    true
                }
            })
            .collect()
    }

    // ---------- Availability Zone Mappings ----------

    pub fn associate_availability_zones(
        &mut self,
        firewall_name: Option<&str>,
        firewall_arn: Option<&str>,
        zones: Vec<AvailabilityZoneMapping>,
    ) -> Result<(&Firewall, Vec<AvailabilityZoneMapping>), NfwError> {
        let arn = self.resolve_firewall_arn(firewall_name, firewall_arn)?;
        let mappings = self
            .availability_zone_mappings
            .entry(arn.clone())
            .or_default();
        for zone in zones {
            if !mappings
                .iter()
                .any(|m| m.availability_zone == zone.availability_zone)
            {
                mappings.push(zone);
            }
        }
        let result_mappings = self.availability_zone_mappings.get(&arn).unwrap().clone();
        Ok((self.firewalls.get(&arn).unwrap(), result_mappings))
    }

    pub fn disassociate_availability_zones(
        &mut self,
        firewall_name: Option<&str>,
        firewall_arn: Option<&str>,
        zones: Vec<&str>,
    ) -> Result<(&Firewall, Vec<AvailabilityZoneMapping>), NfwError> {
        let arn = self.resolve_firewall_arn(firewall_name, firewall_arn)?;
        let mappings = self
            .availability_zone_mappings
            .entry(arn.clone())
            .or_default();
        mappings.retain(|m| !zones.contains(&m.availability_zone.as_str()));
        let result_mappings = self.availability_zone_mappings.get(&arn).unwrap().clone();
        Ok((self.firewalls.get(&arn).unwrap(), result_mappings))
    }

    // ---------- Transit Gateway Attachments ----------

    pub fn accept_transit_gateway_attachment(
        &mut self,
        transit_gateway_attachment_id: &str,
    ) -> Result<&TransitGatewayAttachment, NfwError> {
        let attachment = self
            .transit_gateway_attachments
            .get_mut(transit_gateway_attachment_id);
        match attachment {
            Some(a) => {
                a.status = "ACCEPTED".to_string();
                Ok(self
                    .transit_gateway_attachments
                    .get(transit_gateway_attachment_id)
                    .unwrap())
            }
            None => {
                // Auto-create it in accepted state (mock behaviour)
                let att = TransitGatewayAttachment {
                    transit_gateway_attachment_id: transit_gateway_attachment_id.to_string(),
                    status: "ACCEPTED".to_string(),
                };
                self.transit_gateway_attachments
                    .insert(transit_gateway_attachment_id.to_string(), att);
                Ok(self
                    .transit_gateway_attachments
                    .get(transit_gateway_attachment_id)
                    .unwrap())
            }
        }
    }

    pub fn reject_transit_gateway_attachment(
        &mut self,
        transit_gateway_attachment_id: &str,
    ) -> Result<&TransitGatewayAttachment, NfwError> {
        let attachment = self
            .transit_gateway_attachments
            .get_mut(transit_gateway_attachment_id);
        match attachment {
            Some(a) => {
                a.status = "REJECTED".to_string();
                Ok(self
                    .transit_gateway_attachments
                    .get(transit_gateway_attachment_id)
                    .unwrap())
            }
            None => {
                let att = TransitGatewayAttachment {
                    transit_gateway_attachment_id: transit_gateway_attachment_id.to_string(),
                    status: "REJECTED".to_string(),
                };
                self.transit_gateway_attachments
                    .insert(transit_gateway_attachment_id.to_string(), att);
                Ok(self
                    .transit_gateway_attachments
                    .get(transit_gateway_attachment_id)
                    .unwrap())
            }
        }
    }

    pub fn delete_transit_gateway_attachment(
        &mut self,
        transit_gateway_attachment_id: &str,
    ) -> Result<TransitGatewayAttachment, NfwError> {
        match self
            .transit_gateway_attachments
            .remove(transit_gateway_attachment_id)
        {
            Some(mut a) => {
                a.status = "DELETING".to_string();
                Ok(a)
            }
            None => Err(resource_not_found_error(transit_gateway_attachment_id)),
        }
    }

    // ---------- Proxies ----------

    pub fn create_proxy(
        &mut self,
        proxy_name: &str,
        nat_gateway_id: &str,
        proxy_configuration_arn: Option<&str>,
        proxy_configuration_name: Option<&str>,
        tags: Vec<(String, String)>,
        body: serde_json::Value,
        account_id: &str,
        region: &str,
    ) -> Result<&NfwProxy, NfwError> {
        if self.proxies.values().any(|p| p.proxy_name == proxy_name) {
            return Err(NfwError::InvalidRequestDuplicateName {
                name: proxy_name.to_string(),
            });
        }
        let arn = format!("arn:aws:network-firewall:{region}:{account_id}:proxy/{proxy_name}");
        let proxy = NfwProxy {
            proxy_name: proxy_name.to_string(),
            proxy_arn: arn.clone(),
            nat_gateway_id: nat_gateway_id.to_string(),
            proxy_configuration_arn: proxy_configuration_arn.map(|s| s.to_string()),
            proxy_configuration_name: proxy_configuration_name.map(|s| s.to_string()),
            proxy_state: "READY".to_string(),
            tags,
            body,
        };
        self.proxies.insert(arn.clone(), proxy);
        Ok(self.proxies.get(&arn).unwrap())
    }

    pub fn describe_proxy(
        &self,
        proxy_name: Option<&str>,
        proxy_arn: Option<&str>,
    ) -> Result<&NfwProxy, NfwError> {
        let proxy = if let Some(arn) = proxy_arn {
            self.proxies.get(arn)
        } else if let Some(name) = proxy_name {
            self.proxies.values().find(|p| p.proxy_name == name)
        } else {
            return Err(NfwError::InvalidRequestProxyIdentifier);
        };
        match proxy {
            Some(p) => Ok(p),
            None => Err(resource_not_found_error(
                proxy_arn.or(proxy_name).unwrap_or("unknown"),
            )),
        }
    }

    pub fn delete_proxy(
        &mut self,
        proxy_name: Option<&str>,
        proxy_arn: Option<&str>,
    ) -> Result<NfwProxy, NfwError> {
        let arn = if let Some(arn) = proxy_arn {
            arn.to_string()
        } else if let Some(name) = proxy_name {
            match self.proxies.values().find(|p| p.proxy_name == name) {
                Some(p) => p.proxy_arn.clone(),
                None => return Err(resource_not_found_error(name)),
            }
        } else {
            return Err(NfwError::InvalidRequestProxyIdentifier);
        };
        match self.proxies.remove(&arn) {
            Some(p) => Ok(p),
            None => Err(resource_not_found_error(&arn)),
        }
    }

    pub fn list_proxies(&self) -> Vec<&NfwProxy> {
        self.proxies.values().collect()
    }

    pub fn update_proxy(
        &mut self,
        proxy_name: Option<&str>,
        proxy_arn: Option<&str>,
        body: serde_json::Value,
    ) -> Result<&NfwProxy, NfwError> {
        let arn = if let Some(arn) = proxy_arn {
            arn.to_string()
        } else if let Some(name) = proxy_name {
            match self.proxies.values().find(|p| p.proxy_name == name) {
                Some(p) => p.proxy_arn.clone(),
                None => return Err(resource_not_found_error(name)),
            }
        } else {
            return Err(NfwError::InvalidRequestProxyIdentifier);
        };
        let proxy = match self.proxies.get_mut(&arn) {
            Some(p) => p,
            None => return Err(resource_not_found_error(&arn)),
        };
        proxy.body = body;
        Ok(self.proxies.get(&arn).unwrap())
    }

    // ---------- Proxy Configurations ----------

    pub fn create_proxy_configuration(
        &mut self,
        name: &str,
        description: Option<&str>,
        tags: Vec<(String, String)>,
        body: serde_json::Value,
        account_id: &str,
        region: &str,
    ) -> Result<&NfwProxyConfiguration, NfwError> {
        if self
            .proxy_configurations
            .values()
            .any(|c| c.proxy_configuration_name == name)
        {
            return Err(NfwError::InvalidRequestDuplicateName {
                name: name.to_string(),
            });
        }
        let arn =
            format!("arn:aws:network-firewall:{region}:{account_id}:proxy-configuration/{name}");
        let config = NfwProxyConfiguration {
            proxy_configuration_name: name.to_string(),
            proxy_configuration_arn: arn.clone(),
            description: description.map(|s| s.to_string()),
            tags,
            body,
        };
        self.proxy_configurations.insert(arn.clone(), config);
        Ok(self.proxy_configurations.get(&arn).unwrap())
    }

    pub fn describe_proxy_configuration(
        &self,
        name: Option<&str>,
        arn: Option<&str>,
    ) -> Result<&NfwProxyConfiguration, NfwError> {
        let config = if let Some(a) = arn {
            self.proxy_configurations.get(a)
        } else if let Some(n) = name {
            self.proxy_configurations
                .values()
                .find(|c| c.proxy_configuration_name == n)
        } else {
            return Err(NfwError::InvalidRequestProxyConfigIdentifier);
        };
        match config {
            Some(c) => Ok(c),
            None => Err(resource_not_found_error(arn.or(name).unwrap_or("unknown"))),
        }
    }

    pub fn delete_proxy_configuration(
        &mut self,
        name: Option<&str>,
        arn: Option<&str>,
    ) -> Result<NfwProxyConfiguration, NfwError> {
        let resolved_arn = if let Some(a) = arn {
            a.to_string()
        } else if let Some(n) = name {
            match self
                .proxy_configurations
                .values()
                .find(|c| c.proxy_configuration_name == n)
            {
                Some(c) => c.proxy_configuration_arn.clone(),
                None => return Err(resource_not_found_error(n)),
            }
        } else {
            return Err(NfwError::InvalidRequestProxyConfigIdentifier);
        };
        match self.proxy_configurations.remove(&resolved_arn) {
            Some(c) => Ok(c),
            None => Err(resource_not_found_error(&resolved_arn)),
        }
    }

    pub fn list_proxy_configurations(&self) -> Vec<&NfwProxyConfiguration> {
        self.proxy_configurations.values().collect()
    }

    pub fn update_proxy_configuration(
        &mut self,
        name: Option<&str>,
        arn: Option<&str>,
        body: serde_json::Value,
    ) -> Result<&NfwProxyConfiguration, NfwError> {
        let resolved_arn = if let Some(a) = arn {
            a.to_string()
        } else if let Some(n) = name {
            match self
                .proxy_configurations
                .values()
                .find(|c| c.proxy_configuration_name == n)
            {
                Some(c) => c.proxy_configuration_arn.clone(),
                None => return Err(resource_not_found_error(n)),
            }
        } else {
            return Err(NfwError::InvalidRequestProxyConfigIdentifier);
        };
        let config = match self.proxy_configurations.get_mut(&resolved_arn) {
            Some(c) => c,
            None => return Err(resource_not_found_error(&resolved_arn)),
        };
        config.body = body;
        Ok(self.proxy_configurations.get(&resolved_arn).unwrap())
    }

    // ---------- Proxy Rule Groups ----------

    pub fn create_proxy_rule_group(
        &mut self,
        name: &str,
        description: Option<&str>,
        tags: Vec<(String, String)>,
        body: serde_json::Value,
        account_id: &str,
        region: &str,
    ) -> Result<&NfwProxyRuleGroup, NfwError> {
        if self
            .proxy_rule_groups
            .values()
            .any(|g| g.proxy_rule_group_name == name)
        {
            return Err(NfwError::InvalidRequestDuplicateName {
                name: name.to_string(),
            });
        }
        let arn = format!("arn:aws:network-firewall:{region}:{account_id}:proxy-rule-group/{name}");
        let group = NfwProxyRuleGroup {
            proxy_rule_group_name: name.to_string(),
            proxy_rule_group_arn: arn.clone(),
            description: description.map(|s| s.to_string()),
            tags,
            body,
        };
        self.proxy_rule_groups.insert(arn.clone(), group);
        Ok(self.proxy_rule_groups.get(&arn).unwrap())
    }

    pub fn describe_proxy_rule_group(
        &self,
        name: Option<&str>,
        arn: Option<&str>,
    ) -> Result<&NfwProxyRuleGroup, NfwError> {
        let group = if let Some(a) = arn {
            self.proxy_rule_groups.get(a)
        } else if let Some(n) = name {
            self.proxy_rule_groups
                .values()
                .find(|g| g.proxy_rule_group_name == n)
        } else {
            return Err(NfwError::InvalidRequestProxyRuleGroupIdentifier);
        };
        match group {
            Some(g) => Ok(g),
            None => Err(resource_not_found_error(arn.or(name).unwrap_or("unknown"))),
        }
    }

    pub fn delete_proxy_rule_group(
        &mut self,
        name: Option<&str>,
        arn: Option<&str>,
    ) -> Result<NfwProxyRuleGroup, NfwError> {
        let resolved_arn = if let Some(a) = arn {
            a.to_string()
        } else if let Some(n) = name {
            match self
                .proxy_rule_groups
                .values()
                .find(|g| g.proxy_rule_group_name == n)
            {
                Some(g) => g.proxy_rule_group_arn.clone(),
                None => return Err(resource_not_found_error(n)),
            }
        } else {
            return Err(NfwError::InvalidRequestProxyRuleGroupIdentifier);
        };
        match self.proxy_rule_groups.remove(&resolved_arn) {
            Some(g) => Ok(g),
            None => Err(resource_not_found_error(&resolved_arn)),
        }
    }

    pub fn list_proxy_rule_groups(&self) -> Vec<&NfwProxyRuleGroup> {
        self.proxy_rule_groups.values().collect()
    }

    // ---------- Flow Operations ----------

    pub fn create_flow_operation(
        &mut self,
        firewall_arn: &str,
        flow_operation_type: &str,
        body: serde_json::Value,
    ) -> &FlowOperation {
        let id = uuid::Uuid::new_v4().to_string();
        let op = FlowOperation {
            flow_operation_id: id.clone(),
            firewall_arn: firewall_arn.to_string(),
            flow_operation_type: flow_operation_type.to_string(),
            flow_operation_status: "COMPLETED".to_string(),
            body,
        };
        self.flow_operations.insert(id.clone(), op);
        self.flow_operations.get(&id).unwrap()
    }

    pub fn describe_flow_operation(
        &self,
        flow_operation_id: &str,
    ) -> Result<&FlowOperation, NfwError> {
        match self.flow_operations.get(flow_operation_id) {
            Some(op) => Ok(op),
            None => Err(resource_not_found_error(flow_operation_id)),
        }
    }

    pub fn list_flow_operations(&self, firewall_arn: &str) -> Vec<&FlowOperation> {
        self.flow_operations
            .values()
            .filter(|op| op.firewall_arn == firewall_arn)
            .collect()
    }

    // ---------- Analysis Reports ----------

    pub fn start_analysis_report(
        &mut self,
        firewall_arn: &str,
        analysis_type: &str,
    ) -> &AnalysisReport {
        let id = uuid::Uuid::new_v4().to_string();
        let report = AnalysisReport {
            analysis_report_id: id.clone(),
            firewall_arn: firewall_arn.to_string(),
            analysis_type: analysis_type.to_string(),
            status: "COMPLETED".to_string(),
        };
        self.analysis_reports.insert(id.clone(), report);
        self.analysis_reports.get(&id).unwrap()
    }

    pub fn list_analysis_reports(&self, firewall_arn: &str) -> Vec<&AnalysisReport> {
        self.analysis_reports
            .values()
            .filter(|r| r.firewall_arn == firewall_arn)
            .collect()
    }

    pub fn get_analysis_report(
        &self,
        analysis_report_id: &str,
    ) -> Result<&AnalysisReport, NfwError> {
        match self.analysis_reports.get(analysis_report_id) {
            Some(r) => Ok(r),
            None => Err(resource_not_found_error(analysis_report_id)),
        }
    }

    // ---------- Firewall Analysis Settings ----------

    pub fn update_analysis_settings(
        &mut self,
        firewall_name: Option<&str>,
        firewall_arn: Option<&str>,
        enabled_analysis_types: Vec<String>,
    ) -> Result<(&Firewall, Vec<String>), NfwError> {
        let arn = self.resolve_firewall_arn(firewall_name, firewall_arn)?;
        self.analysis_settings
            .insert(arn.clone(), enabled_analysis_types);
        let types = self.analysis_settings.get(&arn).unwrap().clone();
        Ok((self.firewalls.get(&arn).unwrap(), types))
    }

    // ---------- Encryption Configuration ----------

    pub fn update_encryption_configuration(
        &mut self,
        firewall_name: Option<&str>,
        firewall_arn: Option<&str>,
        config: EncryptionConfig,
    ) -> Result<(&Firewall, EncryptionConfig), NfwError> {
        let arn = self.resolve_firewall_arn(firewall_name, firewall_arn)?;
        self.encryption_configs.insert(arn.clone(), config);
        let enc = self.encryption_configs.get(&arn).unwrap().clone();
        Ok((self.firewalls.get(&arn).unwrap(), enc))
    }

    fn resolve_firewall_arn(
        &self,
        firewall_name: Option<&str>,
        firewall_arn: Option<&str>,
    ) -> Result<String, NfwError> {
        if let Some(arn) = firewall_arn {
            if self.firewalls.contains_key(arn) {
                return Ok(arn.to_string());
            }
            return Err(resource_not_found_error(arn));
        }
        if let Some(name) = firewall_name {
            return match self.firewalls.values().find(|f| f.firewall_name == name) {
                Some(f) => Ok(f.firewall_arn.clone()),
                None => Err(resource_not_found_error(name)),
            };
        }
        Err(NfwError::InvalidRequestFirewallIdentifier)
    }
}

fn resource_not_found_error(identifier: &str) -> NfwError {
    NfwError::ResourceNotFound {
        identifier: identifier.to_string(),
    }
}
