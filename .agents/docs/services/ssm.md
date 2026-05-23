# Amazon Simple Systems Manager (SSM)

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services Systems Manager is the operations hub for your Amazon Web Services applications and resources and a secure end-to-end management solution for hybrid cloud environments that enables safe and secure operations at scale. This reference is intended to be used with the Amazon Web Services Systems Manager User Guide. To get started, see Setting up Amazon Web Services Systems Manager. Related resources For information about each of the tools that comprise Systems Manager, see Using Systems Manager tools in the Amazon Web Services Systems Manager User Guide . For details about predefined runbooks for Automation, a tool in Amazon Web Services Systems Manager, see the Systems Manager Automation Runbook Reference .

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Simple Systems Manager (SSM) where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon Simple Systems Manager (SSM) by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Amazon Simple Systems Manager (SSM) by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon Simple Systems Manager (SSM) workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `Get`, `List`, `Update`, `Delete` operation families, including `DescribeActivations`, `DescribeAssociation`, `DescribeAssociationExecutionTargets`, `DescribeAssociationExecutions`, `GetAccessToken`, `GetAutomationExecution`.

## Service Identity and Protocol

- AWS model slug: `ssm`
- AWS SDK for Rust slug: `ssm`
- Model version: `2014-11-06`
- Model file: `vendor/api-models-aws/models/ssm/service/2014-11-06/ssm-2014-11-06.json`
- SDK ID: `SSM`
- Endpoint prefix: `ssm`
- ARN namespace: `ssm`
- CloudFormation name: `SSM`
- CloudTrail event source: `ssm.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (33), `Get` (27), `List` (18), `Update` (14), `Delete` (12), `Create` (9), `Start` (6), `Deregister` (4).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddTagsToResource`, `AssociateOpsItemRelatedItem`, `CancelCommand`, `CancelMaintenanceWindowExecution`, `CreateActivation`, `CreateAssociation`, `CreateAssociationBatch`, `CreateDocument`, `CreateMaintenanceWindow`, `CreateOpsItem`, `CreateOpsMetadata`, `CreatePatchBaseline`, `CreateResourceDataSync`, `DeleteActivation`, `DeleteAssociation`, `DeleteDocument`, `DeleteInventory`, `DeleteMaintenanceWindow`, `DeleteOpsItem`, `DeleteOpsMetadata`, ... (+42).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeActivations`, `DescribeAssociation`, `DescribeAssociationExecutionTargets`, `DescribeAssociationExecutions`, `DescribeAutomationExecutions`, `DescribeAutomationStepExecutions`, `DescribeAvailablePatches`, `DescribeDocument`, `DescribeDocumentPermission`, `DescribeEffectiveInstanceAssociations`, `DescribeEffectivePatchesForPatchBaseline`, `DescribeInstanceAssociationsStatus`, `DescribeInstanceInformation`, `DescribeInstancePatchStates`, `DescribeInstancePatchStatesForPatchGroup`, `DescribeInstancePatches`, `DescribeInstanceProperties`, `DescribeInventoryDeletions`, `DescribeMaintenanceWindowExecutionTaskInvocations`, `DescribeMaintenanceWindowExecutionTasks`, ... (+58).
- Pagination is modelled for 50 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 5 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelCommand`, `CancelMaintenanceWindowExecution`, `DeregisterTaskFromMaintenanceWindow`, `DescribeAssociationExecutionTargets`, `DescribeAssociationExecutions`, `DescribeAutomationExecutions`, `DescribeAutomationStepExecutions`, `DescribeMaintenanceWindowExecutionTaskInvocations`, `DescribeMaintenanceWindowExecutionTasks`, `DescribeMaintenanceWindowExecutions`, `DescribeMaintenanceWindowTasks`, `GetAutomationExecution`, `GetExecutionPreview`, `GetMaintenanceWindowExecution`, `GetMaintenanceWindowExecutionTask`, `GetMaintenanceWindowExecutionTaskInvocation`, `GetMaintenanceWindowTask`, `RegisterTaskWithMaintenanceWindow`, `StartAccessRequest`, `StartAssociationsOnce`, ... (+6).
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 146 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `EventBridge`, `SNS`, `Lambda`, `EC2/VPC`, `ECR`, `ECS`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-parameter-store.html
- https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_GetParameter.html
- https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_LabelParameterVersion.html
- https://docs.aws.amazon.com/systems-manager/latest/userguide/secure-string-parameter-kms-encryption.html

Research outcomes:
- Parameter Store supports String, StringList, and SecureString parameters. Parameter values cannot reference or nest other parameters.
- String with data type `aws:ec2:image` triggers asynchronous validation that checks AMI format, existence, and caller permission.
- Only SecureString values are encrypted. Parameter names, descriptions, and other properties are not encrypted.
- SecureString parameters use symmetric KMS keys only. WithDecryption is ignored for String and StringList parameters.
- GetParameter can select a version or label using `name:version` or `name:label`.
- Parameter labels are aliases for versions. A version can have at most 10 labels, and the same label cannot be attached to different versions of the same parameter.
- Labels can be moved between versions, but cannot be created during initial parameter creation.
- Standard parameter values are limited to 4 KB; advanced parameters support 8 KB and additional capabilities.

Parity implications:
- Model parameters, versions, labels, tiers, data types, policies, KMS metadata, and asynchronous validation state.
- Implement label uniqueness, label movement, version selectors, latest-version defaults, and version-not-found errors.
- SecureString handling should enforce symmetric KMS-key constraints, WithDecryption behaviour, and encryption-context-sensitive permission checks where applicable.

## Operation Groups

### Describe

- Operations: `DescribeActivations`, `DescribeAssociation`, `DescribeAssociationExecutions`, `DescribeAssociationExecutionTargets`, `DescribeAutomationExecutions`, `DescribeAutomationStepExecutions`, `DescribeAvailablePatches`, `DescribeDocument`, `DescribeDocumentPermission`, `DescribeEffectiveInstanceAssociations`, `DescribeEffectivePatchesForPatchBaseline`, `DescribeInstanceAssociationsStatus`, `DescribeInstanceInformation`, `DescribeInstancePatches`, `DescribeInstancePatchStates`, `DescribeInstancePatchStatesForPatchGroup`, `DescribeInstanceProperties`, `DescribeInventoryDeletions`, `DescribeMaintenanceWindowExecutions`, `DescribeMaintenanceWindowExecutionTaskInvocations`, `DescribeMaintenanceWindowExecutionTasks`, `DescribeMaintenanceWindows`, `DescribeMaintenanceWindowSchedule`, `DescribeMaintenanceWindowsForTarget`, `DescribeMaintenanceWindowTargets`, `DescribeMaintenanceWindowTasks`, `DescribeOpsItems`, `DescribeParameters`, `DescribePatchBaselines`, `DescribePatchGroups`, `DescribePatchGroupState`, `DescribePatchProperties`, `DescribeSessions`
- Traits: `paginated` (29)
- Common required input members in this group: `AssociationId`, `Name`, `InstanceId`, `PatchGroup`, `WindowId`, `WindowExecutionId`

### Get

- Operations: `GetAccessToken`, `GetAutomationExecution`, `GetCalendarState`, `GetCommandInvocation`, `GetConnectionStatus`, `GetDefaultPatchBaseline`, `GetDeployablePatchSnapshotForInstance`, `GetDocument`, `GetExecutionPreview`, `GetInventory`, `GetInventorySchema`, `GetMaintenanceWindow`, `GetMaintenanceWindowExecution`, `GetMaintenanceWindowExecutionTask`, `GetMaintenanceWindowExecutionTaskInvocation`, `GetMaintenanceWindowTask`, `GetOpsItem`, `GetOpsMetadata`, `GetOpsSummary`, `GetParameter`, `GetParameterHistory`, `GetParameters`, `GetParametersByPath`, `GetPatchBaseline`, `GetPatchBaselineForPatchGroup`, `GetResourcePolicies`, `GetServiceSetting`
- Traits: `paginated` (6)
- Common required input members in this group: `InstanceId`, `Name`, `WindowId`, `WindowExecutionId`, `TaskId`

### List

- Operations: `ListAssociations`, `ListAssociationVersions`, `ListCommandInvocations`, `ListCommands`, `ListComplianceItems`, `ListComplianceSummaries`, `ListDocumentMetadataHistory`, `ListDocuments`, `ListDocumentVersions`, `ListInventoryEntries`, `ListNodes`, `ListNodesSummary`, `ListOpsItemEvents`, `ListOpsItemRelatedItems`, `ListOpsMetadata`, `ListResourceComplianceSummaries`, `ListResourceDataSync`, `ListTagsForResource`
- Traits: `paginated` (15)
- Common required input members in this group: `Name`

### Update

- Operations: `UpdateAssociation`, `UpdateAssociationStatus`, `UpdateDocument`, `UpdateDocumentDefaultVersion`, `UpdateDocumentMetadata`, `UpdateMaintenanceWindow`, `UpdateMaintenanceWindowTarget`, `UpdateMaintenanceWindowTask`, `UpdateManagedInstanceRole`, `UpdateOpsItem`, `UpdateOpsMetadata`, `UpdatePatchBaseline`, `UpdateResourceDataSync`, `UpdateServiceSetting`
- Common required input members in this group: `Name`, `InstanceId`, `WindowId`

### Delete

- Operations: `DeleteActivation`, `DeleteAssociation`, `DeleteDocument`, `DeleteInventory`, `DeleteMaintenanceWindow`, `DeleteOpsItem`, `DeleteOpsMetadata`, `DeleteParameter`, `DeleteParameters`, `DeletePatchBaseline`, `DeleteResourceDataSync`, `DeleteResourcePolicy`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `Name`

### Create

- Operations: `CreateActivation`, `CreateAssociation`, `CreateAssociationBatch`, `CreateDocument`, `CreateMaintenanceWindow`, `CreateOpsItem`, `CreateOpsMetadata`, `CreatePatchBaseline`, `CreateResourceDataSync`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `Name`

### Start

- Operations: `StartAccessRequest`, `StartAssociationsOnce`, `StartAutomationExecution`, `StartChangeRequestExecution`, `StartExecutionPreview`, `StartSession`
- Common required input members in this group: `DocumentName`

### Deregister

