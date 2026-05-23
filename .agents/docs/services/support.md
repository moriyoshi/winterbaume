# AWS Support

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services Support The Amazon Web Services Support API Reference is intended for programmers who need detailed information about the Amazon Web Services Support operations and data types. You can use the API to manage your support cases programmatically. The Amazon Web Services Support API uses HTTP methods that return results in JSON format. You must have a Business, Enterprise On-Ramp, or Enterprise Support plan to use the Amazon Web Services Support API. If you call the Amazon Web Services Support API from an account that doesn't have a Business, Enterprise On-Ramp, or Enterprise Support plan, the `SubscriptionRequiredException` error message appears.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-support/tests/scenario_test.rs`: open a support case, inspect it, resolve it, and verify its post-resolution state.
- Backported from `scenario_test.rs`: request Trusted Advisor refreshes for the same check and verify per-check refresh independence.
- Scenario insight from EC2: exercise account or service defaults for AWS Support by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: support case creation, communications, severity/category discovery, attachment sets, service metadata, Trusted Advisor checks, refresh status, and support-plan-aware behaviour.

## Service Identity and Protocol

- AWS model slug: `support`
- AWS SDK for Rust slug: `support`
- Model version: `2013-04-15`
- Model file: `vendor/api-models-aws/models/support/service/2013-04-15/support-2013-04-15.json`
- SDK ID: `Support`
- Endpoint prefix: `support`
- ARN namespace: `support`
- CloudFormation name: `Support`
- CloudTrail event source: `support.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (11), `Add` (2), `Create` (1), `Refresh` (1), `Resolve` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddAttachmentsToSet`, `AddCommunicationToCase`, `CreateCase`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAttachment`, `DescribeCases`, `DescribeCommunications`, `DescribeCreateCaseOptions`, `DescribeServices`, `DescribeSeverityLevels`, `DescribeSupportedLanguages`, `DescribeTrustedAdvisorCheckRefreshStatuses`, `DescribeTrustedAdvisorCheckResult`, `DescribeTrustedAdvisorCheckSummaries`, `DescribeTrustedAdvisorChecks`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- 16 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Describe

- Operations: `DescribeAttachment`, `DescribeCases`, `DescribeCommunications`, `DescribeCreateCaseOptions`, `DescribeServices`, `DescribeSeverityLevels`, `DescribeSupportedLanguages`, `DescribeTrustedAdvisorCheckRefreshStatuses`, `DescribeTrustedAdvisorCheckResult`, `DescribeTrustedAdvisorCheckSummaries`, `DescribeTrustedAdvisorChecks`
- Traits: `paginated` (2)
- Common required input members in this group: `attachmentId`, `caseId`, `categoryCode`, `checkId`, `checkIds`, `issueType`, `language`, `serviceCode`

### Add

- Operations: `AddAttachmentsToSet`, `AddCommunicationToCase`
- Common required input members in this group: `attachments`, `communicationBody`

### Create

- Operations: `CreateCase`
- Common required input members in this group: `communicationBody`, `subject`

### Refresh

- Operations: `RefreshTrustedAdvisorCheck`
- Common required input members in this group: `checkId`

### Resolve

- Operations: `ResolveCase`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddAttachmentsToSet` | - | - | `attachments` | - | `AddAttachmentsToSetResponse` | `AttachmentLimitExceeded`, `AttachmentSetExpired`, `AttachmentSetIdNotFound`, `AttachmentSetSizeLimitExceeded`, `InternalServerError` | Adds one or more attachments to an attachment set. An attachment set is a temporary container for attachments that you add to a case or case communication. |
| `AddCommunicationToCase` | - | - | `communicationBody` | - | `AddCommunicationToCaseResponse` | `AttachmentSetExpired`, `AttachmentSetIdNotFound`, `CaseIdNotFound`, `InternalServerError` | Adds additional customer communication to an Amazon Web Services Support case. Use the `caseId` parameter to identify the case to which to add communication. |
| `CreateCase` | - | - | `communicationBody`, `subject` | - | `CreateCaseResponse` | `AttachmentSetExpired`, `AttachmentSetIdNotFound`, `CaseCreationLimitExceeded`, `InternalServerError` | Creates a case in the Amazon Web Services Support Center. This operation is similar to how you create a case in the Amazon Web Services Support Center Create Case page. |
| `DescribeAttachment` | - | - | `attachmentId` | - | `DescribeAttachmentResponse` | `AttachmentIdNotFound`, `DescribeAttachmentLimitExceeded`, `InternalServerError` | Returns the attachment that has the specified ID. Attachments can include screenshots, error logs, or other files that describe your issue. |
| `DescribeCases` | - | `paginated` | - | - | `DescribeCasesResponse` | `CaseIdNotFound`, `InternalServerError` | Returns a list of cases that you specify by passing one or more case IDs. You can use the `afterTime` and `beforeTime` parameters to filter the cases by date. |
| `DescribeCommunications` | - | `paginated` | `caseId` | - | `DescribeCommunicationsResponse` | `CaseIdNotFound`, `InternalServerError` | Returns communications and attachments for one or more support cases. Use the `afterTime` and `beforeTime` parameters to filter by date. |
| `DescribeCreateCaseOptions` | - | - | `categoryCode`, `issueType`, `language`, `serviceCode` | - | `DescribeCreateCaseOptionsResponse` | `InternalServerError`, `ThrottlingException` | Returns a list of CreateCaseOption types along with the corresponding supported hours and language availability. You can specify the `language` `categoryCode`, `issueType` and `serviceCode` used to retrieve the CreateCaseOptions. |
| `DescribeServices` | - | - | - | - | `DescribeServicesResponse` | `InternalServerError` | Returns the current list of Amazon Web Services services and a list of service categories for each service. You then use service names and categories in your CreateCase requests. |
| `DescribeSeverityLevels` | - | - | - | - | `DescribeSeverityLevelsResponse` | `InternalServerError` | Returns the list of severity levels that you can assign to a support case. The severity level for a case is also a field in the CaseDetails data type that you include for a CreateCase request. |
| `DescribeSupportedLanguages` | - | - | `categoryCode`, `issueType`, `serviceCode` | - | `DescribeSupportedLanguagesResponse` | `InternalServerError`, `ThrottlingException` | Returns a list of supported languages for a specified `categoryCode`, `issueType` and `serviceCode`. The returned supported languages will include a ISO 639-1 code for the `language`, and the language display name. |
| `DescribeTrustedAdvisorCheckRefreshStatuses` | - | - | `checkIds` | - | `DescribeTrustedAdvisorCheckRefreshStatusesResponse` | `InternalServerError`, `ThrottlingException` | Returns the refresh status of the Trusted Advisor checks that have the specified check IDs. You can get the check IDs by calling the DescribeTrustedAdvisorChecks operation. |
| `DescribeTrustedAdvisorCheckResult` | - | - | `checkId` | - | `DescribeTrustedAdvisorCheckResultResponse` | `InternalServerError`, `ThrottlingException` | Returns the results of the Trusted Advisor check that has the specified check ID. You can get the check IDs by calling the DescribeTrustedAdvisorChecks operation. |
| `DescribeTrustedAdvisorCheckSummaries` | - | - | `checkIds` | - | `DescribeTrustedAdvisorCheckSummariesResponse` | `InternalServerError`, `ThrottlingException` | Returns the results for the Trusted Advisor check summaries for the check IDs that you specified. You can get the check IDs by calling the DescribeTrustedAdvisorChecks operation. |
| `DescribeTrustedAdvisorChecks` | - | - | `language` | - | `DescribeTrustedAdvisorChecksResponse` | `InternalServerError`, `ThrottlingException` | Returns information about all available Trusted Advisor checks, including the name, ID, category, description, and metadata. You must specify a language code. |
| `RefreshTrustedAdvisorCheck` | - | - | `checkId` | - | `RefreshTrustedAdvisorCheckResponse` | `InternalServerError` | Refreshes the Trusted Advisor check that you specify using the check ID. You can get the check IDs by calling the DescribeTrustedAdvisorChecks operation. |
| `ResolveCase` | - | - | - | - | `ResolveCaseResponse` | `CaseIdNotFound`, `InternalServerError` | Resolves a support case. This operation takes a `caseId` and returns the initial and final state of the case. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerError` | `structure` | `message` | An internal server error occurred. |
| `ThrottlingException` | `structure` | `message` | You have exceeded the maximum allowed TPS (Transactions Per Second) for the operations. |
| `CaseIdNotFound` | `structure` | `message` | The requested `caseId` couldn't be located. |
| `AttachmentSetExpired` | `structure` | `message` | The expiration time of the attachment set has passed. |
| `AttachmentSetIdNotFound` | `structure` | `message` | An attachment set with the specified ID could not be found. |
| `AddAttachmentsToSetRequest` | `structure` | `attachmentSetId`, `attachments` | - |
| `AddAttachmentsToSetResponse` | `structure` | `attachmentSetId`, `expiryTime` | The ID and expiry time of the attachment set returned by the AddAttachmentsToSet operation. |
| `AttachmentLimitExceeded` | `structure` | `message` | The limit for the number of attachment sets created in a short period of time has been exceeded. |
| `AttachmentSetSizeLimitExceeded` | `structure` | `message` | A limit for the size of an attachment set has been exceeded. |
| `AddCommunicationToCaseRequest` | `structure` | `attachmentSetId`, `caseId`, `ccEmailAddresses`, `communicationBody` | - |
| `AddCommunicationToCaseResponse` | `structure` | `result` | The result of the AddCommunicationToCase operation. |
| `CreateCaseRequest` | `structure` | `attachmentSetId`, `categoryCode`, `ccEmailAddresses`, `communicationBody`, `issueType`, `language`, `serviceCode`, `severityCode`, `subject` | - |
| `CreateCaseResponse` | `structure` | `caseId` | The support case ID returned by a successful completion of the CreateCase operation. |
| `CaseCreationLimitExceeded` | `structure` | `message` | The case creation limit for the account has been exceeded. |
| `DescribeAttachmentRequest` | `structure` | `attachmentId` | - |
| `DescribeAttachmentResponse` | `structure` | `attachment` | The content and file name of the attachment returned by the DescribeAttachment operation. |
| `AttachmentIdNotFound` | `structure` | `message` | An attachment with the specified ID could not be found. |
| `DescribeAttachmentLimitExceeded` | `structure` | `message` | The limit for the number of DescribeAttachment requests in a short period of time has been exceeded. |
| `DescribeCasesRequest` | `structure` | `afterTime`, `beforeTime`, `caseIdList`, `displayId`, `includeCommunications`, `includeResolvedCases`, `language`, `maxResults`, `nextToken` | - |
| `DescribeCasesResponse` | `structure` | `cases`, `nextToken` | Returns an array of CaseDetails objects and a `nextToken` that defines a point for pagination in the result set. |
| `DescribeCommunicationsRequest` | `structure` | `afterTime`, `beforeTime`, `caseId`, `maxResults`, `nextToken` | - |
| `DescribeCommunicationsResponse` | `structure` | `communications`, `nextToken` | The communications returned by the DescribeCommunications operation. |
| `DescribeCreateCaseOptionsRequest` | `structure` | `categoryCode`, `issueType`, `language`, `serviceCode` | - |
| `DescribeCreateCaseOptionsResponse` | `structure` | `communicationTypes`, `languageAvailability` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
