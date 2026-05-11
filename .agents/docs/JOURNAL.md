# Winterbaume Development Journal

## LTM Consolidation Record

The journal has been audited against `.agents/docs/LTM/` and `.agents/docs/TODO.md` as of 2026-05-11. Every substantive entry that previously lived in this file has had its durable knowledge promoted to one or more LTM documents ( and any open follow-ups recorded in `.agents/docs/TODO.md` ); the consolidated entries themselves have been removed per the `reconcile-journal-ltm` workflow. Everything that needed to survive lives in LTM.

### Section → LTM Document Mapping

| Section | LTM Document |
|---------|--------------|
| 2026-03-28 — add-service skill documentation update for StatefulService views | `stateful-service-and-blob-store.md` |
| 2026-03-28 — Test plan: ec2instanceconnect | `aws-doc-test-plan-catalog.md` |
| 2026-03-28 — Test plan: signer | `aws-doc-test-plan-catalog.md` |
| 2026-03-28 — Test plan: sagemakerruntime | `aws-doc-test-plan-catalog.md` |
| 2026-03-28 — Test plan: sso | `aws-doc-test-plan-catalog.md` |
| 2026-03-28 — Test plan: account | `aws-doc-test-plan-catalog.md` |
| 2026-03-28 — Test plan: costexplorer | `aws-doc-test-plan-catalog.md` |
| 2026-03-28 — Test plan: applicationautoscaling | `aws-doc-test-plan-catalog.md` |
| 2026-03-28 — Session summary: write-tests batch run | `aws-doc-test-plan-catalog.md` |
| 2026-03-28 — Batch terraform apply infrastructure for E2E test harness | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-03-28 — S3 and Route53 moto parity work via generated restXml request deserializers | `smithy-codegen-and-wire-serialization.md`, `core-service-expansion-and-coverage.md` |
| 2026-03-28 — StatefulService State-Change Subscription + Batch Service Views | `stateful-service-and-blob-store.md` |
| 2026-03-28 — Audit — `serde_json::Value` in Public View Structs | `stateful-service-and-blob-store.md` |
| 2026-03-28 — VFS-Backed Blob Store — Full Implementation | `stateful-service-and-blob-store.md` |
| 2026-03-28 — StatefulService views.rs batch completion — remaining 86 services | `stateful-service-and-blob-store.md` |
| 2026-03-28 — Code review — recent StatefulService / blob-store rollout | `stateful-service-and-blob-store.md` |
| 2026-03-28 — StatefulService rollout — final service (winterbaume-textract) | `stateful-service-and-blob-store.md` |
| 2026-03-28 — Code review fixes — VFS path traversal, S3 blob error propagation, S3 merge contract | `stateful-service-and-blob-store.md` |
| 2026-03-28 — Terraform E2E Tests — Events, CloudWatch, Lambda | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-03-28 — Full Async Migration — VFS / BlobStore / StatefulService / Terraform Stack | `stateful-service-and-blob-store.md` |
| 2026-03-28 — Code Review — Current Working Tree | `stateful-service-and-blob-store.md` |
| 2026-03-29 — Fixes for Code Review Findings (2026-03-28) | `stateful-service-and-blob-store.md` |
| 2026-03-29 — Terraform E2E Tests — StepFunctions, Kinesis, CognitoIDP, and Full Suite Fix | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-03-29 — DynamoDB PartiQL Integration | `dynamodb-partiql-integration.md` |
| 2026-03-29 — Route53 Integration Test Expansion | `core-service-expansion-and-coverage.md` |
| 2026-03-29 — Pluggable Backend Traits for SQS and SNS | `pluggable-service-backends-and-redis-storage.md` |
| 2026-03-29 — Terraform E2E Tests — ECS, Route53, EFS, ACM (4 new modules) | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-03-30 — Hand-crafted Response Cleanup in ECS and ACM | `smithy-codegen-and-wire-serialization.md` |
| 2026-03-30 — Codegen Fix Plan — Output-Only Shape `@required` Fields as `Option<T>` | `smithy-codegen-and-wire-serialization.md` |
| 2026-03-30 — DuckDB/PartiQL SQL Engine Backend for Athena and Redshift Data | `query-service-sql-engine-backends.md` |
| 2026-03-30 — STUB annotation convention + large coverage push | `core-service-expansion-and-coverage.md` |
| 2026-03-30 — Redis-backed SQS backend (`winterbaume-sqs-redis`) | `pluggable-service-backends-and-redis-storage.md` |
| 2026-03-30 — Codegen Fix Implemented — Output-Only `@required` Fields as `Option<T>` | `smithy-codegen-and-wire-serialization.md` |
| 2026-03-30 — DynamoDB Pluggable Backend + Redis Implementation | `pluggable-service-backends-and-redis-storage.md` |
| 2026-03-30 — Clippy warning cleanup | `smithy-codegen-and-wire-serialization.md` |
| 2026-03-30 — `reconcile-journal-ltm` skill | `journal-ltm-maintenance-workflows.md` |
| 2026-03-31 — tackle-todos batch — behavioral fixes and httpResponseCode extraction | `smithy-codegen-and-wire-serialization.md`, `parallel-agent-build-and-worktree-practices.md` |
| 2026-03-31 — tackle-todos second pass — IAM XML serialization, DynamoDB ListTables, budgets notifications | `smithy-codegen-and-wire-serialization.md` |
| 2026-03-31 — Fix generator instead of generated files | `smithy-codegen-and-wire-serialization.md` |
| 2026-03-31 — Fix — Broken Indentation in Generated Wrapper Structs | `smithy-codegen-and-wire-serialization.md` |
| 2026-03-31 — Eliminate Dual backend+state Pattern in DynamoDB, SQS, SNS | `pluggable-service-backends-and-redis-storage.md` |
| 2026-03-31 — Clippy: Suppress `non_camel_case_types` and `upper_case_acronyms` in generated code | `smithy-codegen-and-wire-serialization.md` |
| 2026-03-31 — CloudFront and WAFv2 Terraform E2E Tests | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-03-31 — ELBv2, EKS, and Organizations Terraform E2E Tests | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-03-31 — Fix DynamoDB Streams shared state | `stateful-service-and-blob-store.md` |
| 2026-03-31 — README Coverage Update and Examples Crate | `workspace-readmes-and-service-examples.md` |
| 2026-03-31 — Renamed winterbaume-tagging to winterbaume-resourcegroupstagging | `workspace-readmes-and-service-examples.md` |
| 2026-03-31 — README Coverage Summary + Per-Service Examples | `workspace-readmes-and-service-examples.md` |
| 2026-04-01 — Implemented winterbaume-s3control and Fixed DynamoDB Streams Terraform Crate | `core-service-expansion-and-coverage.md`, `stateful-service-and-blob-store.md` |
| 2026-03-31 — Terraform E2E Tests — EC2 | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-03-31 — EC2/VPC Service Implementation (winterbaume-ec2) | `core-service-expansion-and-coverage.md`, `smithy-codegen-and-wire-serialization.md` |
| 2026-04-01 — EC2 compile-error fix | `core-service-expansion-and-coverage.md` |
| 2026-04-01 — EC2 Compute and Storage Operation Implementation | `core-service-expansion-and-coverage.md` |
| 2026-04-01 — DynamoDB AttributeValue Enum Refactor, Streams Change Capture, and Expression Audit | `dynamodb-partiql-integration.md`, `stateful-service-and-blob-store.md` |
| 2026-04-01 — S3 Tables — Full 49/49 Operation Coverage | `core-service-expansion-and-coverage.md` |
| 2026-04-01 — PartiQL Feature Completions | `dynamodb-partiql-integration.md` |
| 2026-04-01 — S3 Tables — Documentation-Derived Test Suite | `aws-doc-test-plan-catalog.md` |
| 2026-04-01 — EC2 Moto Parity Implementation Completion | `moto-parity-testing-and-behavioral-gaps.md`, `core-service-expansion-and-coverage.md` |
| 2026-04-01 — Session Wrap-up — EC2 Parity + Coverage Update | `core-service-expansion-and-coverage.md`, `workspace-readmes-and-service-examples.md` |
| 2026-04-01 — tackle-todos pass — budgets inline notifications, backup lock validation, EC2 default ACL | `core-service-expansion-and-coverage.md`, `smithy-codegen-and-wire-serialization.md` |
| 2026-04-01 — S3 Tables — Write-Tests Session Summary | `aws-doc-test-plan-catalog.md` |
| 2026-04-01 — tackle-todos pass (second) — EC2 ACL, sdb clientToken, serde_json::Value policy | `stateful-service-and-blob-store.md`, `core-service-expansion-and-coverage.md` |
| 2026-04-01 — New Service — API Gateway (REST v1) | `core-service-expansion-and-coverage.md`, `smithy-codegen-and-wire-serialization.md` |
| 2026-04-01 — API Coverage Fix — Moto Column for Stub Services | `workspace-readmes-and-service-examples.md` |
| 2026-04-01 — New Crate — winterbaume-apigatewaymanagementapi | `core-service-expansion-and-coverage.md`, `workspace-readmes-and-service-examples.md` |
| 2026-04-01 — EMR (Elastic MapReduce) Service Crate | `core-service-expansion-and-coverage.md` |
| 2026-04-01 — GuardDuty stub-to-real upgrade (IP sets, threat intel sets, tags) | `core-service-expansion-and-coverage.md` |
| 2026-04-02 — winterbaume-codebuild report group operations | `core-service-expansion-and-coverage.md` |
| 2026-04-02 — AWS SDK crate naming audit | `workspace-readmes-and-service-examples.md` |
| 2026-04-03 — winterbaume-amplify implementation | `core-service-expansion-and-coverage.md` |
| 2026-04-03 — OpenSearch — Core Resource Operations (35 new operations) | `core-service-expansion-and-coverage.md` |
| 2026-04-03 — Coverage Expansion Sprint — 50.3% → 55.6% | `core-service-expansion-and-coverage.md` |
| 2026-04-03 — X-Ray integration test expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03 — S3 Control Integration Test Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03 — Lex Integration Test Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03 — api-coverage Terraform E2E coverage reporting | `workspace-readmes-and-service-examples.md` |
| 2026-04-03 — api-coverage integration-test coverage integration | `workspace-readmes-and-service-examples.md` |
| 2026-04-03 — api-coverage skill self-containment | `workspace-readmes-and-service-examples.md` |
| 2026-04-03 — Comprehensive Redshift Integration Test Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03 — Lex V2 Comprehensive Integration Test Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03 — Timestream Query Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03 — Redshift Data API Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03 — Cognito Identity Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03 — ELBv2 Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03 — STS Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03 — RDS Data API Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03 — AppConfig Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03 — ECR Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03 — WorkSpaces Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03 — SSM Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03 — CodeCommit Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03 — networkmanager integration test coverage expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03 — WorkspacesWeb integration test coverage expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03 — EKS Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03 — Lambda integration test coverage expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03 — CloudWatch Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-03 — BedrockAgent Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03 — Cognito IDP Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03 — Transfer Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-03 — CloudWatch Logs Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-03 — Test Coverage Enhancement Batch | `core-service-expansion-and-coverage.md`, `aws-doc-test-plan-catalog.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-03 — Bulk Test Coverage Expansion — Integration Tests + Terraform E2E | `core-service-expansion-and-coverage.md`, `aws-doc-test-plan-catalog.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-03 — Second Wave — Remaining Integration Tests + Expanded E2E Coverage | `core-service-expansion-and-coverage.md`, `aws-doc-test-plan-catalog.md`, `terraform-e2e-harness-and-fix-coverage.md`, `workspace-readmes-and-service-examples.md` |
| 2026-04-03 — Lambda Integration Test Coverage Expansion | `aws-doc-test-plan-catalog.md` |
| 2026-04-04 — Enhancement Wave — Pushing 60-80% Services Toward 80%+ and Broad E2E Expansion | `core-service-expansion-and-coverage.md`, `aws-doc-test-plan-catalog.md`, `terraform-e2e-harness-and-fix-coverage.md`, `workspace-readmes-and-service-examples.md` |
| 2026-04-04 — refactor-state-errors — Batch Execution Across 33+ Services | `state-error-shaping-and-handler-boundaries.md` |
| 2026-04-04 — PartiQL DML Parser — Clippy Fixes and Edge-Case Test Coverage | `dynamodb-partiql-integration.md` |
| 2026-04-04 — Clippy dead_code fix in generated wire.rs (split path) | `smithy-codegen-and-wire-serialization.md` |
| 2026-04-04 — Clippy Warning Fixes — Three Crates | `smithy-codegen-and-wire-serialization.md` |
| 2026-04-04 — Clippy Fix — `box_collection` in Smithy Codegen | `smithy-codegen-and-wire-serialization.md` |
| 2026-04-05 — TODO Batch — Multi-Service Behavioural Fixes | `moto-parity-testing-and-behavioral-gaps.md`, `core-service-expansion-and-coverage.md` |
| 2026-04-05 — Crate rename — aws-sdk-* naming convention alignment | `workspace-readmes-and-service-examples.md` |
| 2026-04-06 — Snapshot Semantics Audit — Blob-Backed and Large-Payload Services | `stateful-service-and-blob-store.md` |
| 2026-04-06 — BlobBackedService trait redesign — lifetime soundness, dyn-compatibility, and bug fix | `stateful-service-and-blob-store.md` |
| 2026-04-06 — BackendState tokio::sync::RwLock Migration + BlobBackedService Trait | `stateful-service-and-blob-store.md`, `runtime-state-and-service-infrastructure-synthesis.md` |
| 2026-04-06 — Terraform Converter Integration Test Coverage — 100% | `terraform-resource-converters.md` |
| 2026-04-09 — E2E Test Bug Fixes — ECR, AppConfig, CloudWatch, BedrockAgent | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-11 — Terraform E2E Harness — Orphaned Processes + Batch Strategy Overhaul | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-11 — Replace `kill` subprocess with `libc::killpg` in E2E harness | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-12 — Extend api-coverage skill with floci and kumo emulator coverage | `workspace-readmes-and-service-examples.md` |
| 2026-04-11 — Fix Route53 `GetDNSSEC` handler not returning key signing keys | `stub-handler-audit-and-promotion.md` |
| 2026-04-11 — Fix compile errors in `winterbaume-guardduty` integration tests | `stub-handler-audit-and-promotion.md` |
| 2026-04-12 — tackle-todos — Stub Handler Audit and Three Fixes | `stub-handler-audit-and-promotion.md` |
| 2026-04-12 — floci/kumo coverage in api-coverage and update-readme skills | `workspace-readmes-and-service-examples.md` |
| 2026-04-12 — Hard-coded stub audit and STUB[no-state] resolution (acmpca, xray, guardduty) | `stub-handler-audit-and-promotion.md` |
| 2026-04-12 — Plan: IAM policy evaluation engine (`winterbaume-iam-rule-eval`) | `rule-evaluator-and-validator-crates.md` |
| 2026-04-12 — Plan: ASL validation engine (`winterbaume-sfn-asl-eval`) | `rule-evaluator-and-validator-crates.md` |
| 2026-04-12 — Plan: WAFv2 WCU capacity calculator (`winterbaume-wafv2-wcu-eval`) | `rule-evaluator-and-validator-crates.md` |
| 2026-04-12 — Revised plan: split WCU eval into two crates | `rule-evaluator-and-validator-crates.md` |
| 2026-04-12 — Plan: Bedrock flow graph validator (`winterbaume-bedrockagent-flow-eval`) | `rule-evaluator-and-validator-crates.md` |
| 2026-04-12 — Bug fixes: synthetics GetCanaryRuns, codebuild ListReportsForReportGroup, apigateway ImportRestApi | `stub-handler-audit-and-promotion.md` |
| 2026-04-12 — X-Ray — Trace data, summaries, batch retrieval, and sampling targets | `stub-handler-audit-and-promotion.md` |
| 2026-04-12 — Work summary: stub-handler fixes and E2E test activation | `stub-handler-audit-and-promotion.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-12 — Bulk STUB[no-state] Handler Promotion — 8 Service Crates | `stub-handler-audit-and-promotion.md` |
| 2026-04-12 — FIX(terraform-e2e) Integration Test Coverage Audit & Gap Closure | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-12 — Implementation: WAFv2 rule parser and WCU calculator (Phase 1–3) | `rule-evaluator-and-validator-crates.md` |
| 2026-04-12 — IAM Rule Evaluator and SFN ASL Evaluator — Phase 1 Implementation | `rule-evaluator-and-validator-crates.md` |
| 2026-04-12 — Code Review — `winterbaume-sfn`, `winterbaume-sfn-asl-eval`, `winterbaume-wafv2-wcu-calculator`, `winterbaume-wafv2-webacl-rule-parser` | `rule-evaluator-and-validator-crates.md` |
| 2026-04-13 — winterbaume-rds — Migrate handlers from manual XML to wire module | `smithy-codegen-and-wire-serialization.md` |
| 2026-04-13 — Fix WCU calculator ByteMatch cost bug | `rule-evaluator-and-validator-crates.md` |
| 2026-04-13 to 2026-04-14 — Quality Gate Sweep Consolidation | `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-13 — Bedrock Flow Definition Parser and Validator | `rule-evaluator-and-validator-crates.md` |
| 2026-04-14 — smithy-codegen rpc-v2-cbor Protocol Support | `smithy-codegen-and-wire-serialization.md` |
| 2026-04-14 — RDS: Migrate handlers from manual query parsing to wire module deserialisation | `smithy-codegen-and-wire-serialization.md` |
| 2026-04-14 — Tackle-TODOs sweep — views expansion, stub promotion, and regression coverage | `quality-gate-workflow-and-recurring-findings.md`, `stub-handler-audit-and-promotion.md`, `stateful-service-and-blob-store.md` |
| 2026-04-14 — Fix kumo coverage parser; regenerate README and API coverage report | `workspace-readmes-and-service-examples.md`, `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-04-14 — Tackle-TODOs sweep 2 — validation, mock improvement, and stub promotion | `quality-gate-workflow-and-recurring-findings.md`, `stub-handler-audit-and-promotion.md`, `rule-evaluator-and-validator-crates.md` |
| 2026-04-15 — CI Failure Fixes — Clippy duplicated_attributes, Rustfmt, lexmodelsv2 unwrap | `smithy-codegen-and-wire-serialization.md`, `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-04-14 — QG-7.2 sesv2 — Replace hand-written JSON response bodies | `smithy-codegen-and-wire-serialization.md`, `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-14 to 2026-04-15 — Tackle-TODOs sweep 3 — QG-7.2 wire serialiser migration and rpc-v2-cbor timestamps | `smithy-codegen-and-wire-serialization.md`, `quality-gate-workflow-and-recurring-findings.md`, `service-implementation-and-validation-synthesis.md` |
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
| 2026-04-15 — CI Failure Fix — Stale Generated Files and Cascading Breakage | `smithy-codegen-and-wire-serialization.md`, `stateful-service-and-blob-store.md`, `terraform-resource-converters.md` |
| 2026-04-16 — CI Failure Fix — Additional Clippy Lints in Generated Files | `smithy-codegen-and-wire-serialization.md`, `service-implementation-and-validation-synthesis.md` |
| 2026-04-16 — CI Failure Fix — `--all-targets` Uncovers Test and Example Warnings | `service-implementation-and-validation-synthesis.md`, `repo-maintenance-and-agent-workflows-synthesis.md`, `workspace-readmes-and-service-examples.md` |
| 2026-04-16 — CI Failure Fix — CloudWatch DeleteAnomalyDetector Missing SingleMetricAnomalyDetector Support | `service-implementation-and-validation-synthesis.md` |
| 2026-04-18 — CI Failure Fix — WAFv2 Clippy + Missing E2E `#[ignore]` Attributes | `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-04-18 — CI Failure Fix — Cascading Clippy + Test + E2E Fixes (runs 24574987225, 24576098332, 24577669727) | `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-04-18 — CI Failure Fix — Redshift Data `clippy::sort_by_key` (run 24592718536) | `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-04-18 — Fix README API coverage table — missing protocols and duplicate entries | `workspace-readmes-and-service-examples.md` |
| 2026-04-18 — Removal of `winterbaume-stubs` crate | `workspace-readmes-and-service-examples.md`, `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-04-19 — FIX(terraform-e2e) Audit — Full Legitimacy Review | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-18 — CI Failure Fix — Bedrock Agent `clippy::collapsible_match` (run 24594377293) | `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-04-18 — Hard-coded mock TODO sweep | `stub-handler-audit-and-promotion.md` |
| 2026-04-18 — Bulk Release Preparation — Crate Publishing Readiness Audit | `crate-publishing-and-release.md` |
| 2026-04-19 — Bulk Release Preparation — Continued | `crate-publishing-and-release.md` |
| 2026-04-19 — Athena — TagResource and UntagResource implementation | `core-service-expansion-and-coverage.md` |
| 2026-04-19 — Bulk STUB[no-state] Elimination — All Crates | `stub-handler-audit-and-promotion.md`, `core-service-expansion-and-coverage.md` |
| 2026-04-20 — CI fix — CloudFormation `test_signal_resource` and `test_continue_update_rollback` | `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-04-20 — Deep Sleep — Synthesis Refresh | `journal-ltm-maintenance-workflows.md` |
| 2026-04-20 — Terraform Converter Coverage Audit and Mass Enhancement | `terraform-resource-converters.md` |
| 2026-04-20 — StateView Expansion for Poor/Fair Converters | `terraform-resource-converters.md` |
| 2026-04-20 — Terraform Converter Mass Expansion — Complete Session Summary | `terraform-resource-converters.md` |
| 2026-04-20 — Converter Field Coverage Enhancement — All Converters to Excellent | `terraform-resource-converters.md` |
| 2026-04-20 — Terraform Converter Skill, Documentation, and TODO Extraction | `terraform-resource-converters.md` |
| 2026-04-21 — Deep Sleep — Synthesis Refresh | `journal-ltm-maintenance-workflows.md` |
| 2026-04-21 — Tackle-TODOs — Terraform Nested Block Implementation Plans | `terraform-resource-converters.md` |
| 2026-04-21 — Integration Test Fixes — costexplorer and dynamodb | `core-service-expansion-and-coverage.md` |
| 2026-04-21 — Terraform Nested Block Implementation — Batch 1 Complete | `terraform-resource-converters.md` |
| 2026-04-21 — AWS Inter-service Integration Map | `aws-inter-service-integration-priorities.md` |
| 2026-04-21 — Peer Review of Cross-service Findings and Batch 1 Journal Entries | `terraform-resource-converters.md`, `aws-inter-service-integration-priorities.md` |
| 2026-04-21 — Cross-Service Analysis — Athena-Glue Integration Gap | `aws-inter-service-integration-priorities.md`, `query-service-sql-engine-backends.md`, `pluggable-backends-and-query-execution-synthesis.md` |
| 2026-04-21 — Terraform Converter Registration (tackle-todos) | `terraform-resource-converters.md`, `parallel-agent-build-and-worktree-practices.md` |
| 2026-04-21 — E2E Terraform Test Failures — Provider Compatibility Fixes | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-22 — Make DuckDB connection injectable in winterbaume-sqlengine-duckdb | `query-service-sql-engine-backends.md` |
| 2026-04-22 — Redshift `ClusterAvailabilityStatus` fix — E2E Terraform failures | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-22 — Quality Gate: All Service Crates (164 services) | `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-22 — list-of-timestamp CBOR codegen: deep-dive findings | `smithy-codegen-and-wire-serialization.md` |
| 2026-04-22 — QG §7 Tackle-TODOs Sweep — Wire Serialiser Migration | `smithy-codegen-and-wire-serialization.md`, `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-22 — Fix Redshift E2E failures — AvailabilityZoneRelocationStatus | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-22 — Terraform Converter Nested Block Mass Expansion | `terraform-resource-converters.md` |
| 2026-04-23 — Redshift E2E Test Failures — Provider Crash Fixes | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-23 — Deep Sleep — Synthesis Refresh | `journal-ltm-maintenance-workflows.md` |
| 2026-04-23 — Correction: `replication_overwrite_protection` is an `awscc` provider attribute, not `aws` | `journal-ltm-maintenance-workflows.md` |
| 2026-04-23 — Quality Gate: All Service Crates | `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-23 — Session Summary — Memory Maintenance and Quality Gate Sweep | `journal-ltm-maintenance-workflows.md`, `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-24 — BlobStore-backed state recovery & VFS path-traversal hardening | `stateful-service-and-blob-store.md` |
| 2026-04-24 — Hand-crafted XML elimination in S3 and EC2 handlers | `smithy-codegen-and-wire-serialization.md`, `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-24 — tackle-todos: DynamoDB and Terraform E2E fixes | `dynamodb-partiql-integration.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-24 — Quality Gate: workspace service-crate audit | `quality-gate-workflow-and-recurring-findings.md`, `smithy-codegen-and-wire-serialization.md`, `dynamodb-partiql-integration.md`, `terraform-resource-converters.md` |
| 2026-04-24 — Account / region separation contract and Lambda scope fix | `terraform-resource-converters.md`, `runtime-state-and-service-infrastructure-synthesis.md` |
| 2026-04-24 — Peer review — CBOR timestamp-list codegen and CloudWatch handler migration | `smithy-codegen-and-wire-serialization.md` |
| 2026-04-24 — BlobStore account/region isolation | `stateful-service-and-blob-store.md`, `runtime-state-and-service-infrastructure-synthesis.md` |
| 2026-04-24 — Design — Multi-region Terraform extraction (terraform-extract-region-symmetry) | `terraform-resource-converters.md` |
| 2026-04-24 — Peer review — Multi-region Terraform extraction design | `terraform-resource-converters.md` |
| 2026-04-24 — Revised design — Multi-region Terraform extraction (Option C) | `terraform-resource-converters.md` |
| 2026-04-24 — Peer review — Multi-region Terraform extraction Option C | `terraform-resource-converters.md` |
| 2026-04-24 — Final design — Multi-scope Terraform conversion (Option D) | `terraform-resource-converters.md` |
| 2026-04-24 — Final design — Multi-region Terraform extraction (Option D) | `terraform-resource-converters.md` |
| 2026-04-24 — E2E Terraform Test Failure Triage and Fixes | `terraform-e2e-harness-and-fix-coverage.md`, `smithy-codegen-and-wire-serialization.md`, `terraform-resource-converters.md` |
| 2026-04-24 — Fix clippy warnings in smithy-codegen | `smithy-codegen-and-wire-serialization.md` |
| 2026-04-24 — Implementation — Multi-scope Terraform extraction (Option D) | `terraform-resource-converters.md`, `runtime-state-and-service-infrastructure-synthesis.md` |
| 2026-04-25 — Access logging for winterbaume-server | `workspace-readmes-and-service-examples.md` |
| 2026-04-25 — Terraform S3 smoke test failure — virtual-hosted-style requests | `terraform-e2e-harness-and-fix-coverage.md`, `workspace-readmes-and-service-examples.md` |
| 2026-04-25 — Glacier blob-backed state recovery | `stateful-service-and-blob-store.md` |
| 2026-04-25 — Tackle TODOs: smithy-codegen items 2 and 3 | `smithy-codegen-and-wire-serialization.md`, `terraform-resource-converters.md` |
| 2026-04-25 — sccache-wrapper tool for cross-worktree cache efficiency | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-25 — Batch Service Implementation — 6 New AWS Services | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-25 — Cross-worktree Rust compilation caching in sccache-wrapper | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-25 — sccache-wrapper — cache key normalisation fix + singleflight | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-25 — sccache-wrapper — external crate cache MISS caused by `--diagnostic-width` | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-25 — sccache-wrapper — cache pollution from target/host dual compilation | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-26 — Stale sccache server hangs builds | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-26 — sccache-wrapper — Replace C FFI flock() with fslock crate | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-26 — sccache-wrapper — incremental stripping, diagnostic storage, and --dump-cache | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-26 — Reflog-derived chronological work summary (2026-03-26 to 2026-04-26) | `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-04-26 — Implement winterbaume-aiops crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — sccache-wrapper — cross-worktree poisoning fix via `.cachekey` sidecars | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-26 — Implement winterbaume-amplifybackend crate (partial) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — Implement winterbaume-appconfigdata crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — GitHub triage scaffolding: service labels + bug-report form | `github-issue-triage-and-automation.md` |
| 2026-04-26 — Implement winterbaume-appfabric crate (partial) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — Expand service labels to cover unimplemented AWS services | `github-issue-triage-and-automation.md` |
| 2026-04-26 — Implement winterbaume-appflow crate (partial) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — Implement winterbaume-applicationcostprofiler crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — Session findings and work summary | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — Add feature-request issue template | `github-issue-triage-and-automation.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — Bug-triage automation — GitHub Actions + sidecar memory branch | `github-issue-triage-and-automation.md`, `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-04-26 — Terraform converters and E2E tests for new crates | `terraform-resource-converters.md`, `terraform-e2e-harness-and-fix-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — Deep Sleep — Synthesis Refresh | `journal-ltm-maintenance-workflows.md` |
| 2026-04-26 — Implement winterbaume-artifact crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — Staged-change review comments | `terraform-resource-converters.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — Cargo target-directory lock contention between concurrent agents | `parallel-agent-build-and-worktree-practices.md`, `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-26 — EC2 Placement Group support added | `core-service-expansion-and-coverage.md`, `terraform-resource-converters.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-26 — Implement winterbaume-autoscalingplans crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — Session checkpoint after 8 service crates + 3 Terraform converters | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md`, `parallel-agent-build-and-worktree-practices.md` |
| 2026-04-26 — Session-final findings and work summary | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — Skill workflow update — scenario tests folded into write-tests, add-service delegates to it | `new-service-implementation-patterns.md` |
| 2026-04-26 — triage-bug guardrail against prompt injection | `github-issue-triage-and-automation.md` |
| 2026-04-26 — AI cross-language guardrail (follow-up) | `github-issue-triage-and-automation.md` |
| 2026-04-26 — sccache-wrapper scoreboard | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-26 — guardrail simplified to AI-only (revision) | `github-issue-triage-and-automation.md` |
| 2026-04-26 — EC2 NetworkInterfacePermission and InstanceConnectEndpoint (cont.) | `core-service-expansion-and-coverage.md` |
| 2026-04-26 — guardrail prompt hardened from real-world attack catalogue | `github-issue-triage-and-automation.md` |
| 2026-04-26 — guardrail prompt: stylised-alphabet substitutions | `github-issue-triage-and-automation.md` |
| 2026-04-26 — guardrail prompt: translation laundering | `github-issue-triage-and-automation.md` |
| 2026-04-26 — guardrail prompt: compositional / derivational translation | `github-issue-triage-and-automation.md` |
| 2026-04-26 — EC2 CapacityReservation operations (cont.) | `core-service-expansion-and-coverage.md` |
| 2026-04-26 — new service: cloud9 (AWS Cloud9) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — new service: cloudfront-keyvaluestore (CloudFront KeyValueStore) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — new service: cloudsearch-domain (CloudSearch Domain data plane) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — new service: cloudtrail-data (CloudTrail Data Service) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — new service: connectparticipant (Connect Participant data plane) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — new service: controlcatalog (Control Catalog) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — new service: freetier (Free Tier) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — new service: pca-connector-scep (Private CA Connector for SCEP) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — new service: personalize-events (Personalize Events) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — new service: personalize-runtime (Personalize Runtime) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — new service: pi (Performance Insights) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — new service: pinpoint-sms-voice (Pinpoint SMS Voice v1) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — new service: pricing (AWS Price List Service) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — new service: rbin (Recycle Bin) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — new service: route53-recovery-cluster (Route 53 ARC — Cluster data plane) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — new service: s3outposts (S3 on Outposts) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — new service: savingsplans (Savings Plans) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — new service: simspaceweaver (SimSpace Weaver) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — new service: snow-device-management (Snow Device Management) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — new service: ssm-quicksetup (Systems Manager Quick Setup) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — new service: support-app (Support App) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — new service: taxsettings (Tax Settings) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-26 — new service: trustedadvisor (Trusted Advisor v2) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 — new service: appintegrations (Amazon Connect AppIntegrations) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 — new service: amplifyuibuilder (Amplify UI Builder) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 — new service: application-discovery-service | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 — new service: application-insights | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 — new service: application-signals (CloudWatch Application Signals) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 — new service: backup-gateway | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 — new service: backupsearch (AWS Backup Search) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 — new service: bcm-dashboards (BCM Dashboards) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 — new service: bcm-data-exports (BCM Data Exports) | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 — new service: bcm-recommended-actions | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 — Implement winterbaume-billing crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 — Implement winterbaume-braket crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 — Implement winterbaume-chimesdkmeetings crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 — Implement winterbaume-codegurureviewer crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 — Implement winterbaume-codegurusecurity crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 — Implement winterbaume-codestarnotifications crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 — Implement winterbaume-cognitosync crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 — Implement winterbaume-connectcontactlens crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 — Implement winterbaume-costandusagereport crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 — Implement winterbaume-costoptimizationhub crate | `core-service-expansion-and-coverage.md`, `new-service-implementation-patterns.md` |
| 2026-04-27 — EC2 coverage expansion — image attributes + egress-only IGW Terraform | `core-service-expansion-and-coverage.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-27 — EC2 coverage expansion — NACL + customer gateway Terraform | `core-service-expansion-and-coverage.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-27 — Split `winterbaume-ec2` into `winterbaume-ec2` + `winterbaume-ec2-generated` | `ec2-crate-split-and-feature-gating.md`, `core-service-expansion-and-coverage.md` |
| 2026-04-27 — Phase 2: feature-gate `winterbaume-ec2-generated` operations | `ec2-crate-split-and-feature-gating.md` |
| 2026-04-27 — EC2 split / feature-gating: agent guidance | `ec2-crate-split-and-feature-gating.md` |
| 2026-04-27 — sccache-wrapper error handling — residual-error fixes | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-27 — sccache-wrapper: GC mode | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-27 — sccache-wrapper: GC must group by compiler program, not just crate identity | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-27 — Bug-triage automation: end-to-end test, guardrail rebuild, action bumps | `github-issue-triage-and-automation.md` |
| 2026-04-27 — GHA workflow security audit and hardening | `repo-security-and-supply-chain.md` |
| 2026-04-27 — Audit mitigation: pin cargo-dist installer by SHA-256 | `repo-security-and-supply-chain.md` |
| 2026-04-27 — Wire 12 newly-added services into the terraform E2E harness | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-27 — coverage report under-counted services (164 reported vs 211 actual) | `workspace-readmes-and-service-examples.md` |
| 2026-04-27 — Quality Gate sweep across all 223 service crates | `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-28 — directconnect tag operations unblocked | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-28 — Terraform E2E batch fixes (8 services, 36 tests) | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-28 — Merge of terraform E2E worktrees back to main | `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-04-28 — sccache-wrapper: cache `--test` invocations and record rustc exit status | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-28 — /tackle-todos sweep — 17 items resolved across 4 work units | `quality-gate-workflow-and-recurring-findings.md`, `core-service-expansion-and-coverage.md`, `stub-handler-audit-and-promotion.md` |
| 2026-04-28 — rename winterbaume-databasemigrationservice → winterbaume-databasemigration | `workspace-readmes-and-service-examples.md` |
| 2026-04-28 — /tackle-todos follow-up sweep — 9 more items resolved (round 2) | `quality-gate-workflow-and-recurring-findings.md`, `stateful-service-and-blob-store.md`, `workspace-readmes-and-service-examples.md` |
| 2026-04-28 — README protocol column: filling missing CRATE_DISPLAY_INFO entries | `workspace-readmes-and-service-examples.md` |
| 2026-04-28 — /tackle-todos final verification (wrap-up) | `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-28 — databasemigration rename — verification outcome (postscript) | `workspace-readmes-and-service-examples.md` |
| 2026-04-28 — /tackle-todos third sweep — 9 more items resolved + cascade fixes (round 3) | `quality-gate-workflow-and-recurring-findings.md`, `core-service-expansion-and-coverage.md`, `smithy-codegen-and-wire-serialization.md`, `terraform-resource-converters.md` |
| 2026-04-28 — server build verification (autonomous-loop) | `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-28 — sccache-wrapper cache_restore filename-rewrite bug | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-28 — Day summary: /tackle-todos × 3 rounds + autonomous verification | `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-28 — sccache-wrapper cross-CARGO_TARGET_DIR cache misses | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-28 — smithy-codegen restJson1 @httpPayload output framing | `smithy-codegen-and-wire-serialization.md` |
| 2026-04-28 — /tackle-todos fourth sweep — 3 more items resolved (round 4) | `quality-gate-workflow-and-recurring-findings.md`, `smithy-codegen-and-wire-serialization.md` |
| 2026-04-28 — restJson1 @httpPayload regen sweep (loop iteration) | `smithy-codegen-and-wire-serialization.md` |
| 2026-04-28 — replaced upstream partiql-parser with hand-rolled DDB-PartiQL parser | `dynamodb-partiql-integration.md` |
| 2026-04-28 — CI uses prebuilt libduckdb instead of the bundled from-source compile | `query-service-sql-engine-backends.md` |
| 2026-04-28 — AWS-spec audit and comprehensive test additions (follow-up) | `dynamodb-partiql-integration.md` |
| 2026-04-28 — release-build and `--no-default-features` semantics for the duckdb prebuilt change (follow-up) | `query-service-sql-engine-backends.md` |
| 2026-04-28 — three small follow-ups (continue) | `dynamodb-partiql-integration.md`, `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-28 — session close (final) | `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-28 — Expression IR + full arithmetic / sub-expression parity (follow-up) | `dynamodb-partiql-integration.md` |
| 2026-04-29 — EXISTS conditional function support | `dynamodb-partiql-integration.md` |
| 2026-04-29 — EXISTS placement — empirical AWS verification (follow-up) | `dynamodb-partiql-integration.md` |
| 2026-04-29 — Wire DuckDB SQL engine into winterbaume-server behind a feature flag | `query-service-sql-engine-backends.md` |
| 2026-04-29 — AWS-fidelity sweep for all PartiQL conditional functions | `dynamodb-partiql-integration.md` |
| 2026-04-29 — deep-sleep refresh | `journal-ltm-maintenance-workflows.md`, `pluggable-backends-and-query-execution-synthesis.md`, `repo-maintenance-and-agent-workflows-synthesis.md`, `service-implementation-and-validation-synthesis.md` |
| 2026-04-29 — /tackle-todos round 5 ( 8 work units across 3 risk-graded waves ) | `quality-gate-workflow-and-recurring-findings.md`, `dynamodb-partiql-integration.md`, `terraform-resource-converters.md`, `query-service-sql-engine-backends.md`, `workspace-readmes-and-service-examples.md` |
| 2026-04-29 — round-5 worktree merge-back — patch-process drift recovery | `parallel-agent-build-and-worktree-practices.md`, `service-implementation-and-validation-synthesis.md` |
| 2026-04-29 — /tackle-todos round 5 — final merge-back outcome ( supersedes earlier same-day entry ) | `parallel-agent-build-and-worktree-practices.md`, `service-implementation-and-validation-synthesis.md`, `terraform-resource-converters.md` |
| 2026-04-30 — terraform E2E ( CI run 25089387163 ) — 18 failing tests across four root causes | `terraform-e2e-harness-and-fix-coverage.md`, `smithy-codegen-and-wire-serialization.md`, `service-implementation-and-validation-synthesis.md` |
| 2026-04-30 — /tackle-todos round 6 — in-place sweep, no worktrees | `quality-gate-workflow-and-recurring-findings.md`, `dynamodb-partiql-integration.md` |
| 2026-04-30 — sccache-wrapper cache not shared across agents — root cause was bypassing `cargo.sh` | `sccache-wrapper-cross-worktree-cache.md`, `repo-maintenance-and-agent-workflows-synthesis.md`, `parallel-agent-build-and-worktree-practices.md` |
| 2026-04-30 — end-to-end verification of the cache-sharing harness changes | `sccache-wrapper-cross-worktree-cache.md` |
| 2026-04-30 — Athena + DuckDB E2E Test Against Prebuilt Server Binary | `query-service-sql-engine-backends.md` |
| 2026-04-30 — DynamoDB + Redis Backend E2E Test Against Prebuilt Server Binary | `pluggable-service-backends-and-redis-storage.md`, `dynamodb-partiql-integration.md` |
| 2026-04-30 — DynamoDB + Redis Backend Extended E2E Sweep | `pluggable-service-backends-and-redis-storage.md`, `dynamodb-partiql-integration.md` |
| 2026-04-30 — SQS + Redis Backend E2E Test Against Prebuilt Server Binary | `pluggable-service-backends-and-redis-storage.md` |
| 2026-04-30 — SES SendEmail Examples and State/View Disparity Sweep | `stateful-service-and-blob-store.md` |
| 2026-04-30 — DynamoDB Sort-Key Filter Fix | `dynamodb-partiql-integration.md`, `pluggable-service-backends-and-redis-storage.md` |
| 2026-04-30 — Terraform E2E CI Triage — 13 Failing Tests + write-e2e-tests Skill Hardening | `terraform-e2e-harness-and-fix-coverage.md`, `quality-gate-workflow-and-recurring-findings.md` |
| 2026-04-30 — SQS + Redis Backend Bug Sweep — Five Fixes | `pluggable-service-backends-and-redis-storage.md` |
| 2026-04-30 — /tackle-todos sweep (codeguru wire migrations + sccache-wrapper d-file fix + provider-v6 enum-casing audit) | `quality-gate-workflow-and-recurring-findings.md`, `smithy-codegen-and-wire-serialization.md`, `sccache-wrapper-cross-worktree-cache.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-05-01 — E2E Verification of DynamoDB-on-Redis and SQS-on-Redis Fix Sweeps | `pluggable-service-backends-and-redis-storage.md`, `dynamodb-partiql-integration.md` |
| 2026-05-01 — /tackle-todos sweep continued (CBOR-XML adapter audit + scoreboard ready-prune + DuckDB end-to-end test) | `query-service-sql-engine-backends.md`, `sccache-wrapper-cross-worktree-cache.md`, `quality-gate-workflow-and-recurring-findings.md` |
| 2026-05-01 — /tackle-todos sweep continued (appintegrations audit + sccache-wrapper GC dry-run hook) | `quality-gate-workflow-and-recurring-findings.md`, `sccache-wrapper-cross-worktree-cache.md` |
| 2026-05-01 — /tackle-todos sweep continued (triage prompt + Models rate-limit backoff) | `github-issue-triage-and-automation.md`, `quality-gate-workflow-and-recurring-findings.md` |
| 2026-05-01 — /tackle-todos sweep continued (8 skill-prompt hardening items) | `quality-gate-workflow-and-recurring-findings.md` |
| 2026-05-01 — /tackle-todos sweep continued (workspace check + DMS test port + dedup) | `quality-gate-workflow-and-recurring-findings.md`, `workspace-readmes-and-service-examples.md` |
| 2026-05-01 — /tackle-todos sweep continued (crate-display-info-derive: protocol derived from Smithy) | `workspace-readmes-and-service-examples.md` |
| 2026-05-01 — deep-sleep service-note extraction workflow | `journal-ltm-maintenance-workflows.md` |
| 2026-05-01 — /tackle-todos sweep continued (triage guardrail-flag audit trail) | `github-issue-triage-and-automation.md` |
| 2026-05-01 — deep-sleep synthesis and service-note extraction | `journal-ltm-maintenance-workflows.md`, `cross-service-integration-and-engine-boundaries-synthesis.md`, `INDEX.md` |
| 2026-05-01 — /tackle-todos sweep continued (stale-item audit + athena failure detail + server rpath docs) | `query-service-sql-engine-backends.md`, `quality-gate-workflow-and-recurring-findings.md` |
| 2026-05-01 — deep-sleep service-note mode refresh | `journal-ltm-maintenance-workflows.md` |
| 2026-05-01 — EC2 remaining-operations push: 484 -> 752 / 756 SDK ops | `ec2-operation-expansion-and-invariants.md`, `ec2-crate-split-and-feature-gating.md`, `terraform-resource-converters.md` |
| 2026-05-01 — TODO.md active-backlog cleanup | `quality-gate-workflow-and-recurring-findings.md`, `journal-ltm-maintenance-workflows.md` |
| 2026-05-01 — v1/v2 service-pair state coherence audit | `cross-service-state-coherence.md` |
| 2026-05-01 — control-plane / data-plane state coherence audit | `cross-service-state-coherence.md` |
| 2026-05-01 — Quality Gate: winterbaume-ec2 | `quality-gate-workflow-and-recurring-findings.md`, `ec2-operation-expansion-and-invariants.md` |
| 2026-05-02 — Scenario test plan: winterbaume-ec2 | `aws-doc-test-plan-catalog.md`, `ec2-operation-expansion-and-invariants.md` |
| 2026-05-02 — Service dossier scenario enhancement | `aws-doc-test-plan-catalog.md`, `workspace-readmes-and-service-examples.md` |
| 2026-05-02 — Orchestration: invariant inventory becomes a first-class artefact | `aws-doc-test-plan-catalog.md`, `quality-gate-workflow-and-recurring-findings.md`, `journal-ltm-maintenance-workflows.md`, `ec2-operation-expansion-and-invariants.md` |
| 2026-05-02 — Audit rollout: dry-run findings + 13th EC2 bug fixed | `ec2-operation-expansion-and-invariants.md`, `quality-gate-workflow-and-recurring-findings.md` |
| 2026-05-02 — EC2 e2e terraform CI failures: 7 of 8 fixed | `ec2-operation-expansion-and-invariants.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-05-02 — Service dossier skill | `journal-ltm-maintenance-workflows.md` |
| 2026-05-02 — EC2 capacity_block_reservation: confirmed upstream AutoFlex bug | `ec2-operation-expansion-and-invariants.md`, `terraform-e2e-harness-and-fix-coverage.md` |
| 2026-05-02 — awsJson services skip generated request deserialisers | `smithy-codegen-and-wire-serialization.md` |
| 2026-05-02 — URL query-string parser duplicated across 59 crates | `new-service-implementation-patterns.md`, `TODO.md` |
| 2026-05-02 — Refining the codegen-deserialiser gap: restJson1 and migration debt | `smithy-codegen-and-wire-serialization.md`, `TODO.md` |
| 2026-05-02 — EC2 e2e CI rescue: complete work summary | `ec2-operation-expansion-and-invariants.md`, `terraform-e2e-harness-and-fix-coverage.md`, `smithy-codegen-and-wire-serialization.md` |
| 2026-05-02 — Stub crate: winterbaume-s3files (restJson1, model 2025-05-05) | `new-service-implementation-patterns.md`, `core-service-expansion-and-coverage.md`, `TODO.md` |
| 2026-05-02 — Network-aware service dossier and README transcription audit | `cross-service-state-coherence.md`, `workspace-readmes-and-service-examples.md` |
| 2026-05-02 — winterbaume-s3files: full operation coverage (21/21) | `new-service-implementation-patterns.md`, `core-service-expansion-and-coverage.md`, `quality-gate-workflow-and-recurring-findings.md`, `TODO.md` |
| 2026-05-02 — Deep Sleep Consolidation Record | `journal-ltm-maintenance-workflows.md`, `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-05-02 — Distill Memories Record | `journal-ltm-maintenance-workflows.md`, `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-05-02 — Core Documentation Rewrite Record | `journal-ltm-maintenance-workflows.md`, `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-05-02 — Quality Gate Rewrite Record | `journal-ltm-maintenance-workflows.md`, `quality-gate-workflow-and-recurring-findings.md` |
| 2026-05-02 — Smithy Codegen JSON Request Deserialisers + SES Adoption | `smithy-codegen-and-wire-serialization.md`, `service-implementation-and-validation-synthesis.md`, `TODO.md` |
| 2026-05-02 — awsQuery + awsJson Deserialiser Adoption Sweep (continuation) | `smithy-codegen-and-wire-serialization.md`, `service-implementation-and-validation-synthesis.md`, `TODO.md` |
| 2026-05-02 — Mass Wire/Model Regeneration Sweep | `smithy-codegen-and-wire-serialization.md`, `TODO.md` |
| 2026-05-03 — DynamoDB Data-Plane Migration | `smithy-codegen-and-wire-serialization.md`, `TODO.md` |
| 2026-05-03 — DynamoDB Expression-Layer Final Cleanup | `smithy-codegen-and-wire-serialization.md`, `TODO.md` |
| 2026-05-05 — Wire Deserialiser Adoption: Cross-Workstream Status | `smithy-codegen-and-wire-serialization.md`, `parallel-agent-build-and-worktree-practices.md`, `service-implementation-and-validation-synthesis.md`, `TODO.md` |
| 2026-05-02 to 2026-05-05 — Wire Deserialiser Adoption Sweep: Consolidated Findings | `smithy-codegen-and-wire-serialization.md`, `parallel-agent-build-and-worktree-practices.md`, `service-implementation-and-validation-synthesis.md`, `TODO.md` |
| 2026-05-08 — Pre-Launch Publish-Readiness Hardening | `crate-publishing-and-release.md`, `repo-maintenance-and-agent-workflows-synthesis.md`, `TODO.md` |
| 2026-05-08 — release-batch CLI Refinements | `crate-publishing-and-release.md`, `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-05-09 — release-batch retry pruning after partial 429 | `crate-publishing-and-release.md`, `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-05-09 — release-batch — recover from "is already published" cargo errors | `crate-publishing-and-release.md`, `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-05-09 — winterbaume-bug skill — make it self-contained | `winterbaume-skill-maintenance.md`, `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-05-09 — winterbaume-bug skill — follow-ups from self-review | `winterbaume-skill-maintenance.md`, `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-05-10 — mass-publish post-mortem — dropped tags after partial 429s, umbrella over the 500-dep limit | `crate-publishing-and-release.md`, `repo-maintenance-and-agent-workflows-synthesis.md` |
| 2026-05-11 — docs/ refresh after public release of all crates | `workspace-readmes-and-service-examples.md`, `smithy-codegen-and-wire-serialization.md`, `repo-maintenance-and-agent-workflows-synthesis.md` |

