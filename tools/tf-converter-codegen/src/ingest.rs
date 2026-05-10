//! Bootstrap a per-service spec by parsing an existing
//! `crates/winterbaume-terraform/src/converters/<service>.rs`.
//!
//! Best-effort: identifies the service crate, the resource types declared
//! by `fn resource_type(&self) -> &str { "..." }`, and every field the
//! converter reads via `optional_*` / `require_str` / `extract_tags` /
//! `attrs.get("...")`. Emits a TOML spec with `mode = "model_only"` so
//! the human reviewer can hand-tune. Multi-resource files are flagged
//! with a TODO comment instructing the reviewer to split fields per
//! resource.
//!
//! This is a bootstrap helper, not a permanent layer — the resulting spec
//! must always be reviewed before generating code.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Result, anyhow};

/// Render a freshly-ingested TOML spec for `service` by parsing
/// `<converters_dir>/<service>.rs`.
pub fn ingest(service: &str, converters_dir: &Path) -> Result<String> {
    let path = converters_dir.join(format!("{service}.rs"));
    let source = std::fs::read_to_string(&path)
        .map_err(|e| anyhow!("read converter source {}: {}", path.display(), e))?;

    let crate_name = detect_crate_name(&source).ok_or_else(|| {
        anyhow!(
            "no `use winterbaume_<crate>::views::...` import in {}",
            path.display()
        )
    })?;
    let resource_types = detect_resource_types(&source);
    if resource_types.is_empty() {
        return Err(anyhow!(
            "no `fn resource_type(&self) -> &str {{ \"...\" }}` declarations found"
        ));
    }
    let view_types = detect_view_types(&source);
    let fields = detect_fields(&source);
    let arn_templates = detect_arn_templates(&source);

    Ok(render_spec(
        service,
        &crate_name,
        &resource_types,
        &view_types,
        &fields,
        &arn_templates,
    ))
}

fn detect_crate_name(source: &str) -> Option<String> {
    // The first `use winterbaume_<crate>::views::...` wins. Skip
    // winterbaume_core / winterbaume_tfstate which every converter uses.
    for line in source.lines() {
        let trimmed = line.trim();
        let Some(rest) = trimmed.strip_prefix("use winterbaume_") else {
            continue;
        };
        let crate_name: String = rest
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect();
        if crate_name == "core" || crate_name == "tfstate" {
            continue;
        }
        if rest[crate_name.len()..].starts_with("::views::")
            || rest[crate_name.len()..].starts_with("::")
        {
            return Some(format!("winterbaume_{crate_name}"));
        }
    }
    None
}

fn detect_resource_types(source: &str) -> Vec<String> {
    let mut out = Vec::new();
    let needle = "fn resource_type(&self) -> &str";
    let mut idx = 0;
    while let Some(pos) = source[idx..].find(needle) {
        let abs = idx + pos + needle.len();
        if let Some(literal) = pull_string_literal_after(&source[abs..])
            && !out.contains(&literal)
        {
            out.push(literal);
        }
        idx = abs;
    }
    out
}

fn pull_string_literal_after(after: &str) -> Option<String> {
    let mut chars = after.char_indices();
    for (_, c) in chars.by_ref() {
        match c {
            '"' => break,
            '{' | ' ' | '\n' | '\r' | '\t' => continue,
            _ => return None,
        }
    }
    let mut out = String::new();
    for (_, c) in chars {
        if c == '"' {
            return Some(out);
        }
        out.push(c);
    }
    None
}

