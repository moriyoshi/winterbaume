//! State management for the EMR service.

use std::collections::HashMap;

use chrono::Utc;
use uuid::Uuid;

use crate::types::*;

#[derive(Debug, Default)]
pub struct EmrState {
    pub clusters: HashMap<String, ClusterData>,
    pub steps: HashMap<String, Vec<StepData>>, // cluster_id -> steps
    pub security_configurations: HashMap<String, SecurityConfigurationData>,
    pub block_public_access_config: Option<BlockPublicAccessConfigurationData>,
    pub studios: HashMap<String, StudioData>,
    /// Session mappings keyed by (studio_id, identity_id, identity_type).
    pub session_mappings: Vec<SessionMappingData>,
    pub notebook_executions: HashMap<String, NotebookExecutionData>,
    pub persistent_app_uis: HashMap<String, PersistentAppUiData>,
}

#[derive(Debug, thiserror::Error)]
pub enum EmrError {
    #[error("Cluster {0} does not exist.")]
    ClusterNotFound(String),
    #[error("Step {0} not found in cluster {1}.")]
    StepNotFound(String, String),
    #[error("Security configuration '{0}' already exists.")]
    SecurityConfigurationAlreadyExists(String),
    #[error("Security configuration '{0}' does not exist.")]
    SecurityConfigurationNotFound(String),
    #[error("Resource {0} not found.")]
    ResourceNotFound(String),
    #[error("Studio {0} not found.")]
    StudioNotFound(String),
    #[error("Instance group {0} not found in cluster {1}.")]
    InstanceGroupNotFound(String, String),
    #[error("Notebook execution {0} not found.")]
    NotebookExecutionNotFound(String),
    #[error("Session mapping not found.")]
    SessionMappingNotFound,
}

impl EmrState {
    // ---- Cluster / Job Flow operations ----

    #[allow(clippy::too_many_arguments)]
    pub fn run_job_flow(
        &mut self,
        name: &str,
        log_uri: Option<String>,
        release_label: Option<String>,
        service_role: Option<String>,
        job_flow_role: Option<String>,
        auto_scaling_role: Option<String>,
        scale_down_behavior: Option<String>,
        security_configuration: Option<String>,
        step_concurrency_level: Option<i32>,
        termination_protected: bool,
        visible_to_all_users: bool,
        applications: Vec<ApplicationData>,
        tags: HashMap<String, String>,
        steps: Vec<StepData>,
        auto_termination_policy: Option<AutoTerminationPolicyData>,
        managed_scaling_policy: Option<ManagedScalingPolicyData>,
        instance_groups: Vec<InstanceGroupData>,
        instance_fleets: Vec<InstanceFleetData>,
        bootstrap_actions: Vec<BootstrapActionData>,
        account_id: &str,
        region: &str,
    ) -> String {
        // Generate cluster ID in j-XXXXXXXXXXXXXXX format
        let raw_id = Uuid::new_v4()
            .as_simple()
            .to_string()
            .to_uppercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .take(13)
            .collect::<String>();
        let cluster_id = format!("j-{raw_id}");
        let cluster_arn =
            format!("arn:aws:elasticmapreduce:{region}:{account_id}:cluster/{cluster_id}");

        let now = Utc::now();

        // The cluster starts in WAITING state for simplicity (like moto).
        let cluster = ClusterData {
            id: cluster_id.clone(),
            name: name.to_string(),
            status: CLUSTER_STATUS_WAITING.to_string(),
            creation_date_time: now,
            ready_date_time: Some(now),
            end_date_time: None,
            termination_protected,
            visible_to_all_users,
            log_uri,
            release_label,
            applications,
            tags,
            service_role,
            job_flow_role,
            auto_scaling_role,
            scale_down_behavior,
            security_configuration,
            step_concurrency_level,
            auto_termination_policy,
            managed_scaling_policy,
            cluster_arn,
            normalized_instance_hours: Some(0),
            master_public_dns_name: None,
            instance_groups,
            instance_fleets,
            bootstrap_actions,
        };

        self.clusters.insert(cluster_id.clone(), cluster);

        // Add any initial steps
        if !steps.is_empty() {
            self.steps
                .entry(cluster_id.clone())
                .or_default()
                .extend(steps);
        }

        cluster_id
    }

