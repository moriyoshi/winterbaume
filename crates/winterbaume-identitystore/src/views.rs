//! Serde-compatible view types for IdentityStore state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::IdentityStoreService;
use crate::state::IdentityStoreState;
use crate::types::{
    Address, Email, ExternalId, Group, GroupMembership, Name, PhoneNumber, Photo, Role, User,
};

/// Serializable view of the entire IdentityStore state.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityStoreStateView {
    /// Users keyed by "identity_store_id/user_id".
    #[serde(default)]
    pub users: HashMap<String, UserView>,
    /// Groups keyed by "identity_store_id/group_id".
    #[serde(default)]
    pub groups: HashMap<String, GroupView>,
    /// Group memberships keyed by "identity_store_id/membership_id".
    #[serde(default)]
    pub memberships: HashMap<String, GroupMembershipView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserView {
    pub identity_store_id: String,
    pub user_id: String,
    pub user_name: Option<String>,
    pub name: Option<Name>,
    pub display_name: Option<String>,
    pub nick_name: Option<String>,
    pub profile_url: Option<String>,
    pub emails: Option<Vec<Email>>,
    pub addresses: Option<Vec<Address>>,
    pub phone_numbers: Option<Vec<PhoneNumber>>,
    pub user_type: Option<String>,
    pub title: Option<String>,
    pub preferred_language: Option<String>,
    pub locale: Option<String>,
    pub timezone: Option<String>,
    #[serde(default)]
    pub external_ids: Vec<ExternalId>,
    pub photos: Option<Vec<Photo>>,
    pub website: Option<String>,
    pub birthdate: Option<String>,
    pub roles: Option<Vec<Role>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupView {
    pub identity_store_id: String,
    pub group_id: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub external_ids: Vec<ExternalId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMembershipView {
    pub identity_store_id: String,
    pub membership_id: String,
    pub group_id: String,
    pub member_user_id: String,
}

// ---------------------------------------------------------------------------
// From conversions
// ---------------------------------------------------------------------------

impl From<&User> for UserView {
    fn from(u: &User) -> Self {
        Self {
            identity_store_id: u.identity_store_id.clone(),
            user_id: u.user_id.clone(),
            user_name: u.user_name.clone(),
            name: u.name.clone(),
            display_name: u.display_name.clone(),
            nick_name: u.nick_name.clone(),
            profile_url: u.profile_url.clone(),
            emails: u.emails.clone(),
            addresses: u.addresses.clone(),
            phone_numbers: u.phone_numbers.clone(),
            user_type: u.user_type.clone(),
            title: u.title.clone(),
            preferred_language: u.preferred_language.clone(),
            locale: u.locale.clone(),
            timezone: u.timezone.clone(),
            external_ids: u.external_ids.clone(),
            photos: u.photos.clone(),
            website: u.website.clone(),
            birthdate: u.birthdate.clone(),
            roles: u.roles.clone(),
        }
    }
}

impl From<&Group> for GroupView {
    fn from(g: &Group) -> Self {
        Self {
            identity_store_id: g.identity_store_id.clone(),
            group_id: g.group_id.clone(),
            display_name: g.display_name.clone(),
            description: g.description.clone(),
            external_ids: g.external_ids.clone(),
        }
    }
}

impl From<&GroupMembership> for GroupMembershipView {
    fn from(m: &GroupMembership) -> Self {
        Self {
            identity_store_id: m.identity_store_id.clone(),
            membership_id: m.membership_id.clone(),
            group_id: m.group_id.clone(),
            member_user_id: m.member_user_id.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for IdentityStoreService {
    type StateView = IdentityStoreStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;

        let users = guard
            .users
            .iter()
            .map(|((store_id, user_id), u)| (format!("{store_id}/{user_id}"), UserView::from(u)))
            .collect();

        let groups = guard
            .groups
            .iter()
            .map(|((store_id, group_id), g)| (format!("{store_id}/{group_id}"), GroupView::from(g)))
            .collect();

        let memberships = guard
            .memberships
            .iter()
            .map(|((store_id, membership_id), m)| {
                (
                    format!("{store_id}/{membership_id}"),
                    GroupMembershipView::from(m),
                )
            })
            .collect();

        IdentityStoreStateView {
            users,
            groups,
            memberships,
        }
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let new_state = state_from_view(view);
        {
            let state = self.state.get(account_id, region);
            *state.write().await = new_state;
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
        let incoming = state_from_view(view);
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            guard.users.extend(incoming.users);
            guard.groups.extend(incoming.groups);
            guard.memberships.extend(incoming.memberships);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

fn state_from_view(view: IdentityStoreStateView) -> IdentityStoreState {
    let mut state = IdentityStoreState::default();

    for (_composite_key, uv) in view.users {
        let key = (uv.identity_store_id.clone(), uv.user_id.clone());
        state.users.insert(
            key,
            User {
                identity_store_id: uv.identity_store_id,
                user_id: uv.user_id,
                user_name: uv.user_name,
                name: uv.name,
                display_name: uv.display_name,
                nick_name: uv.nick_name,
                profile_url: uv.profile_url,
                emails: uv.emails,
                addresses: uv.addresses,
                phone_numbers: uv.phone_numbers,
                user_type: uv.user_type,
                title: uv.title,
                preferred_language: uv.preferred_language,
                locale: uv.locale,
                timezone: uv.timezone,
                external_ids: uv.external_ids,
                photos: uv.photos,
                website: uv.website,
                birthdate: uv.birthdate,
                roles: uv.roles,
            },
        );
    }

    for (_composite_key, gv) in view.groups {
        let key = (gv.identity_store_id.clone(), gv.group_id.clone());
        state.groups.insert(
            key,
            Group {
                identity_store_id: gv.identity_store_id,
                group_id: gv.group_id,
                display_name: gv.display_name,
                description: gv.description,
                external_ids: gv.external_ids,
            },
        );
    }

    for (_composite_key, mv) in view.memberships {
        let key = (mv.identity_store_id.clone(), mv.membership_id.clone());
        state.memberships.insert(
            key,
            GroupMembership {
                identity_store_id: mv.identity_store_id,
                membership_id: mv.membership_id,
                group_id: mv.group_id,
                member_user_id: mv.member_user_id,
            },
        );
    }

    state
}
