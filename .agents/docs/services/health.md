# AWS Health APIs and Notifications

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Health The Health API provides access to the Health information that appears in the Health Dashboard. You can use the API operations to get information about events that might affect your Amazon Web Services services and resources. You must have a Business, Enterprise On-Ramp, or Enterprise Support plan from Amazon Web Services Support to use the Health API. If you call the Health API from an Amazon Web Services account that doesn't have a Business, Enterprise On-Ramp, or Enterprise Support plan, you receive a `SubscriptionRequiredException` error. For API access, you need an access key ID and a secret access key.

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for AWS Health APIs and Notifications by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: represent documented AWS Health APIs and Notifications workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `Disable`, `Enable` operation families, including `DescribeAffectedAccountsForOrganization`, `DescribeAffectedEntities`, `DescribeAffectedEntitiesForOrganization`, `DescribeEntityAggregates`, `DisableHealthServiceAccessForOrganization`, `EnableHealthServiceAccessForOrganization`.

## Service Identity and Protocol

- AWS model slug: `health`
- AWS SDK for Rust slug: `health`
- Model version: `2016-08-04`
- Model file: `vendor/api-models-aws/models/health/service/2016-08-04/health-2016-08-04.json`
- SDK ID: `Health`
- Endpoint prefix: `health`
- ARN namespace: `health`
- CloudFormation name: `Health`
- CloudTrail event source: `health.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (12), `Disable` (1), `Enable` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `DisableHealthServiceAccessForOrganization`, `EnableHealthServiceAccessForOrganization`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAffectedAccountsForOrganization`, `DescribeAffectedEntities`, `DescribeAffectedEntitiesForOrganization`, `DescribeEntityAggregates`, `DescribeEntityAggregatesForOrganization`, `DescribeEventAggregates`, `DescribeEventDetails`, `DescribeEventDetailsForOrganization`, `DescribeEventTypes`, `DescribeEvents`, `DescribeEventsForOrganization`, `DescribeHealthServiceStatusForOrganization`.
- Pagination is modelled for 7 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 14 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- 11 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/health/latest/ug/what-is-aws-health.html
- https://docs.aws.amazon.com/health/latest/ug/about-public-events.html
- https://docs.aws.amazon.com/health/latest/ug/view-organizational-view-events.html

Research outcomes:
- AWS Health provides account-specific and public events about service health, resource performance, scheduled changes, and operational issues.
- Events include event type, category, Region, service, start/end/last-updated times, status, and affected entities where applicable.
- EventBridge rules can monitor AWS Health events. Event scope distinguishes account-specific and public Regional service events.
- Public events can be duplicated across Regions, so consumers should deduplicate by event ARN where appropriate.
- Organisational view aggregates AWS Health events across accounts in an AWS Organization into a central feed.
- Organisational view supports delegated administrator access and affected-account/resource filtering.
- Some AWS Health API features require Business, Enterprise On-Ramp, or Enterprise Support plans.

Parity implications:
- Model events, event types, affected entities, event details, public/account-specific scope, organisation aggregation, delegated administrator state, and EventBridge delivery separately.
- API access should account for support-plan restrictions where relevant.
- Organisation view should aggregate member-account events without erasing source account identity.

## Operation Groups

### Describe

- Operations: `DescribeAffectedAccountsForOrganization`, `DescribeAffectedEntities`, `DescribeAffectedEntitiesForOrganization`, `DescribeEntityAggregates`, `DescribeEntityAggregatesForOrganization`, `DescribeEventAggregates`, `DescribeEventDetails`, `DescribeEventDetailsForOrganization`, `DescribeEventTypes`, `DescribeEvents`, `DescribeEventsForOrganization`, `DescribeHealthServiceStatusForOrganization`
- Traits: `idempotent` (12), `paginated` (7)
- Common required input members in this group: `aggregateField`, `eventArn`, `eventArns`, `filter`, `organizationEventDetailFilters`

### Disable

- Operations: `DisableHealthServiceAccessForOrganization`
- Traits: `idempotent` (1)

### Enable

