//! Serde-compatible view types for QuickSight state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::QuickSightService;
use crate::state::QuickSightState;

/// Serializable view of a dataset.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataSetView {
    pub data_set_id: String,
    pub name: String,
    pub arn: String,
    pub import_mode: String,
    #[serde(default)]
    pub physical_table_map: HashMap<String, Value>,
    pub created_time: String,
    pub last_updated_time: String,
}

/// Serializable view of a data source.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuickSightDataSourceView {
    pub data_source_id: String,
    pub name: String,
    pub arn: String,
    pub r#type: String,
    pub status: String,
    pub created_time: String,
    pub last_updated_time: String,
    /// `credentials` nested block.
    #[serde(default)]
    pub credentials: Option<serde_json::Value>,
    /// `parameters` nested block.
    #[serde(default)]
    pub parameters: Option<serde_json::Value>,
    /// `permission` nested blocks (terraform-shaped: each entry has
    /// `principal: String` and `actions: Vec<String>`).
    #[serde(default)]
    pub permission: Vec<serde_json::Value>,
    /// `vpc_connection_properties` nested block.
    #[serde(default)]
    pub vpc_connection_properties: Option<serde_json::Value>,
}

/// Serializable view of a dashboard.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DashboardView {
    pub dashboard_id: String,
    pub name: String,
    pub arn: String,
    pub version_arn: String,
    pub created_time: String,
    pub last_updated_time: String,
}

/// Serializable view of a group.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuickSightGroupView {
    pub group_name: String,
    pub arn: String,
    pub description: String,
    pub principal_id: String,
}

/// Serializable view of a user.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuickSightUserView {
    pub user_name: String,
    pub arn: String,
    pub email: String,
    pub role: String,
    pub identity_type: String,
    pub active: bool,
    pub principal_id: String,
}

/// Serializable view of an ingestion.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IngestionView {
    pub ingestion_id: String,
    pub arn: String,
    pub ingestion_status: String,
    pub data_set_id: String,
}

/// Serializable view of an analysis.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuickSightAnalysisView {
    pub analysis_id: String,
    pub name: String,
    pub arn: String,
    pub status: String,
    pub created_time: String,
    pub last_updated_time: String,
}

/// Serializable view of a folder.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuickSightFolderView {
    pub folder_id: String,
    pub name: String,
    pub arn: String,
    pub folder_type: String,
    pub created_time: String,
    pub last_updated_time: String,
}

/// Serializable view of a folder member entry.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FolderMemberEntryView {
    pub member_id: String,
    pub member_type: String,
}

/// Serializable view of a namespace.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuickSightNamespaceView {
    pub name: String,
    pub arn: String,
    pub capacity_region: String,
    pub creation_status: String,
    pub identity_store: String,
}

/// Serializable view of a template.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuickSightTemplateView {
    pub template_id: String,
    pub name: String,
    pub arn: String,
    pub version_arn: String,
    pub created_time: String,
    pub last_updated_time: String,
    pub version_number: i64,
}

/// Serializable view of a theme.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuickSightThemeView {
    pub theme_id: String,
    pub name: String,
    pub arn: String,
    pub version_arn: String,
    pub created_time: String,
    pub last_updated_time: String,
    pub version_number: i64,
}

/// Serializable view of account settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountSettingsView {
    pub account_name: String,
    pub edition: String,
    pub default_namespace: String,
    pub notification_email: String,
    pub public_sharing_enabled: bool,
    pub termination_protection_enabled: bool,
}

