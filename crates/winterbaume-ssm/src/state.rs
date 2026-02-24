use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::types::*;

#[derive(Debug, Default)]
pub struct SsmState {
    pub parameters: HashMap<String, Parameter>,
    /// Version history for parameters (keyed by parameter name).
    pub parameter_versions: HashMap<String, Vec<ParameterVersion>>,
    pub documents: HashMap<String, Document>,
    pub maintenance_windows: HashMap<String, MaintenanceWindow>,
    pub patch_baselines: HashMap<String, PatchBaseline>,
    /// Default patch baseline per operating system (OS name → baseline_id).
    pub default_patch_baselines: HashMap<String, String>,
    pub commands: HashMap<String, Command>,
    pub associations: HashMap<String, Association>,
    /// Tags for arbitrary resources (keyed by resource ARN or resource ID).
    pub resource_tags: HashMap<String, HashMap<String, String>>,
    /// OpsItems keyed by ops_item_id.
    pub ops_items: HashMap<String, OpsItem>,
    /// Session Manager sessions keyed by session_id.
    pub sessions: HashMap<String, Session>,
    /// Managed instance activations keyed by activation_id.
    pub activations: HashMap<String, Activation>,
    /// Resource policies keyed by resource_arn, containing a list of policies.
    pub resource_policies: HashMap<String, Vec<ResourcePolicy>>,
    /// Service settings keyed by setting_id.
    pub service_settings: HashMap<String, ServiceSetting>,
    /// Inventory data keyed by (instance_id, type_name).
    pub inventory: HashMap<(String, String), InventoryData>,
    /// Compliance records keyed by (resource_id, compliance_type, item_id).
    pub compliance_records: HashMap<(String, String, String), ComplianceRecord>,
    /// OpsMetadata entries keyed by ops_metadata_arn.
    pub ops_metadata: HashMap<String, OpsMetadataEntry>,
    /// Resource Data Sync configurations keyed by sync_name.
    pub resource_data_syncs: HashMap<String, ResourceDataSyncEntry>,
    /// OpsItem related-item associations keyed by association_id.
    pub ops_item_related_items: HashMap<String, OpsItemRelatedItem>,
    /// Managed instances keyed by instance_id.
    pub managed_instances: HashMap<String, ManagedInstance>,
    /// Inventory deletion records keyed by deletion_id.
    pub inventory_deletions: HashMap<String, InventoryDeletion>,
    /// Maintenance window execution records keyed by window_execution_id.
    pub maintenance_window_executions: HashMap<String, MaintenanceWindowExecution>,
    /// Document review metadata keyed by document name.
    pub document_metadata: HashMap<String, Vec<DocumentMetadataEntry>>,
    next_window_id: u64,
    next_target_id: u64,
    next_task_id: u64,
    next_baseline_id: u64,
    next_command_id: u64,
    next_association_id: u64,
    next_ops_item_id: u64,
    next_session_id: u64,
    next_activation_id: u64,
    next_policy_id: u64,
    next_ops_metadata_id: u64,
    next_ops_item_related_id: u64,
    next_managed_instance_id: u64,
    next_deletion_id: u64,
    next_window_execution_id: u64,
    next_task_execution_id: u64,
    next_invocation_id: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum SsmError {
    #[error(
        "The parameter already exists. To overwrite this value, set the overwrite option in the request to true."
    )]
    ParameterAlreadyExists,
    #[error("Parameter {0} not found.")]
    ParameterNotFound(String),
    #[error("Version {0} of parameter {1} not found.")]
    ParameterVersionNotFound(i64, String),
    #[error("Document {0} already exists.")]
    DocumentAlreadyExists(String),
    #[error("Document {0} does not exist.")]
    InvalidDocument(String),
    #[error("Document version {0} does not exist.")]
    InvalidDocumentVersion(String),
    #[error("Maintenance window {0} does not exist.")]
    MaintenanceWindowDoesNotExist(String),
    #[error("Task {0} does not exist in window {1}.")]
    MaintenanceWindowTaskDoesNotExist(String, String),
    #[error("Patch baseline {0} does not exist.")]
    PatchBaselineDoesNotExist(String),
    #[error("Session {0} not found.")]
    SessionDoesNotExist(String),
    #[error("Association {0} does not exist.")]
    AssociationDoesNotExist(String),
    #[error("Command {0} not found.")]
    InvalidCommandId(String),
    #[error("Invocation for instance {0} not found.")]
    InvocationDoesNotExist(String),
    #[error("OpsItem {0} not found.")]
    OpsItemNotFound(String),
    #[error("Activation {0} not found.")]
    InvalidActivationId(String),
    #[error("OpsMetadata {0} not found.")]
    OpsMetadataNotFound(String),
    #[error("A resource data sync already exists with the name {0}.")]
    ResourceDataSyncAlreadyExists(String),
    #[error("Resource data sync {0} not found.")]
    ResourceDataSyncNotFound(String),
}

impl SsmState {
    fn gen_window_id(&mut self) -> String {
        self.next_window_id += 1;
        format!("mw-{:017x}", self.next_window_id)
    }

    fn gen_target_id(&mut self) -> String {
        self.next_target_id += 1;
        format!("t-{:017x}", self.next_target_id)
    }

    fn gen_task_id(&mut self) -> String {
        self.next_task_id += 1;
        format!("task-{:017x}", self.next_task_id)
    }

    fn gen_baseline_id(&mut self) -> String {
        self.next_baseline_id += 1;
        format!("pb-{:017x}", self.next_baseline_id)
    }

    fn gen_command_id(&mut self) -> String {
        self.next_command_id += 1;
        format!(
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            self.next_command_id, 0, 0, 0, 0
        )
    }

    fn gen_association_id(&mut self) -> String {
        self.next_association_id += 1;
        format!(
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            self.next_association_id, 1, 0, 0, 0
        )
    }

    fn gen_ops_item_id(&mut self) -> String {
        self.next_ops_item_id += 1;
        format!("oi-{:017x}", self.next_ops_item_id)
    }

    fn gen_session_id(&mut self) -> String {
        self.next_session_id += 1;
        format!(
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            self.next_session_id, 2, 0, 0, 0
        )
    }

    fn gen_activation_id(&mut self) -> String {
        self.next_activation_id += 1;
        format!(
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            self.next_activation_id, 3, 0, 0, 0
        )
    }

    fn gen_policy_id(&mut self) -> String {
        self.next_policy_id += 1;
        format!("pol-{:017x}", self.next_policy_id)
    }

    fn gen_ops_metadata_arn(&mut self, account_id: &str, region: &str) -> String {
        self.next_ops_metadata_id += 1;
        format!(
            "arn:aws:ssm:{region}:{account_id}:opsmetadata/aws/ssm/{:017x}",
            self.next_ops_metadata_id
        )
    }

    fn gen_ops_item_related_id(&mut self) -> String {
        self.next_ops_item_related_id += 1;
        format!(
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            self.next_ops_item_related_id, 4, 0, 0, 0
        )
    }

    fn gen_managed_instance_id(&mut self) -> String {
        self.next_managed_instance_id += 1;
        format!("mi-{:017x}", self.next_managed_instance_id)
    }

    fn gen_deletion_id(&mut self) -> String {
        self.next_deletion_id += 1;
        format!(
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            self.next_deletion_id, 5, 0, 0, 0
        )
    }

