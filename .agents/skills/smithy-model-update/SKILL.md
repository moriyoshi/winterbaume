---
name: smithy-model-update
description: Use when the vendored AWS Smithy models under vendor/api-models-aws change, or when API coverage, README service tables, TODO unimplemented-service lists, generated model/wire files, or service dossiers need to be reconciled with the current Smithy checkout.
---

# Smithy Model Update

## Purpose

Coordinate the repo-wide response to `vendor/api-models-aws` changes. A model refresh can affect service discovery, generated `model.rs` / `wire.rs`, API coverage, README output, docs, service dossiers, and `.agents/docs/TODO.md`; handle those surfaces together so stale model state does not leak into later work.

## Quick Audit

Run the bundled drift check first:

```bash
python3 .agents/skills/smithy-model-update/scripts/audit_smithy_model_update.py
```

Use `--check` for a gate and `--json` when another script or agent needs structured output. The audit compares:

- model directories in `vendor/api-models-aws/models`
- service crates under `crates/winterbaume-*`
- `CRATE_TO_MODEL` in `.agents/skills/api-coverage/scripts/generate_coverage.py`
- `CRATE_DISPLAY_INFO` in `.agents/skills/update-readme/scripts/update_readme.py`
- the flat `### Services Not Yet Implemented` section in `.agents/docs/TODO.md`

If the audit reports drift, patch the affected files before regenerating derived docs.

## Workflow

1. Identify what changed in the Smithy checkout:

   ```bash
   git diff -- vendor/api-models-aws
   find vendor/api-models-aws/models -maxdepth 1 -type d | sort
   ```

2. Reconcile service identity:

   - For every implemented service crate, ensure `CRATE_TO_MODEL` maps the crate package name to the exact model directory.
   - Add compact-name exceptions where the crate drops hyphens or AWS SDK naming differs, such as `winterbaume-apigatewaymanagement` -> `apigatewaymanagementapi` or `winterbaume-costandusagereport` -> `cost-and-usage-report-service`.
   - Ensure `CRATE_DISPLAY_INFO` has a human-readable name for every service crate that appears in coverage output.
   - Keep helper crates out of service coverage maps: `winterbaume-core`, `winterbaume-server`, `winterbaume-terraform`, `winterbaume-tfstate`, parser/evaluator helper crates, Redis adapters, and `winterbaume-ec2-generated`.

3. Reconcile TODO:

   - Update `.agents/docs/TODO.md` `### Services Not Yet Implemented` to match Smithy models that have no service crate.
   - Keep richer service-specific work items out of the flat list when they already exist above it. Today `pinpoint-sms-voice-v2` is tracked as a richer follow-up.
   - Preserve unrelated staged or user edits in TODO; only touch the service list unless the user asks for broader cleanup.

4. Regenerate derived reports when mappings change:

   ```bash
   python3 .agents/skills/api-coverage/scripts/generate_coverage.py
   python3 .agents/skills/update-readme/scripts/update_readme.py
   ```

   If remote moto/floci/kumo/fakecloud refresh fails because of network restrictions, rerun with approval or use the existing cache if the task only needs local Smithy/crate alignment.

5. If generated `model.rs` or `wire.rs` is stale, do not edit it manually. Edit `tools/smithy-codegen/src/gen_serializers.rs` only for generator bugs, rebuild through `./.agents/bin/cargo.sh`, then regenerate the affected crate using the model directory from the service dossier or `smithy-codegen list-services`.

6. For service-specific behaviour changes, read `.agents/docs/services/INDEX.md` and the matching service dossier before editing handlers, state, Terraform converters, or tests. Update the dossier if the new model adds durable facts that future agents need.

## Validation

After edits, rerun:

```bash
python3 .agents/skills/smithy-model-update/scripts/audit_smithy_model_update.py --check
```

For documentation-only mapping/TODO work, this audit plus `git diff` is usually enough. For Rust source or generated code changes, also run the mandatory per-crate gate from `AGENTS.md`:

```bash
./.agents/bin/cargo.sh clippy -p <crate> --all-targets --all-features -- -D warnings
./.agents/bin/cargo.sh fmt -p <crate> -- --check
```

## Common Failure Modes

- Coverage says a newly implemented crate is unimplemented because `CRATE_TO_MODEL` was not updated.
- README generation falls back to poor service names because `CRATE_DISPLAY_INFO` was not updated.
- TODO drifts because it was copied from an old `API_COVERAGE.md` instead of the current Smithy checkout.
- A model slug and crate suffix differ only by hyphens, but some services need explicit exceptions; do not assume every compact mapping is semantically correct.
- Running bare `cargo` bypasses the project cache wrapper. Always use `./.agents/bin/cargo.sh` for Rust builds or codegen rebuilds.