    pub fn describe_cluster(&self, cluster_id: &str) -> Result<&ClusterData, EmrError> {
        self.clusters
            .get(cluster_id)
            .ok_or_else(|| EmrError::ClusterNotFound(cluster_id.to_string()))
    }

    pub fn list_clusters(&self, cluster_states: Option<&[String]>) -> Vec<&ClusterData> {
        self.clusters
            .values()
            .filter(|c| {
                if let Some(states) = cluster_states {
                    if !states.is_empty() {
                        return states.iter().any(|s| s == &c.status);
                    }
                }
                true
            })
            .collect()
    }

    pub fn terminate_job_flows(&mut self, cluster_ids: &[String]) {
        let now = Utc::now();
        for cluster_id in cluster_ids {
            if let Some(cluster) = self.clusters.get_mut(cluster_id) {
                cluster.status = CLUSTER_STATUS_TERMINATED.to_string();
                cluster.end_date_time = Some(now);
            }
        }
    }

    pub fn set_termination_protection(&mut self, cluster_ids: &[String], protected: bool) {
        for cluster_id in cluster_ids {
            if let Some(cluster) = self.clusters.get_mut(cluster_id) {
                cluster.termination_protected = protected;
            }
        }
    }

    pub fn set_visible_to_all_users(&mut self, cluster_ids: &[String], visible: bool) {
        for cluster_id in cluster_ids {
            if let Some(cluster) = self.clusters.get_mut(cluster_id) {
                cluster.visible_to_all_users = visible;
            }
        }
    }

    pub fn modify_cluster(
        &mut self,
        cluster_id: &str,
        step_concurrency_level: Option<i32>,
    ) -> Result<Option<i32>, EmrError> {
        let cluster = self
            .clusters
            .get_mut(cluster_id)
            .ok_or_else(|| EmrError::ClusterNotFound(cluster_id.to_string()))?;
        if let Some(level) = step_concurrency_level {
            cluster.step_concurrency_level = Some(level);
        }
        Ok(cluster.step_concurrency_level)
    }

    // ---- Step operations ----

    pub fn add_job_flow_steps(
        &mut self,
        cluster_id: &str,
        steps: Vec<StepData>,
    ) -> Result<Vec<String>, EmrError> {
        if !self.clusters.contains_key(cluster_id) {
            return Err(EmrError::ClusterNotFound(cluster_id.to_string()));
        }
        let ids: Vec<String> = steps.iter().map(|s| s.id.clone()).collect();
        self.steps
            .entry(cluster_id.to_string())
            .or_default()
            .extend(steps);
        Ok(ids)
    }

    pub fn describe_step(&self, cluster_id: &str, step_id: &str) -> Result<&StepData, EmrError> {
        self.steps
            .get(cluster_id)
            .and_then(|steps| steps.iter().find(|s| s.id == step_id))
            .ok_or_else(|| EmrError::StepNotFound(step_id.to_string(), cluster_id.to_string()))
    }

    pub fn list_steps(
        &self,
        cluster_id: &str,
        step_ids: Option<&[String]>,
        step_states: Option<&[String]>,
    ) -> Result<Vec<&StepData>, EmrError> {
        if !self.clusters.contains_key(cluster_id) {
            return Err(EmrError::ClusterNotFound(cluster_id.to_string()));
        }
        let steps = self.steps.get(cluster_id).map_or(&[] as &[StepData], |v| v);
        let filtered = steps
            .iter()
            .filter(|s| {
                if let Some(ids) = step_ids {
                    if !ids.is_empty() && !ids.iter().any(|id| id == &s.id) {
                        return false;
                    }
                }
                if let Some(states) = step_states {
                    if !states.is_empty() && !states.iter().any(|st| st == &s.status) {
                        return false;
                    }
                }
                true
            })
            .collect();
        Ok(filtered)
    }

