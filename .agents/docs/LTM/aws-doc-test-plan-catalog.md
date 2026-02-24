# AWS Doc Test Plan Catalog

## Summary

The journal contains a large body of AWS-doc-driven test planning that complements moto parity work. These entries capture service summaries, implemented-operation inventories, expected behaviours, and candidate test scenarios for services where moto coverage is partial, absent, or not the right source of truth.

## Key Facts

- Docs-driven plans became the default fallback when moto coverage was missing, weak, or nonexistent.
- The plans consistently record four useful inputs: source URLs, service summary, implemented operations, and scenario tables with expected outcomes.
- Many plans were later executed immediately and reported test counts plus any discovered bugs.
- The catalog is especially useful for services with low operational complexity but many exact response-shape or error-shape requirements.
- The workflow continued after the big 2026-03-28 batch with one-off plans for larger or newer services such as EC2, S3 Tables, and S3 Vectors.
- Early-April follow-up plans expanded the same pattern to X-Ray, S3 Control, RDS, App Runner, Audit Manager, Macie2, and Lex.
- The same docs-driven pattern then scaled into late 2026-04-03 and 2026-04-04 coverage waves across Redshift, Lex v2, Timestream Query, Redshift Data, Cognito Identity, STS, RDS Data, AppConfig, ECR, WorkSpaces, SSM, CodeCommit, NetworkManager, WorkSpacesWeb, EKS, Lambda, CloudWatch, BedrockAgent, Cognito IDP, Transfer, CloudWatch Logs, and later EMR, CloudFormation, EFS, Batch, VPC Lattice, ECS, Organizations, GuardDuty, and Inspector2.

## Details

### Main families of planned services

The journal's docs-driven planning concentrated in these groups:

- Data and analytics: OpenSearch, Athena, Timestream Query, Timestream Write, Timestream InfluxDB, DynamoDB Streams, RDS Data.
- Identity and account: Organizations, SSO, CodeCommit, Account, ACM, ACM PCA.
- Networking and routing: Route53, Route53 Domains, VPC Lattice, Direct Connect, App Mesh.
- Messaging and integration: Pipes, SNS, SQS, Transfer, Shield.
- Storage and secrets: S3, Secrets Manager, SDB.
- Contact center and campaign tooling: Connect and Connect Campaigns.
- ML and application platforms: Personalize, SageMaker Metrics, AppConfig, AppSync.
- Terraform-specific regression planning: DynamoDB, S3, SQS, SNS, CloudWatch Logs fixes discovered by E2E work.

### Durable structure for future plans

A useful docs-driven plan in this repository should include:

- AWS doc source links
- a short service summary in project terms
- the exact operations currently implemented in Winterbaume
- a table of candidate tests with explicit expected outcomes
- notes about known omissions, optional fields, idempotency rules, and validation rules

That structure makes it easy to go from planning to implementation without re-reading the whole service API reference.

### Reusable scenario-test archetypes

The later EC2 scenario expansion added four high-value patterns that generalise beyond EC2 and should be considered when enhancing other services' `scenario_test.rs` coverage or service dossier usage scenarios:

- **Mutable binding failover**: move a stable external identifier or endpoint from one backing resource to another, then verify the allocation survives, a fresh association/binding ID is minted when AWS would do so, and the read API points only at the replacement target. EC2's Elastic IP canary failover is the concrete example; analogous candidates include load balancer target attachments, DNS records, event targets, subscriptions, and service-network associations.
- **Account or service defaults propagating into later creates**: toggle an account-level setting, create a resource without the per-resource field, and verify the default is applied to newly created resources rather than only being visible through the toggle's get/enable APIs. EC2 EBS encryption-by-default is the concrete example; analogous candidates include KMS defaults, logging defaults, encryption settings, retention defaults, and regional account settings.
- **Association replacement with ownership transfer**: replace a resource association, verify the old owner no longer lists the child, the new owner does list it, and any generated association ID returned by the replacement API is reflected by subsequent reads. EC2 network ACL subnet takeover is the concrete example; analogous candidates include route table associations, security policy attachments, group memberships, web ACL associations, and folder/resource memberships.
- **Full state-machine walks**: drive one resource through all common state transitions, checking each transition response's previous/current state against read-side state between calls. EC2 instance start/stop/reboot/terminate is the concrete example; analogous candidates include jobs, executions, streams, workspaces, clusters, channels, deployments, meetings, simulations, and migration tasks.

The follow-up service-dossier enhancement propagated these archetypes into non-EC2 `Possible Usage Scenarios` sections using operation and resource names as cues. Keep future refinements concrete: prefer documented association, attachment, membership, target, account-default, and lifecycle concepts over generic mutate/read pairs.

### Cross-call invariant inventory

The EC2 scenario pass turned invariant inventory into an explicit `/write-tests` artefact. For stateful services, the plan should identify these rows before scenario design:

| Category | What to look for |
|----------|------------------|
| Toggle propagation | Enable/disable or account-setting APIs that later create APIs must observe. |
| Modify rewrites sibling state | A parent default, alias, active version, or selected target that also appears on child records. |
| Per-call uniqueness | IDs minted by repeated association, allocation, execution, or task calls. |
| Default inheritance from parent | Create calls that inherit size, encryption, policy, tags, profile, or configuration from an upstream resource. |
| Lifecycle state transitions | Resources with explicit state machines that should be checked between calls. |
| Cross-resource references on read | Reads that must include or resolve linked subnets, routes, groups, attachments, targets, or policies. |

Every row should either map to at least one scenario or carry `N/A - <reason>`. This inventory catches bugs that single-operation round-trip tests cannot see.

### How these plans relate to implementation

Many entries in late March 2026 follow a stable pattern:

1. write a docs-driven plan in `JOURNAL.md`
2. implement the tests
3. note the resulting test count and any bugs found

This makes the plan sections reusable as both:

- future implementation checklists
- historical records of what behaviour was considered important

### 2026-03-28 batch: write-tests skill run

A large batch run on 2026-03-28 applied the `write-tests` skill to ~40 service crates with low test coverage, running 13 agents in parallel per wave. Each agent produced a docs-driven test plan, implemented tests, and reported bugs. Key results:

- **Batch 1**: sagemakermetrics (+4), tagging (+14), dynamodbstreams (+10), account (+4), signer (+5), codecommit (+6), sso (+8), servicecatalogappregistry (+11)
- **Batch 2**: costexplorer (+13), sagemakerruntime (+12), ec2instanceconnect (+6), applicationautoscaling (+12), timestreamquery (+12), rdsdata (+11), connect (+14), connectcampaigns (+17)
- **Batch 3**: ivs (+9), appmesh (+9), pinpoint (+13), workspaces (+16), budgets (+7), cloudhsmv2 (+8), osis (+11), redshiftdata (+14), timestreaminfluxdb (+16), forecast (+6), networkfirewall (+8), rekognition (+10), resourcegroups (+17)
- **Batch 4**: scheduler (+15), servicecatalog (+7), codepipeline (+10), ebs (+7), kinesisanalyticsv2 (+5), pipes (+12), sts (+16), ds (+8), dsql (+8), route53domains (+9), xray (+2), iotdataplane (+10), polly (+16)
- **Batch 5**: sdb (+8), support (+14), kinesisvideo (+6), mediapackagev2 (+8), shield (+8)

15 handler/state bugs were discovered and fixed during this batch (see JOURNAL.md "Session summary: write-tests batch run" for the full list). 4 behavioural gaps were documented with TODO comments but not fixed.

### Representative planned services

Important services with explicit docs-driven planning in the journal include:

- OpenSearch
- Organizations
- DynamoDB Streams
- Tagging
- CodeCommit
- SSO
- Timestream Query
- Connect
- Connect Campaigns
- S3
- Route53
- Route53 Domains
- Secrets Manager
- Pipes
- Personalize
- RDS Data
- SDB
- SageMaker Metrics
- EC2 Instance Connect
- Signer
- SageMaker Runtime
- Application Auto Scaling
- App Mesh
- IVS
- Pinpoint
- Account
- Cost Explorer
- EC2
- S3 Tables
- S3 Vectors
- X-Ray
- S3 Control
- RDS
- App Runner
- Audit Manager
- Macie2
- Lex

### Early-April 2026 follow-up plans and expansions

The 2026-04-03 journal entries show the docs-driven workflow scaling from planning-only notes into large same-session regression expansions:

- **X-Ray** paired a 25-scenario AWS-doc plan with an expansion from 48 to 65 integration tests. The durable behaviours were dual name-or-ARN addressing for groups and sampling rules, optimistic concurrency on resource policies via `PolicyRevisionId`, and a clear split between stateful resource operations and empty-default stubs.
- **S3 Control** paired a 31-scenario plan with a jump from 15 to 72 passing tests. The key lesson is that docs-driven plans must also record SDK endpoint reality: directory-bucket scope APIs route to `s3express-control`, and outposts bucket APIs route to `s3-outposts`, so some cases were intentionally skipped rather than misclassified as missing service behaviour.
- **RDS** expanded to 83 tests across 16 resource families. The durable note is that some XML response members are not surfaced by the SDK, so high-value docs-driven tests may need relaxed assertions even when the service implementation is correct.
- **App Runner**, **Audit Manager**, and **Macie2** used smaller follow-up plans to close obvious lifecycle and list-path gaps while tracking the exact new operations the pass covered.
- **Lex** expanded from 24 to 90 tests and documented a concrete request-shape limitation: the SDK sends `tagKeys` as query parameters for `UntagResource`, while the current handler reads them from the JSON body.

### Late 2026-04-03 docs-driven coverage waves

The next set of journal entries turned docs-driven planning into high-volume same-session implementation work across many services.

Representative one-off expansions:

- **Redshift** grew to 190 tests, with the durable behavioural note that `delete_resource_policy` is idempotent even for missing policies. Two happy paths remained blocked by wire-shape bugs in `modify_cluster_parameter_group` and `cancel_resize`.
- **Lex v2** grew to 147 tests. The durable caveats are that `delete_bot` and `delete_bot_locale` leave some slot, slot-type, and custom-vocabulary entries orphaned in internal state, and bot-version numbering remains max-based rather than gap-filling.
- **Timestream Query**, **Redshift Data**, **RDS Data**, **Cognito Identity**, and **CodeCommit** were representative cases where a compact AWS-doc plan could drive a service all the way to full implemented-operation coverage.
- **Cognito Identity** also established a behavioural constraint: several developer-identity operations are still fixed-response stubs, so tests should assert those empty or success defaults rather than inventing stateful semantics.

Broader late-wave batches covered many medium and large services in parallel:

- **AppConfig**, **ECR**, **WorkSpaces**, **SSM**, **NetworkManager**, **WorkSpacesWeb**, **EKS**, **BedrockAgent**, **Cognito IDP**, **Transfer**, and **CloudWatch Logs** all followed the same plan -> implement -> validate rhythm
- **CloudWatch** and **Lambda** showed that docs-driven tests can expose SDK-shape mismatches rather than pure state bugs; the tests are still valuable because they document the unreachable or partially serialised operations precisely
- **STS** exposed the same class of issue on awsQuery paths: nested or renamed request members can make a handler untestable from the SDK even when the route exists

### 2026-04-04 enhancement wave

The 2026-04-04 enhancement pass reused the same docs-driven structure, but aimed at services that already had moderate coverage:

- **EMR**, **CloudFormation**, **EFS**, and **Batch** reached full implemented-operation coverage
- **StepFunctions**, **VPC Lattice**, **ECS**, **Organizations**, **GuardDuty**, **Inspector2**, and **Lambda** all received another layer of lifecycle and error-path coverage
- **Autoscaling** and **EC2** were the most important contrast cases: autoscaling needed a new comprehensive suite from near-zero, while EC2 needed another deep expansion over an already large surface

The durable lesson is that docs-driven plans are not only for low-coverage services. They remain useful in the late stages when the remaining gaps are skewed towards error cases, alternate identifiers, cascade behaviour, and awkward request or response shapes.

## Files

- `.agents/docs/JOURNAL.md`: the source of the plans and execution notes.
- `crates/winterbaume-*/tests/integration_test.rs`: where the planned SDK tests are implemented.
- `tests/e2e/terraform/*.rs`: where Terraform-oriented plans become end-to-end test modules.

## Test Coverage

Many of these plans were later executed immediately, with recorded outcomes such as:

- added tests for account, ACM, ACM PCA, AppConfig, Athena, Backup, CloudHSM V2, and many others on 2026-03-26
- late-March 2026 service plans for Route53, Route53 Domains, Secrets Manager, RDS Data, Transfer, Shield, S3, Pipes, and related services
- dedicated `FIX(terraform-e2e)` regression tests on DynamoDB, S3, SQS, SNS, and CloudWatch Logs
- later one-off plans added 22 EC2 tests, 13 S3 Tables tests, and 16 S3 Vectors tests, with the S3 Vectors pass finding default-value and delete-conflict bugs
- the 2026-04-03 follow-up wave added or expanded suites for X-Ray (65 tests), S3 Control (72 tests), RDS (83 tests), App Runner (21 tests), Audit Manager (17 tests), Macie2 (51 tests), and Lex (90 tests)
- the later 2026-04-03 waves pushed Redshift to 190 tests, Lex v2 to 147, WorkSpaces to 56, SSM to 85, CodeCommit to 36, EKS to 100, BedrockAgent to 50, and many other medium services into the 70 to 100 percent range
- the 2026-04-04 enhancement pass then completed or deepened suites for EMR (38 operations covered), CloudFormation (43), EFS (31), Batch (39), StepFunctions (35), VPC Lattice (64), ECS (59), Organizations (55), GuardDuty (36), Inspector2 (58), Lambda (60), Autoscaling (52), and EC2 (76)
- the 2026-05-02 EC2 scenario layer later grew from 8 to 12 workflows by adding Elastic IP failover re-association, EBS encryption-by-default propagation, subnet network ACL takeover, and full instance state-machine coverage; those patterns are reusable across services with mutable bindings, account defaults, association replacement, and explicit lifecycle states

## Pitfalls

- AWS docs describe intended service behaviour, but SDK behaviour, Terraform-provider behaviour, and moto behaviour can still expose extra constraints.
- Docs-driven tests can legitimately end by documenting an SDK or wire-shape mismatch rather than by asserting a happy path. Keep that incompatibility in the record instead of silently dropping the case.
- Some planned scenarios depend on features the mock does not yet implement. Keep those tests clearly separated from currently runnable coverage.
- If a service already has strong moto parity coverage, do not duplicate the same cases without a docs-specific reason.
- Record endpoint-routing and request-shape mismatches explicitly. They are durable compatibility notes, not noise to omit from the plan summary.
- Do not let a create/delete lifecycle test substitute for an invariant scenario. The invariant is usually in the second or third call's read-side state.
