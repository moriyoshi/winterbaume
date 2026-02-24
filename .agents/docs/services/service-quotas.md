# Service Quotas

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

With Service Quotas, you can view and manage your quotas easily as your Amazon Web Services workloads grow. Quotas, also referred to as limits, are the maximum number of resources that you can create in your Amazon Web Services account. For more information, see the Service Quotas User Guide. You need Amazon Web Services CLI version 2.13.20 or higher to view and manage resource-level quotas such as `Instances per domain` for Amazon OpenSearch Service.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-servicequotas/tests/scenario_test.rs`: discover services and quotas for a target service.
- Backported from `scenario_test.rs`: compare applied quotas with default quota values.
- Backported from `scenario_test.rs`: survey quotas across multiple services.
- Scenario insight from EC2: include mutable binding failover for Service Quotas where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Service Quotas by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Service Quotas by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: model quota catalogue lookup, default/applied quota comparison, service-code filtering, quota increase request state, templates, and tag-aware quota resources.

## Service Identity and Protocol

- AWS model slug: `service-quotas`
- AWS SDK for Rust slug: `servicequotas`
- Model version: `2019-06-24`
- Model file: `vendor/api-models-aws/models/service-quotas/service/2019-06-24/service-quotas-2019-06-24.json`
- SDK ID: `Service Quotas`
- Endpoint prefix: `servicequotas`
- ARN namespace: `servicequotas`
- CloudFormation name: `ServiceQuotas`
- CloudTrail event source: `servicequotas.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (7), `List` (7), `Start` (2), `Associate` (1), `Create` (1), `Delete` (1), `Disassociate` (1), `Put` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateServiceQuotaTemplate`, `CreateSupportCase`, `DeleteServiceQuotaIncreaseRequestFromTemplate`, `DisassociateServiceQuotaTemplate`, `PutServiceQuotaIncreaseRequestIntoTemplate`, `StartAutoManagement`, `StartQuotaUtilizationReport`, `StopAutoManagement`, `TagResource`, `UntagResource`, `UpdateAutoManagement`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAWSDefaultServiceQuota`, `GetAssociationForServiceQuotaTemplate`, `GetAutoManagementConfiguration`, `GetQuotaUtilizationReport`, `GetRequestedServiceQuotaChange`, `GetServiceQuota`, `GetServiceQuotaIncreaseRequestFromTemplate`, `ListAWSDefaultServiceQuotas`, `ListRequestedServiceQuotaChangeHistory`, `ListRequestedServiceQuotaChangeHistoryByQuota`, `ListServiceQuotaIncreaseRequestsInTemplate`, `ListServiceQuotas`, `ListServices`, `ListTagsForResource`.
- Pagination is modelled for 6 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetQuotaUtilizationReport`, `StartAutoManagement`, `StartQuotaUtilizationReport`, `StopAutoManagement`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 26 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `CloudWatch`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetAWSDefaultServiceQuota`, `GetAssociationForServiceQuotaTemplate`, `GetAutoManagementConfiguration`, `GetQuotaUtilizationReport`, `GetRequestedServiceQuotaChange`, `GetServiceQuota`, `GetServiceQuotaIncreaseRequestFromTemplate`
- Common required input members in this group: `AwsRegion`, `QuotaCode`, `ReportId`, `RequestId`, `ServiceCode`

### List

- Operations: `ListAWSDefaultServiceQuotas`, `ListRequestedServiceQuotaChangeHistory`, `ListRequestedServiceQuotaChangeHistoryByQuota`, `ListServiceQuotaIncreaseRequestsInTemplate`, `ListServiceQuotas`, `ListServices`, `ListTagsForResource`
- Traits: `paginated` (6)
- Common required input members in this group: `QuotaCode`, `ResourceARN`, `ServiceCode`

### Start

- Operations: `StartAutoManagement`, `StartQuotaUtilizationReport`
- Common required input members in this group: `OptInLevel`, `OptInType`

### Associate

- Operations: `AssociateServiceQuotaTemplate`

### Create

- Operations: `CreateSupportCase`
- Common required input members in this group: `RequestId`

### Delete

- Operations: `DeleteServiceQuotaIncreaseRequestFromTemplate`
- Common required input members in this group: `AwsRegion`, `QuotaCode`, `ServiceCode`

### Disassociate

- Operations: `DisassociateServiceQuotaTemplate`