    fn gen_window_execution_id(&mut self) -> String {
        self.next_window_execution_id += 1;
        format!(
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            self.next_window_execution_id, 6, 0, 0, 0
        )
    }

    fn gen_task_execution_id(&mut self) -> String {
        self.next_task_execution_id += 1;
        format!(
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            self.next_task_execution_id, 7, 0, 0, 0
        )
    }

    fn gen_invocation_id(&mut self) -> String {
        self.next_invocation_id += 1;
        format!(
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            self.next_invocation_id, 8, 0, 0, 0
        )
    }

    // ── Parameter operations ──────────────────────────────────────────

    pub fn put_parameter(
        &mut self,
        name: &str,
        value: &str,
        param_type: &str,
        account_id: &str,
        region: &str,
        overwrite: bool,
        tags: HashMap<String, String>,
    ) -> Result<i64, SsmError> {
        let version = if let Some(existing) = self.parameters.get(name) {
            if !overwrite {
                return Err(SsmError::ParameterAlreadyExists);
            }
            existing.version + 1
        } else {
            1
        };

        let arn = if name.starts_with('/') {
            format!("arn:aws:ssm:{region}:{account_id}:parameter{name}")
        } else {
            format!("arn:aws:ssm:{region}:{account_id}:parameter/{name}")
        };
        let now = Utc::now();
        let parameter = Parameter {
            name: name.to_string(),
            r#type: param_type.to_string(),
            value: value.to_string(),
            version,
            last_modified_date: now,
            arn,
            data_type: "text".to_string(),
            tags,
        };

        // Record version history
        let pv = ParameterVersion {
            name: name.to_string(),
            r#type: param_type.to_string(),
            value: value.to_string(),
            version,
            last_modified_date: now,
            labels: Vec::new(),
        };
        self.parameter_versions
            .entry(name.to_string())
            .or_default()
            .push(pv);

        self.parameters.insert(name.to_string(), parameter);
        Ok(version)
    }

    pub fn get_parameter(&self, name: &str) -> Result<&Parameter, SsmError> {
        self.parameters
            .get(name)
            .ok_or_else(|| SsmError::ParameterNotFound(name.to_string()))
    }

    pub fn get_parameters(&self, names: &[&str]) -> (Vec<&Parameter>, Vec<String>) {
        let mut valid = Vec::new();
        let mut invalid = Vec::new();
        for name in names {
            match self.parameters.get(*name) {
                Some(p) => valid.push(p),
                None => invalid.push(name.to_string()),
            }
        }
        (valid, invalid)
    }

    pub fn get_parameters_by_path(&self, path: &str, recursive: bool) -> Vec<&Parameter> {
        let prefix = if path.ends_with('/') {
            path.to_string()
        } else {
            format!("{path}/")
        };

        self.parameters
            .values()
            .filter(|p| {
                if !p.name.starts_with(&prefix) {
                    return false;
                }
                if !recursive {
                    let remainder = &p.name[prefix.len()..];
                    return !remainder.contains('/');
                }
                true
            })
            .collect()
    }

    pub fn delete_parameter(&mut self, name: &str) -> Result<(), SsmError> {
        if self.parameters.remove(name).is_none() {
            return Err(SsmError::ParameterNotFound(name.to_string()));
        }
        self.parameter_versions.remove(name);
        Ok(())
    }

    pub fn delete_parameters(&mut self, names: &[&str]) -> (Vec<String>, Vec<String>) {
        let mut deleted = Vec::new();
        let mut invalid = Vec::new();
        for name in names {
            if self.parameters.remove(*name).is_some() {
                self.parameter_versions.remove(*name);
                deleted.push(name.to_string());
            } else {
                invalid.push(name.to_string());
            }
        }
        (deleted, invalid)
    }

    pub fn describe_parameters(&self) -> Vec<&Parameter> {
        self.parameters.values().collect()
    }

    pub fn get_parameter_history(&self, name: &str) -> Result<&Vec<ParameterVersion>, SsmError> {
        self.parameter_versions
            .get(name)
            .ok_or_else(|| SsmError::ParameterNotFound(name.to_string()))
    }

    pub fn label_parameter_version(
        &mut self,
        name: &str,
        version: Option<i64>,
        labels: Vec<String>,
    ) -> Result<Vec<String>, SsmError> {
        let versions = self
            .parameter_versions
            .get_mut(name)
            .ok_or_else(|| SsmError::ParameterNotFound(name.to_string()))?;

        let target_version = match version {
            Some(v) => v,
            None => {
                // Use latest version
                let param = self
                    .parameters
                    .get(name)
                    .ok_or_else(|| SsmError::ParameterNotFound(name.to_string()))?;
                param.version
            }
        };

        let pv = versions
            .iter_mut()
            .find(|pv| pv.version == target_version)
            .ok_or_else(|| SsmError::ParameterVersionNotFound(target_version, name.to_string()))?;

        let mut invalid_labels = Vec::new();
        for label in &labels {
            if pv.labels.len() >= 10 {
                invalid_labels.push(label.clone());
            } else if !pv.labels.contains(label) {
                pv.labels.push(label.clone());
            }
        }
        Ok(invalid_labels)
    }

    pub fn unlabel_parameter_version(
        &mut self,
        name: &str,
        version: i64,
        labels: Vec<String>,
    ) -> Result<Vec<String>, SsmError> {
        let versions = self
            .parameter_versions
            .get_mut(name)
            .ok_or_else(|| SsmError::ParameterNotFound(name.to_string()))?;

        let pv = versions
            .iter_mut()
            .find(|pv| pv.version == version)
            .ok_or_else(|| SsmError::ParameterVersionNotFound(version, name.to_string()))?;

        let mut removed = Vec::new();
        let mut invalid = Vec::new();
        for label in &labels {
            if let Some(pos) = pv.labels.iter().position(|l| l == label) {
                pv.labels.remove(pos);
                removed.push(label.clone());
            } else {
                invalid.push(label.clone());
            }
        }
        Ok(invalid)
    }

    // ── Tag operations ────────────────────────────────────────────────

    pub fn add_tags_to_resource(
        &mut self,
        resource_id: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), SsmError> {
        // Try parameter first for backward compat, then use generic resource_tags
        if let Some(param) = self.parameters.get_mut(resource_id) {
            param.tags.extend(tags.clone());
        }
        self.resource_tags
            .entry(resource_id.to_string())
            .or_default()
            .extend(tags);
        Ok(())
    }

    pub fn remove_tags_from_resource(
        &mut self,
        resource_id: &str,
        tag_keys: &[String],
    ) -> Result<(), SsmError> {
        if let Some(param) = self.parameters.get_mut(resource_id) {
            for key in tag_keys {
                param.tags.remove(key);
            }
        }
        if let Some(tags) = self.resource_tags.get_mut(resource_id) {
            for key in tag_keys {
                tags.remove(key);
            }
        }
        Ok(())
    }

    pub fn list_tags_for_resource(&self, resource_id: &str) -> HashMap<String, String> {
        self.resource_tags
            .get(resource_id)
            .cloned()
            .unwrap_or_default()
    }

    // ── Document operations ───────────────────────────────────────────

