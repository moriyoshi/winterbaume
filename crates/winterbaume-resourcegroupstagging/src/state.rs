use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct TaggingState {
    /// Resources indexed by ARN, each holding a map of tag key -> value.
    pub resources: HashMap<String, TaggedResource>,
}

/// Domain-specific error type for tagging operations.
#[derive(Debug, Error)]
pub enum TaggingError {
    #[error("Resource ARN must not be empty")]
    EmptyArn,
}

impl TaggingState {
    /// Tag a set of resources. Creates the resource entry if it doesn't exist.
    pub fn tag_resources(
        &mut self,
        arns: &[String],
        tags: &HashMap<String, String>,
    ) -> HashMap<String, TaggingError> {
        let mut failures = HashMap::new();
        for arn in arns {
            if arn.is_empty() {
                failures.insert(arn.clone(), TaggingError::EmptyArn);
                continue;
            }
            let resource = self
                .resources
                .entry(arn.clone())
                .or_insert_with(|| TaggedResource {
                    arn: arn.clone(),
                    tags: HashMap::new(),
                });
            for (key, value) in tags {
                resource.tags.insert(key.clone(), value.clone());
            }
        }
        failures
    }

    /// Remove specified tag keys from the given resources.
    pub fn untag_resources(
        &mut self,
        arns: &[String],
        tag_keys: &[String],
    ) -> HashMap<String, TaggingError> {
        let failures = HashMap::new();
        for arn in arns {
            if let Some(resource) = self.resources.get_mut(arn.as_str()) {
                for key in tag_keys {
                    resource.tags.remove(key);
                }
            }
            // UntagResources does not fail for missing resources per AWS behavior
        }
        // Remove entries with no tags remaining
        self.resources.retain(|_, r| !r.tags.is_empty());
        failures
    }

    /// Get all resources, optionally filtered by tag filters and resource type filters.
    pub fn get_resources(
        &self,
        tag_filters: &[(String, Vec<String>)],
        resource_type_filters: &[String],
    ) -> Vec<&TaggedResource> {
        self.resources
            .values()
            .filter(|r| {
                // Resource type filter: match against the ARN's service/resource-type
                if !resource_type_filters.is_empty() {
                    let matches_type = resource_type_filters
                        .iter()
                        .any(|filter| r.arn.contains(filter));
                    if !matches_type {
                        return false;
                    }
                }

                // Tag filters: all filters must match (AND)
                for (key, values) in tag_filters {
                    match r.tags.get(key) {
                        Some(tag_value) => {
                            if !values.is_empty() && !values.contains(tag_value) {
                                return false;
                            }
                        }
                        None => return false,
                    }
                }

                true
            })
            .collect()
    }

    /// Get all distinct tag keys across all resources.
    pub fn get_tag_keys(&self) -> Vec<String> {
        let mut keys: Vec<String> = self
            .resources
            .values()
            .flat_map(|r| r.tags.keys().cloned())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        keys.sort();
        keys
    }

    /// Get all distinct values for a specific tag key.
    pub fn get_tag_values(&self, key: &str) -> Vec<String> {
        let mut values: Vec<String> = self
            .resources
            .values()
            .filter_map(|r| r.tags.get(key).cloned())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        values.sort();
        values
    }
}
