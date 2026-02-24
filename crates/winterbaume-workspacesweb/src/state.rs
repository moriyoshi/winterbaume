use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct WorkspacesWebState {
    pub portals: HashMap<String, Portal>,
    pub browser_settings: HashMap<String, BrowserSettings>,
    pub network_settings: HashMap<String, NetworkSettings>,
    pub user_access_logging_settings: HashMap<String, UserAccessLoggingSettings>,
    pub user_settings: HashMap<String, UserSettings>,
    pub identity_providers: HashMap<String, IdentityProvider>,
    pub ip_access_settings: HashMap<String, IpAccessSettings>,
    pub trust_stores: HashMap<String, TrustStore>,
    pub data_protection_settings: HashMap<String, DataProtectionSettings>,
    pub session_loggers: HashMap<String, SessionLogger>,
    pub sessions: HashMap<String, Session>,
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspacesWebError {
    #[error("Resource {0} not found.")]
    ResourceNotFound(String),
    #[error("Session {0} not found.")]
    SessionNotFound(String),
}

impl WorkspacesWebState {
    // ── Portal ──────────────────────────────────────────────────────

    pub fn create_portal(
        &mut self,
        display_name: Option<&str>,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&Portal, WorkspacesWebError> {
        let portal_id = uuid::Uuid::new_v4().to_string();
        let portal_arn = format!("arn:aws:workspaces-web:{region}:{account_id}:portal/{portal_id}");
        let portal_endpoint = format!("https://{portal_id}.workspaces-web.{region}.amazonaws.com");

        let name = display_name.unwrap_or("").to_string();

        let portal = Portal {
            portal_arn: portal_arn.clone(),
            portal_endpoint,
            display_name: name,
            portal_status: "Active".to_string(),
            browser_type: "Chrome".to_string(),
            renderer_type: "AppStream".to_string(),
            creation_date: Utc::now(),
            browser_settings_arn: None,
            network_settings_arn: None,
            user_access_logging_settings_arn: None,
            user_settings_arn: None,
            trust_store_arn: None,
            ip_access_settings_arn: None,
            data_protection_settings_arn: None,
            session_logger_arn: None,
            tags,
        };

        self.portals.insert(portal_arn.clone(), portal);
        Ok(self.portals.get(&portal_arn).unwrap())
    }

    pub fn get_portal(&self, portal_arn: &str) -> Result<&Portal, WorkspacesWebError> {
        self.portals
            .get(portal_arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(portal_arn.to_string()))
    }

    pub fn delete_portal(&mut self, portal_arn: &str) -> Result<(), WorkspacesWebError> {
        if self.portals.remove(portal_arn).is_none() {
            return Err(WorkspacesWebError::ResourceNotFound(portal_arn.to_string()));
        }
        Ok(())
    }

    pub fn list_portals(&self) -> Vec<&Portal> {
        self.portals.values().collect()
    }

    // ── BrowserSettings ─────────────────────────────────────────────

    pub fn create_browser_settings(
        &mut self,
        browser_policy: Option<&str>,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&BrowserSettings, WorkspacesWebError> {
        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:workspaces-web:{region}:{account_id}:browserSettings/{id}");

        let bs = BrowserSettings {
            browser_settings_arn: arn.clone(),
            browser_policy: browser_policy.map(|s| s.to_string()),
            associated_portal_arns: Vec::new(),
            tags,
        };

        self.browser_settings.insert(arn.clone(), bs);
        Ok(self.browser_settings.get(&arn).unwrap())
    }

    pub fn get_browser_settings(&self, arn: &str) -> Result<&BrowserSettings, WorkspacesWebError> {
        self.browser_settings
            .get(arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(arn.to_string()))
    }

    pub fn delete_browser_settings(&mut self, arn: &str) -> Result<(), WorkspacesWebError> {
        if self.browser_settings.remove(arn).is_none() {
            return Err(WorkspacesWebError::ResourceNotFound(arn.to_string()));
        }
        Ok(())
    }

    pub fn list_browser_settings(&self) -> Vec<&BrowserSettings> {
        self.browser_settings.values().collect()
    }

    // ── NetworkSettings ─────────────────────────────────────────────

    pub fn create_network_settings(
        &mut self,
        vpc_id: &str,
        subnet_ids: Vec<String>,
        security_group_ids: Vec<String>,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&NetworkSettings, WorkspacesWebError> {
        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:workspaces-web:{region}:{account_id}:networkSettings/{id}");

        let ns = NetworkSettings {
            network_settings_arn: arn.clone(),
            vpc_id: vpc_id.to_string(),
            subnet_ids,
            security_group_ids,
            associated_portal_arns: Vec::new(),
            tags,
        };

        self.network_settings.insert(arn.clone(), ns);
        Ok(self.network_settings.get(&arn).unwrap())
    }

    pub fn get_network_settings(&self, arn: &str) -> Result<&NetworkSettings, WorkspacesWebError> {
        self.network_settings
            .get(arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(arn.to_string()))
    }

    pub fn delete_network_settings(&mut self, arn: &str) -> Result<(), WorkspacesWebError> {
        if self.network_settings.remove(arn).is_none() {
            return Err(WorkspacesWebError::ResourceNotFound(arn.to_string()));
        }
        Ok(())
    }

    pub fn list_network_settings(&self) -> Vec<&NetworkSettings> {
        self.network_settings.values().collect()
    }

    // ── UserAccessLoggingSettings ───────────────────────────────────

    pub fn create_user_access_logging_settings(
        &mut self,
        kinesis_stream_arn: &str,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&UserAccessLoggingSettings, WorkspacesWebError> {
        let id = uuid::Uuid::new_v4().to_string();
        let arn =
            format!("arn:aws:workspaces-web:{region}:{account_id}:userAccessLoggingSettings/{id}");

        let ual = UserAccessLoggingSettings {
            user_access_logging_settings_arn: arn.clone(),
            kinesis_stream_arn: kinesis_stream_arn.to_string(),
            associated_portal_arns: Vec::new(),
            tags,
        };

        self.user_access_logging_settings.insert(arn.clone(), ual);
        Ok(self.user_access_logging_settings.get(&arn).unwrap())
    }

    pub fn get_user_access_logging_settings(
        &self,
        arn: &str,
    ) -> Result<&UserAccessLoggingSettings, WorkspacesWebError> {
        self.user_access_logging_settings
            .get(arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(arn.to_string()))
    }

    pub fn delete_user_access_logging_settings(
        &mut self,
        arn: &str,
    ) -> Result<(), WorkspacesWebError> {
        if self.user_access_logging_settings.remove(arn).is_none() {
            return Err(WorkspacesWebError::ResourceNotFound(arn.to_string()));
        }
        Ok(())
    }

    pub fn list_user_access_logging_settings(&self) -> Vec<&UserAccessLoggingSettings> {
        self.user_access_logging_settings.values().collect()
    }

    // ── UserSettings ────────────────────────────────────────────────

    pub fn create_user_settings(
        &mut self,
        copy_allowed: &str,
        paste_allowed: &str,
        download_allowed: &str,
        upload_allowed: &str,
        print_allowed: &str,
        disconnect_timeout_in_minutes: Option<i32>,
        idle_disconnect_timeout_in_minutes: Option<i32>,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&UserSettings, WorkspacesWebError> {
        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:workspaces-web:{region}:{account_id}:userSettings/{id}");

        let us = UserSettings {
            user_settings_arn: arn.clone(),
            copy_allowed: copy_allowed.to_string(),
            paste_allowed: paste_allowed.to_string(),
            download_allowed: download_allowed.to_string(),
            upload_allowed: upload_allowed.to_string(),
            print_allowed: print_allowed.to_string(),
            disconnect_timeout_in_minutes,
            idle_disconnect_timeout_in_minutes,
            associated_portal_arns: Vec::new(),
            tags,
        };

        self.user_settings.insert(arn.clone(), us);
        Ok(self.user_settings.get(&arn).unwrap())
    }

    pub fn get_user_settings(&self, arn: &str) -> Result<&UserSettings, WorkspacesWebError> {
        self.user_settings
            .get(arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(arn.to_string()))
    }

    pub fn delete_user_settings(&mut self, arn: &str) -> Result<(), WorkspacesWebError> {
        if self.user_settings.remove(arn).is_none() {
            return Err(WorkspacesWebError::ResourceNotFound(arn.to_string()));
        }
        Ok(())
    }

    pub fn list_user_settings(&self) -> Vec<&UserSettings> {
        self.user_settings.values().collect()
    }

    // ── Association helpers ─────────────────────────────────────────

    pub fn associate_browser_settings(
        &mut self,
        portal_arn: &str,
        browser_settings_arn: &str,
    ) -> Result<(), WorkspacesWebError> {
        // Validate both exist
        if !self.portals.contains_key(portal_arn) {
            return Err(WorkspacesWebError::ResourceNotFound(portal_arn.to_string()));
        }
        if !self.browser_settings.contains_key(browser_settings_arn) {
            return Err(WorkspacesWebError::ResourceNotFound(
                browser_settings_arn.to_string(),
            ));
        }
        self.portals
            .get_mut(portal_arn)
            .unwrap()
            .browser_settings_arn = Some(browser_settings_arn.to_string());
        let bs = self.browser_settings.get_mut(browser_settings_arn).unwrap();
        if !bs.associated_portal_arns.contains(&portal_arn.to_string()) {
            bs.associated_portal_arns.push(portal_arn.to_string());
        }
        Ok(())
    }

    pub fn associate_network_settings(
        &mut self,
        portal_arn: &str,
        network_settings_arn: &str,
    ) -> Result<(), WorkspacesWebError> {
        if !self.portals.contains_key(portal_arn) {
            return Err(WorkspacesWebError::ResourceNotFound(portal_arn.to_string()));
        }
        if !self.network_settings.contains_key(network_settings_arn) {
            return Err(WorkspacesWebError::ResourceNotFound(
                network_settings_arn.to_string(),
            ));
        }
        self.portals
            .get_mut(portal_arn)
            .unwrap()
            .network_settings_arn = Some(network_settings_arn.to_string());
        let ns = self.network_settings.get_mut(network_settings_arn).unwrap();
        if !ns.associated_portal_arns.contains(&portal_arn.to_string()) {
            ns.associated_portal_arns.push(portal_arn.to_string());
        }
        Ok(())
    }

    pub fn associate_user_access_logging_settings(
        &mut self,
        portal_arn: &str,
        user_access_logging_settings_arn: &str,
    ) -> Result<(), WorkspacesWebError> {
        if !self.portals.contains_key(portal_arn) {
            return Err(WorkspacesWebError::ResourceNotFound(portal_arn.to_string()));
        }
        if !self
            .user_access_logging_settings
            .contains_key(user_access_logging_settings_arn)
        {
            return Err(WorkspacesWebError::ResourceNotFound(
                user_access_logging_settings_arn.to_string(),
            ));
        }
        self.portals
            .get_mut(portal_arn)
            .unwrap()
            .user_access_logging_settings_arn = Some(user_access_logging_settings_arn.to_string());
        let ual = self
            .user_access_logging_settings
            .get_mut(user_access_logging_settings_arn)
            .unwrap();
        if !ual.associated_portal_arns.contains(&portal_arn.to_string()) {
            ual.associated_portal_arns.push(portal_arn.to_string());
        }
        Ok(())
    }

    pub fn associate_user_settings(
        &mut self,
        portal_arn: &str,
        user_settings_arn: &str,
    ) -> Result<(), WorkspacesWebError> {
        if !self.portals.contains_key(portal_arn) {
            return Err(WorkspacesWebError::ResourceNotFound(portal_arn.to_string()));
        }
        if !self.user_settings.contains_key(user_settings_arn) {
            return Err(WorkspacesWebError::ResourceNotFound(
                user_settings_arn.to_string(),
            ));
        }
        self.portals.get_mut(portal_arn).unwrap().user_settings_arn =
            Some(user_settings_arn.to_string());
        let us = self.user_settings.get_mut(user_settings_arn).unwrap();
        if !us.associated_portal_arns.contains(&portal_arn.to_string()) {
            us.associated_portal_arns.push(portal_arn.to_string());
        }
        Ok(())
    }

    // ── Portal update ────────────────────────────────────────────────

    pub fn update_portal(
        &mut self,
        portal_arn: &str,
        display_name: Option<&str>,
    ) -> Result<&Portal, WorkspacesWebError> {
        let portal = self
            .portals
            .get_mut(portal_arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(portal_arn.to_string()))?;
        if let Some(name) = display_name {
            portal.display_name = name.to_string();
        }
        Ok(self.portals.get(portal_arn).unwrap())
    }

    // ── IdentityProvider ─────────────────────────────────────────────

    pub fn create_identity_provider(
        &mut self,
        portal_arn: &str,
        name: &str,
        provider_type: &str,
        details: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&IdentityProvider, WorkspacesWebError> {
        if !self.portals.contains_key(portal_arn) {
            return Err(WorkspacesWebError::ResourceNotFound(portal_arn.to_string()));
        }
        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:workspaces-web:{region}:{account_id}:identityProvider/{id}");
        let ip = IdentityProvider {
            identity_provider_arn: arn.clone(),
            portal_arn: portal_arn.to_string(),
            identity_provider_name: name.to_string(),
            identity_provider_type: provider_type.to_string(),
            identity_provider_details: details,
        };
        self.identity_providers.insert(arn.clone(), ip);
        Ok(self.identity_providers.get(&arn).unwrap())
    }

    pub fn get_identity_provider(
        &self,
        arn: &str,
    ) -> Result<&IdentityProvider, WorkspacesWebError> {
        self.identity_providers
            .get(arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(arn.to_string()))
    }

    pub fn delete_identity_provider(&mut self, arn: &str) -> Result<(), WorkspacesWebError> {
        if self.identity_providers.remove(arn).is_none() {
            return Err(WorkspacesWebError::ResourceNotFound(arn.to_string()));
        }
        Ok(())
    }

    pub fn list_identity_providers(&self, portal_arn: &str) -> Vec<&IdentityProvider> {
        self.identity_providers
            .values()
            .filter(|ip| ip.portal_arn == portal_arn)
            .collect()
    }

    pub fn update_identity_provider(
        &mut self,
        arn: &str,
        name: Option<&str>,
        provider_type: Option<&str>,
        details: Option<HashMap<String, String>>,
    ) -> Result<&IdentityProvider, WorkspacesWebError> {
        let ip = self
            .identity_providers
            .get_mut(arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(arn.to_string()))?;
        if let Some(n) = name {
            ip.identity_provider_name = n.to_string();
        }
        if let Some(t) = provider_type {
            ip.identity_provider_type = t.to_string();
        }
        if let Some(d) = details {
            ip.identity_provider_details = d;
        }
        Ok(self.identity_providers.get(arn).unwrap())
    }

    // ── IpAccessSettings ─────────────────────────────────────────────

    pub fn create_ip_access_settings(
        &mut self,
        ip_rules: Vec<IpRule>,
        display_name: Option<String>,
        description: Option<String>,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&IpAccessSettings, WorkspacesWebError> {
        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:workspaces-web:{region}:{account_id}:ipAccessSettings/{id}");
        let ias = IpAccessSettings {
            ip_access_settings_arn: arn.clone(),
            display_name,
            description,
            ip_rules,
            associated_portal_arns: Vec::new(),
            creation_date: Utc::now(),
            tags,
        };
        self.ip_access_settings.insert(arn.clone(), ias);
        Ok(self.ip_access_settings.get(&arn).unwrap())
    }

    pub fn get_ip_access_settings(
        &self,
        arn: &str,
    ) -> Result<&IpAccessSettings, WorkspacesWebError> {
        self.ip_access_settings
            .get(arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(arn.to_string()))
    }

    pub fn delete_ip_access_settings(&mut self, arn: &str) -> Result<(), WorkspacesWebError> {
        if self.ip_access_settings.remove(arn).is_none() {
            return Err(WorkspacesWebError::ResourceNotFound(arn.to_string()));
        }
        Ok(())
    }

    pub fn list_ip_access_settings(&self) -> Vec<&IpAccessSettings> {
        self.ip_access_settings.values().collect()
    }

    pub fn update_ip_access_settings(
        &mut self,
        arn: &str,
        ip_rules: Option<Vec<IpRule>>,
        display_name: Option<String>,
        description: Option<String>,
    ) -> Result<&IpAccessSettings, WorkspacesWebError> {
        let ias = self
            .ip_access_settings
            .get_mut(arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(arn.to_string()))?;
        if let Some(rules) = ip_rules {
            ias.ip_rules = rules;
        }
        if display_name.is_some() {
            ias.display_name = display_name;
        }
        if description.is_some() {
            ias.description = description;
        }
        Ok(self.ip_access_settings.get(arn).unwrap())
    }

    pub fn associate_ip_access_settings(
        &mut self,
        portal_arn: &str,
        ip_access_settings_arn: &str,
    ) -> Result<(), WorkspacesWebError> {
        if !self.portals.contains_key(portal_arn) {
            return Err(WorkspacesWebError::ResourceNotFound(portal_arn.to_string()));
        }
        if !self.ip_access_settings.contains_key(ip_access_settings_arn) {
            return Err(WorkspacesWebError::ResourceNotFound(
                ip_access_settings_arn.to_string(),
            ));
        }
        self.portals
            .get_mut(portal_arn)
            .unwrap()
            .ip_access_settings_arn = Some(ip_access_settings_arn.to_string());
        let ias = self
            .ip_access_settings
            .get_mut(ip_access_settings_arn)
            .unwrap();
        if !ias.associated_portal_arns.contains(&portal_arn.to_string()) {
            ias.associated_portal_arns.push(portal_arn.to_string());
        }
        Ok(())
    }

    pub fn disassociate_ip_access_settings(
        &mut self,
        portal_arn: &str,
    ) -> Result<(), WorkspacesWebError> {
        let portal = self
            .portals
            .get_mut(portal_arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(portal_arn.to_string()))?;
        portal.ip_access_settings_arn = None;
        Ok(())
    }

    // ── TrustStore ───────────────────────────────────────────────────

    pub fn create_trust_store(
        &mut self,
        certificate_list: Vec<String>,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&TrustStore, WorkspacesWebError> {
        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:workspaces-web:{region}:{account_id}:trustStore/{id}");
        let certs: Vec<TrustStoreCertificate> = certificate_list
            .into_iter()
            .enumerate()
            .map(|(i, body)| TrustStoreCertificate {
                thumbprint: format!("thumb{i:04}"),
                body,
                issuer: None,
                subject: None,
            })
            .collect();
        let ts = TrustStore {
            trust_store_arn: arn.clone(),
            certificate_list: certs,
            associated_portal_arns: Vec::new(),
            tags,
        };
        self.trust_stores.insert(arn.clone(), ts);
        Ok(self.trust_stores.get(&arn).unwrap())
    }

    pub fn get_trust_store(&self, arn: &str) -> Result<&TrustStore, WorkspacesWebError> {
        self.trust_stores
            .get(arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(arn.to_string()))
    }

    pub fn delete_trust_store(&mut self, arn: &str) -> Result<(), WorkspacesWebError> {
        if self.trust_stores.remove(arn).is_none() {
            return Err(WorkspacesWebError::ResourceNotFound(arn.to_string()));
        }
        Ok(())
    }

    pub fn list_trust_stores(&self) -> Vec<&TrustStore> {
        self.trust_stores.values().collect()
    }

    pub fn update_trust_store(
        &mut self,
        arn: &str,
        certs_to_add: Vec<String>,
        thumbprints_to_delete: Vec<String>,
    ) -> Result<&TrustStore, WorkspacesWebError> {
        let ts = self
            .trust_stores
            .get_mut(arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(arn.to_string()))?;
        ts.certificate_list
            .retain(|c| !thumbprints_to_delete.contains(&c.thumbprint));
        let existing_count = ts.certificate_list.len();
        for (i, body) in certs_to_add.into_iter().enumerate() {
            ts.certificate_list.push(TrustStoreCertificate {
                thumbprint: format!("thumb{:04}", existing_count + i),
                body,
                issuer: None,
                subject: None,
            });
        }
        Ok(self.trust_stores.get(arn).unwrap())
    }

    pub fn associate_trust_store(
        &mut self,
        portal_arn: &str,
        trust_store_arn: &str,
    ) -> Result<(), WorkspacesWebError> {
        if !self.portals.contains_key(portal_arn) {
            return Err(WorkspacesWebError::ResourceNotFound(portal_arn.to_string()));
        }
        if !self.trust_stores.contains_key(trust_store_arn) {
            return Err(WorkspacesWebError::ResourceNotFound(
                trust_store_arn.to_string(),
            ));
        }
        self.portals.get_mut(portal_arn).unwrap().trust_store_arn =
            Some(trust_store_arn.to_string());
        let ts = self.trust_stores.get_mut(trust_store_arn).unwrap();
        if !ts.associated_portal_arns.contains(&portal_arn.to_string()) {
            ts.associated_portal_arns.push(portal_arn.to_string());
        }
        Ok(())
    }

    pub fn disassociate_trust_store(&mut self, portal_arn: &str) -> Result<(), WorkspacesWebError> {
        let portal = self
            .portals
            .get_mut(portal_arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(portal_arn.to_string()))?;
        portal.trust_store_arn = None;
        Ok(())
    }

    // ── DataProtectionSettings ───────────────────────────────────────

    pub fn create_data_protection_settings(
        &mut self,
        display_name: Option<String>,
        description: Option<String>,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&DataProtectionSettings, WorkspacesWebError> {
        let id = uuid::Uuid::new_v4().to_string();
        let arn =
            format!("arn:aws:workspaces-web:{region}:{account_id}:dataProtectionSettings/{id}");
        let dps = DataProtectionSettings {
            data_protection_settings_arn: arn.clone(),
            display_name,
            description,
            associated_portal_arns: Vec::new(),
            creation_date: Utc::now(),
            tags,
        };
        self.data_protection_settings.insert(arn.clone(), dps);
        Ok(self.data_protection_settings.get(&arn).unwrap())
    }

    pub fn get_data_protection_settings(
        &self,
        arn: &str,
    ) -> Result<&DataProtectionSettings, WorkspacesWebError> {
        self.data_protection_settings
            .get(arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(arn.to_string()))
    }

    pub fn delete_data_protection_settings(&mut self, arn: &str) -> Result<(), WorkspacesWebError> {
        if self.data_protection_settings.remove(arn).is_none() {
            return Err(WorkspacesWebError::ResourceNotFound(arn.to_string()));
        }
        Ok(())
    }

    pub fn list_data_protection_settings(&self) -> Vec<&DataProtectionSettings> {
        self.data_protection_settings.values().collect()
    }

    pub fn update_data_protection_settings(
        &mut self,
        arn: &str,
        display_name: Option<String>,
        description: Option<String>,
    ) -> Result<&DataProtectionSettings, WorkspacesWebError> {
        let dps = self
            .data_protection_settings
            .get_mut(arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(arn.to_string()))?;
        if display_name.is_some() {
            dps.display_name = display_name;
        }
        if description.is_some() {
            dps.description = description;
        }
        Ok(self.data_protection_settings.get(arn).unwrap())
    }

    pub fn associate_data_protection_settings(
        &mut self,
        portal_arn: &str,
        data_protection_settings_arn: &str,
    ) -> Result<(), WorkspacesWebError> {
        if !self.portals.contains_key(portal_arn) {
            return Err(WorkspacesWebError::ResourceNotFound(portal_arn.to_string()));
        }
        if !self
            .data_protection_settings
            .contains_key(data_protection_settings_arn)
        {
            return Err(WorkspacesWebError::ResourceNotFound(
                data_protection_settings_arn.to_string(),
            ));
        }
        self.portals
            .get_mut(portal_arn)
            .unwrap()
            .data_protection_settings_arn = Some(data_protection_settings_arn.to_string());
        let dps = self
            .data_protection_settings
            .get_mut(data_protection_settings_arn)
            .unwrap();
        if !dps.associated_portal_arns.contains(&portal_arn.to_string()) {
            dps.associated_portal_arns.push(portal_arn.to_string());
        }
        Ok(())
    }

    pub fn disassociate_data_protection_settings(
        &mut self,
        portal_arn: &str,
    ) -> Result<(), WorkspacesWebError> {
        let portal = self
            .portals
            .get_mut(portal_arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(portal_arn.to_string()))?;
        portal.data_protection_settings_arn = None;
        Ok(())
    }

    // ── SessionLogger ────────────────────────────────────────────────

    pub fn create_session_logger(
        &mut self,
        display_name: Option<String>,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&SessionLogger, WorkspacesWebError> {
        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:workspaces-web:{region}:{account_id}:sessionLogger/{id}");
        let sl = SessionLogger {
            session_logger_arn: arn.clone(),
            display_name,
            associated_portal_arns: Vec::new(),
            creation_date: Utc::now(),
            tags,
        };
        self.session_loggers.insert(arn.clone(), sl);
        Ok(self.session_loggers.get(&arn).unwrap())
    }

    pub fn get_session_logger(&self, arn: &str) -> Result<&SessionLogger, WorkspacesWebError> {
        self.session_loggers
            .get(arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(arn.to_string()))
    }

    pub fn delete_session_logger(&mut self, arn: &str) -> Result<(), WorkspacesWebError> {
        if self.session_loggers.remove(arn).is_none() {
            return Err(WorkspacesWebError::ResourceNotFound(arn.to_string()));
        }
        Ok(())
    }

    pub fn list_session_loggers(&self) -> Vec<&SessionLogger> {
        self.session_loggers.values().collect()
    }

    pub fn update_session_logger(
        &mut self,
        arn: &str,
        display_name: Option<String>,
    ) -> Result<&SessionLogger, WorkspacesWebError> {
        let sl = self
            .session_loggers
            .get_mut(arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(arn.to_string()))?;
        if display_name.is_some() {
            sl.display_name = display_name;
        }
        Ok(self.session_loggers.get(arn).unwrap())
    }

    pub fn associate_session_logger(
        &mut self,
        portal_arn: &str,
        session_logger_arn: &str,
    ) -> Result<(), WorkspacesWebError> {
        if !self.portals.contains_key(portal_arn) {
            return Err(WorkspacesWebError::ResourceNotFound(portal_arn.to_string()));
        }
        if !self.session_loggers.contains_key(session_logger_arn) {
            return Err(WorkspacesWebError::ResourceNotFound(
                session_logger_arn.to_string(),
            ));
        }
        self.portals.get_mut(portal_arn).unwrap().session_logger_arn =
            Some(session_logger_arn.to_string());
        let sl = self.session_loggers.get_mut(session_logger_arn).unwrap();
        if !sl.associated_portal_arns.contains(&portal_arn.to_string()) {
            sl.associated_portal_arns.push(portal_arn.to_string());
        }
        Ok(())
    }

    pub fn disassociate_session_logger(
        &mut self,
        portal_arn: &str,
    ) -> Result<(), WorkspacesWebError> {
        let portal = self
            .portals
            .get_mut(portal_arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(portal_arn.to_string()))?;
        portal.session_logger_arn = None;
        Ok(())
    }

    // ── Sessions ─────────────────────────────────────────────────────

    pub fn get_session(
        &self,
        portal_id: &str,
        session_id: &str,
    ) -> Result<&Session, WorkspacesWebError> {
        self.sessions
            .get(session_id)
            .filter(|s| s.portal_id == portal_id)
            .ok_or_else(|| WorkspacesWebError::SessionNotFound(session_id.to_string()))
    }

    pub fn list_sessions(&self, portal_id: &str) -> Vec<&Session> {
        self.sessions
            .values()
            .filter(|s| s.portal_id == portal_id)
            .collect()
    }

    pub fn expire_session(
        &mut self,
        portal_id: &str,
        session_id: &str,
    ) -> Result<(), WorkspacesWebError> {
        let session = self
            .sessions
            .get_mut(session_id)
            .filter(|s| s.portal_id == portal_id)
            .ok_or_else(|| WorkspacesWebError::SessionNotFound(session_id.to_string()))?;
        session.status = "Terminated".to_string();
        session.end_time = Some(Utc::now());
        Ok(())
    }

    // ── Disassociate browser/network/ual/user settings ───────────────

    pub fn disassociate_browser_settings(
        &mut self,
        portal_arn: &str,
    ) -> Result<(), WorkspacesWebError> {
        let portal = self
            .portals
            .get_mut(portal_arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(portal_arn.to_string()))?;
        portal.browser_settings_arn = None;
        Ok(())
    }

    pub fn disassociate_network_settings(
        &mut self,
        portal_arn: &str,
    ) -> Result<(), WorkspacesWebError> {
        let portal = self
            .portals
            .get_mut(portal_arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(portal_arn.to_string()))?;
        portal.network_settings_arn = None;
        Ok(())
    }

    pub fn disassociate_user_access_logging_settings(
        &mut self,
        portal_arn: &str,
    ) -> Result<(), WorkspacesWebError> {
        let portal = self
            .portals
            .get_mut(portal_arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(portal_arn.to_string()))?;
        portal.user_access_logging_settings_arn = None;
        Ok(())
    }

    pub fn disassociate_user_settings(
        &mut self,
        portal_arn: &str,
    ) -> Result<(), WorkspacesWebError> {
        let portal = self
            .portals
            .get_mut(portal_arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(portal_arn.to_string()))?;
        portal.user_settings_arn = None;
        Ok(())
    }

    // ── Update browser/network/ual/user settings ─────────────────────

    pub fn update_browser_settings(
        &mut self,
        arn: &str,
        browser_policy: Option<String>,
    ) -> Result<&BrowserSettings, WorkspacesWebError> {
        let bs = self
            .browser_settings
            .get_mut(arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(arn.to_string()))?;
        if let Some(p) = browser_policy {
            bs.browser_policy = Some(p);
        }
        Ok(self.browser_settings.get(arn).unwrap())
    }

    pub fn update_network_settings(
        &mut self,
        arn: &str,
        vpc_id: Option<String>,
        subnet_ids: Option<Vec<String>>,
        security_group_ids: Option<Vec<String>>,
    ) -> Result<&NetworkSettings, WorkspacesWebError> {
        let ns = self
            .network_settings
            .get_mut(arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(arn.to_string()))?;
        if let Some(v) = vpc_id {
            ns.vpc_id = v;
        }
        if let Some(s) = subnet_ids {
            ns.subnet_ids = s;
        }
        if let Some(sg) = security_group_ids {
            ns.security_group_ids = sg;
        }
        Ok(self.network_settings.get(arn).unwrap())
    }

    pub fn update_user_access_logging_settings(
        &mut self,
        arn: &str,
        kinesis_stream_arn: Option<String>,
    ) -> Result<&UserAccessLoggingSettings, WorkspacesWebError> {
        let ual = self
            .user_access_logging_settings
            .get_mut(arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(arn.to_string()))?;
        if let Some(k) = kinesis_stream_arn {
            ual.kinesis_stream_arn = k;
        }
        Ok(self.user_access_logging_settings.get(arn).unwrap())
    }

    pub fn update_user_settings(
        &mut self,
        arn: &str,
        copy_allowed: Option<String>,
        paste_allowed: Option<String>,
        download_allowed: Option<String>,
        upload_allowed: Option<String>,
        print_allowed: Option<String>,
        disconnect_timeout: Option<i32>,
        idle_disconnect_timeout: Option<i32>,
    ) -> Result<&UserSettings, WorkspacesWebError> {
        let us = self
            .user_settings
            .get_mut(arn)
            .ok_or_else(|| WorkspacesWebError::ResourceNotFound(arn.to_string()))?;
        if let Some(v) = copy_allowed {
            us.copy_allowed = v;
        }
        if let Some(v) = paste_allowed {
            us.paste_allowed = v;
        }
        if let Some(v) = download_allowed {
            us.download_allowed = v;
        }
        if let Some(v) = upload_allowed {
            us.upload_allowed = v;
        }
        if let Some(v) = print_allowed {
            us.print_allowed = v;
        }
        if let Some(v) = disconnect_timeout {
            us.disconnect_timeout_in_minutes = Some(v);
        }
        if let Some(v) = idle_disconnect_timeout {
            us.idle_disconnect_timeout_in_minutes = Some(v);
        }
        Ok(self.user_settings.get(arn).unwrap())
    }

    // ── Tag helpers ─────────────────────────────────────────────────

    pub fn get_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<&HashMap<String, String>, WorkspacesWebError> {
        if let Some(p) = self.portals.get(resource_arn) {
            return Ok(&p.tags);
        }
        if let Some(bs) = self.browser_settings.get(resource_arn) {
            return Ok(&bs.tags);
        }
        if let Some(ns) = self.network_settings.get(resource_arn) {
            return Ok(&ns.tags);
        }
        if let Some(ual) = self.user_access_logging_settings.get(resource_arn) {
            return Ok(&ual.tags);
        }
        if let Some(us) = self.user_settings.get(resource_arn) {
            return Ok(&us.tags);
        }
        if let Some(ias) = self.ip_access_settings.get(resource_arn) {
            return Ok(&ias.tags);
        }
        if let Some(ts) = self.trust_stores.get(resource_arn) {
            return Ok(&ts.tags);
        }
        if let Some(dps) = self.data_protection_settings.get(resource_arn) {
            return Ok(&dps.tags);
        }
        if let Some(sl) = self.session_loggers.get(resource_arn) {
            return Ok(&sl.tags);
        }
        Err(WorkspacesWebError::ResourceNotFound(
            resource_arn.to_string(),
        ))
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), WorkspacesWebError> {
        let tag_map = self.get_tags_mut(resource_arn)?;
        for (k, v) in tags {
            tag_map.insert(k, v);
        }
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), WorkspacesWebError> {
        let tag_map = self.get_tags_mut(resource_arn)?;
        for key in tag_keys {
            tag_map.remove(key);
        }
        Ok(())
    }

    fn get_tags_mut(
        &mut self,
        resource_arn: &str,
    ) -> Result<&mut HashMap<String, String>, WorkspacesWebError> {
        if self.portals.contains_key(resource_arn) {
            return Ok(&mut self.portals.get_mut(resource_arn).unwrap().tags);
        }
        if self.browser_settings.contains_key(resource_arn) {
            return Ok(&mut self.browser_settings.get_mut(resource_arn).unwrap().tags);
        }
        if self.network_settings.contains_key(resource_arn) {
            return Ok(&mut self.network_settings.get_mut(resource_arn).unwrap().tags);
        }
        if self.user_access_logging_settings.contains_key(resource_arn) {
            return Ok(&mut self
                .user_access_logging_settings
                .get_mut(resource_arn)
                .unwrap()
                .tags);
        }
        if self.user_settings.contains_key(resource_arn) {
            return Ok(&mut self.user_settings.get_mut(resource_arn).unwrap().tags);
        }
        if self.ip_access_settings.contains_key(resource_arn) {
            return Ok(&mut self.ip_access_settings.get_mut(resource_arn).unwrap().tags);
        }
        if self.trust_stores.contains_key(resource_arn) {
            return Ok(&mut self.trust_stores.get_mut(resource_arn).unwrap().tags);
        }
        if self.data_protection_settings.contains_key(resource_arn) {
            return Ok(&mut self
                .data_protection_settings
                .get_mut(resource_arn)
                .unwrap()
                .tags);
        }
        if self.session_loggers.contains_key(resource_arn) {
            return Ok(&mut self.session_loggers.get_mut(resource_arn).unwrap().tags);
        }
        Err(WorkspacesWebError::ResourceNotFound(
            resource_arn.to_string(),
        ))
    }
}