    pub fn create_document(
        &mut self,
        name: &str,
        content: &str,
        document_type: &str,
        document_format: &str,
        owner: &str,
    ) -> Result<&Document, SsmError> {
        if self.documents.contains_key(name) {
            return Err(SsmError::DocumentAlreadyExists(name.to_string()));
        }

        let now = Utc::now();
        let doc = Document {
            name: name.to_string(),
            content: content.to_string(),
            document_type: document_type.to_string(),
            document_format: document_format.to_string(),
            status: "Active".to_string(),
            owner: owner.to_string(),
            default_version: "1".to_string(),
            latest_version: "1".to_string(),
            versions: vec![DocumentVersion {
                version_name: String::new(),
                document_version: "1".to_string(),
                content: content.to_string(),
                created_date: now,
                status: "Active".to_string(),
            }],
            account_permissions: Vec::new(),
            created_date: now,
        };

        self.documents.insert(name.to_string(), doc);
        Ok(self.documents.get(name).unwrap())
    }

    pub fn get_document(&self, name: &str, version: Option<&str>) -> Result<&Document, SsmError> {
        let doc = self
            .documents
            .get(name)
            .ok_or_else(|| SsmError::InvalidDocument(name.to_string()))?;
        // version filtering is handled at the handler level
        let _ = version;
        Ok(doc)
    }

    pub fn describe_document(&self, name: &str) -> Result<&Document, SsmError> {
        self.documents
            .get(name)
            .ok_or_else(|| SsmError::InvalidDocument(name.to_string()))
    }

    pub fn delete_document(&mut self, name: &str) -> Result<(), SsmError> {
        if self.documents.remove(name).is_none() {
            return Err(SsmError::InvalidDocument(name.to_string()));
        }
        Ok(())
    }

    pub fn update_document(
        &mut self,
        name: &str,
        content: &str,
        document_version: Option<&str>,
    ) -> Result<&Document, SsmError> {
        let doc = self
            .documents
            .get_mut(name)
            .ok_or_else(|| SsmError::InvalidDocument(name.to_string()))?;

        let new_version_num: i64 = doc.latest_version.parse::<i64>().unwrap_or(1) + 1;
        let new_version = new_version_num.to_string();

        let _ = document_version; // could be used for optimistic locking

        let now = Utc::now();
        doc.content = content.to_string();
        doc.latest_version = new_version.clone();
        doc.versions.push(DocumentVersion {
            version_name: String::new(),
            document_version: new_version,
            content: content.to_string(),
            created_date: now,
            status: "Active".to_string(),
        });

        Ok(doc)
    }

    pub fn update_document_default_version(
        &mut self,
        name: &str,
        document_version: &str,
    ) -> Result<&Document, SsmError> {
        let doc = self
            .documents
            .get_mut(name)
            .ok_or_else(|| SsmError::InvalidDocument(name.to_string()))?;

        // Verify the version exists
        let version_exists = doc
            .versions
            .iter()
            .any(|v| v.document_version == document_version);
        if !version_exists {
            return Err(SsmError::InvalidDocumentVersion(
                document_version.to_string(),
            ));
        }

        doc.default_version = document_version.to_string();
        Ok(doc)
    }

    pub fn list_documents(&self) -> Vec<&Document> {
        self.documents.values().collect()
    }

    pub fn describe_document_permission(&self, name: &str) -> Result<&Document, SsmError> {
        self.documents
            .get(name)
            .ok_or_else(|| SsmError::InvalidDocument(name.to_string()))
    }

    pub fn modify_document_permission(
        &mut self,
        name: &str,
        permission_type: &str,
        account_ids_to_add: Vec<String>,
        account_ids_to_remove: Vec<String>,
    ) -> Result<(), SsmError> {
        let doc = self
            .documents
            .get_mut(name)
            .ok_or_else(|| SsmError::InvalidDocument(name.to_string()))?;
        let _ = permission_type;

        for id in account_ids_to_add {
            if !doc.account_permissions.contains(&id) {
                doc.account_permissions.push(id);
            }
        }
        for id in &account_ids_to_remove {
            doc.account_permissions.retain(|a| a != id);
        }
        Ok(())
    }

    // ── Maintenance Window operations ─────────────────────────────────

    pub fn create_maintenance_window(
        &mut self,
        name: &str,
        schedule: &str,
        duration: i64,
        cutoff: i64,
        enabled: bool,
    ) -> String {
        let window_id = self.gen_window_id();
        let window = MaintenanceWindow {
            window_id: window_id.clone(),
            name: name.to_string(),
            schedule: schedule.to_string(),
            duration,
            cutoff,
            enabled,
            targets: Vec::new(),
            tasks: Vec::new(),
        };
        self.maintenance_windows.insert(window_id.clone(), window);
        window_id
    }

    pub fn get_maintenance_window(&self, window_id: &str) -> Result<&MaintenanceWindow, SsmError> {
        self.maintenance_windows
            .get(window_id)
            .ok_or_else(|| SsmError::MaintenanceWindowDoesNotExist(window_id.to_string()))
    }

    pub fn delete_maintenance_window(&mut self, window_id: &str) -> Result<(), SsmError> {
        // AWS SSM DeleteMaintenanceWindow does not error if the window doesn't exist
        self.maintenance_windows.remove(window_id);
        Ok(())
    }

    pub fn describe_maintenance_windows(&self) -> Vec<&MaintenanceWindow> {
        self.maintenance_windows.values().collect()
    }

    pub fn register_target_with_maintenance_window(
        &mut self,
        window_id: &str,
        resource_type: &str,
        targets: Vec<Target>,
    ) -> Result<String, SsmError> {
        let target_id = self.gen_target_id();
        let target = MaintenanceWindowTarget {
            window_target_id: target_id.clone(),
            window_id: window_id.to_string(),
            resource_type: resource_type.to_string(),
            targets,
        };
        let window = self
            .maintenance_windows
            .get_mut(window_id)
            .ok_or_else(|| SsmError::MaintenanceWindowDoesNotExist(window_id.to_string()))?;
        window.targets.push(target);
        Ok(target_id)
    }

    pub fn deregister_target_from_maintenance_window(
        &mut self,
        window_id: &str,
        window_target_id: &str,
    ) -> Result<(), SsmError> {
        let window = self
            .maintenance_windows
            .get_mut(window_id)
            .ok_or_else(|| SsmError::MaintenanceWindowDoesNotExist(window_id.to_string()))?;
        window
            .targets
            .retain(|t| t.window_target_id != window_target_id);
        Ok(())
    }

    pub fn describe_maintenance_window_targets(
        &self,
        window_id: &str,
    ) -> Result<&Vec<MaintenanceWindowTarget>, SsmError> {
        let window = self
            .maintenance_windows
            .get(window_id)
            .ok_or_else(|| SsmError::MaintenanceWindowDoesNotExist(window_id.to_string()))?;
        Ok(&window.targets)
    }

    pub fn register_task_with_maintenance_window(
        &mut self,
        window_id: &str,
        task_arn: &str,
        task_type: &str,
        targets: Vec<Target>,
    ) -> Result<String, SsmError> {
        let task_id = self.gen_task_id();
        let task = MaintenanceWindowTask {
            window_task_id: task_id.clone(),
            window_id: window_id.to_string(),
            task_arn: task_arn.to_string(),
            task_type: task_type.to_string(),
            targets,
        };
        let window = self
            .maintenance_windows
            .get_mut(window_id)
            .ok_or_else(|| SsmError::MaintenanceWindowDoesNotExist(window_id.to_string()))?;
        window.tasks.push(task);
        Ok(task_id)
    }

