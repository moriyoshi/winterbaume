use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct EmrContainersState {
    pub virtual_clusters: HashMap<String, VirtualCluster>,
    /// Job runs keyed by job run ID.
    pub job_runs: HashMap<String, JobRun>,
    /// Managed endpoints keyed by endpoint ID.
    pub managed_endpoints: HashMap<String, ManagedEndpoint>,
    /// Job templates keyed by template ID.
    pub job_templates: HashMap<String, JobTemplate>,
    /// Security configurations keyed by config ID.
    pub security_configurations: HashMap<String, SecurityConfiguration>,
}

/// Release labels must follow the pattern emr-X.Y.Z-latest or emr-X.Y.Z-YYYYMMDD
fn is_valid_release_label(label: &str) -> bool {
    if !label.starts_with("emr-") {
        return false;
    }
    let rest = &label[4..];
    // Split at last '-'
    let parts: Vec<&str> = rest.splitn(2, '-').collect();
    if parts.len() < 2 {
        return false;
    }
    // Version part must have at least one dot
    if !parts[0].contains('.') {
        return false;
    }
    // Suffix must be "latest" or a date
    let suffix = parts[1];
    suffix == "latest" || suffix.len() == 8 && suffix.chars().all(|c| c.is_ascii_digit())
}

#[derive(Debug, thiserror::Error)]
pub enum EmrContainersError {
    #[error("A virtual cluster already exists in the given namespace")]
    VirtualClusterNamespaceConflict,
    #[error("Virtual cluster {0} doesn't exist.")]
    VirtualClusterNotFound(String),
    #[error("VirtualCluster does not exist")]
    VirtualClusterDoesNotExist,
    #[error("Virtual cluster {0} doesn't exist.")]
    StartJobRunVirtualClusterNotFound(String),
    #[error("Release {0} doesn't exist.")]
    ReleaseLabelNotFound(String),
    #[error("Invalid job run short id")]
    InvalidJobRunId,
    #[error("Job run {0} doesn't exist.")]
    JobRunNotFound(String),
    #[error("Job run {0} is not in a cancellable state")]
    JobRunNotCancellable(String),
    #[error("Virtual cluster {0} doesn't exist.")]
    ManagedEndpointVirtualClusterNotFound(String),
    #[error("Managed endpoint {0} doesn't exist.")]
    ManagedEndpointNotFound(String),
    #[error("Job template {0} doesn't exist.")]
    JobTemplateNotFound(String),
    #[error("Security configuration {0} doesn't exist.")]
    SecurityConfigurationNotFound(String),
    #[error("Resource {0} doesn't exist.")]
    ResourceNotFound(String),
}

