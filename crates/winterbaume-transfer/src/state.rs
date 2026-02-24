use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct TransferState {
    pub servers: HashMap<String, Server>,
    pub agreements: HashMap<String, Agreement>,
    pub certificates: HashMap<String, Certificate>,
    pub connectors: HashMap<String, Connector>,
    pub profiles: HashMap<String, Profile>,
    pub web_apps: HashMap<String, WebApp>,
    pub workflows: HashMap<String, Workflow>,
}

#[derive(Debug, thiserror::Error)]
pub enum TransferError {
    #[error("Unknown server: '{0}' is not found.")]
    ServerNotFound(String),
    #[error("Unknown user: '{0}' is not found.")]
    UserNotFound(String),
    #[error("User '{0}' already exists.")]
    UserAlreadyExists(String),
    #[error("SSH public key '{0}' not found.")]
    SshPublicKeyNotFound(String),
    #[error("Unknown agreement: '{0}' is not found.")]
    AgreementNotFound(String),
    #[error("Unknown certificate: '{0}' is not found.")]
    CertificateNotFound(String),
    #[error("Unknown connector: '{0}' is not found.")]
    ConnectorNotFound(String),
    #[error("Unknown profile: '{0}' is not found.")]
    ProfileNotFound(String),
    #[error("Unknown web app: '{0}' is not found.")]
    WebAppNotFound(String),
    #[error("Unknown workflow: '{0}' is not found.")]
    WorkflowNotFound(String),
}

impl TransferState {
    pub fn create_server(
        &mut self,
        account_id: &str,
        region: &str,
        endpoint_type: &str,
        identity_provider_type: &str,
        protocols: Vec<String>,
        domain: &str,
        tags: Vec<Tag>,
    ) -> Result<&Server, TransferError> {
        let server_id = format!("s-{}", &uuid::Uuid::new_v4().simple().to_string()[..17]);
        let arn = format!("arn:aws:transfer:{region}:{account_id}:server/{server_id}");

        let server = Server {
            arn: arn.clone(),
            server_id: server_id.clone(),
            state: "ONLINE".to_string(),
            endpoint_type: endpoint_type.to_string(),
            identity_provider_type: identity_provider_type.to_string(),
            protocols,
            domain: domain.to_string(),
            tags,
            created_at: Utc::now(),
            users: HashMap::new(),
            logging_role: None,
            certificate: None,
            host_certificate_chain: None,
            security_policy_name: None,
            extra_config: None,
        };

        self.servers.insert(server_id.clone(), server);
        Ok(self.servers.get(&server_id).unwrap())
    }

    pub fn describe_server(&self, server_id: &str) -> Result<&Server, TransferError> {
        self.servers
            .get(server_id)
            .ok_or_else(|| TransferError::ServerNotFound(server_id.to_string()))
    }

    pub fn list_servers(&self) -> Vec<&Server> {
        self.servers.values().collect()
    }

    pub fn delete_server(&mut self, server_id: &str) -> Result<(), TransferError> {
        if self.servers.remove(server_id).is_none() {
            return Err(TransferError::ServerNotFound(server_id.to_string()));
        }
        Ok(())
    }

    pub fn update_server(
        &mut self,
        server_id: &str,
        protocols: Option<Vec<String>>,
        endpoint_type: Option<&str>,
        logging_role: Option<&str>,
        certificate: Option<&str>,
        security_policy_name: Option<&str>,
    ) -> Result<String, TransferError> {
        let server = self
            .servers
            .get_mut(server_id)
            .ok_or_else(|| TransferError::ServerNotFound(server_id.to_string()))?;

        if let Some(p) = protocols {
            server.protocols = p;
        }
        if let Some(et) = endpoint_type {
            server.endpoint_type = et.to_string();
        }
        if let Some(lr) = logging_role {
            server.logging_role = Some(lr.to_string());
        }
        if let Some(c) = certificate {
            server.certificate = Some(c.to_string());
        }
        if let Some(sp) = security_policy_name {
            server.security_policy_name = Some(sp.to_string());
        }

        Ok(server_id.to_string())
    }

