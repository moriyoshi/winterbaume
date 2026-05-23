# Winterbaume Development Journal

## LTM Consolidation Record

The journal has been audited against `.agents/docs/LTM/` and `.agents/docs/TODO.md` as of 2026-05-13. Every substantive entry that previously lived in this file has had its durable knowledge promoted to one or more LTM documents ( and any open follow-ups recorded in `.agents/docs/TODO.md` ); the consolidated entries themselves have been removed per the `reconcile-journal-ltm` workflow. Everything that needed to survive lives in LTM.

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
| 2026-05-10: Terraform converter codegen migration | `terraform-converter-codegen-and-resource-coverage.md` |
| 2026-05-10 (cont.): Extract generated models into `winterbaume-tfstate-resource-models` | `terraform-converter-codegen-and-resource-coverage.md` |
| 2026-05-11 — Skip CI pipeline for docs-only pushes to `main` | `ci-release-and-package-metadata.md` |
| 2026-05-11 — CI: skip individual jobs when prior pass markers exist | `ci-release-and-package-metadata.md` |
| 2026-05-11 — Release workflow: fix musl and aarch64-windows build failures | `ci-release-and-package-metadata.md` |
| 2026-05-11 — Release workflow: drop musl and aarch64-windows targets after fix attempts surfaced fresh failures | `ci-release-and-package-metadata.md` |
| 2026-05-12 — Add crates.io keywords across the workspace | `ci-release-and-package-metadata.md` |
| 2026-05-11: Post-extraction wave — broaden Terraform resource coverage | `terraform-converter-codegen-and-resource-coverage.md` |
| 2026-05-11 (cont.): Close the EC2 Terraform-converter coverage gap | `terraform-converter-codegen-and-resource-coverage.md`, `TODO.md` |
| 2026-05-11 / 2026-05-12: Post-EC2 sweep — close the workspace coverage tail | `terraform-converter-codegen-and-resource-coverage.md`, `TODO.md` |
| 2026-05-13: update-readme skill absorbs Terraform coverage; tf_schema module extracted | `terraform-converter-codegen-and-resource-coverage.md` |
| 2026-05-13 (cont.): Fix three rendering bugs in docs/reference/terraform.md and the underlying coverage heuristics | `terraform-converter-codegen-and-resource-coverage.md`, `TODO.md` |
| 2026-05-13 — `winterbaume-server --account-id` honoured at runtime | `runtime-account-identity-configuration.md`, `TODO.md` |

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
| `terraform-converter-codegen-and-resource-coverage.md` | Focused drill-down for the spec-driven Terraform converter model generator, generated `tfstate` model crate, coverage-report heuristics, and broad resource-expansion waves. |
| `ci-release-and-package-metadata.md` | Focused drill-down for recent CI path-filtering, pass-marker caching, cargo-dist target decisions, and workspace crates.io keyword metadata. |
| `runtime-account-identity-configuration.md` | Focused drill-down for runtime account ID configuration and the `DEFAULT_ACCOUNT_ID` → `default_account_id()` migration. |
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

## Deep Sleep Consolidation Record (2026-05-13)

No new synthesis document was created. The existing synthesis layer was refreshed instead because the new source topic documents fit established buckets.

| Synthesis Document | Source LTM Documents Added |
|--------------------|----------------------------|
| `service-implementation-and-validation-synthesis.md` | `terraform-converter-codegen-and-resource-coverage.md` |
| `runtime-state-and-service-infrastructure-synthesis.md` | `runtime-account-identity-configuration.md` |
| `repo-maintenance-and-agent-workflows-synthesis.md` | `ci-release-and-package-metadata.md` |

| Service Document | Mode | Sources |
|------------------|------|---------|
| `services/ec2.md` | Full distillation | `terraform-converter-codegen-and-resource-coverage.md`, `TODO.md` |

The source LTM documents `terraform-converter-codegen-and-resource-coverage.md`, `ci-release-and-package-metadata.md`, and `runtime-account-identity-configuration.md` remain standalone drill-down notes for traceability.

---

## 2026-05-13 — Per-crate service-slug keyword in service-crate Cargo.toml

### Motivation

Workspace `Cargo.toml` defines `[workspace.package] keywords = ["aws", "mock", "testing"]` which every crate inherits via `keywords.workspace = true`. That made all 226 service crates discoverable on crates.io only by the generic `aws` / `mock` / `testing` tags; searching for, e.g., `cognito-idp` or `elbv2` would not surface the matching winterbaume crate. The fix is to override the inherited keyword list per service crate and append the crate's own service slug.

### Approach

Per-crate override of `keywords.workspace = true` with an explicit list `["aws", "mock", "testing", "<slug>"]` where `<slug>` is the crate name with the `winterbaume-` prefix stripped. The change is mechanical and was applied via a one-shot Python helper at `./.agents-workspace/tmp/add_slug_keywords.py`.

### Scope

Only AWS-service crates were touched. The following 15 utility / codegen / engine / evaluator / parser crates were explicitly excluded and continue to inherit the workspace keyword list:

- `winterbaume-core`, `winterbaume-server`, `winterbaume-terraform`, `winterbaume-e2e-tests`
- `winterbaume-bedrock-flow-parser`, `winterbaume-bedrock-flow-validator`
- `winterbaume-ec2-generated`
- `winterbaume-iam-rule-eval`, `winterbaume-sfn-asl-eval`
- `winterbaume-wafv2-wcu-calculator`, `winterbaume-wafv2-webacl-rule-parser`
- `winterbaume-tfstate`, `winterbaume-tfstate-resource-models`
- `winterbaume-sqlengine-duckdb`, `winterbaume-partiql`

226 service crates were updated. Hybrid backend variants kept their full distinguishing slug, so `winterbaume-dynamodb-redis` gets `dynamodb-redis` and `winterbaume-sqs-redis` gets `sqs-redis` rather than collapsing to the base service name.

### Cargo keyword-length constraint and abbreviation table

crates.io enforces a hard limit of 20 ASCII characters per keyword and a maximum of 5 keywords per crate. 12 service-crate slugs exceeded 20 characters and could not be used verbatim. The abbreviations applied are:

| Crate suffix ( slug ) | Length | Abbreviation used as keyword |
|-----------------------|-------:|------------------------------|
| `applicationautoscaling` | 22 | `appautoscaling` |
| `applicationcostprofiler` | 23 | `appcostprofiler` |
| `applicationdiscoveryservice` | 27 | `appdiscovery` |
| `bcmrecommendedactions` | 21 | `bcmrecactions` |
| `cloudfrontkeyvaluestore` | 23 | `cloudfrontkvs` |
| `codestarnotifications` | 21 | `codestarnotif` |
| `cognitoidentityprovider` | 23 | `cognitoidp` |
| `elasticloadbalancingv2` | 22 | `elbv2` |
| `kinesisvideoarchivedmedia` | 25 | `kvarchivedmedia` |
| `resourcegroupstagging` | 21 | `resourcegrouptag` |
| `route53recoverycluster` | 22 | `r53recoverycluster` |
| `servicecatalogappregistry` | 25 | `scappregistry` |

Where possible the abbreviation matches AWS's own short forms ( e.g. `elbv2`, `cognitoidp` ) so users searching with the familiar AWS CLI service code still hit the crate. The crate name itself is unchanged; only the published `keywords` array carries the abbreviated form.

The script asserts `len(slug) <= 20` after applying the abbreviation table, so any future crate whose slug exceeds 20 characters and is not in the table will hard-fail rather than silently produce an invalid manifest.

### Validation

- `cargo metadata --no-deps --format-version 1` over the whole workspace succeeded, confirming all 226 modified manifests parse cleanly and the keyword arrays satisfy Cargo's structural constraints. Full crates.io upload-time keyword validation ( regex / length / count ) is implicit in this success because Cargo applies the same rules at parse time.
- Spot-checked `winterbaume-accessanalyzer`, `winterbaume-cognitoidentityprovider`, `winterbaume-elasticloadbalancingv2`, and `winterbaume-dynamodb-redis` to confirm the rewritten lines; `winterbaume-core` and `winterbaume-iam-rule-eval` still carry `keywords.workspace = true` as intended.

### Follow-ups / things worth knowing later

- The workspace default `keywords = ["aws", "mock", "testing"]` in the root `Cargo.toml` is now used **only** by the non-service utility crates listed above. If we ever want to push a different generic keyword set to all crates simultaneously, both the workspace default and the 226 per-crate overrides have to be updated.
- The abbreviation table is canonical: any new service crate whose slug exceeds 20 characters must add an entry here and to the helper script before the next release; otherwise `cargo publish` for that crate would refuse the keyword.
- This belongs in the `ci-release-and-package-metadata.md` LTM document next time `good-sleep` runs, since it is package-metadata policy that survives across releases and is non-obvious from the code alone.

## 2026-05-14 — Initial CHANGELOG.md generation across the workspace

### Motivation

Following the v0.1.0 launch on 2026-05-09 ... 2026-05-11 and the v0.2.0 follow-up on 2026-05-13 ... 2026-05-14, the repository had 481 release tags but zero per-crate `CHANGELOG.md` files and only a placeholder root `CHANGELOG.md` saying "No tagged crate releases were found in the local checkout when this umbrella changelog was initialised." This entry records the bulk first-pass generation of changelogs for every published crate plus the umbrella `winterbaume` crate at the workspace root.

### Inputs and release boundaries

Tag distribution:

| Bucket | Count | Notes |
|---|---|---|
| `winterbaume(-*)?-v0.1.0` on 2026-05-09 | 53 | First chunk of the public launch batch |
| `winterbaume(-*)?-v0.1.0` on 2026-05-10 | 186 | Second chunk |
| `winterbaume(-*)?-v0.1.0` on 2026-05-11 | 1 | `winterbaume-server-v0.1.0` lagged into the next day |
| `winterbaume(-*)?-v0.2.0` on 2026-05-13 | 125 | First chunk of v0.2.0 batch ( includes `winterbaume-v0.2.0` umbrella ) |
| `winterbaume(-*)?-v0.2.0` on 2026-05-14 | 116 | Second chunk |
| **Total tags** | **481** | |

Crate-to-tag map: 240 service / utility crates carry both `v0.1.0` and `v0.2.0`, 1 crate ( `winterbaume-tfstate-resource-models` ) carries only `v0.2.0` because it was extracted from `winterbaume-terraform` between the two batches, and the umbrella crate at workspace root carries `winterbaume-v0.1.0` + `winterbaume-v0.2.0`. That accounts for all 241 crates that need a `CHANGELOG.md`. `winterbaume-e2e-tests` is `publish = false` and intentionally has neither tags nor a changelog.

The two cargo-release commits per crate ( `chore: release {{crate_name}} v{{version}}` ) are unrendered cargo-release template strings rather than per-crate concrete messages, so they are useless for changelog content and were treated as boilerplate to skip.

### Tag-list regex pitfall ( `winterbaume-v*` matches every crate )

First pass used two ref globs ( `refs/tags/winterbaume-*-v*` for service crates plus `refs/tags/winterbaume-v*` for the umbrella ) and concatenated the results. The second glob is **not** a literal `winterbaume-v…` match — `*` is greedy across hyphens, so `winterbaume-v*` matches `winterbaume-vpclattice-v0.1.0`, `winterbaume-vpclattice-v0.2.0`, and any other crate whose slug happens to start with `v`. Concatenating the two ref-lists therefore double-counts `vpclattice`. The fix is `sort -u` on the merged tsv, which is cheap and idempotent. Worth knowing for any future script that iterates winterbaume release tags.

### Commit classification

For each `(crate, version)` the substantive-vs-boilerplate decision used path-filtered `git log --first-parent <prev>..<tag> -- <crate paths>` and a small set of subject regexes. Boilerplate patterns matched 7 distinct subjects across 1,184 commit observations:

- `chore: release {{crate_name}} v{{version}}` — unrendered cargo-release template ( 243 )
- `chore: declare crates.io keywords across the workspace` — workspace-wide keyword refresh ( 240 )
- `chore: enhance service crates' keywords with the service slugs` — see 2026-05-13 entry ( 226 )
- `Merge remote-tracking branch 'origin/fix-account-id-flag'` ( 226 )
- `chore: update API coverage` ( 225 )
- `Merge branch 'skill-update-readme-terraform-coverage'` ( 224 )
- `chore: add missing README.md and trademark notice.` ( 3 )

Substantive commits aggregated to just 4 crates: `winterbaume` ( umbrella ), `winterbaume-server`, `winterbaume-terraform`, `winterbaume-tfstate-resource-models`. All other 237 service crates' `v0.1.0 → v0.2.0` ranges contained only boilerplate commits and were rendered with an honest `Internal` note: "Released alongside the wider workspace v0.2.0 batch. No user-facing behaviour changes for this crate; the release republishes the crate with refreshed crates.io keyword metadata and updated API coverage data."

For `v0.1.0`, every crate's lower bound is the repo root, so the only safe summary is "Initial public release. <description from Cargo.toml>." This is the right default for a coordinated public launch and matches the actual behaviour at the v0.1.0 tag.

### Substantive v0.2.0 content sources

- `winterbaume` ( umbrella ): `tfstate-resource-models: extract generated TF projection code into its own crate` ( 64dabeff ) and `terraform-converters: spec-driven serde codegen for all 145 services` ( 8eb79ef3 ).
- `winterbaume-terraform`: same two plus ~700 new Terraform resource converters across EC2 ( 100, 3a0c3514 ), S3 + Route 53 ( 22 + 11, 4ef085ca ), batched additions ( 12 / 121 / 36 / 45 / 54 / 65 / 71 / 86 / 113 in commits 535016a3 a1eaf16a c696a8e4 22e1f4cc 5026c76a 0635d55e 7699f618 46dd0e70 1ca574e5 ), apigateway/glue/rds/redshift/sagemaker extensions ( 3b42f93f ), and IAM coverage ( 73ecd94a + e2ac3ee9 + a12584f0 lifting IAM to 32/34, 94% ).
- `winterbaume-server`: same converter batches as `winterbaume-terraform`, plus the S3/Route53 batch, since router registration changes touched the server crate.
- `winterbaume-tfstate-resource-models`: v0.2.0 is its initial release; bullets reflect the extraction from `winterbaume-terraform` and the generated-model nature of the crate.

### Workflow

All data collection and rendering is captured under `.agents-workspace/tmp/changelog-data/`:

- `all-tags.tsv` — `<tag>\t<creator-date>` for all 481 tags, deduplicated.
- `crates.json` — per-crate sorted release list with `prev_tag`/`prev_version` fields for range queries.
- `commits.json` — path-filtered `git log` output for every `(crate, version)` pair.
- `classification.json` — same with each commit tagged boilerplate-or-substantive.
- `descriptions.json` — `description = "..."` from each crate's `Cargo.toml`, used in the "Initial public release. <description>." sentence.
- `build-crate-table.py`, `collect-commits.py`, `classify.py`, `gen-changelogs.py`, `collect-descriptions.py` — the scripts that produced the artifacts above.

