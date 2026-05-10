# Winterbaume Development Journal

## LTM Consolidation Record

The journal has been audited against `.agents/docs/LTM/` and `.agents/docs/TODO.md` as of 2026-05-08. Every substantive entry that previously lived in this file has had its durable knowledge promoted to one or more LTM documents ( and any open follow-ups recorded in `.agents/docs/TODO.md` ); the consolidated entries themselves have been removed per the `reconcile-journal-ltm` workflow. Everything that needed to survive lives in LTM.

### Section → LTM Document Mapping

| Section | LTM Document |
|---------|--------------|
| 2026-03-28: add-service skill documentation update for StatefulService views | `stateful-service-and-blob-store.md` |
| 2026-03-28: Test plan: ec2instanceconnect | `aws-doc-test-plan-catalog.md` |
| 2026-03-28: Test plan: signer | `aws-doc-test-plan-catalog.md` |
| 2026-03-28: Test plan: sagemakerruntime | `aws-doc-test-plan-catalog.md` |
| 2026-03-28: Test plan: sso | `aws-doc-test-plan-catalog.md` |
| 2026-03-28: Test plan: account | `aws-doc-test-plan-catalog.md` |
| 2026-03-28: Test plan: costexplorer | `aws-doc-test-plan-catalog.md` |
| 2026-03-28: Test plan: applicationautoscaling | `aws-doc-test-plan-catalog.md` |
| Session summary: write-tests batch run (2026-03-28) | `aws-doc-test-plan-catalog.md` |
| 2026-03-28: Batch terraform apply infrastructure for E2E test harness | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-03-28: S3 and Route53 moto parity work via generated restXml request deserializers | `smithy-codegen-and-wire-serialization.md`, `core-service-expansion-and-coverage.md` |
| 2026-03-28: StatefulService State-Change Subscription + Batch Service Views | `stateful-service-and-blob-store.md` |
| 2026-03-28: Audit — `serde_json::Value` in Public View Structs | `stateful-service-and-blob-store.md` |
| 2026-03-28: VFS-Backed Blob Store — Full Implementation | `stateful-service-and-blob-store.md` |
| 2026-03-28: StatefulService views.rs batch completion — remaining 86 services | `stateful-service-and-blob-store.md` |
| 2026-03-28: Code review - recent StatefulService / blob-store rollout | `stateful-service-and-blob-store.md` |
| 2026-03-28: StatefulService rollout — final service (winterbaume-textract) | `stateful-service-and-blob-store.md` |
| 2026-03-28: Code review fixes — VFS path traversal, S3 blob error propagation, S3 merge contract | `stateful-service-and-blob-store.md` |
| 2026-03-28: Terraform E2E Tests — Events, CloudWatch, Lambda | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-03-28: Full Async Migration — VFS / BlobStore / StatefulService / Terraform Stack | `stateful-service-and-blob-store.md` |
| 2026-03-28: Code Review - Current Working Tree | `stateful-service-and-blob-store.md` |
| 2026-03-29: Fixes for Code Review Findings (2026-03-28) | `stateful-service-and-blob-store.md` |
| 2026-03-29: Terraform E2E Tests — StepFunctions, Kinesis, CognitoIDP, and Full Suite Fix | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-03-29: DynamoDB PartiQL Integration | `dynamodb-partiql-integration.md` |
| 2026-03-29: Route53 Integration Test Expansion | `core-service-expansion-and-coverage.md` |
| 2026-03-29: Pluggable Backend Traits for SQS and SNS | `pluggable-service-backends-and-redis-storage.md` |
| 2026-03-29: Terraform E2E Tests — ECS, Route53, EFS, ACM (4 new modules) | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-03-30: Hand-crafted Response Cleanup in ECS and ACM | `smithy-codegen-and-wire-serialization.md` |
| 2026-03-30: Codegen Fix Plan — Output-Only Shape `@required` Fields as `Option<T>` | `smithy-codegen-and-wire-serialization.md` |
| 2026-03-30: DuckDB/PartiQL SQL Engine Backend for Athena and Redshift Data | `query-service-sql-engine-backends.md` |
| 2026-03-30 — STUB annotation convention + large coverage push | `core-service-expansion-and-coverage.md` |
| 2026-03-30 — Redis-backed SQS backend (`winterbaume-sqs-redis`) | `pluggable-service-backends-and-redis-storage.md` |
| 2026-03-30: Codegen Fix Implemented — Output-Only `@required` Fields as `Option<T>` | `smithy-codegen-and-wire-serialization.md` |
| 2026-03-30: DynamoDB Pluggable Backend + Redis Implementation | `pluggable-service-backends-and-redis-storage.md` |
| 2026-03-30 — Clippy warning cleanup | `smithy-codegen-and-wire-serialization.md` |
| 2026-03-30: `reconcile-journal-ltm` skill | `journal-ltm-maintenance-workflows.md` |
| 2026-03-31: tackle-todos batch — behavioral fixes and httpResponseCode extraction | `smithy-codegen-and-wire-serialization.md`, `parallel-agent-build-and-worktree-practices.md` |
| 2026-03-31: tackle-todos second pass — IAM XML serialization, DynamoDB ListTables, budgets notifications | `smithy-codegen-and-wire-serialization.md` |
| 2026-03-31: Fix generator instead of generated files | `smithy-codegen-and-wire-serialization.md` |
| 2026-03-31: Fix — Broken Indentation in Generated Wrapper Structs | `smithy-codegen-and-wire-serialization.md` |
| 2026-03-31: Eliminate Dual backend+state Pattern in DynamoDB, SQS, SNS | `pluggable-service-backends-and-redis-storage.md` |
| 2026-03-31 — Clippy: Suppress `non_camel_case_types` and `upper_case_acronyms` in generated code | `smithy-codegen-and-wire-serialization.md` |
| 2026-03-31: CloudFront and WAFv2 Terraform E2E Tests | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-03-31: ELBv2, EKS, and Organizations Terraform E2E Tests | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-03-31: Fix DynamoDB Streams shared state | `stateful-service-and-blob-store.md` |
| 2026-03-31: README Coverage Update and Examples Crate | `workspace-readmes-and-service-examples.md` |
| 2026-03-31: Renamed winterbaume-tagging to winterbaume-resourcegroupstagging | `workspace-readmes-and-service-examples.md` |
| 2026-03-31: README Coverage Summary + Per-Service Examples | `workspace-readmes-and-service-examples.md` |
| 2026-04-01: Implemented winterbaume-s3control and Fixed DynamoDB Streams Terraform Crate | `core-service-expansion-and-coverage.md`, `stateful-service-and-blob-store.md` |
| 2026-03-31: Terraform E2E Tests -- EC2 | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-03-31: EC2/VPC Service Implementation (winterbaume-ec2) | `core-service-expansion-and-coverage.md`, `smithy-codegen-and-wire-serialization.md` |
| 2026-04-01: EC2 compile-error fix | `core-service-expansion-and-coverage.md` |
| 2026-04-01: EC2 Compute and Storage Operation Implementation | `core-service-expansion-and-coverage.md` |
| 2026-04-01: DynamoDB AttributeValue Enum Refactor, Streams Change Capture, and Expression Audit | `dynamodb-partiql-integration.md`, `stateful-service-and-blob-store.md` |
| 2026-04-01: S3 Tables - Full 49/49 Operation Coverage | `core-service-expansion-and-coverage.md` |
| 2026-04-01: PartiQL Feature Completions | `dynamodb-partiql-integration.md` |
| 2026-04-01: S3 Tables — Documentation-Derived Test Suite | `aws-doc-test-plan-catalog.md` |
| 2026-04-01: EC2 Moto Parity Implementation Completion | `moto-parity-testing-and-behavioral-gaps.md`, `core-service-expansion-and-coverage.md` |
| 2026-04-01: Session Wrap-up — EC2 Parity + Coverage Update | `core-service-expansion-and-coverage.md`, `workspace-readmes-and-service-examples.md` |
| 2026-04-01: tackle-todos pass — budgets inline notifications, backup lock validation, EC2 default ACL | `core-service-expansion-and-coverage.md`, `smithy-codegen-and-wire-serialization.md` |
| 2026-04-01: S3 Tables — Write-Tests Session Summary | `aws-doc-test-plan-catalog.md` |
| 2026-04-01: tackle-todos pass (second) — EC2 ACL, sdb clientToken, serde_json::Value policy | `stateful-service-and-blob-store.md`, `core-service-expansion-and-coverage.md` |
| 2026-04-01: New Service — API Gateway (REST v1) | `core-service-expansion-and-coverage.md`, `smithy-codegen-and-wire-serialization.md` |
| 2026-04-01: API Coverage Fix - Moto Column for Stub Services | `workspace-readmes-and-service-examples.md` |
| 2026-04-01: New Crate - winterbaume-apigatewaymanagementapi | `core-service-expansion-and-coverage.md`, `workspace-readmes-and-service-examples.md` |
| 2026-04-01: EMR (Elastic MapReduce) Service Crate | `core-service-expansion-and-coverage.md` |
| 2026-04-01: GuardDuty stub-to-real upgrade (IP sets, threat intel sets, tags) | `core-service-expansion-and-coverage.md` |
| 2026-04-02: winterbaume-codebuild report group operations | `core-service-expansion-and-coverage.md` |
| 2026-04-02: AWS SDK crate naming audit | `workspace-readmes-and-service-examples.md` |
| 2026-04-03 — winterbaume-amplify implementation | `core-service-expansion-and-coverage.md` |
| 2026-04-03: OpenSearch — Core Resource Operations (35 new operations) | `core-service-expansion-and-coverage.md` |
| 2026-04-03: Coverage Expansion Sprint — 50.3% → 55.6% | `core-service-expansion-and-coverage.md` |
| 2026-04-03: X-Ray integration test expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03: S3 Control Integration Test Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03: Lex Integration Test Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03: api-coverage Terraform E2E coverage reporting | `workspace-readmes-and-service-examples.md` |
| 2026-04-03: api-coverage integration-test coverage integration | `workspace-readmes-and-service-examples.md` |
| 2026-04-03: api-coverage skill self-containment | `workspace-readmes-and-service-examples.md` |
| 2026-04-03: Comprehensive Redshift Integration Test Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03: Lex V2 Comprehensive Integration Test Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03: Timestream Query Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03: Redshift Data API Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03: Cognito Identity Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03: ELBv2 Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03: STS Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03: RDS Data API Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03: AppConfig Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03: ECR Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03: WorkSpaces Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03: SSM Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03: CodeCommit Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03: networkmanager integration test coverage expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03: WorkspacesWeb integration test coverage expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03: EKS Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03: Lambda integration test coverage expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03: CloudWatch Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-03: BedrockAgent Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03: Cognito IDP Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03: Transfer Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03: CloudWatch Logs Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-03: Test Coverage Enhancement Batch | `core-service-expansion-and-coverage.md`, `aws-doc-test-plan-catalog.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-03: Bulk Test Coverage Expansion — Integration Tests + Terraform E2E | `core-service-expansion-and-coverage.md`, `aws-doc-test-plan-catalog.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-03: Second Wave — Remaining Integration Tests + Expanded E2E Coverage | `core-service-expansion-and-coverage.md`, `aws-doc-test-plan-catalog.md`, `terraform-e2e-harness-and-fix-coverage.md`, `workspace-readmes-and-service-examples.md` |
| 2026-04-03: Lambda Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-04: Enhancement Wave — Pushing 60-80% Services Toward 80%+ and Broad E2E Expansion | `core-service-expansion-and-coverage.md`, `aws-doc-test-plan-catalog.md`, `terraform-e2e-harness-and-fix-coverage.md`, `workspace-readmes-and-service-examples.md` |
| 2026-04-04: refactor-state-errors — Batch Execution Across 33+ Services | `state-error-shaping-and-handler-boundaries.md` |
| 2026-04-04: PartiQL DML Parser — Clippy Fixes and Edge-Case Test Coverage | `dynamodb-partiql-integration.md` |
| 2026-04-04: Clippy dead_code fix in generated wire.rs (split path) | `smithy-codegen-and-wire-serialization.md` |
| 2026-04-04: Clippy Warning Fixes — Three Crates | `smithy-codegen-and-wire-serialization.md` |
| 2026-04-04: Clippy Fix — `box_collection` in Smithy Codegen | `smithy-codegen-and-wire-serialization.md` |
| 2026-04-05: TODO Batch — Multi-Service Behavioural Fixes | `moto-parity-testing-and-behavioral-gaps.md`, `core-service-expansion-and-coverage.md` |
| 2026-04-05: Crate rename — aws-sdk-* naming convention alignment | `workspace-readmes-and-service-examples.md` |
| 2026-04-06: Snapshot Semantics Audit — Blob-Backed and Large-Payload Services | `stateful-service-and-blob-store.md` |
| 2026-04-06: BlobBackedService trait redesign — lifetime soundness, dyn-compatibility, and bug fix | `stateful-service-and-blob-store.md` |
| 2026-04-06: BackendState tokio::sync::RwLock Migration + BlobBackedService Trait | `stateful-service-and-blob-store.md`, `runtime-state-and-service-infrastructure-synthesis.md` |
| 2026-04-06: Terraform Converter Integration Test Coverage — 100% | `terraform-resource-converters.md` |
| 2026-04-09: E2E Test Bug Fixes — ECR, AppConfig, CloudWatch, BedrockAgent | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-11: Terraform E2E Harness — Orphaned Processes + Batch Strategy Overhaul | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-11 — Replace `kill` subprocess with `libc::killpg` in E2E harness | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-12 — Extend api-coverage skill with floci and kumo emulator coverage | `workspace-readmes-and-service-examples.md` |
| 2026-04-11: Fix Route53 `GetDNSSEC` handler not returning key signing keys | `stub-handler-audit-and-promotion.md` |
| 2026-04-11: Fix compile errors in `winterbaume-guardduty` integration tests | `stub-handler-audit-and-promotion.md` |
| 2026-04-12: tackle-todos — Stub Handler Audit and Three Fixes | `stub-handler-audit-and-promotion.md` |
| 2026-04-12 — floci/kumo coverage in api-coverage and update-readme skills | `workspace-readmes-and-service-examples.md` |
| 2026-04-12 — Hard-coded stub audit and STUB[no-state] resolution (acmpca, xray, guardduty) | `stub-handler-audit-and-promotion.md` |
| 2026-04-12 — Plan: IAM policy evaluation engine (`winterbaume-iam-rule-eval`) | `rule-evaluator-and-validator-crates.md` |
| 2026-04-12 — Plan: ASL validation engine (`winterbaume-sfn-asl-eval`) | `rule-evaluator-and-validator-crates.md` |
| 2026-04-12 — Plan: WAFv2 WCU capacity calculator (`winterbaume-wafv2-wcu-eval`) | `rule-evaluator-and-validator-crates.md` |
| 2026-04-12 — Revised plan: split WCU eval into two crates | `rule-evaluator-and-validator-crates.md` |
| 2026-04-12 — Plan: Bedrock flow graph validator (`winterbaume-bedrockagent-flow-eval`) | `rule-evaluator-and-validator-crates.md` |
| 2026-04-12 — Bug fixes: synthetics GetCanaryRuns, codebuild ListReportsForReportGroup, apigateway ImportRestApi | `stub-handler-audit-and-promotion.md` |
| 2026-04-12: X-Ray — Trace data, summaries, batch retrieval, and sampling targets | `stub-handler-audit-and-promotion.md` |
| 2026-04-12 — Work summary: stub-handler fixes and E2E test activation | `stub-handler-audit-and-promotion.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-12: Bulk STUB[no-state] Handler Promotion — 8 Service Crates | `stub-handler-audit-and-promotion.md` |
| 2026-04-12: FIX(terraform-e2e) Integration Test Coverage Audit & Gap Closure | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-12 — Implementation: WAFv2 rule parser and WCU calculator (Phase 1–3) | `rule-evaluator-and-validator-crates.md` |
| 2026-04-12: IAM Rule Evaluator and SFN ASL Evaluator — Phase 1 Implementation | `rule-evaluator-and-validator-crates.md` |
| 2026-04-12: Code Review — `winterbaume-sfn`, `winterbaume-sfn-asl-eval`, `winterbaume-wafv2-wcu-calculator`, `winterbaume-wafv2-webacl-rule-parser` | `rule-evaluator-and-validator-crates.md` |
| 2026-04-13: winterbaume-rds — Migrate handlers from manual XML to wire module | `smithy-codegen-and-wire-serialization.md` |
| 2026-04-13 — Fix WCU calculator ByteMatch cost bug | `rule-evaluator-and-validator-crates.md` |
| 2026-04-13 to 2026-04-14: Quality Gate Sweep Consolidation | `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-13: Bedrock Flow Definition Parser and Validator | `rule-evaluator-and-validator-crates.md` |
| 2026-04-14: smithy-codegen rpc-v2-cbor Protocol Support | `smithy-codegen-and-wire-serialization.md` |
| 2026-04-14 — RDS: Migrate handlers from manual query parsing to wire module deserialisation | `smithy-codegen-and-wire-serialization.md` |
| 2026-04-14: Tackle-TODOs sweep — views expansion, stub promotion, and regression coverage | `quality-gate-workflow-and-recurring-findings.md`, `stub-handler-audit-and-promotion.md`, `stateful-service-and-blob-store.md` |
| 2026-04-14: Fix kumo coverage parser; regenerate README and API coverage report | `workspace-readmes-and-service-examples.md`, `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-04-14: Tackle-TODOs sweep 2 — validation, mock improvement, and stub promotion | `quality-gate-workflow-and-recurring-findings.md`, `stub-handler-audit-and-promotion.md`, `rule-evaluator-and-validator-crates.md` |
| 2026-04-15: CI Failure Fixes — Clippy duplicated_attributes, Rustfmt, lexmodelsv2 unwrap | `smithy-codegen-and-wire-serialization.md`, `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-04-14: QG-7.2 sesv2 — Replace hand-written JSON response bodies | `smithy-codegen-and-wire-serialization.md`, `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-14/15: Tackle-TODOs sweep 3 — QG-7.2 wire serialiser migration and rpc-v2-cbor timestamps | `smithy-codegen-and-wire-serialization.md`, `quality-gate-workflow-and-recurring-findings.md`, `service-implementation-and-validation-synthesis.md` |
| 2026-04-15 — Quality Gate: winterbaume-wafv2 | `quality-gate-workflow-and-recurring-findings.md`, `rule-evaluator-and-validator-crates.md` |
| 2026-04-15 — Quality Gate: winterbaume-macie2 | `quality-gate-workflow-and-recurring-findings.md`, `stub-handler-audit-and-promotion.md` |
| 2026-04-15 — Quality Gate: winterbaume-sesv2 | `quality-gate-workflow-and-recurring-findings.md`, `smithy-codegen-and-wire-serialization.md` |
| 2026-04-15 — Quality Gate: winterbaume-ec2 | `quality-gate-workflow-and-recurring-findings.md`, `smithy-codegen-and-wire-serialization.md`, `stateful-service-and-blob-store.md` |
| 2026-04-15 — Quality Gate: winterbaume-backup | `quality-gate-workflow-and-recurring-findings.md`, `stub-handler-audit-and-promotion.md` |
| 2026-04-15 — Quality Gate: winterbaume-iam | `quality-gate-workflow-and-recurring-findings.md`, `stateful-service-and-blob-store.md` |
| 2026-04-15 — Quality Gate: winterbaume-kms | `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-15 — Quality Gate: winterbaume-glue | `quality-gate-workflow-and-recurring-findings.md`, `stateful-service-and-blob-store.md` |
| 2026-04-15 — Quality Gate: winterbaume-cloudwatch | `quality-gate-workflow-and-recurring-findings.md`, `smithy-codegen-and-wire-serialization.md` |
| 2026-04-15 — Quality Gate: winterbaume-bedrockagent | `quality-gate-workflow-and-recurring-findings.md`, `stateful-service-and-blob-store.md` |
| 2026-04-15 — Quality Gate Sweep: All Three-Sweep TODO Crates | `quality-gate-workflow-and-recurring-findings.md`, `service-implementation-and-validation-synthesis.md` |
| 2026-04-15: CI Failure Fix — Stale Generated Files and Cascading Breakage | `smithy-codegen-and-wire-serialization.md`, `stateful-service-and-blob-store.md`, `terraform-resource-converters.md` |
| 2026-04-16: CI Failure Fix — Additional Clippy Lints in Generated Files | `smithy-codegen-and-wire-serialization.md`, `service-implementation-and-validation-synthesis.md` |
| 2026-04-16: CI Failure Fix — `--all-targets` Uncovers Test and Example Warnings | `service-implementation-and-validation-synthesis.md`, `repo-maintenance-and-agent-workflows-synthesis.md`, `workspace-readmes-and-service-examples.md` |
| 2026-04-16: CI Failure Fix — CloudWatch DeleteAnomalyDetector Missing SingleMetricAnomalyDetector Support | `service-implementation-and-validation-synthesis.md` |
| 2026-04-18: CI Failure Fix — WAFv2 Clippy + Missing E2E `#[ignore]` Attributes | `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-04-18: CI Failure Fix — Cascading Clippy + Test + E2E Fixes (runs 24574987225, 24576098332, 24577669727) | `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-04-18: CI Failure Fix — Redshift Data `clippy::sort_by_key` (run 24592718536) | `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-04-18: Fix README API coverage table — missing protocols and duplicate entries | `workspace-readmes-and-service-examples.md` |
| Removal of `winterbaume-stubs` crate (2026-04-18) | `workspace-readmes-and-service-examples.md`, `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-04-19: FIX(terraform-e2e) Audit — Full Legitimacy Review | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-18: CI Failure Fix — Bedrock Agent `clippy::collapsible_match` (run 24594377293) | `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-04-18: Hard-coded mock TODO sweep | `stub-handler-audit-and-promotion.md` |
| 2026-04-18: Bulk Release Preparation — Crate Publishing Readiness Audit | `crate-publishing-and-release.md` |
| 2026-04-19: Bulk Release Preparation — Continued | `crate-publishing-and-release.md` |
| 2026-04-19: Athena — TagResource and UntagResource implementation | `core-service-expansion-and-coverage.md` |
| 2026-04-19: Bulk STUB[no-state] Elimination — All Crates | `stub-handler-audit-and-promotion.md`, `core-service-expansion-and-coverage.md` |
| 2026-04-20: CI fix — CloudFormation `test_signal_resource` and `test_continue_update_rollback` | `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-04-20: Deep Sleep — Synthesis Refresh | `journal-ltm-maintenance-workflows.md` |
| 2026-04-20: Terraform Converter Coverage Audit and Mass Enhancement | `terraform-resource-converters.md` |
| 2026-04-20: StateView Expansion for Poor/Fair Converters | `terraform-resource-converters.md` |
| 2026-04-20: Terraform Converter Mass Expansion — Complete Session Summary | `terraform-resource-converters.md` |
| 2026-04-20: Converter Field Coverage Enhancement — All Converters to Excellent | `terraform-resource-converters.md` |
| 2026-04-20: Terraform Converter Skill, Documentation, and TODO Extraction | `terraform-resource-converters.md` |
| 2026-04-21: Deep Sleep — Synthesis Refresh | `journal-ltm-maintenance-workflows.md` |
| 2026-04-21: Tackle-TODOs — Terraform Nested Block Implementation Plans | `terraform-resource-converters.md` |
| 2026-04-21: Integration Test Fixes — costexplorer and dynamodb | `core-service-expansion-and-coverage.md` |
| 2026-04-21: Terraform Nested Block Implementation — Batch 1 Complete | `terraform-resource-converters.md` |
| 2026-04-21: AWS Inter-service Integration Map | `aws-inter-service-integration-priorities.md` |
| 2026-04-21: Peer Review of Cross-service Findings and Batch 1 Journal Entries | `terraform-resource-converters.md`, `aws-inter-service-integration-priorities.md` |
| 2026-04-21: Cross-Service Analysis — Athena-Glue Integration Gap | `aws-inter-service-integration-priorities.md`, `query-service-sql-engine-backends.md`, `pluggable-backends-and-query-execution-synthesis.md` |
| 2026-04-21: Terraform Converter Registration (tackle-todos) | `terraform-resource-converters.md`, `parallel-agent-build-and-worktree-practices.md` |
| 2026-04-21: E2E Terraform Test Failures — Provider Compatibility Fixes | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-22: Make DuckDB connection injectable in winterbaume-sqlengine-duckdb | `query-service-sql-engine-backends.md` |
| 2026-04-22: Redshift `ClusterAvailabilityStatus` fix — E2E Terraform failures | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-22 — Quality Gate: All Service Crates (164 services) | `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-22 — list-of-timestamp CBOR codegen: deep-dive findings | `smithy-codegen-and-wire-serialization.md` |
| 2026-04-22: QG §7 Tackle-TODOs Sweep — Wire Serialiser Migration | `smithy-codegen-and-wire-serialization.md`, `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-22: Fix Redshift E2E failures — AvailabilityZoneRelocationStatus | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-22 — Terraform Converter Nested Block Mass Expansion | `terraform-resource-converters.md` |
| 2026-04-23: Redshift E2E Test Failures — Provider Crash Fixes | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-23: Deep Sleep — Synthesis Refresh | `journal-ltm-maintenance-workflows.md` |
| 2026-04-23 — Correction: `replication_overwrite_protection` is an `awscc` provider attribute, not `aws` | `journal-ltm-maintenance-workflows.md` |
| 2026-04-23 — Quality Gate: All Service Crates | `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-23: Session Summary — Memory Maintenance and Quality Gate Sweep | `journal-ltm-maintenance-workflows.md`, `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-24 — BlobStore-backed state recovery & VFS path-traversal hardening | `stateful-service-and-blob-store.md` |
| 2026-04-24: Hand-crafted XML elimination in S3 and EC2 handlers | `smithy-codegen-and-wire-serialization.md`, `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-24 — tackle-todos: DynamoDB and Terraform E2E fixes | `dynamodb-partiql-integration.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-24 — Quality Gate: workspace service-crate audit | `quality-gate-workflow-and-recurring-findings.md`, `smithy-codegen-and-wire-serialization.md`, `dynamodb-partiql-integration.md`, `terraform-resource-converters.md` |
| 2026-04-24: Account / region separation contract and Lambda scope fix | `terraform-resource-converters.md`, `runtime-state-and-service-infrastructure-synthesis.md` |
| 2026-04-24: Peer review — CBOR timestamp-list codegen and CloudWatch handler migration | `smithy-codegen-and-wire-serialization.md` |
| 2026-04-24: BlobStore account/region isolation | `stateful-service-and-blob-store.md`, `runtime-state-and-service-infrastructure-synthesis.md` |
| 2026-04-24: Design — Multi-region Terraform extraction (terraform-extract-region-symmetry) | `terraform-resource-converters.md` |
| 2026-04-24: Peer review — Multi-region Terraform extraction design | `terraform-resource-converters.md` |
| 2026-04-24: Revised design — Multi-region Terraform extraction (Option C) | `terraform-resource-converters.md` |
| 2026-04-24: Peer review — Multi-region Terraform extraction Option C | `terraform-resource-converters.md` |
| 2026-04-24: Final design — Multi-scope Terraform conversion (Option D) | `terraform-resource-converters.md` |
| 2026-04-24: Final design — Multi-region Terraform extraction (Option D) | `terraform-resource-converters.md` |
| 2026-04-24: E2E Terraform Test Failure Triage and Fixes | `terraform-e2e-harness-and-fix-coverage.md`, `smithy-codegen-and-wire-serialization.md`, `terraform-resource-converters.md` |
| 2026-04-24 — Fix clippy warnings in smithy-codegen | `smithy-codegen-and-wire-serialization.md` |
| 2026-04-24: Implementation — Multi-scope Terraform extraction (Option D) | `terraform-resource-converters.md`, `runtime-state-and-service-infrastructure-synthesis.md` |
| 2026-04-25: Access logging for winterbaume-server | `workspace-readmes-and-service-examples.md` |
| 2026-04-25: Terraform S3 smoke test failure — virtual-hosted-style requests | `terraform-e2e-harness-and-fix-coverage.md`, `workspace-readmes-and-service-examples.md` |
| 2026-04-25: Glacier blob-backed state recovery | `stateful-service-and-blob-store.md` |
| 2026-04-25 Tackle TODOs: smithy-codegen items 2 and 3 | `smithy-codegen-and-wire-serialization.md`, `terraform-resource-converters.md` |
| 2026-04-25: sccache-wrapper tool for cross-worktree cache efficiency | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-25: Batch Service Implementation — 6 New AWS Services | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-25: Cross-worktree Rust compilation caching in sccache-wrapper | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-25: sccache-wrapper — cache key normalisation fix + singleflight | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-25: sccache-wrapper — external crate cache MISS caused by `--diagnostic-width` | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-25: sccache-wrapper — cache pollution from target/host dual compilation | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-26: Stale sccache server hangs builds | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-26: sccache-wrapper — Replace C FFI flock() with fslock crate | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-26: sccache-wrapper — incremental stripping, diagnostic storage, and --dump-cache | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-26: Reflog-derived chronological work summary (2026-03-26 to 2026-04-26) | `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-04-26 — Implement winterbaume-aiops crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26: sccache-wrapper — cross-worktree poisoning fix via `.cachekey` sidecars | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-26 — Implement winterbaume-amplifybackend crate (partial) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — Implement winterbaume-appconfigdata crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — GitHub triage scaffolding: service labels + bug-report form | `github-issue-triage-and-automation.md` |
| 2026-04-26 — Implement winterbaume-appfabric crate (partial) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — Expand service labels to cover unimplemented AWS services | `github-issue-triage-and-automation.md` |
| 2026-04-26 — Implement winterbaume-appflow crate (partial) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — Implement winterbaume-applicationcostprofiler crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — Session findings and work summary | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — Add feature-request issue template | `github-issue-triage-and-automation.md`, `new-service-implementation-patterns.md` |
| 2026-04-26: Bug-triage automation — GitHub Actions + sidecar memory branch | `github-issue-triage-and-automation.md`, `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-04-26 — Terraform converters and E2E tests for new crates | `terraform-resource-converters.md`, `terraform-e2e-harness-and-fix-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26: Deep Sleep — Synthesis Refresh | `journal-ltm-maintenance-workflows.md` |
| 2026-04-26 — Implement winterbaume-artifact crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — Staged-change review comments | `terraform-resource-converters.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — Cargo target-directory lock contention between concurrent agents | `parallel-agent-build-and-worktree-practices.md`, `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-26: EC2 Placement Group support added | `core-service-expansion-and-coverage.md`, `terraform-resource-converters.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-26 — Implement winterbaume-autoscalingplans crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — Session checkpoint after 8 service crates + 3 Terraform converters | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md`, `parallel-agent-build-and-worktree-practices.md` |
| 2026-04-26 — Session-final findings and work summary | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26: Skill workflow update — scenario tests folded into write-tests, add-service delegates to it | `new-service-implementation-patterns.md` |
| 2026-04-26 -- triage-bug guardrail against prompt injection | `github-issue-triage-and-automation.md` |
| 2026-04-26 -- AI cross-language guardrail (follow-up) | `github-issue-triage-and-automation.md` |
| 2026-04-26 -- sccache-wrapper scoreboard | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-26 -- guardrail simplified to AI-only (revision) | `github-issue-triage-and-automation.md` |
| 2026-04-26 (cont.): EC2 NetworkInterfacePermission and InstanceConnectEndpoint | `core-service-expansion-and-coverage.md` |
| 2026-04-26 -- guardrail prompt hardened from real-world attack catalogue | `github-issue-triage-and-automation.md` |
| 2026-04-26 -- guardrail prompt: stylised-alphabet substitutions | `github-issue-triage-and-automation.md` |
| 2026-04-26 -- guardrail prompt: translation laundering | `github-issue-triage-and-automation.md` |
| 2026-04-26 -- guardrail prompt: compositional / derivational translation | `github-issue-triage-and-automation.md` |
| 2026-04-26 (cont.): EC2 CapacityReservation operations | `core-service-expansion-and-coverage.md` |
| 2026-04-26 -- new service: cloud9 (AWS Cloud9) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 -- new service: cloudfront-keyvaluestore (CloudFront KeyValueStore) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 -- new service: cloudsearch-domain (CloudSearch Domain data plane) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 -- new service: cloudtrail-data (CloudTrail Data Service) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 -- new service: connectparticipant (Connect Participant data plane) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 -- new service: controlcatalog (Control Catalog) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 -- new service: freetier (Free Tier) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 -- new service: pca-connector-scep (Private CA Connector for SCEP) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 -- new service: personalize-events (Personalize Events) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 -- new service: personalize-runtime (Personalize Runtime) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 -- new service: pi (Performance Insights) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 -- new service: pinpoint-sms-voice (Pinpoint SMS Voice v1) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 -- new service: pricing (AWS Price List Service) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 -- new service: rbin (Recycle Bin) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 -- new service: route53-recovery-cluster (Route 53 ARC -- Cluster data plane) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 -- new service: s3outposts (S3 on Outposts) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 -- new service: savingsplans (Savings Plans) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 -- new service: simspaceweaver (SimSpace Weaver) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 -- new service: snow-device-management (Snow Device Management) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 -- new service: ssm-quicksetup (Systems Manager Quick Setup) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 -- new service: support-app (Support App) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 -- new service: taxsettings (Tax Settings) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 -- new service: trustedadvisor (Trusted Advisor v2) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 -- new service: appintegrations (Amazon Connect AppIntegrations) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 -- new service: amplifyuibuilder (Amplify UI Builder) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 -- new service: application-discovery-service | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 -- new service: application-insights | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 -- new service: application-signals (CloudWatch Application Signals) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 -- new service: backup-gateway | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 -- new service: backupsearch (AWS Backup Search) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 -- new service: bcm-dashboards (BCM Dashboards) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 -- new service: bcm-data-exports (BCM Data Exports) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 -- new service: bcm-recommended-actions | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27: Implement winterbaume-billing crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27: Implement winterbaume-braket crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27: Implement winterbaume-chimesdkmeetings crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27: Implement winterbaume-codegurureviewer crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27: Implement winterbaume-codegurusecurity crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27: Implement winterbaume-codestarnotifications crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27: Implement winterbaume-cognitosync crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27: Implement winterbaume-connectcontactlens crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27: Implement winterbaume-costandusagereport crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27: Implement winterbaume-costoptimizationhub crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27: EC2 coverage expansion -- image attributes + egress-only IGW Terraform | `core-service-expansion-and-coverage.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-27: EC2 coverage expansion -- NACL + customer gateway Terraform | `core-service-expansion-and-coverage.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-27 — Split `winterbaume-ec2` into `winterbaume-ec2` + `winterbaume-ec2-generated` | `ec2-crate-split-and-feature-gating.md`, `core-service-expansion-and-coverage.md` |
| 2026-04-27 — Phase 2: feature-gate `winterbaume-ec2-generated` operations | `ec2-crate-split-and-feature-gating.md` |
| 2026-04-27 — EC2 split / feature-gating: agent guidance | `ec2-crate-split-and-feature-gating.md` |
| 2026-04-27: sccache-wrapper error handling -- residual-error fixes | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-27 -- sccache-wrapper: GC mode | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-27 -- sccache-wrapper: GC must group by compiler program, not just crate identity | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-27 -- Bug-triage automation: end-to-end test, guardrail rebuild, action bumps | `github-issue-triage-and-automation.md` |
| 2026-04-27: GHA workflow security audit and hardening | `repo-security-and-supply-chain.md` |
| 2026-04-27 -- Audit mitigation: pin cargo-dist installer by SHA-256 | `repo-security-and-supply-chain.md` |
| 2026-04-27: Wire 12 newly-added services into the terraform E2E harness | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-27 -- coverage report under-counted services (164 reported vs 211 actual) | `workspace-readmes-and-service-examples.md` |
| 2026-04-27 — Quality Gate sweep across all 223 service crates | `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-28 — directconnect tag operations unblocked | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-28 — Terraform E2E batch fixes (8 services, 36 tests) | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-28 — Merge of terraform E2E worktrees back to main | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-28 — sccache-wrapper: cache `--test` invocations and record rustc exit status | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-28: /tackle-todos sweep — 17 items resolved across 4 work units | `quality-gate-workflow-and-recurring-findings.md`, `core-service-expansion-and-coverage.md`, `stub-handler-audit-and-promotion.md` |
| 2026-04-28: rename winterbaume-databasemigrationservice → winterbaume-databasemigration | `workspace-readmes-and-service-examples.md` |
| 2026-04-28 (round 2): /tackle-todos follow-up sweep — 9 more items resolved | `quality-gate-workflow-and-recurring-findings.md`, `stateful-service-and-blob-store.md`, `workspace-readmes-and-service-examples.md` |
| 2026-04-28 — README protocol column: filling missing CRATE_DISPLAY_INFO entries | `workspace-readmes-and-service-examples.md` |
| 2026-04-28 (wrap-up): /tackle-todos final verification | `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-28 (postscript): databasemigration rename — verification outcome | `workspace-readmes-and-service-examples.md` |
| 2026-04-28 (round 3): /tackle-todos third sweep — 9 more items resolved + cascade fixes | `quality-gate-workflow-and-recurring-findings.md`, `core-service-expansion-and-coverage.md`, `smithy-codegen-and-wire-serialization.md`, `terraform-resource-converters.md` |
| 2026-04-28 (autonomous-loop): server build verification | `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-28: sccache-wrapper cache_restore filename-rewrite bug | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-28 — Day summary: /tackle-todos × 3 rounds + autonomous verification | `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-28: sccache-wrapper cross-CARGO_TARGET_DIR cache misses | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-28: smithy-codegen restJson1 @httpPayload output framing | `smithy-codegen-and-wire-serialization.md` |
| 2026-04-28 (round 4): /tackle-todos fourth sweep — 3 more items resolved | `quality-gate-workflow-and-recurring-findings.md`, `smithy-codegen-and-wire-serialization.md` |
| 2026-04-28 (loop iteration): restJson1 @httpPayload regen sweep | `smithy-codegen-and-wire-serialization.md` |
| 2026-04-28: replaced upstream partiql-parser with hand-rolled DDB-PartiQL parser | `dynamodb-partiql-integration.md` |
| 2026-04-28 — CI uses prebuilt libduckdb instead of the bundled from-source compile | `query-service-sql-engine-backends.md` |
| 2026-04-28 (follow-up): AWS-spec audit and comprehensive test additions | `dynamodb-partiql-integration.md` |
| 2026-04-28 (follow-up): release-build and `--no-default-features` semantics for the duckdb prebuilt change | `query-service-sql-engine-backends.md` |
| 2026-04-28 (continue): three small follow-ups | `dynamodb-partiql-integration.md`, `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-28 (final): session close | `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-28 (follow-up): Expression IR + full arithmetic / sub-expression parity | `dynamodb-partiql-integration.md` |
| 2026-04-29: EXISTS conditional function support | `dynamodb-partiql-integration.md` |
| 2026-04-29 (follow-up): EXISTS placement — empirical AWS verification | `dynamodb-partiql-integration.md` |
| 2026-04-29 — Wire DuckDB SQL engine into winterbaume-server behind a feature flag | `query-service-sql-engine-backends.md` |
| 2026-04-29: AWS-fidelity sweep for all PartiQL conditional functions | `dynamodb-partiql-integration.md` |
| 2026-04-29: deep-sleep refresh | `journal-ltm-maintenance-workflows.md`, `pluggable-backends-and-query-execution-synthesis.md`, `repo-maintenance-and-agent-workflows-synthesis.md`, `service-implementation-and-validation-synthesis.md` |
| 2026-04-29: /tackle-todos round 5 ( 8 work units across 3 risk-graded waves ) | `quality-gate-workflow-and-recurring-findings.md`, `dynamodb-partiql-integration.md`, `terraform-resource-converters.md`, `query-service-sql-engine-backends.md`, `workspace-readmes-and-service-examples.md` |
| 2026-04-29: round-5 worktree merge-back -- patch-process drift recovery | `parallel-agent-build-and-worktree-practices.md`, `service-implementation-and-validation-synthesis.md` |
| /tackle-todos round 5 -- final merge-back outcome ( supersedes earlier same-day entry ) | `parallel-agent-build-and-worktree-practices.md`, `service-implementation-and-validation-synthesis.md`, `terraform-resource-converters.md` |
| 2026-04-30: terraform E2E ( CI run 25089387163 ) -- 18 failing tests across four root causes | `terraform-e2e-harness-and-fix-coverage.md`, `smithy-codegen-and-wire-serialization.md`, `service-implementation-and-validation-synthesis.md` |
| /tackle-todos round 6 -- in-place sweep, no worktrees | `quality-gate-workflow-and-recurring-findings.md`, `dynamodb-partiql-integration.md` |
| 2026-04-30: sccache-wrapper cache not shared across agents -- root cause was bypassing `cargo.sh` | `sccache-wrapper-cross-worktree-cache.md`, `repo-maintenance-and-agent-workflows-synthesis.md`, `parallel-agent-build-and-worktree-practices.md` |
| 2026-04-30: end-to-end verification of the cache-sharing harness changes | `sccache-wrapper-cross-worktree-cache.md` |
| Athena + DuckDB E2E Test Against Prebuilt Server Binary (2026-04-30) | `query-service-sql-engine-backends.md` |
| DynamoDB + Redis Backend E2E Test Against Prebuilt Server Binary (2026-04-30) | `pluggable-service-backends-and-redis-storage.md`, `dynamodb-partiql-integration.md` |
| DynamoDB + Redis Backend Extended E2E Sweep (2026-04-30) | `pluggable-service-backends-and-redis-storage.md`, `dynamodb-partiql-integration.md` |
| SQS + Redis Backend E2E Test Against Prebuilt Server Binary (2026-04-30) | `pluggable-service-backends-and-redis-storage.md` |
| SES SendEmail Examples and State/View Disparity Sweep (2026-04-30) | `stateful-service-and-blob-store.md` |
| DynamoDB Sort-Key Filter Fix (2026-04-30) | `dynamodb-partiql-integration.md`, `pluggable-service-backends-and-redis-storage.md` |
| Terraform E2E CI Triage -- 13 Failing Tests + write-e2e-tests Skill Hardening (2026-04-30) | `terraform-e2e-harness-and-fix-coverage.md`, `quality-gate-workflow-and-recurring-findings.md` |
| SQS + Redis Backend Bug Sweep -- Five Fixes (2026-04-30) | `pluggable-service-backends-and-redis-storage.md` |
| 2026-04-30 -- /tackle-todos sweep (codeguru wire migrations + sccache-wrapper d-file fix + provider-v6 enum-casing audit) | `quality-gate-workflow-and-recurring-findings.md`, `smithy-codegen-and-wire-serialization.md`, `sccache-wrapper-cross-worktree-cache.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| E2E Verification of DynamoDB-on-Redis and SQS-on-Redis Fix Sweeps (2026-05-01) | `pluggable-service-backends-and-redis-storage.md`, `dynamodb-partiql-integration.md` |
| 2026-05-01 -- /tackle-todos sweep continued (CBOR-XML adapter audit + scoreboard ready-prune + DuckDB end-to-end test) | `query-service-sql-engine-backends.md`, `sccache-wrapper-cross-worktree-cache.md`, `quality-gate-workflow-and-recurring-findings.md` |
| 2026-05-01 -- /tackle-todos sweep continued (appintegrations audit + sccache-wrapper GC dry-run hook) | `quality-gate-workflow-and-recurring-findings.md`, `sccache-wrapper-cross-worktree-cache.md` |
| 2026-05-01 -- /tackle-todos sweep continued (triage prompt + Models rate-limit backoff) | `github-issue-triage-and-automation.md`, `quality-gate-workflow-and-recurring-findings.md` |
| 2026-05-01 -- /tackle-todos sweep continued (8 skill-prompt hardening items) | `quality-gate-workflow-and-recurring-findings.md` |
| 2026-05-01 -- /tackle-todos sweep continued (workspace check + DMS test port + dedup) | `quality-gate-workflow-and-recurring-findings.md`, `workspace-readmes-and-service-examples.md` |
| 2026-05-01 -- /tackle-todos sweep continued (crate-display-info-derive: protocol derived from Smithy) | `workspace-readmes-and-service-examples.md` |
| 2026-05-01 -- deep-sleep service-note extraction workflow | `journal-ltm-maintenance-workflows.md` |
| 2026-05-01 -- /tackle-todos sweep continued (triage guardrail-flag audit trail) | `github-issue-triage-and-automation.md` |
| 2026-05-01 -- deep-sleep synthesis and service-note extraction | `journal-ltm-maintenance-workflows.md`, `cross-service-integration-and-engine-boundaries-synthesis.md`, `INDEX.md` |
| 2026-05-01 -- /tackle-todos sweep continued (stale-item audit + athena failure detail + server rpath docs) | `query-service-sql-engine-backends.md`, `quality-gate-workflow-and-recurring-findings.md` |
| 2026-05-01 -- deep-sleep service-note mode refresh | `journal-ltm-maintenance-workflows.md` |
| 2026-05-01 -- EC2 remaining-operations push: 484 -> 752 / 756 SDK ops | `ec2-operation-expansion-and-invariants.md`, `ec2-crate-split-and-feature-gating.md`, `terraform-resource-converters.md` |
| 2026-05-01 -- TODO.md active-backlog cleanup | `quality-gate-workflow-and-recurring-findings.md`, `journal-ltm-maintenance-workflows.md` |
| 2026-05-01 -- v1/v2 service-pair state coherence audit | `cross-service-state-coherence.md` |
| 2026-05-01 -- control-plane / data-plane state coherence audit | `cross-service-state-coherence.md` |
| 2026-05-01 -- Quality Gate: winterbaume-ec2 | `quality-gate-workflow-and-recurring-findings.md`, `ec2-operation-expansion-and-invariants.md` |
| 2026-05-02 -- Scenario test plan: winterbaume-ec2 | `aws-doc-test-plan-catalog.md`, `ec2-operation-expansion-and-invariants.md` |
| 2026-05-02 -- Service dossier scenario enhancement | `aws-doc-test-plan-catalog.md`, `workspace-readmes-and-service-examples.md` |
| 2026-05-02 -- Orchestration: invariant inventory becomes a first-class artefact | `aws-doc-test-plan-catalog.md`, `quality-gate-workflow-and-recurring-findings.md`, `journal-ltm-maintenance-workflows.md`, `ec2-operation-expansion-and-invariants.md` |
| 2026-05-02 -- Audit rollout: dry-run findings + 13th EC2 bug fixed | `ec2-operation-expansion-and-invariants.md`, `quality-gate-workflow-and-recurring-findings.md` |
| 2026-05-02 — EC2 e2e terraform CI failures: 7 of 8 fixed | `ec2-operation-expansion-and-invariants.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-05-02 -- Service dossier skill | `journal-ltm-maintenance-workflows.md` |
| 2026-05-02 — EC2 capacity_block_reservation: confirmed upstream AutoFlex bug | `ec2-operation-expansion-and-invariants.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-05-02 — awsJson services skip generated request deserialisers | `smithy-codegen-and-wire-serialization.md` |
| 2026-05-02 — URL query-string parser duplicated across 59 crates | `new-service-implementation-patterns.md`, `TODO.md` |
| 2026-05-02 — Refining the codegen-deserialiser gap: restJson1 and migration debt | `smithy-codegen-and-wire-serialization.md`, `TODO.md` |
| 2026-05-02 — EC2 e2e CI rescue: complete work summary | `ec2-operation-expansion-and-invariants.md`, `terraform-e2e-harness-and-fix-coverage.md`, `smithy-codegen-and-wire-serialization.md` |
| 2026-05-02 — Stub crate: winterbaume-s3files (restJson1, model 2025-05-05) | `new-service-implementation-patterns.md`, `core-service-expansion-and-coverage.md`, `TODO.md` |
| 2026-05-02 — Network-aware service dossier and README transcription audit | `cross-service-state-coherence.md`, `workspace-readmes-and-service-examples.md` |
| 2026-05-02 — winterbaume-s3files: full operation coverage (21/21) | `new-service-implementation-patterns.md`, `core-service-expansion-and-coverage.md`, `quality-gate-workflow-and-recurring-findings.md`, `TODO.md` |
| Deep Sleep Consolidation Record (2026-05-02) | `journal-ltm-maintenance-workflows.md`, `repo-maintenance-and-agent-workflows-synthesis.md` |
| Distill Memories Record (2026-05-02) | `journal-ltm-maintenance-workflows.md`, `repo-maintenance-and-agent-workflows-synthesis.md` |
| Core Documentation Rewrite Record (2026-05-02) | `journal-ltm-maintenance-workflows.md`, `repo-maintenance-and-agent-workflows-synthesis.md` |
| Quality Gate Rewrite Record (2026-05-02) | `journal-ltm-maintenance-workflows.md`, `quality-gate-workflow-and-recurring-findings.md` |
| Smithy Codegen JSON Request Deserialisers + SES Adoption (2026-05-02) | `smithy-codegen-and-wire-serialization.md`, `service-implementation-and-validation-synthesis.md`, `TODO.md` |
| awsQuery + awsJson Deserialiser Adoption Sweep (2026-05-02 continuation) | `smithy-codegen-and-wire-serialization.md`, `service-implementation-and-validation-synthesis.md`, `TODO.md` |
| Mass Wire/Model Regeneration Sweep (2026-05-02) | `smithy-codegen-and-wire-serialization.md`, `TODO.md` |
| DynamoDB Data-Plane Migration (2026-05-03) | `smithy-codegen-and-wire-serialization.md`, `TODO.md` |
| DynamoDB Expression-Layer Final Cleanup (2026-05-03) | `smithy-codegen-and-wire-serialization.md`, `TODO.md` |
| Wire Deserialiser Adoption: Cross-Workstream Status (2026-05-05) | `smithy-codegen-and-wire-serialization.md`, `parallel-agent-build-and-worktree-practices.md`, `service-implementation-and-validation-synthesis.md`, `TODO.md` |
| Wire Deserialiser Adoption Sweep: Consolidated Findings (2026-05-02 to 2026-05-05) | `smithy-codegen-and-wire-serialization.md`, `parallel-agent-build-and-worktree-practices.md`, `service-implementation-and-validation-synthesis.md`, `TODO.md` |
| Pre-Launch Publish-Readiness Hardening (2026-05-08) | `crate-publishing-and-release.md`, `repo-maintenance-and-agent-workflows-synthesis.md`, `TODO.md` |
| release-batch CLI Refinements (2026-05-08) | `crate-publishing-and-release.md`, `repo-maintenance-and-agent-workflows-synthesis.md` |