fn detect_view_types(source: &str) -> Vec<String> {
    // Look for view struct names referenced in `views::{...View, ...View}`
    // (single- or multi-line) or `views::ViewName`. Also catch references
    // in struct-construction syntax (`FlowView { ... }`) elsewhere in
    // the source as a fallback when imports use `views::*`.
    let mut out: BTreeSet<String> = BTreeSet::new();
    let mut idx = 0;
    let needle = "::views::";
    while let Some(pos) = source[idx..].find(needle) {
        let abs = idx + pos + needle.len();
        let rest = &source[abs..];
        if rest.starts_with('{') {
            if let Some(close) = rest.find('}') {
                let inner = &rest[1..close];
                for piece in inner.split(',') {
                    let name = piece
                        .trim()
                        .trim_start_matches("self::")
                        .trim_end_matches(';');
                    if name.ends_with("View") && is_valid_rust_ident(name) {
                        out.insert(name.to_string());
                    }
                }
                idx = abs + close;
                continue;
            }
        } else {
            let end = rest
                .find(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                .unwrap_or(rest.len());
            let name = &rest[..end];
            if name.ends_with("View") && is_valid_rust_ident(name) {
                out.insert(name.to_string());
            }
        }
        idx = abs;
    }
    out.into_iter().collect()
}

#[derive(Clone, Debug)]
struct DetectedField {
    hcl: String,
    ty: FieldKind,
    required: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FieldKind {
    String,
    Bool,
    I64,
    Tags,
}

impl FieldKind {
    fn spec_type(self, required: bool) -> &'static str {
        match (self, required) {
            (FieldKind::String, true) => "string",
            (FieldKind::String, false) => "string?",
            (FieldKind::Bool, _) => "bool",
            (FieldKind::I64, _) => "i64",
            (FieldKind::Tags, _) => "tags",
        }
    }
}

fn detect_fields(source: &str) -> Vec<DetectedField> {
    // Use a map keyed by hcl name, with required winning over optional.
    let mut map: BTreeMap<String, DetectedField> = BTreeMap::new();

    for (call, kind, required) in [
        ("require_str(attrs, \"", FieldKind::String, true),
        ("optional_str(attrs, \"", FieldKind::String, false),
        ("optional_bool(attrs, \"", FieldKind::Bool, false),
        ("optional_i64(attrs, \"", FieldKind::I64, false),
    ] {
        for hcl in find_quoted_args(source, call) {
            let entry = map.entry(hcl.clone()).or_insert(DetectedField {
                hcl: hcl.clone(),
                ty: kind,
                required: false,
            });
            // String wins type-wise if both string and non-string are seen
            // (very rare). Required wins if any callsite is require_str.
            if required {
                entry.required = true;
                entry.ty = FieldKind::String;
            }
        }
    }

    // attrs.get("...") — ambiguous; record as optional string. Many of
    // these are passthrough Value reads and the human reviewer should
    // remove them or upgrade to a typed field. We include them to make
    // coverage parity loud rather than silent.
    for hcl in find_quoted_args(source, "attrs.get(\"") {
        if !map.contains_key(&hcl) {
            map.insert(
                hcl.clone(),
                DetectedField {
                    hcl,
                    ty: FieldKind::String,
                    required: false,
                },
            );
        }
    }

    if source.contains("extract_tags(attrs)") && !map.contains_key("tags") {
        map.insert(
            "tags".to_string(),
            DetectedField {
                hcl: "tags".to_string(),
                ty: FieldKind::Tags,
                required: false,
            },
        );
    } else if let Some(existing) = map.get_mut("tags")
        && source.contains("extract_tags(attrs)")
    {
        existing.ty = FieldKind::Tags;
    }

    map.into_values().collect()
}

fn find_quoted_args(source: &str, prefix: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut idx = 0;
    while let Some(pos) = source[idx..].find(prefix) {
        let abs_start = idx + pos + prefix.len();
        if let Some(end) = source[abs_start..].find('"') {
            let value = &source[abs_start..abs_start + end];
            if !value.contains('\n') && is_valid_rust_ident(value) {
                out.push(value.to_string());
            }
            idx = abs_start + end + 1;
        } else {
            break;
        }
    }
    out
}

fn is_valid_rust_ident(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    let first = chars.next().unwrap();
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn detect_arn_templates(source: &str) -> Vec<String> {
    // Capture every `format!("arn:aws:...", ...)` literal so the
    // reviewer can lift it into the converter's hand-written ARN
    // construction.
    let mut out = Vec::new();
    let mut idx = 0;
    let needle = "format!(\"arn:aws:";
    while let Some(pos) = source[idx..].find(needle) {
        let abs_start = idx + pos + "format!(\"".len();
        if let Some(end) = source[abs_start..].find('"') {
            out.push(source[abs_start..abs_start + end].to_string());
            idx = abs_start + end + 1;
        } else {
            break;
        }
    }
    out
}

fn render_spec(
    service: &str,
    crate_name: &str,
    resource_types: &[String],
    view_types: &[String],
    fields: &[DetectedField],
    arn_templates: &[String],
) -> String {
    let mut s = String::new();
    writeln!(
        s,
        "# Auto-ingested by `tf-converter-codegen ingest {service}`."
    )
    .unwrap();
    writeln!(s, "# Review and hand-tune before regenerating.").unwrap();
    if resource_types.len() > 1 {
        writeln!(
            s,
            "# TODO: this converter has multiple resource types {:?}; the",
            resource_types
        )
        .unwrap();
        writeln!(
            s,
            "# field list below is the union across them. Split into one"
        )
        .unwrap();
        writeln!(s, "# `[[resource]]` block per resource and assign fields.").unwrap();
    }
    if !arn_templates.is_empty() {
        writeln!(s, "#").unwrap();
        writeln!(s, "# Detected ARN/URL `format!()` templates (lift into").unwrap();
        writeln!(s, "# the hand-written converter, not the spec):").unwrap();
        for t in arn_templates {
            writeln!(s, "#   {t}").unwrap();
        }
    }
    writeln!(s).unwrap();

    writeln!(s, "[service]").unwrap();
    writeln!(s, "name = {:?}", service).unwrap();
    writeln!(s, "crate_name = {:?}", crate_name).unwrap();
    writeln!(s, "state_view_module = \"views\"").unwrap();
    writeln!(s).unwrap();

    let primary_view = view_types
        .iter()
        .find(|v| !v.ends_with("StateView"))
        .or_else(|| view_types.first())
        .cloned()
        .unwrap_or_else(|| "TODO_VIEW".to_string());
    let model_name = primary_view
        .strip_suffix("View")
        .map(|s| format!("{s}TfModel"))
        .unwrap_or_else(|| "TODO_TfModel".to_string());

    let primary_type = resource_types
        .first()
        .cloned()
        .unwrap_or_else(|| "TODO_TYPE".to_string());

    writeln!(s, "[[resource]]").unwrap();
    writeln!(s, "type = {:?}", primary_type).unwrap();
    writeln!(s, "model = {:?}", model_name).unwrap();
    writeln!(s, "view = {:?}", primary_view).unwrap();
    writeln!(s).unwrap();
    writeln!(s, "[resource.modes]").unwrap();
    writeln!(s, "mode = \"model_only\"").unwrap();
    writeln!(s).unwrap();

    for field in fields {
        writeln!(s, "[[resource.field]]").unwrap();
        writeln!(s, "hcl = {:?}", field.hcl).unwrap();
        writeln!(s, "view = {:?}", field.hcl).unwrap();
        writeln!(s, "type = {:?}", field.ty.spec_type(field.required)).unwrap();
        if field.required {
            writeln!(s, "required = true").unwrap();
        }
        writeln!(s).unwrap();
    }

    s
}