/// Serializable view of the entire QuickSight state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuickSightStateView {
    #[serde(default)]
    pub data_sets: HashMap<String, DataSetView>,
    #[serde(default)]
    pub data_sources: HashMap<String, QuickSightDataSourceView>,
    #[serde(default)]
    pub dashboards: HashMap<String, DashboardView>,
    #[serde(default)]
    pub groups: HashMap<String, HashMap<String, QuickSightGroupView>>,
    #[serde(default)]
    pub users: HashMap<String, HashMap<String, QuickSightUserView>>,
    #[serde(default)]
    pub group_memberships: HashMap<String, HashMap<String, Vec<String>>>,
    #[serde(default)]
    pub ingestions: HashMap<String, IngestionView>,
    #[serde(default)]
    pub account_settings: AccountSettingsView,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
    #[serde(default)]
    pub public_sharing_enabled: bool,
    #[serde(default)]
    pub analyses: HashMap<String, QuickSightAnalysisView>,
    #[serde(default)]
    pub folders: HashMap<String, QuickSightFolderView>,
    #[serde(default)]
    pub folder_members: HashMap<String, Vec<FolderMemberEntryView>>,
    #[serde(default)]
    pub namespaces: HashMap<String, QuickSightNamespaceView>,
    #[serde(default)]
    pub templates: HashMap<String, QuickSightTemplateView>,
    #[serde(default)]
    pub themes: HashMap<String, QuickSightThemeView>,
}

// --- From internal types to view types ---