### Synthesis Relationships

| Synthesis Document | Consolidated Source LTM Documents |
|--------------------|-----------------------------------|
| `service-implementation-and-validation-synthesis.md` | `core-service-expansion-and-coverage.md`, `smithy-codegen-and-wire-serialization.md`, `moto-parity-testing-and-behavioral-gaps.md`, `aws-doc-test-plan-catalog.md`, `terraform-e2e-harness-and-fix-coverage.md`, `terraform-resource-converters.md`, `state-error-shaping-and-handler-boundaries.md`, `stub-handler-audit-and-promotion.md`, `quality-gate-workflow-and-recurring-findings.md`, `new-service-implementation-patterns.md`, `ec2-operation-expansion-and-invariants.md` |
| `runtime-state-and-service-infrastructure-synthesis.md` | `service-implementation-and-validation-synthesis.md`, `stateful-service-and-blob-store.md`, `terraform-resource-converters.md` |
| `pluggable-backends-and-query-execution-synthesis.md` | `pluggable-service-backends-and-redis-storage.md`, `query-service-sql-engine-backends.md`, `dynamodb-partiql-integration.md` |
| `repo-maintenance-and-agent-workflows-synthesis.md` | `parallel-agent-build-and-worktree-practices.md`, `sccache-wrapper-cross-worktree-cache.md`, `workspace-readmes-and-service-examples.md`, `crate-publishing-and-release.md`, `github-issue-triage-and-automation.md`, `repo-security-and-supply-chain.md`, `journal-ltm-maintenance-workflows.md` |
| `cross-service-integration-and-engine-boundaries-synthesis.md` | `aws-inter-service-integration-priorities.md`, `rule-evaluator-and-validator-crates.md`, `pluggable-backends-and-query-execution-synthesis.md`, `runtime-state-and-service-infrastructure-synthesis.md`, `cross-service-state-coherence.md`, `ec2-crate-split-and-feature-gating.md` |

