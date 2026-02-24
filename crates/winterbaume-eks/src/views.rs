//! Serde-compatible view types for EKS state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::EksService;
use crate::state::EksState;
use crate::types::{
    AccessEntry, Addon, AssociatedPolicy, CapabilityStore, Cluster, EksAnywhereSubscription,
    FargateProfile, FargateSelector, IdentityProviderConfigStore, Nodegroup,
    PodIdentityAssociation, ScalingConfig, Taint,
};

/// Serializable view of the entire EKS state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EksStateView {
    /// Clusters keyed by cluster name.
    #[serde(default)]
    pub clusters: HashMap<String, ClusterView>,
    /// EKS Anywhere subscriptions keyed by subscription ID.
    #[serde(default)]
    pub eks_anywhere_subscriptions: HashMap<String, EksAnywhereSubscriptionView>,
    /// Tags for resources keyed by ARN.
    #[serde(default)]
    pub resource_tags: HashMap<String, HashMap<String, String>>,
}

/// Serializable view of an EKS cluster.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClusterView {
    pub name: String,
    pub arn: String,
    pub endpoint: String,
    pub role_arn: String,
    pub status: String,
    pub version: String,
    pub created_at: Option<String>,
    #[serde(default)]
    pub nodegroups: HashMap<String, NodegroupView>,
    #[serde(default)]
    pub fargate_profiles: HashMap<String, FargateProfileView>,
    #[serde(default)]
    pub addons: HashMap<String, AddonView>,
    #[serde(default)]
    pub access_entries: HashMap<String, AccessEntryView>,
    #[serde(default)]
    pub identity_provider_configs: HashMap<String, IdentityProviderConfigView>,
    #[serde(default)]
    pub pod_identity_associations: HashMap<String, PodIdentityAssociationView>,
    #[serde(default)]
    pub capabilities: HashMap<String, CapabilityView>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub updates: HashMap<String, UpdateRecordView>,
}

/// Serializable view of an update record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRecordView {
    pub id: String,
    pub cluster_name: String,
    pub update_type: String,
    pub status: String,
    pub created_at: String,
}

/// Serializable view of an EKS addon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddonView {
    pub addon_name: String,
    pub addon_arn: String,
    pub cluster_name: String,
    pub addon_version: String,
    pub service_account_role_arn: Option<String>,
    pub status: String,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of an EKS access entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessEntryView {
    pub principal_arn: String,
    pub access_entry_arn: String,
    pub cluster_name: String,
    #[serde(default)]
    pub kubernetes_groups: Vec<String>,
    pub entry_type: String,
    pub username: Option<String>,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub associated_policies: HashMap<String, AssociatedPolicyView>,
}

/// Serializable view of an associated access policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssociatedPolicyView {
    pub policy_arn: String,
    pub access_scope_type: String,
    #[serde(default)]
    pub namespaces: Vec<String>,
    pub associated_at: Option<String>,
    pub modified_at: Option<String>,
}

/// Serializable view of an identity provider config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityProviderConfigView {
    pub name: String,
    pub config_type: String,
    pub cluster_name: String,
    pub arn: String,
    pub issuer_url: String,
    pub client_id: String,
    pub status: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of a pod identity association.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodIdentityAssociationView {
    pub association_id: String,
    pub association_arn: String,
    pub cluster_name: String,
    pub namespace: String,
    pub service_account: String,
    pub role_arn: String,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of an EKS capability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityView {
    pub arn: String,
    pub capability_name: String,
    pub cluster_name: String,
    pub capability_type: String,
    pub role_arn: String,
    pub status: String,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of an EKS Anywhere subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EksAnywhereSubscriptionView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub auto_renew: bool,
    pub license_quantity: i32,
    pub license_type: String,
    pub status: String,
    pub created_at: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of an EKS node group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodegroupView {
    pub name: String,
    pub arn: String,
    pub cluster_name: String,
    pub node_role: String,
    pub scaling_config: ScalingConfigView,
    pub status: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    #[serde(default)]
    pub taints: Vec<TaintView>,
}

/// Serializable view of an EKS scaling config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingConfigView {
    pub min_size: i32,
    pub max_size: i32,
    pub desired_size: i32,
}

/// Serializable view of an EKS taint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaintView {
    pub key: String,
    pub value: Option<String>,
    pub effect: String,
}

