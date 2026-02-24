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

- Operations: `ListAnswers`, `ListCheckDetails`, `ListCheckSummaries`, `ListLensReviewImprovements`, `ListLensReviews`, `ListLensShares`, `ListLenses`, `ListMilestones`, `ListNotifications`, `ListProfileNotifications`, `ListProfileShares`, `ListProfiles`, `ListReviewTemplateAnswers`, `ListReviewTemplates`, `ListShareInvitations`, `ListTagsForResource`, `ListTemplateShares`, `ListWorkloadShares`, `ListWorkloads`
- Traits: `paginated` (18)
- Common required input members in this group: `ChoiceId`, `LensAlias`, `LensArn`, `PillarId`, `ProfileArn`, `QuestionId`, `TemplateArn`, `WorkloadArn`, `WorkloadId`

### Get

- Operations: `GetAnswer`, `GetConsolidatedReport`, `GetGlobalSettings`, `GetLens`, `GetLensReview`, `GetLensReviewReport`, `GetLensVersionDifference`, `GetMilestone`, `GetProfile`, `GetProfileTemplate`, `GetReviewTemplate`, `GetReviewTemplateAnswer`, `GetReviewTemplateLensReview`, `GetWorkload`
- Traits: `paginated` (1)
- Common required input members in this group: `Format`, `LensAlias`, `MilestoneNumber`, `ProfileArn`, `QuestionId`, `TemplateArn`, `WorkloadId`

### Update

- Operations: `UpdateAnswer`, `UpdateGlobalSettings`, `UpdateIntegration`, `UpdateLensReview`, `UpdateProfile`, `UpdateReviewTemplate`, `UpdateReviewTemplateAnswer`, `UpdateReviewTemplateLensReview`, `UpdateShareInvitation`, `UpdateWorkload`, `UpdateWorkloadShare`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `ClientRequestToken`, `IntegratingService`, `LensAlias`, `PermissionType`, `ProfileArn`, `QuestionId`, `ShareId`, `ShareInvitationAction`, `ShareInvitationId`, `TemplateArn`, `WorkloadId`

### Create

- Operations: `CreateLensShare`, `CreateLensVersion`, `CreateMilestone`, `CreateProfile`, `CreateProfileShare`, `CreateReviewTemplate`, `CreateTemplateShare`, `CreateWorkload`, `CreateWorkloadShare`
- Traits: `idempotency-token` (9)
- Common required input members in this group: `ClientRequestToken`, `Description`, `Environment`, `LensAlias`, `LensVersion`, `Lenses`, `MilestoneName`, `PermissionType`, `ProfileArn`, `ProfileDescription`, `ProfileName`, `ProfileQuestions`, `SharedWith`, `TemplateArn`, `TemplateName`, `WorkloadId`, `WorkloadName`

### Delete

- Operations: `DeleteLens`, `DeleteLensShare`, `DeleteProfile`, `DeleteProfileShare`, `DeleteReviewTemplate`, `DeleteTemplateShare`, `DeleteWorkload`, `DeleteWorkloadShare`
- Traits: `idempotency-token` (8)
- Common required input members in this group: `ClientRequestToken`, `LensAlias`, `LensStatus`, `ProfileArn`, `ShareId`, `TemplateArn`, `WorkloadId`

### Upgrade

- Operations: `UpgradeLensReview`, `UpgradeProfileVersion`, `UpgradeReviewTemplateLensReview`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `LensAlias`, `MilestoneName`, `ProfileArn`, `TemplateArn`, `WorkloadId`

### Associate

- Operations: `AssociateLenses`, `AssociateProfiles`
- Common required input members in this group: `LensAliases`, `ProfileArns`, `WorkloadId`

### Disassociate

- Operations: `DisassociateLenses`, `DisassociateProfiles`
- Common required input members in this group: `LensAliases`, `ProfileArns`, `WorkloadId`

### Export

- Operations: `ExportLens`
- Common required input members in this group: `LensAlias`

### Import

