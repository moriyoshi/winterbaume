# AWS Well-Architected Tool

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Well-Architected Tool This is the Well-Architected Tool API Reference . The WA Tool API provides programmatic access to the Well-Architected Tool in the Amazon Web Services Management Console. For information about the Well-Architected Tool, see the Well-Architected Tool User Guide.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Well-Architected Tool where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS Well-Architected Tool by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented AWS Well-Architected Tool workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Update`, `Create`, `Delete` operation families, including `ListAnswers`, `ListCheckDetails`, `ListCheckSummaries`, `ListLensReviewImprovements`, `GetAnswer`, `GetConsolidatedReport`.

## Service Identity and Protocol

- AWS model slug: `wellarchitected`
- AWS SDK for Rust slug: `wellarchitected`
- Model version: `2020-03-31`
- Model file: `vendor/api-models-aws/models/wellarchitected/service/2020-03-31/wellarchitected-2020-03-31.json`
- SDK ID: `WellArchitected`
- Endpoint prefix: `wellarchitected`
- ARN namespace: `wellarchitected`
- CloudFormation name: `WellArchitected`
- CloudTrail event source: `wellarchitected.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (19), `Get` (14), `Update` (11), `Create` (9), `Delete` (8), `Upgrade` (3), `Associate` (2), `Disassociate` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateLenses`, `AssociateProfiles`, `CreateLensShare`, `CreateLensVersion`, `CreateMilestone`, `CreateProfile`, `CreateProfileShare`, `CreateReviewTemplate`, `CreateTemplateShare`, `CreateWorkload`, `CreateWorkloadShare`, `DeleteLens`, `DeleteLensShare`, `DeleteProfile`, `DeleteProfileShare`, `DeleteReviewTemplate`, `DeleteTemplateShare`, `DeleteWorkload`, `DeleteWorkloadShare`, `DisassociateLenses`, ... (+15).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAnswer`, `GetConsolidatedReport`, `GetGlobalSettings`, `GetLens`, `GetLensReview`, `GetLensReviewReport`, `GetLensVersionDifference`, `GetMilestone`, `GetProfile`, `GetProfileTemplate`, `GetReviewTemplate`, `GetReviewTemplateAnswer`, `GetReviewTemplateLensReview`, `GetWorkload`, `ListAnswers`, `ListCheckDetails`, `ListCheckSummaries`, `ListLensReviewImprovements`, `ListLensReviews`, `ListLensShares`, ... (+13).
- Pagination is modelled for 19 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 20 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `ExportLens`, `GetConsolidatedReport`, `GetLensReviewReport`, `ImportLens`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 72 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `CloudWatch`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListAnswers`, `ListCheckDetails`, `ListCheckSummaries`, `ListLenses`, `ListLensReviewImprovements`, `ListLensReviews`, `ListLensShares`, `ListMilestones`, `ListNotifications`, `ListProfileNotifications`, `ListProfiles`, `ListProfileShares`, `ListReviewTemplateAnswers`, `ListReviewTemplates`, `ListShareInvitations`, `ListTagsForResource`, `ListTemplateShares`, `ListWorkloads`, `ListWorkloadShares`
- Traits: `paginated` (18)
- Common required input members in this group: `WorkloadId`, `LensAlias`, `LensArn`, `PillarId`, `QuestionId`, `ChoiceId`, `TemplateArn`

### Get

- Operations: `GetAnswer`, `GetConsolidatedReport`, `GetGlobalSettings`, `GetLens`, `GetLensReview`, `GetLensReviewReport`, `GetLensVersionDifference`, `GetMilestone`, `GetProfile`, `GetProfileTemplate`, `GetReviewTemplate`, `GetReviewTemplateAnswer`, `GetReviewTemplateLensReview`, `GetWorkload`
- Traits: `paginated` (1)
- Common required input members in this group: `WorkloadId`, `LensAlias`, `QuestionId`, `TemplateArn`

### Update

- Operations: `UpdateAnswer`, `UpdateGlobalSettings`, `UpdateIntegration`, `UpdateLensReview`, `UpdateProfile`, `UpdateReviewTemplate`, `UpdateReviewTemplateAnswer`, `UpdateReviewTemplateLensReview`, `UpdateShareInvitation`, `UpdateWorkload`, `UpdateWorkloadShare`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `WorkloadId`, `LensAlias`, `QuestionId`, `TemplateArn`

### Create

- Operations: `CreateLensShare`, `CreateLensVersion`, `CreateMilestone`, `CreateProfile`, `CreateProfileShare`, `CreateReviewTemplate`, `CreateTemplateShare`, `CreateWorkload`, `CreateWorkloadShare`
- Traits: `idempotency-token` (9)
- Common required input members in this group: `LensAlias`, `SharedWith`, `ClientRequestToken`, `WorkloadId`, `Description`, `Lenses`

### Delete

- Operations: `DeleteLens`, `DeleteLensShare`, `DeleteProfile`, `DeleteProfileShare`, `DeleteReviewTemplate`, `DeleteTemplateShare`, `DeleteWorkload`, `DeleteWorkloadShare`
- Traits: `idempotency-token` (8)
- Common required input members in this group: `LensAlias`, `ClientRequestToken`, `ShareId`, `ProfileArn`, `TemplateArn`, `WorkloadId`

### Upgrade

- Operations: `UpgradeLensReview`, `UpgradeProfileVersion`, `UpgradeReviewTemplateLensReview`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `WorkloadId`, `LensAlias`

### Associate

- Operations: `AssociateLenses`, `AssociateProfiles`
- Common required input members in this group: `WorkloadId`

### Disassociate

- Operations: `DisassociateLenses`, `DisassociateProfiles`
- Common required input members in this group: `WorkloadId`

### Export

- Operations: `ExportLens`
- Common required input members in this group: -

### Import

- Operations: `ImportLens`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateLenses` | `PATCH /workloads/{WorkloadId}/associateLenses` | - | `WorkloadId`, `LensAliases` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associate a lens to a workload. Up to 10 lenses can be associated with a workload in a single API operation. A maximum of 20 lenses can be associated with a workload. Disclaimer By accessing and/or applying custom le ... |
| `AssociateProfiles` | `PATCH /workloads/{WorkloadId}/associateProfiles` | - | `WorkloadId`, `ProfileArns` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associate a profile with a workload. |
| `CreateLensShare` | `POST /lenses/{LensAlias}/shares` | `idempotency-token` | `LensAlias`, `SharedWith`, `ClientRequestToken` | `ClientRequestToken` | `CreateLensShareOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a lens share. The owner of a lens can share it with other Amazon Web Services accounts, users, an organization, and organizational units (OUs) in the same Amazon Web Services Region. Lenses provided by Amazon ... |
| `CreateLensVersion` | `POST /lenses/{LensAlias}/versions` | `idempotency-token` | `LensAlias`, `LensVersion`, `ClientRequestToken` | `ClientRequestToken` | `CreateLensVersionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a new lens version. A lens can have up to 100 versions. Use this operation to publish a new lens version after you have imported a lens. The LensAlias is used to identify the lens to be published. The owner of ... |
| `CreateMilestone` | `POST /workloads/{WorkloadId}/milestones` | `idempotency-token` | `WorkloadId`, `MilestoneName`, `ClientRequestToken` | `ClientRequestToken` | `CreateMilestoneOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a milestone for an existing workload. |
| `CreateProfile` | `POST /profiles` | `idempotency-token` | `ProfileName`, `ProfileDescription`, `ProfileQuestions`, `ClientRequestToken` | `ClientRequestToken` | `CreateProfileOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a profile. |
| `CreateProfileShare` | `POST /profiles/{ProfileArn}/shares` | `idempotency-token` | `ProfileArn`, `SharedWith`, `ClientRequestToken` | `ClientRequestToken` | `CreateProfileShareOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a profile share. |
| `CreateReviewTemplate` | `POST /reviewTemplates` | `idempotency-token` | `TemplateName`, `Description`, `Lenses`, `ClientRequestToken` | `ClientRequestToken` | `CreateReviewTemplateOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a review template. Disclaimer Do not include or gather personal identifiable information (PII) of end users or other identifiable individuals in or via your review templates. If your review template or those s ... |
| `CreateTemplateShare` | `POST /templates/shares/{TemplateArn}` | `idempotency-token` | `TemplateArn`, `SharedWith`, `ClientRequestToken` | `ClientRequestToken` | `CreateTemplateShareOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a review template share. The owner of a review template can share it with other Amazon Web Services accounts, users, an organization, and organizational units (OUs) in the same Amazon Web Services Region. Shar ... |
| `CreateWorkload` | `POST /workloads` | `idempotency-token` | `WorkloadName`, `Description`, `Environment`, `Lenses`, `ClientRequestToken` | `ClientRequestToken` | `CreateWorkloadOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a new workload. The owner of a workload can share the workload with other Amazon Web Services accounts, users, an organization, and organizational units (OUs) in the same Amazon Web Services Region. Only the o ... |
| `CreateWorkloadShare` | `POST /workloads/{WorkloadId}/shares` | `idempotency-token` | `WorkloadId`, `SharedWith`, `PermissionType`, `ClientRequestToken` | `ClientRequestToken` | `CreateWorkloadShareOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a workload share. The owner of a workload can share it with other Amazon Web Services accounts and users in the same Amazon Web Services Region. Shared access to a workload is not removed until the workload in ... |
| `DeleteLens` | `DELETE /lenses/{LensAlias}` | `idempotency-token` | `LensAlias`, `ClientRequestToken`, `LensStatus` | `ClientRequestToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete an existing lens. Only the owner of a lens can delete it. After the lens is deleted, Amazon Web Services accounts and users that you shared the lens with can continue to use it, but they will no longer be able ... |
| `DeleteLensShare` | `DELETE /lenses/{LensAlias}/shares/{ShareId}` | `idempotency-token` | `ShareId`, `LensAlias`, `ClientRequestToken` | `ClientRequestToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete a lens share. After the lens share is deleted, Amazon Web Services accounts, users, organizations, and organizational units (OUs) that you shared the lens with can continue to use it, but they will no longer b ... |
| `DeleteProfile` | `DELETE /profiles/{ProfileArn}` | `idempotency-token` | `ProfileArn`, `ClientRequestToken` | `ClientRequestToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete a profile. Disclaimer By sharing your profile with other Amazon Web Services accounts, you acknowledge that Amazon Web Services will make your profile available to those other accounts. Those other accounts ma ... |
| `DeleteProfileShare` | `DELETE /profiles/{ProfileArn}/shares/{ShareId}` | `idempotency-token` | `ShareId`, `ProfileArn`, `ClientRequestToken` | `ClientRequestToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete a profile share. |
| `DeleteReviewTemplate` | `DELETE /reviewTemplates/{TemplateArn}` | `idempotency-token` | `TemplateArn`, `ClientRequestToken` | `ClientRequestToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete a review template. Only the owner of a review template can delete it. After the review template is deleted, Amazon Web Services accounts, users, organizations, and organizational units (OUs) that you shared th ... |
| `DeleteTemplateShare` | `DELETE /templates/shares/{TemplateArn}/{ShareId}` | `idempotency-token` | `ShareId`, `TemplateArn`, `ClientRequestToken` | `ClientRequestToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete a review template share. After the review template share is deleted, Amazon Web Services accounts, users, organizations, and organizational units (OUs) that you shared the review template with will no longer b ... |
| `DeleteWorkload` | `DELETE /workloads/{WorkloadId}` | `idempotency-token` | `WorkloadId`, `ClientRequestToken` | `ClientRequestToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete an existing workload. |
| `DeleteWorkloadShare` | `DELETE /workloads/{WorkloadId}/shares/{ShareId}` | `idempotency-token` | `ShareId`, `WorkloadId`, `ClientRequestToken` | `ClientRequestToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete a workload share. |
| `DisassociateLenses` | `PATCH /workloads/{WorkloadId}/disassociateLenses` | - | `WorkloadId`, `LensAliases` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociate a lens from a workload. Up to 10 lenses can be disassociated from a workload in a single API operation. The Amazon Web Services Well-Architected Framework lens ( wellarchitected ) cannot be removed from ... |
| `DisassociateProfiles` | `PATCH /workloads/{WorkloadId}/disassociateProfiles` | - | `WorkloadId`, `ProfileArns` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociate a profile from a workload. |
| `ExportLens` | `GET /lenses/{LensAlias}/export` | - | `LensAlias` | - | `ExportLensOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Export an existing lens. Only the owner of a lens can export it. Lenses provided by Amazon Web Services (Amazon Web Services Official Content) cannot be exported. Lenses are defined in JSON. For more information, see ... |
| `GetAnswer` | `GET /workloads/{WorkloadId}/lensReviews/{LensAlias}/answers/{QuestionId}` | - | `WorkloadId`, `LensAlias`, `QuestionId` | - | `GetAnswerOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the answer to a specific question in a workload review. |
| `GetConsolidatedReport` | `GET /consolidatedReport` | `paginated` | `Format` | - | `GetConsolidatedReportOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Get a consolidated report of your workloads. You can optionally choose to include workloads that have been shared with you. |
| `GetGlobalSettings` | `GET /global-settings` | - | - | - | `GetGlobalSettingsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Global settings for all workloads. |
| `GetLens` | `GET /lenses/{LensAlias}` | - | `LensAlias` | - | `GetLensOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get an existing lens. |
| `GetLensReview` | `GET /workloads/{WorkloadId}/lensReviews/{LensAlias}` | - | `WorkloadId`, `LensAlias` | - | `GetLensReviewOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get lens review. |
| `GetLensReviewReport` | `GET /workloads/{WorkloadId}/lensReviews/{LensAlias}/report` | - | `WorkloadId`, `LensAlias` | - | `GetLensReviewReportOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get lens review report. |
| `GetLensVersionDifference` | `GET /lenses/{LensAlias}/versionDifference` | - | `LensAlias` | - | `GetLensVersionDifferenceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get lens version differences. |
| `GetMilestone` | `GET /workloads/{WorkloadId}/milestones/{MilestoneNumber}` | - | `WorkloadId`, `MilestoneNumber` | - | `GetMilestoneOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get a milestone for an existing workload. |
| `GetProfile` | `GET /profiles/{ProfileArn}` | - | `ProfileArn` | - | `GetProfileOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get profile information. |
| `GetProfileTemplate` | `GET /profileTemplate` | - | - | - | `GetProfileTemplateOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get profile template. |
| `GetReviewTemplate` | `GET /reviewTemplates/{TemplateArn}` | - | `TemplateArn` | - | `GetReviewTemplateOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get review template. |
| `GetReviewTemplateAnswer` | `GET /reviewTemplates/{TemplateArn}/lensReviews/{LensAlias}/answers/{QuestionId}` | - | `TemplateArn`, `LensAlias`, `QuestionId` | - | `GetReviewTemplateAnswerOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get review template answer. |
| `GetReviewTemplateLensReview` | `GET /reviewTemplates/{TemplateArn}/lensReviews/{LensAlias}` | - | `TemplateArn`, `LensAlias` | - | `GetReviewTemplateLensReviewOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get a lens review associated with a review template. |
| `GetWorkload` | `GET /workloads/{WorkloadId}` | - | `WorkloadId` | - | `GetWorkloadOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get an existing workload. |
| `ImportLens` | `PUT /importLens` | `idempotency-token` | `JSONString`, `ClientRequestToken` | `ClientRequestToken` | `ImportLensOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Import a new custom lens or update an existing custom lens. To update an existing custom lens, specify its ARN as the LensAlias . If no ARN is specified, a new custom lens is created. The new or updated lens will hav ... |
| `ListAnswers` | `GET /workloads/{WorkloadId}/lensReviews/{LensAlias}/answers` | `paginated` | `WorkloadId`, `LensAlias` | - | `ListAnswersOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List of answers for a particular workload and lens. |
| `ListCheckDetails` | `POST /workloads/{WorkloadId}/checks` | `paginated` | `WorkloadId`, `LensArn`, `PillarId`, `QuestionId`, `ChoiceId` | - | `ListCheckDetailsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List of Trusted Advisor check details by account related to the workload. |
| `ListCheckSummaries` | `POST /workloads/{WorkloadId}/checkSummaries` | `paginated` | `WorkloadId`, `LensArn`, `PillarId`, `QuestionId`, `ChoiceId` | - | `ListCheckSummariesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List of Trusted Advisor checks summarized for all accounts related to the workload. |
| `ListLenses` | `GET /lenses` | `paginated` | - | - | `ListLensesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List the available lenses. |
| `ListLensReviewImprovements` | `GET /workloads/{WorkloadId}/lensReviews/{LensAlias}/improvements` | `paginated` | `WorkloadId`, `LensAlias` | - | `ListLensReviewImprovementsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List the improvements of a particular lens review. |
| `ListLensReviews` | `GET /workloads/{WorkloadId}/lensReviews` | `paginated` | `WorkloadId` | - | `ListLensReviewsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List lens reviews for a particular workload. |
| `ListLensShares` | `GET /lenses/{LensAlias}/shares` | `paginated` | `LensAlias` | - | `ListLensSharesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List the lens shares associated with the lens. |
| `ListMilestones` | `POST /workloads/{WorkloadId}/milestonesSummaries` | `paginated` | `WorkloadId` | - | `ListMilestonesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List all milestones for an existing workload. |
| `ListNotifications` | `POST /notifications` | `paginated` | - | - | `ListNotificationsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List lens notifications. |
| `ListProfileNotifications` | `GET /profileNotifications` | `paginated` | - | - | `ListProfileNotificationsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List profile notifications. |
| `ListProfiles` | `GET /profileSummaries` | `paginated` | - | - | `ListProfilesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List profiles. |
| `ListProfileShares` | `GET /profiles/{ProfileArn}/shares` | `paginated` | `ProfileArn` | - | `ListProfileSharesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List profile shares. |
| `ListReviewTemplateAnswers` | `GET /reviewTemplates/{TemplateArn}/lensReviews/{LensAlias}/answers` | `paginated` | `TemplateArn`, `LensAlias` | - | `ListReviewTemplateAnswersOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List the answers of a review template. |
| `ListReviewTemplates` | `GET /reviewTemplates` | `paginated` | - | - | `ListReviewTemplatesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List review templates. |
| `ListShareInvitations` | `GET /shareInvitations` | `paginated` | - | - | `ListShareInvitationsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List the share invitations. WorkloadNamePrefix , LensNamePrefix , ProfileNamePrefix , and TemplateNamePrefix are mutually exclusive. Use the parameter that matches your ShareResourceType . |
| `ListTagsForResource` | `GET /tags/{WorkloadArn}` | - | `WorkloadArn` | - | `ListTagsForResourceOutput` | `InternalServerException`, `ResourceNotFoundException` | List the tags for a resource. The WorkloadArn parameter can be a workload ARN, a custom lens ARN, a profile ARN, or review template ARN. |
| `ListTemplateShares` | `GET /templates/shares/{TemplateArn}` | `paginated` | `TemplateArn` | - | `ListTemplateSharesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List review template shares. |
| `ListWorkloads` | `POST /workloadsSummaries` | `paginated` | - | - | `ListWorkloadsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Paginated list of workloads. |
| `ListWorkloadShares` | `GET /workloads/{WorkloadId}/shares` | `paginated` | `WorkloadId` | - | `ListWorkloadSharesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List the workload shares associated with the workload. |
| `TagResource` | `POST /tags/{WorkloadArn}` | - | `WorkloadArn`, `Tags` | - | `TagResourceOutput` | `InternalServerException`, `ResourceNotFoundException` | Adds one or more tags to the specified resource. The WorkloadArn parameter can be a workload ARN, a custom lens ARN, a profile ARN, or review template ARN. |
| `UntagResource` | `DELETE /tags/{WorkloadArn}` | - | `WorkloadArn`, `TagKeys` | - | `UntagResourceOutput` | `InternalServerException`, `ResourceNotFoundException` | Deletes specified tags from a resource. The WorkloadArn parameter can be a workload ARN, a custom lens ARN, a profile ARN, or review template ARN. To specify multiple tags, use separate tagKeys parameters, for exampl ... |
| `UpdateAnswer` | `PATCH /workloads/{WorkloadId}/lensReviews/{LensAlias}/answers/{QuestionId}` | - | `WorkloadId`, `LensAlias`, `QuestionId` | - | `UpdateAnswerOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update the answer to a specific question in a workload review. |
| `UpdateGlobalSettings` | `PATCH /global-settings` | - | - | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Update whether the Amazon Web Services account is opted into organization sharing and discovery integration features. |
| `UpdateIntegration` | `POST /workloads/{WorkloadId}/updateIntegration` | `idempotency-token` | `WorkloadId`, `ClientRequestToken`, `IntegratingService` | `ClientRequestToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update integration features. |
| `UpdateLensReview` | `PATCH /workloads/{WorkloadId}/lensReviews/{LensAlias}` | - | `WorkloadId`, `LensAlias` | - | `UpdateLensReviewOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update lens review for a particular workload. |
| `UpdateProfile` | `PATCH /profiles/{ProfileArn}` | - | `ProfileArn` | - | `UpdateProfileOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a profile. |
| `UpdateReviewTemplate` | `PATCH /reviewTemplates/{TemplateArn}` | - | `TemplateArn` | - | `UpdateReviewTemplateOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a review template. |
| `UpdateReviewTemplateAnswer` | `PATCH /reviewTemplates/{TemplateArn}/lensReviews/{LensAlias}/answers/{QuestionId}` | - | `TemplateArn`, `LensAlias`, `QuestionId` | - | `UpdateReviewTemplateAnswerOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a review template answer. |
| `UpdateReviewTemplateLensReview` | `PATCH /reviewTemplates/{TemplateArn}/lensReviews/{LensAlias}` | - | `TemplateArn`, `LensAlias` | - | `UpdateReviewTemplateLensReviewOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a lens review associated with a review template. |
| `UpdateShareInvitation` | `PATCH /shareInvitations/{ShareInvitationId}` | - | `ShareInvitationId`, `ShareInvitationAction` | - | `UpdateShareInvitationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a workload or custom lens share invitation. This API operation can be called independently of any resource. Previous documentation implied that a workload ARN must be specified. |
| `UpdateWorkload` | `PATCH /workloads/{WorkloadId}` | - | `WorkloadId` | - | `UpdateWorkloadOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update an existing workload. |
| `UpdateWorkloadShare` | `PATCH /workloads/{WorkloadId}/shares/{ShareId}` | - | `ShareId`, `WorkloadId`, `PermissionType` | - | `UpdateWorkloadShareOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a workload share. |
| `UpgradeLensReview` | `PUT /workloads/{WorkloadId}/lensReviews/{LensAlias}/upgrade` | - | `WorkloadId`, `LensAlias`, `MilestoneName` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Upgrade lens review for a particular workload. |
| `UpgradeProfileVersion` | `PUT /workloads/{WorkloadId}/profiles/{ProfileArn}/upgrade` | `idempotency-token` | `WorkloadId`, `ProfileArn` | `ClientRequestToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Upgrade a profile. |
| `UpgradeReviewTemplateLensReview` | `PUT /reviewTemplates/{TemplateArn}/lensReviews/{LensAlias}/upgrade` | - | `TemplateArn`, `LensAlias` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Upgrade the lens review of a review template. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DeleteLens` | - | `ClientRequestToken -> ClientRequestToken`, `LensStatus -> LensStatus` | - | - |
| `DeleteLensShare` | - | `ClientRequestToken -> ClientRequestToken` | - | - |
| `DeleteProfile` | - | `ClientRequestToken -> ClientRequestToken` | - | - |
| `DeleteProfileShare` | - | `ClientRequestToken -> ClientRequestToken` | - | - |
| `DeleteReviewTemplate` | - | `ClientRequestToken -> ClientRequestToken` | - | - |
| `DeleteTemplateShare` | - | `ClientRequestToken -> ClientRequestToken` | - | - |
| `DeleteWorkload` | - | `ClientRequestToken -> ClientRequestToken` | - | - |
| `DeleteWorkloadShare` | - | `ClientRequestToken -> ClientRequestToken` | - | - |
| `ExportLens` | - | `LensVersion -> LensVersion` | - | - |
| `GetAnswer` | - | `MilestoneNumber -> MilestoneNumber` | - | - |
| `GetConsolidatedReport` | - | `Format -> Format`, `IncludeSharedResources -> IncludeSharedResources`, `NextToken -> NextToken`, `MaxResults -> MaxResults` | - | - |
| `GetLens` | - | `LensVersion -> LensVersion` | - | - |
| `GetLensReview` | - | `MilestoneNumber -> MilestoneNumber` | - | - |
| `GetLensReviewReport` | - | `MilestoneNumber -> MilestoneNumber` | - | - |
| `GetLensVersionDifference` | - | `BaseLensVersion -> BaseLensVersion`, `TargetLensVersion -> TargetLensVersion` | - | - |
| `GetProfile` | - | `ProfileVersion -> ProfileVersion` | - | - |
| `ListAnswers` | - | `PillarId -> PillarId`, `MilestoneNumber -> MilestoneNumber`, `NextToken -> NextToken`, `MaxResults -> MaxResults`, `QuestionPriority -> QuestionPriority` | - | - |
| `ListLenses` | - | `NextToken -> NextToken`, `MaxResults -> MaxResults`, `LensType -> LensType`, `LensStatus -> LensStatus`, `LensName -> LensName` | - | - |
| `ListLensReviewImprovements` | - | `PillarId -> PillarId`, `MilestoneNumber -> MilestoneNumber`, `NextToken -> NextToken`, `MaxResults -> MaxResults`, `QuestionPriority -> QuestionPriority` | - | - |
| `ListLensReviews` | - | `MilestoneNumber -> MilestoneNumber`, `NextToken -> NextToken`, `MaxResults -> MaxResults` | - | - |
| `ListLensShares` | - | `SharedWithPrefix -> SharedWithPrefix`, `NextToken -> NextToken`, `MaxResults -> MaxResults`, `Status -> Status` | - | - |
| `ListProfileNotifications` | - | `WorkloadId -> WorkloadId`, `NextToken -> NextToken`, `MaxResults -> MaxResults` | - | - |
| `ListProfiles` | - | `ProfileNamePrefix -> ProfileNamePrefix`, `ProfileOwnerType -> ProfileOwnerType`, `NextToken -> NextToken`, `MaxResults -> MaxResults` | - | - |
| `ListProfileShares` | - | `SharedWithPrefix -> SharedWithPrefix`, `NextToken -> NextToken`, `MaxResults -> MaxResults`, `Status -> Status` | - | - |
| `ListReviewTemplateAnswers` | - | `PillarId -> PillarId`, `NextToken -> NextToken`, `MaxResults -> MaxResults` | - | - |
| `ListReviewTemplates` | - | `NextToken -> NextToken`, `MaxResults -> MaxResults` | - | - |
| `ListShareInvitations` | - | `WorkloadNamePrefix -> WorkloadNamePrefix`, `LensNamePrefix -> LensNamePrefix`, `ShareResourceType -> ShareResourceType`, `NextToken -> NextToken`, `MaxResults -> MaxResults`, `ProfileNamePrefix -> ProfileNamePrefix`, `TemplateNamePrefix -> TemplateNamePrefix` | - | - |
| `ListTemplateShares` | - | `SharedWithPrefix -> SharedWithPrefix`, `NextToken -> NextToken`, `MaxResults -> MaxResults`, `Status -> Status` | - | - |
| `ListWorkloadShares` | - | `SharedWithPrefix -> SharedWithPrefix`, `NextToken -> NextToken`, `MaxResults -> MaxResults`, `Status -> Status` | - | - |
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | User does not have sufficient access to perform this action. |
| `ConflictException` | `structure` | Message, ResourceId, ResourceType | The resource has already been processed, was deleted, or is too large. |
| `InternalServerException` | `structure` | Message | There is a problem with the Well-Architected Tool API service. |
| `ResourceNotFoundException` | `structure` | Message, ResourceId, ResourceType | The requested resource was not found. |
| `ServiceQuotaExceededException` | `structure` | Message, ResourceId, ResourceType, QuotaCode, ServiceCode | The user has reached their resource quota. |
| `ThrottlingException` | `structure` | Message, QuotaCode, ServiceCode | Request was denied due to request throttling. |
| `ValidationException` | `structure` | Message, Reason, Fields | The user input is not valid. |
| `AssociateLensesInput` | `structure` | WorkloadId, LensAliases | Input to associate lens reviews. |
| `AssociateProfilesInput` | `structure` | WorkloadId, ProfileArns | - |
| `CreateLensShareInput` | `structure` | LensAlias, SharedWith, ClientRequestToken | - |
| `CreateLensShareOutput` | `structure` | ShareId | - |
| `CreateLensVersionInput` | `structure` | LensAlias, LensVersion, IsMajorVersion, ClientRequestToken | - |
| `CreateLensVersionOutput` | `structure` | LensArn, LensVersion | - |
| `CreateMilestoneInput` | `structure` | WorkloadId, MilestoneName, ClientRequestToken | Input for milestone creation. |
| `CreateMilestoneOutput` | `structure` | WorkloadId, MilestoneNumber | Output of a create milestone call. |
| `CreateProfileInput` | `structure` | ProfileName, ProfileDescription, ProfileQuestions, ClientRequestToken, Tags | - |
| `CreateProfileOutput` | `structure` | ProfileArn, ProfileVersion | - |
| `CreateProfileShareInput` | `structure` | ProfileArn, SharedWith, ClientRequestToken | - |
| `CreateProfileShareOutput` | `structure` | ShareId, ProfileArn | - |
| `CreateReviewTemplateInput` | `structure` | TemplateName, Description, Lenses, Notes, Tags, ClientRequestToken | - |
| `CreateReviewTemplateOutput` | `structure` | TemplateArn | - |
| `CreateTemplateShareInput` | `structure` | TemplateArn, SharedWith, ClientRequestToken | - |
| `CreateTemplateShareOutput` | `structure` | TemplateArn, ShareId | - |
| `CreateWorkloadInput` | `structure` | WorkloadName, Description, Environment, AccountIds, AwsRegions, NonAwsRegions, PillarPriorities, ArchitecturalDesign, ReviewOwner, IndustryType, Industry, Lenses, ... (+8) | Input for workload creation. |
| `CreateWorkloadOutput` | `structure` | WorkloadId, WorkloadArn | Output of a create workload call. |
| `CreateWorkloadShareInput` | `structure` | WorkloadId, SharedWith, PermissionType, ClientRequestToken | Input for Create Workload Share |
| `CreateWorkloadShareOutput` | `structure` | WorkloadId, ShareId | Input for Create Workload Share |
| `DeleteLensInput` | `structure` | LensAlias, ClientRequestToken, LensStatus | - |
| `DeleteLensShareInput` | `structure` | ShareId, LensAlias, ClientRequestToken | - |
| `DeleteProfileInput` | `structure` | ProfileArn, ClientRequestToken | - |
| `DeleteProfileShareInput` | `structure` | ShareId, ProfileArn, ClientRequestToken | - |
| `DeleteReviewTemplateInput` | `structure` | TemplateArn, ClientRequestToken | - |
| `DeleteTemplateShareInput` | `structure` | ShareId, TemplateArn, ClientRequestToken | - |
| `DeleteWorkloadInput` | `structure` | WorkloadId, ClientRequestToken | Input for workload deletion. |
| `DeleteWorkloadShareInput` | `structure` | ShareId, WorkloadId, ClientRequestToken | Input for Delete Workload Share |
| `DisassociateLensesInput` | `structure` | WorkloadId, LensAliases | Input to disassociate lens reviews. |
| `DisassociateProfilesInput` | `structure` | WorkloadId, ProfileArns | - |
| `ExportLensInput` | `structure` | LensAlias, LensVersion | - |
| `ExportLensOutput` | `structure` | LensJSON | - |
| `GetAnswerInput` | `structure` | WorkloadId, LensAlias, QuestionId, MilestoneNumber | Input to get answer. |
| `AccountJiraIssueManagementStatus` | `enum` | ENABLED, DISABLED | - |
| `AdditionalResourceType` | `enum` | HELPFUL_RESOURCE, IMPROVEMENT_PLAN | - |
| `AnswerReason` | `enum` | OUT_OF_SCOPE, BUSINESS_PRIORITIES, ARCHITECTURE_CONSTRAINTS, OTHER, NONE | - |
| `CheckFailureReason` | `enum` | ASSUME_ROLE_ERROR, ACCESS_DENIED, UNKNOWN_ERROR, PREMIUM_SUPPORT_REQUIRED | - |
| `CheckProvider` | `enum` | TRUSTED_ADVISOR | - |
| `CheckStatus` | `enum` | OKAY, WARNING, ERROR, NOT_AVAILABLE, FETCH_FAILED | - |
| `ChoiceReason` | `enum` | OUT_OF_SCOPE, BUSINESS_PRIORITIES, ARCHITECTURE_CONSTRAINTS, OTHER, NONE | - |
| `ChoiceStatus` | `enum` | SELECTED, NOT_APPLICABLE, UNSELECTED | - |
| `DefinitionType` | `enum` | WORKLOAD_METADATA, APP_REGISTRY | - |
| `DifferenceStatus` | `enum` | UPDATED, NEW, DELETED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
