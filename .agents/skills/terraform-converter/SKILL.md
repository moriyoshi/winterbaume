---
name: terraform-converter
description: Implement or enhance a Terraform converter for an AWS service via the spec-driven codegen pipeline. Creates a new spec + converter or adds missing resource types / fields to an existing one.
argument-hint: <service-name>
user_invocable: true
---

# Terraform Converter — Implement or Enhance

Implement or extend a Terraform converter for an AWS service using winterbaume's spec-driven codegen pipeline. Each TF resource type becomes a `[[resource]]` block in `crates/winterbaume-terraform/specs/<svc>.toml`; the `tf-converter-codegen` tool generates a serde-derived `*TfModel` struct in `crates/winterbaume-tfstate-resource-models/src/<svc>.rs`; the hand-written converter at `crates/winterbaume-terraform/src/converters/<svc>.rs` deserializes Terraform attributes via the typed model and builds the AWS-side StateView.

## Arguments

- `$0` — Service name matching the winterbaume crate suffix (e.g., `s3`, `iam`, `sqs`, `lambda`, `glue`)

---

## Architecture overview

Three files are involved per service:

```
crates/winterbaume-terraform/specs/<svc>.toml              ← hand-written, declarative
crates/winterbaume-tfstate-resource-models/src/<svc>.rs    ← AUTO-GENERATED, do not edit
crates/winterbaume-terraform/src/converters/<svc>.rs       ← hand-written, orchestrates inject/extract
```

The codegen tool is `tools/tf-converter-codegen`. It emits a `*TfModel` Rust struct per `[[resource]]` block in the spec. The `winterbaume-terraform` crate re-exports the generated crate as `crate::generated`, so converter source uses:

```rust
use crate::generated::<svc> as <svc>_gen;
let model: <svc>_gen::<Name>TfModel = serde_json::from_value(instance.attributes.clone())
    .map_err(|e| classify_deserialize_error("aws_<type>", e))?;
```

Almost every spec uses `mode = "model_only"`. Full thin-projection ( `into_state_view` and `From<&StateView>` generated for the resource ) is reserved for trivial 1:1 cases; the SQS pilot used it, but the rest of the codebase has settled on `model_only` because real AWS StateViews carry many constants, computed templates, and nested-block reshaping that the spec format can't express. **Default to `model_only`.**

---

## Step 0: Pick the work

### 0a. Is this an existing service?

```bash
ls crates/winterbaume-terraform/specs/<svc>.toml 2>/dev/null && echo EXISTS || echo NEW
```

### 0b. Check current coverage

```bash
python3 .agents/skills/api-coverage/scripts/generate_terraform_resource_coverage.py
grep -E "^\| <svc> " .agents/docs/TERRAFORM_RESOURCE_COVERAGE.md
```

The report tells you:
- How many TF resource types the service already handles.
- How many the AWS provider schema declares for the service's prefix.
- Which specific resource types are missing.

```bash
python3 -c "
import re
text = open('.agents/docs/TERRAFORM_RESOURCE_COVERAGE.md').read()
sections = re.split(r'^### ', text, flags=re.MULTILINE)
for s in sections:
    if s.startswith('<svc> '):
        print(s.rstrip()); break
"
```

If the service's prefix is `aws_<svc>_` and that prefix collides with another service ( `route53` vs `route53resolver`, `s3` vs `s3control` / `s3tables` ), the resource-coverage script's `PREFIX_OVERRIDES` table at the top of `.agents/skills/api-coverage/scripts/generate_terraform_resource_coverage.py` handles the disambiguation. Add an override entry if your service needs one.

### 0c. Identify candidate resources to add

For each candidate, verify the underlying winterbaume-`<svc>` crate has the API operations to back it ( Create/Delete/Get/List for top-level resources; Attach/Detach/Put for sub-resources ):

```bash
grep -E "^\s*async fn handle_" crates/winterbaume-<svc>/src/handlers.rs | sed -E 's/.*handle_([a-z_]+).*/\1/' | sort -u
```

And confirm the StateView has a matching collection:

```bash
grep -E "^pub struct|^    pub [a-z_]+:" crates/winterbaume-<svc>/src/views.rs | head -60
```

If the view has no collection for the resource and adding one would expand the underlying service crate, **skip the resource** and note it in your final report — it belongs to a follow-up `add-service` task, not this one. Trying to add the converter without a backing view causes the converter to be either dead code or to corrupt round-trips.

---

## Step 1: Edit the spec

### 1a. Spec file layout

`crates/winterbaume-terraform/specs/<svc>.toml`:

