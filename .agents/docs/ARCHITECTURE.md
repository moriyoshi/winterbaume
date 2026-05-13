# Winterbaume Architecture

## Repository Layout

The workspace is organised around service-owned crates plus a small set of shared infrastructure and tooling crates:

```text
winterbaume/
  crates/
    winterbaume-core/             # request routing, state scopes, protocol helpers, MockAws
    winterbaume-<service>/        # one crate per AWS service
    winterbaume-*-redis/          # optional Redis-backed storage adapters
    winterbaume-sqlengine-duckdb/ # optional DuckDB-backed SQL execution
    winterbaume-terraform/        # Terraform state inject/extract layer
    winterbaume-tfstate-resource-models/
                                  # generated Terraform state resource models
    winterbaume-partiql/          # DynamoDB PartiQL parser and validator
    winterbaume-iam-rule-eval/    # IAM policy evaluator
    winterbaume-sfn-asl-eval/     # Step Functions ASL validator
    winterbaume-wafv2-*/          # WAFv2 rule parser and WCU calculator
    winterbaume-bedrock-flow-*/   # Bedrock flow parser and validator
    winterbaume-server/           # standalone HTTP endpoint
  tools/
    smithy-codegen/               # generated model.rs and wire.rs
    tf-converter-codegen/         # generated Terraform state model structs
    release-batch/                # dependency-ordered chunked cargo-release driver
    sccache-wrapper/              # cross-session Rust build cache wrapper
  tests/e2e/terraform/            # Terraform provider compatibility harness
  vendor/
    moto/                         # upstream moto reference tests
```

Service crates own their request handling and state transitions. Helper crates own reusable parsers, validators, evaluators, converters, and backend adapters when the logic is too broad or too expensive to keep in an HTTP handler. Shared crates provide infrastructure, not hidden service semantics: if a resource family belongs to an AWS service, its durable representation should be in that service crate's state and view layer unless a documented cross-service boundary says otherwise.

## Request Flow

```
aws_sdk_sts::Client::get_caller_identity()
  -> aws-sdk-rust serializes to HTTP request (awsQuery: POST with form body)
  -> MockAwsClient intercepts (implements HttpClient/HttpConnector)
  -> URL pattern matching finds StsService (regex on host)
  -> Converts smithy Request to MockRequest (method, uri, headers, body bytes)
  -> StsService.handle() dispatches:
     1. Parses URL-encoded form body for Action parameter
     2. Routes to handler (get_caller_identity, assume_role, etc.)
     3. Handler reads/writes StsState via BackendState
     4. Returns MockResponse with XML body
  -> MockAwsClient converts MockResponse back to smithy Response
  -> aws-sdk-rust deserializes XML response
  -> Caller gets typed output (GetCallerIdentityOutput)
```

## Core Components

### `MockAwsClient` (winterbaume-core/src/mock_client.rs)

Implements two aws-smithy-runtime-api traits:
- `HttpConnector::call(Request) -> HttpConnectorFuture` - the actual request handler
- `HttpClient::http_connector() -> SharedHttpConnector` - returns self

Uses regex-based URL routing to dispatch to registered `MockService` implementations.
Falls back to URI-based service name extraction (from host) when no URL pattern matches.

### Protocol helpers (winterbaume-core/src/protocol/)

`winterbaume-core` centralises shared protocol mechanics under `src/protocol/` so service crates can stay focused on service semantics rather than low-level wire formatting.

- `protocol/xml.rs`: XML serialization and response helpers for awsQuery and restXml services
- `protocol/json.rs`: JSON response helpers for awsJson and restJson services
- `protocol/common.rs`: shared utility types and protocol-neutral helpers

### `MockService` trait (winterbaume-core/src/service.rs)

Each AWS service implements:
- `service_name()` - e.g. "sts"
- `url_patterns()` - regex patterns for routing
- `handle(MockRequest) -> Pin<Box<dyn Future<Output = MockResponse>>>` - request handler

### `StatefulService` trait (winterbaume-core/src/views.rs)

All AWS service crates implement `StatefulService` on top of `MockService`.

The trait standardises an async typed per-account / per-region state view contract:

