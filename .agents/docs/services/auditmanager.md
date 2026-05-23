# AWS Audit Manager

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Welcome to the Audit Manager API reference. This guide is for developers who need detailed information about the Audit Manager API operations, data types, and errors. Audit Manager is a service that provides automated evidence collection so that you can continually audit your Amazon Web Services usage. You can use it to assess the effectiveness of your controls, manage risk, and simplify compliance. Audit Manager provides prebuilt frameworks that structure and automate assessments for a given compliance standard.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Audit Manager where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS Audit Manager by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for AWS Audit Manager resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: create assessments, frameworks, controls, evidence folders, and reports for compliance audits.
- From the operation surface: support evidence collection, delegation, assessment status changes, control-set review, organisation-level settings, and tag-based audit inventory.

## Service Identity and Protocol

- AWS model slug: `auditmanager`
- AWS SDK for Rust slug: `auditmanager`
- Model version: `2017-07-25`
- Model file: `vendor/api-models-aws/models/auditmanager/service/2017-07-25/auditmanager-2017-07-25.json`
- SDK ID: `AuditManager`
- Endpoint prefix: `auditmanager`
- ARN namespace: `auditmanager`
- CloudFormation name: `AuditManager`
- CloudTrail event source: `auditmanager.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (18), `List` (12), `Update` (8), `Batch` (5), `Delete` (5), `Create` (4), `Deregister` (2), `Register` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateAssessmentReportEvidenceFolder`, `BatchAssociateAssessmentReportEvidence`, `BatchCreateDelegationByAssessment`, `BatchDeleteDelegationByAssessment`, `BatchDisassociateAssessmentReportEvidence`, `BatchImportEvidenceToAssessmentControl`, `CreateAssessment`, `CreateAssessmentFramework`, `CreateAssessmentReport`, `CreateControl`, `DeleteAssessment`, `DeleteAssessmentFramework`, `DeleteAssessmentFrameworkShare`, `DeleteAssessmentReport`, `DeleteControl`, `DeregisterAccount`, `DeregisterOrganizationAdminAccount`, `DisassociateAssessmentReportEvidenceFolder`, `RegisterAccount`, `RegisterOrganizationAdminAccount`, ... (+11).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAccountStatus`, `GetAssessment`, `GetAssessmentFramework`, `GetAssessmentReportUrl`, `GetChangeLogs`, `GetControl`, `GetDelegations`, `GetEvidence`, `GetEvidenceByEvidenceFolder`, `GetEvidenceFileUploadUrl`, `GetEvidenceFolder`, `GetEvidenceFoldersByAssessment`, `GetEvidenceFoldersByAssessmentControl`, `GetInsights`, `GetInsightsByAssessment`, `GetOrganizationAdminAccount`, `GetServicesInScope`, `GetSettings`, `ListAssessmentControlInsightsByControlDomain`, `ListAssessmentFrameworkShareRequests`, ... (+11).
- Pagination is modelled for 16 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `AssociateAssessmentReportEvidenceFolder`, `BatchAssociateAssessmentReportEvidence`, `BatchDisassociateAssessmentReportEvidence`, `BatchImportEvidenceToAssessmentControl`, `CreateAssessmentReport`, `DeleteAssessmentReport`, `DisassociateAssessmentReportEvidenceFolder`, `GetAssessmentReportUrl`, `ListAssessmentReports`, `StartAssessmentFrameworkShare`, `ValidateAssessmentReportIntegrity`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 62 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/audit-manager/latest/userguide/assessments.html
- https://docs.aws.amazon.com/audit-manager/latest/userguide/review-assessment.html
- https://docs.aws.amazon.com/audit-manager/latest/userguide/security-iam-awsmanpol.html

Research outcomes:
- Audit Manager assessments are created from frameworks and collect evidence for controls across selected AWS accounts and services.
- Frameworks contain controls that map audit requirements to evidence sources.
- Evidence can be collected automatically from AWS service integrations such as AWS Config, Security Hub, CloudTrail, and API calls, or uploaded manually.
- Assessment review is organised by assessment, control set, control, evidence folder, and evidence item.
- Controls expose assessment status, evidence counts, comments, and changelog activity.
- Audit Manager uses service-linked roles and managed policies to perform automated evidence collection.
- Multi-account assessments require appropriate organisation and role permissions.
- Audit Manager integrates with EventBridge and CloudTrail for monitoring service activity.

Parity implications:
- Model frameworks, controls, control sets, assessments, assessment reports, evidence folders, evidence items, delegations, comments, and service role state separately.
- Evidence collection should be asynchronous and source-type dependent.
- Assessment status should aggregate control and evidence state rather than being a static field.

## Operation Groups

### Get

- Operations: `GetAccountStatus`, `GetAssessment`, `GetAssessmentFramework`, `GetAssessmentReportUrl`, `GetChangeLogs`, `GetControl`, `GetDelegations`, `GetEvidence`, `GetEvidenceByEvidenceFolder`, `GetEvidenceFileUploadUrl`, `GetEvidenceFolder`, `GetEvidenceFoldersByAssessment`, `GetEvidenceFoldersByAssessmentControl`, `GetInsights`, `GetInsightsByAssessment`, `GetOrganizationAdminAccount`, `GetServicesInScope`, `GetSettings`
- Traits: `paginated` (5)
- Common required input members in this group: `assessmentId`, `assessmentReportId`, `attribute`, `controlId`, `controlSetId`, `evidenceFolderId`, `evidenceId`, `fileName`, `frameworkId`

### List

- Operations: `ListAssessmentControlInsightsByControlDomain`, `ListAssessmentFrameworkShareRequests`, `ListAssessmentFrameworks`, `ListAssessmentReports`, `ListAssessments`, `ListControlDomainInsights`, `ListControlDomainInsightsByAssessment`, `ListControlInsightsByControlDomain`, `ListControls`, `ListKeywordsForDataSource`, `ListNotifications`, `ListTagsForResource`
- Traits: `paginated` (11)
- Common required input members in this group: `assessmentId`, `controlDomainId`, `controlType`, `frameworkType`, `requestType`, `resourceArn`, `source`

### Update

- Operations: `UpdateAssessment`, `UpdateAssessmentControl`, `UpdateAssessmentControlSetStatus`, `UpdateAssessmentFramework`, `UpdateAssessmentFrameworkShare`, `UpdateAssessmentStatus`, `UpdateControl`, `UpdateSettings`
- Common required input members in this group: `action`, `assessmentId`, `comment`, `controlId`, `controlMappingSources`, `controlSetId`, `controlSets`, `frameworkId`, `name`, `requestId`, `requestType`, `scope`, `status`

### Batch

- Operations: `BatchAssociateAssessmentReportEvidence`, `BatchCreateDelegationByAssessment`, `BatchDeleteDelegationByAssessment`, `BatchDisassociateAssessmentReportEvidence`, `BatchImportEvidenceToAssessmentControl`
- Common required input members in this group: `assessmentId`, `controlId`, `controlSetId`, `createDelegationRequests`, `delegationIds`, `evidenceFolderId`, `evidenceIds`, `manualEvidence`

### Delete

- Operations: `DeleteAssessment`, `DeleteAssessmentFramework`, `DeleteAssessmentFrameworkShare`, `DeleteAssessmentReport`, `DeleteControl`
- Common required input members in this group: `assessmentId`, `assessmentReportId`, `controlId`, `frameworkId`, `requestId`, `requestType`

### Create

- Operations: `CreateAssessment`, `CreateAssessmentFramework`, `CreateAssessmentReport`, `CreateControl`
- Common required input members in this group: `assessmentId`, `assessmentReportsDestination`, `controlMappingSources`, `controlSets`, `frameworkId`, `name`, `roles`, `scope`

### Deregister

- Operations: `DeregisterAccount`, `DeregisterOrganizationAdminAccount`

### Register

- Operations: `RegisterAccount`, `RegisterOrganizationAdminAccount`
- Common required input members in this group: `adminAccountId`

### Associate

- Operations: `AssociateAssessmentReportEvidenceFolder`
- Common required input members in this group: `assessmentId`, `evidenceFolderId`

### Disassociate

- Operations: `DisassociateAssessmentReportEvidenceFolder`
- Common required input members in this group: `assessmentId`, `evidenceFolderId`

### Start

- Operations: `StartAssessmentFrameworkShare`
- Common required input members in this group: `destinationAccount`, `destinationRegion`, `frameworkId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tagKeys`

### Validate

- Operations: `ValidateAssessmentReportIntegrity`
- Common required input members in this group: `s3RelativePath`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateAssessmentReportEvidenceFolder` | `PUT /assessments/{assessmentId}/associateToAssessmentReport` | - | `assessmentId`, `evidenceFolderId` | - | `AssociateAssessmentReportEvidenceFolderResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Associates an evidence folder to an assessment report in an Audit Manager assessment. |
| `BatchAssociateAssessmentReportEvidence` | `PUT /assessments/{assessmentId}/batchAssociateToAssessmentReport` | - | `assessmentId`, `evidenceFolderId`, `evidenceIds` | - | `BatchAssociateAssessmentReportEvidenceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Associates a list of evidence to an assessment report in an Audit Manager assessment. |
| `BatchCreateDelegationByAssessment` | `POST /assessments/{assessmentId}/delegations` | - | `assessmentId`, `createDelegationRequests` | - | `BatchCreateDelegationByAssessmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Creates a batch of delegations for an assessment in Audit Manager. |
| `BatchDeleteDelegationByAssessment` | `PUT /assessments/{assessmentId}/delegations` | - | `assessmentId`, `delegationIds` | - | `BatchDeleteDelegationByAssessmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes a batch of delegations for an assessment in Audit Manager. |
| `BatchDisassociateAssessmentReportEvidence` | `PUT /assessments/{assessmentId}/batchDisassociateFromAssessmentReport` | - | `assessmentId`, `evidenceFolderId`, `evidenceIds` | - | `BatchDisassociateAssessmentReportEvidenceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Disassociates a list of evidence from an assessment report in Audit Manager. |
| `BatchImportEvidenceToAssessmentControl` | `POST /assessments/{assessmentId}/controlSets/{controlSetId}/controls/{controlId}/evidence` | - | `assessmentId`, `controlId`, `controlSetId`, `manualEvidence` | - | `BatchImportEvidenceToAssessmentControlResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds one or more pieces of evidence to a control in an Audit Manager assessment. You can import manual evidence from any S3 bucket by specifying the S3 URI of the object. |
| `CreateAssessment` | `POST /assessments` | - | `assessmentReportsDestination`, `frameworkId`, `name`, `roles`, `scope` | - | `CreateAssessmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an assessment in Audit Manager. |
| `CreateAssessmentFramework` | `POST /assessmentFrameworks` | - | `controlSets`, `name` | - | `CreateAssessmentFrameworkResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a custom framework in Audit Manager. |
| `CreateAssessmentReport` | `POST /assessments/{assessmentId}/reports` | - | `assessmentId`, `name` | - | `CreateAssessmentReportResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Creates an assessment report for the specified assessment. |
| `CreateControl` | `POST /controls` | - | `controlMappingSources`, `name` | - | `CreateControlResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a new custom control in Audit Manager. |
| `DeleteAssessment` | `DELETE /assessments/{assessmentId}` | - | `assessmentId` | - | `DeleteAssessmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes an assessment in Audit Manager. |
| `DeleteAssessmentFramework` | `DELETE /assessmentFrameworks/{frameworkId}` | - | `frameworkId` | - | `DeleteAssessmentFrameworkResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes a custom framework in Audit Manager. |
| `DeleteAssessmentFrameworkShare` | `DELETE /assessmentFrameworkShareRequests/{requestId}` | - | `requestId`, `requestType` | - | `DeleteAssessmentFrameworkShareResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes a share request for a custom framework in Audit Manager. |
| `DeleteAssessmentReport` | `DELETE /assessments/{assessmentId}/reports/{assessmentReportId}` | - | `assessmentId`, `assessmentReportId` | - | `DeleteAssessmentReportResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes an assessment report in Audit Manager. When you run the `DeleteAssessmentReport` operation, Audit Manager attempts to delete the following data: The specified assessment report that’s stored in your S3 bucket The associated metadata that’s stored in... |
| `DeleteControl` | `DELETE /controls/{controlId}` | - | `controlId` | - | `DeleteControlResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes a custom control in Audit Manager. When you invoke this operation, the custom control is deleted from any frameworks or assessments that it’s currently part of. |
| `DeregisterAccount` | `POST /account/deregisterAccount` | - | - | - | `DeregisterAccountResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deregisters an account in Audit Manager. Before you deregister, you can use the UpdateSettings API operation to set your preferred data retention policy. |
| `DeregisterOrganizationAdminAccount` | `POST /account/deregisterOrganizationAdminAccount` | - | - | - | `DeregisterOrganizationAdminAccountResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes the specified Amazon Web Services account as a delegated administrator for Audit Manager. When you remove a delegated administrator from your Audit Manager settings, you continue to have access to the evidence that you previously collected under that... |
| `DisassociateAssessmentReportEvidenceFolder` | `PUT /assessments/{assessmentId}/disassociateFromAssessmentReport` | - | `assessmentId`, `evidenceFolderId` | - | `DisassociateAssessmentReportEvidenceFolderResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Disassociates an evidence folder from the specified assessment report in Audit Manager. |
| `GetAccountStatus` | `GET /account/status` | - | - | - | `GetAccountStatusResponse` | `InternalServerException` | Gets the registration status of an account in Audit Manager. |
| `GetAssessment` | `GET /assessments/{assessmentId}` | - | `assessmentId` | - | `GetAssessmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets information about a specified assessment. |
| `GetAssessmentFramework` | `GET /assessmentFrameworks/{frameworkId}` | - | `frameworkId` | - | `GetAssessmentFrameworkResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets information about a specified framework. |
| `GetAssessmentReportUrl` | `GET /assessments/{assessmentId}/reports/{assessmentReportId}/url` | - | `assessmentId`, `assessmentReportId` | - | `GetAssessmentReportUrlResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets the URL of an assessment report in Audit Manager. |
| `GetChangeLogs` | `GET /assessments/{assessmentId}/changelogs` | `paginated` | `assessmentId` | - | `GetChangeLogsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets a list of changelogs from Audit Manager. |
| `GetControl` | `GET /controls/{controlId}` | - | `controlId` | - | `GetControlResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets information about a specified control. |
| `GetDelegations` | `GET /delegations` | `paginated` | - | - | `GetDelegationsResponse` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Gets a list of delegations from an audit owner to a delegate. |
| `GetEvidence` | `GET /assessments/{assessmentId}/controlSets/{controlSetId}/evidenceFolders/{evidenceFolderId}/evidence/{evidenceId}` | - | `assessmentId`, `controlSetId`, `evidenceFolderId`, `evidenceId` | - | `GetEvidenceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets information about a specified evidence item. |
| `GetEvidenceByEvidenceFolder` | `GET /assessments/{assessmentId}/controlSets/{controlSetId}/evidenceFolders/{evidenceFolderId}/evidence` | `paginated` | `assessmentId`, `controlSetId`, `evidenceFolderId` | - | `GetEvidenceByEvidenceFolderResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets all evidence from a specified evidence folder in Audit Manager. |
| `GetEvidenceFileUploadUrl` | `GET /evidenceFileUploadUrl` | - | `fileName` | - | `GetEvidenceFileUploadUrlResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a presigned Amazon S3 URL that can be used to upload a file as manual evidence. For instructions on how to use this operation, see Upload a file from your browser in the Audit Manager User Guide . |
| `GetEvidenceFolder` | `GET /assessments/{assessmentId}/controlSets/{controlSetId}/evidenceFolders/{evidenceFolderId}` | - | `assessmentId`, `controlSetId`, `evidenceFolderId` | - | `GetEvidenceFolderResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets an evidence folder from a specified assessment in Audit Manager. |
| `GetEvidenceFoldersByAssessment` | `GET /assessments/{assessmentId}/evidenceFolders` | `paginated` | `assessmentId` | - | `GetEvidenceFoldersByAssessmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets the evidence folders from a specified assessment in Audit Manager. |
| `GetEvidenceFoldersByAssessmentControl` | `GET /assessments/{assessmentId}/evidenceFolders-by-assessment-control/{controlSetId}/{controlId}` | `paginated` | `assessmentId`, `controlId`, `controlSetId` | - | `GetEvidenceFoldersByAssessmentControlResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets a list of evidence folders that are associated with a specified control in an Audit Manager assessment. |
| `GetInsights` | `GET /insights` | - | - | - | `GetInsightsResponse` | `AccessDeniedException`, `InternalServerException` | Gets the latest analytics data for all your current active assessments. |
| `GetInsightsByAssessment` | `GET /insights/assessments/{assessmentId}` | - | `assessmentId` | - | `GetInsightsByAssessmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets the latest analytics data for a specific active assessment. |
| `GetOrganizationAdminAccount` | `GET /account/organizationAdminAccount` | - | - | - | `GetOrganizationAdminAccountResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets the name of the delegated Amazon Web Services administrator account for a specified organization. |
| `GetServicesInScope` | `GET /services` | - | - | - | `GetServicesInScopeResponse` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Gets a list of the Amazon Web Services services from which Audit Manager can collect evidence. Audit Manager defines which Amazon Web Services services are in scope for an assessment. |
| `GetSettings` | `GET /settings/{attribute}` | - | `attribute` | - | `GetSettingsResponse` | `AccessDeniedException`, `InternalServerException` | Gets the settings for a specified Amazon Web Services account. |
| `ListAssessmentControlInsightsByControlDomain` | `GET /insights/controls-by-assessment` | `paginated` | `assessmentId`, `controlDomainId` | - | `ListAssessmentControlInsightsByControlDomainResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the latest analytics data for controls within a specific control domain and a specific active assessment. Control insights are listed only if the control belongs to the control domain and assessment that was specified. |
| `ListAssessmentFrameworkShareRequests` | `GET /assessmentFrameworkShareRequests` | `paginated` | `requestType` | - | `ListAssessmentFrameworkShareRequestsResponse` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Returns a list of sent or received share requests for custom frameworks in Audit Manager. |
| `ListAssessmentFrameworks` | `GET /assessmentFrameworks` | `paginated` | `frameworkType` | - | `ListAssessmentFrameworksResponse` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Returns a list of the frameworks that are available in the Audit Manager framework library. |
| `ListAssessmentReports` | `GET /assessmentReports` | `paginated` | - | - | `ListAssessmentReportsResponse` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Returns a list of assessment reports created in Audit Manager. |
| `ListAssessments` | `GET /assessments` | `paginated` | - | - | `ListAssessmentsResponse` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Returns a list of current and past assessments from Audit Manager. |
| `ListControlDomainInsights` | `GET /insights/control-domains` | `paginated` | - | - | `ListControlDomainInsightsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the latest analytics data for control domains across all of your active assessments. Audit Manager supports the control domains that are provided by Amazon Web Services Control Catalog. |
| `ListControlDomainInsightsByAssessment` | `GET /insights/control-domains-by-assessment` | `paginated` | `assessmentId` | - | `ListControlDomainInsightsByAssessmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists analytics data for control domains within a specified active assessment. Audit Manager supports the control domains that are provided by Amazon Web Services Control Catalog. |
| `ListControlInsightsByControlDomain` | `GET /insights/controls` | `paginated` | `controlDomainId` | - | `ListControlInsightsByControlDomainResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the latest analytics data for controls within a specific control domain across all active assessments. Control insights are listed only if the control belongs to the control domain that was specified and the control collected evidence on the... |
| `ListControls` | `GET /controls` | `paginated` | `controlType` | - | `ListControlsResponse` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Returns a list of controls from Audit Manager. |
| `ListKeywordsForDataSource` | `GET /dataSourceKeywords` | `paginated` | `source` | - | `ListKeywordsForDataSourceResponse` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Returns a list of keywords that are pre-mapped to the specified control data source. |
| `ListNotifications` | `GET /notifications` | `paginated` | - | - | `ListNotificationsResponse` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Returns a list of all Audit Manager notifications. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns a list of tags for the specified resource in Audit Manager. |
| `RegisterAccount` | `POST /account/registerAccount` | - | - | - | `RegisterAccountResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables Audit Manager for the specified Amazon Web Services account. |
| `RegisterOrganizationAdminAccount` | `POST /account/registerOrganizationAdminAccount` | - | `adminAccountId` | - | `RegisterOrganizationAdminAccountResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables an Amazon Web Services account within the organization as the delegated administrator for Audit Manager. |
| `StartAssessmentFrameworkShare` | `POST /assessmentFrameworks/{frameworkId}/shareRequests` | - | `destinationAccount`, `destinationRegion`, `frameworkId` | - | `StartAssessmentFrameworkShareResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Creates a share request for a custom framework in Audit Manager. The share request specifies a recipient and notifies them that a custom framework is available. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Tags the specified resource in Audit Manager. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes a tag from a resource in Audit Manager. |
| `UpdateAssessment` | `PUT /assessments/{assessmentId}` | - | `assessmentId`, `scope` | - | `UpdateAssessmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Edits an Audit Manager assessment. |
| `UpdateAssessmentControl` | `PUT /assessments/{assessmentId}/controlSets/{controlSetId}/controls/{controlId}` | - | `assessmentId`, `controlId`, `controlSetId` | - | `UpdateAssessmentControlResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates a control within an assessment in Audit Manager. |
| `UpdateAssessmentControlSetStatus` | `PUT /assessments/{assessmentId}/controlSets/{controlSetId}/status` | - | `assessmentId`, `comment`, `controlSetId`, `status` | - | `UpdateAssessmentControlSetStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates the status of a control set in an Audit Manager assessment. |
| `UpdateAssessmentFramework` | `PUT /assessmentFrameworks/{frameworkId}` | - | `controlSets`, `frameworkId`, `name` | - | `UpdateAssessmentFrameworkResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Updates a custom framework in Audit Manager. |
| `UpdateAssessmentFrameworkShare` | `PUT /assessmentFrameworkShareRequests/{requestId}` | - | `action`, `requestId`, `requestType` | - | `UpdateAssessmentFrameworkShareResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Updates a share request for a custom framework in Audit Manager. |
| `UpdateAssessmentStatus` | `PUT /assessments/{assessmentId}/status` | - | `assessmentId`, `status` | - | `UpdateAssessmentStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Updates the status of an assessment in Audit Manager. |
| `UpdateControl` | `PUT /controls/{controlId}` | - | `controlId`, `controlMappingSources`, `name` | - | `UpdateControlResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates a custom control in Audit Manager. |
| `UpdateSettings` | `PUT /settings` | - | - | - | `UpdateSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Updates Audit Manager settings for the current account. |
| `ValidateAssessmentReportIntegrity` | `POST /assessmentReports/integrity` | - | `s3RelativePath` | - | `ValidateAssessmentReportIntegrityResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Validates the integrity of an assessment report in Audit Manager. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DeleteAssessmentFrameworkShare` | - | `requestType -> requestType` | - | - |
| `GetChangeLogs` | - | `controlSetId -> controlSetId`, `controlId -> controlId`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `GetDelegations` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `GetEvidenceByEvidenceFolder` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `GetEvidenceFileUploadUrl` | - | `fileName -> fileName` | - | - |
| `GetEvidenceFoldersByAssessment` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `GetEvidenceFoldersByAssessmentControl` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListAssessmentControlInsightsByControlDomain` | - | `controlDomainId -> controlDomainId`, `assessmentId -> assessmentId`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListAssessmentFrameworks` | - | `frameworkType -> frameworkType`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListAssessmentFrameworkShareRequests` | - | `requestType -> requestType`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListAssessmentReports` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListAssessments` | - | `status -> status`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListControlDomainInsights` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListControlDomainInsightsByAssessment` | - | `assessmentId -> assessmentId`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListControlInsightsByControlDomain` | - | `controlDomainId -> controlDomainId`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListControls` | - | `controlType -> controlType`, `nextToken -> nextToken`, `maxResults -> maxResults`, `controlCatalogId -> controlCatalogId` | - | - |
| `ListKeywordsForDataSource` | - | `source -> source`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListNotifications` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `message` | An internal service error occurred during the processing of your request. |
| `ValidationException` | `structure` | `fields`, `message`, `reason` | The request has invalid or missing parameters. |
| `AccessDeniedException` | `structure` | `message` | Your account isn't registered with Audit Manager. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The resource that's specified in the request can't be found. |
| `ServiceQuotaExceededException` | `structure` | `message` | You've reached your account quota for this resource type. |
| `ThrottlingException` | `structure` | `message` | The request was denied due to request throttling. |
| `AssociateAssessmentReportEvidenceFolderRequest` | `structure` | `assessmentId`, `evidenceFolderId` | - |
| `AssociateAssessmentReportEvidenceFolderResponse` | `structure` | - | - |
| `BatchAssociateAssessmentReportEvidenceRequest` | `structure` | `assessmentId`, `evidenceFolderId`, `evidenceIds` | - |
| `BatchAssociateAssessmentReportEvidenceResponse` | `structure` | `errors`, `evidenceIds` | - |
| `BatchCreateDelegationByAssessmentRequest` | `structure` | `assessmentId`, `createDelegationRequests` | - |
| `BatchCreateDelegationByAssessmentResponse` | `structure` | `delegations`, `errors` | - |
| `BatchDeleteDelegationByAssessmentRequest` | `structure` | `assessmentId`, `delegationIds` | - |
| `BatchDeleteDelegationByAssessmentResponse` | `structure` | `errors` | - |
| `BatchDisassociateAssessmentReportEvidenceRequest` | `structure` | `assessmentId`, `evidenceFolderId`, `evidenceIds` | - |
| `BatchDisassociateAssessmentReportEvidenceResponse` | `structure` | `errors`, `evidenceIds` | - |
| `BatchImportEvidenceToAssessmentControlRequest` | `structure` | `assessmentId`, `controlId`, `controlSetId`, `manualEvidence` | - |
| `BatchImportEvidenceToAssessmentControlResponse` | `structure` | `errors` | - |
| `CreateAssessmentRequest` | `structure` | `assessmentReportsDestination`, `description`, `frameworkId`, `name`, `roles`, `scope`, `tags` | - |
| `CreateAssessmentResponse` | `structure` | `assessment` | - |
| `CreateAssessmentFrameworkRequest` | `structure` | `complianceType`, `controlSets`, `description`, `name`, `tags` | - |
| `CreateAssessmentFrameworkResponse` | `structure` | `framework` | - |
| `CreateAssessmentReportRequest` | `structure` | `assessmentId`, `description`, `name`, `queryStatement` | - |
| `CreateAssessmentReportResponse` | `structure` | `assessmentReport` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
