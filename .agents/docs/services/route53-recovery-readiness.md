# AWS Route53 Recovery Readiness

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Recovery readiness

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Route53 Recovery Readiness workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Delete`, `Update` operation families, including `GetArchitectureRecommendations`, `GetCell`, `GetCellReadinessSummary`, `GetReadinessCheck`, `ListCells`, `ListCrossAccountAuthorizations`.

## Service Identity and Protocol

- AWS model slug: `route53-recovery-readiness`
- AWS SDK for Rust slug: `route53recoveryreadiness`
- Model version: `2019-12-02`
- Model file: `vendor/api-models-aws/models/route53-recovery-readiness/service/2019-12-02/route53-recovery-readiness-2019-12-02.json`
- SDK ID: `Route53 Recovery Readiness`
- Endpoint prefix: `route53-recovery-readiness`
- ARN namespace: `route53-recovery-readiness`
- CloudFormation name: `Route53RecoveryReadiness`
- CloudTrail event source: `route53-recovery-readiness.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (9), `List` (7), `Create` (5), `Delete` (5), `Update` (4), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateCell`, `CreateCrossAccountAuthorization`, `CreateReadinessCheck`, `CreateRecoveryGroup`, `CreateResourceSet`, `DeleteCell`, `DeleteCrossAccountAuthorization`, `DeleteReadinessCheck`, `DeleteRecoveryGroup`, `DeleteResourceSet`, `TagResource`, `UntagResource`, `UpdateCell`, `UpdateReadinessCheck`, `UpdateRecoveryGroup`, `UpdateResourceSet`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetArchitectureRecommendations`, `GetCell`, `GetCellReadinessSummary`, `GetReadinessCheck`, `GetReadinessCheckResourceStatus`, `GetReadinessCheckStatus`, `GetRecoveryGroup`, `GetRecoveryGroupReadinessSummary`, `GetResourceSet`, `ListCells`, `ListCrossAccountAuthorizations`, `ListReadinessChecks`, `ListRecoveryGroups`, `ListResourceSets`, `ListRules`, `ListTagsForResources`.
- Pagination is modelled for 10 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 32 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetArchitectureRecommendations`, `GetCell`, `GetCellReadinessSummary`, `GetReadinessCheck`, `GetReadinessCheckResourceStatus`, `GetReadinessCheckStatus`, `GetRecoveryGroup`, `GetRecoveryGroupReadinessSummary`, `GetResourceSet`
- Traits: `paginated` (4)
- Common required input members in this group: `CellName`, `ReadinessCheckName`, `RecoveryGroupName`, `ResourceIdentifier`, `ResourceSetName`

### List

- Operations: `ListCells`, `ListCrossAccountAuthorizations`, `ListReadinessChecks`, `ListRecoveryGroups`, `ListResourceSets`, `ListRules`, `ListTagsForResources`
- Traits: `paginated` (6)
- Common required input members in this group: `ResourceArn`

### Create

- Operations: `CreateCell`, `CreateCrossAccountAuthorization`, `CreateReadinessCheck`, `CreateRecoveryGroup`, `CreateResourceSet`
- Common required input members in this group: `CellName`, `CrossAccountAuthorization`, `ReadinessCheckName`, `RecoveryGroupName`, `ResourceSetName`, `ResourceSetType`, `Resources`

### Delete

- Operations: `DeleteCell`, `DeleteCrossAccountAuthorization`, `DeleteReadinessCheck`, `DeleteRecoveryGroup`, `DeleteResourceSet`
- Common required input members in this group: `CellName`, `CrossAccountAuthorization`, `ReadinessCheckName`, `RecoveryGroupName`, `ResourceSetName`

### Update

- Operations: `UpdateCell`, `UpdateReadinessCheck`, `UpdateRecoveryGroup`, `UpdateResourceSet`
- Common required input members in this group: `CellName`, `Cells`, `ReadinessCheckName`, `RecoveryGroupName`, `ResourceSetName`, `ResourceSetType`, `Resources`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateCell` | `POST /cells` | - | `CellName` | - | `CreateCellResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a cell in an account. |
| `CreateCrossAccountAuthorization` | `POST /crossaccountauthorizations` | - | `CrossAccountAuthorization` | - | `CreateCrossAccountAuthorizationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a cross-account readiness authorization. This lets you authorize another account to work with Route 53 Application Recovery Controller, for example, to check the readiness status of resources in a separate account. |
| `CreateReadinessCheck` | `POST /readinesschecks` | - | `ReadinessCheckName`, `ResourceSetName` | - | `CreateReadinessCheckResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a readiness check in an account. A readiness check monitors a resource set in your application, such as a set of Amazon Aurora instances, that Application Recovery Controller is auditing recovery readiness for. |
| `CreateRecoveryGroup` | `POST /recoverygroups` | - | `RecoveryGroupName` | - | `CreateRecoveryGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a recovery group in an account. A recovery group corresponds to an application and includes a list of the cells that make up the application. |
| `CreateResourceSet` | `POST /resourcesets` | - | `ResourceSetName`, `ResourceSetType`, `Resources` | - | `CreateResourceSetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a resource set. A resource set is a set of resources of one type that span multiple cells. |
| `DeleteCell` | `DELETE /cells/{CellName}` | - | `CellName` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete a cell. When successful, the response code is 204, with no response body. |
| `DeleteCrossAccountAuthorization` | `DELETE /crossaccountauthorizations/{CrossAccountAuthorization}` | - | `CrossAccountAuthorization` | - | `DeleteCrossAccountAuthorizationResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes cross account readiness authorization. |
| `DeleteReadinessCheck` | `DELETE /readinesschecks/{ReadinessCheckName}` | - | `ReadinessCheckName` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a readiness check. |
| `DeleteRecoveryGroup` | `DELETE /recoverygroups/{RecoveryGroupName}` | - | `RecoveryGroupName` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a recovery group. |
| `DeleteResourceSet` | `DELETE /resourcesets/{ResourceSetName}` | - | `ResourceSetName` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a resource set. |
| `GetArchitectureRecommendations` | `GET /recoverygroups/{RecoveryGroupName}/architectureRecommendations` | - | `RecoveryGroupName` | - | `GetArchitectureRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets recommendations about architecture designs for improving resiliency for an application, based on a recovery group. |
| `GetCell` | `GET /cells/{CellName}` | - | `CellName` | - | `GetCellResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a cell including cell name, cell Amazon Resource Name (ARN), ARNs of nested cells for this cell, and a list of those cell ARNs with their associated recovery group ARNs. |
| `GetCellReadinessSummary` | `GET /cellreadiness/{CellName}` | `paginated` | `CellName` | - | `GetCellReadinessSummaryResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets readiness for a cell. Aggregates the readiness of all the resources that are associated with the cell into a single value. |
| `GetReadinessCheck` | `GET /readinesschecks/{ReadinessCheckName}` | - | `ReadinessCheckName` | - | `GetReadinessCheckResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets details about a readiness check. |
| `GetReadinessCheckResourceStatus` | `GET /readinesschecks/{ReadinessCheckName}/resource/{ResourceIdentifier}/status` | `paginated` | `ReadinessCheckName`, `ResourceIdentifier` | - | `GetReadinessCheckResourceStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets individual readiness status for a readiness check. To see the overall readiness status for a recovery group, that considers the readiness status for all the readiness checks in the recovery group, use GetRecoveryGroupReadinessSummary. |
| `GetReadinessCheckStatus` | `GET /readinesschecks/{ReadinessCheckName}/status` | `paginated` | `ReadinessCheckName` | - | `GetReadinessCheckStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the readiness status for an individual readiness check. To see the overall readiness status for a recovery group, that considers the readiness status for all the readiness checks in a recovery group, use GetRecoveryGroupReadinessSummary. |
| `GetRecoveryGroup` | `GET /recoverygroups/{RecoveryGroupName}` | - | `RecoveryGroupName` | - | `GetRecoveryGroupResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets details about a recovery group, including a list of the cells that are included in it. |
| `GetRecoveryGroupReadinessSummary` | `GET /recoverygroupreadiness/{RecoveryGroupName}` | `paginated` | `RecoveryGroupName` | - | `GetRecoveryGroupReadinessSummaryResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Displays a summary of information about a recovery group's readiness status. Includes the readiness checks for resources in the recovery group and the readiness status of each one. |
| `GetResourceSet` | `GET /resourcesets/{ResourceSetName}` | - | `ResourceSetName` | - | `GetResourceSetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Displays the details about a resource set, including a list of the resources in the set. |
| `ListCells` | `GET /cells` | `paginated` | - | - | `ListCellsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the cells for an account. |
| `ListCrossAccountAuthorizations` | `GET /crossaccountauthorizations` | `paginated` | - | - | `ListCrossAccountAuthorizationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the cross-account readiness authorizations that are in place for an account. |
| `ListReadinessChecks` | `GET /readinesschecks` | `paginated` | - | - | `ListReadinessChecksResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the readiness checks for an account. |
| `ListRecoveryGroups` | `GET /recoverygroups` | `paginated` | - | - | `ListRecoveryGroupsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the recovery groups in an account. |
| `ListResourceSets` | `GET /resourcesets` | `paginated` | - | - | `ListResourceSetsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the resource sets in an account. |
| `ListRules` | `GET /rules` | `paginated` | - | - | `ListRulesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all readiness rules, or lists the readiness rules for a specific resource type. |
| `ListTagsForResources` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourcesResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the tags for a resource. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Adds a tag to a resource. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `Unit` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes a tag from a resource. |
| `UpdateCell` | `PUT /cells/{CellName}` | - | `CellName`, `Cells` | - | `UpdateCellResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a cell to replace the list of nested cells with a new list of nested cells. |
| `UpdateReadinessCheck` | `PUT /readinesschecks/{ReadinessCheckName}` | - | `ReadinessCheckName`, `ResourceSetName` | - | `UpdateReadinessCheckResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a readiness check. |
| `UpdateRecoveryGroup` | `PUT /recoverygroups/{RecoveryGroupName}` | - | `Cells`, `RecoveryGroupName` | - | `UpdateRecoveryGroupResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a recovery group. |
| `UpdateResourceSet` | `PUT /resourcesets/{ResourceSetName}` | - | `ResourceSetName`, `ResourceSetType`, `Resources` | - | `UpdateResourceSetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a resource set. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `Message` | An unexpected error occurred. |
| `ValidationException` | `structure` | `Message` | The input fails to satisfy the constraints specified by an AWS service. |
| `AccessDeniedException` | `structure` | `Message` | User does not have sufficient access to perform this action. |
| `ThrottlingException` | `structure` | `Message` | Request was denied due to request throttling. |
| `ResourceNotFoundException` | `structure` | `Message` | The requested resource does not exist. |
| `ConflictException` | `structure` | `Message` | Updating or deleting a resource can cause an inconsistent state. |
| `CreateCellRequest` | `structure` | `CellName`, `Cells`, `Tags` | - |
| `CreateCellResponse` | `structure` | `CellArn`, `CellName`, `Cells`, `ParentReadinessScopes`, `Tags` | - |
| `CreateCrossAccountAuthorizationRequest` | `structure` | `CrossAccountAuthorization` | - |
| `CreateCrossAccountAuthorizationResponse` | `structure` | `CrossAccountAuthorization` | - |
| `CreateReadinessCheckRequest` | `structure` | `ReadinessCheckName`, `ResourceSetName`, `Tags` | - |
| `CreateReadinessCheckResponse` | `structure` | `ReadinessCheckArn`, `ReadinessCheckName`, `ResourceSet`, `Tags` | - |
| `CreateRecoveryGroupRequest` | `structure` | `Cells`, `RecoveryGroupName`, `Tags` | - |
| `CreateRecoveryGroupResponse` | `structure` | `Cells`, `RecoveryGroupArn`, `RecoveryGroupName`, `Tags` | - |
| `CreateResourceSetRequest` | `structure` | `ResourceSetName`, `ResourceSetType`, `Resources`, `Tags` | - |
| `CreateResourceSetResponse` | `structure` | `ResourceSetArn`, `ResourceSetName`, `ResourceSetType`, `Resources`, `Tags` | - |
| `DeleteCellRequest` | `structure` | `CellName` | - |
| `DeleteCrossAccountAuthorizationRequest` | `structure` | `CrossAccountAuthorization` | - |
| `DeleteCrossAccountAuthorizationResponse` | `structure` | - | - |
| `DeleteReadinessCheckRequest` | `structure` | `ReadinessCheckName` | - |
| `DeleteRecoveryGroupRequest` | `structure` | `RecoveryGroupName` | - |
| `DeleteResourceSetRequest` | `structure` | `ResourceSetName` | - |
| `GetArchitectureRecommendationsRequest` | `structure` | `MaxResults`, `NextToken`, `RecoveryGroupName` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
