# Cross-Service State Coherence

## Summary

Some AWS APIs are separate protocol surfaces over the same backend state, while others are genuinely separate services. Winterbaume should model the real AWS coupling: shared-backend pairs must observe one another's state, data planes must validate against their control-plane siblings, and intentionally separate v1/v2 services must stay separate.

## Key Facts

- `ses` and `sesv2` are the highest-priority shared-backend pair: identities, configuration sets, templates, suppression lists, dedicated IP pools, and account-level settings are shared in real AWS.
- `mediapackage`/`mediapackagev2` and `waf`/`wafv2` are intentionally separate; do not introduce cross-API visibility there.
- API Gateway v1/v2 and ELB v1/v2 have mostly separate resources, but each pair shares one account+region namespace for custom domain names or load balancer names.
- Data-plane crates should normally validate resources against the corresponding control-plane crate before accepting calls.
- `winterbaume-dynamodbstreams` is the reference derived-service pattern: stream existence and payloads come from DynamoDB, while iterator bookkeeping stays local.

## Details

### v1/v2 API Pairs

| Pair | Real AWS coupling | Winterbaume action |
|------|-------------------|--------------------|
| `ses` / `sesv2` | Same backend, same SDK slug, same `ses` ARN namespace. v2 list APIs can see resources created through v1. | Merge state family by family. Start with identities, then configuration sets, templates, suppression list, dedicated IP pools, and account-level settings. Receipt rule sets stay v1-only; v2-only contact lists, import/export jobs, multi-region endpoints, tenants, deliverability reports, and reputation entities stay v2-only. |
| `mediapackage` / `mediapackagev2` | Separate services. AWS explicitly says v2 cannot access v1 resources and there is no automated migration. | Keep state separate. |
| `waf` / `wafv2` | Separate services. WAFv2 is a rewrite; migration is through a CloudFormation template returned by `CreateWebACLMigrationStack`. | Keep state separate. Do not directly populate WAFv2 state from WAF Classic migration. |
| `apigateway` / `apigatewayv2` | REST APIs and HTTP/WebSocket APIs are separate, but custom domain names share one namespace. | Add a cross-crate domain-name reservation so the same domain cannot be created through both APIs. |
| `elasticloadbalancing` / `elasticloadbalancingv2` | Classic ELB and ALB/NLB/GLB resources are separate, but load balancer names share one namespace. | Add a cross-crate name uniqueness check. |

### Control Plane and Data Plane Pairs

| Pair | Current gap | Priority |
|------|-------------|----------|
| `appconfig` / `appconfigdata` | `GetLatestConfiguration` returns empty payloads instead of deployed configuration from `StartDeployment`. | High |
| `mediastore` / `mediastoredata` | Data plane stores objects in one global map with no container key, so objects leak across containers and control-plane deletion has no object-aware behaviour. | High |
| `sagemaker` / `sagemakerruntime` | Runtime auto-creates endpoints on first invocation; real AWS rejects unknown endpoints. | High |
| `kinesisvideo` / `kinesisvideoarchivedmedia` | Archived media auto-creates streams with mock fragments; real AWS rejects unknown streams. | High |
| `cloudtrail` / `cloudtraildata` | Data plane accepts any `channel_arn` without checking CloudTrail channels. | High |
| `rds` / `rdsdata` | RDS Data is a bring-your-own-result mock and does not validate clusters or Data API enablement. | Low, until a workflow needs parity validation. |
| `redshift` / `redshiftdata` | Redshift Data owns a mock catalogue and does not validate clusters or workgroups. | Low, until a workflow needs parity validation. |

### Network Resource Stub Semantics

The 2026-05-02 network-aware service audit documented current EC2/VPC-like behaviour in 54 service dossiers under `## Current Network Resource Stub Semantics`. EC2 is currently the only crate with in-service VPC and network maps; those maps are not a shared network oracle for the rest of the workspace.

Most network-aware services store EC2 identifiers as opaque service-local strings, synthesise placeholder IDs, or leave VPC-specific operations as explicit 501 stubs. Creating a subnet, security group, ENI, VPC endpoint, or VPC link in `winterbaume-ec2` generally does not make another crate validate against it or create back-references in EC2 state.