    pub fn cancel_steps(
        &mut self,
        cluster_id: &str,
        step_ids: &[String],
    ) -> Result<Vec<(String, &str)>, EmrError> {
        if !self.clusters.contains_key(cluster_id) {
            return Err(EmrError::ClusterNotFound(cluster_id.to_string()));
        }
        let steps = self.steps.entry(cluster_id.to_string()).or_default();
        let mut results = Vec::new();
        let now = Utc::now();
        for step in steps.iter_mut() {
            if step_ids.iter().any(|id| id == &step.id) {
                if step.status == STEP_STATUS_PENDING || step.status == STEP_STATUS_RUNNING {
                    step.status = STEP_STATUS_CANCELLED.to_string();
                    step.end_date_time = Some(now);
                    results.push((step.id.clone(), "SUCCEEDED"));
                } else {
                    results.push((step.id.clone(), "FAILED"));
                }
            }
        }
        Ok(results)
    }

    // ---- Instance Group operations ----

    pub fn add_instance_groups(
        &mut self,
        cluster_id: &str,
        groups: Vec<InstanceGroupData>,
    ) -> Result<Vec<String>, EmrError> {
        let cluster = self
            .clusters
            .get_mut(cluster_id)
            .ok_or_else(|| EmrError::ClusterNotFound(cluster_id.to_string()))?;
        let ids: Vec<String> = groups.iter().map(|g| g.id.clone()).collect();
        cluster.instance_groups.extend(groups);
        Ok(ids)
    }

    pub fn list_instance_groups(&self, cluster_id: &str) -> Result<&[InstanceGroupData], EmrError> {
        let cluster = self
            .clusters
            .get(cluster_id)
            .ok_or_else(|| EmrError::ClusterNotFound(cluster_id.to_string()))?;
        Ok(&cluster.instance_groups)
    }

    pub fn modify_instance_groups(
        &mut self,
        cluster_id: &str,
        modifications: &[(String, Option<i32>)],
    ) -> Result<(), EmrError> {
        let cluster = self
            .clusters
            .get_mut(cluster_id)
            .ok_or_else(|| EmrError::ClusterNotFound(cluster_id.to_string()))?;
        for (ig_id, count) in modifications {
            if let Some(ig) = cluster.instance_groups.iter_mut().find(|g| &g.id == ig_id) {
                if let Some(c) = count {
                    ig.instance_count = *c;
                }
            }
        }
        Ok(())
    }

    // ---- Instance Fleet operations ----

    pub fn add_instance_fleet(
        &mut self,
        cluster_id: &str,
        fleet: InstanceFleetData,
    ) -> Result<String, EmrError> {
        let cluster = self
            .clusters
            .get_mut(cluster_id)
            .ok_or_else(|| EmrError::ClusterNotFound(cluster_id.to_string()))?;
        let id = fleet.id.clone();
        cluster.instance_fleets.push(fleet);
        Ok(id)
    }

    pub fn list_instance_fleets(&self, cluster_id: &str) -> Result<&[InstanceFleetData], EmrError> {
        let cluster = self
            .clusters
            .get(cluster_id)
            .ok_or_else(|| EmrError::ClusterNotFound(cluster_id.to_string()))?;
        Ok(&cluster.instance_fleets)
    }

    pub fn modify_instance_fleet(
        &mut self,
        cluster_id: &str,
        fleet_id: &str,
        target_on_demand: Option<i32>,
        target_spot: Option<i32>,
    ) -> Result<(), EmrError> {
        let cluster = self
            .clusters
            .get_mut(cluster_id)
            .ok_or_else(|| EmrError::ClusterNotFound(cluster_id.to_string()))?;
        if let Some(fleet) = cluster
            .instance_fleets
            .iter_mut()
            .find(|f| f.id == fleet_id)
        {
            if let Some(v) = target_on_demand {
                fleet.target_on_demand_capacity = Some(v);
            }
            if let Some(v) = target_spot {
                fleet.target_spot_capacity = Some(v);
            }
        }
        Ok(())
    }

