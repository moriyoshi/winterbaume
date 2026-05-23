# AWS Resilience Hub

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Resilience Hub helps you proactively prepare and protect your Amazon Web Services applications from disruptions. It offers continual resiliency assessment and validation that integrates into your software development lifecycle. This enables you to uncover resiliency weaknesses, ensure recovery time objective (RTO) and recovery point objective (RPO) targets for your applications are met, and resolve issues before they are released into production.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS Resilience Hub resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Resilience Hub workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Describe`, `Delete`, `Create`, `Update` operation families, including `ListAlarmRecommendations`, `ListAppAssessmentComplianceDrifts`, `ListAppAssessmentResourceDrifts`, `ListAppAssessments`, `DescribeApp`, `DescribeAppAssessment`.

## Service Identity and Protocol

- AWS model slug: `resiliencehub`
- AWS SDK for Rust slug: `resiliencehub`
- Model version: `2020-04-30`
- Model file: `vendor/api-models-aws/models/resiliencehub/service/2020-04-30/resiliencehub-2020-04-30.json`
- SDK ID: `resiliencehub`
- Endpoint prefix: `resiliencehub`
- ARN namespace: `resiliencehub`
- CloudFormation name: `Resiliencehub`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (21), `Describe` (11), `Delete` (7), `Create` (5), `Update` (5), `Start` (3), `Accept` (1), `Add` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptResourceGroupingRecommendations`, `AddDraftAppVersionResourceMappings`, `BatchUpdateRecommendationStatus`, `CreateApp`, `CreateAppVersionAppComponent`, `CreateAppVersionResource`, `CreateRecommendationTemplate`, `CreateResiliencyPolicy`, `DeleteApp`, `DeleteAppAssessment`, `DeleteAppInputSource`, `DeleteAppVersionAppComponent`, `DeleteAppVersionResource`, `DeleteRecommendationTemplate`, `DeleteResiliencyPolicy`, `ImportResourcesToDraftAppVersion`, `PutDraftAppVersionTemplate`, `RejectResourceGroupingRecommendations`, `RemoveDraftAppVersionResourceMappings`, `StartAppAssessment`, ... (+9).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeApp`, `DescribeAppAssessment`, `DescribeAppVersion`, `DescribeAppVersionAppComponent`, `DescribeAppVersionResource`, `DescribeAppVersionResourcesResolutionStatus`, `DescribeAppVersionTemplate`, `DescribeDraftAppVersionResourcesImportStatus`, `DescribeMetricsExport`, `DescribeResiliencyPolicy`, `DescribeResourceGroupingRecommendationTask`, `ListAlarmRecommendations`, `ListAppAssessmentComplianceDrifts`, `ListAppAssessmentResourceDrifts`, `ListAppAssessments`, `ListAppComponentCompliances`, `ListAppComponentRecommendations`, `ListAppInputSources`, `ListAppVersionAppComponents`, `ListAppVersionResourceMappings`, ... (+12).
- Pagination is modelled for 20 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 14 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DescribeDraftAppVersionResourcesImportStatus`, `DescribeMetricsExport`, `DescribeResourceGroupingRecommendationTask`, `ImportResourcesToDraftAppVersion`, `StartAppAssessment`, `StartMetricsExport`, `StartResourceGroupingRecommendationTask`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 63 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `ECS`, `EKS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListAlarmRecommendations`, `ListAppAssessmentComplianceDrifts`, `ListAppAssessmentResourceDrifts`, `ListAppAssessments`, `ListAppComponentCompliances`, `ListAppComponentRecommendations`, `ListAppInputSources`, `ListApps`, `ListAppVersionAppComponents`, `ListAppVersionResourceMappings`, `ListAppVersionResources`, `ListAppVersions`, `ListMetrics`, `ListRecommendationTemplates`, `ListResiliencyPolicies`, `ListResourceGroupingRecommendations`, `ListSopRecommendations`, `ListSuggestedResiliencyPolicies`, `ListTagsForResource`, `ListTestRecommendations`, `ListUnsupportedAppVersionResources`
- Traits: `paginated` (20), `readonly` (8)
- Common required input members in this group: `assessmentArn`, `appArn`, `appVersion`

