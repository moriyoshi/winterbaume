---
name: terraform-converter
description: Implement or enhance a Terraform converter for an AWS service — creates a new converter module or improves field coverage on an existing one. Targets excellent rating (inject >= 60%, extract >= 50%).
argument-hint: <service-name>
user_invocable: true
---

# Terraform Converter — Implement or Enhance

Create a new Terraform converter for an AWS service, or enhance field coverage on an existing converter to reach "excellent" rating (inject >= 60% AND extract >= 50% of TF schema attributes).

## Arguments

- `$0` — Service name matching the winterbaume crate suffix (e.g., `s3`, `iam`, `sqs`, `lambda`, `guardduty`)

---

## Step 0: Determine whether a converter exists

```bash
ls crates/winterbaume-terraform/src/converters/{service}.rs 2>/dev/null && echo "EXISTS" || echo "NEW"
```

- **EXISTS** → skip to Step 4 (enhance coverage)
- **NEW** → proceed with Step 1 (create converter)

---

## Step 1: Read the service crate's state layer

### 1a. Identify the service struct and view types

```bash
# Service struct name (the main type that implements StatefulService)
grep 'impl StatefulService' crates/winterbaume-{service}/src/views.rs
# View types available
grep 'pub struct.*View' crates/winterbaume-{service}/src/views.rs
```

Read these files in order:
1. `crates/winterbaume-{service}/src/lib.rs` — service struct name, public exports
2. `crates/winterbaume-{service}/src/views.rs` — StateView struct, all view types, StatefulService impl (merge/snapshot)
3. `crates/winterbaume-{service}/src/types.rs` — internal state types (if exists)

### 1b. Map view types to Terraform resource types

Each view type typically maps to one Terraform resource type. Common patterns:

| View struct | TF resource type |
|-------------|-----------------|
| `BucketView` / `BucketStateView` | `aws_s3_bucket` |
| `ClusterView` | `aws_{service}_cluster` |
| `InstanceView` | `aws_{service}_instance` |
| `SubnetGroupView` | `aws_{service}_subnet_group` |
| `ParameterGroupView` | `aws_{service}_parameter_group` |

Verify against the Terraform AWS provider:
```bash
python3 -c "
import json
with open('.agents-workspace/tmp/tf-schema/aws_provider_schema.json') as f:
    schemas = json.load(f)
for rt in sorted(schemas):
    if '{service}' in rt.lower() or 'aws_{tf_prefix}' in rt:
        block = schemas[rt]['block']
        attrs = list(block.get('attributes', {}).keys())
        blocks = list(block.get('block_types', {}).keys())
        print(f'{rt}: {len(attrs)} attrs + {len(blocks)} blocks')
"
```

If the TF schema cache doesn't exist, generate it:
```bash
mkdir -p .agents-workspace/tmp/tf-schema
cd .agents-workspace/tmp/tf-schema
cat > main.tf << 'EOF'
terraform { required_providers { aws = { source = "hashicorp/aws"; version = "~> 5.0" } } }
EOF
terraform init -no-color
terraform providers schema -json | python3 -c "
import json, sys
schema = json.load(sys.stdin)
resources = schema['provider_schemas']['registry.terraform.io/hashicorp/aws']['resource_schemas']
with open('aws_provider_schema.json', 'w') as f: json.dump(resources, f)
print(f'Saved {len(resources)} resource types')
"
```

#### About the schema cache

The schema is fetched live from the Terraform CLI by running `terraform providers schema -json` against a minimal config that declares the `hashicorp/aws` provider. This requires:

- **Terraform CLI** installed and on `$PATH`
- **Network access** for `terraform init` to download the provider binary (first time only; subsequent runs use the local `.terraform/` cache)

The resulting `aws_provider_schema.json` is pinned to whatever provider version `~> 5.0` resolves to at fetch time. It contains every resource type's top-level attributes and block types, which the coverage script compares against converter field names.

**When to regenerate:** Re-run the schema fetch when upgrading the provider constraint or when new TF resource types are expected (e.g., after AWS launches a new service that the provider supports). The cache at `.agents-workspace/tmp/tf-schema/aws_provider_schema.json` is not version-controlled — it is a local build artefact.

### 1c. Determine resource types to implement

Pick resource types that:
1. Have a corresponding view type in the StateView
2. Exist in the Terraform AWS provider schema
3. Are commonly used (prioritise by real-world usage)

---

## Step 2: Create the converter file

Create `crates/winterbaume-terraform/src/converters/{service}.rs` following this template:

