//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-servicecatalog

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_accept_portfolio_share_response(
    result: &AcceptPortfolioShareOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_budget_with_resource_response(
    result: &AssociateBudgetWithResourceOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_principal_with_portfolio_response(
    result: &AssociatePrincipalWithPortfolioOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_product_with_portfolio_response(
    result: &AssociateProductWithPortfolioOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_service_action_with_provisioning_artifact_response(
    result: &AssociateServiceActionWithProvisioningArtifactOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_associate_tag_option_with_resource_response(
    result: &AssociateTagOptionWithResourceOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_associate_service_action_with_provisioning_artifact_response(
    result: &BatchAssociateServiceActionWithProvisioningArtifactOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_batch_disassociate_service_action_from_provisioning_artifact_response(
    result: &BatchDisassociateServiceActionFromProvisioningArtifactOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_copy_product_response(result: &CopyProductOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_constraint_response(result: &CreateConstraintOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_portfolio_response(result: &CreatePortfolioOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_portfolio_share_response(
    result: &CreatePortfolioShareOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_product_response(result: &CreateProductOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_provisioned_product_plan_response(
    result: &CreateProvisionedProductPlanOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_provisioning_artifact_response(
    result: &CreateProvisioningArtifactOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_service_action_response(
    result: &CreateServiceActionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_tag_option_response(result: &CreateTagOptionOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_constraint_response(result: &DeleteConstraintOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_portfolio_response(result: &DeletePortfolioOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_portfolio_share_response(
    result: &DeletePortfolioShareOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_product_response(result: &DeleteProductOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_provisioned_product_plan_response(
    result: &DeleteProvisionedProductPlanOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_provisioning_artifact_response(
    result: &DeleteProvisioningArtifactOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_service_action_response(
    result: &DeleteServiceActionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_tag_option_response(result: &DeleteTagOptionOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_constraint_response(result: &DescribeConstraintOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_copy_product_status_response(
    result: &DescribeCopyProductStatusOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_portfolio_response(result: &DescribePortfolioOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_portfolio_share_status_response(
    result: &DescribePortfolioShareStatusOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_portfolio_shares_response(
    result: &DescribePortfolioSharesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_product_response(result: &DescribeProductOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_product_as_admin_response(
    result: &DescribeProductAsAdminOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_product_view_response(
    result: &DescribeProductViewOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_provisioned_product_response(
    result: &DescribeProvisionedProductOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_provisioned_product_plan_response(
    result: &DescribeProvisionedProductPlanOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_provisioning_artifact_response(
    result: &DescribeProvisioningArtifactOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_provisioning_parameters_response(
    result: &DescribeProvisioningParametersOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_record_response(result: &DescribeRecordOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_service_action_response(
    result: &DescribeServiceActionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_service_action_execution_parameters_response(
    result: &DescribeServiceActionExecutionParametersOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_tag_option_response(result: &DescribeTagOptionOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disable_a_w_s_organizations_access_response(
    result: &DisableAWSOrganizationsAccessOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_budget_from_resource_response(
    result: &DisassociateBudgetFromResourceOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_principal_from_portfolio_response(
    result: &DisassociatePrincipalFromPortfolioOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_product_from_portfolio_response(
    result: &DisassociateProductFromPortfolioOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_service_action_from_provisioning_artifact_response(
    result: &DisassociateServiceActionFromProvisioningArtifactOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_tag_option_from_resource_response(
    result: &DisassociateTagOptionFromResourceOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_enable_a_w_s_organizations_access_response(
    result: &EnableAWSOrganizationsAccessOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_execute_provisioned_product_plan_response(
    result: &ExecuteProvisionedProductPlanOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_execute_provisioned_product_service_action_response(
    result: &ExecuteProvisionedProductServiceActionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_a_w_s_organizations_access_status_response(
    result: &GetAWSOrganizationsAccessStatusOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_provisioned_product_outputs_response(
    result: &GetProvisionedProductOutputsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_import_as_provisioned_product_response(
    result: &ImportAsProvisionedProductOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_accepted_portfolio_shares_response(
    result: &ListAcceptedPortfolioSharesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_budgets_for_resource_response(
    result: &ListBudgetsForResourceOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_constraints_for_portfolio_response(
    result: &ListConstraintsForPortfolioOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_launch_paths_response(result: &ListLaunchPathsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_organization_portfolio_access_response(
    result: &ListOrganizationPortfolioAccessOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_portfolio_access_response(
    result: &ListPortfolioAccessOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_portfolios_response(result: &ListPortfoliosOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_portfolios_for_product_response(
    result: &ListPortfoliosForProductOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_principals_for_portfolio_response(
    result: &ListPrincipalsForPortfolioOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_provisioned_product_plans_response(
    result: &ListProvisionedProductPlansOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_provisioning_artifacts_response(
    result: &ListProvisioningArtifactsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_provisioning_artifacts_for_service_action_response(
    result: &ListProvisioningArtifactsForServiceActionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_record_history_response(result: &ListRecordHistoryOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_resources_for_tag_option_response(
    result: &ListResourcesForTagOptionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_service_actions_response(result: &ListServiceActionsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_service_actions_for_provisioning_artifact_response(
    result: &ListServiceActionsForProvisioningArtifactOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_stack_instances_for_provisioned_product_response(
    result: &ListStackInstancesForProvisionedProductOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tag_options_response(result: &ListTagOptionsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_notify_provision_product_engine_workflow_result_response(
    result: &NotifyProvisionProductEngineWorkflowResultOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_notify_terminate_provisioned_product_engine_workflow_result_response(
    result: &NotifyTerminateProvisionedProductEngineWorkflowResultOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_notify_update_provisioned_product_engine_workflow_result_response(
    result: &NotifyUpdateProvisionedProductEngineWorkflowResultOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_provision_product_response(result: &ProvisionProductOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_reject_portfolio_share_response(
    result: &RejectPortfolioShareOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_scan_provisioned_products_response(
    result: &ScanProvisionedProductsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_search_products_response(result: &SearchProductsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_search_products_as_admin_response(
    result: &SearchProductsAsAdminOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_search_provisioned_products_response(
    result: &SearchProvisionedProductsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_terminate_provisioned_product_response(
    result: &TerminateProvisionedProductOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_constraint_response(result: &UpdateConstraintOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_portfolio_response(result: &UpdatePortfolioOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_portfolio_share_response(
    result: &UpdatePortfolioShareOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_product_response(result: &UpdateProductOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_provisioned_product_response(
    result: &UpdateProvisionedProductOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_provisioned_product_properties_response(
    result: &UpdateProvisionedProductPropertiesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_provisioning_artifact_response(
    result: &UpdateProvisioningArtifactOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_service_action_response(
    result: &UpdateServiceActionOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_tag_option_response(result: &UpdateTagOptionOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_accept_portfolio_share_request(
    body: &[u8],
) -> Result<AcceptPortfolioShareInput, String> {
    if body.is_empty() {
        return Ok(AcceptPortfolioShareInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AcceptPortfolioShare request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_budget_with_resource_request(
    body: &[u8],
) -> Result<AssociateBudgetWithResourceInput, String> {
    if body.is_empty() {
        return Ok(AssociateBudgetWithResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateBudgetWithResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_principal_with_portfolio_request(
    body: &[u8],
) -> Result<AssociatePrincipalWithPortfolioInput, String> {
    if body.is_empty() {
        return Ok(AssociatePrincipalWithPortfolioInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociatePrincipalWithPortfolio request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_product_with_portfolio_request(
    body: &[u8],
) -> Result<AssociateProductWithPortfolioInput, String> {
    if body.is_empty() {
        return Ok(AssociateProductWithPortfolioInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateProductWithPortfolio request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_service_action_with_provisioning_artifact_request(
    body: &[u8],
) -> Result<AssociateServiceActionWithProvisioningArtifactInput, String> {
    if body.is_empty() {
        return Ok(AssociateServiceActionWithProvisioningArtifactInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize AssociateServiceActionWithProvisioningArtifact request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_tag_option_with_resource_request(
    body: &[u8],
) -> Result<AssociateTagOptionWithResourceInput, String> {
    if body.is_empty() {
        return Ok(AssociateTagOptionWithResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateTagOptionWithResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_associate_service_action_with_provisioning_artifact_request(
    body: &[u8],
) -> Result<BatchAssociateServiceActionWithProvisioningArtifactInput, String> {
    if body.is_empty() {
        return Ok(BatchAssociateServiceActionWithProvisioningArtifactInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!(
            "Failed to deserialize BatchAssociateServiceActionWithProvisioningArtifact request: {e}"
        )
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_batch_disassociate_service_action_from_provisioning_artifact_request(
    body: &[u8],
) -> Result<BatchDisassociateServiceActionFromProvisioningArtifactInput, String> {
    if body.is_empty() {
        return Ok(BatchDisassociateServiceActionFromProvisioningArtifactInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize BatchDisassociateServiceActionFromProvisioningArtifact request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_copy_product_request(body: &[u8]) -> Result<CopyProductInput, String> {
    if body.is_empty() {
        return Ok(CopyProductInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CopyProduct request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_constraint_request(body: &[u8]) -> Result<CreateConstraintInput, String> {
    if body.is_empty() {
        return Ok(CreateConstraintInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateConstraint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_portfolio_request(body: &[u8]) -> Result<CreatePortfolioInput, String> {
    if body.is_empty() {
        return Ok(CreatePortfolioInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreatePortfolio request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_portfolio_share_request(
    body: &[u8],
) -> Result<CreatePortfolioShareInput, String> {
    if body.is_empty() {
        return Ok(CreatePortfolioShareInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreatePortfolioShare request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_product_request(body: &[u8]) -> Result<CreateProductInput, String> {
    if body.is_empty() {
        return Ok(CreateProductInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateProduct request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_provisioned_product_plan_request(
    body: &[u8],
) -> Result<CreateProvisionedProductPlanInput, String> {
    if body.is_empty() {
        return Ok(CreateProvisionedProductPlanInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateProvisionedProductPlan request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_provisioning_artifact_request(
    body: &[u8],
) -> Result<CreateProvisioningArtifactInput, String> {
    if body.is_empty() {
        return Ok(CreateProvisioningArtifactInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateProvisioningArtifact request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_service_action_request(
    body: &[u8],
) -> Result<CreateServiceActionInput, String> {
    if body.is_empty() {
        return Ok(CreateServiceActionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateServiceAction request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_tag_option_request(body: &[u8]) -> Result<CreateTagOptionInput, String> {
    if body.is_empty() {
        return Ok(CreateTagOptionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateTagOption request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_constraint_request(body: &[u8]) -> Result<DeleteConstraintInput, String> {
    if body.is_empty() {
        return Ok(DeleteConstraintInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteConstraint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_portfolio_request(body: &[u8]) -> Result<DeletePortfolioInput, String> {
    if body.is_empty() {
        return Ok(DeletePortfolioInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeletePortfolio request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_portfolio_share_request(
    body: &[u8],
) -> Result<DeletePortfolioShareInput, String> {
    if body.is_empty() {
        return Ok(DeletePortfolioShareInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeletePortfolioShare request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_product_request(body: &[u8]) -> Result<DeleteProductInput, String> {
    if body.is_empty() {
        return Ok(DeleteProductInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteProduct request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_provisioned_product_plan_request(
    body: &[u8],
) -> Result<DeleteProvisionedProductPlanInput, String> {
    if body.is_empty() {
        return Ok(DeleteProvisionedProductPlanInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteProvisionedProductPlan request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_provisioning_artifact_request(
    body: &[u8],
) -> Result<DeleteProvisioningArtifactInput, String> {
    if body.is_empty() {
        return Ok(DeleteProvisioningArtifactInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteProvisioningArtifact request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_service_action_request(
    body: &[u8],
) -> Result<DeleteServiceActionInput, String> {
    if body.is_empty() {
        return Ok(DeleteServiceActionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteServiceAction request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_tag_option_request(body: &[u8]) -> Result<DeleteTagOptionInput, String> {
    if body.is_empty() {
        return Ok(DeleteTagOptionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteTagOption request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_constraint_request(
    body: &[u8],
) -> Result<DescribeConstraintInput, String> {
    if body.is_empty() {
        return Ok(DescribeConstraintInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeConstraint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_copy_product_status_request(
    body: &[u8],
) -> Result<DescribeCopyProductStatusInput, String> {
    if body.is_empty() {
        return Ok(DescribeCopyProductStatusInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeCopyProductStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_portfolio_request(
    body: &[u8],
) -> Result<DescribePortfolioInput, String> {
    if body.is_empty() {
        return Ok(DescribePortfolioInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribePortfolio request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_portfolio_share_status_request(
    body: &[u8],
) -> Result<DescribePortfolioShareStatusInput, String> {
    if body.is_empty() {
        return Ok(DescribePortfolioShareStatusInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribePortfolioShareStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_portfolio_shares_request(
    body: &[u8],
) -> Result<DescribePortfolioSharesInput, String> {
    if body.is_empty() {
        return Ok(DescribePortfolioSharesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribePortfolioShares request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_product_request(body: &[u8]) -> Result<DescribeProductInput, String> {
    if body.is_empty() {
        return Ok(DescribeProductInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeProduct request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_product_as_admin_request(
    body: &[u8],
) -> Result<DescribeProductAsAdminInput, String> {
    if body.is_empty() {
        return Ok(DescribeProductAsAdminInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeProductAsAdmin request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_product_view_request(
    body: &[u8],
) -> Result<DescribeProductViewInput, String> {
    if body.is_empty() {
        return Ok(DescribeProductViewInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeProductView request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_provisioned_product_request(
    body: &[u8],
) -> Result<DescribeProvisionedProductInput, String> {
    if body.is_empty() {
        return Ok(DescribeProvisionedProductInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeProvisionedProduct request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_provisioned_product_plan_request(
    body: &[u8],
) -> Result<DescribeProvisionedProductPlanInput, String> {
    if body.is_empty() {
        return Ok(DescribeProvisionedProductPlanInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeProvisionedProductPlan request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_provisioning_artifact_request(
    body: &[u8],
) -> Result<DescribeProvisioningArtifactInput, String> {
    if body.is_empty() {
        return Ok(DescribeProvisioningArtifactInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeProvisioningArtifact request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_provisioning_parameters_request(
    body: &[u8],
) -> Result<DescribeProvisioningParametersInput, String> {
    if body.is_empty() {
        return Ok(DescribeProvisioningParametersInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeProvisioningParameters request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_record_request(body: &[u8]) -> Result<DescribeRecordInput, String> {
    if body.is_empty() {
        return Ok(DescribeRecordInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeRecord request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_service_action_request(
    body: &[u8],
) -> Result<DescribeServiceActionInput, String> {
    if body.is_empty() {
        return Ok(DescribeServiceActionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeServiceAction request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_service_action_execution_parameters_request(
    body: &[u8],
) -> Result<DescribeServiceActionExecutionParametersInput, String> {
    if body.is_empty() {
        return Ok(DescribeServiceActionExecutionParametersInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeServiceActionExecutionParameters request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_tag_option_request(
    body: &[u8],
) -> Result<DescribeTagOptionInput, String> {
    if body.is_empty() {
        return Ok(DescribeTagOptionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeTagOption request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disable_a_w_s_organizations_access_request(
    body: &[u8],
) -> Result<DisableAWSOrganizationsAccessInput, String> {
    if body.is_empty() {
        return Ok(DisableAWSOrganizationsAccessInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisableAWSOrganizationsAccess request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_budget_from_resource_request(
    body: &[u8],
) -> Result<DisassociateBudgetFromResourceInput, String> {
    if body.is_empty() {
        return Ok(DisassociateBudgetFromResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisassociateBudgetFromResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_principal_from_portfolio_request(
    body: &[u8],
) -> Result<DisassociatePrincipalFromPortfolioInput, String> {
    if body.is_empty() {
        return Ok(DisassociatePrincipalFromPortfolioInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DisassociatePrincipalFromPortfolio request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_product_from_portfolio_request(
    body: &[u8],
) -> Result<DisassociateProductFromPortfolioInput, String> {
    if body.is_empty() {
        return Ok(DisassociateProductFromPortfolioInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisassociateProductFromPortfolio request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_service_action_from_provisioning_artifact_request(
    body: &[u8],
) -> Result<DisassociateServiceActionFromProvisioningArtifactInput, String> {
    if body.is_empty() {
        return Ok(DisassociateServiceActionFromProvisioningArtifactInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!(
            "Failed to deserialize DisassociateServiceActionFromProvisioningArtifact request: {e}"
        )
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_tag_option_from_resource_request(
    body: &[u8],
) -> Result<DisassociateTagOptionFromResourceInput, String> {
    if body.is_empty() {
        return Ok(DisassociateTagOptionFromResourceInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DisassociateTagOptionFromResource request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_enable_a_w_s_organizations_access_request(
    body: &[u8],
) -> Result<EnableAWSOrganizationsAccessInput, String> {
    if body.is_empty() {
        return Ok(EnableAWSOrganizationsAccessInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize EnableAWSOrganizationsAccess request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_execute_provisioned_product_plan_request(
    body: &[u8],
) -> Result<ExecuteProvisionedProductPlanInput, String> {
    if body.is_empty() {
        return Ok(ExecuteProvisionedProductPlanInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ExecuteProvisionedProductPlan request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_execute_provisioned_product_service_action_request(
    body: &[u8],
) -> Result<ExecuteProvisionedProductServiceActionInput, String> {
    if body.is_empty() {
        return Ok(ExecuteProvisionedProductServiceActionInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ExecuteProvisionedProductServiceAction request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_a_w_s_organizations_access_status_request(
    body: &[u8],
) -> Result<GetAWSOrganizationsAccessStatusInput, String> {
    if body.is_empty() {
        return Ok(GetAWSOrganizationsAccessStatusInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetAWSOrganizationsAccessStatus request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_provisioned_product_outputs_request(
    body: &[u8],
) -> Result<GetProvisionedProductOutputsInput, String> {
    if body.is_empty() {
        return Ok(GetProvisionedProductOutputsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetProvisionedProductOutputs request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_import_as_provisioned_product_request(
    body: &[u8],
) -> Result<ImportAsProvisionedProductInput, String> {
    if body.is_empty() {
        return Ok(ImportAsProvisionedProductInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ImportAsProvisionedProduct request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_accepted_portfolio_shares_request(
    body: &[u8],
) -> Result<ListAcceptedPortfolioSharesInput, String> {
    if body.is_empty() {
        return Ok(ListAcceptedPortfolioSharesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListAcceptedPortfolioShares request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_budgets_for_resource_request(
    body: &[u8],
) -> Result<ListBudgetsForResourceInput, String> {
    if body.is_empty() {
        return Ok(ListBudgetsForResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListBudgetsForResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_constraints_for_portfolio_request(
    body: &[u8],
) -> Result<ListConstraintsForPortfolioInput, String> {
    if body.is_empty() {
        return Ok(ListConstraintsForPortfolioInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListConstraintsForPortfolio request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_launch_paths_request(body: &[u8]) -> Result<ListLaunchPathsInput, String> {
    if body.is_empty() {
        return Ok(ListLaunchPathsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListLaunchPaths request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_organization_portfolio_access_request(
    body: &[u8],
) -> Result<ListOrganizationPortfolioAccessInput, String> {
    if body.is_empty() {
        return Ok(ListOrganizationPortfolioAccessInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListOrganizationPortfolioAccess request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_portfolio_access_request(
    body: &[u8],
) -> Result<ListPortfolioAccessInput, String> {
    if body.is_empty() {
        return Ok(ListPortfolioAccessInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListPortfolioAccess request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_portfolios_request(body: &[u8]) -> Result<ListPortfoliosInput, String> {
    if body.is_empty() {
        return Ok(ListPortfoliosInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListPortfolios request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_portfolios_for_product_request(
    body: &[u8],
) -> Result<ListPortfoliosForProductInput, String> {
    if body.is_empty() {
        return Ok(ListPortfoliosForProductInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListPortfoliosForProduct request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_principals_for_portfolio_request(
    body: &[u8],
) -> Result<ListPrincipalsForPortfolioInput, String> {
    if body.is_empty() {
        return Ok(ListPrincipalsForPortfolioInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListPrincipalsForPortfolio request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_provisioned_product_plans_request(
    body: &[u8],
) -> Result<ListProvisionedProductPlansInput, String> {
    if body.is_empty() {
        return Ok(ListProvisionedProductPlansInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListProvisionedProductPlans request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_provisioning_artifacts_request(
    body: &[u8],
) -> Result<ListProvisioningArtifactsInput, String> {
    if body.is_empty() {
        return Ok(ListProvisioningArtifactsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListProvisioningArtifacts request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_provisioning_artifacts_for_service_action_request(
    body: &[u8],
) -> Result<ListProvisioningArtifactsForServiceActionInput, String> {
    if body.is_empty() {
        return Ok(ListProvisioningArtifactsForServiceActionInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListProvisioningArtifactsForServiceAction request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_record_history_request(
    body: &[u8],
) -> Result<ListRecordHistoryInput, String> {
    if body.is_empty() {
        return Ok(ListRecordHistoryInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListRecordHistory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_resources_for_tag_option_request(
    body: &[u8],
) -> Result<ListResourcesForTagOptionInput, String> {
    if body.is_empty() {
        return Ok(ListResourcesForTagOptionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListResourcesForTagOption request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_service_actions_request(
    body: &[u8],
) -> Result<ListServiceActionsInput, String> {
    if body.is_empty() {
        return Ok(ListServiceActionsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListServiceActions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_service_actions_for_provisioning_artifact_request(
    body: &[u8],
) -> Result<ListServiceActionsForProvisioningArtifactInput, String> {
    if body.is_empty() {
        return Ok(ListServiceActionsForProvisioningArtifactInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListServiceActionsForProvisioningArtifact request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_stack_instances_for_provisioned_product_request(
    body: &[u8],
) -> Result<ListStackInstancesForProvisionedProductInput, String> {
    if body.is_empty() {
        return Ok(ListStackInstancesForProvisionedProductInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize ListStackInstancesForProvisionedProduct request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tag_options_request(body: &[u8]) -> Result<ListTagOptionsInput, String> {
    if body.is_empty() {
        return Ok(ListTagOptionsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTagOptions request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_notify_provision_product_engine_workflow_result_request(
    body: &[u8],
) -> Result<NotifyProvisionProductEngineWorkflowResultInput, String> {
    if body.is_empty() {
        return Ok(NotifyProvisionProductEngineWorkflowResultInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize NotifyProvisionProductEngineWorkflowResult request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_notify_terminate_provisioned_product_engine_workflow_result_request(
    body: &[u8],
) -> Result<NotifyTerminateProvisionedProductEngineWorkflowResultInput, String> {
    if body.is_empty() {
        return Ok(NotifyTerminateProvisionedProductEngineWorkflowResultInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize NotifyTerminateProvisionedProductEngineWorkflowResult request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_notify_update_provisioned_product_engine_workflow_result_request(
    body: &[u8],
) -> Result<NotifyUpdateProvisionedProductEngineWorkflowResultInput, String> {
    if body.is_empty() {
        return Ok(NotifyUpdateProvisionedProductEngineWorkflowResultInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!(
            "Failed to deserialize NotifyUpdateProvisionedProductEngineWorkflowResult request: {e}"
        )
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_provision_product_request(body: &[u8]) -> Result<ProvisionProductInput, String> {
    if body.is_empty() {
        return Ok(ProvisionProductInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ProvisionProduct request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_reject_portfolio_share_request(
    body: &[u8],
) -> Result<RejectPortfolioShareInput, String> {
    if body.is_empty() {
        return Ok(RejectPortfolioShareInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize RejectPortfolioShare request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_scan_provisioned_products_request(
    body: &[u8],
) -> Result<ScanProvisionedProductsInput, String> {
    if body.is_empty() {
        return Ok(ScanProvisionedProductsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ScanProvisionedProducts request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_search_products_request(body: &[u8]) -> Result<SearchProductsInput, String> {
    if body.is_empty() {
        return Ok(SearchProductsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SearchProducts request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_search_products_as_admin_request(
    body: &[u8],
) -> Result<SearchProductsAsAdminInput, String> {
    if body.is_empty() {
        return Ok(SearchProductsAsAdminInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SearchProductsAsAdmin request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_search_provisioned_products_request(
    body: &[u8],
) -> Result<SearchProvisionedProductsInput, String> {
    if body.is_empty() {
        return Ok(SearchProvisionedProductsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SearchProvisionedProducts request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_terminate_provisioned_product_request(
    body: &[u8],
) -> Result<TerminateProvisionedProductInput, String> {
    if body.is_empty() {
        return Ok(TerminateProvisionedProductInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TerminateProvisionedProduct request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_constraint_request(body: &[u8]) -> Result<UpdateConstraintInput, String> {
    if body.is_empty() {
        return Ok(UpdateConstraintInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateConstraint request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_portfolio_request(body: &[u8]) -> Result<UpdatePortfolioInput, String> {
    if body.is_empty() {
        return Ok(UpdatePortfolioInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdatePortfolio request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_portfolio_share_request(
    body: &[u8],
) -> Result<UpdatePortfolioShareInput, String> {
    if body.is_empty() {
        return Ok(UpdatePortfolioShareInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdatePortfolioShare request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_product_request(body: &[u8]) -> Result<UpdateProductInput, String> {
    if body.is_empty() {
        return Ok(UpdateProductInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateProduct request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_provisioned_product_request(
    body: &[u8],
) -> Result<UpdateProvisionedProductInput, String> {
    if body.is_empty() {
        return Ok(UpdateProvisionedProductInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateProvisionedProduct request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_provisioned_product_properties_request(
    body: &[u8],
) -> Result<UpdateProvisionedProductPropertiesInput, String> {
    if body.is_empty() {
        return Ok(UpdateProvisionedProductPropertiesInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize UpdateProvisionedProductProperties request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_provisioning_artifact_request(
    body: &[u8],
) -> Result<UpdateProvisioningArtifactInput, String> {
    if body.is_empty() {
        return Ok(UpdateProvisioningArtifactInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateProvisioningArtifact request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_service_action_request(
    body: &[u8],
) -> Result<UpdateServiceActionInput, String> {
    if body.is_empty() {
        return Ok(UpdateServiceActionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateServiceAction request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_tag_option_request(body: &[u8]) -> Result<UpdateTagOptionInput, String> {
    if body.is_empty() {
        return Ok(UpdateTagOptionInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateTagOption request: {e}"))
}
