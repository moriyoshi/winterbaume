use std::collections::HashMap;

use crate::types::{ElbAttributes, ElbHealthCheck, ElbListener, ElbLoadBalancer, ElbPolicy};

#[derive(Debug, Default)]
pub struct ElbState {
    pub load_balancers: HashMap<String, ElbLoadBalancer>,
}

#[derive(Debug, thiserror::Error)]
pub enum ElbError {
    #[error("A load balancer with the name '{0}' already exists.")]
    DuplicateLoadBalancerName(String),

    #[error("Cannot find Load Balancer: {0}")]
    LoadBalancerNotFound(String),

    #[error("No listener on port {port} for load balancer '{lb_name}'.")]
    ListenerNotFound { lb_name: String, port: i32 },

    #[error("ValidationError: {0}")]
    ValidationError(String),
}

impl ElbState {
    pub fn create_load_balancer(
        &mut self,
        name: &str,
        scheme: &str,
        availability_zones: Vec<String>,
        subnets: Vec<String>,
        security_groups: Vec<String>,
        listeners: Vec<ElbListener>,
        region: &str,
    ) -> Result<&ElbLoadBalancer, ElbError> {
        if self.load_balancers.contains_key(name) {
            return Err(ElbError::DuplicateLoadBalancerName(name.to_string()));
        }

        let hash_part = &uuid::Uuid::new_v4().to_string()[..8];
        let dns_name = if scheme == "internal" {
            format!("internal-{name}-{hash_part}.{region}.elb.amazonaws.com")
        } else {
            format!("{name}-{hash_part}.{region}.elb.amazonaws.com")
        };

        let vpc_id = if !subnets.is_empty() {
            Some("vpc-12345678".to_string())
        } else {
            None
        };

        let lb = ElbLoadBalancer {
            name: name.to_string(),
            dns_name,
            scheme: scheme.to_string(),
            availability_zones,
            subnets,
            security_groups,
            instances: Vec::new(),
            listeners,
            health_check: ElbHealthCheck::default(),
            tags: HashMap::new(),
            policies: Vec::new(),
            attributes: ElbAttributes::default(),
            vpc_id,
        };

        self.load_balancers.insert(name.to_string(), lb);
        Ok(self.load_balancers.get(name).unwrap())
    }

    pub fn delete_load_balancer(&mut self, name: &str) -> Result<(), ElbError> {
        if self.load_balancers.remove(name).is_none() {
            // ELB Classic silently ignores deleting non-existent LBs
        }
        Ok(())
    }

    pub fn describe_load_balancers(
        &self,
        names: &[String],
    ) -> Result<Vec<&ElbLoadBalancer>, ElbError> {
        if names.is_empty() {
            return Ok(self.load_balancers.values().collect());
        }
        let mut result = Vec::new();
        for name in names {
            match self.load_balancers.get(name.as_str()) {
                Some(lb) => result.push(lb),
                None => {
                    return Err(ElbError::LoadBalancerNotFound(name.clone()));
                }
            }
        }
        Ok(result)
    }

    pub fn register_instances(
        &mut self,
        lb_name: &str,
        instance_ids: &[String],
    ) -> Result<Vec<String>, ElbError> {
        let lb = self
            .load_balancers
            .get_mut(lb_name)
            .ok_or_else(|| ElbError::LoadBalancerNotFound(lb_name.to_string()))?;
        for id in instance_ids {
            if !lb.instances.contains(id) {
                lb.instances.push(id.clone());
            }
        }
        Ok(lb.instances.clone())
    }

    pub fn deregister_instances(
        &mut self,
        lb_name: &str,
        instance_ids: &[String],
    ) -> Result<Vec<String>, ElbError> {
        let lb = self
            .load_balancers
            .get_mut(lb_name)
            .ok_or_else(|| ElbError::LoadBalancerNotFound(lb_name.to_string()))?;
        lb.instances.retain(|id| !instance_ids.contains(id));
        Ok(lb.instances.clone())
    }

    pub fn describe_instance_health(
        &self,
        lb_name: &str,
        instance_ids: &[String],
    ) -> Result<Vec<String>, ElbError> {
        let lb = self
            .load_balancers
            .get(lb_name)
            .ok_or_else(|| ElbError::LoadBalancerNotFound(lb_name.to_string()))?;
        if instance_ids.is_empty() {
            Ok(lb.instances.clone())
        } else {
            Ok(instance_ids.to_vec())
        }
    }

