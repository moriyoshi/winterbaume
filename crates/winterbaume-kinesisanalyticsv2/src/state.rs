use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct KinesisAnalyticsV2State {
    pub applications: HashMap<String, Application>,
}

#[derive(Debug, thiserror::Error)]
pub enum KinesisAnalyticsV2Error {
    #[error("Application {0} already exists.")]
    ApplicationAlreadyExists(String),
    #[error("Application {0} is not found.")]
    ApplicationNotFound(String),
    #[error("Application with ARN {0} is not found.")]
    ApplicationArnNotFound(String),
    #[error("Snapshot {0} already exists.")]
    SnapshotAlreadyExists(String),
    #[error("Snapshot {0} is not found.")]
    SnapshotNotFound(String),
}

impl KinesisAnalyticsV2State {
    pub fn create_application(
        &mut self,
        application_name: &str,
        runtime_environment: &str,
        service_execution_role: Option<&str>,
        application_description: Option<&str>,
        account_id: &str,
        region: &str,
    ) -> Result<&Application, KinesisAnalyticsV2Error> {
        if self.applications.contains_key(application_name) {
            return Err(KinesisAnalyticsV2Error::ApplicationAlreadyExists(
                application_name.to_string(),
            ));
        }

        let arn = format!(
            "arn:aws:kinesisanalytics:{region}:{account_id}:application/{application_name}"
        );
        let now = Utc::now();

        let application = Application {
            application_name: application_name.to_string(),
            application_arn: arn,
            application_status: "READY".to_string(),
            application_version_id: 1,
            runtime_environment: runtime_environment.to_string(),
            service_execution_role: service_execution_role.map(|s| s.to_string()),
            create_timestamp: now,
            last_update_timestamp: now,
            application_description: application_description.map(|s| s.to_string()),
            tags: HashMap::new(),
            snapshots: Vec::new(),
            application_configuration: None,
            cloudwatch_logging_options: None,
        };

        self.applications
            .insert(application_name.to_string(), application);
        Ok(self.applications.get(application_name).unwrap())
    }