```toml
[service]
name = "<svc>"
crate_name = "winterbaume_<svc>"
state_view_module = "views"

[[resource]]
type = "aws_<svc>_<resource>"        # exact TF resource type name
model = "<Name>TfModel"               # generated Rust struct name
view = "<Name>View"                   # informational; matches the AWS-side view struct
[resource.modes]
mode = "model_only"

[[resource.field]]
hcl = "name"                          # TF attribute name
view = "name"                         # Rust field on the generated TfModel
                                       # If hcl != view, codegen emits #[serde(rename = "<hcl>")]
type = "string"
required = true

[[resource.field]]
hcl = "max_message_size"
view = "maximum_message_size"
type = "u32"
default = 262144

[[resource.field]]
hcl = "tags"
view = "tags"
type = "tags"                          # HashMap<String, String>
```

### 1b. Spec field-type vocabulary

The codegen accepts ONLY these types:

| Spec type | Rust type | Notes |
|-----------|-----------|-------|
| `string` | `String` | required-by-default semantics; mark `required = true` if missing-input must error |
| `string?` | `Option<String>` | always defaults to `None` |
| `u32` | `u32` | use `default = N` to set a non-zero default |
| `i64` | `i64` | same |
| `bool` | `bool` | use `default = true/false` |
| `tags` | `HashMap<String, String>` | empty map default |

Anything outside this vocabulary ( `f64`, `Vec<String>`, `Option<i64>`, `Option<bool>`, nested blocks, opaque `serde_json::Value` ) **must be dropped from the spec** and read raw in the converter via `instance.attributes.get("<key>").and_then(...)`. This is the established escape hatch ( apprunner, firehose, appflow, etc. ).

### 1c. Reserved keywords as field names

Rust reserved keywords ( `type`, `move`, `match`, etc. ) appear regularly in TF attributes. The codegen escapes them as `r#type` etc. You normally won't need to do anything; if it looks weird, look at how `ivs.toml` renames `type` → `channel_type` to avoid the `r#type` form in `view` while keeping `hcl = "type"`.

### 1d. Common spec patterns

**Field rename** ( TF `max_message_size` → view `maximum_message_size` ):
```toml
[[resource.field]]
hcl = "max_message_size"
view = "maximum_message_size"
type = "u32"
default = 262144
```

**Required field with no default**:
```toml
[[resource.field]]
hcl = "name"
view = "name"
type = "string"
required = true
```

**Optional with `unwrap_or_else` fallback in converter** ( spec is `string?`, the converter applies the fallback after deserialize ):
```toml
[[resource.field]]
hcl = "arn"
view = "arn"
type = "string?"
```

```rust
let arn = model.arn.unwrap_or_else(|| {
    format!("arn:aws:sqs:{}:{}:{}", region, ctx.default_account_id, model.name)
});
```

---

## Step 2: Regenerate the model

```bash
./.agents/bin/cargo.sh run -p tf-converter-codegen --quiet -- gen <svc>
```

This writes `crates/winterbaume-tfstate-resource-models/src/<svc>.rs`. Run `gen-all` instead if you've edited multiple specs:

```bash
./.agents/bin/cargo.sh run -p tf-converter-codegen --quiet -- gen-all
```

Generated files always have the header:

```rust
//! Auto-generated by tf-converter-codegen. Do not edit manually.
//! Source: specs/<svc>.toml
//! Regenerate with: cargo run -p tf-converter-codegen -- gen <svc>
```

A golden test verifies these files are committed in sync: `cargo test -p tf-converter-codegen` runs `check` against every spec and fails if any regenerate produces a diff. Always commit the regenerated `*<svc>.rs` alongside the spec change.

---

## Step 3: Write the converter

`crates/winterbaume-terraform/src/converters/<svc>.rs`. Established skeleton ( mirror the wave-1/2/3 IAM converters or s3 / route53 ):

