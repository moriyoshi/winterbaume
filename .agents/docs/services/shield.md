# AWS Shield

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Shield Advanced This is the Shield Advanced API Reference . This guide is for developers who need detailed information about the Shield Advanced API actions, data types, and errors. For detailed information about WAF and Shield Advanced features and an overview of how to use the WAF and Shield Advanced APIs, see the WAF and Shield Developer Guide.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Shield where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for AWS Shield by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for AWS Shield by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented AWS Shield workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `List`, `Associate`, `Update`, `Create` operation families, including `DescribeAttack`, `DescribeAttackStatistics`, `DescribeDRTAccess`, `DescribeEmergencyContactSettings`, `ListAttacks`, `ListProtectionGroups`.

## Service Identity and Protocol

- AWS model slug: `shield`
- AWS SDK for Rust slug: `shield`
- Model version: `2016-06-02`
- Model file: `vendor/api-models-aws/models/shield/service/2016-06-02/shield-2016-06-02.json`
- SDK ID: `Shield`
- Endpoint prefix: `shield`
- ARN namespace: `shield`
- CloudFormation name: `Shield`
- CloudTrail event source: `shield.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (7), `List` (5), `Associate` (4), `Update` (4), `Create` (3), `Delete` (3), `Disassociate` (3), `Disable` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateDRTLogBucket`, `AssociateDRTRole`, `AssociateHealthCheck`, `AssociateProactiveEngagementDetails`, `CreateProtection`, `CreateProtectionGroup`, `CreateSubscription`, `DeleteProtection`, `DeleteProtectionGroup`, `DeleteSubscription`, `DisableApplicationLayerAutomaticResponse`, `DisableProactiveEngagement`, `DisassociateDRTLogBucket`, `DisassociateDRTRole`, `DisassociateHealthCheck`, `EnableApplicationLayerAutomaticResponse`, `EnableProactiveEngagement`, `TagResource`, `UntagResource`, `UpdateApplicationLayerAutomaticResponse`, ... (+3).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAttack`, `DescribeAttackStatistics`, `DescribeDRTAccess`, `DescribeEmergencyContactSettings`, `DescribeProtection`, `DescribeProtectionGroup`, `DescribeSubscription`, `GetSubscriptionState`, `ListAttacks`, `ListProtectionGroups`, `ListProtections`, `ListResourcesInProtectionGroup`, `ListTagsForResource`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 36 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Describe

- Operations: `DescribeAttack`, `DescribeAttackStatistics`, `DescribeDRTAccess`, `DescribeEmergencyContactSettings`, `DescribeProtection`, `DescribeProtectionGroup`, `DescribeSubscription`
- Common required input members in this group: `AttackId`, `ProtectionGroupId`

### List

- Operations: `ListAttacks`, `ListProtectionGroups`, `ListProtections`, `ListResourcesInProtectionGroup`, `ListTagsForResource`
- Traits: `paginated` (4)
- Common required input members in this group: `ProtectionGroupId`, `ResourceARN`

### Associate

- Operations: `AssociateDRTLogBucket`, `AssociateDRTRole`, `AssociateHealthCheck`, `AssociateProactiveEngagementDetails`
- Common required input members in this group: `EmergencyContactList`, `HealthCheckArn`, `LogBucket`, `ProtectionId`, `RoleArn`

### Update

- Operations: `UpdateApplicationLayerAutomaticResponse`, `UpdateEmergencyContactSettings`, `UpdateProtectionGroup`, `UpdateSubscription`
- Common required input members in this group: `Action`, `Aggregation`, `Pattern`, `ProtectionGroupId`, `ResourceArn`

### Create

- Operations: `CreateProtection`, `CreateProtectionGroup`, `CreateSubscription`
- Common required input members in this group: `Aggregation`, `Name`, `Pattern`, `ProtectionGroupId`, `ResourceArn`

### Delete

- Operations: `DeleteProtection`, `DeleteProtectionGroup`, `DeleteSubscription`
- Common required input members in this group: `ProtectionGroupId`, `ProtectionId`

### Disassociate

- Operations: `DisassociateDRTLogBucket`, `DisassociateDRTRole`, `DisassociateHealthCheck`
- Common required input members in this group: `HealthCheckArn`, `LogBucket`, `ProtectionId`

### Disable

- Operations: `DisableApplicationLayerAutomaticResponse`, `DisableProactiveEngagement`
- Common required input members in this group: `ResourceArn`

### Enable

- Operations: `EnableApplicationLayerAutomaticResponse`, `EnableProactiveEngagement`
- Common required input members in this group: `Action`, `ResourceArn`

### Get

- Operations: `GetSubscriptionState`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateDRTLogBucket` | - | - | `LogBucket` | - | `AssociateDRTLogBucketResponse` | `AccessDeniedForDependencyException`, `InternalErrorException`, `InvalidOperationException`, `InvalidParameterException`, `LimitsExceededException`, `NoAssociatedRoleException`, `OptimisticLockException`, `ResourceNotFoundException` | Authorizes the Shield Response Team (SRT) to access the specified Amazon S3 bucket containing log data such as Application Load Balancer access logs, CloudFront logs, or logs from third party sources. You can associate up to 10 Amazon S3 buckets with your... |
| `AssociateDRTRole` | - | - | `RoleArn` | - | `AssociateDRTRoleResponse` | `AccessDeniedForDependencyException`, `InternalErrorException`, `InvalidOperationException`, `InvalidParameterException`, `OptimisticLockException`, `ResourceNotFoundException` | Authorizes the Shield Response Team (SRT) using the specified role, to access your Amazon Web Services account to assist with DDoS attack mitigation during potential attacks. This enables the SRT to inspect your WAF configuration and create or update WAF... |
| `AssociateHealthCheck` | - | - | `HealthCheckArn`, `ProtectionId` | - | `AssociateHealthCheckResponse` | `InternalErrorException`, `InvalidParameterException`, `InvalidResourceException`, `LimitsExceededException`, `OptimisticLockException`, `ResourceNotFoundException` | Adds health-based detection to the Shield Advanced protection for a resource. Shield Advanced health-based detection uses the health of your Amazon Web Services resource to improve responsiveness and accuracy in attack detection and response. |
| `AssociateProactiveEngagementDetails` | - | - | `EmergencyContactList` | - | `AssociateProactiveEngagementDetailsResponse` | `InternalErrorException`, `InvalidOperationException`, `InvalidParameterException`, `OptimisticLockException`, `ResourceNotFoundException` | Initializes proactive engagement and sets the list of contacts for the Shield Response Team (SRT) to use. You must provide at least one phone number in the emergency contact list. |
| `CreateProtection` | - | - | `Name`, `ResourceArn` | - | `CreateProtectionResponse` | `InternalErrorException`, `InvalidOperationException`, `InvalidParameterException`, `InvalidResourceException`, `LimitsExceededException`, `OptimisticLockException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException` | Enables Shield Advanced for a specific Amazon Web Services resource. The resource can be an Amazon CloudFront distribution, Amazon Route 53 hosted zone, Global Accelerator standard accelerator, Elastic IP Address, Application Load Balancer, or a Classic Load... |
| `CreateProtectionGroup` | - | - | `Aggregation`, `Pattern`, `ProtectionGroupId` | - | `CreateProtectionGroupResponse` | `InternalErrorException`, `InvalidParameterException`, `LimitsExceededException`, `OptimisticLockException`, `ResourceAlreadyExistsException`, `ResourceNotFoundException` | Creates a grouping of protected resources so they can be handled as a collective. This resource grouping improves the accuracy of detection and reduces false positives. |
| `CreateSubscription` | - | - | - | - | `CreateSubscriptionResponse` | `InternalErrorException`, `ResourceAlreadyExistsException` | Activates Shield Advanced for an account. For accounts that are members of an Organizations organization, Shield Advanced subscriptions are billed against the organization's payer account, regardless of whether the payer account itself is subscribed. |
| `DeleteProtection` | - | - | `ProtectionId` | - | `DeleteProtectionResponse` | `InternalErrorException`, `OptimisticLockException`, `ResourceNotFoundException` | Deletes an Shield Advanced Protection. |
| `DeleteProtectionGroup` | - | - | `ProtectionGroupId` | - | `DeleteProtectionGroupResponse` | `InternalErrorException`, `OptimisticLockException`, `ResourceNotFoundException` | Removes the specified protection group. |
| `DeleteSubscription` | - | - | - | - | `DeleteSubscriptionResponse` | `InternalErrorException`, `LockedSubscriptionException`, `ResourceNotFoundException` | Removes Shield Advanced from an account. Shield Advanced requires a 1-year subscription commitment. |
| `DescribeAttack` | - | - | `AttackId` | - | `DescribeAttackResponse` | `AccessDeniedException`, `InternalErrorException` | Describes the details of a DDoS attack. |
| `DescribeAttackStatistics` | - | - | - | - | `DescribeAttackStatisticsResponse` | `InternalErrorException` | Provides information about the number and type of attacks Shield has detected in the last year for all resources that belong to your account, regardless of whether you've defined Shield protections for them. This operation is available to Shield customers as... |
| `DescribeDRTAccess` | - | - | - | - | `DescribeDRTAccessResponse` | `InternalErrorException`, `ResourceNotFoundException` | Returns the current role and list of Amazon S3 log buckets used by the Shield Response Team (SRT) to access your Amazon Web Services account while assisting with attack mitigation. |
| `DescribeEmergencyContactSettings` | - | - | - | - | `DescribeEmergencyContactSettingsResponse` | `InternalErrorException`, `ResourceNotFoundException` | A list of email addresses and phone numbers that the Shield Response Team (SRT) can use to contact you if you have proactive engagement enabled, for escalations to the SRT and to initiate proactive customer support. |
| `DescribeProtection` | - | - | - | - | `DescribeProtectionResponse` | `InternalErrorException`, `InvalidParameterException`, `ResourceNotFoundException` | Lists the details of a Protection object. |
| `DescribeProtectionGroup` | - | - | `ProtectionGroupId` | - | `DescribeProtectionGroupResponse` | `InternalErrorException`, `ResourceNotFoundException` | Returns the specification for the specified protection group. |
| `DescribeSubscription` | - | - | - | - | `DescribeSubscriptionResponse` | `InternalErrorException`, `ResourceNotFoundException` | Provides details about the Shield Advanced subscription for an account. |
| `DisableApplicationLayerAutomaticResponse` | - | - | `ResourceArn` | - | `DisableApplicationLayerAutomaticResponseResponse` | `InternalErrorException`, `InvalidOperationException`, `InvalidParameterException`, `OptimisticLockException`, `ResourceNotFoundException` | Disable the Shield Advanced automatic application layer DDoS mitigation feature for the protected resource. This stops Shield Advanced from creating, verifying, and applying WAF rules for attacks that it detects for the resource. |
| `DisableProactiveEngagement` | - | - | - | - | `DisableProactiveEngagementResponse` | `InternalErrorException`, `InvalidOperationException`, `InvalidParameterException`, `OptimisticLockException`, `ResourceNotFoundException` | Removes authorization from the Shield Response Team (SRT) to notify contacts about escalations to the SRT and to initiate proactive customer support. |
| `DisassociateDRTLogBucket` | - | - | `LogBucket` | - | `DisassociateDRTLogBucketResponse` | `AccessDeniedForDependencyException`, `InternalErrorException`, `InvalidOperationException`, `NoAssociatedRoleException`, `OptimisticLockException`, `ResourceNotFoundException` | Removes the Shield Response Team's (SRT) access to the specified Amazon S3 bucket containing the logs that you shared previously. |
| `DisassociateDRTRole` | - | - | - | - | `DisassociateDRTRoleResponse` | `InternalErrorException`, `InvalidOperationException`, `OptimisticLockException`, `ResourceNotFoundException` | Removes the Shield Response Team's (SRT) access to your Amazon Web Services account. |
| `DisassociateHealthCheck` | - | - | `HealthCheckArn`, `ProtectionId` | - | `DisassociateHealthCheckResponse` | `InternalErrorException`, `InvalidParameterException`, `InvalidResourceException`, `OptimisticLockException`, `ResourceNotFoundException` | Removes health-based detection from the Shield Advanced protection for a resource. Shield Advanced health-based detection uses the health of your Amazon Web Services resource to improve responsiveness and accuracy in attack detection and response. |
| `EnableApplicationLayerAutomaticResponse` | - | - | `Action`, `ResourceArn` | - | `EnableApplicationLayerAutomaticResponseResponse` | `InternalErrorException`, `InvalidOperationException`, `InvalidParameterException`, `LimitsExceededException`, `OptimisticLockException`, `ResourceNotFoundException` | Enable the Shield Advanced automatic application layer DDoS mitigation for the protected resource. This feature is available for Amazon CloudFront distributions and Application Load Balancers only. |
| `EnableProactiveEngagement` | - | - | - | - | `EnableProactiveEngagementResponse` | `InternalErrorException`, `InvalidOperationException`, `InvalidParameterException`, `OptimisticLockException`, `ResourceNotFoundException` | Authorizes the Shield Response Team (SRT) to use email and phone to notify contacts about escalations to the SRT and to initiate proactive customer support. |
| `GetSubscriptionState` | - | - | - | - | `GetSubscriptionStateResponse` | `InternalErrorException` | Returns the `SubscriptionState`, either `Active` or `Inactive`. |
| `ListAttacks` | - | `paginated` | - | - | `ListAttacksResponse` | `InternalErrorException`, `InvalidOperationException`, `InvalidParameterException` | Returns all ongoing DDoS attacks or all DDoS attacks during a specified time period. |
| `ListProtectionGroups` | - | `paginated` | - | - | `ListProtectionGroupsResponse` | `InternalErrorException`, `InvalidPaginationTokenException`, `ResourceNotFoundException` | Retrieves ProtectionGroup objects for the account. You can retrieve all protection groups or you can provide filtering criteria and retrieve just the subset of protection groups that match the criteria. |
| `ListProtections` | - | `paginated` | - | - | `ListProtectionsResponse` | `InternalErrorException`, `InvalidPaginationTokenException`, `ResourceNotFoundException` | Retrieves Protection objects for the account. You can retrieve all protections or you can provide filtering criteria and retrieve just the subset of protections that match the criteria. |
| `ListResourcesInProtectionGroup` | - | `paginated` | `ProtectionGroupId` | - | `ListResourcesInProtectionGroupResponse` | `InternalErrorException`, `InvalidPaginationTokenException`, `ResourceNotFoundException` | Retrieves the resources that are included in the protection group. |
| `ListTagsForResource` | - | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `InternalErrorException`, `InvalidResourceException`, `ResourceNotFoundException` | Gets information about Amazon Web Services tags for a specified Amazon Resource Name (ARN) in Shield. |
| `TagResource` | - | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `InternalErrorException`, `InvalidParameterException`, `InvalidResourceException`, `ResourceNotFoundException` | Adds or updates tags for a resource in Shield. |
| `UntagResource` | - | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `InternalErrorException`, `InvalidParameterException`, `InvalidResourceException`, `ResourceNotFoundException` | Removes tags from a resource in Shield. |
| `UpdateApplicationLayerAutomaticResponse` | - | - | `Action`, `ResourceArn` | - | `UpdateApplicationLayerAutomaticResponseResponse` | `InternalErrorException`, `InvalidOperationException`, `InvalidParameterException`, `OptimisticLockException`, `ResourceNotFoundException` | Updates an existing Shield Advanced automatic application layer DDoS mitigation configuration for the specified resource. |
| `UpdateEmergencyContactSettings` | - | - | - | - | `UpdateEmergencyContactSettingsResponse` | `InternalErrorException`, `InvalidParameterException`, `OptimisticLockException`, `ResourceNotFoundException` | Updates the details of the list of email addresses and phone numbers that the Shield Response Team (SRT) can use to contact you if you have proactive engagement enabled, for escalations to the SRT and to initiate proactive customer support. |
| `UpdateProtectionGroup` | - | - | `Aggregation`, `Pattern`, `ProtectionGroupId` | - | `UpdateProtectionGroupResponse` | `InternalErrorException`, `InvalidParameterException`, `OptimisticLockException`, `ResourceNotFoundException` | Updates an existing protection group. A protection group is a grouping of protected resources so they can be handled as a collective. |
| `UpdateSubscription` | - | - | - | - | `UpdateSubscriptionResponse` | `InternalErrorException`, `InvalidParameterException`, `LockedSubscriptionException`, `OptimisticLockException`, `ResourceNotFoundException` | Updates the details of an existing subscription. Only enter values for parameters you want to change. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalErrorException` | `structure` | `message` | Exception that indicates that a problem occurred with the service infrastructure. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceType` | Exception indicating the specified resource does not exist. |
| `InvalidParameterException` | `structure` | `fields`, `message`, `reason` | Exception that indicates that the parameters passed to the API are invalid. |
| `OptimisticLockException` | `structure` | `message` | Exception that indicates that the resource state has been modified by another client. |
| `InvalidOperationException` | `structure` | `message` | Exception that indicates that the operation would not cause any change to occur. |
| `InvalidResourceException` | `structure` | `message` | Exception that indicates that the resource is invalid. |
| `LimitsExceededException` | `structure` | `Limit`, `Type`, `message` | Exception that indicates that the operation would exceed a limit. |
| `AccessDeniedForDependencyException` | `structure` | `message` | In order to grant the necessary access to the Shield Response Team (SRT) the user submitting the request must have the `iam:PassRole` permission. |
| `ResourceAlreadyExistsException` | `structure` | `message`, `resourceType` | Exception indicating the specified resource already exists. |
| `InvalidPaginationTokenException` | `structure` | `message` | Exception that indicates that the `NextToken` specified in the request is invalid. |
| `NoAssociatedRoleException` | `structure` | `message` | The ARN of the role that you specified does not exist. |
| `LockedSubscriptionException` | `structure` | `message` | You are trying to update a subscription that has not yet completed the 1-year commitment. |
| `AssociateDRTLogBucketRequest` | `structure` | `LogBucket` | - |
| `AssociateDRTLogBucketResponse` | `structure` | - | - |
| `AssociateDRTRoleRequest` | `structure` | `RoleArn` | - |
| `AssociateDRTRoleResponse` | `structure` | - | - |
| `AssociateHealthCheckRequest` | `structure` | `HealthCheckArn`, `ProtectionId` | - |
| `AssociateHealthCheckResponse` | `structure` | - | - |
| `AssociateProactiveEngagementDetailsRequest` | `structure` | `EmergencyContactList` | - |
| `AssociateProactiveEngagementDetailsResponse` | `structure` | - | - |
| `CreateProtectionRequest` | `structure` | `Name`, `ResourceArn`, `Tags` | - |
| `CreateProtectionResponse` | `structure` | `ProtectionId` | - |
| `CreateProtectionGroupRequest` | `structure` | `Aggregation`, `Members`, `Pattern`, `ProtectionGroupId`, `ResourceType`, `Tags` | - |
| `CreateProtectionGroupResponse` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
