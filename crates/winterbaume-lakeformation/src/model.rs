//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-lakeformation

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddLFTagsToResourceRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "LFTags")]
    #[serde(default)]
    pub l_f_tags: Vec<LFTagPair>,
    #[serde(rename = "Resource")]
    #[serde(default)]
    pub resource: Resource,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LFTagPair {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "TagKey")]
    #[serde(default)]
    pub tag_key: String,
    #[serde(rename = "TagValues")]
    #[serde(default)]
    pub tag_values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Resource {
    #[serde(rename = "Catalog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog: Option<CatalogResource>,
    #[serde(rename = "DataCellsFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_cells_filter: Option<DataCellsFilterResource>,
    #[serde(rename = "DataLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_location: Option<DataLocationResource>,
    #[serde(rename = "Database")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<DatabaseResource>,
    #[serde(rename = "LFTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l_f_tag: Option<LFTagKeyResource>,
    #[serde(rename = "LFTagExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l_f_tag_expression: Option<LFTagExpressionResource>,
    #[serde(rename = "LFTagPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l_f_tag_policy: Option<LFTagPolicyResource>,
    #[serde(rename = "Table")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<TableResource>,
    #[serde(rename = "TableWithColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_with_columns: Option<TableWithColumnsResource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CatalogResource {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataCellsFilterResource {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "TableCatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_catalog_id: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataLocationResource {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatabaseResource {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LFTagKeyResource {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "TagKey")]
    #[serde(default)]
    pub tag_key: String,
    #[serde(rename = "TagValues")]
    #[serde(default)]
    pub tag_values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LFTagExpressionResource {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LFTagPolicyResource {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "Expression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<Vec<LFTag>>,
    #[serde(rename = "ExpressionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_name: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LFTag {
    #[serde(rename = "TagKey")]
    #[serde(default)]
    pub tag_key: String,
    #[serde(rename = "TagValues")]
    #[serde(default)]
    pub tag_values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableResource {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "TableWildcard")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_wildcard: Option<TableWildcard>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableWildcard {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableWithColumnsResource {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "ColumnNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_names: Option<Vec<String>>,
    #[serde(rename = "ColumnWildcard")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_wildcard: Option<ColumnWildcard>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnWildcard {
    #[serde(rename = "ExcludedColumnNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_column_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddLFTagsToResourceResponse {
    #[serde(rename = "Failures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<LFTagError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LFTagError {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
    #[serde(rename = "LFTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l_f_tag: Option<LFTagPair>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorDetail {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssumeDecoratedRoleWithSAMLRequest {
    #[serde(rename = "DurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
    #[serde(rename = "PrincipalArn")]
    #[serde(default)]
    pub principal_arn: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "SAMLAssertion")]
    #[serde(default)]
    pub s_a_m_l_assertion: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssumeDecoratedRoleWithSAMLResponse {
    #[serde(rename = "AccessKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "Expiration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<f64>,
    #[serde(rename = "SecretAccessKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
    #[serde(rename = "SessionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGrantPermissionsRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "Entries")]
    #[serde(default)]
    pub entries: Vec<BatchPermissionsRequestEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchPermissionsRequestEntry {
    #[serde(rename = "Condition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "PermissionsWithGrantOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_with_grant_option: Option<Vec<String>>,
    #[serde(rename = "Principal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<DataLakePrincipal>,
    #[serde(rename = "Resource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Resource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Condition {
    #[serde(rename = "Expression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataLakePrincipal {
    #[serde(rename = "DataLakePrincipalIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_lake_principal_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGrantPermissionsResponse {
    #[serde(rename = "Failures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<BatchPermissionsFailureEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchPermissionsFailureEntry {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
    #[serde(rename = "RequestEntry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_entry: Option<BatchPermissionsRequestEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchRevokePermissionsRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "Entries")]
    #[serde(default)]
    pub entries: Vec<BatchPermissionsRequestEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchRevokePermissionsResponse {
    #[serde(rename = "Failures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<BatchPermissionsFailureEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelTransactionRequest {
    #[serde(rename = "TransactionId")]
    #[serde(default)]
    pub transaction_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelTransactionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommitTransactionRequest {
    #[serde(rename = "TransactionId")]
    #[serde(default)]
    pub transaction_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommitTransactionResponse {
    #[serde(rename = "TransactionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataCellsFilterRequest {
    #[serde(rename = "TableData")]
    #[serde(default)]
    pub table_data: DataCellsFilter,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataCellsFilter {
    #[serde(rename = "ColumnNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_names: Option<Vec<String>>,
    #[serde(rename = "ColumnWildcard")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_wildcard: Option<ColumnWildcard>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RowFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_filter: Option<RowFilter>,
    #[serde(rename = "TableCatalogId")]
    #[serde(default)]
    pub table_catalog_id: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RowFilter {
    #[serde(rename = "AllRowsWildcard")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_rows_wildcard: Option<AllRowsWildcard>,
    #[serde(rename = "FilterExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AllRowsWildcard {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataCellsFilterResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLFTagExpressionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Expression")]
    #[serde(default)]
    pub expression: Vec<LFTag>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLFTagExpressionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLFTagRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "TagKey")]
    #[serde(default)]
    pub tag_key: String,
    #[serde(rename = "TagValues")]
    #[serde(default)]
    pub tag_values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLFTagResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLakeFormationIdentityCenterConfigurationRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "ExternalFiltering")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_filtering: Option<ExternalFilteringConfiguration>,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    #[serde(rename = "ServiceIntegrations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_integrations: Option<Vec<ServiceIntegrationUnion>>,
    #[serde(rename = "ShareRecipients")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_recipients: Option<Vec<DataLakePrincipal>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExternalFilteringConfiguration {
    #[serde(rename = "AuthorizedTargets")]
    #[serde(default)]
    pub authorized_targets: Vec<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceIntegrationUnion {
    #[serde(rename = "Redshift")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift: Option<Vec<RedshiftScopeUnion>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftScopeUnion {
    #[serde(rename = "RedshiftConnect")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_connect: Option<RedshiftConnect>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftConnect {
    #[serde(rename = "Authorization")]
    #[serde(default)]
    pub authorization: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLakeFormationIdentityCenterConfigurationResponse {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLakeFormationOptInRequest {
    #[serde(rename = "Condition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    #[serde(rename = "Principal")]
    #[serde(default)]
    pub principal: DataLakePrincipal,
    #[serde(rename = "Resource")]
    #[serde(default)]
    pub resource: Resource,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLakeFormationOptInResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataCellsFilterRequest {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "TableCatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_catalog_id: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataCellsFilterResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLFTagExpressionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLFTagExpressionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLFTagRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "TagKey")]
    #[serde(default)]
    pub tag_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLFTagResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLakeFormationIdentityCenterConfigurationRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLakeFormationIdentityCenterConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLakeFormationOptInRequest {
    #[serde(rename = "Condition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    #[serde(rename = "Principal")]
    #[serde(default)]
    pub principal: DataLakePrincipal,
    #[serde(rename = "Resource")]
    #[serde(default)]
    pub resource: Resource,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLakeFormationOptInResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteObjectsOnCancelRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "Objects")]
    #[serde(default)]
    pub objects: Vec<VirtualObject>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "TransactionId")]
    #[serde(default)]
    pub transaction_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VirtualObject {
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "Uri")]
    #[serde(default)]
    pub uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteObjectsOnCancelResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLakeFormationIdentityCenterConfigurationRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLakeFormationIdentityCenterConfigurationResponse {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "ExternalFiltering")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_filtering: Option<ExternalFilteringConfiguration>,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    #[serde(rename = "ResourceShare")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share: Option<String>,
    #[serde(rename = "ServiceIntegrations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_integrations: Option<Vec<ServiceIntegrationUnion>>,
    #[serde(rename = "ShareRecipients")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_recipients: Option<Vec<DataLakePrincipal>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeResourceResponse {
    #[serde(rename = "ResourceInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_info: Option<ResourceInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceInfo {
    #[serde(rename = "ExpectedResourceOwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_resource_owner_account: Option<String>,
    #[serde(rename = "HybridAccessEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hybrid_access_enabled: Option<bool>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "VerificationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<String>,
    #[serde(rename = "WithFederation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_federation: Option<bool>,
    #[serde(rename = "WithPrivilegedAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_privileged_access: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTransactionRequest {
    #[serde(rename = "TransactionId")]
    #[serde(default)]
    pub transaction_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTransactionResponse {
    #[serde(rename = "TransactionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_description: Option<TransactionDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransactionDescription {
    #[serde(rename = "TransactionEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_end_time: Option<f64>,
    #[serde(rename = "TransactionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(rename = "TransactionStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_start_time: Option<f64>,
    #[serde(rename = "TransactionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExtendTransactionRequest {
    #[serde(rename = "TransactionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExtendTransactionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataCellsFilterRequest {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "TableCatalogId")]
    #[serde(default)]
    pub table_catalog_id: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataCellsFilterResponse {
    #[serde(rename = "DataCellsFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_cells_filter: Option<DataCellsFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataLakePrincipalRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataLakePrincipalResponse {
    #[serde(rename = "Identity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataLakeSettingsRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataLakeSettingsResponse {
    #[serde(rename = "DataLakeSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_lake_settings: Option<DataLakeSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataLakeSettings {
    #[serde(rename = "AllowExternalDataFiltering")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_external_data_filtering: Option<bool>,
    #[serde(rename = "AllowFullTableExternalDataAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_full_table_external_data_access: Option<bool>,
    #[serde(rename = "AuthorizedSessionTagValueList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_session_tag_value_list: Option<Vec<String>>,
    #[serde(rename = "CreateDatabaseDefaultPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_database_default_permissions: Option<Vec<PrincipalPermissions>>,
    #[serde(rename = "CreateTableDefaultPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_table_default_permissions: Option<Vec<PrincipalPermissions>>,
    #[serde(rename = "DataLakeAdmins")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_lake_admins: Option<Vec<DataLakePrincipal>>,
    #[serde(rename = "ExternalDataFilteringAllowList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_data_filtering_allow_list: Option<Vec<DataLakePrincipal>>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ReadOnlyAdmins")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only_admins: Option<Vec<DataLakePrincipal>>,
    #[serde(rename = "TrustedResourceOwners")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_resource_owners: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrincipalPermissions {
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "Principal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<DataLakePrincipal>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEffectivePermissionsForPathRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEffectivePermissionsForPathResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<PrincipalResourcePermissions>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrincipalResourcePermissions {
    #[serde(rename = "AdditionalDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<DetailsMap>,
    #[serde(rename = "Condition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    #[serde(rename = "LastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    #[serde(rename = "LastUpdatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "PermissionsWithGrantOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_with_grant_option: Option<Vec<String>>,
    #[serde(rename = "Principal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<DataLakePrincipal>,
    #[serde(rename = "Resource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Resource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetailsMap {
    #[serde(rename = "ResourceShare")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLFTagExpressionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLFTagExpressionResponse {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Expression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<Vec<LFTag>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLFTagRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "TagKey")]
    #[serde(default)]
    pub tag_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLFTagResponse {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "TagKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    #[serde(rename = "TagValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetQueryStateRequest {
    #[serde(rename = "QueryId")]
    #[serde(default)]
    pub query_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetQueryStateResponse {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetQueryStatisticsRequest {
    #[serde(rename = "QueryId")]
    #[serde(default)]
    pub query_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetQueryStatisticsResponse {
    #[serde(rename = "ExecutionStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_statistics: Option<ExecutionStatistics>,
    #[serde(rename = "PlanningStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planning_statistics: Option<PlanningStatistics>,
    #[serde(rename = "QuerySubmissionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_submission_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionStatistics {
    #[serde(rename = "AverageExecutionTimeMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_execution_time_millis: Option<i64>,
    #[serde(rename = "DataScannedBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_scanned_bytes: Option<i64>,
    #[serde(rename = "WorkUnitsExecutedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_units_executed_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlanningStatistics {
    #[serde(rename = "EstimatedDataToScanBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_data_to_scan_bytes: Option<i64>,
    #[serde(rename = "PlanningTimeMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planning_time_millis: Option<i64>,
    #[serde(rename = "QueueTimeMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_time_millis: Option<i64>,
    #[serde(rename = "WorkUnitsGeneratedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_units_generated_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceLFTagsRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "Resource")]
    #[serde(default)]
    pub resource: Resource,
    #[serde(rename = "ShowAssignedLFTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_assigned_l_f_tags: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceLFTagsResponse {
    #[serde(rename = "LFTagOnDatabase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l_f_tag_on_database: Option<Vec<LFTagPair>>,
    #[serde(rename = "LFTagsOnColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l_f_tags_on_columns: Option<Vec<ColumnLFTag>>,
    #[serde(rename = "LFTagsOnTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l_f_tags_on_table: Option<Vec<LFTagPair>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnLFTag {
    #[serde(rename = "LFTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l_f_tags: Option<Vec<LFTagPair>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableObjectsRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PartitionPredicate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_predicate: Option<String>,
    #[serde(rename = "QueryAsOfTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_as_of_time: Option<f64>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "TransactionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTableObjectsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Objects")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objects: Option<Vec<PartitionObjects>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PartitionObjects {
    #[serde(rename = "Objects")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objects: Option<Vec<TableObject>>,
    #[serde(rename = "PartitionValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableObject {
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "Size")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTemporaryDataLocationCredentialsRequest {
    #[serde(rename = "AuditContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_context: Option<AuditContext>,
    #[serde(rename = "CredentialsScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_scope: Option<String>,
    #[serde(rename = "DataLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_locations: Option<Vec<String>>,
    #[serde(rename = "DurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditContext {
    #[serde(rename = "AdditionalAuditContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_audit_context: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTemporaryDataLocationCredentialsResponse {
    #[serde(rename = "AccessibleDataLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessible_data_locations: Option<Vec<String>>,
    #[serde(rename = "Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<TemporaryCredentials>,
    #[serde(rename = "CredentialsScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemporaryCredentials {
    #[serde(rename = "AccessKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "Expiration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<f64>,
    #[serde(rename = "SecretAccessKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
    #[serde(rename = "SessionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTemporaryGluePartitionCredentialsRequest {
    #[serde(rename = "AuditContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_context: Option<AuditContext>,
    #[serde(rename = "DurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
    #[serde(rename = "Partition")]
    #[serde(default)]
    pub partition: PartitionValueList,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "SupportedPermissionTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_permission_types: Option<Vec<String>>,
    #[serde(rename = "TableArn")]
    #[serde(default)]
    pub table_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PartitionValueList {
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTemporaryGluePartitionCredentialsResponse {
    #[serde(rename = "AccessKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "Expiration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<f64>,
    #[serde(rename = "SecretAccessKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
    #[serde(rename = "SessionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTemporaryGlueTableCredentialsRequest {
    #[serde(rename = "AuditContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_context: Option<AuditContext>,
    #[serde(rename = "DurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "QuerySessionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_session_context: Option<QuerySessionContext>,
    #[serde(rename = "S3Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_path: Option<String>,
    #[serde(rename = "SupportedPermissionTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_permission_types: Option<Vec<String>>,
    #[serde(rename = "TableArn")]
    #[serde(default)]
    pub table_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuerySessionContext {
    #[serde(rename = "AdditionalContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "QueryAuthorizationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_authorization_id: Option<String>,
    #[serde(rename = "QueryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[serde(rename = "QueryStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTemporaryGlueTableCredentialsResponse {
    #[serde(rename = "AccessKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "Expiration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<f64>,
    #[serde(rename = "SecretAccessKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
    #[serde(rename = "SessionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,
    #[serde(rename = "VendedS3Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vended_s3_path: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetWorkUnitResultsRequest {
    #[serde(rename = "QueryId")]
    #[serde(default)]
    pub query_id: String,
    #[serde(rename = "WorkUnitId")]
    #[serde(default)]
    pub work_unit_id: i64,
    #[serde(rename = "WorkUnitToken")]
    #[serde(default)]
    pub work_unit_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetWorkUnitResultsResponse {
    #[serde(rename = "ResultStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_stream: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetWorkUnitsRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "QueryId")]
    #[serde(default)]
    pub query_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetWorkUnitsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QueryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[serde(rename = "WorkUnitRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_unit_ranges: Option<Vec<WorkUnitRange>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkUnitRange {
    #[serde(rename = "WorkUnitIdMax")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_unit_id_max: Option<i64>,
    #[serde(rename = "WorkUnitIdMin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_unit_id_min: Option<i64>,
    #[serde(rename = "WorkUnitToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_unit_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrantPermissionsRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "Condition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(rename = "PermissionsWithGrantOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_with_grant_option: Option<Vec<String>>,
    #[serde(rename = "Principal")]
    #[serde(default)]
    pub principal: DataLakePrincipal,
    #[serde(rename = "Resource")]
    #[serde(default)]
    pub resource: Resource,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrantPermissionsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataCellsFilterRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Table")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<TableResource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataCellsFilterResponse {
    #[serde(rename = "DataCellsFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_cells_filters: Option<Vec<DataCellsFilter>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLFTagExpressionsRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLFTagExpressionsResponse {
    #[serde(rename = "LFTagExpressions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l_f_tag_expressions: Option<Vec<LFTagExpression>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LFTagExpression {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Expression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<Vec<LFTag>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLFTagsRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceShareType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLFTagsResponse {
    #[serde(rename = "LFTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l_f_tags: Option<Vec<LFTagPair>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLakeFormationOptInsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Principal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<DataLakePrincipal>,
    #[serde(rename = "Resource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Resource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLakeFormationOptInsResponse {
    #[serde(rename = "LakeFormationOptInsInfoList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lake_formation_opt_ins_info_list: Option<Vec<LakeFormationOptInsInfo>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LakeFormationOptInsInfo {
    #[serde(rename = "Condition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    #[serde(rename = "LastUpdatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<String>,
    #[serde(rename = "Principal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<DataLakePrincipal>,
    #[serde(rename = "Resource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Resource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPermissionsRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "IncludeRelated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_related: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Principal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<DataLakePrincipal>,
    #[serde(rename = "Resource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Resource>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPermissionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PrincipalResourcePermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_resource_permissions: Option<Vec<PrincipalResourcePermissions>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourcesRequest {
    #[serde(rename = "FilterConditionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_condition_list: Option<Vec<FilterCondition>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterCondition {
    #[serde(rename = "ComparisonOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    #[serde(rename = "Field")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(rename = "StringValueList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourcesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceInfoList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_info_list: Option<Vec<ResourceInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTableStorageOptimizersRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StorageOptimizerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_optimizer_type: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTableStorageOptimizersResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StorageOptimizerList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_optimizer_list: Option<Vec<StorageOptimizer>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StorageOptimizer {
    #[serde(rename = "Config")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "LastRunDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_details: Option<String>,
    #[serde(rename = "StorageOptimizerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_optimizer_type: Option<String>,
    #[serde(rename = "Warnings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTransactionsRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StatusFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_filter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTransactionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Transactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<TransactionDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDataLakeSettingsRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DataLakeSettings")]
    #[serde(default)]
    pub data_lake_settings: DataLakeSettings,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDataLakeSettingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterResourceRequest {
    #[serde(rename = "ExpectedResourceOwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_resource_owner_account: Option<String>,
    #[serde(rename = "HybridAccessEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hybrid_access_enabled: Option<bool>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "UseServiceLinkedRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_service_linked_role: Option<bool>,
    #[serde(rename = "WithFederation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_federation: Option<bool>,
    #[serde(rename = "WithPrivilegedAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_privileged_access: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveLFTagsFromResourceRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "LFTags")]
    #[serde(default)]
    pub l_f_tags: Vec<LFTagPair>,
    #[serde(rename = "Resource")]
    #[serde(default)]
    pub resource: Resource,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveLFTagsFromResourceResponse {
    #[serde(rename = "Failures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<LFTagError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevokePermissionsRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "Condition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(rename = "PermissionsWithGrantOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_with_grant_option: Option<Vec<String>>,
    #[serde(rename = "Principal")]
    #[serde(default)]
    pub principal: DataLakePrincipal,
    #[serde(rename = "Resource")]
    #[serde(default)]
    pub resource: Resource,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevokePermissionsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchDatabasesByLFTagsRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "Expression")]
    #[serde(default)]
    pub expression: Vec<LFTag>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchDatabasesByLFTagsResponse {
    #[serde(rename = "DatabaseList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_list: Option<Vec<TaggedDatabase>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaggedDatabase {
    #[serde(rename = "Database")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<DatabaseResource>,
    #[serde(rename = "LFTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l_f_tags: Option<Vec<LFTagPair>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchTablesByLFTagsRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "Expression")]
    #[serde(default)]
    pub expression: Vec<LFTag>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchTablesByLFTagsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TableList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_list: Option<Vec<TaggedTable>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaggedTable {
    #[serde(rename = "LFTagOnDatabase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l_f_tag_on_database: Option<Vec<LFTagPair>>,
    #[serde(rename = "LFTagsOnColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l_f_tags_on_columns: Option<Vec<ColumnLFTag>>,
    #[serde(rename = "LFTagsOnTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l_f_tags_on_table: Option<Vec<LFTagPair>>,
    #[serde(rename = "Table")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<TableResource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartQueryPlanningRequest {
    #[serde(rename = "QueryPlanningContext")]
    #[serde(default)]
    pub query_planning_context: QueryPlanningContext,
    #[serde(rename = "QueryString")]
    #[serde(default)]
    pub query_string: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryPlanningContext {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "QueryAsOfTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_as_of_time: Option<f64>,
    #[serde(rename = "QueryParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TransactionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartQueryPlanningResponse {
    #[serde(rename = "QueryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTransactionRequest {
    #[serde(rename = "TransactionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTransactionResponse {
    #[serde(rename = "TransactionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataCellsFilterRequest {
    #[serde(rename = "TableData")]
    #[serde(default)]
    pub table_data: DataCellsFilter,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataCellsFilterResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLFTagExpressionRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Expression")]
    #[serde(default)]
    pub expression: Vec<LFTag>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLFTagExpressionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLFTagRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "TagKey")]
    #[serde(default)]
    pub tag_key: String,
    #[serde(rename = "TagValuesToAdd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values_to_add: Option<Vec<String>>,
    #[serde(rename = "TagValuesToDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values_to_delete: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLFTagResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLakeFormationIdentityCenterConfigurationRequest {
    #[serde(rename = "ApplicationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_status: Option<String>,
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "ExternalFiltering")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_filtering: Option<ExternalFilteringConfiguration>,
    #[serde(rename = "ServiceIntegrations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_integrations: Option<Vec<ServiceIntegrationUnion>>,
    #[serde(rename = "ShareRecipients")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_recipients: Option<Vec<DataLakePrincipal>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLakeFormationIdentityCenterConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResourceRequest {
    #[serde(rename = "ExpectedResourceOwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_resource_owner_account: Option<String>,
    #[serde(rename = "HybridAccessEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hybrid_access_enabled: Option<bool>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "WithFederation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_federation: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTableObjectsRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "TransactionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(rename = "WriteOperations")]
    #[serde(default)]
    pub write_operations: Vec<WriteOperation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WriteOperation {
    #[serde(rename = "AddObject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_object: Option<AddObjectInput>,
    #[serde(rename = "DeleteObject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_object: Option<DeleteObjectInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddObjectInput {
    #[serde(rename = "ETag")]
    #[serde(default)]
    pub e_tag: String,
    #[serde(rename = "PartitionValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_values: Option<Vec<String>>,
    #[serde(rename = "Size")]
    #[serde(default)]
    pub size: i64,
    #[serde(rename = "Uri")]
    #[serde(default)]
    pub uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteObjectInput {
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "PartitionValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_values: Option<Vec<String>>,
    #[serde(rename = "Uri")]
    #[serde(default)]
    pub uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTableObjectsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTableStorageOptimizerRequest {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "StorageOptimizerConfig")]
    #[serde(default)]
    pub storage_optimizer_config:
        std::collections::HashMap<String, std::collections::HashMap<String, String>>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTableStorageOptimizerResponse {
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}
