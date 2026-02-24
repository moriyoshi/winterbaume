# New Service Implementation Patterns

## Summary

Recent service-addition sessions established repeatable patterns for adding AWS service crates quickly without losing behavioural quality. The useful durable knowledge is mostly in the small implementation details: how to select an initial operation slice, how SDK getters and errors behave in tests, how to encode composite keys in views, and how to wire notifier and workspace surfaces consistently.

## Key Facts

- The default initial slice for a new service is root-resource lifecycle: create, get or describe, list, delete, and update when it is cheap.
- Full operation coverage is worthwhile when the total surface is small. Larger service hierarchies should land root resources first and defer deeper families behind explicit 501 stubs.
- New service crates should wire the same workspace surfaces every time: workspace members, dependencies, feature flag, server registration, root crate optional dependency, dev-dependency, example, tests, and TODO status.
- AWS SDK Rust `SdkError` should be asserted with `format!("{:?}", err)`, not `Display`, because `Display` often prints only `service error`.
- Smithy `@required` members collapse SDK getters to `&T` instead of `Option<&T>`, even though generated wire output structs may still use `Option<T>`.
- SDK accessor optionality can vary between output shapes in the same service. Check each generated SDK response method instead of assuming a service-wide rule.
- Generated wire field names can differ between sibling output shapes. Check the exact generated struct instead of inferring from a related shape.
- Empty request bodies should parse as `{}` for REST GET handlers that share JSON parsing with body-bearing operations.
- State-change notifier tests should be present for every new stateful service.

## Details

### Operation Selection

The service-addition workflow uses an incremental scope:

- root resource create, get or describe, list, delete
- update when it is part of the same shallow resource family
- tag lifecycle when Terraform or SDK read-back normally probes tags
- deeper child resources only when they are required for a real caller or are small enough to finish completely

This split shipped full coverage for small services such as `aiops`, `appconfigdata`, `applicationcostprofiler`, `cloudcontrol`, `keyspaces`, and `rolesanywhere`. It also allowed partial but useful crates for larger hierarchies:

| Crate | Implemented scope | Deferred scope |
|-------|-------------------|----------------|
| `winterbaume-amplifybackend` | Backend create, get, delete, clone | BackendAPI, BackendAuth, BackendStorage, BackendConfig, Token, BackendJob, and S3 bucket helpers |
| `winterbaume-appfabric` | AppBundle lifecycle and tags | AppAuthorization, Ingestion, IngestionDestination, and user-access tasks |
| `winterbaume-appflow` | Flow lifecycle, start, stop, and tags | ConnectorProfile, connector registration, connector discovery, execution records, and metadata cache reset |

The durable rule is to make the deferred surface explicit and return 501 rather than fabricating behaviour.

### Workspace Wiring

Each new service crate should update the same touch points:

- root `Cargo.toml` workspace members and default members
- root `Cargo.toml` workspace dependency for `winterbaume-<name>`
- root `Cargo.toml` workspace dependency for `aws-sdk-<sdkname>`
- root crate optional dependency and `service-<name>` feature
- root crate `full` feature list
- root crate dev-dependencies for the service and SDK
- `crates/winterbaume-server/Cargo.toml`
- `crates/winterbaume-server/src/main.rs` service registration
- `examples/<name>.rs`
- `.agents/docs/TODO.md` service status

Register plain mock services in the server unless a service has a Terraform converter or other specialised integration path.

### SDK Test Assertions

AWS SDK Rust error display is intentionally terse. Integration tests that need to assert service error type should use debug output:

```rust
let err = client.get_resource().send().await.unwrap_err();
let msg = format!("{:?}", err);
assert!(msg.contains("ResourceNotFoundException"));
```

Using `format!("{}", err)` often yields only `service error` and hides the actual AWS error type.

SDK getters also reflect Smithy `@required`. For required fields, getters return `&str` or `&T`; optional fields return `Option<&T>`. Generated wire response structs may still store `Option<T>` because they derive `Default`. Tests should follow the SDK getter type, not the wire struct type.