    pub fn configure_health_check(
        &mut self,
        lb_name: &str,
        hc: ElbHealthCheck,
    ) -> Result<ElbHealthCheck, ElbError> {
        let lb = self
            .load_balancers
            .get_mut(lb_name)
            .ok_or_else(|| ElbError::LoadBalancerNotFound(lb_name.to_string()))?;
        lb.health_check = hc.clone();
        Ok(hc)
    }

    pub fn create_load_balancer_listeners(
        &mut self,
        lb_name: &str,
        listeners: Vec<ElbListener>,
    ) -> Result<(), ElbError> {
        let lb = self
            .load_balancers
            .get_mut(lb_name)
            .ok_or_else(|| ElbError::LoadBalancerNotFound(lb_name.to_string()))?;
        for listener in listeners {
            // Replace existing listener on the same port
            lb.listeners
                .retain(|l| l.load_balancer_port != listener.load_balancer_port);
            lb.listeners.push(listener);
        }
        Ok(())
    }

    pub fn delete_load_balancer_listeners(
        &mut self,
        lb_name: &str,
        ports: &[i32],
    ) -> Result<(), ElbError> {
        let lb = self
            .load_balancers
            .get_mut(lb_name)
            .ok_or_else(|| ElbError::LoadBalancerNotFound(lb_name.to_string()))?;
        lb.listeners
            .retain(|l| !ports.contains(&l.load_balancer_port));
        Ok(())
    }

    pub fn describe_load_balancer_policies(
        &self,
        lb_name: Option<&str>,
        policy_names: &[String],
    ) -> Result<Vec<&ElbPolicy>, ElbError> {
        if let Some(name) = lb_name {
            let lb = self
                .load_balancers
                .get(name)
                .ok_or_else(|| ElbError::LoadBalancerNotFound(name.to_string()))?;
            if policy_names.is_empty() {
                return Ok(lb.policies.iter().collect());
            }
            Ok(lb
                .policies
                .iter()
                .filter(|p| policy_names.contains(&p.policy_name))
                .collect())
        } else {
            let mut result = Vec::new();
            for lb in self.load_balancers.values() {
                for p in &lb.policies {
                    if policy_names.is_empty() || policy_names.contains(&p.policy_name) {
                        result.push(p);
                    }
                }
            }
            Ok(result)
        }
    }

    pub fn attach_load_balancer_to_subnets(
        &mut self,
        lb_name: &str,
        subnets: &[String],
    ) -> Result<Vec<String>, ElbError> {
        let lb = self
            .load_balancers
            .get_mut(lb_name)
            .ok_or_else(|| ElbError::LoadBalancerNotFound(lb_name.to_string()))?;
        for s in subnets {
            if !lb.subnets.contains(s) {
                lb.subnets.push(s.clone());
            }
        }
        Ok(lb.subnets.clone())
    }

    pub fn detach_load_balancer_from_subnets(
        &mut self,
        lb_name: &str,
        subnets: &[String],
    ) -> Result<Vec<String>, ElbError> {
        let lb = self
            .load_balancers
            .get_mut(lb_name)
            .ok_or_else(|| ElbError::LoadBalancerNotFound(lb_name.to_string()))?;
        lb.subnets.retain(|s| !subnets.contains(s));
        Ok(lb.subnets.clone())
    }

    pub fn apply_security_groups(
        &mut self,
        lb_name: &str,
        security_groups: Vec<String>,
    ) -> Result<Vec<String>, ElbError> {
        let lb = self
            .load_balancers
            .get_mut(lb_name)
            .ok_or_else(|| ElbError::LoadBalancerNotFound(lb_name.to_string()))?;
        lb.security_groups = security_groups.clone();
        Ok(security_groups)
    }

    pub fn add_tags(
        &mut self,
        lb_names: &[String],
        tags: Vec<(String, String)>,
    ) -> Result<(), ElbError> {
        for name in lb_names {
            let lb = self
                .load_balancers
                .get_mut(name.as_str())
                .ok_or_else(|| ElbError::LoadBalancerNotFound(name.clone()))?;
            for (k, v) in &tags {
                lb.tags.insert(k.clone(), v.clone());
            }
        }
        Ok(())
    }