- Operations: `EnableHealthServiceAccessForOrganization`
- Traits: `idempotent` (1)

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DescribeAffectedAccountsForOrganization` | - | `idempotent`, `paginated` | `eventArn` | - | `DescribeAffectedAccountsForOrganizationResponse` | `InvalidPaginationToken` | Returns a list of accounts in the organization from Organizations that are affected by the provided event. For more information about the different types of Health events, see Event. |
| `DescribeAffectedEntities` | - | `idempotent`, `paginated` | `filter` | - | `DescribeAffectedEntitiesResponse` | `InvalidPaginationToken`, `UnsupportedLocale` | Returns a list of entities that have been affected by the specified events, based on the specified filter criteria. Entities can refer to individual customer resources, groups of customer resources, or any other construct, depending on the Amazon Web Services... |
| `DescribeAffectedEntitiesForOrganization` | - | `idempotent`, `paginated` | - | - | `DescribeAffectedEntitiesForOrganizationResponse` | `InvalidPaginationToken`, `UnsupportedLocale` | Returns a list of entities that have been affected by one or more events for one or more accounts in your organization in Organizations, based on the filter criteria. Entities can refer to individual customer resources, groups of customer resources, or any... |
| `DescribeEntityAggregates` | - | `idempotent` | - | - | `DescribeEntityAggregatesResponse` | - | Returns the number of entities that are affected by each of the specified events. |
| `DescribeEntityAggregatesForOrganization` | - | `idempotent` | `eventArns` | - | `DescribeEntityAggregatesForOrganizationResponse` | - | Returns a list of entity aggregates for your Organizations that are affected by each of the specified events. |
| `DescribeEventAggregates` | - | `idempotent`, `paginated` | `aggregateField` | - | `DescribeEventAggregatesResponse` | `InvalidPaginationToken` | Returns the number of events of each event type (issue, scheduled change, and account notification). If no filter is specified, the counts of all events in each category are returned. |
| `DescribeEventDetails` | - | `idempotent` | `eventArns` | - | `DescribeEventDetailsResponse` | `UnsupportedLocale` | Returns detailed information about one or more specified events. Information includes standard event data (Amazon Web Services Region, service, and so on, as returned by DescribeEvents), a detailed event description, and possible additional metadata that... |
| `DescribeEventDetailsForOrganization` | - | `idempotent` | `organizationEventDetailFilters` | - | `DescribeEventDetailsForOrganizationResponse` | `UnsupportedLocale` | Returns detailed information about one or more specified events for one or more Amazon Web Services accounts in your organization. This information includes standard event data (such as the Amazon Web Services Region and service), an event description, and... |
| `DescribeEventTypes` | - | `idempotent`, `paginated` | - | - | `DescribeEventTypesResponse` | `InvalidPaginationToken`, `UnsupportedLocale` | Returns the event types that meet the specified filter criteria. You can use this API operation to find information about the Health event, such as the category, Amazon Web Services service, and event code. |
| `DescribeEvents` | - | `idempotent`, `paginated` | - | - | `DescribeEventsResponse` | `InvalidPaginationToken`, `UnsupportedLocale` | Returns information about events that meet the specified filter criteria. Events are returned in a summary form and do not include the detailed description, any additional metadata that depends on the event type, or any affected resources. |
| `DescribeEventsForOrganization` | - | `idempotent`, `paginated` | - | - | `DescribeEventsForOrganizationResponse` | `InvalidPaginationToken`, `UnsupportedLocale` | Returns information about events across your organization in Organizations. You can use the`filters` parameter to specify the events that you want to return. |
| `DescribeHealthServiceStatusForOrganization` | - | `idempotent` | - | - | `DescribeHealthServiceStatusForOrganizationResponse` | - | This operation provides status information on enabling or disabling Health to work with your organization. To call this operation, you must use the organization's management account. |
| `DisableHealthServiceAccessForOrganization` | - | `idempotent` | - | - | `Unit` | `ConcurrentModificationException` | Disables Health from working with Organizations. To call this operation, you must sign in to the organization's management account. |
| `EnableHealthServiceAccessForOrganization` | - | `idempotent` | - | - | `Unit` | `ConcurrentModificationException` | Enables Health to work with Organizations. You can use the organizational view feature to aggregate events from all Amazon Web Services accounts in your organization in a centralized location. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InvalidPaginationToken` | `structure` | `message` | The specified pagination token (`nextToken`) is not valid. |
| `UnsupportedLocale` | `structure` | `message` | The specified locale is not supported. |
| `ConcurrentModificationException` | `structure` | `message` | EnableHealthServiceAccessForOrganization is already in progress. |
| `DescribeAffectedAccountsForOrganizationRequest` | `structure` | `eventArn`, `maxResults`, `nextToken` | - |
| `DescribeAffectedAccountsForOrganizationResponse` | `structure` | `affectedAccounts`, `eventScopeCode`, `nextToken` | - |
| `DescribeAffectedEntitiesRequest` | `structure` | `filter`, `locale`, `maxResults`, `nextToken` | - |
| `DescribeAffectedEntitiesResponse` | `structure` | `entities`, `nextToken` | - |
| `DescribeAffectedEntitiesForOrganizationRequest` | `structure` | `locale`, `maxResults`, `nextToken`, `organizationEntityAccountFilters`, `organizationEntityFilters` | - |
| `DescribeAffectedEntitiesForOrganizationResponse` | `structure` | `entities`, `failedSet`, `nextToken` | - |
| `DescribeEntityAggregatesRequest` | `structure` | `eventArns` | - |
| `DescribeEntityAggregatesResponse` | `structure` | `entityAggregates` | - |
| `DescribeEntityAggregatesForOrganizationRequest` | `structure` | `awsAccountIds`, `eventArns` | - |
| `DescribeEntityAggregatesForOrganizationResponse` | `structure` | `organizationEntityAggregates` | - |
| `DescribeEventAggregatesRequest` | `structure` | `aggregateField`, `filter`, `maxResults`, `nextToken` | - |
| `DescribeEventAggregatesResponse` | `structure` | `eventAggregates`, `nextToken` | - |
| `DescribeEventDetailsRequest` | `structure` | `eventArns`, `locale` | - |
| `DescribeEventDetailsResponse` | `structure` | `failedSet`, `successfulSet` | - |
| `DescribeEventDetailsForOrganizationRequest` | `structure` | `locale`, `organizationEventDetailFilters` | - |
| `DescribeEventDetailsForOrganizationResponse` | `structure` | `failedSet`, `successfulSet` | - |
| `DescribeEventTypesRequest` | `structure` | `filter`, `locale`, `maxResults`, `nextToken` | - |
| `DescribeEventTypesResponse` | `structure` | `eventTypes`, `nextToken` | - |
| `DescribeEventsRequest` | `structure` | `filter`, `locale`, `maxResults`, `nextToken` | - |
| `DescribeEventsResponse` | `structure` | `events`, `nextToken` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
