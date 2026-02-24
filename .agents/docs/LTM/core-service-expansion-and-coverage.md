# Core Service Expansion and Coverage

## Summary

Winterbaume grew from an STS proof of concept into a large multi-service AWS mock workspace with broad protocol coverage and test depth. The durable pattern is that service expansion, API coverage work, and coverage verification moved in waves: initial core services, bulk crate creation, targeted operation filling, behavioural and Terraform-oriented validation, then late-April batches of smaller new AWS service crates. By early April 2026 this included service additions such as EC2, S3 Control, S3 Vectors, API Gateway, API Gateway Management, EMR, Amplify, and full S3 Tables coverage; by 2026-04-26 it also included new crates such as AIOps, AppConfig Data, AppFabric, AppFlow, Application Cost Profiler, Cloud Control, Keyspaces, and Roles Anywhere.

## Key Facts

- The first implementation phases established the base stack: STS, IAM, S3, SQS, DynamoDB, KMS, and a standalone HTTP server.
- Phase 6 added 20 more service crates and expanded protocol coverage to awsQuery, awsJson1.0, awsJson1.1, restJson1, restXml, and rpc-v2-cbor.
- The March 2026 service-expansion batches pushed the workspace to 136 service crates, and early-April additions brought it to 137 services with generated serialiser support across all mapped crates.
- Coverage reporting moved from 3,777 / 7,824 operations (48.3%) on 2026-03-30 to 3,991 / 8,580 (46.5%) on 2026-04-01, still ahead of moto's 2,644 / 8,580 (30.8%).
- Early-April service work added new crates for API Gateway Management, EMR, and Amplify, while upgrading existing services such as GuardDuty, CodeBuild, and OpenSearch from partial or stubbed behaviour to broader stateful coverage.
- A large 2026-04-03 coverage sprint raised detected workspace coverage to 5,812 / 10,456 operations (55.6%), with moto at 31.1%.
- The follow-on coverage waves on 2026-04-03 and 2026-04-04 pushed integration coverage from 4,264 / 5,809 implemented operations (73.4%) to 4,978 / 5,809 (85.7%), and raised E2E coverage from 25 to 40 services.
- Coverage reporting is directionally useful but not perfect. Generic handlers and non-standard dispatch patterns can make `API_COVERAGE.md` undercount implemented operations.
- The durable workflow is: add or expand a service, port parity or docs-driven tests, then re-run coverage reporting with skepticism about stale entries.
- `// STUB[category]: reason` is now the repo-wide convention for intentionally unimplemented handlers, and coverage reporting excludes those stubs from implemented-operation counts.
- New service additions now follow a pragmatic root-resource-first rule: full coverage for small surfaces, and explicit 501 stubs for deeper resource families deferred from larger services.

## Details

### Foundational phases

The 2026-02-24 phases established the project shape:

- Phase 1: STS end-to-end with hand-written awsQuery handling and the `MockAws` / `MockAwsClient` core.
- Phase 2: IAM and multi-service routing in one in-memory mock instance.
- Phase 3: S3 with REST-XML routing, bucket and object state, and virtual-hosted plus path-style parsing.
- Phase 4: SQS and DynamoDB with awsJson1.0 routing and JSON target dispatch.
- Phase 5: KMS plus a standalone server mode.
- Phase 6: 20 additional crates across several protocol families.

Those early phases established the reusable pattern for later service work:

- each crate owns protocol-aware `handlers.rs`
- each crate persists state in `state.rs`
- integration tests use real `aws-sdk-rust` clients against the in-process mock

### Large-scale crate expansion

On 2026-03-16 through 2026-03-20, service growth shifted from a few manually built services to batch expansion:

- batches 5 through 26 added or stubbed many moto-supported AWS services
- the project reached 136 service crates
- the stub threshold dropped sharply as services moved from placeholder state to handler-backed implementations

Important durable outcomes:

- protocol handling patterns were standardized enough to support wide batch creation
- service-map coverage in smithy codegen became essential infrastructure
- workspace scale made manual reasoning from memory unreliable; `API_COVERAGE.md` and generated serialiser support became required coordination tools

### Coverage push strategy

Coverage improvements happened in distinct passes:

- small early gap-closing passes on services with 1 to 10 missing operations
- large multi-phase operation pushes on medium and large services
- a later batch 8 pass that cleaned up genuine `W[ ] M[x]` gaps while identifying stale coverage entries

