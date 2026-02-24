//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-sts

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

/// Serialize response for awsQuery protocol.
pub fn serialize_assume_role_response(result: &AssumeRoleResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<AssumeRoleResult>{inner_xml}</AssumeRoleResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AssumeRoleResponse xmlns="https://sts.amazonaws.com/doc/2011-06-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AssumeRoleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_assume_role_with_s_a_m_l_response(
    result: &AssumeRoleWithSAMLResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<AssumeRoleWithSAMLResult>{inner_xml}</AssumeRoleWithSAMLResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AssumeRoleWithSAMLResponse xmlns="https://sts.amazonaws.com/doc/2011-06-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AssumeRoleWithSAMLResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_assume_role_with_web_identity_response(
    result: &AssumeRoleWithWebIdentityResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<AssumeRoleWithWebIdentityResult>{inner_xml}</AssumeRoleWithWebIdentityResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AssumeRoleWithWebIdentityResponse xmlns="https://sts.amazonaws.com/doc/2011-06-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AssumeRoleWithWebIdentityResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_assume_root_response(result: &AssumeRootResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<AssumeRootResult>{inner_xml}</AssumeRootResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AssumeRootResponse xmlns="https://sts.amazonaws.com/doc/2011-06-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AssumeRootResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_decode_authorization_message_response(
    result: &DecodeAuthorizationMessageResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DecodeAuthorizationMessageResult>{inner_xml}</DecodeAuthorizationMessageResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DecodeAuthorizationMessageResponse xmlns="https://sts.amazonaws.com/doc/2011-06-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DecodeAuthorizationMessageResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_access_key_info_response(result: &GetAccessKeyInfoResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetAccessKeyInfoResult>{inner_xml}</GetAccessKeyInfoResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetAccessKeyInfoResponse xmlns="https://sts.amazonaws.com/doc/2011-06-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetAccessKeyInfoResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_caller_identity_response(result: &GetCallerIdentityResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetCallerIdentityResult>{inner_xml}</GetCallerIdentityResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetCallerIdentityResponse xmlns="https://sts.amazonaws.com/doc/2011-06-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetCallerIdentityResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_delegated_access_token_response(
    result: &GetDelegatedAccessTokenResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<GetDelegatedAccessTokenResult>{inner_xml}</GetDelegatedAccessTokenResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetDelegatedAccessTokenResponse xmlns="https://sts.amazonaws.com/doc/2011-06-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetDelegatedAccessTokenResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_federation_token_response(
    result: &GetFederationTokenResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetFederationTokenResult>{inner_xml}</GetFederationTokenResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetFederationTokenResponse xmlns="https://sts.amazonaws.com/doc/2011-06-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetFederationTokenResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_session_token_response(result: &GetSessionTokenResponse) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetSessionTokenResult>{inner_xml}</GetSessionTokenResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetSessionTokenResponse xmlns="https://sts.amazonaws.com/doc/2011-06-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetSessionTokenResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_web_identity_token_response(
    result: &GetWebIdentityTokenResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetWebIdentityTokenResult>{inner_xml}</GetWebIdentityTokenResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetWebIdentityTokenResponse xmlns="https://sts.amazonaws.com/doc/2011-06-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetWebIdentityTokenResponse>"#
    );
    MockResponse::xml(200, xml)
}

