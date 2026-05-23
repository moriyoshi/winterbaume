# AWS User Notifications Contacts

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS User Notifications Contacts is a service that allows you to create and manage email contacts for AWS User Notifications. The AWS User Notifications Contacts API Reference provides descriptions, API request parameters, and the JSON response for all email contact related API actions.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS User Notifications Contacts workflows in the local mock. Key resources include `EmailContactResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Activate`, `Create`, `Delete`, `Get` operation families, including `ListEmailContacts`, `ListTagsForResource`, `ActivateEmailContact`, `CreateEmailContact`, `DeleteEmailContact`, `GetEmailContact`.

## Service Identity and Protocol

- AWS model slug: `notificationscontacts`
- AWS SDK for Rust slug: `notificationscontacts`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/notificationscontacts/service/2018-05-10/notificationscontacts-2018-05-10.json`
- SDK ID: `NotificationsContacts`
- Endpoint prefix: `notifications-contacts`
- ARN namespace: `notifications-contacts`
- CloudFormation name: `-`
- CloudTrail event source: `notifications-contacts.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (2), `Activate` (1), `Create` (1), `Delete` (1), `Get` (1), `Send` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateEmailContact`, `DeleteEmailContact`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetEmailContact`, `ListEmailContacts`, `ListTagsForResource`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 4 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 9 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `EmailContactResource` | `arn` | create: `CreateEmailContact`; read: `GetEmailContact`; delete: `DeleteEmailContact`; list: `ListEmailContacts` | `ActivateEmailContact`, `SendActivationCode` | - |
## Operation Groups

### List

- Operations: `ListEmailContacts`, `ListTagsForResource`
- Traits: `paginated` (1), `readonly` (2)
- Common required input members in this group: `arn`

### Activate

- Operations: `ActivateEmailContact`
- Traits: `idempotent` (1)
- Common required input members in this group: `arn`, `code`

### Create

- Operations: `CreateEmailContact`
- Common required input members in this group: `emailAddress`, `name`

### Delete

- Operations: `DeleteEmailContact`
- Traits: `idempotent` (1)
- Common required input members in this group: `arn`

### Get

- Operations: `GetEmailContact`
- Traits: `readonly` (1)
- Common required input members in this group: `arn`

### Send

- Operations: `SendActivationCode`
- Common required input members in this group: `arn`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `arn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `arn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ActivateEmailContact` | `PUT /emailcontacts/{arn}/activate/{code}` | `idempotent` | `arn`, `code` | - | `ActivateEmailContactResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Activates an email contact using an activation code. This code is in the activation email sent to the email address associated with this email contact. |
| `CreateEmailContact` | `POST /2022-09-19/emailcontacts` | - | `emailAddress`, `name` | - | `CreateEmailContactResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an email contact for the provided email address. |
| `DeleteEmailContact` | `DELETE /emailcontacts/{arn}` | `idempotent` | `arn` | - | `DeleteEmailContactResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an email contact. Deleting an email contact removes it from all associated notification configurations. |
| `GetEmailContact` | `GET /emailcontacts/{arn}` | `readonly` | `arn` | - | `GetEmailContactResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns an email contact. |
| `ListEmailContacts` | `GET /emailcontacts` | `readonly`, `paginated` | - | - | `ListEmailContactsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all email contacts created under the Account. |
| `ListTagsForResource` | `GET /tags/{arn}` | `readonly` | `arn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all of the tags associated with the Amazon Resource Name (ARN) that you specify. The resource can be a user, server, or role. |
| `SendActivationCode` | `POST /2022-10-31/emailcontacts/{arn}/activate/send` | - | `arn` | - | `SendActivationCodeResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Sends an activation email to the email address associated with the specified email contact. It might take a few minutes for the activation email to arrive. |
| `TagResource` | `POST /tags/{arn}` | `idempotent` | `arn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Attaches a key-value pair to a resource, as identified by its Amazon Resource Name (ARN). Taggable resources in AWS User Notifications Contacts include email contacts. |
| `UntagResource` | `DELETE /tags/{arn}` | `idempotent` | `arn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Detaches a key-value pair from a resource, as identified by its Amazon Resource Name (ARN). Taggable resources in AWS User Notifications Contacts include email contacts.. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `message` | Unexpected error during processing of request. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `retryAfterSeconds`, `serviceCode` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | The input fails to satisfy the constraints specified by an AWS service. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | Your request references a resource which does not exist. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | Updating or deleting a resource can cause an inconsistent state. |
| `ActivateEmailContactRequest` | `structure` | `arn`, `code` | - |
| `ActivateEmailContactResponse` | `structure` | - | - |
| `CreateEmailContactRequest` | `structure` | `emailAddress`, `name`, `tags` | - |
| `CreateEmailContactResponse` | `structure` | `arn` | - |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType`, `serviceCode` | Request would cause a service quota to be exceeded. |
| `DeleteEmailContactRequest` | `structure` | `arn` | - |
| `DeleteEmailContactResponse` | `structure` | - | - |
| `GetEmailContactRequest` | `structure` | `arn` | - |
| `GetEmailContactResponse` | `structure` | `emailContact` | - |
| `ListEmailContactsRequest` | `structure` | `maxResults`, `nextToken` | - |
| `ListEmailContactsResponse` | `structure` | `emailContacts`, `nextToken` | - |
| `ListTagsForResourceRequest` | `structure` | `arn` | - |
| `ListTagsForResourceResponse` | `structure` | `tags` | - |
| `SendActivationCodeRequest` | `structure` | `arn` | - |
| `SendActivationCodeResponse` | `structure` | - | - |
| `TagResourceRequest` | `structure` | `arn`, `tags` | - |
| `TagResourceResponse` | `structure` | - | - |
| `UntagResourceRequest` | `structure` | `arn`, `tagKeys` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
