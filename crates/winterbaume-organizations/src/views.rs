//! Serde-compatible view types for Organizations state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::OrganizationsService;
use crate::state::OrganizationsState;
use crate::types::{
    Account, DelegatedAdministrator, DelegatedService, EnabledService, Handshake, HandshakeParty,
    HandshakeResource, OrgPolicy, OrgRoot, OrgTag, Organization, OrganizationalUnit,
    PolicyAttachment, PolicyTypeStatus, ResponsibilityTransfer,
};

/// Serializable view of the entire Organizations state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrganizationsStateView {
    /// The organization, if it exists.
    pub organization: Option<OrganizationView>,
    /// Accounts keyed by account ID.
    #[serde(default)]
    pub accounts: HashMap<String, AccountView>,
    /// Organizational units keyed by OU ID.
    #[serde(default)]
    pub ous: HashMap<String, OrganizationalUnitView>,
    /// Delegated administrators.
    #[serde(default)]
    pub delegated_admins: Vec<DelegatedAdministratorView>,
    /// The root, if it exists.
    pub root: Option<OrgRootView>,
    /// Policies keyed by policy ID.
    #[serde(default)]
    pub policies: HashMap<String, OrgPolicyView>,
    /// Policy attachments.
    #[serde(default)]
    pub policy_attachments: Vec<PolicyAttachmentView>,
    /// Enabled AWS services.
    #[serde(default)]
    pub enabled_services: Vec<EnabledServiceView>,
    /// Tags keyed by resource ID.
    #[serde(default)]
    pub tags: HashMap<String, Vec<OrgTagView>>,
    /// Handshakes keyed by handshake ID.
    #[serde(default)]
    pub handshakes: HashMap<String, HandshakeView>,
    /// Resource policy content, if set.
    #[serde(default)]
    pub resource_policy: Option<String>,
    /// Responsibility transfers keyed by transfer ID.
    #[serde(default)]
    pub responsibility_transfers: HashMap<String, ResponsibilityTransferView>,
    /// Next account sequence number (used for deterministic account ID generation).
    #[serde(default)]
    pub next_account_num: u64,
    /// Next policy sequence number (used for policy ID generation).
    #[serde(default)]
    pub next_policy_num: u64,
    /// Next handshake sequence number (used for handshake ID generation).
    #[serde(default)]
    pub next_handshake_num: u64,
    /// Next responsibility transfer sequence number (used for transfer ID generation).
    #[serde(default)]
    pub next_transfer_num: u64,
}

/// Serializable view of an AWS Organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationView {
    pub id: String,
    pub arn: String,
    pub master_account_id: String,
    pub master_account_email: String,
}

/// Serializable view of an AWS account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub email: String,
    pub status: String,
    pub joined_method: String,
    pub joined_timestamp: String,
    pub create_account_status_id: String,
    pub parent_id: String,
}

/// Serializable view of an organizational unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationalUnitView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub parent_id: String,
}

/// Serializable view of a delegated administrator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegatedAdministratorView {
    pub account: AccountView,
    pub delegation_enabled_date: String,
    pub services: Vec<DelegatedServiceView>,
}

/// Serializable view of a delegated service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegatedServiceView {
    pub service_principal: String,
    pub delegation_enabled_date: String,
}

/// Serializable view of an organization root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgRootView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub policy_types: Vec<PolicyTypeStatusView>,
}

/// Serializable view of a policy type status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyTypeStatusView {
    pub policy_type: String,
    pub status: String,
}

/// Serializable view of an organization policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgPolicyView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub description: String,
    pub policy_type: String,
    pub content: String,
    pub aws_managed: bool,
}

/// Serializable view of a policy attachment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyAttachmentView {
    pub policy_id: String,
    pub target_id: String,
}

/// Serializable view of an enabled AWS service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnabledServiceView {
    pub service_principal: String,
    pub date_enabled: String,
}

/// Serializable view of an organization resource tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgTagView {
    pub key: String,
    pub value: String,
}

/// Serializable view of a handshake.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeView {
    pub id: String,
    pub arn: String,
    pub state: String,
    pub action: String,
    pub parties: Vec<HandshakePartyView>,
    pub expiration_timestamp: String,
    pub requested_timestamp: String,
    pub resources: Vec<HandshakeResourceView>,
}

