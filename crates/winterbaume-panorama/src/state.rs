use std::collections::HashMap;

use crate::types::{ApplicationInstance, Device, NodeFromTemplateJob};

#[derive(Debug, Default)]
pub struct PanoramaState {
    pub application_instances: HashMap<String, ApplicationInstance>,
    pub devices: HashMap<String, Device>,
    pub node_from_template_jobs: HashMap<String, NodeFromTemplateJob>,
}

#[derive(Debug, thiserror::Error)]
pub enum PanoramaError {
    #[error("Application instance {0} not found")]
    ApplicationInstanceNotFound(String),
    #[error("Device {0} not found")]
    DeviceNotFound(String),
    #[error("Node from template job {0} not found")]
    NodeFromTemplateJobNotFound(String),
}

impl PanoramaState {
    pub fn create_application_instance(
        &mut self,
        id: String,
        name: String,
        description: Option<String>,
        default_runtime_context_device: String,
        application_instance_id_to_replace: Option<String>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> String {
        let arn = format!(
            "arn:aws:panorama:{}:{}:applicationInstance/{}",
            region, account_id, id
        );
        let instance = ApplicationInstance {
            application_instance_id: id.clone(),
            name,
            status: "DEPLOYMENT_PENDING".to_string(),
            description,
            default_runtime_context_device,
            application_instance_id_to_replace,
            tags,
            arn,
            created_time: current_timestamp(),
        };
        self.application_instances.insert(id.clone(), instance);
        id
    }

    pub fn describe_application_instance(
        &self,
        id: &str,
    ) -> Result<&ApplicationInstance, PanoramaError> {
        self.application_instances
            .get(id)
            .ok_or_else(|| PanoramaError::ApplicationInstanceNotFound(id.to_string()))
    }

    pub fn list_application_instances(&self) -> Vec<&ApplicationInstance> {
        let mut items: Vec<&ApplicationInstance> = self.application_instances.values().collect();
        items.sort_by(|a, b| a.application_instance_id.cmp(&b.application_instance_id));
        items
    }

    pub fn provision_device(
        &mut self,
        id: String,
        name: String,
        description: Option<String>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> &Device {
        let arn = format!("arn:aws:panorama:{}:{}:device/{}", region, account_id, id);
        let device = Device {
            device_id: id.clone(),
            name,
            arn,
            status: "AWAITING_PROVISIONING".to_string(),
            description,
            tags,
            created_time: current_timestamp(),
        };
        self.devices.insert(id.clone(), device);
        self.devices.get(&id).unwrap()
    }

    pub fn describe_device(&self, id: &str) -> Result<&Device, PanoramaError> {
        self.devices
            .get(id)
            .ok_or_else(|| PanoramaError::DeviceNotFound(id.to_string()))
    }

    pub fn delete_device(&mut self, id: &str) -> Result<String, PanoramaError> {
        self.devices
            .remove(id)
            .map(|d| d.device_id)
            .ok_or_else(|| PanoramaError::DeviceNotFound(id.to_string()))
    }

    pub fn list_devices(&self) -> Vec<&Device> {
        let mut items: Vec<&Device> = self.devices.values().collect();
        items.sort_by(|a, b| a.device_id.cmp(&b.device_id));
        items
    }

    pub fn update_device_metadata(
        &mut self,
        id: &str,
        description: Option<String>,
    ) -> Result<String, PanoramaError> {
        let device = self
            .devices
            .get_mut(id)
            .ok_or_else(|| PanoramaError::DeviceNotFound(id.to_string()))?;
        if let Some(desc) = description {
            device.description = Some(desc);
        }
        Ok(id.to_string())
    }

    pub fn create_node_from_template_job(
        &mut self,
        job_id: String,
        node_name: String,
        template_type: String,
    ) -> String {
        let job = NodeFromTemplateJob {
            job_id: job_id.clone(),
            node_name,
            status: "PENDING".to_string(),
            template_type,
            created_time: current_timestamp(),
        };
        self.node_from_template_jobs.insert(job_id.clone(), job);
        job_id
    }

    pub fn describe_node_from_template_job(
        &self,
        job_id: &str,
    ) -> Result<&NodeFromTemplateJob, PanoramaError> {
        self.node_from_template_jobs
            .get(job_id)
            .ok_or_else(|| PanoramaError::NodeFromTemplateJobNotFound(job_id.to_string()))
    }

    pub fn list_nodes(&self) -> Vec<()> {
        // No real nodes in this mock
        vec![]
    }
}

fn current_timestamp() -> String {
    chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}
