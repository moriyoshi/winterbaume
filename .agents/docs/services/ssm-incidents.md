# AWS Systems Manager Incident Manager

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Systems Manager Incident Manager is an incident management console designed to help users mitigate and recover from incidents affecting their Amazon Web Services-hosted applications. An incident is any unplanned interruption or reduction in quality of services. Incident Manager increases incident resolution by notifying responders of impact, highlighting relevant troubleshooting data, and providing collaboration tools to get services back up and running. To achieve the primary goal of reducing the time-to-resolution of critical incidents, Incident Manager automates response plans and enables responder team escalation.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Systems Manager Incident Manager workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Update`, `Delete`, `Get`, `Create` operation families, including `ListIncidentFindings`, `ListIncidentRecords`, `ListRelatedItems`, `ListReplicationSets`, `UpdateDeletionProtection`, `UpdateIncidentRecord`.

## Service Identity and Protocol

- AWS model slug: `ssm-incidents`
- AWS SDK for Rust slug: `ssmincidents`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/ssm-incidents/service/2018-05-10/ssm-incidents-2018-05-10.json`
- SDK ID: `SSM Incidents`
- Endpoint prefix: `-`
- ARN namespace: `ssm-incidents`
- CloudFormation name: `SSMIncidents`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (7), `Update` (6), `Delete` (5), `Get` (5), `Create` (3), `Batch` (1), `Put` (1), `Start` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchGetIncidentFindings`, `CreateReplicationSet`, `CreateResponsePlan`, `CreateTimelineEvent`, `DeleteIncidentRecord`, `DeleteReplicationSet`, `DeleteResourcePolicy`, `DeleteResponsePlan`, `DeleteTimelineEvent`, `PutResourcePolicy`, `StartIncident`, `TagResource`, `UntagResource`, `UpdateDeletionProtection`, `UpdateIncidentRecord`, `UpdateRelatedItems`, `UpdateReplicationSet`, `UpdateResponsePlan`, `UpdateTimelineEvent`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `BatchGetIncidentFindings`, `GetIncidentRecord`, `GetReplicationSet`, `GetResourcePolicies`, `GetResponsePlan`, `GetTimelineEvent`, `ListIncidentFindings`, `ListIncidentRecords`, `ListRelatedItems`, `ListReplicationSets`, `ListResponsePlans`, `ListTagsForResource`, `ListTimelineEvents`.
- Pagination is modelled for 7 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 14 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartIncident`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 31 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `KMS`, `CloudWatch`, `EventBridge`, `SNS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListIncidentFindings`, `ListIncidentRecords`, `ListRelatedItems`, `ListReplicationSets`, `ListResponsePlans`, `ListTagsForResource`, `ListTimelineEvents`
- Traits: `paginated` (6), `readonly` (7)
- Common required input members in this group: `incidentRecordArn`, `resourceArn`

### Update

- Operations: `UpdateDeletionProtection`, `UpdateIncidentRecord`, `UpdateRelatedItems`, `UpdateReplicationSet`, `UpdateResponsePlan`, `UpdateTimelineEvent`
- Traits: `idempotency-token` (6), `idempotent` (4)
- Common required input members in this group: `actions`, `arn`, `deletionProtected`, `eventId`, `incidentRecordArn`, `relatedItemsUpdate`

### Delete

- Operations: `DeleteIncidentRecord`, `DeleteReplicationSet`, `DeleteResourcePolicy`, `DeleteResponsePlan`, `DeleteTimelineEvent`
- Traits: `idempotent` (3)
- Common required input members in this group: `arn`, `eventId`, `incidentRecordArn`, `policyId`, `resourceArn`

### Get

- Operations: `GetIncidentRecord`, `GetReplicationSet`, `GetResourcePolicies`, `GetResponsePlan`, `GetTimelineEvent`
- Traits: `paginated` (1), `readonly` (5)
- Common required input members in this group: `arn`, `eventId`, `incidentRecordArn`, `resourceArn`

### Create

- Operations: `CreateReplicationSet`, `CreateResponsePlan`, `CreateTimelineEvent`
- Traits: `idempotency-token` (3), `idempotent` (2)
- Common required input members in this group: `eventData`, `eventTime`, `eventType`, `incidentRecordArn`, `incidentTemplate`, `name`, `regions`

### Batch

- Operations: `BatchGetIncidentFindings`
- Traits: `readonly` (1)
- Common required input members in this group: `findingIds`, `incidentRecordArn`

### Put

- Operations: `PutResourcePolicy`
- Common required input members in this group: `policy`, `resourceArn`

### Start

