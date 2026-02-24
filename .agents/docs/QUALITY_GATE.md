# Quality Gate

This is the canonical checklist for publication-quality Winterbaume changes. Use it to verify service crates, shared helper crates, generated code, Terraform support, documentation, and release workflow changes.

For deeper rationale, use `.agents/docs/ARCHITECTURE.md` and the relevant LTM synthesis documents. This file should stay organised as a gate: concise rules, commands, and known exceptions.

## 1. Baseline Commands

Agent-run cargo commands must go through the wrapper:

```bash
./.agents/bin/cargo.sh <args>
```

The wrapper sets the per-session target directory, `RUSTC_WRAPPER`, cache roots, and the shared sccache-wrapper scoreboard. Bare `cargo`, even with a custom `CARGO_TARGET_DIR`, bypasses those exports and produces builds that do not use the shared cache.

Use the smallest gate that matches the work:

- For a Rust code change in one service crate:
  ```bash
  ./.agents/bin/cargo.sh clippy -p <crate> --all-targets --all-features -- -D warnings
  ./.agents/bin/cargo.sh fmt -p <crate> -- --check
  ```
- If `fmt --check` fails:
  ```bash
  ./.agents/bin/cargo.sh fmt -p <crate>
  ```
- For workspace-level release or quality-audit work:
  ```bash
  ./.agents/bin/cargo.sh check --workspace
  ./.agents/bin/cargo.sh clippy --workspace --all-targets -- -D warnings
  ./.agents/bin/cargo.sh fmt --all -- --check
  ```
- For generated-code changes, rebuild the generator and regenerate affected crates before trusting downstream checks.

Recurring lint note: `clippy::unnecessary_sort_by` fires on `sort_by(|a, b| b.key.cmp(&a.key))` for `Copy` keys. Use `sort_by_key(|x| std::cmp::Reverse(x.key))` for descending order or `sort_by_key(|x| x.key)` for ascending order.

## 2. Service Crate Shape

Every AWS service crate should follow the same ownership split:

- `src/lib.rs`: module declarations and public service exports.
- `src/handlers.rs`: protocol dispatch, request parsing, operation routing, response shaping, and AWS-facing error mapping.
- `src/state.rs`: mutable runtime state, validation, state transitions, and domain-specific errors.
- `src/types.rs`: service-local domain types.
- `src/views.rs`: stable `StateView` types plus `StatefulService` implementation.
- `src/backend.rs`: optional object-safe backend trait and in-memory adapter.
- `src/model.rs`: generated Smithy-derived shapes.
- `src/wire.rs`: generated serialisers, deserialisers, and model re-exports.
- `tests/integration_test.rs`: SDK integration and parity regressions.
- `tests/scenario_test.rs`: multi-operation scenarios for stateful services.

Gate checks:

- [ ] The crate layout matches the files it needs; generated crates expose `model` and `wire`.
- [ ] `handlers.rs` owns AWS wire concerns; `state.rs` does not construct `{Service}Error { error_type, message, status }`.
- [ ] State-layer failures are domain-specific enums, typically via `thiserror`.
- [ ] Handler mappings use the enum message, usually `err.to_string()`, as the single source for AWS-facing error text.
- [ ] No `error_type()` or `status()` helpers exist on state-layer enums:
  ```bash
  rg 'fn error_type|fn status\b' crates/winterbaume-*/src/state.rs
  ```
- [ ] New service crates are wired through workspace membership, workspace dependencies, root optional dependency, `service-<name>` feature, `full` feature, root dev-dependencies, server registration, examples or docs as applicable, tests, and TODO status.

## 3. State Views and Scope

`StatefulService` is the stable snapshot, restore, merge, Terraform, and state-injection contract. Runtime state may be shaped for fast handling; views are the durable serde-facing contract.

Gate checks:

- [ ] `snapshot()`, `restore()`, `merge()`, and `notify_state_changed()` are implemented for service crates.
- [ ] The top-level `*StateView` represents one account and region worth of state. It does not carry `account_id`, `region`, `accounts`, `regions`, or account/region-keyed wrapper maps unless the AWS service is genuinely global and that exception is documented.
- [ ] Resource-level account, region, owner, ARN, peer-account, bucket-location, or replication metadata is present only when it is part of the AWS resource identity or wire contract.
- [ ] `snapshot(account_id, region)`, `restore(account_id, region, view)`, and `merge(account_id, region, view)` use the supplied scope and do not silently substitute `DEFAULT_ACCOUNT_ID`, `ctx.default_region`, `"123456789012"`, `"us-east-1"`, or a shared global bucket.
- [ ] `restore()` is a full replacement; `merge()` is additive and must not delete resources absent from the incoming view.
- [ ] Collection fields and newly added view fields use `#[serde(default)]` so older snapshots continue to deserialise.
- [ ] Top-level `*StateView` types that external callers construct derive `Default`; injector and test fixtures prefer small builder helpers plus `..Default::default()` over exhaustive field literals.
- [ ] Transient runtime state, caches, counters, derived indices, deleted handles, and in-flight uploads are rebuilt during `restore()` rather than persisted directly.
- [ ] `notify_state_changed(...).await` is called only after successful mutations and after write guards are dropped.
- [ ] No `RwLock` guard is held across blob-store awaits.
- [ ] Derived services do not duplicate durable ownership from upstream services; DynamoDB Streams is the reference split.
- [ ] Terraform converters only claim support for resource families that `views.rs` can faithfully represent and `merge()` can rehydrate.
- [ ] Suspicious scope fallbacks have been reviewed:
  ```bash
  rg 'DEFAULT_ACCOUNT_ID|123456789012|us-east-1|_account_id|_region' \
    crates/winterbaume-*/src/views.rs \
    crates/winterbaume-*/src/state.rs \
    crates/winterbaume-*/src/backend.rs
  ```

## 4. State View Fidelity

Views should capture durable resource configuration, not merely enough fields for one test.

Gate checks:

- [ ] Durable resource configuration is represented in the view. S3 is the reference for breadth: versioning, tags, ACLs, policies, encryption, logging, replication, website, ownership controls, request payment, and public access block.
- [ ] Intentional exclusions are documented, such as S3 `inventory_configurations` and `multipart_uploads`.
- [ ] `Vec<u8>` fields serialise as stable lowercase hex or another documented stable encoding.
- [ ] `DateTime<Utc>` fields serialise as RFC 3339 strings.
- [ ] Enums without serde derives map through stable strings.
- [ ] Tuple-keyed maps flatten into `Vec<...View>` and are rebuilt on restore.
- [ ] Opaque `serde_json::Value` fields are used only for wire-shaped blobs that need no semantic access, such as deeply nested discriminated unions. Round-trip them with `serde_json::from_value(v).ok()` when emitting responses. Replace them with typed views when validation, filtering, or query behaviour needs to inspect the contents.

## 5. Stubs and Deferred Behaviour

An intentional stub accepts a request, returns a well-formed minimal response, and performs no meaningful state mutation because the real behaviour is outside the current mock surface. The coverage report excludes stub handlers from implemented-operation counts.

Approved categories:

| Tag | Meaning |
|---|---|
| `no-state` | The feature area is not tracked in state. All old `STUB[no-state]` annotations have been eliminated; new service work should usually start state-backed. |
| `no-engine` | The operation requires an execution or evaluation engine that is out of scope for a mock. |
| `no-telemetry` | The response depends on real infrastructure telemetry or access history. |
| `org-integration` | The operation requires AWS Organizations or IAM Identity Center cross-account state. |
| `s3-integration` | The operation requires real S3 bucket metadata or object content. |
| `no-auth` | The mock has no authentication layer for this operation. |
| `delegation-api` | The operation requires real delegated-administrator trust. |

Gate checks:

- [ ] Every intentional stub carries `// STUB[category]: reason` immediately before the function.
- [ ] Promoted stubs have the `STUB[...]` comment removed.
- [ ] No unmarked default or empty handler responses remain:
  ```bash
  rg 'Default::default\(\)' crates/winterbaume-*/src/handlers.rs | \
    grep -v 'STUB\[' | grep -v '_to_model'
  ```
- [ ] No placeholder pagination token such as `"next-token-placeholder"` remains.
- [ ] Responses use empty collections (`Some(vec![])`) instead of `None` where AWS returns an empty list.

## 6. Smithy Codegen and Protocol Boundaries

`model.rs` and `wire.rs` are generated by `tools/smithy-codegen`. Never hand-edit generated files as a durable fix. Patch the generator, regenerate, and rerun affected tests.

Gate checks:

- [ ] Generated files bearing the "Do not edit manually" header match generator output.
- [ ] `./.agents/bin/cargo.sh run -p smithy-codegen -- list-services` matches service crates with generated files.
- [ ] Model directory mappings in `tools/smithy-codegen/src/discover.rs` cover all generated service crates.
- [ ] Generator changes that affect emitted patterns across crates are regenerated across every affected crate.
- [ ] Generated `model.rs` files carry required lint allowances such as `non_camel_case_types` and `clippy::upper_case_acronyms`.
- [ ] Generated split-path `wire.rs` files carry `#![allow(dead_code)]` where unused generated helpers are expected.
- [ ] Generated `wire.rs` files have no stale `TODO` placeholders such as `httpResponseCode` comments.
- [ ] XML list-wrapper fields use the actual XML member name as the Rust field name, not only `#[serde(rename)]`.
- [ ] `ec2Query` services use EC2 response framing: no inner `<OpResult>` wrapper and top-level `requestId`.
- [ ] Services with multiple Smithy protocols are checked against SDK and Terraform traffic; a secondary protocol used by real clients must be supported.
- [ ] `awsQueryCompatible` is not treated as proof that a JSON service accepts awsQuery request bodies; confirm whether real clients send secondary-protocol traffic.
- [ ] `rpc-v2-cbor` timestamp output preserves CBOR Tag 1 semantics through generated scalar and list helpers.
- [ ] `httpResponseCode` extraction emits the correct generated Rust field name per output shape.
- [ ] Output-only `@required` members are generated as `Option<T>` with `skip_serializing_if`.
- [ ] Handler parameter lookups use exact AWS API names, not plausible aliases such as `StackNameOrId`.
- [ ] Hand-written JSON or XML success responses are absent where generated `wire::serialize_*_response()` helpers exist:
  ```bash
  rg 'json!\(\{' crates/winterbaume-*/src/handlers.rs
  git grep -n '<?xml' -- 'crates/winterbaume-*/src/handlers.rs'
  ```
- [ ] Hand-written XML in handlers is limited to error envelopes, dynamic-root helpers, or payload-sensitive success responses where a generated serialiser cannot produce the required body shape.
- [ ] Request parsing uses generated `wire::deserialize_*_request` helpers where available.
- [ ] Manual extraction of model-described request fields is classified as generator debt, adoption debt, dispatch-only parsing, validation-only raw-body inspection, or a service-specific exception.
- [ ] JSON-protocol services (`restJson1`, `awsJson1.0`, `awsJson1.1`) use generated request deserialisers for Smithy-described inputs; residual `body.get(...)` reads need a documented hybrid reason or migration plan.
- [ ] Generated input fields are checked per exact shape before applying `.is_empty()`, `.as_deref()`, `Some(...)`, or bare-value patterns; sibling operations may differ in optionality for the same concept.
- [ ] REST JSON handlers treat an empty body as `{}` when empty-body GET requests are valid.
- [ ] Path identifiers that can be either bare IDs or ARNs use one helper that strips the trailing slash component for ARN inputs and leaves bare IDs unchanged.

## 7. Pluggable Backends and Blob Stores

Storage and execution backends are service-owned extension points. Heavy engines live outside the default service crate.

Gate checks:

- [ ] Backend traits live in the service crate and are object-safe, usually returning `Pin<Box<dyn Future<...> + Send>>`.
- [ ] `new()` uses the default in-memory adapter.
- [ ] `with_backend(...)` or `with_query_backend(...)` is the injection point for alternative implementations.
- [ ] Snapshot, restore, and merge delegate through the backend when the backend owns durable state.
- [ ] Redis, DuckDB, and other heavy external engines live in separate crates.
- [ ] Heavy engine crates are excluded from workspace `default-members`.
- [ ] Higher-level request languages stay above backend traits when they can decompose into backend primitives. DynamoDB PartiQL execution through `execute_partiql_via_backend(...)` is the reference.
- [ ] Blob-backed services keep large payloads in `BlobStore`, not heap-backed state structs.
- [ ] Blob stores are scoped by account and region through `BlobStoreMap` or an equivalent child namespace.
- [ ] Views reference blobs by key and metadata, not by embedding raw payload content.
- [ ] Blob read failures in handlers surface as 500 `InternalError`, not silently empty bodies.
- [ ] Composite blobs use `put_composite`.
- [ ] `BlobBackedService` implements `blob_keys()`, `blob_store()`, `export_blobs()`, and `import_blobs()`.

## 8. Pure Evaluator and Validator Crates

Semantic engines belong in helper crates when the logic is reusable, dense, or independently testable.

Gate checks:

- [ ] `winterbaume-iam-rule-eval` covers wildcard matching, explicit-deny precedence, implicit-deny behaviour, and `NotAction` / `NotResource` inversion.
- [ ] `winterbaume-sfn-asl-eval` validates modern `ItemProcessor`, legacy `Iterator`, missing `Type`, and illegal `End` plus `Next` combinations.
- [ ] `winterbaume-wafv2-webacl-rule-parser` and `winterbaume-wafv2-wcu-calculator` account for text-transformation surcharges, `FieldToMatch` surcharges, and custom-key rate-based surcharges.
- [ ] `winterbaume-wafv2` `CheckCapacity` uses the real parser and calculator path, not a fixed capacity response.
- [ ] Bedrock flow handlers invoke parser and validator crates rather than embedding graph checks in handlers.

## 9. SDK and Scenario Tests

Gate checks:

- [ ] Every service crate has `tests/integration_test.rs`.
- [ ] Tests use real `aws-sdk-rust` clients against the in-process mock, not raw HTTP.
- [ ] Read and describe tests create prerequisite resources first and use returned ARNs or identifiers instead of plausible hard-coded values.
- [ ] Service errors are asserted with debug formatting (`format!("{:?}", err)`), not `Display`, because SDK `Display` often collapses to `service error`.
- [ ] Tests account for SDK getter requiredness: required members return `&T`; optional members return `Option<&T>`, and sibling outputs in one service may mix both shapes.
- [ ] SDK enum variants are taken from compiler output or generated SDK docs, not inferred from Smithy JSON acronym spelling.
- [ ] AWS-managed-corpus services with no public `Create*` API seed state through `service.restore(...)` or `service.merge(...)` with a `*StateView`.
- [ ] `tests/scenario_test.rs` exists for stateful services whose resources combine in workflows.
- [ ] Scenario tests use use-case names and top-of-test `Scenario:` comments.
- [ ] Scenario tests chain three or more operations and assert business outcomes, not only per-API return shapes.
- [ ] Stateful-service scenario tests cover every non-`N/A` row in the service dossier's `## Cross-call invariants` section.
- [ ] Scenario tests that depend on unimplemented features may use `#[ignore]` only with a `TODO.md` cross-link.
- [ ] `FIX(terraform-e2e)` markers have matching fast SDK integration tests:
  ```bash
  rg 'FIX\(terraform-e2e\)' crates/winterbaume-*/src/ -l
  ```
