use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct LambdaState {
    pub functions: HashMap<String, LambdaFunction>,
    pub aliases: HashMap<String, Alias>, // key: "{function_name}:{alias_name}"
    pub event_source_mappings: HashMap<String, EventSourceMapping>, // key: uuid
    pub layers: HashMap<String, Vec<LayerVersion>>, // key: layer_name -> versions
    pub layer_next_version: HashMap<String, i64>, // key: layer_name -> next version number
    pub function_url_configs: HashMap<String, FunctionUrlConfig>, // key: function_name
    pub function_permissions: HashMap<String, Vec<Permission>>, // key: function_name
    /// Per-function resource-policy revision id.  Bumped on every successful
    /// `AddPermission` / `RemovePermission` so callers can use the modelled
    /// `RevisionId` optimistic-concurrency check.  Absent until the first
    /// permission is added, cleared when the function is deleted.
    pub function_policy_revisions: HashMap<String, String>, // key: function_name
    pub function_event_invoke_configs: HashMap<String, FunctionEventInvokeConfig>, // key: function_name
    pub account_id: String,
    pub region: String,
    // New fields for additional operations
    pub code_signing_configs: HashMap<String, CodeSigningConfigEntry>, // key: csc_id
    pub provisioned_concurrency: HashMap<String, ProvisionedConcurrencyConfig>, // key: "functionName:qualifier"
    pub capacity_providers: HashMap<String, CapacityProviderEntry>,             // key: name
    pub function_recursion_configs: HashMap<String, FunctionRecursionConfig>, // key: function_name
    pub function_scaling_configs: HashMap<String, FunctionScalingConfig>,     // key: function_name
    pub runtime_management_configs: HashMap<String, RuntimeManagementConfig>, // key: function_name
    pub csc_id_counter: u64,
    pub cp_id_counter: u64,
    /// Shared empty tag map for resources that carry no tags in the mock model.
    pub empty_tags: HashMap<String, String>,
    /// Durable executions keyed by execution ARN.
    pub durable_executions: HashMap<String, DurableExecution>,
}

#[derive(Debug, thiserror::Error)]
pub enum LambdaError {
    #[error("Function already exist: {0}")]
    FunctionAlreadyExists(String),
    #[error("Function not found: arn:aws:lambda:{region}:{account_id}:function:{name}")]
    FunctionNotFoundByArn {
        region: String,
        account_id: String,
        name: String,
    },
    #[error("Function not found: {0}")]
    FunctionNotFound(String),
    #[error("Alias already exists: {0}")]
    AliasAlreadyExists(String),
    #[error("Cannot find alias arn: {0}")]
    AliasNotFoundByArn(String),
    #[error("Alias not found: {0}")]
    AliasNotFound(String),
    #[error("Event source mapping not found: {0}")]
    EventSourceMappingNotFound(String),
    #[error("Layer not found: {0}")]
    LayerNotFound(String),
    #[error("Layer version not found: {0}")]
    LayerVersionNotFound(String),
    #[error("Permission already exists: {0}")]
    PermissionAlreadyExists(String),
    #[error("Permission not found: {0}")]
    PermissionNotFound(String),
    #[error("The resource you requested does not exist.")]
    ResourceDoesNotExist,
    #[error("Function URL config already exists for: {0}")]
    FunctionUrlConfigAlreadyExists(String),
    #[error("Function URL config not found for: {0}")]
    FunctionUrlConfigNotFound(String),
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
    #[error("Event invoke config not found for: {0}")]
    EventInvokeConfigNotFound(String),
    #[error("Code signing config not found: {0}")]
    CodeSigningConfigNotFound(String),
    #[error("No provisioned concurrency config found for: {0}")]
    ProvisionedConcurrencyConfigNotFound(String),
    #[error("Capacity provider not found: {0}")]
    CapacityProviderNotFound(String),
    #[error("Durable execution not found: {0}")]
    DurableExecutionNotFound(String),
    #[error(
        "The Revision Id provided does not match the latest Revision Id. Call the GetFunction/GetAlias API to retrieve the latest Revision Id"
    )]
    PreconditionFailed,
}

impl LambdaState {
    pub fn create_function(
        &mut self,
        name: &str,
        runtime: &str,
        handler: &str,
        role: &str,
        description: &str,
        memory_size: Option<i32>,
        timeout: Option<i32>,
        environment: HashMap<String, String>,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
        code_sha256: &str,
        code_size: i64,
    ) -> Result<&LambdaFunction, LambdaError> {
        if self.functions.contains_key(name) {
            return Err(LambdaError::FunctionAlreadyExists(name.to_string()));
        }

        self.account_id = account_id.to_string();
        self.region = region.to_string();

        let arn = format!("arn:aws:lambda:{region}:{account_id}:function:{name}");

        let func = LambdaFunction {
            function_name: name.to_string(),
            function_arn: arn,
            runtime: runtime.to_string(),
            handler: handler.to_string(),
            role: role.to_string(),
            code_sha256: code_sha256.to_string(),
            code_size,
            memory_size: memory_size.unwrap_or(128),
            timeout: timeout.unwrap_or(3),
            environment,
            description: description.to_string(),
            last_modified: Utc::now(),
            state: "Active".to_string(),
            version: "$LATEST".to_string(),
            revision_id: uuid::Uuid::new_v4().to_string(),
            tags,
            versions: Vec::new(),
            reserved_concurrent_executions: None,
            code_signing_config_arn: None,
            dead_letter_config: None,
            ephemeral_storage: None,
            file_system_config: None,
            image_config: None,
            logging_config: None,
            snap_start: None,
            tracing_config: None,
            vpc_config: None,
        };

        self.functions.insert(name.to_string(), func);
        Ok(self.functions.get(name).unwrap())
    }

    pub fn get_function(&self, name: &str) -> Result<&LambdaFunction, LambdaError> {
        let resolved = self.resolve_name(name);
        let region = if self.region.is_empty() {
            "us-east-1"
        } else {
            &self.region
        };
        let account_id = if self.account_id.is_empty() {
            "123456789012"
        } else {
            &self.account_id
        };
        self.functions
            .get(&resolved)
            .ok_or_else(|| LambdaError::FunctionNotFoundByArn {
                region: region.to_string(),
                account_id: account_id.to_string(),
                name: resolved.clone(),
            })
    }

    pub fn get_function_mut(&mut self, name: &str) -> Result<&mut LambdaFunction, LambdaError> {
        let resolved = self.resolve_name(name);
        let region = if self.region.is_empty() {
            "us-east-1"
        } else {
            &self.region
        };
        let account_id = if self.account_id.is_empty() {
            "123456789012"
        } else {
            &self.account_id
        };
        self.functions
            .get_mut(&resolved)
            .ok_or_else(|| LambdaError::FunctionNotFoundByArn {
                region: region.to_string(),
                account_id: account_id.to_string(),
                name: resolved.clone(),
            })
    }

    pub fn delete_function(&mut self, name: &str) -> Result<(), LambdaError> {
        let resolved = self.resolve_name(name);
        if self.functions.remove(&resolved).is_none() {
            return Err(LambdaError::FunctionNotFound(resolved.clone()));
        }
        // Clean up related resources
        self.aliases
            .retain(|k, _| !k.starts_with(&format!("{resolved}:")));
        self.function_url_configs.remove(&resolved);
        self.function_permissions.remove(&resolved);
        self.function_policy_revisions.remove(&resolved);
        self.function_event_invoke_configs.remove(&resolved);
        Ok(())
    }