### Synthesis Relationships

| Synthesis Document | Consolidated Source LTM Documents |
|--------------------|-----------------------------------|
| `service-implementation-and-validation-synthesis.md` | `core-service-expansion-and-coverage.md`, `smithy-codegen-and-wire-serialization.md`, `moto-parity-testing-and-behavioral-gaps.md`, `aws-doc-test-plan-catalog.md`, `terraform-e2e-harness-and-fix-coverage.md`, `terraform-resource-converters.md`, `state-error-shaping-and-handler-boundaries.md`, `stub-handler-audit-and-promotion.md`, `quality-gate-workflow-and-recurring-findings.md`, `new-service-implementation-patterns.md`, `ec2-operation-expansion-and-invariants.md` |
| `runtime-state-and-service-infrastructure-synthesis.md` | `service-implementation-and-validation-synthesis.md`, `stateful-service-and-blob-store.md`, `terraform-resource-converters.md` |
| `pluggable-backends-and-query-execution-synthesis.md` | `pluggable-service-backends-and-redis-storage.md`, `query-service-sql-engine-backends.md`, `dynamodb-partiql-integration.md` |
| `repo-maintenance-and-agent-workflows-synthesis.md` | `parallel-agent-build-and-worktree-practices.md`, `sccache-wrapper-cross-worktree-cache.md`, `workspace-readmes-and-service-examples.md`, `crate-publishing-and-release.md`, `winterbaume-skill-maintenance.md`, `github-issue-triage-and-automation.md`, `repo-security-and-supply-chain.md`, `journal-ltm-maintenance-workflows.md` |
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

