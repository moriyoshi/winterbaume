# AWS CloudFormation

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

CloudFormation CloudFormation allows you to create and manage Amazon Web Services infrastructure deployments predictably and repeatedly. You can use CloudFormation to leverage Amazon Web Services products, such as Amazon Elastic Compute Cloud, Amazon Elastic Block Store, Amazon Simple Notification Service, ELB, and Amazon EC2 Auto Scaling to build highly reliable, highly scalable, cost-effective applications without creating or configuring the underlying Amazon Web Services infrastructure. With CloudFormation, you declare all your resources and dependencies in a template file. The template defines a collection of resources as a single unit called a stack. CloudFormation creates and deletes all member resources of the stack together and manages all dependencies between the resources for you.

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for AWS CloudFormation by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: add full state-machine walks for AWS CloudFormation resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS CloudFormation workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Describe`, `Create`, `Delete`, `Get` operation families, including `ListChangeSets`, `ListExports`, `ListGeneratedTemplates`, `ListHookResults`, `DescribeAccountLimits`, `DescribeChangeSet`.

## Service Identity and Protocol

- AWS model slug: `cloudformation`
- AWS SDK for Rust slug: `cloudformation`
- Model version: `2010-05-15`
- Model file: `vendor/api-models-aws/models/cloudformation/service/2010-05-15/cloudformation-2010-05-15.json`
- SDK ID: `CloudFormation`
- Endpoint prefix: `cloudformation`
- ARN namespace: `cloudformation`
- CloudFormation name: `CloudFormation`
- CloudTrail event source: `cloudformation.amazonaws.com`
- Protocols: `awsQuery`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (21), `Describe` (20), `Create` (6), `Delete` (5), `Get` (5), `Update` (5), `Detect` (3), `Set` (3).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchDescribeTypeConfigurations`, `CancelUpdateStack`, `CreateChangeSet`, `CreateGeneratedTemplate`, `CreateStack`, `CreateStackInstances`, `CreateStackRefactor`, `CreateStackSet`, `DeleteChangeSet`, `DeleteGeneratedTemplate`, `DeleteStack`, `DeleteStackInstances`, `DeleteStackSet`, `DeregisterType`, `ImportStacksToStackSet`, `RegisterPublisher`, `RegisterType`, `SetStackPolicy`, `SetTypeConfiguration`, `SetTypeDefaultVersion`, ... (+7).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccountLimits`, `DescribeChangeSet`, `DescribeChangeSetHooks`, `DescribeEvents`, `DescribeGeneratedTemplate`, `DescribeOrganizationsAccess`, `DescribePublisher`, `DescribeResourceScan`, `DescribeStackDriftDetectionStatus`, `DescribeStackEvents`, `DescribeStackInstance`, `DescribeStackRefactor`, `DescribeStackResource`, `DescribeStackResourceDrifts`, `DescribeStackResources`, `DescribeStackSet`, `DescribeStackSetOperation`, `DescribeStacks`, `DescribeType`, `DescribeTypeRegistration`, ... (+27).
- Pagination is modelled for 24 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 22 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelUpdateStack`, `DescribeResourceScan`, `ImportStacksToStackSet`, `ListExports`, `ListImports`, `ListResourceScanRelatedResources`, `ListResourceScanResources`, `ListResourceScans`, `StartResourceScan`, `StopStackSetOperation`.
- 62 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `Lambda`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stack-failure-options.html
- https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/drift-aware-change-sets.html
- https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/eventbridge-integration.html

Research outcomes:
- CloudFormation identifies resource dependencies and parallelises independent provisioning paths.
- A failure in one independent provisioning path does not automatically stop other independent paths; provisioning continues until completion or another failure.
- Stack failure options can preserve successfully provisioned resources instead of immediately rolling them back.
- After a preserved failure, users can Retry failed resources, Update the template and retry failed resources, or Roll back to the last stable state.
- Paused rollback requires Preserve successfully provisioned resources and a CREATE_FAILED or UPDATE_FAILED stack status.
- Drift-aware change sets use a three-way comparison and can revert out-of-band drift for supported resource types.
- CloudFormation emits stack, drift, StackSet, and Git sync status change events to EventBridge.

Parity implications:
- Stack operations need asynchronous state machines, dependency ordering, per-resource events, and independent path failure semantics.
- Rollback and preserve-successful-resources are observable stack lifecycle behaviours, not simple success/failure flags.
- Change sets and drift detection need separate state from stack templates and current resource state.

## Operation Groups

### List

- Operations: `ListChangeSets`, `ListExports`, `ListGeneratedTemplates`, `ListHookResults`, `ListImports`, `ListResourceScanRelatedResources`, `ListResourceScanResources`, `ListResourceScans`, `ListStackInstanceResourceDrifts`, `ListStackInstances`, `ListStackRefactorActions`, `ListStackRefactors`, `ListStackResources`, `ListStacks`, `ListStackSetAutoDeploymentTargets`, `ListStackSetOperationResults`, `ListStackSetOperations`, `ListStackSets`, `ListTypeRegistrations`, `ListTypes`, `ListTypeVersions`
- Traits: `paginated` (18), `idempotent` (3)
- Common required input members in this group: `StackName`, `ResourceScanId`, `StackSetName`, `OperationId`

### Describe

