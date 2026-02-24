use std::collections::HashMap;

use chrono::Utc;
use uuid::Uuid;

use crate::types::{
    Certificate, Connection, Endpoint, EventSubscription, ReplicationInstance,
    ReplicationSubnetGroup, ReplicationTask,
};

#[derive(Debug, Default)]
pub struct DmsState {
    pub replication_instances: HashMap<String, ReplicationInstance>,
    pub endpoints: HashMap<String, Endpoint>,
    pub replication_tasks: HashMap<String, ReplicationTask>,
    pub connections: HashMap<String, Connection>,
    pub replication_subnet_groups: HashMap<String, ReplicationSubnetGroup>,
    pub certificates: HashMap<String, Certificate>,
    pub event_subscriptions: HashMap<String, EventSubscription>,
    /// Resource ARN -> tags
    pub tags: HashMap<String, HashMap<String, String>>,
    /// Account quotas.
    pub account_quotas: Vec<crate::model::AccountQuota>,
    /// Event category groups.
    pub event_category_groups: Vec<crate::model::EventCategoryGroup>,
    /// Orderable replication instance types.
    pub orderable_replication_instances: Vec<crate::model::OrderableReplicationInstance>,
    /// Engine version catalogue.
    pub engine_versions: Vec<crate::model::EngineVersion>,
    /// Supported endpoint types.
    pub supported_endpoint_types: Vec<crate::model::SupportedEndpointType>,
    /// Endpoint settings.
    pub endpoint_settings: Vec<crate::model::EndpointSetting>,
    /// DMS events (operational log).
    pub events: Vec<crate::model::Event>,
    /// Pending maintenance actions.
    pub pending_maintenance_actions: Vec<crate::model::ResourcePendingMaintenanceActions>,
    /// Individual assessment names.
    pub individual_assessment_names: Vec<String>,
    /// Replication task assessment results.
    pub replication_task_assessment_results: Vec<crate::model::ReplicationTaskAssessmentResult>,
    /// Replication task assessment runs.
    pub replication_task_assessment_runs: Vec<crate::model::ReplicationTaskAssessmentRun>,
    /// Replication task individual assessments.
    pub replication_task_individual_assessments:
        Vec<crate::model::ReplicationTaskIndividualAssessment>,
}

#[derive(Debug, thiserror::Error)]
pub enum DmsError {
    #[error("Replication instance '{0}' already exists.")]
    ReplicationInstanceAlreadyExists(String),
    #[error("Replication instance '{0}' not found.")]
    ReplicationInstanceNotFound(String),
    #[error("Endpoint '{0}' already exists.")]
    EndpointAlreadyExists(String),
    #[error("Endpoint '{0}' not found.")]
    EndpointNotFound(String),
    #[error("Replication task '{0}' already exists.")]
    ReplicationTaskAlreadyExists(String),
    #[error("Replication task '{0}' not found.")]
    ReplicationTaskNotFound(String),
    #[error("Replication subnet group '{0}' already exists.")]
    ReplicationSubnetGroupAlreadyExists(String),
    #[error("Replication subnet group '{0}' not found.")]
    ReplicationSubnetGroupNotFound(String),
    #[error("Certificate '{0}' already exists.")]
    CertificateAlreadyExists(String),
    #[error("Certificate '{0}' not found.")]
    CertificateNotFound(String),
    #[error("Event subscription '{0}' already exists.")]
    EventSubscriptionAlreadyExists(String),
    #[error("Event subscription '{0}' not found.")]
    EventSubscriptionNotFound(String),
}

impl DmsState {
    // ---- ReplicationInstance ----