- [ ] Integration test coverage is at or above 80% of implemented operations per crate, verified through `/api-coverage`.
- [ ] Tests cover happy paths, error behaviour, idempotency, and edge cases.
- [ ] Service-specific tests pass. Workspace-wide `test --workspace` is reserved for explicit release or quality-audit work.

## 10. Terraform E2E and Converters

Skip Terraform converter and E2E work when the upstream Terraform AWS provider has no `aws_<service>_*` resource for the AWS service. Recent examples include AIOps, Amplify Backend, AppConfig Data, AWS Artifact, Auto Scaling Plans, and S3 Files.

Gate checks:

- [ ] Services with Terraform provider resources have E2E modules under `tests/e2e/terraform/`.
- [ ] Terraform resource-to-crate mapping tables in `.agents/skills/api-coverage/scripts/generate_coverage.py` are current.
- [ ] Converters inject and extract through `StatefulService` views and `merge()`, not by mutating private runtime state.
- [ ] `do_inject()` and `do_extract()` preserve account and region symmetry. Default-scope extraction limitations are explicit and tested.
- [ ] Multi-scope extraction uses `ConversionContext`, scoped `ExtractedResource` metadata, `TerraformInjector::register_with_scopes()`, and `TerraformInjector::extract_all()`.
- [ ] Scope providers are read-only, deterministic, and do not create empty backend buckets.
- [ ] New converter structs are registered in the injector build path used by server and E2E harnesses.
- [ ] Converter tests cover JSON-shape mismatch: Terraform provider-shaped JSON must be translated into AWS-shaped JSON before entering a `*StateView`.
- [ ] Converter tests cover identifier id/name swaps: injected and extracted IDs must preserve provider names and AWS identifiers distinctly.
- [ ] The conversion error variant is `ConversionError::MissingAttribute`.
- [ ] Nested Terraform blocks are verified with apply plus no-drift plan, not only successful apply or converter unit tests.
- [ ] Services requiring isolated servers, such as Kinesis, ECS, and EC2, do not use `batch_apply`.
- [ ] New E2E modules are wired through crate dev-dependencies, `test_services()` registration, and provider endpoint override.
- [ ] `write_provider_tf()` pins the AWS provider version so provider upgrades are deliberate.
- [ ] S3 Terraform examples and smoke tests use `s3_use_path_style = true` unless local virtual-hosted-style routing is explicitly supported.

Provider compatibility checks:

- [ ] Nested structs that AWS always returns are present with zero-valued defaults rather than `None`.
- [ ] Describe and List operations honour request filters for names, ARNs, IDs, and other provider read paths.
- [ ] Identifier formats match provider expectations.
- [ ] Redshift clusters populate `ClusterParameterGroups`, `ClusterNodes`, `MultiAZ`, and title-case `ClusterAvailabilityStatus`.
- [ ] Multi-protocol adapters preserve protocol-specific timestamp sentinels.
- [ ] ELBv2 listener-rule conditions populate both legacy `Values` fields and matching `*Config.Values` blocks.

## 11. Coverage Reports and Generated Documentation

Gate checks:

- [ ] `.agents/docs/API_COVERAGE.md` is regenerated with `/api-coverage` after service operation or test-coverage changes.
- [ ] `README.md` and per-crate READMEs are regenerated with `/update-readme` after coverage changes.
- [ ] Manually-authored README sections live outside generated `<!-- BEGIN AUTO ... -->` / `<!-- END AUTO -->` blocks.
- [ ] Dossier-derived README sections are regenerated from `.agents/docs/services/<model-slug>.md`, not hand-copied into crate READMEs.
- [ ] Source dossier-section counts match generated README-section counts for transcribed sections such as `## Current Network Resource Stub Semantics`.
- [ ] Coverage report false negatives from stale heuristics, stale caches, or resource-to-crate mappings are fixed in the reporting scripts rather than hidden with misleading wrapper tests.
- [ ] `README.md`, service dossiers, and coverage tables agree on crate names, Smithy model slugs, protocol names, implemented operations, stubs, and Terraform support.

## 12. Server, Examples, and Workspace Wiring