### Put

- Operations: `PutServiceQuotaIncreaseRequestIntoTemplate`
- Common required input members in this group: `AwsRegion`, `DesiredValue`, `QuotaCode`, `ServiceCode`

### Request

- Operations: `RequestServiceQuotaIncrease`
- Common required input members in this group: `DesiredValue`, `QuotaCode`, `ServiceCode`

### Stop

- Operations: `StopAutoManagement`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

### Update

- Operations: `UpdateAutoManagement`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateServiceQuotaTemplate` | - | - | - | - | `AssociateServiceQuotaTemplateResponse` | `AWSServiceAccessNotEnabledException`, `AccessDeniedException`, `DependencyAccessDeniedException`, `NoAvailableOrganizationException`, `OrganizationNotInAllFeaturesModeException`, `ServiceException`, `TemplatesNotAvailableInRegionException`, `TooManyRequestsException` | Associates your quota request template with your organization. When a new Amazon Web Services account is created in your organization, the quota increase requests in the template are automatically applied to the account. |
| `CreateSupportCase` | - | - | `RequestId` | - | `CreateSupportCaseResponse` | `AccessDeniedException`, `DependencyAccessDeniedException`, `IllegalArgumentException`, `InvalidResourceStateException`, `NoSuchResourceException`, `ResourceAlreadyExistsException`, `ServiceException`, `TooManyRequestsException` | Creates a Support case for an existing quota increase request. This call only creates a Support case if the request has a `Pending` status. |
| `DeleteServiceQuotaIncreaseRequestFromTemplate` | - | - | `AwsRegion`, `QuotaCode`, `ServiceCode` | - | `DeleteServiceQuotaIncreaseRequestFromTemplateResponse` | `AWSServiceAccessNotEnabledException`, `AccessDeniedException`, `DependencyAccessDeniedException`, `IllegalArgumentException`, `NoAvailableOrganizationException`, `NoSuchResourceException`, `ServiceException`, `TemplatesNotAvailableInRegionException`, ... (+1) | Deletes the quota increase request for the specified quota from your quota request template. |
| `DisassociateServiceQuotaTemplate` | - | - | - | - | `DisassociateServiceQuotaTemplateResponse` | `AWSServiceAccessNotEnabledException`, `AccessDeniedException`, `DependencyAccessDeniedException`, `NoAvailableOrganizationException`, `ServiceException`, `ServiceQuotaTemplateNotInUseException`, `TemplatesNotAvailableInRegionException`, `TooManyRequestsException` | Disables your quota request template. After a template is disabled, the quota increase requests in the template are not applied to new Amazon Web Services accounts in your organization. |
| `GetAWSDefaultServiceQuota` | - | - | `QuotaCode`, `ServiceCode` | - | `GetAWSDefaultServiceQuotaResponse` | `AccessDeniedException`, `IllegalArgumentException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Retrieves the default value for the specified quota. The default value does not reflect any quota increases. |
| `GetAssociationForServiceQuotaTemplate` | - | - | - | - | `GetAssociationForServiceQuotaTemplateResponse` | `AWSServiceAccessNotEnabledException`, `AccessDeniedException`, `DependencyAccessDeniedException`, `NoAvailableOrganizationException`, `ServiceException`, `ServiceQuotaTemplateNotInUseException`, `TemplatesNotAvailableInRegionException`, `TooManyRequestsException` | Retrieves the status of the association for the quota request template. |
| `GetAutoManagementConfiguration` | - | - | - | - | `GetAutoManagementConfigurationResponse` | `AccessDeniedException`, `IllegalArgumentException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Retrieves information about your Service Quotas Automatic Management configuration. Automatic Management monitors your Service Quotas utilization and notifies you before you run out of your allocated quotas. |
| `GetQuotaUtilizationReport` | - | - | `ReportId` | - | `GetQuotaUtilizationReportResponse` | `AccessDeniedException`, `IllegalArgumentException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Retrieves the quota utilization report for your Amazon Web Services account. This operation returns paginated results showing your quota usage across all Amazon Web Services services, sorted by utilization percentage in descending order (highest utilization... |
| `GetRequestedServiceQuotaChange` | - | - | `RequestId` | - | `GetRequestedServiceQuotaChangeResponse` | `AccessDeniedException`, `IllegalArgumentException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Retrieves information about the specified quota increase request. |
| `GetServiceQuota` | - | - | `QuotaCode`, `ServiceCode` | - | `GetServiceQuotaResponse` | `AccessDeniedException`, `IllegalArgumentException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Retrieves the applied quota value for the specified account-level or resource-level quota. For some quotas, only the default values are available. |
| `GetServiceQuotaIncreaseRequestFromTemplate` | - | - | `AwsRegion`, `QuotaCode`, `ServiceCode` | - | `GetServiceQuotaIncreaseRequestFromTemplateResponse` | `AWSServiceAccessNotEnabledException`, `AccessDeniedException`, `DependencyAccessDeniedException`, `IllegalArgumentException`, `NoAvailableOrganizationException`, `NoSuchResourceException`, `ServiceException`, `TemplatesNotAvailableInRegionException`, ... (+1) | Retrieves information about the specified quota increase request in your quota request template. |
| `ListAWSDefaultServiceQuotas` | - | `paginated` | `ServiceCode` | - | `ListAWSDefaultServiceQuotasResponse` | `AccessDeniedException`, `IllegalArgumentException`, `InvalidPaginationTokenException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Lists the default values for the quotas for the specified Amazon Web Services service. A default value does not reflect any quota increases. |
| `ListRequestedServiceQuotaChangeHistory` | - | `paginated` | - | - | `ListRequestedServiceQuotaChangeHistoryResponse` | `AccessDeniedException`, `IllegalArgumentException`, `InvalidPaginationTokenException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Retrieves the quota increase requests for the specified Amazon Web Services service. Filter responses to return quota requests at either the account level, resource level, or all levels. |
| `ListRequestedServiceQuotaChangeHistoryByQuota` | - | `paginated` | `QuotaCode`, `ServiceCode` | - | `ListRequestedServiceQuotaChangeHistoryByQuotaResponse` | `AccessDeniedException`, `IllegalArgumentException`, `InvalidPaginationTokenException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Retrieves the quota increase requests for the specified quota. Filter responses to return quota requests at either the account level, resource level, or all levels. |
| `ListServiceQuotaIncreaseRequestsInTemplate` | - | `paginated` | - | - | `ListServiceQuotaIncreaseRequestsInTemplateResponse` | `AWSServiceAccessNotEnabledException`, `AccessDeniedException`, `DependencyAccessDeniedException`, `IllegalArgumentException`, `NoAvailableOrganizationException`, `ServiceException`, `TemplatesNotAvailableInRegionException`, `TooManyRequestsException` | Lists the quota increase requests in the specified quota request template. |
| `ListServiceQuotas` | - | `paginated` | `ServiceCode` | - | `ListServiceQuotasResponse` | `AccessDeniedException`, `IllegalArgumentException`, `InvalidPaginationTokenException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Lists the applied quota values for the specified Amazon Web Services service. For some quotas, only the default values are available. |
| `ListServices` | - | `paginated` | - | - | `ListServicesResponse` | `AccessDeniedException`, `IllegalArgumentException`, `InvalidPaginationTokenException`, `ServiceException`, `TooManyRequestsException` | Lists the names and codes for the Amazon Web Services services integrated with Service Quotas. |
| `ListTagsForResource` | - | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `IllegalArgumentException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Returns a list of the tags assigned to the specified applied quota. |
| `PutServiceQuotaIncreaseRequestIntoTemplate` | - | - | `AwsRegion`, `DesiredValue`, `QuotaCode`, `ServiceCode` | - | `PutServiceQuotaIncreaseRequestIntoTemplateResponse` | `AWSServiceAccessNotEnabledException`, `AccessDeniedException`, `DependencyAccessDeniedException`, `IllegalArgumentException`, `NoAvailableOrganizationException`, `NoSuchResourceException`, `QuotaExceededException`, `ServiceException`, ... (+2) | Adds a quota increase request to your quota request template. |
| `RequestServiceQuotaIncrease` | - | - | `DesiredValue`, `QuotaCode`, `ServiceCode` | - | `RequestServiceQuotaIncreaseResponse` | `AccessDeniedException`, `DependencyAccessDeniedException`, `IllegalArgumentException`, `InvalidResourceStateException`, `NoSuchResourceException`, `QuotaExceededException`, `ResourceAlreadyExistsException`, `ServiceException`, ... (+1) | Submits a quota increase request for the specified quota at the account or resource level. |
| `StartAutoManagement` | - | - | `OptInLevel`, `OptInType` | - | `StartAutoManagementResponse` | `AccessDeniedException`, `IllegalArgumentException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Starts Service Quotas Automatic Management for an Amazon Web Services account, including notification preferences and excluded quotas configurations. Automatic Management monitors your Service Quotas utilization and notifies you before you run out of your... |
| `StartQuotaUtilizationReport` | - | - | - | - | `StartQuotaUtilizationReportResponse` | `AccessDeniedException`, `IllegalArgumentException`, `InvalidPaginationTokenException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Initiates the generation of a quota utilization report for your Amazon Web Services account. This asynchronous operation analyzes your quota usage across all Amazon Web Services services and returns a unique report identifier that you can use to retrieve the... |
| `StopAutoManagement` | - | - | - | - | `StopAutoManagementResponse` | `AccessDeniedException`, `IllegalArgumentException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Stops Service Quotas Automatic Management for an Amazon Web Services account and removes all associated configurations. Automatic Management monitors your Service Quotas utilization and notifies you before you run out of your allocated quotas. |
| `TagResource` | - | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `IllegalArgumentException`, `NoSuchResourceException`, `ServiceException`, `TagPolicyViolationException`, `TooManyRequestsException`, `TooManyTagsException` | Adds tags to the specified applied quota. You can include one or more tags to add to the quota. |
| `UntagResource` | - | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `IllegalArgumentException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Removes tags from the specified applied quota. You can specify one or more tags to remove. |
| `UpdateAutoManagement` | - | - | - | - | `UpdateAutoManagementResponse` | `AccessDeniedException`, `IllegalArgumentException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Updates your Service Quotas Automatic Management configuration, including notification preferences and excluded quotas. Automatic Management monitors your Service Quotas utilization and notifies you before you run out of your allocated quotas. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | You do not have sufficient permission to perform this action. |
| `ServiceException` | `structure` | `Message` | Something went wrong. |
| `TooManyRequestsException` | `structure` | `Message` | Due to throttling, the request was denied. |
| `IllegalArgumentException` | `structure` | `Message` | Invalid input was provided. |
| `NoSuchResourceException` | `structure` | `Message` | The specified resource does not exist. |
| `DependencyAccessDeniedException` | `structure` | `Message` | You can't perform this action because a dependency does not have access. |
| `AWSServiceAccessNotEnabledException` | `structure` | `Message` | The action you attempted is not allowed unless Service Access with Service Quotas is enabled in your organization. |
| `NoAvailableOrganizationException` | `structure` | `Message` | The Amazon Web Services account making this call is not a member of an organization. |
| `TemplatesNotAvailableInRegionException` | `structure` | `Message` | The Service Quotas template is not available in this Amazon Web Services Region. |
| `InvalidPaginationTokenException` | `structure` | `Message` | Invalid input was provided. |
| `InvalidResourceStateException` | `structure` | `Message` | The resource is in an invalid state. |
| `ResourceAlreadyExistsException` | `structure` | `Message` | The specified resource already exists. |
| `ServiceQuotaTemplateNotInUseException` | `structure` | `Message` | The quota request template is not associated with your organization. |
| `QuotaExceededException` | `structure` | `Message` | You have exceeded your service quota. |
| `AssociateServiceQuotaTemplateRequest` | `structure` | - | - |
| `AssociateServiceQuotaTemplateResponse` | `structure` | - | - |
| `OrganizationNotInAllFeaturesModeException` | `structure` | `Message` | The organization that your Amazon Web Services account belongs to is not in All Features mode. |
| `CreateSupportCaseRequest` | `structure` | `RequestId` | - |
| `CreateSupportCaseResponse` | `structure` | - | - |
| `DeleteServiceQuotaIncreaseRequestFromTemplateRequest` | `structure` | `AwsRegion`, `QuotaCode`, `ServiceCode` | - |
| `DeleteServiceQuotaIncreaseRequestFromTemplateResponse` | `structure` | - | - |
| `DisassociateServiceQuotaTemplateRequest` | `structure` | - | - |
| `DisassociateServiceQuotaTemplateResponse` | `structure` | - | - |
| `GetAWSDefaultServiceQuotaRequest` | `structure` | `QuotaCode`, `ServiceCode` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