    pub fn deregister_task_from_maintenance_window(
        &mut self,
        window_id: &str,
        window_task_id: &str,
    ) -> Result<(), SsmError> {
        let window = self
            .maintenance_windows
            .get_mut(window_id)
            .ok_or_else(|| SsmError::MaintenanceWindowDoesNotExist(window_id.to_string()))?;
        window.tasks.retain(|t| t.window_task_id != window_task_id);
        Ok(())
    }

    pub fn describe_maintenance_window_tasks(
        &self,
        window_id: &str,
    ) -> Result<&Vec<MaintenanceWindowTask>, SsmError> {
        let window = self
            .maintenance_windows
            .get(window_id)
            .ok_or_else(|| SsmError::MaintenanceWindowDoesNotExist(window_id.to_string()))?;
        Ok(&window.tasks)
    }

    pub fn update_maintenance_window(
        &mut self,
        window_id: &str,
        name: Option<&str>,
        schedule: Option<&str>,
        duration: Option<i64>,
        cutoff: Option<i64>,
        enabled: Option<bool>,
    ) -> Result<&MaintenanceWindow, SsmError> {
        let window = self
            .maintenance_windows
            .get_mut(window_id)
            .ok_or_else(|| SsmError::MaintenanceWindowDoesNotExist(window_id.to_string()))?;
        if let Some(n) = name {
            window.name = n.to_string();
        }
        if let Some(s) = schedule {
            window.schedule = s.to_string();
        }
        if let Some(d) = duration {
            window.duration = d;
        }
        if let Some(c) = cutoff {
            window.cutoff = c;
        }
        if let Some(e) = enabled {
            window.enabled = e;
        }
        Ok(window)
    }

    pub fn get_maintenance_window_task(
        &self,
        window_id: &str,
        window_task_id: &str,
    ) -> Result<&MaintenanceWindowTask, SsmError> {
        let window = self
            .maintenance_windows
            .get(window_id)
            .ok_or_else(|| SsmError::MaintenanceWindowDoesNotExist(window_id.to_string()))?;
        window
            .tasks
            .iter()
            .find(|t| t.window_task_id == window_task_id)
            .ok_or_else(|| {
                SsmError::MaintenanceWindowTaskDoesNotExist(
                    window_task_id.to_string(),
                    window_id.to_string(),
                )
            })
    }

    pub fn update_maintenance_window_task(
        &mut self,
        window_id: &str,
        window_task_id: &str,
        task_arn: Option<&str>,
    ) -> Result<&MaintenanceWindowTask, SsmError> {
        let window = self
            .maintenance_windows
            .get_mut(window_id)
            .ok_or_else(|| SsmError::MaintenanceWindowDoesNotExist(window_id.to_string()))?;
        let task = window
            .tasks
            .iter_mut()
            .find(|t| t.window_task_id == window_task_id)
            .ok_or_else(|| {
                SsmError::MaintenanceWindowTaskDoesNotExist(
                    window_task_id.to_string(),
                    window_id.to_string(),
                )
            })?;
        if let Some(arn) = task_arn {
            task.task_arn = arn.to_string();
        }
        Ok(task)
    }

    // ── Association operations ────────────────────────────────────────

    pub fn create_association(
        &mut self,
        name: &str,
        association_name: Option<&str>,
        targets: Vec<Target>,
        schedule_expression: Option<&str>,
        parameters: HashMap<String, Vec<String>>,
        document_version: Option<&str>,
    ) -> &Association {
        let association_id = self.gen_association_id();
        let now = Utc::now();
        let association = Association {
            association_id: association_id.clone(),
            association_name: association_name.map(|s| s.to_string()),
            name: name.to_string(),
            document_version: document_version.map(|s| s.to_string()),
            targets,
            schedule_expression: schedule_expression.map(|s| s.to_string()),
            parameters,
            overview: AssociationOverview {
                status: "Success".to_string(),
                detailed_status: "Success".to_string(),
            },
            last_execution_date: None,
            association_version: "1".to_string(),
            created_date: now,
        };
        self.associations
            .insert(association_id.clone(), association);
        self.associations.get(&association_id).unwrap()
    }

    pub fn get_association(&self, association_id: &str) -> Result<&Association, SsmError> {
        self.associations
            .get(association_id)
            .ok_or_else(|| SsmError::AssociationDoesNotExist(association_id.to_string()))
    }

    pub fn delete_association(&mut self, association_id: &str) -> Result<(), SsmError> {
        self.associations
            .remove(association_id)
            .ok_or_else(|| SsmError::AssociationDoesNotExist(association_id.to_string()))?;
        Ok(())
    }

    pub fn list_associations(&self) -> Vec<&Association> {
        self.associations.values().collect()
    }

    pub fn update_association(
        &mut self,
        association_id: &str,
        schedule_expression: Option<&str>,
        document_version: Option<&str>,
    ) -> Result<&Association, SsmError> {
        let assoc = self
            .associations
            .get_mut(association_id)
            .ok_or_else(|| SsmError::AssociationDoesNotExist(association_id.to_string()))?;
        if let Some(s) = schedule_expression {
            assoc.schedule_expression = Some(s.to_string());
        }
        if let Some(v) = document_version {
            assoc.document_version = Some(v.to_string());
        }
        // Bump version number
        let v: i64 = assoc.association_version.parse().unwrap_or(1);
        assoc.association_version = (v + 1).to_string();
        Ok(assoc)
    }

    // ── Patch Baseline operations ─────────────────────────────────────

    pub fn create_patch_baseline(
        &mut self,
        name: &str,
        operating_system: &str,
        description: Option<&str>,
    ) -> String {
        let baseline_id = self.gen_baseline_id();
        let baseline = PatchBaseline {
            baseline_id: baseline_id.clone(),
            name: name.to_string(),
            operating_system: operating_system.to_string(),
            description: description.map(|s| s.to_string()),
            patch_groups: Vec::new(),
        };
        self.patch_baselines.insert(baseline_id.clone(), baseline);
        baseline_id
    }

    pub fn update_patch_baseline(
        &mut self,
        baseline_id: &str,
        name: Option<&str>,
        description: Option<&str>,
    ) -> Result<&PatchBaseline, SsmError> {
        let baseline = self
            .patch_baselines
            .get_mut(baseline_id)
            .ok_or_else(|| SsmError::PatchBaselineDoesNotExist(baseline_id.to_string()))?;
        if let Some(n) = name {
            baseline.name = n.to_string();
        }
        if let Some(d) = description {
            baseline.description = Some(d.to_string());
        }
        Ok(baseline)
    }

    pub fn get_default_patch_baseline(&self, operating_system: &str) -> Option<&PatchBaseline> {
        let baseline_id = self.default_patch_baselines.get(operating_system)?;
        self.patch_baselines.get(baseline_id)
    }

    pub fn set_default_patch_baseline(&mut self, baseline_id: &str) -> Result<(), SsmError> {
        let baseline = self
            .patch_baselines
            .get(baseline_id)
            .ok_or_else(|| SsmError::PatchBaselineDoesNotExist(baseline_id.to_string()))?;
        let os = baseline.operating_system.clone();
        self.default_patch_baselines
            .insert(os, baseline_id.to_string());
        Ok(())
    }

