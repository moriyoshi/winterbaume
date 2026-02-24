# AWS Route53 Recovery Control Config

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Recovery Control Configuration API Reference for Amazon Route 53 Application Recovery Controller

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Route53 Recovery Control Config workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Delete`, `Describe`, `Update` operation families, including `ListAssociatedRoute53HealthChecks`, `ListClusters`, `ListControlPanels`, `ListRoutingControls`, `CreateCluster`, `CreateControlPanel`.

## Service Identity and Protocol

- AWS model slug: `route53-recovery-control-config`
- AWS SDK for Rust slug: `route53recoverycontrolconfig`
- Model version: `2020-11-02`
- Model file: `vendor/api-models-aws/models/route53-recovery-control-config/service/2020-11-02/route53-recovery-control-config-2020-11-02.json`
- SDK ID: `Route53 Recovery Control Config`
- Endpoint prefix: `route53-recovery-control-config`
- ARN namespace: `route53-recovery-control-config`
- CloudFormation name: `Route53RecoveryControlConfig`
- CloudTrail event source: `route53recoverycontrolconfig.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (6), `Create` (4), `Delete` (4), `Describe` (4), `Update` (4), `Get` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateCluster`, `CreateControlPanel`, `CreateRoutingControl`, `CreateSafetyRule`, `DeleteCluster`, `DeleteControlPanel`, `DeleteRoutingControl`, `DeleteSafetyRule`, `TagResource`, `UntagResource`, `UpdateCluster`, `UpdateControlPanel`, `UpdateRoutingControl`, `UpdateSafetyRule`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeCluster`, `DescribeControlPanel`, `DescribeRoutingControl`, `DescribeSafetyRule`, `GetResourcePolicy`, `ListAssociatedRoute53HealthChecks`, `ListClusters`, `ListControlPanels`, `ListRoutingControls`, `ListSafetyRules`, `ListTagsForResource`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 4 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 25 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListAssociatedRoute53HealthChecks`, `ListClusters`, `ListControlPanels`, `ListRoutingControls`, `ListSafetyRules`, `ListTagsForResource`
- Traits: `paginated` (5)
- Common required input members in this group: `ControlPanelArn`, `ResourceArn`, `RoutingControlArn`

### Create

- Operations: `CreateCluster`, `CreateControlPanel`, `CreateRoutingControl`, `CreateSafetyRule`
- Traits: `idempotency-token` (4)
- Common required input members in this group: `ClusterArn`, `ClusterName`, `ControlPanelName`, `RoutingControlName`

### Delete

- Operations: `DeleteCluster`, `DeleteControlPanel`, `DeleteRoutingControl`, `DeleteSafetyRule`
- Common required input members in this group: `ClusterArn`, `ControlPanelArn`, `RoutingControlArn`, `SafetyRuleArn`

### Describe

- Operations: `DescribeCluster`, `DescribeControlPanel`, `DescribeRoutingControl`, `DescribeSafetyRule`
- Common required input members in this group: `ClusterArn`, `ControlPanelArn`, `RoutingControlArn`, `SafetyRuleArn`

### Update

- Operations: `UpdateCluster`, `UpdateControlPanel`, `UpdateRoutingControl`, `UpdateSafetyRule`
- Common required input members in this group: `ClusterArn`, `ControlPanelArn`, `ControlPanelName`, `NetworkType`, `RoutingControlArn`, `RoutingControlName`

### Get

- Operations: `GetResourcePolicy`
- Common required input members in this group: `ResourceArn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateCluster` | `POST /cluster` | `idempotency-token` | `ClusterName` | `ClientToken` | `CreateClusterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a new cluster. A cluster is a set of redundant Regional endpoints against which you can run API calls to update or get the state of one or more routing controls. |
| `CreateControlPanel` | `POST /controlpanel` | `idempotency-token` | `ClusterArn`, `ControlPanelName` | `ClientToken` | `CreateControlPanelResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new control panel. A control panel represents a group of routing controls that can be changed together in a single transaction. |
| `CreateRoutingControl` | `POST /routingcontrol` | `idempotency-token` | `ClusterArn`, `RoutingControlName` | `ClientToken` | `CreateRoutingControlResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new routing control. A routing control has one of two states: ON and OFF. |
| `CreateSafetyRule` | `POST /safetyrule` | `idempotency-token` | - | `ClientToken` | `CreateSafetyRuleResponse` | `InternalServerException`, `ValidationException` | Creates a safety rule in a control panel. Safety rules let you add safeguards around changing routing control states, and for enabling and disabling routing controls, to help prevent unexpected outcomes. |
| `DeleteCluster` | `DELETE /cluster/{ClusterArn}` | - | `ClusterArn` | - | `DeleteClusterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete a cluster. |
| `DeleteControlPanel` | `DELETE /controlpanel/{ControlPanelArn}` | - | `ControlPanelArn` | - | `DeleteControlPanelResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a control panel. |
| `DeleteRoutingControl` | `DELETE /routingcontrol/{RoutingControlArn}` | - | `RoutingControlArn` | - | `DeleteRoutingControlResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a routing control. |
| `DeleteSafetyRule` | `DELETE /safetyrule/{SafetyRuleArn}` | - | `SafetyRuleArn` | - | `DeleteSafetyRuleResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes a safety rule. /> |
| `DescribeCluster` | `GET /cluster/{ClusterArn}` | - | `ClusterArn` | - | `DescribeClusterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Display the details about a cluster. The response includes the cluster name, endpoints, status, and Amazon Resource Name (ARN). |
| `DescribeControlPanel` | `GET /controlpanel/{ControlPanelArn}` | - | `ControlPanelArn` | - | `DescribeControlPanelResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Displays details about a control panel. |
| `DescribeRoutingControl` | `GET /routingcontrol/{RoutingControlArn}` | - | `RoutingControlArn` | - | `DescribeRoutingControlResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Displays details about a routing control. A routing control has one of two states: ON and OFF. |
| `DescribeSafetyRule` | `GET /safetyrule/{SafetyRuleArn}` | - | `SafetyRuleArn` | - | `DescribeSafetyRuleResponse` | `ResourceNotFoundException`, `ValidationException` | Returns information about a safety rule. |
| `GetResourcePolicy` | `GET /resourcePolicy/{ResourceArn}` | - | `ResourceArn` | - | `GetResourcePolicyResponse` | `InternalServerException`, `ResourceNotFoundException` | Get information about the resource policy for a cluster. |
| `ListAssociatedRoute53HealthChecks` | `GET /routingcontrol/{RoutingControlArn}/associatedRoute53HealthChecks` | `paginated` | `RoutingControlArn` | - | `ListAssociatedRoute53HealthChecksResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns an array of all Amazon Route 53 health checks associated with a specific routing control. |
| `ListClusters` | `GET /cluster` | `paginated` | - | - | `ListClustersResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns an array of all the clusters in an account. |
| `ListControlPanels` | `GET /controlpanels` | `paginated` | - | - | `ListControlPanelsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns an array of control panels in an account or in a cluster. |
| `ListRoutingControls` | `GET /controlpanel/{ControlPanelArn}/routingcontrols` | `paginated` | `ControlPanelArn` | - | `ListRoutingControlsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns an array of routing controls for a control panel. A routing control is an Amazon Route 53 Application Recovery Controller construct that has one of two states: ON and OFF. |
| `ListSafetyRules` | `GET /controlpanel/{ControlPanelArn}/safetyrules` | `paginated` | `ControlPanelArn` | - | `ListSafetyRulesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List the safety rules (the assertion rules and gating rules) that you've defined for the routing controls in a control panel. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the tags for a resource. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Adds a tag to a resource. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes a tag from a resource. |
| `UpdateCluster` | `PUT /cluster` | - | `ClusterArn`, `NetworkType` | - | `UpdateClusterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing cluster. You can only update the network type of a cluster. |
| `UpdateControlPanel` | `PUT /controlpanel` | - | `ControlPanelArn`, `ControlPanelName` | - | `UpdateControlPanelResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a control panel. The only update you can make to a control panel is to change the name of the control panel. |
| `UpdateRoutingControl` | `PUT /routingcontrol` | - | `RoutingControlArn`, `RoutingControlName` | - | `UpdateRoutingControlResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a routing control. You can only update the name of the routing control. |
| `UpdateSafetyRule` | `PUT /safetyrule` | - | - | - | `UpdateSafetyRuleResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Update a safety rule (an assertion rule or gating rule). You can only update the name and the waiting period for a safety rule. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `Message` | 500 response - InternalServiceError. |
| `ResourceNotFoundException` | `structure` | `Message` | 404 response - MalformedQueryString. |
| `ValidationException` | `structure` | `Message` | 400 response - Multiple causes. |
| `AccessDeniedException` | `structure` | `Message` | 403 response - You do not have sufficient access to perform this action. |
| `ThrottlingException` | `structure` | `Message` | 429 response - LimitExceededException or TooManyRequestsException. |
| `ConflictException` | `structure` | `Message` | 409 response - ConflictException. |
| `ServiceQuotaExceededException` | `structure` | `Message` | 402 response - You attempted to create more resources than the service allows based on service quotas. |
| `CreateClusterRequest` | `structure` | `ClientToken`, `ClusterName`, `NetworkType`, `Tags` | Creates a cluster. |
| `CreateClusterResponse` | `structure` | `Cluster` | - |
| `CreateControlPanelRequest` | `structure` | `ClientToken`, `ClusterArn`, `ControlPanelName`, `Tags` | The details of the control panel that you're creating. |
| `CreateControlPanelResponse` | `structure` | `ControlPanel` | - |
| `CreateRoutingControlRequest` | `structure` | `ClientToken`, `ClusterArn`, `ControlPanelArn`, `RoutingControlName` | The details of the routing control that you're creating. |
| `CreateRoutingControlResponse` | `structure` | `RoutingControl` | - |
| `CreateSafetyRuleRequest` | `structure` | `AssertionRule`, `ClientToken`, `GatingRule`, `Tags` | The request body that you include when you create a safety rule. |
| `CreateSafetyRuleResponse` | `structure` | `AssertionRule`, `GatingRule` | - |
| `DeleteClusterRequest` | `structure` | `ClusterArn` | - |
| `DeleteClusterResponse` | `structure` | - | - |
| `DeleteControlPanelRequest` | `structure` | `ControlPanelArn` | - |
| `DeleteControlPanelResponse` | `structure` | - | - |
| `DeleteRoutingControlRequest` | `structure` | `RoutingControlArn` | - |
| `DeleteRoutingControlResponse` | `structure` | - | - |
| `DeleteSafetyRuleRequest` | `structure` | `SafetyRuleArn` | - |
| `DeleteSafetyRuleResponse` | `structure` | - | - |
| `DescribeClusterRequest` | `structure` | `ClusterArn` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