The same service can mix required and optional SDK accessor shapes across operations. S3 Files is the reference trap:

- `CreateFileSystemResponse.file_system_id()` returns `Option<&str>`.
- `GetFileSystemPolicyResponse.policy()` returns `&str`.
- `CreateMountTargetResponse.mount_target_id()` returns `&str`.
- `CreateMountTargetResponse.vpc_id()` returns `Option<&str>`.
- `GetMountTargetResponse.subnet_id()` returns `&str`.

There is no reliable rule from nearby generated `model.rs` fields alone because the SDK applies operation-level required-output inference. Let `cargo build --tests` or the IDE type at the assertion site settle the exact shape.

### Generated Shape Asymmetry

Sibling output shapes can use different field names for the same concept. In `applicationcostprofiler`, `GetReportDefinitionResult` uses `last_updated`, while list-item `ReportDefinition` uses `last_updated_at`. The model defines these as separate shapes, and codegen preserves that difference.

The durable rule is to open the generated type before writing conversions.

### Identifier Resolution

Several services accept either a bare identifier or a full ARN in a path segment. The stable helper pattern is:

- if the value starts with `arn:`, return the trailing slash component
- otherwise use the value unchanged

This keeps handlers simple and avoids scattered ARN parsing. `aiops`, `appfabric`, and similar services use this for get, update, delete, and tag operations.

### Composite Keys in Views

`StateView` maps must serialise cleanly through serde. If runtime state uses tuple keys such as `(app_id, environment_name)`, encode the view key as one string. The recent convention is `format!("{}\x00{}", app_id, environment)` because NUL cannot legally appear in either component.

Views should decode by splitting once on `\x00` and reject malformed keys through `StateViewError`.

### Handler Body Defaults

REST JSON handlers that parse the request body as `serde_json::Value` should use `{}` when `request.body` is empty. Otherwise valid GET requests can fail before routing with an invalid JSON error.

### AWS-Managed Corpus Services