    pub fn list_functions(&self) -> Vec<&LambdaFunction> {
        self.functions.values().collect()
    }

    pub fn update_function_configuration(
        &mut self,
        name: &str,
        description: Option<&str>,
        handler: Option<&str>,
        memory_size: Option<i32>,
        timeout: Option<i32>,
        runtime: Option<&str>,
        environment: Option<HashMap<String, String>>,
        expected_revision_id: Option<&str>,
    ) -> Result<&LambdaFunction, LambdaError> {
        let resolved = self.resolve_name(name);
        let func = self
            .functions
            .get_mut(&resolved)
            .ok_or_else(|| LambdaError::FunctionNotFound(resolved.clone()))?;

        // Optimistic concurrency on the function's configuration revision.
        if let Some(expected) = expected_revision_id.filter(|s| !s.is_empty())
            && expected != func.revision_id
        {
            return Err(LambdaError::PreconditionFailed);
        }

        if let Some(d) = description {
            func.description = d.to_string();
        }
        if let Some(h) = handler {
            func.handler = h.to_string();
        }
        if let Some(m) = memory_size {
            func.memory_size = m;
        }
        if let Some(t) = timeout {
            func.timeout = t;
        }
        if let Some(r) = runtime {
            func.runtime = r.to_string();
        }
        if let Some(e) = environment {
            func.environment = e;
        }
        func.last_modified = Utc::now();
        func.revision_id = uuid::Uuid::new_v4().to_string();

        Ok(self.functions.get(&resolved).unwrap())
    }

    pub fn update_function_code(
        &mut self,
        name: &str,
        expected_revision_id: Option<&str>,
    ) -> Result<&LambdaFunction, LambdaError> {
        let resolved = self.resolve_name(name);
        let func = self
            .functions
            .get_mut(&resolved)
            .ok_or_else(|| LambdaError::FunctionNotFound(resolved.clone()))?;

        if let Some(expected) = expected_revision_id.filter(|s| !s.is_empty())
            && expected != func.revision_id
        {
            return Err(LambdaError::PreconditionFailed);
        }

        func.code_sha256 = mock_code_sha256(&uuid::Uuid::new_v4().to_string());
        func.last_modified = Utc::now();
        func.revision_id = uuid::Uuid::new_v4().to_string();

        Ok(self.functions.get(&resolved).unwrap())
    }

    // ========== Alias operations ==========

    pub fn create_alias(
        &mut self,
        function_name: &str,
        alias_name: &str,
        function_version: &str,
        description: &str,
        routing_config: Option<HashMap<String, f64>>,
    ) -> Result<&Alias, LambdaError> {
        let resolved = self.resolve_name(function_name);
        let func = self
            .functions
            .get(&resolved)
            .ok_or_else(|| LambdaError::FunctionNotFound(resolved.clone()))?;

        let key = format!("{resolved}:{alias_name}");
        if self.aliases.contains_key(&key) {
            return Err(LambdaError::AliasAlreadyExists(format!(
                "{}:{alias_name}",
                func.function_arn
            )));
        }

        let alias_arn = format!("{}:{alias_name}", func.function_arn);
        let alias = Alias {
            name: alias_name.to_string(),
            function_name: resolved.clone(),
            function_version: function_version.to_string(),
            description: description.to_string(),
            alias_arn,
            revision_id: uuid::Uuid::new_v4().to_string(),
            routing_config,
        };

        self.aliases.insert(key.clone(), alias);
        Ok(self.aliases.get(&key).unwrap())
    }

    pub fn get_alias(&self, function_name: &str, alias_name: &str) -> Result<&Alias, LambdaError> {
        let resolved = self.resolve_name(function_name);
        // Ensure function exists
        let func = self.get_function(&resolved)?;
        let key = format!("{resolved}:{alias_name}");
        self.aliases.get(&key).ok_or_else(|| {
            LambdaError::AliasNotFoundByArn(format!("{}:{alias_name}", func.function_arn))
        })
    }

    pub fn update_alias(
        &mut self,
        function_name: &str,
        alias_name: &str,
        function_version: Option<&str>,
        description: Option<&str>,
        routing_config: Option<Option<HashMap<String, f64>>>,
        expected_revision_id: Option<&str>,
    ) -> Result<&Alias, LambdaError> {
        let resolved = self.resolve_name(function_name);
        let key = format!("{resolved}:{alias_name}");
        let alias = self
            .aliases
            .get_mut(&key)
            .ok_or_else(|| LambdaError::AliasNotFound(alias_name.to_string()))?;

        if let Some(expected) = expected_revision_id.filter(|s| !s.is_empty())
            && expected != alias.revision_id
        {
            return Err(LambdaError::PreconditionFailed);
        }

        if let Some(v) = function_version {
            alias.function_version = v.to_string();
        }
        if let Some(d) = description {
            alias.description = d.to_string();
        }
        if let Some(rc) = routing_config {
            alias.routing_config = rc;
        }
        alias.revision_id = uuid::Uuid::new_v4().to_string();

        Ok(self.aliases.get(&key).unwrap())
    }

    pub fn delete_alias(
        &mut self,
        function_name: &str,
        alias_name: &str,
    ) -> Result<(), LambdaError> {
        let resolved = self.resolve_name(function_name);
        let key = format!("{resolved}:{alias_name}");
        if self.aliases.remove(&key).is_none() {
            return Err(LambdaError::AliasNotFound(alias_name.to_string()));
        }
        Ok(())
    }

    pub fn list_aliases(&self, function_name: &str) -> Result<Vec<&Alias>, LambdaError> {
        let resolved = self.resolve_name(function_name);
        self.get_function(&resolved)?;
        Ok(self
            .aliases
            .values()
            .filter(|a| a.function_name == resolved)
            .collect())
    }

    // ========== Event Source Mapping operations ==========

    pub fn create_event_source_mapping(
        &mut self,
        function_name: &str,
        event_source_arn: Option<&str>,
        batch_size: Option<i32>,
        enabled: bool,
        starting_position: Option<&str>,
    ) -> Result<&EventSourceMapping, LambdaError> {
        let resolved = self.resolve_name(function_name);
        let func = self
            .functions
            .get(&resolved)
            .ok_or_else(|| LambdaError::FunctionNotFound(resolved.clone()))?;

        let uuid = uuid::Uuid::new_v4().to_string();
        let esm = EventSourceMapping {
            uuid: uuid.clone(),
            event_source_arn: event_source_arn.map(|s| s.to_string()),
            function_arn: func.function_arn.clone(),
            batch_size,
            enabled,
            state: if enabled {
                "Enabled".to_string()
            } else {
                "Disabled".to_string()
            },
            last_modified: Utc::now(),
            starting_position: starting_position.map(|s| s.to_string()),
            destination_config: None,
            filter_criteria: None,
            self_managed_event_source: None,
            source_access_configuration: None,
            self_managed_kafka_event_source_config: None,
            amazon_managed_kafka_event_source_config: None,
            document_db_event_source_config: None,
            metrics_config: None,
            provisioned_poller_config: None,
        };

        self.event_source_mappings.insert(uuid.clone(), esm);
        Ok(self.event_source_mappings.get(&uuid).unwrap())
    }

