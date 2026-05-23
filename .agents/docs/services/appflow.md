# Amazon Appflow

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Welcome to the Amazon AppFlow API reference. This guide is for developers who need detailed information about the Amazon AppFlow API operations, data types, and errors. Amazon AppFlow is a fully managed integration service that enables you to securely transfer data between software as a service (SaaS) applications like Salesforce, Marketo, Slack, and ServiceNow, and Amazon Web Services like Amazon S3 and Amazon Redshift. Use the following links to get started on the Amazon AppFlow API: Actions: An alphabetical list of all Amazon AppFlow API operations. Data types: An alphabetical list of all Amazon AppFlow data types.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-appflow/tests/scenario_test.rs`: create an S3-to-S3 batch flow, update its definition, start and stop execution, tag it, and delete it.
- Scenario insight from EC2: add full state-machine walks for Amazon Appflow resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent application integration flows between SaaS and AWS sources/destinations, connector profiles, scheduled or on-demand execution, task mappings, error handling, and flow tagging.

## Service Identity and Protocol

- AWS model slug: `appflow`
- AWS SDK for Rust slug: `appflow`
- Model version: `2020-08-23`
- Model file: `vendor/api-models-aws/models/appflow/service/2020-08-23/appflow-2020-08-23.json`
- SDK ID: `Appflow`
- Endpoint prefix: `appflow`
- ARN namespace: `appflow`
- CloudFormation name: `Appflow`
- CloudTrail event source: `appflow.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (6), `List` (4), `Update` (3), `Create` (2), `Delete` (2), `Cancel` (1), `Register` (1), `Reset` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelFlowExecutions`, `CreateConnectorProfile`, `CreateFlow`, `DeleteConnectorProfile`, `DeleteFlow`, `RegisterConnector`, `StartFlow`, `StopFlow`, `TagResource`, `UntagResource`, `UpdateConnectorProfile`, `UpdateConnectorRegistration`, `UpdateFlow`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeConnector`, `DescribeConnectorEntity`, `DescribeConnectorProfiles`, `DescribeConnectors`, `DescribeFlow`, `DescribeFlowExecutionRecords`, `ListConnectorEntities`, `ListConnectors`, `ListFlows`, `ListTagsForResource`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 7 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelFlowExecutions`, `DescribeFlowExecutionRecords`, `StartFlow`, `StopFlow`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 25 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `EventBridge`, `Lambda`, `Glue`, `EC2/VPC`, `ECS`, `Redshift`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/appflow/latest/userguide/flows.html
- https://docs.aws.amazon.com/appflow/latest/userguide/flows-manage.html
- https://docs.aws.amazon.com/appflow/latest/userguide/flow-tutorial-s3-salesforce.html

Research outcomes:
- An AppFlow flow defines a data transfer between a source connector and a destination connector.
- Flows configure source and destination properties, trigger type, field mappings, filters, validation, aggregation, partitioning, and error handling.
- Connector profiles store connection/authentication configuration for SaaS and AWS service connectors.
- Trigger types include on-demand, scheduled, and event-triggered flows where supported by the connector.
- Flow runs produce execution records and transfer statistics that can be viewed in flow history.
- Flows can be activated, deactivated, copied, updated, deleted, and run manually where trigger configuration allows.
- Running flows can be cancelled. Updates to flow mappings and settings affect subsequent executions.
- Some connectors support extra behaviour such as Salesforce Bulk API, upsert operations, private connectivity, and large-event handling through S3.

Parity implications:
- Model connector profiles, flows, trigger configuration, tasks, filters, field mappings, executions, execution records, and per-connector capabilities separately.
- StartFlow should create an execution only for runnable trigger states and should expose asynchronous execution state and statistics.
- Connector-specific settings should drive validation and execution semantics rather than being accepted as opaque blobs.

## Operation Groups

### Describe

- Operations: `DescribeConnector`, `DescribeConnectorEntity`, `DescribeConnectorProfiles`, `DescribeConnectors`, `DescribeFlow`, `DescribeFlowExecutionRecords`
- Traits: `paginated` (3)
- Common required input members in this group: `flowName`

### List

- Operations: `ListConnectorEntities`, `ListConnectors`, `ListFlows`, `ListTagsForResource`
- Traits: `paginated` (2)
- Common required input members in this group: -

### Update

- Operations: `UpdateConnectorProfile`, `UpdateConnectorRegistration`, `UpdateFlow`
- Traits: `idempotency-token` (3)
- Common required input members in this group: -

### Create

- Operations: `CreateConnectorProfile`, `CreateFlow`
- Traits: `idempotency-token` (2)
- Common required input members in this group: -

### Delete

- Operations: `DeleteConnectorProfile`, `DeleteFlow`
- Common required input members in this group: -

### Cancel

- Operations: `CancelFlowExecutions`
- Common required input members in this group: -

### Register

- Operations: `RegisterConnector`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Reset

- Operations: `ResetConnectorMetadataCache`
- Common required input members in this group: -

### Start

- Operations: `StartFlow`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Stop

- Operations: `StopFlow`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Unregister

- Operations: `UnregisterConnector`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelFlowExecutions` | `POST /cancel-flow-executions` | - | `flowName` | - | `CancelFlowExecutionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels active runs for a flow. You can cancel all of the active runs for a flow, or you can cancel specific runs by providing their IDs. You can cancel a flow run only when the run is in progress. You can't cancel a ... |
| `CreateConnectorProfile` | `POST /create-connector-profile` | `idempotency-token` | `connectorProfileName`, `connectorType`, `connectionMode`, `connectorProfileConfig` | `clientToken` | `CreateConnectorProfileResponse` | `ConflictException`, `ConnectorAuthenticationException`, `InternalServerException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a new connector profile associated with your Amazon Web Services account. There is a soft quota of 100 connector profiles per Amazon Web Services account. If you need more connector profiles than this quota a ... |
| `CreateFlow` | `POST /create-flow` | `idempotency-token` | `flowName`, `triggerConfig`, `sourceFlowConfig`, `destinationFlowConfigList`, `tasks` | `clientToken` | `CreateFlowResponse` | `AccessDeniedException`, `ConflictException`, `ConnectorAuthenticationException`, `ConnectorServerException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Enables your application to create a new flow using Amazon AppFlow. You must create a connector profile before calling this API. Please note that the Request Syntax below shows syntax for multiple destinations, howev ... |
| `DeleteConnectorProfile` | `POST /delete-connector-profile` | - | `connectorProfileName` | - | `DeleteConnectorProfileResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException` | Enables you to delete an existing connector profile. |
| `DeleteFlow` | `POST /delete-flow` | - | `flowName` | - | `DeleteFlowResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException` | Enables your application to delete an existing flow. Before deleting the flow, Amazon AppFlow validates the request by checking the flow configuration and status. You can delete flows one at a time. |
| `DescribeConnector` | `POST /describe-connector` | - | `connectorType` | - | `DescribeConnectorResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describes the given custom connector registered in your Amazon Web Services account. This API can be used for custom connectors that are registered in your account and also for Amazon authored connectors. |
| `DescribeConnectorEntity` | `POST /describe-connector-entity` | - | `connectorEntityName` | - | `DescribeConnectorEntityResponse` | `ConnectorAuthenticationException`, `ConnectorServerException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Provides details regarding the entity used with the connector, with a description of the data model for each field in that entity. |
| `DescribeConnectorProfiles` | `POST /describe-connector-profiles` | `paginated` | - | - | `DescribeConnectorProfilesResponse` | `InternalServerException`, `ValidationException` | Returns a list of connector-profile details matching the provided connector-profile names and connector-types . Both input lists are optional, and you can use them to filter the result. If no names or connector-types ... |
| `DescribeConnectors` | `POST /describe-connectors` | `paginated` | - | - | `DescribeConnectorsResponse` | `InternalServerException`, `ValidationException` | Describes the connectors vended by Amazon AppFlow for specified connector types. If you don't specify a connector type, this operation describes all connectors vended by Amazon AppFlow. If there are more connectors t ... |
| `DescribeFlow` | `POST /describe-flow` | - | `flowName` | - | `DescribeFlowResponse` | `InternalServerException`, `ResourceNotFoundException` | Provides a description of the specified flow. |
| `DescribeFlowExecutionRecords` | `POST /describe-flow-execution-records` | `paginated` | `flowName` | - | `DescribeFlowExecutionRecordsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Fetches the execution history of the flow. |
| `ListConnectorEntities` | `POST /list-connector-entities` | - | - | - | `ListConnectorEntitiesResponse` | `ConnectorAuthenticationException`, `ConnectorServerException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns the list of available connector entities supported by Amazon AppFlow. For example, you can query Salesforce for Account and Opportunity entities, or query ServiceNow for the Incident entity. |
| `ListConnectors` | `POST /list-connectors` | `paginated` | - | - | `ListConnectorsResponse` | `InternalServerException`, `ValidationException` | Returns the list of all registered custom connectors in your Amazon Web Services account. This API lists only custom connectors registered in this account, not the Amazon Web Services authored connectors. |
| `ListFlows` | `POST /list-flows` | `paginated` | - | - | `ListFlowsResponse` | `InternalServerException`, `ValidationException` | Lists all of the flows associated with your account. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Retrieves the tags that are associated with a specified flow. |
| `RegisterConnector` | `POST /register-connector` | `idempotency-token` | - | `clientToken` | `RegisterConnectorResponse` | `AccessDeniedException`, `ConflictException`, `ConnectorAuthenticationException`, `ConnectorServerException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Registers a new custom connector with your Amazon Web Services account. Before you can register the connector, you must deploy the associated AWS lambda function in your account. |
| `ResetConnectorMetadataCache` | `POST /reset-connector-metadata-cache` | - | - | - | `ResetConnectorMetadataCacheResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Resets metadata about your connector entities that Amazon AppFlow stored in its cache. Use this action when you want Amazon AppFlow to return the latest information about the data that you have in a source applicatio ... |
| `StartFlow` | `POST /start-flow` | `idempotency-token` | `flowName` | `clientToken` | `StartFlowResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException` | Activates an existing flow. For on-demand flows, this operation runs the flow immediately. For schedule and event-triggered flows, this operation activates the flow. |
| `StopFlow` | `POST /stop-flow` | - | `flowName` | - | `StopFlowResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `UnsupportedOperationException` | Deactivates the existing flow. For on-demand flows, this operation returns an unsupportedOperationException error message. For schedule and event-triggered flows, this operation deactivates the flow. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Applies a tag to the specified flow. |
| `UnregisterConnector` | `POST /unregister-connector` | - | `connectorLabel` | - | `UnregisterConnectorResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException` | Unregisters the custom connector registered in your account that matches the connector label provided in the request. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes a tag from the specified flow. |
| `UpdateConnectorProfile` | `POST /update-connector-profile` | `idempotency-token` | `connectorProfileName`, `connectionMode`, `connectorProfileConfig` | `clientToken` | `UpdateConnectorProfileResponse` | `ConflictException`, `ConnectorAuthenticationException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates a given connector profile associated with your account. |
| `UpdateConnectorRegistration` | `POST /update-connector-registration` | `idempotency-token` | `connectorLabel` | `clientToken` | `UpdateConnectorRegistrationResponse` | `AccessDeniedException`, `ConflictException`, `ConnectorAuthenticationException`, `ConnectorServerException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates a custom connector that you've previously registered. This operation updates the connector with one of the following: The latest version of the AWS Lambda function that's assigned to the connector A new AWS L ... |
| `UpdateFlow` | `POST /update-flow` | `idempotency-token` | `flowName`, `triggerConfig`, `sourceFlowConfig`, `destinationFlowConfigList`, `tasks` | `clientToken` | `UpdateFlowResponse` | `AccessDeniedException`, `ConflictException`, `ConnectorAuthenticationException`, `ConnectorServerException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Updates an existing flow. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | AppFlow/Requester has invalid or missing permissions. |
| `ConflictException` | `structure` | message | There was a conflict when processing the request (for example, a flow with the given name already exists within the account. Check for conflicting resource ... |
| `ConnectorAuthenticationException` | `structure` | message | An error occurred when authenticating with the connector endpoint. |
| `ConnectorServerException` | `structure` | message | An error occurred when retrieving data from the connector endpoint. |
| `InternalServerException` | `structure` | message | An internal service error occurred during the processing of your request. Try again later. |
| `ResourceNotFoundException` | `structure` | message | The resource specified in the request (such as the source or destination connector profile) is not found. |
| `ServiceQuotaExceededException` | `structure` | message | The request would cause a service quota (such as the number of flows) to be exceeded. |
| `ThrottlingException` | `structure` | message | API calls have exceeded the maximum allowed API request rate per account and per Region. |
| `UnsupportedOperationException` | `structure` | message | The requested operation is not supported for the current flow. |
| `ValidationException` | `structure` | message | The request has invalid or missing parameters. |
| `CancelFlowExecutionsRequest` | `structure` | flowName, executionIds | - |
| `CancelFlowExecutionsResponse` | `structure` | invalidExecutions | - |
| `CreateConnectorProfileRequest` | `structure` | connectorProfileName, kmsArn, connectorType, connectorLabel, connectionMode, connectorProfileConfig, clientToken | - |
| `CreateConnectorProfileResponse` | `structure` | connectorProfileArn | - |
| `CreateFlowRequest` | `structure` | flowName, description, kmsArn, triggerConfig, sourceFlowConfig, destinationFlowConfigList, tasks, tags, metadataCatalogConfig, clientToken | - |
| `CreateFlowResponse` | `structure` | flowArn, flowStatus | - |
| `DeleteConnectorProfileRequest` | `structure` | connectorProfileName, forceDelete | - |
| `DeleteConnectorProfileResponse` | `structure` | **empty (no members)** | - |
| `DeleteFlowRequest` | `structure` | flowName, forceDelete | - |
| `DeleteFlowResponse` | `structure` | **empty (no members)** | - |
| `DescribeConnectorRequest` | `structure` | connectorType, connectorLabel | - |
| `DescribeConnectorResponse` | `structure` | connectorConfiguration | - |
| `DescribeConnectorEntityRequest` | `structure` | connectorEntityName, connectorType, connectorProfileName, apiVersion | - |
| `DescribeConnectorEntityResponse` | `structure` | connectorEntityFields | - |
| `DescribeConnectorProfilesRequest` | `structure` | connectorProfileNames, connectorType, connectorLabel, maxResults, nextToken | - |
| `DescribeConnectorProfilesResponse` | `structure` | connectorProfileDetails, nextToken | - |
| `DescribeConnectorsRequest` | `structure` | connectorTypes, maxResults, nextToken | - |
| `DescribeConnectorsResponse` | `structure` | connectorConfigurations, connectors, nextToken | - |
| `DescribeFlowRequest` | `structure` | flowName | - |
| `DescribeFlowResponse` | `structure` | flowArn, description, flowName, kmsArn, flowStatus, flowStatusMessage, sourceFlowConfig, destinationFlowConfigList, lastRunExecutionDetails, triggerConfig, tasks, createdAt, ... (+7) | - |
| `DescribeFlowExecutionRecordsRequest` | `structure` | flowName, maxResults, nextToken | - |
| `DescribeFlowExecutionRecordsResponse` | `structure` | flowExecutions, nextToken | - |
| `ListConnectorEntitiesRequest` | `structure` | connectorProfileName, connectorType, entitiesPath, apiVersion, maxResults, nextToken | - |
| `ListConnectorEntitiesResponse` | `structure` | connectorEntityMap, nextToken | - |
| `ListConnectorsRequest` | `structure` | maxResults, nextToken | - |
| `ListConnectorsResponse` | `structure` | connectors, nextToken | - |
| `ListFlowsRequest` | `structure` | maxResults, nextToken | - |
| `ListFlowsResponse` | `structure` | flows, nextToken | - |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `AggregationType` | `enum` | NONE, SINGLE_FILE | - |
| `AmplitudeConnectorOperator` | `enum` | BETWEEN | - |
| `AuthenticationType` | `enum` | OAUTH2, APIKEY, BASIC, CUSTOM | - |
| `CatalogType` | `enum` | GLUE | - |
| `ConnectionMode` | `enum` | PUBLIC, PRIVATE | - |
| `ConnectorProvisioningType` | `enum` | LAMBDA | The type of provisioning that the connector supports, such as Lambda. |
| `ConnectorType` | `enum` | SALESFORCE, SINGULAR, SLACK, REDSHIFT, S3, MARKETO, GOOGLEANALYTICS, ZENDESK, SERVICENOW, DATADOG, TRENDMICRO, SNOWFLAKE, ... (+12) | - |
| `DataPullMode` | `enum` | INCREMENTAL, COMPLETE | - |
| `DataTransferApiType` | `enum` | SYNC, ASYNC, AUTOMATIC | - |
| `DatadogConnectorOperator` | `enum` | PROJECTION, BETWEEN, EQUAL_TO, ADDITION, MULTIPLICATION, DIVISION, SUBTRACTION, MASK_ALL, MASK_FIRST_N, MASK_LAST_N, VALIDATE_NON_NULL, VALIDATE_NON_ZERO, ... (+3) | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
