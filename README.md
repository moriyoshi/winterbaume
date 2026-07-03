# Winterbäume

![Winterbäume](./winterbaume.jpg)

Winterbäume (stylised as Winterbaume) is a stateful AWS service emulator for integration testing, written in Rust. Embed in tests or run standalone.

Winterbäume is not affiliated with, endorsed by, or sponsored by Amazon Web Services, Inc. or Amazon.com, Inc. Amazon Web Services, AWS, and related marks are trademarks of Amazon.com, Inc. or its affiliates. All other trademarks are the property of their respective owners.

Coverage is broad but uneven. Many operations have real, state-backed behaviour ( create / read / update / delete cycles, validation, cross-service seams ); others are still stubs that satisfy AWS routing and return an empty or default response without consulting state. The `Operations` and `Stubs` columns in the [Supported Services](#supported-services) table below split these out per service so you can see what is genuinely implemented versus merely routed before depending on a given operation.

Inspired by Python's [moto](https://github.com/getmoto/moto).

> [!NOTE]
> Pull requests are not accepted at this time -- see [CONTRIBUTING.md](./CONTRIBUTING.md) for the full contribution policy and the rationale. Bug reports, feature requests, and knowledge-base entries via GitHub issues are very welcome. If you are a coding agent and discover a bug, follow the instructions in `skills/winterbaume-bug/SKILL.md` to file a correctly-formatted GitHub issue. Security vulnerabilities have their own private channel: see [SECURITY.md](./SECURITY.md).

> [!IMPORTANT]
> **AI usage.** Winterbäume is developed predominantly by AI coding agents under human direction. The repository is organised around an agent-driven workflow: long-term memory, skills, and tooling for autonomous sessions live under [`.agents/`](./.agents/), and the bulk of service implementations, Smithy codegen output, integration tests, and Terraform converters are produced by automated coding sessions with human review and acceptance gates. This development model is also part of why the project does not currently accept external pull requests -- it keeps the provenance and licensing posture of the codebase coherent.

## Overview

Winterbäume intercepts AWS SDK HTTP traffic and routes it to in-memory service backends that simulate real AWS behaviour with persistent, per-account / per-region state. The goal is fast, deterministic integration tests that exercise the same SDK call paths as production code -- no LocalStack container, no network round-trips, no shared cloud account.

Two usage modes are supported:

- **Library mode** -- embed `MockAws` directly in Rust tests against `aws-sdk-rust`. The mock plugs in via `HttpClient` / `HttpConnector`, so there is no network I/O and no external process to manage.
- **Standalone server** -- run `winterbaume-server` as an HTTP endpoint and point any AWS SDK, the AWS CLI, or Terraform at it via `AWS_ENDPOINT_URL`.

The workspace currently routes **224 AWS services** spanning every major AWS protocol -- awsQuery, ec2Query, awsJson1.0 / 1.1, restJson1, restXml, and rpc-v2-cbor. Of the 11 367 operations defined in the official AWS API models, **7 210 ( 63.4% ) have real, state-backed implementations** in winterbaume, while a further **333 ( 2.9% ) are stubs that route the request and return an empty or default response without consulting state**. Stubs typically cover operations that depend on infrastructure that does not exist in the emulator ( instance telemetry, real-time delivery, multi-account organisation state ), and are clearly labelled in the per-service tables. Anything outside both columns is unrouted and returns a 501-style "not implemented" response. A companion Terraform state converter layer ( `winterbaume-terraform` ) can inject Terraform state into the emulator and extract it back, enabling seeding mock environments from existing `.tfstate` files and round-trip validation against the official AWS provider schema.

Service backends are hand-written for behaviour and validation, while wire-level types and serialisers are generated from Smithy models via the in-tree `smithy-codegen` tool. Correctness is validated in layers: integration tests against real `aws-sdk-rust` clients, ports of moto's behavioural test suite where applicable, and end-to-end Terraform suites that drive `terraform apply` against an in-process server. Operation-count coverage is a prioritisation signal rather than a behavioural guarantee, so before relying on a particular operation, check the per-crate README for whether it is implemented, stubbed, or unrouted.

## Supported Services

The `Operations` column shows real, state-backed implementations. The `Stubs` column shows operations whose handler routes the request and returns an empty/default response without real behaviour ( either annotated with `// STUB[<category>]: ...` in `handlers.rs`, or detected heuristically as a body that produces a default response without consulting state ). The two columns are disjoint -- stubs are excluded from the operation count.

| Service | Crate | Protocol | Operations | Stubs | moto | floci | kumo | fakecloud |
|---|---|---|---|---|---|---|---|---|
| [Account](crates/winterbaume-account/README.md) | `winterbaume-account` | restJson1 | 14/15 (93.3%) | 1/15 (6.7%) | 3/15 (20.0%) | 0/15 (0.0%) | 0/15 (0.0%) | 15/15 (100.0%) |
| [ACM](crates/winterbaume-acm/README.md) | `winterbaume-acm` | awsJson1.1 | 16/17 (94.1%) | 0/17 (0.0%) | 11/17 (64.7%) | 0/17 (0.0%) | 6/17 (35.3%) | 17/17 (100.0%) |
| [ACM PCA](crates/winterbaume-acmpca/README.md) | `winterbaume-acmpca` | awsJson1.1 | 23/23 (100.0%) | 0/23 (0.0%) | 17/23 (73.9%) | 0/23 (0.0%) | 0/23 (0.0%) | 0/23 (0.0%) |
| [AIOps](crates/winterbaume-aiops/README.md) | `winterbaume-aiops` | restJson1 | 11/11 (100.0%) | 0/11 (0.0%) | 0/11 (0.0%) | 0/11 (0.0%) | 0/11 (0.0%) | 0/11 (0.0%) |
| [AMP/Prometheus](crates/winterbaume-amp/README.md) | `winterbaume-amp` | restJson1 | 17/44 (38.6%) | 0/44 (0.0%) | 17/44 (38.6%) | 0/44 (0.0%) | 0/44 (0.0%) | 0/44 (0.0%) |
| [Amplify](crates/winterbaume-amplify/README.md) | `winterbaume-amplify` | restJson1 | 23/37 (62.2%) | 0/37 (0.0%) | 0/37 (0.0%) | 0/37 (0.0%) | 9/37 (24.3%) | 0/37 (0.0%) |
| [Amplify Backend](crates/winterbaume-amplifybackend/README.md) | `winterbaume-amplifybackend` | restJson1 | 4/31 (12.9%) | 0/31 (0.0%) | 0/31 (0.0%) | 0/31 (0.0%) | 0/31 (0.0%) | 0/31 (0.0%) |
| [Amplify UI Builder](crates/winterbaume-amplifyuibuilder/README.md) | `winterbaume-amplifyuibuilder` | restJson1 | 28/28 (100.0%) | 0/28 (0.0%) | 0/28 (0.0%) | 0/28 (0.0%) | 0/28 (0.0%) | 0/28 (0.0%) |
| [API Gateway](crates/winterbaume-apigateway/README.md) | `winterbaume-apigateway` | restJson1 | 117/124 (94.4%) | 2/124 (1.6%) | 78/124 (62.9%) | 72/124 (58.1%) | 17/124 (13.7%) | 124/124 (100.0%) |
| [API Gateway Management API](crates/winterbaume-apigatewaymanagement/README.md) | `winterbaume-apigatewaymanagement` | restJson1 | 3/3 (100.0%) | 0/3 (0.0%) | 3/3 (100.0%) | 0/3 (0.0%) | 0/3 (0.0%) | 0/3 (0.0%) |
| [API Gateway V2](crates/winterbaume-apigatewayv2/README.md) | `winterbaume-apigatewayv2` | restJson1 | 62/103 (60.2%) | 0/103 (0.0%) | 54/103 (52.4%) | 0/103 (0.0%) | 22/103 (21.4%) | 103/103 (100.0%) |
| [App Mesh](crates/winterbaume-appmesh/README.md) | `winterbaume-appmesh` | restJson1 | 38/38 (100.0%) | 0/38 (0.0%) | 0/38 (0.0%) | 0/38 (0.0%) | 25/38 (65.8%) | 0/38 (0.0%) |
| [App Runner](crates/winterbaume-apprunner/README.md) | `winterbaume-apprunner` | awsJson1.0 | 23/37 (62.2%) | 0/37 (0.0%) | 0/37 (0.0%) | 0/37 (0.0%) | 0/37 (0.0%) | 0/37 (0.0%) |
| [AppConfig](crates/winterbaume-appconfig/README.md) | `winterbaume-appconfig` | restJson1 | 45/45 (100.0%) | 0/45 (0.0%) | 15/45 (33.3%) | 0/45 (0.0%) | 0/45 (0.0%) | 0/45 (0.0%) |
| [AppConfig Data](crates/winterbaume-appconfigdata/README.md) | `winterbaume-appconfigdata` | restJson1 | 2/2 (100.0%) | 0/2 (0.0%) | 0/2 (0.0%) | 0/2 (0.0%) | 0/2 (0.0%) | 0/2 (0.0%) |
| [AppFabric](crates/winterbaume-appfabric/README.md) | `winterbaume-appfabric` | restJson1 | 6/26 (23.1%) | 0/26 (0.0%) | 0/26 (0.0%) | 0/26 (0.0%) | 0/26 (0.0%) | 0/26 (0.0%) |
| [AppFlow](crates/winterbaume-appflow/README.md) | `winterbaume-appflow` | restJson1 | 9/25 (36.0%) | 0/25 (0.0%) | 0/25 (0.0%) | 0/25 (0.0%) | 0/25 (0.0%) | 0/25 (0.0%) |
| [AppIntegrations](crates/winterbaume-appintegrations/README.md) | `winterbaume-appintegrations` | restJson1 | 23/23 (100.0%) | 0/23 (0.0%) | 0/23 (0.0%) | 0/23 (0.0%) | 0/23 (0.0%) | 0/23 (0.0%) |
| [Application Auto Scaling](crates/winterbaume-applicationautoscaling/README.md) | `winterbaume-applicationautoscaling` | awsJson1.1 | 13/14 (92.9%) | 1/14 (7.1%) | 9/14 (64.3%) | 0/14 (0.0%) | 0/14 (0.0%) | 14/14 (100.0%) |
| [Application Cost Profiler](crates/winterbaume-applicationcostprofiler/README.md) | `winterbaume-applicationcostprofiler` | restJson1 | 6/6 (100.0%) | 0/6 (0.0%) | 0/6 (0.0%) | 0/6 (0.0%) | 0/6 (0.0%) | 0/6 (0.0%) |
| [Application Discovery Service](crates/winterbaume-applicationdiscoveryservice/README.md) | `winterbaume-applicationdiscoveryservice` | awsJson1.1 | 28/28 (100.0%) | 0/28 (0.0%) | 0/28 (0.0%) | 0/28 (0.0%) | 0/28 (0.0%) | 0/28 (0.0%) |
| [Application Insights](crates/winterbaume-applicationinsights/README.md) | `winterbaume-applicationinsights` | awsJson1.1 | 33/33 (100.0%) | 0/33 (0.0%) | 0/33 (0.0%) | 0/33 (0.0%) | 0/33 (0.0%) | 0/33 (0.0%) |
| [Application Signals](crates/winterbaume-applicationsignals/README.md) | `winterbaume-applicationsignals` | restJson1 | 10/23 (43.5%) | 3/23 (13.0%) | 0/23 (0.0%) | 0/23 (0.0%) | 0/23 (0.0%) | 0/23 (0.0%) |
| [AppSync](crates/winterbaume-appsync/README.md) | `winterbaume-appsync` | restJson1 | 27/74 (36.5%) | 0/74 (0.0%) | 27/74 (36.5%) | 0/74 (0.0%) | 3/74 (4.1%) | 0/74 (0.0%) |
| [ARC Zonal Shift](crates/winterbaume-arczonalshift/README.md) | `winterbaume-arczonalshift` | restJson1 | 14/15 (93.3%) | 1/15 (6.7%) | 0/15 (0.0%) | 0/15 (0.0%) | 0/15 (0.0%) | 0/15 (0.0%) |
| [Artifact](crates/winterbaume-artifact/README.md) | `winterbaume-artifact` | restJson1 | 8/8 (100.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) |
| [Athena](crates/winterbaume-athena/README.md) | `winterbaume-athena` | awsJson1.1 | 25/70 (35.7%) | 0/70 (0.0%) | 27/70 (38.6%) | 0/70 (0.0%) | 7/70 (10.0%) | 70/70 (100.0%) |
| [Audit Manager](crates/winterbaume-auditmanager/README.md) | `winterbaume-auditmanager` | restJson1 | 15/62 (24.2%) | 0/62 (0.0%) | 0/62 (0.0%) | 0/62 (0.0%) | 0/62 (0.0%) | 0/62 (0.0%) |
| [Aurora DSQL](crates/winterbaume-dsql/README.md) | `winterbaume-dsql` | restJson1 | 12/12 (100.0%) | 0/12 (0.0%) | 5/12 (41.7%) | 0/12 (0.0%) | 0/12 (0.0%) | 12/12 (100.0%) |
| [Auto Scaling](crates/winterbaume-autoscaling/README.md) | `winterbaume-autoscaling` | awsQuery | 52/66 (78.8%) | 0/66 (0.0%) | 39/66 (59.1%) | 0/66 (0.0%) | 0/66 (0.0%) | 13/66 (19.7%) |
| [Auto Scaling Plans](crates/winterbaume-autoscalingplans/README.md) | `winterbaume-autoscalingplans` | awsJson1.1 | 6/6 (100.0%) | 0/6 (0.0%) | 0/6 (0.0%) | 0/6 (0.0%) | 0/6 (0.0%) | 0/6 (0.0%) |
| [Backup](crates/winterbaume-backup/README.md) | `winterbaume-backup` | restJson1 | 105/108 (97.2%) | 3/108 (2.8%) | 17/108 (15.7%) | 0/108 (0.0%) | 12/108 (11.1%) | 0/108 (0.0%) |
| [Backup Gateway](crates/winterbaume-backupgateway/README.md) | `winterbaume-backupgateway` | awsJson1.0 | 25/25 (100.0%) | 0/25 (0.0%) | 0/25 (0.0%) | 0/25 (0.0%) | 0/25 (0.0%) | 0/25 (0.0%) |
| [Backup Search](crates/winterbaume-backupsearch/README.md) | `winterbaume-backupsearch` | restJson1 | 9/12 (75.0%) | 0/12 (0.0%) | 0/12 (0.0%) | 0/12 (0.0%) | 0/12 (0.0%) | 0/12 (0.0%) |
| [Batch](crates/winterbaume-batch/README.md) | `winterbaume-batch` | restJson1 | 39/45 (86.7%) | 0/45 (0.0%) | 24/45 (53.3%) | 0/45 (0.0%) | 10/45 (22.2%) | 45/45 (100.0%) |
| [BCM Dashboards](crates/winterbaume-bcmdashboards/README.md) | `winterbaume-bcmdashboards` | awsJson1.0 | 9/15 (60.0%) | 0/15 (0.0%) | 0/15 (0.0%) | 0/15 (0.0%) | 0/15 (0.0%) | 0/15 (0.0%) |
| [BCM Data Exports](crates/winterbaume-bcmdataexports/README.md) | `winterbaume-bcmdataexports` | awsJson1.1 | 12/12 (100.0%) | 0/12 (0.0%) | 0/12 (0.0%) | 0/12 (0.0%) | 0/12 (0.0%) | 0/12 (0.0%) |
| [BCM Recommended Actions](crates/winterbaume-bcmrecommendedactions/README.md) | `winterbaume-bcmrecommendedactions` | awsJson1.0 | 1/1 (100.0%) | 0/1 (0.0%) | 0/1 (0.0%) | 0/1 (0.0%) | 0/1 (0.0%) | 0/1 (0.0%) |
| [Bedrock](crates/winterbaume-bedrock/README.md) | `winterbaume-bedrock` | restJson1 | 48/101 (47.5%) | 0/101 (0.0%) | 13/101 (12.9%) | 0/101 (0.0%) | 0/101 (0.0%) | 101/101 (100.0%) |
| [Bedrock Agent](crates/winterbaume-bedrockagent/README.md) | `winterbaume-bedrockagent` | restJson1 | 72/72 (100.0%) | 0/72 (0.0%) | 11/72 (15.3%) | 0/72 (0.0%) | 0/72 (0.0%) | 72/72 (100.0%) |
| [Billing](crates/winterbaume-billing/README.md) | `winterbaume-billing` | awsJson1.0 | 12/12 (100.0%) | 0/12 (0.0%) | 0/12 (0.0%) | 0/12 (0.0%) | 0/12 (0.0%) | 0/12 (0.0%) |
| [Braket](crates/winterbaume-braket/README.md) | `winterbaume-braket` | restJson1 | 12/17 (70.6%) | 0/17 (0.0%) | 0/17 (0.0%) | 0/17 (0.0%) | 0/17 (0.0%) | 0/17 (0.0%) |
| [Budgets](crates/winterbaume-budgets/README.md) | `winterbaume-budgets` | awsJson1.1 | 7/26 (26.9%) | 0/26 (0.0%) | 7/26 (26.9%) | 0/26 (0.0%) | 0/26 (0.0%) | 0/26 (0.0%) |
| [Chatbot](crates/winterbaume-chatbot/README.md) | `winterbaume-chatbot` | restJson1 | 15/34 (44.1%) | 0/34 (0.0%) | 0/34 (0.0%) | 0/34 (0.0%) | 0/34 (0.0%) | 0/34 (0.0%) |
| [Chime SDK Meetings](crates/winterbaume-chimesdkmeetings/README.md) | `winterbaume-chimesdkmeetings` | restJson1 | 12/16 (75.0%) | 0/16 (0.0%) | 0/16 (0.0%) | 0/16 (0.0%) | 0/16 (0.0%) | 0/16 (0.0%) |
| [Cloud Control API](crates/winterbaume-cloudcontrol/README.md) | `winterbaume-cloudcontrol` | awsJson1.0 | 8/8 (100.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 6/8 (75.0%) | 8/8 (100.0%) |
| [Cloud Directory](crates/winterbaume-clouddirectory/README.md) | `winterbaume-clouddirectory` | restJson1 | 13/66 (19.7%) | 0/66 (0.0%) | 13/66 (19.7%) | 0/66 (0.0%) | 0/66 (0.0%) | 0/66 (0.0%) |
| [Cloud9](crates/winterbaume-cloud9/README.md) | `winterbaume-cloud9` | awsJson1.1 | 13/13 (100.0%) | 0/13 (0.0%) | 0/13 (0.0%) | 0/13 (0.0%) | 0/13 (0.0%) | 0/13 (0.0%) |
| [CloudFormation](crates/winterbaume-cloudformation/README.md) | `winterbaume-cloudformation` | awsQuery | 40/90 (44.4%) | 3/90 (3.3%) | 33/90 (36.7%) | 0/90 (0.0%) | 8/90 (8.9%) | 90/90 (100.0%) |
| [CloudFront](crates/winterbaume-cloudfront/README.md) | `winterbaume-cloudfront` | restXml | 156/167 (93.4%) | 11/167 (6.6%) | 25/167 (15.0%) | 0/167 (0.0%) | 16/167 (9.6%) | 167/167 (100.0%) |
| [CloudFront KeyValueStore](crates/winterbaume-cloudfrontkeyvaluestore/README.md) | `winterbaume-cloudfrontkeyvaluestore` | restJson1 | 5/6 (83.3%) | 0/6 (0.0%) | 0/6 (0.0%) | 0/6 (0.0%) | 0/6 (0.0%) | 0/6 (0.0%) |
| [CloudHSM v2](crates/winterbaume-cloudhsmv2/README.md) | `winterbaume-cloudhsmv2` | awsJson1.1 | 18/18 (100.0%) | 0/18 (0.0%) | 0/18 (0.0%) | 0/18 (0.0%) | 0/18 (0.0%) | 0/18 (0.0%) |
| [CloudSearch Domain](crates/winterbaume-cloudsearchdomain/README.md) | `winterbaume-cloudsearchdomain` | restJson1 | 2/3 (66.7%) | 0/3 (0.0%) | 0/3 (0.0%) | 0/3 (0.0%) | 0/3 (0.0%) | 0/3 (0.0%) |
| [CloudTrail](crates/winterbaume-cloudtrail/README.md) | `winterbaume-cloudtrail` | awsJson1.1 | 21/60 (35.0%) | 2/60 (3.3%) | 16/60 (26.7%) | 0/60 (0.0%) | 8/60 (13.3%) | 0/60 (0.0%) |
| [CloudTrail Data](crates/winterbaume-cloudtraildata/README.md) | `winterbaume-cloudtraildata` | restJson1 | 1/1 (100.0%) | 0/1 (0.0%) | 0/1 (0.0%) | 0/1 (0.0%) | 0/1 (0.0%) | 0/1 (0.0%) |
| [CloudWatch](crates/winterbaume-cloudwatch/README.md) | `winterbaume-cloudwatch` | awsJson1.0 | 38/46 (82.6%) | 5/46 (10.9%) | 20/46 (43.5%) | 0/46 (0.0%) | 11/46 (23.9%) | 46/46 (100.0%) |
| [CloudWatch Logs](crates/winterbaume-cloudwatchlogs/README.md) | `winterbaume-cloudwatchlogs` | awsJson1.1 | 93/113 (82.3%) | 15/113 (13.3%) | 52/113 (46.0%) | 0/113 (0.0%) | 11/113 (9.7%) | 113/113 (100.0%) |
| [CodeArtifact](crates/winterbaume-codeartifact/README.md) | `winterbaume-codeartifact` | restJson1 | 9/48 (18.8%) | 0/48 (0.0%) | 0/48 (0.0%) | 0/48 (0.0%) | 0/48 (0.0%) | 0/48 (0.0%) |
| [CodeBuild](crates/winterbaume-codebuild/README.md) | `winterbaume-codebuild` | awsJson1.1 | 29/59 (49.2%) | 0/59 (0.0%) | 9/59 (15.3%) | 0/59 (0.0%) | 0/59 (0.0%) | 0/59 (0.0%) |
| [CodeCommit](crates/winterbaume-codecommit/README.md) | `winterbaume-codecommit` | awsJson1.1 | 25/79 (31.6%) | 0/79 (0.0%) | 3/79 (3.8%) | 0/79 (0.0%) | 0/79 (0.0%) | 0/79 (0.0%) |
| [CodeDeploy](crates/winterbaume-codedeploy/README.md) | `winterbaume-codedeploy` | awsJson1.1 | 15/47 (31.9%) | 0/47 (0.0%) | 14/47 (29.8%) | 0/47 (0.0%) | 0/47 (0.0%) | 0/47 (0.0%) |
| [CodeGuru Reviewer](crates/winterbaume-codegurureviewer/README.md) | `winterbaume-codegurureviewer` | restJson1 | 9/14 (64.3%) | 0/14 (0.0%) | 0/14 (0.0%) | 0/14 (0.0%) | 11/14 (78.6%) | 0/14 (0.0%) |
| [CodeGuru Security](crates/winterbaume-codegurusecurity/README.md) | `winterbaume-codegurusecurity` | restJson1 | 11/13 (84.6%) | 0/13 (0.0%) | 0/13 (0.0%) | 0/13 (0.0%) | 0/13 (0.0%) | 0/13 (0.0%) |
| [CodePipeline](crates/winterbaume-codepipeline/README.md) | `winterbaume-codepipeline` | awsJson1.1 | 44/44 (100.0%) | 0/44 (0.0%) | 8/44 (18.2%) | 0/44 (0.0%) | 0/44 (0.0%) | 0/44 (0.0%) |
| [CodeStar Notifications](crates/winterbaume-codestarnotifications/README.md) | `winterbaume-codestarnotifications` | restJson1 | 7/13 (53.8%) | 0/13 (0.0%) | 0/13 (0.0%) | 0/13 (0.0%) | 0/13 (0.0%) | 0/13 (0.0%) |
| [Cognito Identity](crates/winterbaume-cognitoidentity/README.md) | `winterbaume-cognitoidentity` | awsJson1.1 | 20/23 (87.0%) | 3/23 (13.0%) | 10/23 (43.5%) | 0/23 (0.0%) | 0/23 (0.0%) | 23/23 (100.0%) |
| [Cognito Identity Provider](crates/winterbaume-cognitoidentityprovider/README.md) | `winterbaume-cognitoidentityprovider` | awsJson1.1 | 104/122 (85.2%) | 18/122 (14.8%) | 62/122 (50.8%) | 0/122 (0.0%) | 17/122 (13.9%) | 122/122 (100.0%) |
| [Cognito Sync](crates/winterbaume-cognitosync/README.md) | `winterbaume-cognitosync` | restJson1 | 11/17 (64.7%) | 0/17 (0.0%) | 0/17 (0.0%) | 0/17 (0.0%) | 0/17 (0.0%) | 0/17 (0.0%) |
| [Comprehend](crates/winterbaume-comprehend/README.md) | `winterbaume-comprehend` | awsJson1.1 | 60/85 (70.6%) | 5/85 (5.9%) | 63/85 (74.1%) | 0/85 (0.0%) | 12/85 (14.1%) | 0/85 (0.0%) |
| [Config](crates/winterbaume-config/README.md) | `winterbaume-config` | awsJson1.1 | 46/97 (47.4%) | 3/97 (3.1%) | 38/97 (39.2%) | 0/97 (0.0%) | 9/97 (9.3%) | 0/97 (0.0%) |
| [Connect](crates/winterbaume-connect/README.md) | `winterbaume-connect` | restJson1 | 10/370 (2.7%) | 0/370 (0.0%) | 10/370 (2.7%) | 0/370 (0.0%) | 0/370 (0.0%) | 0/370 (0.0%) |
| [Connect Campaigns](crates/winterbaume-connectcampaigns/README.md) | `winterbaume-connectcampaigns` | restJson1 | 14/22 (63.6%) | 0/22 (0.0%) | 14/22 (63.6%) | 0/22 (0.0%) | 0/22 (0.0%) | 0/22 (0.0%) |
| [Connect Contact Lens](crates/winterbaume-connectcontactlens/README.md) | `winterbaume-connectcontactlens` | restJson1 | 0/1 (0.0%) | 0/1 (0.0%) | 0/1 (0.0%) | 0/1 (0.0%) | 0/1 (0.0%) | 0/1 (0.0%) |
| [Connect Participant](crates/winterbaume-connectparticipant/README.md) | `winterbaume-connectparticipant` | restJson1 | 7/11 (63.6%) | 0/11 (0.0%) | 0/11 (0.0%) | 0/11 (0.0%) | 0/11 (0.0%) | 0/11 (0.0%) |
| [Control Catalog](crates/winterbaume-controlcatalog/README.md) | `winterbaume-controlcatalog` | restJson1 | 6/6 (100.0%) | 0/6 (0.0%) | 0/6 (0.0%) | 0/6 (0.0%) | 0/6 (0.0%) | 0/6 (0.0%) |
| [Cost and Usage Report Service](crates/winterbaume-costandusagereport/README.md) | `winterbaume-costandusagereport` | awsJson1.1 | 7/7 (100.0%) | 0/7 (0.0%) | 0/7 (0.0%) | 0/7 (0.0%) | 0/7 (0.0%) | 0/7 (0.0%) |
| [Cost Explorer](crates/winterbaume-costexplorer/README.md) | `winterbaume-costexplorer` | awsJson1.1 | 22/47 (46.8%) | 25/47 (53.2%) | 0/47 (0.0%) | 0/47 (0.0%) | 8/47 (17.0%) | 0/47 (0.0%) |
| [Cost Optimization Hub](crates/winterbaume-costoptimizationhub/README.md) | `winterbaume-costoptimizationhub` | awsJson1.0 | 8/8 (100.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) |
| [Data Pipeline](crates/winterbaume-datapipeline/README.md) | `winterbaume-datapipeline` | awsJson1.1 | 19/19 (100.0%) | 0/19 (0.0%) | 0/19 (0.0%) | 0/19 (0.0%) | 0/19 (0.0%) | 0/19 (0.0%) |
| [Database Migration Service (DMS)](crates/winterbaume-databasemigration/README.md) | `winterbaume-databasemigration` | awsJson1.1 | 42/119 (35.3%) | 0/119 (0.0%) | 17/119 (14.3%) | 0/119 (0.0%) | 0/119 (0.0%) | 0/119 (0.0%) |
| [DataBrew](crates/winterbaume-databrew/README.md) | `winterbaume-databrew` | restJson1 | 32/44 (72.7%) | 1/44 (2.3%) | 24/44 (54.5%) | 0/44 (0.0%) | 0/44 (0.0%) | 0/44 (0.0%) |
| [DataSync](crates/winterbaume-datasync/README.md) | `winterbaume-datasync` | awsJson1.1 | 8/53 (15.1%) | 0/53 (0.0%) | 6/53 (11.3%) | 0/53 (0.0%) | 0/53 (0.0%) | 0/53 (0.0%) |
| [DAX](crates/winterbaume-dax/README.md) | `winterbaume-dax` | awsJson1.1 | 6/21 (28.6%) | 0/21 (0.0%) | 8/21 (38.1%) | 0/21 (0.0%) | 0/21 (0.0%) | 0/21 (0.0%) |
| [Direct Connect](crates/winterbaume-directconnect/README.md) | `winterbaume-directconnect` | awsJson1.1 | 7/63 (11.1%) | 0/63 (0.0%) | 0/63 (0.0%) | 0/63 (0.0%) | 0/63 (0.0%) | 0/63 (0.0%) |
| [Directory Service](crates/winterbaume-directory/README.md) | `winterbaume-directory` | awsJson1.1 | 4/80 (5.0%) | 0/80 (0.0%) | 0/80 (0.0%) | 0/80 (0.0%) | 6/80 (7.5%) | 0/80 (0.0%) |
| [DLM](crates/winterbaume-dlm/README.md) | `winterbaume-dlm` | restJson1 | 2/8 (25.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 5/8 (62.5%) | 0/8 (0.0%) |
| [DynamoDB](crates/winterbaume-dynamodb/README.md) | `winterbaume-dynamodb` | awsJson1.0 | 57/57 (100.0%) | 0/57 (0.0%) | 39/57 (68.4%) | 0/57 (0.0%) | 21/57 (36.8%) | 57/57 (100.0%) |
| [DynamoDB Streams](crates/winterbaume-dynamodbstreams/README.md) | `winterbaume-dynamodbstreams` | awsJson1.0 | 4/4 (100.0%) | 0/4 (0.0%) | 0/4 (0.0%) | 0/4 (0.0%) | 0/4 (0.0%) | 4/4 (100.0%) |
| [EBS](crates/winterbaume-ebs/README.md) | `winterbaume-ebs` | restJson1 | 6/6 (100.0%) | 0/6 (0.0%) | 6/6 (100.0%) | 0/6 (0.0%) | 0/6 (0.0%) | 0/6 (0.0%) |
| [EC2](crates/winterbaume-ec2/README.md) | `winterbaume-ec2` | ec2Query | 713/763 (93.4%) | 50/763 (6.6%) | 223/763 (29.2%) | 0/763 (0.0%) | 39/763 (5.1%) | 763/763 (100.0%) |
| [EC2 Instance Connect](crates/winterbaume-ec2instanceconnect/README.md) | `winterbaume-ec2instanceconnect` | awsJson1.1 | 2/2 (100.0%) | 0/2 (0.0%) | 1/2 (50.0%) | 0/2 (0.0%) | 0/2 (0.0%) | 0/2 (0.0%) |
| [ECR](crates/winterbaume-ecr/README.md) | `winterbaume-ecr` | awsJson1.1 | 58/58 (100.0%) | 0/58 (0.0%) | 29/58 (50.0%) | 0/58 (0.0%) | 11/58 (19.0%) | 58/58 (100.0%) |
| [ECS](crates/winterbaume-ecs/README.md) | `winterbaume-ecs` | awsJson1.1 | 63/76 (82.9%) | 1/76 (1.3%) | 45/76 (59.2%) | 0/76 (0.0%) | 12/76 (15.8%) | 76/76 (100.0%) |
| [EFS](crates/winterbaume-efs/README.md) | `winterbaume-efs` | restJson1 | 31/31 (100.0%) | 0/31 (0.0%) | 19/31 (61.3%) | 0/31 (0.0%) | 0/31 (0.0%) | 0/31 (0.0%) |
| [EKS](crates/winterbaume-eks/README.md) | `winterbaume-eks` | restJson1 | 55/64 (85.9%) | 4/64 (6.2%) | 17/64 (26.6%) | 0/64 (0.0%) | 8/64 (12.5%) | 64/64 (100.0%) |
| [Elastic Beanstalk](crates/winterbaume-elasticbeanstalk/README.md) | `winterbaume-elasticbeanstalk` | awsQuery | 7/47 (14.9%) | 0/47 (0.0%) | 0/47 (0.0%) | 0/47 (0.0%) | 7/47 (14.9%) | 0/47 (0.0%) |
| [ElastiCache](crates/winterbaume-elasticache/README.md) | `winterbaume-elasticache` | awsQuery | 24/75 (32.0%) | 0/75 (0.0%) | 17/75 (22.7%) | 0/75 (0.0%) | 7/75 (9.3%) | 75/75 (100.0%) |
| [ELB](crates/winterbaume-elasticloadbalancing/README.md) | `winterbaume-elasticloadbalancing` | awsQuery | 29/29 (100.0%) | 0/29 (0.0%) | 21/29 (72.4%) | 0/29 (0.0%) | 0/29 (0.0%) | 0/29 (0.0%) |
| [ELBv2](crates/winterbaume-elasticloadbalancingv2/README.md) | `winterbaume-elasticloadbalancingv2` | awsQuery | 50/51 (98.0%) | 1/51 (2.0%) | 33/51 (64.7%) | 0/51 (0.0%) | 28/51 (54.9%) | 51/51 (100.0%) |
| [EMR](crates/winterbaume-emr/README.md) | `winterbaume-emr` | awsJson1.1 | 54/60 (90.0%) | 2/60 (3.3%) | 26/60 (43.3%) | 0/60 (0.0%) | 0/60 (0.0%) | 0/60 (0.0%) |
| [EMR Containers](crates/winterbaume-emrcontainers/README.md) | `winterbaume-emrcontainers` | restJson1 | 23/23 (100.0%) | 0/23 (0.0%) | 8/23 (34.8%) | 0/23 (0.0%) | 0/23 (0.0%) | 0/23 (0.0%) |
| [EMR Serverless](crates/winterbaume-emrserverless/README.md) | `winterbaume-emrserverless` | restJson1 | 16/22 (72.7%) | 0/22 (0.0%) | 11/22 (50.0%) | 0/22 (0.0%) | 11/22 (50.0%) | 0/22 (0.0%) |
| [EventBridge](crates/winterbaume-eventbridge/README.md) | `winterbaume-eventbridge` | awsJson1.1 | 57/57 (100.0%) | 0/57 (0.0%) | 45/57 (78.9%) | 0/57 (0.0%) | 18/57 (31.6%) | 57/57 (100.0%) |
| [EventBridge Pipes](crates/winterbaume-pipes/README.md) | `winterbaume-pipes` | restJson1 | 10/10 (100.0%) | 0/10 (0.0%) | 9/10 (90.0%) | 0/10 (0.0%) | 10/10 (100.0%) | 10/10 (100.0%) |
| [Firehose](crates/winterbaume-firehose/README.md) | `winterbaume-firehose` | awsJson1.1 | 12/12 (100.0%) | 0/12 (0.0%) | 12/12 (100.0%) | 0/12 (0.0%) | 7/12 (58.3%) | 12/12 (100.0%) |
| [FIS](crates/winterbaume-fis/README.md) | `winterbaume-fis` | restJson1 | 8/26 (30.8%) | 0/26 (0.0%) | 5/26 (19.2%) | 0/26 (0.0%) | 0/26 (0.0%) | 0/26 (0.0%) |
| [Forecast](crates/winterbaume-forecast/README.md) | `winterbaume-forecast` | awsJson1.1 | 5/63 (7.9%) | 0/63 (0.0%) | 5/63 (7.9%) | 0/63 (0.0%) | 17/63 (27.0%) | 0/63 (0.0%) |
| [Free Tier](crates/winterbaume-freetier/README.md) | `winterbaume-freetier` | awsJson1.0 | 5/5 (100.0%) | 0/5 (0.0%) | 0/5 (0.0%) | 0/5 (0.0%) | 0/5 (0.0%) | 0/5 (0.0%) |
| [FSx](crates/winterbaume-fsx/README.md) | `winterbaume-fsx` | awsJson1.1 | 9/48 (18.8%) | 0/48 (0.0%) | 9/48 (18.8%) | 0/48 (0.0%) | 0/48 (0.0%) | 0/48 (0.0%) |
| [Glacier](crates/winterbaume-glacier/README.md) | `winterbaume-glacier` | restJson1 | 33/33 (100.0%) | 0/33 (0.0%) | 10/33 (30.3%) | 0/33 (0.0%) | 4/33 (12.1%) | 0/33 (0.0%) |
| [Glue](crates/winterbaume-glue/README.md) | `winterbaume-glue` | awsJson1.1 | 132/265 (49.8%) | 0/265 (0.0%) | 96/265 (36.2%) | 0/265 (0.0%) | 14/265 (5.3%) | 265/265 (100.0%) |
| [Greengrass](crates/winterbaume-greengrass/README.md) | `winterbaume-greengrass` | restJson1 | 71/92 (77.2%) | 0/92 (0.0%) | 55/92 (59.8%) | 0/92 (0.0%) | 0/92 (0.0%) | 0/92 (0.0%) |
| [GuardDuty](crates/winterbaume-guardduty/README.md) | `winterbaume-guardduty` | restJson1 | 85/87 (97.7%) | 2/87 (2.3%) | 12/87 (13.8%) | 0/87 (0.0%) | 0/87 (0.0%) | 0/87 (0.0%) |
| [IAM](crates/winterbaume-iam/README.md) | `winterbaume-iam` | awsQuery | 154/176 (87.5%) | 22/176 (12.5%) | 119/176 (67.6%) | 0/176 (0.0%) | 39/176 (22.2%) | 176/176 (100.0%) |
| [IAM Access Analyzer](crates/winterbaume-accessanalyzer/README.md) | `winterbaume-accessanalyzer` | restJson1 | 11/37 (29.7%) | 0/37 (0.0%) | 0/37 (0.0%) | 0/37 (0.0%) | 0/37 (0.0%) | 0/37 (0.0%) |
| [IAM Roles Anywhere](crates/winterbaume-rolesanywhere/README.md) | `winterbaume-rolesanywhere` | restJson1 | 28/30 (93.3%) | 2/30 (6.7%) | 0/30 (0.0%) | 0/30 (0.0%) | 0/30 (0.0%) | 0/30 (0.0%) |
| [Identity Store](crates/winterbaume-identitystore/README.md) | `winterbaume-identitystore` | awsJson1.1 | 17/19 (89.5%) | 0/19 (0.0%) | 14/19 (73.7%) | 0/19 (0.0%) | 0/19 (0.0%) | 19/19 (100.0%) |
| [Inspector2](crates/winterbaume-inspector2/README.md) | `winterbaume-inspector2` | restJson1 | 49/75 (65.3%) | 21/75 (28.0%) | 19/75 (25.3%) | 0/75 (0.0%) | 0/75 (0.0%) | 0/75 (0.0%) |
| [IoT](crates/winterbaume-iot/README.md) | `winterbaume-iot` | restJson1 | 103/272 (37.9%) | 0/272 (0.0%) | 100/272 (36.8%) | 0/272 (0.0%) | 0/272 (0.0%) | 0/272 (0.0%) |
| [IoT Data Plane](crates/winterbaume-iotdataplane/README.md) | `winterbaume-iotdataplane` | restJson1 | 8/8 (100.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) |
| [IVS](crates/winterbaume-ivs/README.md) | `winterbaume-ivs` | restJson1 | 30/40 (75.0%) | 5/40 (12.5%) | 6/40 (15.0%) | 0/40 (0.0%) | 0/40 (0.0%) | 0/40 (0.0%) |
| [Keyspaces](crates/winterbaume-keyspaces/README.md) | `winterbaume-keyspaces` | awsJson1.0 | 19/19 (100.0%) | 0/19 (0.0%) | 0/19 (0.0%) | 0/19 (0.0%) | 0/19 (0.0%) | 0/19 (0.0%) |
| [Kinesis](crates/winterbaume-kinesis/README.md) | `winterbaume-kinesis` | awsJson1.1 | 38/39 (97.4%) | 0/39 (0.0%) | 31/39 (79.5%) | 0/39 (0.0%) | 10/39 (25.6%) | 39/39 (100.0%) |
| [Kinesis Analytics V2](crates/winterbaume-kinesisanalyticsv2/README.md) | `winterbaume-kinesisanalyticsv2` | awsJson1.1 | 32/33 (97.0%) | 1/33 (3.0%) | 0/33 (0.0%) | 0/33 (0.0%) | 0/33 (0.0%) | 0/33 (0.0%) |
| [Kinesis Video](crates/winterbaume-kinesisvideo/README.md) | `winterbaume-kinesisvideo` | restJson1 | 32/32 (100.0%) | 0/32 (0.0%) | 0/32 (0.0%) | 0/32 (0.0%) | 0/32 (0.0%) | 0/32 (0.0%) |
| [Kinesis Video Archived Media](crates/winterbaume-kinesisvideoarchivedmedia/README.md) | `winterbaume-kinesisvideoarchivedmedia` | restJson1 | 6/6 (100.0%) | 0/6 (0.0%) | 3/6 (50.0%) | 0/6 (0.0%) | 0/6 (0.0%) | 0/6 (0.0%) |
| [KMS](crates/winterbaume-kms/README.md) | `winterbaume-kms` | awsJson1.1 | 53/54 (98.1%) | 0/54 (0.0%) | 40/54 (74.1%) | 0/54 (0.0%) | 22/54 (40.7%) | 54/54 (100.0%) |
| [Lake Formation](crates/winterbaume-lakeformation/README.md) | `winterbaume-lakeformation` | restJson1 | 19/61 (31.1%) | 1/61 (1.6%) | 20/61 (32.8%) | 0/61 (0.0%) | 0/61 (0.0%) | 0/61 (0.0%) |
| [Lambda](crates/winterbaume-lambda/README.md) | `winterbaume-lambda` | restJson1 | 85/85 (100.0%) | 0/85 (0.0%) | 46/85 (54.1%) | 0/85 (0.0%) | 23/85 (27.1%) | 85/85 (100.0%) |
| [Lex Models V2](crates/winterbaume-lexmodelsv2/README.md) | `winterbaume-lexmodelsv2` | restJson1 | 58/107 (54.2%) | 2/107 (1.9%) | 17/107 (15.9%) | 0/107 (0.0%) | 0/107 (0.0%) | 0/107 (0.0%) |
| [Macie2](crates/winterbaume-macie2/README.md) | `winterbaume-macie2` | restJson1 | 67/81 (82.7%) | 14/81 (17.3%) | 13/81 (16.0%) | 0/81 (0.0%) | 24/81 (29.6%) | 0/81 (0.0%) |
| [Managed Blockchain](crates/winterbaume-managedblockchain/README.md) | `winterbaume-managedblockchain` | restJson1 | 27/27 (100.0%) | 0/27 (0.0%) | 20/27 (74.1%) | 0/27 (0.0%) | 0/27 (0.0%) | 0/27 (0.0%) |
| [Marketplace Metering](crates/winterbaume-marketplacemetering/README.md) | `winterbaume-marketplacemetering` | awsJson1.1 | 4/4 (100.0%) | 0/4 (0.0%) | 0/4 (0.0%) | 0/4 (0.0%) | 0/4 (0.0%) | 0/4 (0.0%) |
| [MediaConnect](crates/winterbaume-mediaconnect/README.md) | `winterbaume-mediaconnect` | restJson1 | 21/82 (25.6%) | 0/82 (0.0%) | 18/82 (22.0%) | 0/82 (0.0%) | 0/82 (0.0%) | 0/82 (0.0%) |
| [MediaLive](crates/winterbaume-medialive/README.md) | `winterbaume-medialive` | restJson1 | 16/123 (13.0%) | 0/123 (0.0%) | 12/123 (9.8%) | 0/123 (0.0%) | 0/123 (0.0%) | 0/123 (0.0%) |
| [MediaPackage](crates/winterbaume-mediapackage/README.md) | `winterbaume-mediapackage` | restJson1 | 9/19 (47.4%) | 0/19 (0.0%) | 9/19 (47.4%) | 0/19 (0.0%) | 0/19 (0.0%) | 0/19 (0.0%) |
| [MediaPackage v2](crates/winterbaume-mediapackagev2/README.md) | `winterbaume-mediapackagev2` | restJson1 | 7/30 (23.3%) | 0/30 (0.0%) | 7/30 (23.3%) | 0/30 (0.0%) | 0/30 (0.0%) | 0/30 (0.0%) |
| [MediaStore](crates/winterbaume-mediastore/README.md) | `winterbaume-mediastore` | awsJson1.1 | 11/21 (52.4%) | 0/21 (0.0%) | 11/21 (52.4%) | 0/21 (0.0%) | 0/21 (0.0%) | 0/21 (0.0%) |
| [MediaStore Data](crates/winterbaume-mediastoredata/README.md) | `winterbaume-mediastoredata` | restJson1 | 5/5 (100.0%) | 0/5 (0.0%) | 4/5 (80.0%) | 0/5 (0.0%) | 0/5 (0.0%) | 0/5 (0.0%) |
| [MemoryDB](crates/winterbaume-memorydb/README.md) | `winterbaume-memorydb` | awsJson1.1 | 13/45 (28.9%) | 0/45 (0.0%) | 13/45 (28.9%) | 0/45 (0.0%) | 10/45 (22.2%) | 45/45 (100.0%) |
| [MQ](crates/winterbaume-mq/README.md) | `winterbaume-mq` | restJson1 | 23/24 (95.8%) | 1/24 (4.2%) | 19/24 (79.2%) | 0/24 (0.0%) | 6/24 (25.0%) | 0/24 (0.0%) |
| [MSK](crates/winterbaume-kafka/README.md) | `winterbaume-kafka` | restJson1 | 10/59 (16.9%) | 0/59 (0.0%) | 13/59 (22.0%) | 0/59 (0.0%) | 6/59 (10.2%) | 0/59 (0.0%) |
| [Neptune](crates/winterbaume-neptune/README.md) | `winterbaume-neptune` | awsQuery | 64/70 (91.4%) | 6/70 (8.6%) | 47/70 (67.1%) | 0/70 (0.0%) | 6/70 (8.6%) | 0/70 (0.0%) |
| [Network Firewall](crates/winterbaume-networkfirewall/README.md) | `winterbaume-networkfirewall` | awsJson1.0 | 79/79 (100.0%) | 0/79 (0.0%) | 5/79 (6.3%) | 0/79 (0.0%) | 0/79 (0.0%) | 0/79 (0.0%) |
| [Network Manager](crates/winterbaume-networkmanager/README.md) | `winterbaume-networkmanager` | restJson1 | 53/95 (55.8%) | 0/95 (0.0%) | 18/95 (18.9%) | 0/95 (0.0%) | 0/95 (0.0%) | 0/95 (0.0%) |
| [OpenSearch](crates/winterbaume-opensearch/README.md) | `winterbaume-opensearch` | restJson1 | 44/88 (50.0%) | 0/88 (0.0%) | 11/88 (12.5%) | 0/88 (0.0%) | 0/88 (0.0%) | 0/88 (0.0%) |
| [OpenSearch Ingestion](crates/winterbaume-osis/README.md) | `winterbaume-osis` | restJson1 | 10/22 (45.5%) | 0/22 (0.0%) | 13/22 (59.1%) | 0/22 (0.0%) | 0/22 (0.0%) | 0/22 (0.0%) |
| [OpenSearch Serverless](crates/winterbaume-opensearchserverless/README.md) | `winterbaume-opensearchserverless` | awsJson1.0 | 12/46 (26.1%) | 0/46 (0.0%) | 12/46 (26.1%) | 0/46 (0.0%) | 0/46 (0.0%) | 0/46 (0.0%) |
| [Organizations](crates/winterbaume-organizations/README.md) | `winterbaume-organizations` | awsJson1.1 | 60/63 (95.2%) | 3/63 (4.8%) | 41/63 (65.1%) | 0/63 (0.0%) | 11/63 (17.5%) | 63/63 (100.0%) |
| [Outposts](crates/winterbaume-outposts/README.md) | `winterbaume-outposts` | restJson1 | 13/37 (35.1%) | 0/37 (0.0%) | 0/37 (0.0%) | 0/37 (0.0%) | 0/37 (0.0%) | 0/37 (0.0%) |
| [Panorama](crates/winterbaume-panorama/README.md) | `winterbaume-panorama` | restJson1 | 10/34 (29.4%) | 1/34 (2.9%) | 0/34 (0.0%) | 0/34 (0.0%) | 0/34 (0.0%) | 0/34 (0.0%) |
| [Performance Insights](crates/winterbaume-pi/README.md) | `winterbaume-pi` | awsJson1.1 | 9/13 (69.2%) | 4/13 (30.8%) | 0/13 (0.0%) | 0/13 (0.0%) | 0/13 (0.0%) | 0/13 (0.0%) |
| [Personalize](crates/winterbaume-personalize/README.md) | `winterbaume-personalize` | awsJson1.1 | 66/71 (93.0%) | 5/71 (7.0%) | 4/71 (5.6%) | 0/71 (0.0%) | 0/71 (0.0%) | 0/71 (0.0%) |
| [Personalize Events](crates/winterbaume-personalizeevents/README.md) | `winterbaume-personalizeevents` | restJson1 | 5/5 (100.0%) | 0/5 (0.0%) | 0/5 (0.0%) | 0/5 (0.0%) | 0/5 (0.0%) | 0/5 (0.0%) |
| [Personalize Runtime](crates/winterbaume-personalizeruntime/README.md) | `winterbaume-personalizeruntime` | restJson1 | 3/3 (100.0%) | 0/3 (0.0%) | 0/3 (0.0%) | 0/3 (0.0%) | 0/3 (0.0%) | 0/3 (0.0%) |
| [Pinpoint](crates/winterbaume-pinpoint/README.md) | `winterbaume-pinpoint` | restJson1 | 15/122 (12.3%) | 0/122 (0.0%) | 12/122 (9.8%) | 0/122 (0.0%) | 0/122 (0.0%) | 0/122 (0.0%) |
| [Pinpoint SMS Voice](crates/winterbaume-pinpointsmsvoice/README.md) | `winterbaume-pinpointsmsvoice` | restJson1 | 4/8 (50.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) |
| [Polly](crates/winterbaume-polly/README.md) | `winterbaume-polly` | restJson1 | 9/10 (90.0%) | 0/10 (0.0%) | 5/10 (50.0%) | 0/10 (0.0%) | 0/10 (0.0%) | 0/10 (0.0%) |
| [Pricing](crates/winterbaume-pricing/README.md) | `winterbaume-pricing` | awsJson1.1 | 5/5 (100.0%) | 0/5 (0.0%) | 0/5 (0.0%) | 0/5 (0.0%) | 0/5 (0.0%) | 0/5 (0.0%) |
| [Private CA Connector for SCEP](crates/winterbaume-pcaconnectorscep/README.md) | `winterbaume-pcaconnectorscep` | restJson1 | 11/12 (91.7%) | 0/12 (0.0%) | 0/12 (0.0%) | 0/12 (0.0%) | 0/12 (0.0%) | 0/12 (0.0%) |
| [QuickSight](crates/winterbaume-quicksight/README.md) | `winterbaume-quicksight` | restJson1 | 68/232 (29.3%) | 0/232 (0.0%) | 31/232 (13.4%) | 0/232 (0.0%) | 0/232 (0.0%) | 0/232 (0.0%) |
| [RAM](crates/winterbaume-ram/README.md) | `winterbaume-ram` | restJson1 | 35/35 (100.0%) | 0/35 (0.0%) | 8/35 (22.9%) | 0/35 (0.0%) | 0/35 (0.0%) | 0/35 (0.0%) |
| [RDS](crates/winterbaume-rds/README.md) | `winterbaume-rds` | awsQuery | 146/164 (89.0%) | 4/164 (2.4%) | 85/164 (51.8%) | 0/164 (0.0%) | 12/164 (7.3%) | 164/164 (100.0%) |
| [RDS Data](crates/winterbaume-rdsdata/README.md) | `winterbaume-rdsdata` | restJson1 | 6/6 (100.0%) | 0/6 (0.0%) | 1/6 (16.7%) | 0/6 (0.0%) | 0/6 (0.0%) | 6/6 (100.0%) |
| [Recycle Bin](crates/winterbaume-rbin/README.md) | `winterbaume-rbin` | restJson1 | 9/10 (90.0%) | 0/10 (0.0%) | 0/10 (0.0%) | 0/10 (0.0%) | 0/10 (0.0%) | 0/10 (0.0%) |
| [Redshift](crates/winterbaume-redshift/README.md) | `winterbaume-redshift` | awsQuery | 100/141 (70.9%) | 3/141 (2.1%) | 35/141 (24.8%) | 0/141 (0.0%) | 7/141 (5.0%) | 0/141 (0.0%) |
| [Redshift Data](crates/winterbaume-redshiftdata/README.md) | `winterbaume-redshiftdata` | awsJson1.1 | 11/11 (100.0%) | 0/11 (0.0%) | 4/11 (36.4%) | 0/11 (0.0%) | 0/11 (0.0%) | 0/11 (0.0%) |
| [Rekognition](crates/winterbaume-rekognition/README.md) | `winterbaume-rekognition` | awsJson1.1 | 8/75 (10.7%) | 4/75 (5.3%) | 8/75 (10.7%) | 0/75 (0.0%) | 13/75 (17.3%) | 0/75 (0.0%) |
| [Resilience Hub](crates/winterbaume-resiliencehub/README.md) | `winterbaume-resiliencehub` | restJson1 | 22/63 (34.9%) | 0/63 (0.0%) | 17/63 (27.0%) | 0/63 (0.0%) | 17/63 (27.0%) | 0/63 (0.0%) |
| [Resource Groups](crates/winterbaume-resourcegroups/README.md) | `winterbaume-resourcegroups` | restJson1 | 22/23 (95.7%) | 1/23 (4.3%) | 15/23 (65.2%) | 0/23 (0.0%) | 0/23 (0.0%) | 23/23 (100.0%) |
| [Resource Groups Tagging](crates/winterbaume-resourcegroupstagging/README.md) | `winterbaume-resourcegroupstagging` | awsJson1.1 | 5/9 (55.6%) | 0/9 (0.0%) | 0/9 (0.0%) | 0/9 (0.0%) | 0/9 (0.0%) | 9/9 (100.0%) |
| [Route 53](crates/winterbaume-route53/README.md) | `winterbaume-route53` | restXml | 71/71 (100.0%) | 0/71 (0.0%) | 30/71 (42.3%) | 0/71 (0.0%) | 10/71 (14.1%) | 71/71 (100.0%) |
| [Route 53 Domains](crates/winterbaume-route53domains/README.md) | `winterbaume-route53domains` | awsJson1.1 | 5/34 (14.7%) | 0/34 (0.0%) | 0/34 (0.0%) | 0/34 (0.0%) | 0/34 (0.0%) | 0/34 (0.0%) |
| [Route 53 Recovery Cluster](crates/winterbaume-route53recoverycluster/README.md) | `winterbaume-route53recoverycluster` | awsJson1.0 | 4/4 (100.0%) | 0/4 (0.0%) | 0/4 (0.0%) | 0/4 (0.0%) | 0/4 (0.0%) | 0/4 (0.0%) |
| [Route 53 Resolver](crates/winterbaume-route53resolver/README.md) | `winterbaume-route53resolver` | awsJson1.1 | 28/68 (41.2%) | 0/68 (0.0%) | 28/68 (41.2%) | 0/68 (0.0%) | 11/68 (16.2%) | 0/68 (0.0%) |
| [S3](crates/winterbaume-s3/README.md) | `winterbaume-s3` | restXml | 103/107 (96.3%) | 4/107 (3.7%) | 73/107 (68.2%) | 51/107 (47.7%) | 44/107 (41.1%) | 107/107 (100.0%) |
| [S3 Control](crates/winterbaume-s3control/README.md) | `winterbaume-s3control` | restXml | 87/97 (89.7%) | 10/97 (10.3%) | 0/97 (0.0%) | 0/97 (0.0%) | 7/97 (7.2%) | 0/97 (0.0%) |
| [S3 Files](crates/winterbaume-s3files/README.md) | `winterbaume-s3files` | restJson1 | 21/21 (100.0%) | 0/21 (0.0%) | 0/21 (0.0%) | 0/21 (0.0%) | 0/21 (0.0%) | 0/21 (0.0%) |
| [S3 on Outposts](crates/winterbaume-s3outposts/README.md) | `winterbaume-s3outposts` | restJson1 | 3/5 (60.0%) | 1/5 (20.0%) | 0/5 (0.0%) | 0/5 (0.0%) | 0/5 (0.0%) | 0/5 (0.0%) |
| [S3 Tables](crates/winterbaume-s3tables/README.md) | `winterbaume-s3tables` | restJson1 | 46/49 (93.9%) | 3/49 (6.1%) | 14/49 (28.6%) | 0/49 (0.0%) | 12/49 (24.5%) | 0/49 (0.0%) |
| [S3 Vectors](crates/winterbaume-s3vectors/README.md) | `winterbaume-s3vectors` | restJson1 | 19/19 (100.0%) | 0/19 (0.0%) | 15/19 (78.9%) | 12/19 (63.2%) | 0/19 (0.0%) | 0/19 (0.0%) |
| [SageMaker](crates/winterbaume-sagemaker/README.md) | `winterbaume-sagemaker` | awsJson1.1 | 141/396 (35.6%) | 1/396 (0.3%) | 112/396 (28.3%) | 0/396 (0.0%) | 11/396 (2.8%) | 0/396 (0.0%) |
| [SageMaker Metrics](crates/winterbaume-sagemakermetrics/README.md) | `winterbaume-sagemakermetrics` | restJson1 | 2/2 (100.0%) | 0/2 (0.0%) | 1/2 (50.0%) | 0/2 (0.0%) | 0/2 (0.0%) | 0/2 (0.0%) |
| [SageMaker Runtime](crates/winterbaume-sagemakerruntime/README.md) | `winterbaume-sagemakerruntime` | restJson1 | 3/3 (100.0%) | 0/3 (0.0%) | 2/3 (66.7%) | 0/3 (0.0%) | 0/3 (0.0%) | 0/3 (0.0%) |
| [Savings Plans](crates/winterbaume-savingsplans/README.md) | `winterbaume-savingsplans` | restJson1 | 10/10 (100.0%) | 0/10 (0.0%) | 0/10 (0.0%) | 0/10 (0.0%) | 0/10 (0.0%) | 0/10 (0.0%) |
| [Scheduler](crates/winterbaume-scheduler/README.md) | `winterbaume-scheduler` | restJson1 | 12/12 (100.0%) | 0/12 (0.0%) | 12/12 (100.0%) | 0/12 (0.0%) | 9/12 (75.0%) | 12/12 (100.0%) |
| [Secrets Manager](crates/winterbaume-secretsmanager/README.md) | `winterbaume-secretsmanager` | awsJson1.1 | 22/23 (95.7%) | 1/23 (4.3%) | 21/23 (91.3%) | 0/23 (0.0%) | 11/23 (47.8%) | 23/23 (100.0%) |
| [Security Hub](crates/winterbaume-securityhub/README.md) | `winterbaume-securityhub` | restJson1 | 97/107 (90.7%) | 10/107 (9.3%) | 13/107 (12.1%) | 0/107 (0.0%) | 0/107 (0.0%) | 0/107 (0.0%) |
| [Service Catalog](crates/winterbaume-servicecatalog/README.md) | `winterbaume-servicecatalog` | awsJson1.1 | 4/90 (4.4%) | 0/90 (0.0%) | 0/90 (0.0%) | 0/90 (0.0%) | 0/90 (0.0%) | 0/90 (0.0%) |
| [Service Catalog AppRegistry](crates/winterbaume-servicecatalogappregistry/README.md) | `winterbaume-servicecatalogappregistry` | restJson1 | 23/24 (95.8%) | 1/24 (4.2%) | 0/24 (0.0%) | 0/24 (0.0%) | 0/24 (0.0%) | 0/24 (0.0%) |
| [Service Discovery](crates/winterbaume-servicediscovery/README.md) | `winterbaume-servicediscovery` | awsJson1.1 | 27/30 (90.0%) | 0/30 (0.0%) | 27/30 (90.0%) | 0/30 (0.0%) | 0/30 (0.0%) | 30/30 (100.0%) |
| [Service Quotas](crates/winterbaume-servicequotas/README.md) | `winterbaume-servicequotas` | awsJson1.1 | 5/26 (19.2%) | 0/26 (0.0%) | 2/26 (7.7%) | 0/26 (0.0%) | 8/26 (30.8%) | 0/26 (0.0%) |
| [SES v1](crates/winterbaume-ses/README.md) | `winterbaume-ses` | awsQuery | 38/71 (53.5%) | 2/71 (2.8%) | 38/71 (53.5%) | 0/71 (0.0%) | 6/71 (8.5%) | 14/71 (19.7%) |
| [SES v2](crates/winterbaume-sesv2/README.md) | `winterbaume-sesv2` | restJson1 | 106/110 (96.4%) | 4/110 (3.6%) | 30/110 (27.3%) | 0/110 (0.0%) | 15/110 (13.6%) | 0/110 (0.0%) |
| [Shield](crates/winterbaume-shield/README.md) | `winterbaume-shield` | awsJson1.1 | 9/36 (25.0%) | 0/36 (0.0%) | 9/36 (25.0%) | 0/36 (0.0%) | 0/36 (0.0%) | 0/36 (0.0%) |
| [Signer](crates/winterbaume-signer/README.md) | `winterbaume-signer` | restJson1 | 19/19 (100.0%) | 0/19 (0.0%) | 7/19 (36.8%) | 0/19 (0.0%) | 0/19 (0.0%) | 0/19 (0.0%) |
| [SimpleDB v2](crates/winterbaume-simpledbv2/README.md) | `winterbaume-simpledbv2` | restJson1 | 3/3 (100.0%) | 0/3 (0.0%) | 0/3 (0.0%) | 0/3 (0.0%) | 0/3 (0.0%) | 0/3 (0.0%) |
| [SimSpace Weaver](crates/winterbaume-simspaceweaver/README.md) | `winterbaume-simspaceweaver` | restJson1 | 15/16 (93.8%) | 0/16 (0.0%) | 0/16 (0.0%) | 0/16 (0.0%) | 0/16 (0.0%) | 0/16 (0.0%) |
| [Snow Device Management](crates/winterbaume-snowdevicemanagement/README.md) | `winterbaume-snowdevicemanagement` | restJson1 | 11/13 (84.6%) | 0/13 (0.0%) | 0/13 (0.0%) | 0/13 (0.0%) | 0/13 (0.0%) | 0/13 (0.0%) |
| [SNS](crates/winterbaume-sns/README.md) | `winterbaume-sns` | awsQuery | 41/42 (97.6%) | 1/42 (2.4%) | 33/42 (78.6%) | 0/42 (0.0%) | 15/42 (35.7%) | 42/42 (100.0%) |
| [SQS](crates/winterbaume-sqs/README.md) | `winterbaume-sqs` | awsJson1.0 | 23/23 (100.0%) | 0/23 (0.0%) | 20/23 (87.0%) | 0/23 (0.0%) | 16/23 (69.6%) | 23/23 (100.0%) |
| [SSM](crates/winterbaume-ssm/README.md) | `winterbaume-ssm` | awsJson1.1 | 127/146 (87.0%) | 19/146 (13.0%) | 41/146 (28.1%) | 0/146 (0.0%) | 10/146 (6.8%) | 146/146 (100.0%) |
| [SSM Quick Setup](crates/winterbaume-ssmquicksetup/README.md) | `winterbaume-ssmquicksetup` | restJson1 | 6/14 (42.9%) | 0/14 (0.0%) | 0/14 (0.0%) | 0/14 (0.0%) | 0/14 (0.0%) | 0/14 (0.0%) |
| [SSO](crates/winterbaume-sso/README.md) | `winterbaume-sso` | restJson1 | 4/4 (100.0%) | 0/4 (0.0%) | 0/4 (0.0%) | 0/4 (0.0%) | 0/4 (0.0%) | 0/4 (0.0%) |
| [SSO Admin](crates/winterbaume-ssoadmin/README.md) | `winterbaume-ssoadmin` | awsJson1.1 | 27/79 (34.2%) | 1/79 (1.3%) | 25/79 (31.6%) | 0/79 (0.0%) | 0/79 (0.0%) | 79/79 (100.0%) |
| [Step Functions](crates/winterbaume-sfn/README.md) | `winterbaume-sfn` | awsJson1.0 | 35/37 (94.6%) | 2/37 (5.4%) | 29/37 (78.4%) | 0/37 (0.0%) | 18/37 (48.6%) | 37/37 (100.0%) |
| [STS](crates/winterbaume-sts/README.md) | `winterbaume-sts` | awsQuery | 11/11 (100.0%) | 0/11 (0.0%) | 7/11 (63.6%) | 0/11 (0.0%) | 6/11 (54.5%) | 11/11 (100.0%) |
| [Support](crates/winterbaume-support/README.md) | `winterbaume-support` | awsJson1.1 | 5/16 (31.2%) | 1/16 (6.2%) | 5/16 (31.2%) | 0/16 (0.0%) | 0/16 (0.0%) | 0/16 (0.0%) |
| [Support App](crates/winterbaume-supportapp/README.md) | `winterbaume-supportapp` | restJson1 | 3/10 (30.0%) | 0/10 (0.0%) | 0/10 (0.0%) | 0/10 (0.0%) | 0/10 (0.0%) | 0/10 (0.0%) |
| [SWF](crates/winterbaume-swf/README.md) | `winterbaume-swf` | awsJson1.0 | 30/39 (76.9%) | 0/39 (0.0%) | 21/39 (53.8%) | 0/39 (0.0%) | 0/39 (0.0%) | 0/39 (0.0%) |
| [Synthetics](crates/winterbaume-synthetics/README.md) | `winterbaume-synthetics` | restJson1 | 22/22 (100.0%) | 0/22 (0.0%) | 4/22 (18.2%) | 0/22 (0.0%) | 0/22 (0.0%) | 0/22 (0.0%) |
| [Tax Settings](crates/winterbaume-taxsettings/README.md) | `winterbaume-taxsettings` | restJson1 | 0/16 (0.0%) | 0/16 (0.0%) | 0/16 (0.0%) | 0/16 (0.0%) | 0/16 (0.0%) | 0/16 (0.0%) |
| [Textract](crates/winterbaume-textract/README.md) | `winterbaume-textract` | awsJson1.1 | 6/25 (24.0%) | 0/25 (0.0%) | 5/25 (20.0%) | 0/25 (0.0%) | 0/25 (0.0%) | 0/25 (0.0%) |
| [Timestream InfluxDB](crates/winterbaume-timestreaminfluxdb/README.md) | `winterbaume-timestreaminfluxdb` | awsJson1.0 | 19/19 (100.0%) | 0/19 (0.0%) | 13/19 (68.4%) | 0/19 (0.0%) | 0/19 (0.0%) | 0/19 (0.0%) |
| [Timestream Query](crates/winterbaume-timestreamquery/README.md) | `winterbaume-timestreamquery` | awsJson1.0 | 15/15 (100.0%) | 0/15 (0.0%) | 6/15 (40.0%) | 0/15 (0.0%) | 0/15 (0.0%) | 0/15 (0.0%) |
| [Timestream Write](crates/winterbaume-timestreamwrite/README.md) | `winterbaume-timestreamwrite` | awsJson1.0 | 19/19 (100.0%) | 0/19 (0.0%) | 15/19 (78.9%) | 0/19 (0.0%) | 0/19 (0.0%) | 0/19 (0.0%) |
| [Transcribe](crates/winterbaume-transcribe/README.md) | `winterbaume-transcribe` | awsJson1.1 | 16/43 (37.2%) | 0/43 (0.0%) | 16/43 (37.2%) | 0/43 (0.0%) | 0/43 (0.0%) | 0/43 (0.0%) |
| [Transfer](crates/winterbaume-transfer/README.md) | `winterbaume-transfer` | awsJson1.1 | 44/71 (62.0%) | 0/71 (0.0%) | 14/71 (19.7%) | 0/71 (0.0%) | 0/71 (0.0%) | 0/71 (0.0%) |
| [Trusted Advisor](crates/winterbaume-trustedadvisor/README.md) | `winterbaume-trustedadvisor` | restJson1 | 6/11 (54.5%) | 4/11 (36.4%) | 0/11 (0.0%) | 0/11 (0.0%) | 0/11 (0.0%) | 0/11 (0.0%) |
| [VPC Lattice](crates/winterbaume-vpclattice/README.md) | `winterbaume-vpclattice` | restJson1 | 66/73 (90.4%) | 2/73 (2.7%) | 35/73 (47.9%) | 0/73 (0.0%) | 0/73 (0.0%) | 0/73 (0.0%) |
| [WAFv2](crates/winterbaume-wafv2/README.md) | `winterbaume-wafv2` | awsJson1.1 | 38/55 (69.1%) | 0/55 (0.0%) | 29/55 (52.7%) | 0/55 (0.0%) | 0/55 (0.0%) | 55/55 (100.0%) |
| [WorkSpaces](crates/winterbaume-workspaces/README.md) | `winterbaume-workspaces` | awsJson1.1 | 50/91 (54.9%) | 0/91 (0.0%) | 16/91 (17.6%) | 0/91 (0.0%) | 0/91 (0.0%) | 0/91 (0.0%) |
| [WorkSpaces Web](crates/winterbaume-workspacesweb/README.md) | `winterbaume-workspacesweb` | restJson1 | 68/75 (90.7%) | 0/75 (0.0%) | 27/75 (36.0%) | 0/75 (0.0%) | 0/75 (0.0%) | 0/75 (0.0%) |
| [X-Ray](crates/winterbaume-xray/README.md) | `winterbaume-xray` | restJson1 | 34/38 (89.5%) | 4/38 (10.5%) | 0/38 (0.0%) | 0/38 (0.0%) | 6/38 (15.8%) | 0/38 (0.0%) |

**winterbaume: 7210 / 11367 operations across 224 services (63.4%)**

**winterbaume stubs: 333 / 11367 operations across 224 services (2.9%) - routed but return empty/default responses**

**moto: 3296 / 11367 operations across 224 services (29.0%)**

**floci: 135 / 11367 operations across 224 services (1.2%)**

**kumo: 874 / 11367 operations across 224 services (7.7%)**

**fakecloud: 3980 / 11367 operations across 224 services (35.0%)**

## Terraform State Converter Coverage

Winterbaume includes a Terraform state converter layer (`winterbaume-terraform`) that can inject Terraform state into the emulator and extract it back. This enables seeding mock environments from existing Terraform state files, and round-tripping state for validation.

**267 Terraform resource types** across **137 services** are supported, covering **67.4% of inject attributes** and **59.7% of extract attributes** against the official Terraform AWS provider schema.

| Rating | Count | Criteria |
|--------|-------|----------|
| Excellent | 265 | inject >= 60% AND extract >= 50% |
| Good | 0 | inject >= 40% OR extract >= 30% |
| N/A | 2 | No TF provider schema entry |

See the [full Terraform converter coverage report](docs/reference/terraform.md) for per-resource details.

## Usage

### Library mode (in-process)

```rust
use winterbaume_core::MockAws;
use winterbaume_sts::StsService;
use winterbaume_iam::IamService;
use winterbaume_s3::S3Service;

let mock = MockAws::builder()
    .with_service(StsService::new())
    .with_service(IamService::new())
    .with_service(S3Service::new())
    .build();

let config = aws_config::defaults(BehaviorVersion::latest())
    .http_client(mock.http_client())
    .credentials_provider(mock.credentials_provider())
    .region("us-east-1")
    .load()
    .await;

let sts = aws_sdk_sts::Client::new(&config);
let resp = sts.get_caller_identity().send().await.unwrap();
assert_eq!(resp.account(), Some("123456789012"));
```

### Standalone HTTP server

For non-Rust clients, `winterbaume-server` provides a standalone HTTP server:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

Point the AWS SDK at it:

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws s3 mb s3://my-bucket
```

## Architecture

- **`winterbaume-core`** -- shared infrastructure: `MockAws` builder, `MockService` trait, per-account/region state management, HTTP client/connector bridge.
- **`winterbaume-{service}`** -- each service crate implements `MockService` with its own protocol handling and in-memory state.
- **`winterbaume-server`** -- hyper-based HTTP server that registers all service backends.

State is partitioned per account ID and region, matching real AWS isolation semantics.

## FAQ

### What is the difference between Winterbaume and moto?

Winterbaume targets the Rust AWS SDK directly. It provides an in-process HTTP client for `aws-sdk-rust`, so tests can run without standing up an external mock service. The project also tracks moto coverage and behaviour closely, and vendors moto as a reference implementation.

### When should I use library mode instead of the standalone server?

Use library mode when you are writing Rust tests against `aws-sdk-rust` and want the fastest setup with no network I/O. Use `winterbaume-server` when you need to point non-Rust clients, CLIs, or external processes at a mock AWS endpoint over HTTP.

### What is the difference between an operation and a stub?

The `Operations` column counts handlers with real, state-backed logic: they parse the request, mutate or query the in-memory backend state, validate inputs, and produce an AWS-shaped response. The `Stubs` column counts handlers that route the request and immediately return an empty or default response without consulting state. Stubs are still useful for keeping SDK calls from erroring out ( e.g. callers that issue a `Describe*` operation purely as a precondition check ), but they will not reflect any prior `Create*` or `Update*` you made on the same mock. Stubs are either annotated explicitly with `// STUB[<category>]: <reason>` in the handler source, or detected heuristically by the coverage script when the body is short and free of state access.

### What happens if an operation is not implemented yet?

If the operation appears in the `Operations` column it has real behaviour. If it appears in the `Stubs` column it returns an empty or default response and does not interact with state. If it appears in neither column the request is unrouted and the service returns a 501-style "not implemented" error.

### Why are there so many stubs?

Some AWS APIs depend on infrastructure that does not exist inside an emulator: instance telemetry ( EC2 monitoring, CPU credit specs ), real-time message delivery, multi-account Organizations integration, billing pipelines, etc. Rather than fail noisily on these calls, winterbaume returns an empty default so a workflow that touches them as part of a larger setup does not break. The stub count is shown next to every service so you can assess this trade-off before depending on a particular call.

### Does Winterbaume keep state between requests?

Yes. State is stored in memory and isolated by account ID and region so repeated SDK calls can observe changes made by earlier requests in the same mock environment.

### What is Winterbaume named after?

It is a multilingual pun built around the reference project name `moto`. "Winter tree" is the literal reading, but the real joke is the sound and imagery of a Japanese pop-culture name hiding in plain sight.

## Limitations

The following are known limitations of the current implementation.

**Partial operation coverage for some services.** Several service crates cover only a subset of the total API surface. Unimplemented operations return `501 Not Implemented`. The coverage table above shows the exact count for each service.

**No real execution engines for compute-style operations.** Operations that require an actual evaluation engine — such as Change Calendar evaluation, SSM Automation execution, or Bedrock model invocation — are marked `STUB[no-engine]` and return plausible empty or default responses rather than computed results.

**No real telemetry-driven responses.** Operations whose responses depend on live infrastructure metrics or access histories — for example, IAM credential report generation or patch compliance queries — are marked `STUB[no-telemetry]` and return synthetic or empty data.

**Cross-account and AWS Organizations integration is limited.** Operations that require real AWS Organizations cross-account state (member accounts, SCPs, delegated administrators) are marked `STUB[org-integration]` and return minimal responses. The mock environment is single-account by default.

**No persistence across server restarts.** All state is stored in process memory. Stopping `winterbaume-server` (or dropping a `MockAws` instance) discards all resources. Durable persistence for services such as SSM Parameter Store or Secrets Manager is a known open item.

**Athena-Glue catalogue integration is not wired.** In real AWS, Athena's default data catalogue is the Glue Data Catalogue. Winterbaume accepts `DataCatalogType::Glue` at the CRUD level but does not resolve Glue-managed table metadata during query execution. SQL queries against Glue-backed catalogues will not see schema from the mock Glue state.

**IAM policy simulation fidelity is incomplete.** `SimulateCustomPolicy` and `SimulatePrincipalPolicy` use `winterbaume-iam-rule-eval`, which covers allow, explicit deny, and implicit deny paths. However, richer condition key resolution, permissions boundaries, and full principal-context matching are not yet at full AWS parity.

**`StatefulService::snapshot()` is non-fallible.** Blob read failures (for example, from `S3`, `EBS`, or `Glacier` large-object payloads) are logged but cannot be returned as errors to the caller. State snapshots that encounter blob read failures will be incomplete rather than returning an error.

**Partial rpc-v2-cbor serialisation coverage.** CloudWatch uses the rpc-v2-cbor protocol. Typed output serialisers do not yet cover every timestamp shape the SDK expects, particularly list-of-timestamp paths that require Tag 1-aware response shaping.

**Cross-service integrations follow documented AWS seams only.** Behaviour that couples one AWS service to another (Lambda event sources, EventBridge targets, Step Functions optimised integrations, AppSync data sources) is modelled on documented AWS contracts. Ad hoc or undocumented cross-service interactions are not emulated.

## License

Apache-2.0. See [LICENSE](./LICENSE) for the full text.

This project uses [moto](https://github.com/getmoto/moto) (Copyright 2012 Steve Pulec, Apache-2.0) as a reference implementation. See [NOTICE](./NOTICE) for third-party attribution details.