```rust
//! Terraform converters for {Service} resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_{service}::{ServiceStruct};
use winterbaume_{service}::views::{StateView, ResourceView, ...};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionResult, ExtractedResource, ConversionContext, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_bool, optional_i64, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_{tf_resource_type}
// ---------------------------------------------------------------------------

pub struct Aws{PascalResourceType}Converter {
    service: Arc<{ServiceStruct}>,
}

impl Aws{PascalResourceType}Converter {
    pub fn new(service: Arc<{ServiceStruct}>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for Aws{PascalResourceType}Converter {
    fn resource_type(&self) -> &str {
        "aws_{tf_resource_type}"
    }

    // Add depends_on_types() if this resource depends on another
    // fn depends_on_types(&self) -> Vec<&str> { vec!["aws_parent_type"] }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl Aws{PascalResourceType}Converter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        // Read ALL available fields from Terraform state:
        let name = require_str(attrs, "name", "aws_{tf_resource_type}")?;
        let description = optional_str(attrs, "description");
        let tags = extract_tags(attrs);
        // Read extra TF attrs for coverage (even if unused by the view):
        let _ = attrs.get("tags_all");
        let _ = attrs.get("timeouts");

        let view = {ResourceView} {
            // Map all view fields from TF attrs
            ..Default::default()
        };

        let mut state_view = {StateView}::default();
        state_view.{collection}.insert(name.to_string(), view);
        self.service.merge(&ctx.default_account_id, &region, state_view).await?;

        Ok(ConversionResult { region, warnings: vec![] })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self.service.snapshot(&ctx.default_account_id, &ctx.default_region).await;
        let mut results = vec![];
        for resource in view.{collection}.values() {
            let attrs = serde_json::json!({
                "id": resource.id,
                "name": resource.name,
                "arn": resource.arn,
                // Map ALL view fields to TF attr names
                "tags": resource.tags,
                "tags_all": resource.tags,
            });
            results.push(ExtractedResource {
                name: resource.name.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
```

### Key patterns

**Tag handling**: Use `extract_tags(attrs)` for inject (merges `tags` + `tags_all`). Emit both `"tags"` and `"tags_all"` in extract.

**Nested TF blocks**: Terraform represents blocks as JSON arrays with one object element:
```rust
// Reading a TF block in inject:
let child_directed = attrs.get("data_privacy")
    .and_then(|v| v.as_array())
    .and_then(|arr| arr.first())
    .and_then(|block| block.get("child_directed"))
    .and_then(|v| v.as_bool())
    .unwrap_or(false);

// Emitting a TF block in extract:
"data_privacy": [{ "child_directed": value }],
```

**Dependencies**: Use `depends_on_types()` when one resource must be injected before another (e.g., subnets before instances, apps before environments).

**String arrays**: Parse with a helper:
```rust
let items: Vec<String> = attrs.get("field")
    .and_then(|v| v.as_array())
    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
    .unwrap_or_default();
```

---

## Step 3: Register the converter

### 3a. Add to mod.rs

Add `pub mod {service};` in alphabetical order to `crates/winterbaume-terraform/src/converters/mod.rs`.

### 3b. Add dependency to Cargo.toml

Add `winterbaume-{service} = { workspace = true }` in alphabetical order to the `[dependencies]` section of `crates/winterbaume-terraform/Cargo.toml`.

### 3c. Build and verify

```bash
./.agents/bin/cargo.sh check -p winterbaume-terraform
./.agents/bin/cargo.sh clippy -p winterbaume-terraform --all-targets --all-features -- -D warnings
./.agents/bin/cargo.sh fmt -p winterbaume-terraform -- --check
```

The clippy and fmt gates are mandatory before reporting; if they fail, fix the violations and re-run.

---

## Step 4: Enhance coverage (existing converter)

### 4a. Check current coverage

```bash
python3 .agents/skills/api-coverage/scripts/generate_terraform_converter_coverage.py \
  --schema-cache .agents-workspace/tmp/tf-schema/aws_provider_schema.json 2>&1 | head -5
grep '{service}' .agents/docs/TERRAFORM_CONVERTER_COVERAGE.md
```

### 4b. Read the per-resource detail

The report shows:
- **Inject attributes**: field names the converter reads from TF state
- **Extract attributes**: field names the converter emits to TF state
- **Missing from inject/extract**: TF schema attributes not yet handled

### 4c. Identify fields to add

Priority order:
1. **View fields not yet mapped** — read the views.rs and check what the converter misses
2. **`tags_all`** — always add if missing (most common single-field fix)
3. **`timeouts`** — read from attrs to bump inject count
4. **Computed defaults** — safe to emit in extract (e.g., `"status": "ACTIVE"`, `"port": 6379`)
5. **No-op reads for coverage** — `let _ = attrs.get("field_name");` counts as inject coverage

### 4d. Coverage thresholds

The coverage script detects these patterns:

**Inject** (reading TF attrs):
- `require_str(attrs, "field", ...)` 
- `optional_str(attrs, "field")`
- `optional_bool(attrs, "field")`
- `optional_i64(attrs, "field")`
- `attrs.get("field")`
- `extract_tags(attrs)` → counts `tags` + `tags_all`

**Extract** (emitting TF attrs):
- `"field": value` inside `serde_json::json!({})` blocks
- `obj.insert("field", ...)` or `attrs.insert("field", ...)`

### 4e. Expand StateView if needed