    #[allow(clippy::too_many_arguments)]
    pub fn create_replication_instance(
        &mut self,
        identifier: &str,
        instance_class: &str,
        allocated_storage: Option<i32>,
        availability_zone: Option<&str>,
        publicly_accessible: bool,
        multi_az: bool,
        engine_version: Option<&str>,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&ReplicationInstance, DmsError> {
        if self.replication_instances.contains_key(identifier) {
            return Err(DmsError::ReplicationInstanceAlreadyExists(
                identifier.to_string(),
            ));
        }
        let arn = format!("arn:aws:dms:{region}:{account_id}:rep:{identifier}");
        let now = Utc::now().timestamp() as f64;
        if !tags.is_empty() {
            self.tags.insert(arn.clone(), tags.clone());
        }
        let instance = ReplicationInstance {
            replication_instance_identifier: identifier.to_string(),
            replication_instance_class: instance_class.to_string(),
            allocated_storage: allocated_storage.unwrap_or(50),
            status: "available".to_string(),
            replication_instance_arn: arn.clone(),
            availability_zone: availability_zone.map(|s| s.to_string()),
            publicly_accessible,
            multi_az,
            engine_version: engine_version.map(|s| s.to_string()),
            instance_create_time: now,
            tags,
        };
        self.replication_instances
            .insert(identifier.to_string(), instance);
        Ok(self.replication_instances.get(identifier).unwrap())
    }

    pub fn describe_replication_instances(&self) -> Vec<&ReplicationInstance> {
        self.replication_instances.values().collect()
    }

    pub fn get_replication_instance(
        &self,
        identifier: &str,
    ) -> Result<&ReplicationInstance, DmsError> {
        self.replication_instances
            .get(identifier)
            .ok_or_else(|| DmsError::ReplicationInstanceNotFound(identifier.to_string()))
    }

    pub fn delete_replication_instance(
        &mut self,
        identifier: &str,
    ) -> Result<ReplicationInstance, DmsError> {
        self.replication_instances
            .remove(identifier)
            .ok_or_else(|| DmsError::ReplicationInstanceNotFound(identifier.to_string()))
    }

    pub fn modify_replication_instance(
        &mut self,
        identifier: &str,
        instance_class: Option<&str>,
        allocated_storage: Option<i32>,
        multi_az: Option<bool>,
    ) -> Result<&ReplicationInstance, DmsError> {
        let instance = self
            .replication_instances
            .get_mut(identifier)
            .ok_or_else(|| DmsError::ReplicationInstanceNotFound(identifier.to_string()))?;
        if let Some(class) = instance_class {
            instance.replication_instance_class = class.to_string();
        }
        if let Some(storage) = allocated_storage {
            instance.allocated_storage = storage;
        }
        if let Some(maz) = multi_az {
            instance.multi_az = maz;
        }
        Ok(self.replication_instances.get(identifier).unwrap())
    }

    // ---- Endpoint ----

    #[allow(clippy::too_many_arguments)]
    pub fn create_endpoint(
        &mut self,
        identifier: &str,
        endpoint_type: &str,
        engine_name: &str,
        username: Option<&str>,
        server_name: Option<&str>,
        port: Option<i32>,
        database_name: Option<&str>,
        extra_connection_attributes: Option<&str>,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&Endpoint, DmsError> {
        if self.endpoints.contains_key(identifier) {
            return Err(DmsError::EndpointAlreadyExists(identifier.to_string()));
        }
        let arn = format!(
            "arn:aws:dms:{region}:{account_id}:endpoint:{}",
            Uuid::new_v4()
        );
        if !tags.is_empty() {
            self.tags.insert(arn.clone(), tags.clone());
        }
        let endpoint = Endpoint {
            endpoint_identifier: identifier.to_string(),
            endpoint_type: endpoint_type.to_string(),
            engine_name: engine_name.to_string(),
            username: username.map(|s| s.to_string()),
            server_name: server_name.map(|s| s.to_string()),
            port,
            database_name: database_name.map(|s| s.to_string()),
            status: "active".to_string(),
            endpoint_arn: arn.clone(),
            extra_connection_attributes: extra_connection_attributes.map(|s| s.to_string()),
            tags,
        };
        self.endpoints.insert(identifier.to_string(), endpoint);
        Ok(self.endpoints.get(identifier).unwrap())
    }

    pub fn describe_endpoints(&self) -> Vec<&Endpoint> {
        self.endpoints.values().collect()
    }

    pub fn get_endpoint_by_arn(&self, arn: &str) -> Option<&Endpoint> {
        self.endpoints.values().find(|e| e.endpoint_arn == arn)
    }

    pub fn delete_endpoint(&mut self, arn: &str) -> Result<Endpoint, DmsError> {
        let identifier = self
            .endpoints
            .values()
            .find(|e| e.endpoint_arn == arn)
            .map(|e| e.endpoint_identifier.clone())
            .ok_or_else(|| DmsError::EndpointNotFound(arn.to_string()))?;
        Ok(self.endpoints.remove(&identifier).unwrap())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn modify_endpoint(
        &mut self,
        arn: &str,
        endpoint_type: Option<&str>,
        engine_name: Option<&str>,
        username: Option<&str>,
        server_name: Option<&str>,
        port: Option<i32>,
        database_name: Option<&str>,
    ) -> Result<&Endpoint, DmsError> {
        let identifier = self
            .endpoints
            .values()
            .find(|e| e.endpoint_arn == arn)
            .map(|e| e.endpoint_identifier.clone())
            .ok_or_else(|| DmsError::EndpointNotFound(arn.to_string()))?;
        let endpoint = self.endpoints.get_mut(&identifier).unwrap();
        if let Some(t) = endpoint_type {
            endpoint.endpoint_type = t.to_string();
        }
        if let Some(e) = engine_name {
            endpoint.engine_name = e.to_string();
        }
        if let Some(u) = username {
            endpoint.username = Some(u.to_string());
        }
        if let Some(s) = server_name {
            endpoint.server_name = Some(s.to_string());
        }
        if let Some(p) = port {
            endpoint.port = Some(p);
        }
        if let Some(d) = database_name {
            endpoint.database_name = Some(d.to_string());
        }
        Ok(self.endpoints.get(&identifier).unwrap())
    }

    // ---- ReplicationTask ----

    #[allow(clippy::too_many_arguments)]
    pub fn create_replication_task(
        &mut self,
        identifier: &str,
        source_endpoint_arn: &str,
        target_endpoint_arn: &str,
        replication_instance_arn: &str,
        migration_type: &str,
        table_mappings: &str,
        replication_task_settings: Option<&str>,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&ReplicationTask, DmsError> {
        if self.replication_tasks.contains_key(identifier) {
            return Err(DmsError::ReplicationTaskAlreadyExists(
                identifier.to_string(),
            ));
        }
        let arn = format!("arn:aws:dms:{region}:{account_id}:task:{}", Uuid::new_v4());
        let now = Utc::now().timestamp() as f64;
        if !tags.is_empty() {
            self.tags.insert(arn.clone(), tags.clone());
        }
        let task = ReplicationTask {
            replication_task_identifier: identifier.to_string(),
            source_endpoint_arn: source_endpoint_arn.to_string(),
            target_endpoint_arn: target_endpoint_arn.to_string(),
            replication_instance_arn: replication_instance_arn.to_string(),
            migration_type: migration_type.to_string(),
            table_mappings: table_mappings.to_string(),
            replication_task_settings: replication_task_settings.map(|s| s.to_string()),
            status: "ready".to_string(),
            replication_task_arn: arn.clone(),
            replication_task_creation_date: now,
            replication_task_start_date: None,
            tags,
        };
        self.replication_tasks.insert(identifier.to_string(), task);
        Ok(self.replication_tasks.get(identifier).unwrap())
    }

    pub fn describe_replication_tasks(&self) -> Vec<&ReplicationTask> {
        self.replication_tasks.values().collect()
    }

    pub fn get_replication_task_by_arn(&self, arn: &str) -> Option<&ReplicationTask> {
        self.replication_tasks
            .values()
            .find(|t| t.replication_task_arn == arn)
    }

    pub fn delete_replication_task(&mut self, arn: &str) -> Result<ReplicationTask, DmsError> {
        let identifier = self
            .replication_tasks
            .values()
            .find(|t| t.replication_task_arn == arn)
            .map(|t| t.replication_task_identifier.clone())
            .ok_or_else(|| DmsError::ReplicationTaskNotFound(arn.to_string()))?;
        Ok(self.replication_tasks.remove(&identifier).unwrap())
    }

    pub fn modify_replication_task(
        &mut self,
        arn: &str,
        migration_type: Option<&str>,
        table_mappings: Option<&str>,
        replication_task_settings: Option<&str>,
    ) -> Result<&ReplicationTask, DmsError> {
        let identifier = self
            .replication_tasks
            .values()
            .find(|t| t.replication_task_arn == arn)
            .map(|t| t.replication_task_identifier.clone())
            .ok_or_else(|| DmsError::ReplicationTaskNotFound(arn.to_string()))?;
        let task = self.replication_tasks.get_mut(&identifier).unwrap();
        if let Some(m) = migration_type {
            task.migration_type = m.to_string();
        }
        if let Some(t) = table_mappings {
            task.table_mappings = t.to_string();
        }
        if let Some(s) = replication_task_settings {
            task.replication_task_settings = Some(s.to_string());
        }
        Ok(self.replication_tasks.get(&identifier).unwrap())
    }

    pub fn start_replication_task(&mut self, arn: &str) -> Result<&ReplicationTask, DmsError> {
        let identifier = self
            .replication_tasks
            .values()
            .find(|t| t.replication_task_arn == arn)
            .map(|t| t.replication_task_identifier.clone())
            .ok_or_else(|| DmsError::ReplicationTaskNotFound(arn.to_string()))?;
        let task = self.replication_tasks.get_mut(&identifier).unwrap();
        task.status = "running".to_string();
        task.replication_task_start_date = Some(Utc::now().timestamp() as f64);
        Ok(self.replication_tasks.get(&identifier).unwrap())
    }

    pub fn stop_replication_task(&mut self, arn: &str) -> Result<&ReplicationTask, DmsError> {
        let identifier = self
            .replication_tasks
            .values()
            .find(|t| t.replication_task_arn == arn)
            .map(|t| t.replication_task_identifier.clone())
            .ok_or_else(|| DmsError::ReplicationTaskNotFound(arn.to_string()))?;
        let task = self.replication_tasks.get_mut(&identifier).unwrap();
        task.status = "stopped".to_string();
        Ok(self.replication_tasks.get(&identifier).unwrap())
    }

    // ---- Connections ----

    pub fn describe_connections(&self) -> Vec<&Connection> {
        self.connections.values().collect()
    }

    pub fn test_connection(
        &mut self,
        replication_instance_arn: &str,
        endpoint_arn: &str,
    ) -> Result<&Connection, DmsError> {
        // Validate that the replication instance exists
        let instance = self
            .replication_instances
            .values()
            .find(|i| i.replication_instance_arn == replication_instance_arn)
            .ok_or_else(|| {
                DmsError::ReplicationInstanceNotFound(replication_instance_arn.to_string())
            })?;
        let instance_identifier = instance.replication_instance_identifier.clone();

        // Validate that the endpoint exists
        let endpoint = self
            .endpoints
            .values()
            .find(|e| e.endpoint_arn == endpoint_arn)
            .ok_or_else(|| DmsError::EndpointNotFound(endpoint_arn.to_string()))?;
        let endpoint_identifier = endpoint.endpoint_identifier.clone();

        let key = format!("{replication_instance_arn}:{endpoint_arn}");
        let connection = Connection {
            replication_instance_arn: replication_instance_arn.to_string(),
            endpoint_arn: endpoint_arn.to_string(),
            replication_instance_identifier: instance_identifier,
            endpoint_identifier,
            status: "successful".to_string(),
        };
        self.connections.insert(key.clone(), connection);
        Ok(self.connections.get(&key).unwrap())
    }

    // ---- Tags ----

    pub fn add_tags_to_resource(&mut self, resource_arn: &str, tags: HashMap<String, String>) {
        let entry = self.tags.entry(resource_arn.to_string()).or_default();
        entry.extend(tags);
    }

    pub fn list_tags_for_resource(&self, resource_arn: &str) -> Vec<crate::model::Tag> {
        let tags = self.tags.get(resource_arn).cloned().unwrap_or_default();
        tags.into_iter()
            .map(|(key, value)| crate::model::Tag {
                key: Some(key),
                value: Some(value),
                resource_arn: Some(resource_arn.to_string()),
            })
            .collect()
    }

    pub fn remove_tags_from_resource(&mut self, resource_arn: &str, tag_keys: &[String]) {
        if let Some(tags) = self.tags.get_mut(resource_arn) {
            for key in tag_keys {
                tags.remove(key);
            }
        }
    }

    // ---- ReplicationSubnetGroup ----

    pub fn create_replication_subnet_group(
        &mut self,
        identifier: &str,
        description: &str,
        subnet_ids: Vec<String>,
        vpc_id: Option<&str>,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&ReplicationSubnetGroup, DmsError> {
        if self.replication_subnet_groups.contains_key(identifier) {
            return Err(DmsError::ReplicationSubnetGroupAlreadyExists(
                identifier.to_string(),
            ));
        }
        let arn = format!("arn:aws:dms:{region}:{account_id}:subgrp:{identifier}");
        if !tags.is_empty() {
            self.tags.insert(arn.clone(), tags.clone());
        }
        let group = ReplicationSubnetGroup {
            replication_subnet_group_identifier: identifier.to_string(),
            replication_subnet_group_description: description.to_string(),
            vpc_id: vpc_id.map(|s| s.to_string()),
            subnet_ids,
            tags,
        };
        self.replication_subnet_groups
            .insert(identifier.to_string(), group);
        Ok(self.replication_subnet_groups.get(identifier).unwrap())
    }

    pub fn describe_replication_subnet_groups(&self) -> Vec<&ReplicationSubnetGroup> {
        self.replication_subnet_groups.values().collect()
    }

    pub fn delete_replication_subnet_group(&mut self, identifier: &str) -> Result<(), DmsError> {
        self.replication_subnet_groups
            .remove(identifier)
            .ok_or_else(|| DmsError::ReplicationSubnetGroupNotFound(identifier.to_string()))?;
        Ok(())
    }

    pub fn modify_replication_subnet_group(
        &mut self,
        identifier: &str,
        description: Option<&str>,
        subnet_ids: Option<Vec<String>>,
    ) -> Result<&ReplicationSubnetGroup, DmsError> {
        let group = self
            .replication_subnet_groups
            .get_mut(identifier)
            .ok_or_else(|| DmsError::ReplicationSubnetGroupNotFound(identifier.to_string()))?;
        if let Some(d) = description {
            group.replication_subnet_group_description = d.to_string();
        }
        if let Some(ids) = subnet_ids {
            group.subnet_ids = ids;
        }
        Ok(self.replication_subnet_groups.get(identifier).unwrap())
    }

    // ---- Certificate ----

    pub fn import_certificate(
        &mut self,
        identifier: &str,
        certificate_pem: Option<&str>,
        certificate_wallet: Option<&str>,
        kms_key_id: Option<&str>,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&Certificate, DmsError> {
        if self.certificates.contains_key(identifier) {
            return Err(DmsError::CertificateAlreadyExists(identifier.to_string()));
        }
        let arn = format!("arn:aws:dms:{region}:{account_id}:cert:{identifier}");
        let now = Utc::now().timestamp() as f64;
        if !tags.is_empty() {
            self.tags.insert(arn.clone(), tags);
        }
        let cert = Certificate {
            certificate_identifier: identifier.to_string(),
            certificate_arn: arn.clone(),
            certificate_pem: certificate_pem.map(|s| s.to_string()),
            certificate_wallet: certificate_wallet.map(|s| s.to_string()),
            kms_key_id: kms_key_id.map(|s| s.to_string()),
            certificate_creation_date: now,
        };
        self.certificates.insert(identifier.to_string(), cert);
        Ok(self.certificates.get(identifier).unwrap())
    }

    pub fn describe_certificates(&self) -> Vec<&Certificate> {
        self.certificates.values().collect()
    }

    pub fn delete_certificate(&mut self, arn: &str) -> Result<Certificate, DmsError> {
        let identifier = self
            .certificates
            .values()
            .find(|c| c.certificate_arn == arn)
            .map(|c| c.certificate_identifier.clone())
            .ok_or_else(|| DmsError::CertificateNotFound(arn.to_string()))?;
        Ok(self.certificates.remove(&identifier).unwrap())
    }

    // ---- EventSubscription ----

    #[allow(clippy::too_many_arguments)]
    pub fn create_event_subscription(
        &mut self,
        subscription_name: &str,
        sns_topic_arn: &str,
        source_type: Option<&str>,
        event_categories: Vec<String>,
        source_ids: Vec<String>,
        enabled: bool,
        account_id: &str,
    ) -> Result<&EventSubscription, DmsError> {
        if self.event_subscriptions.contains_key(subscription_name) {
            return Err(DmsError::EventSubscriptionAlreadyExists(
                subscription_name.to_string(),
            ));
        }
        let now = Utc::now().to_rfc3339();
        let sub = EventSubscription {
            subscription_name: subscription_name.to_string(),
            sns_topic_arn: sns_topic_arn.to_string(),
            source_type: source_type.map(|s| s.to_string()),
            event_categories,
            source_ids,
            enabled,
            status: "active".to_string(),
            subscription_creation_time: now,
            customer_aws_id: account_id.to_string(),
        };
        self.event_subscriptions
            .insert(subscription_name.to_string(), sub);
        Ok(self.event_subscriptions.get(subscription_name).unwrap())
    }

    pub fn describe_event_subscriptions(&self) -> Vec<&EventSubscription> {
        self.event_subscriptions.values().collect()
    }

    pub fn delete_event_subscription(
        &mut self,
        subscription_name: &str,
    ) -> Result<EventSubscription, DmsError> {
        self.event_subscriptions
            .remove(subscription_name)
            .ok_or_else(|| DmsError::EventSubscriptionNotFound(subscription_name.to_string()))
    }

    pub fn modify_event_subscription(
        &mut self,
        subscription_name: &str,
        sns_topic_arn: Option<&str>,
        source_type: Option<&str>,
        event_categories: Option<Vec<String>>,
        enabled: Option<bool>,
    ) -> Result<&EventSubscription, DmsError> {
        let sub = self
            .event_subscriptions
            .get_mut(subscription_name)
            .ok_or_else(|| DmsError::EventSubscriptionNotFound(subscription_name.to_string()))?;
        if let Some(a) = sns_topic_arn {
            sub.sns_topic_arn = a.to_string();
        }
        if let Some(t) = source_type {
            sub.source_type = Some(t.to_string());
        }
        if let Some(cats) = event_categories {
            sub.event_categories = cats;
        }
        if let Some(e) = enabled {
            sub.enabled = e;
        }
        Ok(self.event_subscriptions.get(subscription_name).unwrap())
    }

    // ---- Catalog / reference data accessors ----

    pub fn get_account_quotas(&self) -> &[crate::model::AccountQuota] {
        &self.account_quotas
    }

    pub fn get_event_category_groups(&self) -> &[crate::model::EventCategoryGroup] {
        &self.event_category_groups
    }

    pub fn get_orderable_replication_instances(
        &self,
    ) -> &[crate::model::OrderableReplicationInstance] {
        &self.orderable_replication_instances
    }

    pub fn get_engine_versions(&self) -> &[crate::model::EngineVersion] {
        &self.engine_versions
    }

    pub fn get_supported_endpoint_types(&self) -> &[crate::model::SupportedEndpointType] {
        &self.supported_endpoint_types
    }

    pub fn get_endpoint_settings(&self) -> &[crate::model::EndpointSetting] {
        &self.endpoint_settings
    }

    pub fn get_events(&self) -> &[crate::model::Event] {
        &self.events
    }

    pub fn get_pending_maintenance_actions(
        &self,
    ) -> &[crate::model::ResourcePendingMaintenanceActions] {
        &self.pending_maintenance_actions
    }

    pub fn get_individual_assessment_names(&self) -> &[String] {
        &self.individual_assessment_names
    }

    pub fn get_replication_task_assessment_results(
        &self,
    ) -> &[crate::model::ReplicationTaskAssessmentResult] {
        &self.replication_task_assessment_results
    }

    pub fn get_replication_task_assessment_runs(
        &self,
    ) -> &[crate::model::ReplicationTaskAssessmentRun] {
        &self.replication_task_assessment_runs
    }

    pub fn get_replication_task_individual_assessments(
        &self,
    ) -> &[crate::model::ReplicationTaskIndividualAssessment] {
        &self.replication_task_individual_assessments
    }
}