/// Serializable view of a handshake party.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakePartyView {
    pub id: String,
    pub party_type: String,
}

/// Serializable view of a handshake resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeResourceView {
    pub resource_type: String,
    pub value: String,
}

/// Serializable view of a responsibility transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsibilityTransferView {
    pub id: String,
    pub arn: String,
    pub status: String,
    pub name: String,
    pub transfer_type: String,
    pub source_account_id: String,
    pub target_account_id: String,
    pub start_timestamp: String,
    pub active_handshake_id: Option<String>,
}

// --- From internal types to view types ---

impl From<&OrganizationsState> for OrganizationsStateView {
    fn from(state: &OrganizationsState) -> Self {
        OrganizationsStateView {
            organization: state.organization.as_ref().map(OrganizationView::from),
            accounts: state
                .accounts
                .iter()
                .map(|(k, v)| (k.clone(), AccountView::from(v)))
                .collect(),
            ous: state
                .ous
                .iter()
                .map(|(k, v)| (k.clone(), OrganizationalUnitView::from(v)))
                .collect(),
            delegated_admins: state
                .delegated_admins
                .iter()
                .map(DelegatedAdministratorView::from)
                .collect(),
            root: state.root.as_ref().map(OrgRootView::from),
            policies: state
                .policies
                .iter()
                .map(|(k, v)| (k.clone(), OrgPolicyView::from(v)))
                .collect(),
            policy_attachments: state
                .policy_attachments
                .iter()
                .map(PolicyAttachmentView::from)
                .collect(),
            enabled_services: state
                .enabled_services
                .iter()
                .map(EnabledServiceView::from)
                .collect(),
            tags: state
                .tags
                .iter()
                .map(|(k, v)| (k.clone(), v.iter().map(OrgTagView::from).collect()))
                .collect(),
            handshakes: state
                .handshakes
                .iter()
                .map(|(k, v)| (k.clone(), HandshakeView::from(v)))
                .collect(),
            resource_policy: state.resource_policy.clone(),
            responsibility_transfers: state
                .responsibility_transfers
                .iter()
                .map(|(k, v)| (k.clone(), ResponsibilityTransferView::from(v)))
                .collect(),
            next_account_num: state.next_account_num(),
            next_policy_num: state.next_policy_num(),
            next_handshake_num: state.next_handshake_num(),
            next_transfer_num: state.next_transfer_num(),
        }
    }
}

impl From<&Organization> for OrganizationView {
    fn from(org: &Organization) -> Self {
        OrganizationView {
            id: org.id.clone(),
            arn: org.arn.clone(),
            master_account_id: org.master_account_id.clone(),
            master_account_email: org.master_account_email.clone(),
        }
    }
}

impl From<&Account> for AccountView {
    fn from(account: &Account) -> Self {
        AccountView {
            id: account.id.clone(),
            arn: account.arn.clone(),
            name: account.name.clone(),
            email: account.email.clone(),
            status: account.status.clone(),
            joined_method: account.joined_method.clone(),
            joined_timestamp: account.joined_timestamp.to_rfc3339(),
            create_account_status_id: account.create_account_status_id.clone(),
            parent_id: account.parent_id.clone(),
        }
    }
}

impl From<&OrganizationalUnit> for OrganizationalUnitView {
    fn from(ou: &OrganizationalUnit) -> Self {
        OrganizationalUnitView {
            id: ou.id.clone(),
            arn: ou.arn.clone(),
            name: ou.name.clone(),
            parent_id: ou.parent_id.clone(),
        }
    }
}

impl From<&DelegatedAdministrator> for DelegatedAdministratorView {
    fn from(da: &DelegatedAdministrator) -> Self {
        DelegatedAdministratorView {
            account: AccountView::from(&da.account),
            delegation_enabled_date: da.delegation_enabled_date.to_rfc3339(),
            services: da.services.iter().map(DelegatedServiceView::from).collect(),
        }
    }
}

