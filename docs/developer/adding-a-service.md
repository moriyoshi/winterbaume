# Adding a Service

This guide walks through creating a new service crate from scratch, wiring it into the workspace, and reaching the standard of coverage and test quality expected of a winterbaume service.

## 1. Check the coverage table

Before starting, look up the service in the [Service Coverage](/reference/services) table to understand the expected operation count and protocol. Also check [moto's source](https://github.com/getmoto/moto/tree/master/moto) for the reference Python implementation if needed.

## 2. Create the crate

```sh
cargo new --lib crates/winterbaume-{svc}
```

Add it to the workspace `Cargo.toml` `[workspace.members]` list.

Add the standard dependencies to the new `crates/winterbaume-{svc}/Cargo.toml`:

```toml
[dependencies]
winterbaume-core  = { path = "../winterbaume-core" }
aws-smithy-types  = "1"
serde             = { version = "1", features = ["derive"] }
tokio             = { version = "1", features = ["full"] }
tracing           = "0.1"

[dev-dependencies]
aws-sdk-{svc}     = "1"
tokio             = { version = "1", features = ["full"] }
winterbaume-sts   = { path = "../winterbaume-sts" }
```

## 3. Generate `model.rs` and `wire.rs`

Find the slug for the service. `gen-serializers` takes a service slug (usually the crate suffix); models live under `vendor/api-models-aws/models/`:

```sh
cargo run -p smithy-codegen -- list-services
```

Then generate:

```sh
cargo run -p smithy-codegen -- gen-serializers {slug} \
    --output crates/winterbaume-{svc}/src/wire.rs \
    --model-output crates/winterbaume-{svc}/src/model.rs
```

Both files will carry `//! Do not edit manually. Regenerate with: smithy-codegen gen-serializers {slug}` headers. Never hand-edit them — patch the generator if the output is wrong. See [Smithy Codegen](./smithy-codegen).

## 4. Implement `src/types.rs`

Define any internal domain types that are convenient for `state.rs` and `handlers.rs`. These are free to model storage semantics without caring about wire formats.

```rust
// src/types.rs
#[derive(Clone, Debug)]
pub struct MyResource {
    pub id: String,
    pub name: String,
    // ...
}
```

## 5. Implement `src/state.rs`

Define the in-memory state for one `(account_id, region)` pair and implement `Default`:

```rust
// src/state.rs
use std::collections::HashMap;
use crate::types::MyResource;

#[derive(Default)]
pub struct MySvcState {
    pub resources: HashMap<String, MyResource>,
}

impl MySvcState {
    pub fn create_resource(&mut self, id: String, name: String) -> Result<&MyResource, String> {
        if self.resources.contains_key(&id) {
            return Err(format!("Resource {} already exists", id));
        }
        self.resources.insert(id.clone(), MyResource { id: id.clone(), name });
        Ok(self.resources.get(&id).unwrap())
    }
}
```

Validation errors are returned as typed values or `Err(String)` here and converted to `MockResponse` error shapes in `handlers.rs`.

## 6. Implement `src/views.rs`

Define `*StateView` and implement `StatefulService`:

```rust
// src/views.rs
use serde::{Serialize, Deserialize};
use winterbaume_core::{StatefulService, BackendState};
use crate::state::MySvcState;
use crate::types::MyResource;

#[derive(Default, Serialize, Deserialize)]
pub struct MySvcStateView {
    #[serde(default)]
    pub resources: Vec<MyResourceView>,
}

#[derive(Serialize, Deserialize)]
pub struct MyResourceView {
    pub id: String,
    pub name: String,
}

// Conversion helpers
impl From<&MyResource> for MyResourceView { /* ... */ }
impl From<MyResourceView> for MyResource { /* ... */ }

// StatefulService implementation on the service struct
// snapshot() / restore() / merge() read and write BackendState<MySvcState>
```

Rules:
- Use `#[serde(default)]` on every collection field so old snapshots still deserialize after schema evolution.
- `restore()` is a full replacement — drop all existing state for that account/region.
- `merge()` is additive — do not remove resources absent from the incoming view.
- Transient runtime state (caches, derived data) must be rebuilt in `restore()`, not persisted in the view.
- Drop all lock guards before any `.await` point.

## 7. Implement `src/handlers.rs`

This file owns protocol dispatch and operation routing. Structure:

```rust
// src/handlers.rs
use winterbaume_core::{MockRequest, MockResponse, BackendState};
use crate::state::MySvcState;
use crate::wire;

pub struct MySvcHandlers {
    state: BackendState<MySvcState>,
}

impl MySvcHandlers {
    pub fn new() -> Self {
        Self { state: BackendState::default() }
    }

    pub async fn handle(&self, req: MockRequest) -> MockResponse {
        // Parse operation from path / header / body depending on protocol
        let operation = parse_operation(&req);
        match operation.as_str() {
            "CreateResource" => self.handle_create_resource(req).await,
            "DescribeResources" => self.handle_describe_resources(req).await,
            _ => MockResponse::not_implemented(operation),
        }
    }

    async fn handle_create_resource(&self, req: MockRequest) -> MockResponse {
        let (account_id, region) = req.identity();
        let input = wire::parse_create_resource_request(&req).unwrap();
        let mut state = self.state.write(&account_id, &region).await;
        match state.create_resource(input.id, input.name) {
            Ok(r) => wire::serialize_create_resource_response(&wire::CreateResourceOutput {
                resource_id: Some(r.id.clone()),
            }),
            Err(e) => wire::error_resource_already_exists(&e),
        }
    }
}
```

Protocol-specific parsing details:

| Protocol | How to find the operation |
|---|---|
| `awsJson1.0` / `awsJson1.1` | `X-Amz-Target` header: `Service.OperationName` |
| `awsQuery` / `ec2Query` | `Action` field in URL-encoded body |
| `restJson1` / `restXml` | HTTP method + URL path (generated `wire.rs` has a `route()` function) |

## 8. Implement `src/lib.rs`

Expose the service struct that implements `MockService`:

```rust
// src/lib.rs
mod handlers;
mod state;
mod types;
pub mod views;
mod model;
mod wire;

pub use views::MySvcStateView;

use std::pin::Pin;
use futures::Future;
use winterbaume_core::{MockService, MockRequest, MockResponse};
use handlers::MySvcHandlers;

pub struct MySvcService(MySvcHandlers);

impl MySvcService {
    pub fn new() -> Self {
        Self(MySvcHandlers::new())
    }
}

impl MockService for MySvcService {
    fn service_name(&self) -> &str { "mysvc" }
    fn url_patterns(&self) -> &[regex::Regex] { &[] }  // fallback to service_name matching

    fn handle(&self, req: MockRequest)
        -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>>
    {
        Box::pin(self.0.handle(req))
    }
}
```

## 9. Register in `winterbaume-server`

Add the new service to `crates/winterbaume-server/src/main.rs`:

```rust
use winterbaume_mysvc::MySvcService;
// ...
mock.with_service(MySvcService::new())
```

And add the dependency to `crates/winterbaume-server/Cargo.toml`.

## 10. Write integration tests

Add `crates/winterbaume-{svc}/tests/integration_test.rs`:

```rust
use aws_sdk_{svc} as sdk;
use winterbaume_core::MockAws;
use winterbaume_mysvc::MySvcService;
use winterbaume_sts::StsService;

async fn build_client() -> (sdk::Client, MockAws) {
    let mock = MockAws::builder()
        .with_service(StsService::new())
        .with_service(MySvcService::new())
        .build();
    let config = mock.sdk_config("us-east-1").await;
    (sdk::Client::new(&config), mock)
}

#[tokio::test]
async fn test_create_and_describe() {
    let (client, _mock) = build_client().await;
    client.create_resource()
        .id("r-001")
        .name("test")
        .send().await.unwrap();

    let resp = client.describe_resources().send().await.unwrap();
    assert_eq!(resp.resources().len(), 1);
}
```

Port relevant tests from [moto's test suite](https://github.com/getmoto/moto/tree/master/tests) as additional test cases. These are the strongest behavioural specs available.

## 11. Stub handler convention

When an operation is accepted but not yet meaningfully implemented, mark it:

```rust
// STUB[no-state]: TrackingConfig requires live telemetry; always returns empty.
fn handle_get_tracking_config(&self) -> MockResponse {
    wire::serialize_get_tracking_config_response(&Default::default())
}
```

Categories: `no-state`, `no-engine`, `no-telemetry`, `org-integration`, `s3-integration`, `no-auth`, `delegation-api`.

Stubs are excluded from the coverage count. Remove the comment when you implement real logic.

## 12. Add a README

Run `npm run generate` in `docs/` to regenerate the service page from the crate README (once the README is created by the `update-readme` skill or written by hand).
