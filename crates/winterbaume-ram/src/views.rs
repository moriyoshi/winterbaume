//! Serde-compatible view types for RAM state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::RamService;
use crate::state::RamState;

/// Serializable view of a tag.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TagView {
    pub key: String,
    pub value: String,
}

/// Serializable view of a resource share.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceShareView {
    pub resource_share_arn: String,
    pub name: String,
    pub owning_account_id: String,
    pub allow_external_principals: bool,
    pub status: String,
    pub creation_time: f64,
    pub last_updated_time: f64,
    #[serde(default)]
    pub tags: Vec<TagView>,
}

/// Serializable view of a resource.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceView {
    pub arn: String,
    pub r#type: String,
    pub resource_share_arn: String,
    pub status: String,
    pub creation_time: f64,
    pub last_updated_time: f64,
}

/// Serializable view of a resource share association.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceShareAssociationView {
    pub resource_share_arn: String,
    pub resource_share_name: String,
    pub associated_entity: String,
    pub association_type: String,
    pub status: String,
    pub creation_time: f64,
    pub last_updated_time: f64,
    pub external: bool,
}

/// Serializable view of a customer-managed permission.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerManagedPermissionView {
    pub arn: String,
    pub name: String,
    pub resource_type: String,
    pub policy_template: String,
    pub default_version: bool,
    pub version: String,
    pub status: String,
    pub creation_time: f64,
    pub last_updated_time: f64,
    pub permission_type: String,
    #[serde(default)]
    pub tags: Vec<TagView>,
}

/// Serializable view of a resource-share permission entry.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceSharePermissionEntryView {
    pub permission_arn: String,
    pub permission_version: String,
    pub resource_share_arn: String,
    pub resource_type: String,
    pub default_version: bool,
    pub last_updated_time: f64,
    pub status: String,
}

/// Serializable view of a replace-permission-associations work item.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReplacePermissionWorkView {
    pub id: String,
    pub from_permission_arn: String,
    pub from_permission_version: String,
    pub to_permission_arn: String,
    pub to_permission_version: String,
    pub status: String,
    pub creation_time: f64,
    pub last_updated_time: f64,
}

/// Serializable view of a resource share invitation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceShareInvitationView {
    pub invitation_arn: String,
    pub resource_share_arn: String,
    pub resource_share_name: String,
    pub sender_account_id: String,
    pub receiver_account_id: String,
    pub status: String,
    pub invitation_timestamp: f64,
}

/// Serializable view of the entire RAM state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RamStateView {
    #[serde(default)]
    pub resource_shares: HashMap<String, ResourceShareView>,
    #[serde(default)]
    pub resources: Vec<ResourceView>,
    #[serde(default)]
    pub associations: Vec<ResourceShareAssociationView>,
    #[serde(default)]
    pub sharing_with_org_enabled: bool,
    #[serde(default)]
    pub customer_managed_permissions: HashMap<String, CustomerManagedPermissionView>,
    #[serde(default)]
    pub share_permissions: Vec<ResourceSharePermissionEntryView>,
    #[serde(default)]
    pub replace_permission_works: Vec<ReplacePermissionWorkView>,
    #[serde(default)]
    pub invitations: Vec<ResourceShareInvitationView>,
}

// --- From internal types to view types ---

impl From<&crate::types::Tag> for TagView {
    fn from(t: &crate::types::Tag) -> Self {
        TagView {
            key: t.key.clone(),
            value: t.value.clone(),
        }
    }
}

impl From<&crate::types::ResourceShare> for ResourceShareView {
    fn from(rs: &crate::types::ResourceShare) -> Self {
        ResourceShareView {
            resource_share_arn: rs.resource_share_arn.clone(),
            name: rs.name.clone(),
            owning_account_id: rs.owning_account_id.clone(),
            allow_external_principals: rs.allow_external_principals,
            status: rs.status.clone(),
            creation_time: rs.creation_time,
            last_updated_time: rs.last_updated_time,
            tags: rs.tags.iter().map(TagView::from).collect(),
        }
    }
}

impl From<&crate::types::Resource> for ResourceView {
    fn from(r: &crate::types::Resource) -> Self {
        ResourceView {
            arn: r.arn.clone(),
            r#type: r.r#type.clone(),
            resource_share_arn: r.resource_share_arn.clone(),
            status: r.status.clone(),
            creation_time: r.creation_time,
            last_updated_time: r.last_updated_time,
        }
    }
}

impl From<&crate::types::ResourceShareAssociation> for ResourceShareAssociationView {
    fn from(a: &crate::types::ResourceShareAssociation) -> Self {
        ResourceShareAssociationView {
            resource_share_arn: a.resource_share_arn.clone(),
            resource_share_name: a.resource_share_name.clone(),
            associated_entity: a.associated_entity.clone(),
            association_type: a.association_type.clone(),
            status: a.status.clone(),
            creation_time: a.creation_time,
            last_updated_time: a.last_updated_time,
            external: a.external,
        }
    }
}