    pub fn get_event_source_mapping(&self, uuid: &str) -> Result<&EventSourceMapping, LambdaError> {
        self.event_source_mappings
            .get(uuid)
            .ok_or_else(|| LambdaError::EventSourceMappingNotFound(uuid.to_string()))
    }

    pub fn update_event_source_mapping(
        &mut self,
        uuid: &str,
        function_name: Option<&str>,
        batch_size: Option<i32>,
        enabled: Option<bool>,
    ) -> Result<&EventSourceMapping, LambdaError> {
        if !self.event_source_mappings.contains_key(uuid) {
            return Err(LambdaError::EventSourceMappingNotFound(uuid.to_string()));
        }

        // Resolve function ARN before borrowing esm mutably
        let new_function_arn = if let Some(fn_name) = function_name {
            let resolved = self.resolve_name(fn_name);
            self.functions
                .get(&resolved)
                .map(|f| f.function_arn.clone())
        } else {
            None
        };

        let esm = self.event_source_mappings.get_mut(uuid).unwrap();
        if let Some(arn) = new_function_arn {
            esm.function_arn = arn;
        }
        if let Some(bs) = batch_size {
            esm.batch_size = Some(bs);
        }
        if let Some(en) = enabled {
            esm.enabled = en;
            esm.state = if en {
                "Enabled".to_string()
            } else {
                "Disabled".to_string()
            };
        }
        esm.last_modified = Utc::now();

        Ok(self.event_source_mappings.get(uuid).unwrap())
    }

    pub fn delete_event_source_mapping(
        &mut self,
        uuid: &str,
    ) -> Result<EventSourceMapping, LambdaError> {
        self.event_source_mappings
            .remove(uuid)
            .ok_or_else(|| LambdaError::EventSourceMappingNotFound(uuid.to_string()))
    }

    pub fn list_event_source_mappings(&self) -> Vec<&EventSourceMapping> {
        self.event_source_mappings.values().collect()
    }

    // ========== Layer operations ==========

    pub fn publish_layer_version(
        &mut self,
        layer_name: &str,
        description: &str,
        compatible_runtimes: Vec<String>,
        compatible_architectures: Vec<String>,
        license_info: Option<&str>,
        account_id: &str,
        region: &str,
    ) -> Result<&LayerVersion, LambdaError> {
        let versions = self.layers.entry(layer_name.to_string()).or_default();
        // Version number must be monotonically increasing even after deletes
        let next_version = self
            .layer_next_version
            .entry(layer_name.to_string())
            .or_insert(1);
        let version_num = *next_version;
        *next_version = version_num + 1;
        let layer_arn = format!("arn:aws:lambda:{region}:{account_id}:layer:{layer_name}");
        let layer_version_arn = format!("{layer_arn}:{version_num}");

        let lv = LayerVersion {
            layer_name: layer_name.to_string(),
            version: version_num,
            layer_arn,
            layer_version_arn,
            description: description.to_string(),
            compatible_runtimes,
            compatible_architectures,
            license_info: license_info.map(|s| s.to_string()),
            created_date: Utc::now().to_rfc3339(),
            code_sha256: mock_code_sha256(layer_name),
            code_size: 262144,
            permissions: Vec::new(),
            policy_revision_id: String::new(),
        };

        versions.push(lv);
        Ok(versions.last().unwrap())
    }

    pub fn get_layer_version(
        &self,
        layer_name: &str,
        version: i64,
    ) -> Result<&LayerVersion, LambdaError> {
        let versions = self
            .layers
            .get(layer_name)
            .ok_or_else(|| LambdaError::LayerNotFound(layer_name.to_string()))?;
        versions
            .iter()
            .find(|v| v.version == version)
            .ok_or_else(|| LambdaError::LayerVersionNotFound(format!("{layer_name}:{version}")))
    }

    pub fn delete_layer_version(
        &mut self,
        layer_name: &str,
        version: i64,
    ) -> Result<(), LambdaError> {
        let versions = self
            .layers
            .get_mut(layer_name)
            .ok_or_else(|| LambdaError::LayerNotFound(layer_name.to_string()))?;
        let len_before = versions.len();
        versions.retain(|v| v.version != version);
        if versions.len() == len_before {
            // AWS actually returns 204 even if not found for DeleteLayerVersion
        }
        Ok(())
    }

    pub fn list_layer_versions(&self, layer_name: &str) -> Vec<&LayerVersion> {
        self.layers
            .get(layer_name)
            .map(|v| {
                let mut versions: Vec<&LayerVersion> = v.iter().collect();
                versions.sort_by_key(|v| std::cmp::Reverse(v.version)); // descending
                versions
            })
            .unwrap_or_default()
    }

    pub fn list_layers(&self) -> Vec<(&str, &LayerVersion)> {
        let mut result = Vec::new();
        for (name, versions) in &self.layers {
            if !versions.is_empty() {
                // latest = highest version number
                if let Some(latest) = versions.iter().max_by_key(|v| v.version) {
                    result.push((name.as_str(), latest));
                }
            }
        }
        result
    }

    pub fn add_layer_version_permission(
        &mut self,
        layer_name: &str,
        version: i64,
        statement_id: &str,
        action: &str,
        principal: &str,
        organization_id: Option<&str>,
        expected_revision_id: Option<&str>,
    ) -> Result<(String, String), LambdaError> {
        let versions = self
            .layers
            .get_mut(layer_name)
            .ok_or_else(|| LambdaError::LayerNotFound(layer_name.to_string()))?;
        let lv = versions
            .iter_mut()
            .find(|v| v.version == version)
            .ok_or_else(|| LambdaError::LayerVersionNotFound(format!("{layer_name}:{version}")))?;

        if let Some(expected) = expected_revision_id.filter(|s| !s.is_empty())
            && expected != lv.policy_revision_id
        {
            return Err(LambdaError::PreconditionFailed);
        }

        if lv
            .permissions
            .iter()
            .any(|p| p.statement_id == statement_id)
        {
            return Err(LambdaError::PermissionAlreadyExists(
                statement_id.to_string(),
            ));
        }

        let perm = LayerPermission {
            statement_id: statement_id.to_string(),
            action: action.to_string(),
            principal: principal.to_string(),
            organization_id: organization_id.map(|s| s.to_string()),
        };
        lv.permissions.push(perm);
        lv.policy_revision_id = uuid::Uuid::new_v4().to_string();

        // Build policy statement JSON
        // FIX(terraform-e2e): the layer-version permission principal must match
        // real AWS IAM policy syntax — wildcards and AWS account IDs are bare
        // strings (or {"AWS": ...}), only service principals are wrapped as
        // {"Service": ...}. Unconditionally wrapping every principal in
        // {"Service": ...} produced policies like {"Service": "123456789012"},
        // and the terraform-provider-aws lambda layer-permission Read path then
        // called arn.Parse on the account ID and failed with "arn: invalid
        // prefix". Mirrors the same logic used for aws_lambda_permission below.
        let principal_value = layer_principal_value(principal);
        let statement = serde_json::json!({
            "Sid": statement_id,
            "Effect": "Allow",
            "Principal": principal_value,
            "Action": action,
            "Resource": lv.layer_version_arn,
        });
        Ok((statement.to_string(), lv.policy_revision_id.clone()))
    }

