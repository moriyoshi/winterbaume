# Cross-Service Integration and Engine Boundaries Synthesis

## Summary

Winterbaume's higher-fidelity behaviour now depends on three related boundaries: service-to-service contracts documented by AWS, shared-state boundaries between related protocol surfaces, and pure engine crates that keep semantic validation out of protocol handlers. Cross-service work should start from AWS-supported integration surfaces and real AWS state coupling, while engine-backed validation should stay in reusable crates that handlers invoke and shape into AWS responses. EC2 remains the special case for service-scale code generation, where feature-gated generated code is the boundary that keeps local iteration tractable.

## Included Documents

| Document | Focus |
|----------|-------|
| [aws-inter-service-integration-priorities.md](./aws-inter-service-integration-priorities.md) | AWS-documented AppSync, EventBridge, Lambda, Step Functions, API Gateway, Athena, and Glue integration seams |
| [rule-evaluator-and-validator-crates.md](./rule-evaluator-and-validator-crates.md) | IAM, Step Functions, WAFv2, and Bedrock pure evaluator or validator crate boundaries |
| [pluggable-backends-and-query-execution-synthesis.md](./pluggable-backends-and-query-execution-synthesis.md) | Backend and query-engine injection, DynamoDB PartiQL, DuckDB-backed Athena and Redshift Data, and Glue catalogue gaps |
| [runtime-state-and-service-infrastructure-synthesis.md](./runtime-state-and-service-infrastructure-synthesis.md) | StatefulService, StateView, backend-owned state, and derived-service state boundaries |
| [cross-service-state-coherence.md](./cross-service-state-coherence.md) | v1/v2 shared-backend rules, control-plane/data-plane validation, and intentionally separate service pairs |
| [ec2-crate-split-and-feature-gating.md](./ec2-crate-split-and-feature-gating.md) | EC2 generated-crate split, feature taxonomy, regeneration command, and partial-build pitfalls |

## Stable Knowledge

- Cross-service emulator behaviour should follow documented AWS integration contracts rather than invented mock-only coupling.
- AppSync, EventBridge, Lambda, Step Functions, API Gateway, DynamoDB Streams, SQS, SNS, Kinesis, Athena, and Glue form the highest-value near-term integration surface because the repo already has meaningful service coverage there.
- Step Functions service integrations have narrow action scopes. The optimised DynamoDB integration is limited to `GetItem`, `PutItem`, `UpdateItem`, and `DeleteItem`.
- AppSync and EventBridge are bidirectional in AWS documentation: AppSync can use EventBridge as a data source, and EventBridge rules can target public AppSync GraphQL mutations.
- Athena's `DataCatalogType::Glue` is not functional Glue integration by itself. Query execution needs a catalogue-resolution layer that consults `winterbaume-glue` state and passes table definitions into the query backend.
- DynamoDB Streams is a derived service. Table metadata and change records originate in DynamoDB, while iterator bookkeeping belongs in the streams service.
- Same AWS SDK slug is a strong signal that two Winterbaume crates may need shared state. SES and SESv2 are the current highest-priority pair: identities, configuration sets, templates, suppression lists, dedicated IP pools, and account-level settings should converge on one backend family by family.
- The `v2` suffix does not itself imply shared state. MediaPackage v2 and WAFv2 are separate AWS services and should stay separate; API Gateway v1/v2 and ELB v1/v2 mostly stay separate but share narrow account/region namespaces for custom domain names or load balancer names.
- Runtime and data-plane crates should normally validate against their control-plane siblings instead of silently creating upstream resources. Current high-priority gaps include AppConfig Data, MediaStore Data, SageMaker Runtime, Kinesis Video Archived Media, and CloudTrail Data.
- EC2 is not currently a shared network oracle for other service crates. Network-aware services often store VPC, subnet, security group, VPC endpoint, VPC link, or ENI-looking identifiers as service-local metadata, synthesise placeholders, or leave VPC-specific paths as 501 stubs. Creating EC2 network resources generally does not make another crate validate against them unless the service dossier says otherwise.
- Pure evaluator, parser, validator, and calculator crates are the durable home for semantic rules that outgrow handler-local checks.
- Handlers should parse requests, build pure-engine inputs, call the engine, and map outputs or diagnostics into AWS wire shapes.
- First implementations of semantic engines should not be treated as parity-complete. Review findings remain first-class follow-up inputs.
- EC2's split generated crate and feature gates are intentional service-scale infrastructure, not a pattern to apply casually to smaller services.