impl From<&crate::types::CustomerManagedPermission> for CustomerManagedPermissionView {
    fn from(p: &crate::types::CustomerManagedPermission) -> Self {
        CustomerManagedPermissionView {
            arn: p.arn.clone(),
            name: p.name.clone(),
            resource_type: p.resource_type.clone(),
            policy_template: p.policy_template.clone(),
            default_version: p.default_version,
            version: p.version.clone(),
            status: p.status.clone(),
            creation_time: p.creation_time,
            last_updated_time: p.last_updated_time,
            permission_type: p.permission_type.clone(),
            tags: p.tags.iter().map(TagView::from).collect(),
        }
    }
}

impl From<CustomerManagedPermissionView> for crate::types::CustomerManagedPermission {
    fn from(v: CustomerManagedPermissionView) -> Self {
        crate::types::CustomerManagedPermission {
            arn: v.arn,
            name: v.name,
            resource_type: v.resource_type,
            policy_template: v.policy_template,
            default_version: v.default_version,
            version: v.version,
            status: v.status,
            creation_time: v.creation_time,
            last_updated_time: v.last_updated_time,
            permission_type: v.permission_type,
            tags: v.tags.into_iter().map(crate::types::Tag::from).collect(),
        }
    }
}

impl From<&crate::types::ResourceSharePermissionEntry> for ResourceSharePermissionEntryView {
    fn from(e: &crate::types::ResourceSharePermissionEntry) -> Self {
        ResourceSharePermissionEntryView {
            permission_arn: e.permission_arn.clone(),
            permission_version: e.permission_version.clone(),
            resource_share_arn: e.resource_share_arn.clone(),
            resource_type: e.resource_type.clone(),
            default_version: e.default_version,
            last_updated_time: e.last_updated_time,
            status: e.status.clone(),
        }
    }
}

impl From<ResourceSharePermissionEntryView> for crate::types::ResourceSharePermissionEntry {
    fn from(v: ResourceSharePermissionEntryView) -> Self {
        crate::types::ResourceSharePermissionEntry {
            permission_arn: v.permission_arn,
            permission_version: v.permission_version,
            resource_share_arn: v.resource_share_arn,
            resource_type: v.resource_type,
            default_version: v.default_version,
            last_updated_time: v.last_updated_time,
            status: v.status,
        }
    }
}

impl From<&crate::types::ReplacePermissionWork> for ReplacePermissionWorkView {
    fn from(w: &crate::types::ReplacePermissionWork) -> Self {
        ReplacePermissionWorkView {
            id: w.id.clone(),
            from_permission_arn: w.from_permission_arn.clone(),
            from_permission_version: w.from_permission_version.clone(),
            to_permission_arn: w.to_permission_arn.clone(),
            to_permission_version: w.to_permission_version.clone(),
            status: w.status.clone(),
            creation_time: w.creation_time,
            last_updated_time: w.last_updated_time,
        }
    }
}

impl From<ReplacePermissionWorkView> for crate::types::ReplacePermissionWork {
    fn from(v: ReplacePermissionWorkView) -> Self {
        crate::types::ReplacePermissionWork {
            id: v.id,
            from_permission_arn: v.from_permission_arn,
            from_permission_version: v.from_permission_version,
            to_permission_arn: v.to_permission_arn,
            to_permission_version: v.to_permission_version,
            status: v.status,
            creation_time: v.creation_time,
            last_updated_time: v.last_updated_time,
        }
    }
}

impl From<&crate::types::ResourceShareInvitation> for ResourceShareInvitationView {
    fn from(i: &crate::types::ResourceShareInvitation) -> Self {
        ResourceShareInvitationView {
            invitation_arn: i.invitation_arn.clone(),
            resource_share_arn: i.resource_share_arn.clone(),
            resource_share_name: i.resource_share_name.clone(),
            sender_account_id: i.sender_account_id.clone(),
            receiver_account_id: i.receiver_account_id.clone(),
            status: i.status.clone(),
            invitation_timestamp: i.invitation_timestamp,
        }
    }
}

impl From<ResourceShareInvitationView> for crate::types::ResourceShareInvitation {
    fn from(v: ResourceShareInvitationView) -> Self {
        crate::types::ResourceShareInvitation {
            invitation_arn: v.invitation_arn,
            resource_share_arn: v.resource_share_arn,
            resource_share_name: v.resource_share_name,
            sender_account_id: v.sender_account_id,
            receiver_account_id: v.receiver_account_id,
            status: v.status,
            invitation_timestamp: v.invitation_timestamp,
        }
    }
}