The durable lesson is that coverage work should separate three cases:

1. real missing handlers
2. already-implemented operations hidden by coverage-script heuristics
3. behaviourally incorrect handlers that look covered but fail parity

### Caveats about reported coverage

Multiple journal entries show the same pattern:

- `API_COVERAGE.md` can be stale
- generic dispatch hides handlers from naive operation detection
- services such as Greengrass, SecurityHub, LakeFormation, and Kinesis Video Archived Media appeared under-covered even when handlers already existed

Treat coverage numbers as a prioritisation input, not ground truth.

### Route53 expansion as a coverage model

The 2026-03-29 Route53 integration push is a good example of mature coverage work:

- existing lifecycle coverage was already broad
- the expansion targeted error paths, duplicate caller references, list filters, and less-common endpoints
- all 87 tests passed after adding 36 new scenarios

The durable lesson is that once a service's basic CRUD surface is covered, the highest-value next pass is usually error behaviour, alternate identifiers, and provider-adjacent edge cases rather than raw operation count alone.

### STUB annotation convention and coverage detection

By 2026-03-30, the workspace needed a cleaner distinction between genuinely implemented handlers and intentional placeholders. The adopted convention is:

```rust
// STUB[category]: reason
fn handle_xxx(...) -> MockResponse { ... }
```

Coverage scripts now treat these as explicit non-implementations instead of accidental misses. This makes coverage output more honest and reduces churn when a service has broad routing but only a subset of real behaviour behind it.

Representative stub categories used in the sweep:

- `no-state`
- `no-engine`
- `no-telemetry`
- `org-integration`
- `s3-integration`

### Detection heuristics still need maintenance

The same 2026-03-30 sweep also showed that coverage drift can come from the detector rather than the service:

- acronym expansion had to learn `HLS`, `DASH`, `URL`, and `URI`
- inline async handlers, such as Glacier blob operations, needed comment markers so the detector could count them
- Inspector2 had 37 inline dispatch arms that were functionally implemented but invisible until they were refactored into `self.handle_*` methods

This is durable guidance: when coverage numbers look implausible, inspect the detector and dispatch shape before assuming the handler is missing.

### 2026-03-30 large coverage push

The large March 30 push raised Winterbaume coverage to 3,777 / 7,824 operations (48.3%). Twenty services saw material increases, including:

- Inspector2: 19 / 75 -> 75 / 75
- Cost Explorer: 4 / 47 -> 47 / 47
- App Mesh: 4 / 38 -> 38 / 38
- Glacier: ~0 / 33 -> 33 / 33
- S3: 77 / 107 -> 107 / 107
- Kinesis Analytics V2: 4 / 33 -> 33 / 33
- IVS: 6 / 35 -> 35 / 35
- Kinesis Video: 4 / 32 -> 32 / 32
- RAM: 9 / 35 -> 35 / 35
- Service Catalog AppRegistry: 4 / 24 -> 24 / 24

This pass reinforces three durable ideas:

1. coverage tools need explicit conventions (`STUB[...]`, acronym maps, inline markers)
2. dispatch shape affects reported coverage
3. broad operation-count growth is still valuable when paired with tests and clear stub accounting

### Early-April 2026 expansion and coverage deepening

The next wave shifted from broad batch cleanup to deeper service maturity and a few new service introductions:

- `winterbaume-s3control` added a restXml S3 Control surface with access points plus account-level public access block operations. The durable implementation lesson is that region and account extraction can depend on headers and SigV4 context rather than simple host parsing.
- `winterbaume-s3vectors` reached 19 / 19 implemented operations. Its Smithy model says `restJson1`, but the actual routing pattern is mostly `POST /OperationName`, which is a reminder that protocol family alone does not determine dispatch shape.
- `winterbaume-apigateway` established the initial REST API and resource CRUD surface for API Gateway v1, including the rule that `CreateRestApi` must materialize the root `/` resource immediately.
- `winterbaume-s3tables` moved from 15 / 49 to 49 / 49 operations. The main durable lesson was that generated coverage detection depends on handler names matching the full Smithy operation name exactly.
- `winterbaume-ec2` grew from the initial VPC and networking surface into compute and storage flows, then further to a 214 / 756 operation surface that exceeds moto's practical EC2 coverage. That work combined docs-driven tests, Terraform E2E validation, generated `ec2Query` serialisers, and later parity-style lifecycle coverage.