    pub fn remove_layer_version_permission(
        &mut self,
        layer_name: &str,
        version: i64,
        statement_id: &str,
        expected_revision_id: Option<&str>,
    ) -> Result<String, LambdaError> {
        let versions = self
            .layers
            .get_mut(layer_name)
            .ok_or_else(|| LambdaError::LayerNotFound(layer_name.to_string()))?;
        let lv = versions
            .iter_mut()
            .find(|v| v.version == version)
            .ok_or_else(|| LambdaError::LayerVersionNotFound(format!("{layer_name}:{version}")))?;

        if let Some(expected) = expected_revision_id.filter(|s| !s.is_empty())
            && expected != lv.policy_revision_id
        {
            return Err(LambdaError::PreconditionFailed);
        }

        let len_before = lv.permissions.len();
        lv.permissions.retain(|p| p.statement_id != statement_id);
        if lv.permissions.len() == len_before {
            return Err(LambdaError::PermissionNotFound(statement_id.to_string()));
        }
        lv.policy_revision_id = uuid::Uuid::new_v4().to_string();
        Ok(lv.policy_revision_id.clone())
    }

    pub fn get_layer_version_policy(
        &self,
        layer_name: &str,
        version: i64,
    ) -> Result<(String, String), LambdaError> {
        let lv = self.get_layer_version(layer_name, version)?;
        if lv.permissions.is_empty() {
            return Err(LambdaError::ResourceDoesNotExist);
        }
        let statements: Vec<serde_json::Value> = lv
            .permissions
            .iter()
            .map(|p| {
                // FIX(terraform-e2e): same principal-shaping fix as in
                // add_layer_version_permission — see comment there.
                serde_json::json!({
                    "Sid": p.statement_id,
                    "Effect": "Allow",
                    "Principal": layer_principal_value(&p.principal),
                    "Action": p.action,
                    "Resource": lv.layer_version_arn,
                })
            })
            .collect();
        let policy = serde_json::json!({
            "Version": "2012-10-17",
            "Id": "default",
            "Statement": statements,
        });
        Ok((policy.to_string(), lv.policy_revision_id.clone()))
    }

    // ========== Function URL Config operations ==========

    pub fn create_function_url_config(
        &mut self,
        function_name: &str,
        auth_type: &str,
        cors: Option<CorsConfig>,
        invoke_mode: Option<&str>,
    ) -> Result<&FunctionUrlConfig, LambdaError> {
        let resolved = self.resolve_name(function_name);
        let func = self
            .functions
            .get(&resolved)
            .ok_or_else(|| LambdaError::FunctionNotFound(resolved.clone()))?;

        if self.function_url_configs.contains_key(&resolved) {
            return Err(LambdaError::FunctionUrlConfigAlreadyExists(
                resolved.clone(),
            ));
        }

        let now = Utc::now().to_rfc3339();
        let function_url = format!(
            "https://{}.lambda-url.{}.on.aws/",
            &uuid::Uuid::new_v4().to_string().replace('-', "")[..12],
            self.region
        );

        let config = FunctionUrlConfig {
            function_name: resolved.clone(),
            function_arn: func.function_arn.clone(),
            function_url,
            auth_type: auth_type.to_string(),
            cors,
            invoke_mode: invoke_mode.map(|s| s.to_string()),
            creation_time: now.clone(),
            last_modified_time: now,
        };

        self.function_url_configs.insert(resolved.clone(), config);
        Ok(self.function_url_configs.get(&resolved).unwrap())
    }

    pub fn get_function_url_config(
        &self,
        function_name: &str,
    ) -> Result<&FunctionUrlConfig, LambdaError> {
        let resolved = self.resolve_name(function_name);
        self.function_url_configs
            .get(&resolved)
            .ok_or_else(|| LambdaError::FunctionUrlConfigNotFound(resolved.clone()))
    }

    pub fn update_function_url_config(
        &mut self,
        function_name: &str,
        auth_type: Option<&str>,
        cors: Option<Option<CorsConfig>>,
        invoke_mode: Option<&str>,
    ) -> Result<&FunctionUrlConfig, LambdaError> {
        let resolved = self.resolve_name(function_name);
        let config = self
            .function_url_configs
            .get_mut(&resolved)
            .ok_or_else(|| LambdaError::FunctionUrlConfigNotFound(resolved.clone()))?;

        if let Some(at) = auth_type {
            config.auth_type = at.to_string();
        }
        if let Some(c) = cors {
            config.cors = c;
        }
        if let Some(im) = invoke_mode {
            config.invoke_mode = Some(im.to_string());
        }
        config.last_modified_time = Utc::now().to_rfc3339();

        Ok(self.function_url_configs.get(&resolved).unwrap())
    }

    pub fn delete_function_url_config(&mut self, function_name: &str) -> Result<(), LambdaError> {
        let resolved = self.resolve_name(function_name);
        if self.function_url_configs.remove(&resolved).is_none() {
            return Err(LambdaError::FunctionUrlConfigNotFound(resolved.clone()));
        }
        Ok(())
    }

    // ========== Permission operations ==========

    pub fn add_permission(
        &mut self,
        function_name: &str,
        statement_id: &str,
        action: &str,
        principal: &str,
        source_arn: Option<&str>,
        source_account: Option<&str>,
        expected_revision_id: Option<&str>,
    ) -> Result<(String, String), LambdaError> {
        let resolved = self.resolve_name(function_name);
        let func = self
            .functions
            .get(&resolved)
            .ok_or_else(|| LambdaError::FunctionNotFound(resolved.clone()))?;

        // Optimistic concurrency: when supplied, RevisionId must match the
        // policy's current revision.  An empty string is treated as absent
        // (the SDK omits empty strings on the wire, but be defensive).
        if let Some(expected) = expected_revision_id.filter(|s| !s.is_empty()) {
            let current = self.function_policy_revisions.get(&resolved);
            if current.map(String::as_str) != Some(expected) {
                return Err(LambdaError::PreconditionFailed);
            }
        }

        let perms = self
            .function_permissions
            .entry(resolved.clone())
            .or_default();
        if perms.iter().any(|p| p.statement_id == statement_id) {
            return Err(LambdaError::PermissionAlreadyExists(
                statement_id.to_string(),
            ));
        }

        let perm = Permission {
            statement_id: statement_id.to_string(),
            action: action.to_string(),
            principal: principal.to_string(),
            source_arn: source_arn.map(|s| s.to_string()),
            source_account: source_account.map(|s| s.to_string()),
        };
        perms.push(perm);

        let new_revision = uuid::Uuid::new_v4().to_string();
        self.function_policy_revisions
            .insert(resolved.clone(), new_revision.clone());

        // Build principal: if it looks like an account ID (all digits), use it directly
        // If it contains a dot (like "lambda.amazonaws.com"), it's a service principal
        let principal_value = if principal.contains('.') {
            serde_json::json!({"Service": principal})
        } else {
            serde_json::json!(principal)
        };

        let mut statement = serde_json::json!({
            "Sid": statement_id,
            "Effect": "Allow",
            "Principal": principal_value,
            "Action": action,
            "Resource": func.function_arn,
        });

        // Add Condition for SourceArn
        if let Some(src_arn) = source_arn {
            statement.as_object_mut().unwrap().insert(
                "Condition".to_string(),
                serde_json::json!({"ArnLike": {"AWS:SourceArn": src_arn}}),
            );
        }

        Ok((statement.to_string(), new_revision))
    }

