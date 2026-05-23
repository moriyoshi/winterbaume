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

### Get

- Operations: `GetChallengeMetadata`, `GetChallengePassword`, `GetConnector`
- Traits: `readonly` (3)
- Common required input members in this group: `ChallengeArn`, `ConnectorArn`

### List

- Operations: `ListChallengeMetadata`, `ListConnectors`, `ListTagsForResource`
- Traits: `paginated` (2), `readonly` (3)
- Common required input members in this group: `ConnectorArn`, `ResourceArn`

### Create

- Operations: `CreateChallenge`, `CreateConnector`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `CertificateAuthorityArn`, `ConnectorArn`

### Delete

- Operations: `DeleteChallenge`, `DeleteConnector`
- Traits: `idempotent` (2)
- Common required input members in this group: `ChallengeArn`, `ConnectorArn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateChallenge` | `POST /challenges` | `idempotency-token` | `ConnectorArn` | `ClientToken` | `CreateChallengeResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | For general-purpose connectors. Creates a challenge password for the specified connector. |
| `CreateConnector` | `POST /connectors` | `idempotency-token` | `CertificateAuthorityArn` | `ClientToken` | `CreateConnectorResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a SCEP connector. A SCEP connector links Amazon Web Services Private Certificate Authority to your SCEP-compatible devices and mobile device management (MDM) systems. |
| `DeleteChallenge` | `DELETE /challenges/{ChallengeArn}` | `idempotent` | `ChallengeArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified Challenge. |
| `DeleteConnector` | `DELETE /connectors/{ConnectorArn}` | `idempotent` | `ConnectorArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified Connector. This operation also deletes any challenges associated with the connector. |
| `GetChallengeMetadata` | `GET /challengeMetadata/{ChallengeArn}` | `readonly` | `ChallengeArn` | - | `GetChallengeMetadataResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the metadata for the specified Challenge. |
| `GetChallengePassword` | `GET /challengePasswords/{ChallengeArn}` | `readonly` | `ChallengeArn` | - | `GetChallengePasswordResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the challenge password for the specified Challenge. |
| `GetConnector` | `GET /connectors/{ConnectorArn}` | `readonly` | `ConnectorArn` | - | `GetConnectorResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves details about the specified Connector. Calling this action returns important details about the connector, such as the public SCEP URL where your clients can request certificates. |
| `ListChallengeMetadata` | `GET /challengeMetadata` | `readonly`, `paginated` | `ConnectorArn` | - | `ListChallengeMetadataResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the challenge metadata for the specified ARN. |
| `ListConnectors` | `GET /connectors` | `readonly`, `paginated` | - | - | `ListConnectorsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the connectors belonging to your Amazon Web Services account. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the tags associated with the specified resource. Tags are key-value pairs that you can use to categorize and manage your resources, for purposes like billing. |
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
| `AccessDeniedException` | `structure` | `Message` | You can receive this error if you attempt to perform an operation and you don't have the required permissions. |
| `InternalServerException` | `structure` | `Message` | The request processing has failed because of an unknown error, exception or failure with an internal server. |
| `ThrottlingException` | `structure` | `Message` | The limit on the number of requests per second was exceeded. |
| `ValidationException` | `structure` | `Message`, `Reason` | An input validation error occurred. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceId`, `ResourceType` | The operation tried to access a nonexistent resource. |
| `ConflictException` | `structure` | `Message`, `ResourceId`, `ResourceType` | This request can't be completed for one of the following reasons because the requested resource was being concurrently modified by another request. |
| `ServiceQuotaExceededException` | `structure` | `Message`, `QuotaCode`, `ResourceType`, `ServiceCode` | The request would cause a service quota to be exceeded. |
| `CreateChallengeRequest` | `structure` | `ClientToken`, `ConnectorArn`, `Tags` | - |
| `CreateChallengeResponse` | `structure` | `Challenge` | - |
| `BadRequestException` | `structure` | `Message` | The request is malformed or contains an error such as an invalid parameter value or a missing required parameter. |
| `CreateConnectorRequest` | `structure` | `CertificateAuthorityArn`, `ClientToken`, `MobileDeviceManagement`, `Tags`, `VpcEndpointId` | - |
| `CreateConnectorResponse` | `structure` | `ConnectorArn` | - |
| `DeleteChallengeRequest` | `structure` | `ChallengeArn` | - |
| `DeleteConnectorRequest` | `structure` | `ConnectorArn` | - |
| `GetChallengeMetadataRequest` | `structure` | `ChallengeArn` | - |
| `GetChallengeMetadataResponse` | `structure` | `ChallengeMetadata` | - |
| `GetChallengePasswordRequest` | `structure` | `ChallengeArn` | - |
| `GetChallengePasswordResponse` | `structure` | `Password` | - |
| `GetConnectorRequest` | `structure` | `ConnectorArn` | - |
| `GetConnectorResponse` | `structure` | `Connector` | - |
| `ListChallengeMetadataRequest` | `structure` | `ConnectorArn`, `MaxResults`, `NextToken` | - |
| `ListChallengeMetadataResponse` | `structure` | `Challenges`, `NextToken` | - |
| `ListConnectorsRequest` | `structure` | `MaxResults`, `NextToken` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