impl EmrContainersState {
    pub fn create_virtual_cluster(
        &mut self,
        name: &str,
        container_provider: ContainerProvider,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&VirtualCluster, EmrContainersError> {
        // Check namespace uniqueness
        let namespace = container_provider
            .info
            .as_ref()
            .and_then(|info| info.eks_info.as_ref())
            .and_then(|eks| eks.namespace.as_deref())
            .unwrap_or("");
        if !namespace.is_empty() {
            let namespace_already_used = self.virtual_clusters.values().any(|vc| {
                vc.container_provider
                    .info
                    .as_ref()
                    .and_then(|i| i.eks_info.as_ref())
                    .and_then(|e| e.namespace.as_deref())
                    .map(|ns| ns == namespace)
                    .unwrap_or(false)
            });
            if namespace_already_used {
                return Err(EmrContainersError::VirtualClusterNamespaceConflict);
            }
        }

        let id = uuid::Uuid::new_v4()
            .to_string()
            .replace('-', "")
            .chars()
            .take(25)
            .collect::<String>();

        let arn = format!("arn:aws:emr-containers:{region}:{account_id}:/virtualclusters/{id}");

        let cluster = VirtualCluster {
            id: id.clone(),
            name: name.to_string(),
            arn,
            state: "RUNNING".to_string(),
            container_provider,
            created_at: Utc::now(),
            tags,
        };

        self.virtual_clusters.insert(id.clone(), cluster);
        Ok(self.virtual_clusters.get(&id).unwrap())
    }

    pub fn describe_virtual_cluster(
        &self,
        id: &str,
    ) -> Result<&VirtualCluster, EmrContainersError> {
        self.virtual_clusters
            .get(id)
            .ok_or_else(|| EmrContainersError::VirtualClusterNotFound(id.to_string()))
    }

    pub fn delete_virtual_cluster(
        &mut self,
        id: &str,
    ) -> Result<&VirtualCluster, EmrContainersError> {
        match self.virtual_clusters.get_mut(id) {
            Some(cluster) => {
                cluster.state = "TERMINATED".to_string();
                Ok(self.virtual_clusters.get(id).unwrap())
            }
            None => Err(EmrContainersError::VirtualClusterDoesNotExist),
        }
    }

    pub fn list_virtual_clusters(&self) -> Vec<&VirtualCluster> {
        self.virtual_clusters.values().collect()
    }

    pub fn start_job_run(
        &mut self,
        virtual_cluster_id: &str,
        name: Option<&str>,
        execution_role_arn: &str,
        release_label: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&JobRun, EmrContainersError> {
        if !self.virtual_clusters.contains_key(virtual_cluster_id) {
            return Err(EmrContainersError::StartJobRunVirtualClusterNotFound(
                virtual_cluster_id.to_string(),
            ));
        }

        // Validate release label format: must match emr-X.Y.Z-latest or emr-X.Y.Z-YYYYMMDD
        if !is_valid_release_label(release_label) {
            return Err(EmrContainersError::ReleaseLabelNotFound(
                release_label.to_string(),
            ));
        }

        let id = uuid::Uuid::new_v4()
            .to_string()
            .replace('-', "")
            .chars()
            .take(19)
            .collect::<String>();
        let arn = format!(
            "arn:aws:emr-containers:{region}:{account_id}:/virtualclusters/{virtual_cluster_id}/jobruns/{id}"
        );

        let job_run = JobRun {
            id: id.clone(),
            name: name.map(|n| n.to_string()),
            virtual_cluster_id: virtual_cluster_id.to_string(),
            arn,
            state: "RUNNING".to_string(),
            execution_role_arn: execution_role_arn.to_string(),
            release_label: release_label.to_string(),
            created_at: Utc::now(),
        };

        self.job_runs.insert(id.clone(), job_run);
        Ok(self.job_runs.get(&id).unwrap())
    }

    pub fn describe_job_run(
        &self,
        virtual_cluster_id: &str,
        job_run_id: &str,
    ) -> Result<&JobRun, EmrContainersError> {
        // Job run IDs are 19 chars (alphanumeric lowercase)
        if job_run_id.len() != 19 || !job_run_id.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(EmrContainersError::InvalidJobRunId);
        }

        // Check if it exists in this virtual cluster
        match self.job_runs.get(job_run_id) {
            Some(jr) if jr.virtual_cluster_id == virtual_cluster_id => Ok(jr),
            _ => Err(EmrContainersError::JobRunNotFound(job_run_id.to_string())),
        }
    }

    pub fn cancel_job_run(
        &mut self,
        virtual_cluster_id: &str,
        job_run_id: &str,
    ) -> Result<&JobRun, EmrContainersError> {
        // Job run IDs are 19 chars (alphanumeric lowercase)
        if job_run_id.len() != 19 || !job_run_id.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(EmrContainersError::InvalidJobRunId);
        }

        // Check if it exists in this virtual cluster
        match self.job_runs.get(job_run_id) {
            Some(jr) if jr.virtual_cluster_id != virtual_cluster_id => {
                return Err(EmrContainersError::JobRunNotFound(job_run_id.to_string()));
            }
            None => {
                return Err(EmrContainersError::JobRunNotFound(job_run_id.to_string()));
            }
            _ => {}
        }

        let jr = self.job_runs.get_mut(job_run_id).unwrap();

        // Only certain states are cancellable
        let cancellable_states = ["PENDING", "SUBMITTED", "RUNNING", "CANCEL_PENDING"];
        if !cancellable_states.contains(&jr.state.as_str()) {
            return Err(EmrContainersError::JobRunNotCancellable(
                job_run_id.to_string(),
            ));
        }

        jr.state = "CANCELLED".to_string();
        Ok(self.job_runs.get(job_run_id).unwrap())
    }

