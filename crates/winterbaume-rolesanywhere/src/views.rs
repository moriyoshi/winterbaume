//! Serde-compatible view types for IAM Roles Anywhere state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::RolesAnywhereService;
use crate::state::RolesAnywhereState;
use crate::types::{
    AttributeMapping, Crl, MappingRule, NotificationSettingDetail, Profile, Source, SourceData,
    TrustAnchor,
};

/// Serializable view of the entire IAM Roles Anywhere state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RolesAnywhereStateView {
    #[serde(default)]
    pub profiles: HashMap<String, ProfileView>,
    #[serde(default)]
    pub trust_anchors: HashMap<String, TrustAnchorView>,
    #[serde(default)]
    pub crls: HashMap<String, CrlView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileView {
    pub profile_id: String,
    pub profile_arn: String,
    pub name: String,
    pub enabled: bool,
    #[serde(default)]
    pub role_arns: Vec<String>,
    #[serde(default)]
    pub managed_policy_arns: Vec<String>,
    pub session_policy: Option<String>,
    pub duration_seconds: Option<i32>,
    pub require_instance_properties: Option<bool>,
    pub accept_role_session_name: Option<bool>,
    #[serde(default)]
    pub attribute_mappings: Vec<AttributeMappingView>,
    pub created_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeMappingView {
    pub certificate_field: String,
    #[serde(default)]
    pub mapping_rules: Vec<MappingRuleView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingRuleView {
    pub specifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustAnchorView {
    pub trust_anchor_id: String,
    pub trust_anchor_arn: String,
    pub name: String,
    pub source_type: Option<String>,
    pub source_data: Option<SourceDataView>,
    pub enabled: bool,
    #[serde(default)]
    pub notification_settings: Vec<NotificationSettingDetailView>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceDataView {
    X509CertificateData(String),
    AcmPcaArn(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettingDetailView {
    pub enabled: bool,
    pub event: String,
    pub threshold: Option<i32>,
    pub channel: Option<String>,
    pub configured_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrlView {
    pub crl_id: String,
    pub crl_arn: String,
    pub name: String,
    pub enabled: bool,
    /// CRL data stored as base64-encoded string.
    pub crl_data: String,
    pub trust_anchor_arn: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

// ── From conversions ──────────────────────────────────────

impl From<&Profile> for ProfileView {
    fn from(p: &Profile) -> Self {
        Self {
            profile_id: p.profile_id.clone(),
            profile_arn: p.profile_arn.clone(),
            name: p.name.clone(),
            enabled: p.enabled,
            role_arns: p.role_arns.clone(),
            managed_policy_arns: p.managed_policy_arns.clone(),
            session_policy: p.session_policy.clone(),
            duration_seconds: p.duration_seconds,
            require_instance_properties: p.require_instance_properties,
            accept_role_session_name: p.accept_role_session_name,
            attribute_mappings: p
                .attribute_mappings
                .iter()
                .map(AttributeMappingView::from)
                .collect(),
            created_by: p.created_by.clone(),
            created_at: p.created_at.to_rfc3339(),
            updated_at: p.updated_at.to_rfc3339(),
            tags: p.tags.clone(),
        }
    }
}

impl From<&AttributeMapping> for AttributeMappingView {
    fn from(am: &AttributeMapping) -> Self {
        Self {
            certificate_field: am.certificate_field.clone(),
            mapping_rules: am.mapping_rules.iter().map(MappingRuleView::from).collect(),
        }
    }
}

impl From<&MappingRule> for MappingRuleView {
    fn from(mr: &MappingRule) -> Self {
        Self {
            specifier: mr.specifier.clone(),
        }
    }
}

impl From<&TrustAnchor> for TrustAnchorView {
    fn from(ta: &TrustAnchor) -> Self {
        Self {
            trust_anchor_id: ta.trust_anchor_id.clone(),
            trust_anchor_arn: ta.trust_anchor_arn.clone(),
            name: ta.name.clone(),
            source_type: ta.source.source_type.clone(),
            source_data: ta.source.source_data.as_ref().map(SourceDataView::from),
            enabled: ta.enabled,
            notification_settings: ta
                .notification_settings
                .iter()
                .map(NotificationSettingDetailView::from)
                .collect(),
            created_at: ta.created_at.to_rfc3339(),
            updated_at: ta.updated_at.to_rfc3339(),
            tags: ta.tags.clone(),
        }
    }
}

impl From<&SourceData> for SourceDataView {
    fn from(sd: &SourceData) -> Self {
        match sd {
            SourceData::X509CertificateData(data) => {
                SourceDataView::X509CertificateData(data.clone())
            }
            SourceData::AcmPcaArn(arn) => SourceDataView::AcmPcaArn(arn.clone()),
        }
    }
}

impl From<&NotificationSettingDetail> for NotificationSettingDetailView {
    fn from(ns: &NotificationSettingDetail) -> Self {
        Self {
            enabled: ns.enabled,
            event: ns.event.clone(),
            threshold: ns.threshold,
            channel: ns.channel.clone(),
            configured_by: ns.configured_by.clone(),
        }
    }
}

impl From<&Crl> for CrlView {
    fn from(c: &Crl) -> Self {
        use base64::Engine;
        Self {
            crl_id: c.crl_id.clone(),
            crl_arn: c.crl_arn.clone(),
            name: c.name.clone(),
            enabled: c.enabled,
            crl_data: base64::engine::general_purpose::STANDARD.encode(&c.crl_data),
            trust_anchor_arn: c.trust_anchor_arn.clone(),
            created_at: c.created_at.to_rfc3339(),
            updated_at: c.updated_at.to_rfc3339(),
            tags: c.tags.clone(),
        }
    }
}

// ── From view to state ──────────────────────────────────────

impl From<ProfileView> for Profile {
    fn from(v: ProfileView) -> Self {
        Self {
            profile_id: v.profile_id,
            profile_arn: v.profile_arn,
            name: v.name,
            enabled: v.enabled,
            role_arns: v.role_arns,
            managed_policy_arns: v.managed_policy_arns,
            session_policy: v.session_policy,
            duration_seconds: v.duration_seconds,
            require_instance_properties: v.require_instance_properties,
            accept_role_session_name: v.accept_role_session_name,
            attribute_mappings: v
                .attribute_mappings
                .into_iter()
                .map(AttributeMapping::from)
                .collect(),
            created_by: v.created_by,
            created_at: chrono::DateTime::parse_from_rfc3339(&v.created_at)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&v.updated_at)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            tags: v.tags,
        }
    }
}

impl From<AttributeMappingView> for AttributeMapping {
    fn from(v: AttributeMappingView) -> Self {
        Self {
            certificate_field: v.certificate_field,
            mapping_rules: v.mapping_rules.into_iter().map(MappingRule::from).collect(),
        }
    }
}

impl From<MappingRuleView> for MappingRule {
    fn from(v: MappingRuleView) -> Self {
        Self {
            specifier: v.specifier,
        }
    }
}

impl From<TrustAnchorView> for TrustAnchor {
    fn from(v: TrustAnchorView) -> Self {
        Self {
            trust_anchor_id: v.trust_anchor_id,
            trust_anchor_arn: v.trust_anchor_arn,
            name: v.name,
            source: Source {
                source_type: v.source_type,
                source_data: v.source_data.map(SourceData::from),
            },
            enabled: v.enabled,
            notification_settings: v
                .notification_settings
                .into_iter()
                .map(NotificationSettingDetail::from)
                .collect(),
            created_at: chrono::DateTime::parse_from_rfc3339(&v.created_at)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&v.updated_at)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            tags: v.tags,
        }
    }
}

impl From<SourceDataView> for SourceData {
    fn from(v: SourceDataView) -> Self {
        match v {
            SourceDataView::X509CertificateData(data) => SourceData::X509CertificateData(data),
            SourceDataView::AcmPcaArn(arn) => SourceData::AcmPcaArn(arn),
        }
    }
}

impl From<NotificationSettingDetailView> for NotificationSettingDetail {
    fn from(v: NotificationSettingDetailView) -> Self {
        Self {
            enabled: v.enabled,
            event: v.event,
            threshold: v.threshold,
            channel: v.channel,
            configured_by: v.configured_by,
        }
    }
}

impl From<CrlView> for Crl {
    fn from(v: CrlView) -> Self {
        use base64::Engine;
        Self {
            crl_id: v.crl_id,
            crl_arn: v.crl_arn,
            name: v.name,
            enabled: v.enabled,
            crl_data: base64::engine::general_purpose::STANDARD
                .decode(&v.crl_data)
                .unwrap_or_default(),
            trust_anchor_arn: v.trust_anchor_arn,
            created_at: chrono::DateTime::parse_from_rfc3339(&v.created_at)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&v.updated_at)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            tags: v.tags,
        }
    }
}

// ── State conversions ──────────────────────────────────────

impl From<&RolesAnywhereState> for RolesAnywhereStateView {
    fn from(state: &RolesAnywhereState) -> Self {
        Self {
            profiles: state
                .profiles
                .iter()
                .map(|(k, v)| (k.clone(), ProfileView::from(v)))
                .collect(),
            trust_anchors: state
                .trust_anchors
                .iter()
                .map(|(k, v)| (k.clone(), TrustAnchorView::from(v)))
                .collect(),
            crls: state
                .crls
                .iter()
                .map(|(k, v)| (k.clone(), CrlView::from(v)))
                .collect(),
        }
    }
}

impl From<RolesAnywhereStateView> for RolesAnywhereState {
    fn from(view: RolesAnywhereStateView) -> Self {
        Self {
            profiles: view
                .profiles
                .into_iter()
                .map(|(k, v)| (k, Profile::from(v)))
                .collect(),
            trust_anchors: view
                .trust_anchors
                .into_iter()
                .map(|(k, v)| (k, TrustAnchor::from(v)))
                .collect(),
            crls: view
                .crls
                .into_iter()
                .map(|(k, v)| (k, Crl::from(v)))
                .collect(),
        }
    }
}

// ── StatefulService implementation ──────────────────────────

impl StatefulService for RolesAnywhereService {
    type StateView = RolesAnywhereStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        RolesAnywhereStateView::from(&*guard)
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
            *guard = RolesAnywhereState::from(view);
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
            for (id, profile_view) in view.profiles {
                guard.profiles.insert(id, Profile::from(profile_view));
            }
            for (id, ta_view) in view.trust_anchors {
                guard.trust_anchors.insert(id, TrustAnchor::from(ta_view));
            }
            for (id, crl_view) in view.crls {
                guard.crls.insert(id, Crl::from(crl_view));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