/// Serializable view of an EKS Fargate profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FargateProfileView {
    pub name: String,
    pub arn: String,
    pub cluster_name: String,
    pub pod_execution_role_arn: String,
    #[serde(default)]
    pub selectors: Vec<FargateSelectorView>,
    pub status: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of an EKS Fargate selector.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FargateSelectorView {
    pub namespace: String,
    #[serde(default)]
    pub labels: HashMap<String, String>,
}

// --- From internal types to view types ---

impl From<&EksState> for EksStateView {
    fn from(state: &EksState) -> Self {
        EksStateView {
            clusters: state
                .clusters
                .iter()
                .map(|(k, v)| (k.clone(), ClusterView::from(v)))
                .collect(),
            eks_anywhere_subscriptions: state
                .eks_anywhere_subscriptions
                .iter()
                .map(|(k, v)| (k.clone(), EksAnywhereSubscriptionView::from(v)))
                .collect(),
            resource_tags: state.resource_tags.clone(),
        }
    }
}

impl From<&Cluster> for ClusterView {
    fn from(c: &Cluster) -> Self {
        ClusterView {
            name: c.name.clone(),
            arn: c.arn.clone(),
            endpoint: c.endpoint.clone(),
            role_arn: c.role_arn.clone(),
            status: c.status.clone(),
            version: c.version.clone(),
            created_at: Some(c.created_at.to_rfc3339()),
            nodegroups: c
                .nodegroups
                .iter()
                .map(|(k, v)| (k.clone(), NodegroupView::from(v)))
                .collect(),
            fargate_profiles: c
                .fargate_profiles
                .iter()
                .map(|(k, v)| (k.clone(), FargateProfileView::from(v)))
                .collect(),
            addons: c
                .addons
                .iter()
                .map(|(k, v)| (k.clone(), AddonView::from(v)))
                .collect(),
            access_entries: c
                .access_entries
                .iter()
                .map(|(k, v)| (k.clone(), AccessEntryView::from(v)))
                .collect(),
            identity_provider_configs: c
                .identity_provider_configs
                .iter()
                .map(|(k, v)| (k.clone(), IdentityProviderConfigView::from(v)))
                .collect(),
            pod_identity_associations: c
                .pod_identity_associations
                .iter()
                .map(|(k, v)| (k.clone(), PodIdentityAssociationView::from(v)))
                .collect(),
            capabilities: c
                .capabilities
                .iter()
                .map(|(k, v)| (k.clone(), CapabilityView::from(v)))
                .collect(),
            tags: c.tags.clone(),
            updates: c
                .updates
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        UpdateRecordView {
                            id: v.id.clone(),
                            cluster_name: v.cluster_name.clone(),
                            update_type: v.update_type.clone(),
                            status: v.status.clone(),
                            created_at: v.created_at.to_rfc3339(),
                        },
                    )
                })
                .collect(),
        }
    }
}

impl From<&Addon> for AddonView {
    fn from(a: &Addon) -> Self {
        AddonView {
            addon_name: a.addon_name.clone(),
            addon_arn: a.addon_arn.clone(),
            cluster_name: a.cluster_name.clone(),
            addon_version: a.addon_version.clone(),
            service_account_role_arn: a.service_account_role_arn.clone(),
            status: a.status.clone(),
            created_at: Some(a.created_at.to_rfc3339()),
            modified_at: Some(a.modified_at.to_rfc3339()),
            tags: a.tags.clone(),
        }
    }
}

impl From<&AccessEntry> for AccessEntryView {
    fn from(e: &AccessEntry) -> Self {
        AccessEntryView {
            principal_arn: e.principal_arn.clone(),
            access_entry_arn: e.access_entry_arn.clone(),
            cluster_name: e.cluster_name.clone(),
            kubernetes_groups: e.kubernetes_groups.clone(),
            entry_type: e.entry_type.clone(),
            username: e.username.clone(),
            created_at: Some(e.created_at.to_rfc3339()),
            modified_at: Some(e.modified_at.to_rfc3339()),
            tags: e.tags.clone(),
            associated_policies: e
                .associated_policies
                .iter()
                .map(|(k, v)| (k.clone(), AssociatedPolicyView::from(v)))
                .collect(),
        }
    }
}

