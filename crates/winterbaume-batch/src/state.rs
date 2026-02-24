use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct BatchState {
    pub job_queues: HashMap<String, JobQueue>,
    pub job_definitions: HashMap<String, Vec<JobDefinition>>,
    pub compute_environments: HashMap<String, ComputeEnvironment>,
    pub scheduling_policies: HashMap<String, SchedulingPolicy>,
    pub jobs: HashMap<String, Job>,
    pub consumable_resources: HashMap<String, ConsumableResource>,
    pub service_environments: HashMap<String, ServiceEnvironment>,
    pub service_jobs: HashMap<String, ServiceJob>,
    next_revision: HashMap<String, i32>,
}

#[derive(Debug, thiserror::Error)]
pub enum BatchError {
    #[error("Job queue {0} already exists")]
    JobQueueAlreadyExists(String),
    #[error("Job queue {0} does not exist")]
    JobQueueNotFound(String),
    #[error("Job definition {0} does not exist")]
    JobDefinitionNotFound(String),
    #[error("Compute environment {0} already exists")]
    ComputeEnvironmentAlreadyExists(String),
    #[error("Compute environment {0} does not exist")]
    ComputeEnvironmentNotFound(String),
    #[error("Scheduling policy {0} already exists")]
    SchedulingPolicyAlreadyExists(String),
    #[error("Scheduling policy {0} does not exist")]
    SchedulingPolicyNotFound(String),
    #[error("Job {0} does not exist")]
    JobNotFound(String),
    #[error("Resource {0} not found")]
    ResourceNotFound(String),
    #[error("Consumable resource {0} already exists")]
    ConsumableResourceAlreadyExists(String),
    #[error("Consumable resource {0} does not exist")]
    ConsumableResourceNotFound(String),
    #[error("Service environment {0} already exists")]
    ServiceEnvironmentAlreadyExists(String),
    #[error("Service environment {0} does not exist")]
    ServiceEnvironmentNotFound(String),
    #[error("Service job {0} does not exist")]
    ServiceJobNotFound(String),
}

impl BatchState {
    pub fn create_job_queue(
        &mut self,
        name: &str,
        state: &str,
        priority: i32,
        compute_environment_order: Vec<ComputeEnvironmentOrder>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
        scheduling_policy_arn: Option<String>,
    ) -> Result<&JobQueue, BatchError> {
        if self.job_queues.contains_key(name) {
            return Err(BatchError::JobQueueAlreadyExists(name.to_string()));
        }

        let arn = format!("arn:aws:batch:{region}:{account_id}:job-queue/{name}");

        let jq = JobQueue {
            job_queue_name: name.to_string(),
            job_queue_arn: arn,
            state: state.to_string(),
            status: "VALID".to_string(),
            status_reason: "JobQueue Healthy".to_string(),
            priority,
            compute_environment_order,
            tags,
            created_at: Utc::now(),
            scheduling_policy_arn,
        };

        self.job_queues.insert(name.to_string(), jq);
        Ok(self.job_queues.get(name).unwrap())
    }

    pub fn describe_job_queues(&self, names: &[&str]) -> Vec<&JobQueue> {
        if names.is_empty() {
            self.job_queues.values().collect()
        } else {
            names
                .iter()
                .filter_map(|n| {
                    self.job_queues.get(*n).or_else(|| {
                        // Also try matching by ARN
                        self.job_queues.values().find(|jq| jq.job_queue_arn == *n)
                    })
                })
                .collect()
        }
    }

    pub fn delete_job_queue(&mut self, name_or_arn: &str) -> Result<(), BatchError> {
        // Try by name first
        if self.job_queues.remove(name_or_arn).is_some() {
            return Ok(());
        }
        // Try by ARN
        let key = self
            .job_queues
            .iter()
            .find(|(_, jq)| jq.job_queue_arn == name_or_arn)
            .map(|(k, _)| k.clone());
        match key {
            Some(k) => {
                self.job_queues.remove(&k);
                Ok(())
            }
            None => Err(BatchError::JobQueueNotFound(name_or_arn.to_string())),
        }
    }

