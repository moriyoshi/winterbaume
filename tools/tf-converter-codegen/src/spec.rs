//! Per-service TOML mapping spec for the Terraform converter codegen.
//!
//! Each spec file describes one or more Terraform resource types and how their
//! HCL attributes project into a winterbaume `*StateView` struct. The codegen
//! emits a serde-driven Rust model that round-trips between
//! `serde_json::Value` (the TF state attributes) and the StateView, plus an
//! `into_state_view` projection that fills in computed templates (ARN/URL).

use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServiceSpec {
    pub service: Service,
    /// Named under `[[resource]]` in TOML.
    #[serde(default, rename = "resource")]
    pub resources: Vec<Resource>,
}

#[derive(Debug, Deserialize)]
pub struct Service {
    /// Service slug used as the file name (e.g., "sqs"). Informational.
    pub name: String,
    /// Rust crate name to import the StateView types from
    /// (e.g., "winterbaume_sqs"). Used as the leading path segment in
    /// generated `use` statements.
    pub crate_name: String,
    /// Submodule path on top of `crate_name` to reach the StateView types.
    /// Defaults to "views". Generated code emits
    /// `use {crate_name}::{state_view_module}::{view};`.
    #[serde(default = "default_state_view_module")]
    pub state_view_module: String,
}

fn default_state_view_module() -> String {
    "views".to_string()
}

#[derive(Debug, Deserialize)]
pub struct Resource {
    /// Terraform resource type, e.g., "aws_sqs_queue". Informational; used in
    /// the generated module-doc comment.
    #[serde(rename = "type")]
    pub resource_type: String,
    /// Generated struct name, e.g., "SqsQueueTfModel".
    pub model: String,
    /// Target StateView struct, e.g., "QueueStateView".
    pub view: String,
    #[allow(dead_code)] // consumed once the audit subcommand lands
    #[serde(default)]
    pub modes: Modes,
    #[serde(default)]
    pub computed: Computed,
    #[serde(default, rename = "field")]
    pub fields: Vec<Field>,
    #[serde(default, rename = "computed_extract")]
    pub computed_extract: Vec<ComputedExtract>,
    /// Fields on the StateView that the spec does not project from TF input
    /// but that must still be populated in `into_state_view`. Used for things
    /// like `region` (filled from the conversion context), `account_id`
    /// (also context), and `created_timestamp` (set to None in inject).
    #[serde(default, rename = "view_extra")]
    pub view_extras: Vec<ViewExtra>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Modes {
    /// "round_trip" (default), "inject_only", or "extract_only".
    #[allow(dead_code)] // consumed once the audit subcommand lands
    #[serde(default = "default_mode")]
    pub mode: String,
}

fn default_mode() -> String {
    "round_trip".to_string()
}

#[derive(Debug, Deserialize, Default)]
pub struct Computed {
    /// ARN template + optional pre-existing-attribute capture.
    pub arn: Option<TemplatedField>,
    /// URL template + optional pre-existing-attribute capture.
    pub url: Option<TemplatedField>,
}

#[derive(Debug, Deserialize)]
pub struct TemplatedField {
    /// `format!`-style template, e.g.
    /// "arn:aws:sqs:{region}:{account}:{name}".
    /// Recognised placeholders: `{region}`, `{account}`, `{name}`.
    /// `{name}` resolves to the value of the `name` field on the model.
    pub template: String,
    /// Single-attribute name to read from TF input first; if present, its
    /// value wins over the template.
    #[serde(default)]
    pub attr_override: Option<String>,
    /// Multi-attribute fallback chain: try each in order, fall back to
    /// the template only if all are absent.
    #[serde(default)]
    pub attr_override_chain: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Field {
    /// Attribute name on the Terraform side (the JSON key in
    /// `ResourceInstance.attributes`).
    pub hcl: String,
    /// Field name on the AWS-side StateView struct. Becomes the field name
    /// of the generated TfModel struct as well; if `hcl != view`, the model
    /// field gets `#[serde(rename = "{hcl}")]`.
    pub view: String,
    /// Type name. One of: "string", "string?", "u32", "i64", "bool",
    /// "tags". `string?` is `Option<String>`; `tags` is
    /// `HashMap<String, String>`.
    #[serde(rename = "type")]
    pub ty: String,
    /// Required-on-input. When true, the model has no default for this field
    /// and serde fails the deserialize if the attribute is missing.
    #[serde(default)]
    pub required: bool,
    /// Default value for optional fields. TOML accepts integers, strings,
    /// and bools.
    #[serde(default)]
    pub default: Option<toml::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ComputedExtract {
    /// Attribute name that should appear in extract JSON.
    pub hcl: String,
    /// Field name on the StateView whose value gets cloned for `hcl`.
    pub source: String,
    /// Type name. Same vocabulary as `Field::ty`. Defaults to "string".
    #[serde(rename = "type", default)]
    pub ty: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ViewExtra {
    /// Field name on the StateView that the generated `into_state_view`
    /// must populate from the conversion context or a constant.
    pub view: String,
    /// One of: "region" (use the resolved region), "account_id"
    /// (use ctx.default_account_id), "none" (set to None for an Option<>),
    /// "default" (use Default::default() for the type).
    pub source: String,
}

pub fn load(path: &Path) -> Result<ServiceSpec> {
    let raw =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let spec: ServiceSpec =
        toml::from_str(&raw).with_context(|| format!("parsing {}", path.display()))?;
    Ok(spec)
}