impl From<&AssociatedPolicy> for AssociatedPolicyView {
    fn from(p: &AssociatedPolicy) -> Self {
        AssociatedPolicyView {
            policy_arn: p.policy_arn.clone(),
            access_scope_type: p.access_scope_type.clone(),
            namespaces: p.namespaces.clone(),
            associated_at: Some(p.associated_at.to_rfc3339()),
            modified_at: Some(p.modified_at.to_rfc3339()),
        }
    }
}

impl From<&IdentityProviderConfigStore> for IdentityProviderConfigView {
    fn from(c: &IdentityProviderConfigStore) -> Self {
        IdentityProviderConfigView {
            name: c.name.clone(),
            config_type: c.config_type.clone(),
            cluster_name: c.cluster_name.clone(),
            arn: c.arn.clone(),
            issuer_url: c.issuer_url.clone(),
            client_id: c.client_id.clone(),
            status: c.status.clone(),
            tags: c.tags.clone(),
        }
    }
}

impl From<&PodIdentityAssociation> for PodIdentityAssociationView {
    fn from(a: &PodIdentityAssociation) -> Self {
        PodIdentityAssociationView {
            association_id: a.association_id.clone(),
            association_arn: a.association_arn.clone(),
            cluster_name: a.cluster_name.clone(),
            namespace: a.namespace.clone(),
            service_account: a.service_account.clone(),
            role_arn: a.role_arn.clone(),
            created_at: Some(a.created_at.to_rfc3339()),
            modified_at: Some(a.modified_at.to_rfc3339()),
            tags: a.tags.clone(),
        }
    }
}

impl From<&CapabilityStore> for CapabilityView {
    fn from(c: &CapabilityStore) -> Self {
        CapabilityView {
            arn: c.arn.clone(),
            capability_name: c.capability_name.clone(),
            cluster_name: c.cluster_name.clone(),
            capability_type: c.capability_type.clone(),
            role_arn: c.role_arn.clone(),
            status: c.status.clone(),
            created_at: Some(c.created_at.to_rfc3339()),
            modified_at: Some(c.modified_at.to_rfc3339()),
            tags: c.tags.clone(),
        }
    }
}

impl From<&EksAnywhereSubscription> for EksAnywhereSubscriptionView {
    fn from(s: &EksAnywhereSubscription) -> Self {
        EksAnywhereSubscriptionView {
            id: s.id.clone(),
            arn: s.arn.clone(),
            name: s.name.clone(),
            auto_renew: s.auto_renew,
            license_quantity: s.license_quantity,
            license_type: s.license_type.clone(),
            status: s.status.clone(),
            created_at: Some(s.created_at.to_rfc3339()),
            tags: s.tags.clone(),
        }
    }
}

impl From<&Nodegroup> for NodegroupView {
    fn from(ng: &Nodegroup) -> Self {
        NodegroupView {
            name: ng.name.clone(),
            arn: ng.arn.clone(),
            cluster_name: ng.cluster_name.clone(),
            node_role: ng.node_role.clone(),
            scaling_config: ScalingConfigView::from(&ng.scaling_config),
            status: ng.status.clone(),
            tags: ng.tags.clone(),
            labels: ng.labels.clone(),
            taints: ng.taints.iter().map(TaintView::from).collect(),
        }
    }
}

impl From<&ScalingConfig> for ScalingConfigView {
    fn from(sc: &ScalingConfig) -> Self {
        ScalingConfigView {
            min_size: sc.min_size,
            max_size: sc.max_size,
            desired_size: sc.desired_size,
        }
    }
}

impl From<&Taint> for TaintView {
    fn from(t: &Taint) -> Self {
        TaintView {
            key: t.key.clone(),
            value: t.value.clone(),
            effect: t.effect.clone(),
        }
    }
}

impl From<&FargateProfile> for FargateProfileView {
    fn from(fp: &FargateProfile) -> Self {
        FargateProfileView {
            name: fp.name.clone(),
            arn: fp.arn.clone(),
            cluster_name: fp.cluster_name.clone(),
            pod_execution_role_arn: fp.pod_execution_role_arn.clone(),
            selectors: fp.selectors.iter().map(FargateSelectorView::from).collect(),
            status: fp.status.clone(),
            tags: fp.tags.clone(),
        }
    }
}

