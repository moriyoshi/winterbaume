//! Orchestrator that drives Terraform state injection and extraction using
//! registered converters.

use std::collections::{HashMap, HashSet};

use winterbaume_tfstate::{ResourceMode, TerraformState};

use crate::converter::{ConversionContext, ExtractedResource, TerraformResourceConverter};
use crate::error::ConversionError;
use crate::report::InjectionReport;

/// A closure that returns all `(account_id, region)` pairs with state.
type ScopeProvider = Box<dyn Fn() -> Vec<(String, String)> + Send + Sync>;

/// Internal wrapper pairing a converter with an optional scope provider.
struct ConverterEntry {
    converter: Box<dyn TerraformResourceConverter>,
    scope_provider: Option<ScopeProvider>,
}

/// Summary of a bulk extraction operation.
#[derive(Debug, Default)]
pub struct ExtractionReport {
    /// Extracted resources grouped by Terraform resource type.
    pub resources: HashMap<String, Vec<ExtractedResource>>,
    /// Errors encountered during extraction.
    pub errors: Vec<ConversionError>,
}

/// Orchestrates the injection and extraction of Terraform state into/from
/// winterbaume services.
///
/// Register converters for each Terraform resource type you want to handle,
/// then call [`inject_all`](TerraformInjector::inject_all) to process an
/// entire Terraform state file, or [`extract_all`](TerraformInjector::extract_all)
/// to extract resources from service state.
pub struct TerraformInjector {
    converters: HashMap<String, ConverterEntry>,
}

impl TerraformInjector {
    /// Create a new injector with no converters registered.
    pub fn new() -> Self {
        Self {
            converters: HashMap::new(),
        }
    }

    /// Register a converter for a specific Terraform resource type.
    ///
    /// Extraction uses only the context's default scope.
    /// If a converter was previously registered for the same resource type,
    /// it is replaced.
    pub fn register(&mut self, converter: impl TerraformResourceConverter + 'static) {
        let key = converter.resource_type().to_string();
        self.converters.insert(
            key,
            ConverterEntry {
                converter: Box::new(converter),
                scope_provider: None,
            },
        );
    }

    /// Register a converter with a scope provider for multi-scope extraction.
    ///
    /// The `scope_provider` closure returns all `(account_id, region)` pairs
    /// that have state. During [`extract_all`](Self::extract_all), the converter
    /// is called once per returned scope.
    pub fn register_with_scopes(
        &mut self,
        converter: impl TerraformResourceConverter + 'static,
        scope_provider: impl Fn() -> Vec<(String, String)> + Send + Sync + 'static,
    ) {
        let key = converter.resource_type().to_string();
        self.converters.insert(
            key,
            ConverterEntry {
                converter: Box::new(converter),
                scope_provider: Some(Box::new(scope_provider)),
            },
        );
    }

    /// Return the list of Terraform resource types that have registered converters.
    pub fn supported_types(&self) -> Vec<&str> {
        self.converters.keys().map(|s| s.as_str()).collect()
    }

    /// Inject all managed resources from a Terraform state into winterbaume services.
    ///
    /// Resources are processed in dependency order (topological sort based on
    /// each converter's `depends_on_types()`). Data sources are skipped.
    pub async fn inject_all(
        &self,
        tfstate: &TerraformState,
        ctx: &ConversionContext,
    ) -> InjectionReport {
        let mut report = InjectionReport::default();
        let sorted_types = self.topological_sort(&mut report);

        // Build a map from resource_type -> Vec<&Resource> for efficient lookup
        let mut resources_by_type: HashMap<&str, Vec<&winterbaume_tfstate::Resource>> =
            HashMap::new();
        for resource in &tfstate.resources {
            resources_by_type
                .entry(resource.resource_type.as_str())
                .or_default()
                .push(resource);
        }

        // Process in sorted order
        for resource_type in &sorted_types {
            if let Some(resources) = resources_by_type.remove(resource_type.as_str()) {
                let entry = &self.converters[resource_type];
                for resource in resources {
                    // Skip data sources
                    if resource.mode == ResourceMode::Data {
                        continue;
                    }
                    for instance in &resource.instances {
                        match entry.converter.inject(instance, ctx).await {
                            Ok(result) => {
                                report.injected += 1;
                                report.warnings.extend(result.warnings);
                            }
                            Err(e) => {
                                report.errors.push(e);
                            }
                        }
                    }
                }
            }
        }

        // Collect skipped resource types (those without converters)
        let mut seen_skipped: HashSet<String> = HashSet::new();
        for resource in &tfstate.resources {
            if resource.mode == ResourceMode::Data {
                continue;
            }
            if !self.converters.contains_key(&resource.resource_type)
                && seen_skipped.insert(resource.resource_type.clone())
            {
                report.skipped.push(resource.resource_type.clone());
            }
        }

        report
    }