    pub fn delete_patch_baseline(&mut self, baseline_id: &str) -> Result<String, SsmError> {
        self.patch_baselines.remove(baseline_id);
        Ok(baseline_id.to_string())
    }

    pub fn describe_patch_baselines(&self) -> Vec<&PatchBaseline> {
        self.patch_baselines.values().collect()
    }

    pub fn register_patch_baseline_for_patch_group(
        &mut self,
        baseline_id: &str,
        patch_group: &str,
    ) -> Result<(), SsmError> {
        let baseline = self
            .patch_baselines
            .get_mut(baseline_id)
            .ok_or_else(|| SsmError::PatchBaselineDoesNotExist(baseline_id.to_string()))?;
        if !baseline.patch_groups.contains(&patch_group.to_string()) {
            baseline.patch_groups.push(patch_group.to_string());
        }
        Ok(())
    }

    pub fn deregister_patch_baseline_for_patch_group(
        &mut self,
        baseline_id: &str,
        patch_group: &str,
    ) -> Result<(), SsmError> {
        let baseline = self
            .patch_baselines
            .get_mut(baseline_id)
            .ok_or_else(|| SsmError::PatchBaselineDoesNotExist(baseline_id.to_string()))?;
        baseline.patch_groups.retain(|g| g != patch_group);
        Ok(())
    }

    pub fn get_patch_baseline_for_patch_group(&self, patch_group: &str) -> Option<&PatchBaseline> {
        self.patch_baselines
            .values()
            .find(|b| b.patch_groups.contains(&patch_group.to_string()))
    }

    // ── Command operations ────────────────────────────────────────────

    pub fn send_command(
        &mut self,
        document_name: &str,
        instance_ids: Vec<String>,
        parameters: HashMap<String, Vec<String>>,
    ) -> &Command {
        let command_id = self.gen_command_id();
        let command = Command {
            command_id: command_id.clone(),
            instance_ids,
            document_name: document_name.to_string(),
            status: "Pending".to_string(),
            requested_date_time: Utc::now(),
            parameters,
        };
        self.commands.insert(command_id.clone(), command);
        self.commands.get(&command_id).unwrap()
    }

    pub fn list_commands(&self) -> Vec<&Command> {
        self.commands.values().collect()
    }

    pub fn get_command_invocation(
        &self,
        command_id: &str,
        instance_id: &str,
    ) -> Result<&Command, SsmError> {
        let cmd = self
            .commands
            .get(command_id)
            .ok_or_else(|| SsmError::InvalidCommandId(command_id.to_string()))?;
        if !cmd.instance_ids.contains(&instance_id.to_string()) {
            return Err(SsmError::InvocationDoesNotExist(instance_id.to_string()));
        }
        Ok(cmd)
    }

    // ── OpsItem operations ────────────────────────────────────────────

    pub fn create_ops_item(
        &mut self,
        title: &str,
        source: &str,
        description: Option<&str>,
        priority: Option<i32>,
        category: Option<&str>,
        severity: Option<&str>,
        ops_item_type: Option<&str>,
    ) -> &OpsItem {
        let ops_item_id = self.gen_ops_item_id();
        let now = Utc::now();
        let item = OpsItem {
            ops_item_id: ops_item_id.clone(),
            title: title.to_string(),
            description: description.map(|s| s.to_string()),
            source: source.to_string(),
            status: "Open".to_string(),
            priority,
            category: category.map(|s| s.to_string()),
            severity: severity.map(|s| s.to_string()),
            ops_item_type: ops_item_type.map(|s| s.to_string()),
            created_time: now,
            last_modified_time: now,
        };
        self.ops_items.insert(ops_item_id.clone(), item);
        self.ops_items.get(&ops_item_id).unwrap()
    }

    pub fn get_ops_item(&self, ops_item_id: &str) -> Result<&OpsItem, SsmError> {
        self.ops_items
            .get(ops_item_id)
            .ok_or_else(|| SsmError::OpsItemNotFound(ops_item_id.to_string()))
    }

    pub fn update_ops_item(
        &mut self,
        ops_item_id: &str,
        title: Option<&str>,
        status: Option<&str>,
        priority: Option<i32>,
        description: Option<&str>,
    ) -> Result<(), SsmError> {
        let item = self
            .ops_items
            .get_mut(ops_item_id)
            .ok_or_else(|| SsmError::OpsItemNotFound(ops_item_id.to_string()))?;
        if let Some(t) = title {
            item.title = t.to_string();
        }
        if let Some(s) = status {
            item.status = s.to_string();
        }
        if let Some(p) = priority {
            item.priority = Some(p);
        }
        if let Some(d) = description {
            item.description = Some(d.to_string());
        }
        item.last_modified_time = Utc::now();
        Ok(())
    }

    pub fn delete_ops_item(&mut self, ops_item_id: &str) -> Result<(), SsmError> {
        self.ops_items
            .remove(ops_item_id)
            .ok_or_else(|| SsmError::OpsItemNotFound(ops_item_id.to_string()))?;
        Ok(())
    }

    pub fn describe_ops_items(&self) -> Vec<&OpsItem> {
        self.ops_items.values().collect()
    }

    // ── Session Manager operations ────────────────────────────────────

    pub fn start_session(&mut self, target: &str, document_name: Option<&str>) -> &Session {
        let session_id = self.gen_session_id();
        let now = Utc::now();
        let session = Session {
            session_id: session_id.clone(),
            target: target.to_string(),
            status: "Active".to_string(),
            start_date: now,
            end_date: None,
            document_name: document_name.map(|s| s.to_string()),
        };
        self.sessions.insert(session_id.clone(), session);
        self.sessions.get(&session_id).unwrap()
    }

    pub fn describe_sessions(&self, state_filter: Option<&str>) -> Vec<&Session> {
        self.sessions
            .values()
            .filter(|s| {
                if let Some(filter) = state_filter {
                    s.status.to_lowercase() == filter.to_lowercase()
                } else {
                    true
                }
            })
            .collect()
    }

    pub fn resume_session(&self, session_id: &str) -> Result<&Session, SsmError> {
        self.sessions
            .get(session_id)
            .ok_or_else(|| SsmError::SessionDoesNotExist(session_id.to_string()))
    }

    pub fn terminate_session(&mut self, session_id: &str) -> Result<(), SsmError> {
        let session = self
            .sessions
            .get_mut(session_id)
            .ok_or_else(|| SsmError::SessionDoesNotExist(session_id.to_string()))?;
        session.status = "Terminated".to_string();
        session.end_date = Some(Utc::now());
        Ok(())
    }

    // ── Managed Instance Activation operations ────────────────────────

    pub fn create_activation(
        &mut self,
        iam_role: &str,
        description: Option<&str>,
        default_instance_name: Option<&str>,
        registration_limit: Option<i32>,
        expiration_date: Option<DateTime<Utc>>,
    ) -> &Activation {
        let activation_id = self.gen_activation_id();
        let activation_code = format!("code-{activation_id}");
        let now = Utc::now();
        let activation = Activation {
            activation_id: activation_id.clone(),
            activation_code,
            iam_role: iam_role.to_string(),
            description: description.map(|s| s.to_string()),
            default_instance_name: default_instance_name.map(|s| s.to_string()),
            registration_limit,
            registrations_count: 0,
            expiration_date,
            expired: false,
            created_date: now,
        };
        self.activations.insert(activation_id.clone(), activation);
        self.activations.get(&activation_id).unwrap()
    }