    pub fn remove_permission(
        &mut self,
        function_name: &str,
        statement_id: &str,
        expected_revision_id: Option<&str>,
    ) -> Result<String, LambdaError> {
        let resolved = self.resolve_name(function_name);
        self.get_function(&resolved)?;

        if let Some(expected) = expected_revision_id.filter(|s| !s.is_empty()) {
            let current = self.function_policy_revisions.get(&resolved);
            if current.map(String::as_str) != Some(expected) {
                return Err(LambdaError::PreconditionFailed);
            }
        }

        let perms = self
            .function_permissions
            .get_mut(&resolved)
            .ok_or_else(|| LambdaError::PermissionNotFound(statement_id.to_string()))?;

        let len_before = perms.len();
        perms.retain(|p| p.statement_id != statement_id);
        if perms.len() == len_before {
            return Err(LambdaError::PermissionNotFound(statement_id.to_string()));
        }

        let new_revision = uuid::Uuid::new_v4().to_string();
        self.function_policy_revisions
            .insert(resolved, new_revision.clone());
        Ok(new_revision)
    }

    pub fn get_policy(&self, function_name: &str) -> Result<(String, String), LambdaError> {
        let resolved = self.resolve_name(function_name);
        let func = self.get_function(&resolved)?;
        let perms = self.function_permissions.get(&resolved);
        let perms = match perms {
            Some(p) if !p.is_empty() => p,
            _ => {
                return Err(LambdaError::ResourceDoesNotExist);
            }
        };

        let statements: Vec<serde_json::Value> = perms
            .iter()
            .map(|p| {
                let principal_value = if p.principal.contains('.') {
                    serde_json::json!({"Service": p.principal})
                } else {
                    serde_json::json!(p.principal)
                };
                let mut stmt = serde_json::json!({
                    "Sid": p.statement_id,
                    "Effect": "Allow",
                    "Principal": principal_value,
                    "Action": p.action,
                    "Resource": func.function_arn,
                });
                if let Some(ref src_arn) = p.source_arn {
                    stmt.as_object_mut().unwrap().insert(
                        "Condition".to_string(),
                        serde_json::json!({"ArnLike": {"AWS:SourceArn": src_arn}}),
                    );
                }
                stmt
            })
            .collect();
        let policy = serde_json::json!({
            "Version": "2012-10-17",
            "Id": "default",
            "Statement": statements,
        });
        // A policy always has a revision id once it has any statements.  The
        // unwrap is safe because add_permission populates the entry before
        // returning Ok, and remove_permission preserves the entry while at
        // least one statement remains.
        let revision_id = self
            .function_policy_revisions
            .get(&resolved)
            .cloned()
            .unwrap_or_default();
        Ok((policy.to_string(), revision_id))
    }

    // ========== Concurrency operations ==========

    pub fn put_function_concurrency(
        &mut self,
        function_name: &str,
        reserved: i32,
    ) -> Result<i32, LambdaError> {
        let resolved = self.resolve_name(function_name);
        let func = self
            .functions
            .get_mut(&resolved)
            .ok_or_else(|| LambdaError::FunctionNotFound(resolved.clone()))?;
        func.reserved_concurrent_executions = Some(reserved);
        Ok(reserved)
    }

    pub fn get_function_concurrency(
        &self,
        function_name: &str,
    ) -> Result<Option<i32>, LambdaError> {
        let resolved = self.resolve_name(function_name);
        let func = self.get_function(&resolved)?;
        Ok(func.reserved_concurrent_executions)
    }

    pub fn delete_function_concurrency(&mut self, function_name: &str) -> Result<(), LambdaError> {
        let resolved = self.resolve_name(function_name);
        let func = self
            .functions
            .get_mut(&resolved)
            .ok_or_else(|| LambdaError::FunctionNotFound(resolved.clone()))?;
        func.reserved_concurrent_executions = None;
        Ok(())
    }

    // ========== Function Event Invoke Config operations ==========

    pub fn put_function_event_invoke_config(
        &mut self,
        function_name: &str,
        max_retry_attempts: Option<i32>,
        max_event_age: Option<i32>,
        on_success: Option<&str>,
        on_failure: Option<&str>,
    ) -> Result<&FunctionEventInvokeConfig, LambdaError> {
        let resolved = self.resolve_name(function_name);
        let func = self
            .functions
            .get(&resolved)
            .ok_or_else(|| LambdaError::FunctionNotFound(resolved.clone()))?;

        let config = FunctionEventInvokeConfig {
            function_name: resolved.clone(),
            function_arn: func.function_arn.clone(),
            maximum_retry_attempts: max_retry_attempts,
            maximum_event_age_in_seconds: max_event_age,
            on_success_destination: on_success.map(|s| s.to_string()),
            on_failure_destination: on_failure.map(|s| s.to_string()),
            last_modified: Utc::now(),
        };

        self.function_event_invoke_configs
            .insert(resolved.clone(), config);
        Ok(self.function_event_invoke_configs.get(&resolved).unwrap())
    }

    pub fn get_function_event_invoke_config(
        &self,
        function_name: &str,
    ) -> Result<&FunctionEventInvokeConfig, LambdaError> {
        let resolved = self.resolve_name(function_name);
        self.function_event_invoke_configs
            .get(&resolved)
            .ok_or_else(|| LambdaError::EventInvokeConfigNotFound(resolved.clone()))
    }

    pub fn update_function_event_invoke_config(
        &mut self,
        function_name: &str,
        max_retry_attempts: Option<i32>,
        max_event_age: Option<i32>,
        on_success: Option<Option<&str>>,
        on_failure: Option<Option<&str>>,
    ) -> Result<&FunctionEventInvokeConfig, LambdaError> {
        let resolved = self.resolve_name(function_name);
        let config = self
            .function_event_invoke_configs
            .get_mut(&resolved)
            .ok_or_else(|| LambdaError::EventInvokeConfigNotFound(resolved.clone()))?;

        if let Some(r) = max_retry_attempts {
            config.maximum_retry_attempts = Some(r);
        }
        if let Some(a) = max_event_age {
            config.maximum_event_age_in_seconds = Some(a);
        }
        if let Some(s) = on_success {
            config.on_success_destination = s.map(|v| v.to_string());
        }
        if let Some(f) = on_failure {
            config.on_failure_destination = f.map(|v| v.to_string());
        }
        config.last_modified = Utc::now();

        Ok(self.function_event_invoke_configs.get(&resolved).unwrap())
    }

