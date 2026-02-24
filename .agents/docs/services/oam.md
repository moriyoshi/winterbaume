# CloudWatch Observability Access Manager

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Use Amazon CloudWatch Observability Access Manager to create and manage links between source accounts and monitoring accounts by using CloudWatch cross-account observability . With CloudWatch cross-account observability, you can monitor and troubleshoot applications that span multiple accounts within a Region. Seamlessly search, visualize, and analyze your metrics, logs, traces, Application Signals services and service level objectives (SLOs), Application Insights applications, and internet monitors in any of the linked accounts without account boundaries. Set up one or more Amazon Web Services accounts as monitoring accounts and link them with multiple source accounts . A monitoring account is a central Amazon Web Services account that can view and interact with observability data generated from source accounts.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented CloudWatch Observability Access Manager workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Create`, `Delete`, `Put` operation families, including `ListAttachedLinks`, `ListLinks`, `ListSinks`, `ListTagsForResource`, `GetLink`, `GetSink`.

## Service Identity and Protocol

- AWS model slug: `oam`
- AWS SDK for Rust slug: `oam`
- Model version: `2022-06-10`
- Model file: `vendor/api-models-aws/models/oam/service/2022-06-10/oam-2022-06-10.json`
- SDK ID: `OAM`
- Endpoint prefix: `-`
- ARN namespace: `oam`
- CloudFormation name: `-`
- CloudTrail event source: `{{CLOUD_TRAIL_EVENT_SOURCE}}`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Get` (3), `Create` (2), `Delete` (2), `Put` (1), `Tag` (1), `Untag` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateLink`, `CreateSink`, `DeleteLink`, `DeleteSink`, `PutSinkPolicy`, `TagResource`, `UntagResource`, `UpdateLink`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetLink`, `GetSink`, `GetSinkPolicy`, `ListAttachedLinks`, `ListLinks`, `ListSinks`, `ListTagsForResource`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 15 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `CloudWatch`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListAttachedLinks`, `ListLinks`, `ListSinks`, `ListTagsForResource`
- Traits: `paginated` (3), `readonly` (4)
- Common required input members in this group: `ResourceArn`, `SinkIdentifier`

### Get

- Operations: `GetLink`, `GetSink`, `GetSinkPolicy`
- Traits: `readonly` (3)
- Common required input members in this group: `Identifier`, `SinkIdentifier`

### Create

- Operations: `CreateLink`, `CreateSink`
- Common required input members in this group: `LabelTemplate`, `Name`, `ResourceTypes`, `SinkIdentifier`

### Delete

- Operations: `DeleteLink`, `DeleteSink`
- Common required input members in this group: `Identifier`

### Put

- Operations: `PutSinkPolicy`
- Common required input members in this group: `Policy`, `SinkIdentifier`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

### Update

- Operations: `UpdateLink`
- Common required input members in this group: `Identifier`, `ResourceTypes`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateLink` | `POST /CreateLink` | - | `LabelTemplate`, `ResourceTypes`, `SinkIdentifier` | - | `CreateLinkOutput` | `ConflictException`, `InternalServiceFault`, `InvalidParameterException`, `MissingRequiredParameterException`, `ServiceQuotaExceededException` | Creates a link between a source account and a sink that you have created in a monitoring account. After the link is created, data is sent from the source account to the monitoring account. |
| `CreateSink` | `POST /CreateSink` | - | `Name` | - | `CreateSinkOutput` | `ConflictException`, `InternalServiceFault`, `InvalidParameterException`, `MissingRequiredParameterException`, `ServiceQuotaExceededException` | Use this to create a sink in the current account, so that it can be used as a monitoring account in CloudWatch cross-account observability. A sink is a resource that represents an attachment point in a monitoring account. |
| `DeleteLink` | `POST /DeleteLink` | - | `Identifier` | - | `DeleteLinkOutput` | `InternalServiceFault`, `InvalidParameterException`, `MissingRequiredParameterException`, `ResourceNotFoundException` | Deletes a link between a monitoring account sink and a source account. You must run this operation in the source account. |
| `DeleteSink` | `POST /DeleteSink` | - | `Identifier` | - | `DeleteSinkOutput` | `ConflictException`, `InternalServiceFault`, `InvalidParameterException`, `MissingRequiredParameterException`, `ResourceNotFoundException` | Deletes a sink. You must delete all links to a sink before you can delete that sink. |
| `GetLink` | `POST /GetLink` | `readonly` | `Identifier` | - | `GetLinkOutput` | `InternalServiceFault`, `InvalidParameterException`, `MissingRequiredParameterException`, `ResourceNotFoundException` | Returns complete information about one link. To use this operation, provide the link ARN. |
| `GetSink` | `POST /GetSink` | `readonly` | `Identifier` | - | `GetSinkOutput` | `InternalServiceFault`, `InvalidParameterException`, `MissingRequiredParameterException`, `ResourceNotFoundException` | Returns complete information about one monitoring account sink. To use this operation, provide the sink ARN. |
| `GetSinkPolicy` | `POST /GetSinkPolicy` | `readonly` | `SinkIdentifier` | - | `GetSinkPolicyOutput` | `InternalServiceFault`, `InvalidParameterException`, `MissingRequiredParameterException`, `ResourceNotFoundException` | Returns the current sink policy attached to this sink. The sink policy specifies what accounts can attach to this sink as source accounts, and what types of data they can share. |
| `ListAttachedLinks` | `POST /ListAttachedLinks` | `readonly`, `paginated` | `SinkIdentifier` | - | `ListAttachedLinksOutput` | `InternalServiceFault`, `InvalidParameterException`, `MissingRequiredParameterException`, `ResourceNotFoundException` | Returns a list of source account links that are linked to this monitoring account sink. To use this operation, provide the sink ARN. |
| `ListLinks` | `POST /ListLinks` | `readonly`, `paginated` | - | - | `ListLinksOutput` | `InternalServiceFault`, `InvalidParameterException`, `ResourceNotFoundException` | Use this operation in a source account to return a list of links to monitoring account sinks that this source account has. To find a list of links for one monitoring account sink, use ListAttachedLinks from within the monitoring account. |
| `ListSinks` | `POST /ListSinks` | `readonly`, `paginated` | - | - | `ListSinksOutput` | `InternalServiceFault`, `InvalidParameterException`, `ResourceNotFoundException` | Use this operation in a monitoring account to return the list of sinks created in that account. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceOutput` | `ResourceNotFoundException`, `ValidationException` | Displays the tags associated with a resource. Both sinks and links support tagging. |
| `PutSinkPolicy` | `POST /PutSinkPolicy` | - | `Policy`, `SinkIdentifier` | - | `PutSinkPolicyOutput` | `InternalServiceFault`, `InvalidParameterException`, `MissingRequiredParameterException`, `ResourceNotFoundException` | Creates or updates the resource policy that grants permissions to source accounts to link to the monitoring account sink. When you create a sink policy, you can grant permissions to all accounts in an organization or to individual accounts. |
| `TagResource` | `PUT /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceOutput` | `ResourceNotFoundException`, `TooManyTagsException`, `ValidationException` | Assigns one or more tags (key-value pairs) to the specified resource. Both sinks and links can be tagged. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceOutput` | `ResourceNotFoundException`, `ValidationException` | Removes one or more tags from the specified resource. Unlike tagging permissions in other Amazon Web Services services, to tag or untag links and sinks you must have the `oam:ResourceTag` permission. |
| `UpdateLink` | `POST /UpdateLink` | - | `Identifier`, `ResourceTypes` | - | `UpdateLinkOutput` | `InternalServiceFault`, `InvalidParameterException`, `MissingRequiredParameterException`, `ResourceNotFoundException` | Use this operation to change what types of data are shared from a source account to its linked monitoring account sink. You can't change the sink or change the monitoring account with this operation. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ResourceNotFoundException` | `structure` | `Message`, `amznErrorType` | The request references a resource that does not exist. |
| `InternalServiceFault` | `structure` | `Message`, `amznErrorType` | Unexpected error while processing the request. |
| `InvalidParameterException` | `structure` | `amznErrorType`, `message` | A parameter is specified incorrectly. |
| `MissingRequiredParameterException` | `structure` | `amznErrorType`, `message` | A required parameter is missing from the request. |
| `ConflictException` | `structure` | `Message`, `amznErrorType` | A resource was in an inconsistent state during an update or a deletion. |
| `ValidationException` | `structure` | `Message` | The value of a parameter in the request caused an error. |
| `ServiceQuotaExceededException` | `structure` | `Message`, `amznErrorType` | The request would cause a service quota to be exceeded. |
| `CreateLinkInput` | `structure` | `LabelTemplate`, `LinkConfiguration`, `ResourceTypes`, `SinkIdentifier`, `Tags` | - |
| `CreateLinkOutput` | `structure` | `Arn`, `Id`, `Label`, `LabelTemplate`, `LinkConfiguration`, `ResourceTypes`, `SinkArn`, `Tags` | - |
| `CreateSinkInput` | `structure` | `Name`, `Tags` | - |
| `CreateSinkOutput` | `structure` | `Arn`, `Id`, `Name`, `Tags` | - |
| `DeleteLinkInput` | `structure` | `Identifier` | - |
| `DeleteLinkOutput` | `structure` | - | - |
| `DeleteSinkInput` | `structure` | `Identifier` | - |
| `DeleteSinkOutput` | `structure` | - | - |
| `GetLinkInput` | `structure` | `Identifier`, `IncludeTags` | - |
| `GetLinkOutput` | `structure` | `Arn`, `Id`, `Label`, `LabelTemplate`, `LinkConfiguration`, `ResourceTypes`, `SinkArn`, `Tags` | - |
| `GetSinkInput` | `structure` | `Identifier`, `IncludeTags` | - |
| `GetSinkOutput` | `structure` | `Arn`, `Id`, `Name`, `Tags` | - |
| `GetSinkPolicyInput` | `structure` | `SinkIdentifier` | - |
| `GetSinkPolicyOutput` | `structure` | `Policy`, `SinkArn`, `SinkId` | - |
| `ListAttachedLinksInput` | `structure` | `MaxResults`, `NextToken`, `SinkIdentifier` | - |
| `ListAttachedLinksOutput` | `structure` | `Items`, `NextToken` | - |
| `ListLinksInput` | `structure` | `MaxResults`, `NextToken` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
