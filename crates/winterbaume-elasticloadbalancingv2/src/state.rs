use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct ElbV2State {
    pub load_balancers: HashMap<String, LoadBalancer>,
    pub target_groups: HashMap<String, TargetGroup>,
    pub listeners: HashMap<String, Listener>,
    pub rules: HashMap<String, Rule>,
    pub resource_tags: HashMap<String, HashMap<String, String>>,
    pub trust_stores: HashMap<String, TrustStore>,
    /// Trust store associations: trust_store_arn -> list of associated resource ARNs.
    pub trust_store_associations: HashMap<String, Vec<TrustStoreAssociation>>,
    /// Capacity reservations keyed by load balancer ARN.
    pub capacity_reservations: HashMap<String, CapacityReservation>,
    /// Resource-level policies keyed by resource ARN.
    pub resource_policies: HashMap<String, ResourcePolicy>,
    /// IPAM pool assignments keyed by load balancer ARN.
    pub ipam_pools: HashMap<String, IpamPool>,
}

#[derive(Debug, thiserror::Error)]
pub enum ElbV2Error {
    #[error("A load balancer with the name '{0}' already exists.")]
    DuplicateLoadBalancerName(String),
    #[error("Load balancer '{0}' not found.")]
    LoadBalancerNotFound(String),
    #[error("Load balancers '[{0}]' not found")]
    LoadBalancerNotFoundList(String),
    #[error("A target group with the name '{0}' already exists.")]
    DuplicateTargetGroupName(String),
    #[error("Target group '{0}' not found.")]
    TargetGroupNotFound(String),
    #[error("Listener '{0}' not found.")]
    ListenerNotFound(String),
    #[error("One or more listeners not found")]
    ListenerNotFoundList,
    #[error("The specified priority is in use.")]
    PriorityInUse,
    #[error("Rule '{0}' not found.")]
    RuleNotFound(String),
    #[error("Default rules cannot be deleted.")]
    OperationNotPermitted,
    #[error("A trust store with the name '{0}' already exists.")]
    DuplicateTrustStoreName(String),
    #[error("Trust store '{0}' not found.")]
    TrustStoreNotFound(String),
    #[error("You must specify either listener ARNs or a load balancer ARN")]
    ValidationError,
}

