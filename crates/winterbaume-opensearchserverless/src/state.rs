use std::collections::HashMap;

use uuid::Uuid;

use crate::types::*;

#[derive(Debug, Default)]
pub struct OpenSearchServerlessState {
    /// Collection id -> Collection
    pub collections: HashMap<String, Collection>,
    /// (policy_type, policy_name) -> SecurityPolicy
    pub security_policies: HashMap<(String, String), SecurityPolicy>,
    /// VPC endpoint id -> VpcEndpoint
    pub vpc_endpoints: HashMap<String, VpcEndpoint>,
    /// ARN -> tags
    pub tags: TagStore,
}

#[derive(Debug, thiserror::Error)]
pub enum OpenSearchServerlessError {
    #[error("Collection {name} already exists.")]
    CollectionAlreadyExists { name: String },

    #[error("Security policy {name} of type {type_} already exists.")]
    SecurityPolicyAlreadyExists { name: String, type_: String },

    #[error("VPC endpoint {name} already exists.")]
    VpcEndpointAlreadyExists { name: String },

    #[error("Policy version mismatch: expected {expected}, got {got}")]
    PolicyVersionMismatch { expected: String, got: String },

    #[error("Collection {id} not found.")]
    CollectionNotFound { id: String },

    #[error("Security policy {name} of type {type_} not found.")]
    SecurityPolicyNotFound { name: String, type_: String },

    #[error("Resource {arn} not found.")]
    ResourceNotFound { arn: String },
}

impl OpenSearchServerlessState {
    // ---- Collection ----

    pub fn create_collection(
        &mut self,
        name: &str,
        type_: Option<&str>,
        description: Option<&str>,
        tags: Vec<Tag>,
        region: &str,
        account_id: &str,
    ) -> Result<Collection, OpenSearchServerlessError> {
        // Check if name already exists
        if self.collections.values().any(|c| c.name == name) {
            return Err(OpenSearchServerlessError::CollectionAlreadyExists {
                name: name.to_string(),
            });
        }

        let id = generate_collection_id();
        let arn = format!("arn:aws:aoss:{region}:{account_id}:collection/{id}");
        let now = now_millis();

        let collection = Collection {
            id: id.clone(),
            name: name.to_string(),
            arn: arn.clone(),
            type_: type_.unwrap_or("SEARCH").to_string(),
            status: "ACTIVE".to_string(),
            description: description.map(|s| s.to_string()),
            kms_key_arn: None,
            tags: tags.clone(),
            created_date: now,
            last_modified_date: now,
        };

        // Store tags in tag store by ARN
        if !tags.is_empty() {
            self.tags.insert(arn, tags);
        }

        self.collections.insert(id, collection.clone());
        Ok(collection)
    }

    pub fn delete_collection(&mut self, id: &str) -> Result<Collection, OpenSearchServerlessError> {
        let collection = self
            .collections
            .remove(id)
            .ok_or_else(|| OpenSearchServerlessError::CollectionNotFound { id: id.to_string() })?;
        // Remove tags for this collection
        self.tags.remove(&collection.arn);
        Ok(collection)
    }

    pub fn list_collections(&self) -> Vec<&Collection> {
        self.collections.values().collect()
    }

    pub fn batch_get_collection_by_ids(&self, ids: &[String]) -> Vec<&Collection> {
        ids.iter()
            .filter_map(|id| self.collections.get(id.as_str()))
            .collect()
    }

    pub fn batch_get_collection_by_names(&self, names: &[String]) -> Vec<&Collection> {
        self.collections
            .values()
            .filter(|c| names.contains(&c.name))
            .collect()
    }

    // ---- SecurityPolicy ----

    pub fn create_security_policy(
        &mut self,
        type_: &str,
        name: &str,
        policy: serde_json::Value,
        description: Option<&str>,
    ) -> Result<SecurityPolicy, OpenSearchServerlessError> {
        let key = (type_.to_string(), name.to_string());
        if self.security_policies.contains_key(&key) {
            return Err(OpenSearchServerlessError::SecurityPolicyAlreadyExists {
                name: name.to_string(),
                type_: type_.to_string(),
            });
        }

        let now = now_millis();
        let policy_version = generate_policy_version();
        let sp = SecurityPolicy {
            name: name.to_string(),
            type_: type_.to_string(),
            policy,
            policy_version,
            description: description.map(|s| s.to_string()),
            created_date: now,
            last_modified_date: now,
        };
        self.security_policies.insert(key, sp.clone());
        Ok(sp)
    }

    pub fn get_security_policy(
        &self,
        type_: &str,
        name: &str,
    ) -> Result<&SecurityPolicy, OpenSearchServerlessError> {
        self.security_policies
            .get(&(type_.to_string(), name.to_string()))
            .ok_or_else(|| OpenSearchServerlessError::SecurityPolicyNotFound {
                name: name.to_string(),
                type_: type_.to_string(),
            })
    }

