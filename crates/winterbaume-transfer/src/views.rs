//! Serde-compatible view types for Transfer state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::TransferService;
use crate::state::TransferState;
use crate::types::{
    Agreement, Certificate, Connector, Profile, Server, SshPublicKey, Tag, User, WebApp,
    WebAppCustomization, Workflow,
};

/// Serializable view of the entire Transfer state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransferStateView {
    /// Servers keyed by server ID.
    #[serde(default)]
    pub servers: HashMap<String, ServerView>,
    #[serde(default)]
    pub agreements: HashMap<String, AgreementView>,
    #[serde(default)]
    pub certificates: HashMap<String, CertificateView>,
    #[serde(default)]
    pub connectors: HashMap<String, ConnectorView>,
    #[serde(default)]
    pub profiles: HashMap<String, ProfileView>,
    #[serde(default)]
    pub web_apps: HashMap<String, WebAppView>,
    #[serde(default)]
    pub workflows: HashMap<String, WorkflowView>,
}

/// Serializable view of a tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagView {
    pub key: String,
    pub value: String,
}

/// Serializable view of an SSH public key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshPublicKeyView {
    pub ssh_public_key_id: String,
    pub ssh_public_key_body: String,
    pub date_imported: String,
}

/// Serializable view of a Transfer user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserView {
    pub server_id: String,
    pub user_name: String,
    pub arn: String,
    pub role: String,
    pub home_directory: Option<String>,
    pub home_directory_type: String,
    #[serde(default)]
    pub tags: Vec<TagView>,
    #[serde(default)]
    pub ssh_public_keys: Vec<SshPublicKeyView>,
    #[serde(default)]
    pub home_directory_mappings: Option<serde_json::Value>,
    #[serde(default)]
    pub posix_profile: Option<serde_json::Value>,
}

