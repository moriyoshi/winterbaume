//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-iot

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptCertificateTransferRequest {
    #[serde(rename = "certificateId")]
    #[serde(default)]
    pub certificate_id: String,
    #[serde(rename = "setAsActive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_as_active: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddThingToBillingGroupRequest {
    #[serde(rename = "billingGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_group_arn: Option<String>,
    #[serde(rename = "billingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_group_name: Option<String>,
    #[serde(rename = "thingArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_arn: Option<String>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddThingToBillingGroupResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddThingToThingGroupRequest {
    #[serde(rename = "overrideDynamicGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_dynamic_groups: Option<bool>,
    #[serde(rename = "thingArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_arn: Option<String>,
    #[serde(rename = "thingGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_arn: Option<String>,
    #[serde(rename = "thingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_name: Option<String>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddThingToThingGroupResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateSbomWithPackageVersionRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "packageName")]
    #[serde(default)]
    pub package_name: String,
    #[serde(default)]
    pub sbom: Sbom,
    #[serde(rename = "versionName")]
    #[serde(default)]
    pub version_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Sbom {
    #[serde(rename = "s3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_location: Option<S3Location>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Location {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateSbomWithPackageVersionResponse {
    #[serde(rename = "packageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sbom: Option<Sbom>,
    #[serde(rename = "sbomValidationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sbom_validation_status: Option<String>,
    #[serde(rename = "versionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateTargetsWithJobRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "namespaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
    #[serde(default)]
    pub targets: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateTargetsWithJobResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachPolicyRequest {
    #[serde(rename = "policyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(default)]
    pub target: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachPrincipalPolicyRequest {
    #[serde(rename = "policyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(default)]
    pub principal: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachSecurityProfileRequest {
    #[serde(rename = "securityProfileName")]
    #[serde(default)]
    pub security_profile_name: String,
    #[serde(rename = "securityProfileTargetArn")]
    #[serde(default)]
    pub security_profile_target_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachSecurityProfileResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachThingPrincipalRequest {
    #[serde(default)]
    pub principal: String,
    #[serde(rename = "thingName")]
    #[serde(default)]
    pub thing_name: String,
    #[serde(rename = "thingPrincipalType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_principal_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachThingPrincipalResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelAuditMitigationActionsTaskRequest {
    #[serde(rename = "taskId")]
    #[serde(default)]
    pub task_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelAuditMitigationActionsTaskResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelAuditTaskRequest {
    #[serde(rename = "taskId")]
    #[serde(default)]
    pub task_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelAuditTaskResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelCertificateTransferRequest {
    #[serde(rename = "certificateId")]
    #[serde(default)]
    pub certificate_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelDetectMitigationActionsTaskRequest {
    #[serde(rename = "taskId")]
    #[serde(default)]
    pub task_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelDetectMitigationActionsTaskResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelJobExecutionRequest {
    #[serde(rename = "expectedVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_version: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "statusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    pub thing_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelJobRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "reasonCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelJobResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClearDefaultAuthorizerRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClearDefaultAuthorizerResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfirmTopicRuleDestinationRequest {
    #[serde(rename = "confirmationToken")]
    #[serde(default)]
    pub confirmation_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfirmTopicRuleDestinationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAuditSuppressionRequest {
    #[serde(rename = "checkName")]
    #[serde(default)]
    pub check_name: String,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    pub client_request_token: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "expirationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    pub resource_identifier: ResourceIdentifier,
    #[serde(rename = "suppressIndefinitely")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_indefinitely: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceIdentifier {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "caCertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_id: Option<String>,
    #[serde(rename = "clientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "cognitoIdentityPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_identity_pool_id: Option<String>,
    #[serde(rename = "deviceCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_certificate_arn: Option<String>,
    #[serde(rename = "deviceCertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_certificate_id: Option<String>,
    #[serde(rename = "iamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "issuerCertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_certificate_identifier: Option<IssuerCertificateIdentifier>,
    #[serde(rename = "policyVersionIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version_identifier: Option<PolicyVersionIdentifier>,
    #[serde(rename = "roleAliasArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_alias_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IssuerCertificateIdentifier {
    #[serde(rename = "issuerCertificateSerialNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_certificate_serial_number: Option<String>,
    #[serde(rename = "issuerCertificateSubject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_certificate_subject: Option<String>,
    #[serde(rename = "issuerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyVersionIdentifier {
    #[serde(rename = "policyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "policyVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAuditSuppressionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAuthorizerRequest {
    #[serde(rename = "authorizerFunctionArn")]
    #[serde(default)]
    pub authorizer_function_arn: String,
    #[serde(rename = "authorizerName")]
    #[serde(default)]
    pub authorizer_name: String,
    #[serde(rename = "enableCachingForHttp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_caching_for_http: Option<bool>,
    #[serde(rename = "signingDisabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_disabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "tokenKeyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key_name: Option<String>,
    #[serde(rename = "tokenSigningPublicKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_signing_public_keys: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAuthorizerResponse {
    #[serde(rename = "authorizerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_arn: Option<String>,
    #[serde(rename = "authorizerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBillingGroupRequest {
    #[serde(rename = "billingGroupName")]
    #[serde(default)]
    pub billing_group_name: String,
    #[serde(rename = "billingGroupProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_group_properties: Option<BillingGroupProperties>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BillingGroupProperties {
    #[serde(rename = "billingGroupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_group_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBillingGroupResponse {
    #[serde(rename = "billingGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_group_arn: Option<String>,
    #[serde(rename = "billingGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_group_id: Option<String>,
    #[serde(rename = "billingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCertificateFromCsrRequest {
    #[serde(rename = "certificateSigningRequest")]
    #[serde(default)]
    pub certificate_signing_request: String,
    #[serde(rename = "setAsActive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_as_active: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCertificateFromCsrResponse {
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "certificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    #[serde(rename = "certificatePem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_pem: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCertificateProviderRequest {
    #[serde(rename = "accountDefaultForOperations")]
    #[serde(default)]
    pub account_default_for_operations: Vec<String>,
    #[serde(rename = "certificateProviderName")]
    #[serde(default)]
    pub certificate_provider_name: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "lambdaFunctionArn")]
    #[serde(default)]
    pub lambda_function_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCertificateProviderResponse {
    #[serde(rename = "certificateProviderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_provider_arn: Option<String>,
    #[serde(rename = "certificateProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_provider_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCommandRequest {
    #[serde(rename = "commandId")]
    #[serde(default)]
    pub command_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "mandatoryParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandatory_parameters: Option<Vec<CommandParameter>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<CommandPayload>,
    #[serde(rename = "payloadTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_template: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preprocessor: Option<CommandPreprocessor>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommandParameter {
    #[serde(rename = "defaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<CommandParameterValue>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<CommandParameterValue>,
    #[serde(rename = "valueConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_conditions: Option<Vec<CommandParameterValueCondition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommandParameterValue {
    #[serde(rename = "B")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<bool>,
    #[serde(rename = "BIN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_i_n: Option<String>,
    #[serde(rename = "D")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d: Option<f64>,
    #[serde(rename = "I")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i: Option<i32>,
    #[serde(rename = "L")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l: Option<i64>,
    #[serde(rename = "S")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(rename = "UL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_l: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommandParameterValueCondition {
    #[serde(rename = "comparisonOperator")]
    #[serde(default)]
    pub comparison_operator: String,
    #[serde(default)]
    pub operand: CommandParameterValueComparisonOperand,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommandParameterValueComparisonOperand {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(rename = "numberRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_range: Option<CommandParameterValueNumberRange>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numbers: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommandParameterValueNumberRange {
    #[serde(default)]
    pub max: String,
    #[serde(default)]
    pub min: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommandPayload {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "contentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommandPreprocessor {
    #[serde(rename = "awsJsonSubstitution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_json_substitution: Option<AwsJsonSubstitutionCommandPreprocessorConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsJsonSubstitutionCommandPreprocessorConfig {
    #[serde(rename = "outputFormat")]
    #[serde(default)]
    pub output_format: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCommandResponse {
    #[serde(rename = "commandArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_arn: Option<String>,
    #[serde(rename = "commandId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCustomMetricRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    pub client_request_token: String,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "metricName")]
    #[serde(default)]
    pub metric_name: String,
    #[serde(rename = "metricType")]
    #[serde(default)]
    pub metric_type: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCustomMetricResponse {
    #[serde(rename = "metricArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_arn: Option<String>,
    #[serde(rename = "metricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDimensionRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    pub client_request_token: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "stringValues")]
    #[serde(default)]
    pub string_values: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDimensionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDomainConfigurationRequest {
    #[serde(rename = "applicationProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_protocol: Option<String>,
    #[serde(rename = "authenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "authorizerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_config: Option<AuthorizerConfig>,
    #[serde(rename = "clientCertificateConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_config: Option<ClientCertificateConfig>,
    #[serde(rename = "domainConfigurationName")]
    #[serde(default)]
    pub domain_configuration_name: String,
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "serverCertificateArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_arns: Option<Vec<String>>,
    #[serde(rename = "serverCertificateConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_config: Option<ServerCertificateConfig>,
    #[serde(rename = "serviceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "tlsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_config: Option<TlsConfig>,
    #[serde(rename = "validationCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_certificate_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthorizerConfig {
    #[serde(rename = "allowAuthorizerOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_authorizer_override: Option<bool>,
    #[serde(rename = "defaultAuthorizerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authorizer_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClientCertificateConfig {
    #[serde(rename = "clientCertificateCallbackArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_callback_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServerCertificateConfig {
    #[serde(rename = "enableOCSPCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_o_c_s_p_check: Option<bool>,
    #[serde(rename = "ocspAuthorizedResponderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocsp_authorized_responder_arn: Option<String>,
    #[serde(rename = "ocspLambdaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocsp_lambda_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TlsConfig {
    #[serde(rename = "securityPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDomainConfigurationResponse {
    #[serde(rename = "domainConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_configuration_arn: Option<String>,
    #[serde(rename = "domainConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_configuration_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDynamicThingGroupRequest {
    #[serde(rename = "indexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "queryString")]
    #[serde(default)]
    pub query_string: String,
    #[serde(rename = "queryVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "thingGroupName")]
    #[serde(default)]
    pub thing_group_name: String,
    #[serde(rename = "thingGroupProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_properties: Option<ThingGroupProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThingGroupProperties {
    #[serde(rename = "attributePayload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_payload: Option<AttributePayload>,
    #[serde(rename = "thingGroupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttributePayload {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDynamicThingGroupResponse {
    #[serde(rename = "indexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "queryString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
    #[serde(rename = "queryVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_version: Option<String>,
    #[serde(rename = "thingGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_arn: Option<String>,
    #[serde(rename = "thingGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_id: Option<String>,
    #[serde(rename = "thingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFleetMetricRequest {
    #[serde(rename = "aggregationField")]
    #[serde(default)]
    pub aggregation_field: String,
    #[serde(rename = "aggregationType")]
    #[serde(default)]
    pub aggregation_type: AggregationType,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "indexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "metricName")]
    #[serde(default)]
    pub metric_name: String,
    #[serde(default)]
    pub period: i32,
    #[serde(rename = "queryString")]
    #[serde(default)]
    pub query_string: String,
    #[serde(rename = "queryVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregationType {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFleetMetricResponse {
    #[serde(rename = "metricArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_arn: Option<String>,
    #[serde(rename = "metricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateJobRequest {
    #[serde(rename = "abortConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abort_config: Option<AbortConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "destinationPackageVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_package_versions: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    #[serde(rename = "documentParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "documentSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_source: Option<String>,
    #[serde(rename = "jobExecutionsRetryConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_executions_retry_config: Option<JobExecutionsRetryConfig>,
    #[serde(rename = "jobExecutionsRolloutConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_executions_rollout_config: Option<JobExecutionsRolloutConfig>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "jobTemplateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template_arn: Option<String>,
    #[serde(rename = "namespaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
    #[serde(rename = "presignedUrlConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presigned_url_config: Option<PresignedUrlConfig>,
    #[serde(rename = "schedulingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_config: Option<SchedulingConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "targetSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_selection: Option<String>,
    #[serde(default)]
    pub targets: Vec<String>,
    #[serde(rename = "timeoutConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_config: Option<TimeoutConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AbortConfig {
    #[serde(rename = "criteriaList")]
    #[serde(default)]
    pub criteria_list: Vec<AbortCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AbortCriteria {
    #[serde(default)]
    pub action: String,
    #[serde(rename = "failureType")]
    #[serde(default)]
    pub failure_type: String,
    #[serde(rename = "minNumberOfExecutedThings")]
    #[serde(default)]
    pub min_number_of_executed_things: i32,
    #[serde(rename = "thresholdPercentage")]
    #[serde(default)]
    pub threshold_percentage: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobExecutionsRetryConfig {
    #[serde(rename = "criteriaList")]
    #[serde(default)]
    pub criteria_list: Vec<RetryCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetryCriteria {
    #[serde(rename = "failureType")]
    #[serde(default)]
    pub failure_type: String,
    #[serde(rename = "numberOfRetries")]
    #[serde(default)]
    pub number_of_retries: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobExecutionsRolloutConfig {
    #[serde(rename = "exponentialRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exponential_rate: Option<ExponentialRolloutRate>,
    #[serde(rename = "maximumPerMinute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_per_minute: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExponentialRolloutRate {
    #[serde(rename = "baseRatePerMinute")]
    #[serde(default)]
    pub base_rate_per_minute: i32,
    #[serde(rename = "incrementFactor")]
    #[serde(default)]
    pub increment_factor: f64,
    #[serde(rename = "rateIncreaseCriteria")]
    #[serde(default)]
    pub rate_increase_criteria: RateIncreaseCriteria,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RateIncreaseCriteria {
    #[serde(rename = "numberOfNotifiedThings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_notified_things: Option<i32>,
    #[serde(rename = "numberOfSucceededThings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_succeeded_things: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PresignedUrlConfig {
    #[serde(rename = "expiresInSec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in_sec: Option<i64>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchedulingConfig {
    #[serde(rename = "endBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_behavior: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "maintenanceWindows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_windows: Option<Vec<MaintenanceWindow>>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaintenanceWindow {
    #[serde(rename = "durationInMinutes")]
    #[serde(default)]
    pub duration_in_minutes: i32,
    #[serde(rename = "startTime")]
    #[serde(default)]
    pub start_time: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeoutConfig {
    #[serde(rename = "inProgressTimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_progress_timeout_in_minutes: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateJobResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateJobTemplateRequest {
    #[serde(rename = "abortConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abort_config: Option<AbortConfig>,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "destinationPackageVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_package_versions: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    #[serde(rename = "documentSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_source: Option<String>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobExecutionsRetryConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_executions_retry_config: Option<JobExecutionsRetryConfig>,
    #[serde(rename = "jobExecutionsRolloutConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_executions_rollout_config: Option<JobExecutionsRolloutConfig>,
    #[serde(rename = "jobTemplateId")]
    #[serde(default)]
    pub job_template_id: String,
    #[serde(rename = "maintenanceWindows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_windows: Option<Vec<MaintenanceWindow>>,
    #[serde(rename = "presignedUrlConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presigned_url_config: Option<PresignedUrlConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "timeoutConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_config: Option<TimeoutConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateJobTemplateResponse {
    #[serde(rename = "jobTemplateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template_arn: Option<String>,
    #[serde(rename = "jobTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateKeysAndCertificateRequest {
    #[serde(rename = "setAsActive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_as_active: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateKeysAndCertificateResponse {
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "certificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    #[serde(rename = "certificatePem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_pem: Option<String>,
    #[serde(rename = "keyPair")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<KeyPair>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyPair {
    #[serde(rename = "PrivateKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(rename = "PublicKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMitigationActionRequest {
    #[serde(rename = "actionName")]
    #[serde(default)]
    pub action_name: String,
    #[serde(rename = "actionParams")]
    #[serde(default)]
    pub action_params: MitigationActionParams,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MitigationActionParams {
    #[serde(rename = "addThingsToThingGroupParams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_things_to_thing_group_params: Option<AddThingsToThingGroupParams>,
    #[serde(rename = "enableIoTLoggingParams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_io_t_logging_params: Option<EnableIoTLoggingParams>,
    #[serde(rename = "publishFindingToSnsParams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_finding_to_sns_params: Option<PublishFindingToSnsParams>,
    #[serde(rename = "replaceDefaultPolicyVersionParams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_default_policy_version_params: Option<ReplaceDefaultPolicyVersionParams>,
    #[serde(rename = "updateCACertificateParams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_c_a_certificate_params: Option<UpdateCACertificateParams>,
    #[serde(rename = "updateDeviceCertificateParams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_device_certificate_params: Option<UpdateDeviceCertificateParams>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddThingsToThingGroupParams {
    #[serde(rename = "overrideDynamicGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_dynamic_groups: Option<bool>,
    #[serde(rename = "thingGroupNames")]
    #[serde(default)]
    pub thing_group_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableIoTLoggingParams {
    #[serde(rename = "logLevel")]
    #[serde(default)]
    pub log_level: String,
    #[serde(rename = "roleArnForLogging")]
    #[serde(default)]
    pub role_arn_for_logging: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublishFindingToSnsParams {
    #[serde(rename = "topicArn")]
    #[serde(default)]
    pub topic_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplaceDefaultPolicyVersionParams {
    #[serde(rename = "templateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCACertificateParams {
    #[serde(default)]
    pub action: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDeviceCertificateParams {
    #[serde(default)]
    pub action: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMitigationActionResponse {
    #[serde(rename = "actionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_arn: Option<String>,
    #[serde(rename = "actionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOTAUpdateRequest {
    #[serde(rename = "additionalParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "awsJobAbortConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_job_abort_config: Option<AwsJobAbortConfig>,
    #[serde(rename = "awsJobExecutionsRolloutConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_job_executions_rollout_config: Option<AwsJobExecutionsRolloutConfig>,
    #[serde(rename = "awsJobPresignedUrlConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_job_presigned_url_config: Option<AwsJobPresignedUrlConfig>,
    #[serde(rename = "awsJobTimeoutConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_job_timeout_config: Option<AwsJobTimeoutConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub files: Vec<OTAUpdateFile>,
    #[serde(rename = "otaUpdateId")]
    #[serde(default)]
    pub ota_update_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "targetSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_selection: Option<String>,
    #[serde(default)]
    pub targets: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsJobAbortConfig {
    #[serde(rename = "abortCriteriaList")]
    #[serde(default)]
    pub abort_criteria_list: Vec<AwsJobAbortCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsJobAbortCriteria {
    #[serde(default)]
    pub action: String,
    #[serde(rename = "failureType")]
    #[serde(default)]
    pub failure_type: String,
    #[serde(rename = "minNumberOfExecutedThings")]
    #[serde(default)]
    pub min_number_of_executed_things: i32,
    #[serde(rename = "thresholdPercentage")]
    #[serde(default)]
    pub threshold_percentage: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsJobExecutionsRolloutConfig {
    #[serde(rename = "exponentialRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exponential_rate: Option<AwsJobExponentialRolloutRate>,
    #[serde(rename = "maximumPerMinute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_per_minute: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsJobExponentialRolloutRate {
    #[serde(rename = "baseRatePerMinute")]
    #[serde(default)]
    pub base_rate_per_minute: i32,
    #[serde(rename = "incrementFactor")]
    #[serde(default)]
    pub increment_factor: f64,
    #[serde(rename = "rateIncreaseCriteria")]
    #[serde(default)]
    pub rate_increase_criteria: AwsJobRateIncreaseCriteria,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsJobRateIncreaseCriteria {
    #[serde(rename = "numberOfNotifiedThings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_notified_things: Option<i32>,
    #[serde(rename = "numberOfSucceededThings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_succeeded_things: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsJobPresignedUrlConfig {
    #[serde(rename = "expiresInSec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in_sec: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsJobTimeoutConfig {
    #[serde(rename = "inProgressTimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_progress_timeout_in_minutes: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OTAUpdateFile {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "codeSigning")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_signing: Option<CodeSigning>,
    #[serde(rename = "fileLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_location: Option<FileLocation>,
    #[serde(rename = "fileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "fileType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<i32>,
    #[serde(rename = "fileVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeSigning {
    #[serde(rename = "awsSignerJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_signer_job_id: Option<String>,
    #[serde(rename = "customCodeSigning")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_code_signing: Option<CustomCodeSigning>,
    #[serde(rename = "startSigningJobParameter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_signing_job_parameter: Option<StartSigningJobParameter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomCodeSigning {
    #[serde(rename = "certificateChain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<CodeSigningCertificateChain>,
    #[serde(rename = "hashAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_algorithm: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<CodeSigningSignature>,
    #[serde(rename = "signatureAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_algorithm: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeSigningCertificateChain {
    #[serde(rename = "certificateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_name: Option<String>,
    #[serde(rename = "inlineDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_document: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeSigningSignature {
    #[serde(rename = "inlineDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_document: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSigningJobParameter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Destination>,
    #[serde(rename = "signingProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_profile_name: Option<String>,
    #[serde(rename = "signingProfileParameter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_profile_parameter: Option<SigningProfileParameter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Destination {
    #[serde(rename = "s3Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination: Option<S3Destination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Destination {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SigningProfileParameter {
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "certificatePathOnDevice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_path_on_device: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileLocation {
    #[serde(rename = "s3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_location: Option<S3Location>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<Stream>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Stream {
    #[serde(rename = "fileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<i32>,
    #[serde(rename = "streamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOTAUpdateResponse {
    #[serde(rename = "awsIotJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iot_job_arn: Option<String>,
    #[serde(rename = "awsIotJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iot_job_id: Option<String>,
    #[serde(rename = "otaUpdateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update_arn: Option<String>,
    #[serde(rename = "otaUpdateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update_id: Option<String>,
    #[serde(rename = "otaUpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePackageRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "packageName")]
    #[serde(default)]
    pub package_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePackageResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "packageArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_arn: Option<String>,
    #[serde(rename = "packageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePackageVersionRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<PackageVersionArtifact>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "packageName")]
    #[serde(default)]
    pub package_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "versionName")]
    #[serde(default)]
    pub version_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageVersionArtifact {
    #[serde(rename = "s3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_location: Option<S3Location>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePackageVersionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "errorReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<String>,
    #[serde(rename = "packageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(rename = "packageVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "versionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePolicyRequest {
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    pub policy_document: String,
    #[serde(rename = "policyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePolicyResponse {
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    #[serde(rename = "policyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "policyVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePolicyVersionRequest {
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    pub policy_document: String,
    #[serde(rename = "policyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(rename = "setAsDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_as_default: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePolicyVersionResponse {
    #[serde(rename = "isDefaultVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_version: Option<bool>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    #[serde(rename = "policyVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProvisioningClaimRequest {
    #[serde(rename = "templateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProvisioningClaimResponse {
    #[serde(rename = "certificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    #[serde(rename = "certificatePem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_pem: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<f64>,
    #[serde(rename = "keyPair")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<KeyPair>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProvisioningTemplateRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "preProvisioningHook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_provisioning_hook: Option<ProvisioningHook>,
    #[serde(rename = "provisioningRoleArn")]
    #[serde(default)]
    pub provisioning_role_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "templateBody")]
    #[serde(default)]
    pub template_body: String,
    #[serde(rename = "templateName")]
    #[serde(default)]
    pub template_name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisioningHook {
    #[serde(rename = "payloadVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_version: Option<String>,
    #[serde(rename = "targetArn")]
    #[serde(default)]
    pub target_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProvisioningTemplateResponse {
    #[serde(rename = "defaultVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version_id: Option<i32>,
    #[serde(rename = "templateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_arn: Option<String>,
    #[serde(rename = "templateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProvisioningTemplateVersionRequest {
    #[serde(rename = "setAsDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_as_default: Option<bool>,
    #[serde(rename = "templateBody")]
    #[serde(default)]
    pub template_body: String,
    #[serde(rename = "templateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProvisioningTemplateVersionResponse {
    #[serde(rename = "isDefaultVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_version: Option<bool>,
    #[serde(rename = "templateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_arn: Option<String>,
    #[serde(rename = "templateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "versionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRoleAliasRequest {
    #[serde(rename = "credentialDurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_duration_seconds: Option<i32>,
    #[serde(rename = "roleAlias")]
    #[serde(default)]
    pub role_alias: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRoleAliasResponse {
    #[serde(rename = "roleAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_alias: Option<String>,
    #[serde(rename = "roleAliasArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_alias_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateScheduledAuditRequest {
    #[serde(rename = "dayOfMonth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_month: Option<String>,
    #[serde(rename = "dayOfWeek")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<String>,
    #[serde(default)]
    pub frequency: String,
    #[serde(rename = "scheduledAuditName")]
    #[serde(default)]
    pub scheduled_audit_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "targetCheckNames")]
    #[serde(default)]
    pub target_check_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateScheduledAuditResponse {
    #[serde(rename = "scheduledAuditArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_audit_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSecurityProfileRequest {
    #[serde(rename = "additionalMetricsToRetain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_metrics_to_retain: Option<Vec<String>>,
    #[serde(rename = "additionalMetricsToRetainV2")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_metrics_to_retain_v2: Option<Vec<MetricToRetain>>,
    #[serde(rename = "alertTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_targets: Option<std::collections::HashMap<String, AlertTarget>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behaviors: Option<Vec<Behavior>>,
    #[serde(rename = "metricsExportConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_export_config: Option<MetricsExportConfig>,
    #[serde(rename = "securityProfileDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_description: Option<String>,
    #[serde(rename = "securityProfileName")]
    #[serde(default)]
    pub security_profile_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricToRetain {
    #[serde(rename = "exportMetric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_metric: Option<bool>,
    #[serde(default)]
    pub metric: String,
    #[serde(rename = "metricDimension")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_dimension: Option<MetricDimension>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricDimension {
    #[serde(rename = "dimensionName")]
    #[serde(default)]
    pub dimension_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AlertTarget {
    #[serde(rename = "alertTargetArn")]
    #[serde(default)]
    pub alert_target_arn: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Behavior {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criteria: Option<BehaviorCriteria>,
    #[serde(rename = "exportMetric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_metric: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
    #[serde(rename = "metricDimension")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_dimension: Option<MetricDimension>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "suppressAlerts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_alerts: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BehaviorCriteria {
    #[serde(rename = "comparisonOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    #[serde(rename = "consecutiveDatapointsToAlarm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consecutive_datapoints_to_alarm: Option<i32>,
    #[serde(rename = "consecutiveDatapointsToClear")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consecutive_datapoints_to_clear: Option<i32>,
    #[serde(rename = "durationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
    #[serde(rename = "mlDetectionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_detection_config: Option<MachineLearningDetectionConfig>,
    #[serde(rename = "statisticalThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistical_threshold: Option<StatisticalThreshold>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<MetricValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MachineLearningDetectionConfig {
    #[serde(rename = "confidenceLevel")]
    #[serde(default)]
    pub confidence_level: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatisticalThreshold {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricValue {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidrs: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numbers: Option<Vec<f64>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<i32>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricsExportConfig {
    #[serde(rename = "mqttTopic")]
    #[serde(default)]
    pub mqtt_topic: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSecurityProfileResponse {
    #[serde(rename = "securityProfileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_arn: Option<String>,
    #[serde(rename = "securityProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateStreamRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub files: Vec<StreamFile>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "streamId")]
    #[serde(default)]
    pub stream_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamFile {
    #[serde(rename = "fileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<i32>,
    #[serde(rename = "s3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_location: Option<S3Location>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateStreamResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "streamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    #[serde(rename = "streamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "streamVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateThingGroupRequest {
    #[serde(rename = "parentGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "thingGroupName")]
    #[serde(default)]
    pub thing_group_name: String,
    #[serde(rename = "thingGroupProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_properties: Option<ThingGroupProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateThingGroupResponse {
    #[serde(rename = "thingGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_arn: Option<String>,
    #[serde(rename = "thingGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_id: Option<String>,
    #[serde(rename = "thingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateThingRequest {
    #[serde(rename = "attributePayload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_payload: Option<AttributePayload>,
    #[serde(rename = "billingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_group_name: Option<String>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    pub thing_name: String,
    #[serde(rename = "thingTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateThingResponse {
    #[serde(rename = "thingArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_arn: Option<String>,
    #[serde(rename = "thingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_id: Option<String>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateThingTypeRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "thingTypeName")]
    #[serde(default)]
    pub thing_type_name: String,
    #[serde(rename = "thingTypeProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_properties: Option<ThingTypeProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThingTypeProperties {
    #[serde(rename = "mqtt5Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mqtt5_configuration: Option<Mqtt5Configuration>,
    #[serde(rename = "searchableAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable_attributes: Option<Vec<String>>,
    #[serde(rename = "thingTypeDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Mqtt5Configuration {
    #[serde(rename = "propagatingAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagating_attributes: Option<Vec<PropagatingAttribute>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PropagatingAttribute {
    #[serde(rename = "connectionAttribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_attribute: Option<String>,
    #[serde(rename = "thingAttribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_attribute: Option<String>,
    #[serde(rename = "userPropertyKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_property_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateThingTypeResponse {
    #[serde(rename = "thingTypeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_arn: Option<String>,
    #[serde(rename = "thingTypeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_id: Option<String>,
    #[serde(rename = "thingTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTopicRuleDestinationRequest {
    #[serde(rename = "destinationConfiguration")]
    #[serde(default)]
    pub destination_configuration: TopicRuleDestinationConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicRuleDestinationConfiguration {
    #[serde(rename = "httpUrlConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_url_configuration: Option<HttpUrlDestinationConfiguration>,
    #[serde(rename = "vpcConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<VpcDestinationConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpUrlDestinationConfiguration {
    #[serde(rename = "confirmationUrl")]
    #[serde(default)]
    pub confirmation_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcDestinationConfiguration {
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    pub vpc_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTopicRuleDestinationResponse {
    #[serde(rename = "topicRuleDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_rule_destination: Option<TopicRuleDestination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicRuleDestination {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "httpUrlProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_url_properties: Option<HttpUrlDestinationProperties>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "vpcProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_properties: Option<VpcDestinationProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpUrlDestinationProperties {
    #[serde(rename = "confirmationUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcDestinationProperties {
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTopicRuleRequest {
    #[serde(rename = "ruleName")]
    #[serde(default)]
    pub rule_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(rename = "topicRulePayload")]
    #[serde(default)]
    pub topic_rule_payload: TopicRulePayload,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicRulePayload {
    #[serde(default)]
    pub actions: Vec<Action>,
    #[serde(rename = "awsIotSqlVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iot_sql_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "errorAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_action: Option<Action>,
    #[serde(rename = "ruleDisabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_disabled: Option<bool>,
    #[serde(default)]
    pub sql: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Action {
    #[serde(rename = "cloudwatchAlarm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch_alarm: Option<CloudwatchAlarmAction>,
    #[serde(rename = "cloudwatchLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch_logs: Option<CloudwatchLogsAction>,
    #[serde(rename = "cloudwatchMetric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch_metric: Option<CloudwatchMetricAction>,
    #[serde(rename = "dynamoDB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_d_b: Option<DynamoDBAction>,
    #[serde(rename = "dynamoDBv2")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_d_bv2: Option<DynamoDBv2Action>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch: Option<ElasticsearchAction>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firehose: Option<FirehoseAction>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HttpAction>,
    #[serde(rename = "iotAnalytics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_analytics: Option<IotAnalyticsAction>,
    #[serde(rename = "iotEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_events: Option<IotEventsAction>,
    #[serde(rename = "iotSiteWise")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_site_wise: Option<IotSiteWiseAction>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka: Option<KafkaAction>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis: Option<KinesisAction>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda: Option<LambdaAction>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationAction>,
    #[serde(rename = "openSearch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_search: Option<OpenSearchAction>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub republish: Option<RepublishAction>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3Action>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salesforce: Option<SalesforceAction>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns: Option<SnsAction>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs: Option<SqsAction>,
    #[serde(rename = "stepFunctions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_functions: Option<StepFunctionsAction>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestream: Option<TimestreamAction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudwatchAlarmAction {
    #[serde(rename = "alarmName")]
    #[serde(default)]
    pub alarm_name: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "stateReason")]
    #[serde(default)]
    pub state_reason: String,
    #[serde(rename = "stateValue")]
    #[serde(default)]
    pub state_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudwatchLogsAction {
    #[serde(rename = "batchMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_mode: Option<bool>,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    pub log_group_name: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudwatchMetricAction {
    #[serde(rename = "metricName")]
    #[serde(default)]
    pub metric_name: String,
    #[serde(rename = "metricNamespace")]
    #[serde(default)]
    pub metric_namespace: String,
    #[serde(rename = "metricTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_timestamp: Option<String>,
    #[serde(rename = "metricUnit")]
    #[serde(default)]
    pub metric_unit: String,
    #[serde(rename = "metricValue")]
    #[serde(default)]
    pub metric_value: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DynamoDBAction {
    #[serde(rename = "hashKeyField")]
    #[serde(default)]
    pub hash_key_field: String,
    #[serde(rename = "hashKeyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_key_type: Option<String>,
    #[serde(rename = "hashKeyValue")]
    #[serde(default)]
    pub hash_key_value: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "payloadField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_field: Option<String>,
    #[serde(rename = "rangeKeyField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_key_field: Option<String>,
    #[serde(rename = "rangeKeyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_key_type: Option<String>,
    #[serde(rename = "rangeKeyValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_key_value: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "tableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DynamoDBv2Action {
    #[serde(rename = "putItem")]
    #[serde(default)]
    pub put_item: PutItemInput,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutItemInput {
    #[serde(rename = "tableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ElasticsearchAction {
    #[serde(default)]
    pub endpoint: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub index: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirehoseAction {
    #[serde(rename = "batchMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_mode: Option<bool>,
    #[serde(rename = "deliveryStreamName")]
    #[serde(default)]
    pub delivery_stream_name: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpAction {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<HttpAuthorization>,
    #[serde(rename = "batchConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_config: Option<BatchConfig>,
    #[serde(rename = "confirmationUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_url: Option<String>,
    #[serde(rename = "enableBatching")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_batching: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<HttpActionHeader>>,
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpAuthorization {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sigv4: Option<SigV4Authorization>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SigV4Authorization {
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    pub service_name: String,
    #[serde(rename = "signingRegion")]
    #[serde(default)]
    pub signing_region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchConfig {
    #[serde(rename = "batchAcrossTopics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_across_topics: Option<bool>,
    #[serde(rename = "maxBatchOpenMs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_open_ms: Option<i32>,
    #[serde(rename = "maxBatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_size: Option<i32>,
    #[serde(rename = "maxBatchSizeBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_size_bytes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpActionHeader {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IotAnalyticsAction {
    #[serde(rename = "batchMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_mode: Option<bool>,
    #[serde(rename = "channelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_arn: Option<String>,
    #[serde(rename = "channelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IotEventsAction {
    #[serde(rename = "batchMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_mode: Option<bool>,
    #[serde(rename = "inputName")]
    #[serde(default)]
    pub input_name: String,
    #[serde(rename = "messageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IotSiteWiseAction {
    #[serde(rename = "putAssetPropertyValueEntries")]
    #[serde(default)]
    pub put_asset_property_value_entries: Vec<PutAssetPropertyValueEntry>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAssetPropertyValueEntry {
    #[serde(rename = "assetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "entryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_id: Option<String>,
    #[serde(rename = "propertyAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_alias: Option<String>,
    #[serde(rename = "propertyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_id: Option<String>,
    #[serde(rename = "propertyValues")]
    #[serde(default)]
    pub property_values: Vec<AssetPropertyValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetPropertyValue {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
    #[serde(default)]
    pub timestamp: AssetPropertyTimestamp,
    #[serde(default)]
    pub value: AssetPropertyVariant,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetPropertyTimestamp {
    #[serde(rename = "offsetInNanos")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_in_nanos: Option<String>,
    #[serde(rename = "timeInSeconds")]
    #[serde(default)]
    pub time_in_seconds: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetPropertyVariant {
    #[serde(rename = "booleanValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_value: Option<String>,
    #[serde(rename = "doubleValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_value: Option<String>,
    #[serde(rename = "integerValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_value: Option<String>,
    #[serde(rename = "stringValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KafkaAction {
    #[serde(rename = "clientProperties")]
    #[serde(default)]
    pub client_properties: std::collections::HashMap<String, String>,
    #[serde(rename = "destinationArn")]
    #[serde(default)]
    pub destination_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<KafkaActionHeader>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    #[serde(default)]
    pub topic: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KafkaActionHeader {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisAction {
    #[serde(rename = "partitionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "streamName")]
    #[serde(default)]
    pub stream_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaAction {
    #[serde(rename = "functionArn")]
    #[serde(default)]
    pub function_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LocationAction {
    #[serde(rename = "deviceId")]
    #[serde(default)]
    pub device_id: String,
    #[serde(default)]
    pub latitude: String,
    #[serde(default)]
    pub longitude: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<LocationTimestamp>,
    #[serde(rename = "trackerName")]
    #[serde(default)]
    pub tracker_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LocationTimestamp {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenSearchAction {
    #[serde(default)]
    pub endpoint: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub index: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RepublishAction {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<MqttHeaders>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos: Option<i32>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(default)]
    pub topic: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MqttHeaders {
    #[serde(rename = "contentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "correlationData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_data: Option<String>,
    #[serde(rename = "messageExpiry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_expiry: Option<String>,
    #[serde(rename = "payloadFormatIndicator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_format_indicator: Option<String>,
    #[serde(rename = "responseTopic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_topic: Option<String>,
    #[serde(rename = "userProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_properties: Option<Vec<UserProperty>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserProperty {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Action {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "cannedAcl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canned_acl: Option<String>,
    #[serde(default)]
    pub key: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SalesforceAction {
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnsAction {
    #[serde(rename = "messageFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_format: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "targetArn")]
    #[serde(default)]
    pub target_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SqsAction {
    #[serde(rename = "queueUrl")]
    #[serde(default)]
    pub queue_url: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "useBase64")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_base64: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StepFunctionsAction {
    #[serde(rename = "executionNamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_name_prefix: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "stateMachineName")]
    #[serde(default)]
    pub state_machine_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimestreamAction {
    #[serde(rename = "databaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(default)]
    pub dimensions: Vec<TimestreamDimension>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "tableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<TimestreamTimestamp>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimestreamDimension {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimestreamTimestamp {
    #[serde(default)]
    pub unit: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccountAuditConfigurationRequest {
    #[serde(rename = "deleteScheduledAudits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_scheduled_audits: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccountAuditConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAuditSuppressionRequest {
    #[serde(rename = "checkName")]
    #[serde(default)]
    pub check_name: String,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    pub resource_identifier: ResourceIdentifier,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAuditSuppressionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAuthorizerRequest {
    #[serde(rename = "authorizerName")]
    #[serde(default)]
    pub authorizer_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAuthorizerResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBillingGroupRequest {
    #[serde(rename = "billingGroupName")]
    #[serde(default)]
    pub billing_group_name: String,
    #[serde(rename = "expectedVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBillingGroupResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCACertificateRequest {
    #[serde(rename = "certificateId")]
    #[serde(default)]
    pub certificate_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCACertificateResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCertificateProviderRequest {
    #[serde(rename = "certificateProviderName")]
    #[serde(default)]
    pub certificate_provider_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCertificateProviderResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCertificateRequest {
    #[serde(rename = "certificateId")]
    #[serde(default)]
    pub certificate_id: String,
    #[serde(rename = "forceDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCommandExecutionRequest {
    #[serde(rename = "executionId")]
    #[serde(default)]
    pub execution_id: String,
    #[serde(rename = "targetArn")]
    #[serde(default)]
    pub target_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCommandExecutionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCommandRequest {
    #[serde(rename = "commandId")]
    #[serde(default)]
    pub command_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCommandResponse {
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCustomMetricRequest {
    #[serde(rename = "metricName")]
    #[serde(default)]
    pub metric_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCustomMetricResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDimensionRequest {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDimensionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDomainConfigurationRequest {
    #[serde(rename = "domainConfigurationName")]
    #[serde(default)]
    pub domain_configuration_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDomainConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDynamicThingGroupRequest {
    #[serde(rename = "expectedVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_version: Option<i64>,
    #[serde(rename = "thingGroupName")]
    #[serde(default)]
    pub thing_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDynamicThingGroupResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFleetMetricRequest {
    #[serde(rename = "expectedVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_version: Option<i64>,
    #[serde(rename = "metricName")]
    #[serde(default)]
    pub metric_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteJobExecutionRequest {
    #[serde(rename = "executionNumber")]
    #[serde(default)]
    pub execution_number: i64,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "namespaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    pub thing_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteJobRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "namespaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteJobTemplateRequest {
    #[serde(rename = "jobTemplateId")]
    #[serde(default)]
    pub job_template_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMitigationActionRequest {
    #[serde(rename = "actionName")]
    #[serde(default)]
    pub action_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMitigationActionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteOTAUpdateRequest {
    #[serde(rename = "deleteStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_stream: Option<bool>,
    #[serde(rename = "forceDeleteAWSJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete_a_w_s_job: Option<bool>,
    #[serde(rename = "otaUpdateId")]
    #[serde(default)]
    pub ota_update_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteOTAUpdateResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePackageRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "packageName")]
    #[serde(default)]
    pub package_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePackageResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePackageVersionRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "packageName")]
    #[serde(default)]
    pub package_name: String,
    #[serde(rename = "versionName")]
    #[serde(default)]
    pub version_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePackageVersionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePolicyRequest {
    #[serde(rename = "policyName")]
    #[serde(default)]
    pub policy_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePolicyVersionRequest {
    #[serde(rename = "policyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(rename = "policyVersionId")]
    #[serde(default)]
    pub policy_version_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProvisioningTemplateRequest {
    #[serde(rename = "templateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProvisioningTemplateResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProvisioningTemplateVersionRequest {
    #[serde(rename = "templateName")]
    #[serde(default)]
    pub template_name: String,
    #[serde(rename = "versionId")]
    #[serde(default)]
    pub version_id: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProvisioningTemplateVersionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRegistrationCodeRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRegistrationCodeResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRoleAliasRequest {
    #[serde(rename = "roleAlias")]
    #[serde(default)]
    pub role_alias: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRoleAliasResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScheduledAuditRequest {
    #[serde(rename = "scheduledAuditName")]
    #[serde(default)]
    pub scheduled_audit_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteScheduledAuditResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSecurityProfileRequest {
    #[serde(rename = "expectedVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_version: Option<i64>,
    #[serde(rename = "securityProfileName")]
    #[serde(default)]
    pub security_profile_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSecurityProfileResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStreamRequest {
    #[serde(rename = "streamId")]
    #[serde(default)]
    pub stream_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStreamResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteThingGroupRequest {
    #[serde(rename = "expectedVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_version: Option<i64>,
    #[serde(rename = "thingGroupName")]
    #[serde(default)]
    pub thing_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteThingGroupResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteThingRequest {
    #[serde(rename = "expectedVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_version: Option<i64>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    pub thing_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteThingResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteThingTypeRequest {
    #[serde(rename = "thingTypeName")]
    #[serde(default)]
    pub thing_type_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteThingTypeResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTopicRuleDestinationRequest {
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTopicRuleDestinationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTopicRuleRequest {
    #[serde(rename = "ruleName")]
    #[serde(default)]
    pub rule_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteV2LoggingLevelRequest {
    #[serde(rename = "targetName")]
    #[serde(default)]
    pub target_name: String,
    #[serde(rename = "targetType")]
    #[serde(default)]
    pub target_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeprecateThingTypeRequest {
    #[serde(rename = "thingTypeName")]
    #[serde(default)]
    pub thing_type_name: String,
    #[serde(rename = "undoDeprecate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undo_deprecate: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeprecateThingTypeResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountAuditConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountAuditConfigurationResponse {
    #[serde(rename = "auditCheckConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_check_configurations:
        Option<std::collections::HashMap<String, AuditCheckConfiguration>>,
    #[serde(rename = "auditNotificationTargetConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_notification_target_configurations:
        Option<std::collections::HashMap<String, AuditNotificationTarget>>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditCheckConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditNotificationTarget {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "targetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAuditFindingRequest {
    #[serde(rename = "findingId")]
    #[serde(default)]
    pub finding_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAuditFindingResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding: Option<AuditFinding>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditFinding {
    #[serde(rename = "checkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_name: Option<String>,
    #[serde(rename = "findingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_id: Option<String>,
    #[serde(rename = "findingTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_time: Option<f64>,
    #[serde(rename = "isSuppressed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_suppressed: Option<bool>,
    #[serde(rename = "nonCompliantResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_resource: Option<NonCompliantResource>,
    #[serde(rename = "reasonForNonCompliance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_for_non_compliance: Option<String>,
    #[serde(rename = "reasonForNonComplianceCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_for_non_compliance_code: Option<String>,
    #[serde(rename = "relatedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_resources: Option<Vec<RelatedResource>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(rename = "taskStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NonCompliantResource {
    #[serde(rename = "additionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<ResourceIdentifier>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RelatedResource {
    #[serde(rename = "additionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<ResourceIdentifier>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAuditMitigationActionsTaskRequest {
    #[serde(rename = "taskId")]
    #[serde(default)]
    pub task_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAuditMitigationActionsTaskResponse {
    #[serde(rename = "actionsDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_definition: Option<Vec<MitigationAction>>,
    #[serde(rename = "auditCheckToActionsMapping")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_check_to_actions_mapping: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<AuditMitigationActionsTaskTarget>,
    #[serde(rename = "taskStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_statistics: Option<std::collections::HashMap<String, TaskStatisticsForAuditCheck>>,
    #[serde(rename = "taskStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MitigationAction {
    #[serde(rename = "actionParams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_params: Option<MitigationActionParams>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditMitigationActionsTaskTarget {
    #[serde(rename = "auditCheckToReasonCodeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_check_to_reason_code_filter: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "auditTaskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_task_id: Option<String>,
    #[serde(rename = "findingIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskStatisticsForAuditCheck {
    #[serde(rename = "canceledFindingsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_findings_count: Option<i64>,
    #[serde(rename = "failedFindingsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_findings_count: Option<i64>,
    #[serde(rename = "skippedFindingsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped_findings_count: Option<i64>,
    #[serde(rename = "succeededFindingsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded_findings_count: Option<i64>,
    #[serde(rename = "totalFindingsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_findings_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAuditSuppressionRequest {
    #[serde(rename = "checkName")]
    #[serde(default)]
    pub check_name: String,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    pub resource_identifier: ResourceIdentifier,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAuditSuppressionResponse {
    #[serde(rename = "checkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "expirationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<ResourceIdentifier>,
    #[serde(rename = "suppressIndefinitely")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_indefinitely: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAuditTaskRequest {
    #[serde(rename = "taskId")]
    #[serde(default)]
    pub task_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAuditTaskResponse {
    #[serde(rename = "auditDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_details: Option<std::collections::HashMap<String, AuditCheckDetails>>,
    #[serde(rename = "scheduledAuditName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_audit_name: Option<String>,
    #[serde(rename = "taskStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_start_time: Option<f64>,
    #[serde(rename = "taskStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_statistics: Option<TaskStatistics>,
    #[serde(rename = "taskStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
    #[serde(rename = "taskType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditCheckDetails {
    #[serde(rename = "checkCompliant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_compliant: Option<bool>,
    #[serde(rename = "checkRunStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_run_status: Option<String>,
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "nonCompliantResourcesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_resources_count: Option<i64>,
    #[serde(rename = "suppressedNonCompliantResourcesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressed_non_compliant_resources_count: Option<i64>,
    #[serde(rename = "totalResourcesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_resources_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskStatistics {
    #[serde(rename = "canceledChecks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_checks: Option<i32>,
    #[serde(rename = "compliantChecks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_checks: Option<i32>,
    #[serde(rename = "failedChecks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_checks: Option<i32>,
    #[serde(rename = "inProgressChecks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_progress_checks: Option<i32>,
    #[serde(rename = "nonCompliantChecks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_checks: Option<i32>,
    #[serde(rename = "totalChecks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_checks: Option<i32>,
    #[serde(rename = "waitingForDataCollectionChecks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waiting_for_data_collection_checks: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAuthorizerRequest {
    #[serde(rename = "authorizerName")]
    #[serde(default)]
    pub authorizer_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAuthorizerResponse {
    #[serde(rename = "authorizerDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_description: Option<AuthorizerDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthorizerDescription {
    #[serde(rename = "authorizerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_arn: Option<String>,
    #[serde(rename = "authorizerFunctionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_function_arn: Option<String>,
    #[serde(rename = "authorizerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_name: Option<String>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "enableCachingForHttp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_caching_for_http: Option<bool>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "signingDisabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_disabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "tokenKeyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key_name: Option<String>,
    #[serde(rename = "tokenSigningPublicKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_signing_public_keys: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBillingGroupRequest {
    #[serde(rename = "billingGroupName")]
    #[serde(default)]
    pub billing_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBillingGroupResponse {
    #[serde(rename = "billingGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_group_arn: Option<String>,
    #[serde(rename = "billingGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_group_id: Option<String>,
    #[serde(rename = "billingGroupMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_group_metadata: Option<BillingGroupMetadata>,
    #[serde(rename = "billingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_group_name: Option<String>,
    #[serde(rename = "billingGroupProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_group_properties: Option<BillingGroupProperties>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BillingGroupMetadata {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCACertificateRequest {
    #[serde(rename = "certificateId")]
    #[serde(default)]
    pub certificate_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCACertificateResponse {
    #[serde(rename = "certificateDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_description: Option<CACertificateDescription>,
    #[serde(rename = "registrationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_config: Option<RegistrationConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CACertificateDescription {
    #[serde(rename = "autoRegistrationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_registration_status: Option<String>,
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "certificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    #[serde(rename = "certificateMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_mode: Option<String>,
    #[serde(rename = "certificatePem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_pem: Option<String>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "customerVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_version: Option<i32>,
    #[serde(rename = "generationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_id: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "ownedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity: Option<CertificateValidity>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CertificateValidity {
    #[serde(rename = "notAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_after: Option<f64>,
    #[serde(rename = "notBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegistrationConfig {
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "templateBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    #[serde(rename = "templateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCertificateProviderRequest {
    #[serde(rename = "certificateProviderName")]
    #[serde(default)]
    pub certificate_provider_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCertificateProviderResponse {
    #[serde(rename = "accountDefaultForOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_default_for_operations: Option<Vec<String>>,
    #[serde(rename = "certificateProviderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_provider_arn: Option<String>,
    #[serde(rename = "certificateProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_provider_name: Option<String>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "lambdaFunctionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_arn: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCertificateRequest {
    #[serde(rename = "certificateId")]
    #[serde(default)]
    pub certificate_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCertificateResponse {
    #[serde(rename = "certificateDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_description: Option<CertificateDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CertificateDescription {
    #[serde(rename = "caCertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_id: Option<String>,
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "certificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    #[serde(rename = "certificateMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_mode: Option<String>,
    #[serde(rename = "certificatePem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_pem: Option<String>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "customerVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_version: Option<i32>,
    #[serde(rename = "generationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_id: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "ownedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_by: Option<String>,
    #[serde(rename = "previousOwnedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_owned_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "transferData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferData>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity: Option<CertificateValidity>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransferData {
    #[serde(rename = "acceptDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_date: Option<f64>,
    #[serde(rename = "rejectDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_date: Option<f64>,
    #[serde(rename = "rejectReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
    #[serde(rename = "transferDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_date: Option<f64>,
    #[serde(rename = "transferMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCustomMetricRequest {
    #[serde(rename = "metricName")]
    #[serde(default)]
    pub metric_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCustomMetricResponse {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "metricArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_arn: Option<String>,
    #[serde(rename = "metricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "metricType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDefaultAuthorizerRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDefaultAuthorizerResponse {
    #[serde(rename = "authorizerDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_description: Option<AuthorizerDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDetectMitigationActionsTaskRequest {
    #[serde(rename = "taskId")]
    #[serde(default)]
    pub task_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDetectMitigationActionsTaskResponse {
    #[serde(rename = "taskSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_summary: Option<DetectMitigationActionsTaskSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectMitigationActionsTaskSummary {
    #[serde(rename = "actionsDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_definition: Option<Vec<MitigationAction>>,
    #[serde(rename = "onlyActiveViolationsIncluded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_active_violations_included: Option<bool>,
    #[serde(rename = "suppressedAlertsIncluded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressed_alerts_included: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<DetectMitigationActionsTaskTarget>,
    #[serde(rename = "taskEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_end_time: Option<f64>,
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(rename = "taskStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_start_time: Option<f64>,
    #[serde(rename = "taskStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_statistics: Option<DetectMitigationActionsTaskStatistics>,
    #[serde(rename = "taskStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
    #[serde(rename = "violationEventOccurrenceRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_event_occurrence_range: Option<ViolationEventOccurrenceRange>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectMitigationActionsTaskTarget {
    #[serde(rename = "behaviorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior_name: Option<String>,
    #[serde(rename = "securityProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_name: Option<String>,
    #[serde(rename = "violationIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectMitigationActionsTaskStatistics {
    #[serde(rename = "actionsExecuted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_executed: Option<i64>,
    #[serde(rename = "actionsFailed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_failed: Option<i64>,
    #[serde(rename = "actionsSkipped")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_skipped: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ViolationEventOccurrenceRange {
    #[serde(rename = "endTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "startTime")]
    #[serde(default)]
    pub start_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDimensionRequest {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDimensionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "stringValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_values: Option<Vec<String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDomainConfigurationRequest {
    #[serde(rename = "domainConfigurationName")]
    #[serde(default)]
    pub domain_configuration_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDomainConfigurationResponse {
    #[serde(rename = "applicationProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_protocol: Option<String>,
    #[serde(rename = "authenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "authorizerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_config: Option<AuthorizerConfig>,
    #[serde(rename = "clientCertificateConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_config: Option<ClientCertificateConfig>,
    #[serde(rename = "domainConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_configuration_arn: Option<String>,
    #[serde(rename = "domainConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_configuration_name: Option<String>,
    #[serde(rename = "domainConfigurationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_configuration_status: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "domainType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_type: Option<String>,
    #[serde(rename = "lastStatusChangeDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_change_date: Option<f64>,
    #[serde(rename = "serverCertificateConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_config: Option<ServerCertificateConfig>,
    #[serde(rename = "serverCertificates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificates: Option<Vec<ServerCertificateSummary>>,
    #[serde(rename = "serviceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    #[serde(rename = "tlsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_config: Option<TlsConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServerCertificateSummary {
    #[serde(rename = "serverCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_arn: Option<String>,
    #[serde(rename = "serverCertificateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_status: Option<String>,
    #[serde(rename = "serverCertificateStatusDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_status_detail: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEncryptionConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEncryptionConfigurationResponse {
    #[serde(rename = "configurationDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_details: Option<ConfigurationDetails>,
    #[serde(rename = "encryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    #[serde(rename = "kmsAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_access_role_arn: Option<String>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationDetails {
    #[serde(rename = "configurationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_status: Option<String>,
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEndpointRequest {
    #[serde(rename = "endpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEndpointResponse {
    #[serde(rename = "endpointAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEventConfigurationsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEventConfigurationsResponse {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "eventConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_configurations: Option<std::collections::HashMap<String, Configuration>>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Configuration {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFleetMetricRequest {
    #[serde(rename = "metricName")]
    #[serde(default)]
    pub metric_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFleetMetricResponse {
    #[serde(rename = "aggregationField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_field: Option<String>,
    #[serde(rename = "aggregationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<AggregationType>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "indexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "metricArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_arn: Option<String>,
    #[serde(rename = "metricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(rename = "queryString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
    #[serde(rename = "queryVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIndexRequest {
    #[serde(rename = "indexName")]
    #[serde(default)]
    pub index_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIndexResponse {
    #[serde(rename = "indexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "indexStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeJobExecutionRequest {
    #[serde(rename = "executionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_number: Option<i64>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "thingName")]
    #[serde(default)]
    pub thing_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeJobExecutionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution: Option<JobExecution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobExecution {
    #[serde(rename = "approximateSecondsBeforeTimedOut")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_seconds_before_timed_out: Option<i64>,
    #[serde(rename = "executionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_number: Option<i64>,
    #[serde(rename = "forceCanceled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_canceled: Option<bool>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "queuedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_at: Option<f64>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<JobExecutionStatusDetails>,
    #[serde(rename = "thingArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_arn: Option<String>,
    #[serde(rename = "versionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobExecutionStatusDetails {
    #[serde(rename = "detailsMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_map: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeJobRequest {
    #[serde(rename = "beforeSubstitution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_substitution: Option<bool>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeJobResponse {
    #[serde(rename = "documentSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_source: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Job {
    #[serde(rename = "abortConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abort_config: Option<AbortConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "completedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<f64>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "destinationPackageVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_package_versions: Option<Vec<String>>,
    #[serde(rename = "documentParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "forceCanceled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_canceled: Option<bool>,
    #[serde(rename = "isConcurrent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_concurrent: Option<bool>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobExecutionsRetryConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_executions_retry_config: Option<JobExecutionsRetryConfig>,
    #[serde(rename = "jobExecutionsRolloutConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_executions_rollout_config: Option<JobExecutionsRolloutConfig>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "jobProcessDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_process_details: Option<JobProcessDetails>,
    #[serde(rename = "jobTemplateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template_arn: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "namespaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
    #[serde(rename = "presignedUrlConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presigned_url_config: Option<PresignedUrlConfig>,
    #[serde(rename = "reasonCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    #[serde(rename = "scheduledJobRollouts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_job_rollouts: Option<Vec<ScheduledJobRollout>>,
    #[serde(rename = "schedulingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_config: Option<SchedulingConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "targetSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_selection: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
    #[serde(rename = "timeoutConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_config: Option<TimeoutConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobProcessDetails {
    #[serde(rename = "numberOfCanceledThings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_canceled_things: Option<i32>,
    #[serde(rename = "numberOfFailedThings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_failed_things: Option<i32>,
    #[serde(rename = "numberOfInProgressThings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_in_progress_things: Option<i32>,
    #[serde(rename = "numberOfQueuedThings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_queued_things: Option<i32>,
    #[serde(rename = "numberOfRejectedThings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_rejected_things: Option<i32>,
    #[serde(rename = "numberOfRemovedThings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_removed_things: Option<i32>,
    #[serde(rename = "numberOfSucceededThings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_succeeded_things: Option<i32>,
    #[serde(rename = "numberOfTimedOutThings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_timed_out_things: Option<i32>,
    #[serde(rename = "processingTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_targets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledJobRollout {
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeJobTemplateRequest {
    #[serde(rename = "jobTemplateId")]
    #[serde(default)]
    pub job_template_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeJobTemplateResponse {
    #[serde(rename = "abortConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abort_config: Option<AbortConfig>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "destinationPackageVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_package_versions: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    #[serde(rename = "documentSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_source: Option<String>,
    #[serde(rename = "jobExecutionsRetryConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_executions_retry_config: Option<JobExecutionsRetryConfig>,
    #[serde(rename = "jobExecutionsRolloutConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_executions_rollout_config: Option<JobExecutionsRolloutConfig>,
    #[serde(rename = "jobTemplateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template_arn: Option<String>,
    #[serde(rename = "jobTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template_id: Option<String>,
    #[serde(rename = "maintenanceWindows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_windows: Option<Vec<MaintenanceWindow>>,
    #[serde(rename = "presignedUrlConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presigned_url_config: Option<PresignedUrlConfig>,
    #[serde(rename = "timeoutConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_config: Option<TimeoutConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeManagedJobTemplateRequest {
    #[serde(rename = "templateName")]
    #[serde(default)]
    pub template_name: String,
    #[serde(rename = "templateVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeManagedJobTemplateResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    #[serde(rename = "documentParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_parameters: Option<Vec<DocumentParameter>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environments: Option<Vec<String>>,
    #[serde(rename = "templateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_arn: Option<String>,
    #[serde(rename = "templateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "templateVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentParameter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMitigationActionRequest {
    #[serde(rename = "actionName")]
    #[serde(default)]
    pub action_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMitigationActionResponse {
    #[serde(rename = "actionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_arn: Option<String>,
    #[serde(rename = "actionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(rename = "actionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    #[serde(rename = "actionParams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_params: Option<MitigationActionParams>,
    #[serde(rename = "actionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProvisioningTemplateRequest {
    #[serde(rename = "templateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProvisioningTemplateResponse {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "defaultVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version_id: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "preProvisioningHook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_provisioning_hook: Option<ProvisioningHook>,
    #[serde(rename = "provisioningRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_role_arn: Option<String>,
    #[serde(rename = "templateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_arn: Option<String>,
    #[serde(rename = "templateBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    #[serde(rename = "templateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProvisioningTemplateVersionRequest {
    #[serde(rename = "templateName")]
    #[serde(default)]
    pub template_name: String,
    #[serde(rename = "versionId")]
    #[serde(default)]
    pub version_id: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProvisioningTemplateVersionResponse {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "isDefaultVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_version: Option<bool>,
    #[serde(rename = "templateBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    #[serde(rename = "versionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRoleAliasRequest {
    #[serde(rename = "roleAlias")]
    #[serde(default)]
    pub role_alias: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRoleAliasResponse {
    #[serde(rename = "roleAliasDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_alias_description: Option<RoleAliasDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoleAliasDescription {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "credentialDurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_duration_seconds: Option<i32>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "roleAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_alias: Option<String>,
    #[serde(rename = "roleAliasArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_alias_arn: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScheduledAuditRequest {
    #[serde(rename = "scheduledAuditName")]
    #[serde(default)]
    pub scheduled_audit_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeScheduledAuditResponse {
    #[serde(rename = "dayOfMonth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_month: Option<String>,
    #[serde(rename = "dayOfWeek")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    #[serde(rename = "scheduledAuditArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_audit_arn: Option<String>,
    #[serde(rename = "scheduledAuditName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_audit_name: Option<String>,
    #[serde(rename = "targetCheckNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_check_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSecurityProfileRequest {
    #[serde(rename = "securityProfileName")]
    #[serde(default)]
    pub security_profile_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSecurityProfileResponse {
    #[serde(rename = "additionalMetricsToRetain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_metrics_to_retain: Option<Vec<String>>,
    #[serde(rename = "additionalMetricsToRetainV2")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_metrics_to_retain_v2: Option<Vec<MetricToRetain>>,
    #[serde(rename = "alertTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_targets: Option<std::collections::HashMap<String, AlertTarget>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behaviors: Option<Vec<Behavior>>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "metricsExportConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_export_config: Option<MetricsExportConfig>,
    #[serde(rename = "securityProfileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_arn: Option<String>,
    #[serde(rename = "securityProfileDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_description: Option<String>,
    #[serde(rename = "securityProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStreamRequest {
    #[serde(rename = "streamId")]
    #[serde(default)]
    pub stream_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStreamResponse {
    #[serde(rename = "streamInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_info: Option<StreamInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamInfo {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<StreamFile>>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "streamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    #[serde(rename = "streamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "streamVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeThingGroupRequest {
    #[serde(rename = "thingGroupName")]
    #[serde(default)]
    pub thing_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeThingGroupResponse {
    #[serde(rename = "indexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "queryString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
    #[serde(rename = "queryVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "thingGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_arn: Option<String>,
    #[serde(rename = "thingGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_id: Option<String>,
    #[serde(rename = "thingGroupMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_metadata: Option<ThingGroupMetadata>,
    #[serde(rename = "thingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_name: Option<String>,
    #[serde(rename = "thingGroupProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_properties: Option<ThingGroupProperties>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThingGroupMetadata {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "parentGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_name: Option<String>,
    #[serde(rename = "rootToParentThingGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_to_parent_thing_groups: Option<Vec<GroupNameAndArn>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupNameAndArn {
    #[serde(rename = "groupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
    #[serde(rename = "groupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeThingRegistrationTaskRequest {
    #[serde(rename = "taskId")]
    #[serde(default)]
    pub task_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeThingRegistrationTaskResponse {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "failureCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_count: Option<i32>,
    #[serde(rename = "inputFileBucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_file_bucket: Option<String>,
    #[serde(rename = "inputFileKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_file_key: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "percentageProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_progress: Option<i32>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "successCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_count: Option<i32>,
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(rename = "templateBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeThingRequest {
    #[serde(rename = "thingName")]
    #[serde(default)]
    pub thing_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeThingResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "billingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_group_name: Option<String>,
    #[serde(rename = "defaultClientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_client_id: Option<String>,
    #[serde(rename = "thingArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_arn: Option<String>,
    #[serde(rename = "thingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_id: Option<String>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
    #[serde(rename = "thingTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeThingTypeRequest {
    #[serde(rename = "thingTypeName")]
    #[serde(default)]
    pub thing_type_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeThingTypeResponse {
    #[serde(rename = "thingTypeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_arn: Option<String>,
    #[serde(rename = "thingTypeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_id: Option<String>,
    #[serde(rename = "thingTypeMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_metadata: Option<ThingTypeMetadata>,
    #[serde(rename = "thingTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_name: Option<String>,
    #[serde(rename = "thingTypeProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_properties: Option<ThingTypeProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThingTypeMetadata {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(rename = "deprecationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachPolicyRequest {
    #[serde(rename = "policyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(default)]
    pub target: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachPrincipalPolicyRequest {
    #[serde(rename = "policyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(default)]
    pub principal: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachSecurityProfileRequest {
    #[serde(rename = "securityProfileName")]
    #[serde(default)]
    pub security_profile_name: String,
    #[serde(rename = "securityProfileTargetArn")]
    #[serde(default)]
    pub security_profile_target_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachSecurityProfileResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachThingPrincipalRequest {
    #[serde(default)]
    pub principal: String,
    #[serde(rename = "thingName")]
    #[serde(default)]
    pub thing_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachThingPrincipalResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableTopicRuleRequest {
    #[serde(rename = "ruleName")]
    #[serde(default)]
    pub rule_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateSbomFromPackageVersionRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "packageName")]
    #[serde(default)]
    pub package_name: String,
    #[serde(rename = "versionName")]
    #[serde(default)]
    pub version_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateSbomFromPackageVersionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableTopicRuleRequest {
    #[serde(rename = "ruleName")]
    #[serde(default)]
    pub rule_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBehaviorModelTrainingSummariesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "securityProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBehaviorModelTrainingSummariesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<Vec<BehaviorModelTrainingSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BehaviorModelTrainingSummary {
    #[serde(rename = "behaviorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior_name: Option<String>,
    #[serde(rename = "datapointsCollectionPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datapoints_collection_percentage: Option<f64>,
    #[serde(rename = "lastModelRefreshDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_model_refresh_date: Option<f64>,
    #[serde(rename = "modelStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_status: Option<String>,
    #[serde(rename = "securityProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_name: Option<String>,
    #[serde(rename = "trainingDataCollectionStartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_data_collection_start_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBucketsAggregationRequest {
    #[serde(rename = "aggregationField")]
    #[serde(default)]
    pub aggregation_field: String,
    #[serde(rename = "bucketsAggregationType")]
    #[serde(default)]
    pub buckets_aggregation_type: BucketsAggregationType,
    #[serde(rename = "indexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "queryString")]
    #[serde(default)]
    pub query_string: String,
    #[serde(rename = "queryVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BucketsAggregationType {
    #[serde(rename = "termsAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_aggregation: Option<TermsAggregation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TermsAggregation {
    #[serde(rename = "maxBuckets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_buckets: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBucketsAggregationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Vec<Bucket>>,
    #[serde(rename = "totalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Bucket {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "keyValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCardinalityRequest {
    #[serde(rename = "aggregationField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_field: Option<String>,
    #[serde(rename = "indexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "queryString")]
    #[serde(default)]
    pub query_string: String,
    #[serde(rename = "queryVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCardinalityResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardinality: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCommandExecutionRequest {
    #[serde(rename = "executionId")]
    #[serde(default)]
    pub execution_id: String,
    #[serde(rename = "includeResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_result: Option<bool>,
    #[serde(rename = "targetArn")]
    #[serde(default)]
    pub target_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCommandExecutionResponse {
    #[serde(rename = "commandArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_arn: Option<String>,
    #[serde(rename = "completedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<f64>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "executionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    #[serde(rename = "executionTimeoutSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_timeout_seconds: Option<i64>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, CommandParameterValue>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<std::collections::HashMap<String, CommandExecutionResult>>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<StatusReason>,
    #[serde(rename = "targetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
    #[serde(rename = "timeToLive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommandExecutionResult {
    #[serde(rename = "B")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<bool>,
    #[serde(rename = "BIN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_i_n: Option<String>,
    #[serde(rename = "S")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatusReason {
    #[serde(rename = "reasonCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    #[serde(rename = "reasonDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCommandRequest {
    #[serde(rename = "commandId")]
    #[serde(default)]
    pub command_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCommandResponse {
    #[serde(rename = "commandArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_arn: Option<String>,
    #[serde(rename = "commandId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "mandatoryParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandatory_parameters: Option<Vec<CommandParameter>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<CommandPayload>,
    #[serde(rename = "payloadTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_template: Option<String>,
    #[serde(rename = "pendingDeletion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_deletion: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preprocessor: Option<CommandPreprocessor>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEffectivePoliciesRequest {
    #[serde(rename = "cognitoIdentityPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_identity_pool_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEffectivePoliciesResponse {
    #[serde(rename = "effectivePolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_policies: Option<Vec<EffectivePolicy>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EffectivePolicy {
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    #[serde(rename = "policyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIndexingConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIndexingConfigurationResponse {
    #[serde(rename = "thingGroupIndexingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_indexing_configuration: Option<ThingGroupIndexingConfiguration>,
    #[serde(rename = "thingIndexingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_indexing_configuration: Option<ThingIndexingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThingGroupIndexingConfiguration {
    #[serde(rename = "customFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<Field>>,
    #[serde(rename = "managedFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_fields: Option<Vec<Field>>,
    #[serde(rename = "thingGroupIndexingMode")]
    #[serde(default)]
    pub thing_group_indexing_mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Field {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThingIndexingConfiguration {
    #[serde(rename = "customFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<Field>>,
    #[serde(rename = "deviceDefenderIndexingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_defender_indexing_mode: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<IndexingFilter>,
    #[serde(rename = "managedFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_fields: Option<Vec<Field>>,
    #[serde(rename = "namedShadowIndexingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_shadow_indexing_mode: Option<String>,
    #[serde(rename = "thingConnectivityIndexingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_connectivity_indexing_mode: Option<String>,
    #[serde(rename = "thingIndexingMode")]
    #[serde(default)]
    pub thing_indexing_mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IndexingFilter {
    #[serde(rename = "geoLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_locations: Option<Vec<GeoLocationTarget>>,
    #[serde(rename = "namedShadowNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_shadow_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeoLocationTarget {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJobDocumentRequest {
    #[serde(rename = "beforeSubstitution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_substitution: Option<bool>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJobDocumentResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLoggingOptionsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLoggingOptionsResponse {
    #[serde(rename = "logLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOTAUpdateRequest {
    #[serde(rename = "otaUpdateId")]
    #[serde(default)]
    pub ota_update_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOTAUpdateResponse {
    #[serde(rename = "otaUpdateInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update_info: Option<OTAUpdateInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OTAUpdateInfo {
    #[serde(rename = "additionalParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "awsIotJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iot_job_arn: Option<String>,
    #[serde(rename = "awsIotJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iot_job_id: Option<String>,
    #[serde(rename = "awsJobExecutionsRolloutConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_job_executions_rollout_config: Option<AwsJobExecutionsRolloutConfig>,
    #[serde(rename = "awsJobPresignedUrlConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_job_presigned_url_config: Option<AwsJobPresignedUrlConfig>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "errorInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_info: Option<ErrorInfo>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "otaUpdateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update_arn: Option<String>,
    #[serde(rename = "otaUpdateFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update_files: Option<Vec<OTAUpdateFile>>,
    #[serde(rename = "otaUpdateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update_id: Option<String>,
    #[serde(rename = "otaUpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,
    #[serde(rename = "targetSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_selection: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPackageConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPackageConfigurationResponse {
    #[serde(rename = "versionUpdateByJobsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_update_by_jobs_config: Option<VersionUpdateByJobsConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VersionUpdateByJobsConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPackageRequest {
    #[serde(rename = "packageName")]
    #[serde(default)]
    pub package_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPackageResponse {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "defaultVersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "packageArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_arn: Option<String>,
    #[serde(rename = "packageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPackageVersionRequest {
    #[serde(rename = "packageName")]
    #[serde(default)]
    pub package_name: String,
    #[serde(rename = "versionName")]
    #[serde(default)]
    pub version_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPackageVersionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<PackageVersionArtifact>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "errorReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "packageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(rename = "packageVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sbom: Option<Sbom>,
    #[serde(rename = "sbomValidationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sbom_validation_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "versionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPercentilesRequest {
    #[serde(rename = "aggregationField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_field: Option<String>,
    #[serde(rename = "indexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percents: Option<Vec<f64>>,
    #[serde(rename = "queryString")]
    #[serde(default)]
    pub query_string: String,
    #[serde(rename = "queryVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPercentilesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentiles: Option<Vec<PercentPair>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PercentPair {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPolicyRequest {
    #[serde(rename = "policyName")]
    #[serde(default)]
    pub policy_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPolicyResponse {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "defaultVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version_id: Option<String>,
    #[serde(rename = "generationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_id: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    #[serde(rename = "policyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPolicyVersionRequest {
    #[serde(rename = "policyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(rename = "policyVersionId")]
    #[serde(default)]
    pub policy_version_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPolicyVersionResponse {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "generationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_id: Option<String>,
    #[serde(rename = "isDefaultVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_version: Option<bool>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    #[serde(rename = "policyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "policyVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRegistrationCodeRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRegistrationCodeResponse {
    #[serde(rename = "registrationCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStatisticsRequest {
    #[serde(rename = "aggregationField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_field: Option<String>,
    #[serde(rename = "indexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "queryString")]
    #[serde(default)]
    pub query_string: String,
    #[serde(rename = "queryVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStatisticsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Statistics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Statistics {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(rename = "stdDeviation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_deviation: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum: Option<f64>,
    #[serde(rename = "sumOfSquares")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum_of_squares: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variance: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetThingConnectivityDataRequest {
    #[serde(rename = "thingName")]
    #[serde(default)]
    pub thing_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetThingConnectivityDataResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
    #[serde(rename = "disconnectReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_reason: Option<String>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTopicRuleDestinationRequest {
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTopicRuleDestinationResponse {
    #[serde(rename = "topicRuleDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_rule_destination: Option<TopicRuleDestination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTopicRuleRequest {
    #[serde(rename = "ruleName")]
    #[serde(default)]
    pub rule_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTopicRuleResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<TopicRule>,
    #[serde(rename = "ruleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicRule {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
    #[serde(rename = "awsIotSqlVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iot_sql_version: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "errorAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_action: Option<Action>,
    #[serde(rename = "ruleDisabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_disabled: Option<bool>,
    #[serde(rename = "ruleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetV2LoggingOptionsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbose: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetV2LoggingOptionsResponse {
    #[serde(rename = "defaultLogLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_log_level: Option<String>,
    #[serde(rename = "disableAllLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_all_logs: Option<bool>,
    #[serde(rename = "eventConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_configurations: Option<Vec<LogEventConfiguration>>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogEventConfiguration {
    #[serde(rename = "eventType")]
    #[serde(default)]
    pub event_type: String,
    #[serde(rename = "logDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_destination: Option<String>,
    #[serde(rename = "logLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListActiveViolationsRequest {
    #[serde(rename = "behaviorCriteriaType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior_criteria_type: Option<String>,
    #[serde(rename = "listSuppressedAlerts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_suppressed_alerts: Option<bool>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "securityProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_name: Option<String>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
    #[serde(rename = "verificationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListActiveViolationsResponse {
    #[serde(rename = "activeViolations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_violations: Option<Vec<ActiveViolation>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActiveViolation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<Behavior>,
    #[serde(rename = "lastViolationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_violation_time: Option<f64>,
    #[serde(rename = "lastViolationValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_violation_value: Option<MetricValue>,
    #[serde(rename = "securityProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_name: Option<String>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
    #[serde(rename = "verificationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_state: Option<String>,
    #[serde(rename = "verificationStateDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_state_description: Option<String>,
    #[serde(rename = "violationEventAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_event_additional_info: Option<ViolationEventAdditionalInfo>,
    #[serde(rename = "violationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_id: Option<String>,
    #[serde(rename = "violationStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ViolationEventAdditionalInfo {
    #[serde(rename = "confidenceLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_level: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAttachedPoliciesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
    #[serde(default)]
    pub target: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAttachedPoliciesResponse {
    #[serde(rename = "nextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<Policy>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Policy {
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "policyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAuditFindingsRequest {
    #[serde(rename = "checkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_name: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "listSuppressedFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_suppressed_findings: Option<bool>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<ResourceIdentifier>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAuditFindingsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings: Option<Vec<AuditFinding>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAuditMitigationActionsExecutionsRequest {
    #[serde(rename = "actionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_status: Option<String>,
    #[serde(rename = "findingId")]
    #[serde(default)]
    pub finding_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "taskId")]
    #[serde(default)]
    pub task_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAuditMitigationActionsExecutionsResponse {
    #[serde(rename = "actionsExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_executions: Option<Vec<AuditMitigationActionExecutionMetadata>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditMitigationActionExecutionMetadata {
    #[serde(rename = "actionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(rename = "actionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "findingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAuditMitigationActionsTasksRequest {
    #[serde(rename = "auditTaskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_task_id: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "findingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_id: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    pub start_time: f64,
    #[serde(rename = "taskStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAuditMitigationActionsTasksResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<AuditMitigationActionsTaskMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditMitigationActionsTaskMetadata {
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(rename = "taskStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAuditSuppressionsRequest {
    #[serde(rename = "ascendingOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending_order: Option<bool>,
    #[serde(rename = "checkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_name: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<ResourceIdentifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAuditSuppressionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressions: Option<Vec<AuditSuppression>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditSuppression {
    #[serde(rename = "checkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "expirationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<ResourceIdentifier>,
    #[serde(rename = "suppressIndefinitely")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_indefinitely: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAuditTasksRequest {
    #[serde(rename = "endTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    pub start_time: f64,
    #[serde(rename = "taskStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
    #[serde(rename = "taskType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAuditTasksResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<AuditTaskMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditTaskMetadata {
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(rename = "taskStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
    #[serde(rename = "taskType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAuthorizersRequest {
    #[serde(rename = "ascendingOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending_order: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAuthorizersResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizers: Option<Vec<AuthorizerSummary>>,
    #[serde(rename = "nextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthorizerSummary {
    #[serde(rename = "authorizerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_arn: Option<String>,
    #[serde(rename = "authorizerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBillingGroupsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "namePrefixFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix_filter: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBillingGroupsResponse {
    #[serde(rename = "billingGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_groups: Option<Vec<GroupNameAndArn>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCACertificatesRequest {
    #[serde(rename = "ascendingOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending_order: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "templateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCACertificatesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<CACertificate>>,
    #[serde(rename = "nextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CACertificate {
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "certificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCertificateProvidersRequest {
    #[serde(rename = "ascendingOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending_order: Option<bool>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCertificateProvidersResponse {
    #[serde(rename = "certificateProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_providers: Option<Vec<CertificateProviderSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CertificateProviderSummary {
    #[serde(rename = "certificateProviderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_provider_arn: Option<String>,
    #[serde(rename = "certificateProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_provider_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCertificatesByCARequest {
    #[serde(rename = "ascendingOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending_order: Option<bool>,
    #[serde(rename = "caCertificateId")]
    #[serde(default)]
    pub ca_certificate_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCertificatesByCAResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<Certificate>>,
    #[serde(rename = "nextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Certificate {
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "certificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    #[serde(rename = "certificateMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_mode: Option<String>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCertificatesRequest {
    #[serde(rename = "ascendingOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending_order: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCertificatesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<Certificate>>,
    #[serde(rename = "nextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCommandExecutionsRequest {
    #[serde(rename = "commandArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_arn: Option<String>,
    #[serde(rename = "completedTimeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_time_filter: Option<TimeFilter>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(rename = "startedTimeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_time_filter: Option<TimeFilter>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "targetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeFilter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCommandExecutionsResponse {
    #[serde(rename = "commandExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_executions: Option<Vec<CommandExecutionSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommandExecutionSummary {
    #[serde(rename = "commandArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_arn: Option<String>,
    #[serde(rename = "completedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<f64>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "executionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "targetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCommandsRequest {
    #[serde(rename = "commandParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_parameter_name: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCommandsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<CommandSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommandSummary {
    #[serde(rename = "commandArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_arn: Option<String>,
    #[serde(rename = "commandId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "pendingDeletion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_deletion: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCustomMetricsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCustomMetricsResponse {
    #[serde(rename = "metricNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_names: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDetectMitigationActionsExecutionsRequest {
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
    #[serde(rename = "violationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDetectMitigationActionsExecutionsResponse {
    #[serde(rename = "actionsExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_executions: Option<Vec<DetectMitigationActionExecution>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectMitigationActionExecution {
    #[serde(rename = "actionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "executionEndDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_end_date: Option<f64>,
    #[serde(rename = "executionStartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
    #[serde(rename = "violationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDetectMitigationActionsTasksRequest {
    #[serde(rename = "endTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    pub start_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDetectMitigationActionsTasksResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<DetectMitigationActionsTaskSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDimensionsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDimensionsResponse {
    #[serde(rename = "dimensionNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_names: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDomainConfigurationsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "serviceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDomainConfigurationsResponse {
    #[serde(rename = "domainConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_configurations: Option<Vec<DomainConfigurationSummary>>,
    #[serde(rename = "nextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainConfigurationSummary {
    #[serde(rename = "domainConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_configuration_arn: Option<String>,
    #[serde(rename = "domainConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_configuration_name: Option<String>,
    #[serde(rename = "serviceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFleetMetricsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFleetMetricsResponse {
    #[serde(rename = "fleetMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_metrics: Option<Vec<FleetMetricNameAndArn>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FleetMetricNameAndArn {
    #[serde(rename = "metricArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_arn: Option<String>,
    #[serde(rename = "metricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIndicesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIndicesResponse {
    #[serde(rename = "indexNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_names: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobExecutionsForJobRequest {
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobExecutionsForJobResponse {
    #[serde(rename = "executionSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_summaries: Option<Vec<JobExecutionSummaryForJob>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobExecutionSummaryForJob {
    #[serde(rename = "jobExecutionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_execution_summary: Option<JobExecutionSummary>,
    #[serde(rename = "thingArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobExecutionSummary {
    #[serde(rename = "executionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_number: Option<i64>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "queuedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_at: Option<f64>,
    #[serde(rename = "retryAttempt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_attempt: Option<i32>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobExecutionsForThingRequest {
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "namespaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    pub thing_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobExecutionsForThingResponse {
    #[serde(rename = "executionSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_summaries: Option<Vec<JobExecutionSummaryForThing>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobExecutionSummaryForThing {
    #[serde(rename = "jobExecutionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_execution_summary: Option<JobExecutionSummary>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobTemplatesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobTemplatesResponse {
    #[serde(rename = "jobTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_templates: Option<Vec<JobTemplateSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobTemplateSummary {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "jobTemplateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template_arn: Option<String>,
    #[serde(rename = "jobTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "namespaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "targetSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_selection: Option<String>,
    #[serde(rename = "thingGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_id: Option<String>,
    #[serde(rename = "thingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<JobSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobSummary {
    #[serde(rename = "completedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<f64>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "isConcurrent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_concurrent: Option<bool>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "targetSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_selection: Option<String>,
    #[serde(rename = "thingGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListManagedJobTemplatesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "templateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListManagedJobTemplatesResponse {
    #[serde(rename = "managedJobTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_job_templates: Option<Vec<ManagedJobTemplateSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedJobTemplateSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environments: Option<Vec<String>>,
    #[serde(rename = "templateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_arn: Option<String>,
    #[serde(rename = "templateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "templateVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMetricValuesRequest {
    #[serde(rename = "dimensionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_name: Option<String>,
    #[serde(rename = "dimensionValueOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_value_operator: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "metricName")]
    #[serde(default)]
    pub metric_name: String,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    pub start_time: f64,
    #[serde(rename = "thingName")]
    #[serde(default)]
    pub thing_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMetricValuesResponse {
    #[serde(rename = "metricDatumList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_datum_list: Option<Vec<MetricDatum>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricDatum {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<MetricValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMitigationActionsRequest {
    #[serde(rename = "actionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMitigationActionsResponse {
    #[serde(rename = "actionIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_identifiers: Option<Vec<MitigationActionIdentifier>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MitigationActionIdentifier {
    #[serde(rename = "actionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_arn: Option<String>,
    #[serde(rename = "actionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOTAUpdatesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "otaUpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOTAUpdatesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "otaUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_updates: Option<Vec<OTAUpdateSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OTAUpdateSummary {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "otaUpdateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update_arn: Option<String>,
    #[serde(rename = "otaUpdateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOutgoingCertificatesRequest {
    #[serde(rename = "ascendingOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending_order: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOutgoingCertificatesResponse {
    #[serde(rename = "nextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "outgoingCertificates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outgoing_certificates: Option<Vec<OutgoingCertificate>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutgoingCertificate {
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "certificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "transferDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_date: Option<f64>,
    #[serde(rename = "transferMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_message: Option<String>,
    #[serde(rename = "transferredTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferred_to: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPackageVersionsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "packageName")]
    #[serde(default)]
    pub package_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPackageVersionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "packageVersionSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version_summaries: Option<Vec<PackageVersionSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageVersionSummary {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "packageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "versionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPackagesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPackagesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "packageSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_summaries: Option<Vec<PackageSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageSummary {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "defaultVersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version_name: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "packageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPoliciesRequest {
    #[serde(rename = "ascendingOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending_order: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPoliciesResponse {
    #[serde(rename = "nextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<Policy>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPolicyPrincipalsRequest {
    #[serde(rename = "ascendingOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending_order: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "policyName")]
    #[serde(default)]
    pub policy_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPolicyPrincipalsResponse {
    #[serde(rename = "nextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPolicyVersionsRequest {
    #[serde(rename = "policyName")]
    #[serde(default)]
    pub policy_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPolicyVersionsResponse {
    #[serde(rename = "policyVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_versions: Option<Vec<PolicyVersion>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyVersion {
    #[serde(rename = "createDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    #[serde(rename = "isDefaultVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_version: Option<bool>,
    #[serde(rename = "versionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPrincipalPoliciesRequest {
    #[serde(rename = "ascendingOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending_order: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(default)]
    pub principal: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPrincipalPoliciesResponse {
    #[serde(rename = "nextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<Policy>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPrincipalThingsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    pub principal: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPrincipalThingsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub things: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPrincipalThingsV2Request {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    pub principal: String,
    #[serde(rename = "thingPrincipalType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_principal_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPrincipalThingsV2Response {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "principalThingObjects")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_thing_objects: Option<Vec<PrincipalThingObject>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrincipalThingObject {
    #[serde(rename = "thingName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
    #[serde(rename = "thingPrincipalType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_principal_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProvisioningTemplateVersionsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "templateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProvisioningTemplateVersionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<ProvisioningTemplateVersionSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisioningTemplateVersionSummary {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "isDefaultVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_version: Option<bool>,
    #[serde(rename = "versionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProvisioningTemplatesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProvisioningTemplatesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<ProvisioningTemplateSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisioningTemplateSummary {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "templateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_arn: Option<String>,
    #[serde(rename = "templateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRelatedResourcesForAuditFindingRequest {
    #[serde(rename = "findingId")]
    #[serde(default)]
    pub finding_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRelatedResourcesForAuditFindingResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "relatedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_resources: Option<Vec<RelatedResource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRoleAliasesRequest {
    #[serde(rename = "ascendingOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending_order: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRoleAliasesResponse {
    #[serde(rename = "nextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "roleAliases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_aliases: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSbomValidationResultsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "packageName")]
    #[serde(default)]
    pub package_name: String,
    #[serde(rename = "validationResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_result: Option<String>,
    #[serde(rename = "versionName")]
    #[serde(default)]
    pub version_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSbomValidationResultsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "validationResultSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_result_summaries: Option<Vec<SbomValidationResultSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SbomValidationResultSummary {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "fileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "validationResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListScheduledAuditsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListScheduledAuditsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "scheduledAudits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_audits: Option<Vec<ScheduledAuditMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledAuditMetadata {
    #[serde(rename = "dayOfMonth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_month: Option<String>,
    #[serde(rename = "dayOfWeek")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    #[serde(rename = "scheduledAuditArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_audit_arn: Option<String>,
    #[serde(rename = "scheduledAuditName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_audit_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecurityProfilesForTargetRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
    #[serde(rename = "securityProfileTargetArn")]
    #[serde(default)]
    pub security_profile_target_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecurityProfilesForTargetResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "securityProfileTargetMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_target_mappings: Option<Vec<SecurityProfileTargetMapping>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityProfileTargetMapping {
    #[serde(rename = "securityProfileIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_identifier: Option<SecurityProfileIdentifier>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<SecurityProfileTarget>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityProfileIdentifier {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityProfileTarget {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecurityProfilesRequest {
    #[serde(rename = "dimensionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_name: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "metricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecurityProfilesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "securityProfileIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_identifiers: Option<Vec<SecurityProfileIdentifier>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStreamsRequest {
    #[serde(rename = "ascendingOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending_order: Option<bool>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStreamsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streams: Option<Vec<StreamSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "streamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    #[serde(rename = "streamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "streamVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTargetsForPolicyRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "policyName")]
    #[serde(default)]
    pub policy_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTargetsForPolicyResponse {
    #[serde(rename = "nextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTargetsForSecurityProfileRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "securityProfileName")]
    #[serde(default)]
    pub security_profile_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTargetsForSecurityProfileResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "securityProfileTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_targets: Option<Vec<SecurityProfileTarget>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThingGroupsForThingRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    pub thing_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThingGroupsForThingResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "thingGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_groups: Option<Vec<GroupNameAndArn>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThingGroupsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "namePrefixFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix_filter: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "parentGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThingGroupsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "thingGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_groups: Option<Vec<GroupNameAndArn>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThingPrincipalsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    pub thing_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThingPrincipalsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThingPrincipalsV2Request {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    pub thing_name: String,
    #[serde(rename = "thingPrincipalType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_principal_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThingPrincipalsV2Response {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "thingPrincipalObjects")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_principal_objects: Option<Vec<ThingPrincipalObject>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThingPrincipalObject {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
    #[serde(rename = "thingPrincipalType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_principal_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThingRegistrationTaskReportsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "reportType")]
    #[serde(default)]
    pub report_type: String,
    #[serde(rename = "taskId")]
    #[serde(default)]
    pub task_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThingRegistrationTaskReportsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "reportType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_type: Option<String>,
    #[serde(rename = "resourceLinks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_links: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThingRegistrationTasksRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThingRegistrationTasksResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "taskIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThingTypesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "thingTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThingTypesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "thingTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_types: Option<Vec<ThingTypeDefinition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThingTypeDefinition {
    #[serde(rename = "thingTypeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_arn: Option<String>,
    #[serde(rename = "thingTypeMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_metadata: Option<ThingTypeMetadata>,
    #[serde(rename = "thingTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_name: Option<String>,
    #[serde(rename = "thingTypeProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_properties: Option<ThingTypeProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThingsInBillingGroupRequest {
    #[serde(rename = "billingGroupName")]
    #[serde(default)]
    pub billing_group_name: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThingsInBillingGroupResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub things: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThingsInThingGroupRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
    #[serde(rename = "thingGroupName")]
    #[serde(default)]
    pub thing_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThingsInThingGroupResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub things: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThingsRequest {
    #[serde(rename = "attributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "attributeValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "thingTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_name: Option<String>,
    #[serde(rename = "usePrefixAttributeValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_prefix_attribute_value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThingsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub things: Option<Vec<ThingAttribute>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThingAttribute {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "thingArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_arn: Option<String>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
    #[serde(rename = "thingTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTopicRuleDestinationsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTopicRuleDestinationsResponse {
    #[serde(rename = "destinationSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_summaries: Option<Vec<TopicRuleDestinationSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicRuleDestinationSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "httpUrlSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_url_summary: Option<HttpUrlDestinationSummary>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "vpcDestinationSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_destination_summary: Option<VpcDestinationSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpUrlDestinationSummary {
    #[serde(rename = "confirmationUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcDestinationSummary {
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTopicRulesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ruleDisabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_disabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTopicRulesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<TopicRuleListItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicRuleListItem {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "ruleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
    #[serde(rename = "ruleDisabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_disabled: Option<bool>,
    #[serde(rename = "ruleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[serde(rename = "topicPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_pattern: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListV2LoggingLevelsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "targetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListV2LoggingLevelsResponse {
    #[serde(rename = "logTargetConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_target_configurations: Option<Vec<LogTargetConfiguration>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogTargetConfiguration {
    #[serde(rename = "logLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(rename = "logTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_target: Option<LogTarget>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogTarget {
    #[serde(rename = "targetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_name: Option<String>,
    #[serde(rename = "targetType")]
    #[serde(default)]
    pub target_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListViolationEventsRequest {
    #[serde(rename = "behaviorCriteriaType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior_criteria_type: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "listSuppressedAlerts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_suppressed_alerts: Option<bool>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "securityProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_name: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    pub start_time: f64,
    #[serde(rename = "thingName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
    #[serde(rename = "verificationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListViolationEventsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "violationEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_events: Option<Vec<ViolationEvent>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ViolationEvent {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<Behavior>,
    #[serde(rename = "metricValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_value: Option<MetricValue>,
    #[serde(rename = "securityProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_name: Option<String>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
    #[serde(rename = "verificationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_state: Option<String>,
    #[serde(rename = "verificationStateDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_state_description: Option<String>,
    #[serde(rename = "violationEventAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_event_additional_info: Option<ViolationEventAdditionalInfo>,
    #[serde(rename = "violationEventTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_event_time: Option<f64>,
    #[serde(rename = "violationEventType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_event_type: Option<String>,
    #[serde(rename = "violationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutVerificationStateOnViolationRequest {
    #[serde(rename = "verificationState")]
    #[serde(default)]
    pub verification_state: String,
    #[serde(rename = "verificationStateDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_state_description: Option<String>,
    #[serde(rename = "violationId")]
    #[serde(default)]
    pub violation_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutVerificationStateOnViolationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterCACertificateRequest {
    #[serde(rename = "allowAutoRegistration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_auto_registration: Option<bool>,
    #[serde(rename = "caCertificate")]
    #[serde(default)]
    pub ca_certificate: String,
    #[serde(rename = "certificateMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_mode: Option<String>,
    #[serde(rename = "registrationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_config: Option<RegistrationConfig>,
    #[serde(rename = "setAsActive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_as_active: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "verificationCertificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_certificate: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterCACertificateResponse {
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "certificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterCertificateRequest {
    #[serde(rename = "caCertificatePem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_pem: Option<String>,
    #[serde(rename = "certificatePem")]
    #[serde(default)]
    pub certificate_pem: String,
    #[serde(rename = "setAsActive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_as_active: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterCertificateResponse {
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "certificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterCertificateWithoutCARequest {
    #[serde(rename = "certificatePem")]
    #[serde(default)]
    pub certificate_pem: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterCertificateWithoutCAResponse {
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "certificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterThingRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "templateBody")]
    #[serde(default)]
    pub template_body: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterThingResponse {
    #[serde(rename = "certificatePem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_pem: Option<String>,
    #[serde(rename = "resourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectCertificateTransferRequest {
    #[serde(rename = "certificateId")]
    #[serde(default)]
    pub certificate_id: String,
    #[serde(rename = "rejectReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveThingFromBillingGroupRequest {
    #[serde(rename = "billingGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_group_arn: Option<String>,
    #[serde(rename = "billingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_group_name: Option<String>,
    #[serde(rename = "thingArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_arn: Option<String>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveThingFromBillingGroupResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveThingFromThingGroupRequest {
    #[serde(rename = "thingArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_arn: Option<String>,
    #[serde(rename = "thingGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_arn: Option<String>,
    #[serde(rename = "thingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_name: Option<String>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveThingFromThingGroupResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplaceTopicRuleRequest {
    #[serde(rename = "ruleName")]
    #[serde(default)]
    pub rule_name: String,
    #[serde(rename = "topicRulePayload")]
    #[serde(default)]
    pub topic_rule_payload: TopicRulePayload,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchIndexRequest {
    #[serde(rename = "indexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "queryString")]
    #[serde(default)]
    pub query_string: String,
    #[serde(rename = "queryVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchIndexResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "thingGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_groups: Option<Vec<ThingGroupDocument>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub things: Option<Vec<ThingDocument>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThingGroupDocument {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "parentGroupNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_names: Option<Vec<String>>,
    #[serde(rename = "thingGroupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_description: Option<String>,
    #[serde(rename = "thingGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_id: Option<String>,
    #[serde(rename = "thingGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThingDocument {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity: Option<ThingConnectivity>,
    #[serde(rename = "deviceDefender")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_defender: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow: Option<String>,
    #[serde(rename = "thingGroupNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_names: Option<Vec<String>>,
    #[serde(rename = "thingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_id: Option<String>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
    #[serde(rename = "thingTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThingConnectivity {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
    #[serde(rename = "disconnectReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetDefaultAuthorizerRequest {
    #[serde(rename = "authorizerName")]
    #[serde(default)]
    pub authorizer_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetDefaultAuthorizerResponse {
    #[serde(rename = "authorizerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_arn: Option<String>,
    #[serde(rename = "authorizerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetDefaultPolicyVersionRequest {
    #[serde(rename = "policyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(rename = "policyVersionId")]
    #[serde(default)]
    pub policy_version_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetLoggingOptionsRequest {
    #[serde(rename = "loggingOptionsPayload")]
    #[serde(default)]
    pub logging_options_payload: LoggingOptionsPayload,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoggingOptionsPayload {
    #[serde(rename = "logLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetV2LoggingLevelRequest {
    #[serde(rename = "logLevel")]
    #[serde(default)]
    pub log_level: String,
    #[serde(rename = "logTarget")]
    #[serde(default)]
    pub log_target: LogTarget,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetV2LoggingOptionsRequest {
    #[serde(rename = "defaultLogLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_log_level: Option<String>,
    #[serde(rename = "disableAllLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_all_logs: Option<bool>,
    #[serde(rename = "eventConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_configurations: Option<Vec<LogEventConfiguration>>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAuditMitigationActionsTaskRequest {
    #[serde(rename = "auditCheckToActionsMapping")]
    #[serde(default)]
    pub audit_check_to_actions_mapping: std::collections::HashMap<String, Vec<String>>,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    pub client_request_token: String,
    #[serde(default)]
    pub target: AuditMitigationActionsTaskTarget,
    #[serde(rename = "taskId")]
    #[serde(default)]
    pub task_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAuditMitigationActionsTaskResponse {
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDetectMitigationActionsTaskRequest {
    #[serde(default)]
    pub actions: Vec<String>,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    pub client_request_token: String,
    #[serde(rename = "includeOnlyActiveViolations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_only_active_violations: Option<bool>,
    #[serde(rename = "includeSuppressedAlerts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_suppressed_alerts: Option<bool>,
    #[serde(default)]
    pub target: DetectMitigationActionsTaskTarget,
    #[serde(rename = "taskId")]
    #[serde(default)]
    pub task_id: String,
    #[serde(rename = "violationEventOccurrenceRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_event_occurrence_range: Option<ViolationEventOccurrenceRange>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDetectMitigationActionsTaskResponse {
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartOnDemandAuditTaskRequest {
    #[serde(rename = "targetCheckNames")]
    #[serde(default)]
    pub target_check_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartOnDemandAuditTaskResponse {
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartThingRegistrationTaskRequest {
    #[serde(rename = "inputFileBucket")]
    #[serde(default)]
    pub input_file_bucket: String,
    #[serde(rename = "inputFileKey")]
    #[serde(default)]
    pub input_file_key: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "templateBody")]
    #[serde(default)]
    pub template_body: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartThingRegistrationTaskResponse {
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopThingRegistrationTaskRequest {
    #[serde(rename = "taskId")]
    #[serde(default)]
    pub task_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopThingRegistrationTaskResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestAuthorizationRequest {
    #[serde(rename = "authInfos")]
    #[serde(default)]
    pub auth_infos: Vec<AuthInfo>,
    #[serde(rename = "clientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "cognitoIdentityPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_identity_pool_id: Option<String>,
    #[serde(rename = "policyNamesToAdd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_names_to_add: Option<Vec<String>>,
    #[serde(rename = "policyNamesToSkip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_names_to_skip: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthInfo {
    #[serde(rename = "actionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[serde(default)]
    pub resources: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestAuthorizationResponse {
    #[serde(rename = "authResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_results: Option<Vec<AuthResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed: Option<Allowed>,
    #[serde(rename = "authDecision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_decision: Option<String>,
    #[serde(rename = "authInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_info: Option<AuthInfo>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denied: Option<Denied>,
    #[serde(rename = "missingContextValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_context_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Allowed {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<Policy>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Denied {
    #[serde(rename = "explicitDeny")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_deny: Option<ExplicitDeny>,
    #[serde(rename = "implicitDeny")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit_deny: Option<ImplicitDeny>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExplicitDeny {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<Policy>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImplicitDeny {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<Policy>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestInvokeAuthorizerRequest {
    #[serde(rename = "authorizerName")]
    #[serde(default)]
    pub authorizer_name: String,
    #[serde(rename = "httpContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_context: Option<HttpContext>,
    #[serde(rename = "mqttContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mqtt_context: Option<MqttContext>,
    #[serde(rename = "tlsContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_context: Option<TlsContext>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "tokenSignature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_signature: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpContext {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "queryString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MqttContext {
    #[serde(rename = "clientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TlsContext {
    #[serde(rename = "serverName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestInvokeAuthorizerResponse {
    #[serde(rename = "disconnectAfterInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_after_in_seconds: Option<i32>,
    #[serde(rename = "isAuthenticated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_authenticated: Option<bool>,
    #[serde(rename = "policyDocuments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_documents: Option<Vec<String>>,
    #[serde(rename = "principalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "refreshAfterInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_after_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransferCertificateRequest {
    #[serde(rename = "certificateId")]
    #[serde(default)]
    pub certificate_id: String,
    #[serde(rename = "targetAwsAccount")]
    #[serde(default)]
    pub target_aws_account: String,
    #[serde(rename = "transferMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransferCertificateResponse {
    #[serde(rename = "transferredCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferred_certificate_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccountAuditConfigurationRequest {
    #[serde(rename = "auditCheckConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_check_configurations:
        Option<std::collections::HashMap<String, AuditCheckConfiguration>>,
    #[serde(rename = "auditNotificationTargetConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_notification_target_configurations:
        Option<std::collections::HashMap<String, AuditNotificationTarget>>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccountAuditConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAuditSuppressionRequest {
    #[serde(rename = "checkName")]
    #[serde(default)]
    pub check_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "expirationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    pub resource_identifier: ResourceIdentifier,
    #[serde(rename = "suppressIndefinitely")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_indefinitely: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAuditSuppressionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAuthorizerRequest {
    #[serde(rename = "authorizerFunctionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_function_arn: Option<String>,
    #[serde(rename = "authorizerName")]
    #[serde(default)]
    pub authorizer_name: String,
    #[serde(rename = "enableCachingForHttp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_caching_for_http: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "tokenKeyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key_name: Option<String>,
    #[serde(rename = "tokenSigningPublicKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_signing_public_keys: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAuthorizerResponse {
    #[serde(rename = "authorizerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_arn: Option<String>,
    #[serde(rename = "authorizerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBillingGroupRequest {
    #[serde(rename = "billingGroupName")]
    #[serde(default)]
    pub billing_group_name: String,
    #[serde(rename = "billingGroupProperties")]
    #[serde(default)]
    pub billing_group_properties: BillingGroupProperties,
    #[serde(rename = "expectedVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBillingGroupResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCACertificateRequest {
    #[serde(rename = "certificateId")]
    #[serde(default)]
    pub certificate_id: String,
    #[serde(rename = "newAutoRegistrationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_auto_registration_status: Option<String>,
    #[serde(rename = "newStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_status: Option<String>,
    #[serde(rename = "registrationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_config: Option<RegistrationConfig>,
    #[serde(rename = "removeAutoRegistration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_auto_registration: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCertificateProviderRequest {
    #[serde(rename = "accountDefaultForOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_default_for_operations: Option<Vec<String>>,
    #[serde(rename = "certificateProviderName")]
    #[serde(default)]
    pub certificate_provider_name: String,
    #[serde(rename = "lambdaFunctionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCertificateProviderResponse {
    #[serde(rename = "certificateProviderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_provider_arn: Option<String>,
    #[serde(rename = "certificateProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_provider_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCertificateRequest {
    #[serde(rename = "certificateId")]
    #[serde(default)]
    pub certificate_id: String,
    #[serde(rename = "newStatus")]
    #[serde(default)]
    pub new_status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCommandRequest {
    #[serde(rename = "commandId")]
    #[serde(default)]
    pub command_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCommandResponse {
    #[serde(rename = "commandId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCustomMetricRequest {
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: String,
    #[serde(rename = "metricName")]
    #[serde(default)]
    pub metric_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCustomMetricResponse {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "metricArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_arn: Option<String>,
    #[serde(rename = "metricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "metricType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDimensionRequest {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "stringValues")]
    #[serde(default)]
    pub string_values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDimensionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "stringValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_values: Option<Vec<String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDomainConfigurationRequest {
    #[serde(rename = "applicationProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_protocol: Option<String>,
    #[serde(rename = "authenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "authorizerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_config: Option<AuthorizerConfig>,
    #[serde(rename = "clientCertificateConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_config: Option<ClientCertificateConfig>,
    #[serde(rename = "domainConfigurationName")]
    #[serde(default)]
    pub domain_configuration_name: String,
    #[serde(rename = "domainConfigurationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_configuration_status: Option<String>,
    #[serde(rename = "removeAuthorizerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_authorizer_config: Option<bool>,
    #[serde(rename = "serverCertificateConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_config: Option<ServerCertificateConfig>,
    #[serde(rename = "tlsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_config: Option<TlsConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDomainConfigurationResponse {
    #[serde(rename = "domainConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_configuration_arn: Option<String>,
    #[serde(rename = "domainConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_configuration_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDynamicThingGroupRequest {
    #[serde(rename = "expectedVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_version: Option<i64>,
    #[serde(rename = "indexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "queryString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
    #[serde(rename = "queryVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_version: Option<String>,
    #[serde(rename = "thingGroupName")]
    #[serde(default)]
    pub thing_group_name: String,
    #[serde(rename = "thingGroupProperties")]
    #[serde(default)]
    pub thing_group_properties: ThingGroupProperties,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDynamicThingGroupResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEncryptionConfigurationRequest {
    #[serde(rename = "encryptionType")]
    #[serde(default)]
    pub encryption_type: String,
    #[serde(rename = "kmsAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_access_role_arn: Option<String>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEncryptionConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEventConfigurationsRequest {
    #[serde(rename = "eventConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_configurations: Option<std::collections::HashMap<String, Configuration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEventConfigurationsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFleetMetricRequest {
    #[serde(rename = "aggregationField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_field: Option<String>,
    #[serde(rename = "aggregationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<AggregationType>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "expectedVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_version: Option<i64>,
    #[serde(rename = "indexName")]
    #[serde(default)]
    pub index_name: String,
    #[serde(rename = "metricName")]
    #[serde(default)]
    pub metric_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(rename = "queryString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
    #[serde(rename = "queryVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIndexingConfigurationRequest {
    #[serde(rename = "thingGroupIndexingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_indexing_configuration: Option<ThingGroupIndexingConfiguration>,
    #[serde(rename = "thingIndexingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_indexing_configuration: Option<ThingIndexingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIndexingConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateJobRequest {
    #[serde(rename = "abortConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abort_config: Option<AbortConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "jobExecutionsRetryConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_executions_retry_config: Option<JobExecutionsRetryConfig>,
    #[serde(rename = "jobExecutionsRolloutConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_executions_rollout_config: Option<JobExecutionsRolloutConfig>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "namespaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
    #[serde(rename = "presignedUrlConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presigned_url_config: Option<PresignedUrlConfig>,
    #[serde(rename = "timeoutConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_config: Option<TimeoutConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMitigationActionRequest {
    #[serde(rename = "actionName")]
    #[serde(default)]
    pub action_name: String,
    #[serde(rename = "actionParams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_params: Option<MitigationActionParams>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMitigationActionResponse {
    #[serde(rename = "actionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_arn: Option<String>,
    #[serde(rename = "actionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePackageConfigurationRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "versionUpdateByJobsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_update_by_jobs_config: Option<VersionUpdateByJobsConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePackageConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePackageRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "defaultVersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "packageName")]
    #[serde(default)]
    pub package_name: String,
    #[serde(rename = "unsetDefaultVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unset_default_version: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePackageResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePackageVersionRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<PackageVersionArtifact>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "packageName")]
    #[serde(default)]
    pub package_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe: Option<String>,
    #[serde(rename = "versionName")]
    #[serde(default)]
    pub version_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePackageVersionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProvisioningTemplateRequest {
    #[serde(rename = "defaultVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version_id: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "preProvisioningHook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_provisioning_hook: Option<ProvisioningHook>,
    #[serde(rename = "provisioningRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_role_arn: Option<String>,
    #[serde(rename = "removePreProvisioningHook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_pre_provisioning_hook: Option<bool>,
    #[serde(rename = "templateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProvisioningTemplateResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRoleAliasRequest {
    #[serde(rename = "credentialDurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_duration_seconds: Option<i32>,
    #[serde(rename = "roleAlias")]
    #[serde(default)]
    pub role_alias: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRoleAliasResponse {
    #[serde(rename = "roleAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_alias: Option<String>,
    #[serde(rename = "roleAliasArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_alias_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateScheduledAuditRequest {
    #[serde(rename = "dayOfMonth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_month: Option<String>,
    #[serde(rename = "dayOfWeek")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    #[serde(rename = "scheduledAuditName")]
    #[serde(default)]
    pub scheduled_audit_name: String,
    #[serde(rename = "targetCheckNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_check_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateScheduledAuditResponse {
    #[serde(rename = "scheduledAuditArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_audit_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSecurityProfileRequest {
    #[serde(rename = "additionalMetricsToRetain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_metrics_to_retain: Option<Vec<String>>,
    #[serde(rename = "additionalMetricsToRetainV2")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_metrics_to_retain_v2: Option<Vec<MetricToRetain>>,
    #[serde(rename = "alertTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_targets: Option<std::collections::HashMap<String, AlertTarget>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behaviors: Option<Vec<Behavior>>,
    #[serde(rename = "deleteAdditionalMetricsToRetain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_additional_metrics_to_retain: Option<bool>,
    #[serde(rename = "deleteAlertTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_alert_targets: Option<bool>,
    #[serde(rename = "deleteBehaviors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_behaviors: Option<bool>,
    #[serde(rename = "deleteMetricsExportConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_metrics_export_config: Option<bool>,
    #[serde(rename = "expectedVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_version: Option<i64>,
    #[serde(rename = "metricsExportConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_export_config: Option<MetricsExportConfig>,
    #[serde(rename = "securityProfileDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_description: Option<String>,
    #[serde(rename = "securityProfileName")]
    #[serde(default)]
    pub security_profile_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSecurityProfileResponse {
    #[serde(rename = "additionalMetricsToRetain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_metrics_to_retain: Option<Vec<String>>,
    #[serde(rename = "additionalMetricsToRetainV2")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_metrics_to_retain_v2: Option<Vec<MetricToRetain>>,
    #[serde(rename = "alertTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_targets: Option<std::collections::HashMap<String, AlertTarget>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behaviors: Option<Vec<Behavior>>,
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "metricsExportConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_export_config: Option<MetricsExportConfig>,
    #[serde(rename = "securityProfileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_arn: Option<String>,
    #[serde(rename = "securityProfileDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_description: Option<String>,
    #[serde(rename = "securityProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStreamRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<StreamFile>>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "streamId")]
    #[serde(default)]
    pub stream_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStreamResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "streamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    #[serde(rename = "streamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(rename = "streamVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThingGroupRequest {
    #[serde(rename = "expectedVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_version: Option<i64>,
    #[serde(rename = "thingGroupName")]
    #[serde(default)]
    pub thing_group_name: String,
    #[serde(rename = "thingGroupProperties")]
    #[serde(default)]
    pub thing_group_properties: ThingGroupProperties,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThingGroupResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThingGroupsForThingRequest {
    #[serde(rename = "overrideDynamicGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_dynamic_groups: Option<bool>,
    #[serde(rename = "thingGroupsToAdd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_groups_to_add: Option<Vec<String>>,
    #[serde(rename = "thingGroupsToRemove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_groups_to_remove: Option<Vec<String>>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThingGroupsForThingResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThingRequest {
    #[serde(rename = "attributePayload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_payload: Option<AttributePayload>,
    #[serde(rename = "expectedVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_version: Option<i64>,
    #[serde(rename = "removeThingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_thing_type: Option<bool>,
    #[serde(rename = "thingName")]
    #[serde(default)]
    pub thing_name: String,
    #[serde(rename = "thingTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThingResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThingTypeRequest {
    #[serde(rename = "thingTypeName")]
    #[serde(default)]
    pub thing_type_name: String,
    #[serde(rename = "thingTypeProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_properties: Option<ThingTypeProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThingTypeResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTopicRuleDestinationRequest {
    #[serde(default)]
    pub arn: String,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTopicRuleDestinationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidateSecurityProfileBehaviorsRequest {
    #[serde(default)]
    pub behaviors: Vec<Behavior>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidateSecurityProfileBehaviorsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
    #[serde(rename = "validationErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<ValidationError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidationError {
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