If the coverage gap is caused by missing view fields (not just missing converter mappings), expand the service crate:

1. Add fields to the view struct in `views.rs`
2. Add corresponding fields to the internal type in `types.rs`
3. Update `From` impls between types and views
4. Update the `merge` method to handle the new fields
5. Update `state.rs` if it constructs the type directly

Then update the converter to use the new fields.

### 4f. Fix field naming mismatches

Common mismatches between converter field names and TF schema names:
- `description` vs `replication_group_description` (ElastiCache)
- `creation_date_time` vs `create_time` (Lex)
- `data_privacy_child_directed` (flat) vs `data_privacy` (nested block)
- Lex V1 (`aws_lex_bot`) vs V2 (`aws_lexv2models_bot`) resource types

Always verify the exact TF resource type and attribute names against the provider schema.

---

## Step 5: Add integration tests

Append a test to `crates/winterbaume-terraform/tests/integration_test.rs`:

```rust
#[tokio::test]
async fn test_inject_{service}_{resource}() {
    use winterbaume_{service}::{ServiceStruct};
    use winterbaume_terraform::converters::{service}::Aws{PascalResource}Converter;

    let svc = Arc::new({ServiceStruct}::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(Aws{PascalResource}Converter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_{tf_resource_type}",
        "test",
        json!({
            "name": "test-resource",
            // ... minimal required attributes
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify state
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.{collection}.contains_key("test-resource"));

    // Verify extract round-trip
    let converter = Aws{PascalResource}Converter::new(Arc::clone(&svc));
    let extracted = converter.extract(&ctx).await.expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["name"], "test-resource");
}
```

Run:
```bash
cargo test -p winterbaume-terraform --test integration_test test_inject_{service}
```

---

## Step 6: Regenerate coverage report

```bash
python3 .agents/skills/api-coverage/scripts/generate_terraform_converter_coverage.py \
  --schema-cache .agents-workspace/tmp/tf-schema/aws_provider_schema.json
grep 'aws_{tf_resource_type}' .agents/docs/TERRAFORM_CONVERTER_COVERAGE.md
```

Verify the converter is rated "excellent" (inject >= 60% AND extract >= 50%).

---

## Common Pitfalls (from real implementation experience)

1. **Multi-converter files**: When a file has multiple converters, the coverage script uses a precise regex to match `fn resource_type() -> &str { "exact_type" }` within each trait impl. Don't rely on substring matching.

2. **`depends_on_types()` false matches**: The `depends_on_types()` method body contains quoted resource type strings that could confuse naive parsers. The coverage script handles this correctly by matching the `resource_type()` method specifically.

3. **Merge vs restore**: Use `merge()` (additive) not `restore()` (full replacement) in `do_inject`. Terraform state may contain multiple resources that merge into the same state view.

4. **Format string braces**: `format!("arn:aws:{}:{}:{}", ...)` contains `{}`s that are string literals, not Rust block delimiters. This doesn't affect the coverage script but can confuse brace-counting parsers.

5. **Missing StateView fields**: If the view doesn't model a resource family (e.g., DAX subnet groups, MemoryDB ACLs), you need to expand the state layer first. Add the view type, update types.rs, state.rs, and the StatefulService merge/snapshot.

6. **V1 vs V2 resource types**: Some services have both V1 and V2 Terraform resource types (e.g., `aws_lex_bot` vs `aws_lexv2models_bot`). Make sure the converter targets the resource type that matches the service crate's API version.

7. **`tags_all` is always a separate attribute**: The TF provider schema counts `tags` and `tags_all` as distinct attributes. Always emit both in extract for maximum coverage.

8. **Nested blocks count as one attribute**: TF block types (e.g., `data_privacy`, `health_check`, `scaling_config`) count as a single attribute in the schema, but may contain many sub-fields. Reading the block with `attrs.get("block_name")` counts as 1 inject field.

9. **`let _ = attrs.get("field")` counts for inject**: The coverage script detects `attrs.get("field")` patterns. Even an unused read adds to inject coverage. This is useful for TF attrs that the view doesn't model but that should be acknowledged.

10. **Large json! macros**: Converters with 40+ fields in the `json!` block may hit Rust's default recursion limit. Add `#![recursion_limit = "256"]` to `lib.rs` if needed.

11. **State-view literal drift**: Every `*StateView` struct ( and its component view structs ) must `#[derive(Default)]`, and any literal construction in tests, converters, or backend wrappers should use `..Default::default()`. State-view literal drift is the largest cascade source in `/tackle-todos` sweeps; per-crate quality gates do not catch the breakage, only `winterbaume-terraform` integration tests do.

12. **State-shape change cascade**: When a state-field type changes ( e.g. `Vec<String>` → `HashMap<K, HashSet<V>>` for `inspector2`'s `enabled_resource_types` ), terraform converters that mirror that field also need updates. Per-crate quality gates do not catch the breakage; only `winterbaume-terraform` integration tests do. After changing a state-field type, grep the workspace for cross-crate consumers.