    pub fn update_security_policy(
        &mut self,
        type_: &str,
        name: &str,
        policy_version: &str,
        policy: Option<serde_json::Value>,
        description: Option<&str>,
    ) -> Result<SecurityPolicy, OpenSearchServerlessError> {
        let key = (type_.to_string(), name.to_string());
        let sp = self.security_policies.get_mut(&key).ok_or_else(|| {
            OpenSearchServerlessError::SecurityPolicyNotFound {
                name: name.to_string(),
                type_: type_.to_string(),
            }
        })?;

        if sp.policy_version != policy_version {
            return Err(OpenSearchServerlessError::PolicyVersionMismatch {
                expected: sp.policy_version.clone(),
                got: policy_version.to_string(),
            });
        }

        if let Some(p) = policy {
            sp.policy = p;
        }
        if let Some(d) = description {
            sp.description = Some(d.to_string());
        }
        sp.policy_version = generate_policy_version();
        sp.last_modified_date = now_millis();
        Ok(sp.clone())
    }

    pub fn list_security_policies(&self, type_: &str) -> Vec<&SecurityPolicy> {
        self.security_policies
            .iter()
            .filter(|((t, _), _)| t == type_)
            .map(|(_, v)| v)
            .collect()
    }

    // ---- VpcEndpoint ----

    pub fn create_vpc_endpoint(
        &mut self,
        name: &str,
        vpc_id: &str,
        subnet_ids: Vec<String>,
        security_group_ids: Vec<String>,
    ) -> Result<VpcEndpoint, OpenSearchServerlessError> {
        if self.vpc_endpoints.values().any(|e| e.name == name) {
            return Err(OpenSearchServerlessError::VpcEndpointAlreadyExists {
                name: name.to_string(),
            });
        }
        let id = generate_vpc_endpoint_id();
        let ep = VpcEndpoint {
            id: id.clone(),
            name: name.to_string(),
            vpc_id: vpc_id.to_string(),
            subnet_ids,
            security_group_ids,
            status: "ACTIVE".to_string(),
        };
        self.vpc_endpoints.insert(id, ep.clone());
        Ok(ep)
    }

    // ---- Tags ----

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: Vec<Tag>,
    ) -> Result<(), OpenSearchServerlessError> {
        // Verify the ARN refers to a known collection
        if !self.collections.values().any(|c| c.arn == resource_arn) {
            return Err(OpenSearchServerlessError::ResourceNotFound {
                arn: resource_arn.to_string(),
            });
        }
        let existing = self.tags.entry(resource_arn.to_string()).or_default();
        for new_tag in tags {
            if let Some(existing_tag) = existing.iter_mut().find(|t| t.key == new_tag.key) {
                existing_tag.value = new_tag.value;
            } else {
                existing.push(new_tag);
            }
        }
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), OpenSearchServerlessError> {
        if !self.collections.values().any(|c| c.arn == resource_arn) {
            return Err(OpenSearchServerlessError::ResourceNotFound {
                arn: resource_arn.to_string(),
            });
        }
        if let Some(existing) = self.tags.get_mut(resource_arn) {
            existing.retain(|t| !tag_keys.contains(&t.key));
        }
        Ok(())
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<Vec<&Tag>, OpenSearchServerlessError> {
        if !self.collections.values().any(|c| c.arn == resource_arn) {
            return Err(OpenSearchServerlessError::ResourceNotFound {
                arn: resource_arn.to_string(),
            });
        }
        Ok(self
            .tags
            .get(resource_arn)
            .map(|v| v.iter().collect())
            .unwrap_or_default())
    }
}

fn generate_collection_id() -> String {
    // Generate a 12-character alphanumeric ID similar to real AOSS
    let id = Uuid::new_v4().to_string().replace('-', "");
    id[..12].to_string()
}

fn generate_vpc_endpoint_id() -> String {
    format!(
        "vpce-{}",
        &Uuid::new_v4().to_string().replace('-', "")[..17]
    )
}

/// Monotonic per-process counter appended to generated policy versions.
///
/// Real AOSS returns opaque, always-unique `policyVersion` tokens; the only
/// contract callers ( and `UpdateSecurityPolicy` optimistic-concurrency
/// checks ) rely on is that a fresh version differs from the previous one. A
/// bare millisecond timestamp collides when a create and an immediately
/// following update land in the same millisecond, which makes
/// `test_update_security_policy`'s `assert_ne!` flaky on fast runners. The
/// counter guarantees every generated version is distinct within the process.
static POLICY_VERSION_SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn generate_policy_version() -> String {
    // Policy versions look like "MTY4OTk3NzM4OTkzOV8x"
    let seq = POLICY_VERSION_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    format!("v{}_{}", now_millis(), seq)
}

fn now_millis() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}