    pub fn delete_function_event_invoke_config(
        &mut self,
        function_name: &str,
    ) -> Result<(), LambdaError> {
        let resolved = self.resolve_name(function_name);
        if self
            .function_event_invoke_configs
            .remove(&resolved)
            .is_none()
        {
            return Err(LambdaError::EventInvokeConfigNotFound(resolved.clone()));
        }
        Ok(())
    }

    pub fn list_function_event_invoke_configs(
        &self,
        function_name: &str,
    ) -> Result<Vec<&FunctionEventInvokeConfig>, LambdaError> {
        let resolved = self.resolve_name(function_name);
        self.get_function(&resolved)?;
        Ok(self
            .function_event_invoke_configs
            .values()
            .filter(|c| c.function_name == resolved)
            .collect())
    }

    // ========== Tag operations ==========

    pub fn tag_resource(
        &mut self,
        arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), LambdaError> {
        // Extract function name from ARN
        let func_name = self.extract_function_name_from_arn(arn)?;
        let func = self
            .functions
            .get_mut(&func_name)
            .ok_or_else(|| LambdaError::ResourceNotFound(arn.to_string()))?;
        func.tags.extend(tags);
        Ok(())
    }

    pub fn untag_resource(&mut self, arn: &str, tag_keys: &[String]) -> Result<(), LambdaError> {
        let func_name = self.extract_function_name_from_arn(arn)?;
        let func = self
            .functions
            .get_mut(&func_name)
            .ok_or_else(|| LambdaError::ResourceNotFound(arn.to_string()))?;
        for key in tag_keys {
            func.tags.remove(key);
        }
        Ok(())
    }

    pub fn list_tags(&self, arn: &str) -> Result<&HashMap<String, String>, LambdaError> {
        // FIX(terraform-e2e): provider calls ListTags with code-signing-config ARNs
        //   (e.g. "arn:aws:lambda:…:code-signing-config:csc-xxx") but the handler was
        //   only looking up function ARNs, causing 404 for code signing configs.
        if arn.contains(":code-signing-config:") {
            // Code signing configs do not carry tags in the mock; return empty tags if
            // the config exists, or ResourceNotFound if it does not.
            let exists = self
                .code_signing_configs
                .values()
                .any(|c| c.code_signing_config_arn == arn);
            if exists {
                return Ok(&self.empty_tags);
            }
            return Err(LambdaError::ResourceNotFound(arn.to_string()));
        }
        let func_name = self.extract_function_name_from_arn(arn)?;
        let func = self
            .functions
            .get(&func_name)
            .ok_or_else(|| LambdaError::ResourceNotFound(arn.to_string()))?;
        Ok(&func.tags)
    }

    // ========== Version operations ==========

    pub fn publish_version(
        &mut self,
        function_name: &str,
        description: Option<&str>,
        expected_revision_id: Option<&str>,
    ) -> Result<(&LambdaFunction, String), LambdaError> {
        let resolved = self.resolve_name(function_name);
        let region = if self.region.is_empty() {
            "us-east-1"
        } else {
            &self.region
        };
        let account_id = if self.account_id.is_empty() {
            "123456789012"
        } else {
            &self.account_id
        };
        let func = self.functions.get_mut(&resolved).ok_or_else(|| {
            LambdaError::FunctionNotFoundByArn {
                region: region.to_string(),
                account_id: account_id.to_string(),
                name: resolved.clone(),
            }
        })?;

        // PublishVersion's RevisionId guards against publishing a stale
        // configuration: if the function has been updated since the caller
        // last observed the revision id, AWS rejects rather than capturing
        // an unintended snapshot.  Unlike Update*, a successful
        // PublishVersion does *not* bump $LATEST's revision id -- it
        // captures the current config into a new version, leaving $LATEST
        // semantically unchanged.
        if let Some(expected) = expected_revision_id.filter(|s| !s.is_empty())
            && expected != func.revision_id
        {
            return Err(LambdaError::PreconditionFailed);
        }

        let version_num = func.versions.len() as u64 + 1;
        let version_str = version_num.to_string();

        let fv = FunctionVersion {
            version: version_str.clone(),
            description: description.unwrap_or(&func.description).to_string(),
            code_sha256: func.code_sha256.clone(),
            code_size: func.code_size,
            created_at: Utc::now(),
        };
        func.versions.push(fv);

        Ok((self.functions.get(&resolved).unwrap(), version_str))
    }

    pub fn list_versions_by_function(
        &self,
        function_name: &str,
    ) -> Result<Vec<(&LambdaFunction, Option<&FunctionVersion>)>, LambdaError> {
        let resolved = self.resolve_name(function_name);
        match self.functions.get(&resolved) {
            Some(func) => {
                let mut result: Vec<(&LambdaFunction, Option<&FunctionVersion>)> =
                    vec![(func, None)]; // $LATEST
                for v in &func.versions {
                    result.push((func, Some(v)));
                }
                Ok(result)
            }
            None => Ok(vec![]), // moto returns empty list for nonexistent functions
        }
    }

    // ========== Function Code Signing Config ==========

    pub fn get_function_code_signing_config(
        &self,
        function_name: &str,
    ) -> Result<(&LambdaFunction, Option<&str>), LambdaError> {
        let resolved = self.resolve_name(function_name);
        let func = self.get_function(&resolved)?;
        Ok((func, func.code_signing_config_arn.as_deref()))
    }

    // ========== Code Signing Config CRUD ==========

    pub fn create_code_signing_config(
        &mut self,
        description: Option<&str>,
        allowed_publishers: Vec<String>,
        untrusted_artifact_on_deployment: &str,
    ) -> &CodeSigningConfigEntry {
        self.csc_id_counter += 1;
        let id = format!("csc-{:016x}", self.csc_id_counter);
        let region = if self.region.is_empty() {
            "us-east-1"
        } else {
            &self.region
        };
        let account_id = if self.account_id.is_empty() {
            "123456789012"
        } else {
            &self.account_id
        };
        let arn = format!("arn:aws:lambda:{region}:{account_id}:code-signing-config:{id}");
        let entry = CodeSigningConfigEntry {
            code_signing_config_id: id.clone(),
            code_signing_config_arn: arn,
            description: description.map(|s| s.to_string()),
            allowed_publishers,
            untrusted_artifact_on_deployment: untrusted_artifact_on_deployment.to_string(),
            last_modified: Utc::now().to_rfc3339(),
            functions: Vec::new(),
        };
        self.code_signing_configs.insert(id.clone(), entry);
        self.code_signing_configs.get(&id).unwrap()
    }

    pub fn get_code_signing_config(
        &self,
        csc_arn_or_id: &str,
    ) -> Result<&CodeSigningConfigEntry, LambdaError> {
        // Try by ARN (last component) or by id directly
        let id = if csc_arn_or_id.contains(':') {
            csc_arn_or_id
                .split(':')
                .next_back()
                .unwrap_or(csc_arn_or_id)
        } else {
            csc_arn_or_id
        };
        // Also search by full ARN
        self.code_signing_configs
            .values()
            .find(|e| e.code_signing_config_id == id || e.code_signing_config_arn == csc_arn_or_id)
            .ok_or_else(|| LambdaError::CodeSigningConfigNotFound(csc_arn_or_id.to_string()))
    }