    pub fn list_job_runs(&self, virtual_cluster_id: &str) -> Vec<&JobRun> {
        self.job_runs
            .values()
            .filter(|jr| jr.virtual_cluster_id == virtual_cluster_id)
            .collect()
    }

    // ---- Managed Endpoints ----

    pub fn create_managed_endpoint(
        &mut self,
        virtual_cluster_id: &str,
        name: &str,
        endpoint_type: &str,
        release_label: &str,
        execution_role_arn: &str,
        certificate_arn: Option<&str>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&ManagedEndpoint, EmrContainersError> {
        if !self.virtual_clusters.contains_key(virtual_cluster_id) {
            return Err(EmrContainersError::ManagedEndpointVirtualClusterNotFound(
                virtual_cluster_id.to_string(),
            ));
        }

        let id = uuid::Uuid::new_v4()
            .to_string()
            .replace('-', "")
            .chars()
            .take(17)
            .collect::<String>();

        let arn = format!(
            "arn:aws:emr-containers:{region}:{account_id}:/virtualclusters/{virtual_cluster_id}/endpoints/{id}"
        );

        let endpoint = ManagedEndpoint {
            id: id.clone(),
            name: name.to_string(),
            virtual_cluster_id: virtual_cluster_id.to_string(),
            arn,
            state: "ACTIVE".to_string(),
            endpoint_type: endpoint_type.to_string(),
            release_label: release_label.to_string(),
            execution_role_arn: execution_role_arn.to_string(),
            certificate_arn: certificate_arn.map(String::from),
            created_at: Utc::now(),
            tags,
        };

        self.managed_endpoints.insert(id.clone(), endpoint);
        Ok(self.managed_endpoints.get(&id).unwrap())
    }

    pub fn describe_managed_endpoint(
        &self,
        virtual_cluster_id: &str,
        endpoint_id: &str,
    ) -> Result<&ManagedEndpoint, EmrContainersError> {
        match self.managed_endpoints.get(endpoint_id) {
            Some(ep) if ep.virtual_cluster_id == virtual_cluster_id => Ok(ep),
            _ => Err(EmrContainersError::ManagedEndpointNotFound(
                endpoint_id.to_string(),
            )),
        }
    }

    pub fn delete_managed_endpoint(
        &mut self,
        virtual_cluster_id: &str,
        endpoint_id: &str,
    ) -> Result<(String, String), EmrContainersError> {
        match self.managed_endpoints.get(endpoint_id) {
            Some(ep) if ep.virtual_cluster_id == virtual_cluster_id => {
                let id = ep.id.clone();
                let vc_id = ep.virtual_cluster_id.clone();
                self.managed_endpoints.remove(endpoint_id);
                Ok((id, vc_id))
            }
            _ => Err(EmrContainersError::ManagedEndpointNotFound(
                endpoint_id.to_string(),
            )),
        }
    }

    pub fn list_managed_endpoints(&self, virtual_cluster_id: &str) -> Vec<&ManagedEndpoint> {
        self.managed_endpoints
            .values()
            .filter(|ep| ep.virtual_cluster_id == virtual_cluster_id)
            .collect()
    }

    // ---- Job Templates ----

    pub fn create_job_template(
        &mut self,
        name: &str,
        kms_key_arn: Option<&str>,
        job_template_data: serde_json::Value,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&JobTemplate, EmrContainersError> {
        let id = uuid::Uuid::new_v4()
            .to_string()
            .replace('-', "")
            .chars()
            .take(17)
            .collect::<String>();

        let arn = format!("arn:aws:emr-containers:{region}:{account_id}:/jobtemplates/{id}");

        let template = JobTemplate {
            id: id.clone(),
            name: name.to_string(),
            arn,
            created_at: Utc::now(),
            kms_key_arn: kms_key_arn.map(String::from),
            tags,
            job_template_data,
        };

        self.job_templates.insert(id.clone(), template);
        Ok(self.job_templates.get(&id).unwrap())
    }

    pub fn describe_job_template(&self, id: &str) -> Result<&JobTemplate, EmrContainersError> {
        self.job_templates
            .get(id)
            .ok_or_else(|| EmrContainersError::JobTemplateNotFound(id.to_string()))
    }

    pub fn delete_job_template(&mut self, id: &str) -> Result<String, EmrContainersError> {
        match self.job_templates.remove(id) {
            Some(t) => Ok(t.id),
            None => Err(EmrContainersError::JobTemplateNotFound(id.to_string())),
        }
    }

    pub fn list_job_templates(&self) -> Vec<&JobTemplate> {
        self.job_templates.values().collect()
    }

    // ---- Security Configurations ----

    pub fn create_security_configuration(
        &mut self,
        name: &str,
        security_configuration_data: serde_json::Value,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&SecurityConfiguration, EmrContainersError> {
        let id = uuid::Uuid::new_v4()
            .to_string()
            .replace('-', "")
            .chars()
            .take(17)
            .collect::<String>();

        let arn =
            format!("arn:aws:emr-containers:{region}:{account_id}:/securityconfigurations/{id}");

        let config = SecurityConfiguration {
            id: id.clone(),
            name: name.to_string(),
            arn,
            created_at: Utc::now(),
            tags,
            security_configuration_data,
        };

        self.security_configurations.insert(id.clone(), config);
        Ok(self.security_configurations.get(&id).unwrap())
    }

    pub fn describe_security_configuration(
        &self,
        id: &str,
    ) -> Result<&SecurityConfiguration, EmrContainersError> {
        self.security_configurations
            .get(id)
            .ok_or_else(|| EmrContainersError::SecurityConfigurationNotFound(id.to_string()))
    }

    pub fn list_security_configurations(&self) -> Vec<&SecurityConfiguration> {
        self.security_configurations.values().collect()
    }

    // ---- Tags ----

    /// Returns a mutable reference to the tags for a resource identified by its ARN.
    fn tags_for_arn_mut(&mut self, resource_arn: &str) -> Option<&mut HashMap<String, String>> {
        // Virtual clusters
        if let Some(vc) = self
            .virtual_clusters
            .values_mut()
            .find(|v| v.arn == resource_arn)
        {
            return Some(&mut vc.tags);
        }
        // Managed endpoints
        if let Some(ep) = self
            .managed_endpoints
            .values_mut()
            .find(|e| e.arn == resource_arn)
        {
            return Some(&mut ep.tags);
        }
        // Job templates
        if let Some(t) = self
            .job_templates
            .values_mut()
            .find(|t| t.arn == resource_arn)
        {
            return Some(&mut t.tags);
        }
        // Security configurations
        if let Some(sc) = self
            .security_configurations
            .values_mut()
            .find(|s| s.arn == resource_arn)
        {
            return Some(&mut sc.tags);
        }
        None
    }

    fn tags_for_arn(&self, resource_arn: &str) -> Option<&HashMap<String, String>> {
        if let Some(vc) = self
            .virtual_clusters
            .values()
            .find(|v| v.arn == resource_arn)
        {
            return Some(&vc.tags);
        }
        if let Some(ep) = self
            .managed_endpoints
            .values()
            .find(|e| e.arn == resource_arn)
        {
            return Some(&ep.tags);
        }
        if let Some(t) = self.job_templates.values().find(|t| t.arn == resource_arn) {
            return Some(&t.tags);
        }
        if let Some(sc) = self
            .security_configurations
            .values()
            .find(|s| s.arn == resource_arn)
        {
            return Some(&sc.tags);
        }
        None
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<HashMap<String, String>, EmrContainersError> {
        match self.tags_for_arn(resource_arn) {
            Some(tags) => Ok(tags.clone()),
            None => Err(EmrContainersError::ResourceNotFound(
                resource_arn.to_string(),
            )),
        }
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), EmrContainersError> {
        match self.tags_for_arn_mut(resource_arn) {
            Some(existing) => {
                existing.extend(tags);
                Ok(())
            }
            None => Err(EmrContainersError::ResourceNotFound(
                resource_arn.to_string(),
            )),
        }
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), EmrContainersError> {
        match self.tags_for_arn_mut(resource_arn) {
            Some(existing) => {
                for key in tag_keys {
                    existing.remove(key);
                }
                Ok(())
            }
            None => Err(EmrContainersError::ResourceNotFound(
                resource_arn.to_string(),
            )),
        }
    }
}
