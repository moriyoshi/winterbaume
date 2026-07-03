---
name: add-service
description: Implement or enhance an AWS service in winterbaume — creates a new crate from a stub or adds missing operations to an existing crate. Targets moto API coverage parity.
argument-hint: <service-name> [operations...]
user_invocable: true
---

# Implement / Enhance AWS Service

Create a new winterbaume service crate from a stub, or add missing operations to an existing crate. Both paths converge on the same implementation workflow.

## Arguments

- `$0` — Service name (e.g., `guardduty`, `sqs`, `bedrock`, `transfer`)
- `$1...` — (optional) Specific operations to implement. If omitted:
  - **New crate**: choose 3-5 core CRUD operations
  - **Existing crate**: implement ALL operations marked `W[ ] M[x]` in `API_COVERAGE.md`

---

## Step 0: Resolve service context and determine whether crate exists

### 0a. Resolve and read the service dossier

Before choosing operations or editing code, map the requested name through `.agents/docs/services/INDEX.md` and read the matching `.agents/docs/services/<model-slug>.md`. The dossier path is keyed by the **model slug**, not necessarily by the Winterbaume crate suffix or AWS SDK slug.

Use this document as the service dossier:

- `AWS model slug`, `AWS SDK for Rust slug`, and protocol metadata help resolve crate/model/SDK naming mismatches.
- Operation and resource tables identify the real Smithy surface, required inputs, idempotency traits, pagination, and error shapes.
- `Official AWS Documentation Research`, when present, captures AWS-documented semantics worth preserving in handlers and tests.
- `Winterbaume LTM Notes`, when present, records service-specific implementation pitfalls, integration boundaries, and parity guidance from previous work.
- `Research Checklist for Parity Work` lists behaviours to verify before claiming service parity.

If no matching dossier exists, stop this workflow and invoke `/service-dossier {service}` first. Resume only after that skill has created the dossier and updated `.agents/docs/services/INDEX.md`. If the match is ambiguous, do not guess between similarly named services; use the index title, `AWS model slug`, `AWS SDK for Rust slug`, Smithy service title, and endpoint prefix to disambiguate, and invoke `/service-dossier {service}` when the index or dossier needs to be created or corrected.

### 0b. Determine whether crate exists

```bash
ls crates/winterbaume-{service}/src/handlers.rs 2>/dev/null && echo "EXISTS" || echo "NEW"
```

- **EXISTS** → skip to Step 2 (read existing code, then add operations)
- **NEW** → proceed with Step 1 (determine protocol), then Step 3 (create crate)

> **EC2 is special.** `winterbaume-ec2` is split into two crates (`winterbaume-ec2` for hand-written code and `winterbaume-ec2-generated` for `model.rs` / `wire.rs`), and operations are gated behind Cargo features. Before touching it, read [`.agents/docs/LTM/ec2-crate-split-and-feature-gating.md`](../../docs/LTM/ec2-crate-split-and-feature-gating.md) — the regeneration command, the feature taxonomy, and the per-operation `#[cfg]` requirements live there.

---

## Step 1: Determine protocol and API shape from the Smithy model

### 1a. Find the model file

```bash
ls vendor/api-models-aws/models/ | grep -i {service}
```

Common mismatches: `config` -> `config-service`, `cognitoidp` -> `cognito-identity-provider`, `elbv2` -> `elastic-load-balancing-v2`, `logs` -> `cloudwatch-logs`, `events` -> `eventbridge`.

Prefer the `AWS model slug` from `.agents/docs/services/<model-slug>.md`; it is the local canonical mapping from service research to vendored Smithy model directory. Prefer `AWS SDK for Rust slug` from the same document when naming the `aws-sdk-{sdkname}` dependency and SDK imports.

The model JSON is at `vendor/api-models-aws/models/{model-dir}/service/{version}/{name}.json`.

### 1b. Extract protocol

Read the model and find the service shape (`"type": "service"`). Its `traits` contains the protocol:

| Trait key | Protocol | Handler pattern |
|-----------|----------|-----------------|
| `aws.protocols#restJson1` | REST-JSON | HTTP method + path routing, `x-amzn-errortype` header |
| `aws.protocols#awsJson1_0` | awsJson1.0 | `X-Amz-Target` header dispatch, `MockResponse::json()` |
| `aws.protocols#awsJson1_1` | awsJson1.1 | `X-Amz-Target` header dispatch, `MockResponse::json()` |
| `aws.protocols#awsQuery` | awsQuery | `Action` form field dispatch, XML responses |
| `aws.protocols#restXml` | REST-XML | HTTP method + path routing, XML bodies |

### 1c. Use smithy-codegen for operation discovery

```bash
cargo run -p smithy-codegen -- ops {model-name}
```

Lists all operations with `[x]` (implemented) and `[ ]` (missing) markers.

### 1d. Determine which operations to implement

If the user specified operations, use those. Otherwise:

- **New crate**: select 3-5 core operations: Create, Get/Describe, Delete, List, (Update)
- **Existing crate**: read `.agents/docs/API_COVERAGE.md`, find the `winterbaume-{service}` section, and extract all `W[ ] M[x]` entries (ops moto has that winterbaume doesn't)

Cross-check the candidate list against `.agents/docs/services/<model-slug>.md` before implementation. Prefer root-resource lifecycle operations identified by the service resource table, include documented tag operations when they exist, and respect service-specific `Winterbaume LTM Notes` or parity-checklist warnings about derived state, external engines, cross-service integrations, idempotency, pagination, or validation semantics.

If the gap is large (>15 ops), prioritise:
1. CRUD operations for core resources
2. Tag operations (TagResource, UntagResource, ListTagsForResource)
3. Batch operations
4. Configuration/policy operations

### 1e. Extract operation details from the Smithy model

For each operation to implement, read its shape in the model JSON:
- **Input shape members** — field names, types, required markers, `jsonName` overrides
- **Output shape members** — response fields, wire names
- **HTTP routing** (REST protocols) — method, URI pattern from `smithy.api#http` trait
- **Error shapes** — error types the operation can return

Use the service research document as a companion index while reading the model. The generated operation table can quickly surface idempotency tokens, pagination traits, required fields, resource associations, and known error shapes, but the Smithy model remains the source of truth for exact generated Rust fields.

### 1f. Cross-reference with moto (optional)

Check [moto's source](https://github.com/getmoto/moto/tree/master/moto) for implementation hints if needed. Always prefer the Smithy model as source of truth.

---

## Step 2: Read the existing crate (existing crates only)

```
crates/winterbaume-{service}/src/handlers.rs   — dispatch pattern, protocol, existing operations
crates/winterbaume-{service}/src/state.rs      — state structure, existing CRUD methods
crates/winterbaume-{service}/src/types.rs      — domain types
crates/winterbaume-{service}/src/views.rs      — state view types and StatefulService contract ( when the service exposes state views )
crates/winterbaume-{service}/src/wire.rs       — available serialiser functions (auto-generated)
crates/winterbaume-{service}/src/model.rs      — available model types (auto-generated)
crates/winterbaume-{service}/src/lib.rs        — public exports, including any StateView type
crates/winterbaume-{service}/tests/integration_test.rs — existing test patterns, make_client helper
```

**Critical**: Match the existing code style exactly. Note the protocol, error response pattern, dispatch structure, state method signatures, and how wire types are constructed.

Also re-check `.agents/docs/services/<model-slug>.md` after reading the crate. If existing behaviour contradicts documented AWS semantics or the `Winterbaume LTM Notes`, decide whether the contradiction is intentional mock scope, an implementation bug, or deferred parity work before extending the same pattern.

### Check wire.rs for serialisers

```bash
grep 'pub fn serialize' crates/winterbaume-{service}/src/wire.rs | grep -i '{operation}'
```

If serialisers are missing, regenerate:

```bash
cargo run -p smithy-codegen -- gen-serializers {model-name} \
  --output crates/winterbaume-{service}/src/wire.rs \
  --model-output crates/winterbaume-{service}/src/model.rs
```

**After regeneration**, rebuild to check for errors. New model types may have additional fields — add `..Default::default()` to existing wire struct initializers.

---

## Step 3: Create the crate (new crates only)

Create `crates/winterbaume-{name}/` with these files:

### `Cargo.toml`

```toml
[package]
name = "winterbaume-{name}"
version = "0.1.0"
edition.workspace = true
license.workspace = true
description = "{ServiceName} service implementation for winterbaume"

[dependencies]
winterbaume-core = { workspace = true }
tokio = { workspace = true }
bytes = { workspace = true }
http = { workspace = true }
uuid = { workspace = true }
tracing = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = { workspace = true }
# Add chrono = { workspace = true } if you need timestamps

[dev-dependencies]
aws-config = { workspace = true }
aws-credential-types = { workspace = true, features = ["test-util"] }
aws-sdk-{sdkname} = { workspace = true }
aws-smithy-runtime-api = { workspace = true }
aws-smithy-types = { workspace = true }
tokio = { workspace = true }
```

Only include dependencies you actually use.

> **Why a literal `version = "0.1.0"` and not `version.workspace = true`?** The root `[workspace.package]` in this repository only inherits `edition`, `license`, `repository`, and `keywords`. There is no `version` key in `[workspace.package]`, so `version.workspace = true` fails to resolve. Each service crate carries its own version, managed independently by `cargo-release`. If `[workspace.package]` ever gains a `version` field, revisit this template.

### `src/lib.rs`

```rust
pub mod handlers;
pub(crate) mod model;
pub mod state;
pub mod types;
pub mod views;
pub(crate) mod wire;

pub use handlers::{ServiceName}Service;
pub use state::{ServiceName}State;
pub use views::{ServiceName}StateView;
```

### `src/types.rs`

Define domain types with `#[derive(Debug, Clone)]`. Keep minimal.

### `src/state.rs`

```rust
use crate::types::*;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct {ServiceName}State {
    pub items: HashMap<String, YourType>,
}

/// Domain-specific error enum. Contains no HTTP status codes or AWS error type strings —
/// those are mapped in the handler's error-shaping function.
#[derive(Debug, Error)]
pub enum {ServiceName}Error {
    #[error("{resource_type} {name} does not exist.")]
    NotFound { resource_type: &'static str, name: String },
    #[error("{resource_type} {name} already exists.")]
    AlreadyExists { resource_type: &'static str, name: String },
    /// Catch-all for ad-hoc validation errors; use a named variant instead when
    /// the same condition occurs 3+ times.
    #[error("{message}")]
    Validation { message: String },
}

impl {ServiceName}State {
    // CRUD methods returning Result<T, {ServiceName}Error>
}
```

**Design rules for the error enum:**

- **No HTTP knowledge in state.rs.** Never put status codes or AWS error type strings (`"NotFoundException"`, `404`, …) inside the enum definition or any method on it. HTTP mapping belongs exclusively in the handler.
- **Group by semantics.** Use separate variants for distinct resource types or error conditions. Use a parameterised variant (e.g., `NotFound { resource_type, name }`) when the message template is identical for many resource types.
- **Carry minimum data.** Each variant should hold only the fields needed to produce the error message via the `#[error("...")]` attribute. Prefer named fields over positional ones for clarity.
- **Validation catch-all.** Include a `Validation { message: String }` variant for ad-hoc validation errors that don't warrant their own variant. Use it sparingly.

### `src/views.rs`

If the service maintains durable backend state, add a typed serde-compatible view and implement `winterbaume_core::StatefulService` for the service. This is the contract used by state injection / extraction flows.

The contract is:

- `snapshot(account_id, region)` returns a typed per-account / per-region view of the current state
- `restore(account_id, region, view)` replaces the existing state entirely from that view
- `merge(account_id, region, view)` additively merges the view into the existing state and does not remove resources that are already present

Implementation rules:

- Keep the view type separate from the internal state type. The view is the serialization contract; it should not expose implementation-only details.
- Derive `Debug`, `Clone`, `Serialize`, and `Deserialize` on view structs.
- Model durable resource state only. Transient runtime data such as in-flight messages, caches, deleted handles, locks, or permission scratch state should be omitted or reinitialized when restoring from a view.
- Prefer a top-level `{ServiceName}StateView` that represents one account / region, usually keyed by stable resource identifiers.
- Use `#[serde(default)]` on collection fields so partial views are easy to deserialize and merge.
- Convert between internal state and view types with `impl From<&State> for StateView` and `impl From<StateView> for State`.
- If the view is inconsistent or cannot be converted safely, return `StateViewError::Invalid(...)`.

Typical structure:

```rust
use crate::handlers::{ServiceName}Service;
use crate::state::{ResourceState, {ServiceName}State};
use crate::types::Resource;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {ServiceName}StateView {
    #[serde(default)]
    pub resources: HashMap<String, ResourceStateView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceStateView {
    pub name: String,
    // durable fields only
}

impl From<&{ServiceName}State> for {ServiceName}StateView {
    fn from(state: &{ServiceName}State) -> Self {
        Self {
            resources: state
                .resources
                .iter()
                .map(|(k, v)| (k.clone(), ResourceStateView::from(v)))
                .collect(),
        }
    }
}

impl From<&ResourceState> for ResourceStateView {
    fn from(state: &ResourceState) -> Self {
        Self {
            name: state.resource.name.clone(),
        }
    }
}

impl From<{ServiceName}StateView> for {ServiceName}State {
    fn from(view: {ServiceName}StateView) -> Self {
        Self {
            resources: view
                .resources
                .into_iter()
                .map(|(k, v)| (k, ResourceState::from(v)))
                .collect(),
        }
    }
}

impl From<ResourceStateView> for ResourceState {
    fn from(view: ResourceStateView) -> Self {
        Self {
            resource: Resource {
                name: view.name,
            },
            // reinitialise transient fields here
        }
    }
}

impl StatefulService for {ServiceName}Service {
    type StateView = {ServiceName}StateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().unwrap();
        {ServiceName}StateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().unwrap();
            *guard = {ServiceName}State::from(view);
        } // write guard dropped here — must precede notify_state_changed (which takes a read lock)
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().unwrap();
            for (name, resource_view) in view.resources {
                guard.resources.insert(name, ResourceState::from(resource_view));
            }
        } // write guard dropped here — must precede notify_state_changed (which takes a read lock)
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
```

Reference implementations ( all include the `StateChangeNotifier` pattern ):
`crates/winterbaume-s3/src/views.rs`, `crates/winterbaume-sqs/src/views.rs`, `crates/winterbaume-iam/src/views.rs`.

### `src/handlers.rs`

Choose handler pattern based on protocol. Reference implementations:
- **awsJson**: `crates/winterbaume-athena/src/handlers.rs`
- **restJson1**: `crates/winterbaume-sesv2/src/handlers.rs` ( SES v1 is `awsQuery`, not restJson1 — make sure you reach for `sesv2` here )
- **awsQuery**: `crates/winterbaume-sts/src/handlers.rs`

The error-shaping helper must use an **exhaustive match** (no wildcard `_` arm) so that adding a new error variant in `state.rs` forces a compile error until the handler is updated:

```rust
fn service_error_response(err: &{ServiceName}Error) -> MockResponse {
    let (status, error_type) = match err {
        {ServiceName}Error::NotFound { .. } => (404, "NotFoundException"),
        {ServiceName}Error::AlreadyExists { .. } => (400, "AlreadyExistsException"),
        {ServiceName}Error::Validation { .. } => (400, "ValidationException"),
        // ... all variants — NO wildcard arm
    };
    // Message comes from thiserror's Display impl — do NOT duplicate it here
    rest_json_error(status, error_type, &err.to_string())
}
```

The `(status, error_type)` pair is the only thing determined in the match. The message always comes from `err.to_string()` (which delegates to the `#[error("...")]` attribute). Never construct message strings inside match arms — that duplicates the text and creates a maintenance hazard.

Common structure:

```rust
use crate::state::{ServiceError, ServiceState};
use crate::views::{ServiceName}StateView;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, DEFAULT_ACCOUNT_ID,
};
use serde_json::{json, Value};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub struct {ServiceName}Service {
    pub(crate) state: Arc<BackendState<{ServiceName}State>>,
    pub(crate) notifier: StateChangeNotifier<{ServiceName}StateView>,
}

impl {ServiceName}Service {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for {ServiceName}Service {
    fn default() -> Self { Self::new() }
}

impl MockService for {ServiceName}Service {
    fn service_name(&self) -> &str { "{service-name}" }
    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://{service-name}\..*\.amazonaws\.com"]
    }
    fn handle(&self, request: MockRequest) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request) })
    }
}
```

In the `dispatch` method, capture the response in a `let` binding and call
`notify_state_changed` after any successful mutating request before returning:

```rust
fn dispatch(&self, request: MockRequest) -> MockResponse {
    // ... extract account_id, region, method ...
    use winterbaume_core::StatefulService;

    let response = match ... { /* routing match */ };

    if matches!(method, "PUT" | "POST" | "DELETE" | /* protocol-specific mutating verbs */)
        && response.status / 100 == 2
    {
        self.notify_state_changed(account_id, region);
    }
    response
}
```

For awsJson / awsQuery services where every request is a POST, use a per-action allowlist
instead of filtering by HTTP method (e.g., match the action name against a known set of
mutating operations).

### Generate wire.rs and model.rs

```bash
cargo run -p smithy-codegen -- gen-serializers {model-name} \
  --output crates/winterbaume-{name}/src/wire.rs \
  --model-output crates/winterbaume-{name}/src/model.rs
```

---

## Step 4: Implement operations

For each operation:

### 4a. State method

In `state.rs`, add a method returning `Result<T, {Service}Error>` with input validation, existence checks, and state mutation. Return domain-specific enum variants — never construct error structs with raw `error_type`/`message`/`status` strings. Example:

```rust
pub fn get_item(&self, name: &str) -> Result<&MyItem, {ServiceName}Error> {
    self.items.get(name).ok_or_else(|| {ServiceName}Error::NotFound {
        resource_type: "MyItem",
        name: name.to_string(),
    })
}
```

### 4b. Types (if needed)

In `types.rs`, add domain structs for new resource types.

### 4c. Handler dispatch entry

**awsJson**: match arm in `match action.as_str() { ... }`:
```rust
"OperationName" => self.handle_operation_name(&state, &body),
```

**restJson1**: match arm in `match (method, segments.as_slice()) { ... }`:
```rust
("POST", ["resource-path"]) => { ... self.handle_operation_name(...) }
```

If replacing a 501 stub, remove the old stub arm.

### 4d. Handler method

> **Rule: always use the generated serialiser for success responses.** Never build a success response body with `json!()` or `MockResponse::json()` directly. The generated `wire::serialize_*_response()` functions encode correct wire field names (PascalCase), handle `skip_serializing_if = "Option::is_none"` for optional fields, and stay in sync with the model. Hand-rolled JSON silently breaks whenever the model is regenerated.
>
> Error responses may still use `error_response()` / `json!()` because they have a fixed two-field shape (`__type` + `message`) that is not generated.

> **Rule: `@httpPayload` blob bodies are opaque bytes — never text-decode them.** When an operation binds a **blob** member to `@httpPayload` (object bodies, invocation payloads, upload bodies), the codegen renders it as `bytes::Bytes` and the deserialiser stores `request.body.clone()` with no validation. Handlers must treat that field as raw bytes: store / stream it verbatim, and **never** call `std::str::from_utf8`, `serde_json::from_slice` (as a gate), or an XML parser on it as a precondition — doing so `400`s any binary body (issue #12). Store into the blob store or a `Vec<u8>`/`Bytes` state field. If the *service* genuinely constrains the format (e.g. an OpenAPI import, a JSON shadow document, an SDF batch — see the dossier's "Opaque binary payloads" note), validate the *parsed* content inside the handler and return the service's real error, but do not let the wire deserialiser reject bytes up front. Non-payload blobs are a different case — they arrive base64-encoded as `String`.

```rust
fn handle_operation_name(
    &self,
    state: &Arc<std::sync::RwLock<ServiceState>>,
    body: &Value,
) -> MockResponse {
    // 1. Extract and validate input fields
    let field = match body.get("FieldName").and_then(|v| v.as_str()) {
        Some(f) => f,
        None => return error_response(400, "ValidationException", "Missing 'FieldName'"),
    };

    // 2. Call state method
    let mut state = state.write().unwrap();
    match state.operation_name(field) {
        Ok(result) => {
            // 3. Build wire output struct and pass to generated serialiser — never use json!() here
            wire::serialize_operation_name_response(&wire::OperationNameOutput {
                field: result.field.clone(),
                ..Default::default()
            })
        }
        Err(e) => service_error_response(&e),
    }
}
```

### 4e. Timestamp format

Check the generated `model.rs` to see whether timestamp fields are `f64` or `String`:
- `f64` → use `timestamp() as f64` (epoch seconds — default for all JSON protocols)
- `String` → use `to_rfc3339()` (the service has `@timestampFormat("date-time")` override)

---

## Step 5: Add round-trip stub tests

In `tests/integration_test.rs`, add **basic per-operation tests** using the real AWS SDK client. The goal at this stage is to verify each handler you just implemented compiles and round-trips correctly. These tests are **necessary but not sufficient** — they prove the handler exists and serialises, not that it implements the documented contract. Comprehensive per-operation coverage ( full-input round-trip, per-call uniqueness, validation, idempotency ) and the cross-call invariant inventory come from the mandatory Step 6b ( `/write-tests` ), which gates publication.

### Required test categories

1. **Happy path** — at least 1 test per operation
2. **Error path** — get/delete/update nonexistent resources
3. **Lifecycle** — create -> describe -> delete -> verify-gone
4. **Tags** (if implemented) — add -> list -> remove -> verify
5. **State views** (if implemented) — snapshot -> restore -> verify, and merge -> verify additive behaviour without removing pre-existing resources
6. **State change notifications** (if StatefulService implemented) — register a listener, perform a mutating operation, assert the listener was called with the correct account_id/region and a snapshot that reflects the mutation

### Rule: construct `*View` literals through small helper functions, not raw all-field struct literals

When the service exposes state views, **never** open up a `FooView { field: ..., another_field: ..., yet_another: ..., /* every field */ }` literal in the body of a test. Add small helper functions at the top of `tests/integration_test.rs` that take only the load-bearing identifiers and return a fully-populated view with sensible defaults for everything else. Top-level container views ( e.g. `{ServiceName}StateView` ) should `#[derive(Default)]` so callers can write `..Default::default()`; nested resource views with required non-`Default` fields ( strings, timestamps ) use a `mk_foo_view(id, ...)` helper instead.

This rule is load-bearing: `winterbaume-s3files` learned the hard way that adding policy, synchronisation configuration, mount targets, and access points to `FileSystemView` / `S3FilesStateView` broke every raw all-field literal in the test suite until each one was refactored around `fs_view(id, bucket)`. The same cascade hits any state-view-heavy service ( s3, sqs, iam, dynamodb, … ) the moment a new durable field lands. Helper functions absorb the diff in one place.

Pattern, lifted from `crates/winterbaume-s3files/tests/integration_test.rs`:

```rust
use winterbaume_{name}::views::{ResourceView};

fn mk_resource_view(id: &str, /* other load-bearing keys */) -> ResourceView {
    ResourceView {
        id: id.to_string(),
        // every required field gets a sensible default literal here, in one place
        // (note: this helper does *not* end in `..Default::default()` because
        //  ResourceView usually has required non-`Default` fields)
        ..
    }
}

// Callers stay short and resilient to schema growth:
let mut resources = HashMap::new();
resources.insert("res-a".to_string(), mk_resource_view("res-a"));
let view = {ServiceName}StateView { resources, ..Default::default() };
```

When a test needs a non-default value for one of the fields the helper hides, prefer mutating the returned view in-place ( `let mut v = mk_resource_view("res-a"); v.policy = Some(p);` ) over inventing a wider helper signature.

Example notification test:

```rust
#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};
    use winterbaume_core::StatefulService;
    let svc = {ServiceName}Service::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2.lock().unwrap().push((account_id.to_string(), region.to_string()));
    });

    // Trigger a mutation via restore or a handler call
    svc.restore("123456789012", "us-east-1", Default::default()).await.unwrap();
    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(got[0], ("123456789012".to_string(), "us-east-1".to_string()));
}

#[tokio::test]
async fn test_state_change_listener_snapshot_reflects_mutation() {
    use std::sync::{Arc, Mutex};
    use winterbaume_core::StatefulService;
    let svc = {ServiceName}Service::new();

    // Pre-seed state — construct the view via helpers + ..Default::default(),
    // not by spelling out every field of {ServiceName}StateView inline.
    let mut resources = std::collections::HashMap::new();
    resources.insert("expected-key".to_string(), mk_resource_view("expected-key"));
    let view = {ServiceName}StateView { resources, ..Default::default() };
    svc.restore("123456789012", "us-east-1", view).await.unwrap(); // ignore first event

    // Re-register and capture snapshot
    let snapshots: Arc<Mutex<Vec<{ServiceName}StateView>>> = Arc::new(Mutex::new(vec![]));
    let snapshots2 = Arc::clone(&snapshots);
    svc.notifier().subscribe(move |_account_id, _region, view| {
        snapshots2.lock().unwrap().push(view.clone());
    });

    let mut resources2 = std::collections::HashMap::new();
    resources2.insert("expected-key".to_string(), mk_resource_view("expected-key"));
    let view2 = {ServiceName}StateView { resources: resources2, ..Default::default() };
    svc.restore("123456789012", "us-east-1", view2).await.unwrap();
    let got = snapshots.lock().unwrap();
    assert_eq!(got.len(), 1);
    // assert that the snapshot reflects the new state
    assert!(got[0].resources.contains_key("expected-key"));
}
```

### Test structure

```rust
use aws_sdk_{sdkname}::config::BehaviorVersion;
use winterbaume_{name}::{ServiceName}Service;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_{sdkname}::Client {
    let mock = MockAws::builder()
        .with_service({ServiceName}Service::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_{sdkname}::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_{sdkname}::Client::new(&config)
}
```

### Tips

- Required SDK fields return `&str`; optional fields return `Option<&str>`
- **Accessor optionality is per response type, not per service.** The same logical field ( e.g. `file_system_id`, `arn`, `name` ) can be a plain `&str` on one operation's response and `Option<&str>` on another's, because Smithy marks members `@required` or not independently on each output shape. `winterbaume-s3files` is the canonical example: file-system, policy, and mount-target responses mix `&str` and `Option<&str>` accessors for what looks like the same field. Read each response's accessor signature from the SDK source ( or rely on the compiler ) — do not extrapolate from a sibling operation.
- Enum fields need `aws_sdk_xxx::types::EnumName::Variant`, not strings
- For `assert_eq!` on enums, dereference: `&SomeEnum::Variant`

---

## Step 6: Build and test

```bash
cargo build -p winterbaume-{service}
cargo clippy -p winterbaume-{service}
cargo test -p winterbaume-{service}
```

Common issues:
- Missing `..Default::default()` on wire struct initializers
- `Option<T>` vs `T` mismatches (model.rs required vs optional fields)
- Unused imports from replaced unimplemented operation stubs
- `needless_update`: if clippy warns that a `..Default::default()` has no effect (all fields already specified), remove it
- `too_many_arguments`: suppress with `#[allow(clippy::too_many_arguments)]` on state methods that genuinely require many parameters
- `type_complexity`: suppress with `#[allow(clippy::type_complexity)]` on state methods returning complex nested tuples

> **`--maxfail` is pytest, not cargo.** `--maxfail` is a pytest flag, not a Rust libtest flag. For Rust crate verification, use focused filters ( `cargo test <pattern>` ), `--test-threads=N`, `--no-fail-fast`, and per-crate gates instead. Do not pass `--maxfail` to cargo test.

---

## Step 6b: Mandatory — invoke /write-tests before declaring done

Step 5's round-trip stubs are **necessary but not sufficient**. Before this skill returns and before any subsequent invocation of `/quality-gate`, run `/write-tests {service}`. The publication gate ( `quality-gate` §2 ) requires the cross-call invariant inventory and `tests/scenario_test.rs` that `/write-tests` produces — without them the gate fails closed.

`/write-tests` will:

- Reuse `.agents/docs/services/<model-slug>.md` as the local baseline, then refresh or extend it with AWS Documentation MCP research when the service doc is stale, incomplete, or lacks official-doc coverage for the operations you touched. If the dossier is missing, run `/service-dossier {service}` first and resume after the dossier exists.
- Read the AWS *user guide* ( not just the API reference ) to extract a **cross-call invariant inventory** — toggle propagation, modify-rewrites-sibling-state, per-call uniqueness, default inheritance from parent, lifecycle state transitions, cross-resource references on read. The inventory is appended to `JOURNAL.md` and gates the publication scenario file.
- Add per-operation tests to `tests/integration_test.rs` for any documented behaviours Step 5 didn't cover ( validation rules, defaults, error semantics, idempotency, full-input round-trip, per-call uniqueness, FIX(terraform-e2e) markers )
- Add end-to-end **scenario tests** to `tests/scenario_test.rs` that chain multiple operations and explicitly cover each non-N/A invariant row

**Do not skip this step.** A "genuinely stateless" service ( `pricing`, `translate`, `comprehend`, `forecastquery`, `*-data` planes ) still runs `/write-tests`; the invariant inventory simply marks each row `N/A — <reason>` with a doc URL. The gate accepts that, but it requires the artefact.

If a deliberate deferral is unavoidable ( e.g. you are landing a stub crate as a placeholder ), file a `TODO.md` row tracking the missing inventory before the gate runs. The gate will fail closed referencing that TODO row until it is resolved — there is no silent grandfathering.

---

## Step 7: Implement the Terraform converter

The `crates/winterbaume-terraform` crate maps Terraform resource attributes to winterbaume service state via the `TerraformResourceConverter` trait. Each Terraform resource type (e.g. `aws_sns_topic`) needs one converter. Services that expose `StatefulService` state views are eligible; if the service has no state view, skip this step.

### 7a. Check for an existing converter

```bash
ls crates/winterbaume-terraform/src/converters/{service}.rs 2>/dev/null && echo "EXISTS" || echo "NEW"
```

If the file already exists, add any missing resource-type converters to it and skip to step 7e.

### 7b. Identify the Terraform resource types

Look up the Terraform AWS provider docs (or check existing converters for naming conventions) to find the `aws_{service}_{resource}` type names that correspond to the resources managed by this service. Only implement converters for resource types whose underlying data is fully captured by the service's state view.

### 7c. Create `crates/winterbaume-terraform/src/converters/{service}.rs`

Pattern (one struct per Terraform resource type, all sharing the same `Arc<{ServiceName}Service>`):

```rust
//! Terraform converters for {Service} resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_{service}::{ServiceName}Service;
use winterbaume_{service}::views::{ServiceName}StateView;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionResult, ExtractedResource, ConversionContext, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

pub struct Aws{Resource}Converter {
    service: Arc<{ServiceName}Service>,
}

impl Aws{Resource}Converter {
    pub fn new(service: Arc<{ServiceName}Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for Aws{Resource}Converter {
    fn resource_type(&self) -> &str { "aws_{service}_{resource}" }

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
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl Aws{Resource}Converter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        // 1. Pull required/optional fields from attrs with require_str / optional_str
        // 2. Build the typed view structs from the service's views module
        // 3. Call self.service.merge(&ctx.default_account_id, &region, view)
        //    (prefer merge over restore so concurrent resources accumulate)

        Ok(ConversionResult { region, warnings: vec![] })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // 1. Snapshot state: self.service.snapshot(&ctx.default_account_id, &region)
        // 2. Iterate resources, convert to serde_json::Value attribute maps
        // 3. Return Vec<ExtractedResource>
        Ok(vec![])
    }
}
```

Key rules:
- Use `merge(account_id, region, view)` (not `restore`) in `do_inject` so that multiple resources in the same plan accumulate rather than overwriting each other.
- Use `require_str` for mandatory Terraform attributes; use `optional_str` / `optional_i64` for optional ones.
- Derive ARNs from attributes if present, otherwise synthesize them (`arn:aws:{service}:{region}:{account_id}:...`).
- Reference a similar converter for your protocol: `converters/sns.rs` (simple), `converters/dynamodb.rs` (complex with nested blocks), `converters/iam.rs` (policy documents).

### 7d. Register the converter in `converters/mod.rs`

Add `pub mod {service};` in alphabetical order.

### 7e. Register the converter in the server

In `crates/winterbaume-server/src/main.rs`, find where `TerraformInjector` is built (look for `.register(`) and add:

```rust
injector.register(winterbaume_terraform::converters::{service}::Aws{Resource}Converter::new(Arc::clone(&{service}_svc)));
```

### 7f. Build and verify

```bash
cargo build -p winterbaume-terraform
cargo build -p winterbaume-server
```

---

## Step 8: Integrate into workspace (new crates only)

### 8a. Workspace `Cargo.toml`

Add in three places:
1. `[workspace] members` — add `"crates/winterbaume-{name}"`
2. `[workspace.dependencies]` — add `winterbaume-{name} = { path = "crates/winterbaume-{name}" }`
3. AWS SDK deps — add `aws-sdk-{sdkname} = "1"` if not already present

### 8b. Server integration

In `crates/winterbaume-server/Cargo.toml`, add: `winterbaume-{name} = { workspace = true }`

In `crates/winterbaume-server/src/main.rs`, add to `register_all_services()`:
```rust
Arc::new(winterbaume_{name}::{ServiceName}Service::new())
```

### 8b-2. Root package dev-dependencies (new crates only)

In the root `Cargo.toml`, add entries to **both** the `[dev-dependencies]` section and (if not already present) the `[workspace.dependencies]` section:

```toml
# [dev-dependencies]
winterbaume-{name} = { workspace = true }
aws-sdk-{sdkname} = { workspace = true }
```

Insert in alphabetical order alongside the existing entries.

### 8c. Inject 501 stubs for unimplemented operations

```bash
cargo run -p smithy-codegen -- inject {model-name}
```

### 8d. Register the crate in coverage tooling

Register every new crate in `CRATE_TO_MODEL` ( `.agents/skills/api-coverage/scripts/generate_coverage.py` ). The 2026-04-27 audit found 47 missing mappings ( 164 reported services vs 211 actual ) because this step was implicit. After adding the entry, run `python3 .agents/skills/api-coverage/scripts/generate_coverage.py` and verify the three-counts canary ( crates / READMEs / docs pages ) before declaring the service shipped.

---

## Step 9: Add example

Check whether an example already exists:

```bash
ls examples/{service}.rs 2>/dev/null && echo "EXISTS" || echo "NEW"
```

If it EXISTS, skip this step.

If NEW, create `examples/{service}.rs` following this template (pick the most representative **read or list** operation that was just implemented — prefer a `List*` or `Describe*` call that requires no mandatory arguments):

```rust
//! Example: {ServiceNameTitle}
//!
//! Demonstrates using aws-sdk-{sdkname} with winterbaume.
//!
//! Run with:
//!   cargo run --example {service} --package winterbaume-examples

use aws_sdk_{sdkname}::config::BehaviorVersion;
use winterbaume_{name}::{ServiceName}Service;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service({ServiceName}Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_{sdkname}::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_{sdkname}::Client::new(&config);

    let resp = client
        .{list_or_describe_operation}()
        .send()
        .await
        .expect("{list_or_describe_operation} should succeed");
    println!("{ServiceNameTitle}: {:?}", resp.{primary_output_field}());
}
```

**Guidelines:**
- Choose a zero-argument list/describe call as the demo operation — callers can see the service boots and responds without needing to create resources first.
- If no zero-argument list/describe exists (e.g. the service only has create/get/delete), chain a create then a describe or list call and `.expect()` each.
- Keep the example minimal (< ~40 lines). No error handling beyond `.expect()`.
- The `--package winterbaume-examples` label in the run comment matches the convention used by all existing example files.

---

## Step 10: Write Terraform E2E tests

If the service has a Terraform converter (Step 7), write Terraform E2E tests to verify that real `terraform apply` workflows work against the in-process server. Use the `/write-e2e-tests {service}` skill, which covers smoke testing, handler fixes, full test suite authoring, and harness registration.

Skip this step if:
- The service has no Terraform converter (no state view)
- E2E tests already exist for this service (`tests/e2e/terraform/{service}.rs`)

---

## Step 11: Verify and report

```bash
cargo run -p smithy-codegen -- ops {model-name} 2>&1 | grep '\[x\]' | wc -l
```

### Checklist

- [ ] All target operations implemented (handlers + state + tests)
- [ ] `.agents/docs/services/INDEX.md` consulted and the matching `.agents/docs/services/<model-slug>.md` read before selecting scope
- [ ] If no matching dossier existed at the start, `/service-dossier {service}` was invoked and completed before implementation scope was selected
- [ ] `AWS model slug` and `AWS SDK for Rust slug` from the service document used for model lookup and SDK dependency/import naming
- [ ] Service-specific notes from `.agents/docs/services/<model-slug>.md` reflected in implementation and tests, or explicitly skipped with justification
- [ ] `./.agents/bin/cargo.sh build -p winterbaume-{service}` — no errors
- [ ] **Lint gate (mandatory before reporting)**: `./.agents/bin/cargo.sh clippy -p winterbaume-{service} --all-targets --all-features -- -D warnings` — passes with no warnings (model.rs/wire.rs are generated; their warnings are suppressed by the generator). If new clippy violations appear, fix them; do not finish with outstanding warnings.
- [ ] **Fmt gate (mandatory before reporting)**: `./.agents/bin/cargo.sh fmt -p winterbaume-{service} -- --check` — passes. If it fails, run without `--check` to apply the formatter.
- [ ] `./.agents/bin/cargo.sh test -p winterbaume-{service}` — all tests pass
- [ ] Every new operation has at least one integration test
- [ ] If the service exposes state views, snapshot / restore / merge behaviour is covered by tests
- [ ] If the service exposes state views, at least one test asserts that a registered `StateChangeNotifier` listener fires after a mutation (restore, merge, or handler call) and receives a snapshot reflecting the change
- [ ] Wire struct initializers use `..Default::default()`
- [ ] Every success response uses `wire::serialize_*_response()` — no `json!()` or raw `MockResponse::json()` for success bodies
- [ ] If any source file in the crate calls `chrono::Utc::now()` or any other `chrono::` API, the crate's `Cargo.toml` declares `chrono = { workspace = true }`. Recurring failure mode caught manually in the 2026-04-28 sweep ( elasticbeanstalk, panorama, workspaces ) — agents added the call without the dep, cascading to compile-time errors only after they returned.
- [ ] Every `*StateView` struct ( and its component view structs ) `#[derive(Default)]`, and every literal construction in tests, converters, or backend wrappers uses `..Default::default()`. State-view literal drift is the largest cascade source in `/tackle-todos` sweeps; per-crate quality gates do not catch the breakage, only `winterbaume-terraform` integration tests do.
- [ ] No 501 stubs remain for implemented operations
- [ ] `{ServiceName}Error` is a `#[derive(Debug, Error)]` enum in `state.rs` — no `error_type`/`message`/`status` fields, no HTTP status codes anywhere in the enum
- [ ] Every variant has an `#[error("...")]` attribute; the handler uses `err.to_string()` for the message, never duplicating message text in match arms
- [ ] The error-shaping function in `handlers.rs` has an exhaustive match (no wildcard `_` arm)
- [ ] Terraform converter implemented in `crates/winterbaume-terraform/src/converters/{service}.rs` (or skipped with justification if no state view)
- [ ] `cargo build -p winterbaume-terraform` — no errors
- [ ] `examples/{service}.rs` created (or confirmed pre-existing)
- [ ] For new crates: `winterbaume-{name}` and `aws-sdk-{sdkname}` added to root `[dev-dependencies]`
- [ ] `/write-tests {service}` invoked to expand integration coverage and add `tests/scenario_test.rs` (or skipped with justification — every operation is independent and stateless)
- [ ] Terraform E2E tests written and passing (`tests/e2e/terraform/{service}.rs`), or skipped with justification (no converter / no state view)
- [ ] Report: ops added, total test count, pass/fail