    pub fn update_code_signing_config(
        &mut self,
        csc_arn_or_id: &str,
        description: Option<&str>,
        allowed_publishers: Option<Vec<String>>,
        untrusted_artifact_on_deployment: Option<&str>,
    ) -> Result<&CodeSigningConfigEntry, LambdaError> {
        let id = {
            let entry = self.get_code_signing_config(csc_arn_or_id)?;
            entry.code_signing_config_id.clone()
        };
        let entry = self.code_signing_configs.get_mut(&id).unwrap();
        if let Some(d) = description {
            entry.description = Some(d.to_string());
        }
        if let Some(ap) = allowed_publishers {
            entry.allowed_publishers = ap;
        }
        if let Some(u) = untrusted_artifact_on_deployment {
            entry.untrusted_artifact_on_deployment = u.to_string();
        }
        entry.last_modified = Utc::now().to_rfc3339();
        Ok(self.code_signing_configs.get(&id).unwrap())
    }

    pub fn delete_code_signing_config(&mut self, csc_arn_or_id: &str) -> Result<(), LambdaError> {
        let id = {
            let entry = self.get_code_signing_config(csc_arn_or_id)?;
            entry.code_signing_config_id.clone()
        };
        self.code_signing_configs.remove(&id);
        Ok(())
    }

    pub fn list_code_signing_configs(&self) -> Vec<&CodeSigningConfigEntry> {
        self.code_signing_configs.values().collect()
    }

    pub fn put_function_code_signing_config(
        &mut self,
        function_name: &str,
        csc_arn: &str,
    ) -> Result<(), LambdaError> {
        let resolved = self.resolve_name(function_name);
        // Verify function exists
        self.get_function(&resolved)?;
        // Verify CSC exists
        let id = {
            let entry = self.get_code_signing_config(csc_arn)?;
            entry.code_signing_config_id.clone()
        };
        // Link function to CSC
        if let Some(entry) = self.code_signing_configs.get_mut(&id) {
            if !entry.functions.contains(&resolved) {
                entry.functions.push(resolved.clone());
            }
        }
        // Update function's CSC ARN
        if let Some(func) = self.functions.get_mut(&resolved) {
            func.code_signing_config_arn = Some(csc_arn.to_string());
        }
        Ok(())
    }

    pub fn delete_function_code_signing_config(
        &mut self,
        function_name: &str,
    ) -> Result<(), LambdaError> {
        let resolved = self.resolve_name(function_name);
        let func = self
            .functions
            .get_mut(&resolved)
            .ok_or_else(|| LambdaError::FunctionNotFound(resolved.clone()))?;
        let old_arn = func.code_signing_config_arn.take();
        if let Some(arn) = old_arn {
            // Remove function from CSC's list
            let id = arn.split(':').next_back().unwrap_or("").to_string();
            if let Some(entry) = self.code_signing_configs.get_mut(&id) {
                entry.functions.retain(|f| f != &resolved);
            }
        }
        Ok(())
    }

    pub fn list_functions_by_code_signing_config(
        &self,
        csc_arn: &str,
    ) -> Result<Vec<String>, LambdaError> {
        let entry = self.get_code_signing_config(csc_arn)?;
        Ok(entry.functions.clone())
    }

    // ========== Provisioned Concurrency ==========

    pub fn put_provisioned_concurrency_config(
        &mut self,
        function_name: &str,
        qualifier: &str,
        provisioned: i32,
    ) -> Result<&ProvisionedConcurrencyConfig, LambdaError> {
        let resolved = self.resolve_name(function_name);
        self.get_function(&resolved)?;
        let func_arn = self.functions.get(&resolved).unwrap().function_arn.clone();
        let key = format!("{resolved}:{qualifier}");
        let qualified_arn = format!("{func_arn}:{qualifier}");
        let config = ProvisionedConcurrencyConfig {
            function_arn: qualified_arn,
            requested: provisioned,
            last_modified: Utc::now().to_rfc3339(),
        };
        self.provisioned_concurrency.insert(key.clone(), config);
        Ok(self.provisioned_concurrency.get(&key).unwrap())
    }

    pub fn get_provisioned_concurrency_config(
        &self,
        function_name: &str,
        qualifier: &str,
    ) -> Result<&ProvisionedConcurrencyConfig, LambdaError> {
        let resolved = self.resolve_name(function_name);
        let key = format!("{resolved}:{qualifier}");
        self.provisioned_concurrency
            .get(&key)
            .ok_or_else(|| LambdaError::ProvisionedConcurrencyConfigNotFound(key.clone()))
    }

    pub fn delete_provisioned_concurrency_config(
        &mut self,
        function_name: &str,
        qualifier: &str,
    ) -> Result<(), LambdaError> {
        let resolved = self.resolve_name(function_name);
        let key = format!("{resolved}:{qualifier}");
        self.provisioned_concurrency.remove(&key);
        Ok(())
    }

    pub fn list_provisioned_concurrency_configs(
        &self,
        function_name: &str,
    ) -> Vec<&ProvisionedConcurrencyConfig> {
        let resolved = self.resolve_name(function_name);
        let prefix = format!("{resolved}:");
        self.provisioned_concurrency
            .iter()
            .filter(|(k, _)| k.starts_with(&prefix))
            .map(|(_, v)| v)
            .collect()
    }

    // ========== Capacity Providers ==========

    pub fn create_capacity_provider(&mut self, name: &str) -> &CapacityProviderEntry {
        self.cp_id_counter += 1;
        let region = if self.region.is_empty() {
            "us-east-1"
        } else {
            &self.region
        };
        let account_id = if self.account_id.is_empty() {
            "123456789012"
        } else {
            &self.account_id
        };
        let arn = format!("arn:aws:lambda:{region}:{account_id}:capacity-provider:{name}");
        let entry = CapacityProviderEntry {
            capacity_provider_arn: arn,
            state: "Active".to_string(),
            last_modified: Utc::now().to_rfc3339(),
        };
        self.capacity_providers.insert(name.to_string(), entry);
        self.capacity_providers.get(name).unwrap()
    }

    pub fn get_capacity_provider(&self, name: &str) -> Result<&CapacityProviderEntry, LambdaError> {
        self.capacity_providers
            .get(name)
            .ok_or_else(|| LambdaError::CapacityProviderNotFound(name.to_string()))
    }

    pub fn delete_capacity_provider(
        &mut self,
        name: &str,
    ) -> Result<CapacityProviderEntry, LambdaError> {
        self.capacity_providers
            .remove(name)
            .ok_or_else(|| LambdaError::CapacityProviderNotFound(name.to_string()))
    }

    pub fn update_capacity_provider(
        &mut self,
        name: &str,
    ) -> Result<&CapacityProviderEntry, LambdaError> {
        let entry = self
            .capacity_providers
            .get_mut(name)
            .ok_or_else(|| LambdaError::CapacityProviderNotFound(name.to_string()))?;
        entry.last_modified = Utc::now().to_rfc3339();
        Ok(self.capacity_providers.get(name).unwrap())
    }

    pub fn list_capacity_providers(&self) -> Vec<&CapacityProviderEntry> {
        self.capacity_providers.values().collect()
    }

    // ========== Function Recursion Config ==========