- `snapshot(account_id, region) -> StateView` reads the current state and returns a serde-compatible view
- `restore(account_id, region, view)` replaces the current state entirely from the supplied view
- `merge(account_id, region, view)` additively overlays the supplied view onto the current state and must not remove resources that already exist
- `notify_state_changed(account_id, region)` snapshots the updated state and publishes it through `StateChangeNotifier`

This is used for state injection / extraction workflows and deliberately separates the serialized contract from runtime-only implementation details.
`StateChangeNotifier` still invokes synchronous listeners, so any async side effect must be spawned by the listener.
`snapshot()` currently returns a plain view rather than `Result`, so blob read failures can be logged but not propagated directly to the caller.
This same contract also gates Terraform inject or extract support: converter logic is only durable when the underlying `StateView` can represent the resource faithfully and `merge()` can rehydrate it without deleting unrelated state.
When external callers build `StateView` fragments directly, the stable pattern is to derive `Default` on the top-level view and use `..Default::default()` at call sites so future view growth does not break injectors.

### `Vfs` and `BlobStore` (winterbaume-core/src/vfs.rs, blob_store.rs)

`winterbaume-core` provides an async storage abstraction for services that need durable payload persistence outside their in-memory state structs.

- `Vfs` is an object-safe async trait with `put`, `get`, `delete`, `list`, and `stat`
- `MemVfs` stores blobs in memory for tests and in-process runs
- `FsVfs` stores one file per key on disk, rejects path traversal, and treats `list(prefix)` as string-prefix matching rather than directory-boundary matching
- `BlobStore` wraps `Arc<dyn Vfs>` with a namespace and stores current blobs, versioned blobs, and composite blob manifests separately
- `BlobStoreMap` manages account/region-scoped child `BlobStore` instances under `{base}/{account_id}/{region}` so blob-backed payload state follows the same scope as `BackendState`

Blob-backed services such as S3, EBS, and Glacier use this layer to keep large object data out of heap-backed runtime state while preserving snapshot and restore workflows.

### `BackendState<B>` (winterbaume-core/src/state.rs)

Per-(account_id, region) state management. Uses `RwLock<HashMap<..., Arc<RwLock<B>>>>` for thread-safe lazy initialisation. Mirrors moto's `BackendDict` pattern.
`scopes_with_state()` exposes the sorted existing `(account_id, region)` pairs without creating empty buckets; Terraform extraction uses this read-only scope inventory for opted-in regional services.

### Pluggable storage backends (`crates/winterbaume-*/src/backend.rs`)

Several stateful services decouple protocol handlers from persistence behind object-safe async backend traits stored as `Arc<dyn *Backend>`.

- Service crates own the trait so handler semantics stay local to the service boundary
- `new()` keeps the default in-memory adapter
- `with_backend(...)` is the stable injection point for alternative persistence implementations
- External storage engines such as Redis live in separate crates like `winterbaume-sqs-redis` and `winterbaume-dynamodb-redis` so the core service crates stay dependency-light

This pattern currently backs services such as SQS, SNS, and DynamoDB. For those services, the backend is now the authoritative owner of durable state, and `StatefulService` snapshot, restore, and merge operations delegate through the backend rather than through a disconnected in-memory shadow copy.

### `MockAws` builder (winterbaume-core/src/mock_aws.rs)

Entry point. Registers services, builds MockAwsClient, provides credentials.

Key methods:
- `http_client()` - returns `SharedHttpClient` for SDK config
- `credentials_provider()` - returns mock credentials
- `sdk_config(region)` - convenience method that creates a fully-configured `aws_config::SdkConfig` (wraps `http_client()` + `credentials_provider()` + region into an `aws_config::defaults().load()` call)
- `mock_interceptor(rule_mode, rules)` - (requires `smithy-mocks` feature) creates an `aws_smithy_mocks::MockResponseInterceptor` with `allow_passthrough()` enabled, for combining winterbaume backends with per-operation `aws_smithy_mocks` rule overrides

Runtime handlers resolve the default account through `winterbaume_core::default_account_id()`. `winterbaume-server` installs this process-wide value from CLI/env/config precedence before service registration. The override preserves the existing single-global-account architecture; `MockAws::builder().account_id(...)` still affects the builder/getter path rather than creating per-instance handler identity.