    pub fn describe_application(
        &self,
        application_name: &str,
    ) -> Result<&Application, KinesisAnalyticsV2Error> {
        self.applications.get(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })
    }

    pub fn delete_application(
        &mut self,
        application_name: &str,
        _create_timestamp: Option<f64>,
    ) -> Result<(), KinesisAnalyticsV2Error> {
        if self.applications.remove(application_name).is_none() {
            return Err(KinesisAnalyticsV2Error::ApplicationNotFound(
                application_name.to_string(),
            ));
        }
        Ok(())
    }

    pub fn list_applications(&self) -> Vec<&Application> {
        self.applications.values().collect()
    }

    pub fn start_application(
        &mut self,
        application_name: &str,
    ) -> Result<(), KinesisAnalyticsV2Error> {
        let app = self.applications.get_mut(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })?;
        app.application_status = "RUNNING".to_string();
        app.last_update_timestamp = Utc::now();
        Ok(())
    }

    pub fn stop_application(
        &mut self,
        application_name: &str,
    ) -> Result<(), KinesisAnalyticsV2Error> {
        let app = self.applications.get_mut(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })?;
        app.application_status = "READY".to_string();
        app.last_update_timestamp = Utc::now();
        Ok(())
    }

    pub fn update_application(
        &mut self,
        application_name: &str,
        runtime_environment_update: Option<&str>,
        service_execution_role_update: Option<&str>,
    ) -> Result<&Application, KinesisAnalyticsV2Error> {
        let app = self.applications.get_mut(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })?;
        if let Some(rt) = runtime_environment_update {
            app.runtime_environment = rt.to_string();
        }
        if let Some(role) = service_execution_role_update {
            app.service_execution_role = Some(role.to_string());
        }
        app.application_version_id += 1;
        app.last_update_timestamp = Utc::now();
        Ok(self.applications.get(application_name).unwrap())
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: Vec<(String, String)>,
    ) -> Result<(), KinesisAnalyticsV2Error> {
        let app = self
            .applications
            .values_mut()
            .find(|a| a.application_arn == resource_arn)
            .ok_or_else(|| {
                KinesisAnalyticsV2Error::ApplicationArnNotFound(resource_arn.to_string())
            })?;
        for (k, v) in tags {
            app.tags.insert(k, v);
        }
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), KinesisAnalyticsV2Error> {
        let app = self
            .applications
            .values_mut()
            .find(|a| a.application_arn == resource_arn)
            .ok_or_else(|| {
                KinesisAnalyticsV2Error::ApplicationArnNotFound(resource_arn.to_string())
            })?;
        for k in tag_keys {
            app.tags.remove(k);
        }
        Ok(())
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<Vec<(String, String)>, KinesisAnalyticsV2Error> {
        let app = self
            .applications
            .values()
            .find(|a| a.application_arn == resource_arn)
            .ok_or_else(|| {
                KinesisAnalyticsV2Error::ApplicationArnNotFound(resource_arn.to_string())
            })?;
        Ok(app
            .tags
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect())
    }

    pub fn create_application_snapshot(
        &mut self,
        application_name: &str,
        snapshot_name: &str,
    ) -> Result<(), KinesisAnalyticsV2Error> {
        let app = self.applications.get_mut(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })?;
        if app
            .snapshots
            .iter()
            .any(|s| s.snapshot_name == snapshot_name)
        {
            return Err(KinesisAnalyticsV2Error::SnapshotAlreadyExists(
                snapshot_name.to_string(),
            ));
        }
        let snapshot = Snapshot {
            snapshot_name: snapshot_name.to_string(),
            application_version_id: app.application_version_id,
            runtime_environment: app.runtime_environment.clone(),
            snapshot_creation_timestamp: Utc::now(),
        };
        app.snapshots.push(snapshot);
        Ok(())
    }

    pub fn delete_application_snapshot(
        &mut self,
        application_name: &str,
        snapshot_name: &str,
    ) -> Result<(), KinesisAnalyticsV2Error> {
        let app = self.applications.get_mut(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })?;
        let pos = app
            .snapshots
            .iter()
            .position(|s| s.snapshot_name == snapshot_name);
        match pos {
            Some(idx) => {
                app.snapshots.remove(idx);
                Ok(())
            }
            None => Err(KinesisAnalyticsV2Error::SnapshotNotFound(
                snapshot_name.to_string(),
            )),
        }
    }

    pub fn describe_application_snapshot(
        &self,
        application_name: &str,
        snapshot_name: &str,
    ) -> Result<&Snapshot, KinesisAnalyticsV2Error> {
        let app = self.applications.get(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })?;
        app.snapshots
            .iter()
            .find(|s| s.snapshot_name == snapshot_name)
            .ok_or_else(|| KinesisAnalyticsV2Error::SnapshotNotFound(snapshot_name.to_string()))
    }

    pub fn list_application_snapshots(
        &self,
        application_name: &str,
    ) -> Result<Vec<&Snapshot>, KinesisAnalyticsV2Error> {
        let app = self.applications.get(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })?;
        Ok(app.snapshots.iter().collect())
    }

    pub fn list_application_versions(
        &self,
        application_name: &str,
    ) -> Result<Vec<i64>, KinesisAnalyticsV2Error> {
        let app = self.applications.get(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })?;
        // Return version IDs from 1 to current
        Ok((1..=app.application_version_id).collect())
    }

    pub fn describe_application_version(
        &self,
        application_name: &str,
        _application_version_id: i64,
    ) -> Result<&Application, KinesisAnalyticsV2Error> {
        // For simplicity, we return the current application detail regardless of version
        self.applications.get(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })
    }

    pub fn rollback_application(
        &mut self,
        application_name: &str,
    ) -> Result<&Application, KinesisAnalyticsV2Error> {
        let app = self.applications.get_mut(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })?;
        if app.application_version_id > 1 {
            app.application_version_id -= 1;
        }
        app.last_update_timestamp = Utc::now();
        Ok(self.applications.get(application_name).unwrap())
    }

    pub fn update_application_maintenance_configuration(
        &mut self,
        application_name: &str,
    ) -> Result<&Application, KinesisAnalyticsV2Error> {
        let app = self.applications.get_mut(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })?;
        app.last_update_timestamp = Utc::now();
        Ok(self.applications.get(application_name).unwrap())
    }

    pub fn add_application_cloud_watch_logging_option(
        &mut self,
        application_name: &str,
    ) -> Result<&Application, KinesisAnalyticsV2Error> {
        let app = self.applications.get_mut(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })?;
        app.application_version_id += 1;
        app.last_update_timestamp = Utc::now();
        Ok(self.applications.get(application_name).unwrap())
    }

    pub fn delete_application_cloud_watch_logging_option(
        &mut self,
        application_name: &str,
    ) -> Result<&Application, KinesisAnalyticsV2Error> {
        let app = self.applications.get_mut(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })?;
        app.application_version_id += 1;
        app.last_update_timestamp = Utc::now();
        Ok(self.applications.get(application_name).unwrap())
    }

    pub fn add_application_vpc_configuration(
        &mut self,
        application_name: &str,
    ) -> Result<&Application, KinesisAnalyticsV2Error> {
        let app = self.applications.get_mut(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })?;
        app.application_version_id += 1;
        app.last_update_timestamp = Utc::now();
        Ok(self.applications.get(application_name).unwrap())
    }

    pub fn delete_application_vpc_configuration(
        &mut self,
        application_name: &str,
    ) -> Result<&Application, KinesisAnalyticsV2Error> {
        let app = self.applications.get_mut(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })?;
        app.application_version_id += 1;
        app.last_update_timestamp = Utc::now();
        Ok(self.applications.get(application_name).unwrap())
    }

    pub fn add_application_input(
        &mut self,
        application_name: &str,
    ) -> Result<&Application, KinesisAnalyticsV2Error> {
        let app = self.applications.get_mut(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })?;
        app.application_version_id += 1;
        app.last_update_timestamp = Utc::now();
        Ok(self.applications.get(application_name).unwrap())
    }

    pub fn add_application_input_processing_configuration(
        &mut self,
        application_name: &str,
    ) -> Result<&Application, KinesisAnalyticsV2Error> {
        let app = self.applications.get_mut(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })?;
        app.application_version_id += 1;
        app.last_update_timestamp = Utc::now();
        Ok(self.applications.get(application_name).unwrap())
    }

    pub fn delete_application_input_processing_configuration(
        &mut self,
        application_name: &str,
    ) -> Result<&Application, KinesisAnalyticsV2Error> {
        let app = self.applications.get_mut(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })?;
        app.application_version_id += 1;
        app.last_update_timestamp = Utc::now();
        Ok(self.applications.get(application_name).unwrap())
    }

    pub fn add_application_output(
        &mut self,
        application_name: &str,
    ) -> Result<&Application, KinesisAnalyticsV2Error> {
        let app = self.applications.get_mut(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })?;
        app.application_version_id += 1;
        app.last_update_timestamp = Utc::now();
        Ok(self.applications.get(application_name).unwrap())
    }

    pub fn delete_application_output(
        &mut self,
        application_name: &str,
    ) -> Result<&Application, KinesisAnalyticsV2Error> {
        let app = self.applications.get_mut(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })?;
        app.application_version_id += 1;
        app.last_update_timestamp = Utc::now();
        Ok(self.applications.get(application_name).unwrap())
    }

    pub fn add_application_reference_data_source(
        &mut self,
        application_name: &str,
    ) -> Result<&Application, KinesisAnalyticsV2Error> {
        let app = self.applications.get_mut(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })?;
        app.application_version_id += 1;
        app.last_update_timestamp = Utc::now();
        Ok(self.applications.get(application_name).unwrap())
    }

    pub fn delete_application_reference_data_source(
        &mut self,
        application_name: &str,
    ) -> Result<&Application, KinesisAnalyticsV2Error> {
        let app = self.applications.get_mut(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })?;
        app.application_version_id += 1;
        app.last_update_timestamp = Utc::now();
        Ok(self.applications.get(application_name).unwrap())
    }

    pub fn describe_application_operation(
        &self,
        application_name: &str,
    ) -> Result<&Application, KinesisAnalyticsV2Error> {
        self.applications.get(application_name).ok_or_else(|| {
            KinesisAnalyticsV2Error::ApplicationNotFound(application_name.to_string())
        })
    }

    pub fn list_application_operations(
        &self,
        application_name: &str,
    ) -> Result<(), KinesisAnalyticsV2Error> {
        if !self.applications.contains_key(application_name) {
            return Err(KinesisAnalyticsV2Error::ApplicationNotFound(
                application_name.to_string(),
            ));
        }
        Ok(())
    }
}