    pub fn get_function_recursion_config(
        &self,
        function_name: &str,
    ) -> Result<String, LambdaError> {
        let resolved = self.resolve_name(function_name);
        self.get_function(&resolved)?;
        Ok(self
            .function_recursion_configs
            .get(&resolved)
            .map(|c| c.recursive_loop.clone())
            .unwrap_or_else(|| "Allow".to_string()))
    }

    pub fn put_function_recursion_config(
        &mut self,
        function_name: &str,
        recursive_loop: &str,
    ) -> Result<String, LambdaError> {
        let resolved = self.resolve_name(function_name);
        self.get_function(&resolved)?;
        self.function_recursion_configs.insert(
            resolved.clone(),
            FunctionRecursionConfig {
                recursive_loop: recursive_loop.to_string(),
            },
        );
        Ok(recursive_loop.to_string())
    }

    // ========== Function Scaling Config ==========

    pub fn get_function_scaling_config(
        &self,
        function_name: &str,
    ) -> Result<Option<i32>, LambdaError> {
        let resolved = self.resolve_name(function_name);
        self.get_function(&resolved)?;
        Ok(self
            .function_scaling_configs
            .get(&resolved)
            .and_then(|c| c.maximum_concurrency))
    }

    pub fn put_function_scaling_config(
        &mut self,
        function_name: &str,
        maximum_concurrency: Option<i32>,
    ) -> Result<Option<i32>, LambdaError> {
        let resolved = self.resolve_name(function_name);
        self.get_function(&resolved)?;
        self.function_scaling_configs.insert(
            resolved,
            FunctionScalingConfig {
                maximum_concurrency,
            },
        );
        Ok(maximum_concurrency)
    }

    // ========== Runtime Management Config ==========

    pub fn get_runtime_management_config(
        &self,
        function_name: &str,
    ) -> Result<RuntimeManagementConfig, LambdaError> {
        let resolved = self.resolve_name(function_name);
        self.get_function(&resolved)?;
        Ok(self
            .runtime_management_configs
            .get(&resolved)
            .cloned()
            .unwrap_or_else(|| RuntimeManagementConfig {
                update_runtime_on: "Auto".to_string(),
                runtime_version_arn: None,
            }))
    }

    pub fn put_runtime_management_config(
        &mut self,
        function_name: &str,
        update_runtime_on: &str,
        runtime_version_arn: Option<&str>,
    ) -> Result<RuntimeManagementConfig, LambdaError> {
        let resolved = self.resolve_name(function_name);
        let func = self.get_function(&resolved)?;
        let func_arn = func.function_arn.clone();
        let config = RuntimeManagementConfig {
            update_runtime_on: update_runtime_on.to_string(),
            runtime_version_arn: runtime_version_arn.map(|s| s.to_string()),
        };
        self.runtime_management_configs
            .insert(resolved, config.clone());
        let _ = func_arn;
        Ok(config)
    }

    // ========== Durable Execution operations ==========

    pub fn get_durable_execution(&self, exec_arn: &str) -> Result<&DurableExecution, LambdaError> {
        self.durable_executions
            .get(exec_arn)
            .ok_or_else(|| LambdaError::DurableExecutionNotFound(exec_arn.to_string()))
    }

    pub fn get_durable_execution_mut(
        &mut self,
        exec_arn: &str,
    ) -> Result<&mut DurableExecution, LambdaError> {
        self.durable_executions
            .get_mut(exec_arn)
            .ok_or_else(|| LambdaError::DurableExecutionNotFound(exec_arn.to_string()))
    }

    pub fn list_durable_executions_by_function(
        &self,
        function_name: &str,
    ) -> Vec<&DurableExecution> {
        let resolved = self.resolve_name(function_name);
        self.durable_executions
            .values()
            .filter(|e| e.function_name == resolved)
            .collect()
    }

    pub fn stop_durable_execution(&mut self, exec_arn: &str) -> Result<f64, LambdaError> {
        let exec = self
            .durable_executions
            .get_mut(exec_arn)
            .ok_or_else(|| LambdaError::DurableExecutionNotFound(exec_arn.to_string()))?;
        let now = Utc::now().timestamp() as f64;
        exec.status = "STOPPED".to_string();
        exec.end_timestamp = Some(now);
        Ok(now)
    }

    // ========== Layer version by ARN ==========

    pub fn get_layer_version_by_arn(&self, arn: &str) -> Result<&LayerVersion, LambdaError> {
        for versions in self.layers.values() {
            for lv in versions {
                if lv.layer_version_arn == arn {
                    return Ok(lv);
                }
            }
        }
        Err(LambdaError::LayerVersionNotFound(arn.to_string()))
    }

    // ========== Helpers ==========

    fn resolve_name(&self, name_or_arn: &str) -> String {
        if name_or_arn.starts_with("arn:aws:lambda:") {
            // ARN format: arn:aws:lambda:{region}:{account}:function:{name}[:{qualifier}]
            let parts: Vec<&str> = name_or_arn.split(':').collect();
            if parts.len() >= 7 && parts[5] == "function" {
                return parts[6].to_string();
            }
            name_or_arn
                .rsplit(':')
                .next()
                .unwrap_or(name_or_arn)
                .to_string()
        } else if name_or_arn.contains(':') {
            // Could be "function_name:qualifier" format
            name_or_arn
                .split(':')
                .next()
                .unwrap_or(name_or_arn)
                .to_string()
        } else {
            name_or_arn.to_string()
        }
    }

    fn extract_function_name_from_arn(&self, arn: &str) -> Result<String, LambdaError> {
        // ARN format: arn:aws:lambda:{region}:{account}:function:{name}
        if arn.starts_with("arn:aws:lambda:") {
            let parts: Vec<&str> = arn.split(':').collect();
            if parts.len() >= 7 && parts[5] == "function" {
                return Ok(parts[6].to_string());
            }
        }
        // Might be just a function name
        Ok(arn.to_string())
    }
}

/// Shape an `AddLayerVersionPermission` principal into the JSON value the AWS
/// layer-version policy uses. Service principals (those containing a dot, e.g.
/// `lambda.amazonaws.com`) are wrapped as `{"Service": "..."}`; everything
/// else (account IDs, `*`) is emitted as a bare string so the terraform
/// provider's policy parser can round-trip it.
fn layer_principal_value(principal: &str) -> serde_json::Value {
    if principal.contains('.') {
        serde_json::json!({"Service": principal})
    } else {
        serde_json::json!(principal)
    }
}

/// Generate a deterministic mock CodeSha256 value.
/// This is not a real SHA256; it uses std's DefaultHasher to produce a
/// deterministic hex string that looks plausible for testing purposes.
pub fn mock_code_sha256(input: &str) -> String {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let h1 = hasher.finish();
    // Hash again with a different seed to get 256 bits worth of hex
    let mut hasher2 = DefaultHasher::new();
    h1.hash(&mut hasher2);
    let h2 = hasher2.finish();
    let mut hasher3 = DefaultHasher::new();
    h2.hash(&mut hasher3);
    let h3 = hasher3.finish();
    let mut hasher4 = DefaultHasher::new();
    h3.hash(&mut hasher4);
    let h4 = hasher4.finish();
    format!("{h1:016x}{h2:016x}{h3:016x}{h4:016x}")
}