## Per-Service Crate Pattern

Most service crates follow the same internal split:

- `src/handlers.rs`: protocol dispatch, request parsing, operation routing, error shaping, and conversion to response types
- `src/state.rs`: persistent in-memory backend state, validation rules, domain-specific errors, and state transitions
- `src/types.rs`: service-local internal types used by handlers and state
- `src/views.rs`: typed `StateView` definitions plus `StatefulService` implementation for snapshot / restore / merge support
- `src/backend.rs`: object-safe async backend trait plus the default in-memory adapter for services with pluggable storage or execution
- `src/model.rs`: generated Smithy-derived structs for request and response shapes
- `src/wire.rs`: generated serialiser functions plus `pub use super::model::*;`
- `tests/integration_test.rs`: `aws-sdk-rust` integration tests, parity ports, and fast regressions
- `tests/scenario_test.rs`: end-to-end use-case scenarios for services whose resources combine in workflows. Chains three or more operations and asserts business outcomes rather than per-API return shapes. Optional for stateless metadata-only services that opt out with explicit justification.

This pattern is intentional: internal state types remain free to model convenient storage semantics, while `model.rs` and `wire.rs` keep wire compatibility tied to Smithy models.

For service crates, the intended split is:

- `state.rs` owns the mutable runtime representation and transient fields that are convenient for request handling
- `state.rs` should return domain-oriented errors rather than AWS wire error structs or HTTP status codes
- `handlers.rs` owns the protocol boundary: routing, request parsing, and translation from domain failures into AWS-facing error envelopes
- `views.rs` owns the stable serde-facing contract used to serialize durable state
- conversions between the two are explicit so restore / merge code can reinitialise transient fields instead of leaking runtime internals into the view format

Implementation rules for `views.rs` are documented in `.agents/docs/QUALITY_GATE.md` under "StatefulService Contract and State Views".

## Smithy Codegen Subsystem

`tools/smithy-codegen` is part of the runtime architecture, not a side utility. It parses AWS Smithy models, discovers service operations, and generates the `model.rs` / `wire.rs` modules used by service handlers.

Key responsibilities:

- collect operations from both `service.operations` and nested Smithy resources
- generate request deserialisers where the Smithy input shape fully describes the wire request, so service handlers do not reimplement protocol parsing by hand
- resolve protocol-specific serialiser behaviour, including XML container wrappers and JSON/XML timestamp defaults
- keep generated model types separate from serialiser code so handlers can build typed outputs without hand-written response structs

When generated outputs are wrong, the durable fix is to patch `tools/smithy-codegen`, then regenerate and rerun the affected service tests.

Request parsing has the same ownership boundary as response serialisation. Generated `wire::deserialize_*_request` helpers now exist for awsQuery, ec2Query, restXml, restJson1, awsJson1.0, awsJson1.1, and rpc-v2-cbor when the Smithy input shape describes the request. Service handlers should call those helpers for modelled fields instead of normalising `serde_json::Value`, XML strings, or form parameters by hand. Manual parsing should remain reserved for dispatch, path labels, repeated query parameters that the shared helpers cannot yet represent, validation-only raw-body checks, and documented protocol quirks such as PATCH-style fields outside the Smithy model.

## Terraform Converter Subsystem

`crates/winterbaume-terraform` is an external inject or extract layer built on top of service-owned `StatefulService` contracts rather than on direct mutation of private runtime state.

- each converter wraps a concrete service handle and implements `TerraformResourceConverter`
- `do_inject()` parses Terraform attributes into service `StateView` fragments and calls `merge()`
- `do_extract()` snapshots service state and renders Terraform-facing resource attributes from the view
- `depends_on_types()` encodes resource graph ordering for inject flows

Flat Terraform state models are generated from `crates/winterbaume-terraform/specs/*.toml` by `tools/tf-converter-codegen`. The generated serde `*TfModel` structs live in `crates/winterbaume-tfstate-resource-models/src/`, and `winterbaume-terraform` re-exports that crate as `crate::generated` for stable converter imports. Most specs stay in `mode = "model_only"`: generated models decode provider-shaped attributes, while hand-written converters remain responsible for state projection, nested-block reshaping, warning-only semantics, ARN/URL templates, resource ordering, and registration.

