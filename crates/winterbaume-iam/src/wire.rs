//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-iam

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;
fn strip_outer_element(xml: &str) -> &str {
    // Find the end of the opening tag
    if let Some(close_pos) = xml.find('>') {
        let inner_start = close_pos + 1;
        // Find the last closing tag
        if let Some(last_open) = xml.rfind('<') {
            if last_open >= inner_start {
                return &xml[inner_start..last_open];
            }
        }
    }
    xml
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_accept_delegation_request_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AcceptDelegationRequestResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AcceptDelegationRequestResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_add_client_i_d_to_open_i_d_connect_provider_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AddClientIDToOpenIDConnectProviderResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AddClientIDToOpenIDConnectProviderResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_add_role_to_instance_profile_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AddRoleToInstanceProfileResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AddRoleToInstanceProfileResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_add_user_to_group_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AddUserToGroupResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AddUserToGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_associate_delegation_request_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AssociateDelegationRequestResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AssociateDelegationRequestResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_attach_group_policy_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AttachGroupPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AttachGroupPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_attach_role_policy_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AttachRolePolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AttachRolePolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_attach_user_policy_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AttachUserPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AttachUserPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_change_password_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ChangePasswordResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ChangePasswordResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_access_key_response(result: &CreateAccessKeyResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateAccessKeyResult>{inner_xml}</CreateAccessKeyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateAccessKeyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateAccessKeyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_create_account_alias_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateAccountAliasResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateAccountAliasResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_delegation_request_response(
    result: &CreateDelegationRequestResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateDelegationRequestResult>{inner_xml}</CreateDelegationRequestResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateDelegationRequestResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateDelegationRequestResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_group_response(result: &CreateGroupResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateGroupResult>{inner_xml}</CreateGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateGroupResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_instance_profile_response(
    result: &CreateInstanceProfileResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateInstanceProfileResult>{inner_xml}</CreateInstanceProfileResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateInstanceProfileResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateInstanceProfileResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_login_profile_response(
    result: &CreateLoginProfileResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateLoginProfileResult>{inner_xml}</CreateLoginProfileResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateLoginProfileResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateLoginProfileResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_open_i_d_connect_provider_response(
    result: &CreateOpenIDConnectProviderResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<CreateOpenIDConnectProviderResult>{inner_xml}</CreateOpenIDConnectProviderResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateOpenIDConnectProviderResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateOpenIDConnectProviderResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_policy_response(result: &CreatePolicyResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreatePolicyResult>{inner_xml}</CreatePolicyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreatePolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreatePolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_policy_version_response(
    result: &CreatePolicyVersionResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreatePolicyVersionResult>{inner_xml}</CreatePolicyVersionResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreatePolicyVersionResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreatePolicyVersionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_role_response(result: &CreateRoleResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateRoleResult>{inner_xml}</CreateRoleResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateRoleResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateRoleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_s_a_m_l_provider_response(
    result: &CreateSAMLProviderResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateSAMLProviderResult>{inner_xml}</CreateSAMLProviderResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateSAMLProviderResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateSAMLProviderResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_service_linked_role_response(
    result: &CreateServiceLinkedRoleResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateServiceLinkedRoleResult>{inner_xml}</CreateServiceLinkedRoleResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateServiceLinkedRoleResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateServiceLinkedRoleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_service_specific_credential_response(
    result: &CreateServiceSpecificCredentialResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<CreateServiceSpecificCredentialResult>{inner_xml}</CreateServiceSpecificCredentialResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateServiceSpecificCredentialResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateServiceSpecificCredentialResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_user_response(result: &CreateUserResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateUserResult>{inner_xml}</CreateUserResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateUserResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateUserResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_virtual_m_f_a_device_response(
    result: &CreateVirtualMFADeviceResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateVirtualMFADeviceResult>{inner_xml}</CreateVirtualMFADeviceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateVirtualMFADeviceResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateVirtualMFADeviceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_deactivate_m_f_a_device_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeactivateMFADeviceResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeactivateMFADeviceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_access_key_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteAccessKeyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteAccessKeyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_account_alias_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteAccountAliasResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteAccountAliasResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_account_password_policy_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteAccountPasswordPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteAccountPasswordPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_group_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteGroupResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_group_policy_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteGroupPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteGroupPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_instance_profile_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteInstanceProfileResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteInstanceProfileResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_login_profile_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteLoginProfileResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteLoginProfileResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_open_i_d_connect_provider_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteOpenIDConnectProviderResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteOpenIDConnectProviderResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_policy_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeletePolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeletePolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_policy_version_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeletePolicyVersionResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeletePolicyVersionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_role_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteRoleResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteRoleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_role_permissions_boundary_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteRolePermissionsBoundaryResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteRolePermissionsBoundaryResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_role_policy_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteRolePolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteRolePolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_s_a_m_l_provider_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteSAMLProviderResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteSAMLProviderResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_s_s_h_public_key_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteSSHPublicKeyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteSSHPublicKeyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_server_certificate_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteServerCertificateResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteServerCertificateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_service_linked_role_response(
    result: &DeleteServiceLinkedRoleResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DeleteServiceLinkedRoleResult>{inner_xml}</DeleteServiceLinkedRoleResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteServiceLinkedRoleResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteServiceLinkedRoleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_service_specific_credential_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteServiceSpecificCredentialResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteServiceSpecificCredentialResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_signing_certificate_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteSigningCertificateResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteSigningCertificateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_user_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteUserResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteUserResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_user_permissions_boundary_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteUserPermissionsBoundaryResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteUserPermissionsBoundaryResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_user_policy_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteUserPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteUserPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_virtual_m_f_a_device_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteVirtualMFADeviceResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteVirtualMFADeviceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_detach_group_policy_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DetachGroupPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DetachGroupPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_detach_role_policy_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DetachRolePolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DetachRolePolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_detach_user_policy_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DetachUserPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DetachUserPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_disable_organizations_root_credentials_management_response(
    result: &DisableOrganizationsRootCredentialsManagementResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DisableOrganizationsRootCredentialsManagementResult>{inner_xml}</DisableOrganizationsRootCredentialsManagementResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DisableOrganizationsRootCredentialsManagementResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DisableOrganizationsRootCredentialsManagementResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_disable_organizations_root_sessions_response(
    result: &DisableOrganizationsRootSessionsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DisableOrganizationsRootSessionsResult>{inner_xml}</DisableOrganizationsRootSessionsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DisableOrganizationsRootSessionsResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DisableOrganizationsRootSessionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_disable_outbound_web_identity_federation_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DisableOutboundWebIdentityFederationResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DisableOutboundWebIdentityFederationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_enable_m_f_a_device_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<EnableMFADeviceResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</EnableMFADeviceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_enable_organizations_root_credentials_management_response(
    result: &EnableOrganizationsRootCredentialsManagementResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<EnableOrganizationsRootCredentialsManagementResult>{inner_xml}</EnableOrganizationsRootCredentialsManagementResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<EnableOrganizationsRootCredentialsManagementResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</EnableOrganizationsRootCredentialsManagementResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_enable_organizations_root_sessions_response(
    result: &EnableOrganizationsRootSessionsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<EnableOrganizationsRootSessionsResult>{inner_xml}</EnableOrganizationsRootSessionsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<EnableOrganizationsRootSessionsResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</EnableOrganizationsRootSessionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_enable_outbound_web_identity_federation_response(
    result: &EnableOutboundWebIdentityFederationResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<EnableOutboundWebIdentityFederationResult>{inner_xml}</EnableOutboundWebIdentityFederationResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<EnableOutboundWebIdentityFederationResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</EnableOutboundWebIdentityFederationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_generate_credential_report_response(
    result: &GenerateCredentialReportResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<GenerateCredentialReportResult>{inner_xml}</GenerateCredentialReportResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GenerateCredentialReportResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GenerateCredentialReportResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_generate_organizations_access_report_response(
    result: &GenerateOrganizationsAccessReportResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<GenerateOrganizationsAccessReportResult>{inner_xml}</GenerateOrganizationsAccessReportResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GenerateOrganizationsAccessReportResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GenerateOrganizationsAccessReportResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_generate_service_last_accessed_details_response(
    result: &GenerateServiceLastAccessedDetailsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<GenerateServiceLastAccessedDetailsResult>{inner_xml}</GenerateServiceLastAccessedDetailsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GenerateServiceLastAccessedDetailsResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GenerateServiceLastAccessedDetailsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_access_key_last_used_response(
    result: &GetAccessKeyLastUsedResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<GetAccessKeyLastUsedResult>{inner_xml}</GetAccessKeyLastUsedResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetAccessKeyLastUsedResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetAccessKeyLastUsedResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_account_authorization_details_response(
    result: &GetAccountAuthorizationDetailsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<GetAccountAuthorizationDetailsResult>{inner_xml}</GetAccountAuthorizationDetailsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetAccountAuthorizationDetailsResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetAccountAuthorizationDetailsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_account_password_policy_response(
    result: &GetAccountPasswordPolicyResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<GetAccountPasswordPolicyResult>{inner_xml}</GetAccountPasswordPolicyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetAccountPasswordPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetAccountPasswordPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_account_summary_response(result: &GetAccountSummaryResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetAccountSummaryResult>{inner_xml}</GetAccountSummaryResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetAccountSummaryResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetAccountSummaryResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_context_keys_for_custom_policy_response(
    result: &GetContextKeysForPolicyResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<GetContextKeysForCustomPolicyResult>{inner_xml}</GetContextKeysForCustomPolicyResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetContextKeysForCustomPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetContextKeysForCustomPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_context_keys_for_principal_policy_response(
    result: &GetContextKeysForPolicyResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<GetContextKeysForPrincipalPolicyResult>{inner_xml}</GetContextKeysForPrincipalPolicyResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetContextKeysForPrincipalPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetContextKeysForPrincipalPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_credential_report_response(
    result: &GetCredentialReportResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetCredentialReportResult>{inner_xml}</GetCredentialReportResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetCredentialReportResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetCredentialReportResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_delegation_request_response(
    result: &GetDelegationRequestResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<GetDelegationRequestResult>{inner_xml}</GetDelegationRequestResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetDelegationRequestResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetDelegationRequestResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_group_response(result: &GetGroupResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetGroupResult>{inner_xml}</GetGroupResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetGroupResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_group_policy_response(result: &GetGroupPolicyResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetGroupPolicyResult>{inner_xml}</GetGroupPolicyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetGroupPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetGroupPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_human_readable_summary_response(
    result: &GetHumanReadableSummaryResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<GetHumanReadableSummaryResult>{inner_xml}</GetHumanReadableSummaryResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetHumanReadableSummaryResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetHumanReadableSummaryResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_instance_profile_response(
    result: &GetInstanceProfileResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetInstanceProfileResult>{inner_xml}</GetInstanceProfileResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetInstanceProfileResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetInstanceProfileResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_login_profile_response(result: &GetLoginProfileResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetLoginProfileResult>{inner_xml}</GetLoginProfileResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetLoginProfileResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetLoginProfileResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_m_f_a_device_response(result: &GetMFADeviceResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetMFADeviceResult>{inner_xml}</GetMFADeviceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetMFADeviceResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetMFADeviceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_open_i_d_connect_provider_response(
    result: &GetOpenIDConnectProviderResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<GetOpenIDConnectProviderResult>{inner_xml}</GetOpenIDConnectProviderResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetOpenIDConnectProviderResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetOpenIDConnectProviderResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_organizations_access_report_response(
    result: &GetOrganizationsAccessReportResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<GetOrganizationsAccessReportResult>{inner_xml}</GetOrganizationsAccessReportResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetOrganizationsAccessReportResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetOrganizationsAccessReportResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_outbound_web_identity_federation_info_response(
    result: &GetOutboundWebIdentityFederationInfoResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<GetOutboundWebIdentityFederationInfoResult>{inner_xml}</GetOutboundWebIdentityFederationInfoResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetOutboundWebIdentityFederationInfoResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetOutboundWebIdentityFederationInfoResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_policy_response(result: &GetPolicyResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetPolicyResult>{inner_xml}</GetPolicyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_policy_version_response(result: &GetPolicyVersionResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetPolicyVersionResult>{inner_xml}</GetPolicyVersionResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetPolicyVersionResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetPolicyVersionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_role_response(result: &GetRoleResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetRoleResult>{inner_xml}</GetRoleResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetRoleResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetRoleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_role_policy_response(result: &GetRolePolicyResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetRolePolicyResult>{inner_xml}</GetRolePolicyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetRolePolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetRolePolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_s_a_m_l_provider_response(result: &GetSAMLProviderResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetSAMLProviderResult>{inner_xml}</GetSAMLProviderResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetSAMLProviderResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetSAMLProviderResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_s_s_h_public_key_response(result: &GetSSHPublicKeyResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetSSHPublicKeyResult>{inner_xml}</GetSSHPublicKeyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetSSHPublicKeyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetSSHPublicKeyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_server_certificate_response(
    result: &GetServerCertificateResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<GetServerCertificateResult>{inner_xml}</GetServerCertificateResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetServerCertificateResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetServerCertificateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_service_last_accessed_details_response(
    result: &GetServiceLastAccessedDetailsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<GetServiceLastAccessedDetailsResult>{inner_xml}</GetServiceLastAccessedDetailsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetServiceLastAccessedDetailsResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetServiceLastAccessedDetailsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_service_last_accessed_details_with_entities_response(
    result: &GetServiceLastAccessedDetailsWithEntitiesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<GetServiceLastAccessedDetailsWithEntitiesResult>{inner_xml}</GetServiceLastAccessedDetailsWithEntitiesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetServiceLastAccessedDetailsWithEntitiesResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetServiceLastAccessedDetailsWithEntitiesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_service_linked_role_deletion_status_response(
    result: &GetServiceLinkedRoleDeletionStatusResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<GetServiceLinkedRoleDeletionStatusResult>{inner_xml}</GetServiceLinkedRoleDeletionStatusResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetServiceLinkedRoleDeletionStatusResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetServiceLinkedRoleDeletionStatusResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_user_response(result: &GetUserResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetUserResult>{inner_xml}</GetUserResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetUserResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetUserResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_user_policy_response(result: &GetUserPolicyResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetUserPolicyResult>{inner_xml}</GetUserPolicyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetUserPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetUserPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_access_keys_response(result: &ListAccessKeysResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListAccessKeysResult>{inner_xml}</ListAccessKeysResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListAccessKeysResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListAccessKeysResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_account_aliases_response(
    result: &ListAccountAliasesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListAccountAliasesResult>{inner_xml}</ListAccountAliasesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListAccountAliasesResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListAccountAliasesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_attached_group_policies_response(
    result: &ListAttachedGroupPoliciesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListAttachedGroupPoliciesResult>{inner_xml}</ListAttachedGroupPoliciesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListAttachedGroupPoliciesResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListAttachedGroupPoliciesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_attached_role_policies_response(
    result: &ListAttachedRolePoliciesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListAttachedRolePoliciesResult>{inner_xml}</ListAttachedRolePoliciesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListAttachedRolePoliciesResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListAttachedRolePoliciesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_attached_user_policies_response(
    result: &ListAttachedUserPoliciesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListAttachedUserPoliciesResult>{inner_xml}</ListAttachedUserPoliciesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListAttachedUserPoliciesResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListAttachedUserPoliciesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_delegation_requests_response(
    result: &ListDelegationRequestsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListDelegationRequestsResult>{inner_xml}</ListDelegationRequestsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListDelegationRequestsResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListDelegationRequestsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_entities_for_policy_response(
    result: &ListEntitiesForPolicyResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListEntitiesForPolicyResult>{inner_xml}</ListEntitiesForPolicyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListEntitiesForPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListEntitiesForPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_group_policies_response(result: &ListGroupPoliciesResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListGroupPoliciesResult>{inner_xml}</ListGroupPoliciesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListGroupPoliciesResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListGroupPoliciesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_groups_response(result: &ListGroupsResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListGroupsResult>{inner_xml}</ListGroupsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListGroupsResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListGroupsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_groups_for_user_response(result: &ListGroupsForUserResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListGroupsForUserResult>{inner_xml}</ListGroupsForUserResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListGroupsForUserResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListGroupsForUserResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_instance_profile_tags_response(
    result: &ListInstanceProfileTagsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListInstanceProfileTagsResult>{inner_xml}</ListInstanceProfileTagsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListInstanceProfileTagsResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListInstanceProfileTagsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_instance_profiles_response(
    result: &ListInstanceProfilesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListInstanceProfilesResult>{inner_xml}</ListInstanceProfilesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListInstanceProfilesResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListInstanceProfilesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_instance_profiles_for_role_response(
    result: &ListInstanceProfilesForRoleResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ListInstanceProfilesForRoleResult>{inner_xml}</ListInstanceProfilesForRoleResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListInstanceProfilesForRoleResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListInstanceProfilesForRoleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_m_f_a_device_tags_response(
    result: &ListMFADeviceTagsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListMFADeviceTagsResult>{inner_xml}</ListMFADeviceTagsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListMFADeviceTagsResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListMFADeviceTagsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_m_f_a_devices_response(result: &ListMFADevicesResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListMFADevicesResult>{inner_xml}</ListMFADevicesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListMFADevicesResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListMFADevicesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_open_i_d_connect_provider_tags_response(
    result: &ListOpenIDConnectProviderTagsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ListOpenIDConnectProviderTagsResult>{inner_xml}</ListOpenIDConnectProviderTagsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListOpenIDConnectProviderTagsResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListOpenIDConnectProviderTagsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_open_i_d_connect_providers_response(
    result: &ListOpenIDConnectProvidersResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListOpenIDConnectProvidersResult>{inner_xml}</ListOpenIDConnectProvidersResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListOpenIDConnectProvidersResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListOpenIDConnectProvidersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_organizations_features_response(
    result: &ListOrganizationsFeaturesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListOrganizationsFeaturesResult>{inner_xml}</ListOrganizationsFeaturesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListOrganizationsFeaturesResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListOrganizationsFeaturesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_policies_response(result: &ListPoliciesResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListPoliciesResult>{inner_xml}</ListPoliciesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListPoliciesResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListPoliciesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_policies_granting_service_access_response(
    result: &ListPoliciesGrantingServiceAccessResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ListPoliciesGrantingServiceAccessResult>{inner_xml}</ListPoliciesGrantingServiceAccessResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListPoliciesGrantingServiceAccessResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListPoliciesGrantingServiceAccessResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_policy_tags_response(result: &ListPolicyTagsResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListPolicyTagsResult>{inner_xml}</ListPolicyTagsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListPolicyTagsResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListPolicyTagsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_policy_versions_response(
    result: &ListPolicyVersionsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListPolicyVersionsResult>{inner_xml}</ListPolicyVersionsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListPolicyVersionsResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListPolicyVersionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_role_policies_response(result: &ListRolePoliciesResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListRolePoliciesResult>{inner_xml}</ListRolePoliciesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListRolePoliciesResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListRolePoliciesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_role_tags_response(result: &ListRoleTagsResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListRoleTagsResult>{inner_xml}</ListRoleTagsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListRoleTagsResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListRoleTagsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_roles_response(result: &ListRolesResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListRolesResult>{inner_xml}</ListRolesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListRolesResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListRolesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_s_a_m_l_provider_tags_response(
    result: &ListSAMLProviderTagsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListSAMLProviderTagsResult>{inner_xml}</ListSAMLProviderTagsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListSAMLProviderTagsResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListSAMLProviderTagsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_s_a_m_l_providers_response(
    result: &ListSAMLProvidersResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListSAMLProvidersResult>{inner_xml}</ListSAMLProvidersResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListSAMLProvidersResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListSAMLProvidersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_s_s_h_public_keys_response(
    result: &ListSSHPublicKeysResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListSSHPublicKeysResult>{inner_xml}</ListSSHPublicKeysResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListSSHPublicKeysResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListSSHPublicKeysResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_server_certificate_tags_response(
    result: &ListServerCertificateTagsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListServerCertificateTagsResult>{inner_xml}</ListServerCertificateTagsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListServerCertificateTagsResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListServerCertificateTagsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_server_certificates_response(
    result: &ListServerCertificatesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListServerCertificatesResult>{inner_xml}</ListServerCertificatesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListServerCertificatesResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListServerCertificatesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_service_specific_credentials_response(
    result: &ListServiceSpecificCredentialsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ListServiceSpecificCredentialsResult>{inner_xml}</ListServiceSpecificCredentialsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListServiceSpecificCredentialsResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListServiceSpecificCredentialsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_signing_certificates_response(
    result: &ListSigningCertificatesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListSigningCertificatesResult>{inner_xml}</ListSigningCertificatesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListSigningCertificatesResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListSigningCertificatesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_user_policies_response(result: &ListUserPoliciesResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListUserPoliciesResult>{inner_xml}</ListUserPoliciesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListUserPoliciesResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListUserPoliciesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_user_tags_response(result: &ListUserTagsResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListUserTagsResult>{inner_xml}</ListUserTagsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListUserTagsResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListUserTagsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_users_response(result: &ListUsersResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListUsersResult>{inner_xml}</ListUsersResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListUsersResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListUsersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_virtual_m_f_a_devices_response(
    result: &ListVirtualMFADevicesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListVirtualMFADevicesResult>{inner_xml}</ListVirtualMFADevicesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListVirtualMFADevicesResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListVirtualMFADevicesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_put_group_policy_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PutGroupPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PutGroupPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_put_role_permissions_boundary_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PutRolePermissionsBoundaryResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PutRolePermissionsBoundaryResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_put_role_policy_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PutRolePolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PutRolePolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_put_user_permissions_boundary_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PutUserPermissionsBoundaryResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PutUserPermissionsBoundaryResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_put_user_policy_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PutUserPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PutUserPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_reject_delegation_request_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RejectDelegationRequestResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RejectDelegationRequestResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_remove_client_i_d_from_open_i_d_connect_provider_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RemoveClientIDFromOpenIDConnectProviderResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RemoveClientIDFromOpenIDConnectProviderResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_remove_role_from_instance_profile_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RemoveRoleFromInstanceProfileResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RemoveRoleFromInstanceProfileResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_remove_user_from_group_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RemoveUserFromGroupResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RemoveUserFromGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_reset_service_specific_credential_response(
    result: &ResetServiceSpecificCredentialResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ResetServiceSpecificCredentialResult>{inner_xml}</ResetServiceSpecificCredentialResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ResetServiceSpecificCredentialResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ResetServiceSpecificCredentialResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_resync_m_f_a_device_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ResyncMFADeviceResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ResyncMFADeviceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_send_delegation_token_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SendDelegationTokenResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SendDelegationTokenResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_set_default_policy_version_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetDefaultPolicyVersionResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetDefaultPolicyVersionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_set_security_token_service_preferences_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetSecurityTokenServicePreferencesResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetSecurityTokenServicePreferencesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_simulate_custom_policy_response(result: &SimulatePolicyResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<SimulateCustomPolicyResult>{inner_xml}</SimulateCustomPolicyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SimulateCustomPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SimulateCustomPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_simulate_principal_policy_response(
    result: &SimulatePolicyResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<SimulatePrincipalPolicyResult>{inner_xml}</SimulatePrincipalPolicyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SimulatePrincipalPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SimulatePrincipalPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_tag_instance_profile_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<TagInstanceProfileResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</TagInstanceProfileResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_tag_m_f_a_device_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<TagMFADeviceResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</TagMFADeviceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_tag_open_i_d_connect_provider_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<TagOpenIDConnectProviderResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</TagOpenIDConnectProviderResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_tag_policy_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<TagPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</TagPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_tag_role_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<TagRoleResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</TagRoleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_tag_s_a_m_l_provider_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<TagSAMLProviderResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</TagSAMLProviderResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_tag_server_certificate_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<TagServerCertificateResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</TagServerCertificateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_tag_user_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<TagUserResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</TagUserResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_untag_instance_profile_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UntagInstanceProfileResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UntagInstanceProfileResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_untag_m_f_a_device_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UntagMFADeviceResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UntagMFADeviceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_untag_open_i_d_connect_provider_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UntagOpenIDConnectProviderResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UntagOpenIDConnectProviderResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_untag_policy_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UntagPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UntagPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_untag_role_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UntagRoleResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UntagRoleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_untag_s_a_m_l_provider_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UntagSAMLProviderResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UntagSAMLProviderResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_untag_server_certificate_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UntagServerCertificateResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UntagServerCertificateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_untag_user_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UntagUserResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UntagUserResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_update_access_key_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateAccessKeyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateAccessKeyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_update_account_password_policy_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateAccountPasswordPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateAccountPasswordPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_update_assume_role_policy_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateAssumeRolePolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateAssumeRolePolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_update_delegation_request_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateDelegationRequestResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateDelegationRequestResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_update_group_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateGroupResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_update_login_profile_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateLoginProfileResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateLoginProfileResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_update_open_i_d_connect_provider_thumbprint_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateOpenIDConnectProviderThumbprintResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateOpenIDConnectProviderThumbprintResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_update_role_response(result: &UpdateRoleResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<UpdateRoleResult>{inner_xml}</UpdateRoleResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateRoleResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateRoleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_update_role_description_response(
    result: &UpdateRoleDescriptionResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<UpdateRoleDescriptionResult>{inner_xml}</UpdateRoleDescriptionResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateRoleDescriptionResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateRoleDescriptionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_update_s_a_m_l_provider_response(
    result: &UpdateSAMLProviderResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<UpdateSAMLProviderResult>{inner_xml}</UpdateSAMLProviderResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateSAMLProviderResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateSAMLProviderResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_update_s_s_h_public_key_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateSSHPublicKeyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateSSHPublicKeyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_update_server_certificate_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateServerCertificateResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateServerCertificateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_update_service_specific_credential_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateServiceSpecificCredentialResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateServiceSpecificCredentialResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_update_signing_certificate_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateSigningCertificateResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateSigningCertificateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_update_user_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateUserResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateUserResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_upload_s_s_h_public_key_response(
    result: &UploadSSHPublicKeyResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<UploadSSHPublicKeyResult>{inner_xml}</UploadSSHPublicKeyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UploadSSHPublicKeyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UploadSSHPublicKeyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_upload_server_certificate_response(
    result: &UploadServerCertificateResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<UploadServerCertificateResult>{inner_xml}</UploadServerCertificateResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UploadServerCertificateResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UploadServerCertificateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_upload_signing_certificate_response(
    result: &UploadSigningCertificateResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<UploadSigningCertificateResult>{inner_xml}</UploadSigningCertificateResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UploadSigningCertificateResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UploadSigningCertificateResponse>"#
    );
    MockResponse::xml(200, xml)
}

fn deserialize_context_entry_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ContextEntry>, String> {
    let mut item = ContextEntry::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.ContextKeyName")) {
        item.context_key_name = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ContextKeyType")) {
        item.context_key_type = Some(value.to_string());
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.ContextKeyValues");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.context_key_values = Some(ContextKeyValueListType { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_policy_parameter_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<PolicyParameter>, String> {
    let mut item = PolicyParameter::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Name")) {
        item.name = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Type")) {
        item.r#type = Some(value.to_string());
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Values");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.values = Some(policyParameterValuesListType { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_tag_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Tag>, String> {
    let mut item = Tag::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Key")) {
        item.key = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Value")) {
        item.value = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_delegation_permission_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<DelegationPermission>, String> {
    let mut item = DelegationPermission::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Parameters");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_policy_parameter_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.parameters = Some(policyParameterListType { items: sub_items });
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.PolicyTemplateArn")) {
        item.policy_template_arn = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

/// Deserialize awsQuery request for AcceptDelegationRequest.
pub fn deserialize_accept_delegation_request_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AcceptDelegationRequestRequest, String> {
    let mut input = AcceptDelegationRequestRequest::default();
    if let Some(value) = params.get("DelegationRequestId") {
        input.delegation_request_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for AddClientIDToOpenIDConnectProvider.
pub fn deserialize_add_client_i_d_to_open_i_d_connect_provider_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AddClientIDToOpenIDConnectProviderRequest, String> {
    let mut input = AddClientIDToOpenIDConnectProviderRequest::default();
    if let Some(value) = params.get("ClientID") {
        input.client_i_d = value.to_string();
    }
    if let Some(value) = params.get("OpenIDConnectProviderArn") {
        input.open_i_d_connect_provider_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for AddRoleToInstanceProfile.
pub fn deserialize_add_role_to_instance_profile_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AddRoleToInstanceProfileRequest, String> {
    let mut input = AddRoleToInstanceProfileRequest::default();
    if let Some(value) = params.get("InstanceProfileName") {
        input.instance_profile_name = value.to_string();
    }
    if let Some(value) = params.get("RoleName") {
        input.role_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for AddUserToGroup.
pub fn deserialize_add_user_to_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AddUserToGroupRequest, String> {
    let mut input = AddUserToGroupRequest::default();
    if let Some(value) = params.get("GroupName") {
        input.group_name = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for AssociateDelegationRequest.
pub fn deserialize_associate_delegation_request_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AssociateDelegationRequestRequest, String> {
    let mut input = AssociateDelegationRequestRequest::default();
    if let Some(value) = params.get("DelegationRequestId") {
        input.delegation_request_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for AttachGroupPolicy.
pub fn deserialize_attach_group_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AttachGroupPolicyRequest, String> {
    let mut input = AttachGroupPolicyRequest::default();
    if let Some(value) = params.get("GroupName") {
        input.group_name = value.to_string();
    }
    if let Some(value) = params.get("PolicyArn") {
        input.policy_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for AttachRolePolicy.
pub fn deserialize_attach_role_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AttachRolePolicyRequest, String> {
    let mut input = AttachRolePolicyRequest::default();
    if let Some(value) = params.get("PolicyArn") {
        input.policy_arn = value.to_string();
    }
    if let Some(value) = params.get("RoleName") {
        input.role_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for AttachUserPolicy.
pub fn deserialize_attach_user_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AttachUserPolicyRequest, String> {
    let mut input = AttachUserPolicyRequest::default();
    if let Some(value) = params.get("PolicyArn") {
        input.policy_arn = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ChangePassword.
pub fn deserialize_change_password_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ChangePasswordRequest, String> {
    let mut input = ChangePasswordRequest::default();
    if let Some(value) = params.get("NewPassword") {
        input.new_password = value.to_string();
    }
    if let Some(value) = params.get("OldPassword") {
        input.old_password = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateAccessKey.
pub fn deserialize_create_access_key_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateAccessKeyRequest, String> {
    let mut input = CreateAccessKeyRequest::default();
    if let Some(value) = params.get("UserName") {
        input.user_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateAccountAlias.
pub fn deserialize_create_account_alias_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateAccountAliasRequest, String> {
    let mut input = CreateAccountAliasRequest::default();
    if let Some(value) = params.get("AccountAlias") {
        input.account_alias = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateDelegationRequest.
pub fn deserialize_create_delegation_request_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateDelegationRequestRequest, String> {
    let mut input = CreateDelegationRequestRequest::default();
    if let Some(value) = params.get("Description") {
        input.description = value.to_string();
    }
    if let Some(value) = params.get("NotificationChannel") {
        input.notification_channel = value.to_string();
    }
    if let Some(value) = params.get("OnlySendByOwner") {
        input.only_send_by_owner = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse OnlySendByOwner: {e}"))?,
        );
    }
    if let Some(value) = params.get("OwnerAccountId") {
        input.owner_account_id = Some(value.to_string());
    }
    if let Some(val) = deserialize_delegation_permission_from_query(params, "Permissions")? {
        input.permissions = val;
    }
    if let Some(value) = params.get("RedirectUrl") {
        input.redirect_url = Some(value.to_string());
    }
    if let Some(value) = params.get("RequestMessage") {
        input.request_message = Some(value.to_string());
    }
    if let Some(value) = params.get("RequestorWorkflowId") {
        input.requestor_workflow_id = value.to_string();
    }
    if let Some(value) = params.get("SessionDuration") {
        input.session_duration = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse SessionDuration: {e}"))?;
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateGroup.
pub fn deserialize_create_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateGroupRequest, String> {
    let mut input = CreateGroupRequest::default();
    if let Some(value) = params.get("GroupName") {
        input.group_name = value.to_string();
    }
    if let Some(value) = params.get("Path") {
        input.path = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateInstanceProfile.
pub fn deserialize_create_instance_profile_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateInstanceProfileRequest, String> {
    let mut input = CreateInstanceProfileRequest::default();
    if let Some(value) = params.get("InstanceProfileName") {
        input.instance_profile_name = value.to_string();
    }
    if let Some(value) = params.get("Path") {
        input.path = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(tagListType { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateLoginProfile.
pub fn deserialize_create_login_profile_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateLoginProfileRequest, String> {
    let mut input = CreateLoginProfileRequest::default();
    if let Some(value) = params.get("Password") {
        input.password = Some(value.to_string());
    }
    if let Some(value) = params.get("PasswordResetRequired") {
        input.password_reset_required = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse PasswordResetRequired: {e}"))?,
        );
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateOpenIDConnectProvider.
pub fn deserialize_create_open_i_d_connect_provider_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateOpenIDConnectProviderRequest, String> {
    let mut input = CreateOpenIDConnectProviderRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "ClientIDList".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.client_i_d_list = Some(clientIDListType { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(tagListType { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ThumbprintList".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.thumbprint_list = Some(thumbprintListType { items });
        }
    }
    if let Some(value) = params.get("Url") {
        input.url = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreatePolicy.
pub fn deserialize_create_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreatePolicyRequest, String> {
    let mut input = CreatePolicyRequest::default();
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    if let Some(value) = params.get("Path") {
        input.path = Some(value.to_string());
    }
    if let Some(value) = params.get("PolicyDocument") {
        input.policy_document = value.to_string();
    }
    if let Some(value) = params.get("PolicyName") {
        input.policy_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(tagListType { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreatePolicyVersion.
pub fn deserialize_create_policy_version_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreatePolicyVersionRequest, String> {
    let mut input = CreatePolicyVersionRequest::default();
    if let Some(value) = params.get("PolicyArn") {
        input.policy_arn = value.to_string();
    }
    if let Some(value) = params.get("PolicyDocument") {
        input.policy_document = value.to_string();
    }
    if let Some(value) = params.get("SetAsDefault") {
        input.set_as_default = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse SetAsDefault: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateRole.
pub fn deserialize_create_role_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateRoleRequest, String> {
    let mut input = CreateRoleRequest::default();
    if let Some(value) = params.get("AssumeRolePolicyDocument") {
        input.assume_role_policy_document = value.to_string();
    }
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxSessionDuration") {
        input.max_session_duration = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxSessionDuration: {e}"))?,
        );
    }
    if let Some(value) = params.get("Path") {
        input.path = Some(value.to_string());
    }
    if let Some(value) = params.get("PermissionsBoundary") {
        input.permissions_boundary = Some(value.to_string());
    }
    if let Some(value) = params.get("RoleName") {
        input.role_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(tagListType { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateSAMLProvider.
pub fn deserialize_create_s_a_m_l_provider_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateSAMLProviderRequest, String> {
    let mut input = CreateSAMLProviderRequest::default();
    if let Some(value) = params.get("AddPrivateKey") {
        input.add_private_key = Some(value.to_string());
    }
    if let Some(value) = params.get("AssertionEncryptionMode") {
        input.assertion_encryption_mode = Some(value.to_string());
    }
    if let Some(value) = params.get("Name") {
        input.name = value.to_string();
    }
    if let Some(value) = params.get("SAMLMetadataDocument") {
        input.s_a_m_l_metadata_document = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(tagListType { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateServiceLinkedRole.
pub fn deserialize_create_service_linked_role_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateServiceLinkedRoleRequest, String> {
    let mut input = CreateServiceLinkedRoleRequest::default();
    if let Some(value) = params.get("AWSServiceName") {
        input.a_w_s_service_name = value.to_string();
    }
    if let Some(value) = params.get("CustomSuffix") {
        input.custom_suffix = Some(value.to_string());
    }
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateServiceSpecificCredential.
pub fn deserialize_create_service_specific_credential_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateServiceSpecificCredentialRequest, String> {
    let mut input = CreateServiceSpecificCredentialRequest::default();
    if let Some(value) = params.get("CredentialAgeDays") {
        input.credential_age_days = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse CredentialAgeDays: {e}"))?,
        );
    }
    if let Some(value) = params.get("ServiceName") {
        input.service_name = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateUser.
pub fn deserialize_create_user_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateUserRequest, String> {
    let mut input = CreateUserRequest::default();
    if let Some(value) = params.get("Path") {
        input.path = Some(value.to_string());
    }
    if let Some(value) = params.get("PermissionsBoundary") {
        input.permissions_boundary = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(tagListType { items });
        }
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateVirtualMFADevice.
pub fn deserialize_create_virtual_m_f_a_device_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateVirtualMFADeviceRequest, String> {
    let mut input = CreateVirtualMFADeviceRequest::default();
    if let Some(value) = params.get("Path") {
        input.path = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(tagListType { items });
        }
    }
    if let Some(value) = params.get("VirtualMFADeviceName") {
        input.virtual_m_f_a_device_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeactivateMFADevice.
pub fn deserialize_deactivate_m_f_a_device_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeactivateMFADeviceRequest, String> {
    let mut input = DeactivateMFADeviceRequest::default();
    if let Some(value) = params.get("SerialNumber") {
        input.serial_number = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteAccessKey.
pub fn deserialize_delete_access_key_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteAccessKeyRequest, String> {
    let mut input = DeleteAccessKeyRequest::default();
    if let Some(value) = params.get("AccessKeyId") {
        input.access_key_id = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteAccountAlias.
pub fn deserialize_delete_account_alias_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteAccountAliasRequest, String> {
    let mut input = DeleteAccountAliasRequest::default();
    if let Some(value) = params.get("AccountAlias") {
        input.account_alias = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteGroup.
pub fn deserialize_delete_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteGroupRequest, String> {
    let mut input = DeleteGroupRequest::default();
    if let Some(value) = params.get("GroupName") {
        input.group_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteGroupPolicy.
pub fn deserialize_delete_group_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteGroupPolicyRequest, String> {
    let mut input = DeleteGroupPolicyRequest::default();
    if let Some(value) = params.get("GroupName") {
        input.group_name = value.to_string();
    }
    if let Some(value) = params.get("PolicyName") {
        input.policy_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteInstanceProfile.
pub fn deserialize_delete_instance_profile_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteInstanceProfileRequest, String> {
    let mut input = DeleteInstanceProfileRequest::default();
    if let Some(value) = params.get("InstanceProfileName") {
        input.instance_profile_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteLoginProfile.
pub fn deserialize_delete_login_profile_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteLoginProfileRequest, String> {
    let mut input = DeleteLoginProfileRequest::default();
    if let Some(value) = params.get("UserName") {
        input.user_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteOpenIDConnectProvider.
pub fn deserialize_delete_open_i_d_connect_provider_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteOpenIDConnectProviderRequest, String> {
    let mut input = DeleteOpenIDConnectProviderRequest::default();
    if let Some(value) = params.get("OpenIDConnectProviderArn") {
        input.open_i_d_connect_provider_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeletePolicy.
pub fn deserialize_delete_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeletePolicyRequest, String> {
    let mut input = DeletePolicyRequest::default();
    if let Some(value) = params.get("PolicyArn") {
        input.policy_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeletePolicyVersion.
pub fn deserialize_delete_policy_version_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeletePolicyVersionRequest, String> {
    let mut input = DeletePolicyVersionRequest::default();
    if let Some(value) = params.get("PolicyArn") {
        input.policy_arn = value.to_string();
    }
    if let Some(value) = params.get("VersionId") {
        input.version_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteRole.
pub fn deserialize_delete_role_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteRoleRequest, String> {
    let mut input = DeleteRoleRequest::default();
    if let Some(value) = params.get("RoleName") {
        input.role_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteRolePermissionsBoundary.
pub fn deserialize_delete_role_permissions_boundary_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteRolePermissionsBoundaryRequest, String> {
    let mut input = DeleteRolePermissionsBoundaryRequest::default();
    if let Some(value) = params.get("RoleName") {
        input.role_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteRolePolicy.
pub fn deserialize_delete_role_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteRolePolicyRequest, String> {
    let mut input = DeleteRolePolicyRequest::default();
    if let Some(value) = params.get("PolicyName") {
        input.policy_name = value.to_string();
    }
    if let Some(value) = params.get("RoleName") {
        input.role_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteSAMLProvider.
pub fn deserialize_delete_s_a_m_l_provider_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteSAMLProviderRequest, String> {
    let mut input = DeleteSAMLProviderRequest::default();
    if let Some(value) = params.get("SAMLProviderArn") {
        input.s_a_m_l_provider_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteSSHPublicKey.
pub fn deserialize_delete_s_s_h_public_key_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteSSHPublicKeyRequest, String> {
    let mut input = DeleteSSHPublicKeyRequest::default();
    if let Some(value) = params.get("SSHPublicKeyId") {
        input.s_s_h_public_key_id = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteServerCertificate.
pub fn deserialize_delete_server_certificate_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteServerCertificateRequest, String> {
    let mut input = DeleteServerCertificateRequest::default();
    if let Some(value) = params.get("ServerCertificateName") {
        input.server_certificate_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteServiceLinkedRole.
pub fn deserialize_delete_service_linked_role_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteServiceLinkedRoleRequest, String> {
    let mut input = DeleteServiceLinkedRoleRequest::default();
    if let Some(value) = params.get("RoleName") {
        input.role_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteServiceSpecificCredential.
pub fn deserialize_delete_service_specific_credential_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteServiceSpecificCredentialRequest, String> {
    let mut input = DeleteServiceSpecificCredentialRequest::default();
    if let Some(value) = params.get("ServiceSpecificCredentialId") {
        input.service_specific_credential_id = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteSigningCertificate.
pub fn deserialize_delete_signing_certificate_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteSigningCertificateRequest, String> {
    let mut input = DeleteSigningCertificateRequest::default();
    if let Some(value) = params.get("CertificateId") {
        input.certificate_id = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteUser.
pub fn deserialize_delete_user_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteUserRequest, String> {
    let mut input = DeleteUserRequest::default();
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteUserPermissionsBoundary.
pub fn deserialize_delete_user_permissions_boundary_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteUserPermissionsBoundaryRequest, String> {
    let mut input = DeleteUserPermissionsBoundaryRequest::default();
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteUserPolicy.
pub fn deserialize_delete_user_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteUserPolicyRequest, String> {
    let mut input = DeleteUserPolicyRequest::default();
    if let Some(value) = params.get("PolicyName") {
        input.policy_name = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteVirtualMFADevice.
pub fn deserialize_delete_virtual_m_f_a_device_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteVirtualMFADeviceRequest, String> {
    let mut input = DeleteVirtualMFADeviceRequest::default();
    if let Some(value) = params.get("SerialNumber") {
        input.serial_number = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DetachGroupPolicy.
pub fn deserialize_detach_group_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DetachGroupPolicyRequest, String> {
    let mut input = DetachGroupPolicyRequest::default();
    if let Some(value) = params.get("GroupName") {
        input.group_name = value.to_string();
    }
    if let Some(value) = params.get("PolicyArn") {
        input.policy_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DetachRolePolicy.
pub fn deserialize_detach_role_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DetachRolePolicyRequest, String> {
    let mut input = DetachRolePolicyRequest::default();
    if let Some(value) = params.get("PolicyArn") {
        input.policy_arn = value.to_string();
    }
    if let Some(value) = params.get("RoleName") {
        input.role_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DetachUserPolicy.
pub fn deserialize_detach_user_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DetachUserPolicyRequest, String> {
    let mut input = DetachUserPolicyRequest::default();
    if let Some(value) = params.get("PolicyArn") {
        input.policy_arn = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DisableOrganizationsRootCredentialsManagement.
pub fn deserialize_disable_organizations_root_credentials_management_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DisableOrganizationsRootCredentialsManagementRequest, String> {
    let input = DisableOrganizationsRootCredentialsManagementRequest {};
    Ok(input)
}

/// Deserialize awsQuery request for DisableOrganizationsRootSessions.
pub fn deserialize_disable_organizations_root_sessions_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DisableOrganizationsRootSessionsRequest, String> {
    let input = DisableOrganizationsRootSessionsRequest {};
    Ok(input)
}

/// Deserialize awsQuery request for EnableMFADevice.
pub fn deserialize_enable_m_f_a_device_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<EnableMFADeviceRequest, String> {
    let mut input = EnableMFADeviceRequest::default();
    if let Some(value) = params.get("AuthenticationCode1") {
        input.authentication_code1 = value.to_string();
    }
    if let Some(value) = params.get("AuthenticationCode2") {
        input.authentication_code2 = value.to_string();
    }
    if let Some(value) = params.get("SerialNumber") {
        input.serial_number = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for EnableOrganizationsRootCredentialsManagement.
pub fn deserialize_enable_organizations_root_credentials_management_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<EnableOrganizationsRootCredentialsManagementRequest, String> {
    let input = EnableOrganizationsRootCredentialsManagementRequest {};
    Ok(input)
}

/// Deserialize awsQuery request for EnableOrganizationsRootSessions.
pub fn deserialize_enable_organizations_root_sessions_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<EnableOrganizationsRootSessionsRequest, String> {
    let input = EnableOrganizationsRootSessionsRequest {};
    Ok(input)
}

/// Deserialize awsQuery request for GenerateOrganizationsAccessReport.
pub fn deserialize_generate_organizations_access_report_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GenerateOrganizationsAccessReportRequest, String> {
    let mut input = GenerateOrganizationsAccessReportRequest::default();
    if let Some(value) = params.get("EntityPath") {
        input.entity_path = value.to_string();
    }
    if let Some(value) = params.get("OrganizationsPolicyId") {
        input.organizations_policy_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for GenerateServiceLastAccessedDetails.
pub fn deserialize_generate_service_last_accessed_details_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GenerateServiceLastAccessedDetailsRequest, String> {
    let mut input = GenerateServiceLastAccessedDetailsRequest::default();
    if let Some(value) = params.get("Arn") {
        input.arn = value.to_string();
    }
    if let Some(value) = params.get("Granularity") {
        input.granularity = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetAccessKeyLastUsed.
pub fn deserialize_get_access_key_last_used_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetAccessKeyLastUsedRequest, String> {
    let mut input = GetAccessKeyLastUsedRequest::default();
    if let Some(value) = params.get("AccessKeyId") {
        input.access_key_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetAccountAuthorizationDetails.
pub fn deserialize_get_account_authorization_details_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetAccountAuthorizationDetailsRequest, String> {
    let mut input = GetAccountAuthorizationDetailsRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Filter".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.filter = Some(entityListType { items });
        }
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetContextKeysForCustomPolicy.
pub fn deserialize_get_context_keys_for_custom_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetContextKeysForCustomPolicyRequest, String> {
    let mut input = GetContextKeysForCustomPolicyRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "PolicyInputList".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.policy_input_list = SimulationPolicyListType { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetContextKeysForPrincipalPolicy.
pub fn deserialize_get_context_keys_for_principal_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetContextKeysForPrincipalPolicyRequest, String> {
    let mut input = GetContextKeysForPrincipalPolicyRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "PolicyInputList".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.policy_input_list = Some(SimulationPolicyListType { items });
        }
    }
    if let Some(value) = params.get("PolicySourceArn") {
        input.policy_source_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetDelegationRequest.
pub fn deserialize_get_delegation_request_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetDelegationRequestRequest, String> {
    let mut input = GetDelegationRequestRequest::default();
    if let Some(value) = params.get("DelegationPermissionCheck") {
        input.delegation_permission_check = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DelegationPermissionCheck: {e}"))?,
        );
    }
    if let Some(value) = params.get("DelegationRequestId") {
        input.delegation_request_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetGroup.
pub fn deserialize_get_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetGroupRequest, String> {
    let mut input = GetGroupRequest::default();
    if let Some(value) = params.get("GroupName") {
        input.group_name = value.to_string();
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetGroupPolicy.
pub fn deserialize_get_group_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetGroupPolicyRequest, String> {
    let mut input = GetGroupPolicyRequest::default();
    if let Some(value) = params.get("GroupName") {
        input.group_name = value.to_string();
    }
    if let Some(value) = params.get("PolicyName") {
        input.policy_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetHumanReadableSummary.
pub fn deserialize_get_human_readable_summary_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetHumanReadableSummaryRequest, String> {
    let mut input = GetHumanReadableSummaryRequest::default();
    if let Some(value) = params.get("EntityArn") {
        input.entity_arn = value.to_string();
    }
    if let Some(value) = params.get("Locale") {
        input.locale = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetInstanceProfile.
pub fn deserialize_get_instance_profile_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetInstanceProfileRequest, String> {
    let mut input = GetInstanceProfileRequest::default();
    if let Some(value) = params.get("InstanceProfileName") {
        input.instance_profile_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetLoginProfile.
pub fn deserialize_get_login_profile_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetLoginProfileRequest, String> {
    let mut input = GetLoginProfileRequest::default();
    if let Some(value) = params.get("UserName") {
        input.user_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetMFADevice.
pub fn deserialize_get_m_f_a_device_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetMFADeviceRequest, String> {
    let mut input = GetMFADeviceRequest::default();
    if let Some(value) = params.get("SerialNumber") {
        input.serial_number = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetOpenIDConnectProvider.
pub fn deserialize_get_open_i_d_connect_provider_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetOpenIDConnectProviderRequest, String> {
    let mut input = GetOpenIDConnectProviderRequest::default();
    if let Some(value) = params.get("OpenIDConnectProviderArn") {
        input.open_i_d_connect_provider_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetOrganizationsAccessReport.
pub fn deserialize_get_organizations_access_report_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetOrganizationsAccessReportRequest, String> {
    let mut input = GetOrganizationsAccessReportRequest::default();
    if let Some(value) = params.get("JobId") {
        input.job_id = value.to_string();
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("SortKey") {
        input.sort_key = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetPolicy.
pub fn deserialize_get_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetPolicyRequest, String> {
    let mut input = GetPolicyRequest::default();
    if let Some(value) = params.get("PolicyArn") {
        input.policy_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetPolicyVersion.
pub fn deserialize_get_policy_version_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetPolicyVersionRequest, String> {
    let mut input = GetPolicyVersionRequest::default();
    if let Some(value) = params.get("PolicyArn") {
        input.policy_arn = value.to_string();
    }
    if let Some(value) = params.get("VersionId") {
        input.version_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetRole.
pub fn deserialize_get_role_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetRoleRequest, String> {
    let mut input = GetRoleRequest::default();
    if let Some(value) = params.get("RoleName") {
        input.role_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetRolePolicy.
pub fn deserialize_get_role_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetRolePolicyRequest, String> {
    let mut input = GetRolePolicyRequest::default();
    if let Some(value) = params.get("PolicyName") {
        input.policy_name = value.to_string();
    }
    if let Some(value) = params.get("RoleName") {
        input.role_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetSAMLProvider.
pub fn deserialize_get_s_a_m_l_provider_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetSAMLProviderRequest, String> {
    let mut input = GetSAMLProviderRequest::default();
    if let Some(value) = params.get("SAMLProviderArn") {
        input.s_a_m_l_provider_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetSSHPublicKey.
pub fn deserialize_get_s_s_h_public_key_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetSSHPublicKeyRequest, String> {
    let mut input = GetSSHPublicKeyRequest::default();
    if let Some(value) = params.get("Encoding") {
        input.encoding = value.to_string();
    }
    if let Some(value) = params.get("SSHPublicKeyId") {
        input.s_s_h_public_key_id = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetServerCertificate.
pub fn deserialize_get_server_certificate_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetServerCertificateRequest, String> {
    let mut input = GetServerCertificateRequest::default();
    if let Some(value) = params.get("ServerCertificateName") {
        input.server_certificate_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetServiceLastAccessedDetails.
pub fn deserialize_get_service_last_accessed_details_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetServiceLastAccessedDetailsRequest, String> {
    let mut input = GetServiceLastAccessedDetailsRequest::default();
    if let Some(value) = params.get("JobId") {
        input.job_id = value.to_string();
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetServiceLastAccessedDetailsWithEntities.
pub fn deserialize_get_service_last_accessed_details_with_entities_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetServiceLastAccessedDetailsWithEntitiesRequest, String> {
    let mut input = GetServiceLastAccessedDetailsWithEntitiesRequest::default();
    if let Some(value) = params.get("JobId") {
        input.job_id = value.to_string();
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("ServiceNamespace") {
        input.service_namespace = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetServiceLinkedRoleDeletionStatus.
pub fn deserialize_get_service_linked_role_deletion_status_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetServiceLinkedRoleDeletionStatusRequest, String> {
    let mut input = GetServiceLinkedRoleDeletionStatusRequest::default();
    if let Some(value) = params.get("DeletionTaskId") {
        input.deletion_task_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetUser.
pub fn deserialize_get_user_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetUserRequest, String> {
    let mut input = GetUserRequest::default();
    if let Some(value) = params.get("UserName") {
        input.user_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetUserPolicy.
pub fn deserialize_get_user_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetUserPolicyRequest, String> {
    let mut input = GetUserPolicyRequest::default();
    if let Some(value) = params.get("PolicyName") {
        input.policy_name = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListAccessKeys.
pub fn deserialize_list_access_keys_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListAccessKeysRequest, String> {
    let mut input = ListAccessKeysRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListAccountAliases.
pub fn deserialize_list_account_aliases_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListAccountAliasesRequest, String> {
    let mut input = ListAccountAliasesRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListAttachedGroupPolicies.
pub fn deserialize_list_attached_group_policies_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListAttachedGroupPoliciesRequest, String> {
    let mut input = ListAttachedGroupPoliciesRequest::default();
    if let Some(value) = params.get("GroupName") {
        input.group_name = value.to_string();
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("PathPrefix") {
        input.path_prefix = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListAttachedRolePolicies.
pub fn deserialize_list_attached_role_policies_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListAttachedRolePoliciesRequest, String> {
    let mut input = ListAttachedRolePoliciesRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("PathPrefix") {
        input.path_prefix = Some(value.to_string());
    }
    if let Some(value) = params.get("RoleName") {
        input.role_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListAttachedUserPolicies.
pub fn deserialize_list_attached_user_policies_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListAttachedUserPoliciesRequest, String> {
    let mut input = ListAttachedUserPoliciesRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("PathPrefix") {
        input.path_prefix = Some(value.to_string());
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListDelegationRequests.
pub fn deserialize_list_delegation_requests_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListDelegationRequestsRequest, String> {
    let mut input = ListDelegationRequestsRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("OwnerId") {
        input.owner_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListEntitiesForPolicy.
pub fn deserialize_list_entities_for_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListEntitiesForPolicyRequest, String> {
    let mut input = ListEntitiesForPolicyRequest::default();
    if let Some(value) = params.get("EntityFilter") {
        input.entity_filter = Some(value.to_string());
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("PathPrefix") {
        input.path_prefix = Some(value.to_string());
    }
    if let Some(value) = params.get("PolicyArn") {
        input.policy_arn = value.to_string();
    }
    if let Some(value) = params.get("PolicyUsageFilter") {
        input.policy_usage_filter = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListGroupPolicies.
pub fn deserialize_list_group_policies_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListGroupPoliciesRequest, String> {
    let mut input = ListGroupPoliciesRequest::default();
    if let Some(value) = params.get("GroupName") {
        input.group_name = value.to_string();
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListGroups.
pub fn deserialize_list_groups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListGroupsRequest, String> {
    let mut input = ListGroupsRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("PathPrefix") {
        input.path_prefix = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListGroupsForUser.
pub fn deserialize_list_groups_for_user_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListGroupsForUserRequest, String> {
    let mut input = ListGroupsForUserRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListInstanceProfileTags.
pub fn deserialize_list_instance_profile_tags_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListInstanceProfileTagsRequest, String> {
    let mut input = ListInstanceProfileTagsRequest::default();
    if let Some(value) = params.get("InstanceProfileName") {
        input.instance_profile_name = value.to_string();
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListInstanceProfiles.
pub fn deserialize_list_instance_profiles_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListInstanceProfilesRequest, String> {
    let mut input = ListInstanceProfilesRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("PathPrefix") {
        input.path_prefix = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListInstanceProfilesForRole.
pub fn deserialize_list_instance_profiles_for_role_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListInstanceProfilesForRoleRequest, String> {
    let mut input = ListInstanceProfilesForRoleRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("RoleName") {
        input.role_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListMFADeviceTags.
pub fn deserialize_list_m_f_a_device_tags_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListMFADeviceTagsRequest, String> {
    let mut input = ListMFADeviceTagsRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("SerialNumber") {
        input.serial_number = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListMFADevices.
pub fn deserialize_list_m_f_a_devices_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListMFADevicesRequest, String> {
    let mut input = ListMFADevicesRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListOpenIDConnectProviderTags.
pub fn deserialize_list_open_i_d_connect_provider_tags_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListOpenIDConnectProviderTagsRequest, String> {
    let mut input = ListOpenIDConnectProviderTagsRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("OpenIDConnectProviderArn") {
        input.open_i_d_connect_provider_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListOpenIDConnectProviders.
pub fn deserialize_list_open_i_d_connect_providers_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListOpenIDConnectProvidersRequest, String> {
    let input = ListOpenIDConnectProvidersRequest {};
    Ok(input)
}

/// Deserialize awsQuery request for ListOrganizationsFeatures.
pub fn deserialize_list_organizations_features_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListOrganizationsFeaturesRequest, String> {
    let input = ListOrganizationsFeaturesRequest {};
    Ok(input)
}

/// Deserialize awsQuery request for ListPolicies.
pub fn deserialize_list_policies_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListPoliciesRequest, String> {
    let mut input = ListPoliciesRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("OnlyAttached") {
        input.only_attached = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse OnlyAttached: {e}"))?,
        );
    }
    if let Some(value) = params.get("PathPrefix") {
        input.path_prefix = Some(value.to_string());
    }
    if let Some(value) = params.get("PolicyUsageFilter") {
        input.policy_usage_filter = Some(value.to_string());
    }
    if let Some(value) = params.get("Scope") {
        input.scope = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListPoliciesGrantingServiceAccess.
pub fn deserialize_list_policies_granting_service_access_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListPoliciesGrantingServiceAccessRequest, String> {
    let mut input = ListPoliciesGrantingServiceAccessRequest::default();
    if let Some(value) = params.get("Arn") {
        input.arn = value.to_string();
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ServiceNamespaces".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.service_namespaces = serviceNamespaceListType { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListPolicyTags.
pub fn deserialize_list_policy_tags_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListPolicyTagsRequest, String> {
    let mut input = ListPolicyTagsRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("PolicyArn") {
        input.policy_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListPolicyVersions.
pub fn deserialize_list_policy_versions_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListPolicyVersionsRequest, String> {
    let mut input = ListPolicyVersionsRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("PolicyArn") {
        input.policy_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListRolePolicies.
pub fn deserialize_list_role_policies_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListRolePoliciesRequest, String> {
    let mut input = ListRolePoliciesRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("RoleName") {
        input.role_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListRoleTags.
pub fn deserialize_list_role_tags_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListRoleTagsRequest, String> {
    let mut input = ListRoleTagsRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("RoleName") {
        input.role_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListRoles.
pub fn deserialize_list_roles_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListRolesRequest, String> {
    let mut input = ListRolesRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("PathPrefix") {
        input.path_prefix = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListSAMLProviderTags.
pub fn deserialize_list_s_a_m_l_provider_tags_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListSAMLProviderTagsRequest, String> {
    let mut input = ListSAMLProviderTagsRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("SAMLProviderArn") {
        input.s_a_m_l_provider_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListSAMLProviders.
pub fn deserialize_list_s_a_m_l_providers_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListSAMLProvidersRequest, String> {
    let input = ListSAMLProvidersRequest {};
    Ok(input)
}

/// Deserialize awsQuery request for ListSSHPublicKeys.
pub fn deserialize_list_s_s_h_public_keys_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListSSHPublicKeysRequest, String> {
    let mut input = ListSSHPublicKeysRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListServerCertificateTags.
pub fn deserialize_list_server_certificate_tags_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListServerCertificateTagsRequest, String> {
    let mut input = ListServerCertificateTagsRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("ServerCertificateName") {
        input.server_certificate_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListServerCertificates.
pub fn deserialize_list_server_certificates_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListServerCertificatesRequest, String> {
    let mut input = ListServerCertificatesRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("PathPrefix") {
        input.path_prefix = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListServiceSpecificCredentials.
pub fn deserialize_list_service_specific_credentials_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListServiceSpecificCredentialsRequest, String> {
    let mut input = ListServiceSpecificCredentialsRequest::default();
    if let Some(value) = params.get("AllUsers") {
        input.all_users = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AllUsers: {e}"))?,
        );
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("ServiceName") {
        input.service_name = Some(value.to_string());
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListSigningCertificates.
pub fn deserialize_list_signing_certificates_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListSigningCertificatesRequest, String> {
    let mut input = ListSigningCertificatesRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListUserPolicies.
pub fn deserialize_list_user_policies_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListUserPoliciesRequest, String> {
    let mut input = ListUserPoliciesRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListUserTags.
pub fn deserialize_list_user_tags_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListUserTagsRequest, String> {
    let mut input = ListUserTagsRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListUsers.
pub fn deserialize_list_users_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListUsersRequest, String> {
    let mut input = ListUsersRequest::default();
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("PathPrefix") {
        input.path_prefix = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListVirtualMFADevices.
pub fn deserialize_list_virtual_m_f_a_devices_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListVirtualMFADevicesRequest, String> {
    let mut input = ListVirtualMFADevicesRequest::default();
    if let Some(value) = params.get("AssignmentStatus") {
        input.assignment_status = Some(value.to_string());
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for PutGroupPolicy.
pub fn deserialize_put_group_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PutGroupPolicyRequest, String> {
    let mut input = PutGroupPolicyRequest::default();
    if let Some(value) = params.get("GroupName") {
        input.group_name = value.to_string();
    }
    if let Some(value) = params.get("PolicyDocument") {
        input.policy_document = value.to_string();
    }
    if let Some(value) = params.get("PolicyName") {
        input.policy_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for PutRolePermissionsBoundary.
pub fn deserialize_put_role_permissions_boundary_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PutRolePermissionsBoundaryRequest, String> {
    let mut input = PutRolePermissionsBoundaryRequest::default();
    if let Some(value) = params.get("PermissionsBoundary") {
        input.permissions_boundary = value.to_string();
    }
    if let Some(value) = params.get("RoleName") {
        input.role_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for PutRolePolicy.
pub fn deserialize_put_role_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PutRolePolicyRequest, String> {
    let mut input = PutRolePolicyRequest::default();
    if let Some(value) = params.get("PolicyDocument") {
        input.policy_document = value.to_string();
    }
    if let Some(value) = params.get("PolicyName") {
        input.policy_name = value.to_string();
    }
    if let Some(value) = params.get("RoleName") {
        input.role_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for PutUserPermissionsBoundary.
pub fn deserialize_put_user_permissions_boundary_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PutUserPermissionsBoundaryRequest, String> {
    let mut input = PutUserPermissionsBoundaryRequest::default();
    if let Some(value) = params.get("PermissionsBoundary") {
        input.permissions_boundary = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for PutUserPolicy.
pub fn deserialize_put_user_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PutUserPolicyRequest, String> {
    let mut input = PutUserPolicyRequest::default();
    if let Some(value) = params.get("PolicyDocument") {
        input.policy_document = value.to_string();
    }
    if let Some(value) = params.get("PolicyName") {
        input.policy_name = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for RejectDelegationRequest.
pub fn deserialize_reject_delegation_request_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RejectDelegationRequestRequest, String> {
    let mut input = RejectDelegationRequestRequest::default();
    if let Some(value) = params.get("DelegationRequestId") {
        input.delegation_request_id = value.to_string();
    }
    if let Some(value) = params.get("Notes") {
        input.notes = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for RemoveClientIDFromOpenIDConnectProvider.
pub fn deserialize_remove_client_i_d_from_open_i_d_connect_provider_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RemoveClientIDFromOpenIDConnectProviderRequest, String> {
    let mut input = RemoveClientIDFromOpenIDConnectProviderRequest::default();
    if let Some(value) = params.get("ClientID") {
        input.client_i_d = value.to_string();
    }
    if let Some(value) = params.get("OpenIDConnectProviderArn") {
        input.open_i_d_connect_provider_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for RemoveRoleFromInstanceProfile.
pub fn deserialize_remove_role_from_instance_profile_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RemoveRoleFromInstanceProfileRequest, String> {
    let mut input = RemoveRoleFromInstanceProfileRequest::default();
    if let Some(value) = params.get("InstanceProfileName") {
        input.instance_profile_name = value.to_string();
    }
    if let Some(value) = params.get("RoleName") {
        input.role_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for RemoveUserFromGroup.
pub fn deserialize_remove_user_from_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RemoveUserFromGroupRequest, String> {
    let mut input = RemoveUserFromGroupRequest::default();
    if let Some(value) = params.get("GroupName") {
        input.group_name = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ResetServiceSpecificCredential.
pub fn deserialize_reset_service_specific_credential_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ResetServiceSpecificCredentialRequest, String> {
    let mut input = ResetServiceSpecificCredentialRequest::default();
    if let Some(value) = params.get("ServiceSpecificCredentialId") {
        input.service_specific_credential_id = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ResyncMFADevice.
pub fn deserialize_resync_m_f_a_device_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ResyncMFADeviceRequest, String> {
    let mut input = ResyncMFADeviceRequest::default();
    if let Some(value) = params.get("AuthenticationCode1") {
        input.authentication_code1 = value.to_string();
    }
    if let Some(value) = params.get("AuthenticationCode2") {
        input.authentication_code2 = value.to_string();
    }
    if let Some(value) = params.get("SerialNumber") {
        input.serial_number = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for SendDelegationToken.
pub fn deserialize_send_delegation_token_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SendDelegationTokenRequest, String> {
    let mut input = SendDelegationTokenRequest::default();
    if let Some(value) = params.get("DelegationRequestId") {
        input.delegation_request_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetDefaultPolicyVersion.
pub fn deserialize_set_default_policy_version_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetDefaultPolicyVersionRequest, String> {
    let mut input = SetDefaultPolicyVersionRequest::default();
    if let Some(value) = params.get("PolicyArn") {
        input.policy_arn = value.to_string();
    }
    if let Some(value) = params.get("VersionId") {
        input.version_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetSecurityTokenServicePreferences.
pub fn deserialize_set_security_token_service_preferences_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetSecurityTokenServicePreferencesRequest, String> {
    let mut input = SetSecurityTokenServicePreferencesRequest::default();
    if let Some(value) = params.get("GlobalEndpointTokenVersion") {
        input.global_endpoint_token_version = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for SimulateCustomPolicy.
pub fn deserialize_simulate_custom_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SimulateCustomPolicyRequest, String> {
    let mut input = SimulateCustomPolicyRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "ActionNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.action_names = ActionNameListType { items };
        }
    }
    if let Some(value) = params.get("CallerArn") {
        input.caller_arn = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ContextEntries".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_context_entry_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.context_entries = Some(ContextEntryListType { items });
        }
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "PermissionsBoundaryPolicyInputList".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.permissions_boundary_policy_input_list = Some(SimulationPolicyListType { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "PolicyInputList".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.policy_input_list = SimulationPolicyListType { items };
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ResourceArns".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.resource_arns = Some(ResourceNameListType { items });
        }
    }
    if let Some(value) = params.get("ResourceHandlingOption") {
        input.resource_handling_option = Some(value.to_string());
    }
    if let Some(value) = params.get("ResourceOwner") {
        input.resource_owner = Some(value.to_string());
    }
    if let Some(value) = params.get("ResourcePolicy") {
        input.resource_policy = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for SimulatePrincipalPolicy.
pub fn deserialize_simulate_principal_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SimulatePrincipalPolicyRequest, String> {
    let mut input = SimulatePrincipalPolicyRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "ActionNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.action_names = ActionNameListType { items };
        }
    }
    if let Some(value) = params.get("CallerArn") {
        input.caller_arn = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ContextEntries".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_context_entry_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.context_entries = Some(ContextEntryListType { items });
        }
    }
    if let Some(value) = params.get("Marker") {
        input.marker = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "PermissionsBoundaryPolicyInputList".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.permissions_boundary_policy_input_list = Some(SimulationPolicyListType { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "PolicyInputList".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.policy_input_list = Some(SimulationPolicyListType { items });
        }
    }
    if let Some(value) = params.get("PolicySourceArn") {
        input.policy_source_arn = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ResourceArns".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.resource_arns = Some(ResourceNameListType { items });
        }
    }
    if let Some(value) = params.get("ResourceHandlingOption") {
        input.resource_handling_option = Some(value.to_string());
    }
    if let Some(value) = params.get("ResourceOwner") {
        input.resource_owner = Some(value.to_string());
    }
    if let Some(value) = params.get("ResourcePolicy") {
        input.resource_policy = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for TagInstanceProfile.
pub fn deserialize_tag_instance_profile_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<TagInstanceProfileRequest, String> {
    let mut input = TagInstanceProfileRequest::default();
    if let Some(value) = params.get("InstanceProfileName") {
        input.instance_profile_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = tagListType { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for TagMFADevice.
pub fn deserialize_tag_m_f_a_device_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<TagMFADeviceRequest, String> {
    let mut input = TagMFADeviceRequest::default();
    if let Some(value) = params.get("SerialNumber") {
        input.serial_number = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = tagListType { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for TagOpenIDConnectProvider.
pub fn deserialize_tag_open_i_d_connect_provider_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<TagOpenIDConnectProviderRequest, String> {
    let mut input = TagOpenIDConnectProviderRequest::default();
    if let Some(value) = params.get("OpenIDConnectProviderArn") {
        input.open_i_d_connect_provider_arn = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = tagListType { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for TagPolicy.
pub fn deserialize_tag_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<TagPolicyRequest, String> {
    let mut input = TagPolicyRequest::default();
    if let Some(value) = params.get("PolicyArn") {
        input.policy_arn = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = tagListType { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for TagRole.
pub fn deserialize_tag_role_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<TagRoleRequest, String> {
    let mut input = TagRoleRequest::default();
    if let Some(value) = params.get("RoleName") {
        input.role_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = tagListType { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for TagSAMLProvider.
pub fn deserialize_tag_s_a_m_l_provider_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<TagSAMLProviderRequest, String> {
    let mut input = TagSAMLProviderRequest::default();
    if let Some(value) = params.get("SAMLProviderArn") {
        input.s_a_m_l_provider_arn = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = tagListType { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for TagServerCertificate.
pub fn deserialize_tag_server_certificate_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<TagServerCertificateRequest, String> {
    let mut input = TagServerCertificateRequest::default();
    if let Some(value) = params.get("ServerCertificateName") {
        input.server_certificate_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = tagListType { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for TagUser.
pub fn deserialize_tag_user_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<TagUserRequest, String> {
    let mut input = TagUserRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = tagListType { items };
        }
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for UntagInstanceProfile.
pub fn deserialize_untag_instance_profile_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UntagInstanceProfileRequest, String> {
    let mut input = UntagInstanceProfileRequest::default();
    if let Some(value) = params.get("InstanceProfileName") {
        input.instance_profile_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_keys = tagKeyListType { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for UntagMFADevice.
pub fn deserialize_untag_m_f_a_device_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UntagMFADeviceRequest, String> {
    let mut input = UntagMFADeviceRequest::default();
    if let Some(value) = params.get("SerialNumber") {
        input.serial_number = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_keys = tagKeyListType { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for UntagOpenIDConnectProvider.
pub fn deserialize_untag_open_i_d_connect_provider_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UntagOpenIDConnectProviderRequest, String> {
    let mut input = UntagOpenIDConnectProviderRequest::default();
    if let Some(value) = params.get("OpenIDConnectProviderArn") {
        input.open_i_d_connect_provider_arn = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_keys = tagKeyListType { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for UntagPolicy.
pub fn deserialize_untag_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UntagPolicyRequest, String> {
    let mut input = UntagPolicyRequest::default();
    if let Some(value) = params.get("PolicyArn") {
        input.policy_arn = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_keys = tagKeyListType { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for UntagRole.
pub fn deserialize_untag_role_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UntagRoleRequest, String> {
    let mut input = UntagRoleRequest::default();
    if let Some(value) = params.get("RoleName") {
        input.role_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_keys = tagKeyListType { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for UntagSAMLProvider.
pub fn deserialize_untag_s_a_m_l_provider_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UntagSAMLProviderRequest, String> {
    let mut input = UntagSAMLProviderRequest::default();
    if let Some(value) = params.get("SAMLProviderArn") {
        input.s_a_m_l_provider_arn = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_keys = tagKeyListType { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for UntagServerCertificate.
pub fn deserialize_untag_server_certificate_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UntagServerCertificateRequest, String> {
    let mut input = UntagServerCertificateRequest::default();
    if let Some(value) = params.get("ServerCertificateName") {
        input.server_certificate_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_keys = tagKeyListType { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for UntagUser.
pub fn deserialize_untag_user_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UntagUserRequest, String> {
    let mut input = UntagUserRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "TagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_keys = tagKeyListType { items };
        }
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateAccessKey.
pub fn deserialize_update_access_key_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateAccessKeyRequest, String> {
    let mut input = UpdateAccessKeyRequest::default();
    if let Some(value) = params.get("AccessKeyId") {
        input.access_key_id = value.to_string();
    }
    if let Some(value) = params.get("Status") {
        input.status = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateAccountPasswordPolicy.
pub fn deserialize_update_account_password_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateAccountPasswordPolicyRequest, String> {
    let mut input = UpdateAccountPasswordPolicyRequest::default();
    if let Some(value) = params.get("AllowUsersToChangePassword") {
        input.allow_users_to_change_password = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AllowUsersToChangePassword: {e}"))?,
        );
    }
    if let Some(value) = params.get("HardExpiry") {
        input.hard_expiry = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse HardExpiry: {e}"))?,
        );
    }
    if let Some(value) = params.get("MaxPasswordAge") {
        input.max_password_age = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxPasswordAge: {e}"))?,
        );
    }
    if let Some(value) = params.get("MinimumPasswordLength") {
        input.minimum_password_length = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MinimumPasswordLength: {e}"))?,
        );
    }
    if let Some(value) = params.get("PasswordReusePrevention") {
        input.password_reuse_prevention = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse PasswordReusePrevention: {e}"))?,
        );
    }
    if let Some(value) = params.get("RequireLowercaseCharacters") {
        input.require_lowercase_characters = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse RequireLowercaseCharacters: {e}"))?,
        );
    }
    if let Some(value) = params.get("RequireNumbers") {
        input.require_numbers = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse RequireNumbers: {e}"))?,
        );
    }
    if let Some(value) = params.get("RequireSymbols") {
        input.require_symbols = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse RequireSymbols: {e}"))?,
        );
    }
    if let Some(value) = params.get("RequireUppercaseCharacters") {
        input.require_uppercase_characters = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse RequireUppercaseCharacters: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateAssumeRolePolicy.
pub fn deserialize_update_assume_role_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateAssumeRolePolicyRequest, String> {
    let mut input = UpdateAssumeRolePolicyRequest::default();
    if let Some(value) = params.get("PolicyDocument") {
        input.policy_document = value.to_string();
    }
    if let Some(value) = params.get("RoleName") {
        input.role_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateDelegationRequest.
pub fn deserialize_update_delegation_request_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateDelegationRequestRequest, String> {
    let mut input = UpdateDelegationRequestRequest::default();
    if let Some(value) = params.get("DelegationRequestId") {
        input.delegation_request_id = value.to_string();
    }
    if let Some(value) = params.get("Notes") {
        input.notes = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateGroup.
pub fn deserialize_update_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateGroupRequest, String> {
    let mut input = UpdateGroupRequest::default();
    if let Some(value) = params.get("GroupName") {
        input.group_name = value.to_string();
    }
    if let Some(value) = params.get("NewGroupName") {
        input.new_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("NewPath") {
        input.new_path = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateLoginProfile.
pub fn deserialize_update_login_profile_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateLoginProfileRequest, String> {
    let mut input = UpdateLoginProfileRequest::default();
    if let Some(value) = params.get("Password") {
        input.password = Some(value.to_string());
    }
    if let Some(value) = params.get("PasswordResetRequired") {
        input.password_reset_required = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse PasswordResetRequired: {e}"))?,
        );
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateOpenIDConnectProviderThumbprint.
pub fn deserialize_update_open_i_d_connect_provider_thumbprint_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateOpenIDConnectProviderThumbprintRequest, String> {
    let mut input = UpdateOpenIDConnectProviderThumbprintRequest::default();
    if let Some(value) = params.get("OpenIDConnectProviderArn") {
        input.open_i_d_connect_provider_arn = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ThumbprintList".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.thumbprint_list = thumbprintListType { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateRole.
pub fn deserialize_update_role_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateRoleRequest, String> {
    let mut input = UpdateRoleRequest::default();
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxSessionDuration") {
        input.max_session_duration = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxSessionDuration: {e}"))?,
        );
    }
    if let Some(value) = params.get("RoleName") {
        input.role_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateRoleDescription.
pub fn deserialize_update_role_description_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateRoleDescriptionRequest, String> {
    let mut input = UpdateRoleDescriptionRequest::default();
    if let Some(value) = params.get("Description") {
        input.description = value.to_string();
    }
    if let Some(value) = params.get("RoleName") {
        input.role_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateSAMLProvider.
pub fn deserialize_update_s_a_m_l_provider_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateSAMLProviderRequest, String> {
    let mut input = UpdateSAMLProviderRequest::default();
    if let Some(value) = params.get("AddPrivateKey") {
        input.add_private_key = Some(value.to_string());
    }
    if let Some(value) = params.get("AssertionEncryptionMode") {
        input.assertion_encryption_mode = Some(value.to_string());
    }
    if let Some(value) = params.get("RemovePrivateKey") {
        input.remove_private_key = Some(value.to_string());
    }
    if let Some(value) = params.get("SAMLMetadataDocument") {
        input.s_a_m_l_metadata_document = Some(value.to_string());
    }
    if let Some(value) = params.get("SAMLProviderArn") {
        input.s_a_m_l_provider_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateSSHPublicKey.
pub fn deserialize_update_s_s_h_public_key_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateSSHPublicKeyRequest, String> {
    let mut input = UpdateSSHPublicKeyRequest::default();
    if let Some(value) = params.get("SSHPublicKeyId") {
        input.s_s_h_public_key_id = value.to_string();
    }
    if let Some(value) = params.get("Status") {
        input.status = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateServerCertificate.
pub fn deserialize_update_server_certificate_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateServerCertificateRequest, String> {
    let mut input = UpdateServerCertificateRequest::default();
    if let Some(value) = params.get("NewPath") {
        input.new_path = Some(value.to_string());
    }
    if let Some(value) = params.get("NewServerCertificateName") {
        input.new_server_certificate_name = Some(value.to_string());
    }
    if let Some(value) = params.get("ServerCertificateName") {
        input.server_certificate_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateServiceSpecificCredential.
pub fn deserialize_update_service_specific_credential_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateServiceSpecificCredentialRequest, String> {
    let mut input = UpdateServiceSpecificCredentialRequest::default();
    if let Some(value) = params.get("ServiceSpecificCredentialId") {
        input.service_specific_credential_id = value.to_string();
    }
    if let Some(value) = params.get("Status") {
        input.status = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateSigningCertificate.
pub fn deserialize_update_signing_certificate_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateSigningCertificateRequest, String> {
    let mut input = UpdateSigningCertificateRequest::default();
    if let Some(value) = params.get("CertificateId") {
        input.certificate_id = value.to_string();
    }
    if let Some(value) = params.get("Status") {
        input.status = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateUser.
pub fn deserialize_update_user_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateUserRequest, String> {
    let mut input = UpdateUserRequest::default();
    if let Some(value) = params.get("NewPath") {
        input.new_path = Some(value.to_string());
    }
    if let Some(value) = params.get("NewUserName") {
        input.new_user_name = Some(value.to_string());
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for UploadSSHPublicKey.
pub fn deserialize_upload_s_s_h_public_key_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UploadSSHPublicKeyRequest, String> {
    let mut input = UploadSSHPublicKeyRequest::default();
    if let Some(value) = params.get("SSHPublicKeyBody") {
        input.s_s_h_public_key_body = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for UploadServerCertificate.
pub fn deserialize_upload_server_certificate_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UploadServerCertificateRequest, String> {
    let mut input = UploadServerCertificateRequest::default();
    if let Some(value) = params.get("CertificateBody") {
        input.certificate_body = value.to_string();
    }
    if let Some(value) = params.get("CertificateChain") {
        input.certificate_chain = Some(value.to_string());
    }
    if let Some(value) = params.get("Path") {
        input.path = Some(value.to_string());
    }
    if let Some(value) = params.get("PrivateKey") {
        input.private_key = value.to_string();
    }
    if let Some(value) = params.get("ServerCertificateName") {
        input.server_certificate_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(tagListType { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for UploadSigningCertificate.
pub fn deserialize_upload_signing_certificate_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UploadSigningCertificateRequest, String> {
    let mut input = UploadSigningCertificateRequest::default();
    if let Some(value) = params.get("CertificateBody") {
        input.certificate_body = value.to_string();
    }
    if let Some(value) = params.get("UserName") {
        input.user_name = Some(value.to_string());
    }
    Ok(input)
}
