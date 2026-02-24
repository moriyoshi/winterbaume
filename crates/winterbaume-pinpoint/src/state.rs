use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct PinpointState {
    pub apps: HashMap<String, PinpointApp>,
    /// Email channels keyed by application ID.
    pub email_channels: HashMap<String, EmailChannel>,
}

#[derive(Debug, thiserror::Error)]
pub enum PinpointError {
    #[error("Application not found")]
    ApplicationNotFound,

    #[error("Resource not found")]
    ResourceNotFound,
}

impl PinpointState {
    pub fn create_app(
        &mut self,
        name: &str,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&PinpointApp, PinpointError> {
        let id = uuid::Uuid::new_v4().to_string().replace("-", "");
        let arn = format!("arn:aws:mobiletargeting:{region}:{account_id}:apps/{id}");
        let now = Utc::now();

        let app = PinpointApp {
            id: id.clone(),
            arn,
            name: name.to_string(),
            creation_date: now,
            tags,
            settings: None,
            event_stream: None,
            quiet_time: None,
        };

        self.apps.insert(id.clone(), app);
        Ok(self.apps.get(&id).unwrap())
    }

    pub fn get_app(&self, app_id: &str) -> Result<&PinpointApp, PinpointError> {
        self.apps
            .get(app_id)
            .ok_or(PinpointError::ApplicationNotFound)
    }

    pub fn delete_app(&mut self, app_id: &str) -> Result<PinpointApp, PinpointError> {
        self.apps
            .remove(app_id)
            .ok_or(PinpointError::ApplicationNotFound)
    }

    pub fn list_apps(&self) -> Vec<&PinpointApp> {
        self.apps.values().collect()
    }

    pub fn update_application_settings(
        &mut self,
        app_id: &str,
        campaign_hook: Option<CampaignHook>,
        limits: Option<Limits>,
    ) -> Result<&PinpointApp, PinpointError> {
        let app = self
            .apps
            .get_mut(app_id)
            .ok_or(PinpointError::ApplicationNotFound)?;

        let settings = ApplicationSettings {
            campaign_hook,
            limits,
            last_modified_date: Utc::now(),
        };
        app.settings = Some(settings);

        Ok(self.apps.get(app_id).unwrap())
    }

    pub fn get_application_settings(&self, app_id: &str) -> Result<&PinpointApp, PinpointError> {
        self.get_app(app_id)
    }

    pub fn put_event_stream(
        &mut self,
        app_id: &str,
        destination_stream_arn: &str,
        role_arn: &str,
    ) -> Result<&EventStream, PinpointError> {
        let app = self
            .apps
            .get_mut(app_id)
            .ok_or(PinpointError::ApplicationNotFound)?;

        let event_stream = EventStream {
            application_id: app_id.to_string(),
            destination_stream_arn: destination_stream_arn.to_string(),
            role_arn: role_arn.to_string(),
            last_modified_date: Utc::now(),
        };
        app.event_stream = Some(event_stream);

        Ok(self
            .apps
            .get(app_id)
            .unwrap()
            .event_stream
            .as_ref()
            .unwrap())
    }

    pub fn get_event_stream(&self, app_id: &str) -> Result<&EventStream, PinpointError> {
        let app = self.get_app(app_id)?;
        app.event_stream
            .as_ref()
            .ok_or(PinpointError::ResourceNotFound)
    }

    pub fn delete_event_stream(&mut self, app_id: &str) -> Result<EventStream, PinpointError> {
        let app = self
            .apps
            .get_mut(app_id)
            .ok_or(PinpointError::ApplicationNotFound)?;

        app.event_stream
            .take()
            .ok_or(PinpointError::ResourceNotFound)
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), PinpointError> {
        // Find app by ARN
        let app = self
            .apps
            .values_mut()
            .find(|a| a.arn == resource_arn)
            .ok_or(PinpointError::ResourceNotFound)?;

        for (k, v) in tags {
            app.tags.insert(k, v);
        }
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), PinpointError> {
        let app = self
            .apps
            .values_mut()
            .find(|a| a.arn == resource_arn)
            .ok_or(PinpointError::ResourceNotFound)?;

        for key in tag_keys {
            app.tags.remove(key);
        }
        Ok(())
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<&HashMap<String, String>, PinpointError> {
        let app = self
            .apps
            .values()
            .find(|a| a.arn == resource_arn)
            .ok_or(PinpointError::ResourceNotFound)?;

        Ok(&app.tags)
    }

    pub fn update_email_channel(
        &mut self,
        app_id: &str,
        enabled: bool,
        from_address: String,
        identity: String,
        role_arn: Option<String>,
        configuration_set: Option<String>,
    ) -> Result<&EmailChannel, PinpointError> {
        if !self.apps.contains_key(app_id) {
            return Err(PinpointError::ApplicationNotFound);
        }

        let channel = EmailChannel {
            application_id: app_id.to_string(),
            enabled,
            from_address,
            identity,
            role_arn,
            configuration_set,
            messages_per_second: None,
        };

        self.email_channels.insert(app_id.to_string(), channel);
        Ok(self.email_channels.get(app_id).unwrap())
    }

    pub fn get_email_channel(&self, app_id: &str) -> Result<&EmailChannel, PinpointError> {
        self.email_channels
            .get(app_id)
            .ok_or(PinpointError::ResourceNotFound)
    }

    pub fn delete_email_channel(&mut self, app_id: &str) -> Result<EmailChannel, PinpointError> {
        if !self.apps.contains_key(app_id) {
            return Err(PinpointError::ApplicationNotFound);
        }
        self.email_channels
            .remove(app_id)
            .ok_or(PinpointError::ResourceNotFound)
    }
}