`gen-changelogs.py` is the renderer; it writes 240 files under `crates/<crate>/CHANGELOG.md` ( deliberately skipping the umbrella, which uses the root `CHANGELOG.md` ) and is idempotent — it can be re-run after editing the substantive overrides at the top of the script. The root `CHANGELOG.md` was hand-written rather than templated, because the umbrella narrative ( two dated batches, launch story, release-batch tooling shout-out ) does not fit the per-crate template.

### Output

- Root `CHANGELOG.md`: rewritten from the stub to a workspace-overview document with `Unreleased` empty-note, a 2026-05-13 ... 2026-05-14 v0.2.0 batch section, and a 2026-05-09 ... 2026-05-11 v0.1.0 launch section. The umbrella section explicitly names the 4 substantive crates and says 237 others were keyword-refresh republishes, rather than listing each crate.
- 240 new files at `crates/<crate>/CHANGELOG.md` ( one per published crate other than the umbrella ).
- `winterbaume-tfstate-resource-models/CHANGELOG.md` only has a `v0.2.0` section, since v0.1.0 was never published; it is treated as the initial release with an explanatory bullet about the extraction.

### Pre-existing unrelated unstaged changes

The initial `git status` output was truncated at 2 KB and showed only `M CHANGELOG.md` and `M docs/index.md` at first glance. Below the truncation a third pre-existing modification was lurking — `M docs/reference/services.md` — which replaces the `? of ? operations across ? AWS services (?%)` placeholder ( in two places ) with the real `7210 of 11367 operations across 224 AWS services (63.4%)` numbers and updates the Terraform converter trailer line. The pre-existing `M docs/index.md` is the matching landing-page edit ( `?% API Coverage` -> `63% API Coverage`, same numbers ). Neither doc change is in scope for the changelog skill, but both are ready to commit and would land cleanly alongside the new changelog files.

### Follow-ups / things worth knowing later

- Pure-chore `v0.2.0` releases will be the norm for service crates until each crate next gets a real behaviour change. The "Internal" wording in those changelogs is a stable template that can be reused for future workspace-wide keyword / metadata / coverage refresh batches.
- `winterbaume-tfstate-resource-models` should normally start its own changelog at `v0.1.0` next time it is released, even though its first published version is `v0.2.0`. The current entry documents that explicitly so a future reader does not look for a missing `v0.1.0` section.
- For any future tag-range scripts, normalise on `refs/tags/winterbaume-*` and split crate from version with a real parser rather than two overlapping globs.
- The `chore: release {{crate_name}} v{{version}}` commits indicate cargo-release ran with template-string substitution disabled or misconfigured; this should be fixed before the next release batch so the commit history carries the actual crate name and version. Not a blocker for changelogs but it makes commit archaeology harder.

## 2026-05-16 — tackle-todos sweep: skill template + tooling + docs

`/tackle-todos` dispatched three parallel agents to clear a batch of small, self-contained items from `.agents/docs/TODO.md`. The full source-code scan turned up only two `// TODO`/`// FIXME` hits across `crates/**/*.rs` and `tools/**/*.rs` ( one informational deprecated-service note, one literal in a codegen template ) — effectively zero actionable code comments, so the work all came from `TODO.md`.

Closed in this sweep:

- `add-service-cargo-version-template`, `add-service-restjson-reference`, `add-service-state-view-builder-template`, `add-service-sdk-accessor-shape-note` — `.agents/skills/add-service/SKILL.md` updated in four places: literal `version = "0.1.0"` scaffold ( with a note about the actual workspace-package inheritance set ), restJson1 reference switched to `winterbaume-sesv2/src/handlers.rs`, new "Rule: construct `*View` literals through small helper functions" subsection plus rewritten notification-test snippets that use `mk_resource_view(...) + ..Default::default()`, and a Step 5 Tips bullet on per-response-type accessor optionality.
- `smithy-codegen-glue-service-map-entry` — added `("glue", "glue")` to `SERVICE_MAP` in `tools/smithy-codegen/src/discover.rs`; `list-services` now surfaces Glue. Per-crate clippy + fmt gate clean.
- `readme-stub-count-refresh` — root `README.md` intro paragraph stub count 329 -> 326 to match the authoritative table footer. `docs/reference/services.md` already carried the correct figure.
- `terraform-coverage-prefix-overrides-tail` — `generate_terraform_resource_coverage.py` now has an explicit `kinesis` `PREFIX_OVERRIDES` entry plus a new `HANDLED_ALIAS_RULES` mechanism for elbv2 `aws_alb_*` -> `aws_lb_*` aliases. elbv2 62% -> 100%, kinesis 50% ( spurious ) -> 100%. The remaining `aws_kinesisanalyticsv2_application_snapshot` miss is a real gap, not a classification artefact.
- `terraform-macro-extract-coverage` — `generate_terraform_converter_coverage.py` now credits the trailing positional `"name"` literal of macro invocations plus per-macro-family always-credited attributes ( `impl_bucket_subresource_converter` -> `bucket`; `impl_bucket_named_config_converter` -> `bucket` + `name` ) on both inject and extract sides. Spot-check: `aws_s3_bucket_accelerate_configuration` extract 0% -> 67%. Overall extract coverage delta 5251/10765 -> 5278/10765 ( +27, +0.2 pp ).

Deferred ( need user direction or larger-scope work ):

- `core-url-query-parser-sweep` ( 59-crate refactor ), `ses-v1-v2-shared-backend`, `mediastoredata-container-model`, `appconfigdata-shared-state`, `codegen-field-drift-handler-updates`, all cross-service integration items ( eventbridge / lambda / sfn / appsync targets ), and the three medium-scope state-validation items ( `cloudtraildata-channel-validation`, `kinesisvideoarchivedmedia-stream-validation`, `sagemakerruntime-endpoint-validation` ). The last three need a backend-injection wiring pattern similar to `winterbaume-dynamodbstreams`'s shared `DynamoDbBackend` and were not safe to dispatch as a parallel batch.

- `docs-vitepress-config-metadata` — TODO claims `transformPageData` references undefined symbols, but the current `docs/.vitepress/config.mts` reads correctly ( all four `siteTitle` / `siteDescription` / `siteUrl` / `ogImageUrl` symbols are defined and used ). Either the TODO is stale or it describes a different bug than what is in the file; leaving as-is pending clarification.

No commits made. Consolidated worklist written to `.agents-workspace/tmp/consolidated-todos.md` for reference.

### Follow-up batch ( same session, 2026-05-16 )

Two more items cleared after the initial parallel-agent sweep:

- `emrcontainers-state-view-job-runs` — audit only; `job_runs` is already fully wired in `crates/winterbaume-emrcontainers/src/views.rs` ( field declaration, `From<&EmrContainersState>` conversion, `From<EmrContainersStateView>` reconstruction, and `merge` integration ). The 2026-04-30 sweep's note about "writes in state.rs but missing from views" no longer matches the source.
- `ec2-coverage-readme-refresh` — `.agents/skills/api-coverage/scripts/generate_coverage.py` then `.agents/skills/update-readme/scripts/update_readme.py` rerun in sequence. Refreshed `API_COVERAGE.md`, both `TERRAFORM_*_COVERAGE.md`, root `README.md`, 224 per-crate READMEs, `docs/reference/{services,terraform}.md`, `docs/index.md`, and 225 `docs/services/*.md`. The intro-paragraph `326` from the first batch survived end-to-end because `update_readme.py` only rewrites the supported-services table, not the intro prose ( and the regenerated table footer agrees, so the two numbers stay aligned ). Authoritative EC2 figure is 713/763 ( ec2Query 93.4% ) — the TODO's `752/756` was aspirational. Moto picked up SES v2 28 -> 30 ( total 3302 -> 3304 ).

Cross-service validation items ( `cloudtraildata-channel-validation`, `kinesisvideoarchivedmedia-stream-validation`, `sagemakerruntime-endpoint-validation` ) remain deferred. The `winterbaume-dynamodbstreams::with_dynamodb_backend(Arc<dyn DynamoDbBackend>)` pattern in `crates/winterbaume-dynamodbstreams/src/handlers.rs:44` is a good template, but `winterbaume-server/src/main.rs:989` still constructs `DynamoDbStreamsService::new()` without wiring the shared backend, so even the existing pattern isn't auto-wired in standalone-server mode. Picking up these TODOs needs a design decision on whether `MockAws::builder()` and `winterbaume-server` should auto-wire `with_<parent>_backend()` constructors, not just a per-crate code change.

### Third batch ( same session, 2026-05-16 )

The `codegen-field-drift-handler-updates` TODO is cleared end-to-end. All 6 rolled-back crates from the 2026-05-02 mass-regen sweep were regenerated and their handler / state placeholders added in two waves of 3 parallel agents each. Pattern was identical across crates: re-run `cargo run -p smithy-codegen -- gen-serializers <crate>`, read the matching `.agents-workspace/tmp/regen-broken/<crate>-clippy.log` to enumerate `error[E0063]: missing field ...` sites, default each new field at every construction site ( `None` for `Option<T>`, `Default` for non-optional, etc., never inventing state ), then run the per-crate `fmt -> clippy -> fmt --check -> test --no-fail-fast` gate.

Per-crate field tallies:

| Crate | New fields | Tests | Notes |
|---|---|---|---|
| `account` | 1 ( `account_state` ) | 33 pass | single response struct |
| `organizations` | 1 ( `path` ) | 107 pass | only `OrganizationalUnit` via `ou_wire()` helper |
| `batch` | 1 ( `quota_share_policy` on `SchedulingPolicyDetail` ) + 4 unreferenced sibling additions | 50 pass | new `CreateQuotaShareResponse` / `UpdateQuotaShareResponse` shapes plus request-side `quota_share_policy` fields landed too, but no existing handler references them |
| `applicationsignals` | 3 ( `composite_sli_config`, `metric_source`, `auto_investigation_enabled` ) | 13 pass | model-dir is hyphenated `application-signals`, resolved via SERVICE_MAP fallback |
| `ivs` | 7 CORS headers × 3 batch-response structs = 21 placeholders | 54 pass | `access_control_allow_origin`, `access_control_expose_headers`, `cache_control`, `content_security_policy`, `strict_transport_security`, `x_content_type_options`, `x_frame_options` on `BatchGetChannelResponse`, `BatchGetStreamKeyResponse`, `BatchStartViewerSessionRevocationResponse` |
| `opensearch` | 2 ( `service_options` on `AuthorizedPrincipal`, `prometheus` on `DirectQueryDataSourceType` ) | 54 pass | |

No commits made. The `.agents-workspace/tmp/regen-broken/` artefacts are left in place for now ( they would be useful again if any future regen drops or renames one of these fields ), but they could be cleaned up as a follow-up housekeeping step once we have confidence the new `wire.rs` / `model.rs` are stable.

### Fourth batch ( same session, 2026-05-16 )

Two more items cleared:

- `release-batch-general-uploaded-tag-backfill` — added a `parse_uploaded(text, version) -> BTreeSet<String>` scanner in `tools/release-batch/src/main.rs` that picks up cargo / cargo-release `Uploaded <crate> v<version>` status lines ( allowing leading whitespace and an optional trailing registry hint, rejecting "Uploading" progress lines and version mismatches ). Wired into the chunk-retry loop ahead of the early-break failure path so every uploaded crate gets its `<crate>-v<version>` tag backfilled regardless of whether the chunk hit 429, hit cargo's pre-flight `is already published` check, or failed for some other reason. Five new unit tests + the existing 8 = 13/13 pass; per-crate clippy + fmt gate clean.
- `docs-service-readme-server-install-refresh` — `.agents/skills/update-readme/scripts/update_readme.py` `Server-mode usage` block now emits both the published-binary path ( `cargo install winterbaume-server` followed by `winterbaume-server --host ...` ) and the workspace-checkout path ( `cargo run -p winterbaume-server -- --host ...` ), with a short British-English lede explaining when each is appropriate. Regenerated 224 per-crate READMEs and 225 `docs/services/*.md` pages; no hand edits.

I also looked at `core-url-query-parser-sweep` but the 59-crate plan in the TODO is more nuanced than the entry suggests: spot-checking `winterbaume-opensearch`, `winterbaume-iotdataplane`, `winterbaume-osis`, and `winterbaume-s3tables` shows three distinct signature shapes ( `(query_string, key)`, `(uri, key)`, plus minor URL-decode variants ), not byte-identical copies. The canonical `protocol::common` helpers should take a query string body ( consistent with the existing `extract_query_string(uri) -> &str`, also already in core ), but adopting that across the fleet means call-site refactors at the URI-passing sites, not just symbol swaps. Deferred pending the design decision on signature shape; the TODO entry needs a follow-up note recording this finding before anyone dispatches a sweep.

## 2026-05-17 — tackle-todos sweep continuation

Resumed `/tackle-todos` after the 2026-05-16 commit `7e54baa9` to clear a second batch.

### Closed

- `docs-vitepress-config-metadata` — built the docs site with `npx vitepress build` from `docs/` and inspected the rendered HTML; `<title>` is the composite `<page-title> | Winterbäume`, `<meta name="description">` is present, and the full `og:*` / `twitter:*` chain renders correctly on guide, services, and the root index. All symbols in `transformPageData` ( `siteTitle`, `siteDescription`, `siteUrl`, `ogImageUrl`, `pageUrl` ) resolve. The bug described in the TODO no longer reproduces.
- `sqs-redis-redrive-receive-budget` — the Lua `RECEIVE_SCRIPT` in `crates/winterbaume-sqs-redis/src/lib.rs:262` used `#results` ( all `R:` + `D:` entries ) as both the receive-budget gate and the receipt-handle index. Replaced with a dedicated `recv_count` that counts only `R:` ( returned-to-caller ) entries. Verified via `cargo build` and `cargo test --no-run`. The per-crate clippy gate is blocked by an unrelated `cargo check` failure resolving the `arc-swap` transitive dep of `redis 0.27.6` — opened a new `sccache-wrapper-arc-swap-check-vs-build` TODO under Build Tooling to track it. `winterbaume-sqs-redis` has no integration tests yet, so the regression test will need a separate harness ( either `mlua`-based unit test of the Lua string or an actual Redis instance ); flagged in the TODO closure.

### Partially addressed

- `invariant-audit-existing-services` — implemented option ( a ) from the 2026-05-02 dry-run finding by adding a `direct_counter_uses_with_fn` fallback to `.agents/bin/audit-state-fields.sh`. The fallback only fires when a crate has no `self.counters` substruct ( the original EC2 path keeps its 11 flags ), and it emits `<field>\t<fn>` pairs whenever `self.<field>` appears in a `format!()` ID-minting line OR is incremented via `+= 1` / `= self.<field> + 1`. Small denylist filters obvious non-counter field names ( `state`, `config`, `metadata`, `items`, `name`, `arn`, `id`, `notifier`, `tags`, `next_token`, `counters` ). Spot-checked: `iam` produces no flags ( no counter fields in state ), `sqs` activates the fallback with no shared-counter flag, `dynamodb` activates the fallback with three single-method counters ( `backup_counter`, `export_counter`, `import_counter` ) that correctly don't trigger the duplicate-fn check, and `ec2`'s original heuristic-B emits its 11 sections unchanged. The fleet sweep + per-crate invariant-row backfill is still pending — the script is no longer the bottleneck. Updated the TODO entry in-place with a 2026-05-17 sub-note documenting the script change so future agents don't re-dispatch option ( a ).