    pub fn update_job_queue(
        &mut self,
        name_or_arn: &str,
        state: Option<&str>,
        priority: Option<i32>,
        compute_environment_order: Option<Vec<ComputeEnvironmentOrder>>,
    ) -> Result<&JobQueue, BatchError> {
        let key = self.find_job_queue_key(name_or_arn)?;
        let jq = self.job_queues.get_mut(&key).unwrap();
        if let Some(s) = state {
            jq.state = s.to_string();
        }
        if let Some(p) = priority {
            jq.priority = p;
        }
        if let Some(ceo) = compute_environment_order {
            jq.compute_environment_order = ceo;
        }
        Ok(self.job_queues.get(&key).unwrap())
    }

    fn find_job_queue_key(&self, name_or_arn: &str) -> Result<String, BatchError> {
        if self.job_queues.contains_key(name_or_arn) {
            return Ok(name_or_arn.to_string());
        }
        self.job_queues
            .iter()
            .find(|(_, jq)| jq.job_queue_arn == name_or_arn)
            .map(|(k, _)| k.clone())
            .ok_or(BatchError::JobQueueNotFound(name_or_arn.to_string()))
    }

    pub fn register_job_definition(
        &mut self,
        name: &str,
        job_type: &str,
        container_properties: Option<ContainerProperties>,
        account_id: &str,
        region: &str,
    ) -> Result<&JobDefinition, BatchError> {
        let revision = self.next_revision.entry(name.to_string()).or_insert(0);
        *revision += 1;
        let rev = *revision;

        let arn = format!("arn:aws:batch:{region}:{account_id}:job-definition/{name}:{rev}");

        let jd = JobDefinition {
            job_definition_name: name.to_string(),
            job_definition_arn: arn,
            revision: rev,
            status: "ACTIVE".to_string(),
            job_definition_type: job_type.to_string(),
            container_properties,
            tags: HashMap::new(),
            created_at: Utc::now(),
        };

        let defs = self.job_definitions.entry(name.to_string()).or_default();
        defs.push(jd);
        Ok(defs.last().unwrap())
    }

    pub fn describe_job_definitions(
        &self,
        names: &[&str],
        job_definition_name: Option<&str>,
        status: Option<&str>,
    ) -> Vec<&JobDefinition> {
        let mut results: Vec<&JobDefinition> = Vec::new();

        if !names.is_empty() {
            // Filter by specific names/ARNs
            for name in names {
                for defs in self.job_definitions.values() {
                    for jd in defs {
                        if jd.job_definition_name == *name
                            || jd.job_definition_arn == *name
                            || format!("{}:{}", jd.job_definition_name, jd.revision) == *name
                        {
                            results.push(jd);
                        }
                    }
                }
            }
        } else if let Some(jdn) = job_definition_name {
            if let Some(defs) = self.job_definitions.get(jdn) {
                results.extend(defs.iter());
            }
        } else {
            for defs in self.job_definitions.values() {
                results.extend(defs.iter());
            }
        }

        if let Some(s) = status {
            results.retain(|jd| jd.status == s);
        }

        results
    }

    pub fn deregister_job_definition(&mut self, name_or_arn: &str) -> Result<(), BatchError> {
        for defs in self.job_definitions.values_mut() {
            for jd in defs.iter_mut() {
                if jd.job_definition_arn == name_or_arn
                    || jd.job_definition_name == name_or_arn
                    || format!("{}:{}", jd.job_definition_name, jd.revision) == name_or_arn
                {
                    jd.status = "INACTIVE".to_string();
                    return Ok(());
                }
            }
        }
        Err(BatchError::JobDefinitionNotFound(name_or_arn.to_string()))
    }

    // --- Compute Environment operations ---

