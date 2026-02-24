# Amazon Inspector

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Inspector Amazon Inspector enables you to analyze the behavior of your AWS resources and to identify potential security issues. For more information, see Amazon Inspector User Guide.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Inspector where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Amazon Inspector by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for Amazon Inspector resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Inspector workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Describe`, `Create`, `Delete`, `Get` operation families, including `ListAssessmentRunAgents`, `ListAssessmentRuns`, `ListAssessmentTargets`, `ListAssessmentTemplates`, `DescribeAssessmentRuns`, `DescribeAssessmentTargets`.

## Service Identity and Protocol

- AWS model slug: `inspector`
- AWS SDK for Rust slug: `inspector`
- Model version: `2016-02-16`
- Model file: `vendor/api-models-aws/models/inspector/service/2016-02-16/inspector-2016-02-16.json`
- SDK ID: `Inspector`
- Endpoint prefix: `inspector`
- ARN namespace: `inspector`
- CloudFormation name: `Inspector`
- CloudTrail event source: `inspector.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (9), `Describe` (8), `Create` (4), `Delete` (3), `Get` (3), `Add` (1), `Preview` (1), `Register` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddAttributesToFindings`, `CreateAssessmentTarget`, `CreateAssessmentTemplate`, `CreateExclusionsPreview`, `CreateResourceGroup`, `DeleteAssessmentRun`, `DeleteAssessmentTarget`, `DeleteAssessmentTemplate`, `RegisterCrossAccountAccessRole`, `RemoveAttributesFromFindings`, `SetTagsForResource`, `StartAssessmentRun`, `StopAssessmentRun`, `UpdateAssessmentTarget`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAssessmentRuns`, `DescribeAssessmentTargets`, `DescribeAssessmentTemplates`, `DescribeCrossAccountAccessRole`, `DescribeExclusions`, `DescribeFindings`, `DescribeResourceGroups`, `DescribeRulesPackages`, `GetAssessmentReport`, `GetExclusionsPreview`, `GetTelemetryMetadata`, `ListAssessmentRunAgents`, `ListAssessmentRuns`, `ListAssessmentTargets`, `ListAssessmentTemplates`, `ListEventSubscriptions`, `ListExclusions`, `ListFindings`, `ListRulesPackages`, `ListTagsForResource`.
- Pagination is modelled for 10 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetAssessmentReport`, `StartAssessmentRun`, `StopAssessmentRun`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 37 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `SNS`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListAssessmentRunAgents`, `ListAssessmentRuns`, `ListAssessmentTargets`, `ListAssessmentTemplates`, `ListEventSubscriptions`, `ListExclusions`, `ListFindings`, `ListRulesPackages`, `ListTagsForResource`
- Traits: `paginated` (8)
- Common required input members in this group: `assessmentRunArn`, `resourceArn`

### Describe

- Operations: `DescribeAssessmentRuns`, `DescribeAssessmentTargets`, `DescribeAssessmentTemplates`, `DescribeCrossAccountAccessRole`, `DescribeExclusions`, `DescribeFindings`, `DescribeResourceGroups`, `DescribeRulesPackages`
- Common required input members in this group: `assessmentRunArns`, `assessmentTargetArns`, `assessmentTemplateArns`, `exclusionArns`, `findingArns`, `resourceGroupArns`, `rulesPackageArns`

### Create

- Operations: `CreateAssessmentTarget`, `CreateAssessmentTemplate`, `CreateExclusionsPreview`, `CreateResourceGroup`
- Common required input members in this group: `assessmentTargetArn`, `assessmentTargetName`, `assessmentTemplateArn`, `assessmentTemplateName`, `durationInSeconds`, `resourceGroupTags`, `rulesPackageArns`

### Delete

- Operations: `DeleteAssessmentRun`, `DeleteAssessmentTarget`, `DeleteAssessmentTemplate`
- Common required input members in this group: `assessmentRunArn`, `assessmentTargetArn`, `assessmentTemplateArn`

### Get

- Operations: `GetAssessmentReport`, `GetExclusionsPreview`, `GetTelemetryMetadata`
- Traits: `paginated` (1)
- Common required input members in this group: `assessmentRunArn`, `assessmentTemplateArn`, `previewToken`, `reportFileFormat`, `reportType`

