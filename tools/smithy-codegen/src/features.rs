//! Operation-to-feature-flag mapping for partial-build support.
//!
//! When a service crate is split into many Cargo features (one per logical
//! resource group, e.g. `network`, `compute`, `storage`), the codegen emits
//! `#[cfg(feature = "...")]` attributes around operation-specific generated
//! code. The mapping from operation name to feature slug is loaded from a
//! TOML file with the structure:
//!
//! ```toml
//! [operations]
//! CreateVpc = "network"
//! RunInstances = "compute"
//! ```
//!
//! Operations not listed here fall back to a `default_feature` value (also
//! configurable in the same file) so every operation ends up gated by exactly
//! one feature slug. Types referenced by operations in two or more distinct
//! feature slugs are emitted unconditionally (shared "core" code), since they
//! must be available regardless of which subset of features is enabled.

use std::collections::{BTreeSet, HashMap};
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::model::{ResolvedType, ServiceModel};

#[derive(Debug, Deserialize)]
struct FeatureMapFile {
    /// Feature slug used for any operation absent from `[operations]`.
    /// Defaults to `"extras"`.
    #[serde(default = "default_default_feature")]
    default_feature: String,

    /// Operation-name → feature-slug map.
    #[serde(default)]
    operations: HashMap<String, String>,
}

fn default_default_feature() -> String {
    "extras".to_string()
}

/// Resolved feature mapping, with the default slug already applied.
#[derive(Debug, Clone)]
pub struct FeatureMap {
    /// Feature slug for every operation in the model.
    pub operations: HashMap<String, String>,
    /// Slug applied to operations missing from the source TOML.
    pub default_feature: String,
}

impl FeatureMap {
    /// Load the mapping from a TOML file and resolve every operation in `model`
    /// to a feature slug (falling back to `default_feature`).
    pub fn load(path: &Path, model: &ServiceModel) -> Result<Self> {
        let raw = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read features map: {}", path.display()))?;
        let parsed: FeatureMapFile = toml::from_str(&raw)
            .with_context(|| format!("Failed to parse features map: {}", path.display()))?;

        let mut operations = HashMap::with_capacity(model.operations.len());
        for op in &model.operations {
            let slug = parsed
                .operations
                .get(&op.name)
                .cloned()
                .unwrap_or_else(|| parsed.default_feature.clone());
            operations.insert(op.name.clone(), slug);
        }

        Ok(FeatureMap {
            operations,
            default_feature: parsed.default_feature,
        })
    }

    /// Feature slug for `op_name`, falling back to `default_feature`.
    pub fn feature_for(&self, op_name: &str) -> &str {
        self.operations
            .get(op_name)
            .map(String::as_str)
            .unwrap_or(self.default_feature.as_str())
    }

    /// All distinct feature slugs in use, sorted.
    pub fn all_features(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = self.operations.values().cloned().collect();
        set.insert(self.default_feature.clone());
        set.into_iter().collect()
    }
}

/// For each shape FQN, the set of feature slugs whose operations reference that
/// shape (transitively through nested members).
///
/// A shape used by exactly one feature slug is gated to that slug; a shape used
/// by two or more distinct slugs is treated as shared core code (no `#[cfg]`)
/// so it remains available regardless of which subset of features is enabled.
pub type ShapeFeatures = HashMap<String, BTreeSet<String>>;

/// Compute the transitive feature usage for every shape in the model.
pub fn compute_shape_features(model: &ServiceModel, feature_map: &FeatureMap) -> ShapeFeatures {
    let mut shape_features: ShapeFeatures = HashMap::new();

    // Seed: every operation's input/output shape is tagged with that operation's feature.
    for op in &model.operations {
        let feature = feature_map.feature_for(&op.name).to_string();
        if let Some(input) = &op.input_shape {
            shape_features
                .entry(input.clone())
                .or_default()
                .insert(feature.clone());
        }
        if let Some(output) = &op.output_shape {
            shape_features
                .entry(output.clone())
                .or_default()
                .insert(feature);
        }
    }

    // Transitive propagation: a struct's members inherit the parent's feature
    // set. We track features for the member's target FQN itself (so list/set/
    // map shapes that the codegen emits as wrapper structs get cfg-gated) AND
    // for every struct FQN reachable inside that target via List/Map nesting.
    // Iterate worklist-style until nothing changes.
    let mut worklist: Vec<String> = shape_features.keys().cloned().collect();
    while let Some(shape_fqn) = worklist.pop() {
        let parent_features = match shape_features.get(&shape_fqn) {
            Some(f) => f.clone(),
            None => continue,
        };
        for member in model.get_members(&shape_fqn) {
            let mut tagged: Vec<String> = vec![member.target.clone()];
            let resolved = model.resolve_type(&member.target);
            tagged.extend(collect_struct_fqns(&resolved));

            for child_fqn in tagged {
                let child_set = shape_features.entry(child_fqn.clone()).or_default();
                let before = child_set.len();
                for f in &parent_features {
                    child_set.insert(f.clone());
                }
                if child_set.len() != before {
                    worklist.push(child_fqn);
                }
            }
        }
    }

    shape_features
}

fn collect_struct_fqns(resolved: &ResolvedType) -> Vec<String> {
    let mut out = Vec::new();
    walk(resolved, &mut out);
    out
}

fn walk(resolved: &ResolvedType, out: &mut Vec<String>) {
    match resolved {
        ResolvedType::Structure(fqn) => out.push(fqn.clone()),
        ResolvedType::List(inner) => walk(inner, out),
        ResolvedType::Map(k, v) => {
            walk(k, out);
            walk(v, out);
        }
        _ => {}
    }
}

/// Render the `#[cfg(feature = "...")]` attribute for a single feature slug.
/// Returns an empty string when `slug` is the always-on marker (currently
/// none — every slug produces a cfg).
pub fn cfg_attr_for_feature(slug: &str) -> String {
    format!("#[cfg(feature = \"{slug}\")]\n")
}

/// Render the cfg attribute for a shape based on its feature set:
/// - 0 features → empty (should not normally happen; treat as always-on)
/// - 1 feature  → `#[cfg(feature = "X")]`
/// - 2+ features → empty (shared core, always emitted)
pub fn cfg_attr_for_shape(features: Option<&BTreeSet<String>>) -> String {
    match features {
        Some(f) if f.len() == 1 => cfg_attr_for_feature(f.iter().next().unwrap()),
        _ => String::new(),
    }
}