By the 2026-04-01 coverage refresh, Winterbaume reported 3,991 / 8,580 operations (46.5%) across 137 services, with moto at 2,644 / 8,580 (30.8%).

### 2026-04-01 to 2026-04-03 service additions and stateful upgrades

The next journal wave added several new durable implementation patterns:

- `winterbaume-apigatewaymanagementapi` implemented the full 3 / 3 WebSocket connection-management surface. The durable lessons are that routing must find the `@connections` path segment regardless of stage prefix, timestamps must serialize as RFC3339 / `__timestampIso8601`, and the service can expose full `StatefulService` snapshot, restore, and merge behaviour even for connection-like transient resources.
- `winterbaume-emr` added an `awsJson1_1` service with `ElasticMapReduce` target dispatch, 30 implemented operations, typed state views, Terraform converters, and an example binary. The durable pattern is that medium-sized services can land with both stateful handlers and Terraform extraction in the same pass.
- `winterbaume-guardduty` moved important resource families from `STUB[no-state]` placeholders to real stateful behaviour for IP sets, threat-intel sets, and resource tags. The notable behavioural constraint is that `UntagResource` must read the plural `tagKeys` query parameter emitted by the SDK.
- `winterbaume-codebuild` added stateful report-group lifecycle support. This is a representative case where replacing 501 or empty no-state handlers only required one new durable type, one indexed state collection, and a small set of CRUD helpers plus wire-conversion helpers.
- `winterbaume-amplify` added a new restJson1 crate with 23 / 37 operations, typed state views, Terraform converters, and a note that composite-key maps in views can use `\x00`-delimited strings to remain serde-friendly without leaking internal map structure.
- `winterbaume-opensearch` expanded from 11 / 82 to 46 / 82 operations by adding stateful VPC endpoints, data sources, packages, cross-cluster connections, and related view coverage. The durable implementation rule was to push all success responses through generated `wire::serialize_*_response()` helpers rather than hand-crafted payloads.

### 2026-04-03 coverage expansion sprint

The 2026-04-03 sprint is the strongest example of how the repo now does broad coverage work:

- parallel service expansion across 17 crates pushed reported coverage from 5,497 / 10,456 to 5,812 / 10,456 operations
- multiple services reached or approached full coverage in one pass, including API Gateway v1, CloudFront, RDS, S3 Control, Redshift, Lex v2, NetworkManager, WorkSpaces, and OpenSearch
- build hygiene remained part of the work: CloudFront helper mismatches, OpenSearch Terraform-view drift, and WorkSpaces converter type mismatches were fixed as part of the expansion rather than deferred

This reinforced an existing repo rule: coverage numbers are useful only when paired with successful builds, integration tests, and skepticism about detector false negatives.

### Late 2026-04-03 bulk coverage waves

After the main sprint, coverage work continued in two broader waves that focused on depth rather than just new handlers.

The first wave combined large integration-test expansions with some new Terraform modules:

- Redshift grew from 105 to 190 integration tests, Lex v2 from 90 to 147, and several smaller services reached full or near-full integration coverage in the same pass
- AppConfig, ECR, WorkSpaces, SSM, CodeCommit, NetworkManager, WorkSpacesWeb, EKS, BedrockAgent, Cognito IDP, Transfer, and CloudWatch Logs all received substantial late-stage regression suites
- the same session added new Terraform E2E coverage for App Runner, Chatbot, and Amplify

The next wave targeted every non-deferred service still below 60% integration coverage:

- Timestream Query, Redshift Data, Cognito Identity, RDS Data, and CodeCommit reached 100%
- WorkSpaces, STS, SSM, BedrockAgent, AppConfig, NetworkManager, EKS, Logs, Transfer, ELBv2, ECR, Cognito IDP, CloudWatch, and Lambda all crossed the 60% threshold, usually well beyond it
- DSQL, API Gateway v2, and DynamoDB Streams were then used as cleanup cases in the second-wave pass, which also recorded that detector heuristics can miss real tests when local variable names do not match the coverage regex

The durable lesson is that once raw service coverage is broadly in place, the remaining work shifts to:

1. deeper lifecycle and error-path assertions
2. tooling fixes for false negatives in coverage detection
3. targeted E2E additions for resource-provider behaviour

### 2026-04-04 enhancement wave

The third wave deliberately targeted services that were already between 60% and 80% integration coverage and pushed them towards mature suites:

- EMR, CloudFormation, EFS, and Batch reached 100%
- StepFunctions, VPC Lattice, ECS, Organizations, GuardDuty, Inspector2, and Lambda all moved materially upward
- Autoscaling completed a full 0% -> 100% jump with a newly created integration test file
- EC2 climbed from 47.3% to 78.4% through another deep lifecycle pass

The same wave also widened Terraform E2E coverage to 40 services and 166 resource types. One operational lesson from that pass is that coverage reporting needs its own maintenance work:

- missing Terraform resource-prefix mappings hid 15 service coverages and 174 tests until the mapping tables were refreshed
- detector false negatives are a normal part of workspace-scale coverage work and should be treated as tooling debt, not as proof that a service is unimplemented

### S3 multipart upload and Route53 parity (2026-03-28)

To close the moto parity gap, 9 S3 operations were added: `AbortMultipartUpload`, `CompleteMultipartUpload`, `CreateMultipartUpload`, `ListMultipartUploads`, `ListObjectVersions`, `ListParts`, `SelectObjectContent`, `UploadPart`, `UploadPartCopy`. This brought S3 to implementing every operation moto marks as covered (78 operations, 72.9% of total API surface).

Route53 remained at full moto parity for covered operations (30 operations, 42.3%).

Both services benefited from the restXml request deserializer generation work, which replaced hand-written XML parsing in handlers with generated `wire.rs` functions.

### 2026-04-19 Athena tag operations and moto parity

Athena reached moto parity at 25/70 operations with the addition of `TagResource` and `UntagResource`. These operations reused the existing tagging infrastructure (`state.tags`, `resource_exists()`, `extract_tags()`) already in place from `ListTagsForResource`, requiring minimal new code. Four new integration tests cover the full tag lifecycle, overwrite semantics, and error paths for non-existent resources.

### Integration-test authoring lessons from 2026-04-21

The 2026-04-21 fixes in Cost Explorer and DynamoDB reinforced two durable testing rules:

- create prerequisite resources before testing describe or read APIs that require a server-generated ARN or identifier
- assert the lifecycle status the current mock actually models, not the status of the eventual terminal state

The specific examples were:

- Cost Explorer backfill creation should begin in `PROCESSING`, not jump directly to `SUCCEEDED`
- DynamoDB export and import describe tests must first call the create APIs, capture the returned ARN, and then describe that real resource rather than using invented hard-coded ARNs

This is a recurring batch-generation failure mode. Tests that skip setup can look plausible but end up asserting against resources that never existed in state.

### 2026-04-25 and 2026-04-26 new service batches

Late-April service work added two batches of new crates.

The 2026-04-25 batch added six services:

| Service | Crate | Protocol | Implemented ops | Tests |
|---------|-------|----------|-----------------|-------|
| AWS Outposts | `winterbaume-outposts` | restJson1 | 13 / 35 | 18 |
| IAM Access Analyzer | `winterbaume-accessanalyzer` | restJson1 | 11 / 37 | 17 |
| Fault Injection Simulator | `winterbaume-fis` | restJson1 | 8 / 26 | 13 |
| Cloud Control API | `winterbaume-cloudcontrol` | awsJson1_0 | 8 / 8 | 18 |
| Amazon Keyspaces | `winterbaume-keyspaces` | awsJson1_0 | 19 / 19 | 16 |
| IAM Roles Anywhere | `winterbaume-rolesanywhere` | restJson1 | 30 / 30 | 22 |

This batch added 89 implemented operations and 104 integration tests. Cloud Control, Keyspaces, and Roles Anywhere reached full operation coverage immediately.

The 2026-04-26 batch added six more services:

| Service | Crate | Protocol | Implemented ops | Tests | Scope |
|---------|-------|----------|-----------------|-------|-------|
| AWS AIOps | `winterbaume-aiops` | restJson1 | 11 / 11 | 11 | investigation groups, policies, tags |
| Amplify Backend | `winterbaume-amplifybackend` | restJson1 | 4 / 31 | 6 | backend lifecycle only |
| AppConfig Data | `winterbaume-appconfigdata` | restJson1 | 2 / 2 | 6 | configuration session and token rotation |
| AppFabric | `winterbaume-appfabric` | restJson1 | 7 / 26 | 6 | AppBundle lifecycle and tags |
| AppFlow | `winterbaume-appflow` | restJson1 | 10 / 25 | 6 | Flow lifecycle, start/stop, tags |
| Application Cost Profiler | `winterbaume-applicationcostprofiler` | restJson1 | 6 / 6 | 6 | full service surface |