impl From<&FargateSelector> for FargateSelectorView {
    fn from(fs: &FargateSelector) -> Self {
        FargateSelectorView {
            namespace: fs.namespace.clone(),
            labels: fs.labels.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<EksStateView> for EksState {
    fn from(view: EksStateView) -> Self {
        EksState {
            clusters: view
                .clusters
                .into_iter()
                .map(|(k, v)| (k, Cluster::from(v)))
                .collect(),
            eks_anywhere_subscriptions: view
                .eks_anywhere_subscriptions
                .into_iter()
                .map(|(k, v)| (k, EksAnywhereSubscription::from(v)))
                .collect(),
            resource_tags: view.resource_tags,
        }
    }
}

impl From<ClusterView> for Cluster {
    fn from(view: ClusterView) -> Self {
        let created_at = view
            .created_at
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        Cluster {
            name: view.name,
            arn: view.arn,
            endpoint: view.endpoint,
            role_arn: view.role_arn,
            status: view.status,
            version: view.version,
            created_at,
            nodegroups: view
                .nodegroups
                .into_iter()
                .map(|(k, v)| (k, Nodegroup::from(v)))
                .collect(),
            fargate_profiles: view
                .fargate_profiles
                .into_iter()
                .map(|(k, v)| (k, FargateProfile::from(v)))
                .collect(),
            addons: view
                .addons
                .into_iter()
                .map(|(k, v)| (k, Addon::from(v)))
                .collect(),
            access_entries: view
                .access_entries
                .into_iter()
                .map(|(k, v)| (k, AccessEntry::from(v)))
                .collect(),
            identity_provider_configs: view
                .identity_provider_configs
                .into_iter()
                .map(|(k, v)| (k, IdentityProviderConfigStore::from(v)))
                .collect(),
            pod_identity_associations: view
                .pod_identity_associations
                .into_iter()
                .map(|(k, v)| (k, PodIdentityAssociation::from(v)))
                .collect(),
            capabilities: view
                .capabilities
                .into_iter()
                .map(|(k, v)| (k, CapabilityStore::from(v)))
                .collect(),
            tags: view.tags,
            updates: view
                .updates
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        crate::types::UpdateRecord {
                            id: v.id,
                            cluster_name: v.cluster_name,
                            update_type: v.update_type,
                            status: v.status,
                            created_at: v
                                .created_at
                                .parse::<DateTime<Utc>>()
                                .unwrap_or_else(|_| Utc::now()),
                        },
                    )
                })
                .collect(),
        }
    }
}

fn parse_dt(s: Option<&str>) -> DateTime<Utc> {
    s.and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(Utc::now)
}

impl From<AddonView> for Addon {
    fn from(v: AddonView) -> Self {
        Addon {
            addon_name: v.addon_name,
            addon_arn: v.addon_arn,
            cluster_name: v.cluster_name,
            addon_version: v.addon_version,
            service_account_role_arn: v.service_account_role_arn,
            status: v.status,
            created_at: parse_dt(v.created_at.as_deref()),
            modified_at: parse_dt(v.modified_at.as_deref()),
            tags: v.tags,
        }
    }
}

impl From<AccessEntryView> for AccessEntry {
    fn from(v: AccessEntryView) -> Self {
        AccessEntry {
            principal_arn: v.principal_arn,
            access_entry_arn: v.access_entry_arn,
            cluster_name: v.cluster_name,
            kubernetes_groups: v.kubernetes_groups,
            entry_type: v.entry_type,
            username: v.username,
            created_at: parse_dt(v.created_at.as_deref()),
            modified_at: parse_dt(v.modified_at.as_deref()),
            tags: v.tags,
            associated_policies: v
                .associated_policies
                .into_iter()
                .map(|(k, p)| (k, AssociatedPolicy::from(p)))
                .collect(),
        }
    }
}