- Operations: `StartIncident`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `responsePlanArn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchGetIncidentFindings` | `POST /batchGetIncidentFindings` | `readonly` | `findingIds`, `incidentRecordArn` | - | `BatchGetIncidentFindingsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves details about all specified findings for an incident, including descriptive details about each finding. A finding represents a recent application environment change made by an CodeDeploy deployment or an CloudFormation stack creation or update that... |
| `CreateReplicationSet` | `POST /createReplicationSet` | `idempotency-token` | `regions` | `clientToken` | `CreateReplicationSetOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | A replication set replicates and encrypts your data to the provided Regions with the provided KMS key. |
| `CreateResponsePlan` | `POST /createResponsePlan` | `idempotent`, `idempotency-token` | `incidentTemplate`, `name` | `clientToken` | `CreateResponsePlanOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a response plan that automates the initial response to incidents. A response plan engages contacts, starts chat channel collaboration, and initiates runbooks at the beginning of an incident. |
| `CreateTimelineEvent` | `POST /createTimelineEvent` | `idempotent`, `idempotency-token` | `eventData`, `eventTime`, `eventType`, `incidentRecordArn` | `clientToken` | `CreateTimelineEventOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a custom timeline event on the incident details page of an incident record. Incident Manager automatically creates timeline events that mark key moments during an incident. |
| `DeleteIncidentRecord` | `POST /deleteIncidentRecord` | `idempotent` | `arn` | - | `DeleteIncidentRecordOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Delete an incident record from Incident Manager. |
| `DeleteReplicationSet` | `POST /deleteReplicationSet` | - | `arn` | - | `DeleteReplicationSetOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes all Regions in your replication set. Deleting the replication set deletes all Incident Manager data. |
| `DeleteResourcePolicy` | `POST /deleteResourcePolicy` | - | `policyId`, `resourceArn` | - | `DeleteResourcePolicyOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the resource policy that Resource Access Manager uses to share your Incident Manager resource. |
| `DeleteResponsePlan` | `POST /deleteResponsePlan` | `idempotent` | `arn` | - | `DeleteResponsePlanOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes the specified response plan. Deleting a response plan stops all linked CloudWatch alarms and EventBridge events from creating an incident with this response plan. |
| `DeleteTimelineEvent` | `POST /deleteTimelineEvent` | `idempotent` | `eventId`, `incidentRecordArn` | - | `DeleteTimelineEventOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a timeline event from an incident. |
| `GetIncidentRecord` | `GET /getIncidentRecord` | `readonly` | `arn` | - | `GetIncidentRecordOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the details for the specified incident record. |
| `GetReplicationSet` | `GET /getReplicationSet` | `readonly` | `arn` | - | `GetReplicationSetOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieve your Incident Manager replication set. |
| `GetResourcePolicies` | `POST /getResourcePolicies` | `readonly`, `paginated` | `resourceArn` | - | `GetResourcePoliciesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the resource policies attached to the specified response plan. |
| `GetResponsePlan` | `GET /getResponsePlan` | `readonly` | `arn` | - | `GetResponsePlanOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the details of the specified response plan. |
| `GetTimelineEvent` | `GET /getTimelineEvent` | `readonly` | `eventId`, `incidentRecordArn` | - | `GetTimelineEventOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a timeline event based on its ID and incident record. |
| `ListIncidentFindings` | `POST /listIncidentFindings` | `readonly`, `paginated` | `incidentRecordArn` | - | `ListIncidentFindingsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of the IDs of findings, plus their last modified times, that have been identified for a specified incident. A finding represents a recent application environment change made by an CloudFormation stack creation or update or an CodeDeploy... |
| `ListIncidentRecords` | `POST /listIncidentRecords` | `readonly`, `paginated` | - | - | `ListIncidentRecordsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all incident records in your account. Use this command to retrieve the Amazon Resource Name (ARN) of the incident record you want to update. |
| `ListRelatedItems` | `POST /listRelatedItems` | `readonly`, `paginated` | `incidentRecordArn` | - | `ListRelatedItemsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List all related items for an incident record. |
| `ListReplicationSets` | `POST /listReplicationSets` | `readonly`, `paginated` | - | - | `ListReplicationSetsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists details about the replication set configured in your account. |
| `ListResponsePlans` | `POST /listResponsePlans` | `readonly`, `paginated` | - | - | `ListResponsePlansOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all response plans in your account. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the tags that are attached to the specified response plan or incident. |
| `ListTimelineEvents` | `POST /listTimelineEvents` | `readonly`, `paginated` | `incidentRecordArn` | - | `ListTimelineEventsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists timeline events for the specified incident record. |
| `PutResourcePolicy` | `POST /putResourcePolicy` | - | `policy`, `resourceArn` | - | `PutResourcePolicyOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds a resource policy to the specified response plan. The resource policy is used to share the response plan using Resource Access Manager (RAM). |
| `StartIncident` | `POST /startIncident` | `idempotent`, `idempotency-token` | `responsePlanArn` | `clientToken` | `StartIncidentOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Used to start an incident from CloudWatch alarms, EventBridge events, or manually. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Adds a tag to a response plan. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a tag from a resource. |
| `UpdateDeletionProtection` | `POST /updateDeletionProtection` | `idempotency-token` | `arn`, `deletionProtected` | `clientToken` | `UpdateDeletionProtectionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update deletion protection to either allow or deny deletion of the final Region in a replication set. |
| `UpdateIncidentRecord` | `POST /updateIncidentRecord` | `idempotent`, `idempotency-token` | `arn` | `clientToken` | `UpdateIncidentRecordOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update the details of an incident record. You can use this operation to update an incident record from the defined chat channel. |
| `UpdateRelatedItems` | `POST /updateRelatedItems` | `idempotent`, `idempotency-token` | `incidentRecordArn`, `relatedItemsUpdate` | `clientToken` | `UpdateRelatedItemsOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Add or remove related items from the related items tab of an incident record. |
| `UpdateReplicationSet` | `POST /updateReplicationSet` | `idempotency-token` | `actions`, `arn` | `clientToken` | `UpdateReplicationSetOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Add or delete Regions from your replication set. |
| `UpdateResponsePlan` | `POST /updateResponsePlan` | `idempotent`, `idempotency-token` | `arn` | `clientToken` | `UpdateResponsePlanOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the specified response plan. |
| `UpdateTimelineEvent` | `POST /updateTimelineEvent` | `idempotent`, `idempotency-token` | `eventId`, `incidentRecordArn` | `clientToken` | `UpdateTimelineEventOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a timeline event. You can update events of type `Custom Event`. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DeleteReplicationSet` | - | `arn -> arn` | - | - |
| `GetIncidentRecord` | - | `arn -> arn` | - | - |
| `GetReplicationSet` | - | `arn -> arn` | - | - |
| `GetResourcePolicies` | - | `resourceArn -> resourceArn` | - | - |
| `GetResponsePlan` | - | `arn -> arn` | - | - |
| `GetTimelineEvent` | - | `incidentRecordArn -> incidentRecordArn`, `eventId -> eventId` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You don't have sufficient access to perform this operation. |
| `InternalServerException` | `structure` | `message` | The request processing has failed because of an unknown error, exception or failure. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `serviceCode` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `message` | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceIdentifier`, `resourceType` | Request references a resource which doesn't exist. |
| `ConflictException` | `structure` | `message`, `resourceIdentifier`, `resourceType`, `retryAfter` | Updating or deleting a resource causes an inconsistent state. |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceIdentifier`, `resourceType`, `serviceCode` | Request would cause a service quota to be exceeded. |
| `BatchGetIncidentFindingsInput` | `structure` | `findingIds`, `incidentRecordArn` | - |
| `BatchGetIncidentFindingsOutput` | `structure` | `errors`, `findings` | - |
| `CreateReplicationSetInput` | `structure` | `clientToken`, `regions`, `tags` | - |
| `CreateReplicationSetOutput` | `structure` | `arn` | - |
| `CreateResponsePlanInput` | `structure` | `actions`, `chatChannel`, `clientToken`, `displayName`, `engagements`, `incidentTemplate`, `integrations`, `name`, `tags` | - |
| `CreateResponsePlanOutput` | `structure` | `arn` | - |
| `CreateTimelineEventInput` | `structure` | `clientToken`, `eventData`, `eventReferences`, `eventTime`, `eventType`, `incidentRecordArn` | - |
| `CreateTimelineEventOutput` | `structure` | `eventId`, `incidentRecordArn` | - |
| `DeleteIncidentRecordInput` | `structure` | `arn` | - |
| `DeleteIncidentRecordOutput` | `structure` | - | - |
| `DeleteReplicationSetInput` | `structure` | `arn` | - |
| `DeleteReplicationSetOutput` | `structure` | - | - |
| `DeleteResourcePolicyInput` | `structure` | `policyId`, `resourceArn` | - |
| `DeleteResourcePolicyOutput` | `structure` | - | - |
| `DeleteResponsePlanInput` | `structure` | `arn` | - |
| `DeleteResponsePlanOutput` | `structure` | - | - |
| `DeleteTimelineEventInput` | `structure` | `eventId`, `incidentRecordArn` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