### LTM Documents Intentionally Left Standalone

| LTM Document | Reason |
|--------------|--------|
| `aws-inter-service-integration-priorities.md` | Distinct topic about AWS-documented cross-service seams; no synthesis partner. |
| `cross-service-state-coherence.md` | Distinct topic about v1/v2 shared-backend and control-plane/data-plane coherence; intentionally kept as a focused drill-down. |
| `ec2-crate-split-and-feature-gating.md` | EC2-specific drill-down ( split crate, feature taxonomy, regeneration command ); intentionally specialised. |
| `ec2-operation-expansion-and-invariants.md` | EC2-specific drill-down about near-complete operation expansion, invariant inventory, and audit-script findings; intentionally specialised. |
| `rule-evaluator-and-validator-crates.md` | Distinct topic about reusable evaluator/validator crate architecture ( IAM, Step Functions, WAFv2, Bedrock ); referenced by the service-hardening synthesis but kept standalone for drill-down. |
| `new-service-implementation-patterns.md` | Already covered by `service-implementation-and-validation-synthesis.md`; remains useful as a focused drill-down for service-addition gotchas, SDK test behaviour, and root-resource-first scope selection. |
| `quality-gate-workflow-and-recurring-findings.md` | Already covered by `service-implementation-and-validation-synthesis.md`; remains useful as a direct drill-down for recurring gate failures, deferred-work triage, and the `/tackle-todos` multi-round sweep pattern. |
| `stub-handler-audit-and-promotion.md` | Already covered by `service-implementation-and-validation-synthesis.md`; remains useful as a direct drill-down for `STUB[...]` taxonomy and promotion patterns. |
| `terraform-resource-converters.md` | Already covered by `service-implementation-and-validation-synthesis.md` and `runtime-state-and-service-infrastructure-synthesis.md`; remains useful as a direct drill-down for converter contract details and `StateView`-gated support. |
| `runtime-state-and-service-infrastructure-synthesis.md` | Already a cohesive second-stage synthesis without needing another tier above. |
| `pluggable-backends-and-query-execution-synthesis.md` | Already a cohesive second-stage synthesis without needing another tier above. |