impl From<&DelegatedService> for DelegatedServiceView {
    fn from(ds: &DelegatedService) -> Self {
        DelegatedServiceView {
            service_principal: ds.service_principal.clone(),
            delegation_enabled_date: ds.delegation_enabled_date.to_rfc3339(),
        }
    }
}

impl From<&OrgRoot> for OrgRootView {
    fn from(root: &OrgRoot) -> Self {
        OrgRootView {
            id: root.id.clone(),
            arn: root.arn.clone(),
            name: root.name.clone(),
            policy_types: root
                .policy_types
                .iter()
                .map(PolicyTypeStatusView::from)
                .collect(),
        }
    }
}

impl From<&PolicyTypeStatus> for PolicyTypeStatusView {
    fn from(pts: &PolicyTypeStatus) -> Self {
        PolicyTypeStatusView {
            policy_type: pts.policy_type.clone(),
            status: pts.status.clone(),
        }
    }
}

impl From<&OrgPolicy> for OrgPolicyView {
    fn from(policy: &OrgPolicy) -> Self {
        OrgPolicyView {
            id: policy.id.clone(),
            arn: policy.arn.clone(),
            name: policy.name.clone(),
            description: policy.description.clone(),
            policy_type: policy.policy_type.clone(),
            content: policy.content.clone(),
            aws_managed: policy.aws_managed,
        }
    }
}

impl From<&PolicyAttachment> for PolicyAttachmentView {
    fn from(pa: &PolicyAttachment) -> Self {
        PolicyAttachmentView {
            policy_id: pa.policy_id.clone(),
            target_id: pa.target_id.clone(),
        }
    }
}

impl From<&EnabledService> for EnabledServiceView {
    fn from(es: &EnabledService) -> Self {
        EnabledServiceView {
            service_principal: es.service_principal.clone(),
            date_enabled: es.date_enabled.to_rfc3339(),
        }
    }
}

impl From<&OrgTag> for OrgTagView {
    fn from(tag: &OrgTag) -> Self {
        OrgTagView {
            key: tag.key.clone(),
            value: tag.value.clone(),
        }
    }
}

impl From<&Handshake> for HandshakeView {
    fn from(hs: &Handshake) -> Self {
        HandshakeView {
            id: hs.id.clone(),
            arn: hs.arn.clone(),
            state: hs.state.clone(),
            action: hs.action.clone(),
            parties: hs.parties.iter().map(HandshakePartyView::from).collect(),
            expiration_timestamp: hs.expiration_timestamp.to_rfc3339(),
            requested_timestamp: hs.requested_timestamp.to_rfc3339(),
            resources: hs
                .resources
                .iter()
                .map(HandshakeResourceView::from)
                .collect(),
        }
    }
}

impl From<&HandshakeParty> for HandshakePartyView {
    fn from(p: &HandshakeParty) -> Self {
        HandshakePartyView {
            id: p.id.clone(),
            party_type: p.party_type.clone(),
        }
    }
}

impl From<&HandshakeResource> for HandshakeResourceView {
    fn from(r: &HandshakeResource) -> Self {
        HandshakeResourceView {
            resource_type: r.resource_type.clone(),
            value: r.value.clone(),
        }
    }
}