fn deserialize_policy_descriptor_type_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<PolicyDescriptorType>, String> {
    let mut item = PolicyDescriptorType::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.arn")) {
        item.arn = Some(value.to_string());
        found = true;
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

fn deserialize_provided_context_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ProvidedContext>, String> {
    let mut item = ProvidedContext::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.ContextAssertion")) {
        item.context_assertion = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ProviderArn")) {
        item.provider_arn = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

/// Deserialize awsQuery request for AssumeRole.
pub fn deserialize_assume_role_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AssumeRoleRequest, String> {
    let mut input = AssumeRoleRequest::default();
    if let Some(value) = params.get("DurationSeconds") {
        input.duration_seconds = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse DurationSeconds: {e}"))?,
        );
    }
    if let Some(value) = params.get("ExternalId") {
        input.external_id = Some(value.to_string());
    }
    if let Some(value) = params.get("Policy") {
        input.policy = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "PolicyArns".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_policy_descriptor_type_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.policy_arns = Some(policyDescriptorListType { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ProvidedContexts".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_provided_context_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.provided_contexts = Some(ProvidedContextsListType { items });
        }
    }
    if let Some(value) = params.get("RoleArn") {
        input.role_arn = value.to_string();
    }
    if let Some(value) = params.get("RoleSessionName") {
        input.role_session_name = value.to_string();
    }
    if let Some(value) = params.get("SerialNumber") {
        input.serial_number = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceIdentity") {
        input.source_identity = Some(value.to_string());
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
    if let Some(value) = params.get("TokenCode") {
        input.token_code = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TransitiveTagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.transitive_tag_keys = Some(tagKeyListType { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for AssumeRoleWithSAML.
pub fn deserialize_assume_role_with_s_a_m_l_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AssumeRoleWithSAMLRequest, String> {
    let mut input = AssumeRoleWithSAMLRequest::default();
    if let Some(value) = params.get("DurationSeconds") {
        input.duration_seconds = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse DurationSeconds: {e}"))?,
        );
    }
    if let Some(value) = params.get("Policy") {
        input.policy = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "PolicyArns".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_policy_descriptor_type_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.policy_arns = Some(policyDescriptorListType { items });
        }
    }
    if let Some(value) = params.get("PrincipalArn") {
        input.principal_arn = value.to_string();
    }
    if let Some(value) = params.get("RoleArn") {
        input.role_arn = value.to_string();
    }
    if let Some(value) = params.get("SAMLAssertion") {
        input.s_a_m_l_assertion = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for AssumeRoleWithWebIdentity.
pub fn deserialize_assume_role_with_web_identity_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AssumeRoleWithWebIdentityRequest, String> {
    let mut input = AssumeRoleWithWebIdentityRequest::default();
    if let Some(value) = params.get("DurationSeconds") {
        input.duration_seconds = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse DurationSeconds: {e}"))?,
        );
    }
    if let Some(value) = params.get("Policy") {
        input.policy = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "PolicyArns".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_policy_descriptor_type_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.policy_arns = Some(policyDescriptorListType { items });
        }
    }
    if let Some(value) = params.get("ProviderId") {
        input.provider_id = Some(value.to_string());
    }
    if let Some(value) = params.get("RoleArn") {
        input.role_arn = value.to_string();
    }
    if let Some(value) = params.get("RoleSessionName") {
        input.role_session_name = value.to_string();
    }
    if let Some(value) = params.get("WebIdentityToken") {
        input.web_identity_token = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for AssumeRoot.
pub fn deserialize_assume_root_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AssumeRootRequest, String> {
    let mut input = AssumeRootRequest::default();
    if let Some(value) = params.get("DurationSeconds") {
        input.duration_seconds = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse DurationSeconds: {e}"))?,
        );
    }
    if let Some(value) = params.get("TargetPrincipal") {
        input.target_principal = value.to_string();
    }
    if let Some(val) = deserialize_policy_descriptor_type_from_query(params, "TaskPolicyArn")? {
        input.task_policy_arn = val;
    }
    Ok(input)
}

/// Deserialize awsQuery request for DecodeAuthorizationMessage.
pub fn deserialize_decode_authorization_message_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DecodeAuthorizationMessageRequest, String> {
    let mut input = DecodeAuthorizationMessageRequest::default();
    if let Some(value) = params.get("EncodedMessage") {
        input.encoded_message = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetAccessKeyInfo.
pub fn deserialize_get_access_key_info_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetAccessKeyInfoRequest, String> {
    let mut input = GetAccessKeyInfoRequest::default();
    if let Some(value) = params.get("AccessKeyId") {
        input.access_key_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetCallerIdentity.
pub fn deserialize_get_caller_identity_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetCallerIdentityRequest, String> {
    let input = GetCallerIdentityRequest {};
    Ok(input)
}

/// Deserialize awsQuery request for GetDelegatedAccessToken.
pub fn deserialize_get_delegated_access_token_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetDelegatedAccessTokenRequest, String> {
    let mut input = GetDelegatedAccessTokenRequest::default();
    if let Some(value) = params.get("TradeInToken") {
        input.trade_in_token = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetFederationToken.
pub fn deserialize_get_federation_token_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetFederationTokenRequest, String> {
    let mut input = GetFederationTokenRequest::default();
    if let Some(value) = params.get("DurationSeconds") {
        input.duration_seconds = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse DurationSeconds: {e}"))?,
        );
    }
    if let Some(value) = params.get("Name") {
        input.name = value.to_string();
    }
    if let Some(value) = params.get("Policy") {
        input.policy = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "PolicyArns".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_policy_descriptor_type_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.policy_arns = Some(policyDescriptorListType { items });
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
    Ok(input)
}

/// Deserialize awsQuery request for GetSessionToken.
pub fn deserialize_get_session_token_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetSessionTokenRequest, String> {
    let mut input = GetSessionTokenRequest::default();
    if let Some(value) = params.get("DurationSeconds") {
        input.duration_seconds = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse DurationSeconds: {e}"))?,
        );
    }
    if let Some(value) = params.get("SerialNumber") {
        input.serial_number = Some(value.to_string());
    }
    if let Some(value) = params.get("TokenCode") {
        input.token_code = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetWebIdentityToken.
pub fn deserialize_get_web_identity_token_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetWebIdentityTokenRequest, String> {
    let mut input = GetWebIdentityTokenRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Audience".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.audience = webIdentityTokenAudienceListType { items };
        }
    }
    if let Some(value) = params.get("DurationSeconds") {
        input.duration_seconds = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse DurationSeconds: {e}"))?,
        );
    }
    if let Some(value) = params.get("SigningAlgorithm") {
        input.signing_algorithm = value.to_string();
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
