use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;
use uuid::Uuid;

use crate::types::{
    AttributeMapping, Crl, MappingRule, NotificationSettingDetail, Profile, Source, Tag,
    TrustAnchor,
};

#[derive(Debug, Default)]
pub struct RolesAnywhereState {
    pub profiles: HashMap<String, Profile>,
    pub trust_anchors: HashMap<String, TrustAnchor>,
    pub crls: HashMap<String, Crl>,
}

/// Domain-specific error enum. Contains no HTTP status codes or AWS error type strings —
/// those are mapped in the handler's error-shaping function.
#[derive(Debug, Error)]
pub enum RolesAnywhereError {
    #[error("Resource with id {id} not found.")]
    NotFound { id: String },
    #[error("{message}")]
    Validation { message: String },
    #[error("Too many tags. Maximum number of tags is 200.")]
    TooManyTags,
}

impl RolesAnywhereState {
    // ── Profile CRUD ──────────────────────────────────────

    #[allow(clippy::too_many_arguments)]
    pub fn create_profile(
        &mut self,
        name: &str,
        role_arns: Vec<String>,
        managed_policy_arns: Vec<String>,
        session_policy: Option<String>,
        duration_seconds: Option<i32>,
        require_instance_properties: Option<bool>,
        enabled: Option<bool>,
        accept_role_session_name: Option<bool>,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<&Profile, RolesAnywhereError> {
        let profile_id = Uuid::new_v4().to_string();
        let profile_arn =
            format!("arn:aws:rolesanywhere:{region}:{account_id}:profile/{profile_id}");
        let now = Utc::now();
        let tag_map: HashMap<String, String> = tags.into_iter().map(|t| (t.key, t.value)).collect();

        let profile = Profile {
            profile_id: profile_id.clone(),
            profile_arn,
            name: name.to_string(),
            enabled: enabled.unwrap_or(true),
            role_arns,
            managed_policy_arns,
            session_policy,
            duration_seconds,
            require_instance_properties,
            accept_role_session_name,
            attribute_mappings: Vec::new(),
            created_by: None,
            created_at: now,
            updated_at: now,
            tags: tag_map,
        };
        self.profiles.insert(profile_id.clone(), profile);
        Ok(self.profiles.get(&profile_id).unwrap())
    }

    pub fn get_profile(&self, profile_id: &str) -> Result<&Profile, RolesAnywhereError> {
        self.profiles
            .get(profile_id)
            .ok_or_else(|| RolesAnywhereError::NotFound {
                id: profile_id.to_string(),
            })
    }

    pub fn delete_profile(&mut self, profile_id: &str) -> Result<Profile, RolesAnywhereError> {
        self.profiles
            .remove(profile_id)
            .ok_or_else(|| RolesAnywhereError::NotFound {
                id: profile_id.to_string(),
            })
    }

    pub fn list_profiles(&self) -> Vec<&Profile> {
        self.profiles.values().collect()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update_profile(
        &mut self,
        profile_id: &str,
        name: Option<&str>,
        session_policy: Option<&str>,
        role_arns: Option<Vec<String>>,
        managed_policy_arns: Option<Vec<String>>,
        duration_seconds: Option<i32>,
        accept_role_session_name: Option<bool>,
    ) -> Result<&Profile, RolesAnywhereError> {
        let profile =
            self.profiles
                .get_mut(profile_id)
                .ok_or_else(|| RolesAnywhereError::NotFound {
                    id: profile_id.to_string(),
                })?;

        if let Some(n) = name {
            profile.name = n.to_string();
        }
        if let Some(sp) = session_policy {
            profile.session_policy = Some(sp.to_string());
        }
        if let Some(ra) = role_arns {
            profile.role_arns = ra;
        }
        if let Some(mpa) = managed_policy_arns {
            profile.managed_policy_arns = mpa;
        }
        if let Some(ds) = duration_seconds {
            profile.duration_seconds = Some(ds);
        }
        if let Some(arsn) = accept_role_session_name {
            profile.accept_role_session_name = Some(arsn);
        }
        profile.updated_at = Utc::now();
        Ok(self.profiles.get(profile_id).unwrap())
    }

    pub fn enable_profile(&mut self, profile_id: &str) -> Result<&Profile, RolesAnywhereError> {
        let profile =
            self.profiles
                .get_mut(profile_id)
                .ok_or_else(|| RolesAnywhereError::NotFound {
                    id: profile_id.to_string(),
                })?;
        profile.enabled = true;
        profile.updated_at = Utc::now();
        Ok(self.profiles.get(profile_id).unwrap())
    }

    pub fn disable_profile(&mut self, profile_id: &str) -> Result<&Profile, RolesAnywhereError> {
        let profile =
            self.profiles
                .get_mut(profile_id)
                .ok_or_else(|| RolesAnywhereError::NotFound {
                    id: profile_id.to_string(),
                })?;
        profile.enabled = false;
        profile.updated_at = Utc::now();
        Ok(self.profiles.get(profile_id).unwrap())
    }

    pub fn put_attribute_mapping(
        &mut self,
        profile_id: &str,
        certificate_field: &str,
        mapping_rules: Vec<MappingRule>,
    ) -> Result<&Profile, RolesAnywhereError> {
        let profile =
            self.profiles
                .get_mut(profile_id)
                .ok_or_else(|| RolesAnywhereError::NotFound {
                    id: profile_id.to_string(),
                })?;
        // Replace existing mapping for the same certificate field, or add new
        if let Some(existing) = profile
            .attribute_mappings
            .iter_mut()
            .find(|m| m.certificate_field == certificate_field)
        {
            existing.mapping_rules = mapping_rules;
        } else {
            profile.attribute_mappings.push(AttributeMapping {
                certificate_field: certificate_field.to_string(),
                mapping_rules,
            });
        }
        profile.updated_at = Utc::now();
        Ok(self.profiles.get(profile_id).unwrap())
    }

    pub fn delete_attribute_mapping(
        &mut self,
        profile_id: &str,
        certificate_field: &str,
        specifiers: Option<Vec<String>>,
    ) -> Result<&Profile, RolesAnywhereError> {
        let profile =
            self.profiles
                .get_mut(profile_id)
                .ok_or_else(|| RolesAnywhereError::NotFound {
                    id: profile_id.to_string(),
                })?;

        if let Some(specs) = specifiers {
            // Remove specific specifiers from the mapping
            if let Some(mapping) = profile
                .attribute_mappings
                .iter_mut()
                .find(|m| m.certificate_field == certificate_field)
            {
                mapping
                    .mapping_rules
                    .retain(|r| !specs.contains(&r.specifier));
                // If no rules left, remove the entire mapping
                if mapping.mapping_rules.is_empty() {
                    profile
                        .attribute_mappings
                        .retain(|m| m.certificate_field != certificate_field);
                }
            }
        } else {
            // Remove entire mapping for the certificate field
            profile
                .attribute_mappings
                .retain(|m| m.certificate_field != certificate_field);
        }
        profile.updated_at = Utc::now();
        Ok(self.profiles.get(profile_id).unwrap())
    }

    // ── Trust Anchor CRUD ──────────────────────────────────

    pub fn create_trust_anchor(
        &mut self,
        name: &str,
        source: Source,
        enabled: Option<bool>,
        tags: Vec<Tag>,
        notification_settings: Vec<NotificationSettingDetail>,
        account_id: &str,
        region: &str,
    ) -> Result<&TrustAnchor, RolesAnywhereError> {
        let trust_anchor_id = Uuid::new_v4().to_string();
        let trust_anchor_arn =
            format!("arn:aws:rolesanywhere:{region}:{account_id}:trust-anchor/{trust_anchor_id}");
        let now = Utc::now();
        let tag_map: HashMap<String, String> = tags.into_iter().map(|t| (t.key, t.value)).collect();

        let trust_anchor = TrustAnchor {
            trust_anchor_id: trust_anchor_id.clone(),
            trust_anchor_arn,
            name: name.to_string(),
            source,
            enabled: enabled.unwrap_or(true),
            notification_settings,
            created_at: now,
            updated_at: now,
            tags: tag_map,
        };
        self.trust_anchors
            .insert(trust_anchor_id.clone(), trust_anchor);
        Ok(self.trust_anchors.get(&trust_anchor_id).unwrap())
    }

    pub fn get_trust_anchor(
        &self,
        trust_anchor_id: &str,
    ) -> Result<&TrustAnchor, RolesAnywhereError> {
        self.trust_anchors
            .get(trust_anchor_id)
            .ok_or_else(|| RolesAnywhereError::NotFound {
                id: trust_anchor_id.to_string(),
            })
    }

    pub fn delete_trust_anchor(
        &mut self,
        trust_anchor_id: &str,
    ) -> Result<TrustAnchor, RolesAnywhereError> {
        self.trust_anchors
            .remove(trust_anchor_id)
            .ok_or_else(|| RolesAnywhereError::NotFound {
                id: trust_anchor_id.to_string(),
            })
    }

    pub fn list_trust_anchors(&self) -> Vec<&TrustAnchor> {
        self.trust_anchors.values().collect()
    }

    pub fn update_trust_anchor(
        &mut self,
        trust_anchor_id: &str,
        name: Option<&str>,
        source: Option<Source>,
    ) -> Result<&TrustAnchor, RolesAnywhereError> {
        let ta = self.trust_anchors.get_mut(trust_anchor_id).ok_or_else(|| {
            RolesAnywhereError::NotFound {
                id: trust_anchor_id.to_string(),
            }
        })?;
        if let Some(n) = name {
            ta.name = n.to_string();
        }
        if let Some(s) = source {
            ta.source = s;
        }
        ta.updated_at = Utc::now();
        Ok(self.trust_anchors.get(trust_anchor_id).unwrap())
    }

    pub fn enable_trust_anchor(
        &mut self,
        trust_anchor_id: &str,
    ) -> Result<&TrustAnchor, RolesAnywhereError> {
        let ta = self.trust_anchors.get_mut(trust_anchor_id).ok_or_else(|| {
            RolesAnywhereError::NotFound {
                id: trust_anchor_id.to_string(),
            }
        })?;
        ta.enabled = true;
        ta.updated_at = Utc::now();
        Ok(self.trust_anchors.get(trust_anchor_id).unwrap())
    }

    pub fn disable_trust_anchor(
        &mut self,
        trust_anchor_id: &str,
    ) -> Result<&TrustAnchor, RolesAnywhereError> {
        let ta = self.trust_anchors.get_mut(trust_anchor_id).ok_or_else(|| {
            RolesAnywhereError::NotFound {
                id: trust_anchor_id.to_string(),
            }
        })?;
        ta.enabled = false;
        ta.updated_at = Utc::now();
        Ok(self.trust_anchors.get(trust_anchor_id).unwrap())
    }

    pub fn put_notification_settings(
        &mut self,
        trust_anchor_id: &str,
        settings: Vec<NotificationSettingDetail>,
        account_id: &str,
    ) -> Result<&TrustAnchor, RolesAnywhereError> {
        let ta = self.trust_anchors.get_mut(trust_anchor_id).ok_or_else(|| {
            RolesAnywhereError::NotFound {
                id: trust_anchor_id.to_string(),
            }
        })?;
        for setting in settings {
            // Replace existing setting with same event+channel, or add new
            let pos = ta
                .notification_settings
                .iter()
                .position(|s| s.event == setting.event && s.channel == setting.channel);
            let mut detail = setting;
            detail.configured_by = Some(account_id.to_string());
            if let Some(idx) = pos {
                ta.notification_settings[idx] = detail;
            } else {
                ta.notification_settings.push(detail);
            }
        }
        ta.updated_at = Utc::now();
        Ok(self.trust_anchors.get(trust_anchor_id).unwrap())
    }

    pub fn reset_notification_settings(
        &mut self,
        trust_anchor_id: &str,
        keys: Vec<(String, Option<String>)>,
    ) -> Result<&TrustAnchor, RolesAnywhereError> {
        let ta = self.trust_anchors.get_mut(trust_anchor_id).ok_or_else(|| {
            RolesAnywhereError::NotFound {
                id: trust_anchor_id.to_string(),
            }
        })?;
        for (event, channel) in keys {
            if let Some(setting) = ta
                .notification_settings
                .iter_mut()
                .find(|s| s.event == event && s.channel == channel)
            {
                setting.threshold = Some(45);
                setting.configured_by = Some("rolesanywhere.amazonaws.com".to_string());
            }
        }
        ta.updated_at = Utc::now();
        Ok(self.trust_anchors.get(trust_anchor_id).unwrap())
    }

    // ── CRL CRUD ──────────────────────────────────────

    pub fn import_crl(
        &mut self,
        name: &str,
        crl_data: Vec<u8>,
        trust_anchor_arn: &str,
        enabled: Option<bool>,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<&Crl, RolesAnywhereError> {
        let crl_id = Uuid::new_v4().to_string();
        let crl_arn = format!("arn:aws:rolesanywhere:{region}:{account_id}:crl/{crl_id}");
        let now = Utc::now();
        let tag_map: HashMap<String, String> = tags.into_iter().map(|t| (t.key, t.value)).collect();

        let crl = Crl {
            crl_id: crl_id.clone(),
            crl_arn,
            name: name.to_string(),
            enabled: enabled.unwrap_or(true),
            crl_data,
            trust_anchor_arn: trust_anchor_arn.to_string(),
            created_at: now,
            updated_at: now,
            tags: tag_map,
        };
        self.crls.insert(crl_id.clone(), crl);
        Ok(self.crls.get(&crl_id).unwrap())
    }

    pub fn get_crl(&self, crl_id: &str) -> Result<&Crl, RolesAnywhereError> {
        self.crls
            .get(crl_id)
            .ok_or_else(|| RolesAnywhereError::NotFound {
                id: crl_id.to_string(),
            })
    }

    pub fn delete_crl(&mut self, crl_id: &str) -> Result<Crl, RolesAnywhereError> {
        self.crls
            .remove(crl_id)
            .ok_or_else(|| RolesAnywhereError::NotFound {
                id: crl_id.to_string(),
            })
    }

    pub fn list_crls(&self) -> Vec<&Crl> {
        self.crls.values().collect()
    }

    pub fn update_crl(
        &mut self,
        crl_id: &str,
        name: Option<&str>,
        crl_data: Option<Vec<u8>>,
    ) -> Result<&Crl, RolesAnywhereError> {
        let crl = self
            .crls
            .get_mut(crl_id)
            .ok_or_else(|| RolesAnywhereError::NotFound {
                id: crl_id.to_string(),
            })?;
        if let Some(n) = name {
            crl.name = n.to_string();
        }
        if let Some(data) = crl_data {
            crl.crl_data = data;
        }
        crl.updated_at = Utc::now();
        Ok(self.crls.get(crl_id).unwrap())
    }

    pub fn enable_crl(&mut self, crl_id: &str) -> Result<&Crl, RolesAnywhereError> {
        let crl = self
            .crls
            .get_mut(crl_id)
            .ok_or_else(|| RolesAnywhereError::NotFound {
                id: crl_id.to_string(),
            })?;
        crl.enabled = true;
        crl.updated_at = Utc::now();
        Ok(self.crls.get(crl_id).unwrap())
    }

    pub fn disable_crl(&mut self, crl_id: &str) -> Result<&Crl, RolesAnywhereError> {
        let crl = self
            .crls
            .get_mut(crl_id)
            .ok_or_else(|| RolesAnywhereError::NotFound {
                id: crl_id.to_string(),
            })?;
        crl.enabled = false;
        crl.updated_at = Utc::now();
        Ok(self.crls.get(crl_id).unwrap())
    }

    // ── Tag operations ──────────────────────────────────────

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: Vec<Tag>,
    ) -> Result<(), RolesAnywhereError> {
        let tag_map = self.find_tags_mut(resource_arn)?;
        for tag in &tags {
            tag_map.insert(tag.key.clone(), tag.value.clone());
        }
        if tag_map.len() > 200 {
            return Err(RolesAnywhereError::TooManyTags);
        }
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), RolesAnywhereError> {
        let tag_map = self.find_tags_mut(resource_arn)?;
        for key in tag_keys {
            tag_map.remove(key);
        }
        Ok(())
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<Vec<Tag>, RolesAnywhereError> {
        let tag_map = self.find_tags(resource_arn)?;
        Ok(tag_map
            .iter()
            .map(|(k, v)| Tag {
                key: k.clone(),
                value: v.clone(),
            })
            .collect())
    }

    fn find_tags_mut(
        &mut self,
        resource_arn: &str,
    ) -> Result<&mut HashMap<String, String>, RolesAnywhereError> {
        // Try profiles
        for p in self.profiles.values_mut() {
            if p.profile_arn == resource_arn {
                return Ok(&mut p.tags);
            }
        }
        // Try trust anchors
        for ta in self.trust_anchors.values_mut() {
            if ta.trust_anchor_arn == resource_arn {
                return Ok(&mut ta.tags);
            }
        }
        // Try CRLs
        for crl in self.crls.values_mut() {
            if crl.crl_arn == resource_arn {
                return Ok(&mut crl.tags);
            }
        }
        Err(RolesAnywhereError::NotFound {
            id: resource_arn.to_string(),
        })
    }

    fn find_tags(
        &self,
        resource_arn: &str,
    ) -> Result<&HashMap<String, String>, RolesAnywhereError> {
        for p in self.profiles.values() {
            if p.profile_arn == resource_arn {
                return Ok(&p.tags);
            }
        }
        for ta in self.trust_anchors.values() {
            if ta.trust_anchor_arn == resource_arn {
                return Ok(&ta.tags);
            }
        }
        for crl in self.crls.values() {
            if crl.crl_arn == resource_arn {
                return Ok(&crl.tags);
            }
        }
        Err(RolesAnywhereError::NotFound {
            id: resource_arn.to_string(),
        })
    }
}