    // ---- Security Configuration operations ----

    pub fn create_security_configuration(
        &mut self,
        name: &str,
        security_configuration: &str,
    ) -> Result<&SecurityConfigurationData, EmrError> {
        if self.security_configurations.contains_key(name) {
            return Err(EmrError::SecurityConfigurationAlreadyExists(
                name.to_string(),
            ));
        }
        let sc = SecurityConfigurationData {
            name: name.to_string(),
            security_configuration: security_configuration.to_string(),
            creation_date_time: Utc::now(),
        };
        self.security_configurations.insert(name.to_string(), sc);
        Ok(self.security_configurations.get(name).unwrap())
    }

    pub fn describe_security_configuration(
        &self,
        name: &str,
    ) -> Result<&SecurityConfigurationData, EmrError> {
        self.security_configurations
            .get(name)
            .ok_or_else(|| EmrError::SecurityConfigurationNotFound(name.to_string()))
    }

    pub fn delete_security_configuration(&mut self, name: &str) -> Result<(), EmrError> {
        if self.security_configurations.remove(name).is_none() {
            return Err(EmrError::SecurityConfigurationNotFound(name.to_string()));
        }
        Ok(())
    }

    pub fn list_security_configurations(&self) -> Vec<&SecurityConfigurationData> {
        self.security_configurations.values().collect()
    }

    // ---- Managed Scaling Policy operations ----

    pub fn put_managed_scaling_policy(
        &mut self,
        cluster_id: &str,
        policy: ManagedScalingPolicyData,
    ) -> Result<(), EmrError> {
        let cluster = self
            .clusters
            .get_mut(cluster_id)
            .ok_or_else(|| EmrError::ClusterNotFound(cluster_id.to_string()))?;
        cluster.managed_scaling_policy = Some(policy);
        Ok(())
    }

    pub fn get_managed_scaling_policy(
        &self,
        cluster_id: &str,
    ) -> Result<Option<&ManagedScalingPolicyData>, EmrError> {
        let cluster = self
            .clusters
            .get(cluster_id)
            .ok_or_else(|| EmrError::ClusterNotFound(cluster_id.to_string()))?;
        Ok(cluster.managed_scaling_policy.as_ref())
    }

    pub fn remove_managed_scaling_policy(&mut self, cluster_id: &str) -> Result<(), EmrError> {
        let cluster = self
            .clusters
            .get_mut(cluster_id)
            .ok_or_else(|| EmrError::ClusterNotFound(cluster_id.to_string()))?;
        cluster.managed_scaling_policy = None;
        Ok(())
    }

    // ---- Auto Termination Policy operations ----

    pub fn put_auto_termination_policy(
        &mut self,
        cluster_id: &str,
        policy: Option<AutoTerminationPolicyData>,
    ) -> Result<(), EmrError> {
        let cluster = self
            .clusters
            .get_mut(cluster_id)
            .ok_or_else(|| EmrError::ClusterNotFound(cluster_id.to_string()))?;
        cluster.auto_termination_policy = policy;
        Ok(())
    }

    pub fn get_auto_termination_policy(
        &self,
        cluster_id: &str,
    ) -> Result<Option<&AutoTerminationPolicyData>, EmrError> {
        let cluster = self
            .clusters
            .get(cluster_id)
            .ok_or_else(|| EmrError::ClusterNotFound(cluster_id.to_string()))?;
        Ok(cluster.auto_termination_policy.as_ref())
    }