Open follow-up work extracted during consolidation lives in `.agents/docs/TODO.md`. See `.agents/docs/LTM/INDEX.md` for the full index.

---

## Deep Sleep Consolidation Record (2026-05-11)

No new synthesis document was created. The existing `repo-maintenance-and-agent-workflows-synthesis.md` synthesis was refreshed to cover `winterbaume-skill-maintenance.md` as the repo/workflow source for self-contained skill packaging, embedded issue-template contracts, service-slug snapshots, drift CI, and Markdown template hygiene.

| Synthesis Document | Source LTM Documents |
|---|---|
| `repo-maintenance-and-agent-workflows-synthesis.md` | `winterbaume-skill-maintenance.md` added to the existing synthesis set. |

No service documents were updated; the source contains repo/workflow guidance rather than AWS service-specific parity facts. `winterbaume-skill-maintenance.md` remains a standalone source drill-down.

---

## 2026-05-11 — Skip CI pipeline for docs-only pushes to `main`

### Context

`.github/workflows/ci.yml` was unconditionally triggered on every `push` to `main`, kicking the full Rustfmt → Clippy → Tests → Examples → E2E ( Terraform ) chain even when the commit only touched documentation. The user asked to gate the pipeline with `dorny/paths-filter` so docs-only pushes do not consume CI minutes.

