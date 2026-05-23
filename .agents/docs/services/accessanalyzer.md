# Access Analyzer

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Identity and Access Management Access Analyzer helps you to set, verify, and refine your IAM policies by providing a suite of capabilities. Its features include findings for external, internal, and unused access, basic and custom policy checks for validating policies, and policy generation to generate fine-grained policies. To start using IAM Access Analyzer to identify external, internal, or unused access, you first need to create an analyzer. External access analyzers help you identify potential risks of accessing resources by enabling you to identify any resource policies that grant access to an external principal. It does this by using logic-based reasoning to analyze resource-based policies in your Amazon Web Services environment.

## Possible Usage Scenarios
- From the AWS documentation and model: analyse IAM and resource policies to identify external access, unused access, and findings that need review or archive rules.
- From the operation surface: manage analysers, archive rules, policy checks, generated policies, findings, and tags for access-governance workflows.

## Service Identity and Protocol

- AWS model slug: `accessanalyzer`
- AWS SDK for Rust slug: `accessanalyzer`
- Model version: `2019-11-01`
- Model file: `vendor/api-models-aws/models/accessanalyzer/service/2019-11-01/accessanalyzer-2019-11-01.json`
- SDK ID: `AccessAnalyzer`
- Endpoint prefix: `-`
- ARN namespace: `access-analyzer`
- CloudFormation name: `-`
- CloudTrail event source: `access-analyzer.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (9), `List` (9), `Check` (3), `Create` (3), `Update` (3), `Delete` (2), `Start` (2), `Apply` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelPolicyGeneration`, `CreateAccessPreview`, `CreateAnalyzer`, `CreateArchiveRule`, `DeleteAnalyzer`, `DeleteArchiveRule`, `StartPolicyGeneration`, `StartResourceScan`, `TagResource`, `UntagResource`, `UpdateAnalyzer`, `UpdateArchiveRule`, `UpdateFindings`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `CheckAccessNotGranted`, `CheckNoNewAccess`, `CheckNoPublicAccess`, `GetAccessPreview`, `GetAnalyzedResource`, `GetAnalyzer`, `GetArchiveRule`, `GetFinding`, `GetFindingRecommendation`, `GetFindingV2`, `GetFindingsStatistics`, `GetGeneratedPolicy`, `ListAccessPreviewFindings`, `ListAccessPreviews`, `ListAnalyzedResources`, `ListAnalyzers`, `ListArchiveRules`, `ListFindings`, `ListFindingsV2`, `ListPolicyGenerations`, ... (+2).
- Pagination is modelled for 11 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 13 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelPolicyGeneration`, `StartPolicyGeneration`, `StartResourceScan`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 37 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `SNS`, `SQS`, `EC2/VPC`, `ECR`, `ECS`, `RDS`, `Secrets Manager`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `Analyzer` | `analyzerName` | put: `CreateAnalyzer`; read: `GetAnalyzer`; update: `UpdateAnalyzer`; delete: `DeleteAnalyzer`; list: `ListAnalyzers` | - | - |
| `ArchiveRule` | `analyzerName`, `ruleName` | put: `CreateArchiveRule`; read: `GetArchiveRule`; update: `UpdateArchiveRule`; delete: `DeleteArchiveRule`; list: `ListArchiveRules` | - | - |

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/IAM/latest/UserGuide/what-is-access-analyzer.html
- https://docs.aws.amazon.com/IAM/latest/UserGuide/access-analyzer-policy-validation.html
- https://docs.aws.amazon.com/IAM/latest/UserGuide/access-analyzer-findings.html

Research outcomes:
- IAM Access Analyzer creates an analyzer for an account or organisation. That account or organisation is the analyzer's zone of trust.
- External-access analyzers use logic-based reasoning over supported resource-based policies. Access by principals outside the zone of trust produces findings.
- External entities can include another AWS account, a root user, an IAM user or role, a federated user, anonymous users, or other principal forms supported by policy `Principal`.
- For external access, IAM Access Analyzer analyses only supported resources in the Region where the analyzer is enabled. Monitoring all Regions requires an analyzer in each Region.
- Policy additions or changes are normally analysed within about 30 minutes, and periodic scanning happens even without a change notification. Some conditions can delay generated or resolved findings up to 24 hours.
- Findings include the resource, external principal, and granted permissions. Users can rescan a resource with `StartResourceScan`.
- Internal-access analyzers identify which principals inside the account or organisation can access specified business-critical resources.
- Unused-access analyzers continuously monitor IAM roles and users and generate findings for unused roles, unused access keys, unused passwords, and unused services or actions.
- Policy validation checks policy grammar and AWS best practices and returns security warnings, errors, general warnings, and suggestions.
- Policy generation uses CloudTrail activity for a selected IAM user or role and time range to generate a least-privilege policy proposal.

Parity implications:
- Model analyzers, zone of trust, analyzer type, supported resource scope, findings, archive state, archive rules, resource scans, policy checks, and generated-policy jobs separately.
- Finding creation and resolution should be asynchronous and Region-scoped for external-access analysis.
- Archive rules should suppress or archive matching findings without changing the underlying resource policy.

## Operation Groups

### Get

- Operations: `GetAccessPreview`, `GetAnalyzedResource`, `GetFinding`, `GetFindingRecommendation`, `GetFindingsStatistics`, `GetFindingV2`, `GetGeneratedPolicy`
- Traits: `readonly` (7), `paginated` (2)
- Common required input members in this group: `analyzerArn`, `id`

### List

- Operations: `ListAccessPreviewFindings`, `ListAccessPreviews`, `ListAnalyzedResources`, `ListFindings`, `ListFindingsV2`, `ListPolicyGenerations`, `ListTagsForResource`
- Traits: `readonly` (7), `paginated` (6)
- Common required input members in this group: `analyzerArn`

### Check

- Operations: `CheckAccessNotGranted`, `CheckNoNewAccess`, `CheckNoPublicAccess`
- Traits: `readonly` (3)
- Common required input members in this group: `policyDocument`, `policyType`

### Start

- Operations: `StartPolicyGeneration`, `StartResourceScan`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Apply

- Operations: `ApplyArchiveRule`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Cancel

- Operations: `CancelPolicyGeneration`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Create

- Operations: `CreateAccessPreview`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Generate

- Operations: `GenerateFindingRecommendation`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Update

- Operations: `UpdateFindings`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Validate

- Operations: `ValidatePolicy`
- Traits: `readonly` (1), `paginated` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ApplyArchiveRule` | `PUT /archive-rule` | `idempotent`, `idempotency-token` | `analyzerArn`, `ruleName` | `clientToken` | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retroactively applies the archive rule to existing findings that meet the archive rule criteria. |
| `CancelPolicyGeneration` | `PUT /policy/generation/{jobId}` | `idempotent` | `jobId` | - | `CancelPolicyGenerationResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Cancels the requested policy generation. |
| `CheckAccessNotGranted` | `POST /policy/check-access-not-granted` | `readonly` | `policyDocument`, `access`, `policyType` | - | `CheckAccessNotGrantedResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterException`, `ThrottlingException`, `UnprocessableEntityException`, `ValidationException` | Checks whether the specified access isn't allowed by a policy. |
| `CheckNoNewAccess` | `POST /policy/check-no-new-access` | `readonly` | `newPolicyDocument`, `existingPolicyDocument`, `policyType` | - | `CheckNoNewAccessResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterException`, `ThrottlingException`, `UnprocessableEntityException`, `ValidationException` | Checks whether new access is allowed for an updated policy when compared to the existing policy. You can find examples for reference policies and learn how to set up and run a custom policy check for new access in th ... |
| `CheckNoPublicAccess` | `POST /policy/check-no-public-access` | `readonly` | `policyDocument`, `resourceType` | - | `CheckNoPublicAccessResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidParameterException`, `ThrottlingException`, `UnprocessableEntityException`, `ValidationException` | Checks whether a resource policy can grant public access to the specified resource type. |
| `CreateAccessPreview` | `PUT /access-preview` | `idempotent`, `idempotency-token` | `analyzerArn`, `configurations` | `clientToken` | `CreateAccessPreviewResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an access preview that allows you to preview IAM Access Analyzer findings for your resource before deploying resource permissions. |
| `GenerateFindingRecommendation` | `POST /recommendation/{id}` | - | `analyzerArn`, `id` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a recommendation for an unused permissions finding. |
| `GetAccessPreview` | `GET /access-preview/{accessPreviewId}` | `readonly` | `accessPreviewId`, `analyzerArn` | - | `GetAccessPreviewResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about an access preview for the specified analyzer. |
| `GetAnalyzedResource` | `GET /analyzed-resource` | `readonly` | `analyzerArn`, `resourceArn` | - | `GetAnalyzedResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a resource that was analyzed. This action is supported only for external access analyzers. |
| `GetFinding` | `GET /finding/{id}` | `readonly` | `analyzerArn`, `id` | - | `GetFindingResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the specified finding. GetFinding and GetFindingV2 both use access-analyzer:GetFinding in the Action element of an IAM policy statement. You must have permission to perform the access-anal ... |
| `GetFindingRecommendation` | `GET /recommendation/{id}` | `readonly`, `paginated` | `analyzerArn`, `id` | - | `GetFindingRecommendationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a finding recommendation for the specified analyzer. |
| `GetFindingsStatistics` | `POST /analyzer/findings/statistics` | `readonly` | `analyzerArn` | - | `GetFindingsStatisticsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of aggregated finding statistics for an external access or unused access analyzer. |
| `GetFindingV2` | `GET /findingv2/{id}` | `readonly`, `paginated` | `analyzerArn`, `id` | - | `GetFindingV2Response` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the specified finding. GetFinding and GetFindingV2 both use access-analyzer:GetFinding in the Action element of an IAM policy statement. You must have permission to perform the access-anal ... |
| `GetGeneratedPolicy` | `GET /policy/generation/{jobId}` | `readonly` | `jobId` | - | `GetGeneratedPolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves the policy that was generated using StartPolicyGeneration . |
| `ListAccessPreviewFindings` | `POST /access-preview/{accessPreviewId}` | `readonly`, `paginated` | `accessPreviewId`, `analyzerArn` | - | `ListAccessPreviewFindingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of access preview findings generated by the specified access preview. |
| `ListAccessPreviews` | `GET /access-preview` | `readonly`, `paginated` | `analyzerArn` | - | `ListAccessPreviewsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of access previews for the specified analyzer. |
| `ListAnalyzedResources` | `POST /analyzed-resource` | `readonly`, `paginated` | `analyzerArn` | - | `ListAnalyzedResourcesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of resources of the specified type that have been analyzed by the specified analyzer. |
| `ListFindings` | `POST /finding` | `readonly`, `paginated` | `analyzerArn` | - | `ListFindingsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of findings generated by the specified analyzer. ListFindings and ListFindingsV2 both use access-analyzer:ListFindings in the Action element of an IAM policy statement. You must have permission to pe ... |
| `ListFindingsV2` | `POST /findingv2` | `readonly`, `paginated` | `analyzerArn` | - | `ListFindingsV2Response` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of findings generated by the specified analyzer. ListFindings and ListFindingsV2 both use access-analyzer:ListFindings in the Action element of an IAM policy statement. You must have permission to pe ... |
| `ListPolicyGenerations` | `GET /policy/generation` | `readonly`, `paginated` | - | - | `ListPolicyGenerationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all of the policy generations requested in the last seven days. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of tags applied to the specified resource. |
| `StartPolicyGeneration` | `PUT /policy/generation` | `idempotent`, `idempotency-token` | `policyGenerationDetails` | `clientToken` | `StartPolicyGenerationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Starts the policy generation request. |
| `StartResourceScan` | `POST /resource/scan` | - | `analyzerArn`, `resourceArn` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Immediately starts a scan of the policies applied to the specified resource. This action is supported only for external access analyzers. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds a tag to the specified resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a tag from the specified resource. |
| `UpdateFindings` | `PUT /finding` | `idempotent`, `idempotency-token` | `analyzerArn`, `status` | `clientToken` | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the status for the specified findings. |
| `ValidatePolicy` | `POST /policy/validation` | `readonly`, `paginated` | `policyDocument`, `policyType` | - | `ValidatePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Requests the validation of a policy and returns a list of findings. The findings help you identify issues and provide actionable recommendations to resolve the issue and enable you to author functional policies that ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `GenerateFindingRecommendation` | - | `analyzerArn -> analyzerArn` | - | - |
| `GetAccessPreview` | - | `analyzerArn -> analyzerArn` | - | - |
| `GetAnalyzedResource` | - | `analyzerArn -> analyzerArn`, `resourceArn -> resourceArn` | - | - |
| `GetFinding` | - | `analyzerArn -> analyzerArn` | - | - |
| `GetFindingRecommendation` | - | `analyzerArn -> analyzerArn`, `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `GetFindingV2` | - | `analyzerArn -> analyzerArn`, `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `GetGeneratedPolicy` | - | `includeResourcePlaceholders -> includeResourcePlaceholders`, `includeServiceLevelTemplate -> includeServiceLevelTemplate` | - | - |
| `ListAccessPreviews` | - | `analyzerArn -> analyzerArn`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListPolicyGenerations` | - | `principalArn -> principalArn`, `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |
| `ValidatePolicy` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message, resourceId, resourceType | A conflict exception error. |
| `InternalServerException` | `structure` | message, retryAfterSeconds | Internal server error. |
| `InvalidParameterException` | `structure` | message | The specified parameter is invalid. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | The specified resource could not be found. |
| `ServiceQuotaExceededException` | `structure` | message, resourceId, resourceType | Service quote met error. |
| `ThrottlingException` | `structure` | message, retryAfterSeconds | Throttling limit exceeded error. |
| `UnprocessableEntityException` | `structure` | message | The specified entity could not be processed. |
| `ValidationException` | `structure` | message, reason, fieldList | Validation exception error. |
| `ApplyArchiveRuleRequest` | `structure` | analyzerArn, ruleName, clientToken | Retroactively applies an archive rule. |
| `CancelPolicyGenerationRequest` | `structure` | jobId | - |
| `CancelPolicyGenerationResponse` | `structure` | **empty (no members)** | - |
| `CheckAccessNotGrantedRequest` | `structure` | policyDocument, access, policyType | - |
| `CheckAccessNotGrantedResponse` | `structure` | result, message, reasons | - |
| `CheckNoNewAccessRequest` | `structure` | newPolicyDocument, existingPolicyDocument, policyType | - |
| `CheckNoNewAccessResponse` | `structure` | result, message, reasons | - |
| `CheckNoPublicAccessRequest` | `structure` | policyDocument, resourceType | - |
| `CheckNoPublicAccessResponse` | `structure` | result, message, reasons | - |
| `CreateAccessPreviewRequest` | `structure` | analyzerArn, configurations, clientToken | - |
| `CreateAccessPreviewResponse` | `structure` | id | - |
| `GenerateFindingRecommendationRequest` | `structure` | analyzerArn, id | - |
| `GetAccessPreviewRequest` | `structure` | accessPreviewId, analyzerArn | - |
| `GetAccessPreviewResponse` | `structure` | accessPreview | - |
| `GetAnalyzedResourceRequest` | `structure` | analyzerArn, resourceArn | Retrieves an analyzed resource. |
| `GetAnalyzedResourceResponse` | `structure` | resource | The response to the request. |
| `GetFindingRequest` | `structure` | analyzerArn, id | Retrieves a finding. |
| `GetFindingResponse` | `structure` | finding | The response to the request. |
| `GetFindingRecommendationRequest` | `structure` | analyzerArn, id, maxResults, nextToken | - |
| `GetFindingRecommendationResponse` | `structure` | startedAt, completedAt, nextToken, error, resourceArn, recommendedSteps, recommendationType, status | - |
| `GetFindingsStatisticsRequest` | `structure` | analyzerArn | - |
| `GetFindingsStatisticsResponse` | `structure` | findingsStatistics, lastUpdatedAt | - |
| `GetFindingV2Request` | `structure` | analyzerArn, id, maxResults, nextToken | - |
| `GetFindingV2Response` | `structure` | analyzedAt, createdAt, error, id, nextToken, resource, resourceType, resourceOwnerAccount, status, updatedAt, findingDetails, findingType | - |
| `GetGeneratedPolicyRequest` | `structure` | jobId, includeResourcePlaceholders, includeServiceLevelTemplate | - |
| `GetGeneratedPolicyResponse` | `structure` | jobDetails, generatedPolicyResult | - |
| `ListAccessPreviewFindingsRequest` | `structure` | accessPreviewId, analyzerArn, filter, nextToken, maxResults | - |
| `ListAccessPreviewFindingsResponse` | `structure` | findings, nextToken | - |
| `ListAccessPreviewsRequest` | `structure` | analyzerArn, nextToken, maxResults | - |
| `ListAccessPreviewsResponse` | `structure` | accessPreviews, nextToken | - |
| `ListAnalyzedResourcesRequest` | `structure` | analyzerArn, resourceType, nextToken, maxResults | Retrieves a list of resources that have been analyzed. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