    pub fn describe_activations(&self) -> Vec<&Activation> {
        self.activations.values().collect()
    }

    pub fn delete_activation(&mut self, activation_id: &str) -> Result<(), SsmError> {
        self.activations
            .remove(activation_id)
            .ok_or_else(|| SsmError::InvalidActivationId(activation_id.to_string()))?;
        Ok(())
    }

    // ── Resource Policy operations ────────────────────────────────────

    pub fn get_resource_policies(&self, resource_arn: &str) -> Vec<&ResourcePolicy> {
        self.resource_policies
            .get(resource_arn)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    pub fn put_resource_policy(
        &mut self,
        resource_arn: &str,
        policy: &str,
        policy_id: Option<&str>,
    ) -> &ResourcePolicy {
        let id = policy_id
            .map(|s| s.to_string())
            .unwrap_or_else(|| self.gen_policy_id());
        let policy_hash = format!("{:x}", id.len());
        let entry = ResourcePolicy {
            policy_id: id.clone(),
            policy_hash,
            policy: policy.to_string(),
        };
        let policies = self
            .resource_policies
            .entry(resource_arn.to_string())
            .or_default();
        if let Some(pos) = policies.iter().position(|p| p.policy_id == id) {
            policies[pos] = entry;
            &policies[pos]
        } else {
            policies.push(entry);
            policies.last().unwrap()
        }
    }

    pub fn delete_resource_policy(
        &mut self,
        resource_arn: &str,
        policy_id: &str,
    ) -> Result<(), SsmError> {
        if let Some(policies) = self.resource_policies.get_mut(resource_arn) {
            policies.retain(|p| p.policy_id != policy_id);
        }
        Ok(())
    }

    // ── Service Setting operations ────────────────────────────────────

    pub fn get_service_setting(&self, setting_id: &str) -> Option<&ServiceSetting> {
        self.service_settings.get(setting_id)
    }

    pub fn update_service_setting(&mut self, setting_id: &str, setting_value: &str, arn: &str) {
        let now = Utc::now();
        let setting = ServiceSetting {
            setting_id: setting_id.to_string(),
            setting_value: setting_value.to_string(),
            last_modified_time: now,
            arn: arn.to_string(),
        };
        self.service_settings
            .insert(setting_id.to_string(), setting);
    }

    pub fn reset_service_setting(&mut self, setting_id: &str) -> Option<&ServiceSetting> {
        self.service_settings.remove(setting_id);
        None
    }

    // ── Inventory operations ──────────────────────────────────────────

    pub fn put_inventory(
        &mut self,
        instance_id: &str,
        type_name: &str,
        capture_time: &str,
        schema_version: &str,
        content: Vec<HashMap<String, String>>,
    ) {
        let data = InventoryData {
            instance_id: instance_id.to_string(),
            type_name: type_name.to_string(),
            capture_time: capture_time.to_string(),
            schema_version: schema_version.to_string(),
            content,
        };
        self.inventory
            .insert((instance_id.to_string(), type_name.to_string()), data);
    }

    pub fn get_inventory_entries(
        &self,
        instance_id: &str,
        type_name: &str,
    ) -> Option<&InventoryData> {
        self.inventory
            .get(&(instance_id.to_string(), type_name.to_string()))
    }

    // ── Compliance operations ─────────────────────────────────────────

    pub fn put_compliance_items(
        &mut self,
        resource_id: &str,
        resource_type: &str,
        compliance_type: &str,
        execution_type: &str,
        execution_id: &str,
        items: Vec<(String, String, String, String, HashMap<String, String>)>,
    ) {
        let now = Utc::now();
        for (item_id, title, status, severity, details) in items {
            let record = ComplianceRecord {
                compliance_type: compliance_type.to_string(),
                resource_id: resource_id.to_string(),
                resource_type: resource_type.to_string(),
                status: status.clone(),
                severity: severity.clone(),
                execution_type: execution_type.to_string(),
                execution_id: execution_id.to_string(),
                execution_time: now,
                item_id: item_id.clone(),
                title: title.clone(),
                details,
            };
            self.compliance_records.insert(
                (
                    resource_id.to_string(),
                    compliance_type.to_string(),
                    item_id,
                ),
                record,
            );
        }
    }

    pub fn list_compliance_items(
        &self,
        resource_ids: &[String],
        compliance_types: &[String],
    ) -> Vec<&ComplianceRecord> {
        self.compliance_records
            .values()
            .filter(|r| {
                (resource_ids.is_empty() || resource_ids.iter().any(|id| id == &r.resource_id))
                    && (compliance_types.is_empty()
                        || compliance_types.iter().any(|ct| ct == &r.compliance_type))
            })
            .collect()
    }

    pub fn list_compliance_summaries(
        &self,
        compliance_types: &[String],
    ) -> Vec<(String, usize, usize)> {
        // Returns (compliance_type, compliant_count, non_compliant_count)
        let mut map: HashMap<String, (usize, usize)> = HashMap::new();
        for record in self.compliance_records.values() {
            if !compliance_types.is_empty()
                && !compliance_types
                    .iter()
                    .any(|ct| ct == &record.compliance_type)
            {
                continue;
            }
            let entry = map.entry(record.compliance_type.clone()).or_default();
            if record.status.to_lowercase() == "compliant" {
                entry.0 += 1;
            } else {
                entry.1 += 1;
            }
        }
        map.into_iter().map(|(ct, (c, nc))| (ct, c, nc)).collect()
    }

    // ── OpsMetadata operations ────────────────────────────────────────

    pub fn create_ops_metadata(
        &mut self,
        resource_id: &str,
        metadata: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> &OpsMetadataEntry {
        let arn = self.gen_ops_metadata_arn(account_id, region);
        let now = Utc::now();
        let entry = OpsMetadataEntry {
            ops_metadata_arn: arn.clone(),
            resource_id: resource_id.to_string(),
            metadata,
            created_date: now,
            last_modified_date: now,
        };
        self.ops_metadata.insert(arn.clone(), entry);
        self.ops_metadata.get(&arn).unwrap()
    }

    pub fn get_ops_metadata(&self, ops_metadata_arn: &str) -> Option<&OpsMetadataEntry> {
        self.ops_metadata.get(ops_metadata_arn)
    }

    pub fn update_ops_metadata(
        &mut self,
        ops_metadata_arn: &str,
        metadata_to_update: HashMap<String, String>,
        keys_to_delete: Vec<String>,
    ) -> Result<(), SsmError> {
        let entry = self
            .ops_metadata
            .get_mut(ops_metadata_arn)
            .ok_or_else(|| SsmError::OpsMetadataNotFound(ops_metadata_arn.to_string()))?;
        entry.metadata.extend(metadata_to_update);
        for key in &keys_to_delete {
            entry.metadata.remove(key);
        }
        entry.last_modified_date = Utc::now();
        Ok(())
    }

    pub fn delete_ops_metadata(&mut self, ops_metadata_arn: &str) -> Result<(), SsmError> {
        self.ops_metadata
            .remove(ops_metadata_arn)
            .ok_or_else(|| SsmError::OpsMetadataNotFound(ops_metadata_arn.to_string()))?;
        Ok(())
    }

    pub fn list_ops_metadata(&self) -> Vec<&OpsMetadataEntry> {
        self.ops_metadata.values().collect()
    }

    // ── Resource Data Sync operations ─────────────────────────────────

    pub fn create_resource_data_sync(
        &mut self,
        sync_name: &str,
        sync_type: &str,
        s3_destination_bucket: &str,
        s3_destination_region: &str,
        s3_destination_prefix: Option<&str>,
    ) -> Result<(), SsmError> {
        if self.resource_data_syncs.contains_key(sync_name) {
            return Err(SsmError::ResourceDataSyncAlreadyExists(
                sync_name.to_string(),
            ));
        }
        let entry = ResourceDataSyncEntry {
            sync_name: sync_name.to_string(),
            sync_type: sync_type.to_string(),
            s3_destination_bucket: s3_destination_bucket.to_string(),
            s3_destination_region: s3_destination_region.to_string(),
            s3_destination_prefix: s3_destination_prefix.map(|s| s.to_string()),
            created_time: Utc::now(),
            last_status: "InProgress".to_string(),
        };
        self.resource_data_syncs
            .insert(sync_name.to_string(), entry);
        Ok(())
    }

    pub fn delete_resource_data_sync(&mut self, sync_name: &str) -> Result<(), SsmError> {
        self.resource_data_syncs
            .remove(sync_name)
            .ok_or_else(|| SsmError::ResourceDataSyncNotFound(sync_name.to_string()))?;
        Ok(())
    }

    pub fn list_resource_data_syncs(&self) -> Vec<&ResourceDataSyncEntry> {
        self.resource_data_syncs.values().collect()
    }

    pub fn update_resource_data_sync(
        &mut self,
        sync_name: &str,
        sync_type: &str,
    ) -> Result<(), SsmError> {
        let entry = self
            .resource_data_syncs
            .get_mut(sync_name)
            .ok_or_else(|| SsmError::ResourceDataSyncNotFound(sync_name.to_string()))?;
        entry.sync_type = sync_type.to_string();
        Ok(())
    }

    // ── OpsItem Related Item operations ──────────────────────────────

    pub fn associate_ops_item_related_item(
        &mut self,
        ops_item_id: &str,
        association_type: &str,
        resource_type: &str,
        resource_uri: &str,
    ) -> Result<&OpsItemRelatedItem, SsmError> {
        // Verify OpsItem exists
        if !self.ops_items.contains_key(ops_item_id) {
            return Err(SsmError::OpsItemNotFound(ops_item_id.to_string()));
        }
        let association_id = self.gen_ops_item_related_id();
        let item = OpsItemRelatedItem {
            association_id: association_id.clone(),
            ops_item_id: ops_item_id.to_string(),
            association_type: association_type.to_string(),
            resource_type: resource_type.to_string(),
            resource_uri: resource_uri.to_string(),
            created_time: Utc::now(),
        };
        self.ops_item_related_items
            .insert(association_id.clone(), item);
        Ok(self.ops_item_related_items.get(&association_id).unwrap())
    }

    pub fn disassociate_ops_item_related_item(
        &mut self,
        ops_item_id: &str,
        association_id: &str,
    ) -> Result<(), SsmError> {
        if !self.ops_items.contains_key(ops_item_id) {
            return Err(SsmError::OpsItemNotFound(ops_item_id.to_string()));
        }
        self.ops_item_related_items.remove(association_id);
        Ok(())
    }

    pub fn list_ops_item_related_items(
        &self,
        ops_item_id: Option<&str>,
    ) -> Vec<&OpsItemRelatedItem> {
        self.ops_item_related_items
            .values()
            .filter(|item| {
                if let Some(id) = ops_item_id {
                    item.ops_item_id == id
                } else {
                    true
                }
            })
            .collect()
    }

    // ── Command cancel operation ─────────────────────────────────────

    pub fn cancel_command(
        &mut self,
        command_id: &str,
        instance_ids: Option<&[String]>,
    ) -> Result<(), SsmError> {
        let cmd = self
            .commands
            .get_mut(command_id)
            .ok_or_else(|| SsmError::InvalidCommandId(command_id.to_string()))?;
        // If instance_ids provided, only cancel for those; otherwise cancel the whole command
        let _ = instance_ids;
        cmd.status = "Cancelled".to_string();
        Ok(())
    }

    // ── Managed Instance operations ──────────────────────────────────

    pub fn register_managed_instance(
        &mut self,
        activation_id: &str,
    ) -> Result<&ManagedInstance, SsmError> {
        let activation = self
            .activations
            .get_mut(activation_id)
            .ok_or_else(|| SsmError::InvalidActivationId(activation_id.to_string()))?;
        activation.registrations_count += 1;
        let iam_role = activation.iam_role.clone();
        let instance_name = activation.default_instance_name.clone().unwrap_or_default();

        let instance_id = self.gen_managed_instance_id();
        let now = Utc::now();
        let mi = ManagedInstance {
            instance_id: instance_id.clone(),
            ping_status: "Online".to_string(),
            platform_type: "Linux".to_string(),
            platform_name: "Amazon Linux".to_string(),
            platform_version: "2".to_string(),
            activation_id: Some(activation_id.to_string()),
            iam_role: Some(iam_role),
            registration_date: now,
            resource_type: "ManagedInstance".to_string(),
            ip_address: "10.0.0.1".to_string(),
            computer_name: instance_name,
            is_latest_version: true,
            last_ping_date_time: now,
        };
        self.managed_instances.insert(instance_id.clone(), mi);
        Ok(self.managed_instances.get(&instance_id).unwrap())
    }

    pub fn deregister_managed_instance(&mut self, instance_id: &str) -> Result<(), SsmError> {
        self.managed_instances.remove(instance_id);
        Ok(())
    }

    pub fn describe_instance_information(&self) -> Vec<&ManagedInstance> {
        self.managed_instances.values().collect()
    }

    pub fn get_connection_status(&self, target: &str) -> String {
        if self.managed_instances.contains_key(target) {
            "Connected".to_string()
        } else {
            "NotConnected".to_string()
        }
    }

    pub fn update_managed_instance_role(
        &mut self,
        instance_id: &str,
        iam_role: &str,
    ) -> Result<(), SsmError> {
        if let Some(mi) = self.managed_instances.get_mut(instance_id) {
            mi.iam_role = Some(iam_role.to_string());
        }
        Ok(())
    }

    // ── Inventory deletion operations ────────────────────────────────

    pub fn delete_inventory(&mut self, type_name: &str) -> &InventoryDeletion {
        let deletion_id = self.gen_deletion_id();
        // Remove matching inventory entries
        self.inventory.retain(|(_, tn), _| tn != type_name);
        let now = Utc::now();
        let deletion = InventoryDeletion {
            deletion_id: deletion_id.clone(),
            type_name: type_name.to_string(),
            deletion_start_time: now,
            last_status: "Complete".to_string(),
            last_status_message: "Deletion complete".to_string(),
        };
        self.inventory_deletions
            .insert(deletion_id.clone(), deletion);
        self.inventory_deletions.get(&deletion_id).unwrap()
    }

    pub fn describe_inventory_deletions(&self) -> Vec<&InventoryDeletion> {
        self.inventory_deletions.values().collect()
    }

    // ── Maintenance Window Execution operations ──────────────────────

    pub fn cancel_maintenance_window_execution(
        &mut self,
        window_execution_id: &str,
    ) -> Result<(), SsmError> {
        if let Some(exec) = self
            .maintenance_window_executions
            .get_mut(window_execution_id)
        {
            exec.status = "CANCELLED".to_string();
            exec.end_time = Some(Utc::now());
            Ok(())
        } else {
            Ok(()) // AWS does not error on unknown execution IDs
        }
    }

    pub fn describe_maintenance_window_executions(
        &self,
        window_id: &str,
    ) -> Vec<&MaintenanceWindowExecution> {
        self.maintenance_window_executions
            .values()
            .filter(|e| e.window_id == window_id)
            .collect()
    }

    pub fn get_maintenance_window_execution(
        &self,
        window_execution_id: &str,
    ) -> Option<&MaintenanceWindowExecution> {
        self.maintenance_window_executions.get(window_execution_id)
    }

    pub fn describe_maintenance_window_execution_tasks(
        &self,
        window_execution_id: &str,
    ) -> Vec<&MaintenanceWindowExecutionTask> {
        if let Some(exec) = self.maintenance_window_executions.get(window_execution_id) {
            exec.tasks.iter().collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_maintenance_window_execution_task(
        &self,
        window_execution_id: &str,
        task_execution_id: &str,
    ) -> Option<&MaintenanceWindowExecutionTask> {
        let exec = self
            .maintenance_window_executions
            .get(window_execution_id)?;
        exec.tasks
            .iter()
            .find(|t| t.task_execution_id == task_execution_id)
    }

    pub fn describe_maintenance_window_execution_task_invocations(
        &self,
        window_execution_id: &str,
        task_execution_id: &str,
    ) -> Vec<&MaintenanceWindowExecutionTaskInvocation> {
        if let Some(exec) = self.maintenance_window_executions.get(window_execution_id) {
            if let Some(task) = exec
                .tasks
                .iter()
                .find(|t| t.task_execution_id == task_execution_id)
            {
                return task.invocations.iter().collect();
            }
        }
        Vec::new()
    }

    pub fn get_maintenance_window_execution_task_invocation(
        &self,
        window_execution_id: &str,
        task_execution_id: &str,
        invocation_id: &str,
    ) -> Option<&MaintenanceWindowExecutionTaskInvocation> {
        let exec = self
            .maintenance_window_executions
            .get(window_execution_id)?;
        let task = exec
            .tasks
            .iter()
            .find(|t| t.task_execution_id == task_execution_id)?;
        task.invocations
            .iter()
            .find(|i| i.invocation_id == invocation_id)
    }

    pub fn describe_maintenance_window_schedule(
        &self,
        _window_id: Option<&str>,
    ) -> Vec<(&MaintenanceWindow, String)> {
        // Return scheduled entries from maintenance windows.
        // Each window gets a single scheduled entry with its schedule expression.
        self.maintenance_windows
            .values()
            .filter(|w| w.enabled)
            .map(|w| (w, w.schedule.clone()))
            .collect()
    }

    pub fn describe_maintenance_windows_for_target(
        &self,
        targets: &[Target],
    ) -> Vec<&MaintenanceWindow> {
        self.maintenance_windows
            .values()
            .filter(|w| {
                w.targets.iter().any(|wt| {
                    wt.targets.iter().any(|t| {
                        targets.iter().any(|input_t| {
                            t.key == input_t.key
                                && t.values.iter().any(|v| input_t.values.contains(v))
                        })
                    })
                })
            })
            .collect()
    }

    pub fn update_maintenance_window_target(
        &mut self,
        window_id: &str,
        window_target_id: &str,
        name: Option<&str>,
        description: Option<&str>,
        targets: Option<Vec<Target>>,
    ) -> Result<&MaintenanceWindowTarget, SsmError> {
        let window = self
            .maintenance_windows
            .get_mut(window_id)
            .ok_or_else(|| SsmError::MaintenanceWindowDoesNotExist(window_id.to_string()))?;
        let target = window
            .targets
            .iter_mut()
            .find(|t| t.window_target_id == window_target_id)
            .ok_or_else(|| SsmError::MaintenanceWindowDoesNotExist(window_target_id.to_string()))?;
        // name/description are not fields on MaintenanceWindowTarget, but targets can be updated
        let _ = name;
        let _ = description;
        if let Some(new_targets) = targets {
            target.targets = new_targets;
        }
        Ok(target)
    }

    // ── Document metadata operations ─────────────────────────────────

    pub fn update_document_metadata(
        &mut self,
        name: &str,
        status: &str,
        comment: &str,
    ) -> Result<(), SsmError> {
        if !self.documents.contains_key(name) {
            return Err(SsmError::InvalidDocument(name.to_string()));
        }
        let entry = DocumentMetadataEntry {
            reviewer: "user".to_string(),
            status: status.to_string(),
            comment: comment.to_string(),
            updated_date: Utc::now(),
        };
        self.document_metadata
            .entry(name.to_string())
            .or_default()
            .push(entry);
        Ok(())
    }

    pub fn list_document_metadata_history(
        &self,
        name: &str,
    ) -> Result<&Vec<DocumentMetadataEntry>, SsmError> {
        if !self.documents.contains_key(name) {
            return Err(SsmError::InvalidDocument(name.to_string()));
        }
        static EMPTY: Vec<DocumentMetadataEntry> = Vec::new();
        Ok(self.document_metadata.get(name).unwrap_or(&EMPTY))
    }

    // ── Association version / status operations ──────────────────────

    pub fn list_association_versions(
        &self,
        association_id: &str,
    ) -> Result<Vec<&Association>, SsmError> {
        let assoc = self
            .associations
            .get(association_id)
            .ok_or_else(|| SsmError::AssociationDoesNotExist(association_id.to_string()))?;
        // We only store current version; return it as the sole version entry
        Ok(vec![assoc])
    }

    pub fn update_association_status(
        &mut self,
        name: &str,
        instance_id: &str,
        status: &str,
        detailed_status: &str,
    ) -> Result<Option<&Association>, SsmError> {
        // Find association by document name, then update overview status
        let _ = instance_id;
        let assoc = self.associations.values_mut().find(|a| a.name == name);
        if let Some(a) = assoc {
            a.overview.status = status.to_string();
            a.overview.detailed_status = detailed_status.to_string();
            a.last_execution_date = Some(Utc::now());
            let id = a.association_id.clone();
            Ok(self.associations.get(&id))
        } else {
            Ok(None)
        }
    }

    // ── OpsItem events operation ─────────────────────────────────────

    pub fn list_ops_item_events(&self, ops_item_id: Option<&str>) -> Vec<&OpsItem> {
        // Return OpsItems as "events" filtered by ID
        self.ops_items
            .values()
            .filter(|item| {
                if let Some(id) = ops_item_id {
                    item.ops_item_id == id
                } else {
                    true
                }
            })
            .collect()
    }

    // ── GetOpsSummary operation ───────────────────────────────────────

    pub fn get_ops_summary(&self) -> Vec<&OpsItem> {
        self.ops_items.values().collect()
    }

    // ── GetInventorySchema operation ─────────────────────────────────

    pub fn get_inventory_schema(&self) -> Vec<String> {
        // Return unique type_names from inventory data
        let mut type_names: Vec<String> = self.inventory.keys().map(|(_, tn)| tn.clone()).collect();
        type_names.sort();
        type_names.dedup();
        type_names
    }
}