impl From<&RamState> for RamStateView {
    fn from(state: &RamState) -> Self {
        RamStateView {
            resource_shares: state
                .resource_shares
                .iter()
                .map(|(k, v)| (k.clone(), ResourceShareView::from(v)))
                .collect(),
            resources: state.resources.iter().map(ResourceView::from).collect(),
            associations: state
                .associations
                .iter()
                .map(ResourceShareAssociationView::from)
                .collect(),
            sharing_with_org_enabled: state.sharing_with_org_enabled,
            customer_managed_permissions: state
                .customer_managed_permissions
                .iter()
                .map(|(k, v)| (k.clone(), CustomerManagedPermissionView::from(v)))
                .collect(),
            share_permissions: state
                .share_permissions
                .iter()
                .map(ResourceSharePermissionEntryView::from)
                .collect(),
            replace_permission_works: state
                .replace_permission_works
                .iter()
                .map(ReplacePermissionWorkView::from)
                .collect(),
            invitations: state
                .invitations
                .iter()
                .map(ResourceShareInvitationView::from)
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<TagView> for crate::types::Tag {
    fn from(v: TagView) -> Self {
        crate::types::Tag {
            key: v.key,
            value: v.value,
        }
    }
}

impl From<ResourceShareView> for crate::types::ResourceShare {
    fn from(v: ResourceShareView) -> Self {
        crate::types::ResourceShare {
            resource_share_arn: v.resource_share_arn,
            name: v.name,
            owning_account_id: v.owning_account_id,
            allow_external_principals: v.allow_external_principals,
            status: v.status,
            creation_time: v.creation_time,
            last_updated_time: v.last_updated_time,
            tags: v.tags.into_iter().map(crate::types::Tag::from).collect(),
        }
    }
}

impl From<ResourceView> for crate::types::Resource {
    fn from(v: ResourceView) -> Self {
        crate::types::Resource {
            arn: v.arn,
            r#type: v.r#type,
            resource_share_arn: v.resource_share_arn,
            status: v.status,
            creation_time: v.creation_time,
            last_updated_time: v.last_updated_time,
        }
    }
}

impl From<ResourceShareAssociationView> for crate::types::ResourceShareAssociation {
    fn from(v: ResourceShareAssociationView) -> Self {
        crate::types::ResourceShareAssociation {
            resource_share_arn: v.resource_share_arn,
            resource_share_name: v.resource_share_name,
            associated_entity: v.associated_entity,
            association_type: v.association_type,
            status: v.status,
            creation_time: v.creation_time,
            last_updated_time: v.last_updated_time,
            external: v.external,
        }
    }
}

impl From<RamStateView> for RamState {
    fn from(view: RamStateView) -> Self {
        RamState {
            resource_shares: view
                .resource_shares
                .into_iter()
                .map(|(k, v)| (k, crate::types::ResourceShare::from(v)))
                .collect(),
            resources: view
                .resources
                .into_iter()
                .map(crate::types::Resource::from)
                .collect(),
            associations: view
                .associations
                .into_iter()
                .map(crate::types::ResourceShareAssociation::from)
                .collect(),
            sharing_with_org_enabled: view.sharing_with_org_enabled,
            customer_managed_permissions: view
                .customer_managed_permissions
                .into_iter()
                .map(|(k, v)| (k, crate::types::CustomerManagedPermission::from(v)))
                .collect(),
            share_permissions: view
                .share_permissions
                .into_iter()
                .map(crate::types::ResourceSharePermissionEntry::from)
                .collect(),
            replace_permission_works: view
                .replace_permission_works
                .into_iter()
                .map(crate::types::ReplacePermissionWork::from)
                .collect(),
            invitations: view
                .invitations
                .into_iter()
                .map(crate::types::ResourceShareInvitation::from)
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for RamService {
    type StateView = RamStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        RamStateView::from(&*guard)
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
            *guard = RamState::from(view);
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
            for (arn, rs_view) in view.resource_shares {
                guard
                    .resource_shares
                    .insert(arn, crate::types::ResourceShare::from(rs_view));
            }
            for resource_view in view.resources {
                let resource = crate::types::Resource::from(resource_view);
                if !guard.resources.iter().any(|r| r.arn == resource.arn) {
                    guard.resources.push(resource);
                }
            }
            for assoc_view in view.associations {
                let assoc = crate::types::ResourceShareAssociation::from(assoc_view);
                if !guard.associations.iter().any(|a| {
                    a.resource_share_arn == assoc.resource_share_arn
                        && a.associated_entity == assoc.associated_entity
                        && a.association_type == assoc.association_type
                }) {
                    guard.associations.push(assoc);
                }
            }
            if view.sharing_with_org_enabled {
                guard.sharing_with_org_enabled = true;
            }
            for (arn, perm_view) in view.customer_managed_permissions {
                guard.customer_managed_permissions.insert(
                    arn,
                    crate::types::CustomerManagedPermission::from(perm_view),
                );
            }
            for sp_view in view.share_permissions {
                let sp = crate::types::ResourceSharePermissionEntry::from(sp_view);
                if !guard.share_permissions.iter().any(|e| {
                    e.resource_share_arn == sp.resource_share_arn
                        && e.permission_arn == sp.permission_arn
                }) {
                    guard.share_permissions.push(sp);
                }
            }
            for work_view in view.replace_permission_works {
                let work = crate::types::ReplacePermissionWork::from(work_view);
                if !guard
                    .replace_permission_works
                    .iter()
                    .any(|w| w.id == work.id)
                {
                    guard.replace_permission_works.push(work);
                }
            }
            for inv_view in view.invitations {
                let inv = crate::types::ResourceShareInvitation::from(inv_view);
                if !guard
                    .invitations
                    .iter()
                    .any(|i| i.invitation_arn == inv.invitation_arn)
                {
                    guard.invitations.push(inv);
                }
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