This batch added 40 implemented operations and 41 integration tests. AIOps, AppConfig Data, and Application Cost Profiler reached full operation coverage. Amplify Backend, AppFabric, and AppFlow intentionally landed root resource coverage first, with deeper resource families returning 501 until a real caller requires them.

A later 2026-04-26 continuation added two more crates and an EC2 expansion:

| Service | Crate | Protocol | Implemented ops | Tests | Scope |
|---------|-------|----------|-----------------|-------|-------|
| AWS Artifact | `winterbaume-artifact` | restJson1 | 8 / 8 | 7 | full service surface; AWS-managed report and agreement corpus |
| AWS Auto Scaling Plans | `winterbaume-autoscalingplans` | awsJson1_1 | 6 / 6 | 7 | full service surface; ApplicationSource and ScalingInstructions stored as opaque `serde_json::Value` |

Plus three EC2 additions in the same session:

- **PlacementGroup**: `CreatePlacementGroup` ( strategy `cluster|spread|partition`, `partition_count` 2-7 for partition strategy, uniqueness on `group_name` ), `DeletePlacementGroup` ( lookup by name ), `DescribePlacementGroups`. Three error variants in `Ec2Error` ( `PlacementGroupNotFound`, `PlacementGroupAlreadyExists`, `InvalidParameterValue` ) mapped to the AWS-shaped wire codes `InvalidPlacementGroup.Unknown`, `InvalidPlacementGroup.Duplicate`, `InvalidParameterValue`. 8 new integration tests.
- **NetworkInterfacePermission**: `CreateNetworkInterfacePermission` ( permission must be `INSTANCE-ATTACH` or `EIP-ASSOCIATE` ), `DeleteNetworkInterfacePermission`, `DescribeNetworkInterfacePermissions`, `ResetNetworkInterfaceAttribute` ( wired as a no-op stub ). 3 new integration tests.
- **InstanceConnectEndpoint**: `CreateInstanceConnectEndpoint` ( derives `vpc_id` and AZ from the supplied subnet; `InvalidSubnetID.NotFound` on unknown subnet; ARN built from region + account + id ), `DeleteInstanceConnectEndpoint`, `DescribeInstanceConnectEndpoints`. 3 new integration tests.

`Ec2StateView` and `Ec2Counters` extended with the three new collections plus their counters; `From` impls in both directions and merge entries added. EC2 integration test count grew from 117 to 123.

### 2026-04-27 EC2 Coverage Continuation

Three further EC2 expansions landed in the same session as the new-service batch:

- **CapacityReservation**: `CreateCapacityReservation`, `CancelCapacityReservation`, `DescribeCapacityReservations`, `ModifyCapacityReservation`. New domain type with id, ARN, owner, instance type/platform, AZ, tenancy, instance counts, EBS optimisation, ephemeral storage, state, start/end dates, end_date_type, instance match criteria, optional outpost ARN, optional placement group ARN, tags. Validation: `instance_count > 0`, end_date_type in `{unlimited, limited}`. New `Ec2Error::CapacityReservationNotFound` mapped to `InvalidCapacityReservationId.NotFound`. 5 new integration tests; total 128. Open follow-ups in TODO.md: `CreateCapacityReservationFleet`, `CapacityReservationBillingOwnership`, `CreateCapacityReservationBySplitting`, related Fleet describe/cancel; no terraform converter for `aws_ec2_capacity_reservation` yet.
- **Image attributes**: `DescribeImageAttribute`, `ModifyImageAttribute`, `ResetImageAttribute` round-trip the description and launch permissions. New `LaunchPermission { user_id, group }` type and `launch_permissions: Vec<LaunchPermission>` field on `Image`. `handle_modify_image_attribute` parses both the structured ( `LaunchPermission.Add.N.UserId` ) and legacy ( `Attribute=launchPermission` + `OperationType=add` ) wire shapes via a new `collect_launch_permissions` helper. `handle_reset_image_attribute` clears `launch_permissions` when `Attribute=launchPermission`. `Ec2StateView::ImageView` gained `launch_permissions` so snapshots/restores carry the data. 3 new round-trip tests.
- **EgressOnlyInternetGateway, NetworkAcl, CustomerGateway Terraform converters**: `AwsEgressOnlyInternetGatewayConverter` ( resource type `aws_egress_only_internet_gateway`, `depends_on aws_vpc` ); `AwsNetworkAclConverter` ( supports inline `ingress`/`egress` blocks, `subnet_ids`, tags ); `AwsNetworkAclRuleConverter` ( merges a single rule into the existing NACL view by `(rule_number, egress)` key; extract is intentionally a no-op since rule data round-trips through the parent ); `AwsCustomerGatewayConverter`. `handle_create_egress_only_igw` / `handle_describe_egress_only_igws` and `handle_create_customer_gateway` / `handle_describe_customer_gateways` now echo `tagSet` back to the provider so tagged terraform applies do not detect drift on refresh ( marked with `FIX(terraform-e2e)` ). NACL handlers already echoed tags via `nacl_to_model`. Five new isolated-server E2E tests covering basic + tagged + with-rules variants. Subnet-association coverage for NACLs is intentionally not exercised because `DescribeNetworkAcls` does not yet implement the `association.subnet-id` filter that the Terraform provider relies on for swapping the default NACL association ( recorded as a known gap ).

