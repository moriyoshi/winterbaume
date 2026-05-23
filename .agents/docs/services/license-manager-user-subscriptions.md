# AWS License Manager User Subscriptions

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

With License Manager, you can create user-based subscriptions to utilize licensed software with a per user subscription fee on Amazon EC2 instances.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS License Manager User Subscriptions where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS License Manager User Subscriptions by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for AWS License Manager User Subscriptions resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS License Manager User Subscriptions workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Associate`, `Create`, `Delete`, `Deregister` operation families, including `ListIdentityProviders`, `ListInstances`, `ListLicenseServerEndpoints`, `ListProductSubscriptions`, `AssociateUser`, `CreateLicenseServerEndpoint`.

## Service Identity and Protocol

- AWS model slug: `license-manager-user-subscriptions`
- AWS SDK for Rust slug: `licensemanagerusersubscriptions`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/license-manager-user-subscriptions/service/2018-05-10/license-manager-user-subscriptions-2018-05-10.json`
- SDK ID: `License Manager User Subscriptions`
- Endpoint prefix: `-`
- ARN namespace: `license-manager-user-subscriptions`
- CloudFormation name: `-`
- CloudTrail event source: `license-manager-user-subscriptions.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (6), `Associate` (1), `Create` (1), `Delete` (1), `Deregister` (1), `Disassociate` (1), `Register` (1), `Start` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateUser`, `CreateLicenseServerEndpoint`, `DeleteLicenseServerEndpoint`, `DeregisterIdentityProvider`, `DisassociateUser`, `RegisterIdentityProvider`, `StartProductSubscription`, `StopProductSubscription`, `TagResource`, `UntagResource`, `UpdateIdentityProviderSettings`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `ListIdentityProviders`, `ListInstances`, `ListLicenseServerEndpoints`, `ListProductSubscriptions`, `ListTagsForResource`, `ListUserAssociations`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 9 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartProductSubscription`, `StopProductSubscription`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 17 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `EC2/VPC`, `RDS`, `Secrets Manager`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListIdentityProviders`, `ListInstances`, `ListLicenseServerEndpoints`, `ListProductSubscriptions`, `ListTagsForResource`, `ListUserAssociations`
- Traits: `paginated` (5), `readonly` (1)
- Common required input members in this group: `IdentityProvider`

### Associate

- Operations: `AssociateUser`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Create

- Operations: `CreateLicenseServerEndpoint`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Delete

- Operations: `DeleteLicenseServerEndpoint`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Deregister