    /// Extract all resources from winterbaume service state.
    ///
    /// For each converter, queries the scope provider (if registered via
    /// [`register_with_scopes`](Self::register_with_scopes)) and calls
    /// `extract()` once per `(account_id, region)` scope. Converters
    /// registered via [`register`](Self::register) are called once with the
    /// context's default scope.
    ///
    /// Resources with the same `(resource_type, name)` from different scopes
    /// receive scope-qualified names to prevent Terraform address collisions.
    pub async fn extract_all(&self, ctx: &ConversionContext) -> ExtractionReport {
        let mut report = ExtractionReport::default();

        for (resource_type, entry) in &self.converters {
            let scopes: Vec<(String, String)> = match &entry.scope_provider {
                Some(provider) => {
                    let s = provider();
                    if s.is_empty() {
                        vec![(ctx.default_account_id.clone(), ctx.default_region.clone())]
                    } else {
                        s
                    }
                }
                None => vec![(ctx.default_account_id.clone(), ctx.default_region.clone())],
            };

            for (account_id, region) in &scopes {
                let scope_ctx = ConversionContext {
                    default_account_id: account_id.clone(),
                    default_region: region.clone(),
                };
                match entry.converter.extract(&scope_ctx).await {
                    Ok(resources) => {
                        report
                            .resources
                            .entry(resource_type.clone())
                            .or_default()
                            .extend(resources);
                    }
                    Err(e) => report.errors.push(e),
                }
            }
        }

        // Post-collection collision detection: qualify names where the same
        // (resource_type, name) appears from multiple scopes.
        for resources in report.resources.values_mut() {
            let mut name_indices: HashMap<String, Vec<usize>> = HashMap::new();
            for (i, r) in resources.iter().enumerate() {
                name_indices.entry(r.name.clone()).or_default().push(i);
            }
            for indices in name_indices.into_values() {
                if indices.len() > 1 {
                    for i in indices {
                        let suffix = format!(
                            "{}_{}",
                            resources[i].account_id,
                            resources[i].region.replace('-', "_"),
                        );
                        resources[i].name = format!("{}_{}", resources[i].name, suffix);
                    }
                }
            }
        }

        report
    }

    /// Topological sort of converter resource types based on dependency order.
    ///
    /// Falls back to arbitrary order with a warning if cycles are detected.
    fn topological_sort(&self, report: &mut InjectionReport) -> Vec<String> {
        let converter_types: HashSet<&str> = self.converters.keys().map(|s| s.as_str()).collect();

        // Build adjacency list: type -> types it depends on
        let mut deps: HashMap<&str, Vec<&str>> = HashMap::new();
        for (type_name, entry) in &self.converters {
            let converter_deps: Vec<&str> = entry
                .converter
                .depends_on_types()
                .into_iter()
                .filter(|d| converter_types.contains(d))
                .collect();
            deps.insert(type_name.as_str(), converter_deps);
        }

        // Kahn's algorithm
        // Edge: dependency -> dependent (if A depends on B, edge B->A)
        let mut forward: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut in_deg: HashMap<&str, usize> = HashMap::new();
        for &t in &converter_types {
            in_deg.insert(t, 0);
            forward.entry(t).or_default();
        }
        for (&t, dep_list) in &deps {
            *in_deg.entry(t).or_insert(0) = dep_list.len();
            for &dep in dep_list {
                forward.entry(dep).or_default().push(t);
            }
        }

        let mut queue: Vec<&str> = in_deg
            .iter()
            .filter(|(_, deg)| **deg == 0)
            .map(|(t, _)| *t)
            .collect();
        queue.sort(); // deterministic order

        let mut result = Vec::new();
        while let Some(t) = queue.pop() {
            result.push(t.to_string());
            if let Some(dependents) = forward.get(t) {
                for &dep in dependents {
                    if let Some(deg) = in_deg.get_mut(dep) {
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push(dep);
                        }
                    }
                }
            }
        }

        if result.len() < converter_types.len() {
            report.warnings.push(
                "dependency cycle detected among converters; processing remaining types in arbitrary order".to_string(),
            );
            for &t in &converter_types {
                if !result.iter().any(|r| r == t) {
                    result.push(t.to_string());
                }
            }
        }

        result
    }
}

impl Default for TerraformInjector {
    fn default() -> Self {
        Self::new()
    }
}
