//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-servicecatalog

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptPortfolioShareInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "PortfolioId")]
    #[serde(default)]
    pub portfolio_id: String,
    #[serde(rename = "PortfolioShareType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_share_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptPortfolioShareOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateBudgetWithResourceInput {
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    pub budget_name: String,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateBudgetWithResourceOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociatePrincipalWithPortfolioInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "PortfolioId")]
    #[serde(default)]
    pub portfolio_id: String,
    #[serde(rename = "PrincipalARN")]
    #[serde(default)]
    pub principal_a_r_n: String,
    #[serde(rename = "PrincipalType")]
    #[serde(default)]
    pub principal_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociatePrincipalWithPortfolioOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateProductWithPortfolioInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "PortfolioId")]
    #[serde(default)]
    pub portfolio_id: String,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    pub product_id: String,
    #[serde(rename = "SourcePortfolioId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_portfolio_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateProductWithPortfolioOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateServiceActionWithProvisioningArtifactInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    pub product_id: String,
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(default)]
    pub provisioning_artifact_id: String,
    #[serde(rename = "ServiceActionId")]
    #[serde(default)]
    pub service_action_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateServiceActionWithProvisioningArtifactOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateTagOptionWithResourceInput {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "TagOptionId")]
    #[serde(default)]
    pub tag_option_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateTagOptionWithResourceOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchAssociateServiceActionWithProvisioningArtifactInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "ServiceActionAssociations")]
    #[serde(default)]
    pub service_action_associations: Vec<ServiceActionAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceActionAssociation {
    #[serde(rename = "ProductId")]
    #[serde(default)]
    pub product_id: String,
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(default)]
    pub provisioning_artifact_id: String,
    #[serde(rename = "ServiceActionId")]
    #[serde(default)]
    pub service_action_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchAssociateServiceActionWithProvisioningArtifactOutput {
    #[serde(rename = "FailedServiceActionAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_service_action_associations: Option<Vec<FailedServiceActionAssociation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailedServiceActionAssociation {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    #[serde(rename = "ServiceActionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_action_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDisassociateServiceActionFromProvisioningArtifactInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "ServiceActionAssociations")]
    #[serde(default)]
    pub service_action_associations: Vec<ServiceActionAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDisassociateServiceActionFromProvisioningArtifactOutput {
    #[serde(rename = "FailedServiceActionAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_service_action_associations: Option<Vec<FailedServiceActionAssociation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CopyProductInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "CopyOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_options: Option<Vec<String>>,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    pub idempotency_token: String,
    #[serde(rename = "SourceProductArn")]
    #[serde(default)]
    pub source_product_arn: String,
    #[serde(rename = "SourceProvisioningArtifactIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_provisioning_artifact_identifiers:
        Option<Vec<std::collections::HashMap<String, String>>>,
    #[serde(rename = "TargetProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_product_id: Option<String>,
    #[serde(rename = "TargetProductName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_product_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CopyProductOutput {
    #[serde(rename = "CopyProductToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_product_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConstraintInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    pub idempotency_token: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    pub parameters: String,
    #[serde(rename = "PortfolioId")]
    #[serde(default)]
    pub portfolio_id: String,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    pub product_id: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConstraintOutput {
    #[serde(rename = "ConstraintDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_detail: Option<ConstraintDetail>,
    #[serde(rename = "ConstraintParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_parameters: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConstraintDetail {
    #[serde(rename = "ConstraintId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "PortfolioId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_id: Option<String>,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePortfolioInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    pub display_name: String,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    pub idempotency_token: String,
    #[serde(rename = "ProviderName")]
    #[serde(default)]
    pub provider_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePortfolioOutput {
    #[serde(rename = "PortfolioDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_detail: Option<PortfolioDetail>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PortfolioDetail {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePortfolioShareInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "OrganizationNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_node: Option<OrganizationNode>,
    #[serde(rename = "PortfolioId")]
    #[serde(default)]
    pub portfolio_id: String,
    #[serde(rename = "SharePrincipals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_principals: Option<bool>,
    #[serde(rename = "ShareTagOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_tag_options: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationNode {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePortfolioShareOutput {
    #[serde(rename = "PortfolioShareToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_share_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProductInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Distributor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distributor: Option<String>,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    pub idempotency_token: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Owner")]
    #[serde(default)]
    pub owner: String,
    #[serde(rename = "ProductType")]
    #[serde(default)]
    pub product_type: String,
    #[serde(rename = "ProvisioningArtifactParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_parameters: Option<ProvisioningArtifactProperties>,
    #[serde(rename = "SourceConnection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_connection: Option<SourceConnection>,
    #[serde(rename = "SupportDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_description: Option<String>,
    #[serde(rename = "SupportEmail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    #[serde(rename = "SupportUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisioningArtifactProperties {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DisableTemplateValidation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_template_validation: Option<bool>,
    #[serde(rename = "Info")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceConnection {
    #[serde(rename = "ConnectionParameters")]
    #[serde(default)]
    pub connection_parameters: SourceConnectionParameters,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceConnectionParameters {
    #[serde(rename = "CodeStar")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_star: Option<CodeStarParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeStarParameters {
    #[serde(rename = "ArtifactPath")]
    #[serde(default)]
    pub artifact_path: String,
    #[serde(rename = "Branch")]
    #[serde(default)]
    pub branch: String,
    #[serde(rename = "ConnectionArn")]
    #[serde(default)]
    pub connection_arn: String,
    #[serde(rename = "Repository")]
    #[serde(default)]
    pub repository: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProductOutput {
    #[serde(rename = "ProductViewDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_detail: Option<ProductViewDetail>,
    #[serde(rename = "ProvisioningArtifactDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_detail: Option<ProvisioningArtifactDetail>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProductViewDetail {
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "ProductARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_a_r_n: Option<String>,
    #[serde(rename = "ProductViewSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_summary: Option<ProductViewSummary>,
    #[serde(rename = "SourceConnection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_connection: Option<SourceConnectionDetail>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProductViewSummary {
    #[serde(rename = "Distributor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distributor: Option<String>,
    #[serde(rename = "HasDefaultPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_default_path: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "ShortDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    #[serde(rename = "SupportDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_description: Option<String>,
    #[serde(rename = "SupportEmail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    #[serde(rename = "SupportUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceConnectionDetail {
    #[serde(rename = "ConnectionParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_parameters: Option<SourceConnectionParameters>,
    #[serde(rename = "LastSync")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync: Option<LastSync>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LastSync {
    #[serde(rename = "LastSuccessfulSyncProvisioningArtifactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_sync_provisioning_artifact_id: Option<String>,
    #[serde(rename = "LastSuccessfulSyncTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_sync_time: Option<f64>,
    #[serde(rename = "LastSyncStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync_status: Option<String>,
    #[serde(rename = "LastSyncStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync_status_message: Option<String>,
    #[serde(rename = "LastSyncTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisioningArtifactDetail {
    #[serde(rename = "Active")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Guidance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SourceRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_revision: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProvisionedProductPlanInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    pub idempotency_token: String,
    #[serde(rename = "NotificationArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_arns: Option<Vec<String>>,
    #[serde(rename = "PathId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_id: Option<String>,
    #[serde(rename = "PlanName")]
    #[serde(default)]
    pub plan_name: String,
    #[serde(rename = "PlanType")]
    #[serde(default)]
    pub plan_type: String,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    pub product_id: String,
    #[serde(rename = "ProvisionedProductName")]
    #[serde(default)]
    pub provisioned_product_name: String,
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(default)]
    pub provisioning_artifact_id: String,
    #[serde(rename = "ProvisioningParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_parameters: Option<Vec<UpdateProvisioningParameter>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProvisioningParameter {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "UsePreviousValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_previous_value: Option<bool>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProvisionedProductPlanOutput {
    #[serde(rename = "PlanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(rename = "PlanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(rename = "ProvisionProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provision_product_id: Option<String>,
    #[serde(rename = "ProvisionedProductName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_name: Option<String>,
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProvisioningArtifactInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    pub idempotency_token: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    pub parameters: ProvisioningArtifactProperties,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    pub product_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProvisioningArtifactOutput {
    #[serde(rename = "Info")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ProvisioningArtifactDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_detail: Option<ProvisioningArtifactDetail>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateServiceActionInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "Definition")]
    #[serde(default)]
    pub definition: std::collections::HashMap<String, String>,
    #[serde(rename = "DefinitionType")]
    #[serde(default)]
    pub definition_type: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    pub idempotency_token: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateServiceActionOutput {
    #[serde(rename = "ServiceActionDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_action_detail: Option<ServiceActionDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceActionDetail {
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ServiceActionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_action_summary: Option<ServiceActionSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceActionSummary {
    #[serde(rename = "DefinitionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition_type: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTagOptionInput {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTagOptionOutput {
    #[serde(rename = "TagOptionDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_option_detail: Option<TagOptionDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagOptionDetail {
    #[serde(rename = "Active")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConstraintInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConstraintOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePortfolioInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePortfolioOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePortfolioShareInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "OrganizationNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_node: Option<OrganizationNode>,
    #[serde(rename = "PortfolioId")]
    #[serde(default)]
    pub portfolio_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePortfolioShareOutput {
    #[serde(rename = "PortfolioShareToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_share_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProductInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProductOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProvisionedProductPlanInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "IgnoreErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_errors: Option<bool>,
    #[serde(rename = "PlanId")]
    #[serde(default)]
    pub plan_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProvisionedProductPlanOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProvisioningArtifactInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    pub product_id: String,
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(default)]
    pub provisioning_artifact_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProvisioningArtifactOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceActionInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceActionOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTagOptionInput {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTagOptionOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConstraintInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConstraintOutput {
    #[serde(rename = "ConstraintDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_detail: Option<ConstraintDetail>,
    #[serde(rename = "ConstraintParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_parameters: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCopyProductStatusInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "CopyProductToken")]
    #[serde(default)]
    pub copy_product_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCopyProductStatusOutput {
    #[serde(rename = "CopyProductStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_product_status: Option<String>,
    #[serde(rename = "StatusDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_detail: Option<String>,
    #[serde(rename = "TargetProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_product_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePortfolioInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePortfolioOutput {
    #[serde(rename = "Budgets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budgets: Option<Vec<BudgetDetail>>,
    #[serde(rename = "PortfolioDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_detail: Option<PortfolioDetail>,
    #[serde(rename = "TagOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_options: Option<Vec<TagOptionDetail>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BudgetDetail {
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePortfolioShareStatusInput {
    #[serde(rename = "PortfolioShareToken")]
    #[serde(default)]
    pub portfolio_share_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePortfolioShareStatusOutput {
    #[serde(rename = "OrganizationNodeValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_node_value: Option<String>,
    #[serde(rename = "PortfolioId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_id: Option<String>,
    #[serde(rename = "PortfolioShareToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_share_token: Option<String>,
    #[serde(rename = "ShareDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_details: Option<ShareDetails>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ShareDetails {
    #[serde(rename = "ShareErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_errors: Option<Vec<ShareError>>,
    #[serde(rename = "SuccessfulShares")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_shares: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ShareError {
    #[serde(rename = "Accounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<String>>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePortfolioSharesInput {
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "PortfolioId")]
    #[serde(default)]
    pub portfolio_id: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePortfolioSharesOutput {
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "PortfolioShareDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_share_details: Option<Vec<PortfolioShareDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PortfolioShareDetail {
    #[serde(rename = "Accepted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted: Option<bool>,
    #[serde(rename = "PrincipalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "SharePrincipals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_principals: Option<bool>,
    #[serde(rename = "ShareTagOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_tag_options: Option<bool>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProductAsAdminInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SourcePortfolioId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_portfolio_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProductAsAdminOutput {
    #[serde(rename = "Budgets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budgets: Option<Vec<BudgetDetail>>,
    #[serde(rename = "ProductViewDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_detail: Option<ProductViewDetail>,
    #[serde(rename = "ProvisioningArtifactSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_summaries: Option<Vec<ProvisioningArtifactSummary>>,
    #[serde(rename = "TagOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_options: Option<Vec<TagOptionDetail>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisioningArtifactSummary {
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ProvisioningArtifactMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_metadata: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProductInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProductOutput {
    #[serde(rename = "Budgets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budgets: Option<Vec<BudgetDetail>>,
    #[serde(rename = "LaunchPaths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_paths: Option<Vec<LaunchPath>>,
    #[serde(rename = "ProductViewSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_summary: Option<ProductViewSummary>,
    #[serde(rename = "ProvisioningArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifacts: Option<Vec<ProvisioningArtifact>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LaunchPath {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisioningArtifact {
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Guidance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProductViewInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProductViewOutput {
    #[serde(rename = "ProductViewSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_summary: Option<ProductViewSummary>,
    #[serde(rename = "ProvisioningArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifacts: Option<Vec<ProvisioningArtifact>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProvisionedProductInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProvisionedProductOutput {
    #[serde(rename = "CloudWatchDashboards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_dashboards: Option<Vec<CloudWatchDashboard>>,
    #[serde(rename = "ProvisionedProductDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_detail: Option<ProvisionedProductDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchDashboard {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisionedProductDetail {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    #[serde(rename = "LastProvisioningRecordId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_provisioning_record_id: Option<String>,
    #[serde(rename = "LastRecordId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_record_id: Option<String>,
    #[serde(rename = "LastSuccessfulProvisioningRecordId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_provisioning_record_id: Option<String>,
    #[serde(rename = "LaunchRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_role_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProvisionedProductPlanInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "PlanId")]
    #[serde(default)]
    pub plan_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProvisionedProductPlanOutput {
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "ProvisionedProductPlanDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_plan_details: Option<ProvisionedProductPlanDetails>,
    #[serde(rename = "ResourceChanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_changes: Option<Vec<ResourceChange>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisionedProductPlanDetails {
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "NotificationArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_arns: Option<Vec<String>>,
    #[serde(rename = "PathId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_id: Option<String>,
    #[serde(rename = "PlanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(rename = "PlanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(rename = "PlanType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "ProvisionProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provision_product_id: Option<String>,
    #[serde(rename = "ProvisionProductName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provision_product_name: Option<String>,
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    #[serde(rename = "ProvisioningParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_parameters: Option<Vec<UpdateProvisioningParameter>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "UpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceChange {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Details")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<ResourceChangeDetail>>,
    #[serde(rename = "LogicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<String>,
    #[serde(rename = "PhysicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource_id: Option<String>,
    #[serde(rename = "Replacement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceChangeDetail {
    #[serde(rename = "CausingEntity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causing_entity: Option<String>,
    #[serde(rename = "Evaluation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation: Option<String>,
    #[serde(rename = "Target")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<ResourceTargetDefinition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceTargetDefinition {
    #[serde(rename = "Attribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RequiresRecreation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_recreation: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProvisioningArtifactInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "IncludeProvisioningArtifactParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_provisioning_artifact_parameters: Option<bool>,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "ProductName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    #[serde(rename = "ProvisioningArtifactName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_name: Option<String>,
    #[serde(rename = "Verbose")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbose: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProvisioningArtifactOutput {
    #[serde(rename = "Info")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ProvisioningArtifactDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_detail: Option<ProvisioningArtifactDetail>,
    #[serde(rename = "ProvisioningArtifactParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_parameters: Option<Vec<ProvisioningArtifactParameter>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisioningArtifactParameter {
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IsNoEcho")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_no_echo: Option<bool>,
    #[serde(rename = "ParameterConstraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_constraints: Option<ParameterConstraints>,
    #[serde(rename = "ParameterKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_key: Option<String>,
    #[serde(rename = "ParameterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterConstraints {
    #[serde(rename = "AllowedPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_pattern: Option<String>,
    #[serde(rename = "AllowedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<String>>,
    #[serde(rename = "ConstraintDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_description: Option<String>,
    #[serde(rename = "MaxLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<String>,
    #[serde(rename = "MaxValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<String>,
    #[serde(rename = "MinLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<String>,
    #[serde(rename = "MinValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProvisioningParametersInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "PathId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_id: Option<String>,
    #[serde(rename = "PathName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_name: Option<String>,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "ProductName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    #[serde(rename = "ProvisioningArtifactName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProvisioningParametersOutput {
    #[serde(rename = "ConstraintSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_summaries: Option<Vec<ConstraintSummary>>,
    #[serde(rename = "ProvisioningArtifactOutputKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_output_keys: Option<Vec<ProvisioningArtifactOutput>>,
    #[serde(rename = "ProvisioningArtifactOutputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_outputs: Option<Vec<ProvisioningArtifactOutput>>,
    #[serde(rename = "ProvisioningArtifactParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_parameters: Option<Vec<ProvisioningArtifactParameter>>,
    #[serde(rename = "ProvisioningArtifactPreferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_preferences: Option<ProvisioningArtifactPreferences>,
    #[serde(rename = "TagOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_options: Option<Vec<TagOptionSummary>>,
    #[serde(rename = "UsageInstructions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_instructions: Option<Vec<UsageInstruction>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConstraintSummary {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisioningArtifactOutput {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisioningArtifactPreferences {
    #[serde(rename = "StackSetAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_accounts: Option<Vec<String>>,
    #[serde(rename = "StackSetRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_regions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagOptionSummary {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsageInstruction {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRecordInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRecordOutput {
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "RecordDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
    #[serde(rename = "RecordOutputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_outputs: Option<Vec<RecordOutput>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecordDetail {
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "LaunchRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_role_arn: Option<String>,
    #[serde(rename = "PathId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_id: Option<String>,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "ProvisionedProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_id: Option<String>,
    #[serde(rename = "ProvisionedProductName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_name: Option<String>,
    #[serde(rename = "ProvisionedProductType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_type: Option<String>,
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    #[serde(rename = "RecordErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_errors: Option<Vec<RecordError>>,
    #[serde(rename = "RecordId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    #[serde(rename = "RecordTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_tags: Option<Vec<RecordTag>>,
    #[serde(rename = "RecordType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecordError {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecordTag {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecordOutput {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "OutputKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_key: Option<String>,
    #[serde(rename = "OutputValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServiceActionExecutionParametersInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "ProvisionedProductId")]
    #[serde(default)]
    pub provisioned_product_id: String,
    #[serde(rename = "ServiceActionId")]
    #[serde(default)]
    pub service_action_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServiceActionExecutionParametersOutput {
    #[serde(rename = "ServiceActionParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_action_parameters: Option<Vec<ExecutionParameter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionParameter {
    #[serde(rename = "DefaultValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_values: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServiceActionInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServiceActionOutput {
    #[serde(rename = "ServiceActionDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_action_detail: Option<ServiceActionDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTagOptionInput {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTagOptionOutput {
    #[serde(rename = "TagOptionDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_option_detail: Option<TagOptionDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableAWSOrganizationsAccessInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableAWSOrganizationsAccessOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateBudgetFromResourceInput {
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    pub budget_name: String,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateBudgetFromResourceOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociatePrincipalFromPortfolioInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "PortfolioId")]
    #[serde(default)]
    pub portfolio_id: String,
    #[serde(rename = "PrincipalARN")]
    #[serde(default)]
    pub principal_a_r_n: String,
    #[serde(rename = "PrincipalType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociatePrincipalFromPortfolioOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateProductFromPortfolioInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "PortfolioId")]
    #[serde(default)]
    pub portfolio_id: String,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    pub product_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateProductFromPortfolioOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateServiceActionFromProvisioningArtifactInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    pub product_id: String,
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(default)]
    pub provisioning_artifact_id: String,
    #[serde(rename = "ServiceActionId")]
    #[serde(default)]
    pub service_action_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateServiceActionFromProvisioningArtifactOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateTagOptionFromResourceInput {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "TagOptionId")]
    #[serde(default)]
    pub tag_option_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateTagOptionFromResourceOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableAWSOrganizationsAccessInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableAWSOrganizationsAccessOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteProvisionedProductPlanInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    pub idempotency_token: String,
    #[serde(rename = "PlanId")]
    #[serde(default)]
    pub plan_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteProvisionedProductPlanOutput {
    #[serde(rename = "RecordDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteProvisionedProductServiceActionInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "ExecuteToken")]
    #[serde(default)]
    pub execute_token: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "ProvisionedProductId")]
    #[serde(default)]
    pub provisioned_product_id: String,
    #[serde(rename = "ServiceActionId")]
    #[serde(default)]
    pub service_action_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteProvisionedProductServiceActionOutput {
    #[serde(rename = "RecordDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAWSOrganizationsAccessStatusInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAWSOrganizationsAccessStatusOutput {
    #[serde(rename = "AccessStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetProvisionedProductOutputsInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "OutputKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_keys: Option<Vec<String>>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "ProvisionedProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_id: Option<String>,
    #[serde(rename = "ProvisionedProductName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetProvisionedProductOutputsOutput {
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "Outputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<RecordOutput>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportAsProvisionedProductInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    pub idempotency_token: String,
    #[serde(rename = "PhysicalId")]
    #[serde(default)]
    pub physical_id: String,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    pub product_id: String,
    #[serde(rename = "ProvisionedProductName")]
    #[serde(default)]
    pub provisioned_product_name: String,
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(default)]
    pub provisioning_artifact_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportAsProvisionedProductOutput {
    #[serde(rename = "RecordDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAcceptedPortfolioSharesInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "PortfolioShareType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_share_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAcceptedPortfolioSharesOutput {
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "PortfolioDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_details: Option<Vec<PortfolioDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBudgetsForResourceInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBudgetsForResourceOutput {
    #[serde(rename = "Budgets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budgets: Option<Vec<BudgetDetail>>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConstraintsForPortfolioInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "PortfolioId")]
    #[serde(default)]
    pub portfolio_id: String,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConstraintsForPortfolioOutput {
    #[serde(rename = "ConstraintDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_details: Option<Vec<ConstraintDetail>>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLaunchPathsInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    pub product_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLaunchPathsOutput {
    #[serde(rename = "LaunchPathSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_path_summaries: Option<Vec<LaunchPathSummary>>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LaunchPathSummary {
    #[serde(rename = "ConstraintSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_summaries: Option<Vec<ConstraintSummary>>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOrganizationPortfolioAccessInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "OrganizationNodeType")]
    #[serde(default)]
    pub organization_node_type: String,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "PortfolioId")]
    #[serde(default)]
    pub portfolio_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOrganizationPortfolioAccessOutput {
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "OrganizationNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_nodes: Option<Vec<OrganizationNode>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPortfolioAccessInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "OrganizationParentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_parent_id: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "PortfolioId")]
    #[serde(default)]
    pub portfolio_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPortfolioAccessOutput {
    #[serde(rename = "AccountIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPortfoliosForProductInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    pub product_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPortfoliosForProductOutput {
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "PortfolioDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_details: Option<Vec<PortfolioDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPortfoliosInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPortfoliosOutput {
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "PortfolioDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_details: Option<Vec<PortfolioDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPrincipalsForPortfolioInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "PortfolioId")]
    #[serde(default)]
    pub portfolio_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPrincipalsForPortfolioOutput {
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "Principals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<Principal>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Principal {
    #[serde(rename = "PrincipalARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_a_r_n: Option<String>,
    #[serde(rename = "PrincipalType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProvisionedProductPlansInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "AccessLevelFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level_filter: Option<AccessLevelFilter>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "ProvisionProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provision_product_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessLevelFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProvisionedProductPlansOutput {
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "ProvisionedProductPlans")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_plans: Option<Vec<ProvisionedProductPlanSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisionedProductPlanSummary {
    #[serde(rename = "PlanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(rename = "PlanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(rename = "PlanType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    #[serde(rename = "ProvisionProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provision_product_id: Option<String>,
    #[serde(rename = "ProvisionProductName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provision_product_name: Option<String>,
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProvisioningArtifactsForServiceActionInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "ServiceActionId")]
    #[serde(default)]
    pub service_action_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProvisioningArtifactsForServiceActionOutput {
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "ProvisioningArtifactViews")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_views: Option<Vec<ProvisioningArtifactView>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisioningArtifactView {
    #[serde(rename = "ProductViewSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_summary: Option<ProductViewSummary>,
    #[serde(rename = "ProvisioningArtifact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact: Option<ProvisioningArtifact>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProvisioningArtifactsInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    pub product_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProvisioningArtifactsOutput {
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "ProvisioningArtifactDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_details: Option<Vec<ProvisioningArtifactDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecordHistoryInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "AccessLevelFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level_filter: Option<AccessLevelFilter>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "SearchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<ListRecordHistorySearchFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecordHistorySearchFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecordHistoryOutput {
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "RecordDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_details: Option<Vec<RecordDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourcesForTagOptionInput {
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "TagOptionId")]
    #[serde(default)]
    pub tag_option_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourcesForTagOptionOutput {
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "ResourceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_details: Option<Vec<ResourceDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceDetail {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceActionsForProvisioningArtifactInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    pub product_id: String,
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(default)]
    pub provisioning_artifact_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceActionsForProvisioningArtifactOutput {
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "ServiceActionSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_action_summaries: Option<Vec<ServiceActionSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceActionsInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceActionsOutput {
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "ServiceActionSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_action_summaries: Option<Vec<ServiceActionSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStackInstancesForProvisionedProductInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "ProvisionedProductId")]
    #[serde(default)]
    pub provisioned_product_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStackInstancesForProvisionedProductOutput {
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "StackInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_instances: Option<Vec<StackInstance>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StackInstance {
    #[serde(rename = "Account")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "StackInstanceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_instance_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagOptionsInput {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ListTagOptionsFilters>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagOptionsFilters {
    #[serde(rename = "Active")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagOptionsOutput {
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "TagOptionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_option_details: Option<Vec<TagOptionDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotifyProvisionProductEngineWorkflowResultInput {
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    pub idempotency_token: String,
    #[serde(rename = "Outputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<RecordOutput>>,
    #[serde(rename = "RecordId")]
    #[serde(default)]
    pub record_id: String,
    #[serde(rename = "ResourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<EngineWorkflowResourceIdentifier>,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    #[serde(rename = "WorkflowToken")]
    #[serde(default)]
    pub workflow_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EngineWorkflowResourceIdentifier {
    #[serde(rename = "UniqueTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_tag: Option<UniqueTagResourceIdentifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UniqueTagResourceIdentifier {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotifyProvisionProductEngineWorkflowResultOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotifyTerminateProvisionedProductEngineWorkflowResultInput {
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    pub idempotency_token: String,
    #[serde(rename = "RecordId")]
    #[serde(default)]
    pub record_id: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    #[serde(rename = "WorkflowToken")]
    #[serde(default)]
    pub workflow_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotifyTerminateProvisionedProductEngineWorkflowResultOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotifyUpdateProvisionedProductEngineWorkflowResultInput {
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    pub idempotency_token: String,
    #[serde(rename = "Outputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<RecordOutput>>,
    #[serde(rename = "RecordId")]
    #[serde(default)]
    pub record_id: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    #[serde(rename = "WorkflowToken")]
    #[serde(default)]
    pub workflow_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotifyUpdateProvisionedProductEngineWorkflowResultOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisionProductInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "NotificationArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_arns: Option<Vec<String>>,
    #[serde(rename = "PathId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_id: Option<String>,
    #[serde(rename = "PathName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_name: Option<String>,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "ProductName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(rename = "ProvisionToken")]
    #[serde(default)]
    pub provision_token: String,
    #[serde(rename = "ProvisionedProductName")]
    #[serde(default)]
    pub provisioned_product_name: String,
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    #[serde(rename = "ProvisioningArtifactName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_name: Option<String>,
    #[serde(rename = "ProvisioningParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_parameters: Option<Vec<ProvisioningParameter>>,
    #[serde(rename = "ProvisioningPreferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_preferences: Option<ProvisioningPreferences>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisioningParameter {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisioningPreferences {
    #[serde(rename = "StackSetAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_accounts: Option<Vec<String>>,
    #[serde(rename = "StackSetFailureToleranceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_failure_tolerance_count: Option<i32>,
    #[serde(rename = "StackSetFailureTolerancePercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_failure_tolerance_percentage: Option<i32>,
    #[serde(rename = "StackSetMaxConcurrencyCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_max_concurrency_count: Option<i32>,
    #[serde(rename = "StackSetMaxConcurrencyPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_max_concurrency_percentage: Option<i32>,
    #[serde(rename = "StackSetRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_regions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisionProductOutput {
    #[serde(rename = "RecordDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectPortfolioShareInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "PortfolioId")]
    #[serde(default)]
    pub portfolio_id: String,
    #[serde(rename = "PortfolioShareType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_share_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectPortfolioShareOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanProvisionedProductsInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "AccessLevelFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level_filter: Option<AccessLevelFilter>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanProvisionedProductsOutput {
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "ProvisionedProducts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_products: Option<Vec<ProvisionedProductDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchProductsAsAdminInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "PortfolioId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_id: Option<String>,
    #[serde(rename = "ProductSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_source: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchProductsAsAdminOutput {
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "ProductViewDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_details: Option<Vec<ProductViewDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchProductsInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchProductsOutput {
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "ProductViewAggregations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_aggregations:
        Option<std::collections::HashMap<String, Vec<ProductViewAggregationValue>>>,
    #[serde(rename = "ProductViewSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_summaries: Option<Vec<ProductViewSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProductViewAggregationValue {
    #[serde(rename = "ApproximateCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_count: Option<i32>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchProvisionedProductsInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "AccessLevelFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level_filter: Option<AccessLevelFilter>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchProvisionedProductsOutput {
    #[serde(rename = "NextPageToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "ProvisionedProducts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_products: Option<Vec<ProvisionedProductAttribute>>,
    #[serde(rename = "TotalResultsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_results_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisionedProductAttribute {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    #[serde(rename = "LastProvisioningRecordId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_provisioning_record_id: Option<String>,
    #[serde(rename = "LastRecordId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_record_id: Option<String>,
    #[serde(rename = "LastSuccessfulProvisioningRecordId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_provisioning_record_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PhysicalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_id: Option<String>,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "ProductName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    #[serde(rename = "ProvisioningArtifactName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "UserArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
    #[serde(rename = "UserArnSession")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn_session: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminateProvisionedProductInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "IgnoreErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_errors: Option<bool>,
    #[serde(rename = "ProvisionedProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_id: Option<String>,
    #[serde(rename = "ProvisionedProductName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_name: Option<String>,
    #[serde(rename = "RetainPhysicalResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_physical_resources: Option<bool>,
    #[serde(rename = "TerminateToken")]
    #[serde(default)]
    pub terminate_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminateProvisionedProductOutput {
    #[serde(rename = "RecordDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConstraintInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConstraintOutput {
    #[serde(rename = "ConstraintDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_detail: Option<ConstraintDetail>,
    #[serde(rename = "ConstraintParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_parameters: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePortfolioInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "AddTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_tags: Option<Vec<Tag>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "ProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[serde(rename = "RemoveTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePortfolioOutput {
    #[serde(rename = "PortfolioDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_detail: Option<PortfolioDetail>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePortfolioShareInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "OrganizationNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_node: Option<OrganizationNode>,
    #[serde(rename = "PortfolioId")]
    #[serde(default)]
    pub portfolio_id: String,
    #[serde(rename = "SharePrincipals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_principals: Option<bool>,
    #[serde(rename = "ShareTagOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_tag_options: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePortfolioShareOutput {
    #[serde(rename = "PortfolioShareToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_share_token: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProductInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "AddTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_tags: Option<Vec<Tag>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Distributor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distributor: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "RemoveTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_tags: Option<Vec<String>>,
    #[serde(rename = "SourceConnection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_connection: Option<SourceConnection>,
    #[serde(rename = "SupportDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_description: Option<String>,
    #[serde(rename = "SupportEmail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    #[serde(rename = "SupportUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProductOutput {
    #[serde(rename = "ProductViewDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_view_detail: Option<ProductViewDetail>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProvisionedProductInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "PathId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_id: Option<String>,
    #[serde(rename = "PathName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_name: Option<String>,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "ProductName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(rename = "ProvisionedProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_id: Option<String>,
    #[serde(rename = "ProvisionedProductName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_name: Option<String>,
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_id: Option<String>,
    #[serde(rename = "ProvisioningArtifactName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_name: Option<String>,
    #[serde(rename = "ProvisioningParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_parameters: Option<Vec<UpdateProvisioningParameter>>,
    #[serde(rename = "ProvisioningPreferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_preferences: Option<UpdateProvisioningPreferences>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "UpdateToken")]
    #[serde(default)]
    pub update_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProvisioningPreferences {
    #[serde(rename = "StackSetAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_accounts: Option<Vec<String>>,
    #[serde(rename = "StackSetFailureToleranceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_failure_tolerance_count: Option<i32>,
    #[serde(rename = "StackSetFailureTolerancePercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_failure_tolerance_percentage: Option<i32>,
    #[serde(rename = "StackSetMaxConcurrencyCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_max_concurrency_count: Option<i32>,
    #[serde(rename = "StackSetMaxConcurrencyPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_max_concurrency_percentage: Option<i32>,
    #[serde(rename = "StackSetOperationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_operation_type: Option<String>,
    #[serde(rename = "StackSetRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_regions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProvisionedProductOutput {
    #[serde(rename = "RecordDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_detail: Option<RecordDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProvisionedProductPropertiesInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    pub idempotency_token: String,
    #[serde(rename = "ProvisionedProductId")]
    #[serde(default)]
    pub provisioned_product_id: String,
    #[serde(rename = "ProvisionedProductProperties")]
    #[serde(default)]
    pub provisioned_product_properties: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProvisionedProductPropertiesOutput {
    #[serde(rename = "ProvisionedProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_id: Option<String>,
    #[serde(rename = "ProvisionedProductProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_product_properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "RecordId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProvisioningArtifactInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "Active")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Guidance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidance: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ProductId")]
    #[serde(default)]
    pub product_id: String,
    #[serde(rename = "ProvisioningArtifactId")]
    #[serde(default)]
    pub provisioning_artifact_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProvisioningArtifactOutput {
    #[serde(rename = "Info")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ProvisioningArtifactDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_artifact_detail: Option<ProvisioningArtifactDetail>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServiceActionInput {
    #[serde(rename = "AcceptLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServiceActionOutput {
    #[serde(rename = "ServiceActionDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_action_detail: Option<ServiceActionDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTagOptionInput {
    #[serde(rename = "Active")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTagOptionOutput {
    #[serde(rename = "TagOptionDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_option_detail: Option<TagOptionDetail>,
}
