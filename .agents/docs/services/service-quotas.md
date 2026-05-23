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

- Operations: `GetAssociationForServiceQuotaTemplate`, `GetAutoManagementConfiguration`, `GetAWSDefaultServiceQuota`, `GetQuotaUtilizationReport`, `GetRequestedServiceQuotaChange`, `GetServiceQuota`, `GetServiceQuotaIncreaseRequestFromTemplate`
- Common required input members in this group: `ServiceCode`, `QuotaCode`

### List

- Operations: `ListAWSDefaultServiceQuotas`, `ListRequestedServiceQuotaChangeHistory`, `ListRequestedServiceQuotaChangeHistoryByQuota`, `ListServiceQuotaIncreaseRequestsInTemplate`, `ListServiceQuotas`, `ListServices`, `ListTagsForResource`
- Traits: `paginated` (6)
- Common required input members in this group: `ServiceCode`

### Start

- Operations: `StartAutoManagement`, `StartQuotaUtilizationReport`
- Common required input members in this group: -

### Associate

- Operations: `AssociateServiceQuotaTemplate`
- Common required input members in this group: -

### Create

- Operations: `CreateSupportCase`
- Common required input members in this group: -

### Delete

- Operations: `DeleteServiceQuotaIncreaseRequestFromTemplate`
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociateServiceQuotaTemplate`
- Common required input members in this group: -

### Put

- Operations: `PutServiceQuotaIncreaseRequestIntoTemplate`
- Common required input members in this group: -

### Request

- Operations: `RequestServiceQuotaIncrease`
- Common required input members in this group: -

### Stop

- Operations: `StopAutoManagement`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

### Update

- Operations: `UpdateAutoManagement`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateServiceQuotaTemplate` | `-` | - | - | - | `AssociateServiceQuotaTemplateResponse` | `AccessDeniedException`, `AWSServiceAccessNotEnabledException`, `DependencyAccessDeniedException`, `NoAvailableOrganizationException`, `OrganizationNotInAllFeaturesModeException`, `ServiceException`, `TemplatesNotAvailableInRegionException`, `TooManyRequestsException` | Associates your quota request template with your organization. When a new Amazon Web Services account is created in your organization, the quota increase requests in the template are automatically applied to the acco ... |
| `CreateSupportCase` | `-` | - | `RequestId` | - | `CreateSupportCaseResponse` | `AccessDeniedException`, `DependencyAccessDeniedException`, `IllegalArgumentException`, `InvalidResourceStateException`, `NoSuchResourceException`, `ResourceAlreadyExistsException`, `ServiceException`, `TooManyRequestsException` | Creates a Support case for an existing quota increase request. This call only creates a Support case if the request has a Pending status. |
| `DeleteServiceQuotaIncreaseRequestFromTemplate` | `-` | - | `ServiceCode`, `QuotaCode`, `AwsRegion` | - | `DeleteServiceQuotaIncreaseRequestFromTemplateResponse` | `AccessDeniedException`, `AWSServiceAccessNotEnabledException`, `DependencyAccessDeniedException`, `IllegalArgumentException`, `NoAvailableOrganizationException`, `NoSuchResourceException`, `ServiceException`, `TemplatesNotAvailableInRegionException`, `TooManyRequestsException` | Deletes the quota increase request for the specified quota from your quota request template. |
| `DisassociateServiceQuotaTemplate` | `-` | - | - | - | `DisassociateServiceQuotaTemplateResponse` | `AccessDeniedException`, `AWSServiceAccessNotEnabledException`, `DependencyAccessDeniedException`, `NoAvailableOrganizationException`, `ServiceException`, `ServiceQuotaTemplateNotInUseException`, `TemplatesNotAvailableInRegionException`, `TooManyRequestsException` | Disables your quota request template. After a template is disabled, the quota increase requests in the template are not applied to new Amazon Web Services accounts in your organization. Disabling a quota request temp ... |
| `GetAssociationForServiceQuotaTemplate` | `-` | - | - | - | `GetAssociationForServiceQuotaTemplateResponse` | `AccessDeniedException`, `AWSServiceAccessNotEnabledException`, `DependencyAccessDeniedException`, `NoAvailableOrganizationException`, `ServiceException`, `ServiceQuotaTemplateNotInUseException`, `TemplatesNotAvailableInRegionException`, `TooManyRequestsException` | Retrieves the status of the association for the quota request template. |
| `GetAutoManagementConfiguration` | `-` | - | - | - | `GetAutoManagementConfigurationResponse` | `AccessDeniedException`, `IllegalArgumentException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Retrieves information about your Service Quotas Automatic Management configuration. Automatic Management monitors your Service Quotas utilization and notifies you before you run out of your allocated quotas. |
| `GetAWSDefaultServiceQuota` | `-` | - | `ServiceCode`, `QuotaCode` | - | `GetAWSDefaultServiceQuotaResponse` | `AccessDeniedException`, `IllegalArgumentException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Retrieves the default value for the specified quota. The default value does not reflect any quota increases. |
| `GetQuotaUtilizationReport` | `-` | - | `ReportId` | - | `GetQuotaUtilizationReportResponse` | `AccessDeniedException`, `IllegalArgumentException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Retrieves the quota utilization report for your Amazon Web Services account. This operation returns paginated results showing your quota usage across all Amazon Web Services services, sorted by utilization percentage ... |
| `GetRequestedServiceQuotaChange` | `-` | - | `RequestId` | - | `GetRequestedServiceQuotaChangeResponse` | `AccessDeniedException`, `IllegalArgumentException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Retrieves information about the specified quota increase request. |
| `GetServiceQuota` | `-` | - | `ServiceCode`, `QuotaCode` | - | `GetServiceQuotaResponse` | `AccessDeniedException`, `IllegalArgumentException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Retrieves the applied quota value for the specified account-level or resource-level quota. For some quotas, only the default values are available. If the applied quota value is not available for a quota, the quota is ... |
| `GetServiceQuotaIncreaseRequestFromTemplate` | `-` | - | `ServiceCode`, `QuotaCode`, `AwsRegion` | - | `GetServiceQuotaIncreaseRequestFromTemplateResponse` | `AccessDeniedException`, `AWSServiceAccessNotEnabledException`, `DependencyAccessDeniedException`, `IllegalArgumentException`, `NoAvailableOrganizationException`, `NoSuchResourceException`, `ServiceException`, `TemplatesNotAvailableInRegionException`, `TooManyRequestsException` | Retrieves information about the specified quota increase request in your quota request template. |
| `ListAWSDefaultServiceQuotas` | `-` | `paginated` | `ServiceCode` | - | `ListAWSDefaultServiceQuotasResponse` | `AccessDeniedException`, `IllegalArgumentException`, `InvalidPaginationTokenException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Lists the default values for the quotas for the specified Amazon Web Services service. A default value does not reflect any quota increases. |
| `ListRequestedServiceQuotaChangeHistory` | `-` | `paginated` | - | - | `ListRequestedServiceQuotaChangeHistoryResponse` | `AccessDeniedException`, `IllegalArgumentException`, `InvalidPaginationTokenException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Retrieves the quota increase requests for the specified Amazon Web Services service. Filter responses to return quota requests at either the account level, resource level, or all levels. Responses include any open or ... |
| `ListRequestedServiceQuotaChangeHistoryByQuota` | `-` | `paginated` | `ServiceCode`, `QuotaCode` | - | `ListRequestedServiceQuotaChangeHistoryByQuotaResponse` | `AccessDeniedException`, `IllegalArgumentException`, `InvalidPaginationTokenException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Retrieves the quota increase requests for the specified quota. Filter responses to return quota requests at either the account level, resource level, or all levels. |
| `ListServiceQuotaIncreaseRequestsInTemplate` | `-` | `paginated` | - | - | `ListServiceQuotaIncreaseRequestsInTemplateResponse` | `AccessDeniedException`, `AWSServiceAccessNotEnabledException`, `DependencyAccessDeniedException`, `IllegalArgumentException`, `NoAvailableOrganizationException`, `ServiceException`, `TemplatesNotAvailableInRegionException`, `TooManyRequestsException` | Lists the quota increase requests in the specified quota request template. |
| `ListServiceQuotas` | `-` | `paginated` | `ServiceCode` | - | `ListServiceQuotasResponse` | `AccessDeniedException`, `IllegalArgumentException`, `InvalidPaginationTokenException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Lists the applied quota values for the specified Amazon Web Services service. For some quotas, only the default values are available. If the applied quota value is not available for a quota, the quota is not retrieve ... |
| `ListServices` | `-` | `paginated` | - | - | `ListServicesResponse` | `AccessDeniedException`, `IllegalArgumentException`, `InvalidPaginationTokenException`, `ServiceException`, `TooManyRequestsException` | Lists the names and codes for the Amazon Web Services services integrated with Service Quotas. |
| `ListTagsForResource` | `-` | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `IllegalArgumentException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Returns a list of the tags assigned to the specified applied quota. |
| `PutServiceQuotaIncreaseRequestIntoTemplate` | `-` | - | `QuotaCode`, `ServiceCode`, `AwsRegion`, `DesiredValue` | - | `PutServiceQuotaIncreaseRequestIntoTemplateResponse` | `AccessDeniedException`, `AWSServiceAccessNotEnabledException`, `DependencyAccessDeniedException`, `IllegalArgumentException`, `NoAvailableOrganizationException`, `NoSuchResourceException`, `QuotaExceededException`, `ServiceException`, `TemplatesNotAvailableInRegionException`, `TooManyRequestsException` | Adds a quota increase request to your quota request template. |
| `RequestServiceQuotaIncrease` | `-` | - | `ServiceCode`, `QuotaCode`, `DesiredValue` | - | `RequestServiceQuotaIncreaseResponse` | `AccessDeniedException`, `DependencyAccessDeniedException`, `IllegalArgumentException`, `InvalidResourceStateException`, `NoSuchResourceException`, `QuotaExceededException`, `ResourceAlreadyExistsException`, `ServiceException`, `TooManyRequestsException` | Submits a quota increase request for the specified quota at the account or resource level. |
| `StartAutoManagement` | `-` | - | `OptInLevel`, `OptInType` | - | `StartAutoManagementResponse` | `AccessDeniedException`, `IllegalArgumentException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Starts Service Quotas Automatic Management for an Amazon Web Services account, including notification preferences and excluded quotas configurations. Automatic Management monitors your Service Quotas utilization and ... |
| `StartQuotaUtilizationReport` | `-` | - | - | - | `StartQuotaUtilizationReportResponse` | `AccessDeniedException`, `IllegalArgumentException`, `InvalidPaginationTokenException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Initiates the generation of a quota utilization report for your Amazon Web Services account. This asynchronous operation analyzes your quota usage across all Amazon Web Services services and returns a unique report i ... |
| `StopAutoManagement` | `-` | - | - | - | `StopAutoManagementResponse` | `AccessDeniedException`, `IllegalArgumentException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Stops Service Quotas Automatic Management for an Amazon Web Services account and removes all associated configurations. Automatic Management monitors your Service Quotas utilization and notifies you before you run ou ... |
| `TagResource` | `-` | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `IllegalArgumentException`, `NoSuchResourceException`, `ServiceException`, `TagPolicyViolationException`, `TooManyRequestsException`, `TooManyTagsException` | Adds tags to the specified applied quota. You can include one or more tags to add to the quota. |
| `UntagResource` | `-` | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `IllegalArgumentException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Removes tags from the specified applied quota. You can specify one or more tags to remove. |
| `UpdateAutoManagement` | `-` | - | - | - | `UpdateAutoManagementResponse` | `AccessDeniedException`, `IllegalArgumentException`, `NoSuchResourceException`, `ServiceException`, `TooManyRequestsException` | Updates your Service Quotas Automatic Management configuration, including notification preferences and excluded quotas. Automatic Management monitors your Service Quotas utilization and notifies you before you run ou ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AWSServiceAccessNotEnabledException` | `structure` | Message | The action you attempted is not allowed unless Service Access with Service Quotas is enabled in your organization. |
| `AccessDeniedException` | `structure` | Message | You do not have sufficient permission to perform this action. |
| `DependencyAccessDeniedException` | `structure` | Message | You can't perform this action because a dependency does not have access. |
| `IllegalArgumentException` | `structure` | Message | Invalid input was provided. |
| `InvalidPaginationTokenException` | `structure` | Message | Invalid input was provided. |
| `InvalidResourceStateException` | `structure` | Message | The resource is in an invalid state. |
| `NoAvailableOrganizationException` | `structure` | Message | The Amazon Web Services account making this call is not a member of an organization. |
| `NoSuchResourceException` | `structure` | Message | The specified resource does not exist. |
| `OrganizationNotInAllFeaturesModeException` | `structure` | Message | The organization that your Amazon Web Services account belongs to is not in All Features mode. |
| `QuotaExceededException` | `structure` | Message | You have exceeded your service quota. To perform the requested action, remove some of the relevant resources, or use Service Quotas to request a service quo ... |
| `ResourceAlreadyExistsException` | `structure` | Message | The specified resource already exists. |
| `ServiceException` | `structure` | Message | Something went wrong. |
| `ServiceQuotaTemplateNotInUseException` | `structure` | Message | The quota request template is not associated with your organization. |
| `TagPolicyViolationException` | `structure` | Message | The specified tag is a reserved word and cannot be used. |
| `TemplatesNotAvailableInRegionException` | `structure` | Message | The Service Quotas template is not available in this Amazon Web Services Region. |
| `TooManyRequestsException` | `structure` | Message | Due to throttling, the request was denied. Slow down the rate of request calls, or request an increase for this quota. |
| `TooManyTagsException` | `structure` | Message | You've exceeded the number of tags allowed for a resource. For more information, see Tag restrictions in the Service Quotas User Guide . |
| `AssociateServiceQuotaTemplateRequest` | `structure` | **empty (no members)** | - |
| `AssociateServiceQuotaTemplateResponse` | `structure` | **empty (no members)** | - |
| `CreateSupportCaseRequest` | `structure` | RequestId | - |
| `CreateSupportCaseResponse` | `structure` | **empty (no members)** | - |
| `DeleteServiceQuotaIncreaseRequestFromTemplateRequest` | `structure` | ServiceCode, QuotaCode, AwsRegion | - |
| `DeleteServiceQuotaIncreaseRequestFromTemplateResponse` | `structure` | **empty (no members)** | - |
| `DisassociateServiceQuotaTemplateRequest` | `structure` | **empty (no members)** | - |
| `DisassociateServiceQuotaTemplateResponse` | `structure` | **empty (no members)** | - |
| `GetAssociationForServiceQuotaTemplateRequest` | `structure` | **empty (no members)** | - |
| `GetAssociationForServiceQuotaTemplateResponse` | `structure` | ServiceQuotaTemplateAssociationStatus | - |
| `GetAutoManagementConfigurationRequest` | `structure` | **empty (no members)** | - |
| `GetAutoManagementConfigurationResponse` | `structure` | OptInLevel, OptInType, NotificationArn, OptInStatus, ExclusionList | - |
| `GetAWSDefaultServiceQuotaRequest` | `structure` | ServiceCode, QuotaCode | - |
| `GetAWSDefaultServiceQuotaResponse` | `structure` | Quota | - |
| `GetQuotaUtilizationReportRequest` | `structure` | ReportId, NextToken, MaxResults | - |
| `GetQuotaUtilizationReportResponse` | `structure` | ReportId, Status, GeneratedAt, TotalCount, Quotas, NextToken, ErrorCode, ErrorMessage | - |
| `GetRequestedServiceQuotaChangeRequest` | `structure` | RequestId | - |
| `GetRequestedServiceQuotaChangeResponse` | `structure` | RequestedQuota | - |
| `GetServiceQuotaRequest` | `structure` | ServiceCode, QuotaCode, ContextId | - |
| `GetServiceQuotaResponse` | `structure` | Quota | - |
| `GetServiceQuotaIncreaseRequestFromTemplateRequest` | `structure` | ServiceCode, QuotaCode, AwsRegion | - |
| `GetServiceQuotaIncreaseRequestFromTemplateResponse` | `structure` | ServiceQuotaIncreaseRequestInTemplate | - |
| `ListAWSDefaultServiceQuotasRequest` | `structure` | ServiceCode, NextToken, MaxResults | - |
| `AppliedLevelEnum` | `enum` | ACCOUNT, RESOURCE, ALL | - |
| `ErrorCode` | `enum` | DEPENDENCY_ACCESS_DENIED_ERROR, DEPENDENCY_THROTTLING_ERROR, DEPENDENCY_SERVICE_ERROR, SERVICE_QUOTA_NOT_AVAILABLE_ERROR | - |
| `OptInLevel` | `enum` | ACCOUNT | - |
| `OptInStatus` | `enum` | ENABLED, DISABLED | - |
| `OptInType` | `enum` | NotifyOnly, NotifyAndAdjust | - |
| `PeriodUnit` | `enum` | MICROSECOND, MILLISECOND, SECOND, MINUTE, HOUR, DAY, WEEK | - |
| `QuotaContextScope` | `enum` | RESOURCE, ACCOUNT | - |
| `ReportStatus` | `enum` | PENDING, IN_PROGRESS, COMPLETED, FAILED | - |
| `RequestStatus` | `enum` | PENDING, CASE_OPENED, APPROVED, DENIED, CASE_CLOSED, NOT_APPROVED, INVALID_REQUEST | - |
| `RequestType` | `enum` | AutomaticManagement | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