    pub fn create_user(
        &mut self,
        server_id: &str,
        user_name: &str,
        role: &str,
        home_directory: Option<&str>,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<&User, TransferError> {
        let server = self
            .servers
            .get_mut(server_id)
            .ok_or_else(|| TransferError::ServerNotFound(server_id.to_string()))?;

        if server.users.contains_key(user_name) {
            return Err(TransferError::UserAlreadyExists(user_name.to_string()));
        }

        let arn = format!(
            "arn:aws:transfer:{}:{}:user/{}/{}",
            region, account_id, server_id, user_name
        );

        let user = User {
            server_id: server_id.to_string(),
            user_name: user_name.to_string(),
            arn,
            role: role.to_string(),
            home_directory: home_directory.map(|s| s.to_string()),
            home_directory_type: "PATH".to_string(),
            tags,
            ssh_public_keys: Vec::new(),
            extra_config: None,
        };

        server.users.insert(user_name.to_string(), user);
        Ok(server.users.get(user_name).unwrap())
    }

    pub fn describe_user(&self, server_id: &str, user_name: &str) -> Result<&User, TransferError> {
        let server = self
            .servers
            .get(server_id)
            .ok_or_else(|| TransferError::ServerNotFound(server_id.to_string()))?;

        server
            .users
            .get(user_name)
            .ok_or_else(|| TransferError::UserNotFound(user_name.to_string()))
    }

    pub fn list_users(&self, server_id: &str) -> Result<Vec<&User>, TransferError> {
        let server = self
            .servers
            .get(server_id)
            .ok_or_else(|| TransferError::ServerNotFound(server_id.to_string()))?;
        Ok(server.users.values().collect())
    }

    pub fn delete_user(&mut self, server_id: &str, user_name: &str) -> Result<(), TransferError> {
        let server = self
            .servers
            .get_mut(server_id)
            .ok_or_else(|| TransferError::ServerNotFound(server_id.to_string()))?;

        if server.users.remove(user_name).is_none() {
            return Err(TransferError::UserNotFound(user_name.to_string()));
        }
        Ok(())
    }

    pub fn update_user(
        &mut self,
        server_id: &str,
        user_name: &str,
        role: Option<&str>,
        home_directory: Option<&str>,
        home_directory_type: Option<&str>,
    ) -> Result<(String, String), TransferError> {
        let server = self
            .servers
            .get_mut(server_id)
            .ok_or_else(|| TransferError::ServerNotFound(server_id.to_string()))?;

        let user = server
            .users
            .get_mut(user_name)
            .ok_or_else(|| TransferError::UserNotFound(user_name.to_string()))?;

        if let Some(r) = role {
            user.role = r.to_string();
        }
        if let Some(hd) = home_directory {
            user.home_directory = Some(hd.to_string());
        }
        if let Some(hdt) = home_directory_type {
            user.home_directory_type = hdt.to_string();
        }

        Ok((server_id.to_string(), user_name.to_string()))
    }

    pub fn import_ssh_public_key(
        &mut self,
        server_id: &str,
        user_name: &str,
        ssh_public_key_body: &str,
    ) -> Result<String, TransferError> {
        let server = self
            .servers
            .get_mut(server_id)
            .ok_or_else(|| TransferError::ServerNotFound(server_id.to_string()))?;

        let user = server
            .users
            .get_mut(user_name)
            .ok_or_else(|| TransferError::UserNotFound(user_name.to_string()))?;

        let key_id = format!("key-{}", &uuid::Uuid::new_v4().simple().to_string()[..17]);
        let ssh_key = SshPublicKey {
            ssh_public_key_id: key_id.clone(),
            ssh_public_key_body: ssh_public_key_body.to_string(),
            date_imported: Utc::now(),
        };

        user.ssh_public_keys.push(ssh_key);
        Ok(key_id)
    }

    pub fn delete_ssh_public_key(
        &mut self,
        server_id: &str,
        user_name: &str,
        ssh_public_key_id: &str,
    ) -> Result<(), TransferError> {
        let server = self
            .servers
            .get_mut(server_id)
            .ok_or_else(|| TransferError::ServerNotFound(server_id.to_string()))?;

        let user = server
            .users
            .get_mut(user_name)
            .ok_or_else(|| TransferError::UserNotFound(user_name.to_string()))?;

        let orig_len = user.ssh_public_keys.len();
        user.ssh_public_keys
            .retain(|k| k.ssh_public_key_id != ssh_public_key_id);
        if user.ssh_public_keys.len() == orig_len {
            return Err(TransferError::SshPublicKeyNotFound(
                ssh_public_key_id.to_string(),
            ));
        }
        Ok(())
    }

    // --- Agreements ---

    pub fn create_agreement(
        &mut self,
        account_id: &str,
        region: &str,
        server_id: &str,
        local_profile_id: &str,
        partner_profile_id: &str,
        base_directory: &str,
        access_role: &str,
        description: Option<&str>,
        tags: Vec<Tag>,
    ) -> Result<String, TransferError> {
        // Verify server exists
        if !self.servers.contains_key(server_id) {
            return Err(TransferError::ServerNotFound(server_id.to_string()));
        }

        let agreement_id = format!("a-{}", &uuid::Uuid::new_v4().simple().to_string()[..17]);
        let arn = format!("arn:aws:transfer:{region}:{account_id}:agreement/{agreement_id}");

        let agreement = Agreement {
            agreement_id: agreement_id.clone(),
            arn,
            server_id: server_id.to_string(),
            local_profile_id: local_profile_id.to_string(),
            partner_profile_id: partner_profile_id.to_string(),
            base_directory: base_directory.to_string(),
            access_role: access_role.to_string(),
            status: "ACTIVE".to_string(),
            description: description.map(|s| s.to_string()),
            tags,
        };

        self.agreements.insert(agreement_id.clone(), agreement);
        Ok(agreement_id)
    }

    pub fn describe_agreement(&self, agreement_id: &str) -> Result<&Agreement, TransferError> {
        self.agreements
            .get(agreement_id)
            .ok_or_else(|| TransferError::AgreementNotFound(agreement_id.to_string()))
    }

    pub fn list_agreements(&self, server_id: &str) -> Result<Vec<&Agreement>, TransferError> {
        if !self.servers.contains_key(server_id) {
            return Err(TransferError::ServerNotFound(server_id.to_string()));
        }
        Ok(self
            .agreements
            .values()
            .filter(|a| a.server_id == server_id)
            .collect())
    }

    pub fn update_agreement(
        &mut self,
        agreement_id: &str,
        base_directory: Option<&str>,
        access_role: Option<&str>,
        status: Option<&str>,
        local_profile_id: Option<&str>,
        partner_profile_id: Option<&str>,
        description: Option<&str>,
    ) -> Result<String, TransferError> {
        let agreement = self
            .agreements
            .get_mut(agreement_id)
            .ok_or_else(|| TransferError::AgreementNotFound(agreement_id.to_string()))?;

        if let Some(bd) = base_directory {
            agreement.base_directory = bd.to_string();
        }
        if let Some(ar) = access_role {
            agreement.access_role = ar.to_string();
        }
        if let Some(s) = status {
            agreement.status = s.to_string();
        }
        if let Some(lp) = local_profile_id {
            agreement.local_profile_id = lp.to_string();
        }
        if let Some(pp) = partner_profile_id {
            agreement.partner_profile_id = pp.to_string();
        }
        if let Some(d) = description {
            agreement.description = Some(d.to_string());
        }

        Ok(agreement_id.to_string())
    }

    pub fn delete_agreement(&mut self, agreement_id: &str) -> Result<(), TransferError> {
        if self.agreements.remove(agreement_id).is_none() {
            return Err(TransferError::AgreementNotFound(agreement_id.to_string()));
        }
        Ok(())
    }

    // --- Certificates ---

    pub fn import_certificate(
        &mut self,
        account_id: &str,
        region: &str,
        usage: &str,
        certificate: &str,
        certificate_chain: Option<&str>,
        private_key: Option<&str>,
        active_date: Option<f64>,
        inactive_date: Option<f64>,
        description: Option<&str>,
        tags: Vec<Tag>,
    ) -> Result<String, TransferError> {
        let cert_id = format!("cert-{}", &uuid::Uuid::new_v4().simple().to_string()[..13]);
        let arn = format!("arn:aws:transfer:{region}:{account_id}:certificate/{cert_id}");

        let cert_type = if private_key.is_some() {
            "CERTIFICATE_WITH_PRIVATE_KEY"
        } else {
            "CERTIFICATE"
        };

        let cert = Certificate {
            certificate_id: cert_id.clone(),
            arn,
            usage: usage.to_string(),
            status: "ACTIVE".to_string(),
            certificate: certificate.to_string(),
            certificate_chain: certificate_chain.map(|s| s.to_string()),
            private_key: private_key.map(|s| s.to_string()),
            active_date,
            inactive_date,
            description: description.map(|s| s.to_string()),
            certificate_type: cert_type.to_string(),
            tags,
        };

        self.certificates.insert(cert_id.clone(), cert);
        Ok(cert_id)
    }

    pub fn describe_certificate(
        &self,
        certificate_id: &str,
    ) -> Result<&Certificate, TransferError> {
        self.certificates
            .get(certificate_id)
            .ok_or_else(|| TransferError::CertificateNotFound(certificate_id.to_string()))
    }

    pub fn list_certificates(&self) -> Vec<&Certificate> {
        self.certificates.values().collect()
    }

    pub fn update_certificate(
        &mut self,
        certificate_id: &str,
        active_date: Option<f64>,
        inactive_date: Option<f64>,
        description: Option<&str>,
    ) -> Result<String, TransferError> {
        let cert = self
            .certificates
            .get_mut(certificate_id)
            .ok_or_else(|| TransferError::CertificateNotFound(certificate_id.to_string()))?;

        if let Some(ad) = active_date {
            cert.active_date = Some(ad);
        }
        if let Some(id) = inactive_date {
            cert.inactive_date = Some(id);
        }
        if let Some(d) = description {
            cert.description = Some(d.to_string());
        }

        Ok(certificate_id.to_string())
    }

    pub fn delete_certificate(&mut self, certificate_id: &str) -> Result<(), TransferError> {
        if self.certificates.remove(certificate_id).is_none() {
            return Err(TransferError::CertificateNotFound(
                certificate_id.to_string(),
            ));
        }
        Ok(())
    }

    // --- Connectors ---

    pub fn create_connector(
        &mut self,
        account_id: &str,
        region: &str,
        url: Option<&str>,
        as2_config: Option<serde_json::Value>,
        sftp_config: Option<serde_json::Value>,
        access_role: &str,
        logging_role: Option<&str>,
        tags: Vec<Tag>,
    ) -> Result<String, TransferError> {
        let connector_id = format!("c-{}", &uuid::Uuid::new_v4().simple().to_string()[..17]);
        let arn = format!("arn:aws:transfer:{region}:{account_id}:connector/{connector_id}");

        let connector = Connector {
            connector_id: connector_id.clone(),
            arn,
            url: url.map(|s| s.to_string()),
            as2_config,
            sftp_config,
            access_role: access_role.to_string(),
            logging_role: logging_role.map(|s| s.to_string()),
            // FIX(terraform-e2e): provider waits for connector status "ACTIVE" but we
            //   were returning "ONLINE"; use "ACTIVE" so the waiter resolves immediately.
            status: "ACTIVE".to_string(),
            tags,
        };

        self.connectors.insert(connector_id.clone(), connector);
        Ok(connector_id)
    }

    pub fn describe_connector(&self, connector_id: &str) -> Result<&Connector, TransferError> {
        self.connectors
            .get(connector_id)
            .ok_or_else(|| TransferError::ConnectorNotFound(connector_id.to_string()))
    }

    pub fn list_connectors(&self) -> Vec<&Connector> {
        self.connectors.values().collect()
    }

    pub fn update_connector(
        &mut self,
        connector_id: &str,
        url: Option<&str>,
        as2_config: Option<serde_json::Value>,
        sftp_config: Option<serde_json::Value>,
        access_role: Option<&str>,
        logging_role: Option<&str>,
    ) -> Result<String, TransferError> {
        let connector = self
            .connectors
            .get_mut(connector_id)
            .ok_or_else(|| TransferError::ConnectorNotFound(connector_id.to_string()))?;

        if let Some(u) = url {
            connector.url = Some(u.to_string());
        }
        if let Some(c) = as2_config {
            connector.as2_config = Some(c);
        }
        if let Some(c) = sftp_config {
            connector.sftp_config = Some(c);
        }
        if let Some(ar) = access_role {
            connector.access_role = ar.to_string();
        }
        if let Some(lr) = logging_role {
            connector.logging_role = Some(lr.to_string());
        }

        Ok(connector_id.to_string())
    }

    pub fn delete_connector(&mut self, connector_id: &str) -> Result<(), TransferError> {
        if self.connectors.remove(connector_id).is_none() {
            return Err(TransferError::ConnectorNotFound(connector_id.to_string()));
        }
        Ok(())
    }

    // --- Profiles ---

    pub fn create_profile(
        &mut self,
        account_id: &str,
        region: &str,
        profile_type: &str,
        as2_id: &str,
        certificate_ids: Vec<String>,
        tags: Vec<Tag>,
    ) -> Result<String, TransferError> {
        let profile_id = format!("p-{}", &uuid::Uuid::new_v4().simple().to_string()[..17]);
        let arn = format!("arn:aws:transfer:{region}:{account_id}:profile/{profile_id}");

        let profile = Profile {
            profile_id: profile_id.clone(),
            arn,
            profile_type: profile_type.to_string(),
            as2_id: as2_id.to_string(),
            certificate_ids,
            tags,
        };

        self.profiles.insert(profile_id.clone(), profile);
        Ok(profile_id)
    }

    pub fn describe_profile(&self, profile_id: &str) -> Result<&Profile, TransferError> {
        self.profiles
            .get(profile_id)
            .ok_or_else(|| TransferError::ProfileNotFound(profile_id.to_string()))
    }

    pub fn list_profiles(&self, profile_type: Option<&str>) -> Vec<&Profile> {
        self.profiles
            .values()
            .filter(|p| {
                if let Some(pt) = profile_type {
                    p.profile_type == pt
                } else {
                    true
                }
            })
            .collect()
    }

    pub fn update_profile(
        &mut self,
        profile_id: &str,
        certificate_ids: Option<Vec<String>>,
    ) -> Result<String, TransferError> {
        let profile = self
            .profiles
            .get_mut(profile_id)
            .ok_or_else(|| TransferError::ProfileNotFound(profile_id.to_string()))?;

        if let Some(ids) = certificate_ids {
            profile.certificate_ids = ids;
        }

        Ok(profile_id.to_string())
    }

    pub fn delete_profile(&mut self, profile_id: &str) -> Result<(), TransferError> {
        if self.profiles.remove(profile_id).is_none() {
            return Err(TransferError::ProfileNotFound(profile_id.to_string()));
        }
        Ok(())
    }

    // --- WebApps ---

    pub fn create_web_app(
        &mut self,
        account_id: &str,
        region: &str,
        tags: Vec<Tag>,
    ) -> Result<String, TransferError> {
        let web_app_id = format!(
            "webapp-{}",
            &uuid::Uuid::new_v4().simple().to_string()[..13]
        );
        let arn = format!("arn:aws:transfer:{region}:{account_id}:webapp/{web_app_id}");
        let web_app_endpoint = format!("{web_app_id}.webapp.transfer.{region}.amazonaws.com");

        let web_app = WebApp {
            web_app_id: web_app_id.clone(),
            arn,
            web_app_endpoint: Some(web_app_endpoint),
            tags,
            customization: None,
        };

        self.web_apps.insert(web_app_id.clone(), web_app);
        Ok(web_app_id)
    }

    pub fn describe_web_app(&self, web_app_id: &str) -> Result<&WebApp, TransferError> {
        self.web_apps
            .get(web_app_id)
            .ok_or_else(|| TransferError::WebAppNotFound(web_app_id.to_string()))
    }

    pub fn list_web_apps(&self) -> Vec<&WebApp> {
        self.web_apps.values().collect()
    }

    pub fn update_web_app(&mut self, web_app_id: &str) -> Result<String, TransferError> {
        if !self.web_apps.contains_key(web_app_id) {
            return Err(TransferError::WebAppNotFound(web_app_id.to_string()));
        }
        Ok(web_app_id.to_string())
    }

    pub fn delete_web_app(&mut self, web_app_id: &str) -> Result<(), TransferError> {
        if self.web_apps.remove(web_app_id).is_none() {
            return Err(TransferError::WebAppNotFound(web_app_id.to_string()));
        }
        Ok(())
    }

    pub fn describe_web_app_customization(
        &self,
        web_app_id: &str,
    ) -> Result<(&WebApp, &Option<WebAppCustomization>), TransferError> {
        let web_app = self
            .web_apps
            .get(web_app_id)
            .ok_or_else(|| TransferError::WebAppNotFound(web_app_id.to_string()))?;
        Ok((web_app, &web_app.customization))
    }

    pub fn update_web_app_customization(
        &mut self,
        web_app_id: &str,
        title: Option<&str>,
        logo_file: Option<&str>,
        favicon_file: Option<&str>,
    ) -> Result<String, TransferError> {
        let web_app = self
            .web_apps
            .get_mut(web_app_id)
            .ok_or_else(|| TransferError::WebAppNotFound(web_app_id.to_string()))?;

        let cust = web_app.customization.get_or_insert(WebAppCustomization {
            title: None,
            logo_file: None,
            favicon_file: None,
        });

        if let Some(t) = title {
            cust.title = Some(t.to_string());
        }
        if let Some(l) = logo_file {
            cust.logo_file = Some(l.to_string());
        }
        if let Some(f) = favicon_file {
            cust.favicon_file = Some(f.to_string());
        }

        Ok(web_app_id.to_string())
    }

    pub fn delete_web_app_customization(&mut self, web_app_id: &str) -> Result<(), TransferError> {
        let web_app = self
            .web_apps
            .get_mut(web_app_id)
            .ok_or_else(|| TransferError::WebAppNotFound(web_app_id.to_string()))?;
        web_app.customization = None;
        Ok(())
    }

    // --- Workflows ---

    pub fn create_workflow(
        &mut self,
        account_id: &str,
        region: &str,
        steps: Vec<serde_json::Value>,
        on_exception_steps: Vec<serde_json::Value>,
        description: Option<&str>,
        tags: Vec<Tag>,
    ) -> Result<String, TransferError> {
        let workflow_id = format!("w-{}", &uuid::Uuid::new_v4().simple().to_string()[..17]);
        let arn = format!("arn:aws:transfer:{region}:{account_id}:workflow/{workflow_id}");

        let workflow = Workflow {
            workflow_id: workflow_id.clone(),
            arn,
            steps,
            on_exception_steps,
            description: description.map(|s| s.to_string()),
            tags,
        };

        self.workflows.insert(workflow_id.clone(), workflow);
        Ok(workflow_id)
    }

    pub fn describe_workflow(&self, workflow_id: &str) -> Result<&Workflow, TransferError> {
        self.workflows
            .get(workflow_id)
            .ok_or_else(|| TransferError::WorkflowNotFound(workflow_id.to_string()))
    }

    pub fn list_workflows(&self) -> Vec<&Workflow> {
        self.workflows.values().collect()
    }

    pub fn delete_workflow(&mut self, workflow_id: &str) -> Result<(), TransferError> {
        if self.workflows.remove(workflow_id).is_none() {
            return Err(TransferError::WorkflowNotFound(workflow_id.to_string()));
        }
        Ok(())
    }
}