### Add

- Operations: `AddAttributesToFindings`
- Common required input members in this group: `attributes`, `findingArns`

### Preview

- Operations: `PreviewAgents`
- Traits: `paginated` (1)
- Common required input members in this group: `previewAgentsArn`

### Register

- Operations: `RegisterCrossAccountAccessRole`
- Common required input members in this group: `roleArn`

### Remove

- Operations: `RemoveAttributesFromFindings`
- Common required input members in this group: `attributeKeys`, `findingArns`

### Set

- Operations: `SetTagsForResource`
- Common required input members in this group: `resourceArn`

### Start

- Operations: `StartAssessmentRun`
- Common required input members in this group: `assessmentTemplateArn`

### Stop

- Operations: `StopAssessmentRun`
- Common required input members in this group: `assessmentRunArn`

### Subscribe

- Operations: `SubscribeToEvent`
- Common required input members in this group: `event`, `resourceArn`, `topicArn`

### Unsubscribe

- Operations: `UnsubscribeFromEvent`
- Common required input members in this group: `event`, `resourceArn`, `topicArn`

### Update

- Operations: `UpdateAssessmentTarget`
- Common required input members in this group: `assessmentTargetArn`, `assessmentTargetName`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddAttributesToFindings` | - | - | `attributes`, `findingArns` | - | `AddAttributesToFindingsResponse` | `AccessDeniedException`, `InternalException`, `InvalidInputException`, `NoSuchEntityException`, `ServiceTemporarilyUnavailableException` | Assigns attributes (key and value pairs) to the findings that are specified by the ARNs of the findings. |
| `CreateAssessmentTarget` | - | - | `assessmentTargetName` | - | `CreateAssessmentTargetResponse` | `AccessDeniedException`, `InternalException`, `InvalidCrossAccountRoleException`, `InvalidInputException`, `LimitExceededException`, `NoSuchEntityException`, `ServiceTemporarilyUnavailableException` | Creates a new assessment target using the ARN of the resource group that is generated by CreateResourceGroup. If resourceGroupArn is not specified, all EC2 instances in the current AWS account and region are included in the assessment target. |
| `CreateAssessmentTemplate` | - | - | `assessmentTargetArn`, `assessmentTemplateName`, `durationInSeconds`, `rulesPackageArns` | - | `CreateAssessmentTemplateResponse` | `AccessDeniedException`, `InternalException`, `InvalidInputException`, `LimitExceededException`, `NoSuchEntityException`, `ServiceTemporarilyUnavailableException` | Creates an assessment template for the assessment target that is specified by the ARN of the assessment target. If the service-linked role isn’t already registered, this action also creates and registers a service-linked role to grant Amazon Inspector access... |
| `CreateExclusionsPreview` | - | - | `assessmentTemplateArn` | - | `CreateExclusionsPreviewResponse` | `AccessDeniedException`, `InternalException`, `InvalidInputException`, `NoSuchEntityException`, `PreviewGenerationInProgressException`, `ServiceTemporarilyUnavailableException` | Starts the generation of an exclusions preview for the specified assessment template. The exclusions preview lists the potential exclusions (ExclusionPreview) that Inspector can detect before it runs the assessment. |
| `CreateResourceGroup` | - | - | `resourceGroupTags` | - | `CreateResourceGroupResponse` | `AccessDeniedException`, `InternalException`, `InvalidInputException`, `LimitExceededException`, `ServiceTemporarilyUnavailableException` | Creates a resource group using the specified set of tags (key and value pairs) that are used to select the EC2 instances to be included in an Amazon Inspector assessment target. The created resource group is then used to create an Amazon Inspector assessment... |
| `DeleteAssessmentRun` | - | - | `assessmentRunArn` | - | `Unit` | `AccessDeniedException`, `AssessmentRunInProgressException`, `InternalException`, `InvalidInputException`, `NoSuchEntityException`, `ServiceTemporarilyUnavailableException` | Deletes the assessment run that is specified by the ARN of the assessment run. |
| `DeleteAssessmentTarget` | - | - | `assessmentTargetArn` | - | `Unit` | `AccessDeniedException`, `AssessmentRunInProgressException`, `InternalException`, `InvalidInputException`, `NoSuchEntityException`, `ServiceTemporarilyUnavailableException` | Deletes the assessment target that is specified by the ARN of the assessment target. |
| `DeleteAssessmentTemplate` | - | - | `assessmentTemplateArn` | - | `Unit` | `AccessDeniedException`, `AssessmentRunInProgressException`, `InternalException`, `InvalidInputException`, `NoSuchEntityException`, `ServiceTemporarilyUnavailableException` | Deletes the assessment template that is specified by the ARN of the assessment template. |
| `DescribeAssessmentRuns` | - | - | `assessmentRunArns` | - | `DescribeAssessmentRunsResponse` | `InternalException`, `InvalidInputException` | Describes the assessment runs that are specified by the ARNs of the assessment runs. |
| `DescribeAssessmentTargets` | - | - | `assessmentTargetArns` | - | `DescribeAssessmentTargetsResponse` | `InternalException`, `InvalidInputException` | Describes the assessment targets that are specified by the ARNs of the assessment targets. |
| `DescribeAssessmentTemplates` | - | - | `assessmentTemplateArns` | - | `DescribeAssessmentTemplatesResponse` | `InternalException`, `InvalidInputException` | Describes the assessment templates that are specified by the ARNs of the assessment templates. |
| `DescribeCrossAccountAccessRole` | - | - | - | - | `DescribeCrossAccountAccessRoleResponse` | `InternalException` | Describes the IAM role that enables Amazon Inspector to access your AWS account. |
| `DescribeExclusions` | - | - | `exclusionArns` | - | `DescribeExclusionsResponse` | `InternalException`, `InvalidInputException` | Describes the exclusions that are specified by the exclusions' ARNs. |
| `DescribeFindings` | - | - | `findingArns` | - | `DescribeFindingsResponse` | `InternalException`, `InvalidInputException` | Describes the findings that are specified by the ARNs of the findings. |
| `DescribeResourceGroups` | - | - | `resourceGroupArns` | - | `DescribeResourceGroupsResponse` | `InternalException`, `InvalidInputException` | Describes the resource groups that are specified by the ARNs of the resource groups. |
| `DescribeRulesPackages` | - | - | `rulesPackageArns` | - | `DescribeRulesPackagesResponse` | `InternalException`, `InvalidInputException` | Describes the rules packages that are specified by the ARNs of the rules packages. |
| `GetAssessmentReport` | - | - | `assessmentRunArn`, `reportFileFormat`, `reportType` | - | `GetAssessmentReportResponse` | `AccessDeniedException`, `AssessmentRunInProgressException`, `InternalException`, `InvalidInputException`, `NoSuchEntityException`, `ServiceTemporarilyUnavailableException`, `UnsupportedFeatureException` | Produces an assessment report that includes detailed and comprehensive results of a specified assessment run. |
| `GetExclusionsPreview` | - | `paginated` | `assessmentTemplateArn`, `previewToken` | - | `GetExclusionsPreviewResponse` | `AccessDeniedException`, `InternalException`, `InvalidInputException`, `NoSuchEntityException` | Retrieves the exclusions preview (a list of ExclusionPreview objects) specified by the preview token. You can obtain the preview token by running the CreateExclusionsPreview API. |
| `GetTelemetryMetadata` | - | - | `assessmentRunArn` | - | `GetTelemetryMetadataResponse` | `AccessDeniedException`, `InternalException`, `InvalidInputException`, `NoSuchEntityException` | Information about the data that is collected for the specified assessment run. |
| `ListAssessmentRunAgents` | - | `paginated` | `assessmentRunArn` | - | `ListAssessmentRunAgentsResponse` | `AccessDeniedException`, `InternalException`, `InvalidInputException`, `NoSuchEntityException` | Lists the agents of the assessment runs that are specified by the ARNs of the assessment runs. |
| `ListAssessmentRuns` | - | `paginated` | - | - | `ListAssessmentRunsResponse` | `AccessDeniedException`, `InternalException`, `InvalidInputException`, `NoSuchEntityException` | Lists the assessment runs that correspond to the assessment templates that are specified by the ARNs of the assessment templates. |
| `ListAssessmentTargets` | - | `paginated` | - | - | `ListAssessmentTargetsResponse` | `AccessDeniedException`, `InternalException`, `InvalidInputException` | Lists the ARNs of the assessment targets within this AWS account. For more information about assessment targets, see Amazon Inspector Assessment Targets. |
| `ListAssessmentTemplates` | - | `paginated` | - | - | `ListAssessmentTemplatesResponse` | `AccessDeniedException`, `InternalException`, `InvalidInputException`, `NoSuchEntityException` | Lists the assessment templates that correspond to the assessment targets that are specified by the ARNs of the assessment targets. |
| `ListEventSubscriptions` | - | `paginated` | - | - | `ListEventSubscriptionsResponse` | `AccessDeniedException`, `InternalException`, `InvalidInputException`, `NoSuchEntityException` | Lists all the event subscriptions for the assessment template that is specified by the ARN of the assessment template. For more information, see SubscribeToEvent and UnsubscribeFromEvent. |
| `ListExclusions` | - | `paginated` | `assessmentRunArn` | - | `ListExclusionsResponse` | `AccessDeniedException`, `InternalException`, `InvalidInputException`, `NoSuchEntityException` | List exclusions that are generated by the assessment run. |
| `ListFindings` | - | `paginated` | - | - | `ListFindingsResponse` | `AccessDeniedException`, `InternalException`, `InvalidInputException`, `NoSuchEntityException` | Lists findings that are generated by the assessment runs that are specified by the ARNs of the assessment runs. |
| `ListRulesPackages` | - | `paginated` | - | - | `ListRulesPackagesResponse` | `AccessDeniedException`, `InternalException`, `InvalidInputException` | Lists all available Amazon Inspector rules packages. |
| `ListTagsForResource` | - | - | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalException`, `InvalidInputException`, `NoSuchEntityException` | Lists all tags associated with an assessment template. |
| `PreviewAgents` | - | `paginated` | `previewAgentsArn` | - | `PreviewAgentsResponse` | `AccessDeniedException`, `InternalException`, `InvalidCrossAccountRoleException`, `InvalidInputException`, `NoSuchEntityException` | Previews the agents installed on the EC2 instances that are part of the specified assessment target. |
| `RegisterCrossAccountAccessRole` | - | - | `roleArn` | - | `Unit` | `AccessDeniedException`, `InternalException`, `InvalidCrossAccountRoleException`, `InvalidInputException`, `ServiceTemporarilyUnavailableException` | Registers the IAM role that grants Amazon Inspector access to AWS Services needed to perform security assessments. |
| `RemoveAttributesFromFindings` | - | - | `attributeKeys`, `findingArns` | - | `RemoveAttributesFromFindingsResponse` | `AccessDeniedException`, `InternalException`, `InvalidInputException`, `NoSuchEntityException`, `ServiceTemporarilyUnavailableException` | Removes entire attributes (key and value pairs) from the findings that are specified by the ARNs of the findings where an attribute with the specified key exists. |
| `SetTagsForResource` | - | - | `resourceArn` | - | `Unit` | `AccessDeniedException`, `InternalException`, `InvalidInputException`, `NoSuchEntityException`, `ServiceTemporarilyUnavailableException` | Sets tags (key and value pairs) to the assessment template that is specified by the ARN of the assessment template. |
| `StartAssessmentRun` | - | - | `assessmentTemplateArn` | - | `StartAssessmentRunResponse` | `AccessDeniedException`, `AgentsAlreadyRunningAssessmentException`, `InternalException`, `InvalidCrossAccountRoleException`, `InvalidInputException`, `LimitExceededException`, `NoSuchEntityException`, `ServiceTemporarilyUnavailableException` | Starts the assessment run specified by the ARN of the assessment template. For this API to function properly, you must not exceed the limit of running up to 500 concurrent agents per AWS account. |
| `StopAssessmentRun` | - | - | `assessmentRunArn` | - | `Unit` | `AccessDeniedException`, `InternalException`, `InvalidInputException`, `NoSuchEntityException`, `ServiceTemporarilyUnavailableException` | Stops the assessment run that is specified by the ARN of the assessment run. |
| `SubscribeToEvent` | - | - | `event`, `resourceArn`, `topicArn` | - | `Unit` | `AccessDeniedException`, `InternalException`, `InvalidInputException`, `LimitExceededException`, `NoSuchEntityException`, `ServiceTemporarilyUnavailableException` | Enables the process of sending Amazon Simple Notification Service (SNS) notifications about a specified event to a specified SNS topic. |
| `UnsubscribeFromEvent` | - | - | `event`, `resourceArn`, `topicArn` | - | `Unit` | `AccessDeniedException`, `InternalException`, `InvalidInputException`, `NoSuchEntityException`, `ServiceTemporarilyUnavailableException` | Disables the process of sending Amazon Simple Notification Service (SNS) notifications about a specified event to a specified SNS topic. |
| `UpdateAssessmentTarget` | - | - | `assessmentTargetArn`, `assessmentTargetName` | - | `Unit` | `AccessDeniedException`, `InternalException`, `InvalidInputException`, `NoSuchEntityException`, `ServiceTemporarilyUnavailableException` | Updates the assessment target that is specified by the ARN of the assessment target. If resourceGroupArn is not specified, all EC2 instances in the current AWS account and region are included in the assessment target. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalException` | `structure` | `canRetry`, `message` | Internal server error. |
| `InvalidInputException` | `structure` | `canRetry`, `errorCode`, `message` | The request was rejected because an invalid or out-of-range value was supplied for an input parameter. |
| `AccessDeniedException` | `structure` | `canRetry`, `errorCode`, `message` | You do not have required permissions to access the requested resource. |
| `NoSuchEntityException` | `structure` | `canRetry`, `errorCode`, `message` | The request was rejected because it referenced an entity that does not exist. |
| `ServiceTemporarilyUnavailableException` | `structure` | `canRetry`, `message` | The serice is temporary unavailable. |
| `LimitExceededException` | `structure` | `canRetry`, `errorCode`, `message` | The request was rejected because it attempted to create resources beyond the current AWS account limits. |
| `InvalidCrossAccountRoleException` | `structure` | `canRetry`, `errorCode`, `message` | Amazon Inspector cannot assume the cross-account role that it needs to list your EC2 instances during the assessment run. |
| `AssessmentRunInProgressException` | `structure` | `assessmentRunArns`, `assessmentRunArnsTruncated`, `canRetry`, `message` | You cannot perform a specified action if an assessment run is currently in progress. |
| `AddAttributesToFindingsRequest` | `structure` | `attributes`, `findingArns` | - |
| `AddAttributesToFindingsResponse` | `structure` | `failedItems` | - |
| `CreateAssessmentTargetRequest` | `structure` | `assessmentTargetName`, `resourceGroupArn` | - |
| `CreateAssessmentTargetResponse` | `structure` | `assessmentTargetArn` | - |
| `CreateAssessmentTemplateRequest` | `structure` | `assessmentTargetArn`, `assessmentTemplateName`, `durationInSeconds`, `rulesPackageArns`, `userAttributesForFindings` | - |
| `CreateAssessmentTemplateResponse` | `structure` | `assessmentTemplateArn` | - |
| `CreateExclusionsPreviewRequest` | `structure` | `assessmentTemplateArn` | - |
| `CreateExclusionsPreviewResponse` | `structure` | `previewToken` | - |
| `PreviewGenerationInProgressException` | `structure` | `message` | The request is rejected. |
| `CreateResourceGroupRequest` | `structure` | `resourceGroupTags` | - |
| `CreateResourceGroupResponse` | `structure` | `resourceGroupArn` | - |
| `DeleteAssessmentRunRequest` | `structure` | `assessmentRunArn` | - |
| `DeleteAssessmentTargetRequest` | `structure` | `assessmentTargetArn` | - |
| `DeleteAssessmentTemplateRequest` | `structure` | `assessmentTemplateArn` | - |
| `DescribeAssessmentRunsRequest` | `structure` | `assessmentRunArns` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