The architectural rule is that converter coverage follows view coverage. If `views.rs` does not expose a durable resource family, the missing work is in the service view first and the converter second. `aws_route` is the reference asymmetry case: inject mutates route-table state, while extract is intentionally owned by the parent `aws_route_table` converter. The same rule applies to converter-authored view construction: use top-level `StateView` defaults rather than exhaustive field literals so schema growth stays backward-compatible for inject paths.

Nested Terraform blocks are not a converter-only concern. Shapes such as `logging_configuration`, `encryption_configuration`, `vpc_config`, and `snapshot_copy` typically require coordinated changes across service types, request handlers, `views.rs`, converter inject or extract logic, and Terraform E2E no-drift checks. Top-level converter coverage is therefore weaker than proving that apply is followed by a clean no-drift plan.

Two named round-trip-bug classes are worth watching for whenever a converter is authored or refactored: ( a ) **JSON-shape mismatch** -- the converter copies provider-schema-shaped JSON ( singleton block arrays, snake_case keys ) into a `*StateView` that the handler later decodes as AWS REST JSON ( camelCase ); the decode silently returns `None` and the resource loses fields when `Describe*` is called. AppFlow is the reference case. ( b ) **Identifier id/name swap** -- extract emits `"id"` as the resource name and `"<resource>_id"` as the real `<prefix>-...` identifier, but inject reads `"id"` first and falls through to the id-typed field only when absent; round-tripped resources end up with the wrong identifier. EC2 placement-group is the reference case. Both classes need a converter test that injects realistic Terraform state then exercises the corresponding SDK describe or list filter.

Converter implementation is also separate from converter activation. New converter structs are not part of the live system until the injector build path registers them where Terraform injection is assembled.

Terraform extraction is multi-scope only for services that opt in at registration time. `ConversionContext` carries default account and region fallbacks, `ExtractedResource` records the actual account and region, and `TerraformInjector::extract_all()` calls converters once per discovered scope through `register_with_scopes()` providers. After collection, same-type resources with colliding Terraform local names are all qualified with account/region suffixes. Global services remain default-scope unless they document a global-state contract.

## Cross-service Integration Boundaries

Winterbaume does not treat cross-service behaviour as a generic message bus or shared global callback layer. The intended integration boundaries are the same ones AWS documents:

- Lambda event-source mappings for services such as DynamoDB Streams, SQS, SNS, and Kinesis
- EventBridge rule targets and Pipes source adapters
- AppSync data sources, including DynamoDB, Lambda, OpenSearch, RDS Data API, HTTP endpoints, and EventBridge
- Step Functions optimised integrations, including Lambda and the narrower documented DynamoDB action set

This matters architecturally because it keeps service-to-service work anchored to real AWS contracts and prevents handler-local shortcuts from becoming de facto platform behaviour. The DynamoDB case is the clearest example: Step Functions does not imply general DynamoDB integration, only the documented `GetItem`, `PutItem`, `UpdateItem`, and `DeleteItem` path.

The Athena-Glue catalogue relationship is a known open integration seam. In real AWS, Athena's default data catalogue is the Glue Data Catalogue — most users create Glue databases and tables, then query them via Athena. Winterbaume currently accepts `DataCatalogType::Glue` at the CRUD level but does not resolve Glue-managed metadata during query execution. Closing this gap requires a catalogue-resolution layer that consults `winterbaume-glue` state when the target catalogue type is `Glue`, plus a schema-injection mechanism so the `AthenaQueryBackend` (or the DuckDB engine) receives table definitions.

### Shared State and Data Planes

Related AWS protocol surfaces are classified before their state is joined:

- shared-backend pairs should observe the same underlying resources where AWS does, with SES and SESv2 as the reference target.
- narrow shared-namespace pairs keep separate resource state but reserve the AWS-shared names, such as API Gateway custom domains or ELB load balancer names.
- intentionally separate services, including MediaPackage v2 and WAFv2, must not gain cross-API visibility merely because they share a family name.
- runtime and data-plane crates should validate against their control-plane siblings unless AWS explicitly exposes bring-your-own-result behaviour.