    pub fn describe_tags(
        &self,
        lb_names: &[String],
    ) -> Result<Vec<(&str, &HashMap<String, String>)>, ElbError> {
        let mut result = Vec::new();
        for name in lb_names {
            let lb = self
                .load_balancers
                .get(name.as_str())
                .ok_or_else(|| ElbError::LoadBalancerNotFound(name.clone()))?;
            result.push((lb.name.as_str(), &lb.tags));
        }
        Ok(result)
    }

    pub fn remove_tags(
        &mut self,
        lb_names: &[String],
        tag_keys: &[String],
    ) -> Result<(), ElbError> {
        for name in lb_names {
            let lb = self
                .load_balancers
                .get_mut(name.as_str())
                .ok_or_else(|| ElbError::LoadBalancerNotFound(name.clone()))?;
            for key in tag_keys {
                lb.tags.remove(key.as_str());
            }
        }
        Ok(())
    }

    pub fn enable_availability_zones(
        &mut self,
        lb_name: &str,
        zones: &[String],
    ) -> Result<Vec<String>, ElbError> {
        let lb = self
            .load_balancers
            .get_mut(lb_name)
            .ok_or_else(|| ElbError::LoadBalancerNotFound(lb_name.to_string()))?;
        for z in zones {
            if !lb.availability_zones.contains(z) {
                lb.availability_zones.push(z.clone());
            }
        }
        Ok(lb.availability_zones.clone())
    }

    pub fn disable_availability_zones(
        &mut self,
        lb_name: &str,
        zones: &[String],
    ) -> Result<Vec<String>, ElbError> {
        let lb = self
            .load_balancers
            .get_mut(lb_name)
            .ok_or_else(|| ElbError::LoadBalancerNotFound(lb_name.to_string()))?;
        lb.availability_zones.retain(|z| !zones.contains(z));
        Ok(lb.availability_zones.clone())
    }

    pub fn set_load_balancer_policies_of_listener(
        &mut self,
        lb_name: &str,
        port: i32,
        policy_names: Vec<String>,
    ) -> Result<(), ElbError> {
        let lb = self
            .load_balancers
            .get_mut(lb_name)
            .ok_or_else(|| ElbError::LoadBalancerNotFound(lb_name.to_string()))?;
        for listener in lb.listeners.iter_mut() {
            if listener.load_balancer_port == port {
                listener.policy_names = policy_names;
                return Ok(());
            }
        }
        Err(ElbError::ListenerNotFound {
            lb_name: lb_name.to_string(),
            port,
        })
    }

    pub fn create_load_balancer_policy(
        &mut self,
        lb_name: &str,
        policy_name: &str,
        policy_type_name: &str,
        attributes: Vec<(String, String)>,
    ) -> Result<(), ElbError> {
        let lb = self
            .load_balancers
            .get_mut(lb_name)
            .ok_or_else(|| ElbError::LoadBalancerNotFound(lb_name.to_string()))?;
        // Replace if already exists
        lb.policies.retain(|p| p.policy_name != policy_name);
        lb.policies.push(ElbPolicy {
            policy_name: policy_name.to_string(),
            policy_type_name: policy_type_name.to_string(),
            attributes,
        });
        Ok(())
    }

    pub fn modify_load_balancer_attributes(
        &mut self,
        lb_name: &str,
        attrs: ElbAttributes,
    ) -> Result<ElbAttributes, ElbError> {
        let lb = self
            .load_balancers
            .get_mut(lb_name)
            .ok_or_else(|| ElbError::LoadBalancerNotFound(lb_name.to_string()))?;
        lb.attributes = attrs.clone();
        Ok(attrs)
    }

    pub fn describe_load_balancer_attributes(
        &self,
        lb_name: &str,
    ) -> Result<&ElbAttributes, ElbError> {
        let lb = self
            .load_balancers
            .get(lb_name)
            .ok_or_else(|| ElbError::LoadBalancerNotFound(lb_name.to_string()))?;
        Ok(&lb.attributes)
    }
}
