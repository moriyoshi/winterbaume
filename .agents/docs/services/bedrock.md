# Amazon Bedrock

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Describes the API operations for creating, managing, fine-turning, and evaluating Amazon Bedrock models.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Bedrock resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: manage foundation model access, custom/imported models, guardrails, prompts, inference profiles, provisioned throughput, and evaluation or customisation jobs.
- From the operation surface: support model lifecycle administration, model invocation logging, marketplace endpoints, batch/model-copy jobs, automated reasoning policies, and tag-based AI governance.

## Service Identity and Protocol

- AWS model slug: `bedrock`
- AWS SDK for Rust slug: `bedrock`
- Model version: `2023-04-20`
- Model file: `vendor/api-models-aws/models/bedrock/service/2023-04-20/bedrock-2023-04-20.json`
- SDK ID: `Bedrock`
- Endpoint prefix: `bedrock`
- ARN namespace: `-`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (24), `List` (21), `Create` (17), `Delete` (14), `Update` (7), `Put` (3), `Stop` (3), `Start` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchDeleteEvaluationJob`, `CancelAutomatedReasoningPolicyBuildWorkflow`, `CreateAutomatedReasoningPolicy`, `CreateAutomatedReasoningPolicyTestCase`, `CreateAutomatedReasoningPolicyVersion`, `CreateCustomModel`, `CreateCustomModelDeployment`, `CreateEvaluationJob`, `CreateFoundationModelAgreement`, `CreateGuardrail`, `CreateGuardrailVersion`, `CreateInferenceProfile`, `CreateMarketplaceModelEndpoint`, `CreateModelCopyJob`, `CreateModelCustomizationJob`, `CreateModelImportJob`, `CreateModelInvocationJob`, `CreatePromptRouter`, `CreateProvisionedModelThroughput`, `DeleteAutomatedReasoningPolicy`, ... (+32).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `ExportAutomatedReasoningPolicyVersion`, `GetAutomatedReasoningPolicy`, `GetAutomatedReasoningPolicyAnnotations`, `GetAutomatedReasoningPolicyBuildWorkflow`, `GetAutomatedReasoningPolicyBuildWorkflowResultAssets`, `GetAutomatedReasoningPolicyNextScenario`, `GetAutomatedReasoningPolicyTestCase`, `GetAutomatedReasoningPolicyTestResult`, `GetCustomModel`, `GetCustomModelDeployment`, `GetEvaluationJob`, `GetFoundationModel`, `GetFoundationModelAvailability`, `GetGuardrail`, `GetImportedModel`, `GetInferenceProfile`, `GetMarketplaceModelEndpoint`, `GetModelCopyJob`, `GetModelCustomizationJob`, `GetModelImportJob`, ... (+26).
- Pagination is modelled for 18 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 41 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `BatchDeleteEvaluationJob`, `CancelAutomatedReasoningPolicyBuildWorkflow`, `CreateEvaluationJob`, `CreateModelCopyJob`, `CreateModelCustomizationJob`, `CreateModelImportJob`, `CreateModelInvocationJob`, `DeleteImportedModel`, `ExportAutomatedReasoningPolicyVersion`, `GetEvaluationJob`, `GetImportedModel`, `GetModelCopyJob`, `GetModelCustomizationJob`, `GetModelImportJob`, `GetModelInvocationJob`, `ListEvaluationJobs`, `ListImportedModels`, `ListModelCopyJobs`, `ListModelCustomizationJobs`, `ListModelImportJobs`, ... (+6).
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 98 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `SNS`, `Lambda`, `EC2/VPC`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AllowlistResource` | - | - | `GetUseCaseForModelAccess`, `PutUseCaseForModelAccess` | - |
| `AutomatedReasoningPolicyResource` | `policyArn` | create: `CreateAutomatedReasoningPolicy`; read: `GetAutomatedReasoningPolicy`; update: `UpdateAutomatedReasoningPolicy`; delete: `DeleteAutomatedReasoningPolicy`; list: `ListAutomatedReasoningPolicies` | `CancelAutomatedReasoningPolicyBuildWorkflow`, `CreateAutomatedReasoningPolicyTestCase`, `CreateAutomatedReasoningPolicyVersion`, `DeleteAutomatedReasoningPolicyBuildWorkflow`, `DeleteAutomatedReasoningPolicyTestCase`, `ExportAutomatedReasoningPolicyVersion`, `GetAutomatedReasoningPolicyAnnotations`, `GetAutomatedReasoningPolicyBuildWorkflow`, `GetAutomatedReasoningPolicyBuildWorkflowResultAssets`, `GetAutomatedReasoningPolicyNextScenario`, ... (+9) | - |
| `BedrockMarketplaceResource` | - | - | `CreateMarketplaceModelEndpoint`, `DeleteMarketplaceModelEndpoint`, `DeregisterMarketplaceModelEndpoint`, `GetMarketplaceModelEndpoint`, `ListMarketplaceModelEndpoints`, `RegisterMarketplaceModelEndpoint`, `UpdateMarketplaceModelEndpoint` | - |
| `CustomModelDeploymentResource` | - | - | `CreateCustomModelDeployment`, `DeleteCustomModelDeployment`, `GetCustomModelDeployment`, `ListCustomModelDeployments`, `UpdateCustomModelDeployment` | - |
| `CustomModelResource` | - | - | `CreateCustomModel`, `DeleteCustomModel`, `GetCustomModel`, `ListCustomModels` | - |
| `EnforcedGuardrailConfigurationResource` | - | - | `DeleteEnforcedGuardrailConfiguration`, `ListEnforcedGuardrailsConfiguration`, `PutEnforcedGuardrailConfiguration` | - |
| `EvaluationJobResource` | - | - | `BatchDeleteEvaluationJob`, `CreateEvaluationJob`, `GetEvaluationJob`, `ListEvaluationJobs`, `StopEvaluationJob` | - |
| `GuardrailsResource` | `guardrailIdentifier` | create: `CreateGuardrail`; read: `GetGuardrail`; update: `UpdateGuardrail`; delete: `DeleteGuardrail`; list: `ListGuardrails` | `CreateGuardrailVersion` | - |
| `InferenceProfileResource` | `inferenceProfileIdentifier` | create: `CreateInferenceProfile`; read: `GetInferenceProfile`; delete: `DeleteInferenceProfile`; list: `ListInferenceProfiles` | - | - |
| `LoggingResource` | - | - | `DeleteModelInvocationLoggingConfiguration`, `GetModelInvocationLoggingConfiguration`, `PutModelInvocationLoggingConfiguration` | - |
| `ModelCopyResource` | - | - | `CreateModelCopyJob`, `GetModelCopyJob`, `ListModelCopyJobs` | - |
| `ModelImportResource` | - | - | `CreateModelImportJob`, `DeleteImportedModel`, `GetImportedModel`, `GetModelImportJob`, `ListImportedModels`, `ListModelImportJobs` | - |
| `ModelInvocationJobResource` | - | - | `CreateModelInvocationJob`, `GetModelInvocationJob`, `ListModelInvocationJobs`, `StopModelInvocationJob` | - |
| `ModelResource` | - | - | `GetFoundationModel`, `ListFoundationModels` | - |
| `PromptRouterResource` | `promptRouterArn` | create: `CreatePromptRouter`; read: `GetPromptRouter`; delete: `DeletePromptRouter`; list: `ListPromptRouters` | - | - |
| `ProvisionedModelThroughputResource` | - | - | `CreateProvisionedModelThroughput`, `DeleteProvisionedModelThroughput`, `GetProvisionedModelThroughput`, `ListProvisionedModelThroughputs`, `UpdateProvisionedModelThroughput` | - |
| `SubscriptionResource` | - | - | `CreateFoundationModelAgreement`, `DeleteFoundationModelAgreement`, `GetFoundationModelAvailability`, `ListFoundationModelAgreementOffers` | - |
| `TaggingResource` | - | - | `ListTagsForResource`, `TagResource`, `UntagResource` | - |
| `TrainingResource` | - | - | `CreateModelCustomizationJob`, `GetModelCustomizationJob`, `ListModelCustomizationJobs`, `StopModelCustomizationJob` | - |
## Operation Groups

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | The request is denied because of missing access permissions. |
| `ConflictException` | `structure` | message | Error occurred because of a conflict while performing an operation. |
| `InternalServerException` | `structure` | message | An internal server error occurred. Retry your request. |
| `ResourceInUseException` | `structure` | message | Thrown when attempting to delete or modify a resource that is currently being used by other resources or operations. For example, trying to delete an Automa ... |
| `ResourceNotFoundException` | `structure` | message | The specified resource Amazon Resource Name (ARN) was not found. Check the Amazon Resource Name (ARN) and try your request again. |
| `ServiceQuotaExceededException` | `structure` | message | The number of requests exceeds the service quota. Resubmit your request later. |
| `ServiceUnavailableException` | `structure` | message | Returned if the service cannot complete the request. |
| `ThrottlingException` | `structure` | message | The number of requests exceeds the limit. Resubmit your request later. |
| `TooManyTagsException` | `structure` | message, resourceName | The request contains more tags than can be associated with a resource (50 tags per resource). The maximum number of tags includes both existing tags and tho ... |
| `ValidationException` | `structure` | message | Input validation failed. Check your request parameters and retry the request. |
| `AgreementStatus` | `enum` | AVAILABLE, PENDING, NOT_AVAILABLE, ERROR | - |
| `ApplicationType` | `enum` | MODEL_EVALUATION, RAG_EVALUATION | - |
| `AttributeType` | `enum` | STRING, NUMBER, BOOLEAN, STRING_LIST | - |
| `AuthorizationStatus` | `enum` | AUTHORIZED, NOT_AUTHORIZED | - |
| `AutomatedReasoningCheckLogicWarningType` | `enum` | ALWAYS_TRUE, ALWAYS_FALSE | - |
| `AutomatedReasoningCheckResult` | `enum` | VALID, INVALID, SATISFIABLE, IMPOSSIBLE, TRANSLATION_AMBIGUOUS, TOO_COMPLEX, NO_TRANSLATION | - |
| `AutomatedReasoningPolicyAnnotationStatus` | `enum` | APPLIED, FAILED | - |
| `AutomatedReasoningPolicyBuildDocumentContentType` | `enum` | PDF, TEXT | - |
| `AutomatedReasoningPolicyBuildMessageType` | `enum` | INFO, WARNING, ERROR | - |
| `AutomatedReasoningPolicyBuildResultAssetType` | `enum` | BUILD_LOG, QUALITY_REPORT, POLICY_DEFINITION, GENERATED_TEST_CASES, POLICY_SCENARIOS, FIDELITY_REPORT, ASSET_MANIFEST, SOURCE_DOCUMENT | - |
| `AutomatedReasoningPolicyBuildWorkflowStatus` | `enum` | SCHEDULED, CANCEL_REQUESTED, PREPROCESSING, BUILDING, TESTING, COMPLETED, FAILED, CANCELLED | - |
| `AutomatedReasoningPolicyBuildWorkflowType` | `enum` | INGEST_CONTENT, REFINE_POLICY, IMPORT_POLICY, GENERATE_FIDELITY_REPORT, GENERATE_POLICY_SCENARIOS | - |
| `AutomatedReasoningPolicyTestRunResult` | `enum` | PASSED, FAILED | - |
| `AutomatedReasoningPolicyTestRunStatus` | `enum` | NOT_STARTED, SCHEDULED, IN_PROGRESS, COMPLETED, FAILED | - |
| `CommitmentDuration` | `enum` | ONE_MONTH, SIX_MONTHS | - |
| `CustomModelDeploymentStatus` | `enum` | CREATING, ACTIVE, FAILED | - |
| `CustomModelDeploymentUpdateStatus` | `enum` | UPDATING, UPDATE_COMPLETED, UPDATE_FAILED | - |
| `CustomizationType` | `enum` | FINE_TUNING, CONTINUED_PRE_TRAINING, DISTILLATION, REINFORCEMENT_FINE_TUNING, IMPORTED | - |
| `EntitlementAvailability` | `enum` | AVAILABLE, NOT_AVAILABLE | - |
| `EvaluationJobStatus` | `enum` | IN_PROGRESS, COMPLETED, FAILED, STOPPING, STOPPED, DELETING | - |
| `EvaluationJobType` | `enum` | HUMAN, AUTOMATED | - |
| `EvaluationTaskType` | `enum` | SUMMARIZATION, CLASSIFICATION, QUESTION_AND_ANSWER, GENERATION, CUSTOM | - |
| `ExternalSourceType` | `enum` | S3, BYTE_CONTENT | - |
| `FineTuningJobStatus` | `enum` | IN_PROGRESS, COMPLETED, FAILED, STOPPING, STOPPED | - |
| `FoundationModelLifecycleStatus` | `enum` | ACTIVE, LEGACY | - |
| `GuardrailContentFilterAction` | `enum` | BLOCK, NONE | - |
| `GuardrailContentFilterType` | `enum` | SEXUAL, VIOLENCE, HATE, INSULTS, MISCONDUCT, PROMPT_ATTACK | - |
| `GuardrailContentFiltersTierName` | `enum` | CLASSIC, STANDARD | - |
| `GuardrailContextualGroundingAction` | `enum` | BLOCK, NONE | - |
| `GuardrailContextualGroundingFilterType` | `enum` | GROUNDING, RELEVANCE | - |
| `GuardrailFilterStrength` | `enum` | NONE, LOW, MEDIUM, HIGH | - |
| `GuardrailManagedWordsType` | `enum` | PROFANITY | - |
| `GuardrailModality` | `enum` | TEXT, IMAGE | - |
| `GuardrailPiiEntityType` | `enum` | ADDRESS, AGE, AWS_ACCESS_KEY, AWS_SECRET_KEY, CA_HEALTH_NUMBER, CA_SOCIAL_INSURANCE_NUMBER, CREDIT_DEBIT_CARD_CVV, CREDIT_DEBIT_CARD_EXPIRY, CREDIT_DEBIT_CARD_NUMBER, DRIVER_ID, EMAIL, INTERNATIONAL_BANK_ACCOUNT_NUMBER, ... (+19) | - |
| `GuardrailSensitiveInformationAction` | `enum` | BLOCK, ANONYMIZE, NONE | - |
| `GuardrailStatus` | `enum` | CREATING, UPDATING, VERSIONING, READY, FAILED, DELETING | - |
| `GuardrailTopicAction` | `enum` | BLOCK, NONE | - |
| `GuardrailTopicType` | `enum` | DENY | - |
| `GuardrailTopicsTierName` | `enum` | CLASSIC, STANDARD | - |
| `GuardrailWordAction` | `enum` | BLOCK, NONE | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
