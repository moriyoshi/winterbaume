use std::collections::HashMap;

use serde_json::Value;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct IdentityStoreState {
    /// Users keyed by (IdentityStoreId, UserId).
    pub users: HashMap<(String, String), User>,
    /// Groups keyed by (IdentityStoreId, GroupId).
    pub groups: HashMap<(String, String), Group>,
    /// Group memberships keyed by (IdentityStoreId, MembershipId).
    pub memberships: HashMap<(String, String), GroupMembership>,
}

/// Domain-specific error enum for IdentityStore operations.
#[derive(Debug, Error)]
pub enum IdentityStoreError {
    #[error("User {user_id} not found in identity store {identity_store_id}")]
    UserNotFound {
        identity_store_id: String,
        user_id: String,
    },
    #[error("Group {group_id} not found in identity store {identity_store_id}")]
    GroupNotFound {
        identity_store_id: String,
        group_id: String,
    },
    #[error("Membership {membership_id} not found in identity store {identity_store_id}")]
    MembershipNotFound {
        identity_store_id: String,
        membership_id: String,
    },
    #[error(
        "No group found with display name '{display_name}' in identity store {identity_store_id}"
    )]
    GroupNotFoundByDisplayName {
        identity_store_id: String,
        display_name: String,
    },
    #[error("No user found with user name '{user_name}' in identity store {identity_store_id}")]
    UserNotFoundByUserName {
        identity_store_id: String,
        user_name: String,
    },
    #[error("Member {member_user_id} is already a member of group {group_id}")]
    MembershipAlreadyExists {
        group_id: String,
        member_user_id: String,
    },
}

impl IdentityStoreState {
    pub fn create_user(
        &mut self,
        identity_store_id: &str,
        body: &Value,
    ) -> Result<User, IdentityStoreError> {
        let user_id = uuid::Uuid::new_v4().to_string();

        let user = User {
            identity_store_id: identity_store_id.to_string(),
            user_id: user_id.clone(),
            user_name: body
                .get("UserName")
                .and_then(|v| v.as_str())
                .map(String::from),
            name: body
                .get("Name")
                .and_then(|v| serde_json::from_value(v.clone()).ok()),
            display_name: body
                .get("DisplayName")
                .and_then(|v| v.as_str())
                .map(String::from),
            nick_name: body
                .get("NickName")
                .and_then(|v| v.as_str())
                .map(String::from),
            profile_url: body
                .get("ProfileUrl")
                .and_then(|v| v.as_str())
                .map(String::from),
            emails: body
                .get("Emails")
                .and_then(|v| serde_json::from_value(v.clone()).ok()),
            addresses: body
                .get("Addresses")
                .and_then(|v| serde_json::from_value(v.clone()).ok()),
            phone_numbers: body
                .get("PhoneNumbers")
                .and_then(|v| serde_json::from_value(v.clone()).ok()),
            user_type: body
                .get("UserType")
                .and_then(|v| v.as_str())
                .map(String::from),
            title: body.get("Title").and_then(|v| v.as_str()).map(String::from),
            preferred_language: body
                .get("PreferredLanguage")
                .and_then(|v| v.as_str())
                .map(String::from),
            locale: body
                .get("Locale")
                .and_then(|v| v.as_str())
                .map(String::from),
            timezone: body
                .get("Timezone")
                .and_then(|v| v.as_str())
                .map(String::from),
            external_ids: Vec::new(),
            photos: body
                .get("Photos")
                .and_then(|v| serde_json::from_value(v.clone()).ok()),
            website: body
                .get("Website")
                .and_then(|v| v.as_str())
                .map(String::from),
            birthdate: body
                .get("Birthdate")
                .and_then(|v| v.as_str())
                .map(String::from),
            roles: body
                .get("Roles")
                .and_then(|v| serde_json::from_value(v.clone()).ok()),
        };

        let key = (identity_store_id.to_string(), user_id);
        self.users.insert(key.clone(), user);
        Ok(self.users.get(&key).unwrap().clone())
    }

    pub fn describe_user(
        &self,
        identity_store_id: &str,
        user_id: &str,
    ) -> Result<&User, IdentityStoreError> {
        let key = (identity_store_id.to_string(), user_id.to_string());
        self.users
            .get(&key)
            .ok_or_else(|| IdentityStoreError::UserNotFound {
                identity_store_id: identity_store_id.to_string(),
                user_id: user_id.to_string(),
            })
    }