impl From<&ResponsibilityTransfer> for ResponsibilityTransferView {
    fn from(t: &ResponsibilityTransfer) -> Self {
        ResponsibilityTransferView {
            id: t.id.clone(),
            arn: t.arn.clone(),
            status: t.status.clone(),
            name: t.name.clone(),
            transfer_type: t.transfer_type.clone(),
            source_account_id: t.source_account_id.clone(),
            target_account_id: t.target_account_id.clone(),
            start_timestamp: t.start_timestamp.to_rfc3339(),
            active_handshake_id: t.active_handshake_id.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<OrganizationsStateView> for OrganizationsState {
    fn from(view: OrganizationsStateView) -> Self {
        let mut state = OrganizationsState::default();
        state.organization = view.organization.map(Organization::from);
        state.accounts = view
            .accounts
            .into_iter()
            .map(|(k, v)| (k, Account::from(v)))
            .collect();
        state.ous = view
            .ous
            .into_iter()
            .map(|(k, v)| (k, OrganizationalUnit::from(v)))
            .collect();
        state.delegated_admins = view
            .delegated_admins
            .into_iter()
            .map(DelegatedAdministrator::from)
            .collect();
        state.root = view.root.map(OrgRoot::from);
        state.policies = view
            .policies
            .into_iter()
            .map(|(k, v)| (k, OrgPolicy::from(v)))
            .collect();
        state.policy_attachments = view
            .policy_attachments
            .into_iter()
            .map(PolicyAttachment::from)
            .collect();
        state.enabled_services = view
            .enabled_services
            .into_iter()
            .map(EnabledService::from)
            .collect();
        state.tags = view
            .tags
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().map(OrgTag::from).collect()))
            .collect();
        state.handshakes = view
            .handshakes
            .into_iter()
            .map(|(k, v)| (k, Handshake::from(v)))
            .collect();
        state.resource_policy = view.resource_policy;
        state.responsibility_transfers = view
            .responsibility_transfers
            .into_iter()
            .map(|(k, v)| (k, ResponsibilityTransfer::from(v)))
            .collect();
        if view.next_account_num > 0
            || view.next_policy_num > 0
            || view.next_handshake_num > 0
            || view.next_transfer_num > 0
        {
            state.restore_counters(
                view.next_account_num,
                view.next_policy_num,
                view.next_handshake_num,
                view.next_transfer_num,
            );
        }
        state
    }
}

impl From<OrganizationView> for Organization {
    fn from(view: OrganizationView) -> Self {
        Organization {
            id: view.id,
            arn: view.arn,
            master_account_id: view.master_account_id,
            master_account_email: view.master_account_email,
        }
    }
}

impl From<AccountView> for Account {
    fn from(view: AccountView) -> Self {
        let joined_timestamp = view
            .joined_timestamp
            .parse::<DateTime<Utc>>()
            .unwrap_or_else(|_| Utc::now());
        Account {
            id: view.id,
            arn: view.arn,
            name: view.name,
            email: view.email,
            status: view.status,
            joined_method: view.joined_method,
            joined_timestamp,
            create_account_status_id: view.create_account_status_id,
            parent_id: view.parent_id,
        }
    }
}

impl From<OrganizationalUnitView> for OrganizationalUnit {
    fn from(view: OrganizationalUnitView) -> Self {
        OrganizationalUnit {
            id: view.id,
            arn: view.arn,
            name: view.name,
            parent_id: view.parent_id,
        }
    }
}

impl From<DelegatedAdministratorView> for DelegatedAdministrator {
    fn from(view: DelegatedAdministratorView) -> Self {
        let delegation_enabled_date = view
            .delegation_enabled_date
            .parse::<DateTime<Utc>>()
            .unwrap_or_else(|_| Utc::now());
        DelegatedAdministrator {
            account: Account::from(view.account),
            delegation_enabled_date,
            services: view
                .services
                .into_iter()
                .map(DelegatedService::from)
                .collect(),
        }
    }
}

impl From<DelegatedServiceView> for DelegatedService {
    fn from(view: DelegatedServiceView) -> Self {
        let delegation_enabled_date = view
            .delegation_enabled_date
            .parse::<DateTime<Utc>>()
            .unwrap_or_else(|_| Utc::now());
        DelegatedService {
            service_principal: view.service_principal,
            delegation_enabled_date,
        }
    }
}

impl From<OrgRootView> for OrgRoot {
    fn from(view: OrgRootView) -> Self {
        OrgRoot {
            id: view.id,
            arn: view.arn,
            name: view.name,
            policy_types: view
                .policy_types
                .into_iter()
                .map(PolicyTypeStatus::from)
                .collect(),
        }
    }
}

impl From<PolicyTypeStatusView> for PolicyTypeStatus {
    fn from(view: PolicyTypeStatusView) -> Self {
        PolicyTypeStatus {
            policy_type: view.policy_type,
            status: view.status,
        }
    }
}

impl From<OrgPolicyView> for OrgPolicy {
    fn from(view: OrgPolicyView) -> Self {
        OrgPolicy {
            id: view.id,
            arn: view.arn,
            name: view.name,
            description: view.description,
            policy_type: view.policy_type,
            content: view.content,
            aws_managed: view.aws_managed,
        }
    }
}

impl From<PolicyAttachmentView> for PolicyAttachment {
    fn from(view: PolicyAttachmentView) -> Self {
        PolicyAttachment {
            policy_id: view.policy_id,
            target_id: view.target_id,
        }
    }
}

impl From<EnabledServiceView> for EnabledService {
    fn from(view: EnabledServiceView) -> Self {
        let date_enabled = view
            .date_enabled
            .parse::<DateTime<Utc>>()
            .unwrap_or_else(|_| Utc::now());
        EnabledService {
            service_principal: view.service_principal,
            date_enabled,
        }
    }
}

impl From<OrgTagView> for OrgTag {
    fn from(view: OrgTagView) -> Self {
        OrgTag {
            key: view.key,
            value: view.value,
        }
    }
}

impl From<HandshakeView> for Handshake {
    fn from(view: HandshakeView) -> Self {
        let expiration_timestamp = view
            .expiration_timestamp
            .parse::<DateTime<Utc>>()
            .unwrap_or_else(|_| Utc::now());
        let requested_timestamp = view
            .requested_timestamp
            .parse::<DateTime<Utc>>()
            .unwrap_or_else(|_| Utc::now());
        Handshake {
            id: view.id,
            arn: view.arn,
            state: view.state,
            action: view.action,
            parties: view.parties.into_iter().map(HandshakeParty::from).collect(),
            expiration_timestamp,
            requested_timestamp,
            resources: view
                .resources
                .into_iter()
                .map(HandshakeResource::from)
                .collect(),
        }
    }
}

impl From<HandshakePartyView> for HandshakeParty {
    fn from(view: HandshakePartyView) -> Self {
        HandshakeParty {
            id: view.id,
            party_type: view.party_type,
        }
    }
}

impl From<HandshakeResourceView> for HandshakeResource {
    fn from(view: HandshakeResourceView) -> Self {
        HandshakeResource {
            resource_type: view.resource_type,
            value: view.value,
        }
    }
}

impl From<ResponsibilityTransferView> for ResponsibilityTransfer {
    fn from(view: ResponsibilityTransferView) -> Self {
        let start_timestamp = view
            .start_timestamp
            .parse::<DateTime<Utc>>()
            .unwrap_or_else(|_| Utc::now());
        ResponsibilityTransfer {
            id: view.id,
            arn: view.arn,
            status: view.status,
            name: view.name,
            transfer_type: view.transfer_type,
            source_account_id: view.source_account_id,
            target_account_id: view.target_account_id,
            start_timestamp,
            active_handshake_id: view.active_handshake_id,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for OrganizationsService {
    type StateView = OrganizationsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        OrganizationsStateView::from(&*guard)
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
            *guard = OrganizationsState::from(view);
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
            if let Some(org) = view.organization {
                guard.organization = Some(Organization::from(org));
            }
            for (id, account_view) in view.accounts {
                guard.accounts.insert(id, Account::from(account_view));
            }
            for (id, ou_view) in view.ous {
                guard.ous.insert(id, OrganizationalUnit::from(ou_view));
            }
            for da in view.delegated_admins {
                guard
                    .delegated_admins
                    .push(DelegatedAdministrator::from(da));
            }
            if let Some(root) = view.root {
                guard.root = Some(OrgRoot::from(root));
            }
            for (id, policy_view) in view.policies {
                guard.policies.insert(id, OrgPolicy::from(policy_view));
            }
            for pa in view.policy_attachments {
                guard.policy_attachments.push(PolicyAttachment::from(pa));
            }
            for es in view.enabled_services {
                guard.enabled_services.push(EnabledService::from(es));
            }
            for (resource_id, tags) in view.tags {
                guard
                    .tags
                    .insert(resource_id, tags.into_iter().map(OrgTag::from).collect());
            }
            for (id, hs_view) in view.handshakes {
                guard.handshakes.insert(id, Handshake::from(hs_view));
            }
            if let Some(rp) = view.resource_policy {
                guard.resource_policy = Some(rp);
            }
            for (id, rt_view) in view.responsibility_transfers {
                guard
                    .responsibility_transfers
                    .insert(id, ResponsibilityTransfer::from(rt_view));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