- Operations: `DescribeAccountLimits`, `DescribeChangeSet`, `DescribeChangeSetHooks`, `DescribeEvents`, `DescribeGeneratedTemplate`, `DescribeOrganizationsAccess`, `DescribePublisher`, `DescribeResourceScan`, `DescribeStackDriftDetectionStatus`, `DescribeStackEvents`, `DescribeStackInstance`, `DescribeStackRefactor`, `DescribeStackResource`, `DescribeStackResourceDrifts`, `DescribeStackResources`, `DescribeStacks`, `DescribeStackSet`, `DescribeStackSetOperation`, `DescribeType`, `DescribeTypeRegistration`
- Traits: `paginated` (6), `idempotent` (3)
- Common required input members in this group: `ChangeSetName`, `StackName`, `StackSetName`

### Create

- Operations: `CreateChangeSet`, `CreateGeneratedTemplate`, `CreateStack`, `CreateStackInstances`, `CreateStackRefactor`, `CreateStackSet`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `StackName`, `StackSetName`

### Delete

- Operations: `DeleteChangeSet`, `DeleteGeneratedTemplate`, `DeleteStack`, `DeleteStackInstances`, `DeleteStackSet`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `StackSetName`

### Get

- Operations: `GetGeneratedTemplate`, `GetHookResult`, `GetStackPolicy`, `GetTemplate`, `GetTemplateSummary`
- Common required input members in this group: -

### Update

- Operations: `UpdateGeneratedTemplate`, `UpdateStack`, `UpdateStackInstances`, `UpdateStackSet`, `UpdateTerminationProtection`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `StackName`, `StackSetName`

### Detect

- Operations: `DetectStackDrift`, `DetectStackResourceDrift`, `DetectStackSetDrift`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `StackName`

### Set

- Operations: `SetStackPolicy`, `SetTypeConfiguration`, `SetTypeDefaultVersion`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Activate

- Operations: `ActivateOrganizationsAccess`, `ActivateType`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Deactivate

- Operations: `DeactivateOrganizationsAccess`, `DeactivateType`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Execute

- Operations: `ExecuteChangeSet`, `ExecuteStackRefactor`
- Common required input members in this group: -

### Register

- Operations: `RegisterPublisher`, `RegisterType`
- Traits: `idempotent` (2)
- Common required input members in this group: -

### Batch

- Operations: `BatchDescribeTypeConfigurations`
- Common required input members in this group: -

### Cancel

- Operations: `CancelUpdateStack`
- Common required input members in this group: -

### Continue

- Operations: `ContinueUpdateRollback`
- Common required input members in this group: -

### Deregister

- Operations: `DeregisterType`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Estimate

- Operations: `EstimateTemplateCost`
- Common required input members in this group: -

### Import

- Operations: `ImportStacksToStackSet`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Publish

- Operations: `PublishType`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Record

- Operations: `RecordHandlerProgress`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Rollback

- Operations: `RollbackStack`
- Common required input members in this group: -

### Signal

- Operations: `SignalResource`
- Common required input members in this group: -

### Start

- Operations: `StartResourceScan`
- Common required input members in this group: -

### Stop

- Operations: `StopStackSetOperation`
- Common required input members in this group: -

### Test

- Operations: `TestType`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Validate