    pub fn create_compute_environment(
        &mut self,
        name: &str,
        ce_type: &str,
        state: Option<&str>,
        service_role: Option<&str>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&ComputeEnvironment, BatchError> {
        if self.compute_environments.contains_key(name) {
            return Err(BatchError::ComputeEnvironmentAlreadyExists(
                name.to_string(),
            ));
        }

        let arn = format!("arn:aws:batch:{region}:{account_id}:compute-environment/{name}");

        let ce = ComputeEnvironment {
            compute_environment_name: name.to_string(),
            compute_environment_arn: arn,
            ce_type: ce_type.to_string(),
            state: state.unwrap_or("ENABLED").to_string(),
            status: "VALID".to_string(),
            status_reason: "ComputeEnvironment Healthy".to_string(),
            service_role: service_role.map(String::from),
            tags,
            created_at: Utc::now(),
        };

        self.compute_environments.insert(name.to_string(), ce);
        Ok(self.compute_environments.get(name).unwrap())
    }

    pub fn describe_compute_environments(&self, names: &[&str]) -> Vec<&ComputeEnvironment> {
        if names.is_empty() {
            self.compute_environments.values().collect()
        } else {
            names
                .iter()
                .filter_map(|n| {
                    self.compute_environments.get(*n).or_else(|| {
                        self.compute_environments
                            .values()
                            .find(|ce| ce.compute_environment_arn == *n)
                    })
                })
                .collect()
        }
    }

    pub fn delete_compute_environment(&mut self, name_or_arn: &str) -> Result<(), BatchError> {
        if self.compute_environments.remove(name_or_arn).is_some() {
            return Ok(());
        }
        let key = self
            .compute_environments
            .iter()
            .find(|(_, ce)| ce.compute_environment_arn == name_or_arn)
            .map(|(k, _)| k.clone());
        match key {
            Some(k) => {
                self.compute_environments.remove(&k);
                Ok(())
            }
            None => Err(BatchError::ComputeEnvironmentNotFound(
                name_or_arn.to_string(),
            )),
        }
    }

    pub fn update_compute_environment(
        &mut self,
        name_or_arn: &str,
        state: Option<&str>,
        service_role: Option<&str>,
    ) -> Result<&ComputeEnvironment, BatchError> {
        let key = self.find_compute_environment_key(name_or_arn)?;
        let ce = self.compute_environments.get_mut(&key).unwrap();
        if let Some(s) = state {
            ce.state = s.to_string();
        }
        if let Some(sr) = service_role {
            ce.service_role = Some(sr.to_string());
        }
        Ok(self.compute_environments.get(&key).unwrap())
    }

    fn find_compute_environment_key(&self, name_or_arn: &str) -> Result<String, BatchError> {
        if self.compute_environments.contains_key(name_or_arn) {
            return Ok(name_or_arn.to_string());
        }
        self.compute_environments
            .iter()
            .find(|(_, ce)| ce.compute_environment_arn == name_or_arn)
            .map(|(k, _)| k.clone())
            .ok_or(BatchError::ComputeEnvironmentNotFound(
                name_or_arn.to_string(),
            ))
    }

    // --- Scheduling Policy operations ---

    pub fn create_scheduling_policy(
        &mut self,
        name: &str,
        fairshare_policy: Option<FairsharePolicy>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&SchedulingPolicy, BatchError> {
        if self.scheduling_policies.contains_key(name) {
            return Err(BatchError::SchedulingPolicyAlreadyExists(name.to_string()));
        }

        let arn = format!("arn:aws:batch:{region}:{account_id}:scheduling-policy/{name}");

        let sp = SchedulingPolicy {
            name: name.to_string(),
            arn,
            fairshare_policy,
            tags,
        };

        self.scheduling_policies.insert(name.to_string(), sp);
        Ok(self.scheduling_policies.get(name).unwrap())
    }

    pub fn describe_scheduling_policies(&self, arns: &[&str]) -> Vec<&SchedulingPolicy> {
        arns.iter()
            .filter_map(|arn| {
                self.scheduling_policies
                    .values()
                    .find(|sp| sp.arn == *arn || sp.name == *arn)
            })
            .collect()
    }

    pub fn list_scheduling_policies(&self) -> Vec<&SchedulingPolicy> {
        self.scheduling_policies.values().collect()
    }

    pub fn delete_scheduling_policy(&mut self, arn: &str) -> Result<(), BatchError> {
        let key = self
            .scheduling_policies
            .iter()
            .find(|(_, sp)| sp.arn == arn || sp.name == arn)
            .map(|(k, _)| k.clone());
        match key {
            Some(k) => {
                self.scheduling_policies.remove(&k);
                Ok(())
            }
            None => Err(BatchError::SchedulingPolicyNotFound(arn.to_string())),
        }
    }

    pub fn update_scheduling_policy(
        &mut self,
        arn: &str,
        fairshare_policy: Option<FairsharePolicy>,
    ) -> Result<(), BatchError> {
        let key = self
            .scheduling_policies
            .iter()
            .find(|(_, sp)| sp.arn == arn || sp.name == arn)
            .map(|(k, _)| k.clone());
        match key {
            Some(k) => {
                let sp = self.scheduling_policies.get_mut(&k).unwrap();
                if let Some(fp) = fairshare_policy {
                    sp.fairshare_policy = Some(fp);
                }
                Ok(())
            }
            None => Err(BatchError::SchedulingPolicyNotFound(arn.to_string())),
        }
    }

    // --- Job operations ---

    pub fn submit_job(
        &mut self,
        job_name: &str,
        job_queue: &str,
        job_definition: &str,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&Job, BatchError> {
        let job_id = uuid::Uuid::new_v4().to_string();
        let job_arn = format!("arn:aws:batch:{region}:{account_id}:job/{job_id}");
        let now = Utc::now().timestamp();

        let job = Job {
            job_id: job_id.clone(),
            job_name: job_name.to_string(),
            job_arn,
            job_queue: job_queue.to_string(),
            job_definition: job_definition.to_string(),
            status: "SUBMITTED".to_string(),
            status_reason: None,
            created_at: now,
            started_at: 0,
            tags,
        };

        self.jobs.insert(job_id.clone(), job);
        Ok(self.jobs.get(&job_id).unwrap())
    }

    pub fn describe_jobs(&self, job_ids: &[&str]) -> Vec<&Job> {
        job_ids.iter().filter_map(|id| self.jobs.get(*id)).collect()
    }

    pub fn list_jobs(&self, job_queue: Option<&str>, job_status: Option<&str>) -> Vec<&Job> {
        self.jobs
            .values()
            .filter(|j| {
                if let Some(q) = job_queue {
                    if j.job_queue != q && !j.job_queue.ends_with(&format!("/{q}")) {
                        return false;
                    }
                }
                if let Some(s) = job_status {
                    if j.status != s {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    pub fn cancel_job(&mut self, job_id: &str, reason: &str) -> Result<(), BatchError> {
        match self.jobs.get_mut(job_id) {
            Some(job) => {
                if job.status == "SUBMITTED" || job.status == "PENDING" || job.status == "RUNNABLE"
                {
                    job.status = "FAILED".to_string();
                    job.status_reason = Some(reason.to_string());
                }
                Ok(())
            }
            None => Err(BatchError::JobNotFound(job_id.to_string())),
        }
    }

    pub fn terminate_job(&mut self, job_id: &str, reason: &str) -> Result<(), BatchError> {
        match self.jobs.get_mut(job_id) {
            Some(job) => {
                job.status = "FAILED".to_string();
                job.status_reason = Some(reason.to_string());
                Ok(())
            }
            None => Err(BatchError::JobNotFound(job_id.to_string())),
        }
    }

    // --- Tag operations ---

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), BatchError> {
        // Try each resource type
        for jq in self.job_queues.values_mut() {
            if jq.job_queue_arn == resource_arn {
                jq.tags.extend(tags);
                return Ok(());
            }
        }
        for defs in self.job_definitions.values_mut() {
            for jd in defs.iter_mut() {
                if jd.job_definition_arn == resource_arn {
                    jd.tags.extend(tags);
                    return Ok(());
                }
            }
        }
        for ce in self.compute_environments.values_mut() {
            if ce.compute_environment_arn == resource_arn {
                ce.tags.extend(tags);
                return Ok(());
            }
        }
        for sp in self.scheduling_policies.values_mut() {
            if sp.arn == resource_arn {
                sp.tags.extend(tags);
                return Ok(());
            }
        }
        for job in self.jobs.values_mut() {
            if job.job_arn == resource_arn {
                job.tags.extend(tags);
                return Ok(());
            }
        }
        Err(BatchError::ResourceNotFound(resource_arn.to_string()))
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[&str],
    ) -> Result<(), BatchError> {
        for jq in self.job_queues.values_mut() {
            if jq.job_queue_arn == resource_arn {
                for key in tag_keys {
                    jq.tags.remove(*key);
                }
                return Ok(());
            }
        }
        for defs in self.job_definitions.values_mut() {
            for jd in defs.iter_mut() {
                if jd.job_definition_arn == resource_arn {
                    for key in tag_keys {
                        jd.tags.remove(*key);
                    }
                    return Ok(());
                }
            }
        }
        for ce in self.compute_environments.values_mut() {
            if ce.compute_environment_arn == resource_arn {
                for key in tag_keys {
                    ce.tags.remove(*key);
                }
                return Ok(());
            }
        }
        for sp in self.scheduling_policies.values_mut() {
            if sp.arn == resource_arn {
                for key in tag_keys {
                    sp.tags.remove(*key);
                }
                return Ok(());
            }
        }
        for job in self.jobs.values_mut() {
            if job.job_arn == resource_arn {
                for key in tag_keys {
                    job.tags.remove(*key);
                }
                return Ok(());
            }
        }
        Err(BatchError::ResourceNotFound(resource_arn.to_string()))
    }

    // --- ConsumableResource operations ---

    pub fn create_consumable_resource(
        &mut self,
        name: &str,
        total_quantity: i64,
        resource_type: Option<String>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&ConsumableResource, BatchError> {
        if self.consumable_resources.contains_key(name) {
            return Err(BatchError::ConsumableResourceAlreadyExists(
                name.to_string(),
            ));
        }
        let arn = format!("arn:aws:batch:{region}:{account_id}:consumable-resource/{name}");
        let now = Utc::now().timestamp();
        let cr = ConsumableResource {
            consumable_resource_name: name.to_string(),
            consumable_resource_arn: arn,
            total_quantity,
            in_use_quantity: 0,
            resource_type,
            created_at: now,
            tags,
        };
        self.consumable_resources.insert(name.to_string(), cr);
        Ok(self.consumable_resources.get(name).unwrap())
    }

    pub fn delete_consumable_resource(&mut self, name_or_arn: &str) -> Result<(), BatchError> {
        if self.consumable_resources.remove(name_or_arn).is_some() {
            return Ok(());
        }
        let key = self
            .consumable_resources
            .iter()
            .find(|(_, cr)| cr.consumable_resource_arn == name_or_arn)
            .map(|(k, _)| k.clone());
        match key {
            Some(k) => {
                self.consumable_resources.remove(&k);
                Ok(())
            }
            None => Err(BatchError::ConsumableResourceNotFound(
                name_or_arn.to_string(),
            )),
        }
    }

    pub fn describe_consumable_resource(
        &self,
        name_or_arn: &str,
    ) -> Result<&ConsumableResource, BatchError> {
        if let Some(cr) = self.consumable_resources.get(name_or_arn) {
            return Ok(cr);
        }
        self.consumable_resources
            .values()
            .find(|cr| cr.consumable_resource_arn == name_or_arn)
            .ok_or(BatchError::ConsumableResourceNotFound(
                name_or_arn.to_string(),
            ))
    }

    pub fn update_consumable_resource(
        &mut self,
        name_or_arn: &str,
        quantity: Option<i64>,
    ) -> Result<&ConsumableResource, BatchError> {
        let key = if self.consumable_resources.contains_key(name_or_arn) {
            name_or_arn.to_string()
        } else {
            self.consumable_resources
                .iter()
                .find(|(_, cr)| cr.consumable_resource_arn == name_or_arn)
                .map(|(k, _)| k.clone())
                .ok_or(BatchError::ConsumableResourceNotFound(
                    name_or_arn.to_string(),
                ))?
        };
        let cr = self.consumable_resources.get_mut(&key).unwrap();
        if let Some(q) = quantity {
            cr.total_quantity = q;
        }
        Ok(self.consumable_resources.get(&key).unwrap())
    }

    pub fn list_consumable_resources(&self, name_filter: Option<&str>) -> Vec<&ConsumableResource> {
        self.consumable_resources
            .values()
            .filter(|cr| {
                if let Some(filter) = name_filter {
                    cr.consumable_resource_name.contains(filter)
                } else {
                    true
                }
            })
            .collect()
    }

    // --- ServiceEnvironment operations ---

    pub fn create_service_environment(
        &mut self,
        name: &str,
        se_type: &str,
        state: Option<&str>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&ServiceEnvironment, BatchError> {
        if self.service_environments.contains_key(name) {
            return Err(BatchError::ServiceEnvironmentAlreadyExists(
                name.to_string(),
            ));
        }
        let arn = format!("arn:aws:batch:{region}:{account_id}:service-environment/{name}");
        let now = Utc::now().timestamp();
        let se = ServiceEnvironment {
            service_environment_name: name.to_string(),
            service_environment_arn: arn,
            service_environment_type: se_type.to_string(),
            state: state.unwrap_or("ENABLED").to_string(),
            status: "VALID".to_string(),
            tags,
            created_at: now,
        };
        self.service_environments.insert(name.to_string(), se);
        Ok(self.service_environments.get(name).unwrap())
    }

    pub fn delete_service_environment(&mut self, name_or_arn: &str) -> Result<(), BatchError> {
        if self.service_environments.remove(name_or_arn).is_some() {
            return Ok(());
        }
        let key = self
            .service_environments
            .iter()
            .find(|(_, se)| se.service_environment_arn == name_or_arn)
            .map(|(k, _)| k.clone());
        match key {
            Some(k) => {
                self.service_environments.remove(&k);
                Ok(())
            }
            None => Err(BatchError::ServiceEnvironmentNotFound(
                name_or_arn.to_string(),
            )),
        }
    }

    pub fn describe_service_environments(&self, names: &[&str]) -> Vec<&ServiceEnvironment> {
        if names.is_empty() {
            self.service_environments.values().collect()
        } else {
            names
                .iter()
                .filter_map(|n| {
                    self.service_environments.get(*n).or_else(|| {
                        self.service_environments
                            .values()
                            .find(|se| se.service_environment_arn == *n)
                    })
                })
                .collect()
        }
    }

    pub fn update_service_environment(
        &mut self,
        name_or_arn: &str,
        state: Option<&str>,
    ) -> Result<&ServiceEnvironment, BatchError> {
        let key = if self.service_environments.contains_key(name_or_arn) {
            name_or_arn.to_string()
        } else {
            self.service_environments
                .iter()
                .find(|(_, se)| se.service_environment_arn == name_or_arn)
                .map(|(k, _)| k.clone())
                .ok_or(BatchError::ServiceEnvironmentNotFound(
                    name_or_arn.to_string(),
                ))?
        };
        let se = self.service_environments.get_mut(&key).unwrap();
        if let Some(s) = state {
            se.state = s.to_string();
        }
        Ok(self.service_environments.get(&key).unwrap())
    }

    // --- ServiceJob operations ---

    pub fn submit_service_job(
        &mut self,
        job_name: &str,
        job_queue: &str,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&ServiceJob, BatchError> {
        let job_id = uuid::Uuid::new_v4().to_string();
        let job_arn = format!("arn:aws:batch:{region}:{account_id}:job/{job_id}");
        let now = Utc::now().timestamp();
        let sj = ServiceJob {
            job_id: job_id.clone(),
            job_arn,
            job_name: job_name.to_string(),
            job_queue: job_queue.to_string(),
            status: "SUBMITTED".to_string(),
            created_at: now,
            started_at: None,
            stopped_at: None,
            is_terminated: false,
            tags,
        };
        self.service_jobs.insert(job_id.clone(), sj);
        Ok(self.service_jobs.get(&job_id).unwrap())
    }

    pub fn describe_service_job(&self, job_id: &str) -> Result<&ServiceJob, BatchError> {
        self.service_jobs
            .get(job_id)
            .ok_or(BatchError::ServiceJobNotFound(job_id.to_string()))
    }

    pub fn terminate_service_job(&mut self, job_id: &str) -> Result<(), BatchError> {
        match self.service_jobs.get_mut(job_id) {
            Some(sj) => {
                sj.status = "FAILED".to_string();
                sj.is_terminated = true;
                sj.stopped_at = Some(Utc::now().timestamp());
                Ok(())
            }
            None => Err(BatchError::ServiceJobNotFound(job_id.to_string())),
        }
    }

    pub fn list_service_jobs(&self, job_queue: Option<&str>) -> Vec<&ServiceJob> {
        self.service_jobs
            .values()
            .filter(|sj| {
                if let Some(q) = job_queue {
                    sj.job_queue == q || sj.job_queue.ends_with(&format!("/{q}"))
                } else {
                    true
                }
            })
            .collect()
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<HashMap<String, String>, BatchError> {
        for jq in self.job_queues.values() {
            if jq.job_queue_arn == resource_arn {
                return Ok(jq.tags.clone());
            }
        }
        for defs in self.job_definitions.values() {
            for jd in defs {
                if jd.job_definition_arn == resource_arn {
                    return Ok(jd.tags.clone());
                }
            }
        }
        for ce in self.compute_environments.values() {
            if ce.compute_environment_arn == resource_arn {
                return Ok(ce.tags.clone());
            }
        }
        for sp in self.scheduling_policies.values() {
            if sp.arn == resource_arn {
                return Ok(sp.tags.clone());
            }
        }
        for job in self.jobs.values() {
            if job.job_arn == resource_arn {
                return Ok(job.tags.clone());
            }
        }
        Err(BatchError::ResourceNotFound(resource_arn.to_string()))
    }
}