Some services have no public `Create*` API for their primary resource because the content is AWS-curated ( e.g. AWS Artifact's compliance reports and customer agreements ). Tests cannot seed via SDK calls. The convention is to construct the service's `*StateView` directly and call `service.restore(...)` or `service.merge(...)` before exercising the SDK. This doubles as a Terraform-converter contract test, since converters use the same path. Note this in the crate `README.md` so future contributors do not search for a missing "create" API.

### Click-Through Token Patterns

Services that gate access on a click-through agreement ( AWS Artifact's `GetTermForReport` -> `GetReport` ) model the flow with a one-shot rotated token: store the most recently issued token per resource id in `state.term_tokens`, and validate that the supplied token matches before serving. This is the smallest-surface pattern that produces realistic SDK-level errors when tokens are missing or stale.

### Discriminated-Union Request Fields

Services that expose deeply-nested discriminated unions ( AppFlow's 17-variant `SourceConnectorProperties`; autoscalingplans's `ApplicationSource` ( CFN-stack-ARN | TagFilters ); `ScalingInstructions` per-target unions ) are most reliably stored as opaque `serde_json::Value` and round-tripped via `serde_json::from_value(v).ok()` when emitting the response. The trade-off is no inspection of union contents in handlers, which the mock typically does not need; the win is bounded crate compile time and no per-service union modelling.

### SDK Enum Variant Naming

`aws-sdk-*` enum variant naming is inconsistent across services. Within `aws-sdk-autoscalingplans`, `ScalingMetricType::AsgAverageCpuUtilization` is correctly capitalised whereas the older Smithy model exports it as `ASGAverageCPUUtilization`. The SDK builder applies acronym-aware capitalisation when generating Rust idents, but the rule is not predictable from the model name alone. **Always run `cargo test --no-run` first** and let the compiler print the canonical variant names via "help: there is a variant with a similar name" suggestions; do not infer them from the Smithy JSON.

### Required Scalar Getters Return `&str`, Not `Option<&str>`

When the Smithy model marks a member as `smithy.api#required`, the generated SDK getter elides the `Option`. `assert_eq!(plan.scaling_plan_name(), "test-plan")` is correct; `assert_eq!(plan.scaling_plan_name(), Some("test-plan"))` does not compile ( `can't compare &str with Option<&str>` ). This surfaces in newer aws-sdk-rust crates ( appfabric `bundle.arn()`, autoscalingplans `plan.scaling_plan_name()` ) and required two-line test fixes.

### Three-of-Eight: No Terraform Resource

A material fraction of new service crates have no `aws_<service>_*` resources in the Terraform AWS provider. In a recent eight-crate batch, five were skipped at Step 7 / Step 10 of the `add-service` skill: aiops ( brand-new AWS service ), amplifybackend ( deprecated Gen-1 ), appconfigdata ( runtime data API, not a resource type ), artifact ( read-mostly compliance-doc service ), and autoscalingplans ( historically had a resource but it is removed in current provider versions ). Confirm by checking the hashicorp/aws Terraform provider documentation index before authoring a converter.

### `ConversionError` Variant Naming

The Terraform converter error variant is `ConversionError::MissingAttribute`, not `MissingRequiredAttribute`. Caught at compile time on every newly authored converter; remember the correct name to avoid the rebuild loop.

### Notifier Test Boilerplate

Every new stateful service should include a minimal notifier test:

- subscribe a closure that records `(account_id, region)`
- call `restore("123456789012", "us-east-1", Default::default()).await`
- assert exactly one notification with the supplied scope

This is the cheapest regression check that `StatefulService` notification wiring is present.

### Scenario Test File ( `tests/scenario_test.rs` )

Mature service crates ship a separate `tests/scenario_test.rs` alongside the per-operation `tests/integration_test.rs`. Scenario suites exercise coherent end-to-end use cases ( e.g. SQS batch-job pipeline, SNS multi-subscriber hub, DynamoDB optimistic locking, S3 multipart upload ) rather than individual API calls. The `/write-tests` skill produces both files; `/add-service` delegates to it after the per-operation tests pass.

Decision criteria for whether scenario tests fit:

- resources combine in workflows
- stateful semantics emerge across calls
- chains of 3+ operations are realistic

Stateless metadata-only services ( Translate, Pricing, Forecast Query ) explicitly justify skipping in the JOURNAL plan instead of authoring synthetic scenarios.

Authoring rules:

- use-case names, not API names ( e.g. `test_batch_job_pipeline`, not `test_send_message_then_delete_message` )
- a `Scenario:` doc comment at the top of the test
- plausible resource names that match the use case
- assert business outcomes ( "the message was processed exactly once" ), not API-call return shapes
- each scenario must chain 3+ operations
- `#[ignore]` is acceptable for scenarios that depend on unimplemented features, with a *mandatory* cross-link to `TODO.md`. This makes the test a forcing function: when the underlying capability lands, removing `#[ignore]` flips it on without anyone needing to remember it was queued.

Reference suites: `crates/winterbaume-{sqs,sns,s3,dynamodb}/tests/scenario_test.rs`.

## Files

- `crates/winterbaume-*/src/handlers.rs`: operation routing, empty-body handling, error shaping, and wire response construction
- `crates/winterbaume-*/src/state.rs`: domain state, error enum, identifier resolution, and duplicate/not-found behaviour
- `crates/winterbaume-*/src/views.rs`: state view, restore, merge, and notifier wiring
- `crates/winterbaume-*/tests/integration_test.rs`: SDK integration tests and notifier checks
- `crates/winterbaume-server/src/main.rs`: service registration
- `Cargo.toml`: workspace, dependencies, features, and dev-dependencies
- `examples/*.rs`: root package example binaries
- `.agents/docs/TODO.md`: service implementation status

## Test Coverage

Recent new-service batches added immediate integration suites:

| Crate | Operations implemented | Tests |
|-------|------------------------|-------|
| `winterbaume-aiops` | 11 / 11 | 11 |
| `winterbaume-amplifybackend` | 4 / 31 | 6 |
| `winterbaume-appconfigdata` | 2 / 2 | 6 |
| `winterbaume-appfabric` | 7 / 26 | 6 |
| `winterbaume-appflow` | 10 / 25 | 6 |
| `winterbaume-applicationcostprofiler` | 6 / 6 | 6 |
| `winterbaume-artifact` | 8 / 8 | 7 |
| `winterbaume-autoscalingplans` | 6 / 6 | 7 |
| `winterbaume-outposts` | 13 / 35 | 18 |
| `winterbaume-accessanalyzer` | 11 / 37 | 17 |
| `winterbaume-fis` | 8 / 26 | 13 |
| `winterbaume-cloudcontrol` | 8 / 8 | 18 |
| `winterbaume-keyspaces` | 19 / 19 | 16 |
| `winterbaume-rolesanywhere` | 30 / 30 | 22 |
| `winterbaume-s3files` | 21 / 21 | 34 |

Tests should cover lifecycle, duplicate-create or conflict paths, not-found paths, tag lifecycle where implemented, state view round-trip or merge where meaningful, and notifier wiring.

For state-view-heavy crates, tests should construct views through small builder helpers and use `..Default::default()` on top-level state-view literals. S3 Files had to refactor raw `FileSystemView` literals after adding policy, synchronisation configuration, mount targets, and access points; the helper pattern localises future view-field additions.

## Pitfalls

- Do not assert SDK service errors with `Display`; use debug formatting.
- Do not infer SDK getter optionality from generated wire struct fields.
- Do not infer field names from sibling generated shapes.
- Do not start a large deprecated or low-demand service as a broad partial scaffold when a smaller next service can reach full coverage.
- Do not fabricate connector or discovery metadata for a pure mock when there is no source of truth; explicit 501 is better.
- Do not leave deferred operations undocumented.
- Do not copy local query-string helper blocks into new restJson1 crates. Promote missing shared helpers to `winterbaume-core` or keep a minimal local helper only for repeated query parameters.
- Do not use `version.workspace = true` in this workspace's service templates unless `[workspace.package].version` exists.
- Do not write a Terraform converter for a service that has no `aws_<service>_*` resources in the upstream Terraform AWS provider. Confirm by checking the provider documentation index; skipping is the right answer.
- Do not infer SDK enum variant names from Smithy JSON; let the compiler tell you the canonical name via the "help: similar variant" suggestion.
- Do not wrap required SDK getters in `Option<...>` in test assertions; `smithy.api#required` getters return `&str` directly.
- Do not assume getter optionality is uniform inside one service. Mixed `&str` and `Option<&str>` accessors can appear in adjacent response types.
- Do not use raw large `*StateView` struct literals in tests or examples when a builder helper plus `..Default::default()` would absorb future field additions.
- Do not invent a `Create*` SDK call for AWS-managed-corpus services like Artifact. Seed test state via `restore` / `merge` on the public `*StateView`.

## Patterns from the 2026-04-26 / 04-27 Multi-Service Batch

A ~40-crate addition pass produced repeated lessons that supersede earlier guesses. The shape of these crates ( appintegrations, backupsearch, backup-gateway, bcm-* trio, application-* trio, amplifyuibuilder, trustedadvisor, support-app, pricing, freetier, cloud9, s3outposts, route53-recovery-cluster, snow-device-management, simspaceweaver, taxsettings, ssm-quicksetup, pi, pca-connector-scep, cloudfront-keyvaluestore, cloudtrail-data, cloudsearch-domain, savingsplans, personalize-events, personalize-runtime, pinpoint-sms-voice, rbin, costandusagereport, braket, chimesdkmeetings, cognitosync, codestarnotifications, codegurusecurity, codegurureviewer, connectparticipant, connectcontactlens, controlcatalog, costoptimizationhub, billing ) means these patterns are now load-bearing.

### awsJson Protocol Variants

Three protocols dispatch on `X-Amz-Target` ( not URL ): `awsJson1.0`, `awsJson1.1`, and the older `AWSPriceListService.{Operation}` variant. Only the Content-Type header changes between 1.0 and 1.1 ( `application/x-amz-json-1.0` vs `1.1` ). `MockResponse::json()` works for both. Errors must be encoded as `{"__type": "ErrorType", "Message": "..."}` with a matching `x-amzn-errortype` header. Reference implementations: `winterbaume-pi`, `winterbaume-applicationdiscoveryservice`, `winterbaume-pricing`, `winterbaume-billing`.

### restJson1 with `@httpPayload`

Some restJson1 services use `@httpPayload` on the response's main body member. The body IS the payload directly ( `{...}` ), not wrapped in the operation-output envelope ( `{"entity": {...}}` ). The generator now handles struct-typed `@httpPayload` outputs; older local `payload_response<T: Serialize>(payload: &T)` helpers are historical workarounds, not the pattern for new crates.

### REST-JSON Query Parameters

Use `winterbaume_core::extract_query_string`, `parse_query_string`, `extract_path`, `percent_decode`, and `rest_json_error` before writing local URL helpers. A new restJson1 crate copied `extract_query_param` / `extract_query_param_multi` / `percent_decode` / `hex_val` from a sibling before being corrected; that would have extended the already-known query-helper drift across the fleet.

`parse_query_string` currently returns `HashMap<String, String>` and collapses repeated keys. Until `winterbaume-core` grows a shared multi-value helper, a small local repeated-query collector is acceptable for list query params such as `?tagKeys=a&tagKeys=b`. Keep the local helper narrowly named and do not copy the whole percent-decoder block.

### S3 Files Full Coverage

`winterbaume-s3files` is the reference for growing a brand-new AWS service from an explicit partial scaffold to full operation coverage when Terraform has no provider resources yet. It first landed as a FileSystem-only scaffold for model `s3files` version `2025-05-05`, then a second pass implemented the mount-target, access-point, file-system policy, synchronisation-configuration, and mount-target update families. The crate now covers all 21 / 21 operations, with no 501 stubs remaining.

Behavioural choices from the completed crate:

- `CreateFileSystem` and `CreateAccessPoint` idempotency replay by `clientToken` only. Real AWS rejects mismatched token replays with `ConflictException`; the mock returns the first resource for the token.
- file systems become `available` immediately rather than modelling `creating -> available`.
- ARN shape is plausible but provisional: `arn:aws:s3files:{region}:{account_id}:file-system/{fs_id}` with ids shaped as `fs-{uuid_simple}`.
- `ListFileSystems` supports the `bucket` query filter; pagination remains empty.
- `UntagResource` uses repeated `tagKeys` query parameters.
- mount targets derive a VPC tag from subnet ids shaped `subnet-<vpc-tag>-<az-tag>` and derive an availability-zone id through a deterministic byte hash modulo six. The mock enforces one derived VPC per file system and one mount target per derived AZ.
- `DeleteFileSystem` returns `ConflictException` while any mount target or access point references the file system. The `forceDelete` query parameter is parsed but currently does not bypass the in-use dependency check.
- file-system policy support enforces the documented 20,000 character limit; `GetFileSystemPolicy` returns `ResourceNotFoundException` when no policy exists.
- synchronisation configuration stores a version, rejects stale `latestVersionNumber` writes with `ConflictException`, and enforces the 10-rule cap on `importDataRules`; exact root-rule and most-specific-prefix semantics remain unmodelled.

Two skill-template pitfalls surfaced: this workspace does not define `[workspace.package].version`, so new service `Cargo.toml` files must use a literal `version = "0.1.0"` rather than `version.workspace = true`; and SES v1 is awsQuery, not a restJson1 reference. Use a true restJson1 handler such as SES v2 when choosing protocol examples. A later test refactor added another template lesson: large state-view fixtures should use small builder helpers plus `..Default::default()` instead of raw struct literals, because adding new view fields otherwise breaks every test fixture.

### Path Segment Percent-Decoding

In restJson1 dispatch, ARN-bearing path parameters arrive percent-encoded ( `arn%3Aaws%3Abraket%3A...%2Fmy-job` ). `winterbaume_core::protocol::common::percent_decode` is the canonical decoder. Splitting on `/` without decoding will silently miss lookups by ARN; reference cases: braket, chimesdkmeetings, cognitosync.

### Smithy Timestamp Default is Epoch-Seconds Float

For restJson1 services, the smithy `Timestamp` shape with no explicit `@timestampFormat` defaults to **epoch seconds as `f64`**, not RFC 3339 strings. The SDK rejects RFC 3339 strings with `only Infinity, -Infinity, NaN can represent a float as a string but found 2026-04-27T...`. Use `chrono::Utc::now().timestamp() as f64`. Other services like cognitosync also default to epoch seconds, while some have no timestamps at all -- check on a case-by-case basis. Reference case: codegurusecurity.

### Value-Backed State for Deeply Nested Shape Graphs

Services with 30+ nested types ( braket, chimesdkmeetings, applicationsignals, amplifyuibuilder, trustedadvisor, taxsettings, ssm-quicksetup, controlcatalog, application-discovery-service ) store state as `serde_json::Value` keyed by the natural identifier ( ARN / name / tuple ). Handlers compose response JSON directly via `MockResponse::rest_json` rather than using the auto-generated wire helpers, since the wire helpers would just be `serde_json::to_string(result)` over typed structs anyway. The trade-off: handlers do no semantic validation on nested blocks; the wire shape is trusted. Tag this as a deliberate choice in the crate README so quality-gate passes do not flag it as missing typed state.

### Lean Implementation: Skip wire/model Codegen for Tiny Services

Single-operation services or services with no reusable typed structs ( connectcontactlens, costoptimizationhub, billing ) skip wire/model codegen entirely: handlers use `serde_json::json!` literals and `MockResponse::json` / `MockResponse::rest_json`. Cargo.toml omits `chrono` and `uuid` if not needed. Inline `chrono_iso_now()` style helpers convert `SystemTime` to the format the protocol needs. **No `discover.rs::SERVICE_MAP` entry is required** when wire/model codegen is skipped.

### Typed-Record State for Per-Record Bookkeeping

Inverse pattern: services that need typed bookkeeping per record ( cognitosync's UpdateRecords needs sync_count / last_modified per `RecordEntry`; rbin needs ETag versioning ) use typed `*Record` structs in state. Reach for typed state when at least one operation reads or mutates a sub-field of a record after creation; otherwise prefer Value-backed state.

### SDK Accessor Type Variability

The recurring trap across this batch: Smithy `@required` collapses SDK getters to `&T`, optional fields stay `Option<&T>`, but the same struct can mix both. Don't infer; let the compiler tell you. Specific patterns observed:

- `assert_eq!(report.analysis_report_id(), id)` ( `&str`, required ) but `assert_eq!(other.identifier(), Some("db-ABC"))` ( `Option<&str>`, optional ) on the same struct.
- Status enums ( `RoutingControlState`, `EnvironmentStatus`, `JobPrimaryStatus`, `CancellationStatus`, `BulkPublishStatus`, `ControlBehavior`, `AccountPlanType`, `AccountPlanStatus`, `CurrencyCode` ) return `&Enum` directly with `.as_str()`, not `Option<&Enum>`.
- `MonetaryAmount.amount()` returns `f64` directly when required.
- `BatchDeleteConfigurationTaskStatus::Initializing` etc. are typed enum variants, not strings.

### SDK Builder `.build()` Return Variability

Some `Builder.build()` calls return `Self` directly ( no `Result` ); others return `Result<Self, BuildError>` only when at least one `@required` field could be missing. Examples:

- `LockConfiguration::builder().build()` -> `Self` directly.
- `UnlockDelay::builder().build()` -> `Result`.
- `DestinationConfigurations::builder().build()` -> `Self` directly ( all fields default-able ).
- `CodeReviewType::builder().build()` -> `Self` directly.
- `RepositoryHeadSourceCodeType::builder().build()` -> `Result` ( has `@required` fields ).

Test code that defensively `.unwrap()`s every `.build()` will fail to compile against the `Self`-returning variants.

### Tag Wire Shape Inconsistency

Tags appear in three different wire shapes across services:

- `Vec<Tag {Key, Value}>` array ( pi, rbin, billing, snowdevicemanagement, cloud9 ).
- `HashMap<String, String>` object map ( many older awsJson services ).
- Hybrid: handler must accept both ( rbin's first-cut bug -- `parse_tag_map` only accepted the object form, so SDK's array body fell through to "Tags is required" ).

When implementing, accept both forms and normalise to a `HashMap<String, String>` internally.

### URL / Tag Routing Variance

UntagResource carries tag keys differently per service:

- `@httpQuery("tagKeys")` repeated query parameter ( `?tagKeys=k1&tagKeys=k2` ) -- codestarnotifications, codegurureviewer, codegurusecurity, simspaceweaver, snowdevicemanagement, pcaconnectorscep, ssm-quicksetup. `parse_query_string` returns a HashMap which collapses duplicates -- read the raw URI and split on `&` to recover each repeat.
- `@httpQuery("TagKeys")` ( capital T, otherwise same shape ) -- pcaconnectorscep.
- JSON body field `tagKeys: [...]` -- many awsJson services.
- `Resource` body field that contains the ARN -- mediastore-style awsJson.

URL-encoded ARN in path lookup ( s3outposts, mediastore ) requires resolving the ARN by suffix-matching against stored keys, since the SDK encodes `arn:aws:.../foo` as `arn%3Aaws%3A.../foo` in the path.

### Per-Service Endpoint Hostname Patterns

Several services have non-`<service>.{region}.amazonaws.com` hostnames that need explicit URL-pattern handling:

- `sms-voice.pinpoint.{region}.amazonaws.com` ( pinpoint-sms-voice ).
- `api.pricing.{region}.amazonaws.com` ( pricing -- two dots before region ).
- `meetings-chime.{region}.amazonaws.com` ( chimesdkmeetings ).
- `{cluster-prefix}.route53-recovery-cluster.{region}.amazonaws.com` ( route53-recovery-cluster -- optional subdomain ).
- `contact-lens.{region}.amazonaws.com` ( connectcontactlens ).
- `{accountId}.cloudfront-kvs.global.api.aws` ( cloudfront-keyvaluestore -- account-id-prefixed, no region ).
- `search-{name}-{id}.{region}.cloudsearch.amazonaws.com` ( cloudsearch-domain -- per-domain hostname ).
- Both `billing.{region}.api.aws` and `billing.{region}.amazonaws.com` ( billing -- modern + legacy endpoint families ).

Tests against per-cluster / per-domain endpoints must call `.endpoint_url("...")` because the SDK's resolver expects a control-plane lookup result. Without it, the SDK returns NoSuchEndpoint or strips the cluster prefix.

### HTTP 202 for Create Operations

Some services ( pca-connector-scep CreateConnector / CreateChallenge ) return HTTP 202 ( Accepted ), not 200. The smithy `@http(code: 202)` trait controls this. Set `resp.status = 202` after the wire serializer ( which defaults to 200 ).

### Cascading Delete

Most stateful services have parent-child resource hierarchies. Convention: deleting a parent removes dependents. Examples:

- DeleteDataIntegration blocks if associations exist ( returns `ResourceQuotaExceededException` ).
- DeleteEnvironment cascades memberships ( cloud9 ).
- DeleteHypervisor cascades virtual machines ( backup-gateway ).
- DeleteApplication cascades components / log patterns / workloads ( applicationinsights ).
- DeleteSimulation cascades apps ( simspaceweaver ).
- DeleteConnector cascades challenges ( pca-connector-scep ).

Choose between blocking ( referential integrity ) and cascading per the AWS-documented behaviour; mock honesty here is what makes tests realistic.

### Auto-Vivification / Lazy Seeding

Resources that cannot be created via the data-plane API ( route53-recovery-cluster routing controls, snow-device-management devices, controlcatalog catalogue, s3outposts outposts, cloudfront-keyvaluestore stores ) lazily seed sensible defaults on first read so callers do not need a parallel control-plane mock. `ensure_seeded` is called from both the operation handler and the `From<*StateView>` restore path so the seeding holds across snapshot round-trips.

### URL Method Mismatches Worth Calling Out

A few methods do not match the verb the URL implies:

- `ListQuickSetupTypes` is `GET /listQuickSetupTypes` despite living at a verb-style path ( ssm-quicksetup ).
- `DescribeTask` is `POST /task/{taskId}` despite being read-only ( snowdevicemanagement ).
- `GetAccountConfiguration` is `GET /accountConfiguration/get` ( codegurusecurity -- the trailing `/get` is part of the path ).
- `UpdateAccountConfiguration` is `PUT /updateAccountConfiguration` ( codegurusecurity ).
- `CreateConfigurationSetEventDestination` is POST and Update is PUT ( pinpoint-sms-voice ).

Always check the smithy model's `@http(method: ...)` trait rather than guessing from the path shape.

### Operation-Query-String Routing

Several services share a path and method but differ on `?operation=<value>` query selector. Dispatch is `(method, segments, operation_query_value)` rather than `(method, segments)`. Reference: chimesdkmeetings ( `POST /meetings` for CreateMeeting vs `POST /meetings?operation=create-attendees` for CreateMeetingWithAttendees; `POST /tags?operation=tag-resource` vs `?operation=untag-resource`; `POST /meetings/{id}/transcription?operation=start` vs `?operation=stop` ).

### Singular vs Plural Path Inconsistencies

RegisterDevice's path is `/identitypools/{pool}/identity/{id}/device` ( **singular** `identity` ) while every other identity-scoped operation uses `/identitypools/{pool}/identities/{id}/...` ( plural ) -- cognitosync. Easy to miss; reading the smithy `@http` trait line is the only reliable source.

### Smithy Snake-Case-ification of Acronyms

`ResourceARN` becomes `resource_a_r_n` ( underscore between adjacent capitals ); `EC2` becomes `e_c2`; `SNSNotificationArn` becomes `s_n_s_notification_arn`; `CWEMonitorEnabled` becomes `c_w_e_monitor_enabled`. Three uppercase letters in a row each get their own underscore. Recurring confusion -- always check the generated codegen output for the canonical Rust ident.

### Discriminated-Union (oneOf) Builders

`ExportSpecification` ( backupsearch ), `Command` ( snow-device-management `Reboot`/`Unlock` ), `AssociateRepositoryRequest::Repository` ( codegurureviewer ) are smithy `union`s. The SDK exposes them as Rust `enum`s with no `::builder()` method; construction is via the variant directly ( `ExportSpecification::S3ExportSpecification(S3ExportSpecification::builder()...build())` ). Validation accepts any non-empty object; the union's repository-type discriminator emits the corresponding `ProviderType`.

### Reserved-Identifier Field Names

Where a wire-level field name clashes with a Rust keyword ( `type` ), the codegen-generated struct uses `r#type` and the domain type stores it as a renamed scalar ( `action_type`, `r#type` -> `kind`, etc. ) and converts at the boundary. Reference: bcmrecommendedactions `RecommendedAction.type` -> `action_type`.

### Smithy Output Naming Quirks

- Some wire helpers take `&Output {}` for empty outputs ( `serialize_delete_tax_registration_response(&DeleteTaxRegistrationResponse {})` ); others take no arguments. The codegen variance comes from whether the smithy output is `smithy.api#Unit` or an empty named structure.
- Smithy uses `Result` ( as in `CreateSlackChannelConfigurationResult` ) instead of `Response` for some output type names; the wire helpers are still named `serialize_*_response` to match repo convention.
- Field names sometimes include awkward nominalisations: `errors_and_failed_entries_zip` ( applicationdiscoveryservice's `ImportTask`, not `error_url` despite docs naming ); `account_recommendation_lifecycle_summaries` ( trustedadvisor `ListOrganizationRecommendationAccountsResponse` ); `change_events` ( applicationsignals `ListEntityEventsOutput`, not `entity_events` ).
