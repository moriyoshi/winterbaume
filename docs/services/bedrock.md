# winterbaume-bedrock

Amazon Bedrock service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Bedrock |
| AWS model | `bedrock` |
| Protocol | restJson1 |
| winterbaume coverage | 48/101 operations (47.5%) |
| stubs (routed, returns empty/default) | 0/101 operations (0.0%) |
| moto coverage | 13/101 operations (12.9%) |
| floci coverage | 0/101 operations (0.0%) |
| kumo coverage | 0/101 operations (0.0%) |
| Coverage report date | 2026-05-17 |

## Server-mode usage

Install `winterbaume-server` from crates.io or run it from a workspace checkout, then point the AWS CLI at it:

```sh
# Installed binary ( from crates.io ):
cargo install winterbaume-server
winterbaume-server --host 127.0.0.1 --port 5555

# Or, from a workspace checkout:
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws bedrock list-foundation-models
```

## Example

```rust
use aws_sdk_bedrock::config::BehaviorVersion;
use winterbaume_bedrock::BedrockService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(BedrockService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_bedrock::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_bedrock::Client::new(&config);

    let resp = client
        .list_foundation_models()
        .send()
        .await
        .expect("list_foundation_models should succeed");
    println!("Foundation models: {}", resp.model_summaries().len());
}
```

## Implemented APIs (48)

- `CreateEvaluationJob`
- `CreateGuardrail`
- `CreateGuardrailVersion`
- `CreateInferenceProfile`
- `CreateModelCopyJob`
- `CreateModelCustomizationJob`
- `CreateModelImportJob`
- `CreateModelInvocationJob`
- `CreatePromptRouter`
- `CreateProvisionedModelThroughput`
- `DeleteCustomModel`
- `DeleteGuardrail`
- `DeleteInferenceProfile`
- `DeleteModelInvocationLoggingConfiguration`
- `DeletePromptRouter`
- `DeleteProvisionedModelThroughput`
- `GetCustomModel`
- `GetEvaluationJob`
- `GetFoundationModel`
- `GetGuardrail`
- `GetInferenceProfile`
- `GetModelCopyJob`
- `GetModelCustomizationJob`
- `GetModelImportJob`
- `GetModelInvocationJob`
- `GetModelInvocationLoggingConfiguration`
- `GetPromptRouter`
- `GetProvisionedModelThroughput`
- `ListCustomModels`
- `ListEvaluationJobs`
- `ListFoundationModels`
- `ListGuardrails`
- `ListInferenceProfiles`
- `ListModelCopyJobs`
- `ListModelCustomizationJobs`
- `ListModelImportJobs`
- `ListModelInvocationJobs`
- `ListPromptRouters`
- `ListProvisionedModelThroughputs`
- `ListTagsForResource`
- `PutModelInvocationLoggingConfiguration`
- `StopEvaluationJob`
- `StopModelCustomizationJob`
- `StopModelInvocationJob`
- `TagResource`
- `UntagResource`
- `UpdateGuardrail`
- `UpdateProvisionedModelThroughput`

<details><summary>Not yet implemented APIs (53)</summary>

- `BatchDeleteEvaluationJob`
- `CancelAutomatedReasoningPolicyBuildWorkflow`
- `CreateAutomatedReasoningPolicy`
- `CreateAutomatedReasoningPolicyTestCase`
- `CreateAutomatedReasoningPolicyVersion`
- `CreateCustomModel`
- `CreateCustomModelDeployment`
- `CreateFoundationModelAgreement`
- `CreateMarketplaceModelEndpoint`
- `DeleteAutomatedReasoningPolicy`
- `DeleteAutomatedReasoningPolicyBuildWorkflow`
- `DeleteAutomatedReasoningPolicyTestCase`
- `DeleteCustomModelDeployment`
- `DeleteEnforcedGuardrailConfiguration`
- `DeleteFoundationModelAgreement`
- `DeleteImportedModel`
- `DeleteMarketplaceModelEndpoint`
- `DeleteResourcePolicy`
- `DeregisterMarketplaceModelEndpoint`
- `ExportAutomatedReasoningPolicyVersion`
- `GetAutomatedReasoningPolicy`
- `GetAutomatedReasoningPolicyAnnotations`
- `GetAutomatedReasoningPolicyBuildWorkflow`
- `GetAutomatedReasoningPolicyBuildWorkflowResultAssets`
- `GetAutomatedReasoningPolicyNextScenario`
- `GetAutomatedReasoningPolicyTestCase`
- `GetAutomatedReasoningPolicyTestResult`
- `GetCustomModelDeployment`
- `GetFoundationModelAvailability`
- `GetImportedModel`
- `GetMarketplaceModelEndpoint`
- `GetResourcePolicy`
- `GetUseCaseForModelAccess`
- `ListAutomatedReasoningPolicies`
- `ListAutomatedReasoningPolicyBuildWorkflows`
- `ListAutomatedReasoningPolicyTestCases`
- `ListAutomatedReasoningPolicyTestResults`
- `ListCustomModelDeployments`
- `ListEnforcedGuardrailsConfiguration`
- `ListFoundationModelAgreementOffers`
- `ListImportedModels`
- `ListMarketplaceModelEndpoints`
- `PutEnforcedGuardrailConfiguration`
- `PutResourcePolicy`
- `PutUseCaseForModelAccess`
- `RegisterMarketplaceModelEndpoint`
- `StartAutomatedReasoningPolicyBuildWorkflow`
- `StartAutomatedReasoningPolicyTestWorkflow`
- `UpdateAutomatedReasoningPolicy`
- `UpdateAutomatedReasoningPolicyAnnotations`
- `UpdateAutomatedReasoningPolicyTestCase`
- `UpdateCustomModelDeployment`
- `UpdateMarketplaceModelEndpoint`

</details>
