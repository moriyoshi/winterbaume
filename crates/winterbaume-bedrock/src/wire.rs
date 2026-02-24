//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-bedrock

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

#[allow(unused_imports)]
use http::header::HeaderName;
use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for restJson protocol.
pub fn serialize_batch_delete_evaluation_job_response(
    result: &BatchDeleteEvaluationJobResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_cancel_automated_reasoning_policy_build_workflow_response(
    result: &CancelAutomatedReasoningPolicyBuildWorkflowResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_automated_reasoning_policy_response(
    result: &CreateAutomatedReasoningPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_automated_reasoning_policy_test_case_response(
    result: &CreateAutomatedReasoningPolicyTestCaseResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_automated_reasoning_policy_version_response(
    result: &CreateAutomatedReasoningPolicyVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_custom_model_response(result: &CreateCustomModelResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_custom_model_deployment_response(
    result: &CreateCustomModelDeploymentResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_evaluation_job_response(
    result: &CreateEvaluationJobResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_foundation_model_agreement_response(
    result: &CreateFoundationModelAgreementResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_guardrail_response(result: &CreateGuardrailResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_guardrail_version_response(
    result: &CreateGuardrailVersionResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_inference_profile_response(
    result: &CreateInferenceProfileResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_marketplace_model_endpoint_response(
    result: &CreateMarketplaceModelEndpointResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_model_copy_job_response(
    result: &CreateModelCopyJobResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_model_customization_job_response(
    result: &CreateModelCustomizationJobResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_model_import_job_response(
    result: &CreateModelImportJobResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_model_invocation_job_response(
    result: &CreateModelInvocationJobResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_prompt_router_response(
    result: &CreatePromptRouterResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_provisioned_model_throughput_response(
    result: &CreateProvisionedModelThroughputResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_automated_reasoning_policy_response(
    result: &DeleteAutomatedReasoningPolicyResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_automated_reasoning_policy_build_workflow_response(
    result: &DeleteAutomatedReasoningPolicyBuildWorkflowResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_automated_reasoning_policy_test_case_response(
    result: &DeleteAutomatedReasoningPolicyTestCaseResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_custom_model_response(result: &DeleteCustomModelResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_custom_model_deployment_response(
    result: &DeleteCustomModelDeploymentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_enforced_guardrail_configuration_response(
    result: &DeleteEnforcedGuardrailConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_foundation_model_agreement_response(
    result: &DeleteFoundationModelAgreementResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_guardrail_response(result: &DeleteGuardrailResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_imported_model_response(
    result: &DeleteImportedModelResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_inference_profile_response(
    result: &DeleteInferenceProfileResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_marketplace_model_endpoint_response(
    result: &DeleteMarketplaceModelEndpointResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_model_invocation_logging_configuration_response(
    result: &DeleteModelInvocationLoggingConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_prompt_router_response(
    result: &DeletePromptRouterResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_provisioned_model_throughput_response(
    result: &DeleteProvisionedModelThroughputResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_resource_policy_response(
    result: &DeleteResourcePolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_deregister_marketplace_model_endpoint_response(
    result: &DeregisterMarketplaceModelEndpointResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_export_automated_reasoning_policy_version_response(
    result: &ExportAutomatedReasoningPolicyVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.policy_definition).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_automated_reasoning_policy_response(
    result: &GetAutomatedReasoningPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_automated_reasoning_policy_annotations_response(
    result: &GetAutomatedReasoningPolicyAnnotationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_automated_reasoning_policy_build_workflow_response(
    result: &GetAutomatedReasoningPolicyBuildWorkflowResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_automated_reasoning_policy_build_workflow_result_assets_response(
    result: &GetAutomatedReasoningPolicyBuildWorkflowResultAssetsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_automated_reasoning_policy_next_scenario_response(
    result: &GetAutomatedReasoningPolicyNextScenarioResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_automated_reasoning_policy_test_case_response(
    result: &GetAutomatedReasoningPolicyTestCaseResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_automated_reasoning_policy_test_result_response(
    result: &GetAutomatedReasoningPolicyTestResultResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_custom_model_response(result: &GetCustomModelResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_custom_model_deployment_response(
    result: &GetCustomModelDeploymentResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_evaluation_job_response(result: &GetEvaluationJobResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_foundation_model_response(
    result: &GetFoundationModelResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_foundation_model_availability_response(
    result: &GetFoundationModelAvailabilityResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_guardrail_response(result: &GetGuardrailResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_imported_model_response(result: &GetImportedModelResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_inference_profile_response(
    result: &GetInferenceProfileResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_marketplace_model_endpoint_response(
    result: &GetMarketplaceModelEndpointResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_model_copy_job_response(result: &GetModelCopyJobResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_model_customization_job_response(
    result: &GetModelCustomizationJobResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_model_import_job_response(result: &GetModelImportJobResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_model_invocation_job_response(
    result: &GetModelInvocationJobResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_model_invocation_logging_configuration_response(
    result: &GetModelInvocationLoggingConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_prompt_router_response(result: &GetPromptRouterResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_provisioned_model_throughput_response(
    result: &GetProvisionedModelThroughputResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_resource_policy_response(result: &GetResourcePolicyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_use_case_for_model_access_response(
    result: &GetUseCaseForModelAccessResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_automated_reasoning_policies_response(
    result: &ListAutomatedReasoningPoliciesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_automated_reasoning_policy_build_workflows_response(
    result: &ListAutomatedReasoningPolicyBuildWorkflowsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_automated_reasoning_policy_test_cases_response(
    result: &ListAutomatedReasoningPolicyTestCasesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_automated_reasoning_policy_test_results_response(
    result: &ListAutomatedReasoningPolicyTestResultsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_custom_model_deployments_response(
    result: &ListCustomModelDeploymentsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_custom_models_response(result: &ListCustomModelsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_enforced_guardrails_configuration_response(
    result: &ListEnforcedGuardrailsConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_evaluation_jobs_response(
    result: &ListEvaluationJobsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_foundation_model_agreement_offers_response(
    result: &ListFoundationModelAgreementOffersResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_foundation_models_response(
    result: &ListFoundationModelsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_guardrails_response(result: &ListGuardrailsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_imported_models_response(
    result: &ListImportedModelsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_inference_profiles_response(
    result: &ListInferenceProfilesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_marketplace_model_endpoints_response(
    result: &ListMarketplaceModelEndpointsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_model_copy_jobs_response(result: &ListModelCopyJobsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_model_customization_jobs_response(
    result: &ListModelCustomizationJobsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_model_import_jobs_response(
    result: &ListModelImportJobsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_model_invocation_jobs_response(
    result: &ListModelInvocationJobsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_prompt_routers_response(result: &ListPromptRoutersResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_provisioned_model_throughputs_response(
    result: &ListProvisionedModelThroughputsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_enforced_guardrail_configuration_response(
    result: &PutEnforcedGuardrailConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_model_invocation_logging_configuration_response(
    result: &PutModelInvocationLoggingConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_resource_policy_response(result: &PutResourcePolicyResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_use_case_for_model_access_response(
    result: &PutUseCaseForModelAccessResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_register_marketplace_model_endpoint_response(
    result: &RegisterMarketplaceModelEndpointResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_automated_reasoning_policy_build_workflow_response(
    result: &StartAutomatedReasoningPolicyBuildWorkflowResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_automated_reasoning_policy_test_workflow_response(
    result: &StartAutomatedReasoningPolicyTestWorkflowResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_evaluation_job_response(result: &StopEvaluationJobResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_model_customization_job_response(
    result: &StopModelCustomizationJobResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_model_invocation_job_response(
    result: &StopModelInvocationJobResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_automated_reasoning_policy_response(
    result: &UpdateAutomatedReasoningPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_automated_reasoning_policy_annotations_response(
    result: &UpdateAutomatedReasoningPolicyAnnotationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_automated_reasoning_policy_test_case_response(
    result: &UpdateAutomatedReasoningPolicyTestCaseResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_custom_model_deployment_response(
    result: &UpdateCustomModelDeploymentResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_guardrail_response(result: &UpdateGuardrailResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_marketplace_model_endpoint_response(
    result: &UpdateMarketplaceModelEndpointResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_provisioned_model_throughput_response(
    result: &UpdateProvisionedModelThroughputResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_delete_evaluation_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchDeleteEvaluationJobRequest, String> {
    let mut input = BatchDeleteEvaluationJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchDeleteEvaluationJobRequest>(&request.body).map_err(
            |err| format!("failed to deserialize BatchDeleteEvaluationJob request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_automated_reasoning_policy_build_workflow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelAutomatedReasoningPolicyBuildWorkflowRequest, String> {
    let mut input = CancelAutomatedReasoningPolicyBuildWorkflowRequest::default();
    for (name, value) in labels {
        match *name {
            "buildWorkflowId" => {
                input.build_workflow_id = value.to_string();
            }
            "policyArn" => {
                input.policy_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_automated_reasoning_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAutomatedReasoningPolicyRequest, String> {
    let mut input = CreateAutomatedReasoningPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAutomatedReasoningPolicyRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateAutomatedReasoningPolicy request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_automated_reasoning_policy_test_case_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAutomatedReasoningPolicyTestCaseRequest, String> {
    let mut input = CreateAutomatedReasoningPolicyTestCaseRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAutomatedReasoningPolicyTestCaseRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!("failed to deserialize CreateAutomatedReasoningPolicyTestCase request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "policyArn" => {
                input.policy_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_automated_reasoning_policy_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAutomatedReasoningPolicyVersionRequest, String> {
    let mut input = CreateAutomatedReasoningPolicyVersionRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<CreateAutomatedReasoningPolicyVersionRequest>(&request.body)
                .map_err(|err| {
                format!(
                    "failed to deserialize CreateAutomatedReasoningPolicyVersion request: {err}"
                )
            })?;
    }
    for (name, value) in labels {
        match *name {
            "policyArn" => {
                input.policy_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_custom_model_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCustomModelRequest, String> {
    let mut input = CreateCustomModelRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateCustomModelRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateCustomModel request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_custom_model_deployment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCustomModelDeploymentRequest, String> {
    let mut input = CreateCustomModelDeploymentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateCustomModelDeploymentRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateCustomModelDeployment request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_evaluation_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateEvaluationJobRequest, String> {
    let mut input = CreateEvaluationJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateEvaluationJobRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateEvaluationJob request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_foundation_model_agreement_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFoundationModelAgreementRequest, String> {
    let mut input = CreateFoundationModelAgreementRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateFoundationModelAgreementRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateFoundationModelAgreement request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_guardrail_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateGuardrailRequest, String> {
    let mut input = CreateGuardrailRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateGuardrailRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateGuardrail request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_guardrail_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateGuardrailVersionRequest, String> {
    let mut input = CreateGuardrailVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateGuardrailVersionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateGuardrailVersion request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "guardrailIdentifier" => {
                input.guardrail_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_inference_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateInferenceProfileRequest, String> {
    let mut input = CreateInferenceProfileRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateInferenceProfileRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateInferenceProfile request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_marketplace_model_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateMarketplaceModelEndpointRequest, String> {
    let mut input = CreateMarketplaceModelEndpointRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateMarketplaceModelEndpointRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateMarketplaceModelEndpoint request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_model_copy_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateModelCopyJobRequest, String> {
    let mut input = CreateModelCopyJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateModelCopyJobRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateModelCopyJob request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_model_customization_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateModelCustomizationJobRequest, String> {
    let mut input = CreateModelCustomizationJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateModelCustomizationJobRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateModelCustomizationJob request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_model_import_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateModelImportJobRequest, String> {
    let mut input = CreateModelImportJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateModelImportJobRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateModelImportJob request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_model_invocation_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateModelInvocationJobRequest, String> {
    let mut input = CreateModelInvocationJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateModelInvocationJobRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateModelInvocationJob request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_prompt_router_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreatePromptRouterRequest, String> {
    let mut input = CreatePromptRouterRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreatePromptRouterRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreatePromptRouter request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_provisioned_model_throughput_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateProvisionedModelThroughputRequest, String> {
    let mut input = CreateProvisionedModelThroughputRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateProvisionedModelThroughputRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateProvisionedModelThroughput request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_automated_reasoning_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAutomatedReasoningPolicyRequest, String> {
    let mut input = DeleteAutomatedReasoningPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "policyArn" => {
                input.policy_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("force") {
        input.force = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_automated_reasoning_policy_build_workflow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAutomatedReasoningPolicyBuildWorkflowRequest, String> {
    let mut input = DeleteAutomatedReasoningPolicyBuildWorkflowRequest::default();
    for (name, value) in labels {
        match *name {
            "buildWorkflowId" => {
                input.build_workflow_id = value.to_string();
            }
            "policyArn" => {
                input.policy_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("updatedAt") {
        input.last_updated_at = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_automated_reasoning_policy_test_case_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAutomatedReasoningPolicyTestCaseRequest, String> {
    let mut input = DeleteAutomatedReasoningPolicyTestCaseRequest::default();
    for (name, value) in labels {
        match *name {
            "policyArn" => {
                input.policy_arn = value.to_string();
            }
            "testCaseId" => {
                input.test_case_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("updatedAt") {
        input.last_updated_at = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_custom_model_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCustomModelRequest, String> {
    let mut input = DeleteCustomModelRequest::default();
    for (name, value) in labels {
        match *name {
            "modelIdentifier" => {
                input.model_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_custom_model_deployment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCustomModelDeploymentRequest, String> {
    let mut input = DeleteCustomModelDeploymentRequest::default();
    for (name, value) in labels {
        match *name {
            "customModelDeploymentIdentifier" => {
                input.custom_model_deployment_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_enforced_guardrail_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteEnforcedGuardrailConfigurationRequest, String> {
    let mut input = DeleteEnforcedGuardrailConfigurationRequest::default();
    for (name, value) in labels {
        match *name {
            "configId" => {
                input.config_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_foundation_model_agreement_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFoundationModelAgreementRequest, String> {
    let mut input = DeleteFoundationModelAgreementRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteFoundationModelAgreementRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DeleteFoundationModelAgreement request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_guardrail_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteGuardrailRequest, String> {
    let mut input = DeleteGuardrailRequest::default();
    for (name, value) in labels {
        match *name {
            "guardrailIdentifier" => {
                input.guardrail_identifier = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("guardrailVersion") {
        input.guardrail_version = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_imported_model_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteImportedModelRequest, String> {
    let mut input = DeleteImportedModelRequest::default();
    for (name, value) in labels {
        match *name {
            "modelIdentifier" => {
                input.model_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_inference_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteInferenceProfileRequest, String> {
    let mut input = DeleteInferenceProfileRequest::default();
    for (name, value) in labels {
        match *name {
            "inferenceProfileIdentifier" => {
                input.inference_profile_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_marketplace_model_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteMarketplaceModelEndpointRequest, String> {
    let mut input = DeleteMarketplaceModelEndpointRequest::default();
    for (name, value) in labels {
        match *name {
            "endpointArn" => {
                input.endpoint_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_model_invocation_logging_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteModelInvocationLoggingConfigurationRequest, String> {
    let input = DeleteModelInvocationLoggingConfigurationRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_prompt_router_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePromptRouterRequest, String> {
    let mut input = DeletePromptRouterRequest::default();
    for (name, value) in labels {
        match *name {
            "promptRouterArn" => {
                input.prompt_router_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_provisioned_model_throughput_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteProvisionedModelThroughputRequest, String> {
    let mut input = DeleteProvisionedModelThroughputRequest::default();
    for (name, value) in labels {
        match *name {
            "provisionedModelId" => {
                input.provisioned_model_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteResourcePolicyRequest, String> {
    let mut input = DeleteResourcePolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_deregister_marketplace_model_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeregisterMarketplaceModelEndpointRequest, String> {
    let mut input = DeregisterMarketplaceModelEndpointRequest::default();
    for (name, value) in labels {
        match *name {
            "endpointArn" => {
                input.endpoint_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_export_automated_reasoning_policy_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ExportAutomatedReasoningPolicyVersionRequest, String> {
    let mut input = ExportAutomatedReasoningPolicyVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "policyArn" => {
                input.policy_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_automated_reasoning_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAutomatedReasoningPolicyRequest, String> {
    let mut input = GetAutomatedReasoningPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "policyArn" => {
                input.policy_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_automated_reasoning_policy_annotations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAutomatedReasoningPolicyAnnotationsRequest, String> {
    let mut input = GetAutomatedReasoningPolicyAnnotationsRequest::default();
    for (name, value) in labels {
        match *name {
            "buildWorkflowId" => {
                input.build_workflow_id = value.to_string();
            }
            "policyArn" => {
                input.policy_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_automated_reasoning_policy_build_workflow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAutomatedReasoningPolicyBuildWorkflowRequest, String> {
    let mut input = GetAutomatedReasoningPolicyBuildWorkflowRequest::default();
    for (name, value) in labels {
        match *name {
            "buildWorkflowId" => {
                input.build_workflow_id = value.to_string();
            }
            "policyArn" => {
                input.policy_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_automated_reasoning_policy_build_workflow_result_assets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAutomatedReasoningPolicyBuildWorkflowResultAssetsRequest, String> {
    let mut input = GetAutomatedReasoningPolicyBuildWorkflowResultAssetsRequest::default();
    for (name, value) in labels {
        match *name {
            "buildWorkflowId" => {
                input.build_workflow_id = value.to_string();
            }
            "policyArn" => {
                input.policy_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("assetId") {
        input.asset_id = Some(value.to_string());
    }
    if let Some(value) = query.get("assetType") {
        input.asset_type = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_automated_reasoning_policy_next_scenario_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAutomatedReasoningPolicyNextScenarioRequest, String> {
    let mut input = GetAutomatedReasoningPolicyNextScenarioRequest::default();
    for (name, value) in labels {
        match *name {
            "buildWorkflowId" => {
                input.build_workflow_id = value.to_string();
            }
            "policyArn" => {
                input.policy_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_automated_reasoning_policy_test_case_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAutomatedReasoningPolicyTestCaseRequest, String> {
    let mut input = GetAutomatedReasoningPolicyTestCaseRequest::default();
    for (name, value) in labels {
        match *name {
            "policyArn" => {
                input.policy_arn = value.to_string();
            }
            "testCaseId" => {
                input.test_case_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_automated_reasoning_policy_test_result_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAutomatedReasoningPolicyTestResultRequest, String> {
    let mut input = GetAutomatedReasoningPolicyTestResultRequest::default();
    for (name, value) in labels {
        match *name {
            "buildWorkflowId" => {
                input.build_workflow_id = value.to_string();
            }
            "policyArn" => {
                input.policy_arn = value.to_string();
            }
            "testCaseId" => {
                input.test_case_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_custom_model_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCustomModelRequest, String> {
    let mut input = GetCustomModelRequest::default();
    for (name, value) in labels {
        match *name {
            "modelIdentifier" => {
                input.model_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_custom_model_deployment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCustomModelDeploymentRequest, String> {
    let mut input = GetCustomModelDeploymentRequest::default();
    for (name, value) in labels {
        match *name {
            "customModelDeploymentIdentifier" => {
                input.custom_model_deployment_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_evaluation_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetEvaluationJobRequest, String> {
    let mut input = GetEvaluationJobRequest::default();
    for (name, value) in labels {
        match *name {
            "jobIdentifier" => {
                input.job_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_foundation_model_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFoundationModelRequest, String> {
    let mut input = GetFoundationModelRequest::default();
    for (name, value) in labels {
        match *name {
            "modelIdentifier" => {
                input.model_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_foundation_model_availability_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFoundationModelAvailabilityRequest, String> {
    let mut input = GetFoundationModelAvailabilityRequest::default();
    for (name, value) in labels {
        match *name {
            "modelId" => {
                input.model_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_guardrail_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetGuardrailRequest, String> {
    let mut input = GetGuardrailRequest::default();
    for (name, value) in labels {
        match *name {
            "guardrailIdentifier" => {
                input.guardrail_identifier = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("guardrailVersion") {
        input.guardrail_version = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_imported_model_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetImportedModelRequest, String> {
    let mut input = GetImportedModelRequest::default();
    for (name, value) in labels {
        match *name {
            "modelIdentifier" => {
                input.model_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_inference_profile_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetInferenceProfileRequest, String> {
    let mut input = GetInferenceProfileRequest::default();
    for (name, value) in labels {
        match *name {
            "inferenceProfileIdentifier" => {
                input.inference_profile_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_marketplace_model_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMarketplaceModelEndpointRequest, String> {
    let mut input = GetMarketplaceModelEndpointRequest::default();
    for (name, value) in labels {
        match *name {
            "endpointArn" => {
                input.endpoint_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_model_copy_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetModelCopyJobRequest, String> {
    let mut input = GetModelCopyJobRequest::default();
    for (name, value) in labels {
        match *name {
            "jobArn" => {
                input.job_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_model_customization_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetModelCustomizationJobRequest, String> {
    let mut input = GetModelCustomizationJobRequest::default();
    for (name, value) in labels {
        match *name {
            "jobIdentifier" => {
                input.job_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_model_import_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetModelImportJobRequest, String> {
    let mut input = GetModelImportJobRequest::default();
    for (name, value) in labels {
        match *name {
            "jobIdentifier" => {
                input.job_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_model_invocation_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetModelInvocationJobRequest, String> {
    let mut input = GetModelInvocationJobRequest::default();
    for (name, value) in labels {
        match *name {
            "jobIdentifier" => {
                input.job_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_model_invocation_logging_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetModelInvocationLoggingConfigurationRequest, String> {
    let input = GetModelInvocationLoggingConfigurationRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_prompt_router_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPromptRouterRequest, String> {
    let mut input = GetPromptRouterRequest::default();
    for (name, value) in labels {
        match *name {
            "promptRouterArn" => {
                input.prompt_router_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_provisioned_model_throughput_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetProvisionedModelThroughputRequest, String> {
    let mut input = GetProvisionedModelThroughputRequest::default();
    for (name, value) in labels {
        match *name {
            "provisionedModelId" => {
                input.provisioned_model_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetResourcePolicyRequest, String> {
    let mut input = GetResourcePolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_use_case_for_model_access_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetUseCaseForModelAccessRequest, String> {
    let input = GetUseCaseForModelAccessRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_automated_reasoning_policies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAutomatedReasoningPoliciesRequest, String> {
    let mut input = ListAutomatedReasoningPoliciesRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("policyArn") {
        input.policy_arn = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_automated_reasoning_policy_build_workflows_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAutomatedReasoningPolicyBuildWorkflowsRequest, String> {
    let mut input = ListAutomatedReasoningPolicyBuildWorkflowsRequest::default();
    for (name, value) in labels {
        match *name {
            "policyArn" => {
                input.policy_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_automated_reasoning_policy_test_cases_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAutomatedReasoningPolicyTestCasesRequest, String> {
    let mut input = ListAutomatedReasoningPolicyTestCasesRequest::default();
    for (name, value) in labels {
        match *name {
            "policyArn" => {
                input.policy_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_automated_reasoning_policy_test_results_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAutomatedReasoningPolicyTestResultsRequest, String> {
    let mut input = ListAutomatedReasoningPolicyTestResultsRequest::default();
    for (name, value) in labels {
        match *name {
            "buildWorkflowId" => {
                input.build_workflow_id = value.to_string();
            }
            "policyArn" => {
                input.policy_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_custom_model_deployments_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCustomModelDeploymentsRequest, String> {
    let mut input = ListCustomModelDeploymentsRequest::default();
    if let Some(value) = query.get("createdAfter") {
        input.created_after = Some(value.to_string());
    }
    if let Some(value) = query.get("createdBefore") {
        input.created_before = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("modelArnEquals") {
        input.model_arn_equals = Some(value.to_string());
    }
    if let Some(value) = query.get("nameContains") {
        input.name_contains = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("sortBy") {
        input.sort_by = Some(value.to_string());
    }
    if let Some(value) = query.get("sortOrder") {
        input.sort_order = Some(value.to_string());
    }
    if let Some(value) = query.get("statusEquals") {
        input.status_equals = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_custom_models_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCustomModelsRequest, String> {
    let mut input = ListCustomModelsRequest::default();
    if let Some(value) = query.get("baseModelArnEquals") {
        input.base_model_arn_equals = Some(value.to_string());
    }
    if let Some(value) = query.get("creationTimeAfter") {
        input.creation_time_after = Some(value.to_string());
    }
    if let Some(value) = query.get("creationTimeBefore") {
        input.creation_time_before = Some(value.to_string());
    }
    if let Some(value) = query.get("foundationModelArnEquals") {
        input.foundation_model_arn_equals = Some(value.to_string());
    }
    if let Some(value) = query.get("isOwned") {
        input.is_owned = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("modelStatus") {
        input.model_status = Some(value.to_string());
    }
    if let Some(value) = query.get("nameContains") {
        input.name_contains = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("sortBy") {
        input.sort_by = Some(value.to_string());
    }
    if let Some(value) = query.get("sortOrder") {
        input.sort_order = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_enforced_guardrails_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListEnforcedGuardrailsConfigurationRequest, String> {
    let mut input = ListEnforcedGuardrailsConfigurationRequest::default();
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_evaluation_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListEvaluationJobsRequest, String> {
    let mut input = ListEvaluationJobsRequest::default();
    if let Some(value) = query.get("applicationTypeEquals") {
        input.application_type_equals = Some(value.to_string());
    }
    if let Some(value) = query.get("creationTimeAfter") {
        input.creation_time_after = Some(value.to_string());
    }
    if let Some(value) = query.get("creationTimeBefore") {
        input.creation_time_before = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nameContains") {
        input.name_contains = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("sortBy") {
        input.sort_by = Some(value.to_string());
    }
    if let Some(value) = query.get("sortOrder") {
        input.sort_order = Some(value.to_string());
    }
    if let Some(value) = query.get("statusEquals") {
        input.status_equals = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_foundation_model_agreement_offers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFoundationModelAgreementOffersRequest, String> {
    let mut input = ListFoundationModelAgreementOffersRequest::default();
    for (name, value) in labels {
        match *name {
            "modelId" => {
                input.model_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("offerType") {
        input.offer_type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_foundation_models_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFoundationModelsRequest, String> {
    let mut input = ListFoundationModelsRequest::default();
    if let Some(value) = query.get("byCustomizationType") {
        input.by_customization_type = Some(value.to_string());
    }
    if let Some(value) = query.get("byInferenceType") {
        input.by_inference_type = Some(value.to_string());
    }
    if let Some(value) = query.get("byOutputModality") {
        input.by_output_modality = Some(value.to_string());
    }
    if let Some(value) = query.get("byProvider") {
        input.by_provider = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_guardrails_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListGuardrailsRequest, String> {
    let mut input = ListGuardrailsRequest::default();
    if let Some(value) = query.get("guardrailIdentifier") {
        input.guardrail_identifier = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_imported_models_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListImportedModelsRequest, String> {
    let mut input = ListImportedModelsRequest::default();
    if let Some(value) = query.get("creationTimeAfter") {
        input.creation_time_after = Some(value.to_string());
    }
    if let Some(value) = query.get("creationTimeBefore") {
        input.creation_time_before = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nameContains") {
        input.name_contains = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("sortBy") {
        input.sort_by = Some(value.to_string());
    }
    if let Some(value) = query.get("sortOrder") {
        input.sort_order = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_inference_profiles_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListInferenceProfilesRequest, String> {
    let mut input = ListInferenceProfilesRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("type") {
        input.type_equals = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_marketplace_model_endpoints_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListMarketplaceModelEndpointsRequest, String> {
    let mut input = ListMarketplaceModelEndpointsRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("modelSourceIdentifier") {
        input.model_source_equals = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_model_copy_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListModelCopyJobsRequest, String> {
    let mut input = ListModelCopyJobsRequest::default();
    if let Some(value) = query.get("creationTimeAfter") {
        input.creation_time_after = Some(value.to_string());
    }
    if let Some(value) = query.get("creationTimeBefore") {
        input.creation_time_before = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("sortBy") {
        input.sort_by = Some(value.to_string());
    }
    if let Some(value) = query.get("sortOrder") {
        input.sort_order = Some(value.to_string());
    }
    if let Some(value) = query.get("sourceAccountEquals") {
        input.source_account_equals = Some(value.to_string());
    }
    if let Some(value) = query.get("sourceModelArnEquals") {
        input.source_model_arn_equals = Some(value.to_string());
    }
    if let Some(value) = query.get("statusEquals") {
        input.status_equals = Some(value.to_string());
    }
    if let Some(value) = query.get("outputModelNameContains") {
        input.target_model_name_contains = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_model_customization_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListModelCustomizationJobsRequest, String> {
    let mut input = ListModelCustomizationJobsRequest::default();
    if let Some(value) = query.get("creationTimeAfter") {
        input.creation_time_after = Some(value.to_string());
    }
    if let Some(value) = query.get("creationTimeBefore") {
        input.creation_time_before = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nameContains") {
        input.name_contains = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("sortBy") {
        input.sort_by = Some(value.to_string());
    }
    if let Some(value) = query.get("sortOrder") {
        input.sort_order = Some(value.to_string());
    }
    if let Some(value) = query.get("statusEquals") {
        input.status_equals = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_model_import_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListModelImportJobsRequest, String> {
    let mut input = ListModelImportJobsRequest::default();
    if let Some(value) = query.get("creationTimeAfter") {
        input.creation_time_after = Some(value.to_string());
    }
    if let Some(value) = query.get("creationTimeBefore") {
        input.creation_time_before = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nameContains") {
        input.name_contains = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("sortBy") {
        input.sort_by = Some(value.to_string());
    }
    if let Some(value) = query.get("sortOrder") {
        input.sort_order = Some(value.to_string());
    }
    if let Some(value) = query.get("statusEquals") {
        input.status_equals = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_model_invocation_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListModelInvocationJobsRequest, String> {
    let mut input = ListModelInvocationJobsRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nameContains") {
        input.name_contains = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("sortBy") {
        input.sort_by = Some(value.to_string());
    }
    if let Some(value) = query.get("sortOrder") {
        input.sort_order = Some(value.to_string());
    }
    if let Some(value) = query.get("statusEquals") {
        input.status_equals = Some(value.to_string());
    }
    if let Some(value) = query.get("submitTimeAfter") {
        input.submit_time_after = Some(value.to_string());
    }
    if let Some(value) = query.get("submitTimeBefore") {
        input.submit_time_before = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_prompt_routers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPromptRoutersRequest, String> {
    let mut input = ListPromptRoutersRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("type") {
        input.r#type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_provisioned_model_throughputs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListProvisionedModelThroughputsRequest, String> {
    let mut input = ListProvisionedModelThroughputsRequest::default();
    if let Some(value) = query.get("creationTimeAfter") {
        input.creation_time_after = Some(value.to_string());
    }
    if let Some(value) = query.get("creationTimeBefore") {
        input.creation_time_before = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("modelArnEquals") {
        input.model_arn_equals = Some(value.to_string());
    }
    if let Some(value) = query.get("nameContains") {
        input.name_contains = Some(value.to_string());
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("sortBy") {
        input.sort_by = Some(value.to_string());
    }
    if let Some(value) = query.get("sortOrder") {
        input.sort_order = Some(value.to_string());
    }
    if let Some(value) = query.get("statusEquals") {
        input.status_equals = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForResourceRequest, String> {
    let mut input = ListTagsForResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListTagsForResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListTagsForResource request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_enforced_guardrail_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutEnforcedGuardrailConfigurationRequest, String> {
    let mut input = PutEnforcedGuardrailConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutEnforcedGuardrailConfigurationRequest>(&request.body)
            .map_err(|err| {
            format!("failed to deserialize PutEnforcedGuardrailConfiguration request: {err}")
        })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_model_invocation_logging_configuration_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutModelInvocationLoggingConfigurationRequest, String> {
    let mut input = PutModelInvocationLoggingConfigurationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutModelInvocationLoggingConfigurationRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!("failed to deserialize PutModelInvocationLoggingConfiguration request: {err}")
        })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_resource_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutResourcePolicyRequest, String> {
    let mut input = PutResourcePolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutResourcePolicyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutResourcePolicy request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_use_case_for_model_access_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutUseCaseForModelAccessRequest, String> {
    let mut input = PutUseCaseForModelAccessRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutUseCaseForModelAccessRequest>(&request.body).map_err(
            |err| format!("failed to deserialize PutUseCaseForModelAccess request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_register_marketplace_model_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<RegisterMarketplaceModelEndpointRequest, String> {
    let mut input = RegisterMarketplaceModelEndpointRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<RegisterMarketplaceModelEndpointRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize RegisterMarketplaceModelEndpoint request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "endpointIdentifier" => {
                input.endpoint_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_automated_reasoning_policy_build_workflow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartAutomatedReasoningPolicyBuildWorkflowRequest, String> {
    let mut input = StartAutomatedReasoningPolicyBuildWorkflowRequest::default();
    if !request.body.is_empty() {
        let payload = serde_json::from_slice::<AutomatedReasoningPolicyBuildWorkflowSource>(
            &request.body,
        )
        .map_err(|err| {
            format!(
                "failed to deserialize StartAutomatedReasoningPolicyBuildWorkflow payload: {err}"
            )
        })?;
        input.source_content = payload;
    }
    for (name, value) in labels {
        match *name {
            "buildWorkflowType" => {
                input.build_workflow_type = value.to_string();
            }
            "policyArn" => {
                input.policy_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = request
        .headers
        .get("x-amz-client-token")
        .and_then(|value| value.to_str().ok())
    {
        input.client_request_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_automated_reasoning_policy_test_workflow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartAutomatedReasoningPolicyTestWorkflowRequest, String> {
    let mut input = StartAutomatedReasoningPolicyTestWorkflowRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartAutomatedReasoningPolicyTestWorkflowRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!(
                "failed to deserialize StartAutomatedReasoningPolicyTestWorkflow request: {err}"
            )
        })?;
    }
    for (name, value) in labels {
        match *name {
            "buildWorkflowId" => {
                input.build_workflow_id = value.to_string();
            }
            "policyArn" => {
                input.policy_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_evaluation_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopEvaluationJobRequest, String> {
    let mut input = StopEvaluationJobRequest::default();
    for (name, value) in labels {
        match *name {
            "jobIdentifier" => {
                input.job_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_model_customization_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopModelCustomizationJobRequest, String> {
    let mut input = StopModelCustomizationJobRequest::default();
    for (name, value) in labels {
        match *name {
            "jobIdentifier" => {
                input.job_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_model_invocation_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopModelInvocationJobRequest, String> {
    let mut input = StopModelInvocationJobRequest::default();
    for (name, value) in labels {
        match *name {
            "jobIdentifier" => {
                input.job_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_tag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TagResourceRequest, String> {
    let mut input = TagResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TagResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize TagResource request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_untag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UntagResourceRequest, String> {
    let mut input = UntagResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UntagResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UntagResource request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_automated_reasoning_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAutomatedReasoningPolicyRequest, String> {
    let mut input = UpdateAutomatedReasoningPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAutomatedReasoningPolicyRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateAutomatedReasoningPolicy request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "policyArn" => {
                input.policy_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_automated_reasoning_policy_annotations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAutomatedReasoningPolicyAnnotationsRequest, String> {
    let mut input = UpdateAutomatedReasoningPolicyAnnotationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAutomatedReasoningPolicyAnnotationsRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!(
                "failed to deserialize UpdateAutomatedReasoningPolicyAnnotations request: {err}"
            )
        })?;
    }
    for (name, value) in labels {
        match *name {
            "buildWorkflowId" => {
                input.build_workflow_id = value.to_string();
            }
            "policyArn" => {
                input.policy_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_automated_reasoning_policy_test_case_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAutomatedReasoningPolicyTestCaseRequest, String> {
    let mut input = UpdateAutomatedReasoningPolicyTestCaseRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAutomatedReasoningPolicyTestCaseRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!("failed to deserialize UpdateAutomatedReasoningPolicyTestCase request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "policyArn" => {
                input.policy_arn = value.to_string();
            }
            "testCaseId" => {
                input.test_case_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_custom_model_deployment_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCustomModelDeploymentRequest, String> {
    let mut input = UpdateCustomModelDeploymentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateCustomModelDeploymentRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateCustomModelDeployment request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "customModelDeploymentIdentifier" => {
                input.custom_model_deployment_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_guardrail_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateGuardrailRequest, String> {
    let mut input = UpdateGuardrailRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateGuardrailRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateGuardrail request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "guardrailIdentifier" => {
                input.guardrail_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_marketplace_model_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateMarketplaceModelEndpointRequest, String> {
    let mut input = UpdateMarketplaceModelEndpointRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateMarketplaceModelEndpointRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateMarketplaceModelEndpoint request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "endpointArn" => {
                input.endpoint_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_provisioned_model_throughput_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateProvisionedModelThroughputRequest, String> {
    let mut input = UpdateProvisionedModelThroughputRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateProvisionedModelThroughputRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateProvisionedModelThroughput request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "provisionedModelId" => {
                input.provisioned_model_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
