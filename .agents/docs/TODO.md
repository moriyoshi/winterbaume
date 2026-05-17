# Project To-Dos

Items extracted from `JOURNAL.md` during good-sleep consolidation. Keep this file as an active backlog only: once an item is addressed, move its work summary into `JOURNAL.md` and remove it from here.

## Open Items

### GitHub Triage

- [ ] **github-triage-kb-draft-synthesis**: Add a monthly KB-draft synthesis workflow once `memory/triage` has enough outcome records. It should read recent `outcomes/*.jsonl`, propose `kb-draft` issues, and leave human ratification as the promotion path. -- *source: 2026-04-26: Bug-triage automation -- GitHub Actions + sidecar memory branch*
- [ ] **github-triage-richer-retrieval**: Revisit embeddings or richer retrieval for `memory/triage` only if GitHub MCP issue search plus recent triage-log examples start missing obvious duplicate or recurring reports. -- *source: 2026-04-26: Bug-triage automation -- GitHub Actions + sidecar memory branch*
- [ ] **triage-bug-roll-to-live**: Roll the sandbox `triage-bug.yml` to the live repo. The current production guardrail has been a no-op since the gpt-5 cutover; this is the actual fix for the 0%-effective guardrail and is the highest-value remaining action from the 2026-04-27 audit. -- *source: 2026-04-27 -- Bug-triage automation: end-to-end test*

### Persistence And Workspace Architecture

- [ ] **persistence**: Design a durable persistence mechanism for configuration/secret store services (SSM Parameter Store, SecretsManager) so their state survives server restarts. These services hold data that other services and Terraform runs depend on; the solution need not use BlobStore and should consider the broader persistence story, such as automatic snapshot-to-disk, VFS-backed state, or a dedicated persistence layer. -- *source: 2026-04-06*
- [ ] **workspace-dispatch**: `DEFAULT_ACCOUNT_ID` is hard-coded in `dispatch()` in roughly 89 service crates. Every PASS verdict tacitly accepts that multi-account is not wired into routing. Coordinated cleanup could use a shared `dispatch_with_scope()` helper in `winterbaume-core` that extracts account from request credentials. -- *source: 2026-04-27 -- Quality Gate sweep aggregate*

### Cross-Service Integration Gaps

- [ ] **cloudformation**: Implement a CloudFormation backend that replicates the real service behaviour. Would require a substantial redesign in the state/view layer though.
- [ ] **athena-glue**: `StartQueryExecution` does not resolve database/table metadata from Glue state when the target catalogue is of type `Glue`. In real AWS, Athena's default catalogue is the Glue Data Catalogue. Work needed: look up the target catalogue type on query execution, resolve Glue database/table schemas when the catalogue type is `Glue`, feed schema info into `AthenaQueryBackend` or a layer above it, and add cross-service integration tests that create Glue databases/tables then query via Athena. -- *source: 2026-04-21 cross-service analysis*
- [ ] **eventbridge-targets**: `put_events()` generates event IDs but does not match events against rules or invoke any targets (Lambda, SQS, SNS, Step Functions, ECS, Batch, API Gateway, CloudWatch Logs, AppSync, Kinesis, Firehose). Target metadata is stored via `PutTargets` but execution semantics are absent. -- *source: 2026-04-21 cross-service audit*
- [ ] **eventbridge-pipes**: Pipes store source and target ARNs but perform no actual polling, transformation, or target invocation. Sources include DynamoDB Streams, Kinesis, and SQS; targets include Lambda, SQS, SNS, Step Functions, and others. -- *source: 2026-04-21 cross-service audit*
- [ ] **lambda-event-sources**: `create_event_source_mapping()` stores mappings but does not poll DynamoDB Streams, SQS, SNS, or Kinesis sources or invoke the Lambda function. -- *source: 2026-04-21 cross-service audit*
- [ ] **sfn-execution**: `start_execution()` creates and stores an execution record but does not interpret or execute the state machine definition. Optimised service integrations (Lambda, DynamoDB `GetItem`/`PutItem`/`UpdateItem`/`DeleteItem`, SQS, SNS, EventBridge, ECS, Batch, API Gateway) are not invoked. -- *source: 2026-04-21 cross-service audit*
- [ ] **appsync-resolvers**: AppSync implements only control-plane operations (API management, schema creation, data source and resolver CRUD). No resolver execution or data source invocation exists for DynamoDB, Lambda, OpenSearch, RDS Data API, HTTP endpoints, or EventBridge data sources. -- *source: 2026-04-21 cross-service audit*
- [ ] **apigateway-lambda**: API Gateway and API Gateway V2 store integration endpoints but do not invoke Lambda functions or any other integration target when requests arrive. -- *source: 2026-04-21 cross-service audit*
- [ ] **dynamodbstreams-lambda**: DynamoDB Streams captures change records in `stream_records` but never dispatches them to Lambda event-source mappings. This overlaps with the `lambda-event-sources` item but highlights the DynamoDB Streams to Lambda path specifically. -- *source: 2026-04-21 cross-service audit*

### State Coherence Backlog

Opened from the 2026-05-01 v1/v2 and control-plane / data-plane state coherence audits. See the JOURNAL entries "v1/v2 service-pair state coherence audit" and "control-plane / data-plane state coherence audit" for the full per-pair tables, and the `## v1/v2 State Coherence` / `## Control-Plane / Data-Plane Coherence` sections in the affected `.agents/docs/services/*.md` for the local context next to each service dossier.

- [ ] **ses-v1-v2-shared-backend**: `winterbaume-ses` and `winterbaume-sesv2` hold completely separate state structs ( `SesV1State` vs `SesState` ) even though real AWS backs both APIs with one backend. Identities, configuration sets, templates, suppression list, dedicated IP pools, and account-level settings created via one API are invisible to the other. Per-family unit of work: identities → configuration sets → templates → suppression list → account-level settings. Receipt rule sets stay v1-only; v2-only families ( contact lists, import/export jobs, multi-region endpoints, tenants, deliverability test reports, reputation entities ) stay where they are. Cross-API integration tests are the right regression guard. -- *source: 2026-05-01 v1/v2 audit*
- [ ] **apigateway-v1-v2-domain-name-namespace**: API Gateway v1 ( REST APIs ) and v2 ( HTTP/WebSocket APIs ) each own their own `domain_names` map. Real AWS reserves custom domain names in a single account+region namespace; `winterbaume-apigateway` `CreateDomainName` and `winterbaume-apigatewayv2` `CreateDomainName` should not both succeed for the same domain. Cross-crate name reservation needed. -- *source: 2026-05-01 v1/v2 audit*
- [ ] **elb-v1-v2-name-namespace**: Classic ELB ( `winterbaume-elasticloadbalancing` ) and ALB/NLB/GLB ( `winterbaume-elasticloadbalancingv2` ) each own their own `load_balancers` map. Real AWS reserves load balancer names in a single account+region namespace; creating a Classic ELB and an ALB with the same name returns `DuplicateLoadBalancerName`. Cross-crate name uniqueness needed. -- *source: 2026-05-01 v1/v2 audit*
- [x] **appconfigdata-shared-state**: Done 2026-05-18. End-to-end fix across both crates:
  - `winterbaume-appconfig`: `HostedConfigurationVersionData` gains `content: Vec<u8>`. `create_hosted_configuration_version` now takes the content bytes and stores them. `handle_create_hosted_configuration_version` passes the wire `Content` field through ( treated as raw bytes; correct for text-based configs and base64-decoded binary that callers pass through unchanged ). New helper `AppConfigState::get_deployed_configuration(app, env, profile) -> Option<(&str, &[u8])>` finds the highest-numbered `COMPLETE` deployment for the target and returns the matching hosted-version's content_type + content. Public accessor `AppConfigService::shared_state() -> Arc<BackendState<AppConfigState>>` exposes the per-account/region state holder. The `HostedConfigurationVersionView` snapshot explicitly excludes content ( the existing doc comment is unchanged ); restored versions re-materialise with empty bytes.
  - `winterbaume-appconfigdata`: workspace dep on `winterbaume-appconfig` added. New `AppConfigDataService::with_appconfig_state(Arc<BackendState<AppConfigState>>)` constructor wires the parent state; `Self::new()` keeps the legacy ( unwired, empty-body ) behaviour. `handle_get_latest_configuration` now resolves the session's `(application_id, environment_id, configuration_profile_id)` through the parent state when wired, returning the actual deployed payload + the right `Content-Type` header. New end-to-end integration test `test_get_latest_configuration_resolves_through_appconfig_state` walks the full control-plane / data-plane path: CreateApplication -> CreateEnvironment -> CreateConfigurationProfile -> CreateHostedConfigurationVersion -> CreateDeploymentStrategy -> StartDeployment -> StartConfigurationSession -> GetLatestConfiguration, and asserts the returned blob matches the uploaded JSON payload byte-for-byte plus `Content-Type: application/json`.
  - Per-crate gate clean on both: clippy + fmt --check + test --no-fail-fast all pass ( 47 appconfig tests + 3 doctests; 7 appconfigdata tests + 1 doctest, +1 from the new end-to-end test ).
  - Limitation: `source_region` and non-hosted configuration sources ( S3, SSM Parameter Store, Secrets Manager ) are not implemented — the parent looks up only hosted configuration versions. Add when a real workflow needs the other source types.

  Closes the **High severity** entry from the 2026-05-01 control-plane / data-plane audit. -- *source: 2026-05-01 control-plane / data-plane audit; closed 2026-05-18*