### Service Dossier Updates From Consolidation

| Service Document | Source |
|------------------|--------|
| `services/cloudwatch.md` | Full distillation of CloudWatch multi-protocol wire / Terraform compatibility notes and remaining request-deserialiser migration shape from `smithy-codegen-and-wire-serialization.md`, `service-implementation-and-validation-synthesis.md`, and `terraform-e2e-harness-and-fix-coverage.md`. |
| `services/api-gateway.md` | Reference summary for API Gateway's intentional PATCH-style hybrid request parsing from `smithy-codegen-and-wire-serialization.md`. |
| `services/cloudfront.md` | Reference summary for CloudFront's restXml URL-routing request-deserialiser follow-up from `smithy-codegen-and-wire-serialization.md` and `TODO.md`. |
| `services/s3files.md` | Source-line refresh after `reconcile-journal-ltm`; existing full distillation kept from `new-service-implementation-patterns.md`, `core-service-expansion-and-coverage.md`, and `quality-gate-workflow-and-recurring-findings.md`. |

Open follow-up work extracted during consolidation lives in `.agents/docs/TODO.md`.

## 2026-05-09: release-batch retry pruning after partial 429

### Symptom

During the first-launch publish run, `release-batch` hit a crates.io 429 mid-chunk: in the 5-crate chunk `[winterbaume-amp, winterbaume-amplify, winterbaume-amplifybackend, winterbaume-amplifyuibuilder, winterbaume-apigateway]`, cargo successfully uploaded `winterbaume-amp v0.1.0`, then the upload of `winterbaume-amplify` was rejected with `status 429 Too Many Requests` ( "Please try again after Fri, 08 May 2026 15:32:42 GMT" ). The driver correctly parsed the deadline and slept until the window opened, but the retry re-invoked `cargo release ... -p winterbaume-amp -p winterbaume-amplify ...` with the original 5-crate list. cargo-release failed upfront with `error: winterbaume-amp 0.1.0 is already published to crates.io` and the driver returned a non-zero exit, leaving the run wedged.