## Operational Guidance

- When adding a cross-service flow, identify the AWS-documented source, target, and supported action set before wiring services together.
- When touching a same-family pair, decide whether it is a shared-backend pair, a narrow shared-namespace pair, or an intentionally separate service pair before changing state.
- Prefer behaviour-level tests that exercise at least two services when the feature is an integration path, such as EventBridge target execution, Lambda event-source mappings, Pipes source consumption, or Step Functions task invocation.
- For data-plane work, create the control-plane resource first and assert unknown upstream resources are rejected unless AWS explicitly allows bring-your-own-result behaviour.
- Keep prerequisite control-plane resources explicit. AppSync data-source execution needs data sources and resolvers; EventBridge target execution needs stored `PutTargets` metadata; Athena Glue integration needs Glue databases and tables.
- If a feature needs semantic understanding, extend the dedicated engine crate rather than adding ad hoc checks to `handlers.rs`.
- For backend or query-engine work, keep heavy engines opt-in and expose constructor injection such as `with_backend(...)` or `with_query_backend(...)`.
- For derived services, keep the real lifecycle in the source service and store only local cursors, iterators, or delivery state in the derived service.
- For EC2, choose the correct feature bucket first, regenerate generated code with the feature map, gate handler dispatch and helpers, then verify both full and relevant partial-feature builds.

## Files

- `crates/winterbaume-appsync/src/handlers.rs`: AppSync control-plane surface and future data-source execution entry points.
- `crates/winterbaume-eventbridge/src/handlers.rs`: EventBridge rule and target metadata handling.
- `crates/winterbaume-dynamodb/src/backend.rs`: DynamoDB backend trait, including secondary-index support used by normal APIs and PartiQL execution.
- `crates/winterbaume-dynamodb/src/partiql_exec.rs`: backend-aware PartiQL execution bridge.
- `crates/winterbaume-dynamodbstreams/src/handlers.rs`: derived stream-consumption surface over DynamoDB change records.
- `crates/winterbaume-lambda/src/handlers.rs`: Lambda invocation and future event-source behaviour.
- `crates/winterbaume-sfn-asl-eval/src/`: Step Functions ASL parser and validator.
- `crates/winterbaume-sfn/src/handlers.rs`: Step Functions handler integration boundary for validation and service integrations.
- `crates/winterbaume-iam-rule-eval/src/`: IAM policy evaluation engine.
- `crates/winterbaume-wafv2-webacl-rule-parser/src/`: WAFv2 JSON-to-AST rule parser.
- `crates/winterbaume-wafv2-wcu-calculator/src/`: WAFv2 capacity calculation engine.
- `crates/winterbaume-bedrock-flow-parser/src/`: Bedrock flow definition parser.
- `crates/winterbaume-bedrock-flow-validator/src/`: Bedrock flow graph and type validator.
- `crates/winterbaume-athena/src/backend.rs`: Athena query backend trait.
- `crates/winterbaume-glue/src/handlers.rs`: Glue catalogue state that Athena should eventually resolve for Glue-type catalogues.
- `crates/winterbaume-sqlengine-duckdb/src/athena.rs`: DuckDB Athena backend without direct Glue awareness.
- `.agents/docs/services/ses.md` and `.agents/docs/services/sesv2.md`: SES shared-backend notes.
- `.agents/docs/services/api-gateway.md` and `.agents/docs/services/apigatewayv2.md`: custom-domain namespace notes.
- `.agents/docs/services/elastic-load-balancing.md` and `.agents/docs/services/elastic-load-balancing-v2.md`: load-balancer name namespace notes.
- `.agents/docs/services/appconfig.md`, `.agents/docs/services/appconfigdata.md`, `.agents/docs/services/mediastore.md`, `.agents/docs/services/mediastore-data.md`, `.agents/docs/services/sagemaker.md`, `.agents/docs/services/sagemaker-runtime.md`, `.agents/docs/services/kinesis-video.md`, `.agents/docs/services/kinesis-video-archived-media.md`, `.agents/docs/services/cloudtrail.md`, and `.agents/docs/services/cloudtrail-data.md`: control-plane/data-plane notes.
- `.agents/docs/services/*`: current network-aware service dossiers with `## Current Network Resource Stub Semantics` sections.
- `tools/smithy-codegen/ec2-features.toml`: authoritative EC2 operation-to-feature mapping.
- `tools/smithy-codegen/src/features.rs`: feature-map loading and transitive shape feature computation.
- `tools/smithy-codegen/src/gen_serializers.rs`: generated EC2 cfg emission and general serialiser generation.
- `crates/winterbaume-ec2-generated/src/model.rs` and `crates/winterbaume-ec2-generated/src/wire.rs`: regenerated EC2 Smithy output.
- `crates/winterbaume-ec2/src/handlers.rs`: feature-gated EC2 dispatch and handlers.