/// Serializable view of a Transfer server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerView {
    pub arn: String,
    pub server_id: String,
    pub state: String,
    pub endpoint_type: String,
    pub identity_provider_type: String,
    #[serde(default)]
    pub protocols: Vec<String>,
    pub domain: String,
    #[serde(default)]
    pub tags: Vec<TagView>,
    pub created_at: String,
    /// Users keyed by user name.
    #[serde(default)]
    pub users: HashMap<String, UserView>,
    #[serde(default)]
    pub logging_role: Option<String>,
    #[serde(default)]
    pub certificate: Option<String>,
    #[serde(default)]
    pub security_policy_name: Option<String>,
    #[serde(default)]
    pub endpoint_details: Option<serde_json::Value>,
    #[serde(default)]
    pub protocol_details: Option<serde_json::Value>,
    #[serde(default)]
    pub workflow_details: Option<serde_json::Value>,
    #[serde(default)]
    pub s3_storage_options: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementView {
    pub agreement_id: String,
    pub arn: String,
    pub server_id: String,
    pub local_profile_id: String,
    pub partner_profile_id: String,
    pub base_directory: String,
    pub access_role: String,
    pub status: String,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<TagView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateView {
    pub certificate_id: String,
    pub arn: String,
    pub usage: String,
    pub status: String,
    pub certificate: String,
    pub certificate_chain: Option<String>,
    pub private_key: Option<String>,
    pub active_date: Option<f64>,
    pub inactive_date: Option<f64>,
    pub description: Option<String>,
    pub certificate_type: String,
    #[serde(default)]
    pub tags: Vec<TagView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorView {
    pub connector_id: String,
    pub arn: String,
    pub url: Option<String>,
    pub as2_config: Option<serde_json::Value>,
    pub sftp_config: Option<serde_json::Value>,
    pub access_role: String,
    pub logging_role: Option<String>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub tags: Vec<TagView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileView {
    pub profile_id: String,
    pub arn: String,
    pub profile_type: String,
    pub as2_id: String,
    #[serde(default)]
    pub certificate_ids: Vec<String>,
    #[serde(default)]
    pub tags: Vec<TagView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAppCustomizationView {
    pub title: Option<String>,
    pub logo_file: Option<String>,
    pub favicon_file: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAppView {
    pub web_app_id: String,
    pub arn: String,
    pub web_app_endpoint: Option<String>,
    #[serde(default)]
    pub tags: Vec<TagView>,
    pub customization: Option<WebAppCustomizationView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowView {
    pub workflow_id: String,
    pub arn: String,
    #[serde(default)]
    pub steps: Vec<serde_json::Value>,
    #[serde(default)]
    pub on_exception_steps: Vec<serde_json::Value>,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<TagView>,
}

// ---------------------------------------------------------------------------
// From conversions
// ---------------------------------------------------------------------------

impl From<&Tag> for TagView {
    fn from(t: &Tag) -> Self {
        TagView {
            key: t.key.clone(),
            value: t.value.clone(),
        }
    }
}

impl From<&SshPublicKey> for SshPublicKeyView {
    fn from(k: &SshPublicKey) -> Self {
        SshPublicKeyView {
            ssh_public_key_id: k.ssh_public_key_id.clone(),
            ssh_public_key_body: k.ssh_public_key_body.clone(),
            date_imported: k.date_imported.to_rfc3339(),
        }
    }
}

impl From<&User> for UserView {
    fn from(u: &User) -> Self {
        let extra = u.extra_config.as_ref();
        UserView {
            server_id: u.server_id.clone(),
            user_name: u.user_name.clone(),
            arn: u.arn.clone(),
            role: u.role.clone(),
            home_directory: u.home_directory.clone(),
            home_directory_type: u.home_directory_type.clone(),
            tags: u.tags.iter().map(TagView::from).collect(),
            ssh_public_keys: u
                .ssh_public_keys
                .iter()
                .map(SshPublicKeyView::from)
                .collect(),
            home_directory_mappings: extra
                .and_then(|e| e.get("home_directory_mappings"))
                .cloned(),
            posix_profile: extra.and_then(|e| e.get("posix_profile")).cloned(),
        }
    }
}

impl From<&Server> for ServerView {
    fn from(s: &Server) -> Self {
        let extra = s.extra_config.as_ref();
        ServerView {
            arn: s.arn.clone(),
            server_id: s.server_id.clone(),
            state: s.state.clone(),
            endpoint_type: s.endpoint_type.clone(),
            identity_provider_type: s.identity_provider_type.clone(),
            protocols: s.protocols.clone(),
            domain: s.domain.clone(),
            tags: s.tags.iter().map(TagView::from).collect(),
            created_at: s.created_at.to_rfc3339(),
            users: s
                .users
                .iter()
                .map(|(k, v)| (k.clone(), UserView::from(v)))
                .collect(),
            logging_role: s.logging_role.clone(),
            certificate: s.certificate.clone(),
            security_policy_name: s.security_policy_name.clone(),
            endpoint_details: extra.and_then(|e| e.get("endpoint_details")).cloned(),
            protocol_details: extra.and_then(|e| e.get("protocol_details")).cloned(),
            workflow_details: extra.and_then(|e| e.get("workflow_details")).cloned(),
            s3_storage_options: extra.and_then(|e| e.get("s3_storage_options")).cloned(),
        }
    }
}

impl From<&Agreement> for AgreementView {
    fn from(a: &Agreement) -> Self {
        AgreementView {
            agreement_id: a.agreement_id.clone(),
            arn: a.arn.clone(),
            server_id: a.server_id.clone(),
            local_profile_id: a.local_profile_id.clone(),
            partner_profile_id: a.partner_profile_id.clone(),
            base_directory: a.base_directory.clone(),
            access_role: a.access_role.clone(),
            status: a.status.clone(),
            description: a.description.clone(),
            tags: a.tags.iter().map(TagView::from).collect(),
        }
    }
}

impl From<&Certificate> for CertificateView {
    fn from(c: &Certificate) -> Self {
        CertificateView {
            certificate_id: c.certificate_id.clone(),
            arn: c.arn.clone(),
            usage: c.usage.clone(),
            status: c.status.clone(),
            certificate: c.certificate.clone(),
            certificate_chain: c.certificate_chain.clone(),
            private_key: c.private_key.clone(),
            active_date: c.active_date,
            inactive_date: c.inactive_date,
            description: c.description.clone(),
            certificate_type: c.certificate_type.clone(),
            tags: c.tags.iter().map(TagView::from).collect(),
        }
    }
}

impl From<&Connector> for ConnectorView {
    fn from(c: &Connector) -> Self {
        ConnectorView {
            connector_id: c.connector_id.clone(),
            arn: c.arn.clone(),
            url: c.url.clone(),
            as2_config: c.as2_config.clone(),
            sftp_config: c.sftp_config.clone(),
            access_role: c.access_role.clone(),
            logging_role: c.logging_role.clone(),
            status: c.status.clone(),
            tags: c.tags.iter().map(TagView::from).collect(),
        }
    }
}

impl From<&Profile> for ProfileView {
    fn from(p: &Profile) -> Self {
        ProfileView {
            profile_id: p.profile_id.clone(),
            arn: p.arn.clone(),
            profile_type: p.profile_type.clone(),
            as2_id: p.as2_id.clone(),
            certificate_ids: p.certificate_ids.clone(),
            tags: p.tags.iter().map(TagView::from).collect(),
        }
    }
}

impl From<&WebAppCustomization> for WebAppCustomizationView {
    fn from(c: &WebAppCustomization) -> Self {
        WebAppCustomizationView {
            title: c.title.clone(),
            logo_file: c.logo_file.clone(),
            favicon_file: c.favicon_file.clone(),
        }
    }
}

impl From<&WebApp> for WebAppView {
    fn from(w: &WebApp) -> Self {
        WebAppView {
            web_app_id: w.web_app_id.clone(),
            arn: w.arn.clone(),
            web_app_endpoint: w.web_app_endpoint.clone(),
            tags: w.tags.iter().map(TagView::from).collect(),
            customization: w.customization.as_ref().map(WebAppCustomizationView::from),
        }
    }
}

impl From<&Workflow> for WorkflowView {
    fn from(wf: &Workflow) -> Self {
        WorkflowView {
            workflow_id: wf.workflow_id.clone(),
            arn: wf.arn.clone(),
            steps: wf.steps.clone(),
            on_exception_steps: wf.on_exception_steps.clone(),
            description: wf.description.clone(),
            tags: wf.tags.iter().map(TagView::from).collect(),
        }
    }
}

impl From<&TransferState> for TransferStateView {
    fn from(s: &TransferState) -> Self {
        TransferStateView {
            servers: s
                .servers
                .iter()
                .map(|(k, v)| (k.clone(), ServerView::from(v)))
                .collect(),
            agreements: s
                .agreements
                .iter()
                .map(|(k, v)| (k.clone(), AgreementView::from(v)))
                .collect(),
            certificates: s
                .certificates
                .iter()
                .map(|(k, v)| (k.clone(), CertificateView::from(v)))
                .collect(),
            connectors: s
                .connectors
                .iter()
                .map(|(k, v)| (k.clone(), ConnectorView::from(v)))
                .collect(),
            profiles: s
                .profiles
                .iter()
                .map(|(k, v)| (k.clone(), ProfileView::from(v)))
                .collect(),
            web_apps: s
                .web_apps
                .iter()
                .map(|(k, v)| (k.clone(), WebAppView::from(v)))
                .collect(),
            workflows: s
                .workflows
                .iter()
                .map(|(k, v)| (k.clone(), WorkflowView::from(v)))
                .collect(),
        }
    }
}

// ---------------------------------------------------------------------------
// Helper conversion functions
// ---------------------------------------------------------------------------

fn tag_view_to_tag(tv: TagView) -> Tag {
    Tag {
        key: tv.key,
        value: tv.value,
    }
}

fn ssh_key_view_to_ssh_key(kv: SshPublicKeyView) -> SshPublicKey {
    let date_imported = DateTime::parse_from_rfc3339(&kv.date_imported)
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());
    SshPublicKey {
        ssh_public_key_id: kv.ssh_public_key_id,
        ssh_public_key_body: kv.ssh_public_key_body,
        date_imported,
    }
}

fn user_view_to_user(uv: UserView) -> User {
    let mut extra = serde_json::Map::new();
    if let Some(val) = uv.home_directory_mappings {
        extra.insert("home_directory_mappings".to_string(), val);
    }
    if let Some(val) = uv.posix_profile {
        extra.insert("posix_profile".to_string(), val);
    }
    let extra_config = if extra.is_empty() {
        None
    } else {
        Some(serde_json::Value::Object(extra))
    };
    User {
        server_id: uv.server_id,
        user_name: uv.user_name,
        arn: uv.arn,
        role: uv.role,
        home_directory: uv.home_directory,
        home_directory_type: uv.home_directory_type,
        tags: uv.tags.into_iter().map(tag_view_to_tag).collect(),
        ssh_public_keys: uv
            .ssh_public_keys
            .into_iter()
            .map(ssh_key_view_to_ssh_key)
            .collect(),
        extra_config,
    }
}

fn server_view_to_server(sv: ServerView) -> Server {
    let created_at = DateTime::parse_from_rfc3339(&sv.created_at)
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());
    let mut extra = serde_json::Map::new();
    if let Some(val) = sv.endpoint_details {
        extra.insert("endpoint_details".to_string(), val);
    }
    if let Some(val) = sv.protocol_details {
        extra.insert("protocol_details".to_string(), val);
    }
    if let Some(val) = sv.workflow_details {
        extra.insert("workflow_details".to_string(), val);
    }
    if let Some(val) = sv.s3_storage_options {
        extra.insert("s3_storage_options".to_string(), val);
    }
    let extra_config = if extra.is_empty() {
        None
    } else {
        Some(serde_json::Value::Object(extra))
    };
    Server {
        arn: sv.arn,
        server_id: sv.server_id,
        state: sv.state,
        endpoint_type: sv.endpoint_type,
        identity_provider_type: sv.identity_provider_type,
        protocols: sv.protocols,
        domain: sv.domain,
        tags: sv.tags.into_iter().map(tag_view_to_tag).collect(),
        created_at,
        users: sv
            .users
            .into_iter()
            .map(|(k, v)| (k, user_view_to_user(v)))
            .collect(),
        logging_role: sv.logging_role,
        certificate: sv.certificate,
        host_certificate_chain: None,
        security_policy_name: sv.security_policy_name,
        extra_config,
    }
}

fn agreement_view_to_agreement(av: AgreementView) -> Agreement {
    Agreement {
        agreement_id: av.agreement_id,
        arn: av.arn,
        server_id: av.server_id,
        local_profile_id: av.local_profile_id,
        partner_profile_id: av.partner_profile_id,
        base_directory: av.base_directory,
        access_role: av.access_role,
        status: av.status,
        description: av.description,
        tags: av.tags.into_iter().map(tag_view_to_tag).collect(),
    }
}

fn certificate_view_to_certificate(cv: CertificateView) -> Certificate {
    Certificate {
        certificate_id: cv.certificate_id,
        arn: cv.arn,
        usage: cv.usage,
        status: cv.status,
        certificate: cv.certificate,
        certificate_chain: cv.certificate_chain,
        private_key: cv.private_key,
        active_date: cv.active_date,
        inactive_date: cv.inactive_date,
        description: cv.description,
        certificate_type: cv.certificate_type,
        tags: cv.tags.into_iter().map(tag_view_to_tag).collect(),
    }
}

fn connector_view_to_connector(cv: ConnectorView) -> Connector {
    Connector {
        connector_id: cv.connector_id,
        arn: cv.arn,
        url: cv.url,
        as2_config: cv.as2_config,
        sftp_config: cv.sftp_config,
        access_role: cv.access_role,
        logging_role: cv.logging_role,
        // FIX(terraform-e2e): use "ACTIVE" as default; provider waits for "ACTIVE" status.
        status: if cv.status.is_empty() {
            "ACTIVE".to_string()
        } else {
            cv.status
        },
        tags: cv.tags.into_iter().map(tag_view_to_tag).collect(),
    }
}

fn profile_view_to_profile(pv: ProfileView) -> Profile {
    Profile {
        profile_id: pv.profile_id,
        arn: pv.arn,
        profile_type: pv.profile_type,
        as2_id: pv.as2_id,
        certificate_ids: pv.certificate_ids,
        tags: pv.tags.into_iter().map(tag_view_to_tag).collect(),
    }
}

fn web_app_view_to_web_app(wv: WebAppView) -> WebApp {
    WebApp {
        web_app_id: wv.web_app_id,
        arn: wv.arn,
        web_app_endpoint: wv.web_app_endpoint,
        tags: wv.tags.into_iter().map(tag_view_to_tag).collect(),
        customization: wv.customization.map(|c| WebAppCustomization {
            title: c.title,
            logo_file: c.logo_file,
            favicon_file: c.favicon_file,
        }),
    }
}

fn workflow_view_to_workflow(wfv: WorkflowView) -> Workflow {
    Workflow {
        workflow_id: wfv.workflow_id,
        arn: wfv.arn,
        steps: wfv.steps,
        on_exception_steps: wfv.on_exception_steps,
        description: wfv.description,
        tags: wfv.tags.into_iter().map(tag_view_to_tag).collect(),
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for TransferService {
    type StateView = TransferStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        TransferStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let mut new_state = TransferState::default();
        for (id, sv) in view.servers {
            new_state.servers.insert(id, server_view_to_server(sv));
        }
        for (id, av) in view.agreements {
            new_state
                .agreements
                .insert(id, agreement_view_to_agreement(av));
        }
        for (id, cv) in view.certificates {
            new_state
                .certificates
                .insert(id, certificate_view_to_certificate(cv));
        }
        for (id, cv) in view.connectors {
            new_state
                .connectors
                .insert(id, connector_view_to_connector(cv));
        }
        for (id, pv) in view.profiles {
            new_state.profiles.insert(id, profile_view_to_profile(pv));
        }
        for (id, wv) in view.web_apps {
            new_state.web_apps.insert(id, web_app_view_to_web_app(wv));
        }
        for (id, wfv) in view.workflows {
            new_state
                .workflows
                .insert(id, workflow_view_to_workflow(wfv));
        }
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
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (id, sv) in view.servers {
                guard.servers.insert(id, server_view_to_server(sv));
            }
            for (id, av) in view.agreements {
                guard.agreements.insert(id, agreement_view_to_agreement(av));
            }
            for (id, cv) in view.certificates {
                guard
                    .certificates
                    .insert(id, certificate_view_to_certificate(cv));
            }
            for (id, cv) in view.connectors {
                guard.connectors.insert(id, connector_view_to_connector(cv));
            }
            for (id, pv) in view.profiles {
                guard.profiles.insert(id, profile_view_to_profile(pv));
            }
            for (id, wv) in view.web_apps {
                guard.web_apps.insert(id, web_app_view_to_web_app(wv));
            }
            for (id, wfv) in view.workflows {
                guard.workflows.insert(id, workflow_view_to_workflow(wfv));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