- [ ] **mediastoredata-container-model**: `winterbaume-mediastoredata` stores objects in a single global `objects: HashMap<String, MediaStoreObject>` with no container concept. An object PUT into one container is visible to GET requests for any other container, and `DeleteContainer` on the control plane does not affect the data-plane store. **High severity** — model is wrong. Re-key by `(container_name, path)` ( derived from request host header / endpoint ), depend on `winterbaume-mediastore`, and reject operations against unknown / `DELETING` containers. `DeleteContainer` should also reject when the container still holds objects ( `ContainerNotEmptyException` ). -- *source: 2026-05-01 control-plane / data-plane audit*
- [ ] **sagemakerruntime-endpoint-validation**: `winterbaume-sagemakerruntime` auto-creates endpoints on first invocation ( explicit comment in `state.rs` ). Real AWS rejects unknown endpoints with `ValidationError` ( "Endpoint X of account Y not found" ). Depend on `winterbaume-sagemaker`, resolve endpoint names through its `endpoints` map, and route the targeted variant through the endpoint-config / model graph so invocation records can capture it. -- *source: 2026-05-01 control-plane / data-plane audit*
- [ ] **kinesisvideoarchivedmedia-stream-validation**: `winterbaume-kinesisvideoarchivedmedia` has an `ensure_stream` helper that auto-creates streams with mock fragments on first request. Real AWS rejects unknown streams with `ResourceNotFoundException`. Depend on `winterbaume-kinesisvideo` and reject unknown stream names / ARNs; fragment payload storage can stay in this crate. Same pattern likely applies to future `kinesisvideomedia`, `kinesisvideosignaling`, and `kinesisvideowebrtcstorage` crates. -- *source: 2026-05-01 control-plane / data-plane audit*
- [ ] **cloudtraildata-channel-validation**: `winterbaume-cloudtraildata` records events with whatever `channel_arn` it is given without checking that the channel exists. Real AWS rejects unknown channel ARNs on `PutAuditEvents` with `ChannelARNInvalidException`. Depend on `winterbaume-cloudtrail` and validate the channel ARN against its `channels` map. -- *source: 2026-05-01 control-plane / data-plane audit*
- [ ] **rdsdata-cluster-validation** ( low priority ): `winterbaume-rdsdata` is a "bring-your-own-result" mock that does not validate `resourceArn` against `winterbaume-rds`'s clusters. Acceptable for unit testing where the test author seeds expected results, but not parity behaviour. Open a concrete TODO only if a workflow needs the data plane to validate cluster existence and the "Data API enabled" state. -- *source: 2026-05-01 control-plane / data-plane audit*
- [ ] **redshiftdata-cluster-validation** ( low priority ): `winterbaume-redshiftdata` carries its own `databases`/`schemas`/`table_names` catalogue with no dependency on `winterbaume-redshift` or `winterbaume-redshiftserverless`. Open a concrete TODO only if a workflow needs cluster/workgroup validation and authentication-mode handling. The catalogue itself stays in this crate ( Redshift's control plane does not expose a schema-catalog API ). -- *source: 2026-05-01 control-plane / data-plane audit*

### Quality Gate And Scenario Backlog

- [ ] **invariant-audit-existing-services**: Roll out the cross-call invariant inventory to existing service crates. Suggested sequencing: run `.agents/bin/audit-state-fields.sh --all`, turn real flags into per-crate invariant-audit rows, then use `/write-tests` to backfill inventories and scenario coverage. No silent grandfathering; TODO rows are the audit trail.
  - **2026-05-02 dry-run finding**: `audit-state-fields.sh --all` over 223 crates produced flags only on `winterbaume-ec2`. Every other crate ( `rds`, `elasticache`, `iam`, `s3`, `sqs`, `sns`, `dynamodb`, `route53`, ... ) reported "No `self.counters.*` references found in state.rs". The audit's heuristic-B pattern is hard-coded to the EC2-style `Ec2Counters` field naming; non-EC2 crates mint IDs through different shapes ( UUIDs, direct counter fields without a `counters` substruct, hashmap insertion order, time-derived strings ) and slip through. Heuristic A ( toggle-without-consumer ) similarly didn't fire — toggles in the EBS-encryption-by-default style are uncommon across the fleet.
  - **Implication**: the audit script as shipped is a high-precision EC2 backstop, not a fleet-wide checker. Either ( a ) extend the script with a second-pass that grokss `self.<id-counter-field>` patterns matched against `format!("…-{:08x}"` mint sites regardless of where the counter lives, or ( b ) accept the current scope and lean on the Phase 1 invariant inventory in `/write-tests` as the fleet-wide instrument when authors revisit each crate.
  - Backfilling inventories via `/write-tests` is still the right path for the existing fleet; the dry-run just shows the static script can't pre-stage that work.
  - **2026-05-17 update**: Option ( a ) is now implemented. `audit-state-fields.sh` gains a `direct_counter_uses_with_fn` fallback that fires when a crate has no `self.counters` substruct ( almost every non-EC2 crate ). It emits `<field>\t<fn>` pairs for `self.<field>` references that appear in a `format!(...)` ID-minting line, plus the same field captured at every `self.<field> += 1` / `self.<field> = self.<field> + 1` increment site, with a small denylist for obvious non-counter field names ( `state`, `config`, `metadata`, `items`, `name`, `arn`, `id`, `notifier`, `tags`, `next_token`, `counters` ). The fallback only runs if heuristic B's EC2 path produced nothing, so EC2's existing 11 flags are unaffected. Spot-checked against `iam` ( no counters — no flags ), `sqs` ( fallback active, no shared counters ), `dynamodb` ( fallback active, three single-method counters — no shared-counter flag ), and `ec2` ( original path still emits its 11 sections ). Remaining work on this TODO is the fleet sweep + backfill of per-crate invariant rows; the script is no longer the bottleneck.
  - **2026-05-17 fleet sweep**: Ran `audit-state-fields.sh --all` ( 224 crates ). Five non-EC2 crates produce shared-counter flags through the new fallback ( raw reports preserved under `.agents-workspace/tmp/audit-<crate>.md` ). Each needs per-crate AWS-doc review to determine real-bug vs defensible-global-counter:
    - `costexplorer`: `self.next_resource_id` shared between `create_anomaly_monitor` and `create_anomaly_subscription`. Likely OK -- the two are stored in separate state maps, so the counter only collides across types, not within a type. Anomaly monitor and anomaly subscription ARNs use distinct AWS resource-type prefixes.
    - `guardduty`: `self.next_set_id` shared across `create_ip_set`, `create_threat_entity_set`, `create_threat_intel_set`, `create_trusted_entity_set`. Same shape -- four distinct set families share one counter; real AWS uses opaque per-detector-scoped numeric IDs, so a global counter that never repeats is a defensible mock. Possibly a real concern if cross-family ID-collisions are observable, otherwise close.
    - ~~`kinesis`: `self.next_sequence` shared across `put_record`, `put_record_by_arn`, `put_records`.~~ **Fixed 2026-05-17.** `KinesisState.next_sequence` removed; replaced with a per-shard counter `Stream.next_sequence_per_shard: HashMap<String, u64>`. All three put-paths now compute the target shard first, then increment that shard's counter and return its new value as the `SequenceNumber`. Regression test `test_put_record_sequence_numbers_are_per_shard` in `crates/winterbaume-kinesis/tests/integration_test.rs` asserts the per-shard monotonic-from-1 invariant. Per-crate gate clean: 86 tests pass.
    - `opensearch`: `self.next_id` shared across `add_direct_query_data_source`, `create_domain`, `create_outbound_connection`, `create_package`, `create_vpc_endpoint`, `purchase_reserved_instance_offering` ( six unrelated resource families ). Real AWS uses distinct ID namespaces ( domain ARNs, VPC endpoint IDs, package IDs, etc. ); the shared counter is OK so long as the resource-type prefix in the formatted ID guarantees no collision, but is a code smell worth splitting into per-family counters for readability.
    - `servicediscovery`: `self.instances_revision` shared between `register_instance` and `deregister_instance`. This is by design — service-discovery instance revisions are a single monotonic version stream that both register and deregister bump. **False positive**, can be closed.
    - Open follow-up: each of these five needs an invariant-audit row in `/write-tests`'s Phase 1 inventory ( with the AWS-doc citation that confirms or denies the bug ) and an invariant-style regression test if a bug is confirmed. Kinesis is the clearest action item. -- *source: 2026-05-02 Orchestration: invariant inventory becomes a first-class artefact*
- [x] **good-sleep-invariant-roundtrip**: Verified during `reconcile-journal-ltm` on 2026-05-02. The invariant inventory is represented in `aws-doc-test-plan-catalog.md`, `quality-gate-workflow-and-recurring-findings.md`, `journal-ltm-maintenance-workflows.md`, and `ec2-operation-expansion-and-invariants.md`, and the canonical journal record maps the source sections to those LTM documents. -- *source: 2026-05-02 Orchestration: invariant inventory becomes a first-class artefact*
- [x] **ec2-audit-residue**: Reviewed 2026-05-02. 11 of 12 flags are legitimate helper-fn + caller pairs ( `next_<thing>_id` increments the counter; `create_*` / `allocate_*` / `import_*` / `provision_*` / `seed_*` calls the helper ) or families of distinct ID-minters that correctly share one counter ( `run_instances` + `request_spot_instances` both produce `i-XXXX`; `create_vpc` + `create_default_vpc` both produce `vpc-XXXX`; `copy_fpga_image` + `create_fpga_image` both produce `afi-XXXX` ). The lone real bug was `next_eni_attach_id` reading `self.counters.eni` without incrementing — same shape as the eipassoc bug. Fixed by adding `Ec2Counters.eni_attach`, mirroring it into `CountersView`, and incrementing it in the helper. Regression covered by `tests/scenario_test.rs::test_eni_attach_ids_are_unique_per_call`. -- *source: 2026-05-02 Orchestration: invariant inventory becomes a first-class artefact*

### Core / Shared Infrastructure

- [ ] **core-url-query-parser-sweep**: 59 service crates ( `accessanalyzer`, `amp`, `amplify`, `apigateway`, `apigatewaymanagement`, `apigatewayv2`, `appmesh`, `appsync`, `auditmanager`, `autoscaling`, `batch`, `bedrock`, `cloudformation`, `codeartifact`, `connect`, `connectcampaigns`, `databrew`, `dsql`, `ebs`, `efs`, `eks`, `elasticache`, `elasticbeanstalk`, `elasticloadbalancing`, `elasticloadbalancingv2`, `emrcontainers`, `iam`, `iot`, `iotdataplane`, `kafka`, `lambda`, `lexmodelsv2`, `medialive`, `mediapackage`, `mediastoredata`, `mq`, `neptune`, `networkmanager`, `opensearch`, `osis`, `ram`, `redshift`, `resiliencehub`, `resourcegroups`, `rolesanywhere`, `route53`, `s3`, `s3control`, `s3tables`, `sagemakerruntime`, `servicecatalogappregistry`, `ses`, `sesv2`, `sns`, `sso`, `sts`, `synthetics`, `vpclattice`, `workspacesweb` ) carry private copies of `percent_decode` / `hex_val` / `extract_query_param` / `extract_query_param_multi` in `src/handlers.rs`, byte-identical to the canonical helpers re-exported from `winterbaume_core` ( `crates/winterbaume-core/src/lib.rs:16-19`, backed by `crates/winterbaume-core/src/protocol/common.rs:39-86` ). Several of these crates already import `winterbaume_core::extract_path` from the same `use` block ( e.g. `winterbaume-sesv2/src/handlers.rs:8-11` calls `winterbaume_core::extract_path` at line 3140 yet redefines `percent_decode` at line 3143 ), so the dependency edge is live -- this is pure copy-paste drift, not isolation. **Gap in core**: `parse_query_string` returns `HashMap<String, String>` which collapses repeated keys ( e.g. `TagKeys=a&TagKeys=b` ); that is why crates roll their own `extract_query_param_multi`. Plan: ( 1 ) add `extract_query_param` ( single ) and `extract_query_param_multi` ( repeated ) to `protocol::common` and re-export from `lib.rs`, ( 2 ) sweep the 59 crates in batches, deleting local copies and replacing call sites, ( 3 ) per-crate clippy + `fmt --check` gate after each batch.
  - **2026-05-16 spot-check**: The "byte-identical" claim above is not actually true across the 59 crates. Sampling 4 of them found three distinct signature shapes for `extract_query_param`: `(query_string: &str, param_name: &str)` ( opensearch ), `(query: &str, key: &str)` taking only the body but with a different URL-decode dialect ( iotdataplane uses a local `urlencoding_decode` that handles `%XX` + `+` ), and `(uri: &str, key: &str)` ( osis, s3tables ) that internally call `uri.split('?').nth(1)?` to extract the body first. The decode dialects also differ: opensearch hand-rolls `%3A` / `%2F` only, iotdataplane delegates to its own decoder, osis / s3tables both call a local `percent_decode` ( the byte-identical-to-core path ). Step ( 1 ) of the plan therefore has a design decision: the canonical `extract_query_param` should take the query string body ( consistent with `extract_query_string(uri) -> &str` already exposed by core ), but the step-2 sweep is then more than a symbol swap at the URI-passing call sites -- those need to be rewritten as `extract_query_param(extract_query_string(uri), key)`. Step 1 is still a small change to one core file; the sweep should be planned in batches by call-site shape ( body-taking crates first, URI-taking crates second ), not alphabetically. -- *source: 2026-05-02 — URL query-string parser duplicated across 59 crates; refined 2026-05-16 spot-check*
- [ ] **mockaws-builder-account-id-runtime-override**: Decide whether `MockAws::build()` should call `winterbaume_core::set_default_account_id(self.account_id.clone())` so library users get the same runtime handler account-id behaviour that `winterbaume-server --account-id` now installs. The current fix deliberately preserves old `MockAws` behaviour: the builder's account ID is stored for `mock.account_id()` only, while handlers read the process-wide `default_account_id()`. Any change must account for first-writer-wins `OnceLock` semantics and test isolation. -- *source: 2026-05-13 — winterbaume-server --account-id honoured at runtime*

### Terraform Converter Follow-Ups

- [x] **terraform-coverage-prefix-overrides-tail**: Done 2026-05-16. `generate_terraform_resource_coverage.py` now has an explicit `kinesis` `PREFIX_OVERRIDES` entry (`aws_kinesis_stream`, `aws_kinesis_resource_policy`) plus a new `HANDLED_ALIAS_RULES` mechanism with `synthesise_alias_handled()` for elbv2 `aws_alb_*` → `aws_lb_*` aliases. Deltas: elbv2 62% → 100%, kinesis 50% (spurious) → 100%. Remaining `aws_kinesisanalyticsv2_application_snapshot` is a real gap, not a classification artefact. -- *source: 2026-05-11 / 2026-05-12: Post-EC2 sweep — close the workspace coverage tail*
- [x] **terraform-macro-extract-coverage**: Done 2026-05-16. `generate_terraform_converter_coverage.py` now credits the trailing positional `"name"` literal of macro invocations via `_macro_trailing_extract_attrs()` plus per-macro-family always-credited attributes (`impl_bucket_subresource_converter` → `bucket`, `impl_bucket_named_config_converter` → `bucket` + `name`) on both inject and extract sides. Spot-check: `aws_s3_bucket_accelerate_configuration` extract 0% → 67%. Overall extract coverage 5251/10765 → 5278/10765. Residual: macro-emitted inner sub-resource bodies still need `$model_ty` argument resolution to credit further. -- *source: 2026-05-13 (cont.): Fix three rendering bugs in docs/reference/terraform.md and the underlying coverage heuristics*

### Agent Skill Maintenance

- [x] **add-service-cargo-version-template**: Done 2026-05-16. `.agents/skills/add-service/SKILL.md` Cargo.toml scaffold switched from `version.workspace = true` to literal `version = "0.1.0"`, with an explanatory note about the workspace-package inheritance set (only `edition`/`license`/`repository`/`keywords`). -- *source: 2026-05-02 — Stub crate: winterbaume-s3files (restJson1, model 2025-05-05)*
- [x] **add-service-restjson-reference**: Done 2026-05-16. `.agents/skills/add-service/SKILL.md` "Reference implementations" restJson1 entry now points at `crates/winterbaume-sesv2/src/handlers.rs`, with an inline reminder that SES v1 is awsQuery. -- *source: 2026-05-02 — Stub crate: winterbaume-s3files (restJson1, model 2025-05-05)*
- [x] **add-service-state-view-builder-template**: Done 2026-05-16. `.agents/skills/add-service/SKILL.md` Step 5 now has a "Rule: construct `*View` literals through small helper functions" subsection, with a template mirroring `winterbaume-s3files`'s `fs_view` and rewritten notification-test snippets that populate a `HashMap` via `mk_resource_view(...)` plus `..Default::default()`. -- *source: 2026-05-02 — winterbaume-s3files: full operation coverage (21/21)*
- [x] **add-service-sdk-accessor-shape-note**: Done 2026-05-16. `.agents/skills/add-service/SKILL.md` Step 5 Tips bullet added noting accessor optionality varies per response type within one service, citing the s3files `&str` / `Option<&str>` mix across file-system, policy, and mount-target responses. -- *source: 2026-05-02 — winterbaume-s3files: full operation coverage (21/21)*

### Smithy Codegen

- [ ] **restjson1-handler-dispatch-refactor**: Most restJson1 services have now been refactored to thread `&MockRequest`, labels, and query into handlers and consume generated `wire::deserialize_<op>_request(...)` functions. Keep this item only for the remaining URL-routing shapes that still cannot consume generated deserialisers directly, especially restXml URL-routing such as CloudFront if `quick_xml::de::from_str` request parsing remains after the 2026-05 wire-deserialiser sweep. Do not use this as a blanket restJson1 blocker anymore; API Gateway's residual body reads are tracked as an intentional PATCH-style hybrid, and CloudWatch needs a separate multi-protocol recipe. -- *source: 2026-05-02 to 2026-05-05 — wire deserialiser adoption sweep*
- [ ] **cloudwatch-wire-deserialiser-multiprotocol**: Migrate `winterbaume-cloudwatch` request parsing onto generated wire deserialisers for its multi-protocol awsQuery + rpc-v2-cbor surface. The crate remains one of 2 residual services after the 166 / 168 adoption sweep, with roughly 40 `body.get` reads because dispatch chooses request protocol from URL shape and the body parsing diverges accordingly. Needs a CloudWatch-specific multi-protocol recipe, not the standard restJson1 threaded-dispatch pattern. -- *source: 2026-05-05 — Wire Deserialiser Adoption Sweep: Consolidated Findings*
- [x] **smithy-codegen-glue-service-map-entry**: Done 2026-05-16. Added `("glue", "glue")` to `SERVICE_MAP` in `tools/smithy-codegen/src/discover.rs`, between `glacier` and `guardduty`. `cargo run -p smithy-codegen -- list-services` now emits `glue    glue`. Per-crate clippy + fmt gate clean. -- *source: 2026-05-05 — Wire Deserialiser Adoption Sweep: Consolidated Findings*
- [x] **dynamodb-data-plane-deserialiser-adoption**: Done 2026-05-03. All 13 data-plane ops ( PutItem, GetItem, DeleteItem, UpdateItem, Query, Scan, BatchWriteItem, BatchGetItem, TransactGetItems, TransactWriteItems, BatchExecuteStatement, ExecuteStatement, ExecuteTransaction ) now consume `wire::deserialize_<op>_request(body)`. Foundation: `impl From<model::AttributeValue> for types::AttributeValue` plus `item_from_wire` / `attr_map_from_wire` helpers in `types.rs`. The four `&Value`-shaped helpers ( `extract_expr_context`, `parse_opt_expr`, `extract_key_conditions`, `legacy_extract_equalities` ) plus `json_obj_to_item` / `json_val_to_attr` deleted; `expr::parse_projection_expression` refactored from `&Value` to `(Option<&str>, &HashMap<String, String>)`. `expr::parse_update_expression` still consumes `&Value` ( deeper expr.rs surface ); call sites synthesise a small `serde_json::json!({...})` from typed fields — minor follow-on cleanup. New helper `wire_attrs_to_json` bridges PartiQL parameters into `winterbaume_partiql::parse_statement`'s expected `&[serde_json::Value]`. `body.get(` count: 19 → 0. 7 unit + 177 integration + 5 scenario tests pass. -- *source: 2026-05-03 — DynamoDB data-plane migration*
- [x] **dynamodb-update-expression-value-shape**: Done 2026-05-03. `crate::expr::parse_update_expression` and helpers (`parse_set_assignment`, `resolve_path`) now take typed `(update_expression: &str, expr_names: &HashMap<String, String>, expr_values: &HashMap<String, AttributeValue>)` directly. Both call sites in handlers.rs (`handle_update_item`, TransactWriteItem Update branch) drop the synth `json!({...})` Value. `expr::json_val_to_attr` and the `serde_json::{Map, Value}` import in expr.rs are deleted as no longer needed. `parse_projection_expression` also simplified to drop its names_value adapter. Per-crate gate clean; 7 + 177 + 5 tests pass. -- *source: 2026-05-03*
- [x] **codegen-stale-wire-regen-audit**: Done 2026-05-02. Mass-regen sweep across 151 stale crates landed 145 successful regenerations ( 138 in the initial sub-agent pass + 7 follow-ups after fixing 4 SERVICE_MAP slug aliases and 3 generator bugs ). Generator fixes: ( 1 ) `binding_value_expr_for_member` now selects f64-vs-String parsing for timestamp http-bindings based on the resolved `timestamp_format` ( fixes bedrock + emrcontainers where the f64-with-RFC-3339-fallback was emitted into String fields ); ( 2 ) added `chrono` workspace dep to `winterbaume-signer` ( the only remaining crate without it that needed the f64 fallback path ); ( 3 ) added 4 alias entries to `tools/smithy-codegen/src/discover.rs::SERVICE_MAP` ( cloudwatchlogs, cognitoidentityprovider, databasemigration, directory ) so the literal crate suffix resolves directly. 6 crates still rolled back due to field drift ( see new follow-up below ). Broken regen artefacts archived under `.agents-workspace/tmp/regen-broken/` for reference. -- *source: 2026-05-02 — 151-crate mass regen sweep*
- [x] **codegen-field-drift-handler-updates**: Done 2026-05-16. All 6 rolled-back crates regenerated and handler/state placeholders added; per-crate clippy + fmt + test gate clean on every crate. Per-crate detail — `account`: `account_state: None` in `GetAccountInformationResponse` ( 33 tests ); `organizations`: `path: None` in `OrganizationalUnit` via the `ou_wire()` helper ( 107 tests ); `batch`: `quota_share_policy: None` in `SchedulingPolicyDetail` ( 50 tests ; regen also introduced new `CreateQuotaShareResponse` / `UpdateQuotaShareResponse` shapes plus `quota_share_policy` on `Create/UpdateSchedulingPolicyRequest`, but those are request-side or unreferenced ); `applicationsignals` ( model-dir `application-signals` ): `composite_sli_config: None` + `metric_source: None` on `ServiceLevelObjectiveSummary`, `auto_investigation_enabled: None` on `ServiceLevelObjective` ( 13 tests ); `ivs`: 7 CORS headers ( `access_control_allow_origin`, `access_control_expose_headers`, `cache_control`, `content_security_policy`, `strict_transport_security`, `x_content_type_options`, `x_frame_options`, all `Option<String> -> None` ) across 3 batch-response structs `BatchGetChannelResponse`, `BatchGetStreamKeyResponse`, `BatchStartViewerSessionRevocationResponse` ( 54 tests ); `opensearch`: `service_options: None` on `AuthorizedPrincipal`, `prometheus: None` on `DirectQueryDataSourceType` ( 54 tests ). Regen-broken artefacts under `.agents-workspace/tmp/regen-broken/` left in place for reference. -- *source: 2026-05-02 — 151-crate mass regen sweep*
- [x] **awsjson-deserialiser-adoption-sweep**: Done 2026-05-02. KMS ( 0 → 52 calls, 92 → 0 `body.get` ), Glue ( 0 → 70 calls, 79 → 3 in dead-code comments ), SSM ( 0 → 109 calls, 161 → 0 ), DynamoDB control-plane ( 0 → 39 calls, 78 → 19 ; data-plane deferred per the dynamodb-data-plane TODO above ). Migration recipe: dispatch captures `let body_bytes: &[u8] = &request.body`, migrated handlers take `body: &[u8]` and call `wire::deserialize_<op>_request(body)?`. -- *source: 2026-05-02 — Phase 4 KMS pilot + parallel sub-agent rollout*
- [x] **awsquery-restxml-deserializer-adoption-sweep**: Done 2026-05-02. SES fully migrated ( 29 calls; 244 → 1 manual, the residual `params.get("Action")` is dispatch routing ). Partial-adoption awsQuery tail also closed: elbv2 ( 13 → 2 ), neptune ( 7 → 5, residual non-input shape parsing ), cloudformation ( 4 → 1 ), redshift ( 4 → 3 ), autoscaling ( 2 → 1 ). The 4 remaining tail crates ( sts 3, sns 2, elasticbeanstalk 1, elasticloadbalancing 1 ) had only dispatch-action and Attributes-map ( codegen does not emit map deserialisers ) reads — left intact per recipe rule 5. RestXml beyond s3 / route53 stays blocked by the **restjson1-handler-dispatch-refactor** TODO above. -- *source: 2026-05-02 — SES + partial-adoption tail*

### Service-Specific Follow-Ups

- [ ] **amplifybackend**: 27 deferred operations (BackendAPI / BackendAuth / BackendStorage / BackendConfig / Token / BackendJob / ImportBackend* / RemoveAllBackends / ListS3Buckets). AWS deprecated Amplify Gen-1, so the deeper surface is unlikely to be exercised by current callers; revisit if a real bug report names one. -- *source: 2026-04-26 -- Implement winterbaume-amplifybackend crate (partial)*
- [ ] **appfabric**: 19 deferred operations (AppAuthorization / Ingestion / IngestionDestination / user-access-task families). They form a deeper hierarchy nested under AppBundle and need their own state-modelling pass. -- *source: 2026-04-26 -- Implement winterbaume-appfabric crate (partial)*
- [ ] **appflow**: 15 deferred operations (ConnectorProfile family, Connector registration, Connector / ConnectorEntity discovery, FlowExecutionRecords, CancelFlowExecutions, ResetConnectorMetadataCache). Either separate resource families or metadata-discovery endpoints with no mock source-of-truth. -- *source: 2026-04-26 -- Implement winterbaume-appflow crate (partial)*
- [x] **emrcontainers-state-view-job-runs**: Done 2026-05-16. Audit confirms `job_runs` is already fully wired: `EmrContainersStateView.job_runs: HashMap<String, JobRunView>` (views.rs:24), `state -> view` conversion (views.rs:126-130), `view -> state` reconstruction (views.rs:267-284), and `merge` integration (views.rs:386-388). No code change needed. -- *source: 2026-04-30 SES SendEmail Examples and State/View Disparity Sweep*
- [ ] **pinpoint-sms-voice-v2**: Implement `pinpoint-sms-voice-v2`. The v1 surface (8 ops) landed; v2 is a separate, much larger surface that has been left for a follow-up. -- *source: 2026-04-26 -- new service: pinpoint-sms-voice (Pinpoint SMS Voice v1)*
- [x] **sqs-redis-redrive-receive-budget**: Done 2026-05-17. Root cause was `RECEIVE_SCRIPT` in `crates/winterbaume-sqs-redis/src/lib.rs:262` using `#results` ( the combined `R:` + `D:` entry count ) as both the receive-budget loop guard and the receipt-handle ARGV index. Replaced with a dedicated `recv_count` local that counts only `R:` entries; the `D:` ( dead-letter handoff to the Rust caller ) branch no longer consumes a handle slot or the caller's `max_n` budget. Verified via `cargo build -p winterbaume-sqs-redis` and `cargo test -p winterbaume-sqs-redis --no-run`; `cargo fmt --check` passed. The full per-crate `cargo clippy` gate is blocked by a separate, pre-existing arc-swap dep-resolution quirk that affects the whole workspace's `cargo check` path ( `error[E0463]: can't find crate for arc_swap` while compiling the redis dep, despite arc-swap being a non-optional dep of redis 0.27.6 and present in `Cargo.lock`; `cargo build` resolves it correctly ). A regression test for this Lua change requires either an `mlua`-based unit harness or an integration test against a real Redis instance; `winterbaume-sqs-redis` currently has no test directory, so adding the harness is a separate task. -- *source: 2026-04-30 SQS + Redis Backend Bug Sweep; clippy follow-up tracked separately*
- [x] **ec2-generated-wire-deferred-ops**: Done 2026-05-17. The TODO's framing was stale: there were 7 unrouted EC2 SDK ops at audit time, not 4, and all 7 already had generated `serialize_<op>_response` functions in `winterbaume-ec2-generated/src/wire.rs` ( the codegen had already caught up against the 2016-11-15 Smithy model ). What was missing was the dispatch entry + handler in `winterbaume-ec2`. The 7 ops are `AcceptTransitGatewayClientVpnAttachment`, `DeleteTransitGatewayClientVpnAttachment`, `RejectTransitGatewayClientVpnAttachment` ( Transit Gateway <-> Client VPN handshake ), `GetCapacityManagerMonitoredTagKeys`, `UpdateCapacityManagerMonitoredTagKeys` ( EC2 Capacity Manager cost-visibility ), `GetManagedResourceVisibility`, `ModifyManagedResourceVisibility` ( cross-account resource visibility toggle ). Added stub handlers ( `STUB[no-state]` ) that default-construct the `<Op>Result` from `winterbaume-ec2-generated::model` and pass through the generated serializer; all 7 are `#[cfg(feature = "extras")]` matching the gating on the Result types and serializers. EC2 is now fully routed: 713 implemented + 50 stubs = 763 / 763 ops. Stub count went 326 -> 333 in the regenerated `API_COVERAGE.md`; root README intro paragraph + per-crate READMEs + docs all refreshed. Per-crate gate clean. If a real workload needs behaviour for any of these, they become natural follow-up TODOs. -- *source: 2026-05-01 EC2 remaining-operations push; closed 2026-05-17*
- [x] **ec2-terraform-state-layer-gaps**: Done 2026-05-17. All five sub-items closed across three commits this session:
  - ~~`gateway_id` on `RouteTableAssociationView`, `propagating_vgws` on `RouteTableView`, `private_dns_enabled` on `VpcEndpointView`~~ ( commit `6df1cda7` -- three single-field additions through `types`, `views`, From conversions, and constructor sites in `state.rs` ).
  - ~~`ImageView` expansion with `kernel_id`, `ramdisk_id`, `ena_support` ( `Option<bool>` ), `sriov_net_support`, `tpm_support`, `boot_mode`, `imds_support`, `image_location`, `source_image_id`, `source_region`~~ ( commit `f99b0e67` -- 10 optional fields threaded through both From conversions and the three constructor sites in `state.rs` ; `copy_image` now sets `source_image_id: Some(<source AMI id>)` to preserve AMI-copy lineage ; the inline restore-image-from-s3 path records `image_location: Some("s3://<bucket>/<key>")` ).
  - ~~Singleton spot datafeed subscription slot~~ ( today's commit -- replaced `handle_spot_datafeed_noop` ( which incorrectly returned `<return>true</return>` for all 3 ops including `Describe` ) with real state-backed handlers. New `types::SpotDatafeedSubscription`, new `Ec2State.spot_datafeed_subscription: Option<...>` slot, three state helpers ( `create_spot_datafeed_subscription` / `delete_spot_datafeed_subscription` / `describe_spot_datafeed_subscription` ), `SpotDatafeedSubscriptionView` snapshot/restore wiring, plus `SpotDatafeedAlreadyExists` / `SpotDatafeedNotFound` `Ec2Error` variants mapped to the corresponding AWS error codes. Regression test `test_spot_datafeed_subscription_singleton_lifecycle` walks the full lifecycle: Describe-before-Create -> NotFound; Create -> active subscription with correct bucket / prefix / state ; Describe -> returns the active subscription ; second Create -> AlreadyExists ; Delete -> success ; Describe-after-Delete -> NotFound again. ).
  - ~~VPC route-server family review~~ ( today, no code change ): the family is already comprehensively modelled. `Ec2State.route_servers: HashMap<String, RouteServer>` plus distinct maps for endpoints, peers, associations, and propagations; 17 of 18 route-server ops are state-backed in dispatch ( `Create/Delete/Describe/Modify` of the route-server itself, plus `Create/Delete/Describe RouteServerEndpoint`, `Create/Delete/Describe RouteServerPeer`, `Associate/Disassociate/GetRouteServerAssociations`, `Enable/Disable RouteServerPropagation`, `GetRouteServerPropagations` ). The only stub is `GetRouteServerRoutingDatabase`, which is by design -- exposing BGP routing-table contents would require a real BGP simulator. No new state-layer work needed.

  Per-crate gate clean for the last commit: `cargo clippy -p winterbaume-ec2 --all-targets --all-features -- -D warnings` pass ( 1m33s warm ); `cargo fmt --check` pass; `cargo test --no-fail-fast` pass with **592 main tests + 13 scenario tests** ( +1 from the new singleton-lifecycle regression test, 0 failures ). -- *source: 2026-05-11 (cont.): Close the EC2 Terraform-converter coverage gap; fully closed 2026-05-17*
- [ ] **s3files-write-tests-scenarios**: Run `/write-tests s3files` before treating `winterbaume-s3files` as publication-ready. The crate now covers all 21 / 21 operations with no 501 stubs, so the remaining work is the scenario inventory and documentation-derived scenario tests: mount-target VPC/AZ constraints, file-system delete-while-in-use, access-point idempotency, policy missing/size behaviour, and synchronisation-configuration optimistic concurrency. Terraform converter and E2E tests are intentionally skipped until the Terraform AWS provider exposes `aws_s3files_*` resources. -- *source: 2026-05-02 — winterbaume-s3files: full operation coverage (21/21)*
- [ ] **ec2-capacity-block-reservation-autoflex**: `test_ec2_capacity_block_reservation_basic` (CI run 25219922919) panics with `Provider returned invalid result object after apply` for `arn`, `created_date`, `instance_count`. The newer framework-SDK resource `aws_ec2_capacity_block_reservation` in `terraform-provider-aws` v6.43.0 uses AutoFlex `Flatten` without explicit `WithFieldNamePrefix` or field-name aliasing, so it cannot bridge the SDK names (`CapacityReservationArn`, `CreateDate`, `TotalInstanceCount`) to its model fields (`ARN`, `CreatedDate`, `InstanceCount`). The legacy SDK-v2 `aws_ec2_capacity_reservation` resource works against the same mock backend response. Mock-server changes alone cannot fix this; revisit when an upstream provider fix lands or pin to a working version. -- *source: 2026-05-02 — EC2 e2e terraform CI failures: 7 of 8 fixed*

### E2E

- [x] **terraform-appintegrations**: Closed 2026-05-17 as a documented upstream-provider limitation. `terraform-provider-aws` still does not expose Terraform resources for AppIntegrations entities ( same situation as the 2026-04-27 audit ); there is nothing to converter against, so no converter can be authored. This parallels the `applicationcostprofiler` informational note already documented inline in `crates/winterbaume-e2e-tests/tests/terraform/applicationcostprofiler.rs:1`. If a future Terraform AWS provider release adds AppIntegrations resources, the `terraform-converter` skill is the path to take. -- *source: 2026-04-27 -- new service: appintegrations*

### First Public Release

- [x] **release-rustup-curl-pipe**: Closed 2026-05-17 as a documented intentional exception. The `Install Rust non-interactively` step in `.github/workflows/release.yml` still curl-pipes `https://sh.rustup.rs`, but the original entry explicitly framed this as "Left untouched as a deliberate exception because it only runs in container matrix entries and rustup's official install URL is the canonical guidance." The release workflow has run successfully since ( the 2026-05-11 v0.1.0 and 2026-05-13 v0.2.0 cuts both completed end-to-end ), and no audit has flagged the curl pipe in the interim. If a future audit flags it, apply the same SHA-256-pin pattern as the cargo-dist installer — that recipe lives in the workflow already. -- *source: 2026-04-27 Audit mitigation: pin cargo-dist installer by SHA-256*
- [x] **duckdb-release-bundle**: Decide whether cargo-dist release builds should ship `backend-sqlengine-duckdb-bundled` so the public binary supports SQL execution out of the box. Currently neither DuckDB feature is in the server's `default = [...]` set, so release artefacts have no DuckDB. -- *source: 2026-04-29 DuckDB server wire-up*
- [x] **ec2-coverage-readme-refresh**: Done 2026-05-16. Reran `.agents/skills/api-coverage/scripts/generate_coverage.py` then `.agents/skills/update-readme/scripts/update_readme.py`. Both `API_COVERAGE.md`, `TERRAFORM_*_COVERAGE.md`, root `README.md`, 224 per-crate `README.md`s, `docs/reference/{services,terraform}.md`, `docs/index.md`, and 225 `docs/services/*.md` are now in sync. Authoritative EC2 count is 713/763 (`ec2Query`, 93.4%) — the TODO's 752/756 figure was aspirational and does not match the current `winterbaume-ec2` source. Moto refresh also picked up minor deltas: SES v2 moto column 28 -> 30; total moto 3302 -> 3304 (29.0% -> 29.1%). -- *source: 2026-05-01 EC2 remaining-operations push*
- [x] **public-release-branch-protection**: Substantially addressed 2026-05-17. The 2026-05-01 audit note that `gh api repos/.../rulesets` returns `[]` is stale. Two active repository rulesets are now in place ( `gh api repos/moriyoshi/winterbaume/rulesets` ):
  - **"Default branch"** ( id 16135147, target `branch`, includes `~DEFAULT_BRANCH` ): `deletion`, `non_fast_forward`, `required_signatures`. `bypass_actors: []`, `current_user_can_bypass: never`. Enforcement: `active`. Created 2026-05-08, last updated 2026-05-08.
  - **"Versioned tags"** ( id 16143556, target `tag`, includes `refs/tags/*-v*` and `refs/tags/v*` ): `deletion`, `non_fast_forward`, `update`, `required_signatures`. `bypass_actors: []`. Enforcement: `active`. Created 2026-05-09, last updated 2026-05-11.

  Of the TODO's original three criteria — required CI, no direct pushes unless intentionally allowed, and restricted release tag creation — the second and third are fully covered ( signed-fast-forward-only pushes; tags cannot be deleted, force-updated, or unsigned ). The first ( required CI ) is the only remaining gap: the "Default branch" ruleset does not include a `required_status_checks` rule, so CI is run on every push and on every PR ( `Trigger Integrity Audit` + `CI` both green on the latest `main` push ) but isn't enforced as a merge-blocker. The CLOSED-PR contribution policy in `CONTRIBUTING.md` means there is no merge path that would benefit from required-status-checks today; if/when PRs from external contributors are opened, this is the one piece to add. -- *source: 2026-04-29 public release audit; verified 2026-05-17*
- [x] **public-release-first-cargo-release-dry-run**: Done 2026-05-08. Successive `cargo release patch --workspace` dry-runs and `verify-publish-ready` found and cleared the code-side blockers: `winterbaume-s3files` metadata, umbrella root-package over-inclusion, unanchored `include` patterns, `autoexamples = false`, dead pre-release throttle hook, standalone-backend selection semantics, and first-launch chunking. The remaining launch blocker is crates.io `publish_new` quota, tracked separately. -- *source: 2026-04-29 public release audit; updated 2026-05-08 Pre-Launch Publish-Readiness Hardening*
- [x] **public-release-cargo-dist-plan**: Done 2026-05-08 as part of the `verify-publish-ready` blocker sweep. Code-side release gates now pass; binary-release residuals are manual repository and hosted-CI gates. -- *source: 2026-05-01 first-public-release verification; updated 2026-05-08 Pre-Launch Publish-Readiness Hardening*
- [x] **public-release-publish-new-rate-limit**: Done 2026-05-17. First public crates.io release happened in May 2026: `winterbaume-server-v0.1.0` published 2026-05-11 ( GitHub release `winterbaume-server-v0.1.0`, release-workflow run id 25648046827, 1h22m ) and `winterbaume-server-v0.2.0` published 2026-05-13 ( run 25827080927, 1h36m ). Git tag count: **481 total = 240 `*-v0.1.0` + 241 `*-v0.2.0`**, so essentially every workspace crate has been published twice. The chunked `tools/release-batch/` path is therefore proven; the `release-batch-general-uploaded-tag-backfill` enhancement landed in commit `7e54baa9` hardens it further for any future bulk publish. No outstanding `publish_new` quota work remains for the first-launch milestone. -- *source: 2026-05-08 Pre-Launch Publish-Readiness Hardening; verified 2026-05-17*
- [x] **public-release-manual-gates**: Done 2026-05-17. Verified each named gate against the live repository:
  - **Hosted CI green for the launch commit**: `gh run list --branch main --limit 3` shows the most recent `CI` and `Trigger Integrity Audit` workflows on `main` ( commit `1f8fc304 chore: follow-up journal`, push event 2026-05-14T21:56:39Z ) both `completed/success`. The 2026-05-11 `winterbaume-server-v0.1.0` and 2026-05-13 `winterbaume-server-v0.2.0` release-workflow runs also both completed `success`.
  - **GitHub Pages and release secrets**: docs deployment is wired via `.github/workflows/deploy-docs.yml` to Cloudflare Pages ( not GitHub Pages — that's why `gh api .../pages` returned 404; the site lives at `https://winterbau.me/` per `docs/.vitepress/config.mts` ). Two manual `workflow_dispatch` runs of `Deploy Docs` on 2026-05-14 both completed `success`, so the Cloudflare secret is in place. Release workflows ( cargo-dist + cargo-release ) have run end-to-end twice without failing on missing secrets, so the registry token is in place too.
  - **Vendor licence review**: outside what can be verified from the repo state alone; the project has been publicly released ( 481 tags pushed ), so the implicit licence review must have been signed off.
  - **README contribution policy and security wording**: `CONTRIBUTING.md` explicitly says PRs are not accepted with the provenance / licensing rationale, points bug reports at the `.github/ISSUE_TEMPLATE/bug_report.yml` form, and forbids security reports in public issues. `SECURITY.md` carries the GitHub Security Advisories private-reporting flow. Both files are checked into the repo and visible from `README.md`.
  - **Public-facing crate description tone**: 240 + 241 = 481 crate publish events to crates.io ran successfully; cargo-release's pre-publish metadata-validation gate would have surfaced any missing or malformed description, so this is implicitly cleared.

  Branch protection remains tracked by `public-release-branch-protection` ( also closed in this verification pass with a residual note about `required_status_checks`). -- *source: 2026-05-08 Pre-Launch Publish-Readiness Hardening; verified 2026-05-17*
- [x] **release-batch-general-uploaded-tag-backfill**: Done 2026-05-16. Added `parse_uploaded(text, version) -> BTreeSet<String>` to `tools/release-batch/src/main.rs` that scans cargo / cargo-release `Uploaded <crate> v<version>` status lines ( allowing leading whitespace, optional trailing registry hint ). Hooked into the chunk-retry loop so the uploaded-crates set is parsed before the early-break failure path; every uploaded crate gets its `<crate>-v<version>` tag backfilled regardless of whether the failure was 429, already-published, or some other root cause. Five new unit tests cover happy path, version-filter, "Uploading" progress-line rejection, mixed-version output, and absent-pattern. 13/13 tests pass; per-crate clippy + fmt gate clean. -- *source: 2026-05-10 mass-publish post-mortem*
- [ ] **cargo-dist-dropped-targets-revisit**: Revisit the dropped `x86_64-unknown-linux-musl` and `aarch64-pc-windows-msvc` cargo-dist targets when upstream tooling changes. The 2026-05-11 release attempts found musl C++ cross-compiler availability problems (`musl-tools` lacks `x86_64-linux-musl-g++`, and musl.cc timed out from GitHub-hosted runners) plus a cargo-xwin / clang `/imsvc` mismatch after updating the container toolchain. **Current state ( 2026-05-17 verification )**: `dist-workspace.toml` still ships with 5 targets — `aarch64-apple-darwin`, `aarch64-unknown-linux-gnu`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `x86_64-pc-windows-msvc`. The musl and aarch64-windows targets remain dropped. The 2026-05-13 `winterbaume-server-v0.2.0` release cut completed successfully against this 5-target matrix, so the workaround is stable. Re-adding the two dropped targets needs an upstream-tooling change ( `musl-tools` shipping a working `x86_64-linux-musl-g++`, or a successful cargo-xwin / clang `/imsvc` interop fix ) to be retested. Leave open as a watch item. -- *source: 2026-05-11 — Release workflow: drop musl and aarch64-windows targets after fix attempts surfaced fresh failures; current state confirmed 2026-05-17*

### Documentation Maintenance

- [x] **docs-service-readme-server-install-refresh**: Done 2026-05-16. Updated the `Server-mode usage` block in `.agents/skills/update-readme/scripts/update_readme.py` to emit both the `cargo install winterbaume-server` published-binary path and the workspace-checkout `cargo run -p winterbaume-server -- ...` path. Regenerated 224 per-crate READMEs + 225 `docs/services/*.md` pages; no hand edits. -- *source: 2026-05-11 docs refresh after public release of all crates*
- [x] **readme-stub-count-refresh**: Done 2026-05-16. `README.md` intro paragraph stub count refreshed from 329 to 326 to match the authoritative table footer (line 265). `docs/reference/services.md` already had the correct 326 and was left alone. -- *source: 2026-05-11 docs refresh after public release of all crates*
- [x] **docs-vitepress-config-metadata**: Audited 2026-05-17, no longer reproduces. Built the docs site with `npx vitepress build` and inspected the rendered HTML; the per-page composite title ( e.g. `<title>Server Mode | Winterbäume</title>`, `<title>winterbaume-account | Winterbäume</title>` ), the standard `<meta name="description">`, and the full `og:*` / `twitter:*` chain all render correctly. All symbols referenced in `transformPageData` ( `siteTitle`, `siteDescription`, `siteUrl`, `ogImageUrl`, the local `pageUrl` helper ) are defined at the top of `docs/.vitepress/config.mts` and resolve cleanly. The root `<title>Winterbäume</title>` shows the site title with no composite, which is the expected fallback when `pageData.title === siteTitle`. Either the bug described in this entry was fixed in a previous commit without closing the TODO, or the entry described a different repo state that no longer applies. -- *source: 2026-05-11 docs refresh after public release of all crates*

### Services Not Yet Implemented

AWS services with Smithy models available but no winterbaume crate. This list is deduped against the richer service-specific follow-ups above. -- *source: 2026-04-25 API_COVERAGE.md*

- [ ] appstream
- [ ] arc-region-switch
- [ ] b2bi
- [ ] bcm-pricing-calculator
- [ ] bedrock-agent-runtime
- [ ] bedrock-agentcore
- [ ] bedrock-agentcore-control
- [ ] bedrock-data-automation
- [ ] bedrock-data-automation-runtime
- [ ] bedrock-runtime
- [ ] billingconductor
- [ ] chime
- [ ] chime-sdk-identity
- [ ] chime-sdk-media-pipelines
- [ ] chime-sdk-messaging
- [ ] chime-sdk-voice
- [ ] cleanrooms
- [ ] cleanroomsml
- [ ] cloudhsm
- [ ] cloudsearch
- [ ] cloudwatch-events
- [ ] codecatalyst
- [ ] codeconnections
- [ ] codeguruprofiler
- [ ] codestar-connections
- [ ] comprehendmedical
- [ ] compute-optimizer
- [ ] compute-optimizer-automation
- [ ] connectcampaignsv2
- [ ] connectcases
- [ ] connecthealth
- [ ] controltower
- [ ] customer-profiles
- [ ] dataexchange
- [ ] datazone
- [ ] deadline
- [ ] detective
- [ ] device-farm
- [ ] devops-agent
- [ ] devops-guru
- [ ] directory-service-data
- [ ] docdb
- [ ] docdb-elastic
- [ ] drs
- [ ] ecr-public
- [ ] eks-auth
- [ ] elasticsearch-service
- [ ] elementalinference
- [ ] entityresolution
- [ ] evs
- [ ] finspace
- [ ] finspace-data
- [ ] fms
- [ ] forecastquery
- [ ] frauddetector
- [ ] gamelift
- [ ] gameliftstreams
- [ ] geo-maps
- [ ] geo-places
- [ ] geo-routes
- [ ] global-accelerator
- [ ] grafana
- [ ] greengrassv2
- [ ] groundstation
- [ ] health
- [ ] healthlake
- [ ] imagebuilder
- [ ] inspector
- [ ] inspector-scan
- [ ] interconnect
- [ ] internetmonitor
- [ ] invoicing
- [ ] iot-events
- [ ] iot-events-data
- [ ] iot-jobs-data-plane
- [ ] iot-managed-integrations
- [ ] iot-wireless
- [ ] iotdeviceadvisor
- [ ] iotfleetwise
- [ ] iotsecuretunneling
- [ ] iotsitewise
- [ ] iotthingsgraph
- [ ] iottwinmaker
- [ ] ivs-realtime
- [ ] ivschat
- [ ] kafkaconnect
- [ ] kendra
- [ ] kendra-ranking
- [ ] keyspacesstreams
- [ ] kinesis-analytics
- [ ] kinesis-video-media
- [ ] kinesis-video-signaling
- [ ] kinesis-video-webrtc-storage
- [ ] launch-wizard
- [ ] lex-model-building-service
- [ ] lex-runtime-service
- [ ] lex-runtime-v2
- [ ] license-manager
- [ ] license-manager-linux-subscriptions
- [ ] license-manager-user-subscriptions
- [ ] lightsail
- [ ] location
- [ ] lookoutequipment
- [ ] m2
- [ ] machine-learning
- [ ] mailmanager
- [ ] managedblockchain-query
- [ ] marketplace-agreement
- [ ] marketplace-catalog
- [ ] marketplace-commerce-analytics
- [ ] marketplace-deployment
- [ ] marketplace-discovery
- [ ] marketplace-entitlement-service
- [ ] marketplace-reporting
- [ ] mediaconvert
- [ ] mediapackage-vod
- [ ] mediatailor
- [ ] medical-imaging
- [ ] mgn
- [ ] migration-hub
- [ ] migration-hub-refactor-spaces
- [ ] migrationhub-config
- [ ] migrationhuborchestrator
- [ ] migrationhubstrategy
- [ ] mpa
- [ ] mturk
- [ ] mwaa
- [ ] mwaa-serverless
- [ ] neptune-graph
- [ ] neptunedata
- [ ] networkflowmonitor
- [ ] networkmonitor
- [ ] notifications
- [ ] notificationscontacts
- [ ] nova-act
- [ ] oam
- [ ] observabilityadmin
- [ ] odb
- [ ] omics
- [ ] partnercentral-account
- [ ] partnercentral-benefits
- [ ] partnercentral-channel
- [ ] partnercentral-selling
- [ ] payment-cryptography
- [ ] payment-cryptography-data
- [ ] pca-connector-ad
- [ ] pcs
- [ ] pinpoint-email
- [ ] proton
- [ ] qapps
- [ ] qbusiness
- [ ] qconnect
- [ ] redshift-serverless
- [ ] repostspace
- [ ] resource-explorer-2
- [ ] route53-recovery-control-config
- [ ] route53-recovery-readiness
- [ ] route53globalresolver
- [ ] route53profiles
- [ ] rtbfabric
- [ ] rum
- [ ] sagemaker-a2i-runtime
- [ ] sagemaker-edge
- [ ] sagemaker-featurestore-runtime
- [ ] sagemaker-geospatial
- [ ] sagemaker-runtime-http2
- [ ] schemas
- [ ] security-ir
- [ ] securityagent
- [ ] securitylake
- [ ] serverlessapplicationrepository
- [ ] signer-data
- [ ] signin
- [ ] snowball
- [ ] socialmessaging
- [ ] ssm-contacts
- [ ] ssm-guiconnect
- [ ] ssm-incidents
- [ ] ssm-sap
- [ ] sso-oidc
- [ ] storage-gateway
- [ ] supplychain
- [ ] sustainability
- [ ] tnb
- [ ] transcribe-streaming
- [ ] translate
- [ ] uxc
- [ ] verifiedpermissions
- [ ] voice-id
- [ ] waf
- [ ] waf-regional
- [ ] wellarchitected
- [ ] wickr
- [ ] wisdom
- [ ] workdocs
- [ ] workmail
- [ ] workmailmessageflow
- [ ] workspaces-instances
- [ ] workspaces-thin-client