## Tests

Representative checks should stay scoped to the boundary being changed:

```bash
.agents/bin/cargo.sh test -p winterbaume-dynamodb
.agents/bin/cargo.sh test -p winterbaume-dynamodbstreams
.agents/bin/cargo.sh test -p winterbaume-athena
.agents/bin/cargo.sh test -p winterbaume-redshiftdata
.agents/bin/cargo.sh test -p winterbaume-sqlengine-duckdb
.agents/bin/cargo.sh test -p winterbaume-iam-rule-eval
.agents/bin/cargo.sh test -p winterbaume-sfn-asl-eval
.agents/bin/cargo.sh test -p winterbaume-wafv2-webacl-rule-parser
.agents/bin/cargo.sh test -p winterbaume-wafv2-wcu-calculator
.agents/bin/cargo.sh test -p winterbaume-bedrock-flow-validator
.agents/bin/cargo.sh check -p winterbaume-ec2
.agents/bin/cargo.sh check -p winterbaume-ec2 --no-default-features --features network
```

Cross-service features need integration tests that construct the source and target resources through their real service APIs where possible. Engine changes need focused unit tests inside the engine crate plus handler-level coverage for AWS response mapping.

State-coherence changes need paired-service tests. Create through one protocol surface and read through the sibling where AWS shares state, or create a control-plane resource and assert the data plane both accepts the known resource and rejects unknown resources.

## Pitfalls

- Do not turn a documented service relationship into a generic "any service can call any service" abstraction. AWS action sets and payload contracts are often narrow.
- Do not describe Step Functions and DynamoDB as a general DynamoDB integration; the optimised path only covers four item-level actions.
- Do not assume `DataCatalogType::Glue` means Athena can query Glue tables. The current backend has no catalogue-awareness layer.
- Do not move evaluator or validator semantics back into handlers once a dedicated crate exists.
- Do not assume implemented engine infrastructure means complete AWS parity. IAM conditions, WAFv2 surcharges, ASL modern `Map` validation, and Bedrock graph rules can still need refinement.
- Do not make derived services own upstream resource lifecycles. DynamoDB Streams should derive from DynamoDB table and write state.
- Do not infer shared state from naming alone. Confirm the real AWS coupling before joining v1/v2 or data-plane state.
- Do not let data-plane crates create control-plane resources on demand unless real AWS does so.
- Do not assume an EC2 identifier in another service has been validated against EC2 state. Current network-aware crates usually treat those identifiers as local metadata.
- Do not use EC2's feature-gated generated-crate split as a template for normal-sized services without a similar compile-time problem.
- Do not edit EC2 generated `model.rs` or `wire.rs` directly; update `smithy-codegen`, regenerate, and verify full plus partial-feature builds.
