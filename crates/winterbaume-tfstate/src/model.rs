use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::TfStateError;
use crate::provider::ProviderRef;

/// Top-level Terraform state (version 4).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TerraformState {
    /// State file format version (must be 4).
    pub version: u32,
    /// Monotonically increasing serial number.
    pub serial: u64,
    /// UUID identifying the state lineage.
    pub lineage: String,
    /// Terraform version that wrote this state.
    pub terraform_version: String,
    /// Named outputs from the root module.
    #[serde(default)]
    pub outputs: HashMap<String, OutputValue>,
    /// All managed resources and data sources.
    #[serde(default)]
    pub resources: Vec<Resource>,
}

/// A named output value from a Terraform configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutputValue {
    /// The output value.
    pub value: Value,
    /// The output type descriptor.
    #[serde(rename = "type")]
    pub output_type: Value,
    /// Whether the output is marked as sensitive.
    #[serde(default)]
    pub sensitive: bool,
}

/// The mode of a resource (managed or data).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ResourceMode {
    /// A managed resource (created and managed by Terraform).
    Managed,
    /// A data source (read-only).
    Data,
}

/// A Terraform resource in the state file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Resource {
    /// Whether this is a managed resource or data source.
    pub mode: ResourceMode,
    /// The resource type (e.g. "aws_s3_bucket").
    #[serde(rename = "type")]
    pub resource_type: String,
    /// The resource name from the configuration.
    pub name: String,
    /// The provider configuration reference.
    pub provider: String,
    /// Resource instances (one per count/for_each key).
    #[serde(default)]
    pub instances: Vec<ResourceInstance>,
    /// The module path, if this resource is inside a module.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    /// Additional fields for forward compatibility.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// An index key for a resource instance (count index or for_each key).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum IndexKey {
    /// Numeric index from `count`.
    Int(i64),
    /// String key from `for_each`.
    String(String),
}

/// A single instance of a resource.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceInstance {
    /// The schema version of the resource provider.
    pub schema_version: u64,
    /// The resource attributes.
    pub attributes: Value,
    /// Attributes marked as sensitive.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sensitive_attributes: Vec<Value>,
    /// Other resources this instance depends on.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<String>,
    /// The index key (for count/for_each resources).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_key: Option<IndexKey>,
    /// Private provider data (base64-encoded).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<String>,
    /// Whether create_before_destroy lifecycle is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_before_destroy: Option<bool>,
    /// Additional fields for forward compatibility.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl TerraformState {
    /// Parse a Terraform state from a reader.
    ///
    /// Returns an error if the JSON is invalid or the version is not 4.
    pub fn from_reader<R: std::io::Read>(reader: R) -> Result<Self, TfStateError> {
        let state: Self = serde_json::from_reader(reader)?;
        if state.version != 4 {
            return Err(TfStateError::UnsupportedVersion {
                version: state.version,
            });
        }
        Ok(state)
    }

    /// Parse a Terraform state from a string.
    ///
    /// Returns an error if the JSON is invalid or the version is not 4.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self, TfStateError> {
        let state: Self = serde_json::from_str(s)?;
        if state.version != 4 {
            return Err(TfStateError::UnsupportedVersion {
                version: state.version,
            });
        }
        Ok(state)
    }

    /// Serialize the state to a writer with pretty formatting.
    pub fn to_writer_pretty<W: std::io::Write>(&self, writer: W) -> Result<(), TfStateError> {
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }

    /// Serialize the state to a pretty-printed JSON string.
    pub fn to_string_pretty(&self) -> Result<String, TfStateError> {
        let s = serde_json::to_string_pretty(self)?;
        Ok(s)
    }

    /// Return all resources matching the given type.
    pub fn resources_by_type(&self, resource_type: &str) -> Vec<&Resource> {
        self.resources
            .iter()
            .filter(|r| r.resource_type == resource_type)
            .collect()
    }

    /// Find a resource by type and name.
    pub fn get_resource(&self, resource_type: &str, name: &str) -> Option<&Resource> {
        self.resources
            .iter()
            .find(|r| r.resource_type == resource_type && r.name == name)
    }

    /// Find a resource by type and name (mutable).
    pub fn get_resource_mut(&mut self, resource_type: &str, name: &str) -> Option<&mut Resource> {
        self.resources
            .iter_mut()
            .find(|r| r.resource_type == resource_type && r.name == name)
    }

    /// Iterate over all resources.
    pub fn iter_resources(&self) -> impl Iterator<Item = &Resource> {
        self.resources.iter()
    }

    /// Add a resource to the state.
    ///
    /// Returns an error if a resource with the same type and name already exists.
    pub fn add_resource(&mut self, resource: Resource) -> Result<(), TfStateError> {
        if self
            .resources
            .iter()
            .any(|r| r.resource_type == resource.resource_type && r.name == resource.name)
        {
            return Err(TfStateError::DuplicateResource {
                resource_type: resource.resource_type,
                name: resource.name,
            });
        }
        self.resources.push(resource);
        Ok(())
    }

    /// Remove a resource by type and name, returning it if found.
    pub fn remove_resource(&mut self, resource_type: &str, name: &str) -> Option<Resource> {
        let idx = self
            .resources
            .iter()
            .position(|r| r.resource_type == resource_type && r.name == name)?;
        Some(self.resources.remove(idx))
    }

    /// Increment the serial number.
    pub fn bump_serial(&mut self) {
        self.serial += 1;
    }
}

impl Resource {
    /// Get the primary (first) instance of this resource.
    pub fn primary_instance(&self) -> Option<&ResourceInstance> {
        self.instances.first()
    }

    /// Get the primary (first) instance of this resource (mutable).
    pub fn primary_instance_mut(&mut self) -> Option<&mut ResourceInstance> {
        self.instances.first_mut()
    }

    /// Get an instance by its index key.
    pub fn get_instance(&self, key: &IndexKey) -> Option<&ResourceInstance> {
        self.instances
            .iter()
            .find(|i| i.index_key.as_ref() == Some(key))
    }

    /// Parse the provider string into a `ProviderRef`.
    pub fn provider_ref(&self) -> Option<ProviderRef> {
        ProviderRef::parse(&self.provider)
    }

    /// Return the full resource address (e.g. "aws_s3_bucket.example"
    /// or "module.foo.aws_s3_bucket.example").
    pub fn address(&self) -> String {
        match &self.module {
            Some(module) => format!("{}.{}.{}", module, self.resource_type, self.name),
            None => format!("{}.{}", self.resource_type, self.name),
        }
    }
}

impl ResourceInstance {
    /// Get an attribute value by key from the attributes object.
    pub fn get_attribute(&self, key: &str) -> Option<&Value> {
        self.attributes.get(key)
    }

    /// Set an attribute value in the attributes object.
    ///
    /// If `attributes` is not an object, this is a no-op.
    pub fn set_attribute(&mut self, key: &str, value: Value) {
        if let Some(obj) = self.attributes.as_object_mut() {
            obj.insert(key.to_string(), value);
        }
    }

    /// Shorthand to get the "id" attribute as a string.
    pub fn id(&self) -> Option<&str> {
        self.get_attribute("id")?.as_str()
    }
}