### 2026-04-27 EC2 Crate Split + Feature Gating

`winterbaume-ec2` was split into `winterbaume-ec2` ( hand-written + thin re-exports ) and `winterbaume-ec2-generated` ( auto-generated only ). The split fixed a 5+ minute incremental build problem where every edit to `handlers.rs` forced a full recompile of the 143k-line generated code. After the split, incremental edits compile in ~20 s. A second phase added Cargo features ( `network`, `compute`, `storage`, `advanced-network`, `extras` ) so even cold rebuilds of the generated crate during partial-feature development drop from ~7m to ~50 s ( network ) / ~3m ( storage ).

Full mechanics, feature taxonomy, regeneration command, and pitfalls live in [`ec2-crate-split-and-feature-gating.md`](ec2-crate-split-and-feature-gating.md). The `add-service` skill flags the EC2 special case immediately after Step 0 with a pointer to that doc.

### 2026-04-26 / 04-27 Multi-Service Batch ( ~40 New Crates )

Two large batches added approximately 40 new service crates in 2026-04-26 / 04-27. Most followed the root-resource-first pattern from `new-service-implementation-patterns.md` with full or near-full operation coverage. Notable groupings:

- **Cost & Billing**: `costandusagereport`, `costoptimizationhub`, `billing`, `bcmdashboards`, `bcmdataexports`, `bcmrecommendedactions`, `freetier`, `pricing`, `savingsplans`, `taxsettings`.
- **Application observability**: `applicationsignals`, `applicationdiscoveryservice`, `applicationinsights`, `pi` ( Performance Insights ).
- **Backup family**: `backupgateway`, `backupsearch`, `rbin` ( Recycle Bin ).
- **CodeGuru / CodeStar**: `codegurusecurity`, `codegurureviewer`, `codestarnotifications`.
- **Connect family**: `appintegrations`, `connectparticipant`, `connectcontactlens`, `pinpointsmsvoice`.
- **Personalize**: `personalizeevents`, `personalizeruntime`.
- **Chime / Cloud9 / SimSpaceWeaver / Braket**: `chimesdkmeetings`, `cloud9`, `simspaceweaver`, `braket`.
- **Recovery / Snow / Outposts**: `route53recoverycluster`, `snowdevicemanagement`, `s3outposts`.
- **Trusted Advisor / Support / Control Catalog**: `trustedadvisor`, `supportapp`, `controlcatalog`.
- **Cognito Sync / Cognito legacy**: `cognitosync`.
- **Amplify UI Builder**: `amplifyuibuilder` ( 4 lifecycle tests `#[ignore]`d due to SDK strict deserialisation of `@httpPayload` responses; codegen fix tracked in TODO ).
- **PCA Connector for SCEP**: `pcaconnectorscep`.
- **CloudFront KVS / CloudTrail Data / CloudSearch Domain**: `cloudfrontkeyvaluestore`, `cloudtraildata`, `cloudsearchdomain`.
- **SSM Quick Setup**: `ssmquicksetup`.

The recurring patterns across these crates are documented in `new-service-implementation-patterns.md` ( SDK accessor traps, awsJson1.0 vs 1.1, restJson1 with `@httpPayload`, Value-backed vs typed state, percent-decoded path segments, lean codegen-skip, Smithy epoch-seconds-float timestamps, per-service hostname patterns, tag wire shape variance ).

### 2026-05-02 S3 Files Full Coverage