fn dt_to_string(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

fn parse_dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

impl From<&crate::types::DataSet> for DataSetView {
    fn from(ds: &crate::types::DataSet) -> Self {
        DataSetView {
            data_set_id: ds.data_set_id.clone(),
            name: ds.name.clone(),
            arn: ds.arn.clone(),
            import_mode: ds.import_mode.clone(),
            physical_table_map: ds.physical_table_map.clone(),
            created_time: dt_to_string(&ds.created_time),
            last_updated_time: dt_to_string(&ds.last_updated_time),
        }
    }
}

impl From<&crate::types::QuickSightDataSource> for QuickSightDataSourceView {
    fn from(ds: &crate::types::QuickSightDataSource) -> Self {
        let permission = ds
            .permissions
            .iter()
            .map(|p| {
                serde_json::json!({
                    "principal": p.principal,
                    "actions": p.actions,
                })
            })
            .collect();
        QuickSightDataSourceView {
            data_source_id: ds.data_source_id.clone(),
            name: ds.name.clone(),
            arn: ds.arn.clone(),
            r#type: ds.r#type.clone(),
            status: ds.status.clone(),
            created_time: dt_to_string(&ds.created_time),
            last_updated_time: dt_to_string(&ds.last_updated_time),
            credentials: None,
            parameters: None,
            permission,
            vpc_connection_properties: None,
        }
    }
}

impl From<&crate::types::Dashboard> for DashboardView {
    fn from(db: &crate::types::Dashboard) -> Self {
        DashboardView {
            dashboard_id: db.dashboard_id.clone(),
            name: db.name.clone(),
            arn: db.arn.clone(),
            version_arn: db.version_arn.clone(),
            created_time: dt_to_string(&db.created_time),
            last_updated_time: dt_to_string(&db.last_updated_time),
        }
    }
}

impl From<&crate::types::QuickSightGroup> for QuickSightGroupView {
    fn from(g: &crate::types::QuickSightGroup) -> Self {
        QuickSightGroupView {
            group_name: g.group_name.clone(),
            arn: g.arn.clone(),
            description: g.description.clone(),
            principal_id: g.principal_id.clone(),
        }
    }
}

impl From<&crate::types::QuickSightUser> for QuickSightUserView {
    fn from(u: &crate::types::QuickSightUser) -> Self {
        QuickSightUserView {
            user_name: u.user_name.clone(),
            arn: u.arn.clone(),
            email: u.email.clone(),
            role: u.role.clone(),
            identity_type: u.identity_type.clone(),
            active: u.active,
            principal_id: u.principal_id.clone(),
        }
    }
}

impl From<&crate::types::Ingestion> for IngestionView {
    fn from(i: &crate::types::Ingestion) -> Self {
        IngestionView {
            ingestion_id: i.ingestion_id.clone(),
            arn: i.arn.clone(),
            ingestion_status: i.ingestion_status.clone(),
            data_set_id: i.data_set_id.clone(),
        }
    }
}

impl From<&crate::types::AccountSettings> for AccountSettingsView {
    fn from(s: &crate::types::AccountSettings) -> Self {
        AccountSettingsView {
            account_name: s.account_name.clone(),
            edition: s.edition.clone(),
            default_namespace: s.default_namespace.clone(),
            notification_email: s.notification_email.clone(),
            public_sharing_enabled: s.public_sharing_enabled,
            termination_protection_enabled: s.termination_protection_enabled,
        }
    }
}

impl From<&crate::types::QuickSightAnalysis> for QuickSightAnalysisView {
    fn from(a: &crate::types::QuickSightAnalysis) -> Self {
        QuickSightAnalysisView {
            analysis_id: a.analysis_id.clone(),
            name: a.name.clone(),
            arn: a.arn.clone(),
            status: a.status.clone(),
            created_time: dt_to_string(&a.created_time),
            last_updated_time: dt_to_string(&a.last_updated_time),
        }
    }
}

impl From<&crate::types::QuickSightFolder> for QuickSightFolderView {
    fn from(f: &crate::types::QuickSightFolder) -> Self {
        QuickSightFolderView {
            folder_id: f.folder_id.clone(),
            name: f.name.clone(),
            arn: f.arn.clone(),
            folder_type: f.folder_type.clone(),
            created_time: dt_to_string(&f.created_time),
            last_updated_time: dt_to_string(&f.last_updated_time),
        }
    }
}

impl From<&crate::types::FolderMemberEntry> for FolderMemberEntryView {
    fn from(m: &crate::types::FolderMemberEntry) -> Self {
        FolderMemberEntryView {
            member_id: m.member_id.clone(),
            member_type: m.member_type.clone(),
        }
    }
}

impl From<&crate::types::QuickSightNamespace> for QuickSightNamespaceView {
    fn from(ns: &crate::types::QuickSightNamespace) -> Self {
        QuickSightNamespaceView {
            name: ns.name.clone(),
            arn: ns.arn.clone(),
            capacity_region: ns.capacity_region.clone(),
            creation_status: ns.creation_status.clone(),
            identity_store: ns.identity_store.clone(),
        }
    }
}

impl From<&crate::types::QuickSightTemplate> for QuickSightTemplateView {
    fn from(t: &crate::types::QuickSightTemplate) -> Self {
        QuickSightTemplateView {
            template_id: t.template_id.clone(),
            name: t.name.clone(),
            arn: t.arn.clone(),
            version_arn: t.version_arn.clone(),
            created_time: dt_to_string(&t.created_time),
            last_updated_time: dt_to_string(&t.last_updated_time),
            version_number: t.version_number,
        }
    }
}

impl From<&crate::types::QuickSightTheme> for QuickSightThemeView {
    fn from(t: &crate::types::QuickSightTheme) -> Self {
        QuickSightThemeView {
            theme_id: t.theme_id.clone(),
            name: t.name.clone(),
            arn: t.arn.clone(),
            version_arn: t.version_arn.clone(),
            created_time: dt_to_string(&t.created_time),
            last_updated_time: dt_to_string(&t.last_updated_time),
            version_number: t.version_number,
        }
    }
}

impl From<&QuickSightState> for QuickSightStateView {
    fn from(state: &QuickSightState) -> Self {
        QuickSightStateView {
            data_sets: state
                .data_sets
                .iter()
                .map(|(k, v)| (k.clone(), DataSetView::from(v)))
                .collect(),
            data_sources: state
                .data_sources
                .iter()
                .map(|(k, v)| (k.clone(), QuickSightDataSourceView::from(v)))
                .collect(),
            dashboards: state
                .dashboards
                .iter()
                .map(|(k, v)| (k.clone(), DashboardView::from(v)))
                .collect(),
            groups: state
                .groups
                .iter()
                .map(|(ns, ns_groups)| {
                    (
                        ns.clone(),
                        ns_groups
                            .iter()
                            .map(|(gn, g)| (gn.clone(), QuickSightGroupView::from(g)))
                            .collect(),
                    )
                })
                .collect(),
            users: state
                .users
                .iter()
                .map(|(ns, ns_users)| {
                    (
                        ns.clone(),
                        ns_users
                            .iter()
                            .map(|(un, u)| (un.clone(), QuickSightUserView::from(u)))
                            .collect(),
                    )
                })
                .collect(),
            group_memberships: state.group_memberships.clone(),
            ingestions: state
                .ingestions
                .iter()
                .map(|(k, v)| (k.clone(), IngestionView::from(v)))
                .collect(),
            account_settings: AccountSettingsView::from(&state.account_settings),
            tags: state.tags.clone(),
            public_sharing_enabled: state.public_sharing_enabled,
            analyses: state
                .analyses
                .iter()
                .map(|(k, v)| (k.clone(), QuickSightAnalysisView::from(v)))
                .collect(),
            folders: state
                .folders
                .iter()
                .map(|(k, v)| (k.clone(), QuickSightFolderView::from(v)))
                .collect(),
            folder_members: state
                .folder_members
                .iter()
                .map(|(k, members)| {
                    (
                        k.clone(),
                        members.iter().map(FolderMemberEntryView::from).collect(),
                    )
                })
                .collect(),
            namespaces: state
                .namespaces
                .iter()
                .map(|(k, v)| (k.clone(), QuickSightNamespaceView::from(v)))
                .collect(),
            templates: state
                .templates
                .iter()
                .map(|(k, v)| (k.clone(), QuickSightTemplateView::from(v)))
                .collect(),
            themes: state
                .themes
                .iter()
                .map(|(k, v)| (k.clone(), QuickSightThemeView::from(v)))
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<DataSetView> for crate::types::DataSet {
    fn from(v: DataSetView) -> Self {
        crate::types::DataSet {
            data_set_id: v.data_set_id,
            name: v.name,
            arn: v.arn,
            import_mode: v.import_mode,
            physical_table_map: v.physical_table_map,
            created_time: parse_dt(&v.created_time),
            last_updated_time: parse_dt(&v.last_updated_time),
        }
    }
}

impl From<QuickSightDataSourceView> for crate::types::QuickSightDataSource {
    fn from(v: QuickSightDataSourceView) -> Self {
        let permissions = v
            .permission
            .iter()
            .map(|entry| {
                let principal = entry
                    .get("principal")
                    .and_then(|p| p.as_str())
                    .unwrap_or_default()
                    .to_string();
                let actions = entry
                    .get("actions")
                    .and_then(|a| a.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|x| x.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default();
                crate::types::DataSourceResourcePermission { principal, actions }
            })
            .collect();
        crate::types::QuickSightDataSource {
            data_source_id: v.data_source_id,
            name: v.name,
            arn: v.arn,
            r#type: v.r#type,
            status: v.status,
            created_time: parse_dt(&v.created_time),
            last_updated_time: parse_dt(&v.last_updated_time),
            permissions,
        }
    }
}

impl From<DashboardView> for crate::types::Dashboard {
    fn from(v: DashboardView) -> Self {
        crate::types::Dashboard {
            dashboard_id: v.dashboard_id,
            name: v.name,
            arn: v.arn,
            version_arn: v.version_arn,
            created_time: parse_dt(&v.created_time),
            last_updated_time: parse_dt(&v.last_updated_time),
        }
    }
}

impl From<QuickSightGroupView> for crate::types::QuickSightGroup {
    fn from(v: QuickSightGroupView) -> Self {
        crate::types::QuickSightGroup {
            group_name: v.group_name,
            arn: v.arn,
            description: v.description,
            principal_id: v.principal_id,
        }
    }
}

impl From<QuickSightUserView> for crate::types::QuickSightUser {
    fn from(v: QuickSightUserView) -> Self {
        crate::types::QuickSightUser {
            user_name: v.user_name,
            arn: v.arn,
            email: v.email,
            role: v.role,
            identity_type: v.identity_type,
            active: v.active,
            principal_id: v.principal_id,
        }
    }
}

impl From<IngestionView> for crate::types::Ingestion {
    fn from(v: IngestionView) -> Self {
        crate::types::Ingestion {
            ingestion_id: v.ingestion_id,
            arn: v.arn,
            ingestion_status: v.ingestion_status,
            data_set_id: v.data_set_id,
        }
    }
}

impl From<AccountSettingsView> for crate::types::AccountSettings {
    fn from(v: AccountSettingsView) -> Self {
        crate::types::AccountSettings {
            account_name: v.account_name,
            edition: v.edition,
            default_namespace: v.default_namespace,
            notification_email: v.notification_email,
            public_sharing_enabled: v.public_sharing_enabled,
            termination_protection_enabled: v.termination_protection_enabled,
        }
    }
}

impl From<QuickSightAnalysisView> for crate::types::QuickSightAnalysis {
    fn from(v: QuickSightAnalysisView) -> Self {
        crate::types::QuickSightAnalysis {
            analysis_id: v.analysis_id,
            name: v.name,
            arn: v.arn,
            status: v.status,
            created_time: parse_dt(&v.created_time),
            last_updated_time: parse_dt(&v.last_updated_time),
        }
    }
}

impl From<QuickSightFolderView> for crate::types::QuickSightFolder {
    fn from(v: QuickSightFolderView) -> Self {
        crate::types::QuickSightFolder {
            folder_id: v.folder_id,
            name: v.name,
            arn: v.arn,
            folder_type: v.folder_type,
            created_time: parse_dt(&v.created_time),
            last_updated_time: parse_dt(&v.last_updated_time),
            member_ids: Vec::new(),
        }
    }
}

impl From<FolderMemberEntryView> for crate::types::FolderMemberEntry {
    fn from(v: FolderMemberEntryView) -> Self {
        crate::types::FolderMemberEntry {
            member_id: v.member_id,
            member_type: v.member_type,
        }
    }
}

impl From<QuickSightNamespaceView> for crate::types::QuickSightNamespace {
    fn from(v: QuickSightNamespaceView) -> Self {
        crate::types::QuickSightNamespace {
            name: v.name,
            arn: v.arn,
            capacity_region: v.capacity_region,
            creation_status: v.creation_status,
            identity_store: v.identity_store,
        }
    }
}

impl From<QuickSightTemplateView> for crate::types::QuickSightTemplate {
    fn from(v: QuickSightTemplateView) -> Self {
        crate::types::QuickSightTemplate {
            template_id: v.template_id,
            name: v.name,
            arn: v.arn,
            version_arn: v.version_arn,
            created_time: parse_dt(&v.created_time),
            last_updated_time: parse_dt(&v.last_updated_time),
            version_number: v.version_number,
        }
    }
}

impl From<QuickSightThemeView> for crate::types::QuickSightTheme {
    fn from(v: QuickSightThemeView) -> Self {
        crate::types::QuickSightTheme {
            theme_id: v.theme_id,
            name: v.name,
            arn: v.arn,
            version_arn: v.version_arn,
            created_time: parse_dt(&v.created_time),
            last_updated_time: parse_dt(&v.last_updated_time),
            version_number: v.version_number,
        }
    }
}

impl From<QuickSightStateView> for QuickSightState {
    fn from(view: QuickSightStateView) -> Self {
        QuickSightState {
            data_sets: view
                .data_sets
                .into_iter()
                .map(|(k, v)| (k, crate::types::DataSet::from(v)))
                .collect(),
            data_sources: view
                .data_sources
                .into_iter()
                .map(|(k, v)| (k, crate::types::QuickSightDataSource::from(v)))
                .collect(),
            dashboards: view
                .dashboards
                .into_iter()
                .map(|(k, v)| (k, crate::types::Dashboard::from(v)))
                .collect(),
            groups: view
                .groups
                .into_iter()
                .map(|(ns, ns_groups)| {
                    (
                        ns,
                        ns_groups
                            .into_iter()
                            .map(|(gn, g)| (gn, crate::types::QuickSightGroup::from(g)))
                            .collect(),
                    )
                })
                .collect(),
            users: view
                .users
                .into_iter()
                .map(|(ns, ns_users)| {
                    (
                        ns,
                        ns_users
                            .into_iter()
                            .map(|(un, u)| (un, crate::types::QuickSightUser::from(u)))
                            .collect(),
                    )
                })
                .collect(),
            group_memberships: view.group_memberships,
            ingestions: view
                .ingestions
                .into_iter()
                .map(|(k, v)| (k, crate::types::Ingestion::from(v)))
                .collect(),
            account_settings: crate::types::AccountSettings::from(view.account_settings),
            tags: view.tags,
            public_sharing_enabled: view.public_sharing_enabled,
            analyses: view
                .analyses
                .into_iter()
                .map(|(k, v)| (k, crate::types::QuickSightAnalysis::from(v)))
                .collect(),
            folders: view
                .folders
                .into_iter()
                .map(|(k, v)| (k, crate::types::QuickSightFolder::from(v)))
                .collect(),
            folder_members: view
                .folder_members
                .into_iter()
                .map(|(k, members)| {
                    (
                        k,
                        members
                            .into_iter()
                            .map(crate::types::FolderMemberEntry::from)
                            .collect(),
                    )
                })
                .collect(),
            namespaces: view
                .namespaces
                .into_iter()
                .map(|(k, v)| (k, crate::types::QuickSightNamespace::from(v)))
                .collect(),
            templates: view
                .templates
                .into_iter()
                .map(|(k, v)| (k, crate::types::QuickSightTemplate::from(v)))
                .collect(),
            themes: view
                .themes
                .into_iter()
                .map(|(k, v)| (k, crate::types::QuickSightTheme::from(v)))
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for QuickSightService {
    type StateView = QuickSightStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        QuickSightStateView::from(&*guard)
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
            *guard = QuickSightState::from(view);
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
            for (id, ds_view) in view.data_sets {
                guard
                    .data_sets
                    .insert(id, crate::types::DataSet::from(ds_view));
            }
            for (id, ds_view) in view.data_sources {
                guard
                    .data_sources
                    .insert(id, crate::types::QuickSightDataSource::from(ds_view));
            }
            for (id, db_view) in view.dashboards {
                guard
                    .dashboards
                    .insert(id, crate::types::Dashboard::from(db_view));
            }
            for (ns, ns_groups) in view.groups {
                let entry = guard.groups.entry(ns).or_default();
                for (gn, g_view) in ns_groups {
                    entry.insert(gn, crate::types::QuickSightGroup::from(g_view));
                }
            }
            for (ns, ns_users) in view.users {
                let entry = guard.users.entry(ns).or_default();
                for (un, u_view) in ns_users {
                    entry.insert(un, crate::types::QuickSightUser::from(u_view));
                }
            }
            for (ns, ns_memberships) in view.group_memberships {
                let entry = guard.group_memberships.entry(ns).or_default();
                for (gn, members) in ns_memberships {
                    entry.insert(gn, members);
                }
            }
            for (key, ingestion_view) in view.ingestions {
                guard
                    .ingestions
                    .insert(key, crate::types::Ingestion::from(ingestion_view));
            }
            for (arn, tag_map) in view.tags {
                guard.tags.entry(arn).or_default().extend(tag_map);
            }
            for (id, a_view) in view.analyses {
                guard
                    .analyses
                    .insert(id, crate::types::QuickSightAnalysis::from(a_view));
            }
            for (id, f_view) in view.folders {
                guard
                    .folders
                    .insert(id, crate::types::QuickSightFolder::from(f_view));
            }
            for (id, members) in view.folder_members {
                let entry = guard.folder_members.entry(id).or_default();
                for m in members {
                    entry.push(crate::types::FolderMemberEntry::from(m));
                }
            }
            for (id, ns_view) in view.namespaces {
                guard
                    .namespaces
                    .insert(id, crate::types::QuickSightNamespace::from(ns_view));
            }
            for (id, t_view) in view.templates {
                guard
                    .templates
                    .insert(id, crate::types::QuickSightTemplate::from(t_view));
            }
            for (id, th_view) in view.themes {
                guard
                    .themes
                    .insert(id, crate::types::QuickSightTheme::from(th_view));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
