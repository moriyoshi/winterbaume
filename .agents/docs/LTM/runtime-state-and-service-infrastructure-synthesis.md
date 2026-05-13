# Runtime State and Service Infrastructure Synthesis

## Summary

Winterbaume's durable service architecture is the combination of the service-hardening workflow and the shared runtime contract for exporting, restoring, merging, and persisting service state. Future service work should treat `handlers.rs`, generated wire support, `views.rs`, `StatefulService`, backend ownership, blob persistence, and Terraform converter boundaries as one system rather than as separate concerns. The key architectural point is that external inject or extract layers are only trustworthy when the underlying `StateView` contract is complete.

## Included Documents

| Document | Focus |
|----------|-------|
| [service-implementation-and-validation-synthesis.md](./service-implementation-and-validation-synthesis.md) | High-level workflow for service growth, Smithy-backed wire support, and layered validation |
| [stateful-service-and-blob-store.md](./stateful-service-and-blob-store.md) | Async state-view contract, VFS and BlobStore architecture, snapshot-fidelity guidance, and derived-service rules |
| [terraform-resource-converters.md](./terraform-resource-converters.md) | Converter contract, inject or extract semantics, and `StateView`-gated Terraform support |
| [runtime-account-identity-configuration.md](./runtime-account-identity-configuration.md) | Runtime account ID configuration, handler migration off `DEFAULT_ACCOUNT_ID`, and process-wide override limitations |

## Stable Knowledge

- Service work in Winterbaume is durable only when the wire layer, state layer, and validation layer evolve together.
- `StatefulService` is a repo-wide async contract. `snapshot`, `restore`, `merge`, and `notify_state_changed` are part of the normal service architecture, not optional extras.
- Typed `views.rs` modules are the stable serde-facing contract for durable state. Runtime-only fields belong in `state.rs`, not in serialized views.
- The `(account_id, region)` passed to `snapshot`, `restore`, and `merge` is the durable scope selector. Top-level `*StateView` structs must not carry their own account or region selectors.
- Runtime handlers should resolve the default account through `winterbaume_core::default_account_id()`, not by directly reading `DEFAULT_ACCOUNT_ID`. `winterbaume-server` installs the configured account ID at startup from CLI/env/config precedence.
- The runtime account override is process-wide and first-writer-wins through `OnceLock`. It matches the existing single-global-account architecture; it is not a multi-tenant per-`MockAws` design.
- `merge()` is additive. Missing entries in an incoming view must not delete existing resources.
- New view fields should prefer `#[serde(default)]` so older snapshots remain readable after schema expansion.
- If external callers construct `*StateView` values directly, derive `Default` and prefer `..Default::default()` at call sites so view growth does not break Terraform converters or other injectors.
- Terraform converters depend on the same contract. If a resource cannot be represented faithfully in `StateView`, converter support is incomplete no matter how easy the handler-side mutation looks.
- Terraform extraction is now orchestrated through `ConversionContext`, scoped `ExtractedResource` metadata, and `TerraformInjector::extract_all()` for services that opt into scope providers.
- Nested Terraform blocks do not change that rule. If `logging_configuration`, `encryption_configuration`, `vpc_config`, `snapshot_copy`, or similar shapes are missing from `views.rs`, top-level converter coverage can still look healthy while Terraform plans drift.
- Blob-backed payloads belong in `BlobStore` over `Vfs`, not in large in-memory structs. Composite blobs and versioned writes are first-class storage patterns.
- Blob-backed payloads must use the same account/region scope as their service state. `BlobStoreMap` provides scoped child stores for S3, EBS, and Glacier over one shared VFS.
- Backend-owned state is now the stable rule for services with pluggable persistence. A service should not keep a hidden in-memory shadow copy once `with_backend(...)` exists.
- Derived AWS surfaces should not invent their own durable resource store when the real lifecycle belongs to another service. DynamoDB Streams is the reference case: metadata comes from DynamoDB tables, iterator bookkeeping stays local, and payload capture comes from upstream write paths.
- Not every `serde_json::Value` field in a view is a design defect. Opaque wire-shaped blobs are acceptable unless the state layer needs semantic access, as with DynamoDB `AttributeValue`.
- The current state-view contract still has one known design limitation: `snapshot()` is non-fallible, so blob read failures can be logged but not returned to the caller.
- Async lock migrations are architectural changes, not mechanical renames. `tokio::sync::RwLock` guard acquisition is `await`-based, so leftover `.unwrap()` patterns from `std::sync::RwLock` are correctness bugs.

## Operational Guidance

When implementing or extending a stateful service:

1. Confirm the protocol family and existing crate shape.
2. Implement or adjust `handlers.rs` and `state.rs`.
3. Regenerate `model.rs` and `wire.rs` if the service depends on Smithy-backed types or serialisers.
4. Define or extend `views.rs` with an explicit `*StateView` and conversion helpers.
5. Implement `StatefulService` so snapshot, restore, and merge semantics are explicit and additive.
6. If the service supports backend injection, make the backend authoritative for snapshot, restore, and merge as well.
7. Add `notify_state_changed(...).await` only after successful mutations and only after all write guards have been dropped.
8. For large payloads, route object or archive bytes through `BlobStore` instead of embedding them in heap-backed state.
9. For blob-backed services, resolve the blob store from the same account and region used for service state.
10. If Terraform converter support exists or is planned, verify that the needed view types and merge paths exist before implementing inject or extract logic.
11. For nested Terraform blocks, extend the durable service model first, then prove the block survives inject -> snapshot -> extract without drift.
12. Validate the service through the strongest available layers: SDK tests first, then moto parity, docs-driven tests, Terraform converter tests, and Terraform E2E where provider behaviour matters.

When evolving existing state views:

- Keep backward compatibility explicit with `#[serde(default)]` on newly added fields.
- When Terraform converters or tests build view structs manually, add `Default` on the view type and switch callers to `..Default::default()` rather than naming every field.
- Rebuild transient indexes, counters, caches, and in-flight state during restore rather than serializing them directly.
- Preserve existing resources during merge unless the contract explicitly requires replacement.
- Treat lossy snapshots as correctness bugs even if runtime behaviour still passes smoke tests.

## Files

- `crates/winterbaume-core/src/views.rs`: `StatefulService`, `StateChangeNotifier`, and `StateViewError`
- `crates/winterbaume-core/src/vfs.rs`: async `Vfs`, `MemVfs`, `FsVfs`, prefix semantics, and path validation
- `crates/winterbaume-core/src/blob_store.rs`: namespaced blob storage, versioning, composite blob support, and `BlobStoreMap` scoped child stores
- `crates/winterbaume-core/src/mock_aws.rs`: shared VFS injection used by blob-backed services
- `crates/winterbaume-core/src/state.rs`: `DEFAULT_ACCOUNT_ID`, `default_account_id()`, and `set_default_account_id()`
- `crates/winterbaume-server/src/main.rs`: startup installation of the configured runtime account ID
- `crates/winterbaume-*/src/handlers.rs`: protocol routing, response shaping, and mutation-triggered notifications
- `crates/winterbaume-*/src/state.rs`: runtime state and validation rules
- `crates/winterbaume-*/src/views.rs`: durable state-view contract and restore or merge conversions
- `crates/winterbaume-*/src/backend.rs`: authoritative backend traits for services with pluggable persistence
- `crates/winterbaume-terraform/src/converters/*.rs`: external inject or extract layer that depends on faithful service views
- `crates/winterbaume-dynamodbstreams/src/handlers.rs` and `state.rs`: derived-service iterator bookkeeping over upstream DynamoDB state

## Tests

Representative regression layers and commands:

```bash
cargo check --workspace
cargo test -p winterbaume-core -- fs_vfs
cargo test -p winterbaume-core blob_store
cargo test -p winterbaume-s3
cargo test -p winterbaume-cognitoidp
cargo test -p winterbaume-dynamodb
cargo test -p winterbaume-dynamodbstreams
cargo test --test e2e_terraform -- --ignored
```

Choose the smallest suite that exercises the changed contract:

- core unit tests for VFS or BlobStore changes
- per-service integration tests for view, backend, or handler changes
- Terraform converter integration tests when `StateView` coverage or merge semantics change
- moto parity tests for behavioural regressions
- Terraform E2E when provider-facing behaviour, sub-resource reads, or nested-block round-tripping changed

## Pitfalls

- Never call `notify_state_changed(...).await` while a write lock is still alive.
- Never hold `RwLock` guards across awaits that read or write blob-backed data.
- Do not serialize transient runtime state into `StateView` just because it is easy.
- Do not bypass incomplete views with converter-local state mutation. Extend `views.rs` first.
- Do not serialize scope selectors into top-level state views. Use the `StatefulService` method arguments.
- Do not introduce new handler-side uses of `DEFAULT_ACCOUNT_ID`; use `default_account_id()` for runtime account identity.
- Do not assume `MockAws::builder().account_id(...)` changes handler runtime account ID. It still affects only the builder/getter path unless a future change installs the process-wide override.
- Do not interpolate `default_account_id()` directly inside Rust format-string braces. Move it to a positional or named format argument.
- Do not let blob-store namespaces ignore account and region when the corresponding service state is scoped.
- Do not treat passing SDK smoke tests as proof that snapshot fidelity or Terraform compatibility is correct.
- Do not treat `terraform apply` success as enough when the contract really depends on apply followed by a no-drift plan.
- Do not hand-edit generated `model.rs` or `wire.rs` as a lasting fix.
- Do not leave `std::sync::RwLock` idioms such as `.unwrap()` in place after migrating a service onto `tokio::sync::RwLock`.
- Do not assume default-region extraction is enough for resources injected into non-default scopes; use the injector's scope-provider path where the service supports it.
- `FsVfs::list` uses string-prefix semantics, not directory-boundary semantics.
- `snapshot()` still cannot return failures directly, so blob read errors must be surfaced through logs and follow-up design work.