```rust
//! Terraform converters for <Service> resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_<svc>::<svc>Service;
use winterbaume_<svc>::views::{<svc>StateView, <ResourceView>};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::<svc> as <svc>_gen;
use crate::util::{classify_deserialize_error, extract_region};

pub struct AwsXxxConverter {
    service: Arc<<svc>Service>,
}

impl AwsXxxConverter {
    pub fn new(service: Arc<<svc>Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsXxxConverter {
    fn resource_type(&self) -> &str { "aws_<svc>_xxx" }

    // Optional: declare ordering dependencies for inject ( the injector
    // topo-sorts by these strings ).
    // fn depends_on_types(&self) -> Vec<&str> { vec!["aws_<svc>_parent"] }

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

impl AwsXxxConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: <svc>_gen::XxxTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_<svc>_xxx", e))?;

        // ARN/URL templates, constants, and any nested-block reshaping
        // happen here, NOT in the spec.
        let arn = model.arn.unwrap_or_else(|| {
            format!("arn:aws:<svc>:{}:{}:xxx/{}",
                    region, ctx.default_account_id, model.name)
        });

        // Build the AWS-side view.
        let xxx_view = XxxView {
            name: model.name.clone(),
            arn,
            tags: model.tags,
            // ...
        };

        let mut state_view = <svc>StateView::default();
        state_view.xxxs.insert(model.name, xxx_view);
        self.service.merge(&ctx.default_account_id, &region, state_view).await?;

        Ok(ConversionResult { region, warnings: vec![] })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self.service.snapshot(&ctx.default_account_id, &ctx.default_region).await;
        let mut results = vec![];
        for xxx in view.xxxs.values() {
            let attrs = serde_json::json!({
                "id": xxx.name,
                "arn": xxx.arn,
                "name": xxx.name,
                "tags": xxx.tags,
                // ... every key from the spec's [[resource.field]] entries,
                //     plus any read-only / computed fields the TF schema
                //     declares ( tags_all, status, etc. ).
            });
            results.push(ExtractedResource {
                name: xxx.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
```

### Key idioms

**Sub-resource modifiers** ( `aws_iam_role_policy`, `aws_s3_bucket_versioning`, etc. ): the underlying merge is replace-not-append on the parent view, so use **snapshot+mutate+restore**:

```rust
let mut snapshot = self.service.snapshot(&ctx.default_account_id, &region).await;
if let Some(role) = snapshot.roles.get_mut(&model.role) {
    role.inline_policies.retain(|p| p.policy_name != model.name);
    role.inline_policies.push(InlinePolicyView {
        policy_name: model.name,
        policy_document: model.policy,
    });
}
self.service.restore(&ctx.default_account_id, &region, snapshot).await?;
```

Mirror `AwsIamRolePolicyConverter` in `crates/winterbaume-terraform/src/converters/iam.rs`.

**Vec<String> / nested-block raw reads**:

```rust
let users: Vec<String> = instance.attributes.get("users")
    .and_then(|v| v.as_array())
    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
    .unwrap_or_default();

let trigger_config = instance.attributes.get("trigger_config")
    .and_then(|v| v.as_array())
    .and_then(|arr| arr.first())
    .cloned()
    .unwrap_or(serde_json::Value::Null);
```

**Nested-block shape reshape** ( the AppFlow case ): the TF state stores singleton blocks as length-1 arrays; the AWS REST JSON wants discriminated objects. The reshape lives entirely in the converter, between the model deserialize and the StateView construction — never in the spec. See `crates/winterbaume-terraform/src/converters/appflow.rs` `tf_to_aws_*` helpers.

**Tag merging** ( `tags` + `tags_all` precedence ): the existing helper `crate::util::extract_tags(attrs)` merges both ( `tags_all` first, then `tags` overrides ). Use it when the converter needs the merged result:

```rust
use crate::util::extract_tags;
let tags = extract_tags(&instance.attributes);
```

Mark the spec's `tags` field as `type = "tags"` only when you want serde to read `tags` directly into the model. If you use `extract_tags`, omit the `tags` spec field.

**ARN / URL templates**: hand-written in the converter using `format!()` with placeholders from `region`, `ctx.default_account_id`, and `model.<id_field>`. Never in the spec.

**Identifier synthesis**: when TF doesn't pass an id, synthesise via `uuid::Uuid::new_v4()` or the established `generate_id("PREFIX")` helper ( `iam.rs` ). Stable across calls is critical for waiter-friendly extracts.

---

## Step 4: Register the converter

Add the new converter struct to `crates/winterbaume-server/src/main.rs` in alphabetical order within the service's block:

```rust
injector.register(<svc>::AwsXxxConverter::new(Arc::clone(&injectable.<svc>)));
```

If the resource needs multi-scope ( regional ) handling and the service offers `scopes_with_state()`, use:

```rust
{
    let svc = Arc::clone(&injectable.<svc>);
    injector.register_with_scopes(
        <svc>::AwsXxxConverter::new(Arc::clone(&injectable.<svc>)),
        move || svc.scopes_with_state(),
    );
}
```

( Pattern in main.rs for `aws_s3_bucket` and similar account-spanning resources. )

The `winterbaume-terraform/src/converters/mod.rs` already has `pub mod <svc>;` for every service; you don't normally need to touch it. New service crates do require a new `Cargo.toml` dependency on `winterbaume-<svc>` plus a `pub mod <svc>;` line.

---

## Step 5: Verify

Per-crate gate:

```bash
./.agents/bin/cargo.sh fmt -p winterbaume-terraform 2>&1 | tail -3
./.agents/bin/cargo.sh fmt -p winterbaume-tfstate-resource-models 2>&1 | tail -3
./.agents/bin/cargo.sh clippy -p winterbaume-terraform --all-targets --all-features -- -D warnings 2>&1 | tail -5
./.agents/bin/cargo.sh clippy -p winterbaume-tfstate-resource-models --all-targets --all-features -- -D warnings 2>&1 | tail -5
./.agents/bin/cargo.sh clippy -p winterbaume-server --all-targets --all-features -- -D warnings 2>&1 | tail -5
```

Run the integration tests:

```bash
./.agents/bin/cargo.sh test -p winterbaume-terraform --no-fail-fast 2>&1 | grep -E "test result|FAILED" | tail -5
```

Expect `293+ passed; 0 failed`. Any regression in the existing 293 integration tests means your changes broke a prior converter — investigate immediately, don't push past it.

The golden test for the generated code:

```bash
./.agents/bin/cargo.sh test -p tf-converter-codegen 2>&1 | grep "test result:" | tail -3
```

The single passing case verifies every spec re-emits to byte-equal output as the committed generated file — meaning your spec edit produced exactly what's checked in. Failures here mean you forgot to run `gen <svc>` after editing the spec.

---

## Step 6: Refresh the coverage report

```bash
rm -f .agents/docs/TERRAFORM_RESOURCE_COVERAGE.md
python3 .agents/skills/api-coverage/scripts/generate_terraform_resource_coverage.py
grep -E "^\| <svc> " .agents/docs/TERRAFORM_RESOURCE_COVERAGE.md
```

Note: there's a second, complementary coverage script at
`.agents/skills/api-coverage/scripts/generate_terraform_converter_coverage.py`
that measures *per-attribute* coverage within each resource ( the inject %
and extract % thresholds — inject ≥ 60%, extract ≥ 50% for "excellent" ).
Use it when the goal is to deepen an existing resource's field coverage
rather than add new resource types.

---

## Common pitfalls

1. **`*StateView::default()` cascade.** Every `*StateView` and its component `*View` structs must `#[derive(Default)]`. Always construct with `<svc>StateView::default()` then mutate the relevant collection — never enumerate every field, because new fields on the View shape will silently get zero values from the literal construction. The bug only surfaces in the 293 integration tests.

2. **Spec format vocabulary boundary.** If you find yourself wanting to declare `Vec<String>` or `f64` in the spec, stop. Read it raw in the converter and document the omission with a TOML comment like `# <field> is Vec<String>, read raw in converter`.

3. **JSON-shape mismatch ( AppFlow case ).** TF singleton-array blocks must be reshaped before they're stored as AWS-side `serde_json::Value`. Doing the reshape in `do_inject` between the model deserialize and the StateView construction is the documented pattern. Never let the codegen 1:1-rename a nested block.

4. **`id` / `name` swap on extract.** The TF resource id and the human name are different things. For extract, `ExtractedResource.name` is the human-readable TF resource label; the `id` attribute in the emitted JSON is the AWS-side id ( ARN, UUID, etc. ). The placement-group case is the canonical bug from LTM.

5. **Multi-target resources** ( `aws_iam_policy_attachment` attaching to users + groups + roles ): extract returns `Ok(vec![])` intentionally because the resource has no view-side counterpart — the per-entity converters cover the same data. Document this with a comment.

6. **Inject-only resources** ( `aws_route` ): extract returns `Ok(vec![])`. Document.

7. **`from_value` clone hygiene.** `serde_json::from_value(instance.attributes.clone())` is the correct invocation. Skipping the clone borrows from the read-only `&ResourceInstance` and won't compile.

8. **`classify_deserialize_error`.** Always use the `crate::util::classify_deserialize_error("aws_<type>", e)` helper to map serde errors onto the existing `ConversionError::MissingAttribute` / `InvalidAttribute` variants. Don't construct error variants directly — the helper handles "missing field `x`" parsing.

9. **Reserved keyword fields.** If you use `hcl = "type"` and `view = "type"`, codegen emits `pub r#type: String` and serde rename works correctly. If you want a cleaner field name on the Rust side, use `view = "channel_type"` ( IVS pattern ).

10. **State-view literal drift.** When the underlying service crate evolves ( adds a new view field, changes a type from `Vec<X>` to `HashMap<K, X>` ), every converter constructing the view by literal `XxxView { ... }` breaks. The per-crate clippy gate doesn't catch this — only `winterbaume-terraform`'s 293 integration tests do.