    pub fn remove_auto_termination_policy(&mut self, cluster_id: &str) -> Result<(), EmrError> {
        let cluster = self
            .clusters
            .get_mut(cluster_id)
            .ok_or_else(|| EmrError::ClusterNotFound(cluster_id.to_string()))?;
        cluster.auto_termination_policy = None;
        Ok(())
    }

    // ---- Block Public Access Configuration ----

    pub fn put_block_public_access_configuration(
        &mut self,
        block: bool,
        permitted_ranges: Vec<PortRangeData>,
    ) {
        self.block_public_access_config = Some(BlockPublicAccessConfigurationData {
            block_public_security_group_rules: block,
            permitted_ranges,
            creation_date_time: Utc::now(),
            created_by_arn: None,
        });
    }

    pub fn get_block_public_access_configuration(
        &self,
    ) -> Option<&BlockPublicAccessConfigurationData> {
        self.block_public_access_config.as_ref()
    }

    // ---- Tags operations ----

    pub fn add_tags(
        &mut self,
        resource_id: &str,
        new_tags: Vec<(String, String)>,
    ) -> Result<(), EmrError> {
        if let Some(cluster) = self.clusters.get_mut(resource_id) {
            for (k, v) in new_tags {
                cluster.tags.insert(k, v);
            }
            Ok(())
        } else {
            Err(EmrError::ResourceNotFound(resource_id.to_string()))
        }
    }

    pub fn remove_tags(&mut self, resource_id: &str, tag_keys: &[String]) -> Result<(), EmrError> {
        if let Some(cluster) = self.clusters.get_mut(resource_id) {
            for key in tag_keys {
                cluster.tags.remove(key);
            }
            Ok(())
        } else {
            Err(EmrError::ResourceNotFound(resource_id.to_string()))
        }
    }

    // ---- Bootstrap Actions ----

    pub fn list_bootstrap_actions(
        &self,
        cluster_id: &str,
    ) -> Result<&[BootstrapActionData], EmrError> {
        let cluster = self
            .clusters
            .get(cluster_id)
            .ok_or_else(|| EmrError::ClusterNotFound(cluster_id.to_string()))?;
        Ok(&cluster.bootstrap_actions)
    }

    // ---- Auto Scaling Policy operations ----

    pub fn put_auto_scaling_policy(
        &mut self,
        cluster_id: &str,
        instance_group_id: &str,
        policy: AutoScalingPolicyData,
    ) -> Result<(), EmrError> {
        let cluster = self
            .clusters
            .get_mut(cluster_id)
            .ok_or_else(|| EmrError::ClusterNotFound(cluster_id.to_string()))?;
        let ig = cluster
            .instance_groups
            .iter_mut()
            .find(|g| g.id == instance_group_id)
            .ok_or_else(|| {
                EmrError::InstanceGroupNotFound(
                    instance_group_id.to_string(),
                    cluster_id.to_string(),
                )
            })?;
        ig.auto_scaling_policy = Some(policy);
        Ok(())
    }

    pub fn remove_auto_scaling_policy(
        &mut self,
        cluster_id: &str,
        instance_group_id: &str,
    ) -> Result<(), EmrError> {
        let cluster = self
            .clusters
            .get_mut(cluster_id)
            .ok_or_else(|| EmrError::ClusterNotFound(cluster_id.to_string()))?;
        let ig = cluster
            .instance_groups
            .iter_mut()
            .find(|g| g.id == instance_group_id)
            .ok_or_else(|| {
                EmrError::InstanceGroupNotFound(
                    instance_group_id.to_string(),
                    cluster_id.to_string(),
                )
            })?;
        ig.auto_scaling_policy = None;
        Ok(())
    }

    // ---- Studio operations ----