`winterbaume-dynamodbstreams` is the reference derived-service pattern: DynamoDB owns table metadata and change records, while DynamoDB Streams owns iterator and cursor state. New derived or data-plane services should follow that split rather than duplicating upstream lifecycle ownership.

Network-looking identifiers do not automatically imply EC2-backed validation. Many service crates store VPC, subnet, security group, VPC endpoint, VPC link, network interface, or load-balancer identifiers as service-local metadata or deterministic placeholders, and some VPC-specific operations remain explicit stubs. A service validates those identifiers against `winterbaume-ec2` only when its service dossier and implementation say so. The `## Current Network Resource Stub Semantics` section in service dossiers is the source of truth for that boundary and may be transcribed into generated crate READMEs.

## Rule Evaluator and Validator Crates

Winterbaume keeps semantic engines in dedicated crates when the logic is reusable, testable in isolation, or too dense to belong in HTTP handlers.

- `winterbaume-iam-rule-eval` owns IAM policy evaluation semantics used by simulation handlers
- `winterbaume-sfn-asl-eval` owns Step Functions definition parsing and validation used by `ValidateStateMachineDefinition`
- `winterbaume-wafv2-webacl-rule-parser` and `winterbaume-wafv2-wcu-calculator` split WAFv2 parsing from capacity maths
- `winterbaume-bedrock-flow-parser` and `winterbaume-bedrock-flow-validator` split Bedrock flow parsing from graph and type validation

Handlers at these boundaries should only parse requests, assemble engine inputs, invoke the pure crate, and map diagnostics or results back into wire types. When parity gaps remain, the durable fix belongs in the evaluator crate rather than in handler-local heuristics.

## Coverage and Documentation Reporting

Workspace-scale reporting is part of the repo architecture, not just a release artefact.

- `.agents/skills/api-coverage/scripts/generate_coverage.py` generates `.agents/docs/API_COVERAGE.md`
- `.agents/skills/update-readme/scripts/update_readme.py` uses that report to refresh the root `README.md` and per-crate `README.md` files
- `.agents/skills/api-coverage/scripts/generate_terraform_resource_coverage.py` and `generate_terraform_converter_coverage.py` generate Terraform resource and attribute coverage reports
- `.agents/skills/test-coverage/scripts/test_coverage.py` remains a narrower integration-test inspection tool rather than the source of truth for workspace reporting

`API_COVERAGE.md` now combines two reporting layers:

- implementation and emulator comparison coverage ( Winterbaume, moto, floci, kumo where available )
- validation coverage ( SDK integration tests and Terraform E2E coverage )

That split matters architecturally because operation counts, emulator comparison, SDK compatibility, and provider compatibility drift in different ways. The reporting layer also contains repo-specific heuristics and mappings, so missing coverage rows can mean stale detection logic rather than missing handlers or tests. Local verification needs to match CI here: `./.agents/bin/cargo.sh clippy --workspace --all-targets` is the important workspace-wide lint surface because it catches example and test drift that library-only runs miss. Coverage maintenance is partly parser maintenance as well, because some services expose their operation surface through public HTTP handler methods rather than only `handle_*` helpers, cached operation maps can drift alongside the source tree, and supported-service reporting now derives from real workspace crates rather than from a fallback stub registry.

Terraform coverage has the same parser-maintenance property. Resource-prefix overrides, generated `*TfModel` field detection, and macro-generated converter detection are all part of keeping `docs/reference/terraform.md`, `.agents/docs/TERRAFORM_RESOURCE_COVERAGE.md`, and `.agents/docs/TERRAFORM_CONVERTER_COVERAGE.md` trustworthy.

## Release and Publishing Architecture

Publishing is dependency-ordered rather than workspace-flat. `winterbaume-core` and standalone helper crates must publish before service crates, and the umbrella `winterbaume`, `winterbaume-server`, and Terraform meta-crates publish last because they depend on many internal packages.