### Describe

- Operations: `DescribeApp`, `DescribeAppAssessment`, `DescribeAppVersion`, `DescribeAppVersionAppComponent`, `DescribeAppVersionResource`, `DescribeAppVersionResourcesResolutionStatus`, `DescribeAppVersionTemplate`, `DescribeDraftAppVersionResourcesImportStatus`, `DescribeMetricsExport`, `DescribeResiliencyPolicy`, `DescribeResourceGroupingRecommendationTask`
- Common required input members in this group: `appArn`, `appVersion`

### Delete

- Operations: `DeleteApp`, `DeleteAppAssessment`, `DeleteAppInputSource`, `DeleteAppVersionAppComponent`, `DeleteAppVersionResource`, `DeleteRecommendationTemplate`, `DeleteResiliencyPolicy`
- Traits: `idempotency-token` (7)
- Common required input members in this group: `appArn`

### Create

- Operations: `CreateApp`, `CreateAppVersionAppComponent`, `CreateAppVersionResource`, `CreateRecommendationTemplate`, `CreateResiliencyPolicy`
- Traits: `idempotency-token` (5)
- Common required input members in this group: `name`, `appArn`

### Update

- Operations: `UpdateApp`, `UpdateAppVersion`, `UpdateAppVersionAppComponent`, `UpdateAppVersionResource`, `UpdateResiliencyPolicy`
- Common required input members in this group: `appArn`

### Start

- Operations: `StartAppAssessment`, `StartMetricsExport`, `StartResourceGroupingRecommendationTask`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `appArn`

### Accept

- Operations: `AcceptResourceGroupingRecommendations`
- Common required input members in this group: -

### Add

- Operations: `AddDraftAppVersionResourceMappings`
- Common required input members in this group: -

### Batch

- Operations: `BatchUpdateRecommendationStatus`
- Common required input members in this group: -

### Import

- Operations: `ImportResourcesToDraftAppVersion`
- Common required input members in this group: -

### Publish

- Operations: `PublishAppVersion`
- Common required input members in this group: -

### Put

- Operations: `PutDraftAppVersionTemplate`
- Common required input members in this group: -

### Reject

- Operations: `RejectResourceGroupingRecommendations`
- Common required input members in this group: -

### Remove

- Operations: `RemoveDraftAppVersionResourceMappings`
- Common required input members in this group: -

### Resolve