- Operations: `DeregisterIdentityProvider`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociateUser`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Register

- Operations: `RegisterIdentityProvider`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Start

- Operations: `StartProductSubscription`
- Common required input members in this group: -

### Stop

- Operations: `StopProductSubscription`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Update

- Operations: `UpdateIdentityProviderSettings`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateUser` | `POST /user/AssociateUser` | `idempotent` | `Username`, `InstanceId`, `IdentityProvider` | - | `AssociateUserResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates the user to an EC2 instance to utilize user-based subscriptions. Your estimated bill for charges on the number of users and related costs will take 48 hours to appear for billing periods that haven't close ... |
| `CreateLicenseServerEndpoint` | `POST /license-server/CreateLicenseServerEndpoint` | `idempotent` | `IdentityProviderArn`, `LicenseServerSettings` | - | `CreateLicenseServerEndpointResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a network endpoint for the Remote Desktop Services (RDS) license server. |
| `DeleteLicenseServerEndpoint` | `POST /license-server/DeleteLicenseServerEndpoint` | `idempotent` | `LicenseServerEndpointArn`, `ServerType` | - | `DeleteLicenseServerEndpointResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Deletes a LicenseServerEndpoint resource. |
| `DeregisterIdentityProvider` | `POST /identity-provider/DeregisterIdentityProvider` | `idempotent` | - | - | `DeregisterIdentityProviderResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Deregisters the Active Directory identity provider from License Manager user-based subscriptions. |
| `DisassociateUser` | `POST /user/DisassociateUser` | `idempotent` | - | - | `DisassociateUserResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Disassociates the user from an EC2 instance providing user-based subscriptions. |
| `ListIdentityProviders` | `POST /identity-provider/ListIdentityProviders` | `paginated` | - | - | `ListIdentityProvidersResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Lists the Active Directory identity providers for user-based subscriptions. |
| `ListInstances` | `POST /instance/ListInstances` | `paginated` | - | - | `ListInstancesResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Lists the EC2 instances providing user-based subscriptions. |
| `ListLicenseServerEndpoints` | `POST /license-server/ListLicenseServerEndpoints` | `paginated` | - | - | `ListLicenseServerEndpointsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | List the Remote Desktop Services (RDS) License Server endpoints |
| `ListProductSubscriptions` | `POST /user/ListProductSubscriptions` | `paginated` | `IdentityProvider` | - | `ListProductSubscriptionsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Lists the user-based subscription products available from an identity provider. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns the list of tags for the specified resource. |
| `ListUserAssociations` | `POST /user/ListUserAssociations` | `paginated` | `InstanceId`, `IdentityProvider` | - | `ListUserAssociationsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Lists user associations for an identity provider. |
| `RegisterIdentityProvider` | `POST /identity-provider/RegisterIdentityProvider` | `idempotent` | `IdentityProvider`, `Product` | - | `RegisterIdentityProviderResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Registers an identity provider for user-based subscriptions. |
| `StartProductSubscription` | `POST /user/StartProductSubscription` | - | `Username`, `IdentityProvider`, `Product` | - | `StartProductSubscriptionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Starts a product subscription for a user with the specified identity provider. Your estimated bill for charges on the number of users and related costs will take 48 hours to appear for billing periods that haven't cl ... |
| `StopProductSubscription` | `POST /user/StopProductSubscription` | - | - | - | `StopProductSubscriptionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Stops a product subscription for a user with the specified identity provider. |
| `TagResource` | `PUT /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Adds tags to a resource. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException` | Removes tags from a resource. |
| `UpdateIdentityProviderSettings` | `POST /identity-provider/UpdateIdentityProviderSettings` | `idempotent` | `UpdateSettings` | - | `UpdateIdentityProviderSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Updates additional product configuration settings for the registered identity provider. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You don't have sufficient access to perform this action. |
| `ConflictException` | `structure` | message | The request couldn't be completed because it conflicted with the current state of the resource. |
| `InternalServerException` | `structure` | message | An exception occurred with the service. |
| `ResourceNotFoundException` | `structure` | message | The resource couldn't be found. |
| `ServiceQuotaExceededException` | `structure` | message | The request failed because a service quota is exceeded. |
| `ThrottlingException` | `structure` | message | The request was denied because of request throttling. Retry the request. |
| `ValidationException` | `structure` | message | A parameter is not valid. |
| `AssociateUserRequest` | `structure` | Username, InstanceId, IdentityProvider, Domain, Tags | - |
| `AssociateUserResponse` | `structure` | InstanceUserSummary | - |
| `CreateLicenseServerEndpointRequest` | `structure` | IdentityProviderArn, LicenseServerSettings, Tags | - |
| `CreateLicenseServerEndpointResponse` | `structure` | IdentityProviderArn, LicenseServerEndpointArn | - |
| `DeleteLicenseServerEndpointRequest` | `structure` | LicenseServerEndpointArn, ServerType | - |
| `DeleteLicenseServerEndpointResponse` | `structure` | LicenseServerEndpoint | - |
| `DeregisterIdentityProviderRequest` | `structure` | IdentityProvider, Product, IdentityProviderArn | - |
| `DeregisterIdentityProviderResponse` | `structure` | IdentityProviderSummary | - |
| `DisassociateUserRequest` | `structure` | Username, InstanceId, IdentityProvider, InstanceUserArn, Domain | - |
| `DisassociateUserResponse` | `structure` | InstanceUserSummary | - |
| `ListIdentityProvidersRequest` | `structure` | MaxResults, Filters, NextToken | - |
| `ListIdentityProvidersResponse` | `structure` | IdentityProviderSummaries, NextToken | - |
| `ListInstancesRequest` | `structure` | MaxResults, NextToken, Filters | - |
| `ListInstancesResponse` | `structure` | InstanceSummaries, NextToken | - |
| `ListLicenseServerEndpointsRequest` | `structure` | MaxResults, Filters, NextToken | - |
| `ListLicenseServerEndpointsResponse` | `structure` | LicenseServerEndpoints, NextToken | - |
| `ListProductSubscriptionsRequest` | `structure` | Product, IdentityProvider, MaxResults, Filters, NextToken | - |
| `ListProductSubscriptionsResponse` | `structure` | ProductUserSummaries, NextToken | - |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `ListUserAssociationsRequest` | `structure` | InstanceId, IdentityProvider, MaxResults, Filters, NextToken | - |
| `ListUserAssociationsResponse` | `structure` | InstanceUserSummaries, NextToken | - |
| `RegisterIdentityProviderRequest` | `structure` | IdentityProvider, Product, Settings, Tags | - |
| `RegisterIdentityProviderResponse` | `structure` | IdentityProviderSummary | - |
| `StartProductSubscriptionRequest` | `structure` | Username, IdentityProvider, Product, Domain, Tags | - |
| `StartProductSubscriptionResponse` | `structure` | ProductUserSummary | - |
| `StopProductSubscriptionRequest` | `structure` | Username, IdentityProvider, Product, ProductUserArn, Domain | - |
| `StopProductSubscriptionResponse` | `structure` | ProductUserSummary | - |
| `TagResourceRequest` | `structure` | ResourceArn, Tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | ResourceArn, TagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `UpdateIdentityProviderSettingsRequest` | `structure` | IdentityProvider, Product, IdentityProviderArn, UpdateSettings | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
