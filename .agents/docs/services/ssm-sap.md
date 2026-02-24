# AWS Systems Manager for SAP

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This API reference provides descriptions, syntax, and other details about each of the actions and data types for AWS Systems Manager for SAP. The topic for each action shows the API request parameters and responses.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS Systems Manager for SAP resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Systems Manager for SAP workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Start`, `Delete`, `Deregister` operation families, including `ListApplications`, `ListComponents`, `ListConfigurationCheckDefinitions`, `ListConfigurationCheckOperations`, `GetApplication`, `GetComponent`.

## Service Identity and Protocol

- AWS model slug: `ssm-sap`
- AWS SDK for Rust slug: `ssmsap`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/ssm-sap/service/2018-05-10/ssm-sap-2018-05-10.json`
- SDK ID: `Ssm Sap`
- Endpoint prefix: `ssm-sap`
- ARN namespace: `ssm-sap`
- CloudFormation name: `-`
- CloudTrail event source: `ssm-sap.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (10), `Get` (6), `Start` (3), `Delete` (1), `Deregister` (1), `Put` (1), `Register` (1), `Stop` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `DeleteResourcePermission`, `DeregisterApplication`, `PutResourcePermission`, `RegisterApplication`, `StartApplication`, `StartApplicationRefresh`, `StartConfigurationChecks`, `StopApplication`, `TagResource`, `UntagResource`, `UpdateApplicationSettings`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetApplication`, `GetComponent`, `GetConfigurationCheckOperation`, `GetDatabase`, `GetOperation`, `GetResourcePermission`, `ListApplications`, `ListComponents`, `ListConfigurationCheckDefinitions`, `ListConfigurationCheckOperations`, `ListDatabases`, `ListOperationEvents`, `ListOperations`, `ListSubCheckResults`, `ListSubCheckRuleResults`, `ListTagsForResource`.
- Pagination is modelled for 9 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartApplication`, `StartApplicationRefresh`, `StartConfigurationChecks`, `StopApplication`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 27 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `SNS`, `EC2/VPC`.

## Operation Groups

### List

- Operations: `ListApplications`, `ListComponents`, `ListConfigurationCheckDefinitions`, `ListConfigurationCheckOperations`, `ListDatabases`, `ListOperationEvents`, `ListOperations`, `ListSubCheckResults`, `ListSubCheckRuleResults`, `ListTagsForResource`
- Traits: `paginated` (9), `readonly` (10)
- Common required input members in this group: `ApplicationId`, `OperationId`, `SubCheckResultId`, `resourceArn`

### Get

- Operations: `GetApplication`, `GetComponent`, `GetConfigurationCheckOperation`, `GetDatabase`, `GetOperation`, `GetResourcePermission`
- Traits: `readonly` (5)
- Common required input members in this group: `ApplicationId`, `ComponentId`, `OperationId`, `ResourceArn`

### Start

- Operations: `StartApplication`, `StartApplicationRefresh`, `StartConfigurationChecks`
- Common required input members in this group: `ApplicationId`

### Delete

- Operations: `DeleteResourcePermission`
- Common required input members in this group: `ResourceArn`

### Deregister

- Operations: `DeregisterApplication`
- Common required input members in this group: `ApplicationId`

### Put

- Operations: `PutResourcePermission`
- Common required input members in this group: `ActionType`, `ResourceArn`, `SourceResourceArn`

### Register

- Operations: `RegisterApplication`
- Common required input members in this group: `ApplicationId`, `ApplicationType`, `Instances`

### Stop

- Operations: `StopApplication`
- Common required input members in this group: `ApplicationId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

### Update