### Change

Added a new leading `changes` job that runs `dorny/paths-filter@fbd0ab8f3e69293af611ebaee6363fc25e6d187d` ( v4.0.1, pinned to the tag's commit SHA per repo convention ) and exposes a single `code` output. The filter treats these paths as build-relevant:

- `crates/**`, `src/**`, `tools/**`, `examples/**` ( all Rust sources the workflow exercises )
- `vendor/**` ( vendored Smithy models that drive codegen )
- `Cargo.toml`, `Cargo.lock`, `rustfmt.toml`, `dist-workspace.toml`, `.gitmodules`
- `.github/workflows/ci.yml`, `.github/actions/**` ( so changes to CI re-trigger CI )

`docs/**` ( the VitePress site, handled separately by `deploy-docs.yml` ), `.agents/docs/**`, `skills/**`, `mise.toml`, `*.md`, and the other top-level docs are excluded by omission.

`fmt` now depends on `changes` and carries the guard:

```yaml
if: |
  github.event_name != 'push' ||
  github.ref != 'refs/heads/main' ||
  needs.changes.outputs.code == 'true'
```

The condition only skips when **all three** of `push` + `refs/heads/main` + no code-relevant diff hold. `workflow_dispatch` and `workflow_call` ( the latter is how `release.yml` reuses ci.yml on tag pushes — the original `github.event_name` is still `push` but `github.ref` is `refs/tags/...`, which short-circuits the second clause ) always run the full pipeline.

`clippy`, `test`, `examples`, and `e2e` were not modified individually. They cascade-skip via `needs:` because the default `if` for a job is `success()`, which fails when an upstream `needs` job has status `skipped` ( rather than `success` ). Net effect on a docs-only push: only the ≈10 s `changes` job runs.

### Finding: dorny/paths-filter latest stable is v4.0.1

`gh api repos/dorny/paths-filter/releases/latest` reports tag `v4.0.1` ( commit `fbd0ab8f3e69293af611ebaee6363fc25e6d187d` ). The previous major `v3` resolves to a different SHA ( `6852f92...` ) — worth noting if any other workflow later pins paths-filter, the repo convention is full 40-char SHA with a trailing `# vX.Y.Z` comment.

### Finding: cascade-skip via `needs:` is sufficient

There is no need to repeat the path-filter `if:` on every downstream job. GitHub Actions' implicit `if: success()` propagates `skipped` through the `needs:` chain ( `fmt` skipped → `clippy` skipped → `test` / `examples` skipped → `e2e` skipped ). Adding the guard only to the root job keeps the workflow concise.

---

## 2026-05-11 — CI: skip individual jobs when prior pass markers exist

### Context

Following the docs-only paths-filter skip, the next optimisation: when CI is forced to run ( real code change or workflow_dispatch ) but the source tree is byte-identical to a previously-passed run, individual jobs should short-circuit rather than redo deterministic work. The user accepted slightly weaker determinism for `e2e` ( Terraform + external network ) in exchange for the simplicity of uniform treatment across all jobs.

### Change

Added a `fingerprint` job after `changes`. It hashes the code-relevant tree with `hashFiles()` over `crates/**`, `src/**`, `tools/**`, `examples/**`, `Cargo.toml`, `Cargo.lock`, `rustfmt.toml`, `dist-workspace.toml`, `.gitmodules`, `.github/workflows/ci.yml`, `.github/actions/**` ( excluding `vendor/**` because the submodule is not checked out in CI, so it would contribute nothing ), then probes five `ci-pass-{fmt,clippy,test,examples,e2e}-Linux-<hash>` cache keys with `actions/cache/restore@v5.0.5` using `lookup-only: true`. The job exposes `<job>-hit` outputs ( `'true'` or empty ) plus the computed `hash`.

Each downstream job ( `fmt`, `clippy`, `test`, `examples`, `e2e` ) gained:

1. A leading `if:` that combines `!failure() && !cancelled()` ( for non-root jobs ), the per-job hit check, and the paths-filter clause. On a hit the job is fully skipped at the job level — no per-step `if:` plumbing.
2. Two trailing steps: `mkdir .ci-marker && echo <hash> > .ci-marker/marker`, followed by `actions/cache/save@v5.0.5` under the matching `ci-pass-<job>-<os>-<hash>` key. The default `if: success()` on the save step prevents poisoning the cache on a failed `cargo` step.

The `e2e` job's `upload-artifact@v7.0.1` for `tf-logs` keeps `if: always()` — when e2e is fully skipped via job-level `if`, the upload step doesn't run at all ( jobs evaluate `if` before any steps ), so there's no risk of attempting an upload from a non-existent directory.

### Finding: `!failure() && !cancelled()` lets `skipped` upstream flow through

The default job-level `if: success()` requires every `needs:` job to finish with `result == 'success'`. A `skipped` upstream short-circuits the chain. Replacing the default with `!failure() && !cancelled()` ( `skipped` is neither `failure` nor `cancelled` ) lets cache-hit-skipped jobs propagate through `fmt → clippy → test → e2e` while preserving the original fail-fast semantics on a real `failure()`. This avoided having to wrap every body step with a per-step `if:` ( the verbose pattern in the first draft ).

### Finding: `github.ref` guard for `workflow_call` was redundant

The first draft included `github.ref != 'refs/heads/main'` as a safety net for `release.yml` invoking ci.yml on a tag push ( `event_name == 'push'`, ref is a tag ). On reflection, the cache-hit short-circuit already gives release runs the right outcome — same source → cache hit → fast skip — and dorny/paths-filter's tag-push behaviour ( `event.before == 0000…` ) is unlikely to ever return `'false'` for a fresh ref. Removed the clause for simplicity.

### Drawbacks accepted

- Cache markers don't capture toolchain version pinning ( `actions-rust-lang/setup-rust-toolchain@<sha>` is in the hash, but the resolved rustc version is not ) or runner image drift. A GitHub-side rustc bump on the same `ubuntu-latest` won't invalidate markers; first failure manifests as a stale-marker re-run.
- A flaky pass gets frozen — same source means future runs skip rather than retry. Acceptable for now per user direction; `e2e` is the obvious candidate to revisit.
- Branch protection on `main` is not configured to require these as named checks, so `skipped` conclusions do not block anything. If that changes, downstream consumers of CI status will need to treat `skipped` as `success` ( or the markers will need to write a synthetic success status instead ).

## 2026-05-11 — Release workflow: fix musl and aarch64-windows build failures

### Context

The `winterbaume-server-v0.1.0` tag push ( run [25645291318](https://github.com/moriyoshi/winterbaume/actions/runs/25645291318) ) green-lit `ci` and `plan`, then `build-local-artifacts` failed on two targets in the dist matrix while the other five succeeded:

- `x86_64-unknown-linux-musl` — `libduckdb-sys v1.10501.0` build script aborted with `cc-rs: failed to find tool "x86_64-linux-musl-g++"`.
- `aarch64-pc-windows-msvc` — `cargo` aborted at metadata resolution with `rustc 1.89.0 is not supported by the following packages: aws-config@1.8.14 requires rustc 1.91 …` ( and ~20 sibling `aws-*` crates ).

Both targets are gated behind the `backend-sqlengine-duckdb-bundled` feature set by `crates/winterbaume-server/Cargo.toml` ( `[package.metadata.dist]` ), which forces a static DuckDB build for the public binary.

### Findings

**Finding: cargo-dist's musl matrix only installs `musl-tools`, which lacks `g++`.**
The generated matrix entry for `x86_64-unknown-linux-musl` is `runner: ubuntu-22.04`, no container, `packages_install: "sudo apt-get update\nsudo apt-get install musl-tools"`. Ubuntu's `musl-tools` package ships `musl-gcc` / `musl-ar` only — there is no `musl-g++` apt package on 22.04. As long as no crate in the workspace needed a C++ cross-compile, this was fine; the `backend-sqlengine-duckdb-bundled` feature is the first dependency that does, and cc-rs's standard search path ( `x86_64-linux-musl-g++` → `x86_64-linux-musl-c++` → fallback ) finds nothing.

**Finding: the `messense/cargo-xwin` container is rebuilt on its own cadence and lags rustc stable.**
The aarch64-windows matrix entry runs inside `container.image: messense/cargo-xwin` ( no tag, resolves to whatever `:latest` happened to be at pull time ). The image is built from `rust-cross/cargo-xwin` and ships a fixed rustc — currently 1.89.0. The AWS SDK crate family ( `aws-config`, `aws-sigv4`, `aws-smithy-*`, `aws-sdk-{sso,ssooidc,sts}`, `aws-types`, … ) bumped MSRV to 1.91 / 1.91.1 in their latest publishes, so `cargo` rejects the lockfile during target metadata resolution before any compilation starts. The native `x86_64-pc-windows-msvc` target succeeds because it runs on `windows-2022` with rustup-managed stable, not the container.

**Finding: `dist plan` re-reads `dist-workspace.toml` per run; matrix overrides do not require regenerating `release.yml`.**
Investigated whether to switch the musl runner to `messense/rust-musl-cross:x86_64-musl` ( which preinstalls `x86_64-linux-musl-g++` ) via `[dist.github-custom-runners.x86_64-unknown-linux-musl]` in `dist-workspace.toml`. Decided against it: that image is RHEL-derived ( `dnf`-based ), so the matrix's apt-flavoured `packages_install` would also need overriding, and the image's pinned rustc would still need a `rustup update` step. Direct workflow edits with `if: ${{ contains(matrix.targets, …) }}` guards keep the change surface small and target-scoped.

**Finding: `release.yml` is already hand-edited away from `dist generate` output.**
The `Install dist` step uses an explicit SHA256 check against `CARGO_DIST_INSTALLER_SHA256`, which `dist generate` does not emit. Adding more bespoke steps does not increase drift in a meaningful way; the workflow has already opted into manual maintenance.

### Change

`.github/workflows/release.yml`, inside the `build-local-artifacts` job:

1. New step `Update Rust toolchain in cross-compile container` immediately after the existing `Install Rust non-interactively if not already installed`. Runs only when `matrix.container` is truthy ( so today, only on the aarch64-windows entry ). Body is `rustup update stable && rustup default stable`, lifting the in-container rustc past 1.91 so AWS SDK metadata resolution succeeds.

2. New step `Install x86_64-linux-musl C/C++ cross-compiler` after `Install dependencies`. Guarded by `if: ${{ contains(matrix.targets, 'x86_64-unknown-linux-musl') }}`. Downloads `https://musl.cc/x86_64-linux-musl-cross.tgz` ( pinned `MUSL_CROSS_SHA256 = c5d410d9f82a4f24c549fe5d24f988f85b2679b452413a9f7e5f7b956f2fe7ea`, 115 MB ), verifies via `sha256sum -c`, extracts to `/opt`, and prepends `/opt/x86_64-linux-musl-cross/bin` to `$GITHUB_PATH`. cc-rs then finds `x86_64-linux-musl-g++` via its standard target-prefix search without further env-var plumbing.

The SHA256 was computed locally on the same tarball ( `shasum -a 256` ) and pinned in the step's `env:` block; the comment in the workflow points future maintainers at musl.cc as the source of truth.

### Drawbacks accepted

- `musl.cc` is a third-party host ( Rich Felker, musl libc author ). The SHA pin neutralises tampering risk on the byte stream, but availability still depends on the host. If musl.cc goes offline, the musl target build fails until either the URL is mirrored or the SHA is bumped to a new release.
- `rustup update stable` inside the container makes the toolchain version implicit on the runner-day, not deterministic. A future rustc-stable regression would surface here without a Cargo.lock or `rust-toolchain.toml` change in the repo. Acceptable for now because the only alternative is pinning a specific rustc and chasing AWS SDK MSRV bumps every release.
- The two fixes do not address the Node 20 deprecation warnings emitted by `mozilla-actions/sccache-action`; that is upstream's problem and orthogonal to the release-build break.

### Verification

`release.yml` triggers on `pull_request:` as well as tag pushes, so the fix can be exercised by opening a PR rather than re-cutting the tag. Pending: open a PR ( pushing the workflow change to a branch ) and confirm both `build-local-artifacts (x86_64-unknown-linux-musl)` and `build-local-artifacts (aarch64-pc-windows-msvc)` go green.

## 2026-05-11 — Release workflow: drop musl and aarch64-windows targets after fix attempts surfaced fresh failures

### Context

The tag re-push from the previous entry ( commit `c9e07e73`, run [25647595589](https://github.com/moriyoshi/winterbaume/actions/runs/25647595589) ) exercised the two surgical fixes on the real CI matrix. Both targets failed again, with different root causes than the first attempt.

### Findings

**Finding: `musl.cc` is unreliable from GitHub-hosted runners ( exit code 28, curl operation timeout ).**
The `Install x86_64-linux-musl C/C++ cross-compiler` step ran `curl --proto '=https' --tlsv1.2 -LsSf https://musl.cc/x86_64-linux-musl-cross.tgz` and aborted after ~2 min 12 s with curl exit code 28 ( operation timeout ). This matches the GitHub community discussion the user cited ( https://github.com/orgs/community/discussions/27906#discussioncomment-3332440 ) where musl.cc routinely times out from Azure-hosted runners. The SHA256 pin neutralises content-tampering risk but does not help when bytes simply never arrive. Pre-staging the tarball in a release artefact or GitHub-side mirror was considered, but at that point the operational cost ( separate mirror job, signed manifest, rotation policy ) exceeds the value of distributing a musl artefact today.

**Finding: `rustup update stable` lifted the cargo-xwin container to rustc 1.95.0, which then exposed a `ring v0.17.14` / cargo-xwin / clang argument-flavour mismatch.**
With rustc 1.95.0 in the container, AWS SDK metadata resolution succeeded ( original MSRV failure cleared ). The build then advanced to `ring`'s C-source compile through cargo-xwin's clang wrapper and aborted with `clang: error: no such file or directory: '/imsvc'`, repeated for every `.c` file. cargo-xwin's pinned argument templates emit `/imsvc <include>` ( MSVC-style ) when prefixing SDK include paths, but the clang shipped in the container ( pulled in by the rustup update ) now expects `-imsvc <include>` ( clang-style ) and treats the slash-prefixed token as a positional filename. Fixing this needs either a newer cargo-xwin upstream or a downgraded clang — both are upstream-coupled, neither is in repo scope today.

**Finding: dropping a target via `dist-workspace.toml` is sufficient; `dist plan` rebuilds the matrix at job-start.**
No need to regenerate `release.yml`. Removing the target from the `targets = [...]` array in `[dist]` propagates through `dist host --steps=create` ( the `plan` job's command ) and produces a matrix without the dropped entries on the next push. Workflow steps gated by `contains(matrix.targets, '<dropped>')` become permanent no-ops and are safe to remove for clarity.

### Change

1. `dist-workspace.toml`: dropped `x86_64-unknown-linux-musl` and `aarch64-pc-windows-msvc` from the `targets` array. Remaining targets: `aarch64-apple-darwin`, `aarch64-unknown-linux-gnu`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `x86_64-pc-windows-msvc`.
2. `.github/workflows/release.yml`: removed the now-orphan `Install x86_64-linux-musl C/C++ cross-compiler` step and the `Update Rust toolchain in cross-compile container` step. The latter was only useful for the cargo-xwin container; with aarch64-windows dropped, no matrix entry sets `matrix.container`, so the gate `if: ${{ matrix.container }}` would never fire. The original `Install Rust non-interactively if not already installed` step is kept defensively in case a future container-based target reappears.

### Drawbacks accepted

- No fully static Linux binary in the release. The `x86_64-unknown-linux-gnu` build links against the runner's glibc ( ubuntu-22.04, glibc 2.35 ); users on older distros will need to build from source or wait until a working musl path returns. The `backend-sqlengine-duckdb-bundled` feature still applies — the gnu binary statically links DuckDB, so the only dynamic dependency of consequence is libc itself.
- No Windows-on-ARM binary in the release. Anecdotally, ARM Windows users on the Surface / Snapdragon lineup can fall back to x86-64 emulation, which the `x86_64-pc-windows-msvc` artefact covers.
- These drops are reactive ( "ship the targets that work today" ) rather than a designed support matrix. Revisiting belongs in a separate cycle, ideally once `cargo-dist` upstream gains better musl/cargo-xwin support or once we adopt cross-rs's docker-based toolchain layer.

### Verification

Pending: the next tag push will exercise the slimmed-down matrix. Five targets now in `build-local-artifacts` ( the two dropped + five remaining = seven planned originally, five expected after this commit ).