- Operations: `DeregisterManagedInstance`, `DeregisterPatchBaselineForPatchGroup`, `DeregisterTargetFromMaintenanceWindow`, `DeregisterTaskFromMaintenanceWindow`
- Common required input members in this group: `WindowId`

### Put

- Operations: `PutComplianceItems`, `PutInventory`, `PutParameter`, `PutResourcePolicy`
- Common required input members in this group: `Items`

### Register

- Operations: `RegisterDefaultPatchBaseline`, `RegisterPatchBaselineForPatchGroup`, `RegisterTargetWithMaintenanceWindow`, `RegisterTaskWithMaintenanceWindow`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `BaselineId`, `WindowId`

### Cancel

- Operations: `CancelCommand`, `CancelMaintenanceWindowExecution`
- Common required input members in this group: -

### Send

- Operations: `SendAutomationSignal`, `SendCommand`
- Common required input members in this group: -

### Add

- Operations: `AddTagsToResource`
- Common required input members in this group: -

### Associate

- Operations: `AssociateOpsItemRelatedItem`
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociateOpsItemRelatedItem`
- Common required input members in this group: -

### Label

- Operations: `LabelParameterVersion`
- Common required input members in this group: -

### Modify

- Operations: `ModifyDocumentPermission`
- Common required input members in this group: -

### Remove

- Operations: `RemoveTagsFromResource`
- Common required input members in this group: -

### Reset

- Operations: `ResetServiceSetting`
- Common required input members in this group: -

### Resume

- Operations: `ResumeSession`
- Common required input members in this group: -

### Stop

- Operations: `StopAutomationExecution`
- Common required input members in this group: -

### Terminate

- Operations: `TerminateSession`
- Common required input members in this group: -

### Unlabel

- Operations: `UnlabelParameterVersion`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddTagsToResource` | `-` | - | `ResourceType`, `ResourceId`, `Tags` | - | `AddTagsToResourceResult` | `InternalServerError`, `InvalidResourceId`, `InvalidResourceType`, `TooManyTagsError`, `TooManyUpdates` | Adds or overwrites one or more tags for the specified resource. Tags are metadata that you can assign to your automations, documents, managed nodes, maintenance windows, Parameter Store parameters, and patch baseline ... |
| `AssociateOpsItemRelatedItem` | `-` | - | `OpsItemId`, `AssociationType`, `ResourceType`, `ResourceUri` | - | `AssociateOpsItemRelatedItemResponse` | `InternalServerError`, `OpsItemConflictException`, `OpsItemInvalidParameterException`, `OpsItemLimitExceededException`, `OpsItemNotFoundException`, `OpsItemRelatedItemAlreadyExistsException` | Associates a related item to a Systems Manager OpsCenter OpsItem. For example, you can associate an Incident Manager incident or analysis with an OpsItem. Incident Manager and OpsCenter are tools in Amazon Web Servic ... |
| `CancelCommand` | `-` | - | `CommandId` | - | `CancelCommandResult` | `DuplicateInstanceId`, `InternalServerError`, `InvalidCommandId`, `InvalidInstanceId` | Attempts to cancel the command specified by the Command ID. There is no guarantee that the command will be terminated and the underlying process stopped. |
| `CancelMaintenanceWindowExecution` | `-` | - | `WindowExecutionId` | - | `CancelMaintenanceWindowExecutionResult` | `DoesNotExistException`, `InternalServerError` | Stops a maintenance window execution that is already in progress and cancels any tasks in the window that haven't already starting running. Tasks already in progress will continue to completion. |
| `CreateActivation` | `-` | - | `IamRole` | - | `CreateActivationResult` | `InternalServerError`, `InvalidParameters` | Generates an activation code and activation ID you can use to register your on-premises servers, edge devices, or virtual machine (VM) with Amazon Web Services Systems Manager. Registering these machines with Systems ... |
| `CreateAssociation` | `-` | - | `Name` | - | `CreateAssociationResult` | `AssociationAlreadyExists`, `AssociationLimitExceeded`, `InternalServerError`, `InvalidDocument`, `InvalidDocumentVersion`, `InvalidInstanceId`, `InvalidOutputLocation`, `InvalidParameters`, `InvalidSchedule`, `InvalidTag`, `InvalidTarget`, `InvalidTargetMaps`, `UnsupportedPlatformType` | A State Manager association defines the state that you want to maintain on your managed nodes. For example, an association can specify that anti-virus software must be installed and running on your managed nodes, or ... |
| `CreateAssociationBatch` | `-` | - | `Entries` | - | `CreateAssociationBatchResult` | `AssociationLimitExceeded`, `DuplicateInstanceId`, `InternalServerError`, `InvalidDocument`, `InvalidDocumentVersion`, `InvalidInstanceId`, `InvalidOutputLocation`, `InvalidParameters`, `InvalidSchedule`, `InvalidTarget`, `InvalidTargetMaps`, `UnsupportedPlatformType` | Associates the specified Amazon Web Services Systems Manager document (SSM document) with the specified managed nodes or targets. When you associate a document with one or more managed nodes using IDs or tags, Amazon ... |
| `CreateDocument` | `-` | - | `Content`, `Name` | - | `CreateDocumentResult` | `DocumentAlreadyExists`, `DocumentLimitExceeded`, `InternalServerError`, `InvalidDocumentContent`, `InvalidDocumentSchemaVersion`, `MaxDocumentSizeExceeded`, `NoLongerSupportedException`, `TooManyUpdates` | Creates a Amazon Web Services Systems Manager (SSM document). An SSM document defines the actions that Systems Manager performs on your managed nodes. For more information about SSM documents, including information a ... |
| `CreateMaintenanceWindow` | `-` | `idempotency-token` | `Name`, `Schedule`, `Duration`, `Cutoff`, `AllowUnassociatedTargets` | `ClientToken` | `CreateMaintenanceWindowResult` | `IdempotentParameterMismatch`, `InternalServerError`, `ResourceLimitExceededException` | Creates a new maintenance window. The value you specify for Duration determines the specific end time for the maintenance window based on the time it begins. No maintenance window tasks are permitted to start after t ... |
| `CreateOpsItem` | `-` | - | `Description`, `Source`, `Title` | - | `CreateOpsItemResponse` | `InternalServerError`, `OpsItemAccessDeniedException`, `OpsItemAlreadyExistsException`, `OpsItemInvalidParameterException`, `OpsItemLimitExceededException` | Creates a new OpsItem. You must have permission in Identity and Access Management (IAM) to create a new OpsItem. For more information, see Set up OpsCenter in the Amazon Web Services Systems Manager User Guide . Oper ... |
| `CreateOpsMetadata` | `-` | - | `ResourceId` | - | `CreateOpsMetadataResult` | `InternalServerError`, `OpsMetadataAlreadyExistsException`, `OpsMetadataInvalidArgumentException`, `OpsMetadataLimitExceededException`, `OpsMetadataTooManyUpdatesException` | If you create a new application in Application Manager, Amazon Web Services Systems Manager calls this API operation to specify information about the new application, including the application type. |
| `CreatePatchBaseline` | `-` | `idempotency-token` | `Name` | `ClientToken` | `CreatePatchBaselineResult` | `IdempotentParameterMismatch`, `InternalServerError`, `ResourceLimitExceededException` | Creates a patch baseline. For information about valid key-value pairs in PatchFilters for each supported operating system type, see PatchFilter . |
| `CreateResourceDataSync` | `-` | - | `SyncName` | - | `CreateResourceDataSyncResult` | `InternalServerError`, `ResourceDataSyncAlreadyExistsException`, `ResourceDataSyncCountExceededException`, `ResourceDataSyncInvalidConfigurationException` | A resource data sync helps you view data from multiple sources in a single location. Amazon Web Services Systems Manager offers two types of resource data sync: SyncToDestination and SyncFromSource . You can configur ... |
| `DeleteActivation` | `-` | - | `ActivationId` | - | `DeleteActivationResult` | `InternalServerError`, `InvalidActivation`, `InvalidActivationId`, `TooManyUpdates` | Deletes an activation. You aren't required to delete an activation. If you delete an activation, you can no longer use it to register additional managed nodes. Deleting an activation doesn't de-register managed nodes ... |
| `DeleteAssociation` | `-` | - | - | - | `DeleteAssociationResult` | `AssociationDoesNotExist`, `InternalServerError`, `InvalidDocument`, `InvalidInstanceId`, `TooManyUpdates` | Disassociates the specified Amazon Web Services Systems Manager document (SSM document) from the specified managed node. If you created the association by using the Targets parameter, then you must delete the associa ... |
| `DeleteDocument` | `-` | - | `Name` | - | `DeleteDocumentResult` | `AssociatedInstances`, `InternalServerError`, `InvalidDocument`, `InvalidDocumentOperation`, `TooManyUpdates` | Deletes the Amazon Web Services Systems Manager document (SSM document) and all managed node associations to the document. Before you delete the document, we recommend that you use DeleteAssociation to disassociate a ... |
| `DeleteInventory` | `-` | `idempotency-token` | `TypeName` | `ClientToken` | `DeleteInventoryResult` | `InternalServerError`, `InvalidDeleteInventoryParametersException`, `InvalidInventoryRequestException`, `InvalidOptionException`, `InvalidTypeNameException` | Delete a custom inventory type or the data associated with a custom Inventory type. Deleting a custom inventory type is also referred to as deleting a custom inventory schema. |
| `DeleteMaintenanceWindow` | `-` | - | `WindowId` | - | `DeleteMaintenanceWindowResult` | `InternalServerError` | Deletes a maintenance window. |
| `DeleteOpsItem` | `-` | - | `OpsItemId` | - | `DeleteOpsItemResponse` | `InternalServerError`, `OpsItemInvalidParameterException` | Delete an OpsItem. You must have permission in Identity and Access Management (IAM) to delete an OpsItem. Note the following important information about this operation. Deleting an OpsItem is irreversible. You can't ... |
| `DeleteOpsMetadata` | `-` | - | `OpsMetadataArn` | - | `DeleteOpsMetadataResult` | `InternalServerError`, `OpsMetadataInvalidArgumentException`, `OpsMetadataNotFoundException` | Delete OpsMetadata related to an application. |
| `DeleteParameter` | `-` | - | `Name` | - | `DeleteParameterResult` | `InternalServerError`, `ParameterNotFound` | Delete a parameter from the system. After deleting a parameter, wait for at least 30 seconds to create a parameter with the same name. |
| `DeleteParameters` | `-` | - | `Names` | - | `DeleteParametersResult` | `InternalServerError` | Delete a list of parameters. After deleting a parameter, wait for at least 30 seconds to create a parameter with the same name. |
| `DeletePatchBaseline` | `-` | - | `BaselineId` | - | `DeletePatchBaselineResult` | `InternalServerError`, `ResourceInUseException` | Deletes a patch baseline. |
| `DeleteResourceDataSync` | `-` | - | `SyncName` | - | `DeleteResourceDataSyncResult` | `InternalServerError`, `ResourceDataSyncInvalidConfigurationException`, `ResourceDataSyncNotFoundException` | Deletes a resource data sync configuration. After the configuration is deleted, changes to data on managed nodes are no longer synced to or from the target. Deleting a sync configuration doesn't delete data. |
| `DeleteResourcePolicy` | `-` | - | `ResourceArn`, `PolicyId`, `PolicyHash` | - | `DeleteResourcePolicyResponse` | `InternalServerError`, `MalformedResourcePolicyDocumentException`, `ResourceNotFoundException`, `ResourcePolicyConflictException`, `ResourcePolicyInvalidParameterException`, `ResourcePolicyNotFoundException` | Deletes a Systems Manager resource policy. A resource policy helps you to define the IAM entity (for example, an Amazon Web Services account) that can manage your Systems Manager resources. The following resources su ... |
| `DeregisterManagedInstance` | `-` | - | `InstanceId` | - | `DeregisterManagedInstanceResult` | `InternalServerError`, `InvalidInstanceId` | Removes the server or virtual machine from the list of registered servers. If you want to reregister an on-premises server, edge device, or VM, you must use a different Activation Code and Activation ID than used to ... |
| `DeregisterPatchBaselineForPatchGroup` | `-` | - | `BaselineId`, `PatchGroup` | - | `DeregisterPatchBaselineForPatchGroupResult` | `InternalServerError`, `InvalidResourceId` | Removes a patch group from a patch baseline. |
| `DeregisterTargetFromMaintenanceWindow` | `-` | - | `WindowId`, `WindowTargetId` | - | `DeregisterTargetFromMaintenanceWindowResult` | `DoesNotExistException`, `InternalServerError`, `TargetInUseException` | Removes a target from a maintenance window. |
| `DeregisterTaskFromMaintenanceWindow` | `-` | - | `WindowId`, `WindowTaskId` | - | `DeregisterTaskFromMaintenanceWindowResult` | `DoesNotExistException`, `InternalServerError` | Removes a task from a maintenance window. |
| `DescribeActivations` | `-` | `paginated` | - | - | `DescribeActivationsResult` | `InternalServerError`, `InvalidFilter`, `InvalidNextToken` | Describes details about the activation, such as the date and time the activation was created, its expiration date, the Identity and Access Management (IAM) role assigned to the managed nodes in the activation, and th ... |
| `DescribeAssociation` | `-` | - | - | - | `DescribeAssociationResult` | `AssociationDoesNotExist`, `InternalServerError`, `InvalidAssociationVersion`, `InvalidDocument`, `InvalidInstanceId` | Describes the association for the specified target or managed node. If you created the association by using the Targets parameter, then you must retrieve the association by using the association ID. |
| `DescribeAssociationExecutions` | `-` | `paginated` | `AssociationId` | - | `DescribeAssociationExecutionsResult` | `AssociationDoesNotExist`, `InternalServerError`, `InvalidNextToken` | Views all executions for a specific association ID. |
| `DescribeAssociationExecutionTargets` | `-` | `paginated` | `AssociationId`, `ExecutionId` | - | `DescribeAssociationExecutionTargetsResult` | `AssociationDoesNotExist`, `AssociationExecutionDoesNotExist`, `InternalServerError`, `InvalidNextToken` | Views information about a specific execution of a specific association. |
| `DescribeAutomationExecutions` | `-` | `paginated` | - | - | `DescribeAutomationExecutionsResult` | `InternalServerError`, `InvalidFilterKey`, `InvalidFilterValue`, `InvalidNextToken` | Provides details about all active and terminated Automation executions. |
| `DescribeAutomationStepExecutions` | `-` | `paginated` | `AutomationExecutionId` | - | `DescribeAutomationStepExecutionsResult` | `AutomationExecutionNotFoundException`, `InternalServerError`, `InvalidFilterKey`, `InvalidFilterValue`, `InvalidNextToken` | Information about all active and terminated step executions in an Automation workflow. |
| `DescribeAvailablePatches` | `-` | `paginated` | - | - | `DescribeAvailablePatchesResult` | `InternalServerError` | Lists all patches eligible to be included in a patch baseline. Currently, DescribeAvailablePatches supports only the Amazon Linux 1, Amazon Linux 2, and Windows Server operating systems. |
| `DescribeDocument` | `-` | - | `Name` | - | `DescribeDocumentResult` | `InternalServerError`, `InvalidDocument`, `InvalidDocumentVersion` | Describes the specified Amazon Web Services Systems Manager document (SSM document). |
| `DescribeDocumentPermission` | `-` | - | `Name`, `PermissionType` | - | `DescribeDocumentPermissionResponse` | `InternalServerError`, `InvalidDocument`, `InvalidDocumentOperation`, `InvalidNextToken`, `InvalidPermissionType` | Describes the permissions for a Amazon Web Services Systems Manager document (SSM document). If you created the document, you are the owner. If a document is shared, it can either be shared privately (by specifying a ... |
| `DescribeEffectiveInstanceAssociations` | `-` | `paginated` | `InstanceId` | - | `DescribeEffectiveInstanceAssociationsResult` | `InternalServerError`, `InvalidInstanceId`, `InvalidNextToken` | All associations for the managed nodes. |
| `DescribeEffectivePatchesForPatchBaseline` | `-` | `paginated` | `BaselineId` | - | `DescribeEffectivePatchesForPatchBaselineResult` | `DoesNotExistException`, `InternalServerError`, `InvalidResourceId`, `UnsupportedOperatingSystem` | Retrieves the current effective patches (the patch and the approval state) for the specified patch baseline. Applies to patch baselines for Windows only. |
| `DescribeInstanceAssociationsStatus` | `-` | `paginated` | `InstanceId` | - | `DescribeInstanceAssociationsStatusResult` | `InternalServerError`, `InvalidInstanceId`, `InvalidNextToken` | The status of the associations for the managed nodes. |
| `DescribeInstanceInformation` | `-` | `paginated` | - | - | `DescribeInstanceInformationResult` | `InternalServerError`, `InvalidFilterKey`, `InvalidInstanceId`, `InvalidInstanceInformationFilterValue`, `InvalidNextToken` | Provides information about one or more of your managed nodes, including the operating system platform, SSM Agent version, association status, and IP address. This operation does not return information for nodes that ... |
| `DescribeInstancePatches` | `-` | `paginated` | `InstanceId` | - | `DescribeInstancePatchesResult` | `InternalServerError`, `InvalidFilter`, `InvalidInstanceId`, `InvalidNextToken` | Retrieves information about the patches on the specified managed node and their state relative to the patch baseline being used for the node. |
| `DescribeInstancePatchStates` | `-` | `paginated` | `InstanceIds` | - | `DescribeInstancePatchStatesResult` | `InternalServerError`, `InvalidNextToken` | Retrieves the high-level patch state of one or more managed nodes. |
| `DescribeInstancePatchStatesForPatchGroup` | `-` | `paginated` | `PatchGroup` | - | `DescribeInstancePatchStatesForPatchGroupResult` | `InternalServerError`, `InvalidFilter`, `InvalidNextToken` | Retrieves the high-level patch state for the managed nodes in the specified patch group. |
| `DescribeInstanceProperties` | `-` | `paginated` | - | - | `DescribeInstancePropertiesResult` | `InternalServerError`, `InvalidActivationId`, `InvalidDocument`, `InvalidFilterKey`, `InvalidInstanceId`, `InvalidInstancePropertyFilterValue`, `InvalidNextToken` | An API operation used by the Systems Manager console to display information about Systems Manager managed nodes. |
| `DescribeInventoryDeletions` | `-` | `paginated` | - | - | `DescribeInventoryDeletionsResult` | `InternalServerError`, `InvalidDeletionIdException`, `InvalidNextToken` | Describes a specific delete inventory operation. |
| `DescribeMaintenanceWindowExecutions` | `-` | `paginated` | `WindowId` | - | `DescribeMaintenanceWindowExecutionsResult` | `InternalServerError` | Lists the executions of a maintenance window. This includes information about when the maintenance window was scheduled to be active, and information about tasks registered and run with the maintenance window. |
| `DescribeMaintenanceWindowExecutionTaskInvocations` | `-` | `paginated` | `WindowExecutionId`, `TaskId` | - | `DescribeMaintenanceWindowExecutionTaskInvocationsResult` | `DoesNotExistException`, `InternalServerError` | Retrieves the individual task executions (one per target) for a particular task run as part of a maintenance window execution. |
| `DescribeMaintenanceWindowExecutionTasks` | `-` | `paginated` | `WindowExecutionId` | - | `DescribeMaintenanceWindowExecutionTasksResult` | `DoesNotExistException`, `InternalServerError` | For a given maintenance window execution, lists the tasks that were run. |
| `DescribeMaintenanceWindows` | `-` | `paginated` | - | - | `DescribeMaintenanceWindowsResult` | `InternalServerError` | Retrieves the maintenance windows in an Amazon Web Services account. |
| `DescribeMaintenanceWindowSchedule` | `-` | `paginated` | - | - | `DescribeMaintenanceWindowScheduleResult` | `DoesNotExistException`, `InternalServerError` | Retrieves information about upcoming executions of a maintenance window. |
| `DescribeMaintenanceWindowsForTarget` | `-` | `paginated` | `Targets`, `ResourceType` | - | `DescribeMaintenanceWindowsForTargetResult` | `InternalServerError` | Retrieves information about the maintenance window targets or tasks that a managed node is associated with. |
| `DescribeMaintenanceWindowTargets` | `-` | `paginated` | `WindowId` | - | `DescribeMaintenanceWindowTargetsResult` | `DoesNotExistException`, `InternalServerError` | Lists the targets registered with the maintenance window. |
| `DescribeMaintenanceWindowTasks` | `-` | `paginated` | `WindowId` | - | `DescribeMaintenanceWindowTasksResult` | `DoesNotExistException`, `InternalServerError` | Lists the tasks in a maintenance window. For maintenance window tasks without a specified target, you can't supply values for --max-errors and --max-concurrency . Instead, the system inserts a placeholder value of 1 ... |
| `DescribeOpsItems` | `-` | `paginated` | - | - | `DescribeOpsItemsResponse` | `InternalServerError` | Query a set of OpsItems. You must have permission in Identity and Access Management (IAM) to query a list of OpsItems. For more information, see Set up OpsCenter in the Amazon Web Services Systems Manager User Guide ... |
| `DescribeParameters` | `-` | `paginated` | - | - | `DescribeParametersResult` | `InternalServerError`, `InvalidFilterKey`, `InvalidFilterOption`, `InvalidFilterValue`, `InvalidNextToken` | Lists the parameters in your Amazon Web Services account or the parameters shared with you when you enable the Shared option. Request results are returned on a best-effort basis. If you specify MaxResults in the requ ... |
| `DescribePatchBaselines` | `-` | `paginated` | - | - | `DescribePatchBaselinesResult` | `InternalServerError` | Lists the patch baselines in your Amazon Web Services account. |
| `DescribePatchGroups` | `-` | `paginated` | - | - | `DescribePatchGroupsResult` | `InternalServerError` | Lists all patch groups that have been registered with patch baselines. |
| `DescribePatchGroupState` | `-` | - | `PatchGroup` | - | `DescribePatchGroupStateResult` | `InternalServerError`, `InvalidNextToken` | Returns high-level aggregated patch compliance state information for a patch group. |
| `DescribePatchProperties` | `-` | `paginated` | `OperatingSystem`, `Property` | - | `DescribePatchPropertiesResult` | `InternalServerError` | Lists the properties of available patches organized by product, product family, classification, severity, and other properties of available patches. You can use the reported properties in the filters you specify in r ... |
| `DescribeSessions` | `-` | `paginated` | `State` | - | `DescribeSessionsResponse` | `InternalServerError`, `InvalidFilterKey`, `InvalidNextToken` | Retrieves a list of all active sessions (both connected and disconnected) or terminated sessions from the past 30 days. |
| `DisassociateOpsItemRelatedItem` | `-` | - | `OpsItemId`, `AssociationId` | - | `DisassociateOpsItemRelatedItemResponse` | `InternalServerError`, `OpsItemConflictException`, `OpsItemInvalidParameterException`, `OpsItemNotFoundException`, `OpsItemRelatedItemAssociationNotFoundException` | Deletes the association between an OpsItem and a related item. For example, this API operation can delete an Incident Manager incident from an OpsItem. Incident Manager is a tool in Amazon Web Services Systems Manager. |
| `GetAccessToken` | `-` | - | `AccessRequestId` | - | `GetAccessTokenResponse` | `AccessDeniedException`, `InternalServerError`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a credentials set to be used with just-in-time node access. |
| `GetAutomationExecution` | `-` | - | `AutomationExecutionId` | - | `GetAutomationExecutionResult` | `AutomationExecutionNotFoundException`, `InternalServerError` | Get detailed information about a particular Automation execution. |
| `GetCalendarState` | `-` | - | `CalendarNames` | - | `GetCalendarStateResponse` | `InternalServerError`, `InvalidDocument`, `InvalidDocumentType`, `UnsupportedCalendarException` | Gets the state of a Amazon Web Services Systems Manager change calendar at the current time or a specified time. If you specify a time, GetCalendarState returns the state of the calendar at that specific time, and re ... |
| `GetCommandInvocation` | `-` | - | `CommandId`, `InstanceId` | - | `GetCommandInvocationResult` | `InternalServerError`, `InvalidCommandId`, `InvalidInstanceId`, `InvalidPluginName`, `InvocationDoesNotExist` | Returns detailed information about command execution for an invocation or plugin. The Run Command API follows an eventual consistency model, due to the distributed nature of the system supporting the API. This means ... |
| `GetConnectionStatus` | `-` | - | `Target` | - | `GetConnectionStatusResponse` | `InternalServerError` | Retrieves the Session Manager connection status for a managed node to determine whether it is running and ready to receive Session Manager connections. |
| `GetDefaultPatchBaseline` | `-` | - | - | - | `GetDefaultPatchBaselineResult` | `InternalServerError` | Retrieves the default patch baseline. Amazon Web Services Systems Manager supports creating multiple default patch baselines. For example, you can create a default patch baseline for each operating system. If you don ... |
| `GetDeployablePatchSnapshotForInstance` | `-` | - | `InstanceId`, `SnapshotId` | - | `GetDeployablePatchSnapshotForInstanceResult` | `InternalServerError`, `UnsupportedFeatureRequiredException`, `UnsupportedOperatingSystem` | Retrieves the current snapshot for the patch baseline the managed node uses. This API is primarily used by the AWS-RunPatchBaseline Systems Manager document (SSM document). If you run the command locally, such as wit ... |
| `GetDocument` | `-` | - | `Name` | - | `GetDocumentResult` | `InternalServerError`, `InvalidDocument`, `InvalidDocumentVersion` | Gets the contents of the specified Amazon Web Services Systems Manager document (SSM document). |
| `GetExecutionPreview` | `-` | - | `ExecutionPreviewId` | - | `GetExecutionPreviewResponse` | `InternalServerError`, `ResourceNotFoundException` | Initiates the process of retrieving an existing preview that shows the effects that running a specified Automation runbook would have on the targeted resources. |
| `GetInventory` | `-` | `paginated` | - | - | `GetInventoryResult` | `InternalServerError`, `InvalidAggregatorException`, `InvalidFilter`, `InvalidInventoryGroupException`, `InvalidNextToken`, `InvalidResultAttributeException`, `InvalidTypeNameException` | Query inventory information. This includes managed node status, such as Stopped or Terminated . |
| `GetInventorySchema` | `-` | `paginated` | - | - | `GetInventorySchemaResult` | `InternalServerError`, `InvalidNextToken`, `InvalidTypeNameException` | Return a list of inventory type names for the account, or return a list of attribute names for a specific Inventory item type. |
| `GetMaintenanceWindow` | `-` | - | `WindowId` | - | `GetMaintenanceWindowResult` | `DoesNotExistException`, `InternalServerError` | Retrieves a maintenance window. |
| `GetMaintenanceWindowExecution` | `-` | - | `WindowExecutionId` | - | `GetMaintenanceWindowExecutionResult` | `DoesNotExistException`, `InternalServerError` | Retrieves details about a specific a maintenance window execution. |
| `GetMaintenanceWindowExecutionTask` | `-` | - | `WindowExecutionId`, `TaskId` | - | `GetMaintenanceWindowExecutionTaskResult` | `DoesNotExistException`, `InternalServerError` | Retrieves the details about a specific task run as part of a maintenance window execution. |
| `GetMaintenanceWindowExecutionTaskInvocation` | `-` | - | `WindowExecutionId`, `TaskId`, `InvocationId` | - | `GetMaintenanceWindowExecutionTaskInvocationResult` | `DoesNotExistException`, `InternalServerError` | Retrieves information about a specific task running on a specific target. |
| `GetMaintenanceWindowTask` | `-` | - | `WindowId`, `WindowTaskId` | - | `GetMaintenanceWindowTaskResult` | `DoesNotExistException`, `InternalServerError` | Retrieves the details of a maintenance window task. For maintenance window tasks without a specified target, you can't supply values for --max-errors and --max-concurrency . Instead, the system inserts a placeholder ... |
| `GetOpsItem` | `-` | - | `OpsItemId` | - | `GetOpsItemResponse` | `InternalServerError`, `OpsItemAccessDeniedException`, `OpsItemNotFoundException` | Get information about an OpsItem by using the ID. You must have permission in Identity and Access Management (IAM) to view information about an OpsItem. For more information, see Set up OpsCenter in the Amazon Web Se ... |
| `GetOpsMetadata` | `-` | - | `OpsMetadataArn` | - | `GetOpsMetadataResult` | `InternalServerError`, `OpsMetadataInvalidArgumentException`, `OpsMetadataNotFoundException` | View operational metadata related to an application in Application Manager. |
| `GetOpsSummary` | `-` | `paginated` | - | - | `GetOpsSummaryResult` | `InternalServerError`, `InvalidAggregatorException`, `InvalidFilter`, `InvalidNextToken`, `InvalidTypeNameException`, `ResourceDataSyncNotFoundException` | View a summary of operations metadata (OpsData) based on specified filters and aggregators. OpsData can include information about Amazon Web Services Systems Manager OpsCenter operational workitems (OpsItems) as well ... |
| `GetParameter` | `-` | - | `Name` | - | `GetParameterResult` | `InternalServerError`, `InvalidKeyId`, `ParameterNotFound`, `ParameterVersionNotFound` | Get information about a single parameter by specifying the parameter name. Parameter names can't contain spaces. The service removes any spaces specified for the beginning or end of a parameter name. If the specified ... |
| `GetParameterHistory` | `-` | `paginated` | `Name` | - | `GetParameterHistoryResult` | `InternalServerError`, `InvalidKeyId`, `InvalidNextToken`, `ParameterNotFound` | Retrieves the history of all changes to a parameter. Parameter names can't contain spaces. The service removes any spaces specified for the beginning or end of a parameter name. If the specified name for a parameter ... |
| `GetParameters` | `-` | - | `Names` | - | `GetParametersResult` | `InternalServerError`, `InvalidKeyId` | Get information about one or more parameters by specifying multiple parameter names. To get information about a single parameter, you can use the GetParameter operation instead. Parameter names can't contain spaces. ... |
| `GetParametersByPath` | `-` | `paginated` | `Path` | - | `GetParametersByPathResult` | `InternalServerError`, `InvalidFilterKey`, `InvalidFilterOption`, `InvalidFilterValue`, `InvalidKeyId`, `InvalidNextToken` | Retrieve information about one or more parameters under a specified level in a hierarchy. Request results are returned on a best-effort basis. If you specify MaxResults in the request, the response includes informati ... |
| `GetPatchBaseline` | `-` | - | `BaselineId` | - | `GetPatchBaselineResult` | `DoesNotExistException`, `InternalServerError`, `InvalidResourceId` | Retrieves information about a patch baseline. |
| `GetPatchBaselineForPatchGroup` | `-` | - | `PatchGroup` | - | `GetPatchBaselineForPatchGroupResult` | `InternalServerError` | Retrieves the patch baseline that should be used for the specified patch group. |
| `GetResourcePolicies` | `-` | `paginated` | `ResourceArn` | - | `GetResourcePoliciesResponse` | `InternalServerError`, `ResourceNotFoundException`, `ResourcePolicyInvalidParameterException` | Returns an array of the Policy object. |
| `GetServiceSetting` | `-` | - | `SettingId` | - | `GetServiceSettingResult` | `InternalServerError`, `ServiceSettingNotFound` | ServiceSetting is an account-level setting for an Amazon Web Services service. This setting defines how a user interacts with or uses a service or a feature of a service. For example, if an Amazon Web Services servic ... |
| `LabelParameterVersion` | `-` | - | `Name`, `Labels` | - | `LabelParameterVersionResult` | `InternalServerError`, `ParameterNotFound`, `ParameterVersionLabelLimitExceeded`, `ParameterVersionNotFound`, `TooManyUpdates` | A parameter label is a user-defined alias to help you manage different versions of a parameter. When you modify a parameter, Amazon Web Services Systems Manager automatically saves a new version and increments the ve ... |
| `ListAssociations` | `-` | `paginated` | - | - | `ListAssociationsResult` | `InternalServerError`, `InvalidNextToken` | Returns all State Manager associations in the current Amazon Web Services account and Amazon Web Services Region. You can limit the results to a specific State Manager association document or managed node by specifyi ... |
| `ListAssociationVersions` | `-` | `paginated` | `AssociationId` | - | `ListAssociationVersionsResult` | `AssociationDoesNotExist`, `InternalServerError`, `InvalidNextToken` | Retrieves all versions of an association for a specific association ID. |
| `ListCommandInvocations` | `-` | `paginated` | - | - | `ListCommandInvocationsResult` | `InternalServerError`, `InvalidCommandId`, `InvalidFilterKey`, `InvalidInstanceId`, `InvalidNextToken` | An invocation is copy of a command sent to a specific managed node. A command can apply to one or more managed nodes. A command invocation applies to one managed node. For example, if a user runs SendCommand against ... |
| `ListCommands` | `-` | `paginated` | - | - | `ListCommandsResult` | `InternalServerError`, `InvalidCommandId`, `InvalidFilterKey`, `InvalidInstanceId`, `InvalidNextToken` | Lists the commands requested by users of the Amazon Web Services account. |
| `ListComplianceItems` | `-` | `paginated` | - | - | `ListComplianceItemsResult` | `InternalServerError`, `InvalidFilter`, `InvalidNextToken`, `InvalidResourceId`, `InvalidResourceType` | For a specified resource ID, this API operation returns a list of compliance statuses for different resource types. Currently, you can only specify one resource ID per call. List results depend on the criteria specif ... |
| `ListComplianceSummaries` | `-` | `paginated` | - | - | `ListComplianceSummariesResult` | `InternalServerError`, `InvalidFilter`, `InvalidNextToken` | Returns a summary count of compliant and non-compliant resources for a compliance type. For example, this call can return State Manager associations, patches, or custom compliance types according to the filter criter ... |
| `ListDocumentMetadataHistory` | `-` | - | `Name`, `Metadata` | - | `ListDocumentMetadataHistoryResponse` | `InternalServerError`, `InvalidDocument`, `InvalidDocumentVersion`, `InvalidNextToken` | Amazon Web Services Systems Manager Change Manager is no longer open to new customers. Existing customers can continue to use the service as normal. For more information, see Amazon Web Services Systems Manager Chang ... |
| `ListDocuments` | `-` | `paginated` | - | - | `ListDocumentsResult` | `InternalServerError`, `InvalidFilterKey`, `InvalidNextToken` | Returns all Systems Manager (SSM) documents in the current Amazon Web Services account and Amazon Web Services Region. You can limit the results of this request by using a filter. |
| `ListDocumentVersions` | `-` | `paginated` | `Name` | - | `ListDocumentVersionsResult` | `InternalServerError`, `InvalidDocument`, `InvalidNextToken` | List all versions for a document. |
| `ListInventoryEntries` | `-` | - | `InstanceId`, `TypeName` | - | `ListInventoryEntriesResult` | `InternalServerError`, `InvalidFilter`, `InvalidInstanceId`, `InvalidNextToken`, `InvalidTypeNameException` | A list of inventory items returned by the request. |
| `ListNodes` | `-` | `paginated` | - | - | `ListNodesResult` | `InternalServerError`, `InvalidFilter`, `InvalidNextToken`, `ResourceDataSyncNotFoundException`, `UnsupportedOperationException` | Takes in filters and returns a list of managed nodes matching the filter criteria. |
| `ListNodesSummary` | `-` | `paginated` | `Aggregators` | - | `ListNodesSummaryResult` | `InternalServerError`, `InvalidAggregatorException`, `InvalidFilter`, `InvalidNextToken`, `ResourceDataSyncNotFoundException`, `UnsupportedOperationException` | Generates a summary of managed instance/node metadata based on the filters and aggregators you specify. Results are grouped by the input aggregator you specify. |
| `ListOpsItemEvents` | `-` | `paginated` | - | - | `ListOpsItemEventsResponse` | `InternalServerError`, `OpsItemInvalidParameterException`, `OpsItemLimitExceededException`, `OpsItemNotFoundException` | Returns a list of all OpsItem events in the current Amazon Web Services Region and Amazon Web Services account. You can limit the results to events associated with specific OpsItems by specifying a filter. |
| `ListOpsItemRelatedItems` | `-` | `paginated` | - | - | `ListOpsItemRelatedItemsResponse` | `InternalServerError`, `OpsItemInvalidParameterException` | Lists all related-item resources associated with a Systems Manager OpsCenter OpsItem. OpsCenter is a tool in Amazon Web Services Systems Manager. |
| `ListOpsMetadata` | `-` | `paginated` | - | - | `ListOpsMetadataResult` | `InternalServerError`, `OpsMetadataInvalidArgumentException` | Amazon Web Services Systems Manager calls this API operation when displaying all Application Manager OpsMetadata objects or blobs. |
| `ListResourceComplianceSummaries` | `-` | `paginated` | - | - | `ListResourceComplianceSummariesResult` | `InternalServerError`, `InvalidFilter`, `InvalidNextToken` | Returns a resource-level summary count. The summary includes information about compliant and non-compliant statuses and detailed compliance-item severity counts, according to the filter criteria you specify. |
| `ListResourceDataSync` | `-` | `paginated` | - | - | `ListResourceDataSyncResult` | `InternalServerError`, `InvalidNextToken`, `ResourceDataSyncInvalidConfigurationException` | Lists your resource data sync configurations. Includes information about the last time a sync attempted to start, the last sync status, and the last time a sync successfully completed. The number of sync configuratio ... |
| `ListTagsForResource` | `-` | - | `ResourceType`, `ResourceId` | - | `ListTagsForResourceResult` | `InternalServerError`, `InvalidResourceId`, `InvalidResourceType` | Returns a list of the tags assigned to the specified resource. For information about the ID format for each supported resource type, see AddTagsToResource . |
| `ModifyDocumentPermission` | `-` | - | `Name`, `PermissionType` | - | `ModifyDocumentPermissionResponse` | `DocumentLimitExceeded`, `DocumentPermissionLimit`, `InternalServerError`, `InvalidDocument`, `InvalidPermissionType` | Shares a Amazon Web Services Systems Manager document (SSM document)publicly or privately. If you share a document privately, you must specify the Amazon Web Services user IDs for those people who can use the documen ... |
| `PutComplianceItems` | `-` | - | `ResourceId`, `ResourceType`, `ComplianceType`, `ExecutionSummary`, `Items` | - | `PutComplianceItemsResult` | `ComplianceTypeCountLimitExceededException`, `InternalServerError`, `InvalidItemContentException`, `InvalidResourceId`, `InvalidResourceType`, `ItemSizeLimitExceededException`, `TotalSizeLimitExceededException` | Registers a compliance type and other compliance details on a designated resource. This operation lets you register custom compliance details with a resource. This call overwrites existing compliance information on t ... |
| `PutInventory` | `-` | - | `InstanceId`, `Items` | - | `PutInventoryResult` | `CustomSchemaCountLimitExceededException`, `InternalServerError`, `InvalidInstanceId`, `InvalidInventoryItemContextException`, `InvalidItemContentException`, `InvalidTypeNameException`, `ItemContentMismatchException`, `ItemSizeLimitExceededException`, `SubTypeCountLimitExceededException`, `TotalSizeLimitExceededException`, `UnsupportedInventoryItemContextException`, `UnsupportedInventorySchemaVersionException` | Bulk update custom inventory items on one or more managed nodes. The request adds an inventory item, if it doesn't already exist, or updates an inventory item, if it does exist. |
| `PutParameter` | `-` | - | `Name`, `Value` | - | `PutParameterResult` | `HierarchyLevelLimitExceededException`, `HierarchyTypeMismatchException`, `IncompatiblePolicyException`, `InternalServerError`, `InvalidAllowedPatternException`, `InvalidKeyId`, `InvalidPolicyAttributeException`, `InvalidPolicyTypeException`, `ParameterAlreadyExists`, `ParameterLimitExceeded`, `ParameterMaxVersionLimitExceeded`, `ParameterPatternMismatchException`, `PoliciesLimitExceededException`, `TooManyUpdates`, `UnsupportedParameterType` | Create or update a parameter in Parameter Store. |
| `PutResourcePolicy` | `-` | - | `ResourceArn`, `Policy` | - | `PutResourcePolicyResponse` | `InternalServerError`, `MalformedResourcePolicyDocumentException`, `ResourceNotFoundException`, `ResourcePolicyConflictException`, `ResourcePolicyInvalidParameterException`, `ResourcePolicyLimitExceededException`, `ResourcePolicyNotFoundException` | Creates or updates a Systems Manager resource policy. A resource policy helps you to define the IAM entity (for example, an Amazon Web Services account) that can manage your Systems Manager resources. The following r ... |
| `RegisterDefaultPatchBaseline` | `-` | - | `BaselineId` | - | `RegisterDefaultPatchBaselineResult` | `DoesNotExistException`, `InternalServerError`, `InvalidResourceId` | Defines the default patch baseline for the relevant operating system. To reset the Amazon Web Services-predefined patch baseline as the default, specify the full patch baseline Amazon Resource Name (ARN) as the basel ... |
| `RegisterPatchBaselineForPatchGroup` | `-` | - | `BaselineId`, `PatchGroup` | - | `RegisterPatchBaselineForPatchGroupResult` | `AlreadyExistsException`, `DoesNotExistException`, `InternalServerError`, `InvalidResourceId`, `ResourceLimitExceededException` | Registers a patch baseline for a patch group. |
| `RegisterTargetWithMaintenanceWindow` | `-` | `idempotency-token` | `WindowId`, `ResourceType`, `Targets` | `ClientToken` | `RegisterTargetWithMaintenanceWindowResult` | `DoesNotExistException`, `IdempotentParameterMismatch`, `InternalServerError`, `ResourceLimitExceededException` | Registers a target with a maintenance window. |
| `RegisterTaskWithMaintenanceWindow` | `-` | `idempotency-token` | `WindowId`, `TaskArn`, `TaskType` | `ClientToken` | `RegisterTaskWithMaintenanceWindowResult` | `DoesNotExistException`, `FeatureNotAvailableException`, `IdempotentParameterMismatch`, `InternalServerError`, `ResourceLimitExceededException` | Adds a new task to a maintenance window. |
| `RemoveTagsFromResource` | `-` | - | `ResourceType`, `ResourceId`, `TagKeys` | - | `RemoveTagsFromResourceResult` | `InternalServerError`, `InvalidResourceId`, `InvalidResourceType`, `TooManyUpdates` | Removes tag keys from the specified resource. |
| `ResetServiceSetting` | `-` | - | `SettingId` | - | `ResetServiceSettingResult` | `InternalServerError`, `ServiceSettingNotFound`, `TooManyUpdates` | ServiceSetting is an account-level setting for an Amazon Web Services service. This setting defines how a user interacts with or uses a service or a feature of a service. For example, if an Amazon Web Services servic ... |
| `ResumeSession` | `-` | - | `SessionId` | - | `ResumeSessionResponse` | `DoesNotExistException`, `InternalServerError` | Reconnects a session to a managed node after it has been disconnected. Connections can be resumed for disconnected sessions, but not terminated sessions. This command is primarily for use by client machines to automa ... |
| `SendAutomationSignal` | `-` | - | `AutomationExecutionId`, `SignalType` | - | `SendAutomationSignalResult` | `AutomationExecutionNotFoundException`, `AutomationStepNotFoundException`, `InternalServerError`, `InvalidAutomationSignalException` | Sends a signal to an Automation execution to change the current behavior or status of the execution. |
| `SendCommand` | `-` | - | `DocumentName` | - | `SendCommandResult` | `DuplicateInstanceId`, `InternalServerError`, `InvalidDocument`, `InvalidDocumentVersion`, `InvalidInstanceId`, `InvalidNotificationConfig`, `InvalidOutputFolder`, `InvalidParameters`, `InvalidRole`, `MaxDocumentSizeExceeded`, `UnsupportedPlatformType` | Runs commands on one or more managed nodes. |
| `StartAccessRequest` | `-` | - | `Reason`, `Targets` | - | `StartAccessRequestResponse` | `AccessDeniedException`, `InternalServerError`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Starts the workflow for just-in-time node access sessions. |
| `StartAssociationsOnce` | `-` | - | `AssociationIds` | - | `StartAssociationsOnceResult` | `AssociationDoesNotExist`, `InvalidAssociation` | Runs an association immediately and only one time. This operation can be helpful when troubleshooting associations. |
| `StartAutomationExecution` | `-` | - | `DocumentName` | - | `StartAutomationExecutionResult` | `AutomationDefinitionNotFoundException`, `AutomationDefinitionVersionNotFoundException`, `AutomationExecutionLimitExceededException`, `IdempotentParameterMismatch`, `InternalServerError`, `InvalidAutomationExecutionParametersException`, `InvalidTarget` | Initiates execution of an Automation runbook. |
| `StartChangeRequestExecution` | `-` | - | `DocumentName`, `Runbooks` | - | `StartChangeRequestExecutionResult` | `AutomationDefinitionNotApprovedException`, `AutomationDefinitionNotFoundException`, `AutomationDefinitionVersionNotFoundException`, `AutomationExecutionLimitExceededException`, `IdempotentParameterMismatch`, `InternalServerError`, `InvalidAutomationExecutionParametersException`, `NoLongerSupportedException` | Amazon Web Services Systems Manager Change Manager is no longer open to new customers. Existing customers can continue to use the service as normal. For more information, see Amazon Web Services Systems Manager Chang ... |
| `StartExecutionPreview` | `-` | - | `DocumentName` | - | `StartExecutionPreviewResponse` | `InternalServerError`, `ValidationException` | Initiates the process of creating a preview showing the effects that running a specified Automation runbook would have on the targeted resources. |
| `StartSession` | `-` | - | `Target` | - | `StartSessionResponse` | `InternalServerError`, `InvalidDocument`, `TargetNotConnected` | Initiates a connection to a target (for example, a managed node) for a Session Manager session. Returns a URL and token that can be used to open a WebSocket connection for sending input and receiving outputs. Amazon ... |
| `StopAutomationExecution` | `-` | - | `AutomationExecutionId` | - | `StopAutomationExecutionResult` | `AutomationExecutionNotFoundException`, `InternalServerError`, `InvalidAutomationStatusUpdateException` | Stop an Automation that is currently running. |
| `TerminateSession` | `-` | - | `SessionId` | - | `TerminateSessionResponse` | `InternalServerError` | Permanently ends a session and closes the data connection between the Session Manager client and SSM Agent on the managed node. A terminated session can't be resumed. |
| `UnlabelParameterVersion` | `-` | - | `Name`, `ParameterVersion`, `Labels` | - | `UnlabelParameterVersionResult` | `InternalServerError`, `ParameterNotFound`, `ParameterVersionNotFound`, `TooManyUpdates` | Remove a label or labels from a parameter. Parameter names can't contain spaces. The service removes any spaces specified for the beginning or end of a parameter name. If the specified name for a parameter contains s ... |
| `UpdateAssociation` | `-` | - | `AssociationId` | - | `UpdateAssociationResult` | `AssociationDoesNotExist`, `AssociationVersionLimitExceeded`, `InternalServerError`, `InvalidAssociationVersion`, `InvalidDocument`, `InvalidDocumentVersion`, `InvalidOutputLocation`, `InvalidParameters`, `InvalidSchedule`, `InvalidTarget`, `InvalidTargetMaps`, `InvalidUpdate`, `TooManyUpdates` | Updates an association. You can update the association name and version, the document version, schedule, parameters, and Amazon Simple Storage Service (Amazon S3) output. When you call UpdateAssociation , the system ... |
| `UpdateAssociationStatus` | `-` | - | `Name`, `InstanceId`, `AssociationStatus` | - | `UpdateAssociationStatusResult` | `AssociationDoesNotExist`, `InternalServerError`, `InvalidDocument`, `InvalidInstanceId`, `StatusUnchanged`, `TooManyUpdates` | Updates the status of the Amazon Web Services Systems Manager document (SSM document) associated with the specified managed node. UpdateAssociationStatus is primarily used by the Amazon Web Services Systems Manager A ... |
| `UpdateDocument` | `-` | - | `Content`, `Name` | - | `UpdateDocumentResult` | `DocumentVersionLimitExceeded`, `DuplicateDocumentContent`, `DuplicateDocumentVersionName`, `InternalServerError`, `InvalidDocument`, `InvalidDocumentContent`, `InvalidDocumentOperation`, `InvalidDocumentSchemaVersion`, `InvalidDocumentVersion`, `MaxDocumentSizeExceeded` | Updates one or more values for an SSM document. |
| `UpdateDocumentDefaultVersion` | `-` | - | `Name`, `DocumentVersion` | - | `UpdateDocumentDefaultVersionResult` | `InternalServerError`, `InvalidDocument`, `InvalidDocumentSchemaVersion`, `InvalidDocumentVersion` | Set the default version of a document. If you change a document version for a State Manager association, Systems Manager immediately runs the association unless you previously specifed the apply-only-at-cron-interval ... |
| `UpdateDocumentMetadata` | `-` | - | `Name`, `DocumentReviews` | - | `UpdateDocumentMetadataResponse` | `InternalServerError`, `InvalidDocument`, `InvalidDocumentOperation`, `InvalidDocumentVersion`, `TooManyUpdates` | Amazon Web Services Systems Manager Change Manager is no longer open to new customers. Existing customers can continue to use the service as normal. For more information, see Amazon Web Services Systems Manager Chang ... |
| `UpdateMaintenanceWindow` | `-` | - | `WindowId` | - | `UpdateMaintenanceWindowResult` | `DoesNotExistException`, `InternalServerError` | Updates an existing maintenance window. Only specified parameters are modified. The value you specify for Duration determines the specific end time for the maintenance window based on the time it begins. No maintenan ... |
| `UpdateMaintenanceWindowTarget` | `-` | - | `WindowId`, `WindowTargetId` | - | `UpdateMaintenanceWindowTargetResult` | `DoesNotExistException`, `InternalServerError` | Modifies the target of an existing maintenance window. You can change the following: Name Description Owner IDs for an ID target Tags for a Tag target From any supported tag type to another. The three supported tag t ... |
| `UpdateMaintenanceWindowTask` | `-` | - | `WindowId`, `WindowTaskId` | - | `UpdateMaintenanceWindowTaskResult` | `DoesNotExistException`, `InternalServerError` | Modifies a task assigned to a maintenance window. You can't change the task type, but you can change the following values: TaskARN . For example, you can change a RUN_COMMAND task from AWS-RunPowerShellScript to AWS- ... |
| `UpdateManagedInstanceRole` | `-` | - | `InstanceId`, `IamRole` | - | `UpdateManagedInstanceRoleResult` | `InternalServerError`, `InvalidInstanceId` | Changes the Identity and Access Management (IAM) role that is assigned to the on-premises server, edge device, or virtual machines (VM). IAM roles are first assigned to these hybrid nodes during the activation proces ... |
| `UpdateOpsItem` | `-` | - | `OpsItemId` | - | `UpdateOpsItemResponse` | `InternalServerError`, `OpsItemAccessDeniedException`, `OpsItemAlreadyExistsException`, `OpsItemConflictException`, `OpsItemInvalidParameterException`, `OpsItemLimitExceededException`, `OpsItemNotFoundException` | Edit or change an OpsItem. You must have permission in Identity and Access Management (IAM) to update an OpsItem. For more information, see Set up OpsCenter in the Amazon Web Services Systems Manager User Guide . Ope ... |
| `UpdateOpsMetadata` | `-` | - | `OpsMetadataArn` | - | `UpdateOpsMetadataResult` | `InternalServerError`, `OpsMetadataInvalidArgumentException`, `OpsMetadataKeyLimitExceededException`, `OpsMetadataNotFoundException`, `OpsMetadataTooManyUpdatesException` | Amazon Web Services Systems Manager calls this API operation when you edit OpsMetadata in Application Manager. |
| `UpdatePatchBaseline` | `-` | - | `BaselineId` | - | `UpdatePatchBaselineResult` | `DoesNotExistException`, `InternalServerError` | Modifies an existing patch baseline. Fields not specified in the request are left unchanged. For information about valid key-value pairs in PatchFilters for each supported operating system type, see PatchFilter . |
| `UpdateResourceDataSync` | `-` | - | `SyncName`, `SyncType`, `SyncSource` | - | `UpdateResourceDataSyncResult` | `InternalServerError`, `ResourceDataSyncConflictException`, `ResourceDataSyncInvalidConfigurationException`, `ResourceDataSyncNotFoundException` | Update a resource data sync. After you create a resource data sync for a Region, you can't change the account options for that sync. For example, if you create a sync in the us-east-2 (Ohio) Region and you choose the ... |
| `UpdateServiceSetting` | `-` | - | `SettingId`, `SettingValue` | - | `UpdateServiceSettingResult` | `InternalServerError`, `ServiceSettingNotFound`, `TooManyUpdates` | ServiceSetting is an account-level setting for an Amazon Web Services service. This setting defines how a user interacts with or uses a service or a feature of a service. For example, if an Amazon Web Services servic ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | The requester doesn't have permissions to perform the requested operation. |
| `AlreadyExistsException` | `structure` | Message | Error returned if an attempt is made to register a patch group with a patch baseline that is already registered with a different patch baseline. |
| `AssociatedInstances` | `structure` | **empty (no members)** | You must disassociate a document from all managed nodes before you can delete it. |
| `AssociationAlreadyExists` | `structure` | **empty (no members)** | The specified association already exists. |
| `AssociationDoesNotExist` | `structure` | Message | The specified association doesn't exist. |
| `AssociationExecutionDoesNotExist` | `structure` | Message | The specified execution ID doesn't exist. Verify the ID number and try again. |
| `AssociationLimitExceeded` | `structure` | **empty (no members)** | You can have at most 2,000 active associations. |
| `AssociationVersionLimitExceeded` | `structure` | Message | You have reached the maximum number versions allowed for an association. Each association has a limit of 1,000 versions. |
| `AutomationDefinitionNotApprovedException` | `structure` | Message | Indicates that the Change Manager change template used in the change request was rejected or is still in a pending state. |
| `AutomationDefinitionNotFoundException` | `structure` | Message | An Automation runbook with the specified name couldn't be found. |
| `AutomationDefinitionVersionNotFoundException` | `structure` | Message | An Automation runbook with the specified name and version couldn't be found. |
| `AutomationExecutionLimitExceededException` | `structure` | Message | The number of simultaneously running Automation executions exceeded the allowable limit. |
| `AutomationExecutionNotFoundException` | `structure` | Message | There is no automation execution information for the requested automation execution ID. |
| `AutomationStepNotFoundException` | `structure` | Message | The specified step name and execution ID don't exist. Verify the information and try again. |
| `ComplianceTypeCountLimitExceededException` | `structure` | Message | You specified too many custom compliance types. You can specify a maximum of 10 different types. |
| `CustomSchemaCountLimitExceededException` | `structure` | Message | You have exceeded the limit for custom schemas. Delete one or more custom schemas and try again. |
| `DocumentAlreadyExists` | `structure` | Message | The specified document already exists. |
| `DocumentLimitExceeded` | `structure` | Message | You can have at most 500 active SSM documents. |
| `DocumentPermissionLimit` | `structure` | Message | The document can't be shared with more Amazon Web Services accounts. You can specify a maximum of 20 accounts per API operation to share a private document. ... |
| `DocumentVersionLimitExceeded` | `structure` | Message | The document has too many versions. Delete one or more document versions and try again. |
| `DoesNotExistException` | `structure` | Message | Error returned when the ID specified for a resource, such as a maintenance window or patch baseline, doesn't exist. For information about resource quotas in ... |
| `DuplicateDocumentContent` | `structure` | Message | The content of the association document matches another document. Change the content of the document and try again. |
| `DuplicateDocumentVersionName` | `structure` | Message | The version name has already been used in this document. Specify a different version name, and then try again. |
| `DuplicateInstanceId` | `structure` | **empty (no members)** | You can't specify a managed node ID in more than one association. |
| `FeatureNotAvailableException` | `structure` | Message | You attempted to register a LAMBDA or STEP_FUNCTIONS task in a region where the corresponding service isn't available. |
| `HierarchyLevelLimitExceededException` | `structure` | message | A hierarchy can have a maximum of 15 levels. For more information, see Requirements and constraints for parameter names in the Amazon Web Services Systems M ... |
| `HierarchyTypeMismatchException` | `structure` | message | Parameter Store doesn't support changing a parameter type in a hierarchy. For example, you can't change a parameter from a String type to a SecureString typ ... |
| `IdempotentParameterMismatch` | `structure` | Message | Error returned when an idempotent operation is retried and the parameters don't match the original call to the API with the same idempotency token. |
| `IncompatiblePolicyException` | `structure` | message | There is a conflict in the policies specified for this parameter. You can't, for example, specify two Expiration policies for a parameter. Review your polic ... |
| `InternalServerError` | `structure` | Message | An error occurred on the server side. |
| `InvalidActivation` | `structure` | Message | The activation isn't valid. The activation might have been deleted, or the ActivationId and the ActivationCode don't match. |
| `InvalidActivationId` | `structure` | Message | The activation ID isn't valid. Verify that you entered the correct ActivationId or ActivationCode and try again. |
| `InvalidAggregatorException` | `structure` | Message | The specified aggregator isn't valid for the group type. Verify that the aggregator you provided is supported. |
| `InvalidAllowedPatternException` | `structure` | message | The request doesn't meet the regular expression requirement. |
| `InvalidAssociation` | `structure` | Message | The association isn't valid or doesn't exist. |
| `InvalidAssociationVersion` | `structure` | Message | The version you specified isn't valid. Use ListAssociationVersions to view all versions of an association according to the association ID. Or, use the $LATE ... |
| `InvalidAutomationExecutionParametersException` | `structure` | Message | The supplied parameters for invoking the specified Automation runbook are incorrect. For example, they may not match the set of parameters permitted for the ... |
| `InvalidAutomationSignalException` | `structure` | Message | The signal isn't valid for the current Automation execution. |
| `InvalidAutomationStatusUpdateException` | `structure` | Message | The specified update status operation isn't valid. |
| `InvalidCommandId` | `structure` | **empty (no members)** | The specified command ID isn't valid. Verify the ID and try again. |
| `InvalidDeleteInventoryParametersException` | `structure` | Message | One or more of the parameters specified for the delete operation isn't valid. Verify all parameters and try again. |
| `InvalidDeletionIdException` | `structure` | Message | The ID specified for the delete operation doesn't exist or isn't valid. Verify the ID and try again. |
| `InvalidDocument` | `structure` | Message | The specified SSM document doesn't exist. |
| `InvalidDocumentContent` | `structure` | Message | The content for the document isn't valid. |
| `InvalidDocumentOperation` | `structure` | Message | You attempted to delete a document while it is still shared. You must stop sharing the document before you can delete it. |
| `InvalidDocumentSchemaVersion` | `structure` | Message | The version of the document schema isn't supported. |
| `InvalidDocumentType` | `structure` | Message | The SSM document type isn't valid. Valid document types are described in the DocumentType property. |
| `InvalidDocumentVersion` | `structure` | Message | The document version isn't valid or doesn't exist. |
| `InvalidFilter` | `structure` | Message | The filter name isn't valid. Verify that you entered the correct name and try again. |
| `InvalidFilterKey` | `structure` | **empty (no members)** | The specified key isn't valid. |
| `InvalidFilterOption` | `structure` | message | The specified filter option isn't valid. Valid options are Equals and BeginsWith. For Path filter, valid options are Recursive and OneLevel. |
| `InvalidFilterValue` | `structure` | Message | The filter value isn't valid. Verify the value and try again. |
| `InvalidInstanceId` | `structure` | Message | The following problems can cause this exception: You don't have permission to access the managed node. Amazon Web Services Systems Manager Agent (SSM Agent) ... |
| `InvalidInstanceInformationFilterValue` | `structure` | message | The specified filter value isn't valid. |
| `InvalidInstancePropertyFilterValue` | `structure` | message | The specified filter value isn't valid. |
| `InvalidInventoryGroupException` | `structure` | Message | The specified inventory group isn't valid. |
| `InvalidInventoryItemContextException` | `structure` | Message | You specified invalid keys or values in the Context attribute for InventoryItem . Verify the keys and values, and try again. |
| `InvalidInventoryRequestException` | `structure` | Message | The request isn't valid. |
| `InvalidItemContentException` | `structure` | TypeName, Message | One or more content items isn't valid. |
| `InvalidKeyId` | `structure` | message | The query key ID isn't valid. |
| `InvalidNextToken` | `structure` | Message | The specified token isn't valid. |
| `InvalidNotificationConfig` | `structure` | Message | One or more configuration items isn't valid. Verify that a valid Amazon Resource Name (ARN) was provided for an Amazon Simple Notification Service topic. |
| `InvalidOptionException` | `structure` | Message | The delete inventory option specified isn't valid. Verify the option and try again. |
| `InvalidOutputFolder` | `structure` | **empty (no members)** | The S3 bucket doesn't exist. |
| `InvalidOutputLocation` | `structure` | **empty (no members)** | The output location isn't valid or doesn't exist. |
| `InvalidParameters` | `structure` | Message | You must specify values for all required parameters in the Amazon Web Services Systems Manager document (SSM document). You can only supply values to parame ... |
| `InvalidPermissionType` | `structure` | Message | The permission type isn't supported. Share is the only supported permission type. |
| `InvalidPluginName` | `structure` | **empty (no members)** | The plugin name isn't valid. |
| `InvalidPolicyAttributeException` | `structure` | message | A policy attribute or its value is invalid. |
| `InvalidPolicyTypeException` | `structure` | message | The policy type isn't supported. Parameter Store supports the following policy types: Expiration, ExpirationNotification, and NoChangeNotification. |
| `InvalidResourceId` | `structure` | **empty (no members)** | The resource ID isn't valid. Verify that you entered the correct ID and try again. |
| `InvalidResourceType` | `structure` | **empty (no members)** | The resource type isn't valid. For example, if you are attempting to tag an EC2 instance, the instance must be a registered managed node. |
| `InvalidResultAttributeException` | `structure` | Message | The specified inventory item result attribute isn't valid. |
| `InvalidRole` | `structure` | Message | The role name can't contain invalid characters. Also verify that you specified an IAM role for notifications that includes the required trust policy. For in ... |
| `InvalidSchedule` | `structure` | Message | The schedule is invalid. Verify your cron or rate expression and try again. |
| `InvalidTag` | `structure` | Message | The specified tag key or value isn't valid. |
| `InvalidTarget` | `structure` | Message | The target isn't valid or doesn't exist. It might not be configured for Systems Manager or you might not have permission to perform the operation. |
| `InvalidTargetMaps` | `structure` | Message | TargetMap parameter isn't valid. |
| `InvalidTypeNameException` | `structure` | Message | The parameter type name isn't valid. |
| `InvalidUpdate` | `structure` | Message | The update isn't valid. |
| `InvocationDoesNotExist` | `structure` | **empty (no members)** | The command ID and managed node ID you specified didn't match any invocations. Verify the command ID and the managed node ID and try again. |
| `ItemContentMismatchException` | `structure` | TypeName, Message | The inventory item has invalid content. |
| `ItemSizeLimitExceededException` | `structure` | TypeName, Message | The inventory item size has exceeded the size limit. |
| `MalformedResourcePolicyDocumentException` | `structure` | Message | The specified policy document is malformed or invalid, or excessive PutResourcePolicy or DeleteResourcePolicy calls have been made. |
| `MaxDocumentSizeExceeded` | `structure` | Message | The size limit of a document is 64 KB. |
| `NoLongerSupportedException` | `structure` | Message | The requested operation is no longer supported by Systems Manager. |
| `OpsItemAccessDeniedException` | `structure` | Message | You don't have permission to view OpsItems in the specified account. Verify that your account is configured either as a Systems Manager delegated administra ... |
| `OpsItemAlreadyExistsException` | `structure` | Message, OpsItemId | The OpsItem already exists. |
| `OpsItemConflictException` | `structure` | Message | The specified OpsItem is in the process of being deleted. |
| `OpsItemInvalidParameterException` | `structure` | ParameterNames, Message | A specified parameter argument isn't valid. Verify the available arguments and try again. |
| `OpsItemLimitExceededException` | `structure` | ResourceTypes, Limit, LimitType, Message | The request caused OpsItems to exceed one or more quotas. |
| `OpsItemNotFoundException` | `structure` | Message | The specified OpsItem ID doesn't exist. Verify the ID and try again. |
| `OpsItemRelatedItemAlreadyExistsException` | `structure` | Message, ResourceUri, OpsItemId | The Amazon Resource Name (ARN) is already associated with the OpsItem. |
| `OpsItemRelatedItemAssociationNotFoundException` | `structure` | Message | The association wasn't found using the parameters you specified in the call. Verify the information and try again. |
| `OpsMetadataAlreadyExistsException` | `structure` | message | An OpsMetadata object already exists for the selected resource. |
| `OpsMetadataInvalidArgumentException` | `structure` | message | One of the arguments passed is invalid. |
| `OpsMetadataKeyLimitExceededException` | `structure` | message | The OpsMetadata object exceeds the maximum number of OpsMetadata keys that you can assign to an application in Application Manager. |
| `OpsMetadataLimitExceededException` | `structure` | message | Your account reached the maximum number of OpsMetadata objects allowed by Application Manager. The maximum is 200 OpsMetadata objects. Delete one or more Op ... |
| `OpsMetadataNotFoundException` | `structure` | message | The OpsMetadata object doesn't exist. |
| `OpsMetadataTooManyUpdatesException` | `structure` | message | The system is processing too many concurrent updates. Wait a few moments and try again. |
| `ParameterAlreadyExists` | `structure` | message | The parameter already exists. You can't create duplicate parameters. |
| `ParameterLimitExceeded` | `structure` | message | You have exceeded the number of parameters for this Amazon Web Services account. Delete one or more parameters and try again. |
| `ParameterMaxVersionLimitExceeded` | `structure` | message | Parameter Store retains the 100 most recently created versions of a parameter. After this number of versions has been created, Parameter Store deletes the o ... |
| `ParameterNotFound` | `structure` | message | The parameter couldn't be found. Verify the name and try again. For the DeleteParameter and GetParameter actions, if the specified parameter doesn't exist, ... |
| `ParameterPatternMismatchException` | `structure` | message | The parameter name isn't valid. |
| `ParameterVersionLabelLimitExceeded` | `structure` | message | A parameter version can have a maximum of ten labels. |
| `ParameterVersionNotFound` | `structure` | message | The specified parameter version wasn't found. Verify the parameter name and version, and try again. |
| `PoliciesLimitExceededException` | `structure` | message | You specified more than the maximum number of allowed policies for the parameter. The maximum is 10. |
| `ResourceDataSyncAlreadyExistsException` | `structure` | SyncName | A sync configuration with the same name already exists. |
| `ResourceDataSyncConflictException` | `structure` | Message | Another UpdateResourceDataSync request is being processed. Wait a few minutes and try again. |
| `ResourceDataSyncCountExceededException` | `structure` | Message | You have exceeded the allowed maximum sync configurations. |
| `ResourceDataSyncInvalidConfigurationException` | `structure` | Message | The specified sync configuration is invalid. |
| `ResourceDataSyncNotFoundException` | `structure` | SyncName, SyncType, Message | The specified sync name wasn't found. |
| `ResourceInUseException` | `structure` | Message | Error returned if an attempt is made to delete a patch baseline that is registered for a patch group. |
| `ResourceLimitExceededException` | `structure` | Message | Error returned when the caller has exceeded the default resource quotas. For example, too many maintenance windows or patch baselines have been created. For ... |
| `ResourceNotFoundException` | `structure` | Message | The specified parameter to be shared could not be found. |
| `ResourcePolicyConflictException` | `structure` | Message | The hash provided in the call doesn't match the stored hash. This exception is thrown when trying to update an obsolete policy version or when multiple requ ... |
| `ResourcePolicyInvalidParameterException` | `structure` | ParameterNames, Message | One or more parameters specified for the call aren't valid. Verify the parameters and their values and try again. |
| `ResourcePolicyLimitExceededException` | `structure` | Limit, LimitType, Message | The PutResourcePolicy API action enforces two limits. A policy can't be greater than 1024 bytes in size. And only one policy can be attached to OpsItemGroup ... |
| `ResourcePolicyNotFoundException` | `structure` | Message | No policies with the specified policy ID and hash could be found. |
| `ServiceQuotaExceededException` | `structure` | Message, ResourceId, ResourceType, QuotaCode, ServiceCode | The request exceeds the service quota. Service quotas, also referred to as limits, are the maximum number of service resources or operations for your Amazon ... |
| `ServiceSettingNotFound` | `structure` | Message | The specified service setting wasn't found. Either the service name or the setting hasn't been provisioned by the Amazon Web Services service team. |
| `StatusUnchanged` | `structure` | **empty (no members)** | The updated status is the same as the current status. |
| `SubTypeCountLimitExceededException` | `structure` | Message | The sub-type count exceeded the limit for the inventory type. |
| `TargetInUseException` | `structure` | Message | You specified the Safe option for the DeregisterTargetFromMaintenanceWindow operation, but the target is still referenced in a task. |
| `TargetNotConnected` | `structure` | Message | The specified target managed node for the session isn't fully configured for use with Session Manager. For more information, see Setting up Session Manager ... |
| `ThrottlingException` | `structure` | Message, QuotaCode, ServiceCode | The request or operation couldn't be performed because the service is throttling requests. |
| `TooManyTagsError` | `structure` | **empty (no members)** | The Targets parameter includes too many tags. Remove one or more tags and try the command again. |
| `TooManyUpdates` | `structure` | Message | There are concurrent updates for a resource that supports one update at a time. |
| `TotalSizeLimitExceededException` | `structure` | Message | The size of inventory data has exceeded the total size limit for the resource. |
| `UnsupportedCalendarException` | `structure` | Message | The calendar entry contained in the specified SSM document isn't supported. |
| `UnsupportedFeatureRequiredException` | `structure` | Message | Patching for applications released by Microsoft is only available on EC2 instances and advanced instances. To patch applications released by Microsoft on on ... |
| `UnsupportedInventoryItemContextException` | `structure` | TypeName, Message | The Context attribute that you specified for the InventoryItem isn't allowed for this inventory type. You can only use the Context attribute with inventory ... |
| `UnsupportedInventorySchemaVersionException` | `structure` | Message | Inventory item type schema version has to match supported versions in the service. Check output of GetInventorySchema to see the available schema version fo ... |
| `UnsupportedOperatingSystem` | `structure` | Message | The operating systems you specified isn't supported, or the operation isn't supported for the operating system. |
| `UnsupportedOperationException` | `structure` | Message | This operation is not supported for the current account. You must first enable the Systems Manager integrated experience in your account. |
| `UnsupportedParameterType` | `structure` | message | The parameter type isn't supported. |
| `UnsupportedPlatformType` | `structure` | Message | The document doesn't support the platform type of the given managed node IDs. For example, you sent an document for a Windows managed node to a Linux node. |
| `ValidationException` | `structure` | Message, ReasonCode | The request isn't valid. Verify that you entered valid contents for the command and try again. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