    #[allow(clippy::too_many_arguments)]
    pub fn create_studio(
        &mut self,
        name: &str,
        description: Option<String>,
        auth_mode: &str,
        vpc_id: Option<String>,
        subnet_ids: Vec<String>,
        service_role: Option<String>,
        user_role: Option<String>,
        workspace_security_group_id: Option<String>,
        engine_security_group_id: Option<String>,
        default_s3_location: Option<String>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> String {
        let raw_id = Uuid::new_v4()
            .as_simple()
            .to_string()
            .chars()
            .take(13)
            .collect::<String>();
        let studio_id = format!("es-{raw_id}");
        let studio_arn =
            format!("arn:aws:elasticmapreduce:{region}:{account_id}:studio/{studio_id}");
        let url = format!("https://{studio_id}.emrstudio-prod.{region}.amazonaws.com");
        let now = Utc::now();
        let studio = StudioData {
            studio_id: studio_id.clone(),
            name: name.to_string(),
            description,
            auth_mode: auth_mode.to_string(),
            vpc_id,
            subnet_ids,
            service_role,
            user_role,
            workspace_security_group_id,
            engine_security_group_id,
            studio_arn,
            url,
            creation_time: now,
            default_s3_location,
            tags,
        };
        self.studios.insert(studio_id.clone(), studio);
        studio_id
    }

    pub fn describe_studio(&self, studio_id: &str) -> Result<&StudioData, EmrError> {
        self.studios
            .get(studio_id)
            .ok_or_else(|| EmrError::StudioNotFound(studio_id.to_string()))
    }

    pub fn delete_studio(&mut self, studio_id: &str) -> Result<(), EmrError> {
        self.studios
            .remove(studio_id)
            .ok_or_else(|| EmrError::StudioNotFound(studio_id.to_string()))?;
        // Also remove session mappings for this studio
        self.session_mappings.retain(|m| m.studio_id != studio_id);
        Ok(())
    }

    pub fn update_studio(
        &mut self,
        studio_id: &str,
        name: Option<&str>,
        description: Option<&str>,
        default_s3_location: Option<&str>,
    ) -> Result<(), EmrError> {
        let studio = self
            .studios
            .get_mut(studio_id)
            .ok_or_else(|| EmrError::StudioNotFound(studio_id.to_string()))?;
        if let Some(n) = name {
            studio.name = n.to_string();
        }
        if let Some(d) = description {
            studio.description = Some(d.to_string());
        }
        if let Some(s) = default_s3_location {
            studio.default_s3_location = Some(s.to_string());
        }
        Ok(())
    }

    pub fn list_studios(&self) -> Vec<&StudioData> {
        self.studios.values().collect()
    }

    // ---- Session Mapping operations ----

    pub fn create_studio_session_mapping(
        &mut self,
        studio_id: &str,
        identity_id: &str,
        identity_name: &str,
        identity_type: &str,
        session_policy_arn: &str,
    ) -> Result<(), EmrError> {
        if !self.studios.contains_key(studio_id) {
            return Err(EmrError::StudioNotFound(studio_id.to_string()));
        }
        let now = Utc::now();
        let mapping = SessionMappingData {
            studio_id: studio_id.to_string(),
            identity_id: identity_id.to_string(),
            identity_name: identity_name.to_string(),
            identity_type: identity_type.to_string(),
            session_policy_arn: session_policy_arn.to_string(),
            creation_time: now,
            last_modified_time: now,
        };
        self.session_mappings.push(mapping);
        Ok(())
    }

    pub fn get_studio_session_mapping(
        &self,
        studio_id: &str,
        identity_id: &str,
        identity_type: &str,
    ) -> Result<&SessionMappingData, EmrError> {
        self.session_mappings
            .iter()
            .find(|m| {
                m.studio_id == studio_id
                    && m.identity_id == identity_id
                    && m.identity_type == identity_type
            })
            .ok_or(EmrError::SessionMappingNotFound)
    }

    pub fn update_studio_session_mapping(
        &mut self,
        studio_id: &str,
        identity_id: &str,
        identity_type: &str,
        session_policy_arn: &str,
    ) -> Result<(), EmrError> {
        let mapping = self
            .session_mappings
            .iter_mut()
            .find(|m| {
                m.studio_id == studio_id
                    && m.identity_id == identity_id
                    && m.identity_type == identity_type
            })
            .ok_or(EmrError::SessionMappingNotFound)?;
        mapping.session_policy_arn = session_policy_arn.to_string();
        mapping.last_modified_time = Utc::now();
        Ok(())
    }

    pub fn delete_studio_session_mapping(
        &mut self,
        studio_id: &str,
        identity_id: &str,
        identity_type: &str,
    ) -> Result<(), EmrError> {
        let pos = self.session_mappings.iter().position(|m| {
            m.studio_id == studio_id
                && m.identity_id == identity_id
                && m.identity_type == identity_type
        });
        match pos {
            Some(idx) => {
                self.session_mappings.remove(idx);
                Ok(())
            }
            None => Err(EmrError::SessionMappingNotFound),
        }
    }

    pub fn list_studio_session_mappings(
        &self,
        studio_id: Option<&str>,
    ) -> Vec<&SessionMappingData> {
        self.session_mappings
            .iter()
            .filter(|m| {
                if let Some(sid) = studio_id {
                    m.studio_id == sid
                } else {
                    true
                }
            })
            .collect()
    }

    // ---- Notebook Execution operations ----

    pub fn start_notebook_execution(
        &mut self,
        editor_id: &str,
        execution_engine: serde_json::Value,
        notebook_execution_name: Option<String>,
        notebook_s3_location: Option<serde_json::Value>,
        tags: HashMap<String, String>,
    ) -> String {
        let raw_id = Uuid::new_v4()
            .as_simple()
            .to_string()
            .chars()
            .take(13)
            .collect::<String>();
        let execution_id = format!("ex-{raw_id}");
        let now = Utc::now();
        let exec = NotebookExecutionData {
            notebook_execution_id: execution_id.clone(),
            editor_id: editor_id.to_string(),
            execution_engine,
            notebook_execution_name,
            status: "FINISHED".to_string(),
            start_time: now,
            end_time: Some(now),
            notebook_s3_location,
            tags,
        };
        self.notebook_executions.insert(execution_id.clone(), exec);
        execution_id
    }

    pub fn describe_notebook_execution(
        &self,
        notebook_execution_id: &str,
    ) -> Result<&NotebookExecutionData, EmrError> {
        self.notebook_executions
            .get(notebook_execution_id)
            .ok_or_else(|| EmrError::NotebookExecutionNotFound(notebook_execution_id.to_string()))
    }

    pub fn stop_notebook_execution(&mut self, notebook_execution_id: &str) -> Result<(), EmrError> {
        let exec = self
            .notebook_executions
            .get_mut(notebook_execution_id)
            .ok_or_else(|| {
                EmrError::NotebookExecutionNotFound(notebook_execution_id.to_string())
            })?;
        exec.status = "STOPPED".to_string();
        exec.end_time = Some(Utc::now());
        Ok(())
    }

    pub fn list_notebook_executions(&self) -> Vec<&NotebookExecutionData> {
        self.notebook_executions.values().collect()
    }

    // ---- Persistent App UI operations ----

    pub fn create_persistent_app_ui(&mut self) -> String {
        let raw_id = Uuid::new_v4()
            .as_simple()
            .to_string()
            .chars()
            .take(13)
            .collect::<String>();
        let ui_id = format!("pau-{raw_id}");
        let now = Utc::now();
        let ui = PersistentAppUiData {
            persistent_app_ui_id: ui_id.clone(),
            status: "ENABLED".to_string(),
            creation_time: now,
        };
        self.persistent_app_uis.insert(ui_id.clone(), ui);
        ui_id
    }

    pub fn describe_persistent_app_ui(
        &self,
        persistent_app_ui_id: &str,
    ) -> Result<&PersistentAppUiData, EmrError> {
        self.persistent_app_uis
            .get(persistent_app_ui_id)
            .ok_or_else(|| EmrError::ResourceNotFound(persistent_app_ui_id.to_string()))
    }
}