- Operations: `ImportLens`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `ClientRequestToken`, `JSONString`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `Tags`, `WorkloadArn`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `TagKeys`, `WorkloadArn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateLenses` | `PATCH /workloads/{WorkloadId}/associateLenses` | - | `LensAliases`, `WorkloadId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associate a lens to a workload. Up to 10 lenses can be associated with a workload in a single API operation. |
| `AssociateProfiles` | `PATCH /workloads/{WorkloadId}/associateProfiles` | - | `ProfileArns`, `WorkloadId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associate a profile with a workload. |
| `CreateLensShare` | `POST /lenses/{LensAlias}/shares` | `idempotency-token` | `ClientRequestToken`, `LensAlias`, `SharedWith` | `ClientRequestToken` | `CreateLensShareOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a lens share. The owner of a lens can share it with other Amazon Web Services accounts, users, an organization, and organizational units (OUs) in the same Amazon Web Services Region. |
| `CreateLensVersion` | `POST /lenses/{LensAlias}/versions` | `idempotency-token` | `ClientRequestToken`, `LensAlias`, `LensVersion` | `ClientRequestToken` | `CreateLensVersionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a new lens version. A lens can have up to 100 versions. |
| `CreateMilestone` | `POST /workloads/{WorkloadId}/milestones` | `idempotency-token` | `ClientRequestToken`, `MilestoneName`, `WorkloadId` | `ClientRequestToken` | `CreateMilestoneOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a milestone for an existing workload. |
| `CreateProfile` | `POST /profiles` | `idempotency-token` | `ClientRequestToken`, `ProfileDescription`, `ProfileName`, `ProfileQuestions` | `ClientRequestToken` | `CreateProfileOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a profile. |
| `CreateProfileShare` | `POST /profiles/{ProfileArn}/shares` | `idempotency-token` | `ClientRequestToken`, `ProfileArn`, `SharedWith` | `ClientRequestToken` | `CreateProfileShareOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a profile share. |
| `CreateReviewTemplate` | `POST /reviewTemplates` | `idempotency-token` | `ClientRequestToken`, `Description`, `Lenses`, `TemplateName` | `ClientRequestToken` | `CreateReviewTemplateOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a review template. Disclaimer Do not include or gather personal identifiable information (PII) of end users or other identifiable individuals in or via your review templates. |
| `CreateTemplateShare` | `POST /templates/shares/{TemplateArn}` | `idempotency-token` | `ClientRequestToken`, `SharedWith`, `TemplateArn` | `ClientRequestToken` | `CreateTemplateShareOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a review template share. The owner of a review template can share it with other Amazon Web Services accounts, users, an organization, and organizational units (OUs) in the same Amazon Web Services Region. |
| `CreateWorkload` | `POST /workloads` | `idempotency-token` | `ClientRequestToken`, `Description`, `Environment`, `Lenses`, `WorkloadName` | `ClientRequestToken` | `CreateWorkloadOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a new workload. The owner of a workload can share the workload with other Amazon Web Services accounts, users, an organization, and organizational units (OUs) in the same Amazon Web Services Region. |
| `CreateWorkloadShare` | `POST /workloads/{WorkloadId}/shares` | `idempotency-token` | `ClientRequestToken`, `PermissionType`, `SharedWith`, `WorkloadId` | `ClientRequestToken` | `CreateWorkloadShareOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a workload share. The owner of a workload can share it with other Amazon Web Services accounts and users in the same Amazon Web Services Region. |
| `DeleteLens` | `DELETE /lenses/{LensAlias}` | `idempotency-token` | `ClientRequestToken`, `LensAlias`, `LensStatus` | `ClientRequestToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete an existing lens. Only the owner of a lens can delete it. |
| `DeleteLensShare` | `DELETE /lenses/{LensAlias}/shares/{ShareId}` | `idempotency-token` | `ClientRequestToken`, `LensAlias`, `ShareId` | `ClientRequestToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete a lens share. After the lens share is deleted, Amazon Web Services accounts, users, organizations, and organizational units (OUs) that you shared the lens with can continue to use it, but they will no longer be able to apply it to new workloads. |
| `DeleteProfile` | `DELETE /profiles/{ProfileArn}` | `idempotency-token` | `ClientRequestToken`, `ProfileArn` | `ClientRequestToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete a profile. Disclaimer By sharing your profile with other Amazon Web Services accounts, you acknowledge that Amazon Web Services will make your profile available to those other accounts. |
| `DeleteProfileShare` | `DELETE /profiles/{ProfileArn}/shares/{ShareId}` | `idempotency-token` | `ClientRequestToken`, `ProfileArn`, `ShareId` | `ClientRequestToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete a profile share. |
| `DeleteReviewTemplate` | `DELETE /reviewTemplates/{TemplateArn}` | `idempotency-token` | `ClientRequestToken`, `TemplateArn` | `ClientRequestToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete a review template. Only the owner of a review template can delete it. |
| `DeleteTemplateShare` | `DELETE /templates/shares/{TemplateArn}/{ShareId}` | `idempotency-token` | `ClientRequestToken`, `ShareId`, `TemplateArn` | `ClientRequestToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete a review template share. After the review template share is deleted, Amazon Web Services accounts, users, organizations, and organizational units (OUs) that you shared the review template with will no longer be able to apply it to new workloads. |
| `DeleteWorkload` | `DELETE /workloads/{WorkloadId}` | `idempotency-token` | `ClientRequestToken`, `WorkloadId` | `ClientRequestToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete an existing workload. |
| `DeleteWorkloadShare` | `DELETE /workloads/{WorkloadId}/shares/{ShareId}` | `idempotency-token` | `ClientRequestToken`, `ShareId`, `WorkloadId` | `ClientRequestToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete a workload share. |
| `DisassociateLenses` | `PATCH /workloads/{WorkloadId}/disassociateLenses` | - | `LensAliases`, `WorkloadId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociate a lens from a workload. Up to 10 lenses can be disassociated from a workload in a single API operation. |
| `DisassociateProfiles` | `PATCH /workloads/{WorkloadId}/disassociateProfiles` | - | `ProfileArns`, `WorkloadId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociate a profile from a workload. |
| `ExportLens` | `GET /lenses/{LensAlias}/export` | - | `LensAlias` | - | `ExportLensOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Export an existing lens. Only the owner of a lens can export it. |
| `GetAnswer` | `GET /workloads/{WorkloadId}/lensReviews/{LensAlias}/answers/{QuestionId}` | - | `LensAlias`, `QuestionId`, `WorkloadId` | - | `GetAnswerOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the answer to a specific question in a workload review. |
| `GetConsolidatedReport` | `GET /consolidatedReport` | `paginated` | `Format` | - | `GetConsolidatedReportOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Get a consolidated report of your workloads. You can optionally choose to include workloads that have been shared with you. |
| `GetGlobalSettings` | `GET /global-settings` | - | - | - | `GetGlobalSettingsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Global settings for all workloads. |
| `GetLens` | `GET /lenses/{LensAlias}` | - | `LensAlias` | - | `GetLensOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get an existing lens. |
| `GetLensReview` | `GET /workloads/{WorkloadId}/lensReviews/{LensAlias}` | - | `LensAlias`, `WorkloadId` | - | `GetLensReviewOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get lens review. |
| `GetLensReviewReport` | `GET /workloads/{WorkloadId}/lensReviews/{LensAlias}/report` | - | `LensAlias`, `WorkloadId` | - | `GetLensReviewReportOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get lens review report. |
| `GetLensVersionDifference` | `GET /lenses/{LensAlias}/versionDifference` | - | `LensAlias` | - | `GetLensVersionDifferenceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get lens version differences. |
| `GetMilestone` | `GET /workloads/{WorkloadId}/milestones/{MilestoneNumber}` | - | `MilestoneNumber`, `WorkloadId` | - | `GetMilestoneOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get a milestone for an existing workload. |
| `GetProfile` | `GET /profiles/{ProfileArn}` | - | `ProfileArn` | - | `GetProfileOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get profile information. |
| `GetProfileTemplate` | `GET /profileTemplate` | - | - | - | `GetProfileTemplateOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get profile template. |
| `GetReviewTemplate` | `GET /reviewTemplates/{TemplateArn}` | - | `TemplateArn` | - | `GetReviewTemplateOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get review template. |
| `GetReviewTemplateAnswer` | `GET /reviewTemplates/{TemplateArn}/lensReviews/{LensAlias}/answers/{QuestionId}` | - | `LensAlias`, `QuestionId`, `TemplateArn` | - | `GetReviewTemplateAnswerOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get review template answer. |
| `GetReviewTemplateLensReview` | `GET /reviewTemplates/{TemplateArn}/lensReviews/{LensAlias}` | - | `LensAlias`, `TemplateArn` | - | `GetReviewTemplateLensReviewOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get a lens review associated with a review template. |
| `GetWorkload` | `GET /workloads/{WorkloadId}` | - | `WorkloadId` | - | `GetWorkloadOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get an existing workload. |
| `ImportLens` | `PUT /importLens` | `idempotency-token` | `ClientRequestToken`, `JSONString` | `ClientRequestToken` | `ImportLensOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Import a new custom lens or update an existing custom lens. To update an existing custom lens, specify its ARN as the `LensAlias`. |
| `ListAnswers` | `GET /workloads/{WorkloadId}/lensReviews/{LensAlias}/answers` | `paginated` | `LensAlias`, `WorkloadId` | - | `ListAnswersOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List of answers for a particular workload and lens. |
| `ListCheckDetails` | `POST /workloads/{WorkloadId}/checks` | `paginated` | `ChoiceId`, `LensArn`, `PillarId`, `QuestionId`, `WorkloadId` | - | `ListCheckDetailsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List of Trusted Advisor check details by account related to the workload. |
| `ListCheckSummaries` | `POST /workloads/{WorkloadId}/checkSummaries` | `paginated` | `ChoiceId`, `LensArn`, `PillarId`, `QuestionId`, `WorkloadId` | - | `ListCheckSummariesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List of Trusted Advisor checks summarized for all accounts related to the workload. |
| `ListLensReviewImprovements` | `GET /workloads/{WorkloadId}/lensReviews/{LensAlias}/improvements` | `paginated` | `LensAlias`, `WorkloadId` | - | `ListLensReviewImprovementsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List the improvements of a particular lens review. |
| `ListLensReviews` | `GET /workloads/{WorkloadId}/lensReviews` | `paginated` | `WorkloadId` | - | `ListLensReviewsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List lens reviews for a particular workload. |
| `ListLensShares` | `GET /lenses/{LensAlias}/shares` | `paginated` | `LensAlias` | - | `ListLensSharesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List the lens shares associated with the lens. |
| `ListLenses` | `GET /lenses` | `paginated` | - | - | `ListLensesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List the available lenses. |
| `ListMilestones` | `POST /workloads/{WorkloadId}/milestonesSummaries` | `paginated` | `WorkloadId` | - | `ListMilestonesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List all milestones for an existing workload. |
| `ListNotifications` | `POST /notifications` | `paginated` | - | - | `ListNotificationsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List lens notifications. |
| `ListProfileNotifications` | `GET /profileNotifications` | `paginated` | - | - | `ListProfileNotificationsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List profile notifications. |
| `ListProfileShares` | `GET /profiles/{ProfileArn}/shares` | `paginated` | `ProfileArn` | - | `ListProfileSharesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List profile shares. |
| `ListProfiles` | `GET /profileSummaries` | `paginated` | - | - | `ListProfilesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List profiles. |
| `ListReviewTemplateAnswers` | `GET /reviewTemplates/{TemplateArn}/lensReviews/{LensAlias}/answers` | `paginated` | `LensAlias`, `TemplateArn` | - | `ListReviewTemplateAnswersOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List the answers of a review template. |
| `ListReviewTemplates` | `GET /reviewTemplates` | `paginated` | - | - | `ListReviewTemplatesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List review templates. |
| `ListShareInvitations` | `GET /shareInvitations` | `paginated` | - | - | `ListShareInvitationsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List the share invitations. `WorkloadNamePrefix`, `LensNamePrefix`, `ProfileNamePrefix`, and `TemplateNamePrefix` are mutually exclusive. |
| `ListTagsForResource` | `GET /tags/{WorkloadArn}` | - | `WorkloadArn` | - | `ListTagsForResourceOutput` | `InternalServerException`, `ResourceNotFoundException` | List the tags for a resource. The WorkloadArn parameter can be a workload ARN, a custom lens ARN, a profile ARN, or review template ARN. |
| `ListTemplateShares` | `GET /templates/shares/{TemplateArn}` | `paginated` | `TemplateArn` | - | `ListTemplateSharesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List review template shares. |
| `ListWorkloadShares` | `GET /workloads/{WorkloadId}/shares` | `paginated` | `WorkloadId` | - | `ListWorkloadSharesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List the workload shares associated with the workload. |
| `ListWorkloads` | `POST /workloadsSummaries` | `paginated` | - | - | `ListWorkloadsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Paginated list of workloads. |
| `TagResource` | `POST /tags/{WorkloadArn}` | - | `Tags`, `WorkloadArn` | - | `TagResourceOutput` | `InternalServerException`, `ResourceNotFoundException` | Adds one or more tags to the specified resource. The WorkloadArn parameter can be a workload ARN, a custom lens ARN, a profile ARN, or review template ARN. |
| `UntagResource` | `DELETE /tags/{WorkloadArn}` | - | `TagKeys`, `WorkloadArn` | - | `UntagResourceOutput` | `InternalServerException`, `ResourceNotFoundException` | Deletes specified tags from a resource. The WorkloadArn parameter can be a workload ARN, a custom lens ARN, a profile ARN, or review template ARN. |
| `UpdateAnswer` | `PATCH /workloads/{WorkloadId}/lensReviews/{LensAlias}/answers/{QuestionId}` | - | `LensAlias`, `QuestionId`, `WorkloadId` | - | `UpdateAnswerOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update the answer to a specific question in a workload review. |
| `UpdateGlobalSettings` | `PATCH /global-settings` | - | - | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Update whether the Amazon Web Services account is opted into organization sharing and discovery integration features. |
| `UpdateIntegration` | `POST /workloads/{WorkloadId}/updateIntegration` | `idempotency-token` | `ClientRequestToken`, `IntegratingService`, `WorkloadId` | `ClientRequestToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update integration features. |
| `UpdateLensReview` | `PATCH /workloads/{WorkloadId}/lensReviews/{LensAlias}` | - | `LensAlias`, `WorkloadId` | - | `UpdateLensReviewOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update lens review for a particular workload. |
| `UpdateProfile` | `PATCH /profiles/{ProfileArn}` | - | `ProfileArn` | - | `UpdateProfileOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a profile. |
| `UpdateReviewTemplate` | `PATCH /reviewTemplates/{TemplateArn}` | - | `TemplateArn` | - | `UpdateReviewTemplateOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a review template. |
| `UpdateReviewTemplateAnswer` | `PATCH /reviewTemplates/{TemplateArn}/lensReviews/{LensAlias}/answers/{QuestionId}` | - | `LensAlias`, `QuestionId`, `TemplateArn` | - | `UpdateReviewTemplateAnswerOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a review template answer. |
| `UpdateReviewTemplateLensReview` | `PATCH /reviewTemplates/{TemplateArn}/lensReviews/{LensAlias}` | - | `LensAlias`, `TemplateArn` | - | `UpdateReviewTemplateLensReviewOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a lens review associated with a review template. |
| `UpdateShareInvitation` | `PATCH /shareInvitations/{ShareInvitationId}` | - | `ShareInvitationAction`, `ShareInvitationId` | - | `UpdateShareInvitationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a workload or custom lens share invitation. This API operation can be called independently of any resource. |
| `UpdateWorkload` | `PATCH /workloads/{WorkloadId}` | - | `WorkloadId` | - | `UpdateWorkloadOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update an existing workload. |
| `UpdateWorkloadShare` | `PATCH /workloads/{WorkloadId}/shares/{ShareId}` | - | `PermissionType`, `ShareId`, `WorkloadId` | - | `UpdateWorkloadShareOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a workload share. |
| `UpgradeLensReview` | `PUT /workloads/{WorkloadId}/lensReviews/{LensAlias}/upgrade` | - | `LensAlias`, `MilestoneName`, `WorkloadId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Upgrade lens review for a particular workload. |
| `UpgradeProfileVersion` | `PUT /workloads/{WorkloadId}/profiles/{ProfileArn}/upgrade` | `idempotency-token` | `ProfileArn`, `WorkloadId` | `ClientRequestToken` | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Upgrade a profile. |
| `UpgradeReviewTemplateLensReview` | `PUT /reviewTemplates/{TemplateArn}/lensReviews/{LensAlias}/upgrade` | - | `LensAlias`, `TemplateArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Upgrade the lens review of a review template. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `Message` | There is a problem with the Well-Architected Tool API service. |
| `AccessDeniedException` | `structure` | `Message` | User does not have sufficient access to perform this action. |
| `ThrottlingException` | `structure` | `Message`, `QuotaCode`, `ServiceCode` | Request was denied due to request throttling. |
| `ValidationException` | `structure` | `Fields`, `Message`, `Reason` | The user input is not valid. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceId`, `ResourceType` | The requested resource was not found. |
| `ConflictException` | `structure` | `Message`, `ResourceId`, `ResourceType` | The resource has already been processed, was deleted, or is too large. |
| `ServiceQuotaExceededException` | `structure` | `Message`, `QuotaCode`, `ResourceId`, `ResourceType`, `ServiceCode` | The user has reached their resource quota. |
| `AssociateLensesInput` | `structure` | `LensAliases`, `WorkloadId` | Input to associate lens reviews. |
| `AssociateProfilesInput` | `structure` | `ProfileArns`, `WorkloadId` | - |
| `CreateLensShareInput` | `structure` | `ClientRequestToken`, `LensAlias`, `SharedWith` | - |
| `CreateLensShareOutput` | `structure` | `ShareId` | - |
| `CreateLensVersionInput` | `structure` | `ClientRequestToken`, `IsMajorVersion`, `LensAlias`, `LensVersion` | - |
| `CreateLensVersionOutput` | `structure` | `LensArn`, `LensVersion` | - |
| `CreateMilestoneInput` | `structure` | `ClientRequestToken`, `MilestoneName`, `WorkloadId` | Input for milestone creation. |
| `CreateMilestoneOutput` | `structure` | `MilestoneNumber`, `WorkloadId` | Output of a create milestone call. |
| `CreateProfileInput` | `structure` | `ClientRequestToken`, `ProfileDescription`, `ProfileName`, `ProfileQuestions`, `Tags` | - |
| `CreateProfileOutput` | `structure` | `ProfileArn`, `ProfileVersion` | - |
| `CreateProfileShareInput` | `structure` | `ClientRequestToken`, `ProfileArn`, `SharedWith` | - |
| `CreateProfileShareOutput` | `structure` | `ProfileArn`, `ShareId` | - |
| `CreateReviewTemplateInput` | `structure` | `ClientRequestToken`, `Description`, `Lenses`, `Notes`, `Tags`, `TemplateName` | - |
| `CreateReviewTemplateOutput` | `structure` | `TemplateArn` | - |
| `CreateTemplateShareInput` | `structure` | `ClientRequestToken`, `SharedWith`, `TemplateArn` | - |
| `CreateTemplateShareOutput` | `structure` | `ShareId`, `TemplateArn` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
