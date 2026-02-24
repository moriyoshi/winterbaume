use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct EksState {
    pub clusters: HashMap<String, Cluster>,
    /// EKS Anywhere subscriptions keyed by subscription ID
    pub eks_anywhere_subscriptions: HashMap<String, EksAnywhereSubscription>,
    /// Tags for resources: key = ARN
    pub resource_tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, thiserror::Error)]
pub enum EksError {
    #[error("Cluster already exists with name: {0}")]
    ClusterAlreadyExists(String),
    #[error("Cluster already exists: {0}")]
    ClusterAlreadyExistsPlain(String),
    #[error("Cluster has active resources")]
    ClusterHasActiveResources,
    #[error("No cluster found for name: {0}")]
    ClusterNotFound(String),
    #[error("NodeGroup already exists with name: {0}")]
    NodegroupAlreadyExists(String),
    #[error("No node group found for name: {0}")]
    NodegroupNotFound(String),
    #[error("Fargate profile already exists with name: {0}")]
    FargateProfileAlreadyExists(String),
    #[error("No Fargate profile found for name: {0}")]
    FargateProfileNotFound(String),
    #[error("Addon already exists with name: {0}")]
    AddonAlreadyExists(String),
    #[error("No addon found for name: {0}")]
    AddonNotFound(String),
    #[error("Access entry already exists for principal: {0}")]
    AccessEntryAlreadyExists(String),
    #[error("No access entry found for principal: {0}")]
    AccessEntryNotFound(String),
    #[error("Identity provider config already exists: {0}")]
    IdentityProviderConfigAlreadyExists(String),
    #[error("No identity provider config found: {0}")]
    IdentityProviderConfigNotFound(String),
    #[error("No pod identity association found: {0}")]
    PodIdentityAssociationNotFound(String),
    #[error("No EKS Anywhere subscription found: {0}")]
    EksAnywhereSubscriptionNotFound(String),
    #[error("Capability already exists: {0}")]
    CapabilityAlreadyExists(String),
    #[error("No capability found: {0}")]
    CapabilityNotFound(String),
}

impl EksState {
    pub fn create_cluster(
        &mut self,
        name: &str,
        role_arn: &str,
        version: Option<&str>,
        account_id: &str,
        region: &str,
        tags: Option<HashMap<String, String>>,
    ) -> Result<&Cluster, EksError> {
        if self.clusters.contains_key(name) {
            return Err(EksError::ClusterAlreadyExists(name.to_string()));
        }

        let arn = format!("arn:aws:eks:{region}:{account_id}:cluster/{name}");
        let endpoint = format!(
            "https://{}.gr7.{region}.eks.amazonaws.com",
            uuid::Uuid::new_v4()
        );

        let cluster = Cluster {
            name: name.to_string(),
            arn,
            endpoint,
            role_arn: role_arn.to_string(),
            status: "ACTIVE".to_string(),
            version: version.unwrap_or("1.28").to_string(),
            created_at: Utc::now(),
            nodegroups: HashMap::new(),
            fargate_profiles: HashMap::new(),
            addons: HashMap::new(),
            access_entries: HashMap::new(),
            identity_provider_configs: HashMap::new(),
            pod_identity_associations: HashMap::new(),
            capabilities: HashMap::new(),
            tags: tags.clone().unwrap_or_default(),
            updates: HashMap::new(),
        };

        self.clusters.insert(name.to_string(), cluster.clone());
        // Store tags by ARN as well
        if let Some(t) = tags {
            if !t.is_empty() {
                let arn = self.clusters.get(name).unwrap().arn.clone();
                self.resource_tags.entry(arn).or_default().extend(t);
            }
        }
        Ok(self.clusters.get(name).unwrap())
    }

    pub fn describe_cluster(&self, name: &str) -> Result<&Cluster, EksError> {
        self.clusters
            .get(name)
            .ok_or_else(|| cluster_not_found(name))
    }