`tools/release-batch/` is the first-public-release driver. It reads `cargo metadata`, removes `publish = false` packages, topologically sorts publishable crates, and emits or executes chunked `cargo release <version|level> -p ...` commands. This is necessary because crates.io checks the `publish_new` quota before cargo-release reaches any per-crate hook; a hook-based throttler cannot make an oversized first-launch batch pass the upfront quota check. The cargo executable is injectable with `--cargo <path>` or `WB_CARGO`, which lets operators route subprocesses through `.agents/bin/cargo.sh` while keeping displayed chunk commands in canonical `$ cargo ...` form.

The hosted CI workflow has deliberate short-circuiting: docs-only pushes to `main` stop after the path-filter job, and source-identical runs can skip deterministic jobs through prior-pass marker caches. Binary release targets are intentionally limited to currently shippable cargo-dist targets; musl Linux and Windows ARM are excluded until their cross-compilation toolchains are reliable again.

The root umbrella crate has a separate packaging constraint: its manifest is the workspace root `Cargo.toml`, so default cargo packaging would walk the whole repository. The root package therefore needs anchored `include` patterns such as `/Cargo.toml`, `/README.md`, and `/src/**/*.rs`; bare entries such as `README.md` match nested files and can pull in vendored models, generated docs, examples, and agent memory. `autoexamples = false` belongs with that whitelist while root `examples/` stays excluded from the published crate.

## Query Execution Backends

Athena and Redshift Data apply the same abstraction idea one layer higher: handlers keep the AWS protocol surface, while query execution is delegated to object-safe async backend traits in each service crate.

- `winterbaume-athena` exposes `AthenaQueryBackend`
- `winterbaume-redshiftdata` exposes `RedshiftQueryBackend`
- `with_query_backend(...)` injects an alternative engine without changing the protocol handlers
- `winterbaume-sqlengine-duckdb` provides the main concrete external engine, using `papera` plus DuckDB for opt-in SQL execution

The DuckDB engine crate is intentionally excluded from the default workspace path because it pulls in heavier native dependencies than the normal in-memory mock flow.

Both `DuckDbAthenaQueryBackend` and `DuckDbRedshiftQueryBackend` are injectable. Each stores a `Mutex<Connection>` and is constructed via `new(conn: Connection)`. Each query call takes a per-query clone via `Connection::try_clone()`, which opens a lightweight handle to the same underlying in-memory DuckDB database, so all handles share tables and data. This lets callers seed the database with schema or fixture data before constructing the backend, enabling cross-query state and schema-injection tests.

Standalone server integration is opt-in. `winterbaume-server` exposes paired `backend-sqlengine-duckdb` and `backend-sqlengine-duckdb-bundled` features; the first links against a prebuilt libduckdb, while the second enables the wrapper crate's bundled DuckDB build. Runtime configuration uses `[backends] sqlengine_duckdb`, `WB_SQLENGINE_DUCKDB`, or `--sqlengine-duckdb PATH`. When configured and feature-enabled, the server opens one DuckDB database and shares it between Athena and Redshift Data, so SQL state created through either service is visible to the other. When configured without the feature, the server logs a warning and falls back to in-memory backends.

## DynamoDB PartiQL Subsystem

Winterbaume's PartiQL support is split across two crates so parsing and AWS-specific validation stay independent from DynamoDB storage and execution.

- `crates/winterbaume-partiql` parses DynamoDB PartiQL statements directly into a typed `DdbOperation` representation with a hand-rolled lexer and recursive-descent parser under `src/parser/`
- `operation.rs` owns the IR, including `Expression`, `Condition`, statement structs, and the transaction-only `DdbOperation::Exists(Box<SelectOp>)` shape
- `param.rs` binds positional `?` parameters at value positions during parsing, not through textual pre-substitution
- `validate.rs` applies DynamoDB-specific runtime restrictions such as rejecting arithmetic in condition contexts, unary `-path`, and misplaced `EXISTS` with AWS-compatible validation errors
- `crates/winterbaume-dynamodb/src/partiql_exec.rs` maps the intermediate representation onto `DynamoDbBackend` query, scan, put, update, and delete operations through `execute_partiql_via_backend(...)`