    pub fn delete_user(
        &mut self,
        identity_store_id: &str,
        user_id: &str,
    ) -> Result<(), IdentityStoreError> {
        let key = (identity_store_id.to_string(), user_id.to_string());
        if self.users.remove(&key).is_none() {
            return Err(IdentityStoreError::UserNotFound {
                identity_store_id: identity_store_id.to_string(),
                user_id: user_id.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_users(
        &self,
        identity_store_id: &str,
        max_results: Option<usize>,
        next_token: Option<&str>,
    ) -> (Vec<&User>, Option<String>) {
        let mut users: Vec<&User> = self
            .users
            .values()
            .filter(|u| u.identity_store_id == identity_store_id)
            .collect();

        // Sort by user_id for deterministic pagination.
        users.sort_by(|a, b| a.user_id.cmp(&b.user_id));

        let max = max_results.unwrap_or(100).min(100);

        // Find start position based on next_token (which is the last user_id seen).
        let start = if let Some(token) = next_token {
            users
                .iter()
                .position(|u| u.user_id.as_str() > token)
                .unwrap_or(users.len())
        } else {
            0
        };

        let page: Vec<&User> = users.into_iter().skip(start).take(max).collect();

        let new_next_token = if page.len() == max && start + max < self.users.len() {
            page.last().map(|u| u.user_id.clone())
        } else {
            None
        };

        (page, new_next_token)
    }

    // --- Group operations ---

    pub fn create_group(
        &mut self,
        identity_store_id: &str,
        body: &Value,
    ) -> Result<Group, IdentityStoreError> {
        let group_id = uuid::Uuid::new_v4().to_string();

        let group = Group {
            identity_store_id: identity_store_id.to_string(),
            group_id: group_id.clone(),
            display_name: body
                .get("DisplayName")
                .and_then(|v| v.as_str())
                .map(String::from),
            description: body
                .get("Description")
                .and_then(|v| v.as_str())
                .map(String::from),
            external_ids: Vec::new(),
        };

        let key = (identity_store_id.to_string(), group_id);
        self.groups.insert(key.clone(), group);
        Ok(self.groups.get(&key).unwrap().clone())
    }

    pub fn describe_group(
        &self,
        identity_store_id: &str,
        group_id: &str,
    ) -> Result<&Group, IdentityStoreError> {
        let key = (identity_store_id.to_string(), group_id.to_string());
        self.groups
            .get(&key)
            .ok_or_else(|| IdentityStoreError::GroupNotFound {
                identity_store_id: identity_store_id.to_string(),
                group_id: group_id.to_string(),
            })
    }

    pub fn delete_group(
        &mut self,
        identity_store_id: &str,
        group_id: &str,
    ) -> Result<(), IdentityStoreError> {
        let key = (identity_store_id.to_string(), group_id.to_string());
        if self.groups.remove(&key).is_none() {
            return Err(IdentityStoreError::GroupNotFound {
                identity_store_id: identity_store_id.to_string(),
                group_id: group_id.to_string(),
            });
        }
        // Also remove memberships for this group.
        self.memberships
            .retain(|_, m| !(m.identity_store_id == identity_store_id && m.group_id == group_id));
        Ok(())
    }

    pub fn list_groups(
        &self,
        identity_store_id: &str,
        max_results: Option<usize>,
        next_token: Option<&str>,
    ) -> (Vec<&Group>, Option<String>) {
        let mut groups: Vec<&Group> = self
            .groups
            .values()
            .filter(|g| g.identity_store_id == identity_store_id)
            .collect();

        groups.sort_by(|a, b| a.group_id.cmp(&b.group_id));

        let max = max_results.unwrap_or(100).min(100);

        let start = if let Some(token) = next_token {
            groups
                .iter()
                .position(|g| g.group_id.as_str() > token)
                .unwrap_or(groups.len())
        } else {
            0
        };

        let page: Vec<&Group> = groups.into_iter().skip(start).take(max).collect();

        let new_next_token = if page.len() == max && start + max < self.groups.len() {
            page.last().map(|g| g.group_id.clone())
        } else {
            None
        };

        (page, new_next_token)
    }

    pub fn get_group_id(
        &self,
        identity_store_id: &str,
        display_name: &str,
    ) -> Result<&Group, IdentityStoreError> {
        self.groups
            .values()
            .find(|g| {
                g.identity_store_id == identity_store_id
                    && g.display_name.as_deref() == Some(display_name)
            })
            .ok_or_else(|| IdentityStoreError::GroupNotFoundByDisplayName {
                identity_store_id: identity_store_id.to_string(),
                display_name: display_name.to_string(),
            })
    }

    pub fn get_user_id(
        &self,
        identity_store_id: &str,
        user_name: &str,
    ) -> Result<&User, IdentityStoreError> {
        self.users
            .values()
            .find(|u| {
                u.identity_store_id == identity_store_id
                    && u.user_name.as_deref() == Some(user_name)
            })
            .ok_or_else(|| IdentityStoreError::UserNotFoundByUserName {
                identity_store_id: identity_store_id.to_string(),
                user_name: user_name.to_string(),
            })
    }

    // --- Group membership operations ---

    pub fn create_group_membership(
        &mut self,
        identity_store_id: &str,
        group_id: &str,
        member_user_id: &str,
    ) -> Result<GroupMembership, IdentityStoreError> {
        // Verify group exists.
        let gkey = (identity_store_id.to_string(), group_id.to_string());
        if !self.groups.contains_key(&gkey) {
            return Err(IdentityStoreError::GroupNotFound {
                identity_store_id: identity_store_id.to_string(),
                group_id: group_id.to_string(),
            });
        }

        // Check for duplicate membership.
        let already_exists = self.memberships.values().any(|m| {
            m.identity_store_id == identity_store_id
                && m.group_id == group_id
                && m.member_user_id == member_user_id
        });
        if already_exists {
            return Err(IdentityStoreError::MembershipAlreadyExists {
                group_id: group_id.to_string(),
                member_user_id: member_user_id.to_string(),
            });
        }

        let membership_id = uuid::Uuid::new_v4().to_string();
        let membership = GroupMembership {
            identity_store_id: identity_store_id.to_string(),
            membership_id: membership_id.clone(),
            group_id: group_id.to_string(),
            member_user_id: member_user_id.to_string(),
        };

        let key = (identity_store_id.to_string(), membership_id);
        self.memberships.insert(key.clone(), membership);
        Ok(self.memberships.get(&key).unwrap().clone())
    }

    pub fn delete_group_membership(
        &mut self,
        identity_store_id: &str,
        membership_id: &str,
    ) -> Result<(), IdentityStoreError> {
        let key = (identity_store_id.to_string(), membership_id.to_string());
        if self.memberships.remove(&key).is_none() {
            return Err(IdentityStoreError::MembershipNotFound {
                identity_store_id: identity_store_id.to_string(),
                membership_id: membership_id.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_group_memberships(
        &self,
        identity_store_id: &str,
        group_id: &str,
        max_results: Option<usize>,
        next_token: Option<&str>,
    ) -> (Vec<&GroupMembership>, Option<String>) {
        let mut memberships: Vec<&GroupMembership> = self
            .memberships
            .values()
            .filter(|m| m.identity_store_id == identity_store_id && m.group_id == group_id)
            .collect();

        memberships.sort_by(|a, b| a.membership_id.cmp(&b.membership_id));

        let max = max_results.unwrap_or(100).min(100);

        let start = if let Some(token) = next_token {
            memberships
                .iter()
                .position(|m| m.membership_id.as_str() > token)
                .unwrap_or(memberships.len())
        } else {
            0
        };

        let page: Vec<&GroupMembership> = memberships.into_iter().skip(start).take(max).collect();

        let new_next_token = if page.len() == max {
            page.last().map(|m| m.membership_id.clone())
        } else {
            None
        };

        (page, new_next_token)
    }

    pub fn describe_group_membership(
        &self,
        identity_store_id: &str,
        membership_id: &str,
    ) -> Result<&GroupMembership, IdentityStoreError> {
        let key = (identity_store_id.to_string(), membership_id.to_string());
        self.memberships
            .get(&key)
            .ok_or_else(|| IdentityStoreError::MembershipNotFound {
                identity_store_id: identity_store_id.to_string(),
                membership_id: membership_id.to_string(),
            })
    }

    pub fn update_group(
        &mut self,
        identity_store_id: &str,
        group_id: &str,
        operations: &Value,
    ) -> Result<(), IdentityStoreError> {
        let key = (identity_store_id.to_string(), group_id.to_string());
        let group = self
            .groups
            .get_mut(&key)
            .ok_or_else(|| IdentityStoreError::GroupNotFound {
                identity_store_id: identity_store_id.to_string(),
                group_id: group_id.to_string(),
            })?;

        if let Some(ops) = operations.as_array() {
            for op in ops {
                let attr_path = op.get("AttributePath").and_then(|v| v.as_str());
                let attr_value = op.get("AttributeValue");
                match attr_path {
                    Some("displayName") => {
                        group.display_name = attr_value.and_then(|v| v.as_str()).map(String::from);
                    }
                    Some("description") => {
                        group.description = attr_value.and_then(|v| v.as_str()).map(String::from);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn update_user(
        &mut self,
        identity_store_id: &str,
        user_id: &str,
        operations: &Value,
    ) -> Result<(), IdentityStoreError> {
        let key = (identity_store_id.to_string(), user_id.to_string());
        let user = self
            .users
            .get_mut(&key)
            .ok_or_else(|| IdentityStoreError::UserNotFound {
                identity_store_id: identity_store_id.to_string(),
                user_id: user_id.to_string(),
            })?;

        if let Some(ops) = operations.as_array() {
            for op in ops {
                let attr_path = op.get("AttributePath").and_then(|v| v.as_str());
                let attr_value = op.get("AttributeValue");
                match attr_path {
                    Some("displayName") => {
                        user.display_name = attr_value.and_then(|v| v.as_str()).map(String::from);
                    }
                    Some("userName") => {
                        user.user_name = attr_value.and_then(|v| v.as_str()).map(String::from);
                    }
                    Some("name") => {
                        user.name = attr_value.and_then(|v| serde_json::from_value(v.clone()).ok());
                    }
                    Some("emails") => {
                        user.emails =
                            attr_value.and_then(|v| serde_json::from_value(v.clone()).ok());
                    }
                    Some("phoneNumbers") => {
                        user.phone_numbers =
                            attr_value.and_then(|v| serde_json::from_value(v.clone()).ok());
                    }
                    Some("addresses") => {
                        user.addresses =
                            attr_value.and_then(|v| serde_json::from_value(v.clone()).ok());
                    }
                    Some("title") => {
                        user.title = attr_value.and_then(|v| v.as_str()).map(String::from);
                    }
                    Some("userType") => {
                        user.user_type = attr_value.and_then(|v| v.as_str()).map(String::from);
                    }
                    Some("preferredLanguage") => {
                        user.preferred_language =
                            attr_value.and_then(|v| v.as_str()).map(String::from);
                    }
                    Some("locale") => {
                        user.locale = attr_value.and_then(|v| v.as_str()).map(String::from);
                    }
                    Some("timezone") => {
                        user.timezone = attr_value.and_then(|v| v.as_str()).map(String::from);
                    }
                    Some("nickName") => {
                        user.nick_name = attr_value.and_then(|v| v.as_str()).map(String::from);
                    }
                    Some("profileUrl") => {
                        user.profile_url = attr_value.and_then(|v| v.as_str()).map(String::from);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn list_group_memberships_for_member(
        &self,
        identity_store_id: &str,
        member_user_id: &str,
        max_results: Option<usize>,
        next_token: Option<&str>,
    ) -> (Vec<&GroupMembership>, Option<String>) {
        let mut memberships: Vec<&GroupMembership> = self
            .memberships
            .values()
            .filter(|m| {
                m.identity_store_id == identity_store_id && m.member_user_id == member_user_id
            })
            .collect();

        memberships.sort_by(|a, b| a.membership_id.cmp(&b.membership_id));

        let max = max_results.unwrap_or(100).min(100);

        let start = if let Some(token) = next_token {
            memberships
                .iter()
                .position(|m| m.membership_id.as_str() > token)
                .unwrap_or(memberships.len())
        } else {
            0
        };

        let page: Vec<&GroupMembership> = memberships.into_iter().skip(start).take(max).collect();

        let new_next_token = if page.len() == max {
            page.last().map(|m| m.membership_id.clone())
        } else {
            None
        };

        (page, new_next_token)
    }
}
