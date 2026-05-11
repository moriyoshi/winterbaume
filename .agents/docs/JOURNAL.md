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

## 2026-05-10: Terraform converter codegen migration

### Context

Hand-rolled Terraform converters at `crates/winterbaume-terraform/src/converters/<service>.rs`
had grown to ~70 kLOC across 145 services, with roughly 70% of every converter being
mechanical serde plumbing — `optional_str`/`require_str`/`optional_bool`/`optional_i64`
field unpacking, type coercion, ARN/URL `format!` templates, default fallbacks, view
struct construction, `serde_json::json!` extract emission. The shape was repeated across
services with only the field names changing. The user asked for a migration plan to
replace this hand-rolled work with a code-generator analogous to `tools/smithy-codegen/`.

### Plan and execution

The full plan is at `~/.claude/plans/currently-terraform-converters-consist-cuddly-lovelace.md`.
The approved approach is **thin projection via serde**: a per-service codegen tool emits a
`*TfModel` Rust struct with `#[derive(Serialize, Deserialize)]` and per-field
`rename` / `default` / `skip_serializing_if` annotations driven by a hand-curated TOML spec.
Decode and encode at converter call sites become `serde_json::from_value` /
`serde_json::to_value`. Hand-written converters keep ownership of the trait scaffolding,
the inject/extract orchestration, ARN/URL templates that exceed `{region}/{account}/{name}`
substitution, multi-resource composition, and any nested-block reshaping. The plan named
this the "model_only" mode, where the codegen emits only the struct definition because
the AWS-side `*StateView` shape doesn't fit a 1:1 serde projection ( e.g., SNS's
`HashMap<String, String>` attribute bag, AppFlow's singleton-array → AWS REST JSON
discriminated objects ). For the simplest services, the codegen can also emit the full
`into_state_view(self, ctx, region) -> View` projection plus `From<&View> for *TfModel`,
making the converter body shrink to ~30 lines of trait scaffolding + a single
`serde_json::from_value` call.

Execution proceeded through the planned phases:

- **Phase 0 — infrastructure.** `tools/tf-converter-codegen/` crate with subcommands
  `gen`, `gen-all`, `check`, `list`. Mirrors `tools/smithy-codegen/` idioms exactly:
  `clap`, `anyhow`, string concatenation + `rustfmt`, no `quote!`/`syn`, no templating.
  Spec format defined at `tools/tf-converter-codegen/src/spec.rs` with the type
  vocabulary `string`, `string?`, `u32`, `i64`, `bool`, `tags`. Golden test
  ( `tools/tf-converter-codegen/tests/check.rs` ) re-runs `gen` for every spec and
  diffs against the committed `generated/<svc>.rs`.

- **Phase 1 — SQS+SNS pilot.** SQS migrated to full thin projection ( 16 fields,
  ARN/URL templates with attr-override chains, `id`+`tags_all` as
  `[[resource.computed_extract]]` entries ). SNS migrated as `mode = "model_only"`
  because both `aws_sns_topic` and `aws_sns_topic_subscription` store most TF inputs
  in `HashMap<String, String>` attribute bags keyed by AWS PascalCase names; the bag
  packing is hand-written.

- **Phase 2 — AppFlow shape-transform proving.** AppFlow has the documented
  JSON-shape mismatch ( TF singleton-array blocks → AWS REST JSON discriminated
  objects ). The flat fields ( `name`, `arn`, `description`, `kms_arn`, `tags` ) go
  through the spec; the ~300 LOC of `tf_to_aws_*` reshape helpers and 5 reshape unit
  tests stay verbatim in the converter. Validates that the hand-written escape hatch
  is genuinely escape-hatchable.