This split keeps PartiQL parsing table-schema-agnostic while leaving key-condition routing and execution decisions inside the DynamoDB service crate. The stable boundary is that PartiQL remains a protocol-layer orchestration feature above the backend trait, but it no longer bypasses the backend abstraction.
DynamoDB secondary indexes are part of that backend contract: table creation carries typed GSI/LSI definitions, `query()` accepts an optional index name, and custom backends such as `winterbaume-dynamodb-redis` persist those definitions with serde defaults for backward compatibility.
The parser no longer depends on `partiql-parser` or `partiql-ast`; those upstream crates were removed because their lalrpop build script made fresh target directories expensive and the DynamoDB PartiQL surface was small enough to own directly. New syntax belongs in `parser/lexer.rs`, `parser/expr.rs`, and `parser/stmt.rs`; old references to `translate.rs` and `value.rs` are historical.

## Protocol and Routing Model

Services are matched in two stages:

1. URL pattern matching on the full request URI, used for service-specific host and path styles such as S3 virtual-hosted buckets.
2. Service-name extraction from the request host as a fallback.

After routing, protocol dispatch belongs to the service crate. Typical dispatch keys are:

- awsQuery and ec2Query: form body action parameters.
- awsJson1.0 and awsJson1.1: `X-Amz-Target`.
- restJson1 and restXml: HTTP method, path labels, and query parameters.
- rpc-v2-cbor: protocol headers and CBOR body framing.

Shared protocol helpers live in `winterbaume-core/src/protocol/`, but service-specific request semantics stay in `handlers.rs`. Generated `wire::deserialize_*_request` helpers should be used when the Smithy input shape describes the request. Manual parsing remains valid for operation dispatch, path labels, repeated query parameters not yet supported by shared helpers, raw payload responses, and protocol edge cases that cannot be represented by a generated model deserialiser.

## SDK, Server, and Interceptor Integration

`MockAws` is the common entry point for in-process use. It registers service implementations, provides mock credentials, returns a shared HTTP client, and exposes `sdk_config(region)` for SDK clients.

`winterbaume-server` wraps the same service implementations in a Hyper HTTP server. It routes by host and path just like the in-process client, which keeps Terraform and non-Rust client behaviour on the same backend contracts as Rust tests.

The optional `smithy-mocks` feature composes Winterbaume with `aws-smithy-mocks`: typed interceptor rules can match and override selected SDK operations, while unmatched requests pass through to Winterbaume's stateful HTTP backend. The interceptor sits in the SDK config; the Winterbaume HTTP client remains the transport.

## Validation Architecture

Regression coverage maps to the architectural boundary being exercised:

- `tests/integration_test.rs` verifies SDK transport compatibility and per-operation behaviour.
- `tests/scenario_test.rs` verifies multi-operation resource workflows and cross-call invariants.
- Moto parity ports supply behavioural examples where moto has useful coverage.
- AWS documentation plans define expected scenarios for services whose public workflows are richer than the Smithy operation list.
- Terraform E2E suites exercise provider read-after-write, waiter, defaulting, nested-block, and no-drift behaviour.

Terraform-driven service fixes are marked with `FIX(terraform-e2e)` at the code site and should gain a fast SDK integration regression. Coverage reports are intentionally treated as generated observability, not as an oracle: stale parser heuristics or resource-to-crate mappings can undercount real work.

## Architectural Constraints

- State is in-memory by default. Dropping a `MockAws` instance or stopping `winterbaume-server` discards service state unless a service uses an injected backend with its own persistence.
- `StatefulService::snapshot()` is currently non-fallible, so blob read failures can be logged but not returned to the caller as an error.
- Stub handlers are explicit architectural markers. `no-engine`, `no-telemetry`, `org-integration`, and related categories mean the missing behaviour needs a real subsystem or AWS integration boundary, not just a larger default response.
- Terraform converter coverage is bounded by state-view fidelity. If `views.rs` cannot represent a resource family, converter support for that family is incomplete even if a converter can compile.
- EC2's split generated crate and feature map are a special response to service scale. Smaller services should not copy that structure unless compile-time or feature-surface pressure requires it.
