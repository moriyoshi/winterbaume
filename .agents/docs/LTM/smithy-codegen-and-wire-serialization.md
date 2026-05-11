# Smithy Codegen and Wire Serialization

## Summary

Winterbaume's serialisation story evolved from hand-built XML and JSON strings into generated `model.rs` and `wire.rs` modules backed by Smithy models. The durable knowledge is not only that codegen exists, but which protocol-specific gaps had to be fixed for it to be trustworthy: resource-level operation discovery, timestamp-format handling, awsQuery and ec2Query response shaping, request deserialisers across XML, query, JSON, and rpc-v2-cbor protocols, generator-owned lint or rustfmt hygiene, and multi-protocol support for CloudWatch-class services.

## Key Facts

- `smithy-codegen` is core infrastructure. Durable fixes belong in the generator, not in hand-edited generated files.
- Split output is the normal mode: `model.rs` holds generated structs and `wire.rs` holds serialisers, deserialisers, and helper glue.
- awsQuery, restXml, ec2Query, JSON protocols, and now rpc-v2-cbor each have protocol-specific generation rules.
- Request deserialisation is now part of the generator story, not only response serialisation.
- `generate_request_deserializers_for_protocol()` now emits deserialisers for `AwsJson1_0`, `AwsJson1_1`, and `RestJson1` as well as `RestXml`, `AwsQuery`, `Ec2Query`, and `RpcV2Cbor`.
- Generated files should be rustfmt-clean and carry their own narrow lint allowances.
- The generated `#![allow(...)]` banner is part of the generator contract and must track the structural patterns the generator emits, including later Clippy lints such as `single_match` and `let_and_return`.
- CloudWatch no longer depends on fully hand-rolled CBOR helpers; typed rpc-v2-cbor scalar and list timestamp serialisation now uses generated serde helpers and the existing CBOR Tag 1 sentinel path.
- Services that advertise multiple protocols may need secondary-protocol support even when `smithy-codegen` picks a different primary protocol. CloudWatch is the reference case: Terraform uses awsQuery while the Smithy priority path chose rpc-v2-cbor.
- Additional-protocol awsQuery request deserialisers are now generated for services such as CloudWatch, including nested struct deserialisers and non-XML list/timestamp handling.
- The 2026-05 request-deserialiser adoption sweep moved 166 / 168 service crates to `body.get = 0`; the remaining two are intentional or deferred hybrids: API Gateway PATCH-style flat-scalar fallbacks and CloudWatch multi-protocol awsQuery + rpc-v2-cbor request parsing.
- The current `smithy-codegen gen-serializers` CLI takes a service slug, not an `sdk-models/<dir>` path. Use `list-services` to find the slug and model directory.

## Details

### Generator-First Maintenance Rule

The main durable rule is operational:

- if a generated file says "Do not edit manually", do not patch it as the lasting fix
- patch `tools/smithy-codegen`
- rebuild the generator
- regenerate the affected crate

This matters because the same failure mode usually appears in more than one crate.

### Current CLI Contract

The generator resolves service slugs against `vendor/api-models-aws/models` by default. Documentation and agent workflows should use:

```bash
./.agents/bin/cargo.sh run -p smithy-codegen -- list-services
./.agents/bin/cargo.sh run -p smithy-codegen -- gen-serializers <service-slug>
```

`list-services` prints two whitespace-separated columns:

```text
<crate-suffix>  <model-dir>
```

The first column is the slug accepted by `gen-serializers`; the second is the Smithy model directory. They are not always the same, and aliases may exist. For example, `elbv2` is an alias slug, but the workspace crate is `winterbaume-elasticloadbalancingv2`.

There is no `gen-serializers --all` flag. A full sweep is a shell loop over `list-services | awk '{print $1}'`, with normal care for aliases, generated-file blast radius, and per-crate validation. Do not document `sdk-models/`; that directory does not exist in this repo.

### Response Serialisation Evolution

The main codegen milestones were:

- protocol framework and initial serializer generation
- split `model.rs` plus `wire.rs` output
- recursive resource-operation discovery
- timestamp-format handling from protocol defaults plus member or shape overrides
- awsQuery and ec2Query response-shaping fixes
- generated Rust naming and lint hygiene

The durable module pattern is:

- `model.rs`: generated Smithy-derived types
- `wire.rs`: generated serialisers, deserialisers, and protocol helpers
- handlers build generated types or convert domain types into generated types

### XML and Query Protocol Lessons

The important XML and query-protocol lessons are:

- XML list wrappers must use the real wire member name, not only serde rename metadata
- awsQuery and ec2Query are similar but not wire-compatible
- generated helper wrappers are preferable to repeated hand-written XML fragments
- shared output shapes across multiple operations require careful outer-wrapper handling

This later surfaced clearly in RDS:

- success responses were migrated from hand-written XML to generated wire serialisers
- request parsing moved from manual query-string helpers to generated awsQuery request deserialisers
- shared output-shape wrapper issues had to be handled in the generator, not in crate-local code

### Request Deserialiser Generation

The later generator work made request deserialisation a first-class feature.

Durable examples:

- restXml request deserialisers reduced manual XML parsing in services such as S3 and Route53
- awsQuery request deserialisers later let RDS drop hand-written helpers such as `extract_list` and `extract_rds_tags`

The durable rule is that request parsing is generator-owned protocol logic when the Smithy model already describes the request shape.

The old request-deserialiser gap for JSON-shaped protocols is closed. `generate_request_deserializers_for_protocol()` emits:

- `generate_aws_json_request_deserializer(out, op_name, input_shape, _model)`, with signature `pub fn deserialize_<op>_request(body: &[u8]) -> Result<<Input>, String>`.
- `generate_rest_json_request_deserializer(out, op_name, input_shape, model)`, with signature matching the restXml shape: `&MockRequest`, labels, and query parameters.

The JSON generator work also fixed three side effects that matter outside the pilot crates:

- `binding_value_expr_for_member` no longer applies the XML-list wrapper rule to JSON lists; JSON list bindings emit raw `Vec<T>` while XML keeps `Wrapper { items: Vec<T> }`.
- JSON-protocol timestamp httpLabel / httpQuery / httpHeader bindings now try epoch-seconds parsing first and then RFC 3339 fallback through `chrono::DateTime::parse_from_rfc3339`.
- JSON-protocol input shapes keep `@httpLabel` and `@httpQuery` members so request deserialisers have fields to bind into.

Even protocols with generated request deserialisers can carry adoption debt. The 2026-05 adoption sweep made that distinction explicit: first regenerate stale `wire.rs`, then migrate handlers that still hand-extract request fields even though `wire::deserialize_<op>_request(...)` exists.

### Request-Deserialiser Adoption Sweep ( 2026-05 )

The multi-day request-deserialiser sweep replaced hand-extraction of request fields (`body.get(...)`, `params.get(...)`, `quick_xml::de::from_str(body)`) with codegen-emitted `wire::deserialize_<op>_request(...)` calls across nearly the whole service fleet.

Phase pattern:

1. Extend the generator for JSON protocols.
2. Migrate an awsQuery reference (`winterbaume-ses`), an awsJson reference (`winterbaume-kms`), and a restJson1 reference (`winterbaume-lambda` after dispatch refactor).
3. Regenerate stale crates whose `wire.rs` still contained response serialisers only.
4. Sweep awsQuery, awsJson, and restJson1 handler surfaces in batches.
5. Clean bridge patterns that technically deserialised typed input but immediately re-serialised it to `serde_json::Value`.

Reference templates:

| Protocol | Reference pattern |
|----------|-------------------|
| awsJson | `dispatch()` captures `let body_bytes: &[u8] = &request.body`; handlers call `wire::deserialize_<op>_request(body_bytes)?`. KMS is the reference. |
| awsQuery | Keep dispatch-only `Action` reads, but use typed generated input for operation parameters. SES and RDS are the references. |
| restJson1 | Thread `&MockRequest`, labels, and query through dispatch so handlers can call the generated deserialiser. Lambda became the canonical template. |
| DynamoDB data plane | Convert `model::AttributeValue` into the crate's typed `types::AttributeValue` before reaching expression, backend, or PartiQL layers. |

Final adoption state after the 2026-05-05 cleanup: 166 / 168 crates have `body.get = 0`. The two remaining residuals are deliberately different from ordinary adoption debt:

- `winterbaume-apigateway`: PATCH-style update operations accept flat scalar fields that Smithy does not model. Handlers call the wire deserialiser first, then inspect a re-parsed body for those out-of-model flat scalar fallbacks.
- `winterbaume-cloudwatch`: multi-protocol awsQuery + rpc-v2-cbor request handling chooses protocol from URL shape and needs its own migration recipe.

`winterbaume-core::body_has_top_level_field(body: &[u8], key: &str) -> bool` centralises the validation-only raw-body peek needed when a Smithy default-equipped non-Option struct collapses the distinction between "field absent" and "empty object". This preserved 400 ValidationException semantics in EFS, S3 Tables, and Kinesis Video Archived Media without keeping local `body.get` parsing.

### Stale Wire/Model Regeneration Sweep

The adoption sweep found many crates whose `wire.rs` predated request deserialisers and contained only response serialisers. A mass regeneration pass covered 151 stale `winterbaume-*` crates; 145 regenerated successfully after generator and service-map fixes, while 6 were rolled back because the newer `model.rs` introduced handler/state field drift.

Generator and discovery fixes from that sweep:

- `binding_value_expr_for_member` now chooses f64-vs-String timestamp http-binding parsing based on the resolved `timestamp_format`.
- `winterbaume-signer` gained the missing `chrono` workspace dependency needed by the f64-with-RFC-3339 fallback path.
- `tools/smithy-codegen/src/discover.rs::SERVICE_MAP` gained aliases for `cloudwatchlogs`, `cognitoidentityprovider`, `databasemigration`, and `directory`, plus earlier direct entries for several new crate suffixes.

Broken regen artefacts for the six follow-ups live under `.agents-workspace/tmp/regen-broken/<crate>-{wire,model}.rs` and `.agents-workspace/tmp/regen-broken/<crate>-clippy.log`. The durable follow-up path is: regenerate the crate, update handler/state code for the new fields ( often `None` or default values for new optional fields ), then run the per-crate clippy, fmt, and test gate.

The remaining registry gap is `glue`: the fallback model-dir path works, but `tools/smithy-codegen/src/discover.rs::SERVICE_MAP` should include an explicit `glue` entry so `list-services` reports it.

### Generated-Code Hygiene

The quality-gate work reinforced several generator-owned hygiene rules:

- generated files should carry the right narrow lint allowances
- split-path `wire.rs` files should include `#![allow(dead_code)]` where fully stubbed crates need it
- generated output should already pass rustfmt
- recurring warnings across multiple crates belong in the generator, not in workspace-wide lint suppression
- when Rust or Clippy starts rejecting a generator-emitted structural pattern, update the generator banner first and then bulk-refresh already-generated files so CI and local checks agree immediately

This includes formatting-related fixes such as:

- multi-line wrapper generation that rustfmt accepts
- normalised trailing-newline handling
- generated helper formatting that does not cause repeated crate-local fmt churn

Recent generator-owned lint examples included:

- `clippy::single_match` on generated path-label dispatch matches in request deserialisers
- `clippy::let_and_return` on generated response builders that bind and immediately return a `MockResponse`

The durable rule is to suppress those at the generated-module boundary when the emitted pattern is intentional, rather than rewriting every generated call site by hand.

### rpc-v2-cbor Support

The 2026-04-14 work added first-class rpc-v2-cbor support to `smithy-codegen`.

Durable outcomes:

- `Protocol::RpcV2Cbor` is now recognised by the model and generation pipeline
- generator support now emits CBOR-aware helpers and request or response scaffolding
- `MockResponse::cbor()` lives in core so protocol crates can return proper CBOR responses cleanly

CloudWatch became the reference consumer:

- the generator now supports its protocol
- the crate uses generated CBOR helpers in `wire.rs`
- the repo no longer depends on entirely hand-written CBOR conversion utilities there

The remaining durable caveat is no longer response timestamp support. The generator now covers scalar and list timestamp fields through model-level serde helpers that emit a private sentinel recognised by `wire::json_to_cbor()`. The remaining protocol work is request-side and secondary-protocol support.

### list-of-timestamp CBOR Codegen Gap and Resolution

A deep-dive in 2026-04-22 pinpointed exactly why CloudWatch cannot adopt the generated `wire::serialize_*` path. The root issue is in three layers:

1. `resolved_type_to_rust_serde()` maps `Timestamp` (epoch_seconds) to `f64` and `List(Timestamp)` to `Vec<f64>`.
2. `serde_json::to_value` on `Vec<f64>` produces plain JSON numbers with no object sentinel.
3. `json_to_cbor()` only checks for the `{"__cbor_epoch_seconds": N}` sentinel on JSON **objects**, so array elements pass through as bare CBOR floats without CBOR Tag 1.

The entire CloudWatch crate previously bypassed generated serialisers and used one of two manual paths:
- Path B: `ciborium::Value::Tag(1, ...)` construction ( used by `handle_get_metric_data` ).
- Path C: `cbor_timestamp()` sentinel helper inside `json!` macro ( used by all other CloudWatch handlers ).

The list-of-timestamp gap was therefore not just a `GetMetricData` issue. It was the primary reason the whole CloudWatch crate had not migrated to the typed path.

Three fix approaches were initially evaluated: (1) per-field `serialize_with`, (2) newtype wrapper `CborTimestampList`, (3) direct `ciborium::Value` construction in codegen. The implementation followed the per-field `serialize_with` direction while keeping handler code on generated output structs.

The 2026-04-24 generator pass closed this gap:

- generated rpc-v2-cbor model modules now emit `serialize_cbor_timestamp`, `serialize_cbor_timestamp_opt`, `serialize_cbor_timestamp_vec`, and `serialize_cbor_timestamp_vec_opt`
- helpers use a private `CborTimestampSentinel` struct deriving `Serialize`, not `serde_json::json!()`, so model modules avoid an unnecessary helper-level JSON macro dependency
- optional and required scalar timestamp fields, and optional and required list-of-timestamp fields, get the appropriate `#[serde(serialize_with = "...")]`
- generated comments document the intentional output-only asymmetry: request deserialisers convert CBOR Tag 1 into plain JSON epoch numbers before serde sees the values
- shape names are sorted before model emission so regenerated model output is deterministic

CloudWatch `GetMetricData` and `ListDashboards` now build generated output structs and return `wire::serialize_get_metric_data_response` / `wire::serialize_list_dashboards_response`. The old manual `ciborium::Value` maps and `cbor_timestamp()` helper are no longer the reference path.

### QG §7 Wire Serialiser Migration (2026-04-22)

A batch sweep migrated four QG §7 non-compliant crates to use generated `wire::serialize_*` calls:

| Crate | Protocol | Notes |
|-------|----------|-------|
| `sagemakerruntime` | REST-JSON | 3 handlers migrated |
| `scheduler` | REST-JSON | 12 handlers; `schedule_to_json()` → `schedule_to_get_schedule_output()` |
| `neptune` | Query/XML | 70 handlers; 16 new domain-to-wire converter functions |
| `sesv2` | REST-JSON | ~80 handlers; `tags_to_json()` → `tags_to_wire()` |

`route53` and `s3control` were already compliant. `cloudwatch` was the last known blocker at the time of this batch, and the 2026-04-24 timestamp-helper work moved its timestamp-sensitive success responses onto generated serialisers.

### Hand-Crafted XML Detection

The 2026-04-24 XML sweep found that the previous QG §7 detection patterns were too narrow. The old `rg 'xml_response\('` check missed literal `<?xml` response strings built directly with `format!(...)` or hard-coded XML.

The durable detection command is now:

```bash
git grep -n '<?xml' -- 'crates/winterbaume-*/src/handlers.rs'
```

Permitted XML exceptions are narrow:

- error-response shapers such as `s3_error_response`, `ec2_error_response`, `cloudfront_error_response`, and `xml_error_response`
- dynamic-root EC2 no-op helpers where one implementation receives a `response_name: &str` and serves multiple operations whose root element varies
- payload data values that happen to be XML, such as WorkSpaces Web SAML metadata, when they are still returned through generated serializers

S3 and EC2 were cleaned up by replacing avoidable success-response XML with generated wire serializers for list, describe, get, and no-op paths where typed output shapes exist.

### Multi-Protocol CloudWatch Support

CloudWatch exposed a separate generator and handler lesson: Smithy primary-protocol choice is not enough for provider compatibility. The CloudWatch model declares `rpcv2Cbor`, `awsJson1_0`, and `awsQuery`; codegen picked rpc-v2-cbor, but the Terraform AWS provider v5.100.0 used awsQuery.

Durable changes:

- `tools/smithy-codegen/src/model.rs` now tracks `additional_protocols: Vec<Protocol>` in addition to the primary protocol
- `gen_serializers.rs` can emit additional-protocol response serializers with suffixes such as `_query` and `_json`
- XML adapters for CBOR responses must preserve protocol-specific semantic tags. CloudWatch's awsQuery compatibility path originally decoded CBOR Tag 1 timestamps with `wire::cbor_to_json`, which stripped the tag and emitted plain epoch numbers in XML; AWS SDK awsQuery readers expect RFC 3339 strings. The durable fix is a sibling `cbor_to_json_for_xml` path that renders Tag 1 as RFC 3339 with millisecond precision while leaving other tags on the normal conversion path.
- helper imports and protocol helpers are deduplicated while generating multi-protocol output
- CloudWatch dispatch now detects query-protocol requests, parses form-encoded `Action=...` bodies, converts CBOR response structs back into awsQuery XML envelopes, and shapes query-protocol errors correctly

The follow-up additional-protocol generator pass closed the request side for CloudWatch-class awsQuery compatibility:

- the additional-protocol deserialiser loop is enabled
- nested struct deserialisers such as `deserialize_tag_from_query` are emitted when awsQuery is additional rather than primary
- list fields choose wrapper structs for XML-primary models and plain `Vec<T>` for non-XML models
- non-XML query timestamp fields parse as `f64` instead of string timestamps

Regenerating CloudWatch now emits awsQuery deserialisers alongside primary rpc-v2-cbor code. SNS also regenerated cleanly.

The later protocol audit found only three multi-protocol services:

| Service | Protocols | Durable conclusion |
|---------|-----------|--------------------|
| CloudWatch | rpc-v2-cbor, awsQuery, awsJson | Dual-protocol handlers plus generated secondary awsQuery deserialisers are required and implemented |
| SQS | awsJson plus `awsQueryCompatible` trait | The trait affects query-style error headers, not actual awsQuery request parsing |
| Arc-Region-Switch | rpc-v2-cbor plus awsJson | No Winterbaume crate exists yet |

### restJson1 `@httpPayload` Output Framing ( 2026-04-28 )

Per restJson1, when an output member is annotated `@httpPayload`, the body of the response *is* that member's value, not the wrapping struct. The aws-sdk-rust deserialiser reflects this:

```rust
output = output.set_entity(crate::protocol_serde::shape_create_component_output::de_entity_payload(_response_body)?);
// de_entity_payload calls de_component_payload(body) directly
```

`de_component_payload` then runs `aws_smithy_json` over the raw body and expects the inner shape's field names ( `appId`, `id`, `name`, … ) at the top level. Unknown keys are silently skipped, and missing required fields are filled by `*_correct_errors` with `Default::default()` — no hard error, just empty values.

The pre-fix `generate_rest_json_serializer` ignored `@httpPayload` and always serialised the full response struct: `{"entity": {...component...}}`. The SDK saw `entity` as an unknown top-level key, skipped it, and produced an all-default Component. Symptom: `create.entity().unwrap()` returned `Some(Component { id: "", name: "", ... })` and the integration test `assertion failed: !id.is_empty()` panicked. The pre-existing triage suggested making `@required` fields non-optional, but that does not actually solve the bug — the SDK's `*_correct_errors` already fills missing required fields with defaults, so non-optional model fields would not change what the client sees.

Fix: detect a struct-typed `@httpPayload` output member in `generate_rest_json_serializer` and emit only that member's value as the body. The `Option<T>` vs `T` framing is preserved ( the existing model emission is unchanged ):

```rust
if let Some((field_name, required)) = &payload_struct_field {
    if *required {
        out.push_str(&format!(
            "    let body = serde_json::to_string(&result.{field_name})\n        .unwrap_or_else(|_| \"{{}}\".to_string());\n"
        ));
    } else {
        out.push_str(&format!("    let body = match &result.{field_name} {{\n"));
        out.push_str("        Some(v) => serde_json::to_string(v).unwrap_or_else(|_| \"{}\".to_string()),\n");
        out.push_str("        None => String::new(),\n");
        out.push_str("    };\n");
    }
} else {
    // unchanged: serialise the whole response struct
}
```

Non-struct payloads ( Blob, String, Document ) keep the previous behaviour — they are uncommon and out of scope for this fix.

