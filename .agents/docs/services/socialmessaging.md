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

### Get

- Operations: `GetLinkedWhatsAppBusinessAccount`, `GetLinkedWhatsAppBusinessAccountPhoneNumber`, `GetWhatsAppMessageMedia`, `GetWhatsAppMessageTemplate`
- Traits: `readonly` (3)
- Common required input members in this group: `id`, `mediaId`, `metaTemplateId`, `originationPhoneNumberId`

### List

- Operations: `ListLinkedWhatsAppBusinessAccounts`, `ListTagsForResource`, `ListWhatsAppMessageTemplates`, `ListWhatsAppTemplateLibrary`
- Traits: `paginated` (3), `readonly` (4)
- Common required input members in this group: `id`, `resourceArn`

### Create

- Operations: `CreateWhatsAppMessageTemplate`, `CreateWhatsAppMessageTemplateFromLibrary`, `CreateWhatsAppMessageTemplateMedia`
- Common required input members in this group: `id`, `metaLibraryTemplate`, `templateDefinition`

### Delete

- Operations: `DeleteWhatsAppMessageMedia`, `DeleteWhatsAppMessageTemplate`
- Traits: `idempotent` (2)
- Common required input members in this group: `id`, `mediaId`, `originationPhoneNumberId`, `templateName`

### Associate

- Operations: `AssociateWhatsAppBusinessAccount`

### Disassociate

- Operations: `DisassociateWhatsAppBusinessAccount`
- Traits: `idempotent` (1)
- Common required input members in this group: `id`

### Post

- Operations: `PostWhatsAppMessageMedia`
- Common required input members in this group: `originationPhoneNumberId`

### Put

- Operations: `PutWhatsAppBusinessAccountEventDestinations`
- Traits: `idempotent` (1)
- Common required input members in this group: `eventDestinations`, `id`

### Send

- Operations: `SendWhatsAppMessage`
- Common required input members in this group: `message`, `metaApiVersion`, `originationPhoneNumberId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tagKeys`

### Update