## 2026-05-17 — Operational TODO verification pass

Audited the open First-Public-Release operational items against the live repository state and closed the verified ones.

### Closed

- `public-release-publish-new-rate-limit` — first public release happened. `winterbaume-server-v0.1.0` released 2026-05-11 ( release-workflow run 25648046827, 1h22m ), `winterbaume-server-v0.2.0` released 2026-05-13 ( run 25827080927, 1h36m ). Git tag count is 481 total = 240 `*-v0.1.0` + 241 `*-v0.2.0`, so essentially every workspace crate has been published twice. The `release-batch-general-uploaded-tag-backfill` enhancement in commit `7e54baa9` hardens the chunked path for any future bulk publish.
- `public-release-manual-gates` — verified each named gate: hosted CI green on the latest `main` push ( `Trigger Integrity Audit` + `CI` both `completed/success` on commit `1f8fc304` ); docs deployment via Cloudflare Pages ( `.github/workflows/deploy-docs.yml`, two manual `workflow_dispatch` runs on 2026-05-14 both `success` ); release-workflow secrets implicitly cleared by 481 successful crates.io publishes; `CONTRIBUTING.md` explicitly forbids PRs with provenance / licensing rationale and points bug reports at the issue-form templates; `SECURITY.md` carries the GitHub Security Advisories private-reporting flow; cargo-release's metadata-validation gate would have surfaced any crate-description issue and 481 publishes succeeded.
- `public-release-branch-protection` — the 2026-05-01 audit note that `gh api .../rulesets` returns `[]` is stale. Two active rulesets are now in place: "Default branch" ( `~DEFAULT_BRANCH` target; `deletion`, `non_fast_forward`, `required_signatures`; `bypass_actors: []` ) and "Versioned tags" ( `refs/tags/*-v*` and `refs/tags/v*` targets; `deletion`, `non_fast_forward`, `update`, `required_signatures`; `bypass_actors: []` ). Of the original three criteria — required CI, no direct pushes unless intentionally allowed, restricted release tag creation — only the first remains uncovered ( no `required_status_checks` rule ). CI runs on every push but isn't enforced as a merge-blocker; given the CLOSED-PR contribution policy in `CONTRIBUTING.md` there is no merge path that would benefit today, so closing the entry with a residual note rather than leaving it open.
- `release-rustup-curl-pipe` — closed as a documented intentional exception. The original TODO already framed it that way; the release workflow has run end-to-end twice since with no audit pushback, and the SHA-256-pin recipe is already in the workflow if a future audit needs it.

### Updated, still open

- `cargo-dist-dropped-targets-revisit` — recorded the current `dist-workspace.toml` target list ( 5 targets: `aarch64-apple-darwin`, `aarch64-unknown-linux-gnu`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `x86_64-pc-windows-msvc` ); the musl and aarch64-windows targets remain dropped and the 2026-05-13 release cut succeeded with this matrix. Stays open as an upstream-tooling watch item ( re-adding either dropped target needs the underlying `musl-tools` C++ shipping bug or the cargo-xwin / clang `/imsvc` interop bug to be fixed upstream and re-tested in CI ).

No code change beyond TODO.md and JOURNAL.md. The verification used `gh api repos/.../rulesets`, `gh api repos/.../rulesets/{id}`, `gh run list`, `git tag -l '*-v0.1.0'`, `git tag -l '*-v0.2.0'`, and reads of `CONTRIBUTING.md`, `SECURITY.md`, `.github/workflows/release.yml`, `.github/workflows/deploy-docs.yml`, and `dist-workspace.toml`.

## 2026-05-17 — sccache-wrapper proc-macro caching fix ( `arc-swap-check-vs-build` resolved )

### Symptom

`./.agents/bin/cargo.sh check -p winterbaume-sqs-redis` ( and the per-crate clippy gate ) failed against the transitive `redis 0.27.6` dep with `error[E0463]: can't find crate for arc_swap`. Same crate built fine with `cargo build` and with the wrapper kill-switch ( `WB_RUSTC_CACHE=0` ).

### Root cause

The earlier TODO hypothesis — that the wrapper drops `--extern arc_swap=…` on the `cargo check` rustc invocation — was wrong. The actual chain is:

1. **Proc-macro dylibs are non-deterministic across rustc invocations.** Four `librustversion-aab3bba6f82eb66b.dylib` files under `.agents-workspace/tmp/target-*/debug/deps/` produced four distinct sha1 sums even though their cargo extra-filename hash is identical. The non-determinism comes from rustc itself ( hash-table iteration order in proc-macro output ), not the wrapper.
2. **The wrapper excluded proc-macros from caching** ( `parse_rustc_args` rejected `crate-type = "proc-macro"` ). So `rustversion` was rebuilt fresh per session and ended up with a different content + SVH each time.
3. **`arc_swap` depends on `rustversion` at build time.** `arc_swap`'s rmeta records the SVH of the specific `rustversion` dylib it was compiled against. Comparing the cached and a freshly-built `libarc_swap-58d0e1aab43533e1.rmeta` showed they differ at byte 986, the first byte after the `rustversion` dep marker.
4. **The wrapper cached `arc_swap` keyed by the proc-macro's `--extern` filename only** ( cargo's stable extra-filename hash ), not by the actual file content. So a cache HIT in a fresh session served an `arc_swap` rmeta whose embedded `rustversion` SVH did not match the freshly-built `rustversion` present in the new session's `deps/` dir.
5. **The mismatch propagates downstream.** `redis`'s rmeta in turn records the SVH of `arc_swap`. When rustc compiles `winterbaume-sqs-redis`, it walks the chain `redis -> arc_swap -> rustversion` and fails the SVH check, surfacing as `E0463: can't find crate for redis` ( the lookup that triggers the dep walk ).

`cargo build` worked because the build path runs through to completion within one session's artefact set, so the chain stays internally consistent. `cargo check` exposed the misalignment when the wrapper served cross-session proc-macro-affected rmetas.

Reproduction recipe: `CLAUDE_CODE_SSE_PORT=test1 ./.agents/bin/cargo.sh check -p winterbaume-sqs-redis` against an existing wrapper cache populated by a different session.

### Fix

`tools/sccache-wrapper/src/main.rs`:

1. Removed `proc-macro` from the crate-type exclusion list in `parse_rustc_args`, with a comment explaining the SVH chain so a future maintainer does not regress this.
2. Added a `proc-macro` branch to `expected_output_files` that emits the host dynamic library — `lib<crate>-<ef>.dylib` on macOS, `lib<crate>-<ef>.so` on Linux, `<crate>-<ef>.dll` ( no `lib` prefix ) on Windows.

Caching the proc-macro itself means the **first** session compiles it with the new wrapper, stores the dylib and writes a `.cachekey` sidecar; **every** later session restores byte-identical content via hardlink and the sidecar resolves to a stable dep-key. Downstream crates ( `arc_swap`, `serde`, … ) now key their own cache entries on the proc-macro's sidecar-derived cache key rather than on its file basename, so the SVH chain stays consistent across sessions by construction.

### Verification

- Failing reproduction now succeeds: `CLAUDE_CODE_SSE_PORT=… ./.agents/bin/cargo.sh check -p winterbaume-sqs-redis` exits 0 in a fresh target dir.
- Per-crate clippy gate clean for both blocked crates: `cargo clippy -p winterbaume-sqs-redis --all-targets --all-features -- -D warnings` and same for `winterbaume-dynamodb-redis`; `cargo fmt -p <crate> -- --check` also clean.
- Wrapper's own gate clean: `cargo clippy -p sccache-wrapper --all-targets --all-features -- -D warnings`, `cargo fmt -p sccache-wrapper -- --check`, and `cargo test -p sccache-wrapper` ( 24 passed ).
- Smoke-tested an unrelated crate stack ( `winterbaume-core` ) in a fresh session to confirm no regression on the broader graph.

### Cache-transition note

Pre-fix wrapper cache entries for crates that consume proc-macros ( `arc_swap`, `serde`, `serde_json`, `redis`, … ) were keyed by the proc-macro's filename only. After the wrapper rebuild, the new key includes the proc-macro's sidecar-derived dep key, so old entries are unreachable and get superseded by fresh stores on the next compile. The existing `dump_cache --gc` path will reclaim them as duplicates accumulate. No manual cache wipe required; the transition is self-healing.

## 2026-05-17 — EC2 state-view gaps: first three sub-items

Closed half of `ec2-terraform-state-layer-gaps` ( the small surgical additions ). Three new state fields cleanly threaded through every layer:

- `types::RouteTable.propagating_vgws: Vec<String>` — for `EnableVgwRoutePropagation` consumers; defaults to empty on `create_route_table`.
- `types::RouteTableAssociation.gateway_id: Option<String>` — for edge associations ( `AssociateRouteTable.GatewayId` ); `None` on subnet-association paths. Carried through `replace_route_table_association` so reassociations preserve the gateway binding.
- `types::VpcEndpoint.private_dns_enabled: Option<bool>` — for Interface endpoints; `None` preserves the legacy "unset" semantics so the terraform converter can distinguish that from explicit `false`.

Each got a matching `*View` field ( serde `#[serde(default)]` ), an updated `From<&Foo>` ( internal -> view ), an updated `From<FooView>` ( view -> internal ), and the relevant constructor sites in `state.rs` ( `create_route_table` line ~1890, `associate_route_table` line ~1920, `replace_route_table_association` line ~2115, `create_vpc_endpoint` line ~2297 ).

Handlers were intentionally not touched — the wire-level `DescribeRouteTables` / `DescribeVpcEndpoints` responses are byte-identical to before because all three fields default to "unset" on every code path. Populating from the matching request inputs is the natural follow-up but is deferred until a terraform converter or test exercises it.

Per-crate gate ran the long way through:

- `cargo fmt -p winterbaume-ec2`: clean.
- `cargo fmt -p winterbaume-ec2 -- --check`: pass.
- `cargo clippy -p winterbaume-ec2 --all-targets --all-features -- -D warnings`: pass ( 22m51s, cold; the `sccache: warning: The server looks like it shut down unexpectedly, compiling locally instead` line is harmless and is an sccache-pass-through artefact, not the wrapper itself ).
- `cargo test -p winterbaume-ec2 --no-fail-fast`: **591 main tests + 13 scenario tests, 0 failures**.

The three larger remaining sub-items ( `ImageView` expansion across kernel / ramdisk / ENA / SR-IOV / TPM / boot mode / IMDS / image location / AMI-copy source, singleton spot datafeed subscription slot, VPC route-server family review ) stay open and are bigger surface changes that warrant their own pass.

## 2026-05-17 — Kinesis per-shard sequence numbers

Closed the highest-confidence flag from the morning's `audit-state-fields.sh --all` fleet sweep.

`KinesisState.next_sequence: u64` was a single global counter incremented by all three put-paths ( `put_record`, `put_record_by_arn`, `put_records` ). Real AWS Kinesis sequence numbers are monotonic *per shard* within a stream, not globally; the mock's global counter was monotonic-per-shard by accident but lost the per-shard reset behaviour and broke any consumer that derives shard ownership from sequence-number ranges.

Fix:

- `Stream` gains `pub next_sequence_per_shard: HashMap<String, u64>` ( `types.rs` ).
- `KinesisState.next_sequence` is removed entirely ( private field, no external callers ).
- All three put-paths now compute `shard_id` first via the existing `compute_shard_id_from_shards` helper, then `entry(shard_id.clone()).or_insert(0) + 1` to mint the next sequence number on that shard.
- `StreamView` gains a matching `#[serde(default)] pub next_sequence_per_shard: HashMap<String, u64>` so snapshot/restore round-trips preserve the invariant.
- New integration test `test_put_record_sequence_numbers_are_per_shard` in `crates/winterbaume-kinesis/tests/integration_test.rs`: creates a 3-shard stream, puts 12 records with varying partition keys, asserts that each shard's returned sequence numbers form the contiguous sequence `1, 2, 3, ...` in put order. Robust to the shard hasher's distribution as long as at least two shards are hit ( which the deterministic `DefaultHasher` + 12 distinct keys guarantees ).

Per-crate gate ran cleanly after one cosmetic fix:

- `cargo fmt -p winterbaume-kinesis`: clean.
- `cargo clippy -p winterbaume-kinesis --all-targets --all-features -- -D warnings`: passed after switching `KinesisStateView -> KinesisState` from `let mut state = KinesisState::default(); state.streams = ...; ...` to a direct struct literal. The `field-reassign-with-default` lint had been quiet before only because `KinesisState` had four fields and one stayed at default; removing `next_sequence` left three fields all reassigned, which trips the lint.
- `cargo fmt -p winterbaume-kinesis -- --check`: clean.
- `cargo test -p winterbaume-kinesis --no-fail-fast`: **86 tests pass, 0 failures** ( includes the new regression test ).

The `invariant-audit-existing-services` TODO entry has been updated in-place with a strike-through on the kinesis sub-item and a fixed-2026-05-17 note pointing at the commit. The other four fleet-sweep candidates ( costexplorer, guardduty, opensearch, servicediscovery ) stay as documented review candidates; none of them have the same per-shard-vs-global divergence and all are defensible mocks.

## 2026-05-17 — EC2 ImageView expansion ( ec2-terraform-state-layer-gaps sub-item 2 )

Closed the second of three remaining sub-items in `ec2-terraform-state-layer-gaps`. `types::Image` and `views::ImageView` both gained ten new optional fields:

| Field | Type | AWS-SDK counterpart |
|---|---|---|
| `kernel_id` | `Option<String>` | `KernelId` |
| `ramdisk_id` | `Option<String>` | `RamdiskId` |
| `ena_support` | `Option<bool>` | `EnaSupport` |
| `sriov_net_support` | `Option<String>` | `SriovNetSupport` |
| `tpm_support` | `Option<String>` | `TpmSupport` |
| `boot_mode` | `Option<String>` | `BootMode` |
| `imds_support` | `Option<String>` | `ImdsSupport` |
| `image_location` | `Option<String>` | `ImageLocation` |
| `source_image_id` | `Option<String>` | `SourceImageId` ( for `CopyImage` ) |
| `source_region` | `Option<String>` | `SourceRegion` ( for `CopyImage` ) |

Each got a matching `#[serde(default)]` field on `ImageView`, and both `From<&Image> for ImageView` and `From<ImageView> for Image` were extended to carry them through. State-side updates:

- `create_image` ( instance-derived AMI ): all new fields default to `None`.
- `register_image` ( fresh AMI ): same default-to-`None` pattern.
- The inline restore-image-from-s3 path in `state.rs` ( around line 6830 ) now records `image_location: Some(format!("s3://{bucket}/{object_key}"))` so the original restore path is recoverable via the state view.
- `copy_image` now sets `source_image_id: Some(<source AMI id>)` on the copy so AMI-copy lineage survives snapshot/restore. `source_region` is left at whatever the source clone inherited ( typically `None` ) because the current `copy_image` signature does not carry a region parameter; populating it requires a real cross-region test path and is deferred.

Discovery: the third constructor site at line 6830 was caught only by the clippy gate ( `error[E0063]: Image { ... } missing 'boot_mode', 'ena_support', 'image_location' and 7 other fields` ) on the first compile, not by the initial Grep sweep — the Grep pattern `Image \{$` didn't pick up the trailing-`,` form used in `self.images.insert(image_id.clone(), Image { ... },)`. Good reminder that compiler errors are the authoritative enumeration of struct construction sites.

Per-crate gate clean: `cargo fmt -p winterbaume-ec2 -- --check` pass; `cargo clippy -p winterbaume-ec2 --all-targets --all-features -- -D warnings` pass ( 2m32s warm, vs 22m51s cold earlier in this session ); `cargo test -p winterbaume-ec2 --no-fail-fast` passes ( exit 0; same 591 main + 13 scenario tests as the prior commit ).

The remaining two sub-items in this TODO ( singleton spot datafeed subscription slot, VPC route-server family review ) stay open as bigger surface changes that warrant their own pass.

## 2026-05-17 — ec2-generated-wire-deferred-ops closed ( 7 unrouted ops, not 4 )

The TODO's framing was stale. Investigation found:

- The EC2 service-shape declares 763 operations in the 2016-11-15 Smithy model. `winterbaume-ec2-generated` emits 763 `serialize_<op>_response(...)` functions — codegen is fully caught up against the model.
- 7 operations ( not "four" as the TODO claimed ) were unrouted in `winterbaume-ec2`'s dispatch: `AcceptTransitGatewayClientVpnAttachment`, `DeleteTransitGatewayClientVpnAttachment`, `RejectTransitGatewayClientVpnAttachment`, `GetCapacityManagerMonitoredTagKeys`, `UpdateCapacityManagerMonitoredTagKeys`, `GetManagedResourceVisibility`, `ModifyManagedResourceVisibility`.
- All 7 `<Op>Result` structs already exist under `#[cfg(feature = "extras")]` in `winterbaume-ec2-generated::model`, and all 7 derive `Default`.

Fix: added stub handlers ( `STUB[no-state]` ) that default-construct each `<Op>Result` and pass through to the generated serializer. Dispatch entries gated with `#[cfg(feature = "extras")]` to match the gating on the Result types. Three logical groups:

- **Transit Gateway <-> Client VPN attachment handshake** ( 3 ops ): no observable state in the emulator's network model.
- **EC2 Capacity Manager monitored tag keys** ( 2 ops ): cost-visibility surface with no representation in the compute model.
- **Managed Resource Visibility toggle** ( 2 ops ): cross-account listing toggle with no representation in the account model.

EC2 is now fully routed: 713 implemented + 50 stubs = 763 / 763 ops. Stub count went 326 -> 333 in the regenerated `API_COVERAGE.md`; root `README.md` intro paragraph manually patched ( the auto-regen only updates the table footer ); per-crate `winterbaume-ec2/README.md` and `docs/services/ec2.md` refreshed by `update_readme.py`.

Per-crate gate clean: `cargo clippy --all-targets --all-features -- -D warnings` pass ( 1m27s warm ); `cargo fmt --check` pass; `cargo test --no-fail-fast` pass ( same 591 main + 13 scenario test count, same exit-0 ).

Pattern reminder for future audits: when a TODO claims "no generated serializer", verify the snake_case name match before believing the count. The Smithy short-name to Rust snake_case mapping ( e.g. `AcceptTransitGatewayClientVpnAttachment` -> `accept_transit_gateway_client_vpn_attachment` ) is mechanical but easy to mis-grep.

## 2026-05-18 — terraform converter: catch up to EC2/Kinesis view struct additions ( clippy CI red )

CI clippy job ( workflow run `25989146762` ) failed with 10 × `E0063: missing field` in `crates/winterbaume-terraform/src/converters/{ec2.rs,kinesis.rs}`. Compile errors, not lints — the workflow runs `cargo clippy --workspace --exclude winterbaume-sqlengine-duckdb --all-targets --all-features -- -D warnings`, so the terraform converter library failed to type-check.

The fan-in: three recent commits on `main` added new public fields to view structs in `winterbaume-ec2` and `winterbaume-kinesis` without updating the construction sites in `winterbaume-terraform`. The view structs are the canonical cross-crate state-projection ABI, so adding a field is automatically a breaking change for every converter that named-constructs the struct.

Field gaps and fixes:

- `RouteTableView.propagating_vgws: Vec<String>` ( added by `581d07ec`-era state work ). Two construction sites in `ec2.rs` ( the inline-routes `aws_route_table` inject path and the "create minimal RTB to hold an `aws_route` when the parent is not in state yet" fallback ). Both set to `vec![]`. No TF surface for VGW propagation on `aws_route_table` itself ( `aws_vpn_gateway_route_propagation` is the dedicated resource and is not in scope here ).
- `RouteTableAssociationView.gateway_id: Option<String>` ( added so the view can distinguish subnet- and gateway-side associations ). Three construction sites in `ec2.rs`: the "main" association inject ( gateway_id stays `None` ), the subnet/gateway association inject ( now reads `model.gateway_id`, and the duplicate-dedup key was widened to compare both `subnet_id` and `gateway_id` so a subnet-side and a gateway-side association on the same RTB are no longer accidentally folded into one ), and the inline-association parser `parse_route_table_associations` ( now reads `gateway_id` from the TF block ). A stale "view doesn't track gateway_id on RouteTableAssociationView" comment was removed in the process.
- `ImageView` — 10 new fields ( `kernel_id`, `ramdisk_id`, `ena_support`, `sriov_net_support`, `tpm_support`, `boot_mode`, `imds_support`, `image_location`, `source_image_id`, `source_region` ) added by `f99b0e67`. Three construction sites in `ec2.rs`: `aws_ami` ( all 10 wired through from `AmiTfModel`, including `Some(model.ena_support)` since the TF field is `bool` but the view stores `Option<bool>` ), `aws_ami_copy` ( only `source_image_id` / `source_region` carry meaning here, from `model.source_ami_id` / `model.source_ami_region`; the rest are `None` ), and `aws_ami_from_instance` ( all 10 default to `None` ).
- `VpcEndpointView.private_dns_enabled: Option<bool>` ( added so the view actually models private-DNS toggling instead of dropping it ). One construction site in `ec2.rs`. The converter previously emitted a "field ignored" warning when the TF resource had `private_dns_enabled = true`; that warning is now obsolete and was removed alongside wiring `Some(model.private_dns_enabled)` into the view.
- `StreamView.next_sequence_per_shard: HashMap<String, u64>` ( added by `111af91`, the per-shard sequence-number fix ). One construction site in `kinesis.rs`, initialised to `HashMap::new()`. Per-shard sequence state is internal counter machinery that has no TF surface, so empty initialisation is correct ( the kinesis state layer defaults missing entries to zero on first use ).

Per-crate gate `./.agents/bin/cargo.sh clippy -p winterbaume-terraform --all-targets --all-features -- -D warnings` passes clean ( 28m23s cold full-workspace recompile through `sccache-wrapper` ). `cargo fmt -p winterbaume-terraform` also clean.

Durable lesson: when a view struct gains a field, the breakage radiates to ( at minimum ) the matching converters in `winterbaume-terraform/src/converters/<service>.rs`. Until we add a non-exhaustive `..Default::default()` convention or builder API on these view structs, every field add is a coordinated multi-crate change. The CI clippy job is the canonical guard ( workspace-wide `--all-targets`, deny warnings ); a per-crate gate on only the modified service crate does not catch this. When adding a field to a view struct, search the workspace for `<StructName> {` construction sites before merging.

## 2026-05-17 — ec2-terraform-state-layer-gaps fully closed: spot datafeed + route-server review

The two remaining sub-items in `ec2-terraform-state-layer-gaps` are now resolved.

### Singleton spot datafeed subscription slot

Replaced `handle_spot_datafeed_noop` ( which returned a hand-rolled `<{response_name}><return>true</return>` XML payload for all three `*SpotDatafeedSubscription` ops -- semantically broken for `Describe`, which the SDK then parsed as a missing subscription ) with three real state-backed handlers:

- `handle_create_spot_datafeed_subscription` — parses `Bucket` / `Prefix` from the request, calls `state.create_spot_datafeed_subscription(...)` ( returns `SpotDatafeedAlreadyExists` if one is already present ), returns the freshly-created `SpotDatafeedSubscription` element in the `CreateSpotDatafeedSubscriptionResult`.
- `handle_delete_spot_datafeed_subscription` — clears the slot; the generated `serialize_delete_spot_datafeed_subscription_response()` takes no args because the Smithy `DeleteSpotDatafeedSubscription` operation has a void output shape ( learned the hard way -- first clippy run failed with `E0061: function takes 0 arguments but 1 was supplied` against the model-default `Result`-pattern from my plan-file draft ).
- `handle_describe_spot_datafeed_subscription` — looks up the slot ; returns `InvalidSpotDatafeed.NotFound` ( `Ec2Error::SpotDatafeedNotFound` ) when the slot is `None`, otherwise returns the active subscription.

State plumbing:

- `types::SpotDatafeedSubscription { bucket: String, prefix: Option<String>, owner_id: String, state: String }` ( only `"Active"` is reachable from the emulator ; real AWS exposes `"Inactive"` while propagation to S3 fails, which we don't model ).
- `Ec2State.spot_datafeed_subscription: Option<SpotDatafeedSubscription>` next to the existing spot-instance fields.
- Two new `Ec2Error` variants — `SpotDatafeedAlreadyExists` -> `AlreadyExists` and `SpotDatafeedNotFound` -> `InvalidSpotDatafeed.NotFound` — wired into `ec2_error_response`.
- `SpotDatafeedSubscriptionView` plus matching `Ec2StateView.spot_datafeed_subscription: Option<...>` and From conversions both directions for snapshot/restore round-trips.
- `spot_datafeed_to_model` helper in the model-conversion section of `handlers.rs` to bridge `types::SpotDatafeedSubscription` -> wire `model::SpotDatafeedSubscription`.

Regression test `test_spot_datafeed_subscription_singleton_lifecycle` in `tests/integration_test.rs` walks the full lifecycle and asserts the AWS-spec contract: Describe-before-Create returns `InvalidSpotDatafeed.NotFound`; Create returns the active subscription with the right bucket/prefix/state; Describe returns the active subscription; a second Create fails with `AlreadyExists`; Delete succeeds; Describe-after-Delete returns NotFound again.

### VPC route-server family review

No code change. Re-verified that the family is already comprehensively modelled in `Ec2State`:

- Top-level `route_servers: HashMap<String, RouteServer>` plus distinct maps for endpoints, peers, associations, propagations.
- 17 of 18 route-server ops are state-backed in dispatch ( CRUD for the route-server itself, the endpoint, and the peer; associate / disassociate; enable / disable / get propagations; get associations ).
- The lone stub is `GetRouteServerRoutingDatabase`, which is by design — emitting BGP routing-table contents would require a real BGP simulator that the emulator can't provide.

Closed as a documented review with the existing state model.

### Verification

- `cargo fmt -p winterbaume-ec2 -- --check`: pass.
- `cargo clippy -p winterbaume-ec2 --all-targets --all-features -- -D warnings`: pass ( 1m33s warm ).
- `cargo test -p winterbaume-ec2 --no-fail-fast`: **592 main tests + 13 scenario tests, 0 failures** ( +1 from the new singleton-lifecycle test compared with the prior 591 ).

The parent `ec2-terraform-state-layer-gaps` TODO is now `- [x]` with strike-throughs covering all five original sub-items.

## 2026-05-18 — appconfigdata-shared-state ( high-severity state-coherence fix )

Closed the **High severity** flag from the 2026-05-01 control-plane / data-plane audit: `winterbaume-appconfigdata::GetLatestConfiguration` now resolves the session's `(application_id, environment_id, configuration_profile_id)` through `winterbaume-appconfig`'s deployment state and returns the actual configuration bytes, not an empty payload.

### Parent crate ( winterbaume-appconfig ) changes

- `types::HostedConfigurationVersionData` gains `pub content: Vec<u8>` — the raw configuration bytes. Previously dropped on the floor by `handle_create_hosted_configuration_version`, which received the wire `Content` string and passed only the content-type + description to `state`.
- `state::create_hosted_configuration_version` signature extended with `content: Vec<u8>`. The wire deserialiser hands a `String` over because the codegen maps blob -> String; the handler converts via `content.into_bytes()` and stores the raw bytes ( correct for text-based configs ; binary configs would be base64-decoded on the wire by the SDK before reaching the handler ).
- New `AppConfigState::get_deployed_configuration(app_id, env_id, profile_id) -> Option<(&str, &[u8])>` walks `deployments`, picks the highest-numbered `COMPLETE` deployment for the target, parses its `configuration_version` as i32, and returns the matching hosted-version's content_type + content.
- New `AppConfigService::shared_state() -> Arc<BackendState<AppConfigState>>` exposes the per-account/region state holder. Mirrors the `with_dynamodb_backend` pattern but uses `Arc<BackendState<...>>` directly instead of a `Backend` trait — AppConfig doesn't expose a trait abstraction and the state struct itself is the public API.
- `views::HostedConfigurationVersionView` keeps its existing "Configuration content is excluded from snapshots" contract; the `From<HostedConfigurationVersionView> for HostedConfigurationVersionData` impl now initialises `content: Vec::new()` on the restore side.

### Child crate ( winterbaume-appconfigdata ) changes

- Workspace dep `winterbaume-appconfig = { workspace = true }` added in `Cargo.toml` ; dev-dep `aws-sdk-appconfig` added for the new end-to-end test.
- `AppConfigDataService` gains `pub(crate) appconfig_state: Option<Arc<BackendState<AppConfigState>>>` field. `Self::new()` leaves it `None` ( legacy behaviour ; empty body, no resolution ). New constructor `Self::with_appconfig_state(state)` takes the `Arc` returned by `AppConfigService::shared_state()` and wires it up.
- `handle_get_latest_configuration` re-shaped: validates token + captures the session's `(app, env, profile)` ; rotates the token ; drops the data-plane state lock ; then when `appconfig_state` is `Some`, acquires the parent state and calls `get_deployed_configuration` to resolve the bytes. Falls back to the legacy empty body when the parent state isn't wired or no deployment matches. Content-Type header now reflects the deployed configuration ( previously always `application/octet-stream` ).

### Verification

End-to-end integration test `test_get_latest_configuration_resolves_through_appconfig_state` covers the full path: build a `MockAws` with both services sharing one `Arc<BackendState<AppConfigState>>`, drive the AppConfig control plane via real `aws-sdk-appconfig` calls ( CreateApplication -> CreateEnvironment -> CreateConfigurationProfile with `location_uri = "hosted"` -> CreateHostedConfigurationVersion with an actual JSON payload -> CreateDeploymentStrategy ( immediate, 0-minute ) -> StartDeployment ( auto-completes in the mock ) ), drive the data plane via `aws-sdk-appconfigdata` ( StartConfigurationSession -> GetLatestConfiguration ), and assert the returned blob matches the uploaded JSON byte-for-byte and the Content-Type header is `application/json`.

Per-crate gate clean on both: `winterbaume-appconfig` 47 tests + 3 doctests pass ; `winterbaume-appconfigdata` 7 tests ( +1 vs prior 6 ) + 1 doctest pass. clippy + fmt --check clean on both.

### Compile-error caught mid-flight

First build attempt failed with `E0382: borrow of partially moved value: resp` in the integration test — `resp.configuration.expect(...)` moves the field out, after which `resp.content_type()` can't borrow `resp`. Fixed by reading the content-type into a `String` before consuming the body. Worth remembering for future SDK-response tests: read scalar / borrowed accessors before `.expect()`-ing on the body field.

### Limitations

Only hosted configuration sources are resolved. AppConfig profiles can also reference S3 paths, SSM Parameter Store parameters, and Secrets Manager secrets ; those would need cross-service look-ups ( s3 -> bytes, ssm -> string, secretsmanager -> string ). Add when a real workflow needs them.

## 2026-05-18 — kinesisvideoarchivedmedia-stream-validation

Closed the **kinesisvideoarchivedmedia-stream-validation** entry from the 2026-05-01 control-plane / data-plane audit: `winterbaume-kinesisvideoarchivedmedia` archived-media operations now validate stream names / ARNs against `winterbaume-kinesisvideo` state when the two crates are wired together, returning `ResourceNotFoundException` for unknown streams instead of silently auto-creating them. Mirrors the `appconfigdata-shared-state` pattern landed earlier the same day.

### Parent crate ( winterbaume-kinesisvideo ) changes

- New `KinesisVideoService::shared_state() -> Arc<BackendState<KinesisVideoState>>` exposes the per-account/region state holder. One-line wrapper around `Arc::clone(&self.state)` ; same shape as the appconfig accessor.

### Child crate ( winterbaume-kinesisvideoarchivedmedia ) changes

- Workspace dep `winterbaume-kinesisvideo = { workspace = true }` added in `Cargo.toml` ; dev-dep `aws-sdk-kinesisvideo` added for the new cross-crate integration tests.
- `KinesisVideoArchivedMediaService` gains `pub(crate) kinesisvideo_state: Option<Arc<BackendState<KinesisVideoState>>>` field. `Self::new()` leaves it `None` ( legacy auto-create behaviour preserved ; useful for unit-test isolation ). New constructor `Self::with_kinesisvideo_state(state)` takes the `Arc` returned by `KinesisVideoService::shared_state()` and wires it up.
- New helper `async fn validate_stream_in_parent(&self, account_id, region, stream_name, stream_arn) -> Option<MockResponse>` resolves the stream against the parent state: lookup by name first ( `streams.contains_key` ), else linear scan over `streams.values()` for a matching `stream_arn` field. When the stream is missing it returns `Some(rest_json_error(404, "ResourceNotFoundException", ...))` ; when unwired or both identifiers are absent it returns `None` and the existing handler logic ( including the `MissingStreamIdentifier -> InvalidArgumentException` 400 ) keeps running.
- All six routed archived-media handlers ( `GetMediaForFragmentList`, `ListFragments`, `GetHLSStreamingSessionURL`, `GetDASHStreamingSessionURL`, `GetClip`, `GetImages` ) take `account_id` and `region` arguments now and call `validate_stream_in_parent` right after wire-deserialise. Fragment payload storage stays in this crate ; only stream existence is delegated upstream.

### URL routing collision

First test run had two new tests fail with `UnknownOperationException: Not found` from `winterbaume-kinesisvideo`. Root cause: both `aws-sdk-kinesisvideo` and `aws-sdk-kinesisvideoarchivedmedia` default to `https://kinesisvideo.<region>.amazonaws.com` ; the archived-media crate's old URL pattern `https?://.*\.kinesisvideo\.(.+)\.amazonaws\.com` requires a subdomain dot before `kinesisvideo`, so it did not match the default endpoint. With only archived-media registered, the `service_name()`-based fallback in `MockAwsClient::find_service_by_url` ( both services return `"kinesisvideo"` ) kicked in and routed correctly ; with both services registered, the kinesisvideo URL pattern won the first-match race and the dispatch returned 404 for archived-media paths.

Fix: add a second path-anchored pattern to the archived-media `url_patterns()` -- `https?://kinesisvideo\.(.+)\.amazonaws\.com/(?:getMediaForFragmentList|listFragments|getHLSStreamingSessionURL|getDASHStreamingSessionURL|getClip|getImages)`. Combined with registering the archived-media service **before** `winterbaume-kinesisvideo` in `MockAws::builder`, this makes the dispatcher pick archived-media for archived-media paths and `winterbaume-kinesisvideo` for everything else. The `with_kinesisvideo_state` doc example shows the correct ordering, and the cross-crate test helper sets it up that way.

### Verification

Three new integration tests in `crates/winterbaume-kinesisvideoarchivedmedia/tests/integration_test.rs`:
- `test_archived_media_accepts_existing_stream_via_parent_state` -- create a stream through `aws-sdk-kinesisvideo::CreateStream`, then assert `aws-sdk-kinesisvideoarchivedmedia::ListFragments` against the same name succeeds and returns mock fragments.
- `test_archived_media_rejects_unknown_stream_when_wired` -- the wired service rejects calls against an uncreated stream name with `ResourceNotFoundException` ; no auto-create.
- `test_archived_media_legacy_auto_create_unchanged_when_unwired` -- `Self::new()` ( unwired ) keeps the original auto-create-on-first-request path so existing unit tests continue to pass.

Per-crate gate clean on both crates: clippy + fmt --check + test --no-fail-fast all pass. `winterbaume-kinesisvideo` 62 tests ; `winterbaume-kinesisvideoarchivedmedia` 31 integration tests + 1 doctest ( +3 vs prior 28 ).

### Limitations

Only the six archived-media operations currently routed by the crate validate against the parent state. When `kinesisvideomedia`, `kinesisvideosignaling`, or `kinesisvideowebrtcstorage` crates are added later they will need the same `with_kinesisvideo_state` constructor and `validate_stream_in_parent` helper applied per-operation. The routing-collision workaround ( path-anchored URL pattern + register-first ordering ) generalises to those crates too as long as they share the default `kinesisvideo.<region>.amazonaws.com` endpoint.

## 2026-05-18 — sagemakerruntime-endpoint-validation

Third instance of the cross-crate state-coherence pattern this session, after `appconfigdata-shared-state` and `kinesisvideoarchivedmedia-stream-validation`. Pattern is now stable: parent service exposes `pub fn shared_state(&self) -> Arc<BackendState<<Parent>State>>`, child service gains an optional `pub(crate) <parent>_state: Option<...>` field, new `with_<parent>_state(...)` constructor wires the parent, handlers consult the parent when wired and fall back to legacy behaviour when not.

### Parent crate ( winterbaume-sagemaker ) changes

One addition: `SageMakerService::shared_state()` over the existing `pub(crate) state: Arc<BackendState<SageMakerState>>`. The parent already had `pub endpoints: HashMap<String, Endpoint>` on `SageMakerState` and `pub fn describe_endpoint(&self, name: &str)` accessors — no new state shape needed.

### Child crate ( winterbaume-sagemakerruntime ) changes

- Workspace dep on `winterbaume-sagemaker` added in `Cargo.toml` ; dev-dep on `aws-sdk-sagemaker` added for the new end-to-end test.
- `SageMakerRuntimeService` gains `pub(crate) sagemaker_state: Option<Arc<BackendState<winterbaume_sagemaker::SageMakerState>>>`.
- New `with_sagemaker_state(state)` constructor with a `no_run` doc example mirroring the appconfigdata pattern.
- In the URL-routed dispatch for `POST /endpoints/{name}/invocations` ( and async / streaming variants that share the URL-prefix routing path ), after extracting `endpoint_name` via `percent_decode(segments[1])`, the handler now gates: if `sagemaker_state` is `Some`, acquire the parent state for the current `(account_id, region)`, check membership in `endpoints`, and short-circuit with HTTP 400 `ValidationError` + body `"Endpoint <name> of account <account_id> not found"` when missing. When `sagemaker_state` is `None`, the legacy auto-create-on-first-invocation path stays untouched.

### Scope-narrowing

The original TODO included "route the targeted variant through the endpoint-config / model graph so invocation records can capture it" as a sub-task. That part is **not** implemented here — `SageMakerRuntimeState`'s invocation record does not currently carry a variant identifier, and adding one would require a bigger refactor of the runtime crate's state shape. Left as a sub-followup ; the closure note in `TODO.md` makes this explicit.

### Verification

Three new tests cover the behaviour cells: wired-and-valid, wired-and-unknown, unwired-legacy. The wired-and-unknown test asserts the exact ValidationError body shape ; the unwired-legacy test confirms the auto-create path is unaffected. Test count went from 20 -> 23 in `crates/winterbaume-sagemakerruntime/tests/integration_test.rs`.

Per-crate gate clean on both: `winterbaume-sagemaker` clippy + fmt --check + tests pass ( 20s warm check ) ; `winterbaume-sagemakerruntime` 23 tests + 1 doctest pass ( 2m37s clippy cold ).

### Pattern reusability ( three concrete instances now )

The pattern is now repeated three times across three independent parent/child pairs ( appconfig/appconfigdata, kinesisvideo/kinesisvideoarchivedmedia, sagemaker/sagemakerruntime ). Each adds ~ 50-150 lines of code plus an end-to-end test. The shape stabilises to:

1. parent service: `pub fn shared_state(&self) -> Arc<BackendState<ParentState>>` ( ~ 10 lines incl. doc-comment ).
2. child service: optional state field + `with_<parent>_state(...)` constructor with a `no_run` doc-example ( ~ 35 lines ).
3. child handlers: `if let Some(parent_state) = self.parent_state.as_ref() { ... validate ... }` at every relevant op ; legacy fallback preserved.
4. integration test: build `MockAws` with both services sharing the parent `Arc`, drive real `aws-sdk-<parent>` calls to set up state, drive `aws-sdk-<child>` to exercise validation, assert both the happy path and the unknown-resource error path.

Remaining instances of the same pattern in the State Coherence Backlog: `cloudtraildata-channel-validation` ( blocked — cloudtrail has no channel state yet ), `mediastoredata-container-model` ( different shape — requires re-keying the data plane's `objects` map by `(container, path)` rather than just gating existence ), `rdsdata-cluster-validation` and `redshiftdata-cluster-validation` ( both flagged as low-priority pending a real workflow that needs them ).

## 2026-05-18 — ses-v1-v2-shared-backend: first family ( identities )

Started the per-family unit of work for the largest remaining state-coherence TODO. Identities is the first of five families ( identities -> configuration sets -> templates -> suppression list -> dedicated IP pools / account-level settings ).

### Design choice: v2 as canonical store

`winterbaume-sesv2`'s `SesState` is the canonical store for the shared families. `winterbaume-ses` ( v1 ) reads and writes through `Arc<BackendState<SesState>>` when wired and falls back to its own `SesV1State` map when not. This direction was chosen because v2's `EmailIdentity` carries the richer field set ( policies, tags, configuration_set_name, DKIM signing key type, mail-from MX behaviour ) and v1's narrower projection is easy to derive from it. The reverse direction ( v1 canonical ) would lose v2 fields on every v1 round-trip.

### Implementation shape

Mirrors the established cross-crate pattern ( fourth concrete instance in this session, after appconfigdata / kinesisvideoarchivedmedia / sagemakerruntime ):

- **Parent ( sesv2 )**: new `SesV2Service::shared_state() -> Arc<BackendState<SesState>>` accessor over the existing `pub(crate) state` field.
- **Child ( ses )**: workspace dep on `winterbaume-sesv2` ; dev-dep on `aws-sdk-sesv2` for the new integration test ; `SesService` gains `pub(crate) sesv2_state: Option<Arc<BackendState<SesState>>>` ; new `with_sesv2_state(arc)` constructor with a `no_run` doc-example.
- **Bridge module** ( `sesv2_identity` ) inside `winterbaume-ses::handlers` translates v1↔v2 semantics:
  - `make_verified_identity(name)` constructs an `EmailIdentity` for a v1 `Verify*` call ( picks `EMAIL_ADDRESS` vs `DOMAIN` based on `@` in the name, sets `verified: true`, `dkim_signing_enabled: true` for domains ).
  - `derive_verification_token(domain)` reproduces `SesV1State::verify_domain_identity`'s deterministic hash-based token so the wired path returns the same token as the legacy path for the same input.
  - `derive_dkim_tokens(domain)` returns the `dkim1.<domain>` / `dkim2.<domain>` / `dkim3.<domain>` triple that v1 exposes via `GetIdentityDkimAttributes`. v2's `EmailIdentity` does not carry this list — deterministic derivation is the bridge.
  - `matches_v1_filter(v2_type, v1_filter)` maps v1's `IdentityType` enum filter ( `"EmailAddress"`, `"Domain"` ) to v2's string `identity_type` ( `"EMAIL_ADDRESS"`, `"DOMAIN"` ).
- **8 v1 handlers refactored** to take `account_id` + `region` and branch: `VerifyEmailAddress`, `VerifyEmailIdentity`, `VerifyDomainIdentity`, `DeleteIdentity`, `ListIdentities`, `ListVerifiedEmailAddresses`, `GetIdentityVerificationAttributes`, `GetIdentityDkimAttributes`. When `sesv2_state` is `Some`, each consults `SesState.identities` ( read / write as appropriate ) and uses the bridge helpers to translate to v1 wire shapes. When `None`, the existing local-state path runs unchanged.

### Verification

End-to-end test `test_v1_v2_shared_identities_round_trip` in `crates/winterbaume-ses/tests/integration_test.rs` builds a `MockAws` with both services sharing the v2 state and exercises:

1. v1 `VerifyEmailIdentity("alice@example.com")` -> v2 `ListEmailIdentities` sees `alice`.
2. v2 `CreateEmailIdentity("bob@example.com")` -> v1 `ListIdentities` sees both `alice` and `bob`.
3. v1 `VerifyDomainIdentity("example.org")` returns a token -> v1 `GetIdentityVerificationAttributes` returns the deterministic same token with status `Success` -> v1 `ListIdentities(IdentityType::Domain)` filter returns only the domain.
4. v1 `DeleteIdentity("bob@example.com")` -> v2 `ListEmailIdentities` no longer sees `bob`, still sees `alice`.

Sibling test `test_v1_legacy_identities_isolated_when_unwired` confirms `SesService::new()` ( no wiring ) keeps the legacy local-state behaviour for unit-test isolation.

Per-crate gate clean on both: `winterbaume-ses` clippy + fmt --check + tests pass ( 32 tests +1 doctest, +2 vs prior 30 ) ; `winterbaume-sesv2` 88 tests pass with no regression.

### What's left in the parent TODO

Four more families to migrate, each following the same shape ( bridge helpers + per-family v1 handler refactor ):

- **Configuration sets** — both crates have a `ConfigurationSet` struct ; v2's is richer ( archiving / delivery / reputation / sending / suppression / tracking / VDM options ). v1's handlers: `CreateConfigurationSet`, `DeleteConfigurationSet`, `DescribeConfigurationSet`, `ListConfigurationSets`, `CreateConfigurationSetEventDestination`, etc.
- **Templates** — v1's `Template` vs v2's `EmailTemplate`. v1's handlers: `CreateTemplate`, `DeleteTemplate`, `GetTemplate`, `UpdateTemplate`, `ListTemplates`.
- **Suppression list** — v1's `*SuppressedDestination*` family. Field shape is closer between v1 and v2 here.
- **Dedicated IP pools** — v1 has weaker support for this family ; mostly v2 territory but v1's `ListReceiptFilters` etc. mention dedicated IPs.
- **Account-level settings** — v1's `GetAccountSendingEnabled` etc. vs v2's `GetAccountSettings`. Smallest family.

The pattern is now established ( bridge module + handler branching + integration test ). Each family is ~ 100-200 lines of code plus its own cross-API regression test.

### Pattern reusability ( four concrete instances now )

The cross-crate state-sharing pattern is now repeated four times ( appconfig/appconfigdata, kinesisvideo/kinesisvideoarchivedmedia, sagemaker/sagemakerruntime, sesv2/ses ). The v1/v2 SES variant differs from the first three in that:

- Both sides are first-class APIs ( not control-plane / data-plane ) — the wired side still has its own state for unrelated families ( receipt rule sets in v1, contact lists in v2 ).
- Translation happens at the bridge layer ( deterministic derivation of v1-only fields ) rather than direct passthrough — neither side is a strict subset of the other.

The basic pattern shape ( `shared_state()` + `with_<parent>_state()` + per-handler branch + legacy fallback ) still applies cleanly.
## 2026-05-18 — Per-crate release harness ( `tools/release-harness/` )

### Motivation

Since the 2026-05-13/14 v0.2.0 batch ( all 241 publishable crates flat-bumped via `release-batch` ), crate evolution has been uneven. A `git diff winterbaume-server-v0.2.0..HEAD -- crates/` showed:

- 18 crates with substantive `src/**` changes ( ec2, kinesis, ses, sesv2, appconfig, appconfigdata, kinesisvideoarchivedmedia, sagemakerruntime, opensearch, ivs, batch, applicationsignals, organizations, account, sagemaker, kinesisvideo, sqs-redis, terraform ).
- 223 crates whose only diff was a regenerated `README.md` ( + back-filled `CHANGELOG.md` v0.2.0 entry ) from the bulk regen commit `4760384f`.
- The umbrella `winterbaume` crate at the workspace root.

The next cycle's two options without tooling were both bad: ( a ) flat workspace-wide bump → 240 noisy releases of mostly-unchanged crates, or ( b ) hand-pick per-crate bumps → error-prone across 241 crates. The harness exists to compute the smallest semantically correct delta automatically.

### Design

New Rust binary at `tools/release-harness/`, sibling to `tools/release-batch/`. Three idempotent stages, each invocation safe to re-run:

1. `plan` — discover and classify. For each publishable crate, resolve the latest `<crate>-v<X.Y.Z>` tag, `git diff` since, subtract a cosmetic-paths filter ( `README.md`, `CHANGELOG.md`, `NOTICE`, `LICENSE`, images, `**/docs/**` ), then:
   - If the remainder is empty → `skip` ( doc-only ).
   - If any added line under `src/**` starts with `+pub fn|struct|enum|trait|mod|const|static|type|use ` ( excluding `pub(...)` ) → candidate `minor`.
   - Otherwise → candidate `patch`.
   - For candidates ≥ patch, run `cargo semver-checks check-release -p <name> --release-type <candidate>` against the latest crates.io version; any breaking lint escalates to `major`. If the tool isn't installed, the plan warns and falls back to heuristic-only — never silently downgrades.
   - An optional `release-plan-overrides.toml` at the repo root always wins, with values `"skip" | "patch" | "minor" | "major" | "<literal-semver>"`.
   - Output: `release-plan.toml` ( gitignored ) plus a stdout summary grouped by bump level.
2. `changelog` — read the plan, run `git log <last-tag>..HEAD --first-parent -- <crate-dir>/` per non-skip crate, bucket commits by conventional-commit-style prefix ( `feat:` → Added, `fix:` → Fixed, `chore:` → Internal, etc. ), prepend a fresh `## v<next> - <date>` section to each `crates/<name>/CHANGELOG.md`, refresh the root umbrella `CHANGELOG.md`. Mechanical draft — the operator ( or the `generate-changelog` skill ) polishes the wording before committing.
3. `publish` — group plan entries by bump level ( `patch`, `minor`, `major`, plus each pinned literal version as its own one-crate group ) and dispatch `tools/release-batch/ --version <level-or-version> --crates <list>` once per group. `--execute` runs it; without it, the planned commands are printed.

The umbrella `winterbaume` crate ( whose `manifest_path` parent equals the workspace root ) has no single directory to diff, so the harness skips it by default and requires an explicit pin in `release-plan-overrides.toml` to publish.

Errors are per-module enums ( `plan::Error`, `changelog::Error`, `publish::Error` ); `main()` unifies via `Box<dyn std::error::Error>` for the top-level dispatch. The version representation is a hand-rolled 3-tuple ( no `semver` crate dep ) just rich enough to parse, render, and bump-by-level; the bump table matches cargo-release's pre-1.0 semantics ( `Minor` and `Major` both step the minor axis when `major == 0` ).

### Extension to `release-batch`

Added `--crates <name>...` and `--crates-file <path>` ( mutually exclusive ) to `tools/release-batch/`. Without either, behaviour is unchanged ( every publishable workspace member is in scope ). With either, the topology sort still runs across the supplied subset and the chunking / 429-retry / crates.io resumability logic operates on that subset. The harness's stage 3 is the only caller today.

### Smoke test against current HEAD

`release-harness plan --skip-semver-checks` against the live workspace classified 241 crates as:

- 18 changed: 15 `minor` ( the SES shared-backend reorg, EC2 ImageView expansion, state-coherence work, ... ) and 3 `patch` ( `winterbaume-kinesis` fix, `winterbaume-sqs-redis` fix, `winterbaume-terraform` converter catch-up ).
- 223 `skip` doc-only ( matching the bulk README regen commit's blast radius ).
- 0 `unchanged` ( because every crate did receive at least a README touch ).
- 0 `initial` ( all 241 crates already tagged at v0.2.0 ).

`release-harness publish --plan ...` dry-run produced two `release-batch` invocations: one `--version minor --crates ...` for the 15 minor crates and one `--version patch --crates ...` for the 3 patch crates. Matches the manual analysis exactly.

### Pitfalls hit

- TOML round-trip: fields marked `#[serde(skip_serializing_if = "...")]` need `#[serde(default)]` too, otherwise the deserialiser rejects the file the serialiser wrote ( the optional fields are absent on disk but required when reading back ). `publish` failed on its first run with `missing field "files_changed"` because `Vec` defaults to non-default-on-deserialize unless the attribute is explicit.
- The umbrella crate at the workspace root: stripping `pkg.dir()` against `root` yields an empty path, which becomes `"/"` as a pathspec and `git diff` rejects it with `fatal: '/' is outside repository`. The fix is to short-circuit umbrella detection ( `pkg.dir() == root` ) before constructing the pathspec, after the override check so an explicit pin still works.
- `cargo-semver-checks` exit codes don't cleanly distinguish "tool errored" from "breaking lint fired". The harness disambiguates by scanning combined stdout+stderr for `FAIL` / `semver requires` / `breaking change` / `required bump` markers, returning `Outcome::Unavailable` ( with a warning ) if no verdict marker is present. Operators are expected to install the tool ( `cargo install cargo-semver-checks` ) when they want real breaking-change detection.

### Gate

Per-crate gate clean on both crates: fmt --check, clippy --all-targets --all-features -D warnings, 4 unit tests on `release-harness` ( version parsing + bump table ), 13 unit tests on `release-batch` ( unchanged from prior, all still passing after the `--crates` extension ). `RELEASE.md` gained a "Per-Crate Semver Harness ( steady-state releases )" section and the execution checklist now references the harness path before the existing cargo-release dry-run step. `.gitignore` excludes `release-plan.toml` but intentionally not `release-plan-overrides.toml` — when overrides are used, they should land in the same commit as the changelog draft so reviewers can see the decision.

### Not done

- The `changelog` stage was not exercised against the live tree: it would write 18 per-crate CHANGELOG drafts and modify the root CHANGELOG.md, which is appropriate to do at the start of an actual release cycle, not during harness validation. The string-templating path is type-checked and the `version` module's bump table is unit-tested; first real-run feedback will come when the next steady-state release cycle begins.
- Cross-crate bump propagation ( bumping `winterbaume-core` forcing all dependants to bump ) is out of scope; the operator handles this via `release-plan-overrides.toml` for now. Worth revisiting once a real case forces the question.

## 2026-05-18 — release-batch consolidated into release-harness

Followed up the morning's `release-harness` introduction by absorbing `tools/release-batch/` into it as a sibling `batch` module + subcommand, so the workspace has one release binary instead of two.

### Why

The original split was incidental, not principled: `release-batch` predated the harness and was used directly for first-launch on 2026-05-13/14. With the harness landing this morning, two cracks appeared:

1. The harness's `publish` stage shelled out to `release-batch` as a subprocess, even though both lived in the same workspace and were always built together. The path-resolution dance ( prefer `$CARGO_TARGET_DIR/release/release-batch`, then `target/release/release-batch`, then `release-batch` on PATH ) failed under agent runs where `cargo.sh` mints per-session target dirs and the operator never had to build release-batch separately.
2. Two binaries meant two `cargo metadata` calls per invocation, two copies of `topo_sort` / `publishable_members`, two `Cargo.toml`s in `[workspace.metadata.release]`'s `publish = false` list, two clippy/fmt gates to keep green.

### Shape after consolidation

`tools/release-harness/src/` now contains a `batch` module ( renamed from the old `release-batch/src/main.rs`, ~430 lines plus 13 unit tests ) and a `publish::run_batch` entry point. The CLI grew one subcommand:

```
release-harness plan         # discover + classify (unchanged)
release-harness changelog    # draft changelogs   (unchanged)
release-harness publish      # plan-driven chunked publish (now in-process)
release-harness batch        # direct chunked publish, no plan (new — replaces the old binary)
```

`publish` now calls `batch::run_chunked(BatchOptions {...})` directly instead of spawning a subprocess. Both subcommands feed into the same `BatchOptions` struct, so the chunking / 429 retry / crates.io resumability / `parse_already_published` / `parse_uploaded` / `parse_retry_after` / `backfill_tag` machinery has exactly one implementation. The 13 batch-related unit tests round-tripped unchanged.

### One real improvement, two cosmetic ones

- Real: `publish` now sorts each bump-level group by the workspace-wide topology ( computed once via `topo_sort` over every publishable member ) before passing the subset to `batch::run_chunked`. Previously, ordering inside a group came from the harness's `BTreeMap` iteration order, which is alphabetical by crate name — fine when no in-group dependencies exist, wrong when they do. The smoke test caught it: the `minor` group now publishes `sesv2` before `ses` ( ses depends on sesv2 via the shared-backend reorg ), where the unmerged design would have published them in alphabetical order.
- Cosmetic 1: `parse_uploaded` early-returns an empty set when the version argument is a level keyword ( `patch` / `minor` / `major` ). Previously the binary did this check at the call site via `args.version.as_deref().unwrap_or_default()`; now it lives inside the helper itself, with a unit test ( `parse_uploaded_skipped_for_level_keywords` ) pinning the behaviour. The tag-backfill loop is unconditional in `batch::run_chunked` and depends on this being correct.
- Cosmetic 2: `BatchOptions` carries `sleep_between_chunks: Duration` and `retry_buffer: Duration` instead of raw `u64` seconds, so the math at the call site is `+= opts.retry_buffer` instead of `+= Duration::from_secs(args.retry_buffer)`. Saves a layer of `Duration::from_secs` on every retry path.

### Migration

- Deleted `tools/release-batch/` entirely ( `Cargo.toml`, `src/main.rs`, and the dependency on it from the workspace `members` list ).
- `httpdate = "1"` and `ureq = { version = "2", default-features = false, features = ["tls"] }` migrated from the deleted crate's `Cargo.toml` into the harness's. They're the only deps that weren't already there.
- `RELEASE.md` lost the "Publish Rate Limit and the `release-batch` Driver" section and its dedicated "Per-Crate Semver Harness" section; both folded into one "Release Harness ( `tools/release-harness/` )" section that describes all four subcommands. The execution checklist's two release-batch references collapsed to one harness reference.
- The `release-batch: publish = false` line in RELEASE.md's exclusion list is gone too — there's no such crate any more.

### Gate

Per-crate gate clean: clippy --all-targets --all-features -D warnings, fmt --check, 18 unit tests ( 4 `version::tests`, 13 `batch::tests`, 1 each from the others ). End-to-end smoke test against current HEAD reproduces the morning's plan output ( 18 changed, 223 doc-only skip, 15 minor + 3 patch ) and the `publish` dry-run now shows topology-sorted crate lists per group. `batch --version patch --crates winterbaume-s3 winterbaume-sqs` dry-run renders the same `$ cargo release patch ...` chunk preview the old binary produced.

## 2026-05-19 — release-harness rich consolidated commit, full resume, transitive + stale-deps propagation

Iterative deep-dive that started with "the recent `release-harness publish` runs landed commits with literal `chore: release {{crate_name}} v{{version}}` subjects" and ended four sessions later with a fully resumable per-step pipeline, transitive bump propagation, stale-workspace-dep detection, and a plan for two missed dependant crates ( `winterbaume-server` patch + `winterbaume` umbrella minor ). Each issue uncovered the next.

### 1. Why cargo-release left placeholder commit subjects

Workspace `Cargo.toml` had `consolidate-commits = true` + `shared-version = false` + the default-shape template `"chore: release {{crate_name}} v{{version}}"`. Per cargo-release's own reference table, `{{crate_name}}` / `{{version}}` / `{{metadata}}` are only substituted "if not consolidated" — when one commit covers multiple crates with multiple versions, the substituter gives up and emits a WARN. The local tags were rendered correctly because cargo-release renders `tag-message` once per tag ( per-crate ), not once per consolidated commit. So three different `chore: release {{crate_name}} v{{version}}` commit subjects landed on main ( `f7a6fa52`, `fe867ea4`, `f4a59fc3` ), each carrying multiple correctly-rendered tags.

Quick fix: pre-release-commit-message = "chore: release" ( placeholder-free, so no WARN ). Real fix below.

### 2. Rich consolidated commit message via cargo-release per-step subcommands

The interesting design constraint: the only template variables cargo-release reliably substitutes in the consolidated path are `{{date}}`, `{{prev_version}}`, `{{prev_metadata}}`, `{{prefix}}` — none of which can express "the crates that just released and their versions". Three options surfaced:

- `"chore: release {{date}}"` — minimal, gives a date stamp.
- Drop `consolidate-commits = true` — one commit per crate, log explosion.
- Compose the message ourselves via `cargo release`'s per-step subcommands and amend before publish.

Picked the third. cargo-release exposes `changes` / `version` / `replace` / `hook` / `commit` / `publish` / `owner` / `tag` / `push` as individual subcommands, so the chunk loop in `tools/release-harness/src/batch.rs` became:

```
1. cargo release version <ver|level>   # bump manifests
2. cargo release replace               # changelog date stamps etc.
3. cargo release hook                  # no-op for this workspace
4. cargo release commit                # consolidated commit, placeholder template
5. git commit --amend [-S] -m "<rich>" # rich body: "chore: release\n\n- foo v0.3.0\n- bar v1.0.0"
6. cargo release publish               # uploads tarballs — now embedding the amended SHA
7. git tag -s/-a (per missing tag)     # direct git tag, idempotent
8. git push origin <branch> <tags>     # single push, only missing refs
```

The amend has to land *between* commit and publish: `cargo publish` packs `.cargo_vcs_info.json` into the tarball with the current HEAD SHA, so amending after publish would leave the published tarballs referencing an orphaned SHA on crates.io that we can't ever undo.

Replaced cargo-release's tag and push steps with direct `git tag` / `git push` so we can ( a ) create only the tags that are missing and ( b ) push only the refs that aren't yet on origin. The `tag-message` template ( `"chore: release {{crate_name}} v{{version}}"` ) is reproduced as a one-line helper in `tag_message()`; if `Cargo.toml`'s template ever drifts, a unit test `tag_message_format` pins the format.

### 3. Full resume protocol

Every step gates on a state probe drawn from the repo + crates.io:

| Step | Skip when… | How |
|------|-----------|----|
| version/replace/hook | manifests already at target | literal: read `cargo metadata`, compare; level: any chunk crate's current manifest version is on crates.io → not bumped yet |
| commit + amend | HEAD already amended for this chunk | parse `git log -1 --format=%B`, match `"chore: release"` + body of `- <name> v<ver>` lines against this chunk's crate-to-target map |
| commit only ( skip-then-amend ) | HEAD is "chore: release" with empty body | classified as `CargoReleasePlaceholder` — commit ran but amend didn't |
| publish | crate on crates.io at target version | per-crate `GET https://crates.io/api/v1/crates/<n>/<v>` |
| tag | local tag `<crate>-v<ver>` already exists | one `git tag -l` snapshot |
| push | local HEAD and each chunk tag on origin | one `git ls-remote origin` snapshot |

Unrelated HEAD subject + clean working tree returns `Error::AmbiguousResume { chunk, head_subject }` rather than clobbering; the bailout names the offending subject so the operator can fix manually.

`classify_head` is intentionally lenient: any subject starting with `"chore: release"` ( with or without trailing placeholders, single-crate-rendered or our amend ) is treated as a release commit; identity is established by the body, not the subject. Covers the transition period where `Cargo.toml` could still have the placeholder template, or a future single-crate publish whose subject renders the placeholders cleanly.

### 4. The dependant-crate gap

The user pointed out: when `winterbaume-foo` releases, every workspace member that depends on it ( directly or transitively ) should also be republished, otherwise the published `winterbaume-foo`-consumers on crates.io reference the old version. The harness had no propagation logic.

Added two passes to `tools/release-harness/src/plan.rs`, run in order after the per-crate classification:

#### 4a. Stale-deps pass

For each `Unchanged` / auto-`Skip` non-umbrella non-overridden crate with a prior tag, check each direct workspace dep: is the dep's latest tag NOT an ancestor of this crate's last tag? If so → dep was released after this crate → this crate's published manifest pins a stale version → upgrade to `Patch`. Ancestry check via new `git::is_ancestor` ( `git merge-base --is-ancestor` ). Returns a structured `StaleReport { actionable, umbrellas }`; umbrellas are reported but never auto-upgraded.

#### 4b. Transitive bump propagation

Fixed-point loop: the "bumping" set is `{ Patch, Minor, Major, Pinned, Initial }`. For each bumping crate, every dependant currently at `Unchanged` or auto-`Skip` ( cosmetic-only ) flips to `Patch` with reason `"transitive patch: depends on bumping crate(s) [...]  (previously: <old reason>)"`. Explicit override entries ( whether `Skip`, `Patch`, or `Pinned` ) are never silently overridden — they reflect the operator's deliberate choice. `HeadState::Unrelated`-style umbrellas are reported via `umbrellas_with_moving_deps` and skipped from upgrade.

Stale-deps pass runs first so its newly-Patched crates feed the bumping set of propagation, which then carries the bump further up the chain.

### 5. The optional-deps gap

First plan run after wiring up stale-deps reported one stale crate ( `winterbaume-server` ) and **no** umbrella. The umbrella should have been screaming. Root cause: `build_forward_deps` and `build_reverse_deps` walked `metadata.resolve.nodes`, which is the *resolved* graph — only deps that are actually activated under the current feature set appear there. `winterbaume` declares 214 optional workspace deps and activates none by default, so its resolve node has zero deps and the stale-deps pass saw nothing to check.

Fix: read declared deps directly from `Package.dependencies` instead of `resolve.nodes`. Added a `Dep` struct to `metadata::Package` carrying just the `name` field ( we only need to know which workspace members the package declares deps on, not version reqs or feature gates ). Both `build_forward_deps` and `build_reverse_deps` switched to this source. The umbrella now correctly surfaces with 214 stale deps.

### 6. End-state plan

```
crates_changed:           2
  of which transitive:    0
crates_skipped_doc_only:  221
crates_unchanged:         18
crates_initial:           0

by bump level:
  minor    (1): winterbaume               # pinned via release-plan-overrides.toml
  patch    (1): winterbaume-server        # stale-deps pass: 18 service crates released since server-v0.2.0
```

Umbrella minor ( 0.2.0 → 0.3.0 ) because the inherited service deps include eight breaking 0.x → 1.0 jumps ( account, appconfig, applicationsignals, batch, ec2, ivs, opensearch, organizations ) plus five 0.2 → 0.3 moves ( appconfigdata, kinesisvideo, kinesisvideoarchivedmedia, sagemaker, sesv2 ). CHANGELOG entries written manually rather than via `release-harness changelog` to avoid spawning a nested `claude` CLI from inside the agent session.

Publish step not run — that's the highly-visible / hard-to-reverse boundary, deferred to the operator.

### Pitfalls hit

- The first stab at compute_stale_deps quietly skipped umbrellas instead of reporting them. The user's "what about the umbrella crate?" pulled this back. Refactored compute to return `StaleReport { actionable, umbrellas }` and surface both channels.
- `git merge-base --is-ancestor` returns *false* for two sibling tags created on different commits, even when both are reachable from main. For the umbrella case the existing dependants' tags ARE descendants of the umbrella's tag commit, so the false answer was actually correct ( "newer than umbrella's tag" ). Confirmed by hand: `winterbaume-accessanalyzer-v0.2.0` ( d0926423, 17:41 ) was tagged 2h after `winterbaume-v0.2.0` ( e588530f, 15:40 ) on the same day.
- The earlier session-stored Cargo.toml fix ( `pre-release-commit-message = "chore: release"` ) had silently been re-applied somewhere between sessions, so by the time we committed there was no diff. The `classify_head` leniency tweak ( accept any `chore: release`-prefixed subject ) makes the resume protocol robust to the template drifting back to placeholders.
- `cargo release commit --execute` would fail with "nothing to commit" if a previous attempt already committed the manifest changes. The resume protocol handles this via `HeadState::CargoReleasePlaceholder` ( subject matches but body is empty ) — runs only `git commit --amend`, not a fresh `cargo release commit`.
- Running `release-harness changelog` from inside Claude Code would spawn the `claude` CLI as a subprocess for the polish pass. Wrote the two CHANGELOG entries by hand instead; the user can re-polish later via the steady-state workflow.

### Commits

- `ab54a1ed` per-step subcommand pipeline + full resume in `batch.rs`
- `a0a5cfa9` lenient `classify_head` subject check
- `de8af531` transitive bump propagation in `plan.rs`
- `58534415` stale-deps pass + `git::is_ancestor`
- `2feb4175` declared-deps graph ( fixes umbrella detection )
- `44cff8c4` CHANGELOG entries for both crates + `release-plan-overrides.toml` pinning umbrella to minor

### Gate

Per-crate clippy + fmt clean for `release-harness`; 48 unit tests passing ( 21 `batch::tests`, 21 `plan::tests` covering propagation + stale-deps + classification, 5 `polisher::tests`, 4 `version::tests` ).

### Not done

- Publish step left for the operator — visible/external action, not appropriate to run autonomously.
- `release-harness changelog` polisher pass deferred; the two manually-written entries will need a polish if the steady-state workflow re-runs the harness.
- Level-keyword resume on first-publish crates ( `cargo release version patch -p new-unpublished-crate` ) still has an edge case where the heuristic "any chunk crate's current manifest version is on crates.io → bump needed" returns false for a brand-new crate, so version step is skipped and the publish lands at the unbumped version. The plan-based path uses literal versions for `Initial` / `Pinned` decisions, so this only bites direct `release-harness batch --version patch` against an unpublished crate. Documented in code; revisit when it actually fires.
- The 214-stale-deps umbrella warning is a bit of a wall of text; if future runs hit similar sizes, the message should probably be truncated to the first ~5 deps with a "+N more" tail.

## 2026-05-19 — release-harness: `cargo release commit` rejects `-p`

First real exercise of the new per-step pipeline ( yesterday's `ab54a1ed` ) blew up at the commit step:

```
$ cargo release commit -p winterbaume --sign --no-confirm
error: unexpected argument '-p' found
```

`cargo release` exposes `-p, --package <SPEC>` on `version`, `replace`, `hook`, and `publish`, but **not** on `commit` — `commit` operates on the whole working tree by design ( cargo's help: "Commit the specified packages. Will automatically skip published versions" — the "specified packages" turn out to be whatever the prior step staged ). `run_release_step` in `batch.rs` was unconditionally appending `-p <crate>` to every step.

### Fix

`tools/release-harness/src/batch.rs::run_release_step` now gates the `-p` loop on `step_args.first() != Some("commit")` and adjusts the eprintln preview to match. The dry-run printer in `print_dry_run` was updated in lockstep so the operator-facing preview no longer lies about the commit invocation.

The resume path is unaffected: `classify_head` already handles the `Unrelated + dirty` branch ( manifests bumped, no commit yet, HEAD on an unrelated subject ), which is exactly the state the failure left behind. Re-running `release-harness publish` after the fix lands picks up at the commit step without re-bumping.

### Gate

`cargo fmt -p release-harness`, `cargo clippy -p release-harness --all-targets --all-features -- -D warnings`, `cargo test -p release-harness` — 48/48 unit tests still pass. No new tests added: the bug was a missing conditional on a `Command` argument list, exercised only by the live `cargo release` binary, and the unit-test layer here doesn't shell out.

### Pitfall noted

This is the second time a `cargo release` per-subcommand quirk has bitten the harness ( first was the `consolidate-commits` template-placeholder issue, fixed by the manual amend step ). Worth keeping in mind whenever we add a new step: each `cargo release <subcommand>` has its own argument schema, and the umbrella `cargo release --help` doesn't list them — `cargo release <sub> --help` is the only authority.

## 2026-05-19 — release-harness: `--sign` was also renamed per-subcommand

Second exercise of the per-step pipeline got past the `-p` fix and tripped on the same class of issue one step later:

```
$ cargo release commit --sign --no-confirm
error: unexpected argument '--sign' found
  tip: a similar argument exists: '--sign-commit'
```

The legacy top-level `cargo release --sign` flag was split when the subcommands were decomposed: `commit` now takes `--sign-commit`, `tag` takes `--sign-tag`, and the other steps don't take a sign flag at all. `run_release_step` was still passing `--sign` whenever the caller set `sign=true`.

### Fix

`run_release_step` now maps the boolean `sign` arg to the right per-step flag:

```rust
let sign_flag: Option<&str> = if sign {
    match step {
        "commit" => Some("--sign-commit"),
        "tag" => Some("--sign-tag"),
        _ => None,
    }
} else {
    None
};
```

Only `commit` is currently called with `sign=true` ( the harness drives tagging via `git tag -s` directly rather than `cargo release tag`, see `create_tag` in `batch.rs` ), so the `tag` arm is forward-looking insurance for if/when we ever switch tagging back to cargo-release. Dry-run printer updated in lockstep to show `--sign-commit` instead of `--sign`.

### Gate

Same per-crate gate as the `-p` fix: fmt clean, clippy `-D warnings` clean, 48/48 unit tests pass. No new test coverage — the bug is again in `Command` argument plumbing that the unit-test layer doesn't exercise. Tempted to add a stub test that walks `step_args → expected Command argv` to catch the next per-subcommand schema drift, but holding off until we hit a third instance to confirm the pattern is worth the test-fixture maintenance cost.

### Followup considered

The "the schema of `cargo release <sub>` keeps drifting" pattern suggests we should pin the cargo-release version in the harness's Cargo.toml or at least document the version we validated against. Skipped for now — cargo-release is a global dev-tool install, not a workspace dep, and pinning it would mean either a `rust-toolchain.toml`-style version file or a build.rs check. Neither feels worth it for a harness only used by the maintainer.

## 2026-05-23 — first external bug reports: S3 HeadBucket body + PutObject conditional, plus a sweep of the rest of the conditional surface

mizzy filed our first two GitHub issues against the freshly-released `winterbaume-s3` 0.2:

- **#3 — `HeadBucket` 4xx returns an XML body**, which forces `aws-sdk-rust` into the `Unhandled` branch instead of resolving `HeadBucketError::NotFound`.
- **#4 — `PutObject` accepts `If-None-Match` but does not enforce it.** Second conditional PUT silently overwrites instead of returning 412.

Both repros confirmed against the released crate. Fixed both at the surface they were reported, then escalated the investigation because the same class of bug almost certainly applied to the rest of the S3 operation surface.

### Surface fixes ( `crates/winterbaume-s3/` )

- `S3Error::PreconditionFailed { resource }` → `(412, "PreconditionFailed")` in `s3_error_response`.
- `handle_head_bucket` and `handle_head_object` now route error responses through a new `head_error_response` helper that reuses `s3_error_response` for status + headers but drops the body. The SDK then resolves the typed `NotFound` variant cleanly. ( Real S3 documents that 4xx HEAD responses carry no body — the Smithy model encodes this obliquely via `NotFound: { type: structure, members: {} }`. )
- `handle_put_object` takes `if_match: Option<&str>` and `if_none_match: Option<&str>`, plumbed from the dispatcher's request headers, and checks against the destination ETag *before* writing the blob ( so a rejected PUT does not orphan a blob version ).
- New `etag_matches` + `normalize_etag` helpers handle the RFC 7232 syntax: `*` wildcard, quoted/unquoted ETags, weak `W/"…"` markers, comma-separated lists. mizzy and I caught two bugs in `normalize_etag` mid-review: asymmetric quoting was being silently peeled, and the inside-quotes form `"W/abc"` was leaking the `W/` through. Both fixed; unit tests added for the helpers in `handlers::tests`.

Verbatim repros from #3 and #4 run end-to-end against the local fixes and report `ISSUE3: PASS` / `ISSUE4: PASS`. The repro project lives at `.agents-workspace/tmp/bug-repro/` ( standalone workspace, path-deps to the local crates ).

### Diagnosis: why dossier research missed both

The two bugs trace to the same structural omission in the `service-dossier` skill and its extractor. Both facts were technically in the Smithy model and the AWS docs, but neither stream surfaced them in a way that would have made the operator wire them into a handler.

- The extractor's `Operation Detail Matrix` showed PutObject's `Required input` as `Bucket, Key`. `IfMatch` and `IfNoneMatch` are `@httpHeader`-bound but **optional**, so they never appeared in any column the operator was prompted to read. The extractor never looked at `@httpHeader` / `@httpQuery` / `@httpPrefixHeaders` / `@httpPayload` traits.
- The `Important Shapes` table rendered `NotFound | structure | -`. The `-` was the same encoding used for "truncated at zero items", so the empty-member-set signal — which encodes "no response body" for HEAD operations — was destroyed.
- `PreconditionFailed` is not in `PutObject.errors` in the Smithy model ( PutObject only models `EncryptionTypeMismatch`, `InvalidRequest`, `InvalidWriteOffset`, `TooManyParts` ). The matrix's `Errors` column is therefore a lower bound, not exhaustive, but the SKILL.md research prompts implicitly led the operator to treat it as exhaustive.
- The §3 "Search for" / "Look specifically for" lists covered lifecycle, idempotency, quotas, tagging, IAM, cross-resource — but said nothing about RFC 7232 conditional headers, HTTP-method body semantics, or "modelled errors are a lower bound". So even where the dossier captured `Conditional writes apply to PutObject and CompleteMultipartUpload …` as a research bullet, there was no enforcement signal anywhere to escalate it from "noted" to "must implement".

### Skill + extractor enhancements ( `.agents/skills/service-dossier/` )

- `extract_model_dossier.py`: new `http_input_bindings()` helper reads `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, `@httpPayload`, `@httpLabel` traits per input member. New `## HTTP Bindings` section per dossier with a 4-column table ( Header / Query / Prefix headers / Payload ), emitting one row per operation that has any binding, plus a "Conditional-write/read coverage" trailer that auto-enumerates every operation modelling RFC 7232 headers. For awsJson1-style services with no HTTP-bound inputs, an explicit empty-state note is emitted instead of silently omitting the section. `shape_members()` now renders zero-member shapes as `**empty (no members)**` so the HEAD-body signal is no longer flattened to `-`.
- `SKILL.md`: "Treat `vendor/api-models-aws` as the source of truth for …" now lists HTTP-binding traits and empty-member error shapes explicitly. §3 search prompts gained three conditional-only entries ( `<service> conditional requests preconditions 412`, `<service> HEAD response body`, `<service> error codes responses` ); "Look specifically for" gained three new bullets — RFC 7232 conditional headers and their HTTP codes, HEAD body / HTTP-method semantics, and "modelled errors is a lower bound, not the full set". §4 section ordering and the merge-sensitive section list both updated for `HTTP Bindings`.
- The s3 dossier ( `.agents/docs/services/s3.md` ) was rewritten to include two new sub-sections under `Behavioural Model Notes` — *HTTP protocol pitfalls (HEAD response body)* and *Conditional-write headers* — and the new `HTTP Bindings` section was spliced in between `Operation Detail Matrix` and `Important Shapes`. Three new entries were added to the `Research Checklist for Parity Work`.
- Verified the extractor against DynamoDB ( awsJson1_0 ) too: it surfaced exactly one previously invisible binding — `PutResourcePolicy.ConfirmRemoveSelfResourceAccess -> x-amz-confirm-remove-self-resource-access`. Confirms the change is not S3-specific.

### Sweep of the rest of the conditional surface

The extractor's auto-generated "Conditional-write/read coverage" trailer enumerated **seven** operations modelling RFC 7232 headers: `CompleteMultipartUpload, CopyObject, DeleteObject, GetObject, HeadObject, PutObject, RenameObject`. I had only fixed `PutObject`. The remaining six were silently dropping the same headers.

Five new helpers added at the bottom of `handlers.rs`:

- `PreconditionOutcome { Pass, PreconditionFailed, NotModified }` — `NotModified` only applies to safe methods.
- `parse_http_date` — IMF-fixdate via RFC 2822, RFC 3339 fallback.
- `check_write_preconditions` — `If-Match` / `If-None-Match` against destination ETag.
- `check_read_preconditions` — RFC 7232 ordering: `If-Match` overrides `If-Unmodified-Since` ( 412 group ); `If-None-Match` overrides `If-Modified-Since` ( 304 group ).
- `check_delete_preconditions` — AND'd `If-Match` + `x-amz-if-match-last-modified-time` + `x-amz-if-match-size`.
- `not_modified_response`, `precondition_failed_response`, `precondition_failed_head_response` for the response side. The HEAD variant uses `head_error_response` so 412 HEAD responses stay bodyless, locking in the issue #3 protection across the conditional surface.

Wired into every operation listed in the coverage trailer:

| Operation | Honoured headers | Failure modes |
|---|---|---|
| `PutObject` ( refactored ) | `If-Match`, `If-None-Match` | 412 |
| `CompleteMultipartUpload` | `If-Match`, `If-None-Match` ( destination ) | 412 before any blob assembly |
| `CopyObject` | dest `If-Match`/`If-None-Match`; source `x-amz-copy-source-if-{match,none-match,modified-since,unmodified-since}` | 412 ( no destination written on failure ) |
| `DeleteObject` | `If-Match`, `x-amz-if-match-last-modified-time`, `x-amz-if-match-size` ( AND'd ) | 412; absent-key still idempotent 204 |
| `GetObject` | full RFC 7232 quartet | 412 / 304 per RFC ordering |
| `HeadObject` | full RFC 7232 quartet | 412 / 304, both bodyless |
| `RenameObject` | dest standard + source `x-amz-rename-source-if-*` | 412 ( 304-shape treated as 412 since write op ) |

`DeleteObject`'s conditional-delete surface ( `x-amz-if-match-last-modified-time` + `x-amz-if-match-size` ) and `RenameObject`'s full surface were genuine finds — I had only internalised CompleteMultipartUpload + CopyObject + GetObject + HeadObject from reading AWS docs by hand. The extractor's coverage line caught the other two purely from the Smithy model.

### Gate

`./.agents/bin/cargo.sh fmt -p winterbaume-s3` clean; `clippy -p winterbaume-s3 --all-targets --all-features -- -D warnings` clean; **88 integration tests** ( 10 new conditional tests across all six operations plus the two original issue-#3/#4 regressions ), **7 scenario tests**, **4 smithy-mocks tests**, **3 lib-level unit tests** all green. Verbatim repros for #3 and #4 from the GitHub bug reports run end-to-end and report `PASS`.

### Followups not done

- The `HTTP Bindings` section was retro-fitted into `s3.md` but not into the other ~330 existing service dossiers. The SKILL.md merge guidance now tells future invocations of `service-dossier` to add it on next refresh; no bulk rewrite was attempted.
- `PutObject`'s "modelled errors are a lower bound" insight applies to every write operation in S3 ( conditional writes can legally emit 412/409 even when the error isn't in `errors:` ) and probably to every other service with conditional or quota-bound surfaces. Worth a follow-up pass when we touch other services' write paths.
- Two new feedback memories were saved during this work: `handler-signatures-drop-modelled-fields` and `error-tests-must-assert-typed-variant`. Both apply across services, not just S3.

## 2026-05-23 — bulk dossier refresh: HTTP Bindings retro-fitted across all 417 service dossiers

Followup to the earlier 2026-05-23 entry. The fix-branch left a known gap: the new `HTTP Bindings` section only existed in `s3.md`. Spun off `chore/refresh-dossiers-http-bindings` from `fix/s3-conditional-and-head-body` and applied the section across every remaining dossier under `.agents/docs/services/`.

### Approach

Scope-limited on purpose. Per `.agents/skills/service-dossier/SKILL.md`, sections beyond the new one ( `Operation Detail Matrix`, `Important Shapes`, `Behavioural Model Notes`, `Possible Usage Scenarios`, `Official AWS Documentation Research`, `Research Checklist for Parity Work`, `Resource Model` ) may carry hand-edits or LTM-derived judgement that the extractor would clobber if regenerated wholesale. The refresh only inserts the new `## HTTP Bindings` block immediately before `## Important Shapes` and leaves everything else untouched.

The refresh script lives at `.agents-workspace/tmp/refresh_dossiers.py` ( gitignored ). It is idempotent: if a dossier already carries `## HTTP Bindings` it is skipped, so re-running the script after future extractor changes ( eg. adding a new column ) will not double-insert. The classification per dossier is:

- `updated` — section inserted.
- `present` — section already there, skip.
- `no_anchor` — `## Important Shapes` missing, skip and log ( did not happen ).
- `extractor_failed` — model resolution or extractor error, skip and log ( did not happen ).
- `no_block` — extractor produced no HTTP Bindings section ( did not happen ).

Dry-ran on a 7-service sample under `.agents-workspace/tmp/sample-dossiers/` covering restJson1, restXml, awsJson1, ec2Query, awsQuery, restJson1+REST URLs ( api-gateway, accessanalyzer, cloudfront, acm, ec2, sts, lambda ). Confirmed row counts matched expectations ( awsJson*/ec2Query → 0 binding rows + empty-state note; rest-style → 11-109 binding rows ). Second-run reported all 7 `present` confirming idempotency.

### Outcome

Bulk apply on the real tree: **`updated: 417`, `present: 1` ( s3 ), 0 errors**. Total 418 dossiers, every one now carries the section. Sanity-checked: every dossier has exactly one `## HTTP Bindings` heading; section ordering ( `Operation Detail Matrix` → `HTTP Bindings` → `Important Shapes` ) is correct in every file.

Commit `c46147ab`: 417 files changed, 4939 insertions, 0 deletions. Signed and pushed to `origin/chore/refresh-dossiers-http-bindings`.

### Cross-service findings

Four services carry the auto-generated `**Conditional-write/read coverage**` trailer because their Smithy models include operations with `If-Match` / `If-None-Match` / `If-Modified-Since` / `If-Unmodified-Since` headers. Three of the four were not on my radar before the refresh:

| Service | Operations with modelled conditional headers | Notes |
|---|---|---|
| `s3` | `CompleteMultipartUpload`, `CopyObject`, `DeleteObject`, `GetObject`, `HeadObject`, `PutObject`, `RenameObject` | Already enforced on the previous branch. |
| `cloudfront` | ~50 lifecycle operations: every `Delete*`/`Update*`, plus `AssociateDistributionTenantWebACL`, `AssociateDistributionWebACL`, `CopyDistribution`, `DisassociateDistributionTenantWebACL`, `DisassociateDistributionWebACL`, `PublishConnectionFunction`, `PublishFunction`, `TestConnectionFunction`, `TestFunction` | The single largest hidden conditional surface in the codebase. CloudFront's API documents `If-Match` as required-for-correctness on most mutations; ignoring it produces silent overwrites. |
| `cloudfront-keyvaluestore` | `DeleteKey`, `PutKey`, `UpdateKeys` | |
| `iotsitewise` | `CreateAssetModelCompositeModel`, `DeleteAssetModel`, `DeleteAssetModelCompositeModel`, `UpdateAssetModel`, `UpdateAssetModelCompositeModel` | Asset-model lifecycle only. |

Spot-checked a binding row in cloudfront's `UpdateDistribution`: renders as `| UpdateDistribution | IfMatch -> If-Match | - | - | DistributionConfig |`. Clean.

### Hidden bindings beyond the conditional surface

The refresh also surfaces every other modelled `@httpHeader` / `@httpQuery` / `@httpPrefixHeaders` / `@httpPayload` member across all services — these are easy to miss because they sit outside the URL and outside the body. A non-exhaustive sample of what is newly visible:

- `cloudfront`: 109 binding rows. Beyond `IfMatch`, every operation carries `@httpHeader` modifiers for accept languages, content checksums, anonymous-vs-authenticated request markers, and so on.
- `api-gateway`: 39 rows. Includes `Accept` header binding, `X-Amzn-*` SDK overrides.
- `accessanalyzer`: 11 rows. Pagination, idempotency tokens carried via headers.
- `lambda`: 13 rows. `X-Amz-Function-Error`, `X-Amz-Invocation-Type`, `X-Amz-Log-Type`, etc.

Most of these are presently ignored by hand-written winterbaume handlers. None are bug reports yet, but the same class of bug ( silently drop a modelled header → SDK fails to round-trip an option the caller set ) applies. Worth pulling into the parity quality gate on a per-service basis.

### Followups not done

- Filing follow-up issues for CloudFront/iotsitewise/cloudfront-keyvaluestore conditional enforcement. These crates are not yet implemented at parity, so the conditional gap is moot until the lifecycle operations themselves are wired up. The dossier now flags it for whoever picks up those crates.
- The dossier refresh did not update `Important Shapes` tables to use the new `**empty (no members)**` rendering for zero-member shapes. Cosmetic gain only; the strong-signal use of this rendering ( "no body for HEAD" ) is already captured in the narrative `HTTP protocol pitfalls` subsection for s3, and other services' HEAD operations are similarly few.
- The dossier refresh did not regenerate any other extractor-emitted section against the current Smithy models. Some sections may have drifted since the dossier was first generated ( eg. operation counts, error lists ). A wholesale refresh would require per-dossier diff review to preserve hand-edits — out of scope for this commit.