#### Workspace-wide regen sweep

The generator change affected every restJson1 service with `@httpPayload` output members. A scan of `vendor/api-models-aws/models/` for services using both `restJson1` AND `smithy.api#httpPayload` returned 48 candidates, of which 15 are implemented in winterbaume. Each was regenerated via `./.agents/bin/cargo.sh run -p smithy-codegen -- gen-serializers <model-dir> --output ... --model-output ...`. Diff classification:

| Group | Crates | Net wire.rs lines |
|---|---|---|
| Functional fix landed ( silent SDK-visible bug closed ) | bedrock ( +3 ), glacier ( +10 ), iotdataplane ( +2 ), kinesisvideoarchivedmedia ( +2 ), lambda ( +5 ), mediastoredata ( +2 ), pinpoint ( +330 ), sagemakerruntime ( +4 ), amplifyuibuilder ( +44 ) | ~400 net |
| `model.rs` reorder only ( no functional change ) | apigateway, appconfig, appsync, codeartifact, iot, medialive, polly | 0 |

Pinpoint was the standout: ~50+ Create / Get / Update operations, each previously emitting `{"<key>": {...}}` instead of just the inner payload struct. The framing fix flips them to extract `result.<payload_member>` directly.

Net effect: 9 winterbaume restJson1 crates that previously shipped silent client-visible bugs ( SDK clients receiving default-filled empty payload structs because the body was wrapped ) now produce correct wire format. 33 of the 48 vendor restJson1 + `@httpPayload` services are not yet implemented in winterbaume; they will pick up the fix automatically when scaffolded via `add-service`.

#### restXml audit ( 2026-04-28 )

The restXml generator at `gen_serializers.rs:972-1023` sidesteps the wrap-around-payload issue by emitting a function signature that takes `&PayloadType` directly ( not the wrapping response struct ). Different approach from restJson1 but functionally correct — no parallel bug exists for restXml services.

### awsQuery Nested-Struct Deserialiser ( 2026-04-28 )

`generate_aws_query_member_deser` in `tools/smithy-codegen/src/gen_serializers.rs` lacked a `ResolvedType::Structure(_)` arm. Any awsQuery struct field whose member is itself a nested struct generated an empty deserialiser body. `LoadBalancerAttributes` has 4 such nested struct fields ( AccessLog, ConnectionDraining, ConnectionSettings, CrossZoneLoadBalancing ) all silently dropped, which broke `ModifyLoadBalancerAttributes`.

Fix: added the missing `ResolvedType::Structure(s)` arm at `gen_serializers.rs:1700-1721` emitting `deserialize_<name>_from_query(params, &sub_prefix)` with `sub_prefix = format!("{prefix}.{query_key}")`. Regenerated `winterbaume-elasticloadbalancing/src/wire.rs` ( +21 net lines, the 4 missing `if let Some(val) = deserialize_*_from_query(...)` blocks ); removed the inline workaround in `handle_modify_load_balancer_attributes` and the now-unused `extract_member_list` helper.

### EC2 Generated Feature Gates

`winterbaume-ec2-generated` is not a normal generated crate. It must be regenerated with:

```bash
.agents/bin/cargo.sh run -p smithy-codegen -- gen-serializers ec2 \
    --output crates/winterbaume-ec2-generated/src/wire.rs \
    --model-output crates/winterbaume-ec2-generated/src/model.rs \
    --features-map tools/smithy-codegen/ec2-features.toml
```

The `--features-map` flag is load-bearing. Without it, regeneration strips the generated `#[cfg(feature = "...")]` attributes from `model.rs` and `wire.rs`, undoing the split-crate build-time work.

### Partial Adoption Still Matters

Even with broad codegen adoption, some handlers may still need local glue for:

- binary payloads
- protocol-specific error shaping
- SDK-specific timestamp quirks
- helper conversions between domain types and generated types

The durable rule is to distinguish legitimate boundary glue from avoidable hand-written success responses. RDS migration showed the difference clearly: most of its old manual XML was not special behaviour, only unported generator adoption debt.

## Files