- Operations: `UpdateWhatsAppMessageTemplate`
- Common required input members in this group: `id`, `metaTemplateId`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateWhatsAppBusinessAccount` | `POST /v1/whatsapp/signup` | - | - | - | `AssociateWhatsAppBusinessAccountOutput` | `DependencyException`, `InvalidParametersException`, `LimitExceededException`, `ThrottledRequestException` | This is only used through the Amazon Web Services console during sign-up to associate your WhatsApp Business Account to your Amazon Web Services account. |
| `CreateWhatsAppMessageTemplate` | `POST /v1/whatsapp/template/put` | - | `id`, `templateDefinition` | - | `CreateWhatsAppMessageTemplateOutput` | `DependencyException`, `InternalServiceException`, `InvalidParametersException`, `ResourceNotFoundException`, `ThrottledRequestException` | Creates a new WhatsApp message template from a custom definition. Amazon Web Services End User Messaging Social does not store any WhatsApp message template content. |
| `CreateWhatsAppMessageTemplateFromLibrary` | `POST /v1/whatsapp/template/create` | - | `id`, `metaLibraryTemplate` | - | `CreateWhatsAppMessageTemplateFromLibraryOutput` | `DependencyException`, `InternalServiceException`, `InvalidParametersException`, `ResourceNotFoundException`, `ThrottledRequestException` | Creates a new WhatsApp message template using a template from Meta's template library. |
| `CreateWhatsAppMessageTemplateMedia` | `POST /v1/whatsapp/template/media` | - | `id` | - | `CreateWhatsAppMessageTemplateMediaOutput` | `DependencyException`, `InternalServiceException`, `InvalidParametersException`, `ResourceNotFoundException`, `ThrottledRequestException` | Uploads media for use in a WhatsApp message template. |
| `DeleteWhatsAppMessageMedia` | `DELETE /v1/whatsapp/media` | `idempotent` | `mediaId`, `originationPhoneNumberId` | - | `DeleteWhatsAppMessageMediaOutput` | `AccessDeniedByMetaException`, `DependencyException`, `InternalServiceException`, `InvalidParametersException`, `ResourceNotFoundException`, `ThrottledRequestException` | Delete a media object from the WhatsApp service. If the object is still in an Amazon S3 bucket you should delete it from there too. |
| `DeleteWhatsAppMessageTemplate` | `DELETE /v1/whatsapp/template` | `idempotent` | `id`, `templateName` | - | `DeleteWhatsAppMessageTemplateOutput` | `DependencyException`, `InternalServiceException`, `InvalidParametersException`, `ResourceNotFoundException`, `ThrottledRequestException` | Deletes a WhatsApp message template. |
| `DisassociateWhatsAppBusinessAccount` | `DELETE /v1/whatsapp/waba/disassociate` | `idempotent` | `id` | - | `DisassociateWhatsAppBusinessAccountOutput` | `DependencyException`, `InvalidParametersException`, `ResourceNotFoundException`, `ThrottledRequestException` | Disassociate a WhatsApp Business Account (WABA) from your Amazon Web Services account. |
| `GetLinkedWhatsAppBusinessAccount` | `GET /v1/whatsapp/waba/details` | `readonly` | `id` | - | `GetLinkedWhatsAppBusinessAccountOutput` | `DependencyException`, `InternalServiceException`, `InvalidParametersException`, `ResourceNotFoundException`, `ThrottledRequestException` | Get the details of your linked WhatsApp Business Account. |
| `GetLinkedWhatsAppBusinessAccountPhoneNumber` | `GET /v1/whatsapp/waba/phone/details` | `readonly` | `id` | - | `GetLinkedWhatsAppBusinessAccountPhoneNumberOutput` | `DependencyException`, `InternalServiceException`, `InvalidParametersException`, `ResourceNotFoundException`, `ThrottledRequestException` | Retrieve the WABA account id and phone number details of a WhatsApp business account phone number. |
| `GetWhatsAppMessageMedia` | `POST /v1/whatsapp/media/get` | - | `mediaId`, `originationPhoneNumberId` | - | `GetWhatsAppMessageMediaOutput` | `AccessDeniedByMetaException`, `DependencyException`, `InternalServiceException`, `InvalidParametersException`, `ResourceNotFoundException`, `ThrottledRequestException` | Get a media file from the WhatsApp service. On successful completion the media file is retrieved from Meta and stored in the specified Amazon S3 bucket. |
| `GetWhatsAppMessageTemplate` | `GET /v1/whatsapp/template` | `readonly` | `id`, `metaTemplateId` | - | `GetWhatsAppMessageTemplateOutput` | `DependencyException`, `InternalServiceException`, `InvalidParametersException`, `ResourceNotFoundException`, `ThrottledRequestException` | Retrieves a specific WhatsApp message template. |
| `ListLinkedWhatsAppBusinessAccounts` | `GET /v1/whatsapp/waba/list` | `readonly`, `paginated` | - | - | `ListLinkedWhatsAppBusinessAccountsOutput` | `InternalServiceException`, `InvalidParametersException`, `ResourceNotFoundException`, `ThrottledRequestException` | List all WhatsApp Business Accounts linked to your Amazon Web Services account. |
| `ListTagsForResource` | `GET /v1/tags/list` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `InternalServiceException`, `InvalidParametersException`, `ThrottledRequestException` | List all tags associated with a resource, such as a phone number or WABA. |
| `ListWhatsAppMessageTemplates` | `GET /v1/whatsapp/template/list` | `readonly`, `paginated` | `id` | - | `ListWhatsAppMessageTemplatesOutput` | `DependencyException`, `InternalServiceException`, `InvalidParametersException`, `ResourceNotFoundException`, `ThrottledRequestException` | Lists WhatsApp message templates for a specific WhatsApp Business Account. |
| `ListWhatsAppTemplateLibrary` | `POST /v1/whatsapp/template/library` | `readonly`, `paginated` | `id` | - | `ListWhatsAppTemplateLibraryOutput` | `DependencyException`, `InternalServiceException`, `InvalidParametersException`, `ResourceNotFoundException`, `ThrottledRequestException` | Lists templates available in Meta's template library for WhatsApp messaging. |
| `PostWhatsAppMessageMedia` | `POST /v1/whatsapp/media` | - | `originationPhoneNumberId` | - | `PostWhatsAppMessageMediaOutput` | `AccessDeniedByMetaException`, `DependencyException`, `InternalServiceException`, `InvalidParametersException`, `ResourceNotFoundException`, `ThrottledRequestException` | Upload a media file to the WhatsApp service. Only the specified `originationPhoneNumberId` has the permissions to send the media file when using SendWhatsAppMessage. |
| `PutWhatsAppBusinessAccountEventDestinations` | `PUT /v1/whatsapp/waba/eventdestinations` | `idempotent` | `eventDestinations`, `id` | - | `PutWhatsAppBusinessAccountEventDestinationsOutput` | `InternalServiceException`, `InvalidParametersException`, `ThrottledRequestException` | Add an event destination to log event data from WhatsApp for a WhatsApp Business Account (WABA). A WABA can only have one event destination at a time. |
| `SendWhatsAppMessage` | `POST /v1/whatsapp/send` | - | `message`, `metaApiVersion`, `originationPhoneNumberId` | - | `SendWhatsAppMessageOutput` | `DependencyException`, `InternalServiceException`, `InvalidParametersException`, `ResourceNotFoundException`, `ThrottledRequestException` | Send a WhatsApp message. For examples of sending a message using the Amazon Web Services CLI, see Sending messages in the Amazon Web Services End User Messaging Social User Guide . |
| `TagResource` | `POST /v1/tags/tag-resource` | - | `resourceArn`, `tags` | - | `TagResourceOutput` | `InternalServiceException`, `InvalidParametersException`, `ThrottledRequestException` | Adds or overwrites only the specified tags for the specified resource. When you specify an existing tag key, the value is overwritten with the new value. |
| `UntagResource` | `POST /v1/tags/untag-resource` | - | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `InternalServiceException`, `InvalidParametersException`, `ThrottledRequestException` | Removes the specified tags from a resource. |
| `UpdateWhatsAppMessageTemplate` | `POST /v1/whatsapp/template` | - | `id`, `metaTemplateId` | - | `UpdateWhatsAppMessageTemplateOutput` | `DependencyException`, `InternalServiceException`, `InvalidParametersException`, `ResourceNotFoundException`, `ThrottledRequestException` | Updates an existing WhatsApp message template. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListTagsForResource` | - | `resourceArn -> resourceArn` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InvalidParametersException` | `structure` | `message` | One or more parameters provided to the action are not valid. |
| `ThrottledRequestException` | `structure` | `message` | The request was denied due to request throttling. |
| `InternalServiceException` | `structure` | `message` | The request processing has failed because of an unknown error, exception, or failure. |
| `DependencyException` | `structure` | `message` | Thrown when performing an action because a dependency would be broken. |
| `ResourceNotFoundException` | `structure` | `message` | The resource was not found. |
| `AccessDeniedByMetaException` | `structure` | `message` | You do not have sufficient access to perform this action. |
| `AssociateWhatsAppBusinessAccountInput` | `structure` | `setupFinalization`, `signupCallback` | - |
| `AssociateWhatsAppBusinessAccountOutput` | `structure` | `signupCallbackResult`, `statusCode` | - |
| `LimitExceededException` | `structure` | `message` | The request was denied because it would exceed one or more service quotas or limits. |
| `CreateWhatsAppMessageTemplateInput` | `structure` | `id`, `templateDefinition` | - |
| `CreateWhatsAppMessageTemplateOutput` | `structure` | `category`, `metaTemplateId`, `templateStatus` | - |
| `CreateWhatsAppMessageTemplateFromLibraryInput` | `structure` | `id`, `metaLibraryTemplate` | - |
| `CreateWhatsAppMessageTemplateFromLibraryOutput` | `structure` | `category`, `metaTemplateId`, `templateStatus` | - |
| `CreateWhatsAppMessageTemplateMediaInput` | `structure` | `id`, `sourceS3File` | - |
| `CreateWhatsAppMessageTemplateMediaOutput` | `structure` | `metaHeaderHandle` | - |
| `DeleteWhatsAppMessageMediaInput` | `structure` | `mediaId`, `originationPhoneNumberId` | - |
| `DeleteWhatsAppMessageMediaOutput` | `structure` | `success` | - |
| `DeleteWhatsAppMessageTemplateInput` | `structure` | `deleteAllLanguages`, `id`, `metaTemplateId`, `templateName` | - |
| `DeleteWhatsAppMessageTemplateOutput` | `structure` | - | - |
| `DisassociateWhatsAppBusinessAccountInput` | `structure` | `id` | - |
| `DisassociateWhatsAppBusinessAccountOutput` | `structure` | - | - |
| `GetLinkedWhatsAppBusinessAccountInput` | `structure` | `id` | - |
| `GetLinkedWhatsAppBusinessAccountOutput` | `structure` | `account` | - |
| `GetLinkedWhatsAppBusinessAccountPhoneNumberInput` | `structure` | `id` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
