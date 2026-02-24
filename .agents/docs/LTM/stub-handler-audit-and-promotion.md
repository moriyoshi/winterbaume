# Stub Handler Audit and Promotion

## Summary

Winterbaume's later service work shifted from adding new crates to replacing hard-coded handler stubs with real state-backed behaviour. This document captures the stub taxonomy, the promotion patterns that turned tractable `STUB[no-state]` handlers into real implementations, the representative fixes across `acmpca`, `synthetics`, `apigateway`, `xray`, `guardduty`, and the larger eight-crate promotion wave that followed.

## Key Facts

- Not all stubs are equal. The audit taxonomy matters because it tells you whether to add state, build an engine, or defer intentionally.
- `STUB[no-state]` handlers are usually the best promotion targets because the missing work is durable state and handler wiring, not a new evaluator.
- Pagination placeholders and omitted empty collections count as stub debt too.
- `TODO.md` is the right place for unresolved stub work. Durable promotion patterns belong in LTM.
- Several surfaces that were previously treated as engine-only gaps have now moved forward: Bedrock flow validation and IAM simulation no longer belong on the same "future engine" list as before.

## Details

### Audit Taxonomy

The durable classification is:

- `STUB[no-state]` - the handler should read or mutate durable mock state but does not
- `STUB[no-engine]` - the operation needs a real evaluator, parser, validator, or processor
- `STUB[s3-integration]` - the operation depends on S3-backed content or statistics
- `STUB[org-integration]` - the operation depends on organisation-wide or multi-account state
- `STUB[no-telemetry]` - the surface is telemetry-driven and a no-op is acceptable
- `STUB[delegation-api]` - the operation depends on external delegation or root-only flows

This taxonomy is durable because it directs the next engineering step correctly.

### Small but Important Promotions

Three early fixes became reference examples:

- `acmpca ListCertificateAuthorities` moved from a fake continuation token to real ARN-cursor pagination
- `codebuild ListReportsForReportGroup` switched from omitting the field to returning an empty list shape
- Route53 `GetDNSSEC` stopped returning an empty key-signing-key list and now reflects actual state

The durable rule is that structurally correct empty behaviour is often the difference between a usable mock and a misleading one.

### Representative State-Backed Promotions

#### synthetics

`GetCanaryRuns` became state-backed by:

- introducing a run-record type
- storing run history per canary
- creating run entries on `start_canary`
- converting the stored records into generated wire types

#### apigateway

`ImportRestApi` still is not a full importer, but it no longer fabricates the same constant result every time. The durable compromise is:

- parse the OpenAPI body when possible
- extract meaningful metadata such as `info.title`
- leave the rest explicitly unimplemented rather than hiding it behind fake success

#### xray

X-Ray is the reference example for promoting a broader read surface off stubs:

- trace segments moved from raw strings to typed records
- trace summaries and derived flags moved into state
- encryption config and resource tagging became normal CRUD state
- trace-segment-destination and indexing-rule state were later added as durable follow-on work

#### guardduty

GuardDuty became the reference case for a larger `STUB[no-state]` promotion cluster:

- publishing destinations gained full CRUD state
- member-management and invitation flows were wired
- malware protection plans and scans moved onto stored records
- organisation configuration and related flows moved closer to real state-backed behaviour

The durable lesson is that many stub-heavy services already have most of the state logic. The missing piece is often handler wiring.

### Bulk Promotion Wave

The 2026-04-12 bulk promotion wave pushed several services forward in one coordinated pass.

The important promoted crates were:

- CloudFront
- NetworkFirewall
- Backup
- SecurityHub
- GuardDuty
- X-Ray
- SES v2
- SSM, through verification that its remaining stubs were appropriate and fully covered

The durable promotion pattern was consistent:

1. add missing state structs or maps
2. add CRUD or listing methods in `state.rs`
3. wire the handlers and dispatch arms
4. add integration tests for the promoted operations

### Hard-Coded Mock Sweep (2026-04-18)

A targeted sweep replaced hard-coded static/fake responses with input-seeded deterministic generation across three crates:

- **Rekognition** (6 handlers): `CompareFaces`, `DetectLabels`, `DetectText`, `DetectCustomLabels`, `GetFaceSearch`, `GetTextDetection` all now produce results seeded from input hashes. `VideoJob` stores a `seed: u64` at creation so results vary per job.
- **Polly** (1 handler): `SynthesizeSpeech` returns format-specific minimal valid audio bytes (silent MP3 frame, zero-filled PCM, OGG page header, or NDJSON speech marks) instead of a `FAKE_AUDIO_FOR:{text}` string.
- **Signer** (1 handler): `SignPayload` computes a deterministic 32-byte HMAC-like signature from profile name + payload using multi-round `DefaultHasher`, base64-encoded.

The durable pattern for mock analysis/detection stubs: use a hash of the input to seed a deterministic but varying result. This avoids both hard-coded values and non-deterministic randomness.

### Bulk STUB[no-state] Elimination (2026-04-19)

Every `STUB[no-state]` annotation was eliminated from the entire codebase (~400 handlers across 45+ crates) in one coordinated pass using 15 parallel agents.

Promotion categories:

1. **Full state promotion** (majority): New domain types in `types.rs`, state storage and CRUD in `state.rs`, view types with `#[serde(default)]` in `views.rs`, handlers rewired to real state.
2. **Dead code deletion** (EC2, 9 methods): Unreachable stubs already shadowed by `*_stateful` dispatch variants.
3. **Reclassification** (EC2 deprecated features, EMR catalogues): Changed to descriptive comments explaining why empty responses are intentional.
4. **Validation-only** (ecr, batch, codebuild, vpclattice, ivs): Added resource-existence checks to stubs that were otherwise correct.
5. **Comment removal** (19 small crates): Handlers already had proper behaviour; only the misleading tag was removed.

Notable new domain types added: Personalize (14 types), CloudFront (7), Backup (5), CodePipeline (5), SSM (7), NetworkFirewall (8), Neptune (4), SecurityHub (7), ECS (6), EMR (4), Inspector2 (4), and others.

Downstream ripple: DynamoDBStreams needed new error match arms, DynamoDB-Redis needed 5 new trait methods, Terraform converters needed `..Default::default()` additions, and CloudFormation tests needed updated view initialisers.

After this sweep, `grep -r 'STUB[no-state]' --include='*.rs'` returns zero matches.

### When to Defer

Some stubs should remain deferred until the right supporting subsystem exists.

Still-good examples:

- the remaining Macie2 `s3`-integration, engine, and member-management surfaces
- WAFv2 capacity parity work that still belongs in the calculator and handler integration layers

The important update is that not every former engine placeholder still belongs here:

- Bedrock flow validation now has dedicated parser and validator crates
- IAM policy simulation now has evaluator-backed handler logic
- Rekognition, Textract, Polly, KMS, and Signer analysis/crypto stubs have all been promoted to input-seeded or real implementations

## Files

- `.agents/docs/TODO.md` - open stub items and their current status
- `crates/winterbaume-acmpca/src/state.rs` and `handlers.rs` - pagination promotion example
- `crates/winterbaume-synthetics/src/types.rs`, `state.rs`, and `handlers.rs` - state-backed canary runs
- `crates/winterbaume-apigateway/src/handlers.rs` - partial but non-fabricated `ImportRestApi`
- `crates/winterbaume-xray/src/types.rs`, `state.rs`, `views.rs`, and `handlers.rs` - trace, tagging, encryption, and indexing promotions
- `crates/winterbaume-guardduty/src/types.rs`, `state.rs`, and `handlers.rs` - multi-surface promotion example
- `crates/winterbaume-route53/src/handlers.rs` - DNSSEC state-backed fix

## Test Coverage

- promoted handlers should gain targeted integration tests in their owning crate
- X-Ray gained broad integration coverage for trace, summary, and sampling paths
- GuardDuty gained new tests for publishing destinations, malware flows, invitations, and organisation behaviour
- the bulk promotion wave also used Terraform E2E activation where provider-facing behaviour was the right validation surface

## Pitfalls

- Do not keep placeholder pagination tokens once real state exists.
- Do not treat `None` and an empty list as interchangeable in generated AWS responses.
- Do not jump straight to a heavyweight engine if the missing behaviour is only durable state plus handler wiring.
- Do not leave real state methods unused behind stub handlers.
- Do not keep deferred work in LTM when it should live in `TODO.md`.