impl ElbV2State {
    pub fn create_load_balancer(
        &mut self,
        name: &str,
        scheme: &str,
        lb_type: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&LoadBalancer, ElbV2Error> {
        // Check for duplicate name
        if self.load_balancers.values().any(|lb| lb.name == name) {
            return Err(ElbV2Error::DuplicateLoadBalancerName(name.to_string()));
        }

        let id = &uuid::Uuid::new_v4().to_string()[..17];
        let arn = format!(
            "arn:aws:elasticloadbalancing:{region}:{account_id}:loadbalancer/app/{name}/{id}"
        );
        let dns_name = format!("{name}-{}.{region}.elb.amazonaws.com", &id[..8]);

        let subnet_id = "subnet-aaaa1111".to_string();
        let lb = LoadBalancer {
            arn: arn.clone(),
            dns_name,
            name: name.to_string(),
            scheme: scheme.to_string(),
            state: "active".to_string(),
            lb_type: lb_type.to_string(),
            vpc_id: "vpc-12345678".to_string(),
            availability_zones: vec![AvailabilityZone {
                zone_name: format!("{region}a"),
                subnet_id: subnet_id.clone(),
            }],
            created_time: Utc::now(),
            attributes: HashMap::new(),
            ip_address_type: "ipv4".to_string(),
            security_groups: vec!["sg-12345678".to_string()],
            subnets: vec![subnet_id],
        };

        self.load_balancers.insert(arn.clone(), lb);
        Ok(self.load_balancers.get(&arn).unwrap())
    }

    pub fn delete_load_balancer(&mut self, arn: &str) -> Result<(), ElbV2Error> {
        if self.load_balancers.remove(arn).is_none() {
            return Err(ElbV2Error::LoadBalancerNotFound(arn.to_string()));
        }
        // Also remove any listeners for this LB
        let listener_arns: Vec<String> = self
            .listeners
            .iter()
            .filter(|(_, l)| l.load_balancer_arn == arn)
            .map(|(k, _)| k.clone())
            .collect();
        for la in &listener_arns {
            self.listeners.remove(la);
            // Remove rules for those listeners
            self.rules.retain(|_, r| r.listener_arn != *la);
        }
        self.resource_tags.remove(arn);
        Ok(())
    }

    pub fn describe_load_balancers(&self) -> Vec<&LoadBalancer> {
        self.load_balancers.values().collect()
    }

    pub fn create_target_group(
        &mut self,
        name: &str,
        protocol: &str,
        port: i32,
        vpc_id: &str,
        target_type: &str,
        health_check_path: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&TargetGroup, ElbV2Error> {
        if self.target_groups.values().any(|tg| tg.name == name) {
            return Err(ElbV2Error::DuplicateTargetGroupName(name.to_string()));
        }

        let id = &uuid::Uuid::new_v4().to_string()[..17];
        let arn =
            format!("arn:aws:elasticloadbalancing:{region}:{account_id}:targetgroup/{name}/{id}");

        let tg = TargetGroup {
            arn: arn.clone(),
            name: name.to_string(),
            protocol: protocol.to_string(),
            port,
            vpc_id: vpc_id.to_string(),
            health_check_path: health_check_path.to_string(),
            target_type: target_type.to_string(),
            attributes: HashMap::new(),
            targets: Vec::new(),
        };

        self.target_groups.insert(arn.clone(), tg);
        Ok(self.target_groups.get(&arn).unwrap())
    }

    pub fn delete_target_group(&mut self, arn: &str) -> Result<(), ElbV2Error> {
        if self.target_groups.remove(arn).is_none() {
            return Err(ElbV2Error::TargetGroupNotFound(arn.to_string()));
        }
        self.resource_tags.remove(arn);
        Ok(())
    }

    pub fn describe_target_groups(&self) -> Vec<&TargetGroup> {
        self.target_groups.values().collect()
    }

    pub fn create_listener(
        &mut self,
        load_balancer_arn: &str,
        port: i32,
        protocol: &str,
        default_actions: Vec<ListenerAction>,
        account_id: &str,
        region: &str,
    ) -> Result<&Listener, ElbV2Error> {
        if !self.load_balancers.contains_key(load_balancer_arn) {
            return Err(ElbV2Error::LoadBalancerNotFound(
                load_balancer_arn.to_string(),
            ));
        }

        let id = &uuid::Uuid::new_v4().to_string()[..17];
        let arn = format!("arn:aws:elasticloadbalancing:{region}:{account_id}:listener/app/{id}");

        let listener = Listener {
            arn: arn.clone(),
            load_balancer_arn: load_balancer_arn.to_string(),
            port,
            protocol: protocol.to_string(),
            default_actions,
            certificates: Vec::new(),
            attributes: HashMap::new(),
        };

        self.listeners.insert(arn.clone(), listener);
        Ok(self.listeners.get(&arn).unwrap())
    }

    pub fn delete_listener(&mut self, arn: &str) -> Result<(), ElbV2Error> {
        if self.listeners.remove(arn).is_none() {
            return Err(ElbV2Error::ListenerNotFound(arn.to_string()));
        }
        // Remove rules for this listener
        self.rules.retain(|_, r| r.listener_arn != arn);
        self.resource_tags.remove(arn);
        Ok(())
    }

    pub fn describe_listeners(&self, load_balancer_arn: &str) -> Vec<&Listener> {
        self.listeners
            .values()
            .filter(|l| l.load_balancer_arn == load_balancer_arn)
            .collect()
    }

    pub fn modify_listener(
        &mut self,
        listener_arn: &str,
        port: Option<i32>,
        protocol: Option<&str>,
        default_actions: Option<Vec<ListenerAction>>,
    ) -> Result<&Listener, ElbV2Error> {
        let listener = self
            .listeners
            .get_mut(listener_arn)
            .ok_or_else(|| ElbV2Error::ListenerNotFound(listener_arn.to_string()))?;
        if let Some(p) = port {
            listener.port = p;
        }
        if let Some(proto) = protocol {
            listener.protocol = proto.to_string();
        }
        if let Some(actions) = default_actions {
            listener.default_actions = actions;
        }
        Ok(self.listeners.get(listener_arn).unwrap())
    }

    pub fn create_rule(
        &mut self,
        listener_arn: &str,
        priority: &str,
        conditions: Vec<RuleCondition>,
        actions: Vec<RuleAction>,
        account_id: &str,
        region: &str,
    ) -> Result<&Rule, ElbV2Error> {
        if !self.listeners.contains_key(listener_arn) {
            return Err(ElbV2Error::ListenerNotFound(listener_arn.to_string()));
        }

        // Check for duplicate priority on the same listener
        if self
            .rules
            .values()
            .any(|r| r.listener_arn == listener_arn && r.priority == priority)
        {
            return Err(ElbV2Error::PriorityInUse);
        }

        let id = &uuid::Uuid::new_v4().to_string()[..17];
        let rule_arn =
            format!("arn:aws:elasticloadbalancing:{region}:{account_id}:listener-rule/app/{id}");

        let rule = Rule {
            rule_arn: rule_arn.clone(),
            priority: priority.to_string(),
            conditions,
            actions,
            is_default: false,
            listener_arn: listener_arn.to_string(),
        };

        self.rules.insert(rule_arn.clone(), rule);
        Ok(self.rules.get(&rule_arn).unwrap())
    }

    pub fn delete_rule(&mut self, rule_arn: &str) -> Result<(), ElbV2Error> {
        let rule = self
            .rules
            .get(rule_arn)
            .ok_or_else(|| ElbV2Error::RuleNotFound(rule_arn.to_string()))?;
        if rule.is_default {
            return Err(ElbV2Error::OperationNotPermitted);
        }
        self.rules.remove(rule_arn);
        self.resource_tags.remove(rule_arn);
        Ok(())
    }

    pub fn describe_rules(&self, listener_arn: Option<&str>, rule_arns: &[String]) -> Vec<&Rule> {
        if !rule_arns.is_empty() {
            return self
                .rules
                .values()
                .filter(|r| rule_arns.contains(&r.rule_arn))
                .collect();
        }
        if let Some(la) = listener_arn {
            return self
                .rules
                .values()
                .filter(|r| r.listener_arn == la)
                .collect();
        }
        self.rules.values().collect()
    }

    pub fn modify_rule(
        &mut self,
        rule_arn: &str,
        conditions: Option<Vec<RuleCondition>>,
        actions: Option<Vec<RuleAction>>,
    ) -> Result<&Rule, ElbV2Error> {
        let rule = self
            .rules
            .get_mut(rule_arn)
            .ok_or_else(|| ElbV2Error::RuleNotFound(rule_arn.to_string()))?;
        if let Some(c) = conditions {
            rule.conditions = c;
        }
        if let Some(a) = actions {
            rule.actions = a;
        }
        Ok(self.rules.get(rule_arn).unwrap())
    }

    pub fn register_targets(
        &mut self,
        target_group_arn: &str,
        targets: Vec<TargetDescription>,
    ) -> Result<(), ElbV2Error> {
        let tg = self
            .target_groups
            .get_mut(target_group_arn)
            .ok_or_else(|| ElbV2Error::TargetGroupNotFound(target_group_arn.to_string()))?;
        for t in targets {
            if !tg
                .targets
                .iter()
                .any(|existing| existing.id == t.id && existing.port == t.port)
            {
                tg.targets.push(t);
            }
        }
        Ok(())
    }

    pub fn deregister_targets(
        &mut self,
        target_group_arn: &str,
        targets: &[TargetDescription],
    ) -> Result<(), ElbV2Error> {
        let tg = self
            .target_groups
            .get_mut(target_group_arn)
            .ok_or_else(|| ElbV2Error::TargetGroupNotFound(target_group_arn.to_string()))?;
        tg.targets.retain(|existing| {
            !targets
                .iter()
                .any(|t| t.id == existing.id && (t.port.is_none() || t.port == existing.port))
        });
        Ok(())
    }

    pub fn describe_target_health(
        &self,
        target_group_arn: &str,
    ) -> Result<Vec<&TargetDescription>, ElbV2Error> {
        let tg = self
            .target_groups
            .get(target_group_arn)
            .ok_or_else(|| ElbV2Error::TargetGroupNotFound(target_group_arn.to_string()))?;
        Ok(tg.targets.iter().collect())
    }

    pub fn modify_target_group(
        &mut self,
        target_group_arn: &str,
        health_check_path: Option<&str>,
    ) -> Result<&TargetGroup, ElbV2Error> {
        let tg = self
            .target_groups
            .get_mut(target_group_arn)
            .ok_or_else(|| ElbV2Error::TargetGroupNotFound(target_group_arn.to_string()))?;
        if let Some(path) = health_check_path {
            tg.health_check_path = path.to_string();
        }
        Ok(self.target_groups.get(target_group_arn).unwrap())
    }

    pub fn add_listener_certificates(
        &mut self,
        listener_arn: &str,
        certificates: Vec<Certificate>,
    ) -> Result<&[Certificate], ElbV2Error> {
        let listener = self
            .listeners
            .get_mut(listener_arn)
            .ok_or_else(|| ElbV2Error::ListenerNotFound(listener_arn.to_string()))?;
        for cert in certificates {
            if !listener
                .certificates
                .iter()
                .any(|c| c.certificate_arn == cert.certificate_arn)
            {
                listener.certificates.push(cert);
            }
        }
        let listener = self.listeners.get(listener_arn).unwrap();
        Ok(&listener.certificates)
    }

    pub fn remove_listener_certificates(
        &mut self,
        listener_arn: &str,
        certificate_arns: &[String],
    ) -> Result<(), ElbV2Error> {
        let listener = self
            .listeners
            .get_mut(listener_arn)
            .ok_or_else(|| ElbV2Error::ListenerNotFound(listener_arn.to_string()))?;
        listener
            .certificates
            .retain(|c| !certificate_arns.contains(&c.certificate_arn));
        Ok(())
    }

    pub fn describe_listener_certificates(
        &self,
        listener_arn: &str,
    ) -> Result<&[Certificate], ElbV2Error> {
        let listener = self
            .listeners
            .get(listener_arn)
            .ok_or_else(|| ElbV2Error::ListenerNotFound(listener_arn.to_string()))?;
        Ok(&listener.certificates)
    }

    pub fn set_rule_priorities(
        &mut self,
        rule_priorities: &[(String, String)],
    ) -> Result<Vec<&Rule>, ElbV2Error> {
        // Validate all rules exist first
        for (arn, _) in rule_priorities {
            if !self.rules.contains_key(arn) {
                return Err(ElbV2Error::RuleNotFound(arn.to_string()));
            }
        }
        for (arn, priority) in rule_priorities {
            if let Some(rule) = self.rules.get_mut(arn) {
                rule.priority = priority.clone();
            }
        }
        Ok(rule_priorities
            .iter()
            .filter_map(|(arn, _)| self.rules.get(arn))
            .collect())
    }

    pub fn add_tags(
        &mut self,
        resource_arns: &[String],
        tags: &[(String, String)],
    ) -> Result<(), ElbV2Error> {
        for arn in resource_arns {
            let tag_map = self.resource_tags.entry(arn.clone()).or_default();
            for (key, value) in tags {
                tag_map.insert(key.clone(), value.clone());
            }
        }
        Ok(())
    }

    pub fn remove_tags(
        &mut self,
        resource_arns: &[String],
        tag_keys: &[String],
    ) -> Result<(), ElbV2Error> {
        for arn in resource_arns {
            if let Some(tag_map) = self.resource_tags.get_mut(arn) {
                for key in tag_keys {
                    tag_map.remove(key);
                }
            }
        }
        Ok(())
    }

    pub fn describe_tags<'a>(
        &'a self,
        resource_arns: &'a [String],
    ) -> Vec<(&'a str, Option<&'a HashMap<String, String>>)> {
        resource_arns
            .iter()
            .filter(|arn| {
                // Only return entries for resources that actually exist
                self.load_balancers.contains_key(arn.as_str())
                    || self.target_groups.contains_key(arn.as_str())
                    || self.listeners.contains_key(arn.as_str())
                    || self.rules.contains_key(arn.as_str())
                    || self.trust_stores.contains_key(arn.as_str())
                    || self.resource_tags.contains_key(arn.as_str())
            })
            .map(|arn| (arn.as_str(), self.resource_tags.get(arn)))
            .collect()
    }

    pub fn describe_load_balancer_attributes(
        &self,
        lb_arn: &str,
    ) -> Result<&HashMap<String, String>, ElbV2Error> {
        let lb = self
            .load_balancers
            .get(lb_arn)
            .ok_or_else(|| ElbV2Error::LoadBalancerNotFound(lb_arn.to_string()))?;
        Ok(&lb.attributes)
    }

    pub fn modify_load_balancer_attributes(
        &mut self,
        lb_arn: &str,
        attributes: &[(String, String)],
    ) -> Result<&HashMap<String, String>, ElbV2Error> {
        let lb = self
            .load_balancers
            .get_mut(lb_arn)
            .ok_or_else(|| ElbV2Error::LoadBalancerNotFound(lb_arn.to_string()))?;
        for (key, value) in attributes {
            lb.attributes.insert(key.clone(), value.clone());
        }
        Ok(&self.load_balancers.get(lb_arn).unwrap().attributes)
    }

    pub fn describe_listener_attributes(
        &self,
        listener_arn: &str,
    ) -> Result<&HashMap<String, String>, ElbV2Error> {
        let listener = self
            .listeners
            .get(listener_arn)
            .ok_or_else(|| ElbV2Error::ListenerNotFound(listener_arn.to_string()))?;
        Ok(&listener.attributes)
    }

    pub fn modify_listener_attributes(
        &mut self,
        listener_arn: &str,
        attributes: &[(String, String)],
    ) -> Result<&HashMap<String, String>, ElbV2Error> {
        let listener = self
            .listeners
            .get_mut(listener_arn)
            .ok_or_else(|| ElbV2Error::ListenerNotFound(listener_arn.to_string()))?;
        for (key, value) in attributes {
            listener.attributes.insert(key.clone(), value.clone());
        }
        Ok(&self.listeners.get(listener_arn).unwrap().attributes)
    }

    pub fn describe_target_group_attributes(
        &self,
        tg_arn: &str,
    ) -> Result<&HashMap<String, String>, ElbV2Error> {
        let tg = self
            .target_groups
            .get(tg_arn)
            .ok_or_else(|| ElbV2Error::TargetGroupNotFound(tg_arn.to_string()))?;
        Ok(&tg.attributes)
    }

    pub fn modify_target_group_attributes(
        &mut self,
        tg_arn: &str,
        attributes: &[(String, String)],
    ) -> Result<&HashMap<String, String>, ElbV2Error> {
        let tg = self
            .target_groups
            .get_mut(tg_arn)
            .ok_or_else(|| ElbV2Error::TargetGroupNotFound(tg_arn.to_string()))?;
        for (key, value) in attributes {
            tg.attributes.insert(key.clone(), value.clone());
        }
        Ok(&self.target_groups.get(tg_arn).unwrap().attributes)
    }

    pub fn set_ip_address_type(
        &mut self,
        lb_arn: &str,
        ip_address_type: &str,
    ) -> Result<&str, ElbV2Error> {
        let lb = self
            .load_balancers
            .get_mut(lb_arn)
            .ok_or_else(|| ElbV2Error::LoadBalancerNotFound(lb_arn.to_string()))?;
        lb.ip_address_type = ip_address_type.to_string();
        Ok(&self.load_balancers.get(lb_arn).unwrap().ip_address_type)
    }

    pub fn set_security_groups(
        &mut self,
        lb_arn: &str,
        security_groups: Vec<String>,
    ) -> Result<&[String], ElbV2Error> {
        let lb = self
            .load_balancers
            .get_mut(lb_arn)
            .ok_or_else(|| ElbV2Error::LoadBalancerNotFound(lb_arn.to_string()))?;
        lb.security_groups = security_groups;
        Ok(&self.load_balancers.get(lb_arn).unwrap().security_groups)
    }

    // --- TrustStore operations ---

    pub fn create_trust_store(
        &mut self,
        name: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&TrustStore, ElbV2Error> {
        if self.trust_stores.values().any(|ts| ts.name == name) {
            return Err(ElbV2Error::DuplicateTrustStoreName(name.to_string()));
        }
        let id = &uuid::Uuid::new_v4().to_string()[..17];
        let arn =
            format!("arn:aws:elasticloadbalancing:{region}:{account_id}:truststore/{name}/{id}");
        let ts = TrustStore {
            arn: arn.clone(),
            name: name.to_string(),
            status: "ACTIVE".to_string(),
            number_of_ca_certificates: 1,
            total_revoked_entries: 0,
            revocations: HashMap::new(),
            next_revocation_id: 1,
        };
        self.trust_stores.insert(arn.clone(), ts);
        Ok(self.trust_stores.get(&arn).unwrap())
    }

    pub fn delete_trust_store(&mut self, arn: &str) -> Result<(), ElbV2Error> {
        if self.trust_stores.remove(arn).is_none() {
            return Err(ElbV2Error::TrustStoreNotFound(arn.to_string()));
        }
        Ok(())
    }

    pub fn describe_trust_stores(&self, arns: &[String], names: &[String]) -> Vec<&TrustStore> {
        self.trust_stores
            .values()
            .filter(|ts| {
                (arns.is_empty() || arns.contains(&ts.arn))
                    && (names.is_empty() || names.contains(&ts.name))
            })
            .collect()
    }

    pub fn modify_trust_store(&mut self, arn: &str) -> Result<&TrustStore, ElbV2Error> {
        // We accept the new CA bundle params but don't need to do anything meaningful.
        // Just verify the trust store exists.
        if !self.trust_stores.contains_key(arn) {
            return Err(ElbV2Error::TrustStoreNotFound(arn.to_string()));
        }
        Ok(self.trust_stores.get(arn).unwrap())
    }

    pub fn add_trust_store_revocations(
        &mut self,
        trust_store_arn: &str,
        number_of_entries: i64,
        revocation_type: &str,
    ) -> Result<Vec<(i64, i64, String)>, ElbV2Error> {
        let ts = self
            .trust_stores
            .get_mut(trust_store_arn)
            .ok_or_else(|| ElbV2Error::TrustStoreNotFound(trust_store_arn.to_string()))?;
        let rev_id = ts.next_revocation_id;
        ts.next_revocation_id += 1;
        ts.total_revoked_entries += number_of_entries;
        let entry = TrustStoreRevocationEntry {
            revocation_id: rev_id,
            revocation_type: revocation_type.to_string(),
            number_of_revoked_entries: number_of_entries,
        };
        ts.revocations.insert(rev_id, entry);
        Ok(vec![(
            rev_id,
            number_of_entries,
            revocation_type.to_string(),
        )])
    }

    pub fn remove_trust_store_revocations(
        &mut self,
        trust_store_arn: &str,
        revocation_ids: &[i64],
    ) -> Result<(), ElbV2Error> {
        let ts = self
            .trust_stores
            .get_mut(trust_store_arn)
            .ok_or_else(|| ElbV2Error::TrustStoreNotFound(trust_store_arn.to_string()))?;
        for id in revocation_ids {
            if let Some(entry) = ts.revocations.remove(id) {
                ts.total_revoked_entries -= entry.number_of_revoked_entries;
            }
        }
        Ok(())
    }

    pub fn describe_trust_store_revocations(
        &self,
        trust_store_arn: &str,
    ) -> Result<Vec<(i64, i64, String)>, ElbV2Error> {
        let ts = self
            .trust_stores
            .get(trust_store_arn)
            .ok_or_else(|| ElbV2Error::TrustStoreNotFound(trust_store_arn.to_string()))?;
        Ok(ts
            .revocations
            .values()
            .map(|e| {
                (
                    e.revocation_id,
                    e.number_of_revoked_entries,
                    e.revocation_type.clone(),
                )
            })
            .collect())
    }

    pub fn set_subnets(
        &mut self,
        lb_arn: &str,
        subnets: Vec<String>,
        region: &str,
    ) -> Result<Vec<AvailabilityZone>, ElbV2Error> {
        let lb = self
            .load_balancers
            .get_mut(lb_arn)
            .ok_or_else(|| ElbV2Error::LoadBalancerNotFound(lb_arn.to_string()))?;
        lb.subnets = subnets.clone();
        let azs: Vec<AvailabilityZone> = subnets
            .iter()
            .enumerate()
            .map(|(i, subnet_id)| {
                let zone_suffix = (b'a' + (i as u8 % 6)) as char;
                AvailabilityZone {
                    zone_name: format!("{region}{zone_suffix}"),
                    subnet_id: subnet_id.clone(),
                }
            })
            .collect();
        lb.availability_zones = azs.clone();
        Ok(azs)
    }

    // --- Trust store associations ---

    pub fn describe_trust_store_associations(
        &self,
        trust_store_arn: &str,
    ) -> Result<&[TrustStoreAssociation], ElbV2Error> {
        if !self.trust_stores.contains_key(trust_store_arn) {
            return Err(ElbV2Error::TrustStoreNotFound(trust_store_arn.to_string()));
        }
        Ok(self
            .trust_store_associations
            .get(trust_store_arn)
            .map(|v| v.as_slice())
            .unwrap_or(&[]))
    }

    pub fn delete_shared_trust_store_association(
        &mut self,
        trust_store_arn: &str,
        resource_arn: &str,
    ) -> Result<(), ElbV2Error> {
        if !self.trust_stores.contains_key(trust_store_arn) {
            return Err(ElbV2Error::TrustStoreNotFound(trust_store_arn.to_string()));
        }
        if let Some(assocs) = self.trust_store_associations.get_mut(trust_store_arn) {
            assocs.retain(|a| a.resource_arn != resource_arn);
        }
        Ok(())
    }

    // --- Capacity reservations ---

    pub fn get_capacity_reservation(&self, lb_arn: &str) -> Option<&CapacityReservation> {
        self.capacity_reservations.get(lb_arn)
    }

    pub fn modify_capacity_reservation(
        &mut self,
        lb_arn: &str,
        minimum_capacity_units: Option<i32>,
    ) -> Result<&CapacityReservation, ElbV2Error> {
        if !self.load_balancers.contains_key(lb_arn) {
            return Err(ElbV2Error::LoadBalancerNotFound(lb_arn.to_string()));
        }
        let reservation = self
            .capacity_reservations
            .entry(lb_arn.to_string())
            .or_insert_with(|| CapacityReservation {
                load_balancer_arn: lb_arn.to_string(),
                minimum_capacity_units: 0,
                availability_zone_states: vec![],
                decrease_requests_remaining: 10,
            });
        if let Some(units) = minimum_capacity_units {
            reservation.minimum_capacity_units = units;
        }
        Ok(self.capacity_reservations.get(lb_arn).unwrap())
    }

    // --- Resource policies ---

    pub fn get_resource_policy(&self, resource_arn: &str) -> Option<&ResourcePolicy> {
        self.resource_policies.get(resource_arn)
    }

    pub fn put_resource_policy(&mut self, resource_arn: &str, policy: &str) {
        self.resource_policies.insert(
            resource_arn.to_string(),
            ResourcePolicy {
                resource_arn: resource_arn.to_string(),
                policy: policy.to_string(),
            },
        );
    }

    // --- IPAM pools ---

    pub fn modify_ip_pools(
        &mut self,
        lb_arn: &str,
        ipv4_pool_id: Option<&str>,
        ipv6_pool_id: Option<&str>,
    ) -> Result<&IpamPool, ElbV2Error> {
        if !self.load_balancers.contains_key(lb_arn) {
            return Err(ElbV2Error::LoadBalancerNotFound(lb_arn.to_string()));
        }
        let pool = self
            .ipam_pools
            .entry(lb_arn.to_string())
            .or_insert_with(|| IpamPool {
                ipv4_ipam_pool_id: None,
                ipv6_ipam_pool_id: None,
            });
        if let Some(id) = ipv4_pool_id {
            pool.ipv4_ipam_pool_id = Some(id.to_string());
        }
        if let Some(id) = ipv6_pool_id {
            pool.ipv6_ipam_pool_id = Some(id.to_string());
        }
        Ok(self.ipam_pools.get(lb_arn).unwrap())
    }
}