- `tools/smithy-codegen/src/gen_serializers.rs` - main serializer and deserializer generation logic
- `tools/smithy-codegen/src/gen_rpcv2_cbor.rs` - rpc-v2-cbor-specific generation path
- `tools/smithy-codegen/src/model.rs` - protocol detection and model parsing
- `tools/smithy-codegen/src/main.rs` - CLI shape, generation entry points, and post-processing
- `tools/smithy-codegen/ec2-features.toml` - EC2 operation-to-feature mapping used during generated-crate regeneration
- `crates/winterbaume-rds/src/handlers.rs` - reference migration from manual XML and query parsing onto generated wire helpers
- `crates/winterbaume-cloudwatch/src/wire.rs` and `handlers.rs` - reference rpc-v2-cbor adoption path
- `crates/winterbaume-cloudwatch/src/model.rs` - reference generated timestamp serde helpers for rpc-v2-cbor outputs
- `crates/winterbaume-kms/src/handlers.rs` - reference awsJson request-deserialiser adoption path
- `crates/winterbaume-lambda/src/handlers.rs` - reference restJson1 threaded-dispatch adoption path
- `crates/winterbaume-core/src/protocol/json.rs` - owns `body_has_top_level_field` for validation-only raw-body presence checks

## Test Coverage

- RDS integration tests passed after the serializer and deserializer migration
- CloudWatch integration tests passed after rpc-v2-cbor generator support, timestamp-helper generation, secondary awsQuery response support, and wire-helper adoption
- CloudWatch and SNS tests passed after additional-protocol awsQuery request deserialiser generation was enabled
- generator-specific fixes were repeatedly validated through targeted crate regeneration plus crate-local build, fmt, and integration-test runs
- The 2026-05 request-deserialiser adoption sweep verified migrated crates with per-crate fmt, clippy, and focused integration tests; DynamoDB finished with 7 unit + 177 integration + 5 scenario tests passing, Cognito Identity Provider with 98 tests, and the small-residual cleanup with EFS 91, Kinesis Video Archived Media 28, S3 Tables 47, SESv2 88, Glue 147, and Greengrass 112 tests.

## Pitfalls

- Never treat generated files as the durable edit point.
- Do not leave protocol parsing or wrapper quirks in handlers when the Smithy model already describes them.
- Do not assume primary-protocol support is enough for Terraform compatibility when a service advertises multiple protocols.
- Do not treat `awsQueryCompatible` as proof that a JSON service accepts awsQuery request bodies.
- Do not assume protocol support is complete once requests decode and bytes emit; SDK-facing timestamp, wrapper, and secondary-protocol fidelity still matter.
- Do not paper over repeated generated-file warnings with broad workspace lint suppression.
- Do not confuse protocol-boundary error helpers with success-response generation debt.
- Do not assume JSON-protocol request parsing is handler-owned. Generated request deserialisers now exist for restJson1 and awsJson; remaining `body.get` reads need a documented hybrid reason or a migration plan.
- Do not treat the existence of generated request deserialisers as proof handlers use them. Survey adoption before declaring a protocol family migrated.
- Do not assume stale generated files are rare. If `wire.rs` lacks `deserialize_*_request`, regenerate before touching handlers and check whether the crate is one of the field-drift follow-ups.
- Do not copy `.is_empty()` handling between generated input structs without reading the exact field type. The same semantic field can be `String` in one request and `Option<String>` in another.
- Do not replace validation-only raw-body presence checks with typed defaults when the model field is a non-Option struct with `#[serde(default)]`; use `body_has_top_level_field` if absent-vs-empty semantics matter.
- Do not assume `@httpPayload` outputs work just because the model declares the trait. Pre-2026-04-28 generated serialisers ignored the trait and always emitted `{"<member>": value}` instead of just `value`; the SDK's `*_correct_errors` then filled missing required fields with defaults and the bug presented as "client receives empty struct". Test against `aws-sdk-rust` clients, not just the wire bytes.
- Do not regenerate one crate after a generator change and assume the fix has landed for the workspace. The fix only takes effect for the regenerated `wire.rs`; every other restJson1 / restXml / awsQuery crate that uses the same generator path still ships its old `wire.rs` until separately regenerated. Maintain a "regen sweep" task in TODO.md whenever a generator fix lands.
- Do not be alarmed by a `model.rs` reorder that accompanies a `wire.rs` regen. The generator's struct emission order is non-deterministic ( HashMap iteration ); reorders are functionally identical.
- Do not regenerate `winterbaume-ec2-generated` without the EC2 feature map.