impl From<AssociatedPolicyView> for AssociatedPolicy {
    fn from(v: AssociatedPolicyView) -> Self {
        AssociatedPolicy {
            policy_arn: v.policy_arn,
            access_scope_type: v.access_scope_type,
            namespaces: v.namespaces,
            associated_at: parse_dt(v.associated_at.as_deref()),
            modified_at: parse_dt(v.modified_at.as_deref()),
        }
    }
}

impl From<IdentityProviderConfigView> for IdentityProviderConfigStore {
    fn from(v: IdentityProviderConfigView) -> Self {
        IdentityProviderConfigStore {
            name: v.name,
            config_type: v.config_type,
            cluster_name: v.cluster_name,
            arn: v.arn,
            issuer_url: v.issuer_url,
            client_id: v.client_id,
            status: v.status,
            tags: v.tags,
        }
    }
}

impl From<PodIdentityAssociationView> for PodIdentityAssociation {
    fn from(v: PodIdentityAssociationView) -> Self {
        PodIdentityAssociation {
            association_id: v.association_id,
            association_arn: v.association_arn,
            cluster_name: v.cluster_name,
            namespace: v.namespace,
            service_account: v.service_account,
            role_arn: v.role_arn,
            created_at: parse_dt(v.created_at.as_deref()),
            modified_at: parse_dt(v.modified_at.as_deref()),
            tags: v.tags,
        }
    }
}

impl From<CapabilityView> for CapabilityStore {
    fn from(v: CapabilityView) -> Self {
        CapabilityStore {
            arn: v.arn,
            capability_name: v.capability_name,
            cluster_name: v.cluster_name,
            capability_type: v.capability_type,
            role_arn: v.role_arn,
            status: v.status,
            created_at: parse_dt(v.created_at.as_deref()),
            modified_at: parse_dt(v.modified_at.as_deref()),
            tags: v.tags,
        }
    }
}

impl From<EksAnywhereSubscriptionView> for EksAnywhereSubscription {
    fn from(v: EksAnywhereSubscriptionView) -> Self {
        EksAnywhereSubscription {
            id: v.id,
            arn: v.arn,
            name: v.name,
            auto_renew: v.auto_renew,
            license_quantity: v.license_quantity,
            license_type: v.license_type,
            status: v.status,
            created_at: parse_dt(v.created_at.as_deref()),
            tags: v.tags,
        }
    }
}

impl From<NodegroupView> for Nodegroup {
    fn from(view: NodegroupView) -> Self {
        Nodegroup {
            name: view.name,
            arn: view.arn,
            cluster_name: view.cluster_name,
            node_role: view.node_role,
            scaling_config: ScalingConfig::from(view.scaling_config),
            status: view.status,
            tags: view.tags,
            labels: view.labels,
            taints: view.taints.into_iter().map(Taint::from).collect(),
        }
    }
}

impl From<ScalingConfigView> for ScalingConfig {
    fn from(view: ScalingConfigView) -> Self {
        ScalingConfig {
            min_size: view.min_size,
            max_size: view.max_size,
            desired_size: view.desired_size,
        }
    }
}

impl From<TaintView> for Taint {
    fn from(view: TaintView) -> Self {
        Taint {
            key: view.key,
            value: view.value,
            effect: view.effect,
        }
    }
}

impl From<FargateProfileView> for FargateProfile {
    fn from(view: FargateProfileView) -> Self {
        FargateProfile {
            name: view.name,
            arn: view.arn,
            cluster_name: view.cluster_name,
            pod_execution_role_arn: view.pod_execution_role_arn,
            selectors: view
                .selectors
                .into_iter()
                .map(FargateSelector::from)
                .collect(),
            status: view.status,
            tags: view.tags,
        }
    }
}

impl From<FargateSelectorView> for FargateSelector {
    fn from(view: FargateSelectorView) -> Self {
        FargateSelector {
            namespace: view.namespace,
            labels: view.labels,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for EksService {
    type StateView = EksStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        EksStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = EksState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (name, cluster_view) in view.clusters {
                guard.clusters.insert(name, Cluster::from(cluster_view));
            }
            for (id, sub_view) in view.eks_anywhere_subscriptions {
                guard
                    .eks_anywhere_subscriptions
                    .insert(id, EksAnywhereSubscription::from(sub_view));
            }
            for (arn, tags) in view.resource_tags {
                guard.resource_tags.entry(arn).or_default().extend(tags);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