- Operations: `ValidateTemplate`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ActivateOrganizationsAccess` | `-` | - | - | - | `ActivateOrganizationsAccessOutput` | `InvalidOperationException`, `OperationNotFoundException` | Activate trusted access with Organizations. With trusted access between StackSets and Organizations activated, the management account has permissions to create and manage StackSets for your organization. |
| `ActivateType` | `-` | `idempotent` | - | - | `ActivateTypeOutput` | `CFNRegistryException`, `TypeNotFoundException` | Activates a public third-party extension, such as a resource or module, to make it available for use in stack templates in your current account and Region. It can also create CloudFormation Hooks, which allow you to ... |
| `BatchDescribeTypeConfigurations` | `-` | - | `TypeConfigurationIdentifiers` | - | `BatchDescribeTypeConfigurationsOutput` | `CFNRegistryException`, `TypeConfigurationNotFoundException` | Returns configuration data for the specified CloudFormation extensions, from the CloudFormation registry in your current account and Region. For more information, see Edit configuration data for extensions in your ac ... |
| `CancelUpdateStack` | `-` | - | `StackName` | - | `Unit` | `TokenAlreadyExistsException` | Cancels an update on the specified stack. If the call completes successfully, the stack rolls back the update and reverts to the previous stack configuration. You can cancel only stacks that are in the UPDATE_IN_PROG ... |
| `ContinueUpdateRollback` | `-` | - | `StackName` | - | `ContinueUpdateRollbackOutput` | `TokenAlreadyExistsException` | Continues rolling back a stack from UPDATE_ROLLBACK_FAILED to UPDATE_ROLLBACK_COMPLETE state. Depending on the cause of the failure, you can manually fix the error and continue the rollback. By continuing the rollbac ... |
| `CreateChangeSet` | `-` | - | `StackName`, `ChangeSetName` | - | `CreateChangeSetOutput` | `AlreadyExistsException`, `InsufficientCapabilitiesException`, `LimitExceededException` | Creates a list of changes that will be applied to a stack so that you can review the changes before executing them. You can create a change set for a stack that doesn't exist or an existing stack. If you create a cha ... |
| `CreateGeneratedTemplate` | `-` | - | `GeneratedTemplateName` | - | `CreateGeneratedTemplateOutput` | `AlreadyExistsException`, `ConcurrentResourcesLimitExceededException`, `LimitExceededException` | Creates a template from existing resources that are not already managed with CloudFormation. You can check the status of the template generation using the DescribeGeneratedTemplate API action. |
| `CreateStack` | `-` | - | `StackName` | - | `CreateStackOutput` | `AlreadyExistsException`, `InsufficientCapabilitiesException`, `LimitExceededException`, `TokenAlreadyExistsException` | Creates a stack as specified in the template. After the call completes successfully, the stack creation starts. You can check the status of the stack through the DescribeStacks operation. For more information about c ... |
| `CreateStackInstances` | `-` | `idempotency-token` | `StackSetName`, `Regions` | `OperationId` | `CreateStackInstancesOutput` | `InvalidOperationException`, `LimitExceededException`, `OperationIdAlreadyExistsException`, `OperationInProgressException`, `StackSetNotFoundException`, `StaleRequestException` | Creates stack instances for the specified accounts, within the specified Amazon Web Services Regions. A stack instance refers to a stack in a specific account and Region. You must specify at least one value for eithe ... |
| `CreateStackRefactor` | `-` | - | `StackDefinitions` | - | `CreateStackRefactorOutput` | - | Creates a refactor across multiple stacks, with the list of stacks and resources that are affected. |
| `CreateStackSet` | `-` | `idempotency-token` | `StackSetName` | `ClientRequestToken` | `CreateStackSetOutput` | `CreatedButModifiedException`, `LimitExceededException`, `NameAlreadyExistsException` | Creates a StackSet. |
| `DeactivateOrganizationsAccess` | `-` | - | - | - | `DeactivateOrganizationsAccessOutput` | `InvalidOperationException`, `OperationNotFoundException` | Deactivates trusted access with Organizations. If trusted access is deactivated, the management account does not have permissions to create and manage service-managed StackSets for your organization. |
| `DeactivateType` | `-` | `idempotent` | - | - | `DeactivateTypeOutput` | `CFNRegistryException`, `TypeNotFoundException` | Deactivates a public third-party extension, such as a resource or module, or a CloudFormation Hook when you no longer use it. Deactivating an extension deletes the configuration details that are associated with it. T ... |
| `DeleteChangeSet` | `-` | - | `ChangeSetName` | - | `DeleteChangeSetOutput` | `InvalidChangeSetStatusException` | Deletes the specified change set. Deleting change sets ensures that no one executes the wrong change set. If the call successfully completes, CloudFormation successfully deleted the change set. If IncludeNestedStacks ... |
| `DeleteGeneratedTemplate` | `-` | - | `GeneratedTemplateName` | - | `Unit` | `ConcurrentResourcesLimitExceededException`, `GeneratedTemplateNotFoundException` | Deleted a generated template. |
| `DeleteStack` | `-` | - | `StackName` | - | `Unit` | `TokenAlreadyExistsException` | Deletes a specified stack. Once the call completes successfully, stack deletion starts. Deleted stacks don't show up in the DescribeStacks operation if the deletion has been completed successfully. For more informati ... |
| `DeleteStackInstances` | `-` | `idempotency-token` | `StackSetName`, `Regions`, `RetainStacks` | `OperationId` | `DeleteStackInstancesOutput` | `InvalidOperationException`, `OperationIdAlreadyExistsException`, `OperationInProgressException`, `StackSetNotFoundException`, `StaleRequestException` | Deletes stack instances for the specified accounts, in the specified Amazon Web Services Regions. The maximum number of organizational unit (OUs) supported by a DeleteStackInstances operation is 50. If you need more ... |
| `DeleteStackSet` | `-` | - | `StackSetName` | - | `DeleteStackSetOutput` | `OperationInProgressException`, `StackSetNotEmptyException` | Deletes a StackSet. Before you can delete a StackSet, all its member stack instances must be deleted. For more information about how to complete this, see DeleteStackInstances . |
| `DeregisterType` | `-` | `idempotent` | - | - | `DeregisterTypeOutput` | `CFNRegistryException`, `TypeNotFoundException` | Marks an extension or extension version as DEPRECATED in the CloudFormation registry, removing it from active use. Deprecated extensions or extension versions cannot be used in CloudFormation operations. To deregiste ... |
| `DescribeAccountLimits` | `-` | `paginated` | - | - | `DescribeAccountLimitsOutput` | - | Retrieves your account's CloudFormation limits, such as the maximum number of stacks that you can create in your account. For more information about account limits, see Understand CloudFormation quotas in the CloudFo ... |
| `DescribeChangeSet` | `-` | `paginated` | `ChangeSetName` | - | `DescribeChangeSetOutput` | `ChangeSetNotFoundException` | Returns the inputs for the change set and a list of changes that CloudFormation will make if you execute the change set. For more information, see Update CloudFormation stacks using change sets in the CloudFormation ... |
| `DescribeChangeSetHooks` | `-` | - | `ChangeSetName` | - | `DescribeChangeSetHooksOutput` | `ChangeSetNotFoundException` | Returns Hook-related information for the change set and a list of changes that CloudFormation makes when you run the change set. |
| `DescribeEvents` | `-` | `paginated` | - | - | `DescribeEventsOutput` | - | Returns CloudFormation events based on flexible query criteria. Groups events by operation ID, enabling you to focus on individual stack operations during deployment. An operation is any action performed on a stack, ... |
| `DescribeGeneratedTemplate` | `-` | - | `GeneratedTemplateName` | - | `DescribeGeneratedTemplateOutput` | `GeneratedTemplateNotFoundException` | Describes a generated template. The output includes details about the progress of the creation of a generated template started by a CreateGeneratedTemplate API action or the update of a generated template started wit ... |
| `DescribeOrganizationsAccess` | `-` | - | - | - | `DescribeOrganizationsAccessOutput` | `InvalidOperationException`, `OperationNotFoundException` | Retrieves information about the account's OrganizationAccess status. This API can be called either by the management account or the delegated administrator by using the CallAs parameter. This API can also be called w ... |
| `DescribePublisher` | `-` | `idempotent` | - | - | `DescribePublisherOutput` | `CFNRegistryException` | Returns information about a CloudFormation extension publisher. If you don't supply a PublisherId , and you have registered as an extension publisher, DescribePublisher returns information about your own publisher ac ... |
| `DescribeResourceScan` | `-` | - | `ResourceScanId` | - | `DescribeResourceScanOutput` | `ResourceScanNotFoundException` | Describes details of a resource scan. |
| `DescribeStackDriftDetectionStatus` | `-` | - | `StackDriftDetectionId` | - | `DescribeStackDriftDetectionStatusOutput` | - | Returns information about a stack drift detection operation. A stack drift detection operation detects whether a stack's actual configuration differs, or has drifted , from its expected configuration, as defined in t ... |
| `DescribeStackEvents` | `-` | `paginated` | `StackName` | - | `DescribeStackEventsOutput` | - | Returns all stack related events for a specified stack in reverse chronological order. For more information about a stack's event history, see Understand CloudFormation stack creation events in the CloudFormation Use ... |
| `DescribeStackInstance` | `-` | - | `StackSetName`, `StackInstanceAccount`, `StackInstanceRegion` | - | `DescribeStackInstanceOutput` | `StackInstanceNotFoundException`, `StackSetNotFoundException` | Returns the stack instance that's associated with the specified StackSet, Amazon Web Services account, and Amazon Web Services Region. For a list of stack instances that are associated with a specific StackSet, use L ... |
| `DescribeStackRefactor` | `-` | - | `StackRefactorId` | - | `DescribeStackRefactorOutput` | `StackRefactorNotFoundException` | Describes the stack refactor status. |
| `DescribeStackResource` | `-` | - | `StackName`, `LogicalResourceId` | - | `DescribeStackResourceOutput` | - | Returns a description of the specified resource in the specified stack. For deleted stacks, DescribeStackResource returns resource information for up to 90 days after the stack has been deleted. |
| `DescribeStackResourceDrifts` | `-` | `paginated` | `StackName` | - | `DescribeStackResourceDriftsOutput` | - | Returns drift information for the resources that have been checked for drift in the specified stack. This includes actual and expected configuration values for resources where CloudFormation detects configuration dri ... |
| `DescribeStackResources` | `-` | - | - | - | `DescribeStackResourcesOutput` | - | Returns Amazon Web Services resource descriptions for running and deleted stacks. If StackName is specified, all the associated resources that are part of the stack are returned. If PhysicalResourceId is specified, t ... |
| `DescribeStacks` | `-` | `paginated` | - | - | `DescribeStacksOutput` | - | Returns the description for the specified stack; if no stack name was specified, then it returns the description for all the stacks created. For more information about a stack's event history, see Understand CloudFor ... |
| `DescribeStackSet` | `-` | - | `StackSetName` | - | `DescribeStackSetOutput` | `StackSetNotFoundException` | Returns the description of the specified StackSet. This API provides strongly consistent reads meaning it will always return the most up-to-date data. |
| `DescribeStackSetOperation` | `-` | - | `StackSetName`, `OperationId` | - | `DescribeStackSetOperationOutput` | `OperationNotFoundException`, `StackSetNotFoundException` | Returns the description of the specified StackSet operation. This API provides strongly consistent reads meaning it will always return the most up-to-date data. |
| `DescribeType` | `-` | `idempotent` | - | - | `DescribeTypeOutput` | `CFNRegistryException`, `TypeNotFoundException` | Returns detailed information about an extension from the CloudFormation registry in your current account and Region. If you specify a VersionId , DescribeType returns information about that specific extension version ... |
| `DescribeTypeRegistration` | `-` | `idempotent` | `RegistrationToken` | - | `DescribeTypeRegistrationOutput` | `CFNRegistryException` | Returns information about an extension's registration, including its current status and type and version identifiers. When you initiate a registration request using RegisterType , you can then use DescribeTypeRegistr ... |
| `DetectStackDrift` | `-` | - | `StackName` | - | `DetectStackDriftOutput` | - | Detects whether a stack's actual configuration differs, or has drifted , from its expected configuration, as defined in the stack template and any values specified as template parameters. For each resource in the sta ... |
| `DetectStackResourceDrift` | `-` | - | `StackName`, `LogicalResourceId` | - | `DetectStackResourceDriftOutput` | - | Returns information about whether a resource's actual configuration differs, or has drifted , from its expected configuration, as defined in the stack template and any values specified as template parameters. This in ... |
| `DetectStackSetDrift` | `-` | `idempotency-token` | `StackSetName` | `OperationId` | `DetectStackSetDriftOutput` | `InvalidOperationException`, `OperationInProgressException`, `StackSetNotFoundException` | Detect drift on a StackSet. When CloudFormation performs drift detection on a StackSet, it performs drift detection on the stack associated with each stack instance in the StackSet. For more information, see Performi ... |
| `EstimateTemplateCost` | `-` | - | - | - | `EstimateTemplateCostOutput` | - | Returns the estimated monthly cost of a template. The return value is an Amazon Web Services Simple Monthly Calculator URL with a query string that describes the resources required to run the template. |
| `ExecuteChangeSet` | `-` | - | `ChangeSetName` | - | `ExecuteChangeSetOutput` | `ChangeSetNotFoundException`, `InsufficientCapabilitiesException`, `InvalidChangeSetStatusException`, `TokenAlreadyExistsException` | Updates a stack using the input information that was provided when the specified change set was created. After the call successfully completes, CloudFormation starts updating the stack. Use the DescribeStacks action ... |
| `ExecuteStackRefactor` | `-` | - | `StackRefactorId` | - | `Unit` | - | Executes the stack refactor operation. |
| `GetGeneratedTemplate` | `-` | - | `GeneratedTemplateName` | - | `GetGeneratedTemplateOutput` | `GeneratedTemplateNotFoundException` | Retrieves a generated template. If the template is in an InProgress or Pending status then the template returned will be the template when the template was last in a Complete status. If the template has not yet been ... |
| `GetHookResult` | `-` | - | - | - | `GetHookResultOutput` | `HookResultNotFoundException` | Retrieves detailed information and remediation guidance for a Hook invocation result. If the Hook uses a KMS key to encrypt annotations, callers of the GetHookResult operation must have kms:Decrypt permissions. For m ... |
| `GetStackPolicy` | `-` | - | `StackName` | - | `GetStackPolicyOutput` | - | Returns the stack policy for a specified stack. If a stack doesn't have a policy, a null value is returned. |
| `GetTemplate` | `-` | - | - | - | `GetTemplateOutput` | `ChangeSetNotFoundException` | Returns the template body for a specified stack. You can get the template for running or deleted stacks. For deleted stacks, GetTemplate returns the template for up to 90 days after the stack has been deleted. If the ... |
| `GetTemplateSummary` | `-` | - | - | - | `GetTemplateSummaryOutput` | `StackSetNotFoundException` | Returns information about a new or existing template. The GetTemplateSummary action is useful for viewing parameter information, such as default parameter values and parameter types, before you create or update a sta ... |
| `ImportStacksToStackSet` | `-` | `idempotency-token` | `StackSetName` | `OperationId` | `ImportStacksToStackSetOutput` | `InvalidOperationException`, `LimitExceededException`, `OperationIdAlreadyExistsException`, `OperationInProgressException`, `StackNotFoundException`, `StackSetNotFoundException`, `StaleRequestException` | Import existing stacks into a new StackSets. Use the stack import operation to import up to 10 stacks into a new StackSet in the same account as the source stack or in a different administrator account and Region, by ... |
| `ListChangeSets` | `-` | `paginated` | `StackName` | - | `ListChangeSetsOutput` | - | Returns the ID and status of each active change set for a stack. For example, CloudFormation lists change sets that are in the CREATE_IN_PROGRESS or CREATE_PENDING state. |
| `ListExports` | `-` | `paginated` | - | - | `ListExportsOutput` | - | Lists all exported output values in the account and Region in which you call this action. Use this action to see the exported output values that you can import into other stacks. To import values, use the Fn::ImportV ... |
| `ListGeneratedTemplates` | `-` | `paginated` | - | - | `ListGeneratedTemplatesOutput` | - | Lists your generated templates in this Region. |
| `ListHookResults` | `-` | - | - | - | `ListHookResultsOutput` | `HookResultNotFoundException` | Returns summaries of invoked Hooks. For more information, see View invocation summaries for CloudFormation Hooks in the CloudFormation Hooks User Guide . This operation supports the following parameter combinations: ... |
| `ListImports` | `-` | `paginated` | `ExportName` | - | `ListImportsOutput` | - | Lists all stacks that are importing an exported output value. To modify or remove an exported output value, first use this action to see which stacks are using it. To see the exported output values in your account, s ... |
| `ListResourceScanRelatedResources` | `-` | `paginated` | `ResourceScanId`, `Resources` | - | `ListResourceScanRelatedResourcesOutput` | `ResourceScanInProgressException`, `ResourceScanNotFoundException` | Lists the related resources for a list of resources from a resource scan. The response indicates whether each returned resource is already managed by CloudFormation. |
| `ListResourceScanResources` | `-` | `paginated` | `ResourceScanId` | - | `ListResourceScanResourcesOutput` | `ResourceScanInProgressException`, `ResourceScanNotFoundException` | Lists the resources from a resource scan. The results can be filtered by resource identifier, resource type prefix, tag key, and tag value. Only resources that match all specified filters are returned. The response i ... |
| `ListResourceScans` | `-` | `paginated` | - | - | `ListResourceScansOutput` | - | List the resource scans from newest to oldest. By default it will return up to 10 resource scans. |
| `ListStackInstanceResourceDrifts` | `-` | - | `StackSetName`, `StackInstanceAccount`, `StackInstanceRegion`, `OperationId` | - | `ListStackInstanceResourceDriftsOutput` | `OperationNotFoundException`, `StackInstanceNotFoundException`, `StackSetNotFoundException` | Returns drift information for resources in a stack instance. ListStackInstanceResourceDrifts returns drift information for the most recent drift detection operation. If an operation is in progress, it may only return ... |
| `ListStackInstances` | `-` | `paginated` | `StackSetName` | - | `ListStackInstancesOutput` | `StackSetNotFoundException` | Returns summary information about stack instances that are associated with the specified StackSet. You can filter for stack instances that are associated with a specific Amazon Web Services account name or Region, or ... |
| `ListStackRefactorActions` | `-` | `paginated` | `StackRefactorId` | - | `ListStackRefactorActionsOutput` | - | Lists the stack refactor actions that will be taken after calling the ExecuteStackRefactor action. |
| `ListStackRefactors` | `-` | `paginated` | - | - | `ListStackRefactorsOutput` | - | Lists all account stack refactor operations and their statuses. |
| `ListStackResources` | `-` | `paginated` | `StackName` | - | `ListStackResourcesOutput` | - | Returns descriptions of all resources of the specified stack. For deleted stacks, ListStackResources returns resource information for up to 90 days after the stack has been deleted. |
| `ListStacks` | `-` | `paginated` | - | - | `ListStacksOutput` | - | Returns the summary information for stacks whose status matches the specified StackStatusFilter . Summary information for stacks that have been deleted is kept for 90 days after the stack is deleted. If no StackStatu ... |
| `ListStackSetAutoDeploymentTargets` | `-` | - | `StackSetName` | - | `ListStackSetAutoDeploymentTargetsOutput` | `StackSetNotFoundException` | Returns summary information about deployment targets for a StackSet. |
| `ListStackSetOperationResults` | `-` | `paginated` | `StackSetName`, `OperationId` | - | `ListStackSetOperationResultsOutput` | `OperationNotFoundException`, `StackSetNotFoundException` | Returns summary information about the results of a StackSet operation. This API provides eventually consistent reads meaning it may take some time but will eventually return the most up-to-date data. |
| `ListStackSetOperations` | `-` | `paginated` | `StackSetName` | - | `ListStackSetOperationsOutput` | `StackSetNotFoundException` | Returns summary information about operations performed on a StackSet. This API provides eventually consistent reads meaning it may take some time but will eventually return the most up-to-date data. |
| `ListStackSets` | `-` | `paginated` | - | - | `ListStackSetsOutput` | - | Returns summary information about StackSets that are associated with the user. This API provides strongly consistent reads meaning it will always return the most up-to-date data. [Self-managed permissions] If you set ... |
| `ListTypeRegistrations` | `-` | `idempotent`, `paginated` | - | - | `ListTypeRegistrationsOutput` | `CFNRegistryException` | Returns a list of registration tokens for the specified extension(s). |
| `ListTypes` | `-` | `idempotent`, `paginated` | - | - | `ListTypesOutput` | `CFNRegistryException` | Returns summary information about all extensions, including your private resource types, modules, and Hooks as well as all public extensions from Amazon Web Services and third-party publishers. |
| `ListTypeVersions` | `-` | `idempotent`, `paginated` | - | - | `ListTypeVersionsOutput` | `CFNRegistryException` | Returns summary information about the versions of an extension. |
| `PublishType` | `-` | `idempotent` | - | - | `PublishTypeOutput` | `CFNRegistryException`, `TypeNotFoundException` | Publishes the specified extension to the CloudFormation registry as a public extension in this Region. Public extensions are available for use by all CloudFormation users. For more information about publishing extens ... |
| `RecordHandlerProgress` | `-` | `idempotent` | `BearerToken`, `OperationStatus` | - | `RecordHandlerProgressOutput` | `InvalidStateTransitionException`, `OperationStatusCheckFailedException` | Reports progress of a resource handler to CloudFormation. Reserved for use by the CloudFormation CLI . Don't use this API in your code. |
| `RegisterPublisher` | `-` | `idempotent` | - | - | `RegisterPublisherOutput` | `CFNRegistryException` | Registers your account as a publisher of public extensions in the CloudFormation registry. Public extensions are available for use by all CloudFormation users. This publisher ID applies to your account in all Amazon ... |
| `RegisterType` | `-` | `idempotent` | `TypeName`, `SchemaHandlerPackage` | - | `RegisterTypeOutput` | `CFNRegistryException` | Registers an extension with the CloudFormation service. Registering an extension makes it available for use in CloudFormation templates in your Amazon Web Services account, and includes: Validating the extension sche ... |
| `RollbackStack` | `-` | - | `StackName` | - | `RollbackStackOutput` | `TokenAlreadyExistsException` | When specifying RollbackStack , you preserve the state of previously provisioned resources when an operation fails. You can check the status of the stack through the DescribeStacks operation. Rolls back the specified ... |
| `SetStackPolicy` | `-` | - | `StackName` | - | `Unit` | - | Sets a stack policy for a specified stack. |
| `SetTypeConfiguration` | `-` | - | `Configuration` | - | `SetTypeConfigurationOutput` | `CFNRegistryException`, `TypeNotFoundException` | Specifies the configuration data for a CloudFormation extension, such as a resource or Hook, in the given account and Region. For more information, see Edit configuration data for extensions in your account in the Cl ... |
| `SetTypeDefaultVersion` | `-` | `idempotent` | - | - | `SetTypeDefaultVersionOutput` | `CFNRegistryException`, `TypeNotFoundException` | Specify the default version of an extension. The default version of an extension will be used in CloudFormation operations. |
| `SignalResource` | `-` | - | `StackName`, `LogicalResourceId`, `UniqueId`, `Status` | - | `Unit` | - | Sends a signal to the specified resource with a success or failure status. You can use the SignalResource operation in conjunction with a creation policy or update policy. CloudFormation doesn't proceed with a stack ... |
| `StartResourceScan` | `-` | - | - | - | `StartResourceScanOutput` | `ResourceScanInProgressException`, `ResourceScanLimitExceededException` | Starts a scan of the resources in this account in this Region. You can the status of a scan using the ListResourceScans API action. |
| `StopStackSetOperation` | `-` | - | `StackSetName`, `OperationId` | - | `StopStackSetOperationOutput` | `InvalidOperationException`, `OperationNotFoundException`, `StackSetNotFoundException` | Stops an in-progress operation on a StackSet and its associated stack instances. StackSets will cancel all the unstarted stack instance deployments and wait for those are in-progress to complete. |
| `TestType` | `-` | `idempotent` | - | - | `TestTypeOutput` | `CFNRegistryException`, `TypeNotFoundException` | Tests a registered extension to make sure it meets all necessary requirements for being published in the CloudFormation registry. For resource types, this includes passing all contracts tests defined for the type. Fo ... |
| `UpdateGeneratedTemplate` | `-` | - | `GeneratedTemplateName` | - | `UpdateGeneratedTemplateOutput` | `AlreadyExistsException`, `GeneratedTemplateNotFoundException`, `LimitExceededException` | Updates a generated template. This can be used to change the name, add and remove resources, refresh resources, and change the DeletionPolicy and UpdateReplacePolicy settings. You can check the status of the update t ... |
| `UpdateStack` | `-` | - | `StackName` | - | `UpdateStackOutput` | `InsufficientCapabilitiesException`, `TokenAlreadyExistsException` | Updates a stack as specified in the template. After the call completes successfully, the stack update starts. You can check the status of the stack through the DescribeStacks action. To get a copy of the template for ... |
| `UpdateStackInstances` | `-` | `idempotency-token` | `StackSetName`, `Regions` | `OperationId` | `UpdateStackInstancesOutput` | `InvalidOperationException`, `OperationIdAlreadyExistsException`, `OperationInProgressException`, `StackInstanceNotFoundException`, `StackSetNotFoundException`, `StaleRequestException` | Updates the parameter values for stack instances for the specified accounts, within the specified Amazon Web Services Regions. A stack instance refers to a stack in a specific account and Region. You can only update ... |
| `UpdateStackSet` | `-` | `idempotency-token` | `StackSetName` | `OperationId` | `UpdateStackSetOutput` | `InvalidOperationException`, `OperationIdAlreadyExistsException`, `OperationInProgressException`, `StackInstanceNotFoundException`, `StackSetNotFoundException`, `StaleRequestException` | Updates the StackSet and associated stack instances in the specified accounts and Amazon Web Services Regions. Even if the StackSet operation created by updating the StackSet fails (completely or partially, below or ... |
| `UpdateTerminationProtection` | `-` | - | `EnableTerminationProtection`, `StackName` | - | `UpdateTerminationProtectionOutput` | - | Updates termination protection for the specified stack. If a user attempts to delete a stack with termination protection enabled, the operation fails and the stack remains unchanged. For more information, see Protect ... |
| `ValidateTemplate` | `-` | - | - | - | `ValidateTemplateOutput` | - | Validates a specified template. CloudFormation first checks if the template is valid JSON. If it isn't, CloudFormation checks if the template is valid YAML. If both these checks fail, CloudFormation returns a templat ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AlreadyExistsException` | `structure` | Message | The resource with the name requested already exists. |
| `CFNRegistryException` | `structure` | Message | An error occurred during a CloudFormation registry operation. |
| `ChangeSetNotFoundException` | `structure` | Message | The specified change set name or ID doesn't exit. To view valid change sets for a stack, use the ListChangeSets operation. |
| `ConcurrentResourcesLimitExceededException` | `structure` | Message | No more than 5 generated templates can be in an InProgress or Pending status at one time. This error is also returned if a generated template that is in an ... |
| `CreatedButModifiedException` | `structure` | Message | The specified resource exists, but has been changed. |
| `GeneratedTemplateNotFoundException` | `structure` | Message | The generated template was not found. |
| `HookResultNotFoundException` | `structure` | Message | The specified target doesn't have any requested Hook invocations. |
| `InsufficientCapabilitiesException` | `structure` | Message | The template contains resources with capabilities that weren't specified in the Capabilities parameter. |
| `InvalidChangeSetStatusException` | `structure` | Message | The specified change set can't be used to update the stack. For example, the change set status might be CREATE_IN_PROGRESS , or the stack status might be UP ... |
| `InvalidOperationException` | `structure` | Message | The specified operation isn't valid. |
| `InvalidStateTransitionException` | `structure` | Message | Error reserved for use by the CloudFormation CLI . CloudFormation doesn't return this error to users. |
| `LimitExceededException` | `structure` | Message | The quota for the resource has already been reached. For information about resource and stack limitations, see CloudFormation quotas in the CloudFormation U ... |
| `NameAlreadyExistsException` | `structure` | Message | The specified name is already in use. |
| `OperationIdAlreadyExistsException` | `structure` | Message | The specified operation ID already exists. |
| `OperationInProgressException` | `structure` | Message | Another operation is currently in progress for this StackSet. Only one operation can be performed for a stack set at a given time. |
| `OperationNotFoundException` | `structure` | Message | The specified ID refers to an operation that doesn't exist. |
| `OperationStatusCheckFailedException` | `structure` | Message | Error reserved for use by the CloudFormation CLI . CloudFormation doesn't return this error to users. |
| `ResourceScanInProgressException` | `structure` | Message | A resource scan is currently in progress. Only one can be run at a time for an account in a Region. |
| `ResourceScanLimitExceededException` | `structure` | Message | The limit on resource scans has been exceeded. Reasons include: Exceeded the daily quota for resource scans. A resource scan recently failed. You must wait ... |
| `ResourceScanNotFoundException` | `structure` | Message | The resource scan was not found. |
| `StackInstanceNotFoundException` | `structure` | Message | The specified stack instance doesn't exist. |
| `StackNotFoundException` | `structure` | Message | The specified stack ARN doesn't exist or stack doesn't exist corresponding to the ARN in input. |
| `StackRefactorNotFoundException` | `structure` | Message | The specified stack refactor can't be found. |
| `StackSetNotEmptyException` | `structure` | Message | You can't yet delete this StackSet, because it still contains one or more stack instances. Delete all stack instances from the StackSet before deleting the ... |
| `StackSetNotFoundException` | `structure` | Message | The specified StackSet doesn't exist. |
| `StaleRequestException` | `structure` | Message | Another operation has been performed on this StackSet since the specified operation was performed. |
| `TokenAlreadyExistsException` | `structure` | Message | A client request token already exists. |
| `TypeConfigurationNotFoundException` | `structure` | Message | The specified extension configuration can't be found. |
| `TypeNotFoundException` | `structure` | Message | The specified extension doesn't exist in the CloudFormation registry. |
| `ActivateOrganizationsAccessInput` | `structure` | **empty (no members)** | - |
| `ActivateOrganizationsAccessOutput` | `structure` | **empty (no members)** | - |
| `ActivateTypeInput` | `structure` | Type, PublicTypeArn, PublisherId, TypeName, TypeNameAlias, AutoUpdate, LoggingConfig, ExecutionRoleArn, VersionBump, MajorVersion | - |
| `ActivateTypeOutput` | `structure` | Arn | - |
| `BatchDescribeTypeConfigurationsInput` | `structure` | TypeConfigurationIdentifiers | - |
| `BatchDescribeTypeConfigurationsOutput` | `structure` | Errors, UnprocessedTypeConfigurations, TypeConfigurations | - |
| `CancelUpdateStackInput` | `structure` | StackName, ClientRequestToken | The input for the CancelUpdateStack action. |
| `ContinueUpdateRollbackInput` | `structure` | StackName, RoleARN, ResourcesToSkip, ClientRequestToken | The input for the ContinueUpdateRollback action. |
| `ContinueUpdateRollbackOutput` | `structure` | **empty (no members)** | The output for a ContinueUpdateRollback operation. |
| `CreateChangeSetInput` | `structure` | StackName, TemplateBody, TemplateURL, UsePreviousTemplate, Parameters, Capabilities, ResourceTypes, RoleARN, RollbackConfiguration, NotificationARNs, Tags, ChangeSetName, ... (+8) | The input for the CreateChangeSet action. |
| `CreateChangeSetOutput` | `structure` | Id, StackId | The output for the CreateChangeSet action. |
| `AccountFilterType` | `enum` | NONE, INTERSECTION, DIFFERENCE, UNION | - |
| `AccountGateStatus` | `enum` | SUCCEEDED, FAILED, SKIPPED | - |
| `AfterValueFrom` | `enum` | TEMPLATE | - |
| `AnnotationSeverityLevel` | `enum` | INFORMATIONAL, LOW, MEDIUM, HIGH, CRITICAL | - |
| `AnnotationStatus` | `enum` | PASSED, FAILED, SKIPPED | - |
| `AttributeChangeType` | `enum` | Add, Remove, Modify, SyncWithActual | - |
| `BeaconStackOperationStatus` | `enum` | IN_PROGRESS, SUCCEEDED, FAILED | - |
| `BeforeValueFrom` | `enum` | PREVIOUS_DEPLOYMENT_STATE, ACTUAL_STATE | - |
| `CallAs` | `enum` | SELF, DELEGATED_ADMIN | - |
| `Capability` | `enum` | CAPABILITY_IAM, CAPABILITY_NAMED_IAM, CAPABILITY_AUTO_EXPAND | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
