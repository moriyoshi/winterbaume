# AWS End User Messaging Social

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services End User Messaging Social , also referred to as Social messaging, is a messaging service that enables application developers to incorporate WhatsApp into their existing workflows. The Amazon Web Services End User Messaging Social API provides information about the Amazon Web Services End User Messaging Social API resources, including supported HTTP methods, parameters, and schemas. The Amazon Web Services End User Messaging Social API provides programmatic access to options that are unique to the WhatsApp Business Platform. If you're new to the Amazon Web Services End User Messaging Social API , it's also helpful to review What is Amazon Web Services End User Messaging Social in the Amazon Web Services End User Messaging Social User Guide . The Amazon Web Services End User Messaging Social User Guide provides tutorials, code samples, and procedures that demonstrate how to use Amazon Web Services End User Messaging Social API features programmatically and how to integrate functionality into applications.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS End User Messaging Social where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS End User Messaging Social by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented AWS End User Messaging Social workflows in the local mock. Key resources include `LinkedWhatsAppBusinessAccountResource`, `LinkedWhatsAppPhoneNumberResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Delete`, `Associate` operation families, including `GetLinkedWhatsAppBusinessAccount`, `GetLinkedWhatsAppBusinessAccountPhoneNumber`, `GetWhatsAppMessageMedia`, `GetWhatsAppMessageTemplate`, `ListLinkedWhatsAppBusinessAccounts`, `ListTagsForResource`.

## Service Identity and Protocol

- AWS model slug: `socialmessaging`
- AWS SDK for Rust slug: `socialmessaging`
- Model version: `2024-01-01`
- Model file: `vendor/api-models-aws/models/socialmessaging/service/2024-01-01/socialmessaging-2024-01-01.json`
- SDK ID: `SocialMessaging`
- Endpoint prefix: `social-messaging`
- ARN namespace: `social-messaging`
- CloudFormation name: `-`
- CloudTrail event source: `social-messaging.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (4), `List` (4), `Create` (3), `Delete` (2), `Associate` (1), `Disassociate` (1), `Post` (1), `Put` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateWhatsAppBusinessAccount`, `CreateWhatsAppMessageTemplate`, `CreateWhatsAppMessageTemplateFromLibrary`, `CreateWhatsAppMessageTemplateMedia`, `DeleteWhatsAppMessageMedia`, `DeleteWhatsAppMessageTemplate`, `DisassociateWhatsAppBusinessAccount`, `PutWhatsAppBusinessAccountEventDestinations`, `TagResource`, `UntagResource`, `UpdateWhatsAppMessageTemplate`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetLinkedWhatsAppBusinessAccount`, `GetLinkedWhatsAppBusinessAccountPhoneNumber`, `GetWhatsAppMessageMedia`, `GetWhatsAppMessageTemplate`, `ListLinkedWhatsAppBusinessAccounts`, `ListTagsForResource`, `ListWhatsAppMessageTemplates`, `ListWhatsAppTemplateLibrary`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 4 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 21 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `STS`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `LinkedWhatsAppBusinessAccountResource` | `WabaId` | create: `AssociateWhatsAppBusinessAccount`; read: `GetLinkedWhatsAppBusinessAccount`; delete: `DisassociateWhatsAppBusinessAccount`; list: `ListLinkedWhatsAppBusinessAccounts` | `CreateWhatsAppMessageTemplate`, `CreateWhatsAppMessageTemplateFromLibrary`, `CreateWhatsAppMessageTemplateMedia`, `DeleteWhatsAppMessageTemplate`, `GetWhatsAppMessageTemplate`, `ListWhatsAppMessageTemplates`, `ListWhatsAppTemplateLibrary`, `PutWhatsAppBusinessAccountEventDestinations`, `UpdateWhatsAppMessageTemplate` | - |
| `LinkedWhatsAppPhoneNumberResource` | `OriginationPhoneNumberId` | read: `GetLinkedWhatsAppBusinessAccountPhoneNumber` | `DeleteWhatsAppMessageMedia`, `GetWhatsAppMessageMedia`, `PostWhatsAppMessageMedia`, `SendWhatsAppMessage` | - |
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
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListTagsForResource` | `GET /v1/tags/list` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `InternalServiceException`, `InvalidParametersException`, `ThrottledRequestException` | List all tags associated with a resource, such as a phone number or WABA. |
| `TagResource` | `POST /v1/tags/tag-resource` | - | `resourceArn`, `tags` | - | `TagResourceOutput` | `InternalServiceException`, `InvalidParametersException`, `ThrottledRequestException` | Adds or overwrites only the specified tags for the specified resource. When you specify an existing tag key, the value is overwritten with the new value. |
| `UntagResource` | `POST /v1/tags/untag-resource` | - | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `InternalServiceException`, `InvalidParametersException`, `ThrottledRequestException` | Removes the specified tags from a resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListTagsForResource` | - | `resourceArn -> resourceArn` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedByMetaException` | `structure` | message | You do not have sufficient access to perform this action. |
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `DependencyException` | `structure` | message | Thrown when performing an action because a dependency would be broken. |
| `InternalServiceException` | `structure` | message | The request processing has failed because of an unknown error, exception, or failure. |
| `InvalidParametersException` | `structure` | message | One or more parameters provided to the action are not valid. |
| `LimitExceededException` | `structure` | message | The request was denied because it would exceed one or more service quotas or limits. |
| `ResourceNotFoundException` | `structure` | message | The resource was not found. |
| `ThrottledRequestException` | `structure` | message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message | The request contains an invalid parameter value. |
| `ListTagsForResourceInput` | `structure` | resourceArn | - |
| `ListTagsForResourceOutput` | `structure` | statusCode, tags | - |
| `TagResourceInput` | `structure` | resourceArn, tags | - |
| `TagResourceOutput` | `structure` | statusCode | - |
| `UntagResourceInput` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceOutput` | `structure` | statusCode | - |
| `RegistrationStatus` | `enum` | COMPLETE, INCOMPLETE | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
