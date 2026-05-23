# AWS License Manager Linux Subscriptions

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

With License Manager, you can discover and track your commercial Linux subscriptions on running Amazon EC2 instances.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS License Manager Linux Subscriptions workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Deregister`, `Register`, `Tag` operation families, including `ListLinuxSubscriptionInstances`, `ListLinuxSubscriptions`, `ListRegisteredSubscriptionProviders`, `ListTagsForResource`, `GetRegisteredSubscriptionProvider`, `GetServiceSettings`.

## Service Identity and Protocol

- AWS model slug: `license-manager-linux-subscriptions`
- AWS SDK for Rust slug: `licensemanagerlinuxsubscriptions`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/license-manager-linux-subscriptions/service/2018-05-10/license-manager-linux-subscriptions-2018-05-10.json`
- SDK ID: `License Manager Linux Subscriptions`
- Endpoint prefix: `-`
- ARN namespace: `license-manager-linux-subscriptions`
- CloudFormation name: `-`
- CloudTrail event source: `license-manager-linux-subscriptions.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Get` (2), `Deregister` (1), `Register` (1), `Tag` (1), `Untag` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `DeregisterSubscriptionProvider`, `RegisterSubscriptionProvider`, `TagResource`, `UntagResource`, `UpdateServiceSettings`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetRegisteredSubscriptionProvider`, `GetServiceSettings`, `ListLinuxSubscriptionInstances`, `ListLinuxSubscriptions`, `ListRegisteredSubscriptionProviders`, `ListTagsForResource`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 10 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 11 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `EC2/VPC`.

## Operation Groups

### List

- Operations: `ListLinuxSubscriptionInstances`, `ListLinuxSubscriptions`, `ListRegisteredSubscriptionProviders`, `ListTagsForResource`
- Traits: `idempotent` (3), `paginated` (3), `readonly` (1)
- Common required input members in this group: `resourceArn`

### Get

- Operations: `GetRegisteredSubscriptionProvider`, `GetServiceSettings`
- Traits: `idempotent` (2)
- Common required input members in this group: `SubscriptionProviderArn`

### Deregister

- Operations: `DeregisterSubscriptionProvider`
- Traits: `idempotent` (1)
- Common required input members in this group: `SubscriptionProviderArn`

### Register

- Operations: `RegisterSubscriptionProvider`
- Traits: `idempotent` (1)
- Common required input members in this group: `SecretArn`, `SubscriptionProviderSource`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

### Update

- Operations: `UpdateServiceSettings`
- Traits: `idempotent` (1)
- Common required input members in this group: `LinuxSubscriptionsDiscovery`, `LinuxSubscriptionsDiscoverySettings`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DeregisterSubscriptionProvider` | `POST /subscription/DeregisterSubscriptionProvider` | `idempotent` | `SubscriptionProviderArn` | - | `DeregisterSubscriptionProviderResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Remove a third-party subscription provider from the Bring Your Own License (BYOL) subscriptions registered to your account. |
| `GetRegisteredSubscriptionProvider` | `POST /subscription/GetRegisteredSubscriptionProvider` | `idempotent` | `SubscriptionProviderArn` | - | `GetRegisteredSubscriptionProviderResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get details for a Bring Your Own License (BYOL) subscription that's registered to your account. |
| `GetServiceSettings` | `POST /subscription/GetServiceSettings` | `idempotent` | - | - | `GetServiceSettingsResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the Linux subscriptions service settings for your account. |
| `ListLinuxSubscriptionInstances` | `POST /subscription/ListLinuxSubscriptionInstances` | `idempotent`, `paginated` | - | - | `ListLinuxSubscriptionInstancesResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the running Amazon EC2 instances that were discovered with commercial Linux subscriptions. |
| `ListLinuxSubscriptions` | `POST /subscription/ListLinuxSubscriptions` | `idempotent`, `paginated` | - | - | `ListLinuxSubscriptionsResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the Linux subscriptions that have been discovered. If you have linked your organization, the returned results will include data aggregated across your accounts in Organizations. |
| `ListRegisteredSubscriptionProviders` | `POST /subscription/ListRegisteredSubscriptionProviders` | `idempotent`, `paginated` | - | - | `ListRegisteredSubscriptionProvidersResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | List Bring Your Own License (BYOL) subscription registration resources for your account. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | List the metadata tags that are assigned to the specified Amazon Web Services resource. |
| `RegisterSubscriptionProvider` | `POST /subscription/RegisterSubscriptionProvider` | `idempotent` | `SecretArn`, `SubscriptionProviderSource` | - | `RegisterSubscriptionProviderResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Register the supported third-party subscription provider for your Bring Your Own License (BYOL) subscription. |
| `TagResource` | `PUT /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Add metadata tags to the specified Amazon Web Services resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException` | Remove one or more metadata tag from the specified Amazon Web Services resource. |
| `UpdateServiceSettings` | `POST /subscription/UpdateServiceSettings` | `idempotent` | `LinuxSubscriptionsDiscovery`, `LinuxSubscriptionsDiscoverySettings` | - | `UpdateServiceSettingsResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Updates the service settings for Linux subscriptions. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `message` | An exception occurred with the service. |
| `ValidationException` | `structure` | `message` | The provided input is not valid. |
| `ThrottlingException` | `structure` | `message` | The request was denied due to request throttling. |
| `ResourceNotFoundException` | `structure` | `message` | Unable to find the requested Amazon Web Services resource. |
| `DeregisterSubscriptionProviderRequest` | `structure` | `SubscriptionProviderArn` | - |
| `DeregisterSubscriptionProviderResponse` | `structure` | - | - |
| `GetRegisteredSubscriptionProviderRequest` | `structure` | `SubscriptionProviderArn` | - |
| `GetRegisteredSubscriptionProviderResponse` | `structure` | `LastSuccessfulDataRetrievalTime`, `SecretArn`, `SubscriptionProviderArn`, `SubscriptionProviderSource`, `SubscriptionProviderStatus`, `SubscriptionProviderStatusMessage` | - |
| `GetServiceSettingsRequest` | `structure` | - | - |
| `GetServiceSettingsResponse` | `structure` | `HomeRegions`, `LinuxSubscriptionsDiscovery`, `LinuxSubscriptionsDiscoverySettings`, `Status`, `StatusMessage` | - |
| `ListLinuxSubscriptionInstancesRequest` | `structure` | `Filters`, `MaxResults`, `NextToken` | NextToken length limit is half of ddb accepted limit. |
| `ListLinuxSubscriptionInstancesResponse` | `structure` | `Instances`, `NextToken` | - |
| `ListLinuxSubscriptionsRequest` | `structure` | `Filters`, `MaxResults`, `NextToken` | NextToken length limit is half of ddb accepted limit. |
| `ListLinuxSubscriptionsResponse` | `structure` | `NextToken`, `Subscriptions` | - |
| `ListRegisteredSubscriptionProvidersRequest` | `structure` | `MaxResults`, `NextToken`, `SubscriptionProviderSources` | - |
| `ListRegisteredSubscriptionProvidersResponse` | `structure` | `NextToken`, `RegisteredSubscriptionProviders` | - |
| `ListTagsForResourceRequest` | `structure` | `resourceArn` | - |
| `ListTagsForResourceResponse` | `structure` | `tags` | - |
| `RegisterSubscriptionProviderRequest` | `structure` | `SecretArn`, `SubscriptionProviderSource`, `Tags` | - |
| `RegisterSubscriptionProviderResponse` | `structure` | `SubscriptionProviderArn`, `SubscriptionProviderSource`, `SubscriptionProviderStatus` | - |
| `TagResourceRequest` | `structure` | `resourceArn`, `tags` | - |
| `TagResourceResponse` | `structure` | - | - |
| `UntagResourceRequest` | `structure` | `resourceArn`, `tagKeys` | - |
| `UntagResourceResponse` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