- Operations: `ResolveAppVersionResources`
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
| `AcceptResourceGroupingRecommendations` | `POST /accept-resource-grouping-recommendations` | - | `appArn`, `entries` | - | `AcceptResourceGroupingRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Accepts the resource grouping recommendations suggested by Resilience Hub for your application. |
| `AddDraftAppVersionResourceMappings` | `POST /add-draft-app-version-resource-mappings` | - | `appArn`, `resourceMappings` | - | `AddDraftAppVersionResourceMappingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Adds the source of resource-maps to the draft version of an application. During assessment, Resilience Hub will use these resource-maps to resolve the latest physical ID for each resource in the application template. ... |
| `BatchUpdateRecommendationStatus` | `POST /batch-update-recommendation-status` | - | `appArn`, `requestEntries` | - | `BatchUpdateRecommendationStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables you to include or exclude one or more operational recommendations. |
| `CreateApp` | `POST /create-app` | `idempotency-token` | `name` | `clientToken` | `CreateAppResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an Resilience Hub application. An Resilience Hub application is a collection of Amazon Web Services resources structured to prevent and recover Amazon Web Services application disruptions. To describe a Resil ... |
| `CreateAppVersionAppComponent` | `POST /create-app-version-app-component` | `idempotency-token` | `appArn`, `name`, `type` | `clientToken` | `CreateAppVersionAppComponentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new Application Component in the Resilience Hub application. This API updates the Resilience Hub application draft version. To use this Application Component for running assessments, you must publish the Re ... |
| `CreateAppVersionResource` | `POST /create-app-version-resource` | `idempotency-token` | `appArn`, `logicalResourceId`, `physicalResourceId`, `resourceType`, `appComponents` | `clientToken` | `CreateAppVersionResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Adds a resource to the Resilience Hub application and assigns it to the specified Application Components. If you specify a new Application Component, Resilience Hub will automatically create the Application Component ... |
| `CreateRecommendationTemplate` | `POST /create-recommendation-template` | `idempotency-token` | `assessmentArn`, `name` | `clientToken` | `CreateRecommendationTemplateResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new recommendation template for the Resilience Hub application. |
| `CreateResiliencyPolicy` | `POST /create-resiliency-policy` | `idempotency-token` | `policyName`, `tier`, `policy` | `clientToken` | `CreateResiliencyPolicyResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a resiliency policy for an application. Resilience Hub allows you to provide a value of zero for rtoInSecs and rpoInSecs of your resiliency policy. But, while assessing your application, the lowest possible a ... |
| `DeleteApp` | `POST /delete-app` | `idempotency-token` | `appArn` | `clientToken` | `DeleteAppResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an Resilience Hub application. This is a destructive action that can't be undone. |
| `DeleteAppAssessment` | `POST /delete-app-assessment` | `idempotency-token` | `assessmentArn` | `clientToken` | `DeleteAppAssessmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an Resilience Hub application assessment. This is a destructive action that can't be undone. |
| `DeleteAppInputSource` | `POST /delete-app-input-source` | `idempotency-token` | `appArn` | `clientToken` | `DeleteAppInputSourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the input source and all of its imported resources from the Resilience Hub application. |
| `DeleteAppVersionAppComponent` | `POST /delete-app-version-app-component` | `idempotency-token` | `appArn`, `id` | `clientToken` | `DeleteAppVersionAppComponentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an Application Component from the Resilience Hub application. This API updates the Resilience Hub application draft version. To use this Application Component for running assessments, you must publish the Res ... |
| `DeleteAppVersionResource` | `POST /delete-app-version-resource` | `idempotency-token` | `appArn` | `clientToken` | `DeleteAppVersionResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a resource from the Resilience Hub application. You can only delete a manually added resource. To exclude non-manually added resources, use the UpdateAppVersionResource API. This action has no effect outside ... |
| `DeleteRecommendationTemplate` | `POST /delete-recommendation-template` | `idempotency-token` | `recommendationTemplateArn` | `clientToken` | `DeleteRecommendationTemplateResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a recommendation template. This is a destructive action that can't be undone. |
| `DeleteResiliencyPolicy` | `POST /delete-resiliency-policy` | `idempotency-token` | `policyArn` | `clientToken` | `DeleteResiliencyPolicyResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a resiliency policy. This is a destructive action that can't be undone. |
| `DescribeApp` | `POST /describe-app` | - | `appArn` | - | `DescribeAppResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes an Resilience Hub application. |
| `DescribeAppAssessment` | `POST /describe-app-assessment` | - | `assessmentArn` | - | `DescribeAppAssessmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes an assessment for an Resilience Hub application. |
| `DescribeAppVersion` | `POST /describe-app-version` | - | `appArn`, `appVersion` | - | `DescribeAppVersionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the Resilience Hub application version. |
| `DescribeAppVersionAppComponent` | `POST /describe-app-version-app-component` | - | `appArn`, `appVersion`, `id` | - | `DescribeAppVersionAppComponentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes an Application Component in the Resilience Hub application. |
| `DescribeAppVersionResource` | `POST /describe-app-version-resource` | - | `appArn`, `appVersion` | - | `DescribeAppVersionResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes a resource of the Resilience Hub application. This API accepts only one of the following parameters to describe the resource: resourceName logicalResourceId physicalResourceId (Along with physicalResourceId ... |
| `DescribeAppVersionResourcesResolutionStatus` | `POST /describe-app-version-resources-resolution-status` | - | `appArn`, `appVersion` | - | `DescribeAppVersionResourcesResolutionStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the resolution status for the specified resolution identifier for an application version. If resolutionId is not specified, the current resolution status is returned. |
| `DescribeAppVersionTemplate` | `POST /describe-app-version-template` | - | `appArn`, `appVersion` | - | `DescribeAppVersionTemplateResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes details about an Resilience Hub application. |
| `DescribeDraftAppVersionResourcesImportStatus` | `POST /describe-draft-app-version-resources-import-status` | - | `appArn` | - | `DescribeDraftAppVersionResourcesImportStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the status of importing resources to an application version. If you get a 404 error with ResourceImportStatusNotFoundAppMetadataException , you must call importResourcesToDraftAppVersion after creating the ... |
| `DescribeMetricsExport` | `POST /describe-metrics-export` | - | `metricsExportId` | - | `DescribeMetricsExportResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the metrics of the application configuration being exported. |
| `DescribeResiliencyPolicy` | `POST /describe-resiliency-policy` | - | `policyArn` | - | `DescribeResiliencyPolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes a specified resiliency policy for an Resilience Hub application. The returned policy object includes creation time, data location constraints, the Amazon Resource Name (ARN) for the policy, tags, tier, and ... |
| `DescribeResourceGroupingRecommendationTask` | `POST /describe-resource-grouping-recommendation-task` | - | `appArn` | - | `DescribeResourceGroupingRecommendationTaskResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the resource grouping recommendation tasks run by Resilience Hub for your application. |
| `ImportResourcesToDraftAppVersion` | `POST /import-resources-to-draft-app-version` | - | `appArn` | - | `ImportResourcesToDraftAppVersionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Imports resources to Resilience Hub application draft version from different input sources. For more information about the input sources supported by Resilience Hub, see Discover the structure and describe your Resil ... |
| `ListAlarmRecommendations` | `POST /list-alarm-recommendations` | `paginated` | `assessmentArn` | - | `ListAlarmRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the alarm recommendations for an Resilience Hub application. |
| `ListAppAssessmentComplianceDrifts` | `POST /list-app-assessment-compliance-drifts` | `paginated` | `assessmentArn` | - | `ListAppAssessmentComplianceDriftsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List of compliance drifts that were detected while running an assessment. |
| `ListAppAssessmentResourceDrifts` | `POST /list-app-assessment-resource-drifts` | `paginated` | `assessmentArn` | - | `ListAppAssessmentResourceDriftsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List of resource drifts that were detected while running an assessment. |
| `ListAppAssessments` | `GET /list-app-assessments` | `readonly`, `paginated` | - | - | `ListAppAssessmentsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the assessments for an Resilience Hub application. You can use request parameters to refine the results for the response object. |
| `ListAppComponentCompliances` | `POST /list-app-component-compliances` | `paginated` | `assessmentArn` | - | `ListAppComponentCompliancesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the compliances for an Resilience Hub Application Component. |
| `ListAppComponentRecommendations` | `POST /list-app-component-recommendations` | `paginated` | `assessmentArn` | - | `ListAppComponentRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the recommendations for an Resilience Hub Application Component. |
| `ListAppInputSources` | `POST /list-app-input-sources` | `paginated` | `appArn`, `appVersion` | - | `ListAppInputSourcesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all the input sources of the Resilience Hub application. For more information about the input sources supported by Resilience Hub, see Discover the structure and describe your Resilience Hub application . |
| `ListApps` | `GET /list-apps` | `readonly`, `paginated` | - | - | `ListAppsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists your Resilience Hub applications. You can filter applications using only one filter at a time or without using any filter. If you try to filter applications using multiple filters, you will get the following er ... |
| `ListAppVersionAppComponents` | `POST /list-app-version-app-components` | `paginated` | `appArn`, `appVersion` | - | `ListAppVersionAppComponentsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all the Application Components in the Resilience Hub application. |
| `ListAppVersionResourceMappings` | `POST /list-app-version-resource-mappings` | `paginated` | `appArn`, `appVersion` | - | `ListAppVersionResourceMappingsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists how the resources in an application version are mapped/sourced from. Mappings can be physical resource identifiers, CloudFormation stacks, resource-groups, or an application registry app. |
| `ListAppVersionResources` | `POST /list-app-version-resources` | `paginated` | `appArn`, `appVersion` | - | `ListAppVersionResourcesResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all the resources in an Resilience Hub application. |
| `ListAppVersions` | `POST /list-app-versions` | `paginated` | `appArn` | - | `ListAppVersionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the different versions for the Resilience Hub applications. |
| `ListMetrics` | `POST /list-metrics` | `readonly`, `paginated` | - | - | `ListMetricsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the metrics that can be exported. |
| `ListRecommendationTemplates` | `GET /list-recommendation-templates` | `readonly`, `paginated` | - | - | `ListRecommendationTemplatesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the recommendation templates for the Resilience Hub applications. |
| `ListResiliencyPolicies` | `GET /list-resiliency-policies` | `readonly`, `paginated` | - | - | `ListResiliencyPoliciesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the resiliency policies for the Resilience Hub applications. |
| `ListResourceGroupingRecommendations` | `GET /list-resource-grouping-recommendations` | `readonly`, `paginated` | - | - | `ListResourceGroupingRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the resource grouping recommendations suggested by Resilience Hub for your application. |
| `ListSopRecommendations` | `POST /list-sop-recommendations` | `paginated` | `assessmentArn` | - | `ListSopRecommendationsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the standard operating procedure (SOP) recommendations for the Resilience Hub applications. |
| `ListSuggestedResiliencyPolicies` | `GET /list-suggested-resiliency-policies` | `readonly`, `paginated` | - | - | `ListSuggestedResiliencyPoliciesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the suggested resiliency policies for the Resilience Hub applications. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the tags for your resources in your Resilience Hub applications. |
| `ListTestRecommendations` | `POST /list-test-recommendations` | `paginated` | `assessmentArn` | - | `ListTestRecommendationsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the test recommendations for the Resilience Hub application. |
| `ListUnsupportedAppVersionResources` | `POST /list-unsupported-app-version-resources` | `paginated` | `appArn`, `appVersion` | - | `ListUnsupportedAppVersionResourcesResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the resources that are not currently supported in Resilience Hub. An unsupported resource is a resource that exists in the object that was used to create an app, but is not supported by Resilience Hub. |
| `PublishAppVersion` | `POST /publish-app-version` | - | `appArn` | - | `PublishAppVersionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Publishes a new version of a specific Resilience Hub application. |
| `PutDraftAppVersionTemplate` | `POST /put-draft-app-version-template` | - | `appArn`, `appTemplateBody` | - | `PutDraftAppVersionTemplateResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds or updates the app template for an Resilience Hub application draft version. |
| `RejectResourceGroupingRecommendations` | `POST /reject-resource-grouping-recommendations` | - | `appArn`, `entries` | - | `RejectResourceGroupingRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Rejects resource grouping recommendations. |
| `RemoveDraftAppVersionResourceMappings` | `POST /remove-draft-app-version-resource-mappings` | - | `appArn` | - | `RemoveDraftAppVersionResourceMappingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes resource mappings from a draft application version. |
| `ResolveAppVersionResources` | `POST /resolve-app-version-resources` | - | `appArn`, `appVersion` | - | `ResolveAppVersionResourcesResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Resolves the resources for an application version. |
| `StartAppAssessment` | `POST /start-app-assessment` | `idempotency-token` | `appArn`, `appVersion`, `assessmentName` | `clientToken` | `StartAppAssessmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new application assessment for an application. |
| `StartMetricsExport` | `POST /start-metrics-export` | `idempotency-token` | - | `clientToken` | `StartMetricsExportResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Initiates the export task of metrics. |
| `StartResourceGroupingRecommendationTask` | `POST /start-resource-grouping-recommendation-task` | - | `appArn` | - | `StartResourceGroupingRecommendationTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts grouping recommendation task. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Applies one or more tags to a resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes one or more tags from a resource. |
| `UpdateApp` | `POST /update-app` | - | `appArn` | - | `UpdateAppResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an application. |
| `UpdateAppVersion` | `POST /update-app-version` | - | `appArn` | - | `UpdateAppVersionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the Resilience Hub application version. This API updates the Resilience Hub application draft version. To use this information for running resiliency assessments, you must publish the Resilience Hub applicati ... |
| `UpdateAppVersionAppComponent` | `POST /update-app-version-app-component` | - | `appArn`, `id` | - | `UpdateAppVersionAppComponentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing Application Component in the Resilience Hub application. This API updates the Resilience Hub application draft version. To use this Application Component for running assessments, you must publish ... |
| `UpdateAppVersionResource` | `POST /update-app-version-resource` | - | `appArn` | - | `UpdateAppVersionResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the resource details in the Resilience Hub application. This action has no effect outside Resilience Hub. This API updates the Resilience Hub application draft version. To use this resource for running resili ... |
| `UpdateResiliencyPolicy` | `POST /update-resiliency-policy` | - | `policyArn` | - | `UpdateResiliencyPolicyResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a resiliency policy. Resilience Hub allows you to provide a value of zero for rtoInSecs and rpoInSecs of your resiliency policy. But, while assessing your application, the lowest possible assessment result is ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListAppAssessments` | - | `appArn -> appArn`, `assessmentName -> assessmentName`, `assessmentStatus -> assessmentStatus`, `complianceStatus -> complianceStatus`, `invoker -> invoker`, `reverseOrder -> reverseOrder`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListApps` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `name -> name`, `appArn -> appArn`, `fromLastAssessmentTime -> fromLastAssessmentTime`, `toLastAssessmentTime -> toLastAssessmentTime`, `reverseOrder -> reverseOrder`, `awsApplicationArn -> awsApplicationArn` | - | - |
| `ListRecommendationTemplates` | - | `assessmentArn -> assessmentArn`, `reverseOrder -> reverseOrder`, `status -> status`, `recommendationTemplateArn -> recommendationTemplateArn`, `name -> name`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListResiliencyPolicies` | - | `policyName -> policyName`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListResourceGroupingRecommendations` | - | `appArn -> appArn`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListSuggestedResiliencyPolicies` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy att ... |
| `ConflictException` | `structure` | message, resourceId, resourceType | This exception occurs when a conflict with a previous successful write is detected. This generally occurs when the previous write did not have time to propa ... |
| `InternalServerException` | `structure` | message | This exception occurs when there is an internal failure in the Resilience Hub service. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | This exception occurs when the specified resource could not be found. |
| `ServiceQuotaExceededException` | `structure` | message | This exception occurs when you have exceeded your service quota. To perform the requested action, remove some of the relevant resources, or use Service Quot ... |
| `ThrottlingException` | `structure` | message, retryAfterSeconds | This exception occurs when you have exceeded the limit on the number of requests per second. |
| `ValidationException` | `structure` | message | This exception occurs when a request is not valid. |
| `AcceptResourceGroupingRecommendationsRequest` | `structure` | appArn, entries | - |
| `AcceptResourceGroupingRecommendationsResponse` | `structure` | appArn, failedEntries | - |
| `AddDraftAppVersionResourceMappingsRequest` | `structure` | appArn, resourceMappings | - |
| `AddDraftAppVersionResourceMappingsResponse` | `structure` | appArn, appVersion, resourceMappings | - |
| `BatchUpdateRecommendationStatusRequest` | `structure` | appArn, requestEntries | - |
| `BatchUpdateRecommendationStatusResponse` | `structure` | appArn, successfulEntries, failedEntries | - |
| `CreateAppRequest` | `structure` | name, description, policyArn, tags, clientToken, assessmentSchedule, permissionModel, eventSubscriptions, awsApplicationArn | - |
| `CreateAppResponse` | `structure` | app | - |
| `CreateAppVersionAppComponentRequest` | `structure` | appArn, id, name, type, additionalInfo, clientToken | - |
| `CreateAppVersionAppComponentResponse` | `structure` | appArn, appVersion, appComponent | - |
| `CreateAppVersionResourceRequest` | `structure` | appArn, resourceName, logicalResourceId, physicalResourceId, awsRegion, awsAccountId, resourceType, appComponents, additionalInfo, clientToken | - |
| `CreateAppVersionResourceResponse` | `structure` | appArn, appVersion, physicalResource | - |
| `CreateRecommendationTemplateRequest` | `structure` | recommendationIds, format, recommendationTypes, assessmentArn, name, clientToken, tags, bucketName | - |
| `CreateRecommendationTemplateResponse` | `structure` | recommendationTemplate | - |
| `CreateResiliencyPolicyRequest` | `structure` | policyName, policyDescription, dataLocationConstraint, tier, policy, clientToken, tags | - |
| `CreateResiliencyPolicyResponse` | `structure` | policy | - |
| `DeleteAppRequest` | `structure` | appArn, forceDelete, clientToken | - |
| `DeleteAppResponse` | `structure` | appArn | - |
| `DeleteAppAssessmentRequest` | `structure` | assessmentArn, clientToken | - |
| `DeleteAppAssessmentResponse` | `structure` | assessmentArn, assessmentStatus | - |
| `DeleteAppInputSourceRequest` | `structure` | appArn, sourceArn, terraformSource, clientToken, eksSourceClusterNamespace | - |
| `DeleteAppInputSourceResponse` | `structure` | appArn, appInputSource | - |
| `DeleteAppVersionAppComponentRequest` | `structure` | appArn, id, clientToken | - |
| `DeleteAppVersionAppComponentResponse` | `structure` | appArn, appVersion, appComponent | - |
| `DeleteAppVersionResourceRequest` | `structure` | appArn, resourceName, logicalResourceId, physicalResourceId, awsRegion, awsAccountId, clientToken | - |
| `DeleteAppVersionResourceResponse` | `structure` | appArn, appVersion, physicalResource | - |
| `DeleteRecommendationTemplateRequest` | `structure` | recommendationTemplateArn, clientToken | - |
| `DeleteRecommendationTemplateResponse` | `structure` | recommendationTemplateArn, status | - |
| `DeleteResiliencyPolicyRequest` | `structure` | policyArn, clientToken | - |
| `DeleteResiliencyPolicyResponse` | `structure` | policyArn | - |
| `DescribeAppRequest` | `structure` | appArn | - |
| `DescribeAppResponse` | `structure` | app | - |
| `DescribeAppAssessmentRequest` | `structure` | assessmentArn | - |
| `AlarmType` | `enum` | METRIC, COMPOSITE, CANARY, LOGS, EVENT | - |
| `AppAssessmentScheduleType` | `enum` | DISABLED, DAILY | - |
| `AppComplianceStatusType` | `enum` | POLICY_BREACHED, POLICY_MET, NOT_ASSESSED, CHANGES_DETECTED, NOT_APPLICABLE, MISSING_POLICY | - |
| `AppDriftStatusType` | `enum` | NOT_CHECKED, NOT_DETECTED, DETECTED | - |
| `AppStatusType` | `enum` | ACTIVE, DELETING | - |
| `AssessmentInvoker` | `enum` | USER, SYSTEM | - |
| `AssessmentStatus` | `enum` | PENDING, INPROGRESS, FAILED, SUCCESS | - |
| `ComplianceStatus` | `enum` | POLICY_BREACHED, POLICY_MET, NOT_APPLICABLE, MISSING_POLICY | - |
| `ConditionOperatorType` | `enum` | EQUALS, NOT_EQUALS, GREATER_THEN, GREATER_OR_EQUALS, LESS_THEN, LESS_OR_EQUALS | - |
| `ConfigRecommendationOptimizationType` | `enum` | LEAST_COST, LEAST_CHANGE, BEST_AZ_RECOVERY, LEAST_ERRORS, BEST_ATTAINABLE, BEST_REGION_RECOVERY | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