`winterbaume-s3files` added support for AWS S3 Files, a brand-new restJson1 service modelled as `s3files` version `2025-05-05`. The first pass implemented FileSystem lifecycle plus resource tagging; the follow-up pass implemented mount targets, access points, file-system policy, synchronisation configuration, and mount-target update. The crate now covers 21 / 21 operations and has no 501 stubs remaining.

The second pass added `CreateMountTarget`, `GetMountTarget`, `ListMountTargets`, `DeleteMountTarget`, `UpdateMountTarget`, `CreateAccessPoint`, `GetAccessPoint`, `ListAccessPoints`, `DeleteAccessPoint`, `PutFileSystemPolicy`, `GetFileSystemPolicy`, `DeleteFileSystemPolicy`, `PutSynchronizationConfiguration`, and `GetSynchronizationConfiguration`.

The crate is registered in the workspace, server, root feature surface, api-coverage mapping, and root dev-dependencies. Its current integration suite has 34 tests covering FileSystem and tag behaviour plus mount-target VPC/AZ constraints, security-group update, access-point lifecycle and idempotency, unknown-parent errors, policy size/no-policy behaviour, synchronisation-configuration version conflicts and rule limits, and delete-while-in-use conflicts. There are no Terraform AWS provider resources for S3 Files yet, so converter and E2E work is intentionally skipped. `/write-tests` remains the publication-readiness follow-up for scenario inventory and documentation-derived scenarios.

## Files

- `Cargo.toml`: workspace membership and top-level package/test configuration.
- `crates/winterbaume-core/src/*`: shared request routing, state, auth, and client bridge infrastructure.
- `crates/winterbaume-*/src/handlers.rs`: per-service protocol routing and operation dispatch.
- `crates/winterbaume-*/tests/integration_test.rs`: service-level SDK integration tests.
- `tools/smithy-codegen/src/discover.rs`: service-to-model discovery, especially important once the crate count expanded.
- `.agents/docs/API_COVERAGE.md`: reported winterbaume vs moto operation coverage.

## Test Coverage

Coverage validation moved in three layers:

- service integration tests for core flows and edge cases
- parity-porting from moto for exact field and error behaviour
- Terraform E2E tests for provider-driven read and waiter behaviour

By late March 2026, the journal reports:

- 1,600+ cumulative integration tests across ported services
- 816 moto parity tests across 32 services at one milestone
- Terraform E2E coverage for resource flows across multiple services
- Route53 reached 87 integration tests after the late-March error-path expansion
- EC2 reached 62 passing integration tests plus separate Terraform E2E coverage for VPC, subnet, IGW, security-group, route-table, and key-pair flows
- newly added services also landed with immediate regression suites: S3 Control (15 tests), S3 Vectors (37 tests), API Gateway (10 tests), and S3 Tables (47 tests)
- the next wave kept that standard: API Gateway Management (8 tests), EMR (19 tests), GuardDuty upgrades (11 new tests, 40 total), CodeBuild report groups (4 new tests, 39 total), Amplify (15 tests), and OpenSearch expansion (23 new tests, 54 total)
- the late 2026-04-03 and 2026-04-04 waves then pushed that pattern further, with Redshift (190 tests), Lex v2 (147), RDS (83), S3 Control (72), CloudFormation (43), EFS (31), Batch (39), and many medium services crossing the 80% integration threshold
- final session metrics from 2026-04-04 recorded 4,978 / 5,809 implemented operations covered by integration tests (85.7%), 133 services at 80%+ integration coverage, and 40 services with Terraform E2E coverage
- the April 25/26 service batches added 12 new crates, 129 implemented operations, and 145 integration tests
- S3 Files reached 34 integration tests after the full 21-operation pass; fmt, clippy, crate tests, and `winterbaume-server` build passed after registration

## Pitfalls

- `API_COVERAGE.md` can miss operations implemented through generic handlers or unusual naming.
- `API_COVERAGE.md` can also miss implemented test coverage when detection heuristics or Terraform resource-to-crate mappings drift out of date.
- Worktree-based parallel edits can produce stale or conflicting generated files if copied back blindly.
- Operation-count growth is lower value than behavioural correctness. A handler that only satisfies smoke tests still creates regressions.
- Protocol differences matter. A service that looks structurally similar may still need different routing, timestamp handling, headers, or error envelopes.
- Do not hide deferred operations in partial new services. Document the unimplemented families and return explicit 501 responses.