### Root cause

The retry loop in `tools/release-batch/src/main.rs` reused the immutable `chunk: &[String]` slice on every iteration. `cargo publish` uploads crates one at a time, and once a crate has landed on crates.io, it is rejected on the next publish call. The driver had no logic to reconcile its in-memory chunk with what crates.io now contained between attempts.

### Fix

Re-query crates.io between retries and prune crates that were already published.

- `tools/release-batch/src/main.rs`: copy each `chunk` into a mutable `working_chunk: Vec<String>` per iteration. After sleeping past the rate-limit deadline, run `working_chunk.retain(|name| !already_on_crates_io(name, version))` and log each pruned entry as `skip {name} v{version}: published during previous attempt`. If `working_chunk` is empty after pruning ( e.g., the 429 hit only on the index-update step after every crate had already uploaded ), treat the chunk as complete instead of failing. Otherwise log the surviving subset and re-issue the cargo invocation with `-p` flags for only those crates.
- Introduced a small `ChunkOutcome { Success, Failure(ExitStatus) }` enum so the inner `loop` can break with either "all done" ( original success or fully-pruned ) or "give up with this status code".

This piggybacks on the existing `already_on_crates_io` helper (which already drives the pre-flight resumability check at the top of `run`), so the retry path uses the same authoritative source of truth as the initial plan.