Representative patterns:

- API Gateway and API Gateway V2 store VPC links and VPC endpoint IDs locally without validating target load balancer ARNs, VPC endpoints, subnets, or security groups.
- ELBv2 synthesises placeholder networking such as `vpc-12345678`, `subnet-aaaa1111`, and `sg-12345678`; `SetSubnets` and `SetSecurityGroups` only mutate local load balancer fields.
- EFS, S3 Files, S3 Outposts, EC2 Instance Connect, MediaConnect, and related services mint ENI-looking IDs locally rather than inserting network interfaces into EC2 state.
- RDS, Redshift, DMS, ElastiCache, MemoryDB, Neptune, MSK, EMR, EMR Serverless, Timestream InfluxDB, and SageMaker keep subnet, security-group, and VPC references as service-local metadata.
- Route 53, Route 53 Resolver, Service Discovery, Network Firewall, Network Manager, OpenSearch, OpenSearch Serverless, S3 Control, RAM, and VPC Lattice maintain association or endpoint maps without EC2 ownership checks.
- App Runner, DAX subnet groups, DSQL VPC endpoint service names, and several endpoint list/update/delete paths remain explicit not-implemented stubs.

This was documentation-only work, not a state-model change. Treat the dossier sections as the current contract for stub behaviour until a specific cross-service validation project changes it.

### Shared-Slug Heuristic

Same AWS SDK slug is a strong signal that two Winterbaume crates may need shared state. SES is the concrete current example. Future crates such as `pinpoint-email` and `mailmanager` also use the `ses` SDK slug in real AWS and should join the SES shared-backend design if they are implemented.

Multiple crates around a service family also deserve review even without `v2` in the name. `sagemaker` has runtime and metrics surfaces; `kinesisvideo` has multiple data planes; Bedrock has several control and runtime APIs. Runtime/data-plane crates should usually observe control-plane state, while metrics or telemetry-only crates can remain fire-and-forget.

## Files

- `.agents/docs/services/ses.md` and `.agents/docs/services/sesv2.md` - SES shared-backend audit notes.
- `.agents/docs/services/api-gateway.md` and `.agents/docs/services/apigatewayv2.md` - custom-domain namespace notes.
- `.agents/docs/services/elastic-load-balancing.md` and `.agents/docs/services/elastic-load-balancing-v2.md` - load-balancer name namespace notes.
- `.agents/docs/services/appconfig.md`, `.agents/docs/services/appconfig-data.md`, `.agents/docs/services/mediastore.md`, `.agents/docs/services/mediastoredata.md`, `.agents/docs/services/sagemaker.md`, `.agents/docs/services/sagemakerruntime.md`, `.agents/docs/services/kinesis-video.md`, `.agents/docs/services/kinesis-video-archived-media.md`, `.agents/docs/services/cloudtrail.md`, `.agents/docs/services/cloudtrail-data.md` - control-plane/data-plane notes.
- `.agents/docs/services/*` - 54 network-aware service dossiers with `## Current Network Resource Stub Semantics` sections.
- `crates/winterbaume-dynamodbstreams/src/` - reference derived-service implementation over DynamoDB state.

## Test Coverage

State-coherence fixes should use cross-crate integration tests. Examples:

- create an SES identity or configuration set through v1, then list or read it through v2, and vice versa where AWS allows it.
- create a custom domain through API Gateway v1, then assert v2 rejects the duplicate name.
- create a Classic ELB, then assert ELBv2 rejects an ALB with the same name.
- create or deploy the control-plane resource first, then call the data plane and assert unknown resources are rejected.

## Pitfalls

- Do not infer shared state from naming alone. MediaPackage v2 and WAFv2 are separate services despite the suffix.
- Do not make data-plane crates silently create control-plane resources on demand unless real AWS does so.
- Do not assume an EC2 identifier stored in another service has been validated against EC2 state. Current network-aware service crates usually treat those identifiers as local metadata unless their service dossier says otherwise.
- Do not let low-priority bring-your-own-result mocks block useful unit-test workflows. Record the parity gap, then add validation only when a real scenario needs it.
- Do not model derived services as independent owners of upstream lifecycle state. DynamoDB Streams should remain the reference pattern.