Gate checks:

- [ ] `winterbaume-server` compiles and registers all concrete service implementations.
- [ ] `winterbaume-server` emits structured access logs with request scope fields such as `remote_addr`, `method`, `path`, `status`, and `elapsed_ms`.
- [ ] Examples live under root `examples/`, not in a separate workspace crate.
- [ ] `./.agents/bin/cargo.sh build --examples -p winterbaume` compiles examples when examples are touched.
- [ ] Crate names in examples, server wiring, docs, and import paths are consistent with any rename.
- [ ] Root-package `[dev-dependencies]` cover direct handler or protocol helper usage in examples and tests.
- [ ] `Cargo.lock` is committed and current.
- [ ] No circular dependencies exist between service crates. Derived services depend on upstream backends, not the reverse.
- [ ] Optional features such as `smithy-mocks` compile when enabled.
- [ ] `cargo doc --workspace --no-deps` produces no errors for release-readiness work.

## 13. CI and Release Workflow Security

GitHub Actions, cargo-dist, cargo-release, and crates.io publication tooling are part of the release supply chain.

Gate checks:

- [ ] Every `uses:` reference in `.github/workflows/` is pinned to a commit SHA with the version tag in an inline comment.
- [ ] SHA and version-tag comments move together during action bumps.
- [ ] `release.yml` carries a current `CARGO_DIST_INSTALLER_SHA256` matching `CARGO_DIST_INSTALLER_URL`, and the installer is verified before execution.
- [ ] Workflow `permissions:` are scoped at the top level to the minimum needed.
- [ ] Workflows with `contents: write` carry an accepted-risk comment naming branch protection or an equivalent mitigation.
- [ ] `workflow_run` consumers carry inline comments naming the producing workflow, artefact blast radius, and why artefact contents are never executed.
- [ ] Third-party token consumers get the smallest possible permission scope with inline justification.
- [ ] `triage-bug.yml` runs the prompt-injection guardrail before AI triage. The hostile issue body must not reach the triage assistant.
- [ ] The guardrail exposes `safe`, `abuse`, and `inconclusive`; labels `possible-abuse` only on `abuse`; and downstream steps gate on `verdict == 'safe'`.
- [ ] The `Install Rust non-interactively` step in `release.yml` is left as the documented rust-lang.org installer exception unless upstream guidance changes.
- [ ] First public crates.io publication uses either a crates.io `publish_new` quota raise or `tools/release-batch/` with chunks no larger than the current quota. Do not rely on cargo-release hooks to sleep around this limit; crates.io rejects oversized new-crate batches before per-crate hooks run.
- [ ] Release-batch plan output is reviewed before `--execute`: publishable crates are topologically ordered, helper crates such as `winterbaume-core` appear before services, and umbrella / server / Terraform meta-crates appear last.
- [ ] When agents invoke `tools/release-batch/`, pass `--cargo ./.agents/bin/cargo.sh` or set `WB_CARGO=./.agents/bin/cargo.sh` so release subprocesses use the same wrapper environment as other cargo work.
- [ ] Root `winterbaume` package `include` entries are anchored ( for example `/Cargo.toml`, `/README.md`, `/src/**/*.rs` ) so cargo packaging does not include nested docs, vendored Smithy models, examples, or agent memory.
- [ ] Root package `autoexamples = false` remains set while `examples/` is intentionally excluded from the package whitelist.
- [ ] Publishable crates carry `description`, `license`, `repository`, and `readme` metadata. Per-crate metadata exceptions are deliberate and documented.

## 14. Known Non-Blocking Issues

These do not block publication by themselves, but they should inform design and tests:

- `StatefulService::snapshot()` is non-fallible, so blob read failures can only be logged.
- IAM simulation uses `winterbaume-iam-rule-eval`, but richer condition handling, permissions boundaries, and principal-context fidelity remain incomplete.
- Persistence across server restarts is still a design follow-up for services such as SSM Parameter Store and Secrets Manager.