### Verification

- `./.agents/bin/cargo.sh fmt -p release-batch -- --check` clean.
- `./.agents/bin/cargo.sh clippy -p release-batch --all-targets --all-features -- -D warnings` clean.
- `./.agents/bin/cargo.sh test -p release-batch` — 5 existing `parse_retry_after` tests still pass; no new tests added because the pruning path is gated on real HTTP calls to crates.io and a real `cargo release` invocation, both of which are awkward to mock without restructuring.

### Out of scope

The fix only handles the 429-with-`try again after` recovery path. A non-429 failure that includes "is already published" (e.g., resuming after a manual abort, or re-running a chunk after the operator partially completed it by hand) still aborts; that scenario can be added later by extending the retry classifier in `parse_retry_after`'s call site to also detect "is already published" and prune+retry without sleeping. Not done in this change because the current bug only manifests on the 429 path.

## 2026-05-09: release-batch — recover from "is already published" cargo errors

### Symptom

The first fix above ( re-query crates.io between 429 retries ) was insufficient. On a fresh first-launch run, chunk 1 hit a 429 mid-batch with `winterbaume-amplify` and `winterbaume-amplifybackend` already uploaded, slept, then re-ran cargo with the original 5-crate list. The crates.io API probe ( `GET /api/v1/crates/<name>/<version>` ) did NOT remove the two landed crates from the working chunk — no `skip ... published during previous attempt` lines were emitted. cargo's own pre-flight on the second invocation then aborted with:

```
error: winterbaume-amplify 0.1.0 is already published to crates.io
error: winterbaume-amplifybackend 0.1.0 is already published to crates.io
chunk 1 failed (rc=Some(101)); fix the cause and re-run
```

The driver classified that second failure as non-recoverable ( no "try again after" phrase in the second invocation's output ) and exited.

### Root cause and design lesson

The HTTP-probe path is unreliable as a sole recovery signal. crates.io's `/api/v1/crates/{name}/{version}` endpoint is served via Fastly with caching and is not perfectly synchronous with the registry index that cargo consults during pre-flight. A negative cache hit ( 404 cached just before the publish ) can persist for tens of seconds even after the index has the new version, so a probe a few seconds after upload can return 404 for a crate that cargo will reject as already-published moments later. The previous fix relied on this single signal.

cargo, in contrast, queries the registry index directly during its `cargo publish` pre-flight and emits `error: <crate> <version> is already published to crates.io` when it finds the version present. This is ground truth — same source of truth cargo itself uses to decide whether to abort the publish.

### Fix

Make cargo's own error message the primary recovery signal, and treat the HTTP probe as a secondary best-effort path used only on the 429 sleep branch ( where cargo never reached its pre-flight ).

- `tools/release-batch/src/main.rs`: introduce `parse_already_published(text: &str) -> BTreeSet<String>` that scans cargo output line-by-line for the `... is already published to crates.io` suffix ( with optional `error: ` prefix ) and extracts the crate name from `<crate> <version>` via `rsplit_once(' ')`.
- Restructure the per-chunk retry loop so it computes both signals on every failure:
  - `let already_published = parse_already_published(&captured);`
  - `let rate_limit_deadline = parse_retry_after(&captured);`
  - If neither is present, fail the chunk ( unchanged from before ).
  - If `rate_limit_deadline` is `Some`, increment the rate-limit counter, sleep until the deadline + retry buffer, and ( on retry path only ) also probe each surviving crate via the existing `already_on_crates_io` HTTP check.
  - Always prune crates listed in `already_published` from `working_chunk` regardless of whether a 429 was also reported.
  - If `working_chunk` empties out, mark the chunk `Success`. If it shrinks, log the surviving names and retry; if it stays the same and we hit only the 429 branch, re-issue cargo unchanged after the sleep.
- Rename the retry counter from `attempt` to `rate_limit_attempts` and only bump it on 429s. Prune-driven retries are bounded by `chunk_size` ( each one strictly shrinks the working chunk by at least one crate ), so they don't need to count against `--max-retries`.

### Verification

- Three new unit tests cover `parse_already_published`: a verbatim two-crate failure tail from this run, a warnings-only buffer ( returns empty ), and a line without the `error: ` prefix ( still parsed ).
- `./.agents/bin/cargo.sh fmt -p release-batch -- --check`, `./.agents/bin/cargo.sh clippy -p release-batch --all-targets --all-features -- -D warnings`, and `./.agents/bin/cargo.sh test -p release-batch` all clean ( 8 tests pass; the previous 5 `parse_retry_after` tests still hold ).

### Operator note

If `release-batch` is invoked through a pre-built binary on `$PATH` rather than `cargo run -p release-batch`, the operator must rebuild after pulling this change for the new recovery path to take effect. The retry-loop fix only exists in the binary, not in the input data.

## 2026-05-10: mass-publish post-mortem — dropped tags after partial 429s, umbrella over the 500-dep limit

### Context

The first mass-publish run via `release-batch` ( 44 chunks of 5 crates ) completed most of the workspace but surfaced two issues the operator caught after the fact: (1) many published crates lack their `<crate>-v0.1.0` git tag, and (2) the `winterbaume` umbrella crate failed in chunk 43 with `error: ... status 400 Bad Request: crates.io only allows a maximum number of 500 dependencies`. Chunk 43 aborted before chunk 44, so `winterbaume-server` and `winterbaume-terraform` were never reached either.

### Symptom 1: dropped tags

Per chunk, only the *last* crate to publish in the final retry attempt got its `Pushing` tag step. Every earlier crate that landed during a 429-interrupted attempt was missing a tag. From the operator-supplied chunk-39–chunk-43 log alone, the visible drops were `winterbaume-{synthetics,taxsettings,textract,timestreaminfluxdb,timestreamwrite,transcribe,transfer,trustedadvisor,workspaces,workspacesweb,dynamodb,sfn,bedrockagent,ec2,sqlengine-duckdb,sqs-redis,iam,dynamodb-redis,dynamodbstreams,wafv2}` ( 20 crates ). Probing crates.io for every publishable workspace member showed the real total was **167 published-but-untagged crates** across all chunks, not just the late ones — the same race had happened repeatedly throughout the run.

### Root cause: chunk-level tag push is part of the same `cargo release` invocation that publishes

`cargo release` uploads each crate, then ( after the whole batch completes ) creates and pushes the per-crate annotated tags. When a 429 splits a chunk — say `[A, B, C, D, E]` where A/B uploaded, C hit the rate limit — cargo errors out before the tag step, so A, B, and C ( the in-flight one ) are unsigned, untagged. The previous `release-batch` recovery path then prunes the crates that landed during the failed attempt ( "skip A: published during previous attempt" ) and reissues `cargo release -p C -p D -p E`. That second invocation only knows about C/D/E — it has no way to retroactively tag A and B. cargo-release's tag step runs once per invocation, so dropped tags stay dropped.

This is the mirror image of the `2026-05-09: release-batch retry pruning after partial 429` fix: that change correctly prevented re-uploading already-uploaded crates, but it inherited the assumption that "if cargo skips it, cargo will tag it later" — which doesn't hold across separate cargo-release invocations.

### Symptom 2: umbrella exceeds 500-dep limit

`winterbaume`'s root `Cargo.toml` listed 214 optional service crates under `[dependencies]` plus 455 entries under `[dev-dependencies]` ( 211 `aws-sdk-*` clients + 244 `winterbaume-*` re-imports ), totalling 669. cargo-publish keeps `[dev-dependencies]` in the published manifest when each entry has a version ( workspace inheritance counts ), and crates.io applies the `total dependencies <= 500` cap to that combined list. The 224 example files under `examples/*.rs` were the apparent reason for the dev-deps to exist, but the umbrella sets `autoexamples = false` and never declares any `[[example]]` entry — so cargo never compiles those files, and the dev-deps are entirely unused. Their only effect was to inflate the published manifest above the limit.

### Fixes

- **`Cargo.toml`** ( umbrella ): delete the entire `[dev-dependencies]` block ( 455 lines, lines 1292–1747 of the pre-fix file ). Verified with `cargo check -p winterbaume` ( clean, src/lib.rs imports nothing from `aws_*` ) and `cargo package -p winterbaume --no-verify`: produced manifest now lists 214 `[dependencies.<name>]` entries and 0 dev-deps. Crate size dropped from 519.8 KiB / 87.8 KiB compressed to 339.7 KiB / 66.8 KiB compressed.

- **`tools/release-batch/src/main.rs`**: when a chunk is interrupted by a 429 and the recovery path prunes "published during previous attempt" or "cargo reports already published" crates from `working_chunk`, also call a new `backfill_tag(&tag, args.sign, &root)` for each pruned name. The helper checks `git tag -l <tag>`, and if absent, runs `git tag -s -m "" <tag>` ( or `-a -m ""` if `--sign` was off ) at HEAD followed by `git push origin <tag>`. Failures log a warning and continue rather than aborting the chunk — a missing tag is a recoverable bookkeeping issue, not worth wedging the publish run. Empty tag message matches the existing tags' contents ( cargo-release's `tag-message = "chore: release {{crate_name}} v{{version}}"` template never gets substituted in this workspace, so cargo-release's own tags also have empty subjects/bodies — `git for-each-ref` confirmed `msg=[] body=[]` ).

- **Tag backfill** ( one-off ): create signed annotated tags for the 167 published-but-untagged crates locally, all pointing to HEAD ( `b14f5d40` ) to match the tag-target of the existing 70 tags. Each existing tag in this run targets HEAD via a distinct annotated-tag object ( different SHAs because the tag name and signature differ ), so the backfilled tags are structurally identical apart from the name. The 3 publishable members that are *genuinely* unpublished — `winterbaume`, `winterbaume-server`, `winterbaume-terraform` — were excluded from the backfill via a crates.io probe loop ( 167 returned HTTP 200, 3 returned 404 ).

### Verification

- `./.agents/bin/cargo.sh fmt -p release-batch -- --check` clean; `clippy -p release-batch --all-targets --all-features -- -D warnings` clean; `cargo test -p release-batch` — 8 tests pass ( previous suite, no new ones added; the new helper drives `git` and the network and is awkward to mock without restructuring ).
- Local tag count after backfill: `git tag -l 'winterbaume-*-v0.1.0' | wc -l` reports 237 ( 70 pre-existing + 167 backfilled ). Spot-checked `winterbaume-iam-v0.1.0` via `git cat-file -p`: signed annotated tag, points at `b14f5d40`, empty body — matches the existing tags' shape.
- `cargo package -p winterbaume --no-verify --allow-dirty` succeeded post-fix; extracted `Cargo.toml` from the produced `.crate` and counted `^\[dependencies\.` ( 214 ) and `^\[dev-dependencies\.` ( 0 ).

### Out of scope ( awaiting operator confirmation before pushing )

The local state is ready, but four shared-state actions remain and were not taken in this session:

1. Commit the `Cargo.toml` / `Cargo.lock` / `tools/release-batch/src/main.rs` changes ( signed, per the always-sign-commits rule on this repo ).
2. `git push origin --tags` to push the 167 backfilled tags.
3. `cargo release 0.1.0 -p winterbaume --execute` to publish the umbrella now that the dep count is under the cap.
4. Decide whether to publish `winterbaume-server` and `winterbaume-terraform` as part of the same wave — they were in the original publish plan but never reached because chunk 43 aborted on the umbrella's 400.

### Operator note

The `release-batch` tag-backfill path only triggers on the 429 / "is already published" recovery branch. If a future chunk fails for *any other* reason after partially uploading ( e.g., a transient network 5xx between cargo's `Uploaded X` and the tag push ), the dropped tags will still need a manual `git tag -s ... && git push` cleanup. A more general guard would be to scan cargo's `Uploaded <crate> v<ver>` lines on every failure, not just the 429 path, and tag any uploaded crate whose tag is missing. Not done now because the only failure mode observed in 44 chunks was the 429.