- Operations: `UpdateApplicationSettings`
- Common required input members in this group: `ApplicationId`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DeleteResourcePermission` | `POST /delete-resource-permission` | - | `ResourceArn` | - | `DeleteResourcePermissionOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes permissions associated with the target database. |
| `DeregisterApplication` | `POST /deregister-application` | - | `ApplicationId` | - | `DeregisterApplicationOutput` | `InternalServerException`, `UnauthorizedException`, `ValidationException` | Deregister an SAP application with AWS Systems Manager for SAP. This action does not aﬀect the existing setup of your SAP workloads on Amazon EC2. |
| `GetApplication` | `POST /get-application` | `readonly` | - | - | `GetApplicationOutput` | `InternalServerException`, `ValidationException` | Gets an application registered with AWS Systems Manager for SAP. It also returns the components of the application. |
| `GetComponent` | `POST /get-component` | `readonly` | `ApplicationId`, `ComponentId` | - | `GetComponentOutput` | `InternalServerException`, `UnauthorizedException`, `ValidationException` | Gets the component of an application registered with AWS Systems Manager for SAP. |
| `GetConfigurationCheckOperation` | `POST /get-configuration-check-operation` | `readonly` | `OperationId` | - | `GetConfigurationCheckOperationOutput` | `InternalServerException`, `ValidationException` | Gets the details of a configuration check operation by specifying the operation ID. |
| `GetDatabase` | `POST /get-database` | `readonly` | - | - | `GetDatabaseOutput` | `InternalServerException`, `ValidationException` | Gets the SAP HANA database of an application registered with AWS Systems Manager for SAP. |
| `GetOperation` | `POST /get-operation` | `readonly` | `OperationId` | - | `GetOperationOutput` | `InternalServerException`, `ValidationException` | Gets the details of an operation by specifying the operation ID. |
| `GetResourcePermission` | `POST /get-resource-permission` | - | `ResourceArn` | - | `GetResourcePermissionOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets permissions associated with the target database. |
| `ListApplications` | `POST /list-applications` | `readonly`, `paginated` | - | - | `ListApplicationsOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists all the applications registered with AWS Systems Manager for SAP. |
| `ListComponents` | `POST /list-components` | `readonly`, `paginated` | - | - | `ListComponentsOutput` | `InternalServerException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Lists all the components registered with AWS Systems Manager for SAP. |
| `ListConfigurationCheckDefinitions` | `POST /list-configuration-check-definitions` | `readonly`, `paginated` | - | - | `ListConfigurationCheckDefinitionsOutput` | `InternalServerException`, `ValidationException` | Lists all configuration check types supported by AWS Systems Manager for SAP. |
| `ListConfigurationCheckOperations` | `POST /list-configuration-check-operations` | `readonly`, `paginated` | `ApplicationId` | - | `ListConfigurationCheckOperationsOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the configuration check operations performed by AWS Systems Manager for SAP. |
| `ListDatabases` | `POST /list-databases` | `readonly`, `paginated` | - | - | `ListDatabasesOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the SAP HANA databases of an application registered with AWS Systems Manager for SAP. |
| `ListOperationEvents` | `POST /list-operation-events` | `readonly`, `paginated` | `OperationId` | - | `ListOperationEventsOutput` | `InternalServerException`, `ValidationException` | Returns a list of operations events. Available parameters include `OperationID`, as well as optional parameters `MaxResults`, `NextToken`, and `Filters`. |
| `ListOperations` | `POST /list-operations` | `readonly`, `paginated` | `ApplicationId` | - | `ListOperationsOutput` | `InternalServerException`, `ValidationException` | Lists the operations performed by AWS Systems Manager for SAP. |
| `ListSubCheckResults` | `POST /list-sub-check-results` | `readonly`, `paginated` | `OperationId` | - | `ListSubCheckResultsOutput` | `InternalServerException`, `ValidationException` | Lists the sub-check results of a specified configuration check operation. |
| `ListSubCheckRuleResults` | `POST /list-sub-check-rule-results` | `readonly`, `paginated` | `SubCheckResultId` | - | `ListSubCheckRuleResultsOutput` | `InternalServerException`, `ValidationException` | Lists the rules of a specified sub-check belonging to a configuration check operation. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Lists all tags on an SAP HANA application and/or database registered with AWS Systems Manager for SAP. |
| `PutResourcePermission` | `POST /put-resource-permission` | - | `ActionType`, `ResourceArn`, `SourceResourceArn` | - | `PutResourcePermissionOutput` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Adds permissions to the target database. |
| `RegisterApplication` | `POST /register-application` | - | `ApplicationId`, `ApplicationType`, `Instances` | - | `RegisterApplicationOutput` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Register an SAP application with AWS Systems Manager for SAP. You must meet the following requirements before registering. |
| `StartApplication` | `POST /start-application` | - | `ApplicationId` | - | `StartApplicationOutput` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Request is an operation which starts an application. Parameter `ApplicationId` is required. |
| `StartApplicationRefresh` | `POST /start-application-refresh` | - | `ApplicationId` | - | `StartApplicationRefreshOutput` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Refreshes a registered application. |
| `StartConfigurationChecks` | `POST /start-configuration-checks` | - | `ApplicationId` | - | `StartConfigurationChecksOutput` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Initiates configuration check operations against a specified application. |
| `StopApplication` | `POST /stop-application` | - | `ApplicationId` | - | `StopApplicationOutput` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Request is an operation to stop an application. Parameter `ApplicationId` is required. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Creates tag for a resource by specifying the ARN. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Delete the tags for a resource. |
| `UpdateApplicationSettings` | `POST /update-application-settings` | - | `ApplicationId` | - | `UpdateApplicationSettingsOutput` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Updates the settings of an application registered with AWS Systems Manager for SAP. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ValidationException` | `structure` | `Message` | The input fails to satisfy the constraints specified by an AWS service. |
| `InternalServerException` | `structure` | `Message` | An internal error has occurred. |
| `ResourceNotFoundException` | `structure` | `Message` | The resource is not available. |
| `ConflictException` | `structure` | `Message` | A conflict has occurred. |
| `UnauthorizedException` | `structure` | `Message` | The request is not authorized. |
| `DeleteResourcePermissionInput` | `structure` | `ActionType`, `ResourceArn`, `SourceResourceArn` | - |
| `DeleteResourcePermissionOutput` | `structure` | `Policy` | - |
| `DeregisterApplicationInput` | `structure` | `ApplicationId` | - |
| `DeregisterApplicationOutput` | `structure` | - | - |
| `GetApplicationInput` | `structure` | `AppRegistryArn`, `ApplicationArn`, `ApplicationId` | - |
| `GetApplicationOutput` | `structure` | `Application`, `Tags` | - |
| `GetComponentInput` | `structure` | `ApplicationId`, `ComponentId` | - |
| `GetComponentOutput` | `structure` | `Component`, `Tags` | - |
| `GetConfigurationCheckOperationInput` | `structure` | `OperationId` | - |
| `GetConfigurationCheckOperationOutput` | `structure` | `ConfigurationCheckOperation` | - |
| `GetDatabaseInput` | `structure` | `ApplicationId`, `ComponentId`, `DatabaseArn`, `DatabaseId` | - |
| `GetDatabaseOutput` | `structure` | `Database`, `Tags` | - |
| `GetOperationInput` | `structure` | `OperationId` | - |
| `GetOperationOutput` | `structure` | `Operation` | - |
| `GetResourcePermissionInput` | `structure` | `ActionType`, `ResourceArn` | - |
| `GetResourcePermissionOutput` | `structure` | `Policy` | - |
| `ListApplicationsInput` | `structure` | `Filters`, `MaxResults`, `NextToken` | - |
| `ListApplicationsOutput` | `structure` | `Applications`, `NextToken` | - |
| `ListComponentsInput` | `structure` | `ApplicationId`, `MaxResults`, `NextToken` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