    pub fn delete_cluster(&mut self, name: &str) -> Result<Cluster, EksError> {
        let cluster = self
            .clusters
            .get(name)
            .ok_or_else(|| cluster_not_found(name))?;
        if !cluster.nodegroups.is_empty() || !cluster.fargate_profiles.is_empty() {
            return Err(EksError::ClusterHasActiveResources);
        }
        Ok(self.clusters.remove(name).unwrap())
    }

    pub fn list_clusters(&self) -> Vec<&str> {
        self.clusters.keys().map(|k| k.as_str()).collect()
    }

    pub fn update_cluster_config(&mut self, name: &str) -> Result<&Cluster, EksError> {
        // In a real mock we might update logging, vpc config, etc.
        // For now just verify the cluster exists.
        self.clusters
            .get(name)
            .ok_or_else(|| cluster_not_found(name))
    }

    // --- Nodegroup operations ---

    pub fn create_nodegroup(
        &mut self,
        cluster_name: &str,
        nodegroup_name: &str,
        node_role: &str,
        scaling_config: Option<ScalingConfig>,
        account_id: &str,
        region: &str,
    ) -> Result<&Nodegroup, EksError> {
        let cluster = self
            .clusters
            .get_mut(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;

        if cluster.nodegroups.contains_key(nodegroup_name) {
            return Err(EksError::NodegroupAlreadyExists(nodegroup_name.to_string()));
        }

        let arn =
            format!("arn:aws:eks:{region}:{account_id}:nodegroup/{cluster_name}/{nodegroup_name}");

        let scaling = scaling_config.unwrap_or(ScalingConfig {
            min_size: 1,
            max_size: 2,
            desired_size: 2,
        });

        let nodegroup = Nodegroup {
            name: nodegroup_name.to_string(),
            arn,
            cluster_name: cluster_name.to_string(),
            node_role: node_role.to_string(),
            scaling_config: scaling,
            status: "ACTIVE".to_string(),
            tags: HashMap::new(),
            labels: HashMap::new(),
            taints: Vec::new(),
        };

        cluster
            .nodegroups
            .insert(nodegroup_name.to_string(), nodegroup);
        Ok(cluster.nodegroups.get(nodegroup_name).unwrap())
    }

    pub fn describe_nodegroup(
        &self,
        cluster_name: &str,
        nodegroup_name: &str,
    ) -> Result<&Nodegroup, EksError> {
        let cluster = self
            .clusters
            .get(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        cluster
            .nodegroups
            .get(nodegroup_name)
            .ok_or_else(|| EksError::NodegroupNotFound(nodegroup_name.to_string()))
    }

    pub fn delete_nodegroup(
        &mut self,
        cluster_name: &str,
        nodegroup_name: &str,
    ) -> Result<Nodegroup, EksError> {
        let cluster = self
            .clusters
            .get_mut(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        cluster
            .nodegroups
            .remove(nodegroup_name)
            .ok_or_else(|| EksError::NodegroupNotFound(nodegroup_name.to_string()))
    }

    pub fn list_nodegroups(&self, cluster_name: &str) -> Result<Vec<&str>, EksError> {
        let cluster = self
            .clusters
            .get(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        Ok(cluster.nodegroups.keys().map(|k| k.as_str()).collect())
    }

    pub fn update_nodegroup_config(
        &mut self,
        cluster_name: &str,
        nodegroup_name: &str,
        scaling_config: Option<ScalingConfig>,
        labels_add: Option<HashMap<String, String>>,
        labels_remove: Option<Vec<String>>,
        taints_add: Option<Vec<crate::types::Taint>>,
        taints_remove: Option<Vec<(String, String)>>,
    ) -> Result<&Nodegroup, EksError> {
        let cluster = self
            .clusters
            .get_mut(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        let ng = cluster
            .nodegroups
            .get_mut(nodegroup_name)
            .ok_or_else(|| EksError::NodegroupNotFound(nodegroup_name.to_string()))?;
        if let Some(sc) = scaling_config {
            ng.scaling_config = sc;
        }
        if let Some(add) = labels_add {
            ng.labels.extend(add);
        }
        if let Some(remove) = labels_remove {
            for key in remove {
                ng.labels.remove(&key);
            }
        }
        if let Some(add) = taints_add {
            for taint in add {
                // Update existing or add new
                if let Some(existing) = ng
                    .taints
                    .iter_mut()
                    .find(|t| t.key == taint.key && t.effect == taint.effect)
                {
                    existing.value = taint.value;
                } else {
                    ng.taints.push(taint);
                }
            }
        }
        if let Some(remove) = taints_remove {
            ng.taints
                .retain(|t| !remove.iter().any(|(k, e)| t.key == *k && t.effect == *e));
        }
        Ok(ng)
    }

    // --- Fargate Profile operations ---

    pub fn create_fargate_profile(
        &mut self,
        cluster_name: &str,
        profile_name: &str,
        pod_execution_role_arn: &str,
        selectors: Vec<FargateSelector>,
        account_id: &str,
        region: &str,
    ) -> Result<&FargateProfile, EksError> {
        let cluster = self
            .clusters
            .get_mut(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;

        if cluster.fargate_profiles.contains_key(profile_name) {
            return Err(EksError::FargateProfileAlreadyExists(
                profile_name.to_string(),
            ));
        }

        let arn = format!(
            "arn:aws:eks:{region}:{account_id}:fargateprofile/{cluster_name}/{profile_name}/{}",
            uuid::Uuid::new_v4()
        );

        let profile = FargateProfile {
            name: profile_name.to_string(),
            arn,
            cluster_name: cluster_name.to_string(),
            pod_execution_role_arn: pod_execution_role_arn.to_string(),
            selectors,
            status: "ACTIVE".to_string(),
            tags: HashMap::new(),
        };

        cluster
            .fargate_profiles
            .insert(profile_name.to_string(), profile);
        Ok(cluster.fargate_profiles.get(profile_name).unwrap())
    }

    pub fn describe_fargate_profile(
        &self,
        cluster_name: &str,
        profile_name: &str,
    ) -> Result<&FargateProfile, EksError> {
        let cluster = self
            .clusters
            .get(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        cluster
            .fargate_profiles
            .get(profile_name)
            .ok_or_else(|| EksError::FargateProfileNotFound(profile_name.to_string()))
    }

    pub fn delete_fargate_profile(
        &mut self,
        cluster_name: &str,
        profile_name: &str,
    ) -> Result<FargateProfile, EksError> {
        let cluster = self
            .clusters
            .get_mut(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        cluster
            .fargate_profiles
            .remove(profile_name)
            .ok_or_else(|| EksError::FargateProfileNotFound(profile_name.to_string()))
    }

    pub fn list_fargate_profiles(&self, cluster_name: &str) -> Result<Vec<&str>, EksError> {
        let cluster = self
            .clusters
            .get(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        Ok(cluster
            .fargate_profiles
            .keys()
            .map(|k| k.as_str())
            .collect())
    }

    // --- Tag operations ---

    pub fn tag_resource(&mut self, arn: &str, tags: HashMap<String, String>) {
        let entry = self.resource_tags.entry(arn.to_string()).or_default();
        entry.extend(tags);
    }

    pub fn untag_resource(&mut self, arn: &str, tag_keys: &[String]) {
        if let Some(tags) = self.resource_tags.get_mut(arn) {
            for key in tag_keys {
                tags.remove(key);
            }
        }
    }

    pub fn list_tags_for_resource(&self, arn: &str) -> HashMap<String, String> {
        self.resource_tags.get(arn).cloned().unwrap_or_default()
    }

    // --- Addon operations ---

    pub fn create_addon(
        &mut self,
        cluster_name: &str,
        addon_name: &str,
        addon_version: Option<&str>,
        service_account_role_arn: Option<&str>,
        account_id: &str,
        region: &str,
        tags: Option<HashMap<String, String>>,
    ) -> Result<&Addon, EksError> {
        let cluster = self
            .clusters
            .get_mut(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;

        if cluster.addons.contains_key(addon_name) {
            return Err(EksError::AddonAlreadyExists(addon_name.to_string()));
        }

        let now = Utc::now();
        let addon_arn = format!(
            "arn:aws:eks:{region}:{account_id}:addon/{cluster_name}/{addon_name}/{}",
            uuid::Uuid::new_v4()
        );

        let addon = Addon {
            addon_name: addon_name.to_string(),
            addon_arn,
            cluster_name: cluster_name.to_string(),
            addon_version: addon_version.unwrap_or("v1.0.0").to_string(),
            service_account_role_arn: service_account_role_arn.map(|s| s.to_string()),
            status: "ACTIVE".to_string(),
            created_at: now,
            modified_at: now,
            tags: tags.unwrap_or_default(),
        };

        cluster.addons.insert(addon_name.to_string(), addon);
        Ok(cluster.addons.get(addon_name).unwrap())
    }

    pub fn describe_addon(&self, cluster_name: &str, addon_name: &str) -> Result<&Addon, EksError> {
        let cluster = self
            .clusters
            .get(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        cluster
            .addons
            .get(addon_name)
            .ok_or_else(|| EksError::AddonNotFound(addon_name.to_string()))
    }

    pub fn delete_addon(
        &mut self,
        cluster_name: &str,
        addon_name: &str,
    ) -> Result<Addon, EksError> {
        let cluster = self
            .clusters
            .get_mut(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        cluster
            .addons
            .remove(addon_name)
            .ok_or_else(|| EksError::AddonNotFound(addon_name.to_string()))
    }

    pub fn update_addon(
        &mut self,
        cluster_name: &str,
        addon_name: &str,
        addon_version: Option<&str>,
        service_account_role_arn: Option<&str>,
    ) -> Result<&Addon, EksError> {
        let cluster = self
            .clusters
            .get_mut(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        let addon = cluster
            .addons
            .get_mut(addon_name)
            .ok_or_else(|| EksError::AddonNotFound(addon_name.to_string()))?;
        if let Some(v) = addon_version {
            addon.addon_version = v.to_string();
        }
        if let Some(r) = service_account_role_arn {
            addon.service_account_role_arn = Some(r.to_string());
        }
        addon.modified_at = Utc::now();
        Ok(addon)
    }

    pub fn list_addons(&self, cluster_name: &str) -> Result<Vec<&str>, EksError> {
        let cluster = self
            .clusters
            .get(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        Ok(cluster.addons.keys().map(|k| k.as_str()).collect())
    }

    // --- Access entry operations ---

    pub fn create_access_entry(
        &mut self,
        cluster_name: &str,
        principal_arn: &str,
        kubernetes_groups: Vec<String>,
        entry_type: Option<&str>,
        username: Option<&str>,
        account_id: &str,
        region: &str,
        tags: Option<HashMap<String, String>>,
    ) -> Result<&AccessEntry, EksError> {
        let cluster = self
            .clusters
            .get_mut(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;

        if cluster.access_entries.contains_key(principal_arn) {
            return Err(EksError::AccessEntryAlreadyExists(
                principal_arn.to_string(),
            ));
        }

        let now = Utc::now();
        let access_entry_arn = format!(
            "arn:aws:eks:{region}:{account_id}:access-entry/{cluster_name}/{}/{}",
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4()
        );

        let entry = AccessEntry {
            principal_arn: principal_arn.to_string(),
            access_entry_arn,
            cluster_name: cluster_name.to_string(),
            kubernetes_groups,
            entry_type: entry_type.unwrap_or("STANDARD").to_string(),
            username: username.map(|s| s.to_string()),
            created_at: now,
            modified_at: now,
            tags: tags.unwrap_or_default(),
            associated_policies: HashMap::new(),
        };

        cluster
            .access_entries
            .insert(principal_arn.to_string(), entry);
        Ok(cluster.access_entries.get(principal_arn).unwrap())
    }

    pub fn describe_access_entry(
        &self,
        cluster_name: &str,
        principal_arn: &str,
    ) -> Result<&AccessEntry, EksError> {
        let cluster = self
            .clusters
            .get(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        cluster
            .access_entries
            .get(principal_arn)
            .ok_or_else(|| EksError::AccessEntryNotFound(principal_arn.to_string()))
    }

    pub fn delete_access_entry(
        &mut self,
        cluster_name: &str,
        principal_arn: &str,
    ) -> Result<(), EksError> {
        let cluster = self
            .clusters
            .get_mut(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        cluster
            .access_entries
            .remove(principal_arn)
            .ok_or_else(|| EksError::AccessEntryNotFound(principal_arn.to_string()))?;
        Ok(())
    }

    pub fn update_access_entry(
        &mut self,
        cluster_name: &str,
        principal_arn: &str,
        kubernetes_groups: Option<Vec<String>>,
        username: Option<&str>,
    ) -> Result<&AccessEntry, EksError> {
        let cluster = self
            .clusters
            .get_mut(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        let entry = cluster
            .access_entries
            .get_mut(principal_arn)
            .ok_or_else(|| EksError::AccessEntryNotFound(principal_arn.to_string()))?;
        if let Some(groups) = kubernetes_groups {
            entry.kubernetes_groups = groups;
        }
        if let Some(u) = username {
            entry.username = Some(u.to_string());
        }
        entry.modified_at = Utc::now();
        Ok(entry)
    }

    pub fn list_access_entries(&self, cluster_name: &str) -> Result<Vec<String>, EksError> {
        let cluster = self
            .clusters
            .get(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        Ok(cluster.access_entries.keys().cloned().collect())
    }

    pub fn associate_access_policy(
        &mut self,
        cluster_name: &str,
        principal_arn: &str,
        policy_arn: &str,
        access_scope_type: &str,
        namespaces: Vec<String>,
    ) -> Result<&AssociatedPolicy, EksError> {
        let cluster = self
            .clusters
            .get_mut(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        let entry = cluster
            .access_entries
            .get_mut(principal_arn)
            .ok_or_else(|| EksError::AccessEntryNotFound(principal_arn.to_string()))?;
        let now = Utc::now();
        let policy = AssociatedPolicy {
            policy_arn: policy_arn.to_string(),
            access_scope_type: access_scope_type.to_string(),
            namespaces,
            associated_at: now,
            modified_at: now,
        };
        entry
            .associated_policies
            .insert(policy_arn.to_string(), policy);
        Ok(entry.associated_policies.get(policy_arn).unwrap())
    }

    pub fn disassociate_access_policy(
        &mut self,
        cluster_name: &str,
        principal_arn: &str,
        policy_arn: &str,
    ) -> Result<(), EksError> {
        let cluster = self
            .clusters
            .get_mut(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        let entry = cluster
            .access_entries
            .get_mut(principal_arn)
            .ok_or_else(|| EksError::AccessEntryNotFound(principal_arn.to_string()))?;
        entry.associated_policies.remove(policy_arn);
        Ok(())
    }

    pub fn list_associated_access_policies(
        &self,
        cluster_name: &str,
        principal_arn: &str,
    ) -> Result<Vec<&AssociatedPolicy>, EksError> {
        let cluster = self
            .clusters
            .get(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        let entry = cluster
            .access_entries
            .get(principal_arn)
            .ok_or_else(|| EksError::AccessEntryNotFound(principal_arn.to_string()))?;
        Ok(entry.associated_policies.values().collect())
    }

    // --- Identity provider config operations ---

    pub fn associate_identity_provider_config(
        &mut self,
        cluster_name: &str,
        config_name: &str,
        issuer_url: &str,
        client_id: &str,
        account_id: &str,
        region: &str,
        tags: Option<HashMap<String, String>>,
    ) -> Result<&IdentityProviderConfigStore, EksError> {
        let cluster = self
            .clusters
            .get_mut(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;

        if cluster.identity_provider_configs.contains_key(config_name) {
            return Err(EksError::IdentityProviderConfigAlreadyExists(
                config_name.to_string(),
            ));
        }

        let arn = format!(
            "arn:aws:eks:{region}:{account_id}:identityproviderconfig/{cluster_name}/oidc/{config_name}/{}",
            uuid::Uuid::new_v4()
        );

        let config = IdentityProviderConfigStore {
            name: config_name.to_string(),
            config_type: "oidc".to_string(),
            cluster_name: cluster_name.to_string(),
            arn,
            issuer_url: issuer_url.to_string(),
            client_id: client_id.to_string(),
            status: "ACTIVE".to_string(),
            tags: tags.unwrap_or_default(),
        };

        cluster
            .identity_provider_configs
            .insert(config_name.to_string(), config);
        Ok(cluster.identity_provider_configs.get(config_name).unwrap())
    }

    pub fn disassociate_identity_provider_config(
        &mut self,
        cluster_name: &str,
        config_name: &str,
    ) -> Result<(), EksError> {
        let cluster = self
            .clusters
            .get_mut(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        cluster
            .identity_provider_configs
            .remove(config_name)
            .ok_or_else(|| EksError::IdentityProviderConfigNotFound(config_name.to_string()))?;
        Ok(())
    }

    pub fn describe_identity_provider_config(
        &self,
        cluster_name: &str,
        config_name: &str,
    ) -> Result<&IdentityProviderConfigStore, EksError> {
        let cluster = self
            .clusters
            .get(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        cluster
            .identity_provider_configs
            .get(config_name)
            .ok_or_else(|| EksError::IdentityProviderConfigNotFound(config_name.to_string()))
    }

    pub fn list_identity_provider_configs(
        &self,
        cluster_name: &str,
    ) -> Result<Vec<(&str, &str)>, EksError> {
        let cluster = self
            .clusters
            .get(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        Ok(cluster
            .identity_provider_configs
            .iter()
            .map(|(k, v)| (k.as_str(), v.config_type.as_str()))
            .collect())
    }

    // --- Pod identity association operations ---

    pub fn create_pod_identity_association(
        &mut self,
        cluster_name: &str,
        namespace: &str,
        service_account: &str,
        role_arn: &str,
        account_id: &str,
        region: &str,
        tags: Option<HashMap<String, String>>,
    ) -> Result<&PodIdentityAssociation, EksError> {
        let cluster = self
            .clusters
            .get_mut(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;

        let assoc_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        let arn = format!(
            "arn:aws:eks:{region}:{account_id}:podidentityassociation/{cluster_name}/{assoc_id}"
        );

        let assoc = PodIdentityAssociation {
            association_id: assoc_id.clone(),
            association_arn: arn,
            cluster_name: cluster_name.to_string(),
            namespace: namespace.to_string(),
            service_account: service_account.to_string(),
            role_arn: role_arn.to_string(),
            created_at: now,
            modified_at: now,
            tags: tags.unwrap_or_default(),
        };

        cluster
            .pod_identity_associations
            .insert(assoc_id.clone(), assoc);
        Ok(cluster.pod_identity_associations.get(&assoc_id).unwrap())
    }

    pub fn describe_pod_identity_association(
        &self,
        cluster_name: &str,
        association_id: &str,
    ) -> Result<&PodIdentityAssociation, EksError> {
        let cluster = self
            .clusters
            .get(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        cluster
            .pod_identity_associations
            .get(association_id)
            .ok_or_else(|| EksError::PodIdentityAssociationNotFound(association_id.to_string()))
    }

    pub fn delete_pod_identity_association(
        &mut self,
        cluster_name: &str,
        association_id: &str,
    ) -> Result<PodIdentityAssociation, EksError> {
        let cluster = self
            .clusters
            .get_mut(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        cluster
            .pod_identity_associations
            .remove(association_id)
            .ok_or_else(|| EksError::PodIdentityAssociationNotFound(association_id.to_string()))
    }

    pub fn update_pod_identity_association(
        &mut self,
        cluster_name: &str,
        association_id: &str,
        role_arn: Option<&str>,
    ) -> Result<&PodIdentityAssociation, EksError> {
        let cluster = self
            .clusters
            .get_mut(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        let assoc = cluster
            .pod_identity_associations
            .get_mut(association_id)
            .ok_or_else(|| EksError::PodIdentityAssociationNotFound(association_id.to_string()))?;
        if let Some(r) = role_arn {
            assoc.role_arn = r.to_string();
        }
        assoc.modified_at = Utc::now();
        Ok(assoc)
    }

    pub fn list_pod_identity_associations(
        &self,
        cluster_name: &str,
    ) -> Result<Vec<&PodIdentityAssociation>, EksError> {
        let cluster = self
            .clusters
            .get(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        Ok(cluster.pod_identity_associations.values().collect())
    }

    // --- EKS Anywhere subscription operations ---

    pub fn create_eks_anywhere_subscription(
        &mut self,
        name: &str,
        auto_renew: bool,
        license_quantity: i32,
        license_type: &str,
        account_id: &str,
        region: &str,
        tags: Option<HashMap<String, String>>,
    ) -> Result<&EksAnywhereSubscription, EksError> {
        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:eks:{region}:{account_id}:subscription/{id}");
        let sub = EksAnywhereSubscription {
            id: id.clone(),
            arn,
            name: name.to_string(),
            auto_renew,
            license_quantity,
            license_type: license_type.to_string(),
            status: "ACTIVE".to_string(),
            created_at: Utc::now(),
            tags: tags.unwrap_or_default(),
        };
        self.eks_anywhere_subscriptions.insert(id.clone(), sub);
        Ok(self.eks_anywhere_subscriptions.get(&id).unwrap())
    }

    pub fn describe_eks_anywhere_subscription(
        &self,
        id: &str,
    ) -> Result<&EksAnywhereSubscription, EksError> {
        self.eks_anywhere_subscriptions
            .get(id)
            .ok_or_else(|| EksError::EksAnywhereSubscriptionNotFound(id.to_string()))
    }

    pub fn delete_eks_anywhere_subscription(
        &mut self,
        id: &str,
    ) -> Result<EksAnywhereSubscription, EksError> {
        self.eks_anywhere_subscriptions
            .remove(id)
            .ok_or_else(|| EksError::EksAnywhereSubscriptionNotFound(id.to_string()))
    }

    pub fn update_eks_anywhere_subscription(
        &mut self,
        id: &str,
        auto_renew: bool,
    ) -> Result<&EksAnywhereSubscription, EksError> {
        let sub = self
            .eks_anywhere_subscriptions
            .get_mut(id)
            .ok_or_else(|| EksError::EksAnywhereSubscriptionNotFound(id.to_string()))?;
        sub.auto_renew = auto_renew;
        Ok(sub)
    }

    pub fn list_eks_anywhere_subscriptions(&self) -> Vec<&EksAnywhereSubscription> {
        self.eks_anywhere_subscriptions.values().collect()
    }

    // --- Capability operations ---

    pub fn create_capability(
        &mut self,
        cluster_name: &str,
        capability_name: &str,
        capability_type: &str,
        role_arn: &str,
        account_id: &str,
        region: &str,
        tags: Option<HashMap<String, String>>,
    ) -> Result<&CapabilityStore, EksError> {
        let cluster = self
            .clusters
            .get_mut(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;

        if cluster.capabilities.contains_key(capability_name) {
            return Err(EksError::CapabilityAlreadyExists(
                capability_name.to_string(),
            ));
        }

        let now = Utc::now();
        let arn = format!(
            "arn:aws:eks:{region}:{account_id}:capability/{cluster_name}/{capability_name}"
        );

        let cap = CapabilityStore {
            arn,
            capability_name: capability_name.to_string(),
            cluster_name: cluster_name.to_string(),
            capability_type: capability_type.to_string(),
            role_arn: role_arn.to_string(),
            status: "ACTIVE".to_string(),
            created_at: now,
            modified_at: now,
            tags: tags.unwrap_or_default(),
        };

        cluster
            .capabilities
            .insert(capability_name.to_string(), cap);
        Ok(cluster.capabilities.get(capability_name).unwrap())
    }

    pub fn describe_capability(
        &self,
        cluster_name: &str,
        capability_name: &str,
    ) -> Result<&CapabilityStore, EksError> {
        let cluster = self
            .clusters
            .get(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        cluster
            .capabilities
            .get(capability_name)
            .ok_or_else(|| EksError::CapabilityNotFound(capability_name.to_string()))
    }

    pub fn delete_capability(
        &mut self,
        cluster_name: &str,
        capability_name: &str,
    ) -> Result<CapabilityStore, EksError> {
        let cluster = self
            .clusters
            .get_mut(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        cluster
            .capabilities
            .remove(capability_name)
            .ok_or_else(|| EksError::CapabilityNotFound(capability_name.to_string()))
    }

    pub fn update_capability(
        &mut self,
        cluster_name: &str,
        capability_name: &str,
        role_arn: Option<&str>,
    ) -> Result<&CapabilityStore, EksError> {
        let cluster = self
            .clusters
            .get_mut(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        let cap = cluster
            .capabilities
            .get_mut(capability_name)
            .ok_or_else(|| EksError::CapabilityNotFound(capability_name.to_string()))?;
        if let Some(r) = role_arn {
            cap.role_arn = r.to_string();
        }
        cap.modified_at = Utc::now();
        Ok(cap)
    }

    pub fn list_capabilities(&self, cluster_name: &str) -> Result<Vec<&CapabilityStore>, EksError> {
        let cluster = self
            .clusters
            .get(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        Ok(cluster.capabilities.values().collect())
    }

    // --- RegisterCluster / DeregisterCluster ---

    pub fn register_cluster(
        &mut self,
        name: &str,
        connector_provider: &str,
        connector_role_arn: &str,
        account_id: &str,
        region: &str,
        tags: Option<HashMap<String, String>>,
    ) -> Result<&Cluster, EksError> {
        if self.clusters.contains_key(name) {
            return Err(EksError::ClusterAlreadyExistsPlain(name.to_string()));
        }
        let arn = format!("arn:aws:eks:{region}:{account_id}:cluster/{name}");
        let activation_code = uuid::Uuid::new_v4().to_string();
        let activation_id = uuid::Uuid::new_v4().to_string();
        let _ = (
            connector_provider,
            connector_role_arn,
            activation_code,
            activation_id,
        );
        let cluster = Cluster {
            name: name.to_string(),
            arn,
            endpoint: String::new(),
            role_arn: String::new(),
            status: "PENDING".to_string(),
            version: "1.28".to_string(),
            created_at: Utc::now(),
            nodegroups: HashMap::new(),
            fargate_profiles: HashMap::new(),
            addons: HashMap::new(),
            access_entries: HashMap::new(),
            identity_provider_configs: HashMap::new(),
            pod_identity_associations: HashMap::new(),
            capabilities: HashMap::new(),
            tags: tags.unwrap_or_default(),
            updates: HashMap::new(),
        };
        self.clusters.insert(name.to_string(), cluster);
        Ok(self.clusters.get(name).unwrap())
    }

    pub fn deregister_cluster(&mut self, name: &str) -> Result<Cluster, EksError> {
        self.clusters
            .remove(name)
            .ok_or_else(|| cluster_not_found(name))
    }

    // --- Update tracking operations ---

    pub fn record_update(
        &mut self,
        cluster_name: &str,
        update_type: &str,
    ) -> Result<&crate::types::UpdateRecord, EksError> {
        let cluster = self
            .clusters
            .get_mut(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        let id = uuid::Uuid::new_v4().to_string();
        let record = crate::types::UpdateRecord {
            id: id.clone(),
            cluster_name: cluster_name.to_string(),
            update_type: update_type.to_string(),
            status: "Successful".to_string(),
            created_at: Utc::now(),
        };
        cluster.updates.insert(id.clone(), record);
        Ok(cluster.updates.get(&id).unwrap())
    }

    pub fn describe_update(
        &self,
        cluster_name: &str,
        update_id: &str,
    ) -> Result<&crate::types::UpdateRecord, EksError> {
        let cluster = self
            .clusters
            .get(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        cluster
            .updates
            .get(update_id)
            .ok_or_else(|| EksError::ClusterNotFound(format!("Update {update_id} not found")))
    }

    pub fn list_updates(&self, cluster_name: &str) -> Result<Vec<&str>, EksError> {
        let cluster = self
            .clusters
            .get(cluster_name)
            .ok_or_else(|| cluster_not_found(cluster_name))?;
        Ok(cluster.updates.keys().map(|k| k.as_str()).collect())
    }
}

fn cluster_not_found(name: &str) -> EksError {
    EksError::ClusterNotFound(name.to_string())
}
