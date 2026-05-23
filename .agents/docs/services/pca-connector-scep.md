# Private CA Connector for SCEP

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Connector for SCEP creates a connector between Amazon Web Services Private CA and your SCEP-enabled clients and devices. For more information, see Connector for SCEP in the Amazon Web Services Private CA User Guide .

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Private CA Connector for SCEP workflows in the local mock. Key resources include `ChallengeResource`, `ConnectorResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Delete`, `Tag` operation families, including `GetChallengeMetadata`, `GetChallengePassword`, `GetConnector`, `ListChallengeMetadata`, `ListConnectors`, `ListTagsForResource`.

## Service Identity and Protocol

- AWS model slug: `pca-connector-scep`
- AWS SDK for Rust slug: `pcaconnectorscep`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/pca-connector-scep/service/2018-05-10/pca-connector-scep-2018-05-10.json`
- SDK ID: `Pca Connector Scep`
- Endpoint prefix: `-`
- ARN namespace: `pca-connector-scep`
- CloudFormation name: `PCAConnectorSCEP`
- CloudTrail event source: `pca-connector-scep.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (3), `List` (3), `Create` (2), `Delete` (2), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateChallenge`, `CreateConnector`, `DeleteChallenge`, `DeleteConnector`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetChallengeMetadata`, `GetChallengePassword`, `GetConnector`, `ListChallengeMetadata`, `ListConnectors`, `ListTagsForResource`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 5 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 12 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `ChallengeResource` | `ChallengeArn` | create: `CreateChallenge`; read: `GetChallengeMetadata`; delete: `DeleteChallenge`; list: `ListChallengeMetadata` | `GetChallengePassword` | - |
| `ConnectorResource` | `ConnectorArn` | create: `CreateConnector`; read: `GetConnector`; delete: `DeleteConnector`; list: `ListConnectors` | - | - |
## Operation Groups

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the tags associated with the specified resource. Tags are key-value pairs that you can use to categorize and manage your resources, for purposes like billing. For example, you might set the tag key to "cust ... |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds one or more tags to your resource. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes one or more tags from your resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | You can receive this error if you attempt to perform an operation and you don't have the required permissions. This can be caused by insufficient permission ... |
| `BadRequestException` | `structure` | Message | The request is malformed or contains an error such as an invalid parameter value or a missing required parameter. |
| `ConflictException` | `structure` | Message, ResourceId, ResourceType | This request can't be completed for one of the following reasons because the requested resource was being concurrently modified by another request. |
| `InternalServerException` | `structure` | Message | The request processing has failed because of an unknown error, exception or failure with an internal server. |
| `ResourceNotFoundException` | `structure` | Message, ResourceId, ResourceType | The operation tried to access a nonexistent resource. The resource might be incorrectly specified, or it might have a status other than ACTIVE . |
| `ServiceQuotaExceededException` | `structure` | Message, ResourceType, ServiceCode, QuotaCode | The request would cause a service quota to be exceeded. |
| `ThrottlingException` | `structure` | Message | The limit on the number of requests per second was exceeded. |
| `ValidationException` | `structure` | Message, Reason | An input validation error occurred. For example, invalid characters in a name tag, or an invalid pagination token. |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `TagResourceRequest` | `structure` | ResourceArn, Tags | - |
| `UntagResourceRequest` | `structure` | ResourceArn, TagKeys | - |
| `ConnectorStatus` | `enum` | CREATING, ACTIVE, DELETING, FAILED | - |
| `ConnectorStatusReason` | `enum` | INTERNAL_FAILURE, PRIVATECA_ACCESS_DENIED, PRIVATECA_INVALID_STATE, PRIVATECA_RESOURCE_NOT_FOUND, VPC_ENDPOINT_RESOURCE_NOT_FOUND, VPC_ENDPOINT_DNS_ENTRIES_NOT_FOUND | - |
| `ConnectorType` | `enum` | GENERAL_PURPOSE, INTUNE | - |
| `ValidationExceptionReason` | `enum` | CA_CERT_VALIDITY_TOO_SHORT, INVALID_CA_USAGE_MODE, INVALID_CONNECTOR_TYPE, INVALID_STATE, NO_CLIENT_TOKEN, UNKNOWN_OPERATION, OTHER | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