- **Phase 3 — ingest tooling + sweep.** Implemented `tf-converter-codegen ingest <service>`
  that parses an existing converter source, identifies the resource type ( from
  `fn resource_type(&self) -> &str`-returning string literals ), the service crate
  ( from `use winterbaume_<crate>::views::...` ), the View struct names, and every
  field read via `optional_str` / `require_str` / `optional_bool` / `optional_i64` /
  `extract_tags` / `attrs.get`. Emits a best-effort TOML spec with `mode = "model_only"`.
  Required to handle: invalid Rust identifiers ( filtered ), Rust reserved keywords
  ( escaped via `r#`-prefix in codegen.rs ), multi-line `use` statements ( handled by
  index-walking through the source rather than line-by-line iteration ), and
  multi-resource files ( flagged with a TODO header for the human reviewer ). Bootstrap
  generated 120 specs in a single shell loop.

  The remaining 143 converter migrations were dispatched to general-purpose sub-agents
  in 5-service batches ( 22 batches A through Z+AA+BB plus a dedicated EC2 agent ).
  Each agent read the existing converter, the auto-ingested spec, and the views file;
  hand-tuned the spec ( fix model/view names, add field renames, mark `required`,
  drop discarded fields and Vec<T>/nested-block fields whose types the spec format
  doesn't express ); rewrote the converter to deserialize via the typed TfModel and
  apply ARN templates / constants / nested-block parsing in hand-written code; reported
  back. The parent agent regenerated `generated/<svc>.rs` for each batch via
  `gen-all`, ran build + tests + clippy, and dispatched the next batch on success.

### Findings

**The thin-projection / model-only split scales.** Across 145 services the same
spec format covers everything from SQS ( 16 simple flat fields ) to EC2 ( 39
resources, each with 5-15 typed scalar fields plus a sea of nested blocks ). The
break point is the spec's intentional type vocabulary boundary: anything outside
`string`, `string?`, `u32`, `i64`, `bool`, `tags` ( notably `f64`, `Vec<String>`,
`HashMap<String, String>` for non-tag uses, `Option<i64>`/`Option<bool>`, and any
nested-block list-of-objects ) drops out of the spec and into hand-written code
in the converter. This is a feature, not a bug: it forces non-trivial mappings
into a place reviewers can see.

**Auto-ingest is necessary but not sufficient.** The ingest tool catches ~70% of
the work — every flat field is enumerated, ARN templates are captured as a
review comment, the resource type and crate name are detected. The remaining 30%
is human-only: choosing the right View struct when the converter imports several
( the auto-pick of "first View ending in 'View'" was wrong for ~30% of services,
including ssm/PartialHistoryView vs ParameterView, kinesis/ShardView vs
StreamView, ecr/EncryptionConfigView vs RepositoryView ); applying field renames
where TF and AWS disagree ( max_message_size vs maximum_message_size ); marking
required-on-input fields ( the ingest can't tell `require_str` vs `optional_str`
when the converter wraps a fallback ); splitting union specs into per-resource
blocks for multi-resource files; assigning the right type to numeric strings
( the ingest defaults to `string?` for `optional_str` callsites, even when the
field is actually `i64`/`u32` ).

**Sub-agents work for this kind of migration.** The bottleneck was wall-clock
build time, not edit time. With 5 services per batch and 2 batches dispatched
concurrently, each round delivered ~10 services in parallel, then required ~3-5
minutes to regen + build + test sequentially. Total throughput: ~10 services per
~10-minute round, so the 120-service sweep took ~12 rounds of dispatched batches
plus the EC2 dedicated round. Sub-agents occasionally hallucinated "already
migrated" verdicts when the file structure was unfamiliar to them — defended
against by checking `git diff` and the build output before trusting reports.

**The do_extract guarantee held everywhere.** Every sub-agent was told to keep
the `serde_json::json!({ ... })` extract literal byte-equivalent to the original.
Across 145 services, the existing 293-test integration suite at
`crates/winterbaume-terraform/tests/integration_test.rs` passed unchanged after
every batch. This is the load-bearing safety property: as long as extract output
is bit-identical, downstream consumers ( terraform-state JSON readers ) cannot
detect the migration. Inject behaviour can drift slightly in deserialise-strictness
( e.g., null for a typed `bool` now errors instead of falling back ) — accepted
because real `terraform show -json` output respects the provider schema's declared
types.

**Two services don't fit thin projection at all.** `simpledbv2` and `inspector2`
both store their inputs as `HashSet<String>` ( SimpleDB domains; Inspector2
resource types ). For SimpleDB the spec is a single required `name` field and
the converter constructs the HashSet manually. For Inspector2 the spec emits an
empty marker `EnablerTfModel` ( both inputs are `Vec<String>`, outside the spec
vocabulary ) and the converter reads them straight from `instance.attributes`.
Both still fit the migration pattern in the sense that they have a spec file and
( for SimpleDB ) call into the generated module; Inspector2 is the only converter
in the codebase that doesn't import its `generated::*` sibling, and the spec
file makes that explicit.

### Repository deltas

- New crate: `tools/tf-converter-codegen/` ( ~600 LOC: `main.rs`, `spec.rs`,
  `codegen.rs`, `ingest.rs` ).
- New: `crates/winterbaume-terraform/specs/*.toml` × 145.
- New: `crates/winterbaume-terraform/src/generated/<service>.rs` × 145
  ( ~6,800 LOC of pure-mechanical generated code ).
- New: `crates/winterbaume-terraform/src/generated/mod.rs` declaring all 145 modules.
- Modified: `crates/winterbaume-terraform/src/converters/*.rs` — every service
  except `simpledbv2`/`inspector2` ( the marker case ) now opens with
  `use crate::generated::<svc> as <svc>_gen;` and the inject body starts with
  `serde_json::from_value::<<svc>_gen::*TfModel>(...)`. ~50,000 LOC total
  across all converters, down from ~70,000 in the original hand-rolled state.
- New: `crate::util::classify_deserialize_error` for converting `serde_json::Error`
  into the existing `ConversionError::MissingAttribute` / `InvalidAttribute`
  variants. Used by every spec-driven converter.
- Workspace `Cargo.toml`: registers `tools/tf-converter-codegen` in `members` and
  `default-members`.
- Codegen handles Rust reserved keywords as field names by emitting `r#<name>`
  ( e.g., `r#type`, `r#match` ); HashMap import is conditional on whether any
  spec field uses `tags`.

### Verification

All gates green at end of migration:

- `cargo build -p winterbaume-terraform`: clean.
- `cargo test -p winterbaume-terraform --no-fail-fast`: 293 integration tests +
  6 AppFlow reshape unit tests pass; 22 E2E tests ignored ( those require
  Terraform CLI ).
- `cargo clippy -p winterbaume-terraform --all-targets --all-features -- -D warnings`:
  clean.
- `cargo clippy -p tf-converter-codegen --all-targets --all-features -- -D warnings`:
  clean.
- `cargo test -p tf-converter-codegen`: golden test passes ( re-runs `gen` for
  every spec and diffs against committed output — guards against stale generated
  files in CI ).
- `cargo fmt --check -p winterbaume-terraform`, `cargo fmt --check -p tf-converter-codegen`:
  clean.

### Out of scope going forward

Per the plan, several capabilities are intentionally not covered by the codegen
and stay hand-written in converters:

- ARN templates beyond simple `{region}/{account}/{name}` substitution.
- Custom default values not expressible as a literal ( e.g., environment-derived
  defaults ).
- Validator / mutual-exclusivity logic ( `exactly_one_of`, `conflicts_with` ).
- Nested-block reshaping ( handled exclusively via hand-written code that runs
  between `decode` and view construction; AppFlow is the canonical example ).
- Cross-service dependency declarations ( `depends_on_types` ); declared in
  hand-written code.

The auto-ingested specs are a starting point and were hand-tuned during the
sweep. Future converter changes should:
1. Hand-edit the spec when adding/changing TF input fields.
2. Run `cargo run -p tf-converter-codegen -- gen <service>` to regenerate the
   model.
3. Update the converter body to consume new fields off the typed model.
4. Run the per-crate lint gate ( `clippy --all-targets --all-features
   -- -D warnings` and `fmt --check` ) before committing.

## 2026-05-10 (cont.): Extract generated models into `winterbaume-tfstate-resource-models`

### Context

The migration above landed all 145 auto-generated `*TfModel` files at
`crates/winterbaume-terraform/src/generated/<svc>.rs` (~6,800 LOC and growing
as services and resources expand). The user observed that this generated tree
will keep growing fast and asked to isolate it into its own crate so:

- Hand-written changes to `winterbaume-terraform` don't recompile the entire
  generated tree.
- The generated code's footprint sits at a separate, easily-auditable crate
  boundary.
- Future regenerations don't churn diffs in the main converter crate.

The plan is at `~/.claude/plans/currently-terraform-converters-consist-cuddly-lovelace.md`.

### Findings

**SQS was the only blocker.** It was the lone service still in thin-projection
mode (the others all use `mode = "model_only"`), and its generated file
referenced `crate::converter::ConversionContext` (in `winterbaume-terraform`)
plus `winterbaume_sqs::views::QueueStateView` (in the SQS service crate). The
first reference would have created a cyclic dependency the moment the
generated code moved to a separate crate: the new crate would need
`ConversionContext`, which lived in `winterbaume-terraform`, which would need
to depend on the new crate.

The fix was to convert SQS to `mode = "model_only"` like the other 144
services. The `into_state_view` and `From<&QueueStateView>` impls move out of
the generated file and into the hand-written `converters/sqs.rs` (~30 extra
lines of explicit field assignment, mirroring the swf/amp/ivs pattern). The
generated SQS file then needs only `serde`, so the new crate becomes a leaf
with `serde` as its only `[dependencies]` entry. This is a one-time cost; no
future service needs to be in thin-projection mode.

**The re-export trick lets converter source stay unchanged.** Every existing
converter file already wrote `use crate::generated::<svc> as <svc>_gen;`, so a
single line in `winterbaume-terraform/src/lib.rs`:

```rust
pub use winterbaume_tfstate_resource_models as generated;
```

makes that path continue to resolve through the alias. None of the 145 hand-
written converter files ( ~50,000 LOC ) needed to change. Git tracked the
generated-file moves as `R100` renames, so the diff is essentially zero-byte
for those files — only the path changed.

**The codegen tool's default path was the only other knob to turn.** In
`tools/tf-converter-codegen/src/main.rs`, the `--output-dir` default went from
`crates/winterbaume-terraform/src/generated` to
`crates/winterbaume-tfstate-resource-models/src`. The `gen`, `gen-all`, and
`check` subcommands all share that default, so a single one-line change kept
the whole CLI consistent. The golden test at
`tools/tf-converter-codegen/tests/check.rs` runs the binary with the workspace
root as cwd and uses workspace-relative defaults, so it picked up the new
path without any test changes.

**The crate name `winterbaume-tfstate-resource-models` was chosen by the
user.** The implication is that it's a sibling of `winterbaume-tfstate`
( the existing Terraform state-file parser crate ) rather than a sibling of
`winterbaume-terraform` ( the converter crate ). The naming suggests the
generated models are conceptually closer to the state representation than to
the converter logic, which is true: each `*TfModel` is a serde-decoded view of
one TF state resource attribute set. The Rust module identifier
( `winterbaume_tfstate_resource_models`, snake-cased automatically ) is what
the re-export aliases.

### Architecture after split

```
crates/
  winterbaume-tfstate-resource-models/        ← NEW
    Cargo.toml                          (deps: serde with derive)
    src/
      lib.rs                            (mod declarations × 145)
      <service>.rs × 145                (renamed from old generated/)
  winterbaume-terraform/
    Cargo.toml                          (adds winterbaume-tfstate-resource-models dep)
    src/
      lib.rs                            (drops `pub mod generated;`,
                                          adds `pub use winterbaume_tfstate_resource_models as generated;`)
      converters/<service>.rs × 145     (UNCHANGED)
      generated/                        ← REMOVED (moved out)
```

### Repository deltas

- New crate: `crates/winterbaume-tfstate-resource-models/` ( `Cargo.toml` +
  `src/lib.rs` + 145 module files ).
- `Cargo.toml` ( workspace ): added the crate to `members`, `default-members`,
  and `workspace.dependencies`.
- `crates/winterbaume-terraform/Cargo.toml`: added the new crate as a
  dependency.
- `crates/winterbaume-terraform/src/lib.rs`: removed `pub mod generated;` line,
  added `pub use winterbaume_tfstate_resource_models as generated;`.
- `crates/winterbaume-terraform/src/generated/`: directory removed entirely.
- `crates/winterbaume-terraform/specs/sqs.toml`: added
  `[resource.modes] mode = "model_only"`, removed `[resource.computed.arn]`,
  `[resource.computed.url]`, `[[resource.computed_extract]]` × 2,
  `[[resource.view_extra]]` × 4. Added `arn` / `url` / `id` as plain optional
  string fields ( previously they were captured by the computed-template /
  attr-override-chain machinery in the generated code ).
- `crates/winterbaume-terraform/src/converters/sqs.rs`: rewrote `do_inject` to
  build `QueueStateView` explicitly with the ARN and URL templates inline,
  matching the hand-written pattern used by every other service.
- `tools/tf-converter-codegen/src/main.rs`: default `--output-dir` switched.
- 145 generated files renamed ( git rename detection caught all of them with
  `R100` similarity ).

### Verification

- `cargo run -p tf-converter-codegen -- gen-all`: writes 145 files into the
  new crate's `src/`.
- `cargo run -p tf-converter-codegen -- check`: passes ( zero stale files ).
- `./.agents/bin/cargo.sh build -p winterbaume-tfstate-resource-models`:
  clean ( ~22 s, depends only on `serde` ).
- `./.agents/bin/cargo.sh build -p winterbaume-terraform`: clean.
- `./.agents/bin/cargo.sh test -p winterbaume-terraform --no-fail-fast`:
  293 integration tests + 6 reshape tests pass; 22 ignored ( terraform-CLI
  E2E ).
- `./.agents/bin/cargo.sh clippy -p winterbaume-tfstate-resource-models
  --all-targets --all-features -- -D warnings`: clean.
- `./.agents/bin/cargo.sh clippy -p winterbaume-terraform --all-targets
  --all-features -- -D warnings`: clean.
- `./.agents/bin/cargo.sh clippy -p tf-converter-codegen --all-targets
  --all-features -- -D warnings`: clean.
- `./.agents/bin/cargo.sh test -p tf-converter-codegen`: golden test passes.

### Operational notes

- Future regenerations from spec edits land in
  `crates/winterbaume-tfstate-resource-models/src/<svc>.rs`. The codegen
  tool's default output path handles this automatically; no extra flags
  needed.
- The new crate's compile time should drop materially from the previous
  in-tree position because `winterbaume-tfstate-resource-models` only
  depends on `serde`. Any change to a hand-written `winterbaume-terraform`
  converter no longer recompiles the 145-module generated tree.
- The publication-readiness gate ( `RELEASE.md` + `verify-publish-ready`
  skill ) needs to be aware of the new crate. Adding it to the public
  workspace surface is a follow-up task ( not done in this commit ).

## 2026-05-11: Post-extraction wave — broaden Terraform resource coverage

### Context

With the spec-driven codegen and the `winterbaume-tfstate-resource-models`
crate in place, the next push was to widen the supported set of TF resource
types beyond the original 145 services' default scope. Each spec entry maps
one `aws_*` resource type to a `*TfModel`, but most specs only covered one
or two top-level types per service. The user asked to begin with a
visibility step ( per-service resource-type coverage report ) so the
remaining gap could be prioritised, then enhance high-impact services in
batches.

### Findings

**Per-service resource-type coverage report.** Built
`.agents/skills/api-coverage/scripts/generate_terraform_resource_coverage.py`,
which renders `.agents/docs/TERRAFORM_RESOURCE_COVERAGE.md`. It walks the
Terraform AWS provider's resource schema, classifies each `aws_*` resource
by prefix into a winterbaume service, then diffs against what is actually
registered in `winterbaume-server/src/main.rs`. The script needed manual
`PREFIX_OVERRIDES` for heterogeneous services like `aws_route53_*`
( resolved DNS resources versus `aws_route53resolver_*` rules ) and
`aws_iam_*` ( shared by IAM proper, Access Analyzer prefixes, etc. ) — a
pure prefix match misclassifies these. The output is two layers: a
workspace-level rollup ( total handled / missing ) and a per-service
section listing missing resource types so contributors can pick a service,
spec the new resources, and write the converter.

**Coverage waves landed in this push** ( per commit ):

- `iam`: 11 top-level converters ( `aws_iam_openid_connect_provider`,
  `aws_iam_saml_provider`, `aws_iam_virtual_mfa_device`,
  `aws_iam_server_certificate`, `aws_iam_signing_certificate`,
  `aws_iam_service_specific_credential`, `aws_iam_user_ssh_key`,
  `aws_iam_access_key`, `aws_iam_user_login_profile`,
  `aws_iam_account_password_policy`, `aws_iam_account_alias` ).
- `iam`: 7 sub-resource modifiers ( `aws_iam_role_policy`,
  `aws_iam_user_policy`, `aws_iam_group_policy`,
  `aws_iam_role_policy_attachment`, `aws_iam_user_policy_attachment`,
  `aws_iam_group_policy_attachment`, plus the two attachment converters
  that were previously hand-written and are now under the spec ).
- `iam`: 7 multi-target / exclusive converters
  ( `aws_iam_policy_attachment`, `aws_iam_role_policies_exclusive`,
  `aws_iam_user_policies_exclusive`, `aws_iam_group_policies_exclusive`,
  `aws_iam_role_policy_attachments_exclusive`, etc. ); IAM coverage now
  32 / 34 resource types ( 94 % ).
- `s3`: 22 sub-resource and named-config converters ( versioning, ACL,
  logging, lifecycle, replication, encryption, public access block,
  request payment, inventory, accelerate, object ownership, intelligent
  tiering, policy, notification, CORS, website, analytics, metric,
  object-lock configuration, etc. ). Two macros were extracted:
  `impl_bucket_subresource_converter!` for resources keyed solely by
  bucket name, and `impl_bucket_named_config_converter!` for resources
  keyed by `(bucket, name)` such as inventory and analytics. The macros
  collapse what would otherwise be ~22 near-identical converter bodies
  into single-line invocations.
- `route53`: 11 converters bringing route53 to 100 % ( hosted zone DNSSEC,
  query log config, traffic policy + instance, key signing key,
  CIDR collection + location, delegation set, vpc association
  authorization, zone association ).
- `apigateway`: 18 converters ( account, authorizer, base-path mapping,
  client certificate, documentation part/version, domain name +
  access association, gateway response, integration, integration
  response, method response, model, request validator, rest api policy,
  usage plan, usage plan key, vpc link ); apigateway coverage now 92 %.
- `rds`: 14 converters ( proxy, proxy default target group, proxy
  endpoint, proxy target, cluster endpoint, cluster snapshot, cluster
  parameter group, instance role association, snapshot, snapshot copy,
  certificate, integration, db shard group, custom db engine version );
  rds coverage now 66 %.
- `redshift`: 15 converters ( snapshot copy grant, hsm client / config,
  scheduled action, snapshot copy, snapshot schedule + association,
  authentication profile, endpoint access, endpoint authorization,
  event subscription, partner, resource policy, usage limit, logging,
  data share authorization ); redshift coverage now 74 %.
- `glue`: 12 converters ( catalog table, connection, data catalog
  encryption settings, dev endpoint, ML transform, partition, registry,
  resource policy, schema, security configuration, trigger, workflow );
  glue coverage now 75 %.
- `sagemaker`: 8 additional converters on top of existing scope ( model
  package group, code repository, app, app image config, device fleet,
  feature group, human task UI, model bias job definition ); sagemaker
  coverage now 43 %.

**InjectableServices growth.** sagemaker was not previously wired through
the dependency-injection layer in `winterbaume-server/src/main.rs` because
no converter needed it. Adding sagemaker converters meant introducing the
service: a new `sagemaker: Arc<winterbaume_sagemaker::SageMakerService>`
field on `InjectableServices`, an initialisation site, the `MockService`
trait downcast for the SDK-backed pathway, and the `use sagemaker as
sagemaker_conv;` import in the converter registration block. This is the
template for adding any not-yet-wired service: one field, one
initialisation, one MockService downcast, one converter import. None of
the per-service code in `crates/winterbaume-sagemaker/` itself had to
change — only the wiring.

**Spec-driven workflow paid off for sub-resource modifiers.** The
snapshot+mutate+restore idiom — read the current view, deserialize the TF
attrs into the model, merge the model's field set back onto the view,
write the view back — turned out to be the right shape for every "policy
attached to existing principal" pattern in IAM and every bucket-scoped
sub-resource in S3. Both macro layers ( the IAM hand-written helpers
`read_string_list` / `policy_name_from_arn` and the S3
`impl_bucket_subresource_converter!` macro ) sit on top of that idiom.
Once the spec has the right `*TfModel` shape, adding another IAM
sub-resource modifier or S3 named-config converter is a ~10-line edit.

**Hand-written escape hatch for nested-block reshaping.** A handful of TF
schemas use heterogeneously-shaped nested blocks that can't be modelled
as a flat `*TfModel`: e.g. AppFlow source/destination connector configs
that switch shape based on a `connector_type` discriminator. The pattern
that works is to keep the generated `*TfModel` as a thin envelope
holding `serde_json::Value` for the discriminated subtree, then call a
hand-written `tf_to_aws_<connector_type>()` helper from the converter.
This keeps the spec declarative for 95 % of fields and isolates the
inevitable hand-written code to the discriminated 5 %.

**clippy gotcha: field assignment outside Default::default().** Two glue
converters initially used the pattern `let mut sv = GlueStateView::default();
sv.x = Some(view);` which clippy flags in nightly with
`-D warnings`. The fix is the struct-update syntax:
`GlueStateView { x: Some(view), ..Default::default() }`. The hand-written
S3 sub-resource macros already used this form, which is why they passed
clippy on the first run.

### Repository deltas

- `.agents/skills/api-coverage/scripts/generate_terraform_resource_coverage.py`
  ( new ) + `.agents/skills/api-coverage/SKILL.md` ( updated for the
  three coverage reports ).
- `.agents/docs/TERRAFORM_RESOURCE_COVERAGE.md` ( new, refreshed twice ).
- `crates/winterbaume-terraform/specs/{iam,s3,route53,apigateway,rds,redshift,glue,sagemaker}.toml`
  extended with new `[[resource]]` blocks.
- `crates/winterbaume-terraform/src/converters/{iam,s3,route53,apigateway,rds,redshift,glue,sagemaker}.rs`
  extended with the corresponding hand-written converter bodies.
- `crates/winterbaume-tfstate-resource-models/src/{apigateway,rds,redshift,glue,sagemaker}.rs`
  regenerated from the updated specs.
- `crates/winterbaume-server/src/main.rs`: 392 inserted lines wiring the
  new converters, plus the new sagemaker field on `InjectableServices`.
- `.agents/skills/terraform-converter/SKILL.md` rewritten for the
  spec-driven workflow.
- `.agents/skills/write-e2e-tests/SKILL.md` updated with spec-driven
  converter discovery clauses ( Step 0b.1, Step 0d table additions,
  Step 5b ).

### Verification

- `cargo run -p tf-converter-codegen -- check`: clean ( all generated
  files in sync with specs ).
- `./.agents/bin/cargo.sh test -p winterbaume-terraform --no-fail-fast`:
  293 integration tests + 6 reshape tests pass; 22 ignored.
- `./.agents/bin/cargo.sh clippy -p winterbaume-terraform --all-targets
  --all-features -- -D warnings`: clean ( after the glue fixup
  described above ).
- `./.agents/bin/cargo.sh clippy -p winterbaume-tfstate-resource-models
  --all-targets --all-features -- -D warnings`: clean.
- `./.agents/bin/cargo.sh clippy -p winterbaume-server --all-targets
  --all-features -- -D warnings`: clean ( 82 min from cold cache; the
  load-bearing check, since it links every converter registration ).

### Coverage snapshot ( workspace-wide, post-wave )

- Total `aws_*` resources handled by winterbaume: **430** ( 425
  verified against the Terraform AWS provider schema ).
- Missing within classified prefixes: **768**.
- Highest-impact next services to tackle ( by missing count ): `ec2`
  ( 100 missing ), `directconnect` ( 18 ), `quicksight` ( 17 ),
  `sagemaker` ( 17 remaining ), `networkmanager` ( 16 ),
  `cloudfront` ( 15 ), `connect` ( 15 ), `iot` ( 15 ).

### Operational notes

- The user's "run cargo / clippy once after all subagents finish" rule
  proved important: spawning per-converter sub-agents that each invoke
  the per-crate clippy gate causes the workspace target dir to thrash
  under concurrent rustc invocations. The pattern that worked was:
  multiple sub-agents in parallel writing converter + spec changes, then
  a single `cargo fmt && cargo clippy -p winterbaume-terraform` from the
  top-level session once all sub-agents return. The 82-minute cold
  server clippy is unavoidable on a fresh cache but only needs to run
  once per multi-service wave.
- The `--maxfail` flag is pytest-only; for `cargo test` use
  `--no-fail-fast` or omit the flag entirely. ( Captured separately as
  feedback memory. )
- The next iteration should bring the workspace-level cargo clippy
  ( and the `verify-publish-ready` skill ) into a known-clean state
  before any release work; the per-crate gate is sufficient for routine
  development.

## 2026-05-11 (cont.): Close the EC2 Terraform-converter coverage gap

### Context

EC2 was by far the largest remaining coverage gap on the
`TERRAFORM_RESOURCE_COVERAGE.md` report — 100 of the missing 768 resource
types lived under the `ec2` prefix, more than 4× the next service
( `directconnect` at 18 ). The pre-existing EC2 converter file already
held 39 spec-driven converters; the brief was to extend that out to the
full TF AWS provider's EC2 surface.

### Findings

**Six sequential waves, one converter file.** Spec entries, generated
projection models, hand-written converter bodies, and server
registrations all live in shared files
( `crates/winterbaume-terraform/specs/ec2.toml`,
`crates/winterbaume-tfstate-resource-models/src/ec2.rs`,
`crates/winterbaume-terraform/src/converters/ec2.rs`, and
`crates/winterbaume-server/src/main.rs` ), which precludes a
fan-out-then-merge strategy: sub-agents working on the same files in
parallel would conflict on every line. The strategy that worked was
sequential dispatch — one sub-agent at a time, each with a focused brief
( wave-1 critical compute, wave-2 association/AMI families, wave-3 VPC
endpoints + VPN, wave-4 Transit Gateway family, wave-5 Client VPN +
Host/Fleet/Spot + carrier gateway + managed prefix list + peering,
wave-6 default-* + account singletons + IPAM extras + VPC block public
access + route servers + verified-access extras ) followed by an
orchestrator-side cargo gate at the end of the chain. Six waves
covered all 100 missing resources without rework.

**Distribution of "full state" vs "warning-only" converters.** Of the
100 new converters, 98 write real state into the `winterbaume-ec2`
crate. Two converters
( `aws_vpc_ipam_organization_admin_account`,
`aws_vpc_ipam_preview_next_cidr` ) have no matching slot in
`Ec2StateView` and were wired as warning-only injects — they
successfully deserialize the TF model so terraform plan/apply can read
them back out, but the body logs a `tracing`-style warning and does
nothing else. This is the same shape that
`aws_vpc_endpoint_private_dns` and `aws_vpn_gateway_route_propagation`
already use ( for genuinely-absent state ). It is preferable to a hard
error in the converter because a TF plan that references one of these
must still parse: the converter has to claim the resource type so it
isn't reported as "unknown resource".

**Modifier idiom scaled to 50+ converters without becoming repetitive.**
The snapshot+mutate+restore pattern — read the parent view from
`self.service.snapshot()`, mutate the field, write the partial
`Ec2StateView` back via `self.service.merge()`, and return
`Ok(vec![])` from `do_extract` to avoid double-emit — is now the
load-bearing pattern for every TF "association" / "attachment" /
"rule" / "policy" / "accepter" resource. Roughly half of the 100 new
converters are modifiers, and the pattern is the same shape in every
one. The benefit is that each modifier is ~30 lines of code; the cost
is that the parent resource's `do_extract` carries the entire
flattened JSON schema for the resource family ( ingress rules,
attachments, allowed principals, etc. ) so plans round-trip cleanly.

**Generic `aws_ec2_tag` needed a prefix dispatch table.** The TF
`aws_ec2_tag` resource attaches a single tag to any EC2 entity
identified by a generic `resource_id`. The converter dispatches by the
`resource_id` prefix ( `vpc-`, `subnet-`, `igw-`, `sg-`, `rtb-`,
`key-`, `acl-`, `eipalloc-`, `nat-`, `dopt-`, `eigw-`, `fl-`,
`pcx-`, `vpce-`, `pl-`, `cgw-`, `vgw-`, `vpn-`, `cagw-`, `eni-`,
`tgw-`, `i-`, `vol-`, `snap-`, `ami-`, `lt-`, `h-`, `fleet-`,
`sfr-`, `sir-` — 26 distinct prefixes ) to find the right view in
`Ec2StateView` and mutate its `tags` map. This is the only EC2
converter that touches more than one view in a single inject call.

**The "Build is still running" hand-off message.** One sub-agent
( wave 4 — Transit Gateway ) returned a literal "Build is still
running. Let me wait for the monitor event." final message rather than
a structured report. The actual files on disk reflected the
completed work, so the empty report was misleading but not destructive.
Manually verifying ( `grep '^type = "aws_ec2_transit_gateway' specs/ec2.toml`,
`grep AwsEc2TransitGateway server/main.rs`, `tf-converter-codegen check` )
took ~30 seconds and confirmed all 17 wave-4 resources were wired.
Lesson: always trust the file-system delta over the agent's prose
when it disagrees.

**State-layer gaps observed ( not fixed ).** The wave intentionally did
not modify the `winterbaume-ec2` crate's `views.rs` / `types.rs`. The
following gaps were noted across the sub-agent reports for future
state-layer work:

- `RouteTableAssociationView` has no `gateway_id` field, so gateway-side
  `aws_route_table_association` round-trips with a missing field.
- `RouteTableView` has no `propagating_vgws` slot, so
  `aws_vpn_gateway_route_propagation` is warning-only.
- `VpcEndpointView` has no `private_dns_enabled` field, so the toggle
  is warning-only.
- `ImageView` is missing `kernel_id`, `ramdisk_id`, `ena_support`,
  `sriov_net_support`, `tpm_support`, `boot_mode`, `imds_support`,
  `image_location`, plus the `aws_ami_copy` source-AMI metadata.
- `Ec2StateView` has no slot for the singleton spot datafeed
  subscription.
- The new `aws_vpc_route_server*` family ( 5 resources ) has no
  existing state representation; converters are full-state writes into
  fresh slots populated only on inject ( these may need extra
  modelling once the AWS service handlers are completed ).

### Repository deltas

- `crates/winterbaume-terraform/specs/ec2.toml` ( +3,904 lines, +100
  `[[resource]]` blocks ).
- `crates/winterbaume-tfstate-resource-models/src/ec2.rs` ( +1,390
  lines, regenerated ).
- `crates/winterbaume-terraform/src/converters/ec2.rs` ( +10,217 lines
  of new converter bodies, on top of the existing ~5,600-line file ).
- `crates/winterbaume-server/src/main.rs` ( +318 lines, 100 new
  `injector.register(ec2::Aws...Converter::new(...))` calls ).
- `.agents/docs/TERRAFORM_RESOURCE_COVERAGE.md` ( regenerated ).

Total diff: 15,820 insertions, 118 deletions, 5 files changed. Single
signed commit `3036c2ef`.

### Verification

- `./.agents/bin/cargo.sh run -p tf-converter-codegen -- check`: all
  generated files fresh.
- `./.agents/bin/cargo.sh test -p winterbaume-terraform --no-fail-fast`:
  293 + 6 tests pass, 22 ignored ( terraform-CLI E2E ).
- `./.agents/bin/cargo.sh clippy -p winterbaume-terraform --all-targets
  --all-features -- -D warnings`: clean.
- `./.agents/bin/cargo.sh clippy -p winterbaume-tfstate-resource-models
  --all-targets --all-features -- -D warnings`: clean.
- `./.agents/bin/cargo.sh clippy -p winterbaume-server --all-targets
  --all-features -- -D warnings`: clean.

### Coverage snapshot

| Metric                            | Before EC2 wave | After |
|-----------------------------------|----------------:|------:|
| Total `aws_*` resources handled   | 430             | 530   |
| Schema-verified                   | 425             | 525   |
| Missing within classified prefixes | 768             | 668   |
| EC2 missing                       | 100             | 0     |

EC2 is now the first AWS service to reach 100 % TF resource coverage
beyond the bare-essentials baseline. The next-largest gaps remain
`directconnect` ( 18 ), `quicksight` ( 17 ), `sagemaker` ( 17 remaining ),
`networkmanager` ( 16 ), `cloudfront` ( 15 ), `connect` ( 15 ),
`iot` ( 15 ).

### Operational notes

- The cargo-target dir under
  `.agents-workspace/tmp/target-claude-<session>/` accumulated to ~9 GB
  across the six waves because each sub-agent ran its own `cargo check`
  via the tf-converter-codegen tool. The orchestrator-side gate run
  reused this dir and finished in seconds; the cost of the sub-agent
  rebuilds was the warm-cache `cargo run -p tf-converter-codegen -- gen`
  invocation each one made ( ~5 s ).
- The "Build is still running" sub-agent message means: re-verify with
  `git diff --stat` and the four target files. Trust the disk state.
- Three sub-agents independently ran `cargo clippy` despite the brief
  saying "do not run clippy / build". This was harmless because the
  per-crate gate is incremental, but the future-task guideline stands:
  the orchestrator runs the gate once at the end.
